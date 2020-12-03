mod config;
mod io;
mod data;
use config::*;
use data::pathtree::PathTree;

fn main() {
    println!("Welcome to Violet the command interpreter!");
    println!("Violet's version is {};", get_violet_version());
    println!("Created by {}.", get_violet_author());

    let mut app_command_tree = PathTree::new();

    loop {
        let user_input = io::input::get_user_input(get_violet_prompt());
        let pathified = PathTree::create_path(&user_input);
        match user_input.as_str() {
            "exit" => {
                println!("Bye! AYAYA");
                break;
            }
            _ => {
                app_command_tree.set_by_path(String::from("Pogomega"), pathified);
                println!("PathTree now: [{:?}]", app_command_tree.tree);
            },
        }
    }
}
