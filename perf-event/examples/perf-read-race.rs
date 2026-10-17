// Reproducer for https://github.com/jimblandy/perf-event/issues/65
//
// See also:
// - Pure C reproducer: `perf-read-race.c`.
// - Linux Perf mailing list thread:
//   https://lore.kernel.org/linux-perf-users/69859989.a00a0220.34fa92.0026.GAE@google.com/

use perf_event::Builder;
use perf_event::events::{Event, Hardware, Software};

fn main() {
    for _ in 0..100 {
        std::thread::scope(|scope| {
            for _ in 0..128 {
                scope.spawn(|| {
                    for _ in 0..1000 {
                        let mut counters = [
                            Hardware::CPU_CYCLES.into(),
                            Hardware::INSTRUCTIONS.into(),
                            Software::TASK_CLOCK.into(),
                        ]
                        .map(|event: Event| {
                            let mut counter = Builder::new().kind(event).build().unwrap();
                            counter.enable().unwrap();
                            counter.read().unwrap();
                            counter
                        });
                        for counter in &mut counters {
                            counter.read().unwrap();
                            counter.disable().unwrap();
                        }
                    }
                });
            }
        });
    }
}
