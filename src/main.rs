pub mod shell;
pub mod builtins;
pub mod command_dispatcher;
pub mod external_command;
pub mod path_resolver;

#[allow(unused_imports)]
use std::io::{self, Write};
use shell::Shell;

fn main() {
    let shell = Shell{};
    shell.run();
}


