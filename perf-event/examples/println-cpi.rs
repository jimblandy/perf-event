fn main() -> std::io::Result<()> {
    use perf_event::events::Hardware;
    use perf_event::{Builder, Group};

    let mut group = Group::new()?;
    let cycles = Builder::new(Hardware::CPU_CYCLES)
        .group(&mut group)
        .build()?;
    let insns = Builder::new(Hardware::INSTRUCTIONS)
        .group(&mut group)
        .build()?;

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
