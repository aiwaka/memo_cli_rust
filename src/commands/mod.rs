use crate::parser::{AppArgs, Subcommands};

use self::{
    copy::copy_command, edit::edit_command, info::info_command, list::list_memos, new::new_command,
    view::view_command,
};

mod copy;
mod edit;
mod info;
mod list;
mod new;
mod view;

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
        Subcommands::View { name } => {
            view_command(name);
        }
        Subcommands::Remove { name } => {}
        Subcommands::Copy { md, name } => {
            copy_command(md, name);
        }
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
