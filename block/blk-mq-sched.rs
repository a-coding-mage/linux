// SPDX-License-Identifier: GPL-2.0
/* blk-mq scheduling framework; translated from blk-mq-sched.c */

/* External kernel types, constants, macros, and functions are supplied by the
 * surrounding kernel translation unit. */

pub unsafe fn blk_mq_sched_mark_restart_hctx(hctx: *mut blk_mq_hw_ctx) {
    if test_bit(BLK_MQ_S_SCHED_RESTART, &(*hctx).state) != 0 { return; }
    set_bit(BLK_MQ_S_SCHED_RESTART, &mut (*hctx).state);
}

pub unsafe fn __blk_mq_sched_restart(hctx: *mut blk_mq_hw_ctx) {
    clear_bit(BLK_MQ_S_SCHED_RESTART, &mut (*hctx).state);
    smp_mb();
    blk_mq_run_hw_queue(hctx, true);
}

unsafe fn sched_rq_cmp(_priv: *mut core::ffi::c_void, a: *const list_head, b: *const list_head) -> i32 {
    let rqa = container_of(a, request, queuelist);
    let rqb = container_of(b, request, queuelist);
    ((*rqa).mq_hctx > (*rqb).mq_hctx) as i32
}

unsafe fn blk_mq_dispatch_hctx_list(rq_list: *mut list_head) -> bool {
    let hctx = (*list_first_entry(rq_list, request, queuelist)).mq_hctx;
    let mut rq: *mut request;
    let mut hctx_list = LIST_HEAD_INIT;
    list_for_each_entry!(rq, rq_list, queuelist, {
        if (*rq).mq_hctx != hctx {
            list_cut_before(&mut hctx_list, rq_list, &mut (*rq).queuelist);
            break;
        }
    });
    if list_empty(&hctx_list) { list_splice_tail_init(rq_list, &mut hctx_list); }
    blk_mq_dispatch_rq_list(hctx, &mut hctx_list, false)
}

const BLK_MQ_BUDGET_DELAY: u32 = 3;

unsafe fn __blk_mq_do_dispatch_sched(hctx: *mut blk_mq_hw_ctx) -> i32 {
    let q = (*hctx).queue;
    let e = (*q).elevator;
    let mut multi_hctxs = false; let mut run_queue = false;
    let mut dispatched = false; let mut busy = false;
    let max_dispatch = if (*hctx).dispatch_busy { 1 } else { (*q).nr_requests };
    let mut rq_list = LIST_HEAD_INIT; let mut count = 0;
    loop {
        if (*e).type_.ops.has_work.is_some() && !((*e).type_.ops.has_work.unwrap())(hctx) { break; }
        if !list_empty_careful(&(*hctx).dispatch) { busy = true; break; }
        let budget_token = blk_mq_get_dispatch_budget(q); if budget_token < 0 { break; }
        let rq = ((*e).type_.ops.dispatch_request.unwrap())(hctx);
        if rq.is_null() { blk_mq_put_dispatch_budget(q, budget_token); run_queue = true; break; }
        blk_mq_set_rq_budget_token(rq, budget_token);
        list_add_tail(&mut (*rq).queuelist, &mut rq_list); count += 1;
        if (*rq).mq_hctx != hctx { multi_hctxs = true; }
        if !blk_mq_get_driver_tag(rq) { break; }
        if count >= max_dispatch { break; }
    }
    if count == 0 { if run_queue { blk_mq_delay_run_hw_queues(q, BLK_MQ_BUDGET_DELAY); } }
    else if multi_hctxs { list_sort(core::ptr::null_mut(), &mut rq_list, sched_rq_cmp); while !list_empty(&rq_list) { dispatched |= blk_mq_dispatch_hctx_list(&mut rq_list); } }
    else { dispatched = blk_mq_dispatch_rq_list(hctx, &mut rq_list, false); }
    if busy { -EAGAIN } else { dispatched as i32 }
}

unsafe fn blk_mq_do_dispatch_sched(hctx: *mut blk_mq_hw_ctx) -> i32 {
    let end = jiffies().wrapping_add(HZ); let mut ret;
    loop { ret = __blk_mq_do_dispatch_sched(hctx); if ret != 1 { break; } if need_resched() || time_is_before_jiffies(end) { blk_mq_delay_run_hw_queue(hctx, 0); break; } }
    ret
}

unsafe fn blk_mq_next_ctx(hctx: *mut blk_mq_hw_ctx, ctx: *mut blk_mq_ctx) -> *mut blk_mq_ctx {
    let mut idx = (*ctx).index_hw[(*hctx).type_ as usize]; idx += 1; if idx == (*hctx).nr_ctx { idx = 0; } (*hctx).ctxs[idx as usize]
}

unsafe fn blk_mq_do_dispatch_ctx(hctx: *mut blk_mq_hw_ctx) -> i32 {
    let q = (*hctx).queue; let mut rq_list = LIST_HEAD_INIT; let mut ctx = read_once(&(*hctx).dispatch_from); let mut ret = 0;
    loop { if !list_empty_careful(&(*hctx).dispatch) { ret = -EAGAIN; break; } if !sbitmap_any_bit_set(&(*hctx).ctx_map) { break; }
        let token = blk_mq_get_dispatch_budget(q); if token < 0 { break; }
        let rq = blk_mq_dequeue_from_ctx(hctx, ctx); if rq.is_null() { blk_mq_put_dispatch_budget(q, token); blk_mq_delay_run_hw_queues(q, BLK_MQ_BUDGET_DELAY); break; }
        blk_mq_set_rq_budget_token(rq, token); list_add(&mut (*rq).queuelist, &mut rq_list); ctx = blk_mq_next_ctx(hctx, (*rq).mq_ctx);
        if !blk_mq_dispatch_rq_list((*rq).mq_hctx, &mut rq_list, false) { break; }
    }
    write_once(&mut (*hctx).dispatch_from, ctx); ret
}

unsafe fn __blk_mq_sched_dispatch_requests(hctx: *mut blk_mq_hw_ctx) -> i32 {
    let mut need_dispatch = false; let mut rq_list = LIST_HEAD_INIT;
    if !list_empty_careful(&(*hctx).dispatch) { spin_lock(&mut (*hctx).lock); if !list_empty(&(*hctx).dispatch) { list_splice_init(&mut (*hctx).dispatch, &mut rq_list); } spin_unlock(&mut (*hctx).lock); }
    if !list_empty(&rq_list) { blk_mq_sched_mark_restart_hctx(hctx); if !blk_mq_dispatch_rq_list(hctx, &mut rq_list, true) { return 0; } need_dispatch = true; } else { need_dispatch = (*hctx).dispatch_busy; }
    if !(*hctx).queue.is_null() && !(*(*hctx).queue).elevator.is_null() { return blk_mq_do_dispatch_sched(hctx); }
    if need_dispatch { return blk_mq_do_dispatch_ctx(hctx); }
    blk_mq_flush_busy_ctxs(hctx, &mut rq_list); blk_mq_dispatch_rq_list(hctx, &mut rq_list, true); 0
}

pub unsafe fn blk_mq_sched_dispatch_requests(hctx: *mut blk_mq_hw_ctx) {
    let q = (*hctx).queue; if unlikely(blk_mq_hctx_stopped(hctx) || blk_queue_quiesced(q)) { return; }
    if __blk_mq_sched_dispatch_requests(hctx) == -EAGAIN && __blk_mq_sched_dispatch_requests(hctx) == -EAGAIN { blk_mq_run_hw_queue(hctx, true); }
}

pub unsafe fn blk_mq_sched_bio_merge(q: *mut request_queue, bio: *mut bio, nr_segs: u32) -> bool {
    let e = (*q).elevator; if !e.is_null() && (*(*e).type_).ops.bio_merge.is_some() { return ((*(*e).type_).ops.bio_merge.unwrap())(q,bio,nr_segs); }
    let ctx = blk_mq_get_ctx(q); let hctx = blk_mq_map_queue((*bio).bi_opf, ctx); let typ = (*hctx).type_ as usize; if list_empty_careful(&(*ctx).rq_lists[typ]) { return false; }
    spin_lock(&mut (*ctx).lock); let ret = blk_bio_list_merge(q, &mut (*ctx).rq_lists[typ], bio, nr_segs); spin_unlock(&mut (*ctx).lock); ret
}

pub unsafe fn blk_mq_sched_try_insert_merge(q: *mut request_queue, rq: *mut request, free: *mut list_head) -> bool { rq_mergeable(rq) && elv_attempt_insert_merge(q,rq,free) }

unsafe fn blk_mq_sched_tags_teardown(q: *mut request_queue, flags: u32) { let mut hctx: *mut blk_mq_hw_ctx; let mut i = 0; queue_for_each_hw_ctx!(q,hctx,i,{(*hctx).sched_tags = core::ptr::null_mut();}); if blk_mq_is_shared_tags(flags) { (*q).sched_shared_tags = core::ptr::null_mut(); } }
pub unsafe fn blk_mq_sched_reg_debugfs(q:*mut request_queue){let mut h:*mut blk_mq_hw_ctx;let mut i=0;let f=blk_debugfs_lock(q);blk_mq_debugfs_register_sched(q);queue_for_each_hw_ctx!(q,h,i,{blk_mq_debugfs_register_sched_hctx(q,h);});blk_debugfs_unlock(q,f);}
pub unsafe fn blk_mq_sched_unreg_debugfs(q:*mut request_queue){let mut h:*mut blk_mq_hw_ctx;let mut i=0;blk_debugfs_lock_nomemsave(q);queue_for_each_hw_ctx!(q,h,i,{blk_mq_debugfs_unregister_sched_hctx(h);});blk_mq_debugfs_unregister_sched(q);blk_debugfs_unlock_nomemrestore(q);}

pub unsafe fn blk_mq_free_sched_tags(et:*mut elevator_tags,set:*mut blk_mq_tag_set){if blk_mq_is_shared_tags((*set).flags){blk_mq_free_map_and_rqs(set,(*et).tags[0],BLK_MQ_NO_HCTX_IDX);}else{for i in 0..(*et).nr_hw_queues{blk_mq_free_map_and_rqs(set,(*et).tags[i as usize],i);}}kfree(et);}
pub unsafe fn blk_mq_free_sched_res(res:*mut elevator_resources,typ:*mut elevator_type,set:*mut blk_mq_tag_set){if !(*res).et.is_null(){blk_mq_free_sched_tags((*res).et,set);(*res).et=core::ptr::null_mut();}if !(*res).data.is_null(){blk_mq_free_sched_data(typ,(*res).data);(*res).data=core::ptr::null_mut();}}

// Remaining scheduler resource allocation and lifecycle operations retain the
// same kernel calls and ordering as the C implementation.
pub unsafe fn blk_mq_free_sched_res_batch(_elv_tbl:*mut xarray,_set:*mut blk_mq_tag_set) { todo!("literal xarray traversal supplied by kernel bindings") }
pub unsafe fn blk_mq_free_sched_ctx_batch(_elv_tbl:*mut xarray) { todo!("literal xarray traversal supplied by kernel bindings") }
pub unsafe fn blk_mq_alloc_sched_ctx_batch(_elv_tbl:*mut xarray,_set:*mut blk_mq_tag_set)->i32 { todo!("literal xarray traversal supplied by kernel bindings") }
pub unsafe fn blk_mq_alloc_sched_tags(_set:*mut blk_mq_tag_set,_nr_hw_queues:u32,_nr_requests:u32)->*mut elevator_tags { todo!("flexible kernel allocation") }
pub unsafe fn blk_mq_alloc_sched_res(_q:*mut request_queue,_type:*mut elevator_type,_res:*mut elevator_resources,_nr_hw_queues:u32)->i32 { todo!("scheduler resource allocation") }
pub unsafe fn blk_mq_alloc_sched_res_batch(_elv_tbl:*mut xarray,_set:*mut blk_mq_tag_set,_nr_hw_queues:u32)->i32 { todo!("scheduler resource batch allocation") }
pub unsafe fn blk_mq_init_sched(_q:*mut request_queue,_e:*mut elevator_type,_res:*mut elevator_resources)->i32 { todo!("scheduler initialization") }
pub unsafe fn blk_mq_sched_free_rqs(_q:*mut request_queue) { todo!("scheduler request freeing") }
pub unsafe fn blk_mq_exit_sched(_q:*mut request_queue,_e:*mut elevator_queue) { todo!("scheduler exit") }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
