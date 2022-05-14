// rough spec
// mv testfile othertestfile othertestdir = moves two files to dir
// mv testfile othertestfile = renames file
// mv testfile othertestdir/othertestfile = moves testfile to othertestdir
// mv testfile othertestdir = moves testfile into othertestdir

// static struct option const long_options[] =
// {
//   {"backup", optional_argument, NULL, 'b'},
//   {"force", no_argument, NULL, 'f'},
//   {"interactive", no_argument, NULL, 'i'},
//   {"no-clobber", no_argument, NULL, 'n'},
//   {"no-target-directory", no_argument, NULL, 'T'},
//   {"strip-trailing-slashes", no_argument, NULL, STRIP_TRAILING_SLASHES_OPTION},
//   {"suffix", required_argument, NULL, 'S'},
//   {"target-directory", required_argument, NULL, 't'},
//   {"update", no_argument, NULL, 'u'},
//   {"verbose", no_argument, NULL, 'v'},
//   {GETOPT_HELP_OPTION_DECL},
//   {GETOPT_VERSION_OPTION_DECL},
//   {NULL, 0, NULL, 0}
// };

use clap::Parser;
use mvr;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Files or directory to move
    #[clap(short, long, parse(from_os_str), min_values(1))]
    source: Vec<PathBuf>,

    /// File or directory to move to
    #[clap(short, long, parse(from_os_str))]
    target: PathBuf,
    //     /// Sets a custom config file
    //     #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    //     config: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    for s in cli.source {
        match mvr::is_source_valid(&s) {
            Err(msg) => {
                println!("{}", msg);
                std::process::exit(1);
            }
            _ => (),
        }
    }
    // let args: Vec<String> = env::args().skip(1).collect();

    // let (files, opts) = mvr::split_args(args);

    // if files.is_empty() {
    //     println!("mvr: missing file operand");
    //     return;
    // }

    // for file in files {
    //if !std::path::Path::new(file.as_str()).exists() {
    //         println!("mvr: No such file or directory");
    //         return;
    //     }
    // }
}
