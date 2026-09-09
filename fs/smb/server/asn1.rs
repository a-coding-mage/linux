// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * The ASB.1/BER parsing code is derived from ip_nat_snmp_basic.c which was in
 * turn derived from the gxsnmp package by Gregory McLean & Jochen Friedrich
 *
 * Copyright (c) 2000 RP Internet (www.rpi.net.au).
 */

// Kernel and local header dependencies are supplied by the surrounding crate.

const NTLMSSP_OID_LEN: usize = 10;

static mut NTLMSSP_OID_STR: [i8; NTLMSSP_OID_LEN] = [
    0x2b, 0x06, 0x01, 0x04, 0x01, 0x82u8 as i8, 0x37, 0x02, 0x02, 0x0a,
];

pub unsafe fn ksmbd_decode_negTokenInit(
    security_blob: *mut u8,
    length: i32,
    conn: *mut ksmbd_conn,
) -> i32 {
    asn1_ber_decoder(
        &ksmbd_spnego_negtokeninit_decoder,
        conn as *mut core::ffi::c_void,
        security_blob,
        length,
    )
}

pub unsafe fn ksmbd_decode_negTokenTarg(
    security_blob: *mut u8,
    length: i32,
    conn: *mut ksmbd_conn,
) -> i32 {
    asn1_ber_decoder(
        &ksmbd_spnego_negtokentarg_decoder,
        conn as *mut core::ffi::c_void,
        security_blob,
        length,
    )
}

unsafe fn compute_asn_hdr_len_bytes(len: i32) -> i32 {
    if len > 0xFFFFFF { 4 } else if len > 0xFFFF { 3 } else if len > 0xFF { 2 } else if len > 0x7F { 1 } else { 0 }
}

unsafe fn encode_asn_tag(buf: *mut i8, ofs: *mut u32, tag: i8, seq: i8, length: i32) {
    let mut i: i32;
    let mut index = *ofs as usize;
    let hdr_len = compute_asn_hdr_len_bytes(length);
    let mut len = length + 2 + hdr_len;

    *buf.add(index) = tag;
    index += 1;

    if hdr_len == 0 {
        *buf.add(index) = len as i8;
        index += 1;
    } else {
        *buf.add(index) = (0x80 | hdr_len) as i8;
        index += 1;
        i = hdr_len - 1;
        while i >= 0 {
            *buf.add(index) = (len >> (i * 8)) as i8;
            index += 1;
            i -= 1;
        }
    }

    len -= (index as u32 - *ofs) as i32;
    *buf.add(index) = seq;
    index += 1;

    if hdr_len == 0 {
        *buf.add(index) = len as i8;
        index += 1;
    } else {
        *buf.add(index) = (0x80 | hdr_len) as i8;
        index += 1;
        i = hdr_len - 1;
        while i >= 0 {
            *buf.add(index) = (len >> (i * 8)) as i8;
            index += 1;
            i -= 1;
        }
    }

    *ofs += (index as u32 - *ofs);
}

pub unsafe fn build_spnego_ntlmssp_neg_blob(
    pbuffer: *mut *mut u8,
    buflen: *mut u16,
    ntlm_blob: *mut i8,
    ntlm_blob_len: i32,
) -> i32 {
    let neg_result_len = 4 + compute_asn_hdr_len_bytes(1) * 2 + 1;
    let oid_len = 4 + compute_asn_hdr_len_bytes(NTLMSSP_OID_LEN as i32) * 2 + NTLMSSP_OID_LEN as i32;
    let ntlmssp_len = 4 + compute_asn_hdr_len_bytes(ntlm_blob_len) * 2 + ntlm_blob_len;
    let total_len = 4 + compute_asn_hdr_len_bytes(neg_result_len + oid_len + ntlmssp_len) * 2 + neg_result_len + oid_len + ntlmssp_len;
    let buf = kmalloc(total_len as usize, KSMBD_DEFAULT_GFP) as *mut i8;
    if buf.is_null() { return -ENOMEM; }
    let mut ofs = 0u32;

    encode_asn_tag(buf, &mut ofs, 0xa1u8 as i8, 0x30, neg_result_len + oid_len + ntlmssp_len);
    encode_asn_tag(buf, &mut ofs, 0xa0u8 as i8, 0x0a, 1);
    *buf.add(ofs as usize) = 1; ofs += 1;
    encode_asn_tag(buf, &mut ofs, 0xa1u8 as i8, 0x06, NTLMSSP_OID_LEN as i32);
    core::ptr::copy_nonoverlapping(NTLMSSP_OID_STR.as_ptr(), buf.add(ofs as usize), NTLMSSP_OID_LEN); ofs += NTLMSSP_OID_LEN as u32;
    encode_asn_tag(buf, &mut ofs, 0xa2u8 as i8, 0x04, ntlm_blob_len);
    core::ptr::copy_nonoverlapping(ntlm_blob, buf.add(ofs as usize), ntlm_blob_len as usize); ofs += ntlm_blob_len as u32;
    *pbuffer = buf as *mut u8; *buflen = total_len as u16; 0
}

pub unsafe fn build_spnego_ntlmssp_auth_blob(pbuffer: *mut *mut u8, buflen: *mut u16, neg_result: i32) -> i32 {
    let neg_result_len = 4 + compute_asn_hdr_len_bytes(1) * 2 + 1;
    let total_len = 4 + compute_asn_hdr_len_bytes(neg_result_len) * 2 + neg_result_len;
    let buf = kmalloc(total_len as usize, KSMBD_DEFAULT_GFP) as *mut i8;
    if buf.is_null() { return -ENOMEM; }
    let mut ofs = 0u32;
    encode_asn_tag(buf, &mut ofs, 0xa1u8 as i8, 0x30, neg_result_len);
    encode_asn_tag(buf, &mut ofs, 0xa0u8 as i8, 0x0a, 1);
    *buf.add(ofs as usize) = if neg_result != 0 { 2 } else { 0 }; ofs += 1;
    *pbuffer = buf as *mut u8; *buflen = total_len as u16; 0
}

pub unsafe fn ksmbd_gssapi_this_mech(context: *mut core::ffi::c_void, hdrlen: usize, tag: u8, value: *const core::ffi::c_void, vlen: usize) -> i32 {
    let oid = look_up_OID(value, vlen);
    if oid != OID_spnego { let mut buf = [0i8; 50]; sprint_oid(value, vlen, buf.as_mut_ptr(), buf.len()); ksmbd_debug(AUTH, b"Unexpected OID: %s\0".as_ptr(), buf.as_ptr()); return -EBADMSG; } 0
}

pub unsafe fn ksmbd_neg_token_init_mech_type(context: *mut core::ffi::c_void, hdrlen: usize, tag: u8, value: *const core::ffi::c_void, vlen: usize) -> i32 {
    let conn = context as *mut ksmbd_conn;
    let oid = look_up_OID(value, vlen);
    let mech_type = if oid == OID_ntlmssp { KSMBD_AUTH_NTLMSSP } else if oid == OID_mskrb5 { KSMBD_AUTH_MSKRB5 } else if oid == OID_krb5 { KSMBD_AUTH_KRB5 } else if oid == OID_krb5u2u { KSMBD_AUTH_KRB5U2U } else { let mut buf = [0i8; 50]; sprint_oid(value, vlen, buf.as_mut_ptr(), buf.len()); ksmbd_debug(AUTH, b"Unexpected OID: %s\0".as_ptr(), buf.as_ptr()); return -EBADMSG; };
    (*conn).auth_mechs |= mech_type;
    if (*conn).preferred_auth_mech == 0 { (*conn).preferred_auth_mech = mech_type; }
    0
}

unsafe fn ksmbd_neg_token_alloc(context: *mut core::ffi::c_void, hdrlen: usize, tag: u8, value: *const core::ffi::c_void, vlen: usize) -> i32 {
    let conn = context as *mut ksmbd_conn;
    if vlen == 0 { return -EINVAL; }
    (*conn).mechToken = kmemdup_nul(value, vlen, KSMBD_DEFAULT_GFP);
    if (*conn).mechToken.is_null() { return -ENOMEM; }
    (*conn).mechTokenLen = vlen as u32; 0
}

pub unsafe fn ksmbd_neg_token_init_mech_token(context: *mut core::ffi::c_void, hdrlen: usize, tag: u8, value: *const core::ffi::c_void, vlen: usize) -> i32 { ksmbd_neg_token_alloc(context, hdrlen, tag, value, vlen) }

pub unsafe fn ksmbd_neg_token_targ_resp_token(context: *mut core::ffi::c_void, hdrlen: usize, tag: u8, value: *const core::ffi::c_void, vlen: usize) -> i32 { ksmbd_neg_token_alloc(context, hdrlen, tag, value, vlen) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
