use crate::config;
use crate::data::pathtree::PathTree;
use crate::io::input;
use crate::util::string::clone_uppercased;
use crate::util::treepath::TreePath;

use super::commands::*;

pub struct Interpreter {
    builtin_commands: PathTree<Command>,
    aliases_for_builtins: PathTree<String>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut builtins: PathTree<Command> = PathTree::new();
        let mut aliases: PathTree<String> = PathTree::new();
        Interpreter::set_all_builtins(&mut builtins);
        Interpreter::set_all_aliases(&mut aliases);
        Self {
            builtin_commands: builtins,
            aliases_for_builtins: aliases,
        }
    }

    fn set_all_builtins(builtins: &mut PathTree<Command>) {
        builtins.set_by_path(Command::from(ExitCommand), "exit");
        builtins.set_by_path(Command::from(CurrentTimeCommand), "what time is it");
        builtins.set_by_path(Command::from(WhatsYourNameCommand), "what is your name");
        builtins.set_by_path(
            Command::from(SayThisAndThatCommand),
            "please say <ARG> and <ARG>",
        );
    }

    fn set_all_aliases(aliases: &mut PathTree<String>) {
        aliases.set_by_path(
            String::from("please say <ARG> and <ARG>"),
            "utter <ARG> and <ARG> ye heretic",
        );
        aliases.set_by_path(String::from("exit"), "get the heck outta here");
        aliases.set_by_path(String::from("what time is it"), "are we there yet");
        aliases.set_by_path(String::from("what is your name"), "what even are you");
        aliases.set_by_path(
            String::from("please say <ARG> and <ARG>"),
            "blabber <ARG> and <ARG>",
        );
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
            let command_to_invoke: String = match self
                .aliases_for_builtins
                .get_command_and_args_from_path(&user_input)
            {
                None => user_input,
                Some((path, args)) => {
                    if self.aliases_for_builtins.does_node_exist(&path) {
                        TreePath::reconstruct_argumented_path(
                            self.aliases_for_builtins
                                .get_by_path(&path)
                                .unwrap()
                                .value
                                .clone(),
                            args,
                        )
                        .unwrap_or_else(|| String::from(
                            "ERROR: alias and builtin argument counts are different!",
                        ))
                    } else {
                        user_input
                    }
                }
            };

            match self
                .builtin_commands
                .get_command_and_args_from_path(&command_to_invoke)
            {
                None => {
                    println!(
                        "{}: command does not exist.",
                        TreePath::prettify(command_to_invoke.as_str())
                    );
                }
                Some((path, args)) => {
                    if self.builtin_commands.does_node_exist(&path) {
                        let cmd = self.builtin_commands.get_by_path(&path).unwrap();
                        cmd.value.execute(args);
                    } else {
                        println!(
                            "{}: command does not exist.",
                            TreePath::prettify(command_to_invoke.as_str())
                        );
                    }
                }
            }
        }
    }
}
