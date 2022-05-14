pub fn split_args(args: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut files = Vec::new();
    let mut opts = Vec::new();

    for s in args {
        if s.starts_with("-") || s.starts_with("--") {
            opts.push(s);
        } else if !s.is_empty() {
            files.push(s)
        }
    }
    (files, opts)
}
