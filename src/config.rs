use crate::util::string::clone_uppercased;

const VIOLET_UNKNOWN: &str = "???";
const VIOLET_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const VIOLET_AUTHOR: Option<&'static str> = option_env!("CARGO_PKG_AUTHORS");
const VIOLET_PROMPT: &str = "<<VIO>>";
const VIOLET_NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
const VIOLET_EXIT_MESSAGE: &str = "Bye! AYAYA ^_^";
const VIOLET_CONFIG_FILE_NAME: &str = "./config.json";

const VIOLET_HELP_MESSAGE: &str = "Violet is a command interpreter.
When you see the \"<<VIO>> \" prompt, it means you can enter your command and press <ENTER>.
---
Violet is going to try to interpret that command or let you know if it doesn't know such a command.
---
You can exit Violet with
<<VIO>> exit
\tor
<<VIO>> [e]
---
If you want to see the list of available commands, the command for that is:
<<VIO>> list available commands
\tor
<<VIO>> [loc]
---
If you want to see the explanation for an individual command:
<<VIO>> explain command <ARG>
or
<<VIO>> [eca] <ARG>
If the command is a multi-node command (not just \"exit\", but i.e. \"what is your name\"), take care to enclose it in the \" quotation marks, like so:
<<VIO>> explain command \"what is your name\"
---
Violet has some commands with arguments. For example, the
<<VIO>> please say <ARG> and <ARG>
command just echoes the two arguments back to you.
---
Arguments can also be either multi word arguments or single word arguments.
---
Example of using single word arguments:
<<VIO>> please say one and two
Gotcha! Saying one and two!
---
Example of using multi word arguments:
<<VIO>> please say \"one argument\" and \"another argument\"
Gotcha! Saying one argument and another argument!
---
All Violet commands are \"paths\" consisting of \"nodes\" separated by spaces.
Example:
what is your name => This is a \"path\".
what, what is, what is your, what is your name => those are all \"nodes\".
---
Violet has a command shortcut syntax. It works in the following way.
The shortcut command itself is enclosed in [ and ].
The letters for the command are first letters of each node in the original command.
The letter for <ARG> is a.
The arguments themselves are nodes that follow the command shortcut in [], separated by spaces.
---
Shortcut example:
Shortcut for
<<VIO>> exit
\tis
<<VIO>> [e]
---
Shortcut example 2:
Shortcut for
<<VIO>> please say this and that
\tis
<<VIO>> [psaaa] this that
---
Shortcut example 3:
Shortcut for
<<VIO>> remove alias \"alias for exit\"
is
<<VIO>> [raa] \"alias for exit\"
";

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

pub fn get_help_message() -> String {
    VIOLET_HELP_MESSAGE.to_string()
}
