extern crate socket2;

use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};

// this will be common for all our sockets
fn new_socket() -> io::Result<Socket> {
    let domain = Domain::ipv4();

    let socket = Socket::new(domain, Type::dgram(), Some(Protocol::udp()))?;

    // we're going to use read timeouts so that we don't hang waiting for packets
    //socket.set_read_timeout(Some(Duration::from_millis(100)))?;

    Ok(socket)
}

/// On Windows, unlike all Unix variants, it is improper to bind to the multicast address
///
/// see https://msdn.microsoft.com/en-us/library/windows/desktop/ms737550(v=vs.85).aspx
#[cfg(windows)]
fn bind_multicast(socket: &Socket, addr: &SocketAddr) -> io::Result<()> {
    let addr = SocketAddr::new(Ipv4Addr::new(0, 0, 0, 0).into(), addr.port());
    socket.bind(&socket2::SockAddr::from(addr))
}

/// On unixes we bind to the multicast address, which causes multicast packets to be filtered
#[cfg(unix)]
fn bind_multicast(socket: &Socket, addr: &SocketAddr) -> io::Result<()> {
    socket.bind(&socket2::SockAddr::from(*addr))
}

pub fn join_multicast(addr: SocketAddr) -> io::Result<UdpSocket> {
    let ip_addr = addr.ip();

    let socket = new_socket()?;

    if let IpAddr::V4(ip) = ip_addr {
        socket.join_multicast_v4(&ip, &Ipv4Addr::new(0, 0, 0, 0))?;
    }

    // bind us to the socket address.
    bind_multicast(&socket, &addr)?;

    // convert to standard sockets
    Ok(socket.into_udp_socket())
}

pub fn new_sender() -> io::Result<Socket> {
    let socket = new_socket()?;

    socket.set_multicast_if_v4(&Ipv4Addr::new(0, 0, 0, 0))?;

    socket.bind(&SockAddr::from(SocketAddr::new(
        Ipv4Addr::new(0, 0, 0, 0).into(),
        0,
    )))?;

    Ok(socket)
}

pub fn send_message(socket: &Socket, destination: SocketAddr, packet_size: u16, msgid: u32) {
    let message = generate_message(packet_size, msgid);
    socket
        .send_to(message.as_bytes(), &SockAddr::from(destination))
        .expect(&format!("could not send {} to {}", message, destination));
}

pub fn generate_message(size: u16, count: u32) -> String {
    let mut message = format!("test multicast message {} of length {} ", count, size);
    message.push_str(&"x".repeat((size as usize) - message.len()));
    message
}
