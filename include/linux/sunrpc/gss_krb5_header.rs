/*
 *  Adapted from MIT Kerberos 5-1.2.1 lib/include/krb5.h,
 *  lib/gssapi/krb5/gssapiP_krb5.h, and others
 *
 *  Copyright (c) 2000-2008 The Regents of the University of Michigan.
 *  All rights reserved.
 *
 *  Andy Adamson   <andros@umich.edu>
 *  Bruce Fields   <bfields@fields.org>
 */

/*
 * Copyright 1995 by the Massachusetts Institute of Technology.
 * All Rights Reserved.
 *
 * Export of this software from the United States of America may
 *   require a specific license from the United States Government.
 *   It is the responsibility of any person or organization contemplating
 *   export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of M.I.T. not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 * M.I.T. makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 *
 */

// Dependencies supplied by the corresponding Linux SunRPC headers:
// <linux/sunrpc/auth_gss.h>
// <linux/sunrpc/gss_err.h>

/* Maximum key length (in bytes) for the supported crypto algorithms */
pub const GSS_KRB5_MAX_KEYLEN: usize = 32;

/* Maximum checksum function output for the supported enctypes */
pub const GSS_KRB5_MAX_CKSUM_LEN: usize = 24;

/* Maximum blocksize for the supported crypto algorithms */
pub const GSS_KRB5_MAX_BLOCKSIZE: usize = 16;

/* The length of the Kerberos GSS token header */
pub const GSS_KRB5_TOK_HDR_LEN: usize = 16;

pub const KG2_TOK_MIC: u16 = 0x0404;
pub const KG2_TOK_WRAP: u16 = 0x0504;

pub const KG2_TOKEN_FLAG_SENTBYACCEPTOR: u8 = 0x01;
pub const KG2_TOKEN_FLAG_SEALED: u8 = 0x02;
pub const KG2_TOKEN_FLAG_ACCEPTORSUBKEY: u8 = 0x04;

/* from rfc4121 */
pub const KG_USAGE_ACCEPTOR_SEAL: u32 = 22;
pub const KG_USAGE_ACCEPTOR_SIGN: u32 = 23;
pub const KG_USAGE_INITIATOR_SEAL: u32 = 24;
pub const KG_USAGE_INITIATOR_SIGN: u32 = 25;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
