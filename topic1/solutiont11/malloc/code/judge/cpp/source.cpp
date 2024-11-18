#include <assert.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "solve.h"
#include "utils.h"

#define ALIGNMENT 8

#define ALIGN(size) (((size) + (ALIGNMENT - 1)) & ~0x7)

#define SIZE_T_SIZE (ALIGN(sizeof(size_t)))

#define SIZE_PTR(p) ((size_t *)(((char *)(p)) - SIZE_T_SIZE))

int my_init() {
    // you code here

    return 0;
}

void *my_malloc(size_t size) {
    /*
     * In this naive approach, a block is allocated by simply incrementing
     * the brk pointer.  Blocks are never coalesced or reused.  The size of
     * a block is found at the first aligned word before the block (we need
     * it for realloc).
     *
     * This code is correct and blazingly fast, but very bad usage-wise since
     * it never frees anything.
     */

    // you code here
    int newsize = ALIGN(size + SIZE_T_SIZE);
    unsigned char *p = (unsigned char *)mem_sbrk(newsize);

    if ((long)p < 0)
        return NULL;
    else {
        p += SIZE_T_SIZE;
        *SIZE_PTR(p) = size;
        return p;
    }
}

void my_free(void *ptr) {
    // your code here
}

void *my_realloc(void *ptr, size_t size) {
    size_t oldsize;
    void *newptr;

    /* If size == 0 then this is just free, and we return NULL. */
    if (size == 0) {
        my_free(ptr);
        return 0;
    }

    /* If oldptr is NULL, then this is just malloc. */
    if (ptr == NULL) {
        return my_malloc(size);
    }

    newptr = my_malloc(size);

    /* If realloc() fails the original block is left untouched  */
    if (!newptr) {
        return 0;
    }

    /* Copy the old data. */
    oldsize = *SIZE_PTR(ptr);
    if (size < oldsize) oldsize = size;
    memcpy(newptr, ptr, oldsize);

    /* Free the old block. */
    my_free(ptr);

    return newptr;
}
