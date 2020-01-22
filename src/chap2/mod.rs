use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result;

pub fn count_lines() -> Result<usize, std::io::Error> {
    let file = File::open("src/chap2/hightemp.txt")?;
    let reader = BufReader::new(file);
    let mut cnt = 0;
    reader.lines().for_each(|_| cnt += 1);
    Ok(cnt)
}
