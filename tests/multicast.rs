use mctk;

use std::net::SocketAddr;

#[test]
fn test_multicast() {
    let address = "224.1.1.1".parse().unwrap();
    let sockaddr = SocketAddr::new(address, 8201);

    let mut rcvbuf = [0u8; 65536];

    let sender = mctk::new_sender().expect("unable to create sender");
    let listener = mctk::join_multicast(sockaddr).expect("unable to create multicast listener");

    mctk::send_message(&sender, sockaddr, 50, 17);
    let (len, _remote_addr) = listener.recv_from(&mut rcvbuf).unwrap();

    let received_message = &String::from_utf8_lossy(&rcvbuf[..len]);
    assert_eq!(50, received_message.len());
    assert_eq!(
        "test multicast message 17 of length 50 xxxxxxxxxxx",
        received_message
    );
}

#[test]
fn test_generate_message() {
    let size: u16 = 1000;
    let msg = mctk::generate_message(size, 142);
    assert_eq!(size as usize, msg.len());
}
