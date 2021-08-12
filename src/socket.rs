use nix::unistd::unlink;
use std::os::unix::net::{UnixListener, UnixStream};

pub fn socket(sock_name: &str) -> Result<UnixStream, Box<dyn std::error::Error>> {
    println!("Connect on {}", sock_name);
    unlink(sock_name)?;
    let listener = UnixListener::bind(sock_name).expect("Can't bind socket");
    let (stream, _) = listener.accept().expect("Can't listen on bar.sock");
    Ok(stream)
}
