use std::process::Command;

fn main() {
    let output = Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--date=short")
        .arg("--format=%h %cd")
        .output()
        .expect("Failed to execute git command");

    let output = String::from_utf8(output.stdout).expect("Failed to parse git output");

    let mut parts = output.split_whitespace();
    let commit_hash = parts.next().unwrap_or("unknown");
    let commit_date = parts.next().unwrap_or("unknown");

    println!("cargo:rustc-env=GIT_COMMIT_HASH={}", commit_hash);
    println!("cargo:rustc-env=GIT_COMMIT_DATE={}", commit_date);
}
