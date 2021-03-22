extern crate chrono;
extern crate enum_dispatch;
use chrono::Local;
use enum_dispatch::*;

use crate::config::get_exit_message;
use crate::config::get_violet_name;
use crate::config::get_help_message;

use serde::{Deserialize, Serialize};

pub enum InterpretedCommand {
    DoNothing,
    Exit { exit_message: String },
    AddAlias { alias: String, for_builtin: String },
    RemoveAlias { alias: String },
}

pub enum InterpretationError {
    WrongArgumentCount { expected: usize, actual: usize },
    ArgumentEmpty { argument_name: String },
}

#[enum_dispatch]
#[derive(Clone, Serialize, Deserialize)]
pub enum Command {
    ExitCommand,
    CurrentTimeCommand,
    WhatsYourNameCommand,
    SayThisAndThatCommand,
    AddAliasCommand,
    RemoveAliasCommand,
    HelpCommand,
}

#[enum_dispatch(Command)]
pub trait Action {
    fn execute(&self, args: Vec<String>) -> Result<InterpretedCommand, InterpretationError>;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ExitCommand;
impl Action for ExitCommand {
    fn execute(&self, _args: Vec<String>) -> Result<InterpretedCommand, InterpretationError> {
        Ok(InterpretedCommand::Exit {
            exit_message: get_exit_message(),
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CurrentTimeCommand;
impl Action for CurrentTimeCommand {
    fn execute(&self, _args: Vec<String>) -> Result<InterpretedCommand, InterpretationError> {
        println!(
            "Your system clock says it's {} now!",
            Local::now().format("%I:%M %p")
        );

        Ok(InterpretedCommand::DoNothing)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WhatsYourNameCommand;
impl Action for WhatsYourNameCommand {
    fn execute(&self, _args: Vec<String>) -> Result<InterpretedCommand, InterpretationError> {
        println!("My name is {}! Nice to meet you ^_^", &get_violet_name());

        Ok(InterpretedCommand::DoNothing)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SayThisAndThatCommand;
impl Action for SayThisAndThatCommand {
    fn execute(&self, args: Vec<String>) -> Result<InterpretedCommand, InterpretationError> {
        if args.len() != 2 {
            println!("Something went horribly wrong...");
            return Err(InterpretationError::WrongArgumentCount {
                expected: 2,
                actual: args.len(),
            });
        }
        println!(
            "Gotcha. Saying {} and {}!",
            args.get(0).unwrap(),
            args.get(1).unwrap()
        );

        Ok(InterpretedCommand::DoNothing)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AddAliasCommand;
impl Action for AddAliasCommand {
    fn execute(&self, args: Vec<String>) -> Result<InterpretedCommand, InterpretationError> {
        if args.len() != 2 {
            return Err(InterpretationError::WrongArgumentCount {
                expected: 2,
                actual: args.len(),
            });
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
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RemoveAliasCommand;
impl Action for RemoveAliasCommand {
    fn execute(&self, args: Vec<String>) -> Result<InterpretedCommand, InterpretationError> {
        if args.len() != 1 {
            return Err(InterpretationError::WrongArgumentCount {
                expected: 1,
                actual: args.len(),
            });
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
}

#[derive(Clone, Serialize, Deserialize)]
pub struct HelpCommand;
impl Action for HelpCommand {
    fn execute(&self, _args: Vec<String>) -> Result<InterpretedCommand, InterpretationError> {
        println!("{}", &get_help_message());

        Ok(InterpretedCommand::DoNothing)
    }
}
