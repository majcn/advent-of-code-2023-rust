use regex::Captures;
use regex::Regex;

pub trait ParseRegex {
    fn parse_i32<const N: usize>(&self, haystack: &str) -> [i32; N];
    fn parse_u32<const N: usize>(&self, haystack: &str) -> [u32; N];
    fn parse_u64<const N: usize>(&self, haystack: &str) -> [u64; N];
    fn parse_usize<const N: usize>(&self, haystack: &str) -> [usize; N];
}

pub trait ParseCaptures {
    fn parse_i32<const N: usize>(&self) -> [i32; N];
    fn parse_u32<const N: usize>(&self) -> [u32; N];
    fn parse_u64<const N: usize>(&self) -> [u64; N];
    fn parse_usize<const N: usize>(&self) -> [usize; N];
}

impl<'h> ParseCaptures for Captures<'h> {
    fn parse_i32<const N: usize>(&self) -> [i32; N] {
        self.extract::<N>().1.map(|x| x.parse::<i32>().unwrap())
    }

    fn parse_u32<const N: usize>(&self) -> [u32; N] {
        self.extract::<N>().1.map(|x| x.parse::<u32>().unwrap())
    }

    fn parse_u64<const N: usize>(&self) -> [u64; N] {
        self.extract::<N>().1.map(|x| x.parse::<u64>().unwrap())
    }

    fn parse_usize<const N: usize>(&self) -> [usize; N] {
        self.extract::<N>().1.map(|x| x.parse::<usize>().unwrap())
    }
}

impl ParseRegex for Regex {
    fn parse_i32<const N: usize>(&self, haystack: &str) -> [i32; N] {
        self.captures(haystack).unwrap().parse_i32()
    }

    fn parse_u32<const N: usize>(&self, haystack: &str) -> [u32; N] {
        self.captures(haystack).unwrap().parse_u32()
    }

    fn parse_u64<const N: usize>(&self, haystack: &str) -> [u64; N] {
        self.captures(haystack).unwrap().parse_u64()
    }

    fn parse_usize<const N: usize>(&self, haystack: &str) -> [usize; N] {
        self.captures(haystack).unwrap().parse_usize()
    }
}
