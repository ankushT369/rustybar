use rustybar::*;

use std::{thread, time::Duration};

fn main() {
    let total_size = 50_000;

    let mut bar = ProgressBar::new("Downloading bar 1", 40, total_size);
    bar.style(FillStyle::Solid, EmptyStyle::Solid);
    bar.gradient(Color::Yellow, Color::Red);

    let mut downloaded = 0;

    while downloaded < total_size {
        downloaded += 700;
        if downloaded > total_size {
            downloaded = total_size;
        }

        bar.tick(downloaded);
        thread::sleep(Duration::from_millis(80));
    }
}
