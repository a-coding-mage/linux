// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2024 Microsoft Corporation. All rights reserved.
 */

// External Linux kernel dependencies: linux/hex.h, digest.h types
use core::ffi::CStr;
use core::mem;
use core::ptr::{self, NonNull};

// Linux kernel type declarations (from digest.h)
// These are assumed to be defined in external dependencies
#[repr(C)]
pub struct digest_info {
    pub alg: *mut u8,
    pub digest: *mut u8,
    pub digest_len: usize,
}

// Linux kernel type declarations (assumed from external dependencies)
#[repr(C)]
pub struct audit_buffer {
    _private: [u8; 0],
}

// External Linux kernel functions
extern "C" {
    fn kzalloc(size: usize, flags: i32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *const core::ffi::c_void);
    fn kstrndup(s: *const u8, n: usize, flags: i32) -> *mut u8;
    fn strchr(s: *const u8, c: i32) -> *mut u8;
    fn strlen(s: *const u8) -> usize;
    fn strcmp(s1: *const u8, s2: *const u8) -> i32;
    fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32;
    fn hex2bin(dst: *mut u8, src: *const u8, count: usize) -> i32;
    fn audit_log_untrustedstring(ab: *mut audit_buffer, s: *const u8);
    fn audit_log_format(ab: *mut audit_buffer, fmt: *const u8, ...);
    fn audit_log_n_hex(ab: *mut audit_buffer, buf: *const u8, len: usize);
}

const GFP_KERNEL: i32 = 0;
const ENOMEM: i32 = -12;
const EBADMSG: i32 = -74;
const EINVAL: i32 = -22;

// Helper macro: kzalloc_obj equivalent
// #define kzalloc_obj(obj) kzalloc(sizeof(obj), GFP_KERNEL)
unsafe fn kzalloc_obj() -> *mut digest_info {
    kzalloc(mem::size_of::<digest_info>(), GFP_KERNEL) as *mut digest_info
}

// Helper to check if a pointer is an error code encoded in pointer form
fn is_err(ptr: *const core::ffi::c_void) -> bool {
    (ptr as isize) < 0 && (ptr as isize) > -4096
}

fn is_err_or_null(ptr: *const core::ffi::c_void) -> bool {
    ptr.is_null() || is_err(ptr)
}

fn err_ptr(err: i32) -> *mut core::ffi::c_void {
    err as *mut core::ffi::c_void
}

fn ptr_err(ptr: *const core::ffi::c_void) -> i32 {
    ptr as isize as i32
}

/// ipe_digest_parse() - parse a digest in IPE's policy.
/// @valstr: Supplies the string parsed from the policy.
///
/// Digests in IPE are defined in a standard way:
///	<alg_name>:<hex>
///
/// Use this function to create a property to parse the digest
/// consistently. The parsed digest will be saved in @value in IPE's
/// policy.
///
/// Return: The parsed digest_info structure on success. If an error occurs,
/// the function will return the error value (via ERR_PTR).
#[no_mangle]
pub unsafe extern "C" fn ipe_digest_parse(valstr: *const u8) -> *mut digest_info {
    let mut info: *mut digest_info = ptr::null_mut();
    let mut sep: *mut u8;
    let mut raw_digest: *const u8;
    let mut raw_digest_len: usize;
    let mut digest: *mut u8 = ptr::null_mut();
    let mut alg: *mut u8 = ptr::null_mut();
    let mut rc: i32 = 0;

    info = kzalloc_obj();
    if info.is_null() {
        return err_ptr(ENOMEM) as *mut digest_info;
    }

    sep = strchr(valstr, ':' as i32);
    if sep.is_null() {
        rc = EBADMSG;
        // goto err
    } else {
        alg = kstrndup(valstr, (sep as usize) - (valstr as usize), GFP_KERNEL);
        if alg.is_null() {
            rc = ENOMEM;
            // goto err
        } else {
            raw_digest = (sep as *const u8).add(1);
            raw_digest_len = strlen(raw_digest);

            (*info).digest_len = (raw_digest_len + 1) / 2;
            digest = kzalloc((*info).digest_len, GFP_KERNEL) as *mut u8;
            if digest.is_null() {
                rc = ENOMEM;
                // goto err
            } else {
                rc = hex2bin(digest, raw_digest, (*info).digest_len);
                if rc < 0 {
                    rc = EINVAL;
                    // goto err
                } else {
                    (*info).alg = alg;
                    (*info).digest = digest;
                    return info;
                }
            }
        }
    }

    // err:
    kfree(alg as *const core::ffi::c_void);
    kfree(digest as *const core::ffi::c_void);
    kfree(info as *const core::ffi::c_void);
    err_ptr(rc) as *mut digest_info
}

/// ipe_digest_eval() - evaluate an IPE digest against another digest.
/// @expected: Supplies the policy-provided digest value.
/// @digest: Supplies the digest to compare against the policy digest value.
///
/// Return:
/// * true	- digests match
/// * false	- digests do not match
#[no_mangle]
pub unsafe extern "C" fn ipe_digest_eval(
    expected: *const digest_info,
    digest: *const digest_info,
) -> bool {
    ((*expected).digest_len == (*digest).digest_len)
        && (strcmp((*expected).alg, (*digest).alg) == 0)
        && (memcmp(
            (*expected).digest,
            (*digest).digest,
            (*expected).digest_len,
        ) == 0)
}

/// ipe_digest_free() - free an IPE digest.
/// @info: Supplies a pointer the policy-provided digest to free.
#[no_mangle]
pub unsafe extern "C" fn ipe_digest_free(info: *mut digest_info) {
    if is_err_or_null(info as *const core::ffi::c_void) {
        return;
    }

    kfree((*info).alg as *const core::ffi::c_void);
    kfree((*info).digest as *const core::ffi::c_void);
    kfree(info as *const core::ffi::c_void);
}

/// ipe_digest_audit() - audit a digest that was sourced from IPE's policy.
/// @ab: Supplies the audit_buffer to append the formatted result.
/// @info: Supplies a pointer to source the audit record from.
///
/// Digests in IPE are audited in this format:
///	<alg_name>:<hex>
#[no_mangle]
pub unsafe extern "C" fn ipe_digest_audit(ab: *mut audit_buffer, info: *const digest_info) {
    audit_log_untrustedstring(ab, (*info).alg);
    audit_log_format(ab, b":\0".as_ptr());
    audit_log_n_hex(ab, (*info).digest, (*info).digest_len);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
