use anyhow::Result;
use nix::unistd::getpid;
use std::process::{Command, Stdio};

fn main() -> Result<()> {
    let pid = getpid();
    let regex = format!(r"^ *{}", pid);

    println!("Pid: {}", pid);
    println!("Rexex: {}", regex);

    let ps = Command::new("ps")
        .args(["-o", "pid,comm,vsz,rss,min_flt,maj_flt"])
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .unwrap();
    let grep = Command::new("grep").arg(regex).stdin(ps).output()?.stdout;

    println!("{}", String::from_utf8_lossy(&grep));
    Ok(())
}
