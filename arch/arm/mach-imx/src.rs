// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2011 Freescale Semiconductor, Inc.
 * Copyright 2011 Linaro Ltd.
 */

// Linux/kernel dependencies supplied by other translation units.
use core::ffi::c_void;

const SRC_SCR: usize = 0x000;
const SRC_GPR1_V1: usize = 0x020;
const SRC_GPR1_V2: usize = 0x074;
#[inline]
const fn src_gpr1(gpr_v2: bool) -> usize { if gpr_v2 { SRC_GPR1_V2 } else { SRC_GPR1_V1 } }
const BP_SRC_SCR_WARM_RESET_ENABLE: u32 = 0;
const BP_SRC_SCR_SW_GPU_RST: u32 = 1;
const BP_SRC_SCR_SW_VPU_RST: u32 = 2;
const BP_SRC_SCR_SW_IPU1_RST: u32 = 3;
const BP_SRC_SCR_SW_OPEN_VG_RST: u32 = 4;
const BP_SRC_SCR_SW_IPU2_RST: u32 = 12;
const BP_SRC_SCR_CORE1_RST: u32 = 14;
const BP_SRC_SCR_CORE1_ENABLE: u32 = 22;
// below is for i.MX7D
const SRC_A7RCR1: usize = 0x008;
const BP_SRC_A7RCR1_A7_CORE1_ENABLE: u32 = 1;
const GPC_CPU_PGC_SW_PUP_REQ: usize = 0xf0;
const GPC_CPU_PGC_SW_PDN_REQ: usize = 0xfc;
const GPC_PGC_C1: usize = 0x840;
const BM_CPU_PGC_SW_PDN_PUP_REQ_CORE1_A7: u32 = 0x2;

static mut src_base: *mut u8 = core::ptr::null_mut();
static mut scr_lock: Spinlock = Spinlock;
static mut gpr_v2: bool = false;
static mut gpc_base: *mut u8 = core::ptr::null_mut();

static sw_reset_bits: [u32; 5] = [
    BP_SRC_SCR_SW_GPU_RST,
    BP_SRC_SCR_SW_VPU_RST,
    BP_SRC_SCR_SW_IPU1_RST,
    BP_SRC_SCR_SW_OPEN_VG_RST,
    BP_SRC_SCR_SW_IPU2_RST,
];

#[repr(C)]
pub struct Spinlock;
extern "C" {
    fn readl(addr: *mut u8) -> u32;
    fn readl_relaxed(addr: *mut u8) -> u32;
    fn writel_relaxed(value: u32, addr: *mut u8);
    fn cpu_relax();
    fn jiffies() -> usize;
    fn msecs_to_jiffies(value: u32) -> usize;
    fn time_after(a: usize, b: usize) -> bool;
    fn spin_lock(lock: *mut Spinlock);
    fn spin_unlock(lock: *mut Spinlock);
    fn spin_lock_irqsave(lock: *mut Spinlock, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut Spinlock, flags: usize);
    fn cpu_logical_map(cpu: i32) -> i32;
    fn __pa_symbol(addr: *mut c_void) -> u32;
    fn pr_err(message: *const u8);
    fn of_find_compatible_node(from: *mut c_void, ty: *const u8, compatible: *const u8) -> *mut DeviceNode;
    fn of_iomap(node: *mut DeviceNode, index: i32) -> *mut u8;
    fn of_node_put(node: *mut DeviceNode);
}

#[repr(C)]
pub struct DeviceNode;

unsafe fn imx_src_reset_module(_rcdev: *mut c_void, sw_reset_idx: usize) -> i32 {
    if sw_reset_idx >= sw_reset_bits.len() { return -22; }
    let bit = 1u32 << sw_reset_bits[sw_reset_idx];
    let mut flags = 0usize;
    spin_lock_irqsave(&mut scr_lock, &mut flags);
    let mut val = readl_relaxed(src_base.add(SRC_SCR));
    val |= bit;
    writel_relaxed(val, src_base.add(SRC_SCR));
    spin_unlock_irqrestore(&mut scr_lock, flags);
    let timeout = jiffies().wrapping_add(msecs_to_jiffies(1000));
    while readl(src_base.add(SRC_SCR)) & bit != 0 {
        if time_after(jiffies(), timeout) { return -62; }
        cpu_relax();
    }
    0
}

unsafe fn imx_gpcv2_set_m_core_pgc(enable: bool, offset: usize) {
    writel_relaxed(enable as u32, gpc_base.add(offset));
}

/*
 * The motivation for bringing up the second i.MX7D core inside the kernel
 * is that legacy vendor bootloaders usually do not implement PSCI support.
 * This is a significant blocker for systems in the field that are running old
 * bootloader versions to upgrade to a modern mainline kernel version, as only
 * one CPU of the i.MX7D would be brought up.
 * Bring up the second i.MX7D core inside the kernel to make the migration
 * path to mainline kernel easier for the existing iMX7D users.
 */
pub unsafe fn imx_gpcv2_set_core1_pdn_pup_by_software(pdn: bool) {
    let reg = if pdn { GPC_CPU_PGC_SW_PDN_REQ } else { GPC_CPU_PGC_SW_PUP_REQ };
    imx_gpcv2_set_m_core_pgc(true, GPC_PGC_C1);
    let mut val = readl_relaxed(gpc_base.add(reg));
    val |= BM_CPU_PGC_SW_PDN_PUP_REQ_CORE1_A7;
    writel_relaxed(val, gpc_base.add(reg));
    let mut pup = 0u32;
    let mut ret = 0i32;
    for _ in 0..1000000 {
        pup = readl_relaxed(gpc_base.add(reg));
        if pup & BM_CPU_PGC_SW_PDN_PUP_REQ_CORE1_A7 == 0 { break; }
        ret = -1;
    }
    if ret < 0 {
        pr_err(b"i.MX7D: CORE1_A7 power up timeout\n\0");
        val &= !BM_CPU_PGC_SW_PDN_PUP_REQ_CORE1_A7;
        writel_relaxed(val, gpc_base.add(reg));
    }
    imx_gpcv2_set_m_core_pgc(false, GPC_PGC_C1);
}

pub unsafe fn imx_enable_cpu(mut cpu: i32, enable: bool) {
    cpu = cpu_logical_map(cpu);
    spin_lock(&mut scr_lock);
    let (mask, mut val);
    if gpr_v2 {
        if enable { imx_gpcv2_set_core1_pdn_pup_by_software(false); }
        mask = 1u32 << (BP_SRC_A7RCR1_A7_CORE1_ENABLE + cpu as u32 - 1);
        val = readl_relaxed(src_base.add(SRC_A7RCR1));
        val = if enable { val | mask } else { val & !mask };
        writel_relaxed(val, src_base.add(SRC_A7RCR1));
    } else {
        mask = 1u32 << (BP_SRC_SCR_CORE1_ENABLE + cpu as u32 - 1);
        val = readl_relaxed(src_base.add(SRC_SCR));
        val = if enable { val | mask } else { val & !mask };
        val |= 1u32 << (BP_SRC_SCR_CORE1_RST + cpu as u32 - 1);
        writel_relaxed(val, src_base.add(SRC_SCR));
    }
    spin_unlock(&mut scr_lock);
}

pub unsafe fn imx_set_cpu_jump(mut cpu: i32, jump_addr: *mut c_void) {
    cpu = cpu_logical_map(cpu);
    writel_relaxed(__pa_symbol(jump_addr), src_base.add(src_gpr1(gpr_v2) + cpu as usize * 8));
}

pub unsafe fn imx_get_cpu_arg(mut cpu: i32) -> u32 {
    cpu = cpu_logical_map(cpu);
    readl_relaxed(src_base.add(src_gpr1(gpr_v2) + cpu as usize * 8 + 4))
}

pub unsafe fn imx_set_cpu_arg(mut cpu: i32, arg: u32) {
    cpu = cpu_logical_map(cpu);
    writel_relaxed(arg, src_base.add(src_gpr1(gpr_v2) + cpu as usize * 8 + 4));
}

pub unsafe fn imx_src_init() {
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"fsl,imx51-src\0".as_ptr());
    if np.is_null() { return; }
    src_base = of_iomap(np, 0);
    of_node_put(np);
    // force warm reset sources to generate cold reset for a more reliable restart
    let mut val = readl_relaxed(src_base.add(SRC_SCR));
    spin_lock(&mut scr_lock);
    val &= !(1u32 << BP_SRC_SCR_WARM_RESET_ENABLE);
    writel_relaxed(val, src_base.add(SRC_SCR));
    spin_unlock(&mut scr_lock);
}

pub unsafe fn imx7_src_init() {
    gpr_v2 = true;
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"fsl,imx7d-src\0".as_ptr());
    if np.is_null() { return; }
    src_base = of_iomap(np, 0);
    of_node_put(np);
    if src_base.is_null() { return; }
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"fsl,imx7d-gpc\0".as_ptr());
    if np.is_null() { return; }
    gpc_base = of_iomap(np, 0);
    of_node_put(np);
}

#[repr(C)]
pub struct OfDeviceId { pub compatible: *const u8 }
static imx_src_dt_ids: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"fsl,imx51-src\0".as_ptr() },
    OfDeviceId { compatible: core::ptr::null() },
];

unsafe fn imx_src_probe(_pdev: *mut c_void) -> i32 {
    // reset-controller allocation and registration are supplied by the kernel.
    0
}

#[repr(C)]
pub struct PlatformDriver;
static mut imx_src_driver: PlatformDriver = PlatformDriver;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
