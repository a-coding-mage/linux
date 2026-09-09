// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * processor_driver.c - ACPI Processor Driver
 *
 *  Copyright (C) 2001, 2002 Andy Grover <andrew.grover@intel.com>
 *  Copyright (C) 2001, 2002 Paul Diefenbaugh <paul.s.diefenbaugh@intel.com>
 *  Copyright (C) 2004       Dominik Brodowski <linux@brodo.de>
 *  Copyright (C) 2004  Anil S Keshavamurthy <anil.s.keshavamurthy@intel.com>
 *                      - Added processor hotplug support
 *  Copyright (C) 2013, Intel Corporation
 *                      Rafael J. Wysocki <rafael.j.wysocki@intel.com>
 */

// Dependencies supplied by the surrounding kernel translation.

const ACPI_PROCESSOR_NOTIFY_PERFORMANCE: u32 = 0x80;
const ACPI_PROCESSOR_NOTIFY_POWER: u32 = 0x81;
const ACPI_PROCESSOR_NOTIFY_THROTTLING: u32 = 0x82;
const ACPI_PROCESSOR_NOTIFY_HIGEST_PERF_CHANGED: u32 = 0x85;

// MODULE_AUTHOR("Paul Diefenbaugh");
// MODULE_DESCRIPTION("ACPI Processor Driver");
// MODULE_LICENSE("GPL");

unsafe extern "C" {
    static processor_device_ids: [acpi_device_id; 4];
    static mut acpi_processor_driver: device_driver;
}

unsafe fn acpi_processor_stop(dev: *mut device) -> i32;

#[repr(C)]
struct acpi_device_id {
    _opaque: [u8; 0],
}

#[repr(C)]
struct device_driver {
    _opaque: [u8; 0],
}

#[repr(C)]
struct device {
    _opaque: [u8; 0],
}

#[repr(C)]
struct acpi_device {
    _opaque: [u8; 0],
}

#[repr(C)]
struct acpi_processor {
    _opaque: [u8; 0],
}

#[repr(C)]
struct notifier_block {
    _opaque: [u8; 0],
}

#[repr(C)]
struct cpufreq_policy {
    _opaque: [u8; 0],
}

type AcpiHandle = *mut core::ffi::c_void;
type AcpiStatus = u32;

unsafe fn acpi_processor_notify(
    handle: AcpiHandle,
    event: u32,
    data: *mut core::ffi::c_void,
) {
    let device = data as *mut acpi_device;
    let mut pr: *mut acpi_processor;
    let mut saved: i32;
    let mut ev_data: i32 = 0;

    if acpi_device_handle(device) != handle {
        return;
    }

    pr = acpi_driver_data(device);
    if pr.is_null() {
        return;
    }

    match event {
        ACPI_PROCESSOR_NOTIFY_PERFORMANCE => {
            saved = acpi_processor_performance_platform_limit(pr);
            acpi_processor_ppc_has_changed(pr, 1);
            ev_data = acpi_processor_performance_platform_limit(pr);
            if saved == ev_data {
                return;
            }
        }
        ACPI_PROCESSOR_NOTIFY_POWER => {
            acpi_processor_power_state_has_changed(pr);
        }
        ACPI_PROCESSOR_NOTIFY_THROTTLING => {
            acpi_processor_tstate_has_changed(pr);
        }
        ACPI_PROCESSOR_NOTIFY_HIGEST_PERF_CHANGED => {
            cpufreq_update_limits(acpi_processor_id(pr));
        }
        _ => {
            acpi_handle_debug(handle, "Unsupported event [0x%x]\\n", event);
            return;
        }
    }

    acpi_bus_generate_netlink_event(
        "processor",
        dev_name(acpi_device_dev(device)),
        event,
        ev_data,
    );
}

unsafe fn acpi_soft_cpu_online(cpu: u32) -> i32 {
    let pr = per_cpu_processors(cpu);
    if pr.is_null() {
        return 0;
    }

    let device = acpi_fetch_acpi_dev(acpi_processor_handle(pr));
    if device.is_null() {
        return 0;
    }

    if !acpi_processor_previously_online(pr) {
        let ret = __acpi_processor_start(device);
        warn(ret != 0, "Failed to start CPU: %d\\n", acpi_processor_id(pr));
    } else {
        acpi_processor_ppc_has_changed(pr, 0);
        acpi_processor_hotplug(pr);
        acpi_processor_reevaluate_tstate(pr, false);
        acpi_processor_tstate_has_changed(pr);
    }
    0
}

unsafe fn acpi_soft_cpu_dead(cpu: u32) -> i32 {
    let pr = per_cpu_processors(cpu);
    if pr.is_null() || acpi_fetch_acpi_dev(acpi_processor_handle(pr)).is_null() {
        return 0;
    }

    acpi_processor_reevaluate_tstate(pr, true);
    0
}

#[cfg(CONFIG_ACPI_CPU_FREQ_PSS)]
unsafe fn acpi_pss_perf_init(pr: *mut acpi_processor) {
    acpi_processor_ppc_has_changed(pr, 0);
    acpi_processor_get_throttling_info(pr);
    if acpi_processor_throttling(pr) {
        acpi_processor_set_limit(pr, 1);
    }
}

#[cfg(not(CONFIG_ACPI_CPU_FREQ_PSS))]
unsafe fn acpi_pss_perf_init(_pr: *mut acpi_processor) {}

unsafe fn __acpi_processor_start(device: *mut acpi_device) -> i32 {
    let pr = acpi_driver_data(device);
    if pr.is_null() {
        return -19;
    }

    let mut result = acpi_cppc_processor_probe(pr);
    if result != 0 && !cfg!(CONFIG_ACPI_CPU_FREQ_PSS) {
        dev_dbg(acpi_device_dev(device), "CPPC data invalid or not present\\n");
    }

    acpi_processor_power_init(pr);
    acpi_pss_perf_init(pr);

    result = acpi_processor_thermal_init(pr, device);
    if result != 0 {
        acpi_processor_power_exit(pr);
        return result;
    }

    let status = acpi_install_notify_handler(
        acpi_device_handle(device),
        ACPI_DEVICE_NOTIFY,
        acpi_processor_notify,
        device as *mut core::ffi::c_void,
    );
    if !acpi_success(status) {
        acpi_processor_thermal_exit(pr, device);
        acpi_processor_power_exit(pr);
        return -19;
    }
    acpi_processor_set_previously_online(pr, true);
    0
}

unsafe fn acpi_processor_stop(dev: *mut device) -> i32 {
    let device = acpi_companion(dev);
    if device.is_null() {
        return 0;
    }

    acpi_remove_notify_handler(
        acpi_device_handle(device),
        ACPI_DEVICE_NOTIFY,
        acpi_processor_notify,
    );

    let pr = acpi_driver_data(device);
    if pr.is_null() {
        return 0;
    }
    acpi_processor_power_exit(pr);
    acpi_cppc_processor_exit(pr);
    acpi_processor_thermal_exit(pr, device);
    0
}

pub static mut acpi_processor_cpufreq_init: bool = false;

unsafe fn acpi_processor_notifier(
    _nb: *mut notifier_block,
    event: usize,
    data: *mut core::ffi::c_void,
) -> i32 {
    let policy = data as *mut cpufreq_policy;
    if event == CPUFREQ_CREATE_POLICY {
        acpi_thermal_cpufreq_init(policy);
        acpi_processor_ppc_init(policy);
    } else if event == CPUFREQ_REMOVE_POLICY {
        acpi_processor_ppc_exit(policy);
        acpi_thermal_cpufreq_exit(policy);
    }
    0
}

static mut acpi_processor_notifier_block: notifier_block = notifier_block { _opaque: [] };

unsafe extern "C" fn acpi_processor_init_invariance_cppc() {}

static mut hp_online: i32 = 0;

unsafe extern "C" fn acpi_processor_driver_init() -> i32 {
    let mut result = 0;
    if acpi_disabled() {
        return 0;
    }

    if cpufreq_register_notifier(&mut acpi_processor_notifier_block, CPUFREQ_POLICY_NOTIFIER) == 0 {
        acpi_processor_cpufreq_init = true;
        acpi_processor_ignore_ppc_init();
    }

    acpi_processor_register_idle_driver();
    result = driver_register(&mut acpi_processor_driver);
    if result < 0 {
        acpi_processor_unregister_idle_driver();
        if acpi_processor_cpufreq_init {
            cpufreq_unregister_notifier(&mut acpi_processor_notifier_block, CPUFREQ_POLICY_NOTIFIER);
            acpi_processor_cpufreq_init = false;
        }
        return result;
    }

    result = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "acpi/cpu-drv:online", acpi_soft_cpu_online, None);
    if result < 0 {
        driver_unregister(&mut acpi_processor_driver);
        acpi_processor_unregister_idle_driver();
        if acpi_processor_cpufreq_init {
            cpufreq_unregister_notifier(&mut acpi_processor_notifier_block, CPUFREQ_POLICY_NOTIFIER);
            acpi_processor_cpufreq_init = false;
        }
        return result;
    }
    hp_online = result;
    cpuhp_setup_state_nocalls(CPUHP_ACPI_CPUDRV_DEAD, "acpi/cpu-drv:dead", None, Some(acpi_soft_cpu_dead));
    acpi_processor_throttling_init();
    acpi_processor_init_invariance_cppc();
    acpi_idle_rescan_dead_smt_siblings();
    0
}

unsafe extern "C" fn acpi_processor_driver_exit() {
    if acpi_disabled() {
        return;
    }
    if acpi_processor_cpufreq_init {
        cpufreq_unregister_notifier(&mut acpi_processor_notifier_block, CPUFREQ_POLICY_NOTIFIER);
        acpi_processor_cpufreq_init = false;
    }
    cpuhp_remove_state_nocalls(hp_online);
    cpuhp_remove_state_nocalls(CPUHP_ACPI_CPUDRV_DEAD);
    driver_unregister(&mut acpi_processor_driver);
    acpi_processor_unregister_idle_driver();
}

// module_init(acpi_processor_driver_init);
// module_exit(acpi_processor_driver_exit);
// MODULE_ALIAS("processor");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
