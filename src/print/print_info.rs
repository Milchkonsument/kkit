use std::collections::HashMap;

use libc::winsize;

use crate::{color::Color, print_container};

pub fn print_info(info: HashMap<&str, (&str, Option<HashMap<&str, &str>>)>, size: &winsize) {
    print_container(
        "info ",
        info.iter()
            .map(|(k, v)| {
                vec![
                    (1, String::from("\u{eaab}"), Color::White),
                    (1, String::from(*k), Color::Grey),
                    (3, String::from(v.0), Color::Grey),
                ]
            })
            .collect(),
        "",
        Color::Cyan,
        Color::White,
        Color::Cyan,
        size,
    )
}

pub fn print_info_for(cmd: &str, info: Option<HashMap<&str, &str>>, size: &winsize) {
    let infostr = format!("info | {} ", cmd);
    print_container(
        if info.is_none() { "" } else { infostr.as_str() },
        info.unwrap_or(HashMap::from([("have a cookie.", "󰆘")]))
            .into_iter()
            .map(|(k, v)| {
                vec![
                    (1, String::from("\u{eaab}"), Color::White),
                    (1, String::from(k), Color::Grey),
                    (4, String::from(v), Color::Grey),
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
