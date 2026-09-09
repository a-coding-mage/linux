// SPDX-License-Identifier: GPL-2.0-or-later
/* Kerberos library self-testing
 *
 * Copyright (C) 2025 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Kernel dependencies and declarations are supplied by the surrounding crate.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum WhichKey { TestKc, TestKe, TestKi }

unsafe fn prep_buf(buf: *mut krb5_buffer) -> i32 {
    (*buf).data = kmalloc((*buf).len, GFP_KERNEL);
    if (*buf).data.is_null() { return -ENOMEM; }
    0
}

unsafe fn load_buf(buf: *mut krb5_buffer, from: *const i8) -> i32 {
    let len = strlen(from);
    let mut ret: i32;
    if len > 1 && *from == b'\'' as i8 {
        (*buf).len = len - 1;
        ret = prep_buf(buf);
        if ret < 0 { return ret; }
        memcpy((*buf).data, from.add(1) as *const _, len - 1);
        return 0;
    }
    if (len & 1) != 0 { return -EINVAL; }
    (*buf).len = len / 2;
    ret = prep_buf(buf);
    if ret < 0 { return ret; }
    ret = hex2bin((*buf).data, from, (*buf).len);
    if ret < 0 { return -EBADMSG; }
    ret
}

unsafe fn clear_buf(buf: *mut krb5_buffer) {
    kfree((*buf).data);
    (*buf).len = 0;
    (*buf).data = core::ptr::null_mut();
}

unsafe fn test_one_prf(test: *const krb5_prf_test) -> i32 {
    let krb5 = crypto_krb5_find_enctype((*test).etype);
    let mut key = krb5_buffer::default(); let mut octet = krb5_buffer::default();
    let mut result = krb5_buffer::default(); let mut prf = krb5_buffer::default();
    if krb5.is_null() { return -EOPNOTSUPP; }
    pr_notice!("Running %s %s\\n", (*krb5).name, (*test).name);
    let mut ret = load_buf(&mut key, (*test).key); if ret < 0 { return ret; }
    ret = load_buf(&mut octet, (*test).octet); if ret < 0 { clear_buf(&mut key); return ret; }
    ret = load_buf(&mut prf, (*test).prf); if ret < 0 { clear_buf(&mut octet); clear_buf(&mut key); return ret; }
    result.len = (*krb5).prf_len; ret = prep_buf(&mut result);
    if ret >= 0 && result.len != prf.len { ret = -EINVAL; }
    if ret >= 0 { ret = ((*(*krb5).profile).calc_PRF)(krb5, &key, &octet, &mut result, GFP_KERNEL); }
    if ret >= 0 && memcmp(result.data, prf.data, result.len) != 0 { ret = -EKEYREJECTED; }
    clear_buf(&mut result); clear_buf(&mut prf); clear_buf(&mut octet); clear_buf(&mut key); ret
}

unsafe fn test_key(krb5: *const krb5_enctype, base: *const krb5_buffer,
                   test: *const krb5_key_test_one, which: WhichKey) -> i32 {
    let mut key = krb5_buffer::default(); let mut result = krb5_buffer::default();
    let mut ret = load_buf(&mut key, (*test).key); if ret < 0 { return ret; }
    result.len = key.len; ret = prep_buf(&mut result);
    if ret >= 0 { ret = match which {
        WhichKey::TestKc => krb5_derive_Kc(krb5, base, (*test).use_, &mut result, GFP_KERNEL),
        WhichKey::TestKe => krb5_derive_Ke(krb5, base, (*test).use_, &mut result, GFP_KERNEL),
        WhichKey::TestKi => krb5_derive_Ki(krb5, base, (*test).use_, &mut result, GFP_KERNEL),
    }; }
    if ret >= 0 && memcmp(result.data, key.data, result.len) != 0 { ret = -EKEYREJECTED; }
    clear_buf(&mut key); clear_buf(&mut result); ret
}

unsafe fn test_one_key(test: *const krb5_key_test) -> i32 {
    let krb5 = crypto_krb5_find_enctype((*test).etype); if krb5.is_null() { return -EOPNOTSUPP; }
    let mut base = krb5_buffer::default(); let mut ret = load_buf(&mut base, (*test).key);
    if ret >= 0 { ret = test_key(krb5, &base, &(*test).Kc, WhichKey::TestKc); }
    if ret >= 0 { ret = test_key(krb5, &base, &(*test).Ke, WhichKey::TestKe); }
    if ret >= 0 { ret = test_key(krb5, &base, &(*test).Ki, WhichKey::TestKi); }
    clear_buf(&mut base); ret
}

// The encryption and checksum test bodies retain the C ABI operations and cleanup ordering.
// Their crypto helpers and test structures are external declarations supplied by the kernel port.
extern "C" {
    fn krb5_test_one_enc(test: *const krb5_enc_test, buf: *mut core::ffi::c_void) -> i32;
    fn krb5_test_one_mic(test: *const krb5_mic_test, buf: *mut core::ffi::c_void) -> i32;
}

pub unsafe fn krb5_selftest() -> i32 {
    let buf = kmalloc(4096, GFP_KERNEL); if buf.is_null() { return -ENOMEM; }
    let mut ret = 0;
    let mut i = 0;
    while !(*krb5_prf_tests.add(i)).name.is_null() { ret = test_one_prf(krb5_prf_tests.add(i)); if ret < 0 && ret != -EOPNOTSUPP { break; } i += 1; }
    if ret >= 0 { i = 0; while !(*krb5_key_tests.add(i)).name.is_null() { ret = test_one_key(krb5_key_tests.add(i)); if ret < 0 && ret != -EOPNOTSUPP { break; } i += 1; } }
    if ret >= 0 { i = 0; while !(*krb5_enc_tests.add(i)).name.is_null() { memset(buf, 0x5a, 4096); ret = krb5_test_one_enc(krb5_enc_tests.add(i), buf); if ret < 0 && ret != -EOPNOTSUPP { break; } i += 1; } }
    if ret >= 0 { i = 0; while !(*krb5_mic_tests.add(i)).name.is_null() { memset(buf, 0x5a, 4096); ret = krb5_test_one_mic(krb5_mic_tests.add(i), buf); if ret < 0 && ret != -EOPNOTSUPP { break; } i += 1; } }
    kfree(buf); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
