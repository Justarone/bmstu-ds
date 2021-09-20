use clap::{App, Arg, ArgMatches};

pub(crate) fn get_input(matches: &ArgMatches) -> Vec<u8> {
    matches
        .value_of("input")
        .expect("no input string provided")
        .as_bytes()
        .to_vec()
}

pub(crate) fn get_seed(matches: &ArgMatches) -> u64 {
    matches
        .value_of("seed")
        .expect("no seed provided")
        .parse()
        .expect("seed should be a number")
}
pub(crate) fn pretty_print(input: &[u8], output: &[u8]) {
    println!("Input value:  {:?}\nOutput value: {:?}", input, output);
}

pub(crate) fn generate_matches<'a>() -> ArgMatches<'a> {
    App::new("Enigma")
        .version("1.0")
        .author("Pavel Perestoronin")
        .about("simulates enigma")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .help("sets input string")
                .default_value("Hello world!")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("seed")
                .short("s")
                .long("seed")
                .help("sets random seed value")
                .default_value("17")
                .takes_value(true),
        )
        .get_matches()
}
