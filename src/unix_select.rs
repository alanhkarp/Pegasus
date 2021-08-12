use crate::comm_tests::random_sleep;
use crate::command_std::start_client;
use crate::data::Data;
use nix::sys::select::{select, FdSet};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::process::{self, Child};
// Read in threads, send to main thread, use crossbeam select
pub fn main_unix_select(command: &str) -> Result<(), Box<dyn std::error::Error>> {
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
    let to_client1 = client1.stdin.as_mut().expect("Cannot get client 1 stdin");
    let to_client2 = client2.stdin.as_mut().expect("Cannot get client 2 stdin");
    let from_client1 = client1.stdout.as_mut().expect("Cannot get client 1 stdout");
    let from_client2 = client2.stdout.as_mut().expect("Cannot get client 2 stdout");
    let mut fdset = FdSet::new();
    let raw_fd1: RawFd = from_client1.as_raw_fd();
    fdset.insert(raw_fd1);
    let raw_fd2: RawFd = from_client2.as_raw_fd();
    fdset.insert(raw_fd2);
    println!(
        "Server {} sending to client PID {}",
        process::id(),
        client1_pid,
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
    println!("Server reading from clients");
    let mut count = 0;
    while count < 2 {
        let mut fdset_clone = fdset.clone();
        let mut fds = fdset_clone.fds(None);
        println!("FdSet before {:?} {:?}", fds.next(), fds.next());
        random_sleep("Server", std::process::id());
        match select(None, &mut fdset, None, None, None) {
            Ok(__) => (),
            Err(e) => {
                println!("Select error {}", e);
                continue;
            }
        }
        let mut fdset_clone = fdset.clone();
        let mut fds = fdset_clone.fds(None);
        println!("FdSet after {:?} {:?}", fds.next(), fds.next());
        for fd_raw in fdset.fds(None) {
            let (which, client_pid, mut reader) = if fd_raw == raw_fd1 {
                let from_client1 = client1.stdout.as_mut().expect("Cannot get client 1 stdout");
                let reader = BufReader::new(from_client1).lines();
                (1, client1_pid, reader)
            } else {
                let from_client2 = client2.stdout.as_mut().expect("Cannot get client 2 stdout");
                let reader = BufReader::new(from_client2).lines();
                (2, client2_pid, reader)
            };
            println!("Server reading from client {}", which);
            let msg = reader.next();
            if msg.is_some() {
                count = count + 1;
                let data_vec = data.get_mut().entry(client_pid).or_default();
                data_vec.push(msg.unwrap()?);
            } else {
                println!("No message from client {}", which);
            }
        }
        fdset.clear();
        fdset.insert(raw_fd1);
        fdset.insert(raw_fd2);
    }
    Ok(())
}
