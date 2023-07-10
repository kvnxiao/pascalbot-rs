mod api;
pub mod commands;
mod process;
use crate::{
    commands::{eight_ball::EightballCommand, xkcd::XkcdCommand},
    process::process_interactions,
};
use anyhow::Context;
use futures_util::StreamExt;
use std::{env, sync::Arc};
use tracing::Level;
use twilight_gateway::{
    stream::{self, ShardEventStream},
    Config, Intents,
};
use twilight_http::Client;
use twilight_interactions::command::CreateCommand;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = env::var("DISCORD_TOKEN").context("DISCORD_TOKEN environment variable not set")?;

    // Initialize logging with tracing
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(Level::INFO)
        .init();

    // Initialize Twilight HTTP client and gateway configuration.
    let client = Arc::new(Client::new(token.clone()));
    let config = Config::builder(token.clone(), Intents::empty()).build();

    // Register global commands.
    let commands = [
        XkcdCommand::create_command().into(),
        EightballCommand::create_command().into(),
    ];

    let application = client.current_user_application().await?.model().await?;
    let interaction_client = client.interaction(application.id);

    tracing::info!("Logged as {} with ID {}", application.name, application.id);

    if let Err(error) = interaction_client.set_global_commands(&commands).await {
        tracing::error!(?error, "failed to register commands");
    }

    // Start gateway shards.
    let mut shards = stream::create_recommended(&client, config, |_id, builder| builder.build())
        .await?
        .collect::<Vec<_>>();
    let mut stream = ShardEventStream::new(shards.iter_mut());

    // Process Discord events (see `process.rs` file).
    while let Some((shard, event)) = stream.next().await {
        let event = match event {
            Ok(event) => event,
            Err(error) => {
                if error.is_fatal() {
                    tracing::error!(?error, "fatal error while receiving event");
                    break;
                }

                tracing::warn!(?error, "error while receiving event");
                continue;
            }
        };

        tracing::info!(kind = ?event.kind(), shard = ?shard.id().number(), "received event");
        tokio::spawn(process_interactions(event, client.clone()));
    }

    Ok(())
}
