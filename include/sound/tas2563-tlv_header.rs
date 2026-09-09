/* SPDX-License-Identifier: GPL-2.0 */
//
// ALSA SoC Texas Instruments TAS2563 Audio Smart Amplifier
//
// Copyright (C) 2022 - 2024 Texas Instruments Incorporated
// https://www.ti.com
//
// Rust translation of tas2563-tlv.h.

/// Equivalent of `DECLARE_TLV_DB_SCALE(tas2563_dvc_tlv, -12150, 50, 1)`.
/// The concrete ALSA TLV representation is supplied by the consuming driver.
#[allow(non_upper_case_globals)]
pub const tas2563_dvc_tlv: (i32, i32, i32) = (-12150, 50, 1);

/* pow(10, db/20) * pow(2,30) */
#[allow(non_upper_case_globals)]
pub static tas2563_dvc_table: [[u8; 4]; 253] = [[0u8; 4]; 253];


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
