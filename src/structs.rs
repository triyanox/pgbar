use termion::color::Color;

pub struct Style<'a> {
    /// This is the symbol that will be used to fill the progress bar
    pub symbol: String,
    /// This is the color of the progress bar
    ///
    /// The colors are defined in the `termion` crate
    pub color: Option<&'a dyn Color>,
    /// This is the width of the progress bar
    pub width: u32,
    /// This is the time to finish the progress bar
    pub time_to_finish: bool,
    /// this the wrapper symbol defaults to `[]` should be two chars long
    pub wrapper: String,
}

/// This is the progress bar struct
///
/// # Usage
///
/// ```rust
/// use pgbar::{ProgressBar, Style};
/// use termion::color;
///
/// let mut progress_bar = ProgressBar::new(100, 10, Style {
///    symbol: String::from("*"), // The progress bar will be filled with *
///    color: Some(&color::Cyan), // This is the color of the progress bar
///    width: 50, // This is the width of the progress bar
///    time_to_finish: true, // This is the time to finish the progress bar
///    wrapper: String::from("[]"), // This is the wrapper of the progress bar
/// });
///
/// progress_bar.track(); // This will start the progress bar
/// progress_bar.update(50); // This will update the progress bar
/// progress_bar.finish(); // This will finish the progress bar
/// progress_bar.recover(); // This will recover the progress bar
/// progress_bar.reset(); // This will reset the progress bar to 0
/// ```
pub struct ProgressBar<'a> {
    /// The current progress
    pub current: u32,
    /// The max progress
    pub max: u32,
    /// The max time to finish the progress bar in seconds
    pub max_time: u32,
    /// The style of the progress bar defined in the `Style` struct
    pub style: Style<'a>,
}
