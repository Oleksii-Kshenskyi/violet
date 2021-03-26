use crate::argcount_err;
use crate::config;
use crate::data::pathtree::*;
use crate::io::input;
use crate::util::string::clone_uppercased;
use crate::util::treepath::TreePath;

use std::path::Path;
use std::process::exit;

use super::commands::*;

pub struct Interpreter {
    builtin_commands: PathTree<Command>,
    aliases_for_builtins: PathTree<String>,
    aliases_len_on_boot: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut builtins: PathTree<Command> = PathTree::new();
        let config_name = config::get_config_file_name();
        let aliases: PathTree<String> = if Path::new(&config_name).is_file() {
            match std::fs::read_to_string(&config_name) {
                Ok(file_contents) => match serde_json::from_str(&file_contents) {
                    Ok(the_aliases) => {
                        println!("INFO: loaded the saved aliases from config file successfully!");
                        the_aliases
                    }
                    Err(the_err) => {
                        println!("ERROR: the config file is corrupted, couldn't get aliases from it: {:?}", the_err);
                        PathTree::new()
                    }
                },
                Err(the_err) => {
                    println!(
                        "ERROR: couldn't open config file to load the saved aliases from it: {:?}",
                        the_err
                    );
                    PathTree::new()
                }
            }
        } else {
            PathTree::new()
        };
        let alias_len = aliases.tree.len();

        Interpreter::set_all_builtins(&mut builtins);
        Self {
            builtin_commands: builtins,
            aliases_for_builtins: aliases,
            aliases_len_on_boot: alias_len,
        }
    }

    fn set_all_builtins(builtins: &mut PathTree<Command>) {
        builtins.set_by_path_with_shortcut(Command::from(ExitCommand), "exit");
        builtins.set_by_path_with_shortcut(Command::from(CurrentTimeCommand), "what time is it");
        builtins
            .set_by_path_with_shortcut(Command::from(WhatsYourNameCommand), "what is your name");
        builtins.set_by_path_with_shortcut(
            Command::from(SayThisAndThatCommand),
            "please say <ARG> and <ARG>",
        );
        builtins.set_by_path_with_shortcut(
            Command::from(AddAliasCommand),
            "add alias <ARG> for builtin <ARG>",
        );
        builtins.set_by_path_with_shortcut(Command::from(RemoveAliasCommand), "remove alias <ARG>");
        builtins.set_by_path_with_shortcut(Command::from(HelpCommand), "help");
        builtins.set_by_path_with_shortcut(
            Command::from(ListAvailableCommandsCommand),
            "list available commands",
        );
        builtins.set_by_path_with_shortcut(
            Command::from(ExplainCommandCommand),
            "explain command <ARG>",
        );
    }

    fn exit(&mut self, exit_message: String) {
        if !self.aliases_for_builtins.tree.is_empty() {
            match std::fs::File::create(config::get_config_file_name()) {
                Ok(file) => match serde_json::to_writer_pretty(file, &self.aliases_for_builtins) {
                    Ok(()) => println!("INFO: saved aliases successfully before exiting ^_^"),
                    Err(the_err) => println!(
                        "ERROR: opened file, but weren't able to save aliases to it: {:?}",
                        the_err
                    ),
                },
                Err(the_err) => println!(
                    "ERROR: coudln't create a file to save aliases: {:?}",
                    the_err
                ),
            }
        }

        let config_name = config::get_config_file_name();
        if self.aliases_for_builtins.tree.is_empty()
            && self.aliases_len_on_boot != 0
            && std::path::Path::new(&config_name).is_file()
        {
            println!("INFO: all aliases have been removed, removing the config file...");
            std::fs::remove_file(&config_name).unwrap();
        }

        println!("{}", exit_message);
        exit(0);
    }

    fn list_available_commands(&mut self) {
        if !self.builtin_commands.tree.is_empty() {
            println!("Available commands:\n");
            for key in self.builtin_commands.tree.keys() {
                if self.builtin_commands.does_node_contain_value(key)
                    && !TreePath::is_path_a_shortcut(&key)
                {
                    println!("- {};", key);
                }
            }
            println!("\nTo explain an individual command, please run:\n<<VIO>> explain command <ARG>\n, where <ARG> is the command you want explained.\nIf the command consists of several words/nodes, take care to enclose it in quotation marks \" when passing it as an argument to explain command.");
        } else {
            println!("No commands available!");
        }
    }

    fn explain_command(&mut self, command: &str) {
        if !self.builtin_commands.does_node_contain_value(command) {
            println!(
                "ERROR: can't explain command \"{}\" which doesn't exist.",
                command
            );
            return;
        }

        println!(
            "{}",
            self.builtin_commands
                .get_by_path(command)
                .unwrap()
                .to_owned()
                .value
                .unwrap()
                .help()
        );
    }

    fn add_alias(&mut self, alias: String, for_builtin: String) {
        if !self.builtin_commands.does_node_contain_value(&for_builtin) {
            println!(
                "ERROR: Can't set alias, builtin command [{}] does not exist!",
                for_builtin
            );
            return;
        }

        if self.builtin_commands.does_node_contain_value(&alias) {
            println!("ERROR: can't set this alias: [{}] is an existing builtin command name. Choose a different name for the alias.", alias);
            return;
        }

        if self.aliases_for_builtins.does_node_contain_value(&alias) {
            println!("ERROR: Can't set this alias, alias [{}] already exists. Remove the existing one first!", alias);
            return;
        }

        if TreePath::count_x_nodes_for_path(&alias, "<ARG>")
            != TreePath::count_x_nodes_for_path(&for_builtin, "<ARG>")
        {
            println!(
                "ERROR: alias and the builtin command have to have an equal number of arguments!"
            );
            return;
        }

        self.aliases_for_builtins
            .set_by_path(for_builtin, alias.as_str());
    }

    fn remove_alias(&mut self, alias: String) {
        if self.builtin_commands.does_node_contain_value(&alias) {
            println!(
                "ERROR: you can't remove a builtin command. Choose an alias to remove instead."
            );
            return;
        }

        if !self.aliases_for_builtins.does_node_contain_value(&alias) {
            println!(
                "ERROR: alias {} does not exist. Can't remove alias which doesn't exist.",
                &alias
            );
            return;
        }

        match self.aliases_for_builtins.drop_by_path(&alias) {
            Ok(PathTreeOk::DropOk) => (),
            Err(PathTreeErr::DropNodeDoesNotExist) => {
                println!("ERROR: PathTree: node [{}] does not exist!", &alias);
            }
            Err(PathTreeErr::DropNodeIsNull) => {
                println!("ERROR: this node is a null node. Null nodes can't be explicitly deleted by a user.");
            }
        };
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
        println!(
            "To get help with the basics of Violet, type: \nhelp\n\tor\n[h]\n and press <ENTER>."
        );

        loop {
            let user_input = input::get_user_input(config::get_violet_prompt());
            if user_input.is_empty() {
                continue;
            }

            let command_to_invoke: String = match self
                .aliases_for_builtins
                .get_command_and_args_from_path(&user_input)
            {
                None => user_input,
                Some((path, args)) => {
                    if self.aliases_for_builtins.does_node_contain_value(&path) {
                        TreePath::reconstruct_argumented_path(
                            &self
                                .aliases_for_builtins
                                .get_by_path(&path)
                                .unwrap()
                                .to_owned()
                                .value
                                .unwrap(),
                            args,
                        )
                        .unwrap_or_else(|| {
                            String::from("ERROR: keyword <ARG> used in the wrong context, or")
                        })
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
                    if self.builtin_commands.does_node_contain_value(&path) {
                        let node = self.builtin_commands.get_by_path(&path).unwrap();
                        match node.clone().value.unwrap().execute(args) {
                            Ok(InterpretedCommand::DoNothing) => (),
                            Ok(InterpretedCommand::ListAvailableCommands) => self.list_available_commands(),
                            Ok(InterpretedCommand::Exit { exit_message}) => self.exit(exit_message),
                            Ok(InterpretedCommand::AddAlias {alias, for_builtin}) => self.add_alias(alias, for_builtin),
                            Ok(InterpretedCommand::RemoveAlias {alias}) => self.remove_alias(alias),
                            Ok(InterpretedCommand::ExplainCommand {command}) => self.explain_command(&command),

                            Err(InterpretationError::WrongArgumentCount { expected, actual}) => println!(argcount_err!(), expected, actual),
                            Err(InterpretationError::ArgumentEmpty {argument_name}) => println!("ERROR: Argument named [{}] is empty, which is not allowed in this context!", argument_name),
                        }
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
