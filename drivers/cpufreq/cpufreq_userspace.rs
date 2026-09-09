// SPDX-License-Identifier: GPL-2.0-only

/*
 *  linux/drivers/cpufreq/cpufreq_userspace.c
 *
 *  Copyright (C)  2001 Russell King
 *            (C)  2002 - 2004 Dominik Brodowski <linux@brodo.de>
 */

// Dependencies corresponding to the Linux kernel headers are supplied externally.

#[repr(C)]
struct userspace_policy {
    is_managed: core::ffi::c_uint,
    setspeed: core::ffi::c_uint,
    mutex: mutex,
}

/**
 * cpufreq_set - set the CPU frequency
 * @policy: pointer to policy struct where freq is being set
 * @freq: target frequency in kHz
 *
 * Sets the CPU frequency to freq.
 */
unsafe fn cpufreq_set(policy: *mut cpufreq_policy, freq: core::ffi::c_uint) -> i32 {
    let mut ret: i32 = -EINVAL;
    let userspace = (*policy).governor_data as *mut userspace_policy;

    pr_debug!("cpufreq_set for cpu %u, freq %u kHz\n", (*policy).cpu, freq);

    mutex_lock(&mut (*userspace).mutex);
    if (*userspace).is_managed == 0 {
        mutex_unlock(&mut (*userspace).mutex);
        return ret;
    }

    (*userspace).setspeed = freq;

    ret = __cpufreq_driver_target(policy, freq, CPUFREQ_RELATION_L);
    mutex_unlock(&mut (*userspace).mutex);
    ret
}

unsafe fn show_speed(policy: *mut cpufreq_policy, buf: *mut core::ffi::c_char) -> isize {
    let userspace = (*policy).governor_data as *mut userspace_policy;

    sprintf!(buf, "%u\n", (*userspace).setspeed)
}

unsafe fn cpufreq_userspace_policy_init(policy: *mut cpufreq_policy) -> i32 {
    let userspace = kzalloc_obj::<userspace_policy>();
    if userspace.is_null() {
        return -ENOMEM;
    }

    mutex_init(&mut (*userspace).mutex);

    (*policy).governor_data = userspace as *mut core::ffi::c_void;
    0
}

/*
 * Any routine that writes to the policy struct will hold the "rwsem" of
 * policy struct that means it is free to free "governor_data" here.
 */
unsafe fn cpufreq_userspace_policy_exit(policy: *mut cpufreq_policy) {
    kfree((*policy).governor_data);
    (*policy).governor_data = core::ptr::null_mut();
}

unsafe fn cpufreq_userspace_policy_start(policy: *mut cpufreq_policy) -> i32 {
    let userspace = (*policy).governor_data as *mut userspace_policy;

    BUG_ON!((*policy).cur == 0);
    pr_debug!("started managing cpu %u\n", (*policy).cpu);

    mutex_lock(&mut (*userspace).mutex);
    (*userspace).is_managed = 1;
    (*userspace).setspeed = (*policy).cur;
    mutex_unlock(&mut (*userspace).mutex);
    0
}

unsafe fn cpufreq_userspace_policy_stop(policy: *mut cpufreq_policy) {
    let userspace = (*policy).governor_data as *mut userspace_policy;

    pr_debug!("managing cpu %u stopped\n", (*policy).cpu);

    mutex_lock(&mut (*userspace).mutex);
    (*userspace).is_managed = 0;
    (*userspace).setspeed = 0;
    mutex_unlock(&mut (*userspace).mutex);
}

unsafe fn cpufreq_userspace_policy_limits(policy: *mut cpufreq_policy) {
    let userspace = (*policy).governor_data as *mut userspace_policy;

    mutex_lock(&mut (*userspace).mutex);

    pr_debug!(
        "limit event for cpu %u: %u - %u kHz, currently %u kHz, last set to %u kHz\n",
        (*policy).cpu, (*policy).min, (*policy).max, (*policy).cur, (*userspace).setspeed
    );

    if (*policy).max < (*userspace).setspeed {
        __cpufreq_driver_target(policy, (*policy).max, CPUFREQ_RELATION_H);
    } else if (*policy).min > (*userspace).setspeed {
        __cpufreq_driver_target(policy, (*policy).min, CPUFREQ_RELATION_L);
    } else {
        __cpufreq_driver_target(policy, (*userspace).setspeed, CPUFREQ_RELATION_L);
    }

    mutex_unlock(&mut (*userspace).mutex);
}

static mut cpufreq_gov_userspace: cpufreq_governor = cpufreq_governor {
    name: "userspace",
    init: Some(cpufreq_userspace_policy_init),
    exit: Some(cpufreq_userspace_policy_exit),
    start: Some(cpufreq_userspace_policy_start),
    stop: Some(cpufreq_userspace_policy_stop),
    limits: Some(cpufreq_userspace_policy_limits),
    store_setspeed: Some(cpufreq_set),
    show_setspeed: Some(show_speed),
    owner: THIS_MODULE,
    flags: CPUFREQ_GOV_STRICT_TARGET,
};

MODULE_AUTHOR!("Dominik Brodowski <linux@brodo.de>, Russell King <rmk@arm.linux.org.uk>");
MODULE_DESCRIPTION!("CPUfreq policy governor 'userspace'");
MODULE_LICENSE!("GPL");

#[cfg(CONFIG_CPU_FREQ_DEFAULT_GOV_USERSPACE)]
unsafe fn cpufreq_default_governor() -> *mut cpufreq_governor {
    &raw mut cpufreq_gov_userspace
}

cpufreq_governor_init!(cpufreq_gov_userspace);
cpufreq_governor_exit!(cpufreq_gov_userspace);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
