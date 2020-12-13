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
        builtins.set_by_path(Command::from(ExitCommand), "exit");
        builtins.set_by_path(Command::from(CurrentTimeCommand), "what time is it");
        builtins.set_by_path(Command::from(WhatsYourNameCommand), "what is your name");
        builtins.set_by_path(Command::from(SayThisAndThatCommand), "please say <ARG> and <ARG>");
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
            match self.builtin_commands.get_command_and_args_from_path(&user_input) {
                None => {
                    println!(
                        "{}: command does not exist.",
                        TreePath::prettify(user_input.as_str())
                    );
                },
                Some((path, args)) => {
                    let cmd = self.builtin_commands.get_by_path(&path).unwrap();
                    cmd.value.execute(args);
                }
            }
        }
    }
}
