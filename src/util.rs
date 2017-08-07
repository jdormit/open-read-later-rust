use std::io;
use std::io::{Write, BufRead};

pub fn prompt(prompt: &str, buf:&mut String) -> io::Result<usize> {
    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();
    stdout_handle.write(prompt.as_bytes())?;
    stdout_handle.flush();

    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();
    stdin_handle.read_line(buf)
}
