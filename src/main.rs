use std::net::{SocketAddr, UdpSocket};
fn main() -> Result<(), std::io::Error> {
    let socket = UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], 8050)))?;
    let mut buf = [0u8; 1024];
    let mut vec = Vec::new();
    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, addr)) => {
                if size == 4 && &buf[..size] == b"conn" {
                    println!("comming addr {}", addr);
                    vec.push(addr);
                    if vec.len() == 2 {
                        let f = vec[0];
                        let s = vec[1];
                        socket.send_to(f.to_string().as_bytes(), s.clone())?;
                        let mut reply_buf = [0u8; 1024];
                        match socket.recv_from(&mut reply_buf) {
                            Ok((size, origin)) => {
                                if origin == s && size == 2 && &reply_buf[..size] == b"ok" {
                                    println!("{size} {origin}");
                                    socket.send_to(s.to_string().as_bytes(), f.clone())?;
                                    vec.clear();
                                }
                            }
                            Err(_) => {}
                        }
                    }
                }
            }
            Err(_) => {}
        }
    }
}
