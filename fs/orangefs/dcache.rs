// SPDX-License-Identifier: GPL-2.0
/*
 * (C) 2001 Clemson University and The University of Chicago
 *
 * See COPYING in top-level directory.
 */

/*
 *  Implementation of dentry (directory cache) functions.
 */

// Dependencies supplied by protocol.h and orangefs-kernel.h remain external.

/* Returns 1 if dentry can still be trusted, else 0. */
unsafe fn orangefs_revalidate_lookup(
    parent_inode: *mut inode,
    name: *const qstr,
    dentry: *mut dentry,
) -> c_int {
    let parent = ORANGEFS_I(parent_inode);
    let inode = (*dentry).d_inode;
    let mut new_op: *mut orangefs_kernel_op_s;
    let mut ret: c_int = 0;
    let mut err: c_int = 0;

    gossip_debug(GOSSIP_DCACHE_DEBUG, "%s: attempting lookup.\n", __func__);

    new_op = op_alloc(ORANGEFS_VFS_OP_LOOKUP);
    if new_op.is_null() {
        return -ENOMEM;
    }

    (*new_op).upcall.req.lookup.sym_follow = ORANGEFS_LOOKUP_LINK_NO_FOLLOW;
    (*new_op).upcall.req.lookup.parent_refn = (*parent).refn;
    /* op_alloc() leaves ->upcall zeroed */
    memcpy(
        (*new_op).upcall.req.lookup.d_name.as_mut_ptr() as *mut c_void,
        (*name).name as *const c_void,
        min((*name).len, ORANGEFS_NAME_MAX - 1),
    );

    gossip_debug(
        GOSSIP_DCACHE_DEBUG,
        "%s:%s:%d interrupt flag [%d]\n",
        __FILE__,
        __func__,
        __LINE__,
        get_interruptible_flag(parent_inode),
    );

    err = service_operation(
        new_op,
        "orangefs_lookup",
        get_interruptible_flag(parent_inode),
    );

    /* Positive dentry: reject if error or not the same inode. */
    if !inode.is_null() {
        if err != 0 {
            gossip_debug(
                GOSSIP_DCACHE_DEBUG,
                "%s:%s:%d lookup failure.\n",
                __FILE__,
                __func__,
                __LINE__,
            );
            goto_out_drop(new_op, &mut ret);
            return ret;
        }
        if !match_handle((*new_op).downcall.resp.lookup.refn.khandle, inode) {
            gossip_debug(
                GOSSIP_DCACHE_DEBUG,
                "%s:%s:%d no match.\n",
                __FILE__,
                __func__,
                __LINE__,
            );
            goto_out_drop(new_op, &mut ret);
            return ret;
        }
    /* Negative dentry: reject if success or error other than ENOENT. */
    } else {
        gossip_debug(GOSSIP_DCACHE_DEBUG, "%s: negative dentry.\n", __func__);
        if err == 0 || err != -ENOENT {
            if (*new_op).downcall.status != 0 {
                gossip_debug(
                    GOSSIP_DCACHE_DEBUG,
                    "%s:%s:%d lookup failure.\n",
                    __FILE__,
                    __func__,
                    __LINE__,
                );
            }
            goto_out_drop(new_op, &mut ret);
            return ret;
        }
    }

    orangefs_set_timeout(dentry);
    ret = 1;
    op_release(new_op);
    return ret;
}

/*
 * Verify that dentry is valid.
 *
 * Should return 1 if dentry can still be trusted, else 0.
 */
unsafe fn orangefs_d_revalidate(
    dir: *mut inode,
    name: *const qstr,
    dentry: *mut dentry,
    flags: c_uint,
) -> c_int {
    let ret: c_int;
    let time = (*dentry).d_fsdata as c_ulong;

    if time_before(jiffies, time) {
        return 1;
    }

    if flags & LOOKUP_RCU != 0 {
        return -ECHILD;
    }

    gossip_debug(GOSSIP_DCACHE_DEBUG, "%s: called on dentry %p.\n", __func__, dentry);

    /* skip root handle lookups. */
    if !(*dentry).d_inode.is_null() && is_root_handle((*dentry).d_inode) {
        return 1;
    }

    /*
     * If this passes, the positive dentry still exists or the negative
     * dentry still does not exist.
     */
    if orangefs_revalidate_lookup(dir, name, dentry) == 0 {
        return 0;
    }

    /* We do not need to continue with negative dentries. */
    if (*dentry).d_inode.is_null() {
        gossip_debug(
            GOSSIP_DCACHE_DEBUG,
            "%s: negative dentry or positive dentry and inode valid.\n",
            __func__,
        );
        return 1;
    }

    /* Now we must perform a getattr to validate the inode contents. */
    ret = orangefs_inode_check_changed((*dentry).d_inode);
    if ret < 0 {
        gossip_debug(
            GOSSIP_DCACHE_DEBUG,
            "%s:%s:%d getattr failure.\n",
            __FILE__,
            __func__,
            __LINE__,
        );
        return 0;
    }
    if ret == 0 { 1 } else { 0 }
}

#[no_mangle]
pub static orangefs_dentry_operations: dentry_operations = dentry_operations {
    d_revalidate: Some(orangefs_d_revalidate),
};


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
