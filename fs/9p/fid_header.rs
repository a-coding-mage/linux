/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * V9FS FID Management
 *
 *  Copyright (C) 2005 by Eric Van Hensbergen <ericvh@gmail.com>
 */

/* Dependencies supplied by linux/list.h and v9fs.h. */

extern "C" {
    pub fn v9fs_fid_find_inode(
        inode: *mut inode,
        want_writeable: bool,
        uid: kuid_t,
        any: bool,
    ) -> *mut p9_fid;
    pub fn v9fs_fid_lookup(dentry: *mut dentry) -> *mut p9_fid;
    pub fn v9fs_fid_add(dentry: *mut dentry, fid: *mut *mut p9_fid);
    pub fn v9fs_open_fid_add(inode: *mut inode, fid: *mut *mut p9_fid);
    pub fn p9_client_walk(
        fid: *mut p9_fid,
        nwname: i32,
        wnames: *const *const u8,
        clone: i32,
    ) -> *mut p9_fid;
    pub fn p9_fid_put(fid: *mut p9_fid);
}

#[inline]
pub unsafe fn v9fs_parent_fid(dentry: *mut dentry) -> *mut p9_fid {
    v9fs_fid_lookup((*dentry).d_parent)
}

#[inline]
pub unsafe fn clone_fid(fid: *mut p9_fid) -> *mut p9_fid {
    if is_err(fid) {
        fid
    } else {
        p9_client_walk(fid, 0, core::ptr::null(), 1)
    }
}

#[inline]
pub unsafe fn v9fs_fid_clone(dentry: *mut dentry) -> *mut p9_fid {
    let fid: *mut p9_fid = v9fs_fid_lookup(dentry);
    if fid.is_null() || is_err(fid) {
        return fid;
    }

    let nfid: *mut p9_fid = clone_fid(fid);
    p9_fid_put(fid);
    nfid
}

/**
 * v9fs_fid_addmodes - add cache flags to fid mode (for client use only)
 * @fid: fid to augment
 * @s_flags: session info mount flags
 * @s_cache: session info cache flags
 * @f_flags: unix open flags
 *
 * make sure mode reflects flags of underlying mounts
 * also qid.version == 0 reflects a synthetic or legacy file system
 * NOTE: these are set after open so only reflect 9p client not
 * underlying file system on server.
 */
#[inline]
pub unsafe fn v9fs_fid_add_modes(
    fid: *mut p9_fid,
    s_flags: u32,
    s_cache: u32,
    f_flags: u32,
) {
    if (s_cache == 0)
        || ((*fid).qid.version == 0 && (s_flags & V9FS_IGNORE_QV) == 0)
        || (s_flags & V9FS_DIRECT_IO) != 0
        || (f_flags & O_DIRECT) != 0
    {
        (*fid).mode |= P9L_DIRECT; /* no read or write cache */
    } else if ((s_cache & CACHE_WRITEBACK) == 0)
        || (f_flags & O_DSYNC) != 0
        || (s_flags & V9FS_SYNC) != 0
    {
        (*fid).mode |= P9L_NOWRITECACHE;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
