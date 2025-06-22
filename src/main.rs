
pub mod handler_struct;
pub mod messages;
pub mod interactions;
pub mod utils;
pub mod error;

pub mod secrets;

use secrets::TOKEN as token;
use serenity::all::GatewayIntents;
use signal_hook::{consts::SIGINT, iterator::Signals};


async fn a_saver(g_data: std::sync::Arc<tokio::sync::Mutex<crate::handler_struct::GuildInstances>>) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(60 * 15));
    loop {
        interval.tick().await;
        let g_data = g_data.lock().await;
        g_data.save_to_file("guild_data.json").await;   
    }
}

async fn sig_handler(g_data: std::sync::Arc<tokio::sync::Mutex<crate::handler_struct::GuildInstances>>) {
    let mut signals = Signals::new(&[SIGINT]).expect("Error creating signal handler");


    let signals = signals.forever();
    for _ in signals {
        println!("Shutting down");
        g_data.lock().await.save_to_file("guild_data.json").await;
        std::process::exit(0);
    }
}

#[tokio::main]
async fn main() {

    let g_data = crate::handler_struct::GuildInstances::from_file("guild_data.json").await;
    let am_g_data = std::sync::Arc::new(tokio::sync::Mutex::new(g_data));

    let g1 = am_g_data.clone();
    tokio::spawn(async move {
        sig_handler(g1).await
    });
    let g2 = am_g_data.clone();
    tokio::spawn(async move {
        a_saver(g2).await
    });
    let g3 = am_g_data.clone();
    let handler = handler_struct::Handler { guild_instances: g3 };

    let intents = GatewayIntents::non_privileged() | GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES;


    let mut client = serenity::Client::builder(token, intents).event_handler(handler).await.expect("Error Creating Client");
    if let Err(why) = client.start().await {
        eprintln!("Client error: {why:?}");
        std::process::exit(1);
    }


}
