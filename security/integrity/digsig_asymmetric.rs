// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2013 Intel Corporation
 *
 * Author:
 * Dmitry Kasatkin <dmitry.kasatkin@intel.com>
 */

use core::ffi::{c_char, c_void};
use core::mem;
use core::ptr;

// External types from Linux kernel
extern "C" {
    type key;
    type key_ref_t;
    type public_key;
    type public_key_signature;
    type crypto_shash;
    type ima_file_id;
    type signature_v2_hdr;
    type ima_max_digest_data;
    type evm_ima_xattr_type;
    type hash_algo;
}

extern "C" {
    // Key management functions
    fn get_ima_blacklist_keyring() -> *mut key;
    fn keyring_search(
        kref: key_ref_t,
        key_type: *const c_void,
        name: *const c_char,
        recurse: bool,
    ) -> key_ref_t;
    fn key_type_asymmetric() -> *const c_void;
    fn request_key(
        key_type: *const c_void,
        name: *const c_char,
        callout_info: *const c_void,
    ) -> *mut key;
    fn asymmetric_key_public_key(key: *const key) -> *const public_key;
    fn key_put(key: *const key);
    fn key_serial(key: *const key) -> u32;

    // Crypto functions
    fn crypto_alloc_shash(
        name: *const c_char,
        type_: u32,
        mask: u32,
    ) -> *mut crypto_shash;
    fn crypto_free_shash(tfm: *mut crypto_shash);
    fn crypto_shash_tfm_digest(
        tfm: *mut crypto_shash,
        data: *const u8,
        len: usize,
        out: *mut u8,
    ) -> i32;
    fn verify_signature(key: *const key, pks: *const public_key_signature) -> i32;

    // Utility functions
    fn be32_to_cpu(x: u32) -> u32;
    fn be16_to_cpu(x: u16) -> u16;

    // External data
    static hash_algo_name: *const *const c_char;
    static hash_digest_size: *const usize;

    // Logging functions
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_err_ratelimited(fmt: *const c_char, ...);
}

// Linux kernel error handling macros/functions
// These are typically defined in linux/err.h but we model them as helpers
#[inline(always)]
unsafe fn IS_ERR(ptr: *const c_void) -> bool {
    (ptr as isize) < 0 && (ptr as isize) > -4096
}

#[inline(always)]
unsafe fn ERR_PTR(err: i64) -> *const c_void {
    err as *const c_void
}

#[inline(always)]
unsafe fn PTR_ERR(ptr: *const c_void) -> i64 {
    ptr as i64
}

#[inline(always)]
unsafe fn ERR_CAST(ptr: *const c_void) -> *mut key {
    ptr as *mut key
}

#[inline(always)]
unsafe fn key_ref_to_ptr(kref: key_ref_t) -> *const key {
    kref as *const key
}

#[inline(always)]
unsafe fn make_key_ref(key: *const key, possessed: i32) -> key_ref_t {
    ((key as usize) | (possessed as usize)) as key_ref_t
}

// Constants
const HASH_MAX_DIGESTSIZE: usize = 64;

// Error codes
const EKEYREJECTED: i64 = -129;
const EACCES: i64 = -13;
const ENOTDIR: i64 = -20;
const EAGAIN: i64 = -11;
const ENOKEY: i64 = -126;
const EBADMSG: i64 = -74;
const ENOPKG: i64 = -65;
const EINVAL: i64 = -22;

/*
 * Request an asymmetric key.
 */
unsafe fn request_asymmetric_key(keyring: *mut key, keyid: u32) -> *mut key {
    let mut name: [c_char; 12] = [0; 12];

    // Format string: "id:%08x"
    let mut buf = [b'i', b'd', b':', 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let hex = format!("{:08x}", keyid);
    let hex_bytes = hex.as_bytes();
    for (i, &b) in hex_bytes.iter().enumerate() {
        if i + 3 < 12 {
            buf[i + 3] = b;
        }
    }
    for i in 0..12 {
        name[i] = buf[i] as c_char;
    }

    let name_ptr = &name as *const [c_char; 12] as *const c_char;

    pr_debug(b"key search: \"%s\"\n" as *const u8 as *const c_char, name_ptr);

    let key = get_ima_blacklist_keyring();
    if !key.is_null() {
        let kref = keyring_search(
            make_key_ref(key, 1),
            key_type_asymmetric(),
            name_ptr,
            true,
        );
        if !IS_ERR(kref as *const c_void) {
            pr_err(
                b"Key '%s' is in ima_blacklist_keyring\n" as *const u8 as *const c_char,
                name_ptr,
            );
            return ERR_PTR(EKEYREJECTED) as *mut key;
        }
    }

    let result_key = if !keyring.is_null() {
        /* search in specific keyring */
        let kref = keyring_search(
            make_key_ref(keyring, 1),
            key_type_asymmetric(),
            name_ptr,
            true,
        );
        if IS_ERR(kref as *const c_void) {
            ERR_CAST(kref as *const c_void)
        } else {
            key_ref_to_ptr(kref) as *mut key
        }
    } else {
        request_key(key_type_asymmetric(), name_ptr, ptr::null())
    };

    if IS_ERR(result_key as *const c_void) {
        if !keyring.is_null() {
            pr_err_ratelimited(
                b"Request for unknown key '%s' in '%s' keyring. err %ld\n" as *const u8
                    as *const c_char,
                name_ptr,
                (*keyring).description,
                PTR_ERR(result_key as *const c_void),
            );
        } else {
            pr_err_ratelimited(
                b"Request for unknown key '%s' err %ld\n" as *const u8 as *const c_char,
                name_ptr,
                PTR_ERR(result_key as *const c_void),
            );
        }

        match PTR_ERR(result_key as *const c_void) {
            /* Hide some search errors */
            -13 | -20 | -11 => {
                return ERR_PTR(ENOKEY) as *mut key;
            }
            _ => {
                return result_key;
            }
        }
    }

    pr_debug(
        b"%s() = 0 [%x]\n" as *const u8 as *const c_char,
        b"request_asymmetric_key" as *const u8 as *const c_char,
        key_serial(result_key as *const key),
    );

    result_key
}

/**
 * asymmetric_verify_common -- sigv2 and sigv3 common verify function
 * @key: The key to use for signature verification; caller must free it
 * @pk: The associated public key; must not be NULL
 * @sig: The xattr signature
 * @siglen: The length of the xattr signature; must be at least
 *          sizeof(struct signature_v2_hdr)
 * @data: The data to verify the signature on
 * @datalen: Length of @data
 */
unsafe fn asymmetric_verify_common(
    key: *const key,
    pk: *const public_key,
    sig: *const c_char,
    siglen: i32,
    data: *const c_char,
    datalen: i32,
) -> i32 {
    let hdr = sig as *const signature_v2_hdr;
    let mut pks: public_key_signature = mem::zeroed();
    let mut ret: i32;

    let mut sig_len = siglen - mem::size_of::<signature_v2_hdr>() as i32;

    if sig_len != be16_to_cpu((*hdr).sig_size as u16) as i32 {
        return -74; // EBADMSG
    }

    if (*hdr).hash_algo >= 256 {
        // HASH_ALGO__LAST placeholder
        return -65; // ENOPKG
    }

    mem::write_bytes(&mut pks as *mut public_key_signature as *mut c_char, 0, mem::size_of::<public_key_signature>());

    pks.hash_algo = *hash_algo_name.add((*hdr).hash_algo as usize);
    pks.pkey_algo = (*pk).pkey_algo;
    if c_str_eq(pks.pkey_algo, b"rsa\0") {
        pks.encoding = b"pkcs1\0" as *const u8 as *const c_char;
    } else if c_str_startswith(pks.pkey_algo, b"ecdsa-\0") {
        /* edcsa-nist-p192 etc. */
        pks.encoding = b"x962\0" as *const u8 as *const c_char;
    } else if c_str_eq(pks.pkey_algo, b"ecrdsa\0") {
        pks.encoding = b"raw\0" as *const u8 as *const c_char;
    } else {
        ret = -65; // ENOPKG
        // goto out
    }

    pks.m = (data) as *mut u8;
    pks.m_size = datalen as usize;
    pks.s = ((*hdr).sig) as *mut u8;
    pks.s_size = sig_len as usize;
    ret = verify_signature(key, &pks as *const public_key_signature);

    // out:
    pr_debug(
        b"%s() = %d\n" as *const u8 as *const c_char,
        b"asymmetric_verify_common" as *const u8 as *const c_char,
        ret,
    );
    ret
}

#[inline]
unsafe fn c_str_eq(a: *const c_char, b: &[u8]) -> bool {
    if a.is_null() {
        return false;
    }
    let mut i = 0;
    loop {
        let c1 = *a.add(i);
        let c2 = b[i] as c_char;
        if c1 != c2 {
            return false;
        }
        if c1 == 0 {
            return true;
        }
        i += 1;
    }
}

#[inline]
unsafe fn c_str_startswith(a: *const c_char, b: &[u8]) -> bool {
    if a.is_null() {
        return false;
    }
    let mut i = 0;
    loop {
        let c2 = b[i] as c_char;
        if c2 == 0 {
            return true;
        }
        let c1 = *a.add(i);
        if c1 != c2 {
            return false;
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn asymmetric_verify(
    keyring: *mut key,
    sig: *const c_char,
    siglen: i32,
    data: *const c_char,
    datalen: i32,
) -> i32 {
    let hdr = sig as *const signature_v2_hdr;
    let mut pk: *const public_key;
    let mut key: *mut key;
    let mut ret: i32;

    if siglen <= mem::size_of::<signature_v2_hdr>() as i32 {
        return -74; // EBADMSG
    }

    key = request_asymmetric_key(keyring, be32_to_cpu((*hdr).keyid));
    if IS_ERR(key as *const c_void) {
        return PTR_ERR(key as *const c_void) as i32;
    }
    pk = asymmetric_key_public_key(key as *const key);
    if pk.is_null() {
        ret = -126; // ENOKEY
        // goto out
    } else {
        ret = asymmetric_verify_common(key as *const key, pk, sig, siglen, data, datalen);
    }

    // out:
    key_put(key as *const key);

    ret
}

/*
 * calc_file_id_hash - calculate the hash of the ima_file_id struct data
 * @type: xattr type [enum evm_ima_xattr_type]
 * @algo: hash algorithm [enum hash_algo]; caller must ensure valid value
 * @digest: pointer to the digest to be hashed
 * @hash: (out) pointer to the hash
 *
 * IMA signature version 3 disambiguates the data that is signed by
 * indirectly signing the hash of the ima_file_id structure data.
 *
 * Return 0 on success, error code otherwise.
 */
unsafe fn calc_file_id_hash(
    type_: u32,
    algo: u32,
    digest: *const u8,
    hash: *mut ima_max_digest_data,
) -> i32 {
    let mut file_id: ima_file_id = mem::zeroed();
    let digest_size = *hash_digest_size.add(algo as usize);
    let mut tfm: *mut crypto_shash;
    let file_id_size: usize;
    let rc: i32;

    // Check type
    if type_ != 0 && type_ != 1 && type_ != 2 {
        // IMA_VERITY_DIGSIG, EVM_IMA_XATTR_DIGSIG, EVM_XATTR_PORTABLE_DIGSIG
        return -22; // EINVAL
    }

    tfm = crypto_alloc_shash(
        *hash_algo_name.add(algo as usize),
        0,
        0,
    );
    if IS_ERR(tfm as *const c_void) {
        return PTR_ERR(tfm as *const c_void) as i32;
    }

    ptr::copy_nonoverlapping(digest, (&mut file_id).hash as *mut u8, digest_size);

    /* Calculate the ima_file_id struct hash on the portion used. */
    file_id_size = mem::size_of::<ima_file_id>() - (HASH_MAX_DIGESTSIZE - digest_size);

    (*hash).hdr.algo = algo as u8;
    (*hash).hdr.length = digest_size as u16;
    rc = crypto_shash_tfm_digest(
        tfm,
        &file_id as *const ima_file_id as *const u8,
        file_id_size,
        (*hash).digest as *mut u8,
    );

    crypto_free_shash(tfm);
    rc
}

/**
 * asymmetric_verify_v3_hashless - Use hashless signature verification on sigv3
 * @key: The key to use for signature verification; caller must free it
 * @pk: The associated public key; must not be NULL
 * @encoding: The encoding the key type uses
 * @sig: The xattr signature
 * @siglen: The length of the xattr signature; must be at least
 *          sizeof(struct signature_v2_hdr)
 * @algo: hash algorithm [enum hash_algo]; caller must ensure valid value
 * @digest: The file digest
 *
 * Create an ima_file_id structure and use it for signature verification
 * directly. This can be used for ML-DSA in pure mode for example.
 */
unsafe fn asymmetric_verify_v3_hashless(
    key: *mut key,
    pk: *const public_key,
    encoding: *const c_char,
    sig: *const c_char,
    siglen: i32,
    algo: u8,
    digest: *const u8,
) -> i32 {
    let hdr = sig as *const signature_v2_hdr;
    let mut file_id: ima_file_id = mem::zeroed();
    let digest_size = *hash_digest_size.add(algo as usize);
    let mut pks: public_key_signature = mem::zeroed();
    let ret: i32;

    file_id.hash_type = (*hdr).type_;
    file_id.hash_algorithm = algo;

    pks.m = (&file_id) as *const ima_file_id as *mut u8;
    pks.m_size = mem::size_of::<ima_file_id>() - (HASH_MAX_DIGESTSIZE - digest_size);
    pks.s = ((*hdr).sig) as *mut u8;
    pks.s_size = (siglen - mem::size_of::<signature_v2_hdr>() as i32) as usize;
    pks.pkey_algo = (*pk).pkey_algo;
    pks.hash_algo = b"none\0" as *const u8 as *const c_char;
    pks.encoding = encoding;

    if (*hdr).type_ != 0 && (*hdr).type_ != 1 && (*hdr).type_ != 2 {
        // IMA_VERITY_DIGSIG, EVM_IMA_XATTR_DIGSIG, EVM_XATTR_PORTABLE_DIGSIG
        return -22; // EINVAL
    }

    if pks.s_size != be16_to_cpu((*hdr).sig_size as u16) as usize {
        return -74; // EBADMSG
    }

    ptr::copy_nonoverlapping(digest, (&mut file_id).hash as *mut u8, digest_size);

    ret = verify_signature(key as *const key, &pks as *const public_key_signature);
    pr_debug(
        b"%s() = %d\n" as *const u8 as *const c_char,
        b"asymmetric_verify_v3_hashless" as *const u8 as *const c_char,
        ret,
    );
    ret
}

#[no_mangle]
pub unsafe extern "C" fn asymmetric_verify_v3(
    keyring: *mut key,
    sig: *const c_char,
    siglen: i32,
    data: *const c_char,
    datalen: i32,
    algo: u8,
) -> i32 {
    let hdr = sig as *const signature_v2_hdr;
    let mut hash: ima_max_digest_data = mem::zeroed();
    let mut pk: *const public_key;
    let mut key: *mut key;
    let rc: i32;

    if (algo as u32) >= 256 {
        // HASH_ALGO__LAST placeholder
        return -65; // ENOPKG
    }

    if siglen <= mem::size_of::<signature_v2_hdr>() as i32 {
        return -74; // EBADMSG
    }

    key = request_asymmetric_key(keyring, be32_to_cpu((*hdr).keyid));
    if IS_ERR(key as *const c_void) {
        return PTR_ERR(key as *const c_void) as i32;
    }

    pk = asymmetric_key_public_key(key as *const key);
    if pk.is_null() {
        let rc = -126; // ENOKEY
        key_put(key as *const key);
        return rc;
    }
    if c_str_startswith((*pk).pkey_algo, b"mldsa\0") {
        let rc = asymmetric_verify_v3_hashless(
            key,
            pk,
            b"raw\0" as *const u8 as *const c_char,
            sig,
            siglen,
            algo,
            data as *const u8,
        );
        key_put(key as *const key);
        return rc;
    } else {
        let rc = calc_file_id_hash((*hdr).type_, algo as u32, data as *const u8, &mut hash);
        if rc != 0 {
            key_put(key as *const key);
            return -22; // EINVAL
        }

        let rc = asymmetric_verify_common(
            key as *const key,
            pk,
            sig,
            siglen,
            hash.digest as *const c_char,
            hash.hdr.length as i32,
        );
        key_put(key as *const key);
        return rc;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
