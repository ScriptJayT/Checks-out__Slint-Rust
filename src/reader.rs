use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

const BINPATH: &str = "data-bin"; //replace to bin for production-code

fn read_file(_path: AsRef<str> ) -> String {
    let path: String = &format!("{}/{}", _path, BINPATH);
    let file = File::open(_path).unwrap();

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    contents
}

fn create_file(_path: AsRef<str>) -> bool {
    let path: String = &format!("{}/{}", _path, BINPATH);
    File::create(path).unwrap();
    
    true
}

fn file_exists(_path: AsRef<str>) -> bool {
    let path: String = &format!("{}/{}", _path, BINPATH);
    
    Path::new(path).exists()
}

fn files_exist(_paths: [AsRef<str>]) {
    let res = _paths
        .into_iter()
        .map(|a, b| a + file_exists(b) )
        .collect();
}