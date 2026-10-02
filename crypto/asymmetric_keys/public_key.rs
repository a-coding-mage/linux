// SPDX-License-Identifier: GPL-2.0-or-later
/* In-software asymmetric public-key crypto subtype.
 * Translated from the unchanged public_key.c; shared layouts come from C headers.
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 */
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    missing_docs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    unreachable_pub
)]
#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/public_key_generated.rs"
    ));
}
use bindings::*;
use core::ptr::null_mut;
use kernel::ffi::{c_char, c_int, c_void};
#[cfg(MODULE)]
extern "C" {
    static mut __this_module: module;
}
#[path = "../../rust/ffi_export.rs"]
mod ffi_export;

#[inline]
fn neg(error: u32) -> c_int {
    -(error as c_int)
}
#[inline]
fn is_err<T>(ptr: *const T) -> bool {
    ptr as usize >= usize::MAX - 4094
}
#[inline]
fn ptr_err<T>(ptr: *const T) -> c_int {
    ptr as isize as c_int
}

#[inline]
unsafe fn key_payload(key: *const key) -> *mut public_key {
    (*key).__bindgen_anon_4.payload.data[asymmetric_payload_bits_asym_crypto as usize].cast()
}

unsafe extern "C" fn public_key_describe(asymmetric_key: *const key, m: *mut seq_file) {
    let key = key_payload(asymmetric_key);
    if !key.is_null() {
        seq_printf(
            m,
            c"%s.%s".as_ptr().cast(),
            (*key).id_type,
            (*key).pkey_algo,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn public_key_free(key: *mut public_key) {
    if !key.is_null() {
        kfree_sensitive((*key).key);
        kfree((*key).params);
        kfree(key.cast());
    }
}

unsafe extern "C" fn public_key_destroy(payload0: *mut c_void, payload3: *mut c_void) {
    public_key_free(payload0.cast());
    public_key_signature_free(payload3.cast());
}

unsafe fn software_key_determine_akcipher(
    pkey: *const public_key,
    encoding: *const c_char,
    hash_algo: *const c_char,
    alg_name: *mut c_char,
    sig: *mut bool,
    op: kernel_pkey_operation,
) -> c_int {
    *sig = true;
    if encoding.is_null() {
        return neg(EINVAL);
    }
    if strcmp((*pkey).pkey_algo, c"rsa".as_ptr().cast()) == 0 {
        if strcmp(encoding, c"pkcs1".as_ptr().cast()) == 0 {
            *sig = op == kernel_pkey_operation_kernel_pkey_sign
                || op == kernel_pkey_operation_kernel_pkey_verify;
            let n = if !*sig {
                snprintf(
                    alg_name,
                    CRYPTO_MAX_ALG_NAME as usize,
                    c"pkcs1pad(%s)".as_ptr().cast(),
                    (*pkey).pkey_algo,
                )
            } else {
                let hash = if hash_algo.is_null() {
                    c"none".as_ptr().cast()
                } else {
                    hash_algo
                };
                snprintf(
                    alg_name,
                    CRYPTO_MAX_ALG_NAME as usize,
                    c"pkcs1(%s,%s)".as_ptr().cast(),
                    (*pkey).pkey_algo,
                    hash,
                )
            };
            return if n >= CRYPTO_MAX_ALG_NAME as c_int {
                neg(EINVAL)
            } else {
                0
            };
        }
        if strcmp(encoding, c"raw".as_ptr().cast()) != 0 || !hash_algo.is_null() {
            return neg(EINVAL);
        }
        *sig = false;
    } else if strncmp((*pkey).pkey_algo, c"ecdsa".as_ptr().cast(), 5) == 0 {
        if strcmp(encoding, c"x962".as_ptr().cast()) != 0
            && strcmp(encoding, c"p1363".as_ptr().cast()) != 0
        {
            return neg(EINVAL);
        }
        if hash_algo.is_null() {
            return neg(EINVAL);
        }
        let allowed = [
            c"sha1",
            c"sha224",
            c"sha256",
            c"sha384",
            c"sha512",
            c"sha3-256",
            c"sha3-384",
            c"sha3-512",
        ];
        if !allowed
            .iter()
            .any(|hash| strcmp(hash_algo, hash.as_ptr().cast()) == 0)
        {
            return neg(EINVAL);
        }
        let n = snprintf(
            alg_name,
            CRYPTO_MAX_ALG_NAME as usize,
            c"%s(%s)".as_ptr().cast(),
            encoding,
            (*pkey).pkey_algo,
        );
        return if n >= CRYPTO_MAX_ALG_NAME as c_int {
            neg(EINVAL)
        } else {
            0
        };
    } else if strcmp((*pkey).pkey_algo, c"ecrdsa".as_ptr().cast()) == 0 {
        if strcmp(encoding, c"raw".as_ptr().cast()) != 0 || hash_algo.is_null() {
            return neg(EINVAL);
        }
        if strcmp(hash_algo, c"streebog256".as_ptr().cast()) != 0
            && strcmp(hash_algo, c"streebog512".as_ptr().cast()) != 0
        {
            return neg(EINVAL);
        }
    } else if strcmp((*pkey).pkey_algo, c"mldsa44".as_ptr().cast()) == 0
        || strcmp((*pkey).pkey_algo, c"mldsa65".as_ptr().cast()) == 0
        || strcmp((*pkey).pkey_algo, c"mldsa87".as_ptr().cast()) == 0
    {
        if strcmp(encoding, c"raw".as_ptr().cast()) != 0 || hash_algo.is_null() {
            return neg(EINVAL);
        }
        if strcmp(hash_algo, c"none".as_ptr().cast()) != 0
            && strcmp(hash_algo, c"sha512".as_ptr().cast()) != 0
        {
            return neg(EINVAL);
        }
    } else {
        return neg(ENOPKG);
    }
    if sized_strscpy(alg_name, (*pkey).pkey_algo, CRYPTO_MAX_ALG_NAME as usize) < 0 {
        return neg(EINVAL);
    }
    0
}

// This is the C key + native-endian u32 algorithm + native-endian u32 length +
// parameter buffer. The crypto setkey length intentionally excludes the suffix.
unsafe fn pack_key(pkey: *const public_key) -> *mut u8 {
    let size = ((*pkey).keylen as usize)
        .wrapping_add(2 * core::mem::size_of::<u32>())
        .wrapping_add((*pkey).paramlen as usize);
    let key = rust_pkey_kmalloc(size).cast::<u8>();
    if key.is_null() {
        return key;
    }
    if (*pkey).keylen != 0 {
        core::ptr::copy_nonoverlapping((*pkey).key.cast::<u8>(), key, (*pkey).keylen as usize);
    }
    let mut ptr = key.add((*pkey).keylen as usize);
    ptr.cast::<u32>().write_unaligned((*pkey).algo as u32);
    ptr = ptr.add(core::mem::size_of::<u32>());
    ptr.cast::<u32>().write_unaligned((*pkey).paramlen);
    ptr = ptr.add(core::mem::size_of::<u32>());
    if (*pkey).paramlen != 0 {
        core::ptr::copy_nonoverlapping((*pkey).params.cast::<u8>(), ptr, (*pkey).paramlen as usize);
    }
    key
}

unsafe fn set_sig_key(tfm: *mut crypto_sig, pkey: *const public_key, key: *const u8) -> c_int {
    if (*pkey).key_is_private {
        rust_pkey_sig_set_privkey(tfm, key.cast(), (*pkey).keylen)
    } else {
        rust_pkey_sig_set_pubkey(tfm, key.cast(), (*pkey).keylen)
    }
}
unsafe fn set_akcipher_key(
    tfm: *mut crypto_akcipher,
    pkey: *const public_key,
    key: *const u8,
) -> c_int {
    if (*pkey).key_is_private {
        rust_pkey_akcipher_set_priv_key(tfm, key.cast(), (*pkey).keylen)
    } else {
        rust_pkey_akcipher_set_pub_key(tfm, key.cast(), (*pkey).keylen)
    }
}

unsafe extern "C" fn software_key_query(
    params: *const kernel_pkey_params,
    info: *mut kernel_pkey_query,
) -> c_int {
    let pkey = key_payload((*params).key);
    let mut alg_name = [0 as c_char; CRYPTO_MAX_ALG_NAME as usize];
    let mut issig = false;
    let mut ret = software_key_determine_akcipher(
        pkey,
        (*params).encoding,
        (*params).hash_algo,
        alg_name.as_mut_ptr(),
        &mut issig,
        kernel_pkey_operation_kernel_pkey_sign,
    );
    if ret < 0 {
        return ret;
    }
    let key = pack_key(pkey);
    if key.is_null() {
        return neg(ENOMEM);
    }
    core::ptr::write_bytes(info, 0, 1);
    if issig {
        let sig = crypto_alloc_sig(alg_name.as_ptr(), 0, 0);
        if is_err(sig) {
            ret = ptr_err(sig);
        } else {
            ret = set_sig_key(sig, pkey, key);
            if ret >= 0 {
                let len = rust_pkey_sig_keysize(sig) as c_int;
                (*info).key_size = len as u32;
                (*info).max_sig_size = rust_pkey_sig_maxsize(sig) as u16;
                (*info).max_data_size = rust_pkey_sig_digestsize(sig) as u16;
                (*info).supported_ops = KEYCTL_SUPPORTS_VERIFY;
                if (*pkey).key_is_private {
                    (*info).supported_ops |= KEYCTL_SUPPORTS_SIGN;
                }
                if strcmp((*params).encoding, c"pkcs1".as_ptr().cast()) == 0 {
                    (*info).max_enc_size = (len / 8) as u16;
                    (*info).max_dec_size = (len / 8) as u16;
                    (*info).supported_ops |= KEYCTL_SUPPORTS_ENCRYPT;
                    if (*pkey).key_is_private {
                        (*info).supported_ops |= KEYCTL_SUPPORTS_DECRYPT;
                    }
                }
            }
            rust_pkey_free_sig(sig);
        }
    } else {
        let tfm = crypto_alloc_akcipher(alg_name.as_ptr(), 0, 0);
        if is_err(tfm) {
            ret = ptr_err(tfm);
        } else {
            ret = set_akcipher_key(tfm, pkey, key);
            if ret >= 0 {
                let len = rust_pkey_akcipher_maxsize(tfm) as c_int;
                (*info).key_size = len.wrapping_mul(8) as u32;
                (*info).max_sig_size = len as u16;
                (*info).max_data_size = len as u16;
                (*info).max_enc_size = len as u16;
                (*info).max_dec_size = len as u16;
                (*info).supported_ops = KEYCTL_SUPPORTS_ENCRYPT;
                if (*pkey).key_is_private {
                    (*info).supported_ops |= KEYCTL_SUPPORTS_DECRYPT;
                }
            }
            rust_pkey_free_akcipher(tfm);
        }
    }
    kfree_sensitive(key.cast());
    rust_pkey_log_exit(c"software_key_query".as_ptr().cast(), ret);
    ret
}

unsafe extern "C" fn software_key_eds_op(
    params: *mut kernel_pkey_params,
    input: *const c_void,
    output: *mut c_void,
) -> c_int {
    rust_pkey_log_enter(c"software_key_eds_op".as_ptr().cast());
    let pkey = key_payload((*params).key);
    let mut alg_name = [0 as c_char; CRYPTO_MAX_ALG_NAME as usize];
    let mut issig = false;
    let mut ret = software_key_determine_akcipher(
        pkey,
        (*params).encoding,
        (*params).hash_algo,
        alg_name.as_mut_ptr(),
        &mut issig,
        (*params).op(),
    );
    if ret < 0 {
        return ret;
    }
    let key = pack_key(pkey);
    if key.is_null() {
        return neg(ENOMEM);
    }
    let mut tfm = null_mut();
    let mut sig = null_mut();
    if issig {
        sig = crypto_alloc_sig(alg_name.as_ptr(), 0, 0);
        if is_err(sig) {
            ret = ptr_err(sig);
            kfree_sensitive(key.cast());
            rust_pkey_log_exit(c"software_key_eds_op".as_ptr().cast(), ret);
            return ret;
        }
        ret = set_sig_key(sig, pkey, key);
    } else {
        tfm = crypto_alloc_akcipher(alg_name.as_ptr(), 0, 0);
        if is_err(tfm) {
            ret = ptr_err(tfm);
            kfree_sensitive(key.cast());
            rust_pkey_log_exit(c"software_key_eds_op".as_ptr().cast(), ret);
            return ret;
        }
        ret = set_akcipher_key(tfm, pkey, key);
    }
    if ret == 0 {
        ret = neg(EINVAL);
        match (*params).op() {
            kernel_pkey_operation_kernel_pkey_encrypt => {
                if !issig {
                    ret = crypto_akcipher_sync_encrypt(
                        tfm,
                        input,
                        (*params).in_len,
                        output,
                        (*params).__bindgen_anon_1.out_len,
                    );
                }
            }
            kernel_pkey_operation_kernel_pkey_decrypt => {
                if !issig {
                    ret = crypto_akcipher_sync_decrypt(
                        tfm,
                        input,
                        (*params).in_len,
                        output,
                        (*params).__bindgen_anon_1.out_len,
                    );
                }
            }
            kernel_pkey_operation_kernel_pkey_sign => {
                if issig {
                    ret = rust_pkey_sig_sign(
                        sig,
                        input,
                        (*params).in_len,
                        output,
                        (*params).__bindgen_anon_1.out_len,
                    );
                }
            }
            _ => kernel::bindings::BUG(),
        }
        if !issig && ret == 0 {
            ret = rust_pkey_akcipher_maxsize(tfm) as c_int;
        }
    }
    if issig {
        rust_pkey_free_sig(sig);
    } else {
        rust_pkey_free_akcipher(tfm);
    }
    kfree_sensitive(key.cast());
    rust_pkey_log_exit(c"software_key_eds_op".as_ptr().cast(), ret);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn public_key_verify_signature(
    pkey: *const public_key,
    sig: *const public_key_signature,
) -> c_int {
    rust_pkey_log_enter(c"public_key_verify_signature".as_ptr().cast());
    if pkey.is_null() {
        kernel::bindings::BUG();
    }
    if sig.is_null() {
        kernel::bindings::BUG();
    }
    if (*sig).s.is_null() {
        kernel::bindings::BUG();
    }
    if !(*sig).pkey_algo.is_null()
        && strcmp((*pkey).pkey_algo, (*sig).pkey_algo) != 0
        && (strncmp((*pkey).pkey_algo, c"ecdsa-".as_ptr().cast(), 6) != 0
            || strcmp((*sig).pkey_algo, c"ecdsa".as_ptr().cast()) != 0)
    {
        return neg(EKEYREJECTED);
    }
    let mut alg_name = [0 as c_char; CRYPTO_MAX_ALG_NAME as usize];
    let mut issig = false;
    let mut ret = software_key_determine_akcipher(
        pkey,
        (*sig).encoding,
        (*sig).hash_algo,
        alg_name.as_mut_ptr(),
        &mut issig,
        kernel_pkey_operation_kernel_pkey_verify,
    );
    if ret < 0 {
        return ret;
    }
    let tfm = crypto_alloc_sig(alg_name.as_ptr(), 0, 0);
    if is_err(tfm) {
        return ptr_err(tfm);
    }
    let key = pack_key(pkey);
    if key.is_null() {
        ret = neg(ENOMEM);
    } else {
        ret = set_sig_key(tfm, pkey, key);
        if ret == 0 {
            ret = rust_pkey_sig_verify(
                tfm,
                (*sig).s.cast(),
                (*sig).s_size,
                (*sig).m.cast(),
                (*sig).m_size,
            );
        }
        kfree_sensitive(key.cast());
    }
    rust_pkey_free_sig(tfm);
    rust_pkey_log_exit(c"public_key_verify_signature".as_ptr().cast(), ret);
    if rust_pkey_warn_verify(ret > 0) {
        ret = neg(EINVAL);
    }
    ret
}

unsafe extern "C" fn public_key_verify_signature_2(
    key: *const key,
    sig: *const public_key_signature,
) -> c_int {
    public_key_verify_signature(key_payload(key), sig)
}

#[no_mangle]
pub static mut public_key_subtype: asymmetric_key_subtype = asymmetric_key_subtype {
    #[cfg(MODULE)]
    owner: core::ptr::addr_of_mut!(__this_module),
    #[cfg(not(MODULE))]
    owner: null_mut(),
    name: c"public_key".as_ptr().cast(),
    name_len: 10,
    describe: Some(public_key_describe),
    destroy: Some(public_key_destroy),
    query: Some(software_key_query),
    eds_op: Some(software_key_eds_op),
    verify_signature: Some(public_key_verify_signature_2),
};
ffi_export::export_symbol!(public_key_free, public_key_free, "GPL", "");
ffi_export::export_symbol!(
    public_key_verify_signature,
    public_key_verify_signature,
    "GPL",
    ""
);
ffi_export::export_symbol!(public_key_subtype, public_key_subtype, "GPL", "");
// C reference: public_key.c at 81bb5d6483714efb4cee248d4fdb703c592723d2
// C blob: 09a0b83d5d77f918749f5fd4e70c67a8f4163960
