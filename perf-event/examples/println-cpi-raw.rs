use perf_event::events::Raw;
use perf_event::{Builder, Group};

fn main() -> std::io::Result<()> {
    /// Measure CPI on aarch64/x86_64
    ///
    /// Raw events are different for each arch.
    #[cfg(target_arch = "aarch64")]
    const INSNS_RETIRED: Raw = Raw { config: 0x08 };
    #[cfg(target_arch = "aarch64")]
    const CPU_CYCLES: Raw = Raw { config: 0x11 };
    #[cfg(target_arch = "x86_64")]
    const INSNS_RETIRED: Raw = Raw { config: 0x0c0 };
    #[cfg(target_arch = "x86_64")]
    const CPU_CYCLES: Raw = Raw { config: 0x3c };

    let mut group = Group::new()?;
    let raw_insns_retired = Builder::new()
        .group(&mut group)
        .kind(INSNS_RETIRED)
        .include_kernel()
        .build()?;
    let raw_cpu_cycles = Builder::new()
        .group(&mut group)
        .kind(CPU_CYCLES)
        .include_kernel()
        .build()?;

    let vec = (0..=51).collect::<Vec<_>>();

    group.enable()?;
    println!("{:?}", vec);
    group.disable()?;

    let counts = group.read()?;
    println!(
        "cycles / instructions: {} / {} ({:.2} cpi)",
        counts[&raw_cpu_cycles],
        counts[&raw_insns_retired],
        (counts[&raw_cpu_cycles] as f64 / counts[&raw_insns_retired] as f64)
    );

    Ok(())
}
