use std::process::Command;

use assert_cmd::{assert::OutputAssertExt, cargo::CommandCargoExt};

#[test]
fn test() {
    let exe = env!("CARGO_PKG_NAME");
    let mut server = Command::cargo_bin(exe)
        .unwrap()
        .arg("serve")
        .spawn()
        .unwrap();
    let text = "hello event sourcing";
    let client_assert = Command::cargo_bin(exe)
        .unwrap()
        .arg("echo")
        .arg(text)
        .assert();
    server.kill().unwrap();
    client_assert.success().stdout(format!("{text}\n"));
}
