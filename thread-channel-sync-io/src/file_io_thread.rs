use std::error::Error;
use std::fs::{metadata, remove_file, File};
use std::io::prelude::*;
use std::path::Path;
use std::result::Result;
use std::sync::mpsc::{Sender, Receiver, channel};

#[derive(Debug)]
pub enum Messages {
    Open {
        // Name of file open
        file_name: String,
    },
    Start,
    Read,
    Data {
        buf: Vec<u8>
    },
    Empty {
        buf: Vec<u8>
    },
    Done {
        result: Result<(), Box::<dyn Error>>,
    }
}

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
        Ok(_) => {
            log::info!("{} contains: \"{}\"", display, s);
        }
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}

fn open_existing_and_read_buf_at_a_time(
    file_name: &str,
    buf: &mut [u8],
) -> Result<(usize, usize), Box<dyn Error>> {
    log::info!(
        "open_existing_and_read_buf_at_a_time: file_name={file_name} buf.len={}",
        buf.len()
    );

    let path = Path::new(file_name);

    let mut file = File::open(path)?;

    let mut len_read = 0usize;
    loop {
        match file.read(buf) {
            Ok(count) => {
                len_read += count;
                log::info!("read {count} buf={:X?}", &buf[..count]);

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

pub fn read_file_thread(partner_rx: Receiver<Messages>) -> Result<Sender<Messages>, Box<dyn Error>> {

    todo!("Figure out if this will spawn a thread or it's in a spawned thread and shouldn't return until done");

    let (tx, rx) = channel::<Messages>();

    #[derive(Debug)]
    enum States {
        Open, Opened, Reading, WaitingForBuffer
    }
    let mut state = States::Open;
    let mut file: Option<File> = None;
    let mut buffers: Vec<Vec<u8>>;
    let len_read: usize = 0;
    let mut active_file_name: Option<String> = None;
    loop {
        let msg = match partner_rx.recv() {
            Ok(msg) => msg,
            Err(_) => {
                println!("Connection broken, stopping");
                break;
            }
        };
        match state {
            States::Open => {
                match msg {
                    Messages::Open { file_name } => {
                        file = match File::open(&file_name) {
                            Ok(file) => {
                                state = States::Opened;
                                active_file_name = Some(file_name.clone());
                                Some(file)
                            }
                            Err(why) => {
                                tx.send(Messages::Done { result: Err(Box::new(why))});
                                None
                            }
                        };
                    }
                    Messages::Start => panic!("Messages::Start not supported in {state:?}"),
                    Messages::Read => panic!("Messages::Read not supported in {state:?}"),
                    Messages::Data { buf } => panic!("Messages::Data not supported in {state:?}"),
                    Messages::Empty { buf } => buffers.push(buf),
                    Messages::Done { result: _ } => panic!("Messages:Done not supported in {state:?}"),
                }
            }
            States::Opened => {
                match msg {
                    Messages::Open { file_name } => panic!("Messages::Open not supported in {state:?}"),
                    Messages::Start => {
                        // Send message to ourself 
                        tx.send(Messages::Read);   
                        state = States::Reading;
                    }
                    Messages::Read => panic!("Messages::Read not supported in {state:?}"),
                    Messages::Data { buf } => panic!("Messages::Data not supported in {state:?}"),
                    Messages::Empty { buf } => buffers.push(buf),
                    Messages::Done { result: _ } => panic!("Messages:Done not supported in {state:?}"),
                }
            }
            States::Reading => {
                match msg {
                    Messages::Open { file_name } => panic!("Messages::Open not supported in {state:?}"),
                    Messages::Start => panic!("Messages::Read not supported in {state:?}"),
                    Messages::Read => {
                        if let Some(buf) = buffers.pop() {
                            if let Some(f) = file {
                                match f.read(&mut buf) {
                                    Ok(count) => {
                                        len_read += count;
                                        log::info!("read {count} buf={:X?}", &buf[..count]);

                                        if count < buf.len() {
                                            drop(f); // Close the file
                                            file = None; // No file is open
                                            state = States::Open; // Back to Open
                                            active_file_name = None;
                                        }
                                    }
                                    Err(why) => {
                                        // Handles errors!
                                        panic!("Couldn't read {active_file_name:?}: {why}");
                                    }
                                }
                            }
                        } else {
                            state = States::WaitingForBuffer;
                        }
                    }
                    Messages::Data { buf } => panic!("Messages::Data not supported in {state:?}"),
                    Messages::Empty { buf } => buffers.push(buf),
                    Messages::Done { result: _ } => panic!("Messages:Done not supported in {state:?}"),
                }
            }
            States::WaitingForBuffer => {
                match msg {
                    Messages::Open { file_name } => panic!("Messages::Open not supported in {state:?}"),
                    Messages::Start => panic!("Messages::Read not supported in {state:?}"),
                    Messages::Read => panic!("Messages::Read not supported in {state:?}"),
                    Messages::Data { buf } => panic!("Messages::Data not supported in {state:?}"),
                    Messages::Empty { buf } => {
                        buffers.push(buf);
                        tx.send(Messages::Read);   
                        state = States::Reading;
                    }
                    Messages::Done { result: _ } => panic!("Messages:Done not supported in {state:?}"),
                }
            }
        }
    }
    
    log::info!("read_file_thread:-");
    Ok(tx)
}
