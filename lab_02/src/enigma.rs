mod reflector;
mod rotor;

use rand::prelude::*;
use reflector::Reflector;
use rotor::Rotor;

pub(crate) struct Enigma {
    rotors: Vec<Rotor>,
    reflector: Reflector,
}

impl Enigma {
    pub(crate) fn with_seed(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let rotors = (0..3).map(|_| Rotor::new(&mut rng)).collect();
        let reflector = Reflector::new(&mut rng);
        Self { rotors, reflector }
    }

    pub(crate) fn process(&mut self, input: &[u8]) -> Vec<u8> {
        input
            .iter()
            .copied()
            .map(|mut val| {
                self.rotors.iter().for_each(|r| val = r.get(val));
                val = self.reflector.get(val);
                self.rotors.iter().rev().for_each(|r| val = r.get_inv(val));
                self.roll_all();
                val
            })
            .collect()
    }

    pub(crate) fn reset(&mut self) {
        self.rotors.iter_mut().for_each(|r| r.reset());
    }

    fn roll_all(&mut self) {
        for r in self.rotors.iter_mut() {
            if !r.roll() {
                break;
            }
        }
    }
}
