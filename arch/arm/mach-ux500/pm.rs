// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) ST-Ericsson SA 2010-2013
 * Author: Rickard Andersson <rickard.andersson@stericsson.com> for
 *         ST-Ericsson.
 * Author: Daniel Lezcano <daniel.lezcano@linaro.org> for Linaro.
 * Author: Ulf Hansson <ulf.hansson@linaro.org> for Linaro.
 */

// Dependencies supplied by the surrounding kernel translation.

const PRCM_ARM_WFI_STANDBY_WFI0: u32 = 0x08;
const PRCM_ARM_WFI_STANDBY_WFI1: u32 = 0x10;
const PRCM_A9_MASK_REQ_PRCM_A9_MASK_REQ: u32 = 0x1;
const PRCMU_GIC_NUMBER_REGS: i32 = 5;

static mut prcmu_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut dist_base: *mut core::ffi::c_void = core::ptr::null_mut();

unsafe extern "C" {
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn readl_relaxed(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn udelay(usecs: u32);
    fn cpu_do_idle();
    fn ioremap(phy_base: u32, size: u32) -> *mut core::ffi::c_void;
    fn of_find_compatible_node(from: *mut core::ffi::c_void, ty: *mut core::ffi::c_void, compatible: *const i8) -> *mut core::ffi::c_void;
    fn of_iomap(node: *mut core::ffi::c_void, index: i32) -> *mut core::ffi::c_void;
    fn of_node_put(node: *mut core::ffi::c_void);
    fn pr_err(fmt: *const i8, ...);
    fn suspend_set_ops(ops: *const platform_suspend_ops);
}

const GIC_DIST_PENDING_SET: usize = 0x200;
const GIC_DIST_ENABLE_SET: usize = 0x100;

#[repr(C)]
pub struct platform_suspend_ops {
    pub enter: Option<unsafe extern "C" fn(state: suspend_state_t) -> i32>,
    pub valid: Option<unsafe extern "C" fn(state: suspend_state_t) -> i32>,
}

pub type suspend_state_t = i32;
const PM_SUSPEND_MEM: suspend_state_t = 3;
const PM_SUSPEND_STANDBY: suspend_state_t = 1;

unsafe fn ptr_add(base: *mut core::ffi::c_void, offset: usize) -> *mut core::ffi::c_void {
    (base as *mut u8).add(offset) as *mut core::ffi::c_void
}

pub unsafe extern "C" fn prcmu_gic_decouple() -> i32 {
    let addr = ptr_add(prcmu_base, 0x328);
    let val = readl(addr);
    writel(val | PRCM_A9_MASK_REQ_PRCM_A9_MASK_REQ, addr);
    readl(addr);
    udelay(1);
    0
}

pub unsafe extern "C" fn prcmu_gic_recouple() -> i32 {
    let addr = ptr_add(prcmu_base, 0x328);
    let val = readl(addr);
    writel(val & !PRCM_A9_MASK_REQ_PRCM_A9_MASK_REQ, addr);
    0
}

pub unsafe extern "C" fn prcmu_gic_pending_irq() -> bool {
    for i in 0..PRCMU_GIC_NUMBER_REGS {
        let pr = readl_relaxed(ptr_add(dist_base, GIC_DIST_PENDING_SET + (i as usize) * 4));
        let er = readl_relaxed(ptr_add(dist_base, GIC_DIST_ENABLE_SET + (i as usize) * 4));
        if pr & er != 0 { return true; }
    }
    false
}

pub unsafe extern "C" fn prcmu_pending_irq() -> bool {
    for i in 0..(PRCMU_GIC_NUMBER_REGS - 1) {
        let it = readl(ptr_add(prcmu_base, 0x260 + (i as usize) * 4));
        let im = readl(ptr_add(prcmu_base, 0x11c + (i as usize) * 4));
        if it & im != 0 { return true; }
    }
    false
}

pub unsafe extern "C" fn prcmu_is_cpu_in_wfi(cpu: i32) -> bool {
    readl(ptr_add(prcmu_base, 0x130)) & if cpu != 0 { PRCM_ARM_WFI_STANDBY_WFI1 } else { PRCM_ARM_WFI_STANDBY_WFI0 } != 0
}

pub unsafe extern "C" fn prcmu_copy_gic_settings() -> i32 {
    for i in 0..(PRCMU_GIC_NUMBER_REGS - 1) {
        let er = readl_relaxed(ptr_add(dist_base, GIC_DIST_ENABLE_SET + ((i + 1) as usize) * 4));
        writel(er, ptr_add(prcmu_base, 0x11c + (i as usize) * 4));
    }
    0
}

#[cfg(feature = "CONFIG_SUSPEND")]
unsafe extern "C" fn ux500_suspend_enter(_state: suspend_state_t) -> i32 { cpu_do_idle(); 0 }

#[cfg(feature = "CONFIG_SUSPEND")]
unsafe extern "C" fn ux500_suspend_valid(state: suspend_state_t) -> i32 {
    (state == PM_SUSPEND_MEM || state == PM_SUSPEND_STANDBY) as i32
}

#[cfg(feature = "CONFIG_SUSPEND")]
static ux500_suspend_ops: platform_suspend_ops = platform_suspend_ops {
    enter: Some(ux500_suspend_enter), valid: Some(ux500_suspend_valid),
};

#[cfg(feature = "CONFIG_SUSPEND")]
const UX500_SUSPEND_OPS: *const platform_suspend_ops = &ux500_suspend_ops;
#[cfg(not(feature = "CONFIG_SUSPEND"))]
const UX500_SUSPEND_OPS: *const platform_suspend_ops = core::ptr::null();

pub unsafe extern "C" fn ux500_pm_init(phy_base: u32, size: u32) {
    prcmu_base = ioremap(phy_base, size);
    if prcmu_base.is_null() {
        pr_err(c"could not remap PRCMU for PM functions\n".as_ptr());
        return;
    }
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), c"arm,cortex-a9-gic".as_ptr());
    dist_base = of_iomap(np, 0);
    of_node_put(np);
    if dist_base.is_null() {
        pr_err(c"could not remap GIC dist base for PM functions\n".as_ptr());
        return;
    }
    prcmu_gic_recouple();
    suspend_set_ops(UX500_SUSPEND_OPS);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
