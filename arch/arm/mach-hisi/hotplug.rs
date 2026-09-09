// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013 Linaro Ltd.
 * Copyright (c) 2013 HiSilicon Limited.
 */

use core::ffi::c_void;

type U32 = u32;
type Bool = bool;

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

extern "C" {
    fn writel_relaxed(value: U32, addr: *mut c_void);
    fn readl_relaxed(addr: *mut c_void) -> U32;
    fn udelay(usecs: U32);
    fn of_find_compatible_node(from: *mut DeviceNode, type_: *const i8, compatible: *const i8) -> *mut DeviceNode;
    fn of_iomap(node: *mut DeviceNode, index: i32) -> *mut c_void;
    fn of_node_put(node: *mut DeviceNode);
    fn flush_cache_all();
    fn phys_to_virt(addr: usize) -> *mut c_void;
    fn cpu_do_idle();
    fn panic(format: *const i8, ... ) -> !;
    fn jiffies_value() -> usize;
    fn msecs_to_jiffies(msecs: U32) -> usize;
    fn time_after(a: usize, b: usize) -> Bool;
}

extern "C" {
    fn hi3xxx_set_cpu_jump(cpu: U32, jump: *mut c_void);
    fn hi3xxx_get_cpu_jump(cpu: U32) -> *mut c_void;
}

/* Sysctrl registers in Hi3620 SoC */
const SCISOEN: usize = 0xc0;
const SCISODIS: usize = 0xc4;
const SCPERPWREN: usize = 0xd0;
const SCPERPWRDIS: usize = 0xd4;
const SCCPUCOREEN: usize = 0xf4;
const SCCPUCOREDIS: usize = 0xf8;
const SCPERCTRL0: usize = 0x200;
const SCCPURSTEN: usize = 0x410;
const SCCPURSTDIS: usize = 0x414;

const CPU2_ISO_CTRL: U32 = 1 << 5;
const CPU0_WFI_MASK_CFG: U32 = 1 << 28;
const CPU0_HPM_SRST_REQ_EN: U32 = 1 << 22;
const CPU0_DBG_SRST_REQ_EN: U32 = 1 << 12;
const CPU0_NEON_SRST_REQ_EN: U32 = 1 << 4;
const CPU0_SRST_REQ_EN: U32 = 1 << 0;

const HIX5HD2_PERI_CRG20: usize = 0x50;
const CRG20_CPU1_RESET: U32 = 1 << 17;
const HIX5HD2_PERI_PMC0: usize = 0x1000;
const PMC0_CPU1_WAIT_MTCOMS_ACK: U32 = 1 << 8;
const PMC0_CPU1_PMC_ENABLE: U32 = 1 << 7;
const PMC0_CPU1_POWERDOWN: U32 = 1 << 3;
const HIP01_PERI9: usize = 0x50;
const PERI9_CPU1_RESET: U32 = 1 << 1;

const HI3620_CTRL: i32 = 0;
const ERROR_CTRL: i32 = 1;

static mut ctrl_base: *mut c_void = core::ptr::null_mut();
static mut id: i32 = 0;

unsafe fn reg(offset: usize) -> *mut c_void {
    (ctrl_base as *mut u8).add(offset) as *mut c_void
}

unsafe fn set_cpu_hi3620(cpu: i32, enable: Bool) {
    let mut val: U32 = 0;
    if enable {
        if cpu == 2 || cpu == 3 { writel_relaxed(CPU2_ISO_CTRL << (cpu - 2), reg(SCPERPWREN)); }
        udelay(100);
        writel_relaxed(0x01 << cpu, reg(SCCPUCOREEN));
        val = CPU0_DBG_SRST_REQ_EN | CPU0_NEON_SRST_REQ_EN | CPU0_SRST_REQ_EN;
        writel_relaxed(val << cpu, reg(SCCPURSTDIS));
        val |= CPU0_HPM_SRST_REQ_EN;
        writel_relaxed(val << cpu, reg(SCCPURSTEN));
        if cpu == 2 || cpu == 3 { writel_relaxed(CPU2_ISO_CTRL << (cpu - 2), reg(SCISODIS)); }
        udelay(1);
        val = readl_relaxed(reg(SCPERCTRL0));
        val &= !(CPU0_WFI_MASK_CFG << cpu);
        writel_relaxed(val, reg(SCPERCTRL0));
        val = CPU0_DBG_SRST_REQ_EN | CPU0_NEON_SRST_REQ_EN | CPU0_SRST_REQ_EN | CPU0_HPM_SRST_REQ_EN;
        writel_relaxed(val << cpu, reg(SCCPURSTDIS));
    } else {
        val = readl_relaxed(reg(SCPERCTRL0));
        val |= CPU0_WFI_MASK_CFG << cpu;
        writel_relaxed(val, reg(SCPERCTRL0));
        writel_relaxed(0x01 << cpu, reg(SCCPUCOREDIS));
        if cpu == 2 || cpu == 3 { writel_relaxed(CPU2_ISO_CTRL << (cpu - 2), reg(SCISOEN)); udelay(1); }
        val = CPU0_DBG_SRST_REQ_EN | CPU0_NEON_SRST_REQ_EN | CPU0_SRST_REQ_EN | CPU0_HPM_SRST_REQ_EN;
        writel_relaxed(val << cpu, reg(SCCPURSTEN));
        if cpu == 2 || cpu == 3 { writel_relaxed(CPU2_ISO_CTRL << (cpu - 2), reg(SCPERPWRDIS)); udelay(100); }
    }
}

unsafe fn hi3xxx_hotplug_init() -> i32 {
    let node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"hisilicon,sysctrl\0".as_ptr() as *const i8);
    if node.is_null() { id = ERROR_CTRL; return -2; }
    ctrl_base = of_iomap(node, 0); of_node_put(node);
    if ctrl_base.is_null() { id = ERROR_CTRL; return -12; }
    id = HI3620_CTRL; 0
}

#[no_mangle]
pub unsafe extern "C" fn hi3xxx_set_cpu(cpu: i32, enable: Bool) {
    if ctrl_base.is_null() && hi3xxx_hotplug_init() < 0 { return; }
    if id == HI3620_CTRL { set_cpu_hi3620(cpu, enable); }
}

unsafe fn hix5hd2_hotplug_init() -> Bool {
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"hisilicon,cpuctrl\0".as_ptr() as *const i8);
    if np.is_null() { return false; }
    ctrl_base = of_iomap(np, 0); of_node_put(np); !ctrl_base.is_null()
}

#[no_mangle]
pub unsafe extern "C" fn hix5hd2_set_cpu(_cpu: i32, enable: Bool) {
    let mut val: U32;
    if ctrl_base.is_null() && !hix5hd2_hotplug_init() { panic!(b"BUG\0".as_ptr() as *const i8); }
    if enable {
        val = readl_relaxed(reg(HIX5HD2_PERI_PMC0)); val &= !(PMC0_CPU1_WAIT_MTCOMS_ACK | PMC0_CPU1_POWERDOWN); val |= PMC0_CPU1_PMC_ENABLE; writel_relaxed(val, reg(HIX5HD2_PERI_PMC0));
        val = readl_relaxed(reg(HIX5HD2_PERI_CRG20)); val &= !CRG20_CPU1_RESET; writel_relaxed(val, reg(HIX5HD2_PERI_CRG20));
    } else {
        val = readl_relaxed(reg(HIX5HD2_PERI_PMC0)); val |= PMC0_CPU1_PMC_ENABLE | PMC0_CPU1_POWERDOWN; val &= !PMC0_CPU1_WAIT_MTCOMS_ACK; writel_relaxed(val, reg(HIX5HD2_PERI_PMC0));
        val = readl_relaxed(reg(HIX5HD2_PERI_CRG20)); val |= CRG20_CPU1_RESET; writel_relaxed(val, reg(HIX5HD2_PERI_CRG20));
    }
}

#[no_mangle]
pub unsafe extern "C" fn hip01_set_cpu(_cpu: i32, enable: Bool) {
    let mut temp: U32; let np: *mut DeviceNode;
    if ctrl_base.is_null() { np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"hisilicon,hip01-sysctrl\0".as_ptr() as *const i8); if np.is_null() { panic!(b"BUG_ON\0".as_ptr() as *const i8); } ctrl_base = of_iomap(np, 0); of_node_put(np); if ctrl_base.is_null() { panic!(b"BUG_ON\0".as_ptr() as *const i8); } }
    if enable { temp = readl_relaxed(reg(HIP01_PERI9)); temp |= PERI9_CPU1_RESET; writel_relaxed(temp, reg(HIP01_PERI9)); udelay(50); temp = readl_relaxed(reg(HIP01_PERI9)); temp &= !PERI9_CPU1_RESET; writel_relaxed(temp, reg(HIP01_PERI9)); }
}

unsafe fn cpu_enter_lowpower() {
    flush_cache_all();
    // ARM inline assembly: disable coherency and L1 D-cache via CP15 c1 registers.
}

#[cfg(CONFIG_HOTPLUG_CPU)]
#[no_mangle]
pub unsafe extern "C" fn hi3xxx_cpu_die(cpu: U32) { cpu_enter_lowpower(); hi3xxx_set_cpu_jump(cpu, phys_to_virt(0)); cpu_do_idle(); panic!(b"cpu %d unexpectedly exit from shutdown\n\0".as_ptr() as *const i8, cpu); }

#[cfg(CONFIG_HOTPLUG_CPU)]
#[no_mangle]
pub unsafe extern "C" fn hi3xxx_cpu_kill(cpu: U32) -> i32 {
    let timeout = jiffies_value().wrapping_add(msecs_to_jiffies(50));
    while !hi3xxx_get_cpu_jump(cpu).is_null() { if time_after(jiffies_value(), timeout) { return 0; } }
    hi3xxx_set_cpu(cpu as i32, false); 1
}

#[cfg(CONFIG_HOTPLUG_CPU)]
#[no_mangle]
pub unsafe extern "C" fn hix5hd2_cpu_die(cpu: U32) { flush_cache_all(); hix5hd2_set_cpu(cpu as i32, false); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
