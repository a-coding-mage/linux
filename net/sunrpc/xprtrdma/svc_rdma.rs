// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
/*
 * Copyright (c) 2015-2018 Oracle. All rights reserved.
 * Copyright (c) 2005-2006 Network Appliance, Inc. All rights reserved.
 *
 * Translated literally from svc_rdma.c. Kernel and RPC/RDMA declarations are
 * supplied by the surrounding translation unit.
 */

// The following names are external kernel/RPC declarations from the C headers.
use core::ffi::{c_char, c_int, c_longlong, c_void};

const RPCDBG_FACILITY: u32 = RPCDBG_SVCXPRT;

pub static mut svcrdma_ord: u32 = 16;
static mut min_ord: u32 = 1;
static mut max_ord: u32 = 255;
pub static mut svcrdma_max_requests: u32 = RPCRDMA_MAX_REQUESTS;
pub static mut svcrdma_max_bc_requests: u32 = RPCRDMA_MAX_BC_REQUESTS;
static mut min_max_requests: u32 = 4;
static mut max_max_requests: u32 = 16384;
pub static mut svcrdma_max_req_size: u32 = RPCRDMA_DEF_INLINE_THRESH;
static mut min_max_inline: u32 = RPCRDMA_DEF_INLINE_THRESH;
static mut max_max_inline: u32 = RPCRDMA_MAX_INLINE_THRESH;
static mut svcrdma_stat_unused: u32 = 0;
static mut zero: u32 = 0;

extern "C" {
    static mut svcrdma_stat_read: percpu_counter;
    static mut svcrdma_stat_recv: percpu_counter;
    static mut svcrdma_stat_sq_starve: percpu_counter;
    static mut svcrdma_stat_write: percpu_counter;
    static mut svcrdma_table_header: *mut ctl_table_header;
    static mut svcrdma_parm_table: [ctl_table; 12];
    static svc_rdma_class: svc_xprt_class;

    fn percpu_counter_set(stat: *mut percpu_counter, val: i64);
    fn percpu_counter_sum_positive(stat: *const percpu_counter) -> u64;
    fn percpu_counter_init(stat: *mut percpu_counter, val: i64, gfp: u32) -> c_int;
    fn percpu_counter_destroy(stat: *mut percpu_counter);
    fn unregister_sysctl_table(header: *mut ctl_table_header);
    fn register_sysctl(path: *const c_char, table: *mut ctl_table) -> *mut ctl_table_header;
    fn svc_unreg_xprt_class(class: *const svc_xprt_class);
    fn svc_reg_xprt_class(class: *const svc_xprt_class);
    fn dprintk(fmt: *const c_char, ...);
}

#[repr(C)] pub struct percpu_counter { _opaque: [u8; 0] }
#[repr(C)] pub struct ctl_table_header { _opaque: [u8; 0] }
#[repr(C)] pub struct svc_xprt_class { _opaque: [u8; 0] }
#[repr(C)] pub struct ctl_table { _opaque: [u8; 0] }

const SVCRDMA_COUNTER_BUFSIZ: usize = core::mem::size_of::<u64>();

unsafe fn svcrdma_counter_handler(
    table: *const ctl_table, write: c_int, buffer: *mut c_void,
    lenp: *mut usize, ppos: *mut c_longlong,
) -> c_int {
    let stat = (*(table as *const *mut percpu_counter));
    let mut tmp = [0u8; SVCRDMA_COUNTER_BUFSIZ + 1];
    if write != 0 {
        percpu_counter_set(stat, 0);
        return 0;
    }
    // snprintf/strlen/memcpy preserve the C handler's bounded text protocol.
    let value = percpu_counter_sum_positive(stat);
    let text = value.to_string();
    let mut len = text.len() + 1;
    if len > SVCRDMA_COUNTER_BUFSIZ { return -14; }
    tmp[..text.len()].copy_from_slice(text.as_bytes());
    tmp[text.len()] = b'\n';
    len = text.len() + 1;
    if *ppos > len as c_longlong { *lenp = 0; return 0; }
    len -= *ppos as usize;
    if len > *lenp { len = *lenp; }
    if len != 0 { core::ptr::copy_nonoverlapping(tmp.as_ptr().add(*ppos as usize), buffer as *mut u8, len); }
    *lenp = len;
    *ppos += len as c_longlong;
    0
}

unsafe fn svc_rdma_proc_cleanup() {
    if svcrdma_table_header.is_null() { return; }
    unregister_sysctl_table(svcrdma_table_header);
    svcrdma_table_header = core::ptr::null_mut();
    percpu_counter_destroy(&mut svcrdma_stat_write);
    percpu_counter_destroy(&mut svcrdma_stat_sq_starve);
    percpu_counter_destroy(&mut svcrdma_stat_recv);
    percpu_counter_destroy(&mut svcrdma_stat_read);
}

unsafe fn svc_rdma_proc_init() -> c_int {
    let mut rc;
    if !svcrdma_table_header.is_null() { return 0; }
    rc = percpu_counter_init(&mut svcrdma_stat_read, 0, GFP_KERNEL);
    if rc != 0 { return rc; }
    rc = percpu_counter_init(&mut svcrdma_stat_recv, 0, GFP_KERNEL);
    if rc != 0 { percpu_counter_destroy(&mut svcrdma_stat_read); return rc; }
    rc = percpu_counter_init(&mut svcrdma_stat_sq_starve, 0, GFP_KERNEL);
    if rc != 0 { percpu_counter_destroy(&mut svcrdma_stat_recv); percpu_counter_destroy(&mut svcrdma_stat_read); return rc; }
    rc = percpu_counter_init(&mut svcrdma_stat_write, 0, GFP_KERNEL);
    if rc != 0 { percpu_counter_destroy(&mut svcrdma_stat_sq_starve); percpu_counter_destroy(&mut svcrdma_stat_recv); percpu_counter_destroy(&mut svcrdma_stat_read); return rc; }
    svcrdma_table_header = register_sysctl(b"sunrpc/svc_rdma\0".as_ptr() as *const c_char, svcrdma_parm_table.as_mut_ptr());
    if svcrdma_table_header.is_null() {
        percpu_counter_destroy(&mut svcrdma_stat_write); percpu_counter_destroy(&mut svcrdma_stat_sq_starve);
        percpu_counter_destroy(&mut svcrdma_stat_recv); percpu_counter_destroy(&mut svcrdma_stat_read); return -12;
    }
    0
}

pub unsafe fn svc_rdma_cleanup() {
    svc_unreg_xprt_class(&svc_rdma_class);
    svc_rdma_proc_cleanup();
    dprintk(b"SVCRDMA Module Removed, deregister RPC RDMA transport\0".as_ptr() as *const c_char);
}

pub unsafe fn svc_rdma_init() -> c_int {
    let rc = svc_rdma_proc_init();
    if rc != 0 { return rc; }
    svc_reg_xprt_class(&svc_rdma_class);
    dprintk(b"SVCRDMA Module Init, register RPC RDMA transport\0".as_ptr() as *const c_char);
    dprintk(b"\tsvcrdma_ord      : %d\n\0".as_ptr() as *const c_char, svcrdma_ord);
    dprintk(b"\tmax_requests     : %u\n\0".as_ptr() as *const c_char, svcrdma_max_requests);
    dprintk(b"\tmax_bc_requests  : %u\n\0".as_ptr() as *const c_char, svcrdma_max_bc_requests);
    dprintk(b"\tmax_inline       : %d\n\0".as_ptr() as *const c_char, svcrdma_max_req_size);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
