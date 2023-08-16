//REF https://doc.rust-lang.org/cargo/reference/build-scripts.html
extern crate chrono;

use chrono::{Datelike, Local};
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=cargo.toml");

    let now = Local::now();
    let date_today = format!("{}_{}_{}", now.month(), now.day(), now.year());
    let depgraph_filename = format!("depGraph-{}.svg", date_today);

    let _ = are_build_deps_installed();

    // Run the post-build command
    let cargo = Command::new("cargo")
        .args(&["depgraph", "--all-deps"])
        .stdout(Stdio::piped())
        .spawn()?;

    let dot = Command::new("dot")
        .arg("-Tsvg")
        .stdin(cargo.stdout.unwrap())
        .output()?;

    cleanup_old_depgraphs();

    //Generate new dependency graph
    std::fs::write(&depgraph_filename, dot.stdout).expect(&format!(
        "Failed to write Depgraph to output file {}",
        depgraph_filename
    ));
    Ok(())
}

fn cleanup_old_depgraphs() {
    fs::read_dir(Path::new("."))
        .unwrap()
        .filter_map(|file| {
            file.ok().and_then(|file| {
                file.file_name().into_string().ok().and_then(|file_name| {
                    if file_name.starts_with("depGraph") {
                        Some(file_name)
                    } else {
                        None
                    }
                })
            })
        })
        .collect::<Vec<String>>()
        .into_iter()
        .for_each(|f| {
            fs::remove_file(f).unwrap();
        });
}

///Terminates the process if one of required commands is not installed
fn are_build_deps_installed() -> Result<(), Box<dyn std::error::Error>> {
    let depgraph = Command::new("cargo")
        .arg("depgraph")
        .output()?
        .status
        .success();

    if !depgraph {
        eprint!("please install tool 'depgraph' https://github.com/jplatte/cargo-depgraph/\ncargo install cargo-depgraph");
        std::process::exit(1)
    }

    let graphviz = Command::new("dot").arg("-V").output()?.status.success();

    if !graphviz {
        eprint!("please install tool 'graphviz' https://graphviz.org/download/");
        std::process::exit(1)
    }
    Ok(())
}
