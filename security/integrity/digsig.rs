// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2011 Intel Corporation
 *
 * Author:
 * Dmitry Kasatkin <dmitry.kasatkin@intel.com>
 */

// #include <linux/err.h>
// #include <linux/sched.h>
// #include <linux/slab.h>
// #include <linux/cred.h>
// #include <linux/kernel_read_file.h>
// #include <linux/key-type.h>
// #include <linux/digsig.h>
// #include <linux/vmalloc.h>
// #include <crypto/public_key.h>
// #include <keys/system_keyring.h>
//
// #include "integrity.h"

use core::ffi::{c_char, c_int, c_uint};

// External types and constants from kernel and integrity.h
type KeyPermT = u32;
type OffT = i64;
type KeyRefT = *mut core::ffi::c_void;
type SizeT = usize;

// External functions from kernel
extern "C" {
    fn request_key(
        type_: *const core::ffi::c_void,
        description: *const c_char,
        callout_info: *mut c_char,
    ) -> *mut core::ffi::c_void;

    fn keyring_alloc(
        name: *const c_char,
        uid: core::ffi::c_uint,
        gid: core::ffi::c_uint,
        cred: *const core::ffi::c_void,
        perm: KeyPermT,
        flags: c_int,
        restriction: *mut core::ffi::c_void,
        dest_keyring: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;

    fn current_cred() -> *const core::ffi::c_void;

    fn key_create_or_update(
        keyring_ref: KeyRefT,
        type_: *const c_char,
        description: *mut c_char,
        payload: *const core::ffi::c_void,
        plen: SizeT,
        perm: KeyPermT,
        flags: c_int,
    ) -> KeyRefT;

    fn make_key_ref(key: *mut core::ffi::c_void, possession: c_int) -> KeyRefT;

    fn key_ref_to_ptr(key_ref: KeyRefT) -> *mut core::ffi::c_void;

    fn key_ref_put(key_ref: KeyRefT);

    fn kernel_read_file_from_path(
        path: *const c_char,
        offset: core::ffi::c_uint,
        buf: *mut *mut core::ffi::c_void,
        buf_size: core::ffi::c_int,
        file_size: *mut core::ffi::c_int,
        id: c_int,
    ) -> c_int;

    fn vfree(addr: *mut core::ffi::c_void);

    fn digsig_verify(
        keyring: *mut core::ffi::c_void,
        sig: *const c_char,
        siglen: c_int,
        digest: *const c_char,
        digestlen: c_int,
    ) -> c_int;

    fn asymmetric_verify(
        keyring: *mut core::ffi::c_void,
        sig: *const c_char,
        siglen: c_int,
        digest: *const c_char,
        digestlen: c_int,
    ) -> c_int;

    fn asymmetric_verify_v3(
        keyring: *mut core::ffi::c_void,
        sig: *const c_char,
        siglen: c_int,
        digest: *const c_char,
        digestlen: c_int,
        algo: u8,
    ) -> c_int;

    fn ima_modsig_verify(
        keyring: *mut core::ffi::c_void,
        modsig: *const core::ffi::c_void,
    ) -> c_int;

    fn set_platform_trusted_keys(keyring: *mut core::ffi::c_void);

    fn imputed_trust_enabled() -> c_int;

    fn set_machine_trusted_keys(keyring: *mut core::ffi::c_void);

    fn load_module_cert(keyring: *mut core::ffi::c_void);

    fn kzalloc_obj(type_: *const core::ffi::c_void) -> *mut core::ffi::c_void;

    fn kfree(objp: *mut core::ffi::c_void);

    fn pr_err(fmt: *const c_char, ...);

    fn pr_info(fmt: *const c_char, ...);

    fn pr_notice(fmt: *const c_char, ...);
}

// External constants
const INTEGRITY_KEYRING_MAX: usize = 4;

#[cfg(not(feature = "CONFIG_INTEGRITY_TRUSTED_KEYRING"))]
static KEYRING_NAME: &[&[u8]; 4] = &[b"_evm\0", b"_ima\0", b".platform\0", b".machine\0"];

#[cfg(feature = "CONFIG_INTEGRITY_TRUSTED_KEYRING")]
static KEYRING_NAME: &[&[u8]; 4] = &[b".evm\0", b".ima\0", b".platform\0", b".machine\0"];

// Conditional macro for restrict_link_to_ima
// When CONFIG_IMA_KEYRINGS_PERMIT_SIGNED_BY_BUILTIN_OR_SECONDARY is set,
// restrict_link_to_ima = restrict_link_by_digsig_builtin_and_secondary
// Otherwise: restrict_link_to_ima = restrict_link_by_digsig_builtin
// (The actual restriction check function is set in integrity_init_keyring)

static mut KEYRING: [*mut core::ffi::c_void; INTEGRITY_KEYRING_MAX] =
    [core::ptr::null_mut(); INTEGRITY_KEYRING_MAX];

const INTEGRITY_KEYRING_PLATFORM: c_uint = 2;
const INTEGRITY_KEYRING_MACHINE: c_uint = 3;
const INTEGRITY_KEYRING_IMA: c_uint = 1;

const KEY_POS_ALL: KeyPermT = 0x3f000000;
const KEY_POS_SETATTR: KeyPermT = 0x04000000;
const KEY_USR_VIEW: KeyPermT = 0x00010000;
const KEY_USR_READ: KeyPermT = 0x00020000;
const KEY_USR_SEARCH: KeyPermT = 0x00040000;
const KEY_USR_WRITE: KeyPermT = 0x00080000;
const KEY_ALLOC_NOT_IN_QUOTA: c_int = 0x0001;

const EINVAL: c_int = -22;
const ENOMEM: c_int = -12;
const EOPNOTSUPP: c_int = -95;

const INT_MAX: c_int = 0x7fffffff;
const READING_X509_CERTIFICATE: c_int = 5;

fn is_err(ptr: *mut core::ffi::c_void) -> bool {
    unsafe { (ptr as isize) < 0 && (ptr as isize) > -4096 }
}

fn ptr_err(ptr: *mut core::ffi::c_void) -> c_int {
    (ptr as isize) as c_int
}

fn err_ptr(err: c_int) -> *mut core::ffi::c_void {
    err as *mut core::ffi::c_void
}

unsafe fn integrity_keyring_from_id(id: c_uint) -> *mut core::ffi::c_void {
    if id >= INTEGRITY_KEYRING_MAX as c_uint {
        return err_ptr(EINVAL);
    }

    if KEYRING[id as usize].is_null() {
        let keyring_name_bytes = KEYRING_NAME[id as usize];
        let name_ptr = keyring_name_bytes.as_ptr() as *const c_char;

        KEYRING[id as usize] =
            request_key(core::ptr::null(), name_ptr, core::ptr::null_mut());

        if is_err(KEYRING[id as usize]) {
            let err = ptr_err(KEYRING[id as usize]);
            pr_err(
                b"no %s keyring: %d\n\0".as_ptr() as *const c_char,
                name_ptr,
                err,
            );
            KEYRING[id as usize] = core::ptr::null_mut();
            return err_ptr(err);
        }
    }

    KEYRING[id as usize]
}

pub extern "C" fn integrity_digsig_verify(
    id: c_uint,
    sig: *const c_char,
    siglen: c_int,
    digest: *const c_char,
    digestlen: c_int,
    algo: u8,
) -> c_int {
    let keyring: *mut core::ffi::c_void;

    if siglen < 2 {
        return EINVAL;
    }

    unsafe {
        keyring = integrity_keyring_from_id(id);
        if is_err(keyring) {
            return ptr_err(keyring);
        }

        match *sig.add(1) {
            1 => {
                // v1 API expect signature without xattr type
                return digsig_verify(
                    keyring,
                    sig.add(1),
                    siglen - 1,
                    digest,
                    digestlen,
                );
            }
            2 => {
                // regular file data hash based signature
                return asymmetric_verify(keyring, sig, siglen, digest, digestlen);
            }
            3 => {
                // struct ima_file_id data based signature
                return asymmetric_verify_v3(keyring, sig, siglen, digest, digestlen, algo);
            }
            _ => {}
        }
    }

    EOPNOTSUPP
}

pub extern "C" fn integrity_modsig_verify(
    id: c_uint,
    modsig: *const core::ffi::c_void,
) -> c_int {
    let keyring: *mut core::ffi::c_void;

    unsafe {
        keyring = integrity_keyring_from_id(id);
        if is_err(keyring) {
            return ptr_err(keyring);
        }

        return ima_modsig_verify(keyring, modsig);
    }
}

#[inline(never)]
unsafe fn __integrity_init_keyring(
    id: c_uint,
    perm: KeyPermT,
    restriction: *mut core::ffi::c_void,
) -> c_int {
    let cred = current_cred();
    let mut err: c_int = 0;

    let keyring_name_bytes = KEYRING_NAME[id as usize];
    let name_ptr = keyring_name_bytes.as_ptr() as *const c_char;

    KEYRING[id as usize] = keyring_alloc(
        name_ptr,
        0,
        0,
        cred,
        perm,
        KEY_ALLOC_NOT_IN_QUOTA,
        restriction,
        core::ptr::null_mut(),
    );

    if is_err(KEYRING[id as usize]) {
        err = ptr_err(KEYRING[id as usize]);
        pr_info(
            b"Can't allocate %s keyring (%d)\n\0".as_ptr() as *const c_char,
            name_ptr,
            err,
        );
        KEYRING[id as usize] = core::ptr::null_mut();
    } else {
        if id == INTEGRITY_KEYRING_PLATFORM {
            set_platform_trusted_keys(KEYRING[id as usize]);
        }
        if id == INTEGRITY_KEYRING_MACHINE && imputed_trust_enabled() != 0 {
            set_machine_trusted_keys(KEYRING[id as usize]);
        }
        if id == INTEGRITY_KEYRING_IMA {
            load_module_cert(KEYRING[id as usize]);
        }
    }

    err
}

#[inline(never)]
pub extern "C" fn integrity_init_keyring(id: c_uint) -> c_int {
    let mut restriction: *mut core::ffi::c_void;
    let mut perm: KeyPermT;
    let ret: c_int;

    perm = (KEY_POS_ALL & !KEY_POS_SETATTR) | KEY_USR_VIEW | KEY_USR_READ | KEY_USR_SEARCH;

    if id == INTEGRITY_KEYRING_PLATFORM
        || (id == INTEGRITY_KEYRING_MACHINE
            && cfg!(not(feature = "CONFIG_INTEGRITY_CA_MACHINE_KEYRING")))
    {
        restriction = core::ptr::null_mut();
        unsafe {
            return __integrity_init_keyring(id, perm, restriction);
        }
    }

    #[cfg(not(feature = "CONFIG_INTEGRITY_TRUSTED_KEYRING"))]
    {
        return 0;
    }

    #[cfg(feature = "CONFIG_INTEGRITY_TRUSTED_KEYRING")]
    unsafe {
        restriction = kzalloc_obj(core::ptr::null());
        if restriction.is_null() {
            return ENOMEM;
        }

        if id == INTEGRITY_KEYRING_MACHINE {
            // restriction->check = restrict_link_by_ca
            // Set restriction check function for MACHINE keyring
            let restriction_ptr = restriction as *mut KeyRestriction;
            (*restriction_ptr).check = restrict_link_by_ca as *mut core::ffi::c_void;
        } else {
            // restriction->check = restrict_link_to_ima
            let restriction_ptr = restriction as *mut KeyRestriction;
            #[cfg(feature = "CONFIG_IMA_KEYRINGS_PERMIT_SIGNED_BY_BUILTIN_OR_SECONDARY")]
            {
                (*restriction_ptr).check = restrict_link_by_digsig_builtin_and_secondary
                    as *mut core::ffi::c_void;
            }
            #[cfg(not(feature = "CONFIG_IMA_KEYRINGS_PERMIT_SIGNED_BY_BUILTIN_OR_SECONDARY"))]
            {
                (*restriction_ptr).check = restrict_link_by_digsig_builtin as *mut core::ffi::c_void;
            }
        }

        if id != INTEGRITY_KEYRING_MACHINE {
            perm |= KEY_USR_WRITE;
        }

        ret = __integrity_init_keyring(id, perm, restriction);
        if ret != 0 {
            kfree(restriction);
        }
        return ret;
    }
}

#[inline(never)]
unsafe fn integrity_add_key(
    id: c_uint,
    data: *const core::ffi::c_void,
    size: OffT,
    perm: KeyPermT,
) -> c_int {
    let key: KeyRefT;
    let mut rc: c_int = 0;

    if KEYRING[id as usize].is_null() {
        return EINVAL;
    }

    key = key_create_or_update(
        make_key_ref(KEYRING[id as usize], 1),
        b"asymmetric\0".as_ptr() as *const c_char,
        core::ptr::null_mut(),
        data,
        size as SizeT,
        perm,
        KEY_ALLOC_NOT_IN_QUOTA,
    );

    if is_err(key) {
        rc = ptr_err(key);
        if id != INTEGRITY_KEYRING_MACHINE {
            pr_err(
                b"Problem loading X.509 certificate %d\n\0".as_ptr() as *const c_char,
                rc,
            );
        }
    } else {
        let key_ptr = key_ref_to_ptr(key);
        let desc_ptr = (key_ptr as *mut KeyStruct)
            .as_ref()
            .map(|k| k.description)
            .unwrap_or(core::ptr::null());
        pr_notice(
            b"Loaded X.509 cert '%s'\n\0".as_ptr() as *const c_char,
            desc_ptr,
        );
        key_ref_put(key);
    }

    rc
}

pub extern "C" fn integrity_load_x509(id: c_uint, path: *const c_char) -> c_int {
    let mut data: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut size: SizeT = 0;
    let mut rc: c_int;
    let perm: KeyPermT;

    unsafe {
        rc = kernel_read_file_from_path(
            path,
            0,
            &mut data,
            INT_MAX,
            core::ptr::null_mut(),
            READING_X509_CERTIFICATE,
        );
        if rc < 0 {
            pr_err(
                b"Unable to open file: %s (%d)\0".as_ptr() as *const c_char,
                path,
                rc,
            );
            return rc;
        }
        size = rc as SizeT;

        perm = (KEY_POS_ALL & !KEY_POS_SETATTR) | KEY_USR_VIEW | KEY_USR_READ;

        pr_info(
            b"Loading X.509 certificate: %s\n\0".as_ptr() as *const c_char,
            path,
        );
        rc = integrity_add_key(id, data, size as OffT, perm);

        vfree(data);
        return rc;
    }
}

pub extern "C" fn integrity_load_cert(
    id: c_uint,
    source: *const c_char,
    data: *const core::ffi::c_void,
    len: SizeT,
    perm: KeyPermT,
) -> c_int {
    if data.is_null() {
        return EINVAL;
    }

    unsafe {
        pr_info(
            b"Loading X.509 certificate: %s\n\0".as_ptr() as *const c_char,
            source,
        );
        return integrity_add_key(id, data, len as OffT, perm);
    }
}

// Stub types for key-related structures
#[repr(C)]
struct KeyStruct {
    description: *const c_char,
    // Other fields omitted as they are not used in this file
}

#[repr(C)]
struct KeyRestriction {
    check: *mut core::ffi::c_void,
    // Other fields omitted as they are not used in this file
}

// External function stubs for restriction checks
extern "C" {
    fn restrict_link_by_ca() -> c_int;
    fn restrict_link_by_digsig_builtin() -> c_int;
    fn restrict_link_by_digsig_builtin_and_secondary() -> c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
