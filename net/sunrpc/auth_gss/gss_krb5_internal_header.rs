/* SPDX-License-Identifier: GPL-2.0 or BSD-3-Clause */
/*
 * SunRPC GSS Kerberos 5 mechanism internal definitions
 *
 * Copyright (c) 2022 Oracle and/or its affiliates.
 */

/* Dependency: crypto/krb5.h */

/* krb5_ctx flags definitions */
pub const KRB5_CTX_FLAG_INITIATOR: u32 = 0x00000001;
pub const KRB5_CTX_FLAG_ACCEPTOR_SUBKEY: u32 = 0x00000004;

#[repr(C)]
pub struct krb5_ctx {
    pub initiate: i32, /* 1 = initiating, 0 = accepting */
    pub enctype: u32,
    pub flags: u32,
    pub krb5e: *const krb5_enctype, /* crypto/krb5 enctype */
    pub initiator_enc_aead: *mut crypto_aead,
    pub acceptor_enc_aead: *mut crypto_aead,
    pub initiator_sign_shash: *mut crypto_shash,
    pub acceptor_sign_shash: *mut crypto_shash,
    pub Ksess: [u8; GSS_KRB5_MAX_KEYLEN], /* session key */
    pub seq_send64: atomic64_t,
    pub endtime: time64_t,
    pub mech_used: xdr_netobj,
}

/*
 * GSS Kerberos 5 mechanism Per-Message calls.
 */

extern "C" {
    pub fn gss_krb5_get_mic_v2(
        ctx: *mut krb5_ctx,
        text: *mut xdr_buf,
        token: *mut xdr_netobj,
    ) -> u32;

    pub fn gss_krb5_verify_mic_v2(
        ctx: *mut krb5_ctx,
        message_buffer: *mut xdr_buf,
        read_token: *mut xdr_netobj,
    ) -> u32;

    pub fn gss_krb5_wrap_v2(
        kctx: *mut krb5_ctx,
        offset: i32,
        buf: *mut xdr_buf,
        pages: *mut *mut page,
    ) -> u32;

    pub fn gss_krb5_unwrap_v2(
        kctx: *mut krb5_ctx,
        offset: i32,
        len: i32,
        buf: *mut xdr_buf,
        slack: *mut u32,
        align: *mut u32,
    ) -> u32;

    /* Implementation internal functions */

    pub fn xdr_extend_head(
        buf: *mut xdr_buf,
        base: u32,
        shiftlen: u32,
    ) -> i32;

    pub fn gss_krb5_errno_to_status(err: i32) -> u32;

    pub fn gss_krb5_mic_build_sg(
        body: *const xdr_buf,
        cksum: *mut core::ffi::c_void,
        cksum_len: u32,
        hdr: *mut core::ffi::c_void,
        sg_head: *mut scatterlist,
        sg_overflow: *mut *mut scatterlist,
    ) -> i32;

    pub fn gss_krb5_aead_encrypt(
        kctx: *mut krb5_ctx,
        offset: u32,
        buf: *mut xdr_buf,
        pages: *mut *mut page,
    ) -> u32;

    pub fn gss_krb5_aead_decrypt(
        kctx: *mut krb5_ctx,
        offset: u32,
        len: u32,
        buf: *mut xdr_buf,
        headskip: *mut u32,
        tailskip: *mut u32,
    ) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
