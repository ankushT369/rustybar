# rustybar

**rustybar** is a small, dependency-light progress bar library for Rust that renders clean, modern-looking progress bars in the terminal.

It focuses on three things:

* zero setup
* fully customizable appearance
* smooth real-time feedback for CLI tools

If you are writing downloaders, installers — this is made for that.



## Installation

Add it to your project with:

```
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

```
cargo run
```


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


