use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::rc::Rc;
use rustyline::{Changeset, Context, Helper, Result};
use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::line_buffer::LineBuffer;
use rustyline::validate::Validator;

pub struct Completion {
    builtin_commands: HashSet<String>,
    path_executables: HashSet<String>,
    #[allow(unused)]
    complete_dict: Rc<RefCell<HashMap<String, String>>>,

    tab_state: RefCell<TabState>
}

#[derive(Default)]
struct TabState {
    pub last_line: String,
    pub count: usize,
}

impl Completer for Completion{
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, _ctx : &Context<'_>) -> Result<(usize, Vec<Pair>)> {
        if line.trim().is_empty() { return Ok((0, vec![])) }
        let start = line[..pos].rfind(char::is_whitespace).map_or(0, |i| i + 1);
        let current_word = &line[start..pos];
        let is_command_completion = line[..start].trim().is_empty();

        let previous_tokens: Vec<&str> = line[..start].split_whitespace().collect();

        let command_name = previous_tokens.first().copied().unwrap_or(current_word);

        if let Some(script_path) = self.complete_dict.borrow().get(command_name) {
            let arg1 = command_name.to_string();
            let arg2 = current_word.to_string();
            let arg3 = previous_tokens.last().copied().unwrap_or("").to_string();

            let script_matches = self.run_completer_script(line, script_path, pos, arg1, arg2, arg3);

            if !script_matches.is_empty() {
                return self.apply_completion_logic(script_matches, start, current_word);
            }
        }

        let unique_matches:BTreeSet<String>;
        if is_command_completion {
            unique_matches = self.builtin_commands.iter().chain(self.path_executables.iter()).filter(|cmd| cmd.starts_with(current_word)).cloned().map(|word| format!("{} ", word)).collect();
            self.apply_completion_logic(unique_matches, start, current_word)
        }
        else {
            unique_matches = self.get_files_and_directories(current_word);
            self.apply_completion_logic(unique_matches, start, current_word)
        }
    }

    fn update(&self, line: &mut LineBuffer, start: usize, elected: &str, cl: &mut Changeset) {
        line.replace(start..line.pos(), elected, cl);
    }
}

impl Completion {
    pub fn new (builtin_commands: HashSet<String>, path_executables: HashSet<String>, complete_dict: &Rc<RefCell<HashMap<String, String>>>) -> Completion {
        Completion {
            builtin_commands,
            path_executables,
            complete_dict: Rc::clone(complete_dict),
            tab_state: RefCell::new(TabState::default())
        }
    }
    fn ring_bell() {
        use std::io::Write;
        print!("\x07");
        let _ = std::io::stdout().flush();
    }
    fn longest_common_prefix(&self, matches: &BTreeSet<String>) -> String {
        if matches.len() <= 0 {
            return "".to_string();
        }
        let lcp = matches.first().unwrap();
        let mut len = lcp.len();
        for val in matches.iter().skip(1) {
            while !val.trim().starts_with(&lcp.trim()[..len - 1]) {
                let _x = &lcp.trim()[..len - 1];
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

    #[allow(unused)]
    fn run_completer_script(&self, full_text: &str, path: &str, cursor_point: usize, args1: String, args2: String, args3: String)
    -> BTreeSet<String> {
       use std::process::{Command, Stdio};

        let process_output = match Command::new(path)
            .args(vec![args1, args2, args3])
            .env("COMP_LINE", full_text)
            .env("COMP_POINT", cursor_point.to_string())
            .output()
        {
            Ok(output) => output,
            Err(e) => {
                eprintln!("failed to execute {}: {}", path, e);
                return BTreeSet::new()
            }
        };

        let stdout_str = String::from_utf8_lossy(&process_output.stdout);

        let mut sorted_values = BTreeSet::new();

        for line in stdout_str.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                sorted_values.insert(format!("{} ", trimmed));
            }
        }
        sorted_values
    }

    fn apply_completion_logic(&self, unique_matches: BTreeSet<String>, start: usize, current_word: &str) -> Result<(usize, Vec<Pair>)>{
        //Collect and store lcp value early
        //let unique_matches = BTreeSet::from_iter(vec!["xyz_foo".to_owned(), "xyz_foo_bar".to_owned(), "xyz_foo_bar_baz".to_owned()]);
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
                    //let matches:Vec<Pair> = unique_matches.into_iter().map(|cmd| {Pair{display: cmd.to_string(), replacement: cmd.trim_end().to_owned()}}).collect();
                    let matches = vec![Pair {display: lcp.trim_end().to_string(), replacement: lcp.to_string()}];
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

}

impl Helper for Completion {}
impl Hinter for Completion {
    type Hint = String;
}

impl Highlighter for Completion {}
impl Validator for Completion {}
