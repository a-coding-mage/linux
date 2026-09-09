/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Signature verification
 *
 * Copyright (C) 2014 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Dependencies: linux/errno.h and linux/types.h. */

/*
 * Indicate that both builtin trusted keys and secondary trusted keys
 * should be used.
 */
pub const VERIFY_USE_SECONDARY_KEYRING: *mut key = 1usize as *mut key;
pub const VERIFY_USE_PLATFORM_KEYRING: *mut key = 2usize as *mut key;

pub unsafe fn system_keyring_id_check(id: u64) -> i32 {
    if id > VERIFY_USE_PLATFORM_KEYRING as usize as u64 {
        return -EINVAL;
    }

    0
}

/*
 * The use to which an asymmetric key is being put.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum key_being_used_for {
    VERIFYING_MODULE_SIGNATURE,
    VERIFYING_FIRMWARE_SIGNATURE,
    VERIFYING_KEXEC_PE_SIGNATURE,
    VERIFYING_KEY_SIGNATURE,
    VERIFYING_KEY_SELF_SIGNATURE,
    VERIFYING_UNSPECIFIED_SIGNATURE,
    VERIFYING_BPF_SIGNATURE,
    NR__KEY_BEING_USED_FOR,
}

pub enum key {}
pub enum pkcs7_message {}

#[cfg(CONFIG_SYSTEM_DATA_VERIFICATION)]
extern "C" {
    pub fn verify_pkcs7_signature(
        data: *const core::ffi::c_void,
        len: usize,
        raw_pkcs7: *const core::ffi::c_void,
        pkcs7_len: usize,
        trusted_keys: *mut key,
        usage: key_being_used_for,
        view_content: Option<unsafe extern "C" fn(
            ctx: *mut core::ffi::c_void,
            data: *const core::ffi::c_void,
            len: usize,
            asn1hdrlen: usize,
        ) -> i32>,
        ctx: *mut core::ffi::c_void,
    ) -> i32;

    pub fn verify_pkcs7_message_sig(
        data: *const core::ffi::c_void,
        len: usize,
        pkcs7: *mut pkcs7_message,
        trusted_keys: *mut key,
        usage: key_being_used_for,
        view_content: Option<unsafe extern "C" fn(
            ctx: *mut core::ffi::c_void,
            data: *const core::ffi::c_void,
            len: usize,
            asn1hdrlen: usize,
        ) -> i32>,
        ctx: *mut core::ffi::c_void,
    ) -> i32;

    #[cfg(CONFIG_SIGNED_PE_FILE_VERIFICATION)]
    pub fn verify_pefile_signature(
        pebuf: *const core::ffi::c_void,
        pelen: u32,
        trusted_keys: *mut key,
        usage: key_being_used_for,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
