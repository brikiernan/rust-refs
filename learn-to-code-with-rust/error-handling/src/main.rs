use std::fs;
use std::io::{self, Write};
use std::process;

fn write_to_file() -> io::Result<String> {
    let mut file_name = String::new();
    let mut content = String::new();

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    write!(handle, "What file would you like to write to? ").unwrap();
    handle.flush().unwrap();
    stdin.read_line(&mut file_name)?;

    write!(handle, "What would you like to write to the file? ").unwrap();
    handle.flush().unwrap();
    stdin.read_line(&mut content)?;

    fs::write(file_name.trim(), content.trim())?;

    Ok(file_name)
}

fn main() {
    match write_to_file() {
        Ok(file_name) => {
            println!("Successfully wrote to file {}", file_name);
        }
        Err(e) => {
            eprintln!("There was an error: {}", e);
            process::exit(1);
        }
    }
}
