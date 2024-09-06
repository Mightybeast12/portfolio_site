use std::fs;
use std::io::Error;

fn priv_read_file(file_path: &str) -> Result<String, Error> {
    let contents = fs::read_to_string(file_path)?;
    Ok(format!(r#"{}"#, contents))
}

pub fn read_file(path: &str) -> Result<String, Error> {
    match priv_read_file(path) {
        Ok(content) => Ok(content), 
        Err(e) => Err(e), 
    }
}