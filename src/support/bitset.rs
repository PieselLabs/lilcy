use core::fmt;
use std::{fmt::{Display, Formatter}, mem::size_of};

type Word = u32;
pub struct BitSet {
    count_ones: usize,
    buckets: Vec<Word>
}

impl BitSet {
    pub fn new() -> Self {
        Self {
            count_ones: 0,
            buckets: Vec::new(),
        }
    }

    fn bits_in_bucket() -> usize {
        return size_of::<Word>() * 8;
    }

    pub fn len(&self) -> usize {
        return self.buckets.len() * Self::bits_in_bucket();
    }

    pub fn set(&mut self,  val: bool, index : usize) {
        let num_bucket = index / Self::bits_in_bucket();
        if num_bucket + 1 > self.buckets.len() {
            self.buckets.resize(num_bucket + 1, 0);
        }
        let bit_pos = index % (Self::bits_in_bucket());
        let flag = 1 << bit_pos;
        if val {
            self.buckets[num_bucket] |= flag;
            self.count_ones += 1;
        } else {
            self.buckets[num_bucket] &= !flag;
            self.count_ones -= 1;
        }
    }

    pub fn get(&self, index : usize) -> bool {
        if index >= self.buckets.len() * 32 {
            return false;
        }
        let num_bucket = index / Self::bits_in_bucket();
        let bit_pos = index % Self::bits_in_bucket();
        return self.buckets[num_bucket] & (1 << bit_pos) != 0;
    }

    pub fn count_ones(&self) -> usize {
        return self.count_ones;
    }

    pub fn count_zeros(&self) -> usize {
        return self.len() - self.count_ones;
    }

    pub fn shirnk_zero(&mut self) {
        let mut i = self.buckets.len() - 1;
        while self.buckets[i] == 0 {
            self.buckets.pop();
            if i == 0 {
                break;
            }
            i -= 1;
        }
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
            result.set(true, i);
        }
        assert!(result.len() % 32 == 0);
        for i in  0..result.len() {
            assert_eq!(true, result.get(i));
        }
        assert_eq!(result.len(), 32);
        assert_eq!(result.count_ones(), 32);
        assert_eq!(result.count_zeros(), 0);
        result.shirnk_zero();
        assert_eq!(result.to_string(), "11111111111111111111111111111111");
    }

    #[test]
    fn reset_all() {
        let mut result = BitSet::new();
        for i in 0..32 {
            result.set(true, i);
        }
        for i in 0..32 {
            result.set(false, i);
        }
        assert!(result.len() % 32 == 0);
        for i in 0..result.len() {
            assert_eq!(false, result.get(i));
        }
        assert_eq!(result.count_ones(), 0);
        assert_eq!(result.count_zeros(), 32);
        result.shirnk_zero();
        assert_eq!(result.to_string(), "");
    }

    #[test]
    fn expansion() {
        let mut result = BitSet::new();
        for i in 0..32 {
            result.set(true,i);
        }
        result.set(true,64);
        assert!(result.len() % 32 == 0);
        for i in 0..32 {
            assert_eq!(true, result.get(i));
        }
        for i in 32..64 {
            assert_eq!(false, result.get(i));
        }
        assert_eq!(true, result.get(64));
        assert_eq!(result.count_ones(), 33);
        assert_eq!(result.count_zeros(), 63);
        result.shirnk_zero();
        assert_eq!(result.to_string(), "111111111111111111111111111111110000000000000000000000000000000010000000000000000000000000000000");
    }

    #[test]
    fn decrease() {
        let mut result = BitSet::new();
        for i in 0..32 {
            result.set(true, i);
        }
        result.set(true, 64);
        result.set(false, 64);
        assert_eq!(result.count_ones(), 32);
        assert_eq!(result.count_zeros(), 64);
        result.shirnk_zero();
        assert_eq!(result.to_string(), "11111111111111111111111111111111");
    }
}
