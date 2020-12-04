use std::io;
use std::io::Write;

pub fn get_user_input(prompt: String) -> String {
    print!("{}", prompt);
    std::io::stdout()
        .flush()
        .expect("ERROR: flushing stdout failed!");

    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => String::from(user_input.trim()),
        Err(error) => format!("ERROR: read_line(): {}", error),
    }
}
