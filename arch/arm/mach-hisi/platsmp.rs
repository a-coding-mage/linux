// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013 Linaro Ltd.
 * Copyright (c) 2013 HiSilicon Limited.
 * Based on arch/arm/mach-vexpress/platsmp.c, Copyright (C) 2002 ARM Ltd.
 */

// Linux kernel dependencies supplied externally.

const HIX5HD2_BOOT_ADDRESS: usize = 0xffff0000;

static mut ctrl_base: *mut core::ffi::c_void = core::ptr::null_mut();

extern "C" {
    fn cpu_logical_map(cpu: i32) -> i32;
    fn writel_relaxed(value: u32, address: *mut core::ffi::c_void);
    fn readl_relaxed(address: *mut core::ffi::c_void) -> u32;
    fn __pa_symbol(address: *const core::ffi::c_void) -> usize;
    fn scu_a9_has_base() -> bool;
    fn scu_a9_get_base() -> usize;
    fn ioremap(address: usize, size: usize) -> *mut core::ffi::c_void;
    fn iounmap(address: *mut core::ffi::c_void);
    fn scu_enable(address: *mut core::ffi::c_void);
    fn pr_err(message: *const core::ffi::c_char, ...);
    fn of_find_compatible_node(
        from: *mut device_node,
        type_: *const core::ffi::c_char,
        compatible: *const core::ffi::c_char,
    ) -> *mut device_node;
    fn of_iomap(node: *mut device_node, index: i32) -> *mut core::ffi::c_void;
    fn of_node_put(node: *mut device_node);
    fn of_property_read_u32(
        node: *mut device_node,
        property: *const core::ffi::c_char,
        value: *mut u32,
    ) -> i32;
    fn hi3xxx_set_cpu(cpu: u32, enable: bool);
    fn hi3xxx_cpu_die(cpu: u32);
    fn hi3xxx_cpu_kill(cpu: u32) -> bool;
    fn secondary_startup();
    fn arch_send_wakeup_ipi_mask(mask: *const core::ffi::c_void);
    fn cpumask_of(cpu: u32) -> *const core::ffi::c_void;
    fn hix5hd2_set_cpu(cpu: u32, enable: bool);
    fn hix5hd2_cpu_die(cpu: u32);
    fn hip01_set_cpu(cpu: u32, enable: bool);
    fn phys_to_virt(address: usize) -> *mut core::ffi::c_void;
    fn barrier();
    fn warn_on(condition: bool) -> bool;
}

#[repr(C)]
struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
struct task_struct {
    _private: [u8; 0],
}

pub unsafe fn hi3xxx_set_cpu_jump(mut cpu: i32, jump_addr: *mut core::ffi::c_void) {
    cpu = cpu_logical_map(cpu);
    if cpu == 0 || ctrl_base.is_null() {
        return;
    }
    writel_relaxed(__pa_symbol(jump_addr), ctrl_base.add((cpu - 1) as usize * 4));
}

pub unsafe fn hi3xxx_get_cpu_jump(mut cpu: i32) -> u32 {
    cpu = cpu_logical_map(cpu);
    if cpu == 0 || ctrl_base.is_null() {
        return 0;
    }
    readl_relaxed(ctrl_base.add((cpu - 1) as usize * 4))
}

unsafe fn hisi_enable_scu_a9() {
    let mut base: usize = 0;
    let mut scu_base: *mut core::ffi::c_void = core::ptr::null_mut();
    if scu_a9_has_base() {
        base = scu_a9_get_base();
        scu_base = ioremap(base, 0x1000);
        if scu_base.is_null() {
            pr_err(b"ioremap(scu_base) failed\0".as_ptr() as _);
            return;
        }
        scu_enable(scu_base);
        iounmap(scu_base);
    }
}

unsafe fn hi3xxx_smp_prepare_cpus(_max_cpus: u32) {
    let mut np: *mut device_node = core::ptr::null_mut();
    let mut offset: u32 = 0;
    hisi_enable_scu_a9();
    if ctrl_base.is_null() {
        np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"hisilicon,sysctrl\0".as_ptr() as _);
        if np.is_null() { pr_err(b"failed to find hisilicon,sysctrl node\0".as_ptr() as _); return; }
        ctrl_base = of_iomap(np, 0);
        if ctrl_base.is_null() { of_node_put(np); pr_err(b"failed to map address\0".as_ptr() as _); return; }
        if of_property_read_u32(np, b"smp-offset\0".as_ptr() as _, &mut offset) < 0 {
            of_node_put(np); pr_err(b"failed to find smp-offset property\0".as_ptr() as _); return;
        }
        ctrl_base = ctrl_base.add(offset as usize);
        of_node_put(np);
    }
}

unsafe fn hi3xxx_boot_secondary(cpu: u32, _idle: *mut task_struct) -> i32 {
    hi3xxx_set_cpu(cpu, true);
    hi3xxx_set_cpu_jump(cpu as i32, secondary_startup as *mut _);
    arch_send_wakeup_ipi_mask(cpumask_of(cpu));
    0
}

unsafe fn hisi_common_smp_prepare_cpus(_max_cpus: u32) { hisi_enable_scu_a9(); }

unsafe fn hix5hd2_set_scu_boot_addr(start_addr: usize, jump_addr: usize) {
    let virt = ioremap(start_addr, 0x1000);
    writel_relaxed(0xe51ff004, virt);
    writel_relaxed(jump_addr as u32, virt.add(4));
    iounmap(virt);
}

unsafe fn hix5hd2_boot_secondary(cpu: u32, _idle: *mut task_struct) -> i32 {
    let jumpaddr = __pa_symbol(secondary_startup as *const _);
    hix5hd2_set_scu_boot_addr(HIX5HD2_BOOT_ADDRESS, jumpaddr);
    hix5hd2_set_cpu(cpu, true);
    arch_send_wakeup_ipi_mask(cpumask_of(cpu));
    0
}

const SC_SCTL_REMAP_CLR: u32 = 0x00000100;
const HIP01_BOOT_ADDRESS: usize = 0x80000000;
const REG_SC_CTRL: usize = 0x000;

unsafe fn hip01_set_boot_addr(start_addr: usize, jump_addr: usize) {
    let virt = phys_to_virt(start_addr);
    writel_relaxed(0xe51ff004, virt);
    writel_relaxed(jump_addr as u32, virt.add(4));
}

unsafe fn hip01_boot_secondary(cpu: u32, _idle: *mut task_struct) -> i32 {
    let jumpaddr = __pa_symbol(secondary_startup as *const _);
    hip01_set_boot_addr(HIP01_BOOT_ADDRESS, jumpaddr);
    let node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"hisilicon,hip01-sysctrl\0".as_ptr() as _);
    if node.is_null() { return -1; }
    ctrl_base = of_iomap(node, 0);
    of_node_put(node);
    let mut remap_reg_value = readl_relaxed(ctrl_base.add(REG_SC_CTRL));
    barrier();
    remap_reg_value |= SC_SCTL_REMAP_CLR;
    barrier();
    writel_relaxed(remap_reg_value, ctrl_base.add(REG_SC_CTRL));
    hip01_set_cpu(cpu, true);
    0
}

// CPU_METHOD_OF_DECLARE registrations:
// hi3xxx_smp: "hisilicon,hi3620-smp"
// hix5hd2_smp: "hisilicon,hix5hd2-smp"
// hip01_smp: "hisilicon,hip01-smp"

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
