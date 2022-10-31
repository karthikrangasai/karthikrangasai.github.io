use core::fmt::Debug;
use serde::{Deserialize, Serialize};

use crate::terminal::filesystem::FileSystem;

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandOutput {
    user: String,
    path: String,
    command: String,
    output: String,
}

impl CommandOutput {
    pub fn new(user: String, path: String, command: String, output: String) -> CommandOutput {
        CommandOutput {
            user,
            path,
            command,
            output,
        }
    }
}

pub fn no_args_for_command_output(command: String, command_args: Vec<String>) -> String {
    format!(
        "`{}` command expects no arguments. Provided argument(s) `{}`.",
        command,
        command_args.join(",")
    )
}

pub trait Command {
    fn execute(&self, args: Vec<String>, file_system: &mut FileSystem) -> String;
    fn get_help_string(&self) -> String;
}

impl Debug for dyn Command {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.get_help_string().as_str())
    }
}
