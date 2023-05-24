use std::io::Error;
use std::fs::File;

/// Read the data structures of rust file
/// The data structure must contain atleast traits, methodologies, and structures 
#[allow(unused_doc_comments)]
pub fn lost_plus_found(path: String) -> Result<File, Error> {
    /// Open the file from the String path
    match File::open(path) {
        Ok(file) => Ok(file),
        Err(e) => Err(e),
    }
}