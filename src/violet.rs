mod config;
mod io;
use config::*;

fn main() {
    println!("Welcome to Violet the command interpreter!");
    println!("Violet's version is {};", get_violet_version());
    println!("Created by {}.", get_violet_author());

    loop {
        let user_input = io::input::get_user_input(get_violet_prompt().as_str());
        match user_input.as_str() {
            "exit" => {
                println!("Bye! AYAYA");
                break;
            }
            _ => println!("You said: [{}]", user_input),
        }
    }
}
