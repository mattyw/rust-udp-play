use std::thread;
use std::net;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

fn foo(a: &mut i32) -> i32 {
    *a = *a + 1;
    *a + 1
}

fn socket(listen_on: net::SocketAddr) -> net::UdpSocket {
    let attempt = net::UdpSocket::bind(listen_on);
    let mut socket;
    match attempt {
        Ok(sock) => {
            println!("bound to {}", listen_on);
            socket = sock;
        }
        Err(err) => panic!("Could not bind {}", err),
    };
    socket
}

fn read_message(socket: &net::UdpSocket) {
    let mut buf: [u8; 1] = [0; 1];
    let result = socket.recv_from(&mut buf);
    match socket.send(&mut buf) {
        Ok(_) => println!("sent"),
        Err(err) => println!("failed to send {}", err),
    }
    drop(socket);
}

fn listen(listen_on: net::SocketAddr) {
    let sock = socket(listen_on);
    loop {
        read_message(&sock);
    }
}

fn main() {
    let ip = net::Ipv4Addr::new(127, 0, 0, 1);
    let listen_addr = net::SocketAddrV4::new(ip, 4444);
    listen(net::SocketAddr::V4(listen_addr));;
}
