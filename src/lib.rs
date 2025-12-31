use std::io::Write;
use std::io;
use std::time::{Instant};

// Global Array
const UNICODE_BAR_FULL_CHARS: &[char] = &['█', '#', '=', '━'];
const UNICODE_BAR_EMPTY_CHARS: &[char] = &[' ', '-', '━'];

#[allow(dead_code)]
pub enum FillStyle { Solid, Hash, Equal, Thin }

#[allow(dead_code)]
pub enum EmptyStyle { Space, Dash, Thin }

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Color { Red, Green, Yellow, Gray, Cyan, Reset }


impl Color {
    #[inline(always)]
    const fn ch(self) -> &'static str {
        match self {
            Color::Red    => "\x1b[31m",
            Color::Green  => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Gray   => "\x1b[90m",
            Color::Cyan   => "\x1b[36m",
            Color::Reset  => "\x1b[0m",
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
}

impl ProgressBar {
    pub fn new(desc: &str, len: usize, size: usize) -> Self {
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
        }
    }

    pub fn tick(&mut self, progress: usize) {
        clearscreen::clear().expect("Failed to clear screen");

        let percent = (progress * 100) / self.size;
        self.curr = (percent * self.len) / 100;

        let elapsed = self.start_time.elapsed();

        let speed = if elapsed.as_secs_f64() > 0.0 {
            progress as f64 / elapsed.as_secs_f64()
        } else {
            0.0
        };

        let remaining = self.size - progress;

        let eta_secs = if speed > 0.0 {
            remaining as f64 / speed
        } else {
            0.0
        };

        let eta = std::time::Duration::from_secs_f64(eta_secs);

        print!("\r{} ", self.desc);
        io::stdout().flush().unwrap();

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

        println!(
            "{}%  elapsed {:02}:{:02}  <  ETA {:02}:{:02}  @ {:.2} {}",
            percent,
            elapsed.as_secs() / 60,
            elapsed.as_secs() % 60,
            eta.as_secs() / 60,
            eta.as_secs() % 60,
            disp_speed,
            unit,
        );

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
