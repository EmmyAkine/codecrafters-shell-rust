use rustyline::{Changeset, Context, Helper, Result};
use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::line_buffer::LineBuffer;
use rustyline::validate::Validator;

pub struct Completion {
    pub builtin_commands: Vec<String>
}

impl Completer for Completion{
    type Candidate = Pair;

    fn complete(&self, line: &str, _pos: usize, _ctx    : &Context<'_>) -> Result<(usize, Vec<Pair>)> {
        let start = line.rfind(char::is_whitespace).map(|i| i + 1).unwrap_or(0);
        let current_word = &line[start..];
        let matches = self.builtin_commands.iter().filter(|cmd| cmd.starts_with(current_word)).map(|cmd| {Pair{display: cmd.clone(), replacement: cmd.clone()}}).collect();
        Ok((start, matches))
    }

    fn update(&self, line: &mut LineBuffer, _start: usize, elected: &str, cl: &mut Changeset) {
        line.update(elected, elected.len(), cl)
    }
}

impl Helper for Completion {}
impl Hinter for Completion {
    type Hint = String;
}

impl Highlighter for Completion {}
impl Validator for Completion {}
