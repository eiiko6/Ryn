use rustyline::Helper;
use rustyline::completion::{Completer, FilenameCompleter};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;

pub struct RynHelper {
    completer: FilenameCompleter,
}

impl RynHelper {
    pub fn new() -> Self {
        RynHelper {
            completer: FilenameCompleter::new(),
        }
    }
}

impl Completer for RynHelper {
    type Candidate = <FilenameCompleter as Completer>::Candidate;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Helper for RynHelper {}
impl Hinter for RynHelper {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<String> {
        None
    }
}
impl Highlighter for RynHelper {}
impl Validator for RynHelper {}
