use crate::shell::command::CommandExpr;
use crate::shell::eval::{EvalResult, eval_expr};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedOperator(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::UnexpectedOperator(ref op) => {
                write!(f, "syntax error: unexpected '{}'", op)
            }
        }
    }
}

pub fn parse_and_execute(
    input: &str,
    aliases: &HashMap<String, String>,
) -> Result<bool, ParseError> {
    if input.trim().is_empty() {
        return Ok(false);
    }

    let mut tokens = tokenize(input)?;

    let expr = parse_expr(&mut tokens)?;

    if let Some(EvalResult {
        success: _,
        should_exit,
    }) = eval_expr(expr, aliases)
    {
        return Ok(should_exit);
    }

    Ok(false)
}

pub fn tokenize(input: &str) -> Result<Vec<String>, ParseError> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_double_quotes = false;
    let mut in_single_quotes = false;

    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        match c {
            '"' => in_double_quotes = !in_double_quotes,
            '\'' => in_single_quotes = !in_single_quotes,
            ' ' if !in_double_quotes && !in_single_quotes => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            '&' | '|' if !in_double_quotes && !in_single_quotes => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }

                if i + 1 < chars.len() && chars[i + 1] == c {
                    tokens.push(format!("{}{}", c, c));
                    i += 1; // consume both
                } else {
                    tokens.push(c.to_string());
                }
            }
            ';' if !in_double_quotes && !in_single_quotes => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                tokens.push(c.to_string());
            }
            _ => current.push(c),
        }

        i += 1;
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    Ok(tokens)
}

pub fn parse_expr(tokens: &mut Vec<String>) -> Result<CommandExpr, ParseError> {
    let mut lhs = parse_simple_expr(tokens)?;

    while let Some(op) = tokens.first() {
        match op.as_str() {
            "&&" | "||" => {
                let op = tokens.remove(0);
                let rhs = parse_expr(tokens)?;
                lhs = match op.as_str() {
                    "&&" => CommandExpr::And(Box::new(lhs), Box::new(rhs)),
                    "||" => CommandExpr::Or(Box::new(lhs), Box::new(rhs)),
                    _ => unreachable!(),
                };
            }
            ";" => {
                tokens.remove(0);
                let rhs = parse_expr(tokens)?;
                lhs = CommandExpr::Sequence(vec![lhs, rhs]);
            }
            _ => break,
        }
    }

    Ok(lhs)
}

fn parse_simple_expr(tokens: &mut Vec<String>) -> Result<CommandExpr, ParseError> {
    if tokens.is_empty() {
        return Err(ParseError::UnexpectedOperator("empty".to_string()));
    }

    let mut pipeline = Vec::new();

    // First command
    let mut cmd = Vec::new();
    while !tokens.is_empty() && !["|", "&&", "||", ";"].contains(&tokens[0].as_str()) {
        cmd.push(tokens.remove(0));
    }

    if cmd.is_empty() {
        return Err(ParseError::UnexpectedOperator(
            "expected command".to_string(),
        ));
    }

    pipeline.push(CommandExpr::Command(cmd));

    // If there are pipes, collect all commands in the pipeline
    while let Some(tok) = tokens.first() {
        if tok == "|" {
            tokens.remove(0); // consume the pipe
            let mut next_cmd = Vec::new();
            while !tokens.is_empty() && !["|", "&&", "||", ";"].contains(&tokens[0].as_str()) {
                next_cmd.push(tokens.remove(0));
            }

            if next_cmd.is_empty() {
                return Err(ParseError::UnexpectedOperator(
                    "expected command after |".to_string(),
                ));
            }

            pipeline.push(CommandExpr::Command(next_cmd));
        } else {
            break;
        }
    }

    if pipeline.len() == 1 {
        Ok(pipeline.pop().unwrap())
    } else {
        Ok(CommandExpr::Pipeline(pipeline))
    }
}
