use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Arguments {
    /// Set alert time to a config file.
    Set {
        /// Time as an integer (e.g. 2 for 2 hours)
        #[structopt()]
        time: u8,
    },
    /// Disable an alert from the config file
    Disable {
        #[structopt()]
        id: usize,
    },
    /// Show the status of all the alerts (previous & current)
    Status
}

#[derive(Debug, StructOpt)]
#[structopt(
name = "GetUp!",
about = "A command line app that triggers an alert at a time set by the user to get up and stretch/move"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub arguments: Arguments,

    #[structopt(parse(from_os_str), short, long)]
    pub config_file: Option<PathBuf>,
}

use std::fmt;
use std::fmt::Display;

impl Display for CommandLineArgs{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?}", self.arguments, self.config_file)
    }
}