use perf_event::Builder;
use std::io;

fn main() -> io::Result<()> {
    let mut cycles = Builder::new().build()?;

    let vec = (0..=50).collect::<Vec<_>>();

    cycles.enable()?;
    println!("{:?}", vec);
    cycles.disable()?;

    println!("{} cycles", cycles.read()?);

    Ok(())
}
