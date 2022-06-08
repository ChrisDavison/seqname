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

    ///Keep current name while prefixing or suffixing
    #[structopt(short, long)]
    keep_filename: bool,

    /// Show files moved/renamed
    #[structopt(short, long)]
    verbose: bool,

    /// Show files moved/renamed
    #[structopt(short, long)]
    dry_run: bool,

    /// Separator for file components
    #[structopt(long)]
    separator: Option<String>,

    /// Directories with files to rename
    dirs: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let renamer = Renamer::new(
        opt.prefix,
        opt.suffix,
        opt.verbose,
        opt.keep_filename,
        opt.dry_run,
        opt.separator,
    );
    for dir in opt.dirs {
        if let Err(e) = renamer.rename(&dir) {
            eprintln!("Failed in dir {:?}: {}", dir.display(), e);
        }
    }
    Ok(())
}
