use std::{io::Write, net::TcpStream};

fn main() -> std::io::Result<()> {
    const ADDRESS: &str = "127.0.0.1:8080";

    let mut stream: TcpStream = TcpStream::connect(ADDRESS)?;

    let text: &str = "Hello, world!";
    stream.write(text.as_bytes())?;

    Ok(())
}
