use crate::comm_tests::random_sleep;
use crate::command_tokio::start_client;
use crate::data::Data;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Child;
use tokio::process::ChildStdout;
// One thread, read async with select on unknown number of channels
pub async fn main_futures_unordered(command: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _f = "main_select";
    let server_pid = std::process::id();
    println!("Server start with PID {}", server_pid);
    let client1 = start_client(command, "1")?;
    let client2 = start_client(command, "2")?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    let mut data = Data::new();
    talk_to_clients(&mut data, "Hello from Server\n", client1, client2).await?;
    println!("Server {} Messages {}", server_pid, data);
    Ok(())
}
async fn talk_to_clients(
    data: &mut Data,
    msg: &str,
    mut client1: Child,
    mut client2: Child,
) -> Result<(), Box<dyn std::error::Error>> {
    let client1_pid = client1.id().expect("Can't get client 1 PID");
    let client2_pid = client2.id().expect("Can't get client 2 PID");
    let to_client1 = client1.stdin.as_mut().expect("Cannot get client 1 stdin");
    let to_client2 = client2.stdin.as_mut().expect("Cannot get client 2 stdin");
    let from_client1 = client1.stdout.as_mut().expect("Cannot get client 1 stdout");
    let from_client2 = client2.stdout.as_mut().expect("Cannot get client 2 stdout");
    println!(
        "Server {} sending to client PID {}",
        std::process::id(),
        client1_pid
    );
    let msg1 = "1: ".to_owned() + msg;
    to_client1.write_all(msg1.as_bytes()).await?;
    println!(
        "Server {} sending to client PID {}",
        std::process::id(),
        client2_pid
    );
    let msg2 = "2: ".to_owned() + msg;
    to_client2.write_all(msg2.as_bytes()).await?;
    println!("Server reading from clients ");
    let mut futures = FuturesUnordered::new();
    let f1 = listen(client1_pid, from_client1);
    let f2 = listen(client2_pid, from_client2);
    futures.push(f1);
    futures.push(f2);
    let (client_pid, msg) = futures.next().await.expect("No message to read")?;
    let data_vec = data.get_mut().entry(client_pid).or_default();
    data_vec.push(msg);
    let (client_pid, msg) = futures.next().await.expect("No message to read")?;
    let data_vec = data.get_mut().entry(client_pid).or_default();
    data_vec.push(msg);
    Ok(())
}
async fn listen(
    client_pid: u32,
    from_client: &mut ChildStdout,
) -> Result<(u32, String), Box<dyn std::error::Error>> {
    random_sleep("Server", std::process::id());
    let mut reader = BufReader::new(from_client).lines();
    let msg = reader.next_line().await?.expect("No message from client");
    println!(
        "Server got from client {} '{}'",
        client_pid,
        msg.replace("\n", "")
    );
    Ok((client_pid, msg))
}
