/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Low-level power-management support for Alpine platform.
 *
 * Copyright (C) 2015 Annapurna Labs Ltd.
 */

/* Alpine CPU Power Management Services Initialization */
unsafe extern "C" {
    pub fn alpine_cpu_pm_init();
}

/* Wake-up a CPU */
unsafe extern "C" {
    pub fn alpine_cpu_wakeup(phys_cpu: ::core::ffi::c_uint, phys_resume_addr: u32) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
