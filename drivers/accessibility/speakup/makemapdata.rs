// SPDX-License-Identifier: GPL-2.0+
/* makemapdata.c
 * originally written by: Kirk Reiser.
 *
 ** Copyright (C) 2002  Kirk Reiser.
 *  Copyright (C) 2003  David Borowski.
 */

use std::ffi::c_char;
use std::os::raw::{c_int, c_uint, c_void};
use std::ptr;

// Symbols supplied by utils.h and the C runtime.
#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct st_key {
    pub name: *mut c_char,
    pub value: c_int,
    pub shift: c_int,
    pub next: *mut st_key,
}

extern "C" {
    static mut infile: *mut FILE;
    static mut lc: c_int;
    static mut key_table: [st_key; HASHSIZE];

    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fclose(stream: *mut FILE) -> c_int;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn atoi(s: *const c_char) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;

    fn add_key(name: *mut c_char, value: c_int, kind: c_int);
    fn open_input(dir_name: *const c_char, file_name: *const c_char);
    fn find_key(name: *const c_char) -> *mut st_key;
}

extern "C" {
    static is_shift: c_int;
    static is_input: c_int;
    static is_spk: c_int;
}

// These values are supplied by utils.h in the original build.
const HASHSIZE: usize = 256;
const MAXKEYVAL: c_int = 0x1000;

static mut BUFFER: [c_char; 256] = [0; 256];

unsafe fn get_define() -> c_int {
    let mut c: *mut c_char;

    while !fgets(BUFFER.as_mut_ptr(), (BUFFER.len() - 1) as c_int, infile).is_null() {
        lc += 1;
        if libc_strncmp(BUFFER.as_ptr(), b"#define\0".as_ptr() as *const c_char, 7) != 0 {
            continue;
        }
        c = BUFFER.as_mut_ptr().add(7);
        while *c == b' ' as c_char || *c == b'\t' as c_char {
            c = c.add(1);
        }
        def_name = c;
        while *c != 0 && *c != b' ' as c_char && *c != b'\t' as c_char && *c != b'\n' as c_char {
            c = c.add(1);
        }
        if *c == 0 || *c == b'\n' as c_char {
            continue;
        }
        *c = 0;
        c = c.add(1);
        while *c == b' ' as c_char || *c == b'\t' as c_char || *c == b'(' as c_char {
            c = c.add(1);
        }
        def_val = c;
        while *c != 0 && *c != b'\n' as c_char && *c != b')' as c_char {
            c = c.add(1);
        }
        *c = 0;
        return 1;
    }
    fclose(infile);
    infile = ptr::null_mut();
    0
}

extern "C" {
    static mut def_name: *mut c_char;
    static mut def_val: *mut c_char;
}

unsafe fn libc_strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int {
    for i in 0..n {
        let av = *a.add(i) as i32;
        let bv = *b.add(i) as i32;
        if av != bv {
            return av - bv;
        }
        if av == 0 {
            break;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut value: c_int;
    let mut i: usize;
    let mut this: *mut st_key;
    let mut dir_name: *const c_char;
    let mut spk_dir_name: *const c_char;
    let mut cp: *mut c_char;

    dir_name = getenv(b"TOPDIR\0".as_ptr() as *const c_char);
    if dir_name.is_null() {
        dir_name = b".\0".as_ptr() as *const c_char;
    }
    spk_dir_name = getenv(b"SPKDIR\0".as_ptr() as *const c_char);
    if spk_dir_name.is_null() {
        spk_dir_name = b"drivers/accessibility/speakup\0".as_ptr() as *const c_char;
    }
    ptr::write_bytes(key_table.as_mut_ptr() as *mut u8, 0, std::mem::size_of_val(&key_table));
    add_key(b"shift\0".as_ptr() as *mut c_char, 1, is_shift);
    add_key(b"altgr\0".as_ptr() as *mut c_char, 2, is_shift);
    add_key(b"ctrl\0".as_ptr() as *mut c_char, 4, is_shift);
    add_key(b"alt\0".as_ptr() as *mut c_char, 8, is_shift);
    add_key(b"spk\0".as_ptr() as *mut c_char, 16, is_shift);
    add_key(b"double\0".as_ptr() as *mut c_char, 32, is_shift);

    open_input(dir_name, b"include/linux/input.h\0".as_ptr() as *const c_char);
    while get_define() != 0 {
        if libc_strncmp(def_name, b"KEY_\0".as_ptr() as *const c_char, 4) != 0 { continue; }
        value = atoi(def_val);
        if value > 0 && value < MAXKEYVAL { add_key(def_name, value, is_input); }
    }
    open_input(dir_name, b"include/uapi/linux/input-event-codes.h\0".as_ptr() as *const c_char);
    while get_define() != 0 {
        if libc_strncmp(def_name, b"KEY_\0".as_ptr() as *const c_char, 4) != 0 { continue; }
        value = atoi(def_val);
        if value > 0 && value < MAXKEYVAL { add_key(def_name, value, is_input); }
    }
    open_input(spk_dir_name, b"spk_priv_keyinfo.h\0".as_ptr() as *const c_char);
    while get_define() != 0 {
        if strlen(def_val) > 5 {
            cp = strchr(def_val, b'+' as c_int);
            if cp.is_null() { continue; }
            if *cp.sub(1) == b' ' as c_char { *cp.sub(1) = 0; }
            *cp = 0; cp = cp.add(1);
            this = find_key(def_val);
            while *cp == b' ' as c_char { cp = cp.add(1); }
            if this.is_null() || *cp < b'0' as c_char || *cp > b'9' as c_char { continue; }
            value = (*this).value + atoi(cp);
        } else if libc_strncmp(def_val, b"0x\0".as_ptr() as *const c_char, 2) == 0 {
            sscanf(def_val.add(2), b"%x\0".as_ptr() as *const c_char, &mut value);
        } else if *def_val >= b'0' as c_char && *def_val <= b'9' as c_char { value = atoi(def_val); }
        else { continue; }
        add_key(def_name, value, is_spk);
    }
    printf(b"struct st_key_init init_key_data[] = {\n\0".as_ptr() as *const c_char);
    for i in 0..HASHSIZE {
        this = &mut key_table[i];
        if (*this).name.is_null() { continue; }
        loop {
            printf(b"\t{ \"%s\", %d, %d, },\n\0".as_ptr() as *const c_char, (*this).name, (*this).value, (*this).shift);
            this = (*this).next;
            if this.is_null() { break; }
        }
    }
    printf(b"\t{ \".\", 0, 0 }\n};\n\0".as_ptr() as *const c_char);
    exit(0)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
