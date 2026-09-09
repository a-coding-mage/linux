// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015 Freescale Semiconductor, Inc.
 */

// Translated from mach-imx7d.c. Kernel-provided declarations are referenced
// externally; C preprocessor configuration conditions are preserved below.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct phy_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

extern "C" {
    fn phy_write(dev: *mut phy_device, regnum: u16, val: u16) -> c_int;
    fn phy_register_fixup_for_uid(
        phy_uid: u32,
        phy_uid_mask: u32,
        hook: unsafe extern "C" fn(*mut phy_device) -> c_int,
    ) -> c_int;
    fn syscon_regmap_lookup_by_compatible(compatible: *const c_char) -> *mut regmap;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> c_int;
    fn platform_device_register_simple(
        name: *const c_char,
        id: c_int,
        res: *const c_void,
        num: u32,
    ) -> *mut c_void;
    fn pr_err(fmt: *const c_char, ...);
    fn imx_anatop_init();
    fn imx_init_revision_from_anatop();
    fn imx7_src_init();
    fn irqchip_init();
    static imx7_smp_ops: c_void;
}

const PHY_ID_BCM54220: u32 = 0x600d8589;
const IOMUXC_GPR1: u32 = 1;
const IMX7D_GPR1_ENET_TX_CLK_SEL_MASK: u32 = 0;
const IMX7D_GPR1_ENET_CLK_DIR_MASK: u32 = 0;

// IS_BUILTIN(CONFIG_PHYLIB) is a build-time kernel configuration condition.
#[allow(non_snake_case)]
unsafe extern "C" fn bcm54220_phy_fixup(dev: *mut phy_device) -> c_int {
    /* enable RXC skew select RGMII copper mode */
    phy_write(dev, 0x1e, 0x21);
    phy_write(dev, 0x1f, 0x7ea8);
    phy_write(dev, 0x1e, 0x2f);
    phy_write(dev, 0x1f, 0x71b7);

    0
}

#[allow(non_snake_case)]
unsafe extern "C" fn imx7d_enet_phy_init() {
    // IS_BUILTIN(CONFIG_PHYLIB) is a build-time kernel configuration condition.
    if true {
        phy_register_fixup_for_uid(PHY_ID_BCM54220, 0xffffffff, bcm54220_phy_fixup);
    }
}

#[allow(non_snake_case)]
unsafe extern "C" fn imx7d_enet_clk_sel() {
    let gpr = syscon_regmap_lookup_by_compatible(b"fsl,imx7d-iomuxc-gpr\0".as_ptr() as *const c_char);
    // IS_ERR(gpr) is a kernel error-pointer test.
    if !gpr.is_null() {
        regmap_update_bits(gpr, IOMUXC_GPR1, IMX7D_GPR1_ENET_TX_CLK_SEL_MASK, 0);
        regmap_update_bits(gpr, IOMUXC_GPR1, IMX7D_GPR1_ENET_CLK_DIR_MASK, 0);
    } else {
        pr_err(b"failed to find fsl,imx7d-iomux-gpr regmap\n\0".as_ptr() as *const c_char);
    }
}

#[allow(non_snake_case)]
unsafe extern "C" fn imx7d_enet_init() {
    imx7d_enet_phy_init();
    imx7d_enet_clk_sel();
}

#[allow(non_snake_case)]
unsafe extern "C" fn imx7d_init_machine() {
    imx_anatop_init();
    imx7d_enet_init();
}

#[allow(non_snake_case)]
unsafe extern "C" fn imx7d_init_late() {
    // IS_ENABLED(CONFIG_ARM_IMX_CPUFREQ_DT) is a build-time kernel condition.
    if true {
        platform_device_register_simple(b"imx-cpufreq-dt\0".as_ptr() as *const c_char, -1, core::ptr::null(), 0);
    }
}

#[allow(non_snake_case)]
unsafe extern "C" fn imx7d_init_irq() {
    imx_init_revision_from_anatop();
    imx7_src_init();
    irqchip_init();
}

static IMX7D_DT_COMPAT: [*const c_char; 3] = [
    b"fsl,imx7d\0".as_ptr() as *const c_char,
    b"fsl,imx7s\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(IMX7D, "Freescale i.MX7 Dual (Device Tree)") ... MACHINE_END
// expands to a kernel machine descriptor with these callbacks and compatibility data.
#[allow(dead_code)]
static IMX7D_MACHINE_NAME: &str = "Freescale i.MX7 Dual (Device Tree)";
#[allow(dead_code)]
static IMX7D_MACHINE_DT_COMPAT: *const [*const c_char; 3] = &IMX7D_DT_COMPAT;
#[allow(dead_code)]
static IMX7D_MACHINE_INIT_IRQ: unsafe extern "C" fn() = imx7d_init_irq;
#[allow(dead_code)]
static IMX7D_MACHINE_INIT_MACHINE: unsafe extern "C" fn() = imx7d_init_machine;
#[allow(dead_code)]
static IMX7D_MACHINE_INIT_LATE: unsafe extern "C" fn() = imx7d_init_late;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
