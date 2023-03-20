# pgbar - A minimal progress bar written in rust 🦀

[![Crates.io](https://img.shields.io/crates/v/pgbar.svg)](https://crates.io/crates/pgbar)
[![Docs.rs](https://docs.rs/pgbar/badge.svg)](https://docs.rs/pgbar)

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
pgbar = "*"
```

or install it with `cargo add pgbar`.

## Usage

```rust
use pgbar::ProgressBar;
use extern crate termion::color; // pgbar uses `termion` to color the progress bar

fn main(){
    let mut progress_bar = ProgressBar::new(100, 10, Style {
        symbol: String::from("*"), // The progress bar will be filled with *
        color: Some(&color::Cyan), // This is the color of the progress bar
        width: 50, // This is the width of the progress bar
        time_to_finish: true, // This is the time to finish the progress bar
        wrapper: String::from("[]"), // This is the wrapper of the progress bar
    });
    ///
    progress_bar.track(); // This will start the progress bar
    progress_bar.update(50); // This will update the progress bar
    progress_bar.finish(); // This will finish the progress bar
    progress_bar.recover(); // This will recover the progress bar
    progress_bar.reset(); // This will reset the progress bar to 0
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
