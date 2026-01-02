use atty::Stream;
use crossterm::{
    ExecutableCommand,
    cursor::{self, MoveTo},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};
use std::sync::Once;
use std::time::Instant;

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
#[derive(Copy, Clone)]
pub enum Color {
    Red,
    Green,
    Yellow,
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
            Color::Gray => "\x1b[90m",
            Color::Cyan => "\x1b[36m",
            Color::Reset => "\x1b[0m",
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
static mut NEXT_ROW: u16 = 0;

fn cursor_hide() {
    INIT.call_once(|| {
        enable_raw_mode().unwrap();
        let (_, row) = cursor::position().unwrap();
        unsafe {
            NEXT_ROW = row;
        }
        io::stdout().execute(cursor::Hide).unwrap();
    });
}

fn cursor_restore() {
    let mut out = io::stdout();
    out.execute(cursor::Show).unwrap();
    out.execute(MoveTo(0, unsafe { NEXT_ROW + 1 })).unwrap();
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
}

impl ProgressBar {
    pub fn new(desc: &str, len: usize, size: usize) -> Self {
        let term_mode = if !atty::is(Stream::Stdout) {
            TerminalMode::Headless
        } else {
            TerminalMode::Interactive
        };

        if term_mode == TerminalMode::Interactive {
            cursor_hide();
        }

        let row;
        unsafe {
            row = NEXT_ROW;
            NEXT_ROW += 1;
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

            row,
            col: 0,

            term_mode,
        }
    }

    pub fn tick(&mut self, progress: usize) {
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
        print!("{}", self.fill_color);
        for _ in 0..self.curr {
            print!("{}", self.fill_style);
        }
        print!("{}", self.empty_color);
        for _ in self.curr..self.len {
            print!("{}", self.empty_style);
        }
        print!("{} ", Color::Reset.ch());

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
    }

    pub fn style(&mut self, fill: FillStyle, emp: EmptyStyle) {
        self.fill_style = fill.ch();
        self.empty_style = emp.ch();
    }

    pub fn color(&mut self, fill: Color, emp: Color) {
        self.fill_color = fill.ch();
        self.empty_color = emp.ch();
    }
}

impl Drop for ProgressBar {
    fn drop(&mut self) {
        if self.term_mode == TerminalMode::Interactive {
            cursor_restore();
        }
    }
}
