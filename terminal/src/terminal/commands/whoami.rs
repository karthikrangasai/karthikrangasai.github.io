use crate::terminal::{commands::Command, filesystem::FileSystem};

#[derive(Debug)]
pub struct WHOAMI {
    name: String,
    help_message: String,
}

impl WHOAMI {
    pub fn new(name: String, help_message: String) -> WHOAMI {
        WHOAMI { name, help_message }
    }
}

impl Command for WHOAMI {
    fn execute(&self, _args: Vec<String>, file_system: &mut FileSystem) -> String {
        file_system.user()
    }

    fn get_help_string(&self) -> String {
        format!("\t{:<20}  {}", self.name, self.help_message)
    }
}
