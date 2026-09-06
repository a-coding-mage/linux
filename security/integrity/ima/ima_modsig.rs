// SPDX-License-Identifier: GPL-2.0+
/*
 * IMA support for appraising module-style appended signatures.
 *
 * Copyright (C) 2019  IBM Corporation
 *
 * Author:
 * Thiago Jung Bauermann <bauerman@linux.ibm.com>
 */

// External dependencies from Linux kernel headers:
// #include <linux/types.h>
// #include <linux/module_signature.h>
// #include <keys/asymmetric-type.h>
// #include <crypto/pkcs7.h>
// #include "ima.h"

use core::mem;
use core::ptr::{self, null, null_mut};
use core::ffi::c_void;

// External type declarations - these types are defined in other kernel headers
#[repr(C)]
pub struct pkcs7_message {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct key {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct module_signature {
    _opaque: [u8; 0],
}

// Type aliases for external enums - these are assumed to be u32-sized enums
pub type HashAlgo = u32;
pub type ImaHooks = u32;

#[repr(C)]
pub struct modsig {
    pub pkcs7_msg: *mut pkcs7_message,
    pub hash_algo: HashAlgo,
    pub digest: *const u8,
    pub digest_size: u32,
    pub raw_pkcs7_len: i32,
    // Flexible array member: represented as zero-length array
    pub raw_pkcs7: [u8; 0],
}

// External function and constant declarations
extern "C" {
    fn strlen(s: *const i8) -> usize;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> i32;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;

    fn kzalloc_flex(hdr: *mut modsig, field: *const [u8], size: usize) -> *mut modsig;
    fn kfree(ptr: *mut c_void);

    fn be32_to_cpu(x: u32) -> u32;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> i32;

    fn mod_check_sig(sig: *const module_signature, buf_len: usize, token: *const u8) -> i32;
    fn pkcs7_parse_message(buf: *const c_void, len: usize) -> *mut pkcs7_message;
    fn pkcs7_supply_detached_data(msg: *mut pkcs7_message, buf: *const c_void, size: usize) -> i32;
    fn pkcs7_get_digest(
        msg: *mut pkcs7_message,
        digest: *mut *const u8,
        digest_size: *mut u32,
        hash_algo: *mut HashAlgo,
    ) -> i32;
    fn verify_pkcs7_message_sig(
        data: *const c_void,
        len: usize,
        msg: *mut pkcs7_message,
        keyring: *mut key,
        usage: u32,
        arg1: *const c_void,
        arg2: *const c_void,
    ) -> i32;
    fn pkcs7_free_message(msg: *mut pkcs7_message);

    static MODULE_SIGNATURE_MARKER: *const i8;
    static func_tokens: *const *const u8;
    static VERIFYING_MODULE_SIGNATURE: u32;
    static HASH_ALGO__LAST: HashAlgo;
}

/*
 * ima_read_modsig - Read modsig from buf.
 *
 * Return: 0 on success, error code otherwise.
 */
pub unsafe extern "C" fn ima_read_modsig(
    func: ImaHooks,
    buf: *const c_void,
    buf_len: isize,
    modsig: *mut *mut modsig,
) -> i32 {
    const ENOENT: i32 = -2;
    const ENOMEM: i32 = -12;

    let marker_len = strlen(MODULE_SIGNATURE_MARKER);
    let sig: *const module_signature;
    let sig_len: usize;
    let p: *const c_void;
    let mut rc: i32;
    let hdr: *mut modsig;

    if buf_len <= (marker_len + mem::size_of::<module_signature>()) as isize {
        return ENOENT;
    }

    p = (buf as *const u8).add((buf_len as usize) - marker_len) as *const c_void;
    if memcmp(p, MODULE_SIGNATURE_MARKER as *const c_void, marker_len) != 0 {
        return ENOENT;
    }

    let buf_len = buf_len - (marker_len as isize);
    sig = ((p as *const u8).offset(-(mem::size_of::<module_signature>() as isize))) as *const module_signature;

    rc = mod_check_sig(sig, buf_len as usize, *func_tokens.add(func as usize));
    if rc != 0 {
        return rc;
    }

    sig_len = be32_to_cpu(*(sig as *const u32));
    let buf_len = buf_len - (sig_len as isize) - (mem::size_of::<module_signature>() as isize);

    // Allocate sig_len additional bytes to hold the raw PKCS#7 data.
    hdr = kzalloc_flex(null_mut(), &(*null_mut::<modsig>()).raw_pkcs7 as *const [u8], sig_len);
    if hdr.is_null() {
        return ENOMEM;
    }

    (*hdr).raw_pkcs7_len = sig_len as i32;
    (*hdr).pkcs7_msg = pkcs7_parse_message(
        (buf as *const u8).add(buf_len as usize) as *const c_void,
        sig_len,
    );
    if IS_ERR((*hdr).pkcs7_msg as *const c_void) {
        rc = PTR_ERR((*hdr).pkcs7_msg as *const c_void);
        kfree(hdr as *mut c_void);
        return rc;
    }

    memcpy(
        &(*hdr).raw_pkcs7 as *const _ as *mut c_void,
        (buf as *const u8).add(buf_len as usize) as *const c_void,
        sig_len,
    );

    // We don't know the hash algorithm yet.
    (*hdr).hash_algo = HASH_ALGO__LAST;

    *modsig = hdr;

    0
}

/**
 * ima_collect_modsig - Calculate the file hash without the appended signature.
 * @modsig: parsed module signature
 * @buf: data to verify the signature on
 * @size: data size
 *
 * Since the modsig is part of the file contents, the hash used in its signature
 * isn't the same one ordinarily calculated by IMA. Therefore PKCS7 code
 * calculates a separate one for signature verification.
 */
pub unsafe extern "C" fn ima_collect_modsig(
    modsig: *mut modsig,
    buf: *const c_void,
    size: isize,
) {
    let mut rc: i32;

    /*
     * Provide the file contents (minus the appended sig) so that the PKCS7
     * code can calculate the file hash.
     */
    let size = size
        - ((*modsig).raw_pkcs7_len as isize)
        - (strlen(MODULE_SIGNATURE_MARKER) as isize)
        - (mem::size_of::<module_signature>() as isize);
    rc = pkcs7_supply_detached_data((*modsig).pkcs7_msg, buf, size as usize);
    if rc != 0 {
        return;
    }

    // Ask the PKCS7 code to calculate the file hash.
    rc = pkcs7_get_digest(
        (*modsig).pkcs7_msg,
        &mut (*modsig).digest,
        &mut (*modsig).digest_size,
        &mut (*modsig).hash_algo,
    );
}

pub unsafe extern "C" fn ima_modsig_verify(keyring: *mut key, modsig: *const modsig) -> i32 {
    verify_pkcs7_message_sig(
        null(),
        0,
        (*modsig).pkcs7_msg,
        keyring,
        VERIFYING_MODULE_SIGNATURE,
        null(),
        null(),
    )
}

pub unsafe extern "C" fn ima_get_modsig_digest(
    modsig: *const modsig,
    algo: *mut HashAlgo,
    digest: *mut *const u8,
    digest_size: *mut u32,
) -> i32 {
    *algo = (*modsig).hash_algo;
    *digest = (*modsig).digest;
    *digest_size = (*modsig).digest_size;

    0
}

pub unsafe extern "C" fn ima_get_raw_modsig(
    modsig: *const modsig,
    data: *mut *const c_void,
    data_len: *mut u32,
) -> i32 {
    *data = &(*modsig).raw_pkcs7 as *const _ as *const c_void;
    *data_len = (*modsig).raw_pkcs7_len as u32;

    0
}

pub unsafe extern "C" fn ima_free_modsig(modsig: *mut modsig) {
    if modsig.is_null() {
        return;
    }

    pkcs7_free_message((*modsig).pkcs7_msg);
    kfree(modsig as *mut c_void);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
