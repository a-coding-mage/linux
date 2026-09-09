/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * GSS Proxy upcall module
 *
 *  Copyright (C) 2012 Simo Sorce <simo@redhat.com>
 */

// Dependencies supplied by the surrounding kernel/RPC translation.
// The CONFIG_SUNRPC_DEBUG conditional controls RPCDBG_FACILITY when enabled.

pub const LUCID_OPTION: &str = "exported_context_type";
pub const LUCID_VALUE: &str = "linux_lucid_v1";
pub const CREDS_OPTION: &str = "exported_creds_type";
pub const CREDS_VALUE: &str = "linux_creds_v1";

pub type gssx_buffer = xdr_netobj;
pub type utf8string = xdr_netobj;
pub type gssx_OID = xdr_netobj;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum gssx_cred_usage {
    GSSX_C_INITIATE = 1,
    GSSX_C_ACCEPT = 2,
    GSSX_C_BOTH = 3,
}

#[repr(C)]
pub struct gssx_option {
    pub option: gssx_buffer,
    pub value: gssx_buffer,
}

#[repr(C)]
pub struct gssx_option_array {
    pub count: u32,
    pub data: *mut gssx_option,
}

#[repr(C)]
pub struct gssx_status {
    pub major_status: u64,
    pub mech: gssx_OID,
    pub minor_status: u64,
    pub major_status_string: utf8string,
    pub minor_status_string: utf8string,
    pub server_ctx: gssx_buffer,
    pub options: gssx_option_array,
}

#[repr(C)]
pub struct gssx_call_ctx {
    pub locale: utf8string,
    pub server_ctx: gssx_buffer,
    pub options: gssx_option_array,
}

#[repr(C)]
pub struct gssx_name_attr {
    pub attr: gssx_buffer,
    pub value: gssx_buffer,
    pub extensions: gssx_option_array,
}

#[repr(C)]
pub struct gssx_name_attr_array {
    pub count: u32,
    pub data: *mut gssx_name_attr,
}

#[repr(C)]
pub struct gssx_name {
    pub display_name: gssx_buffer,
}
pub type gssx_name_alias = gssx_name;

#[repr(C)]
pub struct gssx_cred_element {
    pub MN: gssx_name,
    pub mech: gssx_OID,
    pub cred_usage: u32,
    pub initiator_time_rec: u64,
    pub acceptor_time_rec: u64,
    pub options: gssx_option_array,
}

#[repr(C)]
pub struct gssx_cred_element_array {
    pub count: u32,
    pub data: *mut gssx_cred_element,
}

#[repr(C)]
pub struct gssx_cred {
    pub desired_name: gssx_name,
    pub elements: gssx_cred_element_array,
    pub cred_handle_reference: gssx_buffer,
    pub needs_release: u32,
}

#[repr(C)]
pub struct gssx_ctx {
    pub exported_context_token: gssx_buffer,
    pub state: gssx_buffer,
    pub need_release: u32,
    pub mech: gssx_OID,
    pub src_name: gssx_name,
    pub targ_name: gssx_name,
    pub lifetime: u64,
    pub ctx_flags: u64,
    pub locally_initiated: u32,
    pub open: u32,
    pub options: gssx_option_array,
}

#[repr(C)]
pub struct gssx_cb {
    pub initiator_addrtype: u64,
    pub initiator_address: gssx_buffer,
    pub acceptor_addrtype: u64,
    pub acceptor_address: gssx_buffer,
    pub application_data: gssx_buffer,
}

/* This structure is not defined in the protocol.
 * It is used in the kernel to carry around a big buffer
 * as a set of pages */
#[repr(C)]
pub struct gssp_in_token {
    pub pages: *mut *mut page, /* Array of contiguous pages */
    pub page_base: c_uint,     /* Start of page data */
    pub page_len: c_uint,      /* Length of page data */
}

#[repr(C)]
pub struct gssx_arg_accept_sec_context {
    pub call_ctx: gssx_call_ctx,
    pub context_handle: *mut gssx_ctx,
    pub cred_handle: *mut gssx_cred,
    pub input_token: gssp_in_token,
    pub input_cb: *mut gssx_cb,
    pub ret_deleg_cred: u32,
    pub options: gssx_option_array,
    pub pages: *mut *mut page,
    pub npages: c_uint,
}

#[repr(C)]
pub struct gssx_res_accept_sec_context {
    pub status: gssx_status,
    pub context_handle: *mut gssx_ctx,
    pub output_token: *mut gssx_buffer,
    /* struct gssx_cred *delegated_cred_handle; not used in kernel */
    pub options: gssx_option_array,
}

// Non-implemented XDR calls are represented by null function pointers.
pub const gssx_enc_indicate_mechs: usize = 0;
pub const gssx_dec_indicate_mechs: usize = 0;
pub const gssx_enc_get_call_context: usize = 0;
pub const gssx_dec_get_call_context: usize = 0;
pub const gssx_enc_import_and_canon_name: usize = 0;
pub const gssx_dec_import_and_canon_name: usize = 0;
pub const gssx_enc_export_cred: usize = 0;
pub const gssx_dec_export_cred: usize = 0;
pub const gssx_enc_import_cred: usize = 0;
pub const gssx_dec_import_cred: usize = 0;
pub const gssx_enc_acquire_cred: usize = 0;
pub const gssx_dec_acquire_cred: usize = 0;
pub const gssx_enc_store_cred: usize = 0;
pub const gssx_dec_store_cred: usize = 0;
pub const gssx_enc_init_sec_context: usize = 0;
pub const gssx_dec_init_sec_context: usize = 0;

extern "C" {
    pub fn gssx_enc_accept_sec_context(req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const c_void);
    pub fn gssx_dec_accept_sec_context(rqstp: *mut rpc_rqst, xdr: *mut xdr_stream, data: *mut c_void) -> c_int;
}

pub const gssx_enc_release_handle: usize = 0;
pub const gssx_dec_release_handle: usize = 0;
pub const gssx_enc_get_mic: usize = 0;
pub const gssx_dec_get_mic: usize = 0;
pub const gssx_enc_verify: usize = 0;
pub const gssx_dec_verify: usize = 0;
pub const gssx_enc_wrap: usize = 0;
pub const gssx_dec_wrap: usize = 0;
pub const gssx_enc_unwrap: usize = 0;
pub const gssx_dec_unwrap: usize = 0;
pub const gssx_enc_wrap_size_limit: usize = 0;
pub const gssx_dec_wrap_size_limit: usize = 0;

/* non implemented calls are set to 0 size */
pub const GSSX_ARG_indicate_mechs_sz: usize = 0;
pub const GSSX_RES_indicate_mechs_sz: usize = 0;
pub const GSSX_ARG_get_call_context_sz: usize = 0;
pub const GSSX_RES_get_call_context_sz: usize = 0;
pub const GSSX_ARG_import_and_canon_name_sz: usize = 0;
pub const GSSX_RES_import_and_canon_name_sz: usize = 0;
pub const GSSX_ARG_export_cred_sz: usize = 0;
pub const GSSX_RES_export_cred_sz: usize = 0;
pub const GSSX_ARG_import_cred_sz: usize = 0;
pub const GSSX_RES_import_cred_sz: usize = 0;
pub const GSSX_ARG_acquire_cred_sz: usize = 0;
pub const GSSX_RES_acquire_cred_sz: usize = 0;
pub const GSSX_ARG_store_cred_sz: usize = 0;
pub const GSSX_RES_store_cred_sz: usize = 0;
pub const GSSX_ARG_init_sec_context_sz: usize = 0;
pub const GSSX_RES_init_sec_context_sz: usize = 0;

const fn c_size(s: &str) -> usize { s.len() + 1 }
pub const GSSX_default_in_call_ctx_sz: usize = 4 + 4 + 4 + 8 + c_size(LUCID_OPTION) + c_size(LUCID_VALUE) + 8 + c_size(CREDS_OPTION) + c_size(CREDS_VALUE);
pub const GSSX_default_in_ctx_hndl_sz: usize = 4 + 4 + 8 + 4 + 4 + 6 * 4 + 6 * 4 + 8 + 8 + 4 + 4 + 4;
pub const GSSX_default_in_cred_sz: usize = 4;
pub const GSSX_default_in_token_sz: usize = 4;
pub const GSSX_default_in_cb_sz: usize = 4;
pub const GSSX_ARG_accept_sec_context_sz: usize = GSSX_default_in_call_ctx_sz + GSSX_default_in_ctx_hndl_sz + GSSX_default_in_cred_sz + GSSX_default_in_token_sz + GSSX_default_in_cb_sz + 4 + 4;
pub const GSSX_default_status_sz: usize = 8 + 24 + 8 + 256 + 256 + 16 + 4;
pub const GSSX_max_output_handle_sz: usize = 128;
pub const GSSX_max_oid_sz: usize = 16;
pub const GSSX_max_princ_sz: usize = 256;
pub const GSSX_default_ctx_sz: usize = GSSX_max_output_handle_sz + 16 + 4 + GSSX_max_oid_sz + 2 * GSSX_max_princ_sz + 8 + 8 + 4 + 4 + 4;
pub const GSSX_max_output_token_sz: usize = 1024;
/* grouplist not included; we allocate separate pages for that: */
pub const GSSX_max_creds_sz: usize = 4 + 4 + 4;
pub const GSSX_RES_accept_sec_context_sz: usize = GSSX_default_status_sz + GSSX_default_ctx_sz + GSSX_max_output_token_sz + 4 + GSSX_max_creds_sz;
pub const GSSX_ARG_release_handle_sz: usize = 0;
pub const GSSX_RES_release_handle_sz: usize = 0;
pub const GSSX_ARG_get_mic_sz: usize = 0;
pub const GSSX_RES_get_mic_sz: usize = 0;
pub const GSSX_ARG_verify_sz: usize = 0;
pub const GSSX_RES_verify_sz: usize = 0;
pub const GSSX_ARG_wrap_sz: usize = 0;
pub const GSSX_RES_wrap_sz: usize = 0;
pub const GSSX_ARG_unwrap_sz: usize = 0;
pub const GSSX_RES_unwrap_sz: usize = 0;
pub const GSSX_ARG_wrap_size_limit_sz: usize = 0;
pub const GSSX_RES_wrap_size_limit_sz: usize = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
