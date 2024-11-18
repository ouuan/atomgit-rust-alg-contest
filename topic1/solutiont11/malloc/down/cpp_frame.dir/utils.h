#include <unistd.h>

#define DATADIR "./data/"

#define DEFAULT_DATAFILES \
    "1.in", \
    "2.in", \
    "3.in", \
    "4.in", \
    "5.in", \
    "6.in", \
    "7.in", \
    "8.in", \
    "9.in", \
    "10.in", \
    "11.in", \
    "12.in", \
    "13.in", \
    "14.in", \
    "15.in", \
    "16.in", \
    "17.in", \
    "18.in", \
    "19.in", \
    "20.in"

#define ALIGNMENT 8

#define MAX_HEAP (100 * (1 << 20))

void mem_init();

void mem_deinit();

void *mem_sbrk(int incr);

void mem_reset_brk();

void *mem_heap_lo();

void *mem_heap_hi();

size_t mem_heapsize();

