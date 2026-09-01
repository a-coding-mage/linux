/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: #include "oxygen.h"

pub const GPIO_MAGIC: u32 = 0x0008;
pub const GPIO_HP_DETECT: u32 = 0x0010;
pub const GPIO_INPUT_ROUTE: u32 = 0x0060;
pub const GPIO_HP_REAR: u32 = 0x0080;
pub const GPIO_OUTPUT_ENABLE: u32 = 0x0100;

pub const CAPTURE_SRC_MIC: u32 = 0;
pub const CAPTURE_SRC_FP_MIC: u32 = 1;
pub const CAPTURE_SRC_LINE: u32 = 2;
pub const CAPTURE_SRC_AUX: u32 = 3;

pub const PLAYBACK_DST_HP: u32 = 0;
pub const PLAYBACK_DST_HP_FP: u32 = 1;
pub const PLAYBACK_DST_MULTICH: u32 = 2;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cs4245_shadow_operation {
    CS4245_SAVE_TO_SHADOW,
    CS4245_LOAD_FROM_SHADOW,
}

#[repr(C)]
pub struct dg {
    /* shadow copy of the CS4245 register space */
    pub cs4245_shadow: [u8; 17],
    /* output select: headphone/speakers */
    pub output_sel: u8,
    /* volumes for all capture sources */
    pub input_vol: [[i8; 2]; 4],
    /* input select: mic/fp mic/line/aux */
    pub input_sel: u8,
}

unsafe extern "C" {
    /* Xonar DG control routines */
    pub fn cs4245_write_spi(chip: *mut oxygen, reg: u8) -> core::ffi::c_int;
    pub fn cs4245_read_spi(chip: *mut oxygen, reg: u8) -> core::ffi::c_int;
    pub fn cs4245_shadow_control(
        chip: *mut oxygen,
        op: cs4245_shadow_operation,
    ) -> core::ffi::c_int;
    pub fn dg_init(chip: *mut oxygen);
    pub fn set_cs4245_dac_params(chip: *mut oxygen, params: *mut snd_pcm_hw_params);
    pub fn set_cs4245_adc_params(chip: *mut oxygen, params: *mut snd_pcm_hw_params);
    pub fn adjust_dg_dac_routing(
        chip: *mut oxygen,
        play_routing: core::ffi::c_uint,
    ) -> core::ffi::c_uint;
    pub fn dump_cs4245_registers(chip: *mut oxygen, buffer: *mut snd_info_buffer);
    pub fn dg_suspend(chip: *mut oxygen);
    pub fn dg_resume(chip: *mut oxygen);
    pub fn dg_cleanup(chip: *mut oxygen);

    pub static model_xonar_dg: oxygen_model;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
