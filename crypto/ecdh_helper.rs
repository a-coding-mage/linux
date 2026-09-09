// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2016, Intel Corporation
 * Authors: Salvatore Benedetto <salvatore.benedetto@intel.com>
 */

// The Linux kernel headers supplying these types and constants are external
// dependencies of this translation.

const ECDH_KPP_SECRET_MIN_SIZE: usize =
    core::mem::size_of::<kpp_secret>() + core::mem::size_of::<i16>();

#[inline]
unsafe fn ecdh_pack_data(
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    sz: usize,
) -> *mut u8 {
    core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, sz);
    (dst as *mut u8).add(sz)
}

#[inline]
unsafe fn ecdh_unpack_data(
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    sz: usize,
) -> *const u8 {
    core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, sz);
    (src as *const u8).add(sz)
}

pub unsafe fn crypto_ecdh_key_len(params: *const ecdh) -> u32 {
    (ECDH_KPP_SECRET_MIN_SIZE as u32) + (*params).key_size
}

pub unsafe fn crypto_ecdh_encode_key(
    buf: *mut i8,
    len: u32,
    params: *const ecdh,
) -> i32 {
    let mut ptr = buf as *mut u8;
    let secret = kpp_secret {
        r#type: CRYPTO_KPP_SECRET_TYPE_ECDH,
        len,
    };

    if buf.is_null() {
        return -EINVAL;
    }

    if len != crypto_ecdh_key_len(params) {
        return -EINVAL;
    }

    ptr = ecdh_pack_data(
        ptr as *mut core::ffi::c_void,
        &secret as *const kpp_secret as *const core::ffi::c_void,
        core::mem::size_of::<kpp_secret>(),
    );
    ptr = ecdh_pack_data(
        ptr as *mut core::ffi::c_void,
        &(*params).key_size as *const u32 as *const core::ffi::c_void,
        core::mem::size_of::<u32>(),
    );
    ecdh_pack_data(
        ptr as *mut core::ffi::c_void,
        (*params).key as *const core::ffi::c_void,
        (*params).key_size as usize,
    );

    0
}

pub unsafe fn crypto_ecdh_decode_key(
    buf: *const i8,
    len: u32,
    params: *mut ecdh,
) -> i32 {
    let mut ptr = buf as *const u8;
    let mut secret: kpp_secret = core::mem::zeroed();

    if buf.is_null() || len < ECDH_KPP_SECRET_MIN_SIZE as u32 {
        return -EINVAL;
    }

    ptr = ecdh_unpack_data(
        &mut secret as *mut kpp_secret as *mut core::ffi::c_void,
        ptr as *const core::ffi::c_void,
        core::mem::size_of::<kpp_secret>(),
    );
    if secret.r#type != CRYPTO_KPP_SECRET_TYPE_ECDH {
        return -EINVAL;
    }

    if len < secret.len {
        return -EINVAL;
    }

    ptr = ecdh_unpack_data(
        &mut (*params).key_size as *mut u32 as *mut core::ffi::c_void,
        ptr as *const core::ffi::c_void,
        core::mem::size_of::<u32>(),
    );
    if secret.len != crypto_ecdh_key_len(params) {
        return -EINVAL;
    }

    /* Don't allocate memory. Set pointer to data
     * within the given buffer
     */
    (*params).key = ptr as *mut core::ffi::c_void;

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
