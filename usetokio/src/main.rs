use tokio::io;
use tokio::task;
use tokio::net::{TcpListener, TcpStream};


#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    let mut listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        task::spawn(echo(socket));
    }

}

async fn echo(socket: TcpStream) -> io::Result<()> {
    let (mut recv, mut send) = io::split(socket);
    io::copy(&mut recv, &mut send).await?;
    Ok(())
}
