use std::cell::RefCell;
use std::collections::{BTreeSet, HashSet};
use rustyline::{Changeset, Context, Helper, Result};
use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::line_buffer::LineBuffer;
use rustyline::validate::Validator;

pub struct Completion {
    builtin_commands: HashSet<String>,
    path_executables: HashSet<String>,

    tab_state: RefCell<TabState>
}

#[derive(Default)]
struct TabState {
    pub last_line: String,
    pub count: usize,
}

impl Completer for Completion{
    type Candidate = Pair;

    fn complete(&self, line: &str, _pos: usize, _ctx    : &Context<'_>) -> Result<(usize, Vec<Pair>)> {
        let start = line.rfind(char::is_whitespace).map(|i| i + 1).unwrap_or(0);
        let current_word = &line[start..];

        let unique_matches:BTreeSet<&String> = self.builtin_commands.iter().chain(self.path_executables.iter()).filter(|cmd| cmd.starts_with(current_word)).collect();
        let matches:Vec<Pair> = unique_matches.into_iter().map(|cmd| {Pair{display: cmd.to_string(), replacement: format!("{} ", cmd)}}).collect();
        
        match matches.len() {
            0 => {
                Self::ring_bell();
                Ok((start, matches))
            },
            1 => Ok((start, matches)),
            _ => {
                let mut state = self.tab_state.borrow_mut();
                if state.last_line == current_word {
                    // Consecutive tab on the same prefix
                    state.count += 1;
                } else {
                    // First tab on a new or modified prefix
                    state.last_line = current_word.to_string();
                    state.count = 1;
                }
                if state.count == 1 {
                    Self::ring_bell();
                    Ok((0, Vec::new()))
                } else {
                    // Tab 2: Reset count, print sorted list manually, and return empty candidates
                    state.count = 0;

                    //Unzip pair into a list and sort
                    let  (mut sorted_matches, _):(Vec<String>, Vec<String>) = matches.into_iter().map(|p| (p.display, p.replacement)).unzip();
                    sorted_matches.sort();

                    // Print matches on a new line
                    print!("\n{}\n", sorted_matches.join("  "));
                    use std::io::Write;
                    let _ = std::io::stdout().flush();

                    state.count = 0;
                    state.last_line.clear();
                    let refresh_pair = vec![Pair {
                        display: current_word.to_string(),
                        replacement: current_word.to_string(),
                    }];

                    Ok((0, refresh_pair))
                }
            }
        }
    }

    fn update(&self, line: &mut LineBuffer, _start: usize, elected: &str, cl: &mut Changeset) {
        line.update(elected, elected.len(), cl)
    }
}

impl Completion {
    pub fn new (builtin_commands: HashSet<String>, path_executables: HashSet<String>) -> Completion {
        Completion {
            builtin_commands,
            path_executables,
            tab_state: RefCell::new(TabState::default())
        }
    }
    fn ring_bell() {
        use std::io::Write;
        print!("\x07");
        let _ = std::io::stdout().flush();
    }

}

impl Helper for Completion {}
impl Hinter for Completion {
    type Hint = String;
}

impl Highlighter for Completion {}
impl Validator for Completion {}
