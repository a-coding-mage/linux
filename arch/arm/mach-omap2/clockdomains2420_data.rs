// SPDX-License-Identifier: GPL-2.0
/*
 * OMAP2420 clockdomains
 *
 * Copyright (C) 2008-2011 Texas Instruments, Inc.
 * Copyright (C) 2008-2010 Nokia Corporation
 *
 * Paul Walmsley, Jouni Högander
 *
 * This file contains clockdomains and clockdomain wakeup dependencies
 * for OMAP2420 chips.  Some notes:
 *
 * A useful validation rule for struct clockdomain: Any clockdomain
 * referenced by a wkdep_srcs must have a dep_bit assigned.  So
 * wkdep_srcs are really just software-controllable dependencies.
 * Non-software-controllable dependencies do exist, but they are not
 * encoded below (yet).
 *
 * 24xx does not support programmable sleep dependencies (SLEEPDEP)
 */

// C header dependencies are supplied by the surrounding translation unit.

/* Clockdomain dependencies for wkdeps */

/* 2420-specific possible wakeup dependencies */

/* 2420 PM_WKDEP_MPU: CORE, DSP, WKUP */
static mut mpu_2420_wkdeps: [clkdm_dep; 5] = [
    clkdm_dep { clkdm_name: "core_l3_clkdm" },
    clkdm_dep { clkdm_name: "core_l4_clkdm" },
    clkdm_dep { clkdm_name: "dsp_clkdm" },
    clkdm_dep { clkdm_name: "wkup_clkdm" },
    clkdm_dep { clkdm_name: core::ptr::null() },
];

/* 2420 PM_WKDEP_CORE: DSP, GFX, MPU, WKUP */
static mut core_2420_wkdeps: [clkdm_dep; 5] = [
    clkdm_dep { clkdm_name: "dsp_clkdm" },
    clkdm_dep { clkdm_name: "gfx_clkdm" },
    clkdm_dep { clkdm_name: "mpu_clkdm" },
    clkdm_dep { clkdm_name: "wkup_clkdm" },
    clkdm_dep { clkdm_name: core::ptr::null() },
];

/* 2420-only clockdomains */

static mut mpu_2420_clkdm: clockdomain = clockdomain {
    name: "mpu_clkdm",
    pwrdm: powerdomain { name: "mpu_pwrdm" },
    flags: CLKDM_CAN_HWSUP,
    wkdep_srcs: unsafe { mpu_2420_wkdeps.as_mut_ptr() },
    clktrctrl_mask: OMAP24XX_AUTOSTATE_MPU_MASK,
};

static mut iva1_2420_clkdm: clockdomain = clockdomain {
    name: "iva1_clkdm",
    pwrdm: powerdomain { name: "dsp_pwrdm" },
    flags: CLKDM_CAN_HWSUP_SWSUP,
    dep_bit: OMAP24XX_PM_WKDEP_MPU_EN_DSP_SHIFT,
    wkdep_srcs: dsp_24xx_wkdeps,
    clktrctrl_mask: OMAP2420_AUTOSTATE_IVA_MASK,
};

static mut dsp_2420_clkdm: clockdomain = clockdomain {
    name: "dsp_clkdm",
    pwrdm: powerdomain { name: "dsp_pwrdm" },
    flags: CLKDM_CAN_HWSUP_SWSUP,
    clktrctrl_mask: OMAP24XX_AUTOSTATE_DSP_MASK,
};

static mut gfx_2420_clkdm: clockdomain = clockdomain {
    name: "gfx_clkdm",
    pwrdm: powerdomain { name: "gfx_pwrdm" },
    flags: CLKDM_CAN_HWSUP_SWSUP,
    wkdep_srcs: gfx_24xx_wkdeps,
    clktrctrl_mask: OMAP24XX_AUTOSTATE_GFX_MASK,
};

static mut core_l3_2420_clkdm: clockdomain = clockdomain {
    name: "core_l3_clkdm",
    pwrdm: powerdomain { name: "core_pwrdm" },
    flags: CLKDM_CAN_HWSUP,
    wkdep_srcs: unsafe { core_2420_wkdeps.as_mut_ptr() },
    clktrctrl_mask: OMAP24XX_AUTOSTATE_L3_MASK,
};

static mut core_l4_2420_clkdm: clockdomain = clockdomain {
    name: "core_l4_clkdm",
    pwrdm: powerdomain { name: "core_pwrdm" },
    flags: CLKDM_CAN_HWSUP,
    wkdep_srcs: unsafe { core_2420_wkdeps.as_mut_ptr() },
    clktrctrl_mask: OMAP24XX_AUTOSTATE_L4_MASK,
};

static mut dss_2420_clkdm: clockdomain = clockdomain {
    name: "dss_clkdm",
    pwrdm: powerdomain { name: "core_pwrdm" },
    flags: CLKDM_CAN_HWSUP,
    clktrctrl_mask: OMAP24XX_AUTOSTATE_DSS_MASK,
};

static mut clockdomains_omap242x: [*mut clockdomain; 9] = [
    &raw mut wkup_common_clkdm,
    &raw mut mpu_2420_clkdm,
    &raw mut iva1_2420_clkdm,
    &raw mut dsp_2420_clkdm,
    &raw mut gfx_2420_clkdm,
    &raw mut core_l3_2420_clkdm,
    &raw mut core_l4_2420_clkdm,
    &raw mut dss_2420_clkdm,
    core::ptr::null_mut(),
];

pub unsafe extern "C" fn omap242x_clockdomains_init() {
    if !cpu_is_omap242x() {
        return;
    }

    clkdm_register_platform_funcs(&omap2_clkdm_operations);
    clkdm_register_clkdms(clockdomains_omap242x.as_mut_ptr());
    clkdm_complete_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
