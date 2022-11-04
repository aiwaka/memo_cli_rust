use std::error::Error;
use std::io::{stdin, stdout, Write};

use crate::io::create_new_file;

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

pub(super) fn new_command(name: &Option<String>) -> Result<(), Box<dyn Error>> {
    let name = if let Some(name) = name {
        name.clone()
    } else {
        // 名前が指定されていないなら聞く
        input_name()
    };
    create_new_file(&name)?;

    Ok(())
}
