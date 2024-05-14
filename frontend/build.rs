use dotenv::dotenv;
use std::process::Command;

fn main() {
    // println!("cargo:rerun-if-changed=.env");
    dotenv().ok();

    for (key, value) in std::env::vars() {
        if key.starts_with("APP_") {
            println!("cargo:rustc-env={}={}", key, value);
        }
    }

    // Set commit hash and url
    let commit_hash_output = Command::new("git")
        .args(["describe", "--dirty=*", "--always", "--match", "NOT A TAG"])
        .output()
        .expect("Failed to get commit hash");
    let commit_hash =
        String::from_utf8(commit_hash_output.stdout).expect("Invalid UTF-8 in commit hash");
    let commit_hash = commit_hash.trim(); // Remove any extra whitespace

    let repo_url_output = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()
        .expect("Failed to get remote URL");
    let repo_url = String::from_utf8(repo_url_output.stdout).expect("Invalid UTF-8 in repo URL");
    let repo_url = repo_url.trim(); // Remove any extra whitespace

    let base_url = repo_url
        .replace("git@github.com:", "https://github.com/")
        .replace(".git", "");
    let commit_url = format!("{}/tree/{}", base_url, commit_hash);

    println!("cargo:rustc-env=APP_COMMIT_HASH={}", commit_hash);
    println!("cargo:rustc-env=APP_COMMIT_URL={}", commit_url);
}
