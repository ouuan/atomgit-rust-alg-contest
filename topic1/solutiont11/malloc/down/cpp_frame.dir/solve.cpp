#include "solve.h"
#include <assert.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
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
    // your code here,you can remove all the code below if you need.
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
    // your code here,you can remove all the code below if you need.
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

    oldsize = *SIZE_PTR(ptr);
    if (size < oldsize) oldsize = size;
    memcpy(newptr, ptr, oldsize);

    my_free(ptr);

    return newptr;
}
