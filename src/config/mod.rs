// use std::{fs::File, io::Read};

// pub struct Config {
//     pub is_case_sensitive: bool
// }

use std::env;

pub fn read_config() -> Vec<String> {
    // let is_case_sensitive = false;

    // // let file = File::open("/.env".to_string());

    // // let mut buf = String::new();

    // // file.unwrap().read_to_string(buf);

    // // let result = buf.split("\n").

    // Config {
    //     is_case_sensitive
    // }
    env::args().collect()
}