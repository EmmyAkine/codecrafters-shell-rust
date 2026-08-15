pub mod shell;
pub mod builtins;
pub mod command_dispatcher;
pub mod external_command;
pub mod path_resolver;
pub mod token;
pub mod tokenizer;
pub mod redirect;
pub mod auto_completion;

use shell::Shell;

fn main() {
    let shell = Shell{};
    shell.run();
}


