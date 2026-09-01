// SPDX-License-Identifier: GPL-2.0-only
/*
 * cs42l51.c
 *
 * ASoC Driver for Cirrus Logic CS42L51 codecs
 *
 * Copyright (c) 2010 Arnaud Patard <apatard@mandriva.com>
 *
 * Based on cs4270.c - Copyright (c) Freescale Semiconductor
 *
 * For now:
 *  - Only I2C is support. Not SPI
 *  - master mode *NOT* supported
 */

// Rust translation of the isolated Linux codec source. Header-provided Linux,
// ASoC, regmap, GPIO, regulator, and cs42l51 symbols are external dependencies.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
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
    pub enumerated: snd_ctl_elem_value_enumerated,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
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
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
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
pub struct regulator_bulk_data {
    pub supply: *const c_char,
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
pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub no_capture_mute: c_uint,
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_ulong,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
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
    pub of_xlate_dai_id: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut device_node) -> c_int>,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}
#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub reg_stride: c_uint,
    pub val_bits: c_uint,
    pub use_single_write: bool,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub max_register: c_uint,
    pub cache_type: c_uint,
}

unsafe extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_new_controls(dapm: *mut snd_soc_dapm_context, widget: *const snd_soc_dapm_widget_desc, num: c_int) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn msleep(msecs: c_uint);
    fn mdelay(msecs: c_uint);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_regulator_bulk_get(dev: *mut device, num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_enable(num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn ERR_PTR(error: c_int) -> *mut c_void;
}

unsafe extern "C" {
    static CS42L51_POWER_CTL1: c_uint;
    static CS42L51_POWER_CTL1_PDN: c_uint;
    static CS42L51_MIC_POWER_CTL: c_uint;
    static CS42L51_MIC_POWER_CTL_SPEED_3: c_uint;
    static CS42L51_MIC_POWER_CTL_MCLK_DIV2: c_uint;
    static CS42L51_MIC_POWER_CTL_AUTO: c_uint;
    static CS42L51_INTF_CTL: c_uint;
    static CS42L51_INTF_CTL_MASTER: c_uint;
    static CS42L51_INTF_CTL_ADC_I2S: c_uint;
    static CS42L51_MIC_CTL: c_uint;
    static CS42L51_ADC_CTL: c_uint;
    static CS42L51_ADC_INPUT: c_uint;
    static CS42L51_DAC_OUT_CTL: c_uint;
    static CS42L51_DAC_OUT_CTL_DACA_MUTE: c_uint;
    static CS42L51_DAC_OUT_CTL_DACB_MUTE: c_uint;
    static CS42L51_DAC_CTL: c_uint;
    static CS42L51_DAC_CTL_AMUTE: c_uint;
    static CS42L51_ALC_PGA_CTL: c_uint;
    static CS42L51_ALC_PGB_CTL: c_uint;
    static CS42L51_ADCA_ATT: c_uint;
    static CS42L51_ADCB_ATT: c_uint;
    static CS42L51_ADCA_VOL: c_uint;
    static CS42L51_ADCB_VOL: c_uint;
    static CS42L51_PCMA_VOL: c_uint;
    static CS42L51_PCMB_VOL: c_uint;
    static CS42L51_BEEP_FREQ: c_uint;
    static CS42L51_BEEP_VOL: c_uint;
    static CS42L51_BEEP_CONF: c_uint;
    static CS42L51_TONE_CTL: c_uint;
    static CS42L51_AOUTA_VOL: c_uint;
    static CS42L51_AOUTB_VOL: c_uint;
    static CS42L51_PCM_MIXER: c_uint;
    static CS42L51_LIMIT_THRES_DIS: c_uint;
    static CS42L51_LIMIT_REL: c_uint;
    static CS42L51_LIMIT_ATT: c_uint;
    static CS42L51_ALC_EN: c_uint;
    static CS42L51_ALC_REL: c_uint;
    static CS42L51_ALC_THRES: c_uint;
    static CS42L51_NOISE_CONF: c_uint;
    static CS42L51_STATUS: c_uint;
    static CS42L51_CHARGE_FREQ: c_uint;
    static CS42L51_CHIP_REV_ID: c_uint;
    static CS42L51_CHIP_ID: c_uint;
    static CS42L51_CHIP_REV_A: c_uint;
    static CS42L51_CHIP_REV_B: c_uint;
    static CS42L51_CHIP_REV_MASK: c_uint;
    static CS42L51_QSM_MODE: c_uchar;
    static CS42L51_HSM_MODE: c_uchar;
    static CS42L51_SSM_MODE: c_uchar;
    static CS42L51_DSM_MODE: c_uchar;
    static CS42L51_DAC_DIF_I2S: c_int;
    static CS42L51_DAC_DIF_LJ24: c_int;
    static CS42L51_DAC_DIF_RJ16: c_int;
    static CS42L51_DAC_DIF_RJ18: c_int;
    static CS42L51_DAC_DIF_RJ20: c_int;
    static CS42L51_DAC_DIF_RJ24: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_PRE_POST_PMD: c_int;
    static SND_SOC_NOPM: c_int;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_ulong;
    static SNDRV_PCM_FMTBIT_S18_3LE: c_ulong;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_ulong;
    static SNDRV_PCM_FMTBIT_S24_LE: c_ulong;
    static SNDRV_PCM_RATE_8000_96000: c_uint;
    static GFP_KERNEL: c_uint;
    static GPIOD_OUT_LOW: c_uint;
    static REGCACHE_MAPLE: c_uint;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static ENODEV: c_int;
}

type c_uchar = u8;

unsafe fn CS42L51_INTF_CTL_DAC_FORMAT(x: c_int) -> c_uint {
    (x as c_uint) /* external cs42l51.h macro */
}
unsafe fn CS42L51_MIC_POWER_CTL_SPEED(x: c_uchar) -> c_uint {
    (x as c_uint) /* external cs42l51.h macro */
}
unsafe fn CS42L51_DAC_CTL_DATA_SEL(x: c_int) -> c_int {
    x /* external cs42l51.h macro */
}
unsafe fn CS42L51_DAC_CTL_DACSZ(x: c_int) -> c_int {
    x /* external cs42l51.h macro */
}
unsafe fn CS42L51_MK_CHIP_REV(chip: c_uint, rev: c_uint) -> c_uint {
    chip | rev /* external cs42l51.h macro */
}

#[repr(C)]
enum master_slave_mode {
    MODE_SLAVE,
    MODE_SLAVE_AUTO,
    MODE_MASTER,
}

static cs42l51_supply_names: [*const c_char; 4] = [
    b"VL\0".as_ptr() as *const c_char,
    b"VD\0".as_ptr() as *const c_char,
    b"VA\0".as_ptr() as *const c_char,
    b"VAHP\0".as_ptr() as *const c_char,
];

#[repr(C)]
struct cs42l51_private {
    mclk: c_uint,
    mclk_handle: *mut clk,
    audio_mode: c_uint, /* The mode (I2S or left-justified) */
    func: master_slave_mode,
    supplies: [regulator_bulk_data; 4],
    reset_gpio: *mut gpio_desc,
    regmap: *mut regmap,
}

unsafe fn CS42L51_FORMATS() -> c_ulong {
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S18_3LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE
}

unsafe extern "C" fn cs42l51_get_chan_mix(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let value: c_ulong = (snd_soc_component_read(component, CS42L51_PCM_MIXER) & 3) as c_ulong;

    match value {
        1 | 2 => {
            /* same value : (L+R)/2 and (R+L)/2 */
            (*ucontrol).value.enumerated.item[0] = 1;
        }
        3 => {
            (*ucontrol).value.enumerated.item[0] = 2;
        }
        _ => {
            (*ucontrol).value.enumerated.item[0] = 0;
        }
    }

    0
}

const CHAN_MIX_NORMAL: c_uchar = 0x00;
const CHAN_MIX_BOTH: c_uchar = 0x55;
const CHAN_MIX_SWAP: c_uchar = 0xFF;

unsafe extern "C" fn cs42l51_set_chan_mix(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let val: c_uchar;

    match (*ucontrol).value.enumerated.item[0] {
        1 => val = CHAN_MIX_BOTH,
        2 => val = CHAN_MIX_SWAP,
        _ => val = CHAN_MIX_NORMAL,
    }

    snd_soc_component_write(component, CS42L51_PCM_MIXER, val as c_uint);

    1
}

// static const DECLARE_TLV_DB_SCALE(adc_pcm_tlv, -5150, 50, 0);
// static const DECLARE_TLV_DB_SCALE(tone_tlv, -1050, 150, 0);
// static const DECLARE_TLV_DB_SCALE(aout_tlv, -10200, 50, 0);
// static const DECLARE_TLV_DB_SCALE(boost_tlv, 1600, 1600, 0);
// static const DECLARE_TLV_DB_SCALE(adc_boost_tlv, 2000, 2000, 0);
static chan_mix: [*const c_char; 3] = [
    b"L R\0".as_ptr() as *const c_char,
    b"L+R\0".as_ptr() as *const c_char,
    b"R L\0".as_ptr() as *const c_char,
];
// static const DECLARE_TLV_DB_SCALE(pga_tlv, -300, 50, 0);
// static const DECLARE_TLV_DB_SCALE(adc_att_tlv, -9600, 100, 0);
// static SOC_ENUM_SINGLE_EXT_DECL(cs42l51_chan_mix, chan_mix);
// static const struct snd_kcontrol_new cs42l51_snd_controls[] = { ... macro controls as in source };

/*
 * to power down, one must:
 * 1.) Enable the PDN bit
 * 2.) enable power-down for the select channels
 * 3.) disable the PDN bit.
 */
unsafe extern "C" fn cs42l51_pdn_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);

    if event == SND_SOC_DAPM_PRE_PMD {
        snd_soc_component_update_bits(component, CS42L51_POWER_CTL1, CS42L51_POWER_CTL1_PDN, CS42L51_POWER_CTL1_PDN);
    } else {
        snd_soc_component_update_bits(component, CS42L51_POWER_CTL1, CS42L51_POWER_CTL1_PDN, 0);
    }

    0
}

static cs42l51_dac_names: [*const c_char; 3] = [
    b"Direct PCM\0".as_ptr() as *const c_char,
    b"DSP PCM\0".as_ptr() as *const c_char,
    b"ADC\0".as_ptr() as *const c_char,
];
static cs42l51_adcl_names: [*const c_char; 4] = [
    b"AIN1 Left\0".as_ptr() as *const c_char,
    b"AIN2 Left\0".as_ptr() as *const c_char,
    b"MIC Left\0".as_ptr() as *const c_char,
    b"MIC+preamp Left\0".as_ptr() as *const c_char,
];
static cs42l51_adcr_names: [*const c_char; 4] = [
    b"AIN1 Right\0".as_ptr() as *const c_char,
    b"AIN2 Right\0".as_ptr() as *const c_char,
    b"MIC Right\0".as_ptr() as *const c_char,
    b"MIC+preamp Right\0".as_ptr() as *const c_char,
];
// DAPM enum/control/widget tables are macro generated by ASoC in C and remain external translation dependencies.
static cs42l51_dapm_widgets: [snd_soc_dapm_widget_desc; 0] = [];

unsafe extern "C" fn mclk_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let comp = snd_soc_dapm_to_component((*w).dapm);
    let cs42l51 = snd_soc_component_get_drvdata(comp) as *mut cs42l51_private;

    if event == SND_SOC_DAPM_PRE_PMU {
        return clk_prepare_enable((*cs42l51).mclk_handle);
    } else if event == SND_SOC_DAPM_POST_PMD {
        /* Delay mclk shutdown to fulfill power-down sequence requirements */
        msleep(20);
        clk_disable_unprepare((*cs42l51).mclk_handle);
    }

    0
}

static cs42l51_dapm_mclk_widgets: [snd_soc_dapm_widget_desc; 0] = [];

static cs42l51_routes: [snd_soc_dapm_route; 20] = [
    snd_soc_dapm_route { sink: b"HPL\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Left DAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"HPR\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Right DAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right DAC\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"DAC Mux\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Left DAC\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"DAC Mux\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DAC Mux\0".as_ptr() as *const c_char, control: b"Direct PCM\0".as_ptr() as *const c_char, source: b"Playback\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DAC Mux\0".as_ptr() as *const c_char, control: b"DSP PCM\0".as_ptr() as *const c_char, source: b"Playback\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Left ADC\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Left PGA\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right ADC\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Right PGA\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Mic Preamp Left\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"MICL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Mic Preamp Right\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"MICR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"PGA-ADC Mux Left\0".as_ptr() as *const c_char, control: b"AIN1 Left\0".as_ptr() as *const c_char, source: b"AIN1L\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"PGA-ADC Mux Left\0".as_ptr() as *const c_char, control: b"AIN2 Left\0".as_ptr() as *const c_char, source: b"AIN2L\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"PGA-ADC Mux Left\0".as_ptr() as *const c_char, control: b"MIC Left\0".as_ptr() as *const c_char, source: b"MICL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"PGA-ADC Mux Left\0".as_ptr() as *const c_char, control: b"MIC+preamp Left\0".as_ptr() as *const c_char, source: b"Mic Preamp Left\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"PGA-ADC Mux Right\0".as_ptr() as *const c_char, control: b"AIN1 Right\0".as_ptr() as *const c_char, source: b"AIN1R\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"PGA-ADC Mux Right\0".as_ptr() as *const c_char, control: b"AIN2 Right\0".as_ptr() as *const c_char, source: b"AIN2R\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"PGA-ADC Mux Right\0".as_ptr() as *const c_char, control: b"MIC Right\0".as_ptr() as *const c_char, source: b"MICR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"PGA-ADC Mux Right\0".as_ptr() as *const c_char, control: b"MIC+preamp Right\0".as_ptr() as *const c_char, source: b"Mic Preamp Right\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Left PGA\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"PGA-ADC Mux Left\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right PGA\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"PGA-ADC Mux Right\0".as_ptr() as *const c_char },
];

unsafe extern "C" fn cs42l51_set_dai_fmt(codec_dai: *mut snd_soc_dai, format: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let cs42l51 = snd_soc_component_get_drvdata(component) as *mut cs42l51_private;

    match format & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S || x == SND_SOC_DAIFMT_LEFT_J || x == SND_SOC_DAIFMT_RIGHT_J => (*cs42l51).audio_mode = format & SND_SOC_DAIFMT_FORMAT_MASK,
        _ => {
            dev_err((*component).dev, b"invalid DAI format\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    match format & SND_SOC_DAIFMT_MASTER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP => (*cs42l51).func = master_slave_mode::MODE_MASTER,
        x if x == SND_SOC_DAIFMT_CBC_CFC => (*cs42l51).func = master_slave_mode::MODE_SLAVE_AUTO,
        _ => {
            dev_err((*component).dev, b"Unknown master/slave configuration\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    0
}

#[repr(C)]
#[derive(Copy, Clone)]
struct cs42l51_ratios {
    ratio: c_uint,
    speed_mode: c_uchar,
    mclk: c_uchar,
}

static mut slave_ratios: [cs42l51_ratios; 22] = [
    cs42l51_ratios { ratio: 512, speed_mode: unsafe { CS42L51_QSM_MODE }, mclk: 0 }, cs42l51_ratios { ratio: 768, speed_mode: unsafe { CS42L51_QSM_MODE }, mclk: 0 },
    cs42l51_ratios { ratio: 1024, speed_mode: unsafe { CS42L51_QSM_MODE }, mclk: 0 }, cs42l51_ratios { ratio: 1536, speed_mode: unsafe { CS42L51_QSM_MODE }, mclk: 0 },
    cs42l51_ratios { ratio: 2048, speed_mode: unsafe { CS42L51_QSM_MODE }, mclk: 0 }, cs42l51_ratios { ratio: 3072, speed_mode: unsafe { CS42L51_QSM_MODE }, mclk: 0 },
    cs42l51_ratios { ratio: 256, speed_mode: unsafe { CS42L51_HSM_MODE }, mclk: 0 }, cs42l51_ratios { ratio: 384, speed_mode: unsafe { CS42L51_HSM_MODE }, mclk: 0 },
    cs42l51_ratios { ratio: 512, speed_mode: unsafe { CS42L51_HSM_MODE }, mclk: 0 }, cs42l51_ratios { ratio: 768, speed_mode: unsafe { CS42L51_HSM_MODE }, mclk: 0 },
    cs42l51_ratios { ratio: 1024, speed_mode: unsafe { CS42L51_HSM_MODE }, mclk: 0 }, cs42l51_ratios { ratio: 1536, speed_mode: unsafe { CS42L51_HSM_MODE }, mclk: 0 },
    cs42l51_ratios { ratio: 128, speed_mode: unsafe { CS42L51_SSM_MODE }, mclk: 0 }, cs42l51_ratios { ratio: 192, speed_mode: unsafe { CS42L51_SSM_MODE }, mclk: 0 },
    cs42l51_ratios { ratio: 256, speed_mode: unsafe { CS42L51_SSM_MODE }, mclk: 0 }, cs42l51_ratios { ratio: 384, speed_mode: unsafe { CS42L51_SSM_MODE }, mclk: 0 },
    cs42l51_ratios { ratio: 512, speed_mode: unsafe { CS42L51_SSM_MODE }, mclk: 0 }, cs42l51_ratios { ratio: 768, speed_mode: unsafe { CS42L51_SSM_MODE }, mclk: 0 },
    cs42l51_ratios { ratio: 128, speed_mode: unsafe { CS42L51_DSM_MODE }, mclk: 0 }, cs42l51_ratios { ratio: 192, speed_mode: unsafe { CS42L51_DSM_MODE }, mclk: 0 },
    cs42l51_ratios { ratio: 256, speed_mode: unsafe { CS42L51_DSM_MODE }, mclk: 0 }, cs42l51_ratios { ratio: 384, speed_mode: unsafe { CS42L51_DSM_MODE }, mclk: 0 },
];

static mut slave_auto_ratios: [cs42l51_ratios; 16] = [
    cs42l51_ratios { ratio: 1024, speed_mode: unsafe { CS42L51_QSM_MODE }, mclk: 0 }, cs42l51_ratios { ratio: 1536, speed_mode: unsafe { CS42L51_QSM_MODE }, mclk: 0 },
    cs42l51_ratios { ratio: 2048, speed_mode: unsafe { CS42L51_QSM_MODE }, mclk: 1 }, cs42l51_ratios { ratio: 3072, speed_mode: unsafe { CS42L51_QSM_MODE }, mclk: 1 },
    cs42l51_ratios { ratio: 512, speed_mode: unsafe { CS42L51_HSM_MODE }, mclk: 0 }, cs42l51_ratios { ratio: 768, speed_mode: unsafe { CS42L51_HSM_MODE }, mclk: 0 },
    cs42l51_ratios { ratio: 1024, speed_mode: unsafe { CS42L51_HSM_MODE }, mclk: 1 }, cs42l51_ratios { ratio: 1536, speed_mode: unsafe { CS42L51_HSM_MODE }, mclk: 1 },
    cs42l51_ratios { ratio: 256, speed_mode: unsafe { CS42L51_SSM_MODE }, mclk: 0 }, cs42l51_ratios { ratio: 384, speed_mode: unsafe { CS42L51_SSM_MODE }, mclk: 0 },
    cs42l51_ratios { ratio: 512, speed_mode: unsafe { CS42L51_SSM_MODE }, mclk: 1 }, cs42l51_ratios { ratio: 768, speed_mode: unsafe { CS42L51_SSM_MODE }, mclk: 1 },
    cs42l51_ratios { ratio: 128, speed_mode: unsafe { CS42L51_DSM_MODE }, mclk: 0 }, cs42l51_ratios { ratio: 192, speed_mode: unsafe { CS42L51_DSM_MODE }, mclk: 0 },
    cs42l51_ratios { ratio: 256, speed_mode: unsafe { CS42L51_DSM_MODE }, mclk: 1 }, cs42l51_ratios { ratio: 384, speed_mode: unsafe { CS42L51_DSM_MODE }, mclk: 1 },
];

/*
 * Master mode mclk/fs ratios.
 * Recommended configurations are SSM for 4-50khz and DSM for 50-100kHz ranges
 * The table below provides support of following ratios:
 * 128: SSM (%128) with div2 disabled
 * 256: SSM (%128) with div2 enabled
 * In both cases, if sampling rate is above 50kHz, SSM is overridden
 * with DSM (%128) configuration
 */
static mut master_ratios: [cs42l51_ratios; 2] = [
    cs42l51_ratios { ratio: 128, speed_mode: unsafe { CS42L51_SSM_MODE }, mclk: 0 },
    cs42l51_ratios { ratio: 256, speed_mode: unsafe { CS42L51_SSM_MODE }, mclk: 1 },
];

unsafe extern "C" fn cs42l51_set_dai_sysclk(codec_dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*codec_dai).component;
    let cs42l51 = snd_soc_component_get_drvdata(component) as *mut cs42l51_private;

    (*cs42l51).mclk = freq;
    0
}

unsafe extern "C" fn cs42l51_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let cs42l51 = snd_soc_component_get_drvdata(component) as *mut cs42l51_private;
    let mut ret: c_int;
    let mut i: c_uint;
    let rate: c_uint;
    let ratio: c_uint;
    let mut ratios: *mut cs42l51_ratios = core::ptr::null_mut();
    let mut nr_ratios: c_int = 0;
    let mut intf_ctl: c_int;
    let mut power_ctl: c_int;
    let fmt: c_int;
    let mut mode: c_uchar;

    match (*cs42l51).func {
        master_slave_mode::MODE_MASTER => {
            ratios = master_ratios.as_mut_ptr();
            nr_ratios = master_ratios.len() as c_int;
        }
        master_slave_mode::MODE_SLAVE => {
            ratios = slave_ratios.as_mut_ptr();
            nr_ratios = slave_ratios.len() as c_int;
        }
        master_slave_mode::MODE_SLAVE_AUTO => {
            ratios = slave_auto_ratios.as_mut_ptr();
            nr_ratios = slave_auto_ratios.len() as c_int;
        }
    }

    /* Figure out which MCLK/LRCK ratio to use */
    rate = params_rate(params);     /* Sampling rate, in Hz */
    ratio = (*cs42l51).mclk / rate;    /* MCLK/LRCK ratio */
    i = 0;
    while i < nr_ratios as c_uint {
        if (*ratios.add(i as usize)).ratio == ratio {
            break;
        }
        i += 1;
    }

    if i == nr_ratios as c_uint {
        /* We did not find a matching ratio */
        dev_err((*component).dev, b"could not find matching ratio\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    intf_ctl = snd_soc_component_read(component, CS42L51_INTF_CTL) as c_int;
    power_ctl = snd_soc_component_read(component, CS42L51_MIC_POWER_CTL) as c_int;

    intf_ctl &= !((CS42L51_INTF_CTL_MASTER | CS42L51_INTF_CTL_ADC_I2S | CS42L51_INTF_CTL_DAC_FORMAT(7)) as c_int);
    power_ctl &= !((CS42L51_MIC_POWER_CTL_SPEED(3) | CS42L51_MIC_POWER_CTL_MCLK_DIV2) as c_int);

    match (*cs42l51).func {
        master_slave_mode::MODE_MASTER => {
            intf_ctl |= CS42L51_INTF_CTL_MASTER as c_int;
            mode = (*ratios.add(i as usize)).speed_mode;
            /* Force DSM mode if sampling rate is above 50kHz */
            if rate > 50000 {
                mode = CS42L51_DSM_MODE;
            }
            power_ctl |= CS42L51_MIC_POWER_CTL_SPEED(mode) as c_int;
            /*
             * Auto detect mode is not applicable for master mode and has to
             * be disabled. Otherwise SPEED[1:0] bits will be ignored.
             */
            power_ctl &= !(CS42L51_MIC_POWER_CTL_AUTO as c_int);
        }
        master_slave_mode::MODE_SLAVE => {
            power_ctl |= CS42L51_MIC_POWER_CTL_SPEED((*ratios.add(i as usize)).speed_mode) as c_int;
        }
        master_slave_mode::MODE_SLAVE_AUTO => {
            power_ctl |= CS42L51_MIC_POWER_CTL_AUTO as c_int;
        }
    }

    match (*cs42l51).audio_mode {
        x if x == SND_SOC_DAIFMT_I2S => {
            intf_ctl |= CS42L51_INTF_CTL_ADC_I2S as c_int;
            intf_ctl |= CS42L51_INTF_CTL_DAC_FORMAT(CS42L51_DAC_DIF_I2S) as c_int;
        }
        x if x == SND_SOC_DAIFMT_LEFT_J => {
            intf_ctl |= CS42L51_INTF_CTL_DAC_FORMAT(CS42L51_DAC_DIF_LJ24) as c_int;
        }
        x if x == SND_SOC_DAIFMT_RIGHT_J => {
            fmt = match params_width(params) {
                16 => CS42L51_DAC_DIF_RJ16,
                18 => CS42L51_DAC_DIF_RJ18,
                20 => CS42L51_DAC_DIF_RJ20,
                24 => CS42L51_DAC_DIF_RJ24,
                _ => {
                    dev_err((*component).dev, b"unknown format\n\0".as_ptr() as *const c_char);
                    return -EINVAL;
                }
            };
            intf_ctl |= CS42L51_INTF_CTL_DAC_FORMAT(fmt) as c_int;
        }
        _ => {
            dev_err((*component).dev, b"unknown format\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    if (*ratios.add(i as usize)).mclk != 0 {
        power_ctl |= CS42L51_MIC_POWER_CTL_MCLK_DIV2 as c_int;
    }

    ret = snd_soc_component_write(component, CS42L51_INTF_CTL, intf_ctl as c_uint);
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_component_write(component, CS42L51_MIC_POWER_CTL, power_ctl as c_uint);
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn cs42l51_dai_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*dai).component;
    let mut reg: c_int;
    let mask: c_int = (CS42L51_DAC_OUT_CTL_DACA_MUTE | CS42L51_DAC_OUT_CTL_DACB_MUTE) as c_int;

    reg = snd_soc_component_read(component, CS42L51_DAC_OUT_CTL) as c_int;

    if mute != 0 {
        reg |= mask;
    } else {
        reg &= !mask;
    }

    snd_soc_component_write(component, CS42L51_DAC_OUT_CTL, reg as c_uint)
}

unsafe extern "C" fn cs42l51_of_xlate_dai_id(_component: *mut snd_soc_component, _endpoint: *mut device_node) -> c_int {
    /* return dai id 0, whatever the endpoint index */
    0
}

static cs42l51_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(cs42l51_hw_params),
    set_sysclk: Some(cs42l51_set_dai_sysclk),
    set_fmt: Some(cs42l51_set_dai_fmt),
    mute_stream: Some(cs42l51_dai_mute),
    no_capture_mute: 1,
};

static mut cs42l51_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"cs42l51-hifi\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: unsafe { SNDRV_PCM_RATE_8000_96000 },
        formats: unsafe { CS42L51_FORMATS() },
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: unsafe { SNDRV_PCM_RATE_8000_96000 },
        formats: unsafe { CS42L51_FORMATS() },
    },
    ops: &cs42l51_dai_ops,
};

unsafe extern "C" fn cs42l51_component_probe(component: *mut snd_soc_component) -> c_int {
    let mut ret: c_int;
    let reg: c_int;
    let dapm: *mut snd_soc_dapm_context;
    let cs42l51: *mut cs42l51_private;

    cs42l51 = snd_soc_component_get_drvdata(component) as *mut cs42l51_private;
    dapm = snd_soc_component_to_dapm(component);

    if !(*cs42l51).mclk_handle.is_null() {
        snd_soc_dapm_new_controls(dapm, cs42l51_dapm_mclk_widgets.as_ptr(), 1);
    }

    /*
     * DAC configuration
     * - Use signal processor
     * - auto mute
     * - vol changes immediate
     * - no de-emphasize
     */
    reg = CS42L51_DAC_CTL_DATA_SEL(1) | CS42L51_DAC_CTL_AMUTE as c_int | CS42L51_DAC_CTL_DACSZ(0);
    ret = snd_soc_component_write(component, CS42L51_DAC_CTL, reg as c_uint);
    if ret < 0 {
        return ret;
    }

    0
}

static soc_component_device_cs42l51: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(cs42l51_component_probe),
    controls: core::ptr::null(),
    num_controls: 0,
    dapm_widgets: cs42l51_dapm_widgets.as_ptr(),
    num_dapm_widgets: cs42l51_dapm_widgets.len() as c_uint,
    dapm_routes: cs42l51_routes.as_ptr(),
    num_dapm_routes: cs42l51_routes.len() as c_uint,
    of_xlate_dai_id: Some(cs42l51_of_xlate_dai_id),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn cs42l51_writeable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        x if x == CS42L51_POWER_CTL1 || x == CS42L51_MIC_POWER_CTL || x == CS42L51_INTF_CTL || x == CS42L51_MIC_CTL ||
             x == CS42L51_ADC_CTL || x == CS42L51_ADC_INPUT || x == CS42L51_DAC_OUT_CTL || x == CS42L51_DAC_CTL ||
             x == CS42L51_ALC_PGA_CTL || x == CS42L51_ALC_PGB_CTL || x == CS42L51_ADCA_ATT || x == CS42L51_ADCB_ATT ||
             x == CS42L51_ADCA_VOL || x == CS42L51_ADCB_VOL || x == CS42L51_PCMA_VOL || x == CS42L51_PCMB_VOL ||
             x == CS42L51_BEEP_FREQ || x == CS42L51_BEEP_VOL || x == CS42L51_BEEP_CONF || x == CS42L51_TONE_CTL ||
             x == CS42L51_AOUTA_VOL || x == CS42L51_AOUTB_VOL || x == CS42L51_PCM_MIXER || x == CS42L51_LIMIT_THRES_DIS ||
             x == CS42L51_LIMIT_REL || x == CS42L51_LIMIT_ATT || x == CS42L51_ALC_EN || x == CS42L51_ALC_REL ||
             x == CS42L51_ALC_THRES || x == CS42L51_NOISE_CONF || x == CS42L51_CHARGE_FREQ => true,
        _ => false,
    }
}

unsafe extern "C" fn cs42l51_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        x if x == CS42L51_STATUS => true,
        _ => false,
    }
}

unsafe extern "C" fn cs42l51_readable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        x if x == CS42L51_CHIP_REV_ID || x == CS42L51_POWER_CTL1 || x == CS42L51_MIC_POWER_CTL || x == CS42L51_INTF_CTL ||
             x == CS42L51_MIC_CTL || x == CS42L51_ADC_CTL || x == CS42L51_ADC_INPUT || x == CS42L51_DAC_OUT_CTL ||
             x == CS42L51_DAC_CTL || x == CS42L51_ALC_PGA_CTL || x == CS42L51_ALC_PGB_CTL || x == CS42L51_ADCA_ATT ||
             x == CS42L51_ADCB_ATT || x == CS42L51_ADCA_VOL || x == CS42L51_ADCB_VOL || x == CS42L51_PCMA_VOL ||
             x == CS42L51_PCMB_VOL || x == CS42L51_BEEP_FREQ || x == CS42L51_BEEP_VOL || x == CS42L51_BEEP_CONF ||
             x == CS42L51_TONE_CTL || x == CS42L51_AOUTA_VOL || x == CS42L51_AOUTB_VOL || x == CS42L51_PCM_MIXER ||
             x == CS42L51_LIMIT_THRES_DIS || x == CS42L51_LIMIT_REL || x == CS42L51_LIMIT_ATT || x == CS42L51_ALC_EN ||
             x == CS42L51_ALC_REL || x == CS42L51_ALC_THRES || x == CS42L51_NOISE_CONF || x == CS42L51_STATUS ||
             x == CS42L51_CHARGE_FREQ => true,
        _ => false,
    }
}

#[no_mangle]
pub static cs42l51_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    reg_stride: 1,
    val_bits: 8,
    use_single_write: true,
    readable_reg: Some(cs42l51_readable_reg),
    volatile_reg: Some(cs42l51_volatile_reg),
    writeable_reg: Some(cs42l51_writeable_reg),
    max_register: unsafe { CS42L51_CHARGE_FREQ },
    cache_type: unsafe { REGCACHE_MAPLE },
};
// EXPORT_SYMBOL_GPL(cs42l51_regmap);

#[no_mangle]
pub unsafe extern "C" fn cs42l51_probe(dev: *mut device, regmap: *mut regmap) -> c_int {
    let cs42l51: *mut cs42l51_private;
    let mut val: c_uint = 0;
    let mut ret: c_int;
    let mut i: c_int;

    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    cs42l51 = devm_kzalloc(dev, core::mem::size_of::<cs42l51_private>(), GFP_KERNEL) as *mut cs42l51_private;
    if cs42l51.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, cs42l51 as *mut c_void);
    (*cs42l51).regmap = regmap;

    (*cs42l51).mclk_handle = devm_clk_get_optional(dev, b"MCLK\0".as_ptr() as *const c_char);
    if IS_ERR((*cs42l51).mclk_handle as *const c_void) {
        return PTR_ERR((*cs42l51).mclk_handle as *const c_void);
    }

    i = 0;
    while i < (*cs42l51).supplies.len() as c_int {
        (*cs42l51).supplies[i as usize].supply = cs42l51_supply_names[i as usize];
        i += 1;
    }

    ret = devm_regulator_bulk_get(dev, (*cs42l51).supplies.len() as c_int, (*cs42l51).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, b"Failed to request supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = regulator_bulk_enable((*cs42l51).supplies.len() as c_int, (*cs42l51).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, b"Failed to enable supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    (*cs42l51).reset_gpio = devm_gpiod_get_optional(dev, b"reset\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
    if IS_ERR((*cs42l51).reset_gpio as *const c_void) {
        ret = PTR_ERR((*cs42l51).reset_gpio as *const c_void);
        goto_error(dev, cs42l51, ret)
    } else {
        if !(*cs42l51).reset_gpio.is_null() {
            dev_dbg(dev, b"Release reset gpio\n\0".as_ptr() as *const c_char);
            gpiod_set_value_cansleep((*cs42l51).reset_gpio, 0);
            mdelay(2);
        }

        /* Verify that we have a CS42L51 */
        ret = regmap_read(regmap, CS42L51_CHIP_REV_ID, &mut val);
        if ret < 0 {
            dev_err(dev, b"failed to read I2C\n\0".as_ptr() as *const c_char);
            return goto_error(dev, cs42l51, ret);
        }

        if val != CS42L51_MK_CHIP_REV(CS42L51_CHIP_ID, CS42L51_CHIP_REV_A) &&
           val != CS42L51_MK_CHIP_REV(CS42L51_CHIP_ID, CS42L51_CHIP_REV_B) {
            dev_err(dev, b"Invalid chip id: %x\n\0".as_ptr() as *const c_char, val);
            ret = -ENODEV;
            return goto_error(dev, cs42l51, ret);
        }
        dev_info(dev, b"Cirrus Logic CS42L51, Revision: %02X\n\0".as_ptr() as *const c_char, val & CS42L51_CHIP_REV_MASK);

        ret = devm_snd_soc_register_component(dev, &soc_component_device_cs42l51, &mut cs42l51_dai, 1);
        if ret < 0 {
            return goto_error(dev, cs42l51, ret);
        }

        0
    }
}

unsafe fn goto_error(dev: *mut device, cs42l51: *mut cs42l51_private, ret: c_int) -> c_int {
    gpiod_set_value_cansleep((*cs42l51).reset_gpio, 1);
    regulator_bulk_disable((*cs42l51).supplies.len() as c_int, (*cs42l51).supplies.as_mut_ptr());
    ret
}
// EXPORT_SYMBOL_GPL(cs42l51_probe);

#[no_mangle]
pub unsafe extern "C" fn cs42l51_remove(dev: *mut device) {
    let cs42l51 = dev_get_drvdata(dev) as *mut cs42l51_private;
    let ret: c_int;

    gpiod_set_value_cansleep((*cs42l51).reset_gpio, 1);

    ret = regulator_bulk_disable((*cs42l51).supplies.len() as c_int, (*cs42l51).supplies.as_mut_ptr());
    if ret != 0 {
        dev_warn(dev, b"Failed to disable all regulators (%pe)\n\0".as_ptr() as *const c_char, ERR_PTR(ret));
    }
}
// EXPORT_SYMBOL_GPL(cs42l51_remove);

#[no_mangle]
pub unsafe extern "C" fn cs42l51_suspend(dev: *mut device) -> c_int {
    let cs42l51 = dev_get_drvdata(dev) as *mut cs42l51_private;

    regcache_cache_only((*cs42l51).regmap, true);
    regcache_mark_dirty((*cs42l51).regmap);

    0
}
// EXPORT_SYMBOL_GPL(cs42l51_suspend);

#[no_mangle]
pub unsafe extern "C" fn cs42l51_resume(dev: *mut device) -> c_int {
    let cs42l51 = dev_get_drvdata(dev) as *mut cs42l51_private;

    regcache_cache_only((*cs42l51).regmap, false);

    regcache_sync((*cs42l51).regmap)
}
// EXPORT_SYMBOL_GPL(cs42l51_resume);

// MODULE_AUTHOR("Arnaud Patard <arnaud.patard@rtp-net.org>");
// MODULE_DESCRIPTION("Cirrus Logic CS42L51 ALSA SoC Codec Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
