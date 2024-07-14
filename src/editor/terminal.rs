use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType,
                            EnterAlternateScreen, LeaveAlternateScreen};
use std::io::{stdout, Error, Write};

#[derive(Default, Copy, Clone)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

#[derive(Copy, Clone, Default)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}
pub struct Terminal;

impl Terminal {
    pub fn terminate() -> Result<(), Error> {
        Self::leave_alternate_screen()?;
        Self::show_caret()?;
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::enter_alternate_screen()?;
        Self::clear_screen()?;
        Self::move_caret_to(Position { col: 0, row: 0 })?;
        Self::execute()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn move_caret_to(position: Position) -> Result<(), Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        queue!(stdout(), MoveTo(position.col as u16, position.row as u16))?;
        Ok(())
    }

    pub fn enter_alternate_screen() -> Result<(), Error> {
        queue!(stdout(), EnterAlternateScreen)?;
        Ok(())
    }

    pub fn leave_alternate_screen() -> Result<(), Error> {
        queue!(stdout(), LeaveAlternateScreen)?;
        Ok(())
    }
    
    pub fn hide_caret() -> Result<(), Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }

    pub fn show_caret() -> Result<(), Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }

    pub fn print_row(row: usize, line_text:&str) -> Result<(), Error> {
        Self::move_caret_to(Position{row, col:0})?;
        Self::clear_line()?;
        Self::print(line_text)?;
        Ok(())
    }

    pub fn print(string: &str) -> Result<(), Error> {
        queue!(stdout(), Print(string))?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?;
        #[allow(clippy::as_conversions)]
        let height = height_u16 as usize;
        let width = width_u16 as usize;
        Ok(Size { height, width})
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

}