use crate::comm_tests::random_sleep;
use crate::command_std::start_client;
use crate::data::Data;
use std::io::{BufRead, BufReader, Write};
use std::process::{self, Child};
// One thread, read from first client then from second
pub fn main_sync(command: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _f = "main_sync";
    let server_pid = std::process::id();
    println!("Server start with PID {}", server_pid);
    let client_id1 = "1";
    let client_id2 = "2";
    let mut client1 = start_client(command, client_id1)?;
    let mut client2 = start_client(command, client_id2)?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    let mut data = Data::new();
    talk_to_client(&mut data, b"1: Hello from Server\n", &mut client1)?;
    talk_to_client(&mut data, b"2: Hello from Server\n", &mut client2)?;
    println!("Server {} Messages {}", server_pid, data);
    Ok(())
}
fn talk_to_client(
    data: &mut Data,
    msg: &[u8],
    client: &mut Child,
) -> Result<(), Box<dyn std::error::Error>> {
    let client_pid = client.id();
    let data_vec = data.get_mut().entry(client_pid).or_default();
    println!(
        "Server {} sending to client PID {}",
        process::id(),
        client_pid,
    );
    let to_client = client.stdin.as_mut().expect("Cannot get client 1 stdin");
    let from_client = client.stdout.as_mut().expect("Cannot get client 2 stdout");
    to_client.write(msg)?;
    println!(
        "Server {} reading from client {}",
        process::id(),
        client_pid,
    );
    random_sleep("Server", std::process::id());
    let mut reader = BufReader::new(from_client).lines();
    let msg = reader
        .next()
        .expect("No message from client")
        .expect("Error reading from client");
    println!(
        "Server {} got from client {} '{}'",
        process::id(),
        client_pid,
        msg
    );
    data_vec.push(msg.to_owned());
    Ok(())
}
