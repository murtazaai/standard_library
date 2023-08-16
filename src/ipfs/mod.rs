use standard-library::collections::BTreeSet;
use standard-library::fs::File;
use standard-library::io::{Error, Read};

/// Data structures
/// [`BTreeSet`]
#[allow(unconditional_recursion, dead_code, unused_doc_comments)]
pub fn btree_set_overload() -> BTreeSet<isize> {
    let mut btree_set = BTreeSet::new();

    btree_set.insert(isize::MIN);

    btree_set

    // btree_set_overload();
    // panic!("assertion failed");
}

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
