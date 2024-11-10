use core::fmt;
use std::fmt::{Display, Formatter};

pub struct BitSet {
    size: usize,
    buckets: Vec<u32>
}

impl BitSet {
    pub fn new() -> Self {
        Self {
            size: 0,
            buckets: Vec::new(),
        }
    }

    fn get_num_bucket(index: usize) -> usize {
        return index / 32 + 1;
    }

    fn get_bit_pos(index: usize) -> usize {
        return index - (Self::get_num_bucket(index) - 1) * 32;
    }

    pub fn size(&self) -> usize {
        return self.size;
    }

    pub fn set(&mut self, index : usize) {
        let num_bucket = Self::get_num_bucket(index);
        if num_bucket > self.buckets.len() {
            for _ in 0..(num_bucket - self.buckets.len()) {
                self.buckets.push(0);
            }
        }
        let bit_pos = Self::get_bit_pos(index);
        self.buckets[num_bucket - 1] |= 1 << bit_pos;
        self.size += 1;
    }

    pub fn reset(&mut self, index : usize) {
        assert!(index <= self.buckets.len() * 32);
        let mut num_bucket = Self::get_num_bucket(index);
        let bit_pos = Self::get_bit_pos(index);
        self.buckets[num_bucket - 1] &= !(1 << bit_pos);
        self.size -= 1;
        while num_bucket != 0 && num_bucket == self.buckets.len() && self.buckets[num_bucket - 1] == 0 {
            self.buckets.pop();
            num_bucket -= 1;
        }
    }

    pub fn is_set(&self, index : usize) -> bool {
        if index >= self.buckets.len() * 32 {
            return false;
        }
        let num_bucket = Self::get_num_bucket(index);
        let bit_pos = Self::get_bit_pos(index);
        return self.buckets[num_bucket - 1] & (1 << bit_pos) != 0;
    }
}

impl Display for BitSet {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut res = String::new();
        for bucket in &self.buckets {
            for i in 0..32 {
                if bucket & (1 << i) != 0 {
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
