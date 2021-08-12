use std::process::Stdio;
use tokio::process::{Child, Command};

pub fn start_client(command: &str, client: &str) -> Result<Child, Box<dyn std::error::Error>> {
    println!("Server starting client {} with async library", client);
    let mut args = command.trim().split(" ");
    let program = args.next().expect("Invalid command");
    let client = Command::new(program)
        .args(args.clone())
        .arg(client)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Spawn failed");
    Ok(client)
}
