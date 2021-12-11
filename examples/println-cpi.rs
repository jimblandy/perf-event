fn main() -> std::io::Result<()> {
    use perf_event::events::Hardware;
    use perf_event::{Builder, Group};

    let mut group = Group::new()?;
    let cycles = Builder::new()
        .group(&mut group)
        .kind(Hardware::CPU_CYCLES)
        .counter()?;
    let insns = Builder::new()
        .group(&mut group)
        .kind(Hardware::INSTRUCTIONS)
        .counter()?;

    let vec = (0..=51).collect::<Vec<_>>();

    group.enable()?;
    println!("{:?}", vec);
    group.disable()?;

    let counts = group.read()?;
    println!(
        "cycles / instructions: {} / {} ({:.2} cpi)",
        counts[&cycles],
        counts[&insns],
        (counts[&cycles] as f64 / counts[&insns] as f64)
    );

    Ok(())
}
