use std::fmt::{Debug, Display};

pub fn f0<T, U>(t: T, u: U) -> String
where
    T: Display + Clone + Debug,
    U: Clone + Debug,
{
    format!("T: {:?}, U: {:?}", t, u)
}
