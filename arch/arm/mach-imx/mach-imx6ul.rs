// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

extern "C" {
    fn cpu_is_imx6ull() -> bool;
    fn imx_print_silicon_rev(revision: *const i8, soc_revision: i32);
    fn imx_get_soc_revision() -> i32;
    fn of_platform_default_populate(
        root: *const c_void,
        matches: *const c_void,
        parent: *const c_void,
    ) -> i32;
    fn imx_anatop_init();
    fn imx6ul_pm_init();
    fn imx_init_revision_from_anatop();
    fn imx_src_init();
    fn irqchip_init();
    fn imx6_pm_ccm_init(compatible: *const i8);
    fn imx6sx_cpuidle_init();
    fn platform_device_register_simple(
        name: *const i8,
        id: i32,
        data: *const c_void,
        size: u32,
    ) -> *mut c_void;
}

unsafe fn imx6ul_init_machine() {
    let revision = if cpu_is_imx6ull() {
        b"i.MX6ULL\0".as_ptr() as *const i8
    } else {
        b"i.MX6UL\0".as_ptr() as *const i8
    };
    imx_print_silicon_rev(revision, imx_get_soc_revision());

    of_platform_default_populate(core::ptr::null(), core::ptr::null(), core::ptr::null());
    imx_anatop_init();
    imx6ul_pm_init();
}

unsafe fn imx6ul_init_irq() {
    imx_init_revision_from_anatop();
    imx_src_init();
    irqchip_init();
    imx6_pm_ccm_init(b"fsl,imx6ul-ccm\0".as_ptr() as *const i8);
}

unsafe fn imx6ul_init_late() {
    imx6sx_cpuidle_init();

    // Preserves: if (IS_ENABLED(CONFIG_ARM_IMX6Q_CPUFREQ)).
    #[cfg(CONFIG_ARM_IMX6Q_CPUFREQ)]
    {
        platform_device_register_simple(
            b"imx6q-cpufreq\0".as_ptr() as *const i8,
            -1,
            core::ptr::null(),
            0,
        );
    }
}

static IMX6UL_DT_COMPAT: [*const i8; 4] = [
    b"fsl,imx6ul\0".as_ptr() as *const i8,
    b"fsl,imx6ull\0".as_ptr() as *const i8,
    b"fsl,imx6ulz\0".as_ptr() as *const i8,
    core::ptr::null(),
];

// DT_MACHINE_START(IMX6UL, "Freescale i.MX6 Ultralite (Device Tree)")
// .init_irq = imx6ul_init_irq,
// .init_machine = imx6ul_init_machine,
// .init_late = imx6ul_init_late,
// .dt_compat = imx6ul_dt_compat,
// MACHINE_END
#[allow(dead_code)]
static IMX6UL_MACHINE: ImxMachineDesc = ImxMachineDesc {
    name: b"Freescale i.MX6 Ultralite (Device Tree)\0".as_ptr() as *const i8,
    init_irq: Some(imx6ul_init_irq),
    init_machine: Some(imx6ul_init_machine),
    init_late: Some(imx6ul_init_late),
    dt_compat: IMX6UL_DT_COMPAT.as_ptr(),
};

// Supplied by asm/mach/arch.h; layout follows the DT_MACHINE_START fields used above.
#[repr(C)]
struct ImxMachineDesc {
    name: *const i8,
    init_irq: Option<unsafe fn()>,
    init_machine: Option<unsafe fn()>,
    init_late: Option<unsafe fn()>,
    dt_compat: *const *const i8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
