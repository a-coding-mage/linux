// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding kernel translation unit.

extern "C" {
    fn of_platform_default_populate(
        root: *mut core::ffi::c_void,
        parent: *mut core::ffi::c_void,
        lookup: *mut core::ffi::c_void,
    );
    fn imx_anatop_init();
    fn imx6sx_pm_init();
    fn imx_gpc_check_dt();
    fn imx_init_revision_from_anatop();
    fn imx_init_l2cache();
    fn imx_src_init();
    fn irqchip_init();
    fn imx6_pm_ccm_init(compat: *const core::ffi::c_char);
    fn imx6sx_cpuidle_init();
    fn platform_device_register_simple(
        name: *const core::ffi::c_char,
        id: i32,
        res: *mut core::ffi::c_void,
        num: u32,
    );
}

// CONFIG_ARM_IMX6Q_CPUFREQ is a build-time configuration condition.
#[cfg(feature = "CONFIG_ARM_IMX6Q_CPUFREQ")]
const CONFIG_ARM_IMX6Q_CPUFREQ: bool = true;
#[cfg(not(feature = "CONFIG_ARM_IMX6Q_CPUFREQ"))]
const CONFIG_ARM_IMX6Q_CPUFREQ: bool = false;

unsafe extern "C" fn imx6sx_init_machine() {
    unsafe {
        of_platform_default_populate(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());

        imx_anatop_init();
        imx6sx_pm_init();
    }
}

unsafe extern "C" fn imx6sx_init_irq() {
    unsafe {
        imx_gpc_check_dt();
        imx_init_revision_from_anatop();
        imx_init_l2cache();
        imx_src_init();
        irqchip_init();
        imx6_pm_ccm_init(c"fsl,imx6sx-ccm".as_ptr());
    }
}

unsafe extern "C" fn imx6sx_init_late() {
    unsafe {
        imx6sx_cpuidle_init();

        if CONFIG_ARM_IMX6Q_CPUFREQ {
            platform_device_register_simple(c"imx6q-cpufreq".as_ptr(), -1, core::ptr::null_mut(), 0);
        }
    }
}

#[used]
#[link_section = ".init.rodata"]
static IMX6SX_DT_COMPAT: [*const core::ffi::c_char; 2] = [
    c"fsl,imx6sx".as_ptr(),
    core::ptr::null(),
];

// Translation of DT_MACHINE_START(IMX6SX, "Freescale i.MX6 SoloX (Device Tree)") ... MACHINE_END.
#[repr(C)]
pub struct MachineDesc {
    pub name: *const core::ffi::c_char,
    pub l2c_aux_val: u32,
    pub l2c_aux_mask: u32,
    pub init_irq: Option<unsafe extern "C" fn()>,
    pub init_machine: Option<unsafe extern "C" fn()>,
    pub dt_compat: *const *const core::ffi::c_char,
    pub init_late: Option<unsafe extern "C" fn()>,
}

#[used]
#[link_section = ".init.rodata"]
pub static IMX6SX: MachineDesc = MachineDesc {
    name: c"Freescale i.MX6 SoloX (Device Tree)".as_ptr(),
    l2c_aux_val: 0,
    l2c_aux_mask: !0,
    init_irq: Some(imx6sx_init_irq),
    init_machine: Some(imx6sx_init_machine),
    dt_compat: IMX6SX_DT_COMPAT.as_ptr(),
    init_late: Some(imx6sx_init_late),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
