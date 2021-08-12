use crate::comm_tests::random_sleep;
use crate::command_tokio::start_client;
use crate::data::Data;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Child;
// One thread, read async with tokio::select
pub async fn main_tokio_spawn(command: &str) -> Result<(), Box<dyn std::error::Error>> {
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
    // Data handling requires clone()
    let mut data_vec1 = data.get_mut().entry(client1_pid).or_default().clone();
    let handle1 = tokio::spawn(async move {
        random_sleep("Server", std::process::id());
        let from_client1 = client1.stdout.as_mut().expect("Cannot get client 1 stdout");
        let mut reader = BufReader::new(from_client1).lines();
        let msg = reader
            .next_line()
            .await
            .expect("Error reading from client")
            .expect("No message from client");
        data_vec1.push(msg);
        data_vec1
    });
    let mut data_vec2 = data.get_mut().entry(client2_pid).or_default().clone();
    let handle2 = tokio::spawn(async move {
        random_sleep("Server", std::process::id());
        let from_client2 = client2.stdout.as_mut().expect("Cannot get client 2 stdout");
        let mut reader = BufReader::new(from_client2).lines();
        let msg = reader
            .next_line()
            .await
            .expect("Error reading from client")
            .expect("No message from client");
        data_vec2.push(msg);
        data_vec2
    });
    let data_vec1 = handle1.await.expect("Task 1 failed");
    let data_vec2 = handle2.await.expect("Task 2 failed");
    data.get_mut().insert(client1_pid, data_vec1);
    data.get_mut().insert(client2_pid, data_vec2);
    Ok(())
}
