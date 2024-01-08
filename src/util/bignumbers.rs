#[derive(Debug, Clone, Copy)]
pub struct UX64<const N: usize>([u64; N]);

impl<const N: usize> UX64<N> {
    pub fn count_ones(&self) -> u32 {
        self.0.iter().map(|x| x.count_ones()).sum()
    }

    pub const ZERO: UX64<N> = UX64([0; N]);
    pub const ONE: UX64<N> = {
        let mut result = UX64([0; N]);
        result.0[0] = 1;
        result
    };
}

impl<const N: usize> std::ops::Shl<usize> for UX64<N> {
    type Output = UX64<N>;

    fn shl(self, rhs: usize) -> Self::Output {
        let mut result = UX64::ZERO;
        let word_shift = rhs / 64;
        let bit_shift = rhs % 64;
        for i in 0..N {
            if bit_shift < 64 && i + word_shift < N {
                result.0[i + word_shift] += self.0[i] << bit_shift;
            }
            if bit_shift > 0 && i + word_shift + 1 < N {
                result.0[i + word_shift + 1] += self.0[i] >> (64 - bit_shift);
            }
        }
        result
    }
}

impl<const N: usize> std::ops::Shr<usize> for UX64<N> {
    type Output = UX64<N>;

    fn shr(self, shift: usize) -> Self::Output {
        let mut result = UX64::ZERO;
        let word_shift = shift / 64;
        let bit_shift = shift % 64;
        for i in word_shift..N {
            result.0[i - word_shift] += self.0[i] >> bit_shift;
            if bit_shift > 0 && i < N - 1 {
                result.0[i - word_shift] += self.0[i + 1] << (64 - bit_shift);
            }
        }
        result
    }
}

impl<const N: usize> std::ops::BitAnd for UX64<N> {
    type Output = UX64<N>;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut result = UX64::ZERO;
        for i in 0..N {
            result.0[i] = self.0[i] & rhs.0[i];
        }
        result
    }
}

impl<const N: usize> std::ops::BitOr for UX64<N> {
    type Output = UX64<N>;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut result = UX64::ZERO;
        for i in 0..N {
            result.0[i] = self.0[i] | rhs.0[i];
        }
        result
    }
}

impl<const N: usize> std::ops::BitOrAssign for UX64<N> {
    fn bitor_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.0[i] = self.0[i] | rhs.0[i];
        }
    }
}

impl<const N: usize> std::ops::Not for UX64<N> {
    type Output = UX64<N>;

    fn not(self) -> Self::Output {
        let mut result = UX64::ZERO;
        for i in 0..N {
            result.0[i] = !self.0[i];
        }
        result
    }
}
