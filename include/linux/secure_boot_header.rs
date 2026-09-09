/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2026 Red Hat, Inc. All Rights Reserved.
 *
 * Author: Coiby Xu <coxu@redhat.com>
 */

// CONFIG_HAVE_ARCH_GET_SECUREBOOT selects the architecture-provided
// implementation; otherwise the inline fallback returns false.

#[cfg(CONFIG_HAVE_ARCH_GET_SECUREBOOT)]
extern "C" {
    /*
     * Returns true if the platform secure boot is enabled.
     * Returns false if disabled or not supported.
     */
    pub fn arch_get_secureboot() -> bool;
}

#[cfg(not(CONFIG_HAVE_ARCH_GET_SECUREBOOT))]
#[inline]
pub fn arch_get_secureboot() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
