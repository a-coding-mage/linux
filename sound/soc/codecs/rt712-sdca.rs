// SPDX-License-Identifier: GPL-2.0-only
//
// rt712-sdca.rs -- rt712 SDCA ALSA SoC audio driver
//
// Copyright(c) 2023 Realtek Semiconductor Corp.
//
// Source-level Rust translation of rt712-sdca.c. C include dependencies from
// Linux/ALSA/SoundWire headers and "rt712-sdca.h" are expected to be supplied by
// the surrounding crate/bindings.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct sdw_slave { pub dev: device, pub id: sdw_slave_id, pub prop: sdw_slave_prop }
#[repr(C)] pub struct sdw_slave_id { pub part_id: c_uint }
#[repr(C)] pub struct sdw_slave_prop { pub lane_control_support: bool }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device, pub card: *mut snd_soc_card }
#[repr(C)] pub struct snd_soc_card { pub instantiated: bool }
#[repr(C)] pub struct snd_soc_jack { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { pub work: work_struct }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { pub private_value: c_ulong }
#[repr(C)] pub struct soc_mixer_control { pub reg: c_uint, pub rreg: c_uint, pub shift: c_uint, pub max: c_uint }
#[repr(C)] pub struct soc_enum { pub items: c_uint, pub shift_l: c_uint }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context }
#[repr(C)] pub struct snd_ctl_elem_info { pub type_: c_uint, pub count: c_uint, pub value: snd_ctl_elem_info_value }
#[repr(C)] pub union snd_ctl_elem_info_value { pub integer: snd_ctl_elem_info_integer }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_info_integer { pub min: i64, pub max: i64 }
#[repr(C)] pub struct snd_ctl_elem_value { pub id: snd_ctl_elem_id, pub value: snd_ctl_elem_value_union }
#[repr(C)] pub struct snd_ctl_elem_id { pub name: [c_char; 44] }
#[repr(C)] pub union snd_ctl_elem_value_union { pub integer: snd_ctl_elem_value_integer, pub enumerated: snd_ctl_elem_value_enumerated }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_integer { pub value: [i64; 128] }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_enumerated { pub item: [c_uint; 128] }
#[repr(C)] pub struct snd_soc_dai { pub component: *mut snd_soc_component, pub dev: *mut device, pub name: *const c_char, pub id: c_int }
#[repr(C)] pub struct snd_pcm_substream { pub stream: c_int }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct sdw_stream_runtime { _private: [u8; 0] }
#[repr(C)] pub struct sdw_stream_config { pub frame_rate: c_uint, pub ch_count: c_uint, pub bps: c_int, pub direction: sdw_data_direction }
#[repr(C)] pub struct sdw_port_config { pub ch_mask: c_uint, pub num: c_int }
#[repr(C)] #[derive(Copy, Clone)] pub enum sdw_data_direction { SDW_DATA_DIR_RX = 0, SDW_DATA_DIR_TX = 1 }

#[repr(C)]
pub struct rt712_dmic_kctrl_priv {
    pub reg_base: c_uint,
    pub count: c_uint,
    pub max: c_uint,
    pub invert: c_uint,
}

#[repr(C)]
pub struct rt712_sdca_priv {
    pub mbq_regmap: *mut regmap,
    pub regmap: *mut regmap,
    pub slave: *mut sdw_slave,
    pub calibrate_mutex: mutex,
    pub disable_irq_lock: mutex,
    pub jack_detect_work: delayed_work,
    pub jack_btn_check_work: delayed_work,
    pub hs_jack: *mut snd_soc_jack,
    pub component: *mut snd_soc_component,
    pub dmic_component: *mut snd_soc_component,
    pub scp_sdca_stat1: c_uint,
    pub scp_sdca_stat2: c_uint,
    pub jack_type: c_int,
    pub version_id: c_uint,
    pub jd_src: c_uint,
    pub hw_id: c_uint,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub fu0f_dapm_mute: bool,
    pub fu0f_mixer_l_mute: bool,
    pub fu0f_mixer_r_mute: bool,
    pub fu05_dapm_mute: bool,
    pub fu05_mixer_l_mute: bool,
    pub fu05_mixer_r_mute: bool,
    pub fu1e_dapm_mute: bool,
    pub fu1e_mixer_mute: [bool; 4],
    pub dmic_function_found: bool,
    pub disable_irq: bool,
}

extern "C" {
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_get_device(map: *mut regmap) -> *mut device;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_cache_bypass(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_init(lock: *mut mutex);
    fn usleep_range(min: c_uint, max: c_uint);
    fn strstr(s: *const c_char, find: *const c_char) -> *mut c_char;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut rt712_sdca_priv;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_kcontrol_to_component(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dapm_kcontrol_to_dapm(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_dapm_mux_update_power(dapm: *mut snd_soc_dapm_context, kcontrol: *mut snd_kcontrol, item: c_uint, e: *mut soc_enum, update: *mut c_void);
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn mod_delayed_work(wq: *mut c_void, dwork: *mut delayed_work, delay: c_ulong) -> bool;
    fn msecs_to_jiffies(msecs: c_uint) -> c_ulong;
    fn snd_soc_enum_item_to_val(e: *mut soc_enum, item: c_uint) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn snd_soc_dai_get_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream) -> *mut sdw_stream_runtime;
    fn snd_soc_dai_dma_data_set(dai: *mut snd_soc_dai, direction: c_int, data: *mut c_void);
    fn snd_soc_dai_set_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream, data: *mut c_void);
    fn sdw_stream_add_slave(slave: *mut sdw_slave, stream_config: *mut sdw_stream_config, port_config: *mut sdw_port_config, num_ports: c_uint, stream: *mut sdw_stream_runtime) -> c_int;
    fn sdw_stream_remove_slave(slave: *mut sdw_slave, stream: *mut sdw_stream_runtime);
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_resume(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device);
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn device_property_read_u32(dev: *mut device, propname: *const c_char, val: *mut c_uint) -> c_int;
    fn sdw_write_no_pm(slave: *mut sdw_slave, reg: c_uint, val: c_uint) -> c_int;
    fn sdca_device_quirk_match(slave: *mut sdw_slave, quirk: c_uint) -> bool;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const c_void, dai_drv: *mut c_void, num_dai: c_int) -> c_int;
}

extern "C" { static mut system_power_efficient_wq: *mut c_void; }

macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const c_char } }
macro_rules! ARRAY_SIZE { ($a:expr) => { ($a).len() as c_int } }
macro_rules! GENMASK { ($h:expr, $l:expr) => { (((!0u32) << ($l)) & ((!0u32) >> (31 - ($h)))) } }
macro_rules! SDW_SDCA_CTL { ($func:expr, $ent:expr, $ctl:expr, $ch:expr) => { (($func) << 16) | (($ent) << 8) | (($ctl) << 4) | ($ch) } }
macro_rules! dev_err { ($($arg:tt)*) => {{ }} }
macro_rules! dev_dbg { ($($arg:tt)*) => {{ }} }
macro_rules! dev_warn { ($($arg:tt)*) => {{ }} }
macro_rules! pr_err_ratelimited { ($($arg:tt)*) => {{ }} }
macro_rules! INIT_DELAYED_WORK { ($work:expr, $handler:expr) => {{ let _ = ($work, $handler); }} }

const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const EACCES: c_int = 13;
const ETIMEDOUT: c_int = 110;

const GFP_KERNEL: c_uint = 0;
const SND_JACK_HEADPHONE: c_int = 0x0001;
const SND_JACK_HEADSET: c_int = 0x0004;
const SND_JACK_BTN_0: c_int = 0x4000;
const SND_JACK_BTN_1: c_int = 0x2000;
const SND_JACK_BTN_2: c_int = 0x1000;
const SND_JACK_BTN_3: c_int = 0x0800;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SND_SOC_DAPM_POST_PMU: c_int = 0x1;
const SND_SOC_DAPM_PRE_PMD: c_int = 0x2;
const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_uint = 1;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 2;

// Constants from rt712-sdca.h and Linux headers are referenced as in C.
extern "C" {
    static RT712_VENDOR_REG: c_uint; static RT712_CC_DET1: c_uint; static RT712_FSM_CTL: c_uint;
    static RT712_VENDOR_CALI: c_uint; static RT712_DAC_DC_CALI_CTL1: c_uint; static RT712_DAC_DC_CALI_TRIGGER: c_uint;
    static RT712_VENDOR_IMS_DRE: c_uint; static RT712_IMS_DIGITAL_CTL1: c_uint; static RT712_IMS_DIGITAL_CTL5: c_uint;
    static RT712_DIGITAL_MISC_CTRL4: c_uint; static FUNC_NUM_HID: c_uint; static RT712_SDCA_ENT_HID01: c_uint;
    static RT712_SDCA_CTL_HIDTX_CURRENT_OWNER: c_uint; static RT712_SDCA_CTL_HIDTX_MESSAGE_OFFSET: c_uint;
    static RT712_BUF_ADDR_HID1: c_uint; static RT712_SDCA_CTL_HIDTX_SET_OWNER_TO_DEVICE: c_uint;
    static RT712_VA: c_uint; static FUNC_NUM_JACK_CODEC: c_uint; static RT712_SDCA_ENT_GE49: c_uint;
    static RT712_SDCA_CTL_DETECTED_MODE: c_uint; static RT712_SDCA_CTL_SELECTED_MODE: c_uint;
    static SDW_SCP_SDCA_INT_SDCA_0: c_uint; static SDW_SCP_SDCA_INT_SDCA_8: c_uint;
    static RT712_VENDOR_HDA_CTL: c_uint; static RT712_UMP_HID_CTL5: c_uint; static RT712_UMP_HID_CTL0: c_uint;
    static RT712_UMP_HID_CTL7: c_uint; static RT712_GE_RELATED_CTL1: c_uint; static RT712_GE_RELATED_CTL2: c_uint;
    static RT712_JD1: c_uint; static SDW_SCP_SDCA_INTMASK1: c_uint; static SDW_SCP_SDCA_INTMASK2: c_uint;
    static SDW_SCP_SDCA_INTMASK_SDCA_0: c_uint; static SDW_SCP_SDCA_INTMASK_SDCA_8: c_uint;
}

unsafe fn set_mask_bits(tmp: *mut c_uint, mask: c_uint, val: c_uint) {
    *tmp = (*tmp & !mask) | (val & mask);
}

unsafe fn rt712_sdca_index_write(rt712: *mut rt712_sdca_priv, nid: c_uint, reg: c_uint, value: c_uint) -> c_int {
    let regmap = (*rt712).mbq_regmap;
    let addr = (nid << 20) | reg;
    let ret = regmap_write(regmap, addr, value);
    if ret < 0 {
        dev_err!(&(*(*rt712).slave).dev, "%s: Failed to set private value: %06x <= %04x ret=%d\n", cstr!("rt712_sdca_index_write"), addr, value, ret);
    }
    ret
}

unsafe fn rt712_sdca_index_read(rt712: *mut rt712_sdca_priv, nid: c_uint, reg: c_uint, value: *mut c_uint) -> c_int {
    let regmap = (*rt712).mbq_regmap;
    let addr = (nid << 20) | reg;
    let ret = regmap_read(regmap, addr, value);
    if ret < 0 {
        dev_err!(&(*(*rt712).slave).dev, "%s: Failed to get private value: %06x => %04x ret=%d\n", cstr!("rt712_sdca_index_read"), addr, *value, ret);
    }
    ret
}

unsafe fn rt712_sdca_index_update_bits(rt712: *mut rt712_sdca_priv, nid: c_uint, reg: c_uint, mask: c_uint, val: c_uint) -> c_int {
    let mut tmp: c_uint = 0;
    let ret = rt712_sdca_index_read(rt712, nid, reg, &mut tmp);
    if ret < 0 { return ret; }
    set_mask_bits(&mut tmp, mask, val);
    rt712_sdca_index_write(rt712, nid, reg, tmp)
}

unsafe fn rt712_sdca_calibration(rt712: *mut rt712_sdca_priv) -> c_int {
    let mut val: c_uint = 0;
    let mut loop_rc: c_uint = 0;
    let mut loop_dc: c_uint = 0;
    let regmap = (*rt712).regmap;
    let chk_cnt: c_uint = 100;
    let mut ret: c_int = 0;
    mutex_lock(&mut (*rt712).calibrate_mutex);
    let dev = regmap_get_device(regmap);
    if (*rt712).version_id == RT712_VA { rt712_sdca_index_write(rt712, RT712_VENDOR_REG, RT712_CC_DET1, 0x043a); }
    rt712_sdca_index_write(rt712, RT712_VENDOR_REG, RT712_FSM_CTL, 0x4100);
    rt712_sdca_index_write(rt712, RT712_VENDOR_CALI, RT712_DAC_DC_CALI_CTL1, 0x7883);
    rt712_sdca_index_write(rt712, RT712_VENDOR_CALI, RT712_DAC_DC_CALI_CTL1, 0xf893);
    rt712_sdca_index_read(rt712, RT712_VENDOR_CALI, RT712_DAC_DC_CALI_CTL1, &mut val);
    while loop_dc < chk_cnt && (val & RT712_DAC_DC_CALI_TRIGGER) != 0 {
        usleep_range(10000, 11000);
        ret = rt712_sdca_index_read(rt712, RT712_VENDOR_CALI, RT712_DAC_DC_CALI_CTL1, &mut val);
        if ret < 0 { break; }
        loop_dc += 1;
    }
    if ret >= 0 {
        if loop_dc == chk_cnt { dev_err!(dev, "%s, calibration time-out!\n", cstr!("rt712_sdca_calibration")); }
        if loop_dc == chk_cnt || loop_rc == chk_cnt { ret = -ETIMEDOUT; }
    }
    rt712_sdca_index_write(rt712, RT712_VENDOR_REG, RT712_FSM_CTL, 0x4500);
    rt712_sdca_index_write(rt712, RT712_VENDOR_IMS_DRE, RT712_IMS_DIGITAL_CTL1, 0x040f);
    rt712_sdca_index_write(rt712, RT712_VENDOR_IMS_DRE, RT712_IMS_DIGITAL_CTL5, 0x0000);
    rt712_sdca_index_write(rt712, RT712_VENDOR_REG, RT712_DIGITAL_MISC_CTRL4, 0x0010);
    mutex_unlock(&mut (*rt712).calibrate_mutex);
    dev_dbg!(dev, "%s calibration complete, ret=%d\n", cstr!("rt712_sdca_calibration"), ret);
    ret
}

unsafe fn rt712_sdca_button_detect(rt712: *mut rt712_sdca_priv) -> c_uint {
    let mut btn_type: c_uint = 0;
    let mut offset: c_uint = 0;
    let mut val: c_uint = 0;
    let mut owner: c_uint = 0;
    let mut buf = [0u8; 3];
    let mut ret = regmap_read((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_HID, RT712_SDCA_ENT_HID01, RT712_SDCA_CTL_HIDTX_CURRENT_OWNER, 0), &mut owner);
    if ret < 0 || owner == 1 { return 0; }
    ret = regmap_read((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_HID, RT712_SDCA_ENT_HID01, RT712_SDCA_CTL_HIDTX_MESSAGE_OFFSET, 0), &mut offset);
    if ret >= 0 {
        for idx in 0..buf.len() {
            ret = regmap_read((*rt712).regmap, RT712_BUF_ADDR_HID1 + offset + idx as c_uint, &mut val);
            if ret < 0 { break; }
            buf[idx] = (val & 0xff) as u8;
        }
        if ret >= 0 && buf[0] == 0x11 {
            match buf[1] & 0xf0 { 0x10 => btn_type |= SND_JACK_BTN_2 as c_uint, 0x20 => btn_type |= SND_JACK_BTN_3 as c_uint, 0x40 => btn_type |= SND_JACK_BTN_0 as c_uint, 0x80 => btn_type |= SND_JACK_BTN_1 as c_uint, _ => {} }
            match buf[2] { 0x01 | 0x10 => btn_type |= SND_JACK_BTN_2 as c_uint, 0x02 | 0x20 => btn_type |= SND_JACK_BTN_3 as c_uint, 0x04 | 0x40 => btn_type |= SND_JACK_BTN_0 as c_uint, 0x08 | 0x80 => btn_type |= SND_JACK_BTN_1 as c_uint, _ => {} }
        }
    }
    if owner == 0 {
        if (*rt712).version_id == RT712_VA {
            regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_HID, RT712_SDCA_ENT_HID01, RT712_SDCA_CTL_HIDTX_SET_OWNER_TO_DEVICE, 0), 0x01);
        } else {
            regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_HID, RT712_SDCA_ENT_HID01, RT712_SDCA_CTL_HIDTX_CURRENT_OWNER, 0), 0x01);
        }
    }
    btn_type
}

unsafe fn rt712_sdca_headset_detect(rt712: *mut rt712_sdca_priv) -> c_int {
    let mut det_mode: c_uint = 0;
    let ret = regmap_read((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_GE49, RT712_SDCA_CTL_DETECTED_MODE, 0), &mut det_mode);
    if ret < 0 {
        pr_err_ratelimited!("IO error in %s, ret %d\n", cstr!("rt712_sdca_headset_detect"), ret);
        return ret;
    }
    match det_mode { 0x00 => (*rt712).jack_type = 0, 0x03 => (*rt712).jack_type = SND_JACK_HEADPHONE, 0x05 => (*rt712).jack_type = SND_JACK_HEADSET, _ => {} }
    if det_mode != 0 {
        let ret2 = regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_GE49, RT712_SDCA_CTL_SELECTED_MODE, 0), det_mode);
        if ret2 < 0 {
            pr_err_ratelimited!("IO error in %s, ret %d\n", cstr!("rt712_sdca_headset_detect"), ret2);
            return ret2;
        }
    }
    dev_dbg!(&(*(*rt712).slave).dev, "%s, detected_mode=0x%x\n", cstr!("rt712_sdca_headset_detect"), det_mode);
    0
}

unsafe fn rt712_sdca_jack_detect_handler(work: *mut work_struct) {
    let rt712 = container_of_jack_detect_work(work);
    let mut btn_type: c_int = 0;
    if (*rt712).hs_jack.is_null() { return; }
    if (*rt712).component.is_null() || (*(*rt712).component).card.is_null() || !(*(*(*rt712).component).card).instantiated { return; }
    if ((*rt712).scp_sdca_stat1 & SDW_SCP_SDCA_INT_SDCA_0) != 0 {
        let ret = rt712_sdca_headset_detect(rt712);
        if ret < 0 { return; }
    }
    if ((*rt712).scp_sdca_stat2 & SDW_SCP_SDCA_INT_SDCA_8) != 0 {
        btn_type = rt712_sdca_button_detect(rt712) as c_int;
    }
    if (*rt712).jack_type == 0 { btn_type = 0; }
    snd_soc_jack_report((*rt712).hs_jack, (*rt712).jack_type | btn_type, SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3);
    if btn_type != 0 {
        snd_soc_jack_report((*rt712).hs_jack, (*rt712).jack_type, SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3);
        mod_delayed_work(system_power_efficient_wq, &mut (*rt712).jack_btn_check_work, msecs_to_jiffies(200));
    }
}

unsafe fn rt712_sdca_btn_check_handler(work: *mut work_struct) {
    let rt712 = container_of_jack_btn_check_work(work);
    let mut btn_type: c_int = 0;
    let mut det_mode = 0; let mut offset = 0; let mut val = 0; let mut buf = [0u8; 3];
    let mut ret = regmap_read((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_GE49, RT712_SDCA_CTL_DETECTED_MODE, 0), &mut det_mode);
    if ret < 0 { pr_err_ratelimited!("IO error in %s, ret %d\n", cstr!("rt712_sdca_btn_check_handler"), ret); return; }
    if det_mode != 0 {
        ret = regmap_read((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_HID, RT712_SDCA_ENT_HID01, RT712_SDCA_CTL_HIDTX_MESSAGE_OFFSET, 0), &mut offset);
        if ret < 0 { pr_err_ratelimited!("IO error in %s, ret %d\n", cstr!("rt712_sdca_btn_check_handler"), ret); return; }
        for idx in 0..buf.len() {
            ret = regmap_read((*rt712).regmap, RT712_BUF_ADDR_HID1 + offset + idx as c_uint, &mut val);
            if ret < 0 { pr_err_ratelimited!("IO error in %s, ret %d\n", cstr!("rt712_sdca_btn_check_handler"), ret); return; }
            buf[idx] = (val & 0xff) as u8;
        }
        if buf[0] == 0x11 {
            match buf[1] & 0xf0 { 0x10 => btn_type |= SND_JACK_BTN_2, 0x20 => btn_type |= SND_JACK_BTN_3, 0x40 => btn_type |= SND_JACK_BTN_0, 0x80 => btn_type |= SND_JACK_BTN_1, _ => {} }
            match buf[2] { 0x01 | 0x10 => btn_type |= SND_JACK_BTN_2, 0x02 | 0x20 => btn_type |= SND_JACK_BTN_3, 0x04 | 0x40 => btn_type |= SND_JACK_BTN_0, 0x08 | 0x80 => btn_type |= SND_JACK_BTN_1, _ => {} }
        }
    } else { (*rt712).jack_type = 0; }
    snd_soc_jack_report((*rt712).hs_jack, (*rt712).jack_type | btn_type, SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3);
    if btn_type != 0 {
        snd_soc_jack_report((*rt712).hs_jack, (*rt712).jack_type, SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3);
        mod_delayed_work(system_power_efficient_wq, &mut (*rt712).jack_btn_check_work, msecs_to_jiffies(200));
    }
}

unsafe fn rt712_sdca_jack_init(rt712: *mut rt712_sdca_priv) {
    mutex_lock(&mut (*rt712).calibrate_mutex);
    if !(*rt712).hs_jack.is_null() {
        rt712_sdca_index_write(rt712, RT712_VENDOR_HDA_CTL, RT712_UMP_HID_CTL5, 0xfff0);
        rt712_sdca_index_update_bits(rt712, RT712_VENDOR_HDA_CTL, RT712_UMP_HID_CTL0, 0x1100, 0x1100);
        rt712_sdca_index_update_bits(rt712, RT712_VENDOR_HDA_CTL, RT712_UMP_HID_CTL7, 0xf000, 0x0000);
        rt712_sdca_index_update_bits(rt712, RT712_VENDOR_HDA_CTL, RT712_GE_RELATED_CTL1, 0x0c00, 0x0c00);
        rt712_sdca_index_update_bits(rt712, RT712_VENDOR_HDA_CTL, RT712_GE_RELATED_CTL2, 0x0020, 0x0000);
        match (*rt712).jd_src {
            x if x == RT712_JD1 => { if (*rt712).version_id == RT712_VA { rt712_sdca_index_write(rt712, RT712_VENDOR_REG, RT712_CC_DET1, 0x043a); } }
            _ => { if !(*rt712).component.is_null() { dev_warn!((*(*rt712).component).dev, "Wrong JD source\n"); } }
        }
        sdw_write_no_pm((*rt712).slave, SDW_SCP_SDCA_INTMASK1, SDW_SCP_SDCA_INTMASK_SDCA_0);
        sdw_write_no_pm((*rt712).slave, SDW_SCP_SDCA_INTMASK2, SDW_SCP_SDCA_INTMASK_SDCA_8);
        rt712_sdca_index_update_bits(rt712, RT712_VENDOR_HDA_CTL, RT712_GE_RELATED_CTL1, 0x0080, 0x0080);
        rt712_sdca_index_update_bits(rt712, RT712_VENDOR_HDA_CTL, RT712_GE_RELATED_CTL1, 0x0080, 0x0000);
    } else {
        rt712_sdca_index_update_bits(rt712, RT712_VENDOR_HDA_CTL, RT712_GE_RELATED_CTL1, 0x0c00, 0x0000);
    }
    mutex_unlock(&mut (*rt712).calibrate_mutex);
}

unsafe fn rt712_sdca_set_jack_detect(component: *mut snd_soc_component, hs_jack: *mut snd_soc_jack, _data: *mut c_void) -> c_int {
    let rt712 = snd_soc_component_get_drvdata(component);
    (*rt712).hs_jack = hs_jack;
    if !(*rt712).first_hw_init { return 0; }
    let ret = pm_runtime_resume_and_get((*component).dev);
    if ret < 0 {
        if ret != -EACCES { dev_err!((*component).dev, "%s: failed to resume %d\n", cstr!("rt712_sdca_set_jack_detect"), ret); return ret; }
        dev_dbg!((*component).dev, "%s: skipping jack init for now\n", cstr!("rt712_sdca_set_jack_detect"));
        return 0;
    }
    rt712_sdca_jack_init(rt712);
    pm_runtime_put_autosuspend((*component).dev);
    0
}

unsafe fn rt712_sdca_set_gain_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let rt712 = snd_soc_component_get_drvdata(component);
    let mut read_l = 0; let mut read_r = 0; let mut lvalue = 0; let mut rvalue = 0;
    let mut adc_vol_flag = 0u32;
    const interval_offset: c_uint = 0xc0; const tendB: c_uint = 0xa00;
    if !strstr((*ucontrol).id.name.as_ptr(), cstr!("FU0F Capture Volume")).is_null() { adc_vol_flag = 1; }
    regmap_read((*rt712).mbq_regmap, (*mc).reg, &mut lvalue);
    regmap_read((*rt712).mbq_regmap, (*mc).rreg, &mut rvalue);
    let mut gain_l_val = (*ucontrol).value.integer.value[0] as c_uint;
    if gain_l_val > (*mc).max { gain_l_val = (*mc).max; }
    if (*mc).shift == 8 { gain_l_val = gain_l_val.wrapping_mul(tendB); } else {
        gain_l_val = if adc_vol_flag != 0 { 0x1e00u32.wrapping_sub(((*mc).max - gain_l_val).wrapping_mul(interval_offset)) } else { 0u32.wrapping_sub(((*mc).max - gain_l_val).wrapping_mul(interval_offset)) };
        gain_l_val &= 0xffff;
    }
    let mut gain_r_val = (*ucontrol).value.integer.value[1] as c_uint;
    if gain_r_val > (*mc).max { gain_r_val = (*mc).max; }
    if (*mc).shift == 8 { gain_r_val = gain_r_val.wrapping_mul(tendB); } else {
        gain_r_val = if adc_vol_flag != 0 { 0x1e00u32.wrapping_sub(((*mc).max - gain_r_val).wrapping_mul(interval_offset)) } else { 0u32.wrapping_sub(((*mc).max - gain_r_val).wrapping_mul(interval_offset)) };
        gain_r_val &= 0xffff;
    }
    if lvalue == gain_l_val && rvalue == gain_r_val { return 0; }
    regmap_write((*rt712).mbq_regmap, (*mc).reg, gain_l_val);
    regmap_write((*rt712).mbq_regmap, (*mc).rreg, gain_r_val);
    regmap_read((*rt712).mbq_regmap, (*mc).reg, &mut read_l);
    regmap_read((*rt712).mbq_regmap, (*mc).rreg, &mut read_r);
    if read_r == gain_r_val && read_l == gain_l_val { 1 } else { -EIO }
}

unsafe fn rt712_sdca_set_gain_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt712 = snd_soc_component_get_drvdata(component);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let mut read_l = 0; let mut read_r = 0; let mut ctl_l = 0; let ctl_r: c_uint;
    let mut adc_vol_flag = 0u32;
    const interval_offset: c_uint = 0xc0; const tendB: c_uint = 0xa00;
    if !strstr((*ucontrol).id.name.as_ptr(), cstr!("FU0F Capture Volume")).is_null() { adc_vol_flag = 1; }
    regmap_read((*rt712).mbq_regmap, (*mc).reg, &mut read_l);
    regmap_read((*rt712).mbq_regmap, (*mc).rreg, &mut read_r);
    if (*mc).shift == 8 { ctl_l = read_l / tendB; } else if adc_vol_flag != 0 { ctl_l = (*mc).max - ((0x1e00u32.wrapping_sub(read_l) & 0xffff) / interval_offset); } else { ctl_l = (*mc).max - ((0u32.wrapping_sub(read_l) & 0xffff) / interval_offset); }
    if read_l != read_r {
        ctl_r = if (*mc).shift == 8 { read_r / tendB } else if adc_vol_flag != 0 { (*mc).max - ((0x1e00u32.wrapping_sub(read_r) & 0xffff) / interval_offset) } else { (*mc).max - ((0u32.wrapping_sub(read_r) & 0xffff) / interval_offset) };
    } else { ctl_r = ctl_l; }
    (*ucontrol).value.integer.value[0] = ctl_l as i64;
    (*ucontrol).value.integer.value[1] = ctl_r as i64;
    0
}

unsafe fn rt712_sdca_set_fu0f_capture_ctl(rt712: *mut rt712_sdca_priv) -> c_int {
    let ch_01 = if (*rt712).fu0f_dapm_mute || (*rt712).fu0f_mixer_l_mute { 0x01 } else { 0x00 };
    let ch_02 = if (*rt712).fu0f_dapm_mute || (*rt712).fu0f_mixer_r_mute { 0x01 } else { 0x00 };
    let mut err = regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_USER_FU0F, RT712_SDCA_CTL_FU_MUTE, CH_01), ch_01);
    if err < 0 { return err; }
    err = regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_USER_FU0F, RT712_SDCA_CTL_FU_MUTE, CH_02), ch_02);
    if err < 0 { return err; }
    0
}

unsafe fn rt712_sdca_fu0f_capture_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rt712 = snd_soc_component_get_drvdata(snd_kcontrol_chip(kcontrol));
    (*ucontrol).value.integer.value[0] = (!(*rt712).fu0f_mixer_l_mute) as i64;
    (*ucontrol).value.integer.value[1] = (!(*rt712).fu0f_mixer_r_mute) as i64;
    0
}

unsafe fn rt712_sdca_fu0f_capture_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rt712 = snd_soc_component_get_drvdata(snd_kcontrol_chip(kcontrol));
    if (*rt712).fu0f_mixer_l_mute == ((*ucontrol).value.integer.value[0] == 0) && (*rt712).fu0f_mixer_r_mute == ((*ucontrol).value.integer.value[1] == 0) { return 0; }
    (*rt712).fu0f_mixer_l_mute = (*ucontrol).value.integer.value[0] == 0;
    (*rt712).fu0f_mixer_r_mute = (*ucontrol).value.integer.value[1] == 0;
    let err = rt712_sdca_set_fu0f_capture_ctl(rt712);
    if err < 0 { return err; }
    1
}

unsafe fn rt712_sdca_set_fu05_playback_ctl(rt712: *mut rt712_sdca_priv) -> c_int {
    let ch_01 = if (*rt712).fu05_dapm_mute || (*rt712).fu05_mixer_l_mute { 0x01 } else { 0x00 };
    let ch_02 = if (*rt712).fu05_dapm_mute || (*rt712).fu05_mixer_r_mute { 0x01 } else { 0x00 };
    let mut err = regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_USER_FU05, RT712_SDCA_CTL_FU_MUTE, CH_01), ch_01);
    if err < 0 { return err; }
    err = regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_USER_FU05, RT712_SDCA_CTL_FU_MUTE, CH_02), ch_02);
    if err < 0 { return err; }
    0
}

unsafe fn rt712_sdca_fu05_playback_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rt712 = snd_soc_component_get_drvdata(snd_kcontrol_chip(kcontrol));
    (*ucontrol).value.integer.value[0] = (!(*rt712).fu05_mixer_l_mute) as i64;
    (*ucontrol).value.integer.value[1] = (!(*rt712).fu05_mixer_r_mute) as i64;
    0
}

unsafe fn rt712_sdca_fu05_playback_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rt712 = snd_soc_component_get_drvdata(snd_kcontrol_chip(kcontrol));
    if (*rt712).fu05_mixer_l_mute == ((*ucontrol).value.integer.value[0] == 0) && (*rt712).fu05_mixer_r_mute == ((*ucontrol).value.integer.value[1] == 0) { return 0; }
    (*rt712).fu05_mixer_l_mute = (*ucontrol).value.integer.value[0] == 0;
    (*rt712).fu05_mixer_r_mute = (*ucontrol).value.integer.value[1] == 0;
    let err = rt712_sdca_set_fu05_playback_ctl(rt712);
    if err < 0 { return err; }
    1
}

// TLV declarations and ASoC control/DAPM static tables translated from C macro
// initializers. Their concrete layout is supplied by ALSA bindings.
macro_rules! DECLARE_TLV_DB_SCALE { ($name:ident, $min:expr, $step:expr, $mute:expr) => { static $name: [c_uint; 4] = [$min as c_uint, $step as c_uint, $mute as c_uint, 0]; } }
DECLARE_TLV_DB_SCALE!(out_vol_tlv, -6525i32, 75, 0);
DECLARE_TLV_DB_SCALE!(mic_vol_tlv, -1725i32, 75, 0);
DECLARE_TLV_DB_SCALE!(boost_vol_tlv, 0, 1000, 0);
DECLARE_TLV_DB_SCALE!(in_vol_tlv, -1725i32, 75, 0);
DECLARE_TLV_DB_SCALE!(dmic_vol_tlv, 0, 1000, 0);

// static const struct snd_kcontrol_new rt712_sdca_controls[] = { SOC_DOUBLE_R_EXT_TLV(...), ... };
// static const struct snd_kcontrol_new rt712_sdca_spk_controls[] = { SOC_DOUBLE_R_EXT_TLV(...), ... };
// static const char * const adc_mux_text[] = { "MIC2", "LINE2" };
static adc_mux_text: [*const c_char; 2] = [cstr!("MIC2"), cstr!("LINE2")];
// SOC_ENUM_SINGLE_DECL(rt712_adc23_enum, SND_SOC_NOPM, 0, adc_mux_text);
// static const struct snd_kcontrol_new rt712_sdca_adc23_mux = SOC_DAPM_ENUM_EXT(...);

unsafe fn rt712_sdca_mux_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rt712 = snd_soc_component_get_drvdata(snd_soc_dapm_kcontrol_to_component(kcontrol));
    let mut val: c_uint = 0; let mask: c_uint = 0x3300;
    rt712_sdca_index_read(rt712, RT712_VENDOR_HDA_CTL, RT712_MIXER_CTL1, &mut val);
    val &= mask;
    match val { 0x3000 => val = 1, 0x0300 => val = 0, _ => {} }
    (*ucontrol).value.enumerated.item[0] = val;
    0
}

unsafe fn rt712_sdca_mux_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let rt712 = snd_soc_component_get_drvdata(component);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let item = (*ucontrol).value.enumerated.item.as_mut_ptr();
    if *item >= (*e).items { return -EINVAL; }
    let mask_sft = if *item == 0 { 12 } else if *item == 1 { 8 } else { return -EINVAL; };
    let mut val = 0;
    rt712_sdca_index_read(rt712, RT712_VENDOR_HDA_CTL, RT712_MIXER_CTL1, &mut val);
    val = (val >> mask_sft) & 0x3;
    if val == 0 { return 0; }
    rt712_sdca_index_write(rt712, RT712_VENDOR_HDA_CTL, RT712_MIXER_CTL1, 0x3fff);
    rt712_sdca_index_update_bits(rt712, RT712_VENDOR_HDA_CTL, RT712_MIXER_CTL1, 0x3 << mask_sft, 0);
    snd_soc_dapm_mux_update_power(dapm, kcontrol, *item, e, ptr::null_mut());
    1
}

unsafe fn rt712_sdca_fu05_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let rt712 = snd_soc_component_get_drvdata(snd_soc_dapm_to_component((*w).dapm));
    match event { SND_SOC_DAPM_POST_PMU => { (*rt712).fu05_dapm_mute = false; rt712_sdca_set_fu05_playback_ctl(rt712); }, SND_SOC_DAPM_PRE_PMD => { (*rt712).fu05_dapm_mute = true; rt712_sdca_set_fu05_playback_ctl(rt712); }, _ => {} }
    0
}

unsafe fn rt712_sdca_fu0f_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let rt712 = snd_soc_component_get_drvdata(snd_soc_dapm_to_component((*w).dapm));
    match event { SND_SOC_DAPM_POST_PMU => { (*rt712).fu0f_dapm_mute = false; rt712_sdca_set_fu0f_capture_ctl(rt712); }, SND_SOC_DAPM_PRE_PMD => { (*rt712).fu0f_dapm_mute = true; rt712_sdca_set_fu0f_capture_ctl(rt712); }, _ => {} }
    0
}

unsafe fn rt712_sdca_parse_dt(rt712: *mut rt712_sdca_priv, dev: *mut device) -> c_int {
    device_property_read_u32(dev, cstr!("realtek,jd-src"), &mut (*rt712).jd_src);
    0
}

unsafe fn rt712_sdca_probe(component: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let rt712 = snd_soc_component_get_drvdata(component);
    rt712_sdca_parse_dt(rt712, &mut (*(*rt712).slave).dev);
    (*rt712).component = component;
    if (*rt712).hw_id != RT712_DEV_ID_713 {
        // snd_soc_add_component_controls(component, rt712_sdca_spk_controls, ARRAY_SIZE(...));
        // snd_soc_dapm_new_controls(dapm, rt712_sdca_spk_dapm_widgets, ARRAY_SIZE(...));
        // snd_soc_dapm_add_routes(dapm, rt712_sdca_spk_dapm_routes, ARRAY_SIZE(...));
        let _ = dapm;
    }
    if !(*rt712).first_hw_init { return 0; }
    let ret = pm_runtime_resume((*component).dev);
    if ret < 0 && ret != -EACCES { return ret; }
    0
}

unsafe fn rt712_sdca_dmic_set_gain_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rt712 = snd_soc_component_get_drvdata(snd_kcontrol_chip(kcontrol));
    let p = (*kcontrol).private_value as *mut rt712_dmic_kctrl_priv;
    let mut adc_vol_flag = 0u32; const interval_offset: c_uint = 0xc0;
    if !strstr((*ucontrol).id.name.as_ptr(), cstr!("FU1E Capture Volume")).is_null() { adc_vol_flag = 1; }
    for i in 0..(*p).count {
        let mut regvalue = 0;
        regmap_read((*rt712).mbq_regmap, (*p).reg_base + i, &mut regvalue);
        let ctl = if adc_vol_flag == 0 { regvalue / 0x0a00 } else { (*p).max - ((0x1e00u32.wrapping_sub(regvalue) & 0xffff) / interval_offset) };
        (*ucontrol).value.integer.value[i as usize] = ctl as i64;
    }
    0
}

unsafe fn rt712_sdca_dmic_set_gain_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let p = (*kcontrol).private_value as *mut rt712_dmic_kctrl_priv;
    let rt712 = snd_soc_component_get_drvdata(component);
    let mut gain_val = [0u32; 4]; let mut regvalue = [0u32; 4];
    let mut adc_vol_flag = 0u32; let mut changed = 0; const interval_offset: c_uint = 0xc0;
    if !strstr((*ucontrol).id.name.as_ptr(), cstr!("FU1E Capture Volume")).is_null() { adc_vol_flag = 1; }
    for i in 0..(*p).count as usize {
        regmap_read((*rt712).mbq_regmap, (*p).reg_base + i as c_uint, &mut regvalue[i]);
        gain_val[i] = (*ucontrol).value.integer.value[i] as c_uint;
        if gain_val[i] > (*p).max { gain_val[i] = (*p).max; }
        if adc_vol_flag == 0 { gain_val[i] = gain_val[i].wrapping_mul(0x0a00); } else { gain_val[i] = 0x1e00u32.wrapping_sub(((*p).max - gain_val[i]).wrapping_mul(interval_offset)) & 0xffff; }
        if regvalue[i] != gain_val[i] { changed = 1; }
    }
    if changed == 0 { return 0; }
    for i in 0..(*p).count as usize {
        let err = regmap_write((*rt712).mbq_regmap, (*p).reg_base + i as c_uint, gain_val[i]);
        if err < 0 { dev_err!(&(*(*rt712).slave).dev, "0x%08x can't be set\n", (*p).reg_base + i as c_uint); }
    }
    changed
}

unsafe fn rt712_sdca_set_fu1e_capture_ctl(rt712: *mut rt712_sdca_priv) -> c_int {
    for i in 0..(*rt712).fu1e_mixer_mute.len() {
        let ch_mute = if (*rt712).fu1e_dapm_mute || (*rt712).fu1e_mixer_mute[i] { 0x01 } else { 0x00 };
        let err = regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_MUTE, CH_01) + i as c_uint, ch_mute);
        if err < 0 { return err; }
    }
    0
}

unsafe fn rt712_sdca_dmic_fu1e_capture_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rt712 = snd_soc_component_get_drvdata(snd_kcontrol_chip(kcontrol));
    let p = (*kcontrol).private_value as *mut rt712_dmic_kctrl_priv;
    for i in 0..(*p).count as usize { (*ucontrol).value.integer.value[i] = (!(*rt712).fu1e_mixer_mute[i]) as i64; }
    0
}

unsafe fn rt712_sdca_dmic_fu1e_capture_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rt712 = snd_soc_component_get_drvdata(snd_kcontrol_chip(kcontrol));
    let p = (*kcontrol).private_value as *mut rt712_dmic_kctrl_priv;
    let mut changed = 0;
    for i in 0..(*p).count as usize {
        if (*rt712).fu1e_mixer_mute[i] != ((*ucontrol).value.integer.value[i] == 0) { changed = 1; }
        (*rt712).fu1e_mixer_mute[i] = (*ucontrol).value.integer.value[i] == 0;
    }
    let err = rt712_sdca_set_fu1e_capture_ctl(rt712);
    if err < 0 { return err; }
    changed
}

unsafe fn rt712_sdca_fu_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let p = (*kcontrol).private_value as *mut rt712_dmic_kctrl_priv;
    (*uinfo).type_ = if (*p).max == 1 { SNDRV_CTL_ELEM_TYPE_BOOLEAN } else { SNDRV_CTL_ELEM_TYPE_INTEGER };
    (*uinfo).count = (*p).count;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = (*p).max as i64;
    0
}

// RT712_SDCA_PR_VALUE, RT712_SDCA_FU_CTRL, RT712_SDCA_EXT_TLV macros and the
// rt712_sdca_dmic_snd_controls table are preserved as macro-initializer intent.
// static const char * const adc_dmic_mux_text[] = { "DMIC1", "DMIC2" };
static adc_dmic_mux_text: [*const c_char; 2] = [cstr!("DMIC1"), cstr!("DMIC2")];

unsafe fn rt712_sdca_dmic_mux_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rt712 = snd_soc_component_get_drvdata(snd_soc_dapm_kcontrol_to_component(kcontrol));
    let mask_sft: c_uint;
    if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 0A Mux")).is_null() { mask_sft = 0; }
    else if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 0B Mux")).is_null() { mask_sft = 4; }
    else { return -EINVAL; }
    let mut val = 0;
    rt712_sdca_index_read(rt712, RT712_VENDOR_HDA_CTL, RT712_HDA_LEGACY_MUX_CTL0, &mut val);
    (*ucontrol).value.enumerated.item[0] = if ((val >> mask_sft) & 0xf) == 0x4 { 0 } else { 1 };
    0
}

unsafe fn rt712_sdca_dmic_mux_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let rt712 = snd_soc_component_get_drvdata(component);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let item = (*ucontrol).value.enumerated.item.as_mut_ptr();
    if *item >= (*e).items { return -EINVAL; }
    let mask_sft = if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 0A Mux")).is_null() { 0 } else if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 0B Mux")).is_null() { 4 } else { return -EINVAL; };
    let mut val = snd_soc_enum_item_to_val(e, *item) << (*e).shift_l;
    let mut val2 = 0;
    rt712_sdca_index_read(rt712, RT712_VENDOR_HDA_CTL, RT712_HDA_LEGACY_MUX_CTL0, &mut val2);
    val2 = ((0xf << mask_sft) & val2) >> mask_sft;
    if val == 0 { val = 0x4; } else if val >= 1 { val = 0xe; }
    let change = if val == val2 { 0 } else { 1 };
    if change != 0 { rt712_sdca_index_update_bits(rt712, RT712_VENDOR_HDA_CTL, RT712_HDA_LEGACY_MUX_CTL0, 0xf << mask_sft, val << mask_sft); }
    snd_soc_dapm_mux_update_power(dapm, kcontrol, *item, e, ptr::null_mut());
    change
}

unsafe fn rt712_sdca_dmic_fu1e_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let rt712 = snd_soc_component_get_drvdata(snd_soc_dapm_to_component((*w).dapm));
    match event { SND_SOC_DAPM_POST_PMU => { (*rt712).fu1e_dapm_mute = false; rt712_sdca_set_fu1e_capture_ctl(rt712); }, SND_SOC_DAPM_PRE_PMD => { (*rt712).fu1e_dapm_mute = true; rt712_sdca_set_fu1e_capture_ctl(rt712); }, _ => {} }
    0
}

unsafe fn rt712_sdca_pde40_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let rt712 = snd_soc_component_get_drvdata(snd_soc_dapm_to_component((*w).dapm));
    let ps0: c_uint = 0x0; let ps3: c_uint = 0x3;
    match event {
        SND_SOC_DAPM_POST_PMU => { regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_PDE40, RT712_SDCA_CTL_REQ_POWER_STATE, 0), ps0); }
        SND_SOC_DAPM_PRE_PMD => { regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_PDE40, RT712_SDCA_CTL_REQ_POWER_STATE, 0), ps3); }
        _ => {}
    }
    0
}

unsafe fn rt712_sdca_pde12_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let rt712 = snd_soc_component_get_drvdata(snd_soc_dapm_to_component((*w).dapm));
    let ps0: c_uint = 0x0; let ps3: c_uint = 0x3;
    match event {
        SND_SOC_DAPM_POST_PMU => { regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_PDE12, RT712_SDCA_CTL_REQ_POWER_STATE, 0), ps0); }
        SND_SOC_DAPM_PRE_PMD => { regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_PDE12, RT712_SDCA_CTL_REQ_POWER_STATE, 0), ps3); }
        _ => {}
    }
    0
}

unsafe fn rt712_sdca_pde23_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let rt712 = snd_soc_component_get_drvdata(snd_soc_dapm_to_component((*w).dapm));
    let ps0: c_uint = 0x0; let ps3: c_uint = 0x3;
    match event {
        SND_SOC_DAPM_POST_PMU => { regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_AMP, RT712_SDCA_ENT_PDE23, RT712_SDCA_CTL_REQ_POWER_STATE, 0), ps0); }
        SND_SOC_DAPM_PRE_PMD => { regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_AMP, RT712_SDCA_ENT_PDE23, RT712_SDCA_CTL_REQ_POWER_STATE, 0), ps3); }
        _ => {}
    }
    0
}

unsafe fn rt712_sdca_dmic_pde11_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let rt712 = snd_soc_component_get_drvdata(snd_soc_dapm_to_component((*w).dapm));
    let ps0: c_uint = 0x0; let ps3: c_uint = 0x3;
    match event {
        SND_SOC_DAPM_POST_PMU => { regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_PDE11, RT712_SDCA_CTL_REQ_POWER_STATE, 0), ps0); }
        SND_SOC_DAPM_PRE_PMD => { regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_PDE11, RT712_SDCA_CTL_REQ_POWER_STATE, 0), ps3); }
        _ => {}
    }
    0
}

unsafe fn rt712_sdca_dmic_probe(component: *mut snd_soc_component) -> c_int {
    let rt712 = snd_soc_component_get_drvdata(component);
    (*rt712).dmic_component = component;
    if !(*rt712).first_hw_init { return 0; }
    let ret = pm_runtime_resume((*component).dev);
    if ret < 0 && ret != -EACCES { return ret; }
    0
}

unsafe fn rt712_sdca_set_sdw_stream(dai: *mut snd_soc_dai, sdw_stream: *mut c_void, direction: c_int) -> c_int {
    snd_soc_dai_dma_data_set(dai, direction, sdw_stream);
    0
}

unsafe fn rt712_sdca_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    snd_soc_dai_set_dma_data(dai, substream, ptr::null_mut());
}

unsafe fn rt712_sdca_pcm_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let rt712 = snd_soc_component_get_drvdata(component);
    let mut stream_config = sdw_stream_config { frame_rate: 0, ch_count: 0, bps: 0, direction: sdw_data_direction::SDW_DATA_DIR_RX };
    let mut port_config = sdw_port_config { ch_mask: 0, num: 0 };
    let direction: sdw_data_direction;
    let port: c_int;
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    if sdw_stream.is_null() || (*rt712).slave.is_null() { return -EINVAL; }
    if (*dai).id == RT712_AIF3 as c_int && (*rt712).version_id == RT712_VA { return -EINVAL; }
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        direction = sdw_data_direction::SDW_DATA_DIR_RX;
        if (*dai).id == RT712_AIF1 as c_int { port = 1; } else if (*dai).id == RT712_AIF2 as c_int { port = 3; } else { return -EINVAL; }
    } else {
        direction = sdw_data_direction::SDW_DATA_DIR_TX;
        if (*dai).id == RT712_AIF1 as c_int { port = 4; } else if (*dai).id == RT712_AIF3 as c_int { port = 8; } else { return -EINVAL; }
    }
    stream_config.frame_rate = params_rate(params);
    stream_config.ch_count = params_channels(params) as c_uint;
    stream_config.bps = snd_pcm_format_width(params_format(params));
    stream_config.direction = direction;
    let num_channels = params_channels(params);
    port_config.ch_mask = GENMASK!((num_channels - 1) as u32, 0);
    port_config.num = port;
    let retval = sdw_stream_add_slave((*rt712).slave, &mut stream_config, &mut port_config, 1, sdw_stream);
    if retval != 0 { dev_err!((*dai).dev, "%s: Unable to configure port\n", cstr!("rt712_sdca_pcm_hw_params")); return retval; }
    if params_channels(params) > 16 { dev_err!((*component).dev, "%s: Unsupported channels %d\n", cstr!("rt712_sdca_pcm_hw_params"), params_channels(params)); return -EINVAL; }
    let sampling_rate = match params_rate(params) {
        44100 => RT712_SDCA_RATE_44100HZ,
        48000 => RT712_SDCA_RATE_48000HZ,
        96000 => RT712_SDCA_RATE_96000HZ,
        192000 => RT712_SDCA_RATE_192000HZ,
        _ => { dev_err!((*component).dev, "%s: Rate %d is not supported\n", cstr!("rt712_sdca_pcm_hw_params"), params_rate(params)); return -EINVAL; }
    };
    match (*dai).id {
        x if x == RT712_AIF1 as c_int => {
            regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_CS01, RT712_SDCA_CTL_SAMPLE_FREQ_INDEX, 0), sampling_rate);
            regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_CS11, RT712_SDCA_CTL_SAMPLE_FREQ_INDEX, 0), sampling_rate);
        }
        x if x == RT712_AIF2 as c_int => { regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_AMP, RT712_SDCA_ENT_CS31, RT712_SDCA_CTL_SAMPLE_FREQ_INDEX, 0), sampling_rate); }
        x if x == RT712_AIF3 as c_int => {
            regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_CS1F, RT712_SDCA_CTL_SAMPLE_FREQ_INDEX, 0), sampling_rate);
            regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_CS1C, RT712_SDCA_CTL_SAMPLE_FREQ_INDEX, 0), sampling_rate);
        }
        _ => { dev_err!((*component).dev, "%s: Wrong DAI id\n", cstr!("rt712_sdca_pcm_hw_params")); return -EINVAL; }
    }
    0
}

unsafe fn rt712_sdca_pcm_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let rt712 = snd_soc_component_get_drvdata(component);
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    if (*rt712).slave.is_null() { return -EINVAL; }
    sdw_stream_remove_slave((*rt712).slave, sdw_stream);
    0
}

pub unsafe extern "C" fn rt712_sdca_init(dev: *mut device, regmap: *mut regmap, mbq_regmap: *mut regmap, slave: *mut sdw_slave) -> c_int {
    let rt712 = devm_kzalloc(dev, core::mem::size_of::<rt712_sdca_priv>(), GFP_KERNEL) as *mut rt712_sdca_priv;
    if rt712.is_null() { return -ENOMEM; }
    dev_set_drvdata(dev, rt712 as *mut c_void);
    (*rt712).slave = slave; (*rt712).regmap = regmap; (*rt712).mbq_regmap = mbq_regmap;
    regcache_cache_only((*rt712).regmap, true);
    regcache_cache_only((*rt712).mbq_regmap, true);
    mutex_init(&mut (*rt712).calibrate_mutex);
    mutex_init(&mut (*rt712).disable_irq_lock);
    INIT_DELAYED_WORK!(&mut (*rt712).jack_detect_work, rt712_sdca_jack_detect_handler);
    INIT_DELAYED_WORK!(&mut (*rt712).jack_btn_check_work, rt712_sdca_btn_check_handler);
    (*rt712).hw_init = false; (*rt712).first_hw_init = false;
    (*rt712).fu0f_dapm_mute = true; (*rt712).fu0f_mixer_l_mute = true; (*rt712).fu0f_mixer_r_mute = true;
    (*rt712).fu1e_dapm_mute = true; (*rt712).fu1e_mixer_mute = [true; 4];
    (*rt712).fu05_dapm_mute = true; (*rt712).fu05_mixer_l_mute = false; (*rt712).fu05_mixer_r_mute = false;
    (*rt712).jd_src = RT712_JD1;
    let mut ret: c_int;
    if (*slave).id.part_id != RT712_PART_ID_713 {
        ret = devm_snd_soc_register_component(dev, &soc_sdca_dev_rt712 as *const _ as *const c_void, rt712_sdca_dai.as_mut_ptr() as *mut c_void, ARRAY_SIZE!(rt712_sdca_dai));
    } else {
        ret = devm_snd_soc_register_component(dev, &soc_sdca_dev_rt712 as *const _ as *const c_void, rt712_sdca_dai.as_mut_ptr() as *mut c_void, 1);
    }
    if ret < 0 { return ret; }
    if sdca_device_quirk_match(slave, SDCA_QUIRKS_RT712_VB) {
        ret = devm_snd_soc_register_component(dev, &soc_sdca_dev_rt712_dmic as *const _ as *const c_void, rt712_sdca_dmic_dai.as_mut_ptr() as *mut c_void, ARRAY_SIZE!(rt712_sdca_dmic_dai));
        if ret < 0 { return ret; }
        (*rt712).dmic_function_found = true;
    }
    pm_runtime_set_autosuspend_delay(dev, 3000);
    pm_runtime_use_autosuspend(dev);
    pm_runtime_mark_last_busy(dev);
    pm_runtime_enable(dev);
    dev_dbg!(dev, "%s\n", cstr!("rt712_sdca_init"));
    0
}

unsafe fn rt712_sdca_va_io_init(rt712: *mut rt712_sdca_priv) {
    let mut ret = 0; let mut hibernation_flag = 0; let dev = &mut (*(*rt712).slave).dev as *mut device;
    rt712_sdca_index_write(rt712, RT712_VENDOR_REG, RT712_ANALOG_BIAS_CTL3, 0xaa81);
    rt712_sdca_index_write(rt712, RT712_VENDOR_REG, RT712_LDO2_3_CTL1, 0xa1e0);
    rt712_sdca_index_write(rt712, RT712_VENDOR_IMS_DRE, RT712_HP_DETECT_RLDET_CTL1, 0x0000);
    rt712_sdca_index_write(rt712, RT712_VENDOR_IMS_DRE, RT712_HP_DETECT_RLDET_CTL2, 0x0000);
    rt712_sdca_index_write(rt712, RT712_VENDOR_ANALOG_CTL, RT712_MISC_POWER_CTL7, 0x0000);
    regmap_write((*rt712).regmap, RT712_RC_CAL, 0x23);
    rt712_sdca_index_read(rt712, RT712_VENDOR_REG, RT712_SW_CONFIG1, &mut hibernation_flag);
    if hibernation_flag == 0 { ret = rt712_sdca_calibration(rt712); if ret < 0 { dev_err!(dev, "%s, calibration failed!\n", cstr!("rt712_sdca_va_io_init")); } }
    rt712_sdca_index_update_bits(rt712, RT712_VENDOR_HDA_CTL, RT712_MIXER_CTL1, 0x3000, 0x0000);
    rt712_sdca_index_write(rt712, RT712_VENDOR_HDA_CTL, RT712_ADC0A_08_PDE_FLOAT_CTL, 0x1112);
    rt712_sdca_index_write(rt712, RT712_VENDOR_HDA_CTL, RT712_MIC2_LINE2_PDE_FLOAT_CTL, 0x3412);
    rt712_sdca_index_write(rt712, RT712_VENDOR_HDA_CTL, RT712_DAC03_HP_PDE_FLOAT_CTL, 0x4040);
    rt712_sdca_index_update_bits(rt712, RT712_VENDOR_HDA_CTL, RT712_HDA_LEGACY_GPIO_WAKE_EN_CTL, 0x0001, 0x0000);
    regmap_write((*rt712).regmap, 0x2f50, 0x00);
    regmap_write((*rt712).regmap, 0x2f54, 0x00);
    regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_IT09, RT712_SDCA_CTL_VENDOR_DEF, 0), 0x01);
    if (*rt712).hw_id != RT712_DEV_ID_713 {
        rt712_sdca_index_write(rt712, RT712_VENDOR_HDA_CTL, RT712_AMP_PDE_FLOAT_CTL, 0x2323);
        rt712_sdca_index_write(rt712, RT712_VENDOR_HDA_CTL, RT712_EAPD_CTL, 0x0002);
        regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_AMP, RT712_SDCA_ENT_OT23, RT712_SDCA_CTL_VENDOR_DEF, 0), 0x04);
    }
}

unsafe fn rt712_sdca_vb_io_init(rt712: *mut rt712_sdca_priv) {
    let mut ret = 0;
    let mut jack_func_status = 0; let mut mic_func_status = 0; let mut amp_func_status = 0;
    let dev = &mut (*(*rt712).slave).dev as *mut device;
    regmap_read((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT0, RT712_SDCA_CTL_FUNC_STATUS, 0), &mut jack_func_status);
    regmap_read((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT0, RT712_SDCA_CTL_FUNC_STATUS, 0), &mut mic_func_status);
    regmap_read((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_AMP, RT712_SDCA_ENT0, RT712_SDCA_CTL_FUNC_STATUS, 0), &mut amp_func_status);
    dev_dbg!(dev, "%s jack/mic/amp func_status=0x%x, 0x%x, 0x%x\n", cstr!("rt712_sdca_vb_io_init"), jack_func_status, mic_func_status, amp_func_status);
    rt712_sdca_index_write(rt712, RT712_VENDOR_REG, RT712_JD_CTL3, 0x7778);
    if (mic_func_status & FUNCTION_NEEDS_INITIALIZATION) != 0 || !(*rt712).first_hw_init {
        rt712_sdca_index_write(rt712, RT712_VENDOR_HDA_CTL, RT712_DMIC2_FU_IT_FLOAT_CTL, 0x1526);
        rt712_sdca_index_write(rt712, RT712_VENDOR_HDA_CTL, RT712_DMIC2_FU_CH12_FLOAT_CTL, 0x0304);
        rt712_sdca_index_write(rt712, RT712_VENDOR_HDA_CTL, RT712_ADC0A_CS_ADC0B_FU_FLOAT_CTL, 0x1f1e);
        rt712_sdca_index_write(rt712, RT712_VENDOR_HDA_CTL, RT712_ADC0B_FU_CH12_FLOAT_CTL, 0x0304);
        rt712_sdca_index_write(rt712, RT712_VENDOR_HDA_CTL, RT712_HDA_LEGACY_CONFIG_CTL0, 0x8010);
        regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_IT11, RT712_SDCA_CTL_VENDOR_DEF, 0), 0x01);
        rt712_sdca_index_write(rt712, RT712_ULTRA_SOUND_DET, RT712_ULTRA_SOUND_DETECTOR6, 0x3200);
        regmap_write((*rt712).regmap, RT712_RC_CAL, 0x23);
        regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT0, RT712_SDCA_CTL_FUNC_STATUS, 0), FUNCTION_NEEDS_INITIALIZATION);
    }
    if (jack_func_status & FUNCTION_NEEDS_INITIALIZATION) != 0 || !(*rt712).first_hw_init {
        rt712_sdca_index_write(rt712, RT712_VENDOR_IMS_DRE, RT712_SEL_VEE2_HP_CTL1, 0x042a);
        rt712_sdca_index_write(rt712, RT712_CHARGE_PUMP, RT712_HP_DET_CTL3, 0x1fff);
        rt712_sdca_index_write(rt712, RT712_VENDOR_REG, RT712_IO_CTL, 0xec67);
        rt712_sdca_index_write(rt712, RT712_VENDOR_REG, RT712_ANALOG_BIAS_CTL3, 0xaa81);
        rt712_sdca_index_write(rt712, RT712_VENDOR_REG, RT712_LDO2_3_CTL1, 0xa1e0);
        rt712_sdca_index_write(rt712, RT712_VENDOR_IMS_DRE, RT712_HP_DETECT_RLDET_CTL1, 0x0000);
        rt712_sdca_index_write(rt712, RT712_VENDOR_IMS_DRE, RT712_HP_DETECT_RLDET_CTL2, 0x0000);
        regmap_write((*rt712).regmap, RT712_RC_CAL, 0x23);
        rt712_sdca_index_write(rt712, RT712_VENDOR_REG, RT712_JD_CTL1, 0x2802);
        rt712_sdca_index_write(rt712, RT712_VENDOR_REG, RT712_CLASSD_AMP_CTL6, 0xf215);
        ret = rt712_sdca_calibration(rt712);
        if ret < 0 { dev_err!(dev, "%s, calibration failed!\n", cstr!("rt712_sdca_vb_io_init")); }
        rt712_sdca_index_update_bits(rt712, RT712_VENDOR_HDA_CTL, RT712_MIXER_CTL1, 0x3000, 0x0000);
        regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_IT09, RT712_SDCA_CTL_VENDOR_DEF, 0), 0x01);
        rt712_sdca_index_write(rt712, RT712_VENDOR_HDA_CTL, RT712_MISC_CTL_FOR_UAJ, 0x0003);
        regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT0, RT712_SDCA_CTL_FUNC_STATUS, 0), FUNCTION_NEEDS_INITIALIZATION);
    }
    if (amp_func_status & FUNCTION_NEEDS_INITIALIZATION) != 0 || !(*rt712).first_hw_init {
        if (*rt712).hw_id != RT712_DEV_ID_713 {
            rt712_sdca_index_write(rt712, RT712_VENDOR_REG, RT712_IO_CTL, 0xec63);
            rt712_sdca_index_write(rt712, RT712_VENDOR_REG, RT712_CLASSD_AMP_CTL1, 0xfff5);
            rt712_sdca_index_write(rt712, RT712_VENDOR_HDA_CTL, RT712_EAPD_CTL, 0x0002);
            regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_AMP, RT712_SDCA_ENT_OT23, RT712_SDCA_CTL_VENDOR_DEF, 0), 0x04);
        }
        regmap_write((*rt712).regmap, SDW_SDCA_CTL!(FUNC_NUM_AMP, RT712_SDCA_ENT0, RT712_SDCA_CTL_FUNC_STATUS, 0), FUNCTION_NEEDS_INITIALIZATION);
    }
}

unsafe fn rt712_sdca_reset(rt712: *mut rt712_sdca_priv) {
    rt712_sdca_index_update_bits(rt712, RT712_VENDOR_REG, RT712_PARA_VERB_CTL, RT712_HIDDEN_REG_SW_RESET, RT712_HIDDEN_REG_SW_RESET);
    rt712_sdca_index_update_bits(rt712, RT712_VENDOR_HDA_CTL, RT712_HDA_LEGACY_RESET_CTL, 0x1, 0x1);
}

pub unsafe extern "C" fn rt712_sdca_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int {
    let rt712 = dev_get_drvdata(dev) as *mut rt712_sdca_priv;
    let mut val = 0;
    let prop = &mut (*slave).prop as *mut sdw_slave_prop;
    (*rt712).disable_irq = false;
    if (*rt712).hw_init { return 0; }
    regcache_cache_only((*rt712).regmap, false);
    regcache_cache_only((*rt712).mbq_regmap, false);
    if (*rt712).first_hw_init {
        regcache_cache_bypass((*rt712).regmap, true);
        regcache_cache_bypass((*rt712).mbq_regmap, true);
    } else {
        pm_runtime_set_active(&mut (*slave).dev);
    }
    pm_runtime_get_noresume(&mut (*slave).dev);
    rt712_sdca_reset(rt712);
    rt712_sdca_index_read(rt712, RT712_VENDOR_REG, RT712_JD_PRODUCT_NUM, &mut val);
    (*rt712).hw_id = (val & 0xf000) >> 12;
    (*rt712).version_id = (val & 0x0f00) >> 8;
    dev_dbg!(&mut (*slave).dev, "%s hw_id=0x%x, version_id=0x%x\n", cstr!("rt712_sdca_io_init"), (*rt712).hw_id, (*rt712).version_id);
    if (*rt712).version_id == RT712_VA {
        if (*rt712).dmic_function_found {
            dev_err!(&mut (*slave).dev, "%s RT712 VA detected but SMART_MIC function exposed in ACPI\n", cstr!("rt712_sdca_io_init"));
            pm_runtime_put_autosuspend(&mut (*slave).dev);
            return 0;
        }
        rt712_sdca_va_io_init(rt712);
    } else {
        if !(*rt712).dmic_function_found { dev_warn!(&mut (*slave).dev, "%s RT712 VB detected but no SMART_MIC function exposed in ACPI\n", cstr!("rt712_sdca_io_init")); }
        (*prop).lane_control_support = true;
        rt712_sdca_vb_io_init(rt712);
    }
    if !(*rt712).hs_jack.is_null() { rt712_sdca_jack_init(rt712); }
    rt712_sdca_index_write(rt712, RT712_VENDOR_REG, RT712_SW_CONFIG1, 0x0001);
    if (*rt712).first_hw_init {
        regcache_cache_bypass((*rt712).regmap, false); regcache_mark_dirty((*rt712).regmap);
        regcache_cache_bypass((*rt712).mbq_regmap, false); regcache_mark_dirty((*rt712).mbq_regmap);
    } else { (*rt712).first_hw_init = true; }
    (*rt712).hw_init = true;
    dev_dbg!(&mut (*slave).dev, "%s hw_init complete\n", cstr!("rt712_sdca_io_init"));
    pm_runtime_put_autosuspend(&mut (*slave).dev);
    0
}

// The remaining driver tables and macro-only declarations from the C source are
// preserved here as dependency-shaped Rust items/comments because their actual
// layouts are provided by kernel/ALSA bindings:
// - rt712_sdca_controls, rt712_sdca_spk_controls
// - rt712_spk_l_dac, rt712_spk_r_dac
// - rt712_sdca_dapm_widgets, rt712_sdca_audio_map
// - rt712_sdca_spk_dapm_widgets, rt712_sdca_spk_dapm_routes
// - rt712_sdca_dmic_snd_controls
// - rt712_sdca_dmic_dapm_widgets, rt712_sdca_dmic_audio_map
// - soc_sdca_dev_rt712, soc_sdca_dev_rt712_dmic
// - rt712_sdca_ops, rt712_sdca_dai, rt712_sdca_dmic_dai
// - MODULE_DESCRIPTION("ASoC RT712 SDCA SDW driver")
// - MODULE_AUTHOR("Shuming Fan <shumingf@realtek.com>")
// - MODULE_LICENSE("GPL")

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
