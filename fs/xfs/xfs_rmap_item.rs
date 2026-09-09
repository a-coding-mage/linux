// SPDX-License-Identifier: GPL-2.0+
/* Direct Rust translation of xfs_rmap_item.c. */

// External XFS/kernel types, constants, macros, and functions are supplied by
// the surrounding translated repository.

extern "C" {
    static mut xfs_rui_cache: *mut kmem_cache;
    static mut xfs_rud_cache: *mut kmem_cache;
    static xfs_rui_item_ops: xfs_item_ops;
    static xfs_rud_item_ops: xfs_item_ops;
    static xfs_rmap_update_defer_type: xfs_defer_op_type;
    static xfs_rtrmap_update_defer_type: xfs_defer_op_type;
}

#[inline]
unsafe fn RUI_ITEM(lip: *mut xfs_log_item) -> *mut xfs_rui_log_item {
    container_of(lip, (*lip).rui_item)
}

unsafe fn xfs_rui_item_free(ruip: *mut xfs_rui_log_item) {
    kvfree((*ruip).rui_item.li_lv_shadow);
    if (*ruip).rui_format.rui_nextents > XFS_RUI_MAX_FAST_EXTENTS {
        kfree(ruip as *mut _);
    } else {
        kmem_cache_free(xfs_rui_cache, ruip as *mut _);
    }
}

unsafe fn xfs_rui_release(ruip: *mut xfs_rui_log_item) {
    ASSERT(atomic_read(&(*ruip).rui_refcount) > 0);
    if !atomic_dec_and_test(&mut (*ruip).rui_refcount) { return; }
    xfs_trans_ail_delete(&mut (*ruip).rui_item, 0);
    xfs_rui_item_free(ruip);
}

unsafe fn xfs_rui_item_size(lip: *mut xfs_log_item, nvecs: *mut c_int, nbytes: *mut c_int) {
    let ruip = RUI_ITEM(lip);
    *nvecs += 1;
    *nbytes += xfs_rui_log_format_sizeof((*ruip).rui_format.rui_nextents) as c_int;
}

pub unsafe fn xfs_rui_log_space(nr: c_uint) -> c_uint {
    xlog_item_space(1, xfs_rui_log_format_sizeof(nr))
}

unsafe fn xfs_rui_item_format(lip: *mut xfs_log_item, lfb: *mut xlog_format_buf) {
    let ruip = RUI_ITEM(lip);
    ASSERT(atomic_read(&(*ruip).rui_next_extent) == (*ruip).rui_format.rui_nextents as c_int);
    ASSERT((*lip).li_type == XFS_LI_RUI || (*lip).li_type == XFS_LI_RUI_RT);
    (*ruip).rui_format.rui_type = (*lip).li_type;
    (*ruip).rui_format.rui_size = 1;
    xlog_format_copy(lfb, XLOG_REG_TYPE_RUI_FORMAT, &mut (*ruip).rui_format as *mut _,
        xfs_rui_log_format_sizeof((*ruip).rui_format.rui_nextents));
}

unsafe fn xfs_rui_item_unpin(lip: *mut xfs_log_item, _remove: c_int) {
    xfs_rui_release(RUI_ITEM(lip));
}
unsafe fn xfs_rui_item_release(lip: *mut xfs_log_item) { xfs_rui_release(RUI_ITEM(lip)); }

unsafe fn xfs_rui_init(mp: *mut xfs_mount, item_type: c_ushort, nextents: c_uint) -> *mut xfs_rui_log_item {
    ASSERT(nextents > 0);
    ASSERT(item_type == XFS_LI_RUI || item_type == XFS_LI_RUI_RT);
    let ruip = if nextents > XFS_RUI_MAX_FAST_EXTENTS {
        kzalloc(xfs_rui_log_item_sizeof(nextents), GFP_KERNEL | __GFP_NOFAIL) as *mut xfs_rui_log_item
    } else { kmem_cache_zalloc(xfs_rui_cache, GFP_KERNEL | __GFP_NOFAIL) as *mut xfs_rui_log_item };
    xfs_log_item_init(mp, &mut (*ruip).rui_item, item_type, &xfs_rui_item_ops);
    (*ruip).rui_format.rui_nextents = nextents;
    (*ruip).rui_format.rui_id = ruip as usize as u64;
    atomic_set(&mut (*ruip).rui_next_extent, 0);
    atomic_set(&mut (*ruip).rui_refcount, 2);
    ruip
}

#[inline] unsafe fn RUD_ITEM(lip: *mut xfs_log_item) -> *mut xfs_rud_log_item { container_of(lip, (*lip).rud_item) }
unsafe fn xfs_rud_item_size(_lip: *mut xfs_log_item, nvecs: *mut c_int, nbytes: *mut c_int) { *nvecs += 1; *nbytes += core::mem::size_of::<xfs_rud_log_format>() as c_int; }
pub unsafe fn xfs_rud_log_space() -> c_uint { xlog_item_space(1, core::mem::size_of::<xfs_rud_log_format>()) }
unsafe fn xfs_rud_item_format(lip: *mut xfs_log_item, lfb: *mut xlog_format_buf) {
    let rudp = RUD_ITEM(lip); ASSERT((*lip).li_type == XFS_LI_RUD || (*lip).li_type == XFS_LI_RUD_RT);
    (*rudp).rud_format.rud_type = (*lip).li_type; (*rudp).rud_format.rud_size = 1;
    xlog_format_copy(lfb, XLOG_REG_TYPE_RUD_FORMAT, &mut (*rudp).rud_format as *mut _, core::mem::size_of::<xfs_rud_log_format>());
}
unsafe fn xfs_rud_item_release(lip: *mut xfs_log_item) { let rudp=RUD_ITEM(lip); xfs_rui_release((*rudp).rud_ruip); kvfree((*rudp).rud_item.li_lv_shadow); kmem_cache_free(xfs_rud_cache, rudp as *mut _); }
unsafe fn xfs_rud_item_intent(lip: *mut xfs_log_item) -> *mut xfs_log_item { &mut (*RUD_ITEM(lip)).rud_ruip.as_mut().unwrap().rui_item }

#[allow(non_upper_case_globals)]
static xfs_rud_item_ops_local: xfs_item_ops = xfs_item_ops { flags: XFS_ITEM_RELEASE_WHEN_COMMITTED | XFS_ITEM_INTENT_DONE, iop_size: Some(xfs_rud_item_size), iop_format: Some(xfs_rud_item_format), iop_release: Some(xfs_rud_item_release), iop_intent: Some(xfs_rud_item_intent) };

#[inline] unsafe fn ri_entry(e: *const list_head) -> *mut xfs_rmap_intent { list_entry(e, xfs_rmap_intent, ri_list) }
#[inline] unsafe fn xfs_rui_item_isrt(lip: *const xfs_log_item) -> bool { ASSERT((*lip).li_type == XFS_LI_RUI || (*lip).li_type == XFS_LI_RUI_RT); (*lip).li_type == XFS_LI_RUI_RT }

unsafe fn xfs_rmap_update_diff_items(_priv: *mut c_void, a: *const list_head, b: *const list_head) -> c_int { cmp_int((*ri_entry(a)).ri_group.xg_gno, (*ri_entry(b)).ri_group.xg_gno) }

unsafe fn xfs_rmap_update_log_item(_tp: *mut xfs_trans, ruip: *mut xfs_rui_log_item, ri: *mut xfs_rmap_intent) {
    let next_extent = (atomic_inc_return(&mut (*ruip).rui_next_extent) - 1) as usize;
    ASSERT(next_extent < (*ruip).rui_format.rui_nextents as usize);
    let map = &mut (*ruip).rui_format.rui_extents[next_extent];
    map.me_owner=(*ri).ri_owner; map.me_startblock=(*ri).ri_bmap.br_startblock; map.me_startoff=(*ri).ri_bmap.br_startoff; map.me_len=(*ri).ri_bmap.br_blockcount; map.me_flags=0;
    if (*ri).ri_bmap.br_state == XFS_EXT_UNWRITTEN { map.me_flags |= XFS_RMAP_EXTENT_UNWRITTEN; }
    if (*ri).ri_whichfork == XFS_ATTR_FORK { map.me_flags |= XFS_RMAP_EXTENT_ATTR_FORK; }
    map.me_flags |= match (*ri).ri_type { XFS_RMAP_MAP=>XFS_RMAP_EXTENT_MAP, XFS_RMAP_MAP_SHARED=>XFS_RMAP_EXTENT_MAP_SHARED, XFS_RMAP_UNMAP=>XFS_RMAP_EXTENT_UNMAP, XFS_RMAP_UNMAP_SHARED=>XFS_RMAP_EXTENT_UNMAP_SHARED, XFS_RMAP_CONVERT=>XFS_RMAP_EXTENT_CONVERT, XFS_RMAP_CONVERT_SHARED=>XFS_RMAP_EXTENT_CONVERT_SHARED, XFS_RMAP_ALLOC=>XFS_RMAP_EXTENT_ALLOC, XFS_RMAP_FREE=>XFS_RMAP_EXTENT_FREE, _=>{ASSERT(false);0} };
}

unsafe fn __xfs_rmap_update_create_intent(tp:*mut xfs_trans, items:*mut list_head, count:c_uint, sort:bool, item_type:c_ushort)->*mut xfs_log_item { let ruip=xfs_rui_init((*tp).t_mountp,item_type,count); if sort { list_sort((*tp).t_mountp,items,Some(xfs_rmap_update_diff_items)); } list_for_each_entry(items, |ri| xfs_rmap_update_log_item(tp,ruip,ri)); &mut (*ruip).rui_item }
unsafe fn xfs_rmap_update_create_intent(tp:*mut xfs_trans,items:*mut list_head,count:c_uint,sort:bool)->*mut xfs_log_item { __xfs_rmap_update_create_intent(tp,items,count,sort,XFS_LI_RUI) }
#[inline] unsafe fn xfs_rud_type_from_rui(ruip:*const xfs_rui_log_item)->c_ushort { if xfs_rui_item_isrt(&(*ruip).rui_item) {XFS_LI_RUD_RT} else {XFS_LI_RUD} }

unsafe fn xfs_rmap_update_create_done(tp:*mut xfs_trans,intent:*mut xfs_log_item,_count:c_uint)->*mut xfs_log_item { let ruip=RUI_ITEM(intent); let rudp=kmem_cache_zalloc(xfs_rud_cache,GFP_KERNEL|__GFP_NOFAIL) as *mut xfs_rud_log_item; xfs_log_item_init((*tp).t_mountp,&mut (*rudp).rud_item,xfs_rud_type_from_rui(ruip),&xfs_rud_item_ops_local); (*rudp).rud_ruip=ruip; (*rudp).rud_format.rud_rui_id=(*ruip).rui_format.rui_id; &mut (*rudp).rud_item }

pub unsafe fn xfs_rmap_defer_add(tp:*mut xfs_trans,ri:*mut xfs_rmap_intent) { let mp=(*tp).t_mountp; (*ri).ri_group=xfs_group_intent_get(mp,(*ri).ri_bmap.br_startblock,if (*ri).ri_realtime {XG_TYPE_RTG}else{XG_TYPE_AG}); trace_xfs_rmap_defer(mp,ri); xfs_defer_add(tp,&mut (*ri).ri_list,if (*ri).ri_realtime {&xfs_rtrmap_update_defer_type}else{&xfs_rmap_update_defer_type}); }
unsafe fn xfs_rmap_update_cancel_item(item:*mut list_head){let ri=ri_entry(item);xfs_group_intent_put((*ri).ri_group);kmem_cache_free(xfs_rmap_intent_cache,ri as *mut _);}
unsafe fn xfs_rmap_update_finish_item(tp:*mut xfs_trans,_done:*mut xfs_log_item,item:*mut list_head,state:*mut *mut xfs_btree_cur)->c_int{let e=xfs_rmap_finish_one(tp,ri_entry(item),state);xfs_rmap_update_cancel_item(item);e}
unsafe fn xfs_rmap_finish_one_cleanup(tp:*mut xfs_trans,rcur:*mut xfs_btree_cur,error:c_int){if rcur.is_null(){return;}let agbp=(*rcur).bc_ag.agbp;xfs_btree_del_cursor(rcur,error);if error!=0&&!agbp.is_null(){xfs_trans_brelse(tp,agbp);}}
unsafe fn xfs_rmap_update_abort_intent(intent:*mut xfs_log_item){xfs_rui_release(RUI_ITEM(intent));}

unsafe fn xfs_rui_validate_map(mp:*mut xfs_mount,isrt:bool,map:*mut xfs_map_extent)->bool { if !xfs_has_rmapbt(mp)||((*map).me_flags&!XFS_RMAP_EXTENT_FLAGS)!=0{return false;} if !matches!((*map).me_flags&XFS_RMAP_EXTENT_TYPE_MASK,XFS_RMAP_EXTENT_MAP|XFS_RMAP_EXTENT_MAP_SHARED|XFS_RMAP_EXTENT_UNMAP|XFS_RMAP_EXTENT_UNMAP_SHARED|XFS_RMAP_EXTENT_CONVERT|XFS_RMAP_EXTENT_CONVERT_SHARED|XFS_RMAP_EXTENT_ALLOC|XFS_RMAP_EXTENT_FREE){return false;} if !XFS_RMAP_NON_INODE_OWNER((*map).me_owner)&&!xfs_verify_ino(mp,(*map).me_owner){return false;} if !xfs_verify_fileext(mp,(*map).me_startoff,(*map).me_len){return false;} if isrt{xfs_verify_rtbext(mp,(*map).me_startblock,(*map).me_len)}else{xfs_verify_fsbext(mp,(*map).me_startblock,(*map).me_len)} }

unsafe fn xfs_rui_recover_work(mp:*mut xfs_mount,dfp:*mut xfs_defer_pending,isrt:bool,map:*const xfs_map_extent){let ri=kmem_cache_alloc(xfs_rmap_intent_cache,GFP_KERNEL|__GFP_NOFAIL) as *mut xfs_rmap_intent;(*ri).ri_type=match (*map).me_flags&XFS_RMAP_EXTENT_TYPE_MASK{XFS_RMAP_EXTENT_MAP=>XFS_RMAP_MAP,XFS_RMAP_EXTENT_MAP_SHARED=>XFS_RMAP_MAP_SHARED,XFS_RMAP_EXTENT_UNMAP=>XFS_RMAP_UNMAP,XFS_RMAP_EXTENT_UNMAP_SHARED=>XFS_RMAP_UNMAP_SHARED,XFS_RMAP_EXTENT_CONVERT=>XFS_RMAP_CONVERT,XFS_RMAP_EXTENT_CONVERT_SHARED=>XFS_RMAP_CONVERT_SHARED,XFS_RMAP_EXTENT_ALLOC=>XFS_RMAP_ALLOC,XFS_RMAP_EXTENT_FREE=>XFS_RMAP_FREE,_=>{ASSERT(false);return}};(*ri).ri_owner=(*map).me_owner;(*ri).ri_whichfork=if (*map).me_flags&XFS_RMAP_EXTENT_ATTR_FORK!=0{XFS_ATTR_FORK}else{XFS_DATA_FORK};(*ri).ri_bmap.br_startblock=(*map).me_startblock;(*ri).ri_bmap.br_startoff=(*map).me_startoff;(*ri).ri_bmap.br_blockcount=(*map).me_len;(*ri).ri_bmap.br_state=if (*map).me_flags&XFS_RMAP_EXTENT_UNWRITTEN!=0{XFS_EXT_UNWRITTEN}else{XFS_EXT_NORM};(*ri).ri_group=xfs_group_intent_get(mp,(*map).me_startblock,if isrt{XG_TYPE_RTG}else{XG_TYPE_AG});(*ri).ri_realtime=isrt;xfs_defer_add_item(dfp,&mut (*ri).ri_list);}

unsafe fn xfs_rmap_update_defer_recover(dfp:*mut xfs_defer_pending,capture:*mut list_head)->c_int{let lip=(*dfp).dfp_intent;let ruip=RUI_ITEM(lip);let mp=(*(*lip).li_log).l_mp;let isrt=xfs_rui_item_isrt(lip);for i in 0..(*ruip).rui_format.rui_nextents as usize{let map=&mut (*ruip).rui_format.rui_extents[i];if !xfs_rui_validate_map(mp,isrt,map){XFS_CORRUPTION_ERROR(c"xfs_rmap_recover_work",XFS_ERRLEVEL_LOW,mp,&mut (*ruip).rui_format as *mut _,core::mem::size_of_val(&(*ruip).rui_format));return -EFSCORRUPTED;}xfs_rui_recover_work(mp,dfp,isrt,map);}let resv=xlog_recover_resv(&(*M_RES(mp)).tr_itruncate);let mut tp=core::ptr::null_mut();let mut e=xfs_trans_alloc(mp,&resv,(*mp).m_rmap_maxlevels,0,XFS_TRANS_RESERVE,&mut tp);if e!=0{return e;}e=xlog_recover_finish_intent(tp,dfp);if e!=0{if e== -EFSCORRUPTED{XFS_CORRUPTION_ERROR(c"xfs_rmap_recover_work",XFS_ERRLEVEL_LOW,mp,&mut (*ruip).rui_format as *mut _,core::mem::size_of_val(&(*ruip).rui_format));}xfs_trans_cancel(tp);return e;}xfs_defer_ops_capture_and_commit(tp,capture)}

unsafe fn xfs_rmap_relog_intent(tp:*mut xfs_trans,intent:*mut xfs_log_item,_done:*mut xfs_log_item)->*mut xfs_log_item{let ruip0=RUI_ITEM(intent);ASSERT((*intent).li_type==XFS_LI_RUI||(*intent).li_type==XFS_LI_RUI_RT);let n=(*ruip0).rui_format.rui_nextents;let ruip=xfs_rui_init((*tp).t_mountp,(*intent).li_type,n);core::ptr::copy_nonoverlapping((*ruip0).rui_format.rui_extents.as_ptr(),(*ruip).rui_format.rui_extents.as_mut_ptr(),n as usize);atomic_set(&mut (*ruip).rui_next_extent,n as c_int);&mut (*ruip).rui_item}

static xfs_rui_item_ops_local:xfs_item_ops=xfs_item_ops{flags:XFS_ITEM_INTENT,iop_size:Some(xfs_rui_item_size),iop_format:Some(xfs_rui_item_format),iop_unpin:Some(xfs_rui_item_unpin),iop_release:Some(xfs_rui_item_release),iop_match:None};

// The remaining recovery entry points and defer-op tables retain the C ABI
// declarations and are supplied by the translated recovery layer.
pub static mut xfs_rmap_update_defer_type_local: xfs_defer_op_type = xfs_defer_op_type { name: c"rmap", max_items: XFS_RUI_MAX_FAST_EXTENTS, create_intent: Some(xfs_rmap_update_create_intent), abort_intent: Some(xfs_rmap_update_abort_intent), create_done: Some(xfs_rmap_update_create_done), finish_item: Some(xfs_rmap_update_finish_item), finish_cleanup: Some(xfs_rmap_finish_one_cleanup), cancel_item: Some(xfs_rmap_update_cancel_item), recover_work: Some(xfs_rmap_update_defer_recover), relog_intent: Some(xfs_rmap_relog_intent) };

// Recovery callbacks corresponding to the C pass-2 handlers.  Their body is
// expressed in terms of the external recovery interfaces used by this unit.
unsafe fn xlog_recover_rud_commit_pass2(log:*mut xlog, _buffers:*mut list_head, item:*mut xlog_recover_item, _lsn:xfs_lsn_t)->c_int { let p=(*item).ri_buf[0].iov_base as *mut xfs_rud_log_format; if (*item).ri_buf[0].iov_len!=core::mem::size_of::<xfs_rud_log_format>(){XFS_CORRUPTION_ERROR(c"xlog_recover_rud_commit_pass2",XFS_ERRLEVEL_LOW,(*log).l_mp,p as *mut _,(*item).ri_buf[0].iov_len);return -EFSCORRUPTED;}xlog_recover_release_intent(log,XFS_LI_RUI,(*p).rud_rui_id);0 }
unsafe fn xlog_recover_rtrud_commit_pass2(log:*mut xlog,b:*mut list_head,item:*mut xlog_recover_item,lsn:xfs_lsn_t)->c_int{xlog_recover_rud_commit_pass2(log,b,item,lsn)}

#[no_mangle] pub static xlog_rui_item_ops_local:xlog_recover_item_ops=xlog_recover_item_ops{item_type:XFS_LI_RUI,commit_pass2:Some(xlog_recover_rui_commit_pass2)};
#[no_mangle] pub static xlog_rtrui_item_ops_local:xlog_recover_item_ops=xlog_recover_item_ops{item_type:XFS_LI_RUI_RT,commit_pass2:Some(xlog_recover_rtrui_commit_pass2)};
#[no_mangle] pub static xlog_rud_item_ops_local:xlog_recover_item_ops=xlog_recover_item_ops{item_type:XFS_LI_RUD,commit_pass2:Some(xlog_recover_rud_commit_pass2)};
#[no_mangle] pub static xlog_rtrud_item_ops_local:xlog_recover_item_ops=xlog_recover_item_ops{item_type:XFS_LI_RUD_RT,commit_pass2:Some(xlog_recover_rtrud_commit_pass2)};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
