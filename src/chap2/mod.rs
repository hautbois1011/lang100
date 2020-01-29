use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result;
use std::path::Path;

pub fn count_lines(path: &Path) -> Result<usize, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut cnt = 0;
    reader.lines().for_each(|_| cnt += 1);
    Ok(cnt)
}

pub fn tab_to_space(path: &Path) -> Result<String, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let a = reader.lines()
        .map(|s| {
            match s {
                Ok(s) => s.replace("\t", " "),
                Err(err) => err.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    Ok(a)
}
