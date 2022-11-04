use crate::{
    parser::{AppArgs, Subcommands},
    vim::edit_with_vim,
};

use self::new::new_command;

mod new;

pub(crate) fn execute_commands(args: &AppArgs) -> Result<(), Box<dyn std::error::Error>> {
    match &args.subcommands {
        Subcommands::New { name } => {
            new_command(name)?;
        }
        Subcommands::List { full } => {}
        Subcommands::Edit { name } => {
            if let Some(name) = name {
                edit_with_vim(name.clone());
            }
            println!("exit")
        }
        Subcommands::View { name } => {}
        Subcommands::Remove { name } => {}
        Subcommands::Spawn { md, name } => {}
        Subcommands::Serve => {}
    }

    Ok(())
}
