use rand::prelude::*;

pub(crate) struct Reflector {
    map: Vec<u8>,
}

impl Reflector {
    pub(crate) fn new(rng: &mut StdRng) -> Self {
        let mut tmp: Vec<_> = (0..256).map(|i| i as u8).collect();
        tmp.shuffle(rng);
        let mut map = vec![0u8; 256];
        for chunk in tmp.chunks(2) {
            map[chunk[0] as usize] = chunk[1];
            map[chunk[1] as usize] = chunk[0];
        }
        Self { map }
    }

    pub(crate) fn get(&self, val: u8) -> u8 {
        self.map[val as usize]
    }
}
