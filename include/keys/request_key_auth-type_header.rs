/* SPDX-License-Identifier: GPL-2.0-or-later */
/* request_key authorisation token key type
 *
 * Copyright (C) 2005 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Dependencies supplied by the surrounding kernel translation. */

/*
 * Authorisation record for request_key().
 */
#[repr(C)]
pub struct request_key_auth {
    pub rcu: rcu_head,
    pub usage: refcount_t,
    pub target_key: *mut key,
    pub dest_keyring: *mut key,
    pub cred: *const cred,
    pub callout_info: *mut core::ffi::c_void,
    pub callout_len: usize,
    pub pid: pid_t,
    pub op: [core::ffi::c_char; 8],
}

pub unsafe fn get_request_key_auth(key: *const key) -> *mut request_key_auth {
    (*key).payload.data[0] as *mut request_key_auth
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
