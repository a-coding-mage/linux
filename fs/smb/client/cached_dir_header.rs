/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Functions to handle the cached directory entries
 *
 *  Copyright (c) 2022, Ronnie Sahlberg <lsahlber@redhat.com>
 */

use core::ffi::{c_char, c_uint, c_ulong};

#[repr(C)]
pub struct cached_dirent {
    pub entry: list_head,
    pub name: *mut c_char,
    pub namelen: i32,
    pub pos: loff_t,
    pub fattr: cifs_fattr,
}

#[repr(C)]
pub struct cached_dirents {
    pub is_valid: bool,
    pub is_failed: bool,
    pub file: *mut file,
    /*
     * Used to associate the cache with a single
     * open file instance.
     */
    pub de_mutex: mutex,
    /* Expected ctx->pos */
    pub pos: loff_t,
    pub entries: list_head,
    /* accounting for cached entries in this directory */
    pub entries_count: c_ulong,
    pub bytes_used: c_ulong,
}

#[repr(C)]
pub struct cached_fid {
    pub entry: list_head,
    pub cfids: *mut cached_fids,
    pub path: *const c_char,
    pub has_lease: bool,
    pub is_open: bool,
    pub on_list: bool,
    pub file_all_info_is_valid: bool,
    /* jiffies of when lease was taken */
    pub time: c_ulong,
    /* jiffies of when last accessed */
    pub last_access_time: c_ulong,
    pub refcount: kref,
    pub fid: cifs_fid,
    pub tcon: *mut cifs_tcon,
    pub dentry: *mut dentry,
    pub put_work: work_struct,
    pub close_work: work_struct,
    pub dirents: cached_dirents,

    /* Must be last as it ends in a flexible-array member. */
    pub file_all_info: smb2_file_all_info,
}

/* default MAX_CACHED_FIDS is 16 */
#[repr(C)]
pub struct cached_fids {
    /* Must be held when:
     * - accessing the cfids->entries list
     * - accessing the cfids->dying list
     */
    pub cfid_list_lock: spinlock_t,
    pub num_entries: i32,
    pub entries: list_head,
    pub dying: list_head,
    pub laundromat_work: delayed_work,
    /* aggregate accounting for all cached dirents under this tcon */
    pub total_dirents_entries: atomic_long_t,
    pub total_dirents_bytes: atomic64_t,
}

/* Module-wide directory cache accounting (defined in cifsfs.c) */
extern "C" {
    pub static mut cifs_dircache_bytes_used: atomic64_t; /* bytes across all mounts */
}

#[inline]
pub unsafe fn is_valid_cached_dir(cfid: *mut cached_fid) -> bool {
    (*cfid).time != 0 && (*cfid).has_lease
}

extern "C" {
    pub fn init_cached_dirs() -> *mut cached_fids;
    pub fn free_cached_dirs(cfids: *mut cached_fids);
    pub fn open_cached_dir(
        xid: c_uint,
        tcon: *mut cifs_tcon,
        path: *const c_char,
        cifs_sb: *mut cifs_sb_info,
        lookup_only: bool,
        ret_cfid: *mut *mut cached_fid,
    ) -> i32;
    pub fn open_cached_dir_by_dentry(
        tcon: *mut cifs_tcon,
        dentry: *mut dentry,
        ret_cfid: *mut *mut cached_fid,
    ) -> i32;
    pub fn close_cached_dir(cfid: *mut cached_fid);
    pub fn drop_cached_dir_by_name(
        xid: c_uint,
        tcon: *mut cifs_tcon,
        name: *const c_char,
        cifs_sb: *mut cifs_sb_info,
    );
    pub fn close_all_cached_dirs(cifs_sb: *mut cifs_sb_info);
    pub fn invalidate_all_cached_dirs(tcon: *mut cifs_tcon, sync: bool);
    pub fn cached_dir_lease_break(tcon: *mut cifs_tcon, lease_key: *mut u8) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
