// SPDX-License-Identifier: GPL-2.0-only
// C kernel dependencies and build-time macros are supplied by the surrounding
// translation environment.

const PERCPU_COUNT_BIAS: c_ulong = 1u64 << (BITS_PER_LONG - 1);

static mut percpu_ref_switch_lock: spinlock_t = DEFINE_SPINLOCK!();
static mut percpu_ref_switch_waitq: wait_queue_head_t = DECLARE_WAIT_QUEUE_HEAD!();

unsafe fn percpu_count_ptr(ref_: *mut percpu_ref) -> *mut c_ulong {
    ((*ref_).percpu_count_ptr & !__PERCPU_REF_ATOMIC_DEAD) as *mut c_ulong
}

pub unsafe fn percpu_ref_init(ref_: *mut percpu_ref, release: percpu_ref_func_t,
                              flags: c_uint, gfp: gfp_t) -> c_int {
    let align = max_t!(usize, 1usize << __PERCPU_REF_FLAG_BITS, align_of::<c_ulong>());
    let mut start_count: c_ulong = 0;
    let data: *mut percpu_ref_data;

    (*ref_).percpu_count_ptr = __alloc_percpu_gfp(size_of::<c_ulong>(), align, gfp) as c_ulong;
    if (*ref_).percpu_count_ptr == 0 { return -ENOMEM; }

    data = kzalloc_obj!((*ref_).data, gfp);
    if data.is_null() {
        free_percpu((*ref_).percpu_count_ptr as *mut c_void);
        (*ref_).percpu_count_ptr = 0;
        return -ENOMEM;
    }

    (*data).force_atomic = flags & PERCPU_REF_INIT_ATOMIC;
    (*data).allow_reinit = flags & PERCPU_REF_ALLOW_REINIT;
    if flags & (PERCPU_REF_INIT_ATOMIC | PERCPU_REF_INIT_DEAD) != 0 {
        (*ref_).percpu_count_ptr |= __PERCPU_REF_ATOMIC;
        (*data).allow_reinit = true;
    } else { start_count = start_count.wrapping_add(PERCPU_COUNT_BIAS); }
    if flags & PERCPU_REF_INIT_DEAD != 0 {
        (*ref_).percpu_count_ptr |= __PERCPU_REF_DEAD;
    } else { start_count = start_count.wrapping_add(1); }

    atomic_long_set(&mut (*data).count, start_count);
    (*data).release = release;
    (*data).confirm_switch = None;
    (*data).ref_ = ref_;
    (*ref_).data = data;
    0
}

unsafe fn __percpu_ref_exit(ref_: *mut percpu_ref) {
    let percpu_count = percpu_count_ptr(ref_);
    if !percpu_count.is_null() {
        WARN_ON_ONCE!(!(*ref_).data.is_null() && !(*(*ref_).data).confirm_switch.is_none());
        free_percpu(percpu_count as *mut c_void);
        (*ref_).percpu_count_ptr = __PERCPU_REF_ATOMIC_DEAD;
    }
}

pub unsafe fn percpu_ref_exit(ref_: *mut percpu_ref) {
    let data = (*ref_).data;
    let mut flags = 0;
    __percpu_ref_exit(ref_);
    if data.is_null() { return; }
    spin_lock_irqsave(&mut percpu_ref_switch_lock, &mut flags);
    (*ref_).percpu_count_ptr |= atomic_long_read(&(*(*ref_).data).count) << __PERCPU_REF_FLAG_BITS;
    (*ref_).data = core::ptr::null_mut();
    spin_unlock_irqrestore(&mut percpu_ref_switch_lock, flags);
    kfree(data as *mut c_void);
}

unsafe fn percpu_ref_call_confirm_rcu(rcu: *mut rcu_head) {
    let data = container_of!(rcu, percpu_ref_data, rcu);
    let ref_ = (*data).ref_;
    if let Some(confirm) = (*data).confirm_switch { confirm(ref_); }
    (*data).confirm_switch = None;
    wake_up_all(&mut percpu_ref_switch_waitq);
    if !(*data).allow_reinit { __percpu_ref_exit(ref_); }
    percpu_ref_put(ref_);
}

unsafe fn percpu_ref_switch_to_atomic_rcu(rcu: *mut rcu_head) {
    let data = container_of!(rcu, percpu_ref_data, rcu);
    let ref_ = (*data).ref_;
    let percpu_count = percpu_count_ptr(ref_);
    static mut underflows: atomic_t = ATOMIC_INIT!(0);
    let mut count: c_ulong = 0;
    for_each_possible_cpu!(cpu) { count = count.wrapping_add(*per_cpu_ptr(percpu_count, cpu)); }
    pr_debug!("global %lu percpu %lu\n", atomic_long_read(&(*data).count), count);
    atomic_long_add((count as c_long).wrapping_sub(PERCPU_COUNT_BIAS as c_long), &mut (*data).count);
    if WARN_ONCE!(atomic_long_read(&(*data).count) <= 0,
                  "percpu ref (%ps) <= 0 (%ld) after switching to atomic",
                  (*data).release, atomic_long_read(&(*data).count)) &&
       atomic_inc_return(&mut underflows) < 4 {
        pr_err!("%s(): percpu_ref underflow", __func__());
        mem_dump_obj(data as *mut c_void);
    }
    percpu_ref_call_confirm_rcu(rcu);
}

unsafe fn percpu_ref_noop_confirm_switch(_ref_: *mut percpu_ref) {}

unsafe fn __percpu_ref_switch_to_atomic(ref_: *mut percpu_ref, confirm_switch: percpu_ref_func_t) {
    if (*ref_).percpu_count_ptr & __PERCPU_REF_ATOMIC != 0 {
        if let Some(confirm) = confirm_switch { confirm(ref_); }
        return;
    }
    (*ref_).percpu_count_ptr |= __PERCPU_REF_ATOMIC;
    (*(*ref_).data).confirm_switch = confirm_switch.or(Some(percpu_ref_noop_confirm_switch));
    percpu_ref_get(ref_);
    call_rcu_hurry!(&mut (*(*ref_).data).rcu, percpu_ref_switch_to_atomic_rcu);
}

unsafe fn __percpu_ref_switch_to_percpu(ref_: *mut percpu_ref) {
    let percpu_count = percpu_count_ptr(ref_);
    BUG_ON!(percpu_count.is_null());
    if (*ref_).percpu_count_ptr & __PERCPU_REF_ATOMIC == 0 { return; }
    if WARN_ON_ONCE!(!(*(*ref_).data).allow_reinit) { return; }
    atomic_long_add(PERCPU_COUNT_BIAS as c_long, &mut (*(*ref_).data).count);
    for_each_possible_cpu!(cpu) { *per_cpu_ptr(percpu_count, cpu) = 0; }
    smp_store_release!(&mut (*ref_).percpu_count_ptr,
                       (*ref_).percpu_count_ptr & !__PERCPU_REF_ATOMIC);
}

unsafe fn __percpu_ref_switch_mode(ref_: *mut percpu_ref, confirm_switch: percpu_ref_func_t) {
    let data = (*ref_).data;
    lockdep_assert_held!(&percpu_ref_switch_lock);
    wait_event_lock_irq!(percpu_ref_switch_waitq, (*data).confirm_switch.is_none(), percpu_ref_switch_lock);
    if (*data).force_atomic || percpu_ref_is_dying(ref_) { __percpu_ref_switch_to_atomic(ref_, confirm_switch); }
    else { __percpu_ref_switch_to_percpu(ref_); }
}

pub unsafe fn percpu_ref_switch_to_atomic(ref_: *mut percpu_ref, confirm_switch: percpu_ref_func_t) {
    let mut flags = 0;
    spin_lock_irqsave(&mut percpu_ref_switch_lock, &mut flags);
    (*(*ref_).data).force_atomic = true;
    __percpu_ref_switch_mode(ref_, confirm_switch);
    spin_unlock_irqrestore(&mut percpu_ref_switch_lock, flags);
}

pub unsafe fn percpu_ref_switch_to_atomic_sync(ref_: *mut percpu_ref) {
    percpu_ref_switch_to_atomic(ref_, None);
    wait_event!(percpu_ref_switch_waitq, (*(*ref_).data).confirm_switch.is_none());
}

pub unsafe fn percpu_ref_switch_to_percpu(ref_: *mut percpu_ref) {
    let mut flags = 0;
    spin_lock_irqsave(&mut percpu_ref_switch_lock, &mut flags);
    (*(*ref_).data).force_atomic = false;
    __percpu_ref_switch_mode(ref_, None);
    spin_unlock_irqrestore(&mut percpu_ref_switch_lock, flags);
}

pub unsafe fn percpu_ref_kill_and_confirm(ref_: *mut percpu_ref, confirm_kill: percpu_ref_func_t) {
    let mut flags = 0;
    spin_lock_irqsave(&mut percpu_ref_switch_lock, &mut flags);
    WARN_ONCE!(percpu_ref_is_dying(ref_), "%s called more than once on %ps!", __func!(), (*(*ref_).data).release);
    (*ref_).percpu_count_ptr |= __PERCPU_REF_DEAD;
    __percpu_ref_switch_mode(ref_, confirm_kill);
    percpu_ref_put(ref_);
    spin_unlock_irqrestore(&mut percpu_ref_switch_lock, flags);
}

pub unsafe fn percpu_ref_is_zero(ref_: *mut percpu_ref) -> bool {
    let mut percpu_count: *mut c_ulong = core::ptr::null_mut();
    let mut count;
    let mut flags = 0;
    if __ref_is_percpu(ref_, &mut percpu_count) { return false; }
    spin_lock_irqsave(&mut percpu_ref_switch_lock, &mut flags);
    if !(*ref_).data.is_null() { count = atomic_long_read(&(*(*ref_).data).count); }
    else { count = (*ref_).percpu_count_ptr >> __PERCPU_REF_FLAG_BITS; }
    spin_unlock_irqrestore(&mut percpu_ref_switch_lock, flags);
    count == 0
}

pub unsafe fn percpu_ref_reinit(ref_: *mut percpu_ref) {
    WARN_ON_ONCE!(!percpu_ref_is_zero(ref_));
    percpu_ref_resurrect(ref_);
}

pub unsafe fn percpu_ref_resurrect(ref_: *mut percpu_ref) {
    let mut percpu_count: *mut c_ulong = core::ptr::null_mut();
    let mut flags = 0;
    spin_lock_irqsave(&mut percpu_ref_switch_lock, &mut flags);
    WARN_ON_ONCE!(!percpu_ref_is_dying(ref_));
    WARN_ON_ONCE!(__ref_is_percpu(ref_, &mut percpu_count));
    (*ref_).percpu_count_ptr &= !__PERCPU_REF_DEAD;
    percpu_ref_get(ref_);
    __percpu_ref_switch_mode(ref_, None);
    spin_unlock_irqrestore(&mut percpu_ref_switch_lock, flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
