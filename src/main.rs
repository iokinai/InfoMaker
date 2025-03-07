mod bot;
mod cfg;

use std::sync::Arc;

use bot::GlobalBot;
use clap::Parser;
use log::{error, info};
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
    pretty_env_logger::init();

    info!("App loading");

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
        info!("Loading states");

        let states = match load_states(String::from(args.states)).await {
            Ok(st) => st,
            Err(e) => {
                error!("Error while loading states: {:?}", e);
                panic!("{:?}", e);
            }
        };

        info!("Initializing bot");

        let bot = Bot::new(args.token);

        info!("Bot initialized successfully");

        info!("Parsing states");

        let global = Arc::new(GlobalBot::new(bot, states));

        info!("Starting bot");
        global.run().await;
    })
    .await
    .unwrap();
}
