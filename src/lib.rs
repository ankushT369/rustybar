use crossterm::{
    ExecutableCommand,
    cursor::{self, MoveTo},
    event,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use std::time::Duration;
use std::{
    io::{self, IsTerminal, Write},
    sync::{
        Once,
        atomic::{AtomicU16, Ordering},
    },
    time::Instant,
};

const UNICODE_BAR_FULL_CHARS: &[char] = &['█', '#', '=', '━'];
const UNICODE_BAR_EMPTY_CHARS: &[char] = &['█', ' ', '-', '━'];

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
enum TerminalMode {
    Interactive,
    Headless,
}

#[allow(dead_code)]
pub enum FillStyle {
    Solid,
    Hash,
    Equal,
    Thin,
}

#[allow(dead_code)]
pub enum EmptyStyle {
    Solid,
    Space,
    Dash,
    Thin,
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Pink,
    Gray,
    Cyan,
    Reset,
}

impl Color {
    #[inline(always)]
    const fn ch(self) -> &'static str {
        match self {
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Pink => "\x1b[35m",
            Color::Gray => "\x1b[90m",
            Color::Cyan => "\x1b[36m",
            Color::Reset => "\x1b[0m",
        }
    }

    #[inline(always)]
    const fn rgb(self) -> (u8, u8, u8) {
        match self {
            Color::Red => (255, 60, 60),
            Color::Green => (60, 255, 120),
            Color::Yellow => (255, 220, 60),
            Color::Blue => (80, 140, 255),
            Color::Pink => (255, 100, 200),
            Color::Gray => (160, 160, 160),
            Color::Cyan => (60, 220, 255),
            Color::Reset => (255, 255, 255),
        }
    }
}

impl FillStyle {
    fn ch(self) -> char {
        UNICODE_BAR_FULL_CHARS[self as usize]
    }
}

impl EmptyStyle {
    fn ch(self) -> char {
        UNICODE_BAR_EMPTY_CHARS[self as usize]
    }
}

static INIT: Once = Once::new();
static NEXT_ROW: AtomicU16 = AtomicU16::new(0);

fn cursor_hide() {
    INIT.call_once(|| {
        enable_raw_mode().unwrap();
        let (_, row) = cursor::position().unwrap();
        NEXT_ROW.store(row, Ordering::Release);
        io::stdout().execute(cursor::Hide).unwrap();
    });
}

fn cursor_restore() {
    let mut out = io::stdout();
    out.execute(cursor::Show).unwrap();
    out.execute(MoveTo(0, NEXT_ROW.load(Ordering::Acquire) + 1))
        .unwrap();
    disable_raw_mode().unwrap();
}

pub struct ProgressBar {
    desc: String,
    len: usize,
    size: usize,

    fill_style: char,
    empty_style: char,

    curr: usize,

    start_time: Instant,

    fill_color: &'static str,
    empty_color: &'static str,

    row: u16,
    col: u16,

    term_mode: TerminalMode,

    grad: bool,
    grad_start: Color,
    grad_end: Color,
}

impl ProgressBar {
    pub fn new(desc: &str, len: usize, size: usize) -> Self {
        let term_mode = if !io::stdout().is_terminal() {
            TerminalMode::Headless
        } else {
            TerminalMode::Interactive
        };

        if term_mode == TerminalMode::Interactive {
            cursor_hide();
        }

        Self {
            desc: desc.to_string(),
            len,
            size,

            fill_style: FillStyle::Hash.ch(),
            empty_style: EmptyStyle::Dash.ch(),

            curr: 0,

            start_time: Instant::now(),

            fill_color: Color::Green.ch(),
            empty_color: Color::Gray.ch(),

            row: NEXT_ROW.fetch_add(1, Ordering::AcqRel),
            col: 0,

            term_mode,

            grad: false,
            grad_start: Color::Green,
            grad_end: Color::Green,
        }
    }

    pub fn tick(&mut self, progress: usize) -> bool {
        if ctrl_c() {
            return true;
        }
        let percent = (progress * 100) / self.size.max(1);
        self.curr = (percent * self.len) / 100;

        let elapsed = self.start_time.elapsed();
        let speed = progress as f64 / elapsed.as_secs_f64().max(0.0001);
        let remaining = self.size.saturating_sub(progress);
        let eta_secs = (remaining as f64 / speed).max(0.0);
        let eta = std::time::Duration::from_secs_f64(eta_secs);

        let mut out = io::stdout();
        if self.term_mode == TerminalMode::Interactive {
            out.execute(MoveTo(self.col, self.row)).unwrap();
        }

        print!("{} ", self.desc);
        if !self.grad {
            self.print_bar();
        } else {
            self.print_grad_bar();
        }

        let mut disp_speed = speed;
        let mut unit = "B/s";

        if disp_speed >= 1024.0 {
            disp_speed /= 1024.0;
            unit = "KB/s";
        }

        if disp_speed >= 1024.0 {
            disp_speed /= 1024.0;
            unit = "MB/s";
        }

        print!(
            "{}%  elapsed {:02}:{:02}  <  ETA {:02}:{:02}  @ {:.2} {}",
            percent,
            elapsed.as_secs() / 60,
            elapsed.as_secs() % 60,
            eta.as_secs() / 60,
            eta.as_secs() % 60,
            disp_speed,
            unit
        );

        out.flush().unwrap();

        false
    }

    pub fn style(&mut self, fill: FillStyle, emp: EmptyStyle) {
        self.fill_style = fill.ch();
        self.empty_style = emp.ch();
    }

    pub fn color(&mut self, fill: Color, emp: Color) {
        self.fill_color = fill.ch();
        self.empty_color = emp.ch();
    }

    pub fn gradient(&mut self, start: Color, end: Color) {
        self.grad = start != end;
        self.grad_start = start;
        self.grad_end = end;
    }

    fn print_bar(&self) {
        print!("{}", self.fill_color);
        for _ in 0..self.curr {
            print!("{}", self.fill_style);
        }
        print!("{}", self.empty_color);
        for _ in self.curr..self.len {
            print!("{}", self.empty_style);
        }
        print!("{} ", Color::Reset.ch());
    }

    fn print_grad_bar(&self) {
        let (sr, sg, sb) = self.grad_start.rgb();
        let (er, eg, eb) = self.grad_end.rgb();

        for i in 0..self.curr {
            let t = i as f32 / (self.curr.saturating_sub(1).max(1)) as f32;

            let r = sr as f32 + t * (er as f32 - sr as f32);
            let g = sg as f32 + t * (eg as f32 - sg as f32);
            let b = sb as f32 + t * (eb as f32 - sb as f32);

            print!(
                "\x1b[38;2;{};{};{}m{}",
                r as u8, g as u8, b as u8, self.fill_style
            );
        }

        print!("{}", self.empty_color);
        for _ in self.curr..self.len {
            print!("{}", self.empty_style);
        }

        print!("{}", Color::Reset.ch());
    }
}

impl Drop for ProgressBar {
    fn drop(&mut self) {
        if self.term_mode == TerminalMode::Interactive {
            cursor_restore();
        }
    }
}

/// Returns true if CTRL+C has been pressed
///
fn ctrl_c() -> bool {
    // event::poll() checks if there is an event ready to be handled
    // if you call event::read() without this it would halt the program until there is an event
    if let Ok(res) = event::poll(Duration::ZERO) {
        if !res {
            return false;
        }
        // Check if the event read was of type event::Event::Key. We don't care about mouse and other events
        if let event::Event::Key(event) = event::read().unwrap() {
            // If the key pressed has the modifier CONTROL and the char of the key is 'c', i.e. CTRL+C
            if event.modifiers == event::KeyModifiers::CONTROL
                && event.code == event::KeyCode::Char('c')
            {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bar = ProgressBar::new("Downloading", 50, 100);

        assert_eq!(bar.desc, "Downloading");
        assert_eq!(bar.len, 50);
        assert_eq!(bar.size, 100);
        assert_eq!(bar.curr, 0);

        assert_eq!(bar.fill_style, FillStyle::Hash.ch());
        assert_eq!(bar.empty_style, EmptyStyle::Dash.ch());
    }
}
