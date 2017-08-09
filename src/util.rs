use std::io;
use std::io::{Read, Write, BufRead};
use std::fs::OpenOptions;

pub fn prompt(prompt: &str, buf: &mut String) -> io::Result<usize> {
    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();
    stdout_handle.write(prompt.as_bytes())?;
    stdout_handle.flush()?;

    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();
    stdin_handle.read_line(buf)
}

pub fn read_from_file(path: &str) -> io::Result<String> {
    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

pub fn overwrite_file(path: &str, text: &str) -> io::Result<usize> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    file.write(text.as_bytes())
}
