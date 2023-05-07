use libc::winsize;

use crate::{color::Color, print_container};

pub fn print_version(size: &winsize) {
    print_container(
        "kkit ",
        vec![vec![
            (1, String::from("\u{f0f4}"), Color::Yellow),
            (
                1,
                format!("{}", option_env!("CARGO_PKG_VERSION").unwrap_or("???")),
                Color::White,
            ),
        ]],
        "",
        Color::Cyan,
        Color::White,
        Color::White,
        size,
    )
}
