use crate::maneatingape::point::Point;

#[derive(Debug)]
pub struct Array2D<T> {
    raw_data: Vec<T>,
    len_x: i32,
    len_y: i32,
}

impl Array2D<u8> {
    pub fn new(s: &str) -> Self {
        let raw_data = s
            .bytes()
            .filter(|x| !x.is_ascii_whitespace())
            .collect::<Vec<_>>();
        let len_x = s.lines().next().unwrap().len() as i32;
        let len_y = raw_data.len() as i32 / len_x;

        Self {
            raw_data,
            len_x,
            len_y,
        }
    }
}

impl<T> Array2D<T> {
    pub fn default(len_x: usize) -> Self {
        Array2D {
            raw_data: vec![],
            len_x: len_x as i32,
            len_y: 0,
        }
    }

    pub fn add_line(&mut self, line_iter: impl IntoIterator<Item = T>) {
        let len_before = self.raw_data.len();
        self.raw_data.extend(line_iter);
        self.len_y += 1;
        let len_after = self.raw_data.len();

        if len_after - len_before != self.len_x as usize {
            panic!("line_iter is too big or too small!")
        }
    }

    #[inline]
    pub fn len_x(&self) -> i32 {
        self.len_x
    }

    #[inline]
    pub fn len_y(&self) -> i32 {
        self.len_y
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.raw_data.is_empty()
    }

    pub fn contains(&self, index: &Point) -> bool {
        index.x >= 0 && index.x < self.len_x() && index.y >= 0 && index.y < self.len_y()
    }
}

impl<T> std::ops::IndexMut<&Point> for Array2D<T> {
    #[inline]
    fn index_mut(&mut self, index: &Point) -> &mut T {
        &mut self.raw_data[(index.y * self.len_x + index.x) as usize]
    }
}

impl<T> std::ops::Index<&Point> for Array2D<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: &Point) -> &Self::Output {
        &self.raw_data[(index.y * self.len_x + index.x) as usize]
    }
}
