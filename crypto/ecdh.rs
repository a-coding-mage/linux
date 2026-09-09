// SPDX-License-Identifier: GPL-2.0-or-later
/* ECDH key-agreement protocol
 *
 * Copyright (c) 2016, Intel Corporation
 * Authors: Salvator Benedetto <salvatore.benedetto@intel.com>
 */

// Kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct ecdh_ctx {
    pub curve_id: ::core::ffi::c_uint,
    pub ndigits: ::core::ffi::c_uint,
    pub private_key: [u64; ECC_MAX_DIGITS],
}

#[inline]
unsafe fn ecdh_get_ctx(tfm: *mut crypto_kpp) -> *mut ecdh_ctx {
    kpp_tfm_ctx(tfm) as *mut ecdh_ctx
}

unsafe fn ecdh_set_secret(tfm: *mut crypto_kpp, buf: *const ::core::ffi::c_void, len: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let ctx = ecdh_get_ctx(tfm);
    let mut params = ecdh { key: core::ptr::null(), key_size: 0 };
    let mut ret: ::core::ffi::c_int = 0;

    if crypto_ecdh_decode_key(buf, len, &mut params) < 0 || params.key_size > (core::mem::size_of::<u64>() * (*ctx).ndigits as usize) as _ {
        return -EINVAL;
    }

    core::ptr::write_bytes((*ctx).private_key.as_mut_ptr(), 0, (*ctx).private_key.len());

    if params.key.is_null() || params.key_size == 0 {
        return ecc_gen_privkey((*ctx).curve_id, (*ctx).ndigits, (*ctx).private_key.as_mut_ptr());
    }

    ecc_digits_from_bytes(params.key, params.key_size, (*ctx).private_key.as_mut_ptr(), (*ctx).ndigits);

    if ecc_is_key_valid((*ctx).curve_id, (*ctx).ndigits, (*ctx).private_key.as_ptr(), params.key_size) < 0 {
        memzero_explicit((*ctx).private_key.as_mut_ptr() as *mut _, params.key_size as usize);
        ret = -EINVAL;
    }
    ret
}

unsafe fn ecdh_compute_value(req: *mut kpp_request) -> ::core::ffi::c_int {
    let tfm = crypto_kpp_reqtfm(req);
    let ctx = ecdh_get_ctx(tfm);
    let mut public_key: *mut u64;
    let mut shared_secret: *mut u64 = core::ptr::null_mut();
    let mut buf: *mut ::core::ffi::c_void;
    let mut copied: usize;
    let mut nbytes = ((*ctx).ndigits << ECC_DIGITS_TO_BYTES_SHIFT) as usize;
    let public_key_sz = 2 * nbytes;
    let mut ret: ::core::ffi::c_int = -ENOMEM;

    public_key = kmalloc(public_key_sz, GFP_KERNEL) as *mut u64;
    if public_key.is_null() { return -ENOMEM; }

    if !(*req).src.is_null() {
        shared_secret = kmalloc(nbytes, GFP_KERNEL) as *mut u64;
        if shared_secret.is_null() { kfree(public_key as *mut _); return ret; }
        ret = -EINVAL;
        if public_key_sz != (*req).src_len as usize { kfree_sensitive(shared_secret as *mut _); kfree(public_key as *mut _); return ret; }
        copied = sg_copy_to_buffer((*req).src, sg_nents_for_len((*req).src, public_key_sz), public_key as *mut _, public_key_sz);
        if copied != public_key_sz { kfree_sensitive(shared_secret as *mut _); kfree(public_key as *mut _); return ret; }
        ret = crypto_ecdh_shared_secret((*ctx).curve_id, (*ctx).ndigits, (*ctx).private_key.as_ptr(), public_key, shared_secret);
        buf = shared_secret as *mut _;
    } else {
        ret = ecc_make_pub_key((*ctx).curve_id, (*ctx).ndigits, (*ctx).private_key.as_ptr(), public_key);
        buf = public_key as *mut _;
        nbytes = public_key_sz;
    }
    if ret < 0 { kfree_sensitive(shared_secret as *mut _); kfree(public_key as *mut _); return ret; }
    nbytes = core::cmp::min(nbytes, (*req).dst_len as usize);
    copied = sg_copy_from_buffer((*req).dst, sg_nents_for_len((*req).dst, nbytes), buf, nbytes);
    if copied != nbytes { ret = -EINVAL; }
    kfree_sensitive(shared_secret as *mut _);
    kfree(public_key as *mut _);
    ret
}

unsafe fn ecdh_max_size(tfm: *mut crypto_kpp) -> ::core::ffi::c_uint {
    let ctx = ecdh_get_ctx(tfm);
    (*ctx).ndigits << (ECC_DIGITS_TO_BYTES_SHIFT + 1)
}

macro_rules! ecdh_init_tfm {
    ($name:ident, $curve:ident, $digits:ident) => {
        unsafe fn $name(tfm: *mut crypto_kpp) -> ::core::ffi::c_int {
            let ctx = ecdh_get_ctx(tfm);
            (*ctx).curve_id = $curve;
            (*ctx).ndigits = $digits;
            0
        }
    };
}

ecdh_init_tfm!(ecdh_nist_p192_init_tfm, ECC_CURVE_NIST_P192, ECC_CURVE_NIST_P192_DIGITS);
ecdh_init_tfm!(ecdh_nist_p256_init_tfm, ECC_CURVE_NIST_P256, ECC_CURVE_NIST_P256_DIGITS);
ecdh_init_tfm!(ecdh_nist_p384_init_tfm, ECC_CURVE_NIST_P384, ECC_CURVE_NIST_P384_DIGITS);

static mut ecdh_nist_p192_registered: bool = false;

unsafe fn ecdh_init() -> ::core::ffi::c_int {
    let mut ret = crypto_register_kpp(&mut ecdh_nist_p192);
    ecdh_nist_p192_registered = ret == 0;
    ret = crypto_register_kpp(&mut ecdh_nist_p256);
    if ret != 0 { if ecdh_nist_p192_registered { crypto_unregister_kpp(&mut ecdh_nist_p192); } return ret; }
    ret = crypto_register_kpp(&mut ecdh_nist_p384);
    if ret != 0 { crypto_unregister_kpp(&mut ecdh_nist_p256); if ecdh_nist_p192_registered { crypto_unregister_kpp(&mut ecdh_nist_p192); } }
    ret
}

unsafe fn ecdh_exit() {
    if ecdh_nist_p192_registered { crypto_unregister_kpp(&mut ecdh_nist_p192); }
    crypto_unregister_kpp(&mut ecdh_nist_p256);
    crypto_unregister_kpp(&mut ecdh_nist_p384);
}

// module_init(ecdh_init); module_exit(ecdh_exit);
// MODULE_ALIAS_CRYPTO("ecdh"); MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("ECDH generic algorithm");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
