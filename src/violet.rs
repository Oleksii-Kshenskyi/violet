mod control;
mod data;
mod io;
mod util;
mod config;

use crate::control::interpreter::Interpreter;

fn main() {
    Interpreter::new().run_repl();
}
