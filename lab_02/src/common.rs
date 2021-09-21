use clap::{App, Arg, ArgMatches};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::os::unix::fs::OpenOptionsExt;

pub(crate) fn get_files(
    input_file: &str,
    output_file: &str,
) -> Result<(BufReader<File>, BufWriter<File>), Box<dyn Error>> {
    let f = File::open(input_file)?;
    let reader = BufReader::new(f);
    let f = OpenOptions::new()
        .mode(0o777)
        .create(true)
        .read(false)
        .write(true)
        .truncate(true)
        .open(output_file)?;
    let writer = BufWriter::new(f);
    Ok((reader, writer))
}

pub(crate) fn input_from_file(mut file: BufReader<File>) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut res = Vec::with_capacity(256);
    file.read_to_end(&mut res)?;
    Ok(res)
}

pub(crate) fn get_filenames<'a>(matches: &'a ArgMatches) -> (&'a str, &'a str) {
    let input_filename = matches
        .value_of("input_file")
        .expect("no input file provided");
    let output_filename = matches
        .value_of("output_file")
        .expect("no output file provided");
    (input_filename, output_filename)
}

pub(crate) fn get_seed(matches: &ArgMatches) -> u64 {
    matches
        .value_of("seed")
        .expect("no seed provided")
        .parse()
        .expect("seed should be a number")
}

pub(crate) fn generate_matches<'a>() -> ArgMatches<'a> {
    App::new("Enigma")
        .version("1.0")
        .author("Pavel Perestoronin")
        .about("simulates enigma")
        .arg(
            Arg::with_name("input_file")
                .short("i")
                .long("input")
                .help("input file to read data")
                .required(true)
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
        .arg(
            Arg::with_name("output_file")
                .short("o")
                .long("output")
                .help("sets output file")
                .required(true)
                .default_value("a.out")
                .takes_value(true),
        )
        .get_matches()
}
