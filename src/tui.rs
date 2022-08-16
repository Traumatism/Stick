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
        println!(
            "{} {}",
            ansi_term::Color::Blue.bold().paint("[~]"),
            ansi_term::Color::White.bold().paint($content)
        )
    };
}

#[macro_export]
macro_rules! ok {
    ($content:expr) => {
        println!(
            "{} {}",
            ansi_term::Color::Green.bold().paint("[+]"),
            ansi_term::Color::White.bold().paint($content)
        )
    };
}

#[macro_export]
macro_rules! error {
    ($content:expr) => {
        println!(
            "{} {}",
            ansi_term::Color::Red.bold().paint("[-]"),
            ansi_term::Color::White.bold().paint($content)
        )
    };
}
