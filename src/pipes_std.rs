use nix::unistd::{pipe, unlink};
use passfd::FdPassingExt;
use std::fs::File;
use std::os::unix::net::UnixListener;
use std::os::unix::prelude::FromRawFd;

pub fn pipes(sock_name: &'static str) -> Result<(File, File), Box<dyn std::error::Error>> {
    let socket = "/tmp/socket/".to_owned() + sock_name;
    println!("Server set up stream {}", socket);
    let (from_client_raw, to_server_raw) = pipe().expect("Can't create pipe 1");
    let (from_server_raw, to_client_raw) = pipe().expect("Can't create pipe 2");
    std::thread::spawn(move || {
        match unlink(&socket[..]) {
            Ok(_) => println!("Server {} unlinked", socket),
            Err(e) => println!("Server {} unlink error {}", socket, e),
        }
        let listener = UnixListener::bind(socket).expect("Can't bind socket");
        println!("Server accepting on {}", sock_name);
        let (stream, _) = listener.accept().expect("Can't listen on bar.sock");
        println!("Server send fds");
        stream
            .send_fd(to_server_raw)
            .expect("Can't send to_server fd");
        stream
            .send_fd(from_server_raw)
            .expect("Can't send from_server fd");
    });
    let to_client = unsafe { File::from_raw_fd(to_client_raw) };
    let from_client = unsafe { File::from_raw_fd(from_client_raw) };
    Ok((to_client, from_client))
}
