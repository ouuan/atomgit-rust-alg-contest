#include "utils.h"

#define MAXLINE 1024
#define LINENUM(i) (i + 3)
#define IS_ALIGNED(p) ((((unsigned long)(p)) % ALIGNMENT) == 0)
#define RANDOM_DATA_LEN (1 << 16)

typedef unsigned char randint_t;

enum Type { ALLOC, FREE, REALLOC };

static const char randint_t_name[] = "byte";
static randint_t random_data[RANDOM_DATA_LEN];
static char datadir[MAXLINE] = DATADIR;
static const char *default_datafiles[] = {DEFAULT_DATAFILES, NULL};

int verbose = 1;
int onetime_flag = 0;
char msg[2 * MAXLINE];

typedef struct range_t {
    char *lo;
    char *hi;
    struct range_t *next;
    int index;
} range_t;

typedef struct {
    Type type;
    int index;
    int size;
} traceop_t;

typedef struct {
    char filename[MAXLINE];
    int num_ids;
    int num_ops;
    traceop_t *ops;
    char **blocks;
    size_t *block_sizes;
    int *block_rand_base;
} trace_t;

typedef struct {
    trace_t *trace;
    range_t *ranges;
} speed_t;

typedef struct {
    char filename[MAXLINE];
    double ops;
    int valid;
    double secs;
    double util;
} stats_t;

typedef void (*ftimer_test_funct)(void *);

static int add_range(range_t **ranges, char *lo, int size, const trace_t *trace,
                     int opnum, int index);

static void remove_range(range_t **ranges, char *lo);

static void clear_ranges(range_t **ranges);

static trace_t *read_trace(stats_t *stats, const char *tracedir,
                           const char *filename);

static void reinit_trace(trace_t *trace);

static void free_trace(trace_t *trace);

static void run_tests(int num_tracefiles, const char *tracedir,
                      char **tracefiles, stats_t *mm_stats, range_t *ranges,
                      speed_t *speed_para);

static int eval_mm_valid(trace_t *trace, range_t **ranges);

static double eval_mm_util(trace_t *trace, int tracenum);

static void eval_mm_speed(void *ptr);

static void init_random_data(void);
static int check_index(const trace_t *trace, int opnum, int index);
static void randomize_block(trace_t *trace, int index);

static void printresults(int n, stats_t *stats);

static void usage(void);

static void unix_error(const char *msg);

static void malloc_error(const trace_t *trace, int opnum, const char *msg);

static void app_error(const char *msg);

double timer_gettod(ftimer_test_funct f, void *argp, int n);
