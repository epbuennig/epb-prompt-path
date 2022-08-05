use crate::ERROR_NON_UNICODE;
use std::{
    fmt::{Display, Write},
    path::{Component, Path, PathBuf},
};

#[derive(Debug)]
pub struct Prompt {
    host: Option<String>,
    home: Option<PathBuf>,
    path: PathBuf,
}

impl Prompt {
    pub fn new(host: Option<String>, home: Option<PathBuf>, mut path: PathBuf) -> Self {
        if !path.is_absolute() {
            path = Path::new("???").join(path);
        }

        debug_assert!(
            home.as_ref().map(|p| p.is_absolute()).unwrap_or(true),
            "home should be absolute"
        );

        Self { host, home, path }
    }
}

impl Display for Prompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use termion::{color, style};

        if let Some(host) = &self.host {
            if f.alternate() {
                write!(f, "[{}{host}{}]:", color::Fg(color::Blue), style::Reset)?;
            } else {
                write!(f, "[{host}]:")?;
            }
        }

        let (dir, is_home) = if let Some(home) = &self.home && let Ok(rest) = self.path.strip_prefix(home) {
            if f.alternate() {
                write!(f, "{}~{}", color::Fg(color::Blue), style::Reset)?;
            } else {
                f.write_char('~')?;
            }
            (rest, true)
        } else if let Ok(rest) = self.path.strip_prefix(Path::new("???")) {
            if f.alternate() {
                write!(f, "{}???{}", color::Fg(color::Blue), style::Reset)?;
            } else {
                f.write_str("???")?;
            }
            (rest, false)
        } else {
            (self.path.as_path(), false)
        };

        let mut iter = dir
            .components()
            .filter(|comp| !matches!(comp, Component::RootDir))
            .peekable();

        if iter.peek().is_some() {
            if f.alternate() {
                for part in iter {
                    write!(
                        f,
                        "/{}{part}{}",
                        color::Fg(color::Blue),
                        style::Reset,
                        part = part.as_os_str().to_str().expect(ERROR_NON_UNICODE)
                    )?;
                }
            } else {
                for part in iter {
                    write!(
                        f,
                        "/{}",
                        part.as_os_str().to_str().expect(ERROR_NON_UNICODE)
                    )?;
                }
            }
        } else if !is_home {
            if f.alternate() {
                write!(f, "{}/{}", color::Fg(color::Blue), style::Reset)?;
            } else {
                f.write_char('/')?;
            }
        }

        Ok(())
    }
}
