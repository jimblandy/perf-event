/*! Observe the L1 data cache hit rate under two different access patterns.

This example measures L1 data cache hit rate while accessing a 40MB
array linearly, and then randomly.

One surprising finding is that, even though the loop accessing the
array is extremely simple, a `dev` build performs seven times as many
reads as a `release` build. Furthermore, in a `dev` build, the L1 data
cache still manages to achieve an 86% hit rate even under the random
access pattern, which should be completely uncacheable.

This suggests that the machine code for a `dev` build generates a lot
of extraneous memory traffic, but the cache is able to cover for most
of it. This would seem to render `dev` builds unsuitable for assessing
cache behavior.

    $ cargo run --quiet --example locality
    linear: misses / reads:  1251734 / 70038297   1.79%
    random: misses / reads: 10015294 / 70451603  14.22%

    $ cargo run --quiet --example locality --release
    linear: misses / reads:  1250392 / 10000713  12.50%
    random: misses / reads:  9998262 / 10011094  99.87%

On some machines, running the same `--release` executable reports more
misses than reads, which should be impossible. If you know why this
occurs, please file an issue at
`https://github.com/jimblandy/perf-event`.

*/

fn main() {
    use std::hint::black_box;

    const SIZE: usize = 10_000_000;

    // Build a vector that `walk` will traverse from start to end.
    let mut vec: Vec<usize> = (1..SIZE).chain(Some(0)).collect();
    measure("linear", || {
        black_box(walk(&vec));
    });

    let mut random = XorShift128Plus::from_seed(1729, 42);
    random.nth(100); // Propagate the 1-bits in the state a bit.

    // Build a vector that `walk` will access randomly.
    //
    // This turns `vec` into a single chain that visits every element.
    // Proof left to the reader.
    vec.clear();
    vec.extend(0..SIZE);
    let mut rest = &mut vec[..];
    while let Some((first, next @ &mut [_, ..])) = rest.split_first_mut() {
        let swap_with = random.next().unwrap() as usize % next.len();
        std::mem::swap(first, &mut next[swap_with]);
        rest = next;
    }

    measure("random", || {
        black_box(walk(&vec));
    });
}

/// Access elements of `indices`, as guided by its contents.
///
/// Treating each element of `indices` as the index of the next
/// element to visit, start with `indices[0]` and follow the path
/// until we get back to `0`.
///
/// Note that the access pattern depends solely on the slice's
/// contents, not on control flow. The caller can produce whatever
/// access pattern it wants by choosing the contents of `indices`
/// appropriately.
///
/// Return the number of steps needed to return to index 0.
fn walk(indices: &[usize]) -> usize {
    let mut count = 0;
    let mut i = 0;
    loop {
        count += 1;
        i = indices[i];
        if i == 0 {
            break;
        }
    }
    count
}

fn measure(label: &str, task: impl FnOnce()) {
    use perf_event::events::{Cache, CacheOp, CacheResult, WhichCache};
    use perf_event::{Builder, Group};

    let mut group = Group::new().expect("creating group is ok");
    let read_counter = Builder::new()
        .group(&mut group)
        .kind(Cache {
            which: WhichCache::L1D,
            operation: CacheOp::READ,
            result: CacheResult::ACCESS,
        })
        .build()
        .expect("building read_counter is ok");
    let read_miss_counter = Builder::new()
        .group(&mut group)
        .kind(Cache {
            which: WhichCache::L1D,
            operation: CacheOp::READ,
            result: CacheResult::MISS,
        })
        .build()
        .expect("building read_miss_counter is ok");
    group.enable().expect("enabling group is ok");
    task();
    group.disable().expect("disabling group is ok");

    let counts = group.read().expect("reading group is ok");
    let reads = counts[&read_counter];
    let read_misses = counts[&read_miss_counter];

    println!(
        "{label}: misses / reads: {read_misses:8} / {reads:8} {:6.2}%",
        (read_misses as f64 / reads as f64) * 100.0,
    );

    if counts.time_enabled() != counts.time_running() {
        println!(
            "time enabled: {}  time running: {}",
            counts.time_enabled(),
            counts.time_running(),
        );
    }
}

/// The [XorShift128+] pseudorandom number generator.
///
/// This implements [`Iterator`], producing pseudorandom `u64` values
/// as items.
///
/// [XorShift128+]: https://en.wikipedia.org/wiki/Xorshift#xorshift+
struct XorShift128Plus {
    state: [u64; 2],
}

impl XorShift128Plus {
    fn from_seed(seed1: u64, seed2: u64) -> Self {
        assert!(seed1 != 0 || seed2 != 0);
        Self {
            state: [seed1, seed2],
        }
    }
}

impl Iterator for XorShift128Plus {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut t = self.state[0];
        let s = self.state[1];
        self.state[0] = s;
        t ^= t << 23;
        t ^= t >> 18;
        t ^= s ^ (s >> 5);
        self.state[1] = t;
        Some(t.wrapping_add(s))
    }
}
