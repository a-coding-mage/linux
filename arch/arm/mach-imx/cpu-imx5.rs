// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2008-2010 Freescale Semiconductor, Inc. All Rights Reserved.
 *
 * This file contains the CPU initialization code.
 */

// Dependencies supplied by the surrounding kernel translation.

use core::ffi::c_char;

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

extern "C" {
    fn of_find_compatible_node(
        from: *mut DeviceNode,
        type_: *const c_char,
        compatible: *const c_char,
    ) -> *mut DeviceNode;
    fn of_iomap(np: *mut DeviceNode, index: i32) -> *mut core::ffi::c_void;
    fn of_node_put(np: *mut DeviceNode);
    fn iounmap(addr: *mut core::ffi::c_void);
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn readl_relaxed(addr: *mut core::ffi::c_void) -> u32;
    fn writel_relaxed(value: u32, addr: *mut core::ffi::c_void);
    fn of_property_read_bool(np: *mut DeviceNode, propname: *const c_char) -> bool;
    fn pr_info(fmt: *const c_char, ...);
    fn warn_on(condition: bool) -> bool;
}

// These constants are supplied by hardware.h/common.h.
extern "C" {
    static mut elf_hwcap: u32;
}

const IMX_CHIP_REVISION_UNKNOWN: i32 = 0;
const IMX_CHIP_REVISION_1_0: i32 = 1;
const IMX_CHIP_REVISION_2_0: i32 = 2;
const IMX_CHIP_REVISION_2_1: i32 = 3;
const IMX_CHIP_REVISION_3_0: i32 = 4;
const HWCAP_NEON: u32 = 1 << 12;

static mut MX5_CPU_REV: i32 = -1;

const IIM_SREV: usize = 0x24;

unsafe fn imx5_read_srev_reg(compat: *const c_char) -> u32 {
    let np: *mut DeviceNode;
    let iim_base: *mut core::ffi::c_void;
    let srev: u32;

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), compat);
    iim_base = of_iomap(np, 0);
    of_node_put(np);
    let _ = warn_on(iim_base.is_null());

    srev = readl(iim_base.add(IIM_SREV)) & 0xff;

    iounmap(iim_base);

    srev
}

unsafe fn get_mx51_srev() -> i32 {
    let rev = imx5_read_srev_reg(c"fsl,imx51-iim".as_ptr());

    match rev {
        0x0 => IMX_CHIP_REVISION_2_0,
        0x10 => IMX_CHIP_REVISION_3_0,
        _ => IMX_CHIP_REVISION_UNKNOWN,
    }
}

/*
 * Returns:
 *	the silicon revision of the cpu
 */
#[no_mangle]
pub unsafe extern "C" fn mx51_revision() -> i32 {
    if MX5_CPU_REV == -1 {
        MX5_CPU_REV = get_mx51_srev();
    }

    MX5_CPU_REV
}

// CONFIG_NEON conditional from the original source.
#[cfg(feature = "CONFIG_NEON")]
/*
 * All versions of the silicon before Rev. 3 have broken NEON implementations.
 * Dependent on link order - so the assumption is that vfp_init is called
 * before us.
 */
#[no_mangle]
pub unsafe extern "C" fn mx51_neon_fixup() -> i32 {
    if mx51_revision() < IMX_CHIP_REVISION_3_0 && (elf_hwcap & HWCAP_NEON) != 0 {
        elf_hwcap &= !HWCAP_NEON;
        pr_info(c"Turning off NEON support, detected broken NEON implementation\n".as_ptr());
    }
    0
}

unsafe fn get_mx53_srev() -> i32 {
    let rev = imx5_read_srev_reg(c"fsl,imx53-iim".as_ptr());

    match rev {
        0x0 => IMX_CHIP_REVISION_1_0,
        0x2 => IMX_CHIP_REVISION_2_0,
        0x3 => IMX_CHIP_REVISION_2_1,
        _ => IMX_CHIP_REVISION_UNKNOWN,
    }
}

/*
 * Returns:
 *	the silicon revision of the cpu
 */
#[no_mangle]
pub unsafe extern "C" fn mx53_revision() -> i32 {
    if MX5_CPU_REV == -1 {
        MX5_CPU_REV = get_mx53_srev();
    }

    MX5_CPU_REV
}

const ARM_GPC: usize = 0x4;
const DBGEN: u32 = 1 << 16;

/*
 * This enables the DBGEN bit in ARM_GPC register, which is
 * required for accessing some performance counter features.
 * Technically it is only required while perf is used, but to
 * keep the source code simple we just enable it all the time
 * when the kernel configuration allows using the feature.
 */
#[no_mangle]
pub unsafe extern "C" fn imx5_pmu_init() {
    let tigerp_base: *mut core::ffi::c_void;
    let mut np: *mut DeviceNode;
    let mut gpc: u32;

    // IS_ENABLED(CONFIG_ARM_PMU) is resolved by the build configuration.
    if !cfg!(feature = "CONFIG_ARM_PMU") {
        return;
    }

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"arm,cortex-a8-pmu".as_ptr());
    if np.is_null() {
        return;
    }

    if !of_property_read_bool(np, c"secure-reg-access".as_ptr()) {
        of_node_put(np);
        return;
    }

    of_node_put(np);

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"fsl,imx51-tigerp".as_ptr());
    if np.is_null() {
        return;
    }

    tigerp_base = of_iomap(np, 0);
    if tigerp_base.is_null() {
        of_node_put(np);
        return;
    }

    gpc = readl_relaxed(tigerp_base.add(ARM_GPC));
    gpc |= DBGEN;
    writel_relaxed(gpc, tigerp_base.add(ARM_GPC));
    iounmap(tigerp_base);
    of_node_put(np);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
