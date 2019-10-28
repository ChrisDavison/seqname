use std::path::PathBuf;
use structopt::StructOpt;

mod renamer;

use renamer::Renamer;

#[derive(Debug, StructOpt)]
#[structopt(name = "seqname", about = "rename files sequentially")]
struct Opt {
    /// Text to insert before number
    #[structopt(short, long)]
    prefix: Option<String>,

    /// Text to insert after number
    #[structopt(short, long)]
    suffix: Option<String>,

    /// Show files moved/renamed
    #[structopt(short, long)]
    verbose: bool,

    /// Directories with files to rename
    dirs: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let renamer = Renamer::new(opt.prefix, opt.suffix, opt.verbose);
    for dir in opt.dirs {
        match renamer.rename(&dir) {
            Ok(_) => continue,
            Err(e) => eprintln!("Failed in dir {:?}: {}", dir.display(), e),
        };
    }
    Ok(())
}
