#[macro_export]
macro_rules! print_banner {
    () => {
        println!(
            "{}",
            r"

 .osintb.  
ug-b  ount 
yrec.      
 'onsca.     stick v1.0.0
    'nnin.   
      'gin   https://traumatism.github.io
fose  csec   https://toast.移动
 'urity?'

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
