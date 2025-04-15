use crate::shell::builtin::handle_builtin;
use crate::shell::command::{execute_command, CommandExpr};

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
        CommandExpr::Pipeline(_pipeline) => {
            println!("pipeline executed");
            Option::None
        }
    }
}
