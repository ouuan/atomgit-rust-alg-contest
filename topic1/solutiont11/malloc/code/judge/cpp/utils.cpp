#include "utils.h"
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/mman.h>
#include <sys/time.h>
#include <string.h>

static char *g_mem_start_brk;
static char *g_mem_brk;
static char *g_mem_max_addr;

double timer_gettod(ftimer_test_funct f, void *argp, int n) {
    int i;
    struct timeval stv, etv;
    double diff;
    gettimeofday(&stv, NULL);
    for (i = 0; i < n; i++) f(argp);
    gettimeofday(&etv, NULL);
    diff = 1E3 * (etv.tv_sec - stv.tv_sec) + 1E-3 * (etv.tv_usec - stv.tv_usec);
    diff /= n;
    return (1E-3 * diff);
}

void mem_init() {
    // int dev_zero = open("/dev/zero", O_RDWR);
    // g_mem_start_brk = (char*)mmap((void *)0x800000000, /* suggested start*/
    // 		MAX_HEAP,				/* length */
    // 		PROT_WRITE,				/* permissions */
    // 		MAP_PRIVATE,			/* private or shared? */
    // 		dev_zero,				/* fd */
    // 		0);						/* offset (dunno)
    // */
    if ((g_mem_start_brk = (char *)malloc(MAX_HEAP)) == NULL) {
        fprintf(stderr, "mem_init error\n");
        exit(1);
    }
    memset(g_mem_start_brk, 1, MAX_HEAP);

    g_mem_max_addr = g_mem_start_brk + MAX_HEAP;
    g_mem_brk = g_mem_start_brk;
}

void mem_deinit() {
    free(g_mem_start_brk);
    // munmap(g_mem_start_brk, MAX_HEAP);
}

void mem_reset_brk() { g_mem_brk = g_mem_start_brk; }

void *mem_sbrk(int incr) {
    char *old_brk = g_mem_brk;
    if ((incr < 0) || ((g_mem_brk + incr) > g_mem_max_addr)) {
        errno = ENOMEM;
        // fprintf(stderr, "ERROR: mem_sbrk failed. Ran out of memory...\n");
        return (void *)-1;
    }
    g_mem_brk += incr;
    return (void *)old_brk;
}

void *mem_heap_lo() { return (void *)g_mem_start_brk; }

void *mem_heap_hi() { return (void *)(g_mem_brk - 1); }

size_t mem_heapsize() { return (size_t)(g_mem_brk - g_mem_start_brk); }

size_t mem_pagesize() { return (size_t)getpagesize(); }
