use std::process::Command;

pub fn ls_command () {
    let ls_output = Command::new("sh")
        .arg("-c")
        .arg("ls -a")
        .output()
        .expect("error");

    let ls_command = ls_output.stdout;

    println!("{:?}", std::str::from_utf8(&ls_command));
}

