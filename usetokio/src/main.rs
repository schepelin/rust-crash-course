use tokio::time;
use tokio::process::Command;
use std::time::Duration;


#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    Command::new("date").spawn()?.await?;
    time::delay_for(Duration::from_secs(1)).await;
    Command::new("date").spawn()?.await?;
    time::delay_for(Duration::from_secs(1)).await;
    Command::new("date").spawn()?.await?;

    Ok(())
}
