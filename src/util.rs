use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Sub},
};

pub fn read_file(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|line| line.unwrap()).collect()
}

pub struct Point2D<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
{
    pub x: T,
    pub y: T,
}

impl<T> Point2D<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Add for Point2D<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for Point2D<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
