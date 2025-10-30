use chrono::prelude::*;
use chrono::{DateTime, Local};
use clap::{ErrorKind, IntoApp, Parser};
use std::error::Error;
use std::fs::{self, DirEntry};
use std::path::Path;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Config {
    /// Source directory
    #[clap(short, long)]
    source: String,

    /// Destination parent directory, which should contain the YYYY folders
    #[clap(short, long)]
    destination: String,
}

// Ensure a directory exists
fn check_directory(path: String) {
    let exists = Path::new(&path).is_dir();
    let mut app = Config::into_app();

    if !exists {
        app.error(
            ErrorKind::InvalidValue,
            format!("{} is not a directory", path),
        )
        .exit();
    }
}

// Actually move the file from <source> to <destination>/YYYY/MM/DD/<filename>.png
fn move_file(destination: String, path: &Path) {
    println!("Moving {:?}", path);

    let metadata = fs::metadata(path.clone()).unwrap();
    let created = metadata.created().unwrap();
    let dt: DateTime<Local> = created.clone().into();

    let new_dir = &path
        .join(destination)
        .join(dt.year().to_string())
        .join(dt.format("%m").to_string())
        .join(dt.format("%d").to_string());

    let dir_exists = Path::new(&path).is_dir();

    if !dir_exists {
        println!("Creating {}", new_dir.to_str().unwrap());
        fs::create_dir_all(new_dir.to_str().unwrap()).unwrap();
    }

    let new_path = &new_dir.join(path.file_name().unwrap().to_str().unwrap());
    fs::rename(path, new_path).unwrap();
}

// Process each .png in the source directory
/// Only handle files that match our naming conventions
fn process_entry(destination: String, entry: DirEntry) -> Result<(), Box<dyn Error>> {
    let path = entry.path();

    let filename = match path.file_name().unwrap().to_str() {
        Some(name) => name,
        None => return Ok(()),
    };

    let metadata = fs::metadata(path.clone())?;
    let created = metadata.created()?.elapsed()?.as_secs();

    // See if this entry was created less than a day ago and return
    if created <= 24 * 3600 && metadata.is_file() {
        println!("Skipping too new {}", filename);
        return Ok(()); // skip it
    }

    if filename.starts_with("CleanShot") || filename.starts_with("Screen Shot") {
        move_file(destination, &path);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::parse();

    // Make sure our directories actually exist
    check_directory(config.source.clone());
    check_directory(config.destination.clone());

    // Loop over all the files in the source directory
    for entry in fs::read_dir(config.source)? {
        let entry = entry?;
        let path = entry.path();
        let extension = path.extension();

        // Process files ending in .png
        match extension {
            Some(_) => process_entry(config.destination.clone(), entry)?,
            _ => continue,
        }
    }

    Ok(())
}
