// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2013 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe extern "C" {
    fn syscon_regmap_lookup_by_compatible(compatible: *const core::ffi::c_char) -> *mut regmap;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn platform_device_register_simple(
        name: *const core::ffi::c_char,
        id: i32,
        data: *const core::ffi::c_void,
        size: usize,
    ) -> *mut core::ffi::c_void;
    fn of_platform_default_populate(
        root: *const core::ffi::c_void,
        matches: *const core::ffi::c_void,
        parent: *const core::ffi::c_void,
    ) -> i32;
    fn imx6sl_cpuidle_init();
    fn imx6sx_cpuidle_init();
    fn imx_anatop_init();
    fn imx6sl_pm_init();
    fn imx_gpc_check_dt();
    fn imx_init_revision_from_anatop();
    fn imx_init_l2cache();
    fn imx_src_init();
    fn irqchip_init();
    fn imx6_pm_ccm_init(compatible: *const core::ffi::c_char);
    fn cpu_is_imx6sl() -> bool;
    fn pr_err(format: *const core::ffi::c_char);
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

const IOMUXC_GPR1: u32 = 0;
const IMX6SL_GPR1_FEC_CLOCK_MUX2_SEL_MASK: u32 = 0;
const IMX6SL_GPR1_FEC_CLOCK_MUX1_SEL_MASK: u32 = 0;

#[inline]
unsafe fn is_err<T>(ptr: *mut T) -> bool {
    (ptr as isize) < 0
}

unsafe fn imx6sl_fec_init() {
    let gpr: *mut regmap;

    /* set FEC clock from internal PLL clock source */
    gpr = syscon_regmap_lookup_by_compatible(b"fsl,imx6sl-iomuxc-gpr\0".as_ptr() as *const _);
    if !is_err(gpr) {
        regmap_update_bits(
            gpr,
            IOMUXC_GPR1,
            IMX6SL_GPR1_FEC_CLOCK_MUX2_SEL_MASK,
            0,
        );
        regmap_update_bits(
            gpr,
            IOMUXC_GPR1,
            IMX6SL_GPR1_FEC_CLOCK_MUX1_SEL_MASK,
            0,
        );
    } else {
        pr_err(b"failed to find fsl,imx6sl-iomux-gpr regmap\n\0".as_ptr() as *const _);
    }
}

unsafe fn imx6sl_init_late() {
    /* imx6sl reuses imx6q cpufreq driver */
    if cfg!(feature = "CONFIG_ARM_IMX6Q_CPUFREQ") {
        platform_device_register_simple(b"imx6q-cpufreq\0".as_ptr() as *const _, -1, core::ptr::null(), 0);
    }

    if cfg!(feature = "CONFIG_SOC_IMX6SL") && cpu_is_imx6sl() {
        imx6sl_cpuidle_init();
    } else if cfg!(feature = "CONFIG_SOC_IMX6SLL") {
        imx6sx_cpuidle_init();
    }
}

unsafe fn imx6sl_init_machine() {
    of_platform_default_populate(core::ptr::null(), core::ptr::null(), core::ptr::null());

    if cpu_is_imx6sl() {
        imx6sl_fec_init();
    }
    imx_anatop_init();
    imx6sl_pm_init();
}

unsafe fn imx6sl_init_irq() {
    imx_gpc_check_dt();
    imx_init_revision_from_anatop();
    imx_init_l2cache();
    imx_src_init();
    irqchip_init();
    if cpu_is_imx6sl() {
        imx6_pm_ccm_init(b"fsl,imx6sl-ccm\0".as_ptr() as *const _);
    } else {
        imx6_pm_ccm_init(b"fsl,imx6sll-ccm\0".as_ptr() as *const _);
    }
}

static IMX6SL_DT_COMPAT: [&'static [u8]; 3] = [
    b"fsl,imx6sl\0",
    b"fsl,imx6sll\0",
    b"\0",
];

// DT_MACHINE_START(IMX6SL, "Freescale i.MX6 SoloLite (Device Tree)")
// .l2c_aux_val = 0, .l2c_aux_mask = ~0,
// .init_irq = imx6sl_init_irq, .init_machine = imx6sl_init_machine,
// .init_late = imx6sl_init_late, .dt_compat = imx6sl_dt_compat,
// MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
