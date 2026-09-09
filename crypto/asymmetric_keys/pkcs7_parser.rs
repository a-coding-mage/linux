// SPDX-License-Identifier: GPL-2.0-or-later
/* PKCS#7 parser
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the kernel PKCS#7, X.509, ASN.1 and public-key
// interfaces are intentionally left as external Rust items.

#[repr(C)]
pub struct Pkcs7ParseContext {
    pub msg: *mut pkcs7_message,
    pub sinfo: *mut pkcs7_signed_info,
    pub ppsinfo: *mut *mut pkcs7_signed_info,
    pub certs: *mut x509_certificate,
    pub ppcerts: *mut *mut x509_certificate,
    pub data: usize,
    pub last_oid: OID,
    pub x509_index: u32,
    pub sinfo_index: u32,
    pub raw_serial: *const core::ffi::c_void,
    pub raw_serial_size: u32,
    pub raw_issuer_size: u32,
    pub raw_issuer: *const core::ffi::c_void,
    pub raw_skid: *const core::ffi::c_void,
    pub raw_skid_size: u32,
    pub expect_skid: bool,
}

unsafe fn pkcs7_free_signed_info(sinfo: *mut pkcs7_signed_info) {
    if !sinfo.is_null() {
        public_key_signature_free((*sinfo).sig);
        kfree(sinfo.cast());
    }
}

pub unsafe fn pkcs7_free_message(mut pkcs7: *mut pkcs7_message) {
    if pkcs7.is_null() { return; }
    while !(*pkcs7).certs.is_null() {
        let cert = (*pkcs7).certs;
        (*pkcs7).certs = (*cert).next;
        x509_free_certificate(cert);
    }
    while !(*pkcs7).crl.is_null() {
        let cert = (*pkcs7).crl;
        (*pkcs7).crl = (*cert).next;
        x509_free_certificate(cert);
    }
    while !(*pkcs7).signed_infos.is_null() {
        let sinfo = (*pkcs7).signed_infos;
        (*pkcs7).signed_infos = (*sinfo).next;
        pkcs7_free_signed_info(sinfo);
    }
    kfree(pkcs7.cast());
}

unsafe fn pkcs7_check_authattrs(msg: *mut pkcs7_message) -> i32 {
    let mut sinfo = (*msg).signed_infos;
    let mut want = false;
    if sinfo.is_null() { return -EINVAL; }
    if !(*sinfo).authattrs.is_null() {
        want = true;
        (*msg).have_authattrs = true;
    } else if (*(*sinfo).sig).algo_takes_data { (*(*sinfo).sig).hash_algo = c"none".as_ptr(); }
    sinfo = (*sinfo).next;
    while !sinfo.is_null() {
        if ((!(*sinfo).authattrs.is_null()) != want) { return -EINVAL; }
        if (*sinfo).authattrs.is_null() && (*(*sinfo).sig).algo_takes_data {
            (*(*sinfo).sig).hash_algo = c"none".as_ptr();
        }
        sinfo = (*sinfo).next;
    }
    0
}

pub unsafe fn pkcs7_parse_message(data: *const core::ffi::c_void, datalen: usize) -> *mut pkcs7_message {
    let ctx = kzalloc::<Pkcs7ParseContext>();
    if ctx.is_null() { return ERR_PTR(-ENOMEM); }
    (*ctx).msg = kzalloc::<pkcs7_message>();
    if (*ctx).msg.is_null() { kfree(ctx.cast()); return ERR_PTR(-ENOMEM); }
    (*ctx).sinfo = kzalloc::<pkcs7_signed_info>();
    if (*ctx).sinfo.is_null() { pkcs7_free_message((*ctx).msg); kfree(ctx.cast()); return ERR_PTR(-ENOMEM); }
    (*(*ctx).sinfo).sig = kzalloc::<public_key_signature>();
    if (*(*ctx).sinfo).sig.is_null() { pkcs7_free_signed_info((*ctx).sinfo); pkcs7_free_message((*ctx).msg); kfree(ctx.cast()); return ERR_PTR(-ENOMEM); }
    (*ctx).data = data as usize;
    (*ctx).ppcerts = &mut (*ctx).certs;
    (*ctx).ppsinfo = &mut (*(*ctx).msg).signed_infos;
    let mut ret = asn1_ber_decoder(&pkcs7_decoder, ctx.cast(), data, datalen);
    if ret >= 0 { ret = pkcs7_check_authattrs((*ctx).msg); }
    if ret < 0 {
        while !(*ctx).certs.is_null() { let cert = (*ctx).certs; (*ctx).certs = (*cert).next; x509_free_certificate(cert); }
        pkcs7_free_signed_info((*ctx).sinfo);
        pkcs7_free_message((*ctx).msg);
        kfree(ctx.cast());
        return ERR_PTR(ret);
    }
    let msg = (*ctx).msg;
    (*ctx).msg = core::ptr::null_mut();
    pkcs7_free_signed_info((*ctx).sinfo);
    kfree(ctx.cast());
    msg
}

pub unsafe fn pkcs7_get_content_data(pkcs7: *const pkcs7_message, data: *mut *const core::ffi::c_void, len: *mut usize, header: *mut usize) -> i32 {
    if (*pkcs7).data.is_null() { return -ENODATA; }
    *data = (*pkcs7).data; *len = (*pkcs7).data_len;
    if !header.is_null() { *header = (*pkcs7).data_hdrlen; } 0
}

pub unsafe fn pkcs7_note_OID(context: *mut core::ffi::c_void, _hdrlen: usize, _tag: u8, value: *const core::ffi::c_void, vlen: usize) -> i32 {
    let ctx = context.cast::<Pkcs7ParseContext>(); (*ctx).last_oid = look_up_OID(value, vlen); 0
}

pub unsafe fn pkcs7_sig_note_digest_algo(context: *mut core::ffi::c_void, _: usize, _: u8, _: *const core::ffi::c_void, _: usize) -> i32 {
    let c = &mut *context.cast::<Pkcs7ParseContext>();
    (*c.sinfo).sig_hash_algo(c.last_oid)
}

pub unsafe fn pkcs7_sig_note_pkey_algo(context: *mut core::ffi::c_void, _: usize, _: u8, _: *const core::ffi::c_void, _: usize) -> i32 {
    let c = &mut *context.cast::<Pkcs7ParseContext>(); (*c.sinfo).sig_pkey_algo(c.last_oid)
}

pub unsafe fn pkcs7_check_content_type(context: *mut core::ffi::c_void, _: usize, _: u8, _: *const core::ffi::c_void, _: usize) -> i32 {
    if (*context.cast::<Pkcs7ParseContext>()).last_oid != OID_signed_data { -EINVAL } else { 0 }
}

pub unsafe fn pkcs7_note_signeddata_version(context: *mut core::ffi::c_void, _: usize, _: u8, value: *const core::ffi::c_void, vlen: usize) -> i32 {
    if vlen != 1 { return -EINVAL; } let c = &mut *context.cast::<Pkcs7ParseContext>(); let v = *(value as *const u8) as u32;
    (*c.msg).version = v; if v == 1 || v == 3 { 0 } else { -EINVAL }
}

pub unsafe fn pkcs7_note_signerinfo_version(context: *mut core::ffi::c_void, _: usize, _: u8, value: *const core::ffi::c_void, vlen: usize) -> i32 {
    if vlen != 1 { return -EINVAL; } let c = &mut *context.cast::<Pkcs7ParseContext>(); let v = *(value as *const u8);
    match v { 1 if (*c.msg).version == 1 => { c.expect_skid = false; 0 }, 3 if (*c.msg).version != 1 => { c.expect_skid = true; 0 }, _ => -EINVAL }
}

pub unsafe fn pkcs7_extract_cert(context: *mut core::ffi::c_void, hdrlen: usize, tag: u8, value: *const core::ffi::c_void, mut vlen: usize) -> i32 {
    let c = &mut *context.cast::<Pkcs7ParseContext>();
    if tag != ((ASN1_UNIV << 6) | ASN1_CONS_BIT | ASN1_SEQ) { return -EBADMSG; }
    let value = (value as *const u8).sub(hdrlen); vlen += hdrlen;
    if *value.add(1) == 0x80 { vlen += 2; }
    let x509 = x509_cert_parse(value.cast(), vlen); if IS_ERR(x509) { return PTR_ERR(x509); }
    (*x509).index = c.x509_index + 1; c.x509_index = (*x509).index;
    *c.ppcerts = x509; c.ppcerts = &mut (*x509).next; 0
}

pub unsafe fn pkcs7_note_certificate_list(context: *mut core::ffi::c_void, _: usize, tag: u8, _: *const core::ffi::c_void, _: usize) -> i32 {
    let c = &mut *context.cast::<Pkcs7ParseContext>(); *c.ppcerts = (*c.msg).certs; (*c.msg).certs = c.certs; c.certs = core::ptr::null_mut(); c.ppcerts = &mut c.certs; 0
}

pub unsafe fn pkcs7_note_content(context: *mut core::ffi::c_void, _: usize, _: u8, _: *const core::ffi::c_void, _: usize) -> i32 {
    let c = &mut *context.cast::<Pkcs7ParseContext>(); if c.last_oid != OID_data && c.last_oid != OID_msIndirectData { return -EINVAL; } (*c.msg).data_type = c.last_oid; 0
}

pub unsafe fn pkcs7_note_data(context: *mut core::ffi::c_void, hdrlen: usize, _: u8, value: *const core::ffi::c_void, vlen: usize) -> i32 {
    let c = &mut *context.cast::<Pkcs7ParseContext>(); (*c.msg).data = value; (*c.msg).data_len = vlen; (*c.msg).data_hdrlen = hdrlen; 0
}

pub unsafe fn pkcs7_sig_note_authenticated_attr(context: *mut core::ffi::c_void, hdrlen: usize, tag: u8, value: *const core::ffi::c_void, vlen: usize) -> i32 {
    let c = &mut *context.cast::<Pkcs7ParseContext>(); let s = &mut *c.sinfo;
    match c.last_oid {
        OID_contentType => { if test_and_set_bit(sinfo_has_content_type, &mut s.aa_set) { return -EKEYREJECTED; } if look_up_OID(value,vlen) != (*c.msg).data_type { return -EBADMSG; } 0 }
        OID_signingTime => { if test_and_set_bit(sinfo_has_signing_time, &mut s.aa_set) { return -EKEYREJECTED; } x509_decode_time(&mut s.signing_time,hdrlen,tag,value,vlen) }
        OID_messageDigest => { if test_and_set_bit(sinfo_has_message_digest,&mut s.aa_set) || tag != ASN1_OTS { return -EBADMSG; } s.msgdigest=value; s.msgdigest_len=vlen; 0 }
        OID_smimeCapabilites => { if test_and_set_bit(sinfo_has_smime_caps,&mut s.aa_set) { return -EKEYREJECTED; } if (*c.msg).data_type != OID_msIndirectData { return -EKEYREJECTED; } 0 }
        OID_msSpOpusInfo => { if test_and_set_bit(sinfo_has_ms_opus_info,&mut s.aa_set) { return -EKEYREJECTED; } if (*c.msg).data_type != OID_msIndirectData { return -EKEYREJECTED; } 0 }
        OID_msStatementType => { if test_and_set_bit(sinfo_has_ms_statement_type,&mut s.aa_set) { return -EKEYREJECTED; } if (*c.msg).data_type != OID_msIndirectData { return -EKEYREJECTED; } 0 }
        _ => 0,
    }
}

pub unsafe fn pkcs7_sig_note_set_of_authattrs(context:*mut core::ffi::c_void,hdrlen:usize,_tag:u8,value:*const core::ffi::c_void,vlen:usize)->i32 { let c=&mut *context.cast::<Pkcs7ParseContext>(); let s=&mut *c.sinfo; if !test_bit(sinfo_has_content_type,s.aa_set)||!test_bit(sinfo_has_message_digest,s.aa_set){return -EBADMSG;} s.authattrs=(value as *const u8).sub(hdrlen).cast(); s.authattrs_len=vlen+hdrlen; 0 }
pub unsafe fn pkcs7_sig_note_serial(c:*mut core::ffi::c_void,_:usize,_:u8,v:*const core::ffi::c_void,n:usize)->i32{let c=&mut *c.cast::<Pkcs7ParseContext>();c.raw_serial=v;c.raw_serial_size=n as u32;0}
pub unsafe fn pkcs7_sig_note_issuer(c:*mut core::ffi::c_void,_:usize,_:u8,v:*const core::ffi::c_void,n:usize)->i32{let c=&mut *c.cast::<Pkcs7ParseContext>();c.raw_issuer=v;c.raw_issuer_size=n as u32;0}
pub unsafe fn pkcs7_sig_note_skid(c:*mut core::ffi::c_void,_:usize,_:u8,v:*const core::ffi::c_void,n:usize)->i32{let c=&mut *c.cast::<Pkcs7ParseContext>();c.raw_skid=v;c.raw_skid_size=n as u32;0}
pub unsafe fn pkcs7_sig_note_signature(c:*mut core::ffi::c_void,_:usize,_:u8,v:*const core::ffi::c_void,n:usize)->i32{let c=&mut *c.cast::<Pkcs7ParseContext>();(*(*c).sinfo).sig.s=kmemdup(v,n,GFP_KERNEL);if (*(*c).sinfo).sig.s.is_null(){return -ENOMEM;}(*(*c).sinfo).sig.s_size=n;0}
pub unsafe fn pkcs7_note_signed_info(c:*mut core::ffi::c_void,_:usize,_:u8,_:*const core::ffi::c_void,_:usize)->i32{
    let c=&mut *c.cast::<Pkcs7ParseContext>(); let s=c.sinfo; if (*c.msg).data_type==OID_msIndirectData&&(*s).authattrs.is_null(){return -EBADMSG;}
    let kid=if !c.expect_skid{asymmetric_key_generate_id(c.raw_serial,c.raw_serial_size as usize,c.raw_issuer,c.raw_issuer_size as usize)}else{asymmetric_key_generate_id(c.raw_skid,c.raw_skid_size as usize,c"".as_ptr().cast(),0)};
    if IS_ERR(kid){return PTR_ERR(kid);} (*s).sig.auth_ids[0]=kid; (*s).index=c.sinfo_index+1;c.sinfo_index=(*s).index;*c.ppsinfo=s;c.ppsinfo=&mut (*s).next;c.sinfo=kzalloc::<pkcs7_signed_info>();if c.sinfo.is_null(){return -ENOMEM;}(*c.sinfo).sig=kzalloc::<public_key_signature>();if (*c.sinfo).sig.is_null(){return -ENOMEM;}0
}

extern "C" {
    static pkcs7_decoder: core::ffi::c_void;
    fn asn1_ber_decoder(decoder: *const core::ffi::c_void, context: *mut core::ffi::c_void, data: *const core::ffi::c_void, len: usize) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
