use std::{cmp::Ordering, path::Path};

use chrono::{DateTime, Utc};
use libc::winsize;

use crate::{color::Color, print_container, print_err};

pub fn print_dir(dir: Option<&String>, hidden: bool, updated: bool, size: &winsize) {
    let path = if dir.is_none() {
        "."
    } else {
        dir.unwrap().as_str()
    };

    let binding = Path::new(path);

    if binding.exists() == false {
        print_err(format!("'{}' does not exist", path).as_str(), size);
        return;
    }

    if binding.is_dir() == false {
        print_err(format!("'{}' is not a directory", path).as_str(), size);
        return;
    }

    let mut list = binding
        .read_dir()
        .unwrap()
        .map(|d| d.unwrap())
        .filter(|d| {
            hidden
                || d.file_name()
                    .into_string()
                    .unwrap_or(String::new())
                    .starts_with(".")
                    == false
        })
        .collect::<Vec<_>>();

    list.sort_by(|a, b| {
        let a_ = a;
        let b_ = b;
        let a = a.metadata().unwrap();
        let b = b.metadata().unwrap();

        if (a.is_dir() && b.is_dir())
            || (a.is_file() && b.is_file())
            || (a.is_symlink() && b.is_symlink())
        {
            return a_.file_name().cmp(&b_.file_name());
        } else if (a.is_dir() && !b.is_dir()) || (a.is_file() && b.is_symlink()) {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    });

    let data = list.iter().map(|d| {
        (
            String::from(if d.metadata().unwrap().is_dir() {
                "\u{eaf7}"
            } else if d.metadata().unwrap().is_file() {
                "\u{ea7b}"
            } else {
                "\u{eb15}"
            }),
            if d.metadata().unwrap().is_dir() {
                Color::Yellow
            } else if d.metadata().unwrap().is_file() {
                Color::White
            } else {
                Color::Grey
            },
            d.file_name().into_string().unwrap(),
            format!("{} B", d.metadata().unwrap().len().to_string()),
            Into::<DateTime<Utc>>::into(d.metadata().unwrap().modified().unwrap())
                .naive_local()
                .format("%b %-d %H:%M:%S"),
        )
    });

    let data = data
        .map(|d| {
            let mut vec = vec![(1, d.0, d.1), (10, d.2, Color::Grey), (2, d.3, Color::Grey)];

            if updated {
                vec.append(&mut vec![(3, format!("{}", d.4), Color::Grey)]);
            }

            vec
        })
        .collect::<Vec<_>>();

    print_container(
        format!("{} ", path).as_str(),
        data,
        "",
        Color::Cyan,
        Color::Grey,
        Color::Grey,
        size,
    )
}
