// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

/* WSA885X codec driver */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

type u8 = u8;
type u32 = u32;
type size_t = usize;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ERANGE: c_int = 34;
const ETIMEDOUT: c_int = 110;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_HIGH: c_uint = 1;
const IRQF_ONESHOT: c_ulong = 0x00002000;
const REGCACHE_MAPLE: c_uint = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SND_SOC_NOPM: c_uint = 0;
const SNDRV_PCM_RATE_8000: c_uint = 1 << 0;
const SNDRV_PCM_RATE_16000: c_uint = 1 << 1;
const SNDRV_PCM_RATE_32000: c_uint = 1 << 2;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 3;
const SNDRV_PCM_RATE_96000: c_uint = 1 << 4;
const SNDRV_PCM_RATE_192000: c_uint = 1 << 5;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << 0;
const SNDRV_PCM_FMTBIT_S24_LE: c_uint = 1 << 1;
const SNDRV_PCM_FMTBIT_S32_LE: c_uint = 1 << 2;

const fn bit(n: c_uint) -> c_uint { 1u32 << n }
const fn genmask(h: c_uint, l: c_uint) -> c_uint {
    ((!0u32) << l) & ((!0u32) >> (31 - h))
}
const fn field_shift(mask: c_uint) -> c_uint { mask.trailing_zeros() }
const fn field_prep(mask: c_uint, val: c_uint) -> c_uint { (val << field_shift(mask)) & mask }

/* Control Registers - Audio Processing */
const WSA885X_SMP_AMP_CTRL_STEREO_STEREO_SMP_AMP_CTRL_I2S: c_uint = 0x0000;
const WSA885X_SMP_AMP_CTRL_STEREO_CMT_GRP_MASK: c_uint = 0x0004;
const WSA885X_SMP_AMP_CTRL_STEREO_IT21_CLUSERINDEX: c_uint = 0x0140;
const WSA885X_SMP_AMP_CTRL_STEREO_CS21_CLOCK_VALID: c_uint = 0x0208;
const WSA885X_SMP_AMP_CTRL_STEREO_CS21_SAMPLERATEINDEX: c_uint = 0x0240;
const WSA885X_SMP_AMP_CTRL_STEREO_PPU21_POSTURENUMBER: c_uint = 0x0340;
const WSA885X_SMP_AMP_CTRL_STEREO_FU21_MUTE_CH2X0: c_uint = 0x4405;
const WSA885X_SMP_AMP_CTRL_STEREO_FU21_MUTE_CH2X1: c_uint = 0x4406;
const WSA885X_SMP_AMP_CTRL_STEREO_FU21_CH_VOL_CH2X0_LSB: c_uint = 0x4409;
const WSA885X_SMP_AMP_CTRL_STEREO_FU21_CH_VOL_CH2X0_MSB: c_uint = 0x6409;
const WSA885X_SMP_AMP_CTRL_STEREO_FU21_CH_VOL_CH2X1_LSB: c_uint = 0x440a;
const WSA885X_SMP_AMP_CTRL_STEREO_FU21_CH_VOL_CH2X1_MSB: c_uint = 0x640a;
const WSA885X_SMP_AMP_CTRL_STEREO_PDE23_REQ_PS: c_uint = 0x0a04;
const WSA885X_SMP_AMP_CTRL_STEREO_PDE23_ACT_PS: c_uint = 0x0a40;
const WSA885X_SMP_AMP_CTRL_STEREO_OT23_USAGE: c_uint = 0x0b10;
const WSA885X_SMP_AMP_CTRL_STEREO_CS24_SAMPLERATEINDEX: c_uint = 0x0e40;

/* Analog Top Registers - Power and Clock Control */
const WSA885X_ANA_TOP_PON_CKSK_CTL_0: c_uint = 0x800d;
const WSA885X_ANA_TOP_BG_TVP_UVLO1_PROG: c_uint = 0x8024;
const WSA885X_ANA_TOP_BG_TVP_UVLO2_PROG: c_uint = 0x8025;
const WSA885X_ANA_TOP_BG_TVP_OVRD_CTL: c_uint = 0x8034;

/* Analog PLL Registers */
const WSA885X_ANA_PLL_DIV_CTL_0: c_uint = 0x8090;
const WSA885X_ANA_PLL_DIV_CTL_1: c_uint = 0x8091;
const WSA885X_ANA_TOP_PLL_VCO_CTL: c_uint = 0x8092;
const WSA885X_ANA_TOP_PLL_LOOPFILT_0: c_uint = 0x8093;
const WSA885X_ANA_TOP_PLL_OVRD_CTL: c_uint = 0x8098;
const WSA885X_ANA_TOP_PLL_STATUS_0: c_uint = 0x809a;
const WSA885X_ANA_TOP_PLL_STATUS_1: c_uint = 0x809b;

/* Analog Boost Control Registers */
const WSA885X_ANA_TOP_BOOST_STB_CTRL2: c_uint = 0x805b;
const WSA885X_ANA_TOP_BOOST_STB_CTRL3: c_uint = 0x805c;
const WSA885X_ANA_TOP_BOOST_BYP_CTRL2: c_uint = 0x805e;
const WSA885X_ANA_TOP_BOOST_BYP_CTRL3: c_uint = 0x805f;
const WSA885X_ANA_TOP_BOOST_MISC: c_uint = 0x8063;
const WSA885X_ANA_TOP_BOOST_PWRSTAGE_CTRL2: c_uint = 0x8065;
const WSA885X_ANA_TOP_BOOST_PWRSTAGE_CTRL4: c_uint = 0x8067;

/* Analog IV Sense ADC Registers */
const WSA885X_ANA_TOP_IVSENSE_ADC_MODE_CTL2: c_uint = 0x80ca;
const WSA885X_ANA_TOP_IVSENSE_ADC_MODE_CTL3: c_uint = 0x80cb;
const WSA885X_ANA_TOP_IVSENSE_ADC_REF_CTL: c_uint = 0x80cc;
const WSA885X_ANA_TOP_IVSENSE_ADC_CDAC_CAL_CTL2: c_uint = 0x80d0;

/* Analog Speaker Power Stage Registers */
const WSA885X_ANA_TOP_SPK_TOP_PWRSTG_CH1_CTRL3: c_uint = 0x8108;
const WSA885X_ANA_TOP_SPK_TOP_PWRSTG_CH1_TUNE3: c_uint = 0x810b;
const WSA885X_ANA_TOP_SPK_TOP_PWRSTG_CH2_CTRL3: c_uint = 0x810e;
const WSA885X_ANA_TOP_SPK_TOP_PWRSTG_CH2_TUNE3: c_uint = 0x8111;
const WSA885X_ANA_TOP_SPK_TOP_SPARE3: c_uint = 0x813c;
const WSA885X_SPK_TOP_LF_CH1_CTRL11: c_uint = 0x811c;
const WSA885X_SPK_TOP_LF_CH1_TUNE1: c_uint = 0x811d;
const WSA885X_SPK_TOP_LF_CH2_TUNE1: c_uint = 0x8129;
const WSA885X_SPK_TOP_LF_CH1_CTRL9: c_uint = 0x811a;
const WSA885X_SPK_TOP_LF_CH2_CTRL9: c_uint = 0x8126;
const WSA885X_SPK_TOP_LF_CH2_CTRL11: c_uint = 0x8128;
const WSA885X_SPK_TOP_COMMON_CTRL2: c_uint = 0x8102;
const WSA885X_SPK_TOP_COMMON_TUNE1: c_uint = 0x8103;
const WSA885X_IVSENSE_VSNS_ISNS_CTL_CH1: c_uint = 0x80ba;
const WSA885X_DIG_CTRL0_TOP_CLK_CFG: c_uint = 0x8418;
const WSA885X_DIG_CTRL0_SDCA_COMMIT: c_uint = 0x8419;
const WSA885X_DIG_CTRL0_CLK_SOURCE_ENABLE: c_uint = 0x841a;
const WSA885X_DIG_CTRL0_SYS_CLK_SEL: c_uint = 0x841b;
const WSA885X_DIG_CTRL0_CDC_CLK_CTL: c_uint = 0x841c;
const WSA885X_DIG_CTRL0_PA_FSM_CTL: c_uint = 0x8420;
const WSA885X_DIG_CTRL0_POWER_FSM_CTL0: c_uint = 0x8423;
const WSA885X_DIG_CTRL0_POWER_FSM_CTL1: c_uint = 0x8424;
const WSA885X_DIG_CTRL0_PA0_FSM_CTL1: c_uint = 0x842b;
const WSA885X_DIG_CTRL0_PA1_FSM_CTL1: c_uint = 0x8435;
const WSA885X_DIG_CTRL0_VBAT_THRM_FLT_CTL: c_uint = 0x8458;
const WSA885X_DIG_CTRL0_CDC_RXTX_FSCNT_CTL: c_uint = 0x8470;
const WSA885X_DIG_CTRL0_GAIN_RAMP0_CTL1: c_uint = 0x84b4;
const WSA885X_DIG_CTRL0_GAIN_RAMP1_CTL1: c_uint = 0x84b7;
const WSA885X_DIG_CTRL0_PCM_DATA_WD0_CTL1: c_uint = 0x84A0;
const WSA885X_DIG_CTRL0_PCM_DATA_WD1_CTL1: c_uint = 0x84A4;

/* Digital Control 1 Registers - I2S/TDM Interface */
const WSA885X_DIG_CTRL1_I2S_CTL0: c_uint = 0x85A0;
const WSA885X_DIG_CTRL1_I2S_CFG0_TDM_TX: c_uint = 0x85A2;
const WSA885X_DIG_CTRL1_I2S_CFG1_TDM_TX: c_uint = 0x85A3;
const WSA885X_DIG_CTRL1_I2S_TDM_CTL0: c_uint = 0x85A7;
const WSA885X_DIG_CTRL1_I2S_TDM_CTL1: c_uint = 0x85A9;
const WSA885X_DIG_CTRL1_I2S_TDM_CH_RX: c_uint = 0x85AA;
const WSA885X_DIG_CTRL1_I2S_TDM_CH_TX: c_uint = 0x85AB;
const WSA885X_DIG_CTRL1_I2S_RESET_CTL: c_uint = 0x85AE;

/* CDC RX Path Registers - Audio Data Path */
const WSA885X_CDC_RX0_RX_PATH_CFG0: c_uint = 0x8601;
const WSA885X_CDC_RX0_RX_PATH_CFG1: c_uint = 0x8602;
const WSA885X_CDC_RX0_RX_PATH_CTL: c_uint = 0x8606;
const WSA885X_RX0_RX_PATH_DSMDEM_CTL: c_uint = 0x8613;
const WSA885X_CDC_RX1_RX_PATH_CFG0: c_uint = 0x8621;
const WSA885X_CDC_RX1_RX_PATH_CFG1: c_uint = 0x8622;
const WSA885X_CDC_RX1_RX_PATH_CTL: c_uint = 0x8626;
const WSA885X_RX1_RX_PATH_DSMDEM_CTL: c_uint = 0x8633;

/* CDC Compander Registers - Dynamic Range Control */
const WSA885X_CDC_COMPANDER0_CTL0: c_uint = 0x8640;
const WSA885X_CDC_COMPANDER0_CTL7: c_uint = 0x8647;
const WSA885X_CDC_COMPANDER1_CTL0: c_uint = 0x8660;
const WSA885X_CDC_COMPANDER1_CTL7: c_uint = 0x8667;

/* CDC Speaker Protection Registers - IV Sense */
const WSA885X_CDC_VSENSE0_SPKR_PROT_PATH_CTL: c_uint = 0x86A1;
const WSA885X_CDC_VSENSE1_SPKR_PROT_PATH_CTL: c_uint = 0x86B1;
const WSA885X_CDC_ISENSE0_SPKR_PROT_PATH_CTL: c_uint = 0x86A9;
const WSA885X_CDC_ISENSE1_SPKR_PROT_PATH_CTL: c_uint = 0x86B9;

/* CDC Class-H Registers - Headroom Control */
const WSA885X_CDC_CLSH_V1P8_BP_CTL1: c_uint = 0x86CD;
const WSA885X_CDC_CLSH_V1P8_BP_CTL0: c_uint = 0x86CC;
const WSA885X_CDC_CLSH_CLSH_SIG_DP_CTL0: c_uint = 0x86C7;
const WSA885X_CDC_CLSH_CLSH_V_HD_PA: c_uint = 0x86C3;
const WSA885X_CDC_CLSH_V1P8_BP_CTL2: c_uint = 0x86CE;

/* Driver Constants */
const WSA885X_CLK_RATE_FIXED: c_uint = 73728000;
const WSA885X_NUM_REGS: usize = 0x03;

/* Interrupt Registers */
const WSA885X_INTR_STATUS0: c_uint = 0x8584;
const WSA885X_INTR_MASK0: c_uint = 0x8581;
const WSA885X_INTR_CLEAR0: c_uint = 0x8587;

/* Power and PA FSM Control Registers */
const WSA885X_PA0_FSM_CTL0: c_uint = 0x842A;
const WSA885X_PA1_FSM_CTL0: c_uint = 0x8434;

/* Digital Control GPIO and Interrupt Registers */
const WSA885X_DIG_CTRL1_PIN_CT: c_uint = 0x8510;
const WSA885X_DIG_CTRL1_SPMI_PAD_GPIO2_CTL: c_uint = 0x8518;
const WSA885X_DIG_CTRL1_INTR_MODE: c_uint = 0x8580;

const WSA885X_I2S_CTL0_PCM_RATE_MASK: c_uint = genmask(4, 1);
const WSA885X_I2S_CTL0_ENABLE_MASK: c_uint = bit(0);
const fn WSA885X_I2S_CTL0_PCM_RATE(v: c_uint) -> c_uint { field_prep(WSA885X_I2S_CTL0_PCM_RATE_MASK, v) }
const WSA885X_I2S_CTL0_PCM_RATE_8KHZ: c_uint = 0x0;
const WSA885X_I2S_CTL0_PCM_RATE_16KHZ: c_uint = 0x1;
const WSA885X_I2S_CTL0_PCM_RATE_32KHZ: c_uint = 0x2;
const WSA885X_I2S_CTL0_PCM_RATE_48_OR_44KHZ: c_uint = 0x3;
const WSA885X_I2S_CTL0_PCM_RATE_96_OR_88KHZ: c_uint = 0x4;
const WSA885X_I2S_CTL0_PCM_RATE_192_OR_176KHZ: c_uint = 0x5;
const WSA885X_I2S_CTL0_PCM_RATE_384_OR_352KHZ: c_uint = 0x6;
const WSA885X_I2S_CFG0_TDM_TX_SLOT0_MASK: c_uint = genmask(2, 0);
const WSA885X_I2S_CFG0_TDM_TX_SLOT1_MASK: c_uint = genmask(6, 4);
const fn WSA885X_I2S_CFG0_TDM_TX_SLOT0(v: c_uint) -> c_uint { field_prep(WSA885X_I2S_CFG0_TDM_TX_SLOT0_MASK, v) }
const fn WSA885X_I2S_CFG0_TDM_TX_SLOT1(v: c_uint) -> c_uint { field_prep(WSA885X_I2S_CFG0_TDM_TX_SLOT1_MASK, v) }
const WSA885X_I2S_CFG1_TDM_TX_SLOT2_MASK: c_uint = genmask(2, 0);
const WSA885X_I2S_CFG1_TDM_TX_SLOT3_MASK: c_uint = genmask(6, 4);
const fn WSA885X_I2S_CFG1_TDM_TX_SLOT2(v: c_uint) -> c_uint { field_prep(WSA885X_I2S_CFG1_TDM_TX_SLOT2_MASK, v) }
const fn WSA885X_I2S_CFG1_TDM_TX_SLOT3(v: c_uint) -> c_uint { field_prep(WSA885X_I2S_CFG1_TDM_TX_SLOT3_MASK, v) }
const WSA885X_I2S_TDM_CTL0_I2S_TDM_EN_MASK: c_uint = bit(0);
const WSA885X_I2S_TDM_CTL0_NUM_CHANNELS_MASK: c_uint = genmask(3, 2);
const WSA885X_I2S_TDM_CTL0_NUM_CHANNELS_2: c_uint = field_prep(WSA885X_I2S_TDM_CTL0_NUM_CHANNELS_MASK, 0);
const WSA885X_I2S_TDM_CTL0_NUM_CHANNELS_4: c_uint = field_prep(WSA885X_I2S_TDM_CTL0_NUM_CHANNELS_MASK, 1);
const WSA885X_I2S_TDM_CTL0_NUM_CHANNELS_8: c_uint = field_prep(WSA885X_I2S_TDM_CTL0_NUM_CHANNELS_MASK, 3);
const WSA885X_I2S_TDM_CH_TX_CH0_EN: c_uint = bit(0);
const WSA885X_I2S_TDM_CH_TX_CH1_EN: c_uint = bit(1);
const WSA885X_I2S_TDM_CH_TX_CH2_EN: c_uint = bit(2);
const WSA885X_I2S_TDM_CH_TX_CH3_EN: c_uint = bit(3);
const WSA885X_I2S_TDM_CH_RX_CH0_EN: c_uint = bit(0);
const WSA885X_I2S_TDM_CH_RX_CH3_EN: c_uint = bit(3);
const WSA885X_I2S_RESET_CTL_RESET_MASK: c_uint = bit(0);
const WSA885X_PCM_DATA_WD_CTL1_PCM_DATA_WD_EN_MASK: c_uint = bit(2);
const WSA885X_POWER_FSM_CTL0_CLEAR_ERROR_MASK: c_uint = bit(3);
const WSA885X_PA_FSM_CTL0_CLEAR_ERROR_MASK: c_uint = bit(2);

const WSA885X_I2S_TX_SLOT_ISENSE0: c_uint = 0x1;
const WSA885X_I2S_TX_SLOT_ISENSE1: c_uint = 0x2;
const WSA885X_I2S_TX_SLOT_CUR_SENSE0: c_uint = 0x5;
const WSA885X_I2S_TX_SLOT_CUR_SENSE1: c_uint = 0x6;

/* RX Sample Rate Index Values - Audio Playback Path */
const WSA885X_RX_RATE_8000HZ: u8 = 0x00;
const WSA885X_RX_RATE_16000HZ: u8 = 0x01;
const WSA885X_RX_RATE_32000HZ: u8 = 0x02;
const WSA885X_RX_RATE_44100HZ: u8 = 0x03;
const WSA885X_RX_RATE_48000HZ: u8 = 0x04;
const WSA885X_RX_RATE_96000HZ: u8 = 0x05;
const WSA885X_RX_RATE_192000HZ: u8 = 0x06;
const WSA885X_RX_RATE_384000HZ: u8 = 0x07;

/* VI Sample Rate Index Values - Voltage/Current Sensing Path */
const WSA885X_VI_RATE_8000HZ: u8 = 0x00;
const WSA885X_VI_RATE_16000HZ: u8 = 0x01;
const WSA885X_VI_RATE_44100HZ: u8 = 0x02;
const WSA885X_VI_RATE_48000HZ: u8 = 0x03;
const WSA885X_VI_RATE_96000HZ: u8 = 0x04;
const WSA885X_VI_RATE_22050HZ: u8 = 0x05;
const WSA885X_VI_RATE_24000HZ: u8 = 0x06;
const WSA885X_VI_RATE_192000HZ: u8 = 0x07;
const WSA885X_VI_RATE_384000HZ: u8 = 0x08;

/* Channel Configuration Masks */
const WSA885X_CHANNEL_STEREO: u32 = 0x03;
const WSA885X_CHANNEL_MONO_LEFT: u32 = 0x01;
const WSA885X_CHANNEL_MONO_RIGHT: u32 = 0x02;

const WSA885X_RATES: c_uint = SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 |
    SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_48000 |
    SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000;

const WSA885X_PLL_LOCK_BIT: c_uint = bit(0);

const WSA885X_FU21_VOL_STEPS: c_int = 124;
const WSA885X_USAGE_MODE_MAX: u32 = 8;
static wsa885x_fu21_digital_gain: [c_uint; 4] = [0, (-8400i32) as c_uint, 100, 0];

unsafe fn wsa885x_is_valid_rx_slot_mask(mask: u32) -> bool {
    mask == WSA885X_CHANNEL_MONO_LEFT ||
        mask == WSA885X_CHANNEL_MONO_RIGHT ||
        mask == WSA885X_CHANNEL_STEREO
}

static wsa885x_supply_name: [*const c_char; 2] = [c"vdd-1p8".as_ptr(), c"vdd-io".as_ptr()];

const WSA885X_BATT_1S: u32 = 1;
const WSA885X_BATT_2S: u32 = 2;

const WSA885X_IRQ_INT_SAF2WAR: c_int = 0;
const WSA885X_IRQ_INT_WAR2SAF: c_int = 1;
const WSA885X_IRQ_INT_DISABLE: c_int = 2;
const WSA885X_IRQ_INT_PA0_OCP: c_int = 3;
const WSA885X_IRQ_INT_PA1_OCP: c_int = 4;
const WSA885X_IRQ_INT_CLIP0: c_int = 5;
const WSA885X_IRQ_INT_CLIP1: c_int = 6;
const WSA885X_IRQ_INT_CLK_WD: c_int = 7;
const WSA885X_IRQ_INT_INTR_GPIO1_PIN: c_int = 8;
const WSA885X_IRQ_INT_INTR_GPIO2_PIN: c_int = 9;
const WSA885X_IRQ_INT_UVLO: c_int = 10;
const WSA885X_IRQ_INT_BOP: c_int = 11;
const WSA885X_IRQ_INT_PA0_FSM_ERR: c_int = 12;
const WSA885X_IRQ_INT_PA1_FSM_ERR: c_int = 13;
const WSA885X_IRQ_INT_MAIN_FSM_ERR: c_int = 14;
const WSA885X_IRQ_INT_PCM_DATA0_WD: c_int = 15;
const WSA885X_IRQ_INT_PCM_DATA1_WD: c_int = 16;
const WSA885X_IRQ_INT_PCM_DATA0_DC: c_int = 17;
const WSA885X_IRQ_INT_PCM_DATA1_DC: c_int = 18;
const WSA885X_IRQ_INT_PLL_UNLOCKED: c_int = 19;
const WSA885X_IRQ_INT_PROT_MODE_CHANGE: c_int = 20;
const WSA885X_IRQ_INT_PB_CLOCK_VALID: c_int = 21;
const WSA885X_IRQ_INT_SENSE_CLOCK_VALID: c_int = 22;
const WSA885X_IRQ_MAX: c_int = 23;

#[repr(C)] struct i2c_client { dev: device, irq: c_int }
#[repr(C)] struct regmap { _priv: [u8; 0] }
#[repr(C)] struct device { _priv: [u8; 0] }
#[repr(C)] struct snd_soc_component { dev: *mut device }
#[repr(C)] struct gpio_desc { _priv: [u8; 0] }
#[repr(C)] struct reset_control { _priv: [u8; 0] }
#[repr(C)] struct mutex { _priv: [u8; 0] }
#[repr(C)] struct snd_pcm_substream { stream: c_int }
#[repr(C)] struct snd_pcm_hw_params { _priv: [u8; 0] }
#[repr(C)] struct snd_soc_dai { component: *mut snd_soc_component }
#[repr(C)] struct snd_kcontrol { _priv: [u8; 0] }
#[repr(C)] struct integer_value { value: [c_long; 128] }
type c_long = isize;
#[repr(C)] union ctl_value { integer: core::mem::ManuallyDrop<integer_value> }
#[repr(C)] struct snd_ctl_elem_value { value: ctl_value }

#[repr(C)]
struct wsa885x_priv {
    client: *mut i2c_client,
    regmap: *mut regmap,
    dev: *mut device,
    component: *mut snd_soc_component,
    sd_n: *mut gpio_desc,
    sd_reset: *mut reset_control,
    usage_mode: u32,
    rx_slot_mask: u32,
    batt_conf: u32,
    stereo_vol_db: c_int,
    state_lock: mutex, /* protects mutable control state */
}

#[repr(C)]
#[derive(Copy, Clone)]
struct wsa885x_reg_update {
    reg: c_uint,
    mask: c_uint,
    val: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct reg_sequence { reg: c_uint, def: c_uint }

#[repr(C)]
#[derive(Copy, Clone)]
struct reg_default { reg: c_uint, def: c_uint }

#[repr(C)]
struct regmap_range_cfg {
    range_min: c_uint,
    range_max: c_uint,
    selector_reg: c_uint,
    selector_mask: c_uint,
    selector_shift: c_uint,
    window_start: c_uint,
    window_len: c_uint,
}

#[repr(C)]
struct regmap_config {
    reg_bits: c_uint,
    val_bits: c_uint,
    max_register: c_uint,
    ranges: *const regmap_range_cfg,
    num_ranges: c_uint,
    reg_defaults: *const reg_default,
    num_reg_defaults: c_uint,
    volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    cache_type: c_uint,
    use_single_read: bool,
    use_single_write: bool,
}

#[repr(C)]
struct snd_soc_dai_ops {
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)] struct snd_soc_pcm_stream { stream_name: *const c_char, channels_min: c_uint, channels_max: c_uint, rates: c_uint, formats: c_uint }
#[repr(C)] struct snd_soc_dai_driver { name: *const c_char, playback: snd_soc_pcm_stream, ops: *const snd_soc_dai_ops }
#[repr(C)] struct snd_kcontrol_new { _priv: [u8; 0] }
#[repr(C)] struct snd_soc_component_driver { name: *const c_char, probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>, controls: *const snd_kcontrol_new, num_controls: c_uint }
#[repr(C)] struct of_device_id { compatible: *const c_char }
#[repr(C)] struct i2c_device_id { name: *const c_char, driver_data: c_ulong }
#[repr(C)] struct device_driver { name: *const c_char, of_match_table: *const of_device_id }
#[repr(C)] struct i2c_driver { driver: device_driver, probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>, id_table: *const i2c_device_id }

#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone)]
enum irqreturn_t { IRQ_NONE = 0, IRQ_HANDLED = 1 }
use irqreturn_t::{IRQ_HANDLED, IRQ_NONE};

unsafe extern "C" {
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_multi_reg_write(map: *mut regmap, regs: *const reg_sequence, num_regs: c_int) -> c_int;
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn_ratelimited(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_init_regmap(component: *mut snd_soc_component, regmap: *mut regmap);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn reset_control_assert(rst: *mut reset_control) -> c_int;
    fn reset_control_deassert(rst: *mut reset_control) -> c_int;
    fn gpiod_direction_output(desc: *mut gpio_desc, value: c_int) -> c_int;
    fn devm_reset_control_get_optional_shared(dev: *mut device, id: *const c_char) -> *mut reset_control;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, cfg: *const regmap_config) -> *mut regmap;
    fn device_property_read_string(dev: *mut device, propname: *const c_char, val: *mut *const c_char) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn devm_regulator_get_enable(dev: *mut device, id: *const c_char) -> c_int;
    fn devm_add_action_or_reset(dev: *mut device, action: Option<unsafe extern "C" fn(*mut c_void)>, data: *mut c_void) -> c_int;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_request_threaded_irq(dev: *mut device, irq: c_uint, handler: *const c_void, thread_fn: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
}

static wsa885x_regmap_ranges: [regmap_range_cfg; 1] = [regmap_range_cfg {
    range_min: 0, range_max: 0x88ff, selector_reg: 0x0, selector_mask: 0xFF,
    selector_shift: 0, window_start: 0, window_len: 0x100,
}];

static wsa885x_codec_reg_defaults: [reg_default; 102] = [
    reg_default { reg: WSA885X_SMP_AMP_CTRL_STEREO_STEREO_SMP_AMP_CTRL_I2S, def: 0x00 },
    reg_default { reg: WSA885X_SMP_AMP_CTRL_STEREO_IT21_CLUSERINDEX, def: 0x01 },
    reg_default { reg: WSA885X_SMP_AMP_CTRL_STEREO_CMT_GRP_MASK, def: 0x00 },
    reg_default { reg: WSA885X_SMP_AMP_CTRL_STEREO_OT23_USAGE, def: 0x00 },
    reg_default { reg: WSA885X_SMP_AMP_CTRL_STEREO_CS21_CLOCK_VALID, def: 0x00 },
    reg_default { reg: WSA885X_SMP_AMP_CTRL_STEREO_CS21_SAMPLERATEINDEX, def: 0x04 },
    reg_default { reg: WSA885X_SMP_AMP_CTRL_STEREO_PPU21_POSTURENUMBER, def: 0x01 },
    reg_default { reg: WSA885X_SMP_AMP_CTRL_STEREO_FU21_MUTE_CH2X0, def: 0x01 },
    reg_default { reg: WSA885X_SMP_AMP_CTRL_STEREO_FU21_MUTE_CH2X1, def: 0x01 },
    reg_default { reg: WSA885X_SMP_AMP_CTRL_STEREO_FU21_CH_VOL_CH2X0_MSB, def: 0xac },
    reg_default { reg: WSA885X_SMP_AMP_CTRL_STEREO_FU21_CH_VOL_CH2X0_LSB, def: 0x00 },
    reg_default { reg: WSA885X_SMP_AMP_CTRL_STEREO_FU21_CH_VOL_CH2X1_MSB, def: 0xac },
    reg_default { reg: WSA885X_SMP_AMP_CTRL_STEREO_FU21_CH_VOL_CH2X1_LSB, def: 0x00 },
    reg_default { reg: WSA885X_SMP_AMP_CTRL_STEREO_PDE23_REQ_PS, def: 0x03 },
    reg_default { reg: WSA885X_SMP_AMP_CTRL_STEREO_PDE23_ACT_PS, def: 0x03 },
    reg_default { reg: WSA885X_SMP_AMP_CTRL_STEREO_CS24_SAMPLERATEINDEX, def: 0x03 },
    reg_default { reg: WSA885X_ANA_TOP_PON_CKSK_CTL_0, def: 0x00 },
    reg_default { reg: WSA885X_ANA_TOP_BG_TVP_UVLO1_PROG, def: 0x19 },
    reg_default { reg: WSA885X_ANA_TOP_BG_TVP_UVLO2_PROG, def: 0x22 },
    reg_default { reg: WSA885X_ANA_PLL_DIV_CTL_0, def: 0x0c },
    reg_default { reg: WSA885X_ANA_PLL_DIV_CTL_1, def: 0x50 },
    reg_default { reg: WSA885X_ANA_TOP_PLL_VCO_CTL, def: 0x00 },
    reg_default { reg: WSA885X_ANA_TOP_PLL_LOOPFILT_0, def: 0xb4 },
    reg_default { reg: WSA885X_ANA_TOP_PLL_OVRD_CTL, def: 0x00 },
    reg_default { reg: WSA885X_ANA_TOP_BG_TVP_OVRD_CTL, def: 0x00 },
    reg_default { reg: WSA885X_ANA_TOP_BOOST_STB_CTRL2, def: 0x03 },
    reg_default { reg: WSA885X_ANA_TOP_BOOST_STB_CTRL3, def: 0x3c },
    reg_default { reg: WSA885X_ANA_TOP_BOOST_BYP_CTRL2, def: 0xc5 },
    reg_default { reg: WSA885X_ANA_TOP_BOOST_BYP_CTRL3, def: 0x13 },
    reg_default { reg: WSA885X_ANA_TOP_BOOST_MISC, def: 0x79 },
    reg_default { reg: WSA885X_ANA_TOP_SPK_TOP_SPARE3, def: 0x00 },
    reg_default { reg: WSA885X_SPK_TOP_COMMON_CTRL2, def: 0x08 },
    reg_default { reg: WSA885X_SPK_TOP_LF_CH1_CTRL11, def: 0x09 },
    reg_default { reg: WSA885X_SPK_TOP_LF_CH1_TUNE1, def: 0x00 },
    reg_default { reg: WSA885X_SPK_TOP_LF_CH2_TUNE1, def: 0x00 },
    reg_default { reg: WSA885X_SPK_TOP_LF_CH1_CTRL9, def: 0x00 },
    reg_default { reg: WSA885X_SPK_TOP_LF_CH2_CTRL9, def: 0x00 },
    reg_default { reg: WSA885X_SPK_TOP_LF_CH2_CTRL11, def: 0x09 },
    reg_default { reg: WSA885X_SPK_TOP_COMMON_TUNE1, def: 0x03 },
    reg_default { reg: WSA885X_IVSENSE_VSNS_ISNS_CTL_CH1, def: 0x00 },
    reg_default { reg: WSA885X_DIG_CTRL0_CDC_CLK_CTL, def: 0x0e },
    reg_default { reg: WSA885X_ANA_TOP_BOOST_PWRSTAGE_CTRL2, def: 0x40 },
    reg_default { reg: WSA885X_ANA_TOP_BOOST_PWRSTAGE_CTRL4, def: 0xff },
    reg_default { reg: WSA885X_ANA_TOP_PLL_STATUS_0, def: 0x00 },
    reg_default { reg: WSA885X_ANA_TOP_PLL_STATUS_1, def: 0x00 },
    reg_default { reg: WSA885X_ANA_TOP_IVSENSE_ADC_MODE_CTL2, def: 0x84 },
    reg_default { reg: WSA885X_ANA_TOP_IVSENSE_ADC_MODE_CTL3, def: 0x02 },
    reg_default { reg: WSA885X_ANA_TOP_IVSENSE_ADC_REF_CTL, def: 0x00 },
    reg_default { reg: WSA885X_ANA_TOP_IVSENSE_ADC_CDAC_CAL_CTL2, def: 0xe0 },
    reg_default { reg: WSA885X_ANA_TOP_SPK_TOP_PWRSTG_CH1_CTRL3, def: 0xa4 },
    reg_default { reg: WSA885X_ANA_TOP_SPK_TOP_PWRSTG_CH1_TUNE3, def: 0xc9 },
    reg_default { reg: WSA885X_ANA_TOP_SPK_TOP_PWRSTG_CH2_CTRL3, def: 0xa4 },
    reg_default { reg: WSA885X_ANA_TOP_SPK_TOP_PWRSTG_CH2_TUNE3, def: 0xc9 },
    reg_default { reg: WSA885X_DIG_CTRL0_TOP_CLK_CFG, def: 0x00 },
    reg_default { reg: WSA885X_DIG_CTRL0_SDCA_COMMIT, def: 0x00 },
    reg_default { reg: WSA885X_DIG_CTRL0_CLK_SOURCE_ENABLE, def: 0x00 },
    reg_default { reg: WSA885X_DIG_CTRL0_SYS_CLK_SEL, def: 0x00 },
    reg_default { reg: WSA885X_DIG_CTRL0_PA_FSM_CTL, def: 0x00 },
    reg_default { reg: WSA885X_DIG_CTRL0_POWER_FSM_CTL0, def: 0x05 },
    reg_default { reg: WSA885X_DIG_CTRL0_POWER_FSM_CTL1, def: 0x00 },
    reg_default { reg: WSA885X_DIG_CTRL0_PA0_FSM_CTL1, def: 0x45 },
    reg_default { reg: WSA885X_DIG_CTRL0_PA1_FSM_CTL1, def: 0x45 },
    reg_default { reg: WSA885X_DIG_CTRL0_VBAT_THRM_FLT_CTL, def: 0x7f },
    reg_default { reg: WSA885X_DIG_CTRL0_CDC_RXTX_FSCNT_CTL, def: 0x00 },
    reg_default { reg: WSA885X_DIG_CTRL0_GAIN_RAMP0_CTL1, def: 0x01 },
    reg_default { reg: WSA885X_DIG_CTRL0_GAIN_RAMP1_CTL1, def: 0x01 },
    reg_default { reg: WSA885X_DIG_CTRL1_I2S_CTL0, def: 0x06 },
    reg_default { reg: WSA885X_DIG_CTRL1_I2S_CFG0_TDM_TX, def: 0x00 },
    reg_default { reg: WSA885X_DIG_CTRL1_I2S_CFG1_TDM_TX, def: 0x00 },
    reg_default { reg: WSA885X_DIG_CTRL1_I2S_TDM_CTL0, def: 0x00 },
    reg_default { reg: WSA885X_DIG_CTRL1_I2S_TDM_CTL1, def: 0x05 },
    reg_default { reg: WSA885X_DIG_CTRL1_I2S_TDM_CH_TX, def: 0x00 },
    reg_default { reg: WSA885X_DIG_CTRL1_I2S_RESET_CTL, def: 0x00 },
    reg_default { reg: WSA885X_DIG_CTRL1_I2S_TDM_CH_RX, def: WSA885X_I2S_TDM_CH_RX_CH3_EN },
    reg_default { reg: WSA885X_CDC_RX0_RX_PATH_CFG0, def: 0x89 },
    reg_default { reg: WSA885X_CDC_RX0_RX_PATH_CFG1, def: 0x64 },
    reg_default { reg: WSA885X_CDC_RX0_RX_PATH_CTL, def: 0x24 },
    reg_default { reg: WSA885X_RX0_RX_PATH_DSMDEM_CTL, def: 0x01 },
    reg_default { reg: WSA885X_CDC_RX1_RX_PATH_CFG0, def: 0x89 },
    reg_default { reg: WSA885X_CDC_RX1_RX_PATH_CFG1, def: 0x64 },
    reg_default { reg: WSA885X_CDC_RX1_RX_PATH_CTL, def: 0x04 },
    reg_default { reg: WSA885X_RX1_RX_PATH_DSMDEM_CTL, def: 0x01 },
    reg_default { reg: WSA885X_CDC_COMPANDER0_CTL0, def: 0x01 },
    reg_default { reg: WSA885X_CDC_COMPANDER0_CTL7, def: 0x2a },
    reg_default { reg: WSA885X_CDC_COMPANDER1_CTL0, def: 0x01 },
    reg_default { reg: WSA885X_CDC_COMPANDER1_CTL7, def: 0x2a },
    reg_default { reg: WSA885X_CDC_VSENSE0_SPKR_PROT_PATH_CTL, def: 0x14 },
    reg_default { reg: WSA885X_CDC_VSENSE1_SPKR_PROT_PATH_CTL, def: 0x14 },
    reg_default { reg: WSA885X_CDC_ISENSE0_SPKR_PROT_PATH_CTL, def: 0x14 },
    reg_default { reg: WSA885X_CDC_ISENSE1_SPKR_PROT_PATH_CTL, def: 0x14 },
    reg_default { reg: WSA885X_CDC_CLSH_V1P8_BP_CTL1, def: 0x50 },
    reg_default { reg: WSA885X_CDC_CLSH_V1P8_BP_CTL0, def: 0x6c },
    reg_default { reg: WSA885X_CDC_CLSH_CLSH_SIG_DP_CTL0, def: 0x0d },
    reg_default { reg: WSA885X_CDC_CLSH_V_HD_PA, def: 0x03 },
    reg_default { reg: WSA885X_CDC_CLSH_V1P8_BP_CTL2, def: 0x05 },
];

unsafe fn wsa885x_multi_update_bits(regmap: *mut regmap, updates: *const wsa885x_reg_update, num_updates: size_t) {
    let mut i: size_t = 0;
    while i < num_updates {
        let u = *updates.add(i);
        regmap_update_bits(regmap, u.reg, u.mask, u.val);
        i += 1;
    }
}

unsafe fn wsa885x_toggle_irq_bit(wsa885x: *mut wsa885x_priv, reg: c_uint, mask: c_uint) {
    regmap_update_bits((*wsa885x).regmap, reg, mask, 0);
    regmap_update_bits((*wsa885x).regmap, reg, mask, mask);
}

unsafe fn wsa885x_pulse_irq_bit(wsa885x: *mut wsa885x_priv, reg: c_uint, mask: c_uint) {
    regmap_update_bits((*wsa885x).regmap, reg, mask, 0);
    regmap_update_bits((*wsa885x).regmap, reg, mask, mask);
    regmap_update_bits((*wsa885x).regmap, reg, mask, 0);
}

unsafe fn wsa885x_tdm_ctl0_slot_num_val(slots: c_int, slot_num_val: *mut c_uint) -> c_int {
    if slot_num_val.is_null() { return -EINVAL; }
    match slots {
        2 => { *slot_num_val = WSA885X_I2S_TDM_CTL0_NUM_CHANNELS_2; 0 }
        4 => { *slot_num_val = WSA885X_I2S_TDM_CTL0_NUM_CHANNELS_4; 0 }
        8 => { *slot_num_val = WSA885X_I2S_TDM_CTL0_NUM_CHANNELS_8; 0 }
        _ => -EINVAL,
    }
}

unsafe fn wsa885x_reg_update_sequence(regmap: *mut regmap, slots: c_int) -> c_int {
    static regs: [reg_sequence; 2] = [
        reg_sequence { reg: WSA885X_DIG_CTRL1_I2S_TDM_CTL1, def: 0x15 },
        reg_sequence { reg: WSA885X_DIG_CTRL1_I2S_TDM_CTL1, def: 0x11 },
    ];
    let mut slot_num_val: c_uint = 0;
    let mut ret: c_int;
    if regmap.is_null() { return -EINVAL; }
    ret = wsa885x_tdm_ctl0_slot_num_val(slots, &mut slot_num_val);
    if ret != 0 { return ret; }
    regmap_multi_reg_write(regmap, regs.as_ptr(), regs.len() as c_int);
    regmap_update_bits(regmap, WSA885X_DIG_CTRL1_I2S_TDM_CTL0, WSA885X_I2S_TDM_CTL0_NUM_CHANNELS_MASK, slot_num_val);
    regmap_update_bits(regmap, WSA885X_DIG_CTRL1_I2S_TDM_CTL0, WSA885X_I2S_TDM_CTL0_I2S_TDM_EN_MASK, WSA885X_I2S_TDM_CTL0_I2S_TDM_EN_MASK);
    regmap_write(regmap, WSA885X_DIG_CTRL1_I2S_TDM_CH_TX, WSA885X_I2S_TDM_CH_TX_CH0_EN);
    regmap_update_bits(regmap, WSA885X_DIG_CTRL1_I2S_TDM_CH_TX, WSA885X_I2S_TDM_CH_TX_CH1_EN, WSA885X_I2S_TDM_CH_TX_CH1_EN);
    0
}

unsafe fn wsa885x_wait_for_pll_lock(wsa885x: *mut wsa885x_priv) -> c_int {
    let mut status: c_uint = 0;
    let mut cnt: c_int = 0;
    let mut ret: c_int;
    loop {
        usleep_range(1000, 1100);
        ret = regmap_read((*wsa885x).regmap, WSA885X_ANA_TOP_PLL_STATUS_0, &mut status);
        if ret != 0 {
            dev_err((*wsa885x).dev, c"PLL status read failed: %d\n".as_ptr(), ret);
            return ret;
        }
        if status & WSA885X_PLL_LOCK_BIT != 0 { return 0; }
        cnt += 1;
        if cnt >= 20 { break; }
    }
    dev_warn((*wsa885x).dev, c"PLL lock timeout after 20ms, status=0x%x\n".as_ptr(), status);
    -ETIMEDOUT
}

unsafe fn wsa885x_2s_conf(wsa885x: *mut wsa885x_priv) -> c_int {
    static regs: [reg_sequence; 5] = [
        reg_sequence { reg: WSA885X_SPK_TOP_COMMON_TUNE1, def: 0x26 },
        reg_sequence { reg: WSA885X_SPK_TOP_LF_CH1_CTRL11, def: 0x0d },
        reg_sequence { reg: WSA885X_SPK_TOP_LF_CH2_CTRL11, def: 0x0d },
        reg_sequence { reg: WSA885X_CDC_CLSH_V1P8_BP_CTL1, def: 0x71 },
        reg_sequence { reg: WSA885X_CDC_CLSH_V1P8_BP_CTL0, def: 0xAA },
    ];
    regmap_multi_reg_write((*wsa885x).regmap, regs.as_ptr(), regs.len() as c_int)
}

static wsa885x_reg_init: [reg_sequence; 55] = [
    reg_sequence { reg: WSA885X_CDC_RX0_RX_PATH_CTL, def: 0x24 }, reg_sequence { reg: WSA885X_CDC_RX1_RX_PATH_CTL, def: 0x24 },
    reg_sequence { reg: WSA885X_RX0_RX_PATH_DSMDEM_CTL, def: 0x01 }, reg_sequence { reg: WSA885X_RX1_RX_PATH_DSMDEM_CTL, def: 0x01 },
    reg_sequence { reg: WSA885X_CDC_COMPANDER0_CTL0, def: 0x01 }, reg_sequence { reg: WSA885X_CDC_COMPANDER1_CTL0, def: 0x01 },
    reg_sequence { reg: WSA885X_CDC_VSENSE0_SPKR_PROT_PATH_CTL, def: 0x14 }, reg_sequence { reg: WSA885X_CDC_VSENSE1_SPKR_PROT_PATH_CTL, def: 0x14 },
    reg_sequence { reg: WSA885X_CDC_ISENSE0_SPKR_PROT_PATH_CTL, def: 0x14 }, reg_sequence { reg: WSA885X_CDC_ISENSE1_SPKR_PROT_PATH_CTL, def: 0x14 },
    reg_sequence { reg: WSA885X_DIG_CTRL0_CDC_CLK_CTL, def: 0x0f }, reg_sequence { reg: WSA885X_DIG_CTRL0_CDC_CLK_CTL, def: 0x4f },
    reg_sequence { reg: WSA885X_DIG_CTRL0_CDC_RXTX_FSCNT_CTL, def: 0x02 }, reg_sequence { reg: WSA885X_DIG_CTRL0_CDC_RXTX_FSCNT_CTL, def: 0x00 },
    reg_sequence { reg: WSA885X_DIG_CTRL0_CDC_RXTX_FSCNT_CTL, def: 0x01 }, reg_sequence { reg: WSA885X_SMP_AMP_CTRL_STEREO_CMT_GRP_MASK, def: 0x01 },
    reg_sequence { reg: WSA885X_CDC_RX0_RX_PATH_CFG1, def: 0x60 }, reg_sequence { reg: WSA885X_CDC_RX1_RX_PATH_CFG1, def: 0x60 },
    reg_sequence { reg: WSA885X_ANA_TOP_SPK_TOP_PWRSTG_CH1_CTRL3, def: 0xa5 }, reg_sequence { reg: WSA885X_ANA_TOP_SPK_TOP_PWRSTG_CH2_CTRL3, def: 0xa5 },
    reg_sequence { reg: WSA885X_ANA_TOP_IVSENSE_ADC_MODE_CTL2, def: 0x85 }, reg_sequence { reg: WSA885X_ANA_TOP_IVSENSE_ADC_MODE_CTL3, def: 0x0c },
    reg_sequence { reg: WSA885X_ANA_TOP_IVSENSE_ADC_MODE_CTL3, def: 0x0e }, reg_sequence { reg: WSA885X_ANA_TOP_IVSENSE_ADC_REF_CTL, def: 0x0c },
    reg_sequence { reg: WSA885X_DIG_CTRL0_GAIN_RAMP0_CTL1, def: 0x01 }, reg_sequence { reg: WSA885X_DIG_CTRL0_GAIN_RAMP1_CTL1, def: 0x01 },
    reg_sequence { reg: WSA885X_CDC_RX0_RX_PATH_CFG0, def: 0x88 }, reg_sequence { reg: WSA885X_CDC_RX0_RX_PATH_CFG0, def: 0x89 },
    reg_sequence { reg: WSA885X_CDC_RX1_RX_PATH_CFG0, def: 0x88 }, reg_sequence { reg: WSA885X_CDC_RX1_RX_PATH_CFG0, def: 0x89 },
    reg_sequence { reg: WSA885X_ANA_TOP_BOOST_STB_CTRL2, def: 0x82 }, reg_sequence { reg: WSA885X_ANA_TOP_BOOST_STB_CTRL3, def: 0x34 },
    reg_sequence { reg: WSA885X_ANA_TOP_BOOST_PWRSTAGE_CTRL2, def: 0x41 }, reg_sequence { reg: WSA885X_ANA_TOP_BOOST_PWRSTAGE_CTRL4, def: 0x7f },
    reg_sequence { reg: WSA885X_CDC_CLSH_V1P8_BP_CTL1, def: 0x50 }, reg_sequence { reg: WSA885X_CDC_CLSH_V1P8_BP_CTL0, def: 0x6c },
    reg_sequence { reg: WSA885X_CDC_CLSH_CLSH_SIG_DP_CTL0, def: 0x0d }, reg_sequence { reg: WSA885X_CDC_CLSH_V_HD_PA, def: 0x03 },
    reg_sequence { reg: WSA885X_DIG_CTRL0_POWER_FSM_CTL0, def: 0x05 }, reg_sequence { reg: WSA885X_ANA_TOP_PON_CKSK_CTL_0, def: 0x20 },
    reg_sequence { reg: WSA885X_ANA_TOP_SPK_TOP_PWRSTG_CH1_TUNE3, def: 0x45 }, reg_sequence { reg: WSA885X_ANA_TOP_SPK_TOP_PWRSTG_CH2_TUNE3, def: 0x45 },
    reg_sequence { reg: WSA885X_CDC_CLSH_V1P8_BP_CTL2, def: 0x05 }, reg_sequence { reg: WSA885X_ANA_TOP_BG_TVP_UVLO1_PROG, def: 0x35 },
    reg_sequence { reg: WSA885X_ANA_TOP_BG_TVP_UVLO2_PROG, def: 0x21 }, reg_sequence { reg: WSA885X_ANA_TOP_BOOST_BYP_CTRL2, def: 0xc7 },
    reg_sequence { reg: WSA885X_ANA_TOP_BOOST_BYP_CTRL3, def: 0x11 }, reg_sequence { reg: WSA885X_ANA_TOP_IVSENSE_ADC_CDAC_CAL_CTL2, def: 0x80 },
    reg_sequence { reg: WSA885X_ANA_TOP_SPK_TOP_SPARE3, def: 0x08 }, reg_sequence { reg: WSA885X_DIG_CTRL0_PA0_FSM_CTL1, def: 0x47 },
    reg_sequence { reg: WSA885X_DIG_CTRL0_PA1_FSM_CTL1, def: 0x47 }, reg_sequence { reg: WSA885X_CDC_COMPANDER0_CTL7, def: 0x34 },
    reg_sequence { reg: WSA885X_CDC_COMPANDER1_CTL7, def: 0x34 }, reg_sequence { reg: WSA885X_DIG_CTRL0_VBAT_THRM_FLT_CTL, def: 0x79 },
];

unsafe fn wsa885x_hw_init(wsa885x: *mut wsa885x_priv) -> c_int {
    static regs: [reg_sequence; 3] = [
        reg_sequence { reg: WSA885X_DIG_CTRL1_SPMI_PAD_GPIO2_CTL, def: 0x2e },
        reg_sequence { reg: WSA885X_DIG_CTRL1_INTR_MODE, def: 0x01 },
        reg_sequence { reg: WSA885X_DIG_CTRL1_PIN_CT, def: 0x04 },
    ];
    let mut ret = regmap_multi_reg_write((*wsa885x).regmap, wsa885x_reg_init.as_ptr(), wsa885x_reg_init.len() as c_int);
    if ret != 0 { return ret; }
    if (*wsa885x).batt_conf == WSA885X_BATT_2S {
        ret = wsa885x_2s_conf(wsa885x);
        if ret != 0 { return ret; }
    }
    regmap_multi_reg_write((*wsa885x).regmap, regs.as_ptr(), regs.len() as c_int)
}

unsafe fn wsa885x_unmask_interrupts(wsa885x: *mut wsa885x_priv) -> c_int {
    static regs: [reg_sequence; 3] = [
        reg_sequence { reg: WSA885X_INTR_MASK0, def: 0x00 },
        reg_sequence { reg: WSA885X_INTR_MASK0 + 1, def: 0x00 },
        reg_sequence { reg: WSA885X_INTR_MASK0 + 2, def: 0xf8 },
    ];
    regmap_multi_reg_write((*wsa885x).regmap, regs.as_ptr(), regs.len() as c_int)
}

unsafe fn wsa885x_wait_for_pde_state(wsa885x: *mut wsa885x_priv, ps: c_int) -> c_int {
    let mut act_ps: c_uint = 0;
    let mut clock_valid: c_uint = 0;
    let mut cnt: c_int = 0;
    let mut rc: c_int;
    if ps < 0 || ps > 3 { return -EINVAL; }
    loop {
        usleep_range(1000, 1500);
        rc = regmap_read((*wsa885x).regmap, WSA885X_SMP_AMP_CTRL_STEREO_PDE23_ACT_PS, &mut act_ps);
        if rc != 0 {
            dev_err((*wsa885x).dev, c"PDE state read failed: %d\n".as_ptr(), rc);
            return rc;
        }
        if act_ps == ps as c_uint { return 0; }
        cnt += 1;
        if cnt >= 5 { break; }
    }
    if regmap_read((*wsa885x).regmap, WSA885X_SMP_AMP_CTRL_STEREO_CS21_CLOCK_VALID, &mut clock_valid) != 0 {
        dev_err((*wsa885x).dev, c"PDE power state %d request failed, actual_ps %d, clock_valid read failed\n".as_ptr(), ps, act_ps);
    } else {
        dev_err((*wsa885x).dev, c"PDE power state %d request failed, actual_ps %d, clock_valid:%d\n".as_ptr(), ps, act_ps, clock_valid);
    }
    -ETIMEDOUT
}

unsafe fn wsa885x_program_stereo_volume(wsa885x: *mut wsa885x_priv, stereo_vol_db: c_int, commit: bool) {
    let v = (stereo_vol_db as i8) as u8 as c_uint;
    regmap_write((*wsa885x).regmap, WSA885X_SMP_AMP_CTRL_STEREO_FU21_CH_VOL_CH2X0_MSB, v);
    regmap_write((*wsa885x).regmap, WSA885X_SMP_AMP_CTRL_STEREO_FU21_CH_VOL_CH2X0_LSB, 0x00);
    regmap_write((*wsa885x).regmap, WSA885X_SMP_AMP_CTRL_STEREO_FU21_CH_VOL_CH2X1_MSB, v);
    regmap_write((*wsa885x).regmap, WSA885X_SMP_AMP_CTRL_STEREO_FU21_CH_VOL_CH2X1_LSB, 0x00);
    if commit { regmap_write((*wsa885x).regmap, WSA885X_DIG_CTRL0_SDCA_COMMIT, 0x01); }
}

unsafe extern "C" fn wsa885x_codec_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let wsa885x = snd_soc_component_get_drvdata((*dai).component) as *mut wsa885x_priv;
    let pcm_rate: u8;
    let cs21_sample_rate_idx: u8;
    let cs24_sample_rate_idx: u8;
    match params_rate(params) {
        8000 => { pcm_rate = WSA885X_I2S_CTL0_PCM_RATE_8KHZ as u8; cs21_sample_rate_idx = WSA885X_RX_RATE_8000HZ; cs24_sample_rate_idx = WSA885X_VI_RATE_8000HZ; }
        16000 => { pcm_rate = WSA885X_I2S_CTL0_PCM_RATE_16KHZ as u8; cs21_sample_rate_idx = WSA885X_RX_RATE_16000HZ; cs24_sample_rate_idx = WSA885X_VI_RATE_16000HZ; }
        32000 => { pcm_rate = WSA885X_I2S_CTL0_PCM_RATE_32KHZ as u8; cs21_sample_rate_idx = WSA885X_RX_RATE_32000HZ; cs24_sample_rate_idx = WSA885X_VI_RATE_48000HZ; }
        44100 => { pcm_rate = WSA885X_I2S_CTL0_PCM_RATE_48_OR_44KHZ as u8; cs21_sample_rate_idx = WSA885X_RX_RATE_44100HZ; cs24_sample_rate_idx = WSA885X_VI_RATE_44100HZ; }
        48000 => { pcm_rate = WSA885X_I2S_CTL0_PCM_RATE_48_OR_44KHZ as u8; cs21_sample_rate_idx = WSA885X_RX_RATE_48000HZ; cs24_sample_rate_idx = WSA885X_VI_RATE_48000HZ; }
        88200 | 96000 => { pcm_rate = WSA885X_I2S_CTL0_PCM_RATE_96_OR_88KHZ as u8; cs21_sample_rate_idx = WSA885X_RX_RATE_96000HZ; cs24_sample_rate_idx = WSA885X_VI_RATE_96000HZ; }
        176400 | 192000 => { pcm_rate = WSA885X_I2S_CTL0_PCM_RATE_192_OR_176KHZ as u8; cs21_sample_rate_idx = WSA885X_RX_RATE_192000HZ; cs24_sample_rate_idx = WSA885X_VI_RATE_192000HZ; }
        352800 | 384000 => { pcm_rate = WSA885X_I2S_CTL0_PCM_RATE_384_OR_352KHZ as u8; cs21_sample_rate_idx = WSA885X_RX_RATE_384000HZ; cs24_sample_rate_idx = WSA885X_VI_RATE_384000HZ; }
        _ => {
            dev_err((*wsa885x).dev, c"sampling rate %d is not supported\n".as_ptr(), params_rate(params));
            return -EINVAL;
        }
    }
    regmap_update_bits((*wsa885x).regmap, WSA885X_DIG_CTRL1_I2S_CTL0, WSA885X_I2S_CTL0_PCM_RATE_MASK | WSA885X_I2S_CTL0_ENABLE_MASK, WSA885X_I2S_CTL0_PCM_RATE(pcm_rate as c_uint) | WSA885X_I2S_CTL0_ENABLE_MASK);
    regmap_write((*wsa885x).regmap, WSA885X_DIG_CTRL1_I2S_RESET_CTL, 0x00);
    regmap_write((*wsa885x).regmap, WSA885X_SMP_AMP_CTRL_STEREO_CS21_SAMPLERATEINDEX, cs21_sample_rate_idx as c_uint);
    regmap_write((*wsa885x).regmap, WSA885X_SMP_AMP_CTRL_STEREO_CS24_SAMPLERATEINDEX, cs24_sample_rate_idx as c_uint);
    mutex_lock(&mut (*wsa885x).state_lock);
    wsa885x_program_stereo_volume(wsa885x, (*wsa885x).stereo_vol_db, false);
    mutex_unlock(&mut (*wsa885x).state_lock);
    regmap_write((*wsa885x).regmap, WSA885X_DIG_CTRL0_SDCA_COMMIT, 0x01);
    0
}

unsafe extern "C" fn wsa885x_codec_set_tdm_slot(dai: *mut snd_soc_dai, _tx_slot_mask: c_uint, rx_slot_mask: c_uint, slots: c_int, _slot_width: c_int) -> c_int {
    static stereo_updates: [wsa885x_reg_update; 4] = [
        wsa885x_reg_update { reg: WSA885X_DIG_CTRL1_I2S_CFG0_TDM_TX, mask: WSA885X_I2S_CFG0_TDM_TX_SLOT0_MASK, val: WSA885X_I2S_CFG0_TDM_TX_SLOT0(WSA885X_I2S_TX_SLOT_ISENSE0) },
        wsa885x_reg_update { reg: WSA885X_DIG_CTRL1_I2S_CFG0_TDM_TX, mask: WSA885X_I2S_CFG0_TDM_TX_SLOT1_MASK, val: WSA885X_I2S_CFG0_TDM_TX_SLOT1(WSA885X_I2S_TX_SLOT_ISENSE1) },
        wsa885x_reg_update { reg: WSA885X_DIG_CTRL1_I2S_CFG1_TDM_TX, mask: WSA885X_I2S_CFG1_TDM_TX_SLOT2_MASK, val: WSA885X_I2S_CFG1_TDM_TX_SLOT2(WSA885X_I2S_TX_SLOT_CUR_SENSE0) },
        wsa885x_reg_update { reg: WSA885X_DIG_CTRL1_I2S_CFG1_TDM_TX, mask: WSA885X_I2S_CFG1_TDM_TX_SLOT3_MASK, val: WSA885X_I2S_CFG1_TDM_TX_SLOT3(WSA885X_I2S_TX_SLOT_CUR_SENSE1) },
    ];
    static mono_left_updates: [wsa885x_reg_update; 2] = [
        wsa885x_reg_update { reg: WSA885X_DIG_CTRL1_I2S_CFG0_TDM_TX, mask: WSA885X_I2S_CFG0_TDM_TX_SLOT0_MASK, val: WSA885X_I2S_CFG0_TDM_TX_SLOT0(WSA885X_I2S_TX_SLOT_ISENSE0) },
        wsa885x_reg_update { reg: WSA885X_DIG_CTRL1_I2S_CFG0_TDM_TX, mask: WSA885X_I2S_CFG0_TDM_TX_SLOT1_MASK, val: WSA885X_I2S_CFG0_TDM_TX_SLOT1(WSA885X_I2S_TX_SLOT_CUR_SENSE0) },
    ];
    static mono_right_updates: [wsa885x_reg_update; 2] = [
        wsa885x_reg_update { reg: WSA885X_DIG_CTRL1_I2S_CFG0_TDM_TX, mask: WSA885X_I2S_CFG0_TDM_TX_SLOT0_MASK, val: WSA885X_I2S_CFG0_TDM_TX_SLOT0(WSA885X_I2S_TX_SLOT_ISENSE1) },
        wsa885x_reg_update { reg: WSA885X_DIG_CTRL1_I2S_CFG0_TDM_TX, mask: WSA885X_I2S_CFG0_TDM_TX_SLOT1_MASK, val: WSA885X_I2S_CFG0_TDM_TX_SLOT1(WSA885X_I2S_TX_SLOT_CUR_SENSE1) },
    ];
    let wsa885x = snd_soc_component_get_drvdata((*dai).component) as *mut wsa885x_priv;
    let mut slot_num_val: c_uint = 0;
    let mut ret = wsa885x_tdm_ctl0_slot_num_val(slots, &mut slot_num_val);
    if ret != 0 {
        dev_err((*wsa885x).dev, c"%s: unsupported slot count %d\n".as_ptr(), c"wsa885x_codec_set_tdm_slot".as_ptr(), slots);
        return ret;
    }
    if rx_slot_mask != 0 && !wsa885x_is_valid_rx_slot_mask(rx_slot_mask) {
        dev_err((*wsa885x).dev, c"%s: unsupported rx_slot_mask 0x%x\n".as_ptr(), c"wsa885x_codec_set_tdm_slot".as_ptr(), rx_slot_mask);
        return -EINVAL;
    }
    mutex_lock(&mut (*wsa885x).state_lock);
    if rx_slot_mask != 0 { (*wsa885x).rx_slot_mask = rx_slot_mask; }
    else if !wsa885x_is_valid_rx_slot_mask((*wsa885x).rx_slot_mask) { (*wsa885x).rx_slot_mask = WSA885X_CHANNEL_STEREO; }
    let mask = (*wsa885x).rx_slot_mask;
    regmap_update_bits((*wsa885x).regmap, WSA885X_DIG_CTRL1_I2S_RESET_CTL, WSA885X_I2S_RESET_CTL_RESET_MASK, WSA885X_I2S_RESET_CTL_RESET_MASK);
    if mask == WSA885X_CHANNEL_STEREO {
        wsa885x_multi_update_bits((*wsa885x).regmap, stereo_updates.as_ptr(), stereo_updates.len());
        ret = wsa885x_reg_update_sequence((*wsa885x).regmap, slots);
        if ret == 0 {
            regmap_update_bits((*wsa885x).regmap, WSA885X_DIG_CTRL1_I2S_TDM_CH_TX, WSA885X_I2S_TDM_CH_TX_CH2_EN, WSA885X_I2S_TDM_CH_TX_CH2_EN);
            regmap_update_bits((*wsa885x).regmap, WSA885X_DIG_CTRL1_I2S_TDM_CH_TX, WSA885X_I2S_TDM_CH_TX_CH3_EN, WSA885X_I2S_TDM_CH_TX_CH3_EN);
        }
    } else if mask == WSA885X_CHANNEL_MONO_LEFT {
        wsa885x_multi_update_bits((*wsa885x).regmap, mono_left_updates.as_ptr(), mono_left_updates.len());
        ret = wsa885x_reg_update_sequence((*wsa885x).regmap, slots);
    } else if mask == WSA885X_CHANNEL_MONO_RIGHT {
        wsa885x_multi_update_bits((*wsa885x).regmap, mono_right_updates.as_ptr(), mono_right_updates.len());
        ret = wsa885x_reg_update_sequence((*wsa885x).regmap, slots);
    }
    if ret == 0 {
        regmap_update_bits((*wsa885x).regmap, WSA885X_DIG_CTRL1_I2S_CTL0, WSA885X_I2S_CTL0_ENABLE_MASK, WSA885X_I2S_CTL0_ENABLE_MASK);
        regmap_update_bits((*wsa885x).regmap, WSA885X_DIG_CTRL1_I2S_RESET_CTL, WSA885X_I2S_RESET_CTL_RESET_MASK, 0);
    }
    mutex_unlock(&mut (*wsa885x).state_lock);
    ret
}

unsafe extern "C" fn wsa885x_codec_set_sysclk(dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    static pll_prep: [reg_sequence; 5] = [
        reg_sequence { reg: WSA885X_ANA_TOP_BG_TVP_OVRD_CTL, def: 0x03 },
        reg_sequence { reg: WSA885X_DIG_CTRL0_SYS_CLK_SEL, def: 0x04 },
        reg_sequence { reg: WSA885X_ANA_TOP_PLL_LOOPFILT_0, def: 0xB4 },
        reg_sequence { reg: WSA885X_ANA_TOP_PLL_VCO_CTL, def: 0x00 },
        reg_sequence { reg: WSA885X_ANA_TOP_PLL_OVRD_CTL, def: 0x00 },
    ];
    static pll_cleanup: [reg_sequence; 3] = [
        reg_sequence { reg: WSA885X_DIG_CTRL0_CLK_SOURCE_ENABLE, def: 0x00 },
        reg_sequence { reg: WSA885X_DIG_CTRL0_SYS_CLK_SEL, def: 0x00 },
        reg_sequence { reg: WSA885X_ANA_TOP_BG_TVP_OVRD_CTL, def: 0x00 },
    ];
    let wsa885x = snd_soc_component_get_drvdata((*dai).component) as *mut wsa885x_priv;
    if freq == 0 { return -EINVAL; }
    if WSA885X_CLK_RATE_FIXED % freq != 0 { return -EINVAL; }
    let pll_div = WSA885X_CLK_RATE_FIXED / freq;
    if pll_div > 0xff { return -EINVAL; }
    regmap_multi_reg_write((*wsa885x).regmap, pll_prep.as_ptr(), pll_prep.len() as c_int);
    regmap_write((*wsa885x).regmap, WSA885X_ANA_PLL_DIV_CTL_0, pll_div);
    regmap_write((*wsa885x).regmap, WSA885X_DIG_CTRL0_CLK_SOURCE_ENABLE, 0x02);
    let ret = wsa885x_wait_for_pll_lock(wsa885x);
    if ret != 0 {
        dev_err((*wsa885x).dev, c"PLL lock failed, aborting sysclk configuration\n".as_ptr());
        regmap_multi_reg_write((*wsa885x).regmap, pll_cleanup.as_ptr(), pll_cleanup.len() as c_int);
        return ret;
    }
    regmap_write((*wsa885x).regmap, WSA885X_DIG_CTRL0_SYS_CLK_SEL, 0x00);
    regmap_write((*wsa885x).regmap, WSA885X_DIG_CTRL0_POWER_FSM_CTL1, 0x01);
    regmap_write((*wsa885x).regmap, WSA885X_ANA_TOP_BG_TVP_OVRD_CTL, 0x00);
    0
}

unsafe extern "C" fn wsa885x_codec_mute_stream(dai: *mut snd_soc_dai, mute: c_int, stream: c_int) -> c_int {
    static mute_regs: [reg_sequence; 2] = [reg_sequence { reg: WSA885X_DIG_CTRL0_PA_FSM_CTL, def: 0x00 }, reg_sequence { reg: WSA885X_SMP_AMP_CTRL_STEREO_PDE23_REQ_PS, def: 0x03 }];
    static mute_commit_regs: [reg_sequence; 3] = [reg_sequence { reg: WSA885X_SMP_AMP_CTRL_STEREO_FU21_MUTE_CH2X0, def: 0x01 }, reg_sequence { reg: WSA885X_SMP_AMP_CTRL_STEREO_FU21_MUTE_CH2X1, def: 0x01 }, reg_sequence { reg: WSA885X_DIG_CTRL0_SDCA_COMMIT, def: 0x01 }];
    static unmute_prep_head_regs: [reg_sequence; 1] = [reg_sequence { reg: WSA885X_DIG_CTRL0_PA_FSM_CTL, def: 0x00 }];
    static unmute_prep_tail_regs: [reg_sequence; 2] = [reg_sequence { reg: WSA885X_SMP_AMP_CTRL_STEREO_IT21_CLUSERINDEX, def: 0x01 }, reg_sequence { reg: WSA885X_SMP_AMP_CTRL_STEREO_PPU21_POSTURENUMBER, def: 0x01 }];
    static unmute_volume_regs: [reg_sequence; 2] = [reg_sequence { reg: WSA885X_SMP_AMP_CTRL_STEREO_FU21_CH_VOL_CH2X0_LSB, def: 0x00 }, reg_sequence { reg: WSA885X_SMP_AMP_CTRL_STEREO_FU21_CH_VOL_CH2X1_LSB, def: 0x00 }];
    static unmute_commit_regs: [reg_sequence; 2] = [reg_sequence { reg: WSA885X_DIG_CTRL0_SDCA_COMMIT, def: 0x01 }, reg_sequence { reg: WSA885X_SMP_AMP_CTRL_STEREO_PDE23_REQ_PS, def: 0x00 }];
    static unmute_finish_regs: [reg_sequence; 3] = [reg_sequence { reg: WSA885X_SMP_AMP_CTRL_STEREO_FU21_MUTE_CH2X0, def: 0x00 }, reg_sequence { reg: WSA885X_SMP_AMP_CTRL_STEREO_FU21_MUTE_CH2X1, def: 0x00 }, reg_sequence { reg: WSA885X_DIG_CTRL0_SDCA_COMMIT, def: 0x01 }];
    let wsa885x = snd_soc_component_get_drvdata((*dai).component) as *mut wsa885x_priv;
    let mut ret = 0;
    if stream != SNDRV_PCM_STREAM_PLAYBACK { return 0; }
    mutex_lock(&mut (*wsa885x).state_lock);
    if (*wsa885x).usage_mode > WSA885X_USAGE_MODE_MAX { ret = -EINVAL; }
    else {
        if !wsa885x_is_valid_rx_slot_mask((*wsa885x).rx_slot_mask) { (*wsa885x).rx_slot_mask = WSA885X_CHANNEL_STEREO; }
        if mute != 0 {
            regmap_multi_reg_write((*wsa885x).regmap, mute_regs.as_ptr(), mute_regs.len() as c_int);
            ret = wsa885x_wait_for_pde_state(wsa885x, 3);
            if ret != 0 { dev_err((*wsa885x).dev, c"PS3 transition failed: %d\n".as_ptr(), ret); }
            else { regmap_multi_reg_write((*wsa885x).regmap, mute_commit_regs.as_ptr(), mute_commit_regs.len() as c_int); }
        } else {
            regmap_multi_reg_write((*wsa885x).regmap, unmute_prep_head_regs.as_ptr(), unmute_prep_head_regs.len() as c_int);
            regmap_write((*wsa885x).regmap, WSA885X_SMP_AMP_CTRL_STEREO_OT23_USAGE, (*wsa885x).usage_mode);
            regmap_multi_reg_write((*wsa885x).regmap, unmute_prep_tail_regs.as_ptr(), unmute_prep_tail_regs.len() as c_int);
            wsa885x_program_stereo_volume(wsa885x, (*wsa885x).stereo_vol_db, false);
            regmap_multi_reg_write((*wsa885x).regmap, unmute_volume_regs.as_ptr(), unmute_volume_regs.len() as c_int);
            regmap_multi_reg_write((*wsa885x).regmap, unmute_commit_regs.as_ptr(), unmute_commit_regs.len() as c_int);
            ret = wsa885x_wait_for_pde_state(wsa885x, 0);
            if ret == 0 {
                if (*wsa885x).rx_slot_mask == WSA885X_CHANNEL_STEREO {
                    regmap_write((*wsa885x).regmap, WSA885X_DIG_CTRL1_I2S_TDM_CH_RX, WSA885X_I2S_TDM_CH_RX_CH0_EN | WSA885X_I2S_TDM_CH_RX_CH3_EN);
                    regmap_write((*wsa885x).regmap, WSA885X_DIG_CTRL0_PA_FSM_CTL, 0x03);
                } else if (*wsa885x).rx_slot_mask == WSA885X_CHANNEL_MONO_LEFT {
                    regmap_write((*wsa885x).regmap, WSA885X_DIG_CTRL1_I2S_TDM_CH_RX, WSA885X_I2S_TDM_CH_RX_CH3_EN);
                    regmap_write((*wsa885x).regmap, WSA885X_DIG_CTRL0_PA_FSM_CTL, 0x01);
                } else if (*wsa885x).rx_slot_mask == WSA885X_CHANNEL_MONO_RIGHT {
                    regmap_write((*wsa885x).regmap, WSA885X_DIG_CTRL1_I2S_TDM_CH_RX, WSA885X_I2S_TDM_CH_RX_CH0_EN);
                    regmap_write((*wsa885x).regmap, WSA885X_DIG_CTRL0_PA_FSM_CTL, 0x02);
                }
                regmap_multi_reg_write((*wsa885x).regmap, unmute_finish_regs.as_ptr(), unmute_finish_regs.len() as c_int);
            }
        }
    }
    mutex_unlock(&mut (*wsa885x).state_lock);
    ret
}

unsafe extern "C" fn wsa885x_codec_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    static regs: [reg_sequence; 1] = [reg_sequence { reg: WSA885X_DIG_CTRL0_PA_FSM_CTL, def: 0x00 }];
    let wsa885x = snd_soc_component_get_drvdata((*dai).component) as *mut wsa885x_priv;
    if (*substream).stream != SNDRV_PCM_STREAM_PLAYBACK { return 0; }
    mutex_lock(&mut (*wsa885x).state_lock);
    regmap_multi_reg_write((*wsa885x).regmap, regs.as_ptr(), regs.len() as c_int);
    mutex_unlock(&mut (*wsa885x).state_lock);
    0
}

static wsa885x_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(wsa885x_codec_hw_params),
    set_tdm_slot: Some(wsa885x_codec_set_tdm_slot),
    set_sysclk: Some(wsa885x_codec_set_sysclk),
    mute_stream: Some(wsa885x_codec_mute_stream),
    hw_free: Some(wsa885x_codec_hw_free),
};

static mut wsa885x_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"wsa885x_dai_drv".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"WSA885X TDM Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: WSA885X_RATES,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
    },
    ops: &wsa885x_dai_ops,
}];

unsafe extern "C" fn wsa885x_reset_assert(data: *mut c_void) {
    let wsa885x = data as *mut wsa885x_priv;
    if !(*wsa885x).sd_reset.is_null() { reset_control_assert((*wsa885x).sd_reset); }
    else { gpiod_direction_output((*wsa885x).sd_n, 1); }
}

unsafe fn wsa885x_reset_deassert(wsa885x: *mut wsa885x_priv) {
    if !(*wsa885x).sd_reset.is_null() { reset_control_deassert((*wsa885x).sd_reset); }
    else { gpiod_direction_output((*wsa885x).sd_n, 0); }
}

unsafe fn wsa885x_get_reset(dev: *mut device, wsa885x: *mut wsa885x_priv) -> c_int {
    (*wsa885x).sd_reset = devm_reset_control_get_optional_shared(dev, ptr::null());
    if IS_ERR((*wsa885x).sd_reset as *const c_void) {
        return dev_err_probe(dev, PTR_ERR((*wsa885x).sd_reset as *const c_void), c"Failed to get reset\n".as_ptr());
    } else if !(*wsa885x).sd_reset.is_null() {
        return 0;
    }
    (*wsa885x).sd_n = devm_gpiod_get_optional(dev, c"powerdown".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR((*wsa885x).sd_n as *const c_void) {
        return dev_err_probe(dev, PTR_ERR((*wsa885x).sd_n as *const c_void), c"Shutdown Control GPIO not found\n".as_ptr());
    }
    0
}

unsafe extern "C" fn wsa885x_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    matches!(reg,
        WSA885X_ANA_TOP_PLL_STATUS_0 | WSA885X_ANA_TOP_PLL_STATUS_1 |
        WSA885X_DIG_CTRL0_SDCA_COMMIT | WSA885X_SMP_AMP_CTRL_STEREO_PDE23_ACT_PS |
        WSA885X_SMP_AMP_CTRL_STEREO_CS21_CLOCK_VALID | WSA885X_INTR_STATUS0 |
        x if x == WSA885X_INTR_STATUS0 + 1 || x == WSA885X_INTR_STATUS0 + 2 ||
             x == WSA885X_INTR_CLEAR0 || x == WSA885X_INTR_CLEAR0 + 1 || x == WSA885X_INTR_CLEAR0 + 2)
}

unsafe extern "C" fn wsa885x_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    if reg == WSA885X_INTR_CLEAR0 || reg == WSA885X_INTR_CLEAR0 + 1 || reg == WSA885X_INTR_CLEAR0 + 2 { return false; }
    reg <= 0x88ff
}

unsafe extern "C" fn wsa885x_writeable_register(_dev: *mut device, reg: c_uint) -> bool {
    if reg > 0x88ff { return false; }
    match reg {
        WSA885X_ANA_TOP_PLL_STATUS_0 | WSA885X_ANA_TOP_PLL_STATUS_1 |
        WSA885X_INTR_STATUS0 | WSA885X_SMP_AMP_CTRL_STEREO_PDE23_ACT_PS |
        WSA885X_SMP_AMP_CTRL_STEREO_CS21_CLOCK_VALID => false,
        x if x == WSA885X_INTR_STATUS0 + 1 || x == WSA885X_INTR_STATUS0 + 2 => false,
        _ => true,
    }
}

static wsa885x_regmap_cfg: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: 0x88FF,
    ranges: wsa885x_regmap_ranges.as_ptr(),
    num_ranges: wsa885x_regmap_ranges.len() as c_uint,
    reg_defaults: wsa885x_codec_reg_defaults.as_ptr(),
    num_reg_defaults: wsa885x_codec_reg_defaults.len() as c_uint,
    volatile_reg: Some(wsa885x_volatile_register),
    writeable_reg: Some(wsa885x_writeable_register),
    readable_reg: Some(wsa885x_readable_register),
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
};

unsafe extern "C" fn wsa885x_component_probe(component: *mut snd_soc_component) -> c_int {
    let wsa885x = snd_soc_component_get_drvdata(component) as *mut wsa885x_priv;
    (*wsa885x).component = component;
    snd_soc_component_init_regmap(component, (*wsa885x).regmap);
    let ret = wsa885x_hw_init(wsa885x);
    if ret != 0 { return ret; }
    wsa885x_unmask_interrupts(wsa885x)
}

unsafe extern "C" fn wsa885x_stereo_gain_offset_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let wsa885x = snd_soc_component_get_drvdata(component) as *mut wsa885x_priv;
    mutex_lock(&mut (*wsa885x).state_lock);
    let val = (*wsa885x).stereo_vol_db + 84;
    mutex_unlock(&mut (*wsa885x).state_lock);
    if val < 0 || val > WSA885X_FU21_VOL_STEPS { return -ERANGE; }
    (*ucontrol).value.integer.value[0] = val as c_long;
    0
}

unsafe extern "C" fn wsa885x_stereo_gain_offset_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let wsa885x = snd_soc_component_get_drvdata(component) as *mut wsa885x_priv;
    let val = (*ucontrol).value.integer.value[0];
    if val < 0 || val > WSA885X_FU21_VOL_STEPS as c_long {
        dev_err((*component).dev, c"%s: Invalid range, Val: %ld\n".as_ptr(), c"wsa885x_stereo_gain_offset_put".as_ptr(), val);
        return -EINVAL;
    }
    let stereo_vol_db = val as c_int - 84;
    mutex_lock(&mut (*wsa885x).state_lock);
    if (*wsa885x).stereo_vol_db == stereo_vol_db {
        mutex_unlock(&mut (*wsa885x).state_lock);
        return 0;
    }
    wsa885x_program_stereo_volume(wsa885x, stereo_vol_db, true);
    (*wsa885x).stereo_vol_db = stereo_vol_db;
    mutex_unlock(&mut (*wsa885x).state_lock);
    1
}

unsafe extern "C" fn wsa885x_usage_modes_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let wsa885x = snd_soc_component_get_drvdata(component) as *mut wsa885x_priv;
    mutex_lock(&mut (*wsa885x).state_lock);
    if (*wsa885x).usage_mode > WSA885X_USAGE_MODE_MAX {
        mutex_unlock(&mut (*wsa885x).state_lock);
        return -ERANGE;
    }
    (*ucontrol).value.integer.value[0] = (*wsa885x).usage_mode as c_long;
    mutex_unlock(&mut (*wsa885x).state_lock);
    0
}

unsafe extern "C" fn wsa885x_usage_modes_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let wsa885x = snd_soc_component_get_drvdata(component) as *mut wsa885x_priv;
    let val = (*ucontrol).value.integer.value[0] as u32;
    if val > WSA885X_USAGE_MODE_MAX { return -EINVAL; }
    mutex_lock(&mut (*wsa885x).state_lock);
    if (*wsa885x).usage_mode == val {
        mutex_unlock(&mut (*wsa885x).state_lock);
        return 0;
    }
    (*wsa885x).usage_mode = val;
    mutex_unlock(&mut (*wsa885x).state_lock);
    1
}

unsafe extern "C" fn wsa885x_rx_slot_mask_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let wsa885x = snd_soc_component_get_drvdata(component) as *mut wsa885x_priv;
    mutex_lock(&mut (*wsa885x).state_lock);
    let mask = (*wsa885x).rx_slot_mask;
    mutex_unlock(&mut (*wsa885x).state_lock);
    if !wsa885x_is_valid_rx_slot_mask(mask) { return -ERANGE; }
    (*ucontrol).value.integer.value[0] = mask as c_long;
    0
}

unsafe extern "C" fn wsa885x_rx_slot_mask_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let wsa885x = snd_soc_component_get_drvdata(component) as *mut wsa885x_priv;
    let mask = (*ucontrol).value.integer.value[0] as u32;
    if !wsa885x_is_valid_rx_slot_mask(mask) { return -EINVAL; }
    mutex_lock(&mut (*wsa885x).state_lock);
    if (*wsa885x).rx_slot_mask == mask {
        mutex_unlock(&mut (*wsa885x).state_lock);
        return 0;
    }
    (*wsa885x).rx_slot_mask = mask;
    mutex_unlock(&mut (*wsa885x).state_lock);
    1
}

/* SOC_SINGLE_EXT and SOC_SINGLE_EXT_TLV expand to initialized snd_kcontrol_new entries in C. */
static wsa885x_snd_controls: [snd_kcontrol_new; 3] = [
    snd_kcontrol_new { _priv: [] },
    snd_kcontrol_new { _priv: [] },
    snd_kcontrol_new { _priv: [] },
];

static wsa885x_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"wsa885x".as_ptr(),
    probe: Some(wsa885x_component_probe),
    controls: wsa885x_snd_controls.as_ptr(),
    num_controls: wsa885x_snd_controls.len() as c_uint,
};

unsafe extern "C" fn wsa885x_handle_irq(irq_idx: c_int, data: *mut c_void) -> irqreturn_t {
    let wsa885x = data as *mut wsa885x_priv;
    if irq_idx < 0 || irq_idx >= WSA885X_IRQ_MAX { return IRQ_NONE; }
    match irq_idx {
        WSA885X_IRQ_INT_PCM_DATA0_WD | WSA885X_IRQ_INT_PCM_DATA1_WD => {
            if irq_idx == WSA885X_IRQ_INT_PCM_DATA0_WD {
                wsa885x_toggle_irq_bit(wsa885x, WSA885X_DIG_CTRL0_PCM_DATA_WD0_CTL1, WSA885X_PCM_DATA_WD_CTL1_PCM_DATA_WD_EN_MASK);
            } else {
                wsa885x_toggle_irq_bit(wsa885x, WSA885X_DIG_CTRL0_PCM_DATA_WD1_CTL1, WSA885X_PCM_DATA_WD_CTL1_PCM_DATA_WD_EN_MASK);
            }
        }
        WSA885X_IRQ_INT_PA0_FSM_ERR | WSA885X_IRQ_INT_PA1_FSM_ERR | WSA885X_IRQ_INT_MAIN_FSM_ERR => {
            if irq_idx == WSA885X_IRQ_INT_MAIN_FSM_ERR {
                wsa885x_pulse_irq_bit(wsa885x, WSA885X_DIG_CTRL0_POWER_FSM_CTL0, WSA885X_POWER_FSM_CTL0_CLEAR_ERROR_MASK);
            } else if irq_idx == WSA885X_IRQ_INT_PA0_FSM_ERR {
                wsa885x_pulse_irq_bit(wsa885x, WSA885X_PA0_FSM_CTL0, WSA885X_PA_FSM_CTL0_CLEAR_ERROR_MASK);
            } else if irq_idx == WSA885X_IRQ_INT_PA1_FSM_ERR {
                wsa885x_pulse_irq_bit(wsa885x, WSA885X_PA1_FSM_CTL0, WSA885X_PA_FSM_CTL0_CLEAR_ERROR_MASK);
            }
        }
        _ => {}
    }
    IRQ_HANDLED
}

unsafe extern "C" fn wsa885x_interrupt_handler(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    static status_reg: [c_uint; WSA885X_NUM_REGS] = [WSA885X_INTR_STATUS0, WSA885X_INTR_STATUS0 + 1, WSA885X_INTR_STATUS0 + 2];
    static clear_reg: [c_uint; WSA885X_NUM_REGS] = [WSA885X_INTR_CLEAR0, WSA885X_INTR_CLEAR0 + 1, WSA885X_INTR_CLEAR0 + 2];
    let mut status: [c_uint; WSA885X_NUM_REGS] = [0; WSA885X_NUM_REGS];
    let wsa885x = data as *mut wsa885x_priv;
    let mut handled = IRQ_NONE;
    let mut i = 0usize;
    while i < WSA885X_NUM_REGS {
        let ret = regmap_read((*wsa885x).regmap, status_reg[i], &mut status[i]);
        if ret != 0 {
            dev_err((*wsa885x).dev, c"Failed to read status_reg[%d] (0x%x): %d\n".as_ptr(), i as c_int, status_reg[i], ret);
            status[i] = 0;
        }
        i += 1;
    }
    i = 0;
    while i < WSA885X_NUM_REGS {
        let mut bit_idx = 0;
        while bit_idx < 8 {
            if status[i] & bit(bit_idx as c_uint) != 0 {
                let irq_num = (i as c_int) * 8 + bit_idx;
                regmap_write((*wsa885x).regmap, clear_reg[i], bit(bit_idx as c_uint));
                regmap_write((*wsa885x).regmap, clear_reg[i], 0);
                if irq_num >= WSA885X_IRQ_MAX {
                    dev_warn_ratelimited((*wsa885x).dev, c"Unexpected IRQ bit %d (reg %d)\n".as_ptr(), bit_idx, i as c_int);
                    handled = IRQ_HANDLED;
                } else if wsa885x_handle_irq(irq_num, wsa885x as *mut c_void) == IRQ_HANDLED {
                    handled = IRQ_HANDLED;
                }
            }
            bit_idx += 1;
        }
        i += 1;
    }
    handled
}

unsafe fn wsa885x_register_irq(wsa885x: *mut wsa885x_priv) -> c_int {
    if (*(*wsa885x).client).irq == 0 {
        return dev_err_probe((*wsa885x).dev, -EINVAL, c"IRQ is not configured\n".as_ptr());
    }
    devm_request_threaded_irq((*wsa885x).dev, (*(*wsa885x).client).irq as c_uint, ptr::null(), Some(wsa885x_interrupt_handler), IRQF_ONESHOT, dev_name((*wsa885x).dev), wsa885x as *mut c_void)
}

unsafe extern "C" fn wsa885x_probe(client: *mut i2c_client) -> c_int {
    let dev: *mut device = &mut (*client).dev;
    let wsa885x = devm_kzalloc(dev, core::mem::size_of::<wsa885x_priv>(), GFP_KERNEL) as *mut wsa885x_priv;
    if wsa885x.is_null() { return -ENOMEM; }
    (*wsa885x).client = client;
    (*wsa885x).dev = dev;
    (*wsa885x).stereo_vol_db = -84;
    (*wsa885x).rx_slot_mask = WSA885X_CHANNEL_STEREO;
    mutex_init(&mut (*wsa885x).state_lock);
    (*wsa885x).regmap = devm_regmap_init_i2c(client, &wsa885x_regmap_cfg);
    if IS_ERR((*wsa885x).regmap as *const c_void) { return PTR_ERR((*wsa885x).regmap as *const c_void); }
    let mut battery_config: *const c_char = ptr::null();
    let mut ret = device_property_read_string(dev, c"qcom,battery-config".as_ptr(), &mut battery_config);
    if ret != 0 { (*wsa885x).batt_conf = WSA885X_BATT_1S; }
    else if strcmp(battery_config, c"1s".as_ptr()) == 0 { (*wsa885x).batt_conf = WSA885X_BATT_1S; }
    else if strcmp(battery_config, c"2s".as_ptr()) == 0 { (*wsa885x).batt_conf = WSA885X_BATT_2S; }
    else { return dev_err_probe(dev, -EINVAL, c"Invalid battery config %s (expected 1s or 2s)\n".as_ptr(), battery_config); }
    let mut i = 0usize;
    while i < wsa885x_supply_name.len() {
        ret = devm_regulator_get_enable(dev, wsa885x_supply_name[i]);
        if ret != 0 { return dev_err_probe(dev, ret, c"Failed to enable regulator %s\n".as_ptr(), wsa885x_supply_name[i]); }
        i += 1;
    }
    ret = wsa885x_get_reset(dev, wsa885x);
    if ret != 0 { return ret; }
    wsa885x_reset_deassert(wsa885x);
    usleep_range(5000, 5500);
    ret = devm_add_action_or_reset(dev, Some(wsa885x_reset_assert), wsa885x as *mut c_void);
    if ret != 0 { return dev_err_probe(dev, ret, c"devm_add_action_or_reset failed\n".as_ptr()); }
    i2c_set_clientdata(client, wsa885x as *mut c_void);
    ret = wsa885x_register_irq(wsa885x);
    if ret != 0 { return dev_err_probe(dev, ret, c"wsa885x irq registration failed\n".as_ptr()); }
    ret = devm_snd_soc_register_component(dev, &wsa885x_component, wsa885x_dai.as_mut_ptr(), wsa885x_dai.len() as c_int);
    if ret != 0 { return dev_err_probe(dev, ret, c"Codec component registration failed\n".as_ptr()); }
    0
}

static wsa885x_dt_match: [of_device_id; 2] = [
    of_device_id { compatible: c"qcom,wsa8855".as_ptr() },
    of_device_id { compatible: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, wsa885x_dt_match); */

static wsa885x_id: [i2c_device_id; 2] = [
    i2c_device_id { name: c"wsa885x".as_ptr(), driver_data: 0 },
    i2c_device_id { name: ptr::null(), driver_data: 0 },
];
/* MODULE_DEVICE_TABLE(i2c, wsa885x_id); */

static mut wsa885x_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"wsa885x".as_ptr(),
        of_match_table: wsa885x_dt_match.as_ptr(),
    },
    probe: Some(wsa885x_probe),
    id_table: wsa885x_id.as_ptr(),
};

/* module_i2c_driver(wsa885x_driver); */

/* MODULE_DESCRIPTION("ASoC WSA885X Stereo Smart PA Codec Driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
