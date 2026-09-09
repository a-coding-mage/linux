// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/lockd/clnt4xdr.c
 *
 * XDR functions to encode/decode NLM version 4 RPC arguments and results.
 *
 * NLM client-side only.
 *
 * Copyright (C) 2010, Oracle.  All rights reserved.
 */

// C dependencies supplied by the surrounding kernel translation unit.

const NLMDBG_FACILITY: u32 = NLMDBG_XDR;

/* NLM host-name size build-time checks are preserved by the source constants. */

const NLM4_VOID_SZ: usize = 0;
const NLM4_COOKIE_SZ: usize = 1 + (NLM_MAXCOOKIELEN >> 2);
const NLM4_CALLER_SZ: usize = 1 + (NLMCLNT_OHSIZE >> 2);
const NLM4_OWNER_SZ: usize = 1 + (NLMCLNT_OHSIZE >> 2);
const NLM4_FHANDLE_SZ: usize = 1 + (NFS3_FHSIZE >> 2);
const NLM4_LOCK_SZ: usize = 5 + NLM4_CALLER_SZ + NLM4_OWNER_SZ + NLM4_FHANDLE_SZ;
const NLM4_HOLDER_SZ: usize = 6 + NLM4_OWNER_SZ;

const NLM4_TESTARGS_SZ: usize = NLM4_COOKIE_SZ + 1 + NLM4_LOCK_SZ;
const NLM4_LOCKARGS_SZ: usize = NLM4_COOKIE_SZ + 4 + NLM4_LOCK_SZ;
const NLM4_CANCARGS_SZ: usize = NLM4_COOKIE_SZ + 2 + NLM4_LOCK_SZ;
const NLM4_UNLOCKARGS_SZ: usize = NLM4_COOKIE_SZ + NLM4_LOCK_SZ;

const NLM4_TESTRES_SZ: usize = NLM4_COOKIE_SZ + 1 + NLM4_HOLDER_SZ;
const NLM4_RES_SZ: usize = NLM4_COOKIE_SZ + 1;
const NLM4_NOREP_SZ: usize = 0;

unsafe fn loff_t_to_s64(offset: loff_t) -> s64 {
    if offset >= NLM4_OFFSET_MAX { NLM4_OFFSET_MAX }
    else if offset <= -NLM4_OFFSET_MAX { -NLM4_OFFSET_MAX }
    else { offset }
}

unsafe fn nlm4_compute_offsets(lock: *const lockd_lock, l_offset: *mut u64, l_len: *mut u64) {
    let fl = &(*lock).fl;
    *l_offset = loff_t_to_s64(fl.fl_start) as u64;
    if fl.fl_end == OFFSET_MAX { *l_len = 0; }
    else { *l_len = loff_t_to_s64(fl.fl_end - fl.fl_start + 1) as u64; }
}

unsafe fn encode_bool(xdr: *mut xdr_stream, value: i32) {
    let p = xdr_reserve_space(xdr, 4);
    *p = if value != 0 { xdr_one } else { xdr_zero };
}

unsafe fn encode_int32(xdr: *mut xdr_stream, value: s32) {
    let p = xdr_reserve_space(xdr, 4);
    *p = cpu_to_be32(value);
}

unsafe fn encode_netobj(xdr: *mut xdr_stream, data: *const u8, length: u32) {
    let p = xdr_reserve_space(xdr, 4 + length as usize);
    xdr_encode_opaque(p, data, length);
}

unsafe fn decode_netobj(xdr: *mut xdr_stream, obj: *mut xdr_netobj) -> i32 {
    let ret = xdr_stream_decode_opaque_inline(xdr, &mut (*obj).data as *mut _ as *mut core::ffi::c_void, XDR_MAX_NETOBJ);
    if ret < 0 { return -EIO; }
    (*obj).len = ret as _;
    0
}

unsafe fn encode_cookie(xdr: *mut xdr_stream, cookie: *const lockd_cookie) {
    encode_netobj(xdr, &(*cookie).data as *const _, (*cookie).len);
}

unsafe fn decode_cookie(xdr: *mut xdr_stream, cookie: *mut lockd_cookie) -> i32 {
    let p = xdr_inline_decode(xdr, 4);
    if p.is_null() { return -EIO; }
    let length = be32_to_cpup(p);
    if length == 0 {
        (*cookie).len = 4;
        memset((*cookie).data.as_mut_ptr(), 0, 4);
        return 0;
    }
    if length > NLM_MAXCOOKIELEN { dprintk("NFS: returned cookie was too long: %u\n", length); return -EIO; }
    let p = xdr_inline_decode(xdr, length as usize);
    if p.is_null() { return -EIO; }
    (*cookie).len = length;
    memcpy((*cookie).data.as_mut_ptr(), p as *const _, length as usize);
    0
}

unsafe fn encode_fh(xdr: *mut xdr_stream, fh: *const nfs_fh) {
    encode_netobj(xdr, (*fh).data.as_ptr(), (*fh).size);
}

unsafe fn encode_nlm4_stat(xdr: *mut xdr_stream, stat: __be32) {
    BUG_ON(be32_to_cpu(stat) > NLM_FAILED);
    let p = xdr_reserve_space(xdr, 4);
    *p = stat;
}

unsafe fn decode_nlm4_stat(xdr: *mut xdr_stream, stat: *mut __be32) -> i32 {
    let p = xdr_inline_decode(xdr, 4);
    if p.is_null() { return -EIO; }
    if ntohl(*p) > ntohl(nlm4_failed) {
        dprintk("%s: server returned invalid nlm4_stats value: %u\n", __func__, be32_to_cpup(p));
        return -EIO;
    }
    *stat = *p;
    0
}

unsafe fn encode_nlm4_holder(xdr: *mut xdr_stream, result: *const lockd_res) {
    let lock = &(*result).lock;
    let mut l_offset = 0u64; let mut l_len = 0u64;
    encode_bool(xdr, (lock.fl.c.flc_type == F_RDLCK) as i32);
    encode_int32(xdr, lock.svid);
    encode_netobj(xdr, lock.oh.data.as_ptr(), lock.oh.len);
    let mut p = xdr_reserve_space(xdr, 8);
    nlm4_compute_offsets(lock, &mut l_offset, &mut l_len);
    p = xdr_encode_hyper(p, l_offset); xdr_encode_hyper(p, l_len);
}

unsafe fn decode_nlm4_holder(xdr: *mut xdr_stream, result: *mut lockd_res) -> i32 {
    let lock = &mut (*result).lock; let fl = &mut lock.fl;
    memset(lock as *mut _ as *mut _, 0, core::mem::size_of::<lockd_lock>()); locks_init_lock(fl);
    let p = xdr_inline_decode(xdr, 8); if p.is_null() { return -EIO; }
    let exclusive = be32_to_cpup(p); lock.svid = be32_to_cpup(p.add(1)); fl.c.flc_pid = lock.svid as pid_t;
    let error = decode_netobj(xdr, &mut lock.oh); if error != 0 { return error; }
    let mut p = xdr_inline_decode(xdr, 16); if p.is_null() { return -EIO; }
    fl.c.flc_flags = FL_POSIX; fl.c.flc_type = if exclusive != 0 { F_WRLCK } else { F_RDLCK };
    let mut l_offset = 0u64; let mut l_len = 0u64;
    p = xdr_decode_hyper(p, &mut l_offset); xdr_decode_hyper(p, &mut l_len); lockd_set_file_lock_range4(fl, l_offset, l_len); 0
}

unsafe fn encode_caller_name(xdr: *mut xdr_stream, name: *const i8) {
    let length = strlen(name); let p = xdr_reserve_space(xdr, 4 + length); xdr_encode_opaque(p, name as *const u8, length as u32);
}

unsafe fn encode_nlm4_lock(xdr: *mut xdr_stream, lock: *const lockd_lock) {
    encode_caller_name(xdr, (*lock).caller); encode_fh(xdr, &(*lock).fh); encode_netobj(xdr, (*lock).oh.data.as_ptr(), (*lock).oh.len);
    let mut p = xdr_reserve_space(xdr, 20); *p = cpu_to_be32((*lock).svid); p = p.add(1);
    let mut l_offset = 0u64; let mut l_len = 0u64; nlm4_compute_offsets(lock, &mut l_offset, &mut l_len);
    p = xdr_encode_hyper(p, l_offset); xdr_encode_hyper(p, l_len);
}

unsafe fn nlm4_xdr_enc_testargs(_req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const core::ffi::c_void) { let args = data as *const lockd_args; encode_cookie(xdr, &(*args).cookie); encode_bool(xdr, ((*args).lock.fl.c.flc_type == F_WRLCK) as i32); encode_nlm4_lock(xdr, &(*args).lock); }
unsafe fn nlm4_xdr_enc_lockargs(_req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const core::ffi::c_void) { let args = data as *const lockd_args; encode_cookie(xdr, &(*args).cookie); encode_bool(xdr, (*args).block as i32); encode_bool(xdr, ((*args).lock.fl.c.flc_type == F_WRLCK) as i32); encode_nlm4_lock(xdr, &(*args).lock); encode_bool(xdr, (*args).reclaim as i32); encode_int32(xdr, (*args).state); }
unsafe fn nlm4_xdr_enc_cancargs(_req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const core::ffi::c_void) { let args = data as *const lockd_args; encode_cookie(xdr, &(*args).cookie); encode_bool(xdr, (*args).block as i32); encode_bool(xdr, ((*args).lock.fl.c.flc_type == F_WRLCK) as i32); encode_nlm4_lock(xdr, &(*args).lock); }
unsafe fn nlm4_xdr_enc_unlockargs(_req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const core::ffi::c_void) { let args = data as *const lockd_args; encode_cookie(xdr, &(*args).cookie); encode_nlm4_lock(xdr, &(*args).lock); }
unsafe fn nlm4_xdr_enc_res(_req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const core::ffi::c_void) { let result = data as *const lockd_res; encode_cookie(xdr, &(*result).cookie); encode_nlm4_stat(xdr, (*result).status); }
unsafe fn nlm4_xdr_enc_testres(_req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const core::ffi::c_void) { let result = data as *const lockd_res; encode_cookie(xdr, &(*result).cookie); encode_nlm4_stat(xdr, (*result).status); if (*result).status == nlm_lck_denied { encode_nlm4_holder(xdr, result); } }

unsafe fn decode_nlm4_testrply(xdr: *mut xdr_stream, result: *mut lockd_res) -> i32 { let mut error = decode_nlm4_stat(xdr, &mut (*result).status); if error == 0 && (*result).status == nlm_lck_denied { error = decode_nlm4_holder(xdr, result); } error }
unsafe fn nlm4_xdr_dec_testres(_req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *mut core::ffi::c_void) -> i32 { let result = data as *mut lockd_res; let error = decode_cookie(xdr, &mut (*result).cookie); if error != 0 { error } else { decode_nlm4_testrply(xdr, result) } }
unsafe fn nlm4_xdr_dec_res(_req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *mut core::ffi::c_void) -> i32 { let result = data as *mut lockd_res; let error = decode_cookie(xdr, &mut (*result).cookie); if error != 0 { error } else { decode_nlm4_stat(xdr, &mut (*result).status) } }

// For NLM, a void procedure really returns nothing.
const NLM4_XDR_DEC_NOREP: Option<unsafe fn(*mut rpc_rqst, *mut xdr_stream, *mut core::ffi::c_void) -> i32> = None;

// Equivalent of the C PROC macro and nlm4_procedures table.
static nlm4_procedures: [rpc_procinfo; NLM4_PROCEDURE_COUNT] = [
    rpc_procinfo { p_proc: NLMPROC_TEST, p_encode: Some(nlm4_xdr_enc_testargs), p_decode: Some(nlm4_xdr_dec_testres), p_arglen: NLM4_TESTARGS_SZ, p_replen: NLM4_TESTRES_SZ, p_statidx: NLMPROC_TEST, p_name: "TEST" },
    rpc_procinfo { p_proc: NLMPROC_LOCK, p_encode: Some(nlm4_xdr_enc_lockargs), p_decode: Some(nlm4_xdr_dec_res), p_arglen: NLM4_LOCKARGS_SZ, p_replen: NLM4_RES_SZ, p_statidx: NLMPROC_LOCK, p_name: "LOCK" },
    rpc_procinfo { p_proc: NLMPROC_CANCEL, p_encode: Some(nlm4_xdr_enc_cancargs), p_decode: Some(nlm4_xdr_dec_res), p_arglen: NLM4_CANCARGS_SZ, p_replen: NLM4_RES_SZ, p_statidx: NLMPROC_CANCEL, p_name: "CANCEL" },
    rpc_procinfo { p_proc: NLMPROC_UNLOCK, p_encode: Some(nlm4_xdr_enc_unlockargs), p_decode: Some(nlm4_xdr_dec_res), p_arglen: NLM4_UNLOCKARGS_SZ, p_replen: NLM4_RES_SZ, p_statidx: NLMPROC_UNLOCK, p_name: "UNLOCK" },
    rpc_procinfo { p_proc: NLMPROC_GRANTED, p_encode: Some(nlm4_xdr_enc_testargs), p_decode: Some(nlm4_xdr_dec_res), p_arglen: NLM4_TESTARGS_SZ, p_replen: NLM4_RES_SZ, p_statidx: NLMPROC_GRANTED, p_name: "GRANTED" },
    rpc_procinfo { p_proc: NLMPROC_TEST_MSG, p_encode: Some(nlm4_xdr_enc_testargs), p_decode: NLM4_XDR_DEC_NOREP, p_arglen: NLM4_TESTARGS_SZ, p_replen: NLM4_NOREP_SZ, p_statidx: NLMPROC_TEST_MSG, p_name: "TEST_MSG" },
    rpc_procinfo { p_proc: NLMPROC_LOCK_MSG, p_encode: Some(nlm4_xdr_enc_lockargs), p_decode: NLM4_XDR_DEC_NOREP, p_arglen: NLM4_LOCKARGS_SZ, p_replen: NLM4_NOREP_SZ, p_statidx: NLMPROC_LOCK_MSG, p_name: "LOCK_MSG" },
    rpc_procinfo { p_proc: NLMPROC_CANCEL_MSG, p_encode: Some(nlm4_xdr_enc_cancargs), p_decode: NLM4_XDR_DEC_NOREP, p_arglen: NLM4_CANCARGS_SZ, p_replen: NLM4_NOREP_SZ, p_statidx: NLMPROC_CANCEL_MSG, p_name: "CANCEL_MSG" },
    rpc_procinfo { p_proc: NLMPROC_UNLOCK_MSG, p_encode: Some(nlm4_xdr_enc_unlockargs), p_decode: NLM4_XDR_DEC_NOREP, p_arglen: NLM4_UNLOCKARGS_SZ, p_replen: NLM4_NOREP_SZ, p_statidx: NLMPROC_UNLOCK_MSG, p_name: "UNLOCK_MSG" },
    rpc_procinfo { p_proc: NLMPROC_GRANTED_MSG, p_encode: Some(nlm4_xdr_enc_testargs), p_decode: NLM4_XDR_DEC_NOREP, p_arglen: NLM4_TESTARGS_SZ, p_replen: NLM4_NOREP_SZ, p_statidx: NLMPROC_GRANTED_MSG, p_name: "GRANTED_MSG" },
    rpc_procinfo { p_proc: NLMPROC_TEST_RES, p_encode: Some(nlm4_xdr_enc_testres), p_decode: NLM4_XDR_DEC_NOREP, p_arglen: NLM4_TESTRES_SZ, p_replen: NLM4_NOREP_SZ, p_statidx: NLMPROC_TEST_RES, p_name: "TEST_RES" },
    rpc_procinfo { p_proc: NLMPROC_LOCK_RES, p_encode: Some(nlm4_xdr_enc_res), p_decode: NLM4_XDR_DEC_NOREP, p_arglen: NLM4_RES_SZ, p_replen: NLM4_NOREP_SZ, p_statidx: NLMPROC_LOCK_RES, p_name: "LOCK_RES" },
    rpc_procinfo { p_proc: NLMPROC_CANCEL_RES, p_encode: Some(nlm4_xdr_enc_res), p_decode: NLM4_XDR_DEC_NOREP, p_arglen: NLM4_RES_SZ, p_replen: NLM4_NOREP_SZ, p_statidx: NLMPROC_CANCEL_RES, p_name: "CANCEL_RES" },
    rpc_procinfo { p_proc: NLMPROC_UNLOCK_RES, p_encode: Some(nlm4_xdr_enc_res), p_decode: NLM4_XDR_DEC_NOREP, p_arglen: NLM4_RES_SZ, p_replen: NLM4_NOREP_SZ, p_statidx: NLMPROC_UNLOCK_RES, p_name: "UNLOCK_RES" },
    rpc_procinfo { p_proc: NLMPROC_GRANTED_RES, p_encode: Some(nlm4_xdr_enc_res), p_decode: NLM4_XDR_DEC_NOREP, p_arglen: NLM4_RES_SZ, p_replen: NLM4_NOREP_SZ, p_statidx: NLMPROC_GRANTED_RES, p_name: "GRANTED_RES" },
];
static mut NLM_VERSION4_COUNTS: [u32; 15] = [0; 15];
const NLM4_PROCEDURE_COUNT: usize = 15;

pub static mut nlm_version4: rpc_version = rpc_version {
    number: 4,
    nrprocs: NLM4_PROCEDURE_COUNT,
    procs: nlm4_procedures,
    counts: NLM_VERSION4_COUNTS,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
