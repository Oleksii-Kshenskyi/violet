extern crate chrono;
extern crate enum_dispatch;
use chrono::Local;
use enum_dispatch::*;

use std::{fmt::Display, io::Write, process::exit};

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

pub trait Printable {
    fn what<T: Display>(&mut self, print_this: T) -> &mut Self;
    fn print_to<W: Write>(&self, print_to_this: W);
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
    fn print_to<W: Write>(&self, mut print_to_this: W) {
        let err_message = "ERROR: exit: couldn't write to the supplied writer!";
        print_to_this.write(self.what.as_bytes()).expect(err_message);
        print_to_this.flush().expect(err_message);
    }
}
impl Action for ExitCommand {
    fn execute(&mut self) {
        self.what("BYE! AYAYA ^_^\n").print_to(std::io::stdout());
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
    fn print_to<W: Write>(&self, mut print_to_this: W) {
        let err_message = "ERROR: current time command: couldn't write to the supplied writer!";
        print_to_this.write(format!("Your system clock says it's {} now!\n", self.time).as_bytes()).expect(err_message);
        print_to_this.flush().expect(err_message);
    }
}
impl Action for CurrentTimeCommand {
    fn execute(&mut self) {
        self.what(Local::now().format("%I:%M %p")).print_to(std::io::stdout());
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
    fn print_to<W: Write>(&self, mut print_to_this: W) {
        let err_message = "ERROR: my name command: couldn't write to the supplied writer!";
        print_to_this.write(format!("My name is {}! Nice to meet you ^_^\n", self.name).as_bytes()).expect(err_message);
        print_to_this.flush().expect(err_message);
    }
}
impl Action for WhatsYourNameCommand {
    fn execute(&mut self) {
        self.what(&get_violet_name()).print_to(std::io::stdout());
    }
}
