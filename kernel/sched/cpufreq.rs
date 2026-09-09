// SPDX-License-Identifier: GPL-2.0
/*
 * Scheduler code and data structures related to cpufreq.
 *
 * Copyright (C) 2016, Intel Corporation
 * Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>
 */

// Dependency declarations and per-CPU machinery are supplied by sched.h.

extern "C" {
    pub static mut cpufreq_update_util_data:
        PerCpu<*mut update_util_data>;
}

#[repr(C)]
pub struct PerCpu<T>(::core::marker::PhantomData<T>);

#[repr(C)]
pub struct update_util_data {
    pub func: Option<unsafe extern "C" fn(data: *mut update_util_data, time: u64, flags: u32)>,
}

#[repr(C)]
pub struct cpufreq_policy {
    pub cpus: *const cpumask,
    pub dvfs_possible_from_any_cpu: bool,
}

#[repr(C)]
pub struct cpumask;

extern "C" {
    fn warn_on(condition: bool) -> bool;
    fn per_cpu<T>(data: *mut PerCpu<T>, cpu: i32) -> *mut T;
    fn rcu_assign_pointer<T>(pointer: *mut T, value: T);
    fn cpumask_test_cpu(cpu: i32, mask: *const cpumask) -> bool;
    fn smp_processor_id() -> i32;
    fn this_cpu_ptr<T>(data: *const PerCpu<T>) -> *mut T;
    fn rcu_dereference_sched<T>(pointer: T) -> T;
}

/**
 * cpufreq_add_update_util_hook - Populate the CPU's update_util_data pointer.
 * @cpu: The CPU to set the pointer for.
 * @data: New pointer value.
 * @func: Callback function to set for the CPU.
 *
 * Set and publish the update_util_data pointer for the given CPU.
 *
 * The update_util_data pointer of @cpu is set to @data and the callback
 * function pointer in the target struct update_util_data is set to @func.
 * That function will be called by cpufreq_update_util() from RCU-sched
 * read-side critical sections, so it must not sleep.  @data will always be
 * passed to it as the first argument which allows the function to get to the
 * target update_util_data structure and its container.
 *
 * The update_util_data pointer of @cpu must be NULL when this function is
 * called or it will WARN() and return with no effect.
 */
#[no_mangle]
pub unsafe extern "C" fn cpufreq_add_update_util_hook(
    cpu: i32,
    data: *mut update_util_data,
    func: Option<unsafe extern "C" fn(data: *mut update_util_data, time: u64, flags: u32)>,
) {
    if warn_on(data.is_null() || func.is_none()) {
        return;
    }

    if warn_on(!per_cpu(&mut cpufreq_update_util_data, cpu).is_null()) {
        return;
    }

    (*data).func = func;
    rcu_assign_pointer(per_cpu(&mut cpufreq_update_util_data, cpu), data);
}

/**
 * cpufreq_remove_update_util_hook - Clear the CPU's update_util_data pointer.
 * @cpu: The CPU to clear the pointer for.
 *
 * Clear the update_util_data pointer for the given CPU.
 *
 * Callers must use RCU callbacks to free any memory that might be
 * accessed via the old update_util_data pointer or invoke synchronize_rcu()
 * right after this function to avoid use-after-free.
 */
#[no_mangle]
pub unsafe extern "C" fn cpufreq_remove_update_util_hook(cpu: i32) {
    rcu_assign_pointer(per_cpu(&mut cpufreq_update_util_data, cpu), core::ptr::null_mut());
}

/**
 * cpufreq_this_cpu_can_update - Check if cpufreq policy can be updated.
 * @policy: cpufreq policy to check.
 *
 * Return 'true' if:
 * - the local and remote CPUs share @policy,
 * - dvfs_possible_from_any_cpu is set in @policy and the local CPU is not going
 *   offline (in which case it is not expected to run cpufreq updates any more).
 */
#[no_mangle]
pub unsafe extern "C" fn cpufreq_this_cpu_can_update(policy: *mut cpufreq_policy) -> bool {
    cpumask_test_cpu(smp_processor_id(), (*policy).cpus)
        || ((*policy).dvfs_possible_from_any_cpu
            && !rcu_dereference_sched(*this_cpu_ptr(&cpufreq_update_util_data)).is_null())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
