// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2011 Freescale Semiconductor, Inc. All Rights Reserved.
 * Copyright 2011 Linaro Ltd.
 */

// Dependency intent from <asm/mach/arch.h>, "common.h", and "hardware.h".

unsafe extern "C" {
    fn mxc_set_cpu_type(cpu_type: u32);
    fn imx_src_init();
    fn imx5_pmu_init();
    fn imx_aips_allow_unprivileged_access(compat: *const core::ffi::c_char);
    fn imx53_pm_init();
}

// MXC_CPU_MX53 is supplied by the machine-specific dependency headers.
const MXC_CPU_MX53: u32 = 53;

unsafe fn imx53_init_early() {
    unsafe {
        mxc_set_cpu_type(MXC_CPU_MX53);
    }
}

unsafe fn imx53_dt_init() {
    unsafe {
        imx_src_init();
        imx5_pmu_init();
        imx_aips_allow_unprivileged_access(
            b"fsl,imx53-aipstz\0".as_ptr() as *const core::ffi::c_char,
        );
    }
}

unsafe fn imx53_init_late() {
    unsafe {
        imx53_pm_init();
    }
}

static IMX53_DT_BOARD_COMPAT: [*const core::ffi::c_char; 2] = [
    b"fsl,imx53\0".as_ptr() as *const core::ffi::c_char,
    core::ptr::null(),
];

// Direct Rust representation of the DT_MACHINE_START(IMX53_DT, ...) machine
// descriptor and its MACHINE_END terminator.
#[repr(C)]
struct MachineDesc {
    name: &'static str,
    init_early: unsafe fn(),
    init_machine: unsafe fn(),
    init_late: unsafe fn(),
    dt_compat: *const *const core::ffi::c_char,
}

#[used]
static IMX53_DT: MachineDesc = MachineDesc {
    name: "Freescale i.MX53 (Device Tree Support)",
    init_early: imx53_init_early,
    init_machine: imx53_dt_init,
    init_late: imx53_init_late,
    dt_compat: IMX53_DT_BOARD_COMPAT.as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
