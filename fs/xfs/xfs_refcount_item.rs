// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2016 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <darrick.wong@oracle.com>
 */
// C dependencies supplied by the surrounding XFS translation.

static mut xfs_cui_cache: *mut kmem_cache = core::ptr::null_mut();
static mut xfs_cud_cache: *mut kmem_cache = core::ptr::null_mut();

static xfs_cui_item_ops: xfs_item_ops = xfs_item_ops {
    flags: XFS_ITEM_INTENT,
    iop_size: Some(xfs_cui_item_size), iop_format: Some(xfs_cui_item_format),
    iop_unpin: Some(xfs_cui_item_unpin), iop_release: Some(xfs_cui_item_release),
    iop_match: Some(xfs_cui_item_match),
};

#[inline]
unsafe fn CUI_ITEM(lip: *mut xfs_log_item) -> *mut xfs_cui_log_item {
    container_of!(lip, xfs_cui_log_item, cui_item)
}

unsafe fn xfs_cui_item_free(cuip: *mut xfs_cui_log_item) {
    kvfree((*cuip).cui_item.li_lv_shadow);
    if (*cuip).cui_format.cui_nextents > XFS_CUI_MAX_FAST_EXTENTS { kfree(cuip as *mut core::ffi::c_void); }
    else { kmem_cache_free(xfs_cui_cache, cuip as *mut core::ffi::c_void); }
}

/* Freeing the CUI requires that we remove it from the AIL if it has already
 * been placed there.  The reference count ensures only the last caller frees it. */
unsafe fn xfs_cui_release(cuip: *mut xfs_cui_log_item) {
    ASSERT!(atomic_read(&(*cuip).cui_refcount) > 0);
    if !atomic_dec_and_test(&mut (*cuip).cui_refcount) { return; }
    xfs_trans_ail_delete(&mut (*cuip).cui_item, 0);
    xfs_cui_item_free(cuip);
}

unsafe fn xfs_cui_item_size(lip: *mut xfs_log_item, nvecs: *mut i32, nbytes: *mut i32) {
    let cuip = CUI_ITEM(lip); *nvecs += 1;
    *nbytes += xfs_cui_log_format_sizeof((*cuip).cui_format.cui_nextents) as i32;
}

pub unsafe fn xfs_cui_log_space(nr: u32) -> u32 { xlog_item_space(1, xfs_cui_log_format_sizeof(nr)) }

unsafe fn xfs_cui_item_format(lip: *mut xfs_log_item, lfb: *mut xlog_format_buf) {
    let cuip = CUI_ITEM(lip);
    ASSERT!(atomic_read(&(*cuip).cui_next_extent) == (*cuip).cui_format.cui_nextents as i32);
    ASSERT!((*lip).li_type == XFS_LI_CUI || (*lip).li_type == XFS_LI_CUI_RT);
    (*cuip).cui_format.cui_type = (*lip).li_type; (*cuip).cui_format.cui_size = 1;
    xlog_format_copy(lfb, XLOG_REG_TYPE_CUI_FORMAT, &mut (*cuip).cui_format,
        xfs_cui_log_format_sizeof((*cuip).cui_format.cui_nextents));
}

unsafe fn xfs_cui_item_unpin(lip: *mut xfs_log_item, _remove: i32) { xfs_cui_release(CUI_ITEM(lip)); }
unsafe fn xfs_cui_item_release(lip: *mut xfs_log_item) { xfs_cui_release(CUI_ITEM(lip)); }

unsafe fn xfs_cui_init(mp: *mut xfs_mount, item_type: u16, nextents: u32) -> *mut xfs_cui_log_item {
    ASSERT!(nextents > 0); ASSERT!(item_type == XFS_LI_CUI || item_type == XFS_LI_CUI_RT);
    let cuip = if nextents > XFS_CUI_MAX_FAST_EXTENTS {
        kzalloc(xfs_cui_log_item_sizeof(nextents), GFP_KERNEL | __GFP_NOFAIL) as *mut xfs_cui_log_item
    } else { kmem_cache_zalloc(xfs_cui_cache, GFP_KERNEL | __GFP_NOFAIL) as *mut xfs_cui_log_item };
    xfs_log_item_init(mp, &mut (*cuip).cui_item, item_type, &xfs_cui_item_ops);
    (*cuip).cui_format.cui_nextents = nextents; (*cuip).cui_format.cui_id = cuip as usize as u64;
    atomic_set(&mut (*cuip).cui_next_extent, 0); atomic_set(&mut (*cuip).cui_refcount, 2); cuip
}

#[inline] unsafe fn CUD_ITEM(lip: *mut xfs_log_item) -> *mut xfs_cud_log_item { container_of!(lip, xfs_cud_log_item, cud_item) }
unsafe fn xfs_cud_item_size(_lip: *mut xfs_log_item, nvecs: *mut i32, nbytes: *mut i32) { *nvecs += 1; *nbytes += core::mem::size_of::<xfs_cud_log_format>() as i32; }
pub unsafe fn xfs_cud_log_space() -> u32 { xlog_item_space(1, core::mem::size_of::<xfs_cud_log_format>()) }
unsafe fn xfs_cud_item_format(lip: *mut xfs_log_item, lfb: *mut xlog_format_buf) { let cudp=CUD_ITEM(lip); ASSERT!((*lip).li_type==XFS_LI_CUD || (*lip).li_type==XFS_LI_CUD_RT); (*cudp).cud_format.cud_type=(*lip).li_type; (*cudp).cud_format.cud_size=1; xlog_format_copy(lfb,XLOG_REG_TYPE_CUD_FORMAT,&mut (*cudp).cud_format,core::mem::size_of::<xfs_cud_log_format>()); }
unsafe fn xfs_cud_item_release(lip: *mut xfs_log_item) { let cudp=CUD_ITEM(lip); xfs_cui_release((*cudp).cud_cuip); kvfree((*cudp).cud_item.li_lv_shadow); kmem_cache_free(xfs_cud_cache,cudp as *mut core::ffi::c_void); }
unsafe fn xfs_cud_item_intent(lip: *mut xfs_log_item) -> *mut xfs_log_item { &mut (*CUD_ITEM(lip)).cud_cuip.as_mut().unwrap().cui_item }
static xfs_cud_item_ops: xfs_item_ops = xfs_item_ops { flags:XFS_ITEM_RELEASE_WHEN_COMMITTED|XFS_ITEM_INTENT_DONE, iop_size:Some(xfs_cud_item_size), iop_format:Some(xfs_cud_item_format), iop_release:Some(xfs_cud_item_release), iop_intent:Some(xfs_cud_item_intent), ..xfs_item_ops::ZERO };

#[inline] unsafe fn ci_entry(e: *const list_head) -> *mut xfs_refcount_intent { list_entry!(e, xfs_refcount_intent, ri_list) }
#[inline] unsafe fn xfs_cui_item_isrt(lip:*const xfs_log_item)->bool { ASSERT!((*lip).li_type==XFS_LI_CUI||(*lip).li_type==XFS_LI_CUI_RT); (*lip).li_type==XFS_LI_CUI_RT }
unsafe fn xfs_refcount_update_diff_items(_priv_:*mut core::ffi::c_void,a:*const list_head,b:*const list_head)->i32 { cmp_int((*(*ci_entry(a)).ri_group).xg_gno,(*(*ci_entry(b)).ri_group).xg_gno) }

unsafe fn xfs_refcount_update_log_item(_tp:*mut xfs_trans,cuip:*mut xfs_cui_log_item,ri:*mut xfs_refcount_intent) {
    let next_extent=(atomic_inc_return(&mut (*cuip).cui_next_extent)-1) as usize; ASSERT!(next_extent < (*cuip).cui_format.cui_nextents as usize);
    let pmap=&mut (*cuip).cui_format.cui_extents[next_extent]; pmap.pe_startblock=(*ri).ri_startblock; pmap.pe_len=(*ri).ri_blockcount; pmap.pe_flags=0;
    match (*ri).ri_type { XFS_REFCOUNT_INCREASE|XFS_REFCOUNT_DECREASE|XFS_REFCOUNT_ALLOC_COW|XFS_REFCOUNT_FREE_COW => pmap.pe_flags|=(*ri).ri_type, _=>ASSERT!(false) }
}

unsafe fn __xfs_refcount_update_create_intent(tp:*mut xfs_trans,items:*mut list_head,count:u32,sort:bool,item_type:u16)->*mut xfs_log_item {
    ASSERT!(count>0); let cuip=xfs_cui_init((*tp).t_mountp,item_type,count); if sort { list_sort((*tp).t_mountp,items,Some(xfs_refcount_update_diff_items)); }
    let mut ri:*mut xfs_refcount_intent=core::ptr::null_mut(); list_for_each_entry!(ri,items,ri_list,{xfs_refcount_update_log_item(tp,cuip,ri);}); &mut (*cuip).cui_item
}
unsafe fn xfs_refcount_update_create_intent(tp:*mut xfs_trans,items:*mut list_head,count:u32,sort:bool)->*mut xfs_log_item { __xfs_refcount_update_create_intent(tp,items,count,sort,XFS_LI_CUI) }
#[inline] unsafe fn xfs_cud_type_from_cui(cuip:*const xfs_cui_log_item)->u16 { if xfs_cui_item_isrt(&(*cuip).cui_item) {XFS_LI_CUD_RT} else {XFS_LI_CUD} }
unsafe fn xfs_refcount_update_create_done(tp:*mut xfs_trans,intent:*mut xfs_log_item,_count:u32)->*mut xfs_log_item { let cuip=CUI_ITEM(intent); let cudp=kmem_cache_zalloc(xfs_cud_cache,GFP_KERNEL|__GFP_NOFAIL) as *mut xfs_cud_log_item; xfs_log_item_init((*tp).t_mountp,&mut (*cudp).cud_item,xfs_cud_type_from_cui(cuip),&xfs_cud_item_ops); (*cudp).cud_cuip=cuip; (*cudp).cud_format.cud_cui_id=(*cuip).cui_format.cui_id; &mut (*cudp).cud_item }

pub unsafe fn xfs_refcount_defer_add(tp:*mut xfs_trans,ri:*mut xfs_refcount_intent) { let mp=(*tp).t_mountp; (*ri).ri_group=xfs_group_intent_get(mp,(*ri).ri_startblock,if (*ri).ri_realtime {XG_TYPE_RTG}else{XG_TYPE_AG}); trace_xfs_refcount_defer(mp,ri); xfs_defer_add(tp,&mut (*ri).ri_list,if (*ri).ri_realtime {&xfs_rtrefcount_update_defer_type}else{&xfs_refcount_update_defer_type}); }
unsafe fn xfs_refcount_update_cancel_item(item:*mut list_head) { let ri=ci_entry(item); xfs_group_intent_put((*ri).ri_group); kmem_cache_free(xfs_refcount_intent_cache,ri as *mut core::ffi::c_void); }
unsafe fn xfs_refcount_update_finish_item(tp:*mut xfs_trans,_done:*mut xfs_log_item,item:*mut list_head,state:*mut *mut xfs_btree_cur)->i32 { let ri=ci_entry(item); let error=xfs_refcount_finish_one(tp,ri,state); if error==0 && (*ri).ri_blockcount>0 {ASSERT!((*ri).ri_type==XFS_REFCOUNT_INCREASE||(*ri).ri_type==XFS_REFCOUNT_DECREASE);return -EAGAIN;} xfs_refcount_update_cancel_item(item);error }
unsafe fn xfs_refcount_finish_one_cleanup(tp:*mut xfs_trans,rcur:*mut xfs_btree_cur,error:i32) { if rcur.is_null(){return;} let agbp=(*rcur).bc_ag.agbp; xfs_btree_del_cursor(rcur,error); if error!=0 && !agbp.is_null(){xfs_trans_brelse(tp,agbp);} }
unsafe fn xfs_refcount_update_abort_intent(intent:*mut xfs_log_item){xfs_cui_release(CUI_ITEM(intent));}

#[inline] unsafe fn xfs_cui_validate_phys(mp:*mut xfs_mount,isrt:bool,pmap:*mut xfs_phys_extent)->bool { if !xfs_has_reflink(mp)||(*pmap).pe_flags&!XFS_REFCOUNT_EXTENT_FLAGS!=0{return false;} match (*pmap).pe_flags&XFS_REFCOUNT_EXTENT_TYPE_MASK {XFS_REFCOUNT_INCREASE|XFS_REFCOUNT_DECREASE|XFS_REFCOUNT_ALLOC_COW|XFS_REFCOUNT_FREE_COW=>{},_=>return false} if isrt{xfs_verify_rtbext(mp,(*pmap).pe_startblock,(*pmap).pe_len)}else{xfs_verify_fsbext(mp,(*pmap).pe_startblock,(*pmap).pe_len)} }
unsafe fn xfs_cui_recover_work(mp:*mut xfs_mount,dfp:*mut xfs_defer_pending,isrt:bool,pmap:*mut xfs_phys_extent){let ri=kmem_cache_alloc(xfs_refcount_intent_cache,GFP_KERNEL|__GFP_NOFAIL) as *mut xfs_refcount_intent;(*ri).ri_type=(*pmap).pe_flags&XFS_REFCOUNT_EXTENT_TYPE_MASK;(*ri).ri_startblock=(*pmap).pe_startblock;(*ri).ri_blockcount=(*pmap).pe_len;(*ri).ri_group=xfs_group_intent_get(mp,(*pmap).pe_startblock,if isrt{XG_TYPE_RTG}else{XG_TYPE_AG});(*ri).ri_realtime=isrt;xfs_defer_add_item(dfp,&mut (*ri).ri_list);}

unsafe fn xfs_refcount_recover_work(dfp:*mut xfs_defer_pending,capture_list:*mut list_head)->i32 { let lip=(*dfp).dfp_intent; let cuip=CUI_ITEM(lip); let mp=(*(*lip).li_log).l_mp; let isrt=xfs_cui_item_isrt(lip); for i in 0..(*cuip).cui_format.cui_nextents as usize {let p=&mut (*cuip).cui_format.cui_extents[i];if !xfs_cui_validate_phys(mp,isrt,p){XFS_CORRUPTION_ERROR!(__func__,XFS_ERRLEVEL_LOW,mp,&(*cuip).cui_format,core::mem::size_of_val(&(*cuip).cui_format));return -EFSCORRUPTED;}xfs_cui_recover_work(mp,dfp,isrt,p);} let resv=xlog_recover_resv(&M_RES!(mp).tr_itruncate); let mut tp: *mut xfs_trans=core::ptr::null_mut(); let mut error=xfs_trans_alloc(mp,&resv,(*mp).m_refc_maxlevels*2,0,XFS_TRANS_RESERVE,&mut tp);if error!=0{return error;}error=xlog_recover_finish_intent(tp,dfp);if error==-EFSCORRUPTED{XFS_CORRUPTION_ERROR!(__func__,XFS_ERRLEVEL_LOW,mp,&(*cuip).cui_format,core::mem::size_of_val(&(*cuip).cui_format));}if error!=0{xfs_trans_cancel(tp);return error;}xfs_defer_ops_capture_and_commit(tp,capture_list)}

unsafe fn xfs_refcount_relog_intent(tp:*mut xfs_trans,intent:*mut xfs_log_item,_done:*mut xfs_log_item)->*mut xfs_log_item {ASSERT!((*intent).li_type==XFS_LI_CUI||(*intent).li_type==XFS_LI_CUI_RT);let old=CUI_ITEM(intent);let count=(*old).cui_format.cui_nextents;let cuip=xfs_cui_init((*tp).t_mountp,(*intent).li_type,count);core::ptr::copy_nonoverlapping((*old).cui_format.cui_extents.as_ptr(),(*cuip).cui_format.cui_extents.as_mut_ptr(),count as usize);atomic_set(&mut (*cuip).cui_next_extent,count as i32);&mut (*cuip).cui_item}

const xfs_refcount_update_defer_type:xfs_defer_op_type=xfs_defer_op_type{name:"refcount",max_items:XFS_CUI_MAX_FAST_EXTENTS,create_intent:Some(xfs_refcount_update_create_intent),abort_intent:Some(xfs_refcount_update_abort_intent),create_done:Some(xfs_refcount_update_create_done),finish_item:Some(xfs_refcount_update_finish_item),finish_cleanup:Some(xfs_refcount_finish_one_cleanup),cancel_item:Some(xfs_refcount_update_cancel_item),recover_work:Some(xfs_refcount_recover_work),relog_intent:Some(xfs_refcount_relog_intent),..xfs_defer_op_type::ZERO};

#[cfg(CONFIG_XFS_RT)] unsafe fn xfs_rtrefcount_update_create_intent(tp:*mut xfs_trans,items:*mut list_head,count:u32,sort:bool)->*mut xfs_log_item{__xfs_refcount_update_create_intent(tp,items,count,sort,XFS_LI_CUI_RT)}
#[cfg(CONFIG_XFS_RT)] unsafe fn xfs_rtrefcount_update_finish_item(tp:*mut xfs_trans,_done:*mut xfs_log_item,item:*mut list_head,state:*mut *mut xfs_btree_cur)->i32{let ri=ci_entry(item);let error=xfs_rtrefcount_finish_one(tp,ri,state);if error==0&&(*ri).ri_blockcount>0{ASSERT!((*ri).ri_type==XFS_REFCOUNT_INCREASE||(*ri).ri_type==XFS_REFCOUNT_DECREASE);return -EAGAIN;}xfs_refcount_update_cancel_item(item);error}
#[cfg(CONFIG_XFS_RT)] unsafe fn xfs_rtrefcount_finish_one_cleanup(_tp:*mut xfs_trans,rcur:*mut xfs_btree_cur,error:i32){if !rcur.is_null(){xfs_btree_del_cursor(rcur,error);}}
#[cfg(CONFIG_XFS_RT)] const xfs_rtrefcount_update_defer_type:xfs_defer_op_type=xfs_defer_op_type{name:"rtrefcount",max_items:XFS_CUI_MAX_FAST_EXTENTS,create_intent:Some(xfs_rtrefcount_update_create_intent),abort_intent:Some(xfs_refcount_update_abort_intent),create_done:Some(xfs_refcount_update_create_done),finish_item:Some(xfs_rtrefcount_update_finish_item),finish_cleanup:Some(xfs_rtrefcount_finish_one_cleanup),cancel_item:Some(xfs_refcount_update_cancel_item),recover_work:Some(xfs_refcount_recover_work),relog_intent:Some(xfs_refcount_relog_intent),..xfs_defer_op_type::ZERO};
#[cfg(not(CONFIG_XFS_RT))] const xfs_rtrefcount_update_defer_type:xfs_defer_op_type=xfs_defer_op_type{name:"rtrefcount",..xfs_defer_op_type::ZERO};

unsafe fn xfs_cui_item_match(lip:*mut xfs_log_item,intent_id:u64)->bool{(*CUI_ITEM(lip)).cui_format.cui_id==intent_id}
unsafe fn xfs_cui_copy_format(dst:*mut xfs_cui_log_format,src:*const xfs_cui_log_format){core::ptr::copy_nonoverlapping(src,dst,1);for i in 0..(*src).cui_nextents as usize{core::ptr::copy_nonoverlapping(&(*src).cui_extents[i],&mut (*dst).cui_extents[i],1);}}

unsafe fn xlog_recover_cui_commit_pass2(log:*mut xlog,_buffer_list:*mut list_head,item:*mut xlog_recover_item,lsn:xfs_lsn_t)->i32{let mp=(*log).l_mp;let fmt=(*item).ri_buf[0].iov_base as *mut xfs_cui_log_format;if (*item).ri_buf[0].iov_len<xfs_cui_log_format_sizeof(0){XFS_CORRUPTION_ERROR!(__func__,XFS_ERRLEVEL_LOW,mp,(*item).ri_buf[0].iov_base,(*item).ri_buf[0].iov_len);return -EFSCORRUPTED;}let len=xfs_cui_log_format_sizeof((*fmt).cui_nextents);if (*item).ri_buf[0].iov_len!=len{XFS_CORRUPTION_ERROR!(__func__,XFS_ERRLEVEL_LOW,mp,(*item).ri_buf[0].iov_base,(*item).ri_buf[0].iov_len);return -EFSCORRUPTED;}let cuip=xfs_cui_init(mp,ITEM_TYPE!(item),(*fmt).cui_nextents);xfs_cui_copy_format(&mut (*cuip).cui_format,fmt);atomic_set(&mut (*cuip).cui_next_extent,(*fmt).cui_nextents as i32);xlog_recover_intent_item(log,&mut (*cuip).cui_item,lsn,&xfs_refcount_update_defer_type);0}
const xlog_cui_item_ops:xlog_recover_item_ops=xlog_recover_item_ops{item_type:XFS_LI_CUI,commit_pass2:Some(xlog_recover_cui_commit_pass2)};

#[cfg(CONFIG_XFS_RT)] unsafe fn xlog_recover_rtcui_commit_pass2(log:*mut xlog,b:*mut list_head,item:*mut xlog_recover_item,lsn:xfs_lsn_t)->i32{xlog_recover_cui_commit_pass2(log,b,item,lsn)}
#[cfg(not(CONFIG_XFS_RT))] unsafe fn xlog_recover_rtcui_commit_pass2(log:*mut xlog,_b:*mut list_head,item:*mut xlog_recover_item,_lsn:xfs_lsn_t)->i32{XFS_CORRUPTION_ERROR!(__func__,XFS_ERRLEVEL_LOW,(*log).l_mp,(*item).ri_buf[0].iov_base,(*item).ri_buf[0].iov_len);-EFSCORRUPTED}
const xlog_rtcui_item_ops:xlog_recover_item_ops=xlog_recover_item_ops{item_type:XFS_LI_CUI_RT,commit_pass2:Some(xlog_recover_rtcui_commit_pass2)};

unsafe fn xlog_recover_cud_commit_pass2(log:*mut xlog,_b:*mut list_head,item:*mut xlog_recover_item,_lsn:xfs_lsn_t)->i32{let p=(*item).ri_buf[0].iov_base as *mut xfs_cud_log_format;if (*item).ri_buf[0].iov_len!=core::mem::size_of::<xfs_cud_log_format>(){XFS_CORRUPTION_ERROR!(__func__,XFS_ERRLEVEL_LOW,(*log).l_mp,(*item).ri_buf[0].iov_base,(*item).ri_buf[0].iov_len);return -EFSCORRUPTED;}xlog_recover_release_intent(log,XFS_LI_CUI,(*p).cud_cui_id);0}
const xlog_cud_item_ops:xlog_recover_item_ops=xlog_recover_item_ops{item_type:XFS_LI_CUD,commit_pass2:Some(xlog_recover_cud_commit_pass2)};
#[cfg(CONFIG_XFS_RT)] unsafe fn xlog_recover_rtcud_commit_pass2(log:*mut xlog,b:*mut list_head,item:*mut xlog_recover_item,lsn:xfs_lsn_t)->i32{xlog_recover_cud_commit_pass2(log,b,item,lsn)}
#[cfg(not(CONFIG_XFS_RT))] const xlog_recover_rtcud_commit_pass2:unsafe fn(*mut xlog,*mut list_head,*mut xlog_recover_item,xfs_lsn_t)->i32=xlog_recover_rtcui_commit_pass2;
const xlog_rtcud_item_ops:xlog_recover_item_ops=xlog_recover_item_ops{item_type:XFS_LI_CUD_RT,commit_pass2:Some(xlog_recover_rtcud_commit_pass2)};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
