// SPDX-License-Identifier: GPL-2.0
/*
 * Block stat tracking code
 *
 * Copyright (C) 2016 Jens Axboe
 */
// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

#[repr(C)]
pub struct blk_queue_stats {
    pub callbacks: list_head,
    pub lock: spinlock_t,
    pub accounting: i32,
}

pub unsafe fn blk_rq_stat_init(stat: *mut blk_rq_stat) {
    (*stat).min = u64::MAX;
    (*stat).max = 0;
    (*stat).nr_samples = 0;
    (*stat).mean = 0;
    (*stat).batch = 0;
}

/* src is a per-cpu stat, mean isn't initialized */
pub unsafe fn blk_rq_stat_sum(dst: *mut blk_rq_stat, src: *mut blk_rq_stat) {
    if (*dst).nr_samples.wrapping_add((*src).nr_samples) <= (*dst).nr_samples {
        return;
    }

    (*dst).min = core::cmp::min((*dst).min, (*src).min);
    (*dst).max = core::cmp::max((*dst).max, (*src).max);
    (*dst).mean = div_u64(
        (*src).batch
            .wrapping_add((*dst).mean.wrapping_mul((*dst).nr_samples)),
        (*dst).nr_samples.wrapping_add((*src).nr_samples),
    );
    (*dst).nr_samples = (*dst).nr_samples.wrapping_add((*src).nr_samples);
}

pub unsafe fn blk_rq_stat_add(stat: *mut blk_rq_stat, value: u64) {
    (*stat).min = core::cmp::min((*stat).min, value);
    (*stat).max = core::cmp::max((*stat).max, value);
    (*stat).batch = (*stat).batch.wrapping_add(value);
    (*stat).nr_samples = (*stat).nr_samples.wrapping_add(1);
}

pub unsafe fn blk_stat_add(rq: *mut request, now: u64) {
    let q = (*rq).q;
    let mut cb: *mut blk_stat_callback;
    let mut stat: *mut blk_rq_stat;
    let mut bucket: i32;
    let cpu: i32;
    let value = if now >= (*rq).io_start_time_ns {
        now - (*rq).io_start_time_ns
    } else {
        0
    };

    rcu_read_lock();
    cpu = get_cpu();
    list_for_each_entry_rcu!(cb, &mut (*(*q).stats).callbacks, list);
    {
        if !blk_stat_is_active(cb) {
            continue;
        }
        bucket = ((*cb).bucket_fn)(rq);
        if bucket < 0 {
            continue;
        }
        stat = &mut *per_cpu_ptr((*cb).cpu_stat, cpu).add(bucket as usize);
        blk_rq_stat_add(stat, value);
    }
    put_cpu();
    rcu_read_unlock();
}

unsafe fn blk_stat_timer_fn(t: *mut timer_list) {
    let cb = timer_container_of!(t, blk_stat_callback, timer);
    let mut bucket: u32;
    let mut cpu: i32;

    bucket = 0;
    while bucket < (*cb).buckets {
        blk_rq_stat_init((*cb).stat.add(bucket as usize));
        bucket += 1;
    }

    for_each_possible_cpu!(cpu);
    {
        let cpu_stat = per_cpu_ptr((*cb).cpu_stat, cpu);
        bucket = 0;
        while bucket < (*cb).buckets {
            blk_rq_stat_sum((*cb).stat.add(bucket as usize), cpu_stat.add(bucket as usize));
            blk_rq_stat_init(cpu_stat.add(bucket as usize));
            bucket += 1;
        }
    }
    ((*cb).timer_fn)(cb);
}

pub unsafe fn blk_stat_alloc_callback(
    timer_fn: Option<unsafe extern "C" fn(*mut blk_stat_callback)>,
    bucket_fn: Option<unsafe extern "C" fn(*const request) -> i32>,
    buckets: u32,
    data: *mut core::ffi::c_void,
) -> *mut blk_stat_callback {
    let cb = kmalloc_obj::<blk_stat_callback>();
    if cb.is_null() { return core::ptr::null_mut(); }
    (*cb).stat = kmalloc_objs::<blk_rq_stat>(buckets);
    if (*cb).stat.is_null() { kfree(cb); return core::ptr::null_mut(); }
    (*cb).cpu_stat = __alloc_percpu(
        buckets as usize * core::mem::size_of::<blk_rq_stat>(),
        core::mem::align_of::<blk_rq_stat>(),
    );
    if (*cb).cpu_stat.is_null() {
        kfree((*cb).stat); kfree(cb); return core::ptr::null_mut();
    }
    (*cb).timer_fn = timer_fn;
    (*cb).bucket_fn = bucket_fn;
    (*cb).data = data;
    (*cb).buckets = buckets;
    timer_setup!(&mut (*cb).timer, blk_stat_timer_fn, 0);
    cb
}

pub unsafe fn blk_stat_add_callback(q: *mut request_queue, cb: *mut blk_stat_callback) {
    let mut cpu: i32;
    for_each_possible_cpu!(cpu);
    {
        let cpu_stat = per_cpu_ptr((*cb).cpu_stat, cpu);
        let mut bucket = 0;
        while bucket < (*cb).buckets {
            blk_rq_stat_init(cpu_stat.add(bucket as usize));
            bucket += 1;
        }
    }
    let mut flags: usize = 0;
    spin_lock_irqsave!(&mut (*(*q).stats).lock, &mut flags);
    list_add_tail_rcu!(&mut (*cb).list, &mut (*(*q).stats).callbacks);
    blk_queue_flag_set(QUEUE_FLAG_STATS, q);
    spin_unlock_irqrestore!(&mut (*(*q).stats).lock, flags);
}

pub unsafe fn blk_stat_remove_callback(q: *mut request_queue, cb: *mut blk_stat_callback) {
    let mut flags: usize = 0;
    spin_lock_irqsave!(&mut (*(*q).stats).lock, &mut flags);
    list_del_rcu!(&mut (*cb).list);
    if list_empty!(&(*(*q).stats).callbacks) && (*(*q).stats).accounting == 0 {
        blk_queue_flag_clear(QUEUE_FLAG_STATS, q);
    }
    spin_unlock_irqrestore!(&mut (*(*q).stats).lock, flags);
    timer_delete_sync!(&mut (*cb).timer);
}

unsafe fn blk_stat_free_callback_rcu(head: *mut rcu_head) {
    let cb = container_of!(head, blk_stat_callback, rcu);
    free_percpu((*cb).cpu_stat);
    kfree((*cb).stat);
    kfree(cb);
}

pub unsafe fn blk_stat_free_callback(cb: *mut blk_stat_callback) {
    if !cb.is_null() { call_rcu!(&mut (*cb).rcu, blk_stat_free_callback_rcu); }
}

pub unsafe fn blk_stat_disable_accounting(q: *mut request_queue) {
    let mut flags: usize = 0;
    spin_lock_irqsave!(&mut (*(*q).stats).lock, &mut flags);
    (*(*q).stats).accounting -= 1;
    if (*(*q).stats).accounting == 0 && list_empty!(&(*(*q).stats).callbacks) {
        blk_queue_flag_clear(QUEUE_FLAG_STATS, q);
    }
    spin_unlock_irqrestore!(&mut (*(*q).stats).lock, flags);
}

pub unsafe fn blk_stat_enable_accounting(q: *mut request_queue) {
    let mut flags: usize = 0;
    spin_lock_irqsave!(&mut (*(*q).stats).lock, &mut flags);
    let was_zero = (*(*q).stats).accounting == 0;
    (*(*q).stats).accounting += 1;
    if was_zero && list_empty!(&(*(*q).stats).callbacks) {
        blk_queue_flag_set(QUEUE_FLAG_STATS, q);
    }
    spin_unlock_irqrestore!(&mut (*(*q).stats).lock, flags);
}

pub unsafe fn blk_alloc_queue_stats() -> *mut blk_queue_stats {
    let stats = kmalloc_obj::<blk_queue_stats>();
    if stats.is_null() { return core::ptr::null_mut(); }
    INIT_LIST_HEAD!(&mut (*stats).callbacks);
    spin_lock_init!(&mut (*stats).lock);
    (*stats).accounting = 0;
    stats
}

pub unsafe fn blk_free_queue_stats(stats: *mut blk_queue_stats) {
    if stats.is_null() { return; }
    WARN_ON!(!list_empty!(&(*stats).callbacks));
    kfree(stats);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
