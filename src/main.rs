use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    
    let src_file:&String = &args[1];
    let dst_file:&String = &args[2];
    fs::copy(src_file, dst_file)?;
    Ok(())
}