pub mod shell;
pub mod builtins;
pub mod command_dispatcher;
pub mod external_command;
pub mod path_resolver;
pub mod token;
pub mod tokenizer;
pub mod redirect;

#[allow(unused_imports)]
use std::io::{self, Write};
use shell::Shell;

fn main() {
    let shell = Shell{};
    shell.run();
}


