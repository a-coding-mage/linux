// SPDX-License-Identifier: GPL-2.0-only
// core.c:11224..11321; configured native sched_change_ctx owns callback types.
#[no_mangle]
pub unsafe extern "C" fn sched_change_begin(
    p: *mut task_struct,
    mut flags: c_uint,
) -> *mut sched_change_ctx {
    // SAFETY: The caller keeps p live, holds its rq lock, and does not nest
    // sched_change operations on this CPU; the per-CPU context remains valid.
    unsafe {
        let ctx = lupos_core_this_sched_change_ctx();
        let rq = lupos_core_task_rq(p);
        lupos_core_warn_once_site_11266(flags & 0xFFFF0000 != 0);
        lupos_core_assert_rq_held(rq);
        if flags & LUPOS_CORE_DEQUEUE_NOCLOCK as c_uint == 0 {
            update_rq_clock(rq);
            flags |= LUPOS_CORE_DEQUEUE_NOCLOCK as c_uint;
        }
        if flags & LUPOS_CORE_DEQUEUE_CLASS as c_uint != 0 {
            if let Some(cb) = (*(*p).sched_class).switching_from {
                cb(rq, p);
            }
        }
        // C compound literal zeroes unspecified members (including prio).
        core::ptr::write_bytes(ctx, 0, 1);
        (*ctx).p = p;
        (*ctx).class = (*p).sched_class;
        (*ctx).flags = flags as c_int;
        (*ctx).queued = lupos_core_task_on_rq_queued(p);
        (*ctx).running = lupos_core_task_current_donor(rq, p);
        if flags & LUPOS_CORE_DEQUEUE_CLASS as c_uint == 0 {
            (*ctx).prio = if let Some(cb) = (*(*p).sched_class).get_prio {
                cb(rq, p)
            } else {
                (*p).prio as u64
            };
        }
        if (*ctx).queued {
            dequeue_task(rq, p, flags as c_int);
        }
        if (*ctx).running {
            lupos_core_put_prev_task(rq, p);
        }
        if flags & LUPOS_CORE_DEQUEUE_CLASS as c_uint != 0 {
            if let Some(cb) = (*(*p).sched_class).switched_from {
                cb(rq, p);
            }
        }
        ctx
    }
}
#[no_mangle]
pub unsafe extern "C" fn sched_change_end(ctx: *mut sched_change_ctx) {
    // SAFETY: ctx is the unmatched context from sched_change_begin on this
    // CPU. The caller still holds the task's rq lock across class callbacks.
    unsafe {
        let p = (*ctx).p;
        let rq = lupos_core_task_rq(p);
        lupos_core_assert_rq_held(rq);
        lupos_core_warn_once_site_11301(
            (*p).sched_class != (*ctx).class && (*ctx).flags & LUPOS_CORE_ENQUEUE_CLASS == 0,
        );
        if (*ctx).flags & LUPOS_CORE_ENQUEUE_CLASS != 0 {
            if let Some(cb) = (*(*p).sched_class).switching_to {
                cb(rq, p);
            }
        }
        if (*ctx).queued {
            enqueue_task(rq, p, (*ctx).flags as c_int);
        }
        if (*ctx).running {
            lupos_core_set_next_task(rq, p);
        }
        if (*ctx).flags & LUPOS_CORE_ENQUEUE_CLASS != 0 {
            if let Some(cb) = (*(*p).sched_class).switched_to {
                cb(rq, p);
            }
            if (*ctx).running {
                if lupos_core_sched_class_above((*p).sched_class, (*ctx).class) {
                    (*(*rq).next_class).wakeup_preempt.unwrap()(rq, p, 0);
                    (*rq).next_class = (*p).sched_class;
                }
                if lupos_core_sched_class_above((*ctx).class, (*p).sched_class) {
                    resched_curr(rq);
                }
            }
        } else {
            (*(*p).sched_class).prio_changed.unwrap()(rq, p, (*ctx).prio);
        }
    }
}
