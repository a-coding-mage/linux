// SPDX-License-Identifier: GPL-2.0
// Translated from C implementation using <byteswap.h>, "memswap.h", and <linux/types.h>.

use core::ffi::c_void;

pub type u32 = core::ffi::c_uint;
pub type u64 = core::ffi::c_ulonglong;

#[no_mangle]
pub unsafe extern "C" fn mem_bswap_32(src: *mut c_void, mut byte_size: core::ffi::c_int) {
    let mut m = src as *mut u32;

    while byte_size > 0 {
        *m = (*m).swap_bytes();
        byte_size -= core::mem::size_of::<u32>() as core::ffi::c_int;
        m = m.add(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn mem_bswap_64(src: *mut c_void, mut byte_size: core::ffi::c_int) {
    let mut m = src as *mut u64;

    while byte_size > 0 {
        *m = (*m).swap_bytes();
        byte_size -= core::mem::size_of::<u64>() as core::ffi::c_int;
        m = m.add(1);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
