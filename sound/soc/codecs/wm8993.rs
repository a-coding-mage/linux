// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8993.rs -- WM8993 ALSA SoC audio driver
 *
 * Rust source-level translation of wm8993.c.
 *
 * C header dependencies intentionally remain external: linux/module.h,
 * linux/i2c.h, linux/regmap.h, sound/soc.h, sound/wm8993.h, wm8993.h,
 * wm_hubs.h, and related kernel headers.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type bool_ = bool;
type u16 = u16;
type u64 = u64;
type irqreturn_t = c_uint;

const WM8993_NUM_SUPPLIES: usize = 6;
const FIXED_FLL_SIZE: u64 = ((1u64 << 16) * 10);
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const INT_MAX: c_int = c_int::MAX;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}
#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}
#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub set_pll: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int, c_uint, c_uint) -> c_int>,
    pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    pub no_capture_mute: c_uint,
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
    pub sig_bits: c_uint,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
}
#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
pub struct reg_sequence {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub cache_type: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}
#[repr(C)]
pub struct i2c_client {
    pub dev: device,
    pub irq: c_int,
}
#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
}
#[repr(C)]
pub struct i2c_driver_inner {
    pub name: *const c_char,
}
#[repr(C)]
pub struct i2c_driver {
    pub driver: i2c_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub id_table: *const i2c_device_id,
}
#[repr(C)]
pub struct wm_hubs_data {
    pub hp_startup_mode: c_int,
    pub dcs_codes_l: c_int,
    pub dcs_codes_r: c_int,
    pub series_startup: c_int,
}
#[repr(C)]
pub struct wm8993_retune_mobile_setting {
    pub name: *const c_char,
    pub rate: c_uint,
    pub config: [u16; 25],
}
#[repr(C)]
pub struct wm8993_platform_data {
    pub lineout1_diff: bool_,
    pub lineout2_diff: bool_,
    pub lineout1fb: c_uint,
    pub lineout2fb: c_uint,
    pub jd_scthr: c_uint,
    pub jd_thr: c_uint,
    pub micbias1_delay: c_uint,
    pub micbias2_delay: c_uint,
    pub micbias1_lvl: c_uint,
    pub micbias2_lvl: c_uint,
    pub num_retune_configs: c_int,
    pub retune_configs: *mut wm8993_retune_mobile_setting,
}
#[repr(C)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON = 0,
    SND_SOC_BIAS_PREPARE = 1,
    SND_SOC_BIAS_STANDBY = 2,
    SND_SOC_BIAS_OFF = 3,
}

#[repr(C)]
pub struct wm8993_priv {
    pub hubs_data: wm_hubs_data,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub supplies: [regulator_bulk_data; WM8993_NUM_SUPPLIES],
    pub pdata: wm8993_platform_data,
    pub fll_lock: completion,
    pub master: c_int,
    pub sysclk_source: c_int,
    pub tdm_slots: c_int,
    pub tdm_width: c_int,
    pub mclk_rate: c_uint,
    pub sysclk_rate: c_uint,
    pub fs: c_uint,
    pub bclk: c_uint,
    pub fll_fref: c_uint,
    pub fll_fout: c_uint,
    pub fll_src: c_int,
}

#[repr(C)]
struct clk_sys_rate_entry {
    ratio: c_int,
    clk_sys_rate: c_int,
}
#[repr(C)]
struct sample_rate_entry {
    rate: c_int,
    sample_rate: c_int,
}
#[repr(C)]
struct bclk_div_entry {
    div: c_int, /* *10 due to .5s */
    bclk_div: c_int,
}
#[repr(C)]
struct fll_div {
    fll_fratio: u16,
    fll_outdiv: u16,
    fll_clk_ref_div: u16,
    n: u16,
    k: u16,
}
#[repr(C)]
struct fll_fratio_entry {
    min: c_uint,
    max: c_uint,
    fll_fratio: u16,
    ratio: c_int,
}

unsafe extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn to_i2c_client(dev: *mut device) -> *mut i2c_client;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_soc_dapm_force_bias_level(dapm: *mut snd_soc_dapm_context, level: snd_soc_bias_level) -> c_int;
    fn snd_soc_dapm_set_idle_bias(dapm: *mut snd_soc_dapm_context, idle: bool_);
    fn snd_soc_add_component_controls(component: *mut snd_soc_component, controls: *const snd_kcontrol_new, num: c_uint) -> c_int;
    fn snd_soc_dapm_new_controls(dapm: *mut snd_soc_dapm_context, widgets: *const snd_soc_dapm_widget_desc, num: c_uint) -> c_int;
    fn snd_soc_dapm_add_routes(dapm: *mut snd_soc_dapm_context, routes: *const snd_soc_dapm_route, num: c_uint) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn regulator_bulk_enable(num: c_uint, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num: c_uint, consumers: *mut regulator_bulk_data) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn try_wait_for_completion(x: *mut completion) -> bool_;
    fn wait_for_completion_timeout(x: *mut completion, timeout: c_ulong) -> c_ulong;
    fn complete(x: *mut completion);
    fn init_completion(x: *mut completion);
    fn msleep(msecs: c_uint);
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_int) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_register_patch(map: *mut regmap, regs: *const reg_sequence, num_regs: c_int) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(i2c: *mut i2c_client) -> *mut c_void;
    fn devm_regulator_bulk_get(dev: *mut device, num: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn request_threaded_irq(irq: c_int, handler: *const c_void, thread_fn: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_uint, name: *const c_char, dev: *mut c_void) -> c_int;
    fn free_irq(irq: c_int, dev_id: *mut c_void);
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn wm_hubs_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int;
    fn wm_hubs_vmid_ena(component: *mut snd_soc_component);
    fn wm_hubs_handle_analogue_pdata(component: *mut snd_soc_component, lineout1_diff: bool_, lineout2_diff: bool_, lineout1fb: c_uint, lineout2fb: c_uint, jd_scthr: c_uint, jd_thr: c_uint, micbias1_delay: c_uint, micbias2_delay: c_uint, micbias1_lvl: c_uint, micbias2_lvl: c_uint);
    fn wm_hubs_add_analogue_controls(component: *mut snd_soc_component) -> c_int;
    fn wm_hubs_add_analogue_routes(component: *mut snd_soc_component, lineout1_diff: bool_, lineout2_diff: bool_) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_crit(dev: *mut device, fmt: *const c_char, ...);
}

unsafe extern "C" {
    static wm_hubs_spkmix_tlv: c_uint;
    static wm_hubs_hpl_mux: snd_kcontrol_new;
    static wm_hubs_hpr_mux: snd_kcontrol_new;
}

/* Register, bit, format, IRQ, regcache, PCM, and DAPM constants are supplied by external headers. */
unsafe extern "C" {
    static WM8993_SOFTWARE_RESET: c_uint;
    static WM8993_POWER_MANAGEMENT_1: c_uint;
    static WM8993_POWER_MANAGEMENT_2: c_uint;
    static WM8993_POWER_MANAGEMENT_3: c_uint;
    static WM8993_AUDIO_INTERFACE_1: c_uint;
    static WM8993_AUDIO_INTERFACE_2: c_uint;
    static WM8993_CLOCKING_1: c_uint;
    static WM8993_CLOCKING_2: c_uint;
    static WM8993_AUDIO_INTERFACE_3: c_uint;
    static WM8993_AUDIO_INTERFACE_4: c_uint;
    static WM8993_DAC_CTRL: c_uint;
    static WM8993_LEFT_DAC_DIGITAL_VOLUME: c_uint;
    static WM8993_RIGHT_DAC_DIGITAL_VOLUME: c_uint;
    static WM8993_DIGITAL_SIDE_TONE: c_uint;
    static WM8993_ADC_CTRL: c_uint;
    static WM8993_LEFT_ADC_DIGITAL_VOLUME: c_uint;
    static WM8993_RIGHT_ADC_DIGITAL_VOLUME: c_uint;
    static WM8993_GPIO_CTRL_1: c_uint;
    static WM8993_GPIO1: c_uint;
    static WM8993_IRQ_DEBOUNCE: c_uint;
    static WM8993_GPIOCTRL_2: c_uint;
    static WM8993_GPIO_POL: c_uint;
    static WM8993_LEFT_LINE_INPUT_1_2_VOLUME: c_uint;
    static WM8993_LEFT_LINE_INPUT_3_4_VOLUME: c_uint;
    static WM8993_RIGHT_LINE_INPUT_1_2_VOLUME: c_uint;
    static WM8993_RIGHT_LINE_INPUT_3_4_VOLUME: c_uint;
    static WM8993_LEFT_OUTPUT_VOLUME: c_uint;
    static WM8993_RIGHT_OUTPUT_VOLUME: c_uint;
    static WM8993_LINE_OUTPUTS_VOLUME: c_uint;
    static WM8993_HPOUT2_VOLUME: c_uint;
    static WM8993_LEFT_OPGA_VOLUME: c_uint;
    static WM8993_RIGHT_OPGA_VOLUME: c_uint;
    static WM8993_SPKMIXL_ATTENUATION: c_uint;
    static WM8993_SPKMIXR_ATTENUATION: c_uint;
    static WM8993_SPKOUT_MIXERS: c_uint;
    static WM8993_SPKOUT_BOOST: c_uint;
    static WM8993_SPEAKER_VOLUME_LEFT: c_uint;
    static WM8993_SPEAKER_VOLUME_RIGHT: c_uint;
    static WM8993_INPUT_MIXER2: c_uint;
    static WM8993_INPUT_MIXER3: c_uint;
    static WM8993_INPUT_MIXER4: c_uint;
    static WM8993_INPUT_MIXER5: c_uint;
    static WM8993_INPUT_MIXER6: c_uint;
    static WM8993_OUTPUT_MIXER1: c_uint;
    static WM8993_OUTPUT_MIXER2: c_uint;
    static WM8993_OUTPUT_MIXER3: c_uint;
    static WM8993_OUTPUT_MIXER4: c_uint;
    static WM8993_OUTPUT_MIXER5: c_uint;
    static WM8993_OUTPUT_MIXER6: c_uint;
    static WM8993_HPOUT2_MIXER: c_uint;
    static WM8993_LINE_MIXER1: c_uint;
    static WM8993_LINE_MIXER2: c_uint;
    static WM8993_SPEAKER_MIXER: c_uint;
    static WM8993_ADDITIONAL_CONTROL: c_uint;
    static WM8993_ANTIPOP1: c_uint;
    static WM8993_ANTIPOP2: c_uint;
    static WM8993_MICBIAS: c_uint;
    static WM8993_FLL_CONTROL_1: c_uint;
    static WM8993_FLL_CONTROL_2: c_uint;
    static WM8993_FLL_CONTROL_3: c_uint;
    static WM8993_FLL_CONTROL_4: c_uint;
    static WM8993_FLL_CONTROL_5: c_uint;
    static WM8993_CLOCKING_3: c_uint;
    static WM8993_CLOCKING_4: c_uint;
    static WM8993_MW_SLAVE_CONTROL: c_uint;
    static WM8993_BUS_CONTROL_1: c_uint;
    static WM8993_WRITE_SEQUENCER_0: c_uint;
    static WM8993_WRITE_SEQUENCER_1: c_uint;
    static WM8993_WRITE_SEQUENCER_2: c_uint;
    static WM8993_WRITE_SEQUENCER_3: c_uint;
    static WM8993_WRITE_SEQUENCER_4: c_uint;
    static WM8993_WRITE_SEQUENCER_5: c_uint;
    static WM8993_CHARGE_PUMP_1: c_uint;
    static WM8993_CLASS_W_0: c_uint;
    static WM8993_DC_SERVO_0: c_uint;
    static WM8993_DC_SERVO_1: c_uint;
    static WM8993_DC_SERVO_3: c_uint;
    static WM8993_DC_SERVO_READBACK_0: c_uint;
    static WM8993_DC_SERVO_READBACK_1: c_uint;
    static WM8993_DC_SERVO_READBACK_2: c_uint;
    static WM8993_ANALOGUE_HP_0: c_uint;
    static WM8993_EQ1: c_uint;
    static WM8993_EQ2: c_uint;
    static WM8993_EQ3: c_uint;
    static WM8993_EQ4: c_uint;
    static WM8993_EQ5: c_uint;
    static WM8993_EQ6: c_uint;
    static WM8993_EQ7: c_uint;
    static WM8993_EQ8: c_uint;
    static WM8993_EQ9: c_uint;
    static WM8993_EQ10: c_uint;
    static WM8993_EQ11: c_uint;
    static WM8993_EQ12: c_uint;
    static WM8993_EQ13: c_uint;
    static WM8993_EQ14: c_uint;
    static WM8993_EQ15: c_uint;
    static WM8993_EQ16: c_uint;
    static WM8993_EQ17: c_uint;
    static WM8993_EQ18: c_uint;
    static WM8993_EQ19: c_uint;
    static WM8993_EQ20: c_uint;
    static WM8993_EQ21: c_uint;
    static WM8993_EQ22: c_uint;
    static WM8993_EQ23: c_uint;
    static WM8993_EQ24: c_uint;
    static WM8993_DIGITAL_PULLS: c_uint;
    static WM8993_DRC_CONTROL_1: c_uint;
    static WM8993_DRC_CONTROL_2: c_uint;
    static WM8993_DRC_CONTROL_3: c_uint;
    static WM8993_DRC_CONTROL_4: c_uint;
    static WM8993_FLL_ENA: c_uint;
    static WM8993_FLL_CLK_SRC_MASK: c_uint;
    static WM8993_FLL_FRAC_MASK: c_uint;
    static WM8993_FLL_OUTDIV_SHIFT: c_uint;
    static WM8993_FLL_FRATIO_SHIFT: c_uint;
    static WM8993_FLL_N_MASK: c_uint;
    static WM8993_FLL_N_SHIFT: c_uint;
    static WM8993_FLL_CLK_REF_DIV_MASK: c_uint;
    static WM8993_FLL_CLK_REF_DIV_SHIFT: c_uint;
    static WM8993_FLL_MCLK: c_uint;
    static WM8993_FLL_LRCLK: c_uint;
    static WM8993_FLL_BCLK: c_uint;
    static WM8993_SYSCLK_MCLK: c_uint;
    static WM8993_SYSCLK_FLL: c_uint;
    static WM8993_MCLK_DIV: c_uint;
    static WM8993_SYSCLK_SRC: c_uint;
    static WM8993_VMID_SEL_MASK: c_uint;
    static WM8993_TSHUT_ENA: c_uint;
    static WM8993_STARTUP_BIAS_ENA: c_uint;
    static WM8993_VMID_BUF_ENA: c_uint;
    static WM8993_VMID_RAMP_MASK: c_uint;
    static WM8993_BIAS_SRC: c_uint;
    static WM8993_LINEOUT_VMID_BUF_ENA: c_uint;
    static WM8993_BIAS_ENA: c_uint;
    static WM8993_BCLK_DIR: c_uint;
    static WM8993_AIF_BCLK_INV: c_uint;
    static WM8993_AIF_LRCLK_INV: c_uint;
    static WM8993_AIF_FMT_MASK: c_uint;
    static WM8993_LRCLK_DIR: c_uint;
    static WM8993_BCLK_DIV_MASK: c_uint;
    static WM8993_CLK_SYS_RATE_MASK: c_uint;
    static WM8993_SAMPLE_RATE_MASK: c_uint;
    static WM8993_AIF_WL_MASK: c_uint;
    static WM8993_LRCLK_RATE_MASK: c_uint;
    static WM8993_CLK_SYS_RATE_SHIFT: c_uint;
    static WM8993_SAMPLE_RATE_SHIFT: c_uint;
    static WM8993_BCLK_DIV_SHIFT: c_uint;
    static WM8993_EQ_ENA: c_uint;
    static WM8993_DAC_MUTE: c_uint;
    static WM8993_AIFADC_TDM: c_uint;
    static WM8993_AIFDAC_TDM: c_uint;
    static WM8993_AIFADC_TDM_CHAN: c_uint;
    static WM8993_AIFDAC_TDM_CHAN: c_uint;
    static WM8993_IRQ: c_int;
    static WM8993_TEMPOK_EINT: c_int;
    static WM8993_FLL_LOCK_EINT: c_int;
    static WM8993_DAC_VU: c_uint;
    static WM8993_ADC_VU: c_uint;
    static WM8993_HPOUT1_AUTO_PU: c_uint;
    static WM8993_SR_MODE: c_uint;
    static WM8993_MAX_REGISTER: c_uint;
    static WM8993_GPIO1_PD: c_uint;
    static WM8993_GPIO1_SEL_MASK: c_uint;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
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
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
    static REGCACHE_MAPLE: c_uint;
    static GFP_KERNEL: c_uint;
    static IRQ_NONE: irqreturn_t;
    static IRQ_HANDLED: irqreturn_t;
    static IRQF_TRIGGER_HIGH: c_uint;
    static IRQF_ONESHOT: c_uint;
}

static wm8993_supply_names: [*const c_char; WM8993_NUM_SUPPLIES] = [
    b"DCVDD\0".as_ptr() as *const c_char,
    b"DBVDD\0".as_ptr() as *const c_char,
    b"AVDD1\0".as_ptr() as *const c_char,
    b"AVDD2\0".as_ptr() as *const c_char,
    b"CPVDD\0".as_ptr() as *const c_char,
    b"SPKVDD\0".as_ptr() as *const c_char,
];

static wm8993_reg_defaults: [reg_default; 100] = [
    reg_default { reg: 1, def: 0x0000 }, reg_default { reg: 2, def: 0x6000 },
    reg_default { reg: 3, def: 0x0000 }, reg_default { reg: 4, def: 0x4050 },
    reg_default { reg: 5, def: 0x4000 }, reg_default { reg: 6, def: 0x01C8 },
    reg_default { reg: 7, def: 0x0000 }, reg_default { reg: 8, def: 0x0000 },
    reg_default { reg: 9, def: 0x0040 }, reg_default { reg: 10, def: 0x0004 },
    reg_default { reg: 11, def: 0x00C0 }, reg_default { reg: 12, def: 0x00C0 },
    reg_default { reg: 13, def: 0x0000 }, reg_default { reg: 14, def: 0x0300 },
    reg_default { reg: 15, def: 0x00C0 }, reg_default { reg: 16, def: 0x00C0 },
    reg_default { reg: 18, def: 0x0000 }, reg_default { reg: 19, def: 0x0010 },
    reg_default { reg: 20, def: 0x0000 }, reg_default { reg: 21, def: 0x0000 },
    reg_default { reg: 22, def: 0x8000 }, reg_default { reg: 23, def: 0x0800 },
    reg_default { reg: 24, def: 0x008B }, reg_default { reg: 25, def: 0x008B },
    reg_default { reg: 26, def: 0x008B }, reg_default { reg: 27, def: 0x008B },
    reg_default { reg: 28, def: 0x006D }, reg_default { reg: 29, def: 0x006D },
    reg_default { reg: 30, def: 0x0066 }, reg_default { reg: 31, def: 0x0020 },
    reg_default { reg: 32, def: 0x0079 }, reg_default { reg: 33, def: 0x0079 },
    reg_default { reg: 34, def: 0x0003 }, reg_default { reg: 35, def: 0x0003 },
    reg_default { reg: 36, def: 0x0011 }, reg_default { reg: 37, def: 0x0100 },
    reg_default { reg: 38, def: 0x0079 }, reg_default { reg: 39, def: 0x0079 },
    reg_default { reg: 40, def: 0x0000 }, reg_default { reg: 41, def: 0x0000 },
    reg_default { reg: 42, def: 0x0000 }, reg_default { reg: 43, def: 0x0000 },
    reg_default { reg: 44, def: 0x0000 }, reg_default { reg: 45, def: 0x0000 },
    reg_default { reg: 46, def: 0x0000 }, reg_default { reg: 47, def: 0x0000 },
    reg_default { reg: 48, def: 0x0000 }, reg_default { reg: 49, def: 0x0000 },
    reg_default { reg: 50, def: 0x0000 }, reg_default { reg: 51, def: 0x0000 },
    reg_default { reg: 52, def: 0x0000 }, reg_default { reg: 53, def: 0x0000 },
    reg_default { reg: 54, def: 0x0000 }, reg_default { reg: 55, def: 0x0000 },
    reg_default { reg: 56, def: 0x0000 }, reg_default { reg: 57, def: 0x0000 },
    reg_default { reg: 58, def: 0x0000 }, reg_default { reg: 60, def: 0x0000 },
    reg_default { reg: 61, def: 0x0000 }, reg_default { reg: 62, def: 0x0000 },
    reg_default { reg: 63, def: 0x2EE0 }, reg_default { reg: 64, def: 0x0002 },
    reg_default { reg: 65, def: 0x2287 }, reg_default { reg: 66, def: 0x025F },
    reg_default { reg: 67, def: 0x0000 }, reg_default { reg: 69, def: 0x0002 },
    reg_default { reg: 70, def: 0x0000 }, reg_default { reg: 71, def: 0x0000 },
    reg_default { reg: 72, def: 0x0000 }, reg_default { reg: 73, def: 0x0000 },
    reg_default { reg: 74, def: 0x0000 }, reg_default { reg: 75, def: 0x0000 },
    reg_default { reg: 76, def: 0x1F25 }, reg_default { reg: 81, def: 0x0000 },
    reg_default { reg: 85, def: 0x054A }, reg_default { reg: 87, def: 0x0000 },
    reg_default { reg: 96, def: 0x0100 }, reg_default { reg: 98, def: 0x0000 },
    reg_default { reg: 99, def: 0x000C }, reg_default { reg: 100, def: 0x000C },
    reg_default { reg: 101, def: 0x000C }, reg_default { reg: 102, def: 0x000C },
    reg_default { reg: 103, def: 0x000C }, reg_default { reg: 104, def: 0x0FCA },
    reg_default { reg: 105, def: 0x0400 }, reg_default { reg: 106, def: 0x00D8 },
    reg_default { reg: 107, def: 0x1EB5 }, reg_default { reg: 108, def: 0xF145 },
    reg_default { reg: 109, def: 0x0B75 }, reg_default { reg: 110, def: 0x01C5 },
    reg_default { reg: 111, def: 0x1C58 }, reg_default { reg: 112, def: 0xF373 },
    reg_default { reg: 113, def: 0x0A54 }, reg_default { reg: 114, def: 0x0558 },
    reg_default { reg: 115, def: 0x168E }, reg_default { reg: 116, def: 0xF829 },
    reg_default { reg: 117, def: 0x07AD }, reg_default { reg: 118, def: 0x1103 },
    reg_default { reg: 119, def: 0x0564 }, reg_default { reg: 120, def: 0x0559 },
    reg_default { reg: 121, def: 0x4000 }, reg_default { reg: 122, def: 0x0000 },
    reg_default { reg: 123, def: 0x0F08 }, reg_default { reg: 124, def: 0x0000 },
    reg_default { reg: 125, def: 0x0080 }, reg_default { reg: 126, def: 0x0000 },
];

static mut clk_sys_rates: [clk_sys_rate_entry; 10] = [
    clk_sys_rate_entry { ratio: 64, clk_sys_rate: 0 },
    clk_sys_rate_entry { ratio: 128, clk_sys_rate: 1 },
    clk_sys_rate_entry { ratio: 192, clk_sys_rate: 2 },
    clk_sys_rate_entry { ratio: 256, clk_sys_rate: 3 },
    clk_sys_rate_entry { ratio: 384, clk_sys_rate: 4 },
    clk_sys_rate_entry { ratio: 512, clk_sys_rate: 5 },
    clk_sys_rate_entry { ratio: 768, clk_sys_rate: 6 },
    clk_sys_rate_entry { ratio: 1024, clk_sys_rate: 7 },
    clk_sys_rate_entry { ratio: 1408, clk_sys_rate: 8 },
    clk_sys_rate_entry { ratio: 1536, clk_sys_rate: 9 },
];

static mut sample_rates: [sample_rate_entry; 9] = [
    sample_rate_entry { rate: 8000, sample_rate: 0 },
    sample_rate_entry { rate: 11025, sample_rate: 1 },
    sample_rate_entry { rate: 12000, sample_rate: 1 },
    sample_rate_entry { rate: 16000, sample_rate: 2 },
    sample_rate_entry { rate: 22050, sample_rate: 3 },
    sample_rate_entry { rate: 24000, sample_rate: 3 },
    sample_rate_entry { rate: 32000, sample_rate: 4 },
    sample_rate_entry { rate: 44100, sample_rate: 5 },
    sample_rate_entry { rate: 48000, sample_rate: 5 },
];

static mut bclk_divs: [bclk_div_entry; 16] = [
    bclk_div_entry { div: 10, bclk_div: 0 }, bclk_div_entry { div: 15, bclk_div: 1 },
    bclk_div_entry { div: 20, bclk_div: 2 }, bclk_div_entry { div: 30, bclk_div: 3 },
    bclk_div_entry { div: 40, bclk_div: 4 }, bclk_div_entry { div: 55, bclk_div: 5 },
    bclk_div_entry { div: 60, bclk_div: 6 }, bclk_div_entry { div: 80, bclk_div: 7 },
    bclk_div_entry { div: 110, bclk_div: 8 }, bclk_div_entry { div: 120, bclk_div: 9 },
    bclk_div_entry { div: 160, bclk_div: 10 }, bclk_div_entry { div: 220, bclk_div: 11 },
    bclk_div_entry { div: 240, bclk_div: 12 }, bclk_div_entry { div: 320, bclk_div: 13 },
    bclk_div_entry { div: 440, bclk_div: 14 }, bclk_div_entry { div: 480, bclk_div: 15 },
];

static mut fll_fratios: [fll_fratio_entry; 5] = [
    fll_fratio_entry { min: 0, max: 64000, fll_fratio: 4, ratio: 16 },
    fll_fratio_entry { min: 64000, max: 128000, fll_fratio: 3, ratio: 8 },
    fll_fratio_entry { min: 128000, max: 256000, fll_fratio: 2, ratio: 4 },
    fll_fratio_entry { min: 256000, max: 1000000, fll_fratio: 1, ratio: 2 },
    fll_fratio_entry { min: 1000000, max: 13500000, fll_fratio: 0, ratio: 1 },
];

unsafe extern "C" fn wm8993_volatile(_dev: *mut device, reg: c_uint) -> bool_ {
    reg == WM8993_SOFTWARE_RESET || reg == WM8993_GPIO_CTRL_1 ||
    reg == WM8993_DC_SERVO_0 || reg == WM8993_DC_SERVO_READBACK_0 ||
    reg == WM8993_DC_SERVO_READBACK_1 || reg == WM8993_DC_SERVO_READBACK_2
}

unsafe extern "C" fn wm8993_readable(_dev: *mut device, reg: c_uint) -> bool_ {
    matches!(reg,
        WM8993_SOFTWARE_RESET | WM8993_POWER_MANAGEMENT_1 | WM8993_POWER_MANAGEMENT_2 |
        WM8993_POWER_MANAGEMENT_3 | WM8993_AUDIO_INTERFACE_1 | WM8993_AUDIO_INTERFACE_2 |
        WM8993_CLOCKING_1 | WM8993_CLOCKING_2 | WM8993_AUDIO_INTERFACE_3 |
        WM8993_AUDIO_INTERFACE_4 | WM8993_DAC_CTRL | WM8993_LEFT_DAC_DIGITAL_VOLUME |
        WM8993_RIGHT_DAC_DIGITAL_VOLUME | WM8993_DIGITAL_SIDE_TONE | WM8993_ADC_CTRL |
        WM8993_LEFT_ADC_DIGITAL_VOLUME | WM8993_RIGHT_ADC_DIGITAL_VOLUME |
        WM8993_GPIO_CTRL_1 | WM8993_GPIO1 | WM8993_IRQ_DEBOUNCE | WM8993_GPIOCTRL_2 |
        WM8993_GPIO_POL | WM8993_LEFT_LINE_INPUT_1_2_VOLUME |
        WM8993_LEFT_LINE_INPUT_3_4_VOLUME | WM8993_RIGHT_LINE_INPUT_1_2_VOLUME |
        WM8993_RIGHT_LINE_INPUT_3_4_VOLUME | WM8993_LEFT_OUTPUT_VOLUME |
        WM8993_RIGHT_OUTPUT_VOLUME | WM8993_LINE_OUTPUTS_VOLUME | WM8993_HPOUT2_VOLUME |
        WM8993_LEFT_OPGA_VOLUME | WM8993_RIGHT_OPGA_VOLUME | WM8993_SPKMIXL_ATTENUATION |
        WM8993_SPKMIXR_ATTENUATION | WM8993_SPKOUT_MIXERS | WM8993_SPKOUT_BOOST |
        WM8993_SPEAKER_VOLUME_LEFT | WM8993_SPEAKER_VOLUME_RIGHT | WM8993_INPUT_MIXER2 |
        WM8993_INPUT_MIXER3 | WM8993_INPUT_MIXER4 | WM8993_INPUT_MIXER5 |
        WM8993_INPUT_MIXER6 | WM8993_OUTPUT_MIXER1 | WM8993_OUTPUT_MIXER2 |
        WM8993_OUTPUT_MIXER3 | WM8993_OUTPUT_MIXER4 | WM8993_OUTPUT_MIXER5 |
        WM8993_OUTPUT_MIXER6 | WM8993_HPOUT2_MIXER | WM8993_LINE_MIXER1 |
        WM8993_LINE_MIXER2 | WM8993_SPEAKER_MIXER | WM8993_ADDITIONAL_CONTROL |
        WM8993_ANTIPOP1 | WM8993_ANTIPOP2 | WM8993_MICBIAS | WM8993_FLL_CONTROL_1 |
        WM8993_FLL_CONTROL_2 | WM8993_FLL_CONTROL_3 | WM8993_FLL_CONTROL_4 |
        WM8993_FLL_CONTROL_5 | WM8993_CLOCKING_3 | WM8993_CLOCKING_4 |
        WM8993_MW_SLAVE_CONTROL | WM8993_BUS_CONTROL_1 | WM8993_WRITE_SEQUENCER_0 |
        WM8993_WRITE_SEQUENCER_1 | WM8993_WRITE_SEQUENCER_2 |
        WM8993_WRITE_SEQUENCER_3 | WM8993_WRITE_SEQUENCER_4 |
        WM8993_WRITE_SEQUENCER_5 | WM8993_CHARGE_PUMP_1 | WM8993_CLASS_W_0 |
        WM8993_DC_SERVO_0 | WM8993_DC_SERVO_1 | WM8993_DC_SERVO_3 |
        WM8993_DC_SERVO_READBACK_0 | WM8993_DC_SERVO_READBACK_1 |
        WM8993_DC_SERVO_READBACK_2 | WM8993_ANALOGUE_HP_0 | WM8993_EQ1 |
        WM8993_EQ2 | WM8993_EQ3 | WM8993_EQ4 | WM8993_EQ5 | WM8993_EQ6 |
        WM8993_EQ7 | WM8993_EQ8 | WM8993_EQ9 | WM8993_EQ10 | WM8993_EQ11 |
        WM8993_EQ12 | WM8993_EQ13 | WM8993_EQ14 | WM8993_EQ15 | WM8993_EQ16 |
        WM8993_EQ17 | WM8993_EQ18 | WM8993_EQ19 | WM8993_EQ20 | WM8993_EQ21 |
        WM8993_EQ22 | WM8993_EQ23 | WM8993_EQ24 | WM8993_DIGITAL_PULLS |
        WM8993_DRC_CONTROL_1 | WM8993_DRC_CONTROL_2 | WM8993_DRC_CONTROL_3 |
        WM8993_DRC_CONTROL_4)
}

unsafe extern "C" fn fll_factors(fll_div: *mut fll_div, mut Fref: c_uint, Fout: c_uint) -> c_int {
    let mut div: c_uint = 1;
    (*fll_div).fll_clk_ref_div = 0;
    while Fref / div > 13_500_000 {
        div *= 2;
        (*fll_div).fll_clk_ref_div += 1;
        if div > 8 {
            pr_err(b"Can't scale %dMHz input down to <=13.5MHz\n\0".as_ptr() as *const c_char, Fref);
            return -EINVAL;
        }
    }
    pr_debug(b"Fref=%u Fout=%u\n\0".as_ptr() as *const c_char, Fref, Fout);
    Fref /= div;
    div = 0;
    let mut target: c_uint = Fout.wrapping_mul(2);
    while target < 90_000_000 {
        div += 1;
        target = target.wrapping_mul(2);
        if div > 7 {
            pr_err(b"Unable to find FLL_OUTDIV for Fout=%uHz\n\0".as_ptr() as *const c_char, Fout);
            return -EINVAL;
        }
    }
    (*fll_div).fll_outdiv = div as u16;
    pr_debug(b"Fvco=%dHz\n\0".as_ptr() as *const c_char, target);

    let mut i: usize = 0;
    while i < fll_fratios.len() {
        if fll_fratios[i].min <= Fref && Fref <= fll_fratios[i].max {
            (*fll_div).fll_fratio = fll_fratios[i].fll_fratio;
            target /= fll_fratios[i].ratio as c_uint;
            break;
        }
        i += 1;
    }
    if i == fll_fratios.len() {
        pr_err(b"Unable to find FLL_FRATIO for Fref=%uHz\n\0".as_ptr() as *const c_char, Fref);
        return -EINVAL;
    }

    let Ndiv = target / Fref;
    (*fll_div).n = Ndiv as u16;
    let Nmod = target % Fref;
    pr_debug(b"Nmod=%d\n\0".as_ptr() as *const c_char, Nmod);

    let mut Kpart: u64 = FIXED_FLL_SIZE.wrapping_mul(Nmod as u64);
    Kpart /= Fref as u64;
    let mut K: c_uint = (Kpart & 0xFFFF_FFFF) as c_uint;
    if K % 10 >= 5 {
        K = K.wrapping_add(5);
    }
    (*fll_div).k = (K / 10) as u16;
    pr_debug(b"N=%x K=%x FLL_FRATIO=%x FLL_OUTDIV=%x FLL_CLK_REF_DIV=%x\n\0".as_ptr() as *const c_char,
        (*fll_div).n as c_int, (*fll_div).k as c_int, (*fll_div).fll_fratio as c_int,
        (*fll_div).fll_outdiv as c_int, (*fll_div).fll_clk_ref_div as c_int);
    0
}

unsafe extern "C" fn _wm8993_set_fll(component: *mut snd_soc_component, fll_id: c_int, source: c_int, Fref: c_uint, Fout: c_uint) -> c_int {
    let wm8993 = snd_soc_component_get_drvdata(component) as *mut wm8993_priv;
    let i2c = to_i2c_client((*component).dev);
    let mut fll_div: fll_div = core::mem::zeroed();
    if Fref == (*wm8993).fll_fref && Fout == (*wm8993).fll_fout {
        return 0;
    }
    if Fout == 0 {
        dev_dbg((*component).dev, b"FLL disabled\n\0".as_ptr() as *const c_char);
        (*wm8993).fll_fref = 0;
        (*wm8993).fll_fout = 0;
        let mut reg1 = snd_soc_component_read(component, WM8993_FLL_CONTROL_1);
        reg1 &= !WM8993_FLL_ENA;
        snd_soc_component_write(component, WM8993_FLL_CONTROL_1, reg1);
        return 0;
    }
    let ret = fll_factors(&mut fll_div, Fref, Fout);
    if ret != 0 {
        return ret;
    }
    let mut reg5 = snd_soc_component_read(component, WM8993_FLL_CONTROL_5);
    reg5 &= !WM8993_FLL_CLK_SRC_MASK;
    if fll_id as c_uint == WM8993_FLL_MCLK {
    } else if fll_id as c_uint == WM8993_FLL_LRCLK {
        reg5 |= 1;
    } else if fll_id as c_uint == WM8993_FLL_BCLK {
        reg5 |= 2;
    } else {
        dev_err((*component).dev, b"Unknown FLL ID %d\n\0".as_ptr() as *const c_char, fll_id);
        return -EINVAL;
    }
    let mut reg1 = snd_soc_component_read(component, WM8993_FLL_CONTROL_1);
    reg1 &= !WM8993_FLL_ENA;
    snd_soc_component_write(component, WM8993_FLL_CONTROL_1, reg1);
    if fll_div.k != 0 {
        reg1 |= WM8993_FLL_FRAC_MASK;
    } else {
        reg1 &= !WM8993_FLL_FRAC_MASK;
    }
    snd_soc_component_write(component, WM8993_FLL_CONTROL_1, reg1);
    snd_soc_component_write(component, WM8993_FLL_CONTROL_2,
        ((fll_div.fll_outdiv as c_uint) << WM8993_FLL_OUTDIV_SHIFT) |
        ((fll_div.fll_fratio as c_uint) << WM8993_FLL_FRATIO_SHIFT));
    snd_soc_component_write(component, WM8993_FLL_CONTROL_3, fll_div.k as c_uint);
    let mut reg4 = snd_soc_component_read(component, WM8993_FLL_CONTROL_4);
    reg4 &= !WM8993_FLL_N_MASK;
    reg4 |= (fll_div.n as c_uint) << WM8993_FLL_N_SHIFT;
    snd_soc_component_write(component, WM8993_FLL_CONTROL_4, reg4);
    reg5 &= !WM8993_FLL_CLK_REF_DIV_MASK;
    reg5 |= (fll_div.fll_clk_ref_div as c_uint) << WM8993_FLL_CLK_REF_DIV_SHIFT;
    snd_soc_component_write(component, WM8993_FLL_CONTROL_5, reg5);

    let mut time_left = if (*i2c).irq != 0 {
        msecs_to_jiffies(20)
    } else if Fref < 1_000_000 {
        msecs_to_jiffies(3)
    } else {
        msecs_to_jiffies(1)
    };
    try_wait_for_completion(&mut (*wm8993).fll_lock);
    snd_soc_component_write(component, WM8993_FLL_CONTROL_1, reg1 | WM8993_FLL_ENA);
    time_left = wait_for_completion_timeout(&mut (*wm8993).fll_lock, time_left);
    if (*i2c).irq != 0 && time_left == 0 {
        dev_warn((*component).dev, b"Timed out waiting for FLL\n\0".as_ptr() as *const c_char);
    }
    dev_dbg((*component).dev, b"FLL enabled at %dHz->%dHz\n\0".as_ptr() as *const c_char, Fref, Fout);
    (*wm8993).fll_fref = Fref;
    (*wm8993).fll_fout = Fout;
    (*wm8993).fll_src = source;
    0
}

unsafe extern "C" fn wm8993_set_fll(dai: *mut snd_soc_dai, fll_id: c_int, source: c_int, Fref: c_uint, Fout: c_uint) -> c_int {
    _wm8993_set_fll((*dai).component, fll_id, source, Fref, Fout)
}

unsafe extern "C" fn configure_clock(component: *mut snd_soc_component) -> c_int {
    let wm8993 = snd_soc_component_get_drvdata(component) as *mut wm8993_priv;
    if (*wm8993).sysclk_source as c_uint == WM8993_SYSCLK_MCLK {
        dev_dbg((*component).dev, b"Using %dHz MCLK\n\0".as_ptr() as *const c_char, (*wm8993).mclk_rate);
        let mut reg = snd_soc_component_read(component, WM8993_CLOCKING_2);
        reg &= !(WM8993_MCLK_DIV | WM8993_SYSCLK_SRC);
        if (*wm8993).mclk_rate > 13_500_000 {
            reg |= WM8993_MCLK_DIV;
            (*wm8993).sysclk_rate = (*wm8993).mclk_rate / 2;
        } else {
            reg &= !WM8993_MCLK_DIV;
            (*wm8993).sysclk_rate = (*wm8993).mclk_rate;
        }
        snd_soc_component_write(component, WM8993_CLOCKING_2, reg);
    } else if (*wm8993).sysclk_source as c_uint == WM8993_SYSCLK_FLL {
        dev_dbg((*component).dev, b"Using %dHz FLL clock\n\0".as_ptr() as *const c_char, (*wm8993).fll_fout);
        let mut reg = snd_soc_component_read(component, WM8993_CLOCKING_2);
        reg |= WM8993_SYSCLK_SRC;
        if (*wm8993).fll_fout > 13_500_000 {
            reg |= WM8993_MCLK_DIV;
            (*wm8993).sysclk_rate = (*wm8993).fll_fout / 2;
        } else {
            reg &= !WM8993_MCLK_DIV;
            (*wm8993).sysclk_rate = (*wm8993).fll_fout;
        }
        snd_soc_component_write(component, WM8993_CLOCKING_2, reg);
    } else {
        dev_err((*component).dev, b"System clock not configured\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    dev_dbg((*component).dev, b"CLK_SYS is %dHz\n\0".as_ptr() as *const c_char, (*wm8993).sysclk_rate);
    0
}

/* TLV, SOC_ENUM, SOC_* control, and SND_SOC_DAPM_* macro expansions depend on external ASoC C macros.
 * The original file declares:
 * sidetone_tlv, drc_comp_threash, drc_comp_amp, drc_min_tlv, drc_max_tlv,
 * drc_qr_tlv, drc_startup_tlv, eq_tlv, digital_tlv, dac_boost_tlv;
 * dac_deemph_text/dac_deemph, adc_hpf_text/adc_hpf, drc_* enums;
 * wm8993_snd_controls, wm8993_eq_controls, left_speaker_mixer,
 * right_speaker_mixer, aif* muxes, sidetone muxes, and wm8993_dapm_widgets.
 */
static wm8993_snd_controls: [snd_kcontrol_new; 0] = [];
static wm8993_eq_controls: [snd_kcontrol_new; 0] = [];
static wm8993_dapm_widgets: [snd_soc_dapm_widget_desc; 0] = [];

unsafe extern "C" fn clk_sys_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    if event == SND_SOC_DAPM_PRE_PMU {
        return configure_clock(component);
    }
    if event == SND_SOC_DAPM_POST_PMD {
    }
    0
}

static routes: [snd_soc_dapm_route; 43] = [
    snd_soc_dapm_route { sink: b"MICBIAS1\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"VMID\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"MICBIAS2\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"VMID\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADCL\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"CLK_SYS\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADCL\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"CLK_DSP\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADCR\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"CLK_SYS\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADCR\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"CLK_DSP\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"AIFOUTL Mux\0".as_ptr() as *const c_char, control: b"Left\0".as_ptr() as *const c_char, source: b"ADCL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"AIFOUTL Mux\0".as_ptr() as *const c_char, control: b"Right\0".as_ptr() as *const c_char, source: b"ADCR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"AIFOUTR Mux\0".as_ptr() as *const c_char, control: b"Left\0".as_ptr() as *const c_char, source: b"ADCL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"AIFOUTR Mux\0".as_ptr() as *const c_char, control: b"Right\0".as_ptr() as *const c_char, source: b"ADCR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"AIFOUTL\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"AIFOUTL Mux\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"AIFOUTR\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"AIFOUTR Mux\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DACL Mux\0".as_ptr() as *const c_char, control: b"Left\0".as_ptr() as *const c_char, source: b"AIFINL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DACL Mux\0".as_ptr() as *const c_char, control: b"Right\0".as_ptr() as *const c_char, source: b"AIFINR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DACR Mux\0".as_ptr() as *const c_char, control: b"Left\0".as_ptr() as *const c_char, source: b"AIFINL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DACR Mux\0".as_ptr() as *const c_char, control: b"Right\0".as_ptr() as *const c_char, source: b"AIFINR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DACL Sidetone\0".as_ptr() as *const c_char, control: b"Left\0".as_ptr() as *const c_char, source: b"ADCL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DACL Sidetone\0".as_ptr() as *const c_char, control: b"Right\0".as_ptr() as *const c_char, source: b"ADCR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DACR Sidetone\0".as_ptr() as *const c_char, control: b"Left\0".as_ptr() as *const c_char, source: b"ADCL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DACR Sidetone\0".as_ptr() as *const c_char, control: b"Right\0".as_ptr() as *const c_char, source: b"ADCR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DACL\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"CLK_SYS\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DACL\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"CLK_DSP\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DACL\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"DACL Mux\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DACL\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"DACL Sidetone\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DACR\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"CLK_SYS\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DACR\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"CLK_DSP\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DACR\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"DACR Mux\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DACR\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"DACR Sidetone\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Left Output Mixer\0".as_ptr() as *const c_char, control: b"DAC Switch\0".as_ptr() as *const c_char, source: b"DACL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right Output Mixer\0".as_ptr() as *const c_char, control: b"DAC Switch\0".as_ptr() as *const c_char, source: b"DACR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Left Output PGA\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"CLK_SYS\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right Output PGA\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"CLK_SYS\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SPKL\0".as_ptr() as *const c_char, control: b"DAC Switch\0".as_ptr() as *const c_char, source: b"DACL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SPKL\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"CLK_SYS\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SPKR\0".as_ptr() as *const c_char, control: b"DAC Switch\0".as_ptr() as *const c_char, source: b"DACR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SPKR\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"CLK_SYS\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Left Headphone Mux\0".as_ptr() as *const c_char, control: b"DAC\0".as_ptr() as *const c_char, source: b"DACL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right Headphone Mux\0".as_ptr() as *const c_char, control: b"DAC\0".as_ptr() as *const c_char, source: b"DACR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: core::ptr::null(), control: core::ptr::null(), source: core::ptr::null() },
    snd_soc_dapm_route { sink: core::ptr::null(), control: core::ptr::null(), source: core::ptr::null() },
    snd_soc_dapm_route { sink: core::ptr::null(), control: core::ptr::null(), source: core::ptr::null() },
    snd_soc_dapm_route { sink: core::ptr::null(), control: core::ptr::null(), source: core::ptr::null() },
    snd_soc_dapm_route { sink: core::ptr::null(), control: core::ptr::null(), source: core::ptr::null() },
];

unsafe extern "C" fn wm8993_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let wm8993 = snd_soc_component_get_drvdata(component) as *mut wm8993_priv;
    let dapm = snd_soc_component_to_dapm(component);
    wm_hubs_set_bias_level(component, level);
    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON | snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {
            snd_soc_component_update_bits(component, WM8993_POWER_MANAGEMENT_1, WM8993_VMID_SEL_MASK, 0x2);
            snd_soc_component_update_bits(component, WM8993_POWER_MANAGEMENT_2, WM8993_TSHUT_ENA, WM8993_TSHUT_ENA);
        }
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) as c_uint == snd_soc_bias_level::SND_SOC_BIAS_OFF as c_uint {
                let ret = regulator_bulk_enable((*wm8993).supplies.len() as c_uint, (*wm8993).supplies.as_mut_ptr());
                if ret != 0 { return ret; }
                regcache_cache_only((*wm8993).regmap, false);
                regcache_sync((*wm8993).regmap);
                wm_hubs_vmid_ena(component);
                snd_soc_component_update_bits(component, WM8993_ANTIPOP2,
                    WM8993_STARTUP_BIAS_ENA | WM8993_VMID_BUF_ENA | WM8993_VMID_RAMP_MASK | WM8993_BIAS_SRC,
                    WM8993_STARTUP_BIAS_ENA | WM8993_VMID_BUF_ENA | WM8993_VMID_RAMP_MASK | WM8993_BIAS_SRC);
                if !(*wm8993).pdata.lineout1_diff || !(*wm8993).pdata.lineout2_diff {
                    snd_soc_component_update_bits(component, WM8993_ANTIPOP1, WM8993_LINEOUT_VMID_BUF_ENA, WM8993_LINEOUT_VMID_BUF_ENA);
                }
                snd_soc_component_update_bits(component, WM8993_POWER_MANAGEMENT_1, WM8993_VMID_SEL_MASK | WM8993_BIAS_ENA, WM8993_BIAS_ENA | 0x2);
                msleep(32);
                snd_soc_component_update_bits(component, WM8993_ANTIPOP2, WM8993_BIAS_SRC | WM8993_STARTUP_BIAS_ENA, 0);
            }
            snd_soc_component_update_bits(component, WM8993_POWER_MANAGEMENT_1, WM8993_VMID_SEL_MASK, 0x4);
            snd_soc_component_update_bits(component, WM8993_POWER_MANAGEMENT_2, WM8993_TSHUT_ENA, 0);
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            snd_soc_component_update_bits(component, WM8993_ANTIPOP1, WM8993_LINEOUT_VMID_BUF_ENA, 0);
            snd_soc_component_update_bits(component, WM8993_POWER_MANAGEMENT_1, WM8993_VMID_SEL_MASK | WM8993_BIAS_ENA, 0);
            snd_soc_component_update_bits(component, WM8993_ANTIPOP2, WM8993_STARTUP_BIAS_ENA | WM8993_VMID_BUF_ENA | WM8993_VMID_RAMP_MASK | WM8993_BIAS_SRC, 0);
            regcache_cache_only((*wm8993).regmap, true);
            regcache_mark_dirty((*wm8993).regmap);
            regulator_bulk_disable((*wm8993).supplies.len() as c_uint, (*wm8993).supplies.as_mut_ptr());
        }
    }
    0
}

unsafe extern "C" fn wm8993_set_sysclk(codec_dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*codec_dai).component;
    let wm8993 = snd_soc_component_get_drvdata(component) as *mut wm8993_priv;
    if clk_id as c_uint == WM8993_SYSCLK_MCLK {
        (*wm8993).mclk_rate = freq;
        (*wm8993).sysclk_source = clk_id;
    } else if clk_id as c_uint == WM8993_SYSCLK_FLL {
        (*wm8993).sysclk_source = clk_id;
    } else {
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn wm8993_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let wm8993 = snd_soc_component_get_drvdata(component) as *mut wm8993_priv;
    let mut aif1 = snd_soc_component_read(component, WM8993_AUDIO_INTERFACE_1);
    let mut aif4 = snd_soc_component_read(component, WM8993_AUDIO_INTERFACE_4);
    aif1 &= !(WM8993_BCLK_DIR | WM8993_AIF_BCLK_INV | WM8993_AIF_LRCLK_INV | WM8993_AIF_FMT_MASK);
    aif4 &= !WM8993_LRCLK_DIR;
    let master = fmt & SND_SOC_DAIFMT_MASTER_MASK;
    if master == SND_SOC_DAIFMT_CBC_CFC { (*wm8993).master = 0; }
    else if master == SND_SOC_DAIFMT_CBC_CFP { aif4 |= WM8993_LRCLK_DIR; (*wm8993).master = 1; }
    else if master == SND_SOC_DAIFMT_CBP_CFC { aif1 |= WM8993_BCLK_DIR; (*wm8993).master = 1; }
    else if master == SND_SOC_DAIFMT_CBP_CFP { aif1 |= WM8993_BCLK_DIR; aif4 |= WM8993_LRCLK_DIR; (*wm8993).master = 1; }
    else { return -EINVAL; }
    let format = fmt & SND_SOC_DAIFMT_FORMAT_MASK;
    if format == SND_SOC_DAIFMT_DSP_B { aif1 |= WM8993_AIF_LRCLK_INV; aif1 |= 0x18; }
    else if format == SND_SOC_DAIFMT_DSP_A { aif1 |= 0x18; }
    else if format == SND_SOC_DAIFMT_I2S { aif1 |= 0x10; }
    else if format == SND_SOC_DAIFMT_RIGHT_J { }
    else if format == SND_SOC_DAIFMT_LEFT_J { aif1 |= 0x8; }
    else { return -EINVAL; }
    let inv = fmt & SND_SOC_DAIFMT_INV_MASK;
    if format == SND_SOC_DAIFMT_DSP_A || format == SND_SOC_DAIFMT_DSP_B {
        if inv == SND_SOC_DAIFMT_NB_NF { }
        else if inv == SND_SOC_DAIFMT_IB_NF { aif1 |= WM8993_AIF_BCLK_INV; }
        else { return -EINVAL; }
    } else {
        if inv == SND_SOC_DAIFMT_NB_NF { }
        else if inv == SND_SOC_DAIFMT_IB_IF { aif1 |= WM8993_AIF_BCLK_INV | WM8993_AIF_LRCLK_INV; }
        else if inv == SND_SOC_DAIFMT_IB_NF { aif1 |= WM8993_AIF_BCLK_INV; }
        else if inv == SND_SOC_DAIFMT_NB_IF { aif1 |= WM8993_AIF_LRCLK_INV; }
        else { return -EINVAL; }
    }
    snd_soc_component_write(component, WM8993_AUDIO_INTERFACE_1, aif1);
    snd_soc_component_write(component, WM8993_AUDIO_INTERFACE_4, aif4);
    0
}

unsafe extern "C" fn wm8993_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let wm8993 = snd_soc_component_get_drvdata(component) as *mut wm8993_priv;
    let mut clocking1 = snd_soc_component_read(component, WM8993_CLOCKING_1) & !WM8993_BCLK_DIV_MASK;
    let mut clocking3 = snd_soc_component_read(component, WM8993_CLOCKING_3) & !(WM8993_CLK_SYS_RATE_MASK | WM8993_SAMPLE_RATE_MASK);
    let mut aif1 = snd_soc_component_read(component, WM8993_AUDIO_INTERFACE_1) & !WM8993_AIF_WL_MASK;
    let mut aif4 = snd_soc_component_read(component, WM8993_AUDIO_INTERFACE_4) & !WM8993_LRCLK_RATE_MASK;
    (*wm8993).fs = params_rate(params);
    (*wm8993).bclk = 2u32.wrapping_mul((*wm8993).fs);
    if (*wm8993).tdm_slots != 0 {
        dev_dbg((*component).dev, b"Configuring for %d %d bit TDM slots\n\0".as_ptr() as *const c_char, (*wm8993).tdm_slots, (*wm8993).tdm_width);
        (*wm8993).bclk = (*wm8993).bclk.wrapping_mul(((*wm8993).tdm_width * (*wm8993).tdm_slots) as c_uint);
    } else {
        match params_width(params) {
            16 => (*wm8993).bclk = (*wm8993).bclk.wrapping_mul(16),
            20 => { (*wm8993).bclk = (*wm8993).bclk.wrapping_mul(20); aif1 |= 0x8; }
            24 => { (*wm8993).bclk = (*wm8993).bclk.wrapping_mul(24); aif1 |= 0x10; }
            32 => { (*wm8993).bclk = (*wm8993).bclk.wrapping_mul(32); aif1 |= 0x18; }
            _ => return -EINVAL,
        }
    }
    dev_dbg((*component).dev, b"Target BCLK is %dHz\n\0".as_ptr() as *const c_char, (*wm8993).bclk);
    let ret = configure_clock(component);
    if ret != 0 { return ret; }
    let mut best: usize = 0;
    let mut best_val = (((*wm8993).sysclk_rate / clk_sys_rates[0].ratio as c_uint) as c_int - (*wm8993).fs as c_int).abs();
    let mut i: usize = 1;
    while i < clk_sys_rates.len() {
        let cur_val = (((*wm8993).sysclk_rate / clk_sys_rates[i].ratio as c_uint) as c_int - (*wm8993).fs as c_int).abs();
        if cur_val < best_val { best = i; best_val = cur_val; }
        i += 1;
    }
    dev_dbg((*component).dev, b"Selected CLK_SYS_RATIO of %d\n\0".as_ptr() as *const c_char, clk_sys_rates[best].ratio);
    clocking3 |= (clk_sys_rates[best].clk_sys_rate as c_uint) << WM8993_CLK_SYS_RATE_SHIFT;
    best = 0;
    best_val = ((*wm8993).fs as c_int - sample_rates[0].rate).abs();
    i = 1;
    while i < sample_rates.len() {
        let cur_val = ((*wm8993).fs as c_int - sample_rates[i].rate).abs();
        if cur_val < best_val { best = i; best_val = cur_val; }
        i += 1;
    }
    dev_dbg((*component).dev, b"Selected SAMPLE_RATE of %dHz\n\0".as_ptr() as *const c_char, sample_rates[best].rate);
    clocking3 |= (sample_rates[best].sample_rate as c_uint) << WM8993_SAMPLE_RATE_SHIFT;
    best = 0;
    best_val = INT_MAX;
    i = 0;
    while i < bclk_divs.len() {
        let cur_val = (((*wm8993).sysclk_rate.wrapping_mul(10) / bclk_divs[i].div as c_uint) as c_int) - (*wm8993).bclk as c_int;
        if cur_val < 0 { break; }
        if cur_val < best_val { best = i; best_val = cur_val; }
        i += 1;
    }
    (*wm8993).bclk = (*wm8993).sysclk_rate.wrapping_mul(10) / bclk_divs[best].div as c_uint;
    dev_dbg((*component).dev, b"Selected BCLK_DIV of %d for %dHz BCLK\n\0".as_ptr() as *const c_char, bclk_divs[best].div, (*wm8993).bclk);
    clocking1 |= (bclk_divs[best].bclk_div as c_uint) << WM8993_BCLK_DIV_SHIFT;
    dev_dbg((*component).dev, b"LRCLK_RATE is %d\n\0".as_ptr() as *const c_char, (*wm8993).bclk / (*wm8993).fs);
    aif4 |= (*wm8993).bclk / (*wm8993).fs;
    snd_soc_component_write(component, WM8993_CLOCKING_1, clocking1);
    snd_soc_component_write(component, WM8993_CLOCKING_3, clocking3);
    snd_soc_component_write(component, WM8993_AUDIO_INTERFACE_1, aif1);
    snd_soc_component_write(component, WM8993_AUDIO_INTERFACE_4, aif4);
    if (*wm8993).pdata.num_retune_configs != 0 {
        let eq1 = snd_soc_component_read(component, WM8993_EQ1) as u16;
        best = 0;
        best_val = ((*(*wm8993).pdata.retune_configs.add(0)).rate as c_int - (*wm8993).fs as c_int).abs();
        i = 0;
        while i < (*wm8993).pdata.num_retune_configs as usize {
            let cur_val = ((*(*wm8993).pdata.retune_configs.add(i)).rate as c_int - (*wm8993).fs as c_int).abs();
            if cur_val < best_val { best_val = cur_val; best = i; }
            i += 1;
        }
        let s = (*wm8993).pdata.retune_configs.add(best);
        dev_dbg((*component).dev, b"ReTune Mobile %s tuned for %dHz\n\0".as_ptr() as *const c_char, (*s).name, (*s).rate);
        snd_soc_component_update_bits(component, WM8993_EQ1, WM8993_EQ_ENA, 0);
        i = 1;
        while i < (*s).config.len() {
            snd_soc_component_write(component, WM8993_EQ1 + i as c_uint, (*s).config[i] as c_uint);
            i += 1;
        }
        snd_soc_component_update_bits(component, WM8993_EQ1, WM8993_EQ_ENA, eq1 as c_uint);
    }
    0
}

unsafe extern "C" fn wm8993_mute(codec_dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*codec_dai).component;
    let mut reg = snd_soc_component_read(component, WM8993_DAC_CTRL);
    if mute != 0 { reg |= WM8993_DAC_MUTE; } else { reg &= !WM8993_DAC_MUTE; }
    snd_soc_component_write(component, WM8993_DAC_CTRL, reg);
    0
}

unsafe extern "C" fn wm8993_set_tdm_slot(dai: *mut snd_soc_dai, tx_mask: c_uint, rx_mask: c_uint, slots: c_int, slot_width: c_int) -> c_int {
    let component = (*dai).component;
    let wm8993 = snd_soc_component_get_drvdata(component) as *mut wm8993_priv;
    let mut aif1: c_uint = 0;
    let mut aif2: c_uint = 0;
    if slots == 0 {
        (*wm8993).tdm_slots = 0;
    } else {
        aif1 |= WM8993_AIFADC_TDM;
        aif2 |= WM8993_AIFDAC_TDM;
        if rx_mask == 3 { } else if rx_mask == 0xc { aif1 |= WM8993_AIFADC_TDM_CHAN; } else { return -EINVAL; }
        if tx_mask == 3 { } else if tx_mask == 0xc { aif2 |= WM8993_AIFDAC_TDM_CHAN; } else { return -EINVAL; }
    }
    (*wm8993).tdm_width = slot_width;
    (*wm8993).tdm_slots = slots / 2;
    snd_soc_component_update_bits(component, WM8993_AUDIO_INTERFACE_1, WM8993_AIFADC_TDM | WM8993_AIFADC_TDM_CHAN, aif1);
    snd_soc_component_update_bits(component, WM8993_AUDIO_INTERFACE_2, WM8993_AIFDAC_TDM | WM8993_AIFDAC_TDM_CHAN, aif2);
    0
}

unsafe extern "C" fn wm8993_irq(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let wm8993 = data as *mut wm8993_priv;
    let mut val: c_int = 0;
    let mut mask: c_int = 0;
    let mut ret = regmap_read((*wm8993).regmap, WM8993_GPIO_CTRL_1, &mut val);
    if ret != 0 {
        dev_err((*wm8993).dev, b"Failed to read interrupt status: %d\n\0".as_ptr() as *const c_char, ret);
        return IRQ_NONE;
    }
    ret = regmap_read((*wm8993).regmap, WM8993_GPIOCTRL_2, &mut mask);
    if ret != 0 {
        dev_err((*wm8993).dev, b"Failed to read interrupt mask: %d\n\0".as_ptr() as *const c_char, ret);
        return IRQ_NONE;
    }
    val &= !(mask | WM8993_IRQ);
    if val == 0 { return IRQ_NONE; }
    if (val & WM8993_TEMPOK_EINT) != 0 {
        dev_crit((*wm8993).dev, b"Thermal warning\n\0".as_ptr() as *const c_char);
    }
    if (val & WM8993_FLL_LOCK_EINT) != 0 {
        dev_dbg((*wm8993).dev, b"FLL locked\n\0".as_ptr() as *const c_char);
        complete(&mut (*wm8993).fll_lock);
    }
    ret = regmap_write((*wm8993).regmap, WM8993_GPIO_CTRL_1, val as c_uint);
    if ret != 0 {
        dev_err((*wm8993).dev, b"Failed to ack interrupt: %d\n\0".as_ptr() as *const c_char, ret);
    }
    IRQ_HANDLED
}

static wm8993_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_sysclk: Some(wm8993_set_sysclk),
    set_fmt: Some(wm8993_set_dai_fmt),
    hw_params: Some(wm8993_hw_params),
    mute_stream: Some(wm8993_mute),
    set_pll: Some(wm8993_set_fll),
    set_tdm_slot: Some(wm8993_set_tdm_slot),
    no_capture_mute: 1,
};

unsafe fn WM8993_RATES() -> c_uint { SNDRV_PCM_RATE_8000_48000 }
unsafe fn WM8993_FORMATS() -> c_uint { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE }

static mut wm8993_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"wm8993-hifi\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 1, channels_max: 2, rates: 0, formats: 0, sig_bits: 24,
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 1, channels_max: 2, rates: 0, formats: 0, sig_bits: 24,
    },
    ops: &wm8993_ops,
    symmetric_rate: 1,
};

unsafe extern "C" fn wm8993_probe(component: *mut snd_soc_component) -> c_int {
    let wm8993 = snd_soc_component_get_drvdata(component) as *mut wm8993_priv;
    let dapm = snd_soc_component_to_dapm(component);
    (*wm8993).hubs_data.hp_startup_mode = 1;
    (*wm8993).hubs_data.dcs_codes_l = -2;
    (*wm8993).hubs_data.dcs_codes_r = -2;
    (*wm8993).hubs_data.series_startup = 1;
    snd_soc_component_update_bits(component, WM8993_RIGHT_DAC_DIGITAL_VOLUME, WM8993_DAC_VU, WM8993_DAC_VU);
    snd_soc_component_update_bits(component, WM8993_RIGHT_ADC_DIGITAL_VOLUME, WM8993_ADC_VU, WM8993_ADC_VU);
    snd_soc_component_update_bits(component, WM8993_ANALOGUE_HP_0, WM8993_HPOUT1_AUTO_PU, 0);
    snd_soc_component_update_bits(component, WM8993_CLOCKING_4, WM8993_SR_MODE, 0);
    wm_hubs_handle_analogue_pdata(component, (*wm8993).pdata.lineout1_diff, (*wm8993).pdata.lineout2_diff,
        (*wm8993).pdata.lineout1fb, (*wm8993).pdata.lineout2fb, (*wm8993).pdata.jd_scthr,
        (*wm8993).pdata.jd_thr, (*wm8993).pdata.micbias1_delay, (*wm8993).pdata.micbias2_delay,
        (*wm8993).pdata.micbias1_lvl, (*wm8993).pdata.micbias2_lvl);
    snd_soc_add_component_controls(component, wm8993_snd_controls.as_ptr(), wm8993_snd_controls.len() as c_uint);
    if (*wm8993).pdata.num_retune_configs != 0 {
        dev_dbg((*component).dev, b"Using ReTune Mobile\n\0".as_ptr() as *const c_char);
    } else {
        dev_dbg((*component).dev, b"No ReTune Mobile, using normal EQ\n\0".as_ptr() as *const c_char);
        snd_soc_add_component_controls(component, wm8993_eq_controls.as_ptr(), wm8993_eq_controls.len() as c_uint);
    }
    snd_soc_dapm_new_controls(dapm, wm8993_dapm_widgets.as_ptr(), wm8993_dapm_widgets.len() as c_uint);
    wm_hubs_add_analogue_controls(component);
    snd_soc_dapm_add_routes(dapm, routes.as_ptr(), routes.len() as c_uint);
    wm_hubs_add_analogue_routes(component, (*wm8993).pdata.lineout1_diff, (*wm8993).pdata.lineout2_diff);
    if (*wm8993).pdata.lineout1_diff && (*wm8993).pdata.lineout2_diff {
        snd_soc_dapm_set_idle_bias(dapm, false);
    }
    0
}

/* CONFIG_PM conditional in C: suspend/resume are present only when CONFIG_PM is enabled, otherwise NULL. */
unsafe extern "C" fn wm8993_suspend(component: *mut snd_soc_component) -> c_int {
    let wm8993 = snd_soc_component_get_drvdata(component) as *mut wm8993_priv;
    let dapm = snd_soc_component_to_dapm(component);
    let fll_fout = (*wm8993).fll_fout;
    let fll_fref = (*wm8993).fll_fref;
    let ret = _wm8993_set_fll(component, 0, 0, 0, 0);
    if ret != 0 {
        dev_err((*component).dev, b"Failed to stop FLL\n\0".as_ptr() as *const c_char);
        return ret;
    }
    (*wm8993).fll_fout = fll_fout;
    (*wm8993).fll_fref = fll_fref;
    snd_soc_dapm_force_bias_level(dapm, snd_soc_bias_level::SND_SOC_BIAS_OFF);
    0
}

unsafe extern "C" fn wm8993_resume(component: *mut snd_soc_component) -> c_int {
    let wm8993 = snd_soc_component_get_drvdata(component) as *mut wm8993_priv;
    let dapm = snd_soc_component_to_dapm(component);
    snd_soc_dapm_force_bias_level(dapm, snd_soc_bias_level::SND_SOC_BIAS_STANDBY);
    if (*wm8993).fll_fout != 0 {
        let fll_fout = (*wm8993).fll_fout;
        let fll_fref = (*wm8993).fll_fref;
        (*wm8993).fll_fref = 0;
        (*wm8993).fll_fout = 0;
        let ret = _wm8993_set_fll(component, 0, (*wm8993).fll_src, fll_fref, fll_fout);
        if ret != 0 {
            dev_err((*component).dev, b"Failed to restart FLL\n\0".as_ptr() as *const c_char);
        }
    }
    0
}

static wm8993_regmap_patch: [reg_sequence; 3] = [
    reg_sequence { reg: 0x44, def: 3 },
    reg_sequence { reg: 0x56, def: 3 },
    reg_sequence { reg: 0x44, def: 0 },
];

static wm8993_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 16,
    max_register: unsafe { WM8993_MAX_REGISTER },
    volatile_reg: Some(wm8993_volatile),
    readable_reg: Some(wm8993_readable),
    cache_type: unsafe { REGCACHE_MAPLE },
    reg_defaults: wm8993_reg_defaults.as_ptr(),
    num_reg_defaults: wm8993_reg_defaults.len() as c_uint,
};

static soc_component_dev_wm8993: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm8993_probe),
    suspend: Some(wm8993_suspend),
    resume: Some(wm8993_resume),
    set_bias_level: Some(wm8993_set_bias_level),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn wm8993_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let wm8993 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<wm8993_priv>(), GFP_KERNEL) as *mut wm8993_priv;
    if wm8993.is_null() { return -ENOMEM; }
    (*wm8993).dev = &mut (*i2c).dev;
    init_completion(&mut (*wm8993).fll_lock);
    (*wm8993).regmap = devm_regmap_init_i2c(i2c, &wm8993_regmap);
    if ((*wm8993).regmap as isize) < 0 {
        let ret = (*wm8993).regmap as isize as c_int;
        dev_err(&mut (*i2c).dev, b"Failed to allocate regmap: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    i2c_set_clientdata(i2c, wm8993 as *mut c_void);
    let mut i: usize = 0;
    while i < (*wm8993).supplies.len() {
        (*wm8993).supplies[i].supply = wm8993_supply_names[i];
        i += 1;
    }
    let mut ret = devm_regulator_bulk_get(&mut (*i2c).dev, (*wm8993).supplies.len() as c_int, (*wm8993).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(&mut (*i2c).dev, b"Failed to request supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret = regulator_bulk_enable((*wm8993).supplies.len() as c_uint, (*wm8993).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(&mut (*i2c).dev, b"Failed to enable supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    let mut reg: c_int = 0;
    ret = regmap_read((*wm8993).regmap, WM8993_SOFTWARE_RESET, &mut reg);
    if ret != 0 {
        dev_err(&mut (*i2c).dev, b"Failed to read chip ID: %d\n\0".as_ptr() as *const c_char, ret);
        regulator_bulk_disable((*wm8993).supplies.len() as c_uint, (*wm8993).supplies.as_mut_ptr());
        return ret;
    }
    if reg != 0x8993 {
        dev_err(&mut (*i2c).dev, b"Invalid ID register value %x\n\0".as_ptr() as *const c_char, reg);
        regulator_bulk_disable((*wm8993).supplies.len() as c_uint, (*wm8993).supplies.as_mut_ptr());
        return -EINVAL;
    }
    ret = regmap_write((*wm8993).regmap, WM8993_SOFTWARE_RESET, 0xffff);
    if ret != 0 {
        regulator_bulk_disable((*wm8993).supplies.len() as c_uint, (*wm8993).supplies.as_mut_ptr());
        return ret;
    }
    ret = regmap_register_patch((*wm8993).regmap, wm8993_regmap_patch.as_ptr(), wm8993_regmap_patch.len() as c_int);
    if ret != 0 {
        dev_warn((*wm8993).dev, b"Failed to apply regmap patch: %d\n\0".as_ptr() as *const c_char, ret);
    }
    if (*i2c).irq != 0 {
        ret = regmap_update_bits((*wm8993).regmap, WM8993_GPIO1, WM8993_GPIO1_PD | WM8993_GPIO1_SEL_MASK, 7);
        if ret != 0 {
            regulator_bulk_disable((*wm8993).supplies.len() as c_uint, (*wm8993).supplies.as_mut_ptr());
            return ret;
        }
        ret = request_threaded_irq((*i2c).irq, core::ptr::null(), wm8993_irq, IRQF_TRIGGER_HIGH | IRQF_ONESHOT, b"wm8993\0".as_ptr() as *const c_char, wm8993 as *mut c_void);
        if ret != 0 {
            regulator_bulk_disable((*wm8993).supplies.len() as c_uint, (*wm8993).supplies.as_mut_ptr());
            return ret;
        }
    }
    regulator_bulk_disable((*wm8993).supplies.len() as c_uint, (*wm8993).supplies.as_mut_ptr());
    regcache_cache_only((*wm8993).regmap, true);
    wm8993_dai.playback.rates = WM8993_RATES();
    wm8993_dai.playback.formats = WM8993_FORMATS();
    wm8993_dai.capture.rates = WM8993_RATES();
    wm8993_dai.capture.formats = WM8993_FORMATS();
    ret = devm_snd_soc_register_component(&mut (*i2c).dev, &soc_component_dev_wm8993, &mut wm8993_dai, 1);
    if ret != 0 {
        dev_err(&mut (*i2c).dev, b"Failed to register CODEC: %d\n\0".as_ptr() as *const c_char, ret);
        if (*i2c).irq != 0 { free_irq((*i2c).irq, wm8993 as *mut c_void); }
        regulator_bulk_disable((*wm8993).supplies.len() as c_uint, (*wm8993).supplies.as_mut_ptr());
        return ret;
    }
    0
}

unsafe extern "C" fn wm8993_i2c_remove(i2c: *mut i2c_client) {
    let wm8993 = i2c_get_clientdata(i2c) as *mut wm8993_priv;
    if (*i2c).irq != 0 {
        free_irq((*i2c).irq, wm8993 as *mut c_void);
    }
    regulator_bulk_disable((*wm8993).supplies.len() as c_uint, (*wm8993).supplies.as_mut_ptr());
}

static wm8993_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: b"wm8993\0".as_ptr() as *const c_char },
    i2c_device_id { name: core::ptr::null() },
];

static mut wm8993_i2c_driver: i2c_driver = i2c_driver {
    driver: i2c_driver_inner { name: b"wm8993\0".as_ptr() as *const c_char },
    probe: Some(wm8993_i2c_probe),
    remove: Some(wm8993_i2c_remove),
    id_table: wm8993_i2c_id.as_ptr(),
};

/* MODULE_DEVICE_TABLE(i2c, wm8993_i2c_id);
 * module_i2c_driver(wm8993_i2c_driver);
 * MODULE_DESCRIPTION("ASoC WM8993 driver");
 * MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>");
 * MODULE_LICENSE("GPL");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
