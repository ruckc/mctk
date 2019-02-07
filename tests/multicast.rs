use mctk;

use std::net::SocketAddr;

#[test]
fn test_multicast() {
    let address = "224.1.1.1".parse().unwrap();
    let sockaddr = SocketAddr::new(address, 8201);

    let message = "a test message";
    let mut rcvbuf = [0u8; 65536];
    let mut buflen: usize = 0;

    let sender = mctk::new_sender().expect("unable to create sender");
    let listener = mctk::join_multicast(sockaddr).expect("unable to create multicast listener");

    sender
        .send_to(message.as_bytes(), &sockaddr)
        .expect("unable to send message");
    match listener.recv_from(&mut rcvbuf) {
        Ok((len, remote_addr)) => {
            buflen = len;
            println!(
                "{} sent {}",
                remote_addr,
                String::from_utf8_lossy(&rcvbuf[..len])
            );
        }
        Err(err) => {
            eprintln!("an error occurred: {}", err);
        }
    }

    let received_message = &String::from_utf8_lossy(&rcvbuf[..buflen]);
    assert_eq!(message, received_message);
}
