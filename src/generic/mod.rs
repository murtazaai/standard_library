use std::fmt::{Debug, Display};

#[allow(dead_code)]
pub fn f0<T, U /*, F*/>(t: T, u: U /*, f: F*/) -> String
where
    T: Display + Clone + Debug,
    U: Clone + Debug,
    // F: FnOnce<T>,
{
    format!("T: {:?}, U: {:?}", t, u)
}

#[allow(dead_code)]
pub struct Point<T, U> {
    pub(crate) x: T,
    pub(crate) y: U,
}

impl<T, U> Point<T, U> {
    #[allow(dead_code)]
    pub(crate) fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<T, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

pub trait Draw {
    fn perimeter(&self) -> i32;
}

pub trait Shape: Draw {
    fn area(&self) -> i32;
}

pub struct Circle  {
    pub length: i32,
}

impl Draw for Circle {
    fn perimeter(&self) -> i32 {
        self.length
    }
}

impl Shape for Circle {
    fn area(&self) -> i32 {
        self.length .pow(2)
    }
}


