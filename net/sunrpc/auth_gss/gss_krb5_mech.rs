// SPDX-License-Identifier: BSD-3-Clause
/*
 *  linux/net/sunrpc/gss_krb5_mech.c
 *
 *  Copyright (c) 2001-2008 The Regents of the University of Michigan.
 *  All rights reserved.
 *
 *  Andy Adamson <andros@umich.edu>
 *  J. Bruce Fields <bfields@umich.edu>
 */

// Linux kernel includes and local headers are supplied by the surrounding crate.

#[cfg(IS_ENABLED_CONFIG_SUNRPC_DEBUG)]
const RPCDBG_FACILITY: u32 = RPCDBG_AUTH;

static mut GSS_KERBEROS_MECH: gss_api_mech = unsafe { core::mem::zeroed() };

/* Candidate enctypes in order of most preferred to least. */
static GSS_KRB5_ENCTYPES: [u32; 6] = [
    KRB5_ENCTYPE_AES256_CTS_HMAC_SHA384_192,
    KRB5_ENCTYPE_AES128_CTS_HMAC_SHA256_128,
    KRB5_ENCTYPE_CAMELLIA256_CTS_CMAC,
    KRB5_ENCTYPE_CAMELLIA128_CTS_CMAC,
    KRB5_ENCTYPE_AES256_CTS_HMAC_SHA1_96,
    KRB5_ENCTYPE_AES128_CTS_HMAC_SHA1_96,
];

static mut GSS_KRB5_ENCTYPE_PRIORITY_LIST: [u8; 64] = [0; 64];

unsafe fn gss_krb5_prepare_enctype_priority_list() {
    let mut total: usize = 0;
    let mut buf = [0u8; 16];
    let mut sep = "";
    GSS_KRB5_ENCTYPE_PRIORITY_LIST[0] = 0;
    for enctype in GSS_KRB5_ENCTYPES {
        if crypto_krb5_find_enctype(enctype).is_null() { continue; }
        let n = snprintf(buf.as_mut_ptr(), buf.len(), sep.as_ptr(), enctype);
        if n < 0 { break; }
        let n = n as usize;
        if total + n >= GSS_KRB5_ENCTYPE_PRIORITY_LIST.len() { break; }
        strcat(GSS_KRB5_ENCTYPE_PRIORITY_LIST.as_mut_ptr(), buf.as_ptr());
        sep = ",";
        total += n;
    }
}

unsafe fn gss_krb5_import_ctx_v2(ctx: *mut krb5_ctx, gfp_mask: gfp_t) -> c_int {
    let tk = krb5_buffer { len: (*(*ctx).krb5e).key_len, data: (*ctx).Ksess.as_mut_ptr() };
    (*ctx).initiator_enc_aead = crypto_krb5_prepare_encryption((*ctx).krb5e, &tk, KG_USAGE_INITIATOR_SEAL, gfp_mask);
    if IS_ERR((*ctx).initiator_enc_aead) { let ret = PTR_ERR((*ctx).initiator_enc_aead); goto out_free; }
    (*ctx).acceptor_enc_aead = crypto_krb5_prepare_encryption((*ctx).krb5e, &tk, KG_USAGE_ACCEPTOR_SEAL, gfp_mask);
    if IS_ERR((*ctx).acceptor_enc_aead) { let ret = PTR_ERR((*ctx).acceptor_enc_aead); goto out_free; }
    (*ctx).initiator_sign_shash = crypto_krb5_prepare_checksum((*ctx).krb5e, &tk, KG_USAGE_INITIATOR_SIGN, gfp_mask);
    if IS_ERR((*ctx).initiator_sign_shash) { let ret = PTR_ERR((*ctx).initiator_sign_shash); goto out_free; }
    (*ctx).acceptor_sign_shash = crypto_krb5_prepare_checksum((*ctx).krb5e, &tk, KG_USAGE_ACCEPTOR_SIGN, gfp_mask);
    if IS_ERR((*ctx).acceptor_sign_shash) { let ret = PTR_ERR((*ctx).acceptor_sign_shash); goto out_free; }
    return 0;
out_free:
    crypto_free_shash((*ctx).acceptor_sign_shash); crypto_free_shash((*ctx).initiator_sign_shash);
    crypto_free_aead((*ctx).acceptor_enc_aead); crypto_free_aead((*ctx).initiator_enc_aead);
    return ret;
}

unsafe fn gss_import_v2_context(mut p: *const c_void, end: *const c_void, ctx: *mut krb5_ctx, gfp_mask: gfp_t) -> c_int {
    let mut seq_send64 = 0u64; let mut keylen: c_int; let mut time32 = 0u32;
    p = simple_get_bytes(p, end, &mut (*ctx).flags as *mut _ as *mut c_void, core::mem::size_of_val(&(*ctx).flags)); if IS_ERR(p) { return PTR_ERR(p); }
    (*ctx).initiate = (*ctx).flags & KRB5_CTX_FLAG_INITIATOR;
    p = simple_get_bytes(p, end, &mut time32 as *mut _ as *mut c_void, 4); if IS_ERR(p) { return PTR_ERR(p); }
    (*ctx).endtime = time32 as time64_t;
    p = simple_get_bytes(p, end, &mut seq_send64 as *mut _ as *mut c_void, 8); if IS_ERR(p) { return PTR_ERR(p); }
    atomic64_set(&mut (*ctx).seq_send64, seq_send64);
    p = simple_get_bytes(p, end, &mut (*ctx).enctype as *mut _ as *mut c_void, 4); if IS_ERR(p) { return PTR_ERR(p); }
    (*ctx).krb5e = crypto_krb5_find_enctype((*ctx).enctype);
    if (*ctx).krb5e.is_null() { return -EINVAL; }
    keylen = (*(*ctx).krb5e).key_len;
    p = simple_get_bytes(p, end, (*ctx).Ksess.as_mut_ptr() as *mut c_void, keylen as usize); if IS_ERR(p) { return PTR_ERR(p); }
    if p != end { return -EINVAL; }
    (*ctx).mech_used.data = kmemdup((*GSS_KERBEROS_MECH).gm_oid.data, (*GSS_KERBEROS_MECH).gm_oid.len, gfp_mask);
    if (*ctx).mech_used.data.is_null() { return -ENOMEM; }
    (*ctx).mech_used.len = (*GSS_KERBEROS_MECH).gm_oid.len;
    let ret = gss_krb5_import_ctx_v2(ctx, gfp_mask); if ret != 0 { kfree((*ctx).mech_used.data); return ret; } 0
}

unsafe fn gss_krb5_import_sec_context(p: *const c_void, len: usize, ctx_id: *mut gss_ctx, endtime: *mut time64_t, gfp_mask: gfp_t) -> c_int {
    let end = (p as *const u8).add(len) as *const c_void;
    let ctx = kzalloc_obj::<krb5_ctx>(gfp_mask); if ctx.is_null() { return -ENOMEM; }
    let ret = gss_import_v2_context(p, end, ctx, gfp_mask); memzero_explicit((*ctx).Ksess.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&(*ctx).Ksess));
    if ret != 0 { kfree(ctx as *mut c_void); return ret; }
    (*ctx_id).internal_ctx_id = ctx as *mut c_void; if !endtime.is_null() { *endtime = (*ctx).endtime; } 0
}

unsafe fn gss_krb5_delete_sec_context(internal_ctx: *mut c_void) { let kctx = internal_ctx as *mut krb5_ctx; crypto_free_shash((*kctx).acceptor_sign_shash); crypto_free_shash((*kctx).initiator_sign_shash); crypto_free_aead((*kctx).acceptor_enc_aead); crypto_free_aead((*kctx).initiator_enc_aead); kfree((*kctx).mech_used.data); kfree(kctx as *mut c_void); }

pub unsafe fn gss_krb5_errno_to_status(err: c_int) -> u32 { match err { 0 => GSS_S_COMPLETE, -EBADMSG => GSS_S_BAD_SIG, -EPROTO => GSS_S_DEFECTIVE_TOKEN, _ => GSS_S_FAILURE } }

unsafe fn gss_krb5_get_mic(gctx: *mut gss_ctx, text: *mut xdr_buf, token: *mut xdr_netobj) -> u32 { gss_krb5_get_mic_v2((*gctx).internal_ctx_id as *mut krb5_ctx, text, token) }
unsafe fn gss_krb5_verify_mic(gctx: *mut gss_ctx, message_buffer: *mut xdr_buf, read_token: *mut xdr_netobj) -> u32 { gss_krb5_verify_mic_v2((*gctx).internal_ctx_id as *mut krb5_ctx, message_buffer, read_token) }
unsafe fn gss_krb5_wrap(gctx: *mut gss_ctx, offset: c_int, buf: *mut xdr_buf, pages: *mut *mut page) -> u32 { gss_krb5_wrap_v2((*gctx).internal_ctx_id as *mut krb5_ctx, offset, buf, pages) }
unsafe fn gss_krb5_unwrap(gctx: *mut gss_ctx, offset: c_int, len: c_int, buf: *mut xdr_buf) -> u32 { gss_krb5_unwrap_v2((*gctx).internal_ctx_id as *mut krb5_ctx, offset, len, buf, &mut (*gctx).slack, &mut (*gctx).align) }

static GSS_KERBEROS_OPS: gss_api_ops = gss_api_ops {
    gss_import_sec_context: Some(gss_krb5_import_sec_context),
    gss_get_mic: Some(gss_krb5_get_mic), gss_verify_mic: Some(gss_krb5_verify_mic),
    gss_wrap: Some(gss_krb5_wrap), gss_unwrap: Some(gss_krb5_unwrap),
    gss_delete_sec_context: Some(gss_krb5_delete_sec_context),
};

static mut GSS_KERBEROS_PFS: [pf_desc; 3] = [
    pf_desc { pseudoflavor: RPC_AUTH_GSS_KRB5, qop: GSS_C_QOP_DEFAULT, service: RPC_GSS_SVC_NONE, name: b"krb5\0".as_ptr() as *const c_char, datatouch: false },
    pf_desc { pseudoflavor: RPC_AUTH_GSS_KRB5I, qop: GSS_C_QOP_DEFAULT, service: RPC_GSS_SVC_INTEGRITY, name: b"krb5i\0".as_ptr() as *const c_char, datatouch: true },
    pf_desc { pseudoflavor: RPC_AUTH_GSS_KRB5P, qop: GSS_C_QOP_DEFAULT, service: RPC_GSS_SVC_PRIVACY, name: b"krb5p\0".as_ptr() as *const c_char, datatouch: true },
];

unsafe fn init_kerberos_module() -> c_int {
    gss_krb5_prepare_enctype_priority_list();
    let status = gss_mech_register(&mut GSS_KERBEROS_MECH);
    if status != 0 { printk(b"Failed to register kerberos gss mechanism!\n\0".as_ptr() as *const c_char); }
    status
}

unsafe fn cleanup_kerberos_module() { gss_mech_unregister(&mut GSS_KERBEROS_MECH); }

// MODULE_ALIAS("rpc-auth-gss-krb5");
// MODULE_ALIAS("rpc-auth-gss-krb5i");
// MODULE_ALIAS("rpc-auth-gss-krb5p");
// MODULE_ALIAS("rpc-auth-gss-390003");
// MODULE_ALIAS("rpc-auth-gss-390004");
// MODULE_ALIAS("rpc-auth-gss-390005");
// MODULE_ALIAS("rpc-auth-gss-1.2.840.113554.1.2.2");
// MODULE_DESCRIPTION("Sun RPC Kerberos 5 module");
// MODULE_LICENSE("GPL");
// module_init(init_kerberos_module);
// module_exit(cleanup_kerberos_module);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
