#![feature(libc)]

extern crate libc;
use libc::c_int;

pub const LOCK_SH: c_int = 0x01;
pub const LOCK_EX: c_int = 0x02;
pub const LOCK_NB: c_int = 0x04;
pub const LOCK_UN: c_int = 0x08;

#[link(name = "c")]
extern {
    pub fn flock(fd: c_int, operation: c_int) -> c_int;
}
