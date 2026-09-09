/*
 *  linux/net/sunrpc/gss_krb5_unseal.c
 *
 *  Adapted from MIT Kerberos 5-1.2.1 lib/gssapi/krb5/k5unseal.c
 *
 *  Copyright (c) 2000-2008 The Regents of the University of Michigan.
 *  All rights reserved.
 *
 *  Andy Adamson   <andros@umich.edu>
 */

/*
 * Copyright 1993 by OpenVision Technologies, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of OpenVision not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. OpenVision makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * OPENVISION DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL OPENVISION BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */

/*
 * Copyright (C) 1998 by the FundsXpress, INC.
 *
 * All rights reserved.
 *
 * Export of this software from the United States of America may require
 * a specific license from the United States Government.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted.
 */

// Dependencies supplied by the surrounding kernel/RPC implementation.

#[allow(clippy::missing_safety_doc)]
pub unsafe fn gss_krb5_verify_mic_v2(
    ctx: *mut krb5_ctx,
    message_buffer: *mut xdr_buf,
    read_token: *mut xdr_netobj,
) -> u32 {
    let krb5 = (*ctx).krb5e;
    let shash = if (*ctx).initiate {
        (*ctx).acceptor_sign_shash
    } else {
        (*ctx).initiator_sign_shash
    };
    let cksum_len = (*krb5).cksum_len;
    let mut sg_head: [scatterlist; XDR_BUF_TO_SG_NENTS] = core::mem::zeroed();
    let mut sg_overflow: *mut scatterlist = core::ptr::null_mut();
    let mut mic_offset: usize;
    let mut mic_len: usize;
    let ptr = (*read_token).data;
    let mut be16_ptr: __be16 = 0;
    let now: time64_t;
    let flags: u8;
    let nsg: i32;
    let ret: i32;

    dprintk!("RPC:       gss_krb5_verify_mic_v2\n");

    if (*read_token).len < GSS_KRB5_TOK_HDR_LEN + cksum_len {
        return GSS_S_DEFECTIVE_TOKEN;
    }

    core::ptr::copy_nonoverlapping(ptr as *const u8, &mut be16_ptr as *mut __be16 as *mut u8, 2);
    if be16_to_cpu(be16_ptr) != KG2_TOK_MIC {
        return GSS_S_DEFECTIVE_TOKEN;
    }

    flags = *ptr.add(2);
    if ((!(*ctx).initiate && (flags & KG2_TOKEN_FLAG_SENTBYACCEPTOR) != 0)
        || ((*ctx).initiate && (flags & KG2_TOKEN_FLAG_SENTBYACCEPTOR) == 0))
    {
        return GSS_S_BAD_SIG;
    }

    if (flags & KG2_TOKEN_FLAG_SEALED) != 0 {
        dprintk!("gss_krb5_verify_mic_v2: token has unexpected sealed flag\n");
        return GSS_S_FAILURE;
    }

    let mut i = 3;
    while i < 8 {
        if *ptr.add(i) != 0xff {
            return GSS_S_DEFECTIVE_TOKEN;
        }
        i += 1;
    }

    nsg = gss_krb5_mic_build_sg(
        message_buffer,
        ptr.add(GSS_KRB5_TOK_HDR_LEN),
        cksum_len,
        ptr,
        sg_head.as_mut_ptr(),
        &mut sg_overflow,
    );
    if nsg < 0 {
        return GSS_S_FAILURE;
    }

    mic_offset = 0;
    mic_len = cksum_len + (*message_buffer).len + GSS_KRB5_TOK_HDR_LEN;
    ret = crypto_krb5_verify_mic(
        krb5,
        shash,
        core::ptr::null_mut(),
        sg_head.as_mut_ptr(),
        nsg,
        &mut mic_offset,
        &mut mic_len,
    );
    kfree(sg_overflow as *mut core::ffi::c_void);
    if ret != 0 {
        return gss_krb5_errno_to_status(ret);
    }

    // It got through unscathed. Make sure the context is unexpired.
    now = ktime_get_real_seconds();
    if now > (*ctx).endtime {
        return GSS_S_CONTEXT_EXPIRED;
    }

    /*
     * NOTE: the sequence number at ptr + 8 is skipped, rpcsec_gss
     * doesn't want it checked; see page 6 of rfc 2203.
     */

    GSS_S_COMPLETE
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
