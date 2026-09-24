/* SPDX-License-Identifier: GPL-2.0+ */
/* utils.h
 * originally written by: Kirk Reiser.
 *
 ** Copyright (C) 2002  Kirk Reiser.
 *  Copyright (C) 2003  David Borowski.
 */

// Keep the original helper names; each generator uses a different subset.
#![allow(non_upper_case_globals, non_camel_case_types)]

// C dependency: stdio.h and the C character/string/allocator routines.
use core::ffi::{c_char, c_int, c_uchar};

pub(super) const MAXKEYS: usize = 512;
pub(super) const MAXKEYVAL: usize = 160;
pub(super) const HASHSIZE: usize = 101;
pub(super) const IS_SHIFT: c_int = -3;
pub(super) const IS_SPK: c_int = -2;
pub(super) const IS_INPUT: c_int = -1;

#[repr(C)]
pub(super) struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(super) struct st_key {
    pub(super) name: *mut c_char,
    pub(super) next: *mut st_key,
    pub(super) value: c_int,
    pub(super) shift: c_int,
}

unsafe extern "C" {
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn isupper(c: c_int) -> c_int;
    fn tolower(c: c_int) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
}

pub(super) static mut key_table: [st_key; MAXKEYS] = [st_key {
    name: core::ptr::null_mut(),
    next: core::ptr::null_mut(),
    value: 0,
    shift: 0,
}; MAXKEYS];
pub(super) static mut extra_keys: *mut st_key = unsafe {
    core::ptr::addr_of_mut!(key_table)
        .cast::<st_key>()
        .add(HASHSIZE)
};
// Only makemapdata parses header defines; genmap shares the other helpers.
#[allow(dead_code)]
pub(super) static mut def_name: *mut c_char = core::ptr::null_mut();
#[allow(dead_code)]
pub(super) static mut def_val: *mut c_char = core::ptr::null_mut();
pub(super) static mut infile: *mut FILE = core::ptr::null_mut();
pub(super) static mut lc: c_int = 0;
pub(super) static mut filename: [c_char; 256] = [0; 256];

#[inline]
pub(super) unsafe fn open_input(dir_name: *const c_char, name: *const c_char) {
    // SAFETY: callers provide live C strings/streams and exclusive access to
    // the single-threaded parser state; pointers follow the original buffers.
    unsafe {
        if !dir_name.is_null() {
            snprintf(
                core::ptr::addr_of_mut!(filename).cast::<c_char>(),
                256,
                b"%s/%s\0".as_ptr() as *const c_char,
                dir_name,
                name,
            );
        } else {
            snprintf(
                core::ptr::addr_of_mut!(filename).cast::<c_char>(),
                256,
                b"%s\0".as_ptr() as *const c_char,
                name,
            );
        }
        infile = fopen(
            core::ptr::addr_of!(filename).cast::<c_char>(),
            b"r\0".as_ptr() as *const c_char,
        );
        if infile.is_null() {
            fprintf(
                stderr(),
                b"can't open %s\n\0".as_ptr() as *const c_char,
                core::ptr::addr_of!(filename).cast::<c_char>(),
            );
            exit(1);
        }
        lc = 0;
    }
}

#[inline]
pub(super) unsafe fn oops(msg: *const c_char, mut info: *const c_char) -> ! {
    // SAFETY: callers provide live C strings/streams and exclusive access to
    // the single-threaded parser state; pointers follow the original buffers.
    unsafe {
        if info.is_null() {
            info = b"\0".as_ptr() as *const c_char;
        }
        fprintf(
            stderr(),
            b"error: file %s line %d\n\0".as_ptr() as *const c_char,
            core::ptr::addr_of!(filename).cast::<c_char>(),
            lc,
        );
        fprintf(stderr(), b"%s %s\n\0".as_ptr() as *const c_char, msg, info);
        exit(1);
    }
}

#[inline]
pub(super) unsafe fn hash_name(name: *mut c_char) -> *mut st_key {
    // SAFETY: callers provide live C strings/streams and exclusive access to
    // the single-threaded parser state; pointers follow the original buffers.
    unsafe {
        let mut pn = name as *mut c_uchar;
        let mut hash: c_int = 0;
        while *pn != 0 {
            // Retain the C host's wrapping arithmetic before its 28-bit mask.
            hash = hash.wrapping_mul(17) & 0x0fffffff;
            if isupper(*pn as c_int) != 0 {
                *pn = tolower(*pn as c_int) as c_uchar;
            }
            hash += *pn as c_int;
            pn = pn.add(1);
        }
        hash %= HASHSIZE as c_int;
        core::ptr::addr_of_mut!(key_table)
            .cast::<st_key>()
            .add(hash as usize)
    }
}

#[inline]
pub(super) unsafe fn find_key(name: *mut c_char) -> *mut st_key {
    // SAFETY: callers provide live C strings/streams and exclusive access to
    // the single-threaded parser state; pointers follow the original buffers.
    unsafe {
        let mut this = hash_name(name);
        while !this.is_null() {
            if !(*this).name.is_null() && strcmp(name, (*this).name) == 0 {
                return this;
            }
            this = (*this).next;
        }
        this
    }
}

#[inline]
pub(super) unsafe fn add_key(name: *mut c_char, value: c_int, shift: c_int) -> *mut st_key {
    // SAFETY: callers provide live C strings/streams and exclusive access to
    // the single-threaded parser state; pointers follow the original buffers.
    unsafe {
        let mut this = hash_name(name);
        if extra_keys.offset_from(core::ptr::addr_of_mut!(key_table).cast::<st_key>())
            >= MAXKEYS as isize
        {
            oops(
                b"out of key table space, enlarge MAXKEYS\0".as_ptr() as *const c_char,
                core::ptr::null(),
            );
        }
        if !(*this).name.is_null() {
            while !(*this).next.is_null() {
                if strcmp(name, (*this).name) == 0 {
                    oops(
                        b"attempt to add duplicate key\0".as_ptr() as *const c_char,
                        name,
                    );
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
}

#[inline]
unsafe fn stderr() -> *mut FILE {
    // SAFETY: callers provide live C strings/streams and exclusive access to
    // the single-threaded parser state; pointers follow the original buffers.
    unsafe {
        // Supplied by the C runtime; retained as an external dependency.
        extern "C" {
            static mut stderr: *mut FILE;
        }
        stderr
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
