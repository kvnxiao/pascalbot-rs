use anyhow::Context;
use twilight_http::Client;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::{application_command::CommandData, Interaction},
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::api::eval::evaluate_expression;

/// Evaluate a mathematical expression
#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "eval")]
pub struct EvalCommand {
    /// The mathematical expression to evaluate
    pub expression: String,
}

impl EvalCommand {
    pub async fn handle(
        interaction: Interaction,
        data: CommandData,
        client: &Client,
    ) -> anyhow::Result<()> {
        let command =
            EvalCommand::from_interaction(data.into()).context("failed to parse command data")?;

        let evaluation = evaluate_expression(&command.expression);
        let message = if evaluation.is_error {
            format!("{}\n\n_**{}**_", evaluation.expression, evaluation.result)
        } else {
            format!("{}\n\n**= {}**", evaluation.expression, evaluation.result)
        };
        let data = InteractionResponseDataBuilder::new()
            .content(message)
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
