// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sysfile.c
 *
 * Initialize, read, write, etc. system files.
 *
 * Copyright (C) 2002, 2004 Oracle.  All rights reserved.
 */

// Linux, cluster, and local OCFS2 declarations are supplied by other files.

#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
static mut OCFS2_SYSFILE_CLUSTER_LOCK_KEY: [lock_class_key; NUM_SYSTEM_INODES] =
    [lock_class_key {}; NUM_SYSTEM_INODES];

#[inline]
unsafe fn is_global_system_inode(type_: i32) -> bool {
    type_ >= OCFS2_FIRST_ONLINE_SYSTEM_INODE
        && type_ <= OCFS2_LAST_GLOBAL_SYSTEM_INODE
}

unsafe fn get_local_system_inode(
    osb: *mut ocfs2_super,
    type_: i32,
    slot: u32,
) -> *mut *mut inode {
    let mut index: usize;
    let mut local_system_inodes: *mut *mut inode;
    let mut free: *mut *mut inode = core::ptr::null_mut();

    BUG_ON(slot == OCFS2_INVALID_SLOT);
    BUG_ON(type_ < OCFS2_FIRST_LOCAL_SYSTEM_INODE
        || type_ > OCFS2_LAST_LOCAL_SYSTEM_INODE);

    spin_lock(&mut (*osb).osb_lock);
    local_system_inodes = (*osb).local_system_inodes;
    spin_unlock(&mut (*osb).osb_lock);

    if unlikely(local_system_inodes.is_null()) {
        local_system_inodes = kzalloc(
            array3_size(
                core::mem::size_of::<*mut inode>(),
                NUM_LOCAL_SYSTEM_INODES,
                (*osb).max_slots,
            ),
            GFP_NOFS,
        );
        if local_system_inodes.is_null() {
            mlog_errno(-ENOMEM);
            /*
             * return NULL here so that ocfs2_get_sytem_file_inodes
             * will try to create an inode and use it. We will try
             * to initialize local_system_inodes next time.
             */
            return core::ptr::null_mut();
        }

        spin_lock(&mut (*osb).osb_lock);
        if !(*osb).local_system_inodes.is_null() {
            /* Someone has initialized it for us. */
            free = local_system_inodes;
            local_system_inodes = (*osb).local_system_inodes;
        } else {
            (*osb).local_system_inodes = local_system_inodes;
        }
        spin_unlock(&mut (*osb).osb_lock);
        kfree(free);
    }

    index = (slot as usize * NUM_LOCAL_SYSTEM_INODES)
        + (type_ - OCFS2_FIRST_LOCAL_SYSTEM_INODE) as usize;

    local_system_inodes.add(index)
}

pub unsafe fn ocfs2_get_system_file_inode(
    osb: *mut ocfs2_super,
    type_: i32,
    slot: u32,
) -> *mut inode {
    let mut inode: *mut inode = core::ptr::null_mut();
    let mut arr: *mut *mut inode = core::ptr::null_mut();

    /* avoid the lookup if cached in local system file array */
    if is_global_system_inode(type_) {
        arr = (*osb).global_system_inodes.as_mut_ptr().add(type_ as usize);
    } else {
        arr = get_local_system_inode(osb, type_, slot);
    }

    if !arr.is_null() && {
        inode = *arr;
        !inode.is_null()
    } {
        /* get a ref in addition to the array ref */
        inode = igrab(inode);
        BUG_ON(inode.is_null());
        return inode;
    }

    /* this gets one ref thru iget */
    inode = _ocfs2_get_system_file_inode(osb, type_, slot);

    /* add one more if putting into array for first time */
    if !inode.is_null() && !arr.is_null() && (*arr).is_null()
        && cmpxchg(arr, core::ptr::null_mut(), inode).is_null()
    {
        inode = igrab(inode);
        BUG_ON(inode.is_null());
    }
    inode
}

unsafe fn _ocfs2_get_system_file_inode(
    osb: *mut ocfs2_super,
    type_: i32,
    slot: u32,
) -> *mut inode {
    let mut namebuf = [0i8; 40];
    let mut inode: *mut inode = core::ptr::null_mut();
    let mut blkno: u64 = 0;
    let mut status: i32 = 0;
    let len: i32;

    len = ocfs2_sprintf_system_inode_name(
        namebuf.as_mut_ptr(),
        namebuf.len(),
        type_,
        slot,
    );

    status = ocfs2_lookup_ino_from_name(
        (*osb).sys_root_inode,
        namebuf.as_ptr(),
        len,
        &mut blkno,
    );
    if status < 0 {
    } else {
        inode = ocfs2_iget(osb, blkno, OCFS2_FI_FLAG_SYSFILE, type_);
        if IS_ERR(inode) {
            mlog_errno(PTR_ERR(inode));
            inode = core::ptr::null_mut();
            goto_bail();
        }
        #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
        {
            if type_ == LOCAL_USER_QUOTA_SYSTEM_INODE
                || type_ == LOCAL_GROUP_QUOTA_SYSTEM_INODE
                || type_ == JOURNAL_SYSTEM_INODE
            {
                /* Ignore inode lock on these inodes as the lock does not
                 * really belong to any process and lockdep cannot handle
                 * that */
                (*OCFS2_I(inode)).ip_inode_lockres.l_lockdep_map.key =
                    core::ptr::null_mut();
            } else {
                lockdep_init_map(
                    &mut (*OCFS2_I(inode)).ip_inode_lockres.l_lockdep_map,
                    ocfs2_system_inodes[type_ as usize].si_name,
                    &mut OCFS2_SYSFILE_CLUSTER_LOCK_KEY[type_ as usize],
                    0,
                );
            }
        }
    }

    inode
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
