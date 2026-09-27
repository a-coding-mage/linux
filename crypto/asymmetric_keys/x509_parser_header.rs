/* SPDX-License-Identifier: GPL-2.0-or-later */
/* X.509 certificate parser internal definitions
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct public_key {
    _private: [u8; 0],
}

#[repr(C)]
pub struct public_key_signature {
    _private: [u8; 0],
}

#[repr(C)]
pub struct asymmetric_key_id {
    _private: [u8; 0],
}

pub const SHA256_DIGEST_SIZE: usize = 32;

#[repr(C)]
pub struct x509_certificate {
    pub next: *mut x509_certificate,
    pub signer: *mut x509_certificate, /* Certificate that signed this one */
    pub r#pub: *mut public_key,        /* Public key details */
    pub sig: *mut public_key_signature, /* Signature parameters */
    pub sha256: [u8; SHA256_DIGEST_SIZE], /* Hash for blacklist purposes */
    pub issuer: *mut ::kernel::ffi::c_char, /* Name of certificate issuer */
    pub subject: *mut ::kernel::ffi::c_char, /* Name of certificate subject */
    pub id: *mut asymmetric_key_id, /* Issuer + Serial number */
    pub skid: *mut asymmetric_key_id, /* Subject + subjectKeyId (optional) */
    pub valid_from: i64,
    pub valid_to: i64,
    pub tbs: *const ::kernel::ffi::c_void, /* Signed data */
    pub tbs_size: ::kernel::ffi::c_uint,   /* Size of signed data */
    pub raw_sig_size: ::kernel::ffi::c_uint, /* Size of signature */
    pub raw_sig: *const ::kernel::ffi::c_void, /* Signature data */
    pub raw_serial: *const ::kernel::ffi::c_void, /* Raw serial number in ASN.1 */
    pub raw_serial_size: ::kernel::ffi::c_uint,
    pub raw_issuer_size: ::kernel::ffi::c_uint,
    pub raw_issuer: *const ::kernel::ffi::c_void, /* Raw issuer name in ASN.1 */
    pub raw_subject: *const ::kernel::ffi::c_void, /* Raw subject name in ASN.1 */
    pub raw_subject_size: ::kernel::ffi::c_uint,
    pub raw_skid_size: ::kernel::ffi::c_uint,
    pub raw_skid: *const ::kernel::ffi::c_void, /* Raw subjectKeyId in ASN.1 */
    pub index: ::kernel::ffi::c_uint,
    pub seen: bool, /* Infinite recursion prevention */
    pub verified: bool,
    pub self_signed: bool, /* T if self-signed (check unsupported_sig too) */
    pub unsupported_sig: bool, /* T if signature uses unsupported crypto */
    pub blacklisted: bool,
}

/*
 * x509_cert_parser.c
 */
extern "C" {
    pub fn x509_free_certificate(cert: *mut x509_certificate);
    pub fn x509_cert_parse(data: *const ::kernel::ffi::c_void, datalen: usize)
        -> *mut x509_certificate;
    pub fn x509_decode_time(
        t: *mut i64,
        hdrlen: usize,
        tag: u8,
        value: *const u8,
        vlen: usize,
    ) -> ::kernel::ffi::c_int;
}

/* DEFINE_FREE(x509_free_certificate, ...) cleanup association is supplied by the caller. */

/*
 * x509_public_key.c
 */
extern "C" {
    pub fn x509_get_sig_params(cert: *mut x509_certificate) -> ::kernel::ffi::c_int;
    pub fn x509_check_for_self_signed(cert: *mut x509_certificate) -> ::kernel::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
