/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Platform data for ST STA350 ASoC codec driver.
 *
 * Copyright: 2014 Raumfeld GmbH
 * Author: Sven Brandau <info@brandau.biz>
 */

pub const STA350_OCFG_2CH: i32 = 0;
pub const STA350_OCFG_2_1CH: i32 = 1;
pub const STA350_OCFG_1CH: i32 = 3;

pub const STA350_OM_CH1: i32 = 0;
pub const STA350_OM_CH2: i32 = 1;
pub const STA350_OM_CH3: i32 = 2;

pub const STA350_THERMAL_ADJUSTMENT_ENABLE: i32 = 1;
pub const STA350_THERMAL_RECOVERY_ENABLE: i32 = 2;
pub const STA350_FAULT_DETECT_RECOVERY_BYPASS: i32 = 1;

pub const STA350_FFX_PM_DROP_COMP: i32 = 0;
pub const STA350_FFX_PM_TAPERED_COMP: i32 = 1;
pub const STA350_FFX_PM_FULL_POWER: i32 = 2;
pub const STA350_FFX_PM_VARIABLE_DROP_COMP: i32 = 3;

#[repr(C)]
pub struct sta350_platform_data {
    pub output_conf: u8,
    pub ch1_output_mapping: u8,
    pub ch2_output_mapping: u8,
    pub ch3_output_mapping: u8,
    pub ffx_power_output_mode: u8,
    pub drop_compensation_ns: u8,
    pub powerdown_delay_divider: u8,
    /* C unsigned-int bit-fields; each field occupies one bit in the source. */
    pub thermal_warning_recovery: u32,
    pub thermal_warning_adjustment: u32,
    pub fault_detect_recovery: u32,
    pub oc_warning_adjustment: u32,
    pub max_power_use_mpcc: u32,
    pub max_power_correction: u32,
    pub am_reduction_mode: u32,
    pub odd_pwm_speed_mode: u32,
    pub distortion_compensation: u32,
    pub invalid_input_detect_mute: u32,
    pub activate_mute_output: u32,
    pub bridge_immediate_off: u32,
    pub noise_shape_dc_cut: u32,
    pub powerdown_master_vol: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
