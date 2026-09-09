/* SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause */
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

// ADC channels for PMIC5 Gen3

pub const fn virt_chan(sid: u32, chan: u32) -> u32 {
    (sid << 8) | chan
}

pub const fn adc5_gen3_ref_gnd(sid: u32) -> u32 { virt_chan(sid, 0x00) }
pub const fn adc5_gen3_1p25vref(sid: u32) -> u32 { virt_chan(sid, 0x01) }
pub const fn adc5_gen3_vref_vadc(sid: u32) -> u32 { virt_chan(sid, 0x02) }
pub const fn adc5_gen3_die_temp(sid: u32) -> u32 { virt_chan(sid, 0x03) }

pub const fn adc5_gen3_amux1_thm(sid: u32) -> u32 { virt_chan(sid, 0x04) }
pub const fn adc5_gen3_amux2_thm(sid: u32) -> u32 { virt_chan(sid, 0x05) }
pub const fn adc5_gen3_amux3_thm(sid: u32) -> u32 { virt_chan(sid, 0x06) }
pub const fn adc5_gen3_amux4_thm(sid: u32) -> u32 { virt_chan(sid, 0x07) }
pub const fn adc5_gen3_amux5_thm(sid: u32) -> u32 { virt_chan(sid, 0x08) }
pub const fn adc5_gen3_amux6_thm(sid: u32) -> u32 { virt_chan(sid, 0x09) }
pub const fn adc5_gen3_amux1_gpio(sid: u32) -> u32 { virt_chan(sid, 0x0a) }
pub const fn adc5_gen3_amux2_gpio(sid: u32) -> u32 { virt_chan(sid, 0x0b) }
pub const fn adc5_gen3_amux3_gpio(sid: u32) -> u32 { virt_chan(sid, 0x0c) }
pub const fn adc5_gen3_amux4_gpio(sid: u32) -> u32 { virt_chan(sid, 0x0d) }

pub const fn adc5_gen3_chg_temp(sid: u32) -> u32 { virt_chan(sid, 0x10) }
pub const fn adc5_gen3_usb_sns_v_16(sid: u32) -> u32 { virt_chan(sid, 0x11) }
pub const fn adc5_gen3_vin_div16_mux(sid: u32) -> u32 { virt_chan(sid, 0x12) }
pub const fn adc5_gen3_vref_bat_therm(sid: u32) -> u32 { virt_chan(sid, 0x15) }
pub const fn adc5_gen3_iin_fb(sid: u32) -> u32 { virt_chan(sid, 0x17) }
pub const fn adc5_gen3_temp_alarm_lite(sid: u32) -> u32 { virt_chan(sid, 0x18) }
pub const fn adc5_gen3_iin_smb(sid: u32) -> u32 { virt_chan(sid, 0x19) }
pub const fn adc5_gen3_ichg_smb(sid: u32) -> u32 { virt_chan(sid, 0x1b) }
pub const fn adc5_gen3_ichg_fb(sid: u32) -> u32 { virt_chan(sid, 0xa1) }

// 30k pull-up
pub const fn adc5_gen3_amux1_thm_30k_pu(sid: u32) -> u32 { virt_chan(sid, 0x24) }
pub const fn adc5_gen3_amux2_thm_30k_pu(sid: u32) -> u32 { virt_chan(sid, 0x25) }
pub const fn adc5_gen3_amux3_thm_30k_pu(sid: u32) -> u32 { virt_chan(sid, 0x26) }
pub const fn adc5_gen3_amux4_thm_30k_pu(sid: u32) -> u32 { virt_chan(sid, 0x27) }
pub const fn adc5_gen3_amux5_thm_30k_pu(sid: u32) -> u32 { virt_chan(sid, 0x28) }
pub const fn adc5_gen3_amux6_thm_30k_pu(sid: u32) -> u32 { virt_chan(sid, 0x29) }
pub const fn adc5_gen3_amux1_gpio_30k_pu(sid: u32) -> u32 { virt_chan(sid, 0x2a) }
pub const fn adc5_gen3_amux2_gpio_30k_pu(sid: u32) -> u32 { virt_chan(sid, 0x2b) }
pub const fn adc5_gen3_amux3_gpio_30k_pu(sid: u32) -> u32 { virt_chan(sid, 0x2c) }
pub const fn adc5_gen3_amux4_gpio_30k_pu(sid: u32) -> u32 { virt_chan(sid, 0x2d) }

// 100k pull-up
pub const fn adc5_gen3_amux1_thm_100k_pu(sid: u32) -> u32 { virt_chan(sid, 0x44) }
pub const fn adc5_gen3_amux2_thm_100k_pu(sid: u32) -> u32 { virt_chan(sid, 0x45) }
pub const fn adc5_gen3_amux3_thm_100k_pu(sid: u32) -> u32 { virt_chan(sid, 0x46) }
pub const fn adc5_gen3_amux4_thm_100k_pu(sid: u32) -> u32 { virt_chan(sid, 0x47) }
pub const fn adc5_gen3_amux5_thm_100k_pu(sid: u32) -> u32 { virt_chan(sid, 0x48) }
pub const fn adc5_gen3_amux6_thm_100k_pu(sid: u32) -> u32 { virt_chan(sid, 0x49) }
pub const fn adc5_gen3_amux1_gpio_100k_pu(sid: u32) -> u32 { virt_chan(sid, 0x4a) }
pub const fn adc5_gen3_amux2_gpio_100k_pu(sid: u32) -> u32 { virt_chan(sid, 0x4b) }
pub const fn adc5_gen3_amux3_gpio_100k_pu(sid: u32) -> u32 { virt_chan(sid, 0x4c) }
pub const fn adc5_gen3_amux4_gpio_100k_pu(sid: u32) -> u32 { virt_chan(sid, 0x4d) }

// 400k pull-up
pub const fn adc5_gen3_amux1_thm_400k_pu(sid: u32) -> u32 { virt_chan(sid, 0x64) }
pub const fn adc5_gen3_amux2_thm_400k_pu(sid: u32) -> u32 { virt_chan(sid, 0x65) }
pub const fn adc5_gen3_amux3_thm_400k_pu(sid: u32) -> u32 { virt_chan(sid, 0x66) }
pub const fn adc5_gen3_amux4_thm_400k_pu(sid: u32) -> u32 { virt_chan(sid, 0x67) }
pub const fn adc5_gen3_amux5_thm_400k_pu(sid: u32) -> u32 { virt_chan(sid, 0x68) }
pub const fn adc5_gen3_amux6_thm_400k_pu(sid: u32) -> u32 { virt_chan(sid, 0x69) }
pub const fn adc5_gen3_amux1_gpio_400k_pu(sid: u32) -> u32 { virt_chan(sid, 0x6a) }
pub const fn adc5_gen3_amux2_gpio_400k_pu(sid: u32) -> u32 { virt_chan(sid, 0x6b) }
pub const fn adc5_gen3_amux3_gpio_400k_pu(sid: u32) -> u32 { virt_chan(sid, 0x6c) }
pub const fn adc5_gen3_amux4_gpio_400k_pu(sid: u32) -> u32 { virt_chan(sid, 0x6d) }

// 1/3 Divider
pub const fn adc5_gen3_amux1_gpio_div3(sid: u32) -> u32 { virt_chan(sid, 0x8a) }
pub const fn adc5_gen3_amux2_gpio_div3(sid: u32) -> u32 { virt_chan(sid, 0x8b) }
pub const fn adc5_gen3_amux3_gpio_div3(sid: u32) -> u32 { virt_chan(sid, 0x8c) }
pub const fn adc5_gen3_amux4_gpio_div3(sid: u32) -> u32 { virt_chan(sid, 0x8d) }

pub const fn adc5_gen3_vph_pwr(sid: u32) -> u32 { virt_chan(sid, 0x8e) }
pub const fn adc5_gen3_vbat_sns_qbg(sid: u32) -> u32 { virt_chan(sid, 0x8f) }

pub const fn adc5_gen3_vbat_sns_chgr(sid: u32) -> u32 { virt_chan(sid, 0x94) }
pub const fn adc5_gen3_vbat_2s_mid_qbg(sid: u32) -> u32 { virt_chan(sid, 0x96) }
pub const fn adc5_gen3_vbat_2s_mid_chgr(sid: u32) -> u32 { virt_chan(sid, 0x9d) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
