use std::io::{self, Write};
use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
pub enum ProgressStyle {
    Ascii,
    Unicode,
    Spinner,
}

#[derive(Clone, Copy)]
pub enum Verbosity {
    Minimal,
    Detailed,
}

pub struct ProgressBar {
    total: usize,
    current: usize,
    start: Instant,
    last_update: Instant,
    style: ProgressStyle,
    verbosity: Verbosity,
    spinner_index: usize,
    spinner_frames: &'static [&'static str],
}

impl ProgressBar {
    pub fn new(total: usize, style: ProgressStyle, verbosity: Verbosity) -> Self {
        ProgressBar {
            total,
            current: 0,
            start: Instant::now(),
            last_update: Instant::now(),
            style,
            verbosity,
            spinner_index: 0,
            spinner_frames: &["-", "\\", "|", "/"],
        }
    }

    pub fn inc(&mut self, n: usize) {
        self.current += n;
        self.last_update = Instant::now();
        self.draw();
    }

    pub fn finish(&mut self) {
        self.current = self.total;
        self.draw();
        println!();
    }

    fn draw(&mut self) {
        let percent = self.current as f64 / self.total as f64;
        let elapsed = self.start.elapsed().as_secs_f64();
        let patterns_per_sec = if elapsed > 0.0 {
            self.current as f64 / elapsed
        } else {
            0.0
        };
        let eta = if self.current > 0 {
            (self.total - self.current) as f64 / patterns_per_sec
        } else {
            0.0
        };

        let color = if eta > 1.0 {
            "\x1b[31m" // Red
        } else if eta > 0.5 {
            "\x1b[33m" // Yellow
        } else {
            "\x1b[32m" // Green
        };

        let reset = "\x1b[0m";

        let bar = match self.style {
            ProgressStyle::Ascii => {
                let width = 20;
                let filled = (percent * width as f64).round() as usize;
                let empty = width - filled;
                format!(
                    "[{}{}]",
                    "=".repeat(filled),
                    " ".repeat(empty)
                )
            }
            ProgressStyle::Unicode => {
                let width = 20;
                let blocks = [" ", "▏", "▎", "▍", "▌", "▋", "▊", "▉", "█"];
                let filled = (percent * width as f64).floor() as usize;
                let remainder = ((percent * width as f64) - filled as f64) * 8.0;
                let mut bar = String::from("[");
                bar.push_str(&"█".repeat(filled));
                if filled < width {
                    bar.push_str(blocks[remainder as usize]);
                    bar.push_str(&" ".repeat(width - filled - 1));
                }
                bar.push(']');
                bar
            }
            ProgressStyle::Spinner => {
                let frame = self.spinner_frames[self.spinner_index % self.spinner_frames.len()];
                self.spinner_index += 1;
                format!("[{}]", frame)
            }
        };

        let minimal = format!("{} {} {}/{} ({:.0}%){}", color, bar, self.current, self.total, percent * 100.0, reset);

        let detailed = format!(
            "{} {} {}/{} ({:.0}%) @ {:.1}K patterns/sec, ETA: {:.1}s{}",
            color,
            bar,
            self.current,
            self.total,
            percent * 100.0,
            patterns_per_sec / 1000.0,
            eta,
            reset
        );

        let output = match self.verbosity {
            Verbosity::Minimal => minimal,
            Verbosity::Detailed => detailed,
        };

        print!("\r{}", output);
        io::stdout().flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dictionary::FIRST_DICT;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_progress_bar_with_dictionary() {
        let total = FIRST_DICT.len();
        let mut pb = ProgressBar::new(total, ProgressStyle::Unicode, Verbosity::Detailed);

        for key in FIRST_DICT.keys() {
            let _ = FIRST_DICT.get(key);
            pb.inc(1);
            thread::sleep(Duration::from_millis(5));
        }
        pb.finish();
    }
}