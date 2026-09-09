/* SPDX-License-Identifier: GPL-2.0-or-later */
/* RxRPC key type
 *
 * Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Dependencies: <linux/key.h> and <crypto/krb5.h>. */

/* key type for AF_RXRPC keys */
extern "C" {
    pub static mut key_type_rxrpc: key_type;
    pub fn rxrpc_get_null_key(description: *const core::ffi::c_char) -> *mut key;
}

/* RxRPC key for Kerberos IV (type-2 security) */
#[repr(C)]
pub struct rxkad_key {
    pub vice_id: u32,
    pub start: u32,             /* time at which ticket starts */
    pub expiry: u32,            /* time at which ticket expires */
    pub kvno: u32,              /* key version number */
    pub primary_flag: u8,       /* T if key for primary cell for this user */
    pub ticket_len: u16,        /* length of ticket[] */
    pub session_key: [u8; 8],   /* DES session key */
    pub ticket: [u8; 0],        /* the encrypted ticket */
}

/* RxRPC key for YFS-RxGK (type-6 security) */
#[repr(C)]
pub struct rxgk_key {
    pub begintime: i64,         /* Time at which the ticket starts */
    pub endtime: i64,           /* Time at which the ticket ends */
    pub lifetime: u64,          /* Maximum lifespan of a connection (seconds) */
    pub bytelife: u64,          /* Maximum number of bytes on a connection */
    pub enctype: core::ffi::c_uint, /* Encoding type */
    pub level: i8,              /* Negotiated security RXRPC_SECURITY_PLAIN/AUTH/ENCRYPT */
    pub key: krb5_buffer,       /* Master key, K0 */
    pub ticket: krb5_buffer,    /* Ticket to be passed to server */
    pub _key: [u8; 0],          /* Key storage */
}

/* list of tokens attached to an rxrpc key */
#[repr(C)]
pub union rxrpc_key_token_data {
    pub kad: *mut rxkad_key,
    pub rxgk: *mut rxgk_key,
}

#[repr(C)]
pub struct rxrpc_key_token {
    pub security_index: u16,    /* RxRPC header security index */
    pub no_leak_key: bool,      /* Don't copy the key to userspace */
    pub next: *mut rxrpc_key_token, /* the next token in the list */
    pub data: rxrpc_key_token_data,
}

/* structure of raw payloads passed to add_key() or instantiate key */
#[repr(C)]
pub struct rxrpc_key_data_v1 {
    pub security_index: u16,
    pub ticket_length: u16,
    pub expiry: u32,             /* time_t */
    pub kvno: u32,
    pub session_key: [u8; 8],
    pub ticket: [u8; 0],
}

/*
 * AF_RXRPC key payload derived from XDR format
 * - based on openafs-1.4.10/src/auth/afs_token.xg
 */
pub const AFSTOKEN_LENGTH_MAX: u32 = 16384; /* max payload size */
pub const AFSTOKEN_STRING_MAX: u32 = 256;   /* max small string length */
pub const AFSTOKEN_DATA_MAX: u32 = 64;      /* max small data length */
pub const AFSTOKEN_CELL_MAX: u32 = 64;      /* max cellname length */
pub const AFSTOKEN_MAX: u32 = 8;            /* max tokens per payload */
pub const AFSTOKEN_BDATALN_MAX: u32 = 16384; /* max big data length */
pub const AFSTOKEN_RK_TIX_MAX: u32 = 12000; /* max RxKAD ticket size */
pub const AFSTOKEN_GK_KEY_MAX: u32 = 64;    /* max GSSAPI key size */
pub const AFSTOKEN_GK_TOKEN_MAX: u32 = 16384; /* max GSSAPI token size */

/*
 * Truncate a time64_t to the range from 1970 to 2106 as in the network
 * protocol.
 */
#[inline]
pub unsafe fn rxrpc_time64_to_u32(time: time64_t) -> u32 {
    if time < 0 {
        return 0;
    }
    if time > u32::MAX as time64_t {
        return u32::MAX;
    }
    time as u32
}

/*
 * Extend u32 back to time64_t using the same 1970-2106 range.
 */
#[inline]
pub unsafe fn rxrpc_u32_to_time64(time: u32) -> time64_t {
    time as time64_t
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
