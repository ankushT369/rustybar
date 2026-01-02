# rustybar
[![crates.io](https://img.shields.io/crates/v/rustybar.svg)](https://crates.io/crates/rustybar)
[![docs.rs](https://docs.rs/rustybar/badge.svg)](https://docs.rs/rustybar)


**rustybar** is a small, dependency-light progress bar library for Rust that renders clean, modern-looking progress bars in the terminal.

It focuses on three things:

* zero setup
* fully customizable appearance
* smooth real-time feedback for CLI tools

If you are writing downloaders, installers — this is made for that.



## Installation

Add it to your project with:

```sh
cargo add rustybar
```


## Quick Example

```rust
use rustybar::ProgressBar;
use std::{thread, time::Duration};

fn main() {
    let total = 20_000;
    let mut bar = ProgressBar::new("Downloading", 40, total);

    let mut done = 0;
    while done < total {
        done += 500;
        bar.tick(done);
        thread::sleep(Duration::from_millis(80));
    }
}
```

Run:

```sh
cargo run
```

Run Demo:
```sh
cargo run --example demo
```

## Preview

Below are real screenshots generated using rustybar while downloading a file.

<p align="center"> <img src="https://raw.githubusercontent.com/ankushT369/rustybar/main/assets/sample1.png" width="700"/> <br/> <img src="https://raw.githubusercontent.com/ankushT369/rustybar/main/assets/sample2.png" width="700"/> <br/> <img src="https://raw.githubusercontent.com/ankushT369/rustybar/main/assets/sample3.png" width="700"/> </p>

## Custom Styling

You can change the bar style:

```rust
bar.style(FillStyle::Solid, EmptyStyle::Dash);
```

Change colors:

```rust
bar.color(Color::Cyan, Color::Gray);
```

## License

Dual-licensed under:

* Apache 2.0
* MIT

Use it freely in personal and commercial projects.
