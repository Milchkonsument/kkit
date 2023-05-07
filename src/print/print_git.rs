use std::process::Command;

use libc::winsize;

use crate::color::Color;

use super::print::print_container;

pub fn print_git(size: &winsize) {
    let branch = Command::new("git")
        .args(["branch", "-a"])
        .output()
        .expect("failed to execute process");

    let branches = String::from_utf8(branch.stdout)
        .unwrap_or(String::new())
        .split("\n")
        .filter(|s| s != &"")
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();

    let branches = branches
        .iter()
        .filter(|b| b.contains("HEAD") == false)
        .map(|b| {
            *b.split_whitespace()
                .collect::<Vec<_>>()
                .last()
                .unwrap_or(&"")
        })
        .collect::<Vec<_>>();

    let local = branches.iter().filter(|b| b.contains("remote") == false);
    let remote = branches.iter().filter(|b| b.contains("remote"));

    print_container(
        "git | ",
        branches
            .iter()
            .map(|b| {
                vec![
                    (
                        1,
                        String::from("\u{eaab}"),
                        if b.len() == 1 {
                            Color::White
                        } else {
                            Color::Cyan
                        },
                    ),
                    (8, String::from(*b), Color::Grey),
                ]
            })
            .collect::<Vec<_>>(),
        "",
        Color::Cyan,
        Color::Grey,
        Color::Cyan,
        size,
    )
}
