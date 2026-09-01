// SPDX-License-Identifier: GPL-2.0-only
//
// rt766-sdca.c -- rt766 SDCA ALSA SoC audio driver
//
// Copyright(c) 2026 Realtek Semiconductor Corp.
//
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type irqreturn_t = c_uint;
type irq_handler_t = Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>;

const IRQ_HANDLED: irqreturn_t = 1;
const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const EACCES: c_int = 13;
const ENOMEM: c_int = 12;
const HID_INPUT_REPORT: c_uint = 1;
const SND_JACK_HEADPHONE: c_uint = 0x0001;
const SND_JACK_HEADSET: c_uint = 0x0003;
const SND_SOC_NOPM: c_int = -1;
const SND_SOC_DAPM_POST_PMU: c_int = 0x1;
const SND_SOC_DAPM_PRE_PMD: c_int = 0x2;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SDW_DATA_DIR_RX: sdw_data_direction = 0;
const SDW_DATA_DIR_TX: sdw_data_direction = 1;
const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_uint = 1;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 2;

const RT766_P75DB_STEP: c_uint = 0xC0; /* 0.75 dB in Q7.8 format */
const RT766_2DB_STEP: c_uint = 0x200; /* 2 dB in Q7.8 format */
const RT766_HP_VOL_MIN: c_int = -127; /* -95.25 dB / 0.75 dB step */
const RT766_HS_VOL_MIN: c_int = -23; /* -17.25 dB / 0.75 dB step */
const RT766_HS_BOOST_VOL_MIN: c_int = -1; /* -2 dB / 2 dB step */
const RT766_SPK_VOL_MIN: c_int = -87; /* -65.25 dB / 0.75 dB step */
const RT766_P_VOL_MAX: c_int = 0; /* 0 dB / 0.75 dB step */
const RT766_HS_VOL_MAX: c_int = 40; /* 30 dB / 0.75 dB step */
const RT766_HS_BOOST_VOL_MAX: c_int = 20; /* 40 dB / 2 dB step */

const RT766_STEREO_RATES: c_uint =
    SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000;
const RT766_DAC_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE;
const RT766_ADC_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

type sdw_data_direction = c_int;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct hid_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
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
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
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
    pub min: i64,
    pub max: i64,
}
#[repr(C)]
pub struct snd_soc_card {
    pub instantiated: bool_,
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
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
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
    pub name: *const c_char,
    pub id: c_int,
}
#[repr(C)]
pub struct sdw_stream_config {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sdw_port_config {
    pub num: c_int,
}
#[repr(C)]
pub struct sdw_stream_runtime {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sdca_function_desc {
    pub type_: c_int,
    pub name: *const c_char,
}
#[repr(C)]
pub struct sdca_iot {
    pub is_dataport: bool_,
}
#[repr(C)]
pub struct sdca_pde {
    pub max_delay: c_uint,
    pub num_max_delay: c_uint,
}
#[repr(C)]
pub struct sdca_control {
    pub sel: c_uint,
    pub interrupt_position: c_int,
}
#[repr(C)]
pub struct sdca_entity {
    pub type_: c_uint,
    pub label: *const c_char,
    pub num_controls: c_int,
    pub controls: *mut sdca_control,
    pub iot: sdca_iot,
    pub pde: sdca_pde,
}
#[repr(C)]
pub struct sdca_function_data {
    pub desc: *mut sdca_function_desc,
    pub num_entities: c_int,
    pub entities: *mut sdca_entity,
}
#[repr(C)]
pub struct sdca_interrupt {
    pub priv_: *mut c_void,
    pub name: *const c_char,
    pub free_priv: Option<unsafe extern "C" fn(*mut sdca_interrupt)>,
}
#[repr(C)]
pub struct sdca_interrupt_info {
    pub irqs: *mut sdca_interrupt,
}
#[repr(C)]
pub struct sdca_slave_function_data {
    pub num_functions: c_int,
    pub function: *mut sdca_function_desc,
}
#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
    pub irq: c_int,
    pub sdca_data: sdca_slave_function_data,
}
#[repr(C)]
pub struct rt_sdca_dmic_kctrl_priv {
    pub reg_base: c_uint,
    pub count: c_uint,
    pub max: c_uint,
}
#[repr(C)]
pub struct rt766_sdca_priv {
    pub slave: *mut sdw_slave,
    pub regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub hs_jack: *mut snd_soc_jack,
    pub hid: *mut hid_device,
    pub irq_info: *mut sdca_interrupt_info,
    pub uaj_func_data: *mut sdca_function_data,
    pub hid_func_data: *mut sdca_function_data,
    pub sa_func_data: *mut sdca_function_data,
    pub sm_func_data: *mut sdca_function_data,
    pub disable_irq_lock: mutex,
    pub disable_irq: bool_,
    pub hw_init: bool_,
    pub first_hw_init: bool_,
    pub jack_type: c_uint,
    pub fu41_dapm_mute: bool_,
    pub fu41_mixer_l_mute: bool_,
    pub fu41_mixer_r_mute: bool_,
    pub fu36_dapm_mute: bool_,
    pub fu36_mixer_l_mute: bool_,
    pub fu36_mixer_r_mute: bool_,
    pub fu21_dapm_mute: bool_,
    pub fu21_mixer_l_mute: bool_,
    pub fu21_mixer_r_mute: bool_,
    pub fu113_dapm_mute: bool_,
    pub fu113_mixer_mute: [bool_; 4],
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub set_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, *mut c_void, c_int) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
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
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub set_jack: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_jack, *mut c_void) -> c_int>,
    pub endianness: c_uint,
}

unsafe extern "C" {
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
    static RT766_AIF1: c_int;
    static RT766_AIF2: c_int;
    static RT766_AIF3: c_int;
    static RT766_DAI_UAJ: usize;
    static RT766_DAI_AMP: usize;
    static RT766_DAI_MIC: usize;
    static RT766_FUNC_NUM_UAJ: c_int;
    static RT766_FUNC_NUM_AMP: c_int;
    static RT766_FUNC_NUM_MIC: c_int;
    static RT766_SDCA_ENT_USER_FU41: c_int;
    static RT766_SDCA_ENT_USER_FU36: c_int;
    static RT766_SDCA_ENT_USER_FU21: c_int;
    static RT766_SDCA_ENT_USER_FU113: c_int;
    static RT766_SDCA_ENT_PDE47: c_int;
    static RT766_SDCA_ENT_PDE34: c_int;
    static RT766_SDCA_ENT_PDE23: c_int;
    static RT766_SDCA_ENT_PDE11: c_int;
    static RT766_SDCA_RATE_44100HZ: c_uint;
    static RT766_SDCA_RATE_48000HZ: c_uint;
    static RT766_SDCA_RATE_96000HZ: c_uint;
    static RT766_SDCA_RATE_192000HZ: c_uint;
    static RT766_BOND_LATCH_ID: c_uint;
    static SDCA_CTL_ENTITY_0_FUNCTION_NEEDS_INITIALIZATION: c_uint;
    static SDCA_FUNCTION_TYPE_UAJ: c_int;
    static SDCA_FUNCTION_TYPE_SMART_AMP: c_int;
    static SDCA_FUNCTION_TYPE_SMART_MIC: c_int;
    static SDCA_FUNCTION_TYPE_HID: c_int;

    fn RT766_SDCA_CTL(function: c_int, entity: c_int, control: c_int) -> c_uint;
    fn RT766_VOLUME_REG(function: c_int, entity: c_int, channel: c_int) -> c_uint;
    fn RT766_GAIN_REG(function: c_int, entity: c_int, channel: c_int) -> c_uint;
    fn RT766_MUTE_REG(function: c_int, entity: c_int, channel: c_int) -> c_uint;
    fn RT766_FUNC_STATUS_REG(function: c_int) -> c_uint;
    fn SDW_SDCA_CTL(function: c_int, entity: c_int, control: c_int, channel: c_int) -> c_uint;
    fn SDCA_CTL_TYPE(entity_type: c_uint, sel: c_uint) -> c_uint;

    static UAJ: c_int;
    static GE49: c_int;
    static HID: c_int;
    static HID101: c_int;
    static AMP: c_int;
    static PPU21: c_int;
    static MIC: c_int;
    static CS41: c_int;
    static CS36: c_int;
    static CS21: c_int;
    static CS113: c_int;
    static USER_FU41: c_int;
    static USER_FU36: c_int;
    static USER_FU21: c_int;
    static USER_FU113: c_int;
    static PLATFORM_FU33: c_int;
    static SDCA_CTL_GE_DETECTED_MODE: c_int;
    static SDCA_CTL_HIDE_HIDTX_CURRENTOWNER: c_int;
    static SDCA_CTL_HIDE_HIDTX_MESSAGELENGTH: c_int;
    static SDCA_CTL_HIDE_HIDTX_MESSAGEOFFSET: c_int;
    static SDCA_CTL_GE_SELECTED_MODE: c_int;
    static SDCA_CTL_FU_MUTE: c_int;
    static SDCA_CTL_PDE_REQUESTED_PS: c_int;
    static SDCA_CTL_PPU_POSTURENUMBER: c_int;
    static SDCA_CTL_CS_SAMPLERATEINDEX: c_int;
    static SDCA_CTL_TYPE_S_GE_DETECTED_MODE: c_uint;
    static SDCA_CTL_TYPE_S_HIDE_HIDTX_CURRENTOWNER: c_uint;
    static RT766_BUF_ADDR_HID1: c_uint;

    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_cache_bypass(map: *mut regmap, enable: bool_);
    fn regcache_mark_dirty(map: *mut regmap);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kfree(dev: *mut device, p: *mut c_void);
    fn devm_mutex_init(dev: *mut device, lock: *mut mutex) -> c_int;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err_ratelimited(fmt: *const c_char, ...);
    fn hid_input_report(hid: *mut hid_device, report_type: c_uint, data: *mut u8, size: c_uint, interrupt: c_int);
    fn hid_destroy_device(hid: *mut hid_device);
    fn sdca_add_hid_device(interrupt: *mut sdca_interrupt) -> c_int;
    fn sdca_irq_data_populate(dev: *mut device, map: *mut regmap, component: *mut snd_soc_component, function: *mut sdca_function_data, entity: *mut sdca_entity, control: *mut sdca_control, interrupt: *mut sdca_interrupt) -> c_int;
    fn sdca_irq_request(dev: *mut device, info: *mut sdca_interrupt_info, irq: c_int, name: *const c_char, handler: irq_handler_t, interrupt: *mut sdca_interrupt) -> c_int;
    fn sdca_irq_cleanup_late(dev: *mut device, function: *mut sdca_function_data, info: *mut sdca_interrupt_info);
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut rt766_sdca_priv;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_uint, mask: c_uint);
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_resume(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device);
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn sdca_find_entity_by_label(function: *mut sdca_function_data, label: *const c_char) -> *const sdca_entity;
    fn sdca_asoc_pde_poll_actual_ps(map: *mut regmap, func_num: c_int, pde_num: c_int, from_ps: c_int, to_ps: c_int, max_delay: c_uint, num_max_delay: c_uint) -> c_int;
    fn snd_soc_dai_dma_data_set(dai: *mut snd_soc_dai, direction: c_int, data: *mut c_void);
    fn snd_soc_dai_set_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream, data: *mut c_void);
    fn snd_soc_dai_get_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream) -> *mut sdw_stream_runtime;
    fn snd_sdw_params_to_config(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, stream_config: *mut sdw_stream_config, port_config: *mut sdw_port_config);
    fn sdw_stream_add_slave(slave: *mut sdw_slave, stream_config: *mut sdw_stream_config, port_config: *mut sdw_port_config, num_ports: c_uint, stream: *mut sdw_stream_runtime) -> c_int;
    fn sdw_stream_remove_slave(slave: *mut sdw_slave, stream: *mut sdw_stream_runtime);
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn sdca_asoc_populate_rate_format(dev: *mut device, function: *mut sdca_function_data, entity: *mut sdca_entity, stream: *mut snd_soc_pcm_stream) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn sdca_parse_function(dev: *mut device, function: *mut sdca_function_data) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn devm_sdca_irq_allocate(dev: *mut device, map: *mut regmap, irq: c_int) -> *mut sdca_interrupt_info;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut rt766_sdca_priv;
    fn sdca_regmap_write_init(dev: *mut device, map: *mut regmap, function: *mut sdca_function_data) -> c_int;
}

unsafe extern "C" fn rt766_sdca_btn_detect(interrupt: *mut sdca_interrupt) -> c_int {
    let rt766 = (*interrupt).priv_ as *mut rt766_sdca_priv;
    let mut buf: *mut u8 = ptr::null_mut();
    let mut offset: c_uint = 0;
    let mut owner: c_uint = 0;
    let mut length: c_uint = 0;
    let mut det_mode: c_uint = 0;
    let mut idx: c_uint;
    let mut val: c_uint = 0;
    let mut ret: c_int;

    ret = regmap_read((*rt766).regmap, RT766_SDCA_CTL(UAJ, GE49, SDCA_CTL_GE_DETECTED_MODE), &mut det_mode);
    if ret < 0 {
        pr_err_ratelimited(c"IO error in %s, ret %d\n".as_ptr(), c"rt766_sdca_btn_detect".as_ptr(), ret);
        return ret;
    }

    /* get current UMP message owner */
    ret = regmap_read((*rt766).regmap, RT766_SDCA_CTL(HID, HID101, SDCA_CTL_HIDE_HIDTX_CURRENTOWNER), &mut owner);
    if ret < 0 {
        pr_err_ratelimited(c"IO error in %s, ret %d\n".as_ptr(), c"rt766_sdca_btn_detect".as_ptr(), ret);
        return ret;
    }

    /* if owner is device then there is no button event from device */
    if owner == 1 {
        return 0;
    }

    if det_mode != 0 {
        /* read UMP message length */
        ret = regmap_read((*rt766).regmap, RT766_SDCA_CTL(HID, HID101, SDCA_CTL_HIDE_HIDTX_MESSAGELENGTH), &mut length);
        if ret < 0 {
            goto_end_btn_det(rt766, buf, owner);
            return 0;
        }

        /* read UMP message offset */
        ret = regmap_read((*rt766).regmap, RT766_SDCA_CTL(HID, HID101, SDCA_CTL_HIDE_HIDTX_MESSAGEOFFSET), &mut offset);
        if ret < 0 {
            goto_end_btn_det(rt766, buf, owner);
            return 0;
        }

        buf = devm_kzalloc(&mut (*(*rt766).slave).dev, length as usize, GFP_KERNEL) as *mut u8;
        if buf.is_null() {
            dev_err(&mut (*(*rt766).slave).dev, c"%s: alloc buf failed\n".as_ptr(), c"rt766_sdca_btn_detect".as_ptr());
            goto_end_btn_det(rt766, buf, owner);
            return 0;
        }

        idx = 0;
        while idx < length {
            ret = regmap_read((*rt766).regmap, RT766_BUF_ADDR_HID1 + offset + idx, &mut val);
            if ret < 0 {
                goto_end_btn_det(rt766, buf, owner);
                return 0;
            }
            *buf.add(idx as usize) = (val & 0xff) as u8;
            idx += 1;
        }

        if !(*rt766).hid.is_null() {
            hid_input_report((*rt766).hid, HID_INPUT_REPORT, buf, length, 1);
        }
    }

    goto_end_btn_det(rt766, buf, owner);
    0
}

unsafe fn goto_end_btn_det(rt766: *mut rt766_sdca_priv, buf: *mut u8, owner: c_uint) {
    if !buf.is_null() {
        devm_kfree(&mut (*(*rt766).slave).dev, buf as *mut c_void);
    }

    /* Host is owner, so set back to device */
    if owner == 0 {
        regmap_write((*rt766).regmap, RT766_SDCA_CTL(HID, HID101, SDCA_CTL_HIDE_HIDTX_CURRENTOWNER), 0x01);
    }
}

unsafe extern "C" fn rt766_sdca_irq_btn_handler(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let interrupt = data as *mut sdca_interrupt;
    let rt766 = (*interrupt).priv_ as *mut rt766_sdca_priv;

    if (*rt766).hs_jack.is_null() {
        return IRQ_HANDLED;
    }
    if (*(*rt766).component).card.is_null() || !(*(*(*rt766).component).card).instantiated {
        return IRQ_HANDLED;
    }

    mutex_lock(&mut (*rt766).disable_irq_lock);
    if !(*rt766).disable_irq {
        rt766_sdca_btn_detect(interrupt);
    }
    mutex_unlock(&mut (*rt766).disable_irq_lock);
    IRQ_HANDLED
}

unsafe extern "C" fn rt766_sdca_headset_detect(rt766: *mut rt766_sdca_priv) -> c_int {
    let mut det_mode: c_uint = 0;
    let mut ret: c_int;

    /* get detected_mode */
    ret = regmap_read((*rt766).regmap, RT766_SDCA_CTL(UAJ, GE49, SDCA_CTL_GE_DETECTED_MODE), &mut det_mode);
    if ret < 0 {
        pr_err_ratelimited(c"IO error in %s, ret %d\n".as_ptr(), c"rt766_sdca_headset_detect".as_ptr(), ret);
        return ret;
    }

    match det_mode {
        0x00 => (*rt766).jack_type = 0,
        0x03 => (*rt766).jack_type = SND_JACK_HEADPHONE,
        0x05 => (*rt766).jack_type = SND_JACK_HEADSET,
        _ => {}
    }

    /* write selected_mode */
    if det_mode != 0 {
        ret = regmap_write((*rt766).regmap, RT766_SDCA_CTL(UAJ, GE49, SDCA_CTL_GE_SELECTED_MODE), det_mode);
        if ret < 0 {
            pr_err_ratelimited(c"IO error in %s, ret %d\n".as_ptr(), c"rt766_sdca_headset_detect".as_ptr(), ret);
            return ret;
        }
    }

    dev_dbg(&mut (*(*rt766).slave).dev, c"%s, detected_mode=0x%x\n".as_ptr(), c"rt766_sdca_headset_detect".as_ptr(), det_mode);
    0
}

unsafe extern "C" fn rt766_sdca_irq_jd_handler(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let interrupt = data as *mut sdca_interrupt;
    let rt766 = (*interrupt).priv_ as *mut rt766_sdca_priv;

    if (*rt766).hs_jack.is_null() {
        return IRQ_HANDLED;
    }
    if (*(*rt766).component).card.is_null() || !(*(*(*rt766).component).card).instantiated {
        return IRQ_HANDLED;
    }

    mutex_lock(&mut (*rt766).disable_irq_lock);
    if !(*rt766).disable_irq {
        rt766_sdca_headset_detect(rt766);
    }
    mutex_unlock(&mut (*rt766).disable_irq_lock);

    dev_dbg(&mut (*(*rt766).slave).dev, c"in %s, jack_type=%d\n".as_ptr(), c"rt766_sdca_irq_jd_handler".as_ptr(), (*rt766).jack_type);
    snd_soc_jack_report((*rt766).hs_jack, (*rt766).jack_type, SND_JACK_HEADSET);
    IRQ_HANDLED
}

unsafe extern "C" fn rt766_sdca_destroy_hid_device(interrupt: *mut sdca_interrupt) {
    let rt766 = (*interrupt).priv_ as *mut rt766_sdca_priv;
    hid_destroy_device((*rt766).hid);
}

unsafe extern "C" fn rt766_sdca_irq_ctl(
    rt766: *mut rt766_sdca_priv,
    function: *mut sdca_function_data,
    component: *mut snd_soc_component,
    info: *mut sdca_interrupt_info,
    enabled: bool_,
) -> c_int {
    let dev = &mut (*(*rt766).slave).dev as *mut device;
    let mut interrupt: *mut sdca_interrupt;
    let mut control: *mut sdca_control;
    let mut entity: *mut sdca_entity;
    let mut handler: irq_handler_t;
    let mut i: c_int = 0;
    let mut ret: c_int;

    while i < (*function).num_entities {
        entity = (*function).entities.add(i as usize);
        let mut j: c_int = 0;
        while j < (*entity).num_controls {
            control = (*entity).controls.add(j as usize);
            let irq = (*control).interrupt_position;

            match SDCA_CTL_TYPE((*entity).type_, (*control).sel) {
                x if x == SDCA_CTL_TYPE_S_GE_DETECTED_MODE => handler = Some(rt766_sdca_irq_jd_handler),
                x if x == SDCA_CTL_TYPE_S_HIDE_HIDTX_CURRENTOWNER => handler = Some(rt766_sdca_irq_btn_handler),
                _ => {
                    j += 1;
                    continue;
                }
            }

            interrupt = (*info).irqs.add(irq as usize);
            if enabled {
                ret = sdca_irq_data_populate(dev, (*rt766).regmap, component, function, entity, control, interrupt);
                if ret != 0 {
                    return ret;
                }

                if handler == Some(rt766_sdca_irq_btn_handler) {
                    ret = sdca_add_hid_device(interrupt);
                    if ret != 0 {
                        return ret;
                    }
                    (*interrupt).free_priv = Some(rt766_sdca_destroy_hid_device);
                    (*rt766).hid = (*interrupt).priv_ as *mut hid_device;
                }

                (*interrupt).priv_ = rt766 as *mut c_void;
                ret = sdca_irq_request(dev, info, irq, (*interrupt).name, handler, interrupt);
                if ret != 0 {
                    dev_err(dev, c"failed to request irq %s: %d\n".as_ptr(), (*interrupt).name, ret);
                    sdca_irq_cleanup_late(dev, function, info);
                    return ret;
                }
                dev_dbg(dev, c"Requesting IRQ %d InterruptName=%s\n".as_ptr(), irq, (*interrupt).name);
            } else {
                sdca_irq_cleanup_late(dev, function, info);
                dev_dbg(dev, c"Freeing IRQ %d\n".as_ptr(), irq);
            }
            j += 1;
        }
        i += 1;
    }
    0
}

unsafe extern "C" fn rt766_sdca_set_jack_detect(component: *mut snd_soc_component, hs_jack: *mut snd_soc_jack, _data: *mut c_void) -> c_int {
    let rt766 = snd_soc_component_get_drvdata(component);
    let mut ret: c_int;

    if (*rt766).uaj_func_data.is_null() {
        dev_err(&mut (*(*rt766).slave).dev, c"The SDCA UAJ function is not supported.\n".as_ptr());
        return -EINVAL;
    }
    (*rt766).hs_jack = hs_jack;
    if !(*rt766).first_hw_init {
        return 0;
    }

    ret = pm_runtime_resume_and_get((*component).dev);
    if ret < 0 {
        if ret != -EACCES {
            dev_err((*component).dev, c"%s: failed to resume %d\n".as_ptr(), c"rt766_sdca_set_jack_detect".as_ptr(), ret);
            return ret;
        }
        /* pm_runtime not enabled yet */
        dev_dbg((*component).dev, c"%s: skipping jack init for now\n".as_ptr(), c"rt766_sdca_set_jack_detect".as_ptr());
        return 0;
    }

    /* disable interrupts if hs_jack is not set */
    if (*rt766).hs_jack.is_null() {
        if !(*rt766).uaj_func_data.is_null() {
            rt766_sdca_irq_ctl(rt766, (*rt766).uaj_func_data, (*rt766).component, (*rt766).irq_info, false);
        }
        if !(*rt766).hid_func_data.is_null() {
            rt766_sdca_irq_ctl(rt766, (*rt766).hid_func_data, (*rt766).component, (*rt766).irq_info, false);
        }
    }
    pm_runtime_put_autosuspend((*component).dev);
    0
}

unsafe extern "C" fn rt766_sdca_set_fu_ctl(rt766: *mut rt766_sdca_priv, func_num: c_int, fu_num: c_int) -> c_int {
    let ch_01: c_uint;
    let ch_02: c_uint;
    let mut ch_mute: c_uint;
    let mut fu_reg: c_uint;
    let mut err: c_int;

    if fu_num == RT766_SDCA_ENT_USER_FU41 {
        ch_01 = if (*rt766).fu41_dapm_mute || (*rt766).fu41_mixer_l_mute { 0x01 } else { 0x00 };
        ch_02 = if (*rt766).fu41_dapm_mute || (*rt766).fu41_mixer_r_mute { 0x01 } else { 0x00 };
    } else if fu_num == RT766_SDCA_ENT_USER_FU36 {
        ch_01 = if (*rt766).fu36_dapm_mute || (*rt766).fu36_mixer_l_mute { 0x01 } else { 0x00 };
        ch_02 = if (*rt766).fu36_dapm_mute || (*rt766).fu36_mixer_r_mute { 0x01 } else { 0x00 };
    } else if fu_num == RT766_SDCA_ENT_USER_FU21 {
        ch_01 = if (*rt766).fu21_dapm_mute || (*rt766).fu21_mixer_l_mute { 0x01 } else { 0x00 };
        ch_02 = if (*rt766).fu21_dapm_mute || (*rt766).fu21_mixer_r_mute { 0x01 } else { 0x00 };
    } else if fu_num == RT766_SDCA_ENT_USER_FU113 {
        let mut i = 0usize;
        while i < (*rt766).fu113_mixer_mute.len() {
            ch_mute = if (*rt766).fu113_dapm_mute || (*rt766).fu113_mixer_mute[i] { 0x01 } else { 0x00 };
            fu_reg = SDW_SDCA_CTL(func_num, fu_num, SDCA_CTL_FU_MUTE, 1) + i as c_uint;
            err = regmap_write((*rt766).regmap, fu_reg, ch_mute);
            if err < 0 {
                return err;
            }
            i += 1;
        }
        return 0;
    } else {
        return 0;
    }

    err = regmap_write((*rt766).regmap, SDW_SDCA_CTL(func_num, fu_num, SDCA_CTL_FU_MUTE, 1), ch_01);
    if err < 0 {
        return err;
    }
    err = regmap_write((*rt766).regmap, SDW_SDCA_CTL(func_num, fu_num, SDCA_CTL_FU_MUTE, 2), ch_02);
    if err < 0 {
        return err;
    }
    0
}

unsafe extern "C" fn rt766_sdca_fu41_playback_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt766 = snd_soc_component_get_drvdata(component);
    (*ucontrol).value.integer.value[0] = (!(*rt766).fu41_mixer_l_mute) as i64;
    (*ucontrol).value.integer.value[1] = (!(*rt766).fu41_mixer_r_mute) as i64;
    0
}

unsafe extern "C" fn rt766_sdca_fu41_playback_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt766 = snd_soc_component_get_drvdata(component);
    if (*rt766).fu41_mixer_l_mute == ((*ucontrol).value.integer.value[0] == 0)
        && (*rt766).fu41_mixer_r_mute == ((*ucontrol).value.integer.value[1] == 0)
    {
        return 0;
    }
    (*rt766).fu41_mixer_l_mute = (*ucontrol).value.integer.value[0] == 0;
    (*rt766).fu41_mixer_r_mute = (*ucontrol).value.integer.value[1] == 0;
    let err = rt766_sdca_set_fu_ctl(rt766, RT766_FUNC_NUM_UAJ, RT766_SDCA_ENT_USER_FU41);
    if err < 0 { return err; }
    1
}

unsafe extern "C" fn rt766_sdca_fu36_capture_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt766 = snd_soc_component_get_drvdata(component);
    (*ucontrol).value.integer.value[0] = (!(*rt766).fu36_mixer_l_mute) as i64;
    (*ucontrol).value.integer.value[1] = (!(*rt766).fu36_mixer_r_mute) as i64;
    0
}

unsafe extern "C" fn rt766_sdca_fu36_capture_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt766 = snd_soc_component_get_drvdata(component);
    if (*rt766).fu36_mixer_l_mute == ((*ucontrol).value.integer.value[0] == 0)
        && (*rt766).fu36_mixer_r_mute == ((*ucontrol).value.integer.value[1] == 0)
    {
        return 0;
    }
    (*rt766).fu36_mixer_l_mute = (*ucontrol).value.integer.value[0] == 0;
    (*rt766).fu36_mixer_r_mute = (*ucontrol).value.integer.value[1] == 0;
    let err = rt766_sdca_set_fu_ctl(rt766, RT766_FUNC_NUM_UAJ, RT766_SDCA_ENT_USER_FU36);
    if err < 0 { return err; }
    1
}

unsafe extern "C" fn rt766_sdca_fu21_playback_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt766 = snd_soc_component_get_drvdata(component);
    (*ucontrol).value.integer.value[0] = (!(*rt766).fu21_mixer_l_mute) as i64;
    (*ucontrol).value.integer.value[1] = (!(*rt766).fu21_mixer_r_mute) as i64;
    0
}

unsafe extern "C" fn rt766_sdca_fu21_playback_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt766 = snd_soc_component_get_drvdata(component);
    if (*rt766).fu21_mixer_l_mute == ((*ucontrol).value.integer.value[0] == 0)
        && (*rt766).fu21_mixer_r_mute == ((*ucontrol).value.integer.value[1] == 0)
    {
        return 0;
    }
    (*rt766).fu21_mixer_l_mute = (*ucontrol).value.integer.value[0] == 0;
    (*rt766).fu21_mixer_r_mute = (*ucontrol).value.integer.value[1] == 0;
    let err = rt766_sdca_set_fu_ctl(rt766, RT766_FUNC_NUM_AMP, RT766_SDCA_ENT_USER_FU21);
    if err < 0 { return err; }
    1
}

unsafe extern "C" fn rt766_sdca_fu113_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt766 = snd_soc_component_get_drvdata(component);
    match event {
        SND_SOC_DAPM_POST_PMU => {
            (*rt766).fu113_dapm_mute = false;
            rt766_sdca_set_fu_ctl(rt766, RT766_FUNC_NUM_MIC, RT766_SDCA_ENT_USER_FU113);
        }
        SND_SOC_DAPM_PRE_PMD => {
            (*rt766).fu113_dapm_mute = true;
            rt766_sdca_set_fu_ctl(rt766, RT766_FUNC_NUM_MIC, RT766_SDCA_ENT_USER_FU113);
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn rt766_sdca_dmic_set_gain_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt766 = snd_soc_component_get_drvdata(component);
    let p = (*kcontrol).private_value as *mut rt_sdca_dmic_kctrl_priv;
    let interval_offset: c_uint = 0xc0;
    let mut regvalue: c_uint = 0;
    let mut i: c_uint = 0;
    /* check all channels */
    while i < (*p).count {
        regmap_read((*rt766).regmap, (*p).reg_base + i, &mut regvalue);
        let ctl = (*p).max - (((0x1e00u32.wrapping_sub(regvalue)) & 0xffff) / interval_offset);
        (*ucontrol).value.integer.value[i as usize] = ctl as i64;
        i += 1;
    }
    0
}

unsafe extern "C" fn rt766_sdca_dmic_set_gain_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let p = (*kcontrol).private_value as *mut rt_sdca_dmic_kctrl_priv;
    let rt766 = snd_soc_component_get_drvdata(component);
    let interval_offset: c_uint = 0xc0;
    let mut gain_val = [0u32; 4];
    let mut regvalue = [0u32; 4];
    let mut changed: c_uint = 0;
    let mut i: c_uint = 0;
    /* check all channels */
    while i < (*p).count {
        regmap_read((*rt766).regmap, (*p).reg_base + i, &mut regvalue[i as usize]);
        gain_val[i as usize] = (*ucontrol).value.integer.value[i as usize] as c_uint;
        if gain_val[i as usize] > (*p).max {
            gain_val[i as usize] = (*p).max;
        }
        gain_val[i as usize] = 0x1e00u32.wrapping_sub(((*p).max - gain_val[i as usize]) * interval_offset);
        gain_val[i as usize] &= 0xffff;
        if regvalue[i as usize] != gain_val[i as usize] {
            changed = 1;
        }
        i += 1;
    }
    if changed == 0 {
        return 0;
    }
    i = 0;
    while i < (*p).count {
        let err = regmap_write((*rt766).regmap, (*p).reg_base + i, gain_val[i as usize]);
        if err < 0 {
            dev_err(&mut (*(*rt766).slave).dev, c"0x%08x can't be set\n".as_ptr(), (*p).reg_base + i);
        }
        i += 1;
    }
    changed as c_int
}

unsafe extern "C" fn rt766_sdca_dmic_fu113_capture_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt766 = snd_soc_component_get_drvdata(component);
    let mut i = 0usize;
    while i < 4 {
        (*ucontrol).value.integer.value[i] = (!(*rt766).fu113_mixer_mute[i]) as i64;
        i += 1;
    }
    0
}

unsafe extern "C" fn rt766_sdca_dmic_fu113_capture_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt766 = snd_soc_component_get_drvdata(component);
    let mut changed: c_int = 0;
    let mut i = 0usize;
    while i < 4 {
        if (*rt766).fu113_mixer_mute[i] != ((*ucontrol).value.integer.value[i] == 0) {
            changed = 1;
        }
        (*rt766).fu113_mixer_mute[i] = (*ucontrol).value.integer.value[i] == 0;
        i += 1;
    }
    let err = rt766_sdca_set_fu_ctl(rt766, RT766_FUNC_NUM_MIC, RT766_SDCA_ENT_USER_FU113);
    if err < 0 { return err; }
    changed
}

unsafe extern "C" fn rt766_sdca_fu41_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt766 = snd_soc_component_get_drvdata(component);
    match event {
        SND_SOC_DAPM_POST_PMU => { (*rt766).fu41_dapm_mute = false; rt766_sdca_set_fu_ctl(rt766, RT766_FUNC_NUM_UAJ, RT766_SDCA_ENT_USER_FU41); }
        SND_SOC_DAPM_PRE_PMD => { (*rt766).fu41_dapm_mute = true; rt766_sdca_set_fu_ctl(rt766, RT766_FUNC_NUM_UAJ, RT766_SDCA_ENT_USER_FU41); }
        _ => {}
    }
    0
}

unsafe extern "C" fn rt766_sdca_pde_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int, func_num: c_int, pde_num: c_int, pde_ent: *const c_char) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt766 = snd_soc_component_get_drvdata(component);
    let func_data: *mut sdca_function_data;
    let ps0: u8 = 0x0;
    let ps3: u8 = 0x3;
    let from_ps: c_int;
    let to_ps: c_int;
    let pde_req_reg = SDW_SDCA_CTL(func_num, pde_num, SDCA_CTL_PDE_REQUESTED_PS, 0);

    if func_num == RT766_FUNC_NUM_UAJ {
        func_data = (*rt766).uaj_func_data;
    } else if func_num == RT766_FUNC_NUM_AMP {
        func_data = (*rt766).sa_func_data;
    } else if func_num == RT766_FUNC_NUM_MIC {
        func_data = (*rt766).sm_func_data;
    } else {
        dev_err((*component).dev, c"%s: unsupported func_num %d\n".as_ptr(), c"rt766_sdca_pde_event".as_ptr(), func_num);
        return -EINVAL;
    }

    match event {
        SND_SOC_DAPM_POST_PMU => {
            regmap_write((*rt766).regmap, pde_req_reg, ps0 as c_uint);
            from_ps = ps3 as c_int;
            to_ps = ps0 as c_int;
        }
        SND_SOC_DAPM_PRE_PMD => {
            regmap_write((*rt766).regmap, pde_req_reg, ps3 as c_uint);
            from_ps = ps0 as c_int;
            to_ps = ps3 as c_int;
        }
        _ => {
            from_ps = 0;
            to_ps = 0;
        }
    }

    let entity = sdca_find_entity_by_label(func_data, pde_ent);
    if entity.is_null() {
        dev_err((*component).dev, c"%s: failed to find entity %s\n".as_ptr(), c"rt766_sdca_pde_event".as_ptr(), pde_ent);
        return -EINVAL;
    }
    let ret = sdca_asoc_pde_poll_actual_ps((*rt766).regmap, func_num, pde_num, from_ps, to_ps, (*entity).pde.max_delay, (*entity).pde.num_max_delay);
    if ret != 0 {
        dev_err((*component).dev, c"%s: PDE transition %x -> %x failed, err=%d\n".as_ptr(), c"rt766_sdca_pde_event".as_ptr(), from_ps, to_ps, ret);
    }
    ret
}

unsafe extern "C" fn rt766_sdca_pde47_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    rt766_sdca_pde_event(w, kcontrol, event, RT766_FUNC_NUM_UAJ, RT766_SDCA_ENT_PDE47, c"PDE 47".as_ptr())
}
unsafe extern "C" fn rt766_sdca_fu36_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt766 = snd_soc_component_get_drvdata(component);
    match event {
        SND_SOC_DAPM_POST_PMU => { (*rt766).fu36_dapm_mute = false; rt766_sdca_set_fu_ctl(rt766, RT766_FUNC_NUM_UAJ, RT766_SDCA_ENT_USER_FU36); }
        SND_SOC_DAPM_PRE_PMD => { (*rt766).fu36_dapm_mute = true; rt766_sdca_set_fu_ctl(rt766, RT766_FUNC_NUM_UAJ, RT766_SDCA_ENT_USER_FU36); }
        _ => {}
    }
    0
}
unsafe extern "C" fn rt766_sdca_pde34_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    rt766_sdca_pde_event(w, kcontrol, event, RT766_FUNC_NUM_UAJ, RT766_SDCA_ENT_PDE34, c"PDE 34".as_ptr())
}
unsafe extern "C" fn rt766_sdca_fu21_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt766 = snd_soc_component_get_drvdata(component);
    match event {
        SND_SOC_DAPM_POST_PMU => { (*rt766).fu21_dapm_mute = false; rt766_sdca_set_fu_ctl(rt766, RT766_FUNC_NUM_AMP, RT766_SDCA_ENT_USER_FU21); }
        SND_SOC_DAPM_PRE_PMD => { (*rt766).fu21_dapm_mute = true; rt766_sdca_set_fu_ctl(rt766, RT766_FUNC_NUM_AMP, RT766_SDCA_ENT_USER_FU21); }
        _ => {}
    }
    0
}
unsafe extern "C" fn rt766_sdca_pde23_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    rt766_sdca_pde_event(w, kcontrol, event, RT766_FUNC_NUM_AMP, RT766_SDCA_ENT_PDE23, c"PDE 23".as_ptr())
}
unsafe extern "C" fn rt766_sdca_pde11_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    rt766_sdca_pde_event(w, kcontrol, event, RT766_FUNC_NUM_MIC, RT766_SDCA_ENT_PDE11, c"PDE 11".as_ptr())
}

unsafe extern "C" fn rt766_dmic_fu_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let p = (*kcontrol).private_value as *mut rt_sdca_dmic_kctrl_priv;
    (*uinfo).type_ = if (*p).max == 1 { SNDRV_CTL_ELEM_TYPE_BOOLEAN } else { SNDRV_CTL_ELEM_TYPE_INTEGER };
    (*uinfo).count = (*p).count;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = (*p).max as i64;
    0
}

static rt766_rx_data_ch_select: [*const c_char; 9] = [
    c"L,R".as_ptr(),
    c"R,L".as_ptr(),
    c"L,L".as_ptr(),
    c"R,R".as_ptr(),
    c"L,L+R".as_ptr(),
    c"R,L+R".as_ptr(),
    c"L+R,L".as_ptr(),
    c"L+R,R".as_ptr(),
    c"L+R,L+R".as_ptr(),
];

/* static SOC_ENUM_SINGLE_DECL(rt766_rx_data_ch_enum,
 *     RT766_SDCA_CTL(AMP, PPU21, SDCA_CTL_PPU_POSTURENUMBER), 0,
 *     rt766_rx_data_ch_select);
 * static const DECLARE_TLV_DB_SCALE(hp_vol_tlv, -9525, 75, 0);
 * static const DECLARE_TLV_DB_SCALE(spk_vol_tlv, -6525, 75, 0);
 * static const DECLARE_TLV_DB_SCALE(mic_vol_tlv, -1725, 75, 0);
 * static const DECLARE_TLV_DB_SCALE(boost_vol_tlv, -200, 200, 0);
 */

/* The following control and widget tables preserve the original macro-defined
 * ALSA data declarations. Their concrete Rust representation depends on the
 * external ASoC/SDCA Rust bindings for these C macros.
 *
 * static const struct snd_kcontrol_new rt766_sdca_controls[] = { ... };
 * static const struct snd_soc_dapm_widget rt766_sdca_dapm_widgets[] = { ... };
 */
static rt766_sdca_controls: [snd_kcontrol_new; 0] = [];
static rt766_sdca_dapm_widgets: [snd_soc_dapm_widget_desc; 0] = [];

static rt766_sdca_audio_map: [snd_soc_dapm_route; 14] = [
    snd_soc_dapm_route { sink: c"FU 41".as_ptr(), control: ptr::null(), source: c"DP3RX".as_ptr() },
    snd_soc_dapm_route { sink: c"DP12TX".as_ptr(), control: ptr::null(), source: c"FU 36".as_ptr() },
    snd_soc_dapm_route { sink: c"FU 36".as_ptr(), control: ptr::null(), source: c"PDE 34".as_ptr() },
    snd_soc_dapm_route { sink: c"FU 36".as_ptr(), control: ptr::null(), source: c"MIC2".as_ptr() },
    snd_soc_dapm_route { sink: c"HP".as_ptr(), control: ptr::null(), source: c"PDE 47".as_ptr() },
    snd_soc_dapm_route { sink: c"HP".as_ptr(), control: ptr::null(), source: c"FU 41".as_ptr() },
    snd_soc_dapm_route { sink: c"FU 21".as_ptr(), control: ptr::null(), source: c"DP1RX".as_ptr() },
    snd_soc_dapm_route { sink: c"FU 21".as_ptr(), control: ptr::null(), source: c"PDE 23".as_ptr() },
    snd_soc_dapm_route { sink: c"SPOL".as_ptr(), control: ptr::null(), source: c"FU 21".as_ptr() },
    snd_soc_dapm_route { sink: c"SPOR".as_ptr(), control: ptr::null(), source: c"FU 21".as_ptr() },
    snd_soc_dapm_route { sink: c"DP8TX".as_ptr(), control: ptr::null(), source: c"FU 113".as_ptr() },
    snd_soc_dapm_route { sink: c"FU 113".as_ptr(), control: ptr::null(), source: c"PDE 11".as_ptr() },
    snd_soc_dapm_route { sink: c"FU 113".as_ptr(), control: ptr::null(), source: c"DMIC1".as_ptr() },
    snd_soc_dapm_route { sink: c"FU 113".as_ptr(), control: ptr::null(), source: c"DMIC2".as_ptr() },
];

unsafe extern "C" fn rt766_sdca_probe(component: *mut snd_soc_component) -> c_int {
    let rt766 = snd_soc_component_get_drvdata(component);
    let dev = &mut (*(*rt766).slave).dev as *mut device;
    (*rt766).component = component;
    let mut ret = pm_runtime_resume((*component).dev);
    if ret < 0 && ret != -EACCES {
        return ret;
    }
    if !(*rt766).uaj_func_data.is_null() {
        dev_dbg(dev, c"%s : irq %d\n".as_ptr(), c"rt766_sdca_probe".as_ptr(), (*(*rt766).slave).irq);
        (*rt766).irq_info = devm_sdca_irq_allocate(dev, (*rt766).regmap, (*(*rt766).slave).irq);
        if IS_ERR((*rt766).irq_info as *const c_void) {
            return PTR_ERR((*rt766).irq_info as *const c_void);
        }
        ret = rt766_sdca_irq_ctl(rt766, (*rt766).uaj_func_data, component, (*rt766).irq_info, true);
        if ret < 0 {
            dev_err(dev, c"Failed to request UAJ SDCA IRQ: %d\n".as_ptr(), ret);
            return ret;
        }
        if !(*rt766).hid_func_data.is_null() {
            ret = rt766_sdca_irq_ctl(rt766, (*rt766).hid_func_data, component, (*rt766).irq_info, true);
            if ret < 0 {
                dev_err(dev, c"Failed to request HID SDCA IRQ: %d\n".as_ptr(), ret);
                return ret;
            }
        }
    }
    0
}

unsafe extern "C" fn rt766_sdca_remove(component: *mut snd_soc_component) {
    let rt766 = snd_soc_component_get_drvdata(component);
    sdca_irq_cleanup_late((*component).dev, (*rt766).uaj_func_data, (*rt766).irq_info);
    sdca_irq_cleanup_late((*component).dev, (*rt766).hid_func_data, (*rt766).irq_info);
}

static soc_sdca_dev_rt766: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rt766_sdca_probe),
    remove: Some(rt766_sdca_remove),
    controls: rt766_sdca_controls.as_ptr(),
    num_controls: rt766_sdca_controls.len() as c_uint,
    dapm_widgets: rt766_sdca_dapm_widgets.as_ptr(),
    num_dapm_widgets: rt766_sdca_dapm_widgets.len() as c_uint,
    dapm_routes: rt766_sdca_audio_map.as_ptr(),
    num_dapm_routes: rt766_sdca_audio_map.len() as c_uint,
    set_jack: Some(rt766_sdca_set_jack_detect),
    endianness: 1,
};

unsafe extern "C" fn rt766_sdca_set_sdw_stream(dai: *mut snd_soc_dai, sdw_stream: *mut c_void, direction: c_int) -> c_int {
    snd_soc_dai_dma_data_set(dai, direction, sdw_stream);
    0
}

unsafe extern "C" fn rt766_sdca_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    snd_soc_dai_set_dma_data(dai, substream, ptr::null_mut());
}

unsafe extern "C" fn rt766_sdca_pcm_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let rt766 = snd_soc_component_get_drvdata(component);
    let mut stream_config: sdw_stream_config = core::mem::zeroed();
    let mut port_config: sdw_port_config = core::mem::zeroed();
    let direction: sdw_data_direction;
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    let sampling_rate: c_uint;
    let port: c_int;

    dev_dbg((*dai).dev, c"%s %s id %d".as_ptr(), c"rt766_sdca_pcm_hw_params".as_ptr(), (*dai).name, (*dai).id);
    if sdw_stream.is_null() {
        return -EINVAL;
    }
    if (*rt766).slave.is_null() {
        return -EINVAL;
    }

    /* SoundWire specific configuration */
    snd_sdw_params_to_config(substream, params, &mut stream_config, &mut port_config);

    /* SoundWire specific configuration */
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        direction = SDW_DATA_DIR_RX;
        if (*dai).id == RT766_AIF1 {
            port = 3;
        } else if (*dai).id == RT766_AIF2 {
            port = 1;
        } else {
            return -EINVAL;
        }
    } else {
        direction = SDW_DATA_DIR_TX;
        if (*dai).id == RT766_AIF1 {
            port = 12;
        } else if (*dai).id == RT766_AIF3 {
            port = 8;
        } else {
            return -EINVAL;
        }
    }
    let _ = direction;

    port_config.num = port;
    let retval = sdw_stream_add_slave((*rt766).slave, &mut stream_config, &mut port_config, 1, sdw_stream);
    if retval != 0 {
        dev_err((*dai).dev, c"%s: Unable to configure port\n".as_ptr(), c"rt766_sdca_pcm_hw_params".as_ptr());
        return retval;
    }

    if params_channels(params) > 16 {
        dev_err((*component).dev, c"%s: Unsupported channels %d\n".as_ptr(), c"rt766_sdca_pcm_hw_params".as_ptr(), params_channels(params));
        return -EINVAL;
    }

    /* sampling rate configuration */
    sampling_rate = match params_rate(params) {
        44100 => RT766_SDCA_RATE_44100HZ,
        48000 => RT766_SDCA_RATE_48000HZ,
        96000 => RT766_SDCA_RATE_96000HZ,
        192000 => RT766_SDCA_RATE_192000HZ,
        _ => {
            dev_err((*component).dev, c"%s: Rate %d is not supported\n".as_ptr(), c"rt766_sdca_pcm_hw_params".as_ptr(), params_rate(params));
            return -EINVAL;
        }
    };

    /* set sampling frequency */
    if (*dai).id == RT766_AIF1 {
        regmap_write((*rt766).regmap, RT766_SDCA_CTL(UAJ, CS41, SDCA_CTL_CS_SAMPLERATEINDEX), sampling_rate);
        regmap_write((*rt766).regmap, RT766_SDCA_CTL(UAJ, CS36, SDCA_CTL_CS_SAMPLERATEINDEX), sampling_rate);
    } else if (*dai).id == RT766_AIF2 {
        regmap_write((*rt766).regmap, RT766_SDCA_CTL(AMP, CS21, SDCA_CTL_CS_SAMPLERATEINDEX), sampling_rate);
    } else if (*dai).id == RT766_AIF3 {
        regmap_write((*rt766).regmap, RT766_SDCA_CTL(MIC, CS113, SDCA_CTL_CS_SAMPLERATEINDEX), sampling_rate);
    } else {
        dev_err((*component).dev, c"%s: Wrong DAI id\n".as_ptr(), c"rt766_sdca_pcm_hw_params".as_ptr());
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn rt766_sdca_pcm_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let rt766 = snd_soc_component_get_drvdata(component);
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    if (*rt766).slave.is_null() {
        return -EINVAL;
    }
    sdw_stream_remove_slave((*rt766).slave, sdw_stream);
    0
}

static rt766_sdca_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(rt766_sdca_pcm_hw_params),
    hw_free: Some(rt766_sdca_pcm_hw_free),
    set_stream: Some(rt766_sdca_set_sdw_stream),
    shutdown: Some(rt766_sdca_shutdown),
};

static mut rt766_sdca_dai: [snd_soc_dai_driver; 3] = [
    snd_soc_dai_driver {
        name: c"rt766-sdca-aif1".as_ptr(),
        id: 0,
        playback: snd_soc_pcm_stream { stream_name: c"DP3 Playback".as_ptr(), channels_min: 1, channels_max: 2, rates: RT766_STEREO_RATES, formats: RT766_DAC_FORMATS },
        capture: snd_soc_pcm_stream { stream_name: c"DP12 Capture".as_ptr(), channels_min: 1, channels_max: 2, rates: RT766_STEREO_RATES, formats: RT766_ADC_FORMATS },
        ops: &rt766_sdca_ops,
        symmetric_rate: 1,
    },
    snd_soc_dai_driver {
        name: c"rt766-sdca-aif2".as_ptr(),
        id: 0,
        playback: snd_soc_pcm_stream { stream_name: c"DP1 Playback".as_ptr(), channels_min: 1, channels_max: 4, rates: RT766_STEREO_RATES, formats: RT766_DAC_FORMATS },
        capture: snd_soc_pcm_stream { stream_name: ptr::null(), channels_min: 0, channels_max: 0, rates: 0, formats: 0 },
        ops: &rt766_sdca_ops,
        symmetric_rate: 0,
    },
    snd_soc_dai_driver {
        name: c"rt766-sdca-aif3".as_ptr(),
        id: 0,
        playback: snd_soc_pcm_stream { stream_name: ptr::null(), channels_min: 0, channels_max: 0, rates: 0, formats: 0 },
        capture: snd_soc_pcm_stream { stream_name: c"DP8 Capture".as_ptr(), channels_min: 1, channels_max: 4, rates: RT766_STEREO_RATES, formats: RT766_ADC_FORMATS },
        ops: &rt766_sdca_ops,
        symmetric_rate: 0,
    },
];

unsafe extern "C" fn rt766_find_dt_rates(dev: *mut device, function: *mut sdca_function_data, label: *const c_char) -> c_uint {
    let mut stream: snd_soc_pcm_stream = core::mem::zeroed();
    let mut i: c_int = 0;
    while i < (*function).num_entities {
        let entity = (*function).entities.add(i as usize);
        if strcmp((*entity).label, label) != 0 {
            i += 1;
            continue;
        }
        /* Can't check earlier as only terminals have an iot member. */
        if !(*entity).iot.is_dataport {
            i += 1;
            continue;
        }
        let ret = sdca_asoc_populate_rate_format(dev, function, entity, &mut stream);
        if ret < 0 {
            dev_dbg(dev, c"%s: failed to parse rates for entity %s\n".as_ptr(), c"rt766_find_dt_rates".as_ptr(), (*entity).label);
            return 0;
        }
        dev_dbg(dev, c"%s: %s supports rates 0x%08x\n".as_ptr(), c"rt766_find_dt_rates".as_ptr(), (*entity).label, stream.rates);
        i += 1;
    }
    stream.rates
}

#[no_mangle]
pub unsafe extern "C" fn rt766_sdca_init(dev: *mut device, regmap: *mut regmap, slave: *mut sdw_slave) -> c_int {
    let rt766 = devm_kzalloc(dev, size_of::<rt766_sdca_priv>(), GFP_KERNEL) as *mut rt766_sdca_priv;
    if rt766.is_null() {
        return -ENOMEM;
    }
    dev_set_drvdata(dev, rt766 as *mut c_void);
    (*rt766).slave = slave;
    (*rt766).regmap = regmap;

    regcache_cache_only((*rt766).regmap, true);
    let mut ret = devm_mutex_init(dev, &mut (*rt766).disable_irq_lock);
    if ret < 0 {
        dev_err(dev, c"Failed to initialize mutex\n".as_ptr());
        return ret;
    }

    /*
     * Mark hw_init to false
     * HW init will be performed when device reports present
     */
    (*rt766).hw_init = false;
    (*rt766).first_hw_init = false;
    (*rt766).fu41_dapm_mute = true;
    (*rt766).fu41_mixer_l_mute = false;
    (*rt766).fu41_mixer_r_mute = false;
    (*rt766).fu36_dapm_mute = true;
    (*rt766).fu36_mixer_l_mute = true;
    (*rt766).fu36_mixer_r_mute = true;
    (*rt766).fu21_dapm_mute = true;
    (*rt766).fu21_mixer_l_mute = false;
    (*rt766).fu21_mixer_r_mute = false;
    (*rt766).fu113_dapm_mute = true;
    (*rt766).fu113_mixer_mute = [true; 4];

    let dai_drv = devm_kzalloc(dev, size_of::<snd_soc_dai_driver>() * 3, GFP_KERNEL) as *mut snd_soc_dai_driver;
    if dai_drv.is_null() {
        dev_err(dev, c"Failed to allocate memory for DAI driver\n".as_ptr());
        return -ENOMEM;
    }
    memcpy(dai_drv as *mut c_void, rt766_sdca_dai.as_ptr() as *const c_void, size_of::<snd_soc_dai_driver>() * 3);

    /* get SDCA function data */
    dev_dbg(dev, c"SDCA functions found: %d".as_ptr(), (*slave).sdca_data.num_functions);
    let mut i: c_int = 0;
    while i < (*slave).sdca_data.num_functions {
        let func_data_ptr = devm_kzalloc(dev, size_of::<sdca_function_data>(), GFP_KERNEL) as *mut sdca_function_data;
        if func_data_ptr.is_null() {
            dev_err(dev, c"Failed to allocate memory for function data\n".as_ptr());
            devm_kfree(dev, dai_drv as *mut c_void);
            return -ENOMEM;
        }
        (*func_data_ptr).desc = (*slave).sdca_data.function.add(i as usize);
        ret = sdca_parse_function(dev, func_data_ptr);
        if ret != 0 {
            devm_kfree(dev, func_data_ptr as *mut c_void);
            devm_kfree(dev, dai_drv as *mut c_void);
            return ret;
        }
        dev_dbg(dev, c"Function type=%d, num_entities=%d".as_ptr(), (*(*slave).sdca_data.function.add(i as usize)).type_, (*func_data_ptr).num_entities);

        if (*(*slave).sdca_data.function.add(i as usize)).type_ == SDCA_FUNCTION_TYPE_UAJ {
            (*rt766).uaj_func_data = func_data_ptr;
            /*
             * Some machines may only support a subset of the sample rates supported by the codec.
             * Therefore, we need to parse the supported sample rates from the DisCo table and
             * configure them in the DAI. If the DisCo table does not provide sample rate information,
             * we will fall back to the default supported rates defined in the codec driver.
             */
            let mut rates = rt766_find_dt_rates(dev, func_data_ptr, c"IT 41".as_ptr());
            if rates != 0 { (*dai_drv.add(RT766_DAI_UAJ)).playback.rates = rates; }
            rates = rt766_find_dt_rates(dev, func_data_ptr, c"OT 36".as_ptr());
            if rates != 0 { (*dai_drv.add(RT766_DAI_UAJ)).capture.rates = rates; }
        } else if (*(*slave).sdca_data.function.add(i as usize)).type_ == SDCA_FUNCTION_TYPE_SMART_AMP {
            (*rt766).sa_func_data = func_data_ptr;
            let rates = rt766_find_dt_rates(dev, func_data_ptr, c"IT 21".as_ptr());
            if rates != 0 { (*dai_drv.add(RT766_DAI_AMP)).playback.rates = rates; }
        } else if (*(*slave).sdca_data.function.add(i as usize)).type_ == SDCA_FUNCTION_TYPE_SMART_MIC {
            (*rt766).sm_func_data = func_data_ptr;
            let rates = rt766_find_dt_rates(dev, func_data_ptr, c"OT 113".as_ptr());
            if rates != 0 { (*dai_drv.add(RT766_DAI_MIC)).capture.rates = rates; }
        } else if (*(*slave).sdca_data.function.add(i as usize)).type_ == SDCA_FUNCTION_TYPE_HID {
            (*rt766).hid_func_data = func_data_ptr;
        } else {
            dev_dbg(dev, c"Unexpected SDCA function type found: %d".as_ptr(), (*(*slave).sdca_data.function.add(i as usize)).type_);
        }
        i += 1;
    }

    ret = devm_snd_soc_register_component(dev, &soc_sdca_dev_rt766, dai_drv, 3);
    if ret < 0 {
        devm_kfree(dev, dai_drv as *mut c_void);
        return ret;
    }

    /* set autosuspend parameters */
    pm_runtime_set_autosuspend_delay(dev, 3000);
    pm_runtime_use_autosuspend(dev);
    /* make sure the device does not suspend immediately */
    pm_runtime_mark_last_busy(dev);
    pm_runtime_enable(dev);
    dev_dbg(dev, c"%s\n".as_ptr(), c"rt766_sdca_init".as_ptr());
    0
}

unsafe extern "C" fn rt766_func_initialize(rt766: *mut rt766_sdca_priv, func_data: *mut sdca_function_data) -> c_int {
    let dev = &mut (*(*rt766).slave).dev as *mut device;
    let func_status_reg: c_uint;
    let mut func_status: c_uint = 0;

    if (*(*func_data).desc).type_ == SDCA_FUNCTION_TYPE_UAJ {
        func_status_reg = RT766_FUNC_STATUS_REG(UAJ);
    } else if (*(*func_data).desc).type_ == SDCA_FUNCTION_TYPE_SMART_AMP {
        func_status_reg = RT766_FUNC_STATUS_REG(AMP);
    } else if (*(*func_data).desc).type_ == SDCA_FUNCTION_TYPE_SMART_MIC {
        func_status_reg = RT766_FUNC_STATUS_REG(MIC);
    } else if (*(*func_data).desc).type_ == SDCA_FUNCTION_TYPE_HID {
        func_status_reg = RT766_FUNC_STATUS_REG(HID);
    } else {
        dev_dbg(dev, c"Unexpected SDCA function type found: %d".as_ptr(), (*(*func_data).desc).type_);
        return -EINVAL;
    }

    regmap_read((*rt766).regmap, func_status_reg, &mut func_status);
    dev_dbg(dev, c"%s, %s func_status=0x%x\n".as_ptr(), c"rt766_func_initialize".as_ptr(), (*(*func_data).desc).name, func_status);
    if (func_status & SDCA_CTL_ENTITY_0_FUNCTION_NEEDS_INITIALIZATION) != 0 || !(*rt766).first_hw_init {
        let ret = sdca_regmap_write_init(dev, (*rt766).regmap, func_data);
        if ret != 0 {
            dev_err(dev, c"%s initialization table update failed\n".as_ptr(), (*(*func_data).desc).name);
            dev_err(dev, c"%s: %s init writes failed, err=%d".as_ptr(), c"rt766_func_initialize".as_ptr(), (*(*func_data).desc).name, ret);
            return ret;
        }
        regmap_write((*rt766).regmap, func_status_reg, SDCA_CTL_ENTITY_0_FUNCTION_NEEDS_INITIALIZATION);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn rt766_sdca_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int {
    let rt766 = dev_get_drvdata(dev);
    let mut val: c_uint = 0;

    (*rt766).disable_irq = false;
    if (*rt766).hw_init {
        return 0;
    }

    regcache_cache_only((*rt766).regmap, false);
    if (*rt766).first_hw_init {
        regcache_cache_bypass((*rt766).regmap, true);
    } else {
        /*
         *  PM runtime status is marked as 'active' only when a Slave reports as Attached
         */
        /* update count of parent 'active' children */
        pm_runtime_set_active(&mut (*slave).dev);
    }

    pm_runtime_get_noresume(&mut (*slave).dev);
    regmap_read((*rt766).regmap, RT766_BOND_LATCH_ID, &mut val);
    dev_dbg(&mut (*slave).dev, c"%s bond ID=0x%x (%s)\n".as_ptr(), c"rt766_sdca_io_init".as_ptr(), val, if val == 0x1 { c"RT767".as_ptr() } else { c"RT766".as_ptr() });

    /* check function status and initialize if needed */
    if !(*rt766).uaj_func_data.is_null() { rt766_func_initialize(rt766, (*rt766).uaj_func_data); }
    if !(*rt766).sa_func_data.is_null() { rt766_func_initialize(rt766, (*rt766).sa_func_data); }
    if !(*rt766).sm_func_data.is_null() { rt766_func_initialize(rt766, (*rt766).sm_func_data); }
    if !(*rt766).hid_func_data.is_null() { rt766_func_initialize(rt766, (*rt766).hid_func_data); }

    if (*rt766).first_hw_init {
        regcache_cache_bypass((*rt766).regmap, false);
        regcache_mark_dirty((*rt766).regmap);
    } else {
        (*rt766).first_hw_init = true;
    }

    /* Mark Slave initialization complete */
    (*rt766).hw_init = true;
    dev_dbg(&mut (*slave).dev, c"%s hw_init complete\n".as_ptr(), c"rt766_sdca_io_init".as_ptr());
    pm_runtime_put_autosuspend(&mut (*slave).dev);
    0
}

/* MODULE_DESCRIPTION("ASoC RT766 SDCA SDW driver");
 * MODULE_AUTHOR("Shuming Fan <shumingf@realtek.com>");
 * MODULE_LICENSE("GPL");
 * MODULE_IMPORT_NS("SND_SOC_SDCA");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
