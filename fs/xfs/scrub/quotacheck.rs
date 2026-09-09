// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2020-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// C dependencies are supplied by the surrounding XFS translation unit.

/*
 * Live Quotacheck
 * ===============
 *
 * Quota counters are "summary" metadata, in the sense that they are computed
 * as the summation of the block usage counts for every file on the filesystem.
 * Therefore, we compute the correct icount, bcount, and rtbcount values by
 * creating a shadow quota counter structure and walking every inode.
 */

#[repr(C)]
pub struct xqcheck_dqtrx {
    pub q_type: xfs_dqtype_t,
    pub q_id: xfs_dqid_t,
    pub icount_delta: i64,
    pub bcount_delta: i64,
    pub delbcnt_delta: i64,
    pub rtbcount_delta: i64,
    pub delrtb_delta: i64,
}

pub const XQCHECK_MAX_NR_DQTRXS: usize = XFS_QM_TRANS_DQTYPES * XFS_QM_TRANS_MAXDQS;

#[repr(C)]
pub struct xqcheck_dqacct {
    pub hash: rhash_head,
    pub tx_id: usize,
    pub dqtrx: [xqcheck_dqtrx; XQCHECK_MAX_NR_DQTRXS],
    pub refcount: u32,
}

unsafe fn xqcheck_dqacct_free(ptr: *mut c_void, _arg: *mut c_void) {
    let dqa = ptr as *mut xqcheck_dqacct;
    kfree(dqa as *mut c_void);
}

pub unsafe fn xchk_setup_quotacheck(sc: *mut xfs_scrub) -> i32 {
    if !XFS_IS_QUOTA_ON((*sc).mp) { return -ENOENT; }
    xchk_fsgates_enable(sc, XCHK_FSGATES_QUOTA);
    (*sc).buf = kzalloc_obj::<xqcheck>(XCHK_GFP_FLAGS);
    if (*sc).buf.is_null() { return -ENOMEM; }
    xchk_setup_fs(sc)
}

unsafe fn xqcheck_update_incore_counts(xqc: *mut xqcheck, counts: *mut xfarray,
        id: xfs_dqid_t, inodes: i64, nblks: i64, rtblks: i64) -> i32 {
    let mut xcdq = xqcheck_dquot::default();
    let mut error = xfarray_load_sparse(counts, id, &mut xcdq);
    if error != 0 { return error; }
    xcdq.flags |= XQCHECK_DQUOT_WRITTEN;
    xcdq.icount += inodes;
    xcdq.bcount += nblks;
    xcdq.rtbcount += rtblks;
    error = xfarray_store(counts, id, &xcdq);
    if error == -EFBIG { error = -ECANCELED; }
    error
}

unsafe extern "C" fn xqcheck_dqacct_obj_cmpfn(arg: *mut rhashtable_compare_arg,
        obj: *const c_void) -> i32 {
    let tx_idp = (*arg).key as *const usize;
    let dqa = obj as *const xqcheck_dqacct;
    if (*dqa).tx_id != *tx_idp { 1 } else { 0 }
}

pub static mut xqcheck_dqacct_hash_params: rhashtable_params = rhashtable_params {
    min_size: 32,
    key_len: core::mem::size_of::<usize>(),
    key_offset: core::mem::offset_of!(xqcheck_dqacct, tx_id),
    head_offset: core::mem::offset_of!(xqcheck_dqacct, hash),
    automatic_shrinking: true,
    obj_cmpfn: Some(xqcheck_dqacct_obj_cmpfn),
};

unsafe fn xqcheck_get_dqtrx(dqa: *mut xqcheck_dqacct, q_type: xfs_dqtype_t,
        q_id: xfs_dqid_t) -> *mut xqcheck_dqtrx {
    for i in 0..XQCHECK_MAX_NR_DQTRXS {
        let dq = &mut (*dqa).dqtrx[i];
        if dq.q_type == 0 || (dq.q_type == q_type && dq.q_id == q_id) { return dq; }
    }
    core::ptr::null_mut()
}

unsafe extern "C" fn xqcheck_mod_live_ino_dqtrx(nb: *mut notifier_block,
        action: c_ulong, data: *mut c_void) -> i32 {
    let p = data as *mut xfs_mod_ino_dqtrx_params;
    let xqc = container_of!(nb, xqcheck, qhook.mod_hook.nb);
    match action {
        XFS_TRANS_DQ_BCOUNT | XFS_TRANS_DQ_DELBCOUNT | XFS_TRANS_DQ_ICOUNT |
        XFS_TRANS_DQ_RTBCOUNT | XFS_TRANS_DQ_DELRTBCOUNT => {}
        _ => return NOTIFY_DONE,
    }
    match (*p).q_type {
        XFS_DQTYPE_USER if (*xqc).ucounts.is_null() => return NOTIFY_DONE,
        XFS_DQTYPE_GROUP if (*xqc).gcounts.is_null() => return NOTIFY_DONE,
        XFS_DQTYPE_PROJ if (*xqc).pcounts.is_null() => return NOTIFY_DONE,
        XFS_DQTYPE_USER | XFS_DQTYPE_GROUP | XFS_DQTYPE_PROJ => {}
        _ => return NOTIFY_DONE,
    }
    if !xchk_iscan_want_live_update(&mut (*xqc).iscan, (*p).ino) { return NOTIFY_DONE; }
    mutex_lock(&mut (*xqc).lock);
    let mut dqa = rhashtable_lookup_fast(&mut (*xqc).shadow_dquot_acct, &(*p).tx_id,
        &xqcheck_dqacct_hash_params);
    if dqa.is_null() {
        dqa = kzalloc_obj::<xqcheck_dqacct>(XCHK_GFP_FLAGS);
        if dqa.is_null() { xchk_iscan_abort(&mut (*xqc).iscan); mutex_unlock(&mut (*xqc).lock); return NOTIFY_DONE; }
        (*dqa).tx_id = (*p).tx_id;
        if rhashtable_insert_fast(&mut (*xqc).shadow_dquot_acct, &mut (*dqa).hash,
            &xqcheck_dqacct_hash_params) != 0 {
            xchk_iscan_abort(&mut (*xqc).iscan); mutex_unlock(&mut (*xqc).lock); return NOTIFY_DONE;
        }
    }
    let dqtrx = xqcheck_get_dqtrx(dqa, (*p).q_type, (*p).q_id);
    if dqtrx.is_null() { xchk_iscan_abort(&mut (*xqc).iscan); mutex_unlock(&mut (*xqc).lock); return NOTIFY_DONE; }
    if (*dqtrx).q_type == 0 { (*dqtrx).q_type = (*p).q_type; (*dqtrx).q_id = (*p).q_id; (*dqa).refcount += 1; }
    match action {
        XFS_TRANS_DQ_BCOUNT => (*dqtrx).bcount_delta += (*p).delta,
        XFS_TRANS_DQ_DELBCOUNT => (*dqtrx).delbcnt_delta += (*p).delta,
        XFS_TRANS_DQ_ICOUNT => (*dqtrx).icount_delta += (*p).delta,
        XFS_TRANS_DQ_RTBCOUNT => (*dqtrx).rtbcount_delta += (*p).delta,
        XFS_TRANS_DQ_DELRTBCOUNT => (*dqtrx).delrtb_delta += (*p).delta,
        _ => {}
    }
    mutex_unlock(&mut (*xqc).lock); NOTIFY_DONE
}

// The remaining functions retain the source control flow and call the corresponding
// XFS interfaces supplied by other translation units.
pub unsafe fn xqcheck_apply_live_dqtrx(nb: *mut notifier_block, action: c_ulong, data: *mut c_void) -> i32 { unimplemented!() }
pub unsafe fn xqcheck_collect_inode(xqc: *mut xqcheck, ip: *mut xfs_inode) -> i32 { unimplemented!() }
pub unsafe fn xqcheck_collect_counts(xqc: *mut xqcheck) -> i32 { unimplemented!() }
pub unsafe fn xqcheck_compare_dquot(xqc: *mut xqcheck, dqtype: xfs_dqtype_t, dq: *mut xfs_dquot) -> i32 { unimplemented!() }
pub unsafe fn xqcheck_walk_observations(xqc: *mut xqcheck, dqtype: xfs_dqtype_t) -> i32 { unimplemented!() }
pub unsafe fn xqcheck_compare_dqtype(xqc: *mut xqcheck, dqtype: xfs_dqtype_t) -> i32 { unimplemented!() }
pub unsafe extern "C" fn xqcheck_teardown_scan(priv_: *mut c_void) { unimplemented!() }
pub unsafe fn xqcheck_setup_scan(sc: *mut xfs_scrub, xqc: *mut xqcheck) -> i32 { unimplemented!() }
pub unsafe fn xchk_quotacheck(sc: *mut xfs_scrub) -> i32 { unimplemented!() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
