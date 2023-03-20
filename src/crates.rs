use crate::structs::{ProgressBar, Style};
use core::time;
use std::{
    io::{self, Write},
    thread,
    time::Instant,
};
use termion::color::{self, Color};

impl ProgressBar<'_> {
    /// This is the constructor of the progress bar
    pub fn new(max: u32, max_time: u32, style: Style) -> ProgressBar {
        let mut progress_bar = ProgressBar {
            current: 0,
            max,
            max_time,
            style,
        };
        if progress_bar.style.symbol.is_empty() {
            progress_bar.style.symbol = "*".to_string();
        }
        if progress_bar.style.width == 0 {
            progress_bar.style.width = 50;
        }
        if progress_bar.style.wrapper.len() != 2 {
            progress_bar.style.wrapper = "[]".to_string();
        }
        if progress_bar.style.color.is_none() {
            progress_bar.style.color = Some(&color::Cyan);
        }
        progress_bar
    }

    fn calc_total_ocupied(&self) -> u32 {
        let percent = (self.current as f32 / self.max as f32) * 100.0;
        let ocupied = (percent / 100.0) * self.style.width as f32;
        ocupied as u32
    }

    fn calc_step(&self) -> u32 {
        let step = (self.max_time * 1000) / self.max;
        std::cmp::max(step, 1)
    }

    fn get_color(&self) -> color::Fg<&dyn Color> {
        color::Fg(self.style.color.unwrap())
    }

    fn calc_time_to_finish(&self) -> String {
        let now = Instant::now();
        let elapsed = now.elapsed().as_secs();
        let remaining = self.max_time - elapsed as u32;
        let minutes = remaining / 60;
        let seconds = remaining % 60;
        format!("{}m {}s", minutes, seconds)
    }

    fn get_wrraper(&self) -> (String, String) {
        let mut wrapper = self.style.wrapper.chars();
        let start = wrapper.next().unwrap();
        let end = wrapper.next().unwrap();
        (start.to_string(), end.to_string())
    }

    fn print(&self) {
        let progress = self.current as f32 / self.max as f32;
        let ocupied = self.calc_total_ocupied();
        let width = self.style.width - ocupied;
        let mut bar = String::new();
        let mut i = 0;
        while i < ocupied {
            bar.push_str(&self.style.symbol);
            i += 1;
        }
        i = 0;
        while i < width {
            bar.push(' ');
            i += 1;
        }
        let color = self.get_color();
        match self.style.time_to_finish {
            true => {
                let remaining = self.calc_time_to_finish();
                print!(
                    "\r{}{}{}{}{}{}{}{}{}{}",
                    self.get_wrraper().0,
                    color,
                    bar,
                    color::Fg(color::Reset),
                    self.get_wrraper().1,
                    " ",
                    (progress * 100.0) as u32,
                    "%",
                    " ",
                    remaining
                );
            }
            false => {
                print!(
                    "\r{}{}{}{}{}{}{}{}",
                    self.get_wrraper().0,
                    color,
                    bar,
                    color::Fg(color::Reset),
                    self.get_wrraper().1,
                    " ",
                    (progress * 100.0) as u32,
                    "%",
                );
            }
        }
        io::stdout().flush().unwrap();
    }

    /// [track]
    ///
    /// This method is used to start the progress bar
    pub fn track(&mut self) {
        let step = self.calc_step();
        while self.current < self.max {
            self.current += 1;
            self.print();
            thread::sleep(time::Duration::from_millis(step.into()));
        }
    }

    /// [finish]
    ///
    /// This method is used to finish the progress bar
    pub fn finish(&mut self) {
        self.update(self.max);
        println!();
    }

    /// [update]
    /// This method is used to update the progress bar
    pub fn update(&mut self, current: u32) {
        self.current = current;
        self.print();
    }

    /// [reset]
    ///
    /// This method is used to reset the progress bar
    pub fn reset(&mut self) {
        self.current = 0;
        self.print();
    }

    /// [recover]
    ///
    /// This method is used to recover the progress bar
    pub fn recover(&mut self) {
        self.print();
    }
}
