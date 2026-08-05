use std::io::Write;

pub mod exit;
pub mod echo;
pub mod type_cmd;
pub mod pwd;
pub mod cd_cmd;

pub trait Command {
    fn get_name_copy(&self) -> String;
    fn execute(&self, args: &[&str], stdout: &mut dyn Write, stderr: &mut dyn Write) -> bool;
}