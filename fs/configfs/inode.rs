// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * inode.c - basic inode and dentry operations.
 *
 * Based on sysfs:
 *	sysfs is Copyright (C) 2001, 2002, 2003 Patrick Mochel
 *
 * configfs Copyright (C) 2005 Oracle.  All rights reserved.
 *
 * Please see Documentation/filesystems/configfs.rst for more
 * information.
 */

// Dependencies supplied by the Linux kernel and configfs headers.

#[cfg(CONFIG_LOCKDEP)]
static mut DEFAULT_GROUP_CLASS: [lock_class_key; MAX_LOCK_DEPTH] =
    [lock_class_key { _private: 0 }; MAX_LOCK_DEPTH];

static CONFIGFS_INODE_OPERATIONS: inode_operations = inode_operations {
    setattr: Some(configfs_setattr),
};

pub unsafe fn configfs_setattr(
    idmap: *mut mnt_idmap,
    dentry: *mut dentry,
    iattr: *mut iattr,
) -> c_int {
    let inode = d_inode(dentry);
    let sd = (*dentry).d_fsdata as *mut configfs_dirent;
    let mut sd_iattr: *mut iattr;
    let ia_valid: c_uint = (*iattr).ia_valid;
    let error: c_int;

    if sd.is_null() {
        return -EINVAL;
    }

    sd_iattr = (*sd).s_iattr;
    if sd_iattr.is_null() {
        // setting attributes for the first time, allocate now
        sd_iattr = kzalloc_obj::<iattr>();
        if sd_iattr.is_null() {
            return -ENOMEM;
        }
        // assign default attributes
        (*sd_iattr).ia_mode = (*sd).s_mode;
        (*sd_iattr).ia_uid = GLOBAL_ROOT_UID;
        (*sd_iattr).ia_gid = GLOBAL_ROOT_GID;
        (*sd_iattr).ia_atime = current_time(inode);
        (*sd_iattr).ia_mtime = (*sd_iattr).ia_atime;
        (*sd_iattr).ia_ctime = (*sd_iattr).ia_mtime;
        (*sd).s_iattr = sd_iattr;
    }
    // attributes were changed atleast once in past

    error = simple_setattr(idmap, dentry, iattr);
    if error != 0 {
        return error;
    }

    if ia_valid & ATTR_UID != 0 {
        (*sd_iattr).ia_uid = (*iattr).ia_uid;
    }
    if ia_valid & ATTR_GID != 0 {
        (*sd_iattr).ia_gid = (*iattr).ia_gid;
    }
    if ia_valid & ATTR_ATIME != 0 {
        (*sd_iattr).ia_atime = (*iattr).ia_atime;
    }
    if ia_valid & ATTR_MTIME != 0 {
        (*sd_iattr).ia_mtime = (*iattr).ia_mtime;
    }
    if ia_valid & ATTR_CTIME != 0 {
        (*sd_iattr).ia_ctime = (*iattr).ia_ctime;
    }
    if ia_valid & ATTR_MODE != 0 {
        let mut mode: umode_t = (*iattr).ia_mode;

        if !in_group_p((*inode).i_gid) && !capable(CAP_FSETID) {
            mode &= !S_ISGID;
        }
        (*sd_iattr).ia_mode = mode;
        (*sd).s_mode = mode;
    }

    error
}

#[inline]
unsafe fn set_default_inode_attr(inode: *mut inode, mode: umode_t) {
    (*inode).i_mode = mode;
    simple_inode_init_ts(inode);
}

#[inline]
unsafe fn set_inode_attr(inode: *mut inode, iattr: *mut iattr) {
    (*inode).i_mode = (*iattr).ia_mode;
    (*inode).i_uid = (*iattr).ia_uid;
    (*inode).i_gid = (*iattr).ia_gid;
    inode_set_atime_to_ts(inode, (*iattr).ia_atime);
    inode_set_mtime_to_ts(inode, (*iattr).ia_mtime);
    inode_set_ctime_to_ts(inode, (*iattr).ia_ctime);
}

pub unsafe fn configfs_new_inode(
    mode: umode_t,
    sd: *mut configfs_dirent,
    s: *mut super_block,
) -> *mut inode {
    let inode = new_inode(s);
    if !inode.is_null() {
        (*inode).i_ino = get_next_ino();
        (*inode).i_mapping.a_ops = &ram_aops;
        (*inode).i_op = &CONFIGFS_INODE_OPERATIONS;

        if !(*sd).s_iattr.is_null() {
            /* sysfs_dirent has non-default attributes
             * get them for the new inode from persistent copy
             * in sysfs_dirent
             */
            set_inode_attr(inode, (*sd).s_iattr);
        } else {
            set_default_inode_attr(inode, mode);
        }
    }
    inode
}

#[cfg(CONFIG_LOCKDEP)]
unsafe fn configfs_set_inode_lock_class(sd: *mut configfs_dirent, inode: *mut inode) {
    let depth = (*sd).s_depth;

    if depth > 0 {
        if depth <= ARRAY_SIZE(&DEFAULT_GROUP_CLASS) {
            lockdep_set_class(
                &mut (*inode).i_rwsem,
                &mut DEFAULT_GROUP_CLASS[(depth - 1) as usize],
            );
        } else {
            /*
             * In practice the maximum level of locking depth is
             * already reached. Just inform about possible reasons.
             */
            pr_info!("Too many levels of inodes for the locking correctness validator.\n");
            pr_info!("Spurious warnings may appear.\n");
        }
    }
}

#[cfg(not(CONFIG_LOCKDEP))]
unsafe fn configfs_set_inode_lock_class(_sd: *mut configfs_dirent, _inode: *mut inode) {}

pub unsafe fn configfs_create(dentry: *mut dentry, mode: umode_t) -> *mut inode {
    let mut inode: *mut inode = core::ptr::null_mut();
    let sd: *mut configfs_dirent;

    if dentry.is_null() {
        return ERR_PTR(-ENOENT);
    }

    if d_really_is_positive(dentry) {
        return ERR_PTR(-EEXIST);
    }

    sd = (*dentry).d_fsdata as *mut configfs_dirent;
    inode = configfs_new_inode(mode, sd, (*dentry).d_sb);
    if inode.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    configfs_set_inode_lock_class(sd, inode);
    inode
}

/*
 * Get the name for corresponding element represented by the given configfs_dirent
 */
pub unsafe fn configfs_get_name(sd: *mut configfs_dirent) -> *const u8 {
    let attr: *mut configfs_attribute;

    BUG_ON(sd.is_null() || (*sd).s_element.is_null());

    // These always have a dentry, so use that
    if (*sd).s_type & (CONFIGFS_DIR | CONFIGFS_ITEM_LINK) != 0 {
        return (*(*sd).s_dentry).d_name.name;
    }

    if (*sd).s_type & (CONFIGFS_ITEM_ATTR | CONFIGFS_ITEM_BIN_ATTR) != 0 {
        attr = (*sd).s_element as *mut configfs_attribute;
        return (*attr).ca_name;
    }
    core::ptr::null()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
