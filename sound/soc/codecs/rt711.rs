// SPDX-License-Identifier: GPL-2.0
//
// rt711.c -- rt711 ALSA SoC audio driver
//
// Copyright(c) 2019 Realtek Semiconductor Corp.
//
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

// Translated from Linux kernel includes:
// linux/cleanup.h, linux/module.h, linux/moduleparam.h, linux/kernel.h,
// linux/init.h, linux/delay.h, linux/pm_runtime.h, linux/pm.h,
// linux/soundwire/sdw.h, linux/regmap.h, linux/slab.h,
// sound/core.h, sound/pcm.h, sound/pcm_params.h, sound/sdw.h, sound/soc.h,
// sound/soc-dapm.h, sound/initval.h, sound/tlv.h, sound/hda_verbs.h,
// sound/jack.h, and "rt711.h".
// External types, constants, and helper macros are expected from those
// dependencies in the final repository integration.

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    pub parent: *mut device,
    _private: [u8; 0],
}
#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
}
#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct delayed_work {
    pub work: work_struct,
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_card {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
}
#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: usize,
}
#[repr(C)]
pub struct snd_ctl_elem_id {
    pub name: [c_char; 44],
}
#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}
#[repr(C)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}
#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: core::mem::ManuallyDrop<snd_ctl_elem_value_integer>,
    pub enumerated: core::mem::ManuallyDrop<snd_ctl_elem_value_enumerated>,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub id: snd_ctl_elem_id,
    pub value: snd_ctl_elem_value_value,
}
#[repr(C)]
pub struct soc_mixer_control {
    pub reg: c_uint,
    pub rreg: c_uint,
    pub shift: c_uint,
    pub max: c_uint,
    pub invert: c_uint,
}
#[repr(C)]
pub struct soc_enum {
    pub items: c_uint,
    pub shift_l: c_uint,
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}
#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: usize,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: usize,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: usize,
    pub set_jack: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_jack, *mut c_void) -> c_int>,
    pub endianness: c_uint,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
    pub name: *const c_char,
    pub id: c_int,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sdw_stream_config {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sdw_port_config {
    pub num: c_uint,
}
#[repr(C)]
pub struct sdw_stream_runtime {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub set_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, *mut c_void, c_int) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_ulong,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}
#[repr(C)]
pub struct rt711_params {
    pub curr_dr_freq: c_uint,
}
#[repr(C)]
pub struct rt711_priv {
    pub regmap: *mut regmap,
    pub sdw_regmap: *mut regmap,
    pub slave: *mut sdw_slave,
    pub component: *mut snd_soc_component,
    pub hs_jack: *mut snd_soc_jack,
    pub jack_detect_work: delayed_work,
    pub jack_btn_check_work: delayed_work,
    pub calibration_work: work_struct,
    pub calibrate_mutex: mutex,
    pub disable_irq_lock: mutex,
    pub disable_irq: bool,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub jack_type: c_uint,
    pub jd_src: c_uint,
    pub params: rt711_params,
}

pub type c_long = i64;
pub type c_ulong = u64;
pub type snd_soc_bias_level = c_uint;

extern "C" {
    static mut system_power_efficient_wq: *mut c_void;

    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_get_device(map: *mut regmap) -> *mut device;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_cache_bypass(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);

    fn pr_err(fmt: *const c_char, ...);
    fn pr_err_ratelimited(fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn usleep_range(min: c_uint, max: c_uint);

    fn snd_soc_card_is_instantiated(card: *mut snd_soc_card) -> bool;
    fn pm_runtime_status_suspended(dev: *mut device) -> bool;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn mod_delayed_work(wq: *mut c_void, dwork: *mut delayed_work, delay: c_ulong) -> bool;
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut rt711_priv;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dapm_kcontrol_to_component(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dapm_kcontrol_to_dapm(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dapm_context;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn snd_soc_enum_item_to_val(e: *mut soc_enum, item: c_uint) -> c_uint;
    fn snd_soc_dapm_mux_update_power(
        dapm: *mut snd_soc_dapm_context,
        kcontrol: *mut snd_kcontrol,
        item: c_uint,
        e: *mut soc_enum,
        update: *mut c_void,
    ) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn device_property_read_u32(dev: *mut device, name: *const c_char, val: *mut c_uint) -> c_int;
    fn pm_runtime_resume(dev: *mut device) -> c_int;
    fn snd_soc_dai_dma_data_set(dai: *mut snd_soc_dai, direction: c_int, data: *mut c_void);
    fn snd_soc_dai_set_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream, data: *mut c_void);
    fn snd_soc_dai_get_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream) -> *mut sdw_stream_runtime;
    fn snd_sdw_params_to_config(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        stream_config: *mut sdw_stream_config,
        port_config: *mut sdw_port_config,
    );
    fn sdw_stream_add_slave(
        slave: *mut sdw_slave,
        stream_config: *mut sdw_stream_config,
        port_config: *mut sdw_port_config,
        num_ports: c_uint,
        stream: *mut sdw_stream_runtime,
    ) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn sdw_stream_remove_slave(slave: *mut sdw_slave, stream: *mut sdw_stream_runtime);
    fn dev_get_drvdata(dev: *mut device) -> *mut rt711_priv;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn mutex_init(lock: *mut mutex);
    fn INIT_DELAYED_WORK(work: *mut delayed_work, func: unsafe extern "C" fn(*mut work_struct));
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn devm_snd_soc_register_component(
        dev: *mut device,
        driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn schedule_work(work: *mut work_struct) -> bool;
}

extern "C" {
    static out_vol_tlv: [c_uint; 0];
    static in_vol_tlv: [c_uint; 0];
    static mic_vol_tlv: [c_uint; 0];
    static rt711_snd_controls: [snd_kcontrol_new; 8];
    static rt711_adc22_mux: snd_kcontrol_new;
    static rt711_adc23_mux: snd_kcontrol_new;
    static rt711_dapm_widgets: [snd_soc_dapm_widget; 14];
}

const ETIMEDOUT: c_int = 110;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const EACCES: c_int = 13;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

extern "C" {
    static RT711_PRIV_INDEX_W_H: c_uint;
    static RT711_FUNC_RESET: c_uint;
    static RT711_VENDOR_REG: c_uint;
    static RT711_PARA_VERB_CTL: c_uint;
    static RT711_HIDDEN_REG_SW_RESET: c_uint;
    static RT711_SET_AUDIO_POWER_STATE: c_uint;
    static AC_PWRST_D0: c_uint;
    static AC_PWRST_D3: c_uint;
    static RT711_FSM_CTL: c_uint;
    static RT711_VENDOR_CALI: c_uint;
    static RT711_DAC_DC_CALI_CTL1: c_uint;
    static RT711_DAC_DC_CALI_TRIGGER: c_uint;
    static RT711_DEPOP_CTL: c_uint;
    static RT711_IRQ_FLAG_TABLE1: c_uint;
    static RT711_IRQ_FLAG_TABLE2: c_uint;
    static RT711_COMBO_JACK_AUTO_CTL2: c_uint;
    static RT711_COMBOJACK_AUTO_DET_STATUS: c_uint;
    static RT711_VERB_GET_PIN_SENSE: c_uint;
    static RT711_HP_OUT: c_uint;
    static RT711_COMBOJACK_AUTO_DET_TRS: c_uint;
    static RT711_COMBOJACK_AUTO_DET_CTIA: c_uint;
    static RT711_COMBOJACK_AUTO_DET_OMTP: c_uint;
    static SND_JACK_BTN_0: c_int;
    static SND_JACK_BTN_1: c_int;
    static SND_JACK_BTN_2: c_int;
    static SND_JACK_BTN_3: c_int;
    static SND_JACK_HEADPHONE: c_uint;
    static SND_JACK_HEADSET: c_uint;
    static RT711_SET_MIC2_UNSOLICITED_ENABLE: c_uint;
    static RT711_SET_HP_UNSOLICITED_ENABLE: c_uint;
    static RT711_SET_INLINE_UNSOLICITED_ENABLE: c_uint;
    static RT711_JD1: c_uint;
    static RT711_JD2: c_uint;
    static RT711_JD2_100K: c_uint;
    static RT711_JD2_1P8V_1PORT: c_uint;
    static RT711_JD_CTL1: c_uint;
    static RT711_JD_CTL2: c_uint;
    static RT711_JD_CTL4: c_uint;
    static RT711_CC_DET1: c_uint;
    static RT711_JD2_2PORT_200K_DECODE_HP: c_uint;
    static RT711_HP_JD_SEL_JD2: c_uint;
    static RT711_HP_JD_FINAL_RESULT_CTL_JD12: c_uint;
    static RT711_JD2_2PORT_100K_DECODE: c_uint;
    static RT711_JD2_1PORT_TYPE_DECODE: c_uint;
    static RT711_JD1_2PORT_TYPE_100K_DECODE: c_uint;
    static RT711_JD2_2PORT_100K_DECODE_HP: c_uint;
    static RT711_JD2_1PORT_JD_HP: c_uint;
    static RT711_JD1_2PORT_JD_RESERVED: c_uint;
    static RT711_JD2_DIGITAL_JD_MODE_SEL: c_uint;
    static RT711_JD2_1_JD_MODE: c_uint;
    static RT711_JD2_PAD_PULL_UP_MASK: c_uint;
    static RT711_JD2_MODE_SEL_MASK: c_uint;
    static RT711_JD2_PAD_PULL_UP: c_uint;
    static RT711_JD2_MODE2_1P8V_1PORT: c_uint;
    static RT711_DIR_OUT_SFT: c_uint;
    static RT711_DIR_IN_SFT: c_uint;
    static RT711_MUTE_SFT: c_uint;
    static RT711_SET_GAIN_DAC2_H: c_uint;
    static RT711_SET_GAIN_DAC2_L: c_uint;
    static RT711_SET_GAIN_ADC2_H: c_uint;
    static RT711_SET_GAIN_ADC2_L: c_uint;
    static RT711_SET_GAIN_ADC1_H: c_uint;
    static RT711_SET_GAIN_ADC1_L: c_uint;
    static RT711_SET_GAIN_AMIC_H: c_uint;
    static RT711_SET_GAIN_AMIC_L: c_uint;
    static RT711_SET_GAIN_DMIC1_H: c_uint;
    static RT711_SET_GAIN_DMIC1_L: c_uint;
    static RT711_SET_GAIN_DMIC2_H: c_uint;
    static RT711_SET_GAIN_DMIC2_L: c_uint;
    static RT711_MIXER_IN1: c_uint;
    static RT711_MIXER_IN2: c_uint;
    static RT711_VERB_SET_CONNECT_SEL: c_uint;
    static SND_SOC_NOPM: c_int;
    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static RT711_SET_STREAMID_DAC2: c_uint;
    static RT711_SET_GAIN_HP_H: c_uint;
    static RT711_SET_STREAMID_ADC1: c_uint;
    static RT711_SET_STREAMID_ADC2: c_uint;
    static SND_SOC_BIAS_PREPARE: snd_soc_bias_level;
    static SND_SOC_BIAS_STANDBY: snd_soc_bias_level;
    static RT711_AIF1: c_int;
    static RT711_AIF2: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static RT711_DAC_FORMAT_H: c_uint;
    static RT711_ADC1_FORMAT_H: c_uint;
    static RT711_ADC2_FORMAT_H: c_uint;
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_ulong;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_ulong;
    static SNDRV_PCM_FMTBIT_S24_LE: c_ulong;
    static SNDRV_PCM_FMTBIT_S8: c_ulong;
    static RT711_SET_PIN_MIC2: c_uint;
    static RT711_SET_PIN_HP: c_uint;
    static RT711_SET_PIN_DMIC1: c_uint;
    static RT711_SET_PIN_DMIC2: c_uint;
    static RT711_SET_PIN_LINE1: c_uint;
    static RT711_SET_PIN_LINE2: c_uint;
    static RT711_TX_RX_MUX_CTL: c_uint;
    static RT711_DIGITAL_MISC_CTRL4: c_uint;
    static RT711_COMBO_JACK_AUTO_CTL1: c_uint;
    static RT711_VREFOUT_CTL: c_uint;
    static RT711_INLINE_CMD_CTL: c_uint;
}

macro_rules! cstr {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn rt711_index_write(
    regmap: *mut regmap,
    nid: c_uint,
    reg: c_uint,
    value: c_uint,
) -> c_int {
    let mut ret: c_int;
    let addr: c_uint = (((RT711_PRIV_INDEX_W_H | nid) << 8) | reg) as c_uint;

    ret = regmap_write(regmap, addr, value);
    if ret < 0 {
        pr_err(
            cstr!("%s: Failed to set private value: %06x <= %04x ret=%d\n"),
            cstr!("rt711_index_write"),
            addr,
            value,
            ret,
        );
    }

    ret
}

unsafe fn rt711_index_read(
    regmap: *mut regmap,
    nid: c_uint,
    reg: c_uint,
    value: *mut c_uint,
) -> c_int {
    let ret: c_int;
    let addr: c_uint = (((RT711_PRIV_INDEX_W_H | nid) << 8) | reg) as c_uint;

    *value = 0;
    ret = regmap_read(regmap, addr, value);
    if ret < 0 {
        pr_err(
            cstr!("%s: Failed to get private value: %06x => %04x ret=%d\n"),
            cstr!("rt711_index_read"),
            addr,
            *value,
            ret,
        );
    }

    ret
}

unsafe fn rt711_index_update_bits(
    regmap: *mut regmap,
    nid: c_uint,
    reg: c_uint,
    mask: c_uint,
    val: c_uint,
) -> c_int {
    let mut tmp: c_uint;
    let mut orig: c_uint = 0;
    let ret: c_int;

    ret = rt711_index_read(regmap, nid, reg, &mut orig);
    if ret < 0 {
        return ret;
    }

    tmp = orig & !mask;
    tmp |= val & mask;

    rt711_index_write(regmap, nid, reg, tmp)
}

unsafe fn rt711_reset(regmap: *mut regmap) {
    regmap_write(regmap, RT711_FUNC_RESET, 0);
    rt711_index_update_bits(
        regmap,
        RT711_VENDOR_REG,
        RT711_PARA_VERB_CTL,
        RT711_HIDDEN_REG_SW_RESET,
        RT711_HIDDEN_REG_SW_RESET,
    );
}

unsafe fn rt711_calibration(rt711: *mut rt711_priv) -> c_int {
    let mut val: c_uint = 0;
    let mut loop_: c_uint = 0;
    let dev: *mut device;
    let regmap: *mut regmap = (*rt711).regmap;
    let mut ret: c_int = 0;

    // C used guard(mutex)(&rt711->calibrate_mutex).
    regmap_write((*rt711).regmap, RT711_SET_AUDIO_POWER_STATE, AC_PWRST_D0);

    dev = regmap_get_device(regmap);

    /* Calibration manual mode */
    rt711_index_update_bits(regmap, RT711_VENDOR_REG, RT711_FSM_CTL, 0xf, 0x0);

    /* trigger */
    rt711_index_update_bits(
        regmap,
        RT711_VENDOR_CALI,
        RT711_DAC_DC_CALI_CTL1,
        RT711_DAC_DC_CALI_TRIGGER,
        RT711_DAC_DC_CALI_TRIGGER,
    );

    /* wait for calibration process */
    rt711_index_read(regmap, RT711_VENDOR_CALI, RT711_DAC_DC_CALI_CTL1, &mut val);

    while (val & RT711_DAC_DC_CALI_TRIGGER) != 0 {
        if loop_ >= 500 {
            pr_err(cstr!("%s, calibration time-out!\n"), cstr!("rt711_calibration"));
            ret = -ETIMEDOUT;
            break;
        }
        loop_ += 1;

        usleep_range(10000, 11000);
        rt711_index_read(regmap, RT711_VENDOR_CALI, RT711_DAC_DC_CALI_CTL1, &mut val);
    }

    /* depop mode */
    rt711_index_update_bits(regmap, RT711_VENDOR_REG, RT711_FSM_CTL, 0xf, RT711_DEPOP_CTL);

    regmap_write((*rt711).regmap, RT711_SET_AUDIO_POWER_STATE, AC_PWRST_D3);

    dev_dbg(dev, cstr!("%s calibration complete, ret=%d\n"), cstr!("rt711_calibration"), ret);
    ret
}

unsafe fn rt711_button_detect(rt711: *mut rt711_priv) -> c_uint {
    let mut btn_type: c_uint = 0;
    let mut val80: c_uint = 0;
    let mut val81: c_uint = 0;
    let mut ret: c_int;

    ret = rt711_index_read((*rt711).regmap, RT711_VENDOR_REG, RT711_IRQ_FLAG_TABLE1, &mut val80);
    if ret < 0 {
        return btn_type;
    }
    ret = rt711_index_read((*rt711).regmap, RT711_VENDOR_REG, RT711_IRQ_FLAG_TABLE2, &mut val81);
    if ret < 0 {
        return btn_type;
    }

    val80 &= 0x0381;
    val81 &= 0xff00;

    match val80 {
        0x0200 | 0x0100 | 0x0080 => btn_type |= SND_JACK_BTN_0 as c_uint,
        0x0001 => btn_type |= SND_JACK_BTN_3 as c_uint,
        _ => {}
    }
    match val81 {
        0x8000 | 0x4000 | 0x2000 => btn_type |= SND_JACK_BTN_1 as c_uint,
        0x1000 | 0x0800 | 0x0400 => btn_type |= SND_JACK_BTN_2 as c_uint,
        0x0200 | 0x0100 => btn_type |= SND_JACK_BTN_3 as c_uint,
        _ => {}
    }
    btn_type
}

unsafe fn rt711_headset_detect(rt711: *mut rt711_priv) -> c_int {
    let mut buf: c_uint = 0;
    let mut loop_: c_uint = 0;
    let mut ret: c_int;
    let mut jack_status: c_uint = 0;
    let mut reg: c_uint;

    ret = rt711_index_read((*rt711).regmap, RT711_VENDOR_REG, RT711_COMBO_JACK_AUTO_CTL2, &mut buf);
    if ret < 0 {
        pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("rt711_headset_detect"), ret);
        return ret;
    }

    while loop_ < 500 && (buf & RT711_COMBOJACK_AUTO_DET_STATUS) == 0 {
        loop_ += 1;

        usleep_range(9000, 10000);
        ret = rt711_index_read((*rt711).regmap, RT711_VENDOR_REG, RT711_COMBO_JACK_AUTO_CTL2, &mut buf);
        if ret < 0 {
            pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("rt711_headset_detect"), ret);
            return ret;
        }

        reg = RT711_VERB_GET_PIN_SENSE | RT711_HP_OUT;
        ret = regmap_read((*rt711).regmap, reg, &mut jack_status);
        if ret < 0 {
            pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("rt711_headset_detect"), ret);
            return ret;
        }
        if (jack_status & (1u32 << 31)) == 0 {
            pr_err_ratelimited(cstr!("Jack removal in %s\n"), cstr!("rt711_headset_detect"));
            return -ENODEV;
        }
    }

    if loop_ >= 500 {
        ret = -ETIMEDOUT;
        pr_err_ratelimited(cstr!("Time-out error in %s\n"), cstr!("rt711_headset_detect"));
        return ret;
    }

    if (buf & RT711_COMBOJACK_AUTO_DET_TRS) != 0 {
        (*rt711).jack_type = SND_JACK_HEADPHONE;
    } else if (buf & RT711_COMBOJACK_AUTO_DET_CTIA) != 0 || (buf & RT711_COMBOJACK_AUTO_DET_OMTP) != 0 {
        (*rt711).jack_type = SND_JACK_HEADSET;
    }

    0
}

unsafe extern "C" fn rt711_jack_detect_handler(work: *mut work_struct) {
    // container_of(work, struct rt711_priv, jack_detect_work.work)
    let rt711 = work as *mut rt711_priv;
    let mut btn_type: c_int = 0;
    let mut ret: c_int;
    let mut jack_status: c_uint = 0;
    let reg: c_uint;

    if (*rt711).hs_jack.is_null() {
        return;
    }

    if !snd_soc_card_is_instantiated((*(*rt711).component).card) {
        return;
    }

    if pm_runtime_status_suspended((*(*rt711).slave).dev.parent) {
        dev_dbg(
            &mut (*(*rt711).slave).dev,
            cstr!("%s: parent device is pm_runtime_status_suspended, skipping jack detection\n"),
            cstr!("rt711_jack_detect_handler"),
        );
        return;
    }

    reg = RT711_VERB_GET_PIN_SENSE | RT711_HP_OUT;
    ret = regmap_read((*rt711).regmap, reg, &mut jack_status);
    if ret < 0 {
        pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("rt711_jack_detect_handler"), ret);
        return;
    }

    /* pin attached */
    if (jack_status & (1u32 << 31)) != 0 {
        /* jack in */
        if (*rt711).jack_type == 0 {
            ret = rt711_headset_detect(rt711);
            if ret < 0 {
                return;
            }
            if (*rt711).jack_type == SND_JACK_HEADSET {
                btn_type = rt711_button_detect(rt711) as c_int;
            }
        } else if (*rt711).jack_type == SND_JACK_HEADSET {
            /* jack is already in, report button event */
            btn_type = rt711_button_detect(rt711) as c_int;
        }
    } else {
        /* jack out */
        (*rt711).jack_type = 0;
    }

    dev_dbg(&mut (*(*rt711).slave).dev, cstr!("in %s, jack_type=0x%x\n"), cstr!("rt711_jack_detect_handler"), (*rt711).jack_type);
    dev_dbg(&mut (*(*rt711).slave).dev, cstr!("in %s, btn_type=0x%x\n"), cstr!("rt711_jack_detect_handler"), btn_type);

    snd_soc_jack_report(
        (*rt711).hs_jack,
        ((*rt711).jack_type as c_int) | btn_type,
        SND_JACK_HEADSET as c_int | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3,
    );

    if btn_type != 0 {
        /* button released */
        snd_soc_jack_report(
            (*rt711).hs_jack,
            (*rt711).jack_type as c_int,
            SND_JACK_HEADSET as c_int | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3,
        );

        mod_delayed_work(
            system_power_efficient_wq,
            &mut (*rt711).jack_btn_check_work,
            msecs_to_jiffies(200),
        );
    }
}

unsafe extern "C" fn rt711_btn_check_handler(work: *mut work_struct) {
    // container_of(work, struct rt711_priv, jack_btn_check_work.work)
    let rt711 = work as *mut rt711_priv;
    let mut btn_type: c_int = 0;
    let mut ret: c_int;
    let mut jack_status: c_uint = 0;
    let mut reg: c_uint;

    reg = RT711_VERB_GET_PIN_SENSE | RT711_HP_OUT;
    ret = regmap_read((*rt711).regmap, reg, &mut jack_status);
    if ret < 0 {
        pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("rt711_btn_check_handler"), ret);
        return;
    }

    /* pin attached */
    if (jack_status & (1u32 << 31)) != 0 {
        if (*rt711).jack_type == SND_JACK_HEADSET {
            /* jack is already in, report button event */
            btn_type = rt711_button_detect(rt711) as c_int;
        }
    } else {
        (*rt711).jack_type = 0;
    }

    /* cbj comparator */
    ret = rt711_index_read((*rt711).regmap, RT711_VENDOR_REG, RT711_COMBO_JACK_AUTO_CTL2, &mut reg);
    if ret < 0 {
        pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("rt711_btn_check_handler"), ret);
        return;
    }

    if (reg & 0xf0) == 0xf0 {
        btn_type = 0;
    }

    dev_dbg(&mut (*(*rt711).slave).dev, cstr!("%s, btn_type=0x%x\n"), cstr!("rt711_btn_check_handler"), btn_type);
    snd_soc_jack_report(
        (*rt711).hs_jack,
        ((*rt711).jack_type as c_int) | btn_type,
        SND_JACK_HEADSET as c_int | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3,
    );

    if btn_type != 0 {
        /* button released */
        snd_soc_jack_report(
            (*rt711).hs_jack,
            (*rt711).jack_type as c_int,
            SND_JACK_HEADSET as c_int | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3,
        );

        mod_delayed_work(
            system_power_efficient_wq,
            &mut (*rt711).jack_btn_check_work,
            msecs_to_jiffies(200),
        );
    }
}

unsafe fn rt711_jack_init(rt711: *mut rt711_priv) {
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm((*rt711).component);

    // C used guard(mutex)(&rt711->calibrate_mutex).
    /* power on */
    if snd_soc_dapm_get_bias_level(dapm) <= SND_SOC_BIAS_STANDBY {
        regmap_write((*rt711).regmap, RT711_SET_AUDIO_POWER_STATE, AC_PWRST_D0);
    }

    if !(*rt711).hs_jack.is_null() {
        /* unsolicited response & IRQ control */
        regmap_write((*rt711).regmap, RT711_SET_MIC2_UNSOLICITED_ENABLE, 0x82);
        regmap_write((*rt711).regmap, RT711_SET_HP_UNSOLICITED_ENABLE, 0x81);
        regmap_write((*rt711).regmap, RT711_SET_INLINE_UNSOLICITED_ENABLE, 0x83);
        rt711_index_write((*rt711).regmap, RT711_VENDOR_REG, 0x10, 0x2420);
        rt711_index_write((*rt711).regmap, RT711_VENDOR_REG, 0x19, 0x2e11);

        match (*rt711).jd_src {
            x if x == RT711_JD1 => {
                /* default settings was already for JD1 */
            }
            x if x == RT711_JD2 => {
                rt711_index_update_bits(
                    (*rt711).regmap,
                    RT711_VENDOR_REG,
                    RT711_JD_CTL2,
                    RT711_JD2_2PORT_200K_DECODE_HP | RT711_HP_JD_SEL_JD2,
                    RT711_JD2_2PORT_200K_DECODE_HP | RT711_HP_JD_SEL_JD2,
                );
                rt711_index_update_bits(
                    (*rt711).regmap,
                    RT711_VENDOR_REG,
                    RT711_CC_DET1,
                    RT711_HP_JD_FINAL_RESULT_CTL_JD12,
                    RT711_HP_JD_FINAL_RESULT_CTL_JD12,
                );
            }
            x if x == RT711_JD2_100K => {
                rt711_index_update_bits(
                    (*rt711).regmap,
                    RT711_VENDOR_REG,
                    RT711_JD_CTL2,
                    RT711_JD2_2PORT_100K_DECODE
                        | RT711_JD2_1PORT_TYPE_DECODE
                        | RT711_HP_JD_SEL_JD2
                        | RT711_JD1_2PORT_TYPE_100K_DECODE,
                    RT711_JD2_2PORT_100K_DECODE_HP
                        | RT711_JD2_1PORT_JD_HP
                        | RT711_HP_JD_SEL_JD2
                        | RT711_JD1_2PORT_JD_RESERVED,
                );
                rt711_index_update_bits(
                    (*rt711).regmap,
                    RT711_VENDOR_REG,
                    RT711_CC_DET1,
                    RT711_HP_JD_FINAL_RESULT_CTL_JD12,
                    RT711_HP_JD_FINAL_RESULT_CTL_JD12,
                );
            }
            x if x == RT711_JD2_1P8V_1PORT => {
                rt711_index_update_bits(
                    (*rt711).regmap,
                    RT711_VENDOR_REG,
                    RT711_JD_CTL1,
                    RT711_JD2_DIGITAL_JD_MODE_SEL,
                    RT711_JD2_1_JD_MODE,
                );
                rt711_index_update_bits(
                    (*rt711).regmap,
                    RT711_VENDOR_REG,
                    RT711_JD_CTL2,
                    RT711_JD2_1PORT_TYPE_DECODE | RT711_HP_JD_SEL_JD2,
                    RT711_JD2_1PORT_JD_HP | RT711_HP_JD_SEL_JD2,
                );
                rt711_index_update_bits(
                    (*rt711).regmap,
                    RT711_VENDOR_REG,
                    RT711_JD_CTL4,
                    RT711_JD2_PAD_PULL_UP_MASK | RT711_JD2_MODE_SEL_MASK,
                    RT711_JD2_PAD_PULL_UP | RT711_JD2_MODE2_1P8V_1PORT,
                );
                rt711_index_update_bits(
                    (*rt711).regmap,
                    RT711_VENDOR_REG,
                    RT711_CC_DET1,
                    RT711_HP_JD_FINAL_RESULT_CTL_JD12,
                    RT711_HP_JD_FINAL_RESULT_CTL_JD12,
                );
            }
            _ => {
                dev_warn((*(*rt711).component).dev, cstr!("%s: Wrong JD source\n"), cstr!("rt711_jack_init"));
            }
        }

        dev_dbg(&mut (*(*rt711).slave).dev, cstr!("in %s enable\n"), cstr!("rt711_jack_init"));

        mod_delayed_work(
            system_power_efficient_wq,
            &mut (*rt711).jack_detect_work,
            msecs_to_jiffies(250),
        );
    } else {
        regmap_write((*rt711).regmap, RT711_SET_MIC2_UNSOLICITED_ENABLE, 0x00);
        regmap_write((*rt711).regmap, RT711_SET_HP_UNSOLICITED_ENABLE, 0x00);
        regmap_write((*rt711).regmap, RT711_SET_INLINE_UNSOLICITED_ENABLE, 0x00);

        dev_dbg(&mut (*(*rt711).slave).dev, cstr!("in %s disable\n"), cstr!("rt711_jack_init"));
    }

    /* power off */
    if snd_soc_dapm_get_bias_level(dapm) <= SND_SOC_BIAS_STANDBY {
        regmap_write((*rt711).regmap, RT711_SET_AUDIO_POWER_STATE, AC_PWRST_D3);
    }
}

unsafe extern "C" fn rt711_set_jack_detect(
    component: *mut snd_soc_component,
    hs_jack: *mut snd_soc_jack,
    _data: *mut c_void,
) -> c_int {
    let rt711: *mut rt711_priv = snd_soc_component_get_drvdata(component);
    let ret: c_int;

    (*rt711).hs_jack = hs_jack;

    /* we can only resume if the device was initialized at least once */
    if !(*rt711).first_hw_init {
        return 0;
    }

    ret = pm_runtime_resume_and_get((*component).dev);
    if ret < 0 {
        if ret != -EACCES {
            dev_err((*component).dev, cstr!("%s: failed to resume %d\n"), cstr!("rt711_set_jack_detect"), ret);
            return ret;
        }

        /* pm_runtime not enabled yet */
        dev_dbg((*component).dev, cstr!("%s: skipping jack init for now\n"), cstr!("rt711_set_jack_detect"));
        return 0;
    }

    rt711_jack_init(rt711);

    pm_runtime_put_autosuspend((*component).dev);

    0
}

unsafe fn rt711_get_gain(
    rt711: *mut rt711_priv,
    addr_h: c_uint,
    addr_l: c_uint,
    mut val_h: c_uint,
    r_val: *mut c_uint,
    l_val: *mut c_uint,
) {
    /* R Channel */
    *r_val = val_h << 8;
    regmap_read((*rt711).regmap, addr_l, r_val);

    /* L Channel */
    val_h |= 0x20;
    *l_val = val_h << 8;
    regmap_read((*rt711).regmap, addr_h, l_val);
}

/* For Verb-Set Amplifier Gain (Verb ID = 3h) */
unsafe extern "C" fn rt711_set_amp_gain_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);
    let mc: *mut soc_mixer_control = (*kcontrol).private_value as *mut soc_mixer_control;
    let rt711: *mut rt711_priv = snd_soc_component_get_drvdata(component);
    let addr_h: c_uint;
    let addr_l: c_uint;
    let mut val_h: c_uint;
    let mut val_ll: c_uint;
    let mut val_lr: c_uint;
    let mut read_ll: c_uint = 0;
    let mut read_rl: c_uint = 0;
    let mut i: c_int;

    // C used guard(mutex)(&rt711->calibrate_mutex).

    /* Can't use update bit function, so read the original value first */
    addr_h = (*mc).reg;
    addr_l = (*mc).rreg;
    if (*mc).shift == RT711_DIR_OUT_SFT {
        /* output */
        val_h = 0x80;
    } else {
        /* input */
        val_h = 0x0;
    }

    rt711_get_gain(rt711, addr_h, addr_l, val_h, &mut read_rl, &mut read_ll);

    /* L Channel */
    if (*mc).invert != 0 {
        /* for mute/unmute */
        val_ll = ((*mc).max - (*core::mem::ManuallyDrop::into_inner(ptr::read(&(*ucontrol).value.integer))).value[0] as c_uint) << RT711_MUTE_SFT;
        /* keep gain */
        read_ll = read_ll & 0x7f;
        val_ll |= read_ll;
    } else {
        /* for gain */
        val_ll = ((*core::mem::ManuallyDrop::into_inner(ptr::read(&(*ucontrol).value.integer))).value[0] as c_uint) & 0x7f;
        if val_ll > (*mc).max {
            val_ll = (*mc).max;
        }
        /* keep mute status */
        read_ll = read_ll & (1 << RT711_MUTE_SFT);
        val_ll |= read_ll;
    }

    if snd_soc_dapm_get_bias_level(dapm) <= SND_SOC_BIAS_STANDBY {
        regmap_write((*rt711).regmap, RT711_SET_AUDIO_POWER_STATE, AC_PWRST_D0);
    }

    /* R Channel */
    if (*mc).invert != 0 {
        /* for mute/unmute */
        val_lr = ((*mc).max - (*core::mem::ManuallyDrop::into_inner(ptr::read(&(*ucontrol).value.integer))).value[1] as c_uint) << RT711_MUTE_SFT;
        /* keep gain */
        read_rl = read_rl & 0x7f;
        val_lr |= read_rl;
    } else {
        /* for gain */
        val_lr = ((*core::mem::ManuallyDrop::into_inner(ptr::read(&(*ucontrol).value.integer))).value[1] as c_uint) & 0x7f;
        if val_lr > (*mc).max {
            val_lr = (*mc).max;
        }
        /* keep mute status */
        read_rl = read_rl & (1 << RT711_MUTE_SFT);
        val_lr |= read_rl;
    }

    i = 0;
    while i < 3 {
        /* retry 3 times at most */
        if val_ll == val_lr {
            /* Set both L/R channels at the same time */
            val_h = (1 << (*mc).shift) | (3 << 4);
            regmap_write((*rt711).regmap, addr_h, (val_h << 8) | val_ll);
            regmap_write((*rt711).regmap, addr_l, (val_h << 8) | val_ll);
        } else {
            /* Lch*/
            val_h = (1 << (*mc).shift) | (1 << 5);
            regmap_write((*rt711).regmap, addr_h, (val_h << 8) | val_ll);

            /* Rch */
            val_h = (1 << (*mc).shift) | (1 << 4);
            regmap_write((*rt711).regmap, addr_l, (val_h << 8) | val_lr);
        }
        /* check result */
        if (*mc).shift == RT711_DIR_OUT_SFT {
            /* output */
            val_h = 0x80;
        } else {
            /* input */
            val_h = 0x0;
        }

        rt711_get_gain(rt711, addr_h, addr_l, val_h, &mut read_rl, &mut read_ll);
        if read_rl == val_lr && read_ll == val_ll {
            break;
        }
        i += 1;
    }

    if snd_soc_dapm_get_bias_level(dapm) <= SND_SOC_BIAS_STANDBY {
        regmap_write((*rt711).regmap, RT711_SET_AUDIO_POWER_STATE, AC_PWRST_D3);
    }

    0
}

unsafe extern "C" fn rt711_set_amp_gain_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let rt711: *mut rt711_priv = snd_soc_component_get_drvdata(component);
    let mc: *mut soc_mixer_control = (*kcontrol).private_value as *mut soc_mixer_control;
    let addr_h: c_uint;
    let addr_l: c_uint;
    let mut val_h: c_uint;
    let mut read_ll: c_uint = 0;
    let mut read_rl: c_uint = 0;

    /* switch to get command */
    addr_h = (*mc).reg;
    addr_l = (*mc).rreg;
    if (*mc).shift == RT711_DIR_OUT_SFT {
        /* output */
        val_h = 0x80;
    } else {
        /* input */
        val_h = 0x0;
    }

    rt711_get_gain(rt711, addr_h, addr_l, val_h, &mut read_rl, &mut read_ll);

    if (*mc).invert != 0 {
        /* mute/unmute for switch controls */
        read_ll = (((read_ll & 0x80) >> RT711_MUTE_SFT) == 0) as c_uint;
        read_rl = (((read_rl & 0x80) >> RT711_MUTE_SFT) == 0) as c_uint;
    } else {
        /* for gain volume controls */
        read_ll = read_ll & 0x7f;
        read_rl = read_rl & 0x7f;
    }
    (*core::mem::ManuallyDrop::into_inner(ptr::read(&(*ucontrol).value.integer))).value[0] = read_ll as c_long;
    (*core::mem::ManuallyDrop::into_inner(ptr::read(&(*ucontrol).value.integer))).value[1] = read_rl as c_long;

    0
}

// static const DECLARE_TLV_DB_SCALE(out_vol_tlv, -6525, 75, 0);
// static const DECLARE_TLV_DB_SCALE(in_vol_tlv, -1725, 75, 0);
// static const DECLARE_TLV_DB_SCALE(mic_vol_tlv, 0, 1000, 0);
//
// static const struct snd_kcontrol_new rt711_snd_controls[] = {
//     SOC_DOUBLE_R_EXT_TLV("DAC Surr Playback Volume", ...),
//     SOC_DOUBLE_R_EXT("ADC 08 Capture Switch", ...),
//     SOC_DOUBLE_R_EXT("ADC 09 Capture Switch", ...),
//     SOC_DOUBLE_R_EXT_TLV("ADC 08 Capture Volume", ...),
//     SOC_DOUBLE_R_EXT_TLV("ADC 09 Capture Volume", ...),
//     SOC_DOUBLE_R_EXT_TLV("AMIC Volume", ...),
//     SOC_DOUBLE_R_EXT_TLV("DMIC1 Volume", ...),
//     SOC_DOUBLE_R_EXT_TLV("DMIC2 Volume", ...),
// };

unsafe extern "C" fn rt711_mux_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let rt711: *mut rt711_priv = snd_soc_component_get_drvdata(component);
    let mut reg: c_uint;
    let mut val: c_uint = 0;
    let nid: c_uint;
    let ret: c_int;

    if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 22 Mux")).is_null() {
        nid = RT711_MIXER_IN1;
    } else if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 23 Mux")).is_null() {
        nid = RT711_MIXER_IN2;
    } else {
        return -EINVAL;
    }

    /* vid = 0xf01 */
    reg = RT711_VERB_SET_CONNECT_SEL | nid;
    ret = regmap_read((*rt711).regmap, reg, &mut val);
    if ret < 0 {
        dev_err((*component).dev, cstr!("%s: sdw read failed: %d\n"), cstr!("rt711_mux_get"), ret);
        return ret;
    }

    (*core::mem::ManuallyDrop::into_inner(ptr::read(&(*ucontrol).value.enumerated))).item[0] = val;

    0
}

unsafe extern "C" fn rt711_mux_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let dapm: *mut snd_soc_dapm_context = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let rt711: *mut rt711_priv = snd_soc_component_get_drvdata(component);
    let e: *mut soc_enum = (*kcontrol).private_value as *mut soc_enum;
    let item: *mut c_uint = (*core::mem::ManuallyDrop::into_inner(ptr::read(&(*ucontrol).value.enumerated))).item.as_mut_ptr();
    let val: c_uint;
    let mut val2: c_uint = 0;
    let change: c_uint;
    let mut reg: c_uint;
    let nid: c_uint;
    let ret: c_int;

    if *item.add(0) >= (*e).items {
        return -EINVAL;
    }

    if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 22 Mux")).is_null() {
        nid = RT711_MIXER_IN1;
    } else if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 23 Mux")).is_null() {
        nid = RT711_MIXER_IN2;
    } else {
        return -EINVAL;
    }

    /* Verb ID = 0x701h */
    val = snd_soc_enum_item_to_val(e, *item.add(0)) << (*e).shift_l;

    reg = RT711_VERB_SET_CONNECT_SEL | nid;
    ret = regmap_read((*rt711).regmap, reg, &mut val2);
    if ret < 0 {
        dev_err((*component).dev, cstr!("%s: sdw read failed: %d\n"), cstr!("rt711_mux_put"), ret);
        return ret;
    }

    if val == val2 {
        change = 0;
    } else {
        change = 1;
    }

    if change != 0 {
        reg = RT711_VERB_SET_CONNECT_SEL | nid;
        regmap_write((*rt711).regmap, reg, val);
    }

    snd_soc_dapm_mux_update_power(dapm, kcontrol, *item.add(0), e, ptr::null_mut());

    change as c_int
}

static adc_mux_text: [*const c_char; 4] = [
    cstr!("MIC2"),
    cstr!("LINE1"),
    cstr!("LINE2"),
    cstr!("DMIC"),
];

// static SOC_ENUM_SINGLE_DECL(rt711_adc22_enum, SND_SOC_NOPM, 0, adc_mux_text);
// static SOC_ENUM_SINGLE_DECL(rt711_adc23_enum, SND_SOC_NOPM, 0, adc_mux_text);
// static const struct snd_kcontrol_new rt711_adc22_mux =
//     SOC_DAPM_ENUM_EXT("ADC 22 Mux", rt711_adc22_enum, rt711_mux_get, rt711_mux_put);
// static const struct snd_kcontrol_new rt711_adc23_mux =
//     SOC_DAPM_ENUM_EXT("ADC 23 Mux", rt711_adc23_enum, rt711_mux_get, rt711_mux_put);

unsafe extern "C" fn rt711_dac_surround_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let rt711: *mut rt711_priv = snd_soc_component_get_drvdata(component);
    let val_h: c_uint = (1 << RT711_DIR_OUT_SFT) | (0x3 << 4);
    let val_l: c_uint;

    if event == SND_SOC_DAPM_POST_PMU {
        regmap_write((*rt711).regmap, RT711_SET_STREAMID_DAC2, 0x10);

        val_l = 0x00;
        regmap_write((*rt711).regmap, RT711_SET_GAIN_HP_H, (val_h << 8) | val_l);
    } else if event == SND_SOC_DAPM_PRE_PMD {
        val_l = 1 << RT711_MUTE_SFT;
        regmap_write((*rt711).regmap, RT711_SET_GAIN_HP_H, (val_h << 8) | val_l);
        usleep_range(50000, 55000);

        regmap_write((*rt711).regmap, RT711_SET_STREAMID_DAC2, 0x00);
    }
    0
}

unsafe extern "C" fn rt711_adc_09_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let rt711: *mut rt711_priv = snd_soc_component_get_drvdata(component);

    if event == SND_SOC_DAPM_POST_PMU {
        regmap_write((*rt711).regmap, RT711_SET_STREAMID_ADC1, 0x10);
    } else if event == SND_SOC_DAPM_PRE_PMD {
        regmap_write((*rt711).regmap, RT711_SET_STREAMID_ADC1, 0x00);
    }
    0
}

unsafe extern "C" fn rt711_adc_08_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let rt711: *mut rt711_priv = snd_soc_component_get_drvdata(component);

    if event == SND_SOC_DAPM_POST_PMU {
        regmap_write((*rt711).regmap, RT711_SET_STREAMID_ADC2, 0x10);
    } else if event == SND_SOC_DAPM_PRE_PMD {
        regmap_write((*rt711).regmap, RT711_SET_STREAMID_ADC2, 0x00);
    }
    0
}

// static const struct snd_soc_dapm_widget rt711_dapm_widgets[] = {
//     SND_SOC_DAPM_OUTPUT("HP"),
//     SND_SOC_DAPM_INPUT("MIC2"),
//     SND_SOC_DAPM_INPUT("DMIC1"),
//     SND_SOC_DAPM_INPUT("DMIC2"),
//     SND_SOC_DAPM_INPUT("LINE1"),
//     SND_SOC_DAPM_INPUT("LINE2"),
//     SND_SOC_DAPM_DAC_E("DAC Surround", NULL, SND_SOC_NOPM, 0, 0,
//         rt711_dac_surround_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
//     SND_SOC_DAPM_ADC_E("ADC 09", NULL, SND_SOC_NOPM, 0, 0,
//         rt711_adc_09_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
//     SND_SOC_DAPM_ADC_E("ADC 08", NULL, SND_SOC_NOPM, 0, 0,
//         rt711_adc_08_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
//     SND_SOC_DAPM_MUX("ADC 22 Mux", SND_SOC_NOPM, 0, 0, &rt711_adc22_mux),
//     SND_SOC_DAPM_MUX("ADC 23 Mux", SND_SOC_NOPM, 0, 0, &rt711_adc23_mux),
//     SND_SOC_DAPM_AIF_IN("DP3RX", "DP3 Playback", 0, SND_SOC_NOPM, 0, 0),
//     SND_SOC_DAPM_AIF_OUT("DP2TX", "DP2 Capture", 0, SND_SOC_NOPM, 0, 0),
//     SND_SOC_DAPM_AIF_OUT("DP4TX", "DP4 Capture", 0, SND_SOC_NOPM, 0, 0),
// };

static rt711_audio_map: [snd_soc_dapm_route; 14] = [
    snd_soc_dapm_route { sink: cstr!("DAC Surround"), control: ptr::null(), source: cstr!("DP3RX") },
    snd_soc_dapm_route { sink: cstr!("DP2TX"), control: ptr::null(), source: cstr!("ADC 09") },
    snd_soc_dapm_route { sink: cstr!("DP4TX"), control: ptr::null(), source: cstr!("ADC 08") },
    snd_soc_dapm_route { sink: cstr!("ADC 09"), control: ptr::null(), source: cstr!("ADC 22 Mux") },
    snd_soc_dapm_route { sink: cstr!("ADC 08"), control: ptr::null(), source: cstr!("ADC 23 Mux") },
    snd_soc_dapm_route { sink: cstr!("ADC 22 Mux"), control: cstr!("DMIC"), source: cstr!("DMIC1") },
    snd_soc_dapm_route { sink: cstr!("ADC 22 Mux"), control: cstr!("LINE1"), source: cstr!("LINE1") },
    snd_soc_dapm_route { sink: cstr!("ADC 22 Mux"), control: cstr!("LINE2"), source: cstr!("LINE2") },
    snd_soc_dapm_route { sink: cstr!("ADC 22 Mux"), control: cstr!("MIC2"), source: cstr!("MIC2") },
    snd_soc_dapm_route { sink: cstr!("ADC 23 Mux"), control: cstr!("DMIC"), source: cstr!("DMIC2") },
    snd_soc_dapm_route { sink: cstr!("ADC 23 Mux"), control: cstr!("LINE1"), source: cstr!("LINE1") },
    snd_soc_dapm_route { sink: cstr!("ADC 23 Mux"), control: cstr!("LINE2"), source: cstr!("LINE2") },
    snd_soc_dapm_route { sink: cstr!("ADC 23 Mux"), control: cstr!("MIC2"), source: cstr!("MIC2") },
    snd_soc_dapm_route { sink: cstr!("HP"), control: ptr::null(), source: cstr!("DAC Surround") },
];

unsafe extern "C" fn rt711_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);
    let rt711: *mut rt711_priv = snd_soc_component_get_drvdata(component);

    if level == SND_SOC_BIAS_PREPARE {
        if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_STANDBY {
            regmap_write((*rt711).regmap, RT711_SET_AUDIO_POWER_STATE, AC_PWRST_D0);
        }
    } else if level == SND_SOC_BIAS_STANDBY {
        // C used scoped_guard(mutex, &rt711->calibrate_mutex).
        regmap_write((*rt711).regmap, RT711_SET_AUDIO_POWER_STATE, AC_PWRST_D3);
    }

    0
}

unsafe fn rt711_parse_dt(rt711: *mut rt711_priv, dev: *mut device) -> c_int {
    device_property_read_u32(dev, cstr!("realtek,jd-src"), &mut (*rt711).jd_src);

    0
}

unsafe extern "C" fn rt711_probe(component: *mut snd_soc_component) -> c_int {
    let rt711: *mut rt711_priv = snd_soc_component_get_drvdata(component);
    let ret: c_int;

    rt711_parse_dt(rt711, &mut (*(*rt711).slave).dev);
    (*rt711).component = component;

    if !(*rt711).first_hw_init {
        return 0;
    }

    ret = pm_runtime_resume((*component).dev);
    if ret < 0 && ret != -EACCES {
        return ret;
    }

    0
}

static soc_codec_dev_rt711: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rt711_probe),
    set_bias_level: Some(rt711_set_bias_level),
    controls: unsafe { rt711_snd_controls.as_ptr() },
    num_controls: 8,
    dapm_widgets: unsafe { rt711_dapm_widgets.as_ptr() },
    num_dapm_widgets: 14,
    dapm_routes: rt711_audio_map.as_ptr(),
    num_dapm_routes: 14,
    set_jack: Some(rt711_set_jack_detect),
    endianness: 1,
};

unsafe extern "C" fn rt711_set_sdw_stream(
    dai: *mut snd_soc_dai,
    sdw_stream: *mut c_void,
    direction: c_int,
) -> c_int {
    snd_soc_dai_dma_data_set(dai, direction, sdw_stream);

    0
}

unsafe extern "C" fn rt711_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    snd_soc_dai_set_dma_data(dai, substream, ptr::null_mut());
}

unsafe extern "C" fn rt711_pcm_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let rt711: *mut rt711_priv = snd_soc_component_get_drvdata(component);
    let mut stream_config: sdw_stream_config = core::mem::zeroed();
    let mut port_config: sdw_port_config = core::mem::zeroed();
    let sdw_stream: *mut sdw_stream_runtime;
    let mut retval: c_int;
    let mut val: c_uint = 0;

    dev_dbg((*dai).dev, cstr!("%s %s"), cstr!("rt711_pcm_hw_params"), (*dai).name);
    sdw_stream = snd_soc_dai_get_dma_data(dai, substream);

    if sdw_stream.is_null() {
        return -EINVAL;
    }

    if (*rt711).slave.is_null() {
        return -EINVAL;
    }

    /* SoundWire specific configuration */
    snd_sdw_params_to_config(substream, params, &mut stream_config, &mut port_config);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        port_config.num = 3;
    } else {
        if (*dai).id == RT711_AIF1 {
            port_config.num = 4;
        } else if (*dai).id == RT711_AIF2 {
            port_config.num = 2;
        } else {
            return -EINVAL;
        }
    }

    retval = sdw_stream_add_slave((*rt711).slave, &mut stream_config, &mut port_config, 1, sdw_stream);
    if retval != 0 {
        dev_err((*dai).dev, cstr!("%s: Unable to configure port\n"), cstr!("rt711_pcm_hw_params"));
        return retval;
    }

    if params_channels(params) <= 16 {
        /* bit 3:0 Number of Channel */
        val |= params_channels(params) - 1;
    } else {
        dev_err(
            (*component).dev,
            cstr!("%s: Unsupported channels %d\n"),
            cstr!("rt711_pcm_hw_params"),
            params_channels(params),
        );
        return -EINVAL;
    }

    match params_width(params) {
        /* bit 6:4 Bits per Sample */
        8 => {}
        16 => val |= 0x1 << 4,
        20 => val |= 0x2 << 4,
        24 => val |= 0x3 << 4,
        32 => val |= 0x4 << 4,
        _ => return -EINVAL,
    }

    /* 48Khz */
    regmap_write((*rt711).regmap, RT711_DAC_FORMAT_H, val);
    regmap_write((*rt711).regmap, RT711_ADC1_FORMAT_H, val);
    regmap_write((*rt711).regmap, RT711_ADC2_FORMAT_H, val);

    retval
}

unsafe extern "C" fn rt711_pcm_hw_free(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let rt711: *mut rt711_priv = snd_soc_component_get_drvdata(component);
    let sdw_stream: *mut sdw_stream_runtime = snd_soc_dai_get_dma_data(dai, substream);

    if (*rt711).slave.is_null() {
        return -EINVAL;
    }

    sdw_stream_remove_slave((*rt711).slave, sdw_stream);
    0
}

// #define RT711_STEREO_RATES (SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000)
// #define RT711_FORMATS (SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | \
//             SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S8)

static rt711_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(rt711_pcm_hw_params),
    hw_free: Some(rt711_pcm_hw_free),
    set_stream: Some(rt711_set_sdw_stream),
    shutdown: Some(rt711_shutdown),
};

static mut rt711_dai: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: cstr!("rt711-aif1"),
        id: unsafe { RT711_AIF1 },
        playback: snd_soc_pcm_stream {
            stream_name: cstr!("DP3 Playback"),
            channels_min: 1,
            channels_max: 2,
            rates: unsafe { SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 },
            formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S8 },
        },
        capture: snd_soc_pcm_stream {
            stream_name: cstr!("DP4 Capture"),
            channels_min: 1,
            channels_max: 2,
            rates: unsafe { SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 },
            formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S8 },
        },
        ops: &rt711_ops,
    },
    snd_soc_dai_driver {
        name: cstr!("rt711-aif2"),
        id: unsafe { RT711_AIF2 },
        playback: snd_soc_pcm_stream {
            stream_name: ptr::null(),
            channels_min: 0,
            channels_max: 0,
            rates: 0,
            formats: 0,
        },
        capture: snd_soc_pcm_stream {
            stream_name: cstr!("DP2 Capture"),
            channels_min: 1,
            channels_max: 2,
            rates: unsafe { SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 },
            formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S8 },
        },
        ops: &rt711_ops,
    },
];

/* Bus clock frequency */
const RT711_CLK_FREQ_9600000HZ: c_uint = 9600000;
const RT711_CLK_FREQ_12000000HZ: c_uint = 12000000;
const RT711_CLK_FREQ_6000000HZ: c_uint = 6000000;
const RT711_CLK_FREQ_4800000HZ: c_uint = 4800000;
const RT711_CLK_FREQ_2400000HZ: c_uint = 2400000;
const RT711_CLK_FREQ_12288000HZ: c_uint = 12288000;

#[no_mangle]
pub unsafe extern "C" fn rt711_clock_config(dev: *mut device) -> c_int {
    let rt711: *mut rt711_priv = dev_get_drvdata(dev);
    let clk_freq: c_uint;
    let value: c_uint;

    clk_freq = (*rt711).params.curr_dr_freq >> 1;

    match clk_freq {
        RT711_CLK_FREQ_12000000HZ => value = 0x0,
        RT711_CLK_FREQ_6000000HZ => value = 0x1,
        RT711_CLK_FREQ_9600000HZ => value = 0x2,
        RT711_CLK_FREQ_4800000HZ => value = 0x3,
        RT711_CLK_FREQ_2400000HZ => value = 0x4,
        RT711_CLK_FREQ_12288000HZ => value = 0x5,
        _ => return -EINVAL,
    }

    regmap_write((*rt711).regmap, 0xe0, value);
    regmap_write((*rt711).regmap, 0xf0, value);

    dev_dbg(dev, cstr!("%s complete, clk_freq=%d\n"), cstr!("rt711_clock_config"), clk_freq);

    0
}

unsafe extern "C" fn rt711_calibration_work(work: *mut work_struct) {
    // container_of(work, struct rt711_priv, calibration_work)
    let rt711: *mut rt711_priv = work as *mut rt711_priv;

    rt711_calibration(rt711);
}

#[no_mangle]
pub unsafe extern "C" fn rt711_init(
    dev: *mut device,
    sdw_regmap: *mut regmap,
    regmap: *mut regmap,
    slave: *mut sdw_slave,
) -> c_int {
    let rt711: *mut rt711_priv;
    let ret: c_int;

    rt711 = devm_kzalloc(dev, core::mem::size_of::<rt711_priv>(), GFP_KERNEL) as *mut rt711_priv;
    if rt711.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, rt711 as *mut c_void);
    (*rt711).slave = slave;
    (*rt711).sdw_regmap = sdw_regmap;
    (*rt711).regmap = regmap;

    regcache_cache_only((*rt711).regmap, true);

    mutex_init(&mut (*rt711).calibrate_mutex);
    mutex_init(&mut (*rt711).disable_irq_lock);

    INIT_DELAYED_WORK(&mut (*rt711).jack_detect_work, rt711_jack_detect_handler);
    INIT_DELAYED_WORK(&mut (*rt711).jack_btn_check_work, rt711_btn_check_handler);
    INIT_WORK(&mut (*rt711).calibration_work, rt711_calibration_work);

    /*
     * Mark hw_init to false
     * HW init will be performed when device reports present
     */
    (*rt711).hw_init = false;
    (*rt711).first_hw_init = false;

    /* JD source uses JD2 in default */
    (*rt711).jd_src = RT711_JD2;

    ret = devm_snd_soc_register_component(
        dev,
        &soc_codec_dev_rt711,
        rt711_dai.as_mut_ptr(),
        rt711_dai.len() as c_int,
    );
    if ret < 0 {
        return ret;
    }

    /* set autosuspend parameters */
    pm_runtime_set_autosuspend_delay(dev, 3000);
    pm_runtime_use_autosuspend(dev);

    /* make sure the device does not suspend immediately */
    pm_runtime_mark_last_busy(dev);

    pm_runtime_enable(dev);

    /* important note: the device is NOT tagged as 'active' and will remain
     * 'suspended' until the hardware is enumerated/initialized. This is required
     * to make sure the ASoC framework use of pm_runtime_get_sync() does not silently
     * fail with -EACCESS because of race conditions between card creation and enumeration
     */

    dev_dbg(dev, cstr!("%s\n"), cstr!("rt711_init"));

    ret
}

#[no_mangle]
pub unsafe extern "C" fn rt711_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int {
    let rt711: *mut rt711_priv = dev_get_drvdata(dev);

    (*rt711).disable_irq = false;

    if (*rt711).hw_init {
        return 0;
    }

    regcache_cache_only((*rt711).regmap, false);
    if (*rt711).first_hw_init {
        regcache_cache_bypass((*rt711).regmap, true);
    }

    /*
     * PM runtime status is marked as 'active' only when a Slave reports as Attached
     */
    if !(*rt711).first_hw_init {
        /* update count of parent 'active' children */
        pm_runtime_set_active(&mut (*slave).dev);
    }

    pm_runtime_get_noresume(&mut (*slave).dev);

    rt711_reset((*rt711).regmap);

    /* power on */
    regmap_write((*rt711).regmap, RT711_SET_AUDIO_POWER_STATE, AC_PWRST_D0);

    /* Set Pin Widget */
    regmap_write((*rt711).regmap, RT711_SET_PIN_MIC2, 0x25);
    regmap_write((*rt711).regmap, RT711_SET_PIN_HP, 0xc0);
    regmap_write((*rt711).regmap, RT711_SET_PIN_DMIC1, 0x20);
    regmap_write((*rt711).regmap, RT711_SET_PIN_DMIC2, 0x20);
    regmap_write((*rt711).regmap, RT711_SET_PIN_LINE1, 0x20);
    regmap_write((*rt711).regmap, RT711_SET_PIN_LINE2, 0x20);

    /* Mute HP/ADC1/ADC2 */
    regmap_write((*rt711).regmap, RT711_SET_GAIN_HP_H, 0xa080);
    regmap_write((*rt711).regmap, RT711_SET_GAIN_HP_H, 0x9080);
    regmap_write((*rt711).regmap, RT711_SET_GAIN_ADC2_H, 0x6080);
    regmap_write((*rt711).regmap, RT711_SET_GAIN_ADC2_H, 0x5080);
    regmap_write((*rt711).regmap, RT711_SET_GAIN_ADC1_H, 0x6080);
    regmap_write((*rt711).regmap, RT711_SET_GAIN_ADC1_H, 0x5080);

    /* Set Configuration Default */
    regmap_write((*rt711).regmap, 0x4f12, 0x91);
    regmap_write((*rt711).regmap, 0x4e12, 0xd6);
    regmap_write((*rt711).regmap, 0x4d12, 0x11);
    regmap_write((*rt711).regmap, 0x4c12, 0x20);
    regmap_write((*rt711).regmap, 0x4f13, 0x91);
    regmap_write((*rt711).regmap, 0x4e13, 0xd6);
    regmap_write((*rt711).regmap, 0x4d13, 0x11);
    regmap_write((*rt711).regmap, 0x4c13, 0x21);
    regmap_write((*rt711).regmap, 0x4c21, 0xf0);
    regmap_write((*rt711).regmap, 0x4d21, 0x11);
    regmap_write((*rt711).regmap, 0x4e21, 0x11);
    regmap_write((*rt711).regmap, 0x4f21, 0x01);

    /* Data port arrangement */
    rt711_index_write((*rt711).regmap, RT711_VENDOR_REG, RT711_TX_RX_MUX_CTL, 0x0154);

    /* Set index */
    rt711_index_write((*rt711).regmap, RT711_VENDOR_REG, RT711_DIGITAL_MISC_CTRL4, 0x201b);
    rt711_index_write((*rt711).regmap, RT711_VENDOR_REG, RT711_COMBO_JACK_AUTO_CTL1, 0x5089);
    rt711_index_write((*rt711).regmap, RT711_VENDOR_REG, RT711_VREFOUT_CTL, 0x5064);
    rt711_index_write((*rt711).regmap, RT711_VENDOR_REG, RT711_INLINE_CMD_CTL, 0xd249);

    /* Finish Initial Settings, set power to D3 */
    regmap_write((*rt711).regmap, RT711_SET_AUDIO_POWER_STATE, AC_PWRST_D3);

    if (*rt711).first_hw_init {
        rt711_calibration(rt711);
    } else {
        schedule_work(&mut (*rt711).calibration_work);
    }

    /*
     * if set_jack callback occurred early than io_init,
     * we set up the jack detection function now
     */
    if !(*rt711).hs_jack.is_null() {
        rt711_jack_init(rt711);
    }

    if (*rt711).first_hw_init {
        regcache_cache_bypass((*rt711).regmap, false);
        regcache_mark_dirty((*rt711).regmap);
    } else {
        (*rt711).first_hw_init = true;
    }

    /* Mark Slave initialization complete */
    (*rt711).hw_init = true;

    pm_runtime_put_autosuspend(&mut (*slave).dev);

    dev_dbg(&mut (*slave).dev, cstr!("%s hw_init complete\n"), cstr!("rt711_io_init"));
    0
}

// MODULE_DESCRIPTION("ASoC RT711 SDW driver");
// MODULE_AUTHOR("Shuming Fan <shumingf@realtek.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
