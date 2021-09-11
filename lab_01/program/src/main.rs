use sha2::{Digest, Sha256};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

const SECRET: [u8; 32] = ['=' as u8; 32];

fn main_old() {
    println!("Full access to program.");
}

fn get_spec_info() -> Result<Vec<u8>, Box<dyn Error>> {
    // man machine-id
    let mut f = File::open("/etc/machine-id")?;
    let mut buffer = Vec::with_capacity(256);
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn check() -> bool {
    let specific_info = get_spec_info().expect("Failed to collect specific info.");

    let mut hasher = Sha256::new();
    hasher.update(specific_info);
    let hash = hasher.finalize();

    SECRET == hash.as_slice()
}

fn main() {
    if check() {
        main_old();
    } else {
        println!("Program is not patched or patched incorrectly.");
    }
}
