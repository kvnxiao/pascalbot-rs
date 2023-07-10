use crate::api::eight_ball::get_random_response;
use anyhow::Context;
use twilight_http::Client;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_mention::Mention;
use twilight_model::{
    application::interaction::{application_command::CommandData, Interaction},
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::InteractionResponseDataBuilder;

/// Ask the magic 8-ball a question.
#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "8ball")]
pub struct EightballCommand {
    /// Question to ask the almighty eight ball
    question: String,
}

impl EightballCommand {
    pub async fn handle(
        interaction: Interaction,
        data: CommandData,
        client: &Client,
    ) -> anyhow::Result<()> {
        let command = EightballCommand::from_interaction(data.into())
            .context("failed to parse command data")?;

        let random_response = get_random_response()?;

        let user_mention = interaction
            .member
            .and_then(|partial_member| partial_member.user)
            .context("missing user from interaction")?
            .mention();
        let data = InteractionResponseDataBuilder::new()
            .content(format!(
                "{}: {}\n:8ball:: **{}**",
                user_mention, command.question, random_response
            ))
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
