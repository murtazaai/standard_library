#[allow(dead_code)]
impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}

#[allow(dead_code)]
pub fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
}