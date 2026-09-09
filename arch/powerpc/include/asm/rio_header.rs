/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * RapidIO architecture support
 *
 * Copyright 2005 MontaVista Software, Inc.
 * Matt Porter <mporter@kernel.crashing.org>
 */

/* C dependency: pt_regs is supplied by another header/module. */

/* CONFIG_FSL_RIO */
#[cfg(CONFIG_FSL_RIO)]
extern "C" {
    pub fn fsl_rio_mcheck_exception(regs: *mut pt_regs) -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_FSL_RIO))]
#[inline]
pub unsafe fn fsl_rio_mcheck_exception(_regs: *mut pt_regs) -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
