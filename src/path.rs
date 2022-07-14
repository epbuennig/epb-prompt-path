use crate::ERROR_NON_UNICODE;
use std::{
    fmt::{Display, Write},
    path::{Component, Path, PathBuf},
};

#[derive(Debug)]
pub struct Prompt {
    home: Option<PathBuf>,
    path: PathBuf,
}

impl Prompt {
    pub fn new(home: Option<PathBuf>, mut path: PathBuf) -> Self {
        if !path.is_absolute() {
            path = Path::new("???").join(path);
        }

        debug_assert!(
            home.as_ref().map(|p| p.is_absolute()).unwrap_or(true),
            "home should be absolute"
        );

        Self { home, path }
    }
}

impl Display for Prompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use termion::{color, style};

        let dir = if let Some(home) = &self.home && let Ok(rest) = self.path.strip_prefix(home) {
            if f.alternate() {
                write!(f, "{}~{}", color::Fg(color::Cyan), style::Reset)?;
            } else {
                f.write_char('~')?;
            }
            rest
        } else if let Ok(rest) = self.path.strip_prefix(Path::new("???")) {
            if f.alternate() {
                write!(f, "{}???{}", color::Fg(color::Cyan), style::Reset)?;
            } else {
                f.write_str("???")?;
            }
            rest
        } else {
            self.path.as_path()
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
                        color::Fg(color::Cyan),
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
        } else {
            if f.alternate() {
                write!(f, "{}/{}", color::Fg(color::Cyan), style::Reset,)?;
            } else {
                f.write_char('/')?;
            }
        }

        Ok(())
    }
}
