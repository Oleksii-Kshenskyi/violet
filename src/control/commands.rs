extern crate chrono;
extern crate enum_dispatch;
use chrono::Local;
use enum_dispatch::*;

use crate::config::get_violet_name;
use crate::config::get_exit_message;

pub enum InterpretedCommand {
    DoNothing,
    Exit { exit_message: String },
    AddAlias { alias: String, for_builtin: String },
    RemoveAlias { alias: String },
}

pub enum InterpretationError {
    WrongArgumentCount {expected: usize, actual: usize},
    ArgumentEmpty {argument_name: String},
}

#[enum_dispatch]
#[derive(Clone)]
pub enum Command {
    ExitCommand,
    CurrentTimeCommand,
    WhatsYourNameCommand,
    SayThisAndThatCommand,
    AddAliasCommand,
    RemoveAliasCommand,
}

#[enum_dispatch(Command)]
pub trait Action {
    fn execute(&self, args: Vec<String>) -> Result<InterpretedCommand, InterpretationError>;
}

#[derive(Clone)]
pub struct ExitCommand;
impl Action for ExitCommand {
    fn execute(&self, _args: Vec<String>) -> Result<InterpretedCommand, InterpretationError> {
        Ok(InterpretedCommand::Exit { exit_message: get_exit_message() })
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct WhatsYourNameCommand;
impl Action for WhatsYourNameCommand {
    fn execute(&self, _args: Vec<String>) -> Result<InterpretedCommand, InterpretationError> {
        println!("My name is {}! Nice to meet you ^_^", &get_violet_name());

        Ok(InterpretedCommand::DoNothing)
    }
}

#[derive(Clone)]
pub struct SayThisAndThatCommand;
impl Action for SayThisAndThatCommand {
    fn execute(&self, args: Vec<String>) -> Result<InterpretedCommand, InterpretationError> {
        if args.len() != 2 {
            println!("Something went horribly wrong...");
            return Err(InterpretationError::WrongArgumentCount { expected: 2, actual: args.len() });
        }
        println!(
            "Gotcha. Saying {} and {}!",
            args.get(0).unwrap(),
            args.get(1).unwrap()
        );

        Ok(InterpretedCommand::DoNothing)
    }
}

#[derive(Clone)]
pub struct AddAliasCommand;
impl Action for AddAliasCommand {
    fn execute(&self, args: Vec<String>) -> Result<InterpretedCommand, InterpretationError> {
        if args.len() != 2 {
            return Err(InterpretationError::WrongArgumentCount { expected: 2, actual: args.len() });
        }

        if args[0].is_empty() {
            return Err(InterpretationError::ArgumentEmpty {argument_name: format!("{}", "alias to add")});
        }
        if args[1].is_empty() {
            return Err(InterpretationError::ArgumentEmpty {argument_name: format!("{}", "builtin name")});
        }

        Ok(InterpretedCommand::AddAlias { alias: args[0].clone(), for_builtin: args[1].clone() })
    }
}

#[derive(Clone)]
pub struct RemoveAliasCommand;
impl Action for RemoveAliasCommand {
    fn execute(&self, args: Vec<String>) -> Result<InterpretedCommand, InterpretationError> {
        if args.len() != 1 {
            return Err(InterpretationError::WrongArgumentCount {expected: 1, actual: args.len()});
        }

        if args[0].is_empty() {
            return Err(InterpretationError::ArgumentEmpty { argument_name: format!("{}", "alias to remove")});
        }

        Ok(InterpretedCommand::RemoveAlias {alias: args[0].clone()})
    }
}
