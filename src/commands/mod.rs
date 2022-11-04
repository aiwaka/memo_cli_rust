use crate::parser::{AppArgs, Subcommands};
use std::io::{stdin, stdout, Write};

use self::new::new_command;

mod new;

fn input_name() -> String {
    let mut name = String::new();
    print!("Input Name: ");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut name)
        .expect("could not read string.");
    if let Some('\n') = name.chars().next_back() {
        name.pop();
    }
    if let Some('\r') = name.chars().next_back() {
        name.pop();
    }
    name
}

pub(crate) fn execute_commands(args: &AppArgs) -> Result<(), Box<dyn std::error::Error>> {
    match &args.subcommands {
        Subcommands::New { name } => {
            let name = if let Some(name) = name {
                name.clone()
            } else {
                // 名前が指定されていないなら聞く
                input_name()
            };
            new_command(name)?;
        }
        Subcommands::List { full } => {}
        Subcommands::Edit { name } => {}
        Subcommands::View { name } => {}
        Subcommands::Remove { name } => {}
        Subcommands::Spawn { md, name } => {}
        Subcommands::Serve => {}
    }

    Ok(())
}
