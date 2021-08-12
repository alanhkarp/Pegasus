use crate::comm_tests::random_sleep;
use crate::command_std::start_client;
use crate::data::Data;
use crossbeam::thread;
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::io::{BufRead, BufReader, Write};
use std::process::{self, Child};
// Read in threads, send to main thread, read one at a time
pub fn main_mpsc(command: &str) -> Result<(), Box<dyn std::error::Error>> {
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
fn talk_to_clients(
    data: &mut Data,
    msg: &str,
    mut client1: Child,
    mut client2: Child,
) -> Result<(), Box<dyn std::error::Error>> {
    let client1_pid = client1.id();
    let client2_pid = client2.id();
    let (tx, rx): (Sender<(u32, String)>, Receiver<(u32, String)>) = unbounded();
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
        let tx_clone = tx.clone();
        scope.spawn(move |_| {
            random_sleep("Server", std::process::id());
            let mut reader = BufReader::new(from_client1).lines();
            let msg = reader
                .next()
                .expect("No message from client 1")
                .expect("Problem reading 1");
            tx.send((client1_pid, msg)).expect("Problem sending 1");
        });
        scope.spawn(move |_| {
            random_sleep("Server", std::process::id());
            let mut reader = BufReader::new(from_client2).lines();
            let msg = reader
                .next()
                .expect("No message from client 2")
                .expect("Problem reading 2");
            tx_clone
                .send((client2_pid, msg))
                .expect("Problem sending 2");
        });
    })
    .expect("Scope failed");
    println!("Server reading from clients ");
    let (client_pid, msg) = rx.recv().expect("Error reading on channel");
    let data_vec = data.get_mut().entry(client_pid).or_default();
    data_vec.push(msg);
    let (client_pid, msg) = rx.recv().expect("Error reading on channel");
    let data_vec = data.get_mut().entry(client_pid).or_default();
    data_vec.push(msg);
    Ok(())
}
