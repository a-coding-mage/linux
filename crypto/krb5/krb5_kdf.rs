// SPDX-License-Identifier: GPL-2.0-or-later
/* Kerberos key derivation.
 *
 * Copyright (C) 2025 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// C kernel includes and build-time symbols are supplied by the surrounding
// kernel translation unit.

use core::ffi::c_void;

extern "C" {
    fn kzalloc(size: usize, gfp: gfp_t) -> *mut c_void;
    fn kfree_sensitive(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

pub type gfp_t = usize;

#[repr(C)]
pub struct krb5_buffer {
    pub len: usize,
    pub data: *mut u8,
}

#[repr(C)]
pub struct krb5_profile {
    pub calc_PRF: unsafe extern "C" fn(
        krb5: *const krb5_enctype,
        k: *const krb5_buffer,
        s: *const krb5_buffer,
        result: *mut krb5_buffer,
        gfp: gfp_t,
    ) -> i32,
    pub calc_Kc: unsafe extern "C" fn(
        krb5: *const krb5_enctype,
        tk: *const krb5_buffer,
        usage_constant: *const krb5_buffer,
        key: *mut krb5_buffer,
        gfp: gfp_t,
    ) -> i32,
    pub calc_Ke: unsafe extern "C" fn(
        krb5: *const krb5_enctype,
        tk: *const krb5_buffer,
        usage_constant: *const krb5_buffer,
        key: *mut krb5_buffer,
        gfp: gfp_t,
    ) -> i32,
    pub calc_Ki: unsafe extern "C" fn(
        krb5: *const krb5_enctype,
        tk: *const krb5_buffer,
        usage_constant: *const krb5_buffer,
        key: *mut krb5_buffer,
        gfp: gfp_t,
    ) -> i32,
}

#[repr(C)]
pub struct krb5_enctype {
    pub prf_len: usize,
    pub Kc_len: usize,
    pub Ke_len: usize,
    pub Ki_len: usize,
    pub profile: *const krb5_profile,
}

extern "C" {
    static KEY_USAGE_SEED_CHECKSUM: u8;
    static KEY_USAGE_SEED_ENCRYPTION: u8;
    static KEY_USAGE_SEED_INTEGRITY: u8;
}

#[inline]
unsafe fn round16(x: usize) -> usize {
    (x + 15) & !15
}

/// crypto_krb5_calc_PRFplus - Calculate PRF+ [RFC4402]
pub unsafe extern "C" fn crypto_krb5_calc_PRFplus(
    krb5: *const krb5_enctype,
    k: *const krb5_buffer,
    l: u32,
    s: *const krb5_buffer,
    result: *mut krb5_buffer,
    gfp: gfp_t,
) -> i32 {
    let mut t_series = krb5_buffer { len: 0, data: core::ptr::null_mut() };
    let mut tn = krb5_buffer { len: 0, data: core::ptr::null_mut() };
    let mut n_s = krb5_buffer { len: 0, data: core::ptr::null_mut() };
    let mut n: i32 = 1;

    tn.len = (*krb5).prf_len;
    n_s.len = 4 + (*s).len;
    let buffer = kzalloc(round16(l as usize + tn.len) + round16(n_s.len), gfp) as *mut u8;
    if buffer.is_null() {
        return -12; // -ENOMEM
    }

    t_series.data = buffer;
    n_s.data = buffer.add(round16(l as usize + tn.len));
    memcpy(n_s.data.add(4) as *mut c_void, (*s).data as *const c_void, (*s).len);

    let mut ret;
    while t_series.len < l as usize {
        let be_n = (n as u32).to_be();
        memcpy(n_s.data as *mut c_void, &be_n as *const u32 as *const c_void, 4);
        tn.data = t_series.data.add(tn.len * (n as usize - 1));
        ret = ((*(*krb5).profile).calc_PRF)(krb5, k, &n_s, &mut tn, gfp);
        if ret < 0 {
            kfree_sensitive(buffer as *mut c_void);
            return ret;
        }
        t_series.len += tn.len;
        n += 1;
    }

    memcpy((*result).data as *mut c_void, t_series.data as *const c_void, l as usize);
    kfree_sensitive(buffer as *mut c_void);
    0
}

pub unsafe extern "C" fn krb5_derive_Kc(
    krb5: *const krb5_enctype, tk: *const krb5_buffer, usage: u32,
    key: *mut krb5_buffer, gfp: gfp_t,
) -> i32 {
    let mut buf = [0u8; 5];
    let usage_be = usage.to_be();
    core::ptr::copy_nonoverlapping(&usage_be as *const u32 as *const u8, buf.as_mut_ptr(), 4);
    buf[4] = KEY_USAGE_SEED_CHECKSUM;
    (*key).len = (*krb5).Kc_len;
    let constant = krb5_buffer { len: 5, data: buf.as_mut_ptr() };
    ((*(*krb5).profile).calc_Kc)(krb5, tk, &constant, key, gfp)
}

pub unsafe extern "C" fn krb5_derive_Ke(
    krb5: *const krb5_enctype, tk: *const krb5_buffer, usage: u32,
    key: *mut krb5_buffer, gfp: gfp_t,
) -> i32 {
    let mut buf = [0u8; 5];
    let usage_be = usage.to_be();
    core::ptr::copy_nonoverlapping(&usage_be as *const u32 as *const u8, buf.as_mut_ptr(), 4);
    buf[4] = KEY_USAGE_SEED_ENCRYPTION;
    (*key).len = (*krb5).Ke_len;
    let constant = krb5_buffer { len: 5, data: buf.as_mut_ptr() };
    ((*(*krb5).profile).calc_Ke)(krb5, tk, &constant, key, gfp)
}

pub unsafe extern "C" fn krb5_derive_Ki(
    krb5: *const krb5_enctype, tk: *const krb5_buffer, usage: u32,
    key: *mut krb5_buffer, gfp: gfp_t,
) -> i32 {
    let mut buf = [0u8; 5];
    let usage_be = usage.to_be();
    core::ptr::copy_nonoverlapping(&usage_be as *const u32 as *const u8, buf.as_mut_ptr(), 4);
    buf[4] = KEY_USAGE_SEED_INTEGRITY;
    (*key).len = (*krb5).Ki_len;
    let constant = krb5_buffer { len: 5, data: buf.as_mut_ptr() };
    ((*(*krb5).profile).calc_Ki)(krb5, tk, &constant, key, gfp)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
