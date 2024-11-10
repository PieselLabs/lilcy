use core::fmt;
use std::fmt::{Display, Formatter};

pub struct BitSet {
    size: usize,
    values: Vec<u32>
}

impl BitSet {
    pub fn new() -> Self {
        Self {
            size: 0,
            values: Vec::new(),
        }
    }

    fn get_num_batch(index: usize) -> usize {
        return index / 32 + 1;
    }

    fn get_bit_pos(index: usize, num_batch: usize) -> usize {
        return index - (num_batch - 1) * 32;
    }

    pub fn size(&self) -> usize {
        return self.size;
    }

    pub fn set(&mut self, index : usize) {
        let num_batch = Self::get_num_batch(index);
        if num_batch > self.values.len() {
            for _ in 0..(num_batch - self.values.len()) {
                self.values.push(0);
            }
        }
        let bit_pos = Self::get_bit_pos(index, num_batch);
        self.values[num_batch - 1] |= 1 << bit_pos;
        self.size += 1;
    }

    pub fn reset(&mut self, index : usize) {
        assert!(index <= self.values.len() * 32);
        let mut num_batch = Self::get_num_batch(index);
        let bit_pos = Self::get_bit_pos(index, num_batch);
        self.values[num_batch - 1] &= !(1 << bit_pos);
        self.size -= 1;
        while num_batch != 0 && num_batch == self.values.len() && self.values[num_batch - 1] == 0 {
            self.values.pop();
            num_batch -= 1;
        }
    }

    pub fn is_set(&self, index : usize) -> bool {
        if index >= self.values.len() * 32 {
            return false;
        }
        let num_batch = Self::get_num_batch(index);
        let bit_pos = Self::get_bit_pos(index, num_batch);
        return self.values[num_batch - 1] & (1 << bit_pos) != 0;
    }
}

impl Display for BitSet {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut res = String::new();
        for batch in &self.values {
            for i in 0..32 {
                if batch & (1 << i) != 0 {
                    res.push_str("1");
                } else {
                    res.push_str("0");
                }
            }
        }
        write!(f, "{}", res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_all() {
        let mut result = BitSet::new();
        for i in 0..32 {
            result.set(i);
        }
        for i in 0..32 {
            assert_eq!(true, result.is_set(i));
        }
        assert_eq!(result.size(), 32);
        assert_eq!(result.to_string(), "11111111111111111111111111111111");
    }

    #[test]
    fn reset_all() {
        let mut result = BitSet::new();
        for i in 0..32 {
            result.set(i);
        }
        for i in 0..32 {
            result.reset(i);
        }
        for i in 0..32 {
            assert_eq!(false, result.is_set(i));
        }
        assert_eq!(result.size(), 0);
        assert_eq!(result.to_string(), "");
    }

    #[test]
    fn expansion() {
        let mut result = BitSet::new();
        for i in 0..32 {
            result.set(i);
        }
        result.set(64);
        for i in 0..32 {
            assert_eq!(true, result.is_set(i));
        }
        for i in 32..64 {
            assert_eq!(false, result.is_set(i));
        }
        assert_eq!(true, result.is_set(64));
        assert_eq!(result.size(), 33);
        assert_eq!(result.to_string(), "111111111111111111111111111111110000000000000000000000000000000010000000000000000000000000000000");
    }

    #[test]
    fn decrease() {
        let mut result = BitSet::new();
        for i in 0..32 {
            result.set(i);
        }
        result.set(64);
        result.reset(64);
        assert_eq!(result.size(), 32);
        assert_eq!(result.to_string(), "11111111111111111111111111111111");
    }
}
