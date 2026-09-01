// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2016, The Linux Foundation. All rights reserved.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

// Dependencies originally supplied by Linux/ALSA headers:
// linux/module.h, linux/err.h, linux/kernel.h, linux/delay.h, linux/types.h,
// linux/clk.h, linux/of.h, linux/platform_device.h, linux/regmap.h,
// linux/mfd/syscon.h, sound/soc.h, sound/pcm.h, sound/pcm_params.h, sound/tlv.h.

type u8 = u8;
type u16 = u16;
type u32 = u32;
type uint32_t = u32;

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

const fn GENMASK(h: u32, l: u32) -> u32 {
    ((!0u32) << l) & ((!0u32) >> (31 - h))
}

const LPASS_CDC_CLK_RX_RESET_CTL: u32 = 0x000;
const LPASS_CDC_CLK_TX_RESET_B1_CTL: u32 = 0x004;
const CLK_RX_RESET_B1_CTL_TX1_RESET_MASK: u32 = BIT(0);
const CLK_RX_RESET_B1_CTL_TX2_RESET_MASK: u32 = BIT(1);
const LPASS_CDC_CLK_DMIC_B1_CTL: u32 = 0x008;
const DMIC_B1_CTL_DMIC0_CLK_SEL_MASK: u32 = GENMASK(3, 1);
const DMIC_B1_CTL_DMIC0_CLK_SEL_DIV2: u32 = 0x0 << 1;
const DMIC_B1_CTL_DMIC0_CLK_SEL_DIV3: u32 = 0x1 << 1;
const DMIC_B1_CTL_DMIC0_CLK_SEL_DIV4: u32 = 0x2 << 1;
const DMIC_B1_CTL_DMIC0_CLK_SEL_DIV6: u32 = 0x3 << 1;
const DMIC_B1_CTL_DMIC0_CLK_SEL_DIV16: u32 = 0x4 << 1;
const DMIC_B1_CTL_DMIC0_CLK_EN_MASK: u32 = BIT(0);
const DMIC_B1_CTL_DMIC0_CLK_EN_ENABLE: u32 = BIT(0);

const LPASS_CDC_CLK_RX_I2S_CTL: u32 = 0x00C;
const RX_I2S_CTL_RX_I2S_MODE_MASK: u32 = BIT(5);
const RX_I2S_CTL_RX_I2S_MODE_16: u32 = BIT(5);
const RX_I2S_CTL_RX_I2S_MODE_32: u32 = 0;
const RX_I2S_CTL_RX_I2S_FS_RATE_MASK: u32 = GENMASK(2, 0);
const RX_I2S_CTL_RX_I2S_FS_RATE_F_8_KHZ: u32 = 0x0;
const RX_I2S_CTL_RX_I2S_FS_RATE_F_16_KHZ: u32 = 0x1;
const RX_I2S_CTL_RX_I2S_FS_RATE_F_32_KHZ: u32 = 0x2;
const RX_I2S_CTL_RX_I2S_FS_RATE_F_48_KHZ: u32 = 0x3;
const RX_I2S_CTL_RX_I2S_FS_RATE_F_96_KHZ: u32 = 0x4;
const RX_I2S_CTL_RX_I2S_FS_RATE_F_192_KHZ: u32 = 0x5;
const LPASS_CDC_CLK_TX_I2S_CTL: u32 = 0x010;
const TX_I2S_CTL_TX_I2S_MODE_MASK: u32 = BIT(5);
const TX_I2S_CTL_TX_I2S_MODE_16: u32 = BIT(5);
const TX_I2S_CTL_TX_I2S_MODE_32: u32 = 0;
const TX_I2S_CTL_TX_I2S_FS_RATE_MASK: u32 = GENMASK(2, 0);
const TX_I2S_CTL_TX_I2S_FS_RATE_F_8_KHZ: u32 = 0x0;
const TX_I2S_CTL_TX_I2S_FS_RATE_F_16_KHZ: u32 = 0x1;
const TX_I2S_CTL_TX_I2S_FS_RATE_F_32_KHZ: u32 = 0x2;
const TX_I2S_CTL_TX_I2S_FS_RATE_F_48_KHZ: u32 = 0x3;
const TX_I2S_CTL_TX_I2S_FS_RATE_F_96_KHZ: u32 = 0x4;
const TX_I2S_CTL_TX_I2S_FS_RATE_F_192_KHZ: u32 = 0x5;

const LPASS_CDC_CLK_OTHR_RESET_B1_CTL: u32 = 0x014;
const LPASS_CDC_CLK_TX_CLK_EN_B1_CTL: u32 = 0x018;
const LPASS_CDC_CLK_OTHR_CTL: u32 = 0x01C;
const LPASS_CDC_CLK_RX_B1_CTL: u32 = 0x020;
const LPASS_CDC_CLK_MCLK_CTL: u32 = 0x024;
const MCLK_CTL_MCLK_EN_MASK: u32 = BIT(0);
const MCLK_CTL_MCLK_EN_ENABLE: u32 = BIT(0);
const MCLK_CTL_MCLK_EN_DISABLE: u32 = 0;
const LPASS_CDC_CLK_PDM_CTL: u32 = 0x028;
const LPASS_CDC_CLK_PDM_CTL_PDM_EN_MASK: u32 = BIT(0);
const LPASS_CDC_CLK_PDM_CTL_PDM_EN: u32 = BIT(0);
const LPASS_CDC_CLK_PDM_CTL_PDM_CLK_SEL_MASK: u32 = BIT(1);
const LPASS_CDC_CLK_PDM_CTL_PDM_CLK_SEL_FB: u32 = BIT(1);
const LPASS_CDC_CLK_PDM_CTL_PDM_CLK_PDM_CLK: u32 = 0;

const LPASS_CDC_CLK_SD_CTL: u32 = 0x02C;
const LPASS_CDC_RX1_B1_CTL: u32 = 0x040;
const LPASS_CDC_RX2_B1_CTL: u32 = 0x060;
const LPASS_CDC_RX3_B1_CTL: u32 = 0x080;
const LPASS_CDC_RX1_B2_CTL: u32 = 0x044;
const LPASS_CDC_RX2_B2_CTL: u32 = 0x064;
const LPASS_CDC_RX3_B2_CTL: u32 = 0x084;
const LPASS_CDC_RX1_B3_CTL: u32 = 0x048;
const LPASS_CDC_RX2_B3_CTL: u32 = 0x068;
const LPASS_CDC_RX3_B3_CTL: u32 = 0x088;
const LPASS_CDC_RX1_B4_CTL: u32 = 0x04C;
const LPASS_CDC_RX2_B4_CTL: u32 = 0x06C;
const LPASS_CDC_RX3_B4_CTL: u32 = 0x08C;
const LPASS_CDC_RX1_B5_CTL: u32 = 0x050;
const LPASS_CDC_RX2_B5_CTL: u32 = 0x070;
const LPASS_CDC_RX3_B5_CTL: u32 = 0x090;
const LPASS_CDC_RX1_B6_CTL: u32 = 0x054;
const RXn_B6_CTL_MUTE_MASK: u32 = BIT(0);
const RXn_B6_CTL_MUTE_ENABLE: u32 = BIT(0);
const RXn_B6_CTL_MUTE_DISABLE: u32 = 0;
const LPASS_CDC_RX2_B6_CTL: u32 = 0x074;
const LPASS_CDC_RX3_B6_CTL: u32 = 0x094;
const LPASS_CDC_RX1_VOL_CTL_B1_CTL: u32 = 0x058;
const LPASS_CDC_RX2_VOL_CTL_B1_CTL: u32 = 0x078;
const LPASS_CDC_RX3_VOL_CTL_B1_CTL: u32 = 0x098;
const LPASS_CDC_RX1_VOL_CTL_B2_CTL: u32 = 0x05C;
const LPASS_CDC_RX2_VOL_CTL_B2_CTL: u32 = 0x07C;
const LPASS_CDC_RX3_VOL_CTL_B2_CTL: u32 = 0x09C;
const LPASS_CDC_TOP_GAIN_UPDATE: u32 = 0x0A0;
const LPASS_CDC_TOP_CTL: u32 = 0x0A4;
const TOP_CTL_DIG_MCLK_FREQ_MASK: u32 = BIT(0);
const TOP_CTL_DIG_MCLK_FREQ_F_12_288MHZ: u32 = 0;
const TOP_CTL_DIG_MCLK_FREQ_F_9_6MHZ: u32 = BIT(0);

const LPASS_CDC_DEBUG_DESER1_CTL: u32 = 0x0E0;
const LPASS_CDC_DEBUG_DESER2_CTL: u32 = 0x0E4;
const LPASS_CDC_DEBUG_B1_CTL_CFG: u32 = 0x0E8;
const LPASS_CDC_DEBUG_B2_CTL_CFG: u32 = 0x0EC;
const LPASS_CDC_DEBUG_B3_CTL_CFG: u32 = 0x0F0;
const LPASS_CDC_IIR1_GAIN_B1_CTL: u32 = 0x100;
const LPASS_CDC_IIR2_GAIN_B1_CTL: u32 = 0x140;
const LPASS_CDC_IIR1_GAIN_B2_CTL: u32 = 0x104;
const LPASS_CDC_IIR2_GAIN_B2_CTL: u32 = 0x144;
const LPASS_CDC_IIR1_GAIN_B3_CTL: u32 = 0x108;
const LPASS_CDC_IIR2_GAIN_B3_CTL: u32 = 0x148;
const LPASS_CDC_IIR1_GAIN_B4_CTL: u32 = 0x10C;
const LPASS_CDC_IIR2_GAIN_B4_CTL: u32 = 0x14C;
const LPASS_CDC_IIR1_GAIN_B5_CTL: u32 = 0x110;
const LPASS_CDC_IIR2_GAIN_B5_CTL: u32 = 0x150;
const LPASS_CDC_IIR1_GAIN_B6_CTL: u32 = 0x114;
const LPASS_CDC_IIR2_GAIN_B6_CTL: u32 = 0x154;
const LPASS_CDC_IIR1_GAIN_B7_CTL: u32 = 0x118;
const LPASS_CDC_IIR2_GAIN_B7_CTL: u32 = 0x158;
const LPASS_CDC_IIR1_GAIN_B8_CTL: u32 = 0x11C;
const LPASS_CDC_IIR2_GAIN_B8_CTL: u32 = 0x15C;
const LPASS_CDC_IIR1_CTL: u32 = 0x120;
const LPASS_CDC_IIR2_CTL: u32 = 0x160;
const LPASS_CDC_IIR1_GAIN_TIMER_CTL: u32 = 0x124;
const LPASS_CDC_IIR2_GAIN_TIMER_CTL: u32 = 0x164;
const LPASS_CDC_IIR1_COEF_B1_CTL: u32 = 0x128;
const LPASS_CDC_IIR2_COEF_B1_CTL: u32 = 0x168;
const LPASS_CDC_IIR1_COEF_B2_CTL: u32 = 0x12C;
const LPASS_CDC_IIR2_COEF_B2_CTL: u32 = 0x16C;
const LPASS_CDC_CONN_RX1_B1_CTL: u32 = 0x180;
const LPASS_CDC_CONN_RX1_B2_CTL: u32 = 0x184;
const LPASS_CDC_CONN_RX1_B3_CTL: u32 = 0x188;
const LPASS_CDC_CONN_RX2_B1_CTL: u32 = 0x18C;
const LPASS_CDC_CONN_RX2_B2_CTL: u32 = 0x190;
const LPASS_CDC_CONN_RX2_B3_CTL: u32 = 0x194;
const LPASS_CDC_CONN_RX3_B1_CTL: u32 = 0x198;
const LPASS_CDC_CONN_RX3_B2_CTL: u32 = 0x19C;
const LPASS_CDC_CONN_TX_B1_CTL: u32 = 0x1A0;
const LPASS_CDC_CONN_EQ1_B1_CTL: u32 = 0x1A8;
const LPASS_CDC_CONN_EQ1_B2_CTL: u32 = 0x1AC;
const LPASS_CDC_CONN_EQ1_B3_CTL: u32 = 0x1B0;
const LPASS_CDC_CONN_EQ1_B4_CTL: u32 = 0x1B4;
const LPASS_CDC_CONN_EQ2_B1_CTL: u32 = 0x1B8;
const LPASS_CDC_CONN_EQ2_B2_CTL: u32 = 0x1BC;
const LPASS_CDC_CONN_EQ2_B3_CTL: u32 = 0x1C0;
const LPASS_CDC_CONN_EQ2_B4_CTL: u32 = 0x1C4;
const LPASS_CDC_CONN_TX_I2S_SD1_CTL: u32 = 0x1C8;
const LPASS_CDC_TX1_VOL_CTL_TIMER: u32 = 0x280;
const LPASS_CDC_TX2_VOL_CTL_TIMER: u32 = 0x2A0;
const LPASS_CDC_TX1_VOL_CTL_GAIN: u32 = 0x284;
const LPASS_CDC_TX2_VOL_CTL_GAIN: u32 = 0x2A4;
const LPASS_CDC_TX1_VOL_CTL_CFG: u32 = 0x288;
const TX_VOL_CTL_CFG_MUTE_EN_MASK: u32 = BIT(0);
const TX_VOL_CTL_CFG_MUTE_EN_ENABLE: u32 = BIT(0);
const LPASS_CDC_TX2_VOL_CTL_CFG: u32 = 0x2A8;
const LPASS_CDC_TX1_MUX_CTL: u32 = 0x28C;
const TX_MUX_CTL_CUT_OFF_FREQ_MASK: u32 = GENMASK(5, 4);
const TX_MUX_CTL_CUT_OFF_FREQ_SHIFT: u32 = 4;
const TX_MUX_CTL_CF_NEG_3DB_4HZ: u32 = 0x0 << 4;
const TX_MUX_CTL_CF_NEG_3DB_75HZ: u32 = 0x1 << 4;
const TX_MUX_CTL_CF_NEG_3DB_150HZ: u32 = 0x2 << 4;
const TX_MUX_CTL_HPF_BP_SEL_MASK: u32 = BIT(3);
const TX_MUX_CTL_HPF_BP_SEL_BYPASS: u32 = BIT(3);
const TX_MUX_CTL_HPF_BP_SEL_NO_BYPASS: u32 = 0;
const LPASS_CDC_TX2_MUX_CTL: u32 = 0x2AC;
const LPASS_CDC_TX1_CLK_FS_CTL: u32 = 0x290;
const LPASS_CDC_TX2_CLK_FS_CTL: u32 = 0x2B0;
const LPASS_CDC_TX1_DMIC_CTL: u32 = 0x294;
const LPASS_CDC_TX2_DMIC_CTL: u32 = 0x2B4;
const TXN_DMIC_CTL_CLK_SEL_MASK: u32 = GENMASK(2, 0);
const TXN_DMIC_CTL_CLK_SEL_DIV2: u32 = 0x0;
const TXN_DMIC_CTL_CLK_SEL_DIV3: u32 = 0x1;
const TXN_DMIC_CTL_CLK_SEL_DIV4: u32 = 0x2;
const TXN_DMIC_CTL_CLK_SEL_DIV6: u32 = 0x3;
const TXN_DMIC_CTL_CLK_SEL_DIV16: u32 = 0x4;

const MSM8916_WCD_DIGITAL_RATES: u32 =
    SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_48000;
const MSM8916_WCD_DIGITAL_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE;

/* Codec supports 2 IIR filters */
const IIR1: c_int = 0;
const IIR2: c_int = 1;
const IIR_MAX: c_int = 2;

/* Codec supports 5 bands */
const BAND1: c_int = 0;
const BAND2: c_int = 1;
const BAND3: c_int = 2;
const BAND4: c_int = 3;
const BAND5: c_int = 4;
const BAND_MAX: usize = 5;

const WCD_IIR_FILTER_SIZE: usize = size_of::<u32>() * BAND_MAX;

#[repr(C)]
struct wcd_iir_filter_ctl {
    iir_idx: c_uint,
    band_idx: c_uint,
    bytes_ext: soc_bytes_ext,
}

#[repr(C)]
struct msm8916_wcd_digital_priv {
    ahbclk: *mut clk,
    mclk: *mut clk,
}

static rx_gain_reg: [c_ulong; 3] = [
    LPASS_CDC_RX1_VOL_CTL_B2_CTL as c_ulong,
    LPASS_CDC_RX2_VOL_CTL_B2_CTL as c_ulong,
    LPASS_CDC_RX3_VOL_CTL_B2_CTL as c_ulong,
];

static tx_gain_reg: [c_ulong; 2] = [
    LPASS_CDC_TX1_VOL_CTL_GAIN as c_ulong,
    LPASS_CDC_TX2_VOL_CTL_GAIN as c_ulong,
];

static rx_mix1_text: [*const c_char; 6] = c_str_array!(b"ZERO\0", b"IIR1\0", b"IIR2\0", b"RX1\0", b"RX2\0", b"RX3\0");
static rx_mix2_text: [*const c_char; 3] = c_str_array!(b"ZERO\0", b"IIR1\0", b"IIR2\0");
static dec_mux_text: [*const c_char; 6] = c_str_array!(b"ZERO\0", b"ADC1\0", b"ADC2\0", b"ADC3\0", b"DMIC1\0", b"DMIC2\0");
static cic_mux_text: [*const c_char; 2] = c_str_array!(b"AMIC\0", b"DMIC\0");

/* RX1 MIX1 */
static rx_mix1_inp_enum: [soc_enum; 3] = [
    SOC_ENUM_SINGLE!(LPASS_CDC_CONN_RX1_B1_CTL, 0, 6, rx_mix1_text),
    SOC_ENUM_SINGLE!(LPASS_CDC_CONN_RX1_B1_CTL, 3, 6, rx_mix1_text),
    SOC_ENUM_SINGLE!(LPASS_CDC_CONN_RX1_B2_CTL, 0, 6, rx_mix1_text),
];

/* RX2 MIX1 */
static rx2_mix1_inp_enum: [soc_enum; 3] = [
    SOC_ENUM_SINGLE!(LPASS_CDC_CONN_RX2_B1_CTL, 0, 6, rx_mix1_text),
    SOC_ENUM_SINGLE!(LPASS_CDC_CONN_RX2_B1_CTL, 3, 6, rx_mix1_text),
    SOC_ENUM_SINGLE!(LPASS_CDC_CONN_RX2_B2_CTL, 0, 6, rx_mix1_text),
];

/* RX3 MIX1 */
static rx3_mix1_inp_enum: [soc_enum; 3] = [
    SOC_ENUM_SINGLE!(LPASS_CDC_CONN_RX3_B1_CTL, 0, 6, rx_mix1_text),
    SOC_ENUM_SINGLE!(LPASS_CDC_CONN_RX3_B1_CTL, 3, 6, rx_mix1_text),
    SOC_ENUM_SINGLE!(LPASS_CDC_CONN_RX3_B2_CTL, 0, 6, rx_mix1_text),
];

/* RX1 MIX2 */
static rx_mix2_inp1_chain_enum: soc_enum =
    SOC_ENUM_SINGLE!(LPASS_CDC_CONN_RX1_B3_CTL, 0, 3, rx_mix2_text);

/* RX2 MIX2 */
static rx2_mix2_inp1_chain_enum: soc_enum =
    SOC_ENUM_SINGLE!(LPASS_CDC_CONN_RX2_B3_CTL, 0, 3, rx_mix2_text);

/* DEC */
static dec1_mux_enum: soc_enum = SOC_ENUM_SINGLE!(LPASS_CDC_CONN_TX_B1_CTL, 0, 6, dec_mux_text);
static dec2_mux_enum: soc_enum = SOC_ENUM_SINGLE!(LPASS_CDC_CONN_TX_B1_CTL, 3, 6, dec_mux_text);

/* CIC */
static cic1_mux_enum: soc_enum = SOC_ENUM_SINGLE!(LPASS_CDC_TX1_MUX_CTL, 0, 2, cic_mux_text);
static cic2_mux_enum: soc_enum = SOC_ENUM_SINGLE!(LPASS_CDC_TX2_MUX_CTL, 0, 2, cic_mux_text);

/* RDAC2 MUX */
static dec1_mux: snd_kcontrol_new = SOC_DAPM_ENUM!(c"DEC1 MUX Mux", dec1_mux_enum);
static dec2_mux: snd_kcontrol_new = SOC_DAPM_ENUM!(c"DEC2 MUX Mux", dec2_mux_enum);
static cic1_mux: snd_kcontrol_new = SOC_DAPM_ENUM!(c"CIC1 MUX Mux", cic1_mux_enum);
static cic2_mux: snd_kcontrol_new = SOC_DAPM_ENUM!(c"CIC2 MUX Mux", cic2_mux_enum);
static rx_mix1_inp1_mux: snd_kcontrol_new = SOC_DAPM_ENUM!(c"RX1 MIX1 INP1 Mux", rx_mix1_inp_enum[0]);
static rx_mix1_inp2_mux: snd_kcontrol_new = SOC_DAPM_ENUM!(c"RX1 MIX1 INP2 Mux", rx_mix1_inp_enum[1]);
static rx_mix1_inp3_mux: snd_kcontrol_new = SOC_DAPM_ENUM!(c"RX1 MIX1 INP3 Mux", rx_mix1_inp_enum[2]);
static rx2_mix1_inp1_mux: snd_kcontrol_new = SOC_DAPM_ENUM!(c"RX2 MIX1 INP1 Mux", rx2_mix1_inp_enum[0]);
static rx2_mix1_inp2_mux: snd_kcontrol_new = SOC_DAPM_ENUM!(c"RX2 MIX1 INP2 Mux", rx2_mix1_inp_enum[1]);
static rx2_mix1_inp3_mux: snd_kcontrol_new = SOC_DAPM_ENUM!(c"RX2 MIX1 INP3 Mux", rx2_mix1_inp_enum[2]);
static rx3_mix1_inp1_mux: snd_kcontrol_new = SOC_DAPM_ENUM!(c"RX3 MIX1 INP1 Mux", rx3_mix1_inp_enum[0]);
static rx3_mix1_inp2_mux: snd_kcontrol_new = SOC_DAPM_ENUM!(c"RX3 MIX1 INP2 Mux", rx3_mix1_inp_enum[1]);
static rx3_mix1_inp3_mux: snd_kcontrol_new = SOC_DAPM_ENUM!(c"RX3 MIX1 INP3 Mux", rx3_mix1_inp_enum[2]);
static rx1_mix2_inp1_mux: snd_kcontrol_new = SOC_DAPM_ENUM!(c"RX1 MIX2 INP1 Mux", rx_mix2_inp1_chain_enum);
static rx2_mix2_inp1_mux: snd_kcontrol_new = SOC_DAPM_ENUM!(c"RX2 MIX2 INP1 Mux", rx2_mix2_inp1_chain_enum);

/* Digital Gain control -84 dB to +40 dB in 1 dB steps */
static digital_gain: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-8400, 100, -8400);

/* Cutoff Freq for High Pass Filter at -3dB */
static hpf_cutoff_text: [*const c_char; 3] = c_str_array!(b"4Hz\0", b"75Hz\0", b"150Hz\0");

static tx1_hpf_cutoff_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(LPASS_CDC_TX1_MUX_CTL, 4, hpf_cutoff_text);
static tx2_hpf_cutoff_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(LPASS_CDC_TX2_MUX_CTL, 4, hpf_cutoff_text);

/* cut off for dc blocker inside rx chain */
static dc_blocker_cutoff_text: [*const c_char; 3] = c_str_array!(b"4Hz\0", b"75Hz\0", b"150Hz\0");

static rx1_dcb_cutoff_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(LPASS_CDC_RX1_B4_CTL, 0, dc_blocker_cutoff_text);
static rx2_dcb_cutoff_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(LPASS_CDC_RX2_B4_CTL, 0, dc_blocker_cutoff_text);
static rx3_dcb_cutoff_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(LPASS_CDC_RX3_B4_CTL, 0, dc_blocker_cutoff_text);

unsafe extern "C" fn msm8x16_wcd_codec_set_iir_gain(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let mut value: c_int = 0;
    let mut reg: c_int = 0;

    match event {
        SND_SOC_DAPM_POST_PMU => {
            if (*w).shift == 0 {
                reg = LPASS_CDC_IIR1_GAIN_B1_CTL as c_int;
            } else if (*w).shift == 1 {
                reg = LPASS_CDC_IIR2_GAIN_B1_CTL as c_int;
            }
            value = snd_soc_component_read(component, reg as c_uint) as c_int;
            snd_soc_component_write(component, reg as c_uint, value as c_uint);
        }
        _ => {}
    }
    0
}

unsafe fn get_iir_band_coeff(
    component: *mut snd_soc_component,
    iir_idx: c_int,
    band_idx: c_int,
    coeff_idx: c_int,
) -> uint32_t {
    let mut value: uint32_t = 0;

    /* Address does not automatically update if reading */
    snd_soc_component_write(
        component,
        LPASS_CDC_IIR1_COEF_B1_CTL + 64 * iir_idx as u32,
        ((band_idx * BAND_MAX as c_int + coeff_idx) * size_of::<uint32_t>() as c_int & 0x7F) as c_uint,
    );

    value |= snd_soc_component_read(component, LPASS_CDC_IIR1_COEF_B2_CTL + 64 * iir_idx as u32);

    snd_soc_component_write(
        component,
        LPASS_CDC_IIR1_COEF_B1_CTL + 64 * iir_idx as u32,
        (((band_idx * BAND_MAX as c_int + coeff_idx) * size_of::<uint32_t>() as c_int + 1) & 0x7F) as c_uint,
    );

    value |= snd_soc_component_read(component, LPASS_CDC_IIR1_COEF_B2_CTL + 64 * iir_idx as u32) << 8;

    snd_soc_component_write(
        component,
        LPASS_CDC_IIR1_COEF_B1_CTL + 64 * iir_idx as u32,
        (((band_idx * BAND_MAX as c_int + coeff_idx) * size_of::<uint32_t>() as c_int + 2) & 0x7F) as c_uint,
    );

    value |= snd_soc_component_read(component, LPASS_CDC_IIR1_COEF_B2_CTL + 64 * iir_idx as u32) << 16;

    snd_soc_component_write(
        component,
        LPASS_CDC_IIR1_COEF_B1_CTL + 64 * iir_idx as u32,
        (((band_idx * BAND_MAX as c_int + coeff_idx) * size_of::<uint32_t>() as c_int + 3) & 0x7F) as c_uint,
    );

    /* Mask bits top 2 bits since they are reserved */
    value |= (snd_soc_component_read(component, LPASS_CDC_IIR1_COEF_B2_CTL + 64 * iir_idx as u32) & 0x3f) << 24;
    value
}

unsafe extern "C" fn msm8x16_wcd_get_iir_band_audio_mixer(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let ctl = (*kcontrol).private_value as *mut wcd_iir_filter_ctl;
    let params = &mut (*ctl).bytes_ext as *mut soc_bytes_ext;
    let iir_idx = (*ctl).iir_idx as c_int;
    let band_idx = (*ctl).band_idx as c_int;
    let mut coeff: [u32; BAND_MAX] = [0; BAND_MAX];

    coeff[0] = get_iir_band_coeff(component, iir_idx, band_idx, 0);
    coeff[1] = get_iir_band_coeff(component, iir_idx, band_idx, 1);
    coeff[2] = get_iir_band_coeff(component, iir_idx, band_idx, 2);
    coeff[3] = get_iir_band_coeff(component, iir_idx, band_idx, 3);
    coeff[4] = get_iir_band_coeff(component, iir_idx, band_idx, 4);

    memcpy(
        (*ucontrol).value.bytes.data.as_mut_ptr() as *mut c_void,
        coeff.as_ptr() as *const c_void,
        (*params).max as usize,
    );

    0
}

unsafe fn set_iir_band_coeff(
    component: *mut snd_soc_component,
    iir_idx: c_int,
    _band_idx: c_int,
    value: uint32_t,
) {
    snd_soc_component_write(component, LPASS_CDC_IIR1_COEF_B2_CTL + 64 * iir_idx as u32, value & 0xFF);
    snd_soc_component_write(component, LPASS_CDC_IIR1_COEF_B2_CTL + 64 * iir_idx as u32, (value >> 8) & 0xFF);
    snd_soc_component_write(component, LPASS_CDC_IIR1_COEF_B2_CTL + 64 * iir_idx as u32, (value >> 16) & 0xFF);

    /* Mask top 2 bits, 7-8 are reserved */
    snd_soc_component_write(component, LPASS_CDC_IIR1_COEF_B2_CTL + 64 * iir_idx as u32, (value >> 24) & 0x3F);
}

unsafe extern "C" fn msm8x16_wcd_put_iir_band_audio_mixer(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let ctl = (*kcontrol).private_value as *mut wcd_iir_filter_ctl;
    let params = &mut (*ctl).bytes_ext as *mut soc_bytes_ext;
    let iir_idx = (*ctl).iir_idx as c_int;
    let band_idx = (*ctl).band_idx as c_int;
    let mut coeff: [u32; BAND_MAX] = [0; BAND_MAX];

    memcpy(
        coeff.as_mut_ptr() as *mut c_void,
        (*ucontrol).value.bytes.data.as_ptr() as *const c_void,
        (*params).max as usize,
    );

    /* Mask top bit it is reserved */
    /* Updates addr automatically for each B2 write */
    snd_soc_component_write(
        component,
        LPASS_CDC_IIR1_COEF_B1_CTL + 64 * iir_idx as u32,
        (band_idx * BAND_MAX as c_int * size_of::<uint32_t>() as c_int & 0x7F) as c_uint,
    );

    set_iir_band_coeff(component, iir_idx, band_idx, coeff[0]);
    set_iir_band_coeff(component, iir_idx, band_idx, coeff[1]);
    set_iir_band_coeff(component, iir_idx, band_idx, coeff[2]);
    set_iir_band_coeff(component, iir_idx, band_idx, coeff[3]);
    set_iir_band_coeff(component, iir_idx, band_idx, coeff[4]);

    0
}

unsafe extern "C" fn wcd_iir_filter_info(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_info,
) -> c_int {
    let ctl = (*kcontrol).private_value as *mut wcd_iir_filter_ctl;
    let params = &mut (*ctl).bytes_ext as *mut soc_bytes_ext;

    (*ucontrol).type_ = SNDRV_CTL_ELEM_TYPE_BYTES;
    (*ucontrol).count = (*params).max;

    0
}

macro_rules! WCD_IIR_FILTER_CTL {
    ($xname:expr, $iidx:expr, $bidx:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname.as_ptr(),
            info: Some(wcd_iir_filter_info),
            get: Some(msm8x16_wcd_get_iir_band_audio_mixer),
            put: Some(msm8x16_wcd_put_iir_band_audio_mixer),
            private_value: &wcd_iir_filter_ctl {
                iir_idx: $iidx as c_uint,
                band_idx: $bidx as c_uint,
                bytes_ext: soc_bytes_ext { max: WCD_IIR_FILTER_SIZE as c_uint },
            } as *const _ as c_ulong,
            ..snd_kcontrol_new::zeroed()
        }
    };
}

static msm8916_wcd_digital_snd_controls: &[snd_kcontrol_new] = &[
    SOC_SINGLE_S8_TLV!(c"RX1 Digital Volume", LPASS_CDC_RX1_VOL_CTL_B2_CTL, -84, 40, digital_gain),
    SOC_SINGLE_S8_TLV!(c"RX2 Digital Volume", LPASS_CDC_RX2_VOL_CTL_B2_CTL, -84, 40, digital_gain),
    SOC_SINGLE_S8_TLV!(c"RX3 Digital Volume", LPASS_CDC_RX3_VOL_CTL_B2_CTL, -84, 40, digital_gain),
    SOC_SINGLE_S8_TLV!(c"TX1 Digital Volume", LPASS_CDC_TX1_VOL_CTL_GAIN, -84, 40, digital_gain),
    SOC_SINGLE_S8_TLV!(c"TX2 Digital Volume", LPASS_CDC_TX2_VOL_CTL_GAIN, -84, 40, digital_gain),
    SOC_ENUM!(c"TX1 HPF Cutoff", tx1_hpf_cutoff_enum),
    SOC_ENUM!(c"TX2 HPF Cutoff", tx2_hpf_cutoff_enum),
    SOC_SINGLE!(c"TX1 HPF Switch", LPASS_CDC_TX1_MUX_CTL, 3, 1, 0),
    SOC_SINGLE!(c"TX2 HPF Switch", LPASS_CDC_TX2_MUX_CTL, 3, 1, 0),
    SOC_ENUM!(c"RX1 DCB Cutoff", rx1_dcb_cutoff_enum),
    SOC_ENUM!(c"RX2 DCB Cutoff", rx2_dcb_cutoff_enum),
    SOC_ENUM!(c"RX3 DCB Cutoff", rx3_dcb_cutoff_enum),
    SOC_SINGLE!(c"RX1 DCB Switch", LPASS_CDC_RX1_B5_CTL, 2, 1, 0),
    SOC_SINGLE!(c"RX2 DCB Switch", LPASS_CDC_RX2_B5_CTL, 2, 1, 0),
    SOC_SINGLE!(c"RX3 DCB Switch", LPASS_CDC_RX3_B5_CTL, 2, 1, 0),
    SOC_SINGLE!(c"RX1 Mute Switch", LPASS_CDC_RX1_B6_CTL, 0, 1, 0),
    SOC_SINGLE!(c"RX2 Mute Switch", LPASS_CDC_RX2_B6_CTL, 0, 1, 0),
    SOC_SINGLE!(c"RX3 Mute Switch", LPASS_CDC_RX3_B6_CTL, 0, 1, 0),
    SOC_SINGLE!(c"IIR1 Band1 Switch", LPASS_CDC_IIR1_CTL, 0, 1, 0),
    SOC_SINGLE!(c"IIR1 Band2 Switch", LPASS_CDC_IIR1_CTL, 1, 1, 0),
    SOC_SINGLE!(c"IIR1 Band3 Switch", LPASS_CDC_IIR1_CTL, 2, 1, 0),
    SOC_SINGLE!(c"IIR1 Band4 Switch", LPASS_CDC_IIR1_CTL, 3, 1, 0),
    SOC_SINGLE!(c"IIR1 Band5 Switch", LPASS_CDC_IIR1_CTL, 4, 1, 0),
    SOC_SINGLE!(c"IIR2 Band1 Switch", LPASS_CDC_IIR2_CTL, 0, 1, 0),
    SOC_SINGLE!(c"IIR2 Band2 Switch", LPASS_CDC_IIR2_CTL, 1, 1, 0),
    SOC_SINGLE!(c"IIR2 Band3 Switch", LPASS_CDC_IIR2_CTL, 2, 1, 0),
    SOC_SINGLE!(c"IIR2 Band4 Switch", LPASS_CDC_IIR2_CTL, 3, 1, 0),
    SOC_SINGLE!(c"IIR2 Band5 Switch", LPASS_CDC_IIR2_CTL, 4, 1, 0),
    WCD_IIR_FILTER_CTL!(c"IIR1 Band1", IIR1, BAND1),
    WCD_IIR_FILTER_CTL!(c"IIR1 Band2", IIR1, BAND2),
    WCD_IIR_FILTER_CTL!(c"IIR1 Band3", IIR1, BAND3),
    WCD_IIR_FILTER_CTL!(c"IIR1 Band4", IIR1, BAND4),
    WCD_IIR_FILTER_CTL!(c"IIR1 Band5", IIR1, BAND5),
    WCD_IIR_FILTER_CTL!(c"IIR2 Band1", IIR2, BAND1),
    WCD_IIR_FILTER_CTL!(c"IIR2 Band2", IIR2, BAND2),
    WCD_IIR_FILTER_CTL!(c"IIR2 Band3", IIR2, BAND3),
    WCD_IIR_FILTER_CTL!(c"IIR2 Band4", IIR2, BAND4),
    WCD_IIR_FILTER_CTL!(c"IIR2 Band5", IIR2, BAND5),
    SOC_SINGLE_S8_TLV!(c"IIR1 INP1 Volume", LPASS_CDC_IIR1_GAIN_B1_CTL, -84, 40, digital_gain),
    SOC_SINGLE_S8_TLV!(c"IIR1 INP2 Volume", LPASS_CDC_IIR1_GAIN_B2_CTL, -84, 40, digital_gain),
    SOC_SINGLE_S8_TLV!(c"IIR1 INP3 Volume", LPASS_CDC_IIR1_GAIN_B3_CTL, -84, 40, digital_gain),
    SOC_SINGLE_S8_TLV!(c"IIR1 INP4 Volume", LPASS_CDC_IIR1_GAIN_B4_CTL, -84, 40, digital_gain),
    SOC_SINGLE_S8_TLV!(c"IIR2 INP1 Volume", LPASS_CDC_IIR2_GAIN_B1_CTL, -84, 40, digital_gain),
    SOC_SINGLE_S8_TLV!(c"IIR2 INP2 Volume", LPASS_CDC_IIR2_GAIN_B2_CTL, -84, 40, digital_gain),
    SOC_SINGLE_S8_TLV!(c"IIR2 INP3 Volume", LPASS_CDC_IIR2_GAIN_B3_CTL, -84, 40, digital_gain),
    SOC_SINGLE_S8_TLV!(c"IIR2 INP4 Volume", LPASS_CDC_IIR2_GAIN_B4_CTL, -84, 40, digital_gain),
];

unsafe extern "C" fn msm8916_wcd_digital_enable_interpolator(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);

    match event {
        SND_SOC_DAPM_POST_PMU => {
            /* apply the digital gain after the interpolator is enabled */
            usleep_range(10000, 10100);
            let reg = rx_gain_reg[(*w).shift as usize] as c_uint;
            snd_soc_component_write(component, reg, snd_soc_component_read(component, reg));
        }
        SND_SOC_DAPM_POST_PMD => {
            snd_soc_component_update_bits(component, LPASS_CDC_CLK_RX_RESET_CTL, 1 << (*w).shift, 1 << (*w).shift);
            snd_soc_component_update_bits(component, LPASS_CDC_CLK_RX_RESET_CTL, 1 << (*w).shift, 0x0);
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn msm8916_wcd_digital_enable_dec(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let decimator: c_uint = (*w).shift as c_uint + 1;
    let dec_reset_reg: u16 = LPASS_CDC_CLK_TX_RESET_B1_CTL as u16;
    let tx_vol_ctl_reg: u16 = (LPASS_CDC_TX1_VOL_CTL_CFG + 32 * (decimator - 1)) as u16;
    let tx_mux_ctl_reg: u16 = (LPASS_CDC_TX1_MUX_CTL + 32 * (decimator - 1)) as u16;
    let mut dec_hpf_cut_of_freq: u8;

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            /* Enable TX digital mute */
            snd_soc_component_update_bits(component, tx_vol_ctl_reg as c_uint, TX_VOL_CTL_CFG_MUTE_EN_MASK, TX_VOL_CTL_CFG_MUTE_EN_ENABLE);
            dec_hpf_cut_of_freq = (snd_soc_component_read(component, tx_mux_ctl_reg as c_uint) & TX_MUX_CTL_CUT_OFF_FREQ_MASK) as u8;
            dec_hpf_cut_of_freq >>= TX_MUX_CTL_CUT_OFF_FREQ_SHIFT;
            if dec_hpf_cut_of_freq as u32 != TX_MUX_CTL_CF_NEG_3DB_150HZ {
                /* set cut of freq to CF_MIN_3DB_150HZ (0x1) */
                snd_soc_component_update_bits(component, tx_mux_ctl_reg as c_uint, TX_MUX_CTL_CUT_OFF_FREQ_MASK, TX_MUX_CTL_CF_NEG_3DB_150HZ);
            }
        }
        SND_SOC_DAPM_POST_PMU => {
            /* enable HPF */
            snd_soc_component_update_bits(component, tx_mux_ctl_reg as c_uint, TX_MUX_CTL_HPF_BP_SEL_MASK, TX_MUX_CTL_HPF_BP_SEL_NO_BYPASS);
            /* apply the digital gain after the decimator is enabled */
            let reg = tx_gain_reg[(*w).shift as usize] as c_uint;
            snd_soc_component_write(component, reg, snd_soc_component_read(component, reg));
            snd_soc_component_update_bits(component, tx_vol_ctl_reg as c_uint, TX_VOL_CTL_CFG_MUTE_EN_MASK, 0);
        }
        SND_SOC_DAPM_PRE_PMD => {
            snd_soc_component_update_bits(component, tx_vol_ctl_reg as c_uint, TX_VOL_CTL_CFG_MUTE_EN_MASK, TX_VOL_CTL_CFG_MUTE_EN_ENABLE);
            snd_soc_component_update_bits(component, tx_mux_ctl_reg as c_uint, TX_MUX_CTL_HPF_BP_SEL_MASK, TX_MUX_CTL_HPF_BP_SEL_BYPASS);
        }
        SND_SOC_DAPM_POST_PMD => {
            snd_soc_component_update_bits(component, dec_reset_reg as c_uint, 1 << (*w).shift, 1 << (*w).shift);
            snd_soc_component_update_bits(component, dec_reset_reg as c_uint, 1 << (*w).shift, 0x0);
            snd_soc_component_update_bits(component, tx_mux_ctl_reg as c_uint, TX_MUX_CTL_HPF_BP_SEL_MASK, TX_MUX_CTL_HPF_BP_SEL_BYPASS);
            snd_soc_component_update_bits(component, tx_vol_ctl_reg as c_uint, TX_VOL_CTL_CFG_MUTE_EN_MASK, 0);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn msm8916_wcd_digital_enable_dmic(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let mut dmic: c_uint = 0;
    let mut ret: c_int;
    /* get dmic number out of widget name */
    let dmic_num = strpbrk((*w).name, c"12".as_ptr());

    if dmic_num.is_null() {
        dev_err((*component).dev, c"Invalid DMIC\n".as_ptr());
        return -EINVAL;
    }
    ret = kstrtouint(dmic_num, 10, &mut dmic);
    if ret < 0 || dmic > 2 {
        dev_err((*component).dev, c"Invalid DMIC line on the component\n".as_ptr());
        return -EINVAL;
    }

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            snd_soc_component_update_bits(component, LPASS_CDC_CLK_DMIC_B1_CTL, DMIC_B1_CTL_DMIC0_CLK_SEL_MASK, DMIC_B1_CTL_DMIC0_CLK_SEL_DIV3);
            match dmic {
                1 => {
                    snd_soc_component_update_bits(component, LPASS_CDC_TX1_DMIC_CTL, TXN_DMIC_CTL_CLK_SEL_MASK, TXN_DMIC_CTL_CLK_SEL_DIV3);
                }
                2 => {
                    snd_soc_component_update_bits(component, LPASS_CDC_TX2_DMIC_CTL, TXN_DMIC_CTL_CLK_SEL_MASK, TXN_DMIC_CTL_CLK_SEL_DIV3);
                }
                _ => {}
            }
        }
        _ => {}
    }

    0
}

static iir_inp1_text: [*const c_char; 6] = c_str_array!(b"ZERO\0", b"DEC1\0", b"DEC2\0", b"RX1\0", b"RX2\0", b"RX3\0");

static iir1_inp1_mux_enum: soc_enum =
    SOC_ENUM_SINGLE!(LPASS_CDC_CONN_EQ1_B1_CTL, 0, 6, iir_inp1_text);
static iir2_inp1_mux_enum: soc_enum =
    SOC_ENUM_SINGLE!(LPASS_CDC_CONN_EQ2_B1_CTL, 0, 6, iir_inp1_text);

static iir1_inp1_mux: snd_kcontrol_new = SOC_DAPM_ENUM!(c"IIR1 INP1 Mux", iir1_inp1_mux_enum);
static iir2_inp1_mux: snd_kcontrol_new = SOC_DAPM_ENUM!(c"IIR2 INP1 Mux", iir2_inp1_mux_enum);

static msm8916_wcd_digital_dapm_widgets: &[snd_soc_dapm_widget] = &[
    /*RX stuff */
    SND_SOC_DAPM_AIF_IN!(c"I2S RX1", ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!(c"I2S RX2", ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!(c"I2S RX3", ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_OUTPUT!(c"PDM_RX1"),
    SND_SOC_DAPM_OUTPUT!(c"PDM_RX2"),
    SND_SOC_DAPM_OUTPUT!(c"PDM_RX3"),
    SND_SOC_DAPM_INPUT!(c"LPASS_PDM_TX"),
    SND_SOC_DAPM_MIXER!(c"RX1 MIX1", SND_SOC_NOPM, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_MIXER!(c"RX2 MIX1", SND_SOC_NOPM, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_MIXER!(c"RX3 MIX1", SND_SOC_NOPM, 0, 0, ptr::null(), 0),
    /* Interpolator */
    SND_SOC_DAPM_MIXER_E!(c"RX1 INT", LPASS_CDC_CLK_RX_B1_CTL, 0, 0, ptr::null(), 0, msm8916_wcd_digital_enable_interpolator, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MIXER_E!(c"RX2 INT", LPASS_CDC_CLK_RX_B1_CTL, 1, 0, ptr::null(), 0, msm8916_wcd_digital_enable_interpolator, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MIXER_E!(c"RX3 INT", LPASS_CDC_CLK_RX_B1_CTL, 2, 0, ptr::null(), 0, msm8916_wcd_digital_enable_interpolator, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MUX!(c"RX1 MIX1 INP1", SND_SOC_NOPM, 0, 0, &rx_mix1_inp1_mux),
    SND_SOC_DAPM_MUX!(c"RX1 MIX1 INP2", SND_SOC_NOPM, 0, 0, &rx_mix1_inp2_mux),
    SND_SOC_DAPM_MUX!(c"RX1 MIX1 INP3", SND_SOC_NOPM, 0, 0, &rx_mix1_inp3_mux),
    SND_SOC_DAPM_MUX!(c"RX2 MIX1 INP1", SND_SOC_NOPM, 0, 0, &rx2_mix1_inp1_mux),
    SND_SOC_DAPM_MUX!(c"RX2 MIX1 INP2", SND_SOC_NOPM, 0, 0, &rx2_mix1_inp2_mux),
    SND_SOC_DAPM_MUX!(c"RX2 MIX1 INP3", SND_SOC_NOPM, 0, 0, &rx2_mix1_inp3_mux),
    SND_SOC_DAPM_MUX!(c"RX3 MIX1 INP1", SND_SOC_NOPM, 0, 0, &rx3_mix1_inp1_mux),
    SND_SOC_DAPM_MUX!(c"RX3 MIX1 INP2", SND_SOC_NOPM, 0, 0, &rx3_mix1_inp2_mux),
    SND_SOC_DAPM_MUX!(c"RX3 MIX1 INP3", SND_SOC_NOPM, 0, 0, &rx3_mix1_inp3_mux),
    SND_SOC_DAPM_MUX!(c"RX1 MIX2 INP1", SND_SOC_NOPM, 0, 0, &rx1_mix2_inp1_mux),
    SND_SOC_DAPM_MUX!(c"RX2 MIX2 INP1", SND_SOC_NOPM, 0, 0, &rx2_mix2_inp1_mux),
    SND_SOC_DAPM_MUX!(c"CIC1 MUX", SND_SOC_NOPM, 0, 0, &cic1_mux),
    SND_SOC_DAPM_MUX!(c"CIC2 MUX", SND_SOC_NOPM, 0, 0, &cic2_mux),
    /* TX */
    SND_SOC_DAPM_MIXER!(c"ADC1", SND_SOC_NOPM, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_MIXER!(c"ADC2", SND_SOC_NOPM, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_MIXER!(c"ADC3", SND_SOC_NOPM, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_MUX_E!(c"DEC1 MUX", LPASS_CDC_CLK_TX_CLK_EN_B1_CTL, 0, 0, &dec1_mux, msm8916_wcd_digital_enable_dec, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MUX_E!(c"DEC2 MUX", LPASS_CDC_CLK_TX_CLK_EN_B1_CTL, 1, 0, &dec2_mux, msm8916_wcd_digital_enable_dec, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_AIF_OUT!(c"I2S TX1", ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!(c"I2S TX2", ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!(c"I2S TX3", ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    /* Digital Mic Inputs */
    SND_SOC_DAPM_ADC_E!(c"DMIC1", ptr::null(), SND_SOC_NOPM, 0, 0, msm8916_wcd_digital_enable_dmic, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_ADC_E!(c"DMIC2", ptr::null(), SND_SOC_NOPM, 0, 0, msm8916_wcd_digital_enable_dmic, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY!(c"DMIC_CLK", LPASS_CDC_CLK_DMIC_B1_CTL, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!(c"RX_I2S_CLK", LPASS_CDC_CLK_RX_I2S_CTL, 4, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!(c"TX_I2S_CLK", LPASS_CDC_CLK_TX_I2S_CTL, 4, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!(c"MCLK", SND_SOC_NOPM, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!(c"PDM_CLK", LPASS_CDC_CLK_PDM_CTL, 0, 0, ptr::null(), 0),
    /* Connectivity Clock */
    SND_SOC_DAPM_SUPPLY_S!(c"CDC_CONN", -2, LPASS_CDC_CLK_OTHR_CTL, 2, 0, ptr::null(), 0),
    SND_SOC_DAPM_MIC!(c"Digital Mic1", ptr::null()),
    SND_SOC_DAPM_MIC!(c"Digital Mic2", ptr::null()),
    /* Sidetone */
    SND_SOC_DAPM_MUX!(c"IIR1 INP1 MUX", SND_SOC_NOPM, 0, 0, &iir1_inp1_mux),
    SND_SOC_DAPM_PGA_E!(c"IIR1", LPASS_CDC_CLK_SD_CTL, 0, 0, ptr::null(), 0, msm8x16_wcd_codec_set_iir_gain, SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_MUX!(c"IIR2 INP1 MUX", SND_SOC_NOPM, 0, 0, &iir2_inp1_mux),
    SND_SOC_DAPM_PGA_E!(c"IIR2", LPASS_CDC_CLK_SD_CTL, 1, 0, ptr::null(), 0, msm8x16_wcd_codec_set_iir_gain, SND_SOC_DAPM_POST_PMU),
];

unsafe extern "C" fn msm8916_wcd_digital_get_clks(
    pdev: *mut platform_device,
    priv_: *mut msm8916_wcd_digital_priv,
) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;

    (*priv_).ahbclk = devm_clk_get(dev, c"ahbix-clk".as_ptr());
    if IS_ERR((*priv_).ahbclk as *const c_void) {
        dev_err(dev, c"failed to get ahbix clk\n".as_ptr());
        return PTR_ERR((*priv_).ahbclk as *const c_void) as c_int;
    }

    (*priv_).mclk = devm_clk_get(dev, c"mclk".as_ptr());
    if IS_ERR((*priv_).mclk as *const c_void) {
        dev_err(dev, c"failed to get mclk\n".as_ptr());
        return PTR_ERR((*priv_).mclk as *const c_void) as c_int;
    }

    0
}

unsafe extern "C" fn msm8916_wcd_digital_component_probe(
    component: *mut snd_soc_component,
) -> c_int {
    let priv_ = dev_get_drvdata((*component).dev) as *mut msm8916_wcd_digital_priv;
    snd_soc_component_set_drvdata(component, priv_ as *mut c_void);
    0
}

unsafe extern "C" fn msm8916_wcd_digital_component_set_sysclk(
    component: *mut snd_soc_component,
    _clk_id: c_int,
    _source: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let p = dev_get_drvdata((*component).dev) as *mut msm8916_wcd_digital_priv;
    clk_set_rate((*p).mclk, freq as c_ulong)
}

unsafe extern "C" fn msm8916_wcd_digital_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let tx_fs_rate: u8;
    let rx_fs_rate: u8;

    match params_rate(params) {
        8000 => {
            tx_fs_rate = TX_I2S_CTL_TX_I2S_FS_RATE_F_8_KHZ as u8;
            rx_fs_rate = RX_I2S_CTL_RX_I2S_FS_RATE_F_8_KHZ as u8;
        }
        16000 => {
            tx_fs_rate = TX_I2S_CTL_TX_I2S_FS_RATE_F_16_KHZ as u8;
            rx_fs_rate = RX_I2S_CTL_RX_I2S_FS_RATE_F_16_KHZ as u8;
        }
        32000 => {
            tx_fs_rate = TX_I2S_CTL_TX_I2S_FS_RATE_F_32_KHZ as u8;
            rx_fs_rate = RX_I2S_CTL_RX_I2S_FS_RATE_F_32_KHZ as u8;
        }
        48000 => {
            tx_fs_rate = TX_I2S_CTL_TX_I2S_FS_RATE_F_48_KHZ as u8;
            rx_fs_rate = RX_I2S_CTL_RX_I2S_FS_RATE_F_48_KHZ as u8;
        }
        _ => {
            dev_err((*(*dai).component).dev, c"Invalid sampling rate %d\n".as_ptr(), params_rate(params));
            return -EINVAL;
        }
    }

    match (*substream).stream {
        SNDRV_PCM_STREAM_CAPTURE => {
            snd_soc_component_update_bits((*dai).component, LPASS_CDC_CLK_TX_I2S_CTL, TX_I2S_CTL_TX_I2S_FS_RATE_MASK, tx_fs_rate as c_uint);
        }
        SNDRV_PCM_STREAM_PLAYBACK => {
            snd_soc_component_update_bits((*dai).component, LPASS_CDC_CLK_RX_I2S_CTL, RX_I2S_CTL_RX_I2S_FS_RATE_MASK, rx_fs_rate as c_uint);
        }
        _ => return -EINVAL,
    }

    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => {
            snd_soc_component_update_bits((*dai).component, LPASS_CDC_CLK_TX_I2S_CTL, TX_I2S_CTL_TX_I2S_MODE_MASK, TX_I2S_CTL_TX_I2S_MODE_16);
            snd_soc_component_update_bits((*dai).component, LPASS_CDC_CLK_RX_I2S_CTL, RX_I2S_CTL_RX_I2S_MODE_MASK, RX_I2S_CTL_RX_I2S_MODE_16);
        }
        SNDRV_PCM_FORMAT_S32_LE => {
            snd_soc_component_update_bits((*dai).component, LPASS_CDC_CLK_TX_I2S_CTL, TX_I2S_CTL_TX_I2S_MODE_MASK, TX_I2S_CTL_TX_I2S_MODE_32);
            snd_soc_component_update_bits((*dai).component, LPASS_CDC_CLK_RX_I2S_CTL, RX_I2S_CTL_RX_I2S_MODE_MASK, RX_I2S_CTL_RX_I2S_MODE_32);
        }
        _ => {
            dev_err((*dai).dev, c"%s: wrong format selected\n".as_ptr(), c"msm8916_wcd_digital_hw_params".as_ptr());
            return -EINVAL;
        }
    }

    0
}

static msm8916_wcd_digital_audio_map: &[snd_soc_dapm_route] = &[
    route!(c"I2S RX1", ptr::null(), c"AIF1 Playback"), route!(c"I2S RX2", ptr::null(), c"AIF1 Playback"), route!(c"I2S RX3", ptr::null(), c"AIF1 Playback"),
    route!(c"AIF1 Capture", ptr::null(), c"I2S TX1"), route!(c"AIF1 Capture", ptr::null(), c"I2S TX2"), route!(c"AIF1 Capture", ptr::null(), c"I2S TX3"),
    route!(c"CIC1 MUX", c"DMIC", c"DEC1 MUX"), route!(c"CIC1 MUX", c"AMIC", c"DEC1 MUX"), route!(c"CIC2 MUX", c"DMIC", c"DEC2 MUX"), route!(c"CIC2 MUX", c"AMIC", c"DEC2 MUX"),
    /* Decimator Inputs */
    route!(c"DEC1 MUX", c"DMIC1", c"DMIC1"), route!(c"DEC1 MUX", c"DMIC2", c"DMIC2"), route!(c"DEC1 MUX", c"ADC1", c"ADC1"), route!(c"DEC1 MUX", c"ADC2", c"ADC2"), route!(c"DEC1 MUX", c"ADC3", c"ADC3"), route!(c"DEC1 MUX", ptr::null(), c"CDC_CONN"),
    route!(c"DEC2 MUX", c"DMIC1", c"DMIC1"), route!(c"DEC2 MUX", c"DMIC2", c"DMIC2"), route!(c"DEC2 MUX", c"ADC1", c"ADC1"), route!(c"DEC2 MUX", c"ADC2", c"ADC2"), route!(c"DEC2 MUX", c"ADC3", c"ADC3"), route!(c"DEC2 MUX", ptr::null(), c"CDC_CONN"),
    route!(c"DMIC1", ptr::null(), c"DMIC_CLK"), route!(c"DMIC2", ptr::null(), c"DMIC_CLK"),
    route!(c"I2S TX1", ptr::null(), c"CIC1 MUX"), route!(c"I2S TX2", ptr::null(), c"CIC2 MUX"),
    route!(c"I2S TX1", ptr::null(), c"TX_I2S_CLK"), route!(c"I2S TX2", ptr::null(), c"TX_I2S_CLK"),
    route!(c"TX_I2S_CLK", ptr::null(), c"MCLK"), route!(c"TX_I2S_CLK", ptr::null(), c"PDM_CLK"),
    route!(c"ADC1", ptr::null(), c"LPASS_PDM_TX"), route!(c"ADC2", ptr::null(), c"LPASS_PDM_TX"), route!(c"ADC3", ptr::null(), c"LPASS_PDM_TX"),
    route!(c"I2S RX1", ptr::null(), c"RX_I2S_CLK"), route!(c"I2S RX2", ptr::null(), c"RX_I2S_CLK"), route!(c"I2S RX3", ptr::null(), c"RX_I2S_CLK"),
    route!(c"RX_I2S_CLK", ptr::null(), c"PDM_CLK"), route!(c"RX_I2S_CLK", ptr::null(), c"MCLK"), route!(c"RX_I2S_CLK", ptr::null(), c"CDC_CONN"),
    /* RX1 PATH.. */
    route!(c"PDM_RX1", ptr::null(), c"RX1 INT"), route!(c"RX1 INT", ptr::null(), c"RX1 MIX1"),
    route!(c"RX1 MIX1", ptr::null(), c"RX1 MIX1 INP1"), route!(c"RX1 MIX1", ptr::null(), c"RX1 MIX1 INP2"), route!(c"RX1 MIX1", ptr::null(), c"RX1 MIX1 INP3"),
    route!(c"RX1 MIX1 INP1", c"RX1", c"I2S RX1"), route!(c"RX1 MIX1 INP1", c"RX2", c"I2S RX2"), route!(c"RX1 MIX1 INP1", c"RX3", c"I2S RX3"), route!(c"RX1 MIX1 INP1", c"IIR1", c"IIR1"), route!(c"RX1 MIX1 INP1", c"IIR2", c"IIR2"),
    route!(c"RX1 MIX1 INP2", c"RX1", c"I2S RX1"), route!(c"RX1 MIX1 INP2", c"RX2", c"I2S RX2"), route!(c"RX1 MIX1 INP2", c"RX3", c"I2S RX3"), route!(c"RX1 MIX1 INP2", c"IIR1", c"IIR1"), route!(c"RX1 MIX1 INP2", c"IIR2", c"IIR2"),
    route!(c"RX1 MIX1 INP3", c"RX1", c"I2S RX1"), route!(c"RX1 MIX1 INP3", c"RX2", c"I2S RX2"), route!(c"RX1 MIX1 INP3", c"RX3", c"I2S RX3"),
    /* RX2 PATH */
    route!(c"PDM_RX2", ptr::null(), c"RX2 INT"), route!(c"RX2 INT", ptr::null(), c"RX2 MIX1"),
    route!(c"RX2 MIX1", ptr::null(), c"RX2 MIX1 INP1"), route!(c"RX2 MIX1", ptr::null(), c"RX2 MIX1 INP2"), route!(c"RX2 MIX1", ptr::null(), c"RX2 MIX1 INP3"),
    route!(c"RX2 MIX1 INP1", c"RX1", c"I2S RX1"), route!(c"RX2 MIX1 INP1", c"RX2", c"I2S RX2"), route!(c"RX2 MIX1 INP1", c"RX3", c"I2S RX3"), route!(c"RX2 MIX1 INP1", c"IIR1", c"IIR1"), route!(c"RX2 MIX1 INP1", c"IIR2", c"IIR2"),
    route!(c"RX2 MIX1 INP2", c"RX1", c"I2S RX1"), route!(c"RX2 MIX1 INP2", c"RX2", c"I2S RX2"), route!(c"RX2 MIX1 INP2", c"RX3", c"I2S RX3"), route!(c"RX2 MIX1 INP1", c"IIR1", c"IIR1"), route!(c"RX2 MIX1 INP1", c"IIR2", c"IIR2"),
    route!(c"RX2 MIX1 INP3", c"RX1", c"I2S RX1"), route!(c"RX2 MIX1 INP3", c"RX2", c"I2S RX2"), route!(c"RX2 MIX1 INP3", c"RX3", c"I2S RX3"),
    /* RX3 PATH */
    route!(c"PDM_RX3", ptr::null(), c"RX3 INT"), route!(c"RX3 INT", ptr::null(), c"RX3 MIX1"),
    route!(c"RX3 MIX1", ptr::null(), c"RX3 MIX1 INP1"), route!(c"RX3 MIX1", ptr::null(), c"RX3 MIX1 INP2"), route!(c"RX3 MIX1", ptr::null(), c"RX3 MIX1 INP3"),
    route!(c"RX3 MIX1 INP1", c"RX1", c"I2S RX1"), route!(c"RX3 MIX1 INP1", c"RX2", c"I2S RX2"), route!(c"RX3 MIX1 INP1", c"RX3", c"I2S RX3"), route!(c"RX3 MIX1 INP1", c"IIR1", c"IIR1"), route!(c"RX3 MIX1 INP1", c"IIR2", c"IIR2"),
    route!(c"RX3 MIX1 INP2", c"RX1", c"I2S RX1"), route!(c"RX3 MIX1 INP2", c"RX2", c"I2S RX2"), route!(c"RX3 MIX1 INP2", c"RX3", c"I2S RX3"), route!(c"RX3 MIX1 INP2", c"IIR1", c"IIR1"), route!(c"RX3 MIX1 INP2", c"IIR2", c"IIR2"),
    route!(c"RX1 MIX2 INP1", c"IIR1", c"IIR1"), route!(c"RX2 MIX2 INP1", c"IIR1", c"IIR1"), route!(c"RX1 MIX2 INP1", c"IIR2", c"IIR2"), route!(c"RX2 MIX2 INP1", c"IIR2", c"IIR2"),
    route!(c"IIR1", ptr::null(), c"IIR1 INP1 MUX"), route!(c"IIR1 INP1 MUX", c"DEC1", c"DEC1 MUX"), route!(c"IIR1 INP1 MUX", c"DEC2", c"DEC2 MUX"),
    route!(c"IIR2", ptr::null(), c"IIR2 INP1 MUX"), route!(c"IIR2 INP1 MUX", c"DEC1", c"DEC1 MUX"), route!(c"IIR2 INP1 MUX", c"DEC2", c"DEC2 MUX"),
    route!(c"RX3 MIX1 INP3", c"RX1", c"I2S RX1"), route!(c"RX3 MIX1 INP3", c"RX2", c"I2S RX2"), route!(c"RX3 MIX1 INP3", c"RX3", c"I2S RX3"),
];

unsafe extern "C" fn msm8916_wcd_digital_startup(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let msm8916_wcd = snd_soc_component_get_drvdata(component) as *mut msm8916_wcd_digital_priv;
    let mclk_rate: c_ulong;

    snd_soc_component_update_bits(component, LPASS_CDC_CLK_MCLK_CTL, MCLK_CTL_MCLK_EN_MASK, MCLK_CTL_MCLK_EN_ENABLE);
    snd_soc_component_update_bits(component, LPASS_CDC_CLK_PDM_CTL, LPASS_CDC_CLK_PDM_CTL_PDM_CLK_SEL_MASK, LPASS_CDC_CLK_PDM_CTL_PDM_CLK_SEL_FB);

    mclk_rate = clk_get_rate((*msm8916_wcd).mclk);
    match mclk_rate {
        12288000 => {
            snd_soc_component_update_bits(component, LPASS_CDC_TOP_CTL, TOP_CTL_DIG_MCLK_FREQ_MASK, TOP_CTL_DIG_MCLK_FREQ_F_12_288MHZ);
        }
        9600000 => {
            snd_soc_component_update_bits(component, LPASS_CDC_TOP_CTL, TOP_CTL_DIG_MCLK_FREQ_MASK, TOP_CTL_DIG_MCLK_FREQ_F_9_6MHZ);
        }
        _ => {
            dev_err((*component).dev, c"Invalid mclk rate %ld\n".as_ptr(), mclk_rate);
        }
    }
    0
}

unsafe extern "C" fn msm8916_wcd_digital_shutdown(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    snd_soc_component_update_bits((*dai).component, LPASS_CDC_CLK_PDM_CTL, LPASS_CDC_CLK_PDM_CTL_PDM_CLK_SEL_MASK, 0);
}

static msm8916_wcd_digital_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(msm8916_wcd_digital_startup),
    shutdown: Some(msm8916_wcd_digital_shutdown),
    hw_params: Some(msm8916_wcd_digital_hw_params),
    ..snd_soc_dai_ops::zeroed()
};

static mut msm8916_wcd_digital_dai: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c"msm8916_wcd_digital_i2s_rx1".as_ptr(),
        id: 0,
        playback: snd_soc_pcm_stream {
            stream_name: c"AIF1 Playback".as_ptr(),
            rates: MSM8916_WCD_DIGITAL_RATES,
            formats: MSM8916_WCD_DIGITAL_FORMATS,
            channels_min: 1,
            channels_max: 3,
            ..snd_soc_pcm_stream::zeroed()
        },
        ops: &msm8916_wcd_digital_dai_ops,
        ..snd_soc_dai_driver::zeroed()
    },
    snd_soc_dai_driver {
        name: c"msm8916_wcd_digital_i2s_tx1".as_ptr(),
        id: 1,
        capture: snd_soc_pcm_stream {
            stream_name: c"AIF1 Capture".as_ptr(),
            rates: MSM8916_WCD_DIGITAL_RATES,
            formats: MSM8916_WCD_DIGITAL_FORMATS,
            channels_min: 1,
            channels_max: 4,
            ..snd_soc_pcm_stream::zeroed()
        },
        ops: &msm8916_wcd_digital_dai_ops,
        ..snd_soc_dai_driver::zeroed()
    },
];

static msm8916_wcd_digital: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(msm8916_wcd_digital_component_probe),
    set_sysclk: Some(msm8916_wcd_digital_component_set_sysclk),
    controls: msm8916_wcd_digital_snd_controls.as_ptr(),
    num_controls: msm8916_wcd_digital_snd_controls.len() as c_uint,
    dapm_widgets: msm8916_wcd_digital_dapm_widgets.as_ptr(),
    num_dapm_widgets: msm8916_wcd_digital_dapm_widgets.len() as c_uint,
    dapm_routes: msm8916_wcd_digital_audio_map.as_ptr(),
    num_dapm_routes: msm8916_wcd_digital_audio_map.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
    ..snd_soc_component_driver::zeroed()
};

static msm8916_codec_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: LPASS_CDC_TX2_DMIC_CTL,
    cache_type: REGCACHE_FLAT,
    ..regmap_config::zeroed()
};

unsafe extern "C" fn msm8916_wcd_digital_probe(pdev: *mut platform_device) -> c_int {
    let mut priv_: *mut msm8916_wcd_digital_priv;
    let dev = &mut (*pdev).dev as *mut device;
    let base: *mut c_void;
    let digital_map: *mut regmap;
    let mut ret: c_int;

    priv_ = devm_kzalloc(dev, size_of::<msm8916_wcd_digital_priv>(), GFP_KERNEL) as *mut msm8916_wcd_digital_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base as *const c_void) {
        return PTR_ERR(base as *const c_void) as c_int;
    }

    digital_map = devm_regmap_init_mmio(&mut (*pdev).dev, base, &msm8916_codec_regmap_config);
    if IS_ERR(digital_map as *const c_void) {
        return PTR_ERR(digital_map as *const c_void) as c_int;
    }

    ret = msm8916_wcd_digital_get_clks(pdev, priv_);
    if ret < 0 {
        return ret;
    }

    ret = clk_prepare_enable((*priv_).ahbclk);
    if ret < 0 {
        dev_err(dev, c"failed to enable ahbclk %d\n".as_ptr(), ret);
        return ret;
    }

    ret = clk_prepare_enable((*priv_).mclk);
    if ret < 0 {
        dev_err(dev, c"failed to enable mclk %d\n".as_ptr(), ret);
        clk_disable_unprepare((*priv_).ahbclk);
        return ret;
    }

    dev_set_drvdata(dev, priv_ as *mut c_void);

    ret = devm_snd_soc_register_component(
        dev,
        &msm8916_wcd_digital,
        msm8916_wcd_digital_dai.as_mut_ptr(),
        msm8916_wcd_digital_dai.len() as c_int,
    );
    if ret != 0 {
        clk_disable_unprepare((*priv_).mclk);
        clk_disable_unprepare((*priv_).ahbclk);
        return ret;
    }

    0
}

unsafe extern "C" fn msm8916_wcd_digital_remove(pdev: *mut platform_device) {
    let priv_ = dev_get_drvdata(&mut (*pdev).dev) as *mut msm8916_wcd_digital_priv;

    clk_disable_unprepare((*priv_).mclk);
    clk_disable_unprepare((*priv_).ahbclk);
}

static msm8916_wcd_digital_match_table: [of_device_id; 2] = [
    of_device_id {
        compatible: c"qcom,msm8916-wcd-digital-codec".as_ptr(),
        ..of_device_id::zeroed()
    },
    of_device_id::zeroed(),
];

MODULE_DEVICE_TABLE!(of, msm8916_wcd_digital_match_table);

static mut msm8916_wcd_digital_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"msm8916-wcd-digital-codec".as_ptr(),
        of_match_table: msm8916_wcd_digital_match_table.as_ptr(),
        ..device_driver::zeroed()
    },
    probe: Some(msm8916_wcd_digital_probe),
    remove: Some(msm8916_wcd_digital_remove),
    ..platform_driver::zeroed()
};

module_platform_driver!(msm8916_wcd_digital_driver);

MODULE_AUTHOR!(c"Srinivas Kandagatla <srinivas.kandagatla@linaro.org>");
MODULE_DESCRIPTION!(c"MSM8916 WCD Digital Codec driver");
MODULE_LICENSE!(c"GPL v2");

extern "C" {
    static SNDRV_PCM_RATE_8000: u32;
    static SNDRV_PCM_RATE_16000: u32;
    static SNDRV_PCM_RATE_32000: u32;
    static SNDRV_PCM_RATE_48000: u32;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
