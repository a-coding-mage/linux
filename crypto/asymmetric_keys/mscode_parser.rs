// SPDX-License-Identifier: GPL-2.0-or-later
/* Parse a Microsoft Individual Code Signing blob
 *
 * Copyright (C) 2014 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_void};

// Declarations supplied by the kernel and by the surrounding parser.
#[repr(C)]
pub struct pefile_context {
    pub digest_algo: *const c_char,
    pub digest: *mut c_void,
    pub digest_len: usize,
}

#[repr(C)]
pub struct asn1_decoder;

#[allow(non_upper_case_globals)]
extern "C" {
    pub static mscode_decoder: asn1_decoder;
    pub fn asn1_ber_decoder(
        decoder: *const asn1_decoder,
        context: *mut pefile_context,
        data: *const c_void,
        data_len: usize,
    ) -> c_int;
    pub fn look_up_OID(value: *const c_void, vlen: usize) -> c_int;
    pub fn sprint_oid(value: *const c_void, vlen: usize, buffer: *mut c_char, buflen: usize);
    pub fn kmemdup(value: *const c_void, vlen: usize, flags: c_uint) -> *mut c_void;
    pub fn pr_devel(fmt: *const c_char, ...);
    pub fn pr_err(fmt: *const c_char, ...);
}

// OID values and errno/GFP constants are supplied by the surrounding build.
extern "C" {
    pub static OID__NR: c_int;
    pub static OID_msPeImageDataObjId: c_int;
    pub static OID_msIndividualSPKeyPurpose: c_int;
    pub static OID_sha1: c_int;
    pub static OID_sha256: c_int;
    pub static OID_sha384: c_int;
    pub static OID_sha512: c_int;
    pub static OID_sha3_256: c_int;
    pub static OID_sha3_384: c_int;
    pub static OID_sha3_512: c_int;
    pub static GFP_KERNEL: c_uint;
}

/* Parse a Microsoft Individual Code Signing blob */
pub unsafe fn mscode_parse(
    _ctx: *mut c_void,
    content_data: *const c_void,
    data_len: usize,
    asn1hdrlen: usize,
) -> c_int {
    let ctx = _ctx as *mut pefile_context;
    let content_data = (content_data as *const c_uchar).sub(asn1hdrlen);
    let data_len = data_len + asn1hdrlen;

    // pr_devel("Data: %zu [%*ph]\n", data_len, (unsigned)(data_len), content_data);
    asn1_ber_decoder(&mscode_decoder, ctx, content_data as *const c_void, data_len)
}

/* Check the content type OID */
pub unsafe fn mscode_note_content_type(
    _context: *mut c_void,
    _hdrlen: usize,
    _tag: c_uchar,
    value: *const c_void,
    vlen: usize,
) -> c_int {
    let oid = look_up_OID(value, vlen);
    if oid == OID__NR {
        let mut buffer = [0 as c_char; 50];
        sprint_oid(value, vlen, buffer.as_mut_ptr(), buffer.len());
        // pr_err("Unknown OID: %s\n", buffer);
        return -74; // -EBADMSG
    }

    /*
     * pesign utility had a bug where it was putting
     * OID_msIndividualSPKeyPurpose instead of OID_msPeImageDataObjId
     * So allow both OIDs.
     */
    if oid != OID_msPeImageDataObjId && oid != OID_msIndividualSPKeyPurpose {
        // pr_err("Unexpected content type OID %u\n", oid);
        return -74; // -EBADMSG
    }
    0
}

/* Note the digest algorithm OID */
pub unsafe fn mscode_note_digest_algo(
    context: *mut c_void,
    _hdrlen: usize,
    _tag: c_uchar,
    value: *const c_void,
    vlen: usize,
) -> c_int {
    let ctx = context as *mut pefile_context;
    let mut buffer = [0 as c_char; 50];
    let oid = look_up_OID(value, vlen);
    let algorithm: *const c_char;

    if oid == OID_sha1 { algorithm = b"sha1\0".as_ptr() as *const c_char;
    } else if oid == OID_sha256 { algorithm = b"sha256\0".as_ptr() as *const c_char;
    } else if oid == OID_sha384 { algorithm = b"sha384\0".as_ptr() as *const c_char;
    } else if oid == OID_sha512 { algorithm = b"sha512\0".as_ptr() as *const c_char;
    } else if oid == OID_sha3_256 { algorithm = b"sha3-256\0".as_ptr() as *const c_char;
    } else if oid == OID_sha3_384 { algorithm = b"sha3-384\0".as_ptr() as *const c_char;
    } else if oid == OID_sha3_512 { algorithm = b"sha3-512\0".as_ptr() as *const c_char;
    } else if oid == OID__NR {
        sprint_oid(value, vlen, buffer.as_mut_ptr(), buffer.len());
        // pr_err("Unknown OID: %s\n", buffer);
        return -74; // -EBADMSG
    } else {
        // pr_err("Unsupported content type: %u\n", oid);
        return -65; // -ENOPKG
    }
    (*ctx).digest_algo = algorithm;
    0
}

/* Note the digest we're guaranteeing with this certificate */
pub unsafe fn mscode_note_digest(
    context: *mut c_void,
    _hdrlen: usize,
    _tag: c_uchar,
    value: *const c_void,
    vlen: usize,
) -> c_int {
    let ctx = context as *mut pefile_context;
    (*ctx).digest = kmemdup(value, vlen, GFP_KERNEL);
    if (*ctx).digest.is_null() {
        return -12; // -ENOMEM
    }
    (*ctx).digest_len = vlen;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
