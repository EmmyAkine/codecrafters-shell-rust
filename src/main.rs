pub mod test;

#[allow(unused_imports)]
use std::io::{self, Write};
use crate::test::printer;

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        if command.trim() == "exit" {
            break;
        }
        printer(&command);
    }
}


