/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here but not defined in this header translation.

pub struct blk_mq_debugfs_attr;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rq_qos_id {
    RQ_QOS_WBT,
    RQ_QOS_LATENCY,
    RQ_QOS_COST,
}

#[repr(C)]
pub struct rq_wait {
    pub wait: wait_queue_head_t,
    pub inflight: atomic_t,
}

#[repr(C)]
pub struct rq_qos {
    pub ops: *const rq_qos_ops,
    pub disk: *mut gendisk,
    pub id: rq_qos_id,
    pub next: *mut rq_qos,
    // CONFIG_BLK_DEBUG_FS
    pub debugfs_dir: *mut dentry,
}

pub type rq_qos_throttle_fn = unsafe extern "C" fn(*mut rq_qos, *mut bio);
pub type rq_qos_track_fn = unsafe extern "C" fn(*mut rq_qos, *mut request, *mut bio);
pub type rq_qos_merge_fn = unsafe extern "C" fn(*mut rq_qos, *mut request, *mut bio);
pub type rq_qos_issue_fn = unsafe extern "C" fn(*mut rq_qos, *mut request);
pub type rq_qos_requeue_fn = unsafe extern "C" fn(*mut rq_qos, *mut request);
pub type rq_qos_done_fn = unsafe extern "C" fn(*mut rq_qos, *mut request);
pub type rq_qos_done_bio_fn = unsafe extern "C" fn(*mut rq_qos, *mut bio);
pub type rq_qos_cleanup_fn = unsafe extern "C" fn(*mut rq_qos, *mut bio);
pub type rq_qos_queue_depth_changed_fn = unsafe extern "C" fn(*mut rq_qos);
pub type rq_qos_exit_fn = unsafe extern "C" fn(*mut rq_qos);

#[repr(C)]
pub struct rq_qos_ops {
    pub throttle: Option<rq_qos_throttle_fn>,
    pub track: Option<rq_qos_track_fn>,
    pub merge: Option<rq_qos_merge_fn>,
    pub issue: Option<rq_qos_issue_fn>,
    pub requeue: Option<rq_qos_requeue_fn>,
    pub done: Option<rq_qos_done_fn>,
    pub done_bio: Option<rq_qos_done_bio_fn>,
    pub cleanup: Option<rq_qos_cleanup_fn>,
    pub queue_depth_changed: Option<rq_qos_queue_depth_changed_fn>,
    pub exit: Option<rq_qos_exit_fn>,
    pub debugfs_attrs: *const blk_mq_debugfs_attr,
}

#[repr(C)]
pub struct rq_depth {
    pub max_depth: ::core::ffi::c_uint,
    pub scale_step: ::core::ffi::c_int,
    pub scaled_max: bool,
    pub queue_depth: ::core::ffi::c_uint,
    pub default_depth: ::core::ffi::c_uint,
}

pub unsafe fn rq_qos_id(q: *mut request_queue, id: rq_qos_id) -> *mut rq_qos {
    let mut rqos = (*q).rq_qos;
    while !rqos.is_null() {
        if (*rqos).id == id { break; }
        rqos = (*rqos).next;
    }
    rqos
}

pub unsafe fn wbt_rq_qos(q: *mut request_queue) -> *mut rq_qos { rq_qos_id(q, rq_qos_id::RQ_QOS_WBT) }
pub unsafe fn iolat_rq_qos(q: *mut request_queue) -> *mut rq_qos { rq_qos_id(q, rq_qos_id::RQ_QOS_LATENCY) }

pub unsafe fn rq_wait_init(rq_wait: *mut rq_wait) {
    atomic_set(&mut (*rq_wait).inflight, 0);
    init_waitqueue_head(&mut (*rq_wait).wait);
}

pub unsafe extern "C" {
    pub fn rq_qos_add(rqos: *mut rq_qos, disk: *mut gendisk, id: rq_qos_id, ops: *const rq_qos_ops) -> ::core::ffi::c_int;
    pub fn rq_qos_del(rqos: *mut rq_qos);
}

pub type acquire_inflight_cb_t = unsafe extern "C" fn(*mut rq_wait, *mut ::core::ffi::c_void) -> bool;
pub type cleanup_cb_t = unsafe extern "C" fn(*mut rq_wait, *mut ::core::ffi::c_void);

pub unsafe extern "C" {
    pub fn rq_qos_wait(rqw: *mut rq_wait, private_data: *mut ::core::ffi::c_void, acquire_inflight_cb: Option<acquire_inflight_cb_t>, cleanup_cb: Option<cleanup_cb_t>);
    pub fn rq_wait_inc_below(rq_wait: *mut rq_wait, limit: ::core::ffi::c_uint) -> bool;
    pub fn rq_depth_scale_up(rqd: *mut rq_depth) -> bool;
    pub fn rq_depth_scale_down(rqd: *mut rq_depth, hard_throttle: bool) -> bool;
    pub fn rq_depth_calc_max_depth(rqd: *mut rq_depth) -> bool;
    pub fn __rq_qos_cleanup(rqos: *mut rq_qos, bio: *mut bio);
    pub fn __rq_qos_done(rqos: *mut rq_qos, rq: *mut request);
    pub fn __rq_qos_issue(rqos: *mut rq_qos, rq: *mut request);
    pub fn __rq_qos_requeue(rqos: *mut rq_qos, rq: *mut request);
    pub fn __rq_qos_throttle(rqos: *mut rq_qos, bio: *mut bio);
    pub fn __rq_qos_track(rqos: *mut rq_qos, rq: *mut request, bio: *mut bio);
    pub fn __rq_qos_merge(rqos: *mut rq_qos, rq: *mut request, bio: *mut bio);
    pub fn __rq_qos_done_bio(rqos: *mut rq_qos, bio: *mut bio);
    pub fn __rq_qos_queue_depth_changed(rqos: *mut rq_qos);
    pub fn rq_qos_exit(q: *mut request_queue);
}

pub unsafe fn rq_qos_cleanup(q: *mut request_queue, bio: *mut bio) {
    if test_bit(QUEUE_FLAG_QOS_ENABLED, &(*q).queue_flags) && !(*q).rq_qos.is_null() {
        __rq_qos_cleanup((*q).rq_qos, bio);
    }
}

pub unsafe fn rq_qos_done(q: *mut request_queue, rq: *mut request) {
    if test_bit(QUEUE_FLAG_QOS_ENABLED, &(*q).queue_flags) && !(*q).rq_qos.is_null() && !blk_rq_is_passthrough(rq) {
        __rq_qos_done((*q).rq_qos, rq);
    }
}

pub unsafe fn rq_qos_issue(q: *mut request_queue, rq: *mut request) {
    if test_bit(QUEUE_FLAG_QOS_ENABLED, &(*q).queue_flags) && !(*q).rq_qos.is_null() { __rq_qos_issue((*q).rq_qos, rq); }
}

pub unsafe fn rq_qos_requeue(q: *mut request_queue, rq: *mut request) {
    if test_bit(QUEUE_FLAG_QOS_ENABLED, &(*q).queue_flags) && !(*q).rq_qos.is_null() { __rq_qos_requeue((*q).rq_qos, rq); }
}

pub unsafe fn rq_qos_done_bio(bio: *mut bio) {
    if (*bio).bi_bdev.is_null() || (!bio_flagged(bio, BIO_QOS_THROTTLED) && !bio_flagged(bio, BIO_QOS_MERGED)) { return; }
    let q = bdev_get_queue((*bio).bi_bdev);
    // A BIO may carry QoS flags even when its associated queue has QoS disabled.
    if test_bit(QUEUE_FLAG_QOS_ENABLED, &(*q).queue_flags) && !(*q).rq_qos.is_null() { __rq_qos_done_bio((*q).rq_qos, bio); }
}

pub unsafe fn rq_qos_throttle(q: *mut request_queue, bio: *mut bio) {
    if test_bit(QUEUE_FLAG_QOS_ENABLED, &(*q).queue_flags) && !(*q).rq_qos.is_null() {
        bio_set_flag(bio, BIO_QOS_THROTTLED);
        __rq_qos_throttle((*q).rq_qos, bio);
    }
}

pub unsafe fn rq_qos_track(q: *mut request_queue, rq: *mut request, bio: *mut bio) {
    if test_bit(QUEUE_FLAG_QOS_ENABLED, &(*q).queue_flags) && !(*q).rq_qos.is_null() { __rq_qos_track((*q).rq_qos, rq, bio); }
}

pub unsafe fn rq_qos_merge(q: *mut request_queue, rq: *mut request, bio: *mut bio) {
    if test_bit(QUEUE_FLAG_QOS_ENABLED, &(*q).queue_flags) && !(*q).rq_qos.is_null() {
        bio_set_flag(bio, BIO_QOS_MERGED);
        __rq_qos_merge((*q).rq_qos, rq, bio);
    }
}

pub unsafe fn rq_qos_queue_depth_changed(q: *mut request_queue) {
    if test_bit(QUEUE_FLAG_QOS_ENABLED, &(*q).queue_flags) && !(*q).rq_qos.is_null() { __rq_qos_queue_depth_changed((*q).rq_qos); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
