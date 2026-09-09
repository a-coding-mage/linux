/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2020 The Linux Foundation. All rights reserved.
 */

// The C header guard is omitted in Rust; item names provide the equivalent
// single-definition behavior.

// In C, PMR735B_SID is defined only when it has not already been supplied by
// the including translation unit. Rust has no file-local conditional macro
// definition, so the header's default value is represented here directly.
pub const PMR735B_SID: u32 = 5;

// Dependency supplied by <dt-bindings/iio/qcom,spmi-vadc.h>.

/* ADC channels for PMR735B_ADC for PMIC7 */
pub const PMR735B_ADC7_REF_GND: u32 = (PMR735B_SID << 8) | ADC7_REF_GND;
pub const PMR735B_ADC7_1P25VREF: u32 = (PMR735B_SID << 8) | ADC7_1P25VREF;
pub const PMR735B_ADC7_VREF_VADC: u32 = (PMR735B_SID << 8) | ADC7_VREF_VADC;
pub const PMR735B_ADC7_DIE_TEMP: u32 = (PMR735B_SID << 8) | ADC7_DIE_TEMP;

pub const PMR735B_ADC7_GPIO1: u32 = (PMR735B_SID << 8) | ADC7_GPIO1;
pub const PMR735B_ADC7_GPIO2: u32 = (PMR735B_SID << 8) | ADC7_GPIO2;
pub const PMR735B_ADC7_GPIO3: u32 = (PMR735B_SID << 8) | ADC7_GPIO3;

/* 100k pull-up2 */
pub const PMR735B_ADC7_GPIO1_100K_PU: u32 = (PMR735B_SID << 8) | ADC7_GPIO1_100K_PU;
pub const PMR735B_ADC7_GPIO2_100K_PU: u32 = (PMR735B_SID << 8) | ADC7_GPIO2_100K_PU;
pub const PMR735B_ADC7_GPIO3_100K_PU: u32 = (PMR735B_SID << 8) | ADC7_GPIO3_100K_PU;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
