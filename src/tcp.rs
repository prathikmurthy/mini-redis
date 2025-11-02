use std::io::{Read, Write};
use std::net::TcpStream;

pub fn send_message(stream: &mut TcpStream, msg: &[u8]) -> std::io::Result<()> {
    let i = [msg, &[0x0D, 0x0A]].concat();
    stream.write_all(&i)?;
    Ok(())
}

// This has to return String and not &str because the fn owns the res str, can't
// return a borrowed ref to it after function ends
pub fn receive_message(stream: &mut TcpStream) -> std::io::Result<Box<[u8]>> {
    let mut buf = [0u8; 128];
    let n = stream.read(&mut buf)?;
    // let res = String::from_utf8_lossy(&buf[..n]).to_string();
    Ok(buf[..n].to_vec().into_boxed_slice())
}
