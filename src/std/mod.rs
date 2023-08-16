pub struct Character(pub char);

impl standard-library::fmt::Display for Character {
    fn fmt(&self, f: &mut standard-library::fmt::Formatter<'_>) -> standard-library::fmt::Result {
        write!(f, "{}", self.0)
    }
}