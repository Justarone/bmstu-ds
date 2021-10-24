use rand::prelude::*;
use std::io::prelude::*;

mod common;
mod math;
mod rsa;

fn main() {
    let matches = common::generate_matches();
    if common::get_key_flag(&matches) {
        let mut rng = thread_rng();
        let (e, d, n) = rsa::generate_keys(&mut rng);
        common::write_keys(&matches, (e, d, n));
    } else {
        let ifile =
            common::open_file(common::get_input_file(&matches)).expect("Can't open input file");
        let mut ofile = common::create_file(common::get_output_file(&matches))
            .expect("Can't create output file");
        let key = common::read_key(&matches);
        let decode = common::get_decode_flag(&matches);
        let mut input = common::input_from_file(ifile).expect("Can't read input file");
        let output = rsa::run(&mut input, key, decode);
        ofile.write(&output).expect("Can't write output");
    }
}
