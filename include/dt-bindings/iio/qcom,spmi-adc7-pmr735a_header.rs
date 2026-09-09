/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2020 The Linux Foundation. All rights reserved.
 */

// Dependency: constants from <dt-bindings/iio/qcom,spmi-vadc.h> are supplied
// externally.

pub const PMR735A_SID: u32 = 4;

/* ADC channels for PMR735A_ADC for PMIC7 */
pub const PMR735A_ADC7_REF_GND: u32 = (PMR735A_SID << 8) | ADC7_REF_GND;
pub const PMR735A_ADC7_1P25VREF: u32 = (PMR735A_SID << 8) | ADC7_1P25VREF;
pub const PMR735A_ADC7_VREF_VADC: u32 = (PMR735A_SID << 8) | ADC7_VREF_VADC;
pub const PMR735A_ADC7_DIE_TEMP: u32 = (PMR735A_SID << 8) | ADC7_DIE_TEMP;

pub const PMR735A_ADC7_GPIO1: u32 = (PMR735A_SID << 8) | ADC7_GPIO1;
pub const PMR735A_ADC7_GPIO2: u32 = (PMR735A_SID << 8) | ADC7_GPIO2;
pub const PMR735A_ADC7_GPIO3: u32 = (PMR735A_SID << 8) | ADC7_GPIO3;

/* 100k pull-up2 */
pub const PMR735A_ADC7_GPIO1_100K_PU: u32 = (PMR735A_SID << 8) | ADC7_GPIO1_100K_PU;
pub const PMR735A_ADC7_GPIO2_100K_PU: u32 = (PMR735A_SID << 8) | ADC7_GPIO2_100K_PU;
pub const PMR735A_ADC7_GPIO3_100K_PU: u32 = (PMR735A_SID << 8) | ADC7_GPIO3_100K_PU;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
