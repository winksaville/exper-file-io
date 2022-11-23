mod file_io_thread;

use crate::file_io_thread::file_io_thread;

use custom_logger::env_logger_init;
use std::error::Error;
use std::result::Result;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger_init("info");
    log::info!("main:+");

    let worker_thread = thread::spawn(move || {
        log::info!("worker_thread:+");
        let r = file_io_thread();
        log::info!("worker_thread:- file_io_thread={:?}", r);
    });

    log::info!("main:  working while worker_thread is not finished");
    while !worker_thread.is_finished() {
        //log::info!("main:  working");
        log::info!("main:  sleeping 5ns");
        thread::sleep(Duration::from_nanos(5));
    }
    let r = worker_thread.join();
    log::info!("main:  joined  worker_thread.join()={:?}", r);

    log::info!("main:-");
    Ok(())
}
