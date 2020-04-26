use tokio::time;
use tokio::process::Command;
use std::time::Duration;


#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    let mut interval = time::interval(Duration::from_secs(1));

    loop {
        interval.tick().await;
        Command::new("date").spawn()?.await?;
    }

}
