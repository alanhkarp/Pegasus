use crate::comm_tests::random_sleep;
use crate::command_tokio::start_client;
use crate::data::Data;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Child;
// One thread, read async with tokio::select
pub async fn main_select_tokio(command: &str) -> Result<(), Box<dyn std::error::Error>> {
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
    let mut reader1 = BufReader::new(from_client1).lines();
    let mut reader2 = BufReader::new(from_client2).lines();
    // Data handled in place but must know how many senders are compile time
    let mut count = 0;
    while count < 2 {
        random_sleep("Server", std::process::id());
        let (which, pid, msg) = tokio::select! {
            msg = reader1.next_line() => {
                (1, client1_pid, msg)
            }
            msg = reader2.next_line() => {
                (2, client1_pid, msg)
            }
        };
        let msg = msg?;
        if msg.is_some() {
            count = count + 1;
            let data_vec = data.get_mut().entry(pid).or_default();
            data_vec.push(msg.unwrap());
        } else {
            println!("No message from client {}", which);
        }
    }
    Ok(())
}
