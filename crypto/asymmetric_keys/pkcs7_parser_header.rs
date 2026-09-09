/* SPDX-License-Identifier: GPL-2.0-or-later */
/* PKCS#7 crypto data parser internal definitions
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// C dependencies: linux/oid_registry.h, crypto/pkcs7.h, and x509_parser.h.

macro_rules! kenter {
    ($fmt:expr $(, $args:expr)*) => {
        pr_devel!("==> {}(" $fmt ")\n", module_path!() $(, $args)*)
    };
}

macro_rules! kleave {
    ($fmt:expr $(, $args:expr)*) => {
        pr_devel!("<== {}()" $fmt "\n", module_path!() $(, $args)*)
    };
}

#[repr(C)]
pub struct pkcs7_signed_info {
    pub next: *mut pkcs7_signed_info,
    pub signer: *mut x509_certificate, /* Signing certificate (in msg->certs) */
    pub index: u32,
    pub unsupported_crypto: bool, /* T if not usable due to missing crypto */
    pub blacklisted: bool,

    /* Message digest - the digest of the Content Data (or NULL) */
    pub msgdigest: *const core::ffi::c_void,
    pub msgdigest_len: u32,

    /* Authenticated Attribute data (or NULL) */
    pub authattrs_len: u32,
    pub authattrs: *const core::ffi::c_void,
    pub aa_set: core::ffi::c_ulong,

    pub signing_time: i64,

    /* Message signature.
     *
     * This contains the generated digest of _either_ the Content Data or
     * the Authenticated Attributes [RFC2315 9.3].  If the latter, one of
     * the attributes contains the digest of the Content Data within it.
     *
     * This also contains the issuing cert serial number and issuer's name
     * [PKCS#7 or CMS ver 1] or issuing cert's SKID [CMS ver 3].
     */
    pub sig: *mut public_key_signature,
}

pub const SINFO_HAS_CONTENT_TYPE: u32 = 0;
pub const SINFO_HAS_SIGNING_TIME: u32 = 1;
pub const SINFO_HAS_MESSAGE_DIGEST: u32 = 2;
pub const SINFO_HAS_SMIME_CAPS: u32 = 3;
pub const SINFO_HAS_MS_OPUS_INFO: u32 = 4;
pub const SINFO_HAS_MS_STATEMENT_TYPE: u32 = 5;

#[repr(C)]
pub struct pkcs7_message {
    pub certs: *mut x509_certificate, /* Certificate list */
    pub crl: *mut x509_certificate, /* Revocation list */
    pub signed_infos: *mut pkcs7_signed_info,
    pub version: u8, /* Version of cert (1 -> PKCS#7 or CMS; 3 -> CMS) */
    pub have_authattrs: bool, /* T if have authattrs */
    // C conditional field: CONFIG_PKCS7_WAIVE_AUTHATTRS_REJECTION_FOR_MLDSA.
    #[cfg(CONFIG_PKCS7_WAIVE_AUTHATTRS_REJECTION_FOR_MLDSA)]
    pub authattrs_rej_waivable: bool, /* T if authatts rejection can be waived */

    /* Content Data (or NULL) */
    pub data_type: OID, /* Type of Data */
    pub data_len: usize, /* Length of Data */
    pub data_hdrlen: usize, /* Length of Data ASN.1 header */
    pub data: *const core::ffi::c_void, /* Content Data (or 0) */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
