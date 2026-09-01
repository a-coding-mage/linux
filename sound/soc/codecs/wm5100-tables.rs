// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm5100-tables.rs  --  WM5100 ALSA SoC Audio driver data
 *
 * Copyright 2011-2 Wolfson Microelectronics plc
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

// Translated from wm5100-tables.c; depends on declarations from wm5100.h.
use core::ffi::c_uint;

pub unsafe extern "C" fn wm5100_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        WM5100_SOFTWARE_RESET => true,
        WM5100_DEVICE_REVISION => true,
        WM5100_FX_CTRL => true,
        WM5100_INTERRUPT_STATUS_1 => true,
        WM5100_INTERRUPT_STATUS_2 => true,
        WM5100_INTERRUPT_STATUS_3 => true,
        WM5100_INTERRUPT_STATUS_4 => true,
        WM5100_INTERRUPT_RAW_STATUS_2 => true,
        WM5100_INTERRUPT_RAW_STATUS_3 => true,
        WM5100_INTERRUPT_RAW_STATUS_4 => true,
        WM5100_OUTPUT_STATUS_1 => true,
        WM5100_OUTPUT_STATUS_2 => true,
        WM5100_INPUT_ENABLES_STATUS => true,
        WM5100_MIC_DETECT_3 => true,
        _ => {
            (reg >= WM5100_DSP1_PM_0 && reg <= WM5100_DSP1_PM_1535) ||
            (reg >= WM5100_DSP1_ZM_0 && reg <= WM5100_DSP1_ZM_2047) ||
            (reg >= WM5100_DSP1_DM_0 && reg <= WM5100_DSP1_DM_511) ||
            (reg >= WM5100_DSP2_PM_0 && reg <= WM5100_DSP2_PM_1535) ||
            (reg >= WM5100_DSP2_ZM_0 && reg <= WM5100_DSP2_ZM_2047) ||
            (reg >= WM5100_DSP2_DM_0 && reg <= WM5100_DSP2_DM_511) ||
            (reg >= WM5100_DSP3_PM_0 && reg <= WM5100_DSP3_PM_1535) ||
            (reg >= WM5100_DSP3_ZM_0 && reg <= WM5100_DSP3_ZM_2047) ||
            (reg >= WM5100_DSP3_DM_0 && reg <= WM5100_DSP3_DM_511)
        }
    }
}

pub unsafe extern "C" fn wm5100_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        WM5100_SOFTWARE_RESET => true,
        WM5100_DEVICE_REVISION => true,
        WM5100_CTRL_IF_1 => true,
        WM5100_TONE_GENERATOR_1 => true,
        WM5100_PWM_DRIVE_1 => true,
        WM5100_PWM_DRIVE_2 => true,
        WM5100_PWM_DRIVE_3 => true,
        WM5100_CLOCKING_1 => true,
        WM5100_CLOCKING_3 => true,
        WM5100_CLOCKING_4 => true,
        WM5100_CLOCKING_5 => true,
        WM5100_CLOCKING_6 => true,
        WM5100_CLOCKING_7 => true,
        WM5100_CLOCKING_8 => true,
        WM5100_ASRC_ENABLE => true,
        WM5100_ASRC_STATUS => true,
        WM5100_ASRC_RATE1 => true,
        WM5100_ISRC_1_CTRL_1 => true,
        WM5100_ISRC_1_CTRL_2 => true,
        WM5100_ISRC_2_CTRL1 => true,
        WM5100_ISRC_2_CTRL_2 => true,
        WM5100_FLL1_CONTROL_1 => true,
        WM5100_FLL1_CONTROL_2 => true,
        WM5100_FLL1_CONTROL_3 => true,
        WM5100_FLL1_CONTROL_5 => true,
        WM5100_FLL1_CONTROL_6 => true,
        WM5100_FLL1_EFS_1 => true,
        WM5100_FLL2_CONTROL_1 => true,
        WM5100_FLL2_CONTROL_2 => true,
        WM5100_FLL2_CONTROL_3 => true,
        WM5100_FLL2_CONTROL_5 => true,
        WM5100_FLL2_CONTROL_6 => true,
        WM5100_FLL2_EFS_1 => true,
        WM5100_MIC_CHARGE_PUMP_1 => true,
        WM5100_MIC_CHARGE_PUMP_2 => true,
        WM5100_HP_CHARGE_PUMP_1 => true,
        WM5100_LDO1_CONTROL => true,
        WM5100_MIC_BIAS_CTRL_1 => true,
        WM5100_MIC_BIAS_CTRL_2 => true,
        WM5100_MIC_BIAS_CTRL_3 => true,
        WM5100_ACCESSORY_DETECT_MODE_1 => true,
        WM5100_HEADPHONE_DETECT_1 => true,
        WM5100_HEADPHONE_DETECT_2 => true,
        WM5100_MIC_DETECT_1 => true,
        WM5100_MIC_DETECT_2 => true,
        WM5100_MIC_DETECT_3 => true,
        WM5100_MISC_CONTROL => true,
        WM5100_INPUT_ENABLES => true,
        WM5100_INPUT_ENABLES_STATUS => true,
        WM5100_IN1L_CONTROL => true,
        WM5100_IN1R_CONTROL => true,
        WM5100_IN2L_CONTROL => true,
        WM5100_IN2R_CONTROL => true,
        WM5100_IN3L_CONTROL => true,
        WM5100_IN3R_CONTROL => true,
        WM5100_IN4L_CONTROL => true,
        WM5100_IN4R_CONTROL => true,
        WM5100_RXANC_SRC => true,
        WM5100_INPUT_VOLUME_RAMP => true,
        WM5100_ADC_DIGITAL_VOLUME_1L => true,
        WM5100_ADC_DIGITAL_VOLUME_1R => true,
        WM5100_ADC_DIGITAL_VOLUME_2L => true,
        WM5100_ADC_DIGITAL_VOLUME_2R => true,
        WM5100_ADC_DIGITAL_VOLUME_3L => true,
        WM5100_ADC_DIGITAL_VOLUME_3R => true,
        WM5100_ADC_DIGITAL_VOLUME_4L => true,
        WM5100_ADC_DIGITAL_VOLUME_4R => true,
        WM5100_OUTPUT_ENABLES_2 => true,
        WM5100_OUTPUT_STATUS_1 => true,
        WM5100_OUTPUT_STATUS_2 => true,
        WM5100_CHANNEL_ENABLES_1 => true,
        WM5100_OUT_VOLUME_1L => true,
        WM5100_OUT_VOLUME_1R => true,
        WM5100_DAC_VOLUME_LIMIT_1L => true,
        WM5100_DAC_VOLUME_LIMIT_1R => true,
        WM5100_OUT_VOLUME_2L => true,
        WM5100_OUT_VOLUME_2R => true,
        WM5100_DAC_VOLUME_LIMIT_2L => true,
        WM5100_DAC_VOLUME_LIMIT_2R => true,
        WM5100_OUT_VOLUME_3L => true,
        WM5100_OUT_VOLUME_3R => true,
        WM5100_DAC_VOLUME_LIMIT_3L => true,
        WM5100_DAC_VOLUME_LIMIT_3R => true,
        WM5100_OUT_VOLUME_4L => true,
        WM5100_OUT_VOLUME_4R => true,
        WM5100_DAC_VOLUME_LIMIT_5L => true,
        WM5100_DAC_VOLUME_LIMIT_5R => true,
        WM5100_DAC_VOLUME_LIMIT_6L => true,
        WM5100_DAC_VOLUME_LIMIT_6R => true,
        WM5100_DAC_AEC_CONTROL_1 => true,
        WM5100_OUTPUT_VOLUME_RAMP => true,
        WM5100_DAC_DIGITAL_VOLUME_1L => true,
        WM5100_DAC_DIGITAL_VOLUME_1R => true,
        WM5100_DAC_DIGITAL_VOLUME_2L => true,
        WM5100_DAC_DIGITAL_VOLUME_2R => true,
        WM5100_DAC_DIGITAL_VOLUME_3L => true,
        WM5100_DAC_DIGITAL_VOLUME_3R => true,
        WM5100_DAC_DIGITAL_VOLUME_4L => true,
        WM5100_DAC_DIGITAL_VOLUME_4R => true,
        WM5100_DAC_DIGITAL_VOLUME_5L => true,
        WM5100_DAC_DIGITAL_VOLUME_5R => true,
        WM5100_DAC_DIGITAL_VOLUME_6L => true,
        WM5100_DAC_DIGITAL_VOLUME_6R => true,
        WM5100_PDM_SPK1_CTRL_1 => true,
        WM5100_PDM_SPK1_CTRL_2 => true,
        WM5100_PDM_SPK2_CTRL_1 => true,
        WM5100_PDM_SPK2_CTRL_2 => true,
        WM5100_AUDIO_IF_1_1 => true,
        WM5100_AUDIO_IF_1_2 => true,
        WM5100_AUDIO_IF_1_3 => true,
        WM5100_AUDIO_IF_1_4 => true,
        WM5100_AUDIO_IF_1_5 => true,
        WM5100_AUDIO_IF_1_6 => true,
        WM5100_AUDIO_IF_1_7 => true,
        WM5100_AUDIO_IF_1_8 => true,
        WM5100_AUDIO_IF_1_9 => true,
        WM5100_AUDIO_IF_1_10 => true,
        WM5100_AUDIO_IF_1_11 => true,
        WM5100_AUDIO_IF_1_12 => true,
        WM5100_AUDIO_IF_1_13 => true,
        WM5100_AUDIO_IF_1_14 => true,
        WM5100_AUDIO_IF_1_15 => true,
        WM5100_AUDIO_IF_1_16 => true,
        WM5100_AUDIO_IF_1_17 => true,
        WM5100_AUDIO_IF_1_18 => true,
        WM5100_AUDIO_IF_1_19 => true,
        WM5100_AUDIO_IF_1_20 => true,
        WM5100_AUDIO_IF_1_21 => true,
        WM5100_AUDIO_IF_1_22 => true,
        WM5100_AUDIO_IF_1_23 => true,
        WM5100_AUDIO_IF_1_24 => true,
        WM5100_AUDIO_IF_1_25 => true,
        WM5100_AUDIO_IF_1_26 => true,
        WM5100_AUDIO_IF_1_27 => true,
        WM5100_AUDIO_IF_2_1 => true,
        WM5100_AUDIO_IF_2_2 => true,
        WM5100_AUDIO_IF_2_3 => true,
        WM5100_AUDIO_IF_2_4 => true,
        WM5100_AUDIO_IF_2_5 => true,
        WM5100_AUDIO_IF_2_6 => true,
        WM5100_AUDIO_IF_2_7 => true,
        WM5100_AUDIO_IF_2_8 => true,
        WM5100_AUDIO_IF_2_9 => true,
        WM5100_AUDIO_IF_2_10 => true,
        WM5100_AUDIO_IF_2_11 => true,
        WM5100_AUDIO_IF_2_18 => true,
        WM5100_AUDIO_IF_2_19 => true,
        WM5100_AUDIO_IF_2_26 => true,
        WM5100_AUDIO_IF_2_27 => true,
        WM5100_AUDIO_IF_3_1 => true,
        WM5100_AUDIO_IF_3_2 => true,
        WM5100_AUDIO_IF_3_3 => true,
        WM5100_AUDIO_IF_3_4 => true,
        WM5100_AUDIO_IF_3_5 => true,
        WM5100_AUDIO_IF_3_6 => true,
        WM5100_AUDIO_IF_3_7 => true,
        WM5100_AUDIO_IF_3_8 => true,
        WM5100_AUDIO_IF_3_9 => true,
        WM5100_AUDIO_IF_3_10 => true,
        WM5100_AUDIO_IF_3_11 => true,
        WM5100_AUDIO_IF_3_18 => true,
        WM5100_AUDIO_IF_3_19 => true,
        WM5100_AUDIO_IF_3_26 => true,
        WM5100_AUDIO_IF_3_27 => true,
        WM5100_PWM1MIX_INPUT_1_SOURCE => true,
        WM5100_PWM1MIX_INPUT_1_VOLUME => true,
        WM5100_PWM1MIX_INPUT_2_SOURCE => true,
        WM5100_PWM1MIX_INPUT_2_VOLUME => true,
        WM5100_PWM1MIX_INPUT_3_SOURCE => true,
        WM5100_PWM1MIX_INPUT_3_VOLUME => true,
        WM5100_PWM1MIX_INPUT_4_SOURCE => true,
        WM5100_PWM1MIX_INPUT_4_VOLUME => true,
        WM5100_PWM2MIX_INPUT_1_SOURCE => true,
        WM5100_PWM2MIX_INPUT_1_VOLUME => true,
        WM5100_PWM2MIX_INPUT_2_SOURCE => true,
        WM5100_PWM2MIX_INPUT_2_VOLUME => true,
        WM5100_PWM2MIX_INPUT_3_SOURCE => true,
        WM5100_PWM2MIX_INPUT_3_VOLUME => true,
        WM5100_PWM2MIX_INPUT_4_SOURCE => true,
        WM5100_PWM2MIX_INPUT_4_VOLUME => true,
        WM5100_OUT1LMIX_INPUT_1_SOURCE => true,
        WM5100_OUT1LMIX_INPUT_1_VOLUME => true,
        WM5100_OUT1LMIX_INPUT_2_SOURCE => true,
        WM5100_OUT1LMIX_INPUT_2_VOLUME => true,
        WM5100_OUT1LMIX_INPUT_3_SOURCE => true,
        WM5100_OUT1LMIX_INPUT_3_VOLUME => true,
        WM5100_OUT1LMIX_INPUT_4_SOURCE => true,
        WM5100_OUT1LMIX_INPUT_4_VOLUME => true,
        WM5100_OUT1RMIX_INPUT_1_SOURCE => true,
        WM5100_OUT1RMIX_INPUT_1_VOLUME => true,
        WM5100_OUT1RMIX_INPUT_2_SOURCE => true,
        WM5100_OUT1RMIX_INPUT_2_VOLUME => true,
        WM5100_OUT1RMIX_INPUT_3_SOURCE => true,
        WM5100_OUT1RMIX_INPUT_3_VOLUME => true,
        WM5100_OUT1RMIX_INPUT_4_SOURCE => true,
        WM5100_OUT1RMIX_INPUT_4_VOLUME => true,
        WM5100_OUT2LMIX_INPUT_1_SOURCE => true,
        WM5100_OUT2LMIX_INPUT_1_VOLUME => true,
        WM5100_OUT2LMIX_INPUT_2_SOURCE => true,
        WM5100_OUT2LMIX_INPUT_2_VOLUME => true,
        WM5100_OUT2LMIX_INPUT_3_SOURCE => true,
        WM5100_OUT2LMIX_INPUT_3_VOLUME => true,
        WM5100_OUT2LMIX_INPUT_4_SOURCE => true,
        WM5100_OUT2LMIX_INPUT_4_VOLUME => true,
        WM5100_OUT2RMIX_INPUT_1_SOURCE => true,
        WM5100_OUT2RMIX_INPUT_1_VOLUME => true,
        WM5100_OUT2RMIX_INPUT_2_SOURCE => true,
        WM5100_OUT2RMIX_INPUT_2_VOLUME => true,
        WM5100_OUT2RMIX_INPUT_3_SOURCE => true,
        WM5100_OUT2RMIX_INPUT_3_VOLUME => true,
        WM5100_OUT2RMIX_INPUT_4_SOURCE => true,
        WM5100_OUT2RMIX_INPUT_4_VOLUME => true,
        WM5100_OUT3LMIX_INPUT_1_SOURCE => true,
        WM5100_OUT3LMIX_INPUT_1_VOLUME => true,
        WM5100_OUT3LMIX_INPUT_2_SOURCE => true,
        WM5100_OUT3LMIX_INPUT_2_VOLUME => true,
        WM5100_OUT3LMIX_INPUT_3_SOURCE => true,
        WM5100_OUT3LMIX_INPUT_3_VOLUME => true,
        WM5100_OUT3LMIX_INPUT_4_SOURCE => true,
        WM5100_OUT3LMIX_INPUT_4_VOLUME => true,
        WM5100_OUT3RMIX_INPUT_1_SOURCE => true,
        WM5100_OUT3RMIX_INPUT_1_VOLUME => true,
        WM5100_OUT3RMIX_INPUT_2_SOURCE => true,
        WM5100_OUT3RMIX_INPUT_2_VOLUME => true,
        WM5100_OUT3RMIX_INPUT_3_SOURCE => true,
        WM5100_OUT3RMIX_INPUT_3_VOLUME => true,
        WM5100_OUT3RMIX_INPUT_4_SOURCE => true,
        WM5100_OUT3RMIX_INPUT_4_VOLUME => true,
        WM5100_OUT4LMIX_INPUT_1_SOURCE => true,
        WM5100_OUT4LMIX_INPUT_1_VOLUME => true,
        WM5100_OUT4LMIX_INPUT_2_SOURCE => true,
        WM5100_OUT4LMIX_INPUT_2_VOLUME => true,
        WM5100_OUT4LMIX_INPUT_3_SOURCE => true,
        WM5100_OUT4LMIX_INPUT_3_VOLUME => true,
        WM5100_OUT4LMIX_INPUT_4_SOURCE => true,
        WM5100_OUT4LMIX_INPUT_4_VOLUME => true,
        WM5100_OUT4RMIX_INPUT_1_SOURCE => true,
        WM5100_OUT4RMIX_INPUT_1_VOLUME => true,
        WM5100_OUT4RMIX_INPUT_2_SOURCE => true,
        WM5100_OUT4RMIX_INPUT_2_VOLUME => true,
        WM5100_OUT4RMIX_INPUT_3_SOURCE => true,
        WM5100_OUT4RMIX_INPUT_3_VOLUME => true,
        WM5100_OUT4RMIX_INPUT_4_SOURCE => true,
        WM5100_OUT4RMIX_INPUT_4_VOLUME => true,
        WM5100_OUT5LMIX_INPUT_1_SOURCE => true,
        WM5100_OUT5LMIX_INPUT_1_VOLUME => true,
        WM5100_OUT5LMIX_INPUT_2_SOURCE => true,
        WM5100_OUT5LMIX_INPUT_2_VOLUME => true,
        WM5100_OUT5LMIX_INPUT_3_SOURCE => true,
        WM5100_OUT5LMIX_INPUT_3_VOLUME => true,
        WM5100_OUT5LMIX_INPUT_4_SOURCE => true,
        WM5100_OUT5LMIX_INPUT_4_VOLUME => true,
        WM5100_OUT5RMIX_INPUT_1_SOURCE => true,
        WM5100_OUT5RMIX_INPUT_1_VOLUME => true,
        WM5100_OUT5RMIX_INPUT_2_SOURCE => true,
        WM5100_OUT5RMIX_INPUT_2_VOLUME => true,
        WM5100_OUT5RMIX_INPUT_3_SOURCE => true,
        WM5100_OUT5RMIX_INPUT_3_VOLUME => true,
        WM5100_OUT5RMIX_INPUT_4_SOURCE => true,
        WM5100_OUT5RMIX_INPUT_4_VOLUME => true,
        WM5100_OUT6LMIX_INPUT_1_SOURCE => true,
        WM5100_OUT6LMIX_INPUT_1_VOLUME => true,
        WM5100_OUT6LMIX_INPUT_2_SOURCE => true,
        WM5100_OUT6LMIX_INPUT_2_VOLUME => true,
        WM5100_OUT6LMIX_INPUT_3_SOURCE => true,
        WM5100_OUT6LMIX_INPUT_3_VOLUME => true,
        WM5100_OUT6LMIX_INPUT_4_SOURCE => true,
        WM5100_OUT6LMIX_INPUT_4_VOLUME => true,
        WM5100_OUT6RMIX_INPUT_1_SOURCE => true,
        WM5100_OUT6RMIX_INPUT_1_VOLUME => true,
        WM5100_OUT6RMIX_INPUT_2_SOURCE => true,
        WM5100_OUT6RMIX_INPUT_2_VOLUME => true,
        WM5100_OUT6RMIX_INPUT_3_SOURCE => true,
        WM5100_OUT6RMIX_INPUT_3_VOLUME => true,
        WM5100_OUT6RMIX_INPUT_4_SOURCE => true,
        WM5100_OUT6RMIX_INPUT_4_VOLUME => true,
        WM5100_AIF1TX1MIX_INPUT_1_SOURCE => true,
        WM5100_AIF1TX1MIX_INPUT_1_VOLUME => true,
        WM5100_AIF1TX1MIX_INPUT_2_SOURCE => true,
        WM5100_AIF1TX1MIX_INPUT_2_VOLUME => true,
        WM5100_AIF1TX1MIX_INPUT_3_SOURCE => true,
        WM5100_AIF1TX1MIX_INPUT_3_VOLUME => true,
        WM5100_AIF1TX1MIX_INPUT_4_SOURCE => true,
        WM5100_AIF1TX1MIX_INPUT_4_VOLUME => true,
        WM5100_AIF1TX2MIX_INPUT_1_SOURCE => true,
        WM5100_AIF1TX2MIX_INPUT_1_VOLUME => true,
        WM5100_AIF1TX2MIX_INPUT_2_SOURCE => true,
        WM5100_AIF1TX2MIX_INPUT_2_VOLUME => true,
        WM5100_AIF1TX2MIX_INPUT_3_SOURCE => true,
        WM5100_AIF1TX2MIX_INPUT_3_VOLUME => true,
        WM5100_AIF1TX2MIX_INPUT_4_SOURCE => true,
        WM5100_AIF1TX2MIX_INPUT_4_VOLUME => true,
        WM5100_AIF1TX3MIX_INPUT_1_SOURCE => true,
        WM5100_AIF1TX3MIX_INPUT_1_VOLUME => true,
        WM5100_AIF1TX3MIX_INPUT_2_SOURCE => true,
        WM5100_AIF1TX3MIX_INPUT_2_VOLUME => true,
        WM5100_AIF1TX3MIX_INPUT_3_SOURCE => true,
        WM5100_AIF1TX3MIX_INPUT_3_VOLUME => true,
        WM5100_AIF1TX3MIX_INPUT_4_SOURCE => true,
        WM5100_AIF1TX3MIX_INPUT_4_VOLUME => true,
        WM5100_AIF1TX4MIX_INPUT_1_SOURCE => true,
        WM5100_AIF1TX4MIX_INPUT_1_VOLUME => true,
        WM5100_AIF1TX4MIX_INPUT_2_SOURCE => true,
        WM5100_AIF1TX4MIX_INPUT_2_VOLUME => true,
        WM5100_AIF1TX4MIX_INPUT_3_SOURCE => true,
        WM5100_AIF1TX4MIX_INPUT_3_VOLUME => true,
        WM5100_AIF1TX4MIX_INPUT_4_SOURCE => true,
        WM5100_AIF1TX4MIX_INPUT_4_VOLUME => true,
        WM5100_AIF1TX5MIX_INPUT_1_SOURCE => true,
        WM5100_AIF1TX5MIX_INPUT_1_VOLUME => true,
        WM5100_AIF1TX5MIX_INPUT_2_SOURCE => true,
        WM5100_AIF1TX5MIX_INPUT_2_VOLUME => true,
        WM5100_AIF1TX5MIX_INPUT_3_SOURCE => true,
        WM5100_AIF1TX5MIX_INPUT_3_VOLUME => true,
        WM5100_AIF1TX5MIX_INPUT_4_SOURCE => true,
        WM5100_AIF1TX5MIX_INPUT_4_VOLUME => true,
        WM5100_AIF1TX6MIX_INPUT_1_SOURCE => true,
        WM5100_AIF1TX6MIX_INPUT_1_VOLUME => true,
        WM5100_AIF1TX6MIX_INPUT_2_SOURCE => true,
        WM5100_AIF1TX6MIX_INPUT_2_VOLUME => true,
        WM5100_AIF1TX6MIX_INPUT_3_SOURCE => true,
        WM5100_AIF1TX6MIX_INPUT_3_VOLUME => true,
        WM5100_AIF1TX6MIX_INPUT_4_SOURCE => true,
        WM5100_AIF1TX6MIX_INPUT_4_VOLUME => true,
        WM5100_AIF1TX7MIX_INPUT_1_SOURCE => true,
        WM5100_AIF1TX7MIX_INPUT_1_VOLUME => true,
        WM5100_AIF1TX7MIX_INPUT_2_SOURCE => true,
        WM5100_AIF1TX7MIX_INPUT_2_VOLUME => true,
        WM5100_AIF1TX7MIX_INPUT_3_SOURCE => true,
        WM5100_AIF1TX7MIX_INPUT_3_VOLUME => true,
        WM5100_AIF1TX7MIX_INPUT_4_SOURCE => true,
        WM5100_AIF1TX7MIX_INPUT_4_VOLUME => true,
        WM5100_AIF1TX8MIX_INPUT_1_SOURCE => true,
        WM5100_AIF1TX8MIX_INPUT_1_VOLUME => true,
        WM5100_AIF1TX8MIX_INPUT_2_SOURCE => true,
        WM5100_AIF1TX8MIX_INPUT_2_VOLUME => true,
        WM5100_AIF1TX8MIX_INPUT_3_SOURCE => true,
        WM5100_AIF1TX8MIX_INPUT_3_VOLUME => true,
        WM5100_AIF1TX8MIX_INPUT_4_SOURCE => true,
        WM5100_AIF1TX8MIX_INPUT_4_VOLUME => true,
        WM5100_AIF2TX1MIX_INPUT_1_SOURCE => true,
        WM5100_AIF2TX1MIX_INPUT_1_VOLUME => true,
        WM5100_AIF2TX1MIX_INPUT_2_SOURCE => true,
        WM5100_AIF2TX1MIX_INPUT_2_VOLUME => true,
        WM5100_AIF2TX1MIX_INPUT_3_SOURCE => true,
        WM5100_AIF2TX1MIX_INPUT_3_VOLUME => true,
        WM5100_AIF2TX1MIX_INPUT_4_SOURCE => true,
        WM5100_AIF2TX1MIX_INPUT_4_VOLUME => true,
        WM5100_AIF2TX2MIX_INPUT_1_SOURCE => true,
        WM5100_AIF2TX2MIX_INPUT_1_VOLUME => true,
        WM5100_AIF2TX2MIX_INPUT_2_SOURCE => true,
        WM5100_AIF2TX2MIX_INPUT_2_VOLUME => true,
        WM5100_AIF2TX2MIX_INPUT_3_SOURCE => true,
        WM5100_AIF2TX2MIX_INPUT_3_VOLUME => true,
        WM5100_AIF2TX2MIX_INPUT_4_SOURCE => true,
        WM5100_AIF2TX2MIX_INPUT_4_VOLUME => true,
        WM5100_AIF3TX1MIX_INPUT_1_SOURCE => true,
        WM5100_AIF3TX1MIX_INPUT_1_VOLUME => true,
        WM5100_AIF3TX1MIX_INPUT_2_SOURCE => true,
        WM5100_AIF3TX1MIX_INPUT_2_VOLUME => true,
        WM5100_AIF3TX1MIX_INPUT_3_SOURCE => true,
        WM5100_AIF3TX1MIX_INPUT_3_VOLUME => true,
        WM5100_AIF3TX1MIX_INPUT_4_SOURCE => true,
        WM5100_AIF3TX1MIX_INPUT_4_VOLUME => true,
        WM5100_AIF3TX2MIX_INPUT_1_SOURCE => true,
        WM5100_AIF3TX2MIX_INPUT_1_VOLUME => true,
        WM5100_AIF3TX2MIX_INPUT_2_SOURCE => true,
        WM5100_AIF3TX2MIX_INPUT_2_VOLUME => true,
        WM5100_AIF3TX2MIX_INPUT_3_SOURCE => true,
        WM5100_AIF3TX2MIX_INPUT_3_VOLUME => true,
        WM5100_AIF3TX2MIX_INPUT_4_SOURCE => true,
        WM5100_AIF3TX2MIX_INPUT_4_VOLUME => true,
        WM5100_EQ1MIX_INPUT_1_SOURCE => true,
        WM5100_EQ1MIX_INPUT_1_VOLUME => true,
        WM5100_EQ1MIX_INPUT_2_SOURCE => true,
        WM5100_EQ1MIX_INPUT_2_VOLUME => true,
        WM5100_EQ1MIX_INPUT_3_SOURCE => true,
        WM5100_EQ1MIX_INPUT_3_VOLUME => true,
        WM5100_EQ1MIX_INPUT_4_SOURCE => true,
        WM5100_EQ1MIX_INPUT_4_VOLUME => true,
        WM5100_EQ2MIX_INPUT_1_SOURCE => true,
        WM5100_EQ2MIX_INPUT_1_VOLUME => true,
        WM5100_EQ2MIX_INPUT_2_SOURCE => true,
        WM5100_EQ2MIX_INPUT_2_VOLUME => true,
        WM5100_EQ2MIX_INPUT_3_SOURCE => true,
        WM5100_EQ2MIX_INPUT_3_VOLUME => true,
        WM5100_EQ2MIX_INPUT_4_SOURCE => true,
        WM5100_EQ2MIX_INPUT_4_VOLUME => true,
        WM5100_EQ3MIX_INPUT_1_SOURCE => true,
        WM5100_EQ3MIX_INPUT_1_VOLUME => true,
        WM5100_EQ3MIX_INPUT_2_SOURCE => true,
        WM5100_EQ3MIX_INPUT_2_VOLUME => true,
        WM5100_EQ3MIX_INPUT_3_SOURCE => true,
        WM5100_EQ3MIX_INPUT_3_VOLUME => true,
        WM5100_EQ3MIX_INPUT_4_SOURCE => true,
        WM5100_EQ3MIX_INPUT_4_VOLUME => true,
        WM5100_EQ4MIX_INPUT_1_SOURCE => true,
        WM5100_EQ4MIX_INPUT_1_VOLUME => true,
        WM5100_EQ4MIX_INPUT_2_SOURCE => true,
        WM5100_EQ4MIX_INPUT_2_VOLUME => true,
        WM5100_EQ4MIX_INPUT_3_SOURCE => true,
        WM5100_EQ4MIX_INPUT_3_VOLUME => true,
        WM5100_EQ4MIX_INPUT_4_SOURCE => true,
        WM5100_EQ4MIX_INPUT_4_VOLUME => true,
        WM5100_DRC1LMIX_INPUT_1_SOURCE => true,
        WM5100_DRC1LMIX_INPUT_1_VOLUME => true,
        WM5100_DRC1LMIX_INPUT_2_SOURCE => true,
        WM5100_DRC1LMIX_INPUT_2_VOLUME => true,
        WM5100_DRC1LMIX_INPUT_3_SOURCE => true,
        WM5100_DRC1LMIX_INPUT_3_VOLUME => true,
        WM5100_DRC1LMIX_INPUT_4_SOURCE => true,
        WM5100_DRC1LMIX_INPUT_4_VOLUME => true,
        WM5100_DRC1RMIX_INPUT_1_SOURCE => true,
        WM5100_DRC1RMIX_INPUT_1_VOLUME => true,
        WM5100_DRC1RMIX_INPUT_2_SOURCE => true,
        WM5100_DRC1RMIX_INPUT_2_VOLUME => true,
        WM5100_DRC1RMIX_INPUT_3_SOURCE => true,
        WM5100_DRC1RMIX_INPUT_3_VOLUME => true,
        WM5100_DRC1RMIX_INPUT_4_SOURCE => true,
        WM5100_DRC1RMIX_INPUT_4_VOLUME => true,
        WM5100_HPLP1MIX_INPUT_1_SOURCE => true,
        WM5100_HPLP1MIX_INPUT_1_VOLUME => true,
        WM5100_HPLP1MIX_INPUT_2_SOURCE => true,
        WM5100_HPLP1MIX_INPUT_2_VOLUME => true,
        WM5100_HPLP1MIX_INPUT_3_SOURCE => true,
        WM5100_HPLP1MIX_INPUT_3_VOLUME => true,
        WM5100_HPLP1MIX_INPUT_4_SOURCE => true,
        WM5100_HPLP1MIX_INPUT_4_VOLUME => true,
        WM5100_HPLP2MIX_INPUT_1_SOURCE => true,
        WM5100_HPLP2MIX_INPUT_1_VOLUME => true,
        WM5100_HPLP2MIX_INPUT_2_SOURCE => true,
        WM5100_HPLP2MIX_INPUT_2_VOLUME => true,
        WM5100_HPLP2MIX_INPUT_3_SOURCE => true,
        WM5100_HPLP2MIX_INPUT_3_VOLUME => true,
        WM5100_HPLP2MIX_INPUT_4_SOURCE => true,
        WM5100_HPLP2MIX_INPUT_4_VOLUME => true,
        WM5100_HPLP3MIX_INPUT_1_SOURCE => true,
        WM5100_HPLP3MIX_INPUT_1_VOLUME => true,
        WM5100_HPLP3MIX_INPUT_2_SOURCE => true,
        WM5100_HPLP3MIX_INPUT_2_VOLUME => true,
        WM5100_HPLP3MIX_INPUT_3_SOURCE => true,
        WM5100_HPLP3MIX_INPUT_3_VOLUME => true,
        WM5100_HPLP3MIX_INPUT_4_SOURCE => true,
        WM5100_HPLP3MIX_INPUT_4_VOLUME => true,
        WM5100_HPLP4MIX_INPUT_1_SOURCE => true,
        WM5100_HPLP4MIX_INPUT_1_VOLUME => true,
        WM5100_HPLP4MIX_INPUT_2_SOURCE => true,
        WM5100_HPLP4MIX_INPUT_2_VOLUME => true,
        WM5100_HPLP4MIX_INPUT_3_SOURCE => true,
        WM5100_HPLP4MIX_INPUT_3_VOLUME => true,
        WM5100_HPLP4MIX_INPUT_4_SOURCE => true,
        WM5100_HPLP4MIX_INPUT_4_VOLUME => true,
        WM5100_DSP1LMIX_INPUT_1_SOURCE => true,
        WM5100_DSP1LMIX_INPUT_1_VOLUME => true,
        WM5100_DSP1LMIX_INPUT_2_SOURCE => true,
        WM5100_DSP1LMIX_INPUT_2_VOLUME => true,
        WM5100_DSP1LMIX_INPUT_3_SOURCE => true,
        WM5100_DSP1LMIX_INPUT_3_VOLUME => true,
        WM5100_DSP1LMIX_INPUT_4_SOURCE => true,
        WM5100_DSP1LMIX_INPUT_4_VOLUME => true,
        WM5100_DSP1RMIX_INPUT_1_SOURCE => true,
        WM5100_DSP1RMIX_INPUT_1_VOLUME => true,
        WM5100_DSP1RMIX_INPUT_2_SOURCE => true,
        WM5100_DSP1RMIX_INPUT_2_VOLUME => true,
        WM5100_DSP1RMIX_INPUT_3_SOURCE => true,
        WM5100_DSP1RMIX_INPUT_3_VOLUME => true,
        WM5100_DSP1RMIX_INPUT_4_SOURCE => true,
        WM5100_DSP1RMIX_INPUT_4_VOLUME => true,
        WM5100_DSP1AUX1MIX_INPUT_1_SOURCE => true,
        WM5100_DSP1AUX2MIX_INPUT_1_SOURCE => true,
        WM5100_DSP1AUX3MIX_INPUT_1_SOURCE => true,
        WM5100_DSP1AUX4MIX_INPUT_1_SOURCE => true,
        WM5100_DSP1AUX5MIX_INPUT_1_SOURCE => true,
        WM5100_DSP1AUX6MIX_INPUT_1_SOURCE => true,
        WM5100_DSP2LMIX_INPUT_1_SOURCE => true,
        WM5100_DSP2LMIX_INPUT_1_VOLUME => true,
        WM5100_DSP2LMIX_INPUT_2_SOURCE => true,
        WM5100_DSP2LMIX_INPUT_2_VOLUME => true,
        WM5100_DSP2LMIX_INPUT_3_SOURCE => true,
        WM5100_DSP2LMIX_INPUT_3_VOLUME => true,
        WM5100_DSP2LMIX_INPUT_4_SOURCE => true,
        WM5100_DSP2LMIX_INPUT_4_VOLUME => true,
        WM5100_DSP2RMIX_INPUT_1_SOURCE => true,
        WM5100_DSP2RMIX_INPUT_1_VOLUME => true,
        WM5100_DSP2RMIX_INPUT_2_SOURCE => true,
        WM5100_DSP2RMIX_INPUT_2_VOLUME => true,
        WM5100_DSP2RMIX_INPUT_3_SOURCE => true,
        WM5100_DSP2RMIX_INPUT_3_VOLUME => true,
        WM5100_DSP2RMIX_INPUT_4_SOURCE => true,
        WM5100_DSP2RMIX_INPUT_4_VOLUME => true,
        WM5100_DSP2AUX1MIX_INPUT_1_SOURCE => true,
        WM5100_DSP2AUX2MIX_INPUT_1_SOURCE => true,
        WM5100_DSP2AUX3MIX_INPUT_1_SOURCE => true,
        WM5100_DSP2AUX4MIX_INPUT_1_SOURCE => true,
        WM5100_DSP2AUX5MIX_INPUT_1_SOURCE => true,
        WM5100_DSP2AUX6MIX_INPUT_1_SOURCE => true,
        WM5100_DSP3LMIX_INPUT_1_SOURCE => true,
        WM5100_DSP3LMIX_INPUT_1_VOLUME => true,
        WM5100_DSP3LMIX_INPUT_2_SOURCE => true,
        WM5100_DSP3LMIX_INPUT_2_VOLUME => true,
        WM5100_DSP3LMIX_INPUT_3_SOURCE => true,
        WM5100_DSP3LMIX_INPUT_3_VOLUME => true,
        WM5100_DSP3LMIX_INPUT_4_SOURCE => true,
        WM5100_DSP3LMIX_INPUT_4_VOLUME => true,
        WM5100_DSP3RMIX_INPUT_1_SOURCE => true,
        WM5100_DSP3RMIX_INPUT_1_VOLUME => true,
        WM5100_DSP3RMIX_INPUT_2_SOURCE => true,
        WM5100_DSP3RMIX_INPUT_2_VOLUME => true,
        WM5100_DSP3RMIX_INPUT_3_SOURCE => true,
        WM5100_DSP3RMIX_INPUT_3_VOLUME => true,
        WM5100_DSP3RMIX_INPUT_4_SOURCE => true,
        WM5100_DSP3RMIX_INPUT_4_VOLUME => true,
        WM5100_DSP3AUX1MIX_INPUT_1_SOURCE => true,
        WM5100_DSP3AUX2MIX_INPUT_1_SOURCE => true,
        WM5100_DSP3AUX3MIX_INPUT_1_SOURCE => true,
        WM5100_DSP3AUX4MIX_INPUT_1_SOURCE => true,
        WM5100_DSP3AUX5MIX_INPUT_1_SOURCE => true,
        WM5100_DSP3AUX6MIX_INPUT_1_SOURCE => true,
        WM5100_ASRC1LMIX_INPUT_1_SOURCE => true,
        WM5100_ASRC1RMIX_INPUT_1_SOURCE => true,
        WM5100_ASRC2LMIX_INPUT_1_SOURCE => true,
        WM5100_ASRC2RMIX_INPUT_1_SOURCE => true,
        WM5100_ISRC1DEC1MIX_INPUT_1_SOURCE => true,
        WM5100_ISRC1DEC2MIX_INPUT_1_SOURCE => true,
        WM5100_ISRC1DEC3MIX_INPUT_1_SOURCE => true,
        WM5100_ISRC1DEC4MIX_INPUT_1_SOURCE => true,
        WM5100_ISRC1INT1MIX_INPUT_1_SOURCE => true,
        WM5100_ISRC1INT2MIX_INPUT_1_SOURCE => true,
        WM5100_ISRC1INT3MIX_INPUT_1_SOURCE => true,
        WM5100_ISRC1INT4MIX_INPUT_1_SOURCE => true,
        WM5100_ISRC2DEC1MIX_INPUT_1_SOURCE => true,
        WM5100_ISRC2DEC2MIX_INPUT_1_SOURCE => true,
        WM5100_ISRC2DEC3MIX_INPUT_1_SOURCE => true,
        WM5100_ISRC2DEC4MIX_INPUT_1_SOURCE => true,
        WM5100_ISRC2INT1MIX_INPUT_1_SOURCE => true,
        WM5100_ISRC2INT2MIX_INPUT_1_SOURCE => true,
        WM5100_ISRC2INT3MIX_INPUT_1_SOURCE => true,
        WM5100_ISRC2INT4MIX_INPUT_1_SOURCE => true,
        WM5100_GPIO_CTRL_1 => true,
        WM5100_GPIO_CTRL_2 => true,
        WM5100_GPIO_CTRL_3 => true,
        WM5100_GPIO_CTRL_4 => true,
        WM5100_GPIO_CTRL_5 => true,
        WM5100_GPIO_CTRL_6 => true,
        WM5100_MISC_PAD_CTRL_1 => true,
        WM5100_MISC_PAD_CTRL_2 => true,
        WM5100_MISC_PAD_CTRL_3 => true,
        WM5100_MISC_PAD_CTRL_4 => true,
        WM5100_MISC_PAD_CTRL_5 => true,
        WM5100_MISC_GPIO_1 => true,
        WM5100_INTERRUPT_STATUS_1 => true,
        WM5100_INTERRUPT_STATUS_2 => true,
        WM5100_INTERRUPT_STATUS_3 => true,
        WM5100_INTERRUPT_STATUS_4 => true,
        WM5100_INTERRUPT_RAW_STATUS_2 => true,
        WM5100_INTERRUPT_RAW_STATUS_3 => true,
        WM5100_INTERRUPT_RAW_STATUS_4 => true,
        WM5100_INTERRUPT_STATUS_1_MASK => true,
        WM5100_INTERRUPT_STATUS_2_MASK => true,
        WM5100_INTERRUPT_STATUS_3_MASK => true,
        WM5100_INTERRUPT_STATUS_4_MASK => true,
        WM5100_INTERRUPT_CONTROL => true,
        WM5100_IRQ_DEBOUNCE_1 => true,
        WM5100_IRQ_DEBOUNCE_2 => true,
        WM5100_FX_CTRL => true,
        WM5100_EQ1_1 => true,
        WM5100_EQ1_2 => true,
        WM5100_EQ1_3 => true,
        WM5100_EQ1_4 => true,
        WM5100_EQ1_5 => true,
        WM5100_EQ1_6 => true,
        WM5100_EQ1_7 => true,
        WM5100_EQ1_8 => true,
        WM5100_EQ1_9 => true,
        WM5100_EQ1_10 => true,
        WM5100_EQ1_11 => true,
        WM5100_EQ1_12 => true,
        WM5100_EQ1_13 => true,
        WM5100_EQ1_14 => true,
        WM5100_EQ1_15 => true,
        WM5100_EQ1_16 => true,
        WM5100_EQ1_17 => true,
        WM5100_EQ1_18 => true,
        WM5100_EQ1_19 => true,
        WM5100_EQ1_20 => true,
        WM5100_EQ2_1 => true,
        WM5100_EQ2_2 => true,
        WM5100_EQ2_3 => true,
        WM5100_EQ2_4 => true,
        WM5100_EQ2_5 => true,
        WM5100_EQ2_6 => true,
        WM5100_EQ2_7 => true,
        WM5100_EQ2_8 => true,
        WM5100_EQ2_9 => true,
        WM5100_EQ2_10 => true,
        WM5100_EQ2_11 => true,
        WM5100_EQ2_12 => true,
        WM5100_EQ2_13 => true,
        WM5100_EQ2_14 => true,
        WM5100_EQ2_15 => true,
        WM5100_EQ2_16 => true,
        WM5100_EQ2_17 => true,
        WM5100_EQ2_18 => true,
        WM5100_EQ2_19 => true,
        WM5100_EQ2_20 => true,
        WM5100_EQ3_1 => true,
        WM5100_EQ3_2 => true,
        WM5100_EQ3_3 => true,
        WM5100_EQ3_4 => true,
        WM5100_EQ3_5 => true,
        WM5100_EQ3_6 => true,
        WM5100_EQ3_7 => true,
        WM5100_EQ3_8 => true,
        WM5100_EQ3_9 => true,
        WM5100_EQ3_10 => true,
        WM5100_EQ3_11 => true,
        WM5100_EQ3_12 => true,
        WM5100_EQ3_13 => true,
        WM5100_EQ3_14 => true,
        WM5100_EQ3_15 => true,
        WM5100_EQ3_16 => true,
        WM5100_EQ3_17 => true,
        WM5100_EQ3_18 => true,
        WM5100_EQ3_19 => true,
        WM5100_EQ3_20 => true,
        WM5100_EQ4_1 => true,
        WM5100_EQ4_2 => true,
        WM5100_EQ4_3 => true,
        WM5100_EQ4_4 => true,
        WM5100_EQ4_5 => true,
        WM5100_EQ4_6 => true,
        WM5100_EQ4_7 => true,
        WM5100_EQ4_8 => true,
        WM5100_EQ4_9 => true,
        WM5100_EQ4_10 => true,
        WM5100_EQ4_11 => true,
        WM5100_EQ4_12 => true,
        WM5100_EQ4_13 => true,
        WM5100_EQ4_14 => true,
        WM5100_EQ4_15 => true,
        WM5100_EQ4_16 => true,
        WM5100_EQ4_17 => true,
        WM5100_EQ4_18 => true,
        WM5100_EQ4_19 => true,
        WM5100_EQ4_20 => true,
        WM5100_DRC1_CTRL1 => true,
        WM5100_DRC1_CTRL2 => true,
        WM5100_DRC1_CTRL3 => true,
        WM5100_DRC1_CTRL4 => true,
        WM5100_DRC1_CTRL5 => true,
        WM5100_HPLPF1_1 => true,
        WM5100_HPLPF1_2 => true,
        WM5100_HPLPF2_1 => true,
        WM5100_HPLPF2_2 => true,
        WM5100_HPLPF3_1 => true,
        WM5100_HPLPF3_2 => true,
        WM5100_HPLPF4_1 => true,
        WM5100_HPLPF4_2 => true,
        WM5100_DSP1_CONTROL_1 => true,
        WM5100_DSP1_CONTROL_2 => true,
        WM5100_DSP1_CONTROL_3 => true,
        WM5100_DSP1_CONTROL_4 => true,
        WM5100_DSP1_CONTROL_5 => true,
        WM5100_DSP1_CONTROL_6 => true,
        WM5100_DSP1_CONTROL_7 => true,
        WM5100_DSP1_CONTROL_8 => true,
        WM5100_DSP1_CONTROL_9 => true,
        WM5100_DSP1_CONTROL_10 => true,
        WM5100_DSP1_CONTROL_11 => true,
        WM5100_DSP1_CONTROL_12 => true,
        WM5100_DSP1_CONTROL_13 => true,
        WM5100_DSP1_CONTROL_14 => true,
        WM5100_DSP1_CONTROL_15 => true,
        WM5100_DSP1_CONTROL_16 => true,
        WM5100_DSP1_CONTROL_17 => true,
        WM5100_DSP1_CONTROL_18 => true,
        WM5100_DSP1_CONTROL_19 => true,
        WM5100_DSP1_CONTROL_20 => true,
        WM5100_DSP1_CONTROL_21 => true,
        WM5100_DSP1_CONTROL_22 => true,
        WM5100_DSP1_CONTROL_23 => true,
        WM5100_DSP1_CONTROL_24 => true,
        WM5100_DSP1_CONTROL_25 => true,
        WM5100_DSP1_CONTROL_26 => true,
        WM5100_DSP1_CONTROL_27 => true,
        WM5100_DSP1_CONTROL_28 => true,
        WM5100_DSP1_CONTROL_29 => true,
        WM5100_DSP1_CONTROL_30 => true,
        WM5100_DSP2_CONTROL_1 => true,
        WM5100_DSP2_CONTROL_2 => true,
        WM5100_DSP2_CONTROL_3 => true,
        WM5100_DSP2_CONTROL_4 => true,
        WM5100_DSP2_CONTROL_5 => true,
        WM5100_DSP2_CONTROL_6 => true,
        WM5100_DSP2_CONTROL_7 => true,
        WM5100_DSP2_CONTROL_8 => true,
        WM5100_DSP2_CONTROL_9 => true,
        WM5100_DSP2_CONTROL_10 => true,
        WM5100_DSP2_CONTROL_11 => true,
        WM5100_DSP2_CONTROL_12 => true,
        WM5100_DSP2_CONTROL_13 => true,
        WM5100_DSP2_CONTROL_14 => true,
        WM5100_DSP2_CONTROL_15 => true,
        WM5100_DSP2_CONTROL_16 => true,
        WM5100_DSP2_CONTROL_17 => true,
        WM5100_DSP2_CONTROL_18 => true,
        WM5100_DSP2_CONTROL_19 => true,
        WM5100_DSP2_CONTROL_20 => true,
        WM5100_DSP2_CONTROL_21 => true,
        WM5100_DSP2_CONTROL_22 => true,
        WM5100_DSP2_CONTROL_23 => true,
        WM5100_DSP2_CONTROL_24 => true,
        WM5100_DSP2_CONTROL_25 => true,
        WM5100_DSP2_CONTROL_26 => true,
        WM5100_DSP2_CONTROL_27 => true,
        WM5100_DSP2_CONTROL_28 => true,
        WM5100_DSP2_CONTROL_29 => true,
        WM5100_DSP2_CONTROL_30 => true,
        WM5100_DSP3_CONTROL_1 => true,
        WM5100_DSP3_CONTROL_2 => true,
        WM5100_DSP3_CONTROL_3 => true,
        WM5100_DSP3_CONTROL_4 => true,
        WM5100_DSP3_CONTROL_5 => true,
        WM5100_DSP3_CONTROL_6 => true,
        WM5100_DSP3_CONTROL_7 => true,
        WM5100_DSP3_CONTROL_8 => true,
        WM5100_DSP3_CONTROL_9 => true,
        WM5100_DSP3_CONTROL_10 => true,
        WM5100_DSP3_CONTROL_11 => true,
        WM5100_DSP3_CONTROL_12 => true,
        WM5100_DSP3_CONTROL_13 => true,
        WM5100_DSP3_CONTROL_14 => true,
        WM5100_DSP3_CONTROL_15 => true,
        WM5100_DSP3_CONTROL_16 => true,
        WM5100_DSP3_CONTROL_17 => true,
        WM5100_DSP3_CONTROL_18 => true,
        WM5100_DSP3_CONTROL_19 => true,
        WM5100_DSP3_CONTROL_20 => true,
        WM5100_DSP3_CONTROL_21 => true,
        WM5100_DSP3_CONTROL_22 => true,
        WM5100_DSP3_CONTROL_23 => true,
        WM5100_DSP3_CONTROL_24 => true,
        WM5100_DSP3_CONTROL_25 => true,
        WM5100_DSP3_CONTROL_26 => true,
        WM5100_DSP3_CONTROL_27 => true,
        WM5100_DSP3_CONTROL_28 => true,
        WM5100_DSP3_CONTROL_29 => true,
        WM5100_DSP3_CONTROL_30 => true,
        _ => {
            (reg >= WM5100_DSP1_PM_0 && reg <= WM5100_DSP1_PM_1535) ||
            (reg >= WM5100_DSP1_ZM_0 && reg <= WM5100_DSP1_ZM_2047) ||
            (reg >= WM5100_DSP1_DM_0 && reg <= WM5100_DSP1_DM_511) ||
            (reg >= WM5100_DSP2_PM_0 && reg <= WM5100_DSP2_PM_1535) ||
            (reg >= WM5100_DSP2_ZM_0 && reg <= WM5100_DSP2_ZM_2047) ||
            (reg >= WM5100_DSP2_DM_0 && reg <= WM5100_DSP2_DM_511) ||
            (reg >= WM5100_DSP3_PM_0 && reg <= WM5100_DSP3_PM_1535) ||
            (reg >= WM5100_DSP3_ZM_0 && reg <= WM5100_DSP3_ZM_2047) ||
            (reg >= WM5100_DSP3_DM_0 && reg <= WM5100_DSP3_DM_511)
        }
    }
}

pub static mut wm5100_reg_defaults: [reg_default; WM5100_REGISTER_COUNT] = [
    reg_default { reg: 0x0000, def: 0x0000 },  /* R0     - software reset */
    reg_default { reg: 0x0001, def: 0x0000 },  /* R1     - Device Revision */
    reg_default { reg: 0x0010, def: 0x0801 },  /* R16    - Ctrl IF 1 */
    reg_default { reg: 0x0020, def: 0x0000 },  /* R32    - Tone Generator 1 */
    reg_default { reg: 0x0030, def: 0x0000 },  /* R48    - PWM Drive 1 */
    reg_default { reg: 0x0031, def: 0x0100 },  /* R49    - PWM Drive 2 */
    reg_default { reg: 0x0032, def: 0x0100 },  /* R50    - PWM Drive 3 */
    reg_default { reg: 0x0100, def: 0x0002 },  /* R256   - Clocking 1 */
    reg_default { reg: 0x0101, def: 0x0000 },  /* R257   - Clocking 3 */
    reg_default { reg: 0x0102, def: 0x0011 },  /* R258   - Clocking 4 */
    reg_default { reg: 0x0103, def: 0x0011 },  /* R259   - Clocking 5 */
    reg_default { reg: 0x0104, def: 0x0011 },  /* R260   - Clocking 6 */
    reg_default { reg: 0x0107, def: 0x0000 },  /* R263   - Clocking 7 */
    reg_default { reg: 0x0108, def: 0x0000 },  /* R264   - Clocking 8 */
    reg_default { reg: 0x0120, def: 0x0000 },  /* R288   - ASRC_ENABLE */
    reg_default { reg: 0x0121, def: 0x0000 },  /* R289   - ASRC_STATUS */
    reg_default { reg: 0x0122, def: 0x0000 },  /* R290   - ASRC_RATE1 */
    reg_default { reg: 0x0141, def: 0x8000 },  /* R321   - ISRC 1 CTRL 1 */
    reg_default { reg: 0x0142, def: 0x0000 },  /* R322   - ISRC 1 CTRL 2 */
    reg_default { reg: 0x0143, def: 0x8000 },  /* R323   - ISRC 2 CTRL1 */
    reg_default { reg: 0x0144, def: 0x0000 },  /* R324   - ISRC 2 CTRL 2 */
    reg_default { reg: 0x0182, def: 0x0000 },  /* R386   - FLL1 Control 1 */
    reg_default { reg: 0x0183, def: 0x0000 },  /* R387   - FLL1 Control 2 */
    reg_default { reg: 0x0184, def: 0x0000 },  /* R388   - FLL1 Control 3 */
    reg_default { reg: 0x0186, def: 0x0177 },  /* R390   - FLL1 Control 5 */
    reg_default { reg: 0x0187, def: 0x0001 },  /* R391   - FLL1 Control 6 */
    reg_default { reg: 0x0188, def: 0x0000 },  /* R392   - FLL1 EFS 1 */
    reg_default { reg: 0x01A2, def: 0x0000 },  /* R418   - FLL2 Control 1 */
    reg_default { reg: 0x01A3, def: 0x0000 },  /* R419   - FLL2 Control 2 */
    reg_default { reg: 0x01A4, def: 0x0000 },  /* R420   - FLL2 Control 3 */
    reg_default { reg: 0x01A6, def: 0x0177 },  /* R422   - FLL2 Control 5 */
    reg_default { reg: 0x01A7, def: 0x0001 },  /* R423   - FLL2 Control 6 */
    reg_default { reg: 0x01A8, def: 0x0000 },  /* R424   - FLL2 EFS 1 */
    reg_default { reg: 0x0200, def: 0x0020 },  /* R512   - Mic Charge Pump 1 */
    reg_default { reg: 0x0201, def: 0xB084 },  /* R513   - Mic Charge Pump 2 */
    reg_default { reg: 0x0202, def: 0xBBDE },  /* R514   - HP Charge Pump 1 */
    reg_default { reg: 0x0211, def: 0x20D4 },  /* R529   - LDO1 Control */
    reg_default { reg: 0x0215, def: 0x0062 },  /* R533   - Mic Bias Ctrl 1 */
    reg_default { reg: 0x0216, def: 0x0062 },  /* R534   - Mic Bias Ctrl 2 */
    reg_default { reg: 0x0217, def: 0x0062 },  /* R535   - Mic Bias Ctrl 3 */
    reg_default { reg: 0x0280, def: 0x0004 },  /* R640   - Accessory Detect Mode 1 */
    reg_default { reg: 0x0288, def: 0x0020 },  /* R648   - Headphone Detect 1 */
    reg_default { reg: 0x0289, def: 0x0000 },  /* R649   - Headphone Detect 2 */
    reg_default { reg: 0x0290, def: 0x1100 },  /* R656   - Mic Detect 1 */
    reg_default { reg: 0x0291, def: 0x009F },  /* R657   - Mic Detect 2 */
    reg_default { reg: 0x0292, def: 0x0000 },  /* R658   - Mic Detect 3 */
    reg_default { reg: 0x0301, def: 0x0000 },  /* R769   - Input Enables */
    reg_default { reg: 0x0302, def: 0x0000 },  /* R770   - Input Enables Status */
    reg_default { reg: 0x0310, def: 0x2280 },  /* R784   - Status */
    reg_default { reg: 0x0311, def: 0x0080 },  /* R785   - IN1R Control */
    reg_default { reg: 0x0312, def: 0x2280 },  /* R786   - IN2L Control */
    reg_default { reg: 0x0313, def: 0x0080 },  /* R787   - IN2R Control */
    reg_default { reg: 0x0314, def: 0x2280 },  /* R788   - IN3L Control */
    reg_default { reg: 0x0315, def: 0x0080 },  /* R789   - IN3R Control */
    reg_default { reg: 0x0316, def: 0x2280 },  /* R790   - IN4L Control */
    reg_default { reg: 0x0317, def: 0x0080 },  /* R791   - IN4R Control */
    reg_default { reg: 0x0318, def: 0x0000 },  /* R792   - RXANC_SRC */
    reg_default { reg: 0x0319, def: 0x0022 },  /* R793   - Input Volume Ramp */
    reg_default { reg: 0x0320, def: 0x0180 },  /* R800   - ADC Digital Volume 1L */
    reg_default { reg: 0x0321, def: 0x0180 },  /* R801   - ADC Digital Volume 1R */
    reg_default { reg: 0x0322, def: 0x0180 },  /* R802   - ADC Digital Volume 2L */
    reg_default { reg: 0x0323, def: 0x0180 },  /* R803   - ADC Digital Volume 2R */
    reg_default { reg: 0x0324, def: 0x0180 },  /* R804   - ADC Digital Volume 3L */
    reg_default { reg: 0x0325, def: 0x0180 },  /* R805   - ADC Digital Volume 3R */
    reg_default { reg: 0x0326, def: 0x0180 },  /* R806   - ADC Digital Volume 4L */
    reg_default { reg: 0x0327, def: 0x0180 },  /* R807   - ADC Digital Volume 4R */
    reg_default { reg: 0x0401, def: 0x0000 },  /* R1025  - Output Enables 2 */
    reg_default { reg: 0x0402, def: 0x0000 },  /* R1026  - Output Status 1 */
    reg_default { reg: 0x0403, def: 0x0000 },  /* R1027  - Output Status 2 */
    reg_default { reg: 0x0408, def: 0x0000 },  /* R1032  - Channel Enables 1 */
    reg_default { reg: 0x0410, def: 0x0080 },  /* R1040  - Out Volume 1L */
    reg_default { reg: 0x0411, def: 0x0080 },  /* R1041  - Out Volume 1R */
    reg_default { reg: 0x0412, def: 0x0080 },  /* R1042  - DAC Volume Limit 1L */
    reg_default { reg: 0x0413, def: 0x0080 },  /* R1043  - DAC Volume Limit 1R */
    reg_default { reg: 0x0414, def: 0x0080 },  /* R1044  - Out Volume 2L */
    reg_default { reg: 0x0415, def: 0x0080 },  /* R1045  - Out Volume 2R */
    reg_default { reg: 0x0416, def: 0x0080 },  /* R1046  - DAC Volume Limit 2L */
    reg_default { reg: 0x0417, def: 0x0080 },  /* R1047  - DAC Volume Limit 2R */
    reg_default { reg: 0x0418, def: 0x0080 },  /* R1048  - Out Volume 3L */
    reg_default { reg: 0x0419, def: 0x0080 },  /* R1049  - Out Volume 3R */
    reg_default { reg: 0x041A, def: 0x0080 },  /* R1050  - DAC Volume Limit 3L */
    reg_default { reg: 0x041B, def: 0x0080 },  /* R1051  - DAC Volume Limit 3R */
    reg_default { reg: 0x041C, def: 0x0080 },  /* R1052  - Out Volume 4L */
    reg_default { reg: 0x041D, def: 0x0080 },  /* R1053  - Out Volume 4R */
    reg_default { reg: 0x041E, def: 0x0080 },  /* R1054  - DAC Volume Limit 5L */
    reg_default { reg: 0x041F, def: 0x0080 },  /* R1055  - DAC Volume Limit 5R */
    reg_default { reg: 0x0420, def: 0x0080 },  /* R1056  - DAC Volume Limit 6L */
    reg_default { reg: 0x0421, def: 0x0080 },  /* R1057  - DAC Volume Limit 6R */
    reg_default { reg: 0x0440, def: 0x0000 },  /* R1088  - DAC AEC Control 1 */
    reg_default { reg: 0x0441, def: 0x0022 },  /* R1089  - Output Volume Ramp */
    reg_default { reg: 0x0480, def: 0x0180 },  /* R1152  - DAC Digital Volume 1L */
    reg_default { reg: 0x0481, def: 0x0180 },  /* R1153  - DAC Digital Volume 1R */
    reg_default { reg: 0x0482, def: 0x0180 },  /* R1154  - DAC Digital Volume 2L */
    reg_default { reg: 0x0483, def: 0x0180 },  /* R1155  - DAC Digital Volume 2R */
    reg_default { reg: 0x0484, def: 0x0180 },  /* R1156  - DAC Digital Volume 3L */
    reg_default { reg: 0x0485, def: 0x0180 },  /* R1157  - DAC Digital Volume 3R */
    reg_default { reg: 0x0486, def: 0x0180 },  /* R1158  - DAC Digital Volume 4L */
    reg_default { reg: 0x0487, def: 0x0180 },  /* R1159  - DAC Digital Volume 4R */
    reg_default { reg: 0x0488, def: 0x0180 },  /* R1160  - DAC Digital Volume 5L */
    reg_default { reg: 0x0489, def: 0x0180 },  /* R1161  - DAC Digital Volume 5R */
    reg_default { reg: 0x048A, def: 0x0180 },  /* R1162  - DAC Digital Volume 6L */
    reg_default { reg: 0x048B, def: 0x0180 },  /* R1163  - DAC Digital Volume 6R */
    reg_default { reg: 0x04C0, def: 0x0069 },  /* R1216  - PDM SPK1 CTRL 1 */
    reg_default { reg: 0x04C1, def: 0x0000 },  /* R1217  - PDM SPK1 CTRL 2 */
    reg_default { reg: 0x04C2, def: 0x0069 },  /* R1218  - PDM SPK2 CTRL 1 */
    reg_default { reg: 0x04C3, def: 0x0000 },  /* R1219  - PDM SPK2 CTRL 2 */
    reg_default { reg: 0x0500, def: 0x000C },  /* R1280  - Audio IF 1_1 */
    reg_default { reg: 0x0501, def: 0x0008 },  /* R1281  - Audio IF 1_2 */
    reg_default { reg: 0x0502, def: 0x0000 },  /* R1282  - Audio IF 1_3 */
    reg_default { reg: 0x0503, def: 0x0000 },  /* R1283  - Audio IF 1_4 */
    reg_default { reg: 0x0504, def: 0x0000 },  /* R1284  - Audio IF 1_5 */
    reg_default { reg: 0x0505, def: 0x0300 },  /* R1285  - Audio IF 1_6 */
    reg_default { reg: 0x0506, def: 0x0300 },  /* R1286  - Audio IF 1_7 */
    reg_default { reg: 0x0507, def: 0x1820 },  /* R1287  - Audio IF 1_8 */
    reg_default { reg: 0x0508, def: 0x1820 },  /* R1288  - Audio IF 1_9 */
    reg_default { reg: 0x0509, def: 0x0000 },  /* R1289  - Audio IF 1_10 */
    reg_default { reg: 0x050A, def: 0x0001 },  /* R1290  - Audio IF 1_11 */
    reg_default { reg: 0x050B, def: 0x0002 },  /* R1291  - Audio IF 1_12 */
    reg_default { reg: 0x050C, def: 0x0003 },  /* R1292  - Audio IF 1_13 */
    reg_default { reg: 0x050D, def: 0x0004 },  /* R1293  - Audio IF 1_14 */
    reg_default { reg: 0x050E, def: 0x0005 },  /* R1294  - Audio IF 1_15 */
    reg_default { reg: 0x050F, def: 0x0006 },  /* R1295  - Audio IF 1_16 */
    reg_default { reg: 0x0510, def: 0x0007 },  /* R1296  - Audio IF 1_17 */
    reg_default { reg: 0x0511, def: 0x0000 },  /* R1297  - Audio IF 1_18 */
    reg_default { reg: 0x0512, def: 0x0001 },  /* R1298  - Audio IF 1_19 */
    reg_default { reg: 0x0513, def: 0x0002 },  /* R1299  - Audio IF 1_20 */
    reg_default { reg: 0x0514, def: 0x0003 },  /* R1300  - Audio IF 1_21 */
    reg_default { reg: 0x0515, def: 0x0004 },  /* R1301  - Audio IF 1_22 */
    reg_default { reg: 0x0516, def: 0x0005 },  /* R1302  - Audio IF 1_23 */
    reg_default { reg: 0x0517, def: 0x0006 },  /* R1303  - Audio IF 1_24 */
    reg_default { reg: 0x0518, def: 0x0007 },  /* R1304  - Audio IF 1_25 */
    reg_default { reg: 0x0519, def: 0x0000 },  /* R1305  - Audio IF 1_26 */
    reg_default { reg: 0x051A, def: 0x0000 },  /* R1306  - Audio IF 1_27 */
    reg_default { reg: 0x0540, def: 0x000C },  /* R1344  - Audio IF 2_1 */
    reg_default { reg: 0x0541, def: 0x0008 },  /* R1345  - Audio IF 2_2 */
    reg_default { reg: 0x0542, def: 0x0000 },  /* R1346  - Audio IF 2_3 */
    reg_default { reg: 0x0543, def: 0x0000 },  /* R1347  - Audio IF 2_4 */
    reg_default { reg: 0x0544, def: 0x0000 },  /* R1348  - Audio IF 2_5 */
    reg_default { reg: 0x0545, def: 0x0300 },  /* R1349  - Audio IF 2_6 */
    reg_default { reg: 0x0546, def: 0x0300 },  /* R1350  - Audio IF 2_7 */
    reg_default { reg: 0x0547, def: 0x1820 },  /* R1351  - Audio IF 2_8 */
    reg_default { reg: 0x0548, def: 0x1820 },  /* R1352  - Audio IF 2_9 */
    reg_default { reg: 0x0549, def: 0x0000 },  /* R1353  - Audio IF 2_10 */
    reg_default { reg: 0x054A, def: 0x0001 },  /* R1354  - Audio IF 2_11 */
    reg_default { reg: 0x0551, def: 0x0000 },  /* R1361  - Audio IF 2_18 */
    reg_default { reg: 0x0552, def: 0x0001 },  /* R1362  - Audio IF 2_19 */
    reg_default { reg: 0x0559, def: 0x0000 },  /* R1369  - Audio IF 2_26 */
    reg_default { reg: 0x055A, def: 0x0000 },  /* R1370  - Audio IF 2_27 */
    reg_default { reg: 0x0580, def: 0x000C },  /* R1408  - Audio IF 3_1 */
    reg_default { reg: 0x0581, def: 0x0008 },  /* R1409  - Audio IF 3_2 */
    reg_default { reg: 0x0582, def: 0x0000 },  /* R1410  - Audio IF 3_3 */
    reg_default { reg: 0x0583, def: 0x0000 },  /* R1411  - Audio IF 3_4 */
    reg_default { reg: 0x0584, def: 0x0000 },  /* R1412  - Audio IF 3_5 */
    reg_default { reg: 0x0585, def: 0x0300 },  /* R1413  - Audio IF 3_6 */
    reg_default { reg: 0x0586, def: 0x0300 },  /* R1414  - Audio IF 3_7 */
    reg_default { reg: 0x0587, def: 0x1820 },  /* R1415  - Audio IF 3_8 */
    reg_default { reg: 0x0588, def: 0x1820 },  /* R1416  - Audio IF 3_9 */
    reg_default { reg: 0x0589, def: 0x0000 },  /* R1417  - Audio IF 3_10 */
    reg_default { reg: 0x058A, def: 0x0001 },  /* R1418  - Audio IF 3_11 */
    reg_default { reg: 0x0591, def: 0x0000 },  /* R1425  - Audio IF 3_18 */
    reg_default { reg: 0x0592, def: 0x0001 },  /* R1426  - Audio IF 3_19 */
    reg_default { reg: 0x0599, def: 0x0000 },  /* R1433  - Audio IF 3_26 */
    reg_default { reg: 0x059A, def: 0x0000 },  /* R1434  - Audio IF 3_27 */
    reg_default { reg: 0x0640, def: 0x0000 },  /* R1600  - PWM1MIX Input 1 Source */
    reg_default { reg: 0x0641, def: 0x0080 },  /* R1601  - PWM1MIX Input 1 Volume */
    reg_default { reg: 0x0642, def: 0x0000 },  /* R1602  - PWM1MIX Input 2 Source */
    reg_default { reg: 0x0643, def: 0x0080 },  /* R1603  - PWM1MIX Input 2 Volume */
    reg_default { reg: 0x0644, def: 0x0000 },  /* R1604  - PWM1MIX Input 3 Source */
    reg_default { reg: 0x0645, def: 0x0080 },  /* R1605  - PWM1MIX Input 3 Volume */
    reg_default { reg: 0x0646, def: 0x0000 },  /* R1606  - PWM1MIX Input 4 Source */
    reg_default { reg: 0x0647, def: 0x0080 },  /* R1607  - PWM1MIX Input 4 Volume */
    reg_default { reg: 0x0648, def: 0x0000 },  /* R1608  - PWM2MIX Input 1 Source */
    reg_default { reg: 0x0649, def: 0x0080 },  /* R1609  - PWM2MIX Input 1 Volume */
    reg_default { reg: 0x064A, def: 0x0000 },  /* R1610  - PWM2MIX Input 2 Source */
    reg_default { reg: 0x064B, def: 0x0080 },  /* R1611  - PWM2MIX Input 2 Volume */
    reg_default { reg: 0x064C, def: 0x0000 },  /* R1612  - PWM2MIX Input 3 Source */
    reg_default { reg: 0x064D, def: 0x0080 },  /* R1613  - PWM2MIX Input 3 Volume */
    reg_default { reg: 0x064E, def: 0x0000 },  /* R1614  - PWM2MIX Input 4 Source */
    reg_default { reg: 0x064F, def: 0x0080 },  /* R1615  - PWM2MIX Input 4 Volume */
    reg_default { reg: 0x0680, def: 0x0000 },  /* R1664  - OUT1LMIX Input 1 Source */
    reg_default { reg: 0x0681, def: 0x0080 },  /* R1665  - OUT1LMIX Input 1 Volume */
    reg_default { reg: 0x0682, def: 0x0000 },  /* R1666  - OUT1LMIX Input 2 Source */
    reg_default { reg: 0x0683, def: 0x0080 },  /* R1667  - OUT1LMIX Input 2 Volume */
    reg_default { reg: 0x0684, def: 0x0000 },  /* R1668  - OUT1LMIX Input 3 Source */
    reg_default { reg: 0x0685, def: 0x0080 },  /* R1669  - OUT1LMIX Input 3 Volume */
    reg_default { reg: 0x0686, def: 0x0000 },  /* R1670  - OUT1LMIX Input 4 Source */
    reg_default { reg: 0x0687, def: 0x0080 },  /* R1671  - OUT1LMIX Input 4 Volume */
    reg_default { reg: 0x0688, def: 0x0000 },  /* R1672  - OUT1RMIX Input 1 Source */
    reg_default { reg: 0x0689, def: 0x0080 },  /* R1673  - OUT1RMIX Input 1 Volume */
    reg_default { reg: 0x068A, def: 0x0000 },  /* R1674  - OUT1RMIX Input 2 Source */
    reg_default { reg: 0x068B, def: 0x0080 },  /* R1675  - OUT1RMIX Input 2 Volume */
    reg_default { reg: 0x068C, def: 0x0000 },  /* R1676  - OUT1RMIX Input 3 Source */
    reg_default { reg: 0x068D, def: 0x0080 },  /* R1677  - OUT1RMIX Input 3 Volume */
    reg_default { reg: 0x068E, def: 0x0000 },  /* R1678  - OUT1RMIX Input 4 Source */
    reg_default { reg: 0x068F, def: 0x0080 },  /* R1679  - OUT1RMIX Input 4 Volume */
    reg_default { reg: 0x0690, def: 0x0000 },  /* R1680  - OUT2LMIX Input 1 Source */
    reg_default { reg: 0x0691, def: 0x0080 },  /* R1681  - OUT2LMIX Input 1 Volume */
    reg_default { reg: 0x0692, def: 0x0000 },  /* R1682  - OUT2LMIX Input 2 Source */
    reg_default { reg: 0x0693, def: 0x0080 },  /* R1683  - OUT2LMIX Input 2 Volume */
    reg_default { reg: 0x0694, def: 0x0000 },  /* R1684  - OUT2LMIX Input 3 Source */
    reg_default { reg: 0x0695, def: 0x0080 },  /* R1685  - OUT2LMIX Input 3 Volume */
    reg_default { reg: 0x0696, def: 0x0000 },  /* R1686  - OUT2LMIX Input 4 Source */
    reg_default { reg: 0x0697, def: 0x0080 },  /* R1687  - OUT2LMIX Input 4 Volume */
    reg_default { reg: 0x0698, def: 0x0000 },  /* R1688  - OUT2RMIX Input 1 Source */
    reg_default { reg: 0x0699, def: 0x0080 },  /* R1689  - OUT2RMIX Input 1 Volume */
    reg_default { reg: 0x069A, def: 0x0000 },  /* R1690  - OUT2RMIX Input 2 Source */
    reg_default { reg: 0x069B, def: 0x0080 },  /* R1691  - OUT2RMIX Input 2 Volume */
    reg_default { reg: 0x069C, def: 0x0000 },  /* R1692  - OUT2RMIX Input 3 Source */
    reg_default { reg: 0x069D, def: 0x0080 },  /* R1693  - OUT2RMIX Input 3 Volume */
    reg_default { reg: 0x069E, def: 0x0000 },  /* R1694  - OUT2RMIX Input 4 Source */
    reg_default { reg: 0x069F, def: 0x0080 },  /* R1695  - OUT2RMIX Input 4 Volume */
    reg_default { reg: 0x06A0, def: 0x0000 },  /* R1696  - OUT3LMIX Input 1 Source */
    reg_default { reg: 0x06A1, def: 0x0080 },  /* R1697  - OUT3LMIX Input 1 Volume */
    reg_default { reg: 0x06A2, def: 0x0000 },  /* R1698  - OUT3LMIX Input 2 Source */
    reg_default { reg: 0x06A3, def: 0x0080 },  /* R1699  - OUT3LMIX Input 2 Volume */
    reg_default { reg: 0x06A4, def: 0x0000 },  /* R1700  - OUT3LMIX Input 3 Source */
    reg_default { reg: 0x06A5, def: 0x0080 },  /* R1701  - OUT3LMIX Input 3 Volume */
    reg_default { reg: 0x06A6, def: 0x0000 },  /* R1702  - OUT3LMIX Input 4 Source */
    reg_default { reg: 0x06A7, def: 0x0080 },  /* R1703  - OUT3LMIX Input 4 Volume */
    reg_default { reg: 0x06A8, def: 0x0000 },  /* R1704  - OUT3RMIX Input 1 Source */
    reg_default { reg: 0x06A9, def: 0x0080 },  /* R1705  - OUT3RMIX Input 1 Volume */
    reg_default { reg: 0x06AA, def: 0x0000 },  /* R1706  - OUT3RMIX Input 2 Source */
    reg_default { reg: 0x06AB, def: 0x0080 },  /* R1707  - OUT3RMIX Input 2 Volume */
    reg_default { reg: 0x06AC, def: 0x0000 },  /* R1708  - OUT3RMIX Input 3 Source */
    reg_default { reg: 0x06AD, def: 0x0080 },  /* R1709  - OUT3RMIX Input 3 Volume */
    reg_default { reg: 0x06AE, def: 0x0000 },  /* R1710  - OUT3RMIX Input 4 Source */
    reg_default { reg: 0x06AF, def: 0x0080 },  /* R1711  - OUT3RMIX Input 4 Volume */
    reg_default { reg: 0x06B0, def: 0x0000 },  /* R1712  - OUT4LMIX Input 1 Source */
    reg_default { reg: 0x06B1, def: 0x0080 },  /* R1713  - OUT4LMIX Input 1 Volume */
    reg_default { reg: 0x06B2, def: 0x0000 },  /* R1714  - OUT4LMIX Input 2 Source */
    reg_default { reg: 0x06B3, def: 0x0080 },  /* R1715  - OUT4LMIX Input 2 Volume */
    reg_default { reg: 0x06B4, def: 0x0000 },  /* R1716  - OUT4LMIX Input 3 Source */
    reg_default { reg: 0x06B5, def: 0x0080 },  /* R1717  - OUT4LMIX Input 3 Volume */
    reg_default { reg: 0x06B6, def: 0x0000 },  /* R1718  - OUT4LMIX Input 4 Source */
    reg_default { reg: 0x06B7, def: 0x0080 },  /* R1719  - OUT4LMIX Input 4 Volume */
    reg_default { reg: 0x06B8, def: 0x0000 },  /* R1720  - OUT4RMIX Input 1 Source */
    reg_default { reg: 0x06B9, def: 0x0080 },  /* R1721  - OUT4RMIX Input 1 Volume */
    reg_default { reg: 0x06BA, def: 0x0000 },  /* R1722  - OUT4RMIX Input 2 Source */
    reg_default { reg: 0x06BB, def: 0x0080 },  /* R1723  - OUT4RMIX Input 2 Volume */
    reg_default { reg: 0x06BC, def: 0x0000 },  /* R1724  - OUT4RMIX Input 3 Source */
    reg_default { reg: 0x06BD, def: 0x0080 },  /* R1725  - OUT4RMIX Input 3 Volume */
    reg_default { reg: 0x06BE, def: 0x0000 },  /* R1726  - OUT4RMIX Input 4 Source */
    reg_default { reg: 0x06BF, def: 0x0080 },  /* R1727  - OUT4RMIX Input 4 Volume */
    reg_default { reg: 0x06C0, def: 0x0000 },  /* R1728  - OUT5LMIX Input 1 Source */
    reg_default { reg: 0x06C1, def: 0x0080 },  /* R1729  - OUT5LMIX Input 1 Volume */
    reg_default { reg: 0x06C2, def: 0x0000 },  /* R1730  - OUT5LMIX Input 2 Source */
    reg_default { reg: 0x06C3, def: 0x0080 },  /* R1731  - OUT5LMIX Input 2 Volume */
    reg_default { reg: 0x06C4, def: 0x0000 },  /* R1732  - OUT5LMIX Input 3 Source */
    reg_default { reg: 0x06C5, def: 0x0080 },  /* R1733  - OUT5LMIX Input 3 Volume */
    reg_default { reg: 0x06C6, def: 0x0000 },  /* R1734  - OUT5LMIX Input 4 Source */
    reg_default { reg: 0x06C7, def: 0x0080 },  /* R1735  - OUT5LMIX Input 4 Volume */
    reg_default { reg: 0x06C8, def: 0x0000 },  /* R1736  - OUT5RMIX Input 1 Source */
    reg_default { reg: 0x06C9, def: 0x0080 },  /* R1737  - OUT5RMIX Input 1 Volume */
    reg_default { reg: 0x06CA, def: 0x0000 },  /* R1738  - OUT5RMIX Input 2 Source */
    reg_default { reg: 0x06CB, def: 0x0080 },  /* R1739  - OUT5RMIX Input 2 Volume */
    reg_default { reg: 0x06CC, def: 0x0000 },  /* R1740  - OUT5RMIX Input 3 Source */
    reg_default { reg: 0x06CD, def: 0x0080 },  /* R1741  - OUT5RMIX Input 3 Volume */
    reg_default { reg: 0x06CE, def: 0x0000 },  /* R1742  - OUT5RMIX Input 4 Source */
    reg_default { reg: 0x06CF, def: 0x0080 },  /* R1743  - OUT5RMIX Input 4 Volume */
    reg_default { reg: 0x06D0, def: 0x0000 },  /* R1744  - OUT6LMIX Input 1 Source */
    reg_default { reg: 0x06D1, def: 0x0080 },  /* R1745  - OUT6LMIX Input 1 Volume */
    reg_default { reg: 0x06D2, def: 0x0000 },  /* R1746  - OUT6LMIX Input 2 Source */
    reg_default { reg: 0x06D3, def: 0x0080 },  /* R1747  - OUT6LMIX Input 2 Volume */
    reg_default { reg: 0x06D4, def: 0x0000 },  /* R1748  - OUT6LMIX Input 3 Source */
    reg_default { reg: 0x06D5, def: 0x0080 },  /* R1749  - OUT6LMIX Input 3 Volume */
    reg_default { reg: 0x06D6, def: 0x0000 },  /* R1750  - OUT6LMIX Input 4 Source */
    reg_default { reg: 0x06D7, def: 0x0080 },  /* R1751  - OUT6LMIX Input 4 Volume */
    reg_default { reg: 0x06D8, def: 0x0000 },  /* R1752  - OUT6RMIX Input 1 Source */
    reg_default { reg: 0x06D9, def: 0x0080 },  /* R1753  - OUT6RMIX Input 1 Volume */
    reg_default { reg: 0x06DA, def: 0x0000 },  /* R1754  - OUT6RMIX Input 2 Source */
    reg_default { reg: 0x06DB, def: 0x0080 },  /* R1755  - OUT6RMIX Input 2 Volume */
    reg_default { reg: 0x06DC, def: 0x0000 },  /* R1756  - OUT6RMIX Input 3 Source */
    reg_default { reg: 0x06DD, def: 0x0080 },  /* R1757  - OUT6RMIX Input 3 Volume */
    reg_default { reg: 0x06DE, def: 0x0000 },  /* R1758  - OUT6RMIX Input 4 Source */
    reg_default { reg: 0x06DF, def: 0x0080 },  /* R1759  - OUT6RMIX Input 4 Volume */
    reg_default { reg: 0x0700, def: 0x0000 },  /* R1792  - AIF1TX1MIX Input 1 Source */
    reg_default { reg: 0x0701, def: 0x0080 },  /* R1793  - AIF1TX1MIX Input 1 Volume */
    reg_default { reg: 0x0702, def: 0x0000 },  /* R1794  - AIF1TX1MIX Input 2 Source */
    reg_default { reg: 0x0703, def: 0x0080 },  /* R1795  - AIF1TX1MIX Input 2 Volume */
    reg_default { reg: 0x0704, def: 0x0000 },  /* R1796  - AIF1TX1MIX Input 3 Source */
    reg_default { reg: 0x0705, def: 0x0080 },  /* R1797  - AIF1TX1MIX Input 3 Volume */
    reg_default { reg: 0x0706, def: 0x0000 },  /* R1798  - AIF1TX1MIX Input 4 Source */
    reg_default { reg: 0x0707, def: 0x0080 },  /* R1799  - AIF1TX1MIX Input 4 Volume */
    reg_default { reg: 0x0708, def: 0x0000 },  /* R1800  - AIF1TX2MIX Input 1 Source */
    reg_default { reg: 0x0709, def: 0x0080 },  /* R1801  - AIF1TX2MIX Input 1 Volume */
    reg_default { reg: 0x070A, def: 0x0000 },  /* R1802  - AIF1TX2MIX Input 2 Source */
    reg_default { reg: 0x070B, def: 0x0080 },  /* R1803  - AIF1TX2MIX Input 2 Volume */
    reg_default { reg: 0x070C, def: 0x0000 },  /* R1804  - AIF1TX2MIX Input 3 Source */
    reg_default { reg: 0x070D, def: 0x0080 },  /* R1805  - AIF1TX2MIX Input 3 Volume */
    reg_default { reg: 0x070E, def: 0x0000 },  /* R1806  - AIF1TX2MIX Input 4 Source */
    reg_default { reg: 0x070F, def: 0x0080 },  /* R1807  - AIF1TX2MIX Input 4 Volume */
    reg_default { reg: 0x0710, def: 0x0000 },  /* R1808  - AIF1TX3MIX Input 1 Source */
    reg_default { reg: 0x0711, def: 0x0080 },  /* R1809  - AIF1TX3MIX Input 1 Volume */
    reg_default { reg: 0x0712, def: 0x0000 },  /* R1810  - AIF1TX3MIX Input 2 Source */
    reg_default { reg: 0x0713, def: 0x0080 },  /* R1811  - AIF1TX3MIX Input 2 Volume */
    reg_default { reg: 0x0714, def: 0x0000 },  /* R1812  - AIF1TX3MIX Input 3 Source */
    reg_default { reg: 0x0715, def: 0x0080 },  /* R1813  - AIF1TX3MIX Input 3 Volume */
    reg_default { reg: 0x0716, def: 0x0000 },  /* R1814  - AIF1TX3MIX Input 4 Source */
    reg_default { reg: 0x0717, def: 0x0080 },  /* R1815  - AIF1TX3MIX Input 4 Volume */
    reg_default { reg: 0x0718, def: 0x0000 },  /* R1816  - AIF1TX4MIX Input 1 Source */
    reg_default { reg: 0x0719, def: 0x0080 },  /* R1817  - AIF1TX4MIX Input 1 Volume */
    reg_default { reg: 0x071A, def: 0x0000 },  /* R1818  - AIF1TX4MIX Input 2 Source */
    reg_default { reg: 0x071B, def: 0x0080 },  /* R1819  - AIF1TX4MIX Input 2 Volume */
    reg_default { reg: 0x071C, def: 0x0000 },  /* R1820  - AIF1TX4MIX Input 3 Source */
    reg_default { reg: 0x071D, def: 0x0080 },  /* R1821  - AIF1TX4MIX Input 3 Volume */
    reg_default { reg: 0x071E, def: 0x0000 },  /* R1822  - AIF1TX4MIX Input 4 Source */
    reg_default { reg: 0x071F, def: 0x0080 },  /* R1823  - AIF1TX4MIX Input 4 Volume */
    reg_default { reg: 0x0720, def: 0x0000 },  /* R1824  - AIF1TX5MIX Input 1 Source */
    reg_default { reg: 0x0721, def: 0x0080 },  /* R1825  - AIF1TX5MIX Input 1 Volume */
    reg_default { reg: 0x0722, def: 0x0000 },  /* R1826  - AIF1TX5MIX Input 2 Source */
    reg_default { reg: 0x0723, def: 0x0080 },  /* R1827  - AIF1TX5MIX Input 2 Volume */
    reg_default { reg: 0x0724, def: 0x0000 },  /* R1828  - AIF1TX5MIX Input 3 Source */
    reg_default { reg: 0x0725, def: 0x0080 },  /* R1829  - AIF1TX5MIX Input 3 Volume */
    reg_default { reg: 0x0726, def: 0x0000 },  /* R1830  - AIF1TX5MIX Input 4 Source */
    reg_default { reg: 0x0727, def: 0x0080 },  /* R1831  - AIF1TX5MIX Input 4 Volume */
    reg_default { reg: 0x0728, def: 0x0000 },  /* R1832  - AIF1TX6MIX Input 1 Source */
    reg_default { reg: 0x0729, def: 0x0080 },  /* R1833  - AIF1TX6MIX Input 1 Volume */
    reg_default { reg: 0x072A, def: 0x0000 },  /* R1834  - AIF1TX6MIX Input 2 Source */
    reg_default { reg: 0x072B, def: 0x0080 },  /* R1835  - AIF1TX6MIX Input 2 Volume */
    reg_default { reg: 0x072C, def: 0x0000 },  /* R1836  - AIF1TX6MIX Input 3 Source */
    reg_default { reg: 0x072D, def: 0x0080 },  /* R1837  - AIF1TX6MIX Input 3 Volume */
    reg_default { reg: 0x072E, def: 0x0000 },  /* R1838  - AIF1TX6MIX Input 4 Source */
    reg_default { reg: 0x072F, def: 0x0080 },  /* R1839  - AIF1TX6MIX Input 4 Volume */
    reg_default { reg: 0x0730, def: 0x0000 },  /* R1840  - AIF1TX7MIX Input 1 Source */
    reg_default { reg: 0x0731, def: 0x0080 },  /* R1841  - AIF1TX7MIX Input 1 Volume */
    reg_default { reg: 0x0732, def: 0x0000 },  /* R1842  - AIF1TX7MIX Input 2 Source */
    reg_default { reg: 0x0733, def: 0x0080 },  /* R1843  - AIF1TX7MIX Input 2 Volume */
    reg_default { reg: 0x0734, def: 0x0000 },  /* R1844  - AIF1TX7MIX Input 3 Source */
    reg_default { reg: 0x0735, def: 0x0080 },  /* R1845  - AIF1TX7MIX Input 3 Volume */
    reg_default { reg: 0x0736, def: 0x0000 },  /* R1846  - AIF1TX7MIX Input 4 Source */
    reg_default { reg: 0x0737, def: 0x0080 },  /* R1847  - AIF1TX7MIX Input 4 Volume */
    reg_default { reg: 0x0738, def: 0x0000 },  /* R1848  - AIF1TX8MIX Input 1 Source */
    reg_default { reg: 0x0739, def: 0x0080 },  /* R1849  - AIF1TX8MIX Input 1 Volume */
    reg_default { reg: 0x073A, def: 0x0000 },  /* R1850  - AIF1TX8MIX Input 2 Source */
    reg_default { reg: 0x073B, def: 0x0080 },  /* R1851  - AIF1TX8MIX Input 2 Volume */
    reg_default { reg: 0x073C, def: 0x0000 },  /* R1852  - AIF1TX8MIX Input 3 Source */
    reg_default { reg: 0x073D, def: 0x0080 },  /* R1853  - AIF1TX8MIX Input 3 Volume */
    reg_default { reg: 0x073E, def: 0x0000 },  /* R1854  - AIF1TX8MIX Input 4 Source */
    reg_default { reg: 0x073F, def: 0x0080 },  /* R1855  - AIF1TX8MIX Input 4 Volume */
    reg_default { reg: 0x0740, def: 0x0000 },  /* R1856  - AIF2TX1MIX Input 1 Source */
    reg_default { reg: 0x0741, def: 0x0080 },  /* R1857  - AIF2TX1MIX Input 1 Volume */
    reg_default { reg: 0x0742, def: 0x0000 },  /* R1858  - AIF2TX1MIX Input 2 Source */
    reg_default { reg: 0x0743, def: 0x0080 },  /* R1859  - AIF2TX1MIX Input 2 Volume */
    reg_default { reg: 0x0744, def: 0x0000 },  /* R1860  - AIF2TX1MIX Input 3 Source */
    reg_default { reg: 0x0745, def: 0x0080 },  /* R1861  - AIF2TX1MIX Input 3 Volume */
    reg_default { reg: 0x0746, def: 0x0000 },  /* R1862  - AIF2TX1MIX Input 4 Source */
    reg_default { reg: 0x0747, def: 0x0080 },  /* R1863  - AIF2TX1MIX Input 4 Volume */
    reg_default { reg: 0x0748, def: 0x0000 },  /* R1864  - AIF2TX2MIX Input 1 Source */
    reg_default { reg: 0x0749, def: 0x0080 },  /* R1865  - AIF2TX2MIX Input 1 Volume */
    reg_default { reg: 0x074A, def: 0x0000 },  /* R1866  - AIF2TX2MIX Input 2 Source */
    reg_default { reg: 0x074B, def: 0x0080 },  /* R1867  - AIF2TX2MIX Input 2 Volume */
    reg_default { reg: 0x074C, def: 0x0000 },  /* R1868  - AIF2TX2MIX Input 3 Source */
    reg_default { reg: 0x074D, def: 0x0080 },  /* R1869  - AIF2TX2MIX Input 3 Volume */
    reg_default { reg: 0x074E, def: 0x0000 },  /* R1870  - AIF2TX2MIX Input 4 Source */
    reg_default { reg: 0x074F, def: 0x0080 },  /* R1871  - AIF2TX2MIX Input 4 Volume */
    reg_default { reg: 0x0780, def: 0x0000 },  /* R1920  - AIF3TX1MIX Input 1 Source */
    reg_default { reg: 0x0781, def: 0x0080 },  /* R1921  - AIF3TX1MIX Input 1 Volume */
    reg_default { reg: 0x0782, def: 0x0000 },  /* R1922  - AIF3TX1MIX Input 2 Source */
    reg_default { reg: 0x0783, def: 0x0080 },  /* R1923  - AIF3TX1MIX Input 2 Volume */
    reg_default { reg: 0x0784, def: 0x0000 },  /* R1924  - AIF3TX1MIX Input 3 Source */
    reg_default { reg: 0x0785, def: 0x0080 },  /* R1925  - AIF3TX1MIX Input 3 Volume */
    reg_default { reg: 0x0786, def: 0x0000 },  /* R1926  - AIF3TX1MIX Input 4 Source */
    reg_default { reg: 0x0787, def: 0x0080 },  /* R1927  - AIF3TX1MIX Input 4 Volume */
    reg_default { reg: 0x0788, def: 0x0000 },  /* R1928  - AIF3TX2MIX Input 1 Source */
    reg_default { reg: 0x0789, def: 0x0080 },  /* R1929  - AIF3TX2MIX Input 1 Volume */
    reg_default { reg: 0x078A, def: 0x0000 },  /* R1930  - AIF3TX2MIX Input 2 Source */
    reg_default { reg: 0x078B, def: 0x0080 },  /* R1931  - AIF3TX2MIX Input 2 Volume */
    reg_default { reg: 0x078C, def: 0x0000 },  /* R1932  - AIF3TX2MIX Input 3 Source */
    reg_default { reg: 0x078D, def: 0x0080 },  /* R1933  - AIF3TX2MIX Input 3 Volume */
    reg_default { reg: 0x078E, def: 0x0000 },  /* R1934  - AIF3TX2MIX Input 4 Source */
    reg_default { reg: 0x078F, def: 0x0080 },  /* R1935  - AIF3TX2MIX Input 4 Volume */
    reg_default { reg: 0x0880, def: 0x0000 },  /* R2176  - EQ1MIX Input 1 Source */
    reg_default { reg: 0x0881, def: 0x0080 },  /* R2177  - EQ1MIX Input 1 Volume */
    reg_default { reg: 0x0882, def: 0x0000 },  /* R2178  - EQ1MIX Input 2 Source */
    reg_default { reg: 0x0883, def: 0x0080 },  /* R2179  - EQ1MIX Input 2 Volume */
    reg_default { reg: 0x0884, def: 0x0000 },  /* R2180  - EQ1MIX Input 3 Source */
    reg_default { reg: 0x0885, def: 0x0080 },  /* R2181  - EQ1MIX Input 3 Volume */
    reg_default { reg: 0x0886, def: 0x0000 },  /* R2182  - EQ1MIX Input 4 Source */
    reg_default { reg: 0x0887, def: 0x0080 },  /* R2183  - EQ1MIX Input 4 Volume */
    reg_default { reg: 0x0888, def: 0x0000 },  /* R2184  - EQ2MIX Input 1 Source */
    reg_default { reg: 0x0889, def: 0x0080 },  /* R2185  - EQ2MIX Input 1 Volume */
    reg_default { reg: 0x088A, def: 0x0000 },  /* R2186  - EQ2MIX Input 2 Source */
    reg_default { reg: 0x088B, def: 0x0080 },  /* R2187  - EQ2MIX Input 2 Volume */
    reg_default { reg: 0x088C, def: 0x0000 },  /* R2188  - EQ2MIX Input 3 Source */
    reg_default { reg: 0x088D, def: 0x0080 },  /* R2189  - EQ2MIX Input 3 Volume */
    reg_default { reg: 0x088E, def: 0x0000 },  /* R2190  - EQ2MIX Input 4 Source */
    reg_default { reg: 0x088F, def: 0x0080 },  /* R2191  - EQ2MIX Input 4 Volume */
    reg_default { reg: 0x0890, def: 0x0000 },  /* R2192  - EQ3MIX Input 1 Source */
    reg_default { reg: 0x0891, def: 0x0080 },  /* R2193  - EQ3MIX Input 1 Volume */
    reg_default { reg: 0x0892, def: 0x0000 },  /* R2194  - EQ3MIX Input 2 Source */
    reg_default { reg: 0x0893, def: 0x0080 },  /* R2195  - EQ3MIX Input 2 Volume */
    reg_default { reg: 0x0894, def: 0x0000 },  /* R2196  - EQ3MIX Input 3 Source */
    reg_default { reg: 0x0895, def: 0x0080 },  /* R2197  - EQ3MIX Input 3 Volume */
    reg_default { reg: 0x0896, def: 0x0000 },  /* R2198  - EQ3MIX Input 4 Source */
    reg_default { reg: 0x0897, def: 0x0080 },  /* R2199  - EQ3MIX Input 4 Volume */
    reg_default { reg: 0x0898, def: 0x0000 },  /* R2200  - EQ4MIX Input 1 Source */
    reg_default { reg: 0x0899, def: 0x0080 },  /* R2201  - EQ4MIX Input 1 Volume */
    reg_default { reg: 0x089A, def: 0x0000 },  /* R2202  - EQ4MIX Input 2 Source */
    reg_default { reg: 0x089B, def: 0x0080 },  /* R2203  - EQ4MIX Input 2 Volume */
    reg_default { reg: 0x089C, def: 0x0000 },  /* R2204  - EQ4MIX Input 3 Source */
    reg_default { reg: 0x089D, def: 0x0080 },  /* R2205  - EQ4MIX Input 3 Volume */
    reg_default { reg: 0x089E, def: 0x0000 },  /* R2206  - EQ4MIX Input 4 Source */
    reg_default { reg: 0x089F, def: 0x0080 },  /* R2207  - EQ4MIX Input 4 Volume */
    reg_default { reg: 0x08C0, def: 0x0000 },  /* R2240  - DRC1LMIX Input 1 Source */
    reg_default { reg: 0x08C1, def: 0x0080 },  /* R2241  - DRC1LMIX Input 1 Volume */
    reg_default { reg: 0x08C2, def: 0x0000 },  /* R2242  - DRC1LMIX Input 2 Source */
    reg_default { reg: 0x08C3, def: 0x0080 },  /* R2243  - DRC1LMIX Input 2 Volume */
    reg_default { reg: 0x08C4, def: 0x0000 },  /* R2244  - DRC1LMIX Input 3 Source */
    reg_default { reg: 0x08C5, def: 0x0080 },  /* R2245  - DRC1LMIX Input 3 Volume */
    reg_default { reg: 0x08C6, def: 0x0000 },  /* R2246  - DRC1LMIX Input 4 Source */
    reg_default { reg: 0x08C7, def: 0x0080 },  /* R2247  - DRC1LMIX Input 4 Volume */
    reg_default { reg: 0x08C8, def: 0x0000 },  /* R2248  - DRC1RMIX Input 1 Source */
    reg_default { reg: 0x08C9, def: 0x0080 },  /* R2249  - DRC1RMIX Input 1 Volume */
    reg_default { reg: 0x08CA, def: 0x0000 },  /* R2250  - DRC1RMIX Input 2 Source */
    reg_default { reg: 0x08CB, def: 0x0080 },  /* R2251  - DRC1RMIX Input 2 Volume */
    reg_default { reg: 0x08CC, def: 0x0000 },  /* R2252  - DRC1RMIX Input 3 Source */
    reg_default { reg: 0x08CD, def: 0x0080 },  /* R2253  - DRC1RMIX Input 3 Volume */
    reg_default { reg: 0x08CE, def: 0x0000 },  /* R2254  - DRC1RMIX Input 4 Source */
    reg_default { reg: 0x08CF, def: 0x0080 },  /* R2255  - DRC1RMIX Input 4 Volume */
    reg_default { reg: 0x0900, def: 0x0000 },  /* R2304  - HPLP1MIX Input 1 Source */
    reg_default { reg: 0x0901, def: 0x0080 },  /* R2305  - HPLP1MIX Input 1 Volume */
    reg_default { reg: 0x0902, def: 0x0000 },  /* R2306  - HPLP1MIX Input 2 Source */
    reg_default { reg: 0x0903, def: 0x0080 },  /* R2307  - HPLP1MIX Input 2 Volume */
    reg_default { reg: 0x0904, def: 0x0000 },  /* R2308  - HPLP1MIX Input 3 Source */
    reg_default { reg: 0x0905, def: 0x0080 },  /* R2309  - HPLP1MIX Input 3 Volume */
    reg_default { reg: 0x0906, def: 0x0000 },  /* R2310  - HPLP1MIX Input 4 Source */
    reg_default { reg: 0x0907, def: 0x0080 },  /* R2311  - HPLP1MIX Input 4 Volume */
    reg_default { reg: 0x0908, def: 0x0000 },  /* R2312  - HPLP2MIX Input 1 Source */
    reg_default { reg: 0x0909, def: 0x0080 },  /* R2313  - HPLP2MIX Input 1 Volume */
    reg_default { reg: 0x090A, def: 0x0000 },  /* R2314  - HPLP2MIX Input 2 Source */
    reg_default { reg: 0x090B, def: 0x0080 },  /* R2315  - HPLP2MIX Input 2 Volume */
    reg_default { reg: 0x090C, def: 0x0000 },  /* R2316  - HPLP2MIX Input 3 Source */
    reg_default { reg: 0x090D, def: 0x0080 },  /* R2317  - HPLP2MIX Input 3 Volume */
    reg_default { reg: 0x090E, def: 0x0000 },  /* R2318  - HPLP2MIX Input 4 Source */
    reg_default { reg: 0x090F, def: 0x0080 },  /* R2319  - HPLP2MIX Input 4 Volume */
    reg_default { reg: 0x0910, def: 0x0000 },  /* R2320  - HPLP3MIX Input 1 Source */
    reg_default { reg: 0x0911, def: 0x0080 },  /* R2321  - HPLP3MIX Input 1 Volume */
    reg_default { reg: 0x0912, def: 0x0000 },  /* R2322  - HPLP3MIX Input 2 Source */
    reg_default { reg: 0x0913, def: 0x0080 },  /* R2323  - HPLP3MIX Input 2 Volume */
    reg_default { reg: 0x0914, def: 0x0000 },  /* R2324  - HPLP3MIX Input 3 Source */
    reg_default { reg: 0x0915, def: 0x0080 },  /* R2325  - HPLP3MIX Input 3 Volume */
    reg_default { reg: 0x0916, def: 0x0000 },  /* R2326  - HPLP3MIX Input 4 Source */
    reg_default { reg: 0x0917, def: 0x0080 },  /* R2327  - HPLP3MIX Input 4 Volume */
    reg_default { reg: 0x0918, def: 0x0000 },  /* R2328  - HPLP4MIX Input 1 Source */
    reg_default { reg: 0x0919, def: 0x0080 },  /* R2329  - HPLP4MIX Input 1 Volume */
    reg_default { reg: 0x091A, def: 0x0000 },  /* R2330  - HPLP4MIX Input 2 Source */
    reg_default { reg: 0x091B, def: 0x0080 },  /* R2331  - HPLP4MIX Input 2 Volume */
    reg_default { reg: 0x091C, def: 0x0000 },  /* R2332  - HPLP4MIX Input 3 Source */
    reg_default { reg: 0x091D, def: 0x0080 },  /* R2333  - HPLP4MIX Input 3 Volume */
    reg_default { reg: 0x091E, def: 0x0000 },  /* R2334  - HPLP4MIX Input 4 Source */
    reg_default { reg: 0x091F, def: 0x0080 },  /* R2335  - HPLP4MIX Input 4 Volume */
    reg_default { reg: 0x0940, def: 0x0000 },  /* R2368  - DSP1LMIX Input 1 Source */
    reg_default { reg: 0x0941, def: 0x0080 },  /* R2369  - DSP1LMIX Input 1 Volume */
    reg_default { reg: 0x0942, def: 0x0000 },  /* R2370  - DSP1LMIX Input 2 Source */
    reg_default { reg: 0x0943, def: 0x0080 },  /* R2371  - DSP1LMIX Input 2 Volume */
    reg_default { reg: 0x0944, def: 0x0000 },  /* R2372  - DSP1LMIX Input 3 Source */
    reg_default { reg: 0x0945, def: 0x0080 },  /* R2373  - DSP1LMIX Input 3 Volume */
    reg_default { reg: 0x0946, def: 0x0000 },  /* R2374  - DSP1LMIX Input 4 Source */
    reg_default { reg: 0x0947, def: 0x0080 },  /* R2375  - DSP1LMIX Input 4 Volume */
    reg_default { reg: 0x0948, def: 0x0000 },  /* R2376  - DSP1RMIX Input 1 Source */
    reg_default { reg: 0x0949, def: 0x0080 },  /* R2377  - DSP1RMIX Input 1 Volume */
    reg_default { reg: 0x094A, def: 0x0000 },  /* R2378  - DSP1RMIX Input 2 Source */
    reg_default { reg: 0x094B, def: 0x0080 },  /* R2379  - DSP1RMIX Input 2 Volume */
    reg_default { reg: 0x094C, def: 0x0000 },  /* R2380  - DSP1RMIX Input 3 Source */
    reg_default { reg: 0x094D, def: 0x0080 },  /* R2381  - DSP1RMIX Input 3 Volume */
    reg_default { reg: 0x094E, def: 0x0000 },  /* R2382  - DSP1RMIX Input 4 Source */
    reg_default { reg: 0x094F, def: 0x0080 },  /* R2383  - DSP1RMIX Input 4 Volume */
    reg_default { reg: 0x0950, def: 0x0000 },  /* R2384  - DSP1AUX1MIX Input 1 Source */
    reg_default { reg: 0x0958, def: 0x0000 },  /* R2392  - DSP1AUX2MIX Input 1 Source */
    reg_default { reg: 0x0960, def: 0x0000 },  /* R2400  - DSP1AUX3MIX Input 1 Source */
    reg_default { reg: 0x0968, def: 0x0000 },  /* R2408  - DSP1AUX4MIX Input 1 Source */
    reg_default { reg: 0x0970, def: 0x0000 },  /* R2416  - DSP1AUX5MIX Input 1 Source */
    reg_default { reg: 0x0978, def: 0x0000 },  /* R2424  - DSP1AUX6MIX Input 1 Source */
    reg_default { reg: 0x0980, def: 0x0000 },  /* R2432  - DSP2LMIX Input 1 Source */
    reg_default { reg: 0x0981, def: 0x0080 },  /* R2433  - DSP2LMIX Input 1 Volume */
    reg_default { reg: 0x0982, def: 0x0000 },  /* R2434  - DSP2LMIX Input 2 Source */
    reg_default { reg: 0x0983, def: 0x0080 },  /* R2435  - DSP2LMIX Input 2 Volume */
    reg_default { reg: 0x0984, def: 0x0000 },  /* R2436  - DSP2LMIX Input 3 Source */
    reg_default { reg: 0x0985, def: 0x0080 },  /* R2437  - DSP2LMIX Input 3 Volume */
    reg_default { reg: 0x0986, def: 0x0000 },  /* R2438  - DSP2LMIX Input 4 Source */
    reg_default { reg: 0x0987, def: 0x0080 },  /* R2439  - DSP2LMIX Input 4 Volume */
    reg_default { reg: 0x0988, def: 0x0000 },  /* R2440  - DSP2RMIX Input 1 Source */
    reg_default { reg: 0x0989, def: 0x0080 },  /* R2441  - DSP2RMIX Input 1 Volume */
    reg_default { reg: 0x098A, def: 0x0000 },  /* R2442  - DSP2RMIX Input 2 Source */
    reg_default { reg: 0x098B, def: 0x0080 },  /* R2443  - DSP2RMIX Input 2 Volume */
    reg_default { reg: 0x098C, def: 0x0000 },  /* R2444  - DSP2RMIX Input 3 Source */
    reg_default { reg: 0x098D, def: 0x0080 },  /* R2445  - DSP2RMIX Input 3 Volume */
    reg_default { reg: 0x098E, def: 0x0000 },  /* R2446  - DSP2RMIX Input 4 Source */
    reg_default { reg: 0x098F, def: 0x0080 },  /* R2447  - DSP2RMIX Input 4 Volume */
    reg_default { reg: 0x0990, def: 0x0000 },  /* R2448  - DSP2AUX1MIX Input 1 Source */
    reg_default { reg: 0x0998, def: 0x0000 },  /* R2456  - DSP2AUX2MIX Input 1 Source */
    reg_default { reg: 0x09A0, def: 0x0000 },  /* R2464  - DSP2AUX3MIX Input 1 Source */
    reg_default { reg: 0x09A8, def: 0x0000 },  /* R2472  - DSP2AUX4MIX Input 1 Source */
    reg_default { reg: 0x09B0, def: 0x0000 },  /* R2480  - DSP2AUX5MIX Input 1 Source */
    reg_default { reg: 0x09B8, def: 0x0000 },  /* R2488  - DSP2AUX6MIX Input 1 Source */
    reg_default { reg: 0x09C0, def: 0x0000 },  /* R2496  - DSP3LMIX Input 1 Source */
    reg_default { reg: 0x09C1, def: 0x0080 },  /* R2497  - DSP3LMIX Input 1 Volume */
    reg_default { reg: 0x09C2, def: 0x0000 },  /* R2498  - DSP3LMIX Input 2 Source */
    reg_default { reg: 0x09C3, def: 0x0080 },  /* R2499  - DSP3LMIX Input 2 Volume */
    reg_default { reg: 0x09C4, def: 0x0000 },  /* R2500  - DSP3LMIX Input 3 Source */
    reg_default { reg: 0x09C5, def: 0x0080 },  /* R2501  - DSP3LMIX Input 3 Volume */
    reg_default { reg: 0x09C6, def: 0x0000 },  /* R2502  - DSP3LMIX Input 4 Source */
    reg_default { reg: 0x09C7, def: 0x0080 },  /* R2503  - DSP3LMIX Input 4 Volume */
    reg_default { reg: 0x09C8, def: 0x0000 },  /* R2504  - DSP3RMIX Input 1 Source */
    reg_default { reg: 0x09C9, def: 0x0080 },  /* R2505  - DSP3RMIX Input 1 Volume */
    reg_default { reg: 0x09CA, def: 0x0000 },  /* R2506  - DSP3RMIX Input 2 Source */
    reg_default { reg: 0x09CB, def: 0x0080 },  /* R2507  - DSP3RMIX Input 2 Volume */
    reg_default { reg: 0x09CC, def: 0x0000 },  /* R2508  - DSP3RMIX Input 3 Source */
    reg_default { reg: 0x09CD, def: 0x0080 },  /* R2509  - DSP3RMIX Input 3 Volume */
    reg_default { reg: 0x09CE, def: 0x0000 },  /* R2510  - DSP3RMIX Input 4 Source */
    reg_default { reg: 0x09CF, def: 0x0080 },  /* R2511  - DSP3RMIX Input 4 Volume */
    reg_default { reg: 0x09D0, def: 0x0000 },  /* R2512  - DSP3AUX1MIX Input 1 Source */
    reg_default { reg: 0x09D8, def: 0x0000 },  /* R2520  - DSP3AUX2MIX Input 1 Source */
    reg_default { reg: 0x09E0, def: 0x0000 },  /* R2528  - DSP3AUX3MIX Input 1 Source */
    reg_default { reg: 0x09E8, def: 0x0000 },  /* R2536  - DSP3AUX4MIX Input 1 Source */
    reg_default { reg: 0x09F0, def: 0x0000 },  /* R2544  - DSP3AUX5MIX Input 1 Source */
    reg_default { reg: 0x09F8, def: 0x0000 },  /* R2552  - DSP3AUX6MIX Input 1 Source */
    reg_default { reg: 0x0A80, def: 0x0000 },  /* R2688  - ASRC1LMIX Input 1 Source */
    reg_default { reg: 0x0A88, def: 0x0000 },  /* R2696  - ASRC1RMIX Input 1 Source */
    reg_default { reg: 0x0A90, def: 0x0000 },  /* R2704  - ASRC2LMIX Input 1 Source */
    reg_default { reg: 0x0A98, def: 0x0000 },  /* R2712  - ASRC2RMIX Input 1 Source */
    reg_default { reg: 0x0B00, def: 0x0000 },  /* R2816  - ISRC1DEC1MIX Input 1 Source */
    reg_default { reg: 0x0B08, def: 0x0000 },  /* R2824  - ISRC1DEC2MIX Input 1 Source */
    reg_default { reg: 0x0B10, def: 0x0000 },  /* R2832  - ISRC1DEC3MIX Input 1 Source */
    reg_default { reg: 0x0B18, def: 0x0000 },  /* R2840  - ISRC1DEC4MIX Input 1 Source */
    reg_default { reg: 0x0B20, def: 0x0000 },  /* R2848  - ISRC1INT1MIX Input 1 Source */
    reg_default { reg: 0x0B28, def: 0x0000 },  /* R2856  - ISRC1INT2MIX Input 1 Source */
    reg_default { reg: 0x0B30, def: 0x0000 },  /* R2864  - ISRC1INT3MIX Input 1 Source */
    reg_default { reg: 0x0B38, def: 0x0000 },  /* R2872  - ISRC1INT4MIX Input 1 Source */
    reg_default { reg: 0x0B40, def: 0x0000 },  /* R2880  - ISRC2DEC1MIX Input 1 Source */
    reg_default { reg: 0x0B48, def: 0x0000 },  /* R2888  - ISRC2DEC2MIX Input 1 Source */
    reg_default { reg: 0x0B50, def: 0x0000 },  /* R2896  - ISRC2DEC3MIX Input 1 Source */
    reg_default { reg: 0x0B58, def: 0x0000 },  /* R2904  - ISRC2DEC4MIX Input 1 Source */
    reg_default { reg: 0x0B60, def: 0x0000 },  /* R2912  - ISRC2INT1MIX Input 1 Source */
    reg_default { reg: 0x0B68, def: 0x0000 },  /* R2920  - ISRC2INT2MIX Input 1 Source */
    reg_default { reg: 0x0B70, def: 0x0000 },  /* R2928  - ISRC2INT3MIX Input 1 Source */
    reg_default { reg: 0x0B78, def: 0x0000 },  /* R2936  - ISRC2INT4MIX Input 1 Source */
    reg_default { reg: 0x0C00, def: 0xA001 },  /* R3072  - GPIO CTRL 1 */
    reg_default { reg: 0x0C01, def: 0xA001 },  /* R3073  - GPIO CTRL 2 */
    reg_default { reg: 0x0C02, def: 0xA001 },  /* R3074  - GPIO CTRL 3 */
    reg_default { reg: 0x0C03, def: 0xA001 },  /* R3075  - GPIO CTRL 4 */
    reg_default { reg: 0x0C04, def: 0xA001 },  /* R3076  - GPIO CTRL 5 */
    reg_default { reg: 0x0C05, def: 0xA001 },  /* R3077  - GPIO CTRL 6 */
    reg_default { reg: 0x0C23, def: 0x4003 },  /* R3107  - Misc Pad Ctrl 1 */
    reg_default { reg: 0x0C24, def: 0x0000 },  /* R3108  - Misc Pad Ctrl 2 */
    reg_default { reg: 0x0C25, def: 0x0000 },  /* R3109  - Misc Pad Ctrl 3 */
    reg_default { reg: 0x0C26, def: 0x0000 },  /* R3110  - Misc Pad Ctrl 4 */
    reg_default { reg: 0x0C27, def: 0x0000 },  /* R3111  - Misc Pad Ctrl 5 */
    reg_default { reg: 0x0C28, def: 0x0000 },  /* R3112  - Misc GPIO 1 */
    reg_default { reg: 0x0D00, def: 0x0000 },  /* R3328  - Interrupt Status 1 */
    reg_default { reg: 0x0D01, def: 0x0000 },  /* R3329  - Interrupt Status 2 */
    reg_default { reg: 0x0D02, def: 0x0000 },  /* R3330  - Interrupt Status 3 */
    reg_default { reg: 0x0D03, def: 0x0000 },  /* R3331  - Interrupt Status 4 */
    reg_default { reg: 0x0D04, def: 0x0000 },  /* R3332  - Interrupt Raw Status 2 */
    reg_default { reg: 0x0D05, def: 0x0000 },  /* R3333  - Interrupt Raw Status 3 */
    reg_default { reg: 0x0D06, def: 0x0000 },  /* R3334  - Interrupt Raw Status 4 */
    reg_default { reg: 0x0D07, def: 0xFFFF },  /* R3335  - Interrupt Status 1 Mask */
    reg_default { reg: 0x0D08, def: 0xFFFF },  /* R3336  - Interrupt Status 2 Mask */
    reg_default { reg: 0x0D09, def: 0xFFFF },  /* R3337  - Interrupt Status 3 Mask */
    reg_default { reg: 0x0D0A, def: 0xFFFF },  /* R3338  - Interrupt Status 4 Mask */
    reg_default { reg: 0x0D1F, def: 0x0000 },  /* R3359  - Interrupt Control */
    reg_default { reg: 0x0D20, def: 0xFFFF },  /* R3360  - IRQ Debounce 1 */
    reg_default { reg: 0x0D21, def: 0xFFFF },  /* R3361  - IRQ Debounce 2 */
    reg_default { reg: 0x0E00, def: 0x0000 },  /* R3584  - FX_Ctrl */
    reg_default { reg: 0x0E10, def: 0x6318 },  /* R3600  - EQ1_1 */
    reg_default { reg: 0x0E11, def: 0x6300 },  /* R3601  - EQ1_2 */
    reg_default { reg: 0x0E12, def: 0x0FC8 },  /* R3602  - EQ1_3 */
    reg_default { reg: 0x0E13, def: 0x03FE },  /* R3603  - EQ1_4 */
    reg_default { reg: 0x0E14, def: 0x00E0 },  /* R3604  - EQ1_5 */
    reg_default { reg: 0x0E15, def: 0x1EC4 },  /* R3605  - EQ1_6 */
    reg_default { reg: 0x0E16, def: 0xF136 },  /* R3606  - EQ1_7 */
    reg_default { reg: 0x0E17, def: 0x0409 },  /* R3607  - EQ1_8 */
    reg_default { reg: 0x0E18, def: 0x04CC },  /* R3608  - EQ1_9 */
    reg_default { reg: 0x0E19, def: 0x1C9B },  /* R3609  - EQ1_10 */
    reg_default { reg: 0x0E1A, def: 0xF337 },  /* R3610  - EQ1_11 */
    reg_default { reg: 0x0E1B, def: 0x040B },  /* R3611  - EQ1_12 */
    reg_default { reg: 0x0E1C, def: 0x0CBB },  /* R3612  - EQ1_13 */
    reg_default { reg: 0x0E1D, def: 0x16F8 },  /* R3613  - EQ1_14 */
    reg_default { reg: 0x0E1E, def: 0xF7D9 },  /* R3614  - EQ1_15 */
    reg_default { reg: 0x0E1F, def: 0x040A },  /* R3615  - EQ1_16 */
    reg_default { reg: 0x0E20, def: 0x1F14 },  /* R3616  - EQ1_17 */
    reg_default { reg: 0x0E21, def: 0x058C },  /* R3617  - EQ1_18 */
    reg_default { reg: 0x0E22, def: 0x0563 },  /* R3618  - EQ1_19 */
    reg_default { reg: 0x0E23, def: 0x4000 },  /* R3619  - EQ1_20 */
    reg_default { reg: 0x0E26, def: 0x6318 },  /* R3622  - EQ2_1 */
    reg_default { reg: 0x0E27, def: 0x6300 },  /* R3623  - EQ2_2 */
    reg_default { reg: 0x0E28, def: 0x0FC8 },  /* R3624  - EQ2_3 */
    reg_default { reg: 0x0E29, def: 0x03FE },  /* R3625  - EQ2_4 */
    reg_default { reg: 0x0E2A, def: 0x00E0 },  /* R3626  - EQ2_5 */
    reg_default { reg: 0x0E2B, def: 0x1EC4 },  /* R3627  - EQ2_6 */
    reg_default { reg: 0x0E2C, def: 0xF136 },  /* R3628  - EQ2_7 */
    reg_default { reg: 0x0E2D, def: 0x0409 },  /* R3629  - EQ2_8 */
    reg_default { reg: 0x0E2E, def: 0x04CC },  /* R3630  - EQ2_9 */
    reg_default { reg: 0x0E2F, def: 0x1C9B },  /* R3631  - EQ2_10 */
    reg_default { reg: 0x0E30, def: 0xF337 },  /* R3632  - EQ2_11 */
    reg_default { reg: 0x0E31, def: 0x040B },  /* R3633  - EQ2_12 */
    reg_default { reg: 0x0E32, def: 0x0CBB },  /* R3634  - EQ2_13 */
    reg_default { reg: 0x0E33, def: 0x16F8 },  /* R3635  - EQ2_14 */
    reg_default { reg: 0x0E34, def: 0xF7D9 },  /* R3636  - EQ2_15 */
    reg_default { reg: 0x0E35, def: 0x040A },  /* R3637  - EQ2_16 */
    reg_default { reg: 0x0E36, def: 0x1F14 },  /* R3638  - EQ2_17 */
    reg_default { reg: 0x0E37, def: 0x058C },  /* R3639  - EQ2_18 */
    reg_default { reg: 0x0E38, def: 0x0563 },  /* R3640  - EQ2_19 */
    reg_default { reg: 0x0E39, def: 0x4000 },  /* R3641  - EQ2_20 */
    reg_default { reg: 0x0E3C, def: 0x6318 },  /* R3644  - EQ3_1 */
    reg_default { reg: 0x0E3D, def: 0x6300 },  /* R3645  - EQ3_2 */
    reg_default { reg: 0x0E3E, def: 0x0FC8 },  /* R3646  - EQ3_3 */
    reg_default { reg: 0x0E3F, def: 0x03FE },  /* R3647  - EQ3_4 */
    reg_default { reg: 0x0E40, def: 0x00E0 },  /* R3648  - EQ3_5 */
    reg_default { reg: 0x0E41, def: 0x1EC4 },  /* R3649  - EQ3_6 */
    reg_default { reg: 0x0E42, def: 0xF136 },  /* R3650  - EQ3_7 */
    reg_default { reg: 0x0E43, def: 0x0409 },  /* R3651  - EQ3_8 */
    reg_default { reg: 0x0E44, def: 0x04CC },  /* R3652  - EQ3_9 */
    reg_default { reg: 0x0E45, def: 0x1C9B },  /* R3653  - EQ3_10 */
    reg_default { reg: 0x0E46, def: 0xF337 },  /* R3654  - EQ3_11 */
    reg_default { reg: 0x0E47, def: 0x040B },  /* R3655  - EQ3_12 */
    reg_default { reg: 0x0E48, def: 0x0CBB },  /* R3656  - EQ3_13 */
    reg_default { reg: 0x0E49, def: 0x16F8 },  /* R3657  - EQ3_14 */
    reg_default { reg: 0x0E4A, def: 0xF7D9 },  /* R3658  - EQ3_15 */
    reg_default { reg: 0x0E4B, def: 0x040A },  /* R3659  - EQ3_16 */
    reg_default { reg: 0x0E4C, def: 0x1F14 },  /* R3660  - EQ3_17 */
    reg_default { reg: 0x0E4D, def: 0x058C },  /* R3661  - EQ3_18 */
    reg_default { reg: 0x0E4E, def: 0x0563 },  /* R3662  - EQ3_19 */
    reg_default { reg: 0x0E4F, def: 0x4000 },  /* R3663  - EQ3_20 */
    reg_default { reg: 0x0E52, def: 0x6318 },  /* R3666  - EQ4_1 */
    reg_default { reg: 0x0E53, def: 0x6300 },  /* R3667  - EQ4_2 */
    reg_default { reg: 0x0E54, def: 0x0FC8 },  /* R3668  - EQ4_3 */
    reg_default { reg: 0x0E55, def: 0x03FE },  /* R3669  - EQ4_4 */
    reg_default { reg: 0x0E56, def: 0x00E0 },  /* R3670  - EQ4_5 */
    reg_default { reg: 0x0E57, def: 0x1EC4 },  /* R3671  - EQ4_6 */
    reg_default { reg: 0x0E58, def: 0xF136 },  /* R3672  - EQ4_7 */
    reg_default { reg: 0x0E59, def: 0x0409 },  /* R3673  - EQ4_8 */
    reg_default { reg: 0x0E5A, def: 0x04CC },  /* R3674  - EQ4_9 */
    reg_default { reg: 0x0E5B, def: 0x1C9B },  /* R3675  - EQ4_10 */
    reg_default { reg: 0x0E5C, def: 0xF337 },  /* R3676  - EQ4_11 */
    reg_default { reg: 0x0E5D, def: 0x040B },  /* R3677  - EQ4_12 */
    reg_default { reg: 0x0E5E, def: 0x0CBB },  /* R3678  - EQ4_13 */
    reg_default { reg: 0x0E5F, def: 0x16F8 },  /* R3679  - EQ4_14 */
    reg_default { reg: 0x0E60, def: 0xF7D9 },  /* R3680  - EQ4_15 */
    reg_default { reg: 0x0E61, def: 0x040A },  /* R3681  - EQ4_16 */
    reg_default { reg: 0x0E62, def: 0x1F14 },  /* R3682  - EQ4_17 */
    reg_default { reg: 0x0E63, def: 0x058C },  /* R3683  - EQ4_18 */
    reg_default { reg: 0x0E64, def: 0x0563 },  /* R3684  - EQ4_19 */
    reg_default { reg: 0x0E65, def: 0x4000 },  /* R3685  - EQ4_20 */
    reg_default { reg: 0x0E80, def: 0x0018 },  /* R3712  - DRC1 ctrl1 */
    reg_default { reg: 0x0E81, def: 0x0933 },  /* R3713  - DRC1 ctrl2 */
    reg_default { reg: 0x0E82, def: 0x0018 },  /* R3714  - DRC1 ctrl3 */
    reg_default { reg: 0x0E83, def: 0x0000 },  /* R3715  - DRC1 ctrl4 */
    reg_default { reg: 0x0E84, def: 0x0000 },  /* R3716  - DRC1 ctrl5 */
    reg_default { reg: 0x0EC0, def: 0x0000 },  /* R3776  - HPLPF1_1 */
    reg_default { reg: 0x0EC1, def: 0x0000 },  /* R3777  - HPLPF1_2 */
    reg_default { reg: 0x0EC4, def: 0x0000 },  /* R3780  - HPLPF2_1 */
    reg_default { reg: 0x0EC5, def: 0x0000 },  /* R3781  - HPLPF2_2 */
    reg_default { reg: 0x0EC8, def: 0x0000 },  /* R3784  - HPLPF3_1 */
    reg_default { reg: 0x0EC9, def: 0x0000 },  /* R3785  - HPLPF3_2 */
    reg_default { reg: 0x0ECC, def: 0x0000 },  /* R3788  - HPLPF4_1 */
    reg_default { reg: 0x0ECD, def: 0x0000 },  /* R3789  - HPLPF4_2 */
    reg_default { reg: 0x0F02, def: 0x0000 },  /* R3842  - DSP1 Control 2 */
    reg_default { reg: 0x0F03, def: 0x0000 },  /* R3843  - DSP1 Control 3 */
    reg_default { reg: 0x0F04, def: 0x0000 },  /* R3844  - DSP1 Control 4 */
    reg_default { reg: 0x1002, def: 0x0000 },  /* R4098  - DSP2 Control 2 */
    reg_default { reg: 0x1003, def: 0x0000 },  /* R4099  - DSP2 Control 3 */
    reg_default { reg: 0x1004, def: 0x0000 },  /* R4100  - DSP2 Control 4 */
    reg_default { reg: 0x1102, def: 0x0000 },  /* R4354  - DSP3 Control 2 */
    reg_default { reg: 0x1103, def: 0x0000 },  /* R4355  - DSP3 Control 3 */
    reg_default { reg: 0x1104, def: 0x0000 },  /* R4356  - DSP3 Control 4 */
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
