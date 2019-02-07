# mctk - multicasting toolkit
> simple utility to test multicast routing

In troubleshooting multicast routing I wanted a lightweight multicast message sender and receiver.  `mctk` provides an extremely lightweight process that can send packets to a multicast group at a configurable rate of a specified size (to test MTU).  it also can receive the packets.

Written in Rust, it is runs on any platform that [Rust](https://www.rust-lang.org/) supports.  It has been tested on Linux (Redhat 6/7) and Windows 10.

# Basic Usage
This will send small multicast packets every 1 second to the 224.1.1.1 multicast group.
```bash
mctk send -g 224.1.1.1
```

This will join the group, and receive multicast packets.
```bash
mctk receive -g 224.1.1.1
```

In both send/receive you can specify the port (-p).  In send you can also specify the sending interval in milliseconds (-i) and the packet size in bytes (-s).

# Full Usage
```bash
$ mctk -h
multicast toolkit 0.1
Curtis Ruck <curtis@ruck.io>
supports testing of multicast routing and packet flow

USAGE:
    mctk [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    receive    Runs a multicast receiver
    send       Runs a multicast sender
```
```bash
$ mctk help send
mctk-send
Runs a multicast sender

USAGE:
    mctk send [OPTIONS] --group <group>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -g, --group <group>              Sets the multicast group
    -i, --interval <milliseconds>    Sets the sender's packet transmission interval [default: 1000]
    -s, --packet-size <bytes>        Sets the sender's packet size [default: 1000]
    -p, --port <port>                Sets the multicast port [default: 8201]
```
```bash
$ mctk help receive
mctk-receive
Runs a multicast receiver

USAGE:
    mctk receive [OPTIONS] --group <group>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -g, --group <group>    Sets the multicast group
    -p, --port <port>      Sets the multicast port [default: 8201]
