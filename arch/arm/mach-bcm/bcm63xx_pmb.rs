// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Broadcom BCM63138 PMB initialization for secondary CPU(s)
 *
 * Copyright (C) 2015 Broadcom Corporation
 * Author: Florian Fainelli <f.fainelli@gmail.com>
 */

// ARM Control register definitions
const CORE_PWR_CTRL_SHIFT: u32 = 0;
const CORE_PWR_CTRL_MASK: u32 = 0x3;
const PLL_PWR_ON: u32 = 1 << 8;
const PLL_LDO_PWR_ON: u32 = 1 << 9;
const PLL_CLAMP_ON: u32 = 1 << 10;
const fn cpu_reset_n(x: u32) -> u32 { 1 << (13 + x) }
const NEON_RESET_N: u32 = 1 << 15;
const PWR_CTRL_STATUS_SHIFT: u32 = 28;
const PWR_CTRL_STATUS_MASK: u32 = 0x3;
const PWR_DOWN_SHIFT: u32 = 30;
const PWR_DOWN_MASK: u32 = 0x3;

// CPU Power control register definitions
const MEM_PWR_OK: u32 = 1 << 0;
const MEM_PWR_ON: u32 = 1 << 1;
const MEM_CLAMP_ON: u32 = 1 << 2;
const MEM_PWR_OK_STATUS: u32 = 1 << 4;
const MEM_PWR_ON_STATUS: u32 = 1 << 5;
const MEM_PDA_SHIFT: u32 = 8;
const MEM_PDA_MASK: u32 = 0xf;
const MEM_PDA_CPU_MASK: u32 = 0x1;
const MEM_PDA_NEON_MASK: u32 = 0xf;
const CLAMP_ON: u32 = 1 << 15;
const PWR_OK_SHIFT: u32 = 16;
const PWR_OK_MASK: u32 = 0xf;
const PWR_ON_SHIFT: u32 = 20;
const PWR_CPU_MASK: u32 = 0x03;
const PWR_NEON_MASK: u32 = 0x01;
const PWR_ON_MASK: u32 = 0xf;
const PWR_OK_STATUS_SHIFT: u32 = 24;
const PWR_OK_STATUS_MASK: u32 = 0xf;
const PWR_ON_STATUS_SHIFT: u32 = 28;
const PWR_ON_STATUS_MASK: u32 = 0xf;

const ARM_CONTROL: u32 = 0x30;
const ARM_PWR_CONTROL_BASE: u32 = 0x34;
const fn arm_pwr_control(x: u32) -> u32 { ARM_PWR_CONTROL_BASE + x * 0x4 }
const ARM_NEON_L2: u32 = 0x3c;

#[repr(C)]
pub struct DeviceNode { _private: [u8; 0] }

extern "C" {
    fn bpcm_wr(master: *mut core::ffi::c_void, addr: u32, off: u32, val: u32) -> i32;
    fn bpcm_rd(master: *mut core::ffi::c_void, addr: u32, off: u32, val: *mut u32) -> i32;
    fn cpu_relax();
    fn of_get_cpu_hwid(dn: *mut DeviceNode, index: u32) -> u32;
    fn of_parse_phandle_with_args(
        dn: *mut DeviceNode,
        name: *const core::ffi::c_char,
        cells_name: *const core::ffi::c_char,
        index: u32,
        args: *mut OfPhandleArgs,
    ) -> i32;
    fn of_iomap(np: *mut DeviceNode, index: i32) -> *mut core::ffi::c_void;
    fn iounmap(addr: *mut core::ffi::c_void);
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn pr_info(fmt: *const core::ffi::c_char, ...);
    fn warn_on(condition: bool) -> bool;
}

#[repr(C)]
pub struct OfPhandleArgs {
    pub np: *mut DeviceNode,
    pub args_count: i32,
    pub args: [u32; 16],
}

static mut PMB_LOCK: u8 = 0;

/* Perform a value write, then spin until the value shifted by
 * shift is seen, masked with mask and is different from cond.
 */
unsafe fn bpcm_wr_rd_mask(
    master: *mut core::ffi::c_void,
    addr: u32,
    off: u32,
    val: *mut u32,
    shift: u32,
    mask: u32,
    cond: u32,
) -> i32 {
    let mut ret = bpcm_wr(master, addr, off, *val);
    if ret != 0 { return ret; }
    loop {
        ret = bpcm_rd(master, addr, off, val);
        if ret != 0 { return ret; }
        cpu_relax();
        if (((*val >> shift) & mask) == cond) { break; }
    }
    ret
}

unsafe fn bcm63xx_pmb_get_resources(
    dn: *mut DeviceNode,
    base: *mut *mut core::ffi::c_void,
    cpu: *mut u32,
    addr: *mut u32,
) -> i32 {
    let mut args = OfPhandleArgs { np: core::ptr::null_mut(), args_count: 0, args: [0; 16] };
    *cpu = of_get_cpu_hwid(dn, 0);
    if *cpu == !0u32 { pr_err(b"CPU is missing a reg node\0".as_ptr() as _); return -19; }
    let ret = of_parse_phandle_with_args(dn, b"resets\0".as_ptr() as _, b"#reset-cells\0".as_ptr() as _, 0, &mut args);
    if ret != 0 { pr_err(b"CPU is missing a resets phandle\0".as_ptr() as _); return ret; }
    if args.args_count != 2 { pr_err(b"reset-controller does not conform to reset-cells\0".as_ptr() as _); return -22; }
    *base = of_iomap(args.np, 0);
    if (*base).is_null() { pr_err(b"failed remapping PMB register\0".as_ptr() as _); return -12; }
    *addr = args.args[0];
    0
}

pub unsafe fn bcm63xx_pmb_power_on_cpu(dn: *mut DeviceNode) -> i32 {
    let mut base = core::ptr::null_mut();
    let mut cpu = 0u32;
    let mut addr = 0u32;
    let mut ret = bcm63xx_pmb_get_resources(dn, &mut base, &mut cpu, &mut addr);
    if ret != 0 { return ret; }
    warn_on(cpu > 1);
    let mut ctrl = 0u32;
    let mut val = 0u32;
    ret = bpcm_rd(base, addr, ARM_CONTROL, &mut ctrl);
    if ret != 0 { iounmap(base); return ret; }
    if ctrl & cpu_reset_n(cpu) != 0 { pr_info(b"PMB: CPU%d is already powered on\0".as_ptr() as _, cpu); iounmap(base); return 0; }
    ret = bpcm_rd(base, addr, arm_pwr_control(cpu), &mut val);
    if ret == 0 { val |= PWR_CPU_MASK << PWR_ON_SHIFT; ret = bpcm_wr_rd_mask(base, addr, arm_pwr_control(cpu), &mut val, PWR_ON_STATUS_SHIFT, PWR_CPU_MASK, PWR_CPU_MASK); }
    if ret == 0 { val |= PWR_CPU_MASK << PWR_OK_SHIFT; ret = bpcm_wr_rd_mask(base, addr, arm_pwr_control(cpu), &mut val, PWR_OK_STATUS_SHIFT, PWR_CPU_MASK, PWR_CPU_MASK); }
    if ret == 0 { val &= !CLAMP_ON; ret = bpcm_wr(base, addr, arm_pwr_control(cpu), val); }
    if ret == 0 { val &= !(MEM_PDA_MASK << MEM_PDA_SHIFT); ret = bpcm_wr(base, addr, arm_pwr_control(cpu), val); }
    if ret == 0 { val |= MEM_PWR_ON; ret = bpcm_wr_rd_mask(base, addr, arm_pwr_control(cpu), &mut val, 0, MEM_PWR_ON_STATUS, MEM_PWR_ON_STATUS); }
    if ret == 0 { val |= MEM_PWR_OK; ret = bpcm_wr_rd_mask(base, addr, arm_pwr_control(cpu), &mut val, 0, MEM_PWR_OK_STATUS, MEM_PWR_OK_STATUS); }
    if ret == 0 { val &= !MEM_CLAMP_ON; ret = bpcm_wr(base, addr, arm_pwr_control(cpu), val); }
    if ret == 0 { ctrl |= cpu_reset_n(cpu); ret = bpcm_wr(base, addr, ARM_CONTROL, ctrl); }
    iounmap(base);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
