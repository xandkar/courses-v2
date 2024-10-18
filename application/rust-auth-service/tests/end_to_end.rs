use std::process::Command;

use assert_cmd::{assert::OutputAssertExt, cargo::CommandCargoExt};

#[test]
fn test() {
    let payload = "hello event sourcing";
    let exe = env!("CARGO_PKG_NAME");
    let mut server = Command::cargo_bin(exe)
        .unwrap()
        .arg("serve")
        .arg(payload)
        .spawn()
        .unwrap();
    let client_assert = Command::cargo_bin(exe).unwrap().arg("get").assert();
    server.kill().unwrap();
    client_assert.success().stdout(format!("{payload}\n"));
}
