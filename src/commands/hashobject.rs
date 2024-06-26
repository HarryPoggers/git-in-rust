use std::{fs::File, io::Read, path::Path};

use sha1::{Digest, Sha1};

use crate::utils;

pub(crate) fn invoke(file_path: &str, write: bool) {
    // Get Content of File
    let path = Path::new(file_path);
    if !path.exists() {
        panic!("Given file path is invalid.")
    }
    let mut file = File::open(&path).expect("File unable to be opened.");
    let mut content = Vec::new();
    file.read_to_end(&mut content).unwrap();

    // Convert the size of the content to a string
    let size_str = content.len().to_string();

    // Create a new vector that starts with "blob ", followed by the size, a null byte, and the content
    let mut to_compress = format!("blob {}\0", size_str).into_bytes();
    to_compress.extend_from_slice(&content);

    // Get sha1 hash of the file before Compression and output it
    let mut hasher = Sha1::new();

    hasher.update(&to_compress);

    let file_hash = hasher.finalize();
    let hex_hash = hex::encode(file_hash);

    if write {
        utils::write_object_file(&hex_hash, to_compress);
    }

    println!("{}", &hex_hash);
}
