use perf_event::{Builder, Group};
use perf_event::events::Hardware;

fn main() -> std::io::Result<()> {
    let mut group = Group::new()?;
    let references = Builder::new().group(&group).kind(Hardware::CACHE_REFERENCES).build()?;
    let misses = Builder::new().group(&group).kind(Hardware::CACHE_MISSES).build()?;

    let vec = (0..=51).collect::<Vec<_>>();

    group.enable()?;
    println!("{:?}", vec);
    group.disable()?;

    let counts = group.read()?;
    println!("cache misses/references: {} / {} ({:.0}%)",
             counts[&misses],
             counts[&references],
             (counts[&misses] as f64 / counts[&references] as f64) * 100.0);

    Ok(())
}
