// SPDX-License-Identifier: GPL-2.0-only
/*
 * cs8409-tables.c  --  HD audio codec driver for Cirrus Logic CS8409 HDA bridge chip
 *
 * Copyright (C) 2021 Cirrus Logic, Inc. and
 *                    Cirrus Logic International Semiconductor Ltd.
 *
 * Author: Lucas Tanure <tanureal@opensource.cirrus.com>
 */

// C dependency intent: #include "cs8409.h"

/******************************************************************************
 *                          CS42L42 Specific Data
 *
 ******************************************************************************/

static CS42L42_DAC_DB_SCALE: &[u32] =
    declare_tlv_db_scale!(CS42L42_HP_VOL_REAL_MIN * 100, 100, 1);

static CS42L42_ADC_DB_SCALE: &[u32] =
    declare_tlv_db_scale!(CS42L42_AMIC_VOL_REAL_MIN * 100, 100, 1);

pub static CS42L42_DAC_VOLUME_MIXER: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    index: 0,
    subdevice: HDA_SUBDEV_AMP_FLAG | HDA_SUBDEV_NID_FLAG,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
    info: Some(cs42l42_volume_info),
    get: Some(cs42l42_volume_get),
    put: Some(cs42l42_volume_put),
    tlv: snd_kcontrol_tlv { p: CS42L42_DAC_DB_SCALE.as_ptr() },
    private_value: HDA_COMPOSE_AMP_VAL_OFS!(
        CS8409_PIN_ASP1_TRANSMITTER_A,
        3,
        CS8409_CODEC0,
        HDA_OUTPUT,
        CS42L42_VOL_DAC
    ) | HDA_AMP_VAL_MIN_MUTE,
};

pub static CS42L42_ADC_VOLUME_MIXER: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    index: 0,
    subdevice: HDA_SUBDEV_AMP_FLAG | HDA_SUBDEV_NID_FLAG,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
    info: Some(cs42l42_volume_info),
    get: Some(cs42l42_volume_get),
    put: Some(cs42l42_volume_put),
    tlv: snd_kcontrol_tlv { p: CS42L42_ADC_DB_SCALE.as_ptr() },
    private_value: HDA_COMPOSE_AMP_VAL_OFS!(
        CS8409_PIN_ASP1_RECEIVER_A,
        1,
        CS8409_CODEC0,
        HDA_INPUT,
        CS42L42_VOL_ADC
    ) | HDA_AMP_VAL_MIN_MUTE,
};

pub static CS42L42_48K_PCM_ANALOG_PLAYBACK: hda_pcm_stream = hda_pcm_stream {
    rates: SNDRV_PCM_RATE_48000, /* fixed rate */
};

pub static CS42L42_48K_PCM_ANALOG_CAPTURE: hda_pcm_stream = hda_pcm_stream {
    rates: SNDRV_PCM_RATE_48000, /* fixed rate */
};

/******************************************************************************
 *                   BULLSEYE / WARLOCK / CYBORG Specific Arrays
 *                               CS8409/CS42L42
 ******************************************************************************/

pub static CS8409_CS42L42_INIT_VERBS: &[hda_verb] = &[
    hda_verb { nid: CS8409_PIN_AFG, verb: AC_VERB_SET_GPIO_WAKE_MASK, param: 0x0018 }, /* WAKE from GPIO 3,4 */
    hda_verb { nid: CS8409_PIN_VENDOR_WIDGET, verb: AC_VERB_SET_PROC_STATE, param: 0x0001 }, /* Enable VPW processing */
    hda_verb { nid: CS8409_PIN_VENDOR_WIDGET, verb: AC_VERB_SET_COEF_INDEX, param: 0x0002 }, /* Configure GPIO 6,7 */
    hda_verb { nid: CS8409_PIN_VENDOR_WIDGET, verb: AC_VERB_SET_PROC_COEF, param: 0x0080 }, /* I2C mode */
    hda_verb { nid: CS8409_PIN_VENDOR_WIDGET, verb: AC_VERB_SET_COEF_INDEX, param: 0x005b }, /* Set I2C bus speed */
    hda_verb { nid: CS8409_PIN_VENDOR_WIDGET, verb: AC_VERB_SET_PROC_COEF, param: 0x0200 }, /* 100kHz I2C_STO = 2 */
    hda_verb::default(), /* terminator */
];

static CS8409_CS42L42_PINCFGS: &[hda_pintbl] = &[
    hda_pintbl { nid: CS8409_PIN_ASP1_TRANSMITTER_A, val: 0x042120f0 }, /* ASP-1-TX */
    hda_pintbl { nid: CS8409_PIN_ASP1_RECEIVER_A, val: 0x04a12050 }, /* ASP-1-RX */
    hda_pintbl { nid: CS8409_PIN_ASP2_TRANSMITTER_A, val: 0x901000f0 }, /* ASP-2-TX */
    hda_pintbl { nid: CS8409_PIN_DMIC1_IN, val: 0x90a00090 }, /* DMIC-1 */
    hda_pintbl::default(), /* terminator */
];

static CS8409_CS42L42_PINCFGS_NO_DMIC: &[hda_pintbl] = &[
    hda_pintbl { nid: CS8409_PIN_ASP1_TRANSMITTER_A, val: 0x042120f0 }, /* ASP-1-TX */
    hda_pintbl { nid: CS8409_PIN_ASP1_RECEIVER_A, val: 0x04a12050 }, /* ASP-1-RX */
    hda_pintbl { nid: CS8409_PIN_ASP2_TRANSMITTER_A, val: 0x901000f0 }, /* ASP-2-TX */
    hda_pintbl::default(), /* terminator */
];

/* Vendor specific HW configuration for CS42L42 */
static CS42L42_INIT_REG_SEQ: &[cs8409_i2c_param] = &[
    cs8409_i2c_param { addr: CS42L42_I2C_TIMEOUT, value: 0xB0, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ADC_CTL, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: 0x1D02, value: 0x06, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ADC_VOLUME, value: 0x9F, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_OSC_SWITCH, value: 0x01, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_MCLK_CTL, value: 0x02, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_SRC_CTL, value: 0x03, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_MCLK_SRC_SEL, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_FRM_CFG, value: 0x13, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_FSYNC_P_LOWER, value: 0xFF, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_FSYNC_P_UPPER, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_CLK_CFG, value: 0x20, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_SPDIF_CLK_CFG, value: 0x0D, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_CH1_AP_RES, value: 0x02, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_CH1_BIT_MSB, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_CH1_BIT_LSB, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_CH2_AP_RES, value: 0x02, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_CH2_BIT_MSB, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_CH2_BIT_LSB, value: 0x20, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_CH3_AP_RES, value: 0x02, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_CH3_BIT_MSB, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_CH3_BIT_LSB, value: 0x80, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_CH4_AP_RES, value: 0x02, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_CH4_BIT_MSB, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_CH4_BIT_LSB, value: 0xA0, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_EN, value: 0x0C, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_TX_CH_EN, value: 0x01, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_TX_CH_AP_RES, value: 0x02, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_TX_CH1_BIT_MSB, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_TX_CH1_BIT_LSB, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_TX_SZ_EN, value: 0x01, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_PWR_CTL1, value: 0x0A, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_PWR_CTL2, value: 0x84, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_MIXER_CHA_VOL, value: 0x3F, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_MIXER_CHB_VOL, value: 0x3F, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_MIXER_ADC_VOL, value: 0x3f, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_HP_CTL, value: 0x0D, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_MIC_DET_CTL1, value: 0xB6, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_TIPSENSE_CTL, value: 0xC2, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_HS_CLAMP_DISABLE, value: 0x01, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_HS_SWITCH_CTL, value: 0xF3, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_PWR_CTL3, value: 0x20, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_RSENSE_CTL2, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_RSENSE_CTL3, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_TSENSE_CTL, value: 0x80, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_HS_BIAS_CTL, value: 0xC0, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_PWR_CTL1, value: 0x02, delay: 10000 },
    cs8409_i2c_param { addr: CS42L42_ADC_OVFL_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_MIXER_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_SRC_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_TX_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_CODEC_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_SRCPL_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_VPMON_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_PLL_LOCK_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_TSRS_PLUG_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_DET_INT1_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_DET_INT2_MASK, value: 0xff, delay: 0 },
];

macro_rules! cir {
    ($nid:expr, $cir:expr, $coef:expr) => {
        cs8409_cir_param { nid: $nid, cir: $cir, coef: $coef }
    };
}

/* Vendor specific hw configuration for CS8409 */
pub static CS8409_CS42L42_HW_CFG: &[cs8409_cir_param] = &[
    /* +PLL1/2_EN, +I2C_EN */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_DEV_CFG1, 0xb008),
    /* ASP1/2_EN=0, ASP1_STP=1 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_DEV_CFG2, 0x0002),
    /* ASP1/2_BUS_IDLE=10, +GPIO_I2C */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_DEV_CFG3, 0x0a80),
    /* ASP1.A: TX.LAP=0, TX.LSZ=24 bits, TX.LCS=0 */
    cir!(CS8409_PIN_VENDOR_WIDGET, ASP1_A_TX_CTRL1, 0x0800),
    /* ASP1.A: TX.RAP=0, TX.RSZ=24 bits, TX.RCS=32 */
    cir!(CS8409_PIN_VENDOR_WIDGET, ASP1_A_TX_CTRL2, 0x0820),
    /* ASP2.A: TX.LAP=0, TX.LSZ=24 bits, TX.LCS=0 */
    cir!(CS8409_PIN_VENDOR_WIDGET, ASP2_A_TX_CTRL1, 0x0800),
    /* ASP2.A: TX.RAP=1, TX.RSZ=24 bits, TX.RCS=0 */
    cir!(CS8409_PIN_VENDOR_WIDGET, ASP2_A_TX_CTRL2, 0x2800),
    /* ASP1.A: RX.LAP=0, RX.LSZ=24 bits, RX.LCS=0 */
    cir!(CS8409_PIN_VENDOR_WIDGET, ASP1_A_RX_CTRL1, 0x0800),
    /* ASP1.A: RX.RAP=0, RX.RSZ=24 bits, RX.RCS=0 */
    cir!(CS8409_PIN_VENDOR_WIDGET, ASP1_A_RX_CTRL2, 0x0800),
    /* ASP1: LCHI = 00h */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_ASP1_CLK_CTRL1, 0x8000),
    /* ASP1: MC/SC_SRCSEL=PLL1, LCPR=FFh */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_ASP1_CLK_CTRL2, 0x28ff),
    /* ASP1: MCEN=0, FSD=011, SCPOL_IN/OUT=0, SCDIV=1:4 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_ASP1_CLK_CTRL3, 0x0062),
    /* ASP2: LCHI=1Fh */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_ASP2_CLK_CTRL1, 0x801f),
    /* ASP2: MC/SC_SRCSEL=PLL1, LCPR=3Fh */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_ASP2_CLK_CTRL2, 0x283f),
    /* ASP2: 5050=1, MCEN=0, FSD=010, SCPOL_IN/OUT=1, SCDIV=1:16 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_ASP2_CLK_CTRL3, 0x805c),
    /* DMIC1_MO=10b, DMIC1/2_SR=1 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_DMIC_CFG, 0x0023),
    /* ASP1/2_BEEP=0 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_BEEP_CFG, 0x0000),
    /* ASP1/2_EN=1, ASP1_STP=1 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_DEV_CFG2, 0x0062),
    /* -PLL2_EN */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_DEV_CFG1, 0x9008),
    /* TX2.A: pre-scale att.=0 dB */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PRE_SCALE_ATTN2, 0x0000),
    /* ASP1/2_xxx_EN=1, ASP1/2_MCLK_EN=0, DMIC1_SCL_EN=1 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PAD_CFG_SLW_RATE_CTRL, 0xfc03),
    /* test mode on */
    cir!(CS8409_PIN_VENDOR_WIDGET, 0xc0, 0x9999),
    /* GPIO hysteresis = 30 us */
    cir!(CS8409_PIN_VENDOR_WIDGET, 0xc5, 0x0000),
    /* test mode off */
    cir!(CS8409_PIN_VENDOR_WIDGET, 0xc0, 0x0000),
    cs8409_cir_param::default(), /* Terminator */
];

pub static CS8409_CS42L42_BULLSEYE_ATN: &[cs8409_cir_param] = &[
    /* EQ_SEL=1, EQ1/2_EN=0 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PFE_CTRL1, 0x4000),
    /* +EQ_ACC */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PFE_COEF_W2, 0x4000),
    /* +EQ2_EN */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PFE_CTRL1, 0x4010),
    /* EQ_DATA_HI=0x0647 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PFE_COEF_W1, 0x0647),
    /* +EQ_WRT, +EQ_ACC, EQ_ADR=0, EQ_DATA_LO=0x67 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PFE_COEF_W2, 0xc0c7),
    /* EQ_DATA_HI=0x0647 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PFE_COEF_W1, 0x0647),
    /* +EQ_WRT, +EQ_ACC, EQ_ADR=1, EQ_DATA_LO=0x67 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PFE_COEF_W2, 0xc1c7),
    /* EQ_DATA_HI=0xf370 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PFE_COEF_W1, 0xf370),
    /* +EQ_WRT, +EQ_ACC, EQ_ADR=2, EQ_DATA_LO=0x71 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PFE_COEF_W2, 0xc271),
    /* EQ_DATA_HI=0x1ef8 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PFE_COEF_W1, 0x1ef8),
    /* +EQ_WRT, +EQ_ACC, EQ_ADR=3, EQ_DATA_LO=0x48 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PFE_COEF_W2, 0xc348),
    /* EQ_DATA_HI=0xc110 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PFE_COEF_W1, 0xc110),
    /* +EQ_WRT, +EQ_ACC, EQ_ADR=4, EQ_DATA_LO=0x5a */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PFE_COEF_W2, 0xc45a),
    /* EQ_DATA_HI=0x1f29 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PFE_COEF_W1, 0x1f29),
    /* +EQ_WRT, +EQ_ACC, EQ_ADR=5, EQ_DATA_LO=0x74 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PFE_COEF_W2, 0xc574),
    /* EQ_DATA_HI=0x1d7a */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PFE_COEF_W1, 0x1d7a),
    /* +EQ_WRT, +EQ_ACC, EQ_ADR=6, EQ_DATA_LO=0x53 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PFE_COEF_W2, 0xc653),
    /* EQ_DATA_HI=0xc38c */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PFE_COEF_W1, 0xc38c),
    /* +EQ_WRT, +EQ_ACC, EQ_ADR=7, EQ_DATA_LO=0x14 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PFE_COEF_W2, 0xc714),
    /* EQ_DATA_HI=0x1ca3 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PFE_COEF_W1, 0x1ca3),
    /* +EQ_WRT, +EQ_ACC, EQ_ADR=8, EQ_DATA_LO=0xc7 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PFE_COEF_W2, 0xc8c7),
    /* EQ_DATA_HI=0xc38c */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PFE_COEF_W1, 0xc38c),
    /* +EQ_WRT, +EQ_ACC, EQ_ADR=9, EQ_DATA_LO=0x14 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PFE_COEF_W2, 0xc914),
    /* -EQ_ACC, -EQ_WRT */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PFE_COEF_W2, 0x0000),
    cs8409_cir_param::default(), /* Terminator */
];

pub static mut CS8409_CS42L42_CODEC: sub_codec = sub_codec {
    addr: CS42L42_I2C_ADDR,
    reset_gpio: CS8409_CS42L42_RESET,
    irq_mask: CS8409_CS42L42_INT,
    init_seq: CS42L42_INIT_REG_SEQ.as_ptr(),
    init_seq_num: CS42L42_INIT_REG_SEQ.len(),
    hp_jack_in: 0,
    mic_jack_in: 0,
    paged: 1,
    suspended: 1,
    no_type_dect: 0,
};

/******************************************************************************
 *                          Dolphin Specific Arrays
 *                            CS8409/ 2 X CS42L42
 ******************************************************************************/

pub static DOLPHIN_INIT_VERBS: &[hda_verb] = &[
    hda_verb { nid: 0x01, verb: AC_VERB_SET_GPIO_WAKE_MASK, param: DOLPHIN_WAKE }, /* WAKE from GPIO 0,4 */
    hda_verb { nid: CS8409_PIN_VENDOR_WIDGET, verb: AC_VERB_SET_PROC_STATE, param: 0x0001 }, /* Enable VPW processing  */
    hda_verb { nid: CS8409_PIN_VENDOR_WIDGET, verb: AC_VERB_SET_COEF_INDEX, param: 0x0002 }, /* Configure GPIO 6,7 */
    hda_verb { nid: CS8409_PIN_VENDOR_WIDGET, verb: AC_VERB_SET_PROC_COEF, param: 0x0080 }, /* I2C mode */
    hda_verb { nid: CS8409_PIN_VENDOR_WIDGET, verb: AC_VERB_SET_COEF_INDEX, param: 0x005b }, /* Set I2C bus speed */
    hda_verb { nid: CS8409_PIN_VENDOR_WIDGET, verb: AC_VERB_SET_PROC_COEF, param: 0x0200 }, /* 100kHz I2C_STO = 2 */
    hda_verb::default(), /* terminator */
];

static DOLPHIN_PINCFGS: &[hda_pintbl] = &[
    hda_pintbl { nid: 0x24, val: 0x022210f0 }, /* ASP-1-TX-A */
    hda_pintbl { nid: 0x25, val: 0x010240f0 }, /* ASP-1-TX-B */
    hda_pintbl { nid: 0x34, val: 0x02a21050 }, /* ASP-1-RX */
    hda_pintbl::default(), /* terminator */
];

/* Vendor specific HW configuration for CS42L42 */
static DOLPHIN_C0_INIT_REG_SEQ: &[cs8409_i2c_param] = &[
    cs8409_i2c_param { addr: CS42L42_I2C_TIMEOUT, value: 0xB0, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ADC_CTL, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: 0x1D02, value: 0x06, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ADC_VOLUME, value: 0x9F, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_OSC_SWITCH, value: 0x01, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_MCLK_CTL, value: 0x02, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_SRC_CTL, value: 0x03, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_MCLK_SRC_SEL, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_FRM_CFG, value: 0x13, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_FSYNC_P_LOWER, value: 0xFF, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_FSYNC_P_UPPER, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_CLK_CFG, value: 0x20, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_SPDIF_CLK_CFG, value: 0x0D, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_CH1_AP_RES, value: 0x02, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_CH1_BIT_MSB, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_CH1_BIT_LSB, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_CH2_AP_RES, value: 0x02, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_CH2_BIT_MSB, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_CH2_BIT_LSB, value: 0x20, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_EN, value: 0x0C, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_TX_CH_EN, value: 0x01, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_TX_CH_AP_RES, value: 0x02, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_TX_CH1_BIT_MSB, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_TX_CH1_BIT_LSB, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_TX_SZ_EN, value: 0x01, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_PWR_CTL1, value: 0x0A, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_PWR_CTL2, value: 0x84, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_HP_CTL, value: 0x0D, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_MIXER_CHA_VOL, value: 0x3F, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_MIXER_CHB_VOL, value: 0x3F, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_MIXER_ADC_VOL, value: 0x3f, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_MIC_DET_CTL1, value: 0xB6, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_TIPSENSE_CTL, value: 0xC2, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_HS_CLAMP_DISABLE, value: 0x01, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_HS_SWITCH_CTL, value: 0xF3, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_PWR_CTL3, value: 0x20, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_RSENSE_CTL2, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_RSENSE_CTL3, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_TSENSE_CTL, value: 0x80, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_HS_BIAS_CTL, value: 0xC0, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_PWR_CTL1, value: 0x02, delay: 10000 },
    cs8409_i2c_param { addr: CS42L42_ADC_OVFL_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_MIXER_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_SRC_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_TX_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_CODEC_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_SRCPL_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_VPMON_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_PLL_LOCK_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_TSRS_PLUG_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_DET_INT1_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_DET_INT2_MASK, value: 0xff, delay: 0 },
];

static DOLPHIN_C1_INIT_REG_SEQ: &[cs8409_i2c_param] = &[
    cs8409_i2c_param { addr: CS42L42_I2C_TIMEOUT, value: 0xB0, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ADC_CTL, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: 0x1D02, value: 0x06, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ADC_VOLUME, value: 0x9F, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_OSC_SWITCH, value: 0x01, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_MCLK_CTL, value: 0x02, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_SRC_CTL, value: 0x03, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_MCLK_SRC_SEL, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_FRM_CFG, value: 0x13, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_FSYNC_P_LOWER, value: 0xFF, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_FSYNC_P_UPPER, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_CLK_CFG, value: 0x20, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_SPDIF_CLK_CFG, value: 0x0D, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_CH1_AP_RES, value: 0x02, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_CH1_BIT_MSB, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_CH1_BIT_LSB, value: 0x80, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_CH2_AP_RES, value: 0x02, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_CH2_BIT_MSB, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_CH2_BIT_LSB, value: 0xA0, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_EN, value: 0x0C, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_TX_CH_EN, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_TX_CH_AP_RES, value: 0x02, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_TX_CH1_BIT_MSB, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_TX_CH1_BIT_LSB, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_TX_SZ_EN, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_PWR_CTL1, value: 0x0E, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_PWR_CTL2, value: 0x84, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_HP_CTL, value: 0x0D, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_MIXER_CHA_VOL, value: 0x3F, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_MIXER_CHB_VOL, value: 0x3F, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_MIXER_ADC_VOL, value: 0x3f, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_MIC_DET_CTL1, value: 0xB6, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_TIPSENSE_CTL, value: 0xC2, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_HS_CLAMP_DISABLE, value: 0x01, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_HS_SWITCH_CTL, value: 0xF3, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_PWR_CTL3, value: 0x20, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_RSENSE_CTL2, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_RSENSE_CTL3, value: 0x00, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_TSENSE_CTL, value: 0x80, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_HS_BIAS_CTL, value: 0xC0, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_PWR_CTL1, value: 0x06, delay: 10000 },
    cs8409_i2c_param { addr: CS42L42_ADC_OVFL_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_MIXER_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_SRC_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_RX_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_ASP_TX_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_CODEC_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_SRCPL_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_VPMON_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_PLL_LOCK_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_TSRS_PLUG_INT_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_DET_INT1_MASK, value: 0xff, delay: 0 },
    cs8409_i2c_param { addr: CS42L42_DET_INT2_MASK, value: 0xff, delay: 0 },
];

/* Vendor specific hw configuration for CS8409 */
pub static DOLPHIN_HW_CFG: &[cs8409_cir_param] = &[
    /* +PLL1/2_EN, +I2C_EN */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_DEV_CFG1, 0xb008),
    /* ASP1_EN=0, ASP1_STP=1 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_DEV_CFG2, 0x0002),
    /* ASP1/2_BUS_IDLE=10, +GPIO_I2C */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_DEV_CFG3, 0x0a80),
    /* ASP1.A: TX.LAP=0, TX.LSZ=24 bits, TX.LCS=0 */
    cir!(CS8409_PIN_VENDOR_WIDGET, ASP1_A_TX_CTRL1, 0x0800),
    /* ASP1.A: TX.RAP=0, TX.RSZ=24 bits, TX.RCS=32 */
    cir!(CS8409_PIN_VENDOR_WIDGET, ASP1_A_TX_CTRL2, 0x0820),
    /* ASP1.B: TX.LAP=0, TX.LSZ=24 bits, TX.LCS=128 */
    cir!(CS8409_PIN_VENDOR_WIDGET, ASP1_B_TX_CTRL1, 0x0880),
    /* ASP1.B: TX.RAP=0, TX.RSZ=24 bits, TX.RCS=160 */
    cir!(CS8409_PIN_VENDOR_WIDGET, ASP1_B_TX_CTRL2, 0x08a0),
    /* ASP1.A: RX.LAP=0, RX.LSZ=24 bits, RX.LCS=0 */
    cir!(CS8409_PIN_VENDOR_WIDGET, ASP1_A_RX_CTRL1, 0x0800),
    /* ASP1.A: RX.RAP=0, RX.RSZ=24 bits, RX.RCS=0 */
    cir!(CS8409_PIN_VENDOR_WIDGET, ASP1_A_RX_CTRL2, 0x0800),
    /* ASP1: LCHI = 00h */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_ASP1_CLK_CTRL1, 0x8000),
    /* ASP1: MC/SC_SRCSEL=PLL1, LCPR=FFh */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_ASP1_CLK_CTRL2, 0x28ff),
    /* ASP1: MCEN=0, FSD=011, SCPOL_IN/OUT=0, SCDIV=1:4 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_ASP1_CLK_CTRL3, 0x0062),
    /* ASP1/2_BEEP=0 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_BEEP_CFG, 0x0000),
    /* ASP1_EN=1, ASP1_STP=1 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_DEV_CFG2, 0x0022),
    /* -PLL2_EN */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_DEV_CFG1, 0x9008),
    /* ASP1_xxx_EN=1, ASP1_MCLK_EN=0 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PAD_CFG_SLW_RATE_CTRL, 0x5400),
    /* test mode on */
    cir!(CS8409_PIN_VENDOR_WIDGET, 0xc0, 0x9999),
    /* GPIO hysteresis = 30 us */
    cir!(CS8409_PIN_VENDOR_WIDGET, 0xc5, 0x0000),
    /* test mode off */
    cir!(CS8409_PIN_VENDOR_WIDGET, 0xc0, 0x0000),
    cs8409_cir_param::default(), /* Terminator */
];

pub static mut DOLPHIN_CS42L42_0: sub_codec = sub_codec {
    addr: DOLPHIN_C0_I2C_ADDR,
    reset_gpio: DOLPHIN_C0_RESET,
    irq_mask: DOLPHIN_C0_INT,
    init_seq: DOLPHIN_C0_INIT_REG_SEQ.as_ptr(),
    init_seq_num: DOLPHIN_C0_INIT_REG_SEQ.len(),
    hp_jack_in: 0,
    mic_jack_in: 0,
    paged: 1,
    suspended: 1,
    no_type_dect: 0,
};

pub static mut DOLPHIN_CS42L42_1: sub_codec = sub_codec {
    addr: DOLPHIN_C1_I2C_ADDR,
    reset_gpio: DOLPHIN_C1_RESET,
    irq_mask: DOLPHIN_C1_INT,
    init_seq: DOLPHIN_C1_INIT_REG_SEQ.as_ptr(),
    init_seq_num: DOLPHIN_C1_INIT_REG_SEQ.len(),
    hp_jack_in: 0,
    mic_jack_in: 0,
    paged: 1,
    suspended: 1,
    no_type_dect: 1,
};

/******************************************************************************
 *                          CDB35L56-FOUR-HD Specific Arrays
 ******************************************************************************/
pub static CS8409_CDB35L56_FOUR_INIT_VERBS: &[hda_verb] = &[
    hda_verb { nid: CS8409_PIN_VENDOR_WIDGET, verb: AC_VERB_SET_PROC_STATE, param: 0x0001 }, /* Enable VPW processing */
    hda_verb::default(), /* terminator */
];

static CS8409_CDB35L56_FOUR_PINCFGS: &[hda_pintbl] = &[
    /* 0xPPLLLLLLDDDDTTTTCCCCMMMMAAAASSSS
     * P = PCON:	AC_JACK_PORT_*
     * L = LOC:	AC_JACK_LOC_*
     * D = DD:	device type AC_JACK_*
     * T = CTYP:	AC_JACK_CONN_*
     * C = COL:	AC_JACK_COLOR_*
     * M = MISC:	?
     * A = DA:	AC_DEFCFG_DEF_ASSOC
     * S = SEQ:	Sequence number in DA group
     */
    hda_pintbl { nid: CS8409_PIN_ASP2_TRANSMITTER_A, val: 0x901000f0 }, /* ASP-2-TX */
    /* "Mic" */
    hda_pintbl { nid: CS8409_PIN_ASP2_RECEIVER_A, val: 0x04a12050 }, /* ASP-2-RX */
    hda_pintbl::default(), /* terminator */
];

pub static CS8409_CDB35L56_FOUR_HW_CFG: &[cs8409_cir_param] = &[
    /* +PLL1/2_EN, +I2C_EN */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_DEV_CFG1, 0xb008),
    /* ASP1/2_EN=0, ASP1_STP=1 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_DEV_CFG2, 0x0002),
    /* ASP1/2_BUS_IDLE=10, +GPIO_I2C */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_DEV_CFG3, 0x0a80),
    /* ASP2.A: TX.LAP=0, TX.LSZ=24 bits, TX.LCS=0 */
    cir!(CS8409_PIN_VENDOR_WIDGET, ASP2_A_TX_CTRL1, 0x0800),
    /* ASP2.A: TX.RAP=1, TX.RSZ=24 bits, TX.RCS=0 */
    cir!(CS8409_PIN_VENDOR_WIDGET, ASP2_A_TX_CTRL2, 0x2800),
    /* ASP2.A: RX.LAP=0, RX.LSZ=24 bits, RX.LCS=0 */
    cir!(CS8409_PIN_VENDOR_WIDGET, ASP2_A_RX_CTRL1, 0x0800),
    /* ASP2.A: RX.RAP=1, RX.RSZ=24 bits, RX.RCS=0 */
    cir!(CS8409_PIN_VENDOR_WIDGET, ASP2_A_RX_CTRL2, 0x2800),
    /* ASP1: LCHI = 00h */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_ASP1_CLK_CTRL1, 0x8000),
    /* ASP1: MC/SC_SRCSEL=PLL1, LCPR=FFh */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_ASP1_CLK_CTRL2, 0x28ff),
    /* ASP1: MCEN=0, FSD=011, SCPOL_IN/OUT=0, SCDIV=1:4 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_ASP1_CLK_CTRL3, 0x0062),
    /* ASP2: LCHI=1Fh */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_ASP2_CLK_CTRL1, 0x801f),
    /* ASP2: MC/SC_SRCSEL=PLL1, LCPR=3Fh */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_ASP2_CLK_CTRL2, 0x283f),
    /* ASP2: 5050=1, MCEN=0, FSD=010, SCPOL_IN/OUT=1, SCDIV=1:16 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_ASP2_CLK_CTRL3, 0x805c),
    /* ASP1/2_BEEP=0 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_BEEP_CFG, 0x0000),
    /* ASP1/2_EN=1, ASP1_STP=1 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_DEV_CFG2, 0x0062),
    /* -PLL2_EN */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_DEV_CFG1, 0x9008), /* TX2.A: pre-scale att.=0 dB */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PRE_SCALE_ATTN2, 0x0000),
    /* ASP1/2_xxx_EN=1, ASP1/2_MCLK_EN=0, DMIC1_SCL_EN=1 */
    cir!(CS8409_PIN_VENDOR_WIDGET, CS8409_PAD_CFG_SLW_RATE_CTRL, 0xfc03),
    cs8409_cir_param::default(), /* Terminator */
];

/******************************************************************************
 *                         CS8409 Patch Driver Structs
 *                    Arrays Used for all projects using CS8409
 ******************************************************************************/

pub static CS8409_FIXUP_TBL: &[hda_quirk] = &[
    SND_PCI_QUIRK!(0x1028, 0x0A11, "Bullseye", CS8409_BULLSEYE),
    SND_PCI_QUIRK!(0x1028, 0x0A12, "Bullseye", CS8409_BULLSEYE),
    SND_PCI_QUIRK!(0x1028, 0x0A23, "Bullseye", CS8409_BULLSEYE),
    SND_PCI_QUIRK!(0x1028, 0x0A24, "Bullseye", CS8409_BULLSEYE),
    SND_PCI_QUIRK!(0x1028, 0x0A25, "Bullseye", CS8409_BULLSEYE),
    SND_PCI_QUIRK!(0x1028, 0x0A29, "Bullseye", CS8409_BULLSEYE),
    SND_PCI_QUIRK!(0x1028, 0x0A2A, "Bullseye", CS8409_BULLSEYE),
    SND_PCI_QUIRK!(0x1028, 0x0A2B, "Bullseye", CS8409_BULLSEYE),
    SND_PCI_QUIRK!(0x1028, 0x0A77, "Cyborg", CS8409_CYBORG),
    SND_PCI_QUIRK!(0x1028, 0x0A78, "Cyborg", CS8409_CYBORG),
    SND_PCI_QUIRK!(0x1028, 0x0A79, "Cyborg", CS8409_CYBORG),
    SND_PCI_QUIRK!(0x1028, 0x0A7A, "Cyborg", CS8409_CYBORG),
    SND_PCI_QUIRK!(0x1028, 0x0A7D, "Cyborg", CS8409_CYBORG),
    SND_PCI_QUIRK!(0x1028, 0x0A7E, "Cyborg", CS8409_CYBORG),
    SND_PCI_QUIRK!(0x1028, 0x0A7F, "Cyborg", CS8409_CYBORG),
    SND_PCI_QUIRK!(0x1028, 0x0A80, "Cyborg", CS8409_CYBORG),
    SND_PCI_QUIRK!(0x1028, 0x0AB0, "Warlock", CS8409_WARLOCK),
    SND_PCI_QUIRK!(0x1028, 0x0AB2, "Warlock", CS8409_WARLOCK),
    SND_PCI_QUIRK!(0x1028, 0x0AB1, "Warlock", CS8409_WARLOCK),
    SND_PCI_QUIRK!(0x1028, 0x0AB3, "Warlock", CS8409_WARLOCK),
    SND_PCI_QUIRK!(0x1028, 0x0AB4, "Warlock", CS8409_WARLOCK),
    SND_PCI_QUIRK!(0x1028, 0x0AB5, "Warlock", CS8409_WARLOCK),
    SND_PCI_QUIRK!(0x1028, 0x0ACF, "Dolphin", CS8409_DOLPHIN),
    SND_PCI_QUIRK!(0x1028, 0x0AD0, "Dolphin", CS8409_DOLPHIN),
    SND_PCI_QUIRK!(0x1028, 0x0AD1, "Dolphin", CS8409_DOLPHIN),
    SND_PCI_QUIRK!(0x1028, 0x0AD2, "Dolphin", CS8409_DOLPHIN),
    SND_PCI_QUIRK!(0x1028, 0x0AD3, "Dolphin", CS8409_DOLPHIN),
    SND_PCI_QUIRK!(0x1028, 0x0AD9, "Warlock", CS8409_WARLOCK),
    SND_PCI_QUIRK!(0x1028, 0x0ADA, "Warlock", CS8409_WARLOCK),
    SND_PCI_QUIRK!(0x1028, 0x0ADB, "Warlock", CS8409_WARLOCK),
    SND_PCI_QUIRK!(0x1028, 0x0ADC, "Warlock", CS8409_WARLOCK),
    SND_PCI_QUIRK!(0x1028, 0x0ADF, "Cyborg", CS8409_CYBORG),
    SND_PCI_QUIRK!(0x1028, 0x0AE0, "Cyborg", CS8409_CYBORG),
    SND_PCI_QUIRK!(0x1028, 0x0AE1, "Cyborg", CS8409_CYBORG),
    SND_PCI_QUIRK!(0x1028, 0x0AE2, "Cyborg", CS8409_CYBORG),
    SND_PCI_QUIRK!(0x1028, 0x0AE9, "Cyborg", CS8409_CYBORG),
    SND_PCI_QUIRK!(0x1028, 0x0AEA, "Cyborg", CS8409_CYBORG),
    SND_PCI_QUIRK!(0x1028, 0x0AEB, "Cyborg", CS8409_CYBORG),
    SND_PCI_QUIRK!(0x1028, 0x0AEC, "Cyborg", CS8409_CYBORG),
    SND_PCI_QUIRK!(0x1028, 0x0AED, "Cyborg", CS8409_CYBORG),
    SND_PCI_QUIRK!(0x1028, 0x0AEE, "Cyborg", CS8409_CYBORG),
    SND_PCI_QUIRK!(0x1028, 0x0AEF, "Cyborg", CS8409_CYBORG),
    SND_PCI_QUIRK!(0x1028, 0x0AF0, "Cyborg", CS8409_CYBORG),
    SND_PCI_QUIRK!(0x1028, 0x0AF4, "Warlock", CS8409_WARLOCK),
    SND_PCI_QUIRK!(0x1028, 0x0AF5, "Warlock", CS8409_WARLOCK),
    SND_PCI_QUIRK!(0x1028, 0x0B92, "Warlock MLK", CS8409_WARLOCK_MLK),
    SND_PCI_QUIRK!(0x1028, 0x0B93, "Warlock MLK Dual Mic", CS8409_WARLOCK_MLK_DUAL_MIC),
    SND_PCI_QUIRK!(0x1028, 0x0B94, "Warlock MLK", CS8409_WARLOCK_MLK),
    SND_PCI_QUIRK!(0x1028, 0x0B95, "Warlock MLK Dual Mic", CS8409_WARLOCK_MLK_DUAL_MIC),
    SND_PCI_QUIRK!(0x1028, 0x0B96, "Warlock MLK", CS8409_WARLOCK_MLK),
    SND_PCI_QUIRK!(0x1028, 0x0B97, "Warlock MLK Dual Mic", CS8409_WARLOCK_MLK_DUAL_MIC),
    SND_PCI_QUIRK!(0x1028, 0x0BA5, "Odin", CS8409_ODIN),
    SND_PCI_QUIRK!(0x1028, 0x0BA6, "Odin", CS8409_ODIN),
    SND_PCI_QUIRK!(0x1028, 0x0BA8, "Odin", CS8409_ODIN),
    SND_PCI_QUIRK!(0x1028, 0x0BAA, "Odin", CS8409_ODIN),
    SND_PCI_QUIRK!(0x1028, 0x0BAE, "Odin", CS8409_ODIN),
    SND_PCI_QUIRK!(0x1028, 0x0BB2, "Warlock MLK", CS8409_WARLOCK_MLK),
    SND_PCI_QUIRK!(0x1028, 0x0BB3, "Warlock MLK", CS8409_WARLOCK_MLK),
    SND_PCI_QUIRK!(0x1028, 0x0BB4, "Warlock MLK", CS8409_WARLOCK_MLK),
    SND_PCI_QUIRK!(0x1028, 0x0BB5, "Warlock N3 15 TGL-U Nuvoton EC", CS8409_WARLOCK),
    SND_PCI_QUIRK!(0x1028, 0x0BB6, "Warlock V3 15 TGL-U Nuvoton EC", CS8409_WARLOCK),
    SND_PCI_QUIRK!(0x1028, 0x0BB8, "Warlock MLK", CS8409_WARLOCK_MLK),
    SND_PCI_QUIRK!(0x1028, 0x0BB9, "Warlock MLK Dual Mic", CS8409_WARLOCK_MLK_DUAL_MIC),
    SND_PCI_QUIRK!(0x1028, 0x0BBA, "Warlock MLK", CS8409_WARLOCK_MLK),
    SND_PCI_QUIRK!(0x1028, 0x0BBB, "Warlock MLK Dual Mic", CS8409_WARLOCK_MLK_DUAL_MIC),
    SND_PCI_QUIRK!(0x1028, 0x0BBC, "Warlock MLK", CS8409_WARLOCK_MLK),
    SND_PCI_QUIRK!(0x1028, 0x0BBD, "Warlock MLK Dual Mic", CS8409_WARLOCK_MLK_DUAL_MIC),
    SND_PCI_QUIRK!(0x1028, 0x0BD4, "Dolphin", CS8409_DOLPHIN),
    SND_PCI_QUIRK!(0x1028, 0x0BD5, "Dolphin", CS8409_DOLPHIN),
    SND_PCI_QUIRK!(0x1028, 0x0BD6, "Dolphin", CS8409_DOLPHIN),
    SND_PCI_QUIRK!(0x1028, 0x0BD7, "Dolphin", CS8409_DOLPHIN),
    SND_PCI_QUIRK!(0x1028, 0x0BD8, "Dolphin", CS8409_DOLPHIN),
    SND_PCI_QUIRK!(0x1028, 0x0C43, "Dolphin", CS8409_DOLPHIN),
    SND_PCI_QUIRK!(0x1028, 0x0C50, "Dolphin", CS8409_DOLPHIN),
    SND_PCI_QUIRK!(0x1028, 0x0C51, "Dolphin", CS8409_DOLPHIN),
    SND_PCI_QUIRK!(0x1028, 0x0C52, "Dolphin", CS8409_DOLPHIN),
    SND_PCI_QUIRK!(0x1028, 0x0C73, "Dolphin", CS8409_DOLPHIN),
    SND_PCI_QUIRK!(0x1028, 0x0C75, "Dolphin", CS8409_DOLPHIN),
    SND_PCI_QUIRK!(0x1028, 0x0C7D, "Dolphin", CS8409_DOLPHIN),
    SND_PCI_QUIRK!(0x1028, 0x0C7F, "Dolphin", CS8409_DOLPHIN),
    hda_quirk::default(), /* terminator */
];

pub static CS8409_MODELS: &[hda_model_fixup] = &[
    hda_model_fixup { id: CS8409_BULLSEYE, name: c"bullseye".as_ptr() },
    hda_model_fixup { id: CS8409_WARLOCK, name: c"warlock".as_ptr() },
    hda_model_fixup { id: CS8409_WARLOCK_MLK, name: c"warlock mlk".as_ptr() },
    hda_model_fixup { id: CS8409_WARLOCK_MLK_DUAL_MIC, name: c"warlock mlk dual mic".as_ptr() },
    hda_model_fixup { id: CS8409_CYBORG, name: c"cyborg".as_ptr() },
    hda_model_fixup { id: CS8409_DOLPHIN, name: c"dolphin".as_ptr() },
    hda_model_fixup { id: CS8409_ODIN, name: c"odin".as_ptr() },
    hda_model_fixup { id: CS8409_CDB35L56_FOUR_HD, name: c"CDB35L56-FOUR-HD".as_ptr() },
    hda_model_fixup::default(),
];

pub static CS8409_FIXUPS: &[hda_fixup] = &[
    hda_fixup { /* [CS8409_BULLSEYE] */
        type_: HDA_FIXUP_PINS,
        v: hda_fixup_union { pins: CS8409_CS42L42_PINCFGS.as_ptr() },
        chained: true,
        chain_id: CS8409_FIXUPS,
    },
    hda_fixup { /* [CS8409_WARLOCK] */
        type_: HDA_FIXUP_PINS,
        v: hda_fixup_union { pins: CS8409_CS42L42_PINCFGS.as_ptr() },
        chained: true,
        chain_id: CS8409_FIXUPS,
    },
    hda_fixup { /* [CS8409_WARLOCK_MLK] */
        type_: HDA_FIXUP_PINS,
        v: hda_fixup_union { pins: CS8409_CS42L42_PINCFGS.as_ptr() },
        chained: true,
        chain_id: CS8409_FIXUPS,
    },
    hda_fixup { /* [CS8409_WARLOCK_MLK_DUAL_MIC] */
        type_: HDA_FIXUP_PINS,
        v: hda_fixup_union { pins: CS8409_CS42L42_PINCFGS.as_ptr() },
        chained: true,
        chain_id: CS8409_FIXUPS,
    },
    hda_fixup { /* [CS8409_CYBORG] */
        type_: HDA_FIXUP_PINS,
        v: hda_fixup_union { pins: CS8409_CS42L42_PINCFGS.as_ptr() },
        chained: true,
        chain_id: CS8409_FIXUPS,
    },
    hda_fixup { /* [CS8409_FIXUPS] */
        type_: HDA_FIXUP_FUNC,
        v: hda_fixup_union { func: Some(cs8409_cs42l42_fixups) },
        ..hda_fixup::default()
    },
    hda_fixup { /* [CS8409_DOLPHIN] */
        type_: HDA_FIXUP_PINS,
        v: hda_fixup_union { pins: DOLPHIN_PINCFGS.as_ptr() },
        chained: true,
        chain_id: CS8409_DOLPHIN_FIXUPS,
    },
    hda_fixup { /* [CS8409_DOLPHIN_FIXUPS] */
        type_: HDA_FIXUP_FUNC,
        v: hda_fixup_union { func: Some(dolphin_fixups) },
        ..hda_fixup::default()
    },
    hda_fixup { /* [CS8409_ODIN] */
        type_: HDA_FIXUP_PINS,
        v: hda_fixup_union { pins: CS8409_CS42L42_PINCFGS_NO_DMIC.as_ptr() },
        chained: true,
        chain_id: CS8409_FIXUPS,
    },
    hda_fixup { /* [CS8409_CDB35L56_FOUR_HD] */
        type_: HDA_FIXUP_PINS,
        v: hda_fixup_union { pins: CS8409_CDB35L56_FOUR_PINCFGS.as_ptr() },
        chained: true,
        chain_id: CS8409_CDB35L56_FOUR_HD_FIXUP,
    },
    hda_fixup { /* [CS8409_CDB35L56_FOUR_HD_FIXUP] */
        type_: HDA_FIXUP_FUNC,
        v: hda_fixup_union { func: Some(cs8409_cdb35l56_four_autodet_fixup) },
        ..hda_fixup::default()
    },
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
