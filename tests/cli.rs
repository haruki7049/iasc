//! This file contains the tests for the CLI.

use assert_cmd::Command;
use std::process::Output;

/// Test the conversion from a prefix to subnet.
#[test]
fn test_prefix_to_subnet() {
    let result = Command::cargo_bin("iasc")
        .unwrap()
        .arg("--conversion-type")
        .arg("prefix-to-subnet")
        .arg("--prefix-length")
        .arg("24")
        .unwrap();

    assert_eq!(result.stdout, b"255.255.255.0\n");
    assert!(result.status.success());
}

/// Test the conversion from a subnet to prefix.
#[test]
fn test_subnet_to_prefix() {
    let result: Output = Command::cargo_bin("iasc")
        .unwrap()
        .arg("--conversion-type")
        .arg("subnet-to-prefix")
        .arg("--subnet-mask")
        .arg("255.255.255.0")
        .unwrap();

    assert_eq!(result.stdout, b"24\n");
    assert!(result.status.success());
}
