use crate::util::treepath::TreePath;
use crate::data::pathtree::PathTree;
use crate::config;
use crate::io::input;

use super::commands::*;

pub struct Interpreter {
    builtin_commands: PathTree<Command>,
}
    
impl Interpreter {
    pub fn new() -> Self {
        let mut builtins: PathTree<Command> = PathTree::new();
        Interpreter::set_all_builtins(&mut builtins);
        Self {
            builtin_commands: builtins
        }
    }


    fn set_all_builtins(builtins: &mut PathTree<Command>) {
        builtins.set_by_path(Command::from(ExitCommand), TreePath::create_path("exit"));
        builtins.set_by_path(Command::from(CurrentTimeCommand), TreePath::create_path("what time is it"));
        builtins.set_by_path(Command::from(WhatsYourNameCommand), TreePath::create_path("what is your name"));
    }


    pub fn run_repl(&mut self) {
        println!("Welcome to Violet the command interpreter!");
        println!("Violet's version is {};", config::get_violet_version());
        println!("Created by {}.", config::get_violet_author());

        loop {
            let user_input = input::get_user_input(config::get_violet_prompt());
            let pathified = TreePath::create_path(&user_input);
            match user_input.as_str() {
                "" => continue,
                _ => {
                    match self.builtin_commands.get_by_path(pathified.clone()) {
                        None => {
                            println!("{}: command does not exist.", pathified.join(" "));
                            continue;
                        },
                        Some(cmd) => {
                            cmd.value.execute();
                        }
                    }
                }
            }
        }
    }
}