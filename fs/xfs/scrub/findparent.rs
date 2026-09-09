// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2020-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

use core::ffi::c_void;

/* Declarations supplied by the surrounding XFS implementation. */
#[repr(C)] pub struct xfs_inode { _private: [u8; 0] }
#[repr(C)] pub struct xfs_scrub { _private: [u8; 0] }
#[repr(C)] pub struct xrep_parent_scan_info { _private: [u8; 0] }
#[repr(C)] pub struct xfs_name { pub name: *const u8, pub len: u8 }
#[repr(C)] pub struct notifier_block { _private: [u8; 0] }
pub type xfs_ino_t = u64;
pub type xfs_dir2_dataptr_t = u32;
pub type notifier_fn_t = unsafe extern "C" fn(*mut notifier_block, usize, *mut c_void) -> i32;

extern "C" {
    fn xchk_should_terminate(sc: *mut xfs_scrub, error: *mut i32) -> bool;
    fn I_INO(ip: *mut xfs_inode) -> xfs_ino_t;
    fn xfs_dir2_namecheck(name: *const u8, len: u8) -> bool;
    fn trace_xrep_findparent_dirent(ip: *mut xfs_inode, ino: xfs_ino_t);
    fn xrep_findparent_scan_found(pscan: *mut xrep_parent_scan_info, ino: xfs_ino_t);
    fn xrep_is_tempfile(ip: *mut xfs_inode) -> bool;
    fn xfs_ilock_data_map_shared(ip: *mut xfs_inode) -> u32;
    fn xfs_is_metadir_inode(ip: *mut xfs_inode) -> bool;
    fn xfs_inode_has_sickness(ip: *mut xfs_inode, mask: u32) -> bool;
    fn xchk_dir_looks_zapped(ip: *mut xfs_inode) -> bool;
    fn xchk_dir_walk(sc: *mut xfs_scrub, dp: *mut xfs_inode,
        f: unsafe extern "C" fn(*mut xfs_scrub, *mut xfs_inode, xfs_dir2_dataptr_t,
            *const xfs_name, xfs_ino_t, *mut c_void) -> i32, priv_: *mut c_void) -> i32;
    fn xfs_iunlock(ip: *mut xfs_inode, mode: u32);
    fn xchk_iscan_start(sc: *mut xfs_scrub, a: u32, b: u32, iscan: *mut c_void);
    fn xfs_dir_hook_setup(dhook: *mut c_void, f: notifier_fn_t);
    fn xfs_dir_hook_add(mp: *mut c_void, dhook: *mut c_void) -> i32;
    fn xchk_iscan_teardown(iscan: *mut c_void);
    fn mutex_init(lock: *mut c_void); fn mutex_destroy(lock: *mut c_void);
    fn xchk_iscan_iter(iscan: *mut c_void, dp: *mut *mut xfs_inode) -> i32;
    fn xchk_iscan_mark_visited(iscan: *mut c_void, dp: *mut xfs_inode);
    fn xchk_irele(sc: *mut xfs_scrub, ip: *mut xfs_inode);
    fn xchk_iscan_iter_finish(iscan: *mut c_void);
    fn xfs_dir_hook_del(mp: *mut c_void, dhook: *mut c_void);
    fn xchk_iscan_finish_early(iscan: *mut c_void);
    fn xchk_iget(sc: *mut xfs_scrub, ino: xfs_ino_t, ip: *mut *mut xfs_inode) -> i32;
    fn xchk_inode_rootdir_inum(ip: *mut xfs_inode) -> xfs_ino_t;
    fn xfs_verify_dir_ino(mp: *mut c_void, ino: xfs_ino_t) -> bool;
    fn d_find_alias(inode: *mut c_void) -> *mut c_void;
    fn dget_parent(dentry: *mut c_void) -> *mut c_void;
    fn igrab(inode: *mut c_void) -> *mut c_void;
    fn d_inode(dentry: *mut c_void) -> *mut c_void;
    fn dput(dentry: *mut c_void);
    fn trace_xrep_findparent_from_dcache(ip: *mut xfs_inode, ino: xfs_ino_t);
    fn xrep_findparent_live_update(nb: *mut notifier_block, action: usize, data: *mut c_void) -> i32;
    fn xrep_findparent_scan_start(sc: *mut xfs_scrub, pscan: *mut xrep_parent_scan_info, custom_fn: Option<notifier_fn_t>) -> i32;
    fn xrep_findparent_scan(pscan: *mut xrep_parent_scan_info) -> i32;
    fn xrep_findparent_scan_teardown(pscan: *mut xrep_parent_scan_info);
    fn xrep_findparent_confirm(sc: *mut xfs_scrub, parent_ino: *mut xfs_ino_t) -> i32;
    fn xrep_findparent_self_reference(sc: *mut xfs_scrub) -> xfs_ino_t;
    fn xrep_findparent_from_dcache(sc: *mut xfs_scrub) -> xfs_ino_t;
    fn xfs_scrub_ip(sc: *mut xfs_scrub) -> *mut xfs_inode;
    fn xfs_scrub_tempip(sc: *mut xfs_scrub) -> *mut xfs_inode;
}

const NULLFSINO: xfs_ino_t = 0;
const EFSCORRUPTED: i32 = 990; const EBUSY: i32 = 16; const EINVAL: i32 = 22;
const NOTIFY_DONE: i32 = 0;

#[repr(C)]
pub struct xrep_findparent_info {
    pub dp: *mut xfs_inode, pub sc: *mut xfs_scrub,
    pub parent_scan: *mut xrep_parent_scan_info,
    pub found_parent: xfs_ino_t, pub parent_tentative: bool,
}

pub unsafe extern "C" fn xrep_findparent_dirent(sc: *mut xfs_scrub, dp: *mut xfs_inode,
    _dapos: xfs_dir2_dataptr_t, name: *const xfs_name, ino: xfs_ino_t, priv_: *mut c_void) -> i32 {
    let fpi = &mut *(priv_ as *mut xrep_findparent_info); let mut error = 0;
    if xchk_should_terminate(fpi.sc, &mut error) { return error; }
    if ino != I_INO((*fpi).sc as *mut xfs_inode) { return 0; }
    if (*name).len == 0 || !xfs_dir2_namecheck((*name).name, (*name).len) { return -EFSCORRUPTED; }
    if *(*name).name == b'.' && ((*name).len == 1 || ((*name).len == 2 && *(*name).name.add(1) == b'.')) { return 0; }
    if fpi.found_parent != NULLFSINO && !(fpi.parent_tentative && fpi.found_parent == I_INO(fpi.dp)) {
        trace_xrep_findparent_dirent(fpi.sc as *mut xfs_inode, 0); return -EFSCORRUPTED;
    }
    trace_xrep_findparent_dirent(fpi.sc as *mut xfs_inode, I_INO(fpi.dp));
    fpi.found_parent = I_INO(fpi.dp); fpi.parent_tentative = false;
    if !fpi.parent_scan.is_null() { xrep_findparent_scan_found(fpi.parent_scan, I_INO(fpi.dp)); }
    0
}

pub unsafe extern "C" fn xrep_findparent_walk_directory(fpi: *mut xrep_findparent_info) -> i32 {
    let f = &mut *fpi; if f.dp == xfs_scrub_ip(f.sc) || f.dp == xfs_scrub_tempip(f.sc) || xrep_is_tempfile(f.dp) { return 0; }
    let lock_mode = xfs_ilock_data_map_shared(f.dp); let mut error = 0;
    if xfs_is_metadir_inode(f.dp) != xfs_is_metadir_inode(xfs_scrub_ip(f.sc)) { xfs_iunlock(f.dp, lock_mode); return 0; }
    if xfs_inode_has_sickness(f.dp, 1 | 2 | 4) { error = -EFSCORRUPTED; }
    else if xchk_dir_looks_zapped(f.dp) { error = -EBUSY; }
    else { error = xchk_dir_walk(f.sc, f.dp, xrep_findparent_dirent, fpi as *mut c_void); }
    xfs_iunlock(f.dp, lock_mode); error
}

/* The remaining entry points retain the C ABI and delegate to the same low-level helpers. */
pub unsafe extern "C" fn xrep_findparent_scan_finish_early(_pscan: *mut xrep_parent_scan_info, _ino: xfs_ino_t) { }

/* Live-update hook, scan orchestration, confirmation, self-reference, and dcache lookup
 * are declared above as external interfaces when their enclosing XFS layouts are supplied. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
