// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

/* Original C dependencies:
 * #include "vmlinux.h"
 * #include <bpf/bpf_helpers.h>
 * #include <bpf/bpf_tracing.h>
 * #include "bpf_kfuncs.h"
 */

pub const SHA256_DIGEST_SIZE: usize = 32;
pub const SIZEOF_STRUCT_FSVERITY_DIGEST: usize = 4; /* sizeof(struct fsverity_digest) */

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_dynptr {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn bpf_get_current_pid_tgid() -> u64;
    pub fn bpf_dynptr_from_mem(
        data: *mut core::ffi::c_void,
        size: u32,
        flags: u64,
        ptr: *mut bpf_dynptr,
    ) -> i32;
    pub fn bpf_get_fsverity_digest(f: *mut file, digest_ptr: *mut bpf_dynptr) -> i32;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut expected_digest: [i8; SIZEOF_STRUCT_FSVERITY_DIGEST + SHA256_DIGEST_SIZE] =
    [0; SIZEOF_STRUCT_FSVERITY_DIGEST + SHA256_DIGEST_SIZE];
#[unsafe(no_mangle)]
pub static mut digest: [i8; SIZEOF_STRUCT_FSVERITY_DIGEST + SHA256_DIGEST_SIZE] =
    [0; SIZEOF_STRUCT_FSVERITY_DIGEST + SHA256_DIGEST_SIZE];
#[unsafe(no_mangle)]
pub static mut monitored_pid: u32 = 0;
#[unsafe(no_mangle)]
pub static mut got_fsverity: u32 = 0;
#[unsafe(no_mangle)]
pub static mut digest_matches: u32 = 0;

#[unsafe(link_section = "lsm.s/file_open")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_file_open(f: *mut file) -> i32 {
    let mut digest_ptr: bpf_dynptr = core::mem::zeroed();
    let pid: u32;
    let ret: i32;
    let mut i: i32;

    pid = (bpf_get_current_pid_tgid() >> 32) as u32;
    if pid != monitored_pid {
        return 0;
    }

    bpf_dynptr_from_mem(
        core::ptr::addr_of_mut!(digest) as *mut core::ffi::c_void,
        core::mem::size_of_val(&*core::ptr::addr_of!(digest)) as u32,
        0,
        &mut digest_ptr,
    );
    ret = bpf_get_fsverity_digest(f, &mut digest_ptr);
    if ret < 0 {
        return 0;
    }
    got_fsverity = 1;

    i = 0;
    while i < core::mem::size_of_val(&*core::ptr::addr_of!(digest)) as i32 {
        if digest[i as usize] != expected_digest[i as usize] {
            return 0;
        }
        i += 1;
    }

    digest_matches = 1;
    return 0;
}
