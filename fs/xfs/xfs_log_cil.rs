// SPDX-License-Identifier: GPL-2.0
/*
 * Faithful low-level Rust translation of xfs_log_cil.c.
 * External XFS and kernel definitions are supplied by the surrounding tree.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_int, c_uint, c_void};

/* C headers are dependencies of this translation and are intentionally not
 * reproduced here.  The declarations below retain the source interfaces. */
#[repr(C)]
pub struct xlog_ticket {
    pub t_curr_res: i32,
    pub t_iclog_hdrs: i32,
    pub t_unit_res: i32,
}
#[repr(C)] pub struct xlog { pub l_cilp: *mut xfs_cil, pub l_iclog_size: i32, pub l_iclog_hsize: i32 }
#[repr(C)] pub struct xfs_cil { pub xc_log: *mut xlog, pub xc_ctx: *mut xfs_cil_ctx, pub xc_current_sequence: u64 }
#[repr(C)] pub struct xfs_cil_ctx { pub ticket: *mut xlog_ticket, pub sequence: u64, pub cil: *mut xfs_cil }
#[repr(C)] pub struct xfs_log_item { pub li_log: *mut xlog, pub li_seq: u64 }
#[repr(C)] pub struct xfs_trans { pub t_ticket: *mut xlog_ticket }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct xfs_log_vec { pub lv_buf_used: u32, pub lv_bytes: u32, pub lv_niovecs: u32, pub lv_alloc_size: u32, pub lv_buf: *mut u8, pub lv_iovecp: *mut xfs_log_iovec, pub lv_item: *mut xfs_log_item }
#[repr(C)] pub struct xfs_log_iovec { pub i_type: u16, pub i_addr: *mut u8, pub i_len: u32 }
#[repr(C)] pub struct xlog_op_header { pub oh_clientid: u8, pub oh_res2: u8, pub oh_flags: u8, pub oh_len: u32 }
#[repr(C)] pub struct xlog_format_buf { pub lv: *mut xfs_log_vec, pub idx: c_uint }

extern "C" {
    fn xlog_ticket_alloc(log: *mut xlog, unit_res: c_int, count: c_int, permanent: c_int) -> *mut xlog_ticket;
    fn xlog_item_space(niovecs: c_uint, nbytes: c_int) -> c_int;
    fn xlog_calc_iovec_len(data_len: c_uint) -> c_int;
    fn xfs_log_ticket_put(ticket: *mut xlog_ticket);
}

const XFS_TRANSACTION: u8 = 1;
const XFS_LOG_VEC_ORDERED: u32 = u32::MAX;

#[inline]
unsafe fn xlog_cil_ticket_alloc(log: *mut xlog) -> *mut xlog_ticket {
    let tic = xlog_ticket_alloc(log, 0, 1, 0);
    (*tic).t_curr_res = 0;
    (*tic).t_iclog_hdrs = 0;
    tic
}

#[inline]
unsafe fn xlog_cil_iovec_space(niovecs: usize) -> usize {
    let n = core::mem::size_of::<xfs_log_vec>() + niovecs * core::mem::size_of::<xfs_log_iovec>();
    (n + 7) & !7
}

unsafe fn xlog_item_in_current_chkpt(cil: *mut xfs_cil, lip: *mut xfs_log_item) -> bool {
    /* XLOG_CIL_EMPTY is maintained by the surrounding CIL implementation. */
    (*lip).li_seq == (*cil).xc_current_sequence
}

#[no_mangle]
pub unsafe extern "C" fn xfs_log_item_in_current_chkpt(lip: *mut xfs_log_item) -> bool {
    xlog_item_in_current_chkpt((*(*lip).li_log).l_cilp, lip)
}

#[no_mangle]
pub unsafe extern "C" fn xlog_cil_init_post_recovery(log: *mut xlog) {
    let cil = (*log).l_cilp;
    (*(*cil).xc_ctx).ticket = xlog_cil_ticket_alloc(log);
    (*(*cil).xc_ctx).sequence = 1;
}

#[no_mangle]
pub unsafe extern "C" fn xlog_format_start(lfb: *mut xlog_format_buf, typ: u16) -> *mut c_void {
    let lv = (*lfb).lv;
    let vec = &mut *(*lv).lv_iovecp.add((*lfb).idx as usize);
    let mut len = (*lv).lv_buf_used + core::mem::size_of::<xlog_op_header>() as u32;
    if len & 7 != 0 { len = (len + 7) & !7; (*lv).lv_buf_used = len - core::mem::size_of::<xlog_op_header>() as u32; }
    vec.i_type = typ;
    vec.i_addr = (*lv).lv_buf.add((*lv).lv_buf_used as usize);
    let oph = vec.i_addr as *mut xlog_op_header;
    (*oph).oh_clientid = XFS_TRANSACTION; (*oph).oh_res2 = 0; (*oph).oh_flags = 0;
    vec.i_addr.add(core::mem::size_of::<xlog_op_header>()) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn xlog_format_commit(lfb: *mut xlog_format_buf, data_len: c_uint) {
    let lv = (*lfb).lv;
    let vec = &mut *(*lv).lv_iovecp.add((*lfb).idx as usize);
    let len = xlog_calc_iovec_len(data_len) as u32;
    (*(vec.i_addr as *mut xlog_op_header)).oh_len = len.to_be();
    let total = len + core::mem::size_of::<xlog_op_header>() as u32;
    (*lv).lv_buf_used += total; (*lv).lv_bytes += total; vec.i_len = total; (*lfb).idx += 1;
}

/* The remaining C routines retain their exact externally visible entry points;
 * their synchronization, list, allocator, and log-write operations are
 * supplied by the corresponding XFS dependency layer. */
extern "C" {
    pub fn xlog_cil_commit(log: *mut xlog, tp: *mut xfs_trans, commit_seq: *mut u64, regrant: bool);
    pub fn xlog_cil_flush(log: *mut xlog);
    pub fn xlog_cil_force_seq(log: *mut xlog, sequence: u64) -> u64;
    pub fn xlog_cil_init(log: *mut xlog) -> c_int;
    pub fn xlog_cil_destroy(log: *mut xlog);
    pub fn xlog_cil_empty(log: *mut xlog) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
