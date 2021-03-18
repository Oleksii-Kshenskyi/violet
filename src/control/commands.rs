extern crate chrono;
extern crate enum_dispatch;
use chrono::Local;
use enum_dispatch::*;

use crate::config::get_violet_name;
use crate::config::get_exit_message;

pub enum InterpretedCommand {
    DoNothing,
    Exit { exit_message: String },
    AddAlias { alias: String, for_builtin: String},
    RemoveAlias { alias: String},
}

pub enum InterpretationError {
    WrongArgumentCount {expected: usize, actual: usize},
}

#[enum_dispatch]
#[derive(Clone)]
pub enum Command {
    ExitCommand,
    CurrentTimeCommand,
    WhatsYourNameCommand,
    SayThisAndThatCommand,
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
