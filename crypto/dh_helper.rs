// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2016, Intel Corporation
 * Authors: Salvatore Benedetto <salvatore.benedetto@intel.com>
 */

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

// Supplied by the crypto headers in the surrounding translation unit.
#[repr(C)]
pub struct kpp_secret {
    pub type_: u32,
    pub len: u32,
}

#[repr(C)]
pub struct dh {
    pub key: *mut c_void,
    pub key_size: u32,
    pub p: *mut c_void,
    pub p_size: u32,
    pub g: *mut c_void,
    pub g_size: u32,
}

const CRYPTO_KPP_SECRET_TYPE_DH: u32 = 1;
const DH_KPP_SECRET_MIN_SIZE: usize = size_of::<kpp_secret>() + 3 * size_of::<i32>();

unsafe fn dh_pack_data(
    dst: *mut u8,
    end: *mut u8,
    src: *const c_void,
    size: usize,
) -> *mut u8 {
    if dst.is_null() || size > end.offset_from(dst) as usize {
        return ptr::null_mut();
    }
    ptr::copy_nonoverlapping(src as *const u8, dst, size);
    dst.add(size)
}

unsafe fn dh_unpack_data(
    dst: *mut c_void,
    src: *const u8,
    size: usize,
) -> *const u8 {
    ptr::copy_nonoverlapping(src, dst as *mut u8, size);
    src.add(size)
}

unsafe fn dh_data_size(p: *const dh) -> usize {
    (*p).key_size as usize + (*p).p_size as usize + (*p).g_size as usize
}

pub unsafe fn crypto_dh_key_len(p: *const dh) -> u32 {
    (DH_KPP_SECRET_MIN_SIZE + dh_data_size(p)) as u32
}

pub unsafe fn crypto_dh_encode_key(buf: *mut i8, len: u32, params: *const dh) -> i32 {
    let mut ptr_ = buf as *mut u8;
    let end = ptr_.add(len as usize);
    let secret = kpp_secret {
        type_: CRYPTO_KPP_SECRET_TYPE_DH,
        len,
    };

    if len == 0 {
        return -22;
    }

    ptr_ = dh_pack_data(ptr_, end, &secret as *const _ as *const c_void, size_of::<kpp_secret>());
    ptr_ = dh_pack_data(ptr_, end, &(*params).key_size as *const _ as *const c_void, size_of::<u32>());
    ptr_ = dh_pack_data(ptr_, end, &(*params).p_size as *const _ as *const c_void, size_of::<u32>());
    ptr_ = dh_pack_data(ptr_, end, &(*params).g_size as *const _ as *const c_void, size_of::<u32>());
    ptr_ = dh_pack_data(ptr_, end, (*params).key, (*params).key_size as usize);
    ptr_ = dh_pack_data(ptr_, end, (*params).p, (*params).p_size as usize);
    ptr_ = dh_pack_data(ptr_, end, (*params).g, (*params).g_size as usize);
    if ptr_ != end {
        return -22;
    }
    0
}

pub unsafe fn __crypto_dh_decode_key(buf: *const i8, len: u32, params: *mut dh) -> i32 {
    let mut ptr_ = buf as *const u8;
    let mut secret: kpp_secret = core::mem::zeroed();

    if buf.is_null() || (len as usize) < DH_KPP_SECRET_MIN_SIZE {
        return -22;
    }

    ptr_ = dh_unpack_data(&mut secret as *mut _ as *mut c_void, ptr_, size_of::<kpp_secret>());
    if secret.type_ != CRYPTO_KPP_SECRET_TYPE_DH {
        return -22;
    }

    ptr_ = dh_unpack_data(&mut (*params).key_size as *mut _ as *mut c_void, ptr_, size_of::<u32>());
    ptr_ = dh_unpack_data(&mut (*params).p_size as *mut _ as *mut c_void, ptr_, size_of::<u32>());
    ptr_ = dh_unpack_data(&mut (*params).g_size as *mut _ as *mut c_void, ptr_, size_of::<u32>());
    if secret.len != crypto_dh_key_len(params) {
        return -22;
    }

    // Don't allocate memory. Set pointers to data within the given buffer.
    (*params).key = ptr_ as *mut c_void;
    (*params).p = ptr_.add((*params).key_size as usize) as *mut c_void;
    (*params).g = ptr_.add((*params).key_size as usize + (*params).p_size as usize) as *mut c_void;

    0
}

pub unsafe fn crypto_dh_decode_key(buf: *const i8, len: u32, params: *mut dh) -> i32 {
    let err = __crypto_dh_decode_key(buf, len, params);
    if err != 0 {
        return err;
    }

    // Don't permit the buffer for 'key' or 'g' to be larger than 'p'.
    if (*params).key_size > (*params).p_size || (*params).g_size > (*params).p_size {
        return -22;
    }

    // Don't permit 'p' to be 0.
    let p = (*params).p as *const u8;
    if (0..(*params).p_size as usize).all(|i| *p.add(i) == 0) {
        return -22;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
