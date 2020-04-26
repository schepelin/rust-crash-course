use tokio::io;
use tokio::task;
use tokio::time;

use std::time::Duration;
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    task::spawn(date()).await??;
    task::spawn(intoout()).await??;

    Ok(())
}

async fn intoout() -> io::Result<()> {

    let mut input = io::stdin();
    let mut stdout = io::stdout();

    io::copy(&mut input, &mut stdout).await?;
    Ok(())
}


async fn date() -> Result<(), std::io::Error> {
    let mut interval = time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        Command::new("date").spawn()?.await?;
    }
}
