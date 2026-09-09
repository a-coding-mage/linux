// SPDX-License-Identifier: GPL-2.0+
/* genmap.c
 * originally written by: Kirk Reiser.
 *
 ** Copyright (C) 2002  Kirk Reiser.
 *  Copyright (C) 2003  David Borowski.
 */

use std::ffi::{c_char, c_int, c_void, CStr};
use std::ptr;

const MAXKEYVAL: usize = 256; // Supplied by utils.h.

#[repr(C)]
struct StKeyInit {
    name: *mut c_char,
    value: c_int,
    shift: c_int,
}

#[repr(C)]
struct StKey {
    value: c_int,
    shift: c_int,
}

extern "C" {
    static mut key_table: [StKey; MAXKEYVAL];
    static mut init_key_data: StKeyInit;
    static mut infile: *mut libc::FILE;
    static mut lc: c_int;
    static is_shift: c_int;
    static is_input: c_int;
    static is_spk: c_int;

    fn oops(message: *const c_char, argument: *const c_char) -> !;
    fn add_key(name: *mut c_char, value: c_int, shift: c_int);
    fn open_input(name: *const c_char, filename: *mut c_char);
    fn find_key(name: *mut c_char) -> *mut StKey;
}

static mut key_data: [[u8; 16]; MAXKEYVAL] = [[0; 16]; MAXKEYVAL];
static mut kp: *mut u8 = ptr::null_mut();

static delims: &[u8] = b"\t\n \0";
static mut cp: *mut c_char = ptr::null_mut();
static mut map_ver: c_int = 119; /* an arbitrary number so speakup can check */
static mut shift_table: [c_int; 17] = [0; 17];
static mut max_states: c_int = 1;
static mut flags: c_int = 0;
/* flags reserved for later, maybe for individual console maps */

extern "C" {
    fn fgets(buffer: *mut c_char, size: c_int, stream: *mut libc::FILE) -> *mut c_char;
    fn fclose(stream: *mut libc::FILE) -> c_int;
    fn strtok(string: *mut c_char, delimiters: *const c_char) -> *mut c_char;
    fn memcmp(first: *const c_void, second: *const c_void, count: usize) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fputs(string: *const c_char, stream: *mut libc::FILE) -> c_int;
    static mut stderr: *mut libc::FILE;
    fn exit(status: c_int) -> !;
}

unsafe fn get_shift_value(state: c_int) -> c_int {
    let mut i = 0;
    while shift_table[i as usize] != state {
        if shift_table[i as usize] == -1 {
            if i >= 16 {
                oops(b"too many shift states\0".as_ptr() as *const c_char, ptr::null());
            }
            shift_table[i as usize] = state;
            max_states = i + 1;
            break;
        }
        i += 1;
    }
    i
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut value: c_int;
    let mut shift_state: c_int;
    let mut i: c_int;
    let mut spk_val: c_int = 0;
    let mut lock_val: c_int = 0;
    let mut max_key_used: c_int = 0;
    let mut num_keys_used: c_int = 0;
    let mut this: *mut StKey;
    let mut p_init: *mut StKeyInit;
    let mut buffer = [0 as c_char; 256];

    ptr::write_bytes(key_table.as_mut_ptr(), 0, key_table.len());
    ptr::write_bytes(key_data.as_mut_ptr(), 0, key_data.len());

    shift_table[0] = 0;
    for n in 1..=16 {
        shift_table[n] = -1;
    }

    if argc < 2 {
        fputs(b"usage: genmap filename\n\0".as_ptr() as *const c_char, stderr);
        exit(1);
    }

    p_init = &mut init_key_data;
    while (*p_init).name.read() != b'.' as c_char {
        add_key((*p_init).name, (*p_init).value, (*p_init).shift);
        p_init = p_init.add(1);
    }

    open_input(ptr::null(), *argv.add(1));
    while !fgets(buffer.as_mut_ptr(), buffer.len() as c_int, infile).is_null() {
        value = 0;
        shift_state = 0;

        cp = strtok(buffer.as_mut_ptr(), delims.as_ptr() as *const c_char);
        if !cp.is_null() && *cp == b'#' as c_char {
            continue;
        }

        while !cp.is_null() {
            if *cp == b'=' as c_char { break; }
            this = find_key(cp);
            if this.is_null() { oops(b"unknown key/modifier\0".as_ptr() as *const c_char, cp); }
            if (*this).shift == is_shift {
                if value != 0 { oops(b"modifiers must come first\0".as_ptr() as *const c_char, cp); }
                shift_state += (*this).value;
            } else if (*this).shift == is_input {
                value = (*this).value;
            } else { oops(b"bad modifier or key\0".as_ptr() as *const c_char, cp); }
            cp = strtok(ptr::null_mut(), delims.as_ptr() as *const c_char);
        }
        if cp.is_null() { oops(b"no = found\0".as_ptr() as *const c_char, ptr::null()); }
        cp = strtok(ptr::null_mut(), delims.as_ptr() as *const c_char);
        if cp.is_null() { oops(b"no speakup function after =\0".as_ptr() as *const c_char, ptr::null()); }
        this = find_key(cp);
        if this.is_null() || (*this).shift != is_spk { oops(b"invalid speakup function\0".as_ptr() as *const c_char, cp); }
        i = get_shift_value(shift_state);
        if key_data[value as usize][i as usize] != 0 {
            while { cp = cp.sub(1); cp > buffer.as_mut_ptr() } { if *cp == 0 { *cp = b' ' as c_char; } }
            oops(b"two functions on same key combination\0".as_ptr() as *const c_char, cp);
        }
        key_data[value as usize][i as usize] = (*this).value as u8;
        if value > max_key_used { max_key_used = value; }
    }
    fclose(infile);

    this = find_key(b"spk_key\0".as_ptr() as *mut c_char);
    if !this.is_null() { spk_val = (*this).value; }
    this = find_key(b"spk_lock\0".as_ptr() as *mut c_char);
    if !this.is_null() { lock_val = (*this).value; }

    lc = 1;
    while lc <= max_key_used {
        kp = key_data[lc as usize].as_mut_ptr();
        if memcmp(key_data[0].as_ptr() as *const c_void, kp as *const c_void, 16) == 0 { lc += 1; continue; }
        num_keys_used += 1;
        i = 0;
        while i < max_states {
            if *kp.add(i as usize) == spk_val as u8 || *kp.add(i as usize) == lock_val as u8 {
                shift_state = shift_table[i as usize];
                if shift_state & 16 == 0 {
                    shift_state = get_shift_value(shift_state + 16);
                    *kp.add(shift_state as usize) = *kp.add(i as usize);
                }
            }
            i += 1;
        }
        lc += 1;
    }

    printf(b"\t%d, %d, %d,\n\t\0".as_ptr() as *const c_char, map_ver, num_keys_used, max_states);
    for n in 0..max_states { printf(b"%d, \0".as_ptr() as *const c_char, shift_table[n as usize]); }
    printf(b"%d,\0".as_ptr() as *const c_char, flags);
    lc = 1;
    while lc <= max_key_used {
        kp = key_data[lc as usize].as_mut_ptr();
        if memcmp(key_data[0].as_ptr() as *const c_void, kp as *const c_void, 16) != 0 {
            printf(b"\n\t%d,\0".as_ptr() as *const c_char, lc);
            for n in 0..max_states { printf(b" %u,\0".as_ptr() as *const c_char, *kp.add(n as usize) as c_int); }
        }
        lc += 1;
    }
    printf(b"\n\t0, %d\n\0".as_ptr() as *const c_char, map_ver);
    exit(0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
