use smol::net::{SocketAddr, UdpSocket};

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

async fn forward_packet(config: &Config, socket: &UdpSocket, buf: &[u8]) -> Result<(), std::io::Error> {
    for downstream in &config.downstream {
        match socket.send_to(buf, downstream).await {
            Ok(_bytes_read) => {}
            Err(e) => {
                eprintln!("Error muxing packet of {} bytes to {downstream}: {e}", buf.len());
            }
        };
    };
    Ok(())
}

async fn shutdown(shutdown_chan: &smol::channel::Receiver<()>) -> Result<(), smol::channel::RecvError> {
    match shutdown_chan.recv().await {
        Ok(())  => {
            println!("Shutting down trivial udp mux");
            std::process::exit(0);
        }
        Err(e) => {
            eprint!("Error while receiving shutdown signal: {e}");
            std::process::exit(1);
        }
    };
}

async fn handle_packet(config: &Config, listener_socket: &UdpSocket, buf: &mut [u8]) -> Result<(), smol::channel::RecvError> {
    match listener_socket.recv(buf).await {
        Ok(bytes_read) => {
            match forward_packet(&config, &listener_socket, &buf[..bytes_read]).await {
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
    Ok(())
}

async fn main_loop(config: &Config, shutdown_chan: smol::channel::Receiver<()>) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = vec![0_u8; config.max_packet_size];
    let listener_socket = UdpSocket::bind(config.bind).await?;
    loop {
        smol::future::race(shutdown(&shutdown_chan), handle_packet(config, &listener_socket, &mut buf)).await?;
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();
    let (tx, rx) = smol::channel::bounded(1);

    ctrlc::set_handler(move || {
        smol::block_on(async {
            tx.send(()).await.expect("Could not send shutdown message.");
        })
    }).expect("Could not install signal handler.");

    println!("Starting the trivial udp muxer.");
    smol::block_on(main_loop(&config, rx))
}

