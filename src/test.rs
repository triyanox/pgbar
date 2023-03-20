#[cfg(test)]
mod tests {
    use crate::{ProgressBar, Style};
    use termion::color;
    #[test]
    fn progress_bar() {
        let pb = ProgressBar::new(
            100,
            10,
            Style {
                symbol: String::from("*"),
                color: Some(&color::Cyan),
                width: 50,
                time_to_finish: true,
                wrapper: String::from("[]"),
            },
        );
        assert_eq!(pb.current, 0);
        assert_eq!(pb.max, 100);
        assert_eq!(pb.max_time, 10);
        assert_eq!(pb.style.symbol, "*");
        assert_eq!(pb.style.width, 50);
        assert_eq!(pb.style.time_to_finish, true);
        assert_eq!(pb.style.wrapper, "[]");
    }
    #[test]
    fn progress_bar_update() {
        let mut pb = ProgressBar::new(
            100,
            10,
            Style {
                symbol: String::from("*"),
                color: Some(&color::Cyan),
                width: 50,
                time_to_finish: true,
                wrapper: String::from("[]"),
            },
        );
        pb.update(50);
        assert_eq!(pb.current, 50);
    }
}
