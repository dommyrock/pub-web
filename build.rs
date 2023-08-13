extern crate chrono;

use chrono::{Datelike, Local};
use std::process::{Command, Stdio};

//REF https://doc.rust-lang.org/cargo/reference/build-scripts.html
fn main() {
    println!("cargo:rerun-if-changed=cargo.toml");

    let now = Local::now();
    let timeformat = format!("{}_{}_{}", now.month(), now.day(), now.year());
    let filename = format!("depGraph-{}.svg", timeformat);

    //Check if 'depgraph' is present
    let output = Command::new("cargo").arg("depgraph").output().expect(
        "failed to execute 'cargo depgraph' please install by running cargo install cargo-depgraph",
    );

    //command does not exist
    if output.status.code() == Some(127) || !output.status.success() {
        println!("please install tool 'depgraph' from https://github.com/jplatte/cargo-depgraph/");
        return;
    }

    // Run the post-build command
    let cargo = Command::new("cargo")
        .args(&["depgraph", "--all-deps"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to run cargo depgraph");

    let dot = Command::new("dot")
        .arg("-Tsvg")
        .stdin(cargo.stdout.unwrap())
        .output()
        .expect("failed to run dot");

    //Latest dependency graph
    std::fs::write(filename, dot.stdout).expect("failed to write output file");
}
