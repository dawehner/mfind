extern crate docopt;
extern crate rustc_serialize;
extern crate regex;
extern crate walkdir;

use docopt::Docopt;
use std::env;
use std::string::String;
use std::path::Path;
use regex::Regex;
use walkdir::WalkDir;


#[derive(Debug, RustcDecodable)]
struct Args {
    arg_filename: String,
    arg_folder: Option<String>,
    flag_help: bool,
    flag_version: bool,
    flag_type: Option<String>
}

fn compute_args(argv: Vec<String>) -> Args {
    const USAGE: &'static str = "
    Usage: mfind [options] <folder> <filename>
           mfind [options] <filename>
           mfind (--help | --version)

    Options:
        --type, -t TYPE  Specify the type (folder or file)
    ";

    let docopt = Docopt::new(USAGE);

    let args: Args = docopt.and_then(|d| return d.argv(argv).decode()).unwrap_or_else(|e| e.exit());
    return args;
}

fn main() {
    let args = compute_args(env::args().map(|res| res).collect());

    let regex = Regex::new(&*args.arg_filename).unwrap();
    match args.arg_folder {
        Some(x) => find_files(Path::new(&*x), &regex),
        None => find_files_current_dir(&regex),
    }
}

fn find_files_current_dir(regex: &Regex) {
    let current_dir = &env::current_dir().unwrap();
    let path = Path::new(current_dir);
    find_files(path, &regex);
}

fn find_files(path: &Path, regex: &Regex) {
    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();

        let file_name_as_str = entry.path().file_name().unwrap().to_str().unwrap();

        let result = regex.find(&file_name_as_str);
        if result.is_some() {
            println!("{:?}", file_name_as_str);
        }
    }
}

#[test]
fn compute_args_test_just_search() {
    let argv = vec!["mfind".to_string(), "myname".to_string()];
    let args = compute_args(argv);
    assert_eq!(args.arg_filename, "myname");
    assert_eq!(args.arg_folder, None);
}

#[test]
fn compute_args_test_search_and_folder() {
    let argv = vec!["mfind".to_string(), "myfolder".to_string(), "myname".to_string()];
    let args = compute_args(argv);
    assert_eq!(args.arg_filename, "myname");
    assert_eq!(args.arg_folder.unwrap(), "myfolder");
}
