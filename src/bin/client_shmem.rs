use nix::unistd::unlink;
use passfd::FdPassingExt;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::os::unix::prelude::FromRawFd;

/// Client for in-memory Key Value Store
///
/// This client is started by Pegasus
///
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let id = args.get(1).expect("Client: no id");
    eprintln!("  Client {} Start", id);
    talk_to_server(&id)?;
    eprintln!("  Client {} Exit", id);
    Ok(())
}
fn talk_to_server(id: &str) -> Result<(), std::io::Error> {
    eprintln!("  Client {} read from server", id,);
    let from_server = stdin();
    let mut to_server = stdout();
    let mut reader = BufReader::new(from_server).lines();
    let line = reader
        .next()
        .expect("No message from server")
        .expect("Error reading from server");
    eprintln!(
        "  Client {} got '{}' from server",
        id,
        line.trim_end_matches("\n")
    );
    to_server.write_all(&format!("Client {} got {}\n", id, line).as_bytes())?;
    to_server.flush()?;
    Ok(())
}
// From a failed attempt to pass file handles of pipes to another process
// Worked fine as long as process is not running in a Docker container
// Also failed if server is async and client is sync
fn _get_fds(id: &str) -> Result<(File, File), Box<dyn std::error::Error>> {
    #[cfg(feature = "docker")]
    let socket = "/socket/".to_owned() + id;
    #[cfg(not(feature = "docker"))]
    let socket = "/tmp/socket/".to_owned() + id;
    eprintln!("  Client {} connecting to socket {}", id, socket);
    let stream =
        UnixStream::connect(socket.clone()).expect(&format!("Can't connect to {}", socket));
    let fd_raw = stream.recv_fd().expect("Can't receive fd");
    let to_server = unsafe { std::fs::File::from_raw_fd(fd_raw) };
    let fd_raw = stream.recv_fd().expect("Can't receive fd");
    let from_server = unsafe { std::fs::File::from_raw_fd(fd_raw) };
    eprintln!("  Client {} unlinking socket", socket);
    unlink(&socket[..]).expect("Can't unlink socket");
    Ok((to_server, from_server))
}
#[test]
fn test_fd() {
    use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
    let mut f = unsafe { std::fs::File::from_raw_fd(6) };
    write!(f, "bar").expect("bar");
}
