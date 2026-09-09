/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/linux/sunrpc/gss_api.h
 *
 * Somewhat simplified version of the gss api.
 *
 * Dug Song <dugsong@monkey.org>
 * Andy Adamson <andros@umich.edu>
 * Bruce Fields <bfields@umich.edu>
 * Copyright (c) 2000 The Regents of the University of Michigan
 */

// C dependencies: linux/sunrpc/xdr.h, linux/sunrpc/msg_prot.h, linux/uio.h

/* The mechanism-independent gss-api context: */
#[repr(C)]
pub struct gss_ctx {
    pub mech_type: *mut gss_api_mech,
    pub internal_ctx_id: *mut core::ffi::c_void,
    pub slack: u32,
    pub align: u32,
}

pub const GSS_C_NO_BUFFER: xdr_netobj = unsafe { core::mem::zeroed() };
pub const GSS_C_NO_CONTEXT: *mut gss_ctx = core::ptr::null_mut();
pub const GSS_C_QOP_DEFAULT: u32 = 0;

/*XXX  arbitrary length - is this set somewhere? */
pub const GSS_OID_MAX_LEN: usize = 32;

#[repr(C)]
pub struct rpcsec_gss_oid {
    pub len: u32,
    pub data: [u8; GSS_OID_MAX_LEN],
}

/* From RFC 3530 */
#[repr(C)]
pub struct rpcsec_gss_info {
    pub oid: rpcsec_gss_oid,
    pub qop: u32,
    pub service: u32,
}

/* gss-api prototypes; note that these are somewhat simplified versions of
 * the prototypes specified in RFC 2744. */
extern "C" {
    pub fn gss_import_sec_context(
        input_token: *const core::ffi::c_void,
        bufsize: usize,
        mech: *mut gss_api_mech,
        ctx_id: *mut *mut gss_ctx,
        endtime: *mut time64_t,
        gfp_mask: gfp_t,
    ) -> i32;
    pub fn gss_get_mic(
        ctx_id: *mut gss_ctx,
        message: *mut xdr_buf,
        mic_token: *mut xdr_netobj,
    ) -> u32;
    pub fn gss_verify_mic(
        ctx_id: *mut gss_ctx,
        message: *mut xdr_buf,
        mic_token: *mut xdr_netobj,
    ) -> u32;
    pub fn gss_wrap(
        ctx_id: *mut gss_ctx,
        offset: i32,
        outbuf: *mut xdr_buf,
        inpages: *mut *mut page,
    ) -> u32;
    pub fn gss_unwrap(
        ctx_id: *mut gss_ctx,
        offset: i32,
        len: i32,
        inbuf: *mut xdr_buf,
    ) -> u32;
    pub fn gss_delete_sec_context(ctx_id: *mut *mut gss_ctx) -> u32;

    pub fn gss_svc_to_pseudoflavor(mech: *mut gss_api_mech, qop: u32, service: u32) -> rpc_authflavor_t;
    pub fn gss_pseudoflavor_to_service(mech: *mut gss_api_mech, pseudoflavor: u32) -> u32;
    pub fn gss_pseudoflavor_to_datatouch(mech: *mut gss_api_mech, pseudoflavor: u32) -> bool;
    pub fn gss_service_to_auth_domain_name(mech: *mut gss_api_mech, service: u32) -> *mut core::ffi::c_char;
}

#[repr(C)]
pub struct pf_desc {
    pub pseudoflavor: u32,
    pub qop: u32,
    pub service: u32,
    pub name: *mut core::ffi::c_char,
    pub auth_domain_name: *mut core::ffi::c_char,
    pub domain: *mut auth_domain,
    pub datatouch: bool,
}

/* Different mechanisms (e.g., krb5 or spkm3) may implement gss-api, and
 * mechanisms may be dynamically registered or unregistered by modules. */

/* Each mechanism is described by the following struct: */
#[repr(C)]
pub struct gss_api_mech {
    pub gm_list: list_head,
    pub gm_owner: *mut module,
    pub gm_oid: rpcsec_gss_oid,
    pub gm_name: *mut core::ffi::c_char,
    pub gm_ops: *const gss_api_ops,
    /* pseudoflavors supported by this mechanism: */
    pub gm_pf_num: i32,
    pub gm_pfs: *mut pf_desc,
    /* Should the following be a callback operation instead? */
    pub gm_upcall_enctypes: *const core::ffi::c_char,
}

/* and must provide the following operations: */
#[repr(C)]
pub struct gss_api_ops {
    pub gss_import_sec_context: Option<unsafe extern "C" fn(
        input_token: *const core::ffi::c_void,
        bufsize: usize,
        ctx_id: *mut gss_ctx,
        endtime: *mut time64_t,
        gfp_mask: gfp_t,
    ) -> i32>,
    pub gss_get_mic: Option<unsafe extern "C" fn(*mut gss_ctx, *mut xdr_buf, *mut xdr_netobj) -> u32>,
    pub gss_verify_mic: Option<unsafe extern "C" fn(*mut gss_ctx, *mut xdr_buf, *mut xdr_netobj) -> u32>,
    pub gss_wrap: Option<unsafe extern "C" fn(*mut gss_ctx, i32, *mut xdr_buf, *mut *mut page) -> u32>,
    pub gss_unwrap: Option<unsafe extern "C" fn(*mut gss_ctx, i32, i32, *mut xdr_buf) -> u32>,
    pub gss_delete_sec_context: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
}

extern "C" {
    pub fn gss_mech_register(mech: *mut gss_api_mech) -> i32;
    pub fn gss_mech_unregister(mech: *mut gss_api_mech);

    /* returns a mechanism descriptor given an OID, and increments the mechanism's
     * reference count. */
    pub fn gss_mech_get_by_OID(oid: *mut rpcsec_gss_oid) -> *mut gss_api_mech;

    /* Given a GSS security tuple, look up a pseudoflavor */
    pub fn gss_mech_info2flavor(info: *mut rpcsec_gss_info) -> rpc_authflavor_t;

    /* Given a pseudoflavor, look up a GSS security tuple */
    pub fn gss_mech_flavor2info(pseudoflavor: rpc_authflavor_t, info: *mut rpcsec_gss_info) -> i32;

    /* Returns a reference to a mechanism, given a name like "krb5" etc. */
    pub fn gss_mech_get_by_name(name: *const core::ffi::c_char) -> *mut gss_api_mech;

    /* Similar, but get by pseudoflavor. */
    pub fn gss_mech_get_by_pseudoflavor(pseudoflavor: u32) -> *mut gss_api_mech;

    pub fn gss_mech_get(mech: *mut gss_api_mech) -> *mut gss_api_mech;

    /* For every successful gss_mech_get or gss_mech_get_by_* call there must be a
     * corresponding call to gss_mech_put. */
    pub fn gss_mech_put(mech: *mut gss_api_mech);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
