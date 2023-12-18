use super::point::Point;

#[derive(Debug)]
pub struct Array2D<T> {
    line_size: usize,
    raw_data: Vec<T>,
}

impl<T> Array2D<T> {
    pub fn new(line_size: usize) -> Self {
        Array2D {
            line_size,
            raw_data: vec![],
        }
    }

    pub fn add_line(&mut self, line_iter: impl IntoIterator<Item = T>) {
        let len_before = self.raw_data.len();
        self.raw_data.extend(line_iter);
        let len_after = self.raw_data.len();

        if len_after - len_before != self.line_size {
            panic!("line_iter is too big or too small!")
        }
    }

    pub fn len(&self) -> usize {
        self.raw_data.len() / self.line_size
    }

    pub fn is_empty(&self) -> bool {
        self.raw_data.is_empty()
    }

    pub fn len_line(&self) -> usize {
        self.line_size
    }

    pub fn iter_keys(&self) -> Array2DIterKeys<T> {
        Array2DIterKeys { data: self, i: 0 }
    }
}

impl<T> std::ops::Index<(usize, usize)> for Array2D<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.raw_data[index.1 * self.line_size + index.0]
    }
}

impl<T> std::ops::Index<&(usize, usize)> for Array2D<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: &(usize, usize)) -> &Self::Output {
        &self.raw_data[index.1 * self.line_size + index.0]
    }
}

impl<T> std::ops::Index<Point> for Array2D<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Point) -> &Self::Output {
        &self.raw_data[index.y as usize * self.line_size + index.x as usize]
    }
}

impl<T> std::ops::Index<&Point> for Array2D<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: &Point) -> &Self::Output {
        &self.raw_data[index.y as usize * self.line_size + index.x as usize]
    }
}

pub struct Array2DIterKeys<'a, T> {
    data: &'a Array2D<T>,
    i: usize,
}

impl<T> Iterator for Array2DIterKeys<'_, T> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.data.raw_data.len() {
            return None;
        }

        let result = (self.i % self.data.line_size, self.i / self.data.line_size);
        self.i += 1;

        Some(result)
    }
}
