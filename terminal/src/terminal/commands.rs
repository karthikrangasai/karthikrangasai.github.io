mod about;
mod clear;
pub mod common;
mod help;
mod pwd;
mod whoami;
mod wrong_command;
use crate::terminal::filesystem::FileSystem;
use crate::terminal::utils::log;
use common::Command;
use std::cell::{Ref, RefCell};
use std::collections::BTreeMap;
use std::rc::Rc;
use wrong_command::WRONGCMD;

type _CommandMap = BTreeMap<String, Box<dyn Command>>;

pub type CommandMap = Rc<RefCell<_CommandMap>>;

#[derive(Debug)]
pub struct CommandRunner {
    commands: CommandMap,
}

impl CommandRunner {
    pub fn new() -> CommandRunner {
        let rc_commands: CommandMap = Rc::new(RefCell::new(BTreeMap::new()));
        let mut command_runner = CommandRunner {
            commands: Rc::clone(&rc_commands),
        };

        // ABOUT command
        command_runner.register_command(
            "about".to_string(),
            Box::new(about::ABOUT::new(
                "about".to_string(),
                "A little something about me.".to_string(),
            )),
        );

        // CLEAR command
        command_runner.register_command(
            "clear".to_string(),
            Box::new(clear::CLEAR::new(
                "clear".to_string(),
                "Clear the contents on the terminal.".to_string(),
            )),
        );

        // PWD command
        command_runner.register_command(
            "pwd".to_string(),
            Box::new(pwd::PWD::new(
                "pwd".to_string(),
                "A little something about me.".to_string(),
            )),
        );

        // WHOAMI command
        command_runner.register_command(
            "whoami".to_string(),
            Box::new(whoami::WHOAMI::new(
                "whoami".to_string(),
                "Display the current user.".to_string(),
            )),
        );

        // HELP command
        command_runner.register_command(
            "help".to_string(),
            Box::new(help::HELP::new(
                "help".to_string(),
                "Lists all the available commands.".to_string(),
                Rc::clone(&rc_commands),
            )),
        );

        command_runner
    }

    pub fn register_command(&mut self, command_verb: String, command_object: Box<dyn Command>) {
        self.commands.borrow_mut().insert(command_verb.clone(), command_object);
    }

    pub fn execute_command(&mut self, full_command: String, file_system: &mut FileSystem) -> String {
        let mut tokens: Vec<String> = full_command.split(" ").map(str::to_string).collect();
        let args: Vec<String> = tokens.split_off(1);
        let command: &String = &tokens[0];

        let wrong_command: Box<dyn Command> = Box::new(WRONGCMD::new(command.to_string())) as Box<dyn Command>;

        let command_map: Ref<_CommandMap> = self.commands.borrow();
        let command_object: &Box<dyn Command> = match command_map.get(command) {
            Some(command_object) => command_object,
            None => &wrong_command,
        };

        log!("Command is {}, args is {:?}", command, args);
        command_object.execute(args, file_system)
    }
}
