use std::fs::{File, OpenOptions};
use std::hash::Hash;
use std::io::{ BufWriter, Write};
use std::net::{SocketAddr};
use std::os::unix::fs::OpenOptionsExt;
use std::time::{SystemTime, UNIX_EPOCH};
use once_cell::sync::Lazy;
use crate::core::mutex::Mutex;

/*
    If a BufWriter was created for the same file in a function, and the server received e.g. 1000 requests in a second,
    this would result to 1000 threads attempting to write to the same file. Hence the mutex.
 */
static FILE_OPT: Lazy<File> = Lazy::new(|| {
    OpenOptions::new()
        .create(true)
        .append(true)
        .custom_flags(libc::O_SYNC)
        .open("log.txt")
        .unwrap()
});
static mut COMMON_BUFFER: Lazy<Mutex<BufWriter<File>>> = Lazy::new(|| unsafe {
    Mutex::new(BufWriter::with_capacity(1000, FILE_OPT.try_clone().unwrap()))
});

pub fn log_request(address: SocketAddr, method: &str, url: &str, user_agent: &String) -> Result<(), ()> {
    let st = format!("ip: {} ,  user-agent: {:?}, method: {:?}, requested resource: {:?}, on: {:?}  \n",
                     address.ip(), user_agent, method, url, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());

    unsafe {
        let mut buf = COMMON_BUFFER.acquire_mut();
        buf.write(st.as_ref()).unwrap();
        COMMON_BUFFER.free();
    }

    Ok(())
}


pub fn flush_log() {
    unsafe {
        let mut buffer = COMMON_BUFFER.acquire_mut();
        let _ = buffer.flush();
        COMMON_BUFFER.free();
    }
}



pub fn write_to_file() {}