// SPDX-License-Identifier: GPL-2.0

// Translated from blk-rq-qos.c; declarations supplied by blk-rq-qos.h remain external.

unsafe fn atomic_inc_below(v: *mut atomic_t, below: c_uint) -> bool {
    let mut cur = atomic_read(v);
    loop {
        if cur >= below { return false; }
        if atomic_try_cmpxchg(v, &mut cur, cur.wrapping_add(1)) { break; }
    }
    true
}

pub unsafe fn rq_wait_inc_below(rq_wait: *mut rq_wait, limit: c_uint) -> bool {
    atomic_inc_below(&mut (*rq_wait).inflight, limit)
}

pub unsafe fn __rq_qos_cleanup(mut rqos: *mut rq_qos, bio: *mut bio) {
    loop { if let Some(f) = (*(*rqos).ops).cleanup { f(rqos, bio); } rqos = (*rqos).next; if rqos.is_null() { break; } }
}
pub unsafe fn __rq_qos_done(mut rqos: *mut rq_qos, rq: *mut request) {
    loop { if let Some(f) = (*(*rqos).ops).done { f(rqos, rq); } rqos = (*rqos).next; if rqos.is_null() { break; } }
}
pub unsafe fn __rq_qos_issue(mut rqos: *mut rq_qos, rq: *mut request) {
    loop { if let Some(f) = (*(*rqos).ops).issue { f(rqos, rq); } rqos = (*rqos).next; if rqos.is_null() { break; } }
}
pub unsafe fn __rq_qos_requeue(mut rqos: *mut rq_qos, rq: *mut request) {
    loop { if let Some(f) = (*(*rqos).ops).requeue { f(rqos, rq); } rqos = (*rqos).next; if rqos.is_null() { break; } }
}
pub unsafe fn __rq_qos_throttle(mut rqos: *mut rq_qos, bio: *mut bio) {
    loop { if let Some(f) = (*(*rqos).ops).throttle { f(rqos, bio); } rqos = (*rqos).next; if rqos.is_null() { break; } }
}
pub unsafe fn __rq_qos_track(mut rqos: *mut rq_qos, rq: *mut request, bio: *mut bio) {
    loop { if let Some(f) = (*(*rqos).ops).track { f(rqos, rq, bio); } rqos = (*rqos).next; if rqos.is_null() { break; } }
}
pub unsafe fn __rq_qos_merge(mut rqos: *mut rq_qos, rq: *mut request, bio: *mut bio) {
    loop { if let Some(f) = (*(*rqos).ops).merge { f(rqos, rq, bio); } rqos = (*rqos).next; if rqos.is_null() { break; } }
}
pub unsafe fn __rq_qos_done_bio(mut rqos: *mut rq_qos, bio: *mut bio) {
    loop { if let Some(f) = (*(*rqos).ops).done_bio { f(rqos, bio); } rqos = (*rqos).next; if rqos.is_null() { break; } }
}
pub unsafe fn __rq_qos_queue_depth_changed(mut rqos: *mut rq_qos) {
    loop { if let Some(f) = (*(*rqos).ops).queue_depth_changed { f(rqos); } rqos = (*rqos).next; if rqos.is_null() { break; } }
}

pub unsafe fn rq_depth_calc_max_depth(rqd: *mut rq_depth) -> bool {
    let mut ret = false;
    if (*rqd).queue_depth == 1 {
        if (*rqd).scale_step > 0 { (*rqd).max_depth = 1; }
        else { (*rqd).max_depth = 2; ret = true; }
    } else {
        let mut depth = min_t((*rqd).default_depth, (*rqd).queue_depth);
        if (*rqd).scale_step > 0 { depth = 1 + ((depth - 1) >> min(31, (*rqd).scale_step)); }
        else if (*rqd).scale_step < 0 {
            let maxd = 3 * (*rqd).queue_depth / 4;
            depth = 1 + ((depth - 1) << -(*rqd).scale_step);
            if depth > maxd { depth = maxd; ret = true; }
        }
        (*rqd).max_depth = depth;
    }
    ret
}

pub unsafe fn rq_depth_scale_up(rqd: *mut rq_depth) -> bool {
    if (*rqd).scaled_max { return false; }
    (*rqd).scale_step -= 1;
    (*rqd).scaled_max = rq_depth_calc_max_depth(rqd);
    true
}

pub unsafe fn rq_depth_scale_down(rqd: *mut rq_depth, hard_throttle: bool) -> bool {
    if (*rqd).max_depth == 1 { return false; }
    if (*rqd).scale_step < 0 && hard_throttle { (*rqd).scale_step = 0; }
    else { (*rqd).scale_step += 1; }
    (*rqd).scaled_max = false;
    rq_depth_calc_max_depth(rqd);
    true
}

#[repr(C)]
pub struct rq_qos_wait_data { pub wq: wait_queue_entry, pub rqw: *mut rq_wait, pub cb: acquire_inflight_cb_t, pub private_data: *mut c_void, pub got_token: bool }

unsafe fn rq_qos_wake_function(curr: *mut wait_queue_entry, mode: c_uint, wake_flags: c_int, key: *mut c_void) -> c_int {
    let data = container_of!(curr, rq_qos_wait_data, wq);
    if !((*data).cb)((*data).rqw, (*data).private_data) { return -1; }
    (*data).got_token = true;
    default_wake_function(curr, mode, wake_flags, key);
    list_del_init_careful(&mut (*curr).entry);
    1
}

pub unsafe fn rq_qos_wait(rqw: *mut rq_wait, private_data: *mut c_void, acquire_inflight_cb: acquire_inflight_cb_t, cleanup_cb: cleanup_cb_t) {
    let mut data = rq_qos_wait_data { wq: core::mem::zeroed(), rqw, cb: acquire_inflight_cb, private_data, got_token: false };
    if !waitqueue_active(&mut (*rqw).wait) && acquire_inflight_cb(rqw, private_data) { return; }
    init_wait_func(&mut data.wq, rq_qos_wake_function);
    let first_waiter = prepare_to_wait_exclusive(&mut (*rqw).wait, &mut data.wq, TASK_UNINTERRUPTIBLE);
    if !data.got_token && first_waiter && acquire_inflight_cb(rqw, private_data) {
        finish_wait(&mut (*rqw).wait, &mut data.wq);
        if data.got_token { cleanup_cb(rqw, private_data); }
        return;
    }
    loop { if data.got_token { break; } io_schedule(); set_current_state(TASK_UNINTERRUPTIBLE); }
    finish_wait(&mut (*rqw).wait, &mut data.wq);
}

pub unsafe fn rq_qos_exit(q: *mut request_queue) {
    mutex_lock(&mut (*q).rq_qos_mutex);
    while !(*q).rq_qos.is_null() { let rqos = (*q).rq_qos; (*q).rq_qos = (*rqos).next; ((*(*rqos).ops).exit)(rqos); }
    blk_queue_flag_clear(QUEUE_FLAG_QOS_ENABLED, q); mutex_unlock(&mut (*q).rq_qos_mutex);
}

pub unsafe fn rq_qos_add(rqos: *mut rq_qos, disk: *mut gendisk, id: rq_qos_id, ops: *const rq_qos_ops) -> c_int {
    let q = (*disk).queue; lockdep_assert_held(&mut (*q).rq_qos_mutex);
    (*rqos).disk = disk; (*rqos).id = id; (*rqos).ops = ops;
    let memflags = blk_mq_freeze_queue(q);
    if !rq_qos_id(q, (*rqos).id).is_null() { blk_mq_unfreeze_queue(q, memflags); return -EBUSY; }
    (*rqos).next = (*q).rq_qos; (*q).rq_qos = rqos; blk_queue_flag_set(QUEUE_FLAG_QOS_ENABLED, q);
    blk_mq_unfreeze_queue(q, memflags); 0
}

pub unsafe fn rq_qos_del(rqos: *mut rq_qos) {
    let q = (*(*rqos).disk).queue; lockdep_assert_held(&mut (*q).rq_qos_mutex);
    let memflags = blk_mq_freeze_queue(q); let mut cur = &mut (*q).rq_qos as *mut *mut rq_qos;
    while !(*cur).is_null() { if *cur == rqos { *cur = (*rqos).next; break; } cur = &mut (**cur).next; }
    if (*q).rq_qos.is_null() { blk_queue_flag_clear(QUEUE_FLAG_QOS_ENABLED, q); } blk_mq_unfreeze_queue(q, memflags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
