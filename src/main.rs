use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
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