use backend;
use std::net::SocketAddr;
use std::{fs::File, io::Read};
use tokio::net::{TcpListener, TcpStream};
use tokio_native_tls::{
    native_tls::{self, Identity},
    TlsStream,
};

#[tokio::main]
async fn main() -> backend::Result<()> {
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr)
        .await
        .expect(&format!("Failed to bind to {addr}"));

    // TODO: Generalize the cert path
    let cert_path = "/home/bram/.localhost/127.0.0.1";

    let mut pem_file = File::open(format!("{cert_path}.pem")).expect("Fix hardcoded path");
    let mut pem = vec![];
    pem_file.read_to_end(&mut pem)?;

    let mut key_file = File::open(format!("{cert_path}-key.pem")).expect("Fix hardcoded path");
    let mut key = vec![];
    key_file.read_to_end(&mut key)?;

    let identity = Identity::from_pkcs8(&pem, &key)?;
    let tls_acceptor =
        tokio_native_tls::TlsAcceptor::from(native_tls::TlsAcceptor::builder(identity).build()?);

    println!("Listening on {addr}");
    while let Ok((stream, peer)) = listener.accept().await {
        let tls = tls_acceptor.clone();
        tokio::spawn(async move {
            match tls.accept(stream).await {
                Ok(s) => accept_connection(peer, s).await,
                Err(e) => Err(backend::Error::TlsErr(e)),
            }
        });
    }

    Ok(())
}

async fn accept_connection(peer: SocketAddr, stream: TlsStream<TcpStream>) -> backend::Result<()> {
    let mut ws = tokio_tungstenite::accept_async(stream).await?;
    println!("Connected to peer at {} {}", peer.ip(), peer.port());

    ws.close(None).await?;

    Ok(())
}
