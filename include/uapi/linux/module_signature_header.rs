/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Module signature handling.
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Translated from the C header; linux/types.h supplies the original integer types. */

/* In stripped ARM and x86-64 modules, ~ is surprisingly rare. */
pub const MODULE_SIGNATURE_MARKER: &str = "~Module signature appended~\n";

#[repr(i32)]
pub enum module_signature_type {
    MODULE_SIGNATURE_TYPE_PKCS7 = 2, /* Signature in PKCS#7 message */
}

/*
 * Module signature information block.
 *
 * The constituents of the signature section are, in order:
 *
 *	- Signer's name
 *	- Key identifier
 *	- Signature data
 *	- Information block
 */
#[repr(C)]
pub struct module_signature {
    pub algo: u8,       /* Public-key crypto algorithm [0] */
    pub hash: u8,       /* Digest algorithm [0] */
    pub id_type: u8,    /* Key identifier type [enum module_signature_type] */
    pub signer_len: u8, /* Length of signer's name [0] */
    pub key_id_len: u8, /* Length of key identifier [0] */
    pub __pad: [u8; 3],
    pub sig_len: u32, /* __be32: Length of signature data */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
