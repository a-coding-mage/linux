// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Google */
// Dependencies from the original C source:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

type u64 = core::ffi::c_ulonglong;

#[repr(C)]
pub struct kernfs_node {
    pub id: u64,
}

#[repr(C)]
pub struct cgroup {
    pub kn: *mut kernfs_node,
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_iter_meta {
    pub seq: *mut seq_file,
    pub seq_num: u64,
}

#[repr(C)]
pub struct bpf_iter__cgroup {
    pub meta: *mut bpf_iter_meta,
    pub cgroup: *mut cgroup,
}

unsafe extern "C" {
    fn BPF_SEQ_PRINTF(seq: *mut seq_file, fmt: *const core::ffi::c_char, ...);
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [core::ffi::c_char; 4] = [
    b'G' as core::ffi::c_char,
    b'P' as core::ffi::c_char,
    b'L' as core::ffi::c_char,
    0,
];

#[unsafe(no_mangle)]
pub static mut terminate_early: core::ffi::c_int = 0;

#[unsafe(no_mangle)]
pub static mut terminal_cgroup: u64 = 0;

#[inline]
unsafe fn cgroup_id(cgrp: *mut cgroup) -> u64 {
    unsafe { (*(*cgrp).kn).id }
}

#[unsafe(link_section = "iter/cgroup")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cgroup_id_printer(ctx: *mut bpf_iter__cgroup) -> core::ffi::c_int {
    let seq: *mut seq_file = unsafe { (*(*ctx).meta).seq };
    let cgrp: *mut cgroup = unsafe { (*ctx).cgroup };

    /* epilogue */
    if cgrp.is_null() {
        unsafe {
            BPF_SEQ_PRINTF(seq, c"epilogue\n".as_ptr());
        }
        return 0;
    }

    /* prologue */
    if unsafe { (*(*ctx).meta).seq_num == 0 } {
        unsafe {
            BPF_SEQ_PRINTF(seq, c"prologue\n".as_ptr());
        }
    }

    unsafe {
        BPF_SEQ_PRINTF(seq, c"%8llu\n".as_ptr(), cgroup_id(cgrp));
    }

    if unsafe { terminal_cgroup == cgroup_id(cgrp) } {
        return 1;
    }

    if unsafe { terminate_early != 0 } {
        1
    } else {
        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
