// SPDX-License-Identifier: GPL-2.0-only
/* -*- linux-c -*- ------------------------------------------------------- *
 *
 *   Copyright (C) 1991, 1992 Linus Torvalds
 *   Copyright 2007-2008 rPath, Inc. - All Rights Reserved
 *
 * ----------------------------------------------------------------------- */

/*
 * arch/x86/boot/cpu.c
 *
 * Check for obligatory CPU features and abort if the features are not
 * present.
 */

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static x86_cap_strs: c_void;
    static NCAPINTS: c_int;

    fn check_cpu(cpu_level: *mut c_int, req_level: *mut c_int,
                 err_flags: *mut *mut u32);
    fn check_knl_erratum() -> c_int;
    fn printf(format: *const c_char, ...);
    fn sprintf(buf: *mut c_char, format: *const c_char, ...);
    fn puts(s: *const c_char) -> c_int;
    fn putchar(c: c_int) -> c_int;
}

unsafe fn cpu_name(level: c_int) -> *const c_char {
    static mut BUF: [c_char; 6] = [0; 6];

    if level == 64 {
        b"x86-64\0".as_ptr() as *const c_char
    } else {
        let mut level = level;
        if level == 15 {
            level = 6;
        }
        sprintf(BUF.as_mut_ptr(), b"i%d86\0".as_ptr() as *const c_char, level);
        BUF.as_ptr()
    }
}

unsafe fn show_cap_strs(err_flags: *mut u32) {
    let mut i: c_int = 0;
    let msg_strs = &x86_cap_strs as *const c_void as *const u8;
    let mut msg_strs = msg_strs;
    while i < NCAPINTS {
        let mut e = *err_flags.add(i as usize);
        let mut j: c_int = 0;
        while j < 32 {
            if *msg_strs < i as u8 || (*msg_strs == i as u8 && *msg_strs.add(1) < j as u8) {
                msg_strs = msg_strs.add(2);
                while *msg_strs != 0 {
                    msg_strs = msg_strs.add(1);
                }
                msg_strs = msg_strs.add(1);
            }
            if e & 1 != 0 {
                if *msg_strs == i as u8 && *msg_strs.add(1) == j as u8 && *msg_strs.add(2) != 0 {
                    printf(b"%s \0".as_ptr() as *const c_char, msg_strs.add(2));
                } else {
                    printf(b"%d:%d \0".as_ptr() as *const c_char, i, j);
                }
            }
            e >>= 1;
            j += 1;
        }
        i += 1;
    }
}

pub unsafe fn validate_cpu() -> c_int {
    let mut err_flags: *mut u32 = core::ptr::null_mut();
    let mut cpu_level: c_int = 0;
    let mut req_level: c_int = 0;

    check_cpu(&mut cpu_level, &mut req_level, &mut err_flags);

    if cpu_level < req_level {
        printf(b"This kernel requires an %s CPU, \0".as_ptr() as *const c_char,
               cpu_name(req_level));
        printf(b"but only detected an %s CPU.\n\0".as_ptr() as *const c_char,
               cpu_name(cpu_level));
        return -1;
    }

    if !err_flags.is_null() {
        puts(b"This kernel requires the following features not present on the CPU:\n\0"
             .as_ptr() as *const c_char);
        show_cap_strs(err_flags);
        putchar(b'\n' as c_int);
        -1
    } else if check_knl_erratum() != 0 {
        -1
    } else {
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
