// mochou-p/dondon/src/utils.rs

use std::fmt::Display;
use std::io::{stdin, stdout, Write as _};
use cpal::SupportedStreamConfig;


pub fn choose<T, D: Display>(
        what:                &str,
    mut options:             Vec   <T>,
    mut default_option:      Option<T>,
        compare:        impl Fn(&T, &T) -> bool,
        get_text:       impl Fn(&T    ) -> D
) -> Option<T> {
    let len = options.len();
    match len {
        0 => {
            eprintln!("\x1b[31merror: \x1b[0mthere is no available {what}");
            return None;
        },
        1 => {
            let option = options.remove(0);
            let text   = get_text(&option);
            println!("\x1b[36m- autoselected \x1b[32m{text}\x1b[36m as the only {what}\x1b[0m\n");

            return Some(option);
        },
        _ => ()
    }

    println!("\x1b[36m- choose the {what}\x1b[0m");

    for (i, option) in options.iter().enumerate() {
        let n    = i+1;
        let pad  = " ".repeat(format!("{len}").len() - format!("{n}").len());
        let text = get_text(option);

        if let Some(ref default) = default_option {
            if compare(option, default) {
                println!("  \x1b[33m{pad}{n})\x1b[0m \x1b[32m{text} (default)\x1b[0m");
            } else {
                println!("  \x1b[33m{pad}{n})\x1b[0m {text}");
            }
        } else {
            println!("  \x1b[33m{pad}{n})\x1b[0m {text}");
        }
    }

    print!("\x1b[34m>\x1b[0m ");
    let _ = stdout().flush();

    let mut choice = String::new();
    let     _      = stdin().read_line(&mut choice);
    let     choice = choice.trim();

    if choice.is_empty() {
        if let Some(default) = default_option.take() {
            println!();
            return Some(default);
        } else {
            eprintln!("\x1b[31merror: \x1b[0mthere is no default {what}");
            return None;
        }
    }

    let Ok(index) = choice.parse::<usize>() else {
        eprintln!("\x1b[31merror: \x1b[0mfailed to parse the input as an unsigned number");
        return None;
    };

    if index < 1 || index > len {
        eprintln!("\x1b[31merror: \x1b[0mthe index {index} is out of range 1..={len}");
        return None;
    }

    println!();
    Some(options.remove(index - 1))
}

pub fn config_text(config: &SupportedStreamConfig) -> String {
    format!(
        "{}x {}Hz {}",
        config.channels(),
        config.sample_rate(),
        config.sample_format()
    )
}

