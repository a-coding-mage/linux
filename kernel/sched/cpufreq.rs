// SPDX-License-Identifier: GPL-2.0
/*
 * Scheduler code and data structures related to cpufreq.
 * Copyright (C) 2016, Intel Corporation
 * Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>
 * Continued from the existing Rust against native commit
 * 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. Native headers own every layout.
 */
#[cfg(CONFIG_RUST_SCHED_CPUFREQ)]
compile_error!("SOURCE ONLY HOLD: scheduler CPUFreq hooks are not admitted");

use kernel::bindings::sched_cpufreq_native::*;
use kernel::ffi::c_int;

/// Publish an initialized hook to a CPU's RCU-protected slot.
///
/// # Safety
/// Caller serializes hook installation and keeps `data` alive through removal
/// and the following RCU grace period. The callback cannot sleep. Native C
/// supplies a live temporary `func` holder for this call; it is never retained.
#[export_name = "lupos_sched_cpufreq_add_update_util_hook"]
pub unsafe extern "C" fn cpufreq_add_update_util_hook(
    cpu: c_int,
    data: *mut update_util_data,
    func: *const update_util_data,
) {
    // SAFETY: Native leaves retain WARN_ON and per-CPU/RCU semantics. Test the
    // stored pointer, not the address of the per-CPU slot.
    unsafe {
        if lupos_sched_cpufreq_warn_invalid_hook(
            data.is_null() || lupos_sched_cpufreq_callback_missing(func),
        ) {
            return;
        }
        if lupos_sched_cpufreq_warn_occupied_hook(
            lupos_sched_cpufreq_hook_present(cpu),
        ) {
            return;
        }
        lupos_sched_cpufreq_hook_set_func(data, func);
        lupos_sched_cpufreq_hook_publish(cpu, data);
    }
}

/// Clear a CPU hook without reclaiming the old object.
///
/// # Safety
/// Caller serializes removal, and waits for an RCU grace period before freeing
/// hook data or any containing object.
#[no_mangle]
pub unsafe extern "C" fn cpufreq_remove_update_util_hook(cpu: c_int) {
    unsafe { lupos_sched_cpufreq_hook_publish(cpu, core::ptr::null_mut()) }
}

/// Check local policy membership or an active remote-update-capable CPU.
///
/// # Safety
/// Policy and masks must be live under native scheduler/CPUFreq serialization.
#[no_mangle]
pub unsafe extern "C" fn cpufreq_this_cpu_can_update(policy: *mut cpufreq_policy) -> bool {
    unsafe {
        lupos_sched_cpufreq_policy_has_current_cpu(policy)
            || (lupos_sched_cpufreq_policy_remote_dvfs(policy)
                && !lupos_sched_cpufreq_this_hook_rcu().is_null())
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
