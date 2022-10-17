use std::{fs, process::Command};

fn main() {
    let problem_number = match std::env::args().nth(1) {
        Some(x) => x,
        None => {
            println!("Usage: rust-generator <problem>");
            return;
        }
    };

    println!("Creating a solution for the prob: {}", problem_number);
    Command::new("cargo")
        .arg("new")
        .arg(format!("/workspaces/leetcode/prob{}", problem_number))
        .status()
        .expect("Failed to create the project");

    fs::copy(
        "/workspaces/leetcode/rust-generator/.gitignore-model",
        format!("/workspaces/leetcode/prob{}/.gitignore", problem_number),
    )
    .expect("Failed to copy the .gitignore file");

    Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg(format!("prob/{}", problem_number))
        .status()
        .expect("Failed to switch the git branch");

    Command::new("code")
        .arg(format!("/workspaces/leetcode/prob{}", problem_number))
        .status().expect("Failed to switch code directory");
}
