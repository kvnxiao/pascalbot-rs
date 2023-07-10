use crate::api::roll::get_random_roll;
use anyhow::Context;
use twilight_http::Client;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_mention::Mention;
use twilight_model::{
    application::interaction::{application_command::CommandData, Interaction},
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::InteractionResponseDataBuilder;

/// Roll a random number between 0-N (default to 0-100)
#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "roll")]
pub struct RollCommand {
    /// Max number to roll up to (inclusive)
    #[command(min_value = 0)]
    pub max: Option<i64>,
}

const DEFAULT_ROLL_MAX: i64 = 100;

impl RollCommand {
    pub async fn handle(
        interaction: Interaction,
        data: CommandData,
        client: &Client,
    ) -> anyhow::Result<()> {
        let command =
            RollCommand::from_interaction(data.into()).context("failed to parse command data")?;

        if command.max.unwrap_or(DEFAULT_ROLL_MAX) <= 0 {
            let data = InteractionResponseDataBuilder::new()
                .content("Can't roll on a number less than or equal to 0!")
                .build();
            let client = client.interaction(interaction.application_id);
            let response = InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: Some(data),
            };
            client
                .create_response(interaction.id, &interaction.token, &response)
                .await?;

            return Ok(());
        }

        let random_roll = get_random_roll(command.max.unwrap_or(DEFAULT_ROLL_MAX).unsigned_abs());

        let user_mention = interaction
            .member
            .and_then(|partial_member| partial_member.user)
            .context("missing user from interaction")?
            .mention();
        let data = InteractionResponseDataBuilder::new()
            .content(format!("{} rolled a **{}**", user_mention, random_roll))
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
