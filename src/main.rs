use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = &args[1];
    let output = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("failed to execute process");
    let hello = output.stdout;
    println!("{}", std::str::from_utf8(&hello).unwrap());
    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(input)
        .output()
        .expect("failed to execute process");
    let hello = output.stdout;
    println!("{}", std::str::from_utf8(&hello).unwrap());
    let output = Command::new("git")
    .arg("push")
    .output()
    .expect("failed to execute process");
    let hello = output.stdout;
    println!("{}", std::str::from_utf8(&hello).unwrap());
}
