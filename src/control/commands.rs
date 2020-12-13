extern crate chrono;
extern crate enum_dispatch;
use chrono::Local;
use enum_dispatch::*;

use std::process::exit;

use crate::config::get_violet_name;

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
    fn execute(&self, args: Vec<String>);
}

#[derive(Clone)]
pub struct ExitCommand;
impl Action for ExitCommand {
    fn execute(&self, _args: Vec<String>) {
        println!("BYE! AYAYA ^_^");
        exit(0);
    }
}

#[derive(Clone)]
pub struct CurrentTimeCommand;
impl Action for CurrentTimeCommand {
    fn execute(&self, _args: Vec<String>) {
        println!(
            "Your system clock says it's {} now!",
            Local::now().format("%I:%M %p")
        );
    }
}

#[derive(Clone)]
pub struct WhatsYourNameCommand;
impl Action for WhatsYourNameCommand {
    fn execute(&self, _args: Vec<String>) {
        println!("My name is {}! Nice to meet you ^_^", &get_violet_name());
    }
}

#[derive(Clone)]
pub struct SayThisAndThatCommand;
impl Action for SayThisAndThatCommand {
    fn execute(&self, args: Vec<String>) {
        if args.len() != 2 {
            println!("Something went horribly wrong...");
            return;
        }
        println!(
            "Gotcha. Saying {} and {}!",
            args.get(0).unwrap(),
            args.get(1).unwrap()
        );
    }
}
