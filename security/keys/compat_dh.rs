// SPDX-License-Identifier: GPL-2.0-or-later
/* 32-bit compatibility syscall for 64-bit systems for DH operations
 *
 * Copyright (C) 2016 Stephan Mueller <smueller@chronox.de>
 */

use core::ffi::{c_char, c_long, c_ulong, c_void};
use core::mem::{size_of, MaybeUninit};
use core::ptr;

const EFAULT: c_long = 14;

#[repr(C)]
pub struct keyctl_dh_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct keyctl_kdf_params {
    pub hashname: *mut c_char,
    pub otherinfo: *mut c_char,
    pub otherinfolen: u32,
    pub __spare: [u32; 8],
}

#[repr(C)]
pub struct compat_keyctl_kdf_params {
    pub hashname: u32,
    pub otherinfo: u32,
    pub otherinfolen: u32,
    pub __spare: [u32; 8],
}

unsafe extern "C" {
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: c_ulong) -> c_ulong;
    fn memcpy(to: *mut c_void, from: *const c_void, n: c_ulong) -> *mut c_void;
    fn __keyctl_dh_compute(
        params: *mut keyctl_dh_params,
        buffer: *mut c_char,
        buflen: c_ulong,
        kdf: *mut keyctl_kdf_params,
    ) -> c_long;
}

#[inline]
unsafe fn compat_ptr(ptr32: u32) -> *mut c_void {
    ptr32 as usize as *mut c_void
}

/*
 * Perform the DH computation or DH based key derivation.
 *
 * If successful, 0 will be returned.
 */
pub unsafe extern "C" fn compat_keyctl_dh_compute(
    params: *mut keyctl_dh_params,
    buffer: *mut c_char,
    buflen: c_ulong,
    kdf: *mut compat_keyctl_kdf_params,
) -> c_long {
    let mut kdfcopy = MaybeUninit::<keyctl_kdf_params>::uninit();
    let mut compat_kdfcopy = MaybeUninit::<compat_keyctl_kdf_params>::uninit();

    if kdf.is_null() {
        return unsafe { __keyctl_dh_compute(params, buffer, buflen, ptr::null_mut()) };
    }

    if unsafe {
        copy_from_user(
            compat_kdfcopy.as_mut_ptr() as *mut c_void,
            kdf as *const c_void,
            size_of::<compat_keyctl_kdf_params>() as c_ulong,
        )
    } != 0
    {
        return -EFAULT;
    }

    let compat_kdfcopy = unsafe { compat_kdfcopy.assume_init() };
    unsafe {
        let kdfcopy_ptr = kdfcopy.as_mut_ptr();
        (*kdfcopy_ptr).hashname = compat_ptr(compat_kdfcopy.hashname) as *mut c_char;
        (*kdfcopy_ptr).otherinfo = compat_ptr(compat_kdfcopy.otherinfo) as *mut c_char;
        (*kdfcopy_ptr).otherinfolen = compat_kdfcopy.otherinfolen;
        memcpy(
            (*kdfcopy_ptr).__spare.as_mut_ptr() as *mut c_void,
            compat_kdfcopy.__spare.as_ptr() as *const c_void,
            size_of::<[u32; 8]>() as c_ulong,
        );

        __keyctl_dh_compute(params, buffer, buflen, kdfcopy.as_mut_ptr())
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
