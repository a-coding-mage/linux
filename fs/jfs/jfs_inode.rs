// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) International Business Machines Corp., 2000-2004
 */

// Linux/JFS dependencies supplied by the surrounding translation unit.

pub unsafe fn jfs_set_inode_flags(inode: *mut inode) {
    let flags: ::std::os::raw::c_uint = (*JFS_IP(inode)).mode2;
    let mut new_fl: ::std::os::raw::c_uint = 0;

    if flags & JFS_IMMUTABLE_FL != 0 {
        new_fl |= S_IMMUTABLE;
    }
    if flags & JFS_APPEND_FL != 0 {
        new_fl |= S_APPEND;
    }
    if flags & JFS_NOATIME_FL != 0 {
        new_fl |= S_NOATIME;
    }
    if flags & JFS_DIRSYNC_FL != 0 {
        new_fl |= S_DIRSYNC;
    }
    if flags & JFS_SYNC_FL != 0 {
        new_fl |= S_SYNC;
    }
    inode_set_flags(
        inode,
        new_fl,
        S_IMMUTABLE | S_APPEND | S_NOATIME | S_DIRSYNC | S_SYNC,
    );
}

/*
 * NAME:    ialloc()
 *
 * FUNCTION:    Allocate a new inode
 *
 */
pub unsafe fn ialloc(parent: *mut inode, mode: umode_t) -> *mut inode {
    let sb: *mut super_block = (*parent).i_sb;
    let mut inode: *mut inode;
    let jfs_inode: *mut jfs_inode_info;
    let mut rc: ::std::os::raw::c_int;

    inode = new_inode(sb);
    if inode.is_null() {
        jfs_warn!("ialloc: new_inode returned NULL!");
        return ERR_PTR(-ENOMEM);
    }

    jfs_inode = JFS_IP(inode);

    rc = diAlloc(parent, S_ISDIR(mode), inode);
    if rc != 0 {
        jfs_warn!("ialloc: diAlloc returned %d!", rc);
        goto_fail_put: {
            iput(inode);
            return ERR_PTR(rc);
        }
    }

    if insert_inode_locked(inode) < 0 {
        rc = -EINVAL;
        iput(inode);
        return ERR_PTR(rc);
    }

    inode_init_owner(&nop_mnt_idmap, inode, parent, mode);
    /*
     * New inodes need to save sane values on disk when
     * uid & gid mount options are used
     */
    (*jfs_inode).saved_uid = (*inode).i_uid;
    (*jfs_inode).saved_gid = (*inode).i_gid;

    /*
     * Allocate inode to quota.
     */
    rc = dquot_initialize(inode);
    if rc != 0 {
        dquot_drop(inode);
        (*inode).i_flags |= S_NOQUOTA;
        clear_nlink(inode);
        discard_new_inode(inode);
        return ERR_PTR(rc);
    }
    rc = dquot_alloc_inode(inode);
    if rc != 0 {
        dquot_drop(inode);
        (*inode).i_flags |= S_NOQUOTA;
        clear_nlink(inode);
        discard_new_inode(inode);
        return ERR_PTR(rc);
    }

    /* inherit flags from parent */
    (*jfs_inode).mode2 = (*JFS_IP(parent)).mode2 & JFS_FL_INHERIT;

    if S_ISDIR(mode) {
        (*jfs_inode).mode2 |= IDIRECTORY;
        (*jfs_inode).mode2 &= !JFS_DIRSYNC_FL;
    } else {
        (*jfs_inode).mode2 |= INLINEEA | ISPARSE;
        if S_ISLNK(mode) {
            (*jfs_inode).mode2 &= !(JFS_IMMUTABLE_FL | JFS_APPEND_FL);
        }
    }
    (*jfs_inode).mode2 |= (*inode).i_mode;

    (*inode).i_blocks = 0;
    simple_inode_init_ts(inode);
    (*jfs_inode).otime = inode_get_ctime_sec(inode);
    (*inode).i_generation = (*JFS_SBI(sb)).gengen;
    (*JFS_SBI(sb)).gengen += 1;

    (*jfs_inode).cflag = 0;

    /* Zero remaining fields */
    ::core::ptr::write_bytes(&mut (*jfs_inode).acl, 0, 1);
    ::core::ptr::write_bytes(&mut (*jfs_inode).ea, 0, 1);
    (*jfs_inode).next_index = 0;
    (*jfs_inode).acltype = 0;
    (*jfs_inode).btorder = 0;
    (*jfs_inode).btindex = 0;
    (*jfs_inode).bxflag = 0;
    (*jfs_inode).blid = 0;
    (*jfs_inode).atlhead = 0;
    (*jfs_inode).atltail = 0;
    (*jfs_inode).xtlid = 0;
    jfs_set_inode_flags(inode);

    jfs_info!("ialloc returns inode = 0x%p", inode);

    inode
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
