use crate::terminal::commands::Command;
use crate::terminal::filesystem::FileSystem;

#[derive(Debug)]
pub struct CLEAR {
    name: String,
    help_message: String,
}

impl CLEAR {
    pub fn new(name: String, help_message: String) -> CLEAR {
        CLEAR { name, help_message }
    }
}

impl Command for CLEAR {
    fn execute(&self, _args: Vec<String>, _file_system: &mut FileSystem) -> String {
        "".to_string()
    }

    fn get_help_string(&self) -> String {
        format!("\t{:<20}  {}", self.name, self.help_message)
    }
}
