// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook
//
// C dependencies removed from executable Rust:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut count: i32 = 0;

extern "C" {
    static START_CHAR: i32;

    fn bpf_seq_write(seq: *mut seq_file, data: *const core::ffi::c_void, len: u64) -> i64;
}

#[repr(C)]
pub struct bpf_iter__task {
    pub meta: *mut bpf_iter_meta,
}

#[repr(C)]
pub struct bpf_iter_meta {
    pub seq: *mut seq_file,
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[no_mangle]
#[link_section = "iter/task"]
pub unsafe extern "C" fn dump_task(ctx: *mut bpf_iter__task) -> i32 {
    let seq: *mut seq_file = (*(*ctx).meta).seq;
    let mut c: i8;

    if count < 4 {
        c = (START_CHAR + count) as i8;
        bpf_seq_write(
            seq,
            &mut c as *mut i8 as *const core::ffi::c_void,
            core::mem::size_of_val(&c) as u64,
        );
        count += 1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
