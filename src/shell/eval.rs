use crate::shell::builtin::handle_builtin;
use crate::shell::command::{CommandExpr, execute_command, spawn_command};
use os_pipe::{PipeReader, pipe};
use std::process::Stdio;

pub struct EvalResult {
    pub success: bool,
    pub should_exit: bool,
}

pub fn eval_expr(expr: CommandExpr) -> Option<EvalResult> {
    match expr {
        CommandExpr::Command(args) => {
            // Check if it's a built-in command
            if let Some(builtin) = handle_builtin(&args) {
                return Some(EvalResult {
                    success: true,
                    should_exit: builtin,
                });
            }

            // Execute external command
            let success = execute_command(&args);
            Some(EvalResult {
                success,
                should_exit: false,
            })
        }
        CommandExpr::Sequence(exprs) => {
            let mut success = true;
            for expr in exprs {
                if let Some(EvalResult {
                    success: s,
                    should_exit,
                }) = eval_expr(expr)
                {
                    if !s {
                        success = false; // If any command fails, the whole sequence is considered failed
                    }
                    if should_exit {
                        return Some(EvalResult {
                            success: s,
                            should_exit: true,
                        }); // Exit early if any command indicates it
                    }
                }
            }
            Some(EvalResult {
                success,
                should_exit: false,
            })
        }
        CommandExpr::And(lhs, rhs) => {
            if let Some(EvalResult {
                success: false,
                should_exit: false,
            }) = eval_expr(*lhs)
            {
                return Some(EvalResult {
                    success: false,
                    should_exit: false,
                });
            }
            eval_expr(*rhs)
        }
        CommandExpr::Or(lhs, rhs) => {
            if let Some(EvalResult {
                success: true,
                should_exit: false,
            }) = eval_expr(*lhs)
            {
                return Some(EvalResult {
                    success: true,
                    should_exit: false,
                });
            }
            eval_expr(*rhs)
        }
        CommandExpr::Pipeline(cmds) => {
            let mut processes = Vec::new();
            let mut prev_reader: Option<PipeReader> = None;

            for (i, expr) in cmds.iter().enumerate() {
                let args = if let CommandExpr::Command(args) = expr {
                    args
                } else {
                    return Some(EvalResult {
                        success: false,
                        should_exit: false,
                    });
                };

                let stdin = if let Some(reader) = prev_reader.take() {
                    Stdio::from(reader)
                } else {
                    Stdio::inherit()
                };

                let stdout = if i < cmds.len() - 1 {
                    let (reader, writer) = pipe().unwrap();
                    prev_reader = Some(reader);
                    Stdio::from(writer)
                } else {
                    Stdio::inherit()
                };

                match spawn_command(args, stdin, stdout, None) {
                    Ok(child) => processes.push(child),
                    Err(err) => {
                        eprintln!("Failed to spawn command '{}': {}", args[0], err);
                        return Some(EvalResult {
                            success: false,
                            should_exit: false,
                        });
                    }
                }
            }

            let mut success = true;
            for mut child in processes {
                if !child.wait().map(|s| s.success()).unwrap_or(false) {
                    success = false;
                }
            }

            Some(EvalResult {
                success,
                should_exit: false,
            })
        }
    }
}
