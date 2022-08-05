#![feature(let_chains)]

use path::Prompt;
use std::{env, error::Error, path::PathBuf, process};
use termion::{color, style};

mod path;

const ERROR_NON_UNICODE: &str = "fatal error on resolving reference, was not utf8";

fn exec() -> Result<Prompt, Box<dyn Error>> {
    let host = if env::var_os("SSH_CLIENT").is_some() {
        Some(
            hostname::get()?
                .into_string()
                .map_err(|os| format!("cannot convert to string: {os:?}"))?,
        )
    } else {
        None
    };

    let home = env::var_os("HOME").map(Into::<PathBuf>::into);
    let dir = match env::args().nth(1) {
        Some(dir) => dir.into(),
        None => env::current_dir()?,
    };

    Ok(Prompt::new(host, home, dir))
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
