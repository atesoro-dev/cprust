use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

fn parse_args(args: &[String]) -> (Vec<String>, Vec<PathBuf>) {
    let mut options = Vec::new();
    let mut paths = Vec::new();

    let mut force_as_path = false;

    for arg in args.iter().skip(1) {

        if force_as_path {
            paths.push(PathBuf::from(arg));
            continue;
        }

        if arg == "--" {
            force_as_path = true;
        }
        else if arg == "-" {
            paths.push(PathBuf::from(arg));
        }
        else if arg.starts_with("--") {
            options.push(arg.clone());
        }
        else if arg.starts_with('-') && arg.len() > 1 && !arg[1..].contains('-'){
            for ch in arg.chars().skip(1) {
                options.push(format!("-{}", ch));
            }
        }
        else if arg.starts_with('-') {
            options.push(arg.clone());
        }
        else {
            paths.push(PathBuf::from(arg));
        }
    }

    (options, paths)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let (options, paths) = parse_args(&args);

    println!("Options: {:?}", options);
    println!("Paths: {:?}", paths);
    
    let dst_path: &Path = Path::new(&args[2]);
    if dst_path.is_dir() {
        println!("{} is a directory", dst_path.display());
    }
    else {
        println!("{} is not a directory", dst_path.display());
    }
    let is_option = &args[1].find('-');
    if is_option.is_none() {
        println!("{} is not an option", &args[1]);
    }
    else {
        println!("{} is an option", &args[1]);
    }
    let src_file:&String = &args[1];
    let dst_file:&String = &args[2];
    fs::copy(src_file, dst_file)?;
    Ok(())
}