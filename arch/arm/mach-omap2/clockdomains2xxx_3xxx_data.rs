// SPDX-License-Identifier: GPL-2.0
/*
 * OMAP2/3 clockdomain common data
 *
 * Copyright (C) 2008-2011 Texas Instruments, Inc.
 * Copyright (C) 2008-2010 Nokia Corporation
 *
 * Paul Walmsley, Jouni Högander
 *
 * This file contains clockdomains and clockdomain wakeup/sleep
 * dependencies for the OMAP2/3 chips.  Some notes:
 *
 * A useful validation rule for struct clockdomain: Any clockdomain
 * referenced by a wkdep_srcs or sleepdep_srcs array must have a
 * dep_bit assigned.  So wkdep_srcs/sleepdep_srcs are really just
 * software-controllable dependencies.  Non-software-controllable
 * dependencies do exist, but they are not encoded below (yet).
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

// C header dependencies are supplied by the surrounding translation unit.

/*
 * Clockdomain dependencies for wkdeps/sleepdeps
 *
 * XXX Hardware dependencies (e.g., dependencies that cannot be
 * changed in software) are not included here yet, but should be.
 */

/* Wakeup dependency source arrays */

/* 2xxx-specific possible dependencies */

/* 2xxx PM_WKDEP_GFX: CORE, MPU, WKUP */
pub static mut gfx_24xx_wkdeps: [clkdm_dep; 5] = [
    clkdm_dep { clkdm_name: b"core_l3_clkdm\0".as_ptr() as *const i8 },
    clkdm_dep { clkdm_name: b"core_l4_clkdm\0".as_ptr() as *const i8 },
    clkdm_dep { clkdm_name: b"mpu_clkdm\0".as_ptr() as *const i8 },
    clkdm_dep { clkdm_name: b"wkup_clkdm\0".as_ptr() as *const i8 },
    clkdm_dep { clkdm_name: core::ptr::null() },
];

/* 2xxx PM_WKDEP_DSP: CORE, MPU, WKUP */
pub static mut dsp_24xx_wkdeps: [clkdm_dep; 5] = [
    clkdm_dep { clkdm_name: b"core_l3_clkdm\0".as_ptr() as *const i8 },
    clkdm_dep { clkdm_name: b"core_l4_clkdm\0".as_ptr() as *const i8 },
    clkdm_dep { clkdm_name: b"mpu_clkdm\0".as_ptr() as *const i8 },
    clkdm_dep { clkdm_name: b"wkup_clkdm\0".as_ptr() as *const i8 },
    clkdm_dep { clkdm_name: core::ptr::null() },
];

/*
 * OMAP2/3-common clockdomains
 *
 * Even though the 2420 has a single PRCM module from the
 * interconnect's perspective, internally it does appear to have
 * separate PRM and CM clockdomains.  The usual test case is
 * sys_clkout/sys_clkout2.
 */

/* This is an implicit clockdomain - it is never defined as such in TRM */
pub static mut wkup_common_clkdm: clockdomain = clockdomain {
    name: b"wkup_clkdm\0".as_ptr() as *const i8,
    pwrdm: powerdomain { name: b"wkup_pwrdm\0".as_ptr() as *const i8 },
    dep_bit: OMAP_EN_WKUP_SHIFT,
    flags: CLKDM_ACTIVE_WITH_MPU,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
