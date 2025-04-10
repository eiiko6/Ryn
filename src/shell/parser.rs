use crate::shell::builtin::handle_builtin;
use crate::shell::commands::execute_command;

#[derive(Debug)]
pub enum CommandExpr {
    Sequence(Vec<CommandExpr>), // a ; b ; c
    // Pipeline(Vec<CommandExpr>),              // a | b | c
    And(Box<CommandExpr>, Box<CommandExpr>), // a && b
    Or(Box<CommandExpr>, Box<CommandExpr>),  // a || b
    // Redirect {
    //     command: Box<CommandExpr>,
    //     kind: RedirectKind,
    //     target: String,
    // },
    // Background(Box<CommandExpr>), // a &
    Command(Vec<String>), // basic command + args
}

// pub enum RedirectKind {
//     Output, // >
//     Append, // >>
//     Input,  // <
// }

struct EvalResult {
    success: bool,
    should_exit: bool,
}

pub fn parse_and_execute(input: &str) -> Option<bool> {
    if input.trim().is_empty() {
        return Some(false);
    }

    let mut tokens = tokenize(input)?;

    let expr = parse_expr(&mut tokens)?;

    if let Some(EvalResult {success: _, should_exit}) = eval_expr(expr) {
        return Some(should_exit);
    }

    Some(false)
}

fn tokenize(input: &str) -> Option<Vec<String>> {
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

    Some(tokens)
}

fn parse_expr(tokens: &mut Vec<String>) -> Option<CommandExpr> {
    let mut exprs: Vec<CommandExpr> = Vec::new();
    while !tokens.is_empty() {
        let token = tokens.remove(0);

        match token.as_str() {
            ";" => {
                continue;
            },
            "&&" => {
                let rhs = parse_expr(tokens)?;
                return Some(CommandExpr::And(Box::new(exprs.pop()?), Box::new(rhs)));
            }
            "||" => {
                let rhs = parse_expr(tokens)?;
                return Some(CommandExpr::Or(Box::new(exprs.pop()?), Box::new(rhs)));
            }
            _ => {
                let mut cmd = vec![token];
                while !tokens.is_empty() && ![";", "&&", "||"].contains(&tokens[0].as_str()) {
                    cmd.push(tokens.remove(0));
                }
                exprs.push(CommandExpr::Command(cmd));
            }
        }
    }

    if exprs.len() == 1 {
        Some(exprs.remove(0))
    } else {
        Some(CommandExpr::Sequence(exprs))
    }
}

fn eval_expr(expr: CommandExpr) -> Option<EvalResult> {
    match expr {
        CommandExpr::Command(args) => {
            // Check if it's a built-in command
            if let Some(builtin) = handle_builtin(&args) {
                return Some(EvalResult {success: true, should_exit: builtin});
            }

            // Execute external command
            let success = execute_command(&args);
            Some(EvalResult {success, should_exit: false})
        }
        CommandExpr::Sequence(exprs) => {
            let mut success = true;
            for expr in exprs {
                if let Some(EvalResult { success: s, should_exit }) = eval_expr(expr) {
                    if !s {
                        success = false; // If any command fails, the whole sequence is considered failed
                    }
                    if should_exit {
                        return Some(EvalResult { success: s, should_exit: true }); // Exit early if any command indicates it
                    }
                }
            }
            Some(EvalResult { success, should_exit: false })
        }
        CommandExpr::And(lhs, rhs) => {
            if let Some(EvalResult { success: false, should_exit: false }) = eval_expr(*lhs) {
                return Some(EvalResult { success: false, should_exit: false });
            }
            eval_expr(*rhs)
        }
        CommandExpr::Or(lhs, rhs) => {
            if let Some(EvalResult { success: true, should_exit: false }) = eval_expr(*lhs) {
                return Some(EvalResult { success: true, should_exit: false });
            }
            eval_expr(*rhs)
        }
    }
}
