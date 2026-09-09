/*
 * governor.c - governor support
 *
 * (C) 2006-2007 Venkatesh Pallipadi <venkatesh.pallipadi@intel.com>
 *               Shaohua Li <shaohua.li@intel.com>
 *               Adam Belay <abelay@novell.com>
 *
 * This code is licenced under the GPL.
 */

// C dependencies supplied by the surrounding kernel translation.
use crate::*;

pub static mut PARAM_GOVERNOR: [core::ffi::c_char; CPUIDLE_NAME_LEN] =
    [0; CPUIDLE_NAME_LEN];

pub static mut CPUIDLE_GOVERNORS: ListHead = ListHead::new();
pub static mut CPUIDLE_CURR_GOVERNOR: *mut CpuidleGovernor = core::ptr::null_mut();
pub static mut CPUIDLE_PREV_GOVERNOR: *mut CpuidleGovernor = core::ptr::null_mut();

/**
 * cpuidle_find_governor - finds a governor of the specified name
 * @str: the name
 *
 * Must be called with cpuidle_lock acquired.
 */
pub unsafe fn cpuidle_find_governor(str_: *const core::ffi::c_char) -> *mut CpuidleGovernor {
    let mut gov: *mut CpuidleGovernor;

    list_for_each_entry!(gov, &mut CPUIDLE_GOVERNORS, governor_list, {
        if strncasecmp(str_, (*gov).name.as_ptr(), CPUIDLE_NAME_LEN) == 0 {
            return gov;
        }
    });

    core::ptr::null_mut()
}

/**
 * cpuidle_switch_governor - changes the governor
 * @gov: the new target governor
 * Must be called with cpuidle_lock acquired.
 */
pub unsafe fn cpuidle_switch_governor(gov: *mut CpuidleGovernor) -> i32 {
    let mut dev: *mut CpuidleDevice;

    if gov.is_null() {
        return -EINVAL;
    }

    if gov == CPUIDLE_CURR_GOVERNOR {
        return 0;
    }

    cpuidle_uninstall_idle_handler();

    if !CPUIDLE_CURR_GOVERNOR.is_null() {
        list_for_each_entry!(dev, &mut CPUIDLE_DETECTED_DEVICES, device_list, {
            cpuidle_disable_device(dev);
        });
    }

    CPUIDLE_CURR_GOVERNOR = gov;

    list_for_each_entry!(dev, &mut CPUIDLE_DETECTED_DEVICES, device_list, {
        cpuidle_enable_device(dev);
    });

    cpuidle_install_idle_handler();
    pr_info!("cpuidle: using governor %s\n", (*gov).name.as_ptr());

    0
}

/**
 * cpuidle_register_governor - registers a governor
 * @gov: the governor
 */
pub unsafe fn cpuidle_register_governor(gov: *mut CpuidleGovernor) -> i32 {
    let mut ret: i32 = -EEXIST;

    if gov.is_null() || (*gov).select.is_none() {
        return -EINVAL;
    }

    if cpuidle_disabled() {
        return -ENODEV;
    }

    mutex_lock(&mut CPUIDLE_LOCK);
    if cpuidle_find_governor((*gov).name.as_ptr()).is_null() {
        ret = 0;
        list_add_tail(&mut (*gov).governor_list, &mut CPUIDLE_GOVERNORS);
        if CPUIDLE_CURR_GOVERNOR.is_null()
            || strncasecmp(PARAM_GOVERNOR.as_ptr(), (*gov).name.as_ptr(), CPUIDLE_NAME_LEN) == 0
            || ((*CPUIDLE_CURR_GOVERNOR).rating < (*gov).rating
                && strncasecmp(
                    PARAM_GOVERNOR.as_ptr(),
                    (*CPUIDLE_CURR_GOVERNOR).name.as_ptr(),
                    CPUIDLE_NAME_LEN,
                ) != 0)
        {
            cpuidle_switch_governor(gov);
        }
    }
    mutex_unlock(&mut CPUIDLE_LOCK);

    ret
}

/**
 * cpuidle_governor_latency_req - Compute a latency constraint for CPU
 * @cpu: Target CPU
 */
pub unsafe fn cpuidle_governor_latency_req(cpu: u32) -> i64 {
    let device = get_cpu_device(cpu);
    let mut device_req = dev_pm_qos_raw_resume_latency(device);
    let mut global_req = cpu_latency_qos_limit();
    let global_wake_req = cpu_wakeup_latency_qos_limit();

    if global_req > global_wake_req {
        global_req = global_wake_req;
    }

    if device_req > global_req {
        device_req = global_req;
    }

    (device_req as i64).wrapping_mul(NSEC_PER_USEC as i64)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
