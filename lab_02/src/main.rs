use common::{generate_matches, get_input, get_seed, pretty_print};
use enigma::Enigma;

mod common;
mod enigma;

fn main() {
    let matches = generate_matches();
    let input = get_input(&matches);
    let seed = get_seed(&matches);

    let mut enigma = Enigma::with_seed(seed);
    let output = enigma.process(&input);
    enigma.reset();
    let input_from_output = enigma.process(&output);

    assert_eq!(input, input_from_output);
    pretty_print(&input, &output);
}
