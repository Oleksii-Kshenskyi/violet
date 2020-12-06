extern crate enum_dispatch;
use std::process::exit;
use enum_dispatch::*;

#[enum_dispatch]
#[derive(Clone)]
pub enum Command {
   ExitCommand
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