use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct BitVec {
    size: usize,
    data: Vec<u8>,
}

impl BitVec {
    pub(crate) fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    pub(crate) fn push_back(&mut self, elem: u8) {
        if self.data.len() < self.size / 8 + 1 {
            self.data.resize(self.size / 8 + 1, 0);
        }
        self.data[self.size / 8] |= (elem & 1) << (self.size % 8);
        self.size += 1;
    }

    pub(crate) fn get(&self, i: usize) -> u8 {
        self.data[i / 8].overflowing_shr(i as u32 % 8).0 & 1
    }

    pub(crate) fn pop_back(&mut self) -> u8 {
        self.size -= 1;
        let res = self.get(self.size);
        self.data[self.size / 8] &= (1_u8.overflowing_shl(self.size as u32 % 8).0)
            .overflowing_sub(1)
            .0;
        res
    }

    // second value = real range size (may be less u8 in case of end of file)
    pub(crate) fn get_range_u8(&self, from: usize) -> u8 {
        assert!(from + 8 <= self.size);
        let mut res = 0;
        for (i, b) in (from..(from + 8)).map(|pos| self.get(pos)).enumerate() {
            res |= b << i;
        }
        res
    }

    pub(crate) fn get_range(&self, from: usize, size: usize) -> BitVec {
        let mut res = BitVec::new();
        assert!(from + size <= self.size);
        for i in from..(from + size) {
            res.push_back(self.get(i));
        }
        res
    }

    pub(crate) fn len(&self) -> usize {
        self.size
    }

    pub(crate) fn bytes(&self) -> usize {
        self.data.len()
    }

    pub(crate) fn concat(&mut self, other: &BitVec) {
        for i in 0..other.len() {
            self.push_back(other.get(i));
        }
    }

    pub(crate) fn push_u8(&mut self, val: u8) {
        for i in 0..8 {
            self.push_back(val.overflowing_shr(i).0 & 1);
        }
    }
}

impl fmt::Debug for BitVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} bytes, {} bits]", self.data.len(), self.size)?;
        write!(f, "[")?;
        for i in 0..self.size {
            write!(f, "{}", self.get(i))?;
        }
        write!(f, "]")
    }
}

impl fmt::Display for BitVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..self.size {
            write!(f, "{}", self.get(i))?;
        }
        write!(f, "]")
    }
}

impl PartialEq for BitVec {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(&other).is_eq()
    }
}

impl PartialOrd for BitVec {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let c = self.size.cmp(&other.size);
        let res = if !c.is_eq() {
            c
        } else {
            (0..self.size)
                .map(|i| self.get(i).cmp(&other.get(i)))
                .filter(|cmp_res| cmp_res.is_ne())
                .next()
                .unwrap_or(std::cmp::Ordering::Equal)
        };
        Some(res)
    }
}

impl Eq for BitVec {}

impl Ord for BitVec {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
