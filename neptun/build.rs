use std::error::Error;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    // Extract last commit sha using cli
    let git_sha = "unknown".to_string();

    // Output GIT_SHA as build time env variable
    println!("cargo:rustc-env=GIT_SHA={}", git_sha);

    // Re-run build script if git head changed
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/heads/");

    Ok(())
}
