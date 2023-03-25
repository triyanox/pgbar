# pgbar - A minimal progress bar written in rust 🦀

[![Crates.io](https://img.shields.io/crates/v/pgbar.svg)](https://crates.io/crates/pgbar)
![Rust CI](https://github.com/triyanox/pgbar/workflows/Rust%20CI/badge.svg)

This is a Rust library to create progress bars on terminal. It provides an easy way to track the progress of some task, allowing the user to customize its appearance and behavior.

## 🎁 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pgbar = "*"
```

## 🧰 Usage

To use the library, first import it on your Rust project:

```rust
use pgbar::{ProgressBar, Style};
```

Then, create a new progress bar object with the desired configuration using the `new()` method:

```rust
let mut pb = ProgressBar::new(100, 10, Style::default());
```

The first parameter represents the maximum value of the progress bar, the second one is the estimated time to complete the task, and the third defines the style of the bar. You can customize it by creating a new `Style` struct with your preferred options.

After that, you can start tracking the progress of your task by calling the `track()` method:

```rust
pb.track();
```

This will update the progress bar until it reaches the maximum value defined.

You can also update, reset or recover the progress bar using the methods `update()`, `reset()` and `recover()`, respectively.

```rust
pb.update(50);
pb.reset();
pb.recover();
```

Finally, to finish the progress bar, call the `finish()` method:

```rust
pb.finish();
```

## 🎨 Customization

The progress bar can be customized with the following options:

- `symbol`: the character used to represent the progress of the task (default: `"*"`).
- `width`: the width of the progress bar in characters (default: `50`).
- `wrapper`: the characters used to wrap the progress bar (default: `"[]"`).
- `color`: the color of the progress bar (default: `Cyan`).
- `time_to_finish`: whether or not to show the estimated time to finish the task (default: `true`).

To customize these options, create a new `Style` struct with your preferred configuration and pass it as a parameter to the `new()` method:

```rust
use termion::color;

let style = Style {
    symbol: "#".to_string(),
    width: 30,
    wrapper: "()".to_string(),
    color: Some(&color::Red),
    time_to_finish: false,
};
let mut pb = ProgressBar::new(100, 10, style);
```

## 🚀 Example

Here is a simple example showing how to use the progress bar library:

```rust
use pgbar::{ProgressBar, Style};
use std::thread;
use std::time::Duration;
use termion::color;


fn main() {
    let style = Style {
        symbol: "=".to_string(),
        width: 50,
        wrapper: "||".to_string(),
        color: Some(&color::Green),
        time_to_finish: true,
    };
    let mut pb = ProgressBar::new(100, 5, style);

    for i in 0..=100 {
        pb.update(i);
        thread::sleep(Duration::from_millis(50));
    }

    pb.finish();
}
```

This will create a progress bar with green color, the symbol "=" and the wrapper "||", updating its progress every 50 milliseconds until it reaches the maximum value of 100.
