// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Google LLC. */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use core::ffi::{c_char, c_void};

type __u8 = u8;
type __u32 = u32;

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;

    /* extern const void schedule __ksym; */
    static schedule: c_void;
}

pub static mut pid: __u32 = 0;

pub static mut num_out: [c_char; 64] = [0; 64];
pub static mut num_ret: i64 = 0;

pub static mut ip_out: [c_char; 64] = [0; 64];
pub static mut ip_ret: i64 = 0;

pub static mut sym_out: [c_char; 64] = [0; 64];
pub static mut sym_ret: i64 = 0;

pub static mut addr_out: [c_char; 64] = [0; 64];
pub static mut addr_ret: i64 = 0;

pub static mut str_out: [c_char; 64] = [0; 64];
pub static mut str_ret: i64 = 0;

pub static mut over_out: [c_char; 6] = [0; 6];
pub static mut over_ret: i64 = 0;

pub static mut pad_out: [c_char; 10] = [0; 10];
pub static mut pad_ret: i64 = 0;

pub static mut noarg_out: [c_char; 64] = [0; 64];
pub static mut noarg_ret: i64 = 0;

pub static mut nobuf_ret: i64 = 0;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn handler(ctx: *const c_void) -> i32 {
    /* Convenient values to pretty-print */
    let ex_ipv4: [__u8; 4] = [127, 0, 0, 1];
    let ex_ipv6: [__u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    static str1: [c_char; 5] = [b's' as c_char, b't' as c_char, b'r' as c_char, b'1' as c_char, 0];
    static longstr: [c_char; 8] = [
        b'l' as c_char,
        b'o' as c_char,
        b'n' as c_char,
        b'g' as c_char,
        b's' as c_char,
        b't' as c_char,
        b'r' as c_char,
        0,
    ];

    if bpf_get_current_pid_tgid() as i32 != pid as i32 {
        return 0;
    }

    /* Integer types */
    num_ret = BPF_SNPRINTF!(
        num_out.as_mut_ptr(),
        core::mem::size_of_val(&num_out),
        b"%d %u %x %li %llu %lX\0".as_ptr() as *const c_char,
        -8i32,
        9u32,
        150u32,
        -424242i64,
        1337u64,
        0xDABBAD00u64
    );
    /* IP addresses */
    ip_ret = BPF_SNPRINTF!(
        ip_out.as_mut_ptr(),
        core::mem::size_of_val(&ip_out),
        b"%pi4 %pI6\0".as_ptr() as *const c_char,
        &ex_ipv4 as *const [__u8; 4],
        &ex_ipv6 as *const [__u8; 16]
    );
    /* Symbol lookup formatting */
    sym_ret = BPF_SNPRINTF!(
        sym_out.as_mut_ptr(),
        core::mem::size_of_val(&sym_out),
        b"%ps %pS %pB\0".as_ptr() as *const c_char,
        &schedule as *const c_void,
        &schedule as *const c_void,
        &schedule as *const c_void
    );
    /* Kernel pointers */
    addr_ret = BPF_SNPRINTF!(
        addr_out.as_mut_ptr(),
        core::mem::size_of_val(&addr_out),
        b"%pK %px %p\0".as_ptr() as *const c_char,
        0usize,
        0xFFFF00000ADD4E55usize,
        0xFFFF00000ADD4E55usize
    );
    /* Strings and single-byte character embedding */
    str_ret = BPF_SNPRINTF!(
        str_out.as_mut_ptr(),
        core::mem::size_of_val(&str_out),
        b"%s % 9c %+2c %-3c %04c %0c %+05s\0".as_ptr() as *const c_char,
        str1.as_ptr(),
        b'a' as i32,
        b'b' as i32,
        b'c' as i32,
        b'd' as i32,
        b'e' as i32,
        longstr.as_ptr()
    );
    /* Overflow */
    over_ret = BPF_SNPRINTF!(
        over_out.as_mut_ptr(),
        core::mem::size_of_val(&over_out),
        b"%%overflow\0".as_ptr() as *const c_char
    );
    /* Padding of fixed width numbers */
    pad_ret = BPF_SNPRINTF!(
        pad_out.as_mut_ptr(),
        core::mem::size_of_val(&pad_out),
        b"%5d %0900000X\0".as_ptr() as *const c_char,
        4i32,
        4u32
    );
    /* No args */
    noarg_ret = BPF_SNPRINTF!(
        noarg_out.as_mut_ptr(),
        core::mem::size_of_val(&noarg_out),
        b"simple case\0".as_ptr() as *const c_char
    );
    /* No buffer */
    nobuf_ret = BPF_SNPRINTF!(
        core::ptr::null_mut::<c_char>(),
        0usize,
        b"only interested in length %d\0".as_ptr() as *const c_char,
        60i32
    );

    return 0;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
