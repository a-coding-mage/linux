// SPDX-License-Identifier: GPL-2.0
/*
 * Cache operations for Coda.
 * For Linux 2.1: (C) 1997 Carnegie Mellon University
 * For Linux 2.3: (C) 2000 Carnegie Mellon University
 *
 * Carnegie Mellon encourages users of this code to contribute improvements
 * to the Coda project http://www.coda.cs.cmu.edu/ <coda@cs.cmu.edu>.
 */

// Linux kernel dependencies are supplied by the surrounding translation.

static mut permission_epoch: atomic_t = unsafe { ATOMIC_INIT(0) };

/* replace or extend an acl cache hit */
pub unsafe fn coda_cache_enter(inode: *mut inode, mask: i32) {
    let cii: *mut coda_inode_info = ITOC(inode);

    spin_lock(&mut (*cii).c_lock);
    (*cii).c_cached_epoch = atomic_read(&permission_epoch);
    if !uid_eq((*cii).c_uid, current_fsuid()) {
        (*cii).c_uid = current_fsuid();
        (*cii).c_cached_perm = mask;
    } else {
        (*cii).c_cached_perm |= mask;
    }
    spin_unlock(&mut (*cii).c_lock);
}

/* remove cached acl from an inode */
pub unsafe fn coda_cache_clear_inode(inode: *mut inode) {
    let cii: *mut coda_inode_info = ITOC(inode);
    spin_lock(&mut (*cii).c_lock);
    (*cii).c_cached_epoch = atomic_read(&permission_epoch) - 1;
    spin_unlock(&mut (*cii).c_lock);
}

/* remove all acl caches */
pub unsafe fn coda_cache_clear_all(_sb: *mut super_block) {
    atomic_inc(&mut permission_epoch);
}

/* check if the mask has been matched against the acl already */
pub unsafe fn coda_cache_check(inode: *mut inode, mask: i32) -> i32 {
    let cii: *mut coda_inode_info = ITOC(inode);
    let hit: i32;

    spin_lock(&mut (*cii).c_lock);
    hit = if (mask & (*cii).c_cached_perm) == mask
        && uid_eq((*cii).c_uid, current_fsuid())
        && (*cii).c_cached_epoch == atomic_read(&permission_epoch)
    {
        1
    } else {
        0
    };
    spin_unlock(&mut (*cii).c_lock);

    hit
}

/* Purging dentries and children */
/* The following routines drop dentries which are not
   in use and flag dentries which are in use to be
   zapped later.

   The flags are detected by:
   - coda_dentry_revalidate (for lookups) if the flag is C_PURGE
   - coda_dentry_delete: to remove dentry from the cache when d_count
     falls to zero
   - an inode method coda_revalidate (for attributes) if the
     flag is C_VATTR
*/

/* this won't do any harm: just flag all children */
unsafe fn coda_flag_children(parent: *mut dentry, flag: i32) {
    let mut de: *mut dentry;

    spin_lock(&mut (*parent).d_lock);
    rcu_read_lock();
    hlist_for_each_entry!(de, (*parent).d_children, d_sib, {
        let inode: *mut inode = d_inode_rcu(de);
        /* don't know what to do with negative dentries */
        if !inode.is_null() {
            coda_flag_inode(inode, flag);
        }
    });
    rcu_read_unlock();
    spin_unlock(&mut (*parent).d_lock);
}

pub unsafe fn coda_flag_inode_children(inode: *mut inode, flag: i32) {
    let alias_de: *mut dentry;

    if inode.is_null() || !S_ISDIR((*inode).i_mode) {
        return;
    }

    alias_de = d_find_alias(inode);
    if alias_de.is_null() {
        return;
    }
    coda_flag_children(alias_de, flag);
    shrink_dcache_parent(alias_de);
    dput(alias_de);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
