// SPDX-License-Identifier: GPL-2.0+
/* genmap.c
 * originally written by: Kirk Reiser.
 *
 ** Copyright (C) 2002  Kirk Reiser.
 *  Copyright (C) 2003  David Borowski.
 */

//! Generate Speakup keyboard data using the original host C interfaces.

// C startup preserves inherited SIGPIPE, argv bytes, and libc environment.
#![no_main]
#![allow(non_upper_case_globals)]

#[path = "utils_header.rs"]
mod utils;
use std::ffi::{c_char, c_int, c_void};
use std::ptr;
use utils::*;

// The selected C frontend validates the original source and materializes only
// initializer data. This format contains little-endian lengths/integers and
// exact NUL-terminated name bytes, with no C source interpretation in Rust.
const MAPDATA: &[u8] = include_bytes!(env!("SPEAKUP_MAPDATA"));

unsafe fn init_keys() {
    // SAFETY: each validated record owns a terminated name; add_key copies it.
    unsafe {
        assert_eq!(&MAPDATA[..8], b"SKMP0001", "frontend mapdata format");
        let mut data = &MAPDATA[8..];
        while !data.is_empty() {
            let length = u32::from_le_bytes(data[..4].try_into().unwrap()) as usize;
            let value = i32::from_le_bytes(data[4..8].try_into().unwrap());
            let shift = i32::from_le_bytes(data[8..12].try_into().unwrap());
            let mut name = data[12..12 + length].to_vec();
            assert_eq!(name.last(), Some(&0), "frontend name terminator");
            add_key(name.as_mut_ptr().cast(), value, shift);
            data = &data[12 + length..];
        }
    }
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
    fn fgets(buffer: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fclose(stream: *mut FILE) -> c_int;
    fn strtok(string: *mut c_char, delimiters: *const c_char) -> *mut c_char;
    fn memcmp(first: *const c_void, second: *const c_void, count: usize) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fputs(string: *const c_char, stream: *mut FILE) -> c_int;
    static mut stderr: *mut FILE;
    fn exit(status: c_int) -> !;
}

unsafe fn get_shift_value(state: c_int) -> c_int {
    // SAFETY: callers provide live C strings/streams and exclusive access to
    // the single-threaded parser state; pointers follow the original buffers.
    unsafe {
        let mut i = 0;
        while shift_table[i as usize] != state {
            if shift_table[i as usize] == -1 {
                if i >= 16 {
                    oops(
                        b"too many shift states\0".as_ptr() as *const c_char,
                        ptr::null(),
                    );
                }
                shift_table[i as usize] = state;
                max_states = i + 1;
                break;
            }
            i += 1;
        }
        i
    }
}

#[no_mangle]
extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    // SAFETY: C startup supplies argc live NUL-terminated argument strings.
    // Forward them directly, including non-UTF8 paths, without Rust startup.
    unsafe { run(argc, argv) };
    0
}

unsafe fn run(argc: c_int, argv: *mut *mut c_char) {
    // SAFETY: callers provide live C strings/streams and exclusive access to
    // the single-threaded parser state; pointers follow the original buffers.
    unsafe {
        let mut value: c_int;
        let mut shift_state: c_int;
        let mut i: c_int;
        let mut spk_val: c_int = 0;
        let mut lock_val: c_int = 0;
        let mut max_key_used: c_int = 0;
        let mut num_keys_used: c_int = 0;
        let mut this: *mut st_key;
        let mut buffer = [0 as c_char; 256];

        shift_table[0] = 0;
        for n in 1..=16 {
            shift_table[n] = -1;
        }

        if argc < 2 {
            fputs(
                b"usage: genmap filename\n\0".as_ptr() as *const c_char,
                stderr,
            );
            exit(1);
        }

        init_keys();

        open_input(ptr::null(), *argv.add(1));
        // Unlike makemapdata, the original genmap does not increment lc here:
        // parse diagnostics therefore intentionally retain line zero.
        while !fgets(buffer.as_mut_ptr(), buffer.len() as c_int, infile).is_null() {
            value = 0;
            shift_state = 0;

            cp = strtok(buffer.as_mut_ptr(), delims.as_ptr() as *const c_char);
            if !cp.is_null() && *cp == b'#' as c_char {
                continue;
            }

            while !cp.is_null() {
                if *cp == b'=' as c_char {
                    break;
                }
                this = find_key(cp);
                if this.is_null() {
                    oops(b"unknown key/modifier\0".as_ptr() as *const c_char, cp);
                }
                if (*this).shift == IS_SHIFT {
                    if value != 0 {
                        oops(b"modifiers must come first\0".as_ptr() as *const c_char, cp);
                    }
                    shift_state += (*this).value;
                } else if (*this).shift == IS_INPUT {
                    value = (*this).value;
                } else {
                    oops(b"bad modifier or key\0".as_ptr() as *const c_char, cp);
                }
                cp = strtok(ptr::null_mut(), delims.as_ptr() as *const c_char);
            }
            if cp.is_null() {
                oops(b"no = found\0".as_ptr() as *const c_char, ptr::null());
            }
            cp = strtok(ptr::null_mut(), delims.as_ptr() as *const c_char);
            if cp.is_null() {
                oops(
                    b"no speakup function after =\0".as_ptr() as *const c_char,
                    ptr::null(),
                );
            }
            this = find_key(cp);
            if this.is_null() || (*this).shift != IS_SPK {
                oops(b"invalid speakup function\0".as_ptr() as *const c_char, cp);
            }
            i = get_shift_value(shift_state);
            if key_data[value as usize][i as usize] != 0 {
                while {
                    cp = cp.sub(1);
                    cp > buffer.as_mut_ptr()
                } {
                    if *cp == 0 {
                        *cp = b' ' as c_char;
                    }
                }
                oops(
                    b"two functions on same key combination\0".as_ptr() as *const c_char,
                    cp,
                );
            }
            key_data[value as usize][i as usize] = (*this).value as u8;
            if value > max_key_used {
                max_key_used = value;
            }
        }
        fclose(infile);

        this = find_key(b"spk_key\0".as_ptr() as *mut c_char);
        if !this.is_null() {
            spk_val = (*this).value;
        }
        this = find_key(b"spk_lock\0".as_ptr() as *mut c_char);
        if !this.is_null() {
            lock_val = (*this).value;
        }

        lc = 1;
        while lc <= max_key_used {
            kp = key_data[lc as usize].as_mut_ptr();
            if memcmp(
                key_data[0].as_ptr() as *const c_void,
                kp as *const c_void,
                16,
            ) == 0
            {
                lc += 1;
                continue;
            }
            num_keys_used += 1;
            i = 0;
            while i < max_states {
                if *kp.add(i as usize) as c_int == spk_val
                    || *kp.add(i as usize) as c_int == lock_val
                {
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

        printf(
            b"\t%d, %d, %d,\n\t\0".as_ptr() as *const c_char,
            map_ver,
            num_keys_used,
            max_states,
        );
        for n in 0..max_states {
            printf(b"%d, \0".as_ptr() as *const c_char, shift_table[n as usize]);
        }
        printf(b"%d,\0".as_ptr() as *const c_char, flags);
        lc = 1;
        while lc <= max_key_used {
            kp = key_data[lc as usize].as_mut_ptr();
            if memcmp(
                key_data[0].as_ptr() as *const c_void,
                kp as *const c_void,
                16,
            ) != 0
            {
                printf(b"\n\t%d,\0".as_ptr() as *const c_char, lc);
                for n in 0..max_states {
                    printf(
                        b" %u,\0".as_ptr() as *const c_char,
                        *kp.add(n as usize) as c_int,
                    );
                }
            }
            lc += 1;
        }
        printf(b"\n\t0, %d\n\0".as_ptr() as *const c_char, map_ver);
        exit(0);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
