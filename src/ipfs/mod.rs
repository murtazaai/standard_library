use std::io::{Error, Read};
use std::fs::File;

#[allow(unused_doc_comments, dead_code)]
pub fn read_file_content(path: String) -> Result<String, Error> {
    /// Open the file from the String path
    let file: Result<File, Error> = match File::open(path) {
        Ok(file) => Ok(file),
        Err(e) => return Err(e),
    };

    let mut content = String::new();

    match file.unwrap().read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(e),
    }
}