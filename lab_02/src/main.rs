use common::{generate_matches, get_filenames, get_files, get_seed, input_from_file};
use enigma::Enigma;
use std::io::prelude::*;

mod common;
mod enigma;

mod tests;

fn main() {
    let matches = generate_matches();
    let (input_filename, output_filename) = get_filenames(&matches);
    let (input_file, mut output_file) =
        get_files(input_filename, output_filename).expect("Problems with files..");

    let input = input_from_file(input_file).expect("Can't read file");
    let seed = get_seed(&matches);

    let mut enigma = Enigma::with_seed(seed);
    let output = enigma.process(&input);

    output_file
        .write_all(&output)
        .expect("Can't write output to file");
}
