use std::net::{SocketAddr, UdpSocket};

use clap::Parser;

#[derive(Parser,Debug)]
#[command(version, about, long_about = None)]
struct Config {
    /// Bind to this socket (ip:port)
    #[arg(short, long)]
    bind: SocketAddr,
    /// Downstreams to mux packets to (host:port)
    #[arg(short, long)]
    downstream: Vec<SocketAddr>,
    /// Truncate packets at this number of bytes
    #[arg(long, default_value_t = u16::MAX as usize)]
    max_packet_size: usize,
}

fn forward_packet(config: &Config, socket: &UdpSocket, buf: &[u8]) -> Result<(), std::io::Error> {
    for downstream in &config.downstream {
        match socket.send_to(buf, downstream) {
            Ok(_bytes_read) => {}
            Err(e) => {
                eprintln!("Error muxing packet of {} bytes to {downstream}: {e}", buf.len());
            }
        };
    };
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();

    println!("Starting the trivial udp muxer.");
    let mut buf = vec![0_u8; config.max_packet_size];
    let listener_socket = UdpSocket::bind(config.bind)?;
    loop {
        match listener_socket.recv(&mut buf) {
            Ok(bytes_read) => {
                match forward_packet(&config, &listener_socket, &buf[..bytes_read]) {
                    Ok(()) => {}
                    Err(e) => {
                        eprintln!("Received a UDP packet, but failed to forward it to all downstreams: {e}.");
                    }
                };
            }
            Err(e) => {
                eprintln!("Error reading from UdpSocket: {e}.");
            }
        };
    };
}

