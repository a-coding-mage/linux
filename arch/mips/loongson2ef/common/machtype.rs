// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2009 Lemote Inc.
 * Author: Wu Zhangjin, wuzhangjin@gmail.com
 *
 * Copyright (c) 2009 Zhang Le <r0bertz@gentoo.org>
 */

use core::ffi::c_char;

/* Dependency declarations supplied by the surrounding kernel sources. */
extern "C" {
    static mut mips_machtype: i32;
    static arcs_cmdline: *const c_char;
    static LOONGSON_MACHTYPE: i32;

    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn strscpy(dst: *mut c_char, src: *const c_char);

    fn mach_prom_init_machtype();
}

/* please ensure the length of the machtype string is less than 50 */
const MACHTYPE_LEN: usize = 50;

/* Values are supplied by <asm/bootinfo.h> and <machine.h>. */
extern "C" {
    static MACH_LOONGSON_UNKNOWN: i32;
    static MACH_LEMOTE_FL2E: i32;
    static MACH_LEMOTE_FL2F: i32;
    static MACH_LEMOTE_ML2F7: i32;
    static MACH_LEMOTE_YL2F89: i32;
    static MACH_DEXXON_GDIUM2F10: i32;
    static MACH_LEMOTE_NAS: i32;
    static MACH_LEMOTE_LL2F: i32;
    static MACH_LOONGSON_END: i32;
}

static system_types: [*const c_char; 9] = [
    b"unknown loongson machine\0".as_ptr() as *const c_char,
    b"lemote-fuloong-2e-box\0".as_ptr() as *const c_char,
    b"lemote-fuloong-2f-box\0".as_ptr() as *const c_char,
    b"lemote-mengloong-2f-7inches\0".as_ptr() as *const c_char,
    b"lemote-yeeloong-2f-8.9inches\0".as_ptr() as *const c_char,
    b"dexxon-gdium-2f\0".as_ptr() as *const c_char,
    b"lemote-nas-2f\0".as_ptr() as *const c_char,
    b"lemote-lynloong-2f\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

pub unsafe fn get_system_type() -> *const c_char {
    system_types[mips_machtype as usize]
}

pub unsafe fn prom_init_machtype() {
    let mut str_buf = [0 as c_char; MACHTYPE_LEN + 1];
    let mut machtype = MACH_LEMOTE_FL2E;

    mips_machtype = LOONGSON_MACHTYPE;

    let mut p = strstr(arcs_cmdline, b"machtype=\0".as_ptr() as *const c_char);
    if p.is_null() {
        mach_prom_init_machtype();
        return;
    }
    p = p.add(strlen(b"machtype=\0".as_ptr() as *const c_char));
    strscpy(str_buf.as_mut_ptr(), p);
    p = strstr(str_buf.as_ptr(), b" \0".as_ptr() as *const c_char);
    if !p.is_null() {
        *p = 0;
    }

    while !system_types[machtype as usize].is_null() {
        if !strstr(system_types[machtype as usize], str_buf.as_ptr()).is_null() {
            mips_machtype = machtype;
            break;
        }
        machtype += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
