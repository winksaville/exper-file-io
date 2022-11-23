use std::error::Error;
use std::result::Result;
use std::fs::{File, metadata, remove_file};
use std::io::prelude::*;
use std::path::Path;

// Panics if file doesn't exist
fn open_existing_and_read(file_name: &str) {
    // Create a path to the desired file
    let path = Path::new(file_name);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}\n", display, s),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}

fn open_existing_and_read_buf_at_a_time(file_name: &str, buf: &mut [u8]) -> Result<(usize, usize), Box<dyn Error>> {
    println!("open_existing_and_read_buf_at_a_time: file_name={file_name} buf.len={}", buf.len());

    let path = Path::new(file_name);

    let mut file = File::open(path)?;

    let mut len_read = 0usize;
    loop {
        match file.read(buf) {
            Ok(count) => {
                len_read += count;
                println!("read {count} buf={:X?}", &buf[..count]);

                if count < buf.len() {
                    return Ok((len_read, count));
                }
            }
            Err(why) => {
                return Err(format!("Couldn't read {path:?}: {why}").into());
            }
        }
    }
}

fn file_exists(file_name: &str) -> bool {
    match metadata(file_name) {
        Ok(md) => md.is_file(),
        Err(_) => false,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create heljlo.txt if it doesn't exist
    let hello_txt = "hello.txt";
    if file_exists(hello_txt) {
        println!("Delete {hello_txt}");
        remove_file(hello_txt)?;
    }
    if !file_exists(hello_txt) {
        println!("Creating {hello_txt}");
        let mut file = File::create(hello_txt)?;
        file.write_all("Hello World".as_bytes())?;
        drop(file);
    }

    open_existing_and_read(hello_txt);


    const BUF_LEN: usize = 5;
    let mut buf = [0u8; BUF_LEN];

    let (len_read, len_last_buf) = open_existing_and_read_buf_at_a_time(hello_txt, &mut buf)?;
    println!("len_read={len_read} len_last_buf={len_last_buf}");

    Ok(())
}

