use std::net::{UdpSocket,SocketAddr};
fn main()->Result<(),std::io::Error> {
	let server = SocketAddr::from(([101,35,230,139],8050));
	let socket = UdpSocket::bind(SocketAddr::from(([0,0,0,0],0)))?;
	socket.send_to(b"conn", server.clone())?;
	let mut buf = [0u8;1024];
	let remote_addr = match socket.recv(& mut buf){
		Ok(size)=>{
			String::from_utf8_lossy(&buf[..size]).to_string()
		}
		Err(e)=>{
			return Err(e);
		}
	};
	socket.set_read_timeout(Some(std::time::Duration::from_secs(2)))?;
	println!("remote addr {}",remote_addr);

	socket.send_to(b"hello", remote_addr.clone())?;

	socket.send_to(b"ok", server.clone())?;

	//socket.send_to(b"hello", s.clone())?;

	loop{
		socket.send_to(b"ok", remote_addr.clone())?;
		println!("send hello");
		let mut buff2 =  [0u8;1024];
		match socket.recv_from(& mut buff2){
			Ok((size,addr))=>{
				if addr.to_string() == remote_addr && size == 2{
					println!("receive from {addr}, buff: {}",String::from_utf8_lossy(&buff2[..size]));
					if &buff2[..size] == b"ok"{
						break;
					}
				}
			}
			Err(e)=>{
				println!("{e:?} {}",line!());
			}
		}
		//std::thread::sleep(std::time::Duration::from_secs(2));
	}

	let mut buff2 =  [0u8;1024];
	println!("send subject");
	let msg = format!("normative text {}",std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis());
	socket.send_to(msg.as_bytes(), remote_addr.clone())?;
	loop{
		//socket.send_to(b"ok", s.clone())?;
		match socket.recv_from(& mut buff2){
			Ok((size,_addr))=>{
				if size == 2 && &buff2[..size] == b"ok"{
					//println!("noise");
					socket.send_to(b"ok", remote_addr.clone())?;
				}else{
					println!("buff: {}",String::from_utf8_lossy(&buff2[..size]));
					break;
				}
			}
			Err(e)=>{
				println!("{e:?}");
			}
		}
		//std::thread::sleep(std::time::Duration::from_secs(2));
	}
	Ok(())
}