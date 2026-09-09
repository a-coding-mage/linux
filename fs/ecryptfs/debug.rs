// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * eCryptfs: Linux filesystem encryption layer
 * Functions only useful for debugging.
 *
 * Copyright (C) 2006 International Business Machines Corp.
 *   Author(s): Michael A. Halcrow <mahalcro@us.ibm.com>
 */

// Linux kernel headers and ecryptfs_kernel.h are supplied by other translation units.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut ecryptfs_verbosity: c_int;

    fn ecryptfs_printk(level: c_int, fmt: *const c_char, ...);
    fn ecryptfs_to_hex(dst: *mut c_char, src: *const u8, bytes: c_int);
    fn ecryptfs_dump_hex(data: *mut c_char, bytes: c_int);
    fn strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn print_hex_dump(
        level: c_int,
        prefix_str: *const c_char,
        prefix_type: c_int,
        rowsize: c_int,
        groupsize: c_int,
        buf: *const c_void,
        len: usize,
        ascii: bool,
    );
}

// These constants and the structure definition are supplied by ecryptfs_kernel.h.
use crate::{
    ecryptfs_auth_tok, ECRYPTFS_CONTAINS_DECRYPTED_KEY, ECRYPTFS_CONTAINS_ENCRYPTED_KEY,
    ECRYPTFS_DEFAULT_KEY_BYTES, ECRYPTFS_PRIVATE_KEY, ECRYPTFS_PERSISTENT_PASSWORD,
    ECRYPTFS_SALT_SIZE, ECRYPTFS_SIG_SIZE_HEX, ECRYPTFS_USERSPACE_SHOULD_TRY_TO_DECRYPT,
    ECRYPTFS_USERSPACE_SHOULD_TRY_TO_ENCRYPT, DUMP_PREFIX_OFFSET, KERN_DEBUG,
};

/*
 * ecryptfs_dump_auth_tok - debug function to print auth toks
 *
 * This function will print the contents of an ecryptfs authentication
 * token.
 */
pub unsafe extern "C" fn ecryptfs_dump_auth_tok(auth_tok: *mut ecryptfs_auth_tok) {
    let mut salt = [0 as c_char; ECRYPTFS_SALT_SIZE * 2 + 1];
    let mut sig = [0 as c_char; ECRYPTFS_SIG_SIZE_HEX + 1];

    ecryptfs_printk(KERN_DEBUG, b"Auth tok at mem loc [%p]:\n\0".as_ptr() as *const c_char, auth_tok);
    if (*auth_tok).flags & ECRYPTFS_PRIVATE_KEY != 0 {
        ecryptfs_printk(KERN_DEBUG, b" * private key type\n\0".as_ptr() as *const c_char);
    } else {
        ecryptfs_printk(KERN_DEBUG, b" * passphrase type\n\0".as_ptr() as *const c_char);
        ecryptfs_to_hex(
            salt.as_mut_ptr(),
            (*auth_tok).token.password.salt.as_ptr(),
            ECRYPTFS_SALT_SIZE as c_int,
        );
        ecryptfs_printk(KERN_DEBUG, b" * salt = [%s]\n\0".as_ptr() as *const c_char, salt.as_ptr());
        if (*auth_tok).token.password.flags & ECRYPTFS_PERSISTENT_PASSWORD != 0 {
            ecryptfs_printk(KERN_DEBUG, b" * persistent\n\0".as_ptr() as *const c_char);
        }
        strscpy(sig.as_mut_ptr(), (*auth_tok).token.password.signature.as_ptr(), sig.len());
        ecryptfs_printk(KERN_DEBUG, b" * signature = [%s]\n\0".as_ptr() as *const c_char, sig.as_ptr());
    }
    ecryptfs_printk(KERN_DEBUG, b" * session_key.flags = [0x%x]\n\0".as_ptr() as *const c_char, (*auth_tok).session_key.flags);
    if (*auth_tok).session_key.flags & ECRYPTFS_USERSPACE_SHOULD_TRY_TO_DECRYPT != 0 {
        ecryptfs_printk(KERN_DEBUG, b" * Userspace decrypt request set\n\0".as_ptr() as *const c_char);
    }
    if (*auth_tok).session_key.flags & ECRYPTFS_USERSPACE_SHOULD_TRY_TO_ENCRYPT != 0 {
        ecryptfs_printk(KERN_DEBUG, b" * Userspace encrypt request set\n\0".as_ptr() as *const c_char);
    }
    if (*auth_tok).session_key.flags & ECRYPTFS_CONTAINS_DECRYPTED_KEY != 0 {
        ecryptfs_printk(KERN_DEBUG, b" * Contains decrypted key\n\0".as_ptr() as *const c_char);
        ecryptfs_printk(KERN_DEBUG, b" * session_key.decrypted_key_size = [0x%x]\n\0".as_ptr() as *const c_char, (*auth_tok).session_key.decrypted_key_size);
        ecryptfs_printk(KERN_DEBUG, b" * Decrypted session key dump:\n\0".as_ptr() as *const c_char);
        if ecryptfs_verbosity > 0 { ecryptfs_dump_hex((*auth_tok).session_key.decrypted_key, ECRYPTFS_DEFAULT_KEY_BYTES as c_int); }
    }
    if (*auth_tok).session_key.flags & ECRYPTFS_CONTAINS_ENCRYPTED_KEY != 0 {
        ecryptfs_printk(KERN_DEBUG, b" * Contains encrypted key\n\0".as_ptr() as *const c_char);
        ecryptfs_printk(KERN_DEBUG, b" * session_key.encrypted_key_size = [0x%x]\n\0".as_ptr() as *const c_char, (*auth_tok).session_key.encrypted_key_size);
        ecryptfs_printk(KERN_DEBUG, b" * Encrypted session key dump:\n\0".as_ptr() as *const c_char);
        if ecryptfs_verbosity > 0 { ecryptfs_dump_hex((*auth_tok).session_key.encrypted_key, (*auth_tok).session_key.encrypted_key_size as c_int); }
    }
}

/** Dump hexadecimal representation of char array */
pub unsafe extern "C" fn ecryptfs_dump_hex(data: *mut c_char, bytes: c_int) {
    if ecryptfs_verbosity < 1 { return; }
    print_hex_dump(KERN_DEBUG, b"ecryptfs: \0".as_ptr() as *const c_char, DUMP_PREFIX_OFFSET, 16, 1, data as *const c_void, bytes as usize, false);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
