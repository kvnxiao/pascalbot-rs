use kalk::parser;

// Use double-precision floating-point format
const PRECISION: u32 = 53;

// Use semicolon as command separator since Discord does not support new-line characters
// in slash commands yet.
const DISCORD_COMMAND_SEPARATOR: &str = ";";

pub struct Evaluation {
    pub expression: String,
    pub result: String,
}

fn evaluate_expression_lines(expression_lines: &[&str]) -> String {
    let mut parser_context = parser::Context::new();

    if let Some((last, lines)) = expression_lines.split_last() {
        for line in lines {
            let result = parser::eval(&mut parser_context, line, PRECISION);
            match result {
                Ok(_) => {}
                Err(err) => return err.to_string(),
            }
        }
        let final_result = parser::eval(&mut parser_context, last, PRECISION);
        return match final_result {
            Ok(res) => res.map(|r| r.to_string_pretty()).unwrap_or_else(|| {
                "The expression was incomplete and did not evaluate to a value.".into()
            }),
            Err(err) => err.to_string(),
        };
    };
    "The expression was empty.".into()
}

pub fn evaluate_expression(expression: &str) -> anyhow::Result<Evaluation> {
    let expression_lines = expression
        .split(DISCORD_COMMAND_SEPARATOR)
        .map(|expr| expr.trim())
        .collect::<Vec<&str>>();
    let expression_string = expression_lines.join("\n");
    let evaluation_result = evaluate_expression_lines(&expression_lines);

    Ok(Evaluation {
        expression: expression_string,
        result: evaluation_result,
    })
}
