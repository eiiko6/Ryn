use rustyline::completion::{Completer, FilenameCompleter};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::history::SearchDirection;
use rustyline::validate::Validator;
use rustyline::{Context, Helper};
use std::borrow::Cow;

pub struct CommandHelper {
    completer: FilenameCompleter,
    hinter: CommandHinter,
}

impl CommandHelper {
    pub fn new() -> Self {
        CommandHelper {
            completer: FilenameCompleter::new(),
            hinter: CommandHinter::new(),
        }
    }
}

impl Completer for CommandHelper {
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

#[derive(Default)]
struct CommandHinter {}

impl CommandHinter {
    fn new() -> Self {
        Self::default()
    }

    fn hint(&self, line: &str, pos: usize, ctx: &Context) -> Option<String> {
        if line.trim().is_empty() {
            return None;
        }

        let history = ctx.history();

        if history.is_empty() {
            return None;
        }

        let mut index = history.len();
        while index > 0 {
            index -= 1;
            if let Ok(Some(result)) = history.get(index, SearchDirection::Reverse) {
                let entry = result.entry;
                if entry.starts_with(line) && entry.len() > pos {
                    return Some(entry[pos..].to_string());
                }
            }
        }

        None
    }
}

impl Hinter for CommandHelper {
    type Hint = String;
    fn hint(&self, line: &str, pos: usize, ctx: &Context) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for CommandHelper {
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        use Cow;
        Cow::Owned(format!("\x1b[90m{}\x1b[0m", hint))
    }
}

impl Validator for CommandHelper {}
impl Helper for CommandHelper {}
