/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/linux/sunrpc/auth_gss.h
 *
 * Declarations for RPCSEC_GSS
 *
 * Dug Song <dugsong@monkey.org>
 * Andy Adamson <andros@umich.edu>
 * Bruce Fields <bfields@umich.edu>
 * Copyright (c) 2000 The Regents of the University of Michigan
 */

/* Dependencies supplied by the corresponding Linux RPC headers. */

pub const RPC_GSS_VERSION: u32 = 1;

pub const MAXSEQ: u32 = 0x8000_0000; /* maximum legal sequence number, from rfc 2203 */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpc_gss_proc {
    RPC_GSS_PROC_DATA = 0,
    RPC_GSS_PROC_INIT = 1,
    RPC_GSS_PROC_CONTINUE_INIT = 2,
    RPC_GSS_PROC_DESTROY = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpc_gss_svc {
    RPC_GSS_SVC_NONE = 1,
    RPC_GSS_SVC_INTEGRITY = 2,
    RPC_GSS_SVC_PRIVACY = 3,
}

/* on-the-wire gss cred: */
#[repr(C)]
pub struct rpc_gss_wire_cred {
    pub gc_v: u32,                 /* version */
    pub gc_proc: u32,              /* control procedure */
    pub gc_seq: u32,               /* sequence number */
    pub gc_svc: u32,               /* service */
    pub gc_ctx: xdr_netobj,        /* context handle */
}

/* on-the-wire gss verifier: */
#[repr(C)]
pub struct rpc_gss_wire_verf {
    pub gv_flavor: u32,
    pub gv_verf: xdr_netobj,
}

/* return from gss NULL PROC init sec context */
#[repr(C)]
pub struct rpc_gss_init_res {
    pub gr_ctx: xdr_netobj,        /* context handle */
    pub gr_major: u32,             /* major status */
    pub gr_minor: u32,             /* minor status */
    pub gr_win: u32,               /* sequence window */
    pub gr_token: xdr_netobj,      /* token */
}

/* The gss_cl_ctx struct holds all the information the rpcsec_gss client
 * code needs to know about a single security context.  In particular,
 * gc_gss_ctx is the context handle that is used to do gss-api calls, while
 * gc_wire_ctx is the context handle that is used to identify the context on
 * the wire when communicating with a server. */
#[repr(C)]
pub struct gss_cl_ctx {
    pub count: refcount_t,
    pub gc_proc: rpc_gss_proc,
    pub gc_seq: u32,
    pub gc_seq_xmit: u32,
    pub gc_seq_lock: spinlock_t,
    pub gc_gss_ctx: *mut gss_ctx,
    pub gc_wire_ctx: xdr_netobj,
    pub gc_acceptor: xdr_netobj,
    pub gc_win: u32,
    pub gc_expiry: usize,
    pub gc_rcu: rcu_head,
}

pub struct gss_upcall_msg;

#[repr(C)]
pub struct gss_cred {
    pub gc_base: rpc_cred,
    pub gc_service: rpc_gss_svc,
    pub gc_ctx: *mut gss_cl_ctx,
    pub gc_upcall: *mut gss_upcall_msg,
    pub gc_principal: *const core::ffi::c_char,
    pub gc_upcall_timestamp: usize,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
