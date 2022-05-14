use mvr;
use std::env;

// rough spec
// mv testfile othertestfile othertestdir = moves two files to dir
// mv testfile othertestfile = renames file
// mv testfile othertestdir/othertestfile = moves testfile to othertestdir
// mv testfile othertestdir = moves testfile into othertestdir

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let (files, opts) = mvr::split_args(args);

    if files.is_empty() {
        println!("mvr: missing file operand");
        return;
    }

    for file in files {
        if !std::path::Path::new(file.as_str()).exists() {
            println!("mvr: No such file or directory");
            return;
        }
    }
}
