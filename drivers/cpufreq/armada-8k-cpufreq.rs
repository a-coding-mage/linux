// SPDX-License-Identifier: GPL-2.0+
/*
 * CPUFreq support for Armada 8K
 *
 * Copyright (C) 2018 Marvell
 *
 * Omri Itach <omrii@marvell.com>
 * Gregory Clement <gregory.clement@bootlin.com>
 */

// Dependency declarations and build-time configuration supplied by the kernel.

extern "C" {
    fn pr_warn(fmt: *const core::ffi::c_char, ...);
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn get_cpu_device(cpu: core::ffi::c_int) -> *mut device;
    fn clk_get(dev: *mut device, id: *const core::ffi::c_char) -> *mut clk;
    fn clk_put(clk: *mut clk);
    fn clk_is_match(a: *mut clk, b: *mut clk) -> bool;
    fn clk_get_rate(clk: *mut clk) -> u32;
    fn cpumask_set_cpu(cpu: core::ffi::c_int, mask: *mut cpumask);
    fn cpumask_copy(dst: *mut cpumask, src: *const cpumask);
    fn cpumask_clear(mask: *mut cpumask);
    fn cpumask_andnot(dst: *mut cpumask, a: *const cpumask, b: *const cpumask);
    fn num_possible_cpus() -> core::ffi::c_int;
    fn of_find_matching_node_and_match(node: *mut device_node, match_table: *const of_device_id, data: *mut *const of_device_id) -> *mut device_node;
    fn of_device_is_available(node: *mut device_node) -> bool;
    fn of_node_put(node: *mut device_node);
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_pm_opp_add(dev: *mut device, freq: u32, u_volt: u32) -> core::ffi::c_int;
    fn dev_pm_opp_remove(dev: *mut device, freq: u32);
    fn dev_pm_opp_set_sharing_cpus(dev: *mut device, mask: *const cpumask);
    fn platform_device_register_simple(name: *const core::ffi::c_char, id: core::ffi::c_int, res: *const core::ffi::c_void, num: u32) -> *mut platform_device;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut core::ffi::c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
}

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct cpumask { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
}

static mut ARMADA_8K_CPUFREQ_PDEV: *mut platform_device = core::ptr::null_mut();
static OPPS_DIV: [i32; 4] = [1, 2, 3, 4];

#[repr(C)]
pub struct freq_table {
    pub cpu_dev: *mut device,
    pub freq: [u32; 4],
}

// If the CPUs share the same clock, then they are in the same cluster.
unsafe fn armada_8k_get_sharing_cpus(cur_clk: *mut clk, cpumask: *mut cpumask) {
    let mut cpu = 0;
    while cpu < num_possible_cpus() {
        let cpu_dev = get_cpu_device(cpu);
        if cpu_dev.is_null() {
            pr_warn(b"Failed to get cpu%d device\0".as_ptr() as _, cpu);
            cpu += 1;
            continue;
        }
        let clk = clk_get(cpu_dev, core::ptr::null());
        if clk.is_null() {
            pr_warn(b"Cannot get clock for CPU %d\n\0".as_ptr() as _, cpu);
        } else {
            if clk_is_match(clk, cur_clk) { cpumask_set_cpu(cpu, cpumask); }
            clk_put(clk);
        }
        cpu += 1;
    }
}

unsafe fn armada_8k_add_opp(clk: *mut clk, cpu_dev: *mut device, freq_tables: *mut freq_table, opps_index: usize) -> core::ffi::c_int {
    let cur_frequency = clk_get_rate(clk);
    if cur_frequency == 0 {
        dev_err(cpu_dev, b"Failed to get clock rate for this CPU\n\0".as_ptr() as _);
        return -22;
    }
    (*freq_tables.add(opps_index)).cpu_dev = cpu_dev;
    for i in 0..OPPS_DIV.len() {
        let freq = cur_frequency / OPPS_DIV[i] as u32;
        let ret = dev_pm_opp_add(cpu_dev, freq, 0);
        if ret != 0 { return ret; }
        (*freq_tables.add(opps_index)).freq[i] = freq;
    }
    0
}

unsafe fn armada_8k_cpufreq_free_table(freq_tables: *mut freq_table) {
    let nb_cpus = num_possible_cpus();
    let mut opps_index = 0;
    while opps_index < nb_cpus {
        if (*freq_tables.add(opps_index as usize)).cpu_dev.is_null() { break; }
        for i in 0..OPPS_DIV.len() {
            let freq = (*freq_tables.add(opps_index as usize)).freq[i];
            if freq == 0 { break; }
            dev_pm_opp_remove((*freq_tables.add(opps_index as usize)).cpu_dev, freq);
        }
        opps_index += 1;
    }
    kfree(freq_tables.cast());
}

unsafe fn armada_8k_cpufreq_init() -> core::ffi::c_int {
    static mut CPUS: cpumask = cpumask { _private: [] };
    static mut SHARED_CPUS: cpumask = cpumask { _private: [] };
    let mut ret = 0;
    let mut opps_index = 0usize;
    let mut cpu;
    let nb_cpus = num_possible_cpus();
    let node = of_find_matching_node_and_match(core::ptr::null_mut(), ARMADA_8K_CPUFREQ_OF_MATCH.as_ptr(), core::ptr::null_mut());
    if node.is_null() || !of_device_is_available(node) { of_node_put(node); return -19; }
    of_node_put(node);
    let freq_tables = alloc_zeroed_freq_tables(nb_cpus as usize);
    if freq_tables.is_null() { return -12; }
    cpumask_copy(&mut CPUS, core::ptr::null());
    cpu = 0;
    while cpu < nb_cpus {
        let cpu_dev = get_cpu_device(cpu);
        if cpu_dev.is_null() { pr_err(b"Cannot get CPU %d\n\0".as_ptr() as _, cpu); cpu += 1; continue; }
        let clk = clk_get(cpu_dev, core::ptr::null());
        if clk.is_null() { pr_err(b"Cannot get clock for CPU %d\n\0".as_ptr() as _, cpu); ret = -1; break; }
        ret = armada_8k_add_opp(clk, cpu_dev, freq_tables, opps_index);
        if ret != 0 { clk_put(clk); break; }
        opps_index += 1;
        cpumask_clear(&mut SHARED_CPUS);
        armada_8k_get_sharing_cpus(clk, &mut SHARED_CPUS);
        dev_pm_opp_set_sharing_cpus(cpu_dev, &SHARED_CPUS);
        cpumask_andnot(&mut CPUS, &CPUS, &SHARED_CPUS);
        clk_put(clk);
        cpu += 1;
    }
    if ret != 0 { armada_8k_cpufreq_free_table(freq_tables); return ret; }
    ARMADA_8K_CPUFREQ_PDEV = platform_device_register_simple(b"cpufreq-dt\0".as_ptr() as _, -1, core::ptr::null(), 0);
    if ARMADA_8K_CPUFREQ_PDEV.is_null() { armada_8k_cpufreq_free_table(freq_tables); return -1; }
    platform_set_drvdata(ARMADA_8K_CPUFREQ_PDEV, freq_tables.cast());
    0
}

unsafe fn armada_8k_cpufreq_exit() {
    let freq_tables = platform_get_drvdata(ARMADA_8K_CPUFREQ_PDEV).cast::<freq_table>();
    platform_device_unregister(ARMADA_8K_CPUFREQ_PDEV);
    armada_8k_cpufreq_free_table(freq_tables);
}

extern "C" { static ARMADA_8K_CPUFREQ_OF_MATCH: [of_device_id; 3]; fn alloc_zeroed_freq_tables(n: usize) -> *mut freq_table; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
