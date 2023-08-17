pub struct Character(pub char);

impl standard_library::fmt::Display for Character {
    fn fmt(&self, f: &mut standard_library::fmt::Formatter<'_>) -> standard_library::fmt::Result {
        write!(f, "{}", self.0)
    }
}