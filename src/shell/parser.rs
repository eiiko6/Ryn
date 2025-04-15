use std::fmt;
use crate::shell::command::CommandExpr;
use crate::shell::eval::{EvalResult, eval_expr};

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

pub fn parse_and_execute(input: &str) -> Result<bool, ParseError> {
    if input.trim().is_empty() {
        return Ok(false);
    }

    let mut tokens = tokenize(input)?;

    let expr = parse_expr(&mut tokens)?;

    if let Some(EvalResult {
        success: _,
        should_exit,
    }) = eval_expr(expr)
    {
        return Ok(should_exit);
    }

    Ok(false)
}

fn tokenize(input: &str) -> Result<Vec<String>, ParseError> {
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

fn parse_expr(tokens: &mut Vec<String>) -> Result<CommandExpr, ParseError> {
    let mut exprs: Vec<CommandExpr> = Vec::new();
    while !tokens.is_empty() {
        let token = tokens.remove(0);

        for t in tokens.iter() {
            println!("!-> {}", t);
        }

        match token.as_str() {
            ";" => {
                continue;
            }
            "&&" => {
                if exprs.is_empty() {
                    return Err(ParseError::UnexpectedOperator("&&".to_string()));
                }
                let lhs = exprs.pop().unwrap();
                let rhs = parse_expr(tokens)?;
                return Ok(CommandExpr::And(Box::new(lhs), Box::new(rhs)));
            }
            "||" => {
                if exprs.is_empty() {
                    return Err(ParseError::UnexpectedOperator("||".to_string()));
                }
                let lhs = exprs.pop().unwrap();
                let rhs = parse_expr(tokens)?;
                return Ok(CommandExpr::Or(Box::new(lhs), Box::new(rhs)));
            }
            "|" => {
                println!("Pipe detected!");
                if exprs.is_empty() {
                    return Err(ParseError::UnexpectedOperator("|".to_string()));
                }
                let mut pipeline = vec![exprs.pop().unwrap()];
                while !tokens.is_empty() {
                    for t in tokens.iter() {
                        println!("#-> {}", t);
                    }

                    let next = parse_expr(tokens)?;
                    pipeline.push(next);

                    // if another | follows, continue; else break
                    if tokens.first().map(String::as_str) != Some("|") {
                        break;
                    } else {
                        tokens.remove(0); // consume "|"
                    }
                }
                return Ok(CommandExpr::Pipeline(pipeline));
            }
            _ => {
                let mut cmd = vec![token];
                while !tokens.is_empty() && ![";", "&&", "||", "|"].contains(&tokens[0].as_str()) {
                    cmd.push(tokens.remove(0));
                }
                exprs.push(CommandExpr::Command(cmd));
            }
        }
    }

    if exprs.len() == 1 {
        Ok(exprs.remove(0))
    } else {
        Ok(CommandExpr::Sequence(exprs))
    }
}
