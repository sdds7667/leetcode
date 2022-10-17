use std::{fs, process::Command};

fn main() {
    let problem_number = match std::env::args().nth(1) {
        Some(x) => x,
        None => {
            println!("Usage: rust-generator <problem>");
            return;
        }
    };

    let main_rs_file_path = format!("/workspaces/leetcode/prob{}/src/main.rs", problem_number);

    println!("Creating a solution for the prob: {}", problem_number);
    Command::new("cargo")
        .arg("new")
        .arg(format!("/workspaces/leetcode/prob{}", problem_number))
        .status()
        .expect("Failed to create the project");

    fs::write(
        &main_rs_file_path,
        format!(
            "struct Solution {{}}\n\n{}",
            (fs::read_to_string(&main_rs_file_path).expect("Failed to read the main.rs file"))
        ),
    )
    .expect("Could not write main.rs");

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
        .status()
        .expect("Failed to switch code directory");
}
