use std::error::Error;

use crate::io::create_new_file;

pub(super) fn new_command(name: String) -> Result<(), Box<dyn Error>> {
    create_new_file(&name)?;

    Ok(())
}
