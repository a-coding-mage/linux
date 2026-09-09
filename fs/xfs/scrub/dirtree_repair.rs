// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2023-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* Directory Tree Structure Repairs */

pub unsafe fn xrep_setup_dirtree(sc: *mut xfs_scrub) -> i32 {
    xrep_orphanage_try_create(sc)
}

#[inline]
unsafe fn xrep_dirpath_set_outcome(
    dl: *mut xchk_dirtree,
    path: *mut xchk_dirpath,
    outcome: xchk_dirpath_outcome,
) {
    trace_xrep_dirpath_set_outcome((*dl).sc, (*path).path_nr, (*path).nr_steps, outcome);
    (*path).outcome = outcome;
}

unsafe fn xrep_dirtree_delete_all_paths(
    dl: *mut xchk_dirtree,
    oc: *mut xchk_dirtree_outcomes,
) {
    let mut path: *mut xchk_dirpath;
    xchk_dirtree_for_each_path!(dl, path, {
        match (*path).outcome {
            XCHK_DIRPATH_CORRUPT | XCHK_DIRPATH_LOOP => {
                (*oc).suspect -= 1;
                (*oc).bad += 1;
                xrep_dirpath_set_outcome(dl, path, XCHK_DIRPATH_DELETE);
            }
            XCHK_DIRPATH_OK => {
                (*oc).good -= 1;
                (*oc).bad += 1;
                xrep_dirpath_set_outcome(dl, path, XCHK_DIRPATH_DELETE);
            }
            _ => {}
        }
    });
    ASSERT!((*oc).suspect == 0);
    ASSERT!((*oc).good == 0);
}

unsafe fn xrep_dirpath_retain_parent(dl: *mut xchk_dirtree, path: *mut xchk_dirpath) {
    let mut step: xchk_dirpath_step = core::mem::zeroed();
    let error = xfarray_load((*dl).path_steps, (*path).first_step, &mut step);
    if error != 0 { return; }
    (*dl).parent_ino = be64_to_cpu(step.pptr_rec.p_ino);
}

unsafe fn xrep_dirtree_find_surviving_path(dl: *mut xchk_dirtree, oc: *mut xchk_dirtree_outcomes) {
    let mut foundit = false;
    let mut path: *mut xchk_dirpath;
    xchk_dirtree_for_each_path!(dl, path, {
        match (*path).outcome {
            XCHK_DIRPATH_CORRUPT | XCHK_DIRPATH_LOOP | XCHK_DIRPATH_OK => {
                if !foundit {
                    xrep_dirpath_retain_parent(dl, path);
                    foundit = true;
                    continue;
                }
                ASSERT!(!foundit);
            }
            _ => {}
        }
    });
    ASSERT!((*oc).suspect + (*oc).good == 1);
}

unsafe fn xrep_dirtree_keep_one_good_path(dl: *mut xchk_dirtree, oc: *mut xchk_dirtree_outcomes) {
    let mut foundit = false;
    let mut path: *mut xchk_dirpath;
    xchk_dirtree_for_each_path!(dl, path, {
        match (*path).outcome {
            XCHK_DIRPATH_CORRUPT | XCHK_DIRPATH_LOOP => {
                (*oc).suspect -= 1; (*oc).bad += 1;
                xrep_dirpath_set_outcome(dl, path, XCHK_DIRPATH_DELETE);
            }
            XCHK_DIRPATH_OK => {
                if !foundit {
                    xrep_dirpath_retain_parent(dl, path); foundit = true; continue;
                }
                (*oc).good -= 1; (*oc).bad += 1;
                xrep_dirpath_set_outcome(dl, path, XCHK_DIRPATH_DELETE);
            }
            _ => {}
        }
    });
    ASSERT!((*oc).suspect == 0); ASSERT!((*oc).good < 2);
}

unsafe fn xrep_dirtree_keep_one_suspect_path(dl: *mut xchk_dirtree, oc: *mut xchk_dirtree_outcomes) {
    let mut foundit = false;
    let mut path: *mut xchk_dirpath;
    xchk_dirtree_for_each_path!(dl, path, {
        match (*path).outcome {
            XCHK_DIRPATH_CORRUPT | XCHK_DIRPATH_LOOP => {
                if !foundit {
                    xrep_dirpath_retain_parent(dl, path); foundit = true; continue;
                }
                (*oc).suspect -= 1; (*oc).bad += 1;
                xrep_dirpath_set_outcome(dl, path, XCHK_DIRPATH_DELETE);
            }
            XCHK_DIRPATH_OK => ASSERT!(false),
            _ => {}
        }
    });
    ASSERT!((*oc).suspect == 1); ASSERT!((*oc).good == 0);
}

unsafe fn xrep_dirtree_decide_fate(dl: *mut xchk_dirtree, oc: *mut xchk_dirtree_outcomes) {
    xchk_dirtree_evaluate(dl, oc);
    if xchk_dirtree_parentless(dl) { xrep_dirtree_delete_all_paths(dl, oc); return; }
    if (*oc).good + (*oc).suspect == 1 { xrep_dirtree_find_surviving_path(dl, oc); return; }
    if (*oc).good + (*oc).suspect == 0 {
        if !(*dl).sc.is_null() && !(*(*dl).sc).orphanage.is_null() { (*oc).needs_adoption = true; }
        return;
    }
    if (*oc).good > 0 { xrep_dirtree_keep_one_good_path(dl, oc); return; }
    xrep_dirtree_keep_one_suspect_path(dl, oc);
}

unsafe fn xrep_dirtree_prep_path(dl: *mut xchk_dirtree, path: *mut xchk_dirpath, step: *mut xchk_dirpath_step) -> i32 {
    let mut error = xfarray_load((*dl).path_steps, (*path).first_step, step);
    if error != 0 { return error; }
    error = xfblob_loadname((*dl).path_names, (*step).name_cookie, &mut (*dl).xname, (*step).name_len);
    if error != 0 { return error; }
    (*dl).pptr_rec = (*step).pptr_rec;
    0
}

unsafe fn xrep_dirtree_purge_dentry(dl: *mut xchk_dirtree, dp: *mut xfs_inode, name: *const xfs_name) -> i32 {
    let mut qname = QSTR_INIT((*name).name, (*name).len);
    let parent_dentry = d_find_alias(VFS_I(dp));
    if parent_dentry.is_null() { return 0; }
    let mut error = 0;
    if !d_is_dir(parent_dentry) { ASSERT!(d_is_dir(parent_dentry)); error = -EFSCORRUPTED; dput(parent_dentry); return error; }
    qname.hash = full_name_hash(parent_dentry, (*name).name, (*name).len);
    let child_dentry = d_lookup(parent_dentry, &qname);
    if child_dentry.is_null() { dput(parent_dentry); return 0; }
    trace_xrep_dirtree_delete_child((*dp).i_mount, child_dentry);
    if !d_is_dir(child_dentry) { ASSERT!(d_is_dir(child_dentry)); error = -EFSCORRUPTED; dput(child_dentry); dput(parent_dentry); return error; }
    d_delete(child_dentry);
    dput(child_dentry); dput(parent_dentry); error
}

#[inline]
unsafe fn xrep_dirtree_unlink_iolock(sc: *mut xfs_scrub, dp: *mut xfs_inode) -> i32 {
    ASSERT!((*sc).ilock_flags & XFS_IOLOCK_EXCL != 0);
    if sc == dp || xfs_ilock_nowait(dp, XFS_IOLOCK_EXCL) { return 0; }
    xchk_iunlock(sc, XFS_IOLOCK_EXCL);
    loop {
        xfs_ilock(dp, XFS_IOLOCK_EXCL);
        if xchk_ilock_nowait(sc, XFS_IOLOCK_EXCL) { break; }
        xfs_iunlock(dp, XFS_IOLOCK_EXCL);
        let mut error = 0;
        if xchk_should_terminate(sc, &mut error) { xchk_ilock(sc, XFS_IOLOCK_EXCL); return error; }
        delay(1);
    }
    0
}

unsafe fn xrep_dirtree_unlink(dl: *mut xchk_dirtree, dp: *mut xfs_inode, path: *mut xchk_dirpath, step: *mut xchk_dirpath_step) -> i32 {
    let sc = (*dl).sc; let mp = (*sc).mp;
    let mut dotdot_ino; let mut parent_ino = (*dl).parent_ino;
    let mut resblks; let mut dontcare = 0;
    let mut error = xrep_dirtree_unlink_iolock(sc, dp); if error != 0 { return error; }
    resblks = xfs_remove_space_res(mp, (*step).name_len);
    if sc == (*dl).sc && sc == sc { /* preserve the source's inode-special case */ }
    if sc == (*dl).sc && (*sc).ip == dp {
        loop { error = xfs_trans_alloc_inode(dp, &M_RES(mp).tr_remove, resblks, 0, false, &mut (*sc).tp); if (error == -ENOSPC || error == -EDQUOT) && resblks > 0 { resblks = 0; continue; } break; }
    } else { error = xfs_trans_alloc_dir(dp, &M_RES(mp).tr_remove, (*sc).ip, &mut resblks, &mut (*sc).tp, &mut dontcare); }
    if error != 0 { if dp != (*sc).ip { xfs_iunlock(dp, XFS_IOLOCK_EXCL); } return error; }
    mutex_lock(&mut (*dl).lock);
    if (*dl).stale { mutex_unlock(&mut (*dl).lock); error = -ESTALE; xchk_trans_cancel(sc); } else { xrep_dirpath_set_outcome(dl, path, XREP_DIRPATH_DELETING); mutex_unlock(&mut (*dl).lock); }
    if error == 0 {
        trace_xrep_dirtree_delete_path((*dl).sc, (*sc).ip, (*path).path_nr, &(*dl).xname, &(*dl).pptr_rec);
        error = xchk_dir_lookup(sc, (*sc).ip, &xfs_name_dotdot, &mut dotdot_ino);
        if error == 0 { if parent_ino == NULLFSINO { parent_ino = (*dl).root_ino; } if dotdot_ino == parent_ino { parent_ino = NULLFSINO; }
            error = xfs_droplink((*sc).tp, dp);
            if error == 0 && parent_ino != NULLFSINO { error = xfs_dir_replace((*sc).tp, (*sc).ip, &xfs_name_dotdot, parent_ino, 0); }
            if error == 0 { error = xfs_droplink((*sc).tp, (*sc).ip); }
            if error == 0 { error = xfs_dir_removename((*sc).tp, dp, &(*dl).xname, I_INO((*sc).ip), resblks); }
            if error == 0 && xfs_has_parent((*sc).mp) { error = xfs_parent_removename((*sc).tp, &(*dl).ppargs, dp, &(*dl).xname, (*sc).ip); }
            if error == 0 { xfs_dir_update_hook(dp, (*sc).ip, -1, &(*dl).xname); error = xrep_dirtree_purge_dentry(dl, dp, &(*dl).xname); }
        }
        if error == 0 { error = xrep_trans_commit(sc); } else { xchk_trans_cancel(sc); }
    }
    xfs_iunlock((*sc).ip, XFS_ILOCK_EXCL); if dp != (*sc).ip { xfs_iunlock(dp, XFS_ILOCK_EXCL); xfs_iunlock(dp, XFS_IOLOCK_EXCL); } error
}

unsafe fn xrep_dirtree_delete_path(dl: *mut xchk_dirtree, path: *mut xchk_dirpath) -> i32 {
    let mut step: xchk_dirpath_step = core::mem::zeroed(); let sc = (*dl).sc; let mut dp = core::ptr::null_mut();
    let mut error = xrep_dirtree_prep_path(dl, path, &mut step); if error != 0 { return error; }
    error = xchk_iget(sc, be64_to_cpu(step.pptr_rec.p_ino), &mut dp); if error != 0 { return error; }
    mutex_unlock(&mut (*dl).lock); xchk_trans_cancel(sc); xchk_iunlock(sc, XFS_ILOCK_EXCL);
    error = xrep_dirtree_unlink(dl, dp, path, &mut step); xchk_irele(sc, dp);
    xchk_trans_alloc_empty(sc); xchk_ilock(sc, XFS_ILOCK_EXCL); mutex_lock(&mut (*dl).lock);
    if error == 0 && (*dl).stale { error = -ESTALE; } error
}

unsafe fn xrep_dirtree_create_adoption_path(dl: *mut xchk_dirtree) -> i32 {
    let sc = (*dl).sc;
    if (*dl).nr_paths > XFS_MAXLINK { ASSERT!((*dl).nr_paths <= XFS_MAXLINK); return -EFSCORRUPTED; }
    let path = kmalloc_obj::<xchk_dirpath>(XCHK_GFP_FLAGS);
    if path.is_null() { return -ENOMEM; }
    INIT_LIST_HEAD(&mut (*path).list); xino_bitmap_init(&mut (*path).seen_inodes);
    (*path).nr_steps = 0; (*path).outcome = XREP_DIRPATH_ADOPTING;
    xfs_inode_to_parent_rec(&mut (*dl).pptr_rec, (*sc).orphanage);
    let mut error = xino_bitmap_set(&mut (*path).seen_inodes, I_INO((*sc).orphanage));
    if error != 0 { kfree(path); return error; }
    trace_xrep_dirtree_create_adoption(sc, (*sc).ip, (*dl).nr_paths, &(*dl).xname, &(*dl).pptr_rec);
    error = xchk_dirpath_append(dl, (*sc).ip, path, &(*dl).xname, &(*dl).pptr_rec);
    if error != 0 { kfree(path); return error; }
    (*path).first_step = xfarray_length((*dl).path_steps) - 1;
    (*path).second_step = XFARRAY_NULLIDX; (*path).path_nr = (*dl).nr_paths;
    list_add_tail(&mut (*path).list, &mut (*dl).path_list); (*dl).nr_paths += 1; 0
}

#[inline]
unsafe fn xrep_dirtree_adopt_iolock(sc: *mut xfs_scrub) -> i32 {
    ASSERT!((*sc).ilock_flags & XFS_IOLOCK_EXCL != 0);
    if xrep_orphanage_ilock_nowait(sc, XFS_IOLOCK_EXCL) { return 0; }
    xchk_iunlock(sc, XFS_IOLOCK_EXCL);
    loop {
        xrep_orphanage_ilock(sc, XFS_IOLOCK_EXCL);
        if xchk_ilock_nowait(sc, XFS_IOLOCK_EXCL) { break; }
        xrep_orphanage_iunlock(sc, XFS_IOLOCK_EXCL); let mut error = 0;
        if xchk_should_terminate(sc, &mut error) { xchk_ilock(sc, XFS_IOLOCK_EXCL); return error; }
        delay(1);
    } 0
}

unsafe fn xrep_dirtree_adopt(dl: *mut xchk_dirtree) -> i32 {
    let sc = (*dl).sc; let mut error = xrep_dirtree_adopt_iolock(sc); if error != 0 { return error; }
    error = xrep_adoption_trans_alloc(sc, &mut (*dl).adoption); if error != 0 { xrep_orphanage_iunlock(sc, XFS_IOLOCK_EXCL); return error; }
    (*dl).adoption.bump_child_nlink = true;
    error = xrep_adoption_compute_name(&mut (*dl).adoption, &mut (*dl).xname);
    if error == 0 { mutex_lock(&mut (*dl).lock); if (*dl).stale { mutex_unlock(&mut (*dl).lock); error = -ESTALE; } else { error = xrep_dirtree_create_adoption_path(dl); mutex_unlock(&mut (*dl).lock); } }
    if error == 0 { error = xrep_adoption_move(&mut (*dl).adoption); }
    if error == 0 { error = xrep_trans_commit(sc); } else { xchk_trans_cancel(sc); }
    xchk_iunlock(sc, XFS_ILOCK_EXCL); xrep_orphanage_iunlock(sc, XFS_ILOCK_EXCL); xrep_orphanage_iunlock(sc, XFS_IOLOCK_EXCL); error
}

unsafe fn xrep_dirtree_move_to_orphanage(dl: *mut xchk_dirtree) -> i32 {
    let sc = (*dl).sc; mutex_unlock(&mut (*dl).lock); xchk_trans_cancel(sc); xchk_iunlock(sc, XFS_ILOCK_EXCL);
    let mut error = xrep_dirtree_adopt(dl); xchk_trans_alloc_empty(sc); xchk_ilock(sc, XFS_ILOCK_EXCL); mutex_lock(&mut (*dl).lock);
    if error == 0 && (*dl).stale { error = -ESTALE; } error
}

unsafe fn xrep_dirtree_fix_problems(dl: *mut xchk_dirtree, oc: *mut xchk_dirtree_outcomes) -> i32 {
    let mut path: *mut xchk_dirpath;
    xchk_dirtree_for_each_path!(dl, path, { if (*path).outcome == XCHK_DIRPATH_DELETE { let error = xrep_dirtree_delete_path(dl, path); if error != 0 { return error; } } });
    if (*oc).needs_adoption { if xrep_orphanage_can_adopt((*dl).sc) { return xrep_dirtree_move_to_orphanage(dl); } return -EFSCORRUPTED; }
    0
}

pub unsafe fn xrep_dirtree(sc: *mut xfs_scrub) -> i32 {
    let dl = (*sc).buf as *mut xchk_dirtree; let mut oc: xchk_dirtree_outcomes = core::mem::zeroed(); let mut error;
    mutex_lock(&mut (*dl).lock);
    loop {
        if !(*dl).stale { xrep_dirtree_decide_fate(dl, &mut oc); trace_xrep_dirtree_decided_fate(dl, &oc); error = xrep_dirtree_fix_problems(dl, &mut oc); if error == 0 || error != -ESTALE { break; } }
        error = xchk_dirtree_find_paths_to_root(dl); if error == -ELNRNG || error == -ENOSR { error = -EFSCORRUPTED; }
        if error != 0 { break; }
    }
    mutex_unlock(&mut (*dl).lock); error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
