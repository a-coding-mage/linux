// SPDX-License-Identifier: GPL-2.0-only
//
// es9356.c -- SoundWire codec driver
//
// Copyright(c) 2025 Everest Semiconductor Co., Ltd
//
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

// Dependencies originally provided by Linux, SoundWire, ASoC, TLV, and "es9356.h".
extern "C" {
    static mut system_power_efficient_wq: *mut workqueue_struct;

    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
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
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);

    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_put_autosuspend(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);

    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_cache_bypass(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn devm_regmap_init_sdw_mbq_cfg(
        dev: *mut device,
        slave: *mut sdw_slave,
        config: *const regmap_config,
        mbq_config: *const regmap_sdw_mbq_cfg,
    ) -> *mut regmap;

    fn sdw_write_no_pm(slave: *mut sdw_slave, addr: c_uint, val: c_uint) -> c_int;
    fn sdw_read_no_pm(slave: *mut sdw_slave, addr: c_uint) -> c_int;
    fn sdw_update_no_pm(slave: *mut sdw_slave, addr: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn sdw_stream_add_slave(
        slave: *mut sdw_slave,
        stream_config: *mut sdw_stream_config,
        port_config: *mut sdw_port_config,
        num_ports: c_uint,
        stream: *mut sdw_stream_runtime,
    ) -> c_int;
    fn sdw_stream_remove_slave(slave: *mut sdw_slave, stream: *mut sdw_stream_runtime);
    fn snd_sdw_params_to_config(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        stream_config: *mut sdw_stream_config,
        port_config: *mut sdw_port_config,
    );
    fn sdw_slave_wait_for_init(slave: *mut sdw_slave, timeout: c_int) -> c_int;
    fn sdw_show_ping_status(bus: *mut sdw_bus, status: bool);

    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: c_int, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn pr_err_ratelimited(fmt: *const c_char, ...);

    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_destroy(lock: *mut mutex);
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn msecs_to_jiffies(msecs: c_uint) -> c_ulong;
    fn mod_delayed_work(wq: *mut workqueue_struct, work: *mut delayed_work, delay: c_ulong) -> bool;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn INIT_DELAYED_WORK(work: *mut delayed_work, func: extern "C" fn(*mut work_struct));
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_to_sdw_dev(dev: *mut device) -> *mut sdw_slave;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn hweight32(w: c_uint) -> c_int;
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
pub struct snd_soc_jack {
    _private: [u8; 0],
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
pub struct sdw_stream_runtime {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sdw_bus {
    _private: [u8; 0],
}
#[repr(C)]
pub struct workqueue_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
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
pub struct snd_soc_card {
    pub instantiated: bool,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub component: *mut snd_soc_component,
    pub id: c_int,
}

#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
    pub prop: sdw_slave_prop,
    pub unattach_request: bool,
    pub bus: *mut sdw_bus,
}

#[repr(C)]
pub struct sdw_slave_prop {
    pub paging_support: bool,
    pub source_ports: c_uint,
    pub sink_ports: c_uint,
    pub src_dpn_prop: *mut sdw_dpn_prop,
    pub sink_dpn_prop: *mut sdw_dpn_prop,
    pub wake_capable: c_int,
}

#[repr(C)]
pub struct sdw_dpn_prop {
    pub num: u32,
    pub type_: c_int,
    pub simple_ch_prep_sm: bool,
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
pub struct sdw_slave_intr_status {
    pub sdca_cascade: bool,
}

pub type sdw_slave_status = c_uint;

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct soc_enum {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_route {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub set_jack: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_jack, *mut c_void) -> c_int>,
    pub endianness: c_int,
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
pub struct regmap_sdw_mbq_cfg {
    pub mbq_size: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub max_register: c_uint,
    pub cache_type: c_uint,
    pub use_single_read: bool,
    pub use_single_write: bool,
}

#[repr(C)]
pub struct sdw_slave_ops {
    pub read_prop: Option<unsafe extern "C" fn(*mut sdw_slave) -> c_int>,
    pub interrupt_callback: Option<unsafe extern "C" fn(*mut sdw_slave, *mut sdw_slave_intr_status) -> c_int>,
    pub update_status: Option<unsafe extern "C" fn(*mut sdw_slave, sdw_slave_status) -> c_int>,
}

#[repr(C)]
pub struct sdw_device_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct sdw_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut sdw_slave, *const sdw_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut sdw_slave)>,
    pub ops: *const sdw_slave_ops,
    pub id_table: *const sdw_device_id,
}

#[repr(C)]
pub struct es9356_sdw_priv {
    pub slave: *mut sdw_slave,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub hs_jack: *mut snd_soc_jack,

    /* lock for irq*/
    pub disable_irq_lock: mutex,

    /* lock for pde*/
    pub pde_lock: mutex,

    pub hw_init: bool,
    pub first_hw_init: bool,
    pub jack_type: c_int,
    pub disable_irq: bool,

    pub interrupt_handle_work: delayed_work,
    pub button_detect_work: delayed_work,
    pub sdca_status: c_uint,
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EACCES: c_int = 13;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;
const SND_SOC_NOPM: c_int = 0;
const SDW_DPN_FULL: c_int = 0;
const SDW_SLAVE_UNATTACHED: sdw_slave_status = 0;
const SDW_SLAVE_ATTACHED: sdw_slave_status = 1;

const SND_JACK_HEADPHONE: c_int = 0x0001;
const SND_JACK_HEADSET: c_int = 0x0002;
const SND_JACK_BTN_0: c_int = 0x4000;
const SND_JACK_BTN_1: c_int = 0x2000;
const SND_JACK_BTN_2: c_int = 0x1000;
const SND_JACK_BTN_3: c_int = 0x0800;
const SND_JACK_BTN_4: c_int = 0x0400;

const SDW_SCP_SDCA_INTMASK1: c_uint = 0;
const SDW_SCP_SDCA_INT1: c_uint = 0;
const SDW_DP0_INT: c_uint = 0;
const SDW_DP0_SDCA_CASCADE: c_uint = 0;
const SDW_SCP_SDCA_INTMASK_SDCA_1: c_uint = 1 << 1;
const SDW_SCP_SDCA_INTMASK_SDCA_5: c_uint = 1 << 5;
const SDW_SCP_SDCA_INTMASK_SDCA_7: c_uint = 1 << 7;
const SDW_SCP_SDCA_INT_SDCA_1: c_uint = 1 << 1;
const SDW_SCP_SDCA_INT_SDCA_5: c_uint = 1 << 5;
const SDW_SCP_SDCA_INT_SDCA_7: c_uint = 1 << 7;

extern "C" {
    static out_vol_tlv: c_uint;
    static amic_gain_tlv: c_uint;
    static dmic_gain_tlv: c_uint;
}

extern "C" {
    fn SDW_SDCA_CTL(func: c_uint, entity: c_uint, ctl: c_uint, ch: c_uint) -> c_uint;
    fn BIT(nr: c_uint) -> c_uint;
    fn pm_ptr(pm: *const dev_pm_ops) -> *const dev_pm_ops;
}

extern "C" {
    static FUNC_NUM_UAJ: c_uint;
    static FUNC_NUM_AMP: c_uint;
    static FUNC_NUM_MIC: c_uint;
    static FUNC_NUM_HID: c_uint;
    static CH_L: c_uint;
    static CH_R: c_uint;
    static ES9356_DMIC: c_int;
    static ES9356_AMP: c_int;
    static ES9356_JACK_IN: c_int;
    static ES9356_JACK_OUT: c_int;
    static ES9356_VOLUME_MIN: c_uint;
    static ES9356_VOLUME_MAX: c_uint;
    static ES9356_VOLUME_STEP: c_uint;
    static ES9356_GAIN_MIN: c_uint;
    static ES9356_AMIC_GAIN_MAX: c_uint;
    static ES9356_AMIC_GAIN_STEP: c_uint;
    static ES9356_DMIC_GAIN_MAX: c_uint;
    static ES9356_DMIC_GAIN_STEP: c_uint;
    static ES9356_DEFAULT_VOLUME: c_uint;
    static ES9356_SDCA_RATE_16000HZ: c_uint;
    static ES9356_SDCA_RATE_44100HZ: c_uint;
    static ES9356_SDCA_RATE_48000HZ: c_uint;
    static ES9356_SDCA_RATE_96000HZ: c_uint;
}

extern "C" {
    static ES9356_DAC_SWAP: c_uint;
    static ES9356_DMIC_GPIO: c_uint;
    static ES9356_BUF_ADDR_HID: c_uint;
    static ES9356_HID_BYTE2: c_uint;
    static ES9356_HID_BYTE3: c_uint;
    static ES9356_HID_BYTE4: c_uint;
    static ES9356_FLAGS_HP: c_uint;
    static ES9356_HP_DETECTTIME: c_uint;
    static ES9356_STATE: c_uint;
    static ES9356_ENDPOINT_MODE: c_uint;
    static ES9356_PRE_DIV_CTL: c_uint;
    static ES9356_ADC_OSR: c_uint;
    static ES9356_ADC_OSRGAIN: c_uint;
    static ES9356_DAC_OSR: c_uint;
    static ES9356_CLK_CTL: c_uint;
    static ES9356_CSM_RESET: c_uint;
    static ES9356_CLK_SEL: c_uint;
    static ES9356_DETCLK_CTL: c_uint;
    static ES9356_HP_TYPE: c_uint;
    static ES9356_MICBIAS_CTL: c_uint;
    static ES9356_HPDETECT_CTL: c_uint;
    static ES9356_ADC_ANA: c_uint;
    static ES9356_PGA_CTL: c_uint;
    static ES9356_ADC_INT: c_uint;
    static ES9356_ADC_LP: c_uint;
    static ES9356_VMID1SEL: c_uint;
    static ES9356_VMID_TIME: c_uint;
    static ES9356_STATE_TIME: c_uint;
    static ES9356_HP_SPK_TIME: c_uint;
    static ES9356_MICBIAS_SEL: c_uint;
    static ES9356_KEY_PRESS_TIME: c_uint;
    static ES9356_KEY_RELEASE_TIME: c_uint;
    static ES9356_KEY_HOLD_TIME: c_uint;
    static ES9356_BTSEL_REF: c_uint;
    static ES9356_KEYD_DETECT: c_uint;
    static ES9356_MICBIAS_RES: c_uint;
    static ES9356_BUTTON_CHARGE: c_uint;
    static ES9356_CALIBRATION_TIME: c_uint;
    static ES9356_CALIBRATION_SETTING: c_uint;
    static ES9356_SPK_VOLUME: c_uint;
    static ES9356_DAC_VROI: c_uint;
    static ES9356_DAC_LP: c_uint;
    static ES9356_HP_IBIAS: c_uint;
    static ES9356_HP_LP: c_uint;
    static ES9356_SPKLDO_CTL: c_uint;
    static ES9356_SPKBIAS_COMP: c_uint;
    static ES9356_VMID1STL: c_uint;
    static ES9356_VMID2STL: c_uint;
    static ES9356_VSEL: c_uint;
    static ES9356_IBIASGEN: c_uint;
    static ES9356_ADC_AMIC_CTL: c_uint;
}

extern "C" {
    static ES9356_SDCA_ENT_FU41: c_uint;
    static ES9356_SDCA_ENT_FU36: c_uint;
    static ES9356_SDCA_ENT_FU33: c_uint;
    static ES9356_SDCA_ENT_FU21: c_uint;
    static ES9356_SDCA_ENT_FU113: c_uint;
    static ES9356_SDCA_ENT_FU11: c_uint;
    static ES9356_SDCA_ENT_HID01: c_uint;
    static ES9356_SDCA_ENT_GE35: c_uint;
    static ES9356_SDCA_ENT_CS113: c_uint;
    static ES9356_SDCA_ENT_PDE11: c_uint;
    static ES9356_SDCA_ENT_CS21: c_uint;
    static ES9356_SDCA_ENT_PDE23: c_uint;
    static ES9356_SDCA_ENT_CS36: c_uint;
    static ES9356_SDCA_ENT_PDE34: c_uint;
    static ES9356_SDCA_ENT_CS41: c_uint;
    static ES9356_SDCA_ENT_PDE47: c_uint;
    static ES9356_SDCA_ENT_XU12: c_uint;
    static ES9356_SDCA_ENT_XU22: c_uint;
    static ES9356_SDCA_ENT_XU42: c_uint;
    static ES9356_SDCA_ENT_XU36: c_uint;
    static ES9356_SDCA_ENT0: c_uint;
    static ES9356_SDCA_CTL_FU_VOLUME: c_uint;
    static ES9356_SDCA_CTL_FU_CH_GAIN: c_uint;
    static ES9356_SDCA_CTL_FU_MUTE: c_uint;
    static ES9356_SDCA_CTL_HIDTX_CURRENT_OWNER: c_uint;
    static ES9356_SDCA_CTL_DETECTED_MODE: c_uint;
    static ES9356_SDCA_CTL_SELECTED_MODE: c_uint;
    static ES9356_SDCA_CTL_ACTUAL_POWER_STATE: c_uint;
    static ES9356_SDCA_CTL_REQ_POWER_STATE: c_uint;
    static ES9356_SDCA_CTL_SAMPLE_FREQ_INDEX: c_uint;
    static ES9356_SDCA_CTL_FUNC_STATUS: c_uint;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! ARRAY_SIZE {
    ($array:expr) => {
        ($array.len() as c_uint)
    };
}

macro_rules! SDCA_SINGLE_Q78_TLV {
    ($($tt:tt)*) => {
        snd_kcontrol_new { _private: [] }
    };
}
macro_rules! SOC_ENUM_SINGLE {
    ($($tt:tt)*) => {
        soc_enum { _private: [] }
    };
}
macro_rules! SOC_DAPM_ENUM {
    ($($tt:tt)*) => {
        snd_kcontrol_new { _private: [] }
    };
}
macro_rules! SND_SOC_DAPM_OUTPUT {
    ($($tt:tt)*) => {
        snd_soc_dapm_widget { _private: [] }
    };
}
macro_rules! SND_SOC_DAPM_INPUT {
    ($($tt:tt)*) => {
        snd_soc_dapm_widget { _private: [] }
    };
}
macro_rules! SND_SOC_DAPM_SUPPLY {
    ($($tt:tt)*) => {
        snd_soc_dapm_widget { _private: [] }
    };
}
macro_rules! SND_SOC_DAPM_AIF_IN {
    ($($tt:tt)*) => {
        snd_soc_dapm_widget { _private: [] }
    };
}
macro_rules! SND_SOC_DAPM_AIF_OUT {
    ($($tt:tt)*) => {
        snd_soc_dapm_widget { _private: [] }
    };
}
macro_rules! SND_SOC_DAPM_PGA {
    ($($tt:tt)*) => {
        snd_soc_dapm_widget { _private: [] }
    };
}
macro_rules! SND_SOC_DAPM_MUX {
    ($($tt:tt)*) => {
        snd_soc_dapm_widget { _private: [] }
    };
}
macro_rules! SND_SOC_DAPM_DAC {
    ($($tt:tt)*) => {
        snd_soc_dapm_widget { _private: [] }
    };
}

unsafe extern "C" fn es9356_sdw_component_probe(component: *mut snd_soc_component) -> c_int {
    let es9356 = snd_soc_component_get_drvdata(component) as *mut es9356_sdw_priv;

    (*es9356).component = component;

    0
}

static es9356_sdca_controls: [snd_kcontrol_new; 10] = [
    SDCA_SINGLE_Q78_TLV!("FU41 Left Playback Volume", SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_FU41, ES9356_SDCA_CTL_FU_VOLUME, CH_L), ES9356_VOLUME_MIN, ES9356_VOLUME_MAX, ES9356_VOLUME_STEP, out_vol_tlv),
    SDCA_SINGLE_Q78_TLV!("FU41 Right Playback Volume", SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_FU41, ES9356_SDCA_CTL_FU_VOLUME, CH_R), ES9356_VOLUME_MIN, ES9356_VOLUME_MAX, ES9356_VOLUME_STEP, out_vol_tlv),
    SDCA_SINGLE_Q78_TLV!("FU36 Left Capture Volume", SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_FU36, ES9356_SDCA_CTL_FU_VOLUME, CH_L), ES9356_VOLUME_MIN, ES9356_VOLUME_MAX, ES9356_VOLUME_STEP, out_vol_tlv),
    SDCA_SINGLE_Q78_TLV!("FU36 Right Capture Volume", SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_FU36, ES9356_SDCA_CTL_FU_VOLUME, CH_R), ES9356_VOLUME_MIN, ES9356_VOLUME_MAX, ES9356_VOLUME_STEP, out_vol_tlv),
    SDCA_SINGLE_Q78_TLV!("FU33 Capture Volume", SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_FU33, ES9356_SDCA_CTL_FU_CH_GAIN, 0), ES9356_GAIN_MIN, ES9356_AMIC_GAIN_MAX, ES9356_AMIC_GAIN_STEP, amic_gain_tlv),
    SDCA_SINGLE_Q78_TLV!("FU21 Left Playback Volume", SDW_SDCA_CTL(FUNC_NUM_AMP, ES9356_SDCA_ENT_FU21, ES9356_SDCA_CTL_FU_VOLUME, CH_L), ES9356_VOLUME_MIN, ES9356_VOLUME_MAX, ES9356_VOLUME_STEP, out_vol_tlv),
    SDCA_SINGLE_Q78_TLV!("FU21 Right Playback Volume", SDW_SDCA_CTL(FUNC_NUM_AMP, ES9356_SDCA_ENT_FU21, ES9356_SDCA_CTL_FU_VOLUME, CH_R), ES9356_VOLUME_MIN, ES9356_VOLUME_MAX, ES9356_VOLUME_STEP, out_vol_tlv),
    SDCA_SINGLE_Q78_TLV!("FU113 Left Capture Volume", SDW_SDCA_CTL(FUNC_NUM_MIC, ES9356_SDCA_ENT_FU113, ES9356_SDCA_CTL_FU_VOLUME, CH_L), ES9356_VOLUME_MIN, ES9356_VOLUME_MAX, ES9356_VOLUME_STEP, out_vol_tlv),
    SDCA_SINGLE_Q78_TLV!("FU113 Right Capture Volume", SDW_SDCA_CTL(FUNC_NUM_MIC, ES9356_SDCA_ENT_FU113, ES9356_SDCA_CTL_FU_VOLUME, CH_R), ES9356_VOLUME_MIN, ES9356_VOLUME_MAX, ES9356_VOLUME_STEP, out_vol_tlv),
    SDCA_SINGLE_Q78_TLV!("FU11 Capture Volume", SDW_SDCA_CTL(FUNC_NUM_MIC, ES9356_SDCA_ENT_FU11, ES9356_SDCA_CTL_FU_CH_GAIN, 0), ES9356_GAIN_MIN, ES9356_DMIC_GAIN_MAX, ES9356_DMIC_GAIN_STEP, dmic_gain_tlv),
];

static es9356_left_mux_txt: [*const c_char; 2] = [cstr!("Left"), cstr!("Right")];
static es9356_right_mux_txt: [*const c_char; 2] = [cstr!("Right"), cstr!("Left")];

static es9356_left_mux_enum: soc_enum =
    SOC_ENUM_SINGLE!(ES9356_DAC_SWAP, 1, ARRAY_SIZE!(es9356_left_mux_txt), es9356_left_mux_txt);
static es9356_right_mux_enum: soc_enum =
    SOC_ENUM_SINGLE!(ES9356_DAC_SWAP, 0, ARRAY_SIZE!(es9356_right_mux_txt), es9356_right_mux_txt);

static es9356_left_mux_controls: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Channel MUX", es9356_left_mux_enum);
static es9356_right_mux_controls: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Channel MUX", es9356_right_mux_enum);

static es9356_dapm_widgets: [snd_soc_dapm_widget; 20] = [
    SND_SOC_DAPM_OUTPUT!("HP"),
    SND_SOC_DAPM_OUTPUT!("SPK"),
    SND_SOC_DAPM_INPUT!("MIC1"),
    SND_SOC_DAPM_INPUT!("PDM_DIN"),
    SND_SOC_DAPM_SUPPLY!("DMIC Clock", ES9356_DMIC_GPIO, 1, 1, ptr::null::<c_void>(), 0),
    SND_SOC_DAPM_AIF_IN!("DP4RX", "DP4 Playback", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!("DP3RX", "DP3 Playback", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("DP1TX", "DP1 Capture", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("DP2TX", "DP2 Capture", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_PGA!("IF DP3RXL", SND_SOC_NOPM, 0, 0, ptr::null::<c_void>(), 0),
    SND_SOC_DAPM_PGA!("IF DP3RXR", SND_SOC_NOPM, 0, 0, ptr::null::<c_void>(), 0),
    SND_SOC_DAPM_MUX!("Left Channel MUX", SND_SOC_NOPM, 0, 0, &es9356_left_mux_controls),
    SND_SOC_DAPM_MUX!("Right Channel MUX", SND_SOC_NOPM, 0, 0, &es9356_right_mux_controls),
    SND_SOC_DAPM_DAC!("FU 21 Left", ptr::null::<c_void>(), SDW_SDCA_CTL(FUNC_NUM_AMP, ES9356_SDCA_ENT_FU21, ES9356_SDCA_CTL_FU_MUTE, CH_L), 0, 1),
    SND_SOC_DAPM_DAC!("FU 21 Right", ptr::null::<c_void>(), SDW_SDCA_CTL(FUNC_NUM_AMP, ES9356_SDCA_ENT_FU21, ES9356_SDCA_CTL_FU_MUTE, CH_R), 0, 1),
    SND_SOC_DAPM_DAC!("FU 41 Left", ptr::null::<c_void>(), SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_FU41, ES9356_SDCA_CTL_FU_MUTE, CH_L), 0, 1),
    SND_SOC_DAPM_DAC!("FU 41 Right", ptr::null::<c_void>(), SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_FU41, ES9356_SDCA_CTL_FU_MUTE, CH_R), 0, 1),
    SND_SOC_DAPM_DAC!("FU 113 Left", ptr::null::<c_void>(), SDW_SDCA_CTL(FUNC_NUM_MIC, ES9356_SDCA_ENT_FU113, ES9356_SDCA_CTL_FU_MUTE, CH_L), 0, 1),
    SND_SOC_DAPM_DAC!("FU 113 Right", ptr::null::<c_void>(), SDW_SDCA_CTL(FUNC_NUM_MIC, ES9356_SDCA_ENT_FU113, ES9356_SDCA_CTL_FU_MUTE, CH_R), 0, 1),
    SND_SOC_DAPM_DAC!("FU 36 Left", ptr::null::<c_void>(), SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_FU36, ES9356_SDCA_CTL_FU_MUTE, CH_L), 0, 1),
];

static es9356_audio_map: [snd_soc_dapm_route; 24] = [snd_soc_dapm_route { _private: [] }; 24];

unsafe extern "C" fn es9356_set_jack_detect(
    component: *mut snd_soc_component,
    hs_jack: *mut snd_soc_jack,
    _data: *mut c_void,
) -> c_int {
    let es9356 = snd_soc_component_get_drvdata(component) as *mut es9356_sdw_priv;
    let mut ret: c_int;

    (*es9356).hs_jack = hs_jack;

    /* we can only resume if the device was initialized at least once */
    if !(*es9356).first_hw_init {
        return 0;
    }

    ret = pm_runtime_resume_and_get((*component).dev);
    if ret < 0 {
        if ret != -EACCES {
            dev_err((*component).dev, cstr!("%s: failed to resume %d\n"), cstr!("es9356_set_jack_detect"), ret);
            return ret;
        }
        /* pm_runtime not enabled yet */
        dev_info((*component).dev, cstr!("%s: skipping jack init for now\n"), cstr!("es9356_set_jack_detect"));
        return 0;
    }

    if !(*es9356).hs_jack.is_null() {
        sdw_write_no_pm(
            (*es9356).slave,
            SDW_SCP_SDCA_INTMASK1,
            SDW_SCP_SDCA_INTMASK_SDCA_7 | SDW_SCP_SDCA_INTMASK_SDCA_5 | SDW_SCP_SDCA_INTMASK_SDCA_1,
        );
    }

    pm_runtime_mark_last_busy((*component).dev);
    pm_runtime_put_autosuspend((*component).dev);

    0
}

static snd_soc_es9356_sdw_component: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(es9356_sdw_component_probe),
    controls: es9356_sdca_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(es9356_sdca_controls),
    dapm_widgets: es9356_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(es9356_dapm_widgets),
    dapm_routes: es9356_audio_map.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(es9356_audio_map),
    set_jack: Some(es9356_set_jack_detect),
    endianness: 1,
};

unsafe extern "C" fn es9356_sdw_set_sdw_stream(
    dai: *mut snd_soc_dai,
    sdw_stream: *mut c_void,
    direction: c_int,
) -> c_int {
    snd_soc_dai_dma_data_set(dai, direction, sdw_stream);

    0
}

unsafe extern "C" fn es9356_sdw_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    snd_soc_dai_set_dma_data(dai, substream, ptr::null_mut());
}

unsafe fn es9356_sdca_button(buffer: *mut c_uint) -> c_int {
    let mut cur_button: c_int = -1;

    if *buffer.add(1) | *buffer.add(2) != 0 {
        return -EINVAL;
    }
    match *buffer {
        0x00 => cur_button = 0,
        0x20 => cur_button = SND_JACK_BTN_4,
        0x10 => cur_button = SND_JACK_BTN_2,
        0x08 => cur_button = SND_JACK_BTN_1,
        0x02 => cur_button = SND_JACK_BTN_3,
        0x01 => cur_button = SND_JACK_BTN_0,
        _ => {}
    }

    cur_button
}

unsafe fn es9356_sdca_button_detect(es9356: *mut es9356_sdw_priv) -> c_int {
    let mut btn_type: c_uint = 0;
    let mut offset: c_uint = 0;
    let mut idx: c_uint;
    let mut val: c_uint = 0;
    let mut owner: c_uint = 0;
    let mut button: [c_uint; 3] = [0; 3];
    let mut ret: c_int;

    ret = regmap_read(
        (*es9356).regmap,
        SDW_SDCA_CTL(FUNC_NUM_HID, ES9356_SDCA_ENT_HID01, ES9356_SDCA_CTL_HIDTX_CURRENT_OWNER, 0),
        &mut owner,
    );
    if ret < 0 || owner == 0x01 {
        return 0;
    }

    ret = regmap_read((*es9356).regmap, ES9356_BUF_ADDR_HID, &mut offset);
    if ret < 0 {
        goto_button_det_end(es9356, owner);
        return btn_type as c_int;
    }

    idx = 0;
    while idx < button.len() as c_uint {
        ret = regmap_read((*es9356).regmap, ES9356_BUF_ADDR_HID + offset + idx, &mut val);
        if ret < 0 {
            goto_button_det_end(es9356, owner);
            return btn_type as c_int;
        }
        button[idx as usize] = val;
        idx += 1;
    }

    btn_type = es9356_sdca_button(&mut button[0]) as c_uint;

    goto_button_det_end(es9356, owner);
    btn_type as c_int
}

unsafe fn goto_button_det_end(es9356: *mut es9356_sdw_priv, owner: c_uint) {
    if owner == 0x00 {
        regmap_write(
            (*es9356).regmap,
            SDW_SDCA_CTL(FUNC_NUM_HID, ES9356_SDCA_ENT_HID01, ES9356_SDCA_CTL_HIDTX_CURRENT_OWNER, 0),
            0x01,
        );
    }
}

unsafe fn es9356_sdca_headset_detect(es9356: *mut es9356_sdw_priv) -> c_int {
    let mut reg: c_uint = 0;
    let ret: c_int;

    ret = regmap_read(
        (*es9356).regmap,
        SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_GE35, ES9356_SDCA_CTL_DETECTED_MODE, 0),
        &mut reg,
    );

    if ret < 0 {
        pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("es9356_sdca_headset_detect"), ret);
        return ret;
    }

    match reg {
        0x00 => (*es9356).jack_type = 0,
        0x03 => (*es9356).jack_type = SND_JACK_HEADPHONE,
        0x04 => (*es9356).jack_type = SND_JACK_HEADSET,
        _ => {
            (*es9356).jack_type = 0;
            return -1;
        }
    }

    if reg != 0 {
        regmap_write(
            (*es9356).regmap,
            SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_GE35, ES9356_SDCA_CTL_SELECTED_MODE, 0),
            reg,
        );
        regmap_write((*es9356).regmap, ES9356_HP_DETECTTIME, 0x75);
    } else {
        regmap_write((*es9356).regmap, ES9356_HP_DETECTTIME, 0xa4);
    }

    0
}

unsafe extern "C" fn es9356_interrupt_handler(work: *mut work_struct) {
    let es9356 = (work as *mut u8).offset(-(0isize)) as *mut es9356_sdw_priv;
    let mut ret: c_int;
    let mut btn_type: c_int = 0;

    if (*es9356).hs_jack.is_null() {
        return;
    }

    if (*(*es9356).component).card.is_null() || !(*(*(*es9356).component).card).instantiated {
        return;
    }

    /* Handling different types of interrupts based on the mask bit */
    if (*es9356).sdca_status & SDW_SCP_SDCA_INT_SDCA_7 != 0 {
        btn_type = es9356_sdca_button_detect(es9356);
        if btn_type < 0 {
            return;
        }
    } else {
        ret = es9356_sdca_headset_detect(es9356);
        if ret < 0 {
            return;
        }
    }

    if (*es9356).jack_type != SND_JACK_HEADSET {
        btn_type = 0;
    }

    snd_soc_jack_report(
        (*es9356).hs_jack,
        (*es9356).jack_type | btn_type,
        SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3 | SND_JACK_BTN_4,
    );

    if btn_type != 0 {
        snd_soc_jack_report(
            (*es9356).hs_jack,
            (*es9356).jack_type,
            SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3 | SND_JACK_BTN_4,
        );
        mod_delayed_work(
            system_power_efficient_wq,
            &mut (*es9356).button_detect_work,
            msecs_to_jiffies(280),
        );
    }
}

unsafe extern "C" fn es9356_button_detect_handler(work: *mut work_struct) {
    let es9356 = (work as *mut u8).offset(-(0isize)) as *mut es9356_sdw_priv;
    let mut ret: c_int;
    let mut idx: c_int;
    let mut btn_type: c_int = 0;
    let mut reg: c_uint = 0;
    let mut offset: c_uint = 0;
    let mut button: [c_uint; 3] = [0; 3];

    /* Check headset */
    ret = regmap_read(
        (*es9356).regmap,
        SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_GE35, ES9356_SDCA_CTL_DETECTED_MODE, 0),
        &mut reg,
    );

    if ret < 0 {
        pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("es9356_button_detect_handler"), ret);
        return;
    }

    if reg == 0x04 {
        ret = regmap_read((*es9356).regmap, ES9356_BUF_ADDR_HID, &mut offset);
        if ret < 0 {
            pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("es9356_button_detect_handler"), ret);
            return;
        }
        idx = 0;
        while idx < button.len() as c_int {
            ret = regmap_read((*es9356).regmap, ES9356_BUF_ADDR_HID + offset + idx as c_uint, &mut reg);
            if ret < 0 {
                pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("es9356_button_detect_handler"), ret);
                return;
            }
            button[idx as usize] = reg;
            idx += 1;
        }
        btn_type = es9356_sdca_button(&mut button[0]);
        if btn_type < 0 {
            return;
        }
    }

    snd_soc_jack_report(
        (*es9356).hs_jack,
        (*es9356).jack_type | btn_type,
        SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3 | SND_JACK_BTN_4,
    );

    if btn_type != 0 {
        snd_soc_jack_report(
            (*es9356).hs_jack,
            (*es9356).jack_type,
            SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3 | SND_JACK_BTN_4,
        );
        mod_delayed_work(
            system_power_efficient_wq,
            &mut (*es9356).button_detect_work,
            msecs_to_jiffies(280),
        );
    }
}

unsafe fn es9356_pde_transition_delay(
    es9356: *mut es9356_sdw_priv,
    func: u8,
    entity: u8,
    ps: u8,
) -> c_int {
    let mut retries: c_uint = 10;
    let mut val: c_uint = 0;

    /* waiting for Actual PDE becomes to PS0/PS3 */
    while retries != 0 {
        regmap_read(
            (*es9356).regmap,
            SDW_SDCA_CTL(func as c_uint, entity as c_uint, ES9356_SDCA_CTL_ACTUAL_POWER_STATE, 0),
            &mut val,
        );
        if val == ps as c_uint {
            return 1;
        }

        usleep_range(1000, 1500);
        retries -= 1;
    }
    if retries == 0 {
        dev_dbg(&mut (*(*es9356).slave).dev, cstr!("%s PDE is NOT %s"), cstr!("es9356_pde_transition_delay"), if ps != 0 { cstr!("PS3") } else { cstr!("PS0") });
    }
    0
}

unsafe fn es9356_power_state(dai: *mut snd_soc_dai, ps: u8, rate: *mut c_uint) -> c_int {
    let component = (*dai).component;
    let es9356 = snd_soc_component_get_drvdata(component) as *mut es9356_sdw_priv;
    let ps0: u8 = 0x0;
    let ps3: u8 = 0x3;
    let func: u8;
    let cs_entity: u8;
    let pde_entity: u8;
    let mut ret: c_int;

    if (*dai).id == ES9356_DMIC {
        func = FUNC_NUM_MIC as u8;
        cs_entity = ES9356_SDCA_ENT_CS113 as u8;
        pde_entity = ES9356_SDCA_ENT_PDE11 as u8;
    } else if (*dai).id == ES9356_AMP {
        func = FUNC_NUM_AMP as u8;
        cs_entity = ES9356_SDCA_ENT_CS21 as u8;
        pde_entity = ES9356_SDCA_ENT_PDE23 as u8;
    } else if (*dai).id == ES9356_JACK_IN {
        func = FUNC_NUM_UAJ as u8;
        cs_entity = ES9356_SDCA_ENT_CS36 as u8;
        pde_entity = ES9356_SDCA_ENT_PDE34 as u8;
    } else if (*dai).id == ES9356_JACK_OUT {
        func = FUNC_NUM_UAJ as u8;
        cs_entity = ES9356_SDCA_ENT_CS41 as u8;
        pde_entity = ES9356_SDCA_ENT_PDE47 as u8;
    } else {
        return -EINVAL;
    }

    /* power state changes are not independent across functions */
    mutex_lock(&mut (*es9356).pde_lock);
    ret = es9356_pde_transition_delay(es9356, func, pde_entity, if ps != 0 { ps0 } else { ps3 });
    if ret != 0 {
        regmap_write(
            (*es9356).regmap,
            SDW_SDCA_CTL(func as c_uint, pde_entity as c_uint, ES9356_SDCA_CTL_REQ_POWER_STATE, 0),
            if ps != 0 { ps3 as c_uint } else { ps0 as c_uint },
        );
        es9356_pde_transition_delay(es9356, func, pde_entity, if ps != 0 { ps3 } else { ps0 });
    } else {
        dev_dbg((*component).dev, cstr!("%s PDE is already %d\n"), cstr!("es9356_power_state"), if ps != 0 { ps0 as c_int } else { ps3 as c_int });
    }
    mutex_unlock(&mut (*es9356).pde_lock);

    if !rate.is_null() {
        regmap_write(
            (*es9356).regmap,
            SDW_SDCA_CTL(func as c_uint, cs_entity as c_uint, ES9356_SDCA_CTL_SAMPLE_FREQ_INDEX, 0),
            *rate,
        );
    }

    0
}

unsafe extern "C" fn es9356_sdw_pcm_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let es9356 = snd_soc_component_get_drvdata(component) as *mut es9356_sdw_priv;
    let mut stream_config: sdw_stream_config = core::mem::zeroed();
    let mut port_config: sdw_port_config = core::mem::zeroed();
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    let ps0: u8 = 0x0;
    let mut rate: c_uint = 0;
    let mut ret: c_int;

    if sdw_stream.is_null() {
        return -EINVAL;
    }

    if (*es9356).slave.is_null() {
        return -EINVAL;
    }

    /* SoundWire specific configuration */
    snd_sdw_params_to_config(substream, params, &mut stream_config, &mut port_config);

    port_config.num = (*dai).id;

    ret = sdw_stream_add_slave((*es9356).slave, &mut stream_config, &mut port_config, 1, sdw_stream);
    if ret != 0 {
        dev_err((*dai).dev, cstr!("Unable to configure port\n"));
        return -EINVAL;
    }

    match params_rate(params) {
        16000 => rate = ES9356_SDCA_RATE_16000HZ,
        44100 => rate = ES9356_SDCA_RATE_44100HZ,
        48000 => rate = ES9356_SDCA_RATE_48000HZ,
        96000 => rate = ES9356_SDCA_RATE_96000HZ,
        _ => {
            dev_err((*component).dev, cstr!("%s: Rate %d is not supported\n"), cstr!("es9356_sdw_pcm_hw_params"), params_rate(params));
            return -EINVAL;
        }
    }

    ret = es9356_power_state(dai, ps0, &mut rate);
    if ret != 0 {
        dev_err((*component).dev, cstr!("%s: Invalid dai id: %d\n"), cstr!("es9356_sdw_pcm_hw_params"), (*dai).id);
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn es9356_sdw_pcm_hw_free(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let es9356 = snd_soc_component_get_drvdata(component) as *mut es9356_sdw_priv;
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    let ps3: u8 = 0x3;
    let mut ret: c_int;

    if (*es9356).slave.is_null() {
        return -EINVAL;
    }

    sdw_stream_remove_slave((*es9356).slave, sdw_stream);

    ret = es9356_power_state(dai, ps3, ptr::null_mut());
    if ret != 0 {
        dev_err((*component).dev, cstr!("%s: Invalid dai id: %d\n"), cstr!("es9356_sdw_pcm_hw_free"), (*dai).id);
        return -EINVAL;
    }

    0
}

static es9356_sdw_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(es9356_sdw_pcm_hw_params),
    hw_free: Some(es9356_sdw_pcm_hw_free),
    set_stream: Some(es9356_sdw_set_sdw_stream),
    shutdown: Some(es9356_sdw_shutdown),
};

static mut es9356_sdw_dai: [snd_soc_dai_driver; 4] = [
    snd_soc_dai_driver { name: cstr!("es9356-sdp-aif4"), id: 0, playback: snd_soc_pcm_stream { stream_name: ptr::null(), channels_min: 0, channels_max: 0 }, capture: snd_soc_pcm_stream { stream_name: cstr!("DP1 Capture"), channels_min: 1, channels_max: 2 }, ops: &es9356_sdw_ops },
    snd_soc_dai_driver { name: cstr!("es9356-sdp-aif2"), id: 0, playback: snd_soc_pcm_stream { stream_name: ptr::null(), channels_min: 0, channels_max: 0 }, capture: snd_soc_pcm_stream { stream_name: cstr!("DP2 Capture"), channels_min: 1, channels_max: 2 }, ops: &es9356_sdw_ops },
    snd_soc_dai_driver { name: cstr!("es9356-sdp-aif3"), id: 0, playback: snd_soc_pcm_stream { stream_name: cstr!("DP3 Playback"), channels_min: 1, channels_max: 2 }, capture: snd_soc_pcm_stream { stream_name: ptr::null(), channels_min: 0, channels_max: 0 }, ops: &es9356_sdw_ops },
    snd_soc_dai_driver { name: cstr!("es9356-sdp-aif1"), id: 0, playback: snd_soc_pcm_stream { stream_name: cstr!("DP4 Playback"), channels_min: 1, channels_max: 2 }, capture: snd_soc_pcm_stream { stream_name: ptr::null(), channels_min: 0, channels_max: 0 }, ops: &es9356_sdw_ops },
];

unsafe extern "C" fn es9356_sdca_mbq_size(_dev: *mut device, reg: c_uint) -> c_int {
    if reg == SDW_SDCA_CTL(FUNC_NUM_MIC, ES9356_SDCA_ENT_FU113, ES9356_SDCA_CTL_FU_VOLUME, CH_L)
        || reg == SDW_SDCA_CTL(FUNC_NUM_MIC, ES9356_SDCA_ENT_FU113, ES9356_SDCA_CTL_FU_VOLUME, CH_R)
        || reg == SDW_SDCA_CTL(FUNC_NUM_AMP, ES9356_SDCA_ENT_FU21, ES9356_SDCA_CTL_FU_VOLUME, CH_L)
        || reg == SDW_SDCA_CTL(FUNC_NUM_AMP, ES9356_SDCA_ENT_FU21, ES9356_SDCA_CTL_FU_VOLUME, CH_R)
        || reg == SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_FU41, ES9356_SDCA_CTL_FU_VOLUME, CH_L)
        || reg == SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_FU41, ES9356_SDCA_CTL_FU_VOLUME, CH_R)
        || reg == SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_FU36, ES9356_SDCA_CTL_FU_VOLUME, CH_L)
        || reg == SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_FU36, ES9356_SDCA_CTL_FU_VOLUME, CH_R)
        || reg == SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_FU33, ES9356_SDCA_CTL_FU_CH_GAIN, 0)
        || reg == SDW_SDCA_CTL(FUNC_NUM_MIC, ES9356_SDCA_ENT_FU11, ES9356_SDCA_CTL_FU_CH_GAIN, 0)
    {
        2
    } else {
        1
    }
}

static es9356_mbq_config: regmap_sdw_mbq_cfg = regmap_sdw_mbq_cfg {
    mbq_size: Some(es9356_sdca_mbq_size),
};

unsafe extern "C" fn es9356_sdca_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    reg == ES9356_BUF_ADDR_HID
        || reg == ES9356_HID_BYTE2
        || reg == ES9356_HID_BYTE3
        || reg == ES9356_HID_BYTE4
        || reg == SDW_SDCA_CTL(FUNC_NUM_HID, ES9356_SDCA_ENT_HID01, ES9356_SDCA_CTL_HIDTX_CURRENT_OWNER, 0)
        || reg == SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_GE35, ES9356_SDCA_CTL_DETECTED_MODE, 0)
        || reg == SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_GE35, ES9356_SDCA_CTL_SELECTED_MODE, 0)
        || reg == SDW_SDCA_CTL(FUNC_NUM_AMP, ES9356_SDCA_ENT_PDE23, ES9356_SDCA_CTL_REQ_POWER_STATE, 0)
        || reg == SDW_SDCA_CTL(FUNC_NUM_AMP, ES9356_SDCA_ENT_PDE23, ES9356_SDCA_CTL_ACTUAL_POWER_STATE, 0)
        || reg == SDW_SDCA_CTL(FUNC_NUM_MIC, ES9356_SDCA_ENT_PDE11, ES9356_SDCA_CTL_REQ_POWER_STATE, 0)
        || reg == SDW_SDCA_CTL(FUNC_NUM_MIC, ES9356_SDCA_ENT_PDE11, ES9356_SDCA_CTL_ACTUAL_POWER_STATE, 0)
        || reg == SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_PDE47, ES9356_SDCA_CTL_REQ_POWER_STATE, 0)
        || reg == SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_PDE47, ES9356_SDCA_CTL_ACTUAL_POWER_STATE, 0)
        || reg == SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_PDE34, ES9356_SDCA_CTL_REQ_POWER_STATE, 0)
        || reg == SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_PDE34, ES9356_SDCA_CTL_ACTUAL_POWER_STATE, 0)
        || reg == ES9356_FLAGS_HP
}

static es9356_sdca_regmap: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 16,
    volatile_reg: Some(es9356_sdca_volatile_register),
    max_register: 0x45ffffff,
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
};

unsafe fn es9356_register_init(es9356: *mut es9356_sdw_priv) {
    let writes: &[(c_uint, c_uint)] = &[
        (ES9356_STATE, 0x02), (ES9356_ENDPOINT_MODE, 0x24), (ES9356_PRE_DIV_CTL, 0x00),
        (ES9356_ADC_OSR, 0x18), (ES9356_ADC_OSRGAIN, 0x13), (ES9356_DAC_OSR, 0x16),
        (ES9356_CLK_CTL, 0x0f), (ES9356_CSM_RESET, 0x01), (ES9356_CLK_SEL, 0x30),
        (ES9356_DETCLK_CTL, 0x51), (ES9356_HP_TYPE, 0x10), (ES9356_MICBIAS_CTL, 0x10),
        (ES9356_HPDETECT_CTL, 0x07), (ES9356_ADC_ANA, 0x30), (ES9356_PGA_CTL, 0xa8),
        (ES9356_ADC_INT, 0xaa), (ES9356_ADC_LP, 0x19), (ES9356_VMID1SEL, 0xbc),
        (ES9356_VMID_TIME, 0x0b), (ES9356_STATE_TIME, 0xbb), (ES9356_HP_SPK_TIME, 0x77),
        (ES9356_HP_DETECTTIME, 0xa4), (ES9356_MICBIAS_SEL, 0x15), (ES9356_KEY_PRESS_TIME, 0xff),
        (ES9356_KEY_RELEASE_TIME, 0xff), (ES9356_KEY_HOLD_TIME, 0x0f), (ES9356_BTSEL_REF, 0x00),
        (ES9356_KEYD_DETECT, 0x18), (ES9356_MICBIAS_RES, 0x03), (ES9356_BUTTON_CHARGE, 0x00),
        (ES9356_CALIBRATION_TIME, 0x13), (ES9356_CALIBRATION_SETTING, 0xf4), (ES9356_SPK_VOLUME, 0x33),
        (ES9356_DAC_VROI, 0x01), (ES9356_DAC_LP, 0x00), (ES9356_HP_IBIAS, 0x04),
        (ES9356_HP_LP, 0x03), (ES9356_SPKLDO_CTL, 0x65), (ES9356_SPKBIAS_COMP, 0x09),
        (ES9356_VMID1STL, 0x00), (ES9356_VMID2STL, 0x00), (ES9356_VSEL, 0xfc),
        (ES9356_IBIASGEN, 0x10), (ES9356_ADC_AMIC_CTL, 0x0d), (ES9356_STATE, 0x0e),
        (ES9356_CSM_RESET, 0x00), (ES9356_HP_TYPE, 0x08),
    ];
    for &(reg, val) in writes {
        regmap_write((*es9356).regmap, reg, val);
    }

    regmap_write((*es9356).regmap, SDW_SDCA_CTL(FUNC_NUM_MIC, ES9356_SDCA_ENT_FU113, ES9356_SDCA_CTL_FU_VOLUME, CH_L), ES9356_DEFAULT_VOLUME);
    regmap_write((*es9356).regmap, SDW_SDCA_CTL(FUNC_NUM_MIC, ES9356_SDCA_ENT_FU113, ES9356_SDCA_CTL_FU_VOLUME, CH_R), ES9356_DEFAULT_VOLUME);
    regmap_write((*es9356).regmap, SDW_SDCA_CTL(FUNC_NUM_AMP, ES9356_SDCA_ENT_FU21, ES9356_SDCA_CTL_FU_VOLUME, CH_L), ES9356_DEFAULT_VOLUME);
    regmap_write((*es9356).regmap, SDW_SDCA_CTL(FUNC_NUM_AMP, ES9356_SDCA_ENT_FU21, ES9356_SDCA_CTL_FU_VOLUME, CH_R), ES9356_DEFAULT_VOLUME);
    regmap_write((*es9356).regmap, SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_FU41, ES9356_SDCA_CTL_FU_VOLUME, CH_L), ES9356_DEFAULT_VOLUME);
    regmap_write((*es9356).regmap, SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_FU41, ES9356_SDCA_CTL_FU_VOLUME, CH_R), ES9356_DEFAULT_VOLUME);
    regmap_write((*es9356).regmap, SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_FU36, ES9356_SDCA_CTL_FU_VOLUME, CH_L), ES9356_DEFAULT_VOLUME);
    regmap_write((*es9356).regmap, SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_FU36, ES9356_SDCA_CTL_FU_VOLUME, CH_R), ES9356_DEFAULT_VOLUME);
}

unsafe fn es9356_sdca_io_init(_dev: *mut device, slave: *mut sdw_slave) -> c_int {
    let es9356 = dev_get_drvdata(&mut (*slave).dev) as *mut es9356_sdw_priv;

    if (*es9356).hw_init {
        return 0;
    }

    (*es9356).disable_irq = false;

    regcache_cache_only((*es9356).regmap, false);

    if (*es9356).first_hw_init {
        regcache_cache_bypass((*es9356).regmap, true);
    } else {
        /* update count of parent 'active' children */
        pm_runtime_set_active(&mut (*slave).dev);

        es9356_register_init(es9356);
    }
    pm_runtime_get_noresume(&mut (*slave).dev);

    regmap_write((*es9356).regmap, SDW_SDCA_CTL(FUNC_NUM_MIC, ES9356_SDCA_ENT_XU12, ES9356_SDCA_CTL_SELECTED_MODE, 0), 0x01);
    regmap_write((*es9356).regmap, SDW_SDCA_CTL(FUNC_NUM_MIC, ES9356_SDCA_ENT0, ES9356_SDCA_CTL_FUNC_STATUS, 0), 0x40);
    regmap_write((*es9356).regmap, SDW_SDCA_CTL(FUNC_NUM_AMP, ES9356_SDCA_ENT_XU22, ES9356_SDCA_CTL_SELECTED_MODE, 0), 0x01);
    regmap_write((*es9356).regmap, SDW_SDCA_CTL(FUNC_NUM_AMP, ES9356_SDCA_ENT0, ES9356_SDCA_CTL_FUNC_STATUS, 0), 0x40);
    regmap_write((*es9356).regmap, SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_XU42, ES9356_SDCA_CTL_SELECTED_MODE, 0), 0x01);
    regmap_write((*es9356).regmap, SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT_XU36, ES9356_SDCA_CTL_SELECTED_MODE, 0), 0x01);
    regmap_write((*es9356).regmap, SDW_SDCA_CTL(FUNC_NUM_UAJ, ES9356_SDCA_ENT0, ES9356_SDCA_CTL_FUNC_STATUS, 0), 0x40);

    if (*es9356).first_hw_init {
        regcache_cache_bypass((*es9356).regmap, false);
        regcache_mark_dirty((*es9356).regmap);
    } else {
        (*es9356).first_hw_init = true;
    }

    (*es9356).hw_init = true;

    pm_runtime_mark_last_busy(&mut (*slave).dev);
    pm_runtime_put_autosuspend(&mut (*slave).dev);

    0
}

unsafe extern "C" fn es9356_sdw_update_status(slave: *mut sdw_slave, status: sdw_slave_status) -> c_int {
    let es9356 = dev_get_drvdata(&mut (*slave).dev) as *mut es9356_sdw_priv;

    if status == SDW_SLAVE_UNATTACHED {
        (*es9356).hw_init = false;
        cancel_delayed_work_sync(&mut (*es9356).interrupt_handle_work);
        cancel_delayed_work_sync(&mut (*es9356).button_detect_work);
        regcache_cache_only((*es9356).regmap, true);
    }

    if status == SDW_SLAVE_ATTACHED {
        if !(*es9356).hs_jack.is_null() {
            sdw_write_no_pm((*es9356).slave, SDW_SCP_SDCA_INTMASK1, SDW_SCP_SDCA_INTMASK_SDCA_7 | SDW_SCP_SDCA_INTMASK_SDCA_5 | SDW_SCP_SDCA_INTMASK_SDCA_1);
        }
    }

    if (*es9356).hw_init || status != SDW_SLAVE_ATTACHED {
        return 0;
    }

    es9356_sdca_io_init(&mut (*slave).dev, slave)
}

unsafe extern "C" fn es9356_sdw_read_prop(slave: *mut sdw_slave) -> c_int {
    let prop = &mut (*slave).prop;
    let mut nval: c_int;
    let mut i: c_int;
    let mut j: c_int;
    let mut bit: u32;
    let mut addr: c_ulong;
    let mut dpn: *mut sdw_dpn_prop;

    prop.paging_support = true;

    /*
     * first we need to allocate memory for set bits in port lists
     * the port allocation is completely arbitrary:
     * DP0 is not supported
     * DP3 and DP4 is sink
     * DP1 and DP2 is source
     */
    prop.source_ports = BIT(1) | BIT(2);
    prop.sink_ports = BIT(3) | BIT(4);

    nval = hweight32(prop.source_ports);
    prop.src_dpn_prop = devm_kcalloc(&mut (*slave).dev, nval, size_of::<sdw_dpn_prop>(), GFP_KERNEL) as *mut sdw_dpn_prop;
    if prop.src_dpn_prop.is_null() {
        return -ENOMEM;
    }

    i = 0;
    dpn = prop.src_dpn_prop;
    addr = prop.source_ports as c_ulong;
    bit = 0;
    while bit < 32 {
        if addr & (1usize.wrapping_shl(bit) as c_ulong) != 0 {
            (*dpn.add(i as usize)).num = bit;
            (*dpn.add(i as usize)).type_ = SDW_DPN_FULL;
            (*dpn.add(i as usize)).simple_ch_prep_sm = true;
            i += 1;
        }
        bit += 1;
    }

    /* do this again for sink now */
    nval = hweight32(prop.sink_ports);
    prop.sink_dpn_prop = devm_kcalloc(&mut (*slave).dev, nval, size_of::<sdw_dpn_prop>(), GFP_KERNEL) as *mut sdw_dpn_prop;
    if prop.sink_dpn_prop.is_null() {
        return -ENOMEM;
    }

    j = 0;
    dpn = prop.sink_dpn_prop;
    addr = prop.sink_ports as c_ulong;
    bit = 0;
    while bit < 32 {
        if addr & (1usize.wrapping_shl(bit) as c_ulong) != 0 {
            (*dpn.add(j as usize)).num = bit;
            (*dpn.add(j as usize)).type_ = SDW_DPN_FULL;
            (*dpn.add(j as usize)).simple_ch_prep_sm = true;
            j += 1;
        }
        bit += 1;
    }

    /* wake-up event */
    prop.wake_capable = 1;

    0
}

unsafe extern "C" fn es9356_sdw_interrupt_callback(
    slave: *mut sdw_slave,
    status: *mut sdw_slave_intr_status,
) -> c_int {
    let es9356 = dev_get_drvdata(&mut (*slave).dev) as *mut es9356_sdw_priv;
    let mut sdca_cascade: c_uint;
    let mut scp_sdca_stat1: c_uint = 0;
    let mut count: c_int = 0;
    let retry: c_int = 3;
    let mut ret: c_int;
    let mut stat: c_int;
    let mut reg: c_int;

    mutex_lock(&mut (*es9356).disable_irq_lock);

    ret = sdw_read_no_pm((*es9356).slave, SDW_SCP_SDCA_INT1);
    if ret < 0 {
        mutex_unlock(&mut (*es9356).disable_irq_lock);
        pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("es9356_sdw_interrupt_callback"), ret);
        return ret;
    }
    (*es9356).sdca_status = ret as c_uint;

    loop {
        reg = sdw_read_no_pm((*es9356).slave, SDW_SCP_SDCA_INT1);
        if reg < 0 {
            ret = reg;
            mutex_unlock(&mut (*es9356).disable_irq_lock);
            pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("es9356_sdw_interrupt_callback"), ret);
            return ret;
        }
        if reg as c_uint & SDW_SCP_SDCA_INTMASK_SDCA_1 != 0 {
            ret = sdw_update_no_pm((*es9356).slave, SDW_SCP_SDCA_INT1, SDW_SCP_SDCA_INT_SDCA_1, SDW_SCP_SDCA_INT_SDCA_1);
            if ret < 0 {
                mutex_unlock(&mut (*es9356).disable_irq_lock);
                pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("es9356_sdw_interrupt_callback"), ret);
                return ret;
            }
        }

        if reg as c_uint & SDW_SCP_SDCA_INTMASK_SDCA_5 != 0 {
            ret = sdw_update_no_pm((*es9356).slave, SDW_SCP_SDCA_INT1, SDW_SCP_SDCA_INT_SDCA_5, SDW_SCP_SDCA_INT_SDCA_5);
            if ret < 0 {
                mutex_unlock(&mut (*es9356).disable_irq_lock);
                pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("es9356_sdw_interrupt_callback"), ret);
                return ret;
            }
        }

        if reg as c_uint & SDW_SCP_SDCA_INTMASK_SDCA_7 != 0 {
            ret = sdw_update_no_pm((*es9356).slave, SDW_SCP_SDCA_INT1, SDW_SCP_SDCA_INT_SDCA_7, SDW_SCP_SDCA_INT_SDCA_7);
            if ret < 0 {
                mutex_unlock(&mut (*es9356).disable_irq_lock);
                pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("es9356_sdw_interrupt_callback"), ret);
                return ret;
            }
        }

        ret = sdw_read_no_pm((*es9356).slave, SDW_DP0_INT);
        if ret < 0 {
            mutex_unlock(&mut (*es9356).disable_irq_lock);
            pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("es9356_sdw_interrupt_callback"), ret);
            return ret;
        }
        sdca_cascade = ret as c_uint & SDW_DP0_SDCA_CASCADE;

        ret = sdw_read_no_pm((*es9356).slave, SDW_SCP_SDCA_INT1);
        if ret < 0 {
            mutex_unlock(&mut (*es9356).disable_irq_lock);
            pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("es9356_sdw_interrupt_callback"), ret);
            return ret;
        }
        scp_sdca_stat1 = ret as c_uint & (SDW_SCP_SDCA_INTMASK_SDCA_1 | SDW_SCP_SDCA_INTMASK_SDCA_5 | SDW_SCP_SDCA_INTMASK_SDCA_7);

        stat = ((scp_sdca_stat1 != 0) || (sdca_cascade != 0)) as c_int;

        count += 1;
        if !(stat != 0 && count < retry) {
            break;
        }
    }

    /* The 280 ms figure was determined through testing */
    if (*status).sdca_cascade && !(*es9356).disable_irq {
        mod_delayed_work(system_power_efficient_wq, &mut (*es9356).interrupt_handle_work, msecs_to_jiffies(280));
    }

    mutex_unlock(&mut (*es9356).disable_irq_lock);
    0
}

static es9356_sdw_slave_ops: sdw_slave_ops = sdw_slave_ops {
    read_prop: Some(es9356_sdw_read_prop),
    interrupt_callback: Some(es9356_sdw_interrupt_callback),
    update_status: Some(es9356_sdw_update_status),
};

unsafe fn es9356_sdca_init(dev: *mut device, regmap: *mut regmap, slave: *mut sdw_slave) -> c_int {
    let es9356: *mut es9356_sdw_priv;
    let mut ret: c_int;

    es9356 = devm_kzalloc(dev, size_of::<es9356_sdw_priv>(), GFP_KERNEL) as *mut es9356_sdw_priv;
    if es9356.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, es9356 as *mut c_void);

    (*es9356).slave = slave;
    (*es9356).regmap = regmap;
    mutex_init(&mut (*es9356).disable_irq_lock);
    mutex_init(&mut (*es9356).pde_lock);

    regcache_cache_only((*es9356).regmap, true);

    (*es9356).hw_init = false;
    (*es9356).first_hw_init = false;

    INIT_DELAYED_WORK(&mut (*es9356).interrupt_handle_work, es9356_interrupt_handler);
    INIT_DELAYED_WORK(&mut (*es9356).button_detect_work, es9356_button_detect_handler);

    ret = devm_snd_soc_register_component(
        dev,
        &snd_soc_es9356_sdw_component,
        es9356_sdw_dai.as_mut_ptr(),
        ARRAY_SIZE!(es9356_sdw_dai) as c_int,
    );
    if ret != 0 {
        dev_err_probe(dev, ret, cstr!("Failed to register component\n"));
        return ret;
    }
    /* set autosuspend parameters */
    pm_runtime_set_autosuspend_delay(dev, 3000);
    pm_runtime_use_autosuspend(dev);
    /* make sure the device does not suspend immediately */
    pm_runtime_mark_last_busy(dev);
    pm_runtime_enable(dev);

    0
}

unsafe extern "C" fn es9356_sdw_probe(slave: *mut sdw_slave, _id: *const sdw_device_id) -> c_int {
    let regmap: *mut regmap;

    regmap = devm_regmap_init_sdw_mbq_cfg(&mut (*slave).dev, slave, &es9356_sdca_regmap, &es9356_mbq_config);
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    es9356_sdca_init(&mut (*slave).dev, regmap, slave)
}

unsafe extern "C" fn es9356_sdw_remove(slave: *mut sdw_slave) {
    let es9356 = dev_get_drvdata(&mut (*slave).dev) as *mut es9356_sdw_priv;

    if (*es9356).hw_init {
        cancel_delayed_work_sync(&mut (*es9356).interrupt_handle_work);
        cancel_delayed_work_sync(&mut (*es9356).button_detect_work);
    }

    if (*es9356).first_hw_init {
        pm_runtime_disable(&mut (*slave).dev);
    }

    mutex_destroy(&mut (*es9356).disable_irq_lock);
    mutex_destroy(&mut (*es9356).pde_lock);
}

static es9356_sdw_id: [sdw_device_id; 3] = [
    sdw_device_id { _private: [] },
    sdw_device_id { _private: [] },
    sdw_device_id { _private: [] },
];
// MODULE_DEVICE_TABLE(sdw, es9356_sdw_id);

unsafe extern "C" fn es9356_sdca_dev_suspend(dev: *mut device) -> c_int {
    let es9356 = dev_get_drvdata(dev) as *mut es9356_sdw_priv;

    cancel_delayed_work_sync(&mut (*es9356).interrupt_handle_work);
    cancel_delayed_work_sync(&mut (*es9356).button_detect_work);

    regcache_cache_only((*es9356).regmap, true);

    0
}

unsafe extern "C" fn es9356_sdca_dev_system_suspend(dev: *mut device) -> c_int {
    let es9356 = dev_get_drvdata(dev) as *mut es9356_sdw_priv;

    mutex_lock(&mut (*es9356).disable_irq_lock);
    (*es9356).disable_irq = true;
    mutex_unlock(&mut (*es9356).disable_irq_lock);

    es9356_sdca_dev_suspend(dev)
}

const es9356_PROBE_TIMEOUT: c_int = 2000;

unsafe extern "C" fn es9356_sdca_dev_resume(dev: *mut device) -> c_int {
    let slave = dev_to_sdw_dev(dev);
    let es9356 = dev_get_drvdata(dev) as *mut es9356_sdw_priv;
    let mut ret: c_int;

    if !(*slave).unattach_request {
        (*es9356).disable_irq = false;
    }

    ret = sdw_slave_wait_for_init(slave, es9356_PROBE_TIMEOUT);
    if ret != 0 {
        sdw_show_ping_status((*slave).bus, true);
        return ret;
    }

    regcache_cache_only((*es9356).regmap, false);
    regcache_sync((*es9356).regmap);
    0
}

static es9356_sdca_pm: dev_pm_ops = dev_pm_ops { _private: [] };

static mut es9356_sdw_driver: sdw_driver = sdw_driver {
    driver: device_driver {
        name: cstr!("es9356"),
        pm: unsafe { pm_ptr(&es9356_sdca_pm) },
    },
    probe: Some(es9356_sdw_probe),
    remove: Some(es9356_sdw_remove),
    ops: &es9356_sdw_slave_ops,
    id_table: es9356_sdw_id.as_ptr(),
};

// module_sdw_driver(es9356_sdw_driver);
// MODULE_IMPORT_NS("SND_SOC_SDCA");
// MODULE_DESCRIPTION("ASoC ES9356 SDCA SDW codec driver");
// MODULE_AUTHOR("Michael Zhang <zhangyi@everest-semi.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
