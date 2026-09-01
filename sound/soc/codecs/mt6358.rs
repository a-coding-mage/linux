// SPDX-License-Identifier: GPL-2.0
//
// mt6358.rs  --  mt6358 ALSA SoC audio codec driver
//
// Copyright (c) 2018 MediaTek Inc.
// Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>
//
// Rust source-level translation of soc/codecs/mt6358.c.
// Linux, ALSA SoC, regmap, regulator, OF, and mt6358 register symbols are
// external dependencies corresponding to the original C includes.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regulator {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: usize,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}
#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}
#[repr(C)]
pub struct soc_mixer_control {
    pub reg: c_uint,
}
#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
    pub kcontrols: *mut *mut snd_kcontrol,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub number: c_int,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct mt6397_chip {
    pub regmap: *mut regmap,
}
#[repr(C)]
pub struct platform_device {
    pub dev: device_with_parent,
}
#[repr(C)]
pub struct device_with_parent {
    pub parent: *mut device,
    pub of_node: *mut c_void,
}

unsafe extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_get_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_dapm_kcontrol_get_value(kcontrol: *mut snd_kcontrol) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_component_init_regmap(component: *mut snd_soc_component, regmap: *mut regmap);
    fn devm_regulator_get(dev: *mut device, id: *const c_char) -> *mut regulator;
    fn regulator_enable(regulator: *mut regulator) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device_with_parent, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device_with_parent, data: *mut c_void);
    fn dev_name(dev: *mut device_with_parent) -> *const c_char;
    fn devm_snd_soc_register_component(
        dev: *mut device_with_parent,
        component_driver: *const c_void,
        dai_driver: *mut c_void,
        num_dai: usize,
    ) -> c_int;
    fn of_property_read_u32(np: *mut c_void, propname: *const c_char, out_value: *mut c_int) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn msleep(msecs: c_uint);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

const AUDIO_ANALOG_VOLUME_HSOUTL: usize = 0;
const AUDIO_ANALOG_VOLUME_HSOUTR: usize = 1;
const AUDIO_ANALOG_VOLUME_HPOUTL: usize = 2;
const AUDIO_ANALOG_VOLUME_HPOUTR: usize = 3;
const AUDIO_ANALOG_VOLUME_LINEOUTL: usize = 4;
const AUDIO_ANALOG_VOLUME_LINEOUTR: usize = 5;
const AUDIO_ANALOG_VOLUME_MICAMP1: usize = 6;
const AUDIO_ANALOG_VOLUME_MICAMP2: usize = 7;
const AUDIO_ANALOG_VOLUME_TYPE_MAX: usize = 8;

const MUX_ADC_L: usize = 0;
const MUX_ADC_R: usize = 1;
const MUX_PGA_L: usize = 2;
const MUX_PGA_R: usize = 3;
const MUX_MIC_TYPE: usize = 4;
const MUX_HP_L: usize = 5;
const MUX_HP_R: usize = 6;
const MUX_NUM: usize = 7;

const DEVICE_HP: usize = 0;
const DEVICE_LO: usize = 1;
const DEVICE_RCV: usize = 2;
const DEVICE_MIC1: usize = 3;
const DEVICE_MIC2: usize = 4;
const DEVICE_NUM: usize = 5;

const SUPPLY_SEQ_CLK_BUF: c_int = 0;
const SUPPLY_SEQ_AUD_GLB: c_int = 1;
const SUPPLY_SEQ_CLKSQ: c_int = 2;
const SUPPLY_SEQ_VOW_AUD_LPW: c_int = 3;
const SUPPLY_SEQ_AUD_VOW: c_int = 4;
const SUPPLY_SEQ_VOW_CLK: c_int = 5;
const SUPPLY_SEQ_VOW_LDO: c_int = 6;
const SUPPLY_SEQ_TOP_CK: c_int = 7;
const SUPPLY_SEQ_TOP_CK_LAST: c_int = 8;
const SUPPLY_SEQ_AUD_TOP: c_int = 9;
const SUPPLY_SEQ_AUD_TOP_LAST: c_int = 10;
const SUPPLY_SEQ_AFE: c_int = 11;
const SUPPLY_SEQ_ADC_SUPPLY: c_int = 12;

const CH_L: c_int = 0;
const CH_R: c_int = 1;
const NUM_CH: c_int = 2;
const REG_STRIDE: c_int = 2;

#[repr(C)]
pub struct mt6358_priv {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub dl_rate: c_uint,
    pub ul_rate: c_uint,
    pub ana_gain: [c_int; AUDIO_ANALOG_VOLUME_TYPE_MAX],
    pub mux_select: [c_uint; MUX_NUM],
    pub dev_counter: [c_int; DEVICE_NUM],
    pub mtkaif_protocol: c_int,
    pub avdd_reg: *mut regulator,
    pub wov_enabled: c_int,
    pub dmic_one_wire_mode: c_int,
}

unsafe extern "C" {
    static MT6358_MTKAIF_PROTOCOL_2_CLK_P2: c_int;
    static MT6358_MTKAIF_PROTOCOL_2: c_int;
    static MT6358_MTKAIF_PROTOCOL_1: c_int;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static SND_SOC_DAPM_WILL_PMU: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static GFP_KERNEL: c_uint;
    static EINVAL: c_int;
    static ENOMEM: c_int;
}

unsafe extern "C" {
    static MT6358_GPIO_MODE2_CLR: c_uint;
    static MT6358_GPIO_MODE2_SET: c_uint;
    static MT6358_GPIO_MODE2: c_uint;
    static MT6358_GPIO_DIR0: c_uint;
    static MT6358_GPIO_MODE3_CLR: c_uint;
    static MT6358_GPIO_MODE3_SET: c_uint;
    static MT6358_GPIO_MODE3: c_uint;
    static MT6358_AFE_ADDA_MTKAIF_CFG0: c_uint;
    static MT6358_AFE_AUD_PAD_TOP: c_uint;
    static MT6358_ZCD_CON0: c_uint;
    static MT6358_AUDDEC_ANA_CON1: c_uint;
    static MT6358_AUDDEC_ANA_CON9: c_uint;
    static MT6358_AUDDEC_ANA_CON4: c_uint;
    static MT6358_ZCD_CON2: c_uint;
    static MT6358_ZCD_CON1: c_uint;
    static MT6358_ZCD_CON3: c_uint;
    static MT6358_AUDENC_ANA_CON0: c_uint;
    static MT6358_AUDENC_ANA_CON1: c_uint;
    static MT6358_AUDDEC_ANA_CON13: c_uint;
    static MT6358_DCXO_CW14: c_uint;
    static MT6358_DCXO_CW13: c_uint;
    static MT6358_AUDENC_ANA_CON9: c_uint;
    static MT6358_AUDENC_ANA_CON8: c_uint;
    static MT6358_AUD_TOP_CKPDN_CON0: c_uint;
    static MT6358_AFE_VOW_CFG0: c_uint;
    static MT6358_AFE_VOW_CFG1: c_uint;
    static MT6358_AFE_VOW_CFG2: c_uint;
    static MT6358_AFE_VOW_CFG3: c_uint;
    static MT6358_AFE_VOW_CFG4: c_uint;
    static MT6358_AFE_VOW_CFG5: c_uint;
    static MT6358_AFE_VOW_POSDIV_CFG0: c_uint;
    static MT6358_AFE_VOW_HPF_CFG0: c_uint;
    static MT6358_AFE_VOW_TOP: c_uint;
    static MT6358_AFUNC_AUD_CON2: c_uint;
    static MT6358_AFUNC_AUD_CON0: c_uint;
    static MT6358_AFE_SGEN_CFG0: c_uint;
    static MT6358_AFE_SGEN_CFG1: c_uint;
    static MT6358_AUDDEC_ANA_CON2: c_uint;
    static MT6358_AUDNCP_CLKDIV_CON1: c_uint;
    static MT6358_AUDNCP_CLKDIV_CON2: c_uint;
    static MT6358_AUDNCP_CLKDIV_CON0: c_uint;
    static MT6358_AUDNCP_CLKDIV_CON4: c_uint;
    static MT6358_AUDNCP_CLKDIV_CON3: c_uint;
    static MT6358_AUDDEC_ANA_CON14: c_uint;
    static MT6358_AUDDEC_ANA_CON15: c_uint;
    static MT6358_AUDDEC_ANA_CON0: c_uint;
    static MT6358_AUDDEC_ANA_CON12: c_uint;
    static MT6358_AUDDEC_ANA_CON11: c_uint;
    static MT6358_AUDDEC_ANA_CON10: c_uint;
    static MT6358_AUDDEC_ANA_CON7: c_uint;
    static MT6358_AUDDEC_ANA_CON6: c_uint;
    static MT6358_AUDENC_ANA_CON6: c_uint;
    static MT6358_AUDENC_ANA_CON3: c_uint;
    static MT6358_AUDENC_ANA_CON10: c_uint;
    static MT6358_AFE_DCCLK_CFG0: c_uint;
    static MT6358_AFE_DCCLK_CFG1: c_uint;
    static MT6358_AFE_UL_SRC_CON0_H: c_uint;
    static MT6358_AFE_UL_SRC_CON0_L: c_uint;
    static MT6358_AFE_DL_SRC2_CON0_L: c_uint;
    static MT6358_AFE_TOP_CON0: c_uint;
    static MT6358_DCXO_CW14_REG: c_uint;
    static MT6358_AUDIO_TOP_CON0: c_uint;
    static MT6358_AFE_UL_DL_CON0: c_uint;
    static MT6358_ACCDET_CON13: c_uint;
    static MT6358_DRV_CON3: c_uint;
}

unsafe extern "C" {
    static RG_AUDHPLGAIN_SFT: c_uint;
    static RG_AUDHPLGAIN_MASK: c_uint;
    static RG_AUDHPRGAIN_SFT: c_uint;
    static RG_AUDHPRGAIN_MASK: c_uint;
    static RG_AUDLOLGAIN_SFT: c_uint;
    static RG_AUDLOLGAIN_MASK: c_uint;
    static RG_AUDLORGAIN_SFT: c_uint;
    static RG_AUDLORGAIN_MASK: c_uint;
    static RG_AUDHSGAIN_SFT: c_uint;
    static RG_AUDHSGAIN_MASK: c_uint;
    static RG_AUDPREAMPLGAIN_SFT: c_uint;
    static RG_AUDPREAMPLGAIN_MASK: c_uint;
    static RG_AUDPREAMPRGAIN_SFT: c_uint;
    static RG_AUDPREAMPRGAIN_MASK: c_uint;
    static RG_CLKSQ_IN_SEL_TEST_MASK_SFT: c_uint;
    static RG_AUDLOLMUXINPUTSEL_VAUDP15_SFT: c_uint;
    static RG_AUDLOLMUXINPUTSEL_VAUDP15_MASK: c_uint;
    static DL_SINE_ON_SFT: c_uint;
    static DL_SINE_ON_MASK: c_uint;
    static UL_SINE_ON_SFT: c_uint;
    static UL_SINE_ON_MASK: c_uint;
    static RG_AUDHSMUXINPUTSEL_VAUDP15_MASK_SFT: c_uint;
    static RG_AUDPREAMPLINPUTSEL_MASK_SFT: c_uint;
    static RG_AUDPREAMPLINPUTSEL_SFT: c_uint;
    static RG_AUDPREAMPLON_MASK_SFT: c_uint;
    static RG_AUDPREAMPLON_SFT: c_uint;
    static RG_AUDPREAMPLDCCEN_MASK_SFT: c_uint;
    static RG_AUDPREAMPLDCCEN_SFT: c_uint;
    static RG_AUDADCLINPUTSEL_MASK_SFT: c_uint;
    static RG_AUDADCLINPUTSEL_SFT: c_uint;
    static RG_AUDADCLPWRUP_MASK_SFT: c_uint;
    static RG_AUDADCLPWRUP_SFT: c_uint;
    static RG_AUDPREAMPRINPUTSEL_MASK_SFT: c_uint;
    static RG_AUDPREAMPRINPUTSEL_SFT: c_uint;
    static RG_AUDPREAMPRON_MASK_SFT: c_uint;
    static RG_AUDPREAMPRON_SFT: c_uint;
    static RG_AUDPREAMPRDCCEN_MASK_SFT: c_uint;
    static RG_AUDPREAMPRDCCEN_SFT: c_uint;
    static RG_AUDADCRINPUTSEL_MASK_SFT: c_uint;
    static RG_AUDADCRINPUTSEL_SFT: c_uint;
    static RG_AUDADCRPWRUP_MASK_SFT: c_uint;
    static RG_AUDADCRPWRUP_SFT: c_uint;
    static RG_AUDPREAMPLDCPRECHARGE_MASK_SFT: c_uint;
    static RG_AUDPREAMPRDCPRECHARGE_MASK_SFT: c_uint;
    static RG_AUDPREAMPLGAIN_MASK_SFT: c_uint;
    static RG_AUDPREAMPRGAIN_MASK_SFT: c_uint;
    static RG_AUDHPLSCDISABLE_VAUDP15_MASK_SFT: c_uint;
    static RG_AUDHPLSCDISABLE_VAUDP15_SFT: c_uint;
    static RG_AUDHPRSCDISABLE_VAUDP15_MASK_SFT: c_uint;
    static RG_AUDHPRSCDISABLE_VAUDP15_SFT: c_uint;
    static RG_AUDHSSCDISABLE_VAUDP15_MASK_SFT: c_uint;
    static RG_AUDHSSCDISABLE_VAUDP15_SFT: c_uint;
    static RG_AUDLOLSCDISABLE_VAUDP15_MASK_SFT: c_uint;
    static RG_AUDLOLSCDISABLE_VAUDP15_SFT: c_uint;
}

const DL_GAIN_8DB: c_int = 0;
const DL_GAIN_0DB: c_int = 8;
const DL_GAIN_N_1DB: c_int = 9;
const DL_GAIN_N_10DB: c_int = 18;
const DL_GAIN_N_40DB: c_int = 0x1f;
const DL_GAIN_N_10DB_REG: c_uint = ((DL_GAIN_N_10DB << 7) | DL_GAIN_N_10DB) as c_uint;
const DL_GAIN_N_40DB_REG: c_uint = ((DL_GAIN_N_40DB << 7) | DL_GAIN_N_40DB) as c_uint;
const DL_GAIN_REG_MASK: c_uint = 0x0f9f;

#[no_mangle]
pub unsafe extern "C" fn mt6358_set_mtkaif_protocol(
    cmpnt: *mut snd_soc_component,
    mtkaif_protocol: c_int,
) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(cmpnt) as *mut mt6358_priv;
    (*priv_).mtkaif_protocol = mtkaif_protocol;
    0
}

unsafe fn playback_gpio_set(priv_: *mut mt6358_priv) {
    regmap_update_bits((*priv_).regmap, MT6358_GPIO_MODE2_CLR, 0x01f8, 0x01f8);
    regmap_update_bits((*priv_).regmap, MT6358_GPIO_MODE2_SET, 0xffff, 0x0249);
    regmap_update_bits((*priv_).regmap, MT6358_GPIO_MODE2, 0xffff, 0x0249);
}

unsafe fn playback_gpio_reset(priv_: *mut mt6358_priv) {
    regmap_update_bits((*priv_).regmap, MT6358_GPIO_MODE2_CLR, 0x01f8, 0x01f8);
    regmap_update_bits((*priv_).regmap, MT6358_GPIO_MODE2, 0x01f8, 0x0000);
    regmap_update_bits((*priv_).regmap, MT6358_GPIO_DIR0, 0xf << 8, 0x0);
}

unsafe fn capture_gpio_set(priv_: *mut mt6358_priv) {
    regmap_update_bits((*priv_).regmap, MT6358_GPIO_MODE3_CLR, 0xffff, 0xffff);
    regmap_update_bits((*priv_).regmap, MT6358_GPIO_MODE3_SET, 0xffff, 0x0249);
    regmap_update_bits((*priv_).regmap, MT6358_GPIO_MODE3, 0xffff, 0x0249);
}

unsafe fn capture_gpio_reset(priv_: *mut mt6358_priv) {
    regmap_update_bits((*priv_).regmap, MT6358_GPIO_MODE3_CLR, 0xffff, 0xffff);
    regmap_update_bits((*priv_).regmap, MT6358_GPIO_MODE3, 0xffff, 0x0000);
    regmap_update_bits((*priv_).regmap, MT6358_GPIO_DIR0, 0xf << 12, 0x0);
}

unsafe fn mt6358_mtkaif_tx_enable(priv_: *mut mt6358_priv) -> c_int {
    if (*priv_).mtkaif_protocol == MT6358_MTKAIF_PROTOCOL_2_CLK_P2 {
        regmap_update_bits((*priv_).regmap, MT6358_AFE_ADDA_MTKAIF_CFG0, 0xffff, 0x0010);
        regmap_update_bits((*priv_).regmap, MT6358_AFE_AUD_PAD_TOP, 0xff00, 0x3800);
        regmap_update_bits((*priv_).regmap, MT6358_AFE_AUD_PAD_TOP, 0xff00, 0x3900);
    } else if (*priv_).mtkaif_protocol == MT6358_MTKAIF_PROTOCOL_2 {
        regmap_update_bits((*priv_).regmap, MT6358_AFE_ADDA_MTKAIF_CFG0, 0xffff, 0x0010);
        regmap_update_bits((*priv_).regmap, MT6358_AFE_AUD_PAD_TOP, 0xff00, 0x3100);
    } else {
        regmap_update_bits((*priv_).regmap, MT6358_AFE_ADDA_MTKAIF_CFG0, 0xffff, 0x0000);
        regmap_update_bits((*priv_).regmap, MT6358_AFE_AUD_PAD_TOP, 0xff00, 0x3100);
    }
    0
}

unsafe fn mt6358_mtkaif_tx_disable(priv_: *mut mt6358_priv) -> c_int {
    regmap_update_bits((*priv_).regmap, MT6358_AFE_AUD_PAD_TOP, 0xff00, 0x3000);
    0
}

unsafe fn hp_zcd_disable(priv_: *mut mt6358_priv) {
    regmap_write((*priv_).regmap, MT6358_ZCD_CON0, 0x0000);
}

unsafe fn hp_main_output_ramp(priv_: *mut mt6358_priv, up: bool) {
    let target: c_int = 7;
    let mut i = 0;
    while i <= target {
        let stage = if up { i } else { target - i };
        regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON1, 0x7 << 8, (stage << 8) as c_uint);
        regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON1, 0x7 << 11, (stage << 11) as c_uint);
        usleep_range(100, 150);
        i += 1;
    }
}

unsafe fn hp_aux_feedback_loop_gain_ramp(priv_: *mut mt6358_priv, up: bool) {
    let mut i = 0;
    while i <= 0xf {
        let stage = if up { i } else { 0xf - i };
        regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON9, 0xf << 12, (stage << 12) as c_uint);
        usleep_range(100, 150);
        i += 1;
    }
}

unsafe fn hp_pull_down(priv_: *mut mt6358_priv, enable: bool) {
    if enable {
        let mut i = 0x0;
        while i <= 0x6 {
            regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON4, 0x7, i);
            usleep_range(600, 700);
            i += 1;
        }
    } else {
        let mut i = 0x6;
        while i >= 0x1 {
            regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON4, 0x7, i);
            usleep_range(600, 700);
            i -= 1;
        }
    }
}

fn is_valid_hp_pga_idx(reg_idx: c_int) -> bool {
    (reg_idx >= DL_GAIN_8DB && reg_idx <= DL_GAIN_N_10DB) || reg_idx == DL_GAIN_N_40DB
}

unsafe fn headset_volume_ramp(priv_: *mut mt6358_priv, from: c_int, to: c_int) {
    let mut count: c_int = 0;
    if !is_valid_hp_pga_idx(from) || !is_valid_hp_pga_idx(to) {
        dev_warn((*priv_).dev, cstr!("%s(), volume index is not valid, from %d, to %d\n"), cstr!("headset_volume_ramp"), from, to);
    }
    dev_info((*priv_).dev, cstr!("%s(), from %d, to %d\n"), cstr!("headset_volume_ramp"), from, to);
    let mut offset = if to > from { to - from } else { from - to };
    while offset >= 0 {
        let reg_idx = if to > from { from + count } else { from - count };
        if is_valid_hp_pga_idx(reg_idx) {
            regmap_update_bits((*priv_).regmap, MT6358_ZCD_CON2, DL_GAIN_REG_MASK, ((reg_idx << 7) | reg_idx) as c_uint);
            usleep_range(200, 300);
        }
        offset -= 1;
        count += 1;
    }
}

unsafe fn mt6358_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_component_get_drvdata(component) as *mut mt6358_priv;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let mut reg: c_uint = 0;
    let ret = snd_soc_put_volsw(kcontrol, ucontrol);
    if ret < 0 {
        return ret;
    }
    if (*mc).reg == MT6358_ZCD_CON2 {
        regmap_read((*priv_).regmap, MT6358_ZCD_CON2, &mut reg);
        (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_HPOUTL] = ((reg >> RG_AUDHPLGAIN_SFT) & RG_AUDHPLGAIN_MASK) as c_int;
        (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_HPOUTR] = ((reg >> RG_AUDHPRGAIN_SFT) & RG_AUDHPRGAIN_MASK) as c_int;
    } else if (*mc).reg == MT6358_ZCD_CON1 {
        regmap_read((*priv_).regmap, MT6358_ZCD_CON1, &mut reg);
        (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_LINEOUTL] = ((reg >> RG_AUDLOLGAIN_SFT) & RG_AUDLOLGAIN_MASK) as c_int;
        (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_LINEOUTR] = ((reg >> RG_AUDLORGAIN_SFT) & RG_AUDLORGAIN_MASK) as c_int;
    } else if (*mc).reg == MT6358_ZCD_CON3 {
        regmap_read((*priv_).regmap, MT6358_ZCD_CON3, &mut reg);
        (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_HSOUTL] = ((reg >> RG_AUDHSGAIN_SFT) & RG_AUDHSGAIN_MASK) as c_int;
        (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_HSOUTR] = ((reg >> RG_AUDHSGAIN_SFT) & RG_AUDHSGAIN_MASK) as c_int;
    } else if (*mc).reg == MT6358_AUDENC_ANA_CON0 || (*mc).reg == MT6358_AUDENC_ANA_CON1 {
        regmap_read((*priv_).regmap, MT6358_AUDENC_ANA_CON0, &mut reg);
        (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_MICAMP1] = ((reg >> RG_AUDPREAMPLGAIN_SFT) & RG_AUDPREAMPLGAIN_MASK) as c_int;
        regmap_read((*priv_).regmap, MT6358_AUDENC_ANA_CON1, &mut reg);
        (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_MICAMP2] = ((reg >> RG_AUDPREAMPRGAIN_SFT) & RG_AUDPREAMPRGAIN_MASK) as c_int;
    }
    ret
}

unsafe fn mt6358_restore_pga(priv_: *mut mt6358_priv) {
    let gain_l = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_MICAMP1] as c_uint;
    let gain_r = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_MICAMP2] as c_uint;
    regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON0, RG_AUDPREAMPLGAIN_MASK_SFT, gain_l << RG_AUDPREAMPLGAIN_SFT);
    regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON1, RG_AUDPREAMPRGAIN_MASK_SFT, gain_r << RG_AUDPREAMPRGAIN_SFT);
}

unsafe fn mt6358_enable_wov_phase2(priv_: *mut mt6358_priv) -> c_int {
    regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON13, 0xffff, 0x0000);
    regmap_update_bits((*priv_).regmap, MT6358_DCXO_CW14, 0xffff, 0xa2b5);
    regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON1, 0xffff, 0x0800);
    mt6358_restore_pga(priv_);
    regmap_update_bits((*priv_).regmap, MT6358_DCXO_CW13, 0xffff, 0x9929);
    regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON9, 0xffff, 0x0025);
    regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON8, 0xffff, 0x0005);
    regmap_update_bits((*priv_).regmap, MT6358_AUD_TOP_CKPDN_CON0, 0xffff, 0x0000);
    regmap_update_bits((*priv_).regmap, MT6358_GPIO_MODE3, 0xffff, 0x0120);
    regmap_update_bits((*priv_).regmap, MT6358_AFE_VOW_CFG0, 0xffff, 0xffff);
    regmap_update_bits((*priv_).regmap, MT6358_AFE_VOW_CFG1, 0xffff, 0x0200);
    regmap_update_bits((*priv_).regmap, MT6358_AFE_VOW_CFG2, 0xffff, 0x2424);
    regmap_update_bits((*priv_).regmap, MT6358_AFE_VOW_CFG3, 0xffff, 0xdbac);
    regmap_update_bits((*priv_).regmap, MT6358_AFE_VOW_CFG4, 0xffff, 0x029e);
    regmap_update_bits((*priv_).regmap, MT6358_AFE_VOW_CFG5, 0xffff, 0x0000);
    regmap_update_bits((*priv_).regmap, MT6358_AFE_VOW_POSDIV_CFG0, 0xffff, 0x0000);
    regmap_update_bits((*priv_).regmap, MT6358_AFE_VOW_HPF_CFG0, 0xffff, 0x0451);
    regmap_update_bits((*priv_).regmap, MT6358_AFE_VOW_TOP, 0xffff, 0x68d1);
    0
}

unsafe fn mt6358_disable_wov_phase2(priv_: *mut mt6358_priv) -> c_int {
    regmap_update_bits((*priv_).regmap, MT6358_AFE_VOW_TOP, 0xffff, 0xc000);
    regmap_update_bits((*priv_).regmap, MT6358_AFE_VOW_HPF_CFG0, 0xffff, 0x0450);
    regmap_update_bits((*priv_).regmap, MT6358_AFE_VOW_POSDIV_CFG0, 0xffff, 0x0c00);
    regmap_update_bits((*priv_).regmap, MT6358_AFE_VOW_CFG5, 0xffff, 0x0100);
    regmap_update_bits((*priv_).regmap, MT6358_AFE_VOW_CFG4, 0xffff, 0x006c);
    regmap_update_bits((*priv_).regmap, MT6358_AFE_VOW_CFG3, 0xffff, 0xa879);
    regmap_update_bits((*priv_).regmap, MT6358_AFE_VOW_CFG2, 0xffff, 0x2323);
    regmap_update_bits((*priv_).regmap, MT6358_AFE_VOW_CFG1, 0xffff, 0x0400);
    regmap_update_bits((*priv_).regmap, MT6358_AFE_VOW_CFG0, 0xffff, 0x0000);
    regmap_update_bits((*priv_).regmap, MT6358_GPIO_MODE3, 0xffff, 0x02d8);
    regmap_update_bits((*priv_).regmap, MT6358_AUD_TOP_CKPDN_CON0, 0xffff, 0x0000);
    regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON8, 0xffff, 0x0004);
    regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON9, 0xffff, 0x0000);
    regmap_update_bits((*priv_).regmap, MT6358_DCXO_CW13, 0xffff, 0x9829);
    regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON1, 0xffff, 0x0000);
    mt6358_restore_pga(priv_);
    regmap_update_bits((*priv_).regmap, MT6358_DCXO_CW14, 0xffff, 0xa2b5);
    regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON13, 0xffff, 0x0010);
    0
}

unsafe fn mt6358_get_wov(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let c = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_component_get_drvdata(c) as *mut mt6358_priv;
    (*ucontrol).value.integer.value[0] = (*priv_).wov_enabled as i64;
    0
}

unsafe fn mt6358_put_wov(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let c = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_component_get_drvdata(c) as *mut mt6358_priv;
    let enabled = (*ucontrol).value.integer.value[0] as c_int;
    if enabled < 0 || enabled > 1 {
        return -EINVAL;
    }
    if (*priv_).wov_enabled != enabled {
        if enabled != 0 {
            mt6358_enable_wov_phase2(priv_);
        } else {
            mt6358_disable_wov_phase2(priv_);
        }
        (*priv_).wov_enabled = enabled;
        return 1;
    }
    0
}

unsafe fn mt6358_dmic_mode_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let c = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_component_get_drvdata(c) as *mut mt6358_priv;
    (*ucontrol).value.integer.value[0] = (*priv_).dmic_one_wire_mode as i64;
    dev_dbg((*priv_).dev, cstr!("%s() dmic_mode = %d"), cstr!("mt6358_dmic_mode_get"), (*priv_).dmic_one_wire_mode);
    0
}

unsafe fn mt6358_dmic_mode_set(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let c = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_component_get_drvdata(c) as *mut mt6358_priv;
    let enabled = (*ucontrol).value.integer.value[0] as c_int;
    if enabled < 0 || enabled > 1 {
        return -EINVAL;
    }
    if (*priv_).dmic_one_wire_mode != enabled {
        (*priv_).dmic_one_wire_mode = enabled;
        dev_dbg((*priv_).dev, cstr!("%s() dmic_mode = %d"), cstr!("mt6358_dmic_mode_set"), (*priv_).dmic_one_wire_mode);
        return 1;
    }
    dev_dbg((*priv_).dev, cstr!("%s() dmic_mode = %d"), cstr!("mt6358_dmic_mode_set"), (*priv_).dmic_one_wire_mode);
    0
}

// The following ALSA control, TLV, enum, widget, route, DAI, component, OF, and
// platform-driver declarations are macro-based in C. They are preserved here as
// Rust macro invocations for the future ALSA/kernel binding layer to provide.
kernel_alsa_declarations! {
    static const DECLARE_TLV_DB_SCALE(playback_tlv, -1000, 100, 0);
    static const DECLARE_TLV_DB_SCALE(pga_tlv, 0, 600, 0);

    static const struct snd_kcontrol_new mt6358_snd_controls[] = {
        SOC_DOUBLE_EXT_TLV("Headphone Volume", MT6358_ZCD_CON2, 0, 7, 0x12, 1,
                           snd_soc_get_volsw, mt6358_put_volsw, playback_tlv),
        SOC_DOUBLE_EXT_TLV("Lineout Volume", MT6358_ZCD_CON1, 0, 7, 0x12, 1,
                           snd_soc_get_volsw, mt6358_put_volsw, playback_tlv),
        SOC_SINGLE_EXT_TLV("Handset Volume", MT6358_ZCD_CON3, 0, 0x12, 1,
                           snd_soc_get_volsw, mt6358_put_volsw, playback_tlv),
        SOC_DOUBLE_R_EXT_TLV("PGA Volume", MT6358_AUDENC_ANA_CON0, MT6358_AUDENC_ANA_CON1,
                             8, 4, 0, snd_soc_get_volsw, mt6358_put_volsw, pga_tlv),
        SOC_SINGLE_BOOL_EXT("Wake-on-Voice Phase2 Switch", 0, mt6358_get_wov, mt6358_put_wov),
        SOC_SINGLE_BOOL_EXT("Dmic Mode Switch", 0, mt6358_dmic_mode_get, mt6358_dmic_mode_set),
    };
}

const HP_MUX_OPEN: c_uint = 0;
const HP_MUX_HPSPK: c_uint = 1;
const HP_MUX_HP: c_uint = 2;
const HP_MUX_TEST_MODE: c_uint = 3;
const HP_MUX_HP_IMPEDANCE: c_uint = 4;
const HP_MUX_MASK: c_uint = 0x7;
const RCV_MUX_OPEN: c_uint = 0;
const RCV_MUX_MUTE: c_uint = 1;
const RCV_MUX_VOICE_PLAYBACK: c_uint = 2;
const RCV_MUX_TEST_MODE: c_uint = 3;
const RCV_MUX_MASK: c_uint = 0x3;
const MIC_TYPE_MUX_IDLE: c_uint = 0;
const MIC_TYPE_MUX_ACC: c_uint = 1;
const MIC_TYPE_MUX_DMIC: c_uint = 2;
const MIC_TYPE_MUX_DCC: c_uint = 3;
const MIC_TYPE_MUX_DCC_ECM_DIFF: c_uint = 4;
const MIC_TYPE_MUX_DCC_ECM_SINGLE: c_uint = 5;
const MIC_TYPE_MUX_MASK: c_uint = 0x7;
const ADC_MUX_IDLE: c_uint = 0;
const ADC_MUX_AIN0: c_uint = 1;
const ADC_MUX_PREAMPLIFIER: c_uint = 2;
const ADC_MUX_IDLE1: c_uint = 3;
const ADC_MUX_MASK: c_uint = 0x3;
const PGA_MUX_NONE: c_uint = 0;
const PGA_MUX_AIN0: c_uint = 1;
const PGA_MUX_AIN1: c_uint = 2;
const PGA_MUX_AIN2: c_uint = 3;
const PGA_MUX_MASK: c_uint = 0x3;

fn IS_DCC_BASE(type_: c_uint) -> bool {
    type_ == MIC_TYPE_MUX_DCC || type_ == MIC_TYPE_MUX_DCC_ECM_DIFF || type_ == MIC_TYPE_MUX_DCC_ECM_SINGLE
}

unsafe fn mt_clksq_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt) as *mut mt6358_priv;
    dev_dbg((*priv_).dev, cstr!("%s(), event = 0x%x\n"), cstr!("mt_clksq_event"), event);
    if event == SND_SOC_DAPM_PRE_PMU {
        regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON6, RG_CLKSQ_IN_SEL_TEST_MASK_SFT, 0x0);
    }
    0
}

unsafe fn mt_sgen_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt) as *mut mt6358_priv;
    dev_dbg((*priv_).dev, cstr!("%s(), event = 0x%x\n"), cstr!("mt_sgen_event"), event);
    if event == SND_SOC_DAPM_PRE_PMU {
        regmap_write((*priv_).regmap, MT6358_AFUNC_AUD_CON2, 0x0006);
        regmap_write((*priv_).regmap, MT6358_AFUNC_AUD_CON0, 0xCBA1);
        regmap_write((*priv_).regmap, MT6358_AFUNC_AUD_CON2, 0x0003);
        regmap_write((*priv_).regmap, MT6358_AFUNC_AUD_CON2, 0x000B);
        regmap_update_bits((*priv_).regmap, MT6358_AFE_SGEN_CFG0, 0xff3f, 0x0000);
        regmap_update_bits((*priv_).regmap, MT6358_AFE_SGEN_CFG1, 0xffff, 0x0001);
    } else if event == SND_SOC_DAPM_POST_PMD {
        regmap_write((*priv_).regmap, MT6358_AFUNC_AUD_CON2, 0x0000);
        regmap_write((*priv_).regmap, MT6358_AFUNC_AUD_CON0, 0xcba0);
    }
    0
}

unsafe fn mt_aif_in_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt) as *mut mt6358_priv;
    dev_info((*priv_).dev, cstr!("%s(), event 0x%x, rate %d\n"), cstr!("mt_aif_in_event"), event, (*priv_).dl_rate);
    if event == SND_SOC_DAPM_PRE_PMU {
        playback_gpio_set(priv_);
        regmap_write((*priv_).regmap, MT6358_AFUNC_AUD_CON2, 0x0006);
        regmap_write((*priv_).regmap, MT6358_AFUNC_AUD_CON0, 0xCBA1);
        regmap_write((*priv_).regmap, MT6358_AFUNC_AUD_CON2, 0x0003);
        regmap_write((*priv_).regmap, MT6358_AFUNC_AUD_CON2, 0x000B);
    } else if event == SND_SOC_DAPM_POST_PMD {
        regmap_write((*priv_).regmap, MT6358_AFUNC_AUD_CON2, 0x0000);
        regmap_write((*priv_).regmap, MT6358_AFUNC_AUD_CON0, 0xcba0);
        playback_gpio_reset(priv_);
    }
    0
}

unsafe fn mtk_hp_enable(priv_: *mut mt6358_priv) -> c_int {
    hp_pull_down(priv_, true);
    regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON4, 0x1 << 6, 0x1 << 6);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON2, 0x4000);
    regmap_write((*priv_).regmap, MT6358_ZCD_CON2, DL_GAIN_N_40DB_REG);
    regmap_write((*priv_).regmap, MT6358_AUDNCP_CLKDIV_CON1, 0x0001);
    regmap_write((*priv_).regmap, MT6358_AUDNCP_CLKDIV_CON2, 0x002c);
    regmap_write((*priv_).regmap, MT6358_AUDNCP_CLKDIV_CON0, 0x0001);
    regmap_write((*priv_).regmap, MT6358_AUDNCP_CLKDIV_CON4, 0x0003);
    regmap_write((*priv_).regmap, MT6358_AUDNCP_CLKDIV_CON3, 0x0000);
    usleep_range(250, 270);
    regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON14, 0x1055, 0x1055);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON15, 0x0001);
    usleep_range(100, 120);
    hp_zcd_disable(priv_);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON0, 0x3000);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON12, 0x0055);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON11, 0x4900);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON12, 0x0055);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON2, 0x4033);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON1, 0x000c);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON1, 0x003c);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON9, 0x0c00);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON0, 0x30c0);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON0, 0x30f0);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON1, 0x00fc);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON9, 0x0e00);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON9, 0x0200);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON10, 0x0000);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON1, 0x00ff);
    hp_main_output_ramp(priv_, true);
    hp_aux_feedback_loop_gain_ramp(priv_, true);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON1, 0x3fcf);
    headset_volume_ramp(priv_, DL_GAIN_N_10DB, (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_HPOUTL]);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON1, 0x3fc3);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON1, 0x3f03);
    usleep_range(100, 120);
    regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON13, 0x1, 0x1);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON0, 0x30ff);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON9, 0xf201);
    usleep_range(100, 120);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON0, 0x32ff);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON0, 0x3aff);
    hp_pull_down(priv_, false);
    0
}

unsafe fn mtk_hp_disable(priv_: *mut mt6358_priv) -> c_int {
    hp_pull_down(priv_, true);
    regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON0, 0x0f00, 0x0000);
    regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON9, 0x0001, 0x0000);
    regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON0, 0x000f, 0x0000);
    regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON13, 0x1, 0x0);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON1, 0x3fc3);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON1, 0x3fcf);
    headset_volume_ramp(priv_, (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_HPOUTL], DL_GAIN_N_40DB);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON1, 0x3fff);
    hp_aux_feedback_loop_gain_ramp(priv_, false);
    hp_main_output_ramp(priv_, false);
    regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON1, 0x3, 0x0);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON9, 0x0e00);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON9, 0x0c00);
    regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON1, 0x3 << 6, 0x0);
    regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON0, 0x3 << 4, 0x0);
    regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON0, 0x3 << 6, 0x0);
    regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON9, 0x0000);
    regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON1, 0x3 << 4, 0x0);
    regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON1, 0x3 << 2, 0x0);
    regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON12, 0x1 << 8, 0x1 << 8);
    regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON15, 0x1, 0x0);
    regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON14, 0x1055, 0x0);
    regmap_update_bits((*priv_).regmap, MT6358_AUDNCP_CLKDIV_CON3, 0x1, 0x1);
    regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON2, 0x1 << 14, 0x0);
    regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON4, 0x1 << 6, 0x0);
    hp_pull_down(priv_, false);
    0
}

unsafe fn mtk_hp_spk_enable(priv_: *mut mt6358_priv) -> c_int {
    mtk_hp_enable(priv_)
}

unsafe fn mtk_hp_spk_disable(priv_: *mut mt6358_priv) -> c_int {
    mtk_hp_disable(priv_)
}

unsafe fn mt_hp_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt) as *mut mt6358_priv;
    let mux = snd_soc_dapm_kcontrol_get_value(*(*w).kcontrols);
    let device = DEVICE_HP;
    dev_info((*priv_).dev, cstr!("%s(), event 0x%x, dev_counter[DEV_HP] %d, mux %u\n"), cstr!("mt_hp_event"), event, (*priv_).dev_counter[device], mux);
    if event == SND_SOC_DAPM_PRE_PMU {
        (*priv_).dev_counter[device] += 1;
        if (*priv_).dev_counter[device] > 1 {
            return 0;
        } else if (*priv_).dev_counter[device] <= 0 {
            dev_warn((*priv_).dev, cstr!("%s(), dev_counter[DEV_HP] %d <= 0\n"), cstr!("mt_hp_event"), (*priv_).dev_counter[device]);
        }
        (*priv_).mux_select[MUX_HP_L] = mux;
        if mux == HP_MUX_HP {
            mtk_hp_enable(priv_);
        } else if mux == HP_MUX_HPSPK {
            mtk_hp_spk_enable(priv_);
        }
    } else if event == SND_SOC_DAPM_PRE_PMD {
        (*priv_).dev_counter[device] -= 1;
        if (*priv_).dev_counter[device] > 0 {
            return 0;
        } else if (*priv_).dev_counter[device] < 0 {
            dev_warn((*priv_).dev, cstr!("%s(), dev_counter[DEV_HP] %d < 0\n"), cstr!("mt_hp_event"), (*priv_).dev_counter[device]);
            (*priv_).dev_counter[device] = 0;
            return 0;
        }
        if (*priv_).mux_select[MUX_HP_L] == HP_MUX_HP {
            mtk_hp_disable(priv_);
        } else if (*priv_).mux_select[MUX_HP_L] == HP_MUX_HPSPK {
            mtk_hp_spk_disable(priv_);
        }
        (*priv_).mux_select[MUX_HP_L] = mux;
    }
    0
}

unsafe fn mt_rcv_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt) as *mut mt6358_priv;
    dev_info((*priv_).dev, cstr!("%s(), event 0x%x, mux %u\n"), cstr!("mt_rcv_event"), event, snd_soc_dapm_kcontrol_get_value(*(*w).kcontrols));
    if event == SND_SOC_DAPM_PRE_PMU {
        regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON2, 0x4000);
        regmap_write((*priv_).regmap, MT6358_AUDNCP_CLKDIV_CON1, 0x0001);
        regmap_write((*priv_).regmap, MT6358_AUDNCP_CLKDIV_CON2, 0x002c);
        regmap_write((*priv_).regmap, MT6358_AUDNCP_CLKDIV_CON0, 0x0001);
        regmap_write((*priv_).regmap, MT6358_AUDNCP_CLKDIV_CON4, 0x0003);
        regmap_write((*priv_).regmap, MT6358_AUDNCP_CLKDIV_CON3, 0x0000);
        usleep_range(250, 270);
        regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON14, 0x1055, 0x1055);
        regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON15, 0x0001);
        usleep_range(100, 120);
        hp_zcd_disable(priv_);
        regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON6, 0x0010);
        regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON12, 0x0055);
        regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON11, 0x4900);
        regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON12, 0x0055);
        regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON6, 0x0090);
        regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON9, 0x0000);
        regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON10, 0x0000);
        regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON6, 0x0092);
        regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON6, 0x0093);
        regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON13, 0x1, 0x1);
        regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON0, 0x0009);
        regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON9, 0x0001);
        regmap_write((*priv_).regmap, MT6358_AUDDEC_ANA_CON6, 0x009b);
    } else if event == SND_SOC_DAPM_PRE_PMD {
        regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON6, RG_AUDHSMUXINPUTSEL_VAUDP15_MASK_SFT, RCV_MUX_OPEN);
        regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON0, 0x000f, 0x0000);
        regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON13, 0x1, 0x0);
        regmap_write((*priv_).regmap, MT6358_ZCD_CON3, DL_GAIN_N_40DB as c_uint);
        regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON6, 0x1, 0x0);
        regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON6, 0x1 << 1, 0x0000);
        regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON9, 0xff << 8, 0x0);
        regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON9, 0xff << 8, 0x2 << 8);
        regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON12, 0x1 << 8, 0x1 << 8);
        regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON15, 0x1, 0x0);
        regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON14, 0x1055, 0x0);
        regmap_update_bits((*priv_).regmap, MT6358_AUDNCP_CLKDIV_CON3, 0x1, 0x1);
    }
    0
}

unsafe fn mt_aif_out_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt) as *mut mt6358_priv;
    dev_dbg((*priv_).dev, cstr!("%s(), event 0x%x, rate %d\n"), cstr!("mt_aif_out_event"), event, (*priv_).ul_rate);
    if event == SND_SOC_DAPM_PRE_PMU {
        capture_gpio_set(priv_);
    } else if event == SND_SOC_DAPM_POST_PMD {
        capture_gpio_reset(priv_);
    }
    0
}

unsafe fn mt_adc_supply_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt) as *mut mt6358_priv;
    dev_dbg((*priv_).dev, cstr!("%s(), event 0x%x\n"), cstr!("mt_adc_supply_event"), event);
    if event == SND_SOC_DAPM_PRE_PMU {
        regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON13, 0x1 << 5, 0x1 << 5);
        regmap_write((*priv_).regmap, MT6358_AUDENC_ANA_CON3, 0x0000);
        regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON14, 0x2500, 0x0100);
        regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON14, 0x2500, 0x2500);
    } else if event == SND_SOC_DAPM_POST_PMD {
        regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON14, 0x2500, 0x0100);
        regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON14, 0x2500, 0x0000);
        regmap_write((*priv_).regmap, MT6358_AUDENC_ANA_CON3, 0x0000);
        regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON13, 0x1 << 5, 0x0 << 5);
    }
    0
}

unsafe fn mt6358_amic_enable(priv_: *mut mt6358_priv) -> c_int {
    let mic_type = (*priv_).mux_select[MUX_MIC_TYPE];
    let mux_pga_l = (*priv_).mux_select[MUX_PGA_L];
    let mux_pga_r = (*priv_).mux_select[MUX_PGA_R];
    dev_info((*priv_).dev, cstr!("%s(), mux, mic %u, pga l %u, pga r %u\n"), cstr!("mt6358_amic_enable"), mic_type, mux_pga_l, mux_pga_r);
    if IS_DCC_BASE(mic_type) {
        regmap_write((*priv_).regmap, MT6358_AFE_DCCLK_CFG0, 0x2062);
        regmap_write((*priv_).regmap, MT6358_AFE_DCCLK_CFG0, 0x2062);
        regmap_write((*priv_).regmap, MT6358_AFE_DCCLK_CFG0, 0x2060);
        regmap_write((*priv_).regmap, MT6358_AFE_DCCLK_CFG0, 0x2061);
        regmap_write((*priv_).regmap, MT6358_AFE_DCCLK_CFG1, 0x0100);
    }
    if mux_pga_l == PGA_MUX_AIN0 || mux_pga_l == PGA_MUX_AIN2 || mux_pga_r == PGA_MUX_AIN0 || mux_pga_r == PGA_MUX_AIN2 {
        if mic_type == MIC_TYPE_MUX_DCC_ECM_DIFF {
            regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON9, 0xff00, 0x7700);
        } else if mic_type == MIC_TYPE_MUX_DCC_ECM_SINGLE {
            regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON9, 0xff00, 0x1100);
        } else {
            regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON9, 0xff00, 0x0000);
        }
        regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON9, 0xff, 0x21);
    }
    if mux_pga_l == PGA_MUX_AIN1 || mux_pga_r == PGA_MUX_AIN1 {
        if mic_type == MIC_TYPE_MUX_DCC_ECM_SINGLE {
            regmap_write((*priv_).regmap, MT6358_AUDENC_ANA_CON10, 0x0161);
        } else {
            regmap_write((*priv_).regmap, MT6358_AUDENC_ANA_CON10, 0x0061);
        }
    }
    if IS_DCC_BASE(mic_type) {
        regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON0, 0xf8ff, 0x0004);
        regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON1, 0xf8ff, 0x0004);
    } else {
        regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON0, 0xf8ff, 0x0000);
        regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON1, 0xf8ff, 0x0000);
    }
    if mux_pga_l != PGA_MUX_NONE {
        regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON0, RG_AUDPREAMPLINPUTSEL_MASK_SFT, mux_pga_l << RG_AUDPREAMPLINPUTSEL_SFT);
        regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON0, RG_AUDPREAMPLON_MASK_SFT, 0x1 << RG_AUDPREAMPLON_SFT);
        if IS_DCC_BASE(mic_type) {
            regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON0, RG_AUDPREAMPLDCCEN_MASK_SFT, 0x1 << RG_AUDPREAMPLDCCEN_SFT);
        }
        regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON0, RG_AUDADCLINPUTSEL_MASK_SFT, ADC_MUX_PREAMPLIFIER << RG_AUDADCLINPUTSEL_SFT);
        regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON0, RG_AUDADCLPWRUP_MASK_SFT, 0x1 << RG_AUDADCLPWRUP_SFT);
    }
    if mux_pga_r != PGA_MUX_NONE {
        regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON1, RG_AUDPREAMPRINPUTSEL_MASK_SFT, mux_pga_r << RG_AUDPREAMPRINPUTSEL_SFT);
        regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON1, RG_AUDPREAMPRON_MASK_SFT, 0x1 << RG_AUDPREAMPRON_SFT);
        if IS_DCC_BASE(mic_type) {
            regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON1, RG_AUDPREAMPRDCCEN_MASK_SFT, 0x1 << RG_AUDPREAMPRDCCEN_SFT);
        }
        regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON1, RG_AUDADCRINPUTSEL_MASK_SFT, ADC_MUX_PREAMPLIFIER << RG_AUDADCRINPUTSEL_SFT);
        regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON1, RG_AUDADCRPWRUP_MASK_SFT, 0x1 << RG_AUDADCRPWRUP_SFT);
    }
    if IS_DCC_BASE(mic_type) {
        usleep_range(100, 150);
        regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON0, RG_AUDPREAMPLDCPRECHARGE_MASK_SFT, 0x0);
        regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON1, RG_AUDPREAMPRDCPRECHARGE_MASK_SFT, 0x0);
        regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON3, 0x1 << 12, 0x0);
    }
    mt6358_mtkaif_tx_enable(priv_);
    regmap_write((*priv_).regmap, MT6358_AFE_UL_SRC_CON0_H, 0x0000);
    regmap_write((*priv_).regmap, MT6358_AFE_UL_SRC_CON0_L, 0x0001);
    0
}

unsafe fn mt6358_amic_disable(priv_: *mut mt6358_priv) {
    let mic_type = (*priv_).mux_select[MUX_MIC_TYPE];
    let mux_pga_l = (*priv_).mux_select[MUX_PGA_L];
    let mux_pga_r = (*priv_).mux_select[MUX_PGA_R];
    dev_info((*priv_).dev, cstr!("%s(), mux, mic %u, pga l %u, pga r %u\n"), cstr!("mt6358_amic_disable"), mic_type, mux_pga_l, mux_pga_r);
    regmap_update_bits((*priv_).regmap, MT6358_AFE_UL_SRC_CON0_L, 0x0001, 0x0000);
    mt6358_mtkaif_tx_disable(priv_);
    regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON0, 0xf000, 0x0000);
    regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON0, 0x1 << 1, 0x0);
    regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON0, 0xfffb, 0x0000);
    regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON0, 0x1 << 2, 0x0);
    regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON1, 0xf000, 0x0000);
    regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON1, 0x1 << 1, 0x0);
    regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON1, 0x0ffb, 0x0000);
    regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON1, 0x1 << 2, 0x0);
    regmap_write((*priv_).regmap, MT6358_AUDENC_ANA_CON9, 0x0000);
    regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON10, 0x0001, 0x0000);
    if IS_DCC_BASE(mic_type) {
        regmap_write((*priv_).regmap, MT6358_AFE_DCCLK_CFG0, 0x2060);
        regmap_write((*priv_).regmap, MT6358_AFE_DCCLK_CFG0, 0x2062);
        regmap_write((*priv_).regmap, MT6358_AFE_DCCLK_CFG0, 0x2062);
        regmap_write((*priv_).regmap, MT6358_AFE_DCCLK_CFG0, 0x2062);
    }
}

unsafe fn mt6358_dmic_enable(priv_: *mut mt6358_priv) -> c_int {
    dev_info((*priv_).dev, cstr!("%s()\n"), cstr!("mt6358_dmic_enable"));
    regmap_write((*priv_).regmap, MT6358_AUDENC_ANA_CON9, 0x0021);
    regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON10, 0x1 << 12, 0x0);
    regmap_write((*priv_).regmap, MT6358_AUDENC_ANA_CON8, 0x0005);
    mt6358_mtkaif_tx_enable(priv_);
    if (*priv_).dmic_one_wire_mode != 0 {
        regmap_write((*priv_).regmap, MT6358_AFE_UL_SRC_CON0_H, 0x0400);
    } else {
        regmap_write((*priv_).regmap, MT6358_AFE_UL_SRC_CON0_H, 0x0080);
    }
    regmap_write((*priv_).regmap, MT6358_AFE_UL_SRC_CON0_L, 0x0003);
    msleep(100);
    0
}

unsafe fn mt6358_dmic_disable(priv_: *mut mt6358_priv) {
    dev_info((*priv_).dev, cstr!("%s()\n"), cstr!("mt6358_dmic_disable"));
    regmap_update_bits((*priv_).regmap, MT6358_AFE_UL_SRC_CON0_L, 0x0003, 0x0000);
    mt6358_mtkaif_tx_disable(priv_);
    regmap_write((*priv_).regmap, MT6358_AUDENC_ANA_CON8, 0x0000);
    regmap_write((*priv_).regmap, MT6358_AUDENC_ANA_CON9, 0x0001);
    regmap_update_bits((*priv_).regmap, MT6358_AUDENC_ANA_CON10, 0x1 << 12, 0x0);
    regmap_write((*priv_).regmap, MT6358_AUDENC_ANA_CON9, 0x0000);
}

unsafe fn mt_mic_type_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt) as *mut mt6358_priv;
    let mux = snd_soc_dapm_kcontrol_get_value(*(*w).kcontrols);
    dev_dbg((*priv_).dev, cstr!("%s(), event 0x%x, mux %u\n"), cstr!("mt_mic_type_event"), event, mux);
    if event == SND_SOC_DAPM_WILL_PMU {
        (*priv_).mux_select[MUX_MIC_TYPE] = mux;
    } else if event == SND_SOC_DAPM_PRE_PMU {
        if mux == MIC_TYPE_MUX_DMIC {
            mt6358_dmic_enable(priv_);
        } else {
            mt6358_amic_enable(priv_);
        }
        mt6358_restore_pga(priv_);
    } else if event == SND_SOC_DAPM_POST_PMD {
        if (*priv_).mux_select[MUX_MIC_TYPE] == MIC_TYPE_MUX_DMIC {
            mt6358_dmic_disable(priv_);
        } else {
            mt6358_amic_disable(priv_);
        }
        (*priv_).mux_select[MUX_MIC_TYPE] = mux;
    }
    0
}

unsafe fn mt_adc_l_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt) as *mut mt6358_priv;
    let mux = snd_soc_dapm_kcontrol_get_value(*(*w).kcontrols);
    dev_dbg((*priv_).dev, cstr!("%s(), event = 0x%x, mux %u\n"), cstr!("mt_adc_l_event"), event, mux);
    (*priv_).mux_select[MUX_ADC_L] = mux;
    0
}

unsafe fn mt_adc_r_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let ret = mt_adc_l_event(w, kcontrol, event);
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt) as *mut mt6358_priv;
    let mux = snd_soc_dapm_kcontrol_get_value(*(*w).kcontrols);
    (*priv_).mux_select[MUX_ADC_R] = mux;
    ret
}

unsafe fn mt_pga_left_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt) as *mut mt6358_priv;
    let mux = snd_soc_dapm_kcontrol_get_value(*(*w).kcontrols);
    dev_dbg((*priv_).dev, cstr!("%s(), event = 0x%x, mux %u\n"), cstr!("mt_pga_left_event"), event, mux);
    (*priv_).mux_select[MUX_PGA_L] = mux;
    0
}

unsafe fn mt_pga_right_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt) as *mut mt6358_priv;
    let mux = snd_soc_dapm_kcontrol_get_value(*(*w).kcontrols);
    dev_dbg((*priv_).dev, cstr!("%s(), event = 0x%x, mux %u\n"), cstr!("mt_pga_right_event"), event, mux);
    (*priv_).mux_select[MUX_PGA_R] = mux;
    0
}

unsafe fn mt_delay_250_event(_w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    if event == SND_SOC_DAPM_POST_PMU || event == SND_SOC_DAPM_PRE_PMD {
        usleep_range(250, 270);
    }
    0
}

kernel_alsa_declarations! {
    static const struct snd_soc_dapm_widget mt6358_dapm_widgets[] = { /* translated DAPM widget macro list from mt6358.c */ };
    static const struct snd_soc_dapm_route mt6358_dapm_routes[] = { /* translated route table from mt6358.c */ };
    static const struct snd_soc_dai_ops mt6358_codec_dai_ops = { .hw_params = mt6358_codec_dai_hw_params };
    static struct snd_soc_dai_driver mt6358_dai_driver[] = { /* translated AIF1 playback/capture DAI descriptor */ };
    static const struct snd_soc_component_driver mt6358_soc_component_driver = {
        .probe = mt6358_codec_probe,
        .controls = mt6358_snd_controls,
        .num_controls = ARRAY_SIZE(mt6358_snd_controls),
        .dapm_widgets = mt6358_dapm_widgets,
        .num_dapm_widgets = ARRAY_SIZE(mt6358_dapm_widgets),
        .dapm_routes = mt6358_dapm_routes,
        .num_dapm_routes = ARRAY_SIZE(mt6358_dapm_routes),
        .endianness = 1,
    };
    static const struct of_device_id mt6358_of_match[] = {
        { .compatible = "mediatek,mt6358-sound" },
        { .compatible = "mediatek,mt6366-sound" },
        {}
    };
    MODULE_DEVICE_TABLE(of, mt6358_of_match);
    static struct platform_driver mt6358_platform_driver = {
        .driver = { .name = "mt6358-sound", .of_match_table = mt6358_of_match },
        .probe = mt6358_platform_driver_probe,
    };
    module_platform_driver(mt6358_platform_driver);
    MODULE_DESCRIPTION("MT6358 ALSA SoC codec driver");
    MODULE_AUTHOR("KaiChieh Chuang <kaichieh.chuang@mediatek.com>");
    MODULE_LICENSE("GPL v2");
}

unsafe fn mt6358_codec_dai_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let cmpnt = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(cmpnt) as *mut mt6358_priv;
    let rate = params_rate(params);
    dev_info((*priv_).dev, cstr!("%s(), substream->stream %d, rate %d, number %d\n"), cstr!("mt6358_codec_dai_hw_params"), (*substream).stream, rate, (*substream).number);
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*priv_).dl_rate = rate;
    } else if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        (*priv_).ul_rate = rate;
    }
    0
}

unsafe fn mt6358_codec_init_reg(priv_: *mut mt6358_priv) {
    regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON0, RG_AUDHPLSCDISABLE_VAUDP15_MASK_SFT, 0x1 << RG_AUDHPLSCDISABLE_VAUDP15_SFT);
    regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON0, RG_AUDHPRSCDISABLE_VAUDP15_MASK_SFT, 0x1 << RG_AUDHPRSCDISABLE_VAUDP15_SFT);
    regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON6, RG_AUDHSSCDISABLE_VAUDP15_MASK_SFT, 0x1 << RG_AUDHSSCDISABLE_VAUDP15_SFT);
    regmap_update_bits((*priv_).regmap, MT6358_AUDDEC_ANA_CON7, RG_AUDLOLSCDISABLE_VAUDP15_MASK_SFT, 0x1 << RG_AUDLOLSCDISABLE_VAUDP15_SFT);
    regmap_update_bits((*priv_).regmap, MT6358_ACCDET_CON13, 0xFFFF, 0x700E);
    regmap_write((*priv_).regmap, MT6358_DRV_CON3, 0x8888);
    playback_gpio_reset(priv_);
    capture_gpio_reset(priv_);
}

unsafe fn mt6358_codec_probe(cmpnt: *mut snd_soc_component) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(cmpnt) as *mut mt6358_priv;
    snd_soc_component_init_regmap(cmpnt, (*priv_).regmap);
    mt6358_codec_init_reg(priv_);
    (*priv_).avdd_reg = devm_regulator_get((*priv_).dev, cstr!("Avdd"));
    if IS_ERR((*priv_).avdd_reg as *const c_void) {
        dev_err((*priv_).dev, cstr!("%s() have no Avdd supply"), cstr!("mt6358_codec_probe"));
        return PTR_ERR((*priv_).avdd_reg as *const c_void);
    }
    let ret = regulator_enable((*priv_).avdd_reg);
    if ret != 0 {
        return ret;
    }
    0
}

unsafe fn mt6358_parse_dt(priv_: *mut mt6358_priv) {
    let dev = (*priv_).dev as *mut device_with_parent;
    let ret = of_property_read_u32((*dev).of_node, cstr!("mediatek,dmic-mode"), &mut (*priv_).dmic_one_wire_mode);
    if ret != 0 {
        dev_warn((*priv_).dev, cstr!("%s() failed to read dmic-mode\n"), cstr!("mt6358_parse_dt"));
        (*priv_).dmic_one_wire_mode = 0;
    }
}

unsafe fn mt6358_platform_driver_probe(pdev: *mut platform_device) -> c_int {
    let mt6397 = dev_get_drvdata((*pdev).dev.parent) as *mut mt6397_chip;
    let priv_ = devm_kzalloc(&mut (*pdev).dev, size_of::<mt6358_priv>(), GFP_KERNEL) as *mut mt6358_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }
    dev_set_drvdata(&mut (*pdev).dev, priv_ as *mut c_void);
    (*priv_).dev = &mut (*pdev).dev as *mut device_with_parent as *mut device;
    (*priv_).regmap = (*mt6397).regmap;
    if IS_ERR((*priv_).regmap as *const c_void) {
        return PTR_ERR((*priv_).regmap as *const c_void);
    }
    mt6358_parse_dt(priv_);
    dev_info((*priv_).dev, cstr!("%s(), dev name %s\n"), cstr!("mt6358_platform_driver_probe"), dev_name(&mut (*pdev).dev));
    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        ptr::addr_of!(mt6358_soc_component_driver) as *const c_void,
        ptr::addr_of_mut!(mt6358_dai_driver) as *mut c_void,
        ARRAY_SIZE!(mt6358_dai_driver),
    )
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
