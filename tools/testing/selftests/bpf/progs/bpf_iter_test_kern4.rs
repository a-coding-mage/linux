// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
// Dependencies in the original C source:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type __u32 = u32;
pub type __u64 = u64;

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    pub id: __u32,
}

#[repr(C)]
pub struct bpf_iter_meta {
    pub seq: *mut seq_file,
    pub seq_num: __u64,
}

#[repr(C)]
pub struct bpf_iter__bpf_map {
    pub meta: *mut bpf_iter_meta,
    pub map: *mut bpf_map,
}

unsafe extern "C" {
    pub fn bpf_seq_write(seq: *mut seq_file, data: *const core::ffi::c_void, len: __u32) -> i64;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut map1_id: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut map2_id: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut map1_accessed: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut map2_accessed: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut map1_seqnum: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut map2_seqnum1: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut map2_seqnum2: __u64 = 0;

unsafe extern "C" {
    pub static print_len: __u32;
    pub static ret1: __u32;
}

#[unsafe(link_section = "iter/bpf_map")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dump_bpf_map(ctx: *mut bpf_iter__bpf_map) -> i32 {
    let seq: *mut seq_file = unsafe { (*(*ctx).meta).seq };
    let map: *mut bpf_map = unsafe { (*ctx).map };
    let seq_num: __u64;
    let mut i: i32;
    let mut ret: i32 = 0;

    if map == core::ptr::null_mut() {
        return 0;
    }

    /* only dump map1_id and map2_id */
    if unsafe { (*map).id != map1_id && (*map).id != map2_id } {
        return 0;
    }

    seq_num = unsafe { (*(*ctx).meta).seq_num };
    if unsafe { (*map).id == map1_id } {
        unsafe {
            map1_seqnum = seq_num;
            map1_accessed = map1_accessed.wrapping_add(1);
        }
    }

    if unsafe { (*map).id == map2_id } {
        unsafe {
            if map2_accessed == 0 {
                map2_seqnum1 = seq_num;
                if ret1 != 0 {
                    ret = 1;
                }
            } else {
                map2_seqnum2 = seq_num;
            }
            map2_accessed = map2_accessed.wrapping_add(1);
        }
    }

    /* fill seq_file buffer */
    i = 0;
    while i < unsafe { print_len as i32 } {
        unsafe {
            bpf_seq_write(
                seq,
                &seq_num as *const __u64 as *const core::ffi::c_void,
                core::mem::size_of_val(&seq_num) as __u32,
            );
        }
        i += 1;
    }

    ret
}
