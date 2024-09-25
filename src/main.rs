use std::net::{SocketAddr, UdpSocket};

use serde::Deserialize;
use serde_yaml::from_reader;

#[derive(Deserialize)]
pub struct Config {
    pub bind_addr: String,
    pub bind_port: u16,
    pub max_packet_size: Option<usize>,
    pub downstreams: Vec<SocketAddr>,
}

fn forward_packet(config: &Config, socket: &UdpSocket, buf: &Vec<u8>) -> Result<(), std::io::Error> {
    for downstream in &config.downstreams {
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
    let conf_reader = std::fs::File::open("/etc/trivial_udp_mux/config.yaml")?;
    let config: Config = from_reader(conf_reader)?;

    println!("Starting the trivial udp muxer.");
    let mut buf = vec![0_u8; config.max_packet_size.unwrap_or(usize::from(u16::MAX))];
    let listener_socket = UdpSocket::bind(format!("{}:{}", config.bind_addr, config.bind_port))?;
    loop {
        match listener_socket.recv(&mut buf) {
            Ok(_bytes_read) => {
                match forward_packet(&config, &listener_socket, &buf) {
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

