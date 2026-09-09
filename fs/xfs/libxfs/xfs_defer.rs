// SPDX-License-Identifier: GPL-2.0+
/* Deferred operations in XFS.  External kernel/XFS declarations are supplied by
 * the surrounding translation unit. */

static mut xfs_defer_pending_cache: *mut kmem_cache = core::ptr::null_mut();

unsafe fn xfs_defer_barrier_create_intent(_tp: *mut xfs_trans, _items: *mut list_head, _count: u32, _sort: bool) -> *mut xfs_log_item { core::ptr::null_mut() }
unsafe fn xfs_defer_barrier_abort_intent(_intent: *mut xfs_log_item) {}
unsafe fn xfs_defer_barrier_create_done(_tp: *mut xfs_trans, _intent: *mut xfs_log_item, _count: u32) -> *mut xfs_log_item { core::ptr::null_mut() }
unsafe fn xfs_defer_barrier_finish_item(_tp: *mut xfs_trans, _done: *mut xfs_log_item, _item: *mut list_head, _state: *mut *mut xfs_btree_cur) -> i32 { ASSERT(0); -EFSCORRUPTED }
unsafe fn xfs_defer_barrier_cancel_item(_item: *mut list_head) { ASSERT(0); }

static xfs_barrier_defer_type: xfs_defer_op_type = xfs_defer_op_type {
    max_items: 1, create_intent: Some(xfs_defer_barrier_create_intent),
    abort_intent: Some(xfs_defer_barrier_abort_intent), create_done: Some(xfs_defer_barrier_create_done),
    finish_item: Some(xfs_defer_barrier_finish_item), cancel_item: Some(xfs_defer_barrier_cancel_item),
    relog_intent: None, recover_work: None, finish_cleanup: None,
};

unsafe fn xfs_defer_create_done(tp: *mut xfs_trans, dfp: *mut xfs_defer_pending) {
    if (*dfp).dfp_intent.is_null() { return; }
    (*tp).t_flags |= XFS_TRANS_DIRTY;
    let lip = ((*dfp).dfp_ops).create_done.unwrap()(tp, (*dfp).dfp_intent, (*dfp).dfp_count);
    if lip.is_null() { return; }
    (*tp).t_flags |= XFS_TRANS_HAS_INTENT_DONE;
    xfs_trans_add_item(tp, lip); set_bit(XFS_LI_DIRTY, &mut (*lip).li_flags);
    (*dfp).dfp_done = lip;
}

unsafe fn xfs_defer_create_intent(tp: *mut xfs_trans, dfp: *mut xfs_defer_pending, sort: bool) -> i32 {
    if !(*dfp).dfp_intent.is_null() { return 1; }
    let lip = ((*dfp).dfp_ops).create_intent.unwrap()(tp, &mut (*dfp).dfp_work, (*dfp).dfp_count, sort);
    if lip.is_null() { return 0; }
    if IS_ERR(lip) { return PTR_ERR(lip); }
    (*tp).t_flags |= XFS_TRANS_DIRTY; xfs_trans_add_item(tp, lip); set_bit(XFS_LI_DIRTY, &mut (*lip).li_flags);
    (*dfp).dfp_intent = lip; 1
}

unsafe fn xfs_defer_create_intents(tp: *mut xfs_trans) -> i32 {
    let mut ret = 0; let mut dfp = list_first_entry((*tp).t_dfops, xfs_defer_pending, dfp_list);
    while !dfp.is_null() { trace_xfs_defer_create_intent((*tp).t_mountp, dfp); let r = xfs_defer_create_intent(tp, dfp, true); if r < 0 { return r; } ret |= r; dfp = list_next_entry(dfp, dfp_list); } ret
}

unsafe fn xfs_defer_pending_abort(mp: *mut xfs_mount, dfp: *mut xfs_defer_pending) { trace_xfs_defer_pending_abort(mp, dfp); if !(*dfp).dfp_intent.is_null() && (*dfp).dfp_done.is_null() { ((*dfp).dfp_ops).abort_intent.unwrap()((*dfp).dfp_intent); (*dfp).dfp_intent = core::ptr::null_mut(); } }
unsafe fn xfs_defer_pending_cancel_work(mp: *mut xfs_mount, dfp: *mut xfs_defer_pending) { let mut p=(*dfp).dfp_work.next; while p != &mut (*dfp).dfp_work { let n=(*p).next; list_del(p); (*dfp).dfp_count-=1; trace_xfs_defer_cancel_item(mp,dfp,p); ((*dfp).dfp_ops).cancel_item.unwrap()(p); p=n; } ASSERT((*dfp).dfp_count==0); kmem_cache_free(xfs_defer_pending_cache,dfp as *mut _); }
unsafe fn xfs_defer_pending_abort_list(mp:*mut xfs_mount, l:*mut list_head){ let mut p=list_first_entry(*l,xfs_defer_pending,dfp_list); while !p.is_null(){xfs_defer_pending_abort(mp,p);p=list_next_entry(p,dfp_list);} }
unsafe fn xfs_defer_trans_abort(tp:*mut xfs_trans,l:*mut list_head){trace_xfs_defer_trans_abort(tp,_RET_IP_);xfs_defer_pending_abort_list((*tp).t_mountp,l);}

unsafe fn xfs_defer_trans_roll(tpp:*mut *mut xfs_trans)->i32 { let mut d=xfs_defer_resources::default(); let e=xfs_defer_save_resources(&mut d,*tpp); if e!=0{return e;} trace_xfs_defer_trans_roll(*tpp,_RET_IP_); let e=xfs_trans_roll(tpp); xfs_defer_restore_resources(*tpp,&mut d); if e!=0{trace_xfs_defer_trans_roll_error(*tpp,e);} e }
unsafe fn xfs_defer_save_resources(_d:*mut xfs_defer_resources,_tp:*mut xfs_trans)->i32 { 0 }
unsafe fn xfs_defer_restore_resources(_tp:*mut xfs_trans,_d:*mut xfs_defer_resources) {}
unsafe fn xfs_defer_cancel_list(mp:*mut xfs_mount,l:*mut list_head){let mut p=list_first_entry(*l,xfs_defer_pending,dfp_list);while !p.is_null(){let n=list_next_entry(p,dfp_list);xfs_defer_pending_cancel_work(mp,p);p=n;}}
unsafe fn xfs_defer_relog_intent(tp:*mut xfs_trans,dfp:*mut xfs_defer_pending){xfs_defer_create_done(tp,dfp);let lip=((*dfp).dfp_ops).relog_intent.unwrap()(tp,(*dfp).dfp_intent,(*dfp).dfp_done);if !lip.is_null(){xfs_trans_add_item(tp,lip);set_bit(XFS_LI_DIRTY,&mut(*lip).li_flags);}(*dfp).dfp_done=core::ptr::null_mut();(*dfp).dfp_intent=lip;}
unsafe fn xfs_defer_relog(_tpp:*mut *mut xfs_trans,_dfops:*mut list_head) {}

pub unsafe fn xfs_defer_finish_one(tp:*mut xfs_trans,dfp:*mut xfs_defer_pending)->i32{let ops=(*dfp).dfp_ops;let mut state=core::ptr::null_mut();let mut li=(*dfp).dfp_work.next;while li!=&mut(*dfp).dfp_work{let n=(*li).next;list_del(li);(*dfp).dfp_count-=1;trace_xfs_defer_finish_item((*tp).t_mountp,dfp,li);let mut e=ops.finish_item.unwrap()(tp,(*dfp).dfp_done,li,&mut state);if e==-EAGAIN{list_add(li,&mut(*dfp).dfp_work);(*dfp).dfp_count+=1;(*dfp).dfp_done=core::ptr::null_mut();(*dfp).dfp_intent=core::ptr::null_mut();let r=xfs_defer_create_intent(tp,dfp,false);if r<0{e=r;}}if e!=0{if let Some(f)=ops.finish_cleanup{f(tp,state,e);}return e;}li=n;}list_del(&mut(*dfp).dfp_list);kmem_cache_free(xfs_defer_pending_cache,dfp as*mut _);if let Some(f)=ops.finish_cleanup{f(tp,state,0);}0}

pub unsafe fn xfs_defer_finish_noroll(tp:*mut *mut xfs_trans)->i32{let mut pending=list_head::new();let mut paused=list_head::new();let mut dfp=core::ptr::null_mut();loop{if list_empty(&pending)&&list_empty(&mut(**tp).t_dfops){break;}let h=xfs_defer_create_intents(*tp);if h<0{return h;}list_splice_init(&mut(**tp).t_dfops,&mut paused);list_splice_init(&mut(**tp).t_dfops,&mut pending);if h!=0||!dfp.is_null(){let e=xfs_defer_trans_roll(tp);if e!=0{return e;}}dfp=list_first_entry_or_null(&pending,xfs_defer_pending,dfp_list);if dfp.is_null(){break;}let e=xfs_defer_finish_one(*tp,dfp);if e!=0&&e!=-EAGAIN{return e;}}list_splice_tail_init(&mut paused,&mut(**tp).t_dfops);0}
pub unsafe fn xfs_defer_finish(tp:*mut *mut xfs_trans)->i32{let e=xfs_defer_finish_noroll(tp);if e!=0{return e;}if (**tp).t_flags&XFS_TRANS_DIRTY!=0{let e=xfs_defer_trans_roll(tp);if e!=0{return e;}}(**tp).t_flags&=!XFS_TRANS_LOWMODE;0}
pub unsafe fn xfs_defer_cancel(tp:*mut xfs_trans){xfs_defer_trans_abort(tp,&mut(*tp).t_dfops);xfs_defer_cancel_list((*tp).t_mountp,&mut(*tp).t_dfops);}

unsafe fn xfs_defer_find_last(tp:*mut xfs_trans,ops:*const xfs_defer_op_type)->*mut xfs_defer_pending{if list_empty(&(*tp).t_dfops){return core::ptr::null_mut();}let p=list_last_entry(&(*tp).t_dfops,xfs_defer_pending,dfp_list);if (*p).dfp_ops!=ops{core::ptr::null_mut()}else{p}}
unsafe fn xfs_defer_can_append(d:*mut xfs_defer_pending,o:*const xfs_defer_op_type)->bool{(*d).dfp_intent.is_null()&&(*d).dfp_flags&XFS_DEFER_PAUSED==0&&((*o).max_items==0||(*d).dfp_count<(*o).max_items)}
unsafe fn xfs_defer_alloc(l:*mut list_head,o:*const xfs_defer_op_type)->*mut xfs_defer_pending{let d=kmem_cache_zalloc(xfs_defer_pending_cache,GFP_KERNEL|__GFP_NOFAIL) as*mut xfs_defer_pending;(*d).dfp_ops=o;INIT_LIST_HEAD(&mut(*d).dfp_work);list_add_tail(&mut(*d).dfp_list,l);d}
pub unsafe fn xfs_defer_add(tp:*mut xfs_trans,li:*mut list_head,o:*const xfs_defer_op_type)->*mut xfs_defer_pending{let mut d=xfs_defer_find_last(tp,o);if d.is_null()||!xfs_defer_can_append(d,o){d=xfs_defer_alloc(&mut(*tp).t_dfops,o);}xfs_defer_add_item(d,li);trace_xfs_defer_add_item((*tp).t_mountp,d,li);d}
pub unsafe fn xfs_defer_add_barrier(tp:*mut xfs_trans){if !xfs_defer_find_last(tp,&xfs_barrier_defer_type).is_null(){return;}let d=xfs_defer_alloc(&mut(*tp).t_dfops,&xfs_barrier_defer_type);trace_xfs_defer_add_item((*tp).t_mountp,d,core::ptr::null_mut());}
pub unsafe fn xfs_defer_start_recovery(lip:*mut xfs_log_item,l:*mut list_head,o:*const xfs_defer_op_type){let d=xfs_defer_alloc(l,o);(*d).dfp_intent=lip;}
pub unsafe fn xfs_defer_cancel_recovery(mp:*mut xfs_mount,d:*mut xfs_defer_pending){xfs_defer_pending_abort(mp,d);xfs_defer_pending_cancel_work(mp,d);}
pub unsafe fn xfs_defer_finish_recovery(mp:*mut xfs_mount,d:*mut xfs_defer_pending,c:*mut list_head)->i32{let e=((*d).dfp_ops).recover_work.unwrap()(d,c);if e!=0{trace_xlog_intent_recovery_failed(mp,(*d).dfp_ops,e);}e}
pub unsafe fn xfs_defer_move(d:*mut xfs_trans,s:*mut xfs_trans){list_splice_init(&mut(*s).t_dfops,&mut(*d).t_dfops);(*d).t_flags|=(*s).t_flags&XFS_TRANS_LOWMODE;(*s).t_flags&=!XFS_TRANS_LOWMODE;}

pub unsafe fn xfs_defer_ops_capture_abort(mp:*mut xfs_mount,dfc:*mut xfs_defer_capture){xfs_defer_pending_abort_list(mp,&mut(*dfc).dfc_dfops);xfs_defer_cancel_list(mp,&mut(*dfc).dfc_dfops);for i in 0..(*dfc).dfc_held.dr_bufs{xfs_buf_relse((*dfc).dfc_held.dr_bp[i as usize]);}for i in 0..(*dfc).dfc_held.dr_inos{xfs_irele((*dfc).dfc_held.dr_ip[i as usize]);}kfree(dfc as*mut _);}
pub unsafe fn xfs_defer_ops_capture_and_commit(tp:*mut xfs_trans,c:*mut list_head)->i32{let d=xfs_defer_ops_capture(tp);if IS_ERR(d){xfs_trans_cancel(tp);return PTR_ERR(d);}if d.is_null(){return xfs_trans_commit(tp);}let e=xfs_trans_commit(tp);if e!=0{xfs_defer_ops_capture_abort((*tp).t_mountp,d);return e;}list_add_tail(&mut(*d).dfc_list,c);0}
pub unsafe fn xfs_defer_ops_continue(d:*mut xfs_defer_capture,tp:*mut xfs_trans,r:*mut xfs_defer_resources){xfs_defer_restore_resources(tp,&mut(*d).dfc_held);*r=(*d).dfc_held;(*r).dr_bufs=0;list_splice_init(&mut(*d).dfc_dfops,&mut(*tp).t_dfops);(*tp).t_flags|=(*d).dfc_tpflags;kfree(d as*mut _);}
pub unsafe fn xfs_defer_resources_rele(r:*mut xfs_defer_resources){for i in 0..(*r).dr_inos{xfs_iunlock((*r).dr_ip[i as usize],XFS_ILOCK_EXCL);xfs_irele((*r).dr_ip[i as usize]);}for i in 0..(*r).dr_bufs{xfs_buf_relse((*r).dr_bp[i as usize]);}(*r).dr_inos=0;(*r).dr_bufs=0;(*r).dr_ordered=0;}
unsafe fn xfs_defer_init_cache()->i32{xfs_defer_pending_cache=kmem_cache_create("xfs_defer_pending",core::mem::size_of::<xfs_defer_pending>(),0,0,core::ptr::null_mut());if xfs_defer_pending_cache.is_null(){-ENOMEM}else{0}}
unsafe fn xfs_defer_destroy_cache(){kmem_cache_destroy(xfs_defer_pending_cache);xfs_defer_pending_cache=core::ptr::null_mut();}
pub unsafe fn xfs_defer_init_item_caches()->i32{xfs_defer_init_cache()}
pub unsafe fn xfs_defer_destroy_item_caches(){xfs_exchmaps_intent_destroy_cache();xfs_attr_intent_destroy_cache();xfs_extfree_intent_destroy_cache();xfs_bmap_intent_destroy_cache();xfs_refcount_intent_destroy_cache();xfs_rmap_intent_destroy_cache();xfs_defer_destroy_cache();}
pub unsafe fn xfs_defer_item_pause(tp:*mut xfs_trans,d:*mut xfs_defer_pending){ASSERT((*d).dfp_flags&XFS_DEFER_PAUSED==0);(*d).dfp_flags|=XFS_DEFER_PAUSED;trace_xfs_defer_item_pause((*tp).t_mountp,d);}
pub unsafe fn xfs_defer_item_unpause(tp:*mut xfs_trans,d:*mut xfs_defer_pending){ASSERT((*d).dfp_flags&XFS_DEFER_PAUSED!=0);(*d).dfp_flags&=!XFS_DEFER_PAUSED;trace_xfs_defer_item_unpause((*tp).t_mountp,d);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
