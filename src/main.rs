use std::thread;
use std::net;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

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

fn read_message(server_name: &str, socket: &net::UdpSocket) {
    let mut buf: [u8; 10] = [0; 10];
    match socket.recv_from(&mut buf) {
        Err(err) => {
            println!("failed to read {}", err);
        }
        Ok((x, addr)) => {
            println!("{} got {:?}",
                     server_name,
                     String::from_utf8_lossy(&buf[0..x]));
            match socket.send_to(&mut &buf[0..x], addr) {
                Ok(_) => println!("sent"),
                Err(err) => println!("failed to send {}", err),
            }
        }
    };
}

fn listen(server_name: &str, sock: net::UdpSocket, listen_on: net::SocketAddr) {
    for i in 0..50 {
        read_message(server_name, &sock);
    }
}

fn send(sock: &net::UdpSocket, addr: net::SocketAddr, msg: &[u8]) {
    sock.send_to(&msg, addr);
}

fn server(server_name: &str,
          server_addr: net::SocketAddr,
          other_addr: net::SocketAddr,
          msg: &[u8]) {
    let sock = socket(server_addr);
    send(&sock, other_addr, &msg);
    listen(server_name, sock, server_addr);
    drop(socket);
}

fn main() {
    let ip = net::Ipv4Addr::new(127, 0, 0, 1);
    let server_listen = net::SocketAddrV4::new(ip, 4444);
    let client_listen = net::SocketAddrV4::new(ip, 5555);
    let server_addr = net::SocketAddr::V4(server_listen);
    let client_addr = net::SocketAddr::V4(client_listen);
    let msg_a: &[u8] = b"msg A";
    let msg_b: &[u8] = b"msg B";

    let t0 = thread::spawn(move || server("A", server_addr, client_addr, msg_a));
    let t1 = thread::spawn(move || server("B", client_addr, server_addr, msg_b));
    let _ = t0.join();
    let _ = t1.join();
}
