// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/clock.c - SuperH clock framework
 *
 *  Copyright (C) 2005 - 2009  Paul Mundt
 *
 * This clock framework is derived from the OMAP version by:
 *
 *	Copyright (C) 2004 - 2008 Nokia Corporation
 *	Written by Tuukka Tikkanen <tuukka.tikkanen@elektrobit.com>
 *
 *  Modified for omap shared clock framework by Tony Lindgren <tony@atomide.com>
 */

// The following symbols are supplied by the surrounding kernel.
unsafe extern "C" {
    fn arch_clk_init() -> i32;
    fn recalculate_root_clocks();
    fn clk_enable_init_clocks();
    fn pr_err(fmt: *const core::ffi::c_char, ...);
}

#[repr(C)]
pub struct ShMachineVector {
    pub mv_clk_init: Option<unsafe extern "C" fn() -> i32>,
}

unsafe extern "C" {
    pub static mut sh_mv: ShMachineVector;
}

#[inline]
unsafe fn unlikely(value: bool) -> bool {
    value
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn clk_init() -> i32 {
    let mut ret: i32 = 0;

    // CONFIG_COMMON_CLK: retain the original conditional compilation intent.
    #[cfg(not(CONFIG_COMMON_CLK))]
    {
        ret = arch_clk_init();
        if unlikely(ret != 0) {
            // %s: CPU clock registration failed.\n
            return ret;
        }
    }

    if let Some(mv_clk_init) = (*core::ptr::addr_of!(sh_mv)).mv_clk_init {
        ret = mv_clk_init();
        if unlikely(ret != 0) {
            // %s: machvec clock initialization failed.\n
            return ret;
        }
    }

    // CONFIG_COMMON_CLK: retain the original conditional compilation intent.
    #[cfg(not(CONFIG_COMMON_CLK))]
    {
        /* Kick the child clocks.. */
        recalculate_root_clocks();

        /* Enable the necessary init clocks */
        clk_enable_init_clocks();
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
