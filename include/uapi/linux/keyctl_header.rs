/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* keyctl.h: keyctl command IDs
 *
 * Copyright (C) 2004, 2008 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version
 * 2 of the License, or (at your option) any later version.
 */

use core::ffi::c_char;

/* special process keyring shortcut IDs */
pub const KEY_SPEC_THREAD_KEYRING: i32 = -1; /* - key ID for thread-specific keyring */
pub const KEY_SPEC_PROCESS_KEYRING: i32 = -2; /* - key ID for process-specific keyring */
pub const KEY_SPEC_SESSION_KEYRING: i32 = -3; /* - key ID for session-specific keyring */
pub const KEY_SPEC_USER_KEYRING: i32 = -4; /* - key ID for UID-specific keyring */
pub const KEY_SPEC_USER_SESSION_KEYRING: i32 = -5; /* - key ID for UID-session keyring */
pub const KEY_SPEC_GROUP_KEYRING: i32 = -6; /* - key ID for GID-specific keyring */
pub const KEY_SPEC_REQKEY_AUTH_KEY: i32 = -7; /* - key ID for assumed request_key auth key */
pub const KEY_SPEC_REQUESTOR_KEYRING: i32 = -8; /* - key ID for request_key() dest keyring */

/* request-key default keyrings */
pub const KEY_REQKEY_DEFL_NO_CHANGE: i32 = -1;
pub const KEY_REQKEY_DEFL_DEFAULT: i32 = 0;
pub const KEY_REQKEY_DEFL_THREAD_KEYRING: i32 = 1;
pub const KEY_REQKEY_DEFL_PROCESS_KEYRING: i32 = 2;
pub const KEY_REQKEY_DEFL_SESSION_KEYRING: i32 = 3;
pub const KEY_REQKEY_DEFL_USER_KEYRING: i32 = 4;
pub const KEY_REQKEY_DEFL_USER_SESSION_KEYRING: i32 = 5;
pub const KEY_REQKEY_DEFL_GROUP_KEYRING: i32 = 6;
pub const KEY_REQKEY_DEFL_REQUESTOR_KEYRING: i32 = 7;

/* keyctl commands */
pub const KEYCTL_GET_KEYRING_ID: i32 = 0; /* ask for a keyring's ID */
pub const KEYCTL_JOIN_SESSION_KEYRING: i32 = 1; /* join or start named session keyring */
pub const KEYCTL_UPDATE: i32 = 2; /* update a key */
pub const KEYCTL_REVOKE: i32 = 3; /* revoke a key */
pub const KEYCTL_CHOWN: i32 = 4; /* set ownership of a key */
pub const KEYCTL_SETPERM: i32 = 5; /* set perms on a key */
pub const KEYCTL_DESCRIBE: i32 = 6; /* describe a key */
pub const KEYCTL_CLEAR: i32 = 7; /* clear contents of a keyring */
pub const KEYCTL_LINK: i32 = 8; /* link a key into a keyring */
pub const KEYCTL_UNLINK: i32 = 9; /* unlink a key from a keyring */
pub const KEYCTL_SEARCH: i32 = 10; /* search for a key in a keyring */
pub const KEYCTL_READ: i32 = 11; /* read a key or keyring's contents */
pub const KEYCTL_INSTANTIATE: i32 = 12; /* instantiate a partially constructed key */
pub const KEYCTL_NEGATE: i32 = 13; /* negate a partially constructed key */
pub const KEYCTL_SET_REQKEY_KEYRING: i32 = 14; /* set default request-key keyring */
pub const KEYCTL_SET_TIMEOUT: i32 = 15; /* set key timeout */
pub const KEYCTL_ASSUME_AUTHORITY: i32 = 16; /* assume request_key() authorisation */
pub const KEYCTL_GET_SECURITY: i32 = 17; /* get key security label */
pub const KEYCTL_SESSION_TO_PARENT: i32 = 18; /* apply session keyring to parent process */
pub const KEYCTL_REJECT: i32 = 19; /* reject a partially constructed key */
pub const KEYCTL_INSTANTIATE_IOV: i32 = 20; /* instantiate a partially constructed key */
pub const KEYCTL_INVALIDATE: i32 = 21; /* invalidate a key */
pub const KEYCTL_GET_PERSISTENT: i32 = 22; /* get a user's persistent keyring */
pub const KEYCTL_DH_COMPUTE: i32 = 23; /* Compute Diffie-Hellman values */
pub const KEYCTL_PKEY_QUERY: i32 = 24; /* Query public key parameters */
pub const KEYCTL_PKEY_ENCRYPT: i32 = 25; /* Encrypt a blob using a public key */
pub const KEYCTL_PKEY_DECRYPT: i32 = 26; /* Decrypt a blob using a public key */
pub const KEYCTL_PKEY_SIGN: i32 = 27; /* Create a public key signature */
pub const KEYCTL_PKEY_VERIFY: i32 = 28; /* Verify a public key signature */
pub const KEYCTL_RESTRICT_KEYRING: i32 = 29; /* Restrict keys allowed to link to a keyring */
pub const KEYCTL_MOVE: i32 = 30; /* Move keys between keyrings */
pub const KEYCTL_CAPABILITIES: i32 = 31; /* Find capabilities of keyrings subsystem */
pub const KEYCTL_WATCH_KEY: i32 = 32; /* Watch a key or ring of keys for changes */

/* keyctl structures */
#[repr(C)]
pub union keyctl_dh_params_private {
    pub r#private: i32,
    pub r#priv: i32,
}

#[repr(C)]
pub struct keyctl_dh_params {
    pub private_or_priv: keyctl_dh_params_private,
    pub prime: i32,
    pub base: i32,
}

#[repr(C)]
pub struct keyctl_kdf_params {
    pub hashname: *mut c_char,
    pub otherinfo: *mut c_char,
    pub otherinfolen: u32,
    pub __spare: [u32; 8],
}

pub const KEYCTL_SUPPORTS_ENCRYPT: i32 = 0x01;
pub const KEYCTL_SUPPORTS_DECRYPT: i32 = 0x02;
pub const KEYCTL_SUPPORTS_SIGN: i32 = 0x04;
pub const KEYCTL_SUPPORTS_VERIFY: i32 = 0x08;

#[repr(C)]
pub struct keyctl_pkey_query {
    pub supported_ops: u32, /* Which ops are supported */
    pub key_size: u32, /* Size of the key in bits */
    pub max_data_size: u16, /* Maximum size of raw data to sign in bytes */
    pub max_sig_size: u16, /* Maximum size of signature in bytes */
    pub max_enc_size: u16, /* Maximum size of encrypted blob in bytes */
    pub max_dec_size: u16, /* Maximum size of decrypted blob in bytes */
    pub __spare: [u32; 10],
}

#[repr(C)]
pub union keyctl_pkey_params_out_len {
    pub out_len: u32, /* Output buffer size (encrypt/decrypt/sign) */
    pub in2_len: u32, /* 2nd input data size (verify) */
}

#[repr(C)]
pub struct keyctl_pkey_params {
    pub key_id: i32, /* Serial no. of public key to use */
    pub in_len: u32, /* Input data size */
    pub out_len_or_in2_len: keyctl_pkey_params_out_len,
    pub __spare: [u32; 7],
}

pub const KEYCTL_MOVE_EXCL: i32 = 0x00000001; /* Do not displace from the to-keyring */

/*
 * Capabilities flags.  The capabilities list is an array of 8-bit integers;
 * each integer can carry up to 8 flags.
 */
pub const KEYCTL_CAPS0_CAPABILITIES: i32 = 0x01; /* KEYCTL_CAPABILITIES supported */
pub const KEYCTL_CAPS0_PERSISTENT_KEYRINGS: i32 = 0x02; /* Persistent keyrings enabled */
pub const KEYCTL_CAPS0_DIFFIE_HELLMAN: i32 = 0x04; /* Diffie-Hellman computation enabled */
pub const KEYCTL_CAPS0_PUBLIC_KEY: i32 = 0x08; /* Public key ops enabled */
pub const KEYCTL_CAPS0_BIG_KEY: i32 = 0x10; /* big_key-type enabled */
pub const KEYCTL_CAPS0_INVALIDATE: i32 = 0x20; /* KEYCTL_INVALIDATE supported */
pub const KEYCTL_CAPS0_RESTRICT_KEYRING: i32 = 0x40; /* KEYCTL_RESTRICT_KEYRING supported */
pub const KEYCTL_CAPS0_MOVE: i32 = 0x80; /* KEYCTL_MOVE supported */
pub const KEYCTL_CAPS1_NS_KEYRING_NAME: i32 = 0x01; /* Keyring names are per-user_namespace */
pub const KEYCTL_CAPS1_NS_KEY_TAG: i32 = 0x02; /* Key indexing can include a namespace tag */
pub const KEYCTL_CAPS1_NOTIFICATIONS: i32 = 0x04; /* Keys generate watchable notifications */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
