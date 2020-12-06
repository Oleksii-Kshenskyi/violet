use crate::util::treepath::TreePath;
use crate::data::pathtree::PathTree;
use crate::config;
use crate::io::input;

pub struct Interpreter {
   commands: PathTree<String>,
}

impl Interpreter {
   pub fn new() -> Self {
      Self {
         commands: PathTree::new()
      }
   }

   pub fn run_repl(&mut self) {
      println!("Welcome to Violet the command interpreter!");
      println!("Violet's version is {};", config::get_violet_version());
      println!("Created by {}.", config::get_violet_author());

      loop {
         let user_input = input::get_user_input(config::get_violet_prompt());
         let pathified = TreePath::create_path(&user_input);
         match user_input.as_str() {
            "exit" => {
                  println!("Bye! AYAYA");
                  break;
            }
            "" => continue,
            _ => {
                  self.commands.set_by_path(String::from("Pogomega"), pathified.clone());
                  println!(
                     "OK: set node at path '{}' to '{}';",
                     pathified.join(" "),
                     self.commands.get_by_path(pathified).unwrap().value
                  );
                  println!();
                  println!("PathTree now: [{:?}]", self.commands.tree);
            }
         }
      }
   }
}