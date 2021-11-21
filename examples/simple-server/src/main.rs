use dotenv::dotenv;
use anyhow::{anyhow, Result, bail};
use socken5::{downstream, upstream, Command, AsyncWrite, AsyncRead, Method, Reply, Addr};

use std::env;
use std::net::{ToSocketAddrs, SocketAddr, Ipv4Addr, IpAddr};

use tokio::io::copy_bidirectional;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let server_addr = env::var("SERVER_ADDRESS")
        .expect("Server address");

    let listener = TcpListener::bind(server_addr)
        .await?;

    loop {
        let (client, _) = listener.accept()
            .await?;

        tokio::spawn(async move {
            handle_client(client).await
        });
    }
}

async fn handle_client(client: TcpStream) -> Result<()> {
    let mut client = client;

    let handshake = upstream::Handshake::read(&mut client)
        .await?;

    if handshake.0.contains(&Method::NoAuth) {
        downstream::Handshake(Method::NoAuth)
            .write(&mut client)
            .await?;
    } else {
        downstream::Handshake(Method::NoAcceptable)
            .write(&mut client)
            .await?;

        bail!("No acceptable method found");
    }

    let request = upstream::CommandRequest::read(&mut client)
        .await?;

    if request.cmd != Command::Connect {
        bail!("Command not implemented");
    }

    let peer_addr = match request.addr {
        Addr::V4(ip) => SocketAddr::new(IpAddr::V4(ip), request.port),
        Addr::V6(ip) => SocketAddr::new(IpAddr::V6(ip), request.port),
        Addr::Domain(domain) => {
            let sockets: Vec<SocketAddr> = format!("{}:{}", domain, request.port)
                .to_socket_addrs()?
                .collect();

            *sockets
                .first()
                .ok_or(anyhow!("No valid address found"))?
        }
    };

    let mut peer = TcpStream::connect(peer_addr)
        .await?;

    downstream::CommandResponse {
        reply: Reply::Success,
        addr: Addr::V4(Ipv4Addr::from(0)),
        port: 0
    }
        .write(&mut client)
        .await?;

    copy_bidirectional(&mut client, &mut peer)
        .await?;
    Ok(())
}
