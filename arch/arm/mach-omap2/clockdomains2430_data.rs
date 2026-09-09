// SPDX-License-Identifier: GPL-2.0
/*
 * OMAP2xxx clockdomains
 *
 * Copyright (C) 2008-2009 Texas Instruments, Inc.
 * Copyright (C) 2008-2010 Nokia Corporation
 *
 * Paul Walmsley, Jouni Högander
 *
 * This file contains clockdomains and clockdomain wakeup dependencies
 * for OMAP2xxx chips.  Some notes:
 *
 * A useful validation rule for struct clockdomain: Any clockdomain
 * referenced by a wkdep_srcs must have a dep_bit assigned.  So
 * wkdep_srcs are really just software-controllable dependencies.
 * Non-software-controllable dependencies do exist, but they are not
 * encoded below (yet).
 *
 * 24xx does not support programmable sleep dependencies (SLEEPDEP)
 *
 * The overly-specific dep_bit names are due to a bit name collision
 * with CM_FCLKEN_{DSP,IVA2}.  The DSP/IVA2 PM_WKDEP and CM_SLEEPDEP shift
 * value are the same for all powerdomains: 2
 *
 * XXX should dep_bit be a mask, so we can test to see if it is 0 as a
 * sanity check?
 * XXX encode hardware fixed wakeup dependencies -- esp. for 3430 CORE
 */

/*
 * To-Do List
 * -> Port the Sleep/Wakeup dependencies for the domains
 *    from the Power domain framework
 */

/* Dependencies supplied by the surrounding OMAP clockdomain implementation. */

/*
 * Clockdomain dependencies for wkdeps
 *
 * XXX Hardware dependencies (e.g., dependencies that cannot be
 * changed in software) are not included here yet, but should be.
 */

/* Wakeup dependency source arrays */

/* 2430-specific possible wakeup dependencies */

/* 2430 PM_WKDEP_CORE: DSP, GFX, MPU, WKUP, MDM */
static mut core_2430_wkdeps: [clkdm_dep; 6] = [
    clkdm_dep { clkdm_name: b"dsp_clkdm\0".as_ptr() as *const _ },
    clkdm_dep { clkdm_name: b"gfx_clkdm\0".as_ptr() as *const _ },
    clkdm_dep { clkdm_name: b"mpu_clkdm\0".as_ptr() as *const _ },
    clkdm_dep { clkdm_name: b"wkup_clkdm\0".as_ptr() as *const _ },
    clkdm_dep { clkdm_name: b"mdm_clkdm\0".as_ptr() as *const _ },
    clkdm_dep { clkdm_name: core::ptr::null() },
];

/* 2430 PM_WKDEP_MPU: CORE, DSP, WKUP, MDM */
static mut mpu_2430_wkdeps: [clkdm_dep; 6] = [
    clkdm_dep { clkdm_name: b"core_l3_clkdm\0".as_ptr() as *const _ },
    clkdm_dep { clkdm_name: b"core_l4_clkdm\0".as_ptr() as *const _ },
    clkdm_dep { clkdm_name: b"dsp_clkdm\0".as_ptr() as *const _ },
    clkdm_dep { clkdm_name: b"wkup_clkdm\0".as_ptr() as *const _ },
    clkdm_dep { clkdm_name: b"mdm_clkdm\0".as_ptr() as *const _ },
    clkdm_dep { clkdm_name: core::ptr::null() },
];

/* 2430 PM_WKDEP_MDM: CORE, MPU, WKUP */
static mut mdm_2430_wkdeps: [clkdm_dep; 5] = [
    clkdm_dep { clkdm_name: b"core_l3_clkdm\0".as_ptr() as *const _ },
    clkdm_dep { clkdm_name: b"core_l4_clkdm\0".as_ptr() as *const _ },
    clkdm_dep { clkdm_name: b"mpu_clkdm\0".as_ptr() as *const _ },
    clkdm_dep { clkdm_name: b"wkup_clkdm\0".as_ptr() as *const _ },
    clkdm_dep { clkdm_name: core::ptr::null() },
];

/* 2430-only clockdomains */

static mut mpu_2430_clkdm: clockdomain = clockdomain {
    name: b"mpu_clkdm\0".as_ptr() as *const _,
    pwrdm: powerdomain { name: b"mpu_pwrdm\0".as_ptr() as *const _ },
    flags: CLKDM_CAN_HWSUP_SWSUP,
    wkdep_srcs: mpu_2430_wkdeps.as_mut_ptr(),
    clktrctrl_mask: OMAP24XX_AUTOSTATE_MPU_MASK,
};

/* Another case of bit name collisions between several registers: EN_MDM */
static mut mdm_clkdm: clockdomain = clockdomain {
    name: b"mdm_clkdm\0".as_ptr() as *const _,
    pwrdm: powerdomain { name: b"mdm_pwrdm\0".as_ptr() as *const _ },
    flags: CLKDM_CAN_HWSUP_SWSUP,
    dep_bit: OMAP2430_PM_WKDEP_MPU_EN_MDM_SHIFT,
    wkdep_srcs: mdm_2430_wkdeps.as_mut_ptr(),
    clktrctrl_mask: OMAP2430_AUTOSTATE_MDM_MASK,
};

static mut dsp_2430_clkdm: clockdomain = clockdomain {
    name: b"dsp_clkdm\0".as_ptr() as *const _,
    pwrdm: powerdomain { name: b"dsp_pwrdm\0".as_ptr() as *const _ },
    flags: CLKDM_CAN_HWSUP_SWSUP,
    dep_bit: OMAP24XX_PM_WKDEP_MPU_EN_DSP_SHIFT,
    wkdep_srcs: dsp_24xx_wkdeps,
    clktrctrl_mask: OMAP24XX_AUTOSTATE_DSP_MASK,
};

static mut gfx_2430_clkdm: clockdomain = clockdomain {
    name: b"gfx_clkdm\0".as_ptr() as *const _,
    pwrdm: powerdomain { name: b"gfx_pwrdm\0".as_ptr() as *const _ },
    flags: CLKDM_CAN_HWSUP_SWSUP,
    wkdep_srcs: gfx_24xx_wkdeps,
    clktrctrl_mask: OMAP24XX_AUTOSTATE_GFX_MASK,
};

/*
 * XXX add usecounting for clkdm dependencies, otherwise the presence
 * of a single dep bit for core_l3_24xx_clkdm and core_l4_24xx_clkdm
 * could cause trouble
 */
static mut core_l3_2430_clkdm: clockdomain = clockdomain {
    name: b"core_l3_clkdm\0".as_ptr() as *const _,
    pwrdm: powerdomain { name: b"core_pwrdm\0".as_ptr() as *const _ },
    flags: CLKDM_CAN_HWSUP,
    dep_bit: OMAP24XX_EN_CORE_SHIFT,
    wkdep_srcs: core_2430_wkdeps.as_mut_ptr(),
    clktrctrl_mask: OMAP24XX_AUTOSTATE_L3_MASK,
};

/* Same dependency/usecounting note as the corresponding C definition. */
static mut core_l4_2430_clkdm: clockdomain = clockdomain {
    name: b"core_l4_clkdm\0".as_ptr() as *const _,
    pwrdm: powerdomain { name: b"core_pwrdm\0".as_ptr() as *const _ },
    flags: CLKDM_CAN_HWSUP,
    dep_bit: OMAP24XX_EN_CORE_SHIFT,
    wkdep_srcs: core_2430_wkdeps.as_mut_ptr(),
    clktrctrl_mask: OMAP24XX_AUTOSTATE_L4_MASK,
};

static mut dss_2430_clkdm: clockdomain = clockdomain {
    name: b"dss_clkdm\0".as_ptr() as *const _,
    pwrdm: powerdomain { name: b"core_pwrdm\0".as_ptr() as *const _ },
    flags: CLKDM_CAN_HWSUP,
    clktrctrl_mask: OMAP24XX_AUTOSTATE_DSS_MASK,
};

static mut clockdomains_omap243x: [*mut clockdomain; 9] = [
    &raw mut wkup_common_clkdm,
    &raw mut mpu_2430_clkdm,
    &raw mut mdm_clkdm,
    &raw mut dsp_2430_clkdm,
    &raw mut gfx_2430_clkdm,
    &raw mut core_l3_2430_clkdm,
    &raw mut core_l4_2430_clkdm,
    &raw mut dss_2430_clkdm,
    core::ptr::null_mut(),
];

pub unsafe fn omap243x_clockdomains_init() {
    if !cpu_is_omap243x() {
        return;
    }

    clkdm_register_platform_funcs(&omap2_clkdm_operations);
    clkdm_register_clkdms(clockdomains_omap243x.as_mut_ptr());
    clkdm_complete_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
