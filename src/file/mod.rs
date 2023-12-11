use std::fs::File;
use std::io;
use std::io::Read;

#[allow(dead_code)]
pub fn read_username_from_file(path: &str, mut username: String) -> Result<String, io::Error> {
    // let username_file_result = File::open("src/file/username");
    //
    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(error) => return Err(error),
    // };

    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }
    File::open(path)?.read_to_string(&mut username)?;

    Ok(username)
}