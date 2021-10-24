use openssl::rsa::{Padding, Rsa};
use openssl::sha::sha256;
use std::io::prelude::*;

mod common;

fn main() {
    let matches = common::generate_matches();
    if common::get_key_flag(&matches) {
        let rsa = Rsa::generate(2048).unwrap();
        let public_key = String::from_utf8(rsa.public_key_to_pem().unwrap()).unwrap();
        let private_key = String::from_utf8(rsa.private_key_to_pem().unwrap()).unwrap();
        common::write_keys(&matches, &public_key, &private_key);
    } else if common::get_verify_flag(&matches) {
        let sign_filename = common::get_sign_file(&matches);
        let input_filename = common::get_input_file(&matches);
        let sfile = common::open_file(sign_filename).expect("Can't open sign file");
        let ifile = common::open_file(input_filename).expect("Can't open input file");
        let sign = common::input_from_file(sfile).expect("Can't read sign from file");
        let input = common::input_from_file(ifile).expect("Can't read input file");

        let hash = sha256(&input);

        let rsa = common::read_public_key(&matches);

        let mut output = vec![0; sign.len() * 3];
        let bytes_read = rsa
            .public_decrypt(&sign, &mut output, Padding::PKCS1)
            .unwrap() as usize;

        let key_filename = common::get_key_file(&matches);
        println!(
            "File: `{}`, sign: `{}`, key: `{}`",
            input_filename, sign_filename, key_filename
        );

        if hash == output[..bytes_read] {
            println!("Verified!");
        } else {
            println!("Something went wrong and sign is bad:(");
        }
    } else {
        let sign_filename = common::get_sign_file(&matches);
        let key_filename = common::get_key_file(&matches);
        let input_filename = common::get_input_file(&matches);

        let mut sfile = common::create_file(sign_filename).expect("Can't create output file");
        let ifile = common::open_file(input_filename).expect("Can't open input file");

        let rsa = common::read_private_key(&matches);
        let input = common::input_from_file(ifile).expect("Can't read input file");
        let hash = sha256(&input);
        let mut output = vec![0; hash.len() * 33];
        let bytes_read = rsa
            .private_encrypt(&hash, &mut output, Padding::PKCS1)
            .unwrap();
        sfile.write(&output[..bytes_read]).unwrap();

        println!(
            "Created sign for `{}` with key from `{}` and saved in `{}`",
            input_filename, key_filename, sign_filename
        )
    }
}
