// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

/* Dependencies from the original C source:
 * vmlinux.h, errno.h, bpf/bpf_tracing.h, bpf_kfuncs.h, bpf_misc.h.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __u32 = u32;
type size_t = usize;

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mnt_idmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_dynptr {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_probe_read_kernel(dst: *mut core::ffi::c_void, size: size_t, unsafe_ptr: *const core::ffi::c_void) -> i64;
    fn bpf_strncmp(s1: *const u8, s1_sz: u32, s2: *const u8) -> i64;
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_dynptr_from_mem(
        data: *mut core::ffi::c_void,
        size: u32,
        flags: u64,
        ptr: *mut bpf_dynptr,
    ) -> i64;
    fn bpf_get_dentry_xattr(dentry: *mut dentry, name: *const u8, value_ptr: *mut bpf_dynptr) -> i32;
    fn bpf_set_dentry_xattr(
        dentry: *mut dentry,
        name: *const u8,
        value_ptr: *mut bpf_dynptr,
        flags: i32,
    ) -> i32;
    fn bpf_remove_dentry_xattr(dentry: *mut dentry, name: *const u8) -> i32;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut monitored_pid: __u32 = 0;

#[unsafe(no_mangle)]
pub static xattr_foo: [u8; 17] = *b"security.bpf.foo\0";
#[unsafe(no_mangle)]
pub static xattr_bar: [u8; 17] = *b"security.bpf.bar\0";
static xattr_selinux: [u8; 17] = *b"security.selinux\0";
#[unsafe(no_mangle)]
pub static mut value_bar: [u8; 6] = *b"world\0";
#[unsafe(no_mangle)]
pub static mut read_value: [u8; 32] = [0; 32];

#[unsafe(no_mangle)]
pub static mut set_security_bpf_bar_success: bool = false;
#[unsafe(no_mangle)]
pub static mut remove_security_bpf_bar_success: bool = false;
#[unsafe(no_mangle)]
pub static mut set_security_selinux_fail: bool = false;
#[unsafe(no_mangle)]
pub static mut remove_security_selinux_fail: bool = false;

#[unsafe(no_mangle)]
pub static mut name_buf: [u8; 32] = [0; 32];

#[inline]
unsafe fn name_match_foo(name: *const u8) -> bool {
    unsafe {
        bpf_probe_read_kernel(
            core::ptr::addr_of_mut!(name_buf).cast::<core::ffi::c_void>(),
            core::mem::size_of::<[u8; 32]>(),
            name.cast::<core::ffi::c_void>(),
        );

        bpf_strncmp(
            core::ptr::addr_of!(name_buf).cast::<u8>(),
            core::mem::size_of_val(&xattr_foo) as u32,
            xattr_foo.as_ptr(),
        ) == 0
    }
}

/* Test bpf_set_dentry_xattr and bpf_remove_dentry_xattr */
#[unsafe(link_section = "lsm.s/inode_getxattr")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_inode_getxattr(dentry: *mut dentry, name: *mut u8) -> i32 {
    let mut value_ptr: bpf_dynptr = unsafe { core::mem::zeroed() };
    let pid: __u32;
    let mut ret: i32;

    unsafe {
        pid = (bpf_get_current_pid_tgid() >> 32) as __u32;
        if pid != monitored_pid {
            return 0;
        }

        /* Only do the following for security.bpf.foo */
        if !name_match_foo(name.cast::<u8>()) {
            return 0;
        }

        bpf_dynptr_from_mem(
            core::ptr::addr_of_mut!(read_value).cast::<core::ffi::c_void>(),
            core::mem::size_of::<[u8; 32]>() as u32,
            0,
            &mut value_ptr,
        );

        /* read security.bpf.bar */
        ret = bpf_get_dentry_xattr(dentry, xattr_bar.as_ptr(), &mut value_ptr);

        if ret < 0 {
            /* If security.bpf.bar doesn't exist, set it */
            bpf_dynptr_from_mem(
                core::ptr::addr_of_mut!(value_bar).cast::<core::ffi::c_void>(),
                core::mem::size_of::<[u8; 6]>() as u32,
                0,
                &mut value_ptr,
            );

            ret = bpf_set_dentry_xattr(dentry, xattr_bar.as_ptr(), &mut value_ptr, 0);
            if ret == 0 {
                set_security_bpf_bar_success = true;
            }
            ret = bpf_set_dentry_xattr(dentry, xattr_selinux.as_ptr(), &mut value_ptr, 0);
            if ret != 0 {
                set_security_selinux_fail = true;
            }
        } else {
            /* If security.bpf.bar exists, remove it */
            ret = bpf_remove_dentry_xattr(dentry, xattr_bar.as_ptr());
            if ret == 0 {
                remove_security_bpf_bar_success = true;
            }

            ret = bpf_remove_dentry_xattr(dentry, xattr_selinux.as_ptr());
            if ret != 0 {
                remove_security_selinux_fail = true;
            }
        }
    }

    0
}

#[unsafe(no_mangle)]
pub static mut locked_set_security_bpf_bar_success: bool = false;
#[unsafe(no_mangle)]
pub static mut locked_remove_security_bpf_bar_success: bool = false;
#[unsafe(no_mangle)]
pub static mut locked_set_security_selinux_fail: bool = false;
#[unsafe(no_mangle)]
pub static mut locked_remove_security_selinux_fail: bool = false;

/* Test bpf_set_dentry_xattr_locked and bpf_remove_dentry_xattr_locked.
 * It not necessary to differentiate the _locked version and the
 * not-_locked version in the BPF program. The verifier will fix them up
 * properly.
 */
#[unsafe(link_section = "lsm.s/inode_setxattr")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_inode_setxattr(
    idmap: *mut mnt_idmap,
    dentry: *mut dentry,
    name: *const u8,
    value: *const core::ffi::c_void,
    size: size_t,
    flags: i32,
) -> i32 {
    let mut value_ptr: bpf_dynptr = unsafe { core::mem::zeroed() };
    let pid: __u32;
    let mut ret: i32;

    let _ = idmap;
    let _ = value;
    let _ = size;
    let _ = flags;

    unsafe {
        pid = (bpf_get_current_pid_tgid() >> 32) as __u32;
        if pid != monitored_pid {
            return 0;
        }

        /* Only do the following for security.bpf.foo */
        if !name_match_foo(name) {
            return 0;
        }

        bpf_dynptr_from_mem(
            core::ptr::addr_of_mut!(read_value).cast::<core::ffi::c_void>(),
            core::mem::size_of::<[u8; 32]>() as u32,
            0,
            &mut value_ptr,
        );

        /* read security.bpf.bar */
        ret = bpf_get_dentry_xattr(dentry, xattr_bar.as_ptr(), &mut value_ptr);

        if ret < 0 {
            /* If security.bpf.bar doesn't exist, set it */
            bpf_dynptr_from_mem(
                core::ptr::addr_of_mut!(value_bar).cast::<core::ffi::c_void>(),
                core::mem::size_of::<[u8; 6]>() as u32,
                0,
                &mut value_ptr,
            );

            ret = bpf_set_dentry_xattr(dentry, xattr_bar.as_ptr(), &mut value_ptr, 0);
            if ret == 0 {
                locked_set_security_bpf_bar_success = true;
            }
            ret = bpf_set_dentry_xattr(dentry, xattr_selinux.as_ptr(), &mut value_ptr, 0);
            if ret != 0 {
                locked_set_security_selinux_fail = true;
            }
        } else {
            /* If security.bpf.bar exists, remove it */
            ret = bpf_remove_dentry_xattr(dentry, xattr_bar.as_ptr());
            if ret == 0 {
                locked_remove_security_bpf_bar_success = true;
            }

            ret = bpf_remove_dentry_xattr(dentry, xattr_selinux.as_ptr());
            if ret != 0 {
                locked_remove_security_selinux_fail = true;
            }
        }
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
