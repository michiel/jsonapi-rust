use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_json_file(filename: &str) -> String {
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();

    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, Error::description(&why));
    };

    s
}
