// SPDX-License-Identifier: GPL-2.0-only
//
// es8326.c -- es8326 ALSA SoC audio driver
// Copyright Everest Semiconductor Co., Ltd
//
// Authors: David Yang <yangxiaohua@everest-semi.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type u8 = u8;
type u16 = u16;
type u32 = u32;
type bool_ = bool;
type irqreturn_t = c_uint;

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct i2c_client {
    pub dev: device,
    pub irq: c_int,
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct delayed_work {
    pub work: work_struct,
}
#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_jack {
    pub status: c_int,
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
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
    pub value: [c_long; 128],
}
type c_long = isize;
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct soc_enum {
    _private: [u8; 0],
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
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub use_single_read: bool,
    pub use_single_write: bool,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
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
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub no_capture_mute: c_uint,
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
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub set_jack: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_jack, *mut c_void) -> c_int>,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}
#[repr(C)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON,
    SND_SOC_BIAS_PREPARE,
    SND_SOC_BIAS_STANDBY,
    SND_SOC_BIAS_OFF,
}
#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
    pub driver_data: usize,
}
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}
#[repr(C)]
pub struct acpi_device_id {
    pub id: [c_char; 16],
    pub driver_data: usize,
}
#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub acpi_match_table: *const acpi_device_id,
    pub of_match_table: *const of_device_id,
}
#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub id_table: *const i2c_device_id,
}

#[repr(C)]
pub struct es8326_priv {
    pub mclk: *mut clk,
    pub i2c: *mut i2c_client,
    pub regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub jack_detect_work: delayed_work,
    pub button_press_work: delayed_work,
    pub jack: *mut snd_soc_jack,
    pub irq: c_int,
    /* The lock protects the situation that an irq is generated
     * while enabling or disabling or during an irq.
     */
    pub lock: mutex,
    pub jack_pol: u8,
    pub interrupt_src: u8,
    pub interrupt_clk: u8,
    pub hpl_vol: u8,
    pub hpr_vol: u8,
    pub jd_inverted: bool,
    pub sysclk: c_uint,
    pub calibrated: bool,
    pub version: c_int,
    pub hp: c_int,
    pub jack_remove_retry: c_int,
}

extern "C" {
    static mut system_dfl_wq: *mut c_void;
    static es8326_acpi_match: [acpi_device_id; 2];
    static es8326_of_match: [of_device_id; 2];

    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut es8326_priv;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_mutex_lock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_mutex_unlock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_force_enable_pin_unlocked(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_disable_pin_unlocked(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_sync_unlocked(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_cache_bypass(map: *mut regmap, enable: bool);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn msleep(msecs: c_uint);
    fn usleep_range(min: c_uint, max: c_uint);
    fn queue_delayed_work(wq: *mut c_void, work: *mut delayed_work, delay: c_uint) -> bool;
    fn msecs_to_jiffies(msecs: c_uint) -> c_uint;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn device_property_read_bool(dev: *mut device, propname: *const c_char) -> bool;
    fn device_property_read_u8(dev: *mut device, propname: *const c_char, val: *mut u8) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut es8326_priv;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_request_threaded_irq(dev: *mut device, irq: c_int, handler: *const c_void, thread_fn: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_uint, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn devm_clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}
macro_rules! ARRAY_SIZE {
    ($a:expr) => {
        ($a.len() as c_uint)
    };
}
macro_rules! SOC_ENUM_SINGLE { ($($tt:tt)*) => { soc_enum { _private: [] } }; }
macro_rules! SOC_SINGLE_TLV { ($($tt:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_ENUM { ($($tt:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_DOUBLE_R_TLV { ($($tt:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_DOUBLE_TLV { ($($tt:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_SINGLE { ($($tt:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_SINGLE_EXT { ($($tt:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SND_SOC_DAPM_INPUT { ($($tt:tt)*) => { snd_soc_dapm_widget { _private: [] } }; }
macro_rules! SND_SOC_DAPM_ADC { ($($tt:tt)*) => { snd_soc_dapm_widget { _private: [] } }; }
macro_rules! SND_SOC_DAPM_AIF_OUT { ($($tt:tt)*) => { snd_soc_dapm_widget { _private: [] } }; }
macro_rules! SND_SOC_DAPM_AIF_IN { ($($tt:tt)*) => { snd_soc_dapm_widget { _private: [] } }; }
macro_rules! SND_SOC_DAPM_DAC { ($($tt:tt)*) => { snd_soc_dapm_widget { _private: [] } }; }
macro_rules! SND_SOC_DAPM_SUPPLY { ($($tt:tt)*) => { snd_soc_dapm_widget { _private: [] } }; }
macro_rules! SND_SOC_DAPM_PGA { ($($tt:tt)*) => { snd_soc_dapm_widget { _private: [] } }; }
macro_rules! SND_SOC_DAPM_OUTPUT { ($($tt:tt)*) => { snd_soc_dapm_widget { _private: [] } }; }
macro_rules! MODULE_DEVICE_TABLE { ($($tt:tt)*) => {}; }
macro_rules! module_i2c_driver { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }

include!("es8326.h.rs"); // dependency intent from "es8326.h"; supplied by the surrounding repository.

unsafe extern "C" fn es8326_crosstalk1_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let es8326 = snd_soc_component_get_drvdata(component);
    let mut crosstalk_h: c_uint = 0;
    let mut crosstalk_l: c_uint = 0;
    let crosstalk: c_uint;

    regmap_read((*es8326).regmap, ES8326_DAC_RAMPRATE, &mut crosstalk_h);
    regmap_read((*es8326).regmap, ES8326_DAC_CROSSTALK, &mut crosstalk_l);
    crosstalk_h &= 0x20;
    crosstalk_l &= 0xf0;
    crosstalk = crosstalk_h >> 1 | crosstalk_l >> 4;
    (*ucontrol).value.integer.value[0] = crosstalk as c_long;

    0
}

unsafe extern "C" fn es8326_crosstalk1_set(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let es8326 = snd_soc_component_get_drvdata(component);
    let mut crosstalk_l: c_uint = 0;
    let crosstalk_h: c_uint;
    let crosstalk: c_uint;

    crosstalk = (*ucontrol).value.integer.value[0] as c_uint;
    regmap_read((*es8326).regmap, ES8326_DAC_CROSSTALK, &mut crosstalk_l);
    crosstalk_h = (crosstalk & 0x10) << 1;
    crosstalk_l &= 0x0f;
    crosstalk_l |= (crosstalk & 0x0f) << 4;
    regmap_update_bits((*es8326).regmap, ES8326_DAC_RAMPRATE, 0x20, crosstalk_h);
    regmap_write((*es8326).regmap, ES8326_DAC_CROSSTALK, crosstalk_l);

    0
}

unsafe extern "C" fn es8326_crosstalk2_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let es8326 = snd_soc_component_get_drvdata(component);
    let mut crosstalk_h: c_uint = 0;
    let mut crosstalk_l: c_uint = 0;
    let crosstalk: c_uint;

    regmap_read((*es8326).regmap, ES8326_DAC_RAMPRATE, &mut crosstalk_h);
    regmap_read((*es8326).regmap, ES8326_DAC_CROSSTALK, &mut crosstalk_l);
    crosstalk_h &= 0x10;
    crosstalk_l &= 0x0f;
    crosstalk = crosstalk_h | crosstalk_l;
    (*ucontrol).value.integer.value[0] = crosstalk as c_long;

    0
}

unsafe extern "C" fn es8326_crosstalk2_set(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let es8326 = snd_soc_component_get_drvdata(component);
    let crosstalk_h: c_uint;
    let mut crosstalk_l: c_uint = 0;
    let crosstalk: c_uint;

    crosstalk = (*ucontrol).value.integer.value[0] as c_uint;
    regmap_read((*es8326).regmap, ES8326_DAC_CROSSTALK, &mut crosstalk_l);
    crosstalk_h = crosstalk & 0x10;
    crosstalk_l &= 0xf0;
    crosstalk_l |= crosstalk & 0x0f;
    regmap_update_bits((*es8326).regmap, ES8326_DAC_RAMPRATE, 0x10, crosstalk_h);
    regmap_write((*es8326).regmap, ES8326_DAC_CROSSTALK, crosstalk_l);

    0
}

unsafe extern "C" fn es8326_hplvol_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let es8326 = snd_soc_component_get_drvdata(component);

    (*ucontrol).value.integer.value[0] = (*es8326).hpl_vol as c_long;

    0
}

unsafe extern "C" fn es8326_hplvol_set(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let es8326 = snd_soc_component_get_drvdata(component);
    let mut hp_vol: c_uint;

    hp_vol = (*ucontrol).value.integer.value[0] as c_uint;
    if hp_vol > 5 {
        return -EINVAL;
    }
    if (*es8326).hpl_vol as c_uint != hp_vol {
        (*es8326).hpl_vol = hp_vol as u8;
        if hp_vol >= 3 {
            hp_vol += 1;
        }
        regmap_update_bits((*es8326).regmap, ES8326_HP_VOL, 0x70, hp_vol << 4);
        return 1;
    }

    0
}

unsafe extern "C" fn es8326_hprvol_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let es8326 = snd_soc_component_get_drvdata(component);

    (*ucontrol).value.integer.value[0] = (*es8326).hpr_vol as c_long;

    0
}

unsafe extern "C" fn es8326_hprvol_set(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let es8326 = snd_soc_component_get_drvdata(component);
    let mut hp_vol: c_uint;

    hp_vol = (*ucontrol).value.integer.value[0] as c_uint;
    if hp_vol > 5 {
        return -EINVAL;
    }
    if (*es8326).hpr_vol as c_uint != hp_vol {
        (*es8326).hpr_vol = hp_vol as u8;
        if hp_vol >= 3 {
            hp_vol += 1;
        }
        regmap_update_bits((*es8326).regmap, ES8326_HP_VOL, 0x07, hp_vol);
        return 1;
    }

    0
}

static dac_vol_tlv: [c_uint; 4] = [SNDRV_CTL_TLVT_DB_SCALE, 2, -9550i32 as c_uint, 50];
static adc_vol_tlv: [c_uint; 4] = [SNDRV_CTL_TLVT_DB_SCALE, 2, -9550i32 as c_uint, 50];
static adc_analog_pga_tlv: [c_uint; 4] = [SNDRV_CTL_TLVT_DB_SCALE, 2, 0, 300];
static adc_pga_tlv: [c_uint; 4] = [SNDRV_CTL_TLVT_DB_SCALE, 2, 0, 600];
static softramp_rate: [c_uint; 4] = [SNDRV_CTL_TLVT_DB_SCALE, 2, 0, 100];
static drc_target_tlv: [c_uint; 4] = [SNDRV_CTL_TLVT_DB_SCALE, 2, -3200i32 as c_uint, 200];
static drc_recovery_tlv: [c_uint; 4] = [SNDRV_CTL_TLVT_DB_SCALE, 2, -125i32 as c_uint, 250];
static dre_gain_tlv: [c_uint; 4] = [SNDRV_CTL_TLVT_DB_SCALE, 2, -9550i32 as c_uint, 400];
static dre_gate_tlv: [c_uint; 4] = [SNDRV_CTL_TLVT_DB_SCALE, 2, -9600i32 as c_uint, 600];

static winsize: [*const c_char; 16] = [
    cstr!("0.25db/2  LRCK"), cstr!("0.25db/4  LRCK"), cstr!("0.25db/8  LRCK"), cstr!("0.25db/16  LRCK"),
    cstr!("0.25db/32  LRCK"), cstr!("0.25db/64  LRCK"), cstr!("0.25db/128  LRCK"), cstr!("0.25db/256  LRCK"),
    cstr!("0.25db/512  LRCK"), cstr!("0.25db/1024  LRCK"), cstr!("0.25db/2048  LRCK"), cstr!("0.25db/4096  LRCK"),
    cstr!("0.25db/8192  LRCK"), cstr!("0.25db/16384  LRCK"), cstr!("0.25db/32768  LRCK"), cstr!("0.25db/65536  LRCK"),
];

static dacpol_txt: [*const c_char; 4] = [cstr!("Normal"), cstr!("R Invert"), cstr!("L Invert"), cstr!("L + R Invert")];

static hp_spkvol_switch: [*const c_char; 4] = [
    cstr!("HPVOL: HPL+HPL, SPKVOL: HPL+HPL"),
    cstr!("HPVOL: HPL+HPR, SPKVOL: HPL+HPR"),
    cstr!("HPVOL: HPL+HPL, SPKVOL: SPKL+SPKR"),
    cstr!("HPVOL: HPL+HPR, SPKVOL: SPKL+SPKR"),
];

static dacpol: soc_enum = SOC_ENUM_SINGLE!(ES8326_DAC_DSM, 4, 4, dacpol_txt);
static dre_winsize: soc_enum = SOC_ENUM_SINGLE!(ES8326_ADC_DRE, 0, 16, winsize);
static alc_winsize: soc_enum = SOC_ENUM_SINGLE!(ES8326_ADC_RAMPRATE, 4, 16, winsize);
static drc_winsize: soc_enum = SOC_ENUM_SINGLE!(ES8326_DRC_WINSIZE, 4, 16, winsize);
static hpvol_spkvol_switch: soc_enum = SOC_ENUM_SINGLE!(ES8326_HP_MISC, 6, 4, hp_spkvol_switch);

static es8326_snd_controls: [snd_kcontrol_new; 35] = [
    SOC_SINGLE_TLV!("DAC Playback Volume", ES8326_DACL_VOL, 0, 0xbf, 0, dac_vol_tlv),
    SOC_ENUM!("Playback Polarity", dacpol),
    SOC_SINGLE_TLV!("DAC Ramp Rate", ES8326_DAC_RAMPRATE, 0, 0x0f, 0, softramp_rate),
    SOC_SINGLE_TLV!("DRC Recovery Level", ES8326_DRC_RECOVERY, 0, 4, 0, drc_recovery_tlv),
    SOC_ENUM!("DRC Winsize", drc_winsize),
    SOC_SINGLE_TLV!("DRC Target Level", ES8326_DRC_WINSIZE, 0, 0x0f, 0, drc_target_tlv),
    SOC_DOUBLE_R_TLV!("ADC Capture Volume", ES8326_ADC1_VOL, ES8326_ADC2_VOL, 0, 0xff, 0, adc_vol_tlv),
    SOC_DOUBLE_TLV!("ADC PGA Volume", ES8326_ADC_SCALE, 4, 0, 5, 0, adc_pga_tlv),
    SOC_SINGLE_TLV!("ADC PGA Gain Volume", ES8326_PGAGAIN, 0, 10, 0, adc_analog_pga_tlv),
    SOC_SINGLE!("ADC PGA SE Switch", ES8326_PGAGAIN, 7, 1, 0),
    SOC_SINGLE_TLV!("ADC Ramp Rate", ES8326_ADC_RAMPRATE, 0, 0x0f, 0, softramp_rate),
    SOC_SINGLE!("ADC4 DRE Switch", ES8326_ADC_DRE, 4, 1, 0),
    SOC_SINGLE!("ADC3 DRE Switch", ES8326_ADC_DRE, 5, 1, 0),
    SOC_SINGLE!("ADC2 DRE Switch", ES8326_ADC_DRE, 6, 1, 0),
    SOC_SINGLE!("ADC1 DRE Switch", ES8326_ADC_DRE, 7, 1, 0),
    SOC_ENUM!("DRE Winsize", dre_winsize),
    SOC_SINGLE!("DRE Gain Switch", ES8326_ADC_DRE_GAIN, 5, 1, 0),
    SOC_SINGLE_TLV!("DRE Gain Volume", ES8326_ADC_DRE_GAIN, 0, 0x1F, 0, dre_gain_tlv),
    SOC_SINGLE_TLV!("DRE Gate Volume", ES8326_ADC_DRE_GATE, 4, 0x07, 0, dre_gate_tlv),
    SOC_SINGLE!("ALC Capture Switch", ES8326_ALC_RECOVERY, 3, 1, 0),
    SOC_SINGLE_TLV!("ALC Capture Recovery Level", ES8326_ALC_LEVEL, 0, 4, 0, drc_recovery_tlv),
    SOC_ENUM!("ALC Capture Winsize", alc_winsize),
    SOC_SINGLE_TLV!("ALC Capture Target Level", ES8326_ALC_LEVEL, 0, 0x0f, 0, drc_target_tlv),
    SOC_SINGLE_EXT!("CROSSTALK1", SND_SOC_NOPM, 0, 31, 0, es8326_crosstalk1_get, es8326_crosstalk1_set),
    SOC_SINGLE_EXT!("CROSSTALK2", SND_SOC_NOPM, 0, 31, 0, es8326_crosstalk2_get, es8326_crosstalk2_set),
    SOC_SINGLE_EXT!("HPL Volume", SND_SOC_NOPM, 0, 5, 0, es8326_hplvol_get, es8326_hplvol_set),
    SOC_SINGLE_EXT!("HPR Volume", SND_SOC_NOPM, 0, 5, 0, es8326_hprvol_get, es8326_hprvol_set),
    SOC_SINGLE_TLV!("HPL Playback Volume", ES8326_DACL_VOL, 0, 0xbf, 0, dac_vol_tlv),
    SOC_SINGLE_TLV!("HPR Playback Volume", ES8326_DACR_VOL, 0, 0xbf, 0, dac_vol_tlv),
    SOC_SINGLE_TLV!("SPKL Playback Volume", ES8326_SPKL_VOL, 0, 0xbf, 0, dac_vol_tlv),
    SOC_SINGLE_TLV!("SPKR Playback Volume", ES8326_SPKR_VOL, 0, 0xbf, 0, dac_vol_tlv),
    SOC_ENUM!("HPVol SPKVol Switch", hpvol_spkvol_switch),
    SOC_SINGLE!("__padding0", 0, 0, 0, 0),
    SOC_SINGLE!("__padding1", 0, 0, 0, 0),
    SOC_SINGLE!("__padding2", 0, 0, 0, 0),
];

static es8326_dapm_widgets: [snd_soc_dapm_widget; 14] = [
    SND_SOC_DAPM_INPUT!("MIC1"),
    SND_SOC_DAPM_INPUT!("MIC2"),
    SND_SOC_DAPM_INPUT!("MIC3"),
    SND_SOC_DAPM_INPUT!("MIC4"),
    SND_SOC_DAPM_ADC!("ADC L", core::ptr::null(), SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_ADC!("ADC R", core::ptr::null(), SND_SOC_NOPM, 0, 0),
    /* Digital Interface */
    SND_SOC_DAPM_AIF_OUT!("I2S OUT", "I2S1 Capture", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!("I2S IN", "I2S1 Playback", 0, SND_SOC_NOPM, 0, 0),
    /* Analog Power Supply*/
    SND_SOC_DAPM_DAC!("Right DAC", core::ptr::null(), ES8326_ANA_PDN, 0, 1),
    SND_SOC_DAPM_DAC!("Left DAC", core::ptr::null(), ES8326_ANA_PDN, 1, 1),
    SND_SOC_DAPM_SUPPLY!("MICBIAS1", ES8326_ANA_MICBIAS, 2, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("MICBIAS2", ES8326_ANA_MICBIAS, 3, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("LHPMIX", ES8326_DAC2HPMIX, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("RHPMIX", ES8326_DAC2HPMIX, 3, 0, core::ptr::null(), 0),
    /* SND_SOC_DAPM_OUTPUT("HPOL"), SND_SOC_DAPM_OUTPUT("HPOR") preserved by external DAPM macro intent. */
];

static es8326_dapm_routes: [snd_soc_dapm_route; 12] = [
    snd_soc_dapm_route { sink: cstr!("ADC L"), control: core::ptr::null(), source: cstr!("MIC1") },
    snd_soc_dapm_route { sink: cstr!("ADC R"), control: core::ptr::null(), source: cstr!("MIC2") },
    snd_soc_dapm_route { sink: cstr!("ADC L"), control: core::ptr::null(), source: cstr!("MIC3") },
    snd_soc_dapm_route { sink: cstr!("ADC R"), control: core::ptr::null(), source: cstr!("MIC4") },
    snd_soc_dapm_route { sink: cstr!("I2S OUT"), control: core::ptr::null(), source: cstr!("ADC L") },
    snd_soc_dapm_route { sink: cstr!("I2S OUT"), control: core::ptr::null(), source: cstr!("ADC R") },
    snd_soc_dapm_route { sink: cstr!("Right DAC"), control: core::ptr::null(), source: cstr!("I2S IN") },
    snd_soc_dapm_route { sink: cstr!("Left DAC"), control: core::ptr::null(), source: cstr!("I2S IN") },
    snd_soc_dapm_route { sink: cstr!("LHPMIX"), control: core::ptr::null(), source: cstr!("Left DAC") },
    snd_soc_dapm_route { sink: cstr!("RHPMIX"), control: core::ptr::null(), source: cstr!("Right DAC") },
    snd_soc_dapm_route { sink: cstr!("HPOL"), control: core::ptr::null(), source: cstr!("LHPMIX") },
    snd_soc_dapm_route { sink: cstr!("HPOR"), control: core::ptr::null(), source: cstr!("RHPMIX") },
];

unsafe extern "C" fn es8326_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        ES8326_HPL_OFFSET_INI | ES8326_HPR_OFFSET_INI | ES8326_HPDET_STA | ES8326_CTIA_OMTP_STA | ES8326_CSM_MUTE_STA => true,
        _ => false,
    }
}

unsafe extern "C" fn es8326_writeable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        ES8326_BIAS_SW1 | ES8326_BIAS_SW2 | ES8326_BIAS_SW3 | ES8326_BIAS_SW4 | ES8326_ADC_HPFS1 | ES8326_ADC_HPFS2 => false,
        _ => true,
    }
}

static es8326_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: 0xff,
    use_single_read: true,
    use_single_write: true,
    volatile_reg: Some(es8326_volatile_register),
    writeable_reg: Some(es8326_writeable_register),
    cache_type: REGCACHE_RBTREE,
};

#[repr(C)]
#[derive(Copy, Clone)]
struct _coeff_div {
    fs: u16,
    rate: u32,
    mclk: u32,
    reg4: u8,
    reg5: u8,
    reg6: u8,
    reg7: u8,
    reg8: u8,
    reg9: u8,
    rega: u8,
    regb: u8,
}

/* codec hifi mclk clock divider coefficients */
/* {ratio, LRCK, MCLK, REG04, REG05, REG06, REG07, REG08, REG09, REG10, REG11} */
static coeff_div_v0: [_coeff_div; 23] = [
    _coeff_div { fs: 64, rate: 8000, mclk: 512000, reg4: 0x60, reg5: 0x01, reg6: 0x0F, reg7: 0x75, reg8: 0x0A, reg9: 0x1B, rega: 0x1F, regb: 0x7F },
    _coeff_div { fs: 64, rate: 16000, mclk: 1024000, reg4: 0x20, reg5: 0x00, reg6: 0x33, reg7: 0x35, reg8: 0x0A, reg9: 0x1B, rega: 0x1F, regb: 0x3F },
    _coeff_div { fs: 64, rate: 44100, mclk: 2822400, reg4: 0xE0, reg5: 0x00, reg6: 0x03, reg7: 0x2D, reg8: 0x4A, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 64, rate: 48000, mclk: 3072000, reg4: 0xE0, reg5: 0x00, reg6: 0x03, reg7: 0x2D, reg8: 0x4A, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 128, rate: 8000, mclk: 1024000, reg4: 0x60, reg5: 0x00, reg6: 0x33, reg7: 0x35, reg8: 0x0A, reg9: 0x1B, rega: 0x1F, regb: 0x7F },
    _coeff_div { fs: 128, rate: 16000, mclk: 2048000, reg4: 0x20, reg5: 0x00, reg6: 0x03, reg7: 0x35, reg8: 0x0A, reg9: 0x1B, rega: 0x1F, regb: 0x3F },
    _coeff_div { fs: 128, rate: 44100, mclk: 5644800, reg4: 0xE0, reg5: 0x01, reg6: 0x03, reg7: 0x2D, reg8: 0x4A, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 128, rate: 48000, mclk: 6144000, reg4: 0xE0, reg5: 0x01, reg6: 0x03, reg7: 0x2D, reg8: 0x4A, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 192, rate: 32000, mclk: 6144000, reg4: 0xE0, reg5: 0x02, reg6: 0x03, reg7: 0x2D, reg8: 0x4A, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 256, rate: 8000, mclk: 2048000, reg4: 0x60, reg5: 0x00, reg6: 0x03, reg7: 0x35, reg8: 0x0A, reg9: 0x1B, rega: 0x1F, regb: 0x7F },
    _coeff_div { fs: 256, rate: 16000, mclk: 4096000, reg4: 0x20, reg5: 0x01, reg6: 0x03, reg7: 0x35, reg8: 0x0A, reg9: 0x1B, rega: 0x1F, regb: 0x3F },
    _coeff_div { fs: 256, rate: 44100, mclk: 11289600, reg4: 0xE0, reg5: 0x00, reg6: 0x30, reg7: 0x2D, reg8: 0x4A, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 256, rate: 48000, mclk: 12288000, reg4: 0xE0, reg5: 0x00, reg6: 0x30, reg7: 0x2D, reg8: 0x4A, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 384, rate: 32000, mclk: 12288000, reg4: 0xE0, reg5: 0x05, reg6: 0x03, reg7: 0x2D, reg8: 0x4A, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 400, rate: 48000, mclk: 19200000, reg4: 0xE9, reg5: 0x04, reg6: 0x0F, reg7: 0x6d, reg8: 0x4A, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 500, rate: 48000, mclk: 24000000, reg4: 0xF8, reg5: 0x04, reg6: 0x3F, reg7: 0x6D, reg8: 0x4A, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 512, rate: 8000, mclk: 4096000, reg4: 0x60, reg5: 0x01, reg6: 0x03, reg7: 0x35, reg8: 0x0A, reg9: 0x1B, rega: 0x1F, regb: 0x7F },
    _coeff_div { fs: 512, rate: 16000, mclk: 8192000, reg4: 0x20, reg5: 0x00, reg6: 0x30, reg7: 0x35, reg8: 0x0A, reg9: 0x1B, rega: 0x1F, regb: 0x3F },
    _coeff_div { fs: 512, rate: 44100, mclk: 22579200, reg4: 0xE0, reg5: 0x00, reg6: 0x00, reg7: 0x2D, reg8: 0x4A, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 512, rate: 48000, mclk: 24576000, reg4: 0xE0, reg5: 0x00, reg6: 0x00, reg7: 0x2D, reg8: 0x4A, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 768, rate: 32000, mclk: 24576000, reg4: 0xE0, reg5: 0x02, reg6: 0x30, reg7: 0x2D, reg8: 0x4A, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 1024, rate: 8000, mclk: 8192000, reg4: 0x60, reg5: 0x00, reg6: 0x30, reg7: 0x35, reg8: 0x0A, reg9: 0x1B, rega: 0x1F, regb: 0x7F },
    _coeff_div { fs: 1024, rate: 16000, mclk: 16384000, reg4: 0x20, reg5: 0x00, reg6: 0x00, reg7: 0x35, reg8: 0x0A, reg9: 0x1B, rega: 0x1F, regb: 0x3F },
];

static coeff_div_v3: [_coeff_div; 60] = [
    _coeff_div { fs: 32, rate: 8000, mclk: 256000, reg4: 0x60, reg5: 0x00, reg6: 0x0F, reg7: 0x75, reg8: 0x8A, reg9: 0x1B, rega: 0x1F, regb: 0x7F },
    _coeff_div { fs: 32, rate: 16000, mclk: 512000, reg4: 0x20, reg5: 0x00, reg6: 0x0D, reg7: 0x75, reg8: 0x8A, reg9: 0x1B, rega: 0x1F, regb: 0x3F },
    _coeff_div { fs: 32, rate: 44100, mclk: 1411200, reg4: 0x00, reg5: 0x00, reg6: 0x13, reg7: 0x2D, reg8: 0x8A, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 32, rate: 48000, mclk: 1536000, reg4: 0x00, reg5: 0x00, reg6: 0x13, reg7: 0x2D, reg8: 0x8A, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 36, rate: 8000, mclk: 288000, reg4: 0x20, reg5: 0x00, reg6: 0x0D, reg7: 0x75, reg8: 0x8A, reg9: 0x1B, rega: 0x23, regb: 0x47 },
    _coeff_div { fs: 36, rate: 16000, mclk: 576000, reg4: 0x20, reg5: 0x00, reg6: 0x0D, reg7: 0x75, reg8: 0x8A, reg9: 0x1B, rega: 0x23, regb: 0x47 },
    _coeff_div { fs: 48, rate: 8000, mclk: 384000, reg4: 0x60, reg5: 0x02, reg6: 0x1F, reg7: 0x75, reg8: 0x8A, reg9: 0x1B, rega: 0x1F, regb: 0x7F },
    _coeff_div { fs: 48, rate: 16000, mclk: 768000, reg4: 0x20, reg5: 0x02, reg6: 0x0F, reg7: 0x75, reg8: 0x8A, reg9: 0x1B, rega: 0x1F, regb: 0x3F },
    _coeff_div { fs: 48, rate: 48000, mclk: 2304000, reg4: 0x00, reg5: 0x02, reg6: 0x0D, reg7: 0x2D, reg8: 0x8A, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 64, rate: 8000, mclk: 512000, reg4: 0x60, reg5: 0x00, reg6: 0x35, reg7: 0x75, reg8: 0x8A, reg9: 0x1B, rega: 0x1F, regb: 0x7F },
    _coeff_div { fs: 64, rate: 16000, mclk: 1024000, reg4: 0x20, reg5: 0x00, reg6: 0x05, reg7: 0x75, reg8: 0x8A, reg9: 0x1B, rega: 0x1F, regb: 0x3F },
    _coeff_div { fs: 64, rate: 44100, mclk: 2822400, reg4: 0xE0, reg5: 0x00, reg6: 0x31, reg7: 0x2D, reg8: 0xCA, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 64, rate: 48000, mclk: 3072000, reg4: 0xE0, reg5: 0x00, reg6: 0x31, reg7: 0x2D, reg8: 0xCA, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 72, rate: 8000, mclk: 576000, reg4: 0x20, reg5: 0x00, reg6: 0x13, reg7: 0x35, reg8: 0x8A, reg9: 0x1B, rega: 0x23, regb: 0x47 },
    _coeff_div { fs: 72, rate: 16000, mclk: 1152000, reg4: 0x20, reg5: 0x00, reg6: 0x05, reg7: 0x75, reg8: 0x8A, reg9: 0x1B, rega: 0x23, regb: 0x47 },
    _coeff_div { fs: 96, rate: 8000, mclk: 768000, reg4: 0x60, reg5: 0x02, reg6: 0x1D, reg7: 0x75, reg8: 0x8A, reg9: 0x1B, rega: 0x1F, regb: 0x7F },
    _coeff_div { fs: 96, rate: 16000, mclk: 1536000, reg4: 0x20, reg5: 0x02, reg6: 0x0D, reg7: 0x75, reg8: 0x8A, reg9: 0x1B, rega: 0x1F, regb: 0x3F },
    _coeff_div { fs: 100, rate: 48000, mclk: 4800000, reg4: 0x04, reg5: 0x04, reg6: 0x3F, reg7: 0x6D, reg8: 0xB8, reg9: 0x08, rega: 0x4f, regb: 0x1f },
    _coeff_div { fs: 125, rate: 48000, mclk: 6000000, reg4: 0x04, reg5: 0x04, reg6: 0x1F, reg7: 0x2D, reg8: 0x8A, reg9: 0x0A, rega: 0x27, regb: 0x27 },
    _coeff_div { fs: 128, rate: 8000, mclk: 1024000, reg4: 0x60, reg5: 0x00, reg6: 0x05, reg7: 0x75, reg8: 0x8A, reg9: 0x1B, rega: 0x1F, regb: 0x7F },
    _coeff_div { fs: 128, rate: 16000, mclk: 2048000, reg4: 0x20, reg5: 0x00, reg6: 0x31, reg7: 0x35, reg8: 0x08, reg9: 0x19, rega: 0x1F, regb: 0x3F },
    _coeff_div { fs: 128, rate: 44100, mclk: 5644800, reg4: 0xE0, reg5: 0x00, reg6: 0x01, reg7: 0x2D, reg8: 0x48, reg9: 0x08, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 128, rate: 48000, mclk: 6144000, reg4: 0xE0, reg5: 0x00, reg6: 0x01, reg7: 0x2D, reg8: 0x48, reg9: 0x08, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 144, rate: 8000, mclk: 1152000, reg4: 0x20, reg5: 0x00, reg6: 0x03, reg7: 0x35, reg8: 0x8A, reg9: 0x1B, rega: 0x23, regb: 0x47 },
    _coeff_div { fs: 144, rate: 16000, mclk: 2304000, reg4: 0x20, reg5: 0x00, reg6: 0x11, reg7: 0x35, reg8: 0x8A, reg9: 0x1B, rega: 0x23, regb: 0x47 },
    _coeff_div { fs: 192, rate: 8000, mclk: 1536000, reg4: 0x60, reg5: 0x02, reg6: 0x0D, reg7: 0x75, reg8: 0x8A, reg9: 0x1B, rega: 0x1F, regb: 0x7F },
    _coeff_div { fs: 192, rate: 32000, mclk: 6144000, reg4: 0xE0, reg5: 0x02, reg6: 0x31, reg7: 0x2D, reg8: 0xCA, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 192, rate: 16000, mclk: 3072000, reg4: 0x20, reg5: 0x02, reg6: 0x05, reg7: 0x75, reg8: 0xCA, reg9: 0x1B, rega: 0x1F, regb: 0x3F },
    _coeff_div { fs: 200, rate: 48000, mclk: 9600000, reg4: 0x04, reg5: 0x04, reg6: 0x0F, reg7: 0x2D, reg8: 0xCA, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 250, rate: 48000, mclk: 12000000, reg4: 0x04, reg5: 0x04, reg6: 0x0F, reg7: 0x2D, reg8: 0xCA, reg9: 0x0A, rega: 0x27, regb: 0x27 },
    _coeff_div { fs: 256, rate: 8000, mclk: 2048000, reg4: 0x60, reg5: 0x00, reg6: 0x31, reg7: 0x35, reg8: 0x08, reg9: 0x19, rega: 0x1F, regb: 0x7F },
    _coeff_div { fs: 256, rate: 16000, mclk: 4096000, reg4: 0x20, reg5: 0x00, reg6: 0x01, reg7: 0x35, reg8: 0x08, reg9: 0x19, rega: 0x1F, regb: 0x3F },
    _coeff_div { fs: 256, rate: 44100, mclk: 11289600, reg4: 0xE0, reg5: 0x01, reg6: 0x01, reg7: 0x2D, reg8: 0x48, reg9: 0x08, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 256, rate: 48000, mclk: 12288000, reg4: 0xE0, reg5: 0x01, reg6: 0x01, reg7: 0x2D, reg8: 0x48, reg9: 0x08, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 288, rate: 8000, mclk: 2304000, reg4: 0x20, reg5: 0x00, reg6: 0x01, reg7: 0x35, reg8: 0x8A, reg9: 0x1B, rega: 0x23, regb: 0x47 },
    _coeff_div { fs: 384, rate: 8000, mclk: 3072000, reg4: 0x60, reg5: 0x02, reg6: 0x05, reg7: 0x75, reg8: 0x8A, reg9: 0x1B, rega: 0x1F, regb: 0x7F },
    _coeff_div { fs: 384, rate: 16000, mclk: 6144000, reg4: 0x20, reg5: 0x02, reg6: 0x03, reg7: 0x35, reg8: 0x8A, reg9: 0x1B, rega: 0x1F, regb: 0x3F },
    _coeff_div { fs: 384, rate: 32000, mclk: 12288000, reg4: 0xE0, reg5: 0x02, reg6: 0x01, reg7: 0x2D, reg8: 0xCA, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 384, rate: 48000, mclk: 18432000, reg4: 0x00, reg5: 0x02, reg6: 0x01, reg7: 0x2D, reg8: 0x8A, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 400, rate: 48000, mclk: 19200000, reg4: 0xE4, reg5: 0x04, reg6: 0x35, reg7: 0x6d, reg8: 0xCA, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 500, rate: 48000, mclk: 24000000, reg4: 0xF8, reg5: 0x04, reg6: 0x3F, reg7: 0x6D, reg8: 0xCA, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 512, rate: 8000, mclk: 4096000, reg4: 0x60, reg5: 0x00, reg6: 0x01, reg7: 0x08, reg8: 0x19, reg9: 0x1B, rega: 0x1F, regb: 0x7F },
    _coeff_div { fs: 512, rate: 16000, mclk: 8192000, reg4: 0x20, reg5: 0x00, reg6: 0x30, reg7: 0x35, reg8: 0x08, reg9: 0x19, rega: 0x1F, regb: 0x3F },
    _coeff_div { fs: 512, rate: 44100, mclk: 22579200, reg4: 0xE0, reg5: 0x00, reg6: 0x00, reg7: 0x2D, reg8: 0x48, reg9: 0x08, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 512, rate: 48000, mclk: 24576000, reg4: 0xE0, reg5: 0x00, reg6: 0x00, reg7: 0x2D, reg8: 0x48, reg9: 0x08, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 768, rate: 8000, mclk: 6144000, reg4: 0x60, reg5: 0x02, reg6: 0x11, reg7: 0x35, reg8: 0x8A, reg9: 0x1B, rega: 0x1F, regb: 0x7F },
    _coeff_div { fs: 768, rate: 16000, mclk: 12288000, reg4: 0x20, reg5: 0x02, reg6: 0x01, reg7: 0x35, reg8: 0x8A, reg9: 0x1B, rega: 0x1F, regb: 0x3F },
    _coeff_div { fs: 768, rate: 32000, mclk: 24576000, reg4: 0xE0, reg5: 0x02, reg6: 0x30, reg7: 0x2D, reg8: 0xCA, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 800, rate: 48000, mclk: 38400000, reg4: 0x00, reg5: 0x18, reg6: 0x13, reg7: 0x2D, reg8: 0x8A, reg9: 0x0A, rega: 0x1F, regb: 0x1F },
    _coeff_div { fs: 1024, rate: 8000, mclk: 8192000, reg4: 0x60, reg5: 0x00, reg6: 0x30, reg7: 0x35, reg8: 0x8A, reg9: 0x1B, rega: 0x1F, regb: 0x7F },
    _coeff_div { fs: 1024, rate: 16000, mclk: 16384000, reg4: 0x20, reg5: 0x00, reg6: 0x00, reg7: 0x35, reg8: 0x8A, reg9: 0x1B, rega: 0x1F, regb: 0x3F },
    _coeff_div { fs: 1152, rate: 16000, mclk: 18432000, reg4: 0x20, reg5: 0x08, reg6: 0x11, reg7: 0x35, reg8: 0x8A, reg9: 0x1B, rega: 0x1F, regb: 0x3F },
    _coeff_div { fs: 1536, rate: 8000, mclk: 12288000, reg4: 0x60, reg5: 0x02, reg6: 0x01, reg7: 0x35, reg8: 0x8A, reg9: 0x1B, rega: 0x1F, regb: 0x7F },
    _coeff_div { fs: 1536, rate: 16000, mclk: 24576000, reg4: 0x20, reg5: 0x02, reg6: 0x10, reg7: 0x35, reg8: 0x8A, reg9: 0x1B, rega: 0x1F, regb: 0x3F },
    _coeff_div { fs: 1625, rate: 8000, mclk: 13000000, reg4: 0x0C, reg5: 0x18, reg6: 0x1F, reg7: 0x2D, reg8: 0x8A, reg9: 0x0A, rega: 0x27, regb: 0x27 },
    _coeff_div { fs: 1625, rate: 16000, mclk: 26000000, reg4: 0x0C, reg5: 0x18, reg6: 0x1F, reg7: 0x2D, reg8: 0x8A, reg9: 0x0A, rega: 0x27, regb: 0x27 },
    _coeff_div { fs: 2048, rate: 8000, mclk: 16384000, reg4: 0x60, reg5: 0x00, reg6: 0x00, reg7: 0x35, reg8: 0x8A, reg9: 0x1B, rega: 0x1F, regb: 0x7F },
    _coeff_div { fs: 2304, rate: 8000, mclk: 18432000, reg4: 0x40, reg5: 0x02, reg6: 0x10, reg7: 0x35, reg8: 0x8A, reg9: 0x1B, rega: 0x1F, regb: 0x5F },
    _coeff_div { fs: 3072, rate: 8000, mclk: 24576000, reg4: 0x60, reg5: 0x02, reg6: 0x10, reg7: 0x35, reg8: 0x8A, reg9: 0x1B, rega: 0x1F, regb: 0x7F },
    _coeff_div { fs: 3250, rate: 8000, mclk: 26000000, reg4: 0x0C, reg5: 0x18, reg6: 0x0F, reg7: 0x2D, reg8: 0x8A, reg9: 0x0A, rega: 0x27, regb: 0x27 },
];

unsafe fn get_coeff(mclk: c_int, rate: c_int, array: c_int, coeff_div: *const _coeff_div) -> c_int {
    let mut i: c_int = 0;
    while i < array {
        if (*coeff_div.add(i as usize)).rate == rate as u32 && (*coeff_div.add(i as usize)).mclk == mclk as u32 {
            return i;
        }
        i += 1;
    }
    -EINVAL
}

unsafe extern "C" fn es8326_set_dai_sysclk(codec_dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let codec = (*codec_dai).component;
    let es8326 = snd_soc_component_get_drvdata(codec);
    (*es8326).sysclk = freq;
    0
}

unsafe extern "C" fn es8326_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let mut iface: u8 = 0;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBC_CFP => {
            snd_soc_component_update_bits(component, ES8326_RESET, ES8326_MASTER_MODE_EN, ES8326_MASTER_MODE_EN);
        }
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {}
        SND_SOC_DAIFMT_RIGHT_J => {
            dev_err((*component).dev, cstr!("Codec driver does not support right justified\n"));
            return -EINVAL;
        }
        SND_SOC_DAIFMT_LEFT_J => iface |= ES8326_DAIFMT_LEFT_J as u8,
        SND_SOC_DAIFMT_DSP_A => iface |= ES8326_DAIFMT_DSP_A as u8,
        SND_SOC_DAIFMT_DSP_B => iface |= ES8326_DAIFMT_DSP_B as u8,
        _ => return -EINVAL,
    }

    snd_soc_component_update_bits(component, ES8326_FMT, ES8326_DAIFMT_MASK, iface as c_uint);
    0
}

unsafe extern "C" fn es8326_pcm_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let coeff_div: *const _coeff_div;
    let es8326 = snd_soc_component_get_drvdata(component);
    let mut srate: u8 = 0;
    let array: c_int;

    if (*es8326).version == 0 {
        coeff_div = coeff_div_v0.as_ptr();
        array = coeff_div_v0.len() as c_int;
    } else {
        coeff_div = coeff_div_v3.as_ptr();
        array = coeff_div_v3.len() as c_int;
    }
    let coeff = get_coeff((*es8326).sysclk as c_int, params_rate(params), array, coeff_div);
    /* bit size */
    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => srate |= ES8326_S16_LE as u8,
        SNDRV_PCM_FORMAT_S20_3LE => srate |= ES8326_S20_3_LE as u8,
        SNDRV_PCM_FORMAT_S18_3LE => srate |= ES8326_S18_LE as u8,
        SNDRV_PCM_FORMAT_S24_LE => srate |= ES8326_S24_LE as u8,
        SNDRV_PCM_FORMAT_S32_LE => srate |= ES8326_S32_LE as u8,
        _ => return -EINVAL,
    }

    /* set iface & srate */
    snd_soc_component_update_bits(component, ES8326_FMT, ES8326_DATA_LEN_MASK, srate as c_uint);

    if coeff >= 0 {
        let c = *coeff_div.add(coeff as usize);
        regmap_write((*es8326).regmap, ES8326_CLK_DIV1, c.reg4 as c_uint);
        regmap_write((*es8326).regmap, ES8326_CLK_DIV2, c.reg5 as c_uint);
        regmap_write((*es8326).regmap, ES8326_CLK_DLL, c.reg6 as c_uint);
        regmap_write((*es8326).regmap, ES8326_CLK_MUX, c.reg7 as c_uint);
        regmap_write((*es8326).regmap, ES8326_CLK_ADC_SEL, c.reg8 as c_uint);
        regmap_write((*es8326).regmap, ES8326_CLK_DAC_SEL, c.reg9 as c_uint);
        regmap_write((*es8326).regmap, ES8326_CLK_ADC_OSR, c.rega as c_uint);
        regmap_write((*es8326).regmap, ES8326_CLK_DAC_OSR, c.regb as c_uint);
    } else {
        dev_warn((*component).dev, cstr!("Clock coefficients do not match"));
    }

    0
}

unsafe extern "C" fn es8326_mute(dai: *mut snd_soc_dai, mute: c_int, direction: c_int) -> c_int {
    let component = (*dai).component;
    let es8326 = snd_soc_component_get_drvdata(component);
    let mut offset_l: c_uint = 0;
    let mut offset_r: c_uint = 0;

    if mute != 0 {
        if direction == SNDRV_PCM_STREAM_PLAYBACK {
            regmap_write((*es8326).regmap, ES8326_HP_CAL, ES8326_HP_OFF);
            regmap_update_bits((*es8326).regmap, ES8326_DAC_MUTE, ES8326_MUTE_MASK, ES8326_MUTE);
            regmap_update_bits((*es8326).regmap, ES8326_HP_DRIVER_REF, 0x30, 0x00);
        } else {
            regmap_update_bits((*es8326).regmap, ES8326_ADC_MUTE, 0x0F, 0x0F);
            if (*es8326).version > ES8326_VERSION_B {
                regmap_update_bits((*es8326).regmap, ES8326_VMIDSEL, 0x40, 0x40);
                regmap_update_bits((*es8326).regmap, ES8326_ANA_MICBIAS, 0x70, 0x30);
            }
        }
    } else {
        if !(*es8326).calibrated {
            regmap_write((*es8326).regmap, ES8326_HP_CAL, ES8326_HP_FORCE_CAL);
            msleep(30);
            regmap_write((*es8326).regmap, ES8326_HP_CAL, ES8326_HP_OFF);
            regmap_read((*es8326).regmap, ES8326_HPL_OFFSET_INI, &mut offset_l);
            regmap_read((*es8326).regmap, ES8326_HPR_OFFSET_INI, &mut offset_r);
            regmap_write((*es8326).regmap, ES8326_HP_OFFSET_CAL, 0x8c);
            regmap_write((*es8326).regmap, ES8326_HPL_OFFSET_INI, offset_l);
            regmap_write((*es8326).regmap, ES8326_HPR_OFFSET_INI, offset_r);
            (*es8326).calibrated = true;
        }
        regmap_update_bits((*es8326).regmap, ES8326_CLK_INV, 0xc0, 0x00);
        regmap_update_bits((*es8326).regmap, ES8326_CLK_MUX, 0x80, 0x00);
        if direction == SNDRV_PCM_STREAM_PLAYBACK {
            regmap_update_bits((*es8326).regmap, ES8326_DAC_DSM, 0x01, 0x01);
            usleep_range(1000, 5000);
            regmap_update_bits((*es8326).regmap, ES8326_DAC_DSM, 0x01, 0x00);
            usleep_range(1000, 5000);
            regmap_update_bits((*es8326).regmap, ES8326_HP_DRIVER_REF, 0x30, 0x20);
            regmap_update_bits((*es8326).regmap, ES8326_HP_DRIVER_REF, 0x30, 0x30);
            regmap_write((*es8326).regmap, ES8326_HP_DRIVER, 0xa1);
            regmap_write((*es8326).regmap, ES8326_HP_CAL, ES8326_HP_ON);
            regmap_update_bits((*es8326).regmap, ES8326_DAC_MUTE, ES8326_MUTE_MASK, !ES8326_MUTE);
        } else {
            msleep(300);
            if (*es8326).version > ES8326_VERSION_B {
                regmap_update_bits((*es8326).regmap, ES8326_ANA_MICBIAS, 0x70, 0x70);
                regmap_update_bits((*es8326).regmap, ES8326_VMIDSEL, 0x40, 0x00);
            }
            regmap_update_bits((*es8326).regmap, ES8326_ADC_MUTE, 0x0F, 0x00);
        }
    }
    0
}

unsafe extern "C" fn es8326_set_bias_level(codec: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let es8326 = snd_soc_component_get_drvdata(codec);
    let mut ret: c_int;

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {
            ret = clk_prepare_enable((*es8326).mclk);
            if ret != 0 {
                return ret;
            }
            regmap_update_bits((*es8326).regmap, ES8326_RESET, 0x02, 0x02);
            usleep_range(5000, 10000);
            regmap_write((*es8326).regmap, ES8326_INTOUT_IO, (*es8326).interrupt_clk as c_uint);
            regmap_write((*es8326).regmap, ES8326_SDINOUT1_IO, ES8326_IO_DMIC_CLK << ES8326_SDINOUT1_SHIFT);
            regmap_write((*es8326).regmap, ES8326_PGA_PDN, 0x40);
            regmap_write((*es8326).regmap, ES8326_ANA_PDN, 0x00);
            regmap_update_bits((*es8326).regmap, ES8326_CLK_CTL, 0x20, 0x20);
            regmap_update_bits((*es8326).regmap, ES8326_RESET, 0x02, 0x00);
            if (*es8326).version > ES8326_VERSION_B {
                regmap_update_bits((*es8326).regmap, ES8326_VMIDSEL, 0x40, 0x40);
                regmap_update_bits((*es8326).regmap, ES8326_ANA_MICBIAS, 0x70, 0x30);
            }
        }
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {}
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            regmap_write((*es8326).regmap, ES8326_ANA_PDN, 0x3b);
            regmap_update_bits((*es8326).regmap, ES8326_CLK_CTL, 0x20, 0x00);
            regmap_write((*es8326).regmap, ES8326_SDINOUT1_IO, ES8326_IO_INPUT);
            if (*es8326).version > ES8326_VERSION_B {
                regmap_update_bits((*es8326).regmap, ES8326_VMIDSEL, 0x40, 0x40);
                regmap_update_bits((*es8326).regmap, ES8326_ANA_MICBIAS, 0x70, 0x10);
            }
            regmap_update_bits((*es8326).regmap, ES8326_CLK_INV, 0xc0, 0xc0);
            regmap_update_bits((*es8326).regmap, ES8326_CLK_MUX, 0x80, 0x80);
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            clk_disable_unprepare((*es8326).mclk);
        }
    }
    0
}

const es8326_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE;

static es8326_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(es8326_pcm_hw_params),
    set_fmt: Some(es8326_set_dai_fmt),
    set_sysclk: Some(es8326_set_dai_sysclk),
    mute_stream: Some(es8326_mute),
    no_capture_mute: 0,
};

static mut es8326_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: cstr!("ES8326 HiFi"),
    playback: snd_soc_pcm_stream {
        stream_name: cstr!("Playback"),
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: es8326_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: cstr!("Capture"),
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: es8326_FORMATS,
    },
    ops: &es8326_ops,
    symmetric_rate: 1,
};

unsafe fn es8326_enable_micbias(component: *mut snd_soc_component) {
    let dapm = snd_soc_component_to_dapm(component);
    snd_soc_dapm_mutex_lock(dapm);
    snd_soc_dapm_force_enable_pin_unlocked(dapm, cstr!("MICBIAS1"));
    snd_soc_dapm_force_enable_pin_unlocked(dapm, cstr!("MICBIAS2"));
    snd_soc_dapm_sync_unlocked(dapm);
    snd_soc_dapm_mutex_unlock(dapm);
}

unsafe fn es8326_disable_micbias(component: *mut snd_soc_component) {
    let dapm = snd_soc_component_to_dapm(component);
    snd_soc_dapm_mutex_lock(dapm);
    snd_soc_dapm_disable_pin_unlocked(dapm, cstr!("MICBIAS1"));
    snd_soc_dapm_disable_pin_unlocked(dapm, cstr!("MICBIAS2"));
    snd_soc_dapm_sync_unlocked(dapm);
    snd_soc_dapm_mutex_unlock(dapm);
}

/*
 *	For button detection, set the following in soundcard
 *	snd_jack_set_key(jack->jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
 *	snd_jack_set_key(jack->jack, SND_JACK_BTN_1, KEY_VOLUMEUP);
 *	snd_jack_set_key(jack->jack, SND_JACK_BTN_2, KEY_VOLUMEDOWN);
 */
static mut button_to_report: c_int = 0;
static mut press_count: c_int = 0;
static mut prev_button: c_int = 0;
static mut cur_button: c_int = 0;

unsafe extern "C" fn es8326_jack_button_handler(work: *mut work_struct) {
    let es8326 = container_of_button_press_work(work);
    let comp = (*es8326).component;
    let iface: c_uint;

    if ((*(*es8326).jack).status & SND_JACK_HEADSET) == 0 {
        return;
    }

    mutex_lock(&mut (*es8326).lock);
    iface = snd_soc_component_read(comp, ES8326_HPDET_STA);
    match iface {
        0x93 => {
            /* pause button detected */
            cur_button = SND_JACK_BTN_0;
        }
        0x6f | 0x4b => {
            /* button volume up */
            if iface == 0x6f && (*es8326).version > ES8326_VERSION_B {
                cur_button = SND_JACK_BTN_0;
            } else {
                cur_button = SND_JACK_BTN_1;
            }
        }
        0x27 => {
            /* button volume down */
            cur_button = SND_JACK_BTN_2;
        }
        0x1e | 0xe2 => {
            /* button released or not pressed */
            cur_button = 0;
        }
        _ => {}
    }

    if prev_button == cur_button && cur_button != 0 {
        press_count += 1;
        if press_count > 3 {
            /* report a press every 120ms */
            snd_soc_jack_report((*es8326).jack, cur_button, SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2);
            press_count = 0;
        }
        button_to_report = cur_button;
        queue_delayed_work(system_dfl_wq, &mut (*es8326).button_press_work, msecs_to_jiffies(35));
    } else if prev_button != cur_button {
        /* mismatch, detect again */
        prev_button = cur_button;
        queue_delayed_work(system_dfl_wq, &mut (*es8326).button_press_work, msecs_to_jiffies(35));
    } else {
        /* released or no pressed */
        if button_to_report != 0 {
            snd_soc_jack_report((*es8326).jack, button_to_report, SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2);
            snd_soc_jack_report((*es8326).jack, 0, SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2);
            button_to_report = 0;
        }
        es8326_disable_micbias((*es8326).component);
    }
    mutex_unlock(&mut (*es8326).lock);
}

unsafe extern "C" fn es8326_jack_detect_handler(work: *mut work_struct) {
    let es8326 = container_of_jack_detect_work(work);
    let comp = (*es8326).component;
    let iface: c_uint;

    mutex_lock(&mut (*es8326).lock);
    iface = snd_soc_component_read(comp, ES8326_HPDET_STA);
    dev_dbg((*comp).dev, cstr!("gpio flag %#04x"), iface);

    if (*es8326).jack_remove_retry == 1 && (*es8326).version < ES8326_VERSION_B {
        if (iface & ES8326_HPINSERT_FLAG) != 0 {
            (*es8326).jack_remove_retry = 2;
        } else {
            (*es8326).jack_remove_retry = 0;
        }

        dev_dbg((*comp).dev, cstr!("remove event check, set HPJACK_POL normal, cnt = %d\n"), (*es8326).jack_remove_retry);
        /*
         * Inverted HPJACK_POL bit to trigger one IRQ to double check HP Removal event
         */
        regmap_update_bits((*es8326).regmap, ES8326_HPDET_TYPE, ES8326_HP_DET_JACK_POL,
            if (*es8326).jd_inverted { !((*es8326).jack_pol as c_uint) } else { (*es8326).jack_pol as c_uint });
        mutex_unlock(&mut (*es8326).lock);
        return;
    }

    if (iface & ES8326_HPINSERT_FLAG) == 0 {
        /* Jack unplugged or spurious IRQ */
        dev_dbg((*comp).dev, cstr!("No headset detected\n"));
        es8326_disable_micbias((*es8326).component);
        if ((*(*es8326).jack).status & SND_JACK_HEADPHONE) != 0 {
            dev_dbg((*comp).dev, cstr!("Report hp remove event\n"));
            snd_soc_jack_report((*es8326).jack, 0, SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2);
            snd_soc_jack_report((*es8326).jack, 0, SND_JACK_HEADSET);
            /* mute adc when mic path switch */
            regmap_write((*es8326).regmap, ES8326_ADC1_SRC, 0x44);
            regmap_write((*es8326).regmap, ES8326_ADC2_SRC, 0x66);
        }
        (*es8326).hp = 0;
        regmap_update_bits((*es8326).regmap, ES8326_HPDET_TYPE, 0x03, 0x01);
        regmap_write((*es8326).regmap, ES8326_SYS_BIAS, 0x0a);
        regmap_update_bits((*es8326).regmap, ES8326_HP_DRIVER_REF, 0x0f, 0x03);
        regmap_write((*es8326).regmap, ES8326_INT_SOURCE, ES8326_INT_SRC_PIN9);
        /*
         * Inverted HPJACK_POL bit to trigger one IRQ to double check HP Removal event
         */
        if (*es8326).jack_remove_retry == 0 && (*es8326).version < ES8326_VERSION_B {
            (*es8326).jack_remove_retry = 1;
            dev_dbg((*comp).dev, cstr!("remove event check, invert HPJACK_POL, cnt = %d\n"), (*es8326).jack_remove_retry);
            regmap_update_bits((*es8326).regmap, ES8326_HPDET_TYPE, ES8326_HP_DET_JACK_POL,
                if (*es8326).jd_inverted { (*es8326).jack_pol as c_uint } else { !((*es8326).jack_pol as c_uint) });
        } else {
            (*es8326).jack_remove_retry = 0;
        }
    } else if (iface & ES8326_HPINSERT_FLAG) == ES8326_HPINSERT_FLAG {
        (*es8326).jack_remove_retry = 0;
        if (*es8326).hp == 0 {
            dev_dbg((*comp).dev, cstr!("First insert, start OMTP/CTIA type check\n"));
            /*
             * set auto-check mode, then restart jack_detect_work after 400ms.
             * Don't report jack status.
             */
            regmap_write((*es8326).regmap, ES8326_INT_SOURCE, 0x00);
            regmap_update_bits((*es8326).regmap, ES8326_HPDET_TYPE, 0x03, 0x01);
            regmap_update_bits((*es8326).regmap, ES8326_HPDET_TYPE, 0x10, 0x00);
            usleep_range(50000, 70000);
            regmap_update_bits((*es8326).regmap, ES8326_HPDET_TYPE, 0x03, 0x00);
            regmap_update_bits((*es8326).regmap, ES8326_HPDET_TYPE, 0x10, 0x10);
            usleep_range(50000, 70000);
            regmap_write((*es8326).regmap, ES8326_INT_SOURCE, ES8326_INT_SRC_PIN9 | ES8326_INT_SRC_BUTTON);
            regmap_write((*es8326).regmap, ES8326_SYS_BIAS, 0x1f);
            regmap_update_bits((*es8326).regmap, ES8326_HP_DRIVER_REF, 0x0f, 0x0d);
            queue_delayed_work(system_dfl_wq, &mut (*es8326).jack_detect_work, msecs_to_jiffies(400));
            (*es8326).hp = 1;
            mutex_unlock(&mut (*es8326).lock);
            return;
        }
        if ((*(*es8326).jack).status & SND_JACK_HEADSET) != 0 {
            /* detect button */
            dev_dbg((*comp).dev, cstr!("button pressed\n"));
            regmap_write((*es8326).regmap, ES8326_INT_SOURCE, ES8326_INT_SRC_PIN9 | ES8326_INT_SRC_BUTTON);
            es8326_enable_micbias((*es8326).component);
            queue_delayed_work(system_dfl_wq, &mut (*es8326).button_press_work, 10);
            mutex_unlock(&mut (*es8326).lock);
            return;
        }
        if (iface & ES8326_HPBUTTON_FLAG) == 0x01 {
            dev_dbg((*comp).dev, cstr!("Headphone detected\n"));
            snd_soc_jack_report((*es8326).jack, SND_JACK_HEADPHONE, SND_JACK_HEADSET);
        } else {
            dev_dbg((*comp).dev, cstr!("Headset detected\n"));
            snd_soc_jack_report((*es8326).jack, SND_JACK_HEADSET, SND_JACK_HEADSET);
            regmap_update_bits((*es8326).regmap, ES8326_PGA_PDN, 0x08, 0x08);
            regmap_write((*es8326).regmap, ES8326_ADC1_SRC, 0x00);
            regmap_write((*es8326).regmap, ES8326_ADC2_SRC, 0x00);
            regmap_update_bits((*es8326).regmap, ES8326_PGA_PDN, 0x08, 0x00);
            usleep_range(10000, 15000);
        }
    }
    mutex_unlock(&mut (*es8326).lock);
}

unsafe extern "C" fn es8326_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let es8326 = dev_id as *mut es8326_priv;

    if (*es8326).jack.is_null() {
        return IRQ_HANDLED;
    }

    if ((*(*es8326).jack).status & SND_JACK_HEADSET) != 0 {
        queue_delayed_work(system_dfl_wq, &mut (*es8326).jack_detect_work, msecs_to_jiffies(10));
    } else {
        queue_delayed_work(system_dfl_wq, &mut (*es8326).jack_detect_work, msecs_to_jiffies(300));
    }

    IRQ_HANDLED
}

unsafe fn es8326_calibrate(component: *mut snd_soc_component) -> c_int {
    let es8326 = snd_soc_component_get_drvdata(component);
    let mut reg: c_uint = 0;
    let mut offset_l: c_uint = 0;
    let mut offset_r: c_uint = 0;

    regmap_read((*es8326).regmap, ES8326_CHIP_VERSION, &mut reg);
    (*es8326).version = reg as c_int;

    if (*es8326).version >= ES8326_VERSION_B && (*es8326).calibrated == false {
        dev_dbg((*component).dev, cstr!("ES8326_VERSION_B, calibrating\n"));
        regmap_write((*es8326).regmap, ES8326_CLK_INV, 0xc0);
        regmap_write((*es8326).regmap, ES8326_CLK_DIV1, 0x03);
        regmap_write((*es8326).regmap, ES8326_CLK_DLL, 0x30);
        regmap_write((*es8326).regmap, ES8326_CLK_MUX, 0xed);
        regmap_write((*es8326).regmap, ES8326_CLK_DAC_SEL, 0x08);
        regmap_write((*es8326).regmap, ES8326_CLK_TRI, 0xc1);
        regmap_write((*es8326).regmap, ES8326_DAC_MUTE, 0x03);
        regmap_write((*es8326).regmap, ES8326_ANA_VSEL, 0x7f);
        regmap_write((*es8326).regmap, ES8326_VMIDLOW, 0x23);
        regmap_write((*es8326).regmap, ES8326_DAC2HPMIX, 0x88);
        usleep_range(15000, 20000);
        regmap_write((*es8326).regmap, ES8326_HP_OFFSET_CAL, 0x8c);
        usleep_range(15000, 20000);
        regmap_write((*es8326).regmap, ES8326_RESET, 0xc0);
        usleep_range(15000, 20000);
        regmap_write((*es8326).regmap, ES8326_HP_OFFSET_CAL, ES8326_HP_OFF);
        regmap_read((*es8326).regmap, ES8326_CSM_MUTE_STA, &mut reg);
        if (reg & 0xf0) != 0x40 {
            msleep(50);
        }
        regmap_write((*es8326).regmap, ES8326_HP_CAL, 0xd4);
        msleep(200);
        regmap_write((*es8326).regmap, ES8326_HP_CAL, 0x4d);
        msleep(200);
        regmap_write((*es8326).regmap, ES8326_HP_CAL, ES8326_HP_OFF);
        regmap_read((*es8326).regmap, ES8326_HPL_OFFSET_INI, &mut offset_l);
        regmap_read((*es8326).regmap, ES8326_HPR_OFFSET_INI, &mut offset_r);
        regmap_write((*es8326).regmap, ES8326_HP_OFFSET_CAL, 0x8c);
        regmap_write((*es8326).regmap, ES8326_HPL_OFFSET_INI, offset_l);
        regmap_write((*es8326).regmap, ES8326_HPR_OFFSET_INI, offset_r);
        regmap_write((*es8326).regmap, ES8326_CLK_INV, 0x00);
        (*es8326).calibrated = true;
    }
    0
}

unsafe fn es8326_init(component: *mut snd_soc_component) {
    let es8326 = snd_soc_component_get_drvdata(component);

    regmap_write((*es8326).regmap, ES8326_RESET, 0x1f);
    regmap_write((*es8326).regmap, ES8326_VMIDSEL, 0x3E);
    regmap_write((*es8326).regmap, ES8326_ANA_LP, 0xf0);
    usleep_range(10000, 15000);
    regmap_write((*es8326).regmap, ES8326_HPJACK_TIMER, 0xd9);
    regmap_write((*es8326).regmap, ES8326_ANA_MICBIAS, 0xd8);
    /* set headphone default type and detect pin */
    regmap_write((*es8326).regmap, ES8326_HPDET_TYPE, 0x83);
    regmap_write((*es8326).regmap, ES8326_CLK_RESAMPLE, 0x05);
    /* set internal oscillator as clock source of headpone cp */
    regmap_write((*es8326).regmap, ES8326_CLK_DIV_CPC, 0x89);
    regmap_write((*es8326).regmap, ES8326_CLK_CTL, ES8326_CLK_ON);
    /* clock manager reset release */
    regmap_write((*es8326).regmap, ES8326_RESET, 0x17);
    /* set headphone detection as half scan mode */
    regmap_write((*es8326).regmap, ES8326_HP_MISC, 0x3d);
    regmap_write((*es8326).regmap, ES8326_PULLUP_CTL, 0x00);
    /* enable headphone driver */
    regmap_write((*es8326).regmap, ES8326_HP_VOL, 0xc4);
    regmap_write((*es8326).regmap, ES8326_HP_DRIVER, 0xa7);
    usleep_range(2000, 5000);
    regmap_write((*es8326).regmap, ES8326_HP_DRIVER_REF, 0x23);
    regmap_write((*es8326).regmap, ES8326_HP_DRIVER_REF, 0x33);
    regmap_write((*es8326).regmap, ES8326_HP_DRIVER, 0xa1);
    regmap_write((*es8326).regmap, ES8326_CLK_INV, 0x00);
    regmap_write((*es8326).regmap, ES8326_CLK_VMIDS1, 0xc4);
    regmap_write((*es8326).regmap, ES8326_CLK_VMIDS2, 0x81);
    regmap_write((*es8326).regmap, ES8326_CLK_CAL_TIME, 0x00);
    /* calibrate for B version */
    es8326_calibrate(component);
    regmap_write((*es8326).regmap, ES8326_DAC_CROSSTALK, 0xaa);
    regmap_write((*es8326).regmap, ES8326_DAC_RAMPRATE, 0x00);
    /* turn off headphone out */
    regmap_write((*es8326).regmap, ES8326_HP_CAL, 0x00);
    /* set ADC and DAC in low power mode */
    regmap_write((*es8326).regmap, ES8326_ANA_LP, 0xf0);
    regmap_write((*es8326).regmap, ES8326_ANA_VSEL, 0x7F);
    /* select vdda as micbias source */
    regmap_write((*es8326).regmap, ES8326_VMIDLOW, 0x03);
    /* set dac dsmclip = 1 */
    regmap_write((*es8326).regmap, ES8326_DAC_DSM, 0x08);
    regmap_write((*es8326).regmap, ES8326_DAC_VPPSCALE, 0x15);
    regmap_write((*es8326).regmap, ES8326_HPDET_TYPE, 0x80 | if (*es8326).version >= ES8326_VERSION_B { ES8326_HP_DET_SRC_PIN9 | (*es8326).jack_pol as c_uint } else { ES8326_HP_DET_SRC_PIN9 | (*es8326).jack_pol as c_uint | 0x04 });
    usleep_range(5000, 10000);
    es8326_enable_micbias((*es8326).component);
    usleep_range(50000, 70000);
    regmap_update_bits((*es8326).regmap, ES8326_HPDET_TYPE, 0x03, 0x00);
    regmap_write((*es8326).regmap, ES8326_INTOUT_IO, (*es8326).interrupt_clk as c_uint);
    regmap_write((*es8326).regmap, ES8326_SDINOUT1_IO, ES8326_IO_INPUT);
    regmap_write((*es8326).regmap, ES8326_SDINOUT23_IO, ES8326_IO_INPUT);
    regmap_write((*es8326).regmap, ES8326_ANA_PDN, 0x00);
    regmap_write((*es8326).regmap, ES8326_RESET, ES8326_CSM_ON);
    regmap_update_bits((*es8326).regmap, ES8326_PGAGAIN, ES8326_MIC_SEL_MASK, ES8326_MIC1_SEL);
    regmap_update_bits((*es8326).regmap, ES8326_DAC_MUTE, ES8326_MUTE_MASK, ES8326_MUTE);
    regmap_write((*es8326).regmap, ES8326_ADC_MUTE, 0x0f);
    regmap_write((*es8326).regmap, ES8326_CLK_DIV_LRCK, 0xff);
    regmap_write((*es8326).regmap, ES8326_ADC1_SRC, 0x44);
    regmap_write((*es8326).regmap, ES8326_ADC2_SRC, 0x66);
    es8326_disable_micbias((*es8326).component);
    if (*es8326).version > ES8326_VERSION_B {
        regmap_update_bits((*es8326).regmap, ES8326_ANA_MICBIAS, 0x73, 0x10);
        regmap_update_bits((*es8326).regmap, ES8326_VMIDSEL, 0x40, 0x40);
    }
    msleep(200);
    regmap_write((*es8326).regmap, ES8326_INT_SOURCE, ES8326_INT_SRC_PIN9);
}

unsafe extern "C" fn es8326_resume(component: *mut snd_soc_component) -> c_int {
    let es8326 = snd_soc_component_get_drvdata(component);
    let mut reg: c_uint = 0;
    regcache_cache_only((*es8326).regmap, false);
    regcache_cache_bypass((*es8326).regmap, true);
    regmap_read((*es8326).regmap, ES8326_CLK_RESAMPLE, &mut reg);
    regcache_cache_bypass((*es8326).regmap, false);
    /* reset internal clock state */
    if reg == 0x05 {
        regmap_write((*es8326).regmap, ES8326_CLK_CTL, ES8326_CLK_ON);
    } else {
        es8326_init(component);
    }
    regcache_sync((*es8326).regmap);
    es8326_irq((*es8326).irq, es8326 as *mut c_void);
    0
}

unsafe extern "C" fn es8326_suspend(component: *mut snd_soc_component) -> c_int {
    let es8326 = snd_soc_component_get_drvdata(component);
    cancel_delayed_work_sync(&mut (*es8326).jack_detect_work);
    es8326_disable_micbias(component);
    (*es8326).calibrated = false;
    regmap_write((*es8326).regmap, ES8326_CLK_MUX, 0x2d);
    regmap_write((*es8326).regmap, ES8326_DAC2HPMIX, 0x00);
    regmap_write((*es8326).regmap, ES8326_ANA_PDN, 0x3b);
    regmap_write((*es8326).regmap, ES8326_CLK_CTL, ES8326_CLK_OFF);
    regcache_cache_only((*es8326).regmap, true);
    /* reset register value to default */
    regmap_write((*es8326).regmap, ES8326_CSM_I2C_STA, 0x01);
    usleep_range(1000, 3000);
    regmap_write((*es8326).regmap, ES8326_CSM_I2C_STA, 0x00);
    regcache_mark_dirty((*es8326).regmap);
    0
}

unsafe extern "C" fn es8326_probe(component: *mut snd_soc_component) -> c_int {
    let es8326 = snd_soc_component_get_drvdata(component);
    let mut ret: c_int;
    (*es8326).component = component;
    (*es8326).jd_inverted = device_property_read_bool((*component).dev, cstr!("everest,jack-detect-inverted"));
    ret = device_property_read_u8((*component).dev, cstr!("everest,jack-pol"), &mut (*es8326).jack_pol);
    if ret != 0 {
        dev_dbg((*component).dev, cstr!("jack-pol return %d"), ret);
        (*es8326).jack_pol = ES8326_HP_TYPE_AUTO as u8;
    }
    dev_dbg((*component).dev, cstr!("jack-pol %x"), (*es8326).jack_pol as c_uint);
    ret = device_property_read_u8((*component).dev, cstr!("everest,interrupt-src"), &mut (*es8326).interrupt_src);
    if ret != 0 {
        dev_dbg((*component).dev, cstr!("interrupt-src return %d"), ret);
        (*es8326).interrupt_src = ES8326_HP_DET_SRC_PIN9 as u8;
    }
    dev_dbg((*component).dev, cstr!("interrupt-src %x"), (*es8326).interrupt_src as c_uint);
    ret = device_property_read_u8((*component).dev, cstr!("everest,interrupt-clk"), &mut (*es8326).interrupt_clk);
    if ret != 0 {
        dev_dbg((*component).dev, cstr!("interrupt-clk return %d"), ret);
        (*es8326).interrupt_clk = 0x00;
    }
    dev_dbg((*component).dev, cstr!("interrupt-clk %x"), (*es8326).interrupt_clk as c_uint);
    es8326_init(component);
    0
}

unsafe fn es8326_enable_jack_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack) {
    let es8326 = snd_soc_component_get_drvdata(component);
    mutex_lock(&mut (*es8326).lock);
    if (*es8326).jd_inverted {
        snd_soc_component_update_bits(component, ES8326_HPDET_TYPE, ES8326_HP_DET_JACK_POL, !((*es8326).jack_pol as c_uint));
    }
    (*es8326).jack = jack;
    mutex_unlock(&mut (*es8326).lock);
    es8326_irq((*es8326).irq, es8326 as *mut c_void);
}

unsafe fn es8326_disable_jack_detect(component: *mut snd_soc_component) {
    let es8326 = snd_soc_component_get_drvdata(component);
    dev_dbg((*component).dev, cstr!("Enter into %s\n"), cstr!("es8326_disable_jack_detect"));
    if (*es8326).jack.is_null() {
        return; /* Already disabled (or never enabled) */
    }
    cancel_delayed_work_sync(&mut (*es8326).jack_detect_work);
    mutex_lock(&mut (*es8326).lock);
    if ((*(*es8326).jack).status & SND_JACK_MICROPHONE) != 0 {
        es8326_disable_micbias(component);
        snd_soc_jack_report((*es8326).jack, 0, SND_JACK_HEADSET);
    }
    (*es8326).jack = core::ptr::null_mut();
    mutex_unlock(&mut (*es8326).lock);
}

unsafe extern "C" fn es8326_set_jack(component: *mut snd_soc_component, jack: *mut snd_soc_jack, _data: *mut c_void) -> c_int {
    if !jack.is_null() {
        es8326_enable_jack_detect(component, jack);
    } else {
        es8326_disable_jack_detect(component);
    }
    0
}

unsafe extern "C" fn es8326_remove(component: *mut snd_soc_component) {
    let es8326 = snd_soc_component_get_drvdata(component);
    es8326_disable_jack_detect(component);
    es8326_set_bias_level(component, snd_soc_bias_level::SND_SOC_BIAS_OFF);
    regmap_write((*es8326).regmap, ES8326_CSM_I2C_STA, 0x01);
    usleep_range(1000, 3000);
    regmap_write((*es8326).regmap, ES8326_CSM_I2C_STA, 0x00);
}

static soc_component_dev_es8326: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(es8326_probe),
    remove: Some(es8326_remove),
    resume: Some(es8326_resume),
    suspend: Some(es8326_suspend),
    set_bias_level: Some(es8326_set_bias_level),
    set_jack: Some(es8326_set_jack),
    dapm_widgets: es8326_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(es8326_dapm_widgets),
    dapm_routes: es8326_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(es8326_dapm_routes),
    controls: es8326_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(es8326_snd_controls),
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn es8326_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let es8326: *mut es8326_priv;
    let mut ret: c_int;
    es8326 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<es8326_priv>(), GFP_KERNEL) as *mut es8326_priv;
    if es8326.is_null() {
        return -ENOMEM;
    }
    i2c_set_clientdata(i2c, es8326 as *mut c_void);
    (*es8326).i2c = i2c;
    mutex_init(&mut (*es8326).lock);
    (*es8326).regmap = devm_regmap_init_i2c(i2c, &es8326_regmap_config);
    if IS_ERR((*es8326).regmap as *const c_void) {
        ret = PTR_ERR((*es8326).regmap as *const c_void);
        dev_err(&mut (*i2c).dev, cstr!("Failed to init regmap: %d\n"), ret);
        return ret;
    }
    (*es8326).irq = (*i2c).irq;
    (*es8326).jack_remove_retry = 0;
    (*es8326).hp = 0;
    (*es8326).hpl_vol = 0x03;
    (*es8326).hpr_vol = 0x03;
    INIT_DELAYED_WORK(&mut (*es8326).jack_detect_work, es8326_jack_detect_handler);
    INIT_DELAYED_WORK(&mut (*es8326).button_press_work, es8326_jack_button_handler);
    /* ES8316 is level-based while ES8326 is edge-based */
    ret = devm_request_threaded_irq(&mut (*i2c).dev, (*es8326).irq, core::ptr::null(), es8326_irq,
        IRQF_TRIGGER_RISING | IRQF_ONESHOT, cstr!("es8326"), es8326 as *mut c_void);
    if ret != 0 {
        dev_warn(&mut (*i2c).dev, cstr!("Failed to request IRQ: %d: %d\n"), (*es8326).irq, ret);
        (*es8326).irq = -ENXIO;
    }
    (*es8326).mclk = devm_clk_get_optional(&mut (*i2c).dev, cstr!("mclk"));
    if IS_ERR((*es8326).mclk as *const c_void) {
        dev_err(&mut (*i2c).dev, cstr!("unable to get mclk\n"));
        return PTR_ERR((*es8326).mclk as *const c_void);
    }
    if (*es8326).mclk.is_null() {
        dev_warn(&mut (*i2c).dev, cstr!("assuming static mclk\n"));
    }
    ret = clk_prepare_enable((*es8326).mclk);
    if ret != 0 {
        dev_err(&mut (*i2c).dev, cstr!("unable to enable mclk\n"));
        return ret;
    }
    devm_snd_soc_register_component(&mut (*i2c).dev, &soc_component_dev_es8326, &mut es8326_dai, 1)
}

unsafe extern "C" fn es8326_i2c_shutdown(i2c: *mut i2c_client) {
    let component: *mut snd_soc_component;
    let es8326: *mut es8326_priv;
    es8326 = i2c_get_clientdata(i2c);
    component = (*es8326).component;
    dev_dbg((*component).dev, cstr!("Enter into %s\n"), cstr!("es8326_i2c_shutdown"));
    cancel_delayed_work_sync(&mut (*es8326).jack_detect_work);
    cancel_delayed_work_sync(&mut (*es8326).button_press_work);
    regmap_write((*es8326).regmap, ES8326_CSM_I2C_STA, 0x01);
    usleep_range(1000, 3000);
    regmap_write((*es8326).regmap, ES8326_CSM_I2C_STA, 0x00);
}

unsafe extern "C" fn es8326_i2c_remove(i2c: *mut i2c_client) {
    es8326_i2c_shutdown(i2c);
}

static es8326_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: [b'e' as c_char, b's' as c_char, b'8' as c_char, b'3' as c_char, b'2' as c_char, b'6' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], driver_data: 0 },
    i2c_device_id { name: [0; 20], driver_data: 0 },
];
MODULE_DEVICE_TABLE!(i2c, es8326_i2c_id);

/* CONFIG_OF: preserve Open Firmware match table intent. */
static es8326_of_match_local: [of_device_id; 2] = [
    of_device_id { compatible: cstr!("everest,es8326") },
    of_device_id { compatible: core::ptr::null() },
];
MODULE_DEVICE_TABLE!(of, es8326_of_match);

/* CONFIG_ACPI: preserve ACPI match table intent. */
static es8326_acpi_match_local: [acpi_device_id; 2] = [
    acpi_device_id { id: [b'E' as c_char, b'S' as c_char, b'S' as c_char, b'X' as c_char, b'8' as c_char, b'3' as c_char, b'2' as c_char, b'6' as c_char, 0, 0, 0, 0, 0, 0, 0, 0], driver_data: 0 },
    acpi_device_id { id: [0; 16], driver_data: 0 },
];
MODULE_DEVICE_TABLE!(acpi, es8326_acpi_match);

static mut es8326_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: cstr!("es8326"),
        acpi_match_table: es8326_acpi_match_local.as_ptr(),
        of_match_table: es8326_of_match_local.as_ptr(),
    },
    probe: Some(es8326_i2c_probe),
    shutdown: Some(es8326_i2c_shutdown),
    remove: Some(es8326_i2c_remove),
    id_table: es8326_i2c_id.as_ptr(),
};
module_i2c_driver!(es8326_i2c_driver);

MODULE_DESCRIPTION!("ASoC es8326 driver");
MODULE_AUTHOR!("David Yang <yangxiaohua@everest-semi.com>");
MODULE_LICENSE!("GPL");

extern "Rust" {
    fn container_of_button_press_work(work: *mut work_struct) -> *mut es8326_priv;
    fn container_of_jack_detect_work(work: *mut work_struct) -> *mut es8326_priv;
    fn INIT_DELAYED_WORK(work: *mut delayed_work, func: unsafe extern "C" fn(*mut work_struct));
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
