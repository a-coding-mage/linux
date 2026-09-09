/* SPDX-License-Identifier: LGPL-2.1 */
/*
 *   SPNEGO upcall management for CIFS
 *
 *   Copyright (c) 2007 Red Hat, Inc.
 *   Author(s): Jeff Layton (jlayton@redhat.com)
 *              Steve French (sfrench@us.ibm.com)
 *
 */

// The C header guard is omitted; Rust item/module inclusion provides the
// corresponding protection.

pub const CIFS_SPNEGO_UPCALL_VERSION: u32 = 2;

/*
 * The version field should always be set to CIFS_SPNEGO_UPCALL_VERSION.
 * The flags field is for future use. The request-key callout should set
 * sesskey_len and secblob_len, and then concatenate the SessKey+SecBlob
 * and stuff it in the data field.
 */
#[repr(C)]
pub struct cifs_spnego_msg {
    pub version: u32,
    pub flags: u32,
    pub sesskey_len: u32,
    pub secblob_len: u32,
    pub data: [u8; 0],
}

extern "C" {
    pub static mut cifs_spnego_key_type: key_type;

    pub fn cifs_get_spnego_key(
        sesInfo: *mut cifs_ses,
        server: *mut TCP_Server_Info,
    ) -> *mut key;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
