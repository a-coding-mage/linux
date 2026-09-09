/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) ST-Ericsson SA 2010-2013
 * Author: Rickard Andersson <rickard.andersson@stericsson.com> for
 *         ST-Ericsson.
 * Author: Daniel Lezcano <daniel.lezcano@linaro.org> for Linaro.
 */

extern "C" {
    pub fn prcmu_gic_decouple() -> ::core::ffi::c_int;
    pub fn prcmu_gic_recouple() -> ::core::ffi::c_int;
    pub fn prcmu_gic_pending_irq() -> bool;
    pub fn prcmu_pending_irq() -> bool;
    pub fn prcmu_is_cpu_in_wfi(cpu: ::core::ffi::c_int) -> bool;
    pub fn prcmu_copy_gic_settings() -> ::core::ffi::c_int;
    pub fn ux500_pm_init(phy_base: u32, size: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
