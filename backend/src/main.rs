use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};
use tokio_tungstenite::tungstenite::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr)
        .await
        .expect(&format!("Failed to bind to {addr}"));

    println!("Listening on {addr}");
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_stream(stream));
    }

    Ok(())
}

async fn handle_stream(mut stream: TcpStream) -> Result<()> {
    let mut buf: &mut [u8] = &mut [0; 1024];
    stream.read(&mut buf).await?;
    print!("{:x?}", buf);

    Ok(())
}
