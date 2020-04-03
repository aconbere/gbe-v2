use std::io::Error;
use std::path::Path;
use std::io::ErrorKind;
use std::fs::File;

pub fn open_file(p: &str) -> Result<File, Error> {
    let path = Path::new(p);

    if !path.exists() {
        return Err(Error::new(ErrorKind::Other, format!("Path does not exist: {}", p)));
    }

    let f = File::open(path)?;

    Ok(f)
}

