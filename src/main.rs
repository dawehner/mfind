extern crate docopt;
extern crate rustc_serialize;
extern crate regex;

use docopt::Docopt;
use std::env;
use std::string::String;
use std::path::Path;
use std::fs;
use regex::Regex;

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
    // docopt.unwrap().argv( = Some(argv);

    let args: Args = docopt.and_then(|d| return d.argv(argv).decode()).unwrap_or_else(|e| e.exit());
    return args;
}

fn main() {
    let args = compute_args(env::args().map(|res| res).collect());

    let current_dir = &env::current_dir().unwrap();
    let path = Path::new(current_dir);
    let regex = Regex::new(&*args.arg_filename).unwrap();
    find_files(path, &regex);
}

fn find_files(path: &Path, regex: &Regex) {
    let paths = fs::read_dir(path).unwrap();

    let names =
    paths.map(|entry| {
        let entry = entry.unwrap();
        let entry_path = entry.path();

        match fs::metadata(entry_path.as_path()) {
            Ok(x) => if x.is_dir() { find_files(entry_path.as_path(), regex) },
            Err(e) => println!("{}", e)
        }
    
        let file_name = entry_path.file_name().unwrap();
        let file_name_as_str = file_name.to_str().unwrap();
        let file_name_as_string = String::from(file_name_as_str);

        return file_name_as_string;
    }).collect::<Vec<String>>();

    for name in names {
        let result = regex.find(&*name);
        if result.is_some() {
            println!("{:?}", name);
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
