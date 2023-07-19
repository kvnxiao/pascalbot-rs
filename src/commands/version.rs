use crate::api::version::{
    get_git_commit_short_sha, get_git_commit_time_formatted, get_git_commit_url,
};
use twilight_http::Client;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::{application_command::CommandData, Interaction},
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::{
    embed::{EmbedBuilder, EmbedFieldBuilder},
    InteractionResponseDataBuilder,
};

/// Prints the version of the current bot
#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "version")]
pub struct VersionCommand;

impl VersionCommand {
    pub async fn handle(
        interaction: Interaction,
        _data: CommandData,
        client: &Client,
    ) -> anyhow::Result<()> {
        let embed = EmbedBuilder::new()
            .field(
                EmbedFieldBuilder::new("Date", get_git_commit_time_formatted()?)
                    .inline()
                    .build(),
            )
            .field(
                EmbedFieldBuilder::new(
                    "Version",
                    format!("[{}]({})", get_git_commit_short_sha(), get_git_commit_url()),
                )
                .inline()
                .build(),
            )
            .build();
        let data = InteractionResponseDataBuilder::new()
            .embeds([embed])
            .build();
        let client = client.interaction(interaction.application_id);
        let response = InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(data),
        };

        client
            .create_response(interaction.id, &interaction.token, &response)
            .await?;

        Ok(())
    }
}
