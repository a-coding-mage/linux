// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022, Oracle and/or its affiliates. */

/* Original C dependencies:
 * #include <vmlinux.h>
 * #include <bpf/bpf_helpers.h>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_ulong};

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_iter_meta {
    pub seq: *mut seq_file,
    pub session_id: u64,
    pub seq_num: u64,
}

#[repr(C)]
pub struct kallsym_iter {
    pub pos: c_ulong,
    pub pos_mod_end: c_ulong,
    pub pos_ftrace_mod_end: c_ulong,
    pub pos_bpf_end: c_ulong,
    pub value: c_ulong,
    pub name: [c_char; 128],
    pub module_name: [c_char; 56],
    pub exported: bool,
    pub show_value: bool,
    pub type_: c_char,
}

#[repr(C)]
pub struct bpf_iter__ksym {
    pub meta: *mut bpf_iter_meta,
    pub ksym: *mut kallsym_iter,
}

unsafe extern "C" {
    /* Rust stand-in for the BPF_SEQ_PRINTF helper macro supplied by bpf_helpers.h. */
    fn BPF_SEQ_PRINTF(seq: *mut seq_file, fmt: *const c_char, ...) -> c_int;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

#[unsafe(no_mangle)]
pub static mut last_sym_value: c_ulong = 0;

#[inline]
unsafe fn to_lower(mut c: c_char) -> c_char {
    if c >= b'A' as c_char && c <= b'Z' as c_char {
        c += (b'a' - b'A') as c_char;
    }
    c
}

#[inline]
unsafe fn to_upper(mut c: c_char) -> c_char {
    if c >= b'a' as c_char && c <= b'z' as c_char {
        c -= (b'a' - b'A') as c_char;
    }
    c
}

/* Dump symbols with max size; the latter is calculated by caching symbol N value
 * and when iterating on symbol N+1, we can print max size of symbol N via
 * address of N+1 - address of N.
 */
#[unsafe(link_section = "iter/ksym")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dump_ksym(ctx: *mut bpf_iter__ksym) -> c_int {
    let seq: *mut seq_file = (*(*ctx).meta).seq;
    let iter: *mut kallsym_iter = (*ctx).ksym;
    let seq_num: u32 = (*(*ctx).meta).seq_num as u32;
    let value: c_ulong;
    let mut type_: c_char;

    if iter.is_null() {
        return 0;
    }

    if seq_num == 0 {
        BPF_SEQ_PRINTF(
            seq,
            b"ADDR TYPE NAME MODULE_NAME KIND MAX_SIZE\n\0".as_ptr() as *const c_char,
        );
        return 0;
    }
    if last_sym_value != 0 {
        BPF_SEQ_PRINTF(
            seq,
            b"0x%x\n\0".as_ptr() as *const c_char,
            (*iter).value.wrapping_sub(last_sym_value),
        );
    } else {
        BPF_SEQ_PRINTF(seq, b"\n\0".as_ptr() as *const c_char);
    }

    value = if (*iter).show_value { (*iter).value } else { 0 };

    last_sym_value = value;

    type_ = (*iter).type_;

    if (*iter).module_name[0] != 0 {
        type_ = if (*iter).exported {
            to_upper(type_)
        } else {
            to_lower(type_)
        };
        BPF_SEQ_PRINTF(
            seq,
            b"0x%llx %c %s [ %s ] \0".as_ptr() as *const c_char,
            value as u64,
            type_ as c_int,
            (*iter).name.as_ptr(),
            (*iter).module_name.as_ptr(),
        );
    } else {
        BPF_SEQ_PRINTF(
            seq,
            b"0x%llx %c %s \0".as_ptr() as *const c_char,
            value as u64,
            type_ as c_int,
            (*iter).name.as_ptr(),
        );
    }
    if (*iter).pos_mod_end == 0 || (*iter).pos_mod_end > (*iter).pos {
        BPF_SEQ_PRINTF(seq, b"MOD \0".as_ptr() as *const c_char);
    } else if (*iter).pos_ftrace_mod_end == 0 || (*iter).pos_ftrace_mod_end > (*iter).pos {
        BPF_SEQ_PRINTF(seq, b"FTRACE_MOD \0".as_ptr() as *const c_char);
    } else if (*iter).pos_bpf_end == 0 || (*iter).pos_bpf_end > (*iter).pos {
        BPF_SEQ_PRINTF(seq, b"BPF \0".as_ptr() as *const c_char);
    } else {
        BPF_SEQ_PRINTF(seq, b"KPROBE \0".as_ptr() as *const c_char);
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
