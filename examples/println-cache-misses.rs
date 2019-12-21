use perf_event::Builder;
use perf_event::events::{Cache, CacheOp, CacheResult, WhichCache};

fn main() -> std::io::Result<()> {
    let mut counter = Builder::new()
        .kind(Cache {
            which: WhichCache::L1D,
            operation: CacheOp::READ,
            result: CacheResult::MISS,
        })
        .build()?;

    let vec = (0..=51).collect::<Vec<_>>();

    counter.enable()?;
    println!("{:?}", vec);
    counter.disable()?;

    println!("{} L1D cache misses", counter.read()?);

    Ok(())
}
