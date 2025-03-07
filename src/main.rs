mod bot;
mod cfg;

use std::sync::Arc;

use bot::GlobalBot;
use tokio::io;

use dotenv::dotenv;
use teloxide::Bot;
use tokio::main;

use cfg::ConfigState;

async fn load_states(path: String) -> io::Result<Vec<ConfigState>> {
    let json = tokio::fs::read(path).await?;

    let result: Vec<ConfigState> = serde_json::from_slice(&json)?;

    Ok(result)
}

#[main]
async fn main() {
    let states = load_states(String::from(
        "/home/okinai/code/chembot/target/debug/test.json",
    ))
    .await
    .unwrap();

    println!("{:?}", states);

    dotenv().ok();

    let bot = Bot::from_env();

    let global = GlobalBot::new(bot, states);
    let global = Arc::new(global);

    global.run().await;
}
