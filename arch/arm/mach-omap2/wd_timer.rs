// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OMAP2+ MPU WD_TIMER-specific code
 *
 * Copyright (C) 2012 Texas Instruments, Inc.
 */

// Dependencies are supplied by the surrounding kernel translation.

const OMAP_WDT_WPS: usize = 0x34;
const OMAP_WDT_SPR: usize = 0x48;

pub unsafe fn omap2_wd_timer_disable(oh: *mut omap_hwmod) -> i32 {
    let base: *mut core::ffi::c_void;

    if oh.is_null() {
        pr_err!("{}: Could not look up wdtimer_hwmod\n", "omap2_wd_timer_disable");
        return -EINVAL;
    }

    base = omap_hwmod_get_mpu_rt_va(oh);
    if base.is_null() {
        pr_err!("{}: Could not get the base address for {}\n",
                (*oh).name, "omap2_wd_timer_disable");
        return -EINVAL;
    }

    /* sequence required to disable watchdog */
    writel_relaxed(0xAAAA, (base as *mut u8).add(OMAP_WDT_SPR) as *mut core::ffi::c_void);
    while readl_relaxed((base as *mut u8).add(OMAP_WDT_WPS) as *mut core::ffi::c_void) & 0x10 != 0 {
        cpu_relax();
    }

    writel_relaxed(0x5555, (base as *mut u8).add(OMAP_WDT_SPR) as *mut core::ffi::c_void);
    while readl_relaxed((base as *mut u8).add(OMAP_WDT_WPS) as *mut core::ffi::c_void) & 0x10 != 0 {
        cpu_relax();
    }

    0
}

/**
 * omap2_wd_timer_reset - reset and disable the WDTIMER IP block
 * @oh: struct omap_hwmod *
 *
 * After the WDTIMER IP blocks are reset on OMAP2/3, we must also take
 * care to execute the special watchdog disable sequence.  This is
 * because the watchdog is re-armed upon OCP softreset.  (On OMAP4,
 * this behavior was apparently changed and the watchdog is no longer
 * re-armed after an OCP soft-reset.)  Returns -ETIMEDOUT if the reset
 * did not complete, or 0 upon success.
 *
 * XXX Most of this code should be moved to the omap_hwmod.c layer
 * during a normal merge window.  omap_hwmod_softreset() should be
 * renamed to omap_hwmod_set_ocp_softreset(), and omap_hwmod_softreset()
 * should call the hwmod _ocp_softreset() code.
 *
 * Returns: %0 on success or -errno value on error.
 */
pub unsafe fn omap2_wd_timer_reset(oh: *mut omap_hwmod) -> i32 {
    let mut c: i32 = 0;

    /* Write to the SOFTRESET bit */
    omap_hwmod_softreset(oh);

    /* Poll on RESETDONE bit */
    while (omap_hwmod_read(oh, (*(*oh).class).sysc.sysc.syss_offs)
        & SYSS_RESETDONE_MASK) == 0 && c < MAX_MODULE_SOFTRESET_WAIT {
        cpu_relax();
        c += 1;
    }

    if (*(*oh).class).sysc.srst_udelay != 0 {
        udelay((*(*oh).class).sysc.srst_udelay);
    }

    if c == MAX_MODULE_SOFTRESET_WAIT {
        pr_warn!("{}: {}: softreset failed (waited {} usec)\n",
                 "omap2_wd_timer_reset", (*oh).name, MAX_MODULE_SOFTRESET_WAIT);
    } else {
        pr_debug!("{}: {}: softreset in {} usec\n",
                  "omap2_wd_timer_reset", (*oh).name, c);
    }

    if c == MAX_MODULE_SOFTRESET_WAIT {
        -ETIMEDOUT
    } else {
        omap2_wd_timer_disable(oh)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
