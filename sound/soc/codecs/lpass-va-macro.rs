// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2018-2020, The Linux Foundation. All rights reserved.

// Rust translation of soc/codecs/lpass-va-macro.c.
// C include dependencies intentionally remain external to this isolated file:
// linux/bitfield.h, linux/clk.h, linux/clk-provider.h, linux/init.h,
// linux/io.h, linux/module.h, linux/of_clk.h, linux/of_platform.h,
// linux/platform_device.h, linux/pm_clock.h, linux/pm_runtime.h,
// linux/regmap.h, linux/regulator/consumer.h, sound/soc.h,
// sound/soc-dapm.h, sound/tlv.h, and lpass-macro-common.h.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

type bool_ = bool;
type u8_ = u8;
type u16_ = u16;
type u32_ = u32;
type s32_ = i32;

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

const fn GENMASK(h: u32, l: u32) -> u32 {
    ((!0u32) << l) & ((!0u32) >> (31 - h))
}

fn field_get(mask: u32, reg: u32) -> u32 {
    (reg & mask) >> mask.trailing_zeros()
}

/* VA macro registers */
const CDC_VA_CLK_RST_CTRL_MCLK_CONTROL: u32 = 0x0000;
const CDC_VA_MCLK_CONTROL_EN: u32 = BIT(0);
const CDC_VA_CLK_RST_CTRL_FS_CNT_CONTROL: u32 = 0x0004;
const CDC_VA_FS_CONTROL_EN: u32 = BIT(0);
const CDC_VA_FS_COUNTER_CLR: u32 = BIT(1);
const CDC_VA_CLK_RST_CTRL_SWR_CONTROL: u32 = 0x0008;
const CDC_VA_SWR_RESET_MASK: u32 = BIT(1);
const CDC_VA_SWR_RESET_ENABLE: u32 = BIT(1);
const CDC_VA_SWR_CLK_EN_MASK: u32 = BIT(0);
const CDC_VA_SWR_CLK_ENABLE: u32 = BIT(0);
const CDC_VA_TOP_CSR_TOP_CFG0: u32 = 0x0080;
const CDC_VA_FS_BROADCAST_EN: u32 = BIT(1);
const CDC_VA_TOP_CSR_DMIC0_CTL: u32 = 0x0084;
const CDC_VA_TOP_CSR_DMIC1_CTL: u32 = 0x0088;
const CDC_VA_TOP_CSR_DMIC2_CTL: u32 = 0x008c;
const CDC_VA_TOP_CSR_DMIC3_CTL: u32 = 0x0090;
const CDC_VA_DMIC_EN_MASK: u32 = BIT(0);
const CDC_VA_DMIC_ENABLE: u32 = BIT(0);
const CDC_VA_DMIC_CLK_SEL_MASK: u32 = GENMASK(3, 1);
const CDC_VA_DMIC_CLK_SEL_SHFT: u32 = 1;
const CDC_VA_DMIC_CLK_SEL_DIV0: u32 = 0x0;
const CDC_VA_DMIC_CLK_SEL_DIV1: u32 = 0x2;
const CDC_VA_DMIC_CLK_SEL_DIV2: u32 = 0x4;
const CDC_VA_DMIC_CLK_SEL_DIV3: u32 = 0x6;
const CDC_VA_DMIC_CLK_SEL_DIV4: u32 = 0x8;
const CDC_VA_DMIC_CLK_SEL_DIV5: u32 = 0xa;
const CDC_VA_TOP_CSR_DMIC_CFG: u32 = 0x0094;
const CDC_VA_RESET_ALL_DMICS_MASK: u32 = BIT(7);
const CDC_VA_RESET_ALL_DMICS_RESET: u32 = BIT(7);
const CDC_VA_RESET_ALL_DMICS_DISABLE: u32 = 0;
const CDC_VA_DMIC3_FREQ_CHANGE_MASK: u32 = BIT(3);
const CDC_VA_DMIC3_FREQ_CHANGE_EN: u32 = BIT(3);
const CDC_VA_DMIC2_FREQ_CHANGE_MASK: u32 = BIT(2);
const CDC_VA_DMIC2_FREQ_CHANGE_EN: u32 = BIT(2);
const CDC_VA_DMIC1_FREQ_CHANGE_MASK: u32 = BIT(1);
const CDC_VA_DMIC1_FREQ_CHANGE_EN: u32 = BIT(1);
const CDC_VA_DMIC0_FREQ_CHANGE_MASK: u32 = BIT(0);
const CDC_VA_DMIC0_FREQ_CHANGE_EN: u32 = BIT(0);
const CDC_VA_DMIC_FREQ_CHANGE_DISABLE: u32 = 0;
const CDC_VA_TOP_CSR_DEBUG_BUS: u32 = 0x009c;
const CDC_VA_TOP_CSR_DEBUG_EN: u32 = 0x00a0;
const CDC_VA_TOP_CSR_TX_I2S_CTL: u32 = 0x00a4;
const CDC_VA_TOP_CSR_I2S_CLK: u32 = 0x00a8;
const CDC_VA_TOP_CSR_I2S_RESET: u32 = 0x00ac;
const CDC_VA_TOP_CSR_CORE_ID_0: u32 = 0x00c0;
const CORE_ID_0_REV_MAJ: u32 = GENMASK(7, 0);
const CDC_VA_TOP_CSR_CORE_ID_1: u32 = 0x00c4;
const CORE_ID_1_HAS_WSAMACRO: u32 = BIT(0);
const CORE_ID_1_HAS_RXMACRO: u32 = BIT(1);
const CORE_ID_1_HAS_TXMACRO: u32 = BIT(2);
const CORE_ID_1_HAS_VAMACRO: u32 = BIT(3);
const CDC_VA_TOP_CSR_CORE_ID_2: u32 = 0x00c8;
const CORE_ID_2_REV_MIN: u32 = GENMASK(7, 4);
const CORE_ID_2_REV_STEP: u32 = GENMASK(3, 0);
const CDC_VA_TOP_CSR_CORE_ID_3: u32 = 0x00cc;
const CDC_VA_TOP_CSR_SWR_MIC_CTL0: u32 = 0x00d0;
const CDC_VA_TOP_CSR_SWR_MIC_CTL1: u32 = 0x00d4;
const CDC_VA_TOP_CSR_SWR_MIC_CTL2: u32 = 0x00d8;
const CDC_VA_SWR_MIC_CLK_SEL_0_1_MASK: u32 = 0xee;
const CDC_VA_SWR_MIC_CLK_SEL_0_1_DIV1: u32 = 0xcc;
const CDC_VA_TOP_CSR_SWR_CTRL: u32 = 0x00dc;
const CDC_VA_INP_MUX_ADC_MUX0_CFG0: u32 = 0x0100;
const CDC_VA_INP_MUX_ADC_MUX0_CFG1: u32 = 0x0104;
const CDC_VA_INP_MUX_ADC_MUX1_CFG0: u32 = 0x0108;
const CDC_VA_INP_MUX_ADC_MUX1_CFG1: u32 = 0x010c;
const CDC_VA_INP_MUX_ADC_MUX2_CFG0: u32 = 0x0110;
const CDC_VA_INP_MUX_ADC_MUX2_CFG1: u32 = 0x0114;
const CDC_VA_INP_MUX_ADC_MUX3_CFG0: u32 = 0x0118;
const CDC_VA_INP_MUX_ADC_MUX3_CFG1: u32 = 0x011c;
const CDC_VA_TX0_TX_PATH_CTL: u32 = 0x0400;
const CDC_VA_TX_PATH_CLK_EN_MASK: u32 = BIT(5);
const CDC_VA_TX_PATH_CLK_EN: u32 = BIT(5);
const CDC_VA_TX_PATH_CLK_DISABLE: u32 = 0;
const CDC_VA_TX_PATH_PGA_MUTE_EN_MASK: u32 = BIT(4);
const CDC_VA_TX_PATH_PGA_MUTE_EN: u32 = BIT(4);
const CDC_VA_TX_PATH_PGA_MUTE_DISABLE: u32 = 0;
const CDC_VA_TX0_TX_PATH_CFG0: u32 = 0x0404;
const CDC_VA_ADC_MODE_MASK: u32 = GENMASK(2, 1);
const CDC_VA_ADC_MODE_SHIFT: u32 = 1;
const TX_HPF_CUT_OFF_FREQ_MASK: u32 = GENMASK(6, 5);
const CF_MIN_3DB_4HZ: u32 = 0x0;
const CF_MIN_3DB_75HZ: u32 = 0x1;
const CF_MIN_3DB_150HZ: u32 = 0x2;
const CDC_VA_TX0_TX_PATH_CFG1: u32 = 0x0408;
const CDC_VA_TX0_TX_VOL_CTL: u32 = 0x040c;
const CDC_VA_TX0_TX_PATH_SEC0: u32 = 0x0410;
const CDC_VA_TX0_TX_PATH_SEC1: u32 = 0x0414;
const CDC_VA_TX0_TX_PATH_SEC2: u32 = 0x0418;
const CDC_VA_TX_HPF_CUTOFF_FREQ_CHANGE_MASK: u32 = BIT(1);
const CDC_VA_TX_HPF_CUTOFF_FREQ_CHANGE_REQ: u32 = BIT(1);
const CDC_VA_TX_HPF_ZERO_GATE_MASK: u32 = BIT(0);
const CDC_VA_TX_HPF_ZERO_NO_GATE: u32 = BIT(0);
const CDC_VA_TX_HPF_ZERO_GATE: u32 = 0;
const CDC_VA_TX0_TX_PATH_SEC3: u32 = 0x041c;
const CDC_VA_TX0_TX_PATH_SEC4: u32 = 0x0420;
const CDC_VA_TX0_TX_PATH_SEC5: u32 = 0x0424;
const CDC_VA_TX0_TX_PATH_SEC6: u32 = 0x0428;
const CDC_VA_TX0_TX_PATH_SEC7: u32 = 0x042c;
const CDC_VA_TX1_TX_PATH_CTL: u32 = 0x0480;
const CDC_VA_TX1_TX_PATH_CFG0: u32 = 0x0484;
const CDC_VA_TX1_TX_PATH_CFG1: u32 = 0x0488;
const CDC_VA_TX1_TX_VOL_CTL: u32 = 0x048c;
const CDC_VA_TX1_TX_PATH_SEC0: u32 = 0x0490;
const CDC_VA_TX1_TX_PATH_SEC1: u32 = 0x0494;
const CDC_VA_TX1_TX_PATH_SEC2: u32 = 0x0498;
const CDC_VA_TX1_TX_PATH_SEC3: u32 = 0x049c;
const CDC_VA_TX1_TX_PATH_SEC4: u32 = 0x04a0;
const CDC_VA_TX1_TX_PATH_SEC5: u32 = 0x04a4;
const CDC_VA_TX1_TX_PATH_SEC6: u32 = 0x04a8;
const CDC_VA_TX2_TX_PATH_CTL: u32 = 0x0500;
const CDC_VA_TX2_TX_PATH_CFG0: u32 = 0x0504;
const CDC_VA_TX2_TX_PATH_CFG1: u32 = 0x0508;
const CDC_VA_TX2_TX_VOL_CTL: u32 = 0x050c;
const CDC_VA_TX2_TX_PATH_SEC0: u32 = 0x0510;
const CDC_VA_TX2_TX_PATH_SEC1: u32 = 0x0514;
const CDC_VA_TX2_TX_PATH_SEC2: u32 = 0x0518;
const CDC_VA_TX2_TX_PATH_SEC3: u32 = 0x051c;
const CDC_VA_TX2_TX_PATH_SEC4: u32 = 0x0520;
const CDC_VA_TX2_TX_PATH_SEC5: u32 = 0x0524;
const CDC_VA_TX2_TX_PATH_SEC6: u32 = 0x0528;
const CDC_VA_TX3_TX_PATH_CTL: u32 = 0x0580;
const CDC_VA_TX3_TX_PATH_CFG0: u32 = 0x0584;
const CDC_VA_TX_PATH_ADC_DMIC_SEL_MASK: u32 = BIT(7);
const CDC_VA_TX_PATH_ADC_DMIC_SEL_DMIC: u32 = BIT(7);
const CDC_VA_TX_PATH_ADC_DMIC_SEL_ADC: u32 = 0;
const CDC_VA_TX3_TX_PATH_CFG1: u32 = 0x0588;
const CDC_VA_TX3_TX_VOL_CTL: u32 = 0x058c;
const CDC_VA_TX3_TX_PATH_SEC0: u32 = 0x0590;
const CDC_VA_TX3_TX_PATH_SEC1: u32 = 0x0594;
const CDC_VA_TX3_TX_PATH_SEC2: u32 = 0x0598;
const CDC_VA_TX3_TX_PATH_SEC3: u32 = 0x059c;
const CDC_VA_TX3_TX_PATH_SEC4: u32 = 0x05a0;
const CDC_VA_TX3_TX_PATH_SEC5: u32 = 0x05a4;
const CDC_VA_TX3_TX_PATH_SEC6: u32 = 0x05a8;

const VA_MAX_OFFSET: u32 = 0x07a8;
const VA_MACRO_NUM_DECIMATORS: usize = 4;
const VA_MACRO_RATES: u32 = SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000;
const VA_MACRO_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S24_3LE;
const VA_MACRO_MCLK_FREQ: u32 = 9600000;
const VA_MACRO_TX_PATH_OFFSET: u32 = 0x80;
const VA_MACRO_SWR_MIC_MUX_SEL_MASK: u32 = 0xf;
const VA_MACRO_ADC_MUX_CFG_OFFSET: u32 = 0x8;
const VA_NUM_CLKS_MAX: usize = 3;

// DECLARE_TLV_DB_SCALE(digital_gain, -8400, 100, -8400)
static digital_gain: [u32; 4] = [0, (-8400i32) as u32, 100, (-8400i32) as u32];

const VA_MACRO_AIF1_CAP: usize = 0;
const VA_MACRO_AIF2_CAP: usize = 1;
const VA_MACRO_AIF3_CAP: usize = 2;
const VA_MACRO_MAX_DAIS: usize = 3;

const VA_MACRO_DEC0: usize = 0;
const VA_MACRO_DEC1: usize = 1;
const VA_MACRO_DEC2: usize = 2;
const VA_MACRO_DEC3: usize = 3;
const VA_MACRO_DEC4: usize = 4;
const VA_MACRO_DEC5: usize = 5;
const VA_MACRO_DEC6: usize = 6;
const VA_MACRO_DEC7: usize = 7;
const VA_MACRO_DEC_MAX: usize = 8;

const VA_MACRO_CLK_DIV_2: u16 = 0;
const VA_MACRO_CLK_DIV_3: u16 = 1;
const VA_MACRO_CLK_DIV_4: u16 = 2;
const VA_MACRO_CLK_DIV_6: u16 = 3;
const VA_MACRO_CLK_DIV_8: u16 = 4;
const VA_MACRO_CLK_DIV_16: u16 = 5;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct clk_hw { pub init: *const clk_init_data }
#[repr(C)] pub struct lpass_macro { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { pub private_value: c_ulong }
#[repr(C)] pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context, pub shift: c_uint }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device }
#[repr(C)] pub struct soc_enum { pub reg: c_uint, pub shift_l: c_uint }
#[repr(C)] pub struct soc_mixer_control { pub shift: c_uint }
#[repr(C)] pub struct snd_soc_dapm_update { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { pub id: c_int, pub component: *mut snd_soc_component }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct snd_soc_dai_ops { pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>, pub get_channel_map: Option<unsafe extern "C" fn(*const snd_soc_dai, *mut c_uint, *mut c_uint, *mut c_uint, *mut c_uint) -> c_int>, pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int> }
#[repr(C)] pub struct snd_soc_pcm_stream { pub stream_name: *const c_char, pub rates: u32, pub formats: u64, pub rate_max: c_uint, pub rate_min: c_uint, pub channels_min: c_uint, pub channels_max: c_uint }
#[repr(C)] pub struct snd_soc_dai_driver { pub name: *const c_char, pub id: c_int, pub capture: snd_soc_pcm_stream, pub ops: *const snd_soc_dai_ops }
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget_desc { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route { pub sink: *const c_char, pub control: *const c_char, pub source: *const c_char }
#[repr(C)] pub struct snd_soc_component_driver { pub name: *const c_char, pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>, pub controls: *const snd_kcontrol_new, pub num_controls: c_uint, pub dapm_widgets: *const snd_soc_dapm_widget_desc, pub num_dapm_widgets: c_uint, pub dapm_routes: *const snd_soc_dapm_route, pub num_dapm_routes: c_uint }
#[repr(C)] pub struct clk_ops { pub prepare: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>, pub unprepare: Option<unsafe extern "C" fn(*mut clk_hw)>, pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int> }
#[repr(C)] pub struct clk_init_data { pub name: *const c_char, pub ops: *const clk_ops, pub flags: c_uint, pub parent_names: *const *const c_char, pub num_parents: c_uint }
#[repr(C)] pub struct dev_pm_ops { _private: [u8; 0] }
#[repr(C)] pub struct of_device_id { pub compatible: *const c_char, pub data: *const c_void }
#[repr(C)] pub struct platform_driver { _private: [u8; 0] }
#[repr(C)] pub struct reg_default { pub reg: c_uint, pub def: c_uint }
#[repr(C)] pub struct regmap_config { pub name: *const c_char, pub reg_bits: c_uint, pub val_bits: c_uint, pub reg_stride: c_uint, pub cache_type: c_uint, pub reg_defaults: *const reg_default, pub num_reg_defaults: c_uint, pub max_register: c_uint, pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>, pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>, pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool> }

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
}
#[repr(C)] pub struct snd_ctl_elem_value_integer { pub value: [i64; 128] }
#[repr(C)] pub struct snd_ctl_elem_value_enumerated { pub item: [c_uint; 128] }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }

#[repr(C)]
struct va_macro {
    dev: *mut device,
    active_ch_mask: [c_ulong; VA_MACRO_MAX_DAIS],
    active_ch_cnt: [c_ulong; VA_MACRO_MAX_DAIS],
    dmic_clk_div: u16_,
    has_swr_master: bool_,
    has_npl_clk: bool_,
    dec_mode: [c_int; VA_MACRO_NUM_DECIMATORS],
    regmap: *mut regmap,
    mclk: *mut clk,
    npl: *mut clk,
    macro_: *mut clk,
    dcodec: *mut clk,
    fsgen: *mut clk,
    hw: clk_hw,
    pds: *mut lpass_macro,
    dmic_0_1_clk_cnt: s32_,
    dmic_2_3_clk_cnt: s32_,
    dmic_4_5_clk_cnt: s32_,
    dmic_6_7_clk_cnt: s32_,
    dmic_0_1_clk_div: u8_,
    dmic_2_3_clk_div: u8_,
    dmic_4_5_clk_div: u8_,
    dmic_6_7_clk_div: u8_,
}

unsafe fn to_va_macro(_hw: *mut clk_hw) -> *mut va_macro {
    (_hw as *mut u8).sub(core::mem::offset_of!(va_macro, hw)) as *mut va_macro
}

#[repr(C)]
struct va_macro_data {
    has_swr_master: bool_,
    has_npl_clk: bool_,
    version: c_int,
}

static sm8250_va_data: va_macro_data = va_macro_data { has_swr_master: false, has_npl_clk: false, version: LPASS_CODEC_VERSION_1_0 };
static sc7280_va_data: va_macro_data = va_macro_data { has_swr_master: false, has_npl_clk: false, version: 0 };
static sm8450_va_data: va_macro_data = va_macro_data { has_swr_master: true, has_npl_clk: true, version: 0 };
static sm8550_va_data: va_macro_data = va_macro_data { has_swr_master: true, has_npl_clk: false, version: 0 };

unsafe extern "C" fn va_is_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        CDC_VA_TOP_CSR_CORE_ID_0 | CDC_VA_TOP_CSR_CORE_ID_1 | CDC_VA_TOP_CSR_CORE_ID_2 | CDC_VA_TOP_CSR_CORE_ID_3 |
        CDC_VA_TOP_CSR_DMIC0_CTL | CDC_VA_TOP_CSR_DMIC1_CTL | CDC_VA_TOP_CSR_DMIC2_CTL | CDC_VA_TOP_CSR_DMIC3_CTL => true,
        _ => false,
    }
}

static va_defaults: [reg_default; 70] = [
    reg_default { reg: CDC_VA_CLK_RST_CTRL_MCLK_CONTROL, def: 0x00 }, reg_default { reg: CDC_VA_CLK_RST_CTRL_FS_CNT_CONTROL, def: 0x00 }, reg_default { reg: CDC_VA_CLK_RST_CTRL_SWR_CONTROL, def: 0x00 }, reg_default { reg: CDC_VA_TOP_CSR_TOP_CFG0, def: 0x00 },
    reg_default { reg: CDC_VA_TOP_CSR_DMIC0_CTL, def: 0x00 }, reg_default { reg: CDC_VA_TOP_CSR_DMIC1_CTL, def: 0x00 }, reg_default { reg: CDC_VA_TOP_CSR_DMIC2_CTL, def: 0x00 }, reg_default { reg: CDC_VA_TOP_CSR_DMIC3_CTL, def: 0x00 },
    reg_default { reg: CDC_VA_TOP_CSR_DMIC_CFG, def: 0x80 }, reg_default { reg: CDC_VA_TOP_CSR_DEBUG_BUS, def: 0x00 }, reg_default { reg: CDC_VA_TOP_CSR_DEBUG_EN, def: 0x00 }, reg_default { reg: CDC_VA_TOP_CSR_TX_I2S_CTL, def: 0x0c },
    reg_default { reg: CDC_VA_TOP_CSR_I2S_CLK, def: 0x00 }, reg_default { reg: CDC_VA_TOP_CSR_I2S_RESET, def: 0x00 }, reg_default { reg: CDC_VA_TOP_CSR_CORE_ID_0, def: 0x00 }, reg_default { reg: CDC_VA_TOP_CSR_CORE_ID_1, def: 0x00 },
    reg_default { reg: CDC_VA_TOP_CSR_CORE_ID_2, def: 0x00 }, reg_default { reg: CDC_VA_TOP_CSR_CORE_ID_3, def: 0x00 }, reg_default { reg: CDC_VA_TOP_CSR_SWR_MIC_CTL0, def: 0xee }, reg_default { reg: CDC_VA_TOP_CSR_SWR_MIC_CTL1, def: 0xee },
    reg_default { reg: CDC_VA_TOP_CSR_SWR_MIC_CTL2, def: 0xee }, reg_default { reg: CDC_VA_TOP_CSR_SWR_CTRL, def: 0x06 },
    reg_default { reg: CDC_VA_INP_MUX_ADC_MUX0_CFG0, def: 0x00 }, reg_default { reg: CDC_VA_INP_MUX_ADC_MUX0_CFG1, def: 0x00 }, reg_default { reg: CDC_VA_INP_MUX_ADC_MUX1_CFG0, def: 0x00 }, reg_default { reg: CDC_VA_INP_MUX_ADC_MUX1_CFG1, def: 0x00 },
    reg_default { reg: CDC_VA_INP_MUX_ADC_MUX2_CFG0, def: 0x00 }, reg_default { reg: CDC_VA_INP_MUX_ADC_MUX2_CFG1, def: 0x00 }, reg_default { reg: CDC_VA_INP_MUX_ADC_MUX3_CFG0, def: 0x00 }, reg_default { reg: CDC_VA_INP_MUX_ADC_MUX3_CFG1, def: 0x00 },
    reg_default { reg: CDC_VA_TX0_TX_PATH_CTL, def: 0x04 }, reg_default { reg: CDC_VA_TX0_TX_PATH_CFG0, def: 0x10 }, reg_default { reg: CDC_VA_TX0_TX_PATH_CFG1, def: 0x0b }, reg_default { reg: CDC_VA_TX0_TX_VOL_CTL, def: 0x00 },
    reg_default { reg: CDC_VA_TX0_TX_PATH_SEC0, def: 0x00 }, reg_default { reg: CDC_VA_TX0_TX_PATH_SEC1, def: 0x00 }, reg_default { reg: CDC_VA_TX0_TX_PATH_SEC2, def: 0x01 }, reg_default { reg: CDC_VA_TX0_TX_PATH_SEC3, def: 0x3c },
    reg_default { reg: CDC_VA_TX0_TX_PATH_SEC4, def: 0x20 }, reg_default { reg: CDC_VA_TX0_TX_PATH_SEC5, def: 0x00 }, reg_default { reg: CDC_VA_TX0_TX_PATH_SEC6, def: 0x00 }, reg_default { reg: CDC_VA_TX0_TX_PATH_SEC7, def: 0x25 },
    reg_default { reg: CDC_VA_TX1_TX_PATH_CTL, def: 0x04 }, reg_default { reg: CDC_VA_TX1_TX_PATH_CFG0, def: 0x10 }, reg_default { reg: CDC_VA_TX1_TX_PATH_CFG1, def: 0x0b }, reg_default { reg: CDC_VA_TX1_TX_VOL_CTL, def: 0x00 },
    reg_default { reg: CDC_VA_TX1_TX_PATH_SEC0, def: 0x00 }, reg_default { reg: CDC_VA_TX1_TX_PATH_SEC1, def: 0x00 }, reg_default { reg: CDC_VA_TX1_TX_PATH_SEC2, def: 0x01 }, reg_default { reg: CDC_VA_TX1_TX_PATH_SEC3, def: 0x3c },
    reg_default { reg: CDC_VA_TX1_TX_PATH_SEC4, def: 0x20 }, reg_default { reg: CDC_VA_TX1_TX_PATH_SEC5, def: 0x00 }, reg_default { reg: CDC_VA_TX1_TX_PATH_SEC6, def: 0x00 },
    reg_default { reg: CDC_VA_TX2_TX_PATH_CTL, def: 0x04 }, reg_default { reg: CDC_VA_TX2_TX_PATH_CFG0, def: 0x10 }, reg_default { reg: CDC_VA_TX2_TX_PATH_CFG1, def: 0x0b }, reg_default { reg: CDC_VA_TX2_TX_VOL_CTL, def: 0x00 },
    reg_default { reg: CDC_VA_TX2_TX_PATH_SEC0, def: 0x00 }, reg_default { reg: CDC_VA_TX2_TX_PATH_SEC1, def: 0x00 }, reg_default { reg: CDC_VA_TX2_TX_PATH_SEC2, def: 0x01 }, reg_default { reg: CDC_VA_TX2_TX_PATH_SEC3, def: 0x3c },
    reg_default { reg: CDC_VA_TX2_TX_PATH_SEC4, def: 0x20 }, reg_default { reg: CDC_VA_TX2_TX_PATH_SEC5, def: 0x00 }, reg_default { reg: CDC_VA_TX2_TX_PATH_SEC6, def: 0x00 },
    reg_default { reg: CDC_VA_TX3_TX_PATH_CTL, def: 0x04 }, reg_default { reg: CDC_VA_TX3_TX_PATH_CFG0, def: 0x10 }, reg_default { reg: CDC_VA_TX3_TX_PATH_CFG1, def: 0x0b }, reg_default { reg: CDC_VA_TX3_TX_VOL_CTL, def: 0x00 },
    reg_default { reg: CDC_VA_TX3_TX_PATH_SEC0, def: 0x00 }, reg_default { reg: CDC_VA_TX3_TX_PATH_SEC1, def: 0x00 }, reg_default { reg: CDC_VA_TX3_TX_PATH_SEC2, def: 0x01 }, reg_default { reg: CDC_VA_TX3_TX_PATH_SEC3, def: 0x3c },
    reg_default { reg: CDC_VA_TX3_TX_PATH_SEC4, def: 0x20 }, reg_default { reg: CDC_VA_TX3_TX_PATH_SEC5, def: 0x00 }, reg_default { reg: CDC_VA_TX3_TX_PATH_SEC6, def: 0x00 },
];

unsafe extern "C" fn va_is_rw_register(_dev: *mut device, reg: c_uint) -> bool {
    matches!(reg,
        CDC_VA_CLK_RST_CTRL_MCLK_CONTROL | CDC_VA_CLK_RST_CTRL_FS_CNT_CONTROL | CDC_VA_CLK_RST_CTRL_SWR_CONTROL | CDC_VA_TOP_CSR_TOP_CFG0 |
        CDC_VA_TOP_CSR_DMIC0_CTL | CDC_VA_TOP_CSR_DMIC1_CTL | CDC_VA_TOP_CSR_DMIC2_CTL | CDC_VA_TOP_CSR_DMIC3_CTL | CDC_VA_TOP_CSR_DMIC_CFG |
        CDC_VA_TOP_CSR_SWR_MIC_CTL0 | CDC_VA_TOP_CSR_SWR_MIC_CTL1 | CDC_VA_TOP_CSR_SWR_MIC_CTL2 | CDC_VA_TOP_CSR_DEBUG_BUS | CDC_VA_TOP_CSR_DEBUG_EN |
        CDC_VA_TOP_CSR_TX_I2S_CTL | CDC_VA_TOP_CSR_I2S_CLK | CDC_VA_TOP_CSR_I2S_RESET | CDC_VA_INP_MUX_ADC_MUX0_CFG0 | CDC_VA_INP_MUX_ADC_MUX0_CFG1 |
        CDC_VA_INP_MUX_ADC_MUX1_CFG0 | CDC_VA_INP_MUX_ADC_MUX1_CFG1 | CDC_VA_INP_MUX_ADC_MUX2_CFG0 | CDC_VA_INP_MUX_ADC_MUX2_CFG1 |
        CDC_VA_INP_MUX_ADC_MUX3_CFG0 | CDC_VA_INP_MUX_ADC_MUX3_CFG1 | CDC_VA_TX0_TX_PATH_CTL | CDC_VA_TX0_TX_PATH_CFG0 | CDC_VA_TX0_TX_PATH_CFG1 |
        CDC_VA_TX0_TX_VOL_CTL | CDC_VA_TX0_TX_PATH_SEC0 | CDC_VA_TX0_TX_PATH_SEC1 | CDC_VA_TX0_TX_PATH_SEC2 | CDC_VA_TX0_TX_PATH_SEC3 |
        CDC_VA_TX0_TX_PATH_SEC4 | CDC_VA_TX0_TX_PATH_SEC5 | CDC_VA_TX0_TX_PATH_SEC6 | CDC_VA_TX0_TX_PATH_SEC7 | CDC_VA_TX1_TX_PATH_CTL |
        CDC_VA_TX1_TX_PATH_CFG0 | CDC_VA_TX1_TX_PATH_CFG1 | CDC_VA_TX1_TX_VOL_CTL | CDC_VA_TX1_TX_PATH_SEC0 | CDC_VA_TX1_TX_PATH_SEC1 |
        CDC_VA_TX1_TX_PATH_SEC2 | CDC_VA_TX1_TX_PATH_SEC3 | CDC_VA_TX1_TX_PATH_SEC4 | CDC_VA_TX1_TX_PATH_SEC5 | CDC_VA_TX1_TX_PATH_SEC6 |
        CDC_VA_TX2_TX_PATH_CTL | CDC_VA_TX2_TX_PATH_CFG0 | CDC_VA_TX2_TX_PATH_CFG1 | CDC_VA_TX2_TX_VOL_CTL | CDC_VA_TX2_TX_PATH_SEC0 |
        CDC_VA_TX2_TX_PATH_SEC1 | CDC_VA_TX2_TX_PATH_SEC2 | CDC_VA_TX2_TX_PATH_SEC3 | CDC_VA_TX2_TX_PATH_SEC4 | CDC_VA_TX2_TX_PATH_SEC5 |
        CDC_VA_TX2_TX_PATH_SEC6 | CDC_VA_TX3_TX_PATH_CTL | CDC_VA_TX3_TX_PATH_CFG0 | CDC_VA_TX3_TX_PATH_CFG1 | CDC_VA_TX3_TX_VOL_CTL |
        CDC_VA_TX3_TX_PATH_SEC0 | CDC_VA_TX3_TX_PATH_SEC1 | CDC_VA_TX3_TX_PATH_SEC2 | CDC_VA_TX3_TX_PATH_SEC3 | CDC_VA_TX3_TX_PATH_SEC4 |
        CDC_VA_TX3_TX_PATH_SEC5 | CDC_VA_TX3_TX_PATH_SEC6)
}

unsafe extern "C" fn va_is_readable_register(dev: *mut device, reg: c_uint) -> bool {
    match reg {
        CDC_VA_TOP_CSR_CORE_ID_0 | CDC_VA_TOP_CSR_CORE_ID_1 | CDC_VA_TOP_CSR_CORE_ID_2 | CDC_VA_TOP_CSR_CORE_ID_3 => true,
        _ => va_is_rw_register(dev, reg),
    }
}

static va_regmap_config: regmap_config = regmap_config {
    name: c"va_macro".as_ptr(),
    reg_bits: 32,
    val_bits: 32,
    reg_stride: 4,
    cache_type: REGCACHE_FLAT,
    reg_defaults: va_defaults.as_ptr(),
    num_reg_defaults: va_defaults.len() as c_uint,
    max_register: VA_MAX_OFFSET,
    volatile_reg: Some(va_is_volatile_register),
    readable_reg: Some(va_is_readable_register),
    writeable_reg: Some(va_is_rw_register),
};

unsafe extern "C" {
    static SNDRV_PCM_RATE_8000: u32; static SNDRV_PCM_RATE_16000: u32; static SNDRV_PCM_RATE_32000: u32; static SNDRV_PCM_RATE_48000: u32; static SNDRV_PCM_RATE_96000: u32; static SNDRV_PCM_RATE_192000: u32;
    static SNDRV_PCM_FMTBIT_S16_LE: u64; static SNDRV_PCM_FMTBIT_S24_LE: u64; static SNDRV_PCM_FMTBIT_S24_3LE: u64;
}

const SND_SOC_DAPM_PRE_PMU: c_int = 1 << 0;
const SND_SOC_DAPM_POST_PMU: c_int = 1 << 1;
const SND_SOC_DAPM_PRE_PMD: c_int = 1 << 2;
const SND_SOC_DAPM_POST_PMD: c_int = 1 << 3;
const SND_SOC_NOPM: c_int = -1;
const REGCACHE_FLAT: c_uint = 0;
const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const EOPNOTSUPP: c_int = 95;
const LPASS_CODEC_VERSION_UNKNOWN: c_int = 0;
const LPASS_CODEC_VERSION_1_0: c_int = 1;
const LPASS_CODEC_VERSION_2_0: c_int = 20;
const LPASS_CODEC_VERSION_2_5: c_int = 25;
const LPASS_CODEC_VERSION_2_6: c_int = 26;
const LPASS_CODEC_VERSION_2_7: c_int = 27;
const LPASS_CODEC_VERSION_2_8: c_int = 28;
const LPASS_CODEC_VERSION_2_9: c_int = 29;

unsafe extern "C" {
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync_region(map: *mut regmap, min: c_uint, max: c_uint) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn __clk_get_name(clk: *mut clk) -> *const c_char;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dapm_kcontrol_to_widget(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dapm_widget;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_dapm_put_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_dapm_get_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_dapm_mixer_update_power(dapm: *mut snd_soc_dapm_context, kcontrol: *mut snd_kcontrol, connect: c_int, update: *mut snd_soc_dapm_update) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn pm_runtime_put_sync_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn devm_pm_runtime_enable(dev: *mut device) -> c_int;
    fn devm_pm_clk_create(dev: *mut device) -> c_int;
    fn of_pm_clk_add_clks(dev: *mut device) -> c_int;
    fn pm_clk_suspend(dev: *mut device) -> c_int;
    fn pm_clk_resume(dev: *mut device) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> c_int;
    fn devm_clk_hw_get_clk(dev: *mut device, hw: *mut clk_hw, con_id: *const c_char) -> *mut clk;
    fn devm_of_clk_add_hw_provider(dev: *mut device, get: *mut c_void, data: *mut clk_hw) -> c_int;
    static mut of_clk_hw_simple_get: c_void;
    fn of_property_read_string(np: *mut device_node, propname: *const c_char, out_string: *mut *const c_char) -> c_int;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut u32) -> c_int;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_regmap_init_mmio(dev: *mut device, regs: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn snd_soc_component_init_regmap(component: *mut snd_soc_component, regmap: *mut regmap);
    fn lpass_macro_pds_init(dev: *mut device) -> *mut lpass_macro;
    fn lpass_macro_pds_exit(pds: *mut lpass_macro);
    fn lpass_macro_set_codec_version(version: c_int);
    fn lpass_macro_get_codec_version_string(version: c_int) -> *const c_char;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
}

unsafe fn IS_ERR<T>(ptr: *mut T) -> bool { (ptr as isize) < 0 && (ptr as isize) > -4096 }
unsafe fn PTR_ERR<T>(ptr: *mut T) -> c_int { ptr as isize as c_int }
unsafe fn test_bit(nr: u32, addr: *const c_ulong) -> bool { ((*addr) & (1usize << nr) as c_ulong) != 0 }
unsafe fn set_bit(nr: u32, addr: *mut c_ulong) { *addr |= (1usize << nr) as c_ulong; }
unsafe fn clear_bit(nr: u32, addr: *mut c_ulong) { *addr &= !((1usize << nr) as c_ulong); }

unsafe extern "C" fn va_clk_rsc_fs_gen_request(va: *mut va_macro, enable: bool) -> c_int {
    let regmap = (*va).regmap;
    if enable {
        regmap_update_bits(regmap, CDC_VA_CLK_RST_CTRL_MCLK_CONTROL, CDC_VA_MCLK_CONTROL_EN, CDC_VA_MCLK_CONTROL_EN);
        /* clear the fs counter */
        regmap_update_bits(regmap, CDC_VA_CLK_RST_CTRL_FS_CNT_CONTROL, CDC_VA_FS_CONTROL_EN | CDC_VA_FS_COUNTER_CLR, CDC_VA_FS_CONTROL_EN | CDC_VA_FS_COUNTER_CLR);
        regmap_update_bits(regmap, CDC_VA_CLK_RST_CTRL_FS_CNT_CONTROL, CDC_VA_FS_CONTROL_EN | CDC_VA_FS_COUNTER_CLR, CDC_VA_FS_CONTROL_EN);
        regmap_update_bits(regmap, CDC_VA_TOP_CSR_TOP_CFG0, CDC_VA_FS_BROADCAST_EN, CDC_VA_FS_BROADCAST_EN);
    } else {
        regmap_update_bits(regmap, CDC_VA_CLK_RST_CTRL_MCLK_CONTROL, CDC_VA_MCLK_CONTROL_EN, 0x0);
        regmap_update_bits(regmap, CDC_VA_CLK_RST_CTRL_FS_CNT_CONTROL, CDC_VA_FS_CONTROL_EN, 0x0);
        regmap_update_bits(regmap, CDC_VA_TOP_CSR_TOP_CFG0, CDC_VA_FS_BROADCAST_EN, 0x0);
    }
    0
}

unsafe extern "C" fn va_macro_mclk_enable(va: *mut va_macro, mclk_enable: bool) -> c_int {
    let regmap = (*va).regmap;
    if mclk_enable {
        va_clk_rsc_fs_gen_request(va, true);
        regcache_mark_dirty(regmap);
        regcache_sync_region(regmap, 0x0, VA_MAX_OFFSET);
    } else {
        va_clk_rsc_fs_gen_request(va, false);
    }
    0
}

unsafe extern "C" fn va_macro_mclk_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let comp = snd_soc_dapm_to_component((*w).dapm);
    let va = snd_soc_component_get_drvdata(comp) as *mut va_macro;
    match event {
        SND_SOC_DAPM_PRE_PMU => return clk_prepare_enable((*va).fsgen),
        SND_SOC_DAPM_POST_PMD => clk_disable_unprepare((*va).fsgen),
        _ => {}
    }
    0
}

unsafe extern "C" fn va_macro_put_dec_enum(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let widget = snd_soc_dapm_kcontrol_to_widget(kcontrol);
    let component = snd_soc_dapm_to_component((*widget).dapm);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let val = (*ucontrol).value.enumerated.item[0];
    let mic_sel_reg: u16 = match (*e).reg {
        CDC_VA_INP_MUX_ADC_MUX0_CFG0 => CDC_VA_TX0_TX_PATH_CFG0 as u16,
        CDC_VA_INP_MUX_ADC_MUX1_CFG0 => CDC_VA_TX1_TX_PATH_CFG0 as u16,
        CDC_VA_INP_MUX_ADC_MUX2_CFG0 => CDC_VA_TX2_TX_PATH_CFG0 as u16,
        CDC_VA_INP_MUX_ADC_MUX3_CFG0 => CDC_VA_TX3_TX_PATH_CFG0 as u16,
        _ => {
            dev_err((*component).dev, c"%s: e->reg: 0x%x not expected\n".as_ptr(), c"va_macro_put_dec_enum".as_ptr(), (*e).reg);
            return -EINVAL;
        }
    };
    if val != 0 {
        snd_soc_component_update_bits(component, mic_sel_reg as c_uint, CDC_VA_TX_PATH_ADC_DMIC_SEL_MASK, CDC_VA_TX_PATH_ADC_DMIC_SEL_DMIC);
    }
    snd_soc_dapm_put_enum_double(kcontrol, ucontrol)
}

unsafe extern "C" fn va_macro_tx_mixer_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let widget = snd_soc_dapm_kcontrol_to_widget(kcontrol);
    let component = snd_soc_dapm_to_component((*widget).dapm);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let dai_id = (*widget).shift as usize;
    let dec_id = (*mc).shift;
    let va = snd_soc_component_get_drvdata(component) as *mut va_macro;
    (*ucontrol).value.integer.value[0] = if test_bit(dec_id, &(*va).active_ch_mask[dai_id]) { 1 } else { 0 };
    0
}

unsafe extern "C" fn va_macro_tx_mixer_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let widget = snd_soc_dapm_kcontrol_to_widget(kcontrol);
    let component = snd_soc_dapm_to_component((*widget).dapm);
    let update: *mut snd_soc_dapm_update = ptr::null_mut();
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let dai_id = (*widget).shift as usize;
    let dec_id = (*mc).shift;
    let enable = (*ucontrol).value.integer.value[0] as c_uint;
    let va = snd_soc_component_get_drvdata(component) as *mut va_macro;
    if enable != 0 {
        set_bit(dec_id, &mut (*va).active_ch_mask[dai_id]);
        (*va).active_ch_cnt[dai_id] = (*va).active_ch_cnt[dai_id].wrapping_add(1);
    } else {
        clear_bit(dec_id, &mut (*va).active_ch_mask[dai_id]);
        (*va).active_ch_cnt[dai_id] = (*va).active_ch_cnt[dai_id].wrapping_sub(1);
    }
    snd_soc_dapm_mixer_update_power((*widget).dapm, kcontrol, enable as c_int, update);
    0
}

unsafe extern "C" fn va_dmic_clk_enable(component: *mut snd_soc_component, dmic: u32, enable: bool) -> c_int {
    let va = snd_soc_component_get_drvdata(component) as *mut va_macro;
    let (dmic_clk_cnt, dmic_clk_div, dmic_clk_reg, freq_change_mask): (*mut s32_, *mut u8_, u16, u8) = match dmic {
        0 | 1 => (&mut (*va).dmic_0_1_clk_cnt, &mut (*va).dmic_0_1_clk_div, CDC_VA_TOP_CSR_DMIC0_CTL as u16, CDC_VA_DMIC0_FREQ_CHANGE_MASK as u8),
        2 | 3 => (&mut (*va).dmic_2_3_clk_cnt, &mut (*va).dmic_2_3_clk_div, CDC_VA_TOP_CSR_DMIC1_CTL as u16, CDC_VA_DMIC1_FREQ_CHANGE_MASK as u8),
        4 | 5 => (&mut (*va).dmic_4_5_clk_cnt, &mut (*va).dmic_4_5_clk_div, CDC_VA_TOP_CSR_DMIC2_CTL as u16, CDC_VA_DMIC2_FREQ_CHANGE_MASK as u8),
        6 | 7 => (&mut (*va).dmic_6_7_clk_cnt, &mut (*va).dmic_6_7_clk_div, CDC_VA_TOP_CSR_DMIC3_CTL as u16, CDC_VA_DMIC3_FREQ_CHANGE_MASK as u8),
        _ => {
            dev_err((*component).dev, c"%s: Invalid DMIC Selection\n".as_ptr(), c"va_dmic_clk_enable".as_ptr());
            return -EINVAL;
        }
    };
    let mut clk_div: u8;
    if enable {
        clk_div = (*va).dmic_clk_div as u8;
        *dmic_clk_cnt += 1;
        if *dmic_clk_cnt == 1 {
            snd_soc_component_update_bits(component, CDC_VA_TOP_CSR_DMIC_CFG, CDC_VA_RESET_ALL_DMICS_MASK, CDC_VA_RESET_ALL_DMICS_DISABLE);
            snd_soc_component_update_bits(component, dmic_clk_reg as c_uint, CDC_VA_DMIC_CLK_SEL_MASK, (clk_div as u32) << CDC_VA_DMIC_CLK_SEL_SHFT);
            snd_soc_component_update_bits(component, dmic_clk_reg as c_uint, CDC_VA_DMIC_EN_MASK, CDC_VA_DMIC_ENABLE);
        } else if *dmic_clk_div > clk_div {
            snd_soc_component_update_bits(component, CDC_VA_TOP_CSR_DMIC_CFG, freq_change_mask as u32, freq_change_mask as u32);
            snd_soc_component_update_bits(component, dmic_clk_reg as c_uint, CDC_VA_DMIC_CLK_SEL_MASK, (clk_div as u32) << CDC_VA_DMIC_CLK_SEL_SHFT);
            snd_soc_component_update_bits(component, CDC_VA_TOP_CSR_DMIC_CFG, freq_change_mask as u32, CDC_VA_DMIC_FREQ_CHANGE_DISABLE);
        } else {
            clk_div = *dmic_clk_div;
        }
        *dmic_clk_div = clk_div;
    } else {
        *dmic_clk_cnt -= 1;
        if *dmic_clk_cnt == 0 {
            snd_soc_component_update_bits(component, dmic_clk_reg as c_uint, CDC_VA_DMIC_EN_MASK, 0);
            clk_div = 0;
            snd_soc_component_update_bits(component, dmic_clk_reg as c_uint, CDC_VA_DMIC_CLK_SEL_MASK, (clk_div as u32) << CDC_VA_DMIC_CLK_SEL_SHFT);
        } else {
            clk_div = (*va).dmic_clk_div as u8;
            if *dmic_clk_div > clk_div {
                clk_div = (*va).dmic_clk_div as u8;
                snd_soc_component_update_bits(component, CDC_VA_TOP_CSR_DMIC_CFG, freq_change_mask as u32, freq_change_mask as u32);
                snd_soc_component_update_bits(component, dmic_clk_reg as c_uint, CDC_VA_DMIC_CLK_SEL_MASK, (clk_div as u32) << CDC_VA_DMIC_CLK_SEL_SHFT);
                snd_soc_component_update_bits(component, CDC_VA_TOP_CSR_DMIC_CFG, freq_change_mask as u32, CDC_VA_DMIC_FREQ_CHANGE_DISABLE);
            } else {
                clk_div = *dmic_clk_div;
            }
        }
        *dmic_clk_div = clk_div;
    }
    0
}

unsafe extern "C" fn va_macro_enable_dmic(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let comp = snd_soc_dapm_to_component((*w).dapm);
    let dmic = (*w).shift;
    match event {
        SND_SOC_DAPM_PRE_PMU => { va_dmic_clk_enable(comp, dmic, true); }
        SND_SOC_DAPM_POST_PMD => { va_dmic_clk_enable(comp, dmic, false); }
        _ => {}
    }
    0
}

unsafe extern "C" fn va_macro_enable_dec(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let comp = snd_soc_dapm_to_component((*w).dapm);
    let decimator = (*w).shift as u32;
    let tx_vol_ctl_reg = CDC_VA_TX0_TX_PATH_CTL + VA_MACRO_TX_PATH_OFFSET * decimator;
    let hpf_gate_reg = CDC_VA_TX0_TX_PATH_SEC2 + VA_MACRO_TX_PATH_OFFSET * decimator;
    let dec_cfg_reg = CDC_VA_TX0_TX_PATH_CFG0 + VA_MACRO_TX_PATH_OFFSET * decimator;
    let tx_gain_ctl_reg = CDC_VA_TX0_TX_VOL_CTL + VA_MACRO_TX_PATH_OFFSET * decimator;
    let va = snd_soc_component_get_drvdata(comp) as *mut va_macro;
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            snd_soc_component_update_bits(comp, dec_cfg_reg, CDC_VA_ADC_MODE_MASK, ((*va).dec_mode[decimator as usize] as u32) << CDC_VA_ADC_MODE_SHIFT);
            /* Enable TX PGA Mute */
        }
        SND_SOC_DAPM_POST_PMU => {
            /* Enable TX CLK */
            snd_soc_component_update_bits(comp, tx_vol_ctl_reg, CDC_VA_TX_PATH_CLK_EN_MASK, CDC_VA_TX_PATH_CLK_EN);
            snd_soc_component_update_bits(comp, hpf_gate_reg, CDC_VA_TX_HPF_ZERO_GATE_MASK, CDC_VA_TX_HPF_ZERO_GATE);
            usleep_range(1000, 1010);
            let hpf_cut_off_freq = (snd_soc_component_read(comp, dec_cfg_reg) & TX_HPF_CUT_OFF_FREQ_MASK) >> 5;
            if hpf_cut_off_freq != CF_MIN_3DB_150HZ {
                snd_soc_component_update_bits(comp, dec_cfg_reg, TX_HPF_CUT_OFF_FREQ_MASK, CF_MIN_3DB_150HZ << 5);
                snd_soc_component_update_bits(comp, hpf_gate_reg, CDC_VA_TX_HPF_CUTOFF_FREQ_CHANGE_MASK, CDC_VA_TX_HPF_CUTOFF_FREQ_CHANGE_REQ);
                /*
                 * Minimum 1 clk cycle delay is required as per HW spec
                 */
                usleep_range(1000, 1010);
                snd_soc_component_update_bits(comp, hpf_gate_reg, CDC_VA_TX_HPF_CUTOFF_FREQ_CHANGE_MASK, 0x0);
            }
            usleep_range(1000, 1010);
            snd_soc_component_update_bits(comp, hpf_gate_reg, CDC_VA_TX_HPF_ZERO_GATE_MASK, CDC_VA_TX_HPF_ZERO_NO_GATE);
            /*
             * 6ms delay is required as per HW spec
             */
            usleep_range(6000, 6010);
            /* apply gain after decimator is enabled */
            snd_soc_component_write(comp, tx_gain_ctl_reg, snd_soc_component_read(comp, tx_gain_ctl_reg));
        }
        SND_SOC_DAPM_POST_PMD => {
            /* Disable TX CLK */
            snd_soc_component_update_bits(comp, tx_vol_ctl_reg, CDC_VA_TX_PATH_CLK_EN_MASK, CDC_VA_TX_PATH_CLK_DISABLE);
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn va_macro_dec_mode_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let va = snd_soc_component_get_drvdata(comp) as *mut va_macro;
    let e = (*kcontrol).private_value as *mut soc_enum;
    let path = (*e).shift_l as usize;
    (*ucontrol).value.enumerated.item[0] = (*va).dec_mode[path] as c_uint;
    0
}

unsafe extern "C" fn va_macro_dec_mode_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let value = (*ucontrol).value.enumerated.item[0] as c_int;
    let e = (*kcontrol).private_value as *mut soc_enum;
    let path = (*e).shift_l as usize;
    let va = snd_soc_component_get_drvdata(comp) as *mut va_macro;
    (*va).dec_mode[path] = value;
    0
}

unsafe extern "C" fn va_macro_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let va_dev = (*component).dev;
    let va = snd_soc_component_get_drvdata(component) as *mut va_macro;
    let sample_rate = params_rate(params);
    let tx_fs_rate: c_int = match sample_rate {
        8000 => 0,
        16000 => 1,
        32000 => 3,
        48000 => 4,
        96000 => 5,
        192000 => 6,
        384000 => 7,
        _ => {
            dev_err(va_dev, c"%s: Invalid TX sample rate: %d\n".as_ptr(), c"va_macro_hw_params".as_ptr(), params_rate(params));
            return -EINVAL;
        }
    };
    for decimator in 0..VA_MACRO_DEC_MAX {
        if test_bit(decimator as u32, &(*va).active_ch_mask[(*dai).id as usize]) {
            let tx_fs_reg = CDC_VA_TX0_TX_PATH_CTL + VA_MACRO_TX_PATH_OFFSET * decimator as u32;
            snd_soc_component_update_bits(component, tx_fs_reg, 0x0f, tx_fs_rate as u32);
        }
    }
    0
}

unsafe extern "C" fn va_macro_get_channel_map(dai: *const snd_soc_dai, tx_num: *mut c_uint, tx_slot: *mut c_uint, _rx_num: *mut c_uint, _rx_slot: *mut c_uint) -> c_int {
    let component = (*dai).component;
    let va_dev = (*component).dev;
    let va = snd_soc_component_get_drvdata(component) as *mut va_macro;
    match (*dai).id as usize {
        VA_MACRO_AIF1_CAP | VA_MACRO_AIF2_CAP | VA_MACRO_AIF3_CAP => {
            *tx_slot = (*va).active_ch_mask[(*dai).id as usize] as c_uint;
            *tx_num = (*va).active_ch_cnt[(*dai).id as usize] as c_uint;
        }
        _ => dev_err(va_dev, c"%s: Invalid AIF\n".as_ptr(), c"va_macro_get_channel_map".as_ptr()),
    }
    0
}

unsafe extern "C" fn va_macro_digital_mute(dai: *mut snd_soc_dai, mute: c_int, _stream: c_int) -> c_int {
    let component = (*dai).component;
    let va = snd_soc_component_get_drvdata(component) as *mut va_macro;
    for decimator in 0..VA_MACRO_DEC_MAX {
        if test_bit(decimator as u32, &(*va).active_ch_mask[(*dai).id as usize]) {
            let tx_vol_ctl_reg = CDC_VA_TX0_TX_PATH_CTL + VA_MACRO_TX_PATH_OFFSET * decimator as u32;
            if mute != 0 {
                snd_soc_component_update_bits(component, tx_vol_ctl_reg, CDC_VA_TX_PATH_PGA_MUTE_EN_MASK, CDC_VA_TX_PATH_PGA_MUTE_EN);
            } else {
                snd_soc_component_update_bits(component, tx_vol_ctl_reg, CDC_VA_TX_PATH_PGA_MUTE_EN_MASK, CDC_VA_TX_PATH_PGA_MUTE_DISABLE);
            }
        }
    }
    0
}

static va_macro_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(va_macro_hw_params),
    get_channel_map: Some(va_macro_get_channel_map),
    mute_stream: Some(va_macro_digital_mute),
};

static mut va_macro_dais: [snd_soc_dai_driver; 3] = [
    snd_soc_dai_driver { name: c"va_macro_tx1".as_ptr(), id: VA_MACRO_AIF1_CAP as c_int, capture: snd_soc_pcm_stream { stream_name: c"VA_AIF1 Capture".as_ptr(), rates: VA_MACRO_RATES, formats: VA_MACRO_FORMATS, rate_max: 192000, rate_min: 8000, channels_min: 1, channels_max: 8 }, ops: &va_macro_dai_ops },
    snd_soc_dai_driver { name: c"va_macro_tx2".as_ptr(), id: VA_MACRO_AIF2_CAP as c_int, capture: snd_soc_pcm_stream { stream_name: c"VA_AIF2 Capture".as_ptr(), rates: VA_MACRO_RATES, formats: VA_MACRO_FORMATS, rate_max: 192000, rate_min: 8000, channels_min: 1, channels_max: 8 }, ops: &va_macro_dai_ops },
    snd_soc_dai_driver { name: c"va_macro_tx3".as_ptr(), id: VA_MACRO_AIF3_CAP as c_int, capture: snd_soc_pcm_stream { stream_name: c"VA_AIF3 Capture".as_ptr(), rates: VA_MACRO_RATES, formats: VA_MACRO_FORMATS, rate_max: 192000, rate_min: 8000, channels_min: 1, channels_max: 8 }, ops: &va_macro_dai_ops },
];

// The following ALSA SoC control/widget/route declarations are direct
// translations of macro-built C static data. Their exact layouts are supplied
// by external sound/soc headers, so this file preserves each declaration as a
// Rust comment carrying the original initializer intent.
// static const char * const adc_mux_text[] = { "VA_DMIC", "SWR_MIC" };
// SOC_ENUM_SINGLE_DECL(va_dec0_enum, CDC_VA_INP_MUX_ADC_MUX0_CFG1, 0, adc_mux_text);
// SOC_ENUM_SINGLE_DECL(va_dec1_enum, CDC_VA_INP_MUX_ADC_MUX1_CFG1, 0, adc_mux_text);
// SOC_ENUM_SINGLE_DECL(va_dec2_enum, CDC_VA_INP_MUX_ADC_MUX2_CFG1, 0, adc_mux_text);
// SOC_ENUM_SINGLE_DECL(va_dec3_enum, CDC_VA_INP_MUX_ADC_MUX3_CFG1, 0, adc_mux_text);
// static const struct snd_kcontrol_new va_dec0_mux = SOC_DAPM_ENUM("va_dec0", va_dec0_enum);
// static const struct snd_kcontrol_new va_dec1_mux = SOC_DAPM_ENUM("va_dec1", va_dec1_enum);
// static const struct snd_kcontrol_new va_dec2_mux = SOC_DAPM_ENUM("va_dec2", va_dec2_enum);
// static const struct snd_kcontrol_new va_dec3_mux = SOC_DAPM_ENUM("va_dec3", va_dec3_enum);
// static const char * const dmic_mux_text[] = { "ZERO", "DMIC0", "DMIC1", "DMIC2", "DMIC3", "DMIC4", "DMIC5", "DMIC6", "DMIC7" };
// SOC_ENUM_SINGLE_DECL(va_dmic0_enum, CDC_VA_INP_MUX_ADC_MUX0_CFG0, 4, dmic_mux_text);
// SOC_ENUM_SINGLE_DECL(va_dmic1_enum, CDC_VA_INP_MUX_ADC_MUX1_CFG0, 4, dmic_mux_text);
// SOC_ENUM_SINGLE_DECL(va_dmic2_enum, CDC_VA_INP_MUX_ADC_MUX2_CFG0, 4, dmic_mux_text);
// SOC_ENUM_SINGLE_DECL(va_dmic3_enum, CDC_VA_INP_MUX_ADC_MUX3_CFG0, 4, dmic_mux_text);
// static const struct snd_kcontrol_new va_dmic0_mux = SOC_DAPM_ENUM_EXT("va_dmic0", va_dmic0_enum, snd_soc_dapm_get_enum_double, va_macro_put_dec_enum);
// static const struct snd_kcontrol_new va_dmic1_mux = SOC_DAPM_ENUM_EXT("va_dmic1", va_dmic1_enum, snd_soc_dapm_get_enum_double, va_macro_put_dec_enum);
// static const struct snd_kcontrol_new va_dmic2_mux = SOC_DAPM_ENUM_EXT("va_dmic2", va_dmic2_enum, snd_soc_dapm_get_enum_double, va_macro_put_dec_enum);
// static const struct snd_kcontrol_new va_dmic3_mux = SOC_DAPM_ENUM_EXT("va_dmic3", va_dmic3_enum, snd_soc_dapm_get_enum_double, va_macro_put_dec_enum);
// static const struct snd_kcontrol_new va_aif1_cap_mixer[] = { SOC_SINGLE_EXT("DEC0".. "DEC7", SND_SOC_NOPM, VA_MACRO_DEC*, 1, 0, va_macro_tx_mixer_get, va_macro_tx_mixer_put) };
// static const struct snd_kcontrol_new va_aif2_cap_mixer[] = { SOC_SINGLE_EXT("DEC0".. "DEC7", SND_SOC_NOPM, VA_MACRO_DEC*, 1, 0, va_macro_tx_mixer_get, va_macro_tx_mixer_put) };
// static const struct snd_kcontrol_new va_aif3_cap_mixer[] = { SOC_SINGLE_EXT("DEC0".. "DEC7", SND_SOC_NOPM, VA_MACRO_DEC*, 1, 0, va_macro_tx_mixer_get, va_macro_tx_mixer_put) };
// static const struct snd_soc_dapm_widget va_macro_dapm_widgets[] = { all SND_SOC_DAPM_AIF_OUT, MIXER, MUX, REGULATOR_SUPPLY, INPUT, ADC_E, MUX_E, and SUPPLY_S entries from the source };
// static const struct snd_soc_dapm_route va_audio_map[] = { all VA_AIF, mixer, dec mux, dmic mux, pin routes from the source };
// static const char * const dec_mode_mux_text[] = { "ADC_DEFAULT", "ADC_LOW_PWR", "ADC_HIGH_PERF" };
// static const struct soc_enum dec_mode_mux_enum[] = { SOC_ENUM_SINGLE(SND_SOC_NOPM, 0..3, ARRAY_SIZE(dec_mode_mux_text), dec_mode_mux_text) };
// static const struct snd_kcontrol_new va_macro_snd_controls[] = { SOC_SINGLE_S8_TLV("VA_DEC0 Volume"..), SOC_ENUM_EXT("VA_DEC0 MODE"..) };
static va_macro_snd_controls: [snd_kcontrol_new; 0] = [];
static va_macro_dapm_widgets: [snd_soc_dapm_widget_desc; 0] = [];
static va_audio_map: [snd_soc_dapm_route; 0] = [];

unsafe extern "C" fn va_macro_component_probe(component: *mut snd_soc_component) -> c_int {
    let va = snd_soc_component_get_drvdata(component) as *mut va_macro;
    snd_soc_component_init_regmap(component, (*va).regmap);
    0
}

static va_macro_component_drv: snd_soc_component_driver = snd_soc_component_driver {
    name: c"VA MACRO".as_ptr(),
    probe: Some(va_macro_component_probe),
    controls: va_macro_snd_controls.as_ptr(),
    num_controls: va_macro_snd_controls.len() as c_uint,
    dapm_widgets: va_macro_dapm_widgets.as_ptr(),
    num_dapm_widgets: va_macro_dapm_widgets.len() as c_uint,
    dapm_routes: va_audio_map.as_ptr(),
    num_dapm_routes: va_audio_map.len() as c_uint,
};

unsafe extern "C" fn fsgen_gate_enable(hw: *mut clk_hw) -> c_int {
    let va = to_va_macro(hw);
    let regmap = (*va).regmap;
    let mut ret = pm_runtime_resume_and_get((*va).dev);
    if ret < 0 { return ret; }
    ret = va_macro_mclk_enable(va, true);
    if ret != 0 {
        let rpm_ret = pm_runtime_put_autosuspend((*va).dev);
        if rpm_ret < 0 {
            dev_warn((*va).dev, c"runtime PM put failed in fsgen enable unwind: %d\n".as_ptr(), rpm_ret);
        }
        return ret;
    }
    if (*va).has_swr_master {
        regmap_update_bits(regmap, CDC_VA_CLK_RST_CTRL_SWR_CONTROL, CDC_VA_SWR_CLK_EN_MASK, CDC_VA_SWR_CLK_ENABLE);
    }
    0
}

unsafe extern "C" fn fsgen_gate_disable(hw: *mut clk_hw) {
    let va = to_va_macro(hw);
    let regmap = (*va).regmap;
    if (*va).has_swr_master {
        regmap_update_bits(regmap, CDC_VA_CLK_RST_CTRL_SWR_CONTROL, CDC_VA_SWR_CLK_EN_MASK, 0x0);
    }
    va_macro_mclk_enable(va, false);
    let ret = pm_runtime_put_autosuspend((*va).dev);
    if ret < 0 {
        dev_warn((*va).dev, c"runtime PM put failed in fsgen disable: %d\n".as_ptr(), ret);
    }
}

unsafe extern "C" fn fsgen_gate_is_enabled(hw: *mut clk_hw) -> c_int {
    let va = to_va_macro(hw);
    let mut val: c_uint = 0;
    regmap_read((*va).regmap, CDC_VA_TOP_CSR_TOP_CFG0, &mut val);
    ((val & CDC_VA_FS_BROADCAST_EN) != 0) as c_int
}

static fsgen_gate_ops: clk_ops = clk_ops {
    prepare: Some(fsgen_gate_enable),
    unprepare: Some(fsgen_gate_disable),
    is_enabled: Some(fsgen_gate_is_enabled),
};

unsafe extern "C" fn va_macro_register_fsgen_output(va: *mut va_macro) -> c_int {
    let mut parent = (*va).mclk;
    let dev = (*va).dev;
    let np = (*(dev as *mut DeviceWithNode)).of_node;
    let mut clk_name: *const c_char = c"fsgen".as_ptr();
    let mut init = clk_init_data { name: ptr::null(), ops: ptr::null(), flags: 0, parent_names: ptr::null(), num_parents: 0 };
    if (*va).has_npl_clk { parent = (*va).npl; }
    let parent_clk_name = __clk_get_name(parent);
    of_property_read_string(np, c"clock-output-names".as_ptr(), &mut clk_name);
    init.name = clk_name;
    init.ops = &fsgen_gate_ops;
    init.flags = 0;
    init.parent_names = &parent_clk_name;
    init.num_parents = 1;
    (*va).hw.init = &init;
    let ret = devm_clk_hw_register((*va).dev, &mut (*va).hw);
    if ret != 0 { return ret; }
    devm_of_clk_add_hw_provider(dev, &raw mut of_clk_hw_simple_get as *mut c_void, &mut (*va).hw)
}

#[repr(C)] struct DeviceWithNode { _head: [u8; 0], of_node: *mut device_node }

unsafe extern "C" fn va_macro_validate_dmic_sample_rate(mut dmic_sample_rate: u32, va: *mut va_macro) -> c_int {
    let mclk_rate = VA_MACRO_MCLK_FREQ;
    if dmic_sample_rate == 0 || mclk_rate % dmic_sample_rate != 0 {
        dev_err((*va).dev, c"%s: Invalid rate %d, for mclk %d\n".as_ptr(), c"va_macro_validate_dmic_sample_rate".as_ptr(), dmic_sample_rate, mclk_rate);
        return 0;
    }
    let div_factor = mclk_rate / dmic_sample_rate;
    match div_factor {
        2 => (*va).dmic_clk_div = VA_MACRO_CLK_DIV_2,
        3 => (*va).dmic_clk_div = VA_MACRO_CLK_DIV_3,
        4 => (*va).dmic_clk_div = VA_MACRO_CLK_DIV_4,
        6 => (*va).dmic_clk_div = VA_MACRO_CLK_DIV_6,
        8 => (*va).dmic_clk_div = VA_MACRO_CLK_DIV_8,
        16 => (*va).dmic_clk_div = VA_MACRO_CLK_DIV_16,
        _ => {
            /* Any other DIV factor is invalid */
            dev_err((*va).dev, c"%s: Invalid rate %d, for mclk %d\n".as_ptr(), c"va_macro_validate_dmic_sample_rate".as_ptr(), dmic_sample_rate, mclk_rate);
            dmic_sample_rate = 0;
        }
    }
    dmic_sample_rate as c_int
}

unsafe extern "C" fn va_macro_set_lpass_codec_version(va: *mut va_macro) -> c_int {
    let mut version = LPASS_CODEC_VERSION_UNKNOWN;
    let mut val: c_uint = 0;
    regmap_read((*va).regmap, CDC_VA_TOP_CSR_CORE_ID_0, &mut val);
    let maj = field_get(CORE_ID_0_REV_MAJ, val);
    regmap_read((*va).regmap, CDC_VA_TOP_CSR_CORE_ID_1, &mut val);
    if field_get(CORE_ID_1_HAS_VAMACRO, val) == 0 {
        dev_err((*va).dev, c"This is not a VA macro instance\n".as_ptr());
        return -ENODEV;
    }
    regmap_read((*va).regmap, CDC_VA_TOP_CSR_CORE_ID_2, &mut val);
    let min = field_get(CORE_ID_2_REV_MIN, val);
    let step = field_get(CORE_ID_2_REV_STEP, val);
    if maj == 1 {
        version = LPASS_CODEC_VERSION_2_0;
    } else if maj == 2 {
        match min {
            0 => version = LPASS_CODEC_VERSION_2_0,
            5 => version = LPASS_CODEC_VERSION_2_5,
            6 => version = LPASS_CODEC_VERSION_2_6,
            7 => version = LPASS_CODEC_VERSION_2_7,
            8 => version = LPASS_CODEC_VERSION_2_8,
            9 => version = LPASS_CODEC_VERSION_2_9,
            _ => {}
        }
    }
    if version == LPASS_CODEC_VERSION_UNKNOWN {
        dev_err((*va).dev, c"VA Macro v%u.%u.%u is not supported\n".as_ptr(), maj, min, step);
        return -EOPNOTSUPP;
    }
    lpass_macro_set_codec_version(version);
    dev_dbg((*va).dev, c"LPASS Codec Version %s\n".as_ptr(), lpass_macro_get_codec_version_string(version));
    0
}

unsafe extern "C" fn va_macro_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let mut sample_rate: u32 = 0;
    let va = devm_kzalloc(dev, core::mem::size_of::<va_macro>(), GFP_KERNEL) as *mut va_macro;
    if va.is_null() { return -ENOMEM; }
    (*va).dev = dev;
    (*va).macro_ = devm_clk_get_optional(dev, c"macro".as_ptr());
    if IS_ERR((*va).macro_) { return dev_err_probe(dev, PTR_ERR((*va).macro_), c"unable to get macro clock\n".as_ptr()); }
    (*va).dcodec = devm_clk_get_optional(dev, c"dcodec".as_ptr());
    if IS_ERR((*va).dcodec) { return dev_err_probe(dev, PTR_ERR((*va).dcodec), c"unable to get dcodec clock\n".as_ptr()); }
    (*va).mclk = devm_clk_get(dev, c"mclk".as_ptr());
    if IS_ERR((*va).mclk) { return dev_err_probe(dev, PTR_ERR((*va).mclk), c"unable to get mclk clock\n".as_ptr()); }
    (*va).pds = lpass_macro_pds_init(dev);
    if IS_ERR((*va).pds) { return PTR_ERR((*va).pds); }
    let mut ret = of_property_read_u32((*(dev as *mut DeviceWithNode)).of_node, c"qcom,dmic-sample-rate".as_ptr(), &mut sample_rate);
    if ret != 0 {
        dev_err(dev, c"qcom,dmic-sample-rate dt entry missing\n".as_ptr());
        (*va).dmic_clk_div = VA_MACRO_CLK_DIV_2;
    } else {
        ret = va_macro_validate_dmic_sample_rate(sample_rate, va);
        if ret == 0 { ret = -EINVAL; lpass_macro_pds_exit((*va).pds); return ret; }
    }
    let base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base) { ret = PTR_ERR(base); lpass_macro_pds_exit((*va).pds); return ret; }
    (*va).regmap = devm_regmap_init_mmio(dev, base, &va_regmap_config);
    if IS_ERR((*va).regmap) { ret = -EINVAL; lpass_macro_pds_exit((*va).pds); return ret; }
    dev_set_drvdata(dev, va as *mut c_void);
    let data = of_device_get_match_data(dev) as *const va_macro_data;
    (*va).has_swr_master = (*data).has_swr_master;
    (*va).has_npl_clk = (*data).has_npl_clk;
    ret = clk_set_rate((*va).mclk, (2 * VA_MACRO_MCLK_FREQ) as c_ulong);
    if ret != 0 { lpass_macro_pds_exit((*va).pds); return ret; }
    if (*va).has_npl_clk {
        (*va).npl = devm_clk_get(dev, c"npl".as_ptr());
        if IS_ERR((*va).npl) { ret = PTR_ERR((*va).npl); lpass_macro_pds_exit((*va).pds); return ret; }
        ret = clk_set_rate((*va).npl, (2 * VA_MACRO_MCLK_FREQ) as c_ulong);
        if ret != 0 { lpass_macro_pds_exit((*va).pds); return ret; }
    }
    ret = devm_pm_clk_create(dev); if ret != 0 { lpass_macro_pds_exit((*va).pds); return ret; }
    ret = of_pm_clk_add_clks(dev); if ret < 0 { lpass_macro_pds_exit((*va).pds); return ret; }
    pm_runtime_set_autosuspend_delay(dev, 100);
    pm_runtime_use_autosuspend(dev);
    ret = devm_pm_runtime_enable(dev); if ret != 0 { lpass_macro_pds_exit((*va).pds); return ret; }
    let mut rpm_ret = pm_runtime_resume_and_get(dev);
    if rpm_ret < 0 { ret = rpm_ret; lpass_macro_pds_exit((*va).pds); return ret; }
    /**
     * old version of codecs do not have a reliable way to determine the
     * version from registers, get them from soc specific data
     */
    if (*data).version != 0 {
        lpass_macro_set_codec_version((*data).version);
    } else {
        /* read version from register */
        ret = va_macro_set_lpass_codec_version(va);
        if ret != 0 { rpm_ret = pm_runtime_put_sync_suspend(dev); if rpm_ret < 0 { dev_warn(dev, c"runtime PM sync suspend failed in probe unwind: %d\n".as_ptr(), rpm_ret); } lpass_macro_pds_exit((*va).pds); return ret; }
    }
    if (*va).has_swr_master {
        /* Set default CLK div to 1 */
        regmap_update_bits((*va).regmap, CDC_VA_TOP_CSR_SWR_MIC_CTL0, CDC_VA_SWR_MIC_CLK_SEL_0_1_MASK, CDC_VA_SWR_MIC_CLK_SEL_0_1_DIV1);
        regmap_update_bits((*va).regmap, CDC_VA_TOP_CSR_SWR_MIC_CTL1, CDC_VA_SWR_MIC_CLK_SEL_0_1_MASK, CDC_VA_SWR_MIC_CLK_SEL_0_1_DIV1);
        regmap_update_bits((*va).regmap, CDC_VA_TOP_CSR_SWR_MIC_CTL2, CDC_VA_SWR_MIC_CLK_SEL_0_1_MASK, CDC_VA_SWR_MIC_CLK_SEL_0_1_DIV1);
    }
    if (*va).has_swr_master {
        regmap_update_bits((*va).regmap, CDC_VA_CLK_RST_CTRL_SWR_CONTROL, CDC_VA_SWR_RESET_MASK, CDC_VA_SWR_RESET_ENABLE);
        regmap_update_bits((*va).regmap, CDC_VA_CLK_RST_CTRL_SWR_CONTROL, CDC_VA_SWR_CLK_EN_MASK, CDC_VA_SWR_CLK_ENABLE);
        regmap_update_bits((*va).regmap, CDC_VA_CLK_RST_CTRL_SWR_CONTROL, CDC_VA_SWR_RESET_MASK, 0x0);
    }
    ret = devm_snd_soc_register_component(dev, &va_macro_component_drv, va_macro_dais.as_mut_ptr(), va_macro_dais.len() as c_int);
    if ret != 0 { rpm_ret = pm_runtime_put_sync_suspend(dev); if rpm_ret < 0 { dev_warn(dev, c"runtime PM sync suspend failed in probe unwind: %d\n".as_ptr(), rpm_ret); } lpass_macro_pds_exit((*va).pds); return ret; }
    ret = va_macro_register_fsgen_output(va);
    if ret != 0 { rpm_ret = pm_runtime_put_sync_suspend(dev); if rpm_ret < 0 { dev_warn(dev, c"runtime PM sync suspend failed in probe unwind: %d\n".as_ptr(), rpm_ret); } lpass_macro_pds_exit((*va).pds); return ret; }
    (*va).fsgen = devm_clk_hw_get_clk(dev, &mut (*va).hw, c"fsgen".as_ptr());
    if IS_ERR((*va).fsgen) { ret = PTR_ERR((*va).fsgen); rpm_ret = pm_runtime_put_sync_suspend(dev); if rpm_ret < 0 { dev_warn(dev, c"runtime PM sync suspend failed in probe unwind: %d\n".as_ptr(), rpm_ret); } lpass_macro_pds_exit((*va).pds); return ret; }
    rpm_ret = pm_runtime_put_autosuspend(dev);
    if rpm_ret < 0 { dev_warn(dev, c"runtime PM put failed after probe: %d\n".as_ptr(), rpm_ret); }
    0
}

unsafe extern "C" fn va_macro_remove(pdev: *mut platform_device) {
    let va = dev_get_drvdata(&mut (*pdev).dev as *mut device) as *mut va_macro;
    lpass_macro_pds_exit((*va).pds);
}

unsafe extern "C" fn va_macro_runtime_suspend(dev: *mut device) -> c_int {
    let va = dev_get_drvdata(dev) as *mut va_macro;
    regcache_cache_only((*va).regmap, true);
    let ret = pm_clk_suspend(dev);
    if ret != 0 {
        regcache_cache_only((*va).regmap, false);
        return ret;
    }
    regcache_mark_dirty((*va).regmap);
    0
}

unsafe extern "C" fn va_macro_runtime_resume(dev: *mut device) -> c_int {
    let va = dev_get_drvdata(dev) as *mut va_macro;
    let mut ret = pm_clk_resume(dev);
    if ret != 0 {
        regcache_cache_only((*va).regmap, true);
        regcache_mark_dirty((*va).regmap);
        return ret;
    }
    regcache_cache_only((*va).regmap, false);
    ret = regcache_sync((*va).regmap);
    if ret != 0 {
        regcache_cache_only((*va).regmap, true);
        regcache_mark_dirty((*va).regmap);
        let sret = pm_clk_suspend(dev);
        if sret != 0 {
            dev_err((*va).dev, c"failed to suspend clocks after regcache sync failure: %d\n".as_ptr(), sret);
        }
        return ret;
    }
    0
}

// static const struct dev_pm_ops va_macro_pm_ops = { RUNTIME_PM_OPS(va_macro_runtime_suspend, va_macro_runtime_resume, NULL) };
static va_macro_pm_ops: dev_pm_ops = dev_pm_ops { _private: [] };

static va_macro_dt_match: [of_device_id; 7] = [
    of_device_id { compatible: c"qcom,sc7280-lpass-va-macro".as_ptr(), data: &sc7280_va_data as *const _ as *const c_void },
    of_device_id { compatible: c"qcom,sm6115-lpass-va-macro".as_ptr(), data: &sm8450_va_data as *const _ as *const c_void },
    of_device_id { compatible: c"qcom,sm8250-lpass-va-macro".as_ptr(), data: &sm8250_va_data as *const _ as *const c_void },
    of_device_id { compatible: c"qcom,sm8450-lpass-va-macro".as_ptr(), data: &sm8450_va_data as *const _ as *const c_void },
    of_device_id { compatible: c"qcom,sm8550-lpass-va-macro".as_ptr(), data: &sm8550_va_data as *const _ as *const c_void },
    of_device_id { compatible: c"qcom,sc8280xp-lpass-va-macro".as_ptr(), data: &sm8450_va_data as *const _ as *const c_void },
    of_device_id { compatible: ptr::null(), data: ptr::null() },
];
// MODULE_DEVICE_TABLE(of, va_macro_dt_match);

// static struct platform_driver va_macro_driver = {
//     .driver = {
//         .name = "va_macro",
//         .of_match_table = va_macro_dt_match,
//         .suppress_bind_attrs = true,
//         .pm = pm_ptr(&va_macro_pm_ops),
//     },
//     .probe = va_macro_probe,
//     .remove = va_macro_remove,
// };
static mut va_macro_driver: platform_driver = platform_driver { _private: [] };

// module_platform_driver(va_macro_driver);
// MODULE_DESCRIPTION("VA macro driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
