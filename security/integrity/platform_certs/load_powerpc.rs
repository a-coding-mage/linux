// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019 IBM Corporation
 * Author: Nayna Jain
 *
 *      - loads keys and hashes stored and controlled by the firmware.
 */

// Linux kernel external dependencies (not included via use statements in kernel code)
extern "C" {
    static mut secvar_ops: *mut SecvarOps;

    fn kmalloc(size: u64, flags: i32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn pr_info(fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
    fn strcmp(s1: *const u8, s2: *const u8) -> i32;
    fn parse_efi_signature_list(
        name: *const u8,
        esl: *mut core::ffi::c_void,
        size: u64,
        get_handler: extern "C" fn() -> *mut core::ffi::c_void,
    ) -> i32;

    fn get_handler_for_db() -> *mut core::ffi::c_void;
    fn get_handler_for_dbx() -> *mut core::ffi::c_void;
    fn get_handler_for_ca_keys() -> *mut core::ffi::c_void;
    fn get_handler_for_code_signing_keys() -> *mut core::ffi::c_void;
}

#[repr(C)]
pub struct SecvarOps {
    pub get: unsafe extern "C" fn(*const u8, u64, *mut core::ffi::c_void, *mut u64) -> i32,
    pub format: unsafe extern "C" fn(*mut u8, u64) -> i64,
}

const GFP_KERNEL: i32 = 0xd0;
const ENOENT: i32 = -2;
const ENOMEM: i32 = -12;
const ENODEV: i32 = -19;

// Helper macro to extract ESL from data buffer with offset
// Translates: do { db = data + offset; size = size - offset; } while (0)
macro_rules! extract_esl {
    ($db:expr, $data:expr, $size:expr, $offset:expr) => {
        $db = ($data as *const u8).add($offset as usize) as *mut core::ffi::c_void;
        $size = $size - $offset;
    };
}

// Helper to check if a pointer encodes an error (Linux kernel pattern)
#[inline]
fn is_err(ptr: *const core::ffi::c_void) -> bool {
    (ptr as usize) > (-4096isize as usize)
}

// Helper to extract error code from error-encoded pointer
#[inline]
fn ptr_err(ptr: *const core::ffi::c_void) -> i32 {
    -(ptr as i32)
}

// Helper to encode error code as pointer
#[inline]
fn err_ptr(error: i32) -> *mut core::ffi::c_void {
    (error as usize) as *mut core::ffi::c_void
}

/*
 * Get a certificate list blob from the named secure variable.
 *
 * Returns:
 *  - a pointer to a kmalloc'd buffer containing the cert list on success
 *  - NULL if the key does not exist
 *  - an ERR_PTR on error
 */
unsafe extern "C" fn get_cert_list(
    key: *const u8,
    keylen: u64,
    size: *mut u64,
) -> *mut core::ffi::c_void {
    let mut rc: i32;
    let mut db: *mut core::ffi::c_void;

    rc = ((*secvar_ops).get)(key, keylen, core::ptr::null_mut(), size);
    if rc != 0 {
        if rc == ENOENT {
            return core::ptr::null_mut();
        }
        return err_ptr(rc);
    }

    db = kmalloc(*size, GFP_KERNEL);
    if db.is_null() {
        return err_ptr(ENOMEM);
    }

    rc = ((*secvar_ops).get)(key, keylen, db, size);
    if rc != 0 {
        kfree(db);
        return err_ptr(rc);
    }

    db
}

/*
 * Load the certs contained in the keys databases into the platform trusted
 * keyring and the blacklisted X.509 cert SHA256 hashes into the blacklist
 * keyring.
 */
unsafe extern "C" fn load_powerpc_certs() -> i32 {
    let mut db: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut dbx: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut data: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut trustedca: *mut core::ffi::c_void;
    let mut moduledb: *mut core::ffi::c_void;
    let mut dsize: u64 = 0;
    let mut offset: u64 = 0;
    let mut rc: i32 = 0;
    let mut len: i64;
    let mut buf: [u8; 32] = [0; 32];

    if secvar_ops.is_null() {
        return ENODEV;
    }

    len = ((*secvar_ops).format)(buf.as_mut_ptr(), 32);
    if len <= 0 {
        return ENODEV;
    }

    // Check for known secure boot implementations from OPAL or PLPKS
    if strcmp(buf.as_ptr(), b"ibm,edk2-compat-v1\0".as_ptr()) != 0
        && strcmp(buf.as_ptr(), b"ibm,plpks-sb-v1\0".as_ptr()) != 0
        && strcmp(buf.as_ptr(), b"ibm,plpks-sb-v0\0".as_ptr()) != 0
    {
        pr_err(
            b"Unsupported secvar implementation \"%s\", not loading certs\n\0".as_ptr(),
            buf.as_ptr(),
        );
        return ENODEV;
    }

    if strcmp(buf.as_ptr(), b"ibm,plpks-sb-v1\0".as_ptr()) == 0
        || strcmp(buf.as_ptr(), b"ibm,plpks-sb-v0\0".as_ptr()) == 0
    {
        /* PLPKS authenticated variables ESL data is prefixed with 8 bytes of timestamp */
        offset = 8;
    }

    /*
     * Get db, and dbx. They might not exist, so it isn't an error if we
     * can't get them.
     */
    data = get_cert_list(b"db\0".as_ptr(), 3, &mut dsize);
    if data.is_null() {
        pr_info(b"Couldn't get db list from firmware\n\0".as_ptr());
    } else if is_err(data) {
        rc = ptr_err(data);
        pr_err(b"Error reading db from firmware: %d\n\0".as_ptr(), rc);
        return rc;
    } else {
        extract_esl!(db, data, dsize, offset);

        rc = parse_efi_signature_list(
            b"powerpc:db\0".as_ptr(),
            db,
            dsize,
            get_handler_for_db,
        );
        if rc != 0 {
            pr_err(b"Couldn't parse db signatures: %d\n\0".as_ptr(), rc);
        }
        kfree(data);
    }

    data = get_cert_list(b"dbx\0".as_ptr(), 4, &mut dsize);
    if data.is_null() {
        pr_info(b"Couldn't get dbx list from firmware\n\0".as_ptr());
    } else if is_err(data) {
        rc = ptr_err(data);
        pr_err(b"Error reading dbx from firmware: %d\n\0".as_ptr(), rc);
        return rc;
    } else {
        extract_esl!(dbx, data, dsize, offset);

        rc = parse_efi_signature_list(
            b"powerpc:dbx\0".as_ptr(),
            dbx,
            dsize,
            get_handler_for_dbx,
        );
        if rc != 0 {
            pr_err(b"Couldn't parse dbx signatures: %d\n\0".as_ptr(), rc);
        }
        kfree(data);
    }

    data = get_cert_list(b"trustedcadb\0".as_ptr(), 12, &mut dsize);
    if data.is_null() {
        pr_info(b"Couldn't get trustedcadb list from firmware\n\0".as_ptr());
    } else if is_err(data) {
        rc = ptr_err(data);
        pr_err(b"Error reading trustedcadb from firmware: %d\n\0".as_ptr(), rc);
    } else {
        extract_esl!(trustedca, data, dsize, offset);

        rc = parse_efi_signature_list(
            b"powerpc:trustedca\0".as_ptr(),
            trustedca,
            dsize,
            get_handler_for_ca_keys,
        );
        if rc != 0 {
            pr_err(
                b"Couldn't parse trustedcadb signatures: %d\n\0".as_ptr(),
                rc,
            );
        }
        kfree(data);
    }

    data = get_cert_list(b"moduledb\0".as_ptr(), 9, &mut dsize);
    if data.is_null() {
        pr_info(b"Couldn't get moduledb list from firmware\n\0".as_ptr());
    } else if is_err(data) {
        rc = ptr_err(data);
        pr_err(b"Error reading moduledb from firmware: %d\n\0".as_ptr(), rc);
    } else {
        extract_esl!(moduledb, data, dsize, offset);

        rc = parse_efi_signature_list(
            b"powerpc:moduledb\0".as_ptr(),
            moduledb,
            dsize,
            get_handler_for_code_signing_keys,
        );
        if rc != 0 {
            pr_err(
                b"Couldn't parse moduledb signatures: %d\n\0".as_ptr(),
                rc,
            );
        }
        kfree(data);
    }

    rc
}

// late_initcall(load_powerpc_certs) in original C
// In Rust kernel modules, initialization is typically handled via module macros


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
