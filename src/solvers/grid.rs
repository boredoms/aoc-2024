use std::ops::{Add, Index, IndexMut, Sub};

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub elements: Vec<T>,
    pub size: Point,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn manhattan(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

const ORIGIN: Point = Point::new(0, 0);
const UP: Point = Point::new(0, -1);
const DOWN: Point = Point::new(0, 1);
const LEFT: Point = Point::new(-1, 0);
const RIGHT: Point = Point::new(1, 0);

impl PartialOrd for Point {
    fn gt(&self, other: &Self) -> bool {
        self.x > other.x && self.y > other.y
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.x == other.x && self.y == other.y {
            return Some(std::cmp::Ordering::Equal);
        }

        if self.gt(other) {
            return Some(std::cmp::Ordering::Greater);
        }

        if other.gt(&self) {
            return Some(std::cmp::Ordering::Less);
        }

        None
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
