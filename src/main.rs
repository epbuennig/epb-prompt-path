#![feature(let_chains)]

use path::PathPrompt;
use std::{env, error::Error, path::PathBuf, process};
use termion::{color, style};

mod path;

const ERROR_NON_UNICODE: &str = "fatal error on resolving reference, was not utf8";

fn exec() -> Result<PathPrompt, Box<dyn Error>> {
    let home = env::var_os("HOME").map(Into::<PathBuf>::into);
    let dir = match env::args().nth(1) {
        Some(dir) => dir.into(),
        None => env::current_dir()?,
    };

    Ok(PathPrompt::new(home, dir))
}

fn main() {
    match exec() {
        Ok(result) => println!("{:#}", result),
        Err(err) => {
            println!(
                "[{}{}error{}]",
                style::Bold,
                color::Fg(color::Red),
                style::Reset
            );

            if let Some("--debug") = env::args().nth(2).as_deref() {
                eprintln!("{err:?}");
            }

            process::exit(1)
        }
    }
}
