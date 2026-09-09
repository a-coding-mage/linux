// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP3 voltage domain data
 *
 * Copyright (C) 2007, 2010 Texas Instruments, Inc.
 * Rajendra Nayak <rnayak@ti.com>
 * Lesly A M <x0080970@ti.com>
 * Thara Gopinath <thara@ti.com>
 *
 * Copyright (C) 2008, 2011 Nokia Corporation
 * Kalle Jokiniemi
 * Paul Walmsley
 */

// Kernel and OMAP declarations are supplied by the surrounding translation unit.

/*
 * VDD data
 */

/* OMAP3-common voltagedomain data */

static mut omap3_voltdm_wkup: voltagedomain = voltagedomain {
    name: "wakeup",
};

/* 34xx/36xx voltagedomain data */

static omap3_vdd1_vfsm: omap_vfsm_instance = omap_vfsm_instance {
    voltsetup_reg: OMAP3_PRM_VOLTSETUP1_OFFSET,
    voltsetup_mask: OMAP3430_SETUP_TIME1_MASK,
};

static omap3_vdd2_vfsm: omap_vfsm_instance = omap_vfsm_instance {
    voltsetup_reg: OMAP3_PRM_VOLTSETUP1_OFFSET,
    voltsetup_mask: OMAP3430_SETUP_TIME2_MASK,
};

static mut omap3_voltdm_mpu: voltagedomain = voltagedomain {
    name: "mpu_iva",
    scalable: true,
    read: Some(omap3_prm_vcvp_read),
    write: Some(omap3_prm_vcvp_write),
    rmw: Some(omap3_prm_vcvp_rmw),
    vc: unsafe { &raw mut omap3_vc_mpu },
    vfsm: &omap3_vdd1_vfsm,
    vp: unsafe { &raw mut omap3_vp_mpu },
};

static mut omap3_voltdm_core: voltagedomain = voltagedomain {
    name: "core",
    scalable: true,
    read: Some(omap3_prm_vcvp_read),
    write: Some(omap3_prm_vcvp_write),
    rmw: Some(omap3_prm_vcvp_rmw),
    vc: unsafe { &raw mut omap3_vc_core },
    vfsm: &omap3_vdd2_vfsm,
    vp: unsafe { &raw mut omap3_vp_core },
};

static mut voltagedomains_omap3: [*mut voltagedomain; 4] = [
    &raw mut omap3_voltdm_mpu,
    &raw mut omap3_voltdm_core,
    &raw mut omap3_voltdm_wkup,
    core::ptr::null_mut(),
];

/* AM35xx voltagedomain data */

static mut am35xx_voltdm_mpu: voltagedomain = voltagedomain {
    name: "mpu_iva",
};

static mut am35xx_voltdm_core: voltagedomain = voltagedomain {
    name: "core",
};

static mut voltagedomains_am35xx: [*mut voltagedomain; 4] = [
    &raw mut am35xx_voltdm_mpu,
    &raw mut am35xx_voltdm_core,
    &raw mut omap3_voltdm_wkup,
    core::ptr::null_mut(),
];

static sys_clk_name: &str = "sys_ck";

pub unsafe fn omap3xxx_voltagedomains_init() {
    let mut voltdm: *mut voltagedomain;
    let voltdms: *mut [*mut voltagedomain; 4];
    let mut i: usize;

    /*
     * XXX Will depend on the process, validation, and binning
     * for the currently-running IC
     */
    // CONFIG_PM_OPP controls this block at build time in the C source.
    #[cfg(CONFIG_PM_OPP)]
    {
        if cpu_is_omap3630() {
            (*(&raw mut omap3_voltdm_mpu)).volt_data = omap36xx_vddmpu_volt_data;
            (*(&raw mut omap3_voltdm_core)).volt_data = omap36xx_vddcore_volt_data;
        } else {
            (*(&raw mut omap3_voltdm_mpu)).volt_data = omap34xx_vddmpu_volt_data;
            (*(&raw mut omap3_voltdm_core)).volt_data = omap34xx_vddcore_volt_data;
        }
    }

    omap3_voltdm_mpu.vp_param = &raw mut omap3_mpu_vp_data;
    omap3_voltdm_core.vp_param = &raw mut omap3_core_vp_data;
    omap3_voltdm_mpu.vc_param = &raw mut omap3_mpu_vc_data;
    omap3_voltdm_core.vc_param = &raw mut omap3_core_vc_data;

    voltdms = if soc_is_am35xx() {
        &raw mut voltagedomains_am35xx
    } else {
        &raw mut voltagedomains_omap3
    };

    i = 0;
    loop {
        voltdm = (*voltdms)[i];
        if voltdm.is_null() {
            break;
        }
        (*voltdm).sys_clk.name = sys_clk_name;
        i += 1;
    }

    voltdm_init(voltdms);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
