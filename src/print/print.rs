use std::{cmp::max, collections::HashMap};

use libc::winsize;
use unicode_segmentation::UnicodeSegmentation;

use crate::{color::Color, Row};

pub fn print_container(
    header: &str,
    rows: Vec<Row>,
    footer: &str,
    color: Color,
    header_color: Color,
    footer_color: Color,
    size: &winsize,
) {
    let len = rows.len();
    print_header(
        if len == 0 {
            vec![(1, String::from(header), header_color)]
        } else {
            let mut vec = vec![(1, String::new(), header_color)];
            vec.append(
                &mut rows[0]
                    .iter()
                    .skip(1)
                    .enumerate()
                    .map(|(i, (w, _, _))| {
                        (*w, String::from(if i == 0 { header } else { "" }), color)
                    })
                    .collect::<Vec<_>>(),
            );
            vec
        },
        color,
        size,
    );
    for row in &rows {
        print_space(row.to_owned(), color, size)
    }
    print_footer(
        if len == 0 {
            vec![(1, String::from(footer), footer_color)]
        } else {
            let mut vec = vec![(1, String::from(footer), footer_color)];
            vec.append(
                &mut rows[0]
                    .iter()
                    .skip(1)
                    .map(|(w, _, _)| (*w, String::new(), color))
                    .collect::<Vec<_>>(),
            );
            vec
        },
        color,
        size,
    );
}

fn print_header(str: Row, color: Color, size: &winsize) {
    print(str, "─", "┌", "┬", "┐", color, size, 1, false);
}

fn print_footer(str: Row, color: Color, size: &winsize) {
    print(str, "─", "└", "┴", "┘", color, size, 1, false);
}

fn print_space(str: Row, color: Color, size: &winsize) {
    print(str, " ", "│", "│", "│", color, size, 1, true);
}

fn print(
    row: Row,
    fill: &str,
    start: &str,
    sep: &str,
    end: &str,
    color: Color,
    size: &winsize,
    padding: u8,
    clear: bool,
) {
    let col_ansi = HashMap::<Color, &str>::from_iter([
        (Color::Red, "\u{001b}[31m"),
        (Color::Green, "\u{001b}[32m"),
        (Color::Yellow, "\u{001b}[33m"),
        (Color::Cyan, "\u{001b}[36m"),
        (Color::White, "\u{001b}[37m"),
        (Color::Grey, "\u{001b}[90m"),
        (Color::Reset, "\u{001b}[0m"),
    ]);

    let sum: u8 = row.iter().map(|(w, _, _)| w).sum();
    let space = vec![" "; padding as usize]
        .iter()
        .fold(String::new(), |a, b| a + b);

    let s = row
        .iter()
        .enumerate()
        .map(|(i, (w, s, c))| {
            let char_cnt = if i == 0 {
                3
            } else {
                max(
                    ((size.ws_col as f32 / sum as f32 * *w as f32).ceil() as i32)
                        - (padding as i32 * 2),
                    0,
                )
            };
            let sp = if i == row.len() - 1 { "" } else { sep };

            match char_cnt {
                0 => String::from(""),
                1 => format!(
                    "{}{}{}{}{}",
                    space,
                    col_ansi.get(c).unwrap(),
                    sp,
                    col_ansi.get(&color).unwrap(),
                    space
                ),
                2 => format!(
                    "{}{}~{}{}{}",
                    space,
                    col_ansi.get(c).unwrap(),
                    col_ansi.get(&color).unwrap(),
                    sp,
                    space
                ),
                _ => match char_cnt.cmp(&(s.graphemes(true).count() as i32)) {
                    std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                        format!(
                            "{}{}{}{}{}{}{}",
                            space,
                            col_ansi.get(c).unwrap(),
                            s.get(..char_cnt as usize - 3).unwrap_or("?"),
                            "~",
                            col_ansi.get(&color).unwrap(),
                            space,
                            sp,
                        )
                    }
                    std::cmp::Ordering::Greater => {
                        format!(
                            "{}{}{}{}{}{}{}",
                            space,
                            col_ansi.get(c).unwrap(),
                            s,
                            col_ansi.get(&color).unwrap(),
                            vec![fill; char_cnt as usize - s.graphemes(true).count() - 2]
                                .iter()
                                .fold(String::new(), |a, b| a + b),
                            space,
                            sp
                        )
                    }
                },
            }
        })
        .fold(String::new(), |a, b| a + b.as_str());

    if clear {
        println!(
            "{}{}{}{}{}{}{}",
            col_ansi.get(&color).unwrap(),
            start,
            col_ansi.get(&Color::Grey).unwrap(),
            s,
            col_ansi.get(&color).unwrap(),
            end,
            col_ansi.get(&Color::Reset).unwrap(),
        );
    } else {
        println!(
            "{}{}{}{}{}{}",
            col_ansi.get(&color).unwrap(),
            start,
            s,
            col_ansi.get(&color).unwrap(),
            end,
            col_ansi.get(&Color::Reset).unwrap(),
        );
    }
}
