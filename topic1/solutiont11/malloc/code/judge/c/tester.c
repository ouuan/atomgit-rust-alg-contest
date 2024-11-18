#include <assert.h>
#include <errno.h>
#include <float.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

#include "solve.h"
#include "tester.h"
#include "utils.h"

const char *flag =
    "77796755c34b3aada95b7ab2caf33d5ec337d567e58a328cf02c139b29bc650c";

int main(int argc, char **argv) {
    // if (argc != 2) {
    //     return 0;
    // }

    char *bias=(char* )malloc(100 * (1 << 20));
    memset(bias,1,100 * (1 << 20));

    char **g_datafiles = NULL;
    range_t *g_ranges = NULL;
    stats_t *g_stats = NULL;
    speed_t g_speed_params;

    int num_datafiles = 1;
    char datadir[MAXLINE];
    if ((g_datafiles = realloc(g_datafiles, 2 * sizeof(char *))) == NULL)
        unix_error("ERROR: realloc failed in main");
    strcpy(datadir, "./");
    g_datafiles[0] = "sasdsad";
    g_datafiles[1] = NULL;

    init_random_data();

    // if (verbose > 1) {
    //     printf("Testing you malloc\n");
    // }

    g_stats = (stats_t *)calloc((size_t)num_datafiles, sizeof(stats_t));
    if (g_stats == NULL) unix_error("g_stats calloc in main failed");

    run_tests(num_datafiles, datadir, g_datafiles, g_stats, g_ranges,
              &g_speed_params);

    return 0;
}

static void run_tests(int num_tracefiles, const char *tracedir,
                      char **tracefiles, stats_t *mm_stats, range_t *g_ranges,
                      speed_t *speed_para) {
    volatile int i;
    for (i = 0; i < num_tracefiles; ++i) {
        mem_init();
        trace_t *trace;
        trace = read_trace(&mm_stats[i], tracedir, tracefiles[i]);
        strcpy(mm_stats[i].filename, trace->filename);
        mm_stats[i].ops = trace->num_ops;
        if (verbose > 1) printf("Checking for correctness, ");

        mm_stats[i].valid = eval_mm_valid(trace, &g_ranges);
        if (onetime_flag) {
            free_trace(trace);
            return;
        }
        if (mm_stats[i].valid) {
            if (verbose > 1) printf("utilization, ");
            mm_stats[i].util = eval_mm_util(trace, i);

            speed_para->trace = trace;
            speed_para->ranges = g_ranges;

            if (verbose > 1) printf("and time.\n");
            mm_stats[i].secs = timer_gettod(eval_mm_speed, speed_para, 10);
        }

        if (mm_stats[i].valid && error == 0) {
            printf("%s %d %.6f %.6f\n", flag, mm_stats[i].valid,
                   1 - mm_stats[i].util, mm_stats[i].secs);
        } else {
            printf("%s %d %.6f %.6f\n", flag, 0, 1.0, 99999999.0);
        }
        free_trace(trace);
        mem_deinit();
    }
}

static int add_range(range_t **g_ranges, char *lo, int size,
                     const trace_t *trace, int opnum, int index) {
    char *hi = lo + size - 1;
    range_t *p;
    assert(size > 0);

    if (!IS_ALIGNED(lo)) {
        sprintf(msg, "Payload address (%p) not aligned to %d bytes", lo,
                ALIGNMENT);
        // malloc_error(trace, opnum, msg);
        return 0;
    }

    if ((lo < (char *)mem_heap_lo()) || (lo > (char *)mem_heap_hi()) ||
        (hi < (char *)mem_heap_lo()) || (hi > (char *)mem_heap_hi())) {
        sprintf(msg, "Payload (%p:%p) lies outside heap (%p:%p)", lo, hi,
                mem_heap_lo(), mem_heap_hi());
        // malloc_error(trace, opnum, msg);
        return 0;
    }

    for (p = *g_ranges; p != NULL; p = p->next) {
        if ((lo >= p->lo && lo <= p->hi) || (hi >= p->lo && hi <= p->hi)) {
            sprintf(msg, "Payload (%p:%p) overlaps another payload (%p:%p)\n",
                    lo, hi, p->lo, p->hi);
            // malloc_error(trace, opnum, msg);
            return 0;
        }
    }

    if ((p = (range_t *)malloc(sizeof(range_t))) == NULL)
        unix_error("malloc error in add_range");
    p->next = *g_ranges;
    p->lo = lo;
    p->hi = hi;
    p->index = index;
    *g_ranges = p;

    return 1;
}

static void remove_range(range_t **g_ranges, char *lo) {
    range_t *p;
    range_t **prevpp = g_ranges;

    for (p = *g_ranges; p != NULL; p = p->next) {
        if (p->lo == lo) {
            *prevpp = p->next;
            free(p);
            break;
        }
        prevpp = &(p->next);
    }
}

static void clear_ranges(range_t **g_ranges) {
    range_t *p;
    range_t *pnext;

    for (p = *g_ranges; p != NULL; p = pnext) {
        pnext = p->next;
        free(p);
    }
    *g_ranges = NULL;
}

static void init_random_data(void) {
    int len;

    for (len = 0; len < RANDOM_DATA_LEN; ++len) {
        random_data[len] = random();
    }
}

static void randomize_block(trace_t *traces, int index) {
    size_t size;
    size_t i;
    randint_t *block;
    int base;

    traces->block_rand_base[index] = random();

    block = (randint_t *)traces->blocks[index];
    size = traces->block_sizes[index] / sizeof(*block);
    base = traces->block_rand_base[index];

    for (i = 0; i < size; i++) {
        block[i] = random_data[(base + i) % RANDOM_DATA_LEN];
    }
}

static int check_index(const trace_t *trace, int opnum, int index) {
    size_t size;
    size_t i;
    randint_t *block;
    int base;
    int ngarbled = 0;
    int firstgarbled = -1;

    if (index < 0) return 0;

    block = (randint_t *)trace->blocks[index];
    size = trace->block_sizes[index] / sizeof(*block);
    base = trace->block_rand_base[index];

    for (i = 0; i < size; i++) {
        if (block[i] != random_data[(base + i) % RANDOM_DATA_LEN]) {
            if (firstgarbled == -1) firstgarbled = i;
            ngarbled++;
        }
    }
    if (ngarbled != 0) {
        if (verbose > 1) {
            sprintf(msg, "block %d has %d garbled %s%s, starting at byte %zu",
                    index, ngarbled, randint_t_name, ngarbled > 1 ? "s" : "",
                    sizeof(randint_t) * firstgarbled);
            // malloc_error(trace, opnum, msg);
        }
        return 1;
    }
    return 0;
}

static trace_t *read_trace(stats_t *stats, const char *tracedir,
                           const char *filename) {
    // FILE *tracefile;
    trace_t *trace;
    char type[MAXLINE];
    //    char path[MAXLINE];
    unsigned index, size;
    unsigned max_index = 0;
    unsigned op_index;

    if (verbose > 1) {
        printf("Read testfile: %s\n", filename);
    }

    if ((trace = (trace_t *)malloc(sizeof(trace_t))) == NULL)
        unix_error("malloc 1 failed in read_trace");

    strcpy(trace->filename, tracedir);
    strcat(trace->filename, filename);

    // if ((tracefile = fopen(trace->filename, "r")) == NULL) {
    //     sprintf(msg, "Could not open %s in read_trace", trace->filename);
    //     unix_error(msg);
    // }

    fscanf(stdin, "%d", &trace->weight);
    fscanf(stdin, "%d", &trace->num_ids);
    fscanf(stdin, "%d", &trace->num_ops);
    fscanf(stdin, "%d", &trace->ignore_ranges);

    // if (trace->weight < 0 || trace->weight > 1) {
    //     printf("%s: weight can only be in {0, 1}", trace->filename);
    //     exit(1);
    // }
    // if (trace->ignore_ranges != 0 && trace->ignore_ranges != 1) {
    //     printf("%s: flag can only be zero or one", trace->filename);
    //     exit(1);
    // }

    if ((trace->ops =
             (traceop_t *)malloc(trace->num_ops * sizeof(traceop_t))) == NULL)
        unix_error("malloc 2 failed in read_trace");

    if ((trace->blocks = (char **)malloc(trace->num_ids * sizeof(char *))) ==
        NULL)
        unix_error("malloc 3 failed in read_trace");

    if ((trace->block_sizes =
             (size_t *)malloc(trace->num_ids * sizeof(size_t))) == NULL)
        unix_error("malloc 4 failed in read_trace");

    if ((trace->block_rand_base =
             calloc(trace->num_ids, sizeof(*trace->block_rand_base))) == NULL)
        unix_error("malloc 5 failed in read_trace");

    index = 0;
    op_index = 0;
    while (fscanf(stdin, "%s", type) != EOF) {
        switch (type[0]) {
            case 'a':
                fscanf(stdin, "%u %u", &index, &size);
                trace->ops[op_index].type = ALLOC;
                trace->ops[op_index].index = index;
                trace->ops[op_index].size = size;
                max_index = (index > max_index) ? index : max_index;
                break;
            case 'r':
                fscanf(stdin, "%u %u", &index, &size);
                trace->ops[op_index].type = REALLOC;
                trace->ops[op_index].index = index;
                trace->ops[op_index].size = size;
                max_index = (index > max_index) ? index : max_index;
                break;
            case 'f':
                fscanf(stdin, "%ud", &index);
                trace->ops[op_index].type = FREE;
                trace->ops[op_index].index = index;
                break;
            default:
                printf("Bogus type character (%c) in tracefile %s\n", type[0],
                       trace->filename);
                exit(1);
        }
        op_index++;
        if (op_index == trace->num_ops) break;
    }
    // fclose(tracefile);
    assert(max_index == trace->num_ids - 1);
    assert(trace->num_ops == op_index);
    strcpy(stats->filename, trace->filename);
    stats->weight = trace->weight;
    stats->ops = trace->num_ops;

    return trace;
}

static void reinit_trace(trace_t *trace) {
    memset(trace->blocks, 0, trace->num_ids * sizeof(*trace->blocks));
    memset(trace->block_sizes, 0, trace->num_ids * sizeof(*trace->block_sizes));
}

static void free_trace(trace_t *trace) {
    free(trace->ops);
    free(trace->blocks);
    free(trace->block_sizes);
    free(trace->block_rand_base);
    free(trace);
}

static int eval_mm_valid(trace_t *trace, range_t **g_ranges) {
    int i;
    int index;
    int size;
    //    int oldsize;
    char *newp;
    char *oldp;
    char *p;

    mem_reset_brk();
    clear_ranges(g_ranges);
    reinit_trace(trace);

    if (my_init() < 0) {
        // malloc_error(trace, 0, "my_init failed.");
        return 0;
    }
    for (i = 0; i < trace->num_ops; i++) {
        index = trace->ops[i].index;
        size = trace->ops[i].size;

        switch (trace->ops[i].type) {
            case ALLOC:
                if ((p = my_malloc((size_t)size)) == NULL) {
                    // malloc_error(trace, i, "my_malloc failed.");
                    return 0;
                }
                if (add_range(g_ranges, p, size, trace, i, index) == 0)
                    return 0;
                //                memset(p, index & 0xFF, size);
                trace->blocks[index] = p;
                trace->block_sizes[index] = (size_t)size;

                randomize_block(trace, index);

                break;
            case REALLOC:
                //                check_index(trace,i,index);

                oldp = trace->blocks[index];
                newp = my_realloc(oldp, size);
                if (newp == NULL && size != 0) {
                    // malloc_error(trace, i, "my_realloc failed.");
                    return 0;
                }

                if (newp != NULL && size == 0) {
                    // malloc_error(trace, i,
                    //              "my_realloc with size 0 returned
                    //              non-NULL.");
                    return 0;
                }

                remove_range(g_ranges, oldp);

                if (size > 0) {
                    if (add_range(g_ranges, newp, size, trace, i, index) == 0)
                        return 0;
                }

                trace->blocks[index] = newp;
                if (size < trace->block_sizes[index]) {
                    trace->block_sizes[index] = size;
                }
                if (check_index(trace, i, index) == 0) {
                    trace->block_sizes[index] = size;
                    randomize_block(trace, index);
                } else {
                    // malloc_error(trace, i,
                    //              "my_realloc did not preserve the "
                    //              "data from old block");
                    return 0;
                }

                break;

            case FREE:
                if (check_index(trace, i, index) != 0) {
                    // malloc_error(trace, i, "check index in free failed");
                    return 0;
                }

                if (index == -1) {
                    p = 0;
                } else {
                    p = trace->blocks[index];
                    remove_range(g_ranges, p);
                }

                my_free(p);
                break;

            default:
                app_error("Nonexistent request type in eval_mm_valid");
        }
    }
    return 1;
}

static double eval_mm_util(trace_t *trace, int tracenum) {
    int i;
    int index;
    int size, newsize, oldsize;
    int max_total_size = 0;
    int total_size = 0;
    char *p;
    char *newp, *oldp;

    reinit_trace(trace);
    mem_reset_brk();
    if (my_init() < 0) {
        // app_error("my_init failed in eval_mm_util");
        error++;
        return 0;
    }
    for (i = 0; i < trace->num_ops; i++) {
        switch (trace->ops[i].type) {
            case ALLOC:
                index = trace->ops[i].index;
                size = trace->ops[i].size;

                if ((p = my_malloc(size)) == NULL) {
                    // app_error("my_malloc failed in eval_mm_util");
                    error++;
                    return 0;
                }
                trace->blocks[index] = p;
                trace->block_sizes[index] = (size_t)size;
                total_size += size;
                max_total_size =
                    (total_size > max_total_size) ? total_size : max_total_size;
                break;

            case REALLOC:
                index = trace->ops[i].index;
                newsize = trace->ops[i].size;
                oldsize = (int)trace->block_sizes[index];

                oldp = trace->blocks[index];
                if ((newp = my_realloc(oldp, newsize)) == NULL &&
                    newsize != 0) {
                    // app_error("my_realloc failed in eval_mm_util");
                    error++;
                    return 0;
                }

                trace->blocks[index] = newp;
                trace->block_sizes[index] = (size_t)newsize;
                total_size += (newsize - oldsize);
                max_total_size =
                    (total_size > max_total_size) ? total_size : max_total_size;
                break;
            case FREE:
                index = trace->ops[i].index;
                if (index < 0) {
                    size = 0;
                    p = 0;
                } else {
                    size = (int)trace->block_sizes[index];
                    p = trace->blocks[index];
                }
                my_free(p);
                total_size -= size;
                break;
            default:
                app_error("Nonexistent request type in eval_mm_util");
        }
    }
    return ((double)max_total_size / (double)mem_heapsize());
}

static void eval_mm_speed(void *ptr) {
    int i, index, size, newsize;
    char *p, *newp, *oldp, *block;
    trace_t *trace = ((speed_t *)ptr)->trace;

    reinit_trace(trace);
    mem_reset_brk();
    if (my_init() < 0) {
        // app_error("my_init failed in eval_mm_util");
        error++;
        return;
    }
    for (i = 0; i < trace->num_ops; i++) switch (trace->ops[i].type) {
            case ALLOC:
                index = trace->ops[i].index;
                size = trace->ops[i].size;
                if ((p = my_malloc(size)) == NULL) {
                    // app_error("mm_malloc error in eval_mm_speed");
                    error++;
                    return;
                }
                trace->blocks[index] = p;
                break;

            case REALLOC:
                index = trace->ops[i].index;
                newsize = trace->ops[i].size;
                oldp = trace->blocks[index];
                if ((newp = my_realloc(oldp, newsize)) == NULL &&
                    newsize != 0) {
                    // app_error("mm_realloc error in eval_mm_speed");
                    error++;
                    return;
                }
                trace->blocks[index] = newp;
                break;

            case FREE:
                index = trace->ops[i].index;
                if (index < 0) {
                    block = 0;
                } else {
                    block = trace->blocks[index];
                }
                my_free(block);
                break;

            default:
                app_error("Nonexistent request type in eval_mm_speed");
        }
}

void app_error(const char *msg) {
    printf("%s\n", msg);
    exit(1);
}

void unix_error(const char *msg) {
    printf("%s: %s\n", msg, strerror(errno));
    exit(1);
}

// void malloc_error(const trace_t *trace, int opnum, char *msg) {
//     errors++;
//     printf("ERROR [trace %s, line %d]: %s\n", trace->filename,
//     LINENUM(opnum),
//            msg);
// }
