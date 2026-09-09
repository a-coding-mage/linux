/*
 *  linux/fs/hfs/sysdep.c
 *
 * Copyright (C) 1996  Paul H. Hargrove
 * (C) 2003 Ardis Technologies <roman@ardistech.com>
 * This file may be distributed under the terms of the GNU General Public License.
 *
 * This file contains the code to do various system dependent things.
 */

/* Dependency declarations and types are supplied by the surrounding kernel code. */

/* dentry case-handling: just lowercase everything */

unsafe fn hfs_revalidate_dentry(
    _dir: *mut inode,
    _name: *const qstr,
    dentry: *mut dentry,
    flags: c_uint,
) -> c_int {
    let inode: *mut inode;
    let mut diff: i32;

    if flags & LOOKUP_RCU != 0 {
        return -ECHILD;
    }

    inode = d_inode(dentry);
    if inode.is_null() {
        return 1;
    }

    /* fix up inode on a timezone change */
    diff = sys_tz.tz_minuteswest * 60 - (*HFS_I(inode)).tz_secondswest;
    if diff != 0 {
        let mut ts: timespec64 = inode_get_ctime(inode);

        inode_set_ctime(inode, ts.tv_sec + diff as i64, ts.tv_nsec);
        ts = inode_get_atime(inode);
        inode_set_atime(inode, ts.tv_sec + diff as i64, ts.tv_nsec);
        ts = inode_get_mtime(inode);
        inode_set_mtime(inode, ts.tv_sec + diff as i64, ts.tv_nsec);
        (*HFS_I(inode)).tz_secondswest += diff;
    }
    1
}

pub static hfs_dentry_operations: dentry_operations = dentry_operations {
    d_revalidate: Some(hfs_revalidate_dentry),
    d_hash: Some(hfs_hash_dentry),
    d_compare: Some(hfs_compare_dentry),
};


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
