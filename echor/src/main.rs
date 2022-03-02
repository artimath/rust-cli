use clap::{App, Arg};

// Continue at page 27 in Command Line Rust

fn main() {
    let matches = App::new("echor")
        .version("1.0")
        .author("Ryan Hunter")
        .about("Echo written in rust")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print new line")
                .takes_value(false),
        )
        .get_matches();

    let omit_newline = matches.is_present("omit_newline");

    let text = matches.values_of_lossy("text").unwrap();

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
