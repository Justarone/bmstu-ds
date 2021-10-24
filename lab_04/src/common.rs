use clap::{App, Arg, ArgMatches};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::os::unix::fs::OpenOptionsExt;

pub(crate) fn write_keys<'a>(matches: &'a ArgMatches, (e, d, n): (u128, u128, u128)) {
    let filename = get_output_file(&matches);
    let mut pub_file =
        create_file(&format!("{}.pub", filename)).expect("Can't create file for public key");
    let mut private_file = create_file(filename).expect("Can't create file for private key");
    pub_file
        .write(format!("{} {}", e, n).as_bytes())
        .expect("Can't write public key");
    private_file
        .write(format!("{} {}", d, n).as_bytes())
        .expect("Can't write private key");
}

pub(crate) fn read_key<'a>(matches: &'a ArgMatches) -> (u128, u128) {
    let filename = get_key(matches);
    let mut file = open_file(filename).expect("Can't open key file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Can't read key file");
    let res: Vec<_> = content
        .trim()
        .split_whitespace()
        .take(2)
        .map(|val| val.parse().expect("Can't parse key part"))
        .collect();
    (res[0], res[1])
}

pub(crate) fn open_file(input_file: &str) -> Result<BufReader<File>, Box<dyn Error>> {
    let f = File::open(input_file)?;
    Ok(BufReader::new(f))
}

pub(crate) fn create_file(output_file: &str) -> Result<BufWriter<File>, Box<dyn Error>> {
    let f = OpenOptions::new()
        .mode(0o660)
        .create(true)
        .write(true)
        .truncate(true)
        .open(output_file)?;
    Ok(BufWriter::new(f))
}

pub(crate) fn input_from_file(mut file: BufReader<File>) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut res = Vec::with_capacity(256);
    file.read_to_end(&mut res)?;
    Ok(res)
}

pub(crate) fn get_input_file<'a>(matches: &'a ArgMatches) -> &'a str {
    matches.value_of("input_file").expect("no input file")
}

pub(crate) fn get_output_file<'a>(matches: &'a ArgMatches) -> &'a str {
    matches.value_of("output_file").expect("no output file")
}

pub(crate) fn get_key<'a>(matches: &'a ArgMatches) -> &'a str {
    matches.value_of("key").expect("no key file provided")
}

pub(crate) fn get_key_flag<'a>(matches: &'a ArgMatches) -> bool {
    matches.is_present("generate_keys")
}

pub(crate) fn get_decode_flag<'a>(matches: &'a ArgMatches) -> bool {
    matches.is_present("decode")
}

pub(crate) fn generate_matches<'a>() -> ArgMatches<'a> {
    App::new("RSA")
        .version("1.0")
        .author("Pavel Perestoronin")
        .about("simple implementation of RSA algorithm")
        .arg(
            Arg::with_name("input_file")
                .short("i")
                .long("input")
                .help("input file to read data")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("key")
                .short("k")
                .long("key")
                .help("file with key")
                .default_value("id_rsa")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("generate_keys")
                .long("generate_keys")
                .help("mode to generate keys"),
        )
        .arg(
            Arg::with_name("decode")
                .long("decode")
                .help("decode mode (it's important for alignments)"),
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
