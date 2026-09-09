// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2011 Freescale Semiconductor, Inc.
 * Copyright 2011 Linaro Ltd.
 */

// Kernel dependencies supplied by the surrounding translation unit/build.

extern "C" {
    static mut scu_base: *mut core::ffi::c_void;
    fn imx_set_cpu_jump(cpu: u32, entry: unsafe extern "C" fn());
    fn v7_secondary_startup();
    fn imx_enable_cpu(cpu: u32, enable: bool);
    fn scu_get_core_count(base: *mut core::ffi::c_void) -> i32;
    fn set_cpu_possible(cpu: i32, possible: bool);
    fn scu_enable(base: *mut core::ffi::c_void);
    fn sync_cache_w(addr: *mut u32);
    fn imx_cpu_die(cpu: u32);
    fn imx_cpu_kill(cpu: u32) -> i32;
    fn arch_send_wakeup_ipi_mask(mask: *const core::ffi::c_void);
    fn cpumask_of(cpu: u32) -> *const core::ffi::c_void;
    fn of_find_compatible_node(from: *mut device_node, type_: *const u8, compatible: *const u8) -> *mut device_node;
    fn of_iomap(node: *mut device_node, index: i32) -> *mut core::ffi::c_void;
    fn of_node_put(node: *mut device_node);
    fn bug_on(condition: bool);
    fn writel_relaxed(value: u32, addr: *mut core::ffi::c_void);
    fn iounmap(addr: *mut core::ffi::c_void);
    fn cpu_to_be32(value: u32) -> u32;
    fn secondary_startup();
    fn __pa_symbol(addr: unsafe extern "C" fn()) -> usize;
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map_desc {
    pub virtual_: usize,
    pub pfn: usize,
    pub length: usize,
    pub type_: u32,
}

#[repr(C)]
pub struct smp_operations {
    pub smp_init_cpus: Option<unsafe extern "C" fn()>,
    pub smp_prepare_cpus: Option<unsafe extern "C" fn(u32)>,
    pub smp_boot_secondary: Option<unsafe extern "C" fn(u32, *mut task_struct) -> i32>,
    // CONFIG_HOTPLUG_CPU fields, when enabled by the surrounding build.
    pub cpu_die: Option<unsafe extern "C" fn(u32)>,
    pub cpu_kill: Option<unsafe extern "C" fn(u32) -> i32>,
}

pub const NR_CPUS: i32 = 4;
pub const SZ_4K: usize = 0x1000;
pub const MT_DEVICE: u32 = 0;
pub const DCFG_CCSR_SCRATCHRW1: usize = 0x200;

pub static mut g_diag_reg: u32 = 0;
static mut scu_io_desc: map_desc = map_desc {
    virtual_: 0,
    pfn: 0,
    length: SZ_4K,
    type_: MT_DEVICE,
};

extern "C" {
    fn iotable_init(desc: *mut map_desc, number: usize);
    fn imx_io_p2v(addr: usize) -> usize;
    fn __phys_to_pfn(addr: usize) -> usize;
    fn imx_io_address(addr: usize) -> *mut core::ffi::c_void;
}

pub unsafe extern "C" fn imx_scu_map_io() {
    let mut base: usize;
    core::arch::asm!("mrc p15, 4, {0}, c15, c0, 0", out(reg) base);

    scu_io_desc.virtual_ = imx_io_p2v(base);
    scu_io_desc.pfn = __phys_to_pfn(base);
    iotable_init(&mut scu_io_desc, 1);

    scu_base = imx_io_address(base);
}

unsafe extern "C" fn imx_boot_secondary(cpu: u32, _idle: *mut task_struct) -> i32 {
    imx_set_cpu_jump(cpu, v7_secondary_startup);
    imx_enable_cpu(cpu, true);
    0
}

unsafe extern "C" fn imx_smp_init_cpus() {
    let ncores = scu_get_core_count(scu_base);
    for i in ncores..NR_CPUS {
        set_cpu_possible(i, false);
    }
}

pub unsafe extern "C" fn imx_smp_prepare() {
    scu_enable(scu_base);
}

unsafe extern "C" fn imx_smp_prepare_cpus(_max_cpus: u32) {
    imx_smp_prepare();
    core::arch::asm!("mrc p15, 0, {0}, c15, c0, 1", out(reg) g_diag_reg, options(nostack));
    sync_cache_w(&mut g_diag_reg);
}

pub static imx_smp_ops: smp_operations = smp_operations {
    smp_init_cpus: Some(imx_smp_init_cpus),
    smp_prepare_cpus: Some(imx_smp_prepare_cpus),
    smp_boot_secondary: Some(imx_boot_secondary),
    cpu_die: Some(imx_cpu_die),
    cpu_kill: Some(imx_cpu_kill),
};

unsafe extern "C" fn imx7_smp_init_cpus() {
    let mut ncores = 0;
    // for_each_of_cpu_node(np)
    // The device-tree iteration is supplied by the surrounding kernel bindings.
    extern "C" {
        fn for_each_of_cpu_node_count() -> i32;
    }
    ncores = for_each_of_cpu_node_count();
    for i in ncores..NR_CPUS {
        set_cpu_possible(i, false);
    }
}

pub static imx7_smp_ops: smp_operations = smp_operations {
    smp_init_cpus: Some(imx7_smp_init_cpus),
    smp_prepare_cpus: None,
    smp_boot_secondary: Some(imx_boot_secondary),
    cpu_die: Some(imx_cpu_die),
    cpu_kill: Some(imx_cpu_kill),
};

unsafe extern "C" fn ls1021a_boot_secondary(cpu: u32, _idle: *mut task_struct) -> i32 {
    arch_send_wakeup_ipi_mask(cpumask_of(cpu));
    0
}

unsafe extern "C" fn ls1021a_smp_prepare_cpus(_max_cpus: u32) {
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"fsl,ls1021a-dcfg\0".as_ptr());
    let dcfg_base = of_iomap(np, 0);
    of_node_put(np);
    bug_on(dcfg_base.is_null());

    let paddr = __pa_symbol(secondary_startup);
    writel_relaxed(cpu_to_be32(paddr as u32), dcfg_base.add(DCFG_CCSR_SCRATCHRW1));
    iounmap(dcfg_base);
}

pub static ls1021a_smp_ops: smp_operations = smp_operations {
    smp_init_cpus: None,
    smp_prepare_cpus: Some(ls1021a_smp_prepare_cpus),
    smp_boot_secondary: Some(ls1021a_boot_secondary),
    cpu_die: None,
    cpu_kill: None,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
