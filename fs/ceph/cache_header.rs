/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Ceph cache definitions.
 *
 *  Copyright (C) 2013 by Adfin Solutions, Inc. All Rights Reserved.
 *  Written by Milosz Tanski (milosz@adfin.com)
 */

/* Translated from C header cache.h.  External kernel types and functions are
 * supplied by the surrounding translation unit. */

#[cfg(feature = "CONFIG_CEPH_FSCACHE")]
extern "C" {
    pub fn ceph_fscache_register_fs(fsc: *mut ceph_fs_client, fc: *mut fs_context) -> ::std::os::raw::c_int;
    pub fn ceph_fscache_unregister_fs(fsc: *mut ceph_fs_client);

    pub fn ceph_fscache_register_inode_cookie(inode: *mut inode);
    pub fn ceph_fscache_unregister_inode_cookie(ci: *mut ceph_inode_info);

    pub fn ceph_fscache_use_cookie(inode: *mut inode, will_modify: bool);
    pub fn ceph_fscache_unuse_cookie(inode: *mut inode, update: bool);

    pub fn ceph_fscache_update(inode: *mut inode);
    pub fn ceph_fscache_invalidate(inode: *mut inode, dio_write: bool);
}

#[cfg(feature = "CONFIG_CEPH_FSCACHE")]
#[inline]
pub unsafe fn ceph_fscache_cookie(ci: *mut ceph_inode_info) -> *mut fscache_cookie {
    netfs_i_cookie(&mut (*ci).netfs)
}

#[cfg(feature = "CONFIG_CEPH_FSCACHE")]
#[inline]
pub unsafe fn ceph_fscache_resize(inode: *mut inode, to: loff_t) {
    let ci = ceph_inode(inode);
    let cookie = ceph_fscache_cookie(ci);
    if !cookie.is_null() {
        ceph_fscache_use_cookie(inode, true);
        fscache_resize_cookie(cookie, to);
        ceph_fscache_unuse_cookie(inode, true);
    }
}

#[cfg(feature = "CONFIG_CEPH_FSCACHE")]
#[inline]
pub unsafe fn ceph_fscache_unpin_writeback(
    inode: *mut inode,
    wbc: *mut writeback_control,
) -> ::std::os::raw::c_int {
    netfs_unpin_writeback(inode, wbc)
}

#[cfg(feature = "CONFIG_CEPH_FSCACHE")]
#[inline]
pub unsafe fn ceph_is_cache_enabled(inode: *mut inode) -> bool {
    fscache_cookie_enabled(ceph_fscache_cookie(ceph_inode(inode)))
}

/* ceph_fscache_dirty_folio is the netfs_dirty_folio symbol. */
#[cfg(feature = "CONFIG_CEPH_FSCACHE")]
pub use netfs_dirty_folio as ceph_fscache_dirty_folio;

#[cfg(not(feature = "CONFIG_CEPH_FSCACHE"))]
#[inline]
pub unsafe fn ceph_fscache_register_fs(
    _fsc: *mut ceph_fs_client,
    _fc: *mut fs_context,
) -> ::std::os::raw::c_int {
    0
}

#[cfg(not(feature = "CONFIG_CEPH_FSCACHE"))]
#[inline]
pub unsafe fn ceph_fscache_unregister_fs(_fsc: *mut ceph_fs_client) {}

#[cfg(not(feature = "CONFIG_CEPH_FSCACHE"))]
#[inline]
pub unsafe fn ceph_fscache_register_inode_cookie(_inode: *mut inode) {}

#[cfg(not(feature = "CONFIG_CEPH_FSCACHE"))]
#[inline]
pub unsafe fn ceph_fscache_unregister_inode_cookie(_ci: *mut ceph_inode_info) {}

#[cfg(not(feature = "CONFIG_CEPH_FSCACHE"))]
#[inline]
pub unsafe fn ceph_fscache_use_cookie(_inode: *mut inode, _will_modify: bool) {}

#[cfg(not(feature = "CONFIG_CEPH_FSCACHE"))]
#[inline]
pub unsafe fn ceph_fscache_unuse_cookie(_inode: *mut inode, _update: bool) {}

#[cfg(not(feature = "CONFIG_CEPH_FSCACHE"))]
#[inline]
pub unsafe fn ceph_fscache_update(_inode: *mut inode) {}

#[cfg(not(feature = "CONFIG_CEPH_FSCACHE"))]
#[inline]
pub unsafe fn ceph_fscache_invalidate(_inode: *mut inode, _dio_write: bool) {}

#[cfg(not(feature = "CONFIG_CEPH_FSCACHE"))]
#[inline]
pub unsafe fn ceph_fscache_cookie(_ci: *mut ceph_inode_info) -> *mut fscache_cookie {
    ::std::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_CEPH_FSCACHE"))]
#[inline]
pub unsafe fn ceph_fscache_resize(_inode: *mut inode, _to: loff_t) {}

#[cfg(not(feature = "CONFIG_CEPH_FSCACHE"))]
#[inline]
pub unsafe fn ceph_fscache_unpin_writeback(
    _inode: *mut inode,
    _wbc: *mut writeback_control,
) -> ::std::os::raw::c_int {
    0
}

/* ceph_fscache_dirty_folio is the filemap_dirty_folio symbol. */
#[cfg(not(feature = "CONFIG_CEPH_FSCACHE"))]
pub use filemap_dirty_folio as ceph_fscache_dirty_folio;

#[cfg(not(feature = "CONFIG_CEPH_FSCACHE"))]
#[inline]
pub unsafe fn ceph_is_cache_enabled(_inode: *mut inode) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
