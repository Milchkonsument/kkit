mod color;
mod print;

use std::{collections::HashMap, env};

use color::Color;
use libc::{winsize, STDOUT_FILENO, TIOCGWINSZ};
use print::{
    print::print_container,
    print_dir::print_dir,
    print_git::print_git,
    print_info::{print_info, print_info_for},
    print_version::print_version,
};
use rand::Rng;

type Row = Vec<(u8, String, Color)>;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let size = get_winsize();
    let info = HashMap::<&str, (&str, Option<HashMap<&str, &str>>)>::from([
        ("i", ("print this menu", None)),
        ("i*", ("print info for command", Some(HashMap::from([])))),
        (
            "l",
            (
                "list dir",
                Some(HashMap::from([
                    ("u", "print last updated"),
                    ("h", "show hidden files"),
                ])),
            ),
        ),
        ("v", ("version", None)),
    ]);

    match args.len() {
        1 => print_info(info, &size),
        2.. => {
            let args_arr = args[1]
                .split("")
                .filter(|arg| *arg != "")
                .collect::<Vec<_>>();

            if let Some(s) = args_arr.get(0) {
                match *s {
                    "i" => match args_arr.get(1) {
                        Some(ch) => print_info_for(
                            *ch,
                            if info.get(ch).is_none() {
                                None
                            } else {
                                info.get(ch).unwrap().to_owned().1
                            },
                            &size,
                        ),
                        None => print_info(info, &size),
                    },
                    "l" => print_dir(
                        args.get(2),
                        args_arr.contains(&"h"),
                        args_arr.contains(&"u"),
                        &size,
                    ),
                    "v" => print_version(&size),
                    "g" => print_git(&size),
                    "_" => {
                        for i in 0..10 {
                            let n = rand::thread_rng().gen_range(0..10);
                            let w = rand::thread_rng().gen_range(1..5);
                            let c = vec![rand::thread_rng().gen_range(0..10); n as usize];

                            print_container(
                                format!("{} [{}|{}]", i.to_string(), n.to_string(), w.to_string(),)
                                    .as_str(),
                                vec![vec![
                                    (
                                        1,
                                        vec!["%"; c.len()].iter().fold(String::new(), |a, b| a + b),
                                        Color::Grey
                                    );
                                    n as usize
                                ]],
                                "",
                                Color::Cyan,
                                Color::White,
                                Color::White,
                                &size,
                            )
                        }
                    }
                    _ => print_err(
                        format!("unknown command : '{}'", args_arr[0]).as_str(),
                        &size,
                    ),
                }
            }
        }
        _ => (),
    }
}

fn get_winsize() -> winsize {
    let w = winsize {
        ws_col: 0,
        ws_row: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    unsafe {
        libc::ioctl(STDOUT_FILENO, TIOCGWINSZ, &w);
    }
    w
}

fn print_err(err: &str, size: &winsize) {
    print_container(
        "kkit ",
        vec![vec![(1, String::from(err), Color::Grey)]],
        "",
        Color::Red,
        Color::Red,
        Color::Red,
        size,
    )
}
