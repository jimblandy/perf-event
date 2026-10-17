// SPDX-License-Identifier: GPL-2.0
/*
 * Reproducer: read() on a perf event can return time_running > time_enabled.
 *
 * __perf_event_read_value() loads event->total_time_enabled and
 * event->total_time_running after perf_event_read() has dropped ctx->lock.
 * If the reading task is preempted between the two loads, event_sched_out()
 * advances both totals, and the reader returns the old time_enabled with
 * the new time_running.
 *
 * Needs a preemptible kernel (preempt=full) to hit reliably. Many threads
 * each open a counter on themselves, enable it, and read it immediately,
 * so time_enabled - time_running is close to zero and any preemption in
 * the window is visible.
 *
 * Build: cc -O2 -pthread -o perf-read-race perf-read-race.c
 *
 * This was initially reported as:
 * https://github.com/jimblandy/perf-event/issues/65
 */
#define _GNU_SOURCE
#include <linux/perf_event.h>
#include <pthread.h>
#include <stdatomic.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/syscall.h>
#include <unistd.h>

#define NR_THREADS	128
#define NR_ITERS	1000000

static atomic_int found;
static atomic_long reads;

struct read_value {
	uint64_t value;
	uint64_t time_enabled;
	uint64_t time_running;
};

static int open_counter(void)
{
	struct perf_event_attr attr;

	memset(&attr, 0, sizeof(attr));
	attr.size = sizeof(attr);
	attr.type = PERF_TYPE_SOFTWARE;
	attr.config = PERF_COUNT_SW_TASK_CLOCK;
	attr.disabled = 1;
	attr.exclude_kernel = 1;
	attr.exclude_hv = 1;
	attr.read_format = PERF_FORMAT_TOTAL_TIME_ENABLED |
			   PERF_FORMAT_TOTAL_TIME_RUNNING;

	/* This thread, any CPU. */
	return syscall(SYS_perf_event_open, &attr, 0, -1, -1, 0);
}

static int check(int fd)
{
	struct read_value rv;

	if (read(fd, &rv, sizeof(rv)) != sizeof(rv)) {
		perror("read");
		exit(1);
	}
	atomic_fetch_add_explicit(&reads, 1, memory_order_relaxed);

	if (rv.time_running > rv.time_enabled) {
		if (atomic_exchange(&found, 1) == 0)
			printf("time_running > time_enabled: enabled=%llu running=%llu (+%llu ns)\n",
			       (unsigned long long)rv.time_enabled,
			       (unsigned long long)rv.time_running,
			       (unsigned long long)(rv.time_running - rv.time_enabled));
		return 1;
	}
	return 0;
}

static void *worker(void *arg)
{
	(void)arg;

	for (int i = 0; i < NR_ITERS && !atomic_load(&found); i++) {
		int fd = open_counter();

		if (fd < 0) {
			perror("perf_event_open");
			exit(1);
		}
		ioctl(fd, PERF_EVENT_IOC_ENABLE, 0);
		check(fd);
		check(fd);
		ioctl(fd, PERF_EVENT_IOC_DISABLE, 0);
		close(fd);
	}
	return NULL;
}

int main(void)
{
	pthread_t threads[NR_THREADS];

	for (int i = 0; i < NR_THREADS; i++)
		if (pthread_create(&threads[i], NULL, worker, NULL)) {
			fprintf(stderr, "pthread_create failed\n");
			return 1;
		}
	for (int i = 0; i < NR_THREADS; i++)
		pthread_join(threads[i], NULL);

	printf("%ld reads\n", atomic_load(&reads));
	if (!atomic_load(&found)) {
		printf("not reproduced\n");
		return 0;
	}
	return 1;
}
