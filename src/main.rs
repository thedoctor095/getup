mod cli;
mod core;

use std::path::PathBuf;
use core::Alert;
use anyhow::anyhow;
use cli::{Arguments::*, CommandLineArgs};
use structopt::StructOpt;

fn find_config() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {path.push(".gp.json"); path})
}

fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        arguments,
        config_file,
    } = CommandLineArgs::from_args();

    let config_file = config_file
        .or_else(find_config)
        .ok_or(anyhow!("Failed to find journal file."))?;

    match arguments {
        Set { time } => core::add_alert(config_file, Alert::new(time)),
        Status => core::list_alerts(config_file),
        Disable { id } => core::disable_alert(config_file, id),
    }?;
    // println!("{}", CommandLineArgs::from_args());
    Ok(())
}
