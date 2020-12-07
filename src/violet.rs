mod config;
mod control;
mod data;
mod io;
mod util;

use crate::control::interpreter::Interpreter;

fn main() {
    Interpreter::new().run_repl();
}
