#![allow(while_true)]
extern crate clap;
extern crate my_pretty_failure;

use clap::{App, Arg, SubCommand};
use my_pretty_failure::myprettyfailure;

use std::net::{IpAddr, SocketAddr};
use std::thread::sleep;
use std::time::Duration;

mod lib;

#[cfg_attr(tarpaulin, skip)]
fn main() {
    let matches = App::new("multicast toolkit")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Curtis Ruck <curtis@ruck.io>")
        .about("supports testing of multicast routing and packet flow")
        .subcommand(
            SubCommand::with_name("send")
                .about("Runs a multicast sender")
                .arg(
                    Arg::with_name("group")
                        .short("g")
                        .long("group")
                        .value_name("group")
                        .help("Sets the multicast group")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("port")
                        .short("p")
                        .long("port")
                        .value_name("port")
                        .help("Sets the multicast port")
                        .default_value("8201")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("interval")
                        .short("i")
                        .long("interval")
                        .value_name("milliseconds")
                        .help("Sets the sender's packet transmission interval")
                        .default_value("1000")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("packet-size")
                        .short("s")
                        .long("packet-size")
                        .value_name("bytes")
                        .help("Sets the sender's packet size")
                        .default_value("1000")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("ttl")
                        .short("t")
                        .long("ttl")
                        .value_name("ttl")
                        .help("Sets the sender's time to live value")
                        .takes_value(true)
                        .default_value("2"),
                ),
        )
        .subcommand(
            SubCommand::with_name("receive")
                .about("Runs a multicast receiver")
                .arg(
                    Arg::with_name("group")
                        .short("g")
                        .long("group")
                        .value_name("group")
                        .help("Sets the multicast group")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("port")
                        .short("p")
                        .long("port")
                        .value_name("port")
                        .help("Sets the multicast port")
                        .default_value("8201")
                        .takes_value(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("send") {
        let group: IpAddr = matches.value_of("group").unwrap().parse().unwrap();
        let port: u16 = matches.value_of("port").unwrap().parse().unwrap();
        let sleep_interval: Duration =
            Duration::from_millis(matches.value_of("interval").unwrap().parse().unwrap());
        let packet_size: u16 = matches.value_of("packet-size").unwrap().parse().unwrap();
        let ttl: u8 = matches.value_of("ttl").unwrap().parse().unwrap();
        let destination_addr = SocketAddr::new(group, port);
        println!("Running a sender to {}", destination_addr);
        let socket: socket2::Socket = lib::new_sender(ttl).expect("could not create sender");
        let mut count: u32 = 0;
        while true {
            count += 1;
            lib::send_message(&socket, destination_addr, packet_size, count);
            sleep(sleep_interval);
        }
    } else if let Some(matches) = matches.subcommand_matches("receive") {
        let group: IpAddr = matches.value_of("group").unwrap().parse().unwrap();
        let port: u16 = matches.value_of("port").unwrap().parse().unwrap();
        println!(
            "Listening for {}:{}",
            matches.value_of("group").unwrap(),
            matches.value_of("port").unwrap()
        );
        let destination_addr = SocketAddr::new(group, port);
        let mut buf = [0u8; 65536]; //receive buffer
        let listener =
            lib::join_multicast(destination_addr).expect("failed to create multicast listener");
        while true {
            match listener.recv_from(&mut buf) {
                Ok((len, remote_addr)) => {
                    let data = &buf[..len];
                    println!("{} sent {}", remote_addr, String::from_utf8_lossy(data));
                }
                Err(err) => {
                    eprintln!("an error occurred: {}", myprettyfailure(&err));
                }
            }
        }
    }
}
