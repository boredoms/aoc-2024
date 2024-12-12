use std::ops::{Index, IndexMut};

use crate::util::point::Point;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub elements: Vec<T>,
    pub size: Point,
}

// iterator for iterating the coordinates of a grid in english reading order
#[derive(Debug)]
pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    n: usize,
}

impl<'a, T> Iterator for GridIterator<'a, T> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == self.grid.len() {
            None
        } else {
            let pos = self.grid.point_from_index(self.n);

            self.n += 1;

            Some(pos)
        }
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self.elements[self.calculate_index(&index)]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        let index = self.calculate_index(&index);
        &mut self.elements[index]
    }
}

impl<T> Grid<T> {
    #[inline]
    fn calculate_index(&self, index: &Point) -> usize {
        (index.y * self.size.x + index.x) as usize
    }

    #[inline]
    fn point_from_index(&self, i: usize) -> Point {
        Point::new((i as i32) % self.size.x, (i as i32) / self.size.x)
    }

    pub fn in_grid(&self, index: &Point) -> bool {
        index.x >= 0 && index.x < self.size.x && index.y >= 0 && index.y < self.size.y
    }

    pub fn try_get(&self, index: &Point) -> Option<&T> {
        if self.in_grid(index) {
            Some(&self[*index])
        } else {
            None
        }
    }

    pub fn find(&self, predicate: impl Fn(&T) -> bool) -> Option<Point> {
        self.elements
            .iter()
            .position(predicate)
            .and_then(|i| Some(self.point_from_index(i)))
    }

    pub fn find_all(&self, predicate: impl Fn(&T) -> bool) -> Vec<Point> {
        let mut res = Vec::new();

        self.elements.iter().enumerate().for_each(|(i, e)| {
            if predicate(e) {
                res.push(self.point_from_index(i));
            }
        });

        res
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn iter(&self) -> GridIterator<T> {
        GridIterator { grid: self, n: 0 }
    }
}

impl<T: Default + Copy> Grid<T> {
    pub fn new(x: i32, y: i32) -> Self {
        Grid {
            elements: vec![T::default(); (x * y) as usize],
            size: Point::new(x, y),
        }
    }

    pub fn new_from_size(size: &Point) -> Self {
        Grid {
            elements: vec![T::default(); (size.x * size.y) as usize],
            size: *size,
        }
    }

    pub fn new_with_same_size<A>(parent: &Grid<A>) -> Self {
        Grid {
            elements: vec![T::default(); parent.len()],
            size: parent.size,
        }
    }

    pub fn get_or_default(&self, index: &Point) -> T {
        if self.in_grid(index) {
            self[*index]
        } else {
            T::default()
        }
    }
}

impl Grid<u8> {
    pub fn from_str(s: &str) -> Self {
        let lines_as_bytes: Vec<_> = s.lines().map(|s| s.as_bytes()).collect();

        let width = lines_as_bytes[0].len();
        let height = lines_as_bytes.len();

        let mut elements = Vec::with_capacity(width * height);

        for line in lines_as_bytes {
            elements.extend_from_slice(line);
        }

        Grid {
            elements,
            size: Point::new(width as i32, height as i32),
        }
    }
}
