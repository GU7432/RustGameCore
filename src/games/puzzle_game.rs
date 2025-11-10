use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, Write};

pub struct Puzzle16 {
    data: Vec<i32>,
}
impl Puzzle16 {
    pub fn new() -> Puzzle16 {
        let mut rng = thread_rng();
        let mut candidate = Puzzle16 {
            data: (0..=15).collect(),
        };
        loop {
            candidate.data.shuffle(&mut rng);
            if candidate.checker() {
                return candidate;
            }
        }
    }
    pub fn action(&mut self, op: i32) {
        match op {
            0 => {
                // up
                let zero = self.find_zero();
                let (i, j) = Self::get_location(zero);

                if i as i32 - 1 >= 0 {
                    let up = Self::get_idx(i - 1, j);
                    self.data.swap(zero, up);
                }
            }
            1 => {
                // down
                let zero = self.find_zero();
                let (i, j) = Self::get_location(zero);

                if i + 1 < 4 {
                    let down = Self::get_idx(i + 1, j);
                    self.data.swap(zero, down);
                }
            }
            2 => {
                // left
                let zero = self.find_zero();
                let (i, j) = Self::get_location(zero);

                if j as i32 - 1i32 >= 0 {
                    let left = Self::get_idx(i, j - 1);
                    self.data.swap(zero, left);
                }
            }
            3 => {
                // right
                let zero = self.find_zero();
                let (i, j) = Self::get_location(zero);

                if j + 1 < 4 {
                    let right = Self::get_idx(i, j + 1);
                    self.data.swap(zero, right);
                }
            }
            _ => {}
        }
    }
    pub fn show<W: Write>(&self, out: &mut W) -> io::Result<()> {
        for i in 0..4 {
            for j in 0..4 {
                write!(out, "{:>3} ", self.data[i * 4 + j])?;
            }
            write!(out, "\r\n")?;
        }
        Ok(())
    }
    pub fn iswin(&self) -> bool {
        for (i, v) in self.data.iter().enumerate() {
            if i + 1 != *v as usize && i != 15 {
                return false
            }
        }
        true
    }
    fn get_idx(i: usize, j: usize) -> usize {
        i * 4 + j
    }
    fn get_location(v: usize) -> (usize, usize) {
        (v / 4, v % 4)
    }
    fn find_zero(&self) -> usize {
        for (i, v) in self.data.iter().enumerate() {
            if *v == 0 {
                return i;
            }
        }
        0
    }
    fn calc_inv(&self) -> i32 {
        let v = &self.data;
        let mut ret = 0;
        let n = v.len();
        for i in 0..n {
            if v[i] == 0 {
                continue;
            }
            for j in (i + 1)..n {
                if v[j] == 0 {
                    continue;
                }
                if v[i] > v[j] {
                    ret += 1;
                }
            }
        }
        ret
    }
    fn checker(&self) -> bool {
        let w = 4usize;
        let zero_idx = self
            .data
            .iter()
            .position(|&x| x == 0)
            .expect("no zero tile");
        let inv = self.calc_inv();
        let row_from_bottom = (w - (zero_idx / w)) as i32; // 底部為 1
        (inv + row_from_bottom) % 2 == 1
    }
}

