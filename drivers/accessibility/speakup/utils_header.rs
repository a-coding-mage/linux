/* SPDX-License-Identifier: GPL-2.0+ */
/* utils.h
 * originally written by: Kirk Reiser.
 *
 ** Copyright (C) 2002  Kirk Reiser.
 *  Copyright (C) 2003  David Borowski.
 */

// C dependency: stdio.h and the C character/string/allocator routines.
use core::ffi::{c_char, c_int, c_uchar, c_void};

pub const MAXKEYS: usize = 512;
pub const MAXKEYVAL: usize = 160;
pub const HASHSIZE: usize = 101;
pub const IS_SHIFT: c_int = -3;
pub const IS_SPK: c_int = -2;
pub const IS_INPUT: c_int = -1;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_key {
    pub name: *mut c_char,
    pub next: *mut st_key,
    pub value: c_int,
    pub shift: c_int,
}

unsafe extern "C" {
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...);
    fn exit(status: c_int) -> !;
    fn isupper(c: c_int) -> c_int;
    fn tolower(c: c_int) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
}

pub static mut key_table: [st_key; MAXKEYS] = [st_key {
    name: core::ptr::null_mut(),
    next: core::ptr::null_mut(),
    value: 0,
    shift: 0,
}; MAXKEYS];
pub static mut extra_keys: *mut st_key = unsafe { key_table.as_mut_ptr().add(HASHSIZE) };
pub static mut def_name: *mut c_char = core::ptr::null_mut();
pub static mut def_val: *mut c_char = core::ptr::null_mut();
pub static mut infile: *mut FILE = core::ptr::null_mut();
pub static mut lc: c_int = 0;
pub static mut filename: [c_char; 256] = [0; 256];

#[inline]
pub unsafe fn open_input(dir_name: *const c_char, name: *const c_char) {
    if !dir_name.is_null() {
        snprintf(filename.as_mut_ptr(), filename.len(), b"%s/%s\0".as_ptr() as *const c_char, dir_name, name);
    } else {
        snprintf(filename.as_mut_ptr(), filename.len(), b"%s\0".as_ptr() as *const c_char, name);
    }
    infile = fopen(filename.as_ptr(), b"r\0".as_ptr() as *const c_char);
    if infile.is_null() {
        fprintf(stderr(), b"can't open %s\n\0".as_ptr() as *const c_char, filename.as_ptr());
        exit(1);
    }
    lc = 0;
}

#[inline]
pub unsafe fn oops(msg: *const c_char, mut info: *const c_char) -> ! {
    if info.is_null() {
        info = b"\0".as_ptr() as *const c_char;
    }
    fprintf(stderr(), b"error: file %s line %d\n\0".as_ptr() as *const c_char, filename.as_ptr(), lc);
    fprintf(stderr(), b"%s %s\n\0".as_ptr() as *const c_char, msg, info);
    exit(1);
}

#[inline]
pub unsafe fn hash_name(mut name: *mut c_char) -> *mut st_key {
    let mut pn = name as *mut c_uchar;
    let mut hash: c_int = 0;
    while *pn != 0 {
        hash = (hash * 17) & 0x0fffffff;
        if isupper(*pn as c_int) != 0 {
            *pn = tolower(*pn as c_int) as c_uchar;
        }
        hash += *pn as c_int;
        pn = pn.add(1);
    }
    hash %= HASHSIZE as c_int;
    key_table.as_mut_ptr().add(hash as usize)
}

#[inline]
pub unsafe fn find_key(name: *mut c_char) -> *mut st_key {
    let mut this = hash_name(name);
    while !this.is_null() {
        if !(*this).name.is_null() && strcmp(name, (*this).name) == 0 {
            return this;
        }
        this = (*this).next;
    }
    this
}

#[inline]
pub unsafe fn add_key(name: *mut c_char, value: c_int, shift: c_int) -> *mut st_key {
    let mut this = hash_name(name);
    if extra_keys.offset_from(key_table.as_mut_ptr()) >= MAXKEYS as isize {
        oops(b"out of key table space, enlarge MAXKEYS\0".as_ptr() as *const c_char, core::ptr::null());
    }
    if !(*this).name.is_null() {
        while !(*this).next.is_null() {
            if strcmp(name, (*this).name) == 0 {
                oops(b"attempt to add duplicate key\0".as_ptr() as *const c_char, name);
            }
            this = (*this).next;
        }
        (*this).next = extra_keys;
        extra_keys = extra_keys.add(1);
        this = (*this).next;
    }
    (*this).name = strdup(name);
    (*this).value = value;
    (*this).shift = shift;
    this
}

#[inline]
unsafe fn stderr() -> *mut FILE {
    // Supplied by the C runtime; retained as an external dependency.
    extern "C" {
        static mut stderr: *mut FILE;
    }
    stderr
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
