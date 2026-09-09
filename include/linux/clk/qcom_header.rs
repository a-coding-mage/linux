/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2026, Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

//! Rust translation of the Qualcomm clock reference header.
//!
//! The C header includes Linux clock-provider, errno, kconfig, regmap, and
//! types definitions.  Those dependencies are supplied externally.

use core::ffi::c_char;

// Opaque types supplied by the included Linux headers.
pub struct clk_hw;
pub struct regmap;
pub struct regmap_config;
pub struct regulator_bulk_data;
pub struct platform_device;

/// Descriptor for a clkref_en gate clock.
#[repr(C)]
pub struct qcom_clk_ref_desc {
    /// Clock name exposed to the common clock framework.
    pub name: *const c_char,
    /// clkref_en register offset from the block base.
    pub offset: u32,
    /// Optional supply names enabled while preparing the clock.
    pub regulator_names: *const *const c_char,
    /// Number of entries in `regulator_names`.
    pub num_regulators: u32,
}

/// Per-clock data for a clkref_en gate clock.
#[repr(C)]
pub struct qcom_clk_ref {
    /// Common clock framework hardware clock handle.
    pub hw: clk_hw,
    /// Register map backing the clkref_en register.
    pub regmap: *mut regmap,
    /// Clock descriptor copied at registration time.
    pub desc: qcom_clk_ref_desc,
    /// Optional bulk regulator handles for `desc.regulator_names`.
    pub regulators: *mut regulator_bulk_data,
}

// When CONFIG_COMMON_CLK_QCOM is enabled, this declaration is provided by
// the Qualcomm common-clock implementation.
#[cfg(feature = "CONFIG_COMMON_CLK_QCOM")]
extern "C" {
    pub fn qcom_clk_ref_probe(
        pdev: *mut platform_device,
        config: *const regmap_config,
        descs: *const *const qcom_clk_ref_desc,
        num_clk_refs: usize,
    ) -> i32;
}

// Equivalent of the C fallback selected when CONFIG_COMMON_CLK_QCOM is
// disabled.  Linux errno EOPNOTSUPP is 95.
#[cfg(not(feature = "CONFIG_COMMON_CLK_QCOM"))]
#[inline]
pub unsafe fn qcom_clk_ref_probe(
    _pdev: *mut platform_device,
    _config: *const regmap_config,
    _descs: *const *const qcom_clk_ref_desc,
    _num_clk_refs: usize,
) -> i32 {
    -95
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
