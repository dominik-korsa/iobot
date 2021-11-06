use std::str::{from_utf8, Utf8Error};

pub fn to_lines(output: &Vec<u8>) -> Result<Vec<String>, Utf8Error> {
    let mut lines: Vec<String> = from_utf8(output)?
        .lines()
        .map(|x| x.trim_end().to_string())
        .collect();
    while lines.ends_with(&["".to_string()]) {
        lines.pop();
    }
    Ok(lines)
}
