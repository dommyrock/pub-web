extern crate chrono;

use chrono::{Datelike, Local};
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

//REF https://doc.rust-lang.org/cargo/reference/build-scripts.html
fn main() {
    println!("cargo:rerun-if-changed=cargo.toml");

    let now = Local::now();
    let date_today = format!("{}_{}_{}", now.month(), now.day(), now.year());
    let depgraph_filename = format!("depGraph-{}.svg", date_today);

    is_depgraph_installed();

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
    std::fs::write(&depgraph_filename, dot.stdout)
        .expect("failed to write Depgraph to output file");

    cleanup_old_depgraphs(depgraph_filename);
}

fn cleanup_old_depgraphs(depgraph_filename: String) {
    let _ = fs::read_dir(Path::new("."))//current dir
        .unwrap()
        .filter_map(|file| {
            let file_name = file.unwrap().file_name().into_string().unwrap();
            if file_name.starts_with("depGraph") && file_name != depgraph_filename {
                Some(file_name)
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .iter()
        .for_each(|f| {
            fs::remove_file(f).unwrap();
        });
}

///Terminates process if 'depgraph' cmd is not installed
fn is_depgraph_installed() {
    let output = Command::new("cargo")
        .arg("depgraph")
        .output()
        .expect("'cargo depgraph' command missing")
        .status
        .success();

    if !output {
        eprint!("please install tool 'depgraph' https://github.com/jplatte/cargo-depgraph/\ncargo install cargo-depgraph");
        std::process::exit(1)
    }
}
