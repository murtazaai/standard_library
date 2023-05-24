use std::io::Error;
use std::fs::File;

/// Read a file and return it
#[allow(unused_doc_comments)]
pub fn lost_plus_found(path: String) -> Result<File, Error> {
    /// Open the file from the String path
    match File::open(path) {
        Ok(file) => Ok(file),
        Err(e) => Err(e),
    }
}