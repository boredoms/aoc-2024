use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]

// TODO: make this generic for different integer types
pub struct Point {
    pub x: i64,
    pub y: i64,
}

pub const ORIGIN: Point = Point::new(0, 0);
pub const UP: Point = Point::new(0, -1);
pub const DOWN: Point = Point::new(0, 1);
pub const LEFT: Point = Point::new(-1, 0);
pub const RIGHT: Point = Point::new(1, 0);

pub const CARDINALS: [Point; 4] = [UP, DOWN, LEFT, RIGHT];

pub const UP_LEFT: Point = Point::new(-1, -1);
pub const UP_RIGHT: Point = Point::new(1, -1);
pub const DOWN_LEFT: Point = Point::new(-1, 1);
pub const DOWN_RIGHT: Point = Point::new(1, 1);

pub const NEIGHBORS: [Point; 8] = [
    UP, UP_RIGHT, RIGHT, DOWN_RIGHT, DOWN, DOWN_LEFT, LEFT, UP_LEFT,
];

impl Point {
    pub const fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    pub fn manhattan(&self, other: &Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

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
