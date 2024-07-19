use clap::{Command, Error};
use std::{fs, path::Path};

fn main() -> Result<(), Error> {
    let matches = Command::new("meg")
        .version("1.0")
        .author("aeiouxx")
        .about("Metabranch tool for git")
        .subcommand(
            Command::new("init")
                .about("Initialize the MetaGit Tool in the current superproject repository"),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", _)) => {
            meg_init()?;
            Ok(())
        }
        _ => {
            println!("Invalid command");
            Ok(())
        }
    }
}

fn meg_init() -> Result<(), Error> {
    let meg_path = Path::new(".meg");

    if (meg_path).exists() {
        println!("MetaGit Tool already initialized");
        return Ok(());
    } else {
        fs::create_dir(meg_path)?;
        println!("MetaGit Tool initialized");
    }

    Ok(())
}
