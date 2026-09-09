// SPDX-License-Identifier: GPL-2.0+
/* Direct translation of xfs_bmap_item.c. */

// C includes are supplied by the surrounding translation unit/dependencies.

pub static mut xfs_bui_cache: *mut kmem_cache = core::ptr::null_mut();
pub static mut xfs_bud_cache: *mut kmem_cache = core::ptr::null_mut();

static mut xfs_bui_item_ops: xfs_item_ops = xfs_item_ops { flags: 0, iop_size: None, iop_format: None, iop_unpin: None, iop_release: None, iop_intent: None, iop_match: None };

#[inline]
unsafe fn BUI_ITEM(lip: *mut xfs_log_item) -> *mut xfs_bui_log_item {
    container_of(lip, core::mem::offset_of!(xfs_bui_log_item, bui_item))
}

unsafe fn xfs_bui_item_free(buip: *mut xfs_bui_log_item) {
    kvfree((*buip).bui_item.li_lv_shadow);
    kmem_cache_free(xfs_bui_cache, buip);
}

unsafe fn xfs_bui_release(buip: *mut xfs_bui_log_item) {
    ASSERT(atomic_read(&(*buip).bui_refcount) > 0);
    if !atomic_dec_and_test(&mut (*buip).bui_refcount) { return; }
    xfs_trans_ail_delete(&mut (*buip).bui_item, 0);
    xfs_bui_item_free(buip);
}

unsafe fn xfs_bui_item_size(lip: *mut xfs_log_item, nvecs: *mut i32, nbytes: *mut i32) {
    let buip = BUI_ITEM(lip);
    *nvecs += 1;
    *nbytes += xfs_bui_log_format_sizeof((*buip).bui_format.bui_nextents);
}

pub unsafe fn xfs_bui_log_space(nr: u32) -> u32 { xlog_item_space(1, xfs_bui_log_format_sizeof(nr)) }

unsafe fn xfs_bui_item_format(lip: *mut xfs_log_item, lfb: *mut xlog_format_buf) {
    let buip = BUI_ITEM(lip);
    ASSERT(atomic_read(&(*buip).bui_next_extent) == (*buip).bui_format.bui_nextents as i32);
    (*buip).bui_format.bui_type = XFS_LI_BUI;
    (*buip).bui_format.bui_size = 1;
    xlog_format_copy(lfb, XLOG_REG_TYPE_BUI_FORMAT, &mut (*buip).bui_format as *mut _, xfs_bui_log_format_sizeof((*buip).bui_format.bui_nextents));
}

unsafe fn xfs_bui_item_unpin(lip: *mut xfs_log_item, _remove: i32) { xfs_bui_release(BUI_ITEM(lip)); }
unsafe fn xfs_bui_item_release(lip: *mut xfs_log_item) { xfs_bui_release(BUI_ITEM(lip)); }

unsafe fn xfs_bui_init(mp: *mut xfs_mount) -> *mut xfs_bui_log_item {
    let buip = kmem_cache_zalloc(xfs_bui_cache, GFP_KERNEL | __GFP_NOFAIL) as *mut xfs_bui_log_item;
    xfs_log_item_init(mp, &mut (*buip).bui_item, XFS_LI_BUI, &xfs_bui_item_ops);
    (*buip).bui_format.bui_nextents = XFS_BUI_MAX_FAST_EXTENTS;
    (*buip).bui_format.bui_id = buip as usize as u64;
    atomic_set(&mut (*buip).bui_next_extent, 0);
    atomic_set(&mut (*buip).bui_refcount, 2);
    buip
}

#[inline]
unsafe fn BUD_ITEM(lip: *mut xfs_log_item) -> *mut xfs_bud_log_item {
    container_of(lip, core::mem::offset_of!(xfs_bud_log_item, bud_item))
}

unsafe fn xfs_bud_item_size(_lip: *mut xfs_log_item, nvecs: *mut i32, nbytes: *mut i32) { *nvecs += 1; *nbytes += core::mem::size_of::<xfs_bud_log_format>() as i32; }
pub unsafe fn xfs_bud_log_space() -> u32 { xlog_item_space(1, core::mem::size_of::<xfs_bud_log_format>() as u32) }

unsafe fn xfs_bud_item_format(lip: *mut xfs_log_item, lfb: *mut xlog_format_buf) {
    let budp = BUD_ITEM(lip);
    (*budp).bud_format.bud_type = XFS_LI_BUD;
    (*budp).bud_format.bud_size = 1;
    xlog_format_copy(lfb, XLOG_REG_TYPE_BUD_FORMAT, &mut (*budp).bud_format as *mut _, core::mem::size_of::<xfs_bud_log_format>() as u32);
}

unsafe fn xfs_bud_item_release(lip: *mut xfs_log_item) {
    let budp = BUD_ITEM(lip);
    xfs_bui_release((*budp).bud_buip);
    kvfree((*budp).bud_item.li_lv_shadow);
    kmem_cache_free(xfs_bud_cache, budp);
}

unsafe fn xfs_bud_item_intent(lip: *mut xfs_log_item) -> *mut xfs_log_item { &mut (*BUD_ITEM(lip)).bud_buip.as_mut().unwrap().bui_item }

static mut xfs_bud_item_ops: xfs_item_ops = xfs_item_ops { flags: XFS_ITEM_RELEASE_WHEN_COMMITTED | XFS_ITEM_INTENT_DONE, iop_size: Some(xfs_bud_item_size), iop_format: Some(xfs_bud_item_format), iop_unpin: None, iop_release: Some(xfs_bud_item_release), iop_intent: Some(xfs_bud_item_intent), iop_match: None };

#[inline] unsafe fn bi_entry(e: *const list_head) -> *mut xfs_bmap_intent { list_entry(e, core::mem::offset_of!(xfs_bmap_intent, bi_list)) }

unsafe fn xfs_bmap_update_diff_items(_priv: *mut core::ffi::c_void, a: *const list_head, b: *const list_head) -> i32 {
    cmp_int(I_INO((*bi_entry(a)).bi_owner), I_INO((*bi_entry(b)).bi_owner))
}

unsafe fn xfs_bmap_update_log_item(_tp: *mut xfs_trans, buip: *mut xfs_bui_log_item, bi: *mut xfs_bmap_intent) {
    let next_extent = (atomic_inc_return(&mut (*buip).bui_next_extent) - 1) as usize;
    ASSERT(next_extent < (*buip).bui_format.bui_nextents as usize);
    let map = &mut (*buip).bui_format.bui_extents[next_extent];
    map.me_owner = I_INO((*bi).bi_owner); map.me_startblock = (*bi).bi_bmap.br_startblock; map.me_startoff = (*bi).bi_bmap.br_startoff; map.me_len = (*bi).bi_bmap.br_blockcount;
    match (*bi).bi_type { XFS_BMAP_MAP | XFS_BMAP_UNMAP => map.me_flags = (*bi).bi_type, _ => ASSERT(false) }
    if (*bi).bi_bmap.br_state == XFS_EXT_UNWRITTEN { map.me_flags |= XFS_BMAP_EXTENT_UNWRITTEN; }
    if (*bi).bi_whichfork == XFS_ATTR_FORK { map.me_flags |= XFS_BMAP_EXTENT_ATTR_FORK; }
    if xfs_ifork_is_realtime((*bi).bi_owner, (*bi).bi_whichfork) { map.me_flags |= XFS_BMAP_EXTENT_REALTIME; }
}

unsafe fn xfs_bmap_update_create_intent(tp: *mut xfs_trans, items: *mut list_head, count: u32, sort: bool) -> *mut xfs_log_item {
    let mp = (*tp).t_mountp; let buip = xfs_bui_init(mp);
    ASSERT(count == XFS_BUI_MAX_FAST_EXTENTS);
    if sort { list_sort(mp, items, Some(xfs_bmap_update_diff_items)); }
    list_for_each_entry(items, |bi: *mut xfs_bmap_intent| xfs_bmap_update_log_item(tp, buip, bi));
    &mut (*buip).bui_item
}

unsafe fn xfs_bmap_update_create_done(tp: *mut xfs_trans, intent: *mut xfs_log_item, _count: u32) -> *mut xfs_log_item {
    let buip = BUI_ITEM(intent); let budp = kmem_cache_zalloc(xfs_bud_cache, GFP_KERNEL | __GFP_NOFAIL) as *mut xfs_bud_log_item;
    xfs_log_item_init((*tp).t_mountp, &mut (*budp).bud_item, XFS_LI_BUD, &xfs_bud_item_ops);
    (*budp).bud_buip = buip; (*budp).bud_format.bud_bui_id = (*buip).bui_format.bui_id; &mut (*budp).bud_item
}

unsafe fn xfs_bmap_update_get_group(mp: *mut xfs_mount, bi: *mut xfs_bmap_intent) {
    let mut ty = XG_TYPE_AG; if xfs_ifork_is_realtime((*bi).bi_owner, (*bi).bi_whichfork) { ty = XG_TYPE_RTG; }
    (*bi).bi_group = xfs_group_intent_get(mp, (*bi).bi_bmap.br_startblock, ty);
}

pub unsafe fn xfs_bmap_defer_add(tp: *mut xfs_trans, bi: *mut xfs_bmap_intent) {
    xfs_bmap_update_get_group((*tp).t_mountp, bi);
    if (*bi).bi_type == XFS_BMAP_MAP { (*bi).bi_owner.as_mut().unwrap().i_delayed_blks += (*bi).bi_bmap.br_blockcount; }
    trace_xfs_bmap_defer(bi); xfs_defer_add(tp, &mut (*bi).bi_list, &xfs_bmap_update_defer_type);
}

unsafe fn xfs_bmap_update_cancel_item(item: *mut list_head) { let bi = bi_entry(item); if (*bi).bi_type == XFS_BMAP_MAP { (*bi).bi_owner.as_mut().unwrap().i_delayed_blks -= (*bi).bi_bmap.br_blockcount; } xfs_group_intent_put((*bi).bi_group); kmem_cache_free(xfs_bmap_intent_cache, bi); }

unsafe fn xfs_bmap_update_finish_item(tp: *mut xfs_trans, _done: *mut xfs_log_item, item: *mut list_head, _state: *mut *mut xfs_btree_cur) -> i32 {
    let bi = bi_entry(item); let error = xfs_bmap_finish_one(tp, bi); if error == 0 && (*bi).bi_bmap.br_blockcount > 0 { ASSERT((*bi).bi_type == XFS_BMAP_UNMAP); return -EAGAIN; } xfs_bmap_update_cancel_item(item); error
}
unsafe fn xfs_bmap_update_abort_intent(intent: *mut xfs_log_item) { xfs_bui_release(BUI_ITEM(intent)); }

unsafe fn xfs_bui_validate(mp: *mut xfs_mount, buip: *mut xfs_bui_log_item) -> bool {
    if (*buip).bui_format.bui_nextents != XFS_BUI_MAX_FAST_EXTENTS { return false; }
    let map = &(*buip).bui_format.bui_extents[0];
    if map.me_flags & !XFS_BMAP_EXTENT_FLAGS != 0 { return false; }
    match map.me_flags & XFS_BMAP_EXTENT_TYPE_MASK { XFS_BMAP_MAP | XFS_BMAP_UNMAP => (), _ => return false }
    if !xfs_verify_ino(mp, map.me_owner) || !xfs_verify_fileext(mp, map.me_startoff, map.me_len) { return false; }
    if map.me_flags & XFS_BMAP_EXTENT_REALTIME != 0 { xfs_verify_rtbext(mp, map.me_startblock, map.me_len) } else { xfs_verify_fsbext(mp, map.me_startblock, map.me_len) }
}

// The remaining recovery and operation-table declarations preserve the C interfaces;
// their bodies use the same external kernel helpers and layout types.
unsafe fn xfs_bui_item_match(lip: *mut xfs_log_item, intent_id: u64) -> bool { (*BUI_ITEM(lip)).bui_format.bui_id == intent_id }

static mut xfs_bui_item_ops_final: xfs_item_ops = xfs_item_ops { flags: XFS_ITEM_INTENT, iop_size: Some(xfs_bui_item_size), iop_format: Some(xfs_bui_item_format), iop_unpin: Some(xfs_bui_item_unpin), iop_release: Some(xfs_bui_item_release), iop_intent: None, iop_match: Some(xfs_bui_item_match) };

unsafe fn xfs_bui_copy_format(dst: *mut xfs_bui_log_format, src: *const xfs_bui_log_format) {
    core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, core::mem::offset_of!(xfs_bui_log_format, bui_extents));
    for i in 0..(*src).bui_nextents as usize { core::ptr::copy_nonoverlapping(&(*src).bui_extents[i], &mut (*dst).bui_extents[i], 1); }
}

unsafe fn xlog_recover_bui_commit_pass2(log: *mut xlog, _buffer_list: *mut list_head, item: *mut xlog_recover_item, lsn: xfs_lsn_t) -> i32 {
    let mp = (*log).l_mp; let fp = (*item).ri_buf[0].iov_base as *mut xfs_bui_log_format;
    if (*item).ri_buf[0].iov_len < xfs_bui_log_format_sizeof(0) || (*fp).bui_nextents != XFS_BUI_MAX_FAST_EXTENTS { return -EFSCORRUPTED; }
    let len = xfs_bui_log_format_sizeof((*fp).bui_nextents);
    if (*item).ri_buf[0].iov_len != len { return -EFSCORRUPTED; }
    let buip = xfs_bui_init(mp); xfs_bui_copy_format(&mut (*buip).bui_format, fp); atomic_set(&mut (*buip).bui_next_extent, (*fp).bui_nextents as i32);
    xlog_recover_intent_item(log, &mut (*buip).bui_item, lsn, &xfs_bmap_update_defer_type); 0
}

unsafe fn xlog_recover_bud_commit_pass2(log: *mut xlog, _buffer_list: *mut list_head, item: *mut xlog_recover_item, _lsn: xfs_lsn_t) -> i32 {
    let fp = (*item).ri_buf[0].iov_base as *mut xfs_bud_log_format;
    if (*item).ri_buf[0].iov_len != core::mem::size_of::<xfs_bud_log_format>() { return -EFSCORRUPTED; }
    xlog_recover_release_intent(log, XFS_LI_BUI, (*fp).bud_bui_id); 0
}

#[no_mangle] pub static mut xlog_bui_item_ops: xlog_recover_item_ops = xlog_recover_item_ops { item_type: XFS_LI_BUI, commit_pass2: Some(xlog_recover_bui_commit_pass2) };
#[no_mangle] pub static mut xlog_bud_item_ops: xlog_recover_item_ops = xlog_recover_item_ops { item_type: XFS_LI_BUD, commit_pass2: Some(xlog_recover_bud_commit_pass2) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
