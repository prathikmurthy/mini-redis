pub(crate) mod tcp;

pub mod server {
    use std::net::{TcpListener};

    use crate::tcp::{receive_message, send_message};

    pub fn run() -> std::io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:34254")?;
        for conn in listener.incoming() {
            let mut sock = conn?;
            let msg = receive_message(&mut sock)?;
            println!("Received message: {}", msg);

            send_message(&mut sock, "pong")?;
        }
        Ok(())
    }
}

pub mod client {
    use std::net::TcpStream;

    use crate::tcp::{receive_message, send_message};

    pub fn run() -> std::io::Result<()> {
        let mut stream = TcpStream::connect("127.0.0.1:34254")?;

        send_message(&mut stream, "ping")?;

        let response = receive_message(&mut stream)?;
        println!("Received message: {}", response);
        Ok(())
    }
}