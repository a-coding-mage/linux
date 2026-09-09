// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP2XXX powerdomain definitions
 *
 * Copyright (C) 2007-2008, 2011 Texas Instruments, Inc.
 * Copyright (C) 2007-2011 Nokia Corporation
 *
 * Paul Walmsley, Jouni Högander
 */

// Dependencies supplied by the surrounding kernel translation.

/* 24XX powerdomains and dependencies */

/* Powerdomains */

static mut dsp_pwrdm: powerdomain = powerdomain {
    name: "dsp_pwrdm",
    prcm_offs: OMAP24XX_DSP_MOD,
    pwrsts: PWRSTS_OFF_RET_ON,
    pwrsts_logic_ret: PWRSTS_RET,
    banks: 1,
    pwrsts_mem_ret: [PWRSTS_RET],
    pwrsts_mem_on: [PWRSTS_ON],
    voltdm: voltagedomain { name: "core" },
};

static mut mpu_24xx_pwrdm: powerdomain = powerdomain {
    name: "mpu_pwrdm",
    prcm_offs: MPU_MOD,
    pwrsts: PWRSTS_OFF_RET_ON,
    pwrsts_logic_ret: PWRSTS_OFF_RET,
    banks: 1,
    pwrsts_mem_ret: [PWRSTS_RET],
    pwrsts_mem_on: [PWRSTS_ON],
    voltdm: voltagedomain { name: "core" },
};

static mut core_24xx_pwrdm: powerdomain = powerdomain {
    name: "core_pwrdm",
    prcm_offs: CORE_MOD,
    pwrsts: PWRSTS_OFF_RET_ON,
    pwrsts_logic_ret: PWRSTS_RET,
    banks: 3,
    pwrsts_mem_ret: [PWRSTS_OFF_RET, PWRSTS_OFF_RET, PWRSTS_OFF_RET],
    pwrsts_mem_on: [PWRSTS_OFF_RET_ON, PWRSTS_OFF_RET_ON, PWRSTS_OFF_RET_ON],
    voltdm: voltagedomain { name: "core" },
};

/*
 * 2430-specific powerdomains
 */

/* XXX 2430 KILLDOMAINWKUP bit?  No current users apparently */

static mut mdm_pwrdm: powerdomain = powerdomain {
    name: "mdm_pwrdm",
    prcm_offs: OMAP2430_MDM_MOD,
    pwrsts: PWRSTS_OFF_RET_ON,
    pwrsts_logic_ret: PWRSTS_RET,
    banks: 1,
    pwrsts_mem_ret: [PWRSTS_RET],
    pwrsts_mem_on: [PWRSTS_ON],
    voltdm: voltagedomain { name: "core" },
};

static mut powerdomains_omap24xx: [*mut powerdomain; 6] = [
    unsafe { &raw mut wkup_omap2_pwrdm },
    unsafe { &raw mut gfx_omap2_pwrdm },
    unsafe { &raw mut dsp_pwrdm },
    unsafe { &raw mut mpu_24xx_pwrdm },
    unsafe { &raw mut core_24xx_pwrdm },
    core::ptr::null_mut(),
];

static mut powerdomains_omap2430: [*mut powerdomain; 2] = [
    unsafe { &raw mut mdm_pwrdm },
    core::ptr::null_mut(),
];

pub unsafe fn omap242x_powerdomains_init() {
    if !cpu_is_omap2420() {
        return;
    }

    pwrdm_register_platform_funcs(&raw const omap2_pwrdm_operations);
    pwrdm_register_pwrdms(powerdomains_omap24xx.as_mut_ptr());
    pwrdm_complete_init();
}

pub unsafe fn omap243x_powerdomains_init() {
    if !cpu_is_omap2430() {
        return;
    }

    pwrdm_register_platform_funcs(&raw const omap2_pwrdm_operations);
    pwrdm_register_pwrdms(powerdomains_omap24xx.as_mut_ptr());
    pwrdm_register_pwrdms(powerdomains_omap2430.as_mut_ptr());
    pwrdm_complete_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
