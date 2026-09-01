/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * HD audio codec driver for Cirrus Logic CS8409 HDA bridge chip
 *
 * Copyright (C) 2021 Cirrus Logic, Inc. and
 *                    Cirrus Logic International Semiconductor Ltd.
 */

/* C header guard and include directives omitted; imported symbols are external dependencies. */

/* CS8409 Specific Definitions */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cs8409_pins {
    CS8409_PIN_ROOT,
    CS8409_PIN_AFG,
    CS8409_PIN_ASP1_OUT_A,
    CS8409_PIN_ASP1_OUT_B,
    CS8409_PIN_ASP1_OUT_C,
    CS8409_PIN_ASP1_OUT_D,
    CS8409_PIN_ASP1_OUT_E,
    CS8409_PIN_ASP1_OUT_F,
    CS8409_PIN_ASP1_OUT_G,
    CS8409_PIN_ASP1_OUT_H,
    CS8409_PIN_ASP2_OUT_A,
    CS8409_PIN_ASP2_OUT_B,
    CS8409_PIN_ASP2_OUT_C,
    CS8409_PIN_ASP2_OUT_D,
    CS8409_PIN_ASP2_OUT_E,
    CS8409_PIN_ASP2_OUT_F,
    CS8409_PIN_ASP2_OUT_G,
    CS8409_PIN_ASP2_OUT_H,
    CS8409_PIN_ASP1_IN_A,
    CS8409_PIN_ASP1_IN_B,
    CS8409_PIN_ASP1_IN_C,
    CS8409_PIN_ASP1_IN_D,
    CS8409_PIN_ASP1_IN_E,
    CS8409_PIN_ASP1_IN_F,
    CS8409_PIN_ASP1_IN_G,
    CS8409_PIN_ASP1_IN_H,
    CS8409_PIN_ASP2_IN_A,
    CS8409_PIN_ASP2_IN_B,
    CS8409_PIN_ASP2_IN_C,
    CS8409_PIN_ASP2_IN_D,
    CS8409_PIN_ASP2_IN_E,
    CS8409_PIN_ASP2_IN_F,
    CS8409_PIN_ASP2_IN_G,
    CS8409_PIN_ASP2_IN_H,
    CS8409_PIN_DMIC1,
    CS8409_PIN_DMIC2,
    CS8409_PIN_ASP1_TRANSMITTER_A,
    CS8409_PIN_ASP1_TRANSMITTER_B,
    CS8409_PIN_ASP1_TRANSMITTER_C,
    CS8409_PIN_ASP1_TRANSMITTER_D,
    CS8409_PIN_ASP1_TRANSMITTER_E,
    CS8409_PIN_ASP1_TRANSMITTER_F,
    CS8409_PIN_ASP1_TRANSMITTER_G,
    CS8409_PIN_ASP1_TRANSMITTER_H,
    CS8409_PIN_ASP2_TRANSMITTER_A,
    CS8409_PIN_ASP2_TRANSMITTER_B,
    CS8409_PIN_ASP2_TRANSMITTER_C,
    CS8409_PIN_ASP2_TRANSMITTER_D,
    CS8409_PIN_ASP2_TRANSMITTER_E,
    CS8409_PIN_ASP2_TRANSMITTER_F,
    CS8409_PIN_ASP2_TRANSMITTER_G,
    CS8409_PIN_ASP2_TRANSMITTER_H,
    CS8409_PIN_ASP1_RECEIVER_A,
    CS8409_PIN_ASP1_RECEIVER_B,
    CS8409_PIN_ASP1_RECEIVER_C,
    CS8409_PIN_ASP1_RECEIVER_D,
    CS8409_PIN_ASP1_RECEIVER_E,
    CS8409_PIN_ASP1_RECEIVER_F,
    CS8409_PIN_ASP1_RECEIVER_G,
    CS8409_PIN_ASP1_RECEIVER_H,
    CS8409_PIN_ASP2_RECEIVER_A,
    CS8409_PIN_ASP2_RECEIVER_B,
    CS8409_PIN_ASP2_RECEIVER_C,
    CS8409_PIN_ASP2_RECEIVER_D,
    CS8409_PIN_ASP2_RECEIVER_E,
    CS8409_PIN_ASP2_RECEIVER_F,
    CS8409_PIN_ASP2_RECEIVER_G,
    CS8409_PIN_ASP2_RECEIVER_H,
    CS8409_PIN_DMIC1_IN,
    CS8409_PIN_DMIC2_IN,
    CS8409_PIN_BEEP_GEN,
    CS8409_PIN_VENDOR_WIDGET,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cs8409_coefficient_index_registers {
    CS8409_DEV_CFG1,
    CS8409_DEV_CFG2,
    CS8409_DEV_CFG3,
    CS8409_ASP1_CLK_CTRL1,
    CS8409_ASP1_CLK_CTRL2,
    CS8409_ASP1_CLK_CTRL3,
    CS8409_ASP2_CLK_CTRL1,
    CS8409_ASP2_CLK_CTRL2,
    CS8409_ASP2_CLK_CTRL3,
    CS8409_DMIC_CFG,
    CS8409_BEEP_CFG,
    ASP1_RX_NULL_INS_RMV,
    ASP1_Rx_RATE1,
    ASP1_Rx_RATE2,
    ASP1_Tx_NULL_INS_RMV,
    ASP1_Tx_RATE1,
    ASP1_Tx_RATE2,
    ASP2_Rx_NULL_INS_RMV,
    ASP2_Rx_RATE1,
    ASP2_Rx_RATE2,
    ASP2_Tx_NULL_INS_RMV,
    ASP2_Tx_RATE1,
    ASP2_Tx_RATE2,
    ASP1_SYNC_CTRL,
    ASP2_SYNC_CTRL,
    ASP1_A_TX_CTRL1,
    ASP1_A_TX_CTRL2,
    ASP1_B_TX_CTRL1,
    ASP1_B_TX_CTRL2,
    ASP1_C_TX_CTRL1,
    ASP1_C_TX_CTRL2,
    ASP1_D_TX_CTRL1,
    ASP1_D_TX_CTRL2,
    ASP1_E_TX_CTRL1,
    ASP1_E_TX_CTRL2,
    ASP1_F_TX_CTRL1,
    ASP1_F_TX_CTRL2,
    ASP1_G_TX_CTRL1,
    ASP1_G_TX_CTRL2,
    ASP1_H_TX_CTRL1,
    ASP1_H_TX_CTRL2,
    ASP2_A_TX_CTRL1,
    ASP2_A_TX_CTRL2,
    ASP2_B_TX_CTRL1,
    ASP2_B_TX_CTRL2,
    ASP2_C_TX_CTRL1,
    ASP2_C_TX_CTRL2,
    ASP2_D_TX_CTRL1,
    ASP2_D_TX_CTRL2,
    ASP2_E_TX_CTRL1,
    ASP2_E_TX_CTRL2,
    ASP2_F_TX_CTRL1,
    ASP2_F_TX_CTRL2,
    ASP2_G_TX_CTRL1,
    ASP2_G_TX_CTRL2,
    ASP2_H_TX_CTRL1,
    ASP2_H_TX_CTRL2,
    ASP1_A_RX_CTRL1,
    ASP1_A_RX_CTRL2,
    ASP1_B_RX_CTRL1,
    ASP1_B_RX_CTRL2,
    ASP1_C_RX_CTRL1,
    ASP1_C_RX_CTRL2,
    ASP1_D_RX_CTRL1,
    ASP1_D_RX_CTRL2,
    ASP1_E_RX_CTRL1,
    ASP1_E_RX_CTRL2,
    ASP1_F_RX_CTRL1,
    ASP1_F_RX_CTRL2,
    ASP1_G_RX_CTRL1,
    ASP1_G_RX_CTRL2,
    ASP1_H_RX_CTRL1,
    ASP1_H_RX_CTRL2,
    ASP2_A_RX_CTRL1,
    ASP2_A_RX_CTRL2,
    ASP2_B_RX_CTRL1,
    ASP2_B_RX_CTRL2,
    ASP2_C_RX_CTRL1,
    ASP2_C_RX_CTRL2,
    ASP2_D_RX_CTRL1,
    ASP2_D_RX_CTRL2,
    ASP2_E_RX_CTRL1,
    ASP2_E_RX_CTRL2,
    ASP2_F_RX_CTRL1,
    ASP2_F_RX_CTRL2,
    ASP2_G_RX_CTRL1,
    ASP2_G_RX_CTRL2,
    ASP2_H_RX_CTRL1,
    ASP2_H_RX_CTRL2,
    CS8409_I2C_ADDR,
    CS8409_I2C_DATA,
    CS8409_I2C_CTRL,
    CS8409_I2C_STS,
    CS8409_I2C_QWRITE,
    CS8409_I2C_QREAD,
    CS8409_SPI_CTRL,
    CS8409_SPI_TX_DATA,
    CS8409_SPI_RX_DATA,
    CS8409_SPI_STS,
    CS8409_PFE_COEF_W1, /* Parametric filter engine coefficient write 1*/
    CS8409_PFE_COEF_W2,
    CS8409_PFE_CTRL1,
    CS8409_PFE_CTRL2,
    CS8409_PRE_SCALE_ATTN1,
    CS8409_PRE_SCALE_ATTN2,
    CS8409_PFE_COEF_MON1, /* Parametric filter engine coefficient monitor 1*/
    CS8409_PFE_COEF_MON2,
    CS8409_ASP1_INTRN_STS,
    CS8409_ASP2_INTRN_STS,
    CS8409_ASP1_RX_SCLK_COUNT,
    CS8409_ASP1_TX_SCLK_COUNT,
    CS8409_ASP2_RX_SCLK_COUNT,
    CS8409_ASP2_TX_SCLK_COUNT,
    CS8409_ASP_UNS_RESP_MASK,
    CS8409_LOOPBACK_CTRL = 0x80,
    CS8409_PAD_CFG_SLW_RATE_CTRL = 0x82, /* Pad Config and Slew Rate Control (CIR = 0x0082) */
}

/* CS42L42 Specific Definitions */

pub const CS8409_MAX_CODECS: usize = 8;
pub const CS42L42_VOLUMES: usize = 4usize;
pub const CS42L42_HP_VOL_REAL_MIN: i32 = -63;
pub const CS42L42_HP_VOL_REAL_MAX: i32 = 0;
pub const CS42L42_AMIC_VOL_REAL_MIN: i32 = -97;
pub const CS42L42_AMIC_VOL_REAL_MAX: i32 = 12;
pub const CS42L42_REG_AMIC_VOL_MASK: u32 = 0x00FF;
pub const CS42L42_HSTYPE_MASK: u32 = 0x03;
pub const CS42L42_I2C_TIMEOUT_US: u32 = 20000;
pub const CS42L42_I2C_SLEEP_US: u32 = 2000;
pub const CS42L42_PDN_TIMEOUT_US: u32 = 250000;
pub const CS42L42_PDN_SLEEP_US: u32 = 2000;
pub const CS42L42_ANA_MUTE_AB: u32 = 0x0C;
pub const CS42L42_FULL_SCALE_VOL_MASK: u32 = 2;
pub const CS42L42_FULL_SCALE_VOL_0DB: u32 = 0;
pub const CS42L42_FULL_SCALE_VOL_MINUS6DB: u32 = 1;

/* Dell BULLSEYE / WARLOCK / CYBORG Specific Definitions */

pub const CS42L42_I2C_ADDR: u32 = 0x48 << 1;
pub const CS8409_CS42L42_RESET: u32 = 1u32 << 5; /* CS8409_GPIO5 */
pub const CS8409_CS42L42_INT: u32 = 1u32 << 4; /* CS8409_GPIO4 */
pub const CS8409_CYBORG_SPEAKER_PDN: u32 = 1u32 << 2; /* CS8409_GPIO2 */
pub const CS8409_WARLOCK_SPEAKER_PDN: u32 = 1u32 << 1; /* CS8409_GPIO1 */
pub const CS8409_CS42L42_HP_PIN_NID: cs8409_pins = cs8409_pins::CS8409_PIN_ASP1_TRANSMITTER_A;
pub const CS8409_CS42L42_SPK_PIN_NID: cs8409_pins = cs8409_pins::CS8409_PIN_ASP2_TRANSMITTER_A;
pub const CS8409_CS42L42_AMIC_PIN_NID: cs8409_pins = cs8409_pins::CS8409_PIN_ASP1_RECEIVER_A;
pub const CS8409_CS42L42_DMIC_PIN_NID: cs8409_pins = cs8409_pins::CS8409_PIN_DMIC1_IN;
pub const CS8409_CS42L42_DMIC_ADC_PIN_NID: cs8409_pins = cs8409_pins::CS8409_PIN_DMIC1;

/* Dolphin */

pub const DOLPHIN_C0_I2C_ADDR: u32 = 0x48 << 1;
pub const DOLPHIN_C1_I2C_ADDR: u32 = 0x49 << 1;
pub const DOLPHIN_HP_PIN_NID: cs8409_pins = cs8409_pins::CS8409_PIN_ASP1_TRANSMITTER_A;
pub const DOLPHIN_LO_PIN_NID: cs8409_pins = cs8409_pins::CS8409_PIN_ASP1_TRANSMITTER_B;
pub const DOLPHIN_AMIC_PIN_NID: cs8409_pins = cs8409_pins::CS8409_PIN_ASP1_RECEIVER_A;

pub const DOLPHIN_C0_INT: u32 = 1u32 << 4;
pub const DOLPHIN_C1_INT: u32 = 1u32 << 0;
pub const DOLPHIN_C0_RESET: u32 = 1u32 << 5;
pub const DOLPHIN_C1_RESET: u32 = 1u32 << 1;
pub const DOLPHIN_WAKE: u32 = DOLPHIN_C0_INT | DOLPHIN_C1_INT;

pub const CS8409_BULLSEYE: u32 = 0;
pub const CS8409_WARLOCK: u32 = 1;
pub const CS8409_WARLOCK_MLK: u32 = 2;
pub const CS8409_WARLOCK_MLK_DUAL_MIC: u32 = 3;
pub const CS8409_CYBORG: u32 = 4;
pub const CS8409_FIXUPS: u32 = 5;
pub const CS8409_DOLPHIN: u32 = 6;
pub const CS8409_DOLPHIN_FIXUPS: u32 = 7;
pub const CS8409_ODIN: u32 = 8;
pub const CS8409_CDB35L56_FOUR_HD: u32 = 9;
pub const CS8409_CDB35L56_FOUR_HD_FIXUP: u32 = 10;

pub const CS8409_CODEC0: u32 = 0;
pub const CS8409_CODEC1: u32 = 1;

pub const CS42L42_VOL_ADC: u32 = 0;
pub const CS42L42_VOL_DAC: u32 = 1;

pub const CS42L42_ADC_VOL_OFFSET: u32 = CS42L42_VOL_ADC;
pub const CS42L42_DAC_CH0_VOL_OFFSET: u32 = CS42L42_VOL_DAC;
pub const CS42L42_DAC_CH1_VOL_OFFSET: u32 = CS42L42_VOL_DAC + 1;

#[repr(C)]
pub struct cs8409_i2c_param {
    pub addr: u32,
    pub value: u32,
    pub delay: u32,
}

#[repr(C)]
pub struct cs8409_cir_param {
    pub nid: u32,
    pub cir: u32,
    pub coeff: u32,
}

#[repr(C)]
pub struct sub_codec {
    pub codec: *mut hda_codec,
    pub addr: u32,
    pub reset_gpio: u32,
    pub irq_mask: u32,
    pub init_seq: *const cs8409_i2c_param,
    pub init_seq_num: u32,

    /* C unsigned int bitfields: hp_jack_in, mic_jack_in, suspended, paged,
     * full_scale_vol, no_type_dect.
     */
    pub bitfields: u32,
    pub last_page: u32,
    pub hsbias_hiz: u32,

    pub vol: [s8; CS42L42_VOLUMES],
}

pub const SUB_CODEC_HP_JACK_IN_SHIFT: u32 = 0;
pub const SUB_CODEC_MIC_JACK_IN_SHIFT: u32 = 1;
pub const SUB_CODEC_SUSPENDED_SHIFT: u32 = 2;
pub const SUB_CODEC_PAGED_SHIFT: u32 = 3;
pub const SUB_CODEC_FULL_SCALE_VOL_SHIFT: u32 = 4;
pub const SUB_CODEC_NO_TYPE_DECT_SHIFT: u32 = 5;

pub const SUB_CODEC_HP_JACK_IN_MASK: u32 = 1u32 << SUB_CODEC_HP_JACK_IN_SHIFT;
pub const SUB_CODEC_MIC_JACK_IN_MASK: u32 = 1u32 << SUB_CODEC_MIC_JACK_IN_SHIFT;
pub const SUB_CODEC_SUSPENDED_MASK: u32 = 1u32 << SUB_CODEC_SUSPENDED_SHIFT;
pub const SUB_CODEC_PAGED_MASK: u32 = 1u32 << SUB_CODEC_PAGED_SHIFT;
pub const SUB_CODEC_FULL_SCALE_VOL_MASK: u32 = 1u32 << SUB_CODEC_FULL_SCALE_VOL_SHIFT;
pub const SUB_CODEC_NO_TYPE_DECT_MASK: u32 = 1u32 << SUB_CODEC_NO_TYPE_DECT_SHIFT;

#[repr(C)]
pub struct cs8409_spec {
    pub gen: hda_gen_spec,
    pub codec: *mut hda_codec,

    pub scodecs: [*mut sub_codec; CS8409_MAX_CODECS],
    pub num_scodecs: u32,

    pub gpio_mask: u32,
    pub gpio_dir: u32,
    pub gpio_data: u32,

    pub speaker_pdn_gpio: i32,

    pub i2c_mux: mutex,
    pub i2c_clck_enabled: u32,
    pub dev_addr: u32,
    pub i2c_clk_work: delayed_work,

    /* C unsigned int bitfields: playback_started, capture_started, init_done,
     * build_ctrl_done, speaker_muted.
     */
    pub bitfields: u32,

    /* verb exec op override */
    pub exec_verb: Option<
        unsafe extern "C" fn(
            dev: *mut hdac_device,
            cmd: u32,
            flags: u32,
            res: *mut u32,
        ) -> i32,
    >,
    /* unsol_event op override */
    pub unsol_event: Option<unsafe extern "C" fn(codec: *mut hda_codec, res: u32)>,

    /* component binding */
    pub match_: *mut component_match,
    pub comps: hda_component_parent,
}

pub const CS8409_SPEC_PLAYBACK_STARTED_SHIFT: u32 = 0;
pub const CS8409_SPEC_CAPTURE_STARTED_SHIFT: u32 = 1;
pub const CS8409_SPEC_INIT_DONE_SHIFT: u32 = 2;
pub const CS8409_SPEC_BUILD_CTRL_DONE_SHIFT: u32 = 3;
pub const CS8409_SPEC_SPEAKER_MUTED_SHIFT: u32 = 4;

pub const CS8409_SPEC_PLAYBACK_STARTED_MASK: u32 = 1u32 << CS8409_SPEC_PLAYBACK_STARTED_SHIFT;
pub const CS8409_SPEC_CAPTURE_STARTED_MASK: u32 = 1u32 << CS8409_SPEC_CAPTURE_STARTED_SHIFT;
pub const CS8409_SPEC_INIT_DONE_MASK: u32 = 1u32 << CS8409_SPEC_INIT_DONE_SHIFT;
pub const CS8409_SPEC_BUILD_CTRL_DONE_MASK: u32 = 1u32 << CS8409_SPEC_BUILD_CTRL_DONE_SHIFT;
pub const CS8409_SPEC_SPEAKER_MUTED_MASK: u32 = 1u32 << CS8409_SPEC_SPEAKER_MUTED_SHIFT;

extern "C" {
    pub static cs42l42_dac_volume_mixer: snd_kcontrol_new;
    pub static cs42l42_adc_volume_mixer: snd_kcontrol_new;

    pub fn cs42l42_volume_info(
        kctrl: *mut snd_kcontrol,
        uinfo: *mut snd_ctl_elem_info,
    ) -> i32;
    pub fn cs42l42_volume_get(
        kctrl: *mut snd_kcontrol,
        uctrl: *mut snd_ctl_elem_value,
    ) -> i32;
    pub fn cs42l42_volume_put(
        kctrl: *mut snd_kcontrol,
        uctrl: *mut snd_ctl_elem_value,
    ) -> i32;

    pub static cs42l42_48k_pcm_analog_playback: hda_pcm_stream;
    pub static cs42l42_48k_pcm_analog_capture: hda_pcm_stream;
    pub static cs8409_fixup_tbl: [hda_quirk; 0];
    pub static cs8409_models: [hda_model_fixup; 0];
    pub static cs8409_fixups: [hda_fixup; 0];
    pub static cs8409_cs42l42_init_verbs: [hda_verb; 0];
    pub static cs8409_cs42l42_hw_cfg: [cs8409_cir_param; 0];
    pub static cs8409_cs42l42_bullseye_atn: [cs8409_cir_param; 0];
    pub static mut cs8409_cs42l42_codec: sub_codec;

    pub static dolphin_init_verbs: [hda_verb; 0];
    pub static dolphin_hw_cfg: [cs8409_cir_param; 0];
    pub static mut dolphin_cs42l42_0: sub_codec;
    pub static mut dolphin_cs42l42_1: sub_codec;

    pub fn cs8409_cs42l42_fixups(
        codec: *mut hda_codec,
        fix: *const hda_fixup,
        action: i32,
    );
    pub fn dolphin_fixups(codec: *mut hda_codec, fix: *const hda_fixup, action: i32);

    pub static cs8409_cdb35l56_four_hw_cfg: [cs8409_cir_param; 0];
    pub static cs8409_cdb35l56_four_init_verbs: [hda_verb; 0];
    pub fn cs8409_cdb35l56_four_autodet_fixup(
        codec: *mut hda_codec,
        fix: *const hda_fixup,
        action: i32,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
