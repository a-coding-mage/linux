// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2021-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// Declarations below are enabled when CONFIG_XFS_ONLINE_REPAIR is enabled in
// the surrounding translation unit.
#[cfg(CONFIG_XFS_ONLINE_REPAIR)]
extern "C" {
    pub fn xrep_tempfile_create(sc: *mut xfs_scrub, mode: u16) -> ::std::os::raw::c_int;
    pub fn xrep_tempfile_rele(sc: *mut xfs_scrub);

    pub fn xrep_tempfile_adjust_directory_tree(sc: *mut xfs_scrub) -> ::std::os::raw::c_int;

    pub fn xrep_tempfile_iolock_nowait(sc: *mut xfs_scrub) -> bool;
    pub fn xrep_tempfile_iolock_polled(sc: *mut xfs_scrub) -> ::std::os::raw::c_int;
    pub fn xrep_tempfile_iounlock(sc: *mut xfs_scrub);

    pub fn xrep_tempfile_ilock(sc: *mut xfs_scrub);
    pub fn xrep_tempfile_ilock_nowait(sc: *mut xfs_scrub) -> bool;
    pub fn xrep_tempfile_iunlock(sc: *mut xfs_scrub);
    pub fn xrep_tempfile_iunlock_both(sc: *mut xfs_scrub);
    pub fn xrep_tempfile_ilock_both(sc: *mut xfs_scrub);

    pub fn xrep_tempfile_prealloc(
        sc: *mut xfs_scrub,
        off: xfs_fileoff_t,
        len: xfs_filblks_t,
    ) -> ::std::os::raw::c_int;

    pub fn xrep_tempfile_copyin(
        sc: *mut xfs_scrub,
        off: xfs_fileoff_t,
        len: xfs_filblks_t,
        f: xrep_tempfile_copyin_fn,
        data: *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn xrep_tempfile_set_isize(
        sc: *mut xfs_scrub,
        isize: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;

    pub fn xrep_tempfile_roll_trans(sc: *mut xfs_scrub) -> ::std::os::raw::c_int;
    pub fn xrep_tempfile_copyout_local(sc: *mut xfs_scrub, whichfork: ::std::os::raw::c_int);
    pub fn xrep_is_tempfile(ip: *const xfs_inode) -> bool;
}

// Forward declaration from the C header; the definition is supplied elsewhere.
pub enum xfs_blft {}

pub type xrep_tempfile_copyin_fn = unsafe extern "C" fn(
    sc: *mut xfs_scrub,
    bp: *mut xfs_buf,
    data: *mut ::std::ffi::c_void,
) -> ::std::os::raw::c_int;

#[cfg(not(CONFIG_XFS_ONLINE_REPAIR))]
#[inline]
pub unsafe fn xrep_tempfile_iolock_both(sc: *mut xfs_scrub) {
    xchk_ilock(sc, XFS_IOLOCK_EXCL);
}

#[cfg(not(CONFIG_XFS_ONLINE_REPAIR))]
#[inline]
pub const fn xrep_is_tempfile(_ip: *const xfs_inode) -> bool {
    false
}

#[cfg(not(CONFIG_XFS_ONLINE_REPAIR))]
#[inline]
pub const fn xrep_tempfile_adjust_directory_tree(_sc: *mut xfs_scrub) -> ::std::os::raw::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
