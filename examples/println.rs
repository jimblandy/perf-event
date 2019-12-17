use perf_event::Builder;
use perf_event::event_kind::{EventKind, Hardware};

fn main() -> std::io::Result<()> {
    let mut cycles = Builder::new()
        .kind(Hardware::BRANCH_INSTRUCTIONS)
        .build()?;

    let vec = (0..=51).collect::<Vec<_>>();

    cycles.enable()?;
    println!("{:?}", vec);
    cycles.disable()?;

    println!("{} cycles", cycles.read()?);

    Ok(())
}
