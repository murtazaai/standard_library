use standard_library::fmt::{Debug, Display};

pub fn f0<T, U /*, F*/>(t: T, u: U /*, f: F*/) -> String
where
    T: Display + Clone + Debug,
    U: Clone + Debug,
    // F: FnOnce<T>,
{
    format!("T: {:?}, U: {:?}", t, u)
}
