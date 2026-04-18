use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    fs::copy(&args[1], &args[2])?;
    Ok(())
}