use crate::parser::{AppArgs, Subcommands};

use self::new::new_command;

mod new;

pub(crate) fn execute_commands(args: &AppArgs) -> Result<(), Box<dyn std::error::Error>> {
    match &args.subcommands {
        Subcommands::New { name } => {
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
