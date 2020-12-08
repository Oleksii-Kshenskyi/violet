extern crate chrono;
extern crate enum_dispatch;
use chrono::Local;
use enum_dispatch::*;

use std::{fmt::Display, process::exit};

use crate::config::get_violet_name;

#[enum_dispatch]
#[derive(Clone)]
pub enum Command {
    ExitCommand,
    CurrentTimeCommand,
    WhatsYourNameCommand,
}

#[enum_dispatch(Command)]
pub trait Action {
    fn execute(&mut self);
}

pub trait Printable: Action {
    fn what<T: Display>(&mut self, print_this: T) -> &mut Self;
    fn print(&self);
}

#[derive(Clone)]
pub struct ExitCommand {
    what: String
}
impl ExitCommand {
    pub fn new() -> Self {
        ExitCommand {
            what: "".to_string()
        }
    }
}
impl Printable for ExitCommand {
    fn what<T: Display>(&mut self, print_this: T) -> &mut Self { 
        self.what = print_this.to_string();
        self
    }
    fn print(&self) {
        println!("{}", self.what);
    }
}
impl Action for ExitCommand {
    fn execute(&mut self) {
        self.what("BYE! AYAYA ^_^").print();
        exit(0);
    }
}

#[derive(Clone)]
pub struct CurrentTimeCommand {
    time: String
}
impl CurrentTimeCommand {
    pub fn new() -> Self {
        Self {
            time: String::from("")
        }
    }
}
impl Printable for CurrentTimeCommand {
    fn what<T: Display>(&mut self, print_this: T) -> &mut Self { 
        self.time = print_this.to_string();
        self
    }
    fn print(&self) {
        println!("Your system clock says it's {} now!", self.time);
    }
}
impl Action for CurrentTimeCommand {
    fn execute(&mut self) {
        self.what(Local::now().format("%I:%M %p")).print();
    }
}

#[derive(Clone)]
pub struct WhatsYourNameCommand {
    name: String
}
impl WhatsYourNameCommand {
    pub fn new() -> Self {
        Self {
            name: String::from("")
        }
    }
}
impl Printable for WhatsYourNameCommand {
    fn what<T: Display>(&mut self, print_this: T) -> &mut Self { 
        self.name = print_this.to_string();
        self
    }
    fn print(&self) {
        println!("My name is {}! Nice to meet you ^_^", self.name);
    }
}
impl Action for WhatsYourNameCommand {
    fn execute(&mut self) {
        self.what(&get_violet_name()).print();
    }
}
