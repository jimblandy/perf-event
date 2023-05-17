use perf_event::events::{Cache, CacheId, CacheOp, CacheResult, Hardware};
use perf_event::{Builder, Group};

fn main() -> std::io::Result<()> {
    const ACCESS: Cache = Cache {
        which: CacheId::L1D,
        operation: CacheOp::READ,
        result: CacheResult::ACCESS,
    };
    const MISS: Cache = Cache {
        result: CacheResult::MISS,
        ..ACCESS
    };

    let mut group = Group::new()?;
    let access_counter = group.add(&Builder::new(ACCESS))?;
    let miss_counter = group.add(&Builder::new(MISS))?;
    let branches = group.add(&Builder::new(Hardware::BRANCH_INSTRUCTIONS))?;
    let missed_branches = group.add(&Builder::new(Hardware::BRANCH_MISSES))?;
    let insns = group.add(&Builder::new(Hardware::INSTRUCTIONS))?;
    let cycles = group.add(&Builder::new(Hardware::CPU_CYCLES))?;

    // Note that if you add more counters than you actually have hardware for,
    // the kernel will time-slice them, which means you may get no coverage for
    // short measurements. See the documentation.
    //
    // On my machine, this program won't collect any data unless I disable the
    // NMI watchdog, as described in the documentation for `Group`. My machine
    // has four counters, and this program tries to use all of them, but the NMI
    // watchdog uses one up.

    let mut vec = (0..=100000).collect::<Vec<_>>();

    group.enable()?;
    vec.sort();
    println!("{:?}", &vec[0..10]);
    group.disable()?;

    let counts = group.read()?;

    let time_enabled = counts.time_enabled().unwrap();
    let time_running = counts.time_running().unwrap();

    println!(
        "enabled for {}ns, actually running for {}ns",
        time_running.as_nanos(),
        time_enabled.as_nanos()
    );

    if time_running.is_zero() {
        println!("Group was never running; no results available.");
        return Ok(());
    }

    if time_running < time_enabled {
        println!("Counts cover only a portion of the execution.");
    }

    println!(
        "L1D cache misses/references: {} / {} ({:.0}%)",
        counts[&miss_counter],
        counts[&access_counter],
        (counts[&miss_counter] as f64 / counts[&access_counter] as f64) * 100.0
    );

    println!(
        "branch prediction misses/total: {} / {} ({:.0}%)",
        counts[&missed_branches],
        counts[&branches],
        (counts[&missed_branches] as f64 / counts[&branches] as f64) * 100.0
    );

    println!(
        "{} instructions, {} cycles ({:.2} cpi)",
        counts[&insns],
        counts[&cycles],
        counts[&cycles] as f64 / counts[&insns] as f64
    );

    // You can iterate over a `Counts` value:
    for entry in &counts {
        println!("Counter id {} has value {}", entry.id(), entry.value());
    }

    Ok(())
}
