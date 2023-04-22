//! Main controls for the CLI.
use core::panic;
use std::path::PathBuf;

use clap::Parser;
use vtk2obj::Mesh;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Minimum [`std`] prints.
    #[arg(short, long)]
    pub input: PathBuf,

    /// Turns on [`the`] [`log`] it's verbose -- not reccommended.
    #[arg(short, long)]
    pub output: PathBuf,
}

impl Cli {
    pub fn init() -> Self {
        Cli::parse()
    }
}
fn main() {
    let cli = Cli::init();
    let mesh = Mesh::from_vtk_file(cli.input);
    match mesh {
        Ok(m) => m.to_obj(cli.output).expect("The app was able to parse the .vtk file, but not able to export an obj to disk. This could be file a file permissions issue."),
                Err(e) => {
            panic!("Unable to convert this file, perhaps the .vtk version is unsupported.. this app was kinda made to be a once off. But, you can tell the authour you received this error: {e}\nIf you can also provide the .vtk file that borked it --we'll take a look!");
        }
    }
}
