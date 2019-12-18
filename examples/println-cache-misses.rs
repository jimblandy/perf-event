use perf_event::Builder;
use perf_event::event_kind::{Cache, CacheOp, CacheResult, WhichCache};

fn main() -> std::io::Result<()> {
    let mut branch_counter = Builder::new()
        .kind(Cache {
            which: WhichCache::L1D,
            operation: CacheOp::READ,
            result: CacheResult::MISS,
        })
        .build()?;

    let vec = (0..=51).collect::<Vec<_>>();

    branch_counter.enable()?;
    println!("{:?}", vec);
    branch_counter.disable()?;

    println!("{} L1D cache misses", branch_counter.read()?);

    Ok(())
}
