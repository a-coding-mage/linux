/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
 */

// Source dependency: <linux/sizes.h>

/* The first 1K of IRAM is permanently reserved for the CPU reset handler */
pub const TEGRA_IRAM_RESET_HANDLER_OFFSET: usize = 0;
pub const TEGRA_IRAM_RESET_HANDLER_SIZE: usize = 1 * 1024;

/*
 * This area is used for LPx resume vector, only while LPx power state is
 * active. At other times, the AVP may use this area for arbitrary purposes
 */
// TEGRA_IRAM_BASE is supplied by the surrounding translation unit.
pub const TEGRA_IRAM_LPx_RESUME_AREA: usize = TEGRA_IRAM_BASE + 4 * 1024;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
