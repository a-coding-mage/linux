// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * processor_thermal.c - Passive cooling submodule of the ACPI processor driver
 *
 *  Copyright (C) 2001, 2002 Andy Grover <andrew.grover@intel.com>
 *  Copyright (C) 2001, 2002 Paul Diefenbaugh <paul.s.diefenbaugh@intel.com>
 *  Copyright (C) 2004       Dominik Brodowski <linux@brodo.de>
 *  Copyright (C) 2004  Anil S Keshavamurthy <anil.s.keshavamurthy@intel.com>
 *                      - Added processor hotplug support
 */

// CONFIG_CPU_FREQ-dependent implementation.

#[cfg(feature = "CONFIG_CPU_FREQ")]
const CPUFREQ_THERMAL_MIN_STEP: i32 = 0;

#[cfg(feature = "CONFIG_CPU_FREQ")]
static mut CPUFREQ_THERMAL_MAX_STEP: i32 = 3;

#[cfg(feature = "CONFIG_CPU_FREQ")]
static mut CPUFREQ_THERMAL_REDUCTION_PCTG: i32 = 20;

#[cfg(feature = "CONFIG_CPU_FREQ")]
static mut CPUFREQ_THERMAL_REDUCTION_STEP: [u32; 1] = [0];

#[cfg(feature = "CONFIG_CPU_FREQ")]
unsafe fn phys_package_first_cpu(cpu: i32) -> i32 {
    let mut i: i32;
    let id = topology_physical_package_id(cpu);

    for_each_online_cpu!(i {
        if topology_physical_package_id(i) == id {
            return i;
        }
    });
    0
}

#[cfg(feature = "CONFIG_CPU_FREQ")]
unsafe fn reduction_step(cpu: i32) -> &'static mut u32 {
    &mut CPUFREQ_THERMAL_REDUCTION_STEP[phys_package_first_cpu(cpu) as usize]
}

#[cfg(feature = "CONFIG_CPU_FREQ")]
unsafe fn cpu_has_cpufreq(cpu: u32) -> bool {
    if !acpi_processor_cpufreq_init {
        return false;
    }

    let policy = cpufreq_cpu_get(cpu);
    if !policy.is_null() {
        put_cpufreq_policy(policy);
    }
    !policy.is_null()
}

#[cfg(feature = "CONFIG_CPU_FREQ")]
unsafe fn cpufreq_get_max_state(cpu: u32) -> i32 {
    if !cpu_has_cpufreq(cpu) {
        return 0;
    }
    CPUFREQ_THERMAL_MAX_STEP
}

#[cfg(feature = "CONFIG_CPU_FREQ")]
unsafe fn cpufreq_get_cur_state(cpu: u32) -> u32 {
    if !cpu_has_cpufreq(cpu) {
        return 0;
    }
    *reduction_step(cpu as i32)
}

#[cfg(feature = "CONFIG_CPU_FREQ")]
unsafe fn cpufreq_update_thermal_limit(cpu: u32, pr: *mut acpi_processor) -> bool {
    let policy = cpufreq_cpu_get(cpu);
    if policy.is_null() {
        return false;
    }

    let max_freq = ((*policy).cpuinfo.max_freq
        * (100 - (*reduction_step(cpu as i32) as i32) * CPUFREQ_THERMAL_REDUCTION_PCTG) as u64)
        / 100;
    let ret = freq_qos_update_request(&mut (*pr).thermal_req, max_freq);
    if ret < 0 {
        pr_warn!("Failed to update thermal freq constraint: CPU{} ({})\n", (*pr).id, ret);
    }
    put_cpufreq_policy(policy);
    true
}

#[cfg(feature = "CONFIG_CPU_FREQ")]
unsafe fn cpufreq_set_cur_state(cpu: u32, state: i32) -> i32 {
    if !cpu_has_cpufreq(cpu) {
        return 0;
    }
    *reduction_step(cpu as i32) = state as u32;

    let mut i: i32;
    for_each_online_cpu!(i {
        if topology_physical_package_id(i) != topology_physical_package_id(cpu as i32) {
            continue;
        }
        let pr = per_cpu!(processors, i);
        if unlikely!(!freq_qos_request_active(&(*pr).thermal_req)) {
            continue;
        }
        if !cpufreq_update_thermal_limit(i as u32, pr) {
            return -EINVAL;
        }
    });
    0
}

#[cfg(feature = "CONFIG_CPU_FREQ")]
unsafe fn acpi_thermal_cpufreq_config() {
    let cpufreq_pctg = acpi_arch_thermal_cpufreq_pctg();
    if cpufreq_pctg == 0 {
        return;
    }
    CPUFREQ_THERMAL_REDUCTION_PCTG = cpufreq_pctg;
    CPUFREQ_THERMAL_MAX_STEP = (100 / cpufreq_pctg) - 2;
}

#[cfg(feature = "CONFIG_CPU_FREQ")]
pub unsafe fn acpi_thermal_cpufreq_init(policy: *mut cpufreq_policy) {
    acpi_thermal_cpufreq_config();
    let mut cpu: u32;
    for_each_cpu!(cpu, (*policy).related_cpus, {
        let pr = per_cpu!(processors, cpu);
        if pr.is_null() { continue; }
        let ret = freq_qos_add_request(&mut (*policy).constraints,
            &mut (*pr).thermal_req, FREQ_QOS_MAX, INT_MAX);
        if ret < 0 {
            pr_err!("Failed to add freq constraint for CPU{} ({})\n", cpu, ret);
            continue;
        }
        thermal_cooling_device_update((*pr).cdev);
    });
}

#[cfg(feature = "CONFIG_CPU_FREQ")]
pub unsafe fn acpi_thermal_cpufreq_exit(policy: *mut cpufreq_policy) {
    let mut cpu: u32;
    for_each_cpu!(cpu, (*policy).related_cpus, {
        let pr = per_cpu!(processors, cpu);
        if pr.is_null() { continue; }
        freq_qos_remove_request(&mut (*pr).thermal_req);
        thermal_cooling_device_update((*pr).cdev);
    });
}

#[cfg(not(feature = "CONFIG_CPU_FREQ"))]
unsafe fn cpufreq_get_max_state(_cpu: u32) -> i32 { 0 }

#[cfg(not(feature = "CONFIG_CPU_FREQ"))]
unsafe fn cpufreq_get_cur_state(_cpu: u32) -> u32 { 0 }

#[cfg(not(feature = "CONFIG_CPU_FREQ"))]
unsafe fn cpufreq_set_cur_state(_cpu: u32, _state: i32) -> i32 { 0 }

unsafe fn acpi_processor_max_state(pr: *mut acpi_processor) -> i32 {
    let mut max_state = cpufreq_get_max_state((*pr).id);
    if (*pr).flags.throttling {
        max_state += (*pr).throttling.state_count - 1;
    }
    max_state
}

unsafe fn processor_get_max_state(cdev: *mut thermal_cooling_device, state: *mut u64) -> i32 {
    let device = (*cdev).devdata;
    if device.is_null() { return -EINVAL; }
    let pr = acpi_driver_data(device);
    if pr.is_null() { return -EINVAL; }
    *state = acpi_processor_max_state(pr) as u64;
    0
}

unsafe fn processor_get_cur_state(cdev: *mut thermal_cooling_device, cur_state: *mut u64) -> i32 {
    let device = (*cdev).devdata;
    if device.is_null() { return -EINVAL; }
    let pr = acpi_driver_data(device);
    if pr.is_null() { return -EINVAL; }
    *cur_state = cpufreq_get_cur_state((*pr).id) as u64;
    if (*pr).flags.throttling { *cur_state += (*pr).throttling.state as u64; }
    0
}

unsafe fn processor_set_cur_state(cdev: *mut thermal_cooling_device, state: u64) -> i32 {
    let device = (*cdev).devdata;
    if device.is_null() { return -EINVAL; }
    let pr = acpi_driver_data(device);
    if pr.is_null() { return -EINVAL; }
    let mut result = 0;
    let max_pstate = cpufreq_get_max_state((*pr).id) as u64;
    if state > acpi_processor_max_state(pr) as u64 { return -EINVAL; }
    if state <= max_pstate {
        if (*pr).flags.throttling && (*pr).throttling.state != 0 {
            result = acpi_processor_set_throttling(pr, 0, false);
        }
        cpufreq_set_cur_state((*pr).id, state as i32);
    } else {
        cpufreq_set_cur_state((*pr).id, max_pstate as i32);
        result = acpi_processor_set_throttling(pr, (state - max_pstate) as i32, false);
    }
    result
}

pub static processor_cooling_ops: thermal_cooling_device_ops = thermal_cooling_device_ops {
    get_max_state: Some(processor_get_max_state),
    get_cur_state: Some(processor_get_cur_state),
    set_cur_state: Some(processor_set_cur_state),
};

pub unsafe fn acpi_processor_thermal_init(pr: *mut acpi_processor, device: *mut acpi_device) -> i32 {
    let mut result = 0;
    (*pr).cdev = thermal_cooling_device_register("Processor", device, &processor_cooling_ops);
    if IS_ERR!((*pr).cdev) { return PTR_ERR!((*pr).cdev); }
    dev_dbg!(&(*device).dev, "registered as cooling_device{}\n", (*(*pr).cdev).id);
    result = sysfs_create_link(&(*device).dev.kobj, &(*(*pr).cdev).device.kobj, "thermal_cooling");
    if result != 0 { goto err_thermal_unregister; }
    result = sysfs_create_link(&(*(*pr).cdev).device.kobj, &(*device).dev.kobj, "device");
    if result != 0 { goto err_remove_sysfs_thermal; }
    return 0;
err_remove_sysfs_thermal:
    sysfs_remove_link(&(*device).dev.kobj, "thermal_cooling");
err_thermal_unregister:
    thermal_cooling_device_unregister((*pr).cdev);
    result
}

pub unsafe fn acpi_processor_thermal_exit(pr: *mut acpi_processor, device: *mut acpi_device) {
    if !(*pr).cdev.is_null() {
        sysfs_remove_link(&(*device).dev.kobj, "thermal_cooling");
        sysfs_remove_link(&(*(*pr).cdev).device.kobj, "device");
        thermal_cooling_device_unregister((*pr).cdev);
        (*pr).cdev = core::ptr::null_mut();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
