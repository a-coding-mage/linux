/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides constants for configuring the I.MX25 ADC
 */

pub const MX25_ADC_REFP_YP: i32 = 0; // YP voltage reference
pub const MX25_ADC_REFP_XP: i32 = 1; // XP voltage reference
pub const MX25_ADC_REFP_EXT: i32 = 2; // External voltage reference
pub const MX25_ADC_REFP_INT: i32 = 3; // Internal voltage reference

pub const MX25_ADC_REFN_XN: i32 = 0; // XN ground reference
pub const MX25_ADC_REFN_YN: i32 = 1; // YN ground reference
pub const MX25_ADC_REFN_NGND: i32 = 2; // Internal ground reference
pub const MX25_ADC_REFN_NGND2: i32 = 3; // External ground reference

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
