use rustybar::*;
use std::io::Read;

use std::{thread, time::Duration};

fn main() {
    let total_size = 50_000;

    let mut bar = ProgressBar::new("Downloading bar 1", 40, total_size);

    let mut downloaded = 0;


    while downloaded < total_size {
        // You can add more sofisticated ways to halt the program, but this works.
        if ctrl_c() {
            panic!()
        }

        downloaded += 700;
        if downloaded > total_size {
            downloaded = total_size;
        }

        bar.tick(downloaded);
        thread::sleep(Duration::from_millis(80));
    }
}
