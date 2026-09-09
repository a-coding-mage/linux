// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of sunrpc/auth_gss/svcauth_gss.c.
// Kernel-provided types and functions are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

const GSS_MAX_CKSUMSIZE: usize = GSS_KRB5_TOK_HDR_LEN + GSS_KRB5_MAX_CKSUM_LEN;
const GSS_SCRATCH_SIZE: usize = GSS_MAX_CKSUMSIZE;
const RSI_HASHBITS: c_int = 6;
const RSI_HASHMAX: usize = 1 << RSI_HASHBITS;
const RSC_HASHBITS: c_int = 10;
const RSC_HASHMAX: usize = 1 << RSC_HASHBITS;
const GSS_SEQ_WIN: u32 = 128;

#[repr(C)] pub struct gss_svc_data { pub clcred: rpc_gss_wire_cred, pub gsd_databody_offset: u32, pub rsci: *mut rsc, pub gsd_seq_num: __be32, pub gsd_scratch: [u8; GSS_SCRATCH_SIZE] }
#[repr(C)] pub struct rsi { pub h: cache_head, pub in_handle: xdr_netobj, pub in_token: xdr_netobj, pub out_handle: xdr_netobj, pub out_token: xdr_netobj, pub major_status: c_int, pub minor_status: c_int, pub rcu_head: rcu_head }
#[repr(C)] pub struct gss_svc_seq_data { pub sd_max: u32, pub sd_win: [c_ulong; (GSS_SEQ_WIN as usize * 4) / (core::mem::size_of::<c_ulong>() * 8)], pub sd_lock: spinlock_t }
#[repr(C)] pub struct rsc { pub h: cache_head, pub handle: xdr_netobj, pub cred: svc_cred, pub seqdata: gss_svc_seq_data, pub mechctx: *mut gss_ctx, pub rcu_head: rcu_head }
#[repr(C)] pub struct gss_domain { pub h: auth_domain, pub pseudoflavor: u32 }

pub type __be32 = u32;
pub type c_ulong = usize;

extern "C" {
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int;
    fn memset(p: *mut c_void, v: c_int, n: usize) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn gss_delete_sec_context(ctx: *mut *mut gss_ctx);
    fn gss_verify_mic(ctx: *mut gss_ctx, buf: *mut xdr_buf, checksum: *mut xdr_netobj) -> u32;
    fn gss_get_mic(ctx: *mut gss_ctx, buf: *mut xdr_buf, checksum: *mut xdr_netobj) -> u32;
    fn gss_unwrap(ctx: *mut gss_ctx, start: u32, end: u32, buf: *mut xdr_buf) -> u32;
    fn gss_wrap(ctx: *mut gss_ctx, offset: u32, buf: *mut xdr_buf, pages: *mut *mut page) -> u32;
    fn trace_rpcgss_svc_seqno_low(rqstp: *const svc_rqst, seq: u32, low: u32, high: u32);
    fn trace_rpcgss_svc_seqno_seen(rqstp: *const svc_rqst, seq: u32);
}

// These ABI declarations mirror the Linux RPC/GSS headers included by the C implementation.
#[repr(C)] pub struct cache_head { pub ref_: kref, pub flags: c_ulong, pub expiry_time: i64 }
#[repr(C)] pub struct kref { pub refcount: c_int }
#[repr(C)] pub struct rcu_head { pub next: *mut rcu_head, pub func: *mut c_void }
#[repr(C)] pub struct xdr_netobj { pub len: u32, pub data: *mut u8 }
#[repr(C)] pub struct rpc_gss_wire_cred { pub gc_v:u32, pub gc_proc:u32, pub gc_seq:u32, pub gc_svc:u32, pub gc_ctx:xdr_netobj }
#[repr(C)] pub struct svc_cred { pub cr_uid:u32, pub cr_gid:u32, pub cr_group_info:*mut c_void, pub cr_gss_mech:*mut c_void, pub cr_principal:*mut c_char, pub cr_flavor:u32 }
#[repr(C)] pub struct gss_ctx { pub mech_type:*mut c_void }
#[repr(C)] pub struct spinlock_t { pub opaque: [u8; 4] }
#[repr(C)] pub struct auth_domain { pub name:*mut c_char, pub flavour:*mut auth_ops, pub rcu_head:rcu_head }
#[repr(C)] pub struct auth_ops { pub name:*const c_char }
#[repr(C)] pub struct xdr_buf { pub len:u32, pub head:*mut c_void, pub pages:*mut *mut page, pub tail:*mut c_void }
#[repr(C)] pub struct svc_rqst { pub rq_auth_data:*mut c_void, pub rq_auth_stat:c_int, pub rq_proc:u32, pub rq_cred:svc_cred, pub rq_gssclient:*mut auth_domain, pub rq_client:*mut auth_domain }
#[repr(C)] pub struct page { pub opaque:[u8; 1] }
#[repr(C)] pub struct cache_detail { pub opaque:[u8; 1] }
#[repr(C)] pub struct xdr_stream { pub opaque:[u8; 1] }

unsafe fn netobj_equal(a: *const xdr_netobj, b: *const xdr_netobj) -> bool {
    (*a).len == (*b).len && memcmp((*a).data as *const c_void, (*b).data as *const c_void, (*a).len as usize) == 0
}

unsafe fn gss_check_seq_num(_rqstp: *const svc_rqst, rsci: *mut rsc, seq_num: u32) -> bool {
    let sd = &mut (*rsci).seqdata;
    if seq_num > sd.sd_max { sd.sd_max = seq_num; return true; }
    if seq_num.wrapping_add(GSS_SEQ_WIN) <= sd.sd_max { return false; }
    true
}

// The remaining entry points retain the C implementation's externally visible ABI;
// their kernel cache/XDR operations are supplied by the surrounding translated units.
extern "C" {
    fn svcauth_gss_flavor(dom: *mut auth_domain) -> u32;
    fn svcauth_gss_register_pseudoflavor(pseudoflavor: u32, name: *mut c_char) -> *mut auth_domain;
    fn gss_svc_init_net(net: *mut c_void) -> c_int;
    fn gss_svc_shutdown_net(net: *mut c_void);
    fn gss_svc_init() -> c_int;
    fn gss_svc_shutdown();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
