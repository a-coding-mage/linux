// SPDX-License-Identifier: GPL-2.0-only
//
// rt722-sdca.rs -- rt722 SDCA ALSA SoC audio driver
//
// Copyright(c) 2023 Realtek Semiconductor Corp.
//
// Translated from rt722-sdca.c. C include dependencies are intentionally left
// as external Rust items/macros expected from the surrounding kernel bindings.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct sdw_slave { pub dev: device }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_jack { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_card { pub instantiated: bool }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device, pub card: *mut snd_soc_card }
#[repr(C)] pub struct snd_kcontrol { pub private_value: c_ulong }
#[repr(C)] pub struct snd_ctl_elem_value { pub id: snd_ctl_elem_id, pub value: snd_ctl_elem_value_union }
#[repr(C)] pub struct snd_ctl_elem_id { pub name: [c_char; 44] }
#[repr(C)] pub union snd_ctl_elem_value_union { pub integer: snd_ctl_elem_value_integer }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_integer { pub value: [c_long; 128] }
type c_long = isize;
#[repr(C)] pub struct snd_ctl_elem_info { pub type_: c_uint, pub count: c_uint, pub value: snd_ctl_elem_info_value }
#[repr(C)] pub union snd_ctl_elem_info_value { pub integer: snd_ctl_elem_info_integer }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_info_integer { pub min: c_long, pub max: c_long }
#[repr(C)] pub struct soc_mixer_control { pub reg: c_uint, pub rreg: c_uint, pub shift: c_uint, pub max: c_uint }
#[repr(C)] pub struct rt722_sdca_dmic_kctrl_priv { pub reg_base: c_uint, pub count: c_uint, pub max: c_uint, pub invert: c_uint }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context }
#[repr(C)] pub struct snd_pcm_substream { pub stream: c_int }
#[repr(C)] pub struct snd_soc_dai { pub component: *mut snd_soc_component, pub dev: *mut device, pub name: *const c_char, pub id: c_int }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct sdw_stream_config { pub frame_rate: c_uint, pub ch_count: c_uint, pub bps: c_int, pub direction: sdw_data_direction }
#[repr(C)] pub struct sdw_port_config { pub num: c_int, pub ch_mask: c_uint }
#[repr(C)] pub struct sdw_stream_runtime { _private: [u8; 0] }
#[repr(C)] pub struct firmware { pub size: usize, pub data: *const u8 }
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route { pub sink: *const c_char, pub control: *const c_char, pub source: *const c_char }
#[repr(C)] pub struct snd_soc_dapm_widget_desc { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai_ops { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai_driver { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_component_driver { _private: [u8; 0] }
#[repr(C)] pub enum sdw_data_direction { SDW_DATA_DIR_RX = 0, SDW_DATA_DIR_TX = 1 }

#[repr(C)]
pub struct rt722_sdca_priv {
    pub regmap: *mut regmap,
    pub slave: *mut sdw_slave,
    pub component: *mut snd_soc_component,
    pub hs_jack: *mut snd_soc_jack,
    pub scp_sdca_stat1: c_uint,
    pub scp_sdca_stat2: c_uint,
    pub jack_type: c_int,
    pub jack_detect_work: delayed_work,
    pub jack_btn_check_work: delayed_work,
    pub calibrate_mutex: mutex,
    pub disable_irq_lock: mutex,
    pub jd_src: c_uint,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub disable_irq: bool,
    pub hw_vid: c_uint,
    pub fu1e_dapm_mute: bool,
    pub fu0f_dapm_mute: bool,
    pub fu06_dapm_mute: bool,
    pub fu06_mixer_l_mute: bool,
    pub fu06_mixer_r_mute: bool,
    pub fu0f_mixer_l_mute: bool,
    pub fu0f_mixer_r_mute: bool,
    pub fu1e_mixer_mute: [bool; 4],
    pub cae_update_done: c_int,
}
#[repr(C)] pub struct delayed_work { pub work: work_struct }
#[repr(C)] pub struct mutex { _private: [u8; 0] }

extern "C" {
    static system_power_efficient_wq: *mut c_void;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regcache_cache_bypass(map: *mut regmap, enable: bool);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err_ratelimited(fmt: *const c_char, ...);
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn mod_delayed_work(wq: *mut c_void, dwork: *mut delayed_work, delay: c_ulong) -> bool;
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn sdw_write_no_pm(slave: *mut sdw_slave, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut rt722_sdca_priv;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_resume(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn dmi_get_system_info(field: c_int) -> *const c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn kasprintf(gfp: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn kfree(p: *mut c_void);
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> c_int;
    fn release_firmware(fw: *const firmware);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn msleep(ms: c_uint);
    fn device_property_read_u32(dev: *mut device, prop: *const c_char, val: *mut c_uint) -> c_int;
    fn snd_soc_dai_dma_data_set(dai: *mut snd_soc_dai, direction: c_int, data: *mut c_void);
    fn snd_soc_dai_set_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream, data: *mut c_void);
    fn snd_soc_dai_get_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream) -> *mut sdw_stream_runtime;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn sdw_stream_add_slave(slave: *mut sdw_slave, stream_config: *mut sdw_stream_config, port_config: *mut sdw_port_config, num_ports: c_int, stream: *mut sdw_stream_runtime) -> c_int;
    fn sdw_stream_remove_slave(slave: *mut sdw_slave, stream: *mut sdw_stream_runtime);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn mutex_init(m: *mut mutex);
    fn INIT_DELAYED_WORK(work: *mut delayed_work, func: unsafe extern "C" fn(*mut work_struct));
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: usize) -> c_int;
}

extern "C" {
    static soc_sdca_dev_rt722: snd_soc_component_driver;
    static mut rt722_sdca_dai: [snd_soc_dai_driver; 3];
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const ETIMEDOUT: c_int = 110;
const EIO: c_int = 5;
const EACCES: c_int = 13;
const GFP_KERNEL: c_uint = 0;

macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const c_char }; }
const fn BIT(n: c_uint) -> c_uint { 1u32 << n }
const fn GENMASK(h: c_int, l: c_int) -> c_uint { (!0u32 << l) & (!0u32 >> (31 - h)) }
const fn RT722_NID_ADDR(nid: c_uint, reg: c_uint) -> c_uint { (nid << 20) | reg }
const fn SDW_SDCA_CTL(func: c_uint, entity: c_uint, ctl: c_uint, ch: c_uint) -> c_uint {
    ((func) << 20) | ((entity) << 12) | ((ctl) << 4) | ch
}

unsafe fn set_mask_bits(p: *mut c_uint, mask: c_uint, val: c_uint) {
    *p = (*p & !mask) | (val & mask);
}

unsafe fn dev_of_slave(rt722: *mut rt722_sdca_priv) -> *mut device {
    &mut (*(*rt722).slave).dev
}

/* External constants supplied by the original headers. */
extern "C" {
    static SND_JACK_BTN_0: c_int; static SND_JACK_BTN_1: c_int; static SND_JACK_BTN_2: c_int; static SND_JACK_BTN_3: c_int;
    static SND_JACK_HEADPHONE: c_int; static SND_JACK_HEADSET: c_int;
}
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;

/* Header-defined numeric constants are referenced by name through const externs. */
extern "C" {
    static FUNC_NUM_HID: c_uint; static FUNC_NUM_JACK_CODEC: c_uint; static FUNC_NUM_AMP: c_uint; static FUNC_NUM_MIC_ARRAY: c_uint;
    static RT722_SDCA_ENT_HID01: c_uint; static RT722_SDCA_ENT_GE49: c_uint; static RT722_SDCA_ENT_XU03: c_uint; static RT722_SDCA_ENT_XU0D: c_uint;
    static RT722_SDCA_CTL_HIDTX_CURRENT_OWNER: c_uint; static RT722_SDCA_CTL_HIDTX_MESSAGE_OFFSET: c_uint; static RT722_BUF_ADDR_HID1: c_uint;
    static RT722_SDCA_CTL_DETECTED_MODE: c_uint; static RT722_SDCA_CTL_SELECTED_MODE: c_uint;
    static RT722_VENDOR_HDA_CTL: c_uint; static RT722_HDA_LEGACY_UNSOL_CTL: c_uint; static RT722_GE_RELATED_CTL1: c_uint; static RT722_GE_RELATED_CTL2: c_uint;
    static SDW_SCP_SDCA_INT_SDCA_0: c_uint; static SDW_SCP_SDCA_INT_SDCA_8: c_uint; static SDW_SCP_SDCA_INTMASK1: c_uint; static SDW_SCP_SDCA_INTMASK2: c_uint;
    static SDW_SCP_SDCA_INTMASK_SDCA_0: c_uint; static SDW_SCP_SDCA_INTMASK_SDCA_8: c_uint;
    static RT722_SPK_CAE_PARAM1: c_uint; static RT722_HP_CAE_PARAM39: c_uint; static RT722_MIC_CAE_PARAM39: c_uint;
    static RT722_SPK_CAE_PARAM38: c_uint; static RT722_HP_CAE_PARAM68: c_uint; static RT722_MIC_CAE_PARAM99: c_uint;
    static RT722_VENDOR_EQ_CAE: c_uint; static RT722_EQ_CTRL_SPK: c_uint; static RT722_EQ_CTRL_HP: c_uint; static RT722_EQ_CTRL_DMIC: c_uint; static RT722_EQ_CTRL_AMIC: c_uint;
    static RT722_VENDOR_REG: c_uint; static RT722_MISC_CTRL1: c_uint; static RT722_SPK_CAE_PARAM34: c_uint; static RT722_HP_CAE_PARAM64: c_uint; static RT722_MIC_CAE_PARAM95: c_uint;
    static RT722_SPK_CAE_PARAM35: c_uint; static RT722_HP_CAE_PARAM65: c_uint; static RT722_MIC_CAE_PARAM96: c_uint;
    static SND_SOC_BIAS_OFF: c_int; static SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_uint; static SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint;
    static RT722_SDCA_ENT_USER_FU1E: c_uint; static RT722_SDCA_ENT_USER_FU06: c_uint; static RT722_SDCA_ENT_USER_FU0F: c_uint;
    static RT722_SDCA_CTL_FU_MUTE: c_uint; static RT722_SDCA_CTL_FU_VOLUME: c_uint; static RT722_SDCA_CTL_FU_CH_GAIN: c_uint;
    static CH_01: c_uint; static CH_L: c_uint; static CH_R: c_uint; static CH_08: c_uint;
    static RT722_SDCA_CTL_ACTUAL_POWER_STATE: c_uint; static RT722_SDCA_CTL_REQ_POWER_STATE: c_uint;
    static RT722_SDCA_ENT_PDE40: c_uint; static RT722_SDCA_ENT_PDE23: c_uint; static RT722_SDCA_ENT_PDE2A: c_uint; static RT722_SDCA_ENT_PDE12: c_uint;
    static RT722_AIF1: c_int; static RT722_AIF2: c_int; static RT722_AIF3: c_int;
    static RT722_SDCA_RATE_44100HZ: c_uint; static RT722_SDCA_RATE_48000HZ: c_uint; static RT722_SDCA_RATE_96000HZ: c_uint; static RT722_SDCA_RATE_192000HZ: c_uint;
    static RT722_SDCA_ENT_CS01: c_uint; static RT722_SDCA_ENT_CS11: c_uint; static RT722_SDCA_ENT_CS31: c_uint; static RT722_SDCA_ENT_CS1F: c_uint;
    static RT722_SDCA_CTL_SAMPLE_FREQ_INDEX: c_uint; static RT722_SDCA_ENT0: c_uint; static RT722_SDCA_CTL_FUNC_STATUS: c_uint;
    static FUNCTION_NEEDS_INITIALIZATION: c_uint; static RT722_VA: c_uint; static RT722_VB: c_uint;
    static RT722_VENDOR_CALI: c_uint; static RT722_JD_PRODUCT_NUM: c_uint; static RT722_LDO1_CTL: c_uint; static RT722_HIDDEN_REG_SW_RESET: c_uint;
    static RT722_HDA_LEGACY_RESET_CTL: c_uint; static RT722_ADC0A_08_PDE_FLOAT_CTL: c_uint; static RT722_ADC10_PDE_FLOAT_CTL: c_uint;
    static RT722_DMIC1_2_PDE_FLOAT_CTL: c_uint; static RT722_DMIC_ENT_FLOAT_CTL: c_uint; static RT722_ADC_ENT_FLOAT_CTL: c_uint;
    static RT722_DMIC_GAIN_ENT_FLOAT_CTL0: c_uint; static RT722_ADC_VOL_CH_FLOAT_CTL: c_uint; static RT722_DMIC_GAIN_ENT_FLOAT_CTL2: c_uint;
    static RT722_HDA_LEGACY_CONFIG_CTL0: c_uint; static RT722_SDCA_ENT_IT26: c_uint; static RT722_SDCA_CTL_VENDOR_DEF: c_uint;
    static RT722_CLSD_CTRL6: c_uint; static RT722_DC_CALIB_CTRL: c_uint; static RT722_AMP_PDE_FLOAT_CTL: c_uint; static RT722_EAPD_CTL: c_uint;
    static RT722_SDCA_ENT_OT23: c_uint; static RT722_ANALOG_BIAS_CTL3: c_uint; static RT722_UMP_HID_CTL4: c_uint; static RT722_UMP_HID_CTL5: c_uint;
    static RT722_UMP_HID_CTL0: c_uint; static RT722_UMP_HID_CTL7: c_uint; static RT722_JD_CTRL1: c_uint; static RT722_FSM_CTL: c_uint;
    static RT722_DAC_DC_CALI_CTL3: c_uint; static RT722_MIC2_LINE2_PDE_FLOAT_CTL: c_uint; static RT722_ET41_LINE2_PDE_FLOAT_CTL: c_uint;
    static RT722_DAC03_HP_PDE_FLOAT_CTL: c_uint; static RT722_ENT_FLOAT_CTRL_1: c_uint; static RT722_FLOAT_CTRL_1: c_uint;
    static RT722_COMBO_JACK_AUTO_CTL1: c_uint; static RT722_VREFO_GAT: c_uint; static RT722_DIGITAL_MISC_CTRL4: c_uint;
}

pub unsafe extern "C" fn rt722_sdca_index_write(rt722: *mut rt722_sdca_priv, nid: c_uint, reg: c_uint, value: c_uint) -> c_int {
    let regmap = (*rt722).regmap;
    let addr = RT722_NID_ADDR(nid, reg);
    let ret = regmap_write(regmap, addr, value);
    if ret < 0 {
        dev_err(dev_of_slave(rt722), cstr!("%s: Failed to set private value: %06x <= %04x ret=%d\n"), cstr!("rt722_sdca_index_write"), addr, value, ret);
    }
    ret
}

pub unsafe extern "C" fn rt722_sdca_index_read(rt722: *mut rt722_sdca_priv, nid: c_uint, reg: c_uint, value: *mut c_uint) -> c_int {
    let regmap = (*rt722).regmap;
    let addr = RT722_NID_ADDR(nid, reg);
    let ret = regmap_read(regmap, addr, value);
    if ret < 0 {
        dev_err(dev_of_slave(rt722), cstr!("%s: Failed to get private value: %06x => %04x ret=%d\n"), cstr!("rt722_sdca_index_read"), addr, *value, ret);
    }
    ret
}

unsafe fn rt722_sdca_index_update_bits(rt722: *mut rt722_sdca_priv, nid: c_uint, reg: c_uint, mask: c_uint, val: c_uint) -> c_int {
    let mut tmp: c_uint = 0;
    let ret = rt722_sdca_index_read(rt722, nid, reg, &mut tmp);
    if ret < 0 { return ret; }
    set_mask_bits(&mut tmp, mask, val);
    rt722_sdca_index_write(rt722, nid, reg, tmp)
}

unsafe fn rt722_sdca_btn_type(buffer: *mut u8) -> c_int {
    if ((*buffer & 0xf0) == 0x10) || ((*buffer & 0x0f) == 0x01) || (*buffer.add(1) == 0x01) || (*buffer.add(1) == 0x10) {
        SND_JACK_BTN_2
    } else if ((*buffer & 0xf0) == 0x20) || ((*buffer & 0x0f) == 0x02) || (*buffer.add(1) == 0x02) || (*buffer.add(1) == 0x20) {
        SND_JACK_BTN_3
    } else if ((*buffer & 0xf0) == 0x40) || ((*buffer & 0x0f) == 0x04) || (*buffer.add(1) == 0x04) || (*buffer.add(1) == 0x40) {
        SND_JACK_BTN_0
    } else if ((*buffer & 0xf0) == 0x80) || ((*buffer & 0x0f) == 0x08) || (*buffer.add(1) == 0x08) || (*buffer.add(1) == 0x80) {
        SND_JACK_BTN_1
    } else { 0 }
}

unsafe fn rt722_sdca_button_detect(rt722: *mut rt722_sdca_priv) -> c_uint {
    let mut btn_type: c_uint = 0;
    let mut offset: c_uint = 0;
    let mut val: c_uint = 0;
    let mut owner: c_uint = 0;
    let mut buf = [0u8; 3];
    let mut ret = regmap_read((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_HID, RT722_SDCA_ENT_HID01, RT722_SDCA_CTL_HIDTX_CURRENT_OWNER, 0), &mut owner);
    if ret < 0 { return 0; }
    if owner == 1 { return 0; }
    ret = regmap_read((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_HID, RT722_SDCA_ENT_HID01, RT722_SDCA_CTL_HIDTX_MESSAGE_OFFSET, 0), &mut offset);
    if ret >= 0 {
        let mut idx = 0usize;
        while idx < buf.len() {
            ret = regmap_read((*rt722).regmap, RT722_BUF_ADDR_HID1 + offset + idx as c_uint, &mut val);
            if ret < 0 { break; }
            buf[idx] = (val & 0xff) as u8;
            idx += 1;
        }
        if ret >= 0 && buf[0] == 0x11 {
            btn_type = rt722_sdca_btn_type(buf.as_mut_ptr().add(1)) as c_uint;
        }
    }
    if owner == 0 {
        regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_HID, RT722_SDCA_ENT_HID01, RT722_SDCA_CTL_HIDTX_CURRENT_OWNER, 0), 0x01);
    }
    btn_type
}

unsafe fn rt722_sdca_headset_detect(rt722: *mut rt722_sdca_priv) -> c_int {
    let mut det_mode: c_uint = 0;
    let ret = regmap_read((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_GE49, RT722_SDCA_CTL_DETECTED_MODE, 0), &mut det_mode);
    if ret < 0 {
        pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("rt722_sdca_headset_detect"), ret);
        return ret;
    }
    match det_mode {
        0x00 => (*rt722).jack_type = 0,
        0x03 => (*rt722).jack_type = SND_JACK_HEADPHONE,
        0x05 => (*rt722).jack_type = SND_JACK_HEADSET,
        _ => {}
    }
    if det_mode != 0 {
        let ret2 = regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_GE49, RT722_SDCA_CTL_SELECTED_MODE, 0), det_mode);
        if ret2 < 0 {
            pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("rt722_sdca_headset_detect"), ret2);
            return ret2;
        }
    }
    dev_dbg(dev_of_slave(rt722), cstr!("%s, detected_mode=0x%x\n"), cstr!("rt722_sdca_headset_detect"), det_mode);
    0
}

unsafe extern "C" fn rt722_sdca_jack_detect_handler(_work: *mut work_struct) {
    /* container_of(work, struct rt722_sdca_priv, jack_detect_work.work) is supplied by kernel bindings in C.
     * This source-level Rust translation cannot compute the parent without the complete type layout metadata. */
    let rt722: *mut rt722_sdca_priv = container_of_jack_detect_work(_work);
    let mut btn_type: c_int = 0;
    if (*rt722).hs_jack.is_null() { return; }
    if (*rt722).component.is_null() || (*(*rt722).component).card.is_null() || !(*(*(*rt722).component).card).instantiated { return; }
    if ((*rt722).scp_sdca_stat1 & SDW_SCP_SDCA_INT_SDCA_0) != 0 {
        let ret = rt722_sdca_headset_detect(rt722);
        if ret < 0 { return; }
    }
    if ((*rt722).scp_sdca_stat2 & SDW_SCP_SDCA_INT_SDCA_8) != 0 {
        btn_type = rt722_sdca_button_detect(rt722) as c_int;
    }
    if (*rt722).jack_type == 0 { btn_type = 0; }
    dev_dbg(dev_of_slave(rt722), cstr!("in %s, jack_type=%d\n"), cstr!("rt722_sdca_jack_detect_handler"), (*rt722).jack_type);
    dev_dbg(dev_of_slave(rt722), cstr!("in %s, btn_type=0x%x\n"), cstr!("rt722_sdca_jack_detect_handler"), btn_type);
    dev_dbg(dev_of_slave(rt722), cstr!("in %s, scp_sdca_stat1=0x%x, scp_sdca_stat2=0x%x\n"), cstr!("rt722_sdca_jack_detect_handler"), (*rt722).scp_sdca_stat1, (*rt722).scp_sdca_stat2);
    snd_soc_jack_report((*rt722).hs_jack, (*rt722).jack_type | btn_type, SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3);
    if btn_type != 0 {
        snd_soc_jack_report((*rt722).hs_jack, (*rt722).jack_type, SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3);
        mod_delayed_work(system_power_efficient_wq, &mut (*rt722).jack_btn_check_work, msecs_to_jiffies(200));
    }
}

extern "C" { fn container_of_jack_detect_work(work: *mut work_struct) -> *mut rt722_sdca_priv; fn container_of_jack_btn_check_work(work: *mut work_struct) -> *mut rt722_sdca_priv; }

unsafe extern "C" fn rt722_sdca_btn_check_handler(work: *mut work_struct) {
    let rt722 = container_of_jack_btn_check_work(work);
    let mut btn_type: c_int = 0;
    let mut det_mode: c_uint = 0;
    let mut offset: c_uint = 0;
    let mut val: c_uint = 0;
    let mut buf = [0u8; 3];
    let mut ret = regmap_read((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_GE49, RT722_SDCA_CTL_DETECTED_MODE, 0), &mut det_mode);
    if ret < 0 { pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("rt722_sdca_btn_check_handler"), ret); return; }
    if det_mode != 0 {
        ret = regmap_read((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_HID, RT722_SDCA_ENT_HID01, RT722_SDCA_CTL_HIDTX_MESSAGE_OFFSET, 0), &mut offset);
        if ret < 0 { pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("rt722_sdca_btn_check_handler"), ret); return; }
        for idx in 0..buf.len() {
            ret = regmap_read((*rt722).regmap, RT722_BUF_ADDR_HID1 + offset + idx as c_uint, &mut val);
            if ret < 0 { pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("rt722_sdca_btn_check_handler"), ret); return; }
            buf[idx] = (val & 0xff) as u8;
        }
        if buf[0] == 0x11 { btn_type = rt722_sdca_btn_type(buf.as_mut_ptr().add(1)); }
    } else { (*rt722).jack_type = 0; }
    dev_dbg(dev_of_slave(rt722), cstr!("%s, btn_type=0x%x\n"), cstr!("rt722_sdca_btn_check_handler"), btn_type);
    snd_soc_jack_report((*rt722).hs_jack, (*rt722).jack_type | btn_type, SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3);
    if btn_type != 0 {
        snd_soc_jack_report((*rt722).hs_jack, (*rt722).jack_type, SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3);
        mod_delayed_work(system_power_efficient_wq, &mut (*rt722).jack_btn_check_work, msecs_to_jiffies(200));
    }
}

unsafe fn rt722_sdca_jack_init(rt722: *mut rt722_sdca_priv) {
    /* guard(mutex)(&rt722->calibrate_mutex); */
    if !(*rt722).hs_jack.is_null() {
        sdw_write_no_pm((*rt722).slave, SDW_SCP_SDCA_INTMASK1, SDW_SCP_SDCA_INTMASK_SDCA_0);
        sdw_write_no_pm((*rt722).slave, SDW_SCP_SDCA_INTMASK2, SDW_SCP_SDCA_INTMASK_SDCA_8);
        dev_dbg(dev_of_slave(rt722), cstr!("in %s enable\n"), cstr!("rt722_sdca_jack_init"));
        rt722_sdca_index_write(rt722, RT722_VENDOR_HDA_CTL, RT722_HDA_LEGACY_UNSOL_CTL, 0x016E);
        regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_XU03, RT722_SDCA_CTL_SELECTED_MODE, 0), 0);
        regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_XU0D, RT722_SDCA_CTL_SELECTED_MODE, 0), 0);
        rt722_sdca_index_write(rt722, RT722_VENDOR_HDA_CTL, RT722_GE_RELATED_CTL1, 0x0000);
        rt722_sdca_index_update_bits(rt722, RT722_VENDOR_HDA_CTL, RT722_GE_RELATED_CTL2, 0x4000, 0x4000);
    }
}

unsafe extern "C" fn rt722_sdca_set_jack_detect(component: *mut snd_soc_component, hs_jack: *mut snd_soc_jack, _data: *mut c_void) -> c_int {
    let rt722 = snd_soc_component_get_drvdata(component);
    (*rt722).hs_jack = hs_jack;
    let ret = pm_runtime_resume_and_get((*component).dev);
    if ret < 0 {
        if ret != -EACCES {
            dev_err((*component).dev, cstr!("%s: failed to resume %d\n"), cstr!("rt722_sdca_set_jack_detect"), ret);
            return ret;
        }
        dev_dbg((*component).dev, cstr!("%s: skipping jack init for now\n"), cstr!("rt722_sdca_set_jack_detect"));
        return 0;
    }
    rt722_sdca_jack_init(rt722);
    pm_runtime_put_autosuspend((*component).dev);
    0
}

unsafe fn rt722_cae_load(rt722: *mut rt722_sdca_priv) -> c_int {
    let dev = dev_of_slave(rt722);
    let func_tag = cstr!("FUNC");
    let xu_tag = cstr!("XU");
    let dmi_vendor = dmi_get_system_info(0);
    let dmi_product = dmi_get_system_info(1);
    let dmi_sku = dmi_get_system_info(2);
    if dmi_vendor.is_null() || dmi_product.is_null() || dmi_sku.is_null() {
        dev_warn(dev, cstr!("%s: Incomplete DMI info\n"), cstr!("rt722_cae_load"));
        return -EINVAL;
    }
    let space = strchr(dmi_vendor, b' ' as c_int);
    let v_len = if !space.is_null() { space.offset_from(dmi_vendor) as c_int } else { strlen(dmi_vendor) as c_int };
    let space = strchr(dmi_product, b' ' as c_int);
    let p_len = if !space.is_null() { space.offset_from(dmi_product) as c_int } else { strlen(dmi_product) as c_int };
    let space = strchr(dmi_sku, b' ' as c_int);
    let s_len = if !space.is_null() { space.offset_from(dmi_sku) as c_int } else { strlen(dmi_sku) as c_int };
    let cae_filename = kasprintf(GFP_KERNEL, cstr!("realtek/rt722/rt722_RAE_%.*s_%.*s_%.*s.dat"), v_len, dmi_vendor, p_len, dmi_product, s_len, dmi_sku);
    if cae_filename.is_null() { return -ENOMEM; }
    dev_dbg(dev, cstr!("%s: try to load CAE file %s\n"), cstr!("rt722_cae_load"), cae_filename);
    regmap_write((*rt722).regmap, RT722_SPK_CAE_PARAM1, 0x5f);
    regmap_write((*rt722).regmap, RT722_HP_CAE_PARAM39, 0x5f);
    regmap_write((*rt722).regmap, RT722_MIC_CAE_PARAM39, 0x5f);
    usleep_range(50000, 60000);
    let mut cae_fw: *const firmware = ptr::null();
    request_firmware(&mut cae_fw, cae_filename, dev);
    kfree(cae_filename as *mut c_void);
    if cae_fw.is_null() {
        dev_err(dev, cstr!("%s: Failed to load CAE firmware\n"), cstr!("rt722_cae_load"));
        return -ENOENT;
    }
    let mut cae_st_spk = 0; let mut cae_st_hp = 0; let mut cae_st_mic = 0;
    regmap_read((*rt722).regmap, RT722_SPK_CAE_PARAM38, &mut cae_st_spk);
    regmap_read((*rt722).regmap, RT722_HP_CAE_PARAM68, &mut cae_st_hp);
    regmap_read((*rt722).regmap, RT722_MIC_CAE_PARAM99, &mut cae_st_mic);
    cae_st_spk &= 0x80; cae_st_hp &= 0x80; cae_st_mic &= 0x80;
    if cae_st_spk != 0 { rt722_sdca_index_update_bits(rt722, RT722_VENDOR_EQ_CAE, RT722_EQ_CTRL_SPK, 0x0008, 0x0008); }
    else if cae_st_hp != 0 { rt722_sdca_index_update_bits(rt722, RT722_VENDOR_EQ_CAE, RT722_EQ_CTRL_HP, 0x0008, 0x0008); }
    else if cae_st_mic != 0 { rt722_sdca_index_update_bits(rt722, RT722_VENDOR_EQ_CAE, RT722_EQ_CTRL_DMIC, 0x0008, 0x0008); }
    rt722_sdca_index_update_bits(rt722, RT722_VENDOR_REG, RT722_MISC_CTRL1, 0x8000, 0x8000);
    regmap_update_bits((*rt722).regmap, RT722_SPK_CAE_PARAM34, 0x1, 0x0);
    regmap_update_bits((*rt722).regmap, RT722_HP_CAE_PARAM64, 0x1, 0x0);
    regmap_update_bits((*rt722).regmap, RT722_MIC_CAE_PARAM95, 0x1, 0x0);
    let mut retry = 50;
    while { retry -= 1; retry != 0 } {
        regmap_read((*rt722).regmap, RT722_SPK_CAE_PARAM35, &mut cae_st_spk);
        regmap_read((*rt722).regmap, RT722_HP_CAE_PARAM65, &mut cae_st_hp);
        regmap_read((*rt722).regmap, RT722_MIC_CAE_PARAM96, &mut cae_st_mic);
        if (cae_st_spk & 0x40) != 0 && (cae_st_hp & 0x40) != 0 && (cae_st_mic & 0x40) != 0 { break; }
        usleep_range(1000, 1100);
    }
    if retry == 0 && !((cae_st_spk & 0x40) != 0 && (cae_st_hp & 0x40) != 0 && (cae_st_mic & 0x40) != 0) {
        rt722_sdca_index_update_bits(rt722, RT722_VENDOR_REG, RT722_MISC_CTRL1, 0x8000, 0x0000);
        release_firmware(cae_fw);
        dev_err(dev, cstr!("%s: CAE is not ready to be loaded.\n"), cstr!("rt722_cae_load"));
        return -ETIMEDOUT;
    }
    rt722_sdca_index_write(rt722, RT722_VENDOR_EQ_CAE, RT722_EQ_CTRL_AMIC, 0x8000);
    rt722_sdca_index_write(rt722, RT722_VENDOR_EQ_CAE, RT722_EQ_CTRL_DMIC, 0x8004);
    rt722_sdca_index_write(rt722, RT722_VENDOR_EQ_CAE, RT722_EQ_CTRL_HP, 0x8074);
    rt722_sdca_index_write(rt722, RT722_VENDOR_EQ_CAE, RT722_EQ_CTRL_SPK, 0xa074);
    regcache_cache_bypass((*rt722).regmap, true);
    let mut fw_offset: c_uint = 0;
    let mut mbq_high_val: u8 = 0;
    let mut ret = 0;
    while (fw_offset as usize) < (*cae_fw).size {
        if fw_offset as usize + 12 > (*cae_fw).size { ret = -EINVAL; break; }
        let fw_data = (*cae_fw).data.add(fw_offset as usize);
        let mut tag = [0i8; 5];
        memcpy(tag.as_mut_ptr() as *mut c_void, fw_data as *const c_void, 4);
        tag[4] = 0;
        if strcmp(tag.as_ptr(), xu_tag) == 0 {
            let mut addr: c_uint = 0; let mut size: c_uint = 0;
            memcpy(&mut addr as *mut _ as *mut c_void, fw_data.add(4) as *const c_void, 4);
            memcpy(&mut size as *mut _ as *mut c_void, fw_data.add(8) as *const c_void, 4);
            if size == 0 || size as usize > (*cae_fw).size - fw_offset as usize - 12 { ret = -EINVAL; break; }
            let param_data = fw_data.add(12);
            if (addr <= 0x05302300 && addr >= 0x05300000) || (addr <= 0x020020b4 && addr >= 0x020000b1) {
                if (addr & BIT(13)) != 0 {
                    mbq_high_val = *param_data;
                    fw_offset += size + 12;
                    continue;
                } else {
                    regcache_cache_bypass((*rt722).regmap, false);
                    let mut combined_val = ((mbq_high_val as c_uint) << 8) | (*param_data as c_uint);
                    if addr == 0x20000b1 || addr == 0x20000b4 { combined_val |= 0x2 << 8; }
                    ret = regmap_write((*rt722).regmap, addr, combined_val);
                    if ret != 0 { regcache_cache_bypass((*rt722).regmap, true); break; }
                    fw_offset += size + 12;
                    regcache_cache_bypass((*rt722).regmap, true);
                    continue;
                }
            }
            let mut i = 0;
            while i < size {
                ret = regmap_write((*rt722).regmap, addr + i, *param_data.add(i as usize) as c_uint);
                if ret != 0 { break; }
                i += 1;
            }
            if ret != 0 { break; }
            fw_offset += size + 12;
        } else if strcmp(tag.as_ptr(), func_tag) == 0 {
            let mut func: c_uint = 0; let mut value: c_uint = 0;
            memcpy(&mut func as *mut _ as *mut c_void, fw_data.add(4) as *const c_void, 4);
            memcpy(&mut value as *mut _ as *mut c_void, fw_data.add(8) as *const c_void, 4);
            if func == 1 { msleep(value); }
            fw_offset += 12;
        } else { ret = -EINVAL; break; }
    }
    regcache_cache_bypass((*rt722).regmap, false);
    rt722_sdca_index_update_bits(rt722, RT722_VENDOR_REG, RT722_MISC_CTRL1, 0x8000, 0x0000);
    release_firmware(cae_fw);
    if ret != 0 {
        dev_err(dev, cstr!("%s: CAE FW update aborted (ret=%d).\n"), cstr!("rt722_cae_load"), if ret != 0 { ret } else { -EIO });
        return if ret != 0 { ret } else { -EIO };
    }
    (*rt722).cae_update_done = 1;
    dev_dbg(dev, cstr!("%s: CAE FW update done.\n"), cstr!("rt722_cae_load"));
    0
}

unsafe extern "C" fn rt722_cae_update_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt722 = snd_soc_component_get_drvdata(component);
    let dapm = snd_soc_component_to_dapm(component);
    let mut changed = 0;
    if !(*rt722).hw_init { return 0; }
    let mut ret = pm_runtime_resume_and_get((*component).dev);
    if ret < 0 && ret != -EACCES { return ret; }
    if (*ucontrol).value.integer.value[0] != 0 {
        if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
            ret = rt722_cae_load(rt722);
            if ret != 0 { dev_err((*component).dev, cstr!("CAE load failed: %d\n"), ret); }
            else { changed = 1; }
        }
    } else if (*rt722).cae_update_done != 0 {
        (*rt722).cae_update_done = 0; changed = 1;
    }
    pm_runtime_mark_last_busy((*component).dev);
    pm_runtime_put_autosuspend((*component).dev);
    if ret < 0 { ret } else { changed }
}

unsafe extern "C" fn rt722_cae_update_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt722 = snd_soc_component_get_drvdata(component);
    (*ucontrol).value.integer.value[0] = (*rt722).cae_update_done as c_long;
    0
}

/* For SDCA control DAC/ADC Gain */
unsafe extern "C" fn rt722_sdca_set_gain_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let rt722 = snd_soc_component_get_drvdata(component);
    let mut read_l = 0; let mut read_r = 0; let mut lvalue = 0; let mut rvalue = 0;
    let mut adc_vol_flag = 0;
    let interval_offset: c_uint = 0xc0; let tendB: c_uint = 0xa00;
    if !strstr((*ucontrol).id.name.as_ptr(), cstr!("FU1E Capture Volume")).is_null() || !strstr((*ucontrol).id.name.as_ptr(), cstr!("FU0F Capture Volume")).is_null() { adc_vol_flag = 1; }
    regmap_read((*rt722).regmap, (*mc).reg, &mut lvalue);
    regmap_read((*rt722).regmap, (*mc).rreg, &mut rvalue);
    let mut gain_l_val = (*ucontrol).value.integer.value[0] as c_uint;
    if gain_l_val > (*mc).max { gain_l_val = (*mc).max; }
    if (*mc).shift == 8 { gain_l_val *= tendB; } else {
        gain_l_val = if adc_vol_flag != 0 { 0x1e00u32.wrapping_sub(((*mc).max - gain_l_val) * interval_offset) } else { 0u32.wrapping_sub(((*mc).max - gain_l_val) * interval_offset) };
        gain_l_val &= 0xffff;
    }
    let mut gain_r_val = (*ucontrol).value.integer.value[1] as c_uint;
    if gain_r_val > (*mc).max { gain_r_val = (*mc).max; }
    if (*mc).shift == 8 { gain_r_val *= tendB; } else {
        gain_r_val = if adc_vol_flag != 0 { 0x1e00u32.wrapping_sub(((*mc).max - gain_r_val) * interval_offset) } else { 0u32.wrapping_sub(((*mc).max - gain_r_val) * interval_offset) };
        gain_r_val &= 0xffff;
    }
    if !(lvalue != gain_l_val || rvalue != gain_r_val) { return 0; }
    regmap_write((*rt722).regmap, (*mc).reg, gain_l_val);
    regmap_write((*rt722).regmap, (*mc).rreg, gain_r_val);
    regmap_read((*rt722).regmap, (*mc).reg, &mut read_l);
    regmap_read((*rt722).regmap, (*mc).rreg, &mut read_r);
    if read_r == gain_r_val && read_l == gain_l_val { 1 } else { -EIO }
}

unsafe extern "C" fn rt722_sdca_set_gain_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt722 = snd_soc_component_get_drvdata(component);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let mut read_l = 0; let mut read_r = 0; let mut adc_vol_flag = 0;
    let interval_offset: c_uint = 0xc0; let tendB: c_uint = 0xa00;
    if !strstr((*ucontrol).id.name.as_ptr(), cstr!("FU1E Capture Volume")).is_null() || !strstr((*ucontrol).id.name.as_ptr(), cstr!("FU0F Capture Volume")).is_null() { adc_vol_flag = 1; }
    regmap_read((*rt722).regmap, (*mc).reg, &mut read_l);
    regmap_read((*rt722).regmap, (*mc).rreg, &mut read_r);
    let ctl_l = if (*mc).shift == 8 { read_l / tendB } else if adc_vol_flag != 0 { (*mc).max - ((0x1e00u32.wrapping_sub(read_l) & 0xffff) / interval_offset) } else { (*mc).max - ((0u32.wrapping_sub(read_l) & 0xffff) / interval_offset) };
    let ctl_r = if read_l != read_r {
        if (*mc).shift == 8 { read_r / tendB } else if adc_vol_flag != 0 { (*mc).max - ((0x1e00u32.wrapping_sub(read_r) & 0xffff) / interval_offset) } else { (*mc).max - ((0u32.wrapping_sub(read_r) & 0xffff) / interval_offset) }
    } else { ctl_l };
    (*ucontrol).value.integer.value[0] = ctl_l as c_long;
    (*ucontrol).value.integer.value[1] = ctl_r as c_long;
    0
}

unsafe fn rt722_sdca_set_fu1e_capture_ctl(rt722: *mut rt722_sdca_priv) -> c_int {
    for i in 0..(*rt722).fu1e_mixer_mute.len() {
        let ch_mute = ((*rt722).fu1e_dapm_mute || (*rt722).fu1e_mixer_mute[i]) as c_uint;
        let err = regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_USER_FU1E, RT722_SDCA_CTL_FU_MUTE, CH_01) + i as c_uint, ch_mute);
        if err < 0 { return err; }
    }
    0
}

unsafe extern "C" fn rt722_sdca_fu1e_capture_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt722 = snd_soc_component_get_drvdata(component);
    let p = (*kcontrol).private_value as *mut rt722_sdca_dmic_kctrl_priv;
    for i in 0..(*p).count as usize { (*ucontrol).value.integer.value[i] = (!(*rt722).fu1e_mixer_mute[i]) as c_long; }
    0
}

unsafe extern "C" fn rt722_sdca_fu1e_capture_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt722 = snd_soc_component_get_drvdata(component);
    let p = (*kcontrol).private_value as *mut rt722_sdca_dmic_kctrl_priv;
    let mut changed = 0;
    for i in 0..(*p).count as usize {
        let v = (*ucontrol).value.integer.value[i] == 0;
        if (*rt722).fu1e_mixer_mute[i] != v { changed = 1; }
        (*rt722).fu1e_mixer_mute[i] = v;
    }
    let err = rt722_sdca_set_fu1e_capture_ctl(rt722);
    if err < 0 { return err; }
    changed
}

unsafe fn rt722_sdca_set_fu06_playback_ctl(rt722: *mut rt722_sdca_priv) -> c_int {
    let ch_l = if (*rt722).fu06_dapm_mute || (*rt722).fu06_mixer_l_mute { 0x01 } else { 0x00 };
    let ch_r = if (*rt722).fu06_dapm_mute || (*rt722).fu06_mixer_r_mute { 0x01 } else { 0x00 };
    let mut err = regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_AMP, RT722_SDCA_ENT_USER_FU06, RT722_SDCA_CTL_FU_MUTE, CH_L), ch_l);
    if err < 0 { return err; }
    err = regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_AMP, RT722_SDCA_ENT_USER_FU06, RT722_SDCA_CTL_FU_MUTE, CH_R), ch_r);
    if err < 0 { return err; }
    0
}

unsafe extern "C" fn rt722_sdca_fu06_playback_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rt722 = snd_soc_component_get_drvdata(snd_kcontrol_chip(kcontrol));
    (*ucontrol).value.integer.value[0] = (!(*rt722).fu06_mixer_l_mute) as c_long;
    (*ucontrol).value.integer.value[1] = (!(*rt722).fu06_mixer_r_mute) as c_long;
    0
}

unsafe extern "C" fn rt722_sdca_fu06_playback_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rt722 = snd_soc_component_get_drvdata(snd_kcontrol_chip(kcontrol));
    let new_l = (*ucontrol).value.integer.value[0] == 0;
    let new_r = (*ucontrol).value.integer.value[1] == 0;
    let changed = ((*rt722).fu06_mixer_l_mute != new_l || (*rt722).fu06_mixer_r_mute != new_r) as c_int;
    (*rt722).fu06_mixer_l_mute = new_l; (*rt722).fu06_mixer_r_mute = new_r;
    let err = rt722_sdca_set_fu06_playback_ctl(rt722);
    if err < 0 { return err; }
    changed
}

unsafe fn rt722_sdca_set_fu0f_capture_ctl(rt722: *mut rt722_sdca_priv) -> c_int {
    let ch_l = if (*rt722).fu0f_dapm_mute || (*rt722).fu0f_mixer_l_mute { 0x01 } else { 0x00 };
    let ch_r = if (*rt722).fu0f_dapm_mute || (*rt722).fu0f_mixer_r_mute { 0x01 } else { 0x00 };
    let mut err = regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_USER_FU0F, RT722_SDCA_CTL_FU_MUTE, CH_L), ch_l);
    if err < 0 { return err; }
    err = regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_USER_FU0F, RT722_SDCA_CTL_FU_MUTE, CH_R), ch_r);
    if err < 0 { return err; }
    0
}

unsafe extern "C" fn rt722_sdca_fu0f_capture_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rt722 = snd_soc_component_get_drvdata(snd_kcontrol_chip(kcontrol));
    (*ucontrol).value.integer.value[0] = (!(*rt722).fu0f_mixer_l_mute) as c_long;
    (*ucontrol).value.integer.value[1] = (!(*rt722).fu0f_mixer_r_mute) as c_long;
    0
}

unsafe extern "C" fn rt722_sdca_fu0f_capture_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rt722 = snd_soc_component_get_drvdata(snd_kcontrol_chip(kcontrol));
    let new_l = (*ucontrol).value.integer.value[0] == 0;
    let new_r = (*ucontrol).value.integer.value[1] == 0;
    let changed = ((*rt722).fu0f_mixer_l_mute != new_l || (*rt722).fu0f_mixer_r_mute != new_r) as c_int;
    (*rt722).fu0f_mixer_l_mute = new_l; (*rt722).fu0f_mixer_r_mute = new_r;
    let err = rt722_sdca_set_fu0f_capture_ctl(rt722);
    if err < 0 { return err; }
    changed
}

unsafe extern "C" fn rt722_sdca_fu_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let p = (*kcontrol).private_value as *mut rt722_sdca_dmic_kctrl_priv;
    (*uinfo).type_ = if (*p).max == 1 { SNDRV_CTL_ELEM_TYPE_BOOLEAN } else { SNDRV_CTL_ELEM_TYPE_INTEGER };
    (*uinfo).count = (*p).count;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = (*p).max as c_long;
    0
}

unsafe extern "C" fn rt722_sdca_dmic_set_gain_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rt722 = snd_soc_component_get_drvdata(snd_kcontrol_chip(kcontrol));
    let p = (*kcontrol).private_value as *mut rt722_sdca_dmic_kctrl_priv;
    let boost_step = 0x0a00; let vol_max = 0x1e00; let interval_offset = 0xc0;
    let adc_vol_flag = !strstr((*ucontrol).id.name.as_ptr(), cstr!("FU1E Capture Volume")).is_null();
    for i in 0..(*p).count as usize {
        let mut regvalue = 0;
        regmap_read((*rt722).regmap, (*p).reg_base + i as c_uint, &mut regvalue);
        let ctl = if !adc_vol_flag { regvalue / boost_step } else { (*p).max - (((vol_max - regvalue) & 0xffff) / interval_offset) };
        (*ucontrol).value.integer.value[i] = ctl as c_long;
    }
    0
}

unsafe extern "C" fn rt722_sdca_dmic_set_gain_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let p = (*kcontrol).private_value as *mut rt722_sdca_dmic_kctrl_priv;
    let rt722 = snd_soc_component_get_drvdata(component);
    let boost_step = 0x0a00; let vol_max = 0x1e00; let interval_offset = 0xc0;
    let adc_vol_flag = !strstr((*ucontrol).id.name.as_ptr(), cstr!("FU1E Capture Volume")).is_null();
    let mut gain_val = [0u32; 4];
    let mut regvalue = [0u32; 4];
    let mut changed = 0;
    for i in 0..(*p).count as usize {
        regmap_read((*rt722).regmap, (*p).reg_base + i as c_uint, &mut regvalue[i]);
        gain_val[i] = (*ucontrol).value.integer.value[i] as c_uint;
        if gain_val[i] > (*p).max { gain_val[i] = (*p).max; }
        if !adc_vol_flag { gain_val[i] *= boost_step; } else { gain_val[i] = vol_max - (((*p).max - gain_val[i]) * interval_offset); gain_val[i] &= 0xffff; }
        if regvalue[i] != gain_val[i] { changed = 1; }
    }
    if changed == 0 { return 0; }
    for i in 0..(*p).count as usize {
        let err = regmap_write((*rt722).regmap, (*p).reg_base + i as c_uint, gain_val[i]);
        if err < 0 { dev_err(dev_of_slave(rt722), cstr!("%s: %#08x can't be set\n"), cstr!("rt722_sdca_dmic_set_gain_put"), (*p).reg_base + i as c_uint); }
    }
    changed
}

/* The following C macro-created control, TLV, enum, widget, route, DAI, and component
 * tables are preserved as declarations/comments because their exact Rust data layout
 * depends on ALSA SoC macro expansions outside this isolated source file:
 * RT722_SDCA_PR_VALUE, RT722_SDCA_FU_CTRL, RT722_SDCA_EXT_TLV,
 * DECLARE_TLV_DB_SCALE(out_vol_tlv/mic_vol_tlv/boost_vol_tlv),
 * rt722_sdca_controls, adc22_mux_text, adc07_10_mux_text,
 * rt722_adc22_enum, rt722_adc24_enum, rt722_adc25_enum,
 * rt722_sdca_adc22_mux, rt722_sdca_adc24_mux, rt722_sdca_adc25_mux,
 * rt722_sdca_dapm_widgets, rt722_sdca_audio_map, soc_sdca_dev_rt722,
 * rt722_sdca_ops, and rt722_sdca_dai.
 */

unsafe extern "C" fn rt722_sdca_fu42_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt722 = snd_soc_component_get_drvdata(component);
    match event {
        SND_SOC_DAPM_POST_PMU => {
            regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_USER_FU05, RT722_SDCA_CTL_FU_MUTE, CH_L), 0);
            regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_USER_FU05, RT722_SDCA_CTL_FU_MUTE, CH_R), 0);
        }
        SND_SOC_DAPM_PRE_PMD => {
            regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_USER_FU05, RT722_SDCA_CTL_FU_MUTE, CH_L), 1);
            regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_USER_FU05, RT722_SDCA_CTL_FU_MUTE, CH_R), 1);
        }
        _ => {}
    }
    0
}

extern "C" { static RT722_SDCA_ENT_USER_FU05: c_uint; static SND_SOC_DAPM_POST_PMU: c_int; static SND_SOC_DAPM_PRE_PMD: c_int; }

unsafe extern "C" fn rt722_sdca_fu21_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let rt722 = snd_soc_component_get_drvdata(snd_soc_dapm_to_component((*w).dapm));
    match event { SND_SOC_DAPM_POST_PMU => (*rt722).fu06_dapm_mute = false, SND_SOC_DAPM_PRE_PMD => (*rt722).fu06_dapm_mute = true, _ => {} }
    rt722_sdca_set_fu06_playback_ctl(rt722)
}

unsafe extern "C" fn rt722_sdca_fu113_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let rt722 = snd_soc_component_get_drvdata(snd_soc_dapm_to_component((*w).dapm));
    match event {
        SND_SOC_DAPM_POST_PMU => { (*rt722).fu1e_dapm_mute = false; rt722_sdca_set_fu1e_capture_ctl(rt722); usleep_range(150000, 160000); }
        SND_SOC_DAPM_PRE_PMD => { (*rt722).fu1e_dapm_mute = true; rt722_sdca_set_fu1e_capture_ctl(rt722); }
        _ => {}
    }
    0
}

unsafe extern "C" fn rt722_sdca_fu36_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let rt722 = snd_soc_component_get_drvdata(snd_soc_dapm_to_component((*w).dapm));
    match event {
        SND_SOC_DAPM_POST_PMU => { (*rt722).fu0f_dapm_mute = false; rt722_sdca_set_fu0f_capture_ctl(rt722); }
        SND_SOC_DAPM_PRE_PMD => { (*rt722).fu0f_dapm_mute = true; rt722_sdca_set_fu0f_capture_ctl(rt722); }
        _ => {}
    }
    0
}

unsafe fn rt722_pde_transition_delay(rt722: *mut rt722_sdca_priv, func: u8, entity: u8, ps: u8) {
    let mut delay = 1000u32;
    let mut val = 0;
    pm_runtime_mark_last_busy(dev_of_slave(rt722));
    while delay != 0 {
        regmap_read((*rt722).regmap, SDW_SDCA_CTL(func as c_uint, entity as c_uint, RT722_SDCA_CTL_ACTUAL_POWER_STATE, 0), &mut val);
        if val == ps as c_uint { break; }
        usleep_range(1000, 1500);
        delay -= 1;
    }
    if delay == 0 { dev_warn(dev_of_slave(rt722), cstr!("%s PDE to %s is NOT ready"), cstr!("rt722_pde_transition_delay"), if ps != 0 { cstr!("PS3") } else { cstr!("PS0") }); }
}

unsafe extern "C" fn rt722_sdca_pde47_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let rt722 = snd_soc_component_get_drvdata(snd_soc_dapm_to_component((*w).dapm));
    match event {
        SND_SOC_DAPM_POST_PMU => { regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_PDE40, RT722_SDCA_CTL_REQ_POWER_STATE, 0), 0); rt722_pde_transition_delay(rt722, FUNC_NUM_JACK_CODEC as u8, RT722_SDCA_ENT_PDE40 as u8, 0); }
        SND_SOC_DAPM_PRE_PMD => { regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_PDE40, RT722_SDCA_CTL_REQ_POWER_STATE, 0), 3); rt722_pde_transition_delay(rt722, FUNC_NUM_JACK_CODEC as u8, RT722_SDCA_ENT_PDE40 as u8, 3); }
        _ => {}
    }
    0
}

unsafe extern "C" fn rt722_sdca_pde23_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let rt722 = snd_soc_component_get_drvdata(snd_soc_dapm_to_component((*w).dapm));
    match event {
        SND_SOC_DAPM_POST_PMU => { regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_AMP, RT722_SDCA_ENT_PDE23, RT722_SDCA_CTL_REQ_POWER_STATE, 0), 0); rt722_pde_transition_delay(rt722, FUNC_NUM_AMP as u8, RT722_SDCA_ENT_PDE23 as u8, 0); }
        SND_SOC_DAPM_PRE_PMD => { regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_AMP, RT722_SDCA_ENT_PDE23, RT722_SDCA_CTL_REQ_POWER_STATE, 0), 3); rt722_pde_transition_delay(rt722, FUNC_NUM_AMP as u8, RT722_SDCA_ENT_PDE23 as u8, 3); }
        _ => {}
    }
    0
}

unsafe extern "C" fn rt722_sdca_pde11_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let rt722 = snd_soc_component_get_drvdata(snd_soc_dapm_to_component((*w).dapm));
    match event {
        SND_SOC_DAPM_POST_PMU => { regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_PDE2A, RT722_SDCA_CTL_REQ_POWER_STATE, 0), 0); rt722_pde_transition_delay(rt722, FUNC_NUM_MIC_ARRAY as u8, RT722_SDCA_ENT_PDE2A as u8, 0); }
        SND_SOC_DAPM_PRE_PMD => { regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_PDE2A, RT722_SDCA_CTL_REQ_POWER_STATE, 0), 3); rt722_pde_transition_delay(rt722, FUNC_NUM_MIC_ARRAY as u8, RT722_SDCA_ENT_PDE2A as u8, 3); }
        _ => {}
    }
    0
}

unsafe extern "C" fn rt722_sdca_pde12_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let rt722 = snd_soc_component_get_drvdata(snd_soc_dapm_to_component((*w).dapm));
    match event {
        SND_SOC_DAPM_POST_PMU => { regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_PDE12, RT722_SDCA_CTL_REQ_POWER_STATE, 0), 0); rt722_pde_transition_delay(rt722, FUNC_NUM_JACK_CODEC as u8, RT722_SDCA_ENT_PDE12 as u8, 0); }
        SND_SOC_DAPM_PRE_PMD => { regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_PDE12, RT722_SDCA_CTL_REQ_POWER_STATE, 0), 3); rt722_pde_transition_delay(rt722, FUNC_NUM_JACK_CODEC as u8, RT722_SDCA_ENT_PDE12 as u8, 3); }
        _ => {}
    }
    0
}

unsafe fn rt722_sdca_parse_dt(rt722: *mut rt722_sdca_priv, dev: *mut device) -> c_int {
    device_property_read_u32(dev, cstr!("realtek,jd-src"), &mut (*rt722).jd_src);
    0
}

unsafe extern "C" fn rt722_sdca_probe(component: *mut snd_soc_component) -> c_int {
    let rt722 = snd_soc_component_get_drvdata(component);
    rt722_sdca_parse_dt(rt722, dev_of_slave(rt722));
    (*rt722).component = component;
    let ret = pm_runtime_resume((*component).dev);
    if ret < 0 && ret != -EACCES { return ret; }
    0
}

unsafe extern "C" fn rt722_sdca_set_sdw_stream(dai: *mut snd_soc_dai, sdw_stream: *mut c_void, direction: c_int) -> c_int {
    snd_soc_dai_dma_data_set(dai, direction, sdw_stream);
    0
}

unsafe extern "C" fn rt722_sdca_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    snd_soc_dai_set_dma_data(dai, substream, ptr::null_mut());
}

unsafe extern "C" fn rt722_sdca_pcm_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let rt722 = snd_soc_component_get_drvdata(component);
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    if sdw_stream.is_null() || (*rt722).slave.is_null() { return -EINVAL; }
    let (direction, port) = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        if (*dai).id == RT722_AIF1 { (sdw_data_direction::SDW_DATA_DIR_RX, 1) } else if (*dai).id == RT722_AIF2 { (sdw_data_direction::SDW_DATA_DIR_RX, 3) } else { return -EINVAL; }
    } else {
        if (*dai).id == RT722_AIF1 { (sdw_data_direction::SDW_DATA_DIR_TX, 2) } else if (*dai).id == RT722_AIF3 { (sdw_data_direction::SDW_DATA_DIR_TX, 6) } else { return -EINVAL; }
    };
    let mut stream_config = sdw_stream_config { frame_rate: params_rate(params), ch_count: params_channels(params) as c_uint, bps: snd_pcm_format_width(params_format(params)), direction };
    let num_channels = params_channels(params);
    let mut port_config = sdw_port_config { ch_mask: GENMASK(num_channels - 1, 0), num: port };
    let retval = sdw_stream_add_slave((*rt722).slave, &mut stream_config, &mut port_config, 1, sdw_stream);
    if retval != 0 { dev_err((*dai).dev, cstr!("%s: Unable to configure port\n"), cstr!("rt722_sdca_pcm_hw_params")); return retval; }
    if params_channels(params) > 16 { dev_err((*component).dev, cstr!("%s: Unsupported channels %d\n"), cstr!("rt722_sdca_pcm_hw_params"), params_channels(params)); return -EINVAL; }
    let sampling_rate = match params_rate(params) {
        44100 => RT722_SDCA_RATE_44100HZ,
        48000 => RT722_SDCA_RATE_48000HZ,
        96000 => RT722_SDCA_RATE_96000HZ,
        192000 => RT722_SDCA_RATE_192000HZ,
        _ => { dev_err((*component).dev, cstr!("%s: Rate %d is not supported\n"), cstr!("rt722_sdca_pcm_hw_params"), params_rate(params)); return -EINVAL; }
    };
    if (*dai).id == RT722_AIF1 {
        regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_CS01, RT722_SDCA_CTL_SAMPLE_FREQ_INDEX, 0), sampling_rate);
        regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_CS11, RT722_SDCA_CTL_SAMPLE_FREQ_INDEX, 0), sampling_rate);
    }
    if (*dai).id == RT722_AIF2 { regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_AMP, RT722_SDCA_ENT_CS31, RT722_SDCA_CTL_SAMPLE_FREQ_INDEX, 0), sampling_rate); }
    if (*dai).id == RT722_AIF3 { regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_CS1F, RT722_SDCA_CTL_SAMPLE_FREQ_INDEX, 0), sampling_rate); }
    0
}

unsafe extern "C" fn rt722_sdca_pcm_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let rt722 = snd_soc_component_get_drvdata((*dai).component);
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    if (*rt722).slave.is_null() { return -EINVAL; }
    sdw_stream_remove_slave((*rt722).slave, sdw_stream);
    0
}

pub unsafe extern "C" fn rt722_sdca_init(dev: *mut device, regmap: *mut regmap, slave: *mut sdw_slave) -> c_int {
    let rt722 = devm_kzalloc(dev, core::mem::size_of::<rt722_sdca_priv>(), GFP_KERNEL) as *mut rt722_sdca_priv;
    if rt722.is_null() { return -ENOMEM; }
    dev_set_drvdata(dev, rt722 as *mut c_void);
    (*rt722).slave = slave;
    (*rt722).regmap = regmap;
    regcache_cache_only((*rt722).regmap, true);
    mutex_init(&mut (*rt722).calibrate_mutex);
    mutex_init(&mut (*rt722).disable_irq_lock);
    INIT_DELAYED_WORK(&mut (*rt722).jack_detect_work, rt722_sdca_jack_detect_handler);
    INIT_DELAYED_WORK(&mut (*rt722).jack_btn_check_work, rt722_sdca_btn_check_handler);
    (*rt722).hw_init = false;
    (*rt722).first_hw_init = false;
    (*rt722).fu1e_dapm_mute = true;
    (*rt722).fu0f_dapm_mute = true;
    (*rt722).fu06_dapm_mute = true;
    (*rt722).fu06_mixer_l_mute = false; (*rt722).fu06_mixer_r_mute = false;
    (*rt722).fu0f_mixer_l_mute = true; (*rt722).fu0f_mixer_r_mute = true;
    (*rt722).fu1e_mixer_mute = [true; 4];
    (*rt722).cae_update_done = 0;
    devm_snd_soc_register_component(dev, &soc_sdca_dev_rt722, rt722_sdca_dai.as_mut_ptr(), 3)
}

unsafe fn rt722_sdca_dmic_preset(rt722: *mut rt722_sdca_priv) {
    let mut mic_func_status = 0;
    regmap_read((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT0, RT722_SDCA_CTL_FUNC_STATUS, 0), &mut mic_func_status);
    if (mic_func_status & FUNCTION_NEEDS_INITIALIZATION) != 0 || !(*rt722).first_hw_init {
        rt722_sdca_index_write(rt722, RT722_VENDOR_HDA_CTL, RT722_ADC0A_08_PDE_FLOAT_CTL, 0x2a29);
        rt722_sdca_index_write(rt722, RT722_VENDOR_HDA_CTL, RT722_ADC10_PDE_FLOAT_CTL, 0x2a00);
        rt722_sdca_index_write(rt722, RT722_VENDOR_HDA_CTL, RT722_DMIC1_2_PDE_FLOAT_CTL, 0x2a2a);
        rt722_sdca_index_write(rt722, RT722_VENDOR_HDA_CTL, RT722_DMIC_ENT_FLOAT_CTL, 0x2626);
        rt722_sdca_index_write(rt722, RT722_VENDOR_HDA_CTL, RT722_ADC_ENT_FLOAT_CTL, 0x1e00);
        rt722_sdca_index_write(rt722, RT722_VENDOR_HDA_CTL, RT722_DMIC_GAIN_ENT_FLOAT_CTL0, 0x1515);
        rt722_sdca_index_write(rt722, RT722_VENDOR_HDA_CTL, RT722_ADC_VOL_CH_FLOAT_CTL, 0x0304);
        rt722_sdca_index_write(rt722, RT722_VENDOR_HDA_CTL, RT722_DMIC_GAIN_ENT_FLOAT_CTL2, 0x0304);
        rt722_sdca_index_write(rt722, RT722_VENDOR_HDA_CTL, RT722_HDA_LEGACY_CONFIG_CTL0, 0x0000);
        regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_IT26, RT722_SDCA_CTL_VENDOR_DEF, 0), 0x01);
        regmap_write((*rt722).regmap, 0x2f5c, 0x25);
        regmap_write((*rt722).regmap, 0x2f03, 0x06);
        if (*rt722).hw_vid == RT722_VB { regmap_write((*rt722).regmap, 0x2f52, 0x00); }
        regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT0, RT722_SDCA_CTL_FUNC_STATUS, 0), FUNCTION_NEEDS_INITIALIZATION);
    }
}

unsafe fn rt722_sdca_amp_preset(rt722: *mut rt722_sdca_priv) {
    let mut amp_func_status = 0;
    regmap_read((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_AMP, RT722_SDCA_ENT0, RT722_SDCA_CTL_FUNC_STATUS, 0), &mut amp_func_status);
    if (amp_func_status & FUNCTION_NEEDS_INITIALIZATION) != 0 || !(*rt722).first_hw_init {
        rt722_sdca_index_write(rt722, RT722_VENDOR_REG, RT722_CLSD_CTRL6, 0xc215);
        rt722_sdca_index_write(rt722, RT722_VENDOR_CALI, RT722_DC_CALIB_CTRL, 0x702c);
        rt722_sdca_index_write(rt722, RT722_VENDOR_CALI, RT722_DC_CALIB_CTRL, 0xf02d);
        rt722_sdca_index_write(rt722, RT722_VENDOR_HDA_CTL, RT722_AMP_PDE_FLOAT_CTL, 0x2323);
        rt722_sdca_index_write(rt722, RT722_VENDOR_HDA_CTL, RT722_EAPD_CTL, 0x0002);
        regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_AMP, RT722_SDCA_ENT_OT23, RT722_SDCA_CTL_VENDOR_DEF, CH_08), 0x04);
        if (*rt722).hw_vid == RT722_VB { regmap_write((*rt722).regmap, 0x2f54, 0x00); }
        regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_AMP, RT722_SDCA_ENT0, RT722_SDCA_CTL_FUNC_STATUS, 0), FUNCTION_NEEDS_INITIALIZATION);
    }
}

unsafe fn rt722_sdca_jack_preset(rt722: *mut rt722_sdca_priv) {
    let mut calib_status = 0; let mut jack_func_status = 0;
    regmap_read((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT0, RT722_SDCA_CTL_FUNC_STATUS, 0), &mut jack_func_status);
    if (jack_func_status & FUNCTION_NEEDS_INITIALIZATION) != 0 || !(*rt722).first_hw_init {
        rt722_sdca_index_write(rt722, RT722_VENDOR_REG, RT722_ANALOG_BIAS_CTL3, 0xa081);
        rt722_sdca_index_write(rt722, RT722_VENDOR_HDA_CTL, RT722_GE_RELATED_CTL2, 0xa009);
        rt722_sdca_index_write(rt722, RT722_VENDOR_HDA_CTL, RT722_UMP_HID_CTL4, 0xcf00);
        rt722_sdca_index_write(rt722, RT722_VENDOR_HDA_CTL, RT722_UMP_HID_CTL5, 0x000f);
        rt722_sdca_index_write(rt722, RT722_VENDOR_HDA_CTL, RT722_UMP_HID_CTL0, 0x1100);
        rt722_sdca_index_write(rt722, RT722_VENDOR_HDA_CTL, RT722_UMP_HID_CTL7, 0x0c12);
        rt722_sdca_index_write(rt722, RT722_VENDOR_REG, RT722_JD_CTRL1, 0x7002);
        rt722_sdca_index_write(rt722, RT722_VENDOR_REG, RT722_CLSD_CTRL6, 0xc215);
        rt722_sdca_index_write(rt722, RT722_VENDOR_REG, RT722_FSM_CTL, 0x4100);
        rt722_sdca_index_write(rt722, RT722_VENDOR_CALI, RT722_DAC_DC_CALI_CTL3, 0x008d);
        let chk_cnt = 100;
        let mut loop_check = 0;
        while loop_check < chk_cnt {
            usleep_range(10000, 11000);
            let ret = rt722_sdca_index_read(rt722, RT722_VENDOR_CALI, RT722_DAC_DC_CALI_CTL3, &mut calib_status);
            if ret < 0 { dev_dbg(dev_of_slave(rt722), cstr!("calibration failed!, ret=%d\n"), ret); }
            if (calib_status & 0x0040) == 0 { break; }
            loop_check += 1;
        }
        if loop_check == chk_cnt { dev_dbg(dev_of_slave(rt722), cstr!("%s, calibration time-out!\n"), cstr!("rt722_sdca_jack_preset")); }
        rt722_sdca_index_write(rt722, RT722_VENDOR_HDA_CTL, RT722_ADC0A_08_PDE_FLOAT_CTL, 0x2a12);
        rt722_sdca_index_write(rt722, RT722_VENDOR_HDA_CTL, RT722_MIC2_LINE2_PDE_FLOAT_CTL, 0x3429);
        rt722_sdca_index_write(rt722, RT722_VENDOR_HDA_CTL, RT722_ET41_LINE2_PDE_FLOAT_CTL, 0x4112);
        rt722_sdca_index_write(rt722, RT722_VENDOR_HDA_CTL, RT722_DAC03_HP_PDE_FLOAT_CTL, 0x4040);
        rt722_sdca_index_write(rt722, RT722_VENDOR_HDA_CTL, RT722_ENT_FLOAT_CTRL_1, 0x4141);
        rt722_sdca_index_write(rt722, RT722_VENDOR_HDA_CTL, RT722_FLOAT_CTRL_1, 0x0101);
        regmap_write((*rt722).regmap, 0x2f58, 0x07);
        regmap_write((*rt722).regmap, 0x2f03, 0x06);
        rt722_sdca_index_update_bits(rt722, RT722_VENDOR_REG, RT722_COMBO_JACK_AUTO_CTL1, 0x0200, 0x0200);
        rt722_sdca_index_update_bits(rt722, RT722_VENDOR_REG, RT722_VREFO_GAT, 0x4000, 0x4000);
        rt722_sdca_index_write(rt722, RT722_VENDOR_REG, RT722_DIGITAL_MISC_CTRL4, 0x0010);
        if (*rt722).hw_vid == RT722_VB { regmap_write((*rt722).regmap, 0x2f51, 0x00); }
        regmap_write((*rt722).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT0, RT722_SDCA_CTL_FUNC_STATUS, 0), FUNCTION_NEEDS_INITIALIZATION);
    }
}

unsafe fn rt722_sdca_reset(rt722: *mut rt722_sdca_priv) {
    rt722_sdca_index_update_bits(rt722, RT722_VENDOR_REG, RT722_LDO1_CTL, RT722_HIDDEN_REG_SW_RESET, RT722_HIDDEN_REG_SW_RESET);
    rt722_sdca_index_update_bits(rt722, RT722_VENDOR_HDA_CTL, RT722_HDA_LEGACY_RESET_CTL, 0x1, 0x1);
    if (*rt722).hw_vid == RT722_VA { rt722_sdca_index_write(rt722, RT722_VENDOR_REG, RT722_LDO1_CTL, 0xb091); }
}

pub unsafe extern "C" fn rt722_sdca_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int {
    let rt722 = dev_get_drvdata(dev) as *mut rt722_sdca_priv;
    let mut val = 0;
    (*rt722).disable_irq = false;
    if (*rt722).hw_init { return 0; }
    regcache_cache_only((*rt722).regmap, false);
    if (*rt722).first_hw_init {
        regcache_cache_bypass((*rt722).regmap, true);
    } else {
        pm_runtime_set_autosuspend_delay(&mut (*slave).dev, 3000);
        pm_runtime_use_autosuspend(&mut (*slave).dev);
        pm_runtime_set_active(&mut (*slave).dev);
        pm_runtime_mark_last_busy(&mut (*slave).dev);
        pm_runtime_enable(&mut (*slave).dev);
    }
    pm_runtime_get_noresume(&mut (*slave).dev);
    rt722_sdca_index_read(rt722, RT722_VENDOR_REG, RT722_JD_PRODUCT_NUM, &mut val);
    (*rt722).hw_vid = (val & 0x0f00) >> 8;
    dev_dbg(&mut (*slave).dev, cstr!("%s hw_vid=0x%x\n"), cstr!("rt722_sdca_io_init"), (*rt722).hw_vid);
    if !(*rt722).first_hw_init { rt722_sdca_reset(rt722); }
    rt722_sdca_dmic_preset(rt722);
    rt722_sdca_amp_preset(rt722);
    rt722_sdca_jack_preset(rt722);
    if !(*rt722).hs_jack.is_null() && !(*rt722).first_hw_init { rt722_sdca_jack_init(rt722); }
    if (*rt722).first_hw_init {
        regcache_cache_bypass((*rt722).regmap, false);
        regcache_mark_dirty((*rt722).regmap);
    } else { (*rt722).first_hw_init = true; }
    (*rt722).hw_init = true;
    pm_runtime_put_autosuspend(&mut (*slave).dev);
    dev_dbg(&mut (*slave).dev, cstr!("%s hw_init complete\n"), cstr!("rt722_sdca_io_init"));
    0
}

/* MODULE_DESCRIPTION("ASoC RT722 SDCA SDW driver");
 * MODULE_AUTHOR("Jack Yu <jack.yu@realtek.com>");
 * MODULE_LICENSE("GPL");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
