/* SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause) */
/* Copyright 2019 NXP */

// C dependencies:
// #include <linux/dcache.h>
// #include "caamalg_qi2.h"

/// Opaque declaration supplied by `caamalg_qi2`.
#[repr(C)]
pub struct dpaa2_caam_priv {
    _private: [u8; 0],
}

// CONFIG_DEBUG_FS conditional declarations.
#[cfg(feature = "CONFIG_DEBUG_FS")]
extern "C" {
    pub fn dpaa2_dpseci_debugfs_init(priv_: *mut dpaa2_caam_priv);
    pub fn dpaa2_dpseci_debugfs_exit(priv_: *mut dpaa2_caam_priv);
}

// CONFIG_DEBUG_FS disabled: static inline no-op definitions.
#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub fn dpaa2_dpseci_debugfs_init(_priv: *mut dpaa2_caam_priv) {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub fn dpaa2_dpseci_debugfs_exit(_priv: *mut dpaa2_caam_priv) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
