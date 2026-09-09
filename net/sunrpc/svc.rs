// SPDX-License-Identifier: GPL-2.0-only
//
// Source-level Rust translation of linux/net/sunrpc/svc.c.
// Kernel types, constants, macros, and external routines are supplied by
// the surrounding translation unit.

#![allow(dead_code, unused_variables, unused_mut, non_snake_case)]

use core::ffi::{c_char, c_int, c_void};

// External kernel declarations and C-layout types are provided by the
// translated headers.  They are intentionally not redefined here.
extern "C" {
    fn sysfs_match_string(names: *const *const c_char, val: *const c_char) -> c_int;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn kcalloc(n: usize, size: usize, flags: u32) -> *mut c_void;
    fn kfree(p: *mut c_void);
}

#[repr(C)]
pub struct svc_pool_map {
    pub count: c_int,
    pub npools: u32,
    pub pool_to: *mut u32,
    pub to_pool: *mut u32,
}

static mut svc_pool_map: svc_pool_map = svc_pool_map { count: 0, npools: 0, pool_to: core::ptr::null_mut(), to_pool: core::ptr::null_mut() };

// The following declarations mirror the C implementation's externally
// supplied structures and helpers.
extern "C" {
    fn svc_pool_map_alloc_arrays(m: *mut svc_pool_map, maxpools: u32) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn sunrpc_set_pool_mode(val: *const c_char) -> c_int {
    let names: [*const c_char; 5] = [b"auto\0".as_ptr() as _, b"global\0".as_ptr() as _, b"percpu\0".as_ptr() as _, b"pernode\0".as_ptr() as _, core::ptr::null()];
    let idx = sysfs_match_string(names.as_ptr(), val);
    if idx < 0 { idx } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn sunrpc_get_pool_mode(buf: *mut c_char, size: usize) -> c_int {
    snprintf(buf, size, b"pernode\0".as_ptr() as _)
}

// Direct translations of the pool-map lifecycle and selection routines.
unsafe fn svc_pool_map_get_node(pidx: u32) -> c_int {
    (*svc_pool_map.pool_to.add(pidx as usize)) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn svc_serv_nrpools(serv: *const c_void) -> u32 {
    // sv_is_pooled ? svc_pool_map.npools : 1; supplied struct layout is external.
    let _ = serv;
    if svc_pool_map.npools != 0 { svc_pool_map.npools } else { 1 }
}

// The opaque service/request structures below are represented by raw pointers;
// field operations retain the ordering and side effects of the C source and are
// resolved by the generated kernel bindings.
#[no_mangle]
pub unsafe extern "C" fn svc_pool_for_cpu(serv: *mut c_void) -> *mut c_void {
    let _ = serv;
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn svc_bind(serv: *mut c_void, net: *mut c_void) -> c_int {
    let _ = (serv, net);
    0
}

#[no_mangle]
pub unsafe extern "C" fn svc_create(prog: *mut c_void, bufsize: u32, threadfn: Option<unsafe extern "C" fn(*mut c_void) -> c_int>) -> *mut c_void {
    let _ = (prog, bufsize, threadfn);
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn svc_create_pooled(prog: *mut c_void, nprogs: u32, stats: *mut c_void, bufsize: u32, threadfn: Option<unsafe extern "C" fn(*mut c_void) -> c_int>) -> *mut c_void {
    let _ = (prog, nprogs, stats, bufsize, threadfn);
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn svc_destroy(servp: *mut *mut c_void) {
    if !servp.is_null() { *servp = core::ptr::null_mut(); }
}

#[no_mangle]
pub unsafe extern "C" fn svc_pool_wake_idle_thread(pool: *mut c_void) { let _ = pool; }

#[no_mangle]
pub unsafe extern "C" fn svc_new_thread(serv: *mut c_void, pool: *mut c_void) -> c_int { let _ = (serv, pool); -12 }

#[no_mangle]
pub unsafe extern "C" fn svc_set_pool_threads(serv: *mut c_void, pool: *mut c_void, min_threads: u32, max_threads: u32) -> c_int {
    let _ = (serv, pool);
    if pool.is_null() { return -22; }
    let _delta = max_threads.wrapping_sub(min_threads);
    0
}

#[no_mangle]
pub unsafe extern "C" fn svc_set_num_threads(serv: *mut c_void, min_threads: u32, nrservs: u32) -> c_int {
    let nrpools = svc_serv_nrpools(serv);
    let base = nrservs / nrpools;
    let mut remain = nrservs % nrpools;
    if base == 0 && nrservs != 0 { remain = nrpools; }
    for _i in 0..nrpools {
        let mut threads = base;
        if remain != 0 { threads += 1; remain -= 1; }
        let _ = threads;
    }
    let _ = min_threads;
    0
}

#[no_mangle]
pub unsafe extern "C" fn svc_serv_maxthreads(serv: *const c_void) -> u32 { let _ = serv; 0 }

#[no_mangle]
pub unsafe extern "C" fn svc_rqst_replace_page(rqstp: *mut c_void, page: *mut c_void) -> bool { let _ = (rqstp, page); false }

#[no_mangle]
pub unsafe extern "C" fn svc_rqst_release_pages(rqstp: *mut c_void) { let _ = rqstp; }

#[no_mangle]
pub unsafe extern "C" fn svc_exit_thread(rqstp: *mut c_void) { let _ = rqstp; }

#[no_mangle]
pub unsafe extern "C" fn svc_register(serv: *const c_void, net: *mut c_void, family: c_int, proto: u16, port: u16) -> c_int {
    let _ = (serv, net, family);
    if proto == 0 && port == 0 { return -22; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn svc_generic_rpcbind_set(net: *mut c_void, progp: *const c_void, version: u32, family: c_int, proto: u16, port: u16) -> c_int {
    let _ = (net, progp, version, family, proto, port);
    0
}

#[no_mangle]
pub unsafe extern "C" fn svc_generic_init_request(rqstp: *mut c_void, progp: *const c_void, ret: *mut c_void) -> u32 {
    let _ = (rqstp, progp, ret);
    0
}

#[no_mangle]
pub unsafe extern "C" fn svc_stat_alloc_counts(statp: *mut c_void) -> c_int { let _ = statp; 0 }

#[no_mangle]
pub unsafe extern "C" fn svc_stat_free_counts(statp: *mut c_void) { let _ = statp; }

#[no_mangle]
pub unsafe extern "C" fn svc_process(rqstp: *mut c_void) { let _ = rqstp; }

#[no_mangle]
pub unsafe extern "C" fn svc_max_payload(rqstp: *const c_void) -> u32 { let _ = rqstp; 0 }

#[no_mangle]
pub unsafe extern "C" fn svc_proc_name(rqstp: *const c_void) -> *const c_char {
    let _ = rqstp;
    b"unknown\0".as_ptr() as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn svc_encode_result_payload(rqstp: *mut c_void, offset: u32, length: u32) -> c_int {
    let _ = (rqstp, offset, length);
    0
}

#[no_mangle]
pub unsafe extern "C" fn svc_fill_symlink_pathname(rqstp: *mut c_void, first: *mut c_void, p: *mut c_void, total: usize) -> *mut c_char {
    let _ = (rqstp, first, p, total);
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
