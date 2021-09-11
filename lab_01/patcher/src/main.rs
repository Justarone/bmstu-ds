use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::os::unix::fs::OpenOptionsExt;

use clap::{App, Arg, ArgMatches};
use sha2::{Digest, Sha256};

fn generate_matches<'a>() -> ArgMatches<'a> {
    App::new("Patcher")
        .version("1.0")
        .author("Pavel Perestoronin")
        .about("patches program for lab 1")
        .arg(
            Arg::with_name("output_file")
                .short("o")
                .long("output")
                .help("sets output file")
                .default_value("a.out")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("program")
                .short("i")
                .long("program")
                .help("sets input program")
                .required(true)
                .takes_value(true),
        )
        .get_matches()
}

fn get_spec_info() -> Result<Vec<u8>, Box<dyn Error>> {
    // man machine-id
    let mut f = File::open("/etc/machine-id")?;
    let mut buffer = Vec::with_capacity(32);
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn get_files(
    input_file: &str,
    output_file: &str,
) -> Result<(BufReader<File>, BufWriter<File>), Box<dyn Error>> {
    let f = File::open(input_file)?;
    let reader = BufReader::new(f);
    let f = OpenOptions::new()
        .mode(0o111)
        .create(true)
        .read(false)
        .write(true)
        .truncate(true)
        .open(output_file)?;
    let writer = BufWriter::new(f);
    Ok((reader, writer))
}

fn patch(mut reader: BufReader<File>, mut writer: BufWriter<File>) -> Result<(), Box<dyn Error>> {
    let pattern = ['=' as u8; 32];

    let specific_info = get_spec_info()?;
    let mut hasher = Sha256::new();
    hasher.update(specific_info);
    let hash = hasher.finalize();

    let mut buffer = Vec::with_capacity(256);
    reader.read_to_end(&mut buffer)?;

    let mut pos = usize::MAX;
    for (i, subslice) in buffer.windows(pattern.len()).enumerate() {
        if subslice == &pattern {
            pos = i;
            break;
        }
    }
    assert_ne!(pos, usize::MAX, "bad input file");
    let (_rest, mut buf) = buffer.split_at_mut(pos);
    buf.write(&hash)?;

    writer.write(&buffer)?;
    Ok(())
}

fn main() {
    let matches = generate_matches();
    let input_file = matches.value_of("program").unwrap();
    let output_file = matches.value_of("output_file").unwrap();
    let (reader, writer) = get_files(input_file, output_file).expect("Something wrong with files");
    patch(reader, writer).expect("Can't create patched program");
}
