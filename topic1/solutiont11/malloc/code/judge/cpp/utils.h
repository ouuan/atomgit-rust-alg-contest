//
// Created by Lincheng Ge on 2018/8/26.
//

#include <unistd.h>

#define ALIGNMENT 8

#define MAX_HEAP (100 * (1 << 20))

typedef void (*ftimer_test_funct)(void *);

double timer_gettod(ftimer_test_funct f, void *argp, int n);

void mem_init();

void mem_deinit();

void *mem_sbrk(int incr);

void mem_reset_brk();

void *mem_heap_lo();

void *mem_heap_hi();

size_t mem_heapsize();

size_t mem_pagesize();
