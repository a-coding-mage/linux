// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2012-2013 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_void;

const MSCM_CPXCOUNT: usize = 0x00c;
const MSCM_CPXCFG1: usize = 0x014;

extern "C" {
    fn of_find_compatible_node(
        from: *mut c_void,
        ty: *const c_void,
        compatible: *const u8,
    ) -> *mut c_void;
    fn of_node_put(node: *mut c_void);
    fn of_iomap(node: *mut c_void, index: i32) -> *mut c_void;
    fn readl_relaxed(address: *const c_void) -> u32;
    fn iounmap(address: *mut c_void);
    fn mxc_set_cpu_type(cpu_type: u32);
    fn of_platform_default_populate(
        root: *mut c_void,
        matches: *mut c_void,
        parent: *mut c_void,
    ) -> i32;
}

// Values are provided by the architecture headers.
extern "C" {
    static MXC_CPU_VF500: u32;
    static MXC_CPU_VF600: u32;
    static MXC_CPU_VFX10: u32;
}

#[inline]
unsafe fn vf610_detect_cpu() {
    let np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"fsl,vf610-mscm-cpucfg\0".as_ptr(),
    );
    // WARN_ON(!np)
    if np.is_null() {
        return;
    }

    let mscm = of_iomap(np, 0);
    of_node_put(np);

    // WARN_ON(!mscm)
    if mscm.is_null() {
        return;
    }

    let cpxcount = readl_relaxed((mscm as *const u8).add(MSCM_CPXCOUNT) as *const c_void);
    let cpxcfg1 = readl_relaxed((mscm as *const u8).add(MSCM_CPXCFG1) as *const c_void);

    iounmap(mscm);

    let mut cpu_type = if cpxcount != 0 {
        MXC_CPU_VF600
    } else {
        MXC_CPU_VF500
    };

    if cpxcfg1 != 0 {
        cpu_type |= MXC_CPU_VFX10;
    }

    mxc_set_cpu_type(cpu_type);
}

#[inline]
unsafe fn vf610_init_machine() {
    vf610_detect_cpu();
    of_platform_default_populate(
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        core::ptr::null_mut(),
    );
}

static VF610_DT_COMPAT: [*const u8; 6] = [
    b"fsl,vf500\0".as_ptr(),
    b"fsl,vf510\0".as_ptr(),
    b"fsl,vf600\0".as_ptr(),
    b"fsl,vf610\0".as_ptr(),
    b"fsl,vf610m4\0".as_ptr(),
    core::ptr::null(),
];

// DT_MACHINE_START(VYBRID_VF610, "Freescale Vybrid VF5xx/VF6xx (Device Tree)")
//     .l2c_aux_val = 0,
//     .l2c_aux_mask = ~0,
//     .init_machine = vf610_init_machine,
//     .dt_compat = vf610_dt_compat,
// MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
