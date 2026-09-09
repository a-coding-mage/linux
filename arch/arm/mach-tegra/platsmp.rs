// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mach-tegra/platsmp.c
 *
 *  Copyright (C) 2002 ARM Ltd.
 *  All Rights Reserved
 *
 *  Copyright (C) 2009 Palm
 *  All Rights Reserved
 */

// Kernel and Tegra dependencies supplied by the surrounding translation unit.

use core::ffi::c_void;

#[repr(C)]
pub struct cpumask_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct smp_operations {
    pub smp_prepare_cpus: Option<unsafe extern "C" fn(u32)>,
    pub smp_secondary_init: Option<unsafe extern "C" fn(u32)>,
    pub smp_boot_secondary: Option<unsafe extern "C" fn(u32, *mut task_struct) -> i32>,
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    pub cpu_kill: Option<unsafe extern "C" fn(u32) -> i32>,
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    pub cpu_die: Option<unsafe extern "C" fn(u32)>,
}

extern "C" {
    static mut tegra_cpu_init_mask: cpumask_t;
    fn cpumask_set_cpu(cpu: u32, mask: *mut cpumask_t);
    fn cpumask_test_cpu(cpu: u32, mask: *const cpumask_t) -> bool;
    fn cpu_logical_map(cpu: u32) -> u32;
    fn tegra_put_cpu_in_reset(cpu: u32);
    fn flowctrl_write_cpu_halt(cpu: u32, value: u32);
    fn tegra_enable_cpu_clock(cpu: u32);
    fn flowctrl_write_cpu_csr(cpu: u32, value: u32);
    fn tegra_cpu_out_of_reset(cpu: u32);
    fn jiffies() -> usize;
    fn msecs_to_jiffies(value: u32) -> usize;
    fn tegra_pmc_cpu_is_powered(cpu: u32) -> bool;
    fn udelay(value: u32);
    fn time_before(a: usize, b: usize) -> bool;
    fn tegra_pmc_cpu_power_on(cpu: u32) -> i32;
    fn tegra_pmc_cpu_remove_clamping(cpu: u32) -> i32;
    fn tegra_get_chip_id() -> u32;
    fn scu_a9_has_base() -> bool;
    fn scu_a9_get_base() -> usize;
    fn io_address(address: usize) -> *mut c_void;
    fn scu_enable(base: *mut c_void);
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    fn tegra_cpu_kill(cpu: u32) -> i32;
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    fn tegra_cpu_die(cpu: u32);
}

const FLOW_CTRL_WAITEVENT: u32 = 1 << 2;
const FLOW_CTRL_SCLK_RESUME: u32 = 1 << 4;
const TEGRA20: u32 = 0x20;
const TEGRA30: u32 = 0x30;
const TEGRA114: u32 = 0x114;
const TEGRA124: u32 = 0x124;

unsafe fn tegra_secondary_init(cpu: u32) {
    cpumask_set_cpu(cpu, &raw mut tegra_cpu_init_mask);
}

unsafe fn tegra20_boot_secondary(mut cpu: u32, _idle: *mut task_struct) -> i32 {
    cpu = cpu_logical_map(cpu);
    tegra_put_cpu_in_reset(cpu);
    flowctrl_write_cpu_halt(cpu, 0);
    tegra_enable_cpu_clock(cpu);
    flowctrl_write_cpu_csr(cpu, 0); // Clear flow controller CSR.
    tegra_cpu_out_of_reset(cpu);
    0
}

unsafe fn tegra30_boot_secondary(mut cpu: u32, _idle: *mut task_struct) -> i32 {
    let ret: i32;
    let mut timeout: usize;
    cpu = cpu_logical_map(cpu);
    tegra_put_cpu_in_reset(cpu);
    flowctrl_write_cpu_halt(cpu, 0);

    if cpumask_test_cpu(cpu, &raw const tegra_cpu_init_mask) {
        timeout = jiffies().wrapping_add(msecs_to_jiffies(50));
        loop {
            if tegra_pmc_cpu_is_powered(cpu) {
                break;
            }
            udelay(10);
            if !time_before(jiffies(), timeout) {
                break;
            }
        }
    }

    ret = tegra_pmc_cpu_power_on(cpu);
    if ret != 0 { return ret; }

    tegra_enable_cpu_clock(cpu);
    udelay(10);
    let ret = tegra_pmc_cpu_remove_clamping(cpu);
    if ret != 0 { return ret; }
    udelay(10);
    flowctrl_write_cpu_csr(cpu, 0); // Clear flow controller CSR.
    tegra_cpu_out_of_reset(cpu);
    0
}

unsafe fn tegra114_boot_secondary(mut cpu: u32, _idle: *mut task_struct) -> i32 {
    let mut ret = 0;
    cpu = cpu_logical_map(cpu);
    if cpumask_test_cpu(cpu, &raw const tegra_cpu_init_mask) {
        // Warm boot flow: the flow controller is in charge of each CPU's power state.
        flowctrl_write_cpu_csr(cpu, 1);
        flowctrl_write_cpu_halt(cpu, FLOW_CTRL_WAITEVENT | FLOW_CTRL_SCLK_RESUME);
    } else {
        // Cold boot flow: the CPU is powered up by toggling PMC directly.
        ret = tegra_pmc_cpu_power_on(cpu);
    }
    ret
}

unsafe fn tegra_boot_secondary(cpu: u32, idle: *mut task_struct) -> i32 {
    // Build-time CONFIG_ARCH_TEGRA_* conditions are represented by feature flags.
    if cfg!(feature = "CONFIG_ARCH_TEGRA_2x_SOC") && tegra_get_chip_id() == TEGRA20 { return tegra20_boot_secondary(cpu, idle); }
    if cfg!(feature = "CONFIG_ARCH_TEGRA_3x_SOC") && tegra_get_chip_id() == TEGRA30 { return tegra30_boot_secondary(cpu, idle); }
    if cfg!(feature = "CONFIG_ARCH_TEGRA_114_SOC") && tegra_get_chip_id() == TEGRA114 { return tegra114_boot_secondary(cpu, idle); }
    if cfg!(feature = "CONFIG_ARCH_TEGRA_124_SOC") && tegra_get_chip_id() == TEGRA124 { return tegra114_boot_secondary(cpu, idle); }
    -22 // -EINVAL
}

unsafe fn tegra_smp_prepare_cpus(_max_cpus: u32) {
    // Always mark the boot CPU (CPU0) as initialized.
    cpumask_set_cpu(0, &raw mut tegra_cpu_init_mask);
    if scu_a9_has_base() {
        scu_enable(io_address(scu_a9_get_base()));
    }
}

#[no_mangle]
pub static tegra_smp_ops: smp_operations = smp_operations {
    smp_prepare_cpus: Some(tegra_smp_prepare_cpus),
    smp_secondary_init: Some(tegra_secondary_init),
    smp_boot_secondary: Some(tegra_boot_secondary),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_kill: Some(tegra_cpu_kill),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_die: Some(tegra_cpu_die),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
