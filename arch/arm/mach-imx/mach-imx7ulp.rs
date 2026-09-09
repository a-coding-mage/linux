// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2016 Freescale Semiconductor, Inc.
 * Copyright 2017-2018 NXP
 *   Author: Dong Aisheng <aisheng.dong@nxp.com>
 */

// Dependencies supplied by the surrounding kernel translation.

const SIM_JTAG_ID_REG: u32 = 0x8c;

#[repr(C)]
pub struct Regmap {
    _private: [u8; 0],
}

extern "C" {
    fn syscon_regmap_lookup_by_compatible(compatible: *const core::ffi::c_char) -> *mut Regmap;
    fn IS_ERR(ptr: *mut Regmap) -> bool;
    fn pr_warn(message: *const core::ffi::c_char, ...);
    fn regmap_read(map: *mut Regmap, register: u32, value: *mut u32) -> i32;
    fn imx_set_soc_revision(revision: u32);
    fn imx7ulp_pm_init();
    fn mxc_set_cpu_type(cpu_type: u32);
    fn of_platform_default_populate(
        root: *mut core::ffi::c_void,
        matches: *mut core::ffi::c_void,
        parent: *mut core::ffi::c_void,
    ) -> i32;
    fn platform_device_register_simple(
        name: *const core::ffi::c_char,
        id: i32,
        resources: *mut core::ffi::c_void,
        num_resources: u32,
    ) -> *mut core::ffi::c_void;
    fn imx7ulp_cpuidle_init();
}

const IMX_CHIP_REVISION_1_0: u32 = 0;
const IMX_CHIP_REVISION_2_0: u32 = 1;
const IMX_CHIP_REVISION_2_1: u32 = 2;
const IMX_CHIP_REVISION_2_2: u32 = 3;
const MXC_CPU_IMX7ULP: u32 = 0;

#[inline]
unsafe fn imx7ulp_set_revision() {
    let sim = syscon_regmap_lookup_by_compatible(b"fsl,imx7ulp-sim\0".as_ptr() as *const _);
    if IS_ERR(sim) {
        pr_warn(b"failed to find fsl,imx7ulp-sim regmap!\n\0".as_ptr() as *const _);
        return;
    }

    let mut revision: u32 = 0;
    if regmap_read(sim, SIM_JTAG_ID_REG, &mut revision) != 0 {
        pr_warn(b"failed to read sim regmap!\n\0".as_ptr() as *const _);
        return;
    }

    /*
     * bit[31:28] of JTAG_ID register defines revision as below from B0:
     * 0001        B0
     * 0010        B1
     * 0011        B2
     */
    match revision >> 28 {
        1 => imx_set_soc_revision(IMX_CHIP_REVISION_2_0),
        2 => imx_set_soc_revision(IMX_CHIP_REVISION_2_1),
        3 => imx_set_soc_revision(IMX_CHIP_REVISION_2_2),
        _ => imx_set_soc_revision(IMX_CHIP_REVISION_1_0),
    }
}

#[inline]
unsafe fn imx7ulp_init_machine() {
    imx7ulp_pm_init();

    mxc_set_cpu_type(MXC_CPU_IMX7ULP);
    imx7ulp_set_revision();
    of_platform_default_populate(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
}

static IMX7ULP_DT_COMPAT: [*const core::ffi::c_char; 2] = [
    b"fsl,imx7ulp\0".as_ptr() as *const _,
    core::ptr::null(),
];

#[inline]
unsafe fn imx7ulp_init_late() {
    // CONFIG_ARM_IMX_CPUFREQ_DT is a build-time condition from the original source.
    #[cfg(CONFIG_ARM_IMX_CPUFREQ_DT)]
    {
        platform_device_register_simple(
            b"imx-cpufreq-dt\0".as_ptr() as *const _,
            -1,
            core::ptr::null_mut(),
            0,
        );
    }

    imx7ulp_cpuidle_init();
}

// DT_MACHINE_START(IMX7ulp, "Freescale i.MX7ULP (Device Tree)")
// MACHINE_END
#[repr(C)]
pub struct MachineDesc {
    pub init_machine: unsafe fn(),
    pub dt_compat: *const *const core::ffi::c_char,
    pub init_late: unsafe fn(),
}

#[no_mangle]
pub static IMX7ULP_MACHINE: MachineDesc = MachineDesc {
    init_machine: imx7ulp_init_machine,
    dt_compat: IMX7ULP_DT_COMPAT.as_ptr(),
    init_late: imx7ulp_init_late,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
