use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

pub(crate) struct Database {
    file_path: String,
    map: HashMap<String, String>,
}

impl Database {
    pub(crate) fn new(file_path: &str) -> Result<Database, Error> {
        let path = PathBuf::from(file_path);
        let map = Database::read_file(path)?;
        Ok(Database {
            file_path: file_path.to_owned(),
            map,
        })
    }

    pub(crate) fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub(crate) fn flush(&self) -> Result<(), Error> {
        let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }

        std::fs::write(self.file_path.to_owned(), contents)
    }

    fn read_file(path: PathBuf) -> Result<HashMap<String, String>, Error> {
        let mut map = HashMap::new();

        if path.exists() {
            for line in std::fs::read_to_string(path)?.lines() {
                let (key, value) = Database::parse_line(line)?;
                map.insert(key, value);
            }
        };

        Ok(map)
    }

    fn parse_line(line: &str) -> Result<(String, String), Error> {
        match line.split_once("\t") {
            Some((key, value)) => Ok((key.to_owned(), value.to_owned())),
            None => return Err(Error::new(ErrorKind::Other, "Corrupted database")),
        }
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}
