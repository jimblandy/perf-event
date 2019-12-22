use libc::pid_t;
use perf_event::Builder;
use perf_event::events::Hardware;
use std::thread::sleep;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let pid: pid_t = std::env::args()
        .nth(1)
        .expect("Usage: insns-for-pid PID")
        .parse()
        .expect("Usage: insns-for-pid PID");

    let mut insns = Builder::new()
        .observe_pid(pid)
        .kind(Hardware::BRANCH_INSTRUCTIONS)
        .build()?;

    // Count instructions in PID for five seconds.
    insns.enable()?;
    sleep(Duration::from_secs(5));
    insns.disable()?;

    println!("instructions in last five seconds: {}", insns.read()?);

    Ok(())
}
