#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use crate::builtins::Command;
use crate::external_command::ExternalCommand;
use crate::tokenizer::Tokenizer;
use crate::redirect::RedirectSpec;
use crate::token::{Token, TokenKind};

pub struct CommandDispatcher{
    commands: HashMap<String, Box<dyn Command>>,
    external_command: ExternalCommand
}

impl CommandDispatcher {
    pub fn new(commands: HashMap<String, Box<dyn Command>>, external_command: ExternalCommand) -> CommandDispatcher {
        let dispatcher = CommandDispatcher{commands, external_command };
        dispatcher
    }
    pub fn dispatch(&self, input: &str) -> bool {
        let tokens = match Tokenizer::tokenize(input) {
            Ok(t) => t,
            Err(e) => {
                println!("{e}");
                return true;
            }
        };

        if tokens.is_empty() {
            return true;
        }

        let redirect = Self::parse_redirects(&tokens);
        let words = Self::extract_words(&tokens);

        if words.is_empty() {
            return true;
        }

        let command = &words[0];
        let args: Vec<&str> = words[1..].iter().map(String::as_str).collect();

        match self.commands.get(command.as_str()) {
            Some(builtin) => self.run_builtin(builtin.as_ref(), &args, &redirect),
            None => {
                if self.external_command.try_run(command, &args, &redirect) {
                    true
                } else {
                    println!("{command}: command not found");
                    true
                }
            }
        }
    }

    // ── run a builtin, wiring its stdout/stderr to files if redirected ────────
    fn run_builtin(&self, builtin: &dyn Command, args: &[&str], redirect: &RedirectSpec) -> bool {
        let mut stdout_file = Self::open_redirect(&redirect.stdout_file, redirect.stdout_append);
        let mut stderr_file = Self::open_redirect(&redirect.stderr_file, redirect.stderr_append);

        let mut real_stdout = io::stdout();
        let mut real_stderr = io::stderr();

        let out: &mut dyn Write = stdout_file.as_mut().map_or(&mut real_stdout as &mut dyn Write, |f| f);
        let err: &mut dyn Write = stderr_file.as_mut().map_or(&mut real_stderr as &mut dyn Write, |f| f);

        builtin.execute(args, out, err)
    }

    // ── word extraction: skips redirect operators AND their filename tokens ───
    fn extract_words(tokens: &[Token]) -> Vec<String> {
        let mut words = Vec::new();
        let mut i = 0;
        while i < tokens.len() {
            let tok = &tokens[i];
            if tok.kind == TokenKind::Word {
                words.push(tok.value.clone());
            } else {
                i += 1; // skip the filename token that follows the operator
            }
            i += 1;
        }
        words
    }

    fn parse_redirects(tokens: &[Token]) -> RedirectSpec {
        let mut stdout_file = None;
        let mut stdout_append = false;
        let mut stderr_file = None;
        let mut stderr_append = false;

        let mut i = 0;
        while i < tokens.len() {
            let tok = &tokens[i];
            let target = if i + 1 < tokens.len() && tokens[i + 1].kind == TokenKind::Word {
                let val = tokens[i + 1].value.clone();
                i += 1;
                Some(val)
            } else {
                None
            };

            if let Some(target) = target {
                match tok.kind {
                    TokenKind::RedirectOut => { stdout_file = Some(target); stdout_append = false; }
                    TokenKind::RedirectAppend => { stdout_file = Some(target); stdout_append = true; }
                    TokenKind::RedirectErr => { stderr_file = Some(target); stderr_append = false; }
                    TokenKind::RedirectErrAppend => { stderr_file = Some(target); stderr_append = true; }
                    _ => {}
                }
            }
            i += 1;
        }

        RedirectSpec { stdout_file, stdout_append, stderr_file, stderr_append }
    }

    fn open_redirect(path: &Option<String>, append: bool) -> Option<File> {
        path.as_ref().map(|p| {
            OpenOptions::new()
                .write(true)
                .create(true)
                .append(append)
                .truncate(!append)
                .open(p)
                .expect("failed to open redirect target")
        })
    }
}