// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Secure Processor Dynamic Boost Control sample library
 *
 * Copyright (C) 2023 Advanced Micro Devices, Inc.
 *
 * Author: Mario Limonciello <mario.limonciello@amd.com>
 */

use core::ffi::{c_int, c_ulong, c_void};

// Dependencies from <linux/psp-dbc.h>.
type __u8 = u8;

unsafe extern "C" {
    static DBCIOCNONCE: c_ulong;
    static DBCIOCUID: c_ulong;
    static DBCIOCPARAM: c_ulong;
}

#[repr(C, packed)]
struct dbc_user_nonce {
    auth_needed: c_int,
    signature: [__u8; 32],
    nonce: [__u8; 32],
}

#[repr(C, packed)]
struct dbc_user_setuid {
    uid: [__u8; 16],
    signature: [__u8; 32],
}

#[repr(C, packed)]
struct dbc_user_param {
    msg_index: c_int,
    param: c_int,
    signature: [__u8; 32],
}

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_nonce(
    fd: c_int,
    nonce_out: *mut c_void,
    signature: *mut c_void,
) -> c_int {
    let mut tmp = dbc_user_nonce {
        auth_needed: (signature != core::ptr::null_mut()) as c_int,
        signature: [0; 32],
        nonce: [0; 32],
    };

    assert!(!nonce_out.is_null());

    if !signature.is_null() {
        unsafe {
            memcpy(
                tmp.signature.as_mut_ptr() as *mut c_void,
                signature as *const c_void,
                core::mem::size_of_val(&tmp.signature),
            );
        }
    }

    if unsafe { ioctl(fd, DBCIOCNONCE, &mut tmp as *mut dbc_user_nonce) } != 0 {
        return unsafe { errno() };
    }
    unsafe {
        memcpy(
            nonce_out,
            tmp.nonce.as_ptr() as *const c_void,
            core::mem::size_of_val(&tmp.nonce),
        );
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_uid(fd: c_int, uid: *mut __u8, signature: *mut __u8) -> c_int {
    let mut tmp = core::mem::MaybeUninit::<dbc_user_setuid>::uninit();

    assert!(!uid.is_null());
    assert!(!signature.is_null());

    unsafe {
        let tmp_ptr = tmp.as_mut_ptr();
        memcpy(
            (*tmp_ptr).uid.as_mut_ptr() as *mut c_void,
            uid as *const c_void,
            core::mem::size_of::<[__u8; 16]>(),
        );
        memcpy(
            (*tmp_ptr).signature.as_mut_ptr() as *mut c_void,
            signature as *const c_void,
            core::mem::size_of::<[__u8; 32]>(),
        );

        if ioctl(fd, DBCIOCUID, tmp_ptr) != 0 {
            return errno();
        }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_param(
    fd: c_int,
    msg_index: c_int,
    signature: *mut __u8,
    data: *mut c_int,
) -> c_int {
    let mut tmp = dbc_user_param {
        msg_index,
        param: unsafe { *data },
        signature: [0; 32],
    };

    assert!(!signature.is_null());
    assert!(!data.is_null());

    unsafe {
        memcpy(
            tmp.signature.as_mut_ptr() as *mut c_void,
            signature as *const c_void,
            core::mem::size_of_val(&tmp.signature),
        );
    }

    if unsafe { ioctl(fd, DBCIOCPARAM, &mut tmp as *mut dbc_user_param) } != 0 {
        return unsafe { errno() };
    }

    unsafe {
        *data = tmp.param;
        memcpy(
            signature as *mut c_void,
            tmp.signature.as_ptr() as *const c_void,
            core::mem::size_of_val(&tmp.signature),
        );
    }
    0
}
