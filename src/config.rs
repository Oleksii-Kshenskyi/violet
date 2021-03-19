use crate::util::string::clone_uppercased;

const VIOLET_UNKNOWN: &str = "???";
const VIOLET_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const VIOLET_AUTHOR: Option<&'static str> = option_env!("CARGO_PKG_AUTHORS");
const VIOLET_PROMPT: &str = "<<VIO>>";
const VIOLET_NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
const VIOLET_EXIT_MESSAGE: &str = "Bye! AYAYA ^_^";
const VIOLET_CONFIG_FILE_NAME: &str = "./config.json";

pub fn get_violet_version() -> String {
    VIOLET_VERSION.unwrap_or(VIOLET_UNKNOWN).to_owned()
}

pub fn get_violet_author() -> String {
    VIOLET_AUTHOR.unwrap_or(VIOLET_UNKNOWN).to_owned()
}

pub fn get_violet_prompt() -> String {
    format!("{} ", VIOLET_PROMPT)
}

pub fn get_violet_name() -> String {
    clone_uppercased(VIOLET_NAME.unwrap_or(VIOLET_UNKNOWN))
}

pub fn get_exit_message() -> String {
    VIOLET_EXIT_MESSAGE.to_string()
}

pub fn get_config_file_name() -> String {
    VIOLET_CONFIG_FILE_NAME.to_string()
}