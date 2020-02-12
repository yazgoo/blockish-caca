extern crate crossterm;
extern crate regex;
use std::process::Command;
use std::os::unix::process::CommandExt;
use std::error::Error;
use std::env;
use regex::Regex;

fn play_video(player: &String, path: &String) -> Result<(), Box<dyn Error>> {
    let exe = env::current_exe()?;
    let bin_dir = exe.parent().ok_or("bin_dir")?.to_str().ok_or("to str")?;
    let mut quiet = "-quiet";
    let mut vo = "-vo";
    let mut ld_preload = format!("{}/libcaca_blockish.so", bin_dir);
    let mut com = Command::new(player);
    if player == "cvlc" {
        quiet = "--quiet";
        vo = "-V";
        com.env("DISPLAY", "");
        ld_preload += ":";
        let output = Command::new("ldconfig").arg("-p").output()?;
        let pattern = Regex::new(r"^[\s]*libcaca.so .* ([^ ]+)$")?;
        ld_preload += String::from_utf8(output.stdout)?
            .lines()
            .filter_map(|line| pattern.captures(line) )
            .map(|cap| cap[1].to_string())
            .collect::<Vec<String>>()[0].as_str();
    }
    com.env("CACA_DRIVER", "raw")
        .env("LD_PRELOAD", ld_preload)
        .arg(quiet)
        .arg(vo)
        .arg("caca")
        .arg(path)
        .exec();
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let player = &args[1];
    let path = &args[2];
    play_video(player, path)
}
