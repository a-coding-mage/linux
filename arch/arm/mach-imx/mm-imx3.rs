// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 1999,2000 Arm Limited
 *  Copyright (C) 2000 Deep Blue Solutions Ltd
 *  Copyright (C) 2002 Shane Nay (shane@minirl.com)
 *  Copyright 2005-2007 Freescale Semiconductor, Inc. All Rights Reserved.
 *    - add MX31 specific definitions
 */

// Dependencies supplied by the surrounding kernel translation.

pub static mut mx3_ccm_base: *mut core::ffi::c_void = core::ptr::null_mut();

unsafe fn imx3_idle() {
    let mut reg: usize = 0;
    core::arch::asm!(
        "mrc p15, 0, {0}, c1, c0, 0",
        "bic {0}, {0}, #0x00001000",
        "bic {0}, {0}, #0x00000004",
        "mcr p15, 0, {0}, c1, c0, 0",
        "mov {0}, #0",
        "mcr p15, 0, {0}, c7, c5, 0",
        "mov {0}, #0",
        "mcr p15, 0, {0}, c7, c14, 0",
        "mov {0}, #0",
        "mcr p15, 0, {0}, c7, c0, 4",
        "nop", "nop", "nop", "nop", "nop", "nop", "nop",
        "mrc p15, 0, {0}, c1, c0, 0",
        "orr {0}, {0}, #0x00001000",
        "orr {0}, {0}, #0x00000004",
        "mcr p15, 0, {0}, c1, c0, 0",
        inout(reg) reg,
    );
}

unsafe fn imx3_ioremap_caller(
    phys_addr: usize,
    size: usize,
    mut mtype: u32,
    caller: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    if mtype == MT_DEVICE {
        // Access peripherals below 0x80000000 as nonshared devices on mx3,
        // but leave l2cc alone; otherwise cache corruptions can occur.
        if phys_addr < 0x80000000 && !addr_in_module(phys_addr, MX3x_L2CC) {
            mtype = MT_DEVICE_NONSHARED;
        }
    }
    __arm_ioremap_caller(phys_addr, size, mtype, caller)
}

#[cfg(CONFIG_SOC_IMX31)]
static mut mx31_io_desc: [map_desc; 5] = [
    imx_map_entry!(MX31, X_MEMC, MT_DEVICE),
    imx_map_entry!(MX31, AVIC, MT_DEVICE_NONSHARED),
    imx_map_entry!(MX31, AIPS1, MT_DEVICE_NONSHARED),
    imx_map_entry!(MX31, AIPS2, MT_DEVICE_NONSHARED),
    imx_map_entry!(MX31, SPBA0, MT_DEVICE_NONSHARED),
];

#[cfg(CONFIG_SOC_IMX31)]
pub unsafe fn mx31_map_io() {
    iotable_init(mx31_io_desc.as_ptr(), mx31_io_desc.len());
}

#[cfg(CONFIG_SOC_IMX31)]
unsafe fn imx31_idle() {
    let mut reg = imx_readl(mx3_ccm_base.add(MXC_CCM_CCMR));
    reg &= !MXC_CCM_CCMR_LPM_MASK;
    imx_writel(reg, mx3_ccm_base.add(MXC_CCM_CCMR));
    imx3_idle();
}

#[cfg(CONFIG_SOC_IMX31)]
pub unsafe fn imx31_init_early() {
    let mut np: *mut device_node;
    mxc_set_cpu_type(MXC_CPU_MX31);
    arch_ioremap_caller = Some(imx3_ioremap_caller);
    arm_pm_idle = Some(imx31_idle);
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,imx31-ccm\0".as_ptr());
    mx3_ccm_base = of_iomap(np, 0);
    of_node_put(np);
    BUG_ON(mx3_ccm_base.is_null());
}

#[cfg(CONFIG_SOC_IMX35)]
static mut mx35_io_desc: [map_desc; 5] = [
    imx_map_entry!(MX35, X_MEMC, MT_DEVICE),
    imx_map_entry!(MX35, AVIC, MT_DEVICE_NONSHARED),
    imx_map_entry!(MX35, AIPS1, MT_DEVICE_NONSHARED),
    imx_map_entry!(MX35, AIPS2, MT_DEVICE_NONSHARED),
    imx_map_entry!(MX35, SPBA0, MT_DEVICE_NONSHARED),
];

#[cfg(CONFIG_SOC_IMX35)]
pub unsafe fn mx35_map_io() {
    iotable_init(mx35_io_desc.as_ptr(), mx35_io_desc.len());
}

#[cfg(CONFIG_SOC_IMX35)]
unsafe fn imx35_idle() {
    let mut reg = imx_readl(mx3_ccm_base.add(MXC_CCM_CCMR));
    reg &= !MXC_CCM_CCMR_LPM_MASK;
    reg |= MXC_CCM_CCMR_LPM_WAIT_MX35;
    imx_writel(reg, mx3_ccm_base.add(MXC_CCM_CCMR));
    imx3_idle();
}

#[cfg(CONFIG_SOC_IMX35)]
pub unsafe fn imx35_init_early() {
    let np: *mut device_node;
    mxc_set_cpu_type(MXC_CPU_MX35);
    arm_pm_idle = Some(imx35_idle);
    arch_ioremap_caller = Some(imx3_ioremap_caller);
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,imx35-ccm\0".as_ptr());
    mx3_ccm_base = of_iomap(np, 0);
    of_node_put(np);
    BUG_ON(mx3_ccm_base.is_null());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
