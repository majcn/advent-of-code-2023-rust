#[derive(Debug, Clone, Copy)]
pub struct U1024([u64; 16]);

impl U1024 {
    pub fn count_ones(&self) -> u32 {
        self.0.iter().map(|x| x.count_ones()).sum()
    }

    pub const ZERO: U1024 = U1024([0; 16]);
    pub const ONE: U1024 = U1024([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

impl std::ops::Shl<usize> for U1024 {
    type Output = U1024;

    fn shl(self, rhs: usize) -> Self::Output {
        let mut result = U1024::ZERO;
        let word_shift = rhs / 64;
        let bit_shift = rhs % 64;
        for i in 0..16 {
            if bit_shift < 64 && i + word_shift < 16 {
                result.0[i + word_shift] += self.0[i] << bit_shift;
            }
            if bit_shift > 0 && i + word_shift + 1 < 16 {
                result.0[i + word_shift + 1] += self.0[i] >> (64 - bit_shift);
            }
        }
        result
    }
}

impl std::ops::Shr<usize> for U1024 {
    type Output = U1024;

    fn shr(self, shift: usize) -> U1024 {
        let mut result = U1024::ZERO;
        let word_shift = shift / 64;
        let bit_shift = shift % 64;
        for i in word_shift..16 {
            result.0[i - word_shift] += self.0[i] >> bit_shift;
            if bit_shift > 0 && i < 16 - 1 {
                result.0[i - word_shift] += self.0[i + 1] << (64 - bit_shift);
            }
        }
        result
    }
}

impl std::ops::BitAnd for U1024 {
    type Output = U1024;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut result = U1024::ZERO;
        for i in 0..16 {
            result.0[i] = self.0[i] & rhs.0[i];
        }
        result
    }
}

impl std::ops::BitOr for U1024 {
    type Output = U1024;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut result = U1024::ZERO;
        for i in 0..16 {
            result.0[i] = self.0[i] | rhs.0[i];
        }
        result
    }
}

impl std::ops::BitOrAssign for U1024 {
    fn bitor_assign(&mut self, rhs: Self) {
        for i in 0..16 {
            self.0[i] = self.0[i] | rhs.0[i];
        }
    }
}

impl std::ops::Not for U1024 {
    type Output = U1024;

    fn not(self) -> Self::Output {
        let mut result = U1024::ZERO;
        for i in 0..16 {
            result.0[i] = !self.0[i];
        }
        result
    }
}
