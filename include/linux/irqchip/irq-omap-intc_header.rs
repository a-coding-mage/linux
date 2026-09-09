/* SPDX-License-Identifier: GPL-2.0-only */
/**
 * irq-omap-intc.h - INTC Idle Functions
 *
 * Copyright (C) 2014 Texas Instruments Incorporated - https://www.ti.com
 *
 * Author: Felipe Balbi <balbi@ti.com>
 */

extern "C" {
    pub fn omap_irq_pending() -> ::core::ffi::c_int;
    pub fn omap_intc_save_context();
    pub fn omap_intc_restore_context();
    pub fn omap3_intc_suspend();
    pub fn omap3_intc_prepare_idle();
    pub fn omap3_intc_resume_idle();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
