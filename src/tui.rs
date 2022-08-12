#[macro_export]
macro_rules! print_banner {
    () => {
        println!(
            "{}",
            r"

    Stick - OSINT tool

    twitter.com/t0x0ast

        "
        )
    };
}

#[macro_export]
macro_rules! info {
    ($content:expr) => {
        use ansi_term::Color::{Blue, White};
        println!(
            "{} {}",
            Blue.bold().paint("[~]"),
            White.bold().paint($content)
        )
    };
}

#[macro_export]
macro_rules! ok {
    ($content:expr) => {
        use ansi_term::Color::{Green, White};
        println!(
            "{} {}",
            Green.bold().paint("[+]"),
            White.bold().paint($content)
        )
    };
}

#[macro_export]
macro_rules! error {
    ($content:expr) => {
        use ansi_term::Color::{Red, White};
        println!(
            "{} {}",
            Red.bold().paint("[-]"),
            White.bold().paint($content)
        )
    };
}

pub fn boxify(string: &str) -> String {
    if !string.contains('\n') {
        return format!(
            "+{}+\n| {} |\n+{}+\n",
            &"-".repeat(string.len() + 2),
            &format!("| {} |\n", string),
            &"-".repeat(string.len() + 2)
        );
    }

    let lmax = string
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .max()
        .unwrap()
        .len();

    print!("{}", lmax);

    let mut s = String::new();

    s.push_str(&format!("+{}+\n", "-".repeat(lmax + 2)));

    for line in string.split('\n') {
        s.push_str(&format!(
            "| {}{} |\n",
            line,
            " ".repeat((lmax - line.len()) + 1)
        ));
    }

    s.push_str(&format!("+{}+\n", "-".repeat(lmax + 2)));
    s
}
