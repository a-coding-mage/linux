// SPDX-License-Identifier: GPL-2.0
/*
 * rt715.c -- rt715 ALSA SoC audio driver
 *
 * Copyright(c) 2019 Realtek Semiconductor Corp.
 *
 * ALC715 ASoC Codec Driver based Intel Dummy SdW codec driver
 *
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Translated from Linux kernel C. Header-provided types, constants, macros, and
// helper functions are referenced as external dependencies of this translation.

type bool_t = bool;

#[repr(C)]
pub struct regmap {
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
    pub enumerated: snd_ctl_elem_value_enumerated,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_int; 128],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}
#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}
#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_integer {
    pub min: c_int,
    pub max: c_int,
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
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
    pub reg: c_uint,
    pub shift_l: c_uint,
    pub items: c_uint,
}
#[repr(C)]
pub struct rt715_priv {
    pub regmap: *mut regmap,
    pub sdw_regmap: *mut regmap,
    pub slave: *mut sdw_slave,
    pub params: sdw_slave_prop,
    pub hw_init: bool_t,
    pub first_hw_init: bool_t,
    pub kctl_2ch_vol_ori: [c_int; 2],
    pub kctl_8ch_switch_ori: [c_int; 8],
    pub kctl_8ch_vol_ori: [c_int; 8],
}
#[repr(C)]
pub struct sdw_slave_prop {
    pub curr_dr_freq: c_uint,
}
#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
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
    pub set_bias_level:
        Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub endianness: c_uint,
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_OFF = 0,
    SND_SOC_BIAS_STANDBY = 1,
    SND_SOC_BIAS_PREPARE = 2,
    SND_SOC_BIAS_ON = 3,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub id: c_int,
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
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
#[derive(Copy, Clone)]
pub struct sdw_stream_config {
    _zero: c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_port_config {
    pub num: c_uint,
}
#[repr(C)]
pub struct sdw_stream_runtime {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub set_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, *mut c_void, c_int) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
}

unsafe extern "C" {
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut rt715_priv;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_soc_dapm_kcontrol_to_component(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dapm_kcontrol_to_dapm(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dapm_context;
    fn snd_soc_enum_item_to_val(e: *mut soc_enum, item: c_uint) -> c_uint;
    fn snd_soc_dapm_mux_update_power(
        dapm: *mut snd_soc_dapm_context,
        kcontrol: *mut snd_kcontrol,
        mux: c_uint,
        e: *mut soc_enum,
        update: *mut c_void,
    ) -> c_int;
    fn pm_runtime_resume(dev: *mut device) -> c_int;
    fn snd_soc_dai_dma_data_set(dai: *mut snd_soc_dai, direction: c_int, data: *mut c_void);
    fn snd_soc_dai_set_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
        data: *mut c_void,
    );
    fn snd_soc_dai_get_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
    ) -> *mut sdw_stream_runtime;
    fn snd_sdw_params_to_config(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        stream: *mut sdw_stream_config,
        port: *mut sdw_port_config,
    );
    fn sdw_stream_add_slave(
        slave: *mut sdw_slave,
        stream: *mut sdw_stream_config,
        port: *mut sdw_port_config,
        num_ports: c_uint,
        sdw_stream: *mut sdw_stream_runtime,
    ) -> c_int;
    fn sdw_stream_remove_slave(slave: *mut sdw_slave, sdw_stream: *mut sdw_stream_runtime);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_get_drvdata(dev: *mut device) -> *mut rt715_priv;
    fn dev_set_drvdata(dev: *mut device, data: *mut rt715_priv);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn regcache_cache_only(map: *mut regmap, enable: bool_t);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn pm_runtime_put_autosuspend(dev: *mut device);
    fn regcache_mark_dirty(map: *mut regmap);
    fn msleep(ms: c_uint);
}

// Header constants from rt715.h and Linux headers.
unsafe extern "C" {
    static RT715_PRIV_INDEX_W_H: c_uint;
    static RT715_PRIV_INDEX_W_H_2: c_uint;
    static RT715_VENDOR_REGISTERS: c_uint;
    static RT715_VD_CLEAR_CTRL: c_uint;
    static RT715_CLEAR_HIDDEN_REG: c_uint;
    static RT715_FUNC_RESET: c_uint;
    static RT715_DIR_OUT_SFT: c_uint;
    static RT715_DIR_IN_SFT: c_uint;
    static RT715_SET_AUDIO_POWER_STATE: c_uint;
    static AC_PWRST_D0: c_uint;
    static AC_PWRST_D3: c_uint;
    static RT715_SET_GAIN_MIC_ADC_H: c_uint;
    static RT715_SET_GAIN_LINE_ADC_H: c_uint;
    static RT715_SET_GAIN_MIX_ADC_H: c_uint;
    static RT715_SET_GAIN_MIX_ADC2_H: c_uint;
    static RT715_SET_GAIN_MIC_ADC_L: c_uint;
    static RT715_SET_GAIN_LINE_ADC_L: c_uint;
    static RT715_SET_GAIN_MIX_ADC_L: c_uint;
    static RT715_SET_GAIN_MIX_ADC2_L: c_uint;
    static RT715_SET_GAIN_DMIC1_H: c_uint;
    static RT715_SET_GAIN_DMIC1_L: c_uint;
    static RT715_SET_GAIN_DMIC2_H: c_uint;
    static RT715_SET_GAIN_DMIC2_L: c_uint;
    static RT715_SET_GAIN_DMIC3_H: c_uint;
    static RT715_SET_GAIN_DMIC3_L: c_uint;
    static RT715_SET_GAIN_DMIC4_H: c_uint;
    static RT715_SET_GAIN_DMIC4_L: c_uint;
    static RT715_SET_GAIN_MIC1_H: c_uint;
    static RT715_SET_GAIN_MIC1_L: c_uint;
    static RT715_SET_GAIN_MIC2_H: c_uint;
    static RT715_SET_GAIN_MIC2_L: c_uint;
    static RT715_SET_GAIN_LINE1_H: c_uint;
    static RT715_SET_GAIN_LINE1_L: c_uint;
    static RT715_SET_GAIN_LINE2_H: c_uint;
    static RT715_SET_GAIN_LINE2_L: c_uint;
    static RT715_VERB_SET_CONNECT_SEL: c_uint;
    static RT715_MUX_IN1: c_uint;
    static RT715_MUX_IN2: c_uint;
    static RT715_MUX_IN3: c_uint;
    static RT715_MUX_IN4: c_uint;
    static SND_SOC_NOPM: c_uint;
    static RT715_SET_STREAMID_MIC_ADC: c_uint;
    static RT715_SET_STREAMID_LINE_ADC: c_uint;
    static RT715_SET_STREAMID_MIX_ADC: c_uint;
    static RT715_SET_STREAMID_MIX_ADC2: c_uint;
    static RT715_POWER_UP_DELAY_MS: c_uint;
    static EACCES: c_int;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static GFP_KERNEL: c_uint;
    static RT715_AIF1: c_int;
    static RT715_AIF2: c_int;
    static RT715_SDW_INPUT_SEL: c_uint;
    static RT715_MIC_ADC_FORMAT_H: c_uint;
    static RT715_MIC_LINE_FORMAT_H: c_uint;
    static RT715_MIX_ADC_FORMAT_H: c_uint;
    static RT715_MIX_ADC2_FORMAT_H: c_uint;
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S8: c_uint;
    static RT715_SET_PIN_DMIC1: c_uint;
    static RT715_SET_PIN_DMIC2: c_uint;
    static RT715_SET_PIN_DMIC3: c_uint;
    static RT715_SET_PIN_DMIC4: c_uint;
    static RT715_SET_DMIC1_CONFIG_DEFAULT1: c_uint;
    static RT715_SET_DMIC1_CONFIG_DEFAULT2: c_uint;
    static RT715_SET_DMIC1_CONFIG_DEFAULT3: c_uint;
    static RT715_SET_DMIC1_CONFIG_DEFAULT4: c_uint;
    static RT715_SET_DMIC2_CONFIG_DEFAULT1: c_uint;
    static RT715_SET_DMIC2_CONFIG_DEFAULT2: c_uint;
    static RT715_SET_DMIC2_CONFIG_DEFAULT3: c_uint;
    static RT715_SET_DMIC2_CONFIG_DEFAULT4: c_uint;
    static RT715_SET_DMIC3_CONFIG_DEFAULT1: c_uint;
    static RT715_SET_DMIC3_CONFIG_DEFAULT2: c_uint;
    static RT715_SET_DMIC3_CONFIG_DEFAULT3: c_uint;
    static RT715_SET_DMIC3_CONFIG_DEFAULT4: c_uint;
    static RT715_SET_DMIC4_CONFIG_DEFAULT1: c_uint;
    static RT715_SET_DMIC4_CONFIG_DEFAULT2: c_uint;
    static RT715_SET_DMIC4_CONFIG_DEFAULT3: c_uint;
    static RT715_SET_DMIC4_CONFIG_DEFAULT4: c_uint;
    static SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_uint;
    static SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint;
}

unsafe fn rt715_index_write(regmap: *mut regmap, reg: c_uint, value: c_uint) -> c_int {
    let addr: c_uint = ((RT715_PRIV_INDEX_W_H) << 8) | reg;

    let ret = regmap_write(regmap, addr, value);
    if ret < 0 {
        pr_err(
            c"%s: Failed to set private value: %08x <= %04x %d\n".as_ptr(),
            c"rt715_index_write".as_ptr(),
            addr,
            value,
            ret,
        );
    }

    ret
}

unsafe fn rt715_index_write_nid(
    regmap: *mut regmap,
    nid: c_uint,
    reg: c_uint,
    value: c_uint,
) -> c_int {
    let addr: c_uint = ((RT715_PRIV_INDEX_W_H_2 | nid) << 8) | reg;

    let ret = regmap_write(regmap, addr, value);
    if ret < 0 {
        pr_err(
            c"%s: Failed to set private value: %06x <= %04x ret=%d\n".as_ptr(),
            c"rt715_index_write_nid".as_ptr(),
            addr,
            value,
            ret,
        );
    }

    ret
}

unsafe fn rt715_index_read_nid(
    regmap: *mut regmap,
    nid: c_uint,
    reg: c_uint,
    value: *mut c_uint,
) -> c_int {
    let addr: c_uint = ((RT715_PRIV_INDEX_W_H_2 | nid) << 8) | reg;

    *value = 0;
    let ret = regmap_read(regmap, addr, value);
    if ret < 0 {
        pr_err(
            c"%s: Failed to get private value: %06x => %04x ret=%d\n".as_ptr(),
            c"rt715_index_read_nid".as_ptr(),
            addr,
            *value,
            ret,
        );
    }

    ret
}

unsafe fn rt715_index_update_bits(
    regmap: *mut regmap,
    nid: c_uint,
    reg: c_uint,
    mask: c_uint,
    val: c_uint,
) -> c_int {
    let mut tmp: c_uint;
    let mut orig: c_uint = 0;

    let ret = rt715_index_read_nid(regmap, nid, reg, &mut orig);
    if ret < 0 {
        return ret;
    }

    tmp = orig & !mask;
    tmp |= val & mask;

    rt715_index_write_nid(regmap, nid, reg, tmp)
}

unsafe fn rt715_reset(regmap: *mut regmap) {
    regmap_write(regmap, RT715_FUNC_RESET, 0);
    rt715_index_update_bits(
        regmap,
        RT715_VENDOR_REGISTERS,
        RT715_VD_CLEAR_CTRL,
        RT715_CLEAR_HIDDEN_REG,
        RT715_CLEAR_HIDDEN_REG,
    );
}

unsafe fn rt715_get_gain(
    rt715: *mut rt715_priv,
    addr_h: c_uint,
    addr_l: c_uint,
    mut val_h: c_uint,
    r_val: *mut c_uint,
    l_val: *mut c_uint,
) {
    /* R Channel */
    *r_val = val_h << 8;
    let mut ret = regmap_read((*rt715).regmap, addr_l, r_val);
    if ret < 0 {
        pr_err(c"Failed to get R channel gain.\n".as_ptr());
    }

    /* L Channel */
    val_h |= 0x20;
    *l_val = val_h << 8;
    ret = regmap_read((*rt715).regmap, addr_h, l_val);
    if ret < 0 {
        pr_err(c"Failed to get L channel gain.\n".as_ptr());
    }
}

/* For Verb-Set Amplifier Gain (Verb ID = 3h) */
unsafe fn rt715_set_amp_gain_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let dapm = snd_soc_component_to_dapm(component);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let rt715 = snd_soc_component_get_drvdata(component);
    let addr_h: c_uint;
    let addr_l: c_uint;
    let mut val_h: c_uint;
    let mut val_ll: c_uint;
    let mut val_lr: c_uint;
    let mut read_ll: c_uint = 0;
    let mut read_rl: c_uint = 0;
    let mut k_vol_changed: c_uint = 0;

    for i in 0..2usize {
        if (*ucontrol).value.integer.value[i] != (*rt715).kctl_2ch_vol_ori[i] {
            k_vol_changed = 1;
            break;
        }
    }

    /* Can't use update bit function, so read the original value first */
    addr_h = (*mc).reg;
    addr_l = (*mc).rreg;

    if (*mc).shift == RT715_DIR_OUT_SFT {
        /* output */
        val_h = 0x80;
    } else {
        /* input */
        val_h = 0x0;
    }

    rt715_get_gain(rt715, addr_h, addr_l, val_h, &mut read_rl, &mut read_ll);

    if snd_soc_dapm_get_bias_level(dapm) as c_int <= snd_soc_bias_level::SND_SOC_BIAS_STANDBY as c_int
    {
        regmap_write((*rt715).regmap, RT715_SET_AUDIO_POWER_STATE, AC_PWRST_D0);
    }

    /* L Channel */
    (*rt715).kctl_2ch_vol_ori[0] = (*ucontrol).value.integer.value[0];
    /* for gain */
    val_ll = ((*ucontrol).value.integer.value[0] as c_uint) & 0x7f;
    if val_ll > (*mc).max {
        val_ll = (*mc).max;
    }
    /* keep mute status */
    val_ll |= read_ll & 0x80;

    /* R Channel */
    (*rt715).kctl_2ch_vol_ori[1] = (*ucontrol).value.integer.value[1];
    /* for gain */
    val_lr = ((*ucontrol).value.integer.value[1] as c_uint) & 0x7f;
    if val_lr > (*mc).max {
        val_lr = (*mc).max;
    }
    /* keep mute status */
    val_lr |= read_rl & 0x80;

    for _i in 0..3 {
        /* retry 3 times at most */
        if val_ll == val_lr {
            /* Set both L/R channels at the same time */
            val_h = (1 << (*mc).shift) | (3 << 4);
            regmap_write((*rt715).regmap, addr_h, (val_h << 8) | val_ll);
            regmap_write((*rt715).regmap, addr_l, (val_h << 8) | val_ll);
        } else {
            /* Lch*/
            val_h = (1 << (*mc).shift) | (1 << 5);
            regmap_write((*rt715).regmap, addr_h, (val_h << 8) | val_ll);
            /* Rch */
            val_h = (1 << (*mc).shift) | (1 << 4);
            regmap_write((*rt715).regmap, addr_l, (val_h << 8) | val_lr);
        }
        /* check result */
        if (*mc).shift == RT715_DIR_OUT_SFT {
            /* output */
            val_h = 0x80;
        } else {
            /* input */
            val_h = 0x0;
        }

        rt715_get_gain(rt715, addr_h, addr_l, val_h, &mut read_rl, &mut read_ll);
        if read_rl == val_lr && read_ll == val_ll {
            break;
        }
    }

    /* D0:power on state, D3: power saving mode */
    if snd_soc_dapm_get_bias_level(dapm) as c_int <= snd_soc_bias_level::SND_SOC_BIAS_STANDBY as c_int
    {
        regmap_write((*rt715).regmap, RT715_SET_AUDIO_POWER_STATE, AC_PWRST_D3);
    }
    k_vol_changed as c_int
}

unsafe fn rt715_set_amp_gain_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt715 = snd_soc_component_get_drvdata(component);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let addr_h: c_uint;
    let addr_l: c_uint;
    let val_h: c_uint;
    let mut read_ll: c_uint = 0;
    let mut read_rl: c_uint = 0;

    addr_h = (*mc).reg;
    addr_l = (*mc).rreg;
    if (*mc).shift == RT715_DIR_OUT_SFT {
        /* output */
        val_h = 0x80;
    } else {
        /* input */
        val_h = 0x0;
    }

    rt715_get_gain(rt715, addr_h, addr_l, val_h, &mut read_rl, &mut read_ll);

    if (*mc).invert != 0 {
        /* for mute status */
        read_ll = if (read_ll & 0x80) == 0 { 1 } else { 0 };
        read_rl = if (read_rl & 0x80) == 0 { 1 } else { 0 };
    } else {
        /* for gain */
        read_ll &= 0x7f;
        read_rl &= 0x7f;
    }
    (*ucontrol).value.integer.value[0] = read_ll as c_int;
    (*ucontrol).value.integer.value[1] = read_rl as c_int;

    0
}

unsafe fn rt715_set_main_switch_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let dapm = snd_soc_component_to_dapm(component);
    let rt715 = snd_soc_component_get_drvdata(component);
    let capture_reg_h: [c_uint; 4] = [
        RT715_SET_GAIN_MIC_ADC_H,
        RT715_SET_GAIN_LINE_ADC_H,
        RT715_SET_GAIN_MIX_ADC_H,
        RT715_SET_GAIN_MIX_ADC2_H,
    ];
    let capture_reg_l: [c_uint; 4] = [
        RT715_SET_GAIN_MIC_ADC_L,
        RT715_SET_GAIN_LINE_ADC_L,
        RT715_SET_GAIN_MIX_ADC_L,
        RT715_SET_GAIN_MIX_ADC2_L,
    ];
    let mut val_h: c_uint = 0x0;
    let mut val_ll: c_uint;
    let mut val_lr: c_uint;
    let k_shift: c_uint = RT715_DIR_IN_SFT;
    let mut k_changed: c_uint = 0;
    let mut read_ll: c_uint = 0;
    let mut read_rl: c_uint = 0;
    let loop_cnt: usize = 4;

    for i in 0..8usize {
        if (*ucontrol).value.integer.value[i] != (*rt715).kctl_8ch_switch_ori[i] {
            k_changed = 1;
        }
    }

    for j in 0..loop_cnt {
        /* Can't use update bit function, so read the original value first */
        let addr_h = capture_reg_h[j];
        let addr_l = capture_reg_l[j];
        rt715_get_gain(rt715, addr_h, addr_l, val_h, &mut read_rl, &mut read_ll);

        if snd_soc_dapm_get_bias_level(dapm) as c_int <= snd_soc_bias_level::SND_SOC_BIAS_STANDBY as c_int
        {
            regmap_write((*rt715).regmap, RT715_SET_AUDIO_POWER_STATE, AC_PWRST_D0);
        }

        /* L Channel */
        /* for mute */
        (*rt715).kctl_8ch_switch_ori[j * 2] = (*ucontrol).value.integer.value[j * 2];
        val_ll = ((if (*ucontrol).value.integer.value[j * 2] == 0 { 1 } else { 0 }) as c_uint) << 7;
        /* keep gain */
        val_ll |= read_ll & 0x7f;

        /* R Channel */
        /* for mute */
        (*rt715).kctl_8ch_switch_ori[j * 2 + 1] =
            (*ucontrol).value.integer.value[j * 2 + 1];
        val_lr =
            ((if (*ucontrol).value.integer.value[j * 2 + 1] == 0 { 1 } else { 0 }) as c_uint)
                << 7;
        /* keep gain */
        val_lr |= read_rl & 0x7f;

        for _i in 0..3 {
            /* retry 3 times at most */
            if val_ll == val_lr {
                /* Set both L/R channels at the same time */
                val_h = (1 << k_shift) | (3 << 4);
                regmap_write((*rt715).regmap, addr_h, (val_h << 8) | val_ll);
                regmap_write((*rt715).regmap, addr_l, (val_h << 8) | val_ll);
            } else {
                /* Lch*/
                val_h = (1 << k_shift) | (1 << 5);
                regmap_write((*rt715).regmap, addr_h, (val_h << 8) | val_ll);
                /* Rch */
                val_h = (1 << k_shift) | (1 << 4);
                regmap_write((*rt715).regmap, addr_l, (val_h << 8) | val_lr);
            }
            val_h = 0x0;
            rt715_get_gain(rt715, addr_h, addr_l, val_h, &mut read_rl, &mut read_ll);
            if read_rl == val_lr && read_ll == val_ll {
                break;
            }
        }
    }

    /* D0:power on state, D3: power saving mode */
    if snd_soc_dapm_get_bias_level(dapm) as c_int <= snd_soc_bias_level::SND_SOC_BIAS_STANDBY as c_int
    {
        regmap_write((*rt715).regmap, RT715_SET_AUDIO_POWER_STATE, AC_PWRST_D3);
    }
    k_changed as c_int
}

unsafe fn rt715_set_main_switch_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt715 = snd_soc_component_get_drvdata(component);
    let capture_reg_h: [c_uint; 4] = [
        RT715_SET_GAIN_MIC_ADC_H,
        RT715_SET_GAIN_LINE_ADC_H,
        RT715_SET_GAIN_MIX_ADC_H,
        RT715_SET_GAIN_MIX_ADC2_H,
    ];
    let capture_reg_l: [c_uint; 4] = [
        RT715_SET_GAIN_MIC_ADC_L,
        RT715_SET_GAIN_LINE_ADC_L,
        RT715_SET_GAIN_MIX_ADC_L,
        RT715_SET_GAIN_MIX_ADC2_L,
    ];
    let val_h: c_uint = 0x0;
    let loop_cnt: usize = 4;
    let mut read_ll: c_uint = 0;
    let mut read_rl: c_uint = 0;

    for i in 0..loop_cnt {
        let addr_h = capture_reg_h[i];
        let addr_l = capture_reg_l[i];
        rt715_get_gain(rt715, addr_h, addr_l, val_h, &mut read_rl, &mut read_ll);

        (*ucontrol).value.integer.value[i * 2] = if (read_ll & 0x80) == 0 { 1 } else { 0 };
        (*ucontrol).value.integer.value[i * 2 + 1] =
            if (read_rl & 0x80) == 0 { 1 } else { 0 };
    }

    0
}

unsafe fn rt715_set_main_vol_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let dapm = snd_soc_component_to_dapm(component);
    let rt715 = snd_soc_component_get_drvdata(component);
    let capture_reg_h: [c_uint; 4] = [
        RT715_SET_GAIN_MIC_ADC_H,
        RT715_SET_GAIN_LINE_ADC_H,
        RT715_SET_GAIN_MIX_ADC_H,
        RT715_SET_GAIN_MIX_ADC2_H,
    ];
    let capture_reg_l: [c_uint; 4] = [
        RT715_SET_GAIN_MIC_ADC_L,
        RT715_SET_GAIN_LINE_ADC_L,
        RT715_SET_GAIN_MIX_ADC_L,
        RT715_SET_GAIN_MIX_ADC2_L,
    ];
    let mut val_h: c_uint = 0x0;
    let mut val_ll: c_uint;
    let mut val_lr: c_uint;
    let mut read_ll: c_uint = 0;
    let mut read_rl: c_uint = 0;
    let loop_cnt: usize = 4;
    let mut k_changed: c_uint = 0;
    let k_shift: c_uint = RT715_DIR_IN_SFT;
    let k_max: c_uint = 0x3f;

    for i in 0..8usize {
        if (*ucontrol).value.integer.value[i] != (*rt715).kctl_8ch_vol_ori[i] {
            k_changed = 1;
        }
    }

    for j in 0..loop_cnt {
        let addr_h = capture_reg_h[j];
        let addr_l = capture_reg_l[j];
        rt715_get_gain(rt715, addr_h, addr_l, val_h, &mut read_rl, &mut read_ll);

        if snd_soc_dapm_get_bias_level(dapm) as c_int <= snd_soc_bias_level::SND_SOC_BIAS_STANDBY as c_int
        {
            regmap_write((*rt715).regmap, RT715_SET_AUDIO_POWER_STATE, AC_PWRST_D0);
        }

        /* L Channel */
        /* for gain */
        (*rt715).kctl_8ch_vol_ori[j * 2] = (*ucontrol).value.integer.value[j * 2];
        val_ll = ((*ucontrol).value.integer.value[j * 2] as c_uint) & 0x7f;
        if val_ll > k_max {
            val_ll = k_max;
        }
        /* keep mute status */
        val_ll |= read_ll & 0x80;

        /* R Channel */
        /* for gain */
        (*rt715).kctl_8ch_vol_ori[j * 2 + 1] =
            (*ucontrol).value.integer.value[j * 2 + 1];
        val_lr = ((*ucontrol).value.integer.value[j * 2 + 1] as c_uint) & 0x7f;
        if val_lr > k_max {
            val_lr = k_max;
        }
        /* keep mute status */
        val_lr |= read_rl & 0x80;

        for _i in 0..3 {
            /* retry 3 times at most */
            if val_ll == val_lr {
                /* Set both L/R channels at the same time */
                val_h = (1 << k_shift) | (3 << 4);
                regmap_write((*rt715).regmap, addr_h, (val_h << 8) | val_ll);
                regmap_write((*rt715).regmap, addr_l, (val_h << 8) | val_ll);
            } else {
                /* Lch*/
                val_h = (1 << k_shift) | (1 << 5);
                regmap_write((*rt715).regmap, addr_h, (val_h << 8) | val_ll);
                /* Rch */
                val_h = (1 << k_shift) | (1 << 4);
                regmap_write((*rt715).regmap, addr_l, (val_h << 8) | val_lr);
            }
            val_h = 0x0;
            rt715_get_gain(rt715, addr_h, addr_l, val_h, &mut read_rl, &mut read_ll);
            if read_rl == val_lr && read_ll == val_ll {
                break;
            }
        }
    }

    /* D0:power on state, D3: power saving mode */
    if snd_soc_dapm_get_bias_level(dapm) as c_int <= snd_soc_bias_level::SND_SOC_BIAS_STANDBY as c_int
    {
        regmap_write((*rt715).regmap, RT715_SET_AUDIO_POWER_STATE, AC_PWRST_D3);
    }
    k_changed as c_int
}

unsafe fn rt715_set_main_vol_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt715 = snd_soc_component_get_drvdata(component);
    let capture_reg_h: [c_uint; 4] = [
        RT715_SET_GAIN_MIC_ADC_H,
        RT715_SET_GAIN_LINE_ADC_H,
        RT715_SET_GAIN_MIX_ADC_H,
        RT715_SET_GAIN_MIX_ADC2_H,
    ];
    let capture_reg_l: [c_uint; 4] = [
        RT715_SET_GAIN_MIC_ADC_L,
        RT715_SET_GAIN_LINE_ADC_L,
        RT715_SET_GAIN_MIX_ADC_L,
        RT715_SET_GAIN_MIX_ADC2_L,
    ];
    let val_h: c_uint = 0x0;
    let loop_cnt: usize = 4;
    let mut read_ll: c_uint = 0;
    let mut read_rl: c_uint = 0;

    for i in 0..loop_cnt {
        let addr_h = capture_reg_h[i];
        let addr_l = capture_reg_l[i];
        rt715_get_gain(rt715, addr_h, addr_l, val_h, &mut read_rl, &mut read_ll);

        (*ucontrol).value.integer.value[i * 2] = (read_ll & 0x7f) as c_int;
        (*ucontrol).value.integer.value[i * 2 + 1] = (read_rl & 0x7f) as c_int;
    }

    0
}

/* static const DECLARE_TLV_DB_SCALE(in_vol_tlv, -1725, 75, 0); */
static in_vol_tlv: [c_uint; 4] = [0, (-1725i32) as c_uint, 75, 0];
/* static const DECLARE_TLV_DB_SCALE(mic_vol_tlv, 0, 1000, 0); */
static mic_vol_tlv: [c_uint; 4] = [0, 0, 1000, 0];

unsafe fn rt715_switch_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    (*uinfo).count = 8;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1;
    0
}

unsafe fn rt715_vol_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 8;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 0x3f;
    0
}

/*
 * RT715_MAIN_SWITCH_EXT, RT715_MAIN_VOL_EXT_TLV, SOC_DOUBLE_R_EXT_TLV,
 * SOC_ENUM_SINGLE_DECL, SOC_VALUE_ENUM_SINGLE_DECL, SOC_DAPM_ENUM_EXT, and
 * SND_SOC_DAPM_* are C initializer macros supplied by ALSA SoC headers. Their
 * generated static objects are preserved here as declarations to keep the same
 * source-level interfaces without reimplementing Linux macro internals.
 */
unsafe extern "C" {
    static rt715_snd_controls: [snd_kcontrol_new; 10];
    static rt715_adc22_enum: soc_enum;
    static rt715_adc23_enum: soc_enum;
    static rt715_adc24_enum: soc_enum;
    static rt715_adc25_enum: soc_enum;
    static rt715_adc22_mux: snd_kcontrol_new;
    static rt715_adc23_mux: snd_kcontrol_new;
    static rt715_adc24_mux: snd_kcontrol_new;
    static rt715_adc25_mux: snd_kcontrol_new;
    static rt715_dapm_widgets: [snd_soc_dapm_widget; 18];
}

unsafe fn rt715_mux_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let rt715 = snd_soc_component_get_drvdata(component);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let mut val: c_uint = 0;

    /* nid = e->reg, vid = 0xf01 */
    let mut reg = RT715_VERB_SET_CONNECT_SEL | (*e).reg;
    let ret = regmap_read((*rt715).regmap, reg, &mut val);
    if ret < 0 {
        dev_err(
            (*component).dev,
            c"%s: sdw read failed: %d\n".as_ptr(),
            c"rt715_mux_get".as_ptr(),
            ret,
        );
        return ret;
    }

    /*
     * The first two indices of ADC Mux 24/25 are routed to the same
     * hardware source. ie, ADC Mux 24 0/1 will both connect to MIC2.
     * To have a unique set of inputs, we skip the index1 of the muxes.
     */
    if ((*e).reg == RT715_MUX_IN3 || (*e).reg == RT715_MUX_IN4) && val > 0 {
        val -= 1;
    }
    (*ucontrol).value.enumerated.item[0] = val;

    0
}

unsafe fn rt715_mux_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let rt715 = snd_soc_component_get_drvdata(component);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let item = (*ucontrol).value.enumerated.item.as_mut_ptr();
    let mut val2: c_uint = 0;
    let change: c_uint;

    if *item.add(0) >= (*e).items {
        return -EINVAL;
    }

    /* Verb ID = 0x701h, nid = e->reg */
    let val = snd_soc_enum_item_to_val(e, *item.add(0)) << (*e).shift_l;

    let mut reg = RT715_VERB_SET_CONNECT_SEL | (*e).reg;
    let ret = regmap_read((*rt715).regmap, reg, &mut val2);
    if ret < 0 {
        dev_err(
            (*component).dev,
            c"%s: sdw read failed: %d\n".as_ptr(),
            c"rt715_mux_put".as_ptr(),
            ret,
        );
        return ret;
    }

    if val == val2 {
        change = 0;
    } else {
        change = 1;
    }

    if change != 0 {
        reg = RT715_VERB_SET_CONNECT_SEL | (*e).reg;
        regmap_write((*rt715).regmap, reg, val);
    }

    snd_soc_dapm_mux_update_power(dapm, kcontrol, *item.add(0), e, core::ptr::null_mut());

    change as c_int
}

static adc_22_23_mux_text: [*const c_char; 8] = [
    c"MIC1".as_ptr(),
    c"MIC2".as_ptr(),
    c"LINE1".as_ptr(),
    c"LINE2".as_ptr(),
    c"DMIC1".as_ptr(),
    c"DMIC2".as_ptr(),
    c"DMIC3".as_ptr(),
    c"DMIC4".as_ptr(),
];

/*
 * Due to mux design for nid 24 (MUX_IN3)/25 (MUX_IN4), connection index 0 and
 * 1 will be connected to the same dmic source, therefore we skip index 1 to
 * avoid misunderstanding on usage of dapm routing.
 */
static rt715_adc_24_25_values: [c_uint; 5] = [0, 2, 3, 4, 5];

static adc_24_mux_text: [*const c_char; 5] = [
    c"MIC2".as_ptr(),
    c"DMIC1".as_ptr(),
    c"DMIC2".as_ptr(),
    c"DMIC3".as_ptr(),
    c"DMIC4".as_ptr(),
];

static adc_25_mux_text: [*const c_char; 5] = [
    c"MIC1".as_ptr(),
    c"DMIC1".as_ptr(),
    c"DMIC2".as_ptr(),
    c"DMIC3".as_ptr(),
    c"DMIC4".as_ptr(),
];

static rt715_audio_map: [snd_soc_dapm_route; 34] = [
    snd_soc_dapm_route { sink: c"DP6TX".as_ptr(), control: core::ptr::null(), source: c"ADC 09".as_ptr() },
    snd_soc_dapm_route { sink: c"DP6TX".as_ptr(), control: core::ptr::null(), source: c"ADC 08".as_ptr() },
    snd_soc_dapm_route { sink: c"DP4TX".as_ptr(), control: core::ptr::null(), source: c"ADC 07".as_ptr() },
    snd_soc_dapm_route { sink: c"DP4TX".as_ptr(), control: core::ptr::null(), source: c"ADC 27".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 09".as_ptr(), control: core::ptr::null(), source: c"ADC 22 Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 08".as_ptr(), control: core::ptr::null(), source: c"ADC 23 Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 07".as_ptr(), control: core::ptr::null(), source: c"ADC 24 Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 27".as_ptr(), control: core::ptr::null(), source: c"ADC 25 Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 22 Mux".as_ptr(), control: c"MIC1".as_ptr(), source: c"MIC1".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 22 Mux".as_ptr(), control: c"MIC2".as_ptr(), source: c"MIC2".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 22 Mux".as_ptr(), control: c"LINE1".as_ptr(), source: c"LINE1".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 22 Mux".as_ptr(), control: c"LINE2".as_ptr(), source: c"LINE2".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 22 Mux".as_ptr(), control: c"DMIC1".as_ptr(), source: c"DMIC1".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 22 Mux".as_ptr(), control: c"DMIC2".as_ptr(), source: c"DMIC2".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 22 Mux".as_ptr(), control: c"DMIC3".as_ptr(), source: c"DMIC3".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 22 Mux".as_ptr(), control: c"DMIC4".as_ptr(), source: c"DMIC4".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 23 Mux".as_ptr(), control: c"MIC1".as_ptr(), source: c"MIC1".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 23 Mux".as_ptr(), control: c"MIC2".as_ptr(), source: c"MIC2".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 23 Mux".as_ptr(), control: c"LINE1".as_ptr(), source: c"LINE1".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 23 Mux".as_ptr(), control: c"LINE2".as_ptr(), source: c"LINE2".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 23 Mux".as_ptr(), control: c"DMIC1".as_ptr(), source: c"DMIC1".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 23 Mux".as_ptr(), control: c"DMIC2".as_ptr(), source: c"DMIC2".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 23 Mux".as_ptr(), control: c"DMIC3".as_ptr(), source: c"DMIC3".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 23 Mux".as_ptr(), control: c"DMIC4".as_ptr(), source: c"DMIC4".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 24 Mux".as_ptr(), control: c"MIC2".as_ptr(), source: c"MIC2".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 24 Mux".as_ptr(), control: c"DMIC1".as_ptr(), source: c"DMIC1".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 24 Mux".as_ptr(), control: c"DMIC2".as_ptr(), source: c"DMIC2".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 24 Mux".as_ptr(), control: c"DMIC3".as_ptr(), source: c"DMIC3".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 24 Mux".as_ptr(), control: c"DMIC4".as_ptr(), source: c"DMIC4".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 25 Mux".as_ptr(), control: c"MIC1".as_ptr(), source: c"MIC1".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 25 Mux".as_ptr(), control: c"DMIC1".as_ptr(), source: c"DMIC1".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 25 Mux".as_ptr(), control: c"DMIC2".as_ptr(), source: c"DMIC2".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 25 Mux".as_ptr(), control: c"DMIC3".as_ptr(), source: c"DMIC3".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC 25 Mux".as_ptr(), control: c"DMIC4".as_ptr(), source: c"DMIC4".as_ptr() },
];

unsafe fn rt715_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let rt715 = snd_soc_component_get_drvdata(component);

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {
            if snd_soc_dapm_get_bias_level(dapm) == snd_soc_bias_level::SND_SOC_BIAS_STANDBY {
                regmap_write((*rt715).regmap, RT715_SET_AUDIO_POWER_STATE, AC_PWRST_D0);
                msleep(RT715_POWER_UP_DELAY_MS);
            }
        }
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            regmap_write((*rt715).regmap, RT715_SET_AUDIO_POWER_STATE, AC_PWRST_D3);
        }
        _ => {}
    }

    0
}

unsafe fn rt715_probe(component: *mut snd_soc_component) -> c_int {
    let rt715 = snd_soc_component_get_drvdata(component);

    if !(*rt715).first_hw_init {
        return 0;
    }

    let ret = pm_runtime_resume((*component).dev);
    if ret < 0 && ret != -EACCES {
        return ret;
    }

    0
}

static soc_codec_dev_rt715: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rt715_probe),
    set_bias_level: Some(rt715_set_bias_level),
    controls: unsafe { rt715_snd_controls.as_ptr() },
    num_controls: 10,
    dapm_widgets: unsafe { rt715_dapm_widgets.as_ptr() },
    num_dapm_widgets: 18,
    dapm_routes: rt715_audio_map.as_ptr(),
    num_dapm_routes: 34,
    endianness: 1,
};

unsafe fn rt715_set_sdw_stream(
    dai: *mut snd_soc_dai,
    sdw_stream: *mut c_void,
    direction: c_int,
) -> c_int {
    snd_soc_dai_dma_data_set(dai, direction, sdw_stream);

    0
}

unsafe fn rt715_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    snd_soc_dai_set_dma_data(dai, substream, core::ptr::null_mut());
}

unsafe fn rt715_pcm_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let rt715 = snd_soc_component_get_drvdata(component);
    let mut stream_config = sdw_stream_config { _zero: 0 };
    let mut port_config = sdw_port_config { num: 0 };
    let sdw_stream: *mut sdw_stream_runtime;
    let retval: c_int;
    let mut val: c_uint = 0;

    sdw_stream = snd_soc_dai_get_dma_data(dai, substream);

    if sdw_stream.is_null() {
        return -EINVAL;
    }

    if (*rt715).slave.is_null() {
        return -EINVAL;
    }

    snd_sdw_params_to_config(substream, params, &mut stream_config, &mut port_config);

    match (*dai).id {
        id if id == RT715_AIF1 => {
            port_config.num = 6;
            rt715_index_write((*rt715).regmap, RT715_SDW_INPUT_SEL, 0xa500);
        }
        id if id == RT715_AIF2 => {
            port_config.num = 4;
            rt715_index_write((*rt715).regmap, RT715_SDW_INPUT_SEL, 0xa000);
        }
        _ => {
            dev_err(
                (*component).dev,
                c"%s: Invalid DAI id %d\n".as_ptr(),
                c"rt715_pcm_hw_params".as_ptr(),
                (*dai).id,
            );
            return -EINVAL;
        }
    }

    retval = sdw_stream_add_slave((*rt715).slave, &mut stream_config, &mut port_config, 1, sdw_stream);
    if retval != 0 {
        dev_err(
            (*dai).dev,
            c"%s: Unable to configure port\n".as_ptr(),
            c"rt715_pcm_hw_params".as_ptr(),
        );
        return retval;
    }

    match params_rate(params) {
        /* bit 14 0:48K 1:44.1K */
        /* bit 15 Stream Type 0:PCM 1:Non-PCM, should always be PCM */
        44100 => {
            val |= 0x40 << 8;
        }
        48000 => {
            val |= 0x0 << 8;
        }
        _ => {
            dev_err(
                (*component).dev,
                c"%s: Unsupported sample rate %d\n".as_ptr(),
                c"rt715_pcm_hw_params".as_ptr(),
                params_rate(params),
            );
            return -EINVAL;
        }
    }

    if params_channels(params) <= 16 {
        /* bit 3:0 Number of Channel */
        val |= params_channels(params) - 1;
    } else {
        dev_err(
            (*component).dev,
            c"%s: Unsupported channels %d\n".as_ptr(),
            c"rt715_pcm_hw_params".as_ptr(),
            params_channels(params),
        );
        return -EINVAL;
    }

    match params_width(params) {
        /* bit 6:4 Bits per Sample */
        8 => {}
        16 => {
            val |= 0x1 << 4;
        }
        20 => {
            val |= 0x2 << 4;
        }
        24 => {
            val |= 0x3 << 4;
        }
        32 => {
            val |= 0x4 << 4;
        }
        _ => return -EINVAL,
    }

    regmap_write((*rt715).regmap, RT715_MIC_ADC_FORMAT_H, val);
    regmap_write((*rt715).regmap, RT715_MIC_LINE_FORMAT_H, val);
    regmap_write((*rt715).regmap, RT715_MIX_ADC_FORMAT_H, val);
    regmap_write((*rt715).regmap, RT715_MIX_ADC2_FORMAT_H, val);

    retval
}

unsafe fn rt715_pcm_hw_free(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let rt715 = snd_soc_component_get_drvdata(component);
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);

    if (*rt715).slave.is_null() {
        return -EINVAL;
    }

    sdw_stream_remove_slave((*rt715).slave, sdw_stream);
    0
}

const RT715_STEREO_RATES_COMMENT: &str =
    "RT715_STEREO_RATES = SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000";
const RT715_FORMATS_COMMENT: &str =
    "RT715_FORMATS = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S8";

static rt715_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(rt715_pcm_hw_params),
    hw_free: Some(rt715_pcm_hw_free),
    set_stream: Some(rt715_set_sdw_stream),
    shutdown: Some(rt715_shutdown),
};

static mut rt715_dai: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c"rt715-aif1".as_ptr(),
        id: 0, /* RT715_AIF1 */
        capture: snd_soc_pcm_stream {
            stream_name: c"DP6 Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: 0,   /* RT715_STEREO_RATES */
            formats: 0, /* RT715_FORMATS */
        },
        ops: &rt715_ops,
    },
    snd_soc_dai_driver {
        name: c"rt715-aif2".as_ptr(),
        id: 0, /* RT715_AIF2 */
        capture: snd_soc_pcm_stream {
            stream_name: c"DP4 Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: 0,   /* RT715_STEREO_RATES */
            formats: 0, /* RT715_FORMATS */
        },
        ops: &rt715_ops,
    },
];

/* Bus clock frequency */
const RT715_CLK_FREQ_9600000HZ: c_uint = 9600000;
const RT715_CLK_FREQ_12000000HZ: c_uint = 12000000;
const RT715_CLK_FREQ_6000000HZ: c_uint = 6000000;
const RT715_CLK_FREQ_4800000HZ: c_uint = 4800000;
const RT715_CLK_FREQ_2400000HZ: c_uint = 2400000;
const RT715_CLK_FREQ_12288000HZ: c_uint = 12288000;

pub unsafe extern "C" fn rt715_clock_config(dev: *mut device) -> c_int {
    let rt715 = dev_get_drvdata(dev);
    let clk_freq: c_uint;
    let value: c_uint;

    clk_freq = (*rt715).params.curr_dr_freq >> 1;

    match clk_freq {
        RT715_CLK_FREQ_12000000HZ => value = 0x0,
        RT715_CLK_FREQ_6000000HZ => value = 0x1,
        RT715_CLK_FREQ_9600000HZ => value = 0x2,
        RT715_CLK_FREQ_4800000HZ => value = 0x3,
        RT715_CLK_FREQ_2400000HZ => value = 0x4,
        RT715_CLK_FREQ_12288000HZ => value = 0x5,
        _ => return -EINVAL,
    }

    regmap_write((*rt715).regmap, 0xe0, value);
    regmap_write((*rt715).regmap, 0xf0, value);

    0
}

pub unsafe extern "C" fn rt715_init(
    dev: *mut device,
    sdw_regmap: *mut regmap,
    regmap: *mut regmap,
    slave: *mut sdw_slave,
) -> c_int {
    let rt715: *mut rt715_priv;

    rt715 = devm_kzalloc(dev, core::mem::size_of::<rt715_priv>(), GFP_KERNEL) as *mut rt715_priv;
    if rt715.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, rt715);
    (*rt715).slave = slave;
    (*rt715).regmap = regmap;
    (*rt715).sdw_regmap = sdw_regmap;

    regcache_cache_only((*rt715).regmap, true);

    /*
     * Mark hw_init to false
     * HW init will be performed when device reports present
     */
    (*rt715).hw_init = false;
    (*rt715).first_hw_init = false;

    unsafe {
        rt715_dai[0].id = RT715_AIF1;
        rt715_dai[0].capture.rates = SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000;
        rt715_dai[0].capture.formats =
            SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S8;
        rt715_dai[1].id = RT715_AIF2;
        rt715_dai[1].capture.rates = SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000;
        rt715_dai[1].capture.formats =
            SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S8;
    }

    let ret = devm_snd_soc_register_component(
        dev,
        &soc_codec_dev_rt715,
        rt715_dai.as_mut_ptr(),
        rt715_dai.len() as c_int,
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

    0
}

pub unsafe extern "C" fn rt715_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int {
    let rt715 = dev_get_drvdata(dev);

    if (*rt715).hw_init {
        return 0;
    }

    regcache_cache_only((*rt715).regmap, false);

    /*
     *  PM runtime status is marked as 'active' only when a Slave reports as Attached
     */
    if !(*rt715).first_hw_init {
        /* update count of parent 'active' children */
        pm_runtime_set_active(&mut (*slave).dev);
    }

    pm_runtime_get_noresume(&mut (*slave).dev);

    rt715_reset((*rt715).regmap);

    /* Mute nid=08h/09h */
    regmap_write((*rt715).regmap, RT715_SET_GAIN_LINE_ADC_H, 0xb080);
    regmap_write((*rt715).regmap, RT715_SET_GAIN_MIX_ADC_H, 0xb080);
    /* Mute nid=07h/27h */
    regmap_write((*rt715).regmap, RT715_SET_GAIN_MIC_ADC_H, 0xb080);
    regmap_write((*rt715).regmap, RT715_SET_GAIN_MIX_ADC2_H, 0xb080);

    /* Set Pin Widget */
    regmap_write((*rt715).regmap, RT715_SET_PIN_DMIC1, 0x20);
    regmap_write((*rt715).regmap, RT715_SET_PIN_DMIC2, 0x20);
    regmap_write((*rt715).regmap, RT715_SET_PIN_DMIC3, 0x20);
    regmap_write((*rt715).regmap, RT715_SET_PIN_DMIC4, 0x20);
    /* Set Converter Stream */
    regmap_write((*rt715).regmap, RT715_SET_STREAMID_LINE_ADC, 0x10);
    regmap_write((*rt715).regmap, RT715_SET_STREAMID_MIX_ADC, 0x10);
    regmap_write((*rt715).regmap, RT715_SET_STREAMID_MIC_ADC, 0x10);
    regmap_write((*rt715).regmap, RT715_SET_STREAMID_MIX_ADC2, 0x10);
    /* Set Configuration Default */
    regmap_write((*rt715).regmap, RT715_SET_DMIC1_CONFIG_DEFAULT1, 0xd0);
    regmap_write((*rt715).regmap, RT715_SET_DMIC1_CONFIG_DEFAULT2, 0x11);
    regmap_write((*rt715).regmap, RT715_SET_DMIC1_CONFIG_DEFAULT3, 0xa1);
    regmap_write((*rt715).regmap, RT715_SET_DMIC1_CONFIG_DEFAULT4, 0x81);
    regmap_write((*rt715).regmap, RT715_SET_DMIC2_CONFIG_DEFAULT1, 0xd1);
    regmap_write((*rt715).regmap, RT715_SET_DMIC2_CONFIG_DEFAULT2, 0x11);
    regmap_write((*rt715).regmap, RT715_SET_DMIC2_CONFIG_DEFAULT3, 0xa1);
    regmap_write((*rt715).regmap, RT715_SET_DMIC2_CONFIG_DEFAULT4, 0x81);
    regmap_write((*rt715).regmap, RT715_SET_DMIC3_CONFIG_DEFAULT1, 0xd0);
    regmap_write((*rt715).regmap, RT715_SET_DMIC3_CONFIG_DEFAULT2, 0x11);
    regmap_write((*rt715).regmap, RT715_SET_DMIC3_CONFIG_DEFAULT3, 0xa1);
    regmap_write((*rt715).regmap, RT715_SET_DMIC3_CONFIG_DEFAULT4, 0x81);
    regmap_write((*rt715).regmap, RT715_SET_DMIC4_CONFIG_DEFAULT1, 0xd1);
    regmap_write((*rt715).regmap, RT715_SET_DMIC4_CONFIG_DEFAULT2, 0x11);
    regmap_write((*rt715).regmap, RT715_SET_DMIC4_CONFIG_DEFAULT3, 0xa1);
    regmap_write((*rt715).regmap, RT715_SET_DMIC4_CONFIG_DEFAULT4, 0x81);

    /* Finish Initial Settings, set power to D3 */
    regmap_write((*rt715).regmap, RT715_SET_AUDIO_POWER_STATE, AC_PWRST_D3);

    if (*rt715).first_hw_init {
        regcache_mark_dirty((*rt715).regmap);
    } else {
        (*rt715).first_hw_init = true;
    }

    /* Mark Slave initialization complete */
    (*rt715).hw_init = true;

    pm_runtime_put_autosuspend(&mut (*slave).dev);

    0
}

/* MODULE_DESCRIPTION("ASoC rt715 driver"); */
/* MODULE_DESCRIPTION("ASoC rt715 driver SDW"); */
/* MODULE_AUTHOR("Jack Yu <jack.yu@realtek.com>"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
