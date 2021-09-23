use rand::prelude::*;

pub(crate) struct Reflector {
    map: Vec<u8>,
}

impl Reflector {
    pub(crate) fn new(rng: &mut StdRng) -> Self {
        let mut tmp: Vec<_> = (0..256).map(|i| i as u8).collect();
        tmp.shuffle(rng);
        let mut map = vec![0u8; 256];
        tmp.chunks(2).for_each(|pair| {
            map[pair[0] as usize] = pair[1];
            map[pair[1] as usize] = pair[0];
        });
        Self { map }
    }

    pub(crate) fn get(&self, val: u8) -> u8 {
        self.map[val as usize]
    }
}
