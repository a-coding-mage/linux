/* SPDX-License-Identifier: GPL-2.0 */
/* C dependencies: linux/bio-integrity.h, linux/blk-crypto.h,
 * linux/part_stat.h, linux/lockdep.h, linux/memblock.h,
 * linux/sched/sysctl.h, linux/timekeeping.h, xen/xen.h,
 * and blk-crypto-internal.h. */

pub const BLK_DEF_MAX_SECTORS_CAP: _ = SZ_4M >> SECTOR_SHIFT;
pub const BLK_DEV_MAX_SECTORS: _ = LLONG_MAX >> 9;
pub const BLK_MIN_SEGMENT_SIZE: usize = 4096;
pub const BLK_MAX_TIMEOUT: _ = 5 * HZ;

pub struct elv_change_ctx;

#[repr(C)]
pub struct blk_flush_queue {
    pub mq_flush_lock: spinlock_t,
    pub flush_pending_idx: u32,
    pub flush_running_idx: u32,
    pub rq_status: blk_status_t,
    pub flush_pending_since: ::core::ffi::c_ulong,
    pub flush_queue: [list_head; 2],
    pub flush_data_in_flight: ::core::ffi::c_ulong,
    pub flush_rq: *mut request,
    pub rcu_head: rcu_head,
}

extern "C" {
    pub static blk_queue_ktype: kobj_type;
    pub static mut blk_debugfs_root: *mut dentry;

    pub fn is_flush_rq(req: *mut request) -> bool;
    pub fn blk_alloc_flush_queue(node: i32, cmd_size: i32, flags: gfp_t) -> *mut blk_flush_queue;
    pub fn blk_free_flush_queue(q: *mut blk_flush_queue);
    pub fn blk_status_to_str(status: blk_status_t) -> *const ::core::ffi::c_char;
    pub fn blk_status_to_tag(status: blk_status_t) -> *const ::core::ffi::c_char;
    pub fn tag_to_blk_status(tag: *const ::core::ffi::c_char) -> blk_status_t;
    pub fn str_to_blk_op(op: *const ::core::ffi::c_char) -> req_op;
    pub fn __blk_mq_unfreeze_queue(q: *mut request_queue, force_atomic: bool) -> bool;
    pub fn blk_queue_start_drain(q: *mut request_queue) -> bool;
    pub fn __blk_freeze_queue_start(q: *mut request_queue, owner: *mut task_struct) -> bool;
    pub fn __bio_queue_enter(q: *mut request_queue, bio: *mut bio) -> i32;
    pub fn submit_bio_noacct_nocheck(bio: *mut bio, split: bool);
    pub fn bio_submit_or_kill(bio: *mut bio, flags: ::core::ffi::c_uint) -> i32;

    pub fn blkdev_get_no_open(dev: dev_t, autoload: bool) -> *mut block_device;
    pub fn blkdev_put_no_open(bdev: *mut block_device);
    pub fn bvec_try_merge_hw_page(q: *mut request_queue, bv: *mut bio_vec, page: *mut page,
                                   len: ::core::ffi::c_uint, offset: ::core::ffi::c_uint) -> bool;
    pub fn blk_rq_timeout(timeout: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn blk_add_timer(req: *mut request);
}

pub unsafe fn blk_try_enter_queue(q: *mut request_queue, pm: bool) -> bool {
    rcu_read_lock();
    if !percpu_ref_tryget_live_rcu(&mut (*q).q_usage_counter) { rcu_read_unlock(); return false; }
    if blk_queue_pm_only(q) && (!pm || queue_rpm_status(q) == RPM_SUSPENDED) {
        blk_queue_exit(q); rcu_read_unlock(); return false;
    }
    rcu_read_unlock(); true
}

pub unsafe fn bio_queue_enter(bio: *mut bio) -> i32 {
    let q = bdev_get_queue((*bio).bi_bdev);
    if blk_try_enter_queue(q, false) {
        rwsem_acquire_read(&mut (*q).io_lockdep_map, 0, 0, _RET_IP_);
        rwsem_release(&mut (*q).io_lockdep_map, _RET_IP_);
        return 0;
    }
    __bio_queue_enter(q, bio)
}

pub unsafe fn blk_wait_io(done: *mut completion) {
    let timeout = sysctl_hung_task_timeout_secs * HZ / 2;
    if timeout != 0 { while wait_for_completion_io_timeout(done, timeout) == 0 {} }
    else { wait_for_completion_io(done); }
}

pub unsafe fn biovec_phys_mergeable(q: *mut request_queue, vec1: *mut bio_vec, vec2: *mut bio_vec) -> bool {
    let mask = queue_segment_boundary(q);
    let addr1 = bvec_phys(vec1); let addr2 = bvec_phys(vec2);
    if IS_ENABLED(CONFIG_KMSAN) { return false; }
    if addr1 + (*vec1).bv_len != addr2 { return false; }
    if !zone_device_pages_have_same_pgmap((*vec1).bv_page, (*vec2).bv_page) { return false; }
    if xen_domain() && !xen_biovec_phys_mergeable(vec1, (*vec2).bv_page) { return false; }
    if (addr1 | mask) != ((addr2 + (*vec2).bv_len - 1) | mask) { return false; }
    true
}

pub unsafe fn zone_device_pages_compatible(a: *const page, b: *const page) -> bool {
    if is_pci_p2pdma_page(a) || is_pci_p2pdma_page(b) { zone_device_pages_have_same_pgmap(a, b) } else { true }
}

pub unsafe fn __bvec_gap_to_prev(lim: *const queue_limits, bprv: *const bio_vec, offset: ::core::ffi::c_uint) -> bool {
    (offset & (*lim).virt_boundary_mask) != 0 || (((*bprv).bv_offset + (*bprv).bv_len) & (*lim).virt_boundary_mask) != 0
}
pub unsafe fn bvec_gap_to_prev(lim: *const queue_limits, bprv: *const bio_vec, offset: ::core::ffi::c_uint) -> bool {
    if (*lim).virt_boundary_mask == 0 { false } else { __bvec_gap_to_prev(lim, bprv, offset) }
}

pub unsafe fn rq_mergeable(rq: *mut request) -> bool {
    if blk_rq_is_passthrough(rq) || req_op(rq) == REQ_OP_FLUSH || req_op(rq) == REQ_OP_WRITE_ZEROES || req_op(rq) == REQ_OP_ZONE_APPEND { return false; }
    if ((*rq).cmd_flags & REQ_NOMERGE_FLAGS) != 0 || ((*rq).rq_flags & RQF_NOMERGE_FLAGS) != 0 { return false; }
    true
}
pub unsafe fn blk_discard_mergable(req: *mut request) -> bool { req_op(req) == REQ_OP_DISCARD && queue_max_discard_segments((*req).q) > 1 }
pub unsafe fn blk_rq_get_max_segments(rq: *mut request) -> ::core::ffi::c_uint { if req_op(rq) == REQ_OP_DISCARD { queue_max_discard_segments((*rq).q) } else { queue_max_segments((*rq).q) } }
pub unsafe fn blk_queue_get_max_sectors(rq: *mut request) -> ::core::ffi::c_uint {
    let q = (*rq).q; let op = req_op(rq);
    if unlikely(op == REQ_OP_DISCARD) { return min((*q).limits.max_discard_sectors, UINT_MAX >> SECTOR_SHIFT); }
    if unlikely(op == REQ_OP_SECURE_ERASE) { return min((*q).limits.max_secure_erase_sectors, UINT_MAX >> SECTOR_SHIFT); }
    if unlikely(op == REQ_OP_WRITE_ZEROES) { return (*q).limits.max_write_zeroes_sectors; }
    if ((*rq).cmd_flags & REQ_ATOMIC) != 0 { return (*q).limits.atomic_write_max_sectors; }
    (*q).limits.max_sectors
}

/* CONFIG_BLK_DEV_INTEGRITY declarations and fallback inline definitions. */
extern "C" {
    pub fn blk_flush_integrity();
    pub fn bio_integrity_free(bio: *mut bio);
    pub fn __bio_integrity_endio(bio: *mut bio) -> bool;
    pub fn blk_integrity_merge_rq(rq: *mut request_queue, r1: *mut request, r2: *mut request) -> bool;
    pub fn blk_integrity_merge_bio(rq: *mut request_queue, r: *mut request, b: *mut bio) -> bool;
    pub static blk_integrity_attr_group: attribute_group;
}
pub unsafe fn bio_integrity_endio(bio: *mut bio) -> bool {
    let bip = bio_integrity(bio);
    if !bip.is_null() && ((*bip).bip_flags & BIP_BLOCK_INTEGRITY) != 0 { __bio_integrity_endio(bio) } else { true }
}
pub unsafe fn integrity_req_gap_back_merge(req: *mut request, next: *mut bio) -> bool {
    let bip = bio_integrity((*req).bio); let bip_next = bio_integrity(next);
    bvec_gap_to_prev(&(*req).q as *const _, &(*bip).bip_vec[(*bip).bip_vcnt - 1], (*bip_next).bip_vec[0].bv_offset)
}
pub unsafe fn integrity_req_gap_front_merge(req: *mut request, bio: *mut bio) -> bool {
    let bip = bio_integrity(bio); let bip_next = bio_integrity((*req).bio);
    bvec_gap_to_prev(&(*req).q as *const _, &(*bip).bip_vec[(*bip).bip_vcnt - 1], (*bip_next).bip_vec[0].bv_offset)
}

pub const BLK_MAX_REQUEST_COUNT: u32 = 32;
pub const BLK_PLUG_FLUSH_SIZE: u32 = 128 * 1024;
pub unsafe fn elv_on_hash(rq: *mut request) -> bool { ((*rq).rq_flags & RQF_HASHED) != 0 }

pub unsafe fn bio_may_need_split(bio: *mut bio, lim: *const queue_limits) -> bool {
    if (*lim).chunk_sectors != 0 || (*bio).bi_io_vec.is_null() { return true; }
    let bv = __bvec_iter_bvec((*bio).bi_io_vec, (*bio).bi_iter);
    if (*bio).bi_iter.bi_size > (*bv).bv_len - (*bio).bi_iter.bi_offset { return true; }
    if (((*bv).bv_offset | (*bv).bv_len) & (*lim).dma_alignment) != 0 { return true; }
    (*bv).bv_len + (*bv).bv_offset > (*lim).max_fast_segment_size
}

pub unsafe fn __bio_split_to_limits(bio: *mut bio, lim: *const queue_limits, nr_segs: *mut ::core::ffi::c_uint) -> *mut bio {
    match bio_op(bio) {
        REQ_OP_READ | REQ_OP_WRITE => { if bio_may_need_split(bio, lim) { bio_split_rw(bio, lim, nr_segs) } else { *nr_segs = 1; bio } }
        REQ_OP_ZONE_APPEND => bio_split_zone_append(bio, lim, nr_segs),
        REQ_OP_DISCARD | REQ_OP_SECURE_ERASE => bio_split_discard(bio, lim, nr_segs),
        REQ_OP_WRITE_ZEROES => bio_split_write_zeroes(bio, lim, nr_segs),
        _ => { *nr_segs = 0; bio }
    }
}
pub unsafe fn get_max_segment_size(lim: *const queue_limits, paddr: phys_addr_t, len: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    min_t::<::core::ffi::c_ulong>(len as _, min((*lim).seg_boundary_mask - ((*lim).seg_boundary_mask & paddr), (*lim).max_segment_size as _ - 1) + 1) as _
}

pub enum bio_merge_status { BIO_MERGE_OK, BIO_MERGE_NONE, BIO_MERGE_FAILED }

extern "C" {
    pub fn part_size_show(dev: *mut device, attr: *mut device_attribute, buf: *mut ::core::ffi::c_char) -> ssize_t;
    pub fn part_stat_show(dev: *mut device, attr: *mut device_attribute, buf: *mut ::core::ffi::c_char) -> ssize_t;
    pub fn part_inflight_show(dev: *mut device, attr: *mut device_attribute, buf: *mut ::core::ffi::c_char) -> ssize_t;
    pub fn part_fail_show(dev: *mut device, attr: *mut device_attribute, buf: *mut ::core::ffi::c_char) -> ssize_t;
    pub fn part_fail_store(dev: *mut device, attr: *mut device_attribute, buf: *const ::core::ffi::c_char, count: usize) -> ssize_t;
    pub fn part_timeout_show(dev: *mut device, attr: *mut device_attribute, buf: *mut ::core::ffi::c_char) -> ssize_t;
    pub fn part_timeout_store(dev: *mut device, attr: *mut device_attribute, buf: *const ::core::ffi::c_char, count: usize) -> ssize_t;
    pub fn blk_insert_flush(rq: *mut request) -> bool;
    pub fn elv_update_nr_hw_queues(q: *mut request_queue, ctx: *mut elv_change_ctx);
    pub fn elevator_set_default(q: *mut request_queue);
    pub fn elevator_set_none(q: *mut request_queue);
    pub fn bio_split_discard(bio: *mut bio, lim: *const queue_limits, nsegs: *mut ::core::ffi::c_uint) -> *mut bio;
    pub fn bio_split_write_zeroes(bio: *mut bio, lim: *const queue_limits, nsegs: *mut ::core::ffi::c_uint) -> *mut bio;
    pub fn bio_split_rw(bio: *mut bio, lim: *const queue_limits, nr_segs: *mut ::core::ffi::c_uint) -> *mut bio;
    pub fn bio_split_zone_append(bio: *mut bio, lim: *const queue_limits, nr_segs: *mut ::core::ffi::c_uint) -> *mut bio;
    pub fn bio_attempt_back_merge(req: *mut request, bio: *mut bio, nr_segs: ::core::ffi::c_uint) -> bio_merge_status;
    pub fn blk_attempt_plug_merge(q: *mut request_queue, bio: *mut bio, nr_segs: ::core::ffi::c_uint) -> bool;
    pub fn blk_bio_list_merge(q: *mut request_queue, list: *mut list_head, bio: *mut bio, nr_segs: ::core::ffi::c_uint) -> bool;
    pub fn ll_back_merge_fn(req: *mut request, bio: *mut bio, nr_segs: ::core::ffi::c_uint) -> i32;
    pub fn blk_attempt_req_merge(q: *mut request_queue, rq: *mut request, next: *mut request) -> bool;
    pub fn blk_recalc_rq_segments(rq: *mut request) -> ::core::ffi::c_uint;
    pub fn blk_rq_merge_ok(rq: *mut request, bio: *mut bio) -> bool;
    pub fn blk_try_merge(rq: *mut request, bio: *mut bio) -> elv_merge;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
