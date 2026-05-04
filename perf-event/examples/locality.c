/* Observe the L1 data cache hit rate under two different access patterns.

   C equivalent of locality.rs, using perf_event_open directly.
   Written by Claude Code.

   Compile with optimization for meaningful results:
       cc -O2 -o locality locality.c && ./locality
*/

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <unistd.h>
#include <sys/ioctl.h>
#include <sys/syscall.h>
#include <linux/perf_event.h>

#define SIZE 10000000

/* XorShift128+ PRNG: https://en.wikipedia.org/wiki/Xorshift#xorshift+ */
typedef struct { uint64_t s[2]; } XorShift128Plus;

static uint64_t xorshift128plus_next(XorShift128Plus *rng) {
    uint64_t t = rng->s[0];
    uint64_t s = rng->s[1];
    rng->s[0] = s;
    t ^= t << 23;
    t ^= t >> 18;
    t ^= s ^ (s >> 5);
    rng->s[1] = t;
    return t + s;
}

static long perf_event_open(struct perf_event_attr *attr, pid_t pid,
                            int cpu, int group_fd, unsigned long flags) {
    return syscall(__NR_perf_event_open, attr, pid, cpu, group_fd, flags);
}

/* Follow the chain encoded in indices[], starting at 0, until we return to 0.
   Returns the number of steps taken. */
__attribute__((noinline))
static size_t walk(size_t *indices) {
    size_t count = 0;
    size_t i = 0;
    do {
        count++;
        i = indices[i];
    } while (i != 0);
    return count;
}

static int open_cache_counter(int group_fd, int which, int op, int result) {
    struct perf_event_attr attr = { .size = sizeof(attr) };
    attr.type   = PERF_TYPE_HW_CACHE;
    attr.config = (uint64_t)which | ((uint64_t)op << 8) | ((uint64_t)result << 16);

    attr.exclude_kernel = 1;
    attr.exclude_hv     = 1;

    if (group_fd == -1) {
        /* Group leader: start disabled, and carry time/group info on read. */
        attr.disabled    = 1;
        attr.read_format = PERF_FORMAT_GROUP
                         | PERF_FORMAT_TOTAL_TIME_ENABLED
                         | PERF_FORMAT_TOTAL_TIME_RUNNING;
    }

    int fd = (int)perf_event_open(&attr, 0 /* this process */, -1 /* any cpu */,
                                  group_fd, 0);
    if (fd < 0) { perror("perf_event_open"); exit(1); }
    return fd;
}

static void measure(const char *label, size_t *vec) {
    int leader_fd   = open_cache_counter(-1,
                          PERF_COUNT_HW_CACHE_L1D,
                          PERF_COUNT_HW_CACHE_OP_READ,
                          PERF_COUNT_HW_CACHE_RESULT_ACCESS);
    int miss_fd     = open_cache_counter(leader_fd,
                          PERF_COUNT_HW_CACHE_L1D,
                          PERF_COUNT_HW_CACHE_OP_READ,
                          PERF_COUNT_HW_CACHE_RESULT_MISS);
    int prefetch_fd = open_cache_counter(leader_fd,
                          PERF_COUNT_HW_CACHE_L1D,
                          PERF_COUNT_HW_CACHE_OP_PREFETCH,
                          PERF_COUNT_HW_CACHE_RESULT_ACCESS);

    ioctl(leader_fd, PERF_EVENT_IOC_RESET,   PERF_IOC_FLAG_GROUP);
    ioctl(leader_fd, PERF_EVENT_IOC_ENABLE,  PERF_IOC_FLAG_GROUP);

    volatile size_t result = walk(vec);
    (void)result;

    ioctl(leader_fd, PERF_EVENT_IOC_DISABLE, PERF_IOC_FLAG_GROUP);

    /* PERF_FORMAT_GROUP read layout:
         uint64_t nr;            (number of counters = 3)
         uint64_t time_enabled;
         uint64_t time_running;
         uint64_t values[3];     (reads, misses, prefetches)  */
    struct {
        uint64_t nr;
        uint64_t time_enabled;
        uint64_t time_running;
        uint64_t values[3];
    } counts;

    if (read(leader_fd, &counts, sizeof(counts)) != (ssize_t)sizeof(counts)) {
        perror("read"); exit(1);
    }

    uint64_t reads      = counts.values[0];
    uint64_t misses     = counts.values[1];
    uint64_t prefetches = counts.values[2];
    uint64_t hits       = reads - misses;

    printf("%s: hits / reads: %8lu / %8lu  %6.2f%%, prefetched %8lu\n",
           label, (unsigned long)hits, (unsigned long)reads,
           (double)hits / (double)reads * 100.0,
           (unsigned long)prefetches);

    if (counts.time_enabled != counts.time_running)
        printf("time enabled: %lu  time running: %lu\n",
               (unsigned long)counts.time_enabled,
               (unsigned long)counts.time_running);

    close(leader_fd);
    close(miss_fd);
    close(prefetch_fd);
}

int main(void) {
    size_t *vec = malloc(SIZE * sizeof(size_t));
    if (!vec) { perror("malloc"); return 1; }

    /* Linear chain: vec[0]=1, vec[1]=2, ..., vec[SIZE-2]=SIZE-1, vec[SIZE-1]=0 */
    for (size_t i = 0; i < SIZE - 1; i++)
        vec[i] = i + 1;
    vec[SIZE - 1] = 0;

    measure("linear", vec);

    /* Random chain: Fisher-Yates shuffle starting from identity permutation,
       producing a single cycle that visits every element before returning to 0. */
    XorShift128Plus rng = { .s = {1729, 42} };
    for (int i = 0; i < 100; i++)   /* propagate 1-bits in state */
        xorshift128plus_next(&rng);

    for (size_t i = 0; i < SIZE; i++)
        vec[i] = i;

    for (size_t i = 0; i < SIZE - 1; i++) {
        size_t remaining = SIZE - 1 - i;
        size_t j = i + 1 + xorshift128plus_next(&rng) % remaining;
        size_t tmp = vec[i]; vec[i] = vec[j]; vec[j] = tmp;
    }

    measure("random", vec);

    free(vec);
    return 0;
}
