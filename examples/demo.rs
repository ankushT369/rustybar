use rustybar::*;

use std::{thread, time::Duration};

fn main() {
    clear_screen().expect("Failed to clear screan");
    hide_cursor().expect("Failed to hide cursor");
    let total_size = 50_000;

    let mut bar = ProgressBar::new("Downloading bar 1", 40, total_size);

    let mut downloaded = 0;

    while downloaded < total_size {
        downloaded += 700;
        if downloaded > total_size {
            downloaded = total_size;
        }

        bar.tick(downloaded);
        thread::sleep(Duration::from_millis(80));
    }

    println!();
    show_cursor().unwrap();
}
