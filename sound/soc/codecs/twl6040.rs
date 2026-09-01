// SPDX-License-Identifier: GPL-2.0-only
/*
 * ALSA SoC TWL6040 codec driver
 *
 * Author:	 Misael Lopez Cruz <x0052729@ti.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type u8 = u8;
type bool_ = bool;
type irqreturn_t = c_int;

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
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
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
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
    pub enumerated: snd_ctl_elem_value_enumerated,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 4],
}
#[repr(C)]
pub struct soc_enum {
    pub reg: c_uint,
}
#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *const c_uint,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}
#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai {
    pub id: c_int,
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    pub parent: *mut device,
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct twl6040 {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dapm_widget {
    _private2: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
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
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub read: Option<unsafe extern "C" fn(*mut snd_soc_component, c_uint) -> c_uint>,
    pub write: Option<unsafe extern "C" fn(*mut snd_soc_component, c_uint, c_uint) -> c_int>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub suspend_bias_off: c_uint,
    pub idle_bias_on: c_uint,
    pub endianness: c_uint,
}
#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}
#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum twl6040_dai_id {
    TWL6040_DAI_LEGACY = 0,
    TWL6040_DAI_UL,
    TWL6040_DAI_DL1,
    TWL6040_DAI_DL2,
    TWL6040_DAI_VIB,
}

type snd_soc_bias_level = c_uint;
const SND_SOC_BIAS_ON: snd_soc_bias_level = 0;
const SND_SOC_BIAS_PREPARE: snd_soc_bias_level = 1;
const SND_SOC_BIAS_STANDBY: snd_soc_bias_level = 2;
const SND_SOC_BIAS_OFF: snd_soc_bias_level = 3;

const TWL6040_RATES: c_uint = SNDRV_PCM_RATE_8000_96000;
const TWL6040_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S32_LE;

const TWL6040_OUTHS_0dB: c_uint = 0x00;
const TWL6040_OUTHS_M30dB: c_uint = 0x0F;
const TWL6040_OUTHF_0dB: c_uint = 0x03;
const TWL6040_OUTHF_M52dB: c_uint = 0x1D;

const TWL6040_CACHEREGNUM: c_uint = TWL6040_REG_STATUS + 1;
const DL12_CACHE_LEN: usize = (TWL6040_REG_HFRCTL - TWL6040_REG_HSLCTL + 1) as usize;

#[repr(C)]
struct twl6040_jack_data {
    jack: *mut snd_soc_jack,
    work: delayed_work,
    report: c_int,
}

/* codec private data */
#[repr(C)]
struct twl6040_data {
    plug_irq: c_int,
    codec_powered: c_int,
    pll: c_int,
    pll_power_mode: c_int,
    hs_power_mode: c_int,
    hs_power_mode_locked: c_int,
    dl1_unmuted: bool_,
    dl2_unmuted: bool_,
    dl12_cache: [u8; DL12_CACHE_LEN],
    clk_in: c_uint,
    sysclk: c_uint,
    hs_jack: twl6040_jack_data,
    component: *mut snd_soc_component,
    mutex: mutex,
}

/* set of rates for each pll: low-power and high-performance */
static lp_rates: [c_uint; 9] = [8000, 11250, 16000, 22500, 32000, 44100, 48000, 88200, 96000];
static hp_rates: [c_uint; 5] = [8000, 16000, 32000, 48000, 96000];

static sysclk_constraints: [snd_pcm_hw_constraint_list; 2] = [
    snd_pcm_hw_constraint_list { count: lp_rates.len() as c_uint, list: lp_rates.as_ptr() },
    snd_pcm_hw_constraint_list { count: hp_rates.len() as c_uint, list: hp_rates.as_ptr() },
];

unsafe fn to_twl6040(component: *mut snd_soc_component) -> *mut twl6040 {
    dev_get_drvdata((*(*component).dev).parent) as *mut twl6040
}

unsafe extern "C" fn twl6040_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut twl6040_data;
    let twl6040 = to_twl6040(component);
    let value: u8;

    if reg >= TWL6040_CACHEREGNUM {
        return -EIO as c_uint;
    }

    match reg {
        TWL6040_REG_HSLCTL | TWL6040_REG_HSRCTL | TWL6040_REG_EARCTL |
        TWL6040_REG_HFLCTL | TWL6040_REG_HFRCTL => {
            value = (*priv_).dl12_cache[(reg - TWL6040_REG_HSLCTL) as usize];
        }
        _ => {
            value = twl6040_reg_read(twl6040, reg) as u8;
        }
    }

    value as c_uint
}

unsafe fn twl6040_can_write_to_chip(component: *mut snd_soc_component, reg: c_uint) -> bool {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut twl6040_data;

    match reg {
        TWL6040_REG_HSLCTL | TWL6040_REG_HSRCTL | TWL6040_REG_EARCTL => (*priv_).dl1_unmuted,
        TWL6040_REG_HFLCTL | TWL6040_REG_HFRCTL => (*priv_).dl2_unmuted,
        _ => true,
    }
}

unsafe fn twl6040_update_dl12_cache(component: *mut snd_soc_component, reg: u8, value: u8) {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut twl6040_data;

    match reg as c_uint {
        TWL6040_REG_HSLCTL | TWL6040_REG_HSRCTL | TWL6040_REG_EARCTL |
        TWL6040_REG_HFLCTL | TWL6040_REG_HFRCTL => {
            (*priv_).dl12_cache[(reg as c_uint - TWL6040_REG_HSLCTL) as usize] = value;
        }
        _ => {}
    }
}

unsafe extern "C" fn twl6040_write(component: *mut snd_soc_component, reg: c_uint, value: c_uint) -> c_int {
    let twl6040 = to_twl6040(component);

    if reg >= TWL6040_CACHEREGNUM {
        return -EIO;
    }

    twl6040_update_dl12_cache(component, reg as u8, value as u8);
    if twl6040_can_write_to_chip(component, reg) {
        twl6040_reg_write(twl6040, reg, value)
    } else {
        0
    }
}

unsafe fn twl6040_init_chip(component: *mut snd_soc_component) {
    twl6040_read(component, TWL6040_REG_TRIM1);
    twl6040_read(component, TWL6040_REG_TRIM2);
    twl6040_read(component, TWL6040_REG_TRIM3);
    twl6040_read(component, TWL6040_REG_HSOTRIM);
    twl6040_read(component, TWL6040_REG_HFOTRIM);

    /* Change chip defaults */
    /* No imput selected for microphone amplifiers */
    twl6040_write(component, TWL6040_REG_MICLCTL, 0x18);
    twl6040_write(component, TWL6040_REG_MICRCTL, 0x18);

    /*
     * We need to lower the default gain values, so the ramp code
     * can work correctly for the first playback.
     * This reduces the pop noise heard at the first playback.
     */
    twl6040_write(component, TWL6040_REG_HSGAIN, 0xff);
    twl6040_write(component, TWL6040_REG_EARCTL, 0x1e);
    twl6040_write(component, TWL6040_REG_HFLGAIN, 0x1d);
    twl6040_write(component, TWL6040_REG_HFRGAIN, 0x1d);
    twl6040_write(component, TWL6040_REG_LINEGAIN, 0);
}

/* set headset dac and driver power mode */
unsafe fn headset_power_mode(component: *mut snd_soc_component, high_perf: c_int) -> c_int {
    let mask = TWL6040_HSDRVMODE | TWL6040_HSDACMODE;
    let mut hslctl = twl6040_read(component, TWL6040_REG_HSLCTL) as c_int;
    let mut hsrctl = twl6040_read(component, TWL6040_REG_HSRCTL) as c_int;

    if high_perf != 0 {
        hslctl &= !(mask as c_int);
        hsrctl &= !(mask as c_int);
    } else {
        hslctl |= mask as c_int;
        hsrctl |= mask as c_int;
    }

    twl6040_write(component, TWL6040_REG_HSLCTL, hslctl as c_uint);
    twl6040_write(component, TWL6040_REG_HSRCTL, hsrctl as c_uint);

    0
}

unsafe extern "C" fn twl6040_hs_dac_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let mut hslctl = twl6040_read(component, TWL6040_REG_HSLCTL) as u8;
    let mut hsrctl = twl6040_read(component, TWL6040_REG_HSRCTL) as u8;

    /*
     * Workaround for Headset DC offset caused pop noise:
     * Both HS DAC need to be turned on (before the HS driver) and off at
     * the same time.
     */
    if SND_SOC_DAPM_EVENT_ON(event) != 0 {
        hslctl |= TWL6040_HSDACENA as u8;
        hsrctl |= TWL6040_HSDACENA as u8;
    } else {
        hslctl &= !(TWL6040_HSDACENA as u8);
        hsrctl &= !(TWL6040_HSDACENA as u8);
    }
    twl6040_write(component, TWL6040_REG_HSLCTL, hslctl as c_uint);
    twl6040_write(component, TWL6040_REG_HSRCTL, hsrctl as c_uint);

    msleep(1);
    0
}

unsafe extern "C" fn twl6040_ep_drv_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(component) as *mut twl6040_data;
    let mut ret = 0;

    if SND_SOC_DAPM_EVENT_ON(event) != 0 {
        /* Earphone doesn't support low power mode */
        (*priv_).hs_power_mode_locked = 1;
        ret = headset_power_mode(component, 1);
    } else {
        (*priv_).hs_power_mode_locked = 0;
        ret = headset_power_mode(component, (*priv_).hs_power_mode);
    }

    msleep(1);
    ret
}

unsafe fn twl6040_hs_jack_report(component: *mut snd_soc_component, jack: *mut snd_soc_jack, report: c_int) {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut twl6040_data;
    let status: c_int;

    mutex_lock(&mut (*priv_).mutex);
    /* Sync status */
    status = twl6040_read(component, TWL6040_REG_STATUS) as c_int;
    if (status & TWL6040_PLUGCOMP as c_int) != 0 {
        snd_soc_jack_report(jack, report, report);
    } else {
        snd_soc_jack_report(jack, 0, report);
    }
    mutex_unlock(&mut (*priv_).mutex);
}

#[no_mangle]
pub unsafe extern "C" fn twl6040_hs_jack_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack, report: c_int) {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut twl6040_data;
    let hs_jack = &mut (*priv_).hs_jack as *mut twl6040_jack_data;

    (*hs_jack).jack = jack;
    (*hs_jack).report = report;

    twl6040_hs_jack_report(component, (*hs_jack).jack, (*hs_jack).report);
}

unsafe extern "C" fn twl6040_accessory_work(work: *mut work_struct) {
    let priv_ = container_of_hs_jack_work_work(work);
    let component = (*priv_).component;
    let hs_jack = &mut (*priv_).hs_jack as *mut twl6040_jack_data;

    twl6040_hs_jack_report(component, (*hs_jack).jack, (*hs_jack).report);
}

/* audio interrupt handler */
unsafe extern "C" fn twl6040_audio_handler(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let component = data as *mut snd_soc_component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut twl6040_data;

    queue_delayed_work(system_power_efficient_wq, &mut (*priv_).hs_jack.work, msecs_to_jiffies(200));

    IRQ_HANDLED
}

unsafe extern "C" fn twl6040_soc_dapm_put_vibra_enum(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let val: c_uint;

    /* Do not allow changes while Input/FF efect is running */
    val = twl6040_read(component, (*e).reg);
    if (val & TWL6040_VIBENA) != 0 && (val & TWL6040_VIBSEL) == 0 {
        return -EBUSY;
    }

    snd_soc_dapm_put_enum_double(kcontrol, ucontrol)
}

/*
 * TLV declarations translated from DECLARE_TLV_DB_SCALE:
 * mic_preamp_tlv: -600, 600, 0
 * mic_amp_tlv: 600, 600, 0
 * afm_amp_tlv: -1800, 600, 0
 * hs_tlv: -3000, 200, 0
 * hf_tlv: -5200, 200, 0
 * ep_tlv: -2400, 200, 0
 */
static mic_preamp_tlv: [c_uint; 4] = TLV_DB_SCALE_ITEM(-600, 600, 0);
static mic_amp_tlv: [c_uint; 4] = TLV_DB_SCALE_ITEM(600, 600, 0);
static afm_amp_tlv: [c_uint; 4] = TLV_DB_SCALE_ITEM(-1800, 600, 0);
static hs_tlv: [c_uint; 4] = TLV_DB_SCALE_ITEM(-3000, 200, 0);
static hf_tlv: [c_uint; 4] = TLV_DB_SCALE_ITEM(-5200, 200, 0);
static ep_tlv: [c_uint; 4] = TLV_DB_SCALE_ITEM(-2400, 200, 0);

/* Left analog microphone selection */
static twl6040_amicl_texts: [*const c_char; 4] = [c"Headset Mic".as_ptr(), c"Main Mic".as_ptr(), c"Aux/FM Left".as_ptr(), c"Off".as_ptr()];
/* Right analog microphone selection */
static twl6040_amicr_texts: [*const c_char; 4] = [c"Headset Mic".as_ptr(), c"Sub Mic".as_ptr(), c"Aux/FM Right".as_ptr(), c"Off".as_ptr()];

static twl6040_enum: [soc_enum; 2] = [
    SOC_ENUM_SINGLE(TWL6040_REG_MICLCTL, 3, twl6040_amicl_texts.len() as c_uint, twl6040_amicl_texts.as_ptr()),
    SOC_ENUM_SINGLE(TWL6040_REG_MICRCTL, 3, twl6040_amicr_texts.len() as c_uint, twl6040_amicr_texts.as_ptr()),
];

static twl6040_hs_texts: [*const c_char; 3] = [c"Off".as_ptr(), c"HS DAC".as_ptr(), c"Line-In amp".as_ptr()];
static twl6040_hs_enum: [soc_enum; 2] = [
    SOC_ENUM_SINGLE(TWL6040_REG_HSLCTL, 5, twl6040_hs_texts.len() as c_uint, twl6040_hs_texts.as_ptr()),
    SOC_ENUM_SINGLE(TWL6040_REG_HSRCTL, 5, twl6040_hs_texts.len() as c_uint, twl6040_hs_texts.as_ptr()),
];

static twl6040_hf_texts: [*const c_char; 3] = [c"Off".as_ptr(), c"HF DAC".as_ptr(), c"Line-In amp".as_ptr()];
static twl6040_hf_enum: [soc_enum; 2] = [
    SOC_ENUM_SINGLE(TWL6040_REG_HFLCTL, 2, twl6040_hf_texts.len() as c_uint, twl6040_hf_texts.as_ptr()),
    SOC_ENUM_SINGLE(TWL6040_REG_HFRCTL, 2, twl6040_hf_texts.len() as c_uint, twl6040_hf_texts.as_ptr()),
];

static twl6040_vibrapath_texts: [*const c_char; 2] = [c"Input FF".as_ptr(), c"Audio PDM".as_ptr()];
static twl6040_vibra_enum: [soc_enum; 2] = [
    SOC_ENUM_SINGLE(TWL6040_REG_VIBCTLL, 1, twl6040_vibrapath_texts.len() as c_uint, twl6040_vibrapath_texts.as_ptr()),
    SOC_ENUM_SINGLE(TWL6040_REG_VIBCTLR, 1, twl6040_vibrapath_texts.len() as c_uint, twl6040_vibrapath_texts.as_ptr()),
];

static amicl_control: snd_kcontrol_new = SOC_DAPM_ENUM(c"Route".as_ptr(), &twl6040_enum[0]);
static amicr_control: snd_kcontrol_new = SOC_DAPM_ENUM(c"Route".as_ptr(), &twl6040_enum[1]);
/* Headset DAC playback switches */
static hsl_mux_controls: snd_kcontrol_new = SOC_DAPM_ENUM(c"Route".as_ptr(), &twl6040_hs_enum[0]);
static hsr_mux_controls: snd_kcontrol_new = SOC_DAPM_ENUM(c"Route".as_ptr(), &twl6040_hs_enum[1]);
/* Handsfree DAC playback switches */
static hfl_mux_controls: snd_kcontrol_new = SOC_DAPM_ENUM(c"Route".as_ptr(), &twl6040_hf_enum[0]);
static hfr_mux_controls: snd_kcontrol_new = SOC_DAPM_ENUM(c"Route".as_ptr(), &twl6040_hf_enum[1]);
static ep_path_enable_control: snd_kcontrol_new = SOC_DAPM_SINGLE_VIRT(c"Switch".as_ptr(), 1);
static auxl_switch_control: snd_kcontrol_new = SOC_DAPM_SINGLE(c"Switch".as_ptr(), TWL6040_REG_HFLCTL, 6, 1, 0);
static auxr_switch_control: snd_kcontrol_new = SOC_DAPM_SINGLE(c"Switch".as_ptr(), TWL6040_REG_HFRCTL, 6, 1, 0);
/* Vibra playback switches */
static vibral_mux_controls: snd_kcontrol_new = SOC_DAPM_ENUM_EXT(c"Route".as_ptr(), &twl6040_vibra_enum[0], snd_soc_dapm_get_enum_double, twl6040_soc_dapm_put_vibra_enum);
static vibrar_mux_controls: snd_kcontrol_new = SOC_DAPM_ENUM_EXT(c"Route".as_ptr(), &twl6040_vibra_enum[1], snd_soc_dapm_get_enum_double, twl6040_soc_dapm_put_vibra_enum);

/* Headset power mode */
static twl6040_power_mode_texts: [*const c_char; 2] = [c"Low-Power".as_ptr(), c"High-Performance".as_ptr()];
static twl6040_power_mode_enum: soc_enum = SOC_ENUM_SINGLE_EXT_DECL(twl6040_power_mode_texts.as_ptr());

unsafe extern "C" fn twl6040_headset_power_get_enum(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_component_get_drvdata(component) as *mut twl6040_data;
    (*ucontrol).value.enumerated.item[0] = (*priv_).hs_power_mode as c_uint;
    0
}

unsafe extern "C" fn twl6040_headset_power_put_enum(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_component_get_drvdata(component) as *mut twl6040_data;
    let high_perf = (*ucontrol).value.enumerated.item[0] as c_int;
    let mut ret = 0;

    if (*priv_).hs_power_mode_locked == 0 {
        ret = headset_power_mode(component, high_perf);
    }

    if ret == 0 {
        (*priv_).hs_power_mode = high_perf;
    }

    ret
}

unsafe extern "C" fn twl6040_pll_get_enum(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_component_get_drvdata(component) as *mut twl6040_data;
    (*ucontrol).value.enumerated.item[0] = (*priv_).pll_power_mode as c_uint;
    0
}

unsafe extern "C" fn twl6040_pll_put_enum(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_component_get_drvdata(component) as *mut twl6040_data;
    (*priv_).pll_power_mode = (*ucontrol).value.enumerated.item[0] as c_int;
    0
}

#[no_mangle]
pub unsafe extern "C" fn twl6040_get_dl1_gain(component: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);

    if snd_soc_dapm_get_pin_status(dapm, c"EP".as_ptr()) != 0 {
        return -1; /* -1dB */
    }

    if snd_soc_dapm_get_pin_status(dapm, c"HSOR".as_ptr()) != 0 ||
        snd_soc_dapm_get_pin_status(dapm, c"HSOL".as_ptr()) != 0 {
        let val = twl6040_read(component, TWL6040_REG_HSLCTL) as u8;
        if (val & TWL6040_HSDACMODE as u8) != 0 {
            return -8; /* -8dB */
        } else {
            return -1; /* -1dB */
        }
    }
    0 /* 0dB */
}

#[no_mangle]
pub unsafe extern "C" fn twl6040_get_clk_id(component: *mut snd_soc_component) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut twl6040_data;
    (*priv_).pll_power_mode
}

#[no_mangle]
pub unsafe extern "C" fn twl6040_get_trim_value(component: *mut snd_soc_component, trim: twl6040_trim) -> c_int {
    if unlikely((trim as c_uint) >= TWL6040_TRIM_INVAL) != 0 {
        return -EINVAL;
    }
    twl6040_read(component, TWL6040_REG_TRIM1 + trim as c_uint) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn twl6040_get_hs_step_size(component: *mut snd_soc_component) -> c_int {
    let twl6040 = to_twl6040(component);
    if twl6040_get_revid(twl6040) < TWL6040_REV_ES1_3 {
        2
    } else {
        1
    }
}

static twl6040_snd_controls: [snd_kcontrol_new; 10] = [
    SOC_DOUBLE_TLV(c"Capture Preamplifier Volume".as_ptr(), TWL6040_REG_MICGAIN, 6, 7, 1, 1, mic_preamp_tlv.as_ptr()),
    SOC_DOUBLE_TLV(c"Capture Volume".as_ptr(), TWL6040_REG_MICGAIN, 0, 3, 4, 0, mic_amp_tlv.as_ptr()),
    SOC_DOUBLE_TLV(c"Aux FM Volume".as_ptr(), TWL6040_REG_LINEGAIN, 0, 3, 7, 0, afm_amp_tlv.as_ptr()),
    SOC_DOUBLE_TLV(c"Headset Playback Volume".as_ptr(), TWL6040_REG_HSGAIN, 0, 4, 0xF, 1, hs_tlv.as_ptr()),
    SOC_DOUBLE_R_TLV(c"Handsfree Playback Volume".as_ptr(), TWL6040_REG_HFLGAIN, TWL6040_REG_HFRGAIN, 0, 0x1D, 1, hf_tlv.as_ptr()),
    SOC_SINGLE_TLV(c"Earphone Playback Volume".as_ptr(), TWL6040_REG_EARCTL, 1, 0xF, 1, ep_tlv.as_ptr()),
    SOC_ENUM_EXT(c"Headset Power Mode".as_ptr(), &twl6040_power_mode_enum, twl6040_headset_power_get_enum, twl6040_headset_power_put_enum),
    SOC_SINGLE(c"Headset Mono to Stereo Playback Switch".as_ptr(), TWL6040_REG_HSRCTL, 7, 1, 0),
    SOC_SINGLE(c"Handsfree Mono to Stereo Playback Switch".as_ptr(), TWL6040_REG_HFRCTL, 5, 1, 0),
    SOC_ENUM_EXT(c"PLL Selection".as_ptr(), &twl6040_power_mode_enum, twl6040_pll_get_enum, twl6040_pll_put_enum),
];

/* DAPM widgets translated as macro-initialized external ASoC widget descriptors. */
static twl6040_dapm_widgets: [snd_soc_dapm_widget; 43] = [
    SND_SOC_DAPM_INPUT(c"MAINMIC".as_ptr()), SND_SOC_DAPM_INPUT(c"HSMIC".as_ptr()), SND_SOC_DAPM_INPUT(c"SUBMIC".as_ptr()), SND_SOC_DAPM_INPUT(c"AFML".as_ptr()), SND_SOC_DAPM_INPUT(c"AFMR".as_ptr()),
    SND_SOC_DAPM_OUTPUT(c"HSOL".as_ptr()), SND_SOC_DAPM_OUTPUT(c"HSOR".as_ptr()), SND_SOC_DAPM_OUTPUT(c"HFL".as_ptr()), SND_SOC_DAPM_OUTPUT(c"HFR".as_ptr()), SND_SOC_DAPM_OUTPUT(c"EP".as_ptr()), SND_SOC_DAPM_OUTPUT(c"AUXL".as_ptr()), SND_SOC_DAPM_OUTPUT(c"AUXR".as_ptr()), SND_SOC_DAPM_OUTPUT(c"VIBRAL".as_ptr()), SND_SOC_DAPM_OUTPUT(c"VIBRAR".as_ptr()),
    SND_SOC_DAPM_MUX(c"Analog Left Capture Route".as_ptr(), SND_SOC_NOPM, 0, 0, &amicl_control), SND_SOC_DAPM_MUX(c"Analog Right Capture Route".as_ptr(), SND_SOC_NOPM, 0, 0, &amicr_control),
    SND_SOC_DAPM_PGA(c"MicAmpL".as_ptr(), TWL6040_REG_MICLCTL, 0, 0, core::ptr::null(), 0), SND_SOC_DAPM_PGA(c"MicAmpR".as_ptr(), TWL6040_REG_MICRCTL, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA(c"AFMAmpL".as_ptr(), TWL6040_REG_MICLCTL, 1, 0, core::ptr::null(), 0), SND_SOC_DAPM_PGA(c"AFMAmpR".as_ptr(), TWL6040_REG_MICRCTL, 1, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_ADC(c"ADC Left".as_ptr(), core::ptr::null(), TWL6040_REG_MICLCTL, 2, 0), SND_SOC_DAPM_ADC(c"ADC Right".as_ptr(), core::ptr::null(), TWL6040_REG_MICRCTL, 2, 0),
    SND_SOC_DAPM_SUPPLY(c"Headset Mic Bias".as_ptr(), TWL6040_REG_AMICBCTL, 0, 0, core::ptr::null(), 0), SND_SOC_DAPM_SUPPLY(c"Main Mic Bias".as_ptr(), TWL6040_REG_AMICBCTL, 4, 0, core::ptr::null(), 0), SND_SOC_DAPM_SUPPLY(c"Digital Mic1 Bias".as_ptr(), TWL6040_REG_DMICBCTL, 0, 0, core::ptr::null(), 0), SND_SOC_DAPM_SUPPLY(c"Digital Mic2 Bias".as_ptr(), TWL6040_REG_DMICBCTL, 4, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_DAC(c"HSDAC Left".as_ptr(), core::ptr::null(), SND_SOC_NOPM, 0, 0), SND_SOC_DAPM_DAC(c"HSDAC Right".as_ptr(), core::ptr::null(), SND_SOC_NOPM, 0, 0), SND_SOC_DAPM_DAC(c"HFDAC Left".as_ptr(), core::ptr::null(), TWL6040_REG_HFLCTL, 0, 0), SND_SOC_DAPM_DAC(c"HFDAC Right".as_ptr(), core::ptr::null(), TWL6040_REG_HFRCTL, 0, 0), SND_SOC_DAPM_DAC(c"VIBRA DAC".as_ptr(), core::ptr::null(), SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_MUX(c"Handsfree Left Playback".as_ptr(), SND_SOC_NOPM, 0, 0, &hfl_mux_controls), SND_SOC_DAPM_MUX(c"Handsfree Right Playback".as_ptr(), SND_SOC_NOPM, 0, 0, &hfr_mux_controls), SND_SOC_DAPM_MUX(c"Headset Left Playback".as_ptr(), SND_SOC_NOPM, 0, 0, &hsl_mux_controls), SND_SOC_DAPM_MUX(c"Headset Right Playback".as_ptr(), SND_SOC_NOPM, 0, 0, &hsr_mux_controls),
    SND_SOC_DAPM_MUX(c"Vibra Left Playback".as_ptr(), SND_SOC_NOPM, 0, 0, &vibral_mux_controls), SND_SOC_DAPM_MUX(c"Vibra Right Playback".as_ptr(), SND_SOC_NOPM, 0, 0, &vibrar_mux_controls),
    SND_SOC_DAPM_SWITCH(c"Earphone Playback".as_ptr(), SND_SOC_NOPM, 0, 0, &ep_path_enable_control), SND_SOC_DAPM_SWITCH(c"AUXL Playback".as_ptr(), SND_SOC_NOPM, 0, 0, &auxl_switch_control), SND_SOC_DAPM_SWITCH(c"AUXR Playback".as_ptr(), SND_SOC_NOPM, 0, 0, &auxr_switch_control),
    SND_SOC_DAPM_OUT_DRV(c"HF Left Driver".as_ptr(), TWL6040_REG_HFLCTL, 4, 0, core::ptr::null(), 0), SND_SOC_DAPM_OUT_DRV(c"HF Right Driver".as_ptr(), TWL6040_REG_HFRCTL, 4, 0, core::ptr::null(), 0), SND_SOC_DAPM_OUT_DRV(c"HS Left Driver".as_ptr(), TWL6040_REG_HSLCTL, 2, 0, core::ptr::null(), 0), SND_SOC_DAPM_OUT_DRV(c"HS Right Driver".as_ptr(), TWL6040_REG_HSRCTL, 2, 0, core::ptr::null(), 0), SND_SOC_DAPM_OUT_DRV_E(c"Earphone Driver".as_ptr(), TWL6040_REG_EARCTL, 0, 0, core::ptr::null(), 0, twl6040_ep_drv_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_OUT_DRV(c"Vibra Left Driver".as_ptr(), TWL6040_REG_VIBCTLL, 0, 0, core::ptr::null(), 0), SND_SOC_DAPM_OUT_DRV(c"Vibra Right Driver".as_ptr(), TWL6040_REG_VIBCTLR, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY(c"Vibra Left Control".as_ptr(), TWL6040_REG_VIBCTLL, 2, 0, core::ptr::null(), 0), SND_SOC_DAPM_SUPPLY(c"Vibra Right Control".as_ptr(), TWL6040_REG_VIBCTLR, 2, 0, core::ptr::null(), 0), SND_SOC_DAPM_SUPPLY_S(c"HSDAC Power".as_ptr(), 1, SND_SOC_NOPM, 0, 0, twl6040_hs_dac_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_PGA(c"HF Left PGA".as_ptr(), TWL6040_REG_HFLCTL, 1, 0, core::ptr::null(), 0), SND_SOC_DAPM_PGA(c"HF Right PGA".as_ptr(), TWL6040_REG_HFRCTL, 1, 0, core::ptr::null(), 0),
];

static intercon: [snd_soc_dapm_route; 67] = [
    route(c"HSDAC Left".as_ptr(), core::ptr::null(), c"Legacy Playback".as_ptr()), route(c"HSDAC Left".as_ptr(), core::ptr::null(), c"Headset Playback".as_ptr()), route(c"HSDAC Right".as_ptr(), core::ptr::null(), c"Legacy Playback".as_ptr()), route(c"HSDAC Right".as_ptr(), core::ptr::null(), c"Headset Playback".as_ptr()),
    route(c"HFDAC Left".as_ptr(), core::ptr::null(), c"Legacy Playback".as_ptr()), route(c"HFDAC Left".as_ptr(), core::ptr::null(), c"Handsfree Playback".as_ptr()), route(c"HFDAC Right".as_ptr(), core::ptr::null(), c"Legacy Playback".as_ptr()), route(c"HFDAC Right".as_ptr(), core::ptr::null(), c"Handsfree Playback".as_ptr()),
    route(c"VIBRA DAC".as_ptr(), core::ptr::null(), c"Legacy Playback".as_ptr()), route(c"VIBRA DAC".as_ptr(), core::ptr::null(), c"Vibra Playback".as_ptr()),
    route(c"Legacy Capture".as_ptr(), core::ptr::null(), c"ADC Left".as_ptr()), route(c"Capture".as_ptr(), core::ptr::null(), c"ADC Left".as_ptr()), route(c"Legacy Capture".as_ptr(), core::ptr::null(), c"ADC Right".as_ptr()), route(c"Capture".as_ptr(), core::ptr::null(), c"ADC Right".as_ptr()),
    route(c"Analog Left Capture Route".as_ptr(), c"Headset Mic".as_ptr(), c"HSMIC".as_ptr()), route(c"Analog Left Capture Route".as_ptr(), c"Main Mic".as_ptr(), c"MAINMIC".as_ptr()), route(c"Analog Left Capture Route".as_ptr(), c"Aux/FM Left".as_ptr(), c"AFML".as_ptr()),
    route(c"Analog Right Capture Route".as_ptr(), c"Headset Mic".as_ptr(), c"HSMIC".as_ptr()), route(c"Analog Right Capture Route".as_ptr(), c"Sub Mic".as_ptr(), c"SUBMIC".as_ptr()), route(c"Analog Right Capture Route".as_ptr(), c"Aux/FM Right".as_ptr(), c"AFMR".as_ptr()),
    route(c"MicAmpL".as_ptr(), core::ptr::null(), c"Analog Left Capture Route".as_ptr()), route(c"MicAmpR".as_ptr(), core::ptr::null(), c"Analog Right Capture Route".as_ptr()),
    route(c"ADC Left".as_ptr(), core::ptr::null(), c"MicAmpL".as_ptr()), route(c"ADC Right".as_ptr(), core::ptr::null(), c"MicAmpR".as_ptr()),
    route(c"AFMAmpL".as_ptr(), core::ptr::null(), c"AFML".as_ptr()), route(c"AFMAmpR".as_ptr(), core::ptr::null(), c"AFMR".as_ptr()),
    route(c"HSDAC Left".as_ptr(), core::ptr::null(), c"HSDAC Power".as_ptr()), route(c"HSDAC Right".as_ptr(), core::ptr::null(), c"HSDAC Power".as_ptr()),
    route(c"Headset Left Playback".as_ptr(), c"HS DAC".as_ptr(), c"HSDAC Left".as_ptr()), route(c"Headset Left Playback".as_ptr(), c"Line-In amp".as_ptr(), c"AFMAmpL".as_ptr()), route(c"Headset Right Playback".as_ptr(), c"HS DAC".as_ptr(), c"HSDAC Right".as_ptr()), route(c"Headset Right Playback".as_ptr(), c"Line-In amp".as_ptr(), c"AFMAmpR".as_ptr()),
    route(c"HS Left Driver".as_ptr(), core::ptr::null(), c"Headset Left Playback".as_ptr()), route(c"HS Right Driver".as_ptr(), core::ptr::null(), c"Headset Right Playback".as_ptr()), route(c"HSOL".as_ptr(), core::ptr::null(), c"HS Left Driver".as_ptr()), route(c"HSOR".as_ptr(), core::ptr::null(), c"HS Right Driver".as_ptr()),
    route(c"Earphone Playback".as_ptr(), c"Switch".as_ptr(), c"HSDAC Left".as_ptr()), route(c"Earphone Driver".as_ptr(), core::ptr::null(), c"Earphone Playback".as_ptr()), route(c"EP".as_ptr(), core::ptr::null(), c"Earphone Driver".as_ptr()),
    route(c"Handsfree Left Playback".as_ptr(), c"HF DAC".as_ptr(), c"HFDAC Left".as_ptr()), route(c"Handsfree Left Playback".as_ptr(), c"Line-In amp".as_ptr(), c"AFMAmpL".as_ptr()), route(c"Handsfree Right Playback".as_ptr(), c"HF DAC".as_ptr(), c"HFDAC Right".as_ptr()), route(c"Handsfree Right Playback".as_ptr(), c"Line-In amp".as_ptr(), c"AFMAmpR".as_ptr()),
    route(c"HF Left PGA".as_ptr(), core::ptr::null(), c"Handsfree Left Playback".as_ptr()), route(c"HF Right PGA".as_ptr(), core::ptr::null(), c"Handsfree Right Playback".as_ptr()), route(c"HF Left Driver".as_ptr(), core::ptr::null(), c"HF Left PGA".as_ptr()), route(c"HF Right Driver".as_ptr(), core::ptr::null(), c"HF Right PGA".as_ptr()),
    route(c"HFL".as_ptr(), core::ptr::null(), c"HF Left Driver".as_ptr()), route(c"HFR".as_ptr(), core::ptr::null(), c"HF Right Driver".as_ptr()), route(c"AUXL Playback".as_ptr(), c"Switch".as_ptr(), c"HF Left PGA".as_ptr()), route(c"AUXR Playback".as_ptr(), c"Switch".as_ptr(), c"HF Right PGA".as_ptr()), route(c"AUXL".as_ptr(), core::ptr::null(), c"AUXL Playback".as_ptr()), route(c"AUXR".as_ptr(), core::ptr::null(), c"AUXR Playback".as_ptr()),
    route(c"Vibra Left Playback".as_ptr(), c"Audio PDM".as_ptr(), c"VIBRA DAC".as_ptr()), route(c"Vibra Right Playback".as_ptr(), c"Audio PDM".as_ptr(), c"VIBRA DAC".as_ptr()), route(c"Vibra Left Driver".as_ptr(), core::ptr::null(), c"Vibra Left Playback".as_ptr()), route(c"Vibra Right Driver".as_ptr(), core::ptr::null(), c"Vibra Right Playback".as_ptr()), route(c"Vibra Left Driver".as_ptr(), core::ptr::null(), c"Vibra Left Control".as_ptr()), route(c"Vibra Right Driver".as_ptr(), core::ptr::null(), c"Vibra Right Control".as_ptr()), route(c"VIBRAL".as_ptr(), core::ptr::null(), c"Vibra Left Driver".as_ptr()), route(c"VIBRAR".as_ptr(), core::ptr::null(), c"Vibra Right Driver".as_ptr()),
];

const fn route(sink: *const c_char, control: *const c_char, source: *const c_char) -> snd_soc_dapm_route {
    snd_soc_dapm_route { sink, control, source }
}

unsafe extern "C" fn twl6040_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let twl6040 = to_twl6040(component);
    let priv_ = snd_soc_component_get_drvdata(component) as *mut twl6040_data;
    let mut ret = 0;

    match level {
        SND_SOC_BIAS_ON => {}
        SND_SOC_BIAS_PREPARE => {}
        SND_SOC_BIAS_STANDBY => {
            if (*priv_).codec_powered != 0 {
                ret = twl6040_set_pll(twl6040, TWL6040_SYSCLK_SEL_LPPLL, 32768, 19200000);
            } else {
                ret = twl6040_power(twl6040, 1);
                if ret == 0 {
                    (*priv_).codec_powered = 1;
                    /* Set external boost GPO */
                    twl6040_write(component, TWL6040_REG_GPOCTL, 0x02);
                }
            }
        }
        SND_SOC_BIAS_OFF => {
            if (*priv_).codec_powered != 0 {
                twl6040_power(twl6040, 0);
                (*priv_).codec_powered = 0;
            }
        }
        _ => {}
    }

    ret
}

unsafe extern "C" fn twl6040_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut twl6040_data;

    snd_pcm_hw_constraint_list((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &sysclk_constraints[(*priv_).pll_power_mode as usize]);
    0
}

unsafe extern "C" fn twl6040_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut twl6040_data;
    let rate = params_rate(params);

    match rate {
        11250 | 22500 | 44100 | 88200 => {
            /* These rates are not supported when HPPLL is in use */
            if unlikely(((*priv_).pll == TWL6040_SYSCLK_SEL_HPPLL) as c_int) != 0 {
                dev_err((*component).dev, c"HPPLL does not support rate %d\n".as_ptr(), rate);
                return -EINVAL;
            }
            (*priv_).sysclk = 17640000;
        }
        8000 | 16000 | 32000 | 48000 | 96000 => {
            (*priv_).sysclk = 19200000;
        }
        _ => {
            dev_err((*component).dev, c"unsupported rate %d\n".as_ptr(), rate);
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn twl6040_prepare(_substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let twl6040 = to_twl6040(component);
    let priv_ = snd_soc_component_get_drvdata(component) as *mut twl6040_data;
    let ret: c_int;

    if (*priv_).sysclk == 0 {
        dev_err((*component).dev, c"no mclk configured, call set_sysclk() on init\n".as_ptr());
        return -EINVAL;
    }

    ret = twl6040_set_pll(twl6040, (*priv_).pll, (*priv_).clk_in, (*priv_).sysclk);
    if ret != 0 {
        dev_err((*component).dev, c"Can not set PLL (%d)\n".as_ptr(), ret);
        return -EPERM;
    }

    0
}

unsafe extern "C" fn twl6040_set_dai_sysclk(codec_dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*codec_dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut twl6040_data;

    match clk_id {
        TWL6040_SYSCLK_SEL_LPPLL | TWL6040_SYSCLK_SEL_HPPLL => {
            (*priv_).pll = clk_id;
            (*priv_).clk_in = freq;
        }
        _ => {
            dev_err((*component).dev, c"unknown clk_id %d\n".as_ptr(), clk_id);
            return -EINVAL;
        }
    }

    0
}

unsafe fn twl6040_mute_path(component: *mut snd_soc_component, id: twl6040_dai_id, mute: c_int) {
    let twl6040 = to_twl6040(component);
    let priv_ = snd_soc_component_get_drvdata(component) as *mut twl6040_data;

    match id {
        twl6040_dai_id::TWL6040_DAI_DL1 => {
            let mut hslctl = twl6040_read(component, TWL6040_REG_HSLCTL) as c_int;
            let mut hsrctl = twl6040_read(component, TWL6040_REG_HSRCTL) as c_int;
            let mut earctl = twl6040_read(component, TWL6040_REG_EARCTL) as c_int;

            if mute != 0 {
                /* Power down drivers and DACs */
                earctl &= !0x01;
                hslctl &= !((TWL6040_HSDRVENA | TWL6040_HSDACENA) as c_int);
                hsrctl &= !((TWL6040_HSDRVENA | TWL6040_HSDACENA) as c_int);
            }

            twl6040_reg_write(twl6040, TWL6040_REG_EARCTL, earctl as c_uint);
            twl6040_reg_write(twl6040, TWL6040_REG_HSLCTL, hslctl as c_uint);
            twl6040_reg_write(twl6040, TWL6040_REG_HSRCTL, hsrctl as c_uint);
            (*priv_).dl1_unmuted = mute == 0;
        }
        twl6040_dai_id::TWL6040_DAI_DL2 => {
            let mut hflctl = twl6040_read(component, TWL6040_REG_HFLCTL) as c_int;
            let mut hfrctl = twl6040_read(component, TWL6040_REG_HFRCTL) as c_int;

            if mute != 0 {
                /* Power down drivers and DACs */
                hflctl &= !((TWL6040_HFDACENA | TWL6040_HFPGAENA | TWL6040_HFDRVENA | TWL6040_HFSWENA) as c_int);
                hfrctl &= !((TWL6040_HFDACENA | TWL6040_HFPGAENA | TWL6040_HFDRVENA | TWL6040_HFSWENA) as c_int);
            }

            twl6040_reg_write(twl6040, TWL6040_REG_HFLCTL, hflctl as c_uint);
            twl6040_reg_write(twl6040, TWL6040_REG_HFRCTL, hfrctl as c_uint);
            (*priv_).dl2_unmuted = mute == 0;
        }
        _ => {}
    }
}

unsafe extern "C" fn twl6040_mute_stream(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    match (*dai).id {
        x if x == twl6040_dai_id::TWL6040_DAI_LEGACY as c_int => {
            twl6040_mute_path((*dai).component, twl6040_dai_id::TWL6040_DAI_DL1, mute);
            twl6040_mute_path((*dai).component, twl6040_dai_id::TWL6040_DAI_DL2, mute);
        }
        x if x == twl6040_dai_id::TWL6040_DAI_DL1 as c_int => {
            twl6040_mute_path((*dai).component, twl6040_dai_id::TWL6040_DAI_DL1, mute);
        }
        x if x == twl6040_dai_id::TWL6040_DAI_DL2 as c_int => {
            twl6040_mute_path((*dai).component, twl6040_dai_id::TWL6040_DAI_DL2, mute);
        }
        _ => {}
    }

    0
}

static twl6040_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(twl6040_startup),
    hw_params: Some(twl6040_hw_params),
    prepare: Some(twl6040_prepare),
    set_sysclk: Some(twl6040_set_dai_sysclk),
    mute_stream: Some(twl6040_mute_stream),
    no_capture_mute: 1,
};

static mut twl6040_dai: [snd_soc_dai_driver; 5] = [
    dai_driver(c"twl6040-legacy".as_ptr(), twl6040_dai_id::TWL6040_DAI_LEGACY as c_int, stream(c"Legacy Playback".as_ptr(), 1, 5, TWL6040_RATES, TWL6040_FORMATS), stream(c"Legacy Capture".as_ptr(), 1, 2, TWL6040_RATES, TWL6040_FORMATS)),
    dai_driver(c"twl6040-ul".as_ptr(), twl6040_dai_id::TWL6040_DAI_UL as c_int, empty_stream(), stream(c"Capture".as_ptr(), 1, 2, TWL6040_RATES, TWL6040_FORMATS)),
    dai_driver(c"twl6040-dl1".as_ptr(), twl6040_dai_id::TWL6040_DAI_DL1 as c_int, stream(c"Headset Playback".as_ptr(), 1, 2, TWL6040_RATES, TWL6040_FORMATS), empty_stream()),
    dai_driver(c"twl6040-dl2".as_ptr(), twl6040_dai_id::TWL6040_DAI_DL2 as c_int, stream(c"Handsfree Playback".as_ptr(), 1, 2, TWL6040_RATES, TWL6040_FORMATS), empty_stream()),
    dai_driver(c"twl6040-vib".as_ptr(), twl6040_dai_id::TWL6040_DAI_VIB as c_int, stream(c"Vibra Playback".as_ptr(), 1, 1, SNDRV_PCM_RATE_CONTINUOUS, TWL6040_FORMATS), empty_stream()),
];

const fn stream(stream_name: *const c_char, channels_min: c_uint, channels_max: c_uint, rates: c_uint, formats: c_uint) -> snd_soc_pcm_stream {
    snd_soc_pcm_stream { stream_name, channels_min, channels_max, rates, formats }
}
const fn empty_stream() -> snd_soc_pcm_stream {
    stream(core::ptr::null(), 0, 0, 0, 0)
}
const fn dai_driver(name: *const c_char, id: c_int, playback: snd_soc_pcm_stream, capture: snd_soc_pcm_stream) -> snd_soc_dai_driver {
    snd_soc_dai_driver { name, id, playback, capture, ops: &twl6040_dai_ops }
}

unsafe extern "C" fn twl6040_probe(component: *mut snd_soc_component) -> c_int {
    let mut priv_: *mut twl6040_data;
    let dapm = snd_soc_component_to_dapm(component);
    let pdev = to_platform_device((*component).dev);
    let mut ret = 0;

    priv_ = devm_kzalloc((*component).dev, core::mem::size_of::<twl6040_data>(), GFP_KERNEL) as *mut twl6040_data;
    if priv_.is_null() {
        return -ENOMEM;
    }

    snd_soc_component_set_drvdata(component, priv_ as *mut c_void);
    (*priv_).component = component;

    (*priv_).plug_irq = platform_get_irq(pdev, 0);
    if (*priv_).plug_irq < 0 {
        return (*priv_).plug_irq;
    }

    INIT_DELAYED_WORK(&mut (*priv_).hs_jack.work, twl6040_accessory_work);
    mutex_init(&mut (*priv_).mutex);

    ret = request_threaded_irq((*priv_).plug_irq, None, Some(twl6040_audio_handler), IRQF_NO_SUSPEND | IRQF_ONESHOT, c"twl6040_irq_plug".as_ptr(), component as *mut c_void);
    if ret != 0 {
        dev_err((*component).dev, c"PLUG IRQ request failed: %d\n".as_ptr(), ret);
        return ret;
    }

    snd_soc_dapm_force_bias_level(dapm, SND_SOC_BIAS_STANDBY);
    twl6040_init_chip(component);

    0
}

unsafe extern "C" fn twl6040_remove(component: *mut snd_soc_component) {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut twl6040_data;
    free_irq((*priv_).plug_irq, component as *mut c_void);
}

static soc_component_dev_twl6040: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(twl6040_probe),
    remove: Some(twl6040_remove),
    read: Some(twl6040_read),
    write: Some(twl6040_write),
    set_bias_level: Some(twl6040_set_bias_level),
    controls: twl6040_snd_controls.as_ptr(),
    num_controls: twl6040_snd_controls.len() as c_uint,
    dapm_widgets: twl6040_dapm_widgets.as_ptr(),
    num_dapm_widgets: twl6040_dapm_widgets.len() as c_uint,
    dapm_routes: intercon.as_ptr(),
    num_dapm_routes: intercon.len() as c_uint,
    suspend_bias_off: 1,
    idle_bias_on: 1,
    endianness: 1,
};

unsafe extern "C" fn twl6040_codec_probe(pdev: *mut platform_device) -> c_int {
    devm_snd_soc_register_component(&mut (*pdev).dev, &soc_component_dev_twl6040, twl6040_dai.as_mut_ptr(), twl6040_dai.len() as c_uint)
}

static mut twl6040_codec_driver: platform_driver = platform_driver {
    driver: device_driver { name: c"twl6040-codec".as_ptr() },
    probe: Some(twl6040_codec_probe),
};

/* module_platform_driver(twl6040_codec_driver); */
/* MODULE_DESCRIPTION("ASoC TWL6040 codec driver"); */
/* MODULE_AUTHOR("Misael Lopez Cruz"); */
/* MODULE_LICENSE("GPL"); */

extern "C" {
    static mut system_power_efficient_wq: *mut c_void;

    static SNDRV_PCM_RATE_8000_96000: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
    static SNDRV_PCM_RATE_CONTINUOUS: c_uint;
    static SNDRV_PCM_HW_PARAM_RATE: c_uint;
    static SND_SOC_NOPM: c_uint;
    static SND_SOC_DAPM_PRE_PMU: c_uint;
    static SND_SOC_DAPM_POST_PMD: c_uint;
    static SND_SOC_DAPM_POST_PMU: c_uint;
    static IRQF_NO_SUSPEND: c_uint;
    static IRQF_ONESHOT: c_uint;
    static IRQ_HANDLED: irqreturn_t;
    static GFP_KERNEL: c_uint;

    static TWL6040_REG_STATUS: c_uint;
    static TWL6040_REG_HSLCTL: c_uint;
    static TWL6040_REG_HSRCTL: c_uint;
    static TWL6040_REG_EARCTL: c_uint;
    static TWL6040_REG_HFLCTL: c_uint;
    static TWL6040_REG_HFRCTL: c_uint;
    static TWL6040_REG_TRIM1: c_uint;
    static TWL6040_REG_TRIM2: c_uint;
    static TWL6040_REG_TRIM3: c_uint;
    static TWL6040_REG_HSOTRIM: c_uint;
    static TWL6040_REG_HFOTRIM: c_uint;
    static TWL6040_REG_MICLCTL: c_uint;
    static TWL6040_REG_MICRCTL: c_uint;
    static TWL6040_REG_HSGAIN: c_uint;
    static TWL6040_REG_HFLGAIN: c_uint;
    static TWL6040_REG_HFRGAIN: c_uint;
    static TWL6040_REG_LINEGAIN: c_uint;
    static TWL6040_REG_VIBCTLL: c_uint;
    static TWL6040_REG_VIBCTLR: c_uint;
    static TWL6040_REG_MICGAIN: c_uint;
    static TWL6040_REG_AMICBCTL: c_uint;
    static TWL6040_REG_DMICBCTL: c_uint;
    static TWL6040_REG_GPOCTL: c_uint;

    static TWL6040_HSDRVMODE: c_uint;
    static TWL6040_HSDACMODE: c_uint;
    static TWL6040_HSDACENA: c_uint;
    static TWL6040_HSDRVENA: c_uint;
    static TWL6040_HFDACENA: c_uint;
    static TWL6040_HFPGAENA: c_uint;
    static TWL6040_HFDRVENA: c_uint;
    static TWL6040_HFSWENA: c_uint;
    static TWL6040_PLUGCOMP: c_uint;
    static TWL6040_VIBENA: c_uint;
    static TWL6040_VIBSEL: c_uint;
    static TWL6040_TRIM_INVAL: c_uint;
    static TWL6040_REV_ES1_3: c_int;
    static TWL6040_SYSCLK_SEL_LPPLL: c_int;
    static TWL6040_SYSCLK_SEL_HPPLL: c_int;

    static EIO: c_int;
    static EBUSY: c_int;
    static EINVAL: c_int;
    static EPERM: c_int;
    static ENOMEM: c_int;

    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_set_drvdata(component: *mut snd_soc_component, data: *mut c_void);
    fn twl6040_reg_read(twl6040: *mut twl6040, reg: c_uint) -> c_int;
    fn twl6040_reg_write(twl6040: *mut twl6040, reg: c_uint, value: c_uint) -> c_int;
    fn twl6040_set_pll(twl6040: *mut twl6040, pll: c_int, clk_in: c_uint, sysclk: c_uint) -> c_int;
    fn twl6040_power(twl6040: *mut twl6040, on: c_int) -> c_int;
    fn twl6040_get_revid(twl6040: *mut twl6040) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_dapm_kcontrol_to_component(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn snd_soc_dapm_put_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_dapm_get_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_pin_status(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_force_bias_level(dapm: *mut snd_soc_dapm_context, level: snd_soc_bias_level);
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_uint, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn msleep(msecs: c_uint);
    fn mutex_init(mutex: *mut mutex);
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn queue_delayed_work(wq: *mut c_void, work: *mut delayed_work, delay: c_uint) -> bool;
    fn msecs_to_jiffies(msecs: c_uint) -> c_uint;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn request_threaded_irq(irq: c_int, handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, thread_fn: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, flags: c_uint, name: *const c_char, dev: *mut c_void) -> c_int;
    fn free_irq(irq: c_int, dev: *mut c_void);
    fn to_platform_device(dev: *mut device) -> *mut platform_device;
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_uint) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn unlikely(v: c_int) -> c_int;

    fn container_of_hs_jack_work_work(work: *mut work_struct) -> *mut twl6040_data;

    fn TLV_DB_SCALE_ITEM(min: c_int, step: c_int, mute: c_int) -> [c_uint; 4];
    fn SOC_ENUM_SINGLE(reg: c_uint, shift: c_uint, items: c_uint, texts: *const *const c_char) -> soc_enum;
    fn SOC_ENUM_SINGLE_EXT_DECL(texts: *const *const c_char) -> soc_enum;
    fn SOC_DAPM_ENUM(name: *const c_char, e: *const soc_enum) -> snd_kcontrol_new;
    fn SOC_DAPM_SINGLE_VIRT(name: *const c_char, max: c_uint) -> snd_kcontrol_new;
    fn SOC_DAPM_SINGLE(name: *const c_char, reg: c_uint, shift: c_uint, max: c_uint, invert: c_uint) -> snd_kcontrol_new;
    fn SOC_DAPM_ENUM_EXT(name: *const c_char, e: *const soc_enum, get: unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int, put: unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int) -> snd_kcontrol_new;
    fn SOC_DOUBLE_TLV(name: *const c_char, reg: c_uint, shift_left: c_uint, shift_right: c_uint, max: c_uint, invert: c_uint, tlv: *const c_uint) -> snd_kcontrol_new;
    fn SOC_DOUBLE_R_TLV(name: *const c_char, reg_left: c_uint, reg_right: c_uint, shift: c_uint, max: c_uint, invert: c_uint, tlv: *const c_uint) -> snd_kcontrol_new;
    fn SOC_SINGLE_TLV(name: *const c_char, reg: c_uint, shift: c_uint, max: c_uint, invert: c_uint, tlv: *const c_uint) -> snd_kcontrol_new;
    fn SOC_ENUM_EXT(name: *const c_char, e: *const soc_enum, get: unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int, put: unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int) -> snd_kcontrol_new;
    fn SOC_SINGLE(name: *const c_char, reg: c_uint, shift: c_uint, max: c_uint, invert: c_uint) -> snd_kcontrol_new;
    fn SND_SOC_DAPM_INPUT(name: *const c_char) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_OUTPUT(name: *const c_char) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_MUX(name: *const c_char, reg: c_uint, shift: c_uint, invert: c_uint, kcontrol: *const snd_kcontrol_new) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_PGA(name: *const c_char, reg: c_uint, shift: c_uint, invert: c_uint, controls: *const snd_kcontrol_new, num_controls: c_uint) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_ADC(name: *const c_char, stream: *const c_char, reg: c_uint, shift: c_uint, invert: c_uint) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_SUPPLY(name: *const c_char, reg: c_uint, shift: c_uint, invert: c_uint, event: *const c_void, event_flags: c_uint) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_DAC(name: *const c_char, stream: *const c_char, reg: c_uint, shift: c_uint, invert: c_uint) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_SWITCH(name: *const c_char, reg: c_uint, shift: c_uint, invert: c_uint, kcontrol: *const snd_kcontrol_new) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_OUT_DRV(name: *const c_char, reg: c_uint, shift: c_uint, invert: c_uint, controls: *const snd_kcontrol_new, num_controls: c_uint) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_OUT_DRV_E(name: *const c_char, reg: c_uint, shift: c_uint, invert: c_uint, controls: *const snd_kcontrol_new, num_controls: c_uint, event: unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int, event_flags: c_uint) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_SUPPLY_S(name: *const c_char, subseq: c_int, reg: c_uint, shift: c_uint, invert: c_uint, event: unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int, event_flags: c_uint) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_EVENT_ON(event: c_int) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum twl6040_trim {
    _Opaque = 0,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
