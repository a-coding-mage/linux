/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Asymmetric Public-key cryptography key type interface
 *
 * See Documentation/crypto/asymmetric-keys.rst
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the surrounding kernel translation.

extern "C" {
    pub static mut key_type_asymmetric: key_type;

    pub fn asymmetric_key_id_same(
        kid1: *const asymmetric_key_id,
        kid2: *const asymmetric_key_id,
    ) -> bool;

    pub fn asymmetric_key_id_partial(
        kid1: *const asymmetric_key_id,
        kid2: *const asymmetric_key_id,
    ) -> bool;

    pub fn asymmetric_key_generate_id(
        val_1: *const core::ffi::c_void,
        len_1: usize,
        val_2: *const core::ffi::c_void,
        len_2: usize,
    ) -> *mut asymmetric_key_id;

    pub fn find_asymmetric_key(
        keyring: *mut key,
        id_0: *const asymmetric_key_id,
        id_1: *const asymmetric_key_id,
        id_2: *const asymmetric_key_id,
        partial: bool,
    ) -> *mut key;

    pub fn x509_load_certificate_list(
        cert_list: *const u8,
        list_size: u64,
        keyring: *const key,
    ) -> core::ffi::c_int;
}

/* The key payload is four words.  The asymmetric-type key uses them as
 * follows:
 */
#[repr(C)]
pub enum asymmetric_payload_bits {
    asym_crypto,
    asym_subtype,
    asym_key_ids,
    asym_auth,
}

/*
 * Identifiers for an asymmetric key ID.  We have three ways of looking up a
 * key derived from an X.509 certificate:
 *
 * (1) Serial Number & Issuer.  Non-optional.  This is the only valid way to
 *     map a PKCS#7 signature to an X.509 certificate.
 *
 * (2) Issuer & Subject Unique IDs.  Optional.  These were the original way to
 *     match X.509 certificates, but have fallen into disuse in favour of (3).
 *
 * (3) Auth & Subject Key Identifiers.  Optional.  SKIDs are only provided on
 *     CA keys that are intended to sign other keys, so don't appear in end
 *     user certificates unless forced.
 *
 * We could also support an PGP key identifier, which is just a SHA1 sum of the
 * public key and certain parameters, but since we don't support PGP keys at
 * the moment, we shall ignore those.
 *
 * What we actually do is provide a place where binary identifiers can be
 * stashed and then compare against them when checking for an id match.
 */
#[repr(C)]
pub struct asymmetric_key_id {
    pub len: u16,
    // C flexible array member: data[] __counted_by(len)
    pub data: [u8; 0],
}

#[repr(C)]
pub struct asymmetric_key_ids {
    pub id: [*mut core::ffi::c_void; 3],
}

#[inline]
pub unsafe fn asymmetric_key_ids(key: *const key) -> *const asymmetric_key_ids {
    (*key).payload.data[asymmetric_payload_bits::asym_key_ids as usize]
        as *const asymmetric_key_ids
}

#[inline]
pub unsafe fn asymmetric_key_public_key(key: *const key) -> *const public_key {
    (*key).payload.data[asymmetric_payload_bits::asym_crypto as usize]
        as *const public_key
}

/* The payload is at the discretion of the subtype. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
