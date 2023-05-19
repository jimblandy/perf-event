use std::process::ExitCode;
use std::time::Duration;

use perf_event::events::KProbe;
use perf_event::Builder;

fn run() -> std::io::Result<()> {
    let func = "__x64_sys_write";
    let probe = KProbe::probe(func, 0)?;
    let mut builder = Builder::new(probe)
        .one_cpu(0)
        .any_pid()
        .enabled(true)
        .build()?;

    std::thread::sleep(Duration::from_secs(5));

    builder.disable()?;
    let count = builder.read()?;

    println!("{func} was called {count} times");

    Ok(())
}

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("{e}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
