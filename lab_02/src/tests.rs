#[test]
fn the_only_test() {
    use super::Enigma;
    let input = "Hello world!\n".as_bytes();
    let mut enigma = Enigma::with_seed(1723489);
    let output = enigma.process(input);
    enigma.reset();
    let input_from_output = enigma.process(&output);
    assert_eq!(input, input_from_output);
}
