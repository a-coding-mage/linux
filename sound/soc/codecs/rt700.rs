// SPDX-License-Identifier: GPL-2.0
//
// rt700.c -- rt700 ALSA SoC audio driver
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

// C includes translated as external dependencies:
// linux/module.h, linux/moduleparam.h, linux/kernel.h, linux/init.h,
// linux/delay.h, linux/pm_runtime.h, linux/pm.h, linux/soundwire/sdw.h,
// linux/regmap.h, linux/slab.h, sound/core.h, sound/pcm.h,
// sound/pcm_params.h, sound/sdw.h, sound/soc.h, sound/soc-dapm.h,
// sound/initval.h, sound/tlv.h, sound/hda_verbs.h, sound/jack.h, "rt700.h".

#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct rt700_priv {
    pub regmap: *mut regmap,
    pub sdw_regmap: *mut regmap,
    pub slave: *mut sdw_slave,
    pub component: *mut snd_soc_component,
    pub hs_jack: *mut snd_soc_jack,
    pub jack_type: c_uint,
    pub first_hw_init: bool,
    pub hw_init: bool,
    pub disable_irq: bool,
    pub disable_irq_lock: mutex,
    pub jack_detect_work: delayed_work,
    pub jack_btn_check_work: delayed_work,
    pub params: rt700_params,
}
#[repr(C)] pub struct rt700_params { pub curr_dr_freq: c_uint }
#[repr(C)] pub struct sdw_slave { pub dev: device }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { pub work: work_struct }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device, pub card: *mut snd_soc_card }
#[repr(C)] pub struct snd_soc_card { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_jack { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { pub private_value: usize }
#[repr(C)] pub struct snd_ctl_elem_value { pub id: snd_ctl_elem_id, pub value: snd_ctl_elem_value_union }
#[repr(C)] pub struct snd_ctl_elem_id { pub name: [c_char; 44] }
#[repr(C)] pub union snd_ctl_elem_value_union { pub integer: snd_ctl_elem_value_integer, pub enumerated: snd_ctl_elem_value_enumerated }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_integer { pub value: [i64; 128] }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_enumerated { pub item: [c_uint; 128] }
#[repr(C)] pub struct soc_mixer_control { pub reg: c_uint, pub rreg: c_uint, pub shift: c_uint, pub max: c_uint, pub invert: c_uint }
#[repr(C)] pub struct soc_enum { pub items: c_uint, pub shift_l: c_uint }
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context }
#[repr(C)] pub struct snd_soc_dapm_route { pub sink: *const c_char, pub control: *const c_char, pub source: *const c_char }
#[repr(C)] pub struct snd_soc_component_driver { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { pub component: *mut snd_soc_component, pub dev: *mut device, pub name: *const c_char, pub id: c_int }
#[repr(C)] pub struct snd_soc_dai_ops { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai_driver { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_substream { pub stream: c_int }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct sdw_stream_config { _private: [u8; 0] }
#[repr(C)] pub struct sdw_port_config { pub num: c_uint }
#[repr(C)] pub struct sdw_stream_runtime { _private: [u8; 0] }

unsafe extern "C" {
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn snd_soc_card_is_instantiated(card: *mut snd_soc_card) -> bool;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn mod_delayed_work(wq: *mut c_void, dwork: *mut delayed_work, delay: c_uint) -> bool;
    fn msecs_to_jiffies(m: c_uint) -> c_uint;
    static mut system_power_efficient_wq: *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut rt700_priv;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dapm_kcontrol_to_component(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dapm_kcontrol_to_dapm(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dapm_context;
    fn snd_soc_enum_item_to_val(e: *mut soc_enum, item: c_uint) -> c_uint;
    fn snd_soc_dapm_mux_update_power(dapm: *mut snd_soc_dapm_context, kcontrol: *mut snd_kcontrol, item: c_uint, e: *mut soc_enum, p: *mut c_void);
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_dai_dma_data_set(dai: *mut snd_soc_dai, direction: c_int, data: *mut c_void);
    fn snd_soc_dai_set_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream, data: *mut c_void);
    fn snd_soc_dai_get_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream) -> *mut sdw_stream_runtime;
    fn snd_sdw_params_to_config(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, sc: *mut sdw_stream_config, pc: *mut sdw_port_config);
    fn sdw_stream_add_slave(slave: *mut sdw_slave, sc: *mut sdw_stream_config, pc: *mut sdw_port_config, n: c_uint, rt: *mut sdw_stream_runtime) -> c_int;
    fn sdw_stream_remove_slave(slave: *mut sdw_slave, rt: *mut sdw_stream_runtime);
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_get_drvdata(dev: *mut device) -> *mut rt700_priv;
    fn dev_set_drvdata(dev: *mut device, data: *mut rt700_priv);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_cache_bypass(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn mutex_init(m: *mut mutex);
    fn devm_snd_soc_register_component(dev: *mut device, driver: *const snd_soc_component_driver, dai: *mut snd_soc_dai_driver, num: c_int) -> c_int;
    fn pm_runtime_resume(dev: *mut device) -> c_int;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device);
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
}

// External constants from included headers.
const RT700_PRIV_INDEX_W_H: c_uint = 0;
const RT700_IRQ_FLAG_TABLE1: c_uint = 0;
const RT700_IRQ_FLAG_TABLE2: c_uint = 0;
const RT700_COMBO_JACK_AUTO_CTL2: c_uint = 0;
const RT700_COMBOJACK_AUTO_DET_STATUS: c_uint = 0;
const RT700_COMBOJACK_AUTO_DET_TRS: c_uint = 0;
const RT700_COMBOJACK_AUTO_DET_CTIA: c_uint = 0;
const RT700_COMBOJACK_AUTO_DET_OMTP: c_uint = 0;
const RT700_VERB_GET_PIN_SENSE: c_uint = 0;
const RT700_HP_OUT: c_uint = 0;
const RT700_SET_AUDIO_POWER_STATE: c_uint = 0;
const RT700_SET_MIC2_UNSOLICITED_ENABLE: c_uint = 0;
const RT700_SET_HP_UNSOLICITED_ENABLE: c_uint = 0;
const RT700_SET_INLINE_UNSOLICITED_ENABLE: c_uint = 0;
const RT700_DIR_OUT_SFT: c_uint = 0;
const RT700_DIR_IN_SFT: c_uint = 0;
const RT700_MUTE_SFT: c_uint = 7;
const RT700_SET_GAIN_DAC1_H: c_uint = 0;
const RT700_SET_GAIN_DAC1_L: c_uint = 0;
const RT700_SET_GAIN_ADC2_H: c_uint = 0;
const RT700_SET_GAIN_ADC2_L: c_uint = 0;
const RT700_SET_GAIN_ADC1_H: c_uint = 0;
const RT700_SET_GAIN_ADC1_L: c_uint = 0;
const RT700_SET_GAIN_AMIC_H: c_uint = 0;
const RT700_SET_GAIN_AMIC_L: c_uint = 0;
const RT700_MIXER_IN1: c_uint = 0;
const RT700_MIXER_IN2: c_uint = 0;
const RT700_VERB_SET_CONNECT_SEL: c_uint = 0;
const RT700_SET_STREAMID_DAC1: c_uint = 0;
const RT700_SET_STREAMID_DAC2: c_uint = 0;
const RT700_SET_STREAMID_ADC1: c_uint = 0;
const RT700_SET_STREAMID_ADC2: c_uint = 0;
const RT700_SET_GAIN_HP_H: c_uint = 0;
const RT700_SET_GAIN_SPK_H: c_uint = 0;
const RT700_AIF1: c_int = 0;
const RT700_AIF2: c_int = 1;
const RT700_DAC_FORMAT_H: c_uint = 0;
const RT700_ADC_FORMAT_H: c_uint = 0;
const RT700_SET_PIN_HP: c_uint = 0;
const RT700_SET_PIN_SPK: c_uint = 0;
const RT700_SET_EAPD_SPK: c_uint = 0;
const RT700_EAPD_HIGH: c_uint = 0;
const RT700_SET_PIN_DMIC1: c_uint = 0;
const RT700_SET_PIN_DMIC2: c_uint = 0;
const RT700_SET_PIN_MIC2: c_uint = 0;
const AC_PWRST_D0: c_uint = 0;
const AC_PWRST_D3: c_uint = 0;
const SND_JACK_BTN_0: c_uint = 1 << 0;
const SND_JACK_BTN_1: c_uint = 1 << 1;
const SND_JACK_BTN_2: c_uint = 1 << 2;
const SND_JACK_BTN_3: c_uint = 1 << 3;
const SND_JACK_HEADPHONE: c_uint = 1 << 4;
const SND_JACK_HEADSET: c_uint = 1 << 5;
const SND_SOC_BIAS_STANDBY: c_int = 1;
const SND_SOC_BIAS_PREPARE: c_int = 2;
const SND_SOC_DAPM_POST_PMU: c_int = 1;
const SND_SOC_DAPM_PRE_PMD: c_int = 2;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const ETIMEDOUT: c_int = 110;
const EACCES: c_int = 13;
const GFP_KERNEL: c_uint = 0;
const SNDRV_PCM_RATE_44100: c_uint = 1 << 0;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 1;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << 0;
const SNDRV_PCM_FMTBIT_S20_3LE: c_uint = 1 << 1;
const SNDRV_PCM_FMTBIT_S24_LE: c_uint = 1 << 2;
const SNDRV_PCM_FMTBIT_S8: c_uint = 1 << 3;

unsafe fn rt700_index_write(regmap: *mut regmap, reg: c_uint, value: c_uint) -> c_int {
    let addr: c_uint = (RT700_PRIV_INDEX_W_H << 8) | reg;
    let ret = regmap_write(regmap, addr, value);
    if ret < 0 {
        // pr_err("%s: Failed to set private value: %06x <= %04x ret=%d\n", __func__, addr, value, ret);
    }
    ret
}

unsafe fn rt700_index_read(regmap: *mut regmap, reg: c_uint, value: *mut c_uint) -> c_int {
    let addr: c_uint = (RT700_PRIV_INDEX_W_H << 8) | reg;
    *value = 0;
    let ret = regmap_read(regmap, addr, value);
    if ret < 0 {
        // pr_err("%s: Failed to get private value: %06x => %04x ret=%d\n", __func__, addr, *value, ret);
    }
    ret
}

unsafe fn rt700_button_detect(rt700: *mut rt700_priv) -> c_uint {
    let mut btn_type: c_uint = 0;
    let mut val80: c_uint = 0;
    let mut val81: c_uint = 0;
    let mut ret: c_int;

    ret = rt700_index_read((*rt700).regmap, RT700_IRQ_FLAG_TABLE1, &mut val80);
    if ret < 0 { return btn_type; }
    ret = rt700_index_read((*rt700).regmap, RT700_IRQ_FLAG_TABLE2, &mut val81);
    if ret < 0 { return btn_type; }

    val80 &= 0x0381;
    val81 &= 0xff00;

    match val80 {
        0x0200 | 0x0100 | 0x0080 => btn_type |= SND_JACK_BTN_0,
        0x0001 => btn_type |= SND_JACK_BTN_3,
        _ => {}
    }
    match val81 {
        0x8000 | 0x4000 | 0x2000 => btn_type |= SND_JACK_BTN_1,
        0x1000 | 0x0800 | 0x0400 => btn_type |= SND_JACK_BTN_2,
        0x0200 | 0x0100 => btn_type |= SND_JACK_BTN_3,
        _ => {}
    }
    btn_type
}

unsafe fn rt700_headset_detect(rt700: *mut rt700_priv) -> c_int {
    let mut buf: c_uint = 0;
    let mut loop_count: c_uint = 0;
    let mut jack_status: c_uint = 0;
    let mut reg: c_uint;
    let mut ret = rt700_index_read((*rt700).regmap, RT700_COMBO_JACK_AUTO_CTL2, &mut buf);
    if ret < 0 { return ret; }

    while loop_count < 500 && (buf & RT700_COMBOJACK_AUTO_DET_STATUS) == 0 {
        loop_count += 1;
        usleep_range(9000, 10000);
        ret = rt700_index_read((*rt700).regmap, RT700_COMBO_JACK_AUTO_CTL2, &mut buf);
        if ret < 0 { return ret; }
        reg = RT700_VERB_GET_PIN_SENSE | RT700_HP_OUT;
        ret = regmap_read((*rt700).regmap, reg, &mut jack_status);
        if (jack_status & (1 << 31)) == 0 { return -ENODEV; }
    }

    if loop_count >= 500 { return -ETIMEDOUT; }

    if (buf & RT700_COMBOJACK_AUTO_DET_TRS) != 0 {
        (*rt700).jack_type = SND_JACK_HEADPHONE;
    } else if (buf & RT700_COMBOJACK_AUTO_DET_CTIA) != 0 || (buf & RT700_COMBOJACK_AUTO_DET_OMTP) != 0 {
        (*rt700).jack_type = SND_JACK_HEADSET;
    }
    0
}

unsafe fn rt700_jack_detect_handler(work: *mut work_struct) {
    let rt700 = container_of_rt700_priv_jack_detect_work(work);
    let mut btn_type: c_int = 0;
    let mut jack_status: c_uint = 0;
    let reg: c_uint;

    if (*rt700).hs_jack.is_null() { return; }
    if !snd_soc_card_is_instantiated((*(*rt700).component).card) { return; }

    reg = RT700_VERB_GET_PIN_SENSE | RT700_HP_OUT;
    let ret = regmap_read((*rt700).regmap, reg, &mut jack_status);
    if ret < 0 { return; }

    /* pin attached */
    if (jack_status & (1 << 31)) != 0 {
        /* jack in */
        if (*rt700).jack_type == 0 {
            let ret = rt700_headset_detect(rt700);
            if ret < 0 { return; }
            if (*rt700).jack_type == SND_JACK_HEADSET {
                btn_type = rt700_button_detect(rt700) as c_int;
            }
        } else if (*rt700).jack_type == SND_JACK_HEADSET {
            /* jack is already in, report button event */
            btn_type = rt700_button_detect(rt700) as c_int;
        }
    } else {
        /* jack out */
        (*rt700).jack_type = 0;
    }

    snd_soc_jack_report((*rt700).hs_jack, ((*rt700).jack_type as c_int) | btn_type,
        (SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3) as c_int);

    if btn_type != 0 {
        /* button released */
        snd_soc_jack_report((*rt700).hs_jack, (*rt700).jack_type as c_int,
            (SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3) as c_int);
        mod_delayed_work(system_power_efficient_wq, &mut (*rt700).jack_btn_check_work, msecs_to_jiffies(200));
    }
}

unsafe fn rt700_btn_check_handler(work: *mut work_struct) {
    let rt700 = container_of_rt700_priv_jack_btn_check_work(work);
    let mut btn_type: c_int = 0;
    let mut jack_status: c_uint = 0;
    let mut reg: c_uint;

    reg = RT700_VERB_GET_PIN_SENSE | RT700_HP_OUT;
    let ret = regmap_read((*rt700).regmap, reg, &mut jack_status);
    if ret < 0 { return; }

    /* pin attached */
    if (jack_status & (1 << 31)) != 0 {
        if (*rt700).jack_type == SND_JACK_HEADSET {
            /* jack is already in, report button event */
            btn_type = rt700_button_detect(rt700) as c_int;
        }
    } else {
        (*rt700).jack_type = 0;
    }

    /* cbj comparator */
    let ret = rt700_index_read((*rt700).regmap, RT700_COMBO_JACK_AUTO_CTL2, &mut reg);
    if ret < 0 { return; }
    if (reg & 0xf0) == 0xf0 { btn_type = 0; }

    snd_soc_jack_report((*rt700).hs_jack, ((*rt700).jack_type as c_int) | btn_type,
        (SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3) as c_int);

    if btn_type != 0 {
        /* button released */
        snd_soc_jack_report((*rt700).hs_jack, (*rt700).jack_type as c_int,
            (SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3) as c_int);
        mod_delayed_work(system_power_efficient_wq, &mut (*rt700).jack_btn_check_work, msecs_to_jiffies(200));
    }
}

unsafe fn rt700_jack_init(rt700: *mut rt700_priv) {
    let dapm = snd_soc_component_to_dapm((*rt700).component);
    /* power on */
    if snd_soc_dapm_get_bias_level(dapm) <= SND_SOC_BIAS_STANDBY {
        regmap_write((*rt700).regmap, RT700_SET_AUDIO_POWER_STATE, AC_PWRST_D0);
    }

    if !(*rt700).hs_jack.is_null() {
        /* Enable Jack Detection */
        regmap_write((*rt700).regmap, RT700_SET_MIC2_UNSOLICITED_ENABLE, 0x82);
        regmap_write((*rt700).regmap, RT700_SET_HP_UNSOLICITED_ENABLE, 0x81);
        regmap_write((*rt700).regmap, RT700_SET_INLINE_UNSOLICITED_ENABLE, 0x83);
        rt700_index_write((*rt700).regmap, 0x10, 0x2420);
        rt700_index_write((*rt700).regmap, 0x19, 0x2e11);
        mod_delayed_work(system_power_efficient_wq, &mut (*rt700).jack_detect_work, msecs_to_jiffies(250));
    } else {
        regmap_write((*rt700).regmap, RT700_SET_MIC2_UNSOLICITED_ENABLE, 0x00);
        regmap_write((*rt700).regmap, RT700_SET_HP_UNSOLICITED_ENABLE, 0x00);
        regmap_write((*rt700).regmap, RT700_SET_INLINE_UNSOLICITED_ENABLE, 0x00);
    }

    /* power off */
    if snd_soc_dapm_get_bias_level(dapm) <= SND_SOC_BIAS_STANDBY {
        regmap_write((*rt700).regmap, RT700_SET_AUDIO_POWER_STATE, AC_PWRST_D3);
    }
}

unsafe fn rt700_set_jack_detect(component: *mut snd_soc_component, hs_jack: *mut snd_soc_jack, _data: *mut c_void) -> c_int {
    let rt700 = snd_soc_component_get_drvdata(component);
    (*rt700).hs_jack = hs_jack;

    /* we can only resume if the device was initialized at least once */
    if !(*rt700).first_hw_init { return 0; }

    let ret = pm_runtime_resume_and_get((*component).dev);
    if ret < 0 {
        if ret != -EACCES { return ret; }
        /* pm_runtime not enabled yet */
        return 0;
    }
    rt700_jack_init(rt700);
    pm_runtime_put_autosuspend((*component).dev);
    0
}

unsafe fn rt700_get_gain(rt700: *mut rt700_priv, addr_h: c_uint, addr_l: c_uint, mut val_h: c_uint, r_val: *mut c_uint, l_val: *mut c_uint) {
    /* R Channel */
    *r_val = val_h << 8;
    regmap_read((*rt700).regmap, addr_l, r_val);

    /* L Channel */
    val_h |= 0x20;
    *l_val = val_h << 8;
    regmap_read((*rt700).regmap, addr_h, l_val);
}

/* For Verb-Set Amplifier Gain (Verb ID = 3h) */
unsafe fn rt700_set_amp_gain_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let dapm = snd_soc_component_to_dapm(component);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let rt700 = snd_soc_component_get_drvdata(component);
    let addr_h = (*mc).reg;
    let addr_l = (*mc).rreg;
    let mut val_h: c_uint = if (*mc).shift == RT700_DIR_OUT_SFT { 0x80 } else { 0x0 };
    let mut read_ll: c_uint = 0;
    let mut read_rl: c_uint = 0;

    /* Can't use update bit function, so read the original value first */
    rt700_get_gain(rt700, addr_h, addr_l, val_h, &mut read_rl, &mut read_ll);

    let mut val_ll: c_uint;
    let mut val_lr: c_uint;
    /* L Channel */
    if (*mc).invert != 0 {
        /* for mute */
        val_ll = ((*mc).max - (*ucontrol).value.integer.value[0] as c_uint) << 7;
        /* keep gain */
        read_ll &= 0x7f;
        val_ll |= read_ll;
    } else {
        /* for gain */
        val_ll = ((*ucontrol).value.integer.value[0] as c_uint) & 0x7f;
        if val_ll > (*mc).max { val_ll = (*mc).max; }
        /* keep mute status */
        read_ll &= 0x80;
        val_ll |= read_ll;
    }

    if snd_soc_dapm_get_bias_level(dapm) <= SND_SOC_BIAS_STANDBY {
        regmap_write((*rt700).regmap, RT700_SET_AUDIO_POWER_STATE, AC_PWRST_D0);
    }

    /* R Channel */
    if (*mc).invert != 0 {
        /* for mute */
        val_lr = ((*mc).max - (*ucontrol).value.integer.value[1] as c_uint) << 7;
        /* keep gain */
        read_rl &= 0x7f;
        val_lr |= read_rl;
    } else {
        /* for gain */
        val_lr = ((*ucontrol).value.integer.value[1] as c_uint) & 0x7f;
        if val_lr > (*mc).max { val_lr = (*mc).max; }
        /* keep mute status */
        read_rl &= 0x80;
        val_lr |= read_rl;
    }

    for _i in 0..3 {
        /* retry 3 times at most */
        if val_ll == val_lr {
            /* Set both L/R channels at the same time */
            val_h = (1 << (*mc).shift) | (3 << 4);
            regmap_write((*rt700).regmap, addr_h, (val_h << 8) | val_ll);
            regmap_write((*rt700).regmap, addr_l, (val_h << 8) | val_ll);
        } else {
            /* Lch*/
            val_h = (1 << (*mc).shift) | (1 << 5);
            regmap_write((*rt700).regmap, addr_h, (val_h << 8) | val_ll);

            /* Rch */
            val_h = (1 << (*mc).shift) | (1 << 4);
            regmap_write((*rt700).regmap, addr_l, (val_h << 8) | val_lr);
        }
        /* check result */
        val_h = if (*mc).shift == RT700_DIR_OUT_SFT { 0x80 } else { 0x0 };
        rt700_get_gain(rt700, addr_h, addr_l, val_h, &mut read_rl, &mut read_ll);
        if read_rl == val_lr && read_ll == val_ll { break; }
    }

    if snd_soc_dapm_get_bias_level(dapm) <= SND_SOC_BIAS_STANDBY {
        regmap_write((*rt700).regmap, RT700_SET_AUDIO_POWER_STATE, AC_PWRST_D3);
    }
    0
}

unsafe fn rt700_set_amp_gain_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt700 = snd_soc_component_get_drvdata(component);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let addr_h = (*mc).reg;
    let addr_l = (*mc).rreg;
    let val_h: c_uint = if (*mc).shift == RT700_DIR_OUT_SFT { 0x80 } else { 0x0 };
    let mut read_ll: c_uint = 0;
    let mut read_rl: c_uint = 0;

    rt700_get_gain(rt700, addr_h, addr_l, val_h, &mut read_rl, &mut read_ll);

    if (*mc).invert != 0 {
        /* for mute status */
        read_ll = if ((read_ll & 0x80) >> RT700_MUTE_SFT) == 0 { 1 } else { 0 };
        read_rl = if ((read_rl & 0x80) >> RT700_MUTE_SFT) == 0 { 1 } else { 0 };
    } else {
        /* for gain */
        read_ll &= 0x7f;
        read_rl &= 0x7f;
    }
    (*ucontrol).value.integer.value[0] = read_ll as i64;
    (*ucontrol).value.integer.value[1] = read_rl as i64;
    0
}

// static const DECLARE_TLV_DB_SCALE(out_vol_tlv, -6525, 75, 0);
// static const DECLARE_TLV_DB_SCALE(in_vol_tlv, -1725, 75, 0);
// static const DECLARE_TLV_DB_SCALE(mic_vol_tlv, 0, 1000, 0);

// static const struct snd_kcontrol_new rt700_snd_controls[] = {
//     SOC_DOUBLE_R_EXT_TLV("DAC Front Playback Volume", RT700_SET_GAIN_DAC1_H, RT700_SET_GAIN_DAC1_L, RT700_DIR_OUT_SFT, 0x57, 0, rt700_set_amp_gain_get, rt700_set_amp_gain_put, out_vol_tlv),
//     SOC_DOUBLE_R_EXT("ADC 08 Capture Switch", RT700_SET_GAIN_ADC2_H, RT700_SET_GAIN_ADC2_L, RT700_DIR_IN_SFT, 1, 1, rt700_set_amp_gain_get, rt700_set_amp_gain_put),
//     SOC_DOUBLE_R_EXT("ADC 09 Capture Switch", RT700_SET_GAIN_ADC1_H, RT700_SET_GAIN_ADC1_L, RT700_DIR_IN_SFT, 1, 1, rt700_set_amp_gain_get, rt700_set_amp_gain_put),
//     SOC_DOUBLE_R_EXT_TLV("ADC 08 Capture Volume", RT700_SET_GAIN_ADC2_H, RT700_SET_GAIN_ADC2_L, RT700_DIR_IN_SFT, 0x3f, 0, rt700_set_amp_gain_get, rt700_set_amp_gain_put, in_vol_tlv),
//     SOC_DOUBLE_R_EXT_TLV("ADC 09 Capture Volume", RT700_SET_GAIN_ADC1_H, RT700_SET_GAIN_ADC1_L, RT700_DIR_IN_SFT, 0x3f, 0, rt700_set_amp_gain_get, rt700_set_amp_gain_put, in_vol_tlv),
//     SOC_DOUBLE_R_EXT_TLV("AMIC Volume", RT700_SET_GAIN_AMIC_H, RT700_SET_GAIN_AMIC_L, RT700_DIR_IN_SFT, 3, 0, rt700_set_amp_gain_get, rt700_set_amp_gain_put, mic_vol_tlv),
// };

unsafe fn name_contains(name: *const c_char, needle: &[u8]) -> bool {
    strstr(name, needle.as_ptr() as *const c_char) != ptr::null_mut()
}

unsafe fn rt700_mux_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let rt700 = snd_soc_component_get_drvdata(component);
    let mut val: c_uint = 0;
    let nid: c_uint;

    if name_contains((*ucontrol).id.name.as_ptr(), b"HPO Mux\0") {
        nid = RT700_HP_OUT;
    } else if name_contains((*ucontrol).id.name.as_ptr(), b"ADC 22 Mux\0") {
        nid = RT700_MIXER_IN1;
    } else if name_contains((*ucontrol).id.name.as_ptr(), b"ADC 23 Mux\0") {
        nid = RT700_MIXER_IN2;
    } else {
        return -EINVAL;
    }

    /* vid = 0xf01 */
    let reg = RT700_VERB_SET_CONNECT_SEL | nid;
    let ret = regmap_read((*rt700).regmap, reg, &mut val);
    if ret < 0 { return ret; }
    (*ucontrol).value.enumerated.item[0] = val;
    0
}

unsafe fn rt700_mux_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let rt700 = snd_soc_component_get_drvdata(component);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let item = (*ucontrol).value.enumerated.item.as_mut_ptr();
    let nid: c_uint;
    let mut val2: c_uint = 0;
    let change: c_uint;

    if *item >= (*e).items { return -EINVAL; }

    if name_contains((*ucontrol).id.name.as_ptr(), b"HPO Mux\0") {
        nid = RT700_HP_OUT;
    } else if name_contains((*ucontrol).id.name.as_ptr(), b"ADC 22 Mux\0") {
        nid = RT700_MIXER_IN1;
    } else if name_contains((*ucontrol).id.name.as_ptr(), b"ADC 23 Mux\0") {
        nid = RT700_MIXER_IN2;
    } else {
        return -EINVAL;
    }

    /* Verb ID = 0x701h */
    let val = snd_soc_enum_item_to_val(e, *item) << (*e).shift_l;
    let mut reg = RT700_VERB_SET_CONNECT_SEL | nid;
    let ret = regmap_read((*rt700).regmap, reg, &mut val2);
    if ret < 0 { return ret; }

    change = if val == val2 { 0 } else { 1 };
    if change != 0 {
        reg = RT700_VERB_SET_CONNECT_SEL | nid;
        regmap_write((*rt700).regmap, reg, val);
    }
    snd_soc_dapm_mux_update_power(dapm, kcontrol, *item, e, ptr::null_mut());
    change as c_int
}

static adc_mux_text: [&[u8]; 4] = [b"MIC2\0", b"LINE1\0", b"LINE2\0", b"DMIC\0"];
// static SOC_ENUM_SINGLE_DECL(rt700_adc22_enum, SND_SOC_NOPM, 0, adc_mux_text);
// static SOC_ENUM_SINGLE_DECL(rt700_adc23_enum, SND_SOC_NOPM, 0, adc_mux_text);
// static const struct snd_kcontrol_new rt700_adc22_mux = SOC_DAPM_ENUM_EXT("ADC 22 Mux", rt700_adc22_enum, rt700_mux_get, rt700_mux_put);
// static const struct snd_kcontrol_new rt700_adc23_mux = SOC_DAPM_ENUM_EXT("ADC 23 Mux", rt700_adc23_enum, rt700_mux_get, rt700_mux_put);

static out_mux_text: [&[u8]; 2] = [b"Front\0", b"Surround\0"];
// static SOC_ENUM_SINGLE_DECL(rt700_hp_enum, SND_SOC_NOPM, 0, out_mux_text);
// static const struct snd_kcontrol_new rt700_hp_mux = SOC_DAPM_ENUM_EXT("HP Mux", rt700_hp_enum, rt700_mux_get, rt700_mux_put);

unsafe fn rt700_dac_front_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt700 = snd_soc_component_get_drvdata(component);
    match event {
        SND_SOC_DAPM_POST_PMU => { regmap_write((*rt700).regmap, RT700_SET_STREAMID_DAC1, 0x10); }
        SND_SOC_DAPM_PRE_PMD => { regmap_write((*rt700).regmap, RT700_SET_STREAMID_DAC1, 0x00); }
        _ => {}
    }
    0
}

unsafe fn rt700_dac_surround_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt700 = snd_soc_component_get_drvdata(component);
    match event {
        SND_SOC_DAPM_POST_PMU => { regmap_write((*rt700).regmap, RT700_SET_STREAMID_DAC2, 0x10); }
        SND_SOC_DAPM_PRE_PMD => { regmap_write((*rt700).regmap, RT700_SET_STREAMID_DAC2, 0x00); }
        _ => {}
    }
    0
}

unsafe fn rt700_adc_09_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt700 = snd_soc_component_get_drvdata(component);
    match event {
        SND_SOC_DAPM_POST_PMU => { regmap_write((*rt700).regmap, RT700_SET_STREAMID_ADC1, 0x10); }
        SND_SOC_DAPM_PRE_PMD => { regmap_write((*rt700).regmap, RT700_SET_STREAMID_ADC1, 0x00); }
        _ => {}
    }
    0
}

unsafe fn rt700_adc_08_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt700 = snd_soc_component_get_drvdata(component);
    match event {
        SND_SOC_DAPM_POST_PMU => { regmap_write((*rt700).regmap, RT700_SET_STREAMID_ADC2, 0x10); }
        SND_SOC_DAPM_PRE_PMD => { regmap_write((*rt700).regmap, RT700_SET_STREAMID_ADC2, 0x00); }
        _ => {}
    }
    0
}

unsafe fn rt700_hpo_mux_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt700 = snd_soc_component_get_drvdata(component);
    let val_h: c_uint = (1 << RT700_DIR_OUT_SFT) | (0x3 << 4);
    let val_l: c_uint;
    match event {
        SND_SOC_DAPM_POST_PMU => {
            val_l = 0x00;
            regmap_write((*rt700).regmap, RT700_SET_GAIN_HP_H, (val_h << 8) | val_l);
        }
        SND_SOC_DAPM_PRE_PMD => {
            val_l = 1 << RT700_MUTE_SFT;
            regmap_write((*rt700).regmap, RT700_SET_GAIN_HP_H, (val_h << 8) | val_l);
            usleep_range(50000, 55000);
        }
        _ => {}
    }
    0
}

unsafe fn rt700_spk_pga_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt700 = snd_soc_component_get_drvdata(component);
    let val_h: c_uint = (1 << RT700_DIR_OUT_SFT) | (0x3 << 4);
    let val_l: c_uint;
    match event {
        SND_SOC_DAPM_POST_PMU => {
            val_l = 0x00;
            regmap_write((*rt700).regmap, RT700_SET_GAIN_SPK_H, (val_h << 8) | val_l);
        }
        SND_SOC_DAPM_PRE_PMD => {
            val_l = 1 << RT700_MUTE_SFT;
            regmap_write((*rt700).regmap, RT700_SET_GAIN_SPK_H, (val_h << 8) | val_l);
        }
        _ => {}
    }
    0
}

// static const struct snd_soc_dapm_widget rt700_dapm_widgets[] = { SND_SOC_DAPM_OUTPUT("HP"), SND_SOC_DAPM_OUTPUT("SPK"), SND_SOC_DAPM_INPUT("DMIC1"), SND_SOC_DAPM_INPUT("DMIC2"), SND_SOC_DAPM_INPUT("MIC2"), SND_SOC_DAPM_INPUT("LINE1"), SND_SOC_DAPM_INPUT("LINE2"), SND_SOC_DAPM_DAC_E("DAC Front", NULL, SND_SOC_NOPM, 0, 0, rt700_dac_front_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD), SND_SOC_DAPM_DAC_E("DAC Surround", NULL, SND_SOC_NOPM, 0, 0, rt700_dac_surround_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD), SND_SOC_DAPM_MUX_E("HPO Mux", SND_SOC_NOPM, 0, 0, &rt700_hp_mux, rt700_hpo_mux_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD), SND_SOC_DAPM_PGA_E("SPK PGA", SND_SOC_NOPM, 0, 0, NULL, 0, rt700_spk_pga_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD), SND_SOC_DAPM_ADC_E("ADC 09", NULL, SND_SOC_NOPM, 0, 0, rt700_adc_09_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD), SND_SOC_DAPM_ADC_E("ADC 08", NULL, SND_SOC_NOPM, 0, 0, rt700_adc_08_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD), SND_SOC_DAPM_MUX("ADC 22 Mux", SND_SOC_NOPM, 0, 0, &rt700_adc22_mux), SND_SOC_DAPM_MUX("ADC 23 Mux", SND_SOC_NOPM, 0, 0, &rt700_adc23_mux), SND_SOC_DAPM_AIF_IN("DP1RX", "DP1 Playback", 0, SND_SOC_NOPM, 0, 0), SND_SOC_DAPM_AIF_IN("DP3RX", "DP3 Playback", 0, SND_SOC_NOPM, 0, 0), SND_SOC_DAPM_AIF_OUT("DP2TX", "DP2 Capture", 0, SND_SOC_NOPM, 0, 0), SND_SOC_DAPM_AIF_OUT("DP4TX", "DP4 Capture", 0, SND_SOC_NOPM, 0, 0) };
// static const struct snd_soc_dapm_route rt700_audio_map[] = { {"DAC Front", NULL, "DP1RX"}, {"DAC Surround", NULL, "DP3RX"}, {"DP2TX", NULL, "ADC 09"}, {"DP4TX", NULL, "ADC 08"}, {"ADC 09", NULL, "ADC 22 Mux"}, {"ADC 08", NULL, "ADC 23 Mux"}, {"ADC 22 Mux", "DMIC", "DMIC1"}, {"ADC 22 Mux", "LINE1", "LINE1"}, {"ADC 22 Mux", "LINE2", "LINE2"}, {"ADC 22 Mux", "MIC2", "MIC2"}, {"ADC 23 Mux", "DMIC", "DMIC2"}, {"ADC 23 Mux", "LINE1", "LINE1"}, {"ADC 23 Mux", "LINE2", "LINE2"}, {"ADC 23 Mux", "MIC2", "MIC2"}, {"HPO Mux", "Front", "DAC Front"}, {"HPO Mux", "Surround", "DAC Surround"}, {"HP", NULL, "HPO Mux"}, {"SPK PGA", NULL, "DAC Front"}, {"SPK", NULL, "SPK PGA"} };

unsafe fn rt700_probe(component: *mut snd_soc_component) -> c_int {
    let rt700 = snd_soc_component_get_drvdata(component);
    (*rt700).component = component;
    if !(*rt700).first_hw_init { return 0; }
    let ret = pm_runtime_resume((*component).dev);
    if ret < 0 && ret != -EACCES { return ret; }
    0
}

unsafe fn rt700_set_bias_level(component: *mut snd_soc_component, level: c_int) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let rt700 = snd_soc_component_get_drvdata(component);
    match level {
        SND_SOC_BIAS_PREPARE => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_STANDBY {
                regmap_write((*rt700).regmap, RT700_SET_AUDIO_POWER_STATE, AC_PWRST_D0);
            }
        }
        SND_SOC_BIAS_STANDBY => {
            regmap_write((*rt700).regmap, RT700_SET_AUDIO_POWER_STATE, AC_PWRST_D3);
        }
        _ => {}
    }
    0
}

// static const struct snd_soc_component_driver soc_codec_dev_rt700 = {
//     .probe = rt700_probe, .set_bias_level = rt700_set_bias_level,
//     .controls = rt700_snd_controls, .num_controls = ARRAY_SIZE(rt700_snd_controls),
//     .dapm_widgets = rt700_dapm_widgets, .num_dapm_widgets = ARRAY_SIZE(rt700_dapm_widgets),
//     .dapm_routes = rt700_audio_map, .num_dapm_routes = ARRAY_SIZE(rt700_audio_map),
//     .set_jack = rt700_set_jack_detect, .endianness = 1,
// };
static soc_codec_dev_rt700: snd_soc_component_driver = snd_soc_component_driver { _private: [] };

unsafe fn rt700_set_sdw_stream(dai: *mut snd_soc_dai, sdw_stream: *mut c_void, direction: c_int) -> c_int {
    snd_soc_dai_dma_data_set(dai, direction, sdw_stream);
    0
}

unsafe fn rt700_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    snd_soc_dai_set_dma_data(dai, substream, ptr::null_mut());
}

unsafe fn rt700_pcm_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let rt700 = snd_soc_component_get_drvdata(component);
    let mut stream_config: sdw_stream_config = core::mem::zeroed();
    let mut port_config: sdw_port_config = core::mem::zeroed();
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    let mut val: c_uint = 0;

    if sdw_stream.is_null() { return -EINVAL; }
    if (*rt700).slave.is_null() { return -EINVAL; }

    /* SoundWire specific configuration */
    snd_sdw_params_to_config(substream, params, &mut stream_config, &mut port_config);

    /* This code assumes port 1 for playback and port 2 for capture */
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK { port_config.num = 1; } else { port_config.num = 2; }

    match (*dai).id {
        RT700_AIF1 => {}
        RT700_AIF2 => { port_config.num += 2; }
        _ => { return -EINVAL; }
    }

    let retval = sdw_stream_add_slave((*rt700).slave, &mut stream_config, &mut port_config, 1, sdw_stream);
    if retval != 0 { return retval; }

    if params_channels(params) <= 16 {
        /* bit 3:0 Number of Channel */
        val |= params_channels(params) - 1;
    } else {
        return -EINVAL;
    }

    match params_width(params) {
        /* bit 6:4 Bits per Sample */
        8 => {}
        16 => { val |= 0x1 << 4; }
        20 => { val |= 0x2 << 4; }
        24 => { val |= 0x3 << 4; }
        32 => { val |= 0x4 << 4; }
        _ => { return -EINVAL; }
    }

    /* 48Khz */
    regmap_write((*rt700).regmap, RT700_DAC_FORMAT_H, val);
    regmap_write((*rt700).regmap, RT700_ADC_FORMAT_H, val);
    retval
}

unsafe fn rt700_pcm_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let rt700 = snd_soc_component_get_drvdata(component);
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    if (*rt700).slave.is_null() { return -EINVAL; }
    sdw_stream_remove_slave((*rt700).slave, sdw_stream);
    0
}

const RT700_STEREO_RATES: c_uint = SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000;
const RT700_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S8;

// static const struct snd_soc_dai_ops rt700_ops = { .hw_params = rt700_pcm_hw_params, .hw_free = rt700_pcm_hw_free, .set_stream = rt700_set_sdw_stream, .shutdown = rt700_shutdown };
static mut rt700_dai: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver { _private: [] }, /* name "rt700-aif1", id RT700_AIF1, playback DP1, capture DP2, ops &rt700_ops */
    snd_soc_dai_driver { _private: [] }, /* name "rt700-aif2", id RT700_AIF2, playback DP3, capture DP4, ops &rt700_ops */
];

/* Bus clock frequency */
const RT700_CLK_FREQ_9600000HZ: c_uint = 9600000;
const RT700_CLK_FREQ_12000000HZ: c_uint = 12000000;
const RT700_CLK_FREQ_6000000HZ: c_uint = 6000000;
const RT700_CLK_FREQ_4800000HZ: c_uint = 4800000;
const RT700_CLK_FREQ_2400000HZ: c_uint = 2400000;
const RT700_CLK_FREQ_12288000HZ: c_uint = 12288000;

#[no_mangle]
pub unsafe extern "C" fn rt700_clock_config(dev: *mut device) -> c_int {
    let rt700 = dev_get_drvdata(dev);
    let clk_freq = (*rt700).params.curr_dr_freq >> 1;
    let value: c_uint = match clk_freq {
        RT700_CLK_FREQ_12000000HZ => 0x0,
        RT700_CLK_FREQ_6000000HZ => 0x1,
        RT700_CLK_FREQ_9600000HZ => 0x2,
        RT700_CLK_FREQ_4800000HZ => 0x3,
        RT700_CLK_FREQ_2400000HZ => 0x4,
        RT700_CLK_FREQ_12288000HZ => 0x5,
        _ => return -EINVAL,
    };
    regmap_write((*rt700).regmap, 0xe0, value);
    regmap_write((*rt700).regmap, 0xf0, value);
    0
}

#[no_mangle]
pub unsafe extern "C" fn rt700_init(dev: *mut device, sdw_regmap: *mut regmap, regmap_p: *mut regmap, slave: *mut sdw_slave) -> c_int {
    let rt700 = devm_kzalloc(dev, core::mem::size_of::<rt700_priv>(), GFP_KERNEL) as *mut rt700_priv;
    if rt700.is_null() { return -ENOMEM; }

    dev_set_drvdata(dev, rt700);
    (*rt700).slave = slave;
    (*rt700).sdw_regmap = sdw_regmap;
    (*rt700).regmap = regmap_p;

    regcache_cache_only((*rt700).regmap, true);
    mutex_init(&mut (*rt700).disable_irq_lock);

    // INIT_DELAYED_WORK(&rt700->jack_detect_work, rt700_jack_detect_handler);
    // INIT_DELAYED_WORK(&rt700->jack_btn_check_work, rt700_btn_check_handler);

    /*
     * Mark hw_init to false
     * HW init will be performed when device reports present
     */
    (*rt700).hw_init = false;
    (*rt700).first_hw_init = false;

    let ret = devm_snd_soc_register_component(dev, &soc_codec_dev_rt700, rt700_dai.as_mut_ptr(), rt700_dai.len() as c_int);
    if ret < 0 { return ret; }

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
    0
}

#[no_mangle]
pub unsafe extern "C" fn rt700_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int {
    let rt700 = dev_get_drvdata(dev);
    (*rt700).disable_irq = false;

    if (*rt700).hw_init { return 0; }

    regcache_cache_only((*rt700).regmap, false);
    if (*rt700).first_hw_init { regcache_cache_bypass((*rt700).regmap, true); }

    /*
     * PM runtime is only enabled when a Slave reports as Attached
     */
    if !(*rt700).first_hw_init {
        /* PM runtime status is marked as 'active' only when a Slave reports as Attached */
        pm_runtime_set_active(&mut (*slave).dev);
    }
    pm_runtime_get_noresume(&mut (*slave).dev);

    /* reset */
    regmap_write((*rt700).regmap, 0xff01, 0x0000);
    regmap_write((*rt700).regmap, 0x7520, 0x001a);
    regmap_write((*rt700).regmap, 0x7420, 0xc003);

    /* power on */
    regmap_write((*rt700).regmap, RT700_SET_AUDIO_POWER_STATE, AC_PWRST_D0);
    /* Set Pin Widget */
    regmap_write((*rt700).regmap, RT700_SET_PIN_HP, 0x40);
    regmap_write((*rt700).regmap, RT700_SET_PIN_SPK, 0x40);
    regmap_write((*rt700).regmap, RT700_SET_EAPD_SPK, RT700_EAPD_HIGH);
    regmap_write((*rt700).regmap, RT700_SET_PIN_DMIC1, 0x20);
    regmap_write((*rt700).regmap, RT700_SET_PIN_DMIC2, 0x20);
    regmap_write((*rt700).regmap, RT700_SET_PIN_MIC2, 0x20);

    /* Set Configuration Default */
    regmap_write((*rt700).regmap, 0x4f12, 0x91);
    regmap_write((*rt700).regmap, 0x4e12, 0xd6);
    regmap_write((*rt700).regmap, 0x4d12, 0x11);
    regmap_write((*rt700).regmap, 0x4c12, 0x20);
    regmap_write((*rt700).regmap, 0x4f13, 0x91);
    regmap_write((*rt700).regmap, 0x4e13, 0xd6);
    regmap_write((*rt700).regmap, 0x4d13, 0x11);
    regmap_write((*rt700).regmap, 0x4c13, 0x21);

    regmap_write((*rt700).regmap, 0x4f19, 0x02);
    regmap_write((*rt700).regmap, 0x4e19, 0xa1);
    regmap_write((*rt700).regmap, 0x4d19, 0x90);
    regmap_write((*rt700).regmap, 0x4c19, 0x80);

    /* Enable Line2 */
    regmap_write((*rt700).regmap, 0x371b, 0x40);
    regmap_write((*rt700).regmap, 0x731b, 0xb0);
    regmap_write((*rt700).regmap, 0x839b, 0x00);

    /* Set index */
    rt700_index_write((*rt700).regmap, 0x4a, 0x201b);
    rt700_index_write((*rt700).regmap, 0x45, 0x5089);
    rt700_index_write((*rt700).regmap, 0x6b, 0x5064);
    rt700_index_write((*rt700).regmap, 0x48, 0xd249);

    /* Finish Initial Settings, set power to D3 */
    regmap_write((*rt700).regmap, RT700_SET_AUDIO_POWER_STATE, AC_PWRST_D3);

    /*
     * if set_jack callback occurred early than io_init,
     * we set up the jack detection function now
     */
    if !(*rt700).hs_jack.is_null() { rt700_jack_init(rt700); }

    if (*rt700).first_hw_init {
        regcache_cache_bypass((*rt700).regmap, false);
        regcache_mark_dirty((*rt700).regmap);
    } else {
        (*rt700).first_hw_init = true;
    }

    /* Mark Slave initialization complete */
    (*rt700).hw_init = true;

    pm_runtime_put_autosuspend(&mut (*slave).dev);
    0
}

unsafe fn container_of_rt700_priv_jack_detect_work(_work: *mut work_struct) -> *mut rt700_priv {
    // container_of(work, struct rt700_priv, jack_detect_work.work)
    unimplemented!()
}

unsafe fn container_of_rt700_priv_jack_btn_check_work(_work: *mut work_struct) -> *mut rt700_priv {
    // container_of(work, struct rt700_priv, jack_btn_check_work.work)
    unimplemented!()
}

// MODULE_DESCRIPTION("ASoC RT700 driver SDW");
// MODULE_AUTHOR("Shuming Fan <shumingf@realtek.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
