use perf_event::{events, Builder};
use std::time::{Duration, Instant};

fn main() -> std::io::Result<()> {
    let mut handles: Vec<std::thread::JoinHandle<std::io::Result<()>>> = vec![];

    let end = Instant::now() + Duration::from_secs(10);

    for cpu in 0..8 {
        let handle = std::thread::spawn(move || {
            let sample_stream = Builder::new()
                .kind(events::Hardware::CPU_CYCLES)
                .one_cpu(cpu)
                .observe_all()
                .sample_callchain()
                .sample_frequency(4000)
                .sample_ip()
                .sample_tid()
                .sample_time()
                .sample_cpu()
                .sample_period()
                .sample_stream()?;

            sample_stream.enable()?;

            let mut now = Instant::now();
            while now < end {
                if let Some(sample) = sample_stream.read(Some(end - now))? {
                    println!("{:#?}", sample);
                }
                now = Instant::now();
            }

            Ok(())
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap()?;
    }
    Ok(())
}
