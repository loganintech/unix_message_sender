use std::env;
use std::process::{Output, Command};
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 2 {
        panic!("Must include a username and a message.");
    }

    let username = &args[0];
    let message = args[1..].join(" ");

    let tty = get_tty(username);
    match write_message(username, &message, &tty) {
        Ok(text) => println!("{}", text),
        Err(text) => println!("{}", text),
    }
}

fn get_tty(username: &str) -> String {
    let output: Output = Command::new("w")
            .arg(username)
            .output()
            .expect("Failed to run the `w` command.");

    match String::from_utf8(output.stdout).unwrap_or_else(|_| {
        println!("Got bad output from `w` command.");
        panic!();
    }).lines().filter(|val| val.contains(username)).last() {
        Some(val) => val.to_owned(),
        None => "".to_owned(),
    }
}

fn write_message(username: &str, message: &str, tty: &str) -> std::result::Result<String, String> {
    let echo_command = Command::new("echo")
                              .arg(message)
                              .spawn()
                              .expect("Failed to run echo.");

    let out = match echo_command.stdout {
        Some(out) => out,
        None => panic!("Echo's stdout resulted in None"),
    };

    let output: Output = Command::new("write")
            .stdin(out)
            .arg(username)
            .arg(tty)
            .output()
            .expect("Failed to run write.");

    if output.status.success() {
        Ok(format!("Sent message. stdout: {}", String::from_utf8_lossy(&output.stdout)))
    } else {
        Err(format!("Error sending message. stderr: {}", String::from_utf8_lossy(&output.stderr)))
    }
}
