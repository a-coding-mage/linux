// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub tgid: ::core::ffi::c_int,
}

#[repr(C)]
pub struct bpf_iter_meta {
    pub seq: *mut seq_file,
}

#[repr(C)]
pub struct bpf_iter__task {
    pub meta: *mut bpf_iter_meta,
    pub task: *mut task_struct,
}

unsafe extern "C" {
    fn bpf_seq_write(
        seq: *mut seq_file,
        data: *const ::core::ffi::c_void,
        len: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(link_section = "iter/task")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dump_task(ctx: *mut bpf_iter__task) -> ::core::ffi::c_int {
    let seq: *mut seq_file = unsafe { (*(*ctx).meta).seq };
    let task: *mut task_struct = unsafe { (*ctx).task };
    let tgid: ::core::ffi::c_int;

    tgid = unsafe { (*task).tgid };
    unsafe {
        bpf_seq_write(
            seq,
            &tgid as *const _ as *const ::core::ffi::c_void,
            ::core::mem::size_of_val(&tgid) as ::core::ffi::c_ulong,
        );
    }
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
