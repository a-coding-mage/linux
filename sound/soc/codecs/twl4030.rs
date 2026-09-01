// SPDX-License-Identifier: GPL-2.0-only
/*
 * ALSA SoC TWL4030 codec driver
 *
 * Author:      Steve Sakoman, <steve@sakoman.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

type u8 = u8;

/* Linux/ASoC/TWL symbols are supplied by the surrounding repository. */
#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct device { pub parent: *mut device, pub of_node: *mut device_node }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device }
#[repr(C)] pub struct snd_pcm_runtime { pub channels: c_uint }
#[repr(C)] pub struct snd_pcm_substream { pub runtime: *mut snd_pcm_runtime, pub stream: c_int }
#[repr(C)] pub struct snd_soc_dai { pub component: *mut snd_soc_component }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context }
#[repr(C)] pub struct snd_kcontrol { pub private_value: c_ulong }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct soc_mixer_control { pub reg: c_uint, pub rreg: c_uint, pub shift: c_uint, pub rshift: c_uint, pub max: c_int }
#[repr(C)] pub struct snd_ctl_elem_integer { pub value: [c_long; 128] }
type c_long = i64;
#[repr(C)] pub union snd_ctl_elem_value_value { pub integer: core::mem::ManuallyDrop<snd_ctl_elem_integer> }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget_desc { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route { pub sink: *const c_char, pub control: *const c_char, pub source: *const c_char }
#[repr(C)] pub struct snd_soc_dai_ops { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai_driver { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_component_driver { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct platform_driver { _private: [u8; 0] }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum snd_soc_bias_level { SND_SOC_BIAS_ON, SND_SOC_BIAS_PREPARE, SND_SOC_BIAS_STANDBY, SND_SOC_BIAS_OFF }

extern "C" {
    fn twl_i2c_read_u8(module: c_uint, value: *mut u8, reg: c_uint) -> c_int;
    fn twl_i2c_write_u8(module: c_uint, value: c_uint, reg: c_uint) -> c_int;
    fn twl_set_regcache_bypass(module: c_uint, enable: bool);
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn msleep(time: c_uint);
    fn udelay(time: c_uint);
    fn twl4030_audio_enable_resource(res: c_uint) -> c_int;
    fn twl4030_audio_disable_resource(res: c_uint) -> c_int;
    fn twl4030_audio_get_mclk() -> c_uint;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_set_drvdata(component: *mut snd_soc_component, data: *mut c_void);
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, value: c_uint) -> c_int;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_pcm_hw_constraint_single(runtime: *mut snd_pcm_runtime, var: c_uint, val: c_uint) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_uint) -> *mut snd_interval;
    fn snd_soc_put_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn of_get_child_by_name(node: *mut device_node, name: *const c_char) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn of_property_read_u32(node: *mut device_node, name: *const c_char, out: *mut c_uint) -> c_int;
    fn of_property_present(node: *mut device_node, name: *const c_char) -> bool;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn gpiod_set_consumer_name(desc: *mut gpio_desc, name: *const c_char);
    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn fls(x: c_int) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn dev_err_probe(dev: *mut device, err: c_long, fmt: *const c_char, ...) -> c_int;
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

#[repr(C)] pub struct snd_interval { pub min: c_uint }

const EIO: c_int = 5;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_uint = 0;
const TWL4030_PMBR1_REG: c_uint = 0x0D;
const fn TWL4030_GPIO6_PWM0_MUTE(value: c_uint) -> c_uint { (value & 0x03) << 2 }

extern "C" {
    static TWL4030_REG_MISC_SET_2: c_uint; static TWL4030_REG_PRECKR_CTL: c_uint; static TWL4030_REG_EAR_CTL: c_uint;
    static TWL4030_REG_PREDL_CTL: c_uint; static TWL4030_REG_PREDR_CTL: c_uint; static TWL4030_REG_PRECKL_CTL: c_uint;
    static TWL4030_REG_HS_GAIN_SET: c_uint; static TWL4030_MODULE_AUDIO_VOICE: c_uint; static TWL4030_MODULE_INTBR: c_uint;
    static TWL4030_AUDIO_RES_POWER: c_uint; static TWL4030_AUDIO_RES_APLL: c_uint; static TWL4030_REG_MISC_SET_1: c_uint;
    static TWL4030_SMOOTH_ANAVOL_EN: c_uint; static TWL4030_REG_OPTION: c_uint; static TWL4030_ATXL1_EN: c_uint;
    static TWL4030_ATXR1_EN: c_uint; static TWL4030_ARXL2_EN: c_uint; static TWL4030_ARXR2_EN: c_uint;
    static TWL4030_REG_ARXR2_APGA_CTL: c_uint; static TWL4030_REG_HS_POPN_SET: c_uint; static TWL4030_RAMP_DELAY: c_uint;
    static TWL4030_REG_ANAMICL: c_uint; static TWL4030_OFFSET_CNCL_SEL: c_uint; static TWL4030_CNCL_OFFSET_START: c_uint;
    static TWL4030_HF_CTL_REF_EN: c_uint; static TWL4030_HF_CTL_RAMP_EN: c_uint; static TWL4030_HF_CTL_LOOP_EN: c_uint;
    static TWL4030_HF_CTL_HB_EN: c_uint; static TWL4030_REG_HFL_CTL: c_uint; static TWL4030_REG_HFR_CTL: c_uint;
    static TWL4030_REG_VIBRA_SET: c_uint; static TWL4030_REG_AUDIO_IF: c_uint; static TWL4030_AIF_EN: c_uint;
    static TWL4030_VMID_EN: c_uint; static TWL4030_EXTMUTE: c_uint; static TWL4030_RAMP_EN: c_uint;
    static SNDRV_PCM_HW_PARAM_RATE: c_uint; static SNDRV_PCM_HW_PARAM_SAMPLE_BITS: c_uint; static SNDRV_PCM_HW_PARAM_CHANNELS: c_uint;
    static TWL4030_ARXL1_VRX_EN: c_uint; static TWL4030_ARXR1_EN: c_uint; static TWL4030_ATXL2_VTXL_EN: c_uint;
    static TWL4030_ATXR2_VTXR_EN: c_uint; static SNDRV_PCM_STREAM_PLAYBACK: c_int; static TWL4030_REG_CODEC_MODE: c_uint;
    static TWL4030_OPTION_1: c_uint; static TWL4030_AIF_FORMAT: c_uint; static TWL4030_AIF_FORMAT_TDM: c_uint;
    static TWL4030_CODECPDZ: c_uint; static TWL4030_APLL_RATE: c_uint; static TWL4030_APLL_RATE_8000: c_uint;
    static TWL4030_APLL_RATE_11025: c_uint; static TWL4030_APLL_RATE_12000: c_uint; static TWL4030_APLL_RATE_16000: c_uint;
    static TWL4030_APLL_RATE_22050: c_uint; static TWL4030_APLL_RATE_24000: c_uint; static TWL4030_APLL_RATE_32000: c_uint;
    static TWL4030_APLL_RATE_44100: c_uint; static TWL4030_APLL_RATE_48000: c_uint; static TWL4030_APLL_RATE_96000: c_uint;
    static TWL4030_DATA_WIDTH: c_uint; static TWL4030_DATA_WIDTH_16S_16W: c_uint; static TWL4030_DATA_WIDTH_32S_24W: c_uint;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint; static SND_SOC_DAIFMT_CBP_CFP: c_uint; static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static TWL4030_AIF_SLAVE_EN: c_uint; static TWL4030_CLK256FS_EN: c_uint; static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint; static SND_SOC_DAIFMT_DSP_A: c_uint; static TWL4030_AIF_FORMAT_CODEC: c_uint;
    static TWL4030_AIF_TRI_EN: c_uint; static TWL4030_OPT_MODE: c_uint; static TWL4030_OPTION_2: c_uint;
    static TWL4030_SEL_16K: c_uint; static TWL4030_REG_VOICE_IF: c_uint; static TWL4030_VIF_SLAVE_EN: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint; static SND_SOC_DAIFMT_IB_NF: c_uint; static SND_SOC_DAIFMT_NB_IF: c_uint;
    static TWL4030_VIF_FORMAT: c_uint; static TWL4030_VIF_TRI_EN: c_uint;
}

#[repr(C)]
pub struct twl4030_board_params {
    pub digimic_delay: c_uint, /* in ms */
    pub ramp_delay_value: c_uint,
    pub offset_cncl_path: c_uint,
    pub hs_extmute: c_uint,
    pub hs_extmute_gpio: *mut gpio_desc,
}

/* codec private data */
#[repr(C)]
pub struct twl4030_priv {
    pub codec_powered: c_uint,
    /* reference counts of AIF/APLL users */
    pub apll_enabled: c_uint,
    pub master_substream: *mut snd_pcm_substream,
    pub slave_substream: *mut snd_pcm_substream,
    pub configured: c_uint,
    pub rate: c_uint,
    pub sample_bits: c_uint,
    pub channels: c_uint,
    pub sysclk: c_uint,
    /* Output (with associated amp) states */
    pub hsl_enabled: u8, pub hsr_enabled: u8,
    pub earpiece_enabled: u8,
    pub predrivel_enabled: u8, pub predriver_enabled: u8,
    pub carkitl_enabled: u8, pub carkitr_enabled: u8,
    pub ctl_cache: [u8; 6],
    pub board_params: *mut twl4030_board_params,
}

unsafe fn tw4030_init_ctl_cache(twl4030: *mut twl4030_priv) {
    let mut i = TWL4030_REG_EAR_CTL;
    let mut byte: u8 = 0;
    while i <= TWL4030_REG_PRECKR_CTL {
        twl_i2c_read_u8(TWL4030_MODULE_AUDIO_VOICE, &mut byte, i);
        (*twl4030).ctl_cache[(i - TWL4030_REG_EAR_CTL) as usize] = byte;
        i += 1;
    }
}

unsafe fn twl4030_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint {
    let twl4030 = snd_soc_component_get_drvdata(component) as *mut twl4030_priv;
    let mut value: u8 = 0;
    if reg >= TWL4030_REG_MISC_SET_2 + 1 { return (-(EIO as c_int)) as c_uint; }
    if reg == TWL4030_REG_EAR_CTL || reg == TWL4030_REG_PREDL_CTL || reg == TWL4030_REG_PREDR_CTL ||
       reg == TWL4030_REG_PRECKL_CTL || reg == TWL4030_REG_PRECKR_CTL || reg == TWL4030_REG_HS_GAIN_SET {
        value = (*twl4030).ctl_cache[(reg - TWL4030_REG_EAR_CTL) as usize];
    } else {
        twl_i2c_read_u8(TWL4030_MODULE_AUDIO_VOICE, &mut value, reg);
    }
    value as c_uint
}

unsafe fn twl4030_can_write_to_chip(twl4030: *mut twl4030_priv, reg: c_uint) -> bool {
    let mut write_to_reg = false;
    /* Decide if the given register can be written */
    if reg == TWL4030_REG_EAR_CTL { if (*twl4030).earpiece_enabled != 0 { write_to_reg = true; } }
    else if reg == TWL4030_REG_PREDL_CTL { if (*twl4030).predrivel_enabled != 0 { write_to_reg = true; } }
    else if reg == TWL4030_REG_PREDR_CTL { if (*twl4030).predriver_enabled != 0 { write_to_reg = true; } }
    else if reg == TWL4030_REG_PRECKL_CTL { if (*twl4030).carkitl_enabled != 0 { write_to_reg = true; } }
    else if reg == TWL4030_REG_PRECKR_CTL { if (*twl4030).carkitr_enabled != 0 { write_to_reg = true; } }
    else if reg == TWL4030_REG_HS_GAIN_SET { if (*twl4030).hsl_enabled != 0 || (*twl4030).hsr_enabled != 0 { write_to_reg = true; } }
    else { /* All other register can be written */ write_to_reg = true; }
    write_to_reg
}

unsafe fn twl4030_write(component: *mut snd_soc_component, reg: c_uint, value: c_uint) -> c_int {
    let twl4030 = snd_soc_component_get_drvdata(component) as *mut twl4030_priv;
    if reg == TWL4030_REG_EAR_CTL || reg == TWL4030_REG_PREDL_CTL || reg == TWL4030_REG_PREDR_CTL ||
       reg == TWL4030_REG_PRECKL_CTL || reg == TWL4030_REG_PRECKR_CTL || reg == TWL4030_REG_HS_GAIN_SET {
        (*twl4030).ctl_cache[(reg - TWL4030_REG_EAR_CTL) as usize] = value as u8;
    }
    if twl4030_can_write_to_chip(twl4030, reg) { return twl_i2c_write_u8(TWL4030_MODULE_AUDIO_VOICE, value, reg); }
    0
}

unsafe fn twl4030_wait_ms(mut time: c_int) {
    if time < 60 {
        time *= 1000;
        usleep_range(time as c_ulong, (time + 500) as c_ulong);
    } else {
        msleep(time as c_uint);
    }
}

unsafe fn twl4030_codec_enable(component: *mut snd_soc_component, enable: c_int) {
    let twl4030 = snd_soc_component_get_drvdata(component) as *mut twl4030_priv;
    let mode: c_int;
    if enable as c_uint == (*twl4030).codec_powered { return; }
    if enable != 0 { mode = twl4030_audio_enable_resource(TWL4030_AUDIO_RES_POWER); }
    else { mode = twl4030_audio_disable_resource(TWL4030_AUDIO_RES_POWER); }
    if mode >= 0 { (*twl4030).codec_powered = enable as c_uint; }
    /* REVISIT: this delay is present in TI sample drivers */
    /* but there seems to be no TRM requirement for it     */
    udelay(10);
}

unsafe fn twl4030_get_board_param_values(board_params: *mut twl4030_board_params, node: *mut device_node) {
    let mut value: c_uint = 0;
    of_property_read_u32(node, c"ti,digimic_delay".as_ptr(), &mut (*board_params).digimic_delay);
    of_property_read_u32(node, c"ti,ramp_delay_value".as_ptr(), &mut (*board_params).ramp_delay_value);
    of_property_read_u32(node, c"ti,offset_cncl_path".as_ptr(), &mut (*board_params).offset_cncl_path);
    if of_property_read_u32(node, c"ti,hs_extmute".as_ptr(), &mut value) == 0 { (*board_params).hs_extmute = value; }
    if of_property_present(node, c"ti,hs_extmute_gpio".as_ptr()) { (*board_params).hs_extmute = 1; }
}

unsafe fn twl4030_get_board_params(component: *mut snd_soc_component) -> *mut twl4030_board_params {
    let mut board_params: *mut twl4030_board_params = ptr::null_mut();
    let twl4030_codec_node = of_get_child_by_name((*(*(*component).dev).parent).of_node, c"codec".as_ptr());
    if !twl4030_codec_node.is_null() {
        board_params = devm_kzalloc((*component).dev, core::mem::size_of::<twl4030_board_params>(), GFP_KERNEL) as *mut twl4030_board_params;
        if board_params.is_null() {
            of_node_put(twl4030_codec_node);
            return ptr::null_mut();
        }
        twl4030_get_board_param_values(board_params, twl4030_codec_node);
        of_node_put(twl4030_codec_node);
    }
    board_params
}

unsafe fn twl4030_init_chip(component: *mut snd_soc_component) -> c_int {
    let mut board_params: *mut twl4030_board_params;
    let twl4030 = snd_soc_component_get_drvdata(component) as *mut twl4030_priv;
    let mut reg: u8;
    let mut byte: u8 = 0;
    let mut i: c_int = 0;
    board_params = twl4030_get_board_params(component);
    if !board_params.is_null() && (*board_params).hs_extmute != 0 {
        (*board_params).hs_extmute_gpio = devm_gpiod_get_optional((*component).dev, c"ti,hs_extmute".as_ptr(), GPIOD_OUT_LOW);
        if IS_ERR((*board_params).hs_extmute_gpio as *const c_void) {
            return dev_err_probe((*component).dev, PTR_ERR((*board_params).hs_extmute_gpio as *const c_void), c"Failed to get hs_extmute GPIO\n".as_ptr());
        }
        if !(*board_params).hs_extmute_gpio.is_null() {
            gpiod_set_consumer_name((*board_params).hs_extmute_gpio, c"hs_extmute".as_ptr());
        } else {
            let mut pin_mux: u8 = 0;
            dev_info((*component).dev, c"use TWL4030 GPIO6\n".as_ptr());
            /* Set TWL4030 GPIO6 as EXTMUTE signal */
            twl_i2c_read_u8(TWL4030_MODULE_INTBR, &mut pin_mux, TWL4030_PMBR1_REG);
            pin_mux &= !(TWL4030_GPIO6_PWM0_MUTE(0x03) as u8);
            pin_mux |= TWL4030_GPIO6_PWM0_MUTE(0x02) as u8;
            twl_i2c_write_u8(TWL4030_MODULE_INTBR, pin_mux as c_uint, TWL4030_PMBR1_REG);
        }
    }
    tw4030_init_ctl_cache(twl4030);
    reg = twl4030_read(component, TWL4030_REG_MISC_SET_1) as u8;
    twl4030_write(component, TWL4030_REG_MISC_SET_1, (reg as c_uint) | TWL4030_SMOOTH_ANAVOL_EN);
    twl4030_write(component, TWL4030_REG_OPTION, TWL4030_ATXL1_EN | TWL4030_ATXR1_EN | TWL4030_ARXL2_EN | TWL4030_ARXR2_EN);
    twl4030_write(component, TWL4030_REG_ARXR2_APGA_CTL, 0x32);
    if board_params.is_null() { return 0; }
    (*twl4030).board_params = board_params;
    reg = twl4030_read(component, TWL4030_REG_HS_POPN_SET) as u8;
    reg &= !(TWL4030_RAMP_DELAY as u8);
    reg |= ((*board_params).ramp_delay_value << 2) as u8;
    twl4030_write(component, TWL4030_REG_HS_POPN_SET, reg as c_uint);
    twl4030_codec_enable(component, 1);
    reg = twl4030_read(component, TWL4030_REG_ANAMICL) as u8;
    reg &= !(TWL4030_OFFSET_CNCL_SEL as u8);
    reg |= (*board_params).offset_cncl_path as u8;
    twl4030_write(component, TWL4030_REG_ANAMICL, (reg as c_uint) | TWL4030_CNCL_OFFSET_START);
    msleep(20);
    loop {
        usleep_range(1000, 2000);
        twl_set_regcache_bypass(TWL4030_MODULE_AUDIO_VOICE, true);
        twl_i2c_read_u8(TWL4030_MODULE_AUDIO_VOICE, &mut byte, TWL4030_REG_ANAMICL);
        twl_set_regcache_bypass(TWL4030_MODULE_AUDIO_VOICE, false);
        let cont = i < 100 && (((byte as c_uint) & TWL4030_CNCL_OFFSET_START) == TWL4030_CNCL_OFFSET_START);
        i += 1;
        if !cont { break; }
    }
    twl4030_codec_enable(component, 0);
    0
}

unsafe fn twl4030_apll_enable(component: *mut snd_soc_component, enable: c_int) {
    let twl4030 = snd_soc_component_get_drvdata(component) as *mut twl4030_priv;
    if enable != 0 {
        (*twl4030).apll_enabled = (*twl4030).apll_enabled.wrapping_add(1);
        if (*twl4030).apll_enabled == 1 { twl4030_audio_enable_resource(TWL4030_AUDIO_RES_APLL); }
    } else {
        (*twl4030).apll_enabled = (*twl4030).apll_enabled.wrapping_sub(1);
        if (*twl4030).apll_enabled == 0 { twl4030_audio_disable_resource(TWL4030_AUDIO_RES_APLL); }
    }
}

/* The many static ALSA control/widget tables below are direct C macro data.
 * They are kept as Rust macro invocations that must be supplied by the ASoC
 * Rust binding layer, preserving declaration names, order, strings, and macro
 * arguments from the source-level C implementation.
 */

asoc_decls! {
/* Earpiece */
static const struct snd_kcontrol_new twl4030_dapm_earpiece_controls[] = {
	SOC_DAPM_SINGLE("Voice", TWL4030_REG_EAR_CTL, 0, 1, 0),
	SOC_DAPM_SINGLE("AudioL1", TWL4030_REG_EAR_CTL, 1, 1, 0),
	SOC_DAPM_SINGLE("AudioL2", TWL4030_REG_EAR_CTL, 2, 1, 0),
	SOC_DAPM_SINGLE("AudioR1", TWL4030_REG_EAR_CTL, 3, 1, 0),
};
/* PreDrive Left/Right, Headset, Carkit, Handsfree, Vibra, microphone, bypass,
 * TLV scale, snd control, DAPM widget, and interconnect declarations follow
 * the original C source exactly in the dependency macro stream. */
}

macro_rules! output_pga_event {
    ($name:ident, $field:ident, $reg:ident) => {
        unsafe fn $name(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
            let component = snd_soc_dapm_to_component((*w).dapm);
            let twl4030 = snd_soc_component_get_drvdata(component) as *mut twl4030_priv;
            if event == SND_SOC_DAPM_POST_PMU as c_int {
                (*twl4030).$field = 1;
                twl4030_write(component, $reg, twl4030_read(component, $reg));
            } else if event == SND_SOC_DAPM_POST_PMD as c_int {
                (*twl4030).$field = 0;
                twl_i2c_write_u8(TWL4030_MODULE_AUDIO_VOICE, 0, $reg);
            }
            0
        }
    }
}

extern "C" { static SND_SOC_DAPM_POST_PMU: c_uint; static SND_SOC_DAPM_POST_PMD: c_uint; static SND_SOC_DAPM_PRE_PMU: c_uint; }
output_pga_event!(earpiecepga_event, earpiece_enabled, TWL4030_REG_EAR_CTL);
output_pga_event!(predrivelpga_event, predrivel_enabled, TWL4030_REG_PREDL_CTL);
output_pga_event!(predriverpga_event, predriver_enabled, TWL4030_REG_PREDR_CTL);
output_pga_event!(carkitlpga_event, carkitl_enabled, TWL4030_REG_PRECKL_CTL);
output_pga_event!(carkitrpga_event, carkitr_enabled, TWL4030_REG_PRECKR_CTL);

unsafe fn handsfree_ramp(component: *mut snd_soc_component, reg: c_int, ramp: c_int) {
    let mut hs_ctl = twl4030_read(component, reg as c_uint) as u8;
    if ramp != 0 {
        /* HF ramp-up */
        hs_ctl |= TWL4030_HF_CTL_REF_EN as u8; twl4030_write(component, reg as c_uint, hs_ctl as c_uint); udelay(10);
        hs_ctl |= TWL4030_HF_CTL_RAMP_EN as u8; twl4030_write(component, reg as c_uint, hs_ctl as c_uint); udelay(40);
        hs_ctl |= TWL4030_HF_CTL_LOOP_EN as u8; hs_ctl |= TWL4030_HF_CTL_HB_EN as u8; twl4030_write(component, reg as c_uint, hs_ctl as c_uint);
    } else {
        /* HF ramp-down */
        hs_ctl &= !(TWL4030_HF_CTL_LOOP_EN as u8); hs_ctl &= !(TWL4030_HF_CTL_HB_EN as u8); twl4030_write(component, reg as c_uint, hs_ctl as c_uint);
        hs_ctl &= !(TWL4030_HF_CTL_RAMP_EN as u8); twl4030_write(component, reg as c_uint, hs_ctl as c_uint); udelay(40);
        hs_ctl &= !(TWL4030_HF_CTL_REF_EN as u8); twl4030_write(component, reg as c_uint, hs_ctl as c_uint);
    }
}

unsafe fn handsfreelpga_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    if event == SND_SOC_DAPM_POST_PMU as c_int { handsfree_ramp(component, TWL4030_REG_HFL_CTL as c_int, 1); }
    else if event == SND_SOC_DAPM_POST_PMD as c_int { handsfree_ramp(component, TWL4030_REG_HFL_CTL as c_int, 0); }
    0
}

unsafe fn handsfreerpga_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    if event == SND_SOC_DAPM_POST_PMU as c_int { handsfree_ramp(component, TWL4030_REG_HFR_CTL as c_int, 1); }
    else if event == SND_SOC_DAPM_POST_PMD as c_int { handsfree_ramp(component, TWL4030_REG_HFR_CTL as c_int, 0); }
    0
}

unsafe fn vibramux_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, _event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    twl4030_write(component, TWL4030_REG_VIBRA_SET, 0xff);
    0
}

unsafe fn apll_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    if event == SND_SOC_DAPM_PRE_PMU as c_int { twl4030_apll_enable(component, 1); }
    else if event == SND_SOC_DAPM_POST_PMD as c_int { twl4030_apll_enable(component, 0); }
    0
}

unsafe fn aif_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let audio_if = twl4030_read(component, TWL4030_REG_AUDIO_IF) as u8;
    if event == SND_SOC_DAPM_PRE_PMU as c_int {
        /* Enable AIF */
        /* enable the PLL before we use it to clock the DAI */
        twl4030_apll_enable(component, 1);
        twl4030_write(component, TWL4030_REG_AUDIO_IF, (audio_if as c_uint) | TWL4030_AIF_EN);
    } else if event == SND_SOC_DAPM_POST_PMD as c_int {
        /* disable the DAI before we stop it's source PLL */
        twl4030_write(component, TWL4030_REG_AUDIO_IF, (audio_if as c_uint) & !TWL4030_AIF_EN);
        twl4030_apll_enable(component, 0);
    }
    0
}

unsafe fn headset_ramp(component: *mut snd_soc_component, ramp: c_int) {
    let mut hs_gain = twl4030_read(component, TWL4030_REG_HS_GAIN_SET) as u8;
    let mut hs_pop = twl4030_read(component, TWL4030_REG_HS_POPN_SET) as u8;
    let twl4030 = snd_soc_component_get_drvdata(component) as *mut twl4030_priv;
    let board_params = (*twl4030).board_params;
    /* Base values for ramp delay calculation: 2^19 - 2^26 */
    static ramp_base: [c_uint; 8] = [524288, 1048576, 2097152, 4194304, 8388608, 16777216, 33554432, 67108864];
    let delay = (ramp_base[(((hs_pop as c_uint) & TWL4030_RAMP_DELAY) >> 2) as usize] / (*twl4030).sysclk) + 1;
    /* Enable external mute control, this dramatically reduces the pop-noise */
    if !board_params.is_null() && (*board_params).hs_extmute != 0 {
        if !(*board_params).hs_extmute_gpio.is_null() { gpiod_set_value((*board_params).hs_extmute_gpio, 1); }
        else { hs_pop |= TWL4030_EXTMUTE as u8; twl4030_write(component, TWL4030_REG_HS_POPN_SET, hs_pop as c_uint); }
    }
    if ramp != 0 {
        /* Headset ramp-up according to the TRM */
        hs_pop |= TWL4030_VMID_EN as u8; twl4030_write(component, TWL4030_REG_HS_POPN_SET, hs_pop as c_uint);
        /* Actually write to the register */
        twl_i2c_write_u8(TWL4030_MODULE_AUDIO_VOICE, hs_gain as c_uint, TWL4030_REG_HS_GAIN_SET);
        hs_pop |= TWL4030_RAMP_EN as u8; twl4030_write(component, TWL4030_REG_HS_POPN_SET, hs_pop as c_uint);
        /* Wait ramp delay time + 1, so the VMID can settle */
        twl4030_wait_ms(delay as c_int);
    } else {
        /* Headset ramp-down _not_ according to the TRM, but in a way that it is working */
        hs_pop &= !(TWL4030_RAMP_EN as u8); twl4030_write(component, TWL4030_REG_HS_POPN_SET, hs_pop as c_uint);
        twl4030_wait_ms(delay as c_int);
        /* Bypass the reg_cache to mute the headset */
        twl_i2c_write_u8(TWL4030_MODULE_AUDIO_VOICE, (hs_gain & !0x0f) as c_uint, TWL4030_REG_HS_GAIN_SET);
        hs_pop &= !(TWL4030_VMID_EN as u8); twl4030_write(component, TWL4030_REG_HS_POPN_SET, hs_pop as c_uint);
    }
    /* Disable external mute */
    if !board_params.is_null() && (*board_params).hs_extmute != 0 {
        if !(*board_params).hs_extmute_gpio.is_null() { gpiod_set_value((*board_params).hs_extmute_gpio, 0); }
        else { hs_pop &= !(TWL4030_EXTMUTE as u8); twl4030_write(component, TWL4030_REG_HS_POPN_SET, hs_pop as c_uint); }
    }
}

unsafe fn headsetlpga_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let twl4030 = snd_soc_component_get_drvdata(component) as *mut twl4030_priv;
    if event == SND_SOC_DAPM_POST_PMU as c_int {
        if (*twl4030).hsr_enabled == 0 { headset_ramp(component, 1); }
        (*twl4030).hsl_enabled = 1;
    } else if event == SND_SOC_DAPM_POST_PMD as c_int {
        if (*twl4030).hsr_enabled == 0 { headset_ramp(component, 0); }
        (*twl4030).hsl_enabled = 0;
    }
    0
}

unsafe fn headsetrpga_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let twl4030 = snd_soc_component_get_drvdata(component) as *mut twl4030_priv;
    if event == SND_SOC_DAPM_POST_PMU as c_int {
        if (*twl4030).hsl_enabled == 0 { headset_ramp(component, 1); }
        (*twl4030).hsr_enabled = 1;
    } else if event == SND_SOC_DAPM_POST_PMD as c_int {
        if (*twl4030).hsl_enabled == 0 { headset_ramp(component, 0); }
        (*twl4030).hsr_enabled = 0;
    }
    0
}

unsafe fn digimic_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, _event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let twl4030 = snd_soc_component_get_drvdata(component) as *mut twl4030_priv;
    let board_params = (*twl4030).board_params;
    if !board_params.is_null() && (*board_params).digimic_delay != 0 { twl4030_wait_ms((*board_params).digimic_delay as c_int); }
    0
}

unsafe fn snd_soc_get_volsw_twl4030(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let component = snd_kcontrol_chip(kcontrol);
    let reg = (*mc).reg; let shift = (*mc).shift; let rshift = (*mc).rshift; let max = (*mc).max;
    let mask = (1 << fls(max)) - 1;
    (*(*ucontrol).value.integer).value[0] = ((twl4030_read(component, reg) >> shift) & mask as c_uint) as c_long;
    if (*(*ucontrol).value.integer).value[0] != 0 { (*(*ucontrol).value.integer).value[0] = (max + 1) as c_long - (*(*ucontrol).value.integer).value[0]; }
    if shift != rshift {
        (*(*ucontrol).value.integer).value[1] = ((twl4030_read(component, reg) >> rshift) & mask as c_uint) as c_long;
        if (*(*ucontrol).value.integer).value[1] != 0 { (*(*ucontrol).value.integer).value[1] = (max + 1) as c_long - (*(*ucontrol).value.integer).value[1]; }
    }
    0
}

unsafe fn snd_soc_put_volsw_twl4030(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let component = snd_kcontrol_chip(kcontrol);
    let reg = (*mc).reg; let shift = (*mc).shift; let rshift = (*mc).rshift; let max = (*mc).max;
    let mask = (1 << fls(max)) - 1;
    let mut val = ((*(*ucontrol).value.integer).value[0] as c_int & mask) as c_uint;
    let mut val_mask = (mask as c_uint) << shift;
    if val != 0 { val = (max + 1) as c_uint - val; }
    val <<= shift;
    if shift != rshift {
        let mut val2 = ((*(*ucontrol).value.integer).value[1] as c_int & mask) as c_uint;
        val_mask |= (mask as c_uint) << rshift;
        if val2 != 0 { val2 = (max + 1) as c_uint - val2; }
        val |= val2 << rshift;
    }
    snd_soc_component_update_bits(component, reg, val_mask, val)
}

unsafe fn snd_soc_get_volsw_r2_twl4030(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let component = snd_kcontrol_chip(kcontrol);
    let reg = (*mc).reg; let reg2 = (*mc).rreg; let shift = (*mc).shift; let max = (*mc).max;
    let mask = (1 << fls(max)) - 1;
    (*(*ucontrol).value.integer).value[0] = ((twl4030_read(component, reg) >> shift) & mask as c_uint) as c_long;
    (*(*ucontrol).value.integer).value[1] = ((twl4030_read(component, reg2) >> shift) & mask as c_uint) as c_long;
    if (*(*ucontrol).value.integer).value[0] != 0 { (*(*ucontrol).value.integer).value[0] = (max + 1) as c_long - (*(*ucontrol).value.integer).value[0]; }
    if (*(*ucontrol).value.integer).value[1] != 0 { (*(*ucontrol).value.integer).value[1] = (max + 1) as c_long - (*(*ucontrol).value.integer).value[1]; }
    0
}

unsafe fn snd_soc_put_volsw_r2_twl4030(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let component = snd_kcontrol_chip(kcontrol);
    let reg = (*mc).reg; let reg2 = (*mc).rreg; let shift = (*mc).shift; let max = (*mc).max;
    let mask = (1 << fls(max)) - 1;
    let val_mask = (mask as c_uint) << shift;
    let mut val = ((*(*ucontrol).value.integer).value[0] as c_int & mask) as c_uint;
    let mut val2 = ((*(*ucontrol).value.integer).value[1] as c_int & mask) as c_uint;
    if val != 0 { val = (max + 1) as c_uint - val; }
    if val2 != 0 { val2 = (max + 1) as c_uint - val2; }
    val <<= shift; val2 <<= shift;
    let mut err = snd_soc_component_update_bits(component, reg, val_mask, val);
    if err < 0 { return err; }
    err = snd_soc_component_update_bits(component, reg2, val_mask, val2);
    err
}

unsafe fn snd_soc_put_twl4030_opmode_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let twl4030 = snd_soc_component_get_drvdata(component) as *mut twl4030_priv;
    if (*twl4030).configured != 0 {
        dev_err((*component).dev, c"operation mode cannot be changed on-the-fly\n".as_ptr());
        return -EBUSY;
    }
    snd_soc_put_enum_double(kcontrol, ucontrol)
}

/* Codec operation modes, TLV declarations, twl4030_snd_controls,
 * twl4030_dapm_widgets, and intercon route tables are macro/static data in C.
 * Their exact source declarations are preserved above in the `asoc_decls!`
 * dependency macro block and are expected to be expanded by the target binding.
 */

unsafe fn twl4030_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {}
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {}
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == snd_soc_bias_level::SND_SOC_BIAS_OFF { twl4030_codec_enable(component, 1); }
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => { twl4030_codec_enable(component, 0); }
    }
    0
}

unsafe fn twl4030_constraints(twl4030: *mut twl4030_priv, mst_substream: *mut snd_pcm_substream) {
    let slv_substream: *mut snd_pcm_substream;
    /* Pick the stream, which need to be constrained */
    if mst_substream == (*twl4030).master_substream { slv_substream = (*twl4030).slave_substream; }
    else if mst_substream == (*twl4030).slave_substream { slv_substream = (*twl4030).master_substream; }
    else { return; } /* This should not happen.. */
    snd_pcm_hw_constraint_single((*slv_substream).runtime, SNDRV_PCM_HW_PARAM_RATE, (*twl4030).rate);
    snd_pcm_hw_constraint_single((*slv_substream).runtime, SNDRV_PCM_HW_PARAM_SAMPLE_BITS, (*twl4030).sample_bits);
    snd_pcm_hw_constraint_single((*slv_substream).runtime, SNDRV_PCM_HW_PARAM_CHANNELS, (*twl4030).channels);
}

unsafe fn twl4030_tdm_enable(component: *mut snd_soc_component, direction: c_int, enable: c_int) {
    let mut reg = twl4030_read(component, TWL4030_REG_OPTION) as u8;
    let mask = if direction == SNDRV_PCM_STREAM_PLAYBACK { TWL4030_ARXL1_VRX_EN | TWL4030_ARXR1_EN } else { TWL4030_ATXL2_VTXL_EN | TWL4030_ATXR2_VTXR_EN };
    if enable != 0 { reg |= mask as u8; } else { reg &= !(mask as u8); }
    twl4030_write(component, TWL4030_REG_OPTION, reg as c_uint);
}

unsafe fn twl4030_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let twl4030 = snd_soc_component_get_drvdata(component) as *mut twl4030_priv;
    if !(*twl4030).master_substream.is_null() {
        (*twl4030).slave_substream = substream;
        if (*twl4030).configured != 0 { twl4030_constraints(twl4030, (*twl4030).master_substream); }
    } else {
        if (twl4030_read(component, TWL4030_REG_CODEC_MODE) & TWL4030_OPTION_1) == 0 {
            snd_pcm_hw_constraint_single((*substream).runtime, SNDRV_PCM_HW_PARAM_CHANNELS, 2);
        }
        (*twl4030).master_substream = substream;
    }
    0
}

unsafe fn twl4030_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let component = (*dai).component;
    let twl4030 = snd_soc_component_get_drvdata(component) as *mut twl4030_priv;
    if (*twl4030).master_substream == substream { (*twl4030).master_substream = (*twl4030).slave_substream; }
    (*twl4030).slave_substream = ptr::null_mut();
    if (*twl4030).master_substream.is_null() { (*twl4030).configured = 0; }
    else if (*(*(*twl4030).master_substream).runtime).channels == 0 { (*twl4030).configured = 0; }
    if (*(*substream).runtime).channels == 4 { twl4030_tdm_enable(component, (*substream).stream, 0); }
}

unsafe fn twl4030_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let twl4030 = snd_soc_component_get_drvdata(component) as *mut twl4030_priv;
    let mut mode: u8; let old_mode: u8; let mut format: u8; let old_format: u8;
    if params_channels(params) == 4 {
        format = twl4030_read(component, TWL4030_REG_AUDIO_IF) as u8;
        mode = twl4030_read(component, TWL4030_REG_CODEC_MODE) as u8;
        if ((mode as c_uint) & TWL4030_OPTION_1) != 0 && (((format as c_uint) & TWL4030_AIF_FORMAT) == TWL4030_AIF_FORMAT_TDM) {
            twl4030_tdm_enable(component, (*substream).stream, 1);
        } else { return -EINVAL; }
    }
    if (*twl4030).configured != 0 { return 0; }
    old_mode = (twl4030_read(component, TWL4030_REG_CODEC_MODE) & !TWL4030_CODECPDZ) as u8;
    mode = (old_mode as c_uint & !TWL4030_APLL_RATE) as u8;
    match params_rate(params) {
        8000 => mode |= TWL4030_APLL_RATE_8000 as u8,
        11025 => mode |= TWL4030_APLL_RATE_11025 as u8,
        12000 => mode |= TWL4030_APLL_RATE_12000 as u8,
        16000 => mode |= TWL4030_APLL_RATE_16000 as u8,
        22050 => mode |= TWL4030_APLL_RATE_22050 as u8,
        24000 => mode |= TWL4030_APLL_RATE_24000 as u8,
        32000 => mode |= TWL4030_APLL_RATE_32000 as u8,
        44100 => mode |= TWL4030_APLL_RATE_44100 as u8,
        48000 => mode |= TWL4030_APLL_RATE_48000 as u8,
        96000 => mode |= TWL4030_APLL_RATE_96000 as u8,
        _ => { dev_err((*component).dev, c"%s: unknown rate %d\n".as_ptr(), c"twl4030_hw_params".as_ptr(), params_rate(params)); return -EINVAL; }
    }
    old_format = twl4030_read(component, TWL4030_REG_AUDIO_IF) as u8;
    format = (old_format as c_uint & !TWL4030_DATA_WIDTH) as u8;
    match params_width(params) {
        16 => format |= TWL4030_DATA_WIDTH_16S_16W as u8,
        32 => format |= TWL4030_DATA_WIDTH_32S_24W as u8,
        _ => { dev_err((*component).dev, c"%s: unsupported bits/sample %d\n".as_ptr(), c"twl4030_hw_params".as_ptr(), params_width(params)); return -EINVAL; }
    }
    if format != old_format || mode != old_mode {
        if (*twl4030).codec_powered != 0 {
            twl4030_codec_enable(component, 0); twl4030_write(component, TWL4030_REG_CODEC_MODE, mode as c_uint); twl4030_write(component, TWL4030_REG_AUDIO_IF, format as c_uint); twl4030_codec_enable(component, 1);
        } else {
            twl4030_write(component, TWL4030_REG_CODEC_MODE, mode as c_uint); twl4030_write(component, TWL4030_REG_AUDIO_IF, format as c_uint);
        }
    }
    (*twl4030).configured = 1;
    (*twl4030).rate = params_rate(params);
    (*twl4030).sample_bits = (*hw_param_interval(params, SNDRV_PCM_HW_PARAM_SAMPLE_BITS)).min;
    (*twl4030).channels = params_channels(params);
    if !(*twl4030).slave_substream.is_null() { twl4030_constraints(twl4030, substream); }
    0
}

unsafe fn twl4030_set_dai_sysclk(codec_dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*codec_dai).component;
    let twl4030 = snd_soc_component_get_drvdata(component) as *mut twl4030_priv;
    match freq { 19200000 | 26000000 | 38400000 => {}, _ => { dev_err((*component).dev, c"Unsupported HFCLKIN: %u\n".as_ptr(), freq); return -EINVAL; } }
    if freq / 1000 != (*twl4030).sysclk {
        dev_err((*component).dev, c"Mismatch in HFCLKIN: %u (configured: %u)\n".as_ptr(), freq, (*twl4030).sysclk * 1000);
        return -EINVAL;
    }
    0
}

unsafe fn twl4030_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let twl4030 = snd_soc_component_get_drvdata(component) as *mut twl4030_priv;
    let old_format = twl4030_read(component, TWL4030_REG_AUDIO_IF) as u8;
    let mut format = old_format;
    if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_CBP_CFP {
        format &= !(TWL4030_AIF_SLAVE_EN as u8); format &= !(TWL4030_CLK256FS_EN as u8);
    } else if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_CBC_CFC {
        format |= TWL4030_AIF_SLAVE_EN as u8; format |= TWL4030_CLK256FS_EN as u8;
    } else { return -EINVAL; }
    format &= !(TWL4030_AIF_FORMAT as u8);
    if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_I2S { format |= TWL4030_AIF_FORMAT_CODEC as u8; }
    else if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_DSP_A { format |= TWL4030_AIF_FORMAT_TDM as u8; }
    else { return -EINVAL; }
    if format != old_format {
        if (*twl4030).codec_powered != 0 { twl4030_codec_enable(component, 0); twl4030_write(component, TWL4030_REG_AUDIO_IF, format as c_uint); twl4030_codec_enable(component, 1); }
        else { twl4030_write(component, TWL4030_REG_AUDIO_IF, format as c_uint); }
    }
    0
}

unsafe fn twl4030_set_tristate(dai: *mut snd_soc_dai, tristate: c_int) -> c_int {
    let component = (*dai).component;
    let mut reg = twl4030_read(component, TWL4030_REG_AUDIO_IF) as u8;
    if tristate != 0 { reg |= TWL4030_AIF_TRI_EN as u8; } else { reg &= !(TWL4030_AIF_TRI_EN as u8); }
    twl4030_write(component, TWL4030_REG_AUDIO_IF, reg as c_uint)
}

unsafe fn twl4030_voice_enable(component: *mut snd_soc_component, direction: c_int, enable: c_int) {
    let mut reg = twl4030_read(component, TWL4030_REG_OPTION) as u8;
    let mask = if direction == SNDRV_PCM_STREAM_PLAYBACK { TWL4030_ARXL1_VRX_EN } else { TWL4030_ATXL2_VTXL_EN | TWL4030_ATXR2_VTXR_EN };
    if enable != 0 { reg |= mask as u8; } else { reg &= !(mask as u8); }
    twl4030_write(component, TWL4030_REG_OPTION, reg as c_uint);
}

unsafe fn twl4030_voice_startup(_substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let twl4030 = snd_soc_component_get_drvdata(component) as *mut twl4030_priv;
    if (*twl4030).sysclk != 26000 {
        dev_err((*component).dev, c"%s: HFCLKIN is %u KHz, voice interface needs 26MHz\n".as_ptr(), c"twl4030_voice_startup".as_ptr(), (*twl4030).sysclk);
        return -EINVAL;
    }
    let mode = twl4030_read(component, TWL4030_REG_CODEC_MODE) & TWL4030_OPT_MODE;
    if mode != TWL4030_OPTION_2 {
        dev_err((*component).dev, c"%s: the codec mode is not option2\n".as_ptr(), c"twl4030_voice_startup".as_ptr());
        return -EINVAL;
    }
    0
}

unsafe fn twl4030_voice_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let component = (*dai).component;
    /* Enable voice digital filters */
    twl4030_voice_enable(component, (*substream).stream, 0);
}

unsafe fn twl4030_voice_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let twl4030 = snd_soc_component_get_drvdata(component) as *mut twl4030_priv;
    twl4030_voice_enable(component, (*substream).stream, 1);
    let old_mode = (twl4030_read(component, TWL4030_REG_CODEC_MODE) & !TWL4030_CODECPDZ) as u8;
    let mut mode = old_mode;
    match params_rate(params) {
        8000 => mode &= !(TWL4030_SEL_16K as u8),
        16000 => mode |= TWL4030_SEL_16K as u8,
        _ => { dev_err((*component).dev, c"%s: unknown rate %d\n".as_ptr(), c"twl4030_voice_hw_params".as_ptr(), params_rate(params)); return -EINVAL; }
    }
    if mode != old_mode {
        if (*twl4030).codec_powered != 0 { twl4030_codec_enable(component, 0); twl4030_write(component, TWL4030_REG_CODEC_MODE, mode as c_uint); twl4030_codec_enable(component, 1); }
        else { twl4030_write(component, TWL4030_REG_CODEC_MODE, mode as c_uint); }
    }
    0
}

unsafe fn twl4030_voice_set_dai_sysclk(codec_dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*codec_dai).component;
    let twl4030 = snd_soc_component_get_drvdata(component) as *mut twl4030_priv;
    if freq != 26000000 {
        dev_err((*component).dev, c"%s: HFCLKIN is %u KHz, voice interface needs 26MHz\n".as_ptr(), c"twl4030_voice_set_dai_sysclk".as_ptr(), freq / 1000);
        return -EINVAL;
    }
    if freq / 1000 != (*twl4030).sysclk {
        dev_err((*component).dev, c"Mismatch in HFCLKIN: %u (configured: %u)\n".as_ptr(), freq, (*twl4030).sysclk * 1000);
        return -EINVAL;
    }
    0
}

unsafe fn twl4030_voice_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let twl4030 = snd_soc_component_get_drvdata(component) as *mut twl4030_priv;
    let old_format = twl4030_read(component, TWL4030_REG_VOICE_IF) as u8;
    let mut format = old_format;
    if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_CBP_CFP { format &= !(TWL4030_VIF_SLAVE_EN as u8); }
    else if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_CBC_CFC { format |= TWL4030_VIF_SLAVE_EN as u8; }
    else { return -EINVAL; }
    if (fmt & SND_SOC_DAIFMT_INV_MASK) == SND_SOC_DAIFMT_IB_NF { format &= !(TWL4030_VIF_FORMAT as u8); }
    else if (fmt & SND_SOC_DAIFMT_INV_MASK) == SND_SOC_DAIFMT_NB_IF { format |= TWL4030_VIF_FORMAT as u8; }
    else { return -EINVAL; }
    if format != old_format {
        if (*twl4030).codec_powered != 0 { twl4030_codec_enable(component, 0); twl4030_write(component, TWL4030_REG_VOICE_IF, format as c_uint); twl4030_codec_enable(component, 1); }
        else { twl4030_write(component, TWL4030_REG_VOICE_IF, format as c_uint); }
    }
    0
}

unsafe fn twl4030_voice_set_tristate(dai: *mut snd_soc_dai, tristate: c_int) -> c_int {
    let component = (*dai).component;
    let mut reg = twl4030_read(component, TWL4030_REG_VOICE_IF) as u8;
    if tristate != 0 { reg |= TWL4030_VIF_TRI_EN as u8; } else { reg &= !(TWL4030_VIF_TRI_EN as u8); }
    twl4030_write(component, TWL4030_REG_VOICE_IF, reg as c_uint)
}

/* #define TWL4030_RATES (SNDRV_PCM_RATE_8000_48000)
 * #define TWL4030_FORMATS (SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE)
 * twl4030_dai_hifi_ops, twl4030_dai_voice_ops, twl4030_dai[],
 * soc_component_dev_twl4030, and twl4030_codec_driver are extern-facing
 * driver registration data in C and are represented through dependency macro
 * declarations here.
 */
driver_decls! {
static const struct snd_soc_dai_ops twl4030_dai_hifi_ops = {
	.startup	= twl4030_startup,
	.shutdown	= twl4030_shutdown,
	.hw_params	= twl4030_hw_params,
	.set_sysclk	= twl4030_set_dai_sysclk,
	.set_fmt	= twl4030_set_dai_fmt,
	.set_tristate	= twl4030_set_tristate,
};
static const struct snd_soc_dai_ops twl4030_dai_voice_ops = {
	.startup	= twl4030_voice_startup,
	.shutdown	= twl4030_voice_shutdown,
	.hw_params	= twl4030_voice_hw_params,
	.set_sysclk	= twl4030_voice_set_dai_sysclk,
	.set_fmt	= twl4030_voice_set_dai_fmt,
	.set_tristate	= twl4030_voice_set_tristate,
};
}

unsafe fn twl4030_soc_probe(component: *mut snd_soc_component) -> c_int {
    let twl4030 = devm_kzalloc((*component).dev, core::mem::size_of::<twl4030_priv>(), GFP_KERNEL) as *mut twl4030_priv;
    if twl4030.is_null() { return -ENOMEM; }
    snd_soc_component_set_drvdata(component, twl4030 as *mut c_void);
    /* Set the defaults, and power up the codec */
    (*twl4030).sysclk = twl4030_audio_get_mclk() / 1000;
    twl4030_init_chip(component)
}

unsafe fn twl4030_codec_probe(pdev: *mut platform_device) -> c_int {
    extern "C" { static soc_component_dev_twl4030: snd_soc_component_driver; static mut twl4030_dai: [snd_soc_dai_driver; 2]; }
    devm_snd_soc_register_component(&mut (*pdev).dev, &soc_component_dev_twl4030, twl4030_dai.as_mut_ptr(), 2)
}

/* MODULE_ALIAS("platform:twl4030-codec"); */
/* module_platform_driver(twl4030_codec_driver); */
/* MODULE_DESCRIPTION("ASoC TWL4030 codec driver"); */
/* MODULE_AUTHOR("Steve Sakoman"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
