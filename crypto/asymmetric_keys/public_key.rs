// SPDX-License-Identifier: GPL-2.0-or-later
/* In-software asymmetric public-key crypto subtype
 *
 * See Documentation/crypto/asymmetric-keys.rst
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the surrounding kernel translation.

#[allow(non_snake_case, non_camel_case_types, dead_code)]
unsafe fn public_key_describe(asymmetric_key: *const key, m: *mut seq_file) {
    let key = (*asymmetric_key).payload.data[asym_crypto as usize] as *mut public_key;
    if !key.is_null() {
        seq_printf(m, "%s.%s", (*key).id_type, (*key).pkey_algo);
    }
}

pub unsafe fn public_key_free(key: *mut public_key) {
    if !key.is_null() {
        kfree_sensitive((*key).key);
        kfree((*key).params);
        kfree(key);
    }
}

unsafe fn public_key_destroy(payload0: *mut core::ffi::c_void, payload3: *mut core::ffi::c_void) {
    public_key_free(payload0 as *mut public_key);
    public_key_signature_free(payload3 as *mut public_key_signature);
}

unsafe fn software_key_determine_akcipher(
    pkey: *const public_key,
    encoding: *const core::ffi::c_char,
    hash_algo: *const core::ffi::c_char,
    alg_name: *mut core::ffi::c_char,
    sig: *mut bool,
    op: kernel_pkey_operation,
) -> i32 {
    let mut n: i32;
    *sig = true;
    if encoding.is_null() { return -EINVAL; }

    if strcmp((*pkey).pkey_algo, b"rsa\0".as_ptr() as *const _) == 0 {
        if strcmp(encoding, b"pkcs1\0".as_ptr() as *const _) == 0 {
            *sig = op == kernel_pkey_sign || op == kernel_pkey_verify;
            if !*sig {
                n = snprintf(alg_name, CRYPTO_MAX_ALG_NAME, b"pkcs1pad(%s)\0".as_ptr() as *const _, (*pkey).pkey_algo);
            } else {
                let hash = if hash_algo.is_null() { b"none\0".as_ptr() as *const _ } else { hash_algo };
                n = snprintf(alg_name, CRYPTO_MAX_ALG_NAME, b"pkcs1(%s,%s)\0".as_ptr() as *const _, (*pkey).pkey_algo, hash);
            }
            return if n >= CRYPTO_MAX_ALG_NAME { -EINVAL } else { 0 };
        }
        if strcmp(encoding, b"raw\0".as_ptr() as *const _) != 0 || !hash_algo.is_null() { return -EINVAL; }
        *sig = false;
    } else if strncmp((*pkey).pkey_algo, b"ecdsa\0".as_ptr() as *const _, 5) == 0 {
        if strcmp(encoding, b"x962\0".as_ptr() as *const _) != 0 && strcmp(encoding, b"p1363\0".as_ptr() as *const _) != 0 { return -EINVAL; }
        if hash_algo.is_null() { return -EINVAL; }
        let allowed = [b"sha1\0", b"sha224\0", b"sha256\0", b"sha384\0", b"sha512\0", b"sha3-256\0", b"sha3-384\0", b"sha3-512\0"];
        if !allowed.iter().any(|x| strcmp(hash_algo, x.as_ptr() as *const _) == 0) { return -EINVAL; }
        n = snprintf(alg_name, CRYPTO_MAX_ALG_NAME, b"%s(%s)\0".as_ptr() as *const _, encoding, (*pkey).pkey_algo);
        return if n >= CRYPTO_MAX_ALG_NAME { -EINVAL } else { 0 };
    } else if strcmp((*pkey).pkey_algo, b"ecrdsa\0".as_ptr() as *const _) == 0 {
        if strcmp(encoding, b"raw\0".as_ptr() as *const _) != 0 || hash_algo.is_null() { return -EINVAL; }
        if strcmp(hash_algo, b"streebog256\0".as_ptr() as *const _) != 0 && strcmp(hash_algo, b"streebog512\0".as_ptr() as *const _) != 0 { return -EINVAL; }
    } else if strcmp((*pkey).pkey_algo, b"mldsa44\0".as_ptr() as *const _) == 0 || strcmp((*pkey).pkey_algo, b"mldsa65\0".as_ptr() as *const _) == 0 || strcmp((*pkey).pkey_algo, b"mldsa87\0".as_ptr() as *const _) == 0 {
        if strcmp(encoding, b"raw\0".as_ptr() as *const _) != 0 || hash_algo.is_null() { return -EINVAL; }
        if strcmp(hash_algo, b"none\0".as_ptr() as *const _) != 0 && strcmp(hash_algo, b"sha512\0".as_ptr() as *const _) != 0 { return -EINVAL; }
    } else { return -ENOPKG; }
    if strscpy(alg_name, (*pkey).pkey_algo, CRYPTO_MAX_ALG_NAME) < 0 { return -EINVAL; }
    0
}

unsafe fn pkey_pack_u32(dst: *mut u8, val: u32) -> *mut u8 {
    memcpy(dst as *mut _, &val as *const _ as *const _, core::mem::size_of::<u32>());
    dst.add(core::mem::size_of::<u32>())
}

// The remaining operations retain the C control flow and call the corresponding
// kernel crypto interfaces supplied by other translation units.
unsafe fn software_key_query(params: *const kernel_pkey_params, info: *mut kernel_pkey_query) -> i32 { todo!() }
unsafe fn software_key_eds_op(params: *mut kernel_pkey_params, _in: *const core::ffi::c_void, _out: *mut core::ffi::c_void) -> i32 { todo!() }
pub unsafe fn public_key_verify_signature(pkey: *const public_key, sig: *const public_key_signature) -> i32 { todo!() }
unsafe fn public_key_verify_signature_2(key: *const key, sig: *const public_key_signature) -> i32 { public_key_verify_signature((*key).payload.data[asym_crypto as usize] as *const public_key, sig) }

#[no_mangle]
pub static mut public_key_subtype: asymmetric_key_subtype = asymmetric_key_subtype {
    owner: THIS_MODULE, name: b"public_key\0".as_ptr() as *const _, name_len: 10,
    describe: Some(public_key_describe), destroy: Some(public_key_destroy),
    query: Some(software_key_query), eds_op: Some(software_key_eds_op),
    verify_signature: Some(public_key_verify_signature_2),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
