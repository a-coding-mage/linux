// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP5 Voltage Management Routines
 *
 * Based on voltagedomains44xx_data.c
 *
 * Copyright (C) 2013 Texas Instruments Incorporated - https://www.ti.com
 */

// Dependencies supplied by the surrounding kernel translation.

extern "C" {
    static mut omap4_vc_mpu: omap_vc_instance;
    static mut omap4_vc_iva: omap_vc_instance;
    static mut omap4_vc_core: omap_vc_instance;
    static mut omap4_vp_mpu: omap_vp_instance;
    static mut omap4_vp_iva: omap_vp_instance;
    static mut omap4_vp_core: omap_vp_instance;

    fn omap4_prm_vcvp_read(
        voltdm: *mut voltagedomain,
        reg: u8,
    ) -> u32;
    fn omap4_prm_vcvp_write(
        voltdm: *mut voltagedomain,
        reg: u8,
        val: u32,
    );
    fn omap4_prm_vcvp_rmw(
        voltdm: *mut voltagedomain,
        reg: u8,
        mask: u32,
        bits: u32,
    );
    fn voltdm_init(voltdms: *mut *mut voltagedomain);
}

// Types and register offsets are declared by the included kernel headers.

static omap5_vdd_mpu_vfsm: omap_vfsm_instance = omap_vfsm_instance {
    voltsetup_reg: OMAP54XX_PRM_VOLTSETUP_MPU_RET_SLEEP_OFFSET,
};

static omap5_vdd_mm_vfsm: omap_vfsm_instance = omap_vfsm_instance {
    voltsetup_reg: OMAP54XX_PRM_VOLTSETUP_MM_RET_SLEEP_OFFSET,
};

static omap5_vdd_core_vfsm: omap_vfsm_instance = omap_vfsm_instance {
    voltsetup_reg: OMAP54XX_PRM_VOLTSETUP_CORE_RET_SLEEP_OFFSET,
};

static mut omap5_voltdm_mpu: voltagedomain = voltagedomain {
    name: b"mpu\0".as_ptr() as *const i8,
    scalable: true,
    read: Some(omap4_prm_vcvp_read),
    write: Some(omap4_prm_vcvp_write),
    rmw: Some(omap4_prm_vcvp_rmw),
    vc: unsafe { &mut omap4_vc_mpu },
    vfsm: &omap5_vdd_mpu_vfsm,
    vp: unsafe { &mut omap4_vp_mpu },
};

static mut omap5_voltdm_mm: voltagedomain = voltagedomain {
    name: b"mm\0".as_ptr() as *const i8,
    scalable: true,
    read: Some(omap4_prm_vcvp_read),
    write: Some(omap4_prm_vcvp_write),
    rmw: Some(omap4_prm_vcvp_rmw),
    vc: unsafe { &mut omap4_vc_iva },
    vfsm: &omap5_vdd_mm_vfsm,
    vp: unsafe { &mut omap4_vp_iva },
};

static mut omap5_voltdm_core: voltagedomain = voltagedomain {
    name: b"core\0".as_ptr() as *const i8,
    scalable: true,
    read: Some(omap4_prm_vcvp_read),
    write: Some(omap4_prm_vcvp_write),
    rmw: Some(omap4_prm_vcvp_rmw),
    vc: unsafe { &mut omap4_vc_core },
    vfsm: &omap5_vdd_core_vfsm,
    vp: unsafe { &mut omap4_vp_core },
};

static mut omap5_voltdm_wkup: voltagedomain = voltagedomain {
    name: b"wkup\0".as_ptr() as *const i8,
};

static mut voltagedomains_omap5: [*mut voltagedomain; 5] = [
    unsafe { &mut omap5_voltdm_mpu },
    unsafe { &mut omap5_voltdm_mm },
    unsafe { &mut omap5_voltdm_core },
    unsafe { &mut omap5_voltdm_wkup },
    core::ptr::null_mut(),
];

static sys_clk_name: *const i8 = b"sys_clkin\0".as_ptr() as *const i8;

pub unsafe extern "C" fn omap54xx_voltagedomains_init() {
    let mut i: usize = 0;
    loop {
        let voltdm = voltagedomains_omap5[i];
        if voltdm.is_null() {
            break;
        }
        (*voltdm).sys_clk.name = sys_clk_name;
        i += 1;
    }

    voltdm_init(voltagedomains_omap5.as_mut_ptr());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
