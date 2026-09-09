/*
 *  linux/net/sunrpc/gss_krb5_seal.c
 *
 *  Adapted from MIT Kerberos 5-1.2.1 lib/gssapi/krb5/k5seal.c
 *
 *  Copyright (c) 2000-2008 The Regents of the University of Michigan.
 *  All rights reserved.
 *
 *  Andy Adamson <andros@umich.edu>
 *  J. Bruce Fields <bfields@redhat.com>
 */

/* The following license and copyright notices are preserved from the C source. */
/*
 * Copyright 1993 by OpenVision Technologies, Inc.
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of OpenVision not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. OpenVision makes no
 * representations about the suitability of this software for any purpose.
 * It is provided "as is" without express or implied warranty.
 */
/* Copyright (C) 1998 by the FundsXpress, INC. All rights reserved. */

/* C dependencies supplied by the surrounding kernel/Rust translation. */

unsafe fn setup_token_v2(ctx: *mut krb5_ctx, token: *mut xdr_netobj) -> *mut core::ffi::c_void {
    let mut ptr: *mut u16;
    let krb5_hdr: *mut core::ffi::c_void;
    let mut p: *mut u8;
    let mut flags: u8 = 0x00;

    if !(*ctx).initiate {
        flags |= KG2_TOKEN_FLAG_SENTBYACCEPTOR;
    }
    if (*ctx).flags & KRB5_CTX_FLAG_ACCEPTOR_SUBKEY != 0 {
        flags |= KG2_TOKEN_FLAG_ACCEPTORSUBKEY;
    }

    /* Per rfc 4121, sec 4.2.6.1, there is no header,
     * just start the token.
     */
    krb5_hdr = (*token).data as *mut core::ffi::c_void;
    ptr = krb5_hdr as *mut u16;

    *ptr = KG2_TOK_MIC;
    ptr = ptr.add(1);
    p = ptr as *mut u8;
    *p = flags;
    p = p.add(1);
    *p = 0xff;
    p = p.add(1);
    ptr = p as *mut u16;
    *ptr = 0xffff;
    ptr = ptr.add(1);
    *ptr = 0xffff;

    (*token).len = GSS_KRB5_TOK_HDR_LEN + (*ctx).krb5e.as_ref().unwrap().cksum_len;
    krb5_hdr
}

pub unsafe fn gss_krb5_get_mic_v2(
    ctx: *mut krb5_ctx,
    text: *mut xdr_buf,
    token: *mut xdr_netobj,
) -> u32 {
    let krb5: *const krb5_enctype = (*ctx).krb5e;
    let shash: *mut crypto_shash = if (*ctx).initiate {
        (*ctx).initiator_sign_shash
    } else {
        (*ctx).acceptor_sign_shash
    };
    let cksum_len: usize = (*krb5).cksum_len;
    let mut sg_head: [scatterlist; XDR_BUF_TO_SG_NENTS] = core::mem::zeroed();
    let mut sg_overflow: *mut scatterlist = core::ptr::null_mut();
    let mut seq_send_be64: u64;
    let krb5_hdr: *mut core::ffi::c_void;
    let now: time64_t;
    let ret: isize;
    let nsg: i32;

    dprintk!("RPC:       %s\\n", "gss_krb5_get_mic_v2");

    krb5_hdr = setup_token_v2(ctx, token);

    /* Set up the sequence number. Now 64-bits in clear
     * text and w/o direction indicator */
    seq_send_be64 = cpu_to_be64((*ctx).seq_send64.fetch_add(1, core::sync::atomic::Ordering::SeqCst));
    core::ptr::copy_nonoverlapping(
        &seq_send_be64 as *const u64 as *const u8,
        (krb5_hdr as *mut u8).add(8),
        8,
    );

    /*
     * The checksum is written directly into the token buffer.
     * This is safe: crypto_krb5_get_mic uses shash (software
     * hash), so the scatterlist is never DMA-mapped.
     */
    nsg = gss_krb5_mic_build_sg(
        text,
        (krb5_hdr as *mut u8).add(GSS_KRB5_TOK_HDR_LEN) as *mut core::ffi::c_void,
        cksum_len,
        krb5_hdr,
        sg_head.as_mut_ptr(),
        &mut sg_overflow,
    );
    if nsg < 0 {
        return GSS_S_FAILURE;
    }

    ret = crypto_krb5_get_mic(
        krb5,
        shash,
        core::ptr::null_mut(),
        sg_head.as_mut_ptr(),
        nsg,
        cksum_len + (*text).len + GSS_KRB5_TOK_HDR_LEN,
        cksum_len,
        (*text).len + GSS_KRB5_TOK_HDR_LEN,
    );
    kfree(sg_overflow);
    if ret < 0 {
        return GSS_S_FAILURE;
    }

    now = ktime_get_real_seconds();
    if (*ctx).endtime < now {
        GSS_S_CONTEXT_EXPIRED
    } else {
        GSS_S_COMPLETE
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
