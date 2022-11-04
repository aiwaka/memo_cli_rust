use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::APP_CONFIG;

pub(crate) fn create_new_file(filename: &str) -> Result<(), Box<dyn Error>> {
    let filename = format!("{}.txt", filename);
    let storage_dir = Path::new(&APP_CONFIG.get().unwrap().storage_dir);

    let path = storage_dir.join(Path::new(&filename));
    let display = path.display();
    println!("{}", display);

    Ok(())
}
