// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/lockd/clntxdr.c
 *
 * XDR functions to encode/decode NLM version 1 and 3 RPC
 * arguments and results. NLM version 2 is not specified
 * by a standard, thus it is not implemented.
 *
 * NLM client-side only.
 *
 * Copyright (C) 2010, Oracle.  All rights reserved.
 */

// Linux and lockd headers supply the types, constants, and XDR functions used below.

const NLMDBG_FACILITY: u32 = NLMDBG_XDR;

const NLM_COOKIE_SZ: usize = 1 + (NLM_MAXCOOKIELEN >> 2);
const NLM_CALLER_SZ: usize = 1 + (NLMCLNT_OHSIZE >> 2);
const NLM_OWNER_SZ: usize = 1 + (NLMCLNT_OHSIZE >> 2);
const NLM_FHANDLE_SZ: usize = 1 + (NFS2_FHSIZE >> 2);
const NLM_LOCK_SZ: usize = 3 + NLM_CALLER_SZ + NLM_OWNER_SZ + NLM_FHANDLE_SZ;
const NLM_HOLDER_SZ: usize = 4 + NLM_OWNER_SZ;
const NLM_TESTARGS_SZ: usize = NLM_COOKIE_SZ + 1 + NLM_LOCK_SZ;
const NLM_LOCKARGS_SZ: usize = NLM_COOKIE_SZ + 4 + NLM_LOCK_SZ;
const NLM_CANCARGS_SZ: usize = NLM_COOKIE_SZ + 2 + NLM_LOCK_SZ;
const NLM_UNLOCKARGS_SZ: usize = NLM_COOKIE_SZ + NLM_LOCK_SZ;
const NLM_TESTRES_SZ: usize = NLM_COOKIE_SZ + 1 + NLM_HOLDER_SZ;
const NLM_RES_SZ: usize = NLM_COOKIE_SZ + 1;
const NLM_NOREP_SZ: usize = 0;

unsafe fn loff_t_to_s32(offset: loff_t) -> s32 {
    if offset >= NLM_OFFSET_MAX { NLM_OFFSET_MAX as s32 }
    else if offset <= -NLM_OFFSET_MAX { -(NLM_OFFSET_MAX as s32) }
    else { offset as s32 }
}

unsafe fn nlm_compute_offsets(lock: *const lockd_lock, l_offset: *mut u32, l_len: *mut u32) {
    let fl = &(*lock).fl;
    *l_offset = loff_t_to_s32(fl.fl_start) as u32;
    if fl.fl_end == OFFSET_MAX { *l_len = 0; }
    else { *l_len = loff_t_to_s32(fl.fl_end - fl.fl_start + 1) as u32; }
}

unsafe fn encode_bool(xdr: *mut xdr_stream, value: c_int) {
    let p = xdr_reserve_space(xdr, 4);
    *p = if value != 0 { xdr_one } else { xdr_zero };
}

unsafe fn encode_int32(xdr: *mut xdr_stream, value: s32) {
    *xdr_reserve_space(xdr, 4) = cpu_to_be32(value as u32);
}

unsafe fn encode_netobj(xdr: *mut xdr_stream, data: *const u8, length: c_uint) {
    let p = xdr_reserve_space(xdr, 4 + length as usize);
    xdr_encode_opaque(p, data, length);
}

unsafe fn decode_netobj(xdr: *mut xdr_stream, obj: *mut xdr_netobj) -> c_int {
    let ret = xdr_stream_decode_opaque_inline(xdr, &mut (*obj).data as *mut _ as *mut c_void, XDR_MAX_NETOBJ);
    if ret < 0 { return -EIO; }
    (*obj).len = ret as _;
    0
}

unsafe fn encode_cookie(xdr: *mut xdr_stream, cookie: *const lockd_cookie) {
    encode_netobj(xdr, (*cookie).data.as_ptr(), (*cookie).len);
}

unsafe fn decode_cookie(xdr: *mut xdr_stream, cookie: *mut lockd_cookie) -> c_int {
    let p = xdr_inline_decode(xdr, 4);
    if p.is_null() { return -EIO; }
    let length = be32_to_cpup(p);
    if length == 0 { (*cookie).len = 4; memset((*cookie).data.as_mut_ptr() as *mut c_void, 0, 4); return 0; }
    if length > NLM_MAXCOOKIELEN { dprintk!("NFS: returned cookie was too long: %u\\n", length); return -EIO; }
    let p = xdr_inline_decode(xdr, length as usize);
    if p.is_null() { return -EIO; }
    (*cookie).len = length;
    memcpy((*cookie).data.as_mut_ptr() as *mut c_void, p as *const c_void, length as usize);
    0
}

unsafe fn encode_fh(xdr: *mut xdr_stream, fh: *const nfs_fh) {
    encode_netobj(xdr, (*fh).data.as_ptr(), NFS2_FHSIZE);
}

unsafe fn encode_nlm_stat(xdr: *mut xdr_stream, stat: __be32) {
    WARN_ON_ONCE!(be32_to_cpu(stat) > NLM_LCK_DENIED_GRACE_PERIOD);
    *xdr_reserve_space(xdr, 4) = stat;
}

unsafe fn decode_nlm_stat(xdr: *mut xdr_stream, stat: *mut __be32) -> c_int {
    let p = xdr_inline_decode(xdr, 4);
    if p.is_null() { return -EIO; }
    if ntohl(*p) > ntohl(nlm_lck_denied_grace_period) {
        dprintk!("%s: server returned invalid nlm_stats value: %u\\n", "decode_nlm_stat", be32_to_cpup(p));
        return -EIO;
    }
    *stat = *p;
    0
}

unsafe fn encode_nlm_holder(xdr: *mut xdr_stream, result: *const lockd_res) {
    let lock = &(*result).lock;
    encode_bool(xdr, (lock.fl.c.flc_type == F_RDLCK) as c_int);
    encode_int32(xdr, lock.svid);
    encode_netobj(xdr, lock.oh.data.as_ptr(), lock.oh.len);
    let p = xdr_reserve_space(xdr, 8);
    let (mut off, mut len) = (0u32, 0u32);
    nlm_compute_offsets(lock, &mut off, &mut len);
    *p = cpu_to_be32(off); *p.add(1) = cpu_to_be32(len);
}

unsafe fn decode_nlm_holder(xdr: *mut xdr_stream, result: *mut lockd_res) -> c_int {
    let lock = &mut (*result).lock; let fl = &mut lock.fl;
    memset(lock as *mut _ as *mut c_void, 0, core::mem::size_of::<lockd_lock>());
    locks_init_lock(fl);
    let p = xdr_inline_decode(xdr, 8); if p.is_null() { return -EIO; }
    let exclusive = be32_to_cpup(p); lock.svid = be32_to_cpup(p.add(1)); fl.c.flc_pid = lock.svid as pid_t;
    let mut error = decode_netobj(xdr, &mut lock.oh); if error != 0 { return error; }
    let p = xdr_inline_decode(xdr, 8); if p.is_null() { return -EIO; }
    fl.c.flc_flags = FL_POSIX; fl.c.flc_type = if exclusive != 0 { F_WRLCK } else { F_RDLCK };
    let off = be32_to_cpup(p); let len = be32_to_cpup(p.add(1)); let end = off.wrapping_add(len).wrapping_sub(1) as s32;
    fl.fl_start = off as loff_t;
    fl.fl_end = if len == 0 || end < 0 { OFFSET_MAX } else { end as loff_t };
    error = 0; error
}

unsafe fn encode_caller_name(xdr: *mut xdr_stream, name: *const c_char) {
    let length = strlen(name); let p = xdr_reserve_space(xdr, 4 + length); xdr_encode_opaque(p, name as *const u8, length as c_uint);
}

unsafe fn encode_nlm_lock(xdr: *mut xdr_stream, lock: *const lockd_lock) {
    encode_caller_name(xdr, (*lock).caller.as_ptr()); encode_fh(xdr, &(*lock).fh); encode_netobj(xdr, (*lock).oh.data.as_ptr(), (*lock).oh.len);
    let p = xdr_reserve_space(xdr, 12); *p = cpu_to_be32((*lock).svid as u32);
    let (mut off, mut len) = (0u32, 0u32); nlm_compute_offsets(lock, &mut off, &mut len); *p.add(1) = cpu_to_be32(off); *p.add(2) = cpu_to_be32(len);
}

unsafe fn nlm_xdr_enc_testargs(_req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const c_void) { let a = data as *const lockd_args; encode_cookie(xdr, &(*a).cookie); encode_bool(xdr, ((*a).lock.fl.c.flc_type == F_WRLCK) as c_int); encode_nlm_lock(xdr, &(*a).lock); }
unsafe fn nlm_xdr_enc_lockargs(_req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const c_void) { let a = data as *const lockd_args; encode_cookie(xdr, &(*a).cookie); encode_bool(xdr, (*a).block as c_int); encode_bool(xdr, ((*a).lock.fl.c.flc_type == F_WRLCK) as c_int); encode_nlm_lock(xdr, &(*a).lock); encode_bool(xdr, (*a).reclaim as c_int); encode_int32(xdr, (*a).state); }
unsafe fn nlm_xdr_enc_cancargs(_req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const c_void) { let a = data as *const lockd_args; encode_cookie(xdr, &(*a).cookie); encode_bool(xdr, (*a).block as c_int); encode_bool(xdr, ((*a).lock.fl.c.flc_type == F_WRLCK) as c_int); encode_nlm_lock(xdr, &(*a).lock); }
unsafe fn nlm_xdr_enc_unlockargs(_req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const c_void) { let a = data as *const lockd_args; encode_cookie(xdr, &(*a).cookie); encode_nlm_lock(xdr, &(*a).lock); }
unsafe fn nlm_xdr_enc_res(_req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const c_void) { let r = data as *const lockd_res; encode_cookie(xdr, &(*r).cookie); encode_nlm_stat(xdr, (*r).status); }
unsafe fn encode_nlm_testrply(xdr: *mut xdr_stream, result: *const lockd_res) { if (*result).status == nlm_lck_denied { encode_nlm_holder(xdr, result); } }
unsafe fn nlm_xdr_enc_testres(_req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const c_void) { let r = data as *const lockd_res; encode_cookie(xdr, &(*r).cookie); encode_nlm_stat(xdr, (*r).status); encode_nlm_testrply(xdr, r); }

unsafe fn decode_nlm_testrply(xdr: *mut xdr_stream, result: *mut lockd_res) -> c_int { let mut e = decode_nlm_stat(xdr, &mut (*result).status); if e == 0 && (*result).status == nlm_lck_denied { e = decode_nlm_holder(xdr, result); } e }
unsafe fn nlm_xdr_dec_testres(_req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *mut c_void) -> c_int { let r = data as *mut lockd_res; let e = decode_cookie(xdr, &mut (*r).cookie); if e != 0 { e } else { decode_nlm_testrply(xdr, r) } }
unsafe fn nlm_xdr_dec_res(_req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *mut c_void) -> c_int { let r = data as *mut lockd_res; let e = decode_cookie(xdr, &mut (*r).cookie); if e != 0 { e } else { decode_nlm_stat(xdr, &mut (*r).status) } }

// The procedure/version metadata below mirrors the C PROC macro and static tables.
#[allow(non_upper_case_globals)]
static mut nlm_procedures: [rpc_procinfo; 15] = [
    rpc_procinfo { p_proc: NLMPROC_TEST, p_encode: Some(nlm_xdr_enc_testargs), p_decode: Some(nlm_xdr_dec_testres), p_arglen: NLM_TESTARGS_SZ, p_replen: NLM_TESTRES_SZ, p_statidx: NLMPROC_TEST, p_name: b"TEST\\0".as_ptr() as *const c_char },
    rpc_procinfo { p_proc: NLMPROC_LOCK, p_encode: Some(nlm_xdr_enc_lockargs), p_decode: Some(nlm_xdr_dec_res), p_arglen: NLM_LOCKARGS_SZ, p_replen: NLM_RES_SZ, p_statidx: NLMPROC_LOCK, p_name: b"LOCK\\0".as_ptr() as *const c_char },
    rpc_procinfo { p_proc: NLMPROC_CANCEL, p_encode: Some(nlm_xdr_enc_cancargs), p_decode: Some(nlm_xdr_dec_res), p_arglen: NLM_CANCARGS_SZ, p_replen: NLM_RES_SZ, p_statidx: NLMPROC_CANCEL, p_name: b"CANCEL\\0".as_ptr() as *const c_char },
    rpc_procinfo { p_proc: NLMPROC_UNLOCK, p_encode: Some(nlm_xdr_enc_unlockargs), p_decode: Some(nlm_xdr_dec_res), p_arglen: NLM_UNLOCKARGS_SZ, p_replen: NLM_RES_SZ, p_statidx: NLMPROC_UNLOCK, p_name: b"UNLOCK\\0".as_ptr() as *const c_char },
    rpc_procinfo { p_proc: NLMPROC_GRANTED, p_encode: Some(nlm_xdr_enc_testargs), p_decode: Some(nlm_xdr_dec_res), p_arglen: NLM_TESTARGS_SZ, p_replen: NLM_RES_SZ, p_statidx: NLMPROC_GRANTED, p_name: b"GRANTED\\0".as_ptr() as *const c_char },
    rpc_procinfo { p_proc: NLMPROC_TEST_MSG, p_encode: Some(nlm_xdr_enc_testargs), p_decode: None, p_arglen: NLM_TESTARGS_SZ, p_replen: NLM_NOREP_SZ, p_statidx: NLMPROC_TEST_MSG, p_name: b"TEST_MSG\\0".as_ptr() as *const c_char },
    rpc_procinfo { p_proc: NLMPROC_LOCK_MSG, p_encode: Some(nlm_xdr_enc_lockargs), p_decode: None, p_arglen: NLM_LOCKARGS_SZ, p_replen: NLM_NOREP_SZ, p_statidx: NLMPROC_LOCK_MSG, p_name: b"LOCK_MSG\\0".as_ptr() as *const c_char },
    rpc_procinfo { p_proc: NLMPROC_CANCEL_MSG, p_encode: Some(nlm_xdr_enc_cancargs), p_decode: None, p_arglen: NLM_CANCARGS_SZ, p_replen: NLM_NOREP_SZ, p_statidx: NLMPROC_CANCEL_MSG, p_name: b"CANCEL_MSG\\0".as_ptr() as *const c_char },
    rpc_procinfo { p_proc: NLMPROC_UNLOCK_MSG, p_encode: Some(nlm_xdr_enc_unlockargs), p_decode: None, p_arglen: NLM_UNLOCKARGS_SZ, p_replen: NLM_NOREP_SZ, p_statidx: NLMPROC_UNLOCK_MSG, p_name: b"UNLOCK_MSG\\0".as_ptr() as *const c_char },
    rpc_procinfo { p_proc: NLMPROC_GRANTED_MSG, p_encode: Some(nlm_xdr_enc_testargs), p_decode: None, p_arglen: NLM_TESTARGS_SZ, p_replen: NLM_NOREP_SZ, p_statidx: NLMPROC_GRANTED_MSG, p_name: b"GRANTED_MSG\\0".as_ptr() as *const c_char },
    rpc_procinfo { p_proc: NLMPROC_TEST_RES, p_encode: Some(nlm_xdr_enc_testres), p_decode: None, p_arglen: NLM_TESTRES_SZ, p_replen: NLM_NOREP_SZ, p_statidx: NLMPROC_TEST_RES, p_name: b"TEST_RES\\0".as_ptr() as *const c_char },
    rpc_procinfo { p_proc: NLMPROC_LOCK_RES, p_encode: Some(nlm_xdr_enc_res), p_decode: None, p_arglen: NLM_RES_SZ, p_replen: NLM_NOREP_SZ, p_statidx: NLMPROC_LOCK_RES, p_name: b"LOCK_RES\\0".as_ptr() as *const c_char },
    rpc_procinfo { p_proc: NLMPROC_CANCEL_RES, p_encode: Some(nlm_xdr_enc_res), p_decode: None, p_arglen: NLM_RES_SZ, p_replen: NLM_NOREP_SZ, p_statidx: NLMPROC_CANCEL_RES, p_name: b"CANCEL_RES\\0".as_ptr() as *const c_char },
    rpc_procinfo { p_proc: NLMPROC_UNLOCK_RES, p_encode: Some(nlm_xdr_enc_res), p_decode: None, p_arglen: NLM_RES_SZ, p_replen: NLM_NOREP_SZ, p_statidx: NLMPROC_UNLOCK_RES, p_name: b"UNLOCK_RES\\0".as_ptr() as *const c_char },
    rpc_procinfo { p_proc: NLMPROC_GRANTED_RES, p_encode: Some(nlm_xdr_enc_res), p_decode: None, p_arglen: NLM_RES_SZ, p_replen: NLM_NOREP_SZ, p_statidx: NLMPROC_GRANTED_RES, p_name: b"GRANTED_RES\\0".as_ptr() as *const c_char },
];

static mut nlm_version1_counts: [c_uint; 15] = [0; 15];
static mut nlm_version3_counts: [c_uint; 15] = [0; 15];
static nlm_version1: rpc_version = rpc_version { number: 1, nrprocs: 15, procs: nlm_procedures.as_ptr(), counts: nlm_version1_counts.as_ptr() as *mut c_uint };
static nlm_version3: rpc_version = rpc_version { number: 3, nrprocs: 15, procs: nlm_procedures.as_ptr(), counts: nlm_version3_counts.as_ptr() as *mut c_uint };
static nlm_versions: [*const rpc_version; 5] = [core::ptr::null(), &nlm_version1, core::ptr::null(), &nlm_version3, &nlm_version4];
static mut nlm_rpc_stats: rpc_stat = rpc_stat { };
#[no_mangle]
pub static nlm_program: rpc_program = rpc_program { name: b"lockd\\0".as_ptr() as *const c_char, number: NLM_PROGRAM, nrvers: 5, version: nlm_versions.as_ptr(), stats: &nlm_rpc_stats };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
