use std::fs::File;
use std::io::prelude::*;
use crate::application::Applications;

pub fn _print_list(mut app: Applications) -> std::io::Result<()> {
    let mut file = File::create("output/file.txt")?;
    writeln!(file, "{}", app._view_apps()).expect("Failed to write.");
    Ok(())
}
