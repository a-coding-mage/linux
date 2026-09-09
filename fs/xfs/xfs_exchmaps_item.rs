// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2020-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// C header dependencies are supplied by the surrounding translation unit.

pub static mut xfs_xmi_cache: *mut kmem_cache = core::ptr::null_mut();
pub static mut xfs_xmd_cache: *mut kmem_cache = core::ptr::null_mut();

static xfs_xmi_item_ops: xfs_item_ops = xfs_item_ops {};

#[inline]
unsafe fn XMI_ITEM(lip: *mut xfs_log_item) -> *mut xfs_xmi_log_item {
    container_of!(lip, xfs_xmi_log_item, xmi_item)
}

unsafe fn xfs_xmi_item_free(xmi_lip: *mut xfs_xmi_log_item) {
    kvfree((*xmi_lip).xmi_item.li_lv_shadow);
    kmem_cache_free(xfs_xmi_cache, xmi_lip);
}

unsafe fn xfs_xmi_release(xmi_lip: *mut xfs_xmi_log_item) {
    ASSERT!(atomic_read(&(*xmi_lip).xmi_refcount) > 0);
    if atomic_dec_and_test(&mut (*xmi_lip).xmi_refcount) {
        xfs_trans_ail_delete(&mut (*xmi_lip).xmi_item, 0);
        xfs_xmi_item_free(xmi_lip);
    }
}

unsafe fn xfs_xmi_item_size(
    _lip: *mut xfs_log_item,
    nvecs: *mut i32,
    nbytes: *mut i32,
) {
    *nvecs += 1;
    *nbytes += core::mem::size_of::<xfs_xmi_log_format>() as i32;
}

unsafe fn xfs_xmi_item_format(lip: *mut xfs_log_item, lfb: *mut xlog_format_buf) {
    let xmi_lip = XMI_ITEM(lip);
    (*xmi_lip).xmi_format.xmi_type = XFS_LI_XMI;
    (*xmi_lip).xmi_format.xmi_size = 1;
    xlog_format_copy(
        lfb,
        XLOG_REG_TYPE_XMI_FORMAT,
        &(*xmi_lip).xmi_format,
        core::mem::size_of::<xfs_xmi_log_format>(),
    );
}

unsafe fn xfs_xmi_item_unpin(lip: *mut xfs_log_item, _remove: i32) {
    xfs_xmi_release(XMI_ITEM(lip));
}

unsafe fn xfs_xmi_item_release(lip: *mut xfs_log_item) {
    xfs_xmi_release(XMI_ITEM(lip));
}

unsafe fn xfs_xmi_init(mp: *mut xfs_mount) -> *mut xfs_xmi_log_item {
    let xmi_lip = kmem_cache_zalloc(xfs_xmi_cache, GFP_KERNEL | __GFP_NOFAIL)
        as *mut xfs_xmi_log_item;
    xfs_log_item_init(
        mp,
        &mut (*xmi_lip).xmi_item,
        XFS_LI_XMI,
        &xfs_xmi_item_ops,
    );
    (*xmi_lip).xmi_format.xmi_id = xmi_lip as usize;
    atomic_set(&mut (*xmi_lip).xmi_refcount, 2);
    xmi_lip
}

#[inline]
unsafe fn XMD_ITEM(lip: *mut xfs_log_item) -> *mut xfs_xmd_log_item {
    container_of!(lip, xfs_xmd_log_item, xmd_item)
}

unsafe fn xfs_xmd_item_size(
    _lip: *mut xfs_log_item,
    nvecs: *mut i32,
    nbytes: *mut i32,
) {
    *nvecs += 1;
    *nbytes += core::mem::size_of::<xfs_xmd_log_format>() as i32;
}

unsafe fn xfs_xmd_item_format(lip: *mut xfs_log_item, lfb: *mut xlog_format_buf) {
    let xmd_lip = XMD_ITEM(lip);
    (*xmd_lip).xmd_format.xmd_type = XFS_LI_XMD;
    (*xmd_lip).xmd_format.xmd_size = 1;
    xlog_format_copy(
        lfb,
        XLOG_REG_TYPE_XMD_FORMAT,
        &(*xmd_lip).xmd_format,
        core::mem::size_of::<xfs_xmd_log_format>(),
    );
}

unsafe fn xfs_xmd_item_release(lip: *mut xfs_log_item) {
    let xmd_lip = XMD_ITEM(lip);
    xfs_xmi_release((*xmd_lip).xmd_intent_log_item);
    kvfree((*xmd_lip).xmd_item.li_lv_shadow);
    kmem_cache_free(xfs_xmd_cache, xmd_lip);
}

unsafe fn xfs_xmd_item_intent(lip: *mut xfs_log_item) -> *mut xfs_log_item {
    &mut (*XMD_ITEM(lip)).xmd_intent_log_item.as_mut().unwrap().xmi_item
}

static xfs_xmd_item_ops: xfs_item_ops = xfs_item_ops {
    flags: XFS_ITEM_RELEASE_WHEN_COMMITTED | XFS_ITEM_INTENT_DONE,
    iop_size: Some(xfs_xmd_item_size),
    iop_format: Some(xfs_xmd_item_format),
    iop_release: Some(xfs_xmd_item_release),
    iop_intent: Some(xfs_xmd_item_intent),
};

unsafe fn xfs_exchmaps_create_intent(
    tp: *mut xfs_trans,
    items: *mut list_head,
    count: u32,
    _sort: bool,
) -> *mut xfs_log_item {
    ASSERT!(count == 1);
    let xmi = list_first_entry_or_null!(items, xfs_exchmaps_intent, xmi_list);
    let xmi_lip = xfs_xmi_init((*tp).t_mountp);
    let xlf = &mut (*xmi_lip).xmi_format;
    xlf.xmi_inode1 = I_INO((*xmi).xmi_ip1);
    xlf.xmi_igen1 = VFS_I((*xmi).xmi_ip1).i_generation;
    xlf.xmi_inode2 = I_INO((*xmi).xmi_ip2);
    xlf.xmi_igen2 = VFS_I((*xmi).xmi_ip2).i_generation;
    xlf.xmi_startoff1 = (*xmi).xmi_startoff1;
    xlf.xmi_startoff2 = (*xmi).xmi_startoff2;
    xlf.xmi_blockcount = (*xmi).xmi_blockcount;
    xlf.xmi_isize1 = (*xmi).xmi_isize1;
    xlf.xmi_isize2 = (*xmi).xmi_isize2;
    xlf.xmi_flags = (*xmi).xmi_flags & XFS_EXCHMAPS_LOGGED_FLAGS;
    &mut (*xmi_lip).xmi_item
}

unsafe fn xfs_exchmaps_create_done(
    tp: *mut xfs_trans,
    intent: *mut xfs_log_item,
    _count: u32,
) -> *mut xfs_log_item {
    let xmi_lip = XMI_ITEM(intent);
    let xmd_lip = kmem_cache_zalloc(xfs_xmd_cache, GFP_KERNEL | __GFP_NOFAIL)
        as *mut xfs_xmd_log_item;
    xfs_log_item_init(
        (*tp).t_mountp,
        &mut (*xmd_lip).xmd_item,
        XFS_LI_XMD,
        &xfs_xmd_item_ops,
    );
    (*xmd_lip).xmd_intent_log_item = xmi_lip;
    (*xmd_lip).xmd_format.xmd_xmi_id = (*xmi_lip).xmi_format.xmi_id;
    &mut (*xmd_lip).xmd_item
}

pub unsafe fn xfs_exchmaps_defer_add(tp: *mut xfs_trans, xmi: *mut xfs_exchmaps_intent) {
    trace_xfs_exchmaps_defer((*tp).t_mountp, xmi);
    xfs_defer_add(tp, &mut (*xmi).xmi_list, &xfs_exchmaps_defer_type);
}

#[inline]
unsafe fn xmi_entry(e: *const list_head) -> *mut xfs_exchmaps_intent {
    list_entry!(e, xfs_exchmaps_intent, xmi_list)
}

unsafe fn xfs_exchmaps_cancel_item(item: *mut list_head) {
    kmem_cache_free(xfs_exchmaps_intent_cache, xmi_entry(item));
}

unsafe fn xfs_exchmaps_finish_item(
    tp: *mut xfs_trans,
    _done: *mut xfs_log_item,
    item: *mut list_head,
    _state: *mut *mut xfs_btree_cur,
) -> i32 {
    let error = xfs_exchmaps_finish_one(tp, xmi_entry(item));
    if error != -EAGAIN {
        xfs_exchmaps_cancel_item(item);
    }
    error
}

unsafe fn xfs_exchmaps_abort_intent(intent: *mut xfs_log_item) {
    xfs_xmi_release(XMI_ITEM(intent));
}

unsafe fn xfs_xmi_validate(mp: *mut xfs_mount, xmi_lip: *mut xfs_xmi_log_item) -> bool {
    let xlf = &(*xmi_lip).xmi_format;
    if !xfs_has_exchange_range(mp) || xlf.__pad != 0 {
        return false;
    }
    if xlf.xmi_flags & !XFS_EXCHMAPS_LOGGED_FLAGS != 0 {
        return false;
    }
    if !xfs_verify_ino(mp, xlf.xmi_inode1) || !xfs_verify_ino(mp, xlf.xmi_inode2) {
        return false;
    }
    if !xfs_verify_fileext(mp, xlf.xmi_startoff1, xlf.xmi_blockcount) {
        return false;
    }
    xfs_verify_fileext(mp, xlf.xmi_startoff2, xlf.xmi_blockcount)
}

unsafe fn xfs_xmi_item_recover_intent(
    mp: *mut xfs_mount,
    dfp: *mut xfs_defer_pending,
    xlf: *const xfs_xmi_log_format,
    req: *mut xfs_exchmaps_req,
    ipp1: *mut *mut xfs_inode,
    ipp2: *mut *mut xfs_inode,
) -> *mut xfs_exchmaps_intent {
    let mut ip1 = core::ptr::null_mut();
    let mut ip2 = core::ptr::null_mut();
    let mut error = xlog_recover_iget_handle(mp, (*xlf).xmi_inode1, (*xlf).xmi_igen1, &mut ip1);
    if error != 0 {
        XFS_CORRUPTION_ERROR(__func__(), XFS_ERRLEVEL_LOW, mp, xlf, core::mem::size_of_val(&*xlf));
        return ERR_PTR(error) as *mut xfs_exchmaps_intent;
    }
    error = xlog_recover_iget_handle(mp, (*xlf).xmi_inode2, (*xlf).xmi_igen2, &mut ip2);
    if error != 0 {
        XFS_CORRUPTION_ERROR(__func__(), XFS_ERRLEVEL_LOW, mp, xlf, core::mem::size_of_val(&*xlf));
        goto!(err_rele1);
    }
    (*req).ip1 = ip1;
    (*req).ip2 = ip2;
    (*req).startoff1 = (*xlf).xmi_startoff1;
    (*req).startoff2 = (*xlf).xmi_startoff2;
    (*req).blockcount = (*xlf).xmi_blockcount;
    (*req).flags = (*xlf).xmi_flags & XFS_EXCHMAPS_PARAMS;
    xfs_exchrange_ilock(core::ptr::null_mut(), ip1, ip2);
    error = xfs_exchmaps_estimate(req);
    xfs_exchrange_iunlock(ip1, ip2);
    if error != 0 {
        goto!(err_rele2);
    }
    *ipp1 = ip1;
    *ipp2 = ip2;
    let xmi = xfs_exchmaps_init_intent(req);
    xfs_defer_add_item(dfp, &mut (*xmi).xmi_list);
    return xmi;
err_rele2:
    xfs_irele(ip2);
err_rele1:
    xfs_irele(ip1);
    (*req).ip2 = core::ptr::null_mut();
    (*req).ip1 = core::ptr::null_mut();
    ERR_PTR(error) as *mut xfs_exchmaps_intent
}

unsafe fn xfs_exchmaps_recover_work(
    dfp: *mut xfs_defer_pending,
    capture_list: *mut list_head,
) -> i32 {
    let mut req = xfs_exchmaps_req { flags: 0, ..core::mem::zeroed() };
    let mut resv = core::mem::zeroed::<xfs_trans_res>();
    let lip = (*dfp).dfp_intent;
    let xmi_lip = XMI_ITEM(lip);
    let mp = (*(*lip).li_log).l_mp;
    let mut tp = core::ptr::null_mut();
    let mut ip1 = core::ptr::null_mut();
    let mut ip2 = core::ptr::null_mut();
    let mut error = 0;
    if !xfs_xmi_validate(mp, xmi_lip) {
        XFS_CORRUPTION_ERROR(__func__(), XFS_ERRLEVEL_LOW, mp, &(*xmi_lip).xmi_format, core::mem::size_of_val(&(*xmi_lip).xmi_format));
        return -EFSCORRUPTED;
    }
    let xmi = xfs_xmi_item_recover_intent(mp, dfp, &(*xmi_lip).xmi_format, &mut req, &mut ip1, &mut ip2);
    if IS_ERR(xmi) { return PTR_ERR(xmi); }
    trace_xfs_exchmaps_recover(mp, xmi);
    resv = xlog_recover_resv(&M_RES(mp).tr_write);
    error = xfs_trans_alloc(mp, &resv, req.resblks, 0, 0, &mut tp);
    if error != 0 { goto!(err_rele); }
    xfs_exchrange_ilock(tp, ip1, ip2);
    xfs_exchmaps_ensure_reflink(tp, xmi);
    xfs_exchmaps_upgrade_extent_counts(tp, xmi);
    error = xlog_recover_finish_intent(tp, dfp);
    if error == -EFSCORRUPTED { XFS_CORRUPTION_ERROR(__func__(), XFS_ERRLEVEL_LOW, mp, &(*xmi_lip).xmi_format, core::mem::size_of_val(&(*xmi_lip).xmi_format)); }
    if error != 0 { goto!(err_cancel); }
    error = xfs_defer_ops_capture_and_commit(tp, capture_list);
    goto!(err_unlock);
err_cancel:
    xfs_trans_cancel(tp);
err_unlock:
    xfs_exchrange_iunlock(ip1, ip2);
err_rele:
    xfs_irele(ip2);
    xfs_irele(ip1);
    error
}

unsafe fn xfs_exchmaps_relog_intent(
    tp: *mut xfs_trans,
    intent: *mut xfs_log_item,
    _done_item: *mut xfs_log_item,
) -> *mut xfs_log_item {
    let old_xlf = &XMI_ITEM(intent).as_ref().unwrap().xmi_format;
    let xmi_lip = xfs_xmi_init((*tp).t_mountp);
    let new_xlf = &mut (*xmi_lip).xmi_format;
    new_xlf.xmi_inode1 = old_xlf.xmi_inode1;
    new_xlf.xmi_inode2 = old_xlf.xmi_inode2;
    new_xlf.xmi_igen1 = old_xlf.xmi_igen1;
    new_xlf.xmi_igen2 = old_xlf.xmi_igen2;
    new_xlf.xmi_startoff1 = old_xlf.xmi_startoff1;
    new_xlf.xmi_startoff2 = old_xlf.xmi_startoff2;
    new_xlf.xmi_blockcount = old_xlf.xmi_blockcount;
    new_xlf.xmi_flags = old_xlf.xmi_flags;
    new_xlf.xmi_isize1 = old_xlf.xmi_isize1;
    new_xlf.xmi_isize2 = old_xlf.xmi_isize2;
    &mut (*xmi_lip).xmi_item
}

pub static xfs_exchmaps_defer_type: xfs_defer_op_type = xfs_defer_op_type {
    name: "exchmaps",
    max_items: 1,
    create_intent: Some(xfs_exchmaps_create_intent),
    abort_intent: Some(xfs_exchmaps_abort_intent),
    create_done: Some(xfs_exchmaps_create_done),
    finish_item: Some(xfs_exchmaps_finish_item),
    cancel_item: Some(xfs_exchmaps_cancel_item),
    recover_work: Some(xfs_exchmaps_recover_work),
    relog_intent: Some(xfs_exchmaps_relog_intent),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
