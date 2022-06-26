use crate::ERROR_NON_UNICODE;
use std::{
    fmt::{Display, Write},
    path::PathBuf,
};

#[derive(Debug)]
pub struct PathPrompt {
    home: Option<PathBuf>,
    path: PathBuf,
}

impl PathPrompt {
    pub fn new(home: Option<PathBuf>, path: PathBuf) -> Self {
        debug_assert!(path.is_absolute(), "path should be absolute");
        debug_assert!(
            home.as_ref().map(|p| p.is_absolute()).unwrap_or(true),
            "home should be absolute"
        );

        Self { home, path }
    }
}

impl Display for PathPrompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use termion::{color, style};

        let dir = if let Some(home) = &self.home && let Ok(rest) = self.path.strip_prefix(home) {
            if f.alternate() {
                write!(f, "{}~{}", color::Fg(color::Cyan), style::Reset)?;
            } else {
                f.write_char('~')?;
            }
            rest
        } else {
            self.path.as_path()
        };

        let mut iter = dir.components();
        if let Some(target) = iter.next_back() {
            f.write_char('/')?;

            let target = target.as_os_str().to_str().expect(ERROR_NON_UNICODE);

            if f.alternate() {
                for part in iter {
                    write!(
                        f,
                        "{}{part}{}/",
                        color::Fg(color::Cyan),
                        style::Reset,
                        part = part.as_os_str().to_str().expect(ERROR_NON_UNICODE)
                    )?;
                }

                write!(f, "{}{target}{}", color::Fg(color::Cyan), style::Reset)?;
            } else {
                for part in iter {
                    write!(
                        f,
                        "{}/",
                        part.as_os_str().to_str().expect(ERROR_NON_UNICODE)
                    )?;
                }

                write!(f, "{target}",)?;
            }
        }

        Ok(())
    }
}
