/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ALSA SoC CX20721/CX20723 codec driver
 *
 * Copyright:	(C) 2017 Conexant Systems, Inc.
 * Author:	Simon Ho, <Simon.ho@conexant.com>
 */

/* Dependency from the original C header: SNDRV_PCM_RATE_48000. */

pub const CX2072X_MCLK_PLL: u32 = 1;
pub const CX2072X_MCLK_EXTERNAL_PLL: u32 = 1;
pub const CX2072X_MCLK_INTERNAL_OSC: u32 = 2;

/*#define CX2072X_RATES		SNDRV_PCM_RATE_8000_192000*/
pub const CX2072X_RATES_DSP: u32 = SNDRV_PCM_RATE_48000;

pub const CX2072X_REG_MAX: u32 = 0x8a3c;

pub const CX2072X_VENDOR_ID: u32 = 0x0200;
pub const CX2072X_REVISION_ID: u32 = 0x0208;
pub const CX2072X_CURRENT_BCLK_FREQUENCY: u32 = 0x00dc;
pub const CX2072X_AFG_POWER_STATE: u32 = 0x0414;
pub const CX2072X_UM_RESPONSE: u32 = 0x0420;
pub const CX2072X_GPIO_DATA: u32 = 0x0454;
pub const CX2072X_GPIO_ENABLE: u32 = 0x0458;
pub const CX2072X_GPIO_DIRECTION: u32 = 0x045c;
pub const CX2072X_GPIO_WAKE: u32 = 0x0460;
pub const CX2072X_GPIO_UM_ENABLE: u32 = 0x0464;
pub const CX2072X_GPIO_STICKY_MASK: u32 = 0x0468;
pub const CX2072X_AFG_FUNCTION_RESET: u32 = 0x07fc;
pub const CX2072X_DAC1_CONVERTER_FORMAT: u32 = 0x43c8;
pub const CX2072X_DAC1_AMP_GAIN_RIGHT: u32 = 0x41c0;
pub const CX2072X_DAC1_AMP_GAIN_LEFT: u32 = 0x41e0;
pub const CX2072X_DAC1_POWER_STATE: u32 = 0x4014;
pub const CX2072X_DAC1_CONVERTER_STREAM_CHANNEL: u32 = 0x4018;
pub const CX2072X_DAC1_EAPD_ENABLE: u32 = 0x4030;
pub const CX2072X_DAC2_CONVERTER_FORMAT: u32 = 0x47c8;
pub const CX2072X_DAC2_AMP_GAIN_RIGHT: u32 = 0x45c0;
pub const CX2072X_DAC2_AMP_GAIN_LEFT: u32 = 0x45e0;
pub const CX2072X_DAC2_POWER_STATE: u32 = 0x4414;
pub const CX2072X_DAC2_CONVERTER_STREAM_CHANNEL: u32 = 0x4418;
pub const CX2072X_ADC1_CONVERTER_FORMAT: u32 = 0x4fc8;
pub const CX2072X_ADC1_AMP_GAIN_RIGHT_0: u32 = 0x4d80;
pub const CX2072X_ADC1_AMP_GAIN_LEFT_0: u32 = 0x4da0;
pub const CX2072X_ADC1_AMP_GAIN_RIGHT_1: u32 = 0x4d84;
pub const CX2072X_ADC1_AMP_GAIN_LEFT_1: u32 = 0x4da4;
pub const CX2072X_ADC1_AMP_GAIN_RIGHT_2: u32 = 0x4d88;
pub const CX2072X_ADC1_AMP_GAIN_LEFT_2: u32 = 0x4da8;
pub const CX2072X_ADC1_AMP_GAIN_RIGHT_3: u32 = 0x4d8c;
pub const CX2072X_ADC1_AMP_GAIN_LEFT_3: u32 = 0x4dac;
pub const CX2072X_ADC1_AMP_GAIN_RIGHT_4: u32 = 0x4d90;
pub const CX2072X_ADC1_AMP_GAIN_LEFT_4: u32 = 0x4db0;
pub const CX2072X_ADC1_AMP_GAIN_RIGHT_5: u32 = 0x4d94;
pub const CX2072X_ADC1_AMP_GAIN_LEFT_5: u32 = 0x4db4;
pub const CX2072X_ADC1_AMP_GAIN_RIGHT_6: u32 = 0x4d98;
pub const CX2072X_ADC1_AMP_GAIN_LEFT_6: u32 = 0x4db8;
pub const CX2072X_ADC1_CONNECTION_SELECT_CONTROL: u32 = 0x4c04;
pub const CX2072X_ADC1_POWER_STATE: u32 = 0x4c14;
pub const CX2072X_ADC1_CONVERTER_STREAM_CHANNEL: u32 = 0x4c18;
pub const CX2072X_ADC2_CONVERTER_FORMAT: u32 = 0x53c8;
pub const CX2072X_ADC2_AMP_GAIN_RIGHT_0: u32 = 0x5180;
pub const CX2072X_ADC2_AMP_GAIN_LEFT_0: u32 = 0x51a0;
pub const CX2072X_ADC2_AMP_GAIN_RIGHT_1: u32 = 0x5184;
pub const CX2072X_ADC2_AMP_GAIN_LEFT_1: u32 = 0x51a4;
pub const CX2072X_ADC2_AMP_GAIN_RIGHT_2: u32 = 0x5188;
pub const CX2072X_ADC2_AMP_GAIN_LEFT_2: u32 = 0x51a8;
pub const CX2072X_ADC2_CONNECTION_SELECT_CONTROL: u32 = 0x5004;
pub const CX2072X_ADC2_POWER_STATE: u32 = 0x5014;
pub const CX2072X_ADC2_CONVERTER_STREAM_CHANNEL: u32 = 0x5018;
pub const CX2072X_PORTA_CONNECTION_SELECT_CTRL: u32 = 0x5804;
pub const CX2072X_PORTA_POWER_STATE: u32 = 0x5814;
pub const CX2072X_PORTA_PIN_CTRL: u32 = 0x581c;
pub const CX2072X_PORTA_UNSOLICITED_RESPONSE: u32 = 0x5820;
pub const CX2072X_PORTA_PIN_SENSE: u32 = 0x5824;
pub const CX2072X_PORTA_EAPD_BTL: u32 = 0x5830;
pub const CX2072X_PORTB_POWER_STATE: u32 = 0x6014;
pub const CX2072X_PORTB_PIN_CTRL: u32 = 0x601c;
pub const CX2072X_PORTB_UNSOLICITED_RESPONSE: u32 = 0x6020;
pub const CX2072X_PORTB_PIN_SENSE: u32 = 0x6024;
pub const CX2072X_PORTB_EAPD_BTL: u32 = 0x6030;
pub const CX2072X_PORTB_GAIN_RIGHT: u32 = 0x6180;
pub const CX2072X_PORTB_GAIN_LEFT: u32 = 0x61a0;
pub const CX2072X_PORTC_POWER_STATE: u32 = 0x6814;
pub const CX2072X_PORTC_PIN_CTRL: u32 = 0x681c;
pub const CX2072X_PORTC_GAIN_RIGHT: u32 = 0x6980;
pub const CX2072X_PORTC_GAIN_LEFT: u32 = 0x69a0;
pub const CX2072X_PORTD_POWER_STATE: u32 = 0x6414;
pub const CX2072X_PORTD_PIN_CTRL: u32 = 0x641c;
pub const CX2072X_PORTD_UNSOLICITED_RESPONSE: u32 = 0x6420;
pub const CX2072X_PORTD_PIN_SENSE: u32 = 0x6424;
pub const CX2072X_PORTD_GAIN_RIGHT: u32 = 0x6580;
pub const CX2072X_PORTD_GAIN_LEFT: u32 = 0x65a0;
pub const CX2072X_PORTE_CONNECTION_SELECT_CTRL: u32 = 0x7404;
pub const CX2072X_PORTE_POWER_STATE: u32 = 0x7414;
pub const CX2072X_PORTE_PIN_CTRL: u32 = 0x741c;
pub const CX2072X_PORTE_UNSOLICITED_RESPONSE: u32 = 0x7420;
pub const CX2072X_PORTE_PIN_SENSE: u32 = 0x7424;
pub const CX2072X_PORTE_EAPD_BTL: u32 = 0x7430;
pub const CX2072X_PORTE_GAIN_RIGHT: u32 = 0x7580;
pub const CX2072X_PORTE_GAIN_LEFT: u32 = 0x75a0;
pub const CX2072X_PORTF_POWER_STATE: u32 = 0x7814;
pub const CX2072X_PORTF_PIN_CTRL: u32 = 0x781c;
pub const CX2072X_PORTF_UNSOLICITED_RESPONSE: u32 = 0x7820;
pub const CX2072X_PORTF_PIN_SENSE: u32 = 0x7824;
pub const CX2072X_PORTF_GAIN_RIGHT: u32 = 0x7980;
pub const CX2072X_PORTF_GAIN_LEFT: u32 = 0x79a0;
pub const CX2072X_PORTG_POWER_STATE: u32 = 0x5c14;
pub const CX2072X_PORTG_PIN_CTRL: u32 = 0x5c1c;
pub const CX2072X_PORTG_CONNECTION_SELECT_CTRL: u32 = 0x5c04;
pub const CX2072X_PORTG_EAPD_BTL: u32 = 0x5c30;
pub const CX2072X_PORTM_POWER_STATE: u32 = 0x8814;
pub const CX2072X_PORTM_PIN_CTRL: u32 = 0x881c;
pub const CX2072X_PORTM_CONNECTION_SELECT_CTRL: u32 = 0x8804;
pub const CX2072X_PORTM_EAPD_BTL: u32 = 0x8830;
pub const CX2072X_MIXER_POWER_STATE: u32 = 0x5414;
pub const CX2072X_MIXER_GAIN_RIGHT_0: u32 = 0x5580;
pub const CX2072X_MIXER_GAIN_LEFT_0: u32 = 0x55a0;
pub const CX2072X_MIXER_GAIN_RIGHT_1: u32 = 0x5584;
pub const CX2072X_MIXER_GAIN_LEFT_1: u32 = 0x55a4;
pub const CX2072X_EQ_ENABLE_BYPASS: u32 = 0x6d00;
pub const CX2072X_EQ_B0_COEFF: u32 = 0x6d02;
pub const CX2072X_EQ_B1_COEFF: u32 = 0x6d04;
pub const CX2072X_EQ_B2_COEFF: u32 = 0x6d06;
pub const CX2072X_EQ_A1_COEFF: u32 = 0x6d08;
pub const CX2072X_EQ_A2_COEFF: u32 = 0x6d0a;
pub const CX2072X_EQ_G_COEFF: u32 = 0x6d0c;
pub const CX2072X_EQ_BAND: u32 = 0x6d0d;
pub const CX2072X_SPKR_DRC_ENABLE_STEP: u32 = 0x6d10;
pub const CX2072X_SPKR_DRC_CONTROL: u32 = 0x6d14;
pub const CX2072X_SPKR_DRC_TEST: u32 = 0x6d18;
pub const CX2072X_DIGITAL_BIOS_TEST0: u32 = 0x6d80;
pub const CX2072X_DIGITAL_BIOS_TEST2: u32 = 0x6d84;
pub const CX2072X_I2SPCM_CONTROL1: u32 = 0x6e00;
pub const CX2072X_I2SPCM_CONTROL2: u32 = 0x6e04;
pub const CX2072X_I2SPCM_CONTROL3: u32 = 0x6e08;
pub const CX2072X_I2SPCM_CONTROL4: u32 = 0x6e0c;
pub const CX2072X_I2SPCM_CONTROL5: u32 = 0x6e10;
pub const CX2072X_I2SPCM_CONTROL6: u32 = 0x6e18;
pub const CX2072X_UM_INTERRUPT_CRTL_E: u32 = 0x6e14;
pub const CX2072X_CODEC_TEST2: u32 = 0x7108;
pub const CX2072X_CODEC_TEST9: u32 = 0x7124;
pub const CX2072X_CODEC_TESTXX: u32 = 0x7290;
pub const CX2072X_CODEC_TEST20: u32 = 0x7310;
pub const CX2072X_CODEC_TEST24: u32 = 0x731c;
pub const CX2072X_CODEC_TEST26: u32 = 0x7328;
pub const CX2072X_ANALOG_TEST3: u32 = 0x718c;
pub const CX2072X_ANALOG_TEST4: u32 = 0x7190;
pub const CX2072X_ANALOG_TEST5: u32 = 0x7194;
pub const CX2072X_ANALOG_TEST6: u32 = 0x7198;
pub const CX2072X_ANALOG_TEST7: u32 = 0x719c;
pub const CX2072X_ANALOG_TEST8: u32 = 0x71a0;
pub const CX2072X_ANALOG_TEST9: u32 = 0x71a4;
pub const CX2072X_ANALOG_TEST10: u32 = 0x71a8;
pub const CX2072X_ANALOG_TEST11: u32 = 0x71ac;
pub const CX2072X_ANALOG_TEST12: u32 = 0x71b0;
pub const CX2072X_ANALOG_TEST13: u32 = 0x71b4;
pub const CX2072X_DIGITAL_TEST0: u32 = 0x7200;
pub const CX2072X_DIGITAL_TEST1: u32 = 0x7204;
pub const CX2072X_DIGITAL_TEST11: u32 = 0x722c;
pub const CX2072X_DIGITAL_TEST12: u32 = 0x7230;
pub const CX2072X_DIGITAL_TEST15: u32 = 0x723c;
pub const CX2072X_DIGITAL_TEST16: u32 = 0x7080;
pub const CX2072X_DIGITAL_TEST17: u32 = 0x7084;
pub const CX2072X_DIGITAL_TEST18: u32 = 0x7088;
pub const CX2072X_DIGITAL_TEST19: u32 = 0x708c;
pub const CX2072X_DIGITAL_TEST20: u32 = 0x7090;

/* not used in the current code, for future extensions (if any) */
pub const CX2072X_MAX_EQ_BAND: u32 = 7;
pub const CX2072X_MAX_EQ_COEFF: u32 = 11;
pub const CX2072X_MAX_DRC_REGS: u32 = 9;
pub const CX2072X_MIC_EQ_COEFF: u32 = 10;
pub const CX2072X_PLBK_EQ_BAND_NUM: u32 = 7;
pub const CX2072X_PLBK_EQ_COEF_LEN: u32 = 11;
pub const CX2072X_PLBK_DRC_PARM_LEN: u32 = 9;
pub const CX2072X_CLASSD_AMP_LEN: u32 = 6;

/* DAI interface type */
pub const CX2072X_DAI_HIFI: u32 = 1;
pub const CX2072X_DAI_DSP: u32 = 2;
pub const CX2072X_DAI_DSP_PWM: u32 = 3; /* 4 ch, including mic and AEC */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cx2072x_reg_sample_size {
    CX2072X_SAMPLE_SIZE_8_BITS = 0,
    CX2072X_SAMPLE_SIZE_16_BITS = 1,
    CX2072X_SAMPLE_SIZE_24_BITS = 2,
    CX2072X_SAMPLE_SIZE_RESERVED = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx2072x_reg_i2spcm_ctrl_reg1_r {
    pub bits: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cx2072x_reg_i2spcm_ctrl_reg1 {
    pub r: cx2072x_reg_i2spcm_ctrl_reg1_r,
    pub ulval: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx2072x_reg_i2spcm_ctrl_reg2_r {
    pub bits: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cx2072x_reg_i2spcm_ctrl_reg2 {
    pub r: cx2072x_reg_i2spcm_ctrl_reg2_r,
    pub ulval: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx2072x_reg_i2spcm_ctrl_reg3_r {
    pub bits: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cx2072x_reg_i2spcm_ctrl_reg3 {
    pub r: cx2072x_reg_i2spcm_ctrl_reg3_r,
    pub ulval: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx2072x_reg_i2spcm_ctrl_reg4_r {
    pub bits: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cx2072x_reg_i2spcm_ctrl_reg4 {
    pub r: cx2072x_reg_i2spcm_ctrl_reg4_r,
    pub ulval: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx2072x_reg_i2spcm_ctrl_reg5_r {
    pub bits: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cx2072x_reg_i2spcm_ctrl_reg5 {
    pub r: cx2072x_reg_i2spcm_ctrl_reg5_r,
    pub ulval: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx2072x_reg_i2spcm_ctrl_reg6_r {
    pub bits: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cx2072x_reg_i2spcm_ctrl_reg6 {
    pub r: cx2072x_reg_i2spcm_ctrl_reg6_r,
    pub ulval: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx2072x_reg_digital_bios_test2_r {
    pub bits: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cx2072x_reg_digital_bios_test2 {
    pub r: cx2072x_reg_digital_bios_test2_r,
    pub ulval: u32,
}

macro_rules! bitfield_accessors {
    ($ty:ty, $(($get:ident, $set:ident, $shift:expr, $width:expr)),+ $(,)?) => {
        impl $ty {
            $(
                pub fn $get(&self) -> u32 {
                    (self.bits >> $shift) & ((1u32 << $width) - 1)
                }

                pub fn $set(&mut self, val: u32) {
                    let mask = ((1u32 << $width) - 1) << $shift;
                    self.bits = (self.bits & !mask) | ((val << $shift) & mask);
                }
            )+
        }
    };
}

bitfield_accessors!(
    cx2072x_reg_i2spcm_ctrl_reg1_r,
    (rx_data_one_line, set_rx_data_one_line, 0, 1),
    (rx_ws_pol, set_rx_ws_pol, 1, 1),
    (rx_ws_wid, set_rx_ws_wid, 2, 7),
    (rx_frm_len, set_rx_frm_len, 9, 5),
    (rx_sa_size, set_rx_sa_size, 14, 2),
    (tx_data_one_line, set_tx_data_one_line, 16, 1),
    (tx_ws_pol, set_tx_ws_pol, 17, 1),
    (tx_ws_wid, set_tx_ws_wid, 18, 7),
    (tx_frm_len, set_tx_frm_len, 25, 5),
    (tx_sa_size, set_tx_sa_size, 30, 2),
);

bitfield_accessors!(
    cx2072x_reg_i2spcm_ctrl_reg2_r,
    (tx_en_ch1, set_tx_en_ch1, 0, 1),
    (tx_en_ch2, set_tx_en_ch2, 1, 1),
    (tx_en_ch3, set_tx_en_ch3, 2, 1),
    (tx_en_ch4, set_tx_en_ch4, 3, 1),
    (tx_en_ch5, set_tx_en_ch5, 4, 1),
    (tx_en_ch6, set_tx_en_ch6, 5, 1),
    (tx_slot_1, set_tx_slot_1, 6, 5),
    (tx_slot_2, set_tx_slot_2, 11, 5),
    (tx_slot_3, set_tx_slot_3, 16, 5),
    (tx_slot_4, set_tx_slot_4, 21, 5),
    (res, set_res, 26, 1),
    (tx_data_neg_bclk, set_tx_data_neg_bclk, 27, 1),
    (tx_master, set_tx_master, 28, 1),
    (tx_tri_n, set_tx_tri_n, 29, 1),
    (tx_endian_sel, set_tx_endian_sel, 30, 1),
    (tx_dstart_dly, set_tx_dstart_dly, 31, 1),
);

bitfield_accessors!(
    cx2072x_reg_i2spcm_ctrl_reg3_r,
    (rx_en_ch1, set_rx_en_ch1, 0, 1),
    (rx_en_ch2, set_rx_en_ch2, 1, 1),
    (rx_en_ch3, set_rx_en_ch3, 2, 1),
    (rx_en_ch4, set_rx_en_ch4, 3, 1),
    (rx_en_ch5, set_rx_en_ch5, 4, 1),
    (rx_en_ch6, set_rx_en_ch6, 5, 1),
    (rx_slot_1, set_rx_slot_1, 6, 5),
    (rx_slot_2, set_rx_slot_2, 11, 5),
    (rx_slot_3, set_rx_slot_3, 16, 5),
    (rx_slot_4, set_rx_slot_4, 21, 5),
    (res, set_res, 26, 1),
    (rx_data_neg_bclk, set_rx_data_neg_bclk, 27, 1),
    (rx_master, set_rx_master, 28, 1),
    (rx_tri_n, set_rx_tri_n, 29, 1),
    (rx_endian_sel, set_rx_endian_sel, 30, 1),
    (rx_dstart_dly, set_rx_dstart_dly, 31, 1),
);

bitfield_accessors!(
    cx2072x_reg_i2spcm_ctrl_reg4_r,
    (rx_mute, set_rx_mute, 0, 1),
    (tx_mute, set_tx_mute, 1, 1),
    (reserved, set_reserved, 2, 1),
    (dac_34_independent, set_dac_34_independent, 3, 1),
    (dac_bclk_lrck_share, set_dac_bclk_lrck_share, 4, 1),
    (bclk_lrck_share_en, set_bclk_lrck_share_en, 5, 1),
    (reserved2, set_reserved2, 6, 2),
    (rx_last_dac_ch_en, set_rx_last_dac_ch_en, 8, 1),
    (rx_last_dac_ch, set_rx_last_dac_ch, 9, 3),
    (tx_last_adc_ch_en, set_tx_last_adc_ch_en, 12, 1),
    (tx_last_adc_ch, set_tx_last_adc_ch, 13, 3),
    (rx_slot_5, set_rx_slot_5, 16, 5),
    (rx_slot_6, set_rx_slot_6, 21, 5),
    (reserved3, set_reserved3, 26, 6),
);

bitfield_accessors!(
    cx2072x_reg_i2spcm_ctrl_reg5_r,
    (tx_slot_5, set_tx_slot_5, 0, 5),
    (reserved, set_reserved, 5, 3),
    (tx_slot_6, set_tx_slot_6, 8, 5),
    (reserved2, set_reserved2, 13, 3),
    (reserved3, set_reserved3, 16, 8),
    (i2s_pcm_clk_div, set_i2s_pcm_clk_div, 24, 7),
    (i2s_pcm_clk_div_chan_en, set_i2s_pcm_clk_div_chan_en, 31, 1),
);

bitfield_accessors!(
    cx2072x_reg_i2spcm_ctrl_reg6_r,
    (reserved, set_reserved, 0, 5),
    (rx_pause_cycles, set_rx_pause_cycles, 5, 3),
    (rx_pause_start_pos, set_rx_pause_start_pos, 8, 8),
    (reserved2, set_reserved2, 16, 5),
    (tx_pause_cycles, set_tx_pause_cycles, 21, 3),
    (tx_pause_start_pos, set_tx_pause_start_pos, 24, 8),
);

bitfield_accessors!(
    cx2072x_reg_digital_bios_test2_r,
    (pull_down_eapd, set_pull_down_eapd, 0, 2),
    (input_en_eapd_pad, set_input_en_eapd_pad, 2, 1),
    (push_pull_mode, set_push_pull_mode, 3, 1),
    (eapd_pad_output_driver, set_eapd_pad_output_driver, 4, 2),
    (pll_source, set_pll_source, 6, 1),
    (i2s_bclk_en, set_i2s_bclk_en, 7, 1),
    (i2s_bclk_invert, set_i2s_bclk_invert, 8, 1),
    (pll_ref_clock, set_pll_ref_clock, 9, 1),
    (class_d_shield_clk, set_class_d_shield_clk, 10, 1),
    (audio_pll_bypass_mode, set_audio_pll_bypass_mode, 11, 1),
    (reserved, set_reserved, 12, 4),
);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
