/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright (C) 2021 Microsoft Corporation
 *
 * Author: Lakshmi Ramasubramanian (nramas@linux.microsoft.com)
 *
 * Measure critical data structures maintained by SELinux
 * using IMA subsystem.
 */

/* Depends on declarations from "security.h". */

#[cfg(CONFIG_IMA)]
unsafe extern "C" {
    /* C declaration used __init. */
    pub fn selinux_ima_config_len_init();
    pub fn selinux_ima_measure_state();
    pub fn selinux_ima_measure_state_locked();
}

#[cfg(not(CONFIG_IMA))]
#[inline]
pub fn selinux_ima_config_len_init() {}

#[cfg(not(CONFIG_IMA))]
#[inline]
pub fn selinux_ima_measure_state() {}

#[cfg(not(CONFIG_IMA))]
#[inline]
pub fn selinux_ima_measure_state_locked() {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
