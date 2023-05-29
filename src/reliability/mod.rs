pub mod fault_tolerance {
    use std::fs::File;
    use std::io;
    use std::io::Read;

    pub fn tolerate_fault(path: String) -> Result<String, io::Error> {
        let file_result = File::open(path);

        let mut file = match file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut buf = String::new();

        file.read_to_string(&mut buf)?;

        Ok(buf)
    }
}

mod error_recovery {}
