use std::{env, error::Error, fmt::Write as _, process};
use termion::color;

// cmd <path> [size]
fn main() {
    match exec() {
        Ok(result) => println!("{}", result),
        Err(_) => {
            println!(
                "[{fg}error{n}]",
                fg = color::Fg(color::Red),
                n = color::Fg(color::Reset)
            );

            process::exit(1)
        }
    }
}

fn exec() -> Result<String, Box<dyn Error>> {
    let mut args = env::args();
    let dir = match args.nth(1) {
        Some(dir) => dir,
        None => env::current_dir()?.to_string_lossy().into_owned(),
    };

    let home = env::var("HOME")?;

    let mut buf = String::new();

    // strip home
    let dir = if let Some(no_home) = dir.strip_prefix(&home) {
        write!(
            &mut buf,
            "{fg_t}~{n}",
            fg_t = color::Fg(color::Cyan),
            n = color::Fg(color::Reset)
        )?;
        no_home
    } else {
        &dir
    };

    let mut iter = dir.split('/').filter(|s| !s.is_empty());
    if let Some(target) = iter.next_back() {
        write!(&mut buf, "/")?;
        let size = args
            .next()
            .map(|size| size.parse().ok())
            .flatten()
            .unwrap_or(48);

        // write prefix
        for part in iter {
            if dir.len() > size {
                write!(
                    &mut buf,
                    "{fg_t}{part}{n}/",
                    fg_t = color::Fg(color::LightBlack),
                    part = &part[..1],
                    n = color::Fg(color::Reset)
                )?;
            } else {
                write!(
                    &mut buf,
                    "{fg_t}{part}{n}/",
                    fg_t = color::Fg(color::Cyan),
                    n = color::Fg(color::Reset)
                )?;
            };
        }

        // write target separately
        write!(
            &mut buf,
            "{fg_t}{target}{n}",
            fg_t = color::Fg(color::Cyan),
            n = color::Fg(color::Reset)
        )?;
    }

    Ok(buf)
}
