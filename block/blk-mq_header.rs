/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct blk_mq_ctxs {
    pub kobj: kobject,
    pub queue_ctx: *mut blk_mq_ctx,
}

#[repr(C)]
pub struct blk_mq_ctx {
    pub lock: spinlock_t,
    pub rq_lists: [list_head; HCTX_MAX_TYPES],
    pub cpu: c_uint,
    pub index_hw: [c_ushort; HCTX_MAX_TYPES],
    pub hctxs: [*mut blk_mq_hw_ctx; HCTX_MAX_TYPES],
    pub queue: *mut request_queue,
    pub ctxs: *mut blk_mq_ctxs,
    pub kobj: kobject,
}

pub const BLK_MQ_NO_TAG: c_uint = !0u32;
pub const BLK_MQ_TAG_MIN: c_uint = 1;
pub const BLK_MQ_TAG_MAX: c_uint = BLK_MQ_NO_TAG - 1;
pub const BLK_MQ_CPU_WORK_BATCH: c_uint = 8;

pub type blk_insert_t = c_uint;
pub const BLK_MQ_INSERT_AT_HEAD: blk_insert_t = 0x01;

extern "C" {
    pub fn blk_mq_submit_bio(bio: *mut bio);
    pub fn blk_mq_poll(q: *mut request_queue, cookie: blk_qc_t,
                       iob: *mut io_comp_batch, flags: c_uint) -> c_int;
    pub fn blk_mq_exit_queue(q: *mut request_queue);
    pub fn blk_mq_update_nr_requests(q: *mut request_queue, tags: *mut elevator_tags,
                                     nr: c_uint) -> *mut elevator_tags;
    pub fn blk_mq_wake_waiters(q: *mut request_queue);
    pub fn blk_mq_dispatch_rq_list(hctx: *mut blk_mq_hw_ctx, list: *mut list_head, b: bool) -> bool;
    pub fn blk_mq_flush_busy_ctxs(hctx: *mut blk_mq_hw_ctx, list: *mut list_head);
    pub fn blk_mq_dequeue_from_ctx(hctx: *mut blk_mq_hw_ctx, start: *mut blk_mq_ctx) -> *mut request;
    pub fn blk_mq_put_rq_ref(rq: *mut request);
    pub fn blk_mq_free_rqs(set: *mut blk_mq_tag_set, tags: *mut blk_mq_tags, hctx_idx: c_uint);
    pub fn blk_mq_free_rq_map(set: *mut blk_mq_tag_set, tags: *mut blk_mq_tags);
    pub fn blk_mq_alloc_map_and_rqs(set: *mut blk_mq_tag_set, hctx_idx: c_uint, depth: c_uint) -> *mut blk_mq_tags;
    pub fn blk_mq_free_map_and_rqs(set: *mut blk_mq_tag_set, tags: *mut blk_mq_tags, hctx_idx: c_uint);
    pub fn blk_mq_hw_queue_to_node(qmap: *mut blk_mq_queue_map, index: c_uint) -> c_int;
    pub fn blk_mq_sysfs_init(q: *mut request_queue);
    pub fn blk_mq_sysfs_deinit(q: *mut request_queue);
    pub fn blk_mq_sysfs_register(disk: *mut gendisk) -> c_int;
    pub fn blk_mq_sysfs_unregister(disk: *mut gendisk);
    pub fn blk_mq_sysfs_register_hctxs(q: *mut request_queue) -> c_int;
    pub fn blk_mq_sysfs_unregister_hctxs(q: *mut request_queue);
    pub fn blk_mq_hctx_kobj_init(hctx: *mut blk_mq_hw_ctx);
    pub fn blk_mq_free_plug_rqs(plug: *mut blk_plug);
    pub fn blk_mq_flush_plug_list(plug: *mut blk_plug, from_schedule: bool);
    pub fn blk_mq_cancel_work_sync(q: *mut request_queue);
    pub fn blk_mq_release(q: *mut request_queue);
    pub fn blk_mq_init_tags(nr_tags: c_uint, reserved_tags: c_uint, flags: c_uint, node: c_int) -> *mut blk_mq_tags;
    pub fn blk_mq_free_tags(set: *mut blk_mq_tag_set, tags: *mut blk_mq_tags);
    pub fn blk_mq_get_tag(data: *mut blk_mq_alloc_data) -> c_uint;
    pub fn blk_mq_get_tags(data: *mut blk_mq_alloc_data, nr_tags: c_int, offset: *mut c_uint) -> c_ulong;
    pub fn blk_mq_put_tag(tags: *mut blk_mq_tags, ctx: *mut blk_mq_ctx, tag: c_uint);
    pub fn blk_mq_put_tags(tags: *mut blk_mq_tags, tag_array: *mut c_int, nr_tags: c_int);
    pub fn blk_mq_tag_resize_shared_tags(set: *mut blk_mq_tag_set, size: c_uint);
    pub fn blk_mq_tag_update_sched_shared_tags(q: *mut request_queue, nr: c_uint);
    pub fn blk_mq_tag_wakeup_all(tags: *mut blk_mq_tags, _: bool);
    pub fn blk_mq_queue_tag_busy_iter(q: *mut request_queue, f: *mut busy_tag_iter_fn, priv_: *mut c_void);
    pub fn blk_mq_all_tag_iter(tags: *mut blk_mq_tags, f: *mut busy_tag_iter_fn, priv_: *mut c_void);
    pub fn __blk_mq_tag_busy(hctx: *mut blk_mq_hw_ctx);
    pub fn __blk_mq_tag_idle(hctx: *mut blk_mq_hw_ctx);
    pub fn blk_mq_in_driver_rw(part: *mut block_device, inflight: *mut [c_uint; 2]);
    pub fn __blk_mq_alloc_driver_tag(rq: *mut request) -> bool;
}

#[repr(C)]
pub struct blk_mq_alloc_data {
    pub q: *mut request_queue,
    pub flags: blk_mq_req_flags_t,
    pub shallow_depth: c_uint,
    pub cmd_flags: blk_opf_t,
    pub rq_flags: req_flags_t,
    pub nr_tags: c_uint,
    pub cached_rqs: *mut rq_list,
    pub ctx: *mut blk_mq_ctx,
    pub hctx: *mut blk_mq_hw_ctx,
}

#[inline]
pub unsafe fn blk_mq_map_queue_type(q: *mut request_queue, ty: hctx_type, cpu: c_uint) -> *mut blk_mq_hw_ctx {
    queue_hctx(q, (*(*q).tag_set).map[ty as usize].mq_map[cpu as usize])
}

#[inline]
pub unsafe fn blk_mq_get_hctx_type(opf: blk_opf_t) -> hctx_type {
    if opf & REQ_POLLED != 0 { HCTX_TYPE_POLL }
    else if opf & REQ_OP_MASK == REQ_OP_READ { HCTX_TYPE_READ }
    else { HCTX_TYPE_DEFAULT }
}

#[inline]
pub unsafe fn blk_mq_map_queue(opf: blk_opf_t, ctx: *mut blk_mq_ctx) -> *mut blk_mq_hw_ctx {
    (*ctx).hctxs[blk_mq_get_hctx_type(opf) as usize]
}

#[inline]
pub unsafe fn blk_mq_default_nr_requests(set: *mut blk_mq_tag_set) -> c_uint {
    2 * min_t((*set).queue_depth, BLKDEV_DEFAULT_RQ)
}

#[inline]
pub unsafe fn __blk_mq_get_ctx(q: *mut request_queue, cpu: c_uint) -> *mut blk_mq_ctx {
    per_cpu_ptr((*q).queue_ctx, cpu)
}

#[inline]
pub unsafe fn blk_mq_get_ctx(q: *mut request_queue) -> *mut blk_mq_ctx {
    __blk_mq_get_ctx(q, raw_smp_processor_id())
}

#[inline]
pub unsafe fn bt_wait_ptr(bt: *mut sbitmap_queue, hctx: *mut blk_mq_hw_ctx) -> *mut sbq_wait_state {
    if hctx.is_null() { &mut (*bt).ws[0] } else { sbq_wait_ptr(bt, &mut (*hctx).wait_index) }
}

#[inline]
pub unsafe fn blk_mq_tag_busy(hctx: *mut blk_mq_hw_ctx) {
    if (*hctx).flags & BLK_MQ_F_TAG_QUEUE_SHARED != 0 { __blk_mq_tag_busy(hctx); }
}
#[inline]
pub unsafe fn blk_mq_tag_idle(hctx: *mut blk_mq_hw_ctx) {
    if (*hctx).flags & BLK_MQ_F_TAG_QUEUE_SHARED != 0 { __blk_mq_tag_idle(hctx); }
}
#[inline]
pub unsafe fn blk_mq_tag_is_reserved(tags: *mut blk_mq_tags, tag: c_uint) -> bool { tag < (*tags).nr_reserved_tags }
#[inline]
pub unsafe fn blk_mq_is_shared_tags(flags: c_uint) -> bool { flags & BLK_MQ_F_TAG_HCTX_SHARED != 0 }
#[inline]
pub unsafe fn blk_mq_tags_from_data(data: *mut blk_mq_alloc_data) -> *mut blk_mq_tags {
    if (*data).rq_flags & RQF_SCHED_TAGS != 0 { (*(*data).hctx).sched_tags } else { (*(*data).hctx).tags }
}

#[inline]
pub unsafe fn blk_mq_hctx_stopped(hctx: *mut blk_mq_hw_ctx) -> bool {
    if likely(!test_bit(BLK_MQ_S_STOPPED, &(*hctx).state)) { return false; }
    smp_mb();
    test_bit(BLK_MQ_S_STOPPED, &(*hctx).state)
}
#[inline]
pub unsafe fn blk_mq_hw_queue_mapped(hctx: *mut blk_mq_hw_ctx) -> bool { (*hctx).nr_ctx != 0 && !(*hctx).tags.is_null() }

#[inline]
pub unsafe fn blk_mq_put_dispatch_budget(q: *mut request_queue, token: c_int) {
    if let Some(f) = (*(*q).mq_ops).put_budget { f(q, token); }
}
#[inline]
pub unsafe fn blk_mq_get_dispatch_budget(q: *mut request_queue) -> c_int {
    if let Some(f) = (*(*q).mq_ops).get_budget { f(q) } else { 0 }
}
#[inline]
pub unsafe fn blk_mq_set_rq_budget_token(rq: *mut request, token: c_int) {
    if token < 0 { return; }
    if let Some(f) = (*(*(*rq).q).mq_ops).set_rq_budget_token { f(rq, token); }
}
#[inline]
pub unsafe fn blk_mq_get_rq_budget_token(rq: *mut request) -> c_int {
    if let Some(f) = (*(*(*rq).q).mq_ops).get_rq_budget_token { f(rq) } else { -1 }
}

#[inline] pub unsafe fn __blk_mq_add_active_requests(h: *mut blk_mq_hw_ctx, val: c_int) { if blk_mq_is_shared_tags((*h).flags) { atomic_add(val, &mut (*(*h).queue).nr_active_requests_shared_tags) } else { atomic_add(val, &mut (*h).nr_active) } }
#[inline] pub unsafe fn __blk_mq_inc_active_requests(h: *mut blk_mq_hw_ctx) { __blk_mq_add_active_requests(h, 1); }
#[inline] pub unsafe fn __blk_mq_sub_active_requests(h: *mut blk_mq_hw_ctx, val: c_int) { if blk_mq_is_shared_tags((*h).flags) { atomic_sub(val, &mut (*(*h).queue).nr_active_requests_shared_tags) } else { atomic_sub(val, &mut (*h).nr_active) } }
#[inline] pub unsafe fn __blk_mq_dec_active_requests(h: *mut blk_mq_hw_ctx) { __blk_mq_sub_active_requests(h, 1); }
#[inline] pub unsafe fn blk_mq_add_active_requests(h: *mut blk_mq_hw_ctx, v: c_int) { if (*h).flags & BLK_MQ_F_TAG_QUEUE_SHARED != 0 { __blk_mq_add_active_requests(h, v); } }
#[inline] pub unsafe fn blk_mq_inc_active_requests(h: *mut blk_mq_hw_ctx) { if (*h).flags & BLK_MQ_F_TAG_QUEUE_SHARED != 0 { __blk_mq_inc_active_requests(h); } }
#[inline] pub unsafe fn blk_mq_sub_active_requests(h: *mut blk_mq_hw_ctx, v: c_int) { if (*h).flags & BLK_MQ_F_TAG_QUEUE_SHARED != 0 { __blk_mq_sub_active_requests(h, v); } }
#[inline] pub unsafe fn blk_mq_dec_active_requests(h: *mut blk_mq_hw_ctx) { if (*h).flags & BLK_MQ_F_TAG_QUEUE_SHARED != 0 { __blk_mq_dec_active_requests(h); } }
#[inline] pub unsafe fn __blk_mq_active_requests(h: *mut blk_mq_hw_ctx) -> c_int { if blk_mq_is_shared_tags((*h).flags) { atomic_read(&(*(*h).queue).nr_active_requests_shared_tags) } else { atomic_read(&(*h).nr_active) } }
#[inline] pub unsafe fn __blk_mq_put_driver_tag(h: *mut blk_mq_hw_ctx, rq: *mut request) { blk_mq_dec_active_requests(h); blk_mq_put_tag((*h).tags, (*rq).mq_ctx, (*rq).tag); (*rq).tag = BLK_MQ_NO_TAG; }
#[inline] pub unsafe fn blk_mq_put_driver_tag(rq: *mut request) { if (*rq).tag == BLK_MQ_NO_TAG || (*rq).internal_tag == BLK_MQ_NO_TAG { return; } __blk_mq_put_driver_tag((*rq).mq_hctx, rq); }
#[inline] pub unsafe fn blk_mq_get_driver_tag(rq: *mut request) -> bool { if (*rq).tag == BLK_MQ_NO_TAG && !__blk_mq_alloc_driver_tag(rq) { return false; } true }

#[inline]
pub unsafe fn blk_mq_clear_mq_map(qmap: *mut blk_mq_queue_map) {
    for_each_possible_cpu!(cpu => { (*qmap).mq_map[cpu as usize] = 0; });
}

#[inline]
pub unsafe fn blk_mq_free_requests(list: *mut list_head) {
    while !list_empty(list) { let rq = list_entry_rq((*list).next); list_del_init(&mut (*rq).queuelist); blk_mq_free_request(rq); }
}

#[inline]
pub unsafe fn hctx_may_queue(hctx: *mut blk_mq_hw_ctx, bt: *mut sbitmap_queue) -> bool {
    if hctx.is_null() || (*hctx).flags & BLK_MQ_F_TAG_QUEUE_SHARED == 0 { return true; }
    if (*bt).sb.depth == 1 { return true; }
    if blk_mq_is_shared_tags((*hctx).flags) {
        let q = (*hctx).queue;
        if !test_bit(QUEUE_FLAG_HCTX_ACTIVE, &(*q).queue_flags) { return true; }
    } else if !test_bit(BLK_MQ_S_TAG_ACTIVE, &(*hctx).state) { return true; }
    let users = READ_ONCE((*(*hctx).tags).active_queues);
    if users == 0 { return true; }
    let depth = max(((*bt).sb.depth + users - 1) / users, 4);
    __blk_mq_active_requests(hctx) < depth as c_int
}

#[inline]
pub unsafe fn blk_mq_can_poll(q: *mut request_queue) -> bool {
    (*q).limits.features & BLK_FEAT_POLL != 0 && (*(*q).tag_set).map[HCTX_TYPE_POLL as usize].nr_queues != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
