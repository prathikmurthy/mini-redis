pub(crate) mod tcp;

pub mod server {
    use std::net::{TcpListener};

    use crate::tcp::{receive_message, send_message};

    pub fn run() -> std::io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:34254")?;
        for conn in listener.incoming() {
            let mut sock = conn?;
            loop {
                let msg = receive_message(&mut sock)?;
                if msg.is_empty() {
                    break;
                }
                println!("Received message: {}", msg);

                match msg.as_str() {
                    "ping" => {
                        send_message(&mut sock, "pong")?;
                    }
                    "quit" => {
                        send_message(&mut sock, "")?;
                        break;
                    }
                    other => {
                        send_message(&mut sock, other)?;
                    }
                }
            }
        }
        Ok(())
    }
}

pub mod client {
    use std::{net::TcpStream};

    use prompted::input;

    use crate::tcp::{receive_message, send_message};

    pub fn run() -> std::io::Result<()> {
        let mut stream = TcpStream::connect("127.0.0.1:34254")?;

        loop {
            let msg = input!("");

            send_message(&mut stream, &msg)?;
    
            let response = receive_message(&mut stream)?;
            
            if response.is_empty() {
                break;
            }

            println!("Received message: {}", response);
            
        }

        Ok(())
    }
}