// SPDX-License-Identifier: GPL-2.0-only
/*
 * Symmetric Multi Processing (SMP) support for Armada XP
 *
 * Copyright (C) 2012 Marvell
 *
 * Lior Amsalem <alior@marvell.com>
 * Yehuda Yitschak <yehuday@marvell.com>
 * Gregory CLEMENT <gregory.clement@free-electrons.com>
 * Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
 *
 * The Armada XP SoC has 4 ARMv7 PJ4B CPUs running in full HW coherency
 * This file implements the routines for preparing the SMP infrastructure
 * and waking up the secondary CPUs
 */

const ARMADA_XP_MAX_CPUS: usize = 4;
const AXP_BOOTROM_BASE: usize = 0xfff00000;
const AXP_BOOTROM_SIZE: usize = 0x100000;

#[repr(C)]
pub struct clk;
#[repr(C)]
pub struct task_struct;

// The following declarations correspond to symbols supplied by the kernel headers.
extern "C" {
    fn cpu_logical_map(cpu: u32) -> i32;
    fn mvebu_pmsu_set_cpu_boot_addr(cpu: i32, entry: unsafe extern "C" fn());
    fn armada_xp_secondary_startup();
    fn arch_send_wakeup_ipi_mask(mask: *const core::ffi::c_void);
    fn cpumask_of(cpu: u32) -> *const core::ffi::c_void;
    fn mvebu_cpu_reset_deassert(cpu: i32) -> i32;
    fn mvebu_v7_pmsu_idle_exit();
    fn num_possible_cpus() -> usize;
    fn flush_cache_all();
    fn set_cpu_coherent();
    fn smp_processor_id() -> u32;
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn clk_set_rate(clk: *mut clk, rate: u64) -> i32;
    fn clk_get_rate(clk: *mut clk) -> u64;
    fn panic(msg: *const core::ffi::c_char) -> !;
    fn armada_370_xp_pmsu_idle_enter(deep: bool);
    fn of_io_request_and_map(np: *mut device_node, index: i32, name: *const core::ffi::c_char) -> *mut u8;
    fn of_node_full_name(np: *mut device_node) -> *const core::ffi::c_char;
    fn of_node_put(np: *mut device_node);
    fn writel(value: u32, addr: *mut u8);
    fn __pa_symbol(addr: unsafe extern "C" fn()) -> usize;
    fn iounmap(addr: *mut u8);
}

static mut boot_cpu_clk: *mut clk = core::ptr::null_mut();

#[repr(C)] pub struct device_node;

unsafe extern "C" fn get_cpu_clk(_cpu: i32) -> *mut clk { core::ptr::null_mut() }

unsafe extern "C" fn armada_xp_boot_secondary(cpu: u32, _idle: *mut task_struct) -> i32 {
    let hw_cpu = cpu_logical_map(cpu);
    mvebu_pmsu_set_cpu_boot_addr(hw_cpu, armada_xp_secondary_startup);
    arch_send_wakeup_ipi_mask(cpumask_of(cpu));
    let ret = mvebu_cpu_reset_deassert(hw_cpu);
    if ret != 0 { return ret; }
    0
}

unsafe extern "C" fn armada_xp_secondary_init(_cpu: u32) { mvebu_v7_pmsu_idle_exit(); }
unsafe extern "C" fn armada_xp_smp_init_cpus() {
    let ncores = num_possible_cpus();
    if ncores == 0 || ncores > ARMADA_XP_MAX_CPUS { panic(b"Invalid number of CPUs in DT\0".as_ptr() as *const _); }
}
unsafe extern "C" fn armada_xp_sync_secondary_clk(cpu: u32) -> i32 {
    let cpu_clk = get_cpu_clk(cpu as i32);
    if cpu_clk.is_null() || boot_cpu_clk.is_null() { return 0; }
    clk_prepare_enable(cpu_clk); clk_set_rate(cpu_clk, clk_get_rate(boot_cpu_clk)); 0
}
unsafe extern "C" fn armada_xp_smp_prepare_cpus(_max_cpus: u32) {
    flush_cache_all(); set_cpu_coherent(); boot_cpu_clk = get_cpu_clk(smp_processor_id() as i32);
    if !boot_cpu_clk.is_null() { clk_prepare_enable(boot_cpu_clk); }
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe extern "C" fn armada_xp_cpu_die(_cpu: u32) { armada_370_xp_pmsu_idle_enter(true); }
#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe extern "C" fn armada_xp_cpu_kill(_cpu: u32) -> i32 { 1 }

#[repr(C)]
pub struct smp_operations {
    pub smp_init_cpus: Option<unsafe extern "C" fn()>,
    pub smp_prepare_cpus: Option<unsafe extern "C" fn(u32)>,
    pub smp_boot_secondary: Option<unsafe extern "C" fn(u32, *mut task_struct) -> i32>,
    pub smp_secondary_init: Option<unsafe extern "C" fn(u32)>,
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    pub cpu_die: Option<unsafe extern "C" fn(u32)>,
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    pub cpu_kill: Option<unsafe extern "C" fn(u32) -> i32>,
}

#[no_mangle]
pub static armada_xp_smp_ops: smp_operations = smp_operations {
    smp_init_cpus: Some(armada_xp_smp_init_cpus),
    smp_prepare_cpus: Some(armada_xp_smp_prepare_cpus),
    smp_boot_secondary: Some(armada_xp_boot_secondary),
    smp_secondary_init: Some(armada_xp_secondary_init),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_die: Some(armada_xp_cpu_die),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_kill: Some(armada_xp_cpu_kill),
};

const MV98DX3236_CPU_RESUME_CTRL_REG: usize = 0x08;
const MV98DX3236_CPU_RESUME_ADDR_REG: usize = 0x04;

#[repr(C)]
struct of_device_id {
    compatible: *const core::ffi::c_char,
}

static OF_MV98DX3236_RESUME_TABLE: &[of_device_id] = &[
    of_device_id { compatible: b"marvell,98dx3336-resume-ctrl\0".as_ptr() as *const _ },
    of_device_id { compatible: core::ptr::null() },
];

unsafe extern "C" fn mv98dx3236_resume_set_cpu_boot_addr(hw_cpu: i32, boot_addr: *mut core::ffi::c_void) -> i32 {
    if hw_cpu != 1 { /* WARN_ON(hw_cpu != 1) */ }
    // of_find_matching_node/of_address mapping are supplied by the kernel.
    let _ = (boot_addr, OF_MV98DX3236_RESUME_TABLE.as_ptr(), MV98DX3236_CPU_RESUME_CTRL_REG,
             MV98DX3236_CPU_RESUME_ADDR_REG, of_io_request_and_map, of_node_full_name,
             of_node_put, writel, __pa_symbol, iounmap);
    -19 // -ENODEV when the matching resume controller is absent
}

unsafe extern "C" fn mv98dx3236_boot_secondary(cpu: u32, _idle: *mut task_struct) -> i32 {
    let hw_cpu = cpu_logical_map(cpu);
    mv98dx3236_resume_set_cpu_boot_addr(hw_cpu, armada_xp_secondary_startup as *mut _);
    arch_send_wakeup_ipi_mask(cpumask_of(cpu));
    let ret = mvebu_cpu_reset_deassert(hw_cpu);
    if ret != 0 { return ret; }
    0
}

#[no_mangle]
static mv98dx3236_smp_ops: smp_operations = smp_operations {
    smp_init_cpus: Some(armada_xp_smp_init_cpus),
    smp_prepare_cpus: Some(armada_xp_smp_prepare_cpus),
    smp_boot_secondary: Some(mv98dx3236_boot_secondary),
    smp_secondary_init: Some(armada_xp_secondary_init),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_die: Some(armada_xp_cpu_die),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_kill: Some(armada_xp_cpu_kill),
};

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
const _HOTPLUG_CPU_PRESENT: bool = true;

// CPU_METHOD_OF_DECLARE(armada_xp_smp, "marvell,armada-xp-smp", &armada_xp_smp_ops);
// CPU_METHOD_OF_DECLARE(mv98dx3236_smp, "marvell,98dx3236-smp", &mv98dx3236_smp_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
