use nix::unistd::{setuid, Uid};
use std::process::Command;
use std::os::unix::process::CommandExt;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    setuid(Uid::from_raw(1000))?; // drop privs
    let proxy = "socks5h://127.0.0.1:9050";
    Command::new("bwrap")
        .arg("--ro-bind").arg("/usr").arg("/usr")
        .arg("--ro-bind").arg("/lib").arg("/lib")
        .arg("--tmpfs").arg("/home/user")
        .arg("--tmpfs").arg("/tmp")
        .arg("--dev").arg("/dev")
        .arg("--proc").arg("/proc")
        .arg("--unshare-all")
        .arg("--die-with-parent")
        .arg("--setenv").arg("ALL_PROXY").arg(proxy)
        .arg("/usr/bin/psychy-browser")
        .args(args)
        .exec();
    unreachable!()
}
