# The Violet interpreter

Violet is a command interpreter.
Its main intended feature is a syntax close to plain English.

The very basic alpha version (0.2.0) has just been released, so the very basic mechanics have been implemented. Violet has a few commands, can take single word and multi word arguments, you can set and remove aliases for existing commands, and Violet also provides a shortcut syntax for commands.

For example, Violet currently has these basic commands:
- `explain command <ARG>`
- `list available commands`
- `what time is it`
- `please say <ARG> and <ARG>`
- `help`
- `add alias <ARG> for builtin <ARG>`
- `exit`
- `remove alias <ARG>`
- `what is your name`

If you're interested in playing around with Violet, you can either clone it from this git repo and issue the standard `cargo build` + `cargo run` commands for Rust projects in the command line (if you have the latest Rust stable installed), or you can take the distibution from the Releases section (0.2.0 is available). However, Violet doesn't do anything too useful yet, as 0.2.0 was a milestone for implementing the basic underlying mechanics of an interpreter. Future milestones are probably going to include implementing more useful features.

`help` (or `[h]`) command is a pretty big information dump for users to get familiar with the basic mechanics of Violet. `list available commands` (or `[lac]`) will tell you what commands Violet has available. `explain command "<command name>"` (or `[eca] "<command name>"`) can explain specific commands to you in more detail.

### IMPORTANT

The project is extremely new, only the basics have been implemented and it doesn't do anything useful yet. It should be stable and not have any critical bugs though.

## License

This is a completely free and opensource project licensed under MIT. Provided you fulfill MIT's conditions, you can do whatever you want with it.
