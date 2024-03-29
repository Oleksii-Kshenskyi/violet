extern crate chrono;
extern crate enum_dispatch;
use chrono::Local;
use enum_dispatch::*;

use crate::config::get_exit_message;
use crate::config::get_help_message;
use crate::config::get_violet_name;
use crate::config::Help;

use serde::{Deserialize, Serialize};

pub enum InterpretedCommand {
    DoNothing,
    ListAvailableCommands,
    Exit { exit_message: String },
    AddAlias { alias: String, for_builtin: String },
    RemoveAlias { alias: String },
    ExplainCommand { command: String },
}

pub enum InterpretationError {
    ArgumentEmpty { argument_name: String },
    ArgSpecifierMisused,
}

#[enum_dispatch]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Command {
    ExitCommand,
    CurrentTimeCommand,
    WhatsYourNameCommand,
    SayThisAndThatCommand,
    AddAliasCommand,
    RemoveAliasCommand,
    HelpCommand,
    ListAvailableCommandsCommand,
    ExplainCommandCommand,
}

#[enum_dispatch(Command)]
pub trait Action {
    fn execute(&self, args: Vec<String>) -> Result<InterpretedCommand, InterpretationError>;
    fn help(&self) -> &str;
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ExitCommand;
impl Action for ExitCommand {
    fn execute(&self, _args: Vec<String>) -> Result<InterpretedCommand, InterpretationError> {
        Ok(InterpretedCommand::Exit {
            exit_message: get_exit_message(),
        })
    }

    fn help(&self) -> &str {
        Help::exit()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CurrentTimeCommand;
impl Action for CurrentTimeCommand {
    fn execute(&self, _args: Vec<String>) -> Result<InterpretedCommand, InterpretationError> {
        println!(
            "Your system clock says it's {} now!",
            Local::now().format("%I:%M %p")
        );

        Ok(InterpretedCommand::DoNothing)
    }

    fn help(&self) -> &str {
        Help::what_time_is_it()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WhatsYourNameCommand;
impl Action for WhatsYourNameCommand {
    fn execute(&self, _args: Vec<String>) -> Result<InterpretedCommand, InterpretationError> {
        println!("My name is {}! Nice to meet you ^_^", &get_violet_name());

        Ok(InterpretedCommand::DoNothing)
    }

    fn help(&self) -> &str {
        Help::what_is_your_name()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SayThisAndThatCommand;
impl Action for SayThisAndThatCommand {
    fn execute(&self, args: Vec<String>) -> Result<InterpretedCommand, InterpretationError> {
        if args.iter().any(|arg| arg == "<ARG>") {
            return Err(InterpretationError::ArgSpecifierMisused);
        }

        println!(
            "Gotcha. Saying {} and {}!",
            args.get(0).unwrap(),
            args.get(1).unwrap()
        );

        Ok(InterpretedCommand::DoNothing)
    }

    fn help(&self) -> &str {
        Help::please_say_arg_and_arg()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AddAliasCommand;
impl Action for AddAliasCommand {
    fn execute(&self, args: Vec<String>) -> Result<InterpretedCommand, InterpretationError> {
        if args.iter().any(|arg| arg == "<ARG>") {
            return Err(InterpretationError::ArgSpecifierMisused);
        }

        if args[0].is_empty() {
            return Err(InterpretationError::ArgumentEmpty {
                argument_name: "alias to add".to_string(),
            });
        }
        if args[1].is_empty() {
            return Err(InterpretationError::ArgumentEmpty {
                argument_name: "builtin name".to_string(),
            });
        }

        Ok(InterpretedCommand::AddAlias {
            alias: args[0].clone(),
            for_builtin: args[1].clone(),
        })
    }

    fn help(&self) -> &str {
        Help::add_alias_arg_for_builtin_arg()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RemoveAliasCommand;
impl Action for RemoveAliasCommand {
    fn execute(&self, args: Vec<String>) -> Result<InterpretedCommand, InterpretationError> {
        if args.iter().any(|arg| arg == "<ARG>") {
            return Err(InterpretationError::ArgSpecifierMisused);
        }

        if args[0].is_empty() {
            return Err(InterpretationError::ArgumentEmpty {
                argument_name: "alias to remove".to_string(),
            });
        }

        Ok(InterpretedCommand::RemoveAlias {
            alias: args[0].clone(),
        })
    }

    fn help(&self) -> &str {
        Help::remove_alias_arg()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct HelpCommand;
impl Action for HelpCommand {
    fn execute(&self, _args: Vec<String>) -> Result<InterpretedCommand, InterpretationError> {
        println!("{}", &get_help_message());

        Ok(InterpretedCommand::DoNothing)
    }

    fn help(&self) -> &str {
        Help::help()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ListAvailableCommandsCommand;
impl Action for ListAvailableCommandsCommand {
    fn execute(&self, _args: Vec<String>) -> Result<InterpretedCommand, InterpretationError> {
        Ok(InterpretedCommand::ListAvailableCommands)
    }

    fn help(&self) -> &str {
        Help::list_available_commands()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ExplainCommandCommand;
impl Action for ExplainCommandCommand {
    fn execute(&self, args: Vec<String>) -> Result<InterpretedCommand, InterpretationError> {
        if args.iter().any(|arg| arg == "<ARG>") {
            return Err(InterpretationError::ArgSpecifierMisused);
        }

        if args[0].is_empty() {
            return Err(InterpretationError::ArgumentEmpty {
                argument_name: "command to explain".to_string(),
            });
        }

        Ok(InterpretedCommand::ExplainCommand {
            command: args[0].clone(),
        })
    }

    fn help(&self) -> &str {
        Help::explain_command_arg()
    }
}
