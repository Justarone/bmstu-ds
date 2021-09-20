use rand::prelude::*;

pub(crate) struct Rotor {
    map: Vec<u8>,
    map_inv: Vec<u8>,
    offset: usize,
}

impl Rotor {
    pub(crate) fn new(rng: &mut StdRng) -> Self {
        let mut map: Vec<_> = (0..256).map(|i| i as u8).collect();
        map.shuffle(rng);
        let map_inv = Self::build_inv(&map);
        Self {
            map,
            map_inv,
            offset: 0,
        }
    }

    pub(crate) fn get(&self, val: u8) -> u8 {
        let index = (val as usize + self.offset) % 256;
        self.map[index]
    }

    pub(crate) fn get_inv(&self, val: u8) -> u8 {
        let no_offset_val = self.map_inv[val as usize] as usize;
        ((256 + no_offset_val - self.offset) % 256) as u8
    }

    // returns true in case of full circle
    pub(crate) fn roll(&mut self) -> bool {
        if self.offset == 255 {
            self.offset = 0;
            true
        } else {
            self.offset += 1;
            false
        }
    }

    pub(crate) fn reset(&mut self) {
        self.offset = 0;
    }

    fn build_inv(map: &[u8]) -> Vec<u8> {
        let mut res = vec![0; 256];
        map.iter()
            .enumerate()
            .for_each(|(i, val)| res[*val as usize] = i as u8);
        res
    }
}
