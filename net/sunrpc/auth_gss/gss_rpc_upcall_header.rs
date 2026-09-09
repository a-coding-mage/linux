/* SPDX-License-Identifier: GPL-2.0+ */
/*
 *  linux/net/sunrpc/gss_rpc_upcall.h
 *
 *  Copyright (C) 2012 Simo Sorce <simo@redhat.com>
 */

// Dependencies supplied by the corresponding kernel/Rust translation units:
// linux/sunrpc/gss_api.h, linux/sunrpc/auth_gss.h, gss_rpc_xdr.h, and ../netns.h

#[repr(C)]
pub struct gssp_upcall_data {
    pub in_handle: xdr_netobj,
    pub in_token: gssp_in_token,
    pub out_handle: xdr_netobj,
    pub out_token: xdr_netobj,
    pub mech_oid: rpcsec_gss_oid,
    pub creds: svc_cred,
    pub found_creds: i32,
    pub major_status: i32,
    pub minor_status: i32,
}

extern "C" {
    pub fn gssp_accept_sec_context_upcall(
        net: *mut net,
        data: *mut gssp_upcall_data,
    ) -> i32;
    pub fn gssp_free_upcall_data(data: *mut gssp_upcall_data);

    pub fn set_gssp_clnt(net: *mut net) -> i32;
    pub fn clear_gssp_clnt(sunrpc_net: *mut sunrpc_net);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
