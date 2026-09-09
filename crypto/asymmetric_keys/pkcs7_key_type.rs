// SPDX-License-Identifier: GPL-2.0-or-later
/* Testing module to load key from trusted PKCS#7 message
 *
 * Copyright (C) 2014 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Linux kernel headers and module macros provide the declarations used below.

const MODULE_LICENSE: &str = "GPL";
const MODULE_DESCRIPTION: &str = "PKCS#7 testing key type";
const MODULE_AUTHOR: &str = "Red Hat, Inc.";

static mut pkcs7_usage: u32 = 0;
// module_param_named(usage, pkcs7_usage, uint, S_IWUSR | S_IRUGO);
// MODULE_PARM_DESC(pkcs7_usage,
//                  "Usage to specify when verifying the PKCS#7 message");

extern "C" {
    fn user_preparse(prep: *mut key_preparsed_payload) -> i32;
    fn verify_pkcs7_signature(
        data: *const core::ffi::c_void,
        len: usize,
        raw_pkcs7: *const core::ffi::c_void,
        raw_pkcs7_len: usize,
        trusted_keys: u32,
        usage: key_being_used_for,
        view_content: unsafe extern "C" fn(
            ctx: *mut core::ffi::c_void,
            data: *const core::ffi::c_void,
            len: usize,
            asn1hdrlen: usize,
        ) -> i32,
        ctx: *mut core::ffi::c_void,
    ) -> i32;
    fn user_free_preparse(prep: *mut key_preparsed_payload);
    fn generic_key_instantiate(
        key: *mut key,
        prep: *mut key_preparsed_payload,
    ) -> i32;
    fn user_revoke(key: *mut key);
    fn user_destroy(key: *mut key);
    fn user_describe(seq: *mut seq_file, key: *const key);
    fn user_read(
        key: *const key,
        buffer: *mut u8,
        buflen: usize,
    ) -> isize;
    fn register_key_type(ktype: *mut key_type) -> i32;
    fn unregister_key_type(ktype: *mut key_type);
    fn pr_err(fmt: *const u8, ...);
}

#[repr(C)]
pub struct key_preparsed_payload {
    pub data: *const core::ffi::c_void,
    pub datalen: usize,
}

#[repr(C)]
pub struct key;
#[repr(C)]
pub struct seq_file;

#[repr(C)]
pub struct key_type {
    pub name: *const u8,
    pub preparse: Option<unsafe extern "C" fn(*mut key_preparsed_payload) -> i32>,
    pub free_preparse: Option<unsafe extern "C" fn(*mut key_preparsed_payload)>,
    pub instantiate: Option<unsafe extern "C" fn(*mut key, *mut key_preparsed_payload) -> i32>,
    pub revoke: Option<unsafe extern "C" fn(*mut key)>,
    pub destroy: Option<unsafe extern "C" fn(*mut key)>,
    pub describe: Option<unsafe extern "C" fn(*mut seq_file, *const key)>,
    pub read: Option<unsafe extern "C" fn(*const key, *mut u8, usize) -> isize>,
}

type key_being_used_for = u32;
const NR__KEY_BEING_USED_FOR: key_being_used_for = 0;
const VERIFY_USE_SECONDARY_KEYRING: u32 = 0;
const EINVAL: i32 = 22;

/* Retrieve the PKCS-7 message content. */
unsafe extern "C" fn pkcs7_view_content(
    ctx: *mut core::ffi::c_void,
    data: *const core::ffi::c_void,
    len: usize,
    _asn1hdrlen: usize,
) -> i32 {
    let prep = ctx as *mut key_preparsed_payload;
    let saved_prep_data = (*prep).data;
    let saved_prep_datalen = (*prep).datalen;
    (*prep).data = data;
    (*prep).datalen = len;

    let ret = user_preparse(prep);

    (*prep).data = saved_prep_data;
    (*prep).datalen = saved_prep_datalen;
    ret
}

/* Preparse a PKCS-7 wrapped and validated data blob. */
unsafe extern "C" fn pkcs7_preparse(prep: *mut key_preparsed_payload) -> i32 {
    let usage: key_being_used_for = pkcs7_usage;

    if usage >= NR__KEY_BEING_USED_FOR {
        // pr_err("PKCS7key: Invalid usage type %d\n", usage);
        return -EINVAL;
    }

    verify_pkcs7_signature(
        core::ptr::null(),
        0,
        (*prep).data,
        (*prep).datalen,
        VERIFY_USE_SECONDARY_KEYRING,
        usage,
        pkcs7_view_content,
        prep as *mut core::ffi::c_void,
    )
}

/* user defined keys take an arbitrary string as the description and an
 * arbitrary blob of data as the payload
 */
static mut key_type_pkcs7: key_type = key_type {
    name: b"pkcs7_test\0".as_ptr(),
    preparse: Some(pkcs7_preparse),
    free_preparse: Some(user_free_preparse),
    instantiate: Some(generic_key_instantiate),
    revoke: Some(user_revoke),
    destroy: Some(user_destroy),
    describe: Some(user_describe),
    read: Some(user_read),
};

/* Module stuff */
unsafe extern "C" fn pkcs7_key_init() -> i32 {
    register_key_type(&mut key_type_pkcs7)
}

unsafe extern "C" fn pkcs7_key_cleanup() {
    unregister_key_type(&mut key_type_pkcs7);
}

// module_init(pkcs7_key_init);
// module_exit(pkcs7_key_cleanup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
