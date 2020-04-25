use tokio::io;


#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    let mut stdout = io::stdout();
    let mut hello: &[u8] = b"Hello world!\n";
    io::copy(&mut hello, &mut stdout).await?;
    Ok(())
}
