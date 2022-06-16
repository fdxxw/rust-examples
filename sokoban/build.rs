use std::{path::PathBuf, error};

use zip_extensions::zip_create_from_directory;

fn main() -> Result<(), Box<dyn error::Error>> {
    let archive_file: PathBuf = PathBuf::from("./resources.zip");
    let source_dir: PathBuf = PathBuf::from("./resources");
    zip_create_from_directory(&archive_file, &source_dir)?;
    Ok(())
}
