// SPDX-License-Identifier: GPL-2.0-or-later
/* PKCS#8 Private Key parser [RFC 5208].
 *
 * Copyright (C) 2016 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// C dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct Pkcs8ParseContext {
    pub pub_: *mut public_key,
    pub data: libc::c_ulong, // Start of data
    pub last_oid: OID,       // Last OID encountered
    pub algo_oid: OID,       // Algorithm OID
    pub key_size: u32,
    pub key: *const libc::c_void,
}

/*
 * Note an OID when we find one for later processing when we know how to
 * interpret it.
 */
pub unsafe extern "C" fn pkcs8_note_OID(
    context: *mut libc::c_void,
    _hdrlen: usize,
    _tag: libc::c_uchar,
    value: *const libc::c_void,
    vlen: usize,
) -> libc::c_int {
    let ctx = &mut *(context as *mut Pkcs8ParseContext);

    ctx.last_oid = look_up_OID(value, vlen);
    if ctx.last_oid == OID__NR {
        let mut buffer = [0i8; 50];
        sprint_oid(value, vlen, buffer.as_mut_ptr(), buffer.len());
        pr_info!(
            "Unknown OID: [{}] {}\n",
            (value as libc::c_ulong).wrapping_sub(ctx.data),
            buffer.as_ptr()
        );
    }
    0
}

/*
 * Note the version number of the ASN.1 blob.
 */
pub unsafe extern "C" fn pkcs8_note_version(
    _context: *mut libc::c_void,
    _hdrlen: usize,
    _tag: libc::c_uchar,
    value: *const libc::c_void,
    vlen: usize,
) -> libc::c_int {
    if vlen != 1 || *(value as *const u8) != 0 {
        pr_warn!("Unsupported PKCS#8 version\n");
        return -EBADMSG;
    }
    0
}

/*
 * Note the public algorithm.
 */
pub unsafe extern "C" fn pkcs8_note_algo(
    context: *mut libc::c_void,
    _hdrlen: usize,
    _tag: libc::c_uchar,
    _value: *const libc::c_void,
    _vlen: usize,
) -> libc::c_int {
    let ctx = &mut *(context as *mut Pkcs8ParseContext);

    if ctx.last_oid != OID_rsaEncryption {
        return -ENOPKG;
    }

    (*ctx.pub_).pkey_algo = b"rsa\0".as_ptr() as *const libc::c_char;
    0
}

/*
 * Note the key data of the ASN.1 blob.
 */
pub unsafe extern "C" fn pkcs8_note_key(
    context: *mut libc::c_void,
    _hdrlen: usize,
    _tag: libc::c_uchar,
    value: *const libc::c_void,
    vlen: usize,
) -> libc::c_int {
    let ctx = &mut *(context as *mut Pkcs8ParseContext);

    ctx.key = value;
    ctx.key_size = vlen as u32;
    0
}

/*
 * Parse a PKCS#8 private key blob.
 */
unsafe fn pkcs8_parse(data: *const libc::c_void, datalen: usize) -> *mut public_key {
    let mut ctx: Pkcs8ParseContext = core::mem::zeroed();
    let mut ret: libc::c_long;
    let pub_: *mut public_key;

    ret = -ENOMEM as libc::c_long;
    ctx.pub_ = kzalloc_obj::<public_key>();
    if ctx.pub_.is_null() {
        return ERR_PTR(ret);
    }

    ctx.data = data as libc::c_ulong;

    /* Attempt to decode the private key */
    ret = asn1_ber_decoder(&pkcs8_decoder, &mut ctx, data, datalen);
    if ret < 0 {
        kfree(ctx.pub_);
        return ERR_PTR(ret);
    }

    ret = -ENOMEM as libc::c_long;
    pub_ = ctx.pub_;
    (*pub_).key = kmemdup(ctx.key, ctx.key_size as usize, GFP_KERNEL);
    if (*pub_).key.is_null() {
        kfree(ctx.pub_);
        return ERR_PTR(ret);
    }

    (*pub_).keylen = ctx.key_size;
    (*pub_).key_is_private = true;
    pub_
}

/*
 * Attempt to parse a data blob for a key as a PKCS#8 private key.
 */
unsafe fn pkcs8_key_preparse(prep: *mut key_preparsed_payload) -> libc::c_int {
    let pub_ = pkcs8_parse((*prep).data, (*prep).datalen);
    if IS_ERR(pub_) {
        return PTR_ERR(pub_);
    }

    pr_devel!("Cert Key Algo: %s\n", (*pub_).pkey_algo);
    (*pub_).id_type = b"PKCS8\0".as_ptr() as *const libc::c_char;

    /* We're pinning the module by being linked against it */
    __module_get(public_key_subtype.owner);
    (*prep).payload.data[asym_subtype] = &public_key_subtype as *const _ as *mut _;
    (*prep).payload.data[asym_key_ids] = core::ptr::null_mut();
    (*prep).payload.data[asym_crypto] = pub_ as *mut _;
    (*prep).payload.data[asym_auth] = core::ptr::null_mut();
    (*prep).quotalen = 100;
    0
}

static mut pkcs8_key_parser: asymmetric_key_parser = asymmetric_key_parser {
    owner: THIS_MODULE,
    name: b"pkcs8\0".as_ptr() as *const libc::c_char,
    parse: Some(pkcs8_key_preparse),
};

/* Module stuff */
unsafe extern "C" fn pkcs8_key_init() -> libc::c_int {
    register_asymmetric_key_parser(&mut pkcs8_key_parser)
}

unsafe extern "C" fn pkcs8_key_exit() {
    unregister_asymmetric_key_parser(&mut pkcs8_key_parser);
}

module_init!(pkcs8_key_init);
module_exit!(pkcs8_key_exit);

module_description!("PKCS#8 certificate parser");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
