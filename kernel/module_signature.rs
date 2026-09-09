// SPDX-License-Identifier: GPL-2.0+
/*
 * Module signature checker
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

use core::ffi::{c_char, c_int};

// Supplied by the corresponding kernel dependencies.
unsafe extern "C" {
    fn pr_err(fmt: *const c_char, ...);
}

/**
 * mod_check_sig - check that the given signature is sane
 *
 * @ms:        Signature to check.
 * @file_len:  Size of the file to which @ms is appended.
 * @name:      What is being checked. Used for error messages.
 */
pub unsafe fn mod_check_sig(
    ms: *const crate::module_signature::module_signature,
    file_len: usize,
    name: *const c_char,
) -> c_int {
    if u32::from_be((*ms).sig_len) as usize >= file_len - core::mem::size_of::<crate::module_signature::module_signature>() {
        return -crate::errno::EBADMSG;
    }

    if (*ms).id_type != crate::module_signature::MODULE_SIGNATURE_TYPE_PKCS7 {
        pr_err(b"%s: not signed with expected PKCS#7 message\n\0".as_ptr() as *const c_char, name);
        return -crate::errno::ENOPKG;
    }

    if (*ms).algo != 0
        || (*ms).hash != 0
        || (*ms).signer_len != 0
        || (*ms).key_id_len != 0
        || (*ms).__pad[0] != 0
        || (*ms).__pad[1] != 0
        || (*ms).__pad[2] != 0
    {
        pr_err(
            b"%s: PKCS#7 signature info has unexpected non-zero params\n\0".as_ptr()
                as *const c_char,
            name,
        );
        return -crate::errno::EBADMSG;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
