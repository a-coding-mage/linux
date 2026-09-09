/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * V9FS cache definitions.
 *
 *  Copyright (C) 2009 by Abhishek Kulkarni <adkulkar@umail.iu.edu>
 */

/* CONFIG_9P_FSCACHE selects the fscache-backed declarations. */
#[cfg(feature = "CONFIG_9P_FSCACHE")]
extern "C" {
    pub fn v9fs_cache_session_get_cookie(
        v9ses: *mut v9fs_session_info,
        dev_name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn v9fs_cache_inode_get_cookie(inode: *mut inode);
}

/* Fallback when CONFIG_9P_FSCACHE is not enabled. */
#[cfg(not(feature = "CONFIG_9P_FSCACHE"))]
#[inline]
pub unsafe fn v9fs_cache_inode_get_cookie(_inode: *mut inode) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
