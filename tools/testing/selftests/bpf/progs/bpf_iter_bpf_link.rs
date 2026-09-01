// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Red Hat, Inc. */
// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_iter_meta {
    pub seq: *mut seq_file,
}

#[repr(C)]
pub struct bpf_link {
    pub id: u32,
}

#[repr(C)]
pub struct bpf_iter__bpf_link {
    pub meta: *mut bpf_iter_meta,
    pub link: *mut bpf_link,
}

extern "C" {
    fn bpf_seq_write(seq: *mut seq_file, data: *const c_void, len: u64) -> c_int;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
#[link_section = "iter/bpf_link"]
pub unsafe extern "C" fn dump_bpf_link(ctx: *mut bpf_iter__bpf_link) -> c_int {
    let seq: *mut seq_file = (*(*ctx).meta).seq;
    let link: *mut bpf_link = (*ctx).link;
    let link_id: c_int;

    if link.is_null() {
        return 0;
    }

    link_id = (*link).id as c_int;
    bpf_seq_write(
        seq,
        &link_id as *const c_int as *const c_void,
        core::mem::size_of_val(&link_id) as u64,
    );
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
