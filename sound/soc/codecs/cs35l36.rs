// SPDX-License-Identifier: GPL-2.0
//
// cs35l36.rs -- CS35L36 ALSA SoC audio driver
//
// Copyright 2018 Cirrus Logic, Inc.
//
// Author: James Schulman <james.schulman@cirrus.com>
//
// Rust source-level translation of soc/codecs/cs35l36.c. Kernel, ALSA, regmap,
// GPIO, regulator, OF, IRQ, and module symbols are supplied by surrounding
// translation units/bindings.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type bool_ = bool;
type u32 = c_uint;
type irqreturn_t = c_int;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct i2c_client { pub dev: device, pub irq: c_int }
#[repr(C)] pub struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }
#[repr(C)] pub struct snd_ctl_elem_value_value { pub integer: snd_ctl_elem_value_integer }
#[repr(C)] pub struct snd_ctl_elem_value_integer { pub value: [c_int; 128] }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device }
#[repr(C)] pub struct snd_soc_dai { pub component: *mut snd_soc_component }
#[repr(C)] pub struct snd_pcm_substream { pub stream: c_int }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context }
#[repr(C)] pub struct irq_data { _private: [u8; 0] }
#[repr(C)] pub struct regulator_bulk_data { pub supply: *const c_char }
#[repr(C)] pub struct reg_default { pub reg: c_uint, pub def: c_uint }
#[repr(C)] pub struct reg_sequence { pub reg: c_uint, pub def: c_uint }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l36_vpbr_cfg {
    pub is_present: bool,
    pub vpbr_en: c_uint,
    pub vpbr_thld: c_uint,
    pub vpbr_atk_rate: c_uint,
    pub vpbr_atk_vol: c_uint,
    pub vpbr_max_attn: c_uint,
    pub vpbr_wait: c_uint,
    pub vpbr_rel_rate: c_uint,
    pub vpbr_mute_en: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l36_platform_data {
    pub extern_boost: bool,
    pub dcm_mode: bool,
    pub amp_pcm_inv: bool,
    pub multi_amp_mode: bool,
    pub imon_pol_inv: bool,
    pub vmon_pol_inv: bool,
    pub bst_vctl: c_uint,
    pub bst_vctl_sel: c_uint,
    pub bst_ipk: c_uint,
    pub boost_ind: c_uint,
    pub temp_warn_thld: c_uint,
    pub irq_drv_sel: c_uint,
    pub irq_gpio_sel: c_uint,
    pub vpbr_config: cs35l36_vpbr_cfg,
}

#[repr(C)]
pub struct cs35l36_private {
    pub dev: *mut device,
    pub pdata: cs35l36_platform_data,
    pub regmap: *mut regmap,
    pub supplies: [regulator_bulk_data; 2],
    pub num_supplies: c_int,
    pub clksrc: c_int,
    pub chip_version: c_int,
    pub rev_id: c_int,
    pub ldm_mode_sel: c_int,
    pub reset_gpio: *mut gpio_desc,
}

#[repr(C)]
pub struct cs35l36_pll_config {
    pub freq: c_int,
    pub clk_cfg: c_int,
    pub fll_igain: c_int,
}

#[repr(C)]
pub struct cs35l36_global_fs_config {
    pub rate: c_int,
    pub fs_cfg: c_int,
}

pub const CS35L36_VALID_PDATA: c_uint = 0x80000000;

const VA: &[u8] = b"VA\0";
const VP: &[u8] = b"VP\0";
static cs35l36_supplies: [*const c_char; 2] = [
    VA.as_ptr() as *const c_char,
    VP.as_ptr() as *const c_char,
];

static cs35l36_pll_sysclk: [cs35l36_pll_config; 64] = [
    cs35l36_pll_config { freq: 32768, clk_cfg: 0x00, fll_igain: 0x05 },
    cs35l36_pll_config { freq: 8000, clk_cfg: 0x01, fll_igain: 0x03 },
    cs35l36_pll_config { freq: 11025, clk_cfg: 0x02, fll_igain: 0x03 },
    cs35l36_pll_config { freq: 12000, clk_cfg: 0x03, fll_igain: 0x03 },
    cs35l36_pll_config { freq: 16000, clk_cfg: 0x04, fll_igain: 0x04 },
    cs35l36_pll_config { freq: 22050, clk_cfg: 0x05, fll_igain: 0x04 },
    cs35l36_pll_config { freq: 24000, clk_cfg: 0x06, fll_igain: 0x04 },
    cs35l36_pll_config { freq: 32000, clk_cfg: 0x07, fll_igain: 0x05 },
    cs35l36_pll_config { freq: 44100, clk_cfg: 0x08, fll_igain: 0x05 },
    cs35l36_pll_config { freq: 48000, clk_cfg: 0x09, fll_igain: 0x05 },
    cs35l36_pll_config { freq: 88200, clk_cfg: 0x0A, fll_igain: 0x06 },
    cs35l36_pll_config { freq: 96000, clk_cfg: 0x0B, fll_igain: 0x06 },
    cs35l36_pll_config { freq: 128000, clk_cfg: 0x0C, fll_igain: 0x07 },
    cs35l36_pll_config { freq: 176400, clk_cfg: 0x0D, fll_igain: 0x07 },
    cs35l36_pll_config { freq: 192000, clk_cfg: 0x0E, fll_igain: 0x07 },
    cs35l36_pll_config { freq: 256000, clk_cfg: 0x0F, fll_igain: 0x08 },
    cs35l36_pll_config { freq: 352800, clk_cfg: 0x10, fll_igain: 0x08 },
    cs35l36_pll_config { freq: 384000, clk_cfg: 0x11, fll_igain: 0x08 },
    cs35l36_pll_config { freq: 512000, clk_cfg: 0x12, fll_igain: 0x09 },
    cs35l36_pll_config { freq: 705600, clk_cfg: 0x13, fll_igain: 0x09 },
    cs35l36_pll_config { freq: 750000, clk_cfg: 0x14, fll_igain: 0x09 },
    cs35l36_pll_config { freq: 768000, clk_cfg: 0x15, fll_igain: 0x09 },
    cs35l36_pll_config { freq: 1000000, clk_cfg: 0x16, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 1024000, clk_cfg: 0x17, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 1200000, clk_cfg: 0x18, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 1411200, clk_cfg: 0x19, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 1500000, clk_cfg: 0x1A, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 1536000, clk_cfg: 0x1B, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 2000000, clk_cfg: 0x1C, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 2048000, clk_cfg: 0x1D, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 2400000, clk_cfg: 0x1E, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 2822400, clk_cfg: 0x1F, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 3000000, clk_cfg: 0x20, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 3072000, clk_cfg: 0x21, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 3200000, clk_cfg: 0x22, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 4000000, clk_cfg: 0x23, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 4096000, clk_cfg: 0x24, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 4800000, clk_cfg: 0x25, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 5644800, clk_cfg: 0x26, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 6000000, clk_cfg: 0x27, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 6144000, clk_cfg: 0x28, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 6250000, clk_cfg: 0x29, fll_igain: 0x08 },
    cs35l36_pll_config { freq: 6400000, clk_cfg: 0x2A, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 6500000, clk_cfg: 0x2B, fll_igain: 0x08 },
    cs35l36_pll_config { freq: 6750000, clk_cfg: 0x2C, fll_igain: 0x09 },
    cs35l36_pll_config { freq: 7526400, clk_cfg: 0x2D, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 8000000, clk_cfg: 0x2E, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 8192000, clk_cfg: 0x2F, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 9600000, clk_cfg: 0x30, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 11289600, clk_cfg: 0x31, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 12000000, clk_cfg: 0x32, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 12288000, clk_cfg: 0x33, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 12500000, clk_cfg: 0x34, fll_igain: 0x08 },
    cs35l36_pll_config { freq: 12800000, clk_cfg: 0x35, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 13000000, clk_cfg: 0x36, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 13500000, clk_cfg: 0x37, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 19200000, clk_cfg: 0x38, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 22579200, clk_cfg: 0x39, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 24000000, clk_cfg: 0x3A, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 24576000, clk_cfg: 0x3B, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 25000000, clk_cfg: 0x3C, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 25600000, clk_cfg: 0x3D, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 26000000, clk_cfg: 0x3E, fll_igain: 0x0A },
    cs35l36_pll_config { freq: 27000000, clk_cfg: 0x3F, fll_igain: 0x0A },
];

macro_rules! ext_const { ($($name:ident),+ $(,)?) => { $(extern "C" { static $name: c_uint; })+ }; }
ext_const!(
    CS35L36_TESTKEY_CTRL, CS35L36_USERKEY_CTL, CS35L36_OTP_CTRL1, CS35L36_OTP_CTRL2,
    CS35L36_OTP_CTRL3, CS35L36_OTP_CTRL4, CS35L36_OTP_CTRL5, CS35L36_PAC_CTL1,
    CS35L36_PAC_CTL2, CS35L36_PAC_CTL3, CS35L36_PWR_CTRL1, CS35L36_PWR_CTRL2,
    CS35L36_PWR_CTRL3, CS35L36_CTRL_OVRRIDE, CS35L36_AMP_OUT_MUTE,
    CS35L36_OTP_TRIM_STATUS, CS35L36_DISCH_FILT, CS35L36_PROTECT_REL_ERR,
    CS35L36_PAD_INTERFACE, CS35L36_PLL_CLK_CTRL, CS35L36_GLOBAL_CLK_CTRL,
    CS35L36_ADC_CLK_CTRL, CS35L36_SWIRE_CLK_CTRL, CS35L36_SP_SCLK_CLK_CTRL,
    CS35L36_MDSYNC_EN, CS35L36_MDSYNC_TX_ID, CS35L36_MDSYNC_PWR_CTRL,
    CS35L36_MDSYNC_DATA_TX, CS35L36_MDSYNC_TX_STATUS, CS35L36_MDSYNC_RX_STATUS,
    CS35L36_MDSYNC_ERR_STATUS, CS35L36_BSTCVRT_VCTRL1, CS35L36_BSTCVRT_VCTRL2,
    CS35L36_BSTCVRT_PEAK_CUR, CS35L36_BSTCVRT_SFT_RAMP, CS35L36_BSTCVRT_COEFF,
    CS35L36_BSTCVRT_SLOPE_LBST, CS35L36_BSTCVRT_SW_FREQ, CS35L36_BSTCVRT_DCM_CTRL,
    CS35L36_BSTCVRT_DCM_MODE_FORCE, CS35L36_BSTCVRT_OVERVOLT_CTRL,
    CS35L36_VPI_LIMIT_MODE, CS35L36_VPI_LIMIT_MINMAX, CS35L36_VPI_VP_THLD,
    CS35L36_VPI_TRACK_CTRL, CS35L36_VPI_TRIG_MODE_CTRL, CS35L36_VPI_TRIG_STEPS,
    CS35L36_VI_SPKMON_FILT, CS35L36_VI_SPKMON_GAIN, CS35L36_VI_SPKMON_IP_SEL,
    CS35L36_DTEMP_WARN_THLD, CS35L36_DTEMP_STATUS, CS35L36_VPVBST_FS_SEL,
    CS35L36_VPVBST_VP_CTRL, CS35L36_VPVBST_VBST_CTRL, CS35L36_ASP_TX_PIN_CTRL,
    CS35L36_ASP_RATE_CTRL, CS35L36_ASP_FORMAT, CS35L36_ASP_FRAME_CTRL,
    CS35L36_ASP_TX1_TX2_SLOT, CS35L36_ASP_TX3_TX4_SLOT, CS35L36_ASP_TX5_TX6_SLOT,
    CS35L36_ASP_TX7_TX8_SLOT, CS35L36_ASP_RX1_SLOT, CS35L36_ASP_RX_TX_EN,
    CS35L36_ASP_RX1_SEL, CS35L36_ASP_TX1_SEL, CS35L36_ASP_TX2_SEL,
    CS35L36_ASP_TX3_SEL, CS35L36_ASP_TX4_SEL, CS35L36_ASP_TX5_SEL,
    CS35L36_ASP_TX6_SEL, CS35L36_SWIRE_P1_TX1_SEL, CS35L36_SWIRE_P1_TX2_SEL,
    CS35L36_SWIRE_P2_TX1_SEL, CS35L36_SWIRE_P2_TX2_SEL, CS35L36_SWIRE_P2_TX3_SEL,
    CS35L36_SWIRE_DP1_FIFO_CFG, CS35L36_SWIRE_DP2_FIFO_CFG, CS35L36_SWIRE_DP3_FIFO_CFG,
    CS35L36_SWIRE_PCM_RX_DATA, CS35L36_SWIRE_FS_SEL, CS35L36_AMP_DIG_VOL_CTRL,
    CS35L36_VPBR_CFG, CS35L36_VBBR_CFG, CS35L36_VPBR_STATUS, CS35L36_VBBR_STATUS,
    CS35L36_OVERTEMP_CFG, CS35L36_AMP_ERR_VOL, CS35L36_CLASSH_CFG,
    CS35L36_CLASSH_FET_DRV_CFG, CS35L36_NG_CFG, CS35L36_AMP_GAIN_CTRL,
    CS35L36_PWM_MOD_IO_CTRL, CS35L36_PWM_MOD_STATUS, CS35L36_DAC_MSM_CFG,
    CS35L36_AMP_SLOPE_CTRL, CS35L36_AMP_PDM_VOLUME, CS35L36_AMP_PDM_RATE_CTRL,
    CS35L36_PDM_CH_SEL, CS35L36_AMP_NG_CTRL, CS35L36_PDM_HIGHFILT_CTRL,
    CS35L36_PAC_INT0_CTRL, CS35L36_PAC_INT1_CTRL, CS35L36_PAC_INT2_CTRL,
    CS35L36_PAC_INT3_CTRL, CS35L36_PAC_INT4_CTRL, CS35L36_PAC_INT5_CTRL,
    CS35L36_PAC_INT6_CTRL, CS35L36_PAC_INT7_CTRL, CS35L36_SW_RESET,
    CS35L36_SW_REV, CS35L36_HW_REV, CS35L36_OTP_MEM30, CS35L36_DEVICE_ID,
    CS35L36_FAB_ID, CS35L36_REV_ID, CS35L36_TST_FS_MON0, CS35L36_BST_TST_MANUAL,
    CS35L36_BST_ANA2_TEST, CS35L36_INT1_STATUS, CS35L36_INT2_STATUS,
    CS35L36_INT3_STATUS, CS35L36_INT4_STATUS, CS35L36_INT1_RAW_STATUS,
    CS35L36_INT2_RAW_STATUS, CS35L36_INT3_RAW_STATUS, CS35L36_INT4_RAW_STATUS,
    CS35L36_INT1_MASK, CS35L36_INT2_MASK, CS35L36_INT3_MASK, CS35L36_INT4_MASK,
    CS35L36_INT1_EDGE_LVL_CTRL, CS35L36_INT3_EDGE_LVL_CTRL, CS35L36_PAC_INT_STATUS,
    CS35L36_PAC_INT_RAW_STATUS, CS35L36_PAC_INT_FLUSH_CTRL, CS35L36_PAC_PMEM_WORD0,
    CS35L36_PAC_PMEM_WORD1023
);

unsafe fn regv(reg: c_uint) -> c_uint { reg }

unsafe fn cs35l36_reg_defaults() -> [reg_default; 113] {
    [
        reg_default { reg: CS35L36_TESTKEY_CTRL, def: 0x00000000 }, reg_default { reg: CS35L36_USERKEY_CTL, def: 0x00000000 },
        reg_default { reg: CS35L36_OTP_CTRL1, def: 0x00002460 }, reg_default { reg: CS35L36_OTP_CTRL2, def: 0x00000000 },
        reg_default { reg: CS35L36_OTP_CTRL3, def: 0x00000000 }, reg_default { reg: CS35L36_OTP_CTRL4, def: 0x00000000 },
        reg_default { reg: CS35L36_OTP_CTRL5, def: 0x00000000 }, reg_default { reg: CS35L36_PAC_CTL1, def: 0x00000004 },
        reg_default { reg: CS35L36_PAC_CTL2, def: 0x00000000 }, reg_default { reg: CS35L36_PAC_CTL3, def: 0x00000000 },
        reg_default { reg: CS35L36_PWR_CTRL1, def: 0x00000000 }, reg_default { reg: CS35L36_PWR_CTRL2, def: 0x00003321 },
        reg_default { reg: CS35L36_PWR_CTRL3, def: 0x01000010 }, reg_default { reg: CS35L36_CTRL_OVRRIDE, def: 0x00000002 },
        reg_default { reg: CS35L36_AMP_OUT_MUTE, def: 0x00000000 }, reg_default { reg: CS35L36_OTP_TRIM_STATUS, def: 0x00000000 },
        reg_default { reg: CS35L36_DISCH_FILT, def: 0x00000000 }, reg_default { reg: CS35L36_PROTECT_REL_ERR, def: 0x00000000 },
        reg_default { reg: CS35L36_PAD_INTERFACE, def: 0x00000038 }, reg_default { reg: CS35L36_PLL_CLK_CTRL, def: 0x00000010 },
        reg_default { reg: CS35L36_GLOBAL_CLK_CTRL, def: 0x00000003 }, reg_default { reg: CS35L36_ADC_CLK_CTRL, def: 0x00000000 },
        reg_default { reg: CS35L36_SWIRE_CLK_CTRL, def: 0x00000000 }, reg_default { reg: CS35L36_SP_SCLK_CLK_CTRL, def: 0x00000000 },
        reg_default { reg: CS35L36_MDSYNC_EN, def: 0x00000000 }, reg_default { reg: CS35L36_MDSYNC_TX_ID, def: 0x00000000 },
        reg_default { reg: CS35L36_MDSYNC_PWR_CTRL, def: 0x00000000 }, reg_default { reg: CS35L36_MDSYNC_DATA_TX, def: 0x00000000 },
        reg_default { reg: CS35L36_MDSYNC_TX_STATUS, def: 0x00000002 }, reg_default { reg: CS35L36_MDSYNC_RX_STATUS, def: 0x00000000 },
        reg_default { reg: CS35L36_MDSYNC_ERR_STATUS, def: 0x00000000 }, reg_default { reg: CS35L36_BSTCVRT_VCTRL1, def: 0x00000000 },
        reg_default { reg: CS35L36_BSTCVRT_VCTRL2, def: 0x00000001 }, reg_default { reg: CS35L36_BSTCVRT_PEAK_CUR, def: 0x0000004A },
        reg_default { reg: CS35L36_BSTCVRT_SFT_RAMP, def: 0x00000003 }, reg_default { reg: CS35L36_BSTCVRT_COEFF, def: 0x00002424 },
        reg_default { reg: CS35L36_BSTCVRT_SLOPE_LBST, def: 0x00005800 }, reg_default { reg: CS35L36_BSTCVRT_SW_FREQ, def: 0x00010000 },
        reg_default { reg: CS35L36_BSTCVRT_DCM_CTRL, def: 0x00002001 }, reg_default { reg: CS35L36_BSTCVRT_DCM_MODE_FORCE, def: 0x00000000 },
        reg_default { reg: CS35L36_BSTCVRT_OVERVOLT_CTRL, def: 0x00000130 }, reg_default { reg: CS35L36_VPI_LIMIT_MODE, def: 0x00000000 },
        reg_default { reg: CS35L36_VPI_LIMIT_MINMAX, def: 0x00003000 }, reg_default { reg: CS35L36_VPI_VP_THLD, def: 0x00101010 },
        reg_default { reg: CS35L36_VPI_TRACK_CTRL, def: 0x00000000 }, reg_default { reg: CS35L36_VPI_TRIG_MODE_CTRL, def: 0x00000000 },
        reg_default { reg: CS35L36_VPI_TRIG_STEPS, def: 0x00000000 }, reg_default { reg: CS35L36_VI_SPKMON_FILT, def: 0x00000003 },
        reg_default { reg: CS35L36_VI_SPKMON_GAIN, def: 0x00000909 }, reg_default { reg: CS35L36_VI_SPKMON_IP_SEL, def: 0x00000000 },
        reg_default { reg: CS35L36_DTEMP_WARN_THLD, def: 0x00000002 }, reg_default { reg: CS35L36_DTEMP_STATUS, def: 0x00000000 },
        reg_default { reg: CS35L36_VPVBST_FS_SEL, def: 0x00000001 }, reg_default { reg: CS35L36_VPVBST_VP_CTRL, def: 0x000001C0 },
        reg_default { reg: CS35L36_VPVBST_VBST_CTRL, def: 0x000001C0 }, reg_default { reg: CS35L36_ASP_TX_PIN_CTRL, def: 0x00000028 },
        reg_default { reg: CS35L36_ASP_RATE_CTRL, def: 0x00090000 }, reg_default { reg: CS35L36_ASP_FORMAT, def: 0x00000002 },
        reg_default { reg: CS35L36_ASP_FRAME_CTRL, def: 0x00180018 }, reg_default { reg: CS35L36_ASP_TX1_TX2_SLOT, def: 0x00010000 },
        reg_default { reg: CS35L36_ASP_TX3_TX4_SLOT, def: 0x00030002 }, reg_default { reg: CS35L36_ASP_TX5_TX6_SLOT, def: 0x00050004 },
        reg_default { reg: CS35L36_ASP_TX7_TX8_SLOT, def: 0x00070006 }, reg_default { reg: CS35L36_ASP_RX1_SLOT, def: 0x00000000 },
        reg_default { reg: CS35L36_ASP_RX_TX_EN, def: 0x00000000 }, reg_default { reg: CS35L36_ASP_RX1_SEL, def: 0x00000008 },
        reg_default { reg: CS35L36_ASP_TX1_SEL, def: 0x00000018 }, reg_default { reg: CS35L36_ASP_TX2_SEL, def: 0x00000019 },
        reg_default { reg: CS35L36_ASP_TX3_SEL, def: 0x00000028 }, reg_default { reg: CS35L36_ASP_TX4_SEL, def: 0x00000029 },
        reg_default { reg: CS35L36_ASP_TX5_SEL, def: 0x00000020 }, reg_default { reg: CS35L36_ASP_TX6_SEL, def: 0x00000000 },
        reg_default { reg: CS35L36_SWIRE_P1_TX1_SEL, def: 0x00000018 }, reg_default { reg: CS35L36_SWIRE_P1_TX2_SEL, def: 0x00000019 },
        reg_default { reg: CS35L36_SWIRE_P2_TX1_SEL, def: 0x00000028 }, reg_default { reg: CS35L36_SWIRE_P2_TX2_SEL, def: 0x00000029 },
        reg_default { reg: CS35L36_SWIRE_P2_TX3_SEL, def: 0x00000020 }, reg_default { reg: CS35L36_SWIRE_DP1_FIFO_CFG, def: 0x0000001B },
        reg_default { reg: CS35L36_SWIRE_DP2_FIFO_CFG, def: 0x0000001B }, reg_default { reg: CS35L36_SWIRE_DP3_FIFO_CFG, def: 0x0000001B },
        reg_default { reg: CS35L36_SWIRE_PCM_RX_DATA, def: 0x00000000 }, reg_default { reg: CS35L36_SWIRE_FS_SEL, def: 0x00000001 },
        reg_default { reg: CS35L36_AMP_DIG_VOL_CTRL, def: 0x00008000 }, reg_default { reg: CS35L36_VPBR_CFG, def: 0x02AA1905 },
        reg_default { reg: CS35L36_VBBR_CFG, def: 0x02AA1905 }, reg_default { reg: CS35L36_VPBR_STATUS, def: 0x00000000 },
        reg_default { reg: CS35L36_VBBR_STATUS, def: 0x00000000 }, reg_default { reg: CS35L36_OVERTEMP_CFG, def: 0x00000001 },
        reg_default { reg: CS35L36_AMP_ERR_VOL, def: 0x00000000 }, reg_default { reg: CS35L36_CLASSH_CFG, def: 0x000B0405 },
        reg_default { reg: CS35L36_CLASSH_FET_DRV_CFG, def: 0x00000111 }, reg_default { reg: CS35L36_NG_CFG, def: 0x00000033 },
        reg_default { reg: CS35L36_AMP_GAIN_CTRL, def: 0x00000273 }, reg_default { reg: CS35L36_PWM_MOD_IO_CTRL, def: 0x00000000 },
        reg_default { reg: CS35L36_PWM_MOD_STATUS, def: 0x00000000 }, reg_default { reg: CS35L36_DAC_MSM_CFG, def: 0x00000000 },
        reg_default { reg: CS35L36_AMP_SLOPE_CTRL, def: 0x00000B00 }, reg_default { reg: CS35L36_AMP_PDM_VOLUME, def: 0x00000000 },
        reg_default { reg: CS35L36_AMP_PDM_RATE_CTRL, def: 0x00000000 }, reg_default { reg: CS35L36_PDM_CH_SEL, def: 0x00000000 },
        reg_default { reg: CS35L36_AMP_NG_CTRL, def: 0x0000212F }, reg_default { reg: CS35L36_PDM_HIGHFILT_CTRL, def: 0x00000000 },
        reg_default { reg: CS35L36_PAC_INT0_CTRL, def: 0x00000001 }, reg_default { reg: CS35L36_PAC_INT1_CTRL, def: 0x00000001 },
        reg_default { reg: CS35L36_PAC_INT2_CTRL, def: 0x00000001 }, reg_default { reg: CS35L36_PAC_INT3_CTRL, def: 0x00000001 },
        reg_default { reg: CS35L36_PAC_INT4_CTRL, def: 0x00000001 }, reg_default { reg: CS35L36_PAC_INT5_CTRL, def: 0x00000001 },
        reg_default { reg: CS35L36_PAC_INT6_CTRL, def: 0x00000001 }, reg_default { reg: CS35L36_PAC_INT7_CTRL, def: 0x00000001 },
    ]
}

unsafe fn cs35l36_readable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        CS35L36_SW_RESET | CS35L36_SW_REV | CS35L36_HW_REV | CS35L36_TESTKEY_CTRL |
        CS35L36_USERKEY_CTL | CS35L36_OTP_MEM30 | CS35L36_OTP_CTRL1 | CS35L36_OTP_CTRL2 |
        CS35L36_OTP_CTRL3 | CS35L36_OTP_CTRL4 | CS35L36_OTP_CTRL5 | CS35L36_PAC_CTL1 |
        CS35L36_PAC_CTL2 | CS35L36_PAC_CTL3 | CS35L36_DEVICE_ID | CS35L36_FAB_ID |
        CS35L36_REV_ID | CS35L36_PWR_CTRL1 | CS35L36_PWR_CTRL2 | CS35L36_PWR_CTRL3 |
        CS35L36_CTRL_OVRRIDE | CS35L36_AMP_OUT_MUTE | CS35L36_OTP_TRIM_STATUS |
        CS35L36_DISCH_FILT | CS35L36_PROTECT_REL_ERR | CS35L36_PAD_INTERFACE |
        CS35L36_PLL_CLK_CTRL | CS35L36_GLOBAL_CLK_CTRL | CS35L36_ADC_CLK_CTRL |
        CS35L36_SWIRE_CLK_CTRL | CS35L36_SP_SCLK_CLK_CTRL | CS35L36_TST_FS_MON0 |
        CS35L36_MDSYNC_EN | CS35L36_MDSYNC_TX_ID | CS35L36_MDSYNC_PWR_CTRL |
        CS35L36_MDSYNC_DATA_TX | CS35L36_MDSYNC_TX_STATUS | CS35L36_MDSYNC_RX_STATUS |
        CS35L36_MDSYNC_ERR_STATUS | CS35L36_BSTCVRT_VCTRL1 | CS35L36_BSTCVRT_VCTRL2 |
        CS35L36_BSTCVRT_PEAK_CUR | CS35L36_BSTCVRT_SFT_RAMP | CS35L36_BSTCVRT_COEFF |
        CS35L36_BSTCVRT_SLOPE_LBST | CS35L36_BSTCVRT_SW_FREQ | CS35L36_BSTCVRT_DCM_CTRL |
        CS35L36_BSTCVRT_DCM_MODE_FORCE | CS35L36_BSTCVRT_OVERVOLT_CTRL |
        CS35L36_BST_TST_MANUAL | CS35L36_BST_ANA2_TEST | CS35L36_VPI_LIMIT_MODE |
        CS35L36_VPI_LIMIT_MINMAX | CS35L36_VPI_VP_THLD | CS35L36_VPI_TRACK_CTRL |
        CS35L36_VPI_TRIG_MODE_CTRL | CS35L36_VPI_TRIG_STEPS | CS35L36_VI_SPKMON_FILT |
        CS35L36_VI_SPKMON_GAIN | CS35L36_VI_SPKMON_IP_SEL | CS35L36_DTEMP_WARN_THLD |
        CS35L36_DTEMP_STATUS | CS35L36_VPVBST_FS_SEL | CS35L36_VPVBST_VP_CTRL |
        CS35L36_VPVBST_VBST_CTRL | CS35L36_ASP_TX_PIN_CTRL | CS35L36_ASP_RATE_CTRL |
        CS35L36_ASP_FORMAT | CS35L36_ASP_FRAME_CTRL | CS35L36_ASP_TX1_TX2_SLOT |
        CS35L36_ASP_TX3_TX4_SLOT | CS35L36_ASP_TX5_TX6_SLOT | CS35L36_ASP_TX7_TX8_SLOT |
        CS35L36_ASP_RX1_SLOT | CS35L36_ASP_RX_TX_EN | CS35L36_ASP_RX1_SEL |
        CS35L36_ASP_TX1_SEL | CS35L36_ASP_TX2_SEL | CS35L36_ASP_TX3_SEL |
        CS35L36_ASP_TX4_SEL | CS35L36_ASP_TX5_SEL | CS35L36_ASP_TX6_SEL |
        CS35L36_SWIRE_P1_TX1_SEL | CS35L36_SWIRE_P1_TX2_SEL | CS35L36_SWIRE_P2_TX1_SEL |
        CS35L36_SWIRE_P2_TX2_SEL | CS35L36_SWIRE_P2_TX3_SEL | CS35L36_SWIRE_DP1_FIFO_CFG |
        CS35L36_SWIRE_DP2_FIFO_CFG | CS35L36_SWIRE_DP3_FIFO_CFG | CS35L36_SWIRE_PCM_RX_DATA |
        CS35L36_SWIRE_FS_SEL | CS35L36_AMP_DIG_VOL_CTRL | CS35L36_VPBR_CFG |
        CS35L36_VBBR_CFG | CS35L36_VPBR_STATUS | CS35L36_VBBR_STATUS |
        CS35L36_OVERTEMP_CFG | CS35L36_AMP_ERR_VOL | CS35L36_CLASSH_CFG |
        CS35L36_CLASSH_FET_DRV_CFG | CS35L36_NG_CFG | CS35L36_AMP_GAIN_CTRL |
        CS35L36_PWM_MOD_IO_CTRL | CS35L36_PWM_MOD_STATUS | CS35L36_DAC_MSM_CFG |
        CS35L36_AMP_SLOPE_CTRL | CS35L36_AMP_PDM_VOLUME | CS35L36_AMP_PDM_RATE_CTRL |
        CS35L36_PDM_CH_SEL | CS35L36_AMP_NG_CTRL | CS35L36_PDM_HIGHFILT_CTRL |
        CS35L36_INT1_STATUS | CS35L36_INT2_STATUS | CS35L36_INT3_STATUS |
        CS35L36_INT4_STATUS | CS35L36_INT1_RAW_STATUS | CS35L36_INT2_RAW_STATUS |
        CS35L36_INT3_RAW_STATUS | CS35L36_INT4_RAW_STATUS | CS35L36_INT1_MASK |
        CS35L36_INT2_MASK | CS35L36_INT3_MASK | CS35L36_INT4_MASK |
        CS35L36_INT1_EDGE_LVL_CTRL | CS35L36_INT3_EDGE_LVL_CTRL |
        CS35L36_PAC_INT_STATUS | CS35L36_PAC_INT_RAW_STATUS | CS35L36_PAC_INT_FLUSH_CTRL |
        CS35L36_PAC_INT0_CTRL | CS35L36_PAC_INT1_CTRL | CS35L36_PAC_INT2_CTRL |
        CS35L36_PAC_INT3_CTRL | CS35L36_PAC_INT4_CTRL | CS35L36_PAC_INT5_CTRL |
        CS35L36_PAC_INT6_CTRL | CS35L36_PAC_INT7_CTRL => true,
        _ => reg >= CS35L36_PAC_PMEM_WORD0 && reg <= CS35L36_PAC_PMEM_WORD1023,
    }
}

unsafe fn cs35l36_precious_reg(_dev: *mut device, reg: c_uint) -> bool {
    matches!(reg, CS35L36_TESTKEY_CTRL | CS35L36_USERKEY_CTL | CS35L36_TST_FS_MON0)
}

unsafe fn cs35l36_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        CS35L36_SW_RESET | CS35L36_SW_REV | CS35L36_HW_REV | CS35L36_TESTKEY_CTRL |
        CS35L36_USERKEY_CTL | CS35L36_DEVICE_ID | CS35L36_FAB_ID | CS35L36_REV_ID |
        CS35L36_INT1_STATUS | CS35L36_INT2_STATUS | CS35L36_INT3_STATUS |
        CS35L36_INT4_STATUS | CS35L36_INT1_RAW_STATUS | CS35L36_INT2_RAW_STATUS |
        CS35L36_INT3_RAW_STATUS | CS35L36_INT4_RAW_STATUS | CS35L36_INT1_MASK |
        CS35L36_INT2_MASK | CS35L36_INT3_MASK | CS35L36_INT4_MASK |
        CS35L36_INT1_EDGE_LVL_CTRL | CS35L36_INT3_EDGE_LVL_CTRL |
        CS35L36_PAC_INT_STATUS | CS35L36_PAC_INT_RAW_STATUS | CS35L36_PAC_INT_FLUSH_CTRL => true,
        _ => reg >= CS35L36_PAC_PMEM_WORD0 && reg <= CS35L36_PAC_PMEM_WORD1023,
    }
}

extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_bulk_read(map: *mut regmap, reg: c_uint, val: *mut c_uint, val_count: usize) -> c_int;
    fn regmap_register_patch(map: *mut regmap, regs: *const reg_sequence, num_regs: usize) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_crit(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn WARN_ON_ONCE(condition: c_uint) -> c_int;
}

macro_rules! bstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const c_char }; }

unsafe extern "C" fn cs35l36_ldm_sel_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let cs35l36 = snd_soc_component_get_drvdata(component) as *mut cs35l36_private;
    (*ucontrol).value.integer.value[0] = (*cs35l36).ldm_mode_sel;
    0
}

unsafe extern "C" fn cs35l36_ldm_sel_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let cs35l36 = snd_soc_component_get_drvdata(component) as *mut cs35l36_private;
    let val = if (*ucontrol).value.integer.value[0] != 0 { CS35L36_NG_AMP_EN_MASK } else { 0 };
    (*cs35l36).ldm_mode_sel = val as c_int;
    regmap_update_bits((*cs35l36).regmap, CS35L36_NG_CFG, CS35L36_NG_AMP_EN_MASK, val);
    0
}

ext_const!(
    CS35L36_NG_AMP_EN_MASK, SND_SOC_DAPM_POST_PMU, SND_SOC_DAPM_PRE_PMD,
    SND_SOC_DAPM_POST_PMD, CS35L36_GLOBAL_EN_MASK, CS35L36_GLOBAL_EN_SHIFT,
    CS35L36_INT4_RAW_STATUS, CS35L36_PLL_UNLOCK_MASK, CS35L36_PCM_RX_SEL_MASK,
    CS35L36_PCM_RX_SEL_PCM, CS35L36_PCM_RX_SEL_ZERO, CS35L36_AMP_MUTE_MASK,
    CS35L36_AMP_MUTE_SHIFT, CS35L36_BST_EN_MASK, CS35L36_BST_EN,
    CS35L36_BST_EN_SHIFT, CS35L36_BST_DIS_VP, EINVAL
);

unsafe extern "C" fn cs35l36_main_amp_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let cs35l36 = snd_soc_component_get_drvdata(component) as *mut cs35l36_private;
    let mut reg: u32 = 0;
    match event as c_uint {
        SND_SOC_DAPM_POST_PMU => {
            regmap_update_bits((*cs35l36).regmap, CS35L36_PWR_CTRL1, CS35L36_GLOBAL_EN_MASK, 1 << CS35L36_GLOBAL_EN_SHIFT);
            usleep_range(2000, 2100);
            regmap_read((*cs35l36).regmap, CS35L36_INT4_RAW_STATUS, &mut reg);
            if WARN_ON_ONCE(reg & CS35L36_PLL_UNLOCK_MASK) != 0 {
                dev_crit((*cs35l36).dev, bstr!("PLL Unlocked\n"));
            }
            regmap_update_bits((*cs35l36).regmap, CS35L36_ASP_RX1_SEL, CS35L36_PCM_RX_SEL_MASK, CS35L36_PCM_RX_SEL_PCM);
            regmap_update_bits((*cs35l36).regmap, CS35L36_AMP_OUT_MUTE, CS35L36_AMP_MUTE_MASK, 0 << CS35L36_AMP_MUTE_SHIFT);
        }
        SND_SOC_DAPM_PRE_PMD => {
            regmap_update_bits((*cs35l36).regmap, CS35L36_ASP_RX1_SEL, CS35L36_PCM_RX_SEL_MASK, CS35L36_PCM_RX_SEL_ZERO);
            regmap_update_bits((*cs35l36).regmap, CS35L36_AMP_OUT_MUTE, CS35L36_AMP_MUTE_MASK, 1 << CS35L36_AMP_MUTE_SHIFT);
        }
        SND_SOC_DAPM_POST_PMD => {
            regmap_update_bits((*cs35l36).regmap, CS35L36_PWR_CTRL1, CS35L36_GLOBAL_EN_MASK, 0 << CS35L36_GLOBAL_EN_SHIFT);
            usleep_range(2000, 2100);
        }
        _ => {
            dev_dbg((*component).dev, bstr!("Invalid event = 0x%x\n"), event);
            return -(EINVAL as c_int);
        }
    }
    0
}

unsafe extern "C" fn cs35l36_boost_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let cs35l36 = snd_soc_component_get_drvdata(component) as *mut cs35l36_private;
    match event as c_uint {
        SND_SOC_DAPM_POST_PMU => {
            if !(*cs35l36).pdata.extern_boost {
                regmap_update_bits((*cs35l36).regmap, CS35L36_PWR_CTRL2, CS35L36_BST_EN_MASK, CS35L36_BST_EN << CS35L36_BST_EN_SHIFT);
            }
        }
        SND_SOC_DAPM_POST_PMD => {
            if !(*cs35l36).pdata.extern_boost {
                regmap_update_bits((*cs35l36).regmap, CS35L36_PWR_CTRL2, CS35L36_BST_EN_MASK, CS35L36_BST_DIS_VP << CS35L36_BST_EN_SHIFT);
            }
        }
        _ => {
            dev_dbg((*component).dev, bstr!("Invalid event = 0x%x\n"), event);
            return -(EINVAL as c_int);
        }
    }
    0
}

static cs35l36_chan_text: [&[u8]; 2] = [b"RX1\0", b"RX2\0"];
static asp_tx_src_text: [&[u8]; 7] = [b"Zero Fill\0", b"ASPRX1\0", b"VMON\0", b"IMON\0", b"ERRVOL\0", b"VPMON\0", b"VBSTMON\0"];
static asp_tx_src_values: [c_uint; 7] = [0x00, 0x08, 0x18, 0x19, 0x20, 0x28, 0x29];

ext_const!(
    SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK, SND_SOC_DAIFMT_CBP_CFP, SND_SOC_DAIFMT_CBC_CFC,
    CS35L36_SCLK_MSTR_MASK, CS35L36_SCLK_MSTR_SHIFT, CS35L36_LRCLK_MSTR_MASK,
    CS35L36_LRCLK_MSTR_SHIFT, SND_SOC_DAIFMT_CLOCK_MASK, SND_SOC_DAIFMT_CONT,
    SND_SOC_DAIFMT_GATED, CS35L36_SCLK_FRC_MASK, CS35L36_SCLK_FRC_SHIFT,
    CS35L36_LRCLK_FRC_MASK, CS35L36_LRCLK_FRC_SHIFT, SND_SOC_DAIFMT_FORMAT_MASK,
    SND_SOC_DAIFMT_DSP_A, SND_SOC_DAIFMT_I2S, SND_SOC_DAIFMT_INV_MASK,
    SND_SOC_DAIFMT_NB_IF, SND_SOC_DAIFMT_IB_NF, SND_SOC_DAIFMT_IB_IF,
    SND_SOC_DAIFMT_NB_NF, CS35L36_LRCLK_INV_MASK, CS35L36_LRCLK_INV_SHIFT,
    CS35L36_SCLK_INV_MASK, CS35L36_SCLK_INV_SHIFT, CS35L36_ASP_FMT_MASK
);

unsafe extern "C" fn cs35l36_set_dai_fmt(component_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let cs35l36 = snd_soc_component_get_drvdata((*component_dai).component) as *mut cs35l36_private;
    let (asp_fmt, lrclk_fmt, sclk_fmt, clock_provider, clk_frc): (c_uint, c_uint, c_uint, c_uint, c_uint);
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP => clock_provider = 1,
        x if x == SND_SOC_DAIFMT_CBC_CFC => clock_provider = 0,
        _ => return -(EINVAL as c_int),
    }
    regmap_update_bits((*cs35l36).regmap, CS35L36_ASP_TX_PIN_CTRL, CS35L36_SCLK_MSTR_MASK, clock_provider << CS35L36_SCLK_MSTR_SHIFT);
    regmap_update_bits((*cs35l36).regmap, CS35L36_ASP_RATE_CTRL, CS35L36_LRCLK_MSTR_MASK, clock_provider << CS35L36_LRCLK_MSTR_SHIFT);
    match fmt & SND_SOC_DAIFMT_CLOCK_MASK {
        x if x == SND_SOC_DAIFMT_CONT => clk_frc = 1,
        x if x == SND_SOC_DAIFMT_GATED => clk_frc = 0,
        _ => return -(EINVAL as c_int),
    }
    regmap_update_bits((*cs35l36).regmap, CS35L36_ASP_TX_PIN_CTRL, CS35L36_SCLK_FRC_MASK, clk_frc << CS35L36_SCLK_FRC_SHIFT);
    regmap_update_bits((*cs35l36).regmap, CS35L36_ASP_RATE_CTRL, CS35L36_LRCLK_FRC_MASK, clk_frc << CS35L36_LRCLK_FRC_SHIFT);
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_DSP_A => asp_fmt = 0,
        x if x == SND_SOC_DAIFMT_I2S => asp_fmt = 2,
        _ => return -(EINVAL as c_int),
    }
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_IF => { lrclk_fmt = 1; sclk_fmt = 0; }
        x if x == SND_SOC_DAIFMT_IB_NF => { lrclk_fmt = 0; sclk_fmt = 1; }
        x if x == SND_SOC_DAIFMT_IB_IF => { lrclk_fmt = 1; sclk_fmt = 1; }
        x if x == SND_SOC_DAIFMT_NB_NF => { lrclk_fmt = 0; sclk_fmt = 0; }
        _ => return -(EINVAL as c_int),
    }
    regmap_update_bits((*cs35l36).regmap, CS35L36_ASP_RATE_CTRL, CS35L36_LRCLK_INV_MASK, lrclk_fmt << CS35L36_LRCLK_INV_SHIFT);
    regmap_update_bits((*cs35l36).regmap, CS35L36_ASP_TX_PIN_CTRL, CS35L36_SCLK_INV_MASK, sclk_fmt << CS35L36_SCLK_INV_SHIFT);
    regmap_update_bits((*cs35l36).regmap, CS35L36_ASP_FORMAT, CS35L36_ASP_FMT_MASK, asp_fmt);
    0
}

static cs35l36_fs_rates: [cs35l36_global_fs_config; 14] = [
    cs35l36_global_fs_config { rate: 12000, fs_cfg: 0x01 }, cs35l36_global_fs_config { rate: 24000, fs_cfg: 0x02 },
    cs35l36_global_fs_config { rate: 48000, fs_cfg: 0x03 }, cs35l36_global_fs_config { rate: 96000, fs_cfg: 0x04 },
    cs35l36_global_fs_config { rate: 192000, fs_cfg: 0x05 }, cs35l36_global_fs_config { rate: 384000, fs_cfg: 0x06 },
    cs35l36_global_fs_config { rate: 11025, fs_cfg: 0x09 }, cs35l36_global_fs_config { rate: 22050, fs_cfg: 0x0A },
    cs35l36_global_fs_config { rate: 44100, fs_cfg: 0x0B }, cs35l36_global_fs_config { rate: 88200, fs_cfg: 0x0C },
    cs35l36_global_fs_config { rate: 176400, fs_cfg: 0x0D }, cs35l36_global_fs_config { rate: 8000, fs_cfg: 0x11 },
    cs35l36_global_fs_config { rate: 16000, fs_cfg: 0x12 }, cs35l36_global_fs_config { rate: 32000, fs_cfg: 0x13 },
];

ext_const!(
    CS35L36_GLOBAL_FS_MASK, CS35L36_GLOBAL_FS_SHIFT, CS35L36_ASP_WIDTH_16,
    CS35L36_ASP_WIDTH_24, CS35L36_ASP_WIDTH_32, SNDRV_PCM_STREAM_PLAYBACK,
    CS35L36_ASP_RX_WIDTH_MASK, CS35L36_ASP_RX_WIDTH_SHIFT, CS35L36_ASP_TX_WIDTH_MASK,
    CS35L36_ASP_TX_WIDTH_SHIFT
);

unsafe extern "C" fn cs35l36_pcm_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let cs35l36 = snd_soc_component_get_drvdata((*dai).component) as *mut cs35l36_private;
    let global_fs = params_rate(params);
    for r in cs35l36_fs_rates.iter() {
        if global_fs == r.rate as c_uint {
            regmap_update_bits((*cs35l36).regmap, CS35L36_GLOBAL_CLK_CTRL, CS35L36_GLOBAL_FS_MASK, (r.fs_cfg as c_uint) << CS35L36_GLOBAL_FS_SHIFT);
        }
    }
    let asp_width = match params_width(params) {
        16 => CS35L36_ASP_WIDTH_16,
        24 => CS35L36_ASP_WIDTH_24,
        32 => CS35L36_ASP_WIDTH_32,
        _ => return -(EINVAL as c_int),
    };
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK as c_int {
        regmap_update_bits((*cs35l36).regmap, CS35L36_ASP_FRAME_CTRL, CS35L36_ASP_RX_WIDTH_MASK, asp_width << CS35L36_ASP_RX_WIDTH_SHIFT);
    } else {
        regmap_update_bits((*cs35l36).regmap, CS35L36_ASP_FRAME_CTRL, CS35L36_ASP_TX_WIDTH_MASK, asp_width << CS35L36_ASP_TX_WIDTH_SHIFT);
    }
    0
}

ext_const!(
    CS35L36_FS_NOM_6MHZ, CS35L36_FS1_DEFAULT_VAL, CS35L36_FS2_DEFAULT_VAL,
    CS35L36_FS1_WINDOW_MASK, CS35L36_FS2_WINDOW_MASK, CS35L36_FS2_WINDOW_SHIFT,
    CS35L36_TEST_UNLOCK1, CS35L36_TEST_UNLOCK2, CS35L36_TEST_LOCK1, CS35L36_TEST_LOCK2
);

fn div_round_up(n: c_uint, d: c_uint) -> c_uint { (n + d - 1) / d }

unsafe extern "C" fn cs35l36_dai_set_sysclk(dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*dai).component;
    let cs35l36 = snd_soc_component_get_drvdata(component) as *mut cs35l36_private;
    let (fs1, fs2) = if freq > CS35L36_FS_NOM_6MHZ {
        (CS35L36_FS1_DEFAULT_VAL, CS35L36_FS2_DEFAULT_VAL)
    } else {
        (3 * div_round_up(CS35L36_FS_NOM_6MHZ * 4, freq) + 4, 5 * div_round_up(CS35L36_FS_NOM_6MHZ * 4, freq) + 4)
    };
    regmap_write((*cs35l36).regmap, CS35L36_TESTKEY_CTRL, CS35L36_TEST_UNLOCK1);
    regmap_write((*cs35l36).regmap, CS35L36_TESTKEY_CTRL, CS35L36_TEST_UNLOCK2);
    regmap_update_bits((*cs35l36).regmap, CS35L36_TST_FS_MON0, CS35L36_FS1_WINDOW_MASK | CS35L36_FS2_WINDOW_MASK, fs1 | (fs2 << CS35L36_FS2_WINDOW_SHIFT));
    regmap_write((*cs35l36).regmap, CS35L36_TESTKEY_CTRL, CS35L36_TEST_LOCK1);
    regmap_write((*cs35l36).regmap, CS35L36_TESTKEY_CTRL, CS35L36_TEST_LOCK2);
    0
}

unsafe fn cs35l36_get_clk_config(_cs35l36: *mut cs35l36_private, freq: c_int) -> *const cs35l36_pll_config {
    for cfg in cs35l36_pll_sysclk.iter() {
        if cfg.freq == freq {
            return cfg as *const cs35l36_pll_config;
        }
    }
    ptr::null()
}

pub const CS35L36_RATES: c_uint =
    SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_12000 | SNDRV_PCM_RATE_24000 |
    SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_176400 |
    SNDRV_PCM_RATE_192000 | SNDRV_PCM_RATE_384000;
ext_const!(
    SNDRV_PCM_RATE_8000_48000, SNDRV_PCM_RATE_12000, SNDRV_PCM_RATE_24000,
    SNDRV_PCM_RATE_88200, SNDRV_PCM_RATE_96000, SNDRV_PCM_RATE_176400,
    SNDRV_PCM_RATE_192000, SNDRV_PCM_RATE_384000, CS35L36_RX_FORMATS, CS35L36_TX_FORMATS
);

ext_const!(
    CS35L36_PLLSRC_SCLK, CS35L36_PLLSRC_LRCLK, CS35L36_PLLSRC_PDMCLK,
    CS35L36_PLLSRC_SELF, CS35L36_PLLSRC_MCLK, CS35L36_PLL_OPENLOOP_MASK,
    CS35L36_PLL_OPENLOOP_SHIFT, CS35L36_REFCLK_FREQ_MASK, CS35L36_REFCLK_FREQ_SHIFT,
    CS35L36_PLL_REFCLK_EN_MASK, CS35L36_PLL_REFCLK_EN_SHIFT, CS35L36_PLL_CLK_SEL_MASK,
    CS35L36_REV_A0, CS35L36_DCO_CTRL, CS35L36_MISC_CTRL, CS35L36_PLL_LOOP_PARAMS,
    CS35L36_PLL_IGAIN_MASK, CS35L36_PLL_IGAIN, CS35L36_PLL_IGAIN_SHIFT,
    CS35L36_PLL_FFL_IGAIN_MASK, CS35L36_NG_DELAY_MASK, CS35L36_NG_DELAY_SHIFT,
    CS35L36_PDM_MODE_MASK, CS35L36_PDM_MODE_SHIFT
);

unsafe extern "C" fn cs35l36_component_set_sysclk(component: *mut snd_soc_component, clk_id: c_int, _source: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let cs35l36 = snd_soc_component_get_drvdata(component) as *mut cs35l36_private;
    let prev_clksrc = (*cs35l36).clksrc;
    (*cs35l36).clksrc = match clk_id {
        0 => CS35L36_PLLSRC_SCLK as c_int,
        1 => CS35L36_PLLSRC_LRCLK as c_int,
        2 => CS35L36_PLLSRC_PDMCLK as c_int,
        3 => CS35L36_PLLSRC_SELF as c_int,
        4 => CS35L36_PLLSRC_MCLK as c_int,
        _ => return -(EINVAL as c_int),
    };
    let clk_cfg = cs35l36_get_clk_config(cs35l36, freq as c_int);
    if clk_cfg.is_null() {
        dev_err((*component).dev, bstr!("Invalid CLK Config Freq: %d\n"), freq);
        return -(EINVAL as c_int);
    }
    regmap_update_bits((*cs35l36).regmap, CS35L36_PLL_CLK_CTRL, CS35L36_PLL_OPENLOOP_MASK, 1 << CS35L36_PLL_OPENLOOP_SHIFT);
    regmap_update_bits((*cs35l36).regmap, CS35L36_PLL_CLK_CTRL, CS35L36_REFCLK_FREQ_MASK, ((*clk_cfg).clk_cfg as c_uint) << CS35L36_REFCLK_FREQ_SHIFT);
    regmap_update_bits((*cs35l36).regmap, CS35L36_PLL_CLK_CTRL, CS35L36_PLL_REFCLK_EN_MASK, 0 << CS35L36_PLL_REFCLK_EN_SHIFT);
    regmap_update_bits((*cs35l36).regmap, CS35L36_PLL_CLK_CTRL, CS35L36_PLL_CLK_SEL_MASK, (*cs35l36).clksrc as c_uint);
    regmap_update_bits((*cs35l36).regmap, CS35L36_PLL_CLK_CTRL, CS35L36_PLL_OPENLOOP_MASK, 0 << CS35L36_PLL_OPENLOOP_SHIFT);
    regmap_update_bits((*cs35l36).regmap, CS35L36_PLL_CLK_CTRL, CS35L36_PLL_REFCLK_EN_MASK, 1 << CS35L36_PLL_REFCLK_EN_SHIFT);
    if (*cs35l36).rev_id == CS35L36_REV_A0 as c_int {
        regmap_write((*cs35l36).regmap, CS35L36_TESTKEY_CTRL, CS35L36_TEST_UNLOCK1);
        regmap_write((*cs35l36).regmap, CS35L36_TESTKEY_CTRL, CS35L36_TEST_UNLOCK2);
        regmap_write((*cs35l36).regmap, CS35L36_DCO_CTRL, 0x00036DA8);
        regmap_write((*cs35l36).regmap, CS35L36_MISC_CTRL, 0x0100EE0E);
        regmap_update_bits((*cs35l36).regmap, CS35L36_PLL_LOOP_PARAMS, CS35L36_PLL_IGAIN_MASK, CS35L36_PLL_IGAIN << CS35L36_PLL_IGAIN_SHIFT);
        regmap_update_bits((*cs35l36).regmap, CS35L36_PLL_LOOP_PARAMS, CS35L36_PLL_FFL_IGAIN_MASK, (*clk_cfg).fll_igain as c_uint);
        regmap_write((*cs35l36).regmap, CS35L36_TESTKEY_CTRL, CS35L36_TEST_LOCK1);
        regmap_write((*cs35l36).regmap, CS35L36_TESTKEY_CTRL, CS35L36_TEST_LOCK2);
    }
    if (*cs35l36).clksrc == CS35L36_PLLSRC_PDMCLK as c_int {
        let pdm_switch = (*cs35l36).ldm_mode_sel != 0 && prev_clksrc != CS35L36_PLLSRC_PDMCLK as c_int;
        if pdm_switch { regmap_update_bits((*cs35l36).regmap, CS35L36_NG_CFG, CS35L36_NG_DELAY_MASK, 0 << CS35L36_NG_DELAY_SHIFT); }
        regmap_update_bits((*cs35l36).regmap, CS35L36_DAC_MSM_CFG, CS35L36_PDM_MODE_MASK, 1 << CS35L36_PDM_MODE_SHIFT);
        if pdm_switch { regmap_update_bits((*cs35l36).regmap, CS35L36_NG_CFG, CS35L36_NG_DELAY_MASK, 3 << CS35L36_NG_DELAY_SHIFT); }
    } else {
        let pdm_switch = (*cs35l36).ldm_mode_sel != 0 && prev_clksrc == CS35L36_PLLSRC_PDMCLK as c_int;
        if pdm_switch { regmap_update_bits((*cs35l36).regmap, CS35L36_NG_CFG, CS35L36_NG_DELAY_MASK, 0 << CS35L36_NG_DELAY_SHIFT); }
        regmap_update_bits((*cs35l36).regmap, CS35L36_DAC_MSM_CFG, CS35L36_PDM_MODE_MASK, 0 << CS35L36_PDM_MODE_SHIFT);
        if pdm_switch { regmap_update_bits((*cs35l36).regmap, CS35L36_NG_CFG, CS35L36_NG_DELAY_MASK, 3 << CS35L36_NG_DELAY_SHIFT); }
    }
    0
}

ext_const!(
    CS35L36_BSTCVRT_K1_MASK, CS35L36_BSTCVRT_K2_MASK, CS35L36_BSTCVRT_K2_SHIFT,
    CS35L36_BSTCVRT_CCMFREQ_MASK, CS35L36_BSTCVRT_SLOPE_MASK,
    CS35L36_BSTCVRT_SLOPE_SHIFT, CS35L36_BSTCVRT_LBSTVAL_MASK
);

unsafe fn cs35l36_boost_inductor(cs35l36: *mut cs35l36_private, inductor: c_int) -> c_int {
    regmap_update_bits((*cs35l36).regmap, CS35L36_BSTCVRT_COEFF, CS35L36_BSTCVRT_K1_MASK, 0x3C);
    regmap_update_bits((*cs35l36).regmap, CS35L36_BSTCVRT_COEFF, CS35L36_BSTCVRT_K2_MASK, 0x3C << CS35L36_BSTCVRT_K2_SHIFT);
    regmap_update_bits((*cs35l36).regmap, CS35L36_BSTCVRT_SW_FREQ, CS35L36_BSTCVRT_CCMFREQ_MASK, 0x00);
    match inductor {
        1000 => {
            /* 1 uH */
            regmap_update_bits((*cs35l36).regmap, CS35L36_BSTCVRT_SLOPE_LBST, CS35L36_BSTCVRT_SLOPE_MASK, 0x75 << CS35L36_BSTCVRT_SLOPE_SHIFT);
            regmap_update_bits((*cs35l36).regmap, CS35L36_BSTCVRT_SLOPE_LBST, CS35L36_BSTCVRT_LBSTVAL_MASK, 0x00);
        }
        1200 => {
            /* 1.2 uH */
            regmap_update_bits((*cs35l36).regmap, CS35L36_BSTCVRT_SLOPE_LBST, CS35L36_BSTCVRT_SLOPE_MASK, 0x6B << CS35L36_BSTCVRT_SLOPE_SHIFT);
            regmap_update_bits((*cs35l36).regmap, CS35L36_BSTCVRT_SLOPE_LBST, CS35L36_BSTCVRT_LBSTVAL_MASK, 0x01);
        }
        _ => {
            dev_err((*cs35l36).dev, bstr!("%s Invalid Inductor Value %d uH\n"), bstr!("cs35l36_boost_inductor"), inductor);
            return -(EINVAL as c_int);
        }
    }
    0
}

ext_const!(
    CS35L36_DCM_AUTO_MASK, CS35L36_BST_MAN_IPKCOMP_MASK, CS35L36_BST_MAN_IPKCOMP_SHIFT,
    CS35L36_BST_MAN_IPKCOMP_EN_MASK, CS35L36_AMP_PCM_INV_MASK, CS35L36_ASP_TX_HIZ_MASK,
    CS35L36_IMON_POL_MASK, CS35L36_VMON_POL_MASK, CS35L35_BSTCVRT_CTL_MASK,
    CS35L35_BSTCVRT_CTL_SEL_MASK, CS35L36_BST_IPK_MASK, CS35L36_TEMP_THLD_MASK,
    CS35L36_INT_DRV_SEL_MASK, CS35L36_INT_DRV_SEL_SHIFT, CS35L36_INT_GPIO_SEL_MASK,
    CS35L36_INT_GPIO_SEL_SHIFT, CS35L36_10V_L36, CS35L36_BST_OVP_THLD_MASK,
    CS35L36_BST_OVP_THLD_11V, CS35L36_BST_OVP_TRIM_MASK, CS35L36_BST_OVP_TRIM_11V,
    CS35L36_BST_OVP_TRIM_SHIFT, CS35L36_BST_CTRL_LIM_MASK, CS35L36_BST_CTRL_LIM_SHIFT,
    CS35L36_BST_CTRL_10V_CLAMP, CS35L36_SYNC_GLOBAL_OVR_MASK, CS35L36_SYNC_GLOBAL_OVR_SHIFT
);

unsafe extern "C" fn cs35l36_component_probe(component: *mut snd_soc_component) -> c_int {
    let cs35l36 = snd_soc_component_get_drvdata(component) as *mut cs35l36_private;
    let mut ret: c_int;
    if (*cs35l36).rev_id == CS35L36_REV_A0 as c_int && (*cs35l36).pdata.dcm_mode {
        regmap_update_bits((*cs35l36).regmap, CS35L36_BSTCVRT_DCM_CTRL, CS35L36_DCM_AUTO_MASK, CS35L36_DCM_AUTO_MASK);
        regmap_write((*cs35l36).regmap, CS35L36_TESTKEY_CTRL, CS35L36_TEST_UNLOCK1);
        regmap_write((*cs35l36).regmap, CS35L36_TESTKEY_CTRL, CS35L36_TEST_UNLOCK2);
        regmap_update_bits((*cs35l36).regmap, CS35L36_BST_TST_MANUAL, CS35L36_BST_MAN_IPKCOMP_MASK, 0 << CS35L36_BST_MAN_IPKCOMP_SHIFT);
        regmap_update_bits((*cs35l36).regmap, CS35L36_BST_TST_MANUAL, CS35L36_BST_MAN_IPKCOMP_EN_MASK, CS35L36_BST_MAN_IPKCOMP_EN_MASK);
        regmap_write((*cs35l36).regmap, CS35L36_TESTKEY_CTRL, CS35L36_TEST_LOCK1);
        regmap_write((*cs35l36).regmap, CS35L36_TESTKEY_CTRL, CS35L36_TEST_LOCK2);
    }
    if (*cs35l36).pdata.amp_pcm_inv { regmap_update_bits((*cs35l36).regmap, CS35L36_AMP_DIG_VOL_CTRL, CS35L36_AMP_PCM_INV_MASK, CS35L36_AMP_PCM_INV_MASK); }
    if (*cs35l36).pdata.multi_amp_mode { regmap_update_bits((*cs35l36).regmap, CS35L36_ASP_TX_PIN_CTRL, CS35L36_ASP_TX_HIZ_MASK, CS35L36_ASP_TX_HIZ_MASK); }
    if (*cs35l36).pdata.imon_pol_inv { regmap_update_bits((*cs35l36).regmap, CS35L36_VI_SPKMON_FILT, CS35L36_IMON_POL_MASK, 0); }
    if (*cs35l36).pdata.vmon_pol_inv { regmap_update_bits((*cs35l36).regmap, CS35L36_VI_SPKMON_FILT, CS35L36_VMON_POL_MASK, 0); }
    if (*cs35l36).pdata.bst_vctl != 0 { regmap_update_bits((*cs35l36).regmap, CS35L36_BSTCVRT_VCTRL1, CS35L35_BSTCVRT_CTL_MASK, (*cs35l36).pdata.bst_vctl); }
    if (*cs35l36).pdata.bst_vctl_sel != 0 { regmap_update_bits((*cs35l36).regmap, CS35L36_BSTCVRT_VCTRL2, CS35L35_BSTCVRT_CTL_SEL_MASK, (*cs35l36).pdata.bst_vctl_sel); }
    if (*cs35l36).pdata.bst_ipk != 0 { regmap_update_bits((*cs35l36).regmap, CS35L36_BSTCVRT_PEAK_CUR, CS35L36_BST_IPK_MASK, (*cs35l36).pdata.bst_ipk); }
    if (*cs35l36).pdata.boost_ind != 0 {
        ret = cs35l36_boost_inductor(cs35l36, (*cs35l36).pdata.boost_ind as c_int);
        if ret < 0 {
            dev_err((*cs35l36).dev, bstr!("Boost inductor config failed(%d)\n"), ret);
            return ret;
        }
    }
    if (*cs35l36).pdata.temp_warn_thld != 0 { regmap_update_bits((*cs35l36).regmap, CS35L36_DTEMP_WARN_THLD, CS35L36_TEMP_THLD_MASK, (*cs35l36).pdata.temp_warn_thld); }
    if (*cs35l36).pdata.irq_drv_sel != 0 { regmap_update_bits((*cs35l36).regmap, CS35L36_PAD_INTERFACE, CS35L36_INT_DRV_SEL_MASK, (*cs35l36).pdata.irq_drv_sel << CS35L36_INT_DRV_SEL_SHIFT); }
    if (*cs35l36).pdata.irq_gpio_sel != 0 { regmap_update_bits((*cs35l36).regmap, CS35L36_PAD_INTERFACE, CS35L36_INT_GPIO_SEL_MASK, (*cs35l36).pdata.irq_gpio_sel << CS35L36_INT_GPIO_SEL_SHIFT); }
    /*
     * Rev B0 has 2 versions. L36 is 10V, L37 is 12V. If L36 clamp some values
     * for safety after probe has setup dt values.
     */
    if (*cs35l36).chip_version == CS35L36_10V_L36 as c_int {
        regmap_update_bits((*cs35l36).regmap, CS35L36_BSTCVRT_OVERVOLT_CTRL, CS35L36_BST_OVP_THLD_MASK, CS35L36_BST_OVP_THLD_11V);
        regmap_write((*cs35l36).regmap, CS35L36_TESTKEY_CTRL, CS35L36_TEST_UNLOCK1);
        regmap_write((*cs35l36).regmap, CS35L36_TESTKEY_CTRL, CS35L36_TEST_UNLOCK2);
        regmap_update_bits((*cs35l36).regmap, CS35L36_BST_ANA2_TEST, CS35L36_BST_OVP_TRIM_MASK, CS35L36_BST_OVP_TRIM_11V << CS35L36_BST_OVP_TRIM_SHIFT);
        regmap_update_bits((*cs35l36).regmap, CS35L36_BSTCVRT_VCTRL2, CS35L36_BST_CTRL_LIM_MASK, 1 << CS35L36_BST_CTRL_LIM_SHIFT);
        regmap_update_bits((*cs35l36).regmap, CS35L36_BSTCVRT_VCTRL1, CS35L35_BSTCVRT_CTL_MASK, CS35L36_BST_CTRL_10V_CLAMP);
        regmap_write((*cs35l36).regmap, CS35L36_TESTKEY_CTRL, CS35L36_TEST_LOCK1);
        regmap_write((*cs35l36).regmap, CS35L36_TESTKEY_CTRL, CS35L36_TEST_LOCK2);
    }
    /*
     * RevA and B require the disabling of SYNC_GLOBAL_OVR when GLOBAL_EN = 0.
     * Just turn it off from default.
     */
    regmap_update_bits((*cs35l36).regmap, CS35L36_CTRL_OVRRIDE, CS35L36_SYNC_GLOBAL_OVR_MASK, 0 << CS35L36_SYNC_GLOBAL_OVR_SHIFT);
    0
}

ext_const!(
    IRQ_NONE, IRQ_HANDLED, CS35L36_AMP_SHORT_ERR, CS35L36_AMP_SHORT_ERR_RLS,
    CS35L36_TEMP_WARN, CS35L36_TEMP_WARN_ERR_RLS, CS35L36_TEMP_ERR,
    CS35L36_TEMP_ERR_RLS, CS35L36_BST_OVP_ERR, CS35L36_BST_DCM_UVP_ERR,
    CS35L36_BST_UVP_ERR_RLS, CS35L36_BST_SHORT_ERR, CS35L36_BST_SHORT_ERR_RLS
);

unsafe extern "C" fn cs35l36_irq(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let cs35l36 = data as *mut cs35l36_private;
    let mut status = [0u32; 4];
    let mut masks = [0u32; 4];
    let mut ret = IRQ_NONE as c_int;
    /* ack the irq by reading all status registers */
    regmap_bulk_read((*cs35l36).regmap, CS35L36_INT1_STATUS, status.as_mut_ptr(), status.len());
    regmap_bulk_read((*cs35l36).regmap, CS35L36_INT1_MASK, masks.as_mut_ptr(), masks.len());
    /* Check to see if unmasked bits are active */
    if (status[0] & !masks[0]) == 0 && (status[1] & !masks[1]) == 0 && (status[2] & !masks[2]) == 0 && (status[3] & !masks[3]) == 0 {
        return IRQ_NONE as c_int;
    }
    /*
     * The following interrupts require a protection release cycle to get the
     * speaker out of Safe-Mode.
     */
    if status[2] & CS35L36_AMP_SHORT_ERR != 0 {
        dev_crit((*cs35l36).dev, bstr!("Amp short error\n"));
        regmap_update_bits((*cs35l36).regmap, CS35L36_PROTECT_REL_ERR, CS35L36_AMP_SHORT_ERR_RLS, 0);
        regmap_update_bits((*cs35l36).regmap, CS35L36_PROTECT_REL_ERR, CS35L36_AMP_SHORT_ERR_RLS, CS35L36_AMP_SHORT_ERR_RLS);
        regmap_update_bits((*cs35l36).regmap, CS35L36_PROTECT_REL_ERR, CS35L36_AMP_SHORT_ERR_RLS, 0);
        regmap_update_bits((*cs35l36).regmap, CS35L36_INT3_STATUS, CS35L36_AMP_SHORT_ERR, CS35L36_AMP_SHORT_ERR);
        ret = IRQ_HANDLED as c_int;
    }
    if status[0] & CS35L36_TEMP_WARN != 0 {
        dev_crit((*cs35l36).dev, bstr!("Over temperature warning\n"));
        regmap_update_bits((*cs35l36).regmap, CS35L36_PROTECT_REL_ERR, CS35L36_TEMP_WARN_ERR_RLS, 0);
        regmap_update_bits((*cs35l36).regmap, CS35L36_PROTECT_REL_ERR, CS35L36_TEMP_WARN_ERR_RLS, CS35L36_TEMP_WARN_ERR_RLS);
        regmap_update_bits((*cs35l36).regmap, CS35L36_PROTECT_REL_ERR, CS35L36_TEMP_WARN_ERR_RLS, 0);
        regmap_update_bits((*cs35l36).regmap, CS35L36_INT1_STATUS, CS35L36_TEMP_WARN, CS35L36_TEMP_WARN);
        ret = IRQ_HANDLED as c_int;
    }
    if status[0] & CS35L36_TEMP_ERR != 0 {
        dev_crit((*cs35l36).dev, bstr!("Over temperature error\n"));
        regmap_update_bits((*cs35l36).regmap, CS35L36_PROTECT_REL_ERR, CS35L36_TEMP_ERR_RLS, 0);
        regmap_update_bits((*cs35l36).regmap, CS35L36_PROTECT_REL_ERR, CS35L36_TEMP_ERR_RLS, CS35L36_TEMP_ERR_RLS);
        regmap_update_bits((*cs35l36).regmap, CS35L36_PROTECT_REL_ERR, CS35L36_TEMP_ERR_RLS, 0);
        regmap_update_bits((*cs35l36).regmap, CS35L36_INT1_STATUS, CS35L36_TEMP_ERR, CS35L36_TEMP_ERR);
        ret = IRQ_HANDLED as c_int;
    }
    if status[0] & CS35L36_BST_OVP_ERR != 0 {
        dev_crit((*cs35l36).dev, bstr!("VBST Over Voltage error\n"));
        regmap_update_bits((*cs35l36).regmap, CS35L36_PROTECT_REL_ERR, CS35L36_TEMP_ERR_RLS, 0);
        regmap_update_bits((*cs35l36).regmap, CS35L36_PROTECT_REL_ERR, CS35L36_TEMP_ERR_RLS, CS35L36_TEMP_ERR_RLS);
        regmap_update_bits((*cs35l36).regmap, CS35L36_PROTECT_REL_ERR, CS35L36_TEMP_ERR_RLS, 0);
        regmap_update_bits((*cs35l36).regmap, CS35L36_INT1_STATUS, CS35L36_BST_OVP_ERR, CS35L36_BST_OVP_ERR);
        ret = IRQ_HANDLED as c_int;
    }
    if status[0] & CS35L36_BST_DCM_UVP_ERR != 0 {
        dev_crit((*cs35l36).dev, bstr!("DCM VBST Under Voltage Error\n"));
        regmap_update_bits((*cs35l36).regmap, CS35L36_PROTECT_REL_ERR, CS35L36_BST_UVP_ERR_RLS, 0);
        regmap_update_bits((*cs35l36).regmap, CS35L36_PROTECT_REL_ERR, CS35L36_BST_UVP_ERR_RLS, CS35L36_BST_UVP_ERR_RLS);
        regmap_update_bits((*cs35l36).regmap, CS35L36_PROTECT_REL_ERR, CS35L36_BST_UVP_ERR_RLS, 0);
        regmap_update_bits((*cs35l36).regmap, CS35L36_INT1_STATUS, CS35L36_BST_DCM_UVP_ERR, CS35L36_BST_DCM_UVP_ERR);
        ret = IRQ_HANDLED as c_int;
    }
    if status[0] & CS35L36_BST_SHORT_ERR != 0 {
        dev_crit((*cs35l36).dev, bstr!("LBST SHORT error!\n"));
        regmap_update_bits((*cs35l36).regmap, CS35L36_PROTECT_REL_ERR, CS35L36_BST_SHORT_ERR_RLS, 0);
        regmap_update_bits((*cs35l36).regmap, CS35L36_PROTECT_REL_ERR, CS35L36_BST_SHORT_ERR_RLS, CS35L36_BST_SHORT_ERR_RLS);
        regmap_update_bits((*cs35l36).regmap, CS35L36_PROTECT_REL_ERR, CS35L36_BST_SHORT_ERR_RLS, 0);
        regmap_update_bits((*cs35l36).regmap, CS35L36_INT1_STATUS, CS35L36_BST_SHORT_ERR, CS35L36_BST_SHORT_ERR);
        ret = IRQ_HANDLED as c_int;
    }
    ret
}

extern "C" {
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut c_uint) -> c_int;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool;
    fn of_get_child_by_name(np: *mut device_node, name: *const c_char) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
}

unsafe extern "C" fn cs35l36_handle_of_data(i2c_client: *mut i2c_client, pdata: *mut cs35l36_platform_data) -> c_int {
    let np = (*(ptr::addr_of_mut!((*i2c_client).dev) as *mut device_with_of_node)).of_node;
    let vpbr_config = &mut (*pdata).vpbr_config as *mut cs35l36_vpbr_cfg;
    let mut val: c_uint = 0;
    let mut ret: c_int;
    if np.is_null() { return 0; }
    ret = of_property_read_u32(np, bstr!("cirrus,boost-ctl-millivolt"), &mut val);
    if ret == 0 {
        if val < 2550 || val > 12000 {
            dev_err(&mut (*i2c_client).dev, bstr!("Invalid Boost Voltage %d mV\n"), val);
            return -(EINVAL as c_int);
        }
        (*pdata).bst_vctl = (((val - 2550) / 100) + 1) << 1;
    } else {
        dev_err(&mut (*i2c_client).dev, bstr!("Unable to find required parameter 'cirrus,boost-ctl-millivolt'"));
        return -(EINVAL as c_int);
    }
    ret = of_property_read_u32(np, bstr!("cirrus,boost-ctl-select"), &mut val);
    if ret == 0 { (*pdata).bst_vctl_sel = val | CS35L36_VALID_PDATA; }
    ret = of_property_read_u32(np, bstr!("cirrus,boost-peak-milliamp"), &mut val);
    if ret == 0 {
        if val < 1600 || val > 4500 {
            dev_err(&mut (*i2c_client).dev, bstr!("Invalid Boost Peak Current %u mA\n"), val);
            return -(EINVAL as c_int);
        }
        (*pdata).bst_ipk = (val - 1600) / 50;
    } else {
        dev_err(&mut (*i2c_client).dev, bstr!("Unable to find required parameter 'cirrus,boost-peak-milliamp'"));
        return -(EINVAL as c_int);
    }
    (*pdata).multi_amp_mode = of_property_read_bool(np, bstr!("cirrus,multi-amp-mode"));
    (*pdata).dcm_mode = of_property_read_bool(np, bstr!("cirrus,dcm-mode-enable"));
    (*pdata).amp_pcm_inv = of_property_read_bool(np, bstr!("cirrus,amp-pcm-inv"));
    (*pdata).imon_pol_inv = of_property_read_bool(np, bstr!("cirrus,imon-pol-inv"));
    (*pdata).vmon_pol_inv = of_property_read_bool(np, bstr!("cirrus,vmon-pol-inv"));
    if of_property_read_u32(np, bstr!("cirrus,temp-warn-threshold"), &mut val) >= 0 { (*pdata).temp_warn_thld = val | CS35L36_VALID_PDATA; }
    if of_property_read_u32(np, bstr!("cirrus,boost-ind-nanohenry"), &mut val) >= 0 {
        (*pdata).boost_ind = val;
    } else {
        dev_err(&mut (*i2c_client).dev, bstr!("Inductor not specified.\n"));
        return -(EINVAL as c_int);
    }
    if of_property_read_u32(np, bstr!("cirrus,irq-drive-select"), &mut val) >= 0 { (*pdata).irq_drv_sel = val | CS35L36_VALID_PDATA; }
    if of_property_read_u32(np, bstr!("cirrus,irq-gpio-select"), &mut val) >= 0 { (*pdata).irq_gpio_sel = val | CS35L36_VALID_PDATA; }
    /* VPBR Config */
    let vpbr_node = of_get_child_by_name(np, bstr!("cirrus,vpbr-config"));
    (*vpbr_config).is_present = !vpbr_node.is_null();
    if (*vpbr_config).is_present {
        if of_property_read_u32(vpbr_node, bstr!("cirrus,vpbr-en"), &mut val) >= 0 { (*vpbr_config).vpbr_en = val; }
        if of_property_read_u32(vpbr_node, bstr!("cirrus,vpbr-thld"), &mut val) >= 0 { (*vpbr_config).vpbr_thld = val; }
        if of_property_read_u32(vpbr_node, bstr!("cirrus,vpbr-atk-rate"), &mut val) >= 0 { (*vpbr_config).vpbr_atk_rate = val; }
        if of_property_read_u32(vpbr_node, bstr!("cirrus,vpbr-atk-vol"), &mut val) >= 0 { (*vpbr_config).vpbr_atk_vol = val; }
        if of_property_read_u32(vpbr_node, bstr!("cirrus,vpbr-max-attn"), &mut val) >= 0 { (*vpbr_config).vpbr_max_attn = val; }
        if of_property_read_u32(vpbr_node, bstr!("cirrus,vpbr-wait"), &mut val) >= 0 { (*vpbr_config).vpbr_wait = val; }
        if of_property_read_u32(vpbr_node, bstr!("cirrus,vpbr-rel-rate"), &mut val) >= 0 { (*vpbr_config).vpbr_rel_rate = val; }
        if of_property_read_u32(vpbr_node, bstr!("cirrus,vpbr-mute-en"), &mut val) >= 0 { (*vpbr_config).vpbr_mute_en = val; }
    }
    of_node_put(vpbr_node);
    0
}

#[repr(C)] struct device_with_of_node { pub of_node: *mut device_node }

ext_const!(
    CS35L36_REV_B0, CS35L36_PAC_RESET, CS35L36_PAC_MEM_ACCESS, CS35L36_B0_PAC_PATCH,
    CS35L36_PAC_MEM_ACCESS_CLR, CS35L36_PAC_ENABLE_MASK, CS35L36_MCU_CONFIG_CLR
);

unsafe fn cs35l36_pac(cs35l36: *mut cs35l36_private) -> c_int {
    let mut ret: c_int;
    let mut count: c_int;
    let mut val: c_uint = 0;
    if (*cs35l36).rev_id != CS35L36_REV_B0 as c_int { return 0; }
    /*
     * Magic code for internal PAC
     */
    regmap_write((*cs35l36).regmap, CS35L36_TESTKEY_CTRL, CS35L36_TEST_UNLOCK1);
    regmap_write((*cs35l36).regmap, CS35L36_TESTKEY_CTRL, CS35L36_TEST_UNLOCK2);
    usleep_range(9500, 10500);
    regmap_write((*cs35l36).regmap, CS35L36_PAC_CTL1, CS35L36_PAC_RESET);
    regmap_write((*cs35l36).regmap, CS35L36_PAC_CTL3, CS35L36_PAC_MEM_ACCESS);
    regmap_write((*cs35l36).regmap, CS35L36_PAC_PMEM_WORD0, CS35L36_B0_PAC_PATCH);
    regmap_write((*cs35l36).regmap, CS35L36_PAC_CTL3, CS35L36_PAC_MEM_ACCESS_CLR);
    regmap_write((*cs35l36).regmap, CS35L36_PAC_CTL1, CS35L36_PAC_ENABLE_MASK);
    usleep_range(9500, 10500);
    ret = regmap_read((*cs35l36).regmap, CS35L36_INT4_STATUS, &mut val);
    if ret < 0 {
        dev_err((*cs35l36).dev, bstr!("Failed to read int4_status %d\n"), ret);
        return ret;
    }
    count = 0;
    while val & CS35L36_MCU_CONFIG_CLR == 0 {
        usleep_range(100, 200);
        count += 1;
        ret = regmap_read((*cs35l36).regmap, CS35L36_INT4_STATUS, &mut val);
        if ret < 0 {
            dev_err((*cs35l36).dev, bstr!("Failed to read int4_status %d\n"), ret);
            return ret;
        }
        if count >= 100 { return -(EINVAL as c_int); }
    }
    regmap_write((*cs35l36).regmap, CS35L36_INT4_STATUS, CS35L36_MCU_CONFIG_CLR);
    regmap_update_bits((*cs35l36).regmap, CS35L36_PAC_CTL1, CS35L36_PAC_ENABLE_MASK, 0);
    regmap_write((*cs35l36).regmap, CS35L36_TESTKEY_CTRL, CS35L36_TEST_LOCK1);
    regmap_write((*cs35l36).regmap, CS35L36_TESTKEY_CTRL, CS35L36_TEST_LOCK2);
    0
}

ext_const!(
    CS35L36_VPBR_EN_MASK, CS35L36_VPBR_EN_SHIFT, CS35L36_VPBR_THLD_MASK,
    CS35L36_VPBR_THLD_SHIFT, CS35L36_VPBR_MAX_ATTN_MASK, CS35L36_VPBR_MAX_ATTN_SHIFT,
    CS35L36_VPBR_ATK_VOL_MASK, CS35L36_VPBR_ATK_VOL_SHIFT, CS35L36_VPBR_ATK_RATE_MASK,
    CS35L36_VPBR_ATK_RATE_SHIFT, CS35L36_VPBR_WAIT_MASK, CS35L36_VPBR_WAIT_SHIFT,
    CS35L36_VPBR_REL_RATE_MASK, CS35L36_VPBR_REL_RATE_SHIFT, CS35L36_VPBR_MUTE_EN_MASK,
    CS35L36_VPBR_MUTE_EN_SHIFT
);

unsafe fn cs35l36_apply_vpbr_config(cs35l36: *mut cs35l36_private) {
    let pdata = &mut (*cs35l36).pdata as *mut cs35l36_platform_data;
    let vpbr_config = &mut (*pdata).vpbr_config as *mut cs35l36_vpbr_cfg;
    regmap_update_bits((*cs35l36).regmap, CS35L36_PWR_CTRL3, CS35L36_VPBR_EN_MASK, (*vpbr_config).vpbr_en << CS35L36_VPBR_EN_SHIFT);
    regmap_update_bits((*cs35l36).regmap, CS35L36_VPBR_CFG, CS35L36_VPBR_THLD_MASK, (*vpbr_config).vpbr_thld << CS35L36_VPBR_THLD_SHIFT);
    regmap_update_bits((*cs35l36).regmap, CS35L36_VPBR_CFG, CS35L36_VPBR_MAX_ATTN_MASK, (*vpbr_config).vpbr_max_attn << CS35L36_VPBR_MAX_ATTN_SHIFT);
    regmap_update_bits((*cs35l36).regmap, CS35L36_VPBR_CFG, CS35L36_VPBR_ATK_VOL_MASK, (*vpbr_config).vpbr_atk_vol << CS35L36_VPBR_ATK_VOL_SHIFT);
    regmap_update_bits((*cs35l36).regmap, CS35L36_VPBR_CFG, CS35L36_VPBR_ATK_RATE_MASK, (*vpbr_config).vpbr_atk_rate << CS35L36_VPBR_ATK_RATE_SHIFT);
    regmap_update_bits((*cs35l36).regmap, CS35L36_VPBR_CFG, CS35L36_VPBR_WAIT_MASK, (*vpbr_config).vpbr_wait << CS35L36_VPBR_WAIT_SHIFT);
    regmap_update_bits((*cs35l36).regmap, CS35L36_VPBR_CFG, CS35L36_VPBR_REL_RATE_MASK, (*vpbr_config).vpbr_rel_rate << CS35L36_VPBR_REL_RATE_SHIFT);
    regmap_update_bits((*cs35l36).regmap, CS35L36_VPBR_CFG, CS35L36_VPBR_MUTE_EN_MASK, (*vpbr_config).vpbr_mute_en << CS35L36_VPBR_MUTE_EN_SHIFT);
}

unsafe fn cs35l36_reva0_errata_patch() -> [reg_sequence; 33] {
    [
        reg_sequence { reg: CS35L36_TESTKEY_CTRL, def: CS35L36_TEST_UNLOCK1 }, reg_sequence { reg: CS35L36_TESTKEY_CTRL, def: CS35L36_TEST_UNLOCK2 },
        /* Errata Writes */
        reg_sequence { reg: CS35L36_OTP_CTRL1, def: 0x00002060 }, reg_sequence { reg: CS35L36_OTP_CTRL2, def: 0x00000001 },
        reg_sequence { reg: CS35L36_OTP_CTRL1, def: 0x00002460 }, reg_sequence { reg: CS35L36_OTP_CTRL2, def: 0x00000001 },
        reg_sequence { reg: 0x00002088, def: 0x012A1838 }, reg_sequence { reg: 0x00003014, def: 0x0100EE0E },
        reg_sequence { reg: 0x00003008, def: 0x0008184A }, reg_sequence { reg: 0x00007418, def: 0x509001C8 },
        reg_sequence { reg: 0x00007064, def: 0x0929A800 }, reg_sequence { reg: 0x00002D10, def: 0x0002C01C },
        reg_sequence { reg: 0x0000410C, def: 0x00000A11 }, reg_sequence { reg: 0x00006E08, def: 0x8B19140C },
        reg_sequence { reg: 0x00006454, def: 0x0300000A }, reg_sequence { reg: CS35L36_AMP_NG_CTRL, def: 0x000020EF },
        reg_sequence { reg: 0x00007E34, def: 0x0000000E }, reg_sequence { reg: 0x0000410C, def: 0x00000A11 },
        reg_sequence { reg: 0x00007410, def: 0x20514B00 },
        /* PAC Config */
        reg_sequence { reg: CS35L36_CTRL_OVRRIDE, def: 0x00000000 },
        reg_sequence { reg: CS35L36_PAC_INT0_CTRL, def: 0x00860001 }, reg_sequence { reg: CS35L36_PAC_INT1_CTRL, def: 0x00860001 },
        reg_sequence { reg: CS35L36_PAC_INT2_CTRL, def: 0x00860001 }, reg_sequence { reg: CS35L36_PAC_INT3_CTRL, def: 0x00860001 },
        reg_sequence { reg: CS35L36_PAC_INT4_CTRL, def: 0x00860001 }, reg_sequence { reg: CS35L36_PAC_INT5_CTRL, def: 0x00860001 },
        reg_sequence { reg: CS35L36_PAC_INT6_CTRL, def: 0x00860001 }, reg_sequence { reg: CS35L36_PAC_INT7_CTRL, def: 0x00860001 },
        reg_sequence { reg: CS35L36_PAC_INT_FLUSH_CTRL, def: 0x000000FF },
        reg_sequence { reg: CS35L36_TESTKEY_CTRL, def: CS35L36_TEST_LOCK1 }, reg_sequence { reg: CS35L36_TESTKEY_CTRL, def: CS35L36_TEST_LOCK2 },
    ]
}

unsafe fn cs35l36_revb0_errata_patch() -> [reg_sequence; 16] {
    [
        reg_sequence { reg: CS35L36_TESTKEY_CTRL, def: CS35L36_TEST_UNLOCK1 }, reg_sequence { reg: CS35L36_TESTKEY_CTRL, def: CS35L36_TEST_UNLOCK2 },
        reg_sequence { reg: 0x00007064, def: 0x0929A800 }, reg_sequence { reg: 0x00007850, def: 0x00002FA9 },
        reg_sequence { reg: 0x00007854, def: 0x0003F1D5 }, reg_sequence { reg: 0x00007858, def: 0x0003F5E3 },
        reg_sequence { reg: 0x0000785C, def: 0x00001137 }, reg_sequence { reg: 0x00007860, def: 0x0001A7A5 },
        reg_sequence { reg: 0x00007864, def: 0x0002F16A }, reg_sequence { reg: 0x00007868, def: 0x00003E21 },
        reg_sequence { reg: 0x00007848, def: 0x00000001 }, reg_sequence { reg: 0x00003854, def: 0x05180240 },
        reg_sequence { reg: 0x00007418, def: 0x509001C8 }, reg_sequence { reg: 0x0000394C, def: 0x028764BD },
        reg_sequence { reg: CS35L36_TESTKEY_CTRL, def: CS35L36_TEST_LOCK1 }, reg_sequence { reg: CS35L36_TESTKEY_CTRL, def: CS35L36_TEST_LOCK2 },
    ]
}

extern "C" {
    fn dev_get_platdata(dev: *mut device) -> *mut cs35l36_platform_data;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const c_void) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_regulator_bulk_get(dev: *mut device, num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_enable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn irq_get_irq_data(irq: c_uint) -> *mut irq_data;
    fn irqd_get_trigger_type(d: *mut irq_data) -> c_int;
    fn devm_request_threaded_irq(dev: *mut device, irq: c_uint, handler: *const c_void, thread_fn: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, irqflags: c_uint, devname: *const c_char, dev_id: *mut c_void) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const c_void, dai_drv: *mut c_void, num_dai: c_int) -> c_int;
}

ext_const!(
    GFP_KERNEL, ENOMEM, GPIOD_OUT_LOW, EBUSY, ENODEV, CS35L36_CHIP_ID,
    CS35L36_OTP_REV_MASK, CS35L36_OTP_REV_L37, CS35L36_12V_L37,
    IRQF_TRIGGER_FALLING, IRQF_TRIGGER_LOW, IRQF_TRIGGER_RISING, IRQF_TRIGGER_HIGH,
    CS35L36_INT_POL_SEL_MASK, CS35L36_INT_POL_SEL_SHIFT, IRQF_ONESHOT,
    CS35L36_INT_OUTPUT_EN_MASK, CS35L36_INT1_MASK_DEFAULT, CS35L36_INT3_MASK_DEFAULT,
    CS35L36_INT1_MASK_RESET, CS35L36_INT3_MASK_RESET
);

static mut soc_component_dev_cs35l36: *const c_void = ptr::null();
static mut cs35l36_dai: *mut c_void = ptr::null_mut();

unsafe extern "C" fn cs35l36_i2c_probe(i2c_client: *mut i2c_client) -> c_int {
    let dev = &mut (*i2c_client).dev as *mut device;
    let mut pdata = dev_get_platdata(dev);
    let cs35l36 = devm_kzalloc(dev, core::mem::size_of::<cs35l36_private>(), GFP_KERNEL) as *mut cs35l36_private;
    if cs35l36.is_null() { return -(ENOMEM as c_int); }
    (*cs35l36).dev = dev;
    i2c_set_clientdata(i2c_client, cs35l36 as *mut c_void);
    (*cs35l36).regmap = devm_regmap_init_i2c(i2c_client, ptr::null());
    if IS_ERR((*cs35l36).regmap as *const c_void) {
        let ret = PTR_ERR((*cs35l36).regmap as *const c_void);
        dev_err(dev, bstr!("regmap_init() failed: %d\n"), ret);
        return ret;
    }
    (*cs35l36).num_supplies = cs35l36_supplies.len() as c_int;
    for i in 0..cs35l36_supplies.len() {
        (*cs35l36).supplies[i].supply = cs35l36_supplies[i];
    }
    let mut ret = devm_regulator_bulk_get(dev, (*cs35l36).num_supplies, (*cs35l36).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, bstr!("Failed to request core supplies: %d\n"), ret);
        return ret;
    }
    if !pdata.is_null() {
        (*cs35l36).pdata = *pdata;
    } else {
        pdata = devm_kzalloc(dev, core::mem::size_of::<cs35l36_platform_data>(), GFP_KERNEL) as *mut cs35l36_platform_data;
        if pdata.is_null() { return -(ENOMEM as c_int); }
        if !(*(ptr::addr_of_mut!((*i2c_client).dev) as *mut device_with_of_node)).of_node.is_null() {
            ret = cs35l36_handle_of_data(i2c_client, pdata);
            if ret != 0 { return ret; }
        }
        (*cs35l36).pdata = *pdata;
    }
    ret = regulator_bulk_enable((*cs35l36).num_supplies, (*cs35l36).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, bstr!("Failed to enable core supplies: %d\n"), ret);
        return ret;
    }
    /* returning NULL can be an option if in stereo mode */
    (*cs35l36).reset_gpio = devm_gpiod_get_optional(dev, bstr!("reset"), GPIOD_OUT_LOW);
    if IS_ERR((*cs35l36).reset_gpio as *const c_void) {
        ret = PTR_ERR((*cs35l36).reset_gpio as *const c_void);
        (*cs35l36).reset_gpio = ptr::null_mut();
        if ret == -(EBUSY as c_int) {
            dev_info(dev, bstr!("Reset line busy, assuming shared reset\n"));
        } else {
            dev_err(dev, bstr!("Failed to get reset GPIO: %d\n"), ret);
            regulator_bulk_disable((*cs35l36).num_supplies, (*cs35l36).supplies.as_mut_ptr());
            return ret;
        }
    }
    if !(*cs35l36).reset_gpio.is_null() { gpiod_set_value_cansleep((*cs35l36).reset_gpio, 1); }
    usleep_range(2000, 2100);
    let mut reg_id: u32 = 0;
    ret = regmap_read((*cs35l36).regmap, CS35L36_SW_RESET, &mut reg_id);
    if ret < 0 {
        dev_err(dev, bstr!("Get Device ID failed %d\n"), ret);
        gpiod_set_value_cansleep((*cs35l36).reset_gpio, 0);
        regulator_bulk_disable((*cs35l36).num_supplies, (*cs35l36).supplies.as_mut_ptr());
        return ret;
    }
    if reg_id != CS35L36_CHIP_ID {
        dev_err(dev, bstr!("Device ID (%X). Expected ID %X\n"), reg_id, CS35L36_CHIP_ID);
        ret = -(ENODEV as c_int);
        gpiod_set_value_cansleep((*cs35l36).reset_gpio, 0);
        regulator_bulk_disable((*cs35l36).num_supplies, (*cs35l36).supplies.as_mut_ptr());
        return ret;
    }
    let mut reg_revid: u32 = 0;
    ret = regmap_read((*cs35l36).regmap, CS35L36_REV_ID, &mut reg_revid);
    if ret < 0 {
        dev_err(&mut (*i2c_client).dev, bstr!("Get Revision ID failed %d\n"), ret);
        gpiod_set_value_cansleep((*cs35l36).reset_gpio, 0);
        regulator_bulk_disable((*cs35l36).num_supplies, (*cs35l36).supplies.as_mut_ptr());
        return ret;
    }
    (*cs35l36).rev_id = (reg_revid >> 8) as c_int;
    let mut l37_id_reg: u32 = 0;
    ret = regmap_read((*cs35l36).regmap, CS35L36_OTP_MEM30, &mut l37_id_reg);
    if ret < 0 {
        dev_err(&mut (*i2c_client).dev, bstr!("Failed to read otp_id Register %d\n"), ret);
        gpiod_set_value_cansleep((*cs35l36).reset_gpio, 0);
        regulator_bulk_disable((*cs35l36).num_supplies, (*cs35l36).supplies.as_mut_ptr());
        return ret;
    }
    if (l37_id_reg & CS35L36_OTP_REV_MASK) == CS35L36_OTP_REV_L37 {
        (*cs35l36).chip_version = CS35L36_12V_L37 as c_int;
    } else {
        (*cs35l36).chip_version = CS35L36_10V_L36 as c_int;
    }
    match (*cs35l36).rev_id {
        x if x == CS35L36_REV_A0 as c_int => {
            let patch = cs35l36_reva0_errata_patch();
            ret = regmap_register_patch((*cs35l36).regmap, patch.as_ptr(), patch.len());
            if ret < 0 {
                dev_err(dev, bstr!("Failed to apply A0 errata patch %d\n"), ret);
                gpiod_set_value_cansleep((*cs35l36).reset_gpio, 0);
                regulator_bulk_disable((*cs35l36).num_supplies, (*cs35l36).supplies.as_mut_ptr());
                return ret;
            }
        }
        x if x == CS35L36_REV_B0 as c_int => {
            ret = cs35l36_pac(cs35l36);
            if ret < 0 {
                dev_err(dev, bstr!("Failed to Trim OTP %d\n"), ret);
                gpiod_set_value_cansleep((*cs35l36).reset_gpio, 0);
                regulator_bulk_disable((*cs35l36).num_supplies, (*cs35l36).supplies.as_mut_ptr());
                return ret;
            }
            let patch = cs35l36_revb0_errata_patch();
            ret = regmap_register_patch((*cs35l36).regmap, patch.as_ptr(), patch.len());
            if ret < 0 {
                dev_err(dev, bstr!("Failed to apply B0 errata patch %d\n"), ret);
                gpiod_set_value_cansleep((*cs35l36).reset_gpio, 0);
                regulator_bulk_disable((*cs35l36).num_supplies, (*cs35l36).supplies.as_mut_ptr());
                return ret;
            }
        }
        _ => {}
    }
    if (*pdata).vpbr_config.is_present { cs35l36_apply_vpbr_config(cs35l36); }
    let irq_d = irq_get_irq_data((*i2c_client).irq as c_uint);
    if irq_d.is_null() {
        dev_err(&mut (*i2c_client).dev, bstr!("Invalid IRQ: %d\n"), (*i2c_client).irq);
        ret = -(ENODEV as c_int);
        gpiod_set_value_cansleep((*cs35l36).reset_gpio, 0);
        regulator_bulk_disable((*cs35l36).num_supplies, (*cs35l36).supplies.as_mut_ptr());
        return ret;
    }
    let irq_pol = irqd_get_trigger_type(irq_d);
    let chip_irq_pol = match irq_pol as c_uint {
        IRQF_TRIGGER_FALLING | IRQF_TRIGGER_LOW => 0,
        IRQF_TRIGGER_RISING | IRQF_TRIGGER_HIGH => 1,
        _ => {
            dev_err((*cs35l36).dev, bstr!("Invalid IRQ polarity: %d\n"), irq_pol);
            gpiod_set_value_cansleep((*cs35l36).reset_gpio, 0);
            regulator_bulk_disable((*cs35l36).num_supplies, (*cs35l36).supplies.as_mut_ptr());
            return -(EINVAL as c_int);
        }
    };
    regmap_update_bits((*cs35l36).regmap, CS35L36_PAD_INTERFACE, CS35L36_INT_POL_SEL_MASK, chip_irq_pol << CS35L36_INT_POL_SEL_SHIFT);
    ret = devm_request_threaded_irq(dev, (*i2c_client).irq as c_uint, ptr::null(), cs35l36_irq, IRQF_ONESHOT | irq_pol as c_uint, bstr!("cs35l36"), cs35l36 as *mut c_void);
    if ret != 0 {
        dev_err(dev, bstr!("Failed to request IRQ: %d\n"), ret);
        gpiod_set_value_cansleep((*cs35l36).reset_gpio, 0);
        regulator_bulk_disable((*cs35l36).num_supplies, (*cs35l36).supplies.as_mut_ptr());
        return ret;
    }
    regmap_update_bits((*cs35l36).regmap, CS35L36_PAD_INTERFACE, CS35L36_INT_OUTPUT_EN_MASK, 1);
    /* Set interrupt masks for critical errors */
    regmap_write((*cs35l36).regmap, CS35L36_INT1_MASK, CS35L36_INT1_MASK_DEFAULT);
    regmap_write((*cs35l36).regmap, CS35L36_INT3_MASK, CS35L36_INT3_MASK_DEFAULT);
    dev_info(&mut (*i2c_client).dev, bstr!("Cirrus Logic CS35L%d, Revision: %02X\n"), (*cs35l36).chip_version, reg_revid >> 8);
    ret = devm_snd_soc_register_component(dev, soc_component_dev_cs35l36, cs35l36_dai, 1);
    if ret < 0 {
        dev_err(dev, bstr!("%s: Register component failed %d\n"), bstr!("cs35l36_i2c_probe"), ret);
        gpiod_set_value_cansleep((*cs35l36).reset_gpio, 0);
        regulator_bulk_disable((*cs35l36).num_supplies, (*cs35l36).supplies.as_mut_ptr());
        return ret;
    }
    0
}

unsafe extern "C" fn cs35l36_i2c_remove(client: *mut i2c_client) {
    let cs35l36 = i2c_get_clientdata(client) as *mut cs35l36_private;
    /* Reset interrupt masks for device removal */
    regmap_write((*cs35l36).regmap, CS35L36_INT1_MASK, CS35L36_INT1_MASK_RESET);
    regmap_write((*cs35l36).regmap, CS35L36_INT3_MASK, CS35L36_INT3_MASK_RESET);
    if !(*cs35l36).reset_gpio.is_null() {
        gpiod_set_value_cansleep((*cs35l36).reset_gpio, 0);
    }
    regulator_bulk_disable((*cs35l36).num_supplies, (*cs35l36).supplies.as_mut_ptr());
}

/*
 * The original C file declares ALSA controls/widgets/routes, snd_soc_dai_ops,
 * snd_soc_dai_driver, snd_soc_component_driver, regmap_config, OF/I2C match
 * tables, i2c_driver, module_i2c_driver(cs35l36_i2c_driver), and MODULE_*
 * metadata via kernel C macros:
 *
 *   - TLV/SOC control macros for Digital PCM Volume, Analog PCM Volume,
 *     PCM Soft Ramp, Amp Gain Zero-Cross, PDM LDM ramps, and LDM Select.
 *   - DAPM widgets/routes for Channel Mux, SDIN, Main AMP, SPK, AMP Enable,
 *     CLASS H, BOOST Enable, ASPTX1-6, monitor ADCs, VP/VBST/VSENSE.
 *   - DAI "cs35l36-pcm" with playback/capture "AMP Playback"/"AMP Capture",
 *     CS35L36_RATES, CS35L36_RX_FORMATS, CS35L36_TX_FORMATS, symmetric_rate.
 *   - regmap_config with 32-bit registers/values, stride 4, maple cache, max
 *     register CS35L36_PAC_PMEM_WORD1023, defaults cs35l36_reg, and callbacks
 *     cs35l36_readable_reg/cs35l36_precious_reg/cs35l36_volatile_reg.
 *   - of_device_id compatible "cirrus,cs35l36" and i2c_device_id "cs35l36".
 *   - MODULE_DESCRIPTION("ASoC CS35L36 driver"), MODULE_AUTHOR(...),
 *     MODULE_LICENSE("GPL").
 *
 * These macro-expanded data layouts are dependency-provided in the target Rust
 * environment; the translated callbacks and behavioral code above preserve the
 * file-local behavior and externally visible function ordering.
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
