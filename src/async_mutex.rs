use crate::comm_tests::random_sleep;
use crate::command_tokio::start_client;
use crate::data::Data;
use std::sync::{Arc, Mutex};
use tokio::io::AsyncWriteExt;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Child;
use tokio::try_join;
// Async communication with clients, mutex to manage data
pub async fn main_async_mutex(command: &str) -> Result<(), Box<dyn std::error::Error>> {
    let server_pid = std::process::id();
    println!("Server start with PID {}", server_pid);
    let client_id1 = "1";
    let client_id2 = "2";
    let mut client1 = start_client(command, client_id1)?;
    let mut client2 = start_client(command, client_id2)?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    let data = Arc::new(Mutex::new(Data::new()));
    let f1 = talk_to_client(data.clone(), b"1: Hello from server\n", &mut client1);
    let f2 = talk_to_client(data.clone(), b"2: Hello from server\n", &mut client2);
    try_join!(f1, f2)?;
    let data = data.lock().unwrap();
    println!("Server {} Messages {}", server_pid, data);
    Ok(())
}
pub async fn talk_to_client(
    data: Arc<Mutex<Data>>,
    msg: &[u8],
    client: &mut Child,
) -> Result<(), Box<dyn std::error::Error>> {
    let client_pid = client.id().expect("No PID for client");
    let to_client = client.stdin.as_mut().expect("Cannot get client 1 stdin");
    let from_client = client.stdout.as_mut().expect("Cannot get client 2 stdout");
    let mut reader = BufReader::new(from_client).lines();
    println!("Server sending to client PID {}", client_pid);
    to_client.write(msg).await?;
    random_sleep("Server", std::process::id());
    println!("Server reading from client {}", client_pid);
    let msg = reader.next_line().await?.expect("No message from client");
    println!(
        "Server got from client {} '{}'",
        client_pid,
        msg.replace("\n", "")
    );
    let mut locked_data = data.lock().unwrap();
    let data_vec = locked_data.get_mut().entry(client_pid).or_default();
    data_vec.push(msg);
    Ok(())
}
