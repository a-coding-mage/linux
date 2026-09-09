/* SPDX-License-Identifier: LGPL-2.1 */
/*
 *   CIFS filesystem cache interface definitions
 *
 *   Copyright (c) 2010 Novell, Inc.
 *   Authors(s): Suresh Jayaraman (sjayaraman@suse.de>
 *
 */

// Dependencies supplied by the surrounding kernel/CIFS translation.

#[repr(C, packed)]
pub struct cifs_fscache_volume_coherency_data {
    pub resource_id: __le64,
    pub vol_create_time: __le64,
    pub vol_serial_number: __le32,
}

#[repr(C)]
pub struct cifs_fscache_inode_coherency_data {
    pub last_write_time_sec: __le64,
    pub last_change_time_sec: __le64,
    pub last_write_time_nsec: __le32,
    pub last_change_time_nsec: __le32,
}

#[cfg(CONFIG_CIFS_FSCACHE)]
extern "C" {
    pub fn cifs_fscache_get_super_cookie(tcon: *mut cifs_tcon) -> ::core::ffi::c_int;
    pub fn cifs_fscache_release_super_cookie(tcon: *mut cifs_tcon);
    pub fn cifs_fscache_get_inode_cookie(inode: *mut inode);
    pub fn cifs_fscache_unuse_inode_cookie(inode: *mut inode, update: bool);
    pub fn cifs_fscache_release_inode_cookie(inode: *mut inode);
}

#[cfg(CONFIG_CIFS_FSCACHE)]
#[inline]
pub unsafe fn cifs_fscache_fill_coherency(
    inode: *mut inode,
    cd: *mut cifs_fscache_inode_coherency_data,
) {
    let ctime: timespec64 = inode_get_ctime(inode);
    let mtime: timespec64 = inode_get_mtime(inode);

    core::ptr::write_bytes(cd, 0, 1);
    (*cd).last_write_time_sec = cpu_to_le64(mtime.tv_sec);
    (*cd).last_write_time_nsec = cpu_to_le32(mtime.tv_nsec);
    (*cd).last_change_time_sec = cpu_to_le64(ctime.tv_sec);
    (*cd).last_change_time_nsec = cpu_to_le32(ctime.tv_nsec);
}

#[cfg(CONFIG_CIFS_FSCACHE)]
#[inline]
pub unsafe fn cifs_inode_cookie(inode: *mut inode) -> *mut fscache_cookie {
    netfs_i_cookie(&mut (*CIFS_I(inode)).netfs)
}

#[cfg(CONFIG_CIFS_FSCACHE)]
#[inline]
pub unsafe fn cifs_invalidate_cache(inode: *mut inode, flags: ::core::ffi::c_uint) {
    let mut cd = core::mem::MaybeUninit::<cifs_fscache_inode_coherency_data>::uninit();
    cifs_fscache_fill_coherency(inode, cd.as_mut_ptr());
    fscache_invalidate(
        cifs_inode_cookie(inode),
        cd.as_ptr(),
        i_size_read(inode),
        flags,
    );
}

#[cfg(CONFIG_CIFS_FSCACHE)]
#[inline]
pub unsafe fn cifs_fscache_enabled(inode: *mut inode) -> bool {
    fscache_cookie_enabled(cifs_inode_cookie(inode))
}

#[cfg(not(CONFIG_CIFS_FSCACHE))]
#[inline]
pub unsafe fn cifs_fscache_fill_coherency(
    _inode: *mut inode,
    _cd: *mut cifs_fscache_inode_coherency_data,
) {
}

#[cfg(not(CONFIG_CIFS_FSCACHE))]
#[inline]
pub unsafe fn cifs_fscache_get_super_cookie(_tcon: *mut cifs_tcon) -> ::core::ffi::c_int { 0 }

#[cfg(not(CONFIG_CIFS_FSCACHE))]
#[inline]
pub unsafe fn cifs_fscache_release_super_cookie(_tcon: *mut cifs_tcon) {}

#[cfg(not(CONFIG_CIFS_FSCACHE))]
#[inline]
pub unsafe fn cifs_fscache_get_inode_cookie(_inode: *mut inode) {}

#[cfg(not(CONFIG_CIFS_FSCACHE))]
#[inline]
pub unsafe fn cifs_fscache_release_inode_cookie(_inode: *mut inode) {}

#[cfg(not(CONFIG_CIFS_FSCACHE))]
#[inline]
pub unsafe fn cifs_fscache_unuse_inode_cookie(_inode: *mut inode, _update: bool) {}

#[cfg(not(CONFIG_CIFS_FSCACHE))]
#[inline]
pub unsafe fn cifs_inode_cookie(_inode: *mut inode) -> *mut fscache_cookie { core::ptr::null_mut() }

#[cfg(not(CONFIG_CIFS_FSCACHE))]
#[inline]
pub unsafe fn cifs_invalidate_cache(_inode: *mut inode, _flags: ::core::ffi::c_uint) {}

#[cfg(not(CONFIG_CIFS_FSCACHE))]
#[inline]
pub unsafe fn cifs_fscache_enabled(_inode: *mut inode) -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
