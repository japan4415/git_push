use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = &args[1];

    let add_result = add();
    println!("{}", add_result);

    let commit_result = commit(input);
    println!("{}", commit_result);

    let push_result = push();
    println!("{}", push_result);
}

fn add() -> String {
    let output = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("failed to execute process");
    std::str::from_utf8(&output.stdout).unwrap().to_string()
}

fn commit(input: &str) -> String {
    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(input)
        .output()
        .expect("failed to execute process");
    std::str::from_utf8(&output.stdout).unwrap().to_string()
}

fn push() -> String {
    let output = Command::new("git")
        .arg("push")
        .output()
        .expect("failed to execute process");
    std::str::from_utf8(&output.stdout).unwrap().to_string()
}
