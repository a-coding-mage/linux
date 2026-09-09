/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2020, The Linux Foundation. All rights reserved.
 */

// Dependency supplied by dt-bindings/iio/qcom,spmi-vadc.h.

/* ADC channels for PM8350_ADC for PMIC7 */
macro_rules! PM8350_ADC7_REF_GND { ($sid:expr) => { (($sid) << 8 | ADC7_REF_GND) }; }
macro_rules! PM8350_ADC7_1P25VREF { ($sid:expr) => { (($sid) << 8 | ADC7_1P25VREF) }; }
macro_rules! PM8350_ADC7_VREF_VADC { ($sid:expr) => { (($sid) << 8 | ADC7_VREF_VADC) }; }
macro_rules! PM8350_ADC7_DIE_TEMP { ($sid:expr) => { (($sid) << 8 | ADC7_DIE_TEMP) }; }

macro_rules! PM8350_ADC7_AMUX_THM1 { ($sid:expr) => { (($sid) << 8 | ADC7_AMUX_THM1) }; }
macro_rules! PM8350_ADC7_AMUX_THM2 { ($sid:expr) => { (($sid) << 8 | ADC7_AMUX_THM2) }; }
macro_rules! PM8350_ADC7_AMUX_THM3 { ($sid:expr) => { (($sid) << 8 | ADC7_AMUX_THM3) }; }
macro_rules! PM8350_ADC7_AMUX_THM4 { ($sid:expr) => { (($sid) << 8 | ADC7_AMUX_THM4) }; }
macro_rules! PM8350_ADC7_AMUX_THM5 { ($sid:expr) => { (($sid) << 8 | ADC7_AMUX_THM5) }; }
macro_rules! PM8350_ADC7_GPIO1 { ($sid:expr) => { (($sid) << 8 | ADC7_GPIO1) }; }
macro_rules! PM8350_ADC7_GPIO2 { ($sid:expr) => { (($sid) << 8 | ADC7_GPIO2) }; }
macro_rules! PM8350_ADC7_GPIO3 { ($sid:expr) => { (($sid) << 8 | ADC7_GPIO3) }; }
macro_rules! PM8350_ADC7_GPIO4 { ($sid:expr) => { (($sid) << 8 | ADC7_GPIO4) }; }

/* 30k pull-up1 */
macro_rules! PM8350_ADC7_AMUX_THM1_30K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_AMUX_THM1_30K_PU) }; }
macro_rules! PM8350_ADC7_AMUX_THM2_30K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_AMUX_THM2_30K_PU) }; }
macro_rules! PM8350_ADC7_AMUX_THM3_30K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_AMUX_THM3_30K_PU) }; }
macro_rules! PM8350_ADC7_AMUX_THM4_30K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_AMUX_THM4_30K_PU) }; }
macro_rules! PM8350_ADC7_AMUX_THM5_30K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_AMUX_THM5_30K_PU) }; }
macro_rules! PM8350_ADC7_GPIO1_30K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_GPIO1_30K_PU) }; }
macro_rules! PM8350_ADC7_GPIO2_30K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_GPIO2_30K_PU) }; }
macro_rules! PM8350_ADC7_GPIO3_30K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_GPIO3_30K_PU) }; }
macro_rules! PM8350_ADC7_GPIO4_30K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_GPIO4_30K_PU) }; }

/* 100k pull-up2 */
macro_rules! PM8350_ADC7_AMUX_THM1_100K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_AMUX_THM1_100K_PU) }; }
macro_rules! PM8350_ADC7_AMUX_THM2_100K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_AMUX_THM2_100K_PU) }; }
macro_rules! PM8350_ADC7_AMUX_THM3_100K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_AMUX_THM3_100K_PU) }; }
macro_rules! PM8350_ADC7_AMUX_THM4_100K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_AMUX_THM4_100K_PU) }; }
macro_rules! PM8350_ADC7_AMUX_THM5_100K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_AMUX_THM5_100K_PU) }; }
macro_rules! PM8350_ADC7_GPIO1_100K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_GPIO1_100K_PU) }; }
macro_rules! PM8350_ADC7_GPIO2_100K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_GPIO2_100K_PU) }; }
macro_rules! PM8350_ADC7_GPIO3_100K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_GPIO3_100K_PU) }; }
macro_rules! PM8350_ADC7_GPIO4_100K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_GPIO4_100K_PU) }; }

/* 400k pull-up3 */
macro_rules! PM8350_ADC7_AMUX_THM1_400K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_AMUX_THM1_400K_PU) }; }
macro_rules! PM8350_ADC7_AMUX_THM2_400K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_AMUX_THM2_400K_PU) }; }
macro_rules! PM8350_ADC7_AMUX_THM3_400K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_AMUX_THM3_400K_PU) }; }
macro_rules! PM8350_ADC7_AMUX_THM4_400K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_AMUX_THM4_400K_PU) }; }
macro_rules! PM8350_ADC7_AMUX_THM5_400K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_AMUX_THM5_400K_PU) }; }
macro_rules! PM8350_ADC7_GPIO1_400K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_GPIO1_400K_PU) }; }
macro_rules! PM8350_ADC7_GPIO2_400K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_GPIO2_400K_PU) }; }
macro_rules! PM8350_ADC7_GPIO3_400K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_GPIO3_400K_PU) }; }
macro_rules! PM8350_ADC7_GPIO4_400K_PU { ($sid:expr) => { (($sid) << 8 | ADC7_GPIO4_400K_PU) }; }

/* 1/3 Divider */
macro_rules! PM8350_ADC7_GPIO4_DIV3 { ($sid:expr) => { (($sid) << 8 | ADC7_GPIO4_DIV3) }; }

macro_rules! PM8350_ADC7_VPH_PWR { ($sid:expr) => { (($sid) << 8 | ADC7_VPH_PWR) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
