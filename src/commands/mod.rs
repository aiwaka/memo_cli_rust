use crate::{
    editor::edit_with_vim,
    parser::{AppArgs, Subcommands},
};

use self::{edit::edit_command, info::info_command, list::list_memos, new::new_command};

mod edit;
mod info;
mod list;
mod new;

pub(crate) fn execute_commands(args: &AppArgs) -> Result<(), Box<dyn std::error::Error>> {
    match &args.subcommands {
        Subcommands::New { name } => {
            new_command(name)?;
        }
        Subcommands::List { full } => {
            list_memos(full);
        }
        Subcommands::Edit { name } => {
            edit_command(name);
        }
        Subcommands::View { name } => {}
        Subcommands::Remove { name } => {}
        Subcommands::Spawn { md, name } => {}
        Subcommands::Serve => {}
        Subcommands::Info {
            version,
            storage,
            all,
        } => {
            info_command(version, storage, all);
        }
    }

    Ok(())
}
