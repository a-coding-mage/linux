// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of util.c. */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

// Kernel and GFS2 dependencies are supplied by the surrounding translation.

extern "C" {
    static mut gfs2_glock_cachep: *mut kmem_cache;
    static mut gfs2_glock_aspace_cachep: *mut kmem_cache;
    static mut gfs2_inode_cachep: *mut kmem_cache;
    static mut gfs2_bufdata_cachep: *mut kmem_cache;
    static mut gfs2_rgrpd_cachep: *mut kmem_cache;
    static mut gfs2_quotad_cachep: *mut kmem_cache;
    static mut gfs2_qadata_cachep: *mut kmem_cache;
    static mut gfs2_trans_cachep: *mut kmem_cache;
    static mut gfs2_page_pool: *mut mempool_t;
}

#[repr(C)] pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)] pub struct mempool_t { _private: [u8; 0] }
#[repr(C)] pub struct gfs2_sbd { _private: [u8; 0] }
#[repr(C)] pub struct gfs2_jdesc { _private: [u8; 0] }
#[repr(C)] pub struct gfs2_inode { _private: [u8; 0] }
#[repr(C)] pub struct gfs2_rgrpd { _private: [u8; 0] }
#[repr(C)] pub struct buffer_head { b_blocknr: u64 }
#[repr(C)] pub struct gfs2_log_header_host { lh_flags: u32 }
#[repr(C)] pub struct gfs2_holder { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }

extern "C" {
    fn fs_emerg(sdp: *mut gfs2_sbd, fmt: *const c_char, ...);
    fn fs_err(sdp: *mut gfs2_sbd, fmt: *const c_char, ...);
    fn fs_warn(sdp: *mut gfs2_sbd, fmt: *const c_char, ...);
    fn gfs2_glock_nq_init(ip: *mut c_void, state: c_int, flags: c_int, gh: *mut gfs2_holder) -> c_int;
    fn gfs2_glock_dq_uninit(gh: *mut gfs2_holder);
    fn gfs2_holder_initialized(gh: *mut gfs2_holder) -> bool;
    fn gfs2_jdesc_check(jd: *mut gfs2_jdesc) -> c_int;
    fn gfs2_find_jhead(jd: *mut gfs2_jdesc, head: *mut gfs2_log_header_host) -> c_int;
    fn gfs2_ail_drain(sdp: *mut gfs2_sbd);
    fn gfs2_log_is_empty(sdp: *mut gfs2_sbd) -> bool;
    fn gfs2_withdraw_glocks(sdp: *mut gfs2_sbd);
    fn gfs2_tune_get(sdp: *mut gfs2_sbd, tune: c_int) -> c_long;
    fn gfs2_offline_uevent(sdp: *mut gfs2_sbd) -> bool;
    fn gfs2_withdraw(sdp: *mut gfs2_sbd);
    fn gfs2_withdrawn(sdp: *mut gfs2_sbd) -> bool;
    fn gfs2_dump_glock(a: *mut c_void, gl: *mut c_void, n: c_int);
    fn gfs2_rgrp_dump(a: *mut c_void, rgd: *mut gfs2_rgrpd, id: *const c_char);
    fn dump_stack();
    fn schedule_work(work: *mut work_struct);
    fn panic(fmt: *const c_char, ... ) -> !;
}

pub unsafe fn gfs2_assert_i(sdp: *mut gfs2_sbd) { fs_emerg(sdp, c"fatal assertion failed\n".as_ptr()); }

pub unsafe fn check_journal_clean(sdp: *mut gfs2_sbd, jd: *mut gfs2_jdesc, verbose: bool) -> c_int {
    let mut error: c_int;
    let mut j_gh = core::mem::MaybeUninit::<gfs2_holder>::zeroed().assume_init();
    let mut head = core::mem::MaybeUninit::<gfs2_log_header_host>::zeroed().assume_init();
    let ip = jd as *mut c_void;
    error = gfs2_glock_nq_init(ip, 1, 0, &mut j_gh);
    if error != 0 { if verbose { fs_err(sdp, c"Error %d locking journal for spectator mount.\n".as_ptr(), error); } return -1; }
    error = gfs2_jdesc_check(jd);
    if error != 0 { if verbose { fs_err(sdp, c"Error checking journal for spectator mount.\n".as_ptr()); } gfs2_glock_dq_uninit(&mut j_gh); return error; }
    error = gfs2_find_jhead(jd, &mut head);
    if error != 0 { if verbose { fs_err(sdp, c"Error parsing journal for spectator mount.\n".as_ptr()); } gfs2_glock_dq_uninit(&mut j_gh); return error; }
    if head.lh_flags & 1 == 0 { error = -1; if verbose { fs_err(sdp, c"Journal is dirty, so the first mounter must not be a spectator.\n".as_ptr()); } }
    gfs2_glock_dq_uninit(&mut j_gh); error
}

pub unsafe fn gfs2_freeze_lock_shared(sdp: *mut gfs2_sbd) -> c_int { gfs2_glock_nq_init(sdp as *mut c_void, 1, 0, core::ptr::null_mut()) }
pub unsafe fn gfs2_freeze_unlock(_sdp: *mut gfs2_sbd) {}

unsafe fn do_withdraw(sdp: *mut gfs2_sbd) { gfs2_ail_drain(sdp); gfs2_withdraw_glocks(sdp); }
pub unsafe fn gfs2_lm(sdp: *mut gfs2_sbd, _fmt: *const c_char, _args: ...) { fs_err(sdp, c"filesystem message\n".as_ptr()); }
pub unsafe fn gfs2_withdraw_func(_work: *mut work_struct) { }
pub unsafe fn gfs2_withdraw(sdp: *mut gfs2_sbd) { do_withdraw(sdp); }

pub unsafe fn gfs2_assert_withdraw_i(sdp: *mut gfs2_sbd, assertion: *mut c_char, function: *const c_char, file: *mut c_char, line: c_uint) { if gfs2_withdrawn(sdp) { return; } fs_err(sdp, c"fatal: assertion failed - function = %s, file = %s, line = %u\n".as_ptr(), assertion, function, file, line); gfs2_withdraw(sdp); dump_stack(); }
pub unsafe fn gfs2_assert_warn_i(_sdp: *mut gfs2_sbd, _assertion: *mut c_char, _function: *const c_char, _file: *mut c_char, _line: c_uint) { dump_stack(); }
pub unsafe fn gfs2_consist_i(sdp: *mut gfs2_sbd, _function: *const c_char, _file: *mut c_char, _line: c_uint) { gfs2_withdraw(sdp); }
pub unsafe fn gfs2_consist_inode_i(ip: *mut gfs2_inode, _function: *const c_char, _file: *mut c_char, _line: c_uint) { gfs2_dump_glock(core::ptr::null_mut(), ip as *mut c_void, 1); }
pub unsafe fn gfs2_consist_rgrpd_i(rgd: *mut gfs2_rgrpd, _function: *const c_char, _file: *mut c_char, _line: c_uint) { gfs2_rgrp_dump(core::ptr::null_mut(), rgd, c"fsid=: ".as_ptr()); }
pub unsafe fn gfs2_meta_check_ii(sdp: *mut gfs2_sbd, _bh: *mut buffer_head, _function: *const c_char, _file: *mut c_char, _line: c_uint) { gfs2_withdraw(sdp); }
pub unsafe fn gfs2_metatype_check_ii(sdp: *mut gfs2_sbd, _bh: *mut buffer_head, _ty: u16, _t: u16, _function: *const c_char, _file: *mut c_char, _line: c_uint) { gfs2_withdraw(sdp); }
pub unsafe fn gfs2_io_error_i(sdp: *mut gfs2_sbd, _function: *const c_char, _file: *mut c_char, _line: c_uint) { gfs2_withdraw(sdp); }
pub unsafe fn gfs2_io_error_bh_i(sdp: *mut gfs2_sbd, bh: *mut buffer_head, _function: *const c_char, _file: *mut c_char, _line: c_uint) { if !gfs2_withdrawn(sdp) { fs_err(sdp, c"fatal: I/O error - block = %llu\n".as_ptr(), (*bh).b_blocknr); gfs2_withdraw(sdp); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
