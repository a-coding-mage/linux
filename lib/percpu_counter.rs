// SPDX-License-Identifier: GPL-2.0
/*
 * Fast batching percpu counters.
 *
 * Direct translation of the Linux kernel implementation.  Kernel-provided
 * types, macros, functions, and per-CPU primitives are external dependencies.
 */

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
static mut PERCPU_COUNTERS: ListHead = ListHead::new();
#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
static mut PERCPU_COUNTERS_LOCK: SpinLock = SpinLock::new();

#[cfg(feature = "CONFIG_DEBUG_OBJECTS_PERCPU_COUNTER")]
static PERCPU_COUNTER_DEBUG_DESCR: DebugObjDescr = DebugObjDescr {
    name: "percpu_counter",
    fixup_free: Some(percpu_counter_fixup_free),
};

#[cfg(feature = "CONFIG_DEBUG_OBJECTS_PERCPU_COUNTER")]
unsafe fn percpu_counter_fixup_free(addr: *mut core::ffi::c_void, state: DebugObjState) -> bool {
    let fbc = addr as *mut PercpuCounter;
    match state {
        DebugObjState::Active => {
            percpu_counter_destroy(fbc);
            debug_object_free(fbc, &PERCPU_COUNTER_DEBUG_DESCR);
            true
        }
        _ => false,
    }
}

#[cfg(feature = "CONFIG_DEBUG_OBJECTS_PERCPU_COUNTER")]
#[inline]
unsafe fn debug_percpu_counter_activate(fbc: *mut PercpuCounter) {
    debug_object_init(fbc, &PERCPU_COUNTER_DEBUG_DESCR);
    debug_object_activate(fbc, &PERCPU_COUNTER_DEBUG_DESCR);
}

#[cfg(feature = "CONFIG_DEBUG_OBJECTS_PERCPU_COUNTER")]
#[inline]
unsafe fn debug_percpu_counter_deactivate(fbc: *mut PercpuCounter) {
    debug_object_deactivate(fbc, &PERCPU_COUNTER_DEBUG_DESCR);
    debug_object_free(fbc, &PERCPU_COUNTER_DEBUG_DESCR);
}

#[cfg(not(feature = "CONFIG_DEBUG_OBJECTS_PERCPU_COUNTER"))]
#[inline]
unsafe fn debug_percpu_counter_activate(_fbc: *mut PercpuCounter) {}

#[cfg(not(feature = "CONFIG_DEBUG_OBJECTS_PERCPU_COUNTER"))]
#[inline]
unsafe fn debug_percpu_counter_deactivate(_fbc: *mut PercpuCounter) {}

pub unsafe fn percpu_counter_set(fbc: *mut PercpuCounter, amount: i64) {
    let flags: c_ulong = 0;
    raw_spin_lock_irqsave(&mut (*fbc).lock, flags);
    for_each_possible_cpu!(cpu, {
        let pcount = per_cpu_ptr((*fbc).counters, cpu);
        *pcount = 0;
    });
    (*fbc).count = amount;
    raw_spin_unlock_irqrestore(&mut (*fbc).lock, flags);
}

#[cfg(feature = "CONFIG_HAVE_CMPXCHG_LOCAL")]
pub unsafe fn percpu_counter_add_batch(fbc: *mut PercpuCounter, amount: i64, batch: i32) {
    let mut count = this_cpu_read!(*(*fbc).counters);
    loop {
        if count.wrapping_add(amount).abs() >= batch as i64 {
            let flags: c_ulong = 0;
            raw_spin_lock_irqsave(&mut (*fbc).lock, flags);
            count = __this_cpu_read!(*(*fbc).counters);
            (*fbc).count = (*fbc).count.wrapping_add(count.wrapping_add(amount));
            __this_cpu_sub!(*(*fbc).counters, count);
            raw_spin_unlock_irqrestore(&mut (*fbc).lock, flags);
            return;
        }
        if this_cpu_try_cmpxchg!(*(*fbc).counters, &mut count, count.wrapping_add(amount)) {
            break;
        }
    }
}

#[cfg(not(feature = "CONFIG_HAVE_CMPXCHG_LOCAL"))]
pub unsafe fn percpu_counter_add_batch(fbc: *mut PercpuCounter, amount: i64, batch: i32) {
    let flags: c_ulong = 0;
    local_irq_save(flags);
    let count = __this_cpu_read!(*(*fbc).counters).wrapping_add(amount);
    if count.abs() >= batch as i64 {
        raw_spin_lock(&mut (*fbc).lock);
        (*fbc).count = (*fbc).count.wrapping_add(count);
        __this_cpu_sub!(*(*fbc).counters, count.wrapping_sub(amount));
        raw_spin_unlock(&mut (*fbc).lock);
    } else {
        this_cpu_add!(*(*fbc).counters, amount);
    }
    local_irq_restore(flags);
}

pub unsafe fn percpu_counter_sync(fbc: *mut PercpuCounter) {
    let flags: c_ulong = 0;
    raw_spin_lock_irqsave(&mut (*fbc).lock, flags);
    let count = __this_cpu_read!(*(*fbc).counters);
    (*fbc).count = (*fbc).count.wrapping_add(count);
    __this_cpu_sub!(*(*fbc).counters, count);
    raw_spin_unlock_irqrestore(&mut (*fbc).lock, flags);
}

pub unsafe fn __percpu_counter_sum(fbc: *mut PercpuCounter) -> i64 {
    let flags: c_ulong = 0;
    raw_spin_lock_irqsave(&mut (*fbc).lock, flags);
    let mut ret = (*fbc).count;
    for_each_cpu_or!(cpu, cpu_online_mask, cpu_dying_mask, {
        ret = ret.wrapping_add(*per_cpu_ptr((*fbc).counters, cpu) as i64);
    });
    raw_spin_unlock_irqrestore(&mut (*fbc).lock, flags);
    ret
}

pub unsafe fn __percpu_counter_init_many(
    fbc: *mut PercpuCounter, amount: i64, gfp: GfpT, nr_counters: u32,
    key: *mut LockClassKey,
) -> i32 {
    let counter_size = align_of::<i32>();
    let counters = __alloc_percpu_gfp(nr_counters as usize * counter_size, align_of::<i32>(), gfp);
    if counters.is_null() {
        (*fbc).counters = core::ptr::null_mut();
        return -12;
    }
    for i in 0..nr_counters {
        let current = fbc.add(i as usize);
        raw_spin_lock_init(&mut (*current).lock);
        lockdep_set_class(&mut (*current).lock, key);
        #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
        init_list_head(&mut (*current).list);
        (*current).count = amount;
        (*current).counters = counters.add(i as usize * counter_size) as *mut i32;
        debug_percpu_counter_activate(current);
    }
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    {
        let flags: c_ulong = 0;
        spin_lock_irqsave(&mut PERCPU_COUNTERS_LOCK, flags);
        for i in 0..nr_counters { list_add(&mut (*fbc.add(i as usize)).list, &mut PERCPU_COUNTERS); }
        spin_unlock_irqrestore(&mut PERCPU_COUNTERS_LOCK, flags);
    }
    0
}

pub unsafe fn percpu_counter_destroy_many(fbc: *mut PercpuCounter, nr_counters: u32) {
    if fbc.is_null() || (*fbc).counters.is_null() { return; }
    for i in 0..nr_counters { debug_percpu_counter_deactivate(fbc.add(i as usize)); }
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    {
        let flags: c_ulong = 0;
        spin_lock_irqsave(&mut PERCPU_COUNTERS_LOCK, flags);
        for i in 0..nr_counters { list_del(&mut (*fbc.add(i as usize)).list); }
        spin_unlock_irqrestore(&mut PERCPU_COUNTERS_LOCK, flags);
    }
    free_percpu((*fbc).counters);
    for i in 0..nr_counters { (*fbc.add(i as usize)).counters = core::ptr::null_mut(); }
}

pub static mut PERCPU_COUNTER_BATCH: i32 = 32;

unsafe fn compute_batch_value(_cpu: u32) -> i32 {
    let nr = num_online_cpus();
    PERCPU_COUNTER_BATCH = core::cmp::max(32, nr * 2);
    0
}

unsafe fn percpu_counter_cpu_dead(cpu: u32) -> i32 {
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    {
        compute_batch_value(cpu);
        spin_lock_irq(&mut PERCPU_COUNTERS_LOCK);
        list_for_each_entry!(fbc, &mut PERCPU_COUNTERS, list, {
            raw_spin_lock(&mut (*fbc).lock);
            let pcount = per_cpu_ptr((*fbc).counters, cpu);
            (*fbc).count = (*fbc).count.wrapping_add(*pcount as i64);
            *pcount = 0;
            raw_spin_unlock(&mut (*fbc).lock);
        });
        spin_unlock_irq(&mut PERCPU_COUNTERS_LOCK);
    }
    0
}

pub unsafe fn __percpu_counter_compare(fbc: *mut PercpuCounter, rhs: i64, batch: i32) -> i32 {
    let count = percpu_counter_read(fbc);
    if (count - rhs).abs() > batch as i64 * num_online_cpus() as i64 { return if count > rhs { 1 } else { -1 }; }
    let count = percpu_counter_sum(fbc);
    if count > rhs { 1 } else if count < rhs { -1 } else { 0 }
}

pub unsafe fn __percpu_counter_limited_add(fbc: *mut PercpuCounter, limit: i64, amount: i64, batch: i32) -> bool {
    if amount == 0 { return true; }
    let flags: c_ulong = 0;
    local_irq_save(flags);
    let unknown = batch as i64 * num_online_cpus() as i64;
    let count = __this_cpu_read!(*(*fbc).counters);
    if (count + amount).abs() <= batch as i64 && ((amount > 0 && (*fbc).count + unknown <= limit) || (amount < 0 && (*fbc).count - unknown >= limit)) {
        this_cpu_add!(*(*fbc).counters, amount);
        local_irq_restore(flags);
        return true;
    }
    raw_spin_lock(&mut (*fbc).lock);
    let mut count = (*fbc).count + amount;
    let mut good = false;
    if amount > 0 {
        if count - unknown > limit { raw_spin_unlock(&mut (*fbc).lock); local_irq_restore(flags); return false; }
        if count + unknown <= limit { good = true; }
    } else {
        if count + unknown < limit { raw_spin_unlock(&mut (*fbc).lock); local_irq_restore(flags); return false; }
        if count - unknown >= limit { good = true; }
    }
    if !good {
        for_each_cpu_or!(cpu, cpu_online_mask, cpu_dying_mask, { count += *per_cpu_ptr((*fbc).counters, cpu) as i64; });
        if (amount > 0 && count > limit) || (amount < 0 && count < limit) { raw_spin_unlock(&mut (*fbc).lock); local_irq_restore(flags); return false; }
        good = true;
    }
    let local_count = __this_cpu_read!(*(*fbc).counters);
    (*fbc).count += local_count + amount;
    __this_cpu_sub!(*(*fbc).counters, local_count);
    raw_spin_unlock(&mut (*fbc).lock);
    local_irq_restore(flags);
    good
}

unsafe fn percpu_counter_startup() -> i32 {
    let ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "lib/percpu_cnt:online", compute_batch_value, None);
    warn_on(ret < 0);
    let ret = cpuhp_setup_state_nocalls(CPUHP_PERCPU_CNT_DEAD, "lib/percpu_cnt:dead", None, Some(percpu_counter_cpu_dead));
    warn_on(ret < 0);
    0
}

// module_init!(percpu_counter_startup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
