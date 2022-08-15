#[macro_export]
macro_rules! print_banner {
    () => {
        use ansi_term::Color;
        println!(
            r"
 .osintb.
ug-b  ount
yrec.
 'onsca.     {}
    'nnin.
      'gin   {}
fose  csec   {}
 'urity?'
            ",
            Color::Red.bold().paint("stick v1.0.0"),
            Color::Green.bold().paint("https://traumatism.github.io"),
            Color::Green.bold().paint("https://toast.移动")
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
        use ansi_term::Color::Green;
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
