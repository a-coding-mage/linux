// SPDX-License-Identifier: GPL-2.0-only
//
// Utility module for cs_dsp KUnit testing.
//
// Copyright (C) 2024 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

// The C source includes <linux/module.h>, which supplies the module metadata
// macros below. Rust has no direct file-local equivalent for those kernel
// registration macros, so preserve their metadata as constants.
pub const MODULE_DESCRIPTION: &str = "Utilities for Cirrus Logic DSP driver testing";
pub const MODULE_AUTHOR: &str = "Richard Fitzgerald <rf@opensource.cirrus.com>";
pub const MODULE_LICENSE: &str = "GPL";

// MODULE_IMPORT_NS("FW_CS_DSP");
// Kernel namespace import retained as a source-level dependency intent.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
