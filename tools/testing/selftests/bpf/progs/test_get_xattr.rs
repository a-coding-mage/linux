// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

/* Dependencies in the original C source:
 * vmlinux.h, errno.h, bpf/bpf_helpers.h, bpf/bpf_tracing.h,
 * bpf_kfuncs.h, and bpf_misc.h.
 */

type __u32 = u32;

const EINVAL: i32 = 22;

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_dynptr {
    _private: [u8; 0],
}

extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_dynptr_from_mem(
        data: *mut core::ffi::c_void,
        size: u32,
        flags: u64,
        ptr: *mut bpf_dynptr,
    ) -> i32;
    fn bpf_get_file_xattr(f: *mut file, name: *const u8, value_p: *mut bpf_dynptr) -> i32;
    fn bpf_get_dentry_xattr(
        dentry: *mut dentry,
        name: *const u8,
        value_p: *mut bpf_dynptr,
    ) -> i32;
    fn bpf_strncmp(s1: *const u8, s1_sz: u32, s2: *const u8) -> i32;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut monitored_pid: __u32 = 0;
#[no_mangle]
pub static mut found_xattr_from_file: __u32 = 0;
#[no_mangle]
pub static mut found_xattr_from_dentry: __u32 = 0;

static expected_value: [u8; 6] = *b"hello\0";
#[no_mangle]
pub static mut value1: [u8; 32] = [0; 32];
#[no_mangle]
pub static mut value2: [u8; 32] = [0; 32];

/* Matches caller of test_get_xattr() in prog_tests/fs_kfuncs.c */
static xattr_names: [[u8; 64]; 4] = [
    {
        /* The following work. */
        let mut bytes = [0u8; 64];
        let src = *b"user.kfuncs\0";
        let mut i = 0;
        while i < src.len() {
            bytes[i] = src[i];
            i += 1;
        }
        bytes
    },
    {
        let mut bytes = [0u8; 64];
        let src = *b"security.bpf.xxx\0";
        let mut i = 0;
        while i < src.len() {
            bytes[i] = src[i];
            i += 1;
        }
        bytes
    },
    {
        /* The following do not work. */
        let mut bytes = [0u8; 64];
        let src = *b"security.bpf\0";
        let mut i = 0;
        while i < src.len() {
            bytes[i] = src[i];
            i += 1;
        }
        bytes
    },
    {
        let mut bytes = [0u8; 64];
        let src = *b"security.selinux\0";
        let mut i = 0;
        while i < src.len() {
            bytes[i] = src[i];
            i += 1;
        }
        bytes
    },
];

#[no_mangle]
#[link_section = "lsm.s/file_open"]
pub unsafe extern "C" fn test_file_open(f: *mut file) -> i32 {
    let mut value_ptr: bpf_dynptr = core::mem::MaybeUninit::zeroed().assume_init();
    let pid: __u32;
    let mut ret: i32 = 0;
    let mut i: i32;

    pid = (bpf_get_current_pid_tgid() >> 32) as __u32;
    if pid != monitored_pid {
        return 0;
    }

    bpf_dynptr_from_mem(
        value1.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&value1) as u32,
        0,
        &mut value_ptr,
    );

    i = 0;
    while i < xattr_names.len() as i32 {
        ret = bpf_get_file_xattr(f, xattr_names[i as usize].as_ptr(), &mut value_ptr);
        if ret == core::mem::size_of_val(&expected_value) as i32 {
            break;
        }
        i += 1;
    }
    if ret != core::mem::size_of_val(&expected_value) as i32 {
        return 0;
    }
    if bpf_strncmp(value1.as_ptr(), ret as u32, expected_value.as_ptr()) != 0 {
        return 0;
    }
    found_xattr_from_file = 1;
    return 0;
}

#[no_mangle]
#[link_section = "lsm.s/inode_getxattr"]
pub unsafe extern "C" fn test_inode_getxattr(dentry: *mut dentry, name: *mut u8) -> i32 {
    let mut value_ptr: bpf_dynptr = core::mem::MaybeUninit::zeroed().assume_init();
    let pid: __u32;
    let mut ret: i32 = 0;
    let mut i: i32;

    let _ = name;

    pid = (bpf_get_current_pid_tgid() >> 32) as __u32;
    if pid != monitored_pid {
        return 0;
    }

    bpf_dynptr_from_mem(
        value2.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&value2) as u32,
        0,
        &mut value_ptr,
    );

    i = 0;
    while i < xattr_names.len() as i32 {
        ret = bpf_get_dentry_xattr(dentry, xattr_names[i as usize].as_ptr(), &mut value_ptr);
        if ret == core::mem::size_of_val(&expected_value) as i32 {
            break;
        }
        i += 1;
    }
    if ret != core::mem::size_of_val(&expected_value) as i32 {
        return 0;
    }
    if bpf_strncmp(value2.as_ptr(), ret as u32, expected_value.as_ptr()) != 0 {
        return 0;
    }
    found_xattr_from_dentry = 1;

    /* return non-zero to fail getxattr from user space */
    return -EINVAL;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
