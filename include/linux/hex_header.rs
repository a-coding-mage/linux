/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_char;

extern "C" {
    pub static hex_asc: [c_char; 16];
}

macro_rules! hex_asc_lo {
    ($x:expr) => {
        hex_asc[(($x as u8 & 0x0f) as usize)]
    };
}

macro_rules! hex_asc_hi {
    ($x:expr) => {
        hex_asc[(((($x as u8) & 0xf0) >> 4) as usize)]
    };
}

#[inline]
pub unsafe fn hex_byte_pack(mut buf: *mut c_char, byte: u8) -> *mut c_char {
    *buf = hex_asc_hi!(byte);
    buf = buf.add(1);
    *buf = hex_asc_lo!(byte);
    buf = buf.add(1);
    buf
}

extern "C" {
    pub static hex_asc_upper: [c_char; 16];
}

macro_rules! hex_asc_upper_lo {
    ($x:expr) => {
        hex_asc_upper[(($x as u8 & 0x0f) as usize)]
    };
}

macro_rules! hex_asc_upper_hi {
    ($x:expr) => {
        hex_asc_upper[(((($x as u8) & 0xf0) >> 4) as usize)]
    };
}

#[inline]
pub unsafe fn hex_byte_pack_upper(mut buf: *mut c_char, byte: u8) -> *mut c_char {
    *buf = hex_asc_upper_hi!(byte);
    buf = buf.add(1);
    *buf = hex_asc_upper_lo!(byte);
    buf = buf.add(1);
    buf
}

extern "C" {
    pub fn hex_to_bin(ch: u8) -> i32;
    pub fn hex2bin(dst: *mut u8, src: *const c_char, count: usize) -> i32;
    pub fn bin2hex(dst: *mut c_char, src: *const core::ffi::c_void, count: usize) -> *mut c_char;
    pub fn mac_pton(s: *const c_char, mac: *mut u8) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
