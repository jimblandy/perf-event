/* Try to reproduce #68. */

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <memory.h>
#include <unistd.h>
#include <sys/ioctl.h>
#include <sys/syscall.h>
#include <linux/perf_event.h>
#include <linux/hw_breakpoint.h>

static long perf_event_open(struct perf_event_attr *attr, pid_t pid,
                            int cpu, int group_fd, unsigned long flags) {
    return syscall(__NR_perf_event_open, attr, pid, cpu, group_fd, flags);
}

static int calls;

__attribute__((noinline)) static void callee() {
  calls++;
}

__attribute__((noinline)) static void indirect_callee() {
  callee();
}

static int set_execution_breakpoint(uintptr_t addr) {
  struct perf_event_attr attr;
  memset(&attr, 0, sizeof(attr));
  attr.size = sizeof(attr);
  attr.type = PERF_TYPE_BREAKPOINT;
  attr.config = 0;

  attr.bp_type = HW_BREAKPOINT_X;
  attr.bp_addr = addr;
  attr.bp_len = sizeof(long);

  attr.exclude_kernel = 1;
  attr.exclude_hv = 1;

  attr.disabled = 1;
  attr.read_format = PERF_FORMAT_TOTAL_TIME_ENABLED | PERF_FORMAT_TOTAL_TIME_RUNNING;

  int fd = (int)perf_event_open(&attr, 0 /* this process */, -1 /* any cpu */,
                                -1, 0);
  if (fd < 0) { perror("perf_event_open"); exit(1); }
  return fd;
}

static void measure(size_t count) {
  int breakpoint_fd = set_execution_breakpoint((uintptr_t)callee);
  
  ioctl(breakpoint_fd, PERF_EVENT_IOC_RESET);
  ioctl(breakpoint_fd, PERF_EVENT_IOC_ENABLE);

  for (size_t i = 0; i < count; i++) {
    callee();
    indirect_callee();
  }

  ioctl(breakpoint_fd, PERF_EVENT_IOC_DISABLE);

  /* non-PERF_FORMAT_GROUP read layout:
     uint64_t value;
     uint64_t time_enabled;
     uint64_t time_running;
  */
  struct {
    uint64_t value;
    uint64_t time_enabled;
    uint64_t time_running;
  } counts;

  if (read(breakpoint_fd, &counts, sizeof(counts)) != (ssize_t)sizeof(counts)) {
    perror("read"); exit(1);
  }

  printf("value: %lu\n", counts.value);

  if (counts.time_enabled != counts.time_running)
    printf("time enabled: %lu  time running: %lu\n",
           (unsigned long)counts.time_enabled,
           (unsigned long)counts.time_running);

  close(breakpoint_fd);
}

int main(void) {
    measure(1729);
    return 0;
}
