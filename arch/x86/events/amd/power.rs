// SPDX-License-Identifier: GPL-2.0-only
/*
 * Performance events - AMD Processor Power Reporting Mechanism
 *
 * Copyright (C) 2016 Advanced Micro Devices, Inc.
 *
 * Author: Huang Rui <ray.huang@amd.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

/* Event code: LSB 8 bits, passed in attr->config any other bit is reserved. */
const AMD_POWER_EVENT_MASK: u64 = 0xFF;

/* Accumulated power status counters. */
const AMD_POWER_EVENTSEL_PKG: u64 = 1;

/*
 * The ratio of compute unit power accumulator sample period to the
 * PTSC period.
 */
static mut CPU_PWR_SAMPLE_RATIO: u32 = 0;

/* Maximum accumulated power of a compute unit. */
static mut MAX_CU_ACC_POWER: u64 = 0;

/* Accumulated power represents the sum of each compute unit's (CU) power
 * consumption. On any core of each CU we read the total accumulated power from
 * MSR_F15H_CU_PWR_ACCUMULATOR. cpu_mask represents CPU bit map of all cores
 * which are picked to measure the power for the CUs they belong to.
 */
static mut CPU_MASK: cpumask_t = cpumask_t::new();

unsafe fn event_update(event: *mut perf_event) {
    let hwc: *mut hw_perf_event = &mut (*event).hw;
    let prev_pwr_acc = (*hwc).pwr_acc;
    let prev_ptsc = (*hwc).ptsc;
    let mut new_pwr_acc: u64 = 0;
    let mut new_ptsc: u64 = 0;
    rdmsrq(MSR_F15H_CU_PWR_ACCUMULATOR, &mut new_pwr_acc);
    rdmsrq(MSR_F15H_PTSC, &mut new_ptsc);

    /* Calculate the CU power consumption over a time period, the unit of
     * final value (delta) is micro-Watts. Then add it to the event count.
     */
    let mut delta = if new_pwr_acc < prev_pwr_acc {
        MAX_CU_ACC_POWER.wrapping_add(new_pwr_acc).wrapping_sub(prev_pwr_acc)
    } else {
        new_pwr_acc.wrapping_sub(prev_pwr_acc)
    };
    delta = delta.wrapping_mul((CPU_PWR_SAMPLE_RATIO as u64).wrapping_mul(1000));
    let tdelta = new_ptsc.wrapping_sub(prev_ptsc);
    delta /= tdelta;
    local64_add(delta, &mut (*event).count);
}

unsafe fn __pmu_event_start(event: *mut perf_event) {
    if warn_on_once((*event).hw.state & PERF_HES_STOPPED == 0) {
        return;
    }
    (*event).hw.state = 0;
    rdmsrq(MSR_F15H_PTSC, &mut (*event).hw.ptsc);
    rdmsrq(MSR_F15H_CU_PWR_ACCUMULATOR, &mut (*event).hw.pwr_acc);
}

unsafe fn pmu_event_start(event: *mut perf_event, _mode: i32) {
    __pmu_event_start(event);
}

unsafe fn pmu_event_stop(event: *mut perf_event, mode: i32) {
    let hwc = &mut (*event).hw;
    /* Mark event as deactivated and stopped. */
    if hwc.state & PERF_HES_STOPPED == 0 {
        hwc.state |= PERF_HES_STOPPED;
    }
    /* Check if software counter update is necessary. */
    if mode & PERF_EF_UPDATE != 0 && hwc.state & PERF_HES_UPTODATE == 0 {
        /* Drain the remaining delta count out of an event that we are disabling: */
        event_update(event);
        hwc.state |= PERF_HES_UPTODATE;
    }
}

unsafe fn pmu_event_add(event: *mut perf_event, mode: i32) -> i32 {
    (*event).hw.state = PERF_HES_UPTODATE | PERF_HES_STOPPED;
    if mode & PERF_EF_START != 0 {
        __pmu_event_start(event);
    }
    0
}

unsafe fn pmu_event_del(event: *mut perf_event, _flags: i32) {
    pmu_event_stop(event, PERF_EF_UPDATE);
}

unsafe fn pmu_event_init(event: *mut perf_event) -> i32 {
    let cfg = (*event).attr.config & AMD_POWER_EVENT_MASK;
    /* Only look at AMD power events. */
    if (*event).attr.type_ != PMU_CLASS.type_ { return -ENOENT; }
    /* Unsupported modes and filters. */
    if (*event).attr.sample_period != 0 { return -EINVAL; }
    if cfg != AMD_POWER_EVENTSEL_PKG { return -EINVAL; }
    0
}

unsafe fn pmu_event_read(event: *mut perf_event) { event_update(event); }

unsafe fn get_attr_cpumask(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    sysfs_emit(buf, "%*pbl\n", cpumask_pr_args(&CPU_MASK))
}

// DEVICE_ATTR(cpumask, S_IRUGO, get_attr_cpumask, NULL);
// EVENT_ATTR_STR(power-pkg, power_pkg, "event=0x01");
// EVENT_ATTR_STR(power-pkg.unit, power_pkg_unit, "mWatts");
// EVENT_ATTR_STR(power-pkg.scale, power_pkg_scale, "1.000000e-3");
// PMU_FORMAT_ATTR(event, "config:0-7");

unsafe fn power_cpu_exit(cpu: u32) -> i32 {
    if !cpumask_test_and_clear_cpu(cpu, &mut CPU_MASK) { return 0; }
    let target = cpumask_any_but(topology_sibling_cpumask(cpu), cpu);
    if target < NR_CPUMASK_BITS {
        cpumask_set_cpu(target, &mut CPU_MASK);
        perf_pmu_migrate_context(&mut PMU_CLASS, cpu, target);
    }
    0
}

unsafe fn power_cpu_init(cpu: u32) -> i32 {
    let target = cpumask_any_but(topology_sibling_cpumask(cpu), cpu);
    if target >= NR_CPUMASK_BITS { cpumask_set_cpu(cpu, &mut CPU_MASK); }
    0
}

// X86_MATCH_VENDOR_FAM(AMD, 0x15, NULL), {};

unsafe fn amd_power_pmu_init() -> i32 {
    if !x86_match_cpu() { return -ENODEV; }
    if !boot_cpu_has(X86_FEATURE_ACC_POWER) { return -ENODEV; }
    CPU_PWR_SAMPLE_RATIO = cpuid_ecx(0x80000007);
    if rdmsrq_safe(MSR_F15H_CU_MAX_PWR_ACCUMULATOR, &mut MAX_CU_ACC_POWER) != 0 {
        pr_err("Failed to read max compute unit power accumulator MSR\n");
        return -ENODEV;
    }
    cpuhp_setup_state(CPUHP_AP_PERF_X86_AMD_POWER_ONLINE,
        "perf/x86/amd/power:online", power_cpu_init, power_cpu_exit);
    let ret = perf_pmu_register(&mut PMU_CLASS, "power", -1);
    if warn_on(ret != 0) {
        pr_warn("AMD Power PMU registration failed\n");
        return ret;
    }
    pr_info("AMD Power PMU detected\n");
    ret
}

unsafe fn amd_power_pmu_exit() {
    cpuhp_remove_state_nocalls(CPUHP_AP_PERF_X86_AMD_POWER_ONLINE);
    perf_pmu_unregister(&mut PMU_CLASS);
}

// module_init!(amd_power_pmu_init);
// module_exit!(amd_power_pmu_exit);
// MODULE_AUTHOR!("Huang Rui <ray.huang@amd.com>");
// MODULE_DESCRIPTION!("AMD Processor Power Reporting Mechanism");
// MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
