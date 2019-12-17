use perf_event::Builder;
use perf_event::event_kind::Hardware;

fn main() -> std::io::Result<()> {
    let mut branch_counter = Builder::new()
        .kind(Hardware::BRANCH_INSTRUCTIONS)
        .build()?;

    let vec = (0..=51).collect::<Vec<_>>();

    branch_counter.enable()?;
    println!("{:?}", vec);
    branch_counter.disable()?;

    println!("{} branches", branch_counter.read()?);

    Ok(())
}
