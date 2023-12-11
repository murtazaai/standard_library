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
pub struct Point<X1, Y1> {
    pub(crate) x: X1,
    pub(crate) y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    #[allow(dead_code)]
    pub(crate) fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}