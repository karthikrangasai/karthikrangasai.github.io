use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CommandOutput {
    pub user: String,
    pub path: String,
    pub command: String,
    pub output: String,
}

#[rustfmt::skip]
pub fn help_output() -> String {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    format!(
        "karthikrangasai.github.io v{}

COMMANDS:

	whoami    Display the current user.
	about     A little something about me.
	pwd       Dsiplay the current working directory.
	clear     Clear the contents on the temrinal.
	help      Display the current message.

	",
        VERSION
    )
}

pub fn no_args_for_command_output(command: String, command_args: Vec<String>) -> String {
    format!(
        "`{}` command expects no arguments. Provided argument(s) `{}`.",
        command,
        command_args.join(",")
    )
}
