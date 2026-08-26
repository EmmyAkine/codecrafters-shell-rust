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

    fn complete(&self, line: &str, _pos: usize, _ctx : &Context<'_>) -> Result<(usize, Vec<Pair>)> {

        /*let start = line.rfind(char::is_whitespace).map(|i| i + 1).unwrap_or(0);
        let current_word = &line[start..];
        let tokens = line.split_whitespace().collect::<Vec<_>>();
        let last_token = &tokens[tokens.len() - 1];
        let command = tokens[0];*/
        if line.trim().is_empty() { return Ok((0, vec![])) }
        let start = line[.._pos].rfind(char::is_whitespace).map_or(0, |i| i + 1);
        let current_word = &line[start.._pos];
        let is_command_completion = line[..start].trim().is_empty();

        let unique_matches:BTreeSet<String>;
        if is_command_completion {
            unique_matches = self.builtin_commands.iter().chain(self.path_executables.iter()).filter(|cmd| cmd.starts_with(current_word)).cloned().map(|word| format!("{} ", word)).collect();
        }
        else {
            unique_matches = self.get_files_and_directories(current_word);
        }



        //Collect and store lcp value early
        let lcp = self.longest_common_prefix(&unique_matches);

        match unique_matches.len() {
            0 => {
                Self::ring_bell();
                Ok((start, vec![]))
            },
            1 => {
                let mut state = self.tab_state.borrow_mut();
                state.count = 0;
                state.last_line.clear();

                let matches:Vec<Pair> = unique_matches.into_iter().map(|cmd| {Pair{display: cmd.to_string(), replacement: format!("{}", cmd)}}).collect();
                Ok((start, matches))
            }
            _ => {
                if lcp.trim_end().trim_end_matches('/') != current_word.trim_end().trim_end_matches('/') && !lcp.is_empty()  {
                    let matches:Vec<Pair> = unique_matches.into_iter().map(|cmd| {Pair{display: cmd.to_string(), replacement: cmd.trim_end().to_owned()}}).collect();
                    return Ok((start, matches));
                }
                let mut state = self.tab_state.borrow_mut();
                if state.last_line == current_word {
                    // Consecutive tab on the same prefix
                    state.count += 1;
                }
                else {
                    // First tab on a new or modified prefix
                    state.last_line = current_word.to_string();
                    state.count = 1;
                }
                if state.count == 1 {
                    Self::ring_bell();
                    Ok((start, Vec::new()))
                }
                else {
                    // Tab 2: Reset count, print sorted list manually, and return empty candidates
                    state.count = 0;

                    let formated_matches: String = unique_matches.into_iter().collect::<Vec<_>>().join("  ");

                    // Print matches on a new line
                    print!("\n{}\n", formated_matches);
                    use std::io::Write;
                    let _ = std::io::stdout().flush();

                    state.count = 0;
                    state.last_line.clear();
                    let refresh_pair = vec![Pair {
                        display: current_word.to_string(),
                        replacement: current_word.to_string(),
                    }];

                    Ok((start, refresh_pair))
                }
            }
        }
    }

    fn update(&self, line: &mut LineBuffer, start: usize, elected: &str, cl: &mut Changeset) {
        line.replace(start..line.pos(), elected, cl);
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

 /*   fn longest_common_prefix(words: &[String]) -> String {
        if words.is_empty() {
            return String::new();
        }

        let first = &words[0];
        let mut len = first.len();

        for s in &words[1..] {
            while !s.starts_with(&first[..len]) {
                len -= 1;
                if len == 0 {
                    return String::new();
                }
            }
        }

        first[..len].to_string()
    }*/

    /*fn longest_common_prefix<'a, StringCollections>(&self, s: StringCollections) -> String
        where StringCollections: IntoIterator<Item=&'a String> {
        TODO!()
    }*/

    fn longest_common_prefix(&self, matches: &BTreeSet<String>) -> String {
        if matches.len() <= 0 {
            return "".to_string();
        }
        let lcp = matches.first().unwrap();
        let mut len = lcp.len();
        for val in matches.iter().skip(1) {
            while !val.trim().starts_with(&lcp.trim()[..len - 1]) {
                #[allow(unused)]
                let x = &lcp.trim()[..len - 1];
                len -= 1;
                if len == 0 {
                    return "".to_string();
                }
            }
        }
        lcp[..len].to_string()
    }

    fn get_files_and_directories(&self, text: &str) -> BTreeSet<String> {
        use std::path::Path;
        use std::fs;
        use std::env;

        let has_trailing_slash = text.ends_with('/') || text.ends_with('\\');

        let (dir_part, parent_str, file_part) = if has_trailing_slash {
            let p = Path::new(text);
            let absolute_dir = if p.is_absolute() {
                p.to_path_buf()
            } else {
                env::current_dir().unwrap().join(p)
            };
            (absolute_dir, text.to_string(), None)
        }
        else {
            // e.g. "app/grape" -> dir is "app", parent_str is "app/", file_part is Some("grape")
            let new_path = Path::new(text);
            let (dir, p_str) = match new_path.parent() {
                Some(p) if !p.as_os_str().is_empty() => {
                    let p_str = p.to_string_lossy();
                    let prefix = if p_str.ends_with('/') || p_str.ends_with('\\') {
                        p_str.into_owned()
                    } else {
                        format!("{}/", p_str)
                    };
                    (env::current_dir().unwrap().join(p), prefix)
                }
                _ => (env::current_dir().unwrap(), String::new()),
            };
            (dir, p_str, new_path.file_name())
        };

        // If directory doesn't exist, bell and return
        if !dir_part.exists() {
            Self::ring_bell();
            return BTreeSet::new();
        }

        let mut match_values: BTreeSet<String> = BTreeSet::new();

        let entries = match fs::read_dir(&dir_part) {
            Ok(e) => e,
            Err(_) => return match_values,
        };
        if file_part.is_none() {
            for entry in entries.flatten() {
                let file_name = entry.file_name().to_string_lossy().into_owned();
                let is_dir = entry.file_type().map_or(false, |ft| ft.is_dir());
                let suffix = if is_dir { "/" } else { " " };

                match_values.insert(format!("{}{}{}", parent_str, file_name, suffix));
            }
            return match_values;
        }


        let target = file_part.unwrap().to_string_lossy();
        for entry in entries.flatten(){
            let file_name = entry.file_name().to_string_lossy().into_owned();
            if file_name.starts_with(target.as_ref()) {
                let is_dir = entry.file_type().map_or(false, |ft| ft.is_dir());
                let suffix = if is_dir { "/" } else { " " };

                match_values.insert(format!("{}{}{}", parent_str, file_name, suffix));
            }
        }
        match_values
    }

}

impl Helper for Completion {}
impl Hinter for Completion {
    type Hint = String;
}

impl Highlighter for Completion {}
impl Validator for Completion {}
