/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * cs_dsp.h  --  Private header for cs_dsp driver.
 *
 * Copyright (C) 2026 Cirrus Logic, Inc. and
 *                    Cirrus Logic International Semiconductor Ltd.
 */

// C header guard: FW_CS_DSP_H

// Equivalent to the C condition: IS_ENABLED(CONFIG_KUNIT).
#[cfg(feature = "kunit")]
unsafe extern "C" {
    pub fn cs_dsp_can_emit_message() -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
