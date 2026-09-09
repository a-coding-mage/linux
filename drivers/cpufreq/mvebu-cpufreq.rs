// SPDX-License-Identifier: GPL-2.0-only
/*
 * CPUFreq support for Armada 370/XP platforms.
 *
 * Copyright (C) 2012-2016 Marvell
 *
 * Yehuda Yitschak <yehuday@marvell.com>
 * Gregory Clement <gregory.clement@free-electrons.com>
 * Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
 */

// C dependencies supplied by the surrounding kernel translation unit.
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Resource {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Device {
    pub id: c_int,
}

#[repr(C)]
pub struct Clk {
    _private: [u8; 0],
}

extern "C" {
    fn of_machine_is_compatible(compat: *const c_char) -> c_int;
    fn of_find_compatible_node(
        from: *mut DeviceNode,
        ty: *const c_char,
        compatible: *const c_char,
    ) -> *mut DeviceNode;
    fn of_address_to_resource(
        node: *mut DeviceNode,
        index: c_uint,
        resource: *mut Resource,
    ) -> c_int;
    fn of_node_put(node: *mut DeviceNode);
    fn get_cpu_device(cpu: c_int) -> *mut Device;
    fn clk_get(device: *mut Device, id: *const c_char) -> *mut Clk;
    fn clk_get_rate(clk: *mut Clk) -> c_ulong;
    fn clk_put(clk: *mut Clk);
    fn dev_pm_opp_add(device: *mut Device, freq: c_ulong, u_volt: c_ulong) -> c_int;
    fn dev_pm_opp_remove(device: *mut Device, freq: c_ulong);
    fn dev_pm_opp_set_sharing_cpus(device: *mut Device, cpumask: *const c_void) -> c_int;
    fn cpumask_of(cpu: c_int) -> *const c_void;
    fn platform_device_register_simple(
        name: *const c_char,
        id: c_int,
        data: *const c_void,
        size: c_ulong,
    ) -> *mut c_void;
    fn for_each_present_cpu_next(cpu: c_int) -> c_int;
    fn pr_warn(message: *const c_char, ...);
    fn pr_err(message: *const c_char, ...);
    fn dev_err(device: *mut Device, message: *const c_char, ...);
    fn ptr_is_err<T>(pointer: *mut T) -> bool;
    fn ptr_err<T>(pointer: *mut T) -> c_int;
}

const FW_WARN: &[u8] = b"FW_WARN \\0";

// __init / device_initcall are build-system annotations and registration macros.
#[allow(dead_code)]
unsafe fn armada_xp_pmsu_cpufreq_init() -> c_int {
    let mut np: *mut DeviceNode;
    let mut res = core::mem::MaybeUninit::<Resource>::uninit();
    let mut ret: c_int;
    let mut cpu: c_int = for_each_present_cpu_next(-1);

    if of_machine_is_compatible(b"marvell,armadaxp\\0".as_ptr() as *const c_char) == 0 {
        return 0;
    }

    /*
     * In order to have proper cpufreq handling, we need to ensure
     * that the Device Tree description of the CPU clock includes
     * the definition of the PMU DFS registers. If not, we do not
     * register the clock notifier and the cpufreq driver. This
     * piece of code is only for compatibility with old Device
     * Trees.
     */
    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"marvell,armada-xp-cpu-clock\\0".as_ptr() as *const c_char,
    );
    if np.is_null() {
        return 0;
    }

    ret = of_address_to_resource(np, 1, res.as_mut_ptr());
    if ret != 0 {
        pr_warn(b"FW_WARN not enabling cpufreq, deprecated armada-xp-cpu-clock binding\\n\\0".as_ptr() as *const c_char);
        of_node_put(np);
        return 0;
    }

    of_node_put(np);

    /*
     * For each CPU, this loop registers the operating points
     * supported (which are the nominal CPU frequency and half of
     * it), and registers the clock notifier that will take care
     * of doing the PMSU part of a frequency transition.
     */
    while cpu >= 0 {
        let cpu_dev = get_cpu_device(cpu);
        let clk: *mut Clk;

        if cpu_dev.is_null() {
            pr_err(b"Cannot get CPU %d\\n\\0".as_ptr() as *const c_char, cpu);
            cpu = for_each_present_cpu_next(cpu);
            continue;
        }

        clk = clk_get(cpu_dev, core::ptr::null());
        if ptr_is_err(clk) {
            pr_err(b"Cannot get clock for CPU %d\\n\\0".as_ptr() as *const c_char, cpu);
            return ptr_err(clk);
        }

        ret = dev_pm_opp_add(cpu_dev, clk_get_rate(clk), 0);
        if ret != 0 {
            clk_put(clk);
            return ret;
        }

        ret = dev_pm_opp_add(cpu_dev, clk_get_rate(clk) / 2, 0);
        if ret != 0 {
            dev_pm_opp_remove(cpu_dev, clk_get_rate(clk));
            clk_put(clk);
            dev_err(cpu_dev, b"Failed to register OPPs\\n\\0".as_ptr() as *const c_char);
            return ret;
        }

        ret = dev_pm_opp_set_sharing_cpus(cpu_dev, cpumask_of((*cpu_dev).id));
        if ret != 0 {
            dev_err(cpu_dev, b"%s: failed to mark OPPs as shared: %d\\n\\0".as_ptr() as *const c_char, b"armada_xp_pmsu_cpufreq_init\\0".as_ptr(), ret);
        }
        clk_put(clk);
        cpu = for_each_present_cpu_next(cpu);
    }

    platform_device_register_simple(b"cpufreq-dt\\0".as_ptr() as *const c_char, -1, core::ptr::null(), 0);
    0
}

// device_initcall(armada_xp_pmsu_cpufreq_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
