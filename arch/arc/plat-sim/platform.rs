// SPDX-License-Identifier: GPL-2.0-only
/*
 * ARC simulation Platform support code
 *
 * Copyright (C) 2012 Synopsys, Inc. (www.synopsys.com)
 */

// The C source includes <linux/init.h> and <asm/mach_desc.h>.
// Their declarations and build-time configuration are supplied externally.

/*----------------------- Machine Descriptions ------------------------------
 *
 * Machine description is simply a set of platform/board specific callbacks
 * This is not directly related to DeviceTree based dynamic device creation,
 * however as part of early device tree scan, we also select the right
 * callback set, by matching the DT compatible name.
 */

use core::ffi::c_char;

#[cfg(CONFIG_ISA_ARCOMPACT)]
static simulation_compat: [*const c_char; 3] = [
    b"snps,nsim\0".as_ptr() as *const c_char,
    b"snps,nsimosci\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

#[cfg(not(CONFIG_ISA_ARCOMPACT))]
static simulation_compat: [*const c_char; 3] = [
    b"snps,nsimosci_hs\0".as_ptr() as *const c_char,
    b"snps,zebu_hs\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// C equivalent:
// MACHINE_START(SIMULATION, "simulation")
//     .dt_compat = simulation_compat,
// MACHINE_END
// The MACHINE_START/MACHINE_END registration is provided by the external
// machine-description infrastructure.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
