// SPDX-License-Identifier: GPL-2.0
/*
 * SMP support for Allwinner SoCs
 *
 * Copyright (C) 2013 Maxime Ripard
 *
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 *
 * Based on code
 *  Copyright (C) 2012-2013 Allwinner Ltd.
 *
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

const fn cpucfg_cpu_pwr_clamp_status_reg(cpu: usize) -> usize { cpu * 0x40 + 0x64 }
const fn cpucfg_cpu_rst_ctrl_reg(cpu: usize) -> usize { (cpu + 1) * 0x40 }
const fn cpucfg_cpu_ctrl_reg(cpu: usize) -> usize { (cpu + 1) * 0x40 + 0x04 }
const fn cpucfg_cpu_status_reg(cpu: usize) -> usize { (cpu + 1) * 0x40 + 0x08 }
const CPUCFG_GEN_CTRL_REG: usize = 0x184;
const CPUCFG_PRIVATE0_REG: usize = 0x1a4;
const CPUCFG_PRIVATE1_REG: usize = 0x1a8;
const CPUCFG_DBG_CTL0_REG: usize = 0x1e0;
const CPUCFG_DBG_CTL1_REG: usize = 0x1e4;

const PRCM_CPU_PWROFF_REG: usize = 0x100;
const fn prcm_cpu_pwr_clamp_reg(cpu: usize) -> usize { cpu * 4 + 0x140 }

#[repr(C)]
pub struct DeviceNode { _private: [u8; 0] }
#[repr(C)]
pub struct TaskStruct { _private: [u8; 0] }
#[repr(C)]
pub struct SmpOperations {
    pub smp_prepare_cpus: Option<unsafe extern "C" fn(u32)>,
    pub smp_boot_secondary: Option<unsafe extern "C" fn(u32, *mut TaskStruct) -> i32>,
}

extern "C" {
    fn of_find_compatible_node(from: *mut DeviceNode, ty: *const i8, compatible: *const i8) -> *mut DeviceNode;
    fn of_iomap(node: *mut DeviceNode, index: i32) -> *mut u8;
    fn of_node_put(node: *mut DeviceNode);
    fn pr_err(fmt: *const i8, ...);
    fn writel(value: u32, address: *mut u8);
    fn readl(address: *mut u8) -> u32;
    fn spin_lock(lock: *mut u8);
    fn spin_unlock(lock: *mut u8);
    fn mdelay(milliseconds: u32);
    fn __pa_symbol(symbol: unsafe extern "C" fn());
    fn secondary_startup();
}

static mut cpucfg_membase: *mut u8 = core::ptr::null_mut();
static mut prcm_membase: *mut u8 = core::ptr::null_mut();
static mut cpu_lock: u8 = 0;

unsafe extern "C" fn sun6i_smp_prepare_cpus(_max_cpus: u32) {
    let mut node: *mut DeviceNode;

    node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"allwinner,sun6i-a31-prcm\0".as_ptr() as *const i8);
    if node.is_null() {
        pr_err(b"Missing A31 PRCM node in the device tree\n\0".as_ptr() as *const i8);
        return;
    }

    prcm_membase = of_iomap(node, 0);
    of_node_put(node);
    if prcm_membase.is_null() {
        pr_err(b"Couldn't map A31 PRCM registers\n\0".as_ptr() as *const i8);
        return;
    }

    node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"allwinner,sun6i-a31-cpuconfig\0".as_ptr() as *const i8);
    if node.is_null() {
        pr_err(b"Missing A31 CPU config node in the device tree\n\0".as_ptr() as *const i8);
        return;
    }

    cpucfg_membase = of_iomap(node, 0);
    of_node_put(node);
    if cpucfg_membase.is_null() {
        pr_err(b"Couldn't map A31 CPU config registers\n\0".as_ptr() as *const i8);
    }
}

unsafe extern "C" fn sun6i_smp_boot_secondary(cpu: u32, _idle: *mut TaskStruct) -> i32 {
    let mut reg: u32;
    let mut i: i32;
    if prcm_membase.is_null() || cpucfg_membase.is_null() { return -14; }
    spin_lock(&mut cpu_lock);
    writel(__pa_symbol(secondary_startup), cpucfg_membase.add(CPUCFG_PRIVATE0_REG));
    writel(0, cpucfg_membase.add(cpucfg_cpu_rst_ctrl_reg(cpu as usize)));
    reg = readl(cpucfg_membase.add(CPUCFG_GEN_CTRL_REG));
    writel(reg & !(1u32 << cpu), cpucfg_membase.add(CPUCFG_GEN_CTRL_REG));
    reg = readl(cpucfg_membase.add(CPUCFG_DBG_CTL1_REG));
    writel(reg & !(1u32 << cpu), cpucfg_membase.add(CPUCFG_DBG_CTL1_REG));
    i = 0;
    while i <= 8 {
        writel(0xffu32 >> i, prcm_membase.add(prcm_cpu_pwr_clamp_reg(cpu as usize)));
        i += 1;
    }
    mdelay(10);
    reg = readl(prcm_membase.add(PRCM_CPU_PWROFF_REG));
    writel(reg & !(1u32 << cpu), prcm_membase.add(PRCM_CPU_PWROFF_REG));
    mdelay(1);
    writel(3, cpucfg_membase.add(cpucfg_cpu_rst_ctrl_reg(cpu as usize)));
    reg = readl(cpucfg_membase.add(CPUCFG_DBG_CTL1_REG));
    writel(reg | (1u32 << cpu), cpucfg_membase.add(CPUCFG_DBG_CTL1_REG));
    spin_unlock(&mut cpu_lock);
    0
}

static sun6i_smp_ops: SmpOperations = SmpOperations { smp_prepare_cpus: Some(sun6i_smp_prepare_cpus), smp_boot_secondary: Some(sun6i_smp_boot_secondary) };
// CPU_METHOD_OF_DECLARE(sun6i_a31_smp, "allwinner,sun6i-a31", &sun6i_smp_ops);

unsafe extern "C" fn sun8i_smp_prepare_cpus(_max_cpus: u32) {
    let mut node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"allwinner,sun8i-a23-prcm\0".as_ptr() as *const i8);
    if node.is_null() { pr_err(b"Missing A23 PRCM node in the device tree\n\0".as_ptr() as *const i8); return; }
    prcm_membase = of_iomap(node, 0); of_node_put(node);
    if prcm_membase.is_null() { pr_err(b"Couldn't map A23 PRCM registers\n\0".as_ptr() as *const i8); return; }
    node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"allwinner,sun8i-a23-cpuconfig\0".as_ptr() as *const i8);
    if node.is_null() { pr_err(b"Missing A23 CPU config node in the device tree\n\0".as_ptr() as *const i8); return; }
    cpucfg_membase = of_iomap(node, 0); of_node_put(node);
    if cpucfg_membase.is_null() { pr_err(b"Couldn't map A23 CPU config registers\n\0".as_ptr() as *const i8); }
}

unsafe extern "C" fn sun8i_smp_boot_secondary(cpu: u32, _idle: *mut TaskStruct) -> i32 {
    if prcm_membase.is_null() || cpucfg_membase.is_null() { return -14; }
    spin_lock(&mut cpu_lock);
    writel(__pa_symbol(secondary_startup), cpucfg_membase.add(CPUCFG_PRIVATE0_REG));
    writel(0, cpucfg_membase.add(cpucfg_cpu_rst_ctrl_reg(cpu as usize)));
    let reg = readl(cpucfg_membase.add(CPUCFG_GEN_CTRL_REG));
    writel(reg & !(1u32 << cpu), cpucfg_membase.add(CPUCFG_GEN_CTRL_REG));
    let reg = readl(prcm_membase.add(PRCM_CPU_PWROFF_REG));
    writel(reg & !(1u32 << cpu), prcm_membase.add(PRCM_CPU_PWROFF_REG));
    mdelay(1);
    writel(3, cpucfg_membase.add(cpucfg_cpu_rst_ctrl_reg(cpu as usize)));
    spin_unlock(&mut cpu_lock);
    0
}

static sun8i_smp_ops: SmpOperations = SmpOperations { smp_prepare_cpus: Some(sun8i_smp_prepare_cpus), smp_boot_secondary: Some(sun8i_smp_boot_secondary) };
// CPU_METHOD_OF_DECLARE(sun8i_a23_smp, "allwinner,sun8i-a23", &sun8i_smp_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
