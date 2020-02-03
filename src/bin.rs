extern crate crossterm;
use std::process::Command;
use std::os::unix::process::CommandExt;
use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let player = &args[1];
    let path = &args[2];
    let (width, height) = crossterm::terminal::size()?;
    let width = width * 8;
    let height = height * 16 - 1;
   Command::new(player)
       .env("CACA_DRIVER", "raw")
       .env("LD_PRELOAD", "target/release/libcaca_blockish.so")
       .env("BLOCKISH_WIDTH", width.to_string())
       .env("BLOCKISH_HEIGHT", height.to_string())
       .arg("-quiet")
       .arg("-vo")
       .arg("caca")
       .arg("-vf")
       .arg(format!("scale={}:{}", &width, &height))
       .arg(path)
       .exec();

    Ok(())
}
