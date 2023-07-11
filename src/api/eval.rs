use anyhow::anyhow;
use anyhow::Context;
use kalk::parser;

// Use double-precision floating-point format
const PRECISION: u32 = 53;

// Use semicolon as command separator since Discord does not support new-line characters
// in slash commands yet.
const DISCORD_COMMAND_SEPARATOR: &str = ";";

pub struct Evaluation {
    pub expression: String,
    pub result: String,
    pub is_error: bool,
}

fn evaluate_expression_lines(expression_lines: &[&str]) -> anyhow::Result<String> {
    let mut parser_context = parser::Context::new();

    if let Some((last, lines)) = expression_lines.split_last() {
        for line in lines {
            match parser::eval(&mut parser_context, line, PRECISION) {
                Ok(_) => {}
                Err(err) => return Err(anyhow!(err.to_string())),
            }
        }
        return match parser::eval(&mut parser_context, last, PRECISION) {
            Ok(res) => res
                .map(|r| r.to_string_pretty())
                .context("The expression was incomplete and did not evaluate to a value."),
            Err(err) => Err(anyhow!(err.to_string())),
        };
    };
    Err(anyhow!("The expression was empty."))
}

pub fn evaluate_expression(expression: &str) -> Evaluation {
    let expression_lines = expression
        .split(DISCORD_COMMAND_SEPARATOR)
        .map(|expr| expr.trim())
        .collect::<Vec<&str>>();
    let expression_string = expression_lines.join("\n");
    let evaluation_result = evaluate_expression_lines(&expression_lines);

    match evaluation_result {
        Ok(res) => Evaluation {
            expression: expression_string,
            result: res,
            is_error: false,
        },
        Err(err) => Evaluation {
            expression: expression_string,
            result: err.to_string(),
            is_error: true,
        },
    }
}
