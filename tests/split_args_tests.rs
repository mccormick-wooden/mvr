// use mvr;

// #[test]
// fn split_args_ignores_empty_args() {
//     let args: Vec<String> = ["".to_string(), "".to_string()].to_vec();

//     let (files, opts) = mvr::split_args(args);

//     assert!(files.is_empty());
//     assert!(opts.is_empty());
// }

// #[test]
// fn split_args_returns_empty_files_and_empty_opts() {
//     let args: Vec<String> = [].to_vec();

//     let (files, opts) = mvr::split_args(args);

//     assert!(files.is_empty());
//     assert!(opts.is_empty());
// }

// #[test]
// fn split_args_returns_some_files_and_empty_opts() {
//     let args: Vec<String> = [
//         "testfile1".to_string(),
//         "testfile2".to_string(),
//         "testdir".to_string(),
//     ]
//     .to_vec();
//     let args_comp = args.clone();

//     let (files, opts) = mvr::split_args(args);

//     assert_eq!(files.len(), args_comp.len());
//     assert!(opts.is_empty());

//     assert_eq!(files, args_comp);
// }

// #[test]
// fn split_args_returns_empty_files_and_some_opts() {
//     let args: Vec<String> = ["-v".to_string(), "--update".to_string()].to_vec();
//     let args_comp = args.clone();

//     let (files, opts) = mvr::split_args(args);

//     assert!(files.is_empty());
//     assert_eq!(opts.len(), args_comp.len());

//     assert_eq!(opts, args_comp);
// }

// #[test]
// fn split_args_returns_some_files_and_some_opts() {
//     let args: Vec<String> = [
//         "testfile1".to_string(),
//         "-v".to_string(),
//         "testfile2".to_string(),
//         "--update".to_string(),
//         "testdir".to_string(),
//     ]
//     .to_vec();
//     let args_comp = args.clone();

//     let (files, opts) = mvr::split_args(args);

//     assert_eq!(files.len(), 3);
//     assert_eq!(opts.len(), 2);

//     assert_eq!(files[0], args_comp[0]);
//     assert_eq!(files[1], args_comp[2]);
//     assert_eq!(files[2], args_comp[4]);

//     assert_eq!(opts[0], args_comp[1]);
//     assert_eq!(opts[1], args_comp[3]);
// }
