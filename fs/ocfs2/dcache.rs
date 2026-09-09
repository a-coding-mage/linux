// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * dcache.c
 *
 * dentry cache handling code
 *
 * Copyright (C) 2002, 2004 Oracle.  All rights reserved.
 */

// External Linux kernel and OCFS2 dependencies are supplied by other files.

pub unsafe fn ocfs2_dentry_attach_gen(dentry: *mut dentry) {
    let gen: c_ulong = (*OCFS2_I(d_inode((*dentry).d_parent))).ip_dir_lock_gen;
    BUG_ON(!d_inode(dentry).is_null());
    (*dentry).d_fsdata = gen as *mut c_void;
}

unsafe fn ocfs2_dentry_revalidate(
    dir: *mut inode,
    name: *const qstr,
    dentry: *mut dentry,
    flags: c_uint,
) -> c_int {
    let mut ret: c_int = 0; // if all else fails, just return false
    let osb: *mut ocfs2_super;

    if flags & LOOKUP_RCU != 0 {
        return -ECHILD;
    }

    let inode = d_inode(dentry);
    osb = OCFS2_SB((*dentry).d_sb);

    trace_ocfs2_dentry_revalidate(dentry, (*name).len, (*name).name);

    /* For a negative dentry - check the generation number of the parent. */
    if inode.is_null() {
        let gen = (*dentry).d_fsdata as c_ulong;
        let pgen = (*OCFS2_I(dir)).ip_dir_lock_gen;
        trace_ocfs2_dentry_revalidate_negative((*name).len, (*name).name, pgen, gen);
        if gen != pgen {
            goto_bail!();
        }
        goto_valid!();
    }

    BUG_ON(osb.is_null());
    if inode == (*osb).root_inode || is_bad_inode(inode) {
        goto_bail!();
    }

    spin_lock(&mut (*OCFS2_I(inode)).ip_lock);
    if (*OCFS2_I(inode)).ip_flags & OCFS2_INODE_DELETED != 0 {
        spin_unlock(&mut (*OCFS2_I(inode)).ip_lock);
        trace_ocfs2_dentry_revalidate_delete((*OCFS2_I(inode)).ip_blkno as c_ulonglong);
        goto_bail!();
    }
    spin_unlock(&mut (*OCFS2_I(inode)).ip_lock);

    if (*inode).i_nlink == 0 {
        trace_ocfs2_dentry_revalidate_orphaned(
            (*OCFS2_I(inode)).ip_blkno as c_ulonglong,
            S_ISDIR((*inode).i_mode),
        );
        goto_bail!();
    }

    if (*dentry).d_fsdata.is_null() {
        trace_ocfs2_dentry_revalidate_nofsdata((*OCFS2_I(inode)).ip_blkno as c_ulonglong);
        goto_bail!();
    }

    ret = 1;
    goto_bail!();

    // These local labels model the C valid/bail exits.
    #[allow(unreachable_code)]
    {
        ret = 1;
    }
    trace_ocfs2_dentry_revalidate_ret(ret);
    ret
}

unsafe fn ocfs2_match_dentry(
    dentry: *mut dentry,
    parent_blkno: u64,
    skip_unhashed: c_int,
) -> c_int {
    if (*dentry).d_fsdata.is_null() {
        return 0;
    }
    if skip_unhashed != 0 && d_unhashed(dentry) {
        return 0;
    }
    let parent = d_inode((*dentry).d_parent);
    if (*OCFS2_I(parent)).ip_blkno != parent_blkno {
        return 0;
    }
    1
}

/* Walk the inode alias list and find a dentry which has a given parent. */
pub unsafe fn ocfs2_find_local_alias(
    inode: *mut inode,
    parent_blkno: u64,
    skip_unhashed: c_int,
) -> *mut dentry {
    let mut dentry: *mut dentry = core::ptr::null_mut();
    spin_lock(&mut (*inode).i_lock);
    for_each_alias!(dentry, inode) {
        spin_lock(&mut (*dentry).d_lock);
        if ocfs2_match_dentry(dentry, parent_blkno, skip_unhashed) != 0 {
            trace_ocfs2_find_local_alias((*dentry).d_name.len, (*dentry).d_name.name);
            dget_dlock(dentry);
            spin_unlock(&mut (*dentry).d_lock);
            spin_unlock(&mut (*inode).i_lock);
            return dentry;
        }
        spin_unlock(&mut (*dentry).d_lock);
    }
    spin_unlock(&mut (*inode).i_lock);
    core::ptr::null_mut()
}

pub static mut dentry_attach_lock: spinlock_t = spinlock_t::new();

/* Attach this dentry to a cluster lock. */
pub unsafe fn ocfs2_dentry_attach_lock(
    dentry: *mut dentry,
    inode: *mut inode,
    parent_blkno: u64,
) -> c_int {
    let mut ret: c_int;
    let mut alias: *mut dentry = core::ptr::null_mut();
    let mut dl = (*dentry).d_fsdata as *mut ocfs2_dentry_lock;

    trace_ocfs2_dentry_attach_lock((*dentry).d_name.len, (*dentry).d_name.name,
        parent_blkno as c_ulonglong, dl);
    if inode.is_null() { return 0; }

    if d_really_is_negative(dentry) && !(*dentry).d_fsdata.is_null() {
        (*dentry).d_fsdata = core::ptr::null_mut();
        dl = core::ptr::null_mut();
    }
    if !dl.is_null() {
        mlog_bug_on_msg((*dl).dl_parent_blkno != parent_blkno, " \"%pd\": old parent: %llu, new: %llu\n", dentry, parent_blkno, (*dl).dl_parent_blkno);
        return 0;
    }

    alias = ocfs2_find_local_alias(inode, parent_blkno, 0);
    if !alias.is_null() {
        dl = (*alias).d_fsdata as *mut ocfs2_dentry_lock;
        mlog_bug_on_msg(dl.is_null(), "parent %llu, ino %llu\n", parent_blkno, (*OCFS2_I(inode)).ip_blkno);
        mlog_bug_on_msg((*dl).dl_parent_blkno != parent_blkno, " \"%pd\": old parent: %llu, new: %llu\n", dentry, parent_blkno, (*dl).dl_parent_blkno);
        trace_ocfs2_dentry_attach_lock_found((*dl).dl_lockres.l_name, parent_blkno, (*OCFS2_I(inode)).ip_blkno);
    } else {
        dl = kmalloc_obj!(ocfs2_dentry_lock, GFP_NOFS);
        if dl.is_null() { ret = -ENOMEM; mlog_errno(ret); return ret; }
        (*dl).dl_count = 0;
        (*dl).dl_inode = igrab(inode);
        (*dl).dl_parent_blkno = parent_blkno;
        ocfs2_dentry_lock_res_init(dl, parent_blkno, inode);
    }

    spin_lock(&mut dentry_attach_lock);
    if !(*dentry).d_fsdata.is_null() && alias.is_null() {
        spin_unlock(&mut dentry_attach_lock);
        iput((*dl).dl_inode); ocfs2_lock_res_free(&mut (*dl).dl_lockres); kfree(dl as *mut c_void);
        return 0;
    }
    (*dentry).d_fsdata = dl as *mut c_void;
    (*dl).dl_count += 1;
    spin_unlock(&mut dentry_attach_lock);

    ret = ocfs2_dentry_lock(dentry, 0);
    if ret == 0 { ocfs2_dentry_unlock(dentry, 0); } else { mlog_errno(ret); }
    if ret < 0 && alias.is_null() {
        ocfs2_lock_res_free(&mut (*dl).dl_lockres);
        BUG_ON((*dl).dl_count != 1);
        spin_lock(&mut dentry_attach_lock); (*dentry).d_fsdata = core::ptr::null_mut(); spin_unlock(&mut dentry_attach_lock);
        kfree(dl as *mut c_void); iput(inode);
    }
    dput(alias);
    ret
}

/* Drop a dentry lock after its final reference. */
unsafe fn ocfs2_drop_dentry_lock(osb: *mut ocfs2_super, dl: *mut ocfs2_dentry_lock) {
    iput((*dl).dl_inode);
    ocfs2_simple_drop_lockres(osb, &mut (*dl).dl_lockres);
    ocfs2_lock_res_free(&mut (*dl).dl_lockres);
    kfree(dl as *mut c_void);
}

pub unsafe fn ocfs2_dentry_lock_put(osb: *mut ocfs2_super, dl: *mut ocfs2_dentry_lock) {
    BUG_ON((*dl).dl_count == 0);
    spin_lock(&mut dentry_attach_lock);
    (*dl).dl_count -= 1;
    let unlock = (*dl).dl_count == 0;
    spin_unlock(&mut dentry_attach_lock);
    if unlock { ocfs2_drop_dentry_lock(osb, dl); }
}

unsafe fn ocfs2_dentry_iput(dentry: *mut dentry, inode: *mut inode) {
    let dl = (*dentry).d_fsdata as *mut ocfs2_dentry_lock;
    if dl.is_null() {
        if (*dentry).d_flags & DCACHE_DISCONNECTED == 0 && !d_unhashed(dentry) {
            let ino = if inode.is_null() { 0 } else { (*OCFS2_I(inode)).ip_blkno };
            mlog!(ML_ERROR, "Dentry is missing cluster lock. inode: %llu, d_flags: 0x%x, d_name: %pd\n", ino, (*dentry).d_flags, dentry);
        }
        iput(inode); return;
    }
    mlog_bug_on_msg((*dl).dl_count == 0, "dentry: %pd, count: %u\n", dentry, (*dl).dl_count);
    ocfs2_dentry_lock_put(OCFS2_SB((*dentry).d_sb), dl);
    iput(inode);
}

/* d_move(), but keep the locks in sync. */
pub unsafe fn ocfs2_dentry_move(dentry: *mut dentry, target: *mut dentry, old_dir: *mut inode, new_dir: *mut inode) {
    let osb = OCFS2_SB((*old_dir).i_sb);
    let inode = d_inode(dentry);
    if old_dir != new_dir {
        ocfs2_dentry_lock_put(osb, (*dentry).d_fsdata as *mut ocfs2_dentry_lock);
        (*dentry).d_fsdata = core::ptr::null_mut();
        let ret = ocfs2_dentry_attach_lock(dentry, inode, (*OCFS2_I(new_dir)).ip_blkno);
        if ret != 0 { mlog_errno(ret); }
    }
    d_move(dentry, target);
}

pub static ocfs2_dentry_ops: dentry_operations = dentry_operations {
    d_revalidate: Some(ocfs2_dentry_revalidate),
    d_iput: Some(ocfs2_dentry_iput),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
