use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::SetCursorStyle;
use crossterm::ExecutableCommand;
use std::io::stdout;

pub struct RawMode;

impl RawMode {
    pub fn new() -> Result<Self, crossterm::ErrorKind> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        stdout().execute(SetCursorStyle::BlinkingBar)?;

        Ok(RawMode)
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        let _ = stdout().execute(LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}
