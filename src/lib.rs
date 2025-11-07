pub(crate) mod tcp;

pub mod server {
    use std::net::{TcpListener};

    use crate::tcp::{receive_message};

    pub fn run() -> std::io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:34254")?;

        let mut buffer: Vec<u8> = Vec::new();

        let mut last_seen = 0;

        for conn in listener.incoming() {
            let mut sock = conn?;
            loop {
                let msg = receive_message(&mut sock)?;
                if msg.is_empty() {
                    break;
                }
                
                buffer.extend_from_slice(&msg);

                match buffer[0] {
                    0x2B => { // '+': simple string
                        for i in last_seen..buffer.len() {
                            if buffer[i] == 0x0A
                                && i > 0
                                && buffer[i-1] == 0x0D {
                                let good_string = &buffer[1..i-1];
                                println!("string {}", String::from_utf8_lossy(good_string));
                                buffer.clear();
                                last_seen = 0;
                                break;
                            }
                        }
                        if !buffer.is_empty() {
                            last_seen = buffer.len().saturating_sub(1);
                        }
                    },
                    0x2D => { // '-': error 

                    },
                    0x3A => { // ':': integer 

                    },
                    0x24 => { // '$': bulk string 

                    },
                    0x2A => { // '*': array 

                    },
                    _ => {
                        panic!("Unknown operator byte value: {}", buffer[0])
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

    use crate::tcp::{send_message};

    pub fn run() -> std::io::Result<()> {
        let mut stream = TcpStream::connect("127.0.0.1:34254")?;
        
        loop {
            let msg = input!("");

            send_message(&mut stream, msg.as_bytes())?;
    
            // let response = receive_message(&mut stream)?;

            // if response.is_empty() {
            //     break;
            // }

            // println!("Received message: {}", response);
            
        }

        // Ok(())
    }
}