use std::process::Command;

fn main() {

    let output = Command::new("sh")
        .arg("-c")
        .arg("echo Hello")
        .output()
        .expect("failed to excute this command");

    let echo_hello = output.stdout;

    println!("{}", std::str::from_utf8(&echo_hello).unwrap());

}

