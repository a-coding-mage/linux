// SPDX-License-Identifier: GPL-2.0-only
//
// rt721-sdca.c -- rt721 SDCA ALSA SoC audio driver
//
// Copyright(c) 2024 Realtek Semiconductor Corp.
//
// Rust translation of the isolated C implementation source. Kernel/ALSA/SDW
// headers and local codec headers are expected to provide the referenced
// types, constants, functions, and construction macros.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type bool_t = bool;

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct delayed_work {
    pub work: work_struct,
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
}
#[repr(C)]
pub struct snd_soc_card {
    pub instantiated: bool_t,
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
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
pub struct soc_mixer_control {
    pub reg: c_uint,
    pub rreg: c_uint,
    pub shift: c_uint,
    pub max: c_uint,
}
#[repr(C)]
pub struct rt721_sdca_dmic_kctrl_priv {
    pub reg_base: c_uint,
    pub count: c_uint,
    pub max: c_uint,
}
#[repr(C)]
pub struct snd_ctl_elem_id {
    pub name: [c_char; 44],
}
#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_uint; 128],
}
#[repr(C)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}
#[repr(C)]
pub union snd_ctl_elem_value_data {
    pub integer: core::mem::ManuallyDrop<snd_ctl_elem_value_integer>,
    pub enumerated: core::mem::ManuallyDrop<snd_ctl_elem_value_enumerated>,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub id: snd_ctl_elem_id,
    pub value: snd_ctl_elem_value_data,
}
#[repr(C)]
pub struct snd_ctl_elem_info_integer {
    pub min: c_int,
    pub max: c_int,
}
#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: core::mem::ManuallyDrop<snd_ctl_elem_info_integer>,
}
#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
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
pub struct soc_enum {
    pub items: c_uint,
    pub shift_l: c_uint,
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
pub struct snd_soc_dapm_widget_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
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
    pub frame_rate: c_uint,
    pub ch_count: c_uint,
    pub bps: c_uint,
    pub direction: sdw_data_direction,
}
pub type sdw_data_direction = c_uint;
#[repr(C)]
pub struct sdw_port_config {
    pub ch_mask: c_uint,
    pub num: c_int,
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
    pub formats: c_uint,
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
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct rt721_sdca_priv {
    pub hs_jack: *mut snd_soc_jack,
    pub component: *mut snd_soc_component,
    pub scp_sdca_stat1: c_uint,
    pub scp_sdca_stat2: c_uint,
    pub jack_type: c_int,
    pub regmap: *mut regmap,
    pub mbq_regmap: *mut regmap,
    pub slave: *mut sdw_slave,
    pub jack_detect_work: delayed_work,
    pub jack_btn_check_work: delayed_work,
    pub calibrate_mutex: mutex,
    pub disable_irq_lock: mutex,
    pub jd_src: c_uint,
    pub hw_init: bool_t,
    pub first_hw_init: bool_t,
    pub fu1e_dapm_mute: bool_t,
    pub fu1e_mixer_mute: [bool_t; 4],
    pub fu0f_dapm_mute: bool_t,
    pub fu0f_mixer_l_mute: bool_t,
    pub fu0f_mixer_r_mute: bool_t,
    pub disable_irq: bool_t,
}

extern "C" {
    static system_power_efficient_wq: *mut c_void;

    fn container_of_rt721_jack_detect(work: *mut work_struct) -> *mut rt721_sdca_priv;
    fn container_of_rt721_jack_btn_check(work: *mut work_struct) -> *mut rt721_sdca_priv;
    fn rt_sdca_headset_detect(regmap: *mut regmap, entity: c_uint) -> c_int;
    fn rt_sdca_button_detect(regmap: *mut regmap, entity: c_uint, addr: c_uint, hid_id: c_uint) -> c_int;
    fn rt_sdca_btn_type(buf: *mut u8) -> c_int;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn mod_delayed_work(wq: *mut c_void, dwork: *mut delayed_work, delay: c_uint) -> bool_t;
    fn msecs_to_jiffies(msecs: c_uint) -> c_uint;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn rt_sdca_index_write(map: *mut regmap, nid: c_uint, reg: c_uint, val: c_uint) -> c_int;
    fn rt_sdca_index_read(map: *mut regmap, nid: c_uint, reg: c_uint, val: *mut c_uint) -> c_int;
    fn rt_sdca_index_update_bits(map: *mut regmap, nid: c_uint, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut rt721_sdca_priv;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn snd_soc_dapm_kcontrol_to_component(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dapm_kcontrol_to_dapm(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dapm_context;
    fn snd_soc_enum_item_to_val(e: *mut soc_enum, item: c_uint) -> c_uint;
    fn snd_soc_dapm_mux_update_power(dapm: *mut snd_soc_dapm_context, kcontrol: *mut snd_kcontrol, item: c_uint, e: *mut soc_enum, update: *mut c_void) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn msleep(msecs: c_uint);
    fn device_property_read_u32(dev: *mut device, propname: *const c_char, val: *mut c_uint) -> c_int;
    fn pm_runtime_resume(dev: *mut device) -> c_int;
    fn snd_soc_dai_dma_data_set(dai: *mut snd_soc_dai, direction: c_int, data: *mut c_void);
    fn snd_soc_dai_set_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream, data: *mut c_void);
    fn snd_soc_dai_get_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream) -> *mut sdw_stream_runtime;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_format_width(format: c_int) -> c_uint;
    fn sdw_stream_add_slave(slave: *mut sdw_slave, stream_config: *mut sdw_stream_config, port_config: *mut sdw_port_config, num_ports: c_uint, stream: *mut sdw_stream_runtime) -> c_int;
    fn sdw_stream_remove_slave(slave: *mut sdw_slave, stream: *mut sdw_stream_runtime);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut rt721_sdca_priv;
    fn regcache_cache_only(map: *mut regmap, enable: bool_t);
    fn regcache_cache_bypass(map: *mut regmap, enable: bool_t);
    fn regcache_mark_dirty(map: *mut regmap);
    fn mutex_init(lock: *mut mutex);
    fn INIT_DELAYED_WORK(work: *mut delayed_work, func: unsafe extern "C" fn(*mut work_struct));
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device) -> c_int;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}
macro_rules! SDW_SDCA_CTL { ($($t:tt)*) => { SDW_SDCA_CTL($($t)*) }; }
macro_rules! GENMASK { ($h:expr, $l:expr) => { (((!0u32) << ($l)) & ((!0u32) >> (31 - ($h)))) }; }
macro_rules! DECLARE_TLV_DB_SCALE { ($name:ident, $min:expr, $step:expr, $mute:expr) => { static $name: [c_uint; 4] = [0, $min as c_uint, $step as c_uint, $mute as c_uint]; }; }

extern "C" {
    fn SDW_SDCA_CTL(func: c_uint, ent: c_uint, ctl: c_uint, ch: c_uint) -> c_uint;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err_ratelimited(fmt: *const c_char, ...);
}

const EACCES: c_int = 13;
const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;

unsafe extern "C" fn rt721_sdca_jack_detect_handler(work: *mut work_struct) {
    let rt721 = container_of_rt721_jack_detect(work);
    let mut btn_type: c_int = 0;

    if (*rt721).hs_jack.is_null() {
        return;
    }
    if (*(*rt721).component).card.is_null() || !(*(*(*rt721).component).card).instantiated {
        return;
    }

    /* SDW_SCP_SDCA_INT_SDCA_6 is used for jack detection */
    if (*rt721).scp_sdca_stat1 & SDW_SCP_SDCA_INT_SDCA_0 != 0 {
        (*rt721).jack_type = rt_sdca_headset_detect((*rt721).regmap, RT721_SDCA_ENT_GE49);
        if (*rt721).jack_type < 0 {
            return;
        }
    }

    /* SDW_SCP_SDCA_INT_SDCA_8 is used for button detection */
    if (*rt721).scp_sdca_stat2 & SDW_SCP_SDCA_INT_SDCA_8 != 0 {
        btn_type = rt_sdca_button_detect((*rt721).regmap, RT721_SDCA_ENT_HID01, RT721_BUF_ADDR_HID1, RT721_SDCA_HID_ID);
    }

    if (*rt721).jack_type == 0 {
        btn_type = 0;
    }

    dev_dbg(&mut (*(*rt721).slave).dev, cstr!("in %s, jack_type=%d\n"), cstr!("rt721_sdca_jack_detect_handler"), (*rt721).jack_type);
    dev_dbg(&mut (*(*rt721).slave).dev, cstr!("in %s, btn_type=0x%x\n"), cstr!("rt721_sdca_jack_detect_handler"), btn_type);
    dev_dbg(&mut (*(*rt721).slave).dev, cstr!("in %s, scp_sdca_stat1=0x%x, scp_sdca_stat2=0x%x\n"), cstr!("rt721_sdca_jack_detect_handler"), (*rt721).scp_sdca_stat1, (*rt721).scp_sdca_stat2);

    snd_soc_jack_report((*rt721).hs_jack, (*rt721).jack_type | btn_type,
        SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3);

    if btn_type != 0 {
        /* button released */
        snd_soc_jack_report((*rt721).hs_jack, (*rt721).jack_type,
            SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3);
        mod_delayed_work(system_power_efficient_wq, &mut (*rt721).jack_btn_check_work, msecs_to_jiffies(200));
    }
}

unsafe extern "C" fn rt721_sdca_btn_check_handler(work: *mut work_struct) {
    let rt721 = container_of_rt721_jack_btn_check(work);
    let mut btn_type: c_int = 0;
    let mut ret: c_int;
    let mut det_mode: c_uint = 0;
    let mut offset: c_uint = 0;
    let mut val: c_uint = 0;
    let mut buf = [0u8; 3];

    ret = regmap_read((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_GE49, RT721_SDCA_CTL_DETECTED_MODE, 0), &mut det_mode);
    if ret < 0 {
        pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("rt721_sdca_btn_check_handler"), ret);
        return;
    }

    /* pin attached */
    if det_mode != 0 {
        /* read UMP message offset */
        ret = regmap_read((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_HID, RT721_SDCA_ENT_HID01, RT721_SDCA_CTL_HIDTX_MESSAGE_OFFSET, 0), &mut offset);
        if ret < 0 {
            pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("rt721_sdca_btn_check_handler"), ret);
            return;
        }
        for idx in 0..buf.len() {
            ret = regmap_read((*rt721).regmap, RT721_BUF_ADDR_HID1 + offset + idx as c_uint, &mut val);
            if ret < 0 {
                pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("rt721_sdca_btn_check_handler"), ret);
                return;
            }
            buf[idx] = (val & 0xff) as u8;
        }
        /* Report ID for HID1 */
        if buf[0] == 0x11 {
            btn_type = rt_sdca_btn_type(buf[1..].as_mut_ptr());
        }
    } else {
        (*rt721).jack_type = 0;
    }

    dev_dbg(&mut (*(*rt721).slave).dev, cstr!("%s, btn_type=0x%x\n"), cstr!("rt721_sdca_btn_check_handler"), btn_type);
    snd_soc_jack_report((*rt721).hs_jack, (*rt721).jack_type | btn_type,
        SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3);
    if btn_type != 0 {
        /* button released */
        snd_soc_jack_report((*rt721).hs_jack, (*rt721).jack_type,
            SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3);
        mod_delayed_work(system_power_efficient_wq, &mut (*rt721).jack_btn_check_work, msecs_to_jiffies(200));
    }
}

unsafe fn rt721_sdca_dmic_preset(rt721: *mut rt721_sdca_priv) {
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_VENDOR_ANA_CTL, RT721_MISC_POWER_CTL31, 0x8000);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_ANA_POW_PART, RT721_VREF1_HV_CTRL1, 0xe000);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_VENDOR_ANA_CTL, RT721_MISC_POWER_CTL31, 0x8007);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_ENT_FLOAT_CTL9, 0x2a2a);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_ENT_FLOAT_CTL10, 0x2a00);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_ENT_FLOAT_CTL6, 0x2a2a);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_ENT_FLOAT_CTL5, 0x2626);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_ENT_FLOAT_CTL8, 0x1e00);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_ENT_FLOAT_CTL7, 0x1515);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_CH_FLOAT_CTL3, 0x0304);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_CH_FLOAT_CTL4, 0x0304);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_HDA_LEGACY_CTL1, 0x0000);
    regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT721_SDCA_ENT_IT26, RT721_SDCA_CTL_VENDOR_DEF, 0), 0x01);
    regmap_write((*rt721).mbq_regmap, 0x5910009, 0x2e01);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_RC_CALIB_CTRL, RT721_RC_CALIB_CTRL0, 0x0b00);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_RC_CALIB_CTRL, RT721_RC_CALIB_CTRL0, 0x0b40);
    regmap_write((*rt721).regmap, 0x2f5c, 0x25);
}

unsafe fn rt721_sdca_amp_preset(rt721: *mut rt721_sdca_priv) {
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_VENDOR_ANA_CTL, RT721_MISC_POWER_CTL31, 0x8000);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_ANA_POW_PART, RT721_VREF1_HV_CTRL1, 0xe000);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_VENDOR_ANA_CTL, RT721_MISC_POWER_CTL31, 0x8007);
    regmap_write((*rt721).mbq_regmap, 0x5810000, 0x6420);
    regmap_write((*rt721).mbq_regmap, 0x5810000, 0x6421);
    regmap_write((*rt721).mbq_regmap, 0x5810000, 0xe421);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_CH_FLOAT_CTL6, 0x5561);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_VENDOR_REG, RT721_GPIO_PAD_CTRL5, 0x8003);
    regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_AMP, RT721_SDCA_ENT_OT23, RT721_SDCA_CTL_VENDOR_DEF, 0), 0x04);
    regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_AMP, RT721_SDCA_ENT_PDE23, RT721_SDCA_CTL_FU_MUTE, CH_01), 0x00);
    regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_AMP, RT721_SDCA_ENT_PDE23, RT721_SDCA_CTL_FU_MUTE, CH_02), 0x00);
    regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_AMP, RT721_SDCA_ENT_FU55, RT721_SDCA_CTL_FU_MUTE, CH_01), 0x00);
    regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_AMP, RT721_SDCA_ENT_FU55, RT721_SDCA_CTL_FU_MUTE, CH_02), 0x00);
}

unsafe fn rt721_sdca_jack_preset(rt721: *mut rt721_sdca_priv) {
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_VENDOR_ANA_CTL, RT721_MISC_POWER_CTL31, 0x8000);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_ANA_POW_PART, RT721_VREF1_HV_CTRL1, 0xe000);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_VENDOR_ANA_CTL, RT721_MISC_POWER_CTL31, 0x8007);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_GE_REL_CTRL1, 0x8011);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_UMP_HID_CTRL3, 0xcf00);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_UMP_HID_CTRL4, 0x000f);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_UMP_HID_CTRL1, 0x1100);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_UMP_HID_CTRL5, 0x0c12);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_JD_CTRL, RT721_JD_1PIN_GAT_CTRL2, 0xc002);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_RC_CALIB_CTRL, RT721_RC_CALIB_CTRL0, 0x0b00);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_RC_CALIB_CTRL, RT721_RC_CALIB_CTRL0, 0x0b40);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_VENDOR_ANA_CTL, RT721_UAJ_TOP_TCON14, 0x3333);
    regmap_write((*rt721).mbq_regmap, 0x5810035, 0x0036);
    regmap_write((*rt721).mbq_regmap, 0x5810030, 0xee00);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_CAP_PORT_CTRL, RT721_HP_AMP_2CH_CAL1, 0x0140);
    regmap_write((*rt721).mbq_regmap, 0x5810000, 0x0021);
    regmap_write((*rt721).mbq_regmap, 0x5810000, 0x8021);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_CAP_PORT_CTRL, RT721_HP_AMP_2CH_CAL18, 0x5522);
    regmap_write((*rt721).mbq_regmap, 0x5b10007, 0x2000);
    regmap_write((*rt721).mbq_regmap, 0x5B10017, 0x1b0f);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_CBJ_CTRL, RT721_CBJ_A0_GAT_CTRL1, 0x2205);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_CAP_PORT_CTRL, RT721_HP_AMP_2CH_CAL4, 0xa105);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_VENDOR_ANA_CTL, RT721_UAJ_TOP_TCON14, 0x3b33);
    regmap_write((*rt721).mbq_regmap, 0x310400, 0x3043);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_VENDOR_ANA_CTL, RT721_UAJ_TOP_TCON14, 0x3f33);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_VENDOR_ANA_CTL, RT721_UAJ_TOP_TCON13, 0x6048);
    regmap_write((*rt721).mbq_regmap, 0x310401, 0x3000);
    regmap_write((*rt721).mbq_regmap, 0x310402, 0x1b00);
    regmap_write((*rt721).mbq_regmap, 0x310300, 0x000f);
    regmap_write((*rt721).mbq_regmap, 0x310301, 0x3000);
    regmap_write((*rt721).mbq_regmap, 0x310302, 0x1b00);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_VENDOR_ANA_CTL, RT721_UAJ_TOP_TCON17, 0x0008);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_DAC_CTRL, RT721_DAC_2CH_CTRL3, 0x55ff);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_DAC_CTRL, RT721_DAC_2CH_CTRL4, 0xcc00);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_ANA_POW_PART, RT721_MBIAS_LV_CTRL2, 0x6677);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_ANA_POW_PART, RT721_VREF2_LV_CTRL1, 0x7600);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_ENT_FLOAT_CTL2, 0x1234);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_ENT_FLOAT_CTL3, 0x3512);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_ENT_FLOAT_CTL1, 0x4040);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_ENT_FLOAT_CTL4, 0x1201);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_BOOST_CTRL, RT721_BST_4CH_TOP_GATING_CTRL1, 0x002a);
    regmap_write((*rt721).regmap, 0x2f58, 0x07);
    regmap_write((*rt721).regmap, 0x2f51, 0x00);
    rt_sdca_index_write((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_MISC_CTL, 0x0004);
}

unsafe fn rt721_sdca_jack_init(rt721: *mut rt721_sdca_priv) {
    /* C used guard(mutex)(&rt721->calibrate_mutex); scoped locking is supplied by linux/cleanup.h. */
    if !(*rt721).hs_jack.is_null() {
        sdw_write_no_pm((*rt721).slave, SDW_SCP_SDCA_INTMASK1, SDW_SCP_SDCA_INTMASK_SDCA_0);
        sdw_write_no_pm((*rt721).slave, SDW_SCP_SDCA_INTMASK2, SDW_SCP_SDCA_INTMASK_SDCA_8);
        dev_dbg(&mut (*(*rt721).slave).dev, cstr!("in %s enable\n"), cstr!("rt721_sdca_jack_init"));
        rt_sdca_index_write((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_HDA_LEGACY_UAJ_CTL, 0x036E);
        regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_XU03, RT721_SDCA_CTL_SELECTED_MODE, 0), 0);
        regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_XU0D, RT721_SDCA_CTL_SELECTED_MODE, 0), 0);
        rt_sdca_index_write((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_XU_REL_CTRL, 0x0000);
        rt_sdca_index_update_bits((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_GE_REL_CTRL1, 0x4000, 0x4000);
    }
}

unsafe extern "C" fn rt721_sdca_set_jack_detect(component: *mut snd_soc_component, hs_jack: *mut snd_soc_jack, _data: *mut c_void) -> c_int {
    let rt721 = snd_soc_component_get_drvdata(component);
    (*rt721).hs_jack = hs_jack;
    let ret = pm_runtime_resume_and_get((*component).dev);
    if ret < 0 {
        if ret != -EACCES {
            dev_err((*component).dev, cstr!("%s: failed to resume %d\n"), cstr!("rt721_sdca_set_jack_detect"), ret);
            return ret;
        }
        /* pm_runtime not enabled yet */
        dev_dbg((*component).dev, cstr!("%s: skipping jack init for now\n"), cstr!("rt721_sdca_set_jack_detect"));
        return 0;
    }
    rt721_sdca_jack_init(rt721);
    pm_runtime_put_autosuspend((*component).dev);
    0
}

/* For SDCA control DAC/ADC Gain */
unsafe extern "C" fn rt721_sdca_set_gain_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let rt721 = snd_soc_component_get_drvdata(component);
    let mut read_l = 0;
    let mut read_r = 0;
    let mut adc_vol_flag = 0u32;
    let mut changed = 0;
    let mut lvalue = 0;
    let mut rvalue = 0;
    let interval_offset = 0xc0u32;
    let tendA = 0x200u32;
    let tendB = 0xa00u32;

    if !strstr((*ucontrol).id.name.as_ptr(), cstr!("FU1E Capture Volume")).is_null()
        || !strstr((*ucontrol).id.name.as_ptr(), cstr!("FU0F Capture Volume")).is_null() {
        adc_vol_flag = 1;
    }

    regmap_read((*rt721).mbq_regmap, (*mc).reg, &mut lvalue);
    regmap_read((*rt721).mbq_regmap, (*mc).rreg, &mut rvalue);

    let mut gain_l_val = (*ucontrol).value.integer.value[0];
    if gain_l_val > (*mc).max { gain_l_val = (*mc).max; }
    if (*mc).shift == 8 {
        /* boost gain */
        gain_l_val = gain_l_val.wrapping_mul(tendB);
    } else if (*mc).shift == 1 {
        /* FU33 boost gain */
        if gain_l_val == 0 { gain_l_val = 0x8000; } else { gain_l_val = (gain_l_val - 1).wrapping_mul(tendA); }
    } else {
        /* ADC/DAC gain */
        if adc_vol_flag != 0 {
            gain_l_val = 0x1e00u32.wrapping_sub(((*mc).max - gain_l_val).wrapping_mul(interval_offset));
        } else {
            gain_l_val = 0u32.wrapping_sub(((*mc).max - gain_l_val).wrapping_mul(interval_offset));
        }
        gain_l_val &= 0xffff;
    }

    let mut gain_r_val = (*ucontrol).value.integer.value[1];
    if gain_r_val > (*mc).max { gain_r_val = (*mc).max; }
    if (*mc).shift == 8 {
        /* boost gain */
        gain_r_val = gain_r_val.wrapping_mul(tendB);
    } else if (*mc).shift == 1 {
        /* FU33 boost gain */
        if gain_r_val == 0 { gain_r_val = 0x8000; } else { gain_r_val = (gain_r_val - 1).wrapping_mul(tendA); }
    } else {
        /* ADC/DAC gain */
        if adc_vol_flag != 0 {
            gain_r_val = 0x1e00u32.wrapping_sub(((*mc).max - gain_r_val).wrapping_mul(interval_offset));
        } else {
            gain_r_val = 0u32.wrapping_sub(((*mc).max - gain_r_val).wrapping_mul(interval_offset));
        }
        gain_r_val &= 0xffff;
    }

    if lvalue != gain_l_val || rvalue != gain_r_val { changed = 1; } else { return 0; }

    /* Lch*/
    regmap_write((*rt721).mbq_regmap, (*mc).reg, gain_l_val);
    /* Rch */
    regmap_write((*rt721).mbq_regmap, (*mc).rreg, gain_r_val);
    regmap_read((*rt721).mbq_regmap, (*mc).reg, &mut read_l);
    regmap_read((*rt721).mbq_regmap, (*mc).rreg, &mut read_r);
    if read_r == gain_r_val && read_l == gain_l_val { return changed; }
    -EIO
}

unsafe extern "C" fn rt721_sdca_set_gain_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt721 = snd_soc_component_get_drvdata(component);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let mut read_l = 0u32;
    let mut read_r = 0u32;
    let mut ctl_l = 0u32;
    let mut ctl_r;
    let mut adc_vol_flag = 0u32;
    let interval_offset = 0xc0u32;
    let tendA = 0x200u32;
    let tendB = 0xa00u32;

    if !strstr((*ucontrol).id.name.as_ptr(), cstr!("FU1E Capture Volume")).is_null()
        || !strstr((*ucontrol).id.name.as_ptr(), cstr!("FU0F Capture Volume")).is_null() {
        adc_vol_flag = 1;
    }
    regmap_read((*rt721).mbq_regmap, (*mc).reg, &mut read_l);
    regmap_read((*rt721).mbq_regmap, (*mc).rreg, &mut read_r);
    if (*mc).shift == 8 {
        /* boost gain */
        ctl_l = read_l / tendB;
    } else if (*mc).shift == 1 {
        /* FU33 boost gain */
        if read_l == 0x8000 || read_l == 0xfe00 { ctl_l = 0; } else { ctl_l = read_l / tendA + 1; }
    } else if adc_vol_flag != 0 {
        ctl_l = (*mc).max - ((0x1e00u32.wrapping_sub(read_l) & 0xffff) / interval_offset);
    } else {
        ctl_l = (*mc).max - ((0u32.wrapping_sub(read_l) & 0xffff) / interval_offset);
    }
    if read_l != read_r {
        if (*mc).shift == 8 {
            /* boost gain */
            ctl_r = read_r / tendB;
        } else if (*mc).shift == 1 {
            /* FU33 boost gain */
            if read_r == 0x8000 || read_r == 0xfe00 { ctl_r = 0; } else { ctl_r = read_r / tendA + 1; }
        } else if adc_vol_flag != 0 {
            ctl_r = (*mc).max - ((0x1e00u32.wrapping_sub(read_r) & 0xffff) / interval_offset);
        } else {
            ctl_r = (*mc).max - ((0u32.wrapping_sub(read_r) & 0xffff) / interval_offset);
        }
    } else {
        ctl_r = ctl_l;
    }
    (*ucontrol).value.integer.value[0] = ctl_l;
    (*ucontrol).value.integer.value[1] = ctl_r;
    0
}

unsafe fn rt721_sdca_set_fu1e_capture_ctl(rt721: *mut rt721_sdca_priv) -> c_int {
    for i in 0..(*rt721).fu1e_mixer_mute.len() {
        let ch_mute: c_uint = ((*rt721).fu1e_dapm_mute || (*rt721).fu1e_mixer_mute[i]) as c_uint;
        let err = regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT721_SDCA_ENT_USER_FU1E, RT721_SDCA_CTL_FU_MUTE, CH_01) + i as c_uint, ch_mute);
        if err < 0 { return err; }
    }
    0
}

unsafe extern "C" fn rt721_sdca_fu1e_capture_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt721 = snd_soc_component_get_drvdata(component);
    let p = (*kcontrol).private_value as *mut rt721_sdca_dmic_kctrl_priv;
    for i in 0..(*p).count as usize {
        (*ucontrol).value.integer.value[i] = (!(*rt721).fu1e_mixer_mute[i]) as c_uint;
    }
    0
}

unsafe extern "C" fn rt721_sdca_fu1e_capture_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt721 = snd_soc_component_get_drvdata(component);
    let p = (*kcontrol).private_value as *mut rt721_sdca_dmic_kctrl_priv;
    let mut changed = 0;
    for i in 0..(*p).count as usize {
        if (*rt721).fu1e_mixer_mute[i] != ((*ucontrol).value.integer.value[i] == 0) {
            changed = 1;
        }
        (*rt721).fu1e_mixer_mute[i] = (*ucontrol).value.integer.value[i] == 0;
    }
    let err = rt721_sdca_set_fu1e_capture_ctl(rt721);
    if err < 0 { return err; }
    changed
}

unsafe fn rt721_sdca_set_fu0f_capture_ctl(rt721: *mut rt721_sdca_priv) -> c_int {
    let ch_l = if (*rt721).fu0f_dapm_mute || (*rt721).fu0f_mixer_l_mute { 0x01 } else { 0x00 };
    let ch_r = if (*rt721).fu0f_dapm_mute || (*rt721).fu0f_mixer_r_mute { 0x01 } else { 0x00 };
    let mut err = regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_USER_FU0F, RT721_SDCA_CTL_FU_MUTE, CH_L), ch_l);
    if err < 0 { return err; }
    err = regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_USER_FU0F, RT721_SDCA_CTL_FU_MUTE, CH_R), ch_r);
    if err < 0 { return err; }
    0
}

unsafe extern "C" fn rt721_sdca_fu0f_capture_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt721 = snd_soc_component_get_drvdata(component);
    (*ucontrol).value.integer.value[0] = (!(*rt721).fu0f_mixer_l_mute) as c_uint;
    (*ucontrol).value.integer.value[1] = (!(*rt721).fu0f_mixer_r_mute) as c_uint;
    0
}

unsafe extern "C" fn rt721_sdca_fu0f_capture_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt721 = snd_soc_component_get_drvdata(component);
    let mut changed = 0;
    if (*rt721).fu0f_mixer_l_mute != ((*ucontrol).value.integer.value[0] == 0)
        || (*rt721).fu0f_mixer_r_mute != ((*ucontrol).value.integer.value[1] == 0) {
        changed = 1;
    }
    (*rt721).fu0f_mixer_l_mute = (*ucontrol).value.integer.value[0] == 0;
    (*rt721).fu0f_mixer_r_mute = (*ucontrol).value.integer.value[1] == 0;
    let err = rt721_sdca_set_fu0f_capture_ctl(rt721);
    if err < 0 { return err; }
    changed
}

unsafe extern "C" fn rt721_sdca_fu_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let p = (*kcontrol).private_value as *mut rt721_sdca_dmic_kctrl_priv;
    (*uinfo).type_ = if (*p).max == 1 { SNDRV_CTL_ELEM_TYPE_BOOLEAN } else { SNDRV_CTL_ELEM_TYPE_INTEGER };
    (*uinfo).count = (*p).count;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = (*p).max as c_int;
    0
}

unsafe extern "C" fn rt721_sdca_dmic_set_gain_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt721 = snd_soc_component_get_drvdata(component);
    let p = (*kcontrol).private_value as *mut rt721_sdca_dmic_kctrl_priv;
    let boost_step = 0x0a00u32;
    let vol_max = 0x1e00u32;
    let interval_offset = 0xc0u32;
    let mut adc_vol_flag = 0u32;
    if !strstr((*ucontrol).id.name.as_ptr(), cstr!("FU1E Capture Volume")).is_null() {
        adc_vol_flag = 1;
    }
    /* check all channels */
    for i in 0..(*p).count as usize {
        let mut regvalue = 0u32;
        regmap_read((*rt721).mbq_regmap, (*p).reg_base + i as c_uint, &mut regvalue);
        let ctl = if adc_vol_flag == 0 {
            /* boost gain */
            regvalue / boost_step
        } else {
            /* ADC gain */
            (*p).max - ((vol_max.wrapping_sub(regvalue) & 0xffff) / interval_offset)
        };
        (*ucontrol).value.integer.value[i] = ctl;
    }
    0
}

unsafe extern "C" fn rt721_sdca_dmic_set_gain_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let p = (*kcontrol).private_value as *mut rt721_sdca_dmic_kctrl_priv;
    let rt721 = snd_soc_component_get_drvdata(component);
    let boost_step = 0x0a00u32;
    let vol_max = 0x1e00u32;
    let interval_offset = 0xc0u32;
    let mut gain_val = [0u32; 4];
    let mut regvalue = [0u32; 4];
    let mut adc_vol_flag = 0u32;
    let mut changed = 0;
    if !strstr((*ucontrol).id.name.as_ptr(), cstr!("FU1E Capture Volume")).is_null() {
        adc_vol_flag = 1;
    }
    /* check all channels */
    for i in 0..(*p).count as usize {
        regmap_read((*rt721).mbq_regmap, (*p).reg_base + i as c_uint, &mut regvalue[i]);
        gain_val[i] = (*ucontrol).value.integer.value[i];
        if gain_val[i] > (*p).max { gain_val[i] = (*p).max; }
        if adc_vol_flag == 0 {
            /* boost gain */
            gain_val[i] = gain_val[i].wrapping_mul(boost_step);
        } else {
            /* ADC gain */
            gain_val[i] = vol_max.wrapping_sub(((*p).max - gain_val[i]).wrapping_mul(interval_offset));
            gain_val[i] &= 0xffff;
        }
        if regvalue[i] != gain_val[i] { changed = 1; }
    }
    if changed == 0 { return 0; }
    for i in 0..(*p).count as usize {
        let err = regmap_write((*rt721).mbq_regmap, (*p).reg_base + i as c_uint, gain_val[i]);
        if err < 0 {
            dev_err(&mut (*(*rt721).slave).dev, cstr!("%#08x can't be set\n"), (*p).reg_base + i as c_uint);
        }
    }
    changed
}

DECLARE_TLV_DB_SCALE!(out_vol_tlv, -6525, 75, 0);
DECLARE_TLV_DB_SCALE!(mic_vol_tlv, -1725, 75, 0);
DECLARE_TLV_DB_SCALE!(boost_vol_tlv, 0, 1000, 0);
DECLARE_TLV_DB_SCALE!(mic2_boost_vol_tlv, -200, 200, 0);

/* Macro-constructed ALSA controls from the C source are preserved as dependency-provided macro intent. */
static rt721_sdca_controls: [snd_kcontrol_new; 0] = [];

unsafe extern "C" fn rt721_sdca_adc_mux_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let rt721 = snd_soc_component_get_drvdata(component);
    let mut val = 0u32;
    let mask_sft: c_uint;
    let mask: c_uint;
    if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 09 Mux")).is_null() {
        mask_sft = 12; mask = 0x7;
    } else if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 08 R Mux")).is_null() {
        mask_sft = 10; mask = 0x3;
    } else if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 08 L Mux")).is_null() {
        mask_sft = 8; mask = 0x3;
    } else if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 10 R Mux")).is_null() {
        mask_sft = 6; mask = 0x3;
    } else if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 10 L Mux")).is_null() {
        mask_sft = 4; mask = 0x3;
    } else if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 07 R Mux")).is_null() {
        mask_sft = 2; mask = 0x3;
    } else if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 07 L Mux")).is_null() {
        mask_sft = 0; mask = 0x3;
    } else {
        return -EINVAL;
    }
    rt_sdca_index_read((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_HDA_LEGACY_MUX_CTL0, &mut val);
    (*ucontrol).value.enumerated.item[0] = (val >> mask_sft) & mask;
    0
}

unsafe extern "C" fn rt721_sdca_adc_mux_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let rt721 = snd_soc_component_get_drvdata(component);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let item = (*ucontrol).value.enumerated.item.as_mut_ptr();
    let mask_sft: c_uint;
    let mask: c_uint;
    if *item >= (*e).items { return -EINVAL; }
    if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 09 Mux")).is_null() {
        mask_sft = 12; mask = 0x7;
    } else if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 08 R Mux")).is_null() {
        mask_sft = 10; mask = 0x3;
    } else if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 08 L Mux")).is_null() {
        mask_sft = 8; mask = 0x3;
    } else if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 10 R Mux")).is_null() {
        mask_sft = 6; mask = 0x3;
    } else if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 10 L Mux")).is_null() {
        mask_sft = 4; mask = 0x3;
    } else if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 07 R Mux")).is_null() {
        mask_sft = 2; mask = 0x3;
    } else if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 07 L Mux")).is_null() {
        mask_sft = 0; mask = 0x3;
    } else {
        return -EINVAL;
    }
    let val = snd_soc_enum_item_to_val(e, *item) << (*e).shift_l;
    let mut val2 = 0u32;
    rt_sdca_index_read((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_HDA_LEGACY_MUX_CTL0, &mut val2);
    if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 09 Mux")).is_null() {
        val2 = (val2 >> mask_sft) & 0x7;
    } else {
        val2 = (val2 >> mask_sft) & 0x3;
    }
    let change = if val == val2 { 0 } else { 1 };
    if change != 0 {
        let mut check = 0u32;
        rt_sdca_index_read((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_HDA_LEGACY_MUX_CTL0, &mut check);
        rt_sdca_index_update_bits((*rt721).mbq_regmap, RT721_HDA_SDCA_FLOAT, RT721_HDA_LEGACY_MUX_CTL0, mask << mask_sft, val << mask_sft);
    }
    snd_soc_dapm_mux_update_power(dapm, kcontrol, *item, e, ptr::null_mut());
    change
}

static adc09_mux_text: [*const c_char; 3] = [cstr!("MIC2"), cstr!("LINE1"), cstr!("LINE2")];
static adc07_10_mux_text: [*const c_char; 4] = [cstr!("DMIC1 RE"), cstr!("DMIC1 FE"), cstr!("DMIC2 RE"), cstr!("DMIC2 FE")];

/* SOC_ENUM_SINGLE_DECL and SOC_DAPM_ENUM_EXT values are dependency-provided in the original C build. */
static rt721_sdca_adc09_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static rt721_sdca_adc08_r_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static rt721_sdca_adc08_l_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static rt721_sdca_adc10_r_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static rt721_sdca_adc10_l_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static rt721_sdca_adc07_r_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static rt721_sdca_adc07_l_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

unsafe extern "C" fn rt721_sdca_fu42_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt721 = snd_soc_component_get_drvdata(component);
    let unmute = 0x0u32;
    let mute = 0x1u32;
    match event {
        SND_SOC_DAPM_POST_PMU => {
            msleep(100);
            regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_USER_FU05, RT721_SDCA_CTL_FU_MUTE, CH_L), unmute);
            regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_USER_FU05, RT721_SDCA_CTL_FU_MUTE, CH_R), unmute);
        }
        SND_SOC_DAPM_PRE_PMD => {
            regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_USER_FU05, RT721_SDCA_CTL_FU_MUTE, CH_L), mute);
            regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_USER_FU05, RT721_SDCA_CTL_FU_MUTE, CH_R), mute);
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn rt721_sdca_fu21_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt721 = snd_soc_component_get_drvdata(component);
    let unmute = 0x0u32;
    let mute = 0x1u32;
    match event {
        SND_SOC_DAPM_POST_PMU => {
            regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_AMP, RT721_SDCA_ENT_USER_FU06, RT721_SDCA_CTL_FU_MUTE, CH_L), unmute);
            regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_AMP, RT721_SDCA_ENT_USER_FU06, RT721_SDCA_CTL_FU_MUTE, CH_R), unmute);
        }
        SND_SOC_DAPM_PRE_PMD => {
            regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_AMP, RT721_SDCA_ENT_USER_FU06, RT721_SDCA_CTL_FU_MUTE, CH_L), mute);
            regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_AMP, RT721_SDCA_ENT_USER_FU06, RT721_SDCA_CTL_FU_MUTE, CH_R), mute);
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn rt721_sdca_fu23_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt721 = snd_soc_component_get_drvdata(component);
    let unmute = 0x0u32;
    let mute = 0x1u32;
    match event {
        SND_SOC_DAPM_POST_PMU => {
            regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_AMP, RT721_SDCA_ENT_PDE23, RT721_SDCA_CTL_FU_MUTE, CH_L), unmute);
            regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_AMP, RT721_SDCA_ENT_PDE23, RT721_SDCA_CTL_FU_MUTE, CH_R), unmute);
        }
        SND_SOC_DAPM_PRE_PMD => {
            regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_AMP, RT721_SDCA_ENT_PDE23, RT721_SDCA_CTL_FU_MUTE, CH_L), mute);
            regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_AMP, RT721_SDCA_ENT_PDE23, RT721_SDCA_CTL_FU_MUTE, CH_R), mute);
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn rt721_sdca_fu113_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt721 = snd_soc_component_get_drvdata(component);
    match event {
        SND_SOC_DAPM_POST_PMU => { (*rt721).fu1e_dapm_mute = false; rt721_sdca_set_fu1e_capture_ctl(rt721); }
        SND_SOC_DAPM_PRE_PMD => { (*rt721).fu1e_dapm_mute = true; rt721_sdca_set_fu1e_capture_ctl(rt721); }
        _ => {}
    }
    0
}

unsafe extern "C" fn rt721_sdca_fu36_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt721 = snd_soc_component_get_drvdata(component);
    match event {
        SND_SOC_DAPM_POST_PMU => { (*rt721).fu0f_dapm_mute = false; rt721_sdca_set_fu0f_capture_ctl(rt721); }
        SND_SOC_DAPM_PRE_PMD => { (*rt721).fu0f_dapm_mute = true; rt721_sdca_set_fu0f_capture_ctl(rt721); }
        _ => {}
    }
    0
}

unsafe extern "C" fn rt721_sdca_pde47_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt721 = snd_soc_component_get_drvdata(component);
    match event {
        SND_SOC_DAPM_POST_PMU => { regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_PDE40, RT721_SDCA_CTL_REQ_POWER_STATE, 0), 0x0); }
        SND_SOC_DAPM_PRE_PMD => { regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_PDE40, RT721_SDCA_CTL_REQ_POWER_STATE, 0), 0x3); }
        _ => {}
    }
    0
}

unsafe extern "C" fn rt721_sdca_pde41_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt721 = snd_soc_component_get_drvdata(component);
    match event {
        SND_SOC_DAPM_POST_PMU => { regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_AMP, RT721_SDCA_ENT_PDE41, RT721_SDCA_CTL_REQ_POWER_STATE, 0), 0x0); }
        SND_SOC_DAPM_PRE_PMD => { regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_AMP, RT721_SDCA_ENT_PDE41, RT721_SDCA_CTL_REQ_POWER_STATE, 0), 0x3); }
        _ => {}
    }
    0
}

unsafe extern "C" fn rt721_sdca_pde11_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt721 = snd_soc_component_get_drvdata(component);
    match event {
        SND_SOC_DAPM_POST_PMU => { regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT721_SDCA_ENT_PDE2A, RT721_SDCA_CTL_REQ_POWER_STATE, 0), 0x0); }
        SND_SOC_DAPM_PRE_PMD => { regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT721_SDCA_ENT_PDE2A, RT721_SDCA_CTL_REQ_POWER_STATE, 0), 0x3); }
        _ => {}
    }
    0
}

unsafe extern "C" fn rt721_sdca_pde34_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt721 = snd_soc_component_get_drvdata(component);
    match event {
        SND_SOC_DAPM_POST_PMU => { regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_PDE12, RT721_SDCA_CTL_REQ_POWER_STATE, 0), 0x0); }
        SND_SOC_DAPM_PRE_PMD => { regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_PDE12, RT721_SDCA_CTL_REQ_POWER_STATE, 0), 0x3); }
        _ => {}
    }
    0
}

/* DAPM widgets from macro constructors in the C source. */
static rt721_sdca_dapm_widgets: [snd_soc_dapm_widget_desc; 0] = [];

static rt721_sdca_audio_map: [snd_soc_dapm_route; 47] = [
    snd_soc_dapm_route { sink: cstr!("FU 42"), control: ptr::null(), source: cstr!("DP1RX") },
    snd_soc_dapm_route { sink: cstr!("FU 21"), control: ptr::null(), source: cstr!("DP3RX") },
    snd_soc_dapm_route { sink: cstr!("FU 23"), control: ptr::null(), source: cstr!("DP3RX") },
    snd_soc_dapm_route { sink: cstr!("ADC 09 Mux"), control: cstr!("MIC2"), source: cstr!("MIC2") },
    snd_soc_dapm_route { sink: cstr!("ADC 09 Mux"), control: cstr!("LINE1"), source: cstr!("LINE1") },
    snd_soc_dapm_route { sink: cstr!("ADC 09 Mux"), control: cstr!("LINE2"), source: cstr!("LINE2") },
    snd_soc_dapm_route { sink: cstr!("ADC 07 R Mux"), control: cstr!("DMIC1 RE"), source: cstr!("DMIC1_2") },
    snd_soc_dapm_route { sink: cstr!("ADC 07 R Mux"), control: cstr!("DMIC1 FE"), source: cstr!("DMIC1_2") },
    snd_soc_dapm_route { sink: cstr!("ADC 07 R Mux"), control: cstr!("DMIC2 RE"), source: cstr!("DMIC3_4") },
    snd_soc_dapm_route { sink: cstr!("ADC 07 R Mux"), control: cstr!("DMIC2 FE"), source: cstr!("DMIC3_4") },
    snd_soc_dapm_route { sink: cstr!("ADC 07 L Mux"), control: cstr!("DMIC1 RE"), source: cstr!("DMIC1_2") },
    snd_soc_dapm_route { sink: cstr!("ADC 07 L Mux"), control: cstr!("DMIC1 FE"), source: cstr!("DMIC1_2") },
    snd_soc_dapm_route { sink: cstr!("ADC 07 L Mux"), control: cstr!("DMIC2 RE"), source: cstr!("DMIC3_4") },
    snd_soc_dapm_route { sink: cstr!("ADC 07 L Mux"), control: cstr!("DMIC2 FE"), source: cstr!("DMIC3_4") },
    snd_soc_dapm_route { sink: cstr!("ADC 08 R Mux"), control: cstr!("DMIC1 RE"), source: cstr!("DMIC1_2") },
    snd_soc_dapm_route { sink: cstr!("ADC 08 R Mux"), control: cstr!("DMIC1 FE"), source: cstr!("DMIC1_2") },
    snd_soc_dapm_route { sink: cstr!("ADC 08 R Mux"), control: cstr!("DMIC2 RE"), source: cstr!("DMIC3_4") },
    snd_soc_dapm_route { sink: cstr!("ADC 08 R Mux"), control: cstr!("DMIC2 FE"), source: cstr!("DMIC3_4") },
    snd_soc_dapm_route { sink: cstr!("ADC 08 L Mux"), control: cstr!("DMIC1 RE"), source: cstr!("DMIC1_2") },
    snd_soc_dapm_route { sink: cstr!("ADC 08 L Mux"), control: cstr!("DMIC1 FE"), source: cstr!("DMIC1_2") },
    snd_soc_dapm_route { sink: cstr!("ADC 08 L Mux"), control: cstr!("DMIC2 RE"), source: cstr!("DMIC3_4") },
    snd_soc_dapm_route { sink: cstr!("ADC 08 L Mux"), control: cstr!("DMIC2 FE"), source: cstr!("DMIC3_4") },
    snd_soc_dapm_route { sink: cstr!("ADC 10 R Mux"), control: cstr!("DMIC1 RE"), source: cstr!("DMIC1_2") },
    snd_soc_dapm_route { sink: cstr!("ADC 10 R Mux"), control: cstr!("DMIC1 FE"), source: cstr!("DMIC1_2") },
    snd_soc_dapm_route { sink: cstr!("ADC 10 R Mux"), control: cstr!("DMIC2 RE"), source: cstr!("DMIC3_4") },
    snd_soc_dapm_route { sink: cstr!("ADC 10 R Mux"), control: cstr!("DMIC2 FE"), source: cstr!("DMIC3_4") },
    snd_soc_dapm_route { sink: cstr!("ADC 10 L Mux"), control: cstr!("DMIC1 RE"), source: cstr!("DMIC1_2") },
    snd_soc_dapm_route { sink: cstr!("ADC 10 L Mux"), control: cstr!("DMIC1 FE"), source: cstr!("DMIC1_2") },
    snd_soc_dapm_route { sink: cstr!("ADC 10 L Mux"), control: cstr!("DMIC2 RE"), source: cstr!("DMIC3_4") },
    snd_soc_dapm_route { sink: cstr!("ADC 10 L Mux"), control: cstr!("DMIC2 FE"), source: cstr!("DMIC3_4") },
    snd_soc_dapm_route { sink: cstr!("FU 36"), control: ptr::null(), source: cstr!("PDE 34") },
    snd_soc_dapm_route { sink: cstr!("FU 36"), control: ptr::null(), source: cstr!("ADC 09 Mux") },
    snd_soc_dapm_route { sink: cstr!("FU 113"), control: ptr::null(), source: cstr!("PDE 11") },
    snd_soc_dapm_route { sink: cstr!("FU 113"), control: ptr::null(), source: cstr!("ADC 07 R Mux") },
    snd_soc_dapm_route { sink: cstr!("FU 113"), control: ptr::null(), source: cstr!("ADC 07 L Mux") },
    snd_soc_dapm_route { sink: cstr!("FU 113"), control: ptr::null(), source: cstr!("ADC 10 R Mux") },
    snd_soc_dapm_route { sink: cstr!("FU 113"), control: ptr::null(), source: cstr!("ADC 10 L Mux") },
    snd_soc_dapm_route { sink: cstr!("DP2TX"), control: ptr::null(), source: cstr!("FU 36") },
    snd_soc_dapm_route { sink: cstr!("DP6TX"), control: ptr::null(), source: cstr!("FU 113") },
    snd_soc_dapm_route { sink: cstr!("HP"), control: ptr::null(), source: cstr!("PDE 47") },
    snd_soc_dapm_route { sink: cstr!("HP"), control: ptr::null(), source: cstr!("FU 42") },
    snd_soc_dapm_route { sink: cstr!("SPK"), control: ptr::null(), source: cstr!("PDE 41") },
    snd_soc_dapm_route { sink: cstr!("SPK"), control: ptr::null(), source: cstr!("FU 21") },
    snd_soc_dapm_route { sink: cstr!("SPK"), control: ptr::null(), source: cstr!("FU 23") },
    snd_soc_dapm_route { sink: ptr::null(), control: ptr::null(), source: ptr::null() },
    snd_soc_dapm_route { sink: ptr::null(), control: ptr::null(), source: ptr::null() },
    snd_soc_dapm_route { sink: ptr::null(), control: ptr::null(), source: ptr::null() },
];

unsafe fn rt721_sdca_parse_dt(rt721: *mut rt721_sdca_priv, dev: *mut device) -> c_int {
    device_property_read_u32(dev, cstr!("realtek,jd-src"), &mut (*rt721).jd_src);
    0
}

unsafe extern "C" fn rt721_sdca_probe(component: *mut snd_soc_component) -> c_int {
    let rt721 = snd_soc_component_get_drvdata(component);
    rt721_sdca_parse_dt(rt721, &mut (*(*rt721).slave).dev);
    (*rt721).component = component;
    let ret = pm_runtime_resume((*component).dev);
    if ret < 0 && ret != -EACCES { return ret; }
    0
}

static soc_sdca_dev_rt721: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rt721_sdca_probe),
    controls: rt721_sdca_controls.as_ptr(),
    num_controls: rt721_sdca_controls.len() as c_uint,
    dapm_widgets: rt721_sdca_dapm_widgets.as_ptr(),
    num_dapm_widgets: rt721_sdca_dapm_widgets.len() as c_uint,
    dapm_routes: rt721_sdca_audio_map.as_ptr(),
    num_dapm_routes: rt721_sdca_audio_map.len() as c_uint,
    set_jack: Some(rt721_sdca_set_jack_detect),
    endianness: 1,
};

unsafe extern "C" fn rt721_sdca_set_sdw_stream(dai: *mut snd_soc_dai, sdw_stream: *mut c_void, direction: c_int) -> c_int {
    snd_soc_dai_dma_data_set(dai, direction, sdw_stream);
    0
}

unsafe extern "C" fn rt721_sdca_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    snd_soc_dai_set_dma_data(dai, substream, ptr::null_mut());
}

unsafe extern "C" fn rt721_sdca_pcm_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let rt721 = snd_soc_component_get_drvdata(component);
    let mut stream_config = sdw_stream_config { frame_rate: 0, ch_count: 0, bps: 0, direction: 0 };
    let mut port_config = sdw_port_config { ch_mask: 0, num: 0 };
    let direction: sdw_data_direction;
    let port: c_int;

    dev_dbg((*dai).dev, cstr!("%s %s"), cstr!("rt721_sdca_pcm_hw_params"), (*dai).name);
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    if sdw_stream.is_null() { return -EINVAL; }
    if (*rt721).slave.is_null() { return -EINVAL; }

    /*
     * RT721_AIF1 with port = 1 for headphone playback
     * RT721_AIF1 with port = 2 for headset-mic capture
     * RT721_AIF2 with port = 3 for speaker playback
     * RT721_AIF3 with port = 6 for digital-mic capture
     */
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        direction = SDW_DATA_DIR_RX;
        if (*dai).id == RT721_AIF1 { port = 1; }
        else if (*dai).id == RT721_AIF2 { port = 3; }
        else { return -EINVAL; }
    } else {
        direction = SDW_DATA_DIR_TX;
        if (*dai).id == RT721_AIF1 { port = 2; }
        else if (*dai).id == RT721_AIF3 { port = 6; }
        else { return -EINVAL; }
    }
    stream_config.frame_rate = params_rate(params);
    stream_config.ch_count = params_channels(params);
    stream_config.bps = snd_pcm_format_width(params_format(params));
    stream_config.direction = direction;
    let num_channels = params_channels(params);
    port_config.ch_mask = GENMASK!(num_channels - 1, 0);
    port_config.num = port;
    let retval = sdw_stream_add_slave((*rt721).slave, &mut stream_config, &mut port_config, 1, sdw_stream);
    if retval != 0 {
        dev_err((*dai).dev, cstr!("Unable to configure port\n"));
        return retval;
    }
    if params_channels(params) > 16 {
        dev_err((*component).dev, cstr!("Unsupported channels %d\n"), params_channels(params));
        return -EINVAL;
    }

    /* sampling rate configuration */
    let sampling_rate = match params_rate(params) {
        8000 => RT721_SDCA_RATE_8000HZ,
        16000 => RT721_SDCA_RATE_16000HZ,
        24000 => RT721_SDCA_RATE_24000HZ,
        32000 => RT721_SDCA_RATE_32000HZ,
        44100 => RT721_SDCA_RATE_44100HZ,
        48000 => RT721_SDCA_RATE_48000HZ,
        96000 => RT721_SDCA_RATE_96000HZ,
        192000 => RT721_SDCA_RATE_192000HZ,
        384000 => RT721_SDCA_RATE_384000HZ,
        768000 => RT721_SDCA_RATE_768000HZ,
        _ => {
            dev_err((*component).dev, cstr!("Rate %d is not supported\n"), params_rate(params));
            return -EINVAL;
        }
    };

    /* set sampling frequency */
    if (*dai).id == RT721_AIF1 {
        regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_CS01, RT721_SDCA_CTL_SAMPLE_FREQ_INDEX, 0), sampling_rate);
        regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_CS11, RT721_SDCA_CTL_SAMPLE_FREQ_INDEX, 0), sampling_rate);
    }
    if (*dai).id == RT721_AIF2 {
        regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_AMP, RT721_SDCA_ENT_CS31, RT721_SDCA_CTL_SAMPLE_FREQ_INDEX, 0), sampling_rate);
    }
    if (*dai).id == RT721_AIF3 {
        regmap_write((*rt721).regmap, SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT721_SDCA_ENT_CS1F, RT721_SDCA_CTL_SAMPLE_FREQ_INDEX, 0), sampling_rate);
    }
    0
}

unsafe extern "C" fn rt721_sdca_pcm_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let rt721 = snd_soc_component_get_drvdata(component);
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    if (*rt721).slave.is_null() { return -EINVAL; }
    sdw_stream_remove_slave((*rt721).slave, sdw_stream);
    0
}

const RT721_STEREO_RATES: c_uint = SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000;
const RT721_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE;

static rt721_sdca_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(rt721_sdca_pcm_hw_params),
    hw_free: Some(rt721_sdca_pcm_hw_free),
    set_stream: Some(rt721_sdca_set_sdw_stream),
    shutdown: Some(rt721_sdca_shutdown),
};

static mut rt721_sdca_dai: [snd_soc_dai_driver; 3] = [
    snd_soc_dai_driver {
        name: cstr!("rt721-sdca-aif1"),
        id: RT721_AIF1,
        playback: snd_soc_pcm_stream { stream_name: cstr!("DP1 Headphone Playback"), channels_min: 1, channels_max: 2, rates: RT721_STEREO_RATES, formats: RT721_FORMATS },
        capture: snd_soc_pcm_stream { stream_name: cstr!("DP2 Headset Capture"), channels_min: 1, channels_max: 2, rates: RT721_STEREO_RATES, formats: RT721_FORMATS },
        ops: &rt721_sdca_ops,
    },
    snd_soc_dai_driver {
        name: cstr!("rt721-sdca-aif2"),
        id: RT721_AIF2,
        playback: snd_soc_pcm_stream { stream_name: cstr!("DP3 Speaker Playback"), channels_min: 1, channels_max: 2, rates: RT721_STEREO_RATES, formats: RT721_FORMATS },
        capture: snd_soc_pcm_stream { stream_name: ptr::null(), channels_min: 0, channels_max: 0, rates: 0, formats: 0 },
        ops: &rt721_sdca_ops,
    },
    snd_soc_dai_driver {
        name: cstr!("rt721-sdca-aif3"),
        id: RT721_AIF3,
        playback: snd_soc_pcm_stream { stream_name: ptr::null(), channels_min: 0, channels_max: 0, rates: 0, formats: 0 },
        capture: snd_soc_pcm_stream { stream_name: cstr!("DP6 DMic Capture"), channels_min: 1, channels_max: 4, rates: RT721_STEREO_RATES, formats: RT721_FORMATS },
        ops: &rt721_sdca_ops,
    },
];

#[no_mangle]
pub unsafe extern "C" fn rt721_sdca_init(dev: *mut device, regmap: *mut regmap, mbq_regmap: *mut regmap, slave: *mut sdw_slave) -> c_int {
    let rt721 = devm_kzalloc(dev, core::mem::size_of::<rt721_sdca_priv>(), GFP_KERNEL) as *mut rt721_sdca_priv;
    if rt721.is_null() { return -ENOMEM; }
    dev_set_drvdata(dev, rt721 as *mut c_void);
    (*rt721).slave = slave;
    (*rt721).regmap = regmap;
    (*rt721).mbq_regmap = mbq_regmap;
    regcache_cache_only((*rt721).regmap, true);
    regcache_cache_only((*rt721).mbq_regmap, true);
    mutex_init(&mut (*rt721).calibrate_mutex);
    mutex_init(&mut (*rt721).disable_irq_lock);
    INIT_DELAYED_WORK(&mut (*rt721).jack_detect_work, rt721_sdca_jack_detect_handler);
    INIT_DELAYED_WORK(&mut (*rt721).jack_btn_check_work, rt721_sdca_btn_check_handler);
    /*
     * Mark hw_init to false
     * HW init will be performed when device reports present
     */
    (*rt721).hw_init = false;
    (*rt721).first_hw_init = false;
    (*rt721).fu1e_dapm_mute = true;
    (*rt721).fu0f_dapm_mute = true;
    (*rt721).fu0f_mixer_r_mute = true;
    (*rt721).fu0f_mixer_l_mute = (*rt721).fu0f_mixer_r_mute;
    (*rt721).fu1e_mixer_mute[3] = true;
    (*rt721).fu1e_mixer_mute[2] = (*rt721).fu1e_mixer_mute[3];
    (*rt721).fu1e_mixer_mute[1] = (*rt721).fu1e_mixer_mute[2];
    (*rt721).fu1e_mixer_mute[0] = (*rt721).fu1e_mixer_mute[1];
    devm_snd_soc_register_component(dev, &soc_sdca_dev_rt721, rt721_sdca_dai.as_mut_ptr(), rt721_sdca_dai.len() as c_int)
}

#[no_mangle]
pub unsafe extern "C" fn rt721_sdca_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int {
    let rt721 = dev_get_drvdata(dev);
    (*rt721).disable_irq = false;
    if (*rt721).hw_init { return 0; }
    regcache_cache_only((*rt721).regmap, false);
    regcache_cache_only((*rt721).mbq_regmap, false);
    if (*rt721).first_hw_init {
        regcache_cache_bypass((*rt721).regmap, true);
        regcache_cache_bypass((*rt721).mbq_regmap, true);
    } else {
        /*
         * PM runtime is only enabled when a Slave reports as Attached
         */

        /* set autosuspend parameters */
        pm_runtime_set_autosuspend_delay(&mut (*slave).dev, 3000);
        pm_runtime_use_autosuspend(&mut (*slave).dev);
        /* update count of parent 'active' children */
        pm_runtime_set_active(&mut (*slave).dev);
        /* make sure the device does not suspend immediately */
        pm_runtime_mark_last_busy(&mut (*slave).dev);
        pm_runtime_enable(&mut (*slave).dev);
    }
    pm_runtime_get_noresume(&mut (*slave).dev);
    rt721_sdca_dmic_preset(rt721);
    rt721_sdca_amp_preset(rt721);
    rt721_sdca_jack_preset(rt721);
    if (*rt721).first_hw_init {
        regcache_cache_bypass((*rt721).regmap, false);
        regcache_mark_dirty((*rt721).regmap);
        regcache_cache_bypass((*rt721).mbq_regmap, false);
        regcache_mark_dirty((*rt721).mbq_regmap);
    } else {
        (*rt721).first_hw_init = true;
    }
    /* Mark Slave initialization complete */
    (*rt721).hw_init = true;
    pm_runtime_put_autosuspend(&mut (*slave).dev);
    dev_dbg(&mut (*slave).dev, cstr!("%s hw_init complete\n"), cstr!("rt721_sdca_io_init"));
    0
}

/* MODULE_DESCRIPTION("ASoC RT721 SDCA SDW driver"); */
/* MODULE_AUTHOR("Jack Yu <jack.yu@realtek.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
