use perf_event::{Builder, Event};
use std::io;

fn main() -> io::Result<()> {
    let mut cycles = Builder::new().build()?;

    let vec = (0..=50).collect::<Vec<_>>();

    cycles.enable()?;
    println!("{:?}", vec);
    let count = cycles.read()?;

    println!("{} cycles", count);

    Ok(())
}
