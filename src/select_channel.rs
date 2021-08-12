use crate::comm_tests::random_sleep;
use crate::command_std::start_client;
use crate::data::Data;
use crossbeam::thread;
use crossbeam_channel::{select, unbounded, Receiver, Sender};
use std::io::{BufRead, BufReader, Write};
use std::process::{self, Child};
// Read in threads, send to main thread, use crossbeam select
pub fn main_select_channel(command: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _f = "main_select";
    let server_pid = std::process::id();
    println!("Server start with PID {}", server_pid);
    let client1 = start_client(command, "1")?;
    let client2 = start_client(command, "2")?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    let mut data = Data::new();
    talk_to_clients(&mut data, "Hello from Server\n", client1, client2)?;
    println!("Server {} Messages {}", server_pid, data);
    Ok(())
}
pub fn talk_to_clients(
    data: &mut Data,
    msg: &str,
    mut client1: Child,
    mut client2: Child,
) -> Result<(), Box<dyn std::error::Error>> {
    let client1_pid = client1.id();
    let client2_pid = client2.id();
    let (tx1, rx1): (Sender<String>, Receiver<String>) = unbounded();
    let (tx2, rx2): (Sender<String>, Receiver<String>) = unbounded();
    let to_client1 = client1.stdin.as_mut().expect("Cannot get client 1 stdin");
    let to_client2 = client2.stdin.as_mut().expect("Cannot get client 2 stdin");
    let from_client1 = client1.stdout.as_mut().expect("Cannot get client 1 stdout");
    let from_client2 = client2.stdout.as_mut().expect("Cannot get client 2 stdout");
    println!(
        "Server {} sending to client PID {}",
        process::id(),
        client1_pid
    );
    let msg1 = "1: ".to_owned() + msg;
    to_client1.write_all(msg1.as_bytes())?;
    println!(
        "Server {} sending to client PID {}",
        process::id(),
        client2_pid
    );
    let msg2 = "2: ".to_owned() + msg;
    to_client2.write_all(msg2.as_bytes())?;
    thread::scope(|scope| {
        scope.spawn(move |_| {
            random_sleep("Server", std::process::id());
            let mut reader = BufReader::new(from_client1).lines();
            let msg = reader
                .next()
                .expect("Problem reading from client 1")
                .expect("No message from client 1");
            tx1.send(msg).expect("Problem sending 1");
        });
        scope.spawn(move |_| {
            random_sleep("Server", std::process::id());
            let mut reader = BufReader::new(from_client2).lines();
            let msg = reader
                .next()
                .expect("Problem reading 2")
                .expect("No message fro client 2");
            tx2.send(msg)
        });
    })
    .expect("Scope failed");
    println!("Server reading from clients ");
    for _ in 0..2 {
        select! {
            recv(rx1) -> result => {
                let data_vec1 = data.get_mut().entry(client1_pid).or_default();
                let msg = result.expect("Error reading from 1");
                data_vec1.push(msg);
            }
            recv(rx2) -> result => {
                let data_vec2 = data.get_mut().entry(client2_pid).or_default();
                let msg = result.expect("Error reading from 2");
                data_vec2.push(msg);
            }
        }
    }
    Ok(())
}
