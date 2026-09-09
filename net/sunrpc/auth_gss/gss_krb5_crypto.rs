/*
 *  linux/net/sunrpc/gss_krb5_crypto.c
 *
 *  Copyright (c) 2000-2008 The Regents of the University of Michigan.
 *  All rights reserved.
 *
 *  Andy Adamson   <andros@umich.edu>
 *  Bruce Fields   <bfields@redhat.com>
 */

/*
 * Copyright (C) 1998 by the FundsXpress, INC.
 *
 * All rights reserved.
 *
 * Export of this software from the United States of America may require
 * a specific license from the United States Government.  It is the
 * responsibility of any person or organization contemplating export to
 * obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of FundsXpress. not be used in advertising or publicity
 * pertaining to distribution of the software without specific, written
 * prior permission.  FundsXpress makes no representations about the
 * suitability of this software for any purpose.  It is provided "as is"
 * without express or implied warranty.
 *
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND WITHOUT ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED
 * WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
 */

// Linux headers and gss_krb5_internal.h provide the declarations used below.

#[cfg(CONFIG_SUNRPC_DEBUG)]
// RPCDBG_FACILITY is RPCDBG_AUTH when CONFIG_SUNRPC_DEBUG is enabled.

/*
 * This function makes the assumption that it was ultimately called
 * from gss_wrap().
 *
 * The client auth_gss code moves any existing tail data into a
 * separate page before calling gss_wrap.
 * The server svcauth_gss code ensures that both the head and the
 * tail have slack space of RPC_MAX_AUTH_SIZE before calling gss_wrap.
 *
 * Even with that guarantee, this function may be called more than
 * once in the processing of gss_wrap().  The best we can do is
 * verify at compile-time (see GSS_KRB5_MAX_SLACK_NEEDED) that the
 * largest expected shift will fit within RPC_MAX_AUTH_SIZE.
 * At run-time we can verify that a single invocation of this
 * function doesn't attempt to use more the RPC_MAX_AUTH_SIZE.
 */
pub unsafe fn xdr_extend_head(
    buf: *mut xdr_buf,
    base: ::std::primitive::u32,
    shiftlen: ::std::primitive::u32,
) -> ::std::primitive::i32 {
    if shiftlen == 0 {
        return 0;
    }

    BUG_ON(shiftlen > RPC_MAX_AUTH_SIZE);

    let p = (*buf).head[0].iov_base.add(base as usize);
    ::std::ptr::copy(p, p.add(shiftlen as usize), (*buf).head[0].iov_len - base as usize);

    (*buf).head[0].iov_len += shiftlen as usize;
    (*buf).len += shiftlen as usize;

    0
}

/**
 * gss_krb5_aead_encrypt - Encrypt a wrap token using crypto/krb5
 * @kctx: Kerberos context
 * @offset: byte offset of the GSS token header in @buf
 * @buf: OUT: send buffer
 * @pages: plaintext payload pages (page cache data)
 *
 * The xdr_buf setup mirrors the original per-enctype encrypt
 * functions, but the CBC-CTS encryption and HMAC are replaced
 * by a single AEAD operation through the crypto/krb5 library.
 */
pub unsafe fn gss_krb5_aead_encrypt(
    kctx: *mut krb5_ctx,
    offset: u32,
    buf: *mut xdr_buf,
    pages: *mut *mut page,
) -> u32 {
    let krb5 = (*kctx).krb5e;
    let aead = if (*kctx).initiate { (*kctx).initiator_enc_aead } else { (*kctx).acceptor_enc_aead };
    let conflen = (*krb5).conf_len;
    let cksum_len = (*krb5).cksum_len;
    let mut sg: [scatterlist; XDR_BUF_TO_SG_NENTS as usize] = ::std::mem::zeroed();
    let mut sg_overflow: *mut scatterlist = ::std::ptr::null_mut();

    if xdr_extend_head(buf, offset + GSS_KRB5_TOK_HDR_LEN, conflen) != 0 { return GSS_S_FAILURE; }

    if (*buf).tail[0].iov_base.is_null() {
        (*buf).tail[0].iov_base = (*buf).head[0].iov_base.add((*buf).head[0].iov_len);
        (*buf).tail[0].iov_len = 0;
    }
    ::std::ptr::copy_nonoverlapping(
        (*buf).head[0].iov_base.add(offset as usize),
        (*buf).tail[0].iov_base.add((*buf).tail[0].iov_len),
        GSS_KRB5_TOK_HDR_LEN as usize,
    );
    (*buf).tail[0].iov_len += GSS_KRB5_TOK_HDR_LEN as usize;
    (*buf).len += GSS_KRB5_TOK_HDR_LEN as usize;
    (*buf).tail[0].iov_len += cksum_len as usize;
    (*buf).len += cksum_len as usize;

    if pages != (*buf).pages {
        let mut plen = (*buf).page_len;
        let mut i = ((*buf).page_base >> PAGE_SHIFT) as usize;
        let mut off = offset_in_page((*buf).page_base) as usize;
        while plen != 0 {
            let n = ::std::cmp::min(plen as usize, PAGE_SIZE as usize - off);
            memcpy_page((*buf).pages.add(i), off, pages.add(i), off, n);
            plen -= n as _; i += 1; off = 0;
        }
    }

    let sec_offset = offset + GSS_KRB5_TOK_HDR_LEN;
    let sec_len = (*buf).len - sec_offset;
    let data_len = sec_len - conflen - cksum_len;
    let nsg = xdr_buf_to_sg_alloc(buf, sec_offset, sec_len, sg.as_mut_ptr(), XDR_BUF_TO_SG_NENTS, &mut sg_overflow, GFP_NOFS);
    if nsg < 0 { return GSS_S_FAILURE; }
    let ret = crypto_krb5_encrypt(krb5, aead, sg.as_mut_ptr(), nsg, sec_len, conflen, data_len, false);
    kfree(sg_overflow as *mut ::std::ffi::c_void);
    if ret < 0 { return GSS_S_FAILURE; }
    GSS_S_COMPLETE
}

/** Decrypt a wrap token using crypto/krb5. */
pub unsafe fn gss_krb5_aead_decrypt(
    kctx: *mut krb5_ctx, offset: u32, len: u32, buf: *mut xdr_buf,
    headskip: *mut u32, tailskip: *mut u32,
) -> u32 {
    let krb5 = (*kctx).krb5e;
    let aead = if (*kctx).initiate { (*kctx).acceptor_enc_aead } else { (*kctx).initiator_enc_aead };
    let sec_offset = offset + GSS_KRB5_TOK_HDR_LEN;
    if len < sec_offset { return GSS_S_DEFECTIVE_TOKEN; }
    let sec_len = len - sec_offset;
    let mut sg: [scatterlist; XDR_BUF_TO_SG_NENTS as usize] = ::std::mem::zeroed();
    let mut sg_overflow: *mut scatterlist = ::std::ptr::null_mut();
    let nsg = xdr_buf_to_sg_alloc(buf, sec_offset, sec_len, sg.as_mut_ptr(), XDR_BUF_TO_SG_NENTS, &mut sg_overflow, GFP_NOFS);
    if nsg < 0 { return GSS_S_FAILURE; }
    let mut data_offset: usize = 0;
    let mut data_len: usize = sec_len as usize;
    let ret = crypto_krb5_decrypt(krb5, aead, sg.as_mut_ptr(), nsg, &mut data_offset, &mut data_len);
    kfree(sg_overflow as *mut ::std::ffi::c_void);
    if ret < 0 { return gss_krb5_errno_to_status(ret); }
    *headskip = data_offset as u32;
    *tailskip = sec_len - data_offset as u32 - data_len as u32;
    GSS_S_COMPLETE
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
