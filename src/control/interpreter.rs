use crate::config;
use crate::data::pathtree::PathTree;
use crate::io::input;
use crate::util::string::clone_uppercased;
use crate::util::treepath::TreePath;

use super::commands::*;

pub struct Interpreter {
    builtin_commands: PathTree<Command>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut builtins: PathTree<Command> = PathTree::new();
        Interpreter::set_all_builtins(&mut builtins);
        Self {
            builtin_commands: builtins,
        }
    }

    fn set_all_builtins(builtins: &mut PathTree<Command>) {
        builtins.set_by_path(Command::from(ExitCommand::new()), "exit");
        builtins.set_by_path(Command::from(CurrentTimeCommand::new()), "what time is it");
        builtins.set_by_path(Command::from(WhatsYourNameCommand::new()), "what is your name");
    }

    pub fn run_repl(&mut self) {
        println!(
            "Welcome to {} the command interpreter!",
            clone_uppercased(&config::get_violet_name())
        );
        println!(
            "{}'s version is {};",
            clone_uppercased(&config::get_violet_name()),
            config::get_violet_version()
        );
        println!("Created by {}.", config::get_violet_author());

        loop {
            let user_input = input::get_user_input(config::get_violet_prompt());
            match user_input.as_str() {
                "" => continue,
                _ => match self.builtin_commands.get_by_path(user_input.as_str()) {
                    None => {
                        println!(
                            "{}: command does not exist.",
                            TreePath::prettify(user_input.as_str())
                        );
                        continue;
                    }
                    Some(cmd) => {
                        cmd.value.execute();
                    }
                },
            }
        }
    }
}
