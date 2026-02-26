use std::io::Read;
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    const ADDRESS: &str = "127.0.0.1:8080";

    let listener = TcpListener::bind(ADDRESS)?;

    let (mut stream, addr) = listener.accept()?;

    println!("Client connected: {addr}");

    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    println!("{}", String::from_utf8_lossy(&buffer));

    Ok(())
}
