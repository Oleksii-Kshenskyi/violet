extern crate enum_dispatch;
extern crate chrono;
use enum_dispatch::*;
use chrono::Local;

use std::process::exit;


#[enum_dispatch]
#[derive(Clone)]
pub enum Command {
   ExitCommand,
   CurrentTimeCommand
}

#[enum_dispatch(Command)]
pub trait Action {
   fn execute(&self);
}

#[derive(Clone)]
pub struct ExitCommand;
impl Action for ExitCommand {
   fn execute(&self) {
      println!("BYE! AYAYA ^_^");
      exit(0);
   }
}

#[derive(Clone)]
pub struct CurrentTimeCommand;
impl Action for CurrentTimeCommand {
   fn execute(&self) {
      println!("Your system clock says it's {} now!", Local::now().format("%I:%M %p"));
   }
}