// SPDX-License-Identifier: GPL-2.0-only
/*
 * This file contians vfs dentry ops for the 9P2000 protocol.
 *
 *  Copyright (C) 2004 by Eric Van Hensbergen <ericvh@gmail.com>
 *  Copyright (C) 2002 by Ron Minnich <rminnich@lanl.gov>
 */

// Dependencies supplied by the Linux kernel and the surrounding 9P/VFS code.

/**
 * v9fs_ndentry_is_expired - Check if negative dentry lookup has expired
 *
 * This should be called to know if a negative dentry should be removed from
 * cache.
 *
 * @dentry: dentry in question
 */
unsafe fn v9fs_ndentry_is_expired(dentry: *const dentry) -> bool {
    let v9ses = v9fs_dentry2v9ses(dentry);
    let v9fs_dentry = to_v9fs_dentry(dentry);

    if (*v9ses).ndentry_timeout_ms == NDENTRY_TIMEOUT_NEVER {
        return false;
    }

    time_before_eq64((*v9fs_dentry).expire_time, get_jiffies_64())
}

/**
 * v9fs_ndentry_refresh_timeout - Refresh negative dentry lookup cache timeout
 *
 * This should be called when a look up yields a negative entry.
 *
 * @dentry: dentry in question
 */
pub unsafe fn v9fs_ndentry_refresh_timeout(dentry: *mut dentry) {
    let v9ses = v9fs_dentry2v9ses(dentry);
    let v9fs_dentry = to_v9fs_dentry(dentry);

    if (*v9ses).ndentry_timeout_ms == NDENTRY_TIMEOUT_NEVER {
        return;
    }

    (*v9fs_dentry).expire_time = get_jiffies_64()
        .wrapping_add(msecs_to_jiffies((*v9ses).ndentry_timeout_ms));
}

/**
 * v9fs_cached_dentry_delete - called when dentry refcount equals 0
 * @dentry: dentry in question
 */
unsafe fn v9fs_cached_dentry_delete(dentry: *const dentry) -> i32 {
    p9_debug(P9_DEBUG_VFS, " dentry: %pd (%p)\n", dentry, dentry);

    if !d_really_is_negative(dentry) {
        return 0;
    }

    v9fs_ndentry_is_expired(dentry) as i32
}

unsafe fn __v9fs_dentry_fid_remove(dentry: *mut dentry) {
    let v9fs_dentry = to_v9fs_dentry(dentry);
    let mut p: *mut hlist_node;
    let mut n: *mut hlist_node;
    let mut head: hlist_head = core::mem::zeroed();

    p9_debug(P9_DEBUG_VFS, " dentry: %pd (%p)\n", dentry, dentry);

    spin_lock(&mut (*dentry).d_lock);
    hlist_move_list(&mut (*v9fs_dentry).head, &mut head);
    spin_unlock(&mut (*dentry).d_lock);

    hlist_for_each_safe(p, n, &mut head, {
        p9_fid_put(hlist_entry(p, core::mem::size_of::<p9_fid>(), p9_fid, dlist));
    });
}

/**
 * v9fs_dentry_fid_remove - Release all dentry's fids
 * @dentry: dentry in question
 */
pub unsafe fn v9fs_dentry_fid_remove(dentry: *mut dentry) {
    __v9fs_dentry_fid_remove(dentry);
}

/**
 * v9fs_dentry_init - Initialize v9fs dentry data
 * @dentry: dentry in question
 */
unsafe fn v9fs_dentry_init(dentry: *mut dentry) -> i32 {
    let v9fs_dentry = kzalloc(core::mem::size_of::<v9fs_dentry>(), GFP_KERNEL);

    if v9fs_dentry.is_null() {
        return -ENOMEM;
    }

    INIT_HLIST_HEAD(&mut (*v9fs_dentry).head);
    (*dentry).d_fsdata = v9fs_dentry as *mut core::ffi::c_void;
    0
}

/**
 * v9fs_dentry_release - called when dentry is going to be freed
 * @dentry: dentry that is being released
 */
unsafe fn v9fs_dentry_release(dentry: *mut dentry) {
    let v9fs_dentry = to_v9fs_dentry(dentry);

    __v9fs_dentry_fid_remove(dentry);
    kfree_rcu(v9fs_dentry, rcu);
}

unsafe fn __v9fs_lookup_revalidate(dentry: *mut dentry, flags: u32) -> i32 {
    let mut fid: *mut p9_fid;
    let inode: *mut inode;
    let v9inode: *mut v9fs_inode;

    if flags & LOOKUP_RCU != 0 {
        return -ECHILD;
    }

    inode = d_inode(dentry);
    if inode.is_null() {
        return (!v9fs_ndentry_is_expired(dentry)) as i32;
    }

    v9inode = V9FS_I(inode);
    if (*v9inode).cache_validity & V9FS_INO_INVALID_ATTR != 0 {
        let retval: i32;
        let v9ses: *mut v9fs_session_info;

        fid = v9fs_fid_lookup(dentry);
        if IS_ERR(fid) {
            p9_debug(P9_DEBUG_VFS,
                "v9fs_fid_lookup: dentry = %pd (%p), got error %pe\n",
                dentry, dentry, fid);
            return PTR_ERR(fid);
        }

        v9ses = v9fs_inode2v9ses(inode);
        if v9fs_proto_dotl(v9ses) {
            retval = v9fs_refresh_inode_dotl(fid, inode);
        } else {
            retval = v9fs_refresh_inode(fid, inode);
        }
        p9_fid_put(fid);

        if retval == -ENOENT {
            p9_debug(P9_DEBUG_VFS, "dentry: %pd (%p) invalidated due to ENOENT\n", dentry, dentry);
            return 0;
        }
        if (*v9inode).cache_validity & V9FS_INO_INVALID_ATTR != 0 {
            p9_debug(P9_DEBUG_VFS, "dentry: %pd (%p) invalidated due to type change\n", dentry, dentry);
            return 0;
        }
        if retval < 0 {
            p9_debug(P9_DEBUG_VFS,
                "refresh inode: dentry = %pd (%p), got error %pe\n",
                dentry, dentry, ERR_PTR(retval));
            return retval;
        }
    }
    p9_debug(P9_DEBUG_VFS, "dentry: %pd (%p) is valid\n", dentry, dentry);
    1
}

unsafe fn v9fs_lookup_revalidate(_dir: *mut inode, _name: *const qstr,
                                 dentry: *mut dentry, flags: u32) -> i32 {
    __v9fs_lookup_revalidate(dentry, flags)
}

unsafe fn v9fs_dentry_unalias_trylock(dentry: *const dentry) -> bool {
    let v9ses = v9fs_dentry2v9ses(dentry);
    down_write_trylock(&mut (*v9ses).rename_sem)
}

unsafe fn v9fs_dentry_unalias_unlock(dentry: *const dentry) {
    let v9ses = v9fs_dentry2v9ses(dentry);
    up_write(&mut (*v9ses).rename_sem);
}

pub static v9fs_cached_dentry_operations: dentry_operations = dentry_operations {
    d_revalidate: Some(v9fs_lookup_revalidate),
    d_weak_revalidate: Some(__v9fs_lookup_revalidate),
    d_delete: Some(v9fs_cached_dentry_delete),
    d_init: Some(v9fs_dentry_init),
    d_release: Some(v9fs_dentry_release),
    d_unalias_trylock: Some(v9fs_dentry_unalias_trylock),
    d_unalias_unlock: Some(v9fs_dentry_unalias_unlock),
};

pub static v9fs_dentry_operations: dentry_operations = dentry_operations {
    d_init: Some(v9fs_dentry_init),
    d_release: Some(v9fs_dentry_release),
    d_unalias_trylock: Some(v9fs_dentry_unalias_trylock),
    d_unalias_unlock: Some(v9fs_dentry_unalias_unlock),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
