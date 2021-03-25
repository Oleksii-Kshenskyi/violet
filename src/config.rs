use crate::util::string::clone_uppercased;

const VIOLET_UNKNOWN: &str = "???";
const VIOLET_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const VIOLET_AUTHOR: Option<&'static str> = option_env!("CARGO_PKG_AUTHORS");
const VIOLET_PROMPT: &str = "<<VIO>>";
const VIOLET_NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
const VIOLET_EXIT_MESSAGE: &str = "Bye! AYAYA ^_^";
const VIOLET_CONFIG_FILE_NAME: &str = "./config.json";

#[macro_export]
macro_rules! argcount_err { () => {
        "ERROR: Wrong argument count; expected {}, found {}!\n\nNOTE: This sometimes happens when you put the <ARG> argument specifier directly into the command as an argument.\n      If you did that, please specify an actual argument instead.\n      Passing <ARG> as a single self-contained argument without quotation marks (like this: please say <ARG> and <ARG>) to a command is considered a mistake on the user's side.\nExample: instead of\n<<VIO>> explain command <ARG>\n  please use\n<<VIO>> explain command help\n"
    }
}

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
<<VIO>> [lac]
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
---
If the same shortcut is created several times due to name collision, the second and subsequent shortcuts are appended with a sequential number.
Example:
Assume we have three [eaa] commands, and they were added in this order:
1. enable alias <ARG>
2. exists argument <ARG>
3. eat ate <ARG>
Three aliases would be created for that:
- [eaa] <ARG>, which calls enable alias <ARG>;
- [eaa2] <ARG>, which calls exists argument <ARG>;
- [eaa3] <ARG>, which calls eat <ARG> <ARG>.
---
NOTE: aliases with different arity (number of arguments) are considered different commands. Therefore, if we have:
1. enable alias <ARG> // arity 1
2. eat <ARG> <ARG> // arity 2
The aliases created are going to be:
- [eaa] <ARG> // for enable alias <ARG>
- [eaa] <ARG> <ARG> // for eat <ARG> <ARG>
NOTE: there are no sequential numbers like [eaa2] here. The reason is the different arities of the two commands.
---
You can see what a certain shortcut expands to with \"explain command <ARG>\":
Example: if you have two commands with the [eca] shortcut name, \"explain command <ARG>\" and \"eat candy <ARG>\"
<<VIO>> explain command \"[eca] <ARG>\"
  [[help for explain command <ARG>]]
<<VIO>> explain command \"[eca2] <ARG>\"
  [[help for eat candy <ARG>]]
---
You can of course use shortcuts while meta-asking for explanation about explanation:
<<VIO>> [eca] \"[eca] <ARG>\"
  [[help for explain command <ARG>]]
";

pub struct Help;
impl Help {
    pub fn exit() -> &'static str {
        "<<VIO>> exit
          Exits Violet and saves Violet's data correctly.
        ---
        WARNING: if you don't want Violet's data (such as aliases) to be lost between sessions, please always take care to exit via the exit command."
    }

    pub fn what_is_your_name() -> &'static str {
        "<<VIO>> what is your name
          Makes Violet give you a short and cute introduction! ^_^
        "
    }

    pub fn what_time_is_it() -> &'static str {
        "<<VIO>> what time is it
          Violet tells you what time it is according to your system clock.
        ---
        NOTE: this doesn't fetch current time from the web. If your system clock is incorrect, then you're going to get the incorrect time.
        "
    }

    pub fn please_say_arg_and_arg() -> &'static str {
        "<<VIO>> please say <ARG> and <ARG>
          This command just echoes the two arguments you provide back at you.
        ---
        Example:
        <<VIO>> please say one and two
        Gotcha! Saying one and two!
        Example 2:
        <<VIO>> please say \"this is arg\" and \"this is also arg\"
        Gotcha! Saying this is arg and this is also arg!
        "
    }

    pub fn add_alias_arg_for_builtin_arg() -> &'static str {
        "<<VIO>> add alias <ARG> for builtin <ARG>
          Adds a new alias for an existing built-in command so that the new alias would invoke the same command as the builtin when called.
        ---
        Example 1:
        <<VIO>> add alias shutdown for builtin exit
        <<VIO>> shutdown
        Bye! AYAYA ^_^
        ---
        Example 2:
        <<VIO>> add alias \"alias <ARG> for <ARG>\" for builtin \"add alias <ARG> for builtin <ARG>\"
        <<VIO>> alias \"blow up\" for \"exit\"
        <<VIO>> blow up
        Bye! AYAYA ^_^
        ---
        NOTE 1: You can't set an alias if an identical one already exists.
        NOTE 2: You can't set an alias to a name identical to an existing built-in command.
        NOTE 3: The alias and the builtin have to have the same amount of <ARG>s, otherwise the alias won't be set successfully.
        NOTE 4: You can set an alias for both add alias and remove alias commands.
        "
    }

    pub fn remove_alias_arg() -> &'static str {
        "<<VIO>> remove alias <ARG>
          Removes an existing alias.
        ---
        Example:
        <<VIO>> add alias \"shutdown\" for builtin \"exit\"
        <<VIO>> remove alias \"shutdown\"
        <<VIO>> shutdown
        shutdown: command does not exist.
        ---
        NOTE 1: You obviously cannot remove an alias which doesn't exist.
        NOTE 2: If you try to invoke this with a builtin as an argument, Violet will explicitly tell you that you can't remove builtins.
        "
    }

    pub fn list_available_commands() -> &'static str {
        "<<VIO>> list available commands
          lists all the currently available built-in commands.
        "
    }

    pub fn explain_command_arg() -> &'static str {
        "<<VIO>> explain command <ARG>
          The command currently being invoked. Explains the command specified in the <ARG>.
        "
    }

    pub fn help() -> &'static str {
        "<<VIO>> help
          A concise yet information-dense intro to the basics of Violet.
        "
    }
}

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
