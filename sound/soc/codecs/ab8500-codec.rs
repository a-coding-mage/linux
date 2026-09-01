// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) ST-Ericsson SA 2012
 *
 * Author: Ola Lilja <ola.o.lilja@stericsson.com>,
 *         Kristoffer Karlsson <kristoffer.karlsson@stericsson.com>,
 *         Roger Nilsson <roger.xr.nilsson@stericsson.com>,
 *         for ST-Ericsson.
 *
 *         Based on the early work done by:
 *         Mikko J. Lehto <mikko.lehto@symbio.com>,
 *         Mikko Sarmanne <mikko.sarmanne@symbio.com>,
 *         Jarmo K. Kuronen <jarmo.kuronen@symbio.com>,
 *         for ST-Ericsson.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

/* Includes translated as external dependency intent:
 * linux/cleanup.h, kernel.h, module.h, device.h, slab.h, moduleparam.h,
 * init.h, delay.h, pm.h, platform_device.h, mutex.h, mfd/abx500,
 * regulator/consumer.h, of.h, sound core/pcm/soc/tlv, and ab8500-codec.h.
 */

type u8 = u8;
type u32 = u32;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
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
    pub enumerated: snd_ctl_elem_value_enumerated,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}
#[repr(C)]
pub struct snd_soc_dapm_context {
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
pub struct snd_soc_dai_ops {
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_tdm_slot: Option<
        unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int,
    >,
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
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
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub controls: *mut snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}
#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
}
#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}
#[repr(C)]
pub struct regmap_config {
    pub reg_read: Option<unsafe extern "C" fn(*mut c_void, c_uint, *mut c_uint) -> c_int>,
    pub reg_write: Option<unsafe extern "C" fn(*mut c_void, c_uint, c_uint) -> c_int>,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amic_micbias {
    AMIC_MICBIAS_VAMIC1 = 0,
    AMIC_MICBIAS_VAMIC2 = 1,
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amic_type {
    AMIC_TYPE_DIFFERENTIAL = 0,
    AMIC_TYPE_SINGLE_ENDED = 1,
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ear_cm_voltage {
    EAR_CMV_UNKNOWN = -1,
    EAR_CMV_0_95V = 0,
    EAR_CMV_1_10V = 1,
    EAR_CMV_1_27V = 2,
    EAR_CMV_1_58V = 3,
}
#[repr(C)]
pub struct amic_settings {
    pub mic1a_micbias: amic_micbias,
    pub mic1b_micbias: amic_micbias,
    pub mic2_micbias: amic_micbias,
    pub mic1_type: amic_type,
    pub mic2_type: amic_type,
}
#[repr(C)]
pub struct ab8500_codec_platform_data {
    pub amics: amic_settings,
    pub ear_cmv: ear_cm_voltage,
}

/* Macrocell value definitions */
const CLK_32K_OUT2_DISABLE: c_uint = 0x01;
const INACTIVE_RESET_AUDIO: c_uint = 0x02;
const ENABLE_AUDIO_CLK_TO_AUDIO_BLK: c_uint = 0x10;
const ENABLE_VINTCORE12_SUPPLY: c_uint = 0x04;
const GPIO27_DIR_OUTPUT: c_uint = 0x04;
const GPIO29_DIR_OUTPUT: c_uint = 0x10;
const GPIO31_DIR_OUTPUT: c_uint = 0x40;

/* Macrocell register definitions */
const AB8500_GPIO_DIR4_REG: c_uint = 0x13; /* Bank AB8500_MISC */

/* Nr of FIR/IIR-coeff banks in ANC-block */
const AB8500_NR_OF_ANC_COEFF_BANKS: c_uint = 2;

/* Minimum duration to keep ANC IIR Init bit high or
 * low before proceeding with the configuration sequence
 */
const AB8500_ANC_SM_DELAY: c_uint = 2000;

/* Sidetone states */
static enum_sid_state: [*const c_char; 3] = [c"Unconfigured".as_ptr(), c"Apply FIR".as_ptr(), c"FIR is configured".as_ptr()];

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum sid_state {
    SID_UNCONFIGURED = 0,
    SID_APPLY_FIR = 1,
    SID_FIR_CONFIGURED = 2,
}

/* Private data for AB8500 device-driver */
#[repr(C)]
struct ab8500_codec_drvdata {
    regmap: *mut regmap,
    ctrl_lock: mutex,

    /* Sidetone */
    sid_status: sid_state,
}

unsafe fn amic_micbias_str(micbias: amic_micbias) -> *const c_char {
    match micbias {
        amic_micbias::AMIC_MICBIAS_VAMIC1 => c"VAMIC1".as_ptr(),
        amic_micbias::AMIC_MICBIAS_VAMIC2 => c"VAMIC2".as_ptr(),
    }
}

unsafe fn amic_type_str(type_: amic_type) -> *const c_char {
    match type_ {
        amic_type::AMIC_TYPE_DIFFERENTIAL => c"DIFFERENTIAL".as_ptr(),
        amic_type::AMIC_TYPE_SINGLE_ENDED => c"SINGLE ENDED".as_ptr(),
    }
}

unsafe extern "C" {
    static mut AB8500_AUDIO: c_uint;
    static mut AB8500_MISC: c_uint;
    static mut AB8500_STW4500CTRL3: c_uint;
    static mut AB8500_STW4500CTRL3_CLK32KOUT2DIS: c_uint;
    static mut AB8500_STW4500CTRL3_RESETAUDN: c_uint;
    static mut AB8500_SUPPORTED_RATE: c_uint;
    static mut AB8500_SUPPORTED_FMT: u64;

    fn abx500_get_register_interruptible(
        dev: *mut device,
        bank: c_uint,
        reg: c_uint,
        value: *mut u8,
    ) -> c_int;
    fn abx500_set_register_interruptible(
        dev: *mut device,
        bank: c_uint,
        reg: c_uint,
        value: c_uint,
    ) -> c_int;
    fn ab8500_sysctrl_write(reg: c_uint, mask: c_uint, value: c_uint) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, value: c_uint);
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        value: c_uint,
    ) -> c_int;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init(
        dev: *mut device,
        bus: *const c_void,
        bus_context: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut u32) -> c_int;
    fn hweight32(w: c_uint) -> c_uint;
    fn ffs(x: c_uint) -> c_uint;
    fn fls(x: c_uint) -> c_uint;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
}

const EIO: c_int = 5;
const EPERM: c_int = 1;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const SND_SOC_NOPM: c_uint = !0;
const AB8500_MASK_NONE: c_uint = 0;

unsafe fn BIT(n: c_uint) -> c_uint {
    1u32.wrapping_shl(n)
}

/* External register/bit constants from ab8500-codec.h and sound headers are
 * referenced by their original names below. Static macro-expanded ASoC tables
 * (SOC_ENUM*, SND_SOC_DAPM_*, SOC_* controls, TLV declarations) are preserved
 * in structured comments because their Rust item layout depends on external
 * kernel macro expansions not present in this isolated file.
 */

/*
Controls - DAPM

Earpiece:
enum_ear_lineout_source = ["Headset Left", "Speaker Left"]
SOC_ENUM_SINGLE_DECL(dapm_enum_ear_lineout_source, AB8500_DMICFILTCONF,
        AB8500_DMICFILTCONF_DA3TOEAR, enum_ear_lineout_source)
dapm_ear_lineout_source =
        SOC_DAPM_ENUM("Earpiece or LineOut Mono Source", dapm_enum_ear_lineout_source)

LineOut:
enum_lineout_source = ["Mono Path", "Stereo Path"]
SOC_ENUM_DOUBLE_DECL(dapm_enum_lineout_source, AB8500_ANACONF5,
        AB8500_ANACONF5_HSLDACTOLOL, AB8500_ANACONF5_HSRDACTOLOR, enum_lineout_source)
dapm_lineout_source[] = { SOC_DAPM_ENUM("LineOut Source", dapm_enum_lineout_source) }

Handsfree:
enum_HFx_sel = ["Audio Path", "ANC"]
SOC_ENUM_SINGLE_DECL(dapm_enum_HFl_sel, AB8500_DIGMULTCONF2, AB8500_DIGMULTCONF2_HFLSEL, enum_HFx_sel)
dapm_HFl_select[] = { SOC_DAPM_ENUM("Speaker Left Source", dapm_enum_HFl_sel) }
SOC_ENUM_SINGLE_DECL(dapm_enum_HFr_sel, AB8500_DIGMULTCONF2, AB8500_DIGMULTCONF2_HFRSEL, enum_HFx_sel)
dapm_HFr_select[] = { SOC_DAPM_ENUM("Speaker Right Source", dapm_enum_HFr_sel) }

Mic/LineIn/ANC/Sidetone/Vibra selector declarations:
enum_mic1ab_sel = ["Mic 1b", "Mic 1a"]
enum_ad3_sel = ["Mic 1", "DMic 3"]
enum_ad6_sel = ["Mic 1", "DMic 6"]
enum_ad5_sel = ["Mic 2", "DMic 5"]
enum_ad1_sel = ["LineIn Left", "DMic 1"]
enum_mic2lr_sel = ["Mic 2", "LineIn Right"]
enum_ad2_sel = ["LineIn Right", "DMic 2"]
enum_anc_in_sel = ["Mic 1 / DMic 6", "Mic 2 / DMic 5"]
enum_stfir1_in_sel = ["LineIn Left", "LineIn Right", "Mic 1", "Headset Left"]
enum_stfir2_in_sel = ["LineIn Right", "Mic 1", "DMic 4", "Headset Right"]
enum_pwm2vibx = ["Audio Path", "PWM Generator"]

ab8500_dapm_widgets[] contains, in source order, all SND_SOC_DAPM_CLOCK_SUPPLY,
REGULATOR_SUPPLY, SUPPLY, INPUT, ADC, DAC, OUTPUT, AIF_IN, AIF_OUT, PGA, MIXER,
MUX, and SWITCH entries from the C source lines defining clocks, regulators,
main DA/AD, headset, lineout, earpiece, handsfree, vibrator, Mic 1, Mic 2,
LineIn, HD capture, digital microphones, ANC, and sidetone paths.
*/

macro_rules! route {
    ($sink:literal, NULL, $source:literal) => {
        snd_soc_dapm_route { sink: c$sink.as_ptr(), control: ptr::null(), source: c$source.as_ptr() }
    };
    ($sink:literal, $control:literal, $source:literal) => {
        snd_soc_dapm_route { sink: c$sink.as_ptr(), control: c$control.as_ptr(), source: c$source.as_ptr() }
    };
}

/* DAPM-routes */
static ab8500_dapm_routes: &[snd_soc_dapm_route] = &[
    route!("Main Supply", NULL, "V-AUD"),
    route!("Main Supply", NULL, "audioclk"),
    route!("Main Supply", NULL, "Audio Power"),
    route!("Main Supply", NULL, "Audio Analog Power"),
    route!("DAC", NULL, "ab8500_0p"),
    route!("DAC", NULL, "Main Supply"),
    route!("ADC", NULL, "ab8500_0c"),
    route!("ADC", NULL, "Main Supply"),
    route!("ANC Configure Input", NULL, "Main Supply"),
    route!("ANC Configure Output", NULL, "ANC Configure Input"),
    route!("ADC", NULL, "ADC Input"),
    route!("DAC Output", NULL, "DAC"),
    route!("DA_IN1", NULL, "ab8500_0p"),
    route!("DA_IN1", NULL, "Charge Pump"),
    route!("DA_IN2", NULL, "ab8500_0p"),
    route!("DA_IN2", NULL, "Charge Pump"),
    route!("DA1 Enable", NULL, "DA_IN1"),
    route!("DA2 Enable", NULL, "DA_IN2"),
    route!("HSL Digital Volume", NULL, "DA1 Enable"),
    route!("HSR Digital Volume", NULL, "DA2 Enable"),
    route!("HSL DAC", NULL, "HSL Digital Volume"),
    route!("HSR DAC", NULL, "HSR Digital Volume"),
    route!("HSL DAC Mute", NULL, "HSL DAC"),
    route!("HSR DAC Mute", NULL, "HSR DAC"),
    route!("HSL DAC Driver", NULL, "HSL DAC Mute"),
    route!("HSR DAC Driver", NULL, "HSR DAC Mute"),
    route!("HSL Mute", NULL, "HSL DAC Driver"),
    route!("HSR Mute", NULL, "HSR DAC Driver"),
    route!("HSL Enable", NULL, "HSL Mute"),
    route!("HSR Enable", NULL, "HSR Mute"),
    route!("HSL Volume", NULL, "HSL Enable"),
    route!("HSR Volume", NULL, "HSR Enable"),
    route!("Headset Left", NULL, "HSL Volume"),
    route!("Headset Right", NULL, "HSR Volume"),
    route!("DA_IN3", NULL, "ab8500_0p"),
    route!("DA3 Channel Volume", NULL, "DA_IN3"),
    route!("DA_IN4", NULL, "ab8500_0p"),
    route!("DA4 Channel Volume", NULL, "DA_IN4"),
    route!("Speaker Left Source", "Audio Path", "DA3 Channel Volume"),
    route!("Speaker Right Source", "Audio Path", "DA4 Channel Volume"),
    route!("DA3 or ANC path to HfL", NULL, "Speaker Left Source"),
    route!("DA4 or ANC path to HfR", NULL, "Speaker Right Source"),
    route!("HFL DAC", NULL, "DA3 or ANC path to HfL"),
    route!("HFR DAC", NULL, "DA4 or ANC path to HfR"),
    route!("HFL Enable", NULL, "HFL DAC"),
    route!("HFR Enable", NULL, "HFR DAC"),
    route!("Speaker Left", NULL, "HFL Enable"),
    route!("Speaker Right", NULL, "HFR Enable"),
    route!("Earpiece or LineOut Mono Source", "Headset Left", "HSL Digital Volume"),
    route!("Earpiece or LineOut Mono Source", "Speaker Left", "DA3 or ANC path to HfL"),
    route!("EAR DAC", NULL, "Earpiece or LineOut Mono Source"),
    route!("EAR Mute", NULL, "EAR DAC"),
    route!("EAR Enable", NULL, "EAR Mute"),
    route!("Earpiece", NULL, "EAR Enable"),
    route!("LineOut Source", "Stereo Path", "HSL DAC Driver"),
    route!("LineOut Source", "Stereo Path", "HSR DAC Driver"),
    route!("LineOut Source", "Mono Path", "EAR DAC"),
    route!("LOL Disable HFL", NULL, "LineOut Source"),
    route!("LOR Disable HFR", NULL, "LineOut Source"),
    route!("LOL Enable", NULL, "LOL Disable HFL"),
    route!("LOR Enable", NULL, "LOR Disable HFR"),
    route!("LineOut Left", NULL, "LOL Enable"),
    route!("LineOut Right", NULL, "LOR Enable"),
    route!("DA_IN5", NULL, "ab8500_0p"),
    route!("DA5 Channel Volume", NULL, "DA_IN5"),
    route!("DA_IN6", NULL, "ab8500_0p"),
    route!("DA6 Channel Volume", NULL, "DA_IN6"),
    route!("VIB1 DAC", NULL, "DA5 Channel Volume"),
    route!("VIB2 DAC", NULL, "DA6 Channel Volume"),
    route!("Vibra 1 Controller", "Audio Path", "VIB1 DAC"),
    route!("Vibra 2 Controller", "Audio Path", "VIB2 DAC"),
    route!("Vibra 1 Controller", "PWM Generator", "PWMGEN1"),
    route!("Vibra 2 Controller", "PWM Generator", "PWMGEN2"),
    route!("VIB1 Enable", NULL, "Vibra 1 Controller"),
    route!("VIB2 Enable", NULL, "Vibra 2 Controller"),
    route!("Vibra 1", NULL, "VIB1 Enable"),
    route!("Vibra 2", NULL, "VIB2 Enable"),
    route!("MIC2 V-AMICx Enable", NULL, "Mic 2"),
    route!("LINL Mute", NULL, "LineIn Left"),
    route!("LINR Mute", NULL, "LineIn Right"),
    route!("LINL Enable", NULL, "LINL Mute"),
    route!("LINR Enable", NULL, "LINR Mute"),
    route!("Mic 2 or LINR Select", "LineIn Right", "LINR Enable"),
    route!("Mic 2 or LINR Select", "Mic 2", "MIC2 V-AMICx Enable"),
    route!("LINL ADC", NULL, "LINL Enable"),
    route!("LINR ADC", NULL, "Mic 2 or LINR Select"),
    route!("AD1 Source Select", "LineIn Left", "LINL ADC"),
    route!("AD2 Source Select", "LineIn Right", "LINR ADC"),
    route!("AD1 Channel Volume", NULL, "AD1 Source Select"),
    route!("AD2 Channel Volume", NULL, "AD2 Source Select"),
    route!("AD12 Enable", NULL, "AD1 Channel Volume"),
    route!("AD12 Enable", NULL, "AD2 Channel Volume"),
    route!("AD_OUT1", NULL, "ab8500_0c"),
    route!("AD_OUT1", NULL, "AD12 Enable"),
    route!("AD_OUT2", NULL, "ab8500_0c"),
    route!("AD_OUT2", NULL, "AD12 Enable"),
    route!("MIC1 Mute", NULL, "Mic 1"),
    route!("MIC1A V-AMICx Enable", NULL, "MIC1 Mute"),
    route!("MIC1B V-AMICx Enable", NULL, "MIC1 Mute"),
    route!("Mic 1a or 1b Select", "Mic 1a", "MIC1A V-AMICx Enable"),
    route!("Mic 1a or 1b Select", "Mic 1b", "MIC1B V-AMICx Enable"),
    route!("MIC1 ADC", NULL, "Mic 1a or 1b Select"),
    route!("AD3 Source Select", "Mic 1", "MIC1 ADC"),
    route!("AD3 Channel Volume", NULL, "AD3 Source Select"),
    route!("AD3 Enable", NULL, "AD3 Channel Volume"),
    route!("AD_OUT3", NULL, "ab8500_0c"),
    route!("AD_OUT3", NULL, "AD3 Enable"),
    route!("AD5 Source Select", "Mic 2", "LINR ADC"),
    route!("AD6 Source Select", "Mic 1", "MIC1 ADC"),
    route!("AD5 Channel Volume", NULL, "AD5 Source Select"),
    route!("AD6 Channel Volume", NULL, "AD6 Source Select"),
    route!("AD57 Enable", NULL, "AD5 Channel Volume"),
    route!("AD68 Enable", NULL, "AD6 Channel Volume"),
    route!("AD_OUT57", NULL, "ab8500_0c"),
    route!("AD_OUT57", NULL, "AD57 Enable"),
    route!("AD_OUT68", NULL, "ab8500_0c"),
    route!("AD_OUT68", NULL, "AD68 Enable"),
    route!("DMic 1", NULL, "V-DMIC"),
    route!("DMic 2", NULL, "V-DMIC"),
    route!("DMic 3", NULL, "V-DMIC"),
    route!("DMic 4", NULL, "V-DMIC"),
    route!("DMic 5", NULL, "V-DMIC"),
    route!("DMic 6", NULL, "V-DMIC"),
    route!("AD1 Source Select", NULL, "DMic 1"),
    route!("AD2 Source Select", NULL, "DMic 2"),
    route!("AD3 Source Select", NULL, "DMic 3"),
    route!("AD5 Source Select", NULL, "DMic 5"),
    route!("AD6 Source Select", NULL, "DMic 6"),
    route!("AD4 Channel Volume", NULL, "DMic 4"),
    route!("AD4 Enable", NULL, "AD4 Channel Volume"),
    route!("AD_OUT4", NULL, "ab8500_0c"),
    route!("AD_OUT4", NULL, "AD4 Enable"),
    route!("LINL to HSL Volume", NULL, "LINL Enable"),
    route!("LINR to HSR Volume", NULL, "LINR Enable"),
    route!("HSL DAC Driver", NULL, "LINL to HSL Volume"),
    route!("HSR DAC Driver", NULL, "LINR to HSR Volume"),
    route!("ANC Source", "Mic 2 / DMic 5", "AD5 Channel Volume"),
    route!("ANC Source", "Mic 1 / DMic 6", "AD6 Channel Volume"),
    route!("ANC", "Switch", "ANC Source"),
    route!("Speaker Left Source", "ANC", "ANC"),
    route!("Speaker Right Source", "ANC", "ANC"),
    route!("ANC to Earpiece", "Switch", "ANC"),
    route!("HSL Digital Volume", NULL, "ANC to Earpiece"),
    route!("Sidetone Left Source", "LineIn Left", "AD12 Enable"),
    route!("Sidetone Left Source", "LineIn Right", "AD12 Enable"),
    route!("Sidetone Left Source", "Mic 1", "AD3 Enable"),
    route!("Sidetone Left Source", "Headset Left", "DA_IN1"),
    route!("Sidetone Right Source", "LineIn Right", "AD12 Enable"),
    route!("Sidetone Right Source", "Mic 1", "AD3 Enable"),
    route!("Sidetone Right Source", "DMic 4", "AD4 Enable"),
    route!("Sidetone Right Source", "Headset Right", "DA_IN2"),
    route!("STFIR1 Control", NULL, "Sidetone Left Source"),
    route!("STFIR2 Control", NULL, "Sidetone Right Source"),
    route!("STFIR1 Volume", NULL, "STFIR1 Control"),
    route!("STFIR2 Volume", NULL, "STFIR2 Control"),
    route!("DA1 Enable", NULL, "STFIR1 Volume"),
    route!("DA2 Enable", NULL, "STFIR2 Volume"),
];

static ab8500_dapm_routes_mic1a_vamicx: &[snd_soc_dapm_route] = &[
    route!("MIC1A V-AMICx Enable", NULL, "V-AMIC1"),
    route!("MIC1A V-AMICx Enable", NULL, "V-AMIC2"),
];
static ab8500_dapm_routes_mic1b_vamicx: &[snd_soc_dapm_route] = &[
    route!("MIC1B V-AMICx Enable", NULL, "V-AMIC1"),
    route!("MIC1B V-AMICx Enable", NULL, "V-AMIC2"),
];
static ab8500_dapm_routes_mic2_vamicx: &[snd_soc_dapm_route] = &[
    route!("MIC2 V-AMICx Enable", NULL, "V-AMIC1"),
    route!("MIC2 V-AMICx Enable", NULL, "V-AMIC2"),
];

/* Read a register from the audio-bank of AB8500 */
unsafe extern "C" fn ab8500_codec_read_reg(context: *mut c_void, reg: c_uint, value: *mut c_uint) -> c_int {
    let dev = context as *mut device;
    let mut value8: u8 = 0;
    let status = abx500_get_register_interruptible(dev, AB8500_AUDIO, reg, &mut value8);
    *value = value8 as c_uint;
    status
}

/* Write to a register in the audio-bank of AB8500 */
unsafe extern "C" fn ab8500_codec_write_reg(context: *mut c_void, reg: c_uint, value: c_uint) -> c_int {
    let dev = context as *mut device;
    abx500_set_register_interruptible(dev, AB8500_AUDIO, reg, value)
}

static ab8500_codec_regmap: regmap_config = regmap_config {
    reg_read: Some(ab8500_codec_read_reg),
    reg_write: Some(ab8500_codec_write_reg),
};

unsafe extern "C" fn sid_status_control_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let drvdata = dev_get_drvdata((*component).dev) as *mut ab8500_codec_drvdata;

    mutex_lock(&mut (*drvdata).ctrl_lock);
    (*ucontrol).value.enumerated.item[0] = (*drvdata).sid_status as c_uint;
    mutex_unlock(&mut (*drvdata).ctrl_lock);

    0
}

/* Write sidetone FIR-coefficients configuration sequence */
unsafe extern "C" fn sid_status_control_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let drvdata = dev_get_drvdata((*component).dev) as *mut ab8500_codec_drvdata;
    let mut status: c_int = 1;

    dev_dbg((*component).dev, c"%s: Enter\n".as_ptr(), c"sid_status_control_put".as_ptr());

    if (*ucontrol).value.enumerated.item[0] != sid_state::SID_APPLY_FIR as c_uint {
        dev_err(
            (*component).dev,
            c"%s: ERROR: This control supports '%s' only!\n".as_ptr(),
            c"sid_status_control_put".as_ptr(),
            enum_sid_state[sid_state::SID_APPLY_FIR as usize],
        );
        return -EIO;
    }

    mutex_lock(&mut (*drvdata).ctrl_lock);

    let sidconf = snd_soc_component_read(component, AB8500_SIDFIRCONF);
    if (sidconf & BIT(AB8500_SIDFIRCONF_FIRSIDBUSY)) != 0 {
        if (sidconf & BIT(AB8500_SIDFIRCONF_ENFIRSIDS)) == 0 {
            dev_err((*component).dev, c"%s: Sidetone busy while off!\n".as_ptr(), c"sid_status_control_put".as_ptr());
            status = -EPERM;
        } else {
            status = -EBUSY;
        }
        dev_dbg((*component).dev, c"%s: Exit\n".as_ptr(), c"sid_status_control_put".as_ptr());
        mutex_unlock(&mut (*drvdata).ctrl_lock);
        return status;
    }

    snd_soc_component_write(component, AB8500_SIDFIRADR, 0);

    let mut param: c_uint = 0;
    while param < AB8500_SID_FIR_COEFFS {
        snd_soc_component_write(component, AB8500_SIDFIRCOEF1, 0);
        snd_soc_component_write(component, AB8500_SIDFIRCOEF2, 0);
        param += 1;
    }

    snd_soc_component_update_bits(
        component,
        AB8500_SIDFIRADR,
        BIT(AB8500_SIDFIRADR_FIRSIDSET),
        BIT(AB8500_SIDFIRADR_FIRSIDSET),
    );
    snd_soc_component_update_bits(component, AB8500_SIDFIRADR, BIT(AB8500_SIDFIRADR_FIRSIDSET), 0);

    (*drvdata).sid_status = sid_state::SID_FIR_CONFIGURED;

    dev_dbg((*component).dev, c"%s: Exit\n".as_ptr(), c"sid_status_control_put".as_ptr());
    mutex_unlock(&mut (*drvdata).ctrl_lock);

    status
}

/*
Controls - Non-DAPM ASoC

DECLARE_TLV_DB_SCALE(adx_dig_gain_tlv, -3200, 100, 1)  // -32dB = Mute
DECLARE_TLV_DB_SCALE(dax_dig_gain_tlv, -6300, 100, 1)  // -63dB = Mute
DECLARE_TLV_DB_SCALE(hs_ear_dig_gain_tlv, -100, 100, 1) // -1dB = Mute
DECLARE_TLV_DB_RANGE(hs_gain_tlv,
        0, 3, TLV_DB_SCALE_ITEM(-3200, 400, 0),
        4, 15, TLV_DB_SCALE_ITEM(-1800, 200, 0))
DECLARE_TLV_DB_SCALE(mic_gain_tlv, 0, 100, 0)
DECLARE_TLV_DB_SCALE(lin_gain_tlv, -1000, 200, 0)
DECLARE_TLV_DB_SCALE(lin2hs_gain_tlv, -3800, 200, 1) // -38dB = Mute

Enums:
enum_hsfadspeed = ["2ms", "0.5ms", "10.6ms", "5ms"]
enum_envdetthre = ["250mV", "300mV", "350mV", "400mV", "450mV", "500mV", "550mV", "600mV",
                  "650mV", "700mV", "750mV", "800mV", "850mV", "900mV", "950mV", "1.00V"]
enum_envdettime = ["26.6us", "53.2us", "106us", "213us", "426us", "851us", "1.70ms", "3.40ms",
                  "6.81ms", "13.6ms", "27.2ms", "54.5ms", "109ms", "218ms", "436ms", "872ms"]
enum_sinc31 = ["Sinc 3", "Sinc 1"]
enum_fadespeed = ["1ms", "4ms", "8ms", "16ms"]
enum_lowpow = ["Normal", "Low Power"]
enum_av_mode = ["Audio", "Voice"]
enum_da2hslr = ["Sidetone", "Audio Path"]
enum_sinc53 = ["Sinc 5", "Sinc 3"]
enum_da_from_slot_map = ["SLOT0".."SLOT31"]
enum_ad_to_slot_map = ["AD_OUT1", "AD_OUT2", "AD_OUT3", "AD_OUT4", "AD_OUT5", "AD_OUT6", "AD_OUT7", "AD_OUT8",
                       "zeroes", "zeroes", "zeroes", "zeroes", "tristate", "tristate", "tristate", "tristate"]
enum_mask = ["Unmasked", "Masked"]
enum_bitclk0 = ["19_2_MHz", "38_4_MHz"]
enum_slavemaster = ["Slave", "Master"]
SOC_ENUM_SINGLE_EXT_DECL(soc_enum_sidstate, enum_sid_state)

ab8500_ctrls[] preserves every SOC_ENUM, SOC_SINGLE, SOC_DOUBLE, SOC_*_TLV,
SOC_SINGLE_XR_SX, SOC_ENUM_EXT, and SOC_SINGLE_STROBE control from the source,
in order: charge pump, headset, earpiece, handsfree, vibra, ClassD, mic, linein,
DMic, digital gains, analog loopback, DA slot map, AD slot map, AD loopback,
burst FIFO, ANC, and sidetone controls.
*/

unsafe extern "C" fn ab8500_audio_init_audioblock(component: *mut snd_soc_component) -> c_int {
    dev_dbg((*component).dev, c"%s: Enter.\n".as_ptr(), c"ab8500_audio_init_audioblock".as_ptr());

    /* Reset audio-registers and disable 32kHz-clock output 2 */
    let status = ab8500_sysctrl_write(
        AB8500_STW4500CTRL3,
        AB8500_STW4500CTRL3_CLK32KOUT2DIS | AB8500_STW4500CTRL3_RESETAUDN,
        AB8500_STW4500CTRL3_RESETAUDN,
    );
    if status < 0 {
        return status;
    }

    0
}

unsafe extern "C" fn ab8500_audio_setup_mics(
    component: *mut snd_soc_component,
    amics: *mut amic_settings,
) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let mut value8: u8 = 0;
    let mut status: c_int;
    let mut route: *const snd_soc_dapm_route;

    dev_dbg((*component).dev, c"%s: Enter.\n".as_ptr(), c"ab8500_audio_setup_mics".as_ptr());

    /* Set DMic-clocks to outputs */
    status = abx500_get_register_interruptible((*component).dev, AB8500_MISC, AB8500_GPIO_DIR4_REG, &mut value8);
    if status < 0 {
        return status;
    }
    let value = value8 as c_uint | GPIO27_DIR_OUTPUT | GPIO29_DIR_OUTPUT | GPIO31_DIR_OUTPUT;
    status = abx500_set_register_interruptible((*component).dev, AB8500_MISC, AB8500_GPIO_DIR4_REG, value);
    if status < 0 {
        return status;
    }

    /* Attach regulators to AMic DAPM-paths */
    dev_dbg((*component).dev, c"%s: Mic 1a regulator: %s\n".as_ptr(), c"ab8500_audio_setup_mics".as_ptr(), amic_micbias_str((*amics).mic1a_micbias));
    route = &ab8500_dapm_routes_mic1a_vamicx[(*amics).mic1a_micbias as usize];
    status = snd_soc_dapm_add_routes(dapm, route, 1);
    dev_dbg((*component).dev, c"%s: Mic 1b regulator: %s\n".as_ptr(), c"ab8500_audio_setup_mics".as_ptr(), amic_micbias_str((*amics).mic1b_micbias));
    route = &ab8500_dapm_routes_mic1b_vamicx[(*amics).mic1b_micbias as usize];
    status |= snd_soc_dapm_add_routes(dapm, route, 1);
    dev_dbg((*component).dev, c"%s: Mic 2 regulator: %s\n".as_ptr(), c"ab8500_audio_setup_mics".as_ptr(), amic_micbias_str((*amics).mic2_micbias));
    route = &ab8500_dapm_routes_mic2_vamicx[(*amics).mic2_micbias as usize];
    status |= snd_soc_dapm_add_routes(dapm, route, 1);
    if status < 0 {
        dev_err((*component).dev, c"%s: Failed to add AMic-regulator DAPM-routes (%d).\n".as_ptr(), c"ab8500_audio_setup_mics".as_ptr(), status);
        return status;
    }

    /* Set AMic-configuration */
    dev_dbg((*component).dev, c"%s: Mic 1 mic-type: %s\n".as_ptr(), c"ab8500_audio_setup_mics".as_ptr(), amic_type_str((*amics).mic1_type));
    snd_soc_component_update_bits(
        component,
        AB8500_ANAGAIN1,
        AB8500_ANAGAINX_ENSEMICX,
        if (*amics).mic1_type == amic_type::AMIC_TYPE_DIFFERENTIAL { 0 } else { AB8500_ANAGAINX_ENSEMICX },
    );
    dev_dbg((*component).dev, c"%s: Mic 2 mic-type: %s\n".as_ptr(), c"ab8500_audio_setup_mics".as_ptr(), amic_type_str((*amics).mic2_type));
    snd_soc_component_update_bits(
        component,
        AB8500_ANAGAIN2,
        AB8500_ANAGAINX_ENSEMICX,
        if (*amics).mic2_type == amic_type::AMIC_TYPE_DIFFERENTIAL { 0 } else { AB8500_ANAGAINX_ENSEMICX },
    );

    0
}

unsafe extern "C" fn ab8500_audio_set_ear_cmv(
    component: *mut snd_soc_component,
    ear_cmv: ear_cm_voltage,
) -> c_int {
    let cmv_str: *const c_char;

    match ear_cmv {
        ear_cm_voltage::EAR_CMV_0_95V => cmv_str = c"0.95V".as_ptr(),
        ear_cm_voltage::EAR_CMV_1_10V => cmv_str = c"1.10V".as_ptr(),
        ear_cm_voltage::EAR_CMV_1_27V => cmv_str = c"1.27V".as_ptr(),
        ear_cm_voltage::EAR_CMV_1_58V => cmv_str = c"1.58V".as_ptr(),
        _ => {
            dev_err((*component).dev, c"%s: Unknown earpiece CM-voltage (%d)!\n".as_ptr(), c"ab8500_audio_set_ear_cmv".as_ptr(), ear_cmv as c_int);
            return -EINVAL;
        }
    }
    dev_dbg((*component).dev, c"%s: Earpiece CM-voltage: %s\n".as_ptr(), c"ab8500_audio_set_ear_cmv".as_ptr(), cmv_str);
    snd_soc_component_update_bits(component, AB8500_ANACONF1, AB8500_ANACONF1_EARSELCM, ear_cmv as c_uint);

    0
}

unsafe extern "C" fn ab8500_audio_set_bit_delay(dai: *mut snd_soc_dai, delay: c_uint) -> c_int {
    let component = (*dai).component;
    let mask = BIT(AB8500_DIGIFCONF2_IF0DEL);
    let mut val = 0;

    match delay {
        0 => {}
        1 => val |= BIT(AB8500_DIGIFCONF2_IF0DEL),
        _ => {
            dev_err((*(*dai).component).dev, c"%s: ERROR: Unsupported bit-delay (0x%x)!\n".as_ptr(), c"ab8500_audio_set_bit_delay".as_ptr(), delay);
            return -EINVAL;
        }
    }

    dev_dbg((*(*dai).component).dev, c"%s: IF0 Bit-delay: %d bits.\n".as_ptr(), c"ab8500_audio_set_bit_delay".as_ptr(), delay);
    snd_soc_component_update_bits(component, AB8500_DIGIFCONF2, mask, val);

    0
}

/* Gates clocking according format mask */
unsafe extern "C" fn ab8500_codec_set_dai_clock_gate(
    component: *mut snd_soc_component,
    fmt: c_uint,
) -> c_int {
    let mask = BIT(AB8500_DIGIFCONF1_ENMASTGEN) | BIT(AB8500_DIGIFCONF1_ENFSBITCLK0);
    let mut val = BIT(AB8500_DIGIFCONF1_ENMASTGEN);

    match fmt & SND_SOC_DAIFMT_CLOCK_MASK {
        SND_SOC_DAIFMT_CONT => {
            /* continuous clock */
            dev_dbg((*component).dev, c"%s: IF0 Clock is continuous.\n".as_ptr(), c"ab8500_codec_set_dai_clock_gate".as_ptr());
            val |= BIT(AB8500_DIGIFCONF1_ENFSBITCLK0);
        }
        SND_SOC_DAIFMT_GATED => {
            /* clock is gated */
            dev_dbg((*component).dev, c"%s: IF0 Clock is gated.\n".as_ptr(), c"ab8500_codec_set_dai_clock_gate".as_ptr());
        }
        _ => {
            dev_err((*component).dev, c"%s: ERROR: Unsupported clock mask (0x%x)!\n".as_ptr(), c"ab8500_codec_set_dai_clock_gate".as_ptr(), fmt & SND_SOC_DAIFMT_CLOCK_MASK);
            return -EINVAL;
        }
    }

    snd_soc_component_update_bits(component, AB8500_DIGIFCONF1, mask, val);
    0
}

unsafe extern "C" fn ab8500_codec_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let mut mask = BIT(AB8500_DIGIFCONF3_IF1DATOIF0AD)
        | BIT(AB8500_DIGIFCONF3_IF1CLKTOIF0CLK)
        | BIT(AB8500_DIGIFCONF3_IF0BFIFOEN)
        | BIT(AB8500_DIGIFCONF3_IF0MASTER);
    let mut val = 0;

    dev_dbg((*component).dev, c"%s: Enter (fmt = 0x%x)\n".as_ptr(), c"ab8500_codec_set_dai_fmt".as_ptr(), fmt);

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => {
            dev_dbg((*(*dai).component).dev, c"%s: IF0 Master-mode: AB8500 provider.\n".as_ptr(), c"ab8500_codec_set_dai_fmt".as_ptr());
            val |= BIT(AB8500_DIGIFCONF3_IF0MASTER);
        }
        SND_SOC_DAIFMT_CBC_CFC => {
            dev_dbg((*(*dai).component).dev, c"%s: IF0 Master-mode: AB8500 consumer.\n".as_ptr(), c"ab8500_codec_set_dai_fmt".as_ptr());
        }
        SND_SOC_DAIFMT_CBC_CFP | SND_SOC_DAIFMT_CBP_CFC => {
            dev_err((*(*dai).component).dev, c"%s: ERROR: The device is either a provider or a consumer.\n".as_ptr(), c"ab8500_codec_set_dai_fmt".as_ptr());
            dev_err((*(*dai).component).dev, c"%s: ERROR: Unsupporter clocking mask 0x%x\n".as_ptr(), c"ab8500_codec_set_dai_fmt".as_ptr(), fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK);
            return -EINVAL;
        }
        _ => {
            dev_err((*(*dai).component).dev, c"%s: ERROR: Unsupporter clocking mask 0x%x\n".as_ptr(), c"ab8500_codec_set_dai_fmt".as_ptr(), fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK);
            return -EINVAL;
        }
    }

    snd_soc_component_update_bits(component, AB8500_DIGIFCONF3, mask, val);

    /* Set clock gating */
    let status = ab8500_codec_set_dai_clock_gate(component, fmt);
    if status != 0 {
        dev_err((*(*dai).component).dev, c"%s: ERROR: Failed to set clock gate (%d).\n".as_ptr(), c"ab8500_codec_set_dai_fmt".as_ptr(), status);
        return status;
    }

    /* Setting data transfer format */
    mask = BIT(AB8500_DIGIFCONF2_IF0FORMAT0)
        | BIT(AB8500_DIGIFCONF2_IF0FORMAT1)
        | BIT(AB8500_DIGIFCONF2_FSYNC0P)
        | BIT(AB8500_DIGIFCONF2_BITCLK0P);
    val = 0;

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            /* I2S mode */
            dev_dbg((*(*dai).component).dev, c"%s: IF0 Protocol: I2S\n".as_ptr(), c"ab8500_codec_set_dai_fmt".as_ptr());
            val |= BIT(AB8500_DIGIFCONF2_IF0FORMAT1);
            ab8500_audio_set_bit_delay(dai, 0);
        }
        SND_SOC_DAIFMT_DSP_A => {
            /* L data MSB after FRM LRC */
            dev_dbg((*(*dai).component).dev, c"%s: IF0 Protocol: DSP A (TDM)\n".as_ptr(), c"ab8500_codec_set_dai_fmt".as_ptr());
            val |= BIT(AB8500_DIGIFCONF2_IF0FORMAT0);
            ab8500_audio_set_bit_delay(dai, 1);
        }
        SND_SOC_DAIFMT_DSP_B => {
            /* L data MSB during FRM LRC */
            dev_dbg((*(*dai).component).dev, c"%s: IF0 Protocol: DSP B (TDM)\n".as_ptr(), c"ab8500_codec_set_dai_fmt".as_ptr());
            val |= BIT(AB8500_DIGIFCONF2_IF0FORMAT0);
            ab8500_audio_set_bit_delay(dai, 0);
        }
        _ => {
            dev_err((*(*dai).component).dev, c"%s: ERROR: Unsupported format (0x%x)!\n".as_ptr(), c"ab8500_codec_set_dai_fmt".as_ptr(), fmt & SND_SOC_DAIFMT_FORMAT_MASK);
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {
            dev_dbg((*(*dai).component).dev, c"%s: IF0: Normal bit clock, normal frame\n".as_ptr(), c"ab8500_codec_set_dai_fmt".as_ptr());
        }
        SND_SOC_DAIFMT_NB_IF => {
            dev_dbg((*(*dai).component).dev, c"%s: IF0: Normal bit clock, inverted frame\n".as_ptr(), c"ab8500_codec_set_dai_fmt".as_ptr());
            val |= BIT(AB8500_DIGIFCONF2_FSYNC0P);
        }
        SND_SOC_DAIFMT_IB_NF => {
            dev_dbg((*(*dai).component).dev, c"%s: IF0: Inverted bit clock, normal frame\n".as_ptr(), c"ab8500_codec_set_dai_fmt".as_ptr());
            val |= BIT(AB8500_DIGIFCONF2_BITCLK0P);
        }
        SND_SOC_DAIFMT_IB_IF => {
            dev_dbg((*(*dai).component).dev, c"%s: IF0: Inverted bit clock, inverted frame\n".as_ptr(), c"ab8500_codec_set_dai_fmt".as_ptr());
            val |= BIT(AB8500_DIGIFCONF2_FSYNC0P);
            val |= BIT(AB8500_DIGIFCONF2_BITCLK0P);
        }
        _ => {
            dev_err((*(*dai).component).dev, c"%s: ERROR: Unsupported INV mask 0x%x\n".as_ptr(), c"ab8500_codec_set_dai_fmt".as_ptr(), fmt & SND_SOC_DAIFMT_INV_MASK);
            return -EINVAL;
        }
    }

    snd_soc_component_update_bits(component, AB8500_DIGIFCONF2, mask, val);
    0
}

unsafe extern "C" fn ab8500_codec_set_dai_tdm_slot(
    dai: *mut snd_soc_dai,
    mut tx_mask: c_uint,
    mut rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let component = (*dai).component;
    let mut mask = BIT(AB8500_DIGIFCONF2_IF0WL0) | BIT(AB8500_DIGIFCONF2_IF0WL1);
    let mut val = 0;
    let mut slot: c_uint;
    let mut slots_active: c_uint;

    match slot_width {
        16 => {}
        20 => val |= BIT(AB8500_DIGIFCONF2_IF0WL0),
        24 => val |= BIT(AB8500_DIGIFCONF2_IF0WL1),
        32 => val |= BIT(AB8500_DIGIFCONF2_IF0WL1) | BIT(AB8500_DIGIFCONF2_IF0WL0),
        _ => {
            dev_err((*(*dai).component).dev, c"%s: Unsupported slot-width 0x%x\n".as_ptr(), c"ab8500_codec_set_dai_tdm_slot".as_ptr(), slot_width);
            return -EINVAL;
        }
    }

    dev_dbg((*(*dai).component).dev, c"%s: IF0 slot-width: %d bits.\n".as_ptr(), c"ab8500_codec_set_dai_tdm_slot".as_ptr(), slot_width);
    snd_soc_component_update_bits(component, AB8500_DIGIFCONF2, mask, val);

    /* Setup TDM clocking according to slot count */
    dev_dbg((*(*dai).component).dev, c"%s: Slots, total: %d\n".as_ptr(), c"ab8500_codec_set_dai_tdm_slot".as_ptr(), slots);
    mask = BIT(AB8500_DIGIFCONF1_IF0BITCLKOS0) | BIT(AB8500_DIGIFCONF1_IF0BITCLKOS1);
    match slots {
        2 => val = AB8500_MASK_NONE,
        4 => val = BIT(AB8500_DIGIFCONF1_IF0BITCLKOS0),
        8 => val = BIT(AB8500_DIGIFCONF1_IF0BITCLKOS1),
        16 => val = BIT(AB8500_DIGIFCONF1_IF0BITCLKOS0) | BIT(AB8500_DIGIFCONF1_IF0BITCLKOS1),
        _ => {
            dev_err((*(*dai).component).dev, c"%s: ERROR: Unsupported number of slots (%d)!\n".as_ptr(), c"ab8500_codec_set_dai_tdm_slot".as_ptr(), slots);
            return -EINVAL;
        }
    }
    snd_soc_component_update_bits(component, AB8500_DIGIFCONF1, mask, val);

    /* Setup TDM DA according to active tx slots */
    if (tx_mask & !0xff) != 0 {
        return -EINVAL;
    }

    mask = AB8500_DASLOTCONFX_SLTODAX_MASK;
    tx_mask <<= AB8500_DA_DATA0_OFFSET;
    slots_active = hweight32(tx_mask);

    dev_dbg((*(*dai).component).dev, c"%s: Slots, active, TX: %d\n".as_ptr(), c"ab8500_codec_set_dai_tdm_slot".as_ptr(), slots_active);

    match slots_active {
        0 => {}
        1 => {
            slot = ffs(tx_mask);
            snd_soc_component_update_bits(component, AB8500_DASLOTCONF1, mask, slot);
            snd_soc_component_update_bits(component, AB8500_DASLOTCONF3, mask, slot);
            snd_soc_component_update_bits(component, AB8500_DASLOTCONF2, mask, slot);
            snd_soc_component_update_bits(component, AB8500_DASLOTCONF4, mask, slot);
        }
        2 => {
            slot = ffs(tx_mask);
            snd_soc_component_update_bits(component, AB8500_DASLOTCONF1, mask, slot);
            snd_soc_component_update_bits(component, AB8500_DASLOTCONF3, mask, slot);
            slot = fls(tx_mask);
            snd_soc_component_update_bits(component, AB8500_DASLOTCONF2, mask, slot);
            snd_soc_component_update_bits(component, AB8500_DASLOTCONF4, mask, slot);
        }
        8 => {
            dev_dbg((*(*dai).component).dev, c"%s: In 8-channel mode DA-from-slot mapping is set manually.".as_ptr(), c"ab8500_codec_set_dai_tdm_slot".as_ptr());
        }
        _ => {
            dev_err((*(*dai).component).dev, c"%s: Unsupported number of active TX-slots (%d)!\n".as_ptr(), c"ab8500_codec_set_dai_tdm_slot".as_ptr(), slots_active);
            return -EINVAL;
        }
    }

    /* Setup TDM AD according to active RX-slots */
    if (rx_mask & !0xff) != 0 {
        return -EINVAL;
    }

    rx_mask <<= AB8500_AD_DATA0_OFFSET;
    slots_active = hweight32(rx_mask);

    dev_dbg((*(*dai).component).dev, c"%s: Slots, active, RX: %d\n".as_ptr(), c"ab8500_codec_set_dai_tdm_slot".as_ptr(), slots_active);

    match slots_active {
        0 => {}
        1 => {
            slot = ffs(rx_mask);
            snd_soc_component_update_bits(
                component,
                AB8500_ADSLOTSEL(slot),
                AB8500_MASK_SLOT(slot),
                AB8500_ADSLOTSELX_AD_OUT_TO_SLOT(AB8500_AD_OUT3, slot),
            );
        }
        2 => {
            slot = ffs(rx_mask);
            snd_soc_component_update_bits(
                component,
                AB8500_ADSLOTSEL(slot),
                AB8500_MASK_SLOT(slot),
                AB8500_ADSLOTSELX_AD_OUT_TO_SLOT(AB8500_AD_OUT3, slot),
            );
            slot = fls(rx_mask);
            snd_soc_component_update_bits(
                component,
                AB8500_ADSLOTSEL(slot),
                AB8500_MASK_SLOT(slot),
                AB8500_ADSLOTSELX_AD_OUT_TO_SLOT(AB8500_AD_OUT2, slot),
            );
        }
        8 => {
            dev_dbg((*(*dai).component).dev, c"%s: In 8-channel mode AD-to-slot mapping is set manually.".as_ptr(), c"ab8500_codec_set_dai_tdm_slot".as_ptr());
        }
        _ => {
            dev_err((*(*dai).component).dev, c"%s: Unsupported number of active RX-slots (%d)!\n".as_ptr(), c"ab8500_codec_set_dai_tdm_slot".as_ptr(), slots_active);
            return -EINVAL;
        }
    }

    0
}

static ab8500_codec_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(ab8500_codec_set_dai_fmt),
    set_tdm_slot: Some(ab8500_codec_set_dai_tdm_slot),
};

static mut ab8500_codec_dai: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c"ab8500-codec-dai.0".as_ptr(),
        id: 0,
        playback: snd_soc_pcm_stream {
            stream_name: c"ab8500_0p".as_ptr(),
            channels_min: 1,
            channels_max: 8,
            rates: unsafe { AB8500_SUPPORTED_RATE },
            formats: unsafe { AB8500_SUPPORTED_FMT },
        },
        capture: snd_soc_pcm_stream {
            stream_name: ptr::null(),
            channels_min: 0,
            channels_max: 0,
            rates: 0,
            formats: 0,
        },
        ops: &ab8500_codec_ops,
        symmetric_rate: 1,
    },
    snd_soc_dai_driver {
        name: c"ab8500-codec-dai.1".as_ptr(),
        id: 1,
        playback: snd_soc_pcm_stream {
            stream_name: ptr::null(),
            channels_min: 0,
            channels_max: 0,
            rates: 0,
            formats: 0,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"ab8500_0c".as_ptr(),
            channels_min: 1,
            channels_max: 8,
            rates: unsafe { AB8500_SUPPORTED_RATE },
            formats: unsafe { AB8500_SUPPORTED_FMT },
        },
        ops: &ab8500_codec_ops,
        symmetric_rate: 1,
    },
];

unsafe extern "C" fn ab8500_codec_of_probe(
    dev: *mut device,
    np: *mut device_node,
    codec: *mut ab8500_codec_platform_data,
) {
    let mut value: u32 = 0;

    if of_property_read_bool(np, c"stericsson,amic1-type-single-ended".as_ptr()) {
        (*codec).amics.mic1_type = amic_type::AMIC_TYPE_SINGLE_ENDED;
    } else {
        (*codec).amics.mic1_type = amic_type::AMIC_TYPE_DIFFERENTIAL;
    }

    if of_property_read_bool(np, c"stericsson,amic2-type-single-ended".as_ptr()) {
        (*codec).amics.mic2_type = amic_type::AMIC_TYPE_SINGLE_ENDED;
    } else {
        (*codec).amics.mic2_type = amic_type::AMIC_TYPE_DIFFERENTIAL;
    }

    /* Has a non-standard Vamic been requested? */
    if of_property_read_bool(np, c"stericsson,amic1a-bias-vamic2".as_ptr()) {
        (*codec).amics.mic1a_micbias = amic_micbias::AMIC_MICBIAS_VAMIC2;
    } else {
        (*codec).amics.mic1a_micbias = amic_micbias::AMIC_MICBIAS_VAMIC1;
    }

    if of_property_read_bool(np, c"stericsson,amic1b-bias-vamic2".as_ptr()) {
        (*codec).amics.mic1b_micbias = amic_micbias::AMIC_MICBIAS_VAMIC2;
    } else {
        (*codec).amics.mic1b_micbias = amic_micbias::AMIC_MICBIAS_VAMIC1;
    }

    if of_property_read_bool(np, c"stericsson,amic2-bias-vamic1".as_ptr()) {
        (*codec).amics.mic2_micbias = amic_micbias::AMIC_MICBIAS_VAMIC1;
    } else {
        (*codec).amics.mic2_micbias = amic_micbias::AMIC_MICBIAS_VAMIC2;
    }

    if of_property_read_u32(np, c"stericsson,earpeice-cmv".as_ptr(), &mut value) == 0 {
        match value {
            950 => (*codec).ear_cmv = ear_cm_voltage::EAR_CMV_0_95V,
            1100 => (*codec).ear_cmv = ear_cm_voltage::EAR_CMV_1_10V,
            1270 => (*codec).ear_cmv = ear_cm_voltage::EAR_CMV_1_27V,
            1580 => (*codec).ear_cmv = ear_cm_voltage::EAR_CMV_1_58V,
            _ => {
                (*codec).ear_cmv = ear_cm_voltage::EAR_CMV_UNKNOWN;
                dev_err(dev, c"Unsuitable earpiece voltage found in DT\n".as_ptr());
            }
        }
    } else {
        dev_warn(dev, c"No earpiece voltage found in DT - using default\n".as_ptr());
        (*codec).ear_cmv = ear_cm_voltage::EAR_CMV_0_95V;
    }
}

unsafe extern "C" fn ab8500_codec_probe(component: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let dev = (*component).dev;
    let np = (*dev).of_node;
    let drvdata = dev_get_drvdata(dev) as *mut ab8500_codec_drvdata;
    let mut codec_pdata = core::mem::zeroed::<ab8500_codec_platform_data>();
    let mut status: c_int;

    dev_dbg(dev, c"%s: Enter.\n".as_ptr(), c"ab8500_codec_probe".as_ptr());

    ab8500_codec_of_probe(dev, np, &mut codec_pdata);

    status = ab8500_audio_setup_mics(component, &mut codec_pdata.amics);
    if status < 0 {
        pr_err(c"%s: Failed to setup mics (%d)!\n".as_ptr(), c"ab8500_codec_probe".as_ptr(), status);
        return status;
    }
    status = ab8500_audio_set_ear_cmv(component, codec_pdata.ear_cmv);
    if status < 0 {
        pr_err(c"%s: Failed to set earpiece CM-voltage (%d)!\n".as_ptr(), c"ab8500_codec_probe".as_ptr(), status);
        return status;
    }

    status = ab8500_audio_init_audioblock(component);
    if status < 0 {
        dev_err(dev, c"%s: failed to init audio-block (%d)!\n".as_ptr(), c"ab8500_codec_probe".as_ptr(), status);
        return status;
    }

    /* Override HW-defaults */
    snd_soc_component_write(component, AB8500_ANACONF5, BIT(AB8500_ANACONF5_HSAUTOEN));
    snd_soc_component_write(component, AB8500_SHORTCIRCONF, BIT(AB8500_SHORTCIRCONF_HSZCDDIS));

    snd_soc_dapm_disable_pin(dapm, c"ANC Configure Input".as_ptr());

    mutex_init(&mut (*drvdata).ctrl_lock);

    status
}

static mut ab8500_ctrls: [snd_kcontrol_new; 0] = [];
static ab8500_dapm_widgets: [snd_soc_dapm_widget; 0] = [];

static ab8500_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(ab8500_codec_probe),
    controls: unsafe { ab8500_ctrls.as_mut_ptr() },
    num_controls: 0, /* ARRAY_SIZE(ab8500_ctrls); macro-expanded controls preserved above. */
    dapm_widgets: ab8500_dapm_widgets.as_ptr(),
    num_dapm_widgets: 0, /* ARRAY_SIZE(ab8500_dapm_widgets); macro-expanded widgets preserved above. */
    dapm_routes: ab8500_dapm_routes.as_ptr(),
    num_dapm_routes: ab8500_dapm_routes.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn ab8500_codec_driver_probe(pdev: *mut platform_device) -> c_int {
    let mut status: c_int;

    dev_dbg(&mut (*pdev).dev, c"%s: Enter.\n".as_ptr(), c"ab8500_codec_driver_probe".as_ptr());

    /* Create driver private-data struct */
    let drvdata = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<ab8500_codec_drvdata>(),
        GFP_KERNEL,
    ) as *mut ab8500_codec_drvdata;
    if drvdata.is_null() {
        return -ENOMEM;
    }
    (*drvdata).sid_status = sid_state::SID_UNCONFIGURED;
    dev_set_drvdata(&mut (*pdev).dev, drvdata as *mut c_void);

    (*drvdata).regmap = devm_regmap_init(
        &mut (*pdev).dev,
        ptr::null(),
        &mut (*pdev).dev as *mut device as *mut c_void,
        &ab8500_codec_regmap,
    );
    if IS_ERR((*drvdata).regmap as *const c_void) {
        status = PTR_ERR((*drvdata).regmap as *const c_void);
        dev_err(&mut (*pdev).dev, c"%s: Failed to allocate regmap: %d\n".as_ptr(), c"ab8500_codec_driver_probe".as_ptr(), status);
        return status;
    }

    dev_dbg(&mut (*pdev).dev, c"%s: Register codec.\n".as_ptr(), c"ab8500_codec_driver_probe".as_ptr());
    status = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &ab8500_component_driver,
        ab8500_codec_dai.as_mut_ptr(),
        ab8500_codec_dai.len() as c_int,
    );
    if status < 0 {
        dev_err(&mut (*pdev).dev, c"%s: Error: Failed to register codec (%d).\n".as_ptr(), c"ab8500_codec_driver_probe".as_ptr(), status);
    }

    status
}

static ab8500_codec_platform_driver: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: c"ab8500-codec".as_ptr(),
    },
    probe: Some(ab8500_codec_driver_probe),
};

/* module_platform_driver(ab8500_codec_platform_driver); */
/* MODULE_DESCRIPTION("ASoC AB8500 codec driver"); */
/* MODULE_LICENSE("GPL v2"); */

unsafe extern "C" {
    static mut AB8500_SIDFIRCONF: c_uint;
    static mut AB8500_SIDFIRCONF_FIRSIDBUSY: c_uint;
    static mut AB8500_SIDFIRCONF_ENFIRSIDS: c_uint;
    static mut AB8500_SIDFIRADR: c_uint;
    static mut AB8500_SID_FIR_COEFFS: c_uint;
    static mut AB8500_SIDFIRCOEF1: c_uint;
    static mut AB8500_SIDFIRCOEF2: c_uint;
    static mut AB8500_SIDFIRADR_FIRSIDSET: c_uint;
    static mut AB8500_ANAGAIN1: c_uint;
    static mut AB8500_ANAGAIN2: c_uint;
    static mut AB8500_ANAGAINX_ENSEMICX: c_uint;
    static mut AB8500_ANACONF1: c_uint;
    static mut AB8500_ANACONF1_EARSELCM: c_uint;
    static mut AB8500_DIGIFCONF2: c_uint;
    static mut AB8500_DIGIFCONF2_IF0DEL: c_uint;
    static mut AB8500_DIGIFCONF1: c_uint;
    static mut AB8500_DIGIFCONF1_ENMASTGEN: c_uint;
    static mut AB8500_DIGIFCONF1_ENFSBITCLK0: c_uint;
    static mut SND_SOC_DAIFMT_CLOCK_MASK: c_uint;
    static mut SND_SOC_DAIFMT_CONT: c_uint;
    static mut SND_SOC_DAIFMT_GATED: c_uint;
    static mut AB8500_DIGIFCONF3: c_uint;
    static mut AB8500_DIGIFCONF3_IF1DATOIF0AD: c_uint;
    static mut AB8500_DIGIFCONF3_IF1CLKTOIF0CLK: c_uint;
    static mut AB8500_DIGIFCONF3_IF0BFIFOEN: c_uint;
    static mut AB8500_DIGIFCONF3_IF0MASTER: c_uint;
    static mut SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static mut SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static mut SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static mut SND_SOC_DAIFMT_CBC_CFP: c_uint;
    static mut SND_SOC_DAIFMT_CBP_CFC: c_uint;
    static mut AB8500_DIGIFCONF2_IF0FORMAT0: c_uint;
    static mut AB8500_DIGIFCONF2_IF0FORMAT1: c_uint;
    static mut AB8500_DIGIFCONF2_FSYNC0P: c_uint;
    static mut AB8500_DIGIFCONF2_BITCLK0P: c_uint;
    static mut SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static mut SND_SOC_DAIFMT_I2S: c_uint;
    static mut SND_SOC_DAIFMT_DSP_A: c_uint;
    static mut SND_SOC_DAIFMT_DSP_B: c_uint;
    static mut SND_SOC_DAIFMT_INV_MASK: c_uint;
    static mut SND_SOC_DAIFMT_NB_NF: c_uint;
    static mut SND_SOC_DAIFMT_NB_IF: c_uint;
    static mut SND_SOC_DAIFMT_IB_NF: c_uint;
    static mut SND_SOC_DAIFMT_IB_IF: c_uint;
    static mut AB8500_DIGIFCONF2_IF0WL0: c_uint;
    static mut AB8500_DIGIFCONF2_IF0WL1: c_uint;
    static mut AB8500_DIGIFCONF1_IF0BITCLKOS0: c_uint;
    static mut AB8500_DIGIFCONF1_IF0BITCLKOS1: c_uint;
    static mut AB8500_DASLOTCONFX_SLTODAX_MASK: c_uint;
    static mut AB8500_DA_DATA0_OFFSET: c_uint;
    static mut AB8500_DASLOTCONF1: c_uint;
    static mut AB8500_DASLOTCONF2: c_uint;
    static mut AB8500_DASLOTCONF3: c_uint;
    static mut AB8500_DASLOTCONF4: c_uint;
    static mut AB8500_AD_DATA0_OFFSET: c_uint;
    static mut AB8500_AD_OUT2: c_uint;
    static mut AB8500_AD_OUT3: c_uint;
    static mut AB8500_ANACONF5: c_uint;
    static mut AB8500_ANACONF5_HSAUTOEN: c_uint;
    static mut AB8500_SHORTCIRCONF: c_uint;
    static mut AB8500_SHORTCIRCONF_HSZCDDIS: c_uint;
    fn AB8500_ADSLOTSEL(slot: c_uint) -> c_uint;
    fn AB8500_MASK_SLOT(slot: c_uint) -> c_uint;
    fn AB8500_ADSLOTSELX_AD_OUT_TO_SLOT(ad_out: c_uint, slot: c_uint) -> c_uint;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
