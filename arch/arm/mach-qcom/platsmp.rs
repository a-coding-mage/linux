// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (C) 2002 ARM Ltd.
 *  All Rights Reserved
 *  Copyright (c) 2010, Code Aurora Forum. All rights reserved.
 *  Copyright (c) 2014 The Linux Foundation. All rights reserved.
 */

const VDD_SC1_ARRAY_CLAMP_GFS_CTL: usize = 0x35a0;
const SCSS_CPU1CORE_RESET: usize = 0x2d80;
const SCSS_DBG_STATUS_CORE_PWRDUP: usize = 0x2e64;

const APCS_CPU_PWR_CTL: usize = 0x04;
const PLL_CLAMP: u32 = 1 << 8;
const CORE_PWRD_UP: u32 = 1 << 7;
const COREPOR_RST: u32 = 1 << 5;
const CORE_RST: u32 = 1 << 4;
const L2DT_SLP: u32 = 1 << 3;
const CORE_MEM_CLAMP: u32 = 1 << 1;
const CLAMP: u32 = 1 << 0;

const APC_PWR_GATE_CTL: usize = 0x14;
const BHS_CNT_SHIFT: u32 = 24;
const LDO_PWR_DWN_SHIFT: u32 = 16;
const LDO_BYP_SHIFT: u32 = 8;
const BHS_SEG_SHIFT: u32 = 1;
const BHS_EN: u32 = 1 << 0;

const APCS_SAW2_VCTL: usize = 0x14;
const APCS_SAW2_2_VCTL: usize = 0x1c;

#[repr(C)]
pub struct DeviceNode { _private: [u8; 0] }
#[repr(C)]
pub struct TaskStruct { _private: [u8; 0] }

extern "C" {
    fn secondary_startup_arm();
    fn of_find_compatible_node(from: *mut DeviceNode, ty: *mut core::ffi::c_char, compatible: *const core::ffi::c_char) -> *mut DeviceNode;
    fn of_iomap(node: *mut DeviceNode, index: i32) -> *mut u8;
    fn of_node_put(node: *mut DeviceNode);
    fn of_get_cpu_node(cpu: u32, thread: *mut u32) -> *mut DeviceNode;
    fn of_parse_phandle(node: *mut DeviceNode, name: *const core::ffi::c_char, index: i32) -> *mut DeviceNode;
    fn iounmap(addr: *mut u8);
    fn writel(value: u32, addr: *mut u8);
    fn writel_relaxed(value: u32, addr: *mut u8);
    fn mb();
    fn udelay(usecs: u32);
    fn ndelay(nsecs: u32);
    fn arch_send_wakeup_ipi_mask(mask: *const core::ffi::c_void);
    fn cpumask_of(cpu: u32) -> *const core::ffi::c_void;
    fn smp_processor_id() -> u32;
    fn set_cpu_present(cpu: u32, present: bool);
    fn qcom_scm_set_cold_boot_addr(addr: unsafe extern "C" fn()) -> i32;
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn pr_warn(fmt: *const core::ffi::c_char, ...);
    fn wfi();
}

const ENXIO: i32 = 6;
const ENOMEM: i32 = 12;
const ENODEV: i32 = 19;

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn qcom_cpu_die(_cpu: u32) { wfi(); }

unsafe fn scss_release_secondary(_cpu: u32) -> i32 {
    let node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), b"qcom,gcc-msm8660\0".as_ptr() as *const _);
    if node.is_null() { pr_err(b"%s: can't find node\n\0".as_ptr() as *const _, b"scss_release_secondary\0".as_ptr() as *const _); return -ENXIO; }
    let base = of_iomap(node, 0); of_node_put(node);
    if base.is_null() { return -ENOMEM; }
    writel_relaxed(0, base.add(VDD_SC1_ARRAY_CLAMP_GFS_CTL));
    writel_relaxed(0, base.add(SCSS_CPU1CORE_RESET));
    writel_relaxed(3, base.add(SCSS_DBG_STATUS_CORE_PWRDUP)); mb(); iounmap(base); 0
}

unsafe fn cortex_a7_release_secondary(cpu: u32) -> i32 {
    let mut ret = 0; let cpu_node = of_get_cpu_node(cpu, core::ptr::null_mut());
    if cpu_node.is_null() { return -ENODEV; }
    let acc_node = of_parse_phandle(cpu_node, b"qcom,acc\0".as_ptr() as *const _, 0);
    if acc_node.is_null() { ret = -ENODEV; of_node_put(cpu_node); return ret; }
    let reg = of_iomap(acc_node, 0);
    if reg.is_null() { ret = -ENOMEM; of_node_put(acc_node); of_node_put(cpu_node); return ret; }
    let mut reg_val = CORE_RST | COREPOR_RST | CLAMP | CORE_MEM_CLAMP;
    writel(reg_val, reg.add(APCS_CPU_PWR_CTL));
    writel(BHS_EN | (0x10 << BHS_CNT_SHIFT), reg.add(APC_PWR_GATE_CTL)); udelay(2);
    reg_val &= !CORE_MEM_CLAMP; writel(reg_val, reg.add(APCS_CPU_PWR_CTL));
    reg_val |= L2DT_SLP; writel(reg_val, reg.add(APCS_CPU_PWR_CTL)); udelay(2);
    reg_val = (reg_val | (1 << 17)) & !CLAMP; writel(reg_val, reg.add(APCS_CPU_PWR_CTL)); udelay(2);
    reg_val &= !(CORE_RST | COREPOR_RST); writel(reg_val, reg.add(APCS_CPU_PWR_CTL));
    reg_val |= CORE_PWRD_UP; writel(reg_val, reg.add(APCS_CPU_PWR_CTL)); iounmap(reg);
    of_node_put(acc_node); of_node_put(cpu_node); ret
}

unsafe fn kpssv1_release_secondary(cpu: u32) -> i32 {
    let cpu_node = of_get_cpu_node(cpu, core::ptr::null_mut()); if cpu_node.is_null() { return -ENODEV; }
    let acc_node = of_parse_phandle(cpu_node, b"qcom,acc\0".as_ptr() as *const _, 0); if acc_node.is_null() { of_node_put(cpu_node); return -ENODEV; }
    let saw_node = of_parse_phandle(cpu_node, b"qcom,saw\0".as_ptr() as *const _, 0); if saw_node.is_null() { of_node_put(acc_node); of_node_put(cpu_node); return -ENODEV; }
    let reg = of_iomap(acc_node, 0); if reg.is_null() { of_node_put(saw_node); of_node_put(acc_node); of_node_put(cpu_node); return -ENOMEM; }
    let saw_reg = of_iomap(saw_node, 0); if saw_reg.is_null() { iounmap(reg); of_node_put(saw_node); of_node_put(acc_node); of_node_put(cpu_node); return -ENOMEM; }
    writel_relaxed(0xA4, saw_reg.add(APCS_SAW2_VCTL)); mb(); udelay(512);
    let mut val = PLL_CLAMP | L2DT_SLP | CLAMP; writel_relaxed(val, reg.add(APCS_CPU_PWR_CTL)); val &= !L2DT_SLP; writel_relaxed(val, reg.add(APCS_CPU_PWR_CTL)); mb(); ndelay(300);
    val |= COREPOR_RST; writel_relaxed(val, reg.add(APCS_CPU_PWR_CTL)); mb(); udelay(2); val &= !CLAMP; writel_relaxed(val, reg.add(APCS_CPU_PWR_CTL)); mb(); udelay(2); val &= !COREPOR_RST; writel_relaxed(val, reg.add(APCS_CPU_PWR_CTL)); mb(); udelay(100); val |= CORE_PWRD_UP; writel_relaxed(val, reg.add(APCS_CPU_PWR_CTL)); mb();
    iounmap(saw_reg); iounmap(reg); of_node_put(saw_node); of_node_put(acc_node); of_node_put(cpu_node); 0
}

unsafe fn kpssv2_release_secondary(cpu: u32) -> i32 {
    let cpu_node = of_get_cpu_node(cpu, core::ptr::null_mut()); if cpu_node.is_null() { return -ENODEV; }
    let acc_node = of_parse_phandle(cpu_node, b"qcom,acc\0".as_ptr() as *const _, 0); if acc_node.is_null() { of_node_put(cpu_node); return -ENODEV; }
    let l2_node = of_parse_phandle(cpu_node, b"next-level-cache\0".as_ptr() as *const _, 0); if l2_node.is_null() { of_node_put(acc_node); of_node_put(cpu_node); return -ENODEV; }
    let saw_node = of_parse_phandle(l2_node, b"qcom,saw\0".as_ptr() as *const _, 0); if saw_node.is_null() { of_node_put(l2_node); of_node_put(acc_node); of_node_put(cpu_node); return -ENODEV; }
    let reg = of_iomap(acc_node, 0); if reg.is_null() { of_node_put(saw_node); of_node_put(l2_node); of_node_put(acc_node); of_node_put(cpu_node); return -ENOMEM; }
    let l2_saw_base = of_iomap(saw_node, 0); if l2_saw_base.is_null() { iounmap(reg); of_node_put(saw_node); of_node_put(l2_node); of_node_put(acc_node); of_node_put(cpu_node); return -ENOMEM; }
    let mut reg_val = (64 << BHS_CNT_SHIFT) | (0x3f << LDO_PWR_DWN_SHIFT) | BHS_EN; writel_relaxed(reg_val, reg.add(APC_PWR_GATE_CTL)); mb(); udelay(1); reg_val |= 0x3f << BHS_SEG_SHIFT; writel_relaxed(reg_val, reg.add(APC_PWR_GATE_CTL)); mb(); udelay(1); reg_val |= 0x3f << LDO_BYP_SHIFT; writel_relaxed(reg_val, reg.add(APC_PWR_GATE_CTL)); writel_relaxed(0x10003, l2_saw_base.add(APCS_SAW2_2_VCTL)); mb(); udelay(50);
    reg_val = COREPOR_RST | CLAMP; writel_relaxed(reg_val, reg.add(APCS_CPU_PWR_CTL)); mb(); udelay(2); reg_val &= !CLAMP; writel_relaxed(reg_val, reg.add(APCS_CPU_PWR_CTL)); mb(); udelay(2); reg_val &= !COREPOR_RST; writel_relaxed(reg_val, reg.add(APCS_CPU_PWR_CTL)); mb(); reg_val |= CORE_PWRD_UP; writel_relaxed(reg_val, reg.add(APCS_CPU_PWR_CTL)); mb();
    iounmap(l2_saw_base); iounmap(reg); of_node_put(saw_node); of_node_put(l2_node); of_node_put(acc_node); of_node_put(cpu_node); 0
}

static mut cold_boot_done: [i32; 256] = [0; 256];
unsafe fn qcom_boot_secondary(cpu: u32, func: unsafe fn(u32) -> i32) -> i32 { let mut ret = 0; if cold_boot_done[cpu as usize] == 0 { ret = func(cpu); if ret == 0 { cold_boot_done[cpu as usize] = 1; } } arch_send_wakeup_ipi_mask(cpumask_of(cpu)); ret }
unsafe fn msm8660_boot_secondary(cpu: u32, _idle: *mut TaskStruct) -> i32 { qcom_boot_secondary(cpu, scss_release_secondary) }
unsafe fn cortex_a7_boot_secondary(cpu: u32, _idle: *mut TaskStruct) -> i32 { qcom_boot_secondary(cpu, cortex_a7_release_secondary) }
unsafe fn kpssv1_boot_secondary(cpu: u32, _idle: *mut TaskStruct) -> i32 { qcom_boot_secondary(cpu, kpssv1_release_secondary) }
unsafe fn kpssv2_boot_secondary(cpu: u32, _idle: *mut TaskStruct) -> i32 { qcom_boot_secondary(cpu, kpssv2_release_secondary) }

unsafe fn qcom_smp_prepare_cpus(_max_cpus: u32) {
    if qcom_scm_set_cold_boot_addr(secondary_startup_arm) != 0 {
        // for_each_present_cpu(cpu)
        // set_cpu_present(cpu, false) for every present CPU except smp_processor_id().
        pr_warn(b"Failed to set CPU boot address, disabling SMP\n\0".as_ptr() as *const _);
    }
}

// CPU_METHOD_OF_DECLARE registrations and CONFIG_HOTPLUG_CPU fields are build-system declarations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
