pub mod server {
    use std::io::{Read, Write};
    use std::net::{TcpListener};

    pub fn run() -> std::io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:34254")?;
        for conn in listener.incoming() {
            let mut sock = conn?;
            let mut buf = [0u8; 128];
            let n = sock.read(&mut buf)?;
            println!("Server got {} bytes: {:?}", n, String::from_utf8_lossy(&buf[..n]));
            let _ = sock.write_all(b"pong")?;
        }
        Ok(())
    }
}

pub mod client {
    use std::io::{Read, Write};
    use std::net::TcpStream;

    pub fn run() -> std::io::Result<()> {
        let mut stream = TcpStream::connect("127.0.0.1:34254")?;
        stream.write_all(b"ping")?;
        let mut buf = [0u8; 128];
        let n = stream.read(&mut buf)?;
        println!("Client got {} bytes: {:?}", n, String::from_utf8_lossy(&buf[..n]));
        Ok(())
    }
}