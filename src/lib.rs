#![no_std]

mod log;
use log::*;

pub fn hello() {
    info!("hello world");
}
