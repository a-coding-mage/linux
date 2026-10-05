// SPDX-License-Identifier: GPL-2.0
/* CPUFreq governor based on scheduler-provided CPU utilization data.
 * Existing Rust continued against native commit
 * 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. The native private header owns
 * sugov storage layouts; no guessed Rust policy, lock or per-CPU layouts.
 */
#[cfg(CONFIG_RUST_SCHED_CPUFREQ_SCHEDUTIL)]
compile_error!("SOURCE ONLY HOLD: scheduler schedutil is not admitted");

use core::ptr::{addr_of_mut, null_mut};
use kernel::bindings::sched_cpufreq_native::*;
use kernel::ffi::{c_char, c_int, c_uint, c_ulong};

const IOWAIT_BOOST_MIN: c_uint = LUPOS_SUGOV_IOWAIT_BOOST_MIN;

// All objects below are native allocations or native per-CPU storage. Field
// access uses raw pointers rather than Rust references to concurrently shared
// kernel state. Scheduler rq locks/update_lock and governor/sysfs serialization
// retain the oracle's ownership contracts; limits_changed stays READ/WRITE_ONCE.
unsafe fn sugov_update_rate_limit_us(p: *mut sugov_policy) {
    unsafe {
        (*p).freq_update_delay_ns =
            ((*(*p).tunables).rate_limit_us as i64) * LUPOS_SUGOV_NSEC_PER_USEC;
    }
}

unsafe fn sugov_should_update_freq(p: *mut sugov_policy, time: u64) -> bool {
    unsafe {
        if !cpufreq_this_cpu_can_update((*p).policy) {
            return false;
        }
        if lupos_sugov_limits_read(p) {
            lupos_sugov_limits_write(p, false);
            (*p).need_freq_update = true;
            // Paired with sugov_limits before resolving policy limits.
            lupos_sugov_full_barrier();
            return true;
        } else if (*p).need_freq_update {
            return true;
        }
        (time.wrapping_sub((*p).last_freq_update_time) as i64) >= (*p).freq_update_delay_ns
    }
}

unsafe fn sugov_update_next_freq(p: *mut sugov_policy, time: u64, next: c_uint) -> bool {
    unsafe {
        if (*p).need_freq_update {
            (*p).need_freq_update = false;
            if (*p).next_freq == next
                && !cpufreq_driver_test_flags(LUPOS_SUGOV_NEED_UPDATE_LIMITS)
            {
                return false;
            }
        } else if (*p).next_freq == next {
            return false;
        }
        (*p).next_freq = next;
        (*p).last_freq_update_time = time;
        true
    }
}

unsafe fn sugov_deferred_update(p: *mut sugov_policy) {
    unsafe {
        if !(*p).work_in_progress {
            (*p).work_in_progress = true;
            lupos_sugov_queue_irq(p);
        }
    }
}

unsafe fn get_capacity_ref_freq(policy: *mut cpufreq_policy) -> c_ulong {
    unsafe {
        let freq = lupos_sugov_arch_ref_freq(lupos_sugov_policy_cpu(policy));
        if freq != 0 {
            return freq as c_ulong;
        }
        if lupos_sugov_arch_invariant() {
            return lupos_sugov_policy_max_freq(policy) as c_ulong;
        }
        let cur = lupos_sugov_policy_cur(policy);
        // C adds unsigned int operands before conversion to unsigned long.
        cur.wrapping_add(cur >> 2) as c_ulong
    }
}

unsafe fn get_next_freq(p: *mut sugov_policy, util: c_ulong, max: c_ulong) -> c_uint {
    unsafe {
        let policy = (*p).policy;
        // Preserve each unsigned-int assignment in the native body, including
        // narrowing the reference frequency before map_util_freq.
        let freq = get_capacity_ref_freq(policy) as c_uint;
        let freq = lupos_sugov_map_freq(util, freq as c_ulong, max) as c_uint;
        if freq == (*p).cached_raw_freq && !(*p).need_freq_update {
            return (*p).next_freq;
        }
        (*p).cached_raw_freq = freq;
        cpufreq_driver_resolve_freq(policy, freq)
    }
}

/// Map actual CPU utilization to the bounded scheduler performance target.
///
/// # Safety
/// Caller supplies values in the native scheduler capacity scale.
#[no_mangle]
pub unsafe extern "C" fn sugov_effective_cpu_perf(
    _cpu: c_int,
    mut actual: c_ulong,
    min: c_ulong,
    mut max: c_ulong,
) -> c_ulong {
    unsafe {
        actual = lupos_sugov_map_perf(actual);
        if actual < max {
            max = actual;
        }
        min.max(max)
    }
}

unsafe fn sugov_get_util(c: *mut sugov_cpu, boost: c_ulong) {
    unsafe {
        let mut min = 0;
        let mut max = 0;
        let mut util = lupos_sugov_scx_target((*c).cpu);
        if !lupos_sugov_scx_all() {
            util = util.wrapping_add(cpu_util_cfs_boost((*c).cpu as c_int));
        }
        util = effective_cpu_util((*c).cpu as c_int, util, &mut min, &mut max);
        util = util.max(boost);
        (*c).bw_min = min;
        (*c).bw_max = max;
        (*c).util = sugov_effective_cpu_perf((*c).cpu as c_int, util, min, max);
    }
}

unsafe fn sugov_iowait_reset(c: *mut sugov_cpu, time: u64, set: bool) -> bool {
    unsafe {
        if (time.wrapping_sub((*c).last_update) as i64) <= LUPOS_SUGOV_TICK_NSEC {
            return false;
        }
        (*c).iowait_boost = if set { IOWAIT_BOOST_MIN } else { 0 };
        (*c).iowait_boost_pending = set;
        true
    }
}

unsafe fn sugov_iowait_boost(c: *mut sugov_cpu, time: u64, flags: c_uint) {
    unsafe {
        let set = flags & LUPOS_SUGOV_IOWAIT != 0;
        if (*c).iowait_boost != 0 && sugov_iowait_reset(c, time, set) {
            return;
        }
        if !set || (*c).iowait_boost_pending {
            return;
        }
        (*c).iowait_boost_pending = true;
        if (*c).iowait_boost != 0 {
            (*c).iowait_boost = ((*c).iowait_boost << 1).min(LUPOS_SUGOV_CAPACITY_SCALE);
        } else {
            (*c).iowait_boost = IOWAIT_BOOST_MIN;
        }
    }
}

unsafe fn sugov_iowait_apply(c: *mut sugov_cpu, time: u64, max_cap: c_ulong) -> c_ulong {
    unsafe {
        if (*c).iowait_boost == 0 || sugov_iowait_reset(c, time, false) {
            return 0;
        }
        if !(*c).iowait_boost_pending {
            (*c).iowait_boost >>= 1;
            if (*c).iowait_boost < IOWAIT_BOOST_MIN {
                (*c).iowait_boost = 0;
                return 0;
            }
        }
        (*c).iowait_boost_pending = false;
        ((*c).iowait_boost as c_ulong).wrapping_mul(max_cap) >> LUPOS_SUGOV_CAPACITY_SHIFT
    }
}

#[cfg(CONFIG_NO_HZ_COMMON)]
unsafe fn sugov_hold_freq(c: *mut sugov_cpu) -> bool {
    unsafe {
        if lupos_sugov_scx_all() || lupos_sugov_capped((*c).cpu) {
            return false;
        }
        let calls = lupos_sugov_idle_calls((*c).cpu);
        let ret = calls == (*c).saved_idle_calls;
        (*c).saved_idle_calls = calls;
        ret
    }
}

#[cfg(not(CONFIG_NO_HZ_COMMON))]
unsafe fn sugov_hold_freq(_c: *mut sugov_cpu) -> bool {
    false
}

unsafe fn ignore_dl_rate_limit(c: *mut sugov_cpu) {
    unsafe {
        if lupos_sugov_dl_bw((*c).cpu) > (*c).bw_min {
            (*(*c).sg_policy).need_freq_update = true;
        }
    }
}

unsafe fn sugov_update_single_common(
    c: *mut sugov_cpu,
    time: u64,
    max_cap: c_ulong,
    flags: c_uint,
) -> bool {
    unsafe {
        sugov_iowait_boost(c, time, flags);
        (*c).last_update = time;
        ignore_dl_rate_limit(c);
        if !sugov_should_update_freq((*c).sg_policy, time) {
            return false;
        }
        let boost = sugov_iowait_apply(c, time, max_cap);
        sugov_get_util(c, boost);
        true
    }
}

/// Update a single-CPU policy from its RCU-published scheduler hook.
/// # Safety
/// Hook data is live under the native rq lock and RCU read-side contract.
#[export_name = "lupos_sugov_update_single_freq"]
pub unsafe extern "C" fn sugov_update_single_freq(
    hook: *mut update_util_data,
    time: u64,
    flags: c_uint,
) {
    unsafe {
        let c = lupos_sugov_cpu_from_hook(hook);
        let p = (*c).sg_policy;
        let cached = (*p).cached_raw_freq;
        let max = lupos_sugov_arch_capacity((*c).cpu);
        if !sugov_update_single_common(c, time, max, flags) {
            return;
        }
        let mut next = get_next_freq(p, (*c).util, max);
        if sugov_hold_freq(c) && next < (*p).next_freq && !(*p).need_freq_update {
            next = (*p).next_freq;
            (*p).cached_raw_freq = cached;
        }
        if !sugov_update_next_freq(p, time, next) {
            return;
        }
        if lupos_sugov_policy_fast((*p).policy) {
            cpufreq_driver_fast_switch((*p).policy, next);
        } else {
            lupos_sugov_update_lock(p);
            sugov_deferred_update(p);
            lupos_sugov_update_unlock(p);
        }
    }
}

/// Update direct performance for a single-CPU policy.
/// # Safety
/// Hook data is live under the native rq lock and RCU read-side contract.
#[export_name = "lupos_sugov_update_single_perf"]
pub unsafe extern "C" fn sugov_update_single_perf(
    hook: *mut update_util_data,
    time: u64,
    flags: c_uint,
) {
    unsafe {
        let c = lupos_sugov_cpu_from_hook(hook);
        let p = (*c).sg_policy;
        let prev = (*c).util;
        if !lupos_sugov_arch_invariant() {
            sugov_update_single_freq(hook, time, flags);
            return;
        }
        let max = lupos_sugov_arch_capacity((*c).cpu);
        if !sugov_update_single_common(c, time, max, flags) {
            return;
        }
        if sugov_hold_freq(c) && (*c).util < prev {
            (*c).util = prev;
        }
        cpufreq_driver_adjust_perf((*p).policy, (*c).bw_min, (*c).util, (*c).bw_max, max);
        (*p).need_freq_update = false;
        (*p).last_freq_update_time = time;
    }
}

unsafe fn sugov_next_freq_shared(c: *mut sugov_cpu, time: u64) -> c_uint {
    unsafe {
        let p = (*c).sg_policy;
        let policy = (*p).policy;
        let max = lupos_sugov_arch_capacity((*c).cpu);
        let mut util = 0;
        let mut cpu = lupos_sugov_first_cpu(policy);
        while lupos_sugov_valid_cpu(cpu) {
            let sibling = lupos_sugov_cpu_storage(cpu);
            let boost = sugov_iowait_apply(sibling, time, max);
            sugov_get_util(sibling, boost);
            util = util.max((*sibling).util);
            cpu = lupos_sugov_next_cpu(policy, cpu);
        }
        get_next_freq(p, util, max)
    }
}

/// Serialize a shared policy update across its sibling CPU hooks.
/// # Safety
/// Hook data is live under the native rq lock and RCU read-side contract.
#[export_name = "lupos_sugov_update_shared"]
pub unsafe extern "C" fn sugov_update_shared(
    hook: *mut update_util_data,
    time: u64,
    flags: c_uint,
) {
    unsafe {
        let c = lupos_sugov_cpu_from_hook(hook);
        let p = (*c).sg_policy;
        lupos_sugov_update_lock(p);
        sugov_iowait_boost(c, time, flags);
        (*c).last_update = time;
        ignore_dl_rate_limit(c);
        if sugov_should_update_freq(p, time) {
            let next = sugov_next_freq_shared(c, time);
            if sugov_update_next_freq(p, time, next) {
                if lupos_sugov_policy_fast((*p).policy) {
                    cpufreq_driver_fast_switch((*p).policy, next);
                } else {
                    sugov_deferred_update(p);
                }
            }
        }
        lupos_sugov_update_unlock(p);
    }
}

/// Slow-path worker callback; live policy owned by the governor lifecycle.
/// # Safety
/// Native kthread work owns this callback and stop drains/cancels it before free.
#[export_name = "lupos_sugov_work"]
pub unsafe extern "C" fn sugov_work(work: *mut kthread_work) {
    unsafe {
        let p = lupos_sugov_policy_from_work(work);
        let flags = lupos_sugov_update_lock_irqsave(p);
        let freq = (*p).next_freq;
        (*p).work_in_progress = false;
        lupos_sugov_update_unlock_irqrestore(p, flags);
        lupos_sugov_work_lock(p);
        __cpufreq_driver_target((*p).policy, freq, LUPOS_SUGOV_RELATION_L);
        lupos_sugov_work_unlock(p);
    }
}

/// Queue a live governor's slow-path work.
/// # Safety
/// IRQ-work storage remains live until irq_work_sync completes in stop.
#[export_name = "lupos_sugov_irq_work"]
pub unsafe extern "C" fn sugov_irq_work(work: *mut irq_work) {
    unsafe { lupos_sugov_queue_work(lupos_sugov_policy_from_irq(work)) }
}

unsafe fn sugov_policy_alloc(policy: *mut cpufreq_policy) -> *mut sugov_policy {
    unsafe {
        let p = lupos_sugov_policy_alloc();
        if p.is_null() {
            return null_mut();
        }
        (*p).policy = policy;
        lupos_sugov_update_lock_init(p);
        p
    }
}

unsafe fn sugov_policy_free(p: *mut sugov_policy) {
    unsafe { lupos_sugov_policy_free(p) }
}

/// Final kobject release, after all tunable references have been put.
/// # Safety
/// Called exactly once by the native kobject type's release callback.
#[export_name = "lupos_sugov_tunables_release"]
pub unsafe extern "C" fn sugov_tunables_free(kobj: *mut kobject) {
    unsafe { lupos_sugov_tunables_free(lupos_sugov_tunables_from_kobj(kobj)) }
}

/// Native sysfs show callback.
/// # Safety
/// Native governor sysfs holds the attribute set alive and provides its buffer.
#[export_name = "lupos_sugov_rate_limit_us_show"]
pub unsafe extern "C" fn rate_limit_us_show(a: *mut gov_attr_set, buf: *mut c_char) -> ssize_t {
    unsafe { lupos_sugov_rate_emit(buf, (*lupos_sugov_tunables_from_attr(a)).rate_limit_us) }
}

/// Native sysfs store callback.
/// # Safety
/// Native governor sysfs serializes policy_list and supplies count bytes in buf.
#[export_name = "lupos_sugov_rate_limit_us_store"]
pub unsafe extern "C" fn rate_limit_us_store(
    a: *mut gov_attr_set,
    buf: *const c_char,
    count: size_t,
) -> ssize_t {
    unsafe {
        let tunables = lupos_sugov_tunables_from_attr(a);
        let mut rate = 0;
        if kstrtouint(buf, 10, &mut rate) != 0 {
            return -(LUPOS_SUGOV_EINVAL as ssize_t);
        }
        (*tunables).rate_limit_us = rate;
        let mut p = lupos_sugov_attr_first(a);
        while !p.is_null() {
            sugov_update_rate_limit_us(p);
            p = lupos_sugov_attr_next(a, p);
        }
        count as ssize_t
    }
}

unsafe fn sugov_kthread_create(p: *mut sugov_policy) -> c_int {
    unsafe {
        let policy = (*p).policy;
        if lupos_sugov_policy_fast(policy) {
            return 0;
        }
        lupos_sugov_work_init(p);
        lupos_sugov_worker_init(p);
        let thread = lupos_sugov_thread_create(p);
        if lupos_sugov_thread_is_err(thread) {
            lupos_sugov_thread_create_error(thread);
            return lupos_sugov_thread_error(thread);
        }
        // Native construction supplies every sched_attr member from the oracle,
        // including SCHED_DEADLINE, SCHED_FLAG_SUGOV and the bandwidth workaround.
        let ret = lupos_sugov_thread_setattr(thread);
        if ret != 0 {
            kthread_stop(thread);
            lupos_sugov_thread_sched_error();
            return ret;
        }
        (*p).thread = thread;
        if lupos_sched_cpufreq_policy_remote_dvfs(policy) {
            lupos_sugov_thread_allowed(p);
        } else {
            lupos_sugov_thread_bind(p);
        }
        lupos_sugov_irq_init(p);
        lupos_sugov_work_lock_init(p);
        wake_up_process(thread);
        0
    }
}

unsafe fn sugov_kthread_stop(p: *mut sugov_policy) {
    unsafe {
        if lupos_sugov_policy_fast((*p).policy) {
            return;
        }
        lupos_sugov_worker_flush(p);
        kthread_stop((*p).thread);
        lupos_sugov_work_lock_destroy(p);
    }
}

unsafe fn sugov_tunables_alloc(p: *mut sugov_policy) -> *mut sugov_tunables {
    unsafe {
        let tunables = lupos_sugov_tunables_alloc();
        if !tunables.is_null() {
            lupos_sugov_attr_init(tunables, p);
            if !have_governor_per_policy() {
                lupos_sugov_global_write(tunables);
            }
        }
        tunables
    }
}

unsafe fn sugov_clear_global_tunables() {
    unsafe {
        if !have_governor_per_policy() {
            lupos_sugov_global_write(null_mut());
        }
    }
}

/// Initialize one governor policy, sharing tunables when required by CPUFreq.
/// # Safety
/// The CPUFreq governor lifecycle serializes this callback for the live policy.
#[export_name = "lupos_sugov_init"]
pub unsafe extern "C" fn sugov_init(policy: *mut cpufreq_policy) -> c_int {
    unsafe {
        if !lupos_sugov_policy_data(policy).is_null() {
            return -LUPOS_SUGOV_EBUSY;
        }
        cpufreq_enable_fast_switch(policy);
        let p = sugov_policy_alloc(policy);
        if p.is_null() {
            let ret = -LUPOS_SUGOV_ENOMEM;
            cpufreq_disable_fast_switch(policy);
            lupos_sugov_init_error(ret);
            return ret;
        }
        let ret = sugov_kthread_create(p);
        if ret != 0 {
            sugov_policy_free(p);
            cpufreq_disable_fast_switch(policy);
            lupos_sugov_init_error(ret);
            return ret;
        }
        lupos_sugov_global_lock();
        // This block is the native locked init path; all failures below stop
        // the worker before unlocking, then free policy and disable fast switch.
        let ret = 'locked: {
            let global = lupos_sugov_global_read();
            if !global.is_null() {
                if lupos_sugov_warn_per_policy(have_governor_per_policy()) {
                    break 'locked -LUPOS_SUGOV_EINVAL;
                }
                lupos_sugov_policy_set_data(policy, p);
                (*p).tunables = global;
                lupos_sugov_attr_get(global, p);
                break 'locked 0;
            }
            let tunables = sugov_tunables_alloc(p);
            if tunables.is_null() {
                break 'locked -LUPOS_SUGOV_ENOMEM;
            }
            (*tunables).rate_limit_us = cpufreq_policy_transition_delay_us(policy);
            lupos_sugov_policy_set_data(policy, p);
            (*p).tunables = tunables;
            let ret = lupos_sugov_tunables_kobject_add(tunables, policy);
            if ret != 0 {
                // kobject_put may free tunables; do not access it after this.
                lupos_sugov_tunables_kobject_put(tunables);
                lupos_sugov_policy_set_data(policy, null_mut());
                sugov_clear_global_tunables();
            }
            ret
        };
        if ret == 0 {
            lupos_sugov_rebuild_domains();
            lupos_sugov_global_unlock();
            return 0;
        }
        sugov_kthread_stop(p);
        lupos_sugov_global_unlock();
        sugov_policy_free(p);
        cpufreq_disable_fast_switch(policy);
        lupos_sugov_init_error(ret);
        ret
    }
}

/// Drop a governor policy's tunable reference and destroy its worker/storage.
/// # Safety
/// CPUFreq has stopped this policy and excluded further hook/worker users.
#[export_name = "lupos_sugov_exit"]
pub unsafe extern "C" fn sugov_exit(policy: *mut cpufreq_policy) {
    unsafe {
        let p = lupos_sugov_policy_data(policy);
        let tunables = (*p).tunables;
        lupos_sugov_global_lock();
        let count = lupos_sugov_attr_put(tunables, p);
        lupos_sugov_policy_set_data(policy, null_mut());
        if count == 0 {
            sugov_clear_global_tunables();
        }
        lupos_sugov_global_unlock();
        sugov_kthread_stop(p);
        sugov_policy_free(p);
        cpufreq_disable_fast_switch(policy);
        lupos_sugov_rebuild_domains();
    }
}

/// Initialize every sibling CPU before publishing any policy hook.
/// # Safety
/// CPUFreq owns the stopped policy and its CPU mask throughout this callback.
#[export_name = "lupos_sugov_start"]
pub unsafe extern "C" fn sugov_start(policy: *mut cpufreq_policy) -> c_int {
    unsafe {
        let p = lupos_sugov_policy_data(policy);
        sugov_update_rate_limit_us(p);
        (*p).last_freq_update_time = 0;
        (*p).next_freq = 0;
        (*p).work_in_progress = false;
        // There are no update callbacks yet, matching the native plain store.
        (*p).limits_changed = false;
        (*p).cached_raw_freq = 0;
        (*p).need_freq_update = cpufreq_driver_test_flags(LUPOS_SUGOV_NEED_UPDATE_LIMITS);
        let update: unsafe extern "C" fn(c_int, *mut update_util_data) =
            if lupos_sugov_policy_shared(policy) {
                lupos_sugov_add_shared_hook
            } else if lupos_sugov_policy_fast(policy) && cpufreq_driver_has_adjust_perf() {
                lupos_sugov_add_single_perf_hook
            } else {
                lupos_sugov_add_single_freq_hook
            };
        let mut cpu = lupos_sugov_first_cpu(policy);
        while lupos_sugov_valid_cpu(cpu) {
            let c = lupos_sugov_cpu_storage(cpu);
            lupos_sugov_cpu_zero(c);
            (*c).cpu = cpu;
            (*c).sg_policy = p;
            cpu = lupos_sugov_next_cpu(policy, cpu);
        }
        cpu = lupos_sugov_first_cpu(policy);
        while lupos_sugov_valid_cpu(cpu) {
            let c = lupos_sugov_cpu_storage(cpu);
            update(cpu as c_int, addr_of_mut!((*c).update_util));
            cpu = lupos_sugov_next_cpu(policy, cpu);
        }
        0
    }
}

/// Withdraw hooks and wait for all readers, IRQ work and kthread work.
/// # Safety
/// Called under the CPUFreq governor lifecycle before teardown or restart.
#[export_name = "lupos_sugov_stop"]
pub unsafe extern "C" fn sugov_stop(policy: *mut cpufreq_policy) {
    unsafe {
        let p = lupos_sugov_policy_data(policy);
        let mut cpu = lupos_sugov_first_cpu(policy);
        while lupos_sugov_valid_cpu(cpu) {
            cpufreq_remove_update_util_hook(cpu as c_int);
            cpu = lupos_sugov_next_cpu(policy, cpu);
        }
        synchronize_rcu();
        if !lupos_sugov_policy_fast(policy) {
            lupos_sugov_irq_sync(p);
            lupos_sugov_work_cancel_sync(p);
        }
    }
}

/// Apply slow-path limits synchronously and notify the next utilization update.
/// # Safety
/// CPUFreq supplies the live policy and its normal limits-update serialization.
#[export_name = "lupos_sugov_limits"]
pub unsafe extern "C" fn sugov_limits(policy: *mut cpufreq_policy) {
    unsafe {
        let p = lupos_sugov_policy_data(policy);
        if !lupos_sugov_policy_fast(policy) {
            lupos_sugov_work_lock(p);
            lupos_sugov_policy_apply_limits(policy);
            lupos_sugov_work_unlock(p);
        }
        lupos_sugov_write_barrier();
        lupos_sugov_limits_write(p, true);
    }
}

/// Return the configured default governor's native registration object.
/// # Safety
/// The native metadata has static lifetime and is registered by core_initcall.
#[cfg(CONFIG_CPU_FREQ_DEFAULT_GOV_SCHEDUTIL)]
#[no_mangle]
pub unsafe extern "C" fn cpufreq_default_governor() -> *mut cpufreq_governor {
    unsafe { lupos_sugov_governor() }
}

/// Compare policy identity with the single native schedutil governor object.
/// # Safety
/// Policy is live under CPUFreq serialization.
#[no_mangle]
pub unsafe extern "C" fn sugov_is_governor(policy: *mut cpufreq_policy) -> bool {
    unsafe { lupos_sugov_policy_governor(policy) == lupos_sugov_governor() }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
