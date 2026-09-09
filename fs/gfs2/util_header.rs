/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
 */

// Translated from util.h. Linux includes and symbols from incore.h are external dependencies.

#[macro_export]
macro_rules! fs_emerg {
    ($fs:expr, $fmt:expr $(, $arg:expr)*) => { pr_emerg!("fsid=%s: " $fmt $(, $arg)*, ($fs).sd_fsname) };
}
#[macro_export]
macro_rules! fs_warn {
    ($fs:expr, $fmt:expr $(, $arg:expr)*) => { pr_warn!("fsid=%s: " $fmt $(, $arg)*, ($fs).sd_fsname) };
}
#[macro_export]
macro_rules! fs_err {
    ($fs:expr, $fmt:expr $(, $arg:expr)*) => { pr_err!("fsid=%s: " $fmt $(, $arg)*, ($fs).sd_fsname) };
}
#[macro_export]
macro_rules! fs_info {
    ($fs:expr, $fmt:expr $(, $arg:expr)*) => { pr_info!("fsid=%s: " $fmt $(, $arg)*, ($fs).sd_fsname) };
}

extern "C" {
    pub fn gfs2_assert_i(sdp: *mut gfs2_sbd);
    pub fn gfs2_assert_withdraw_i(sdp: *mut gfs2_sbd, assertion: *mut c_char,
        function: *const c_char, file: *mut c_char, line: c_uint);
    pub fn gfs2_assert_warn_i(sdp: *mut gfs2_sbd, assertion: *mut c_char,
        function: *const c_char, file: *mut c_char, line: c_uint);
    pub fn gfs2_consist_i(sdp: *mut gfs2_sbd, function: *const c_char,
        file: *mut c_char, line: c_uint);
    pub fn gfs2_consist_inode_i(ip: *mut gfs2_inode, function: *const c_char,
        file: *mut c_char, line: c_uint);
    pub fn gfs2_consist_rgrpd_i(rgd: *mut gfs2_rgrpd, function: *const c_char,
        file: *mut c_char, line: c_uint);
    pub fn gfs2_meta_check_ii(sdp: *mut gfs2_sbd, bh: *mut buffer_head,
        function: *const c_char, file: *mut c_char, line: c_uint);
    pub fn gfs2_metatype_check_ii(sdp: *mut gfs2_sbd, bh: *mut buffer_head,
        typ: u16, t: u16, function: *const c_char, file: *mut c_char, line: c_uint);
    pub fn gfs2_io_error_i(sdp: *mut gfs2_sbd, function: *const c_char,
        file: *mut c_char, line: c_uint);
    pub fn check_journal_clean(sdp: *mut gfs2_sbd, jd: *mut gfs2_jdesc, verbose: bool) -> c_int;
    pub fn gfs2_freeze_lock_shared(sdp: *mut gfs2_sbd) -> c_int;
    pub fn gfs2_freeze_unlock(sdp: *mut gfs2_sbd);
    pub fn gfs2_io_error_bh_i(sdp: *mut gfs2_sbd, bh: *mut buffer_head,
        function: *const c_char, file: *mut c_char, line: c_uint);
    pub fn gfs2_lm(sdp: *mut gfs2_sbd, fmt: *const c_char, ...);
    pub fn gfs2_withdraw_func(work: *mut work_struct);
    pub fn gfs2_withdraw(sdp: *mut gfs2_sbd);
}

#[macro_export]
macro_rules! gfs2_assert { ($sdp:expr, $assertion:expr) => {{ if !($assertion) { unsafe { gfs2_assert_i($sdp); BUG!(); } } }}; }
#[macro_export]
macro_rules! gfs2_assert_withdraw { ($sdp:expr, $assertion:expr) => {{ let b = $assertion; if !b { unsafe { gfs2_assert_withdraw_i($sdp, stringify!($assertion).as_ptr() as *mut c_char, concat!(module_path!(), "\0").as_ptr() as *const c_char, file!().as_ptr() as *mut c_char, line!()); } } !b }}; }
#[macro_export]
macro_rules! gfs2_assert_warn { ($sdp:expr, $assertion:expr) => {{ let b = $assertion; if !b { unsafe { gfs2_assert_warn_i($sdp, stringify!($assertion).as_ptr() as *mut c_char, concat!(module_path!(), "\0").as_ptr() as *const c_char, file!().as_ptr() as *mut c_char, line!()); } } !b }}; }
#[macro_export] macro_rules! gfs2_consist { ($sdp:expr) => { unsafe { gfs2_consist_i($sdp, concat!(module_path!(), "\0").as_ptr() as *const c_char, file!().as_ptr() as *mut c_char, line!()) } }; }
#[macro_export] macro_rules! gfs2_consist_inode { ($ip:expr) => { unsafe { gfs2_consist_inode_i($ip, concat!(module_path!(), "\0").as_ptr() as *const c_char, file!().as_ptr() as *mut c_char, line!()) } }; }
#[macro_export] macro_rules! gfs2_consist_rgrpd { ($rgd:expr) => { unsafe { gfs2_consist_rgrpd_i($rgd, concat!(module_path!(), "\0").as_ptr() as *const c_char, file!().as_ptr() as *mut c_char, line!()) } }; }

#[inline]
pub unsafe fn gfs2_meta_check(sdp: *mut gfs2_sbd, bh: *mut buffer_head) -> c_int {
    let mh = (*bh).b_data as *mut gfs2_meta_header;
    let magic = be32_to_cpu((*mh).mh_magic);
    if magic != GFS2_MAGIC { fs_err!(sdp, "Magic number missing at %llu\n", (*bh).b_blocknr); return -EIO; }
    0
}

#[inline]
pub unsafe fn gfs2_metatype_check_i(sdp: *mut gfs2_sbd, bh: *mut buffer_head, typ: u16,
    function: *const c_char, file: *mut c_char, line: c_uint) -> c_int {
    let mh = (*bh).b_data as *mut gfs2_meta_header;
    let magic = be32_to_cpu((*mh).mh_magic);
    let t = be32_to_cpu((*mh).mh_type);
    if magic != GFS2_MAGIC { gfs2_meta_check_ii(sdp, bh, function, file, line); return -EIO; }
    if t != typ { gfs2_metatype_check_ii(sdp, bh, typ, t, function, file, line); return -EIO; }
    0
}

#[macro_export] macro_rules! gfs2_metatype_check { ($sdp:expr, $bh:expr, $typ:expr) => { unsafe { gfs2_metatype_check_i($sdp, $bh, $typ, concat!(module_path!(), "\0").as_ptr() as *const c_char, file!().as_ptr() as *mut c_char, line!()) } }; }
#[inline] pub unsafe fn gfs2_metatype_set(bh: *mut buffer_head, typ: u16, format: u16) { let mh = (*bh).b_data as *mut gfs2_meta_header; (*mh).mh_type = cpu_to_be32(typ); (*mh).mh_format = cpu_to_be32(format); }
#[macro_export] macro_rules! gfs2_io_error { ($sdp:expr) => { unsafe { gfs2_io_error_i($sdp, concat!(module_path!(), "\0").as_ptr() as *const c_char, file!().as_ptr() as *mut c_char, line!()) } }; }
#[macro_export] macro_rules! gfs2_io_error_bh { ($sdp:expr, $bh:expr) => { unsafe { gfs2_io_error_bh_i($sdp, $bh, concat!(module_path!(), "\0").as_ptr() as *const c_char, file!().as_ptr() as *mut c_char, line!()) } }; }

extern "C" {
    pub static mut gfs2_glock_cachep: *mut kmem_cache;
    pub static mut gfs2_glock_aspace_cachep: *mut kmem_cache;
    pub static mut gfs2_inode_cachep: *mut kmem_cache;
    pub static mut gfs2_bufdata_cachep: *mut kmem_cache;
    pub static mut gfs2_rgrpd_cachep: *mut kmem_cache;
    pub static mut gfs2_quotad_cachep: *mut kmem_cache;
    pub static mut gfs2_qadata_cachep: *mut kmem_cache;
    pub static mut gfs2_trans_cachep: *mut kmem_cache;
    pub static mut gfs2_page_pool: *mut mempool_t;
    pub static mut gfs2_control_wq: *mut workqueue_struct;
}

#[inline] pub unsafe fn gfs2_tune_get_i(gt: *mut gfs2_tune, p: *mut c_uint) -> c_uint { spin_lock(&mut (*gt).gt_spin); let x = *p; spin_unlock(&mut (*gt).gt_spin); x }
#[inline] pub unsafe fn gfs2_withdrawn(sdp: *mut gfs2_sbd) -> bool { test_bit(SDF_WITHDRAWN, &(*sdp).sd_flags) != 0 }
#[macro_export] macro_rules! gfs2_tune_get { ($sdp:expr, $field:ident) => { unsafe { gfs2_tune_get_i(&mut (*$sdp).sd_tune, &mut (*$sdp).sd_tune.$field) } }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
