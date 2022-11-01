use crate::terminal::commands::{Command, CommandMap};
use crate::terminal::filesystem::FileSystem;

#[derive(Debug)]
pub struct HELP {
    name: String,
    help_message: String,
    commands: CommandMap,
}

impl HELP {
    pub fn new(name: String, help_message: String, commands: CommandMap) -> HELP {
        HELP {
            name,
            help_message,
            commands,
        }
    }
}

impl Command for HELP {
    fn execute(&self, _args: Vec<String>, _file_system: &mut FileSystem) -> String {
        const VERSION: &str = env!("CARGO_PKG_VERSION");

        let mut help_output = format!("karthikrangasai.github.io v{}\n", VERSION);
        help_output.push_str("\n");
        help_output.push_str("COMMANDS:\n");
        help_output.push_str("\n");

        let command_map = self.commands.borrow();
        for (_, command_object) in command_map.iter() {
            help_output.push_str(command_object.get_help_string().as_str());
            help_output.push_str("\n");
        }
        help_output.push_str("\n");
        help_output.to_string()
    }

    fn get_help_string(&self) -> String {
        format!("\t{:<20}  {}", self.name, self.help_message)
    }
}
