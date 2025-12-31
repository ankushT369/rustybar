//bar.rs is the configuration for the process bar
use std::io::Write;
use std::io;
use std::time::{Instant, Duration};

const UNICODE_BAR_FULL_CHARS: &[char] = &['█', '#', '='];
const UNICODE_BAR_EMPTY_CHARS: &[char] = &[' ', '-'];

pub enum FillStyle { Solid, Hash, Equal }
pub enum EmptyStyle { Space, Dash }

impl FillStyle {
    pub fn ch(self) -> char {
        UNICODE_BAR_FULL_CHARS[self as usize]
    }
}

impl EmptyStyle {
    pub fn ch(self) -> char {
        UNICODE_BAR_EMPTY_CHARS[self as usize]
    }
}

pub struct ProgressBar {
    desc: String,
    len: usize,
    size: usize,
    fill_style: char,
    empty_style: char,
    unit: String,
    curr: usize,

    start_time: Instant,
}

impl ProgressBar {
    pub fn new(desc: &str, len: usize, size: usize, unit: &str) -> Self {
        Self {
            desc: desc.to_string(),
            len,
            size,
            fill_style: FillStyle::Hash.ch(),
            empty_style: EmptyStyle::Dash.ch(),
            unit: unit.to_string(),
            curr: 0,
            start_time: Instant::now(),
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

        print!("\r{}", self.desc);
        io::stdout().flush().unwrap();

        print!("[");
        for _ in 0..self.curr {
            print!("{}", self.fill_style);
        }
        for _ in self.curr..self.len {
            print!("{}", self.empty_style);
        }
        print!("] ");

        println!(
            "{}%  elapsed {:02}:{:02}  <  ETA {:02}:{:02}  @ {:.2} KB/s",
            percent,
            elapsed.as_secs() / 60,
            elapsed.as_secs() % 60,
            eta.as_secs() / 60,
            eta.as_secs() % 60,
            speed / 1024.0,
        );
    }

    pub fn style(&mut self, fill: FillStyle, emp: EmptyStyle) {
        self.fill_style = fill.ch();
        self.empty_style = emp.ch();
    }
}
