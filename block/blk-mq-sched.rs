// SPDX-License-Identifier: GPL-2.0
//! blk-mq scheduling framework, translated from the retained blk-mq-sched.c.
//!
//! All kernel structures come from this configuration's authoritative headers.
//! C boundaries below are existing kernel services or macro/inline adapters;
//! dispatch, ownership, traversal, initialization and unwind logic stay here.
#![allow(missing_docs, unsafe_op_in_unsafe_fn)]

#[allow(clippy::all, dead_code, non_camel_case_types, non_snake_case,
         non_upper_case_globals, improper_ctypes, unreachable_pub)]
mod bindings {
    use core::ffi;
    type __kernel_size_t = usize;
    type __kernel_ssize_t = isize;
    type __kernel_ptrdiff_t = isize;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/blk_mq_sched_generated.rs"));
}
use bindings::*;
use core::{ffi::{c_int, c_uint, c_ulong, c_void}, mem::{offset_of, MaybeUninit}, ptr::{addr_of, addr_of_mut, null_mut}};

const BLK_MQ_BUDGET_DELAY: c_ulong = 3;

unsafe fn empty(head: *const list_head) -> bool {
    (*head).next == head.cast_mut()
}

unsafe fn rq_list(rq: *mut request) -> *mut list_head {
    addr_of_mut!((*rq).__bindgen_anon_1.queuelist)
}

unsafe fn list_rq(entry: *const list_head) -> *mut request {
    entry.cast::<u8>().sub(offset_of!(request, __bindgen_anon_1)).cast_mut().cast()
}

unsafe fn list_queue(entry: *mut list_head) -> *mut request_queue {
    entry.cast::<u8>().sub(offset_of!(request_queue, tag_set_list)).cast()
}

fn shared_tags(flags: c_uint) -> bool {
    flags & BLK_MQ_F_TAG_HCTX_SHARED != 0
}

unsafe fn tag_at(et: *mut elevator_tags, index: usize) -> *mut blk_mq_tags {
    *(*et).tags.as_ptr().add(index)
}

unsafe fn set_tag(et: *mut elevator_tags, index: usize, tags: *mut blk_mq_tags) {
    *(*et).tags.as_mut_ptr().add(index) = tags;
}

// xa_for_each's first/next operations preserve sparse indices and XA_PRESENT.
unsafe fn xa_first(table: *mut xarray, index: *mut c_ulong) -> *mut c_void {
    xa_find(table, index, c_ulong::MAX, LUPOS_SCHED_XA_PRESENT)
}
unsafe fn xa_next(table: *mut xarray, index: *mut c_ulong) -> *mut c_void {
    xa_find_after(table, index, c_ulong::MAX, LUPOS_SCHED_XA_PRESENT)
}

macro_rules! for_each_hctx {
    ($q:expr, $hctx:ident, $index:ident, $body:block) => {{
        let mut $index: c_ulong = 0;
        while $index < (*$q).nr_hw_queues as c_ulong {
            let $hctx = lupos_sched_queue_hctx($q, $index as c_int);
            $body
            $index += 1;
        }
    }};
}

#[no_mangle]
pub unsafe extern "C" fn blk_mq_sched_mark_restart_hctx(hctx: *mut blk_mq_hw_ctx) {
    let state = addr_of_mut!((*hctx).__bindgen_anon_1.state);
    if lupos_sched_test_bit(BLK_MQ_S_SCHED_RESTART, state) {
        return;
    }
    lupos_sched_set_bit(BLK_MQ_S_SCHED_RESTART, state);
}

#[no_mangle]
pub unsafe extern "C" fn __blk_mq_sched_restart(hctx: *mut blk_mq_hw_ctx) {
    lupos_sched_clear_bit(BLK_MQ_S_SCHED_RESTART, addr_of_mut!((*hctx).__bindgen_anon_1.state));
    // Pairs with blk_mq_dispatch_rq_list: publish the cleared restart flag
    // before blk_mq_run_hw_queue checks hctx->dispatch.
    lupos_sched_mb();
    blk_mq_run_hw_queue(hctx, true);
}

unsafe extern "C" fn sched_rq_cmp(_priv: *mut c_void, a: *const list_head, b: *const list_head) -> c_int {
    ((*list_rq(a)).mq_hctx > (*list_rq(b)).mq_hctx) as c_int
}

unsafe fn blk_mq_dispatch_hctx_list(list: *mut list_head) -> bool {
    let hctx = (*list_rq((*list).next)).mq_hctx;
    let mut storage = MaybeUninit::<list_head>::uninit();
    let hctx_list = storage.as_mut_ptr();
    lupos_sched_init_list(hctx_list);
    let mut entry = (*list).next;
    while entry != list {
        if (*list_rq(entry)).mq_hctx != hctx {
            lupos_sched_list_cut_before(hctx_list, list, entry);
            return blk_mq_dispatch_rq_list(hctx, hctx_list, false);
        }
        entry = (*entry).next;
    }
    lupos_sched_list_splice_tail_init(list, hctx_list);
    blk_mq_dispatch_rq_list(hctx, hctx_list, false)
}

unsafe fn __blk_mq_do_dispatch_sched(hctx: *mut blk_mq_hw_ctx) -> c_int {
    let q = (*hctx).queue;
    let e = (*q).elevator;
    let mut multi_hctxs = false;
    let mut run_queue = false;
    let mut dispatched = false;
    let mut busy = false;
    let max_dispatch = if (*hctx).dispatch_busy != 0 { 1 } else { (*q).nr_requests };
    let mut storage = MaybeUninit::<list_head>::uninit();
    let list = storage.as_mut_ptr();
    lupos_sched_init_list(list);
    let mut count = 0;
    loop {
        if let Some(has_work) = (*(*e).type_).ops.has_work {
            if !has_work(hctx) { break; }
        }
        if !lupos_sched_list_empty_careful(addr_of!((*hctx).__bindgen_anon_1.dispatch)) {
            busy = true;
            break;
        }
        let budget = lupos_sched_get_budget(q);
        if budget < 0 { break; }
        let rq = ((*(*e).type_).ops.dispatch_request.unwrap_unchecked())(hctx);
        if rq.is_null() {
            lupos_sched_put_budget(q, budget);
            run_queue = true;
            break;
        }
        lupos_sched_set_budget(rq, budget);
        // From here the request owns the budget, including failed dispatch.
        lupos_sched_list_add_tail(rq_list(rq), list);
        count += 1;
        if (*rq).mq_hctx != hctx { multi_hctxs = true; }
        if !lupos_sched_get_driver_tag(rq) || count >= max_dispatch { break; }
    }
    if count == 0 {
        if run_queue { blk_mq_delay_run_hw_queues(q, BLK_MQ_BUDGET_DELAY); }
    } else if multi_hctxs {
        list_sort(null_mut(), list, Some(sched_rq_cmp));
        while !empty(list) { dispatched |= blk_mq_dispatch_hctx_list(list); }
    } else {
        dispatched = blk_mq_dispatch_rq_list(hctx, list, false);
    }
    if busy { -(EAGAIN as c_int) } else { dispatched as c_int }
}

unsafe fn blk_mq_do_dispatch_sched(hctx: *mut blk_mq_hw_ctx) -> c_int {
    let end = lupos_sched_jiffies().wrapping_add(LUPOS_SCHED_HZ as c_ulong);
    loop {
        let ret = __blk_mq_do_dispatch_sched(hctx);
        if ret != 1 { return ret; }
        // time_is_before_jiffies(end), using the kernel's wrapping signed delta.
        if lupos_sched_need_resched() || (end.wrapping_sub(lupos_sched_jiffies()) as isize) < 0 {
            blk_mq_delay_run_hw_queue(hctx, 0);
            return ret;
        }
    }
}

unsafe fn blk_mq_next_ctx(hctx: *mut blk_mq_hw_ctx, ctx: *mut blk_mq_ctx) -> *mut blk_mq_ctx {
    let mut index = (*ctx).index_hw[(*hctx).type_ as usize].wrapping_add(1);
    if index == (*hctx).nr_ctx { index = 0; }
    *(*hctx).ctxs.add(index as usize)
}

unsafe fn blk_mq_do_dispatch_ctx(hctx: *mut blk_mq_hw_ctx) -> c_int {
    let q = (*hctx).queue;
    let mut storage = MaybeUninit::<list_head>::uninit();
    let list = storage.as_mut_ptr();
    lupos_sched_init_list(list);
    let mut ctx = lupos_sched_read_dispatch_from(hctx);
    let mut ret = 0;
    loop {
        if !lupos_sched_list_empty_careful(addr_of!((*hctx).__bindgen_anon_1.dispatch)) {
            ret = -(EAGAIN as c_int);
            break;
        }
        if !sbitmap_any_bit_set(addr_of!((*hctx).ctx_map)) { break; }
        let token = lupos_sched_get_budget(q);
        if token < 0 { break; }
        let rq = blk_mq_dequeue_from_ctx(hctx, ctx);
        if rq.is_null() {
            lupos_sched_put_budget(q, token);
            blk_mq_delay_run_hw_queues(q, BLK_MQ_BUDGET_DELAY);
            break;
        }
        lupos_sched_set_budget(rq, token);
        lupos_sched_list_add(rq_list(rq), list);
        ctx = blk_mq_next_ctx(hctx, (*rq).mq_ctx);
        if !blk_mq_dispatch_rq_list((*rq).mq_hctx, list, false) { break; }
    }
    lupos_sched_write_dispatch_from(hctx, ctx);
    ret
}

unsafe fn __blk_mq_sched_dispatch_requests(hctx: *mut blk_mq_hw_ctx) -> c_int {
    let mut storage = MaybeUninit::<list_head>::uninit();
    let list = storage.as_mut_ptr();
    lupos_sched_init_list(list);
    let dispatch = addr_of_mut!((*hctx).__bindgen_anon_1.dispatch);
    let lock = addr_of_mut!((*hctx).__bindgen_anon_1.lock);
    if !lupos_sched_list_empty_careful(dispatch) {
        lupos_sched_spin_lock(lock);
        if !empty(dispatch) { lupos_sched_list_splice_init(dispatch, list); }
        lupos_sched_spin_unlock(lock);
    }
    let need_dispatch;
    if !empty(list) {
        blk_mq_sched_mark_restart_hctx(hctx);
        if !blk_mq_dispatch_rq_list(hctx, list, true) { return 0; }
        need_dispatch = true;
    } else {
        need_dispatch = (*hctx).dispatch_busy != 0;
    }
    if !(*(*hctx).queue).elevator.is_null() { return blk_mq_do_dispatch_sched(hctx); }
    if need_dispatch { return blk_mq_do_dispatch_ctx(hctx); }
    blk_mq_flush_busy_ctxs(hctx, list);
    blk_mq_dispatch_rq_list(hctx, list, true);
    0
}

#[no_mangle]
pub unsafe extern "C" fn blk_mq_sched_dispatch_requests(hctx: *mut blk_mq_hw_ctx) {
    if lupos_sched_hctx_stopped(hctx) || lupos_sched_queue_quiesced((*hctx).queue) { return; }
    if __blk_mq_sched_dispatch_requests(hctx) == -(EAGAIN as c_int)
        && __blk_mq_sched_dispatch_requests(hctx) == -(EAGAIN as c_int) {
        blk_mq_run_hw_queue(hctx, true);
    }
}

#[no_mangle]
pub unsafe extern "C" fn blk_mq_sched_bio_merge(q: *mut request_queue, bio: *mut bio, nr_segs: c_uint) -> bool {
    let e = (*q).elevator;
    if !e.is_null() {
        if let Some(merge) = (*(*e).type_).ops.bio_merge { return merge(q, bio, nr_segs); }
    }
    let ctx = lupos_sched_get_ctx(q);
    let hctx = lupos_sched_map_queue((*bio).bi_opf, ctx);
    let list = addr_of_mut!((*ctx).__bindgen_anon_1.rq_lists[(*hctx).type_ as usize]);
    if lupos_sched_list_empty_careful(list) { return false; }
    let lock = addr_of_mut!((*ctx).__bindgen_anon_1.lock);
    lupos_sched_spin_lock(lock);
    let ret = blk_bio_list_merge(q, list, bio, nr_segs);
    lupos_sched_spin_unlock(lock);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn blk_mq_sched_try_insert_merge(q: *mut request_queue, rq: *mut request, free: *mut list_head) -> bool {
    lupos_sched_rq_mergeable(rq) && elv_attempt_insert_merge(q, rq, free)
}

unsafe fn blk_mq_sched_tags_teardown(q: *mut request_queue, flags: c_uint) {
    for_each_hctx!(q, hctx, index, { (*hctx).sched_tags = null_mut(); });
    if shared_tags(flags) { (*q).sched_shared_tags = null_mut(); }
}

#[no_mangle]
pub unsafe extern "C" fn blk_mq_sched_reg_debugfs(q: *mut request_queue) {
    let flags = lupos_sched_debugfs_lock(q);
    lupos_sched_debugfs_register(q);
    for_each_hctx!(q, hctx, index, { lupos_sched_debugfs_register_hctx(q, hctx); });
    lupos_sched_debugfs_unlock(q, flags);
}

#[no_mangle]
pub unsafe extern "C" fn blk_mq_sched_unreg_debugfs(q: *mut request_queue) {
    lupos_sched_debugfs_lock_nomemsave(q);
    for_each_hctx!(q, hctx, index, { lupos_sched_debugfs_unregister_hctx(hctx); });
    lupos_sched_debugfs_unregister(q);
    lupos_sched_debugfs_unlock_nomemrestore(q);
}

#[no_mangle]
pub unsafe extern "C" fn blk_mq_free_sched_tags(et: *mut elevator_tags, set: *mut blk_mq_tag_set) {
    if shared_tags((*set).flags) {
        blk_mq_free_map_and_rqs(set, tag_at(et, 0), BLK_MQ_NO_HCTX_IDX as c_uint);
    } else {
        for index in 0..(*et).nr_hw_queues {
            blk_mq_free_map_and_rqs(set, tag_at(et, index as usize), index);
        }
    }
    kfree(et.cast());
}

#[no_mangle]
pub unsafe extern "C" fn blk_mq_free_sched_res(res: *mut elevator_resources, typ: *mut elevator_type, set: *mut blk_mq_tag_set) {
    if !(*res).et.is_null() {
        blk_mq_free_sched_tags((*res).et, set);
        (*res).et = null_mut();
    }
    if !(*res).data.is_null() {
        if !typ.is_null() {
            if let Some(free) = (*typ).ops.free_sched_data { free((*res).data); }
        }
        (*res).data = null_mut();
    }
}

#[no_mangle]
pub unsafe extern "C" fn blk_mq_free_sched_res_batch(table: *mut xarray, set: *mut blk_mq_tag_set) {
    lupos_sched_assert_update_locked(set);
    let head = addr_of_mut!((*set).tag_list);
    let mut entry = (*head).next;
    while entry != head {
        let q = list_queue(entry);
        if !(*q).elevator.is_null() {
            let ctx = xa_load(table, (*q).id as c_ulong).cast::<elv_change_ctx>();
            if ctx.is_null() {
                lupos_sched_warn_missing_free();
            } else {
                blk_mq_free_sched_res(addr_of_mut!((*ctx).res), (*ctx).type_, set);
            }
        }
        entry = (*entry).next;
    }
}

#[no_mangle]
pub unsafe extern "C" fn blk_mq_free_sched_ctx_batch(table: *mut xarray) {
    let mut index = 0;
    let mut ctx = xa_first(table, &mut index);
    while !ctx.is_null() {
        xa_erase(table, index);
        kfree(ctx);
        ctx = xa_next(table, &mut index);
    }
}

#[no_mangle]
pub unsafe extern "C" fn blk_mq_alloc_sched_ctx_batch(table: *mut xarray, set: *mut blk_mq_tag_set) -> c_int {
    lupos_sched_assert_update_locked(set);
    let head = addr_of_mut!((*set).tag_list);
    let mut entry = (*head).next;
    while entry != head {
        let q = list_queue(entry);
        let ctx = lupos_sched_alloc_ctx();
        if ctx.is_null() { return -(ENOMEM as c_int); }
        if lupos_sched_xa_insert(table, (*q).id as c_ulong, ctx.cast()) != 0 {
            kfree(ctx.cast());
            return -(ENOMEM as c_int);
        }
        entry = (*entry).next;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn blk_mq_alloc_sched_tags(set: *mut blk_mq_tag_set, nr_hw_queues: c_uint, nr_requests: c_uint) -> *mut elevator_tags {
    let shared = shared_tags((*set).flags);
    let et = lupos_sched_alloc_tags_storage(if shared { 1 } else { nr_hw_queues });
    if et.is_null() { return null_mut(); }
    (*et).nr_requests = nr_requests;
    (*et).nr_hw_queues = nr_hw_queues;
    if shared {
        let tags = blk_mq_alloc_map_and_rqs(set, BLK_MQ_NO_HCTX_IDX as c_uint, MAX_SCHED_RQ);
        set_tag(et, 0, tags);
        if tags.is_null() {
            kfree(et.cast());
            return null_mut();
        }
    } else {
        for index in 0..(*et).nr_hw_queues {
            let tags = blk_mq_alloc_map_and_rqs(set, index, (*et).nr_requests);
            set_tag(et, index as usize, tags);
            if tags.is_null() {
                // Only successfully allocated predecessors belong to unwind.
                for previous in (0..index).rev() {
                    blk_mq_free_map_and_rqs(set, tag_at(et, previous as usize), previous);
                }
                kfree(et.cast());
                return null_mut();
            }
        }
    }
    et
}

#[no_mangle]
pub unsafe extern "C" fn blk_mq_alloc_sched_res(q: *mut request_queue, typ: *mut elevator_type, res: *mut elevator_resources, nr_hw_queues: c_uint) -> c_int {
    let set = (*q).tag_set;
    let nr_requests = 2 * core::cmp::min((*set).queue_depth, BLKDEV_DEFAULT_RQ);
    (*res).et = blk_mq_alloc_sched_tags(set, nr_hw_queues, nr_requests);
    if (*res).et.is_null() { return -(ENOMEM as c_int); }
    // Literal blk_mq_alloc_sched_data semantics: absent callback means NULL;
    // a present callback returning NULL is converted to ERR_PTR(-ENOMEM).
    (*res).data = if typ.is_null() {
        null_mut()
    } else if let Some(alloc) = (*typ).ops.alloc_sched_data {
        let data = alloc(q);
        if data.is_null() { (-(ENOMEM as isize)) as *mut c_void } else { data }
    } else {
        null_mut()
    };
    if (*res).data as usize >= (-(MAX_ERRNO as isize)) as usize {
        blk_mq_free_sched_tags((*res).et, set);
        // The caller unwinds predecessors only; retain the original C output
        // state here rather than introducing a new ownership contract.
        return -(ENOMEM as c_int);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn blk_mq_alloc_sched_res_batch(table: *mut xarray, set: *mut blk_mq_tag_set, nr_hw_queues: c_uint) -> c_int {
    lupos_sched_assert_update_locked(set);
    let head = addr_of_mut!((*set).tag_list);
    let mut entry = (*head).next;
    let mut ret = -(ENOMEM as c_int);
    while entry != head {
        let q = list_queue(entry);
        if !(*q).elevator.is_null() {
            let ctx = xa_load(table, (*q).id as c_ulong).cast::<elv_change_ctx>();
            if ctx.is_null() {
                lupos_sched_warn_missing_alloc();
                ret = -(ENOENT as c_int);
                break;
            }
            ret = blk_mq_alloc_sched_res(q, (*(*q).elevator).type_, addr_of_mut!((*ctx).res), nr_hw_queues);
            if ret != 0 { break; }
        }
        entry = (*entry).next;
    }
    if entry == head { return 0; }
    // list_for_each_entry_continue_reverse starts before the failed queue.
    entry = (*entry).prev;
    while entry != head {
        let q = list_queue(entry);
        if !(*q).elevator.is_null() {
            let ctx = xa_load(table, (*q).id as c_ulong).cast::<elv_change_ctx>();
            if !ctx.is_null() {
                blk_mq_free_sched_res(addr_of_mut!((*ctx).res), (*ctx).type_, set);
            }
        }
        entry = (*entry).prev;
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn blk_mq_init_sched(q: *mut request_queue, e: *mut elevator_type, res: *mut elevator_resources) -> c_int {
    let flags = (*(*q).tag_set).flags;
    let et = (*res).et;
    let eq = elevator_alloc(q, e, res);
    if eq.is_null() { return -(ENOMEM as c_int); }
    (*q).nr_requests = (*et).nr_requests;
    if shared_tags(flags) {
        (*q).sched_shared_tags = tag_at(et, 0);
        blk_mq_tag_update_sched_shared_tags(q, (*et).nr_requests);
    }
    for_each_hctx!(q, hctx, index, {
        (*hctx).sched_tags = if shared_tags(flags) { (*q).sched_shared_tags } else { tag_at(et, index as usize) };
    });
    let ret = ((*e).ops.init_sched.unwrap_unchecked())(q, eq);
    if ret != 0 {
        blk_mq_sched_tags_teardown(q, flags);
        kobject_put(addr_of_mut!((*eq).kobj));
        (*q).elevator = null_mut();
        return ret;
    }
    for_each_hctx!(q, hctx, index, {
        if let Some(init) = (*e).ops.init_hctx {
            let ret = init(hctx, index as c_uint);
            if ret != 0 {
                blk_mq_exit_sched(q, eq);
                kobject_put(addr_of_mut!((*eq).kobj));
                return ret;
            }
        }
    });
    0
}

#[no_mangle]
pub unsafe extern "C" fn blk_mq_sched_free_rqs(q: *mut request_queue) {
    let set = (*q).tag_set;
    if shared_tags((*set).flags) {
        blk_mq_free_rqs(set, (*q).sched_shared_tags, BLK_MQ_NO_HCTX_IDX as c_uint);
    } else {
        for_each_hctx!(q, hctx, index, {
            if !(*hctx).sched_tags.is_null() {
                blk_mq_free_rqs(set, (*hctx).sched_tags, index as c_uint);
            }
        });
    }
}

#[no_mangle]
pub unsafe extern "C" fn blk_mq_exit_sched(q: *mut request_queue, e: *mut elevator_queue) {
    let mut flags = 0;
    for_each_hctx!(q, hctx, index, {
        if let Some(exit) = (*(*e).type_).ops.exit_hctx {
            if !(*hctx).sched_data.is_null() {
                exit(hctx, index as c_uint);
                (*hctx).sched_data = null_mut();
            }
        }
        flags = (*hctx).flags as c_uint;
    });
    if let Some(exit) = (*(*e).type_).ops.exit_sched { exit(e); }
    blk_mq_sched_tags_teardown(q, flags);
    lupos_sched_set_bit(ELEVATOR_FLAG_DYING, addr_of_mut!((*(*q).elevator).flags));
    (*q).elevator = null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
