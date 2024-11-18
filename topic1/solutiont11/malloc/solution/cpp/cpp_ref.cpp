//
// Created by Lincheng Ge on 2018/10/6.
//

#include <assert.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "solve.h"
#include "utils.h"

#define ALIGN(p) (((size_t)(p) + (ALIGNMENT - 1)) & ~0x7)

#define WSIZE 4
#define DSIZE 8
#define CHUNKSIZE 168

#define MAX(x, y) ((x) > (y) ? (x) : (y))

#define PACK(size, alloc) ((size) | (alloc))

#define GET(p) (*(unsigned int *)(p))
#define PUT(p, val) (*(unsigned int *)(p) = (val))

#define GET_SIZE(p) (GET(p) & ~0x7)
#define GET_ALLOC(p) (GET(p) & 0x1)
#define GET_PREV_ALLOC(p) (GET(p) & 0x2)
#define GET_NEXT_ALLOC(p) (GET(p) & 0x4)
#define HDRP(bp) ((char *)(bp)-WSIZE)
#define FTRP(bp) ((char *)(bp) + GET_SIZE(HDRP(bp)) - DSIZE)

#define NEXT_BLKP(bp) ((char *)(bp) + GET_SIZE(((char *)(bp)-WSIZE)))
#define PREV_BLKP(bp) ((char *)(bp)-GET_SIZE(((char *)(bp)-DSIZE)))

#define GET_NEXTP(bp) (*(void **)(bp + DSIZE))
#define GET_PREVP(bp) (*(void **)bp)
#define SET_NEXTP(bp, ptr) (GET_NEXTP(bp) = ptr)
#define SET_PREVP(bp, ptr) (GET_PREVP(bp) = ptr)
#define GET_SEGI(seg_list, i) (*(void **)(seg_list + (i * DSIZE)))
#define SET_SEGI(seglist, i, ptr) ((GET_SEGI(seg_list, i)) = ptr)

#define MIN_BLOCK_SIZE (3 * DSIZE)
#define SEG_LIST_SIZE_DIFF 32
#define NO_OF_LISTS 15

static char *heap_listp = 0;
static char *free_listp = 0;
static char *seg_list = 0;
static int temp;

#define SIZE_T_SIZE (ALIGN(sizeof(size_t)))
#define SIZE_PTR(p) ((size_t *)(((char *)(p)) - SIZE_T_SIZE))

static void *extend_heap(size_t words);
static void place(void *bp, size_t asize);
static void *add_free_list_lifo(void *bp);
static void delete_free_list(void *bp);
static void *coalesce_block(void *bp);
static unsigned int get_list_no(size_t asize);
static void *find_fit_segregated(size_t asize);

int my_init() {
    heap_listp = NULL;
    free_listp = NULL;
    seg_list = NULL;

    if ((seg_list = (char *)mem_sbrk(NO_OF_LISTS * DSIZE)) == (void *)-1)
        return -1;

    for (int i = 0; i < NO_OF_LISTS; ++i) {
        SET_SEGI(seg_list, i, NULL);
    }

    if ((heap_listp = (char *)mem_sbrk(4 * WSIZE)) == (void *)-1) return -1;
    PUT(heap_listp, 0);
    PUT(heap_listp + (1 * WSIZE), PACK(DSIZE, 1));
    PUT(heap_listp + (2 * WSIZE), PACK(DSIZE, 1));
    PUT(heap_listp + (3 * WSIZE), PACK(0, 1));
    heap_listp += (2 * WSIZE);

    if (extend_heap(CHUNKSIZE / WSIZE) == NULL) {
        return -1;
    }
    return 0;
}

void *my_malloc(size_t size) {
    size_t asize;
    size_t extendsize;
    char *bp;
    if (heap_listp == 0) {
        my_init();
    }
    if (size == 0) return NULL;

    if (size <= DSIZE) {
        asize = 3 * DSIZE;
    } else {
        asize = ALIGN(DSIZE + size);
    }
    if ((bp = (char *)find_fit_segregated(asize)) != NULL) {
        place(bp, asize);
        return bp;
    }
    extendsize = MAX(asize, CHUNKSIZE);
    if ((bp = (char *)extend_heap(extendsize / WSIZE)) == NULL) return NULL;
    place(bp, asize);
    return bp;
}

void my_free(void *ptr) {
    if (ptr == 0) return;
    size_t size = GET_SIZE(HDRP(ptr));
    if (heap_listp == 0) {
        my_init();
    }
    PUT(HDRP(ptr), PACK(size, 0));
    PUT(FTRP(ptr), PACK(size, 0));
    ptr = add_free_list_lifo(ptr);
}

void *my_realloc(void *ptr, size_t size) {
    size_t oldsize;
    void *newptr;

    if (size == 0) {
        my_free(ptr);
        return 0;
    }
    if (ptr == NULL) {
        return my_malloc(size);
    }
    newptr = my_malloc(size);
    if (!newptr) {
        return 0;
    }
    oldsize = GET_SIZE(HDRP(ptr));
    if (size < oldsize) oldsize = size;
    memcpy(newptr, ptr, oldsize);
    my_free(ptr);
    return newptr;
}

static inline void *coalesce_block(void *bp) {
    size_t prev_alloc = GET_ALLOC(FTRP(PREV_BLKP(bp)));
    size_t next_alloc = GET_ALLOC(HDRP(NEXT_BLKP(bp)));
    size_t size = GET_SIZE(HDRP(bp));
    if (size == 0) {
    }

    if (prev_alloc && next_alloc) {
        return bp;
    } else if (prev_alloc && !next_alloc) {
        size += GET_SIZE(HDRP(NEXT_BLKP(bp)));
        delete_free_list(NEXT_BLKP(bp));
        PUT(HDRP(bp), PACK(size, 0));
        PUT(FTRP(bp), PACK(size, 0));
    } else if (!prev_alloc && next_alloc) {
        bp = PREV_BLKP(bp);
        size += GET_SIZE(HDRP(bp));
        delete_free_list(bp);
        PUT(HDRP(bp), PACK(size, 0));
        PUT(FTRP(bp), PACK(size, 0));
    } else {
        size += GET_SIZE(HDRP(PREV_BLKP(bp))) + GET_SIZE(FTRP(NEXT_BLKP(bp)));
        delete_free_list(PREV_BLKP(bp));
        delete_free_list(NEXT_BLKP(bp));
        PUT(HDRP(PREV_BLKP(bp)), PACK(size, 0));
        PUT(FTRP(NEXT_BLKP(bp)), PACK(size, 0));
        bp = PREV_BLKP(bp);
    }
    return bp;
}

static inline void *extend_heap(size_t words) {
    char *bp;
    size_t size;

    size = (words % 2) ? (words + 1) * WSIZE : words * WSIZE;
    if ((long)(bp = (char *)mem_sbrk(size)) == -1) return NULL;

    PUT(HDRP(bp), PACK(size, 0));
    PUT(FTRP(bp), PACK(size, 0));
    PUT(HDRP(NEXT_BLKP(bp)), PACK(0, 1));
    bp = (char *)add_free_list_lifo(bp);
    return bp;
}

static inline void place(void *bp, size_t asize) {
    size_t csize = GET_SIZE(HDRP(bp));
    int prev_alloc = 0, next_alloc = 0;
    delete_free_list(bp);
    prev_alloc = GET_PREV_ALLOC(PREV_BLKP(bp));
    next_alloc = GET_NEXT_ALLOC(NEXT_BLKP(bp));
    if ((csize - asize) >= MIN_BLOCK_SIZE) {
        PUT(HDRP(bp), PACK(asize, (next_alloc | prev_alloc | 1)));
        PUT(FTRP(bp), PACK(asize, (next_alloc | prev_alloc | 1)));
        bp = NEXT_BLKP(bp);
        PUT(HDRP(bp), PACK(csize - asize, 0));
        PUT(FTRP(bp), PACK(csize - asize, 0));
        bp = add_free_list_lifo(bp);
    } else {
        PUT(HDRP(bp), PACK(csize, (next_alloc | prev_alloc | 1)));
        PUT(FTRP(bp), PACK(csize, (next_alloc | prev_alloc | 1)));
    }
}

static inline void *find_fit_segregated(size_t asize) {
    void *bp = 0;
    unsigned int list_no = get_list_no(asize);

    for (int i = list_no; i < NO_OF_LISTS; i++) {
        for (bp = GET_SEGI(seg_list, i); (bp != NULL) && GET_SIZE(HDRP(bp)) > 0;
             bp = GET_NEXTP(bp)) {
            if (bp != NULL && (asize <= GET_SIZE(HDRP(bp)))) {
                return bp;
            }
        }
    }
    return NULL;
}

static void *add_free_list_lifo(void *bp) {
    int list_no;

    bp = coalesce_block(bp);

    list_no = get_list_no(GET_SIZE(HDRP(bp)));
    if (GET_SEGI(seg_list, list_no) == NULL) {
        SET_PREVP(bp, NULL);
        SET_NEXTP(bp, NULL);
    } else if (GET_SEGI(seg_list, list_no) != NULL) {
        SET_NEXTP(bp, GET_SEGI(seg_list, list_no));
        SET_PREVP(bp, NULL);
        SET_PREVP(GET_SEGI(seg_list, list_no), bp);
    }

    SET_SEGI(seg_list, list_no, bp);

    return bp;
}

static inline void delete_free_list(void *bp) {
    int list_no = get_list_no(GET_SIZE(HDRP(bp)));
    temp++;
    void *next = GET_NEXTP(bp);
    void *prev = GET_PREVP(bp);
    if (bp == GET_SEGI(seg_list, list_no)) {
        SET_SEGI(seg_list, list_no, next);
    }

    if (prev != NULL) {
        SET_NEXTP(prev, next);
    }

    if (next != NULL) {
        SET_PREVP(next, prev);
    }

    SET_NEXTP(bp, NULL);
    SET_PREVP(bp, NULL);
}

static inline unsigned int get_list_no(size_t asize) {
    if (asize <= 24)
        return 0;
    else if (asize <= 48)
        return 1;
    else if (asize <= 72)
        return 2;
    else if (asize <= 96)
        return 3;
    else if (asize <= 120)
        return 4;
    else if (asize <= 240)
        return 5;
    else if (asize <= 480)
        return 6;
    else if (asize <= 960)
        return 7;
    else if (asize <= 1920)
        return 8;
    else if (asize <= 3840)
        return 9;
    else if (asize <= 7680)
        return 10;
    else if (asize <= 15360)
        return 11;
    else if (asize <= 30720)
        return 12;
    else if (asize <= 61440)
        return 13;
    else
        return 14;
}