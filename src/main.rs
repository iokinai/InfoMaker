mod bot;
mod cfg;

use std::sync::Arc;

use bot::GlobalBot;
use clap::Parser;
use tokio::{io, runtime::Builder};

use teloxide::Bot;

use cfg::ConfigState;

/// Database builder telegram bot
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Config file with states
    #[arg(short, long)]
    states: String,
    /// Telegram bot API Key
    #[arg(short, long)]
    token: String,
}

async fn load_states(path: String) -> io::Result<Vec<ConfigState>> {
    let json = tokio::fs::read(path).await?;

    let result: Vec<ConfigState> = serde_json::from_slice(&json)?;

    Ok(result)
}

fn main() {
    let args = Args::parse();

    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .thread_stack_size(3 * 1024 * 1024)
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async_main(args));
}

async fn async_main(args: Args) {
    tokio::spawn(async {
        let states = load_states(String::from(args.states)).await.unwrap();

        let bot = Bot::new(args.token);

        let global = Arc::new(GlobalBot::new(bot, states));

        global.run().await;
    })
    .await
    .unwrap();
}
