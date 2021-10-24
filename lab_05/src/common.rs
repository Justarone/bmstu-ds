use clap::{App, Arg, ArgMatches};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::os::unix::fs::OpenOptionsExt;

use openssl::pkey::{Private, Public};
use openssl::rsa::Rsa;

pub(crate) fn write_keys<'a>(matches: &'a ArgMatches, public_key: &str, private_key: &str) {
    let filename = get_key_file(&matches);
    let pub_filename = format!("{}.pub", filename);
    let mut pub_file = create_file(&pub_filename).expect("Can't create file for public key");
    let mut private_file = create_file(filename).expect("Can't create file for private key");
    pub_file
        .write(public_key.as_bytes())
        .expect("Can't write public key");
    private_file
        .write(private_key.as_bytes())
        .expect("Can't write private key");
    println!(
        "Generated keys and wrote them to `{}` and `{}`!",
        pub_filename, filename
    );
}

pub(crate) fn read_private_key<'a>(matches: &'a ArgMatches) -> Rsa<Private> {
    let filename = get_key_file(matches);
    let mut file = open_file(filename).expect("Can't open key file");
    let mut content = Vec::new();
    file.read_to_end(&mut content).expect("Can't read key file");
    Rsa::private_key_from_pem(&content).expect("Can't create private component")
}

pub(crate) fn read_public_key<'a>(matches: &'a ArgMatches) -> Rsa<Public> {
    let filename = get_key_file(matches);
    let mut file = open_file(filename).expect("Can't open key file");
    let mut content = Vec::new();
    file.read_to_end(&mut content).expect("Can't read key file");
    Rsa::public_key_from_pem(&content).expect("Can't create public component")
}

pub(crate) fn open_file(input_file: &str) -> Result<BufReader<File>, Box<dyn Error>> {
    let f = File::open(input_file)?;
    Ok(BufReader::new(f))
}

pub(crate) fn create_file(file_to_create: &str) -> Result<BufWriter<File>, Box<dyn Error>> {
    let f = OpenOptions::new()
        .mode(0o660)
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_to_create)?;
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

pub(crate) fn get_sign_file<'a>(matches: &'a ArgMatches) -> &'a str {
    matches.value_of("sign_file").expect("no sign file")
}

pub(crate) fn get_key_file<'a>(matches: &'a ArgMatches) -> &'a str {
    matches.value_of("key").expect("no key file provided")
}

pub(crate) fn get_key_flag<'a>(matches: &'a ArgMatches) -> bool {
    matches.is_present("generate_keys")
}

pub(crate) fn get_verify_flag<'a>(matches: &'a ArgMatches) -> bool {
    matches.is_present("verify")
}

pub(crate) fn generate_matches<'a>() -> ArgMatches<'a> {
    App::new("Signs")
        .version("1.0")
        .author("Pavel Perestoronin")
        .about("Implementation of signs")
        .arg(
            Arg::with_name("sign_file")
                .short("s")
                .long("sign")
                .help("file to store sign")
                .takes_value(true),
        )
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
            Arg::with_name("verify")
                .short("v")
                .long("verify")
                .help("verify mode"),
        )
        .get_matches()
}
