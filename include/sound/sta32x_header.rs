/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Platform data for ST STA32x ASoC codec driver.
 *
 * Copyright: 2011 Raumfeld GmbH
 * Author: Johannes Stezenbach <js@sig21.net>
 */

pub const STA32X_OCFG_2CH: i32 = 0;
pub const STA32X_OCFG_2_1CH: i32 = 1;
pub const STA32X_OCFG_1CH: i32 = 3;

pub const STA32X_OM_CH1: i32 = 0;
pub const STA32X_OM_CH2: i32 = 1;
pub const STA32X_OM_CH3: i32 = 2;

pub const STA32X_THERMAL_ADJUSTMENT_ENABLE: i32 = 1;
pub const STA32X_THERMAL_RECOVERY_ENABLE: i32 = 2;

#[repr(C)]
pub struct sta32x_platform_data {
    pub output_conf: u8,
    pub ch1_output_mapping: u8,
    pub ch2_output_mapping: u8,
    pub ch3_output_mapping: u8,
    pub needs_esd_watchdog: i32,
    pub drop_compensation_ns: u8,
    /* C unsigned-int bit-fields; represented individually as their underlying integer type. */
    pub thermal_warning_recovery: u32,
    pub thermal_warning_adjustment: u32,
    pub fault_detect_recovery: u32,
    pub max_power_use_mpcc: u32,
    pub max_power_correction: u32,
    pub am_reduction_mode: u32,
    pub odd_pwm_speed_mode: u32,
    pub invalid_input_detect_mute: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
