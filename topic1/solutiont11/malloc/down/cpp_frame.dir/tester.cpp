#include <assert.h>
#include <errno.h>
#include <float.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/time.h>
#include <time.h>
#include <unistd.h>

#include "solve.h"
#include "tester.h"
#include "utils.h"

int main(int argc, char **argv) {
    int opt;
    char **datafiles = NULL;
    int num_datafiles = 0;
    range_t *ranges = NULL;
    stats_t *my_stats = NULL;
    speed_t speed_params;

    while ((opt = getopt(argc, argv, "f:c:hv")) != EOF) {
        switch (opt) {
            case 'f':
                num_datafiles = 1;
                if ((datafiles = (char **)realloc(datafiles,
                                                  2 * sizeof(char *))) == NULL)
                    unix_error("ERROR: realloc failed in main");
                strcpy(datadir, "./");
                datafiles[0] = strdup(optarg);
                datafiles[1] = NULL;
                break;

            case 'c':
                num_datafiles = 1;
                onetime_flag = 1;
                if ((datafiles = (char **)realloc(datafiles,
                                                  2 * sizeof(char *))) == NULL)
                    unix_error("ERROR: realloc failed in main");
                strcpy(datadir, "./");
                datafiles[0] = strdup(optarg);
                datafiles[1] = NULL;
                break;
            case 'v':
                verbose = 2;
                break;
            case 'h':
                usage();
                exit(0);
            default:
                usage();
                exit(1);
        }
    }

    if (datafiles == NULL) {
        datafiles = const_cast<char **>(default_datafiles);
        num_datafiles = sizeof(default_datafiles) / sizeof(char *) - 1;
        printf("Using default datafiles in %s\n", datadir);
    }

    init_random_data();

    if (verbose > 1) {
        printf("Testing you malloc\n");
    }

    my_stats = (stats_t *)calloc((size_t)num_datafiles, sizeof(stats_t));
    if (my_stats == NULL) unix_error("my_stats calloc in main failed");

    run_tests(num_datafiles, datadir, datafiles, my_stats, ranges,
              &speed_params);

    if (onetime_flag) {
        printf("Correctness check finished, by running testfile \"%s\".\n",
               datafiles[num_datafiles - 1]);
        if (my_stats[num_datafiles - 1].valid) {
            printf(" => correct. \n");
        } else {
            printf(" => incorrect. \n");
        }
    } else {
        printf("\nResults for your malloc:\n");
        printresults(num_datafiles, my_stats);
        printf("\n");
    }

    return 0;
}

static void run_tests(int num_tracefiles, const char *tracedir,
                      char **tracefiles, stats_t *mm_stats, range_t *ranges,
                      speed_t *speed_para) {
    volatile int i;
    for (i = 0; i < num_tracefiles; ++i) {
        mem_init();
        trace_t *trace;
        trace = read_trace(&mm_stats[i], tracedir, tracefiles[i]);
        strcpy(mm_stats[i].filename, trace->filename);
        mm_stats[i].ops = trace->num_ops;
        if (verbose > 1) printf("Checking for correctness, ");

        mm_stats[i].valid = eval_mm_valid(trace, &ranges);
        if (onetime_flag) {
            free_trace(trace);
            return;
        }
        if (mm_stats[i].valid) {
            if (verbose > 1) printf("utilization, ");
            mm_stats[i].util = eval_mm_util(trace, i);

            speed_para->trace = trace;
            speed_para->ranges = ranges;

            if (verbose > 1) printf("and time.\n");
            mm_stats[i].secs = timer_gettod(eval_mm_speed, speed_para, 10);
        }

        free_trace(trace);
        mem_deinit();
    }
}

static int add_range(range_t **ranges, char *lo, int size, const trace_t *trace,
                     int opnum, int index) {
    char *hi = lo + size - 1;
    range_t *p;
    assert(size > 0);

    if (!IS_ALIGNED(lo)) {
        sprintf(msg, "Payload address (%p) not aligned to %d bytes", lo,
                ALIGNMENT);
        malloc_error(trace, opnum, msg);
        return 0;
    }

    if ((lo < (char *)mem_heap_lo()) || (lo > (char *)mem_heap_hi()) ||
        (hi < (char *)mem_heap_lo()) || (hi > (char *)mem_heap_hi())) {
        sprintf(msg, "Payload (%p:%p) lies outside heap (%p:%p)", lo, hi,
                mem_heap_lo(), mem_heap_hi());
        malloc_error(trace, opnum, msg);
        return 0;
    }

    for (p = *ranges; p != NULL; p = p->next) {
        if ((lo >= p->lo && lo <= p->hi) || (hi >= p->lo && hi <= p->hi)) {
            sprintf(msg, "Payload (%p:%p) overlaps another payload (%p:%p)\n",
                    lo, hi, p->lo, p->hi);
            malloc_error(trace, opnum, msg);
            return 0;
        }
    }

    if ((p = (range_t *)malloc(sizeof(range_t))) == NULL)
        unix_error("malloc error in add_range");
    p->next = *ranges;
    p->lo = lo;
    p->hi = hi;
    p->index = index;
    *ranges = p;

    return 1;
}

static void remove_range(range_t **ranges, char *lo) {
    range_t *p;
    range_t **prevpp = ranges;

    for (p = *ranges; p != NULL; p = p->next) {
        if (p->lo == lo) {
            *prevpp = p->next;
            free(p);
            break;
        }
        prevpp = &(p->next);
    }
}

static void clear_ranges(range_t **ranges) {
    range_t *p;
    range_t *pnext;

    for (p = *ranges; p != NULL; p = pnext) {
        pnext = p->next;
        free(p);
    }
    *ranges = NULL;
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
            malloc_error(trace, opnum, msg);
        }
        return 1;
    }
    return 0;
}

static trace_t *read_trace(stats_t *stats, const char *tracedir,
                           const char *filename) {
    FILE *tracefile;
    trace_t *trace;
    char type[MAXLINE];
    int index, size;
    int max_index = 0;
    int op_index;

    if (verbose > 1) {
        printf("Read testfile: %s\n", filename);
    }

    if ((trace = (trace_t *)malloc(sizeof(trace_t))) == NULL)
        unix_error("malloc 1 failed in read_trace");

    strcpy(trace->filename, tracedir);
    strcat(trace->filename, filename);
    if ((tracefile = fopen(trace->filename, "r")) == NULL) {
        sprintf(msg, "Could not open %s in read_trace", trace->filename);
        unix_error(msg);
    }
    fscanf(tracefile, "%d", &trace->num_ids);
    fscanf(tracefile, "%d", &trace->num_ops);

    if ((trace->ops =
             (traceop_t *)malloc(trace->num_ops * sizeof(traceop_t))) == NULL)
        unix_error("malloc 2 failed in read_trace");

    if ((trace->blocks = (char **)malloc(trace->num_ids * sizeof(char *))) ==
        NULL)
        unix_error("malloc 3 failed in read_trace");

    if ((trace->block_sizes =
             (size_t *)malloc(trace->num_ids * sizeof(size_t))) == NULL)
        unix_error("malloc 4 failed in read_trace");

    if ((trace->block_rand_base = (int *)calloc(
             trace->num_ids, sizeof(*trace->block_rand_base))) == NULL)
        unix_error("malloc 5 failed in read_trace");

    index = 0;
    op_index = 0;
    while (fscanf(tracefile, "%s", type) != EOF) {
        switch (type[0]) {
            case 'a':
                fscanf(tracefile, "%u %u", &index, &size);
                trace->ops[op_index].type = ALLOC;
                trace->ops[op_index].index = index;
                trace->ops[op_index].size = size;
                max_index = (index > max_index) ? index : max_index;
                break;
            case 'r':
                fscanf(tracefile, "%u %u", &index, &size);
                trace->ops[op_index].type = REALLOC;
                trace->ops[op_index].index = index;
                trace->ops[op_index].size = size;
                max_index = (index > max_index) ? index : max_index;
                break;
            case 'f':
                fscanf(tracefile, "%ud", &index);
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
    fclose(tracefile);
    assert(max_index == trace->num_ids - 1);
    assert(trace->num_ops == op_index);
    strcpy(stats->filename, trace->filename);
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

static int eval_mm_valid(trace_t *trace, range_t **ranges) {
    int i;
    int index;
    int size;
    char *newp;
    char *oldp;
    char *p;

    mem_reset_brk();
    clear_ranges(ranges);
    reinit_trace(trace);

    if (my_init() < 0) {
        malloc_error(trace, 0, "my_init failed.");
        return 0;
    }
    for (i = 0; i < trace->num_ops; i++) {
        index = trace->ops[i].index;
        size = trace->ops[i].size;

        switch (trace->ops[i].type) {
            case ALLOC:
                if ((p = (char *)my_malloc((size_t)size)) == NULL) {
                    malloc_error(trace, i, "my_malloc failed.");
                    return 0;
                }
                if (add_range(ranges, p, size, trace, i, index) == 0) return 0;
                trace->blocks[index] = p;
                trace->block_sizes[index] = (size_t)size;

                randomize_block(trace, index);

                break;
            case REALLOC:

                oldp = trace->blocks[index];
                newp = (char *)my_realloc(oldp, size);
                if (newp == NULL && size != 0) {
                    malloc_error(trace, i, "my_realloc failed.");
                    return 0;
                }

                if (newp != NULL && size == 0) {
                    malloc_error(trace, i,
                                 "my_realloc with size 0 returned non-NULL.");
                    return 0;
                }

                remove_range(ranges, oldp);

                if (size > 0) {
                    if (add_range(ranges, newp, size, trace, i, index) == 0)
                        return 0;
                }

                trace->blocks[index] = newp;
                if ((size_t)size < trace->block_sizes[index]) {
                    trace->block_sizes[index] = size;
                }
                if (check_index(trace, i, index) == 0) {
                    trace->block_sizes[index] = size;
                    randomize_block(trace, index);
                } else {
                    malloc_error(trace, i,
                                 "my_realloc did not preserve the "
                                 "data from old block");
                    return 0;
                }

                break;

            case FREE:
                if (check_index(trace, i, index) != 0) {
                    malloc_error(trace, i, "check index in free failed");
                    return 0;
                }
                if (index == -1) {
                    p = 0;
                } else {
                    p = trace->blocks[index];
                    remove_range(ranges, p);
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
    if (my_init() < 0) app_error("my_init failed in eval_mm_util");
    for (i = 0; i < trace->num_ops; i++) {
        switch (trace->ops[i].type) {
            case ALLOC:
                index = trace->ops[i].index;
                size = trace->ops[i].size;

                if ((p = (char *)my_malloc(size)) == NULL)
                    app_error("my_malloc failed in eval_mm_util");
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
                if ((newp = (char *)my_realloc(oldp, newsize)) == NULL &&
                    newsize != 0)
                    app_error("my_realloc failed in eval_mm_util");

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
    if (my_init() < 0) app_error("mm_init failed in eval_mm_speed");

    for (i = 0; i < trace->num_ops; i++) switch (trace->ops[i].type) {
            case ALLOC:
                index = trace->ops[i].index;
                size = trace->ops[i].size;
                if ((p = (char *)my_malloc(size)) == NULL)
                    app_error("mm_malloc error in eval_mm_speed");
                trace->blocks[index] = p;
                break;

            case REALLOC:
                index = trace->ops[i].index;
                newsize = trace->ops[i].size;
                oldp = trace->blocks[index];
                if ((newp = (char *)my_realloc(oldp, newsize)) == NULL &&
                    newsize != 0)
                    app_error("mm_realloc error in eval_mm_speed");
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

static void printresults(int n, stats_t *stats) {
    int i;
    printf("%4s%10s%10s%10s %s\n", "id", " valid", "util", "secs", "filename");
    for (i = 0; i < n; i++) {
        if (stats[i].valid) {
            printf("%4d%10s%10.6f%10.6f %s\n", i, "yes", stats[i].util,
                   stats[i].secs, stats[i].filename);

        } else {
            printf("%4d%10s%10s%10s %s\n", i, "no", "-", "-",
                   stats[i].filename);
        }
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

void malloc_error(const trace_t *trace, int opnum, const char *msg) {
    printf("ERROR [Testfile %s, line %d]: %s\n", trace->filename, LINENUM(opnum),
           msg);
}

static void usage(void) {
    fprintf(stderr, "Usage: tester [-h] [-v] [-f/c <file>]\n");
    fprintf(stderr, "Options\n");
    fprintf(stderr, "\t-f <file>  Use <file> as the test file.\n");
    fprintf(stderr,
            "\t-c <file>  Run test file <file> once, check for correctness "
            "only.\n");
    fprintf(stderr, "\t-v         Print extra info as each testfile is run.\n");
    fprintf(stderr, "\t-h         Print help.\n");
}

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