// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8904.rs -- WM8904 ALSA SoC Audio driver
 *
 * Rust source-level translation of soc/codecs/wm8904.c.
 *
 * Linux/ASoC/regmap symbols and register constants are external dependencies
 * supplied by the surrounding kernel tree.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type bool_ = bool;
type u16 = u16;
type u32 = u32;
type u64 = u64;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const INT_MAX: c_int = c_int::MAX;

const WM8904_NUM_DCS_CHANNELS: usize = 4;
const WM8904_NUM_SUPPLIES: usize = 5;
const FIXED_FLL_SIZE: u64 = ((1u64 << 16) * 10);

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum wm8904_type {
    WM8904 = 0,
    WM8912 = 1,
}

#[repr(C)]
struct device { _private: [u8; 0] }
#[repr(C)]
struct device_node { _private: [u8; 0] }
#[repr(C)]
struct regmap { _private: [u8; 0] }
#[repr(C)]
struct clk { _private: [u8; 0] }
#[repr(C)]
struct snd_soc_component { dev: *mut device }
#[repr(C)]
struct snd_soc_dai { component: *mut snd_soc_component, dev: *mut device }
#[repr(C)]
struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)]
struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)]
struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)]
struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)]
struct snd_soc_dapm_widget { dapm: *mut snd_soc_dapm_context, shift: c_int }

#[repr(C)]
struct snd_ctl_elem_value_integer { value: [c_long; 128] }
type c_long = isize;
#[repr(C)]
struct snd_ctl_elem_value_enumerated { item: [c_uint; 128] }
#[repr(C)]
union snd_ctl_elem_value_value {
    integer: core::mem::ManuallyDrop<snd_ctl_elem_value_integer>,
    enumerated: core::mem::ManuallyDrop<snd_ctl_elem_value_enumerated>,
}
#[repr(C)]
struct snd_ctl_elem_value { value: snd_ctl_elem_value_value }

#[repr(C)]
struct regulator_bulk_data { supply: *const c_char }
#[repr(C)]
struct reg_default { reg: c_uint, def: c_uint }
#[repr(C)]
struct soc_enum { items: c_uint, texts: *const *const c_char }
#[repr(C)]
struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}
#[repr(C)]
struct snd_soc_dapm_widget_desc { _private: [u8; 0] }
#[repr(C)]
struct snd_soc_dai_ops { _private: [u8; 0] }
#[repr(C)]
struct snd_soc_pcm_stream { _private: [u8; 0] }
#[repr(C)]
struct snd_soc_dai_driver { _private: [u8; 0] }
#[repr(C)]
struct snd_soc_component_driver { _private: [u8; 0] }
#[repr(C)]
struct regmap_config { _private: [u8; 0] }
#[repr(C)]
struct of_device_id { compatible: *const c_char, data: *const c_void }
#[repr(C)]
struct i2c_device_id { name: [c_char; 20], driver_data: c_ulong }
#[repr(C)]
struct device_driver { _private: [u8; 0] }
#[repr(C)]
struct i2c_driver { _private: [u8; 0] }
#[repr(C)]
struct i2c_client_dev {
    of_node: *const device_node,
    platform_data: *mut c_void,
}
#[repr(C)]
struct i2c_client { dev: i2c_client_dev }

#[repr(C)]
struct wm8904_drc_cfg {
    name: *const c_char,
    regs: [u16; WM8904_DRC_REGS as usize],
}
#[repr(C)]
struct wm8904_retune_mobile_cfg {
    name: *const c_char,
    rate: u32,
    regs: [u16; WM8904_EQ_REGS as usize],
}
#[repr(C)]
struct wm8904_pdata {
    in1l_as_dmicdat1: bool,
    in1r_as_dmicdat2: bool,
    gpio_cfg: [u32; WM8904_GPIO_REGS as usize],
    mic_cfg: [u32; WM8904_MIC_REGS as usize],
    drc_cfgs: *mut wm8904_drc_cfg,
    num_drc_cfgs: c_int,
    retune_mobile_cfgs: *mut wm8904_retune_mobile_cfg,
    num_retune_mobile_cfgs: c_int,
}

#[repr(C)]
struct wm8904_priv {
    regmap: *mut regmap,
    mclk: *mut clk,
    devtype: wm8904_type,
    supplies: [regulator_bulk_data; WM8904_NUM_SUPPLIES],
    pdata: *mut wm8904_pdata,
    deemph: c_int,
    /* Platform provided DRC configuration */
    drc_texts: *mut *const c_char,
    drc_cfg: c_int,
    drc_enum: soc_enum,
    /* Platform provided ReTune mobile configuration */
    num_retune_mobile_texts: c_int,
    retune_mobile_texts: *mut *const c_char,
    retune_mobile_cfg: c_int,
    retune_mobile_enum: soc_enum,
    /* FLL setup */
    fll_src: c_int,
    fll_fref: c_int,
    fll_fout: c_int,
    /* Clocking configuration */
    mclk_rate: c_uint,
    sysclk_src: c_int,
    sysclk_rate: c_uint,
    tdm_width: c_int,
    tdm_slots: c_int,
    bclk: c_int,
    fs: c_int,
    /* DC servo configuration - cached offset values */
    dcs_state: [c_int; WM8904_NUM_DCS_CHANNELS],
}

unsafe extern "C" {
    static WM8904_SW_RESET_AND_ID: c_uint;
    static WM8904_REVISION: c_uint;
    static WM8904_BIAS_CONTROL_0: c_uint;
    static WM8904_VMID_CONTROL_0: c_uint;
    static WM8904_MIC_BIAS_CONTROL_0: c_uint;
    static WM8904_MIC_BIAS_CONTROL_1: c_uint;
    static WM8904_ANALOGUE_DAC_0: c_uint;
    static WM8904_MIC_FILTER_CONTROL: c_uint;
    static WM8904_ANALOGUE_ADC_0: c_uint;
    static WM8904_POWER_MANAGEMENT_0: c_uint;
    static WM8904_POWER_MANAGEMENT_2: c_uint;
    static WM8904_POWER_MANAGEMENT_3: c_uint;
    static WM8904_POWER_MANAGEMENT_6: c_uint;
    static WM8904_CLOCK_RATES_0: c_uint;
    static WM8904_CLOCK_RATES_1: c_uint;
    static WM8904_CLOCK_RATES_2: c_uint;
    static WM8904_AUDIO_INTERFACE_0: c_uint;
    static WM8904_AUDIO_INTERFACE_1: c_uint;
    static WM8904_AUDIO_INTERFACE_2: c_uint;
    static WM8904_AUDIO_INTERFACE_3: c_uint;
    static WM8904_DAC_DIGITAL_VOLUME_LEFT: c_uint;
    static WM8904_DAC_DIGITAL_VOLUME_RIGHT: c_uint;
    static WM8904_DAC_DIGITAL_0: c_uint;
    static WM8904_DAC_DIGITAL_1: c_uint;
    static WM8904_ADC_DIGITAL_VOLUME_LEFT: c_uint;
    static WM8904_ADC_DIGITAL_VOLUME_RIGHT: c_uint;
    static WM8904_ADC_DIGITAL_0: c_uint;
    static WM8904_DIGITAL_MICROPHONE_0: c_uint;
    static WM8904_DRC_0: c_uint;
    static WM8904_DRC_1: c_uint;
    static WM8904_DRC_2: c_uint;
    static WM8904_DRC_3: c_uint;
    static WM8904_ANALOGUE_LEFT_INPUT_0: c_uint;
    static WM8904_ANALOGUE_RIGHT_INPUT_0: c_uint;
    static WM8904_ANALOGUE_LEFT_INPUT_1: c_uint;
    static WM8904_ANALOGUE_RIGHT_INPUT_1: c_uint;
    static WM8904_ANALOGUE_OUT1_LEFT: c_uint;
    static WM8904_ANALOGUE_OUT1_RIGHT: c_uint;
    static WM8904_ANALOGUE_OUT2_LEFT: c_uint;
    static WM8904_ANALOGUE_OUT2_RIGHT: c_uint;
    static WM8904_ANALOGUE_OUT12_ZC: c_uint;
    static WM8904_DC_SERVO_0: c_uint;
    static WM8904_DC_SERVO_1: c_uint;
    static WM8904_DC_SERVO_2: c_uint;
    static WM8904_DC_SERVO_4: c_uint;
    static WM8904_DC_SERVO_5: c_uint;
    static WM8904_DC_SERVO_6: c_uint;
    static WM8904_DC_SERVO_7: c_uint;
    static WM8904_DC_SERVO_8: c_uint;
    static WM8904_DC_SERVO_9: c_uint;
    static WM8904_DC_SERVO_READBACK_0: c_uint;
    static WM8904_ANALOGUE_HP_0: c_uint;
    static WM8904_ANALOGUE_LINEOUT_0: c_uint;
    static WM8904_CHARGE_PUMP_0: c_uint;
    static WM8904_CLASS_W_0: c_uint;
    static WM8904_WRITE_SEQUENCER_0: c_uint;
    static WM8904_WRITE_SEQUENCER_1: c_uint;
    static WM8904_WRITE_SEQUENCER_2: c_uint;
    static WM8904_WRITE_SEQUENCER_3: c_uint;
    static WM8904_WRITE_SEQUENCER_4: c_uint;
    static WM8904_FLL_CONTROL_1: c_uint;
    static WM8904_FLL_CONTROL_2: c_uint;
    static WM8904_FLL_CONTROL_3: c_uint;
    static WM8904_FLL_CONTROL_4: c_uint;
    static WM8904_FLL_CONTROL_5: c_uint;
    static WM8904_GPIO_CONTROL_1: c_uint;
    static WM8904_GPIO_CONTROL_2: c_uint;
    static WM8904_GPIO_CONTROL_3: c_uint;
    static WM8904_GPIO_CONTROL_4: c_uint;
    static WM8904_DIGITAL_PULLS: c_uint;
    static WM8904_INTERRUPT_STATUS: c_uint;
    static WM8904_INTERRUPT_STATUS_MASK: c_uint;
    static WM8904_INTERRUPT_POLARITY: c_uint;
    static WM8904_INTERRUPT_DEBOUNCE: c_uint;
    static WM8904_EQ1: c_uint;
    static WM8904_EQ2: c_uint;
    static WM8904_EQ3: c_uint;
    static WM8904_EQ4: c_uint;
    static WM8904_EQ5: c_uint;
    static WM8904_EQ6: c_uint;
    static WM8904_EQ7: c_uint;
    static WM8904_EQ8: c_uint;
    static WM8904_EQ9: c_uint;
    static WM8904_EQ10: c_uint;
    static WM8904_EQ11: c_uint;
    static WM8904_EQ12: c_uint;
    static WM8904_EQ13: c_uint;
    static WM8904_EQ14: c_uint;
    static WM8904_EQ15: c_uint;
    static WM8904_EQ16: c_uint;
    static WM8904_EQ17: c_uint;
    static WM8904_EQ18: c_uint;
    static WM8904_EQ19: c_uint;
    static WM8904_EQ20: c_uint;
    static WM8904_EQ21: c_uint;
    static WM8904_EQ22: c_uint;
    static WM8904_EQ23: c_uint;
    static WM8904_EQ24: c_uint;
    static WM8904_CONTROL_INTERFACE_TEST_1: c_uint;
    static WM8904_ADC_TEST_0: c_uint;
    static WM8904_ANALOGUE_OUTPUT_BIAS_0: c_uint;
    static WM8904_FLL_NCO_TEST_0: c_uint;
    static WM8904_FLL_NCO_TEST_1: c_uint;
    static WM8904_MAX_REGISTER: c_uint;
    static WM8904_SYSCLK_SRC: c_uint;
    static WM8904_FLL_OSC_ENA: c_uint;
    static WM8904_FLL_ENA: c_uint;
    static WM8904_MCLK_DIV: c_uint;
    static WM8904_CLK_SYS_ENA: c_uint;
    static WM8904_DRC_REGS: c_int;
    static WM8904_DRC_ENA: c_uint;
    static WM8904_DRC_DAC_PATH: c_uint;
    static WM8904_EQ_REGS: c_int;
    static WM8904_EQ_ENA: c_uint;
    static WM8904_DEEMPH_SHIFT: c_uint;
    static WM8904_DEEMPH_MASK: c_uint;
    static WM8904_ADC_128_OSR_TST_MODE: c_uint;
    static WM8904_ADC_BIASX1P5: c_uint;
    static WM8904_DCS_ENA_CHAN_0: c_int;
    static WM8904_DCS_ENA_CHAN_1: c_int;
    static WM8904_DCS_ENA_CHAN_2: c_int;
    static WM8904_DCS_ENA_CHAN_3: c_int;
    static WM8904_DCS_TRIG_STARTUP_0_SHIFT: c_uint;
    static WM8904_DCS_CAL_COMPLETE_SHIFT: c_uint;
    static WM8904_DMIC_SRC_SHIFT: c_uint;
    static WM8904_DMIC_ENA_SHIFT: c_uint;
    static WM8904_CLK_SYS_RATE_SHIFT: c_uint;
    static WM8904_SAMPLE_RATE_SHIFT: c_uint;
    static WM8904_DAC_SB_FILT: c_uint;
    static WM8904_AIF_WL_MASK: c_uint;
    static WM8904_BCLK_DIV_MASK: c_uint;
    static WM8904_LRCLK_RATE_MASK: c_uint;
    static WM8904_SAMPLE_RATE_MASK: c_uint;
    static WM8904_CLK_SYS_RATE_MASK: c_uint;
    static WM8904_LRCLK_DIR: c_uint;
    static WM8904_BCLK_DIR: c_uint;
    static WM8904_AIF_LRCLK_INV: c_uint;
    static WM8904_AIF_BCLK_INV: c_uint;
    static WM8904_AIF_FMT_MASK: c_uint;
    static WM8904_AIFADC_TDM: c_int;
    static WM8904_AIFDAC_TDM: c_int;
    static WM8904_AIFADC_TDM_CHAN: c_int;
    static WM8904_AIFDAC_TDM_CHAN: c_int;
    static WM8904_FLL_MCLK: c_int;
    static WM8904_FLL_LRCLK: c_int;
    static WM8904_FLL_BCLK: c_int;
    static WM8904_FLL_FREE_RUNNING: c_int;
    static WM8904_FLL_FRC_NCO: c_int;
    static WM8904_USER_KEY: c_uint;
    static WM8904_FLL_CLK_REF_SRC_MASK: c_uint;
    static WM8904_FLL_FRACN_ENA: c_int;
    static WM8904_FLL_OUTDIV_MASK: c_uint;
    static WM8904_FLL_FRATIO_MASK: c_uint;
    static WM8904_FLL_OUTDIV_SHIFT: c_uint;
    static WM8904_FLL_FRATIO_SHIFT: c_uint;
    static WM8904_FLL_N_MASK: c_uint;
    static WM8904_FLL_N_SHIFT: c_uint;
    static WM8904_FLL_CLK_REF_DIV_MASK: c_uint;
    static WM8904_FLL_CLK_REF_DIV_SHIFT: c_uint;
    static WM8904_CLK_AUTO: c_int;
    static WM8904_CLK_MCLK: c_int;
    static WM8904_CLK_FLL: c_int;
    static WM8904_DAC_MUTE: c_int;
    static WM8904_VMID_RES_MASK: c_uint;
    static WM8904_VMID_RES_SHIFT: c_uint;
    static WM8904_ISEL_MASK: c_uint;
    static WM8904_ISEL_SHIFT: c_uint;
    static WM8904_BIAS_ENA: c_uint;
    static WM8904_VMID_ENA: c_uint;
    static WM8904_GPIO_REGS: c_int;
    static WM8904_MIC_REGS: c_int;
    static WM8904_DMIC_ENA_MASK: c_uint;
    static WM8904_DMIC_SRC_MASK: c_uint;
    static WM8904_ADC_VU: c_uint;
    static WM8904_DAC_VU: c_uint;
    static WM8904_HPOUT_VU: c_uint;
    static WM8904_HPOUTLZC: c_uint;
    static WM8904_HPOUTRZC: c_uint;
    static WM8904_LINEOUT_VU: c_uint;
    static WM8904_LINEOUTLZC: c_uint;
    static WM8904_LINEOUTRZC: c_uint;
    static WM8904_SR_MODE: c_uint;
    static WM8904_CP_DYN_PWR: c_uint;
    static WM8904_POBCTRL: c_uint;
    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static SND_SOC_BIAS_OFF: c_int;
    static SND_SOC_BIAS_ON: c_int;
    static SND_SOC_BIAS_PREPARE: c_int;
    static SND_SOC_BIAS_STANDBY: c_int;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_CBC_CFP: c_uint;
    static SND_SOC_DAIFMT_CBP_CFC: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SNDRV_PCM_RATE_8000_96000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;

    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_get_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_dapm_new_controls(dapm: *mut snd_soc_dapm_context, widgets: *const snd_soc_dapm_widget_desc, num: usize) -> c_int;
    fn snd_soc_dapm_add_routes(dapm: *mut snd_soc_dapm_context, routes: *const snd_soc_dapm_route, num: usize) -> c_int;
    fn snd_soc_add_component_controls(component: *mut snd_soc_component, controls: *const snd_kcontrol_new, num: usize) -> c_int;
    fn snd_soc_calc_bclk(rate: c_int, sample_size: c_int, channels: c_int, slots: c_int) -> c_int;
    fn snd_soc_params_to_bclk(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn regulator_bulk_enable(num: usize, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num: usize, consumers: *mut regulator_bulk_data) -> c_int;
    fn devm_regulator_bulk_get(dev: *mut device, num: usize, consumers: *mut regulator_bulk_data) -> c_int;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn kmalloc_array(n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn krealloc(p: *mut c_void, size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn msleep(msecs: c_uint);
    fn udelay(usecs: c_uint);
    fn of_property_count_strings(np: *const device_node, propname: *const c_char) -> c_int;
    fn of_property_read_u16_index(np: *const device_node, propname: *const c_char, index: c_uint, out_value: *mut u16) -> c_int;
    fn of_property_read_u32_index(np: *const device_node, propname: *const c_char, index: c_uint, out_value: *mut u32) -> c_int;
    fn of_property_read_string_index(np: *const device_node, propname: *const c_char, index: c_int, out_string: *mut *const c_char) -> c_int;
    fn of_property_read_bool(np: *const device_node, propname: *const c_char) -> bool;
    fn of_property_read_u32_array(np: *const device_node, propname: *const c_char, out_values: *mut u32, sz: usize) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn i2c_get_match_data(i2c: *mut i2c_client) -> *const c_void;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn WARN_ON(condition: bool) -> bool;
    fn WARN(condition: c_int, fmt: *const c_char, ...) -> bool;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}
macro_rules! ARRAY_SIZE {
    ($a:expr) => {
        ($a).len()
    };
}

static wm8904_supply_names: [*const c_char; WM8904_NUM_SUPPLIES] = [
    cstr!("DCVDD"), cstr!("DBVDD"), cstr!("AVDD"), cstr!("CPVDD"), cstr!("MICVDD"),
];

static wm8904_reg_defaults: [reg_default; 86] = [
    reg_default { reg: 4, def: 0x0018 }, reg_default { reg: 5, def: 0x0000 },
    reg_default { reg: 6, def: 0x0000 }, reg_default { reg: 7, def: 0x0000 },
    reg_default { reg: 8, def: 0x0001 }, reg_default { reg: 9, def: 0x9696 },
    reg_default { reg: 10, def: 0x0001 }, reg_default { reg: 12, def: 0x0000 },
    reg_default { reg: 14, def: 0x0000 }, reg_default { reg: 15, def: 0x0000 },
    reg_default { reg: 18, def: 0x0000 }, reg_default { reg: 20, def: 0x945E },
    reg_default { reg: 21, def: 0x0C05 }, reg_default { reg: 22, def: 0x0006 },
    reg_default { reg: 24, def: 0x0050 }, reg_default { reg: 25, def: 0x000A },
    reg_default { reg: 26, def: 0x00E4 }, reg_default { reg: 27, def: 0x0040 },
    reg_default { reg: 30, def: 0x00C0 }, reg_default { reg: 31, def: 0x00C0 },
    reg_default { reg: 32, def: 0x0000 }, reg_default { reg: 33, def: 0x0008 },
    reg_default { reg: 36, def: 0x00C0 }, reg_default { reg: 37, def: 0x00C0 },
    reg_default { reg: 38, def: 0x0010 }, reg_default { reg: 39, def: 0x0000 },
    reg_default { reg: 40, def: 0x01AF }, reg_default { reg: 41, def: 0x3248 },
    reg_default { reg: 42, def: 0x0000 }, reg_default { reg: 43, def: 0x0000 },
    reg_default { reg: 44, def: 0x0085 }, reg_default { reg: 45, def: 0x0085 },
    reg_default { reg: 46, def: 0x0044 }, reg_default { reg: 47, def: 0x0044 },
    reg_default { reg: 57, def: 0x002D }, reg_default { reg: 58, def: 0x002D },
    reg_default { reg: 59, def: 0x0039 }, reg_default { reg: 60, def: 0x0039 },
    reg_default { reg: 61, def: 0x0000 }, reg_default { reg: 67, def: 0x0000 },
    reg_default { reg: 69, def: 0xAAAA }, reg_default { reg: 71, def: 0xAAAA },
    reg_default { reg: 72, def: 0xAAAA }, reg_default { reg: 90, def: 0x0000 },
    reg_default { reg: 94, def: 0x0000 }, reg_default { reg: 98, def: 0x0000 },
    reg_default { reg: 104, def: 0x0004 }, reg_default { reg: 108, def: 0x0000 },
    reg_default { reg: 109, def: 0x0000 }, reg_default { reg: 110, def: 0x0000 },
    reg_default { reg: 111, def: 0x0000 }, reg_default { reg: 112, def: 0x0000 },
    reg_default { reg: 116, def: 0x0000 }, reg_default { reg: 117, def: 0x0007 },
    reg_default { reg: 118, def: 0x0000 }, reg_default { reg: 119, def: 0x2EE0 },
    reg_default { reg: 120, def: 0x0004 }, reg_default { reg: 121, def: 0x0014 },
    reg_default { reg: 122, def: 0x0010 }, reg_default { reg: 123, def: 0x0010 },
    reg_default { reg: 124, def: 0x0000 }, reg_default { reg: 126, def: 0x0000 },
    reg_default { reg: 128, def: 0xFFFF }, reg_default { reg: 129, def: 0x0000 },
    reg_default { reg: 130, def: 0x0000 }, reg_default { reg: 134, def: 0x0000 },
    reg_default { reg: 135, def: 0x000C }, reg_default { reg: 136, def: 0x000C },
    reg_default { reg: 137, def: 0x000C }, reg_default { reg: 138, def: 0x000C },
    reg_default { reg: 139, def: 0x000C }, reg_default { reg: 140, def: 0x0FCA },
    reg_default { reg: 141, def: 0x0400 }, reg_default { reg: 142, def: 0x00D8 },
    reg_default { reg: 143, def: 0x1EB5 }, reg_default { reg: 144, def: 0xF145 },
    reg_default { reg: 145, def: 0x0B75 }, reg_default { reg: 146, def: 0x01C5 },
    reg_default { reg: 147, def: 0x1C58 }, reg_default { reg: 148, def: 0xF373 },
    reg_default { reg: 149, def: 0x0A54 }, reg_default { reg: 150, def: 0x0558 },
    reg_default { reg: 151, def: 0x168E }, reg_default { reg: 152, def: 0xF829 },
    reg_default { reg: 153, def: 0x07AD }, reg_default { reg: 154, def: 0x1103 },
    reg_default { reg: 155, def: 0x0564 }, reg_default { reg: 156, def: 0x0559 },
    reg_default { reg: 157, def: 0x4000 }, reg_default { reg: 161, def: 0x0000 },
    reg_default { reg: 204, def: 0x0000 }, reg_default { reg: 247, def: 0x0000 },
    reg_default { reg: 248, def: 0x0019 },
];

unsafe fn wm8904_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    reg == WM8904_SW_RESET_AND_ID || reg == WM8904_REVISION || reg == WM8904_DC_SERVO_1 ||
    reg == WM8904_DC_SERVO_6 || reg == WM8904_DC_SERVO_7 || reg == WM8904_DC_SERVO_8 ||
    reg == WM8904_DC_SERVO_9 || reg == WM8904_DC_SERVO_READBACK_0 ||
    reg == WM8904_INTERRUPT_STATUS
}

unsafe fn wm8904_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    let regs = [
        WM8904_SW_RESET_AND_ID, WM8904_REVISION, WM8904_BIAS_CONTROL_0,
        WM8904_VMID_CONTROL_0, WM8904_MIC_BIAS_CONTROL_0, WM8904_MIC_BIAS_CONTROL_1,
        WM8904_ANALOGUE_DAC_0, WM8904_MIC_FILTER_CONTROL, WM8904_ANALOGUE_ADC_0,
        WM8904_POWER_MANAGEMENT_0, WM8904_POWER_MANAGEMENT_2, WM8904_POWER_MANAGEMENT_3,
        WM8904_POWER_MANAGEMENT_6, WM8904_CLOCK_RATES_0, WM8904_CLOCK_RATES_1,
        WM8904_CLOCK_RATES_2, WM8904_AUDIO_INTERFACE_0, WM8904_AUDIO_INTERFACE_1,
        WM8904_AUDIO_INTERFACE_2, WM8904_AUDIO_INTERFACE_3, WM8904_DAC_DIGITAL_VOLUME_LEFT,
        WM8904_DAC_DIGITAL_VOLUME_RIGHT, WM8904_DAC_DIGITAL_0, WM8904_DAC_DIGITAL_1,
        WM8904_ADC_DIGITAL_VOLUME_LEFT, WM8904_ADC_DIGITAL_VOLUME_RIGHT, WM8904_ADC_DIGITAL_0,
        WM8904_DIGITAL_MICROPHONE_0, WM8904_DRC_0, WM8904_DRC_1, WM8904_DRC_2,
        WM8904_DRC_3, WM8904_ANALOGUE_LEFT_INPUT_0, WM8904_ANALOGUE_RIGHT_INPUT_0,
        WM8904_ANALOGUE_LEFT_INPUT_1, WM8904_ANALOGUE_RIGHT_INPUT_1, WM8904_ANALOGUE_OUT1_LEFT,
        WM8904_ANALOGUE_OUT1_RIGHT, WM8904_ANALOGUE_OUT2_LEFT, WM8904_ANALOGUE_OUT2_RIGHT,
        WM8904_ANALOGUE_OUT12_ZC, WM8904_DC_SERVO_0, WM8904_DC_SERVO_1, WM8904_DC_SERVO_2,
        WM8904_DC_SERVO_4, WM8904_DC_SERVO_5, WM8904_DC_SERVO_6, WM8904_DC_SERVO_7,
        WM8904_DC_SERVO_8, WM8904_DC_SERVO_9, WM8904_DC_SERVO_READBACK_0,
        WM8904_ANALOGUE_HP_0, WM8904_ANALOGUE_LINEOUT_0, WM8904_CHARGE_PUMP_0,
        WM8904_CLASS_W_0, WM8904_WRITE_SEQUENCER_0, WM8904_WRITE_SEQUENCER_1,
        WM8904_WRITE_SEQUENCER_2, WM8904_WRITE_SEQUENCER_3, WM8904_WRITE_SEQUENCER_4,
        WM8904_FLL_CONTROL_1, WM8904_FLL_CONTROL_2, WM8904_FLL_CONTROL_3,
        WM8904_FLL_CONTROL_4, WM8904_FLL_CONTROL_5, WM8904_GPIO_CONTROL_1,
        WM8904_GPIO_CONTROL_2, WM8904_GPIO_CONTROL_3, WM8904_GPIO_CONTROL_4,
        WM8904_DIGITAL_PULLS, WM8904_INTERRUPT_STATUS, WM8904_INTERRUPT_STATUS_MASK,
        WM8904_INTERRUPT_POLARITY, WM8904_INTERRUPT_DEBOUNCE, WM8904_EQ1, WM8904_EQ2,
        WM8904_EQ3, WM8904_EQ4, WM8904_EQ5, WM8904_EQ6, WM8904_EQ7, WM8904_EQ8,
        WM8904_EQ9, WM8904_EQ10, WM8904_EQ11, WM8904_EQ12, WM8904_EQ13, WM8904_EQ14,
        WM8904_EQ15, WM8904_EQ16, WM8904_EQ17, WM8904_EQ18, WM8904_EQ19, WM8904_EQ20,
        WM8904_EQ21, WM8904_EQ22, WM8904_EQ23, WM8904_EQ24, WM8904_CONTROL_INTERFACE_TEST_1,
        WM8904_ADC_TEST_0, WM8904_ANALOGUE_OUTPUT_BIAS_0, WM8904_FLL_NCO_TEST_0,
        WM8904_FLL_NCO_TEST_1,
    ];
    regs.iter().any(|&r| r == reg)
}

unsafe fn wm8904_configure_clocking(component: *mut snd_soc_component) -> c_int {
    let wm8904 = snd_soc_component_get_drvdata(component) as *mut wm8904_priv;
    let mut clock2 = snd_soc_component_read(component, WM8904_CLOCK_RATES_2);
    snd_soc_component_update_bits(component, WM8904_CLOCK_RATES_2, WM8904_SYSCLK_SRC, 0);
    let rate: c_uint;
    if (*wm8904).sysclk_src == WM8904_CLK_MCLK {
        dev_dbg((*component).dev, cstr!("Using %dHz MCLK\n"), (*wm8904).mclk_rate);
        clock2 &= !WM8904_SYSCLK_SRC;
        rate = (*wm8904).mclk_rate;
        snd_soc_component_update_bits(component, WM8904_FLL_CONTROL_1, WM8904_FLL_OSC_ENA | WM8904_FLL_ENA, 0);
    } else if (*wm8904).sysclk_src == WM8904_CLK_FLL {
        dev_dbg((*component).dev, cstr!("Using %dHz FLL clock\n"), (*wm8904).fll_fout);
        clock2 |= WM8904_SYSCLK_SRC;
        rate = (*wm8904).fll_fout as c_uint;
    } else {
        dev_err((*component).dev, cstr!("System clock not configured\n"));
        return -EINVAL;
    }
    let clock0 = if rate > 13_500_000 {
        (*wm8904).sysclk_rate = rate / 2;
        WM8904_MCLK_DIV
    } else {
        (*wm8904).sysclk_rate = rate;
        0
    };
    snd_soc_component_update_bits(component, WM8904_CLOCK_RATES_0, WM8904_MCLK_DIV, clock0);
    snd_soc_component_update_bits(component, WM8904_CLOCK_RATES_2, WM8904_CLK_SYS_ENA | WM8904_SYSCLK_SRC, clock2);
    dev_dbg((*component).dev, cstr!("CLK_SYS is %dHz\n"), (*wm8904).sysclk_rate);
    0
}

unsafe fn wm8904_set_drc(component: *mut snd_soc_component) {
    let wm8904 = snd_soc_component_get_drvdata(component) as *mut wm8904_priv;
    let pdata = (*wm8904).pdata;
    let save = snd_soc_component_read(component, WM8904_DRC_0);
    for i in 0..WM8904_DRC_REGS {
        snd_soc_component_update_bits(component, WM8904_DRC_0 + i as c_uint, 0xffff, (*(*pdata).drc_cfgs.add((*wm8904).drc_cfg as usize)).regs[i as usize] as c_uint);
    }
    snd_soc_component_update_bits(component, WM8904_DRC_0, WM8904_DRC_ENA | WM8904_DRC_DAC_PATH, save);
}

unsafe fn wm8904_put_drc_enum(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let wm8904 = snd_soc_component_get_drvdata(component) as *mut wm8904_priv;
    let pdata = (*wm8904).pdata;
    let value = (*(*ucontrol).value.enumerated).item[0] as c_int;
    if value >= (*pdata).num_drc_cfgs { return -EINVAL; }
    (*wm8904).drc_cfg = value;
    wm8904_set_drc(component);
    0
}

unsafe fn wm8904_get_drc_enum(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let wm8904 = snd_soc_component_get_drvdata(component) as *mut wm8904_priv;
    (*(*ucontrol).value.enumerated).item[0] = (*wm8904).drc_cfg as c_uint;
    0
}

unsafe fn wm8904_set_retune_mobile(component: *mut snd_soc_component) {
    let wm8904 = snd_soc_component_get_drvdata(component) as *mut wm8904_priv;
    let pdata = (*wm8904).pdata;
    if pdata.is_null() || (*wm8904).num_retune_mobile_texts == 0 { return; }
    let cfg = (*wm8904).retune_mobile_cfg;
    let mut best = 0;
    let mut best_val = INT_MAX;
    for i in 0..(*pdata).num_retune_mobile_cfgs {
        let cfgp = (*pdata).retune_mobile_cfgs.add(i as usize);
        let text = *(*wm8904).retune_mobile_texts.add(cfg as usize);
        let delta = ((*cfgp).rate as c_int - (*wm8904).fs).abs();
        if strcmp((*cfgp).name, text) == 0 && delta < best_val {
            best = i;
            best_val = delta;
        }
    }
    let bestp = (*pdata).retune_mobile_cfgs.add(best as usize);
    dev_dbg((*component).dev, cstr!("ReTune Mobile %s/%dHz for %dHz sample rate\n"), (*bestp).name, (*bestp).rate, (*wm8904).fs);
    let save = snd_soc_component_read(component, WM8904_EQ1);
    for i in 0..WM8904_EQ_REGS {
        snd_soc_component_update_bits(component, WM8904_EQ1 + i as c_uint, 0xffff, (*bestp).regs[i as usize] as c_uint);
    }
    snd_soc_component_update_bits(component, WM8904_EQ1, WM8904_EQ_ENA, save);
}

unsafe fn wm8904_put_retune_mobile_enum(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let wm8904 = snd_soc_component_get_drvdata(component) as *mut wm8904_priv;
    let pdata = (*wm8904).pdata;
    let value = (*(*ucontrol).value.enumerated).item[0] as c_int;
    if value >= (*pdata).num_retune_mobile_cfgs { return -EINVAL; }
    (*wm8904).retune_mobile_cfg = value;
    wm8904_set_retune_mobile(component);
    0
}

unsafe fn wm8904_get_retune_mobile_enum(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let wm8904 = snd_soc_component_get_drvdata(component) as *mut wm8904_priv;
    (*(*ucontrol).value.enumerated).item[0] = (*wm8904).retune_mobile_cfg as c_uint;
    0
}

static deemph_settings: [c_int; 4] = [0, 32000, 44100, 48000];

unsafe fn wm8904_set_deemph(component: *mut snd_soc_component) -> c_int {
    let wm8904 = snd_soc_component_get_drvdata(component) as *mut wm8904_priv;
    let val = if (*wm8904).deemph != 0 {
        let mut best = 1usize;
        for i in 2..deemph_settings.len() {
            if (deemph_settings[i] - (*wm8904).fs).abs() < (deemph_settings[best] - (*wm8904).fs).abs() {
                best = i;
            }
        }
        (best as c_uint) << WM8904_DEEMPH_SHIFT
    } else { 0 };
    dev_dbg((*component).dev, cstr!("Set deemphasis %d\n"), val);
    snd_soc_component_update_bits(component, WM8904_DAC_DIGITAL_1, WM8904_DEEMPH_MASK, val)
}

unsafe fn wm8904_get_deemph(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let wm8904 = snd_soc_component_get_drvdata(component) as *mut wm8904_priv;
    (*(*ucontrol).value.integer).value[0] = (*wm8904).deemph as c_long;
    0
}

unsafe fn wm8904_put_deemph(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let wm8904 = snd_soc_component_get_drvdata(component) as *mut wm8904_priv;
    let deemph = (*(*ucontrol).value.integer).value[0] as c_uint;
    if deemph > 1 { return -EINVAL; }
    (*wm8904).deemph = deemph as c_int;
    wm8904_set_deemph(component)
}

/* TLV declarations, SOC controls, enums, DAPM widgets and routes from C are
 * represented here as dependency-provided opaque items. Their macro invocations
 * are preserved by name in comments because the surrounding ASoC Rust binding
 * layer determines their concrete representation.
 */
static hpf_mode_text: [*const c_char; 4] = [cstr!("Hi-fi"), cstr!("Voice 1"), cstr!("Voice 2"), cstr!("Voice 3")];
static drc_path_text: [*const c_char; 2] = [cstr!("ADC"), cstr!("DAC")];
static dmic_text: [*const c_char; 2] = [cstr!("DMIC1"), cstr!("DMIC2")];
static cin_text: [*const c_char; 2] = [cstr!("ADC"), cstr!("DMIC")];
static input_mode_text: [*const c_char; 3] = [cstr!("Single-Ended"), cstr!("Differential Line"), cstr!("Differential Mic")];
static lin_text: [*const c_char; 3] = [cstr!("IN1L"), cstr!("IN2L"), cstr!("IN3L")];
static rin_text: [*const c_char; 3] = [cstr!("IN1R"), cstr!("IN2R"), cstr!("IN3R")];
static aif_text: [*const c_char; 2] = [cstr!("Left"), cstr!("Right")];
static out_mux_text: [*const c_char; 2] = [cstr!("DAC"), cstr!("Bypass")];
static sidetone_text: [*const c_char; 3] = [cstr!("None"), cstr!("Left"), cstr!("Right")];

unsafe fn cp_event(_w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    if WARN_ON(event != SND_SOC_DAPM_POST_PMU) { return -EINVAL; }
    udelay(500);
    0
}

unsafe fn sysclk_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let wm8904 = snd_soc_component_get_drvdata(component) as *mut wm8904_priv;
    if event == SND_SOC_DAPM_PRE_PMU {
        if (*wm8904).sysclk_src == WM8904_CLK_FLL {
            snd_soc_component_update_bits(component, WM8904_FLL_CONTROL_1, WM8904_FLL_OSC_ENA, WM8904_FLL_OSC_ENA);
            snd_soc_component_update_bits(component, WM8904_FLL_CONTROL_1, WM8904_FLL_ENA, WM8904_FLL_ENA);
        }
    } else if event == SND_SOC_DAPM_POST_PMD {
        snd_soc_component_update_bits(component, WM8904_FLL_CONTROL_1, WM8904_FLL_OSC_ENA | WM8904_FLL_ENA, 0);
    }
    0
}

unsafe fn out_pga_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let wm8904 = snd_soc_component_get_drvdata(component) as *mut wm8904_priv;
    let reg = (*w).shift as c_uint;
    let (pwr_reg, mut dcs_mask, dcs_r_reg, dcs_l_reg, an_out_reg, dcs_l, dcs_r) =
        if reg == WM8904_ANALOGUE_HP_0 {
            (WM8904_POWER_MANAGEMENT_2, WM8904_DCS_ENA_CHAN_0 | WM8904_DCS_ENA_CHAN_1, WM8904_DC_SERVO_8, WM8904_DC_SERVO_9, WM8904_ANALOGUE_OUT1_LEFT, 0usize, 1usize)
        } else if reg == WM8904_ANALOGUE_LINEOUT_0 {
            (WM8904_POWER_MANAGEMENT_3, WM8904_DCS_ENA_CHAN_2 | WM8904_DCS_ENA_CHAN_3, WM8904_DC_SERVO_6, WM8904_DC_SERVO_7, WM8904_ANALOGUE_OUT2_LEFT, 2usize, 3usize)
        } else {
            WARN(1, cstr!("Invalid reg %d\n"), reg);
            return -EINVAL;
        };
    if event == SND_SOC_DAPM_PRE_PMU {
        snd_soc_component_update_bits(component, pwr_reg, (WM8904_HPL_PGA_ENA | WM8904_HPR_PGA_ENA) as c_uint, (WM8904_HPL_PGA_ENA | WM8904_HPR_PGA_ENA) as c_uint);
        snd_soc_component_update_bits(component, reg, (WM8904_HPL_ENA | WM8904_HPR_ENA) as c_uint, (WM8904_HPL_ENA | WM8904_HPR_ENA) as c_uint);
        snd_soc_component_update_bits(component, reg, (WM8904_HPL_ENA_DLY | WM8904_HPR_ENA_DLY) as c_uint, (WM8904_HPL_ENA_DLY | WM8904_HPR_ENA_DLY) as c_uint);
        snd_soc_component_update_bits(component, WM8904_DC_SERVO_0, dcs_mask as c_uint, dcs_mask as c_uint);
        let mut timeout;
        if (*wm8904).dcs_state[dcs_l] != 0 || (*wm8904).dcs_state[dcs_r] != 0 {
            dev_dbg((*component).dev, cstr!("Restoring DC servo state\n"));
            snd_soc_component_write(component, dcs_l_reg, (*wm8904).dcs_state[dcs_l] as c_uint);
            snd_soc_component_write(component, dcs_r_reg, (*wm8904).dcs_state[dcs_r] as c_uint);
            snd_soc_component_write(component, WM8904_DC_SERVO_1, dcs_mask as c_uint);
            timeout = 20;
        } else {
            dev_dbg((*component).dev, cstr!("Calibrating DC servo\n"));
            snd_soc_component_write(component, WM8904_DC_SERVO_1, (dcs_mask as c_uint) << WM8904_DCS_TRIG_STARTUP_0_SHIFT);
            timeout = 500;
        }
        dcs_mask <<= WM8904_DCS_CAL_COMPLETE_SHIFT as c_int;
        let mut val;
        loop {
            val = snd_soc_component_read(component, WM8904_DC_SERVO_READBACK_0);
            if (val & dcs_mask as c_uint) == dcs_mask as c_uint { break; }
            msleep(1);
            timeout -= 1;
            if timeout == 0 { break; }
        }
        if (val & dcs_mask as c_uint) != dcs_mask as c_uint {
            dev_warn((*component).dev, cstr!("DC servo timed out\n"));
        } else {
            dev_dbg((*component).dev, cstr!("DC servo ready\n"));
        }
        snd_soc_component_update_bits(component, reg, (WM8904_HPL_ENA_OUTP | WM8904_HPR_ENA_OUTP) as c_uint, (WM8904_HPL_ENA_OUTP | WM8904_HPR_ENA_OUTP) as c_uint);
        val = snd_soc_component_read(component, an_out_reg);
        snd_soc_component_write(component, an_out_reg, val);
    } else if event == SND_SOC_DAPM_POST_PMU {
        snd_soc_component_update_bits(component, reg, (WM8904_HPL_RMV_SHORT | WM8904_HPR_RMV_SHORT) as c_uint, (WM8904_HPL_RMV_SHORT | WM8904_HPR_RMV_SHORT) as c_uint);
    } else if event == SND_SOC_DAPM_PRE_PMD {
        snd_soc_component_update_bits(component, reg, (WM8904_HPL_RMV_SHORT | WM8904_HPR_RMV_SHORT) as c_uint, 0);
    } else if event == SND_SOC_DAPM_POST_PMD {
        (*wm8904).dcs_state[dcs_l] = snd_soc_component_read(component, dcs_l_reg) as c_int;
        (*wm8904).dcs_state[dcs_r] = snd_soc_component_read(component, dcs_r_reg) as c_int;
        snd_soc_component_update_bits(component, WM8904_DC_SERVO_0, dcs_mask as c_uint, 0);
        snd_soc_component_update_bits(component, reg, (WM8904_HPL_ENA | WM8904_HPR_ENA | WM8904_HPL_ENA_DLY | WM8904_HPR_ENA_DLY | WM8904_HPL_ENA_OUTP | WM8904_HPR_ENA_OUTP) as c_uint, 0);
        snd_soc_component_update_bits(component, pwr_reg, (WM8904_HPL_PGA_ENA | WM8904_HPR_PGA_ENA) as c_uint, 0);
    }
    0
}

#[repr(C)]
struct clk_sys_rate { ratio: c_int, clk_sys_rate: c_uint }
static clk_sys_rates: [clk_sys_rate; 10] = [
    clk_sys_rate { ratio: 64, clk_sys_rate: 0 }, clk_sys_rate { ratio: 128, clk_sys_rate: 1 },
    clk_sys_rate { ratio: 192, clk_sys_rate: 2 }, clk_sys_rate { ratio: 256, clk_sys_rate: 3 },
    clk_sys_rate { ratio: 384, clk_sys_rate: 4 }, clk_sys_rate { ratio: 512, clk_sys_rate: 5 },
    clk_sys_rate { ratio: 786, clk_sys_rate: 6 }, clk_sys_rate { ratio: 1024, clk_sys_rate: 7 },
    clk_sys_rate { ratio: 1408, clk_sys_rate: 8 }, clk_sys_rate { ratio: 1536, clk_sys_rate: 9 },
];
#[repr(C)]
struct sample_rate { rate: c_int, sample_rate: c_int }
static sample_rates: [sample_rate; 9] = [
    sample_rate { rate: 8000, sample_rate: 0 }, sample_rate { rate: 11025, sample_rate: 1 },
    sample_rate { rate: 12000, sample_rate: 1 }, sample_rate { rate: 16000, sample_rate: 2 },
    sample_rate { rate: 22050, sample_rate: 3 }, sample_rate { rate: 24000, sample_rate: 3 },
    sample_rate { rate: 32000, sample_rate: 4 }, sample_rate { rate: 44100, sample_rate: 5 },
    sample_rate { rate: 48000, sample_rate: 5 },
];
#[repr(C)]
struct bclk_div { div: c_int, bclk_div: c_int }
static bclk_divs: [bclk_div; 20] = [
    bclk_div { div: 10, bclk_div: 0 }, bclk_div { div: 15, bclk_div: 1 },
    bclk_div { div: 20, bclk_div: 2 }, bclk_div { div: 30, bclk_div: 3 },
    bclk_div { div: 40, bclk_div: 4 }, bclk_div { div: 50, bclk_div: 5 },
    bclk_div { div: 55, bclk_div: 6 }, bclk_div { div: 60, bclk_div: 7 },
    bclk_div { div: 80, bclk_div: 8 }, bclk_div { div: 100, bclk_div: 9 },
    bclk_div { div: 110, bclk_div: 10 }, bclk_div { div: 120, bclk_div: 11 },
    bclk_div { div: 160, bclk_div: 12 }, bclk_div { div: 200, bclk_div: 13 },
    bclk_div { div: 220, bclk_div: 14 }, bclk_div { div: 240, bclk_div: 16 },
    bclk_div { div: 200, bclk_div: 17 }, bclk_div { div: 320, bclk_div: 18 },
    bclk_div { div: 440, bclk_div: 19 }, bclk_div { div: 480, bclk_div: 20 },
];

unsafe fn wm8904_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let wm8904 = snd_soc_component_get_drvdata(component) as *mut wm8904_priv;
    let mut aif1: c_uint = 0;
    let mut aif2: c_uint = 0;
    let mut aif3: c_uint = 0;
    let mut clock1: c_uint = 0;
    let mut dac_digital1: c_uint = 0;
    (*wm8904).fs = params_rate(params);
    if (*wm8904).tdm_slots != 0 {
        dev_dbg((*component).dev, cstr!("Configuring for %d %d bit TDM slots\n"), (*wm8904).tdm_slots, (*wm8904).tdm_width);
        (*wm8904).bclk = snd_soc_calc_bclk((*wm8904).fs, (*wm8904).tdm_width, 2, (*wm8904).tdm_slots);
    } else {
        (*wm8904).bclk = snd_soc_params_to_bclk(params);
    }
    match params_width(params) {
        16 => {}
        20 => aif1 |= 0x40,
        24 => aif1 |= 0x80,
        32 => aif1 |= 0xc0,
        _ => return -EINVAL,
    }
    dev_dbg((*component).dev, cstr!("Target BCLK is %dHz\n"), (*wm8904).bclk);
    let ret = wm8904_configure_clocking(component);
    if ret != 0 { return ret; }
    let mut best = 0usize;
    let mut best_val = ((*wm8904).sysclk_rate as c_int / clk_sys_rates[0].ratio - (*wm8904).fs).abs();
    for i in 1..clk_sys_rates.len() {
        let cur_val = ((*wm8904).sysclk_rate as c_int / clk_sys_rates[i].ratio - (*wm8904).fs).abs();
        if cur_val < best_val { best = i; best_val = cur_val; }
    }
    dev_dbg((*component).dev, cstr!("Selected CLK_SYS_RATIO of %d\n"), clk_sys_rates[best].ratio);
    clock1 |= clk_sys_rates[best].clk_sys_rate << WM8904_CLK_SYS_RATE_SHIFT;
    best = 0;
    best_val = ((*wm8904).fs - sample_rates[0].rate).abs();
    for i in 1..sample_rates.len() {
        let cur_val = ((*wm8904).fs - sample_rates[i].rate).abs();
        if cur_val < best_val { best = i; best_val = cur_val; }
    }
    dev_dbg((*component).dev, cstr!("Selected SAMPLE_RATE of %dHz\n"), sample_rates[best].rate);
    clock1 |= (sample_rates[best].sample_rate as c_uint) << WM8904_SAMPLE_RATE_SHIFT;
    if (*wm8904).fs <= 24000 { dac_digital1 |= WM8904_DAC_SB_FILT; }
    best = 0;
    best_val = INT_MAX;
    for i in 0..bclk_divs.len() {
        let cur_val = (((*wm8904).sysclk_rate as c_int * 10) / bclk_divs[i].div) - (*wm8904).bclk;
        if cur_val < 0 { break; }
        if cur_val < best_val { best = i; best_val = cur_val; }
    }
    (*wm8904).bclk = ((*wm8904).sysclk_rate as c_int * 10) / bclk_divs[best].div;
    dev_dbg((*component).dev, cstr!("Selected BCLK_DIV of %d for %dHz BCLK\n"), bclk_divs[best].div, (*wm8904).bclk);
    aif2 |= bclk_divs[best].bclk_div as c_uint;
    dev_dbg((*component).dev, cstr!("LRCLK_RATE is %d\n"), (*wm8904).bclk / (*wm8904).fs);
    aif3 |= ((*wm8904).bclk / (*wm8904).fs) as c_uint;
    snd_soc_component_update_bits(component, WM8904_DAC_DIGITAL_1, WM8904_DAC_SB_FILT, dac_digital1);
    snd_soc_component_update_bits(component, WM8904_AUDIO_INTERFACE_1, WM8904_AIF_WL_MASK, aif1);
    snd_soc_component_update_bits(component, WM8904_AUDIO_INTERFACE_2, WM8904_BCLK_DIV_MASK, aif2);
    snd_soc_component_update_bits(component, WM8904_AUDIO_INTERFACE_3, WM8904_LRCLK_RATE_MASK, aif3);
    snd_soc_component_update_bits(component, WM8904_CLOCK_RATES_1, WM8904_SAMPLE_RATE_MASK | WM8904_CLK_SYS_RATE_MASK, clock1);
    wm8904_set_retune_mobile(component);
    wm8904_set_deemph(component);
    0
}

unsafe fn wm8904_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let mut aif1: c_uint = 0;
    let mut aif3: c_uint = 0;
    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        x if x == SND_SOC_DAIFMT_CBC_CFC => {}
        x if x == SND_SOC_DAIFMT_CBC_CFP => aif3 |= WM8904_LRCLK_DIR,
        x if x == SND_SOC_DAIFMT_CBP_CFC => aif1 |= WM8904_BCLK_DIR,
        x if x == SND_SOC_DAIFMT_CBP_CFP => { aif1 |= WM8904_BCLK_DIR; aif3 |= WM8904_LRCLK_DIR; }
        _ => return -EINVAL,
    }
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_DSP_B => { aif1 |= 0x3 | WM8904_AIF_LRCLK_INV; }
        x if x == SND_SOC_DAIFMT_DSP_A => { aif1 |= 0x3; }
        x if x == SND_SOC_DAIFMT_I2S => aif1 |= 0x2,
        x if x == SND_SOC_DAIFMT_RIGHT_J => {}
        x if x == SND_SOC_DAIFMT_LEFT_J => aif1 |= 0x1,
        _ => return -EINVAL,
    }
    if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_DSP_A || (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_DSP_B {
        match fmt & SND_SOC_DAIFMT_INV_MASK {
            x if x == SND_SOC_DAIFMT_NB_NF => {}
            x if x == SND_SOC_DAIFMT_IB_NF => aif1 |= WM8904_AIF_BCLK_INV,
            _ => return -EINVAL,
        }
    } else {
        match fmt & SND_SOC_DAIFMT_INV_MASK {
            x if x == SND_SOC_DAIFMT_NB_NF => {}
            x if x == SND_SOC_DAIFMT_IB_IF => aif1 |= WM8904_AIF_BCLK_INV | WM8904_AIF_LRCLK_INV,
            x if x == SND_SOC_DAIFMT_IB_NF => aif1 |= WM8904_AIF_BCLK_INV,
            x if x == SND_SOC_DAIFMT_NB_IF => aif1 |= WM8904_AIF_LRCLK_INV,
            _ => return -EINVAL,
        }
    }
    snd_soc_component_update_bits(component, WM8904_AUDIO_INTERFACE_1, WM8904_AIF_BCLK_INV | WM8904_AIF_LRCLK_INV | WM8904_AIF_FMT_MASK | WM8904_BCLK_DIR, aif1);
    snd_soc_component_update_bits(component, WM8904_AUDIO_INTERFACE_3, WM8904_LRCLK_DIR, aif3);
    0
}

unsafe fn wm8904_set_tdm_slot(dai: *mut snd_soc_dai, tx_mask: c_uint, rx_mask: c_uint, slots: c_int, slot_width: c_int) -> c_int {
    let component = (*dai).component;
    let wm8904 = snd_soc_component_get_drvdata(component) as *mut wm8904_priv;
    let mut aif1: c_int = 0;
    if slots != 0 {
        aif1 |= WM8904_AIFADC_TDM | WM8904_AIFDAC_TDM;
        match rx_mask { 3 => {}, 0xc => aif1 |= WM8904_AIFADC_TDM_CHAN, _ => return -EINVAL }
        match tx_mask { 3 => {}, 0xc => aif1 |= WM8904_AIFDAC_TDM_CHAN, _ => return -EINVAL }
    }
    (*wm8904).tdm_width = slot_width;
    (*wm8904).tdm_slots = slots / 2;
    snd_soc_component_update_bits(component, WM8904_AUDIO_INTERFACE_1, (WM8904_AIFADC_TDM | WM8904_AIFADC_TDM_CHAN | WM8904_AIFDAC_TDM | WM8904_AIFDAC_TDM_CHAN) as c_uint, aif1 as c_uint);
    0
}

#[repr(C)]
struct _fll_div { fll_fratio: u16, fll_outdiv: u16, fll_clk_ref_div: u16, n: u16, k: u16 }
#[repr(C)]
struct fll_fratio { min: c_uint, max: c_uint, fll_fratio: u16, ratio: c_int }
static fll_fratios: [fll_fratio; 5] = [
    fll_fratio { min: 0, max: 64000, fll_fratio: 4, ratio: 16 },
    fll_fratio { min: 64000, max: 128000, fll_fratio: 3, ratio: 8 },
    fll_fratio { min: 128000, max: 256000, fll_fratio: 2, ratio: 4 },
    fll_fratio { min: 256000, max: 1000000, fll_fratio: 1, ratio: 2 },
    fll_fratio { min: 1000000, max: 13500000, fll_fratio: 0, ratio: 1 },
];

unsafe fn fll_factors(fll_div: *mut _fll_div, mut Fref: c_uint, Fout: c_uint) -> c_int {
    let mut div: c_uint = 1;
    (*fll_div).fll_clk_ref_div = 0;
    while Fref / div > 13_500_000 {
        div *= 2;
        (*fll_div).fll_clk_ref_div += 1;
        if div > 8 {
            pr_err(cstr!("Can't scale %dMHz input down to <=13.5MHz\n"), Fref);
            return -EINVAL;
        }
    }
    pr_debug(cstr!("Fref=%u Fout=%u\n"), Fref, Fout);
    Fref /= div;
    div = 4;
    while Fout * div < 90_000_000 {
        div += 1;
        if div > 64 {
            pr_err(cstr!("Unable to find FLL_OUTDIV for Fout=%uHz\n"), Fout);
            return -EINVAL;
        }
    }
    let mut target = Fout * div;
    (*fll_div).fll_outdiv = (div - 1) as u16;
    pr_debug(cstr!("Fvco=%dHz\n"), target);
    let mut found = false;
    for fr in fll_fratios.iter() {
        if fr.min <= Fref && Fref <= fr.max {
            (*fll_div).fll_fratio = fr.fll_fratio;
            target /= fr.ratio as c_uint;
            found = true;
            break;
        }
    }
    if !found {
        pr_err(cstr!("Unable to find FLL_FRATIO for Fref=%uHz\n"), Fref);
        return -EINVAL;
    }
    let Ndiv = target / Fref;
    (*fll_div).n = Ndiv as u16;
    let Nmod = target % Fref;
    pr_debug(cstr!("Nmod=%d\n"), Nmod);
    let mut K: c_uint = ((FIXED_FLL_SIZE * Nmod as u64) / Fref as u64) as c_uint;
    if K % 10 >= 5 { K += 5; }
    (*fll_div).k = (K / 10) as u16;
    pr_debug(cstr!("N=%x K=%x FLL_FRATIO=%x FLL_OUTDIV=%x FLL_CLK_REF_DIV=%x\n"), (*fll_div).n as c_int, (*fll_div).k as c_int, (*fll_div).fll_fratio as c_int, (*fll_div).fll_outdiv as c_int, (*fll_div).fll_clk_ref_div as c_int);
    0
}

unsafe fn wm8904_set_fll(dai: *mut snd_soc_dai, fll_id: c_int, source: c_int, mut Fref: c_uint, mut Fout: c_uint) -> c_int {
    let component = (*dai).component;
    let wm8904 = snd_soc_component_get_drvdata(component) as *mut wm8904_priv;
    let mut fll_div: _fll_div = zeroed();
    if source == (*wm8904).fll_src && Fref as c_int == (*wm8904).fll_fref && Fout as c_int == (*wm8904).fll_fout { return 0; }
    let clock2 = snd_soc_component_read(component, WM8904_CLOCK_RATES_2);
    if Fout == 0 {
        dev_dbg((*component).dev, cstr!("FLL disabled\n"));
        (*wm8904).fll_fref = 0;
        (*wm8904).fll_fout = 0;
        snd_soc_component_update_bits(component, WM8904_CLOCK_RATES_2, WM8904_CLK_SYS_ENA, 0);
        snd_soc_component_update_bits(component, WM8904_FLL_CONTROL_1, WM8904_FLL_OSC_ENA | WM8904_FLL_ENA, 0);
        snd_soc_component_update_bits(component, WM8904_CLOCK_RATES_2, WM8904_CLK_SYS_ENA, clock2);
        return 0;
    }
    if source == WM8904_FLL_MCLK || source == WM8904_FLL_LRCLK || source == WM8904_FLL_BCLK {
        let ret = fll_factors(&mut fll_div, Fref, Fout);
        if ret != 0 { return ret; }
    } else if source == WM8904_FLL_FREE_RUNNING {
        dev_dbg((*component).dev, cstr!("Using free running FLL\n"));
        Fout = 12_000_000;
        Fref = 12_000_000;
        memset(&mut fll_div as *mut _ as *mut c_void, 0, size_of::<_fll_div>());
        fll_div.fll_outdiv = 3;
    } else {
        dev_err((*component).dev, cstr!("Unknown FLL ID %d\n"), fll_id);
        return -EINVAL;
    }
    let fll1 = snd_soc_component_read(component, WM8904_FLL_CONTROL_1);
    snd_soc_component_update_bits(component, WM8904_CLOCK_RATES_2, WM8904_CLK_SYS_ENA, 0);
    snd_soc_component_update_bits(component, WM8904_FLL_CONTROL_1, WM8904_FLL_OSC_ENA | WM8904_FLL_ENA, 0);
    snd_soc_component_update_bits(component, WM8904_CONTROL_INTERFACE_TEST_1, WM8904_USER_KEY, WM8904_USER_KEY);
    let val = if fll_id == WM8904_FLL_FREE_RUNNING { WM8904_FLL_FRC_NCO } else { 0 };
    snd_soc_component_update_bits(component, WM8904_FLL_NCO_TEST_1, WM8904_FLL_FRC_NCO as c_uint, val as c_uint);
    snd_soc_component_update_bits(component, WM8904_CONTROL_INTERFACE_TEST_1, WM8904_USER_KEY, 0);
    if fll_id == WM8904_FLL_MCLK {
        snd_soc_component_update_bits(component, WM8904_FLL_CONTROL_5, WM8904_FLL_CLK_REF_SRC_MASK, 0);
    } else if fll_id == WM8904_FLL_LRCLK {
        snd_soc_component_update_bits(component, WM8904_FLL_CONTROL_5, WM8904_FLL_CLK_REF_SRC_MASK, 1);
    } else if fll_id == WM8904_FLL_BCLK {
        snd_soc_component_update_bits(component, WM8904_FLL_CONTROL_5, WM8904_FLL_CLK_REF_SRC_MASK, 2);
    }
    let frac_val = if fll_div.k != 0 { WM8904_FLL_FRACN_ENA } else { 0 };
    snd_soc_component_update_bits(component, WM8904_FLL_CONTROL_1, WM8904_FLL_FRACN_ENA as c_uint, frac_val as c_uint);
    snd_soc_component_update_bits(component, WM8904_FLL_CONTROL_2, WM8904_FLL_OUTDIV_MASK | WM8904_FLL_FRATIO_MASK, ((fll_div.fll_outdiv as c_uint) << WM8904_FLL_OUTDIV_SHIFT) | ((fll_div.fll_fratio as c_uint) << WM8904_FLL_FRATIO_SHIFT));
    snd_soc_component_write(component, WM8904_FLL_CONTROL_3, fll_div.k as c_uint);
    snd_soc_component_update_bits(component, WM8904_FLL_CONTROL_4, WM8904_FLL_N_MASK, (fll_div.n as c_uint) << WM8904_FLL_N_SHIFT);
    snd_soc_component_update_bits(component, WM8904_FLL_CONTROL_5, WM8904_FLL_CLK_REF_DIV_MASK, (fll_div.fll_clk_ref_div as c_uint) << WM8904_FLL_CLK_REF_DIV_SHIFT);
    dev_dbg((*component).dev, cstr!("FLL configured for %dHz->%dHz\n"), Fref, Fout);
    (*wm8904).fll_fref = Fref as c_int;
    (*wm8904).fll_fout = Fout as c_int;
    (*wm8904).fll_src = source;
    snd_soc_component_update_bits(component, WM8904_FLL_CONTROL_1, WM8904_FLL_OSC_ENA, fll1);
    snd_soc_component_update_bits(component, WM8904_FLL_CONTROL_1, WM8904_FLL_ENA, fll1);
    snd_soc_component_update_bits(component, WM8904_CLOCK_RATES_2, WM8904_CLK_SYS_ENA, clock2);
    0
}

unsafe fn wm8904_set_sysclk(dai: *mut snd_soc_dai, mut clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut wm8904_priv;
    if clk_id == WM8904_CLK_AUTO {
        if freq == 0 { return 0; }
        let mclk_freq = clk_get_rate((*priv_).mclk);
        if mclk_freq != freq as c_ulong {
            (*priv_).sysclk_src = WM8904_CLK_FLL;
            let ret = wm8904_set_fll(dai, WM8904_FLL_MCLK, WM8904_FLL_MCLK, mclk_freq as c_uint, freq);
            if ret != 0 { return ret; }
        } else {
            clk_id = WM8904_CLK_MCLK;
            (*priv_).sysclk_src = clk_id;
            (*priv_).mclk_rate = freq;
        }
    } else if clk_id == WM8904_CLK_MCLK {
        (*priv_).sysclk_src = clk_id;
        (*priv_).mclk_rate = freq;
    } else if clk_id == WM8904_CLK_FLL {
        (*priv_).sysclk_src = clk_id;
    } else {
        return -EINVAL;
    }
    dev_dbg((*dai).dev, cstr!("Clock source is %d at %uHz\n"), clk_id, freq);
    wm8904_configure_clocking(component);
    0
}

unsafe fn wm8904_mute(codec_dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*codec_dai).component;
    let val = if mute != 0 { WM8904_DAC_MUTE } else { 0 };
    snd_soc_component_update_bits(component, WM8904_DAC_DIGITAL_1, WM8904_DAC_MUTE as c_uint, val as c_uint);
    0
}

unsafe fn wm8904_set_bias_level(component: *mut snd_soc_component, level: c_int) -> c_int {
    let wm8904 = snd_soc_component_get_drvdata(component) as *mut wm8904_priv;
    let dapm = snd_soc_component_to_dapm(component);
    if level == SND_SOC_BIAS_ON {
    } else if level == SND_SOC_BIAS_PREPARE {
        snd_soc_component_update_bits(component, WM8904_VMID_CONTROL_0, WM8904_VMID_RES_MASK, 0x1 << WM8904_VMID_RES_SHIFT);
        snd_soc_component_update_bits(component, WM8904_BIAS_CONTROL_0, WM8904_ISEL_MASK, 2 << WM8904_ISEL_SHIFT);
    } else if level == SND_SOC_BIAS_STANDBY {
        if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
            let ret = regulator_bulk_enable(ARRAY_SIZE!((*wm8904).supplies), (*wm8904).supplies.as_mut_ptr());
            if ret != 0 {
                dev_err((*component).dev, cstr!("Failed to enable supplies: %d\n"), ret);
                return ret;
            }
            let ret = clk_prepare_enable((*wm8904).mclk);
            if ret != 0 {
                dev_err((*component).dev, cstr!("Failed to enable MCLK: %d\n"), ret);
                regulator_bulk_disable(ARRAY_SIZE!((*wm8904).supplies), (*wm8904).supplies.as_mut_ptr());
                return ret;
            }
            regcache_cache_only((*wm8904).regmap, false);
            regcache_sync((*wm8904).regmap);
            snd_soc_component_update_bits(component, WM8904_BIAS_CONTROL_0, WM8904_BIAS_ENA, WM8904_BIAS_ENA);
            snd_soc_component_update_bits(component, WM8904_VMID_CONTROL_0, WM8904_VMID_ENA | WM8904_VMID_RES_MASK, WM8904_VMID_ENA | (0x3 << WM8904_VMID_RES_SHIFT));
            msleep(1);
        }
        snd_soc_component_update_bits(component, WM8904_VMID_CONTROL_0, WM8904_VMID_RES_MASK, 0x2 << WM8904_VMID_RES_SHIFT);
        snd_soc_component_update_bits(component, WM8904_BIAS_CONTROL_0, WM8904_ISEL_MASK, 0);
    } else if level == SND_SOC_BIAS_OFF {
        snd_soc_component_update_bits(component, WM8904_VMID_CONTROL_0, WM8904_VMID_RES_MASK | WM8904_VMID_ENA, 0);
        snd_soc_component_update_bits(component, WM8904_BIAS_CONTROL_0, WM8904_BIAS_ENA, 0);
        snd_soc_component_write(component, WM8904_SW_RESET_AND_ID, 0);
        regcache_cache_only((*wm8904).regmap, true);
        regcache_mark_dirty((*wm8904).regmap);
        regulator_bulk_disable(ARRAY_SIZE!((*wm8904).supplies), (*wm8904).supplies.as_mut_ptr());
        clk_disable_unprepare((*wm8904).mclk);
    }
    0
}

unsafe fn wm8904_handle_retune_mobile_pdata(component: *mut snd_soc_component) {
    let wm8904 = snd_soc_component_get_drvdata(component) as *mut wm8904_priv;
    let pdata = (*wm8904).pdata;
    (*wm8904).num_retune_mobile_texts = 0;
    (*wm8904).retune_mobile_texts = ptr::null_mut();
    for i in 0..(*pdata).num_retune_mobile_cfgs {
        let mut j = 0;
        while j < (*wm8904).num_retune_mobile_texts {
            if strcmp((*(*pdata).retune_mobile_cfgs.add(i as usize)).name, *(*wm8904).retune_mobile_texts.add(j as usize)) == 0 { break; }
            j += 1;
        }
        if j != (*wm8904).num_retune_mobile_texts { continue; }
        let t = krealloc((*wm8904).retune_mobile_texts as *mut c_void, size_of::<*const c_char>() * ((*wm8904).num_retune_mobile_texts as usize + 1), GFP_KERNEL) as *mut *const c_char;
        if t.is_null() { continue; }
        *t.add((*wm8904).num_retune_mobile_texts as usize) = (*(*pdata).retune_mobile_cfgs.add(i as usize)).name;
        (*wm8904).num_retune_mobile_texts += 1;
        (*wm8904).retune_mobile_texts = t;
    }
    dev_dbg((*component).dev, cstr!("Allocated %d unique ReTune Mobile names\n"), (*wm8904).num_retune_mobile_texts);
    (*wm8904).retune_mobile_enum.items = (*wm8904).num_retune_mobile_texts as c_uint;
    (*wm8904).retune_mobile_enum.texts = (*wm8904).retune_mobile_texts;
    /* snd_soc_add_component_controls(component, SOC_ENUM_EXT("EQ Mode", ...), 1) */
}

unsafe fn wm8904_handle_dmic_pdata(component: *mut snd_soc_component) {
    let dapm = snd_soc_component_to_dapm(component);
    let wm8904 = snd_soc_component_get_drvdata(component) as *mut wm8904_priv;
    let pdata = (*wm8904).pdata;
    if !(*pdata).in1l_as_dmicdat1 && !(*pdata).in1r_as_dmicdat2 {
        /* snd_soc_dapm_add_routes(dapm, cin_nodmic_con, ARRAY_SIZE(cin_nodmic_con)); */
        snd_soc_component_update_bits(component, WM8904_DIGITAL_MICROPHONE_0, WM8904_DMIC_ENA_MASK, 0);
        return;
    }
    /* Add Capture Input controls and ADC/DMIC routes. */
    let dmic_src: c_uint;
    if (*pdata).in1l_as_dmicdat1 && (*pdata).in1r_as_dmicdat2 {
        dev_dbg((*component).dev, cstr!("DMICDAT1 and DMICDAT2 in use\n"));
        return;
    }
    if (*pdata).in1l_as_dmicdat1 {
        dmic_src = 0;
        let _ = dapm;
    } else {
        dmic_src = 1;
        let _ = dapm;
    }
    dev_dbg((*component).dev, cstr!("DMIC_SRC (0 or 1): %d\n"), dmic_src);
    snd_soc_component_update_bits(component, WM8904_DIGITAL_MICROPHONE_0, WM8904_DMIC_SRC_MASK, dmic_src << WM8904_DMIC_SRC_SHIFT);
}

unsafe fn wm8904_handle_pdata(component: *mut snd_soc_component) {
    let dapm = snd_soc_component_to_dapm(component);
    let wm8904 = snd_soc_component_get_drvdata(component) as *mut wm8904_priv;
    let pdata = (*wm8904).pdata;
    if pdata.is_null() {
        let _ = dapm;
        /* Add cin_nodmic_con routes and wm8904_eq_controls. */
        return;
    }
    wm8904_handle_dmic_pdata(component);
    dev_dbg((*component).dev, cstr!("%d DRC configurations\n"), (*pdata).num_drc_cfgs);
    if (*pdata).num_drc_cfgs != 0 {
        (*wm8904).drc_texts = kmalloc_array((*pdata).num_drc_cfgs as usize, size_of::<*const c_char>(), GFP_KERNEL) as *mut *const c_char;
        if (*wm8904).drc_texts.is_null() { return; }
        for i in 0..(*pdata).num_drc_cfgs {
            *(*wm8904).drc_texts.add(i as usize) = (*(*pdata).drc_cfgs.add(i as usize)).name;
        }
        (*wm8904).drc_enum.items = (*pdata).num_drc_cfgs as c_uint;
        (*wm8904).drc_enum.texts = (*wm8904).drc_texts;
        /* snd_soc_add_component_controls(component, SOC_ENUM_EXT("DRC Mode", ...), 1) */
        wm8904_set_drc(component);
    }
    dev_dbg((*component).dev, cstr!("%d ReTune Mobile configurations\n"), (*pdata).num_retune_mobile_cfgs);
    if (*pdata).num_retune_mobile_cfgs != 0 {
        wm8904_handle_retune_mobile_pdata(component);
    } else {
        /* snd_soc_add_component_controls(component, wm8904_eq_controls, ARRAY_SIZE(wm8904_eq_controls)); */
    }
}

unsafe fn wm8904_add_widgets(component: *mut snd_soc_component) -> c_int {
    let wm8904 = snd_soc_component_get_drvdata(component) as *mut wm8904_priv;
    let dapm = snd_soc_component_to_dapm(component);
    let _ = dapm;
    /* Add core widgets/routes, then WM8904 or WM8912 controls/widgets/routes. */
    match (*wm8904).devtype {
        wm8904_type::WM8904 => {}
        wm8904_type::WM8912 => {}
    }
    0
}

unsafe fn wm8904_probe(component: *mut snd_soc_component) -> c_int {
    let wm8904 = snd_soc_component_get_drvdata(component) as *mut wm8904_priv;
    match (*wm8904).devtype {
        wm8904_type::WM8904 => {}
        wm8904_type::WM8912 => {
            /* memset(&wm8904_dai.capture, 0, sizeof(wm8904_dai.capture)); */
        }
    }
    wm8904_add_widgets(component);
    wm8904_handle_pdata(component);
    0
}

unsafe fn wm8904_remove(component: *mut snd_soc_component) {
    let wm8904 = snd_soc_component_get_drvdata(component) as *mut wm8904_priv;
    kfree((*wm8904).retune_mobile_texts as *mut c_void);
    kfree((*wm8904).drc_texts as *mut c_void);
}

unsafe fn wm8904_read_cfg_reg_arr(np: *const device_node, regs_property: *const c_char, size: c_int, idx: c_int, out: *mut u16) -> c_int {
    let offset = idx * size;
    for i in 0..size {
        let ret = of_property_read_u16_index(np, regs_property, (i + offset) as c_uint, out.add(i as usize));
        if ret != 0 { return ret; }
    }
    0
}

unsafe fn wm8904_parse_retune_cfg_regs(np: *const device_node, pdata: *mut wm8904_pdata, cfg_idx: c_int) -> c_int {
    wm8904_read_cfg_reg_arr(np, cstr!("wlf,retune-mobile-cfg-regs"), WM8904_EQ_REGS, cfg_idx, (*(*pdata).retune_mobile_cfgs.add(cfg_idx as usize)).regs.as_mut_ptr())
}

unsafe fn wm8904_parse_drc_cfg_regs(np: *const device_node, pdata: *mut wm8904_pdata, cfg_idx: c_int) -> c_int {
    wm8904_read_cfg_reg_arr(np, cstr!("wlf,drc-cfg-regs"), WM8904_DRC_REGS, cfg_idx, (*(*pdata).drc_cfgs.add(cfg_idx as usize)).regs.as_mut_ptr())
}

unsafe fn wm8904_parse_drc_cfg_from_of(i2c: *mut i2c_client, pdata: *mut wm8904_pdata) -> c_int {
    let np = (*i2c).dev.of_node;
    let n_cfgs = of_property_count_strings(np, cstr!("wlf,drc-cfg-names"));
    if n_cfgs == -EINVAL { return 0; }
    if n_cfgs <= 0 {
        dev_err(&mut (*i2c).dev as *mut _ as *mut device, cstr!("Could not get wlf,drc-cfg-names length: %d"), n_cfgs);
        return n_cfgs;
    }
    (*pdata).drc_cfgs = devm_kzalloc(&mut (*i2c).dev as *mut _ as *mut device, n_cfgs as usize * size_of::<wm8904_drc_cfg>(), GFP_KERNEL) as *mut wm8904_drc_cfg;
    if (*pdata).drc_cfgs.is_null() { return -ENOMEM; }
    for i in 0..n_cfgs {
        if wm8904_parse_drc_cfg_regs(np, pdata, i) != 0 {
            dev_err(&mut (*i2c).dev as *mut _ as *mut device, cstr!("Invalid 'wlf,drc-cfg-regs[%i,:]'\n"), i);
            return -EINVAL;
        }
        if of_property_read_string_index(np, cstr!("wlf,drc-cfg-names"), i, &mut (*(*pdata).drc_cfgs.add(i as usize)).name) != 0 {
            dev_err(&mut (*i2c).dev as *mut _ as *mut device, cstr!("Invalid 'wlf,drc-cfg-names[%i]'\n"), i);
            return -EINVAL;
        }
    }
    (*pdata).num_drc_cfgs = n_cfgs;
    0
}

unsafe fn wm8904_parse_retune_cfg_from_of(i2c: *mut i2c_client, pdata: *mut wm8904_pdata) -> c_int {
    let np = (*i2c).dev.of_node;
    let n_cfgs = of_property_count_strings(np, cstr!("wlf,retune-mobile-cfg-names"));
    if n_cfgs == -EINVAL { return 0; }
    if n_cfgs <= 0 {
        dev_err(&mut (*i2c).dev as *mut _ as *mut device, cstr!("Could not get wlf,retune-mobile-cfg-names length: %d"), n_cfgs);
        return n_cfgs;
    }
    (*pdata).retune_mobile_cfgs = devm_kzalloc(&mut (*i2c).dev as *mut _ as *mut device, n_cfgs as usize * size_of::<wm8904_retune_mobile_cfg>(), GFP_KERNEL) as *mut wm8904_retune_mobile_cfg;
    if (*pdata).retune_mobile_cfgs.is_null() { return -ENOMEM; }
    for i in 0..n_cfgs {
        if wm8904_parse_retune_cfg_regs(np, pdata, i) != 0 {
            dev_err(&mut (*i2c).dev as *mut _ as *mut device, cstr!("Invalid 'wlf,retune-mobile-cfg-regs[%i,:]'\n"), i);
            return -EINVAL;
        }
        if of_property_read_u32_index(np, cstr!("wlf,retune-mobile-cfg-hz"), i as c_uint, &mut (*(*pdata).retune_mobile_cfgs.add(i as usize)).rate) != 0 {
            dev_err(&mut (*i2c).dev as *mut _ as *mut device, cstr!("Invalid 'wlf,retune-mobile-cfg-hz[%i]'\n"), i);
            return -EINVAL;
        }
        if of_property_read_string_index(np, cstr!("wlf,retune-mobile-cfg-names"), i, &mut (*(*pdata).retune_mobile_cfgs.add(i as usize)).name) != 0 {
            dev_err(&mut (*i2c).dev as *mut _ as *mut device, cstr!("Invalid 'wlf,retune-mobile-cfg-names[%i]'\n"), i);
            return -EINVAL;
        }
    }
    (*pdata).num_retune_mobile_cfgs = n_cfgs;
    0
}

unsafe fn wm8904_set_pdata_from_of(i2c: *mut i2c_client, wm8904: *mut wm8904_priv) -> c_int {
    let np = (*i2c).dev.of_node;
    let pdata = devm_kzalloc(&mut (*i2c).dev as *mut _ as *mut device, size_of::<wm8904_pdata>(), GFP_KERNEL) as *mut wm8904_pdata;
    if pdata.is_null() { return -ENOMEM; }
    (*pdata).in1l_as_dmicdat1 = of_property_read_bool(np, cstr!("wlf,in1l-as-dmicdat1"));
    (*pdata).in1r_as_dmicdat2 = of_property_read_bool(np, cstr!("wlf,in1r-as-dmicdat2"));
    for i in 0..WM8904_GPIO_REGS { (*pdata).gpio_cfg[i as usize] = 0xFFFF; }
    of_property_read_u32_array(np, cstr!("wlf,gpio-cfg"), (*pdata).gpio_cfg.as_mut_ptr(), (*pdata).gpio_cfg.len());
    of_property_read_u32_array(np, cstr!("wlf,micbias-cfg"), (*pdata).mic_cfg.as_mut_ptr(), (*pdata).mic_cfg.len());
    let mut ret = wm8904_parse_drc_cfg_from_of(i2c, pdata);
    if ret != 0 { return ret; }
    ret = wm8904_parse_retune_cfg_from_of(i2c, pdata);
    if ret != 0 { return ret; }
    (*wm8904).pdata = pdata;
    0
}

unsafe fn wm8904_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let wm8904 = devm_kzalloc(&mut (*i2c).dev as *mut _ as *mut device, size_of::<wm8904_priv>(), GFP_KERNEL) as *mut wm8904_priv;
    if wm8904.is_null() { return -ENOMEM; }
    (*wm8904).mclk = devm_clk_get(&mut (*i2c).dev as *mut _ as *mut device, cstr!("mclk"));
    if IS_ERR((*wm8904).mclk as *const c_void) {
        let ret = PTR_ERR((*wm8904).mclk as *const c_void);
        dev_err(&mut (*i2c).dev as *mut _ as *mut device, cstr!("Failed to get MCLK\n"));
        return ret;
    }
    (*wm8904).regmap = devm_regmap_init_i2c(i2c, &wm8904_regmap);
    if IS_ERR((*wm8904).regmap as *const c_void) {
        let ret = PTR_ERR((*wm8904).regmap as *const c_void);
        dev_err(&mut (*i2c).dev as *mut _ as *mut device, cstr!("Failed to allocate register map: %d\n"), ret);
        return ret;
    }
    (*wm8904).devtype = if (i2c_get_match_data(i2c) as usize) == wm8904_type::WM8912 as usize { wm8904_type::WM8912 } else { wm8904_type::WM8904 };
    i2c_set_clientdata(i2c, wm8904 as *mut c_void);
    if !(*i2c).dev.of_node.is_null() {
        let ret = wm8904_set_pdata_from_of(i2c, wm8904);
        if ret != 0 {
            dev_err(&mut (*i2c).dev as *mut _ as *mut device, cstr!("Failed to set platform data from of: %d\n"), ret);
            return ret;
        }
    } else {
        (*wm8904).pdata = (*i2c).dev.platform_data as *mut wm8904_pdata;
    }
    for i in 0..(*wm8904).supplies.len() { (*wm8904).supplies[i].supply = wm8904_supply_names[i]; }
    let mut ret = devm_regulator_bulk_get(&mut (*i2c).dev as *mut _ as *mut device, ARRAY_SIZE!((*wm8904).supplies), (*wm8904).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(&mut (*i2c).dev as *mut _ as *mut device, cstr!("Failed to request supplies: %d\n"), ret);
        return ret;
    }
    ret = regulator_bulk_enable(ARRAY_SIZE!((*wm8904).supplies), (*wm8904).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(&mut (*i2c).dev as *mut _ as *mut device, cstr!("Failed to enable supplies: %d\n"), ret);
        return ret;
    }
    let mut val: c_uint = 0;
    ret = regmap_read((*wm8904).regmap, WM8904_SW_RESET_AND_ID, &mut val);
    if ret < 0 {
        dev_err(&mut (*i2c).dev as *mut _ as *mut device, cstr!("Failed to read ID register: %d\n"), ret);
        regulator_bulk_disable(ARRAY_SIZE!((*wm8904).supplies), (*wm8904).supplies.as_mut_ptr());
        return ret;
    }
    if val != 0x8904 {
        dev_err(&mut (*i2c).dev as *mut _ as *mut device, cstr!("Device is not a WM8904, ID is %x\n"), val);
        regulator_bulk_disable(ARRAY_SIZE!((*wm8904).supplies), (*wm8904).supplies.as_mut_ptr());
        return -EINVAL;
    }
    ret = regmap_read((*wm8904).regmap, WM8904_REVISION, &mut val);
    if ret < 0 {
        dev_err(&mut (*i2c).dev as *mut _ as *mut device, cstr!("Failed to read device revision: %d\n"), ret);
        regulator_bulk_disable(ARRAY_SIZE!((*wm8904).supplies), (*wm8904).supplies.as_mut_ptr());
        return ret;
    }
    dev_info(&mut (*i2c).dev as *mut _ as *mut device, cstr!("revision %c\n"), val + b'A' as c_uint);
    ret = regmap_write((*wm8904).regmap, WM8904_SW_RESET_AND_ID, 0);
    if ret < 0 {
        dev_err(&mut (*i2c).dev as *mut _ as *mut device, cstr!("Failed to issue reset: %d\n"), ret);
        regulator_bulk_disable(ARRAY_SIZE!((*wm8904).supplies), (*wm8904).supplies.as_mut_ptr());
        return ret;
    }
    regmap_update_bits((*wm8904).regmap, WM8904_ADC_DIGITAL_VOLUME_LEFT, WM8904_ADC_VU, WM8904_ADC_VU);
    regmap_update_bits((*wm8904).regmap, WM8904_ADC_DIGITAL_VOLUME_RIGHT, WM8904_ADC_VU, WM8904_ADC_VU);
    regmap_update_bits((*wm8904).regmap, WM8904_DAC_DIGITAL_VOLUME_LEFT, WM8904_DAC_VU, WM8904_DAC_VU);
    regmap_update_bits((*wm8904).regmap, WM8904_DAC_DIGITAL_VOLUME_RIGHT, WM8904_DAC_VU, WM8904_DAC_VU);
    regmap_update_bits((*wm8904).regmap, WM8904_ANALOGUE_OUT1_LEFT, WM8904_HPOUT_VU | WM8904_HPOUTLZC, WM8904_HPOUT_VU | WM8904_HPOUTLZC);
    regmap_update_bits((*wm8904).regmap, WM8904_ANALOGUE_OUT1_RIGHT, WM8904_HPOUT_VU | WM8904_HPOUTRZC, WM8904_HPOUT_VU | WM8904_HPOUTRZC);
    regmap_update_bits((*wm8904).regmap, WM8904_ANALOGUE_OUT2_LEFT, WM8904_LINEOUT_VU | WM8904_LINEOUTLZC, WM8904_LINEOUT_VU | WM8904_LINEOUTLZC);
    regmap_update_bits((*wm8904).regmap, WM8904_ANALOGUE_OUT2_RIGHT, WM8904_LINEOUT_VU | WM8904_LINEOUTRZC, WM8904_LINEOUT_VU | WM8904_LINEOUTRZC);
    regmap_update_bits((*wm8904).regmap, WM8904_CLOCK_RATES_0, WM8904_SR_MODE, 0);
    if !(*wm8904).pdata.is_null() {
        for i in 0..WM8904_GPIO_REGS {
            if (*(*wm8904).pdata).gpio_cfg[i as usize] == 0xffff { continue; }
            regmap_update_bits((*wm8904).regmap, WM8904_GPIO_CONTROL_1 + i as c_uint, 0xffff, (*(*wm8904).pdata).gpio_cfg[i as usize]);
        }
        for i in 0..WM8904_MIC_REGS {
            regmap_update_bits((*wm8904).regmap, WM8904_MIC_BIAS_CONTROL_0 + i as c_uint, 0xffff, (*(*wm8904).pdata).mic_cfg[i as usize]);
        }
    }
    regmap_update_bits((*wm8904).regmap, WM8904_CLASS_W_0, WM8904_CP_DYN_PWR, WM8904_CP_DYN_PWR);
    regmap_update_bits((*wm8904).regmap, WM8904_BIAS_CONTROL_0, WM8904_POBCTRL, 0);
    regmap_read((*wm8904).regmap, WM8904_ADC_TEST_0, &mut val);
    regcache_cache_only((*wm8904).regmap, true);
    regulator_bulk_disable(ARRAY_SIZE!((*wm8904).supplies), (*wm8904).supplies.as_mut_ptr());
    ret = devm_snd_soc_register_component(&mut (*i2c).dev as *mut _ as *mut device, &soc_component_dev_wm8904, &mut wm8904_dai, 1);
    if ret != 0 { return ret; }
    0
}

const WM8904_RATES: c_uint = 0; /* SNDRV_PCM_RATE_8000_96000 */
const WM8904_FORMATS: c_uint = 0; /* SNDRV_PCM_FMTBIT_S16_LE | S20_3LE | S24_LE | S32_LE */

static mut wm8904_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops { _private: [] };
static mut wm8904_dai: snd_soc_dai_driver = snd_soc_dai_driver { _private: [] };
static soc_component_dev_wm8904: snd_soc_component_driver = snd_soc_component_driver { _private: [] };
static wm8904_regmap: regmap_config = regmap_config { _private: [] };

/* CONFIG_OF: of_device_id wm8904_of_match[] = {
 * { .compatible = "wlf,wm8904", .data = (void *)WM8904 },
 * { .compatible = "wlf,wm8912", .data = (void *)WM8912 },
 * { sentinel }
 * };
 */

static wm8904_i2c_id: [i2c_device_id; 4] = [
    i2c_device_id { name: [0; 20], driver_data: wm8904_type::WM8904 as c_ulong },
    i2c_device_id { name: [0; 20], driver_data: wm8904_type::WM8912 as c_ulong },
    i2c_device_id { name: [0; 20], driver_data: wm8904_type::WM8904 as c_ulong },
    i2c_device_id { name: [0; 20], driver_data: 0 },
];
static mut wm8904_i2c_driver: i2c_driver = i2c_driver { _private: [] };

/* module_i2c_driver(wm8904_i2c_driver);
 * MODULE_DESCRIPTION("ASoC WM8904 driver");
 * MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>");
 * MODULE_LICENSE("GPL");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
