// SPDX-License-Identifier: GPL-2.0-only
/*
 * es8316.rs -- es8316 ALSA SoC audio driver
 * Copyright Everest Semiconductor Co.,Ltd
 *
 * Authors: David Yang <yangxiaohua@everest-semi.com>,
 *          Daniel Drake <drake@endlessm.com>
 */

// Translated from soc/codecs/es8316.c.  C includes were:
// linux/module.h, linux/acpi.h, linux/cleanup.h, linux/clk.h, linux/delay.h,
// linux/i2c.h, linux/mutex.h, linux/regmap.h, linux/regulator/consumer.h,
// sound/pcm.h, sound/pcm_params.h, sound/soc.h, sound/soc-dapm.h,
// sound/tlv.h, sound/jack.h, and "es8316.h".

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type U8 = u8;
type U16 = u16;
type IrqReturnT = c_uint;

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
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
pub struct snd_soc_jack {
    pub status: c_uint,
}
#[repr(C)]
pub struct i2c_client {
    pub dev: device,
    pub irq: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *mut c_uint,
    pub mask: c_uint,
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
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
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
    pub formats: u64,
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
    pub set_jack: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_jack, *mut c_void) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}
#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub use_single_read: bool,
    pub use_single_write: bool,
    pub max_register: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
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
    pub id_table: *const i2c_device_id,
}

unsafe extern "C" {
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn devm_clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_mutex_lock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_mutex_unlock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_force_enable_pin_unlocked(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_disable_pin_unlocked(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_sync_unlocked(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_uint, l: *mut snd_pcm_hw_constraint_list) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_uint;
    fn msleep(msecs: c_uint);
    fn usleep_range(min: c_uint, max: c_uint);
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_uint, mask: c_uint);
    fn device_property_read_bool(dev: *mut device, propname: *const c_char) -> bool;
    fn enable_irq(irq: c_int);
    fn disable_irq(irq: c_int);
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn devm_regulator_bulk_get_enable(dev: *mut device, num: c_uint, supplies: *const *const c_char) -> c_int;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn devm_request_threaded_irq(
        dev: *mut device,
        irq: c_int,
        handler: *const c_void,
        thread_fn: Option<unsafe extern "C" fn(c_int, *mut c_void) -> IrqReturnT>,
        flags: c_uint,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
}

extern "Rust" {
    static ES8316_RESET: c_uint;
    static ES8316_RESET_CSM_ON: c_uint;
    static ES8316_CLKMGR_CLKSW: c_uint;
    static ES8316_CLKMGR_CLKSW_MCLK_ON: c_uint;
    static ES8316_CLKMGR_CLKSW_BCLK_ON: c_uint;
    static ES8316_CLKMGR_CLKSW_MCLK_DIV: c_uint;
    static ES8316_CLKMGR_ADCDIV1: c_uint;
    static ES8316_CLKMGR_ADCDIV2: c_uint;
    static ES8316_CLKMGR_DACDIV1: c_uint;
    static ES8316_CLKMGR_DACDIV2: c_uint;
    static ES8316_CLKMGR_ADCOSR: c_uint;
    static ES8316_SYS_PDN: c_uint;
    static ES8316_SYS_VMIDSEL: c_uint;
    static ES8316_SERDATA1: c_uint;
    static ES8316_SERDATA1_MASTER: c_uint;
    static ES8316_SERDATA1_BCLK_INV: c_uint;
    static ES8316_SERDATA_ADC: c_uint;
    static ES8316_SERDATA_DAC: c_uint;
    static ES8316_SERDATA2_ADCLRP: c_uint;
    static ES8316_SERDATA2_FMT_MASK: c_uint;
    static ES8316_SERDATA2_LEN_MASK: c_uint;
    static ES8316_SERDATA2_LEN_16: c_uint;
    static ES8316_SERDATA2_LEN_20: c_uint;
    static ES8316_SERDATA2_LEN_24: c_uint;
    static ES8316_SERDATA2_LEN_32: c_uint;
    static ES8316_DAC_SET1: c_uint;
    static ES8316_DAC_SET2: c_uint;
    static ES8316_DAC_SET3: c_uint;
    static ES8316_DAC_VOLL: c_uint;
    static ES8316_DAC_VOLR: c_uint;
    static ES8316_DAC_PDN: c_uint;
    static ES8316_ADC_MUTE: c_uint;
    static ES8316_ADC_D2SEPGA: c_uint;
    static ES8316_ADC_VOLUME: c_uint;
    static ES8316_ADC_PGAGAIN: c_uint;
    static ES8316_ADC_DMIC: c_uint;
    static ES8316_ADC_ALC1: c_uint;
    static ES8316_ADC_ALC2: c_uint;
    static ES8316_ADC_ALC3: c_uint;
    static ES8316_ADC_ALC4: c_uint;
    static ES8316_ADC_ALC_NG: c_uint;
    static ES8316_ADC_PDN_LINSEL: c_uint;
    static ES8316_HPMIX_SEL: c_uint;
    static ES8316_HPMIX_SWITCH: c_uint;
    static ES8316_HPMIX_VOL: c_uint;
    static ES8316_HPMIX_PDN: c_uint;
    static ES8316_CPHP_ICAL_VOL: c_uint;
    static ES8316_CPHP_OUTEN: c_uint;
    static ES8316_CPHP_PDN1: c_uint;
    static ES8316_CPHP_PDN2: c_uint;
    static ES8316_GPIO_FLAG: c_uint;
    static ES8316_GPIO_FLAG_HP_NOT_INSERTED: c_uint;
    static ES8316_GPIO_FLAG_GM_NOT_SHORTED: c_uint;
    static ES8316_GPIO_DEBOUNCE: c_uint;
    static ES8316_GPIO_ENABLE_INTERRUPT: c_uint;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SNDRV_PCM_HW_PARAM_RATE: c_uint;
    static SNDRV_PCM_FORMAT_S16_LE: c_uint;
    static SNDRV_PCM_FORMAT_S20_3LE: c_uint;
    static SNDRV_PCM_FORMAT_S24_LE: c_uint;
    static SNDRV_PCM_FORMAT_S24_3LE: c_uint;
    static SNDRV_PCM_FORMAT_S32_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S20_3LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SND_SOC_NOPM: c_uint;
    static SND_JACK_MICROPHONE: c_uint;
    static SND_JACK_HEADPHONE: c_uint;
    static SND_JACK_HEADSET: c_uint;
    static SND_JACK_BTN_0: c_uint;
    static IRQ_HANDLED: IrqReturnT;
    static IRQF_TRIGGER_HIGH: c_uint;
    static IRQF_ONESHOT: c_uint;
    static IRQF_NO_AUTOEN: c_uint;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static ENXIO: c_int;
    static GFP_KERNEL: c_uint;
    static REGCACHE_MAPLE: c_uint;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! array_size {
    ($a:expr) => {
        $a.len() as c_uint
    };
}

#[repr(C)]
struct es8316_priv {
    lock: mutex,
    mclk: *mut clk,
    regmap: *mut regmap,
    component: *mut snd_soc_component,
    jack: *mut snd_soc_jack,
    irq: c_int,
    sysclk: c_uint,
    /* ES83xx supports halving the MCLK so it supports twice as many rates
     */
    allowed_rates: [c_uint; SUPPORTED_MCLK_LRCK_RATIOS.len() * 2],
    sysclk_constraints: snd_pcm_hw_constraint_list,
    jd_inverted: bool,
}

/* In slave mode at single speed, the codec is documented as accepting 5
 * MCLK/LRCK ratios, but we also add ratio 400, which is commonly used on
 * Intel Cherry Trail platforms (19.2MHz MCLK, 48kHz LRCK).
 */
static SUPPORTED_MCLK_LRCK_RATIOS: [c_uint; 7] = [256, 384, 400, 500, 512, 768, 1024];

static ES8316_SUPPLY_NAMES: [*const c_char; 4] = [
    cstr!("avdd"),
    cstr!("cpvdd"),
    cstr!("dvdd"),
    cstr!("pvdd"),
];

/*
 * ES8316 controls
 */
static DAC_VOL_TLV: &[c_uint] = SNDRV_CTL_TLVD_DECLARE_DB_SCALE!(-9600, 50, 1);
static ADC_VOL_TLV: &[c_uint] = SNDRV_CTL_TLVD_DECLARE_DB_SCALE!(-9600, 50, 1);
static ALC_MAX_GAIN_TLV: &[c_uint] = SNDRV_CTL_TLVD_DECLARE_DB_SCALE!(-650, 150, 0);
static ALC_MIN_GAIN_TLV: &[c_uint] = SNDRV_CTL_TLVD_DECLARE_DB_SCALE!(-1200, 150, 0);

static ALC_TARGET_TLV: &[c_uint] = SNDRV_CTL_TLVD_DECLARE_DB_RANGE!(
    0, 10, TLV_DB_SCALE_ITEM!(-1650, 150, 0),
    11, 11, TLV_DB_SCALE_ITEM!(-150, 0, 0),
);

static HPMIXER_GAIN_TLV: &[c_uint] = SNDRV_CTL_TLVD_DECLARE_DB_RANGE!(
    0, 4, TLV_DB_SCALE_ITEM!(-1200, 150, 0),
    8, 11, TLV_DB_SCALE_ITEM!(-450, 150, 0),
);

static ADC_PGA_GAIN_TLV: &[c_uint] = SNDRV_CTL_TLVD_DECLARE_DB_RANGE!(
    0, 0, TLV_DB_SCALE_ITEM!(-350, 0, 0),
    1, 1, TLV_DB_SCALE_ITEM!(0, 0, 0),
    2, 2, TLV_DB_SCALE_ITEM!(250, 0, 0),
    3, 3, TLV_DB_SCALE_ITEM!(450, 0, 0),
    4, 7, TLV_DB_SCALE_ITEM!(700, 300, 0),
    8, 10, TLV_DB_SCALE_ITEM!(1800, 300, 0),
);

static HPOUT_VOL_TLV: &[c_uint] = SNDRV_CTL_TLVD_DECLARE_DB_RANGE!(
    0, 0, TLV_DB_SCALE_ITEM!(-4800, 0, 0),
    1, 3, TLV_DB_SCALE_ITEM!(-2400, 1200, 0),
);

static NG_TYPE_TXT: [*const c_char; 2] = [cstr!("Constant PGA Gain"), cstr!("Mute ADC Output")];
static NG_TYPE: soc_enum = SOC_ENUM_SINGLE!(ES8316_ADC_ALC_NG, 6, 2, NG_TYPE_TXT);

static ADCPOL_TXT: [*const c_char; 2] = [cstr!("Normal"), cstr!("Invert")];
static ADCPOL: soc_enum = SOC_ENUM_SINGLE!(ES8316_ADC_MUTE, 1, 2, ADCPOL_TXT);
static DACPOL_TXT: [*const c_char; 4] = [
    cstr!("Normal"),
    cstr!("R Invert"),
    cstr!("L Invert"),
    cstr!("L + R Invert"),
];
static DACPOL: soc_enum = SOC_ENUM_SINGLE!(ES8316_DAC_SET1, 0, 4, DACPOL_TXT);

static ES8316_SND_CONTROLS: &[snd_kcontrol_new] = &[
    SOC_DOUBLE_TLV!("Headphone Playback Volume", ES8316_CPHP_ICAL_VOL, 4, 0, 3, 1, HPOUT_VOL_TLV),
    SOC_DOUBLE_TLV!("Headphone Mixer Volume", ES8316_HPMIX_VOL, 4, 0, 11, 0, HPMIXER_GAIN_TLV),
    SOC_ENUM!("Playback Polarity", DACPOL),
    SOC_DOUBLE_R_TLV!("DAC Playback Volume", ES8316_DAC_VOLL, ES8316_DAC_VOLR, 0, 0xc0, 1, DAC_VOL_TLV),
    SOC_SINGLE!("DAC Soft Ramp Switch", ES8316_DAC_SET1, 4, 1, 1),
    SOC_SINGLE!("DAC Soft Ramp Rate", ES8316_DAC_SET1, 2, 3, 0),
    SOC_SINGLE!("DAC Notch Filter Switch", ES8316_DAC_SET2, 6, 1, 0),
    SOC_SINGLE!("DAC Double Fs Switch", ES8316_DAC_SET2, 7, 1, 0),
    SOC_SINGLE!("DAC Stereo Enhancement", ES8316_DAC_SET3, 0, 7, 0),
    SOC_SINGLE!("DAC Mono Mix Switch", ES8316_DAC_SET3, 3, 1, 0),
    SOC_ENUM!("Capture Polarity", ADCPOL),
    SOC_SINGLE!("Mic Boost Switch", ES8316_ADC_D2SEPGA, 0, 1, 0),
    SOC_SINGLE_TLV!("ADC Capture Volume", ES8316_ADC_VOLUME, 0, 0xc0, 1, ADC_VOL_TLV),
    SOC_SINGLE_TLV!("ADC PGA Gain Volume", ES8316_ADC_PGAGAIN, 4, 10, 0, ADC_PGA_GAIN_TLV),
    SOC_SINGLE!("ADC Soft Ramp Switch", ES8316_ADC_MUTE, 4, 1, 0),
    SOC_SINGLE!("ADC Double Fs Switch", ES8316_ADC_DMIC, 4, 1, 0),
    SOC_SINGLE!("ALC Capture Switch", ES8316_ADC_ALC1, 6, 1, 0),
    SOC_SINGLE_TLV!("ALC Capture Max Volume", ES8316_ADC_ALC1, 0, 28, 0, ALC_MAX_GAIN_TLV),
    SOC_SINGLE_TLV!("ALC Capture Min Volume", ES8316_ADC_ALC2, 0, 28, 0, ALC_MIN_GAIN_TLV),
    SOC_SINGLE_TLV!("ALC Capture Target Volume", ES8316_ADC_ALC3, 4, 11, 0, ALC_TARGET_TLV),
    SOC_SINGLE!("ALC Capture Hold Time", ES8316_ADC_ALC3, 0, 10, 0),
    SOC_SINGLE!("ALC Capture Decay Time", ES8316_ADC_ALC4, 4, 10, 0),
    SOC_SINGLE!("ALC Capture Attack Time", ES8316_ADC_ALC4, 0, 10, 0),
    SOC_SINGLE!("ALC Capture Noise Gate Switch", ES8316_ADC_ALC_NG, 5, 1, 0),
    SOC_SINGLE!("ALC Capture Noise Gate Threshold", ES8316_ADC_ALC_NG, 0, 31, 0),
    SOC_ENUM!("ALC Capture Noise Gate Type", NG_TYPE),
];

/* Analog Input Mux */
static ES8316_ANALOG_IN_TXT: [*const c_char; 4] = [
    cstr!("lin1-rin1"),
    cstr!("lin2-rin2"),
    cstr!("lin1-rin1 with 20db Boost"),
    cstr!("lin2-rin2 with 20db Boost"),
];
static ES8316_ANALOG_IN_VALUES: [c_uint; 4] = [0, 1, 2, 3];
static ES8316_ANALOG_INPUT_ENUM: soc_enum = SOC_VALUE_ENUM_SINGLE!(
    ES8316_ADC_PDN_LINSEL,
    4,
    3,
    array_size!(ES8316_ANALOG_IN_TXT),
    ES8316_ANALOG_IN_TXT,
    ES8316_ANALOG_IN_VALUES
);
static ES8316_ANALOG_IN_MUX_CONTROLS: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Route", ES8316_ANALOG_INPUT_ENUM);

static ES8316_DMIC_TXT: [*const c_char; 3] = [
    cstr!("dmic disable"),
    cstr!("dmic data at high level"),
    cstr!("dmic data at low level"),
];
static ES8316_DMIC_VALUES: [c_uint; 3] = [0, 2, 3];
static ES8316_DMIC_SRC_ENUM: soc_enum = SOC_VALUE_ENUM_SINGLE!(
    ES8316_ADC_DMIC,
    0,
    3,
    array_size!(ES8316_DMIC_TXT),
    ES8316_DMIC_TXT,
    ES8316_DMIC_VALUES
);
static ES8316_DMIC_SRC_CONTROLS: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", ES8316_DMIC_SRC_ENUM);

/* hp mixer mux */
static ES8316_HPMUX_TEXTS: [*const c_char; 4] = [
    cstr!("lin1-rin1"),
    cstr!("lin2-rin2"),
    cstr!("lin-rin with Boost"),
    cstr!("lin-rin with Boost and PGA"),
];

static ES8316_LEFT_HPMUX_ENUM: soc_enum = SOC_ENUM_SINGLE_DECL!(ES8316_HPMIX_SEL, 4, ES8316_HPMUX_TEXTS);
static ES8316_LEFT_HPMUX_CONTROLS: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", ES8316_LEFT_HPMUX_ENUM);
static ES8316_RIGHT_HPMUX_ENUM: soc_enum = SOC_ENUM_SINGLE_DECL!(ES8316_HPMIX_SEL, 0, ES8316_HPMUX_TEXTS);
static ES8316_RIGHT_HPMUX_CONTROLS: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", ES8316_RIGHT_HPMUX_ENUM);

/* headphone Output Mixer */
static ES8316_OUT_LEFT_MIX: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("LLIN Switch", ES8316_HPMIX_SWITCH, 6, 1, 0),
    SOC_DAPM_SINGLE!("Left DAC Switch", ES8316_HPMIX_SWITCH, 7, 1, 0),
];
static ES8316_OUT_RIGHT_MIX: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("RLIN Switch", ES8316_HPMIX_SWITCH, 2, 1, 0),
    SOC_DAPM_SINGLE!("Right DAC Switch", ES8316_HPMIX_SWITCH, 3, 1, 0),
];

/* DAC data source mux */
static ES8316_DACSRC_TEXTS: [*const c_char; 4] = [
    cstr!("LDATA TO LDAC, RDATA TO RDAC"),
    cstr!("LDATA TO LDAC, LDATA TO RDAC"),
    cstr!("RDATA TO LDAC, RDATA TO RDAC"),
    cstr!("RDATA TO LDAC, LDATA TO RDAC"),
];

static ES8316_DACSRC_MUX_ENUM: soc_enum = SOC_ENUM_SINGLE_DECL!(ES8316_DAC_SET1, 6, ES8316_DACSRC_TEXTS);
static ES8316_DACSRC_MUX_CONTROLS: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", ES8316_DACSRC_MUX_ENUM);

static ES8316_DAPM_WIDGETS: &[snd_soc_dapm_widget] = &[
    SND_SOC_DAPM_SUPPLY!("Bias", ES8316_SYS_PDN, 3, 1, ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!("Analog power", ES8316_SYS_PDN, 4, 1, ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!("Mic Bias", ES8316_SYS_PDN, 5, 1, ptr::null_mut(), 0),
    SND_SOC_DAPM_INPUT!("DMIC"),
    SND_SOC_DAPM_INPUT!("MIC1"),
    SND_SOC_DAPM_INPUT!("MIC2"),
    /* Input Mux */
    SND_SOC_DAPM_MUX!("Differential Mux", SND_SOC_NOPM, 0, 0, &ES8316_ANALOG_IN_MUX_CONTROLS),
    SND_SOC_DAPM_SUPPLY!("ADC Vref", ES8316_SYS_PDN, 1, 1, ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!("ADC bias", ES8316_SYS_PDN, 2, 1, ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!("ADC Clock", ES8316_CLKMGR_CLKSW, 3, 0, ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA!("Line input PGA", ES8316_ADC_PDN_LINSEL, 7, 1, ptr::null_mut(), 0),
    SND_SOC_DAPM_ADC!("Mono ADC", ptr::null(), ES8316_ADC_PDN_LINSEL, 6, 1),
    SND_SOC_DAPM_MUX!("Digital Mic Mux", SND_SOC_NOPM, 0, 0, &ES8316_DMIC_SRC_CONTROLS),
    /* Digital Interface */
    SND_SOC_DAPM_AIF_OUT!("I2S OUT", "I2S1 Capture", 1, ES8316_SERDATA_ADC, 6, 1),
    SND_SOC_DAPM_AIF_IN!("I2S IN", "I2S1 Playback", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_MUX!("DAC Source Mux", SND_SOC_NOPM, 0, 0, &ES8316_DACSRC_MUX_CONTROLS),
    SND_SOC_DAPM_SUPPLY!("DAC Vref", ES8316_SYS_PDN, 0, 1, ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!("DAC Clock", ES8316_CLKMGR_CLKSW, 2, 0, ptr::null_mut(), 0),
    SND_SOC_DAPM_DAC!("Right DAC", ptr::null(), ES8316_DAC_PDN, 0, 1),
    SND_SOC_DAPM_DAC!("Left DAC", ptr::null(), ES8316_DAC_PDN, 4, 1),
    /* Headphone Output Side */
    SND_SOC_DAPM_MUX!("Left Headphone Mux", SND_SOC_NOPM, 0, 0, &ES8316_LEFT_HPMUX_CONTROLS),
    SND_SOC_DAPM_MUX!("Right Headphone Mux", SND_SOC_NOPM, 0, 0, &ES8316_RIGHT_HPMUX_CONTROLS),
    SND_SOC_DAPM_MIXER!("Left Headphone Mixer", ES8316_HPMIX_PDN, 5, 1, &ES8316_OUT_LEFT_MIX[0], array_size!(ES8316_OUT_LEFT_MIX)),
    SND_SOC_DAPM_MIXER!("Right Headphone Mixer", ES8316_HPMIX_PDN, 1, 1, &ES8316_OUT_RIGHT_MIX[0], array_size!(ES8316_OUT_RIGHT_MIX)),
    SND_SOC_DAPM_PGA!("Left Headphone Mixer Out", ES8316_HPMIX_PDN, 4, 1, ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA!("Right Headphone Mixer Out", ES8316_HPMIX_PDN, 0, 1, ptr::null_mut(), 0),
    SND_SOC_DAPM_OUT_DRV!("Left Headphone Charge Pump", ES8316_CPHP_OUTEN, 6, 0, ptr::null_mut(), 0),
    SND_SOC_DAPM_OUT_DRV!("Right Headphone Charge Pump", ES8316_CPHP_OUTEN, 2, 0, ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!("Headphone Charge Pump", ES8316_CPHP_PDN2, 5, 1, ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!("Headphone Charge Pump Clock", ES8316_CLKMGR_CLKSW, 4, 0, ptr::null_mut(), 0),
    SND_SOC_DAPM_OUT_DRV!("Left Headphone Driver", ES8316_CPHP_OUTEN, 5, 0, ptr::null_mut(), 0),
    SND_SOC_DAPM_OUT_DRV!("Right Headphone Driver", ES8316_CPHP_OUTEN, 1, 0, ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!("Headphone Out", ES8316_CPHP_PDN1, 2, 1, ptr::null_mut(), 0),
    /* pdn_Lical and pdn_Rical bits are documented as Reserved, but must
     * be explicitly unset in order to enable HP output
     */
    SND_SOC_DAPM_SUPPLY!("Left Headphone ical", ES8316_CPHP_ICAL_VOL, 7, 1, ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!("Right Headphone ical", ES8316_CPHP_ICAL_VOL, 3, 1, ptr::null_mut(), 0),
    SND_SOC_DAPM_OUTPUT!("HPOL"),
    SND_SOC_DAPM_OUTPUT!("HPOR"),
];

static ES8316_DAPM_ROUTES: &[snd_soc_dapm_route] = &[
    /* Recording */
    SND_SOC_DAPM_ROUTE!("MIC1", ptr::null(), "Mic Bias"),
    SND_SOC_DAPM_ROUTE!("MIC2", ptr::null(), "Mic Bias"),
    SND_SOC_DAPM_ROUTE!("MIC1", ptr::null(), "Bias"),
    SND_SOC_DAPM_ROUTE!("MIC2", ptr::null(), "Bias"),
    SND_SOC_DAPM_ROUTE!("MIC1", ptr::null(), "Analog power"),
    SND_SOC_DAPM_ROUTE!("MIC2", ptr::null(), "Analog power"),
    SND_SOC_DAPM_ROUTE!("Differential Mux", "lin1-rin1", "MIC1"),
    SND_SOC_DAPM_ROUTE!("Differential Mux", "lin2-rin2", "MIC2"),
    SND_SOC_DAPM_ROUTE!("Line input PGA", ptr::null(), "Differential Mux"),
    SND_SOC_DAPM_ROUTE!("Mono ADC", ptr::null(), "ADC Clock"),
    SND_SOC_DAPM_ROUTE!("Mono ADC", ptr::null(), "ADC Vref"),
    SND_SOC_DAPM_ROUTE!("Mono ADC", ptr::null(), "ADC bias"),
    SND_SOC_DAPM_ROUTE!("Mono ADC", ptr::null(), "Line input PGA"),
    /* It's not clear why, but to avoid recording only silence,
     * the DAC clock must be running for the ADC to work.
     */
    SND_SOC_DAPM_ROUTE!("Mono ADC", ptr::null(), "DAC Clock"),
    SND_SOC_DAPM_ROUTE!("Digital Mic Mux", "dmic disable", "Mono ADC"),
    SND_SOC_DAPM_ROUTE!("I2S OUT", ptr::null(), "Digital Mic Mux"),
    /* Playback */
    SND_SOC_DAPM_ROUTE!("DAC Source Mux", "LDATA TO LDAC, RDATA TO RDAC", "I2S IN"),
    SND_SOC_DAPM_ROUTE!("Left DAC", ptr::null(), "DAC Clock"),
    SND_SOC_DAPM_ROUTE!("Right DAC", ptr::null(), "DAC Clock"),
    SND_SOC_DAPM_ROUTE!("Left DAC", ptr::null(), "DAC Vref"),
    SND_SOC_DAPM_ROUTE!("Right DAC", ptr::null(), "DAC Vref"),
    SND_SOC_DAPM_ROUTE!("Left DAC", ptr::null(), "DAC Source Mux"),
    SND_SOC_DAPM_ROUTE!("Right DAC", ptr::null(), "DAC Source Mux"),
    SND_SOC_DAPM_ROUTE!("Left Headphone Mux", "lin-rin with Boost and PGA", "Line input PGA"),
    SND_SOC_DAPM_ROUTE!("Right Headphone Mux", "lin-rin with Boost and PGA", "Line input PGA"),
    SND_SOC_DAPM_ROUTE!("Left Headphone Mixer", "LLIN Switch", "Left Headphone Mux"),
    SND_SOC_DAPM_ROUTE!("Left Headphone Mixer", "Left DAC Switch", "Left DAC"),
    SND_SOC_DAPM_ROUTE!("Right Headphone Mixer", "RLIN Switch", "Right Headphone Mux"),
    SND_SOC_DAPM_ROUTE!("Right Headphone Mixer", "Right DAC Switch", "Right DAC"),
    SND_SOC_DAPM_ROUTE!("Left Headphone Mixer Out", ptr::null(), "Left Headphone Mixer"),
    SND_SOC_DAPM_ROUTE!("Right Headphone Mixer Out", ptr::null(), "Right Headphone Mixer"),
    SND_SOC_DAPM_ROUTE!("Left Headphone Charge Pump", ptr::null(), "Left Headphone Mixer Out"),
    SND_SOC_DAPM_ROUTE!("Right Headphone Charge Pump", ptr::null(), "Right Headphone Mixer Out"),
    SND_SOC_DAPM_ROUTE!("Left Headphone Charge Pump", ptr::null(), "Headphone Charge Pump"),
    SND_SOC_DAPM_ROUTE!("Right Headphone Charge Pump", ptr::null(), "Headphone Charge Pump"),
    SND_SOC_DAPM_ROUTE!("Left Headphone Charge Pump", ptr::null(), "Headphone Charge Pump Clock"),
    SND_SOC_DAPM_ROUTE!("Right Headphone Charge Pump", ptr::null(), "Headphone Charge Pump Clock"),
    SND_SOC_DAPM_ROUTE!("Left Headphone Driver", ptr::null(), "Left Headphone Charge Pump"),
    SND_SOC_DAPM_ROUTE!("Right Headphone Driver", ptr::null(), "Right Headphone Charge Pump"),
    SND_SOC_DAPM_ROUTE!("HPOL", ptr::null(), "Left Headphone Driver"),
    SND_SOC_DAPM_ROUTE!("HPOR", ptr::null(), "Right Headphone Driver"),
    SND_SOC_DAPM_ROUTE!("HPOL", ptr::null(), "Left Headphone ical"),
    SND_SOC_DAPM_ROUTE!("HPOR", ptr::null(), "Right Headphone ical"),
    SND_SOC_DAPM_ROUTE!("Headphone Out", ptr::null(), "Bias"),
    SND_SOC_DAPM_ROUTE!("Headphone Out", ptr::null(), "Analog power"),
    SND_SOC_DAPM_ROUTE!("HPOL", ptr::null(), "Headphone Out"),
    SND_SOC_DAPM_ROUTE!("HPOR", ptr::null(), "Headphone Out"),
];

unsafe extern "C" fn es8316_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let es8316 = snd_soc_component_get_drvdata(component) as *mut es8316_priv;
    let mut count: usize = 0;

    (*es8316).sysclk = freq;
    (*es8316).sysclk_constraints.list = ptr::null_mut();
    (*es8316).sysclk_constraints.count = 0;

    if freq == 0 {
        return 0;
    }

    let ret = clk_set_rate((*es8316).mclk, freq);
    if ret != 0 {
        return ret;
    }

    /* Limit supported sample rates to ones that can be autodetected
     * by the codec running in slave mode.
     */
    for ratio in SUPPORTED_MCLK_LRCK_RATIOS {
        if freq % ratio == 0 {
            (*es8316).allowed_rates[count] = freq / ratio;
            count += 1;
        }

        /* We also check if the halved MCLK produces a valid rate
         * since the codec supports halving the MCLK.
         */
        if (freq / ratio) % 2 == 0 {
            (*es8316).allowed_rates[count] = freq / ratio / 2;
            count += 1;
        }
    }

    if count != 0 {
        (*es8316).sysclk_constraints.list = (*es8316).allowed_rates.as_mut_ptr();
        (*es8316).sysclk_constraints.count = count as c_uint;
    }

    0
}

unsafe extern "C" fn es8316_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let mut serdata1: U8 = 0;
    let mut serdata2: U8 = 0;
    let clksw: U8;
    let mut mask: U8;

    if (fmt & SND_SOC_DAIFMT_MASTER_MASK) == SND_SOC_DAIFMT_CBP_CFP {
        serdata1 |= ES8316_SERDATA1_MASTER as U8;
    }

    if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) != SND_SOC_DAIFMT_I2S {
        dev_err(component.as_ref().unwrap().dev, cstr!("Codec driver only supports I2S format\n"));
        return -EINVAL;
    }

    /* Clock inversion */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => {}
        x if x == SND_SOC_DAIFMT_IB_IF => {
            serdata1 |= ES8316_SERDATA1_BCLK_INV as U8;
            serdata2 |= ES8316_SERDATA2_ADCLRP as U8;
        }
        x if x == SND_SOC_DAIFMT_IB_NF => {
            serdata1 |= ES8316_SERDATA1_BCLK_INV as U8;
        }
        x if x == SND_SOC_DAIFMT_NB_IF => {
            serdata2 |= ES8316_SERDATA2_ADCLRP as U8;
        }
        _ => return -EINVAL,
    }

    mask = (ES8316_SERDATA1_MASTER | ES8316_SERDATA1_BCLK_INV) as U8;
    snd_soc_component_update_bits(component, ES8316_SERDATA1, mask as c_uint, serdata1 as c_uint);

    mask = (ES8316_SERDATA2_FMT_MASK | ES8316_SERDATA2_ADCLRP) as U8;
    snd_soc_component_update_bits(component, ES8316_SERDATA_ADC, mask as c_uint, serdata2 as c_uint);
    snd_soc_component_update_bits(component, ES8316_SERDATA_DAC, mask as c_uint, serdata2 as c_uint);

    /* Enable BCLK and MCLK inputs in slave mode */
    clksw = (ES8316_CLKMGR_CLKSW_MCLK_ON | ES8316_CLKMGR_CLKSW_BCLK_ON) as U8;
    snd_soc_component_update_bits(component, ES8316_CLKMGR_CLKSW, clksw as c_uint, clksw as c_uint);

    0
}

unsafe extern "C" fn es8316_pcm_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let es8316 = snd_soc_component_get_drvdata(component) as *mut es8316_priv;

    if !(*es8316).sysclk_constraints.list.is_null() {
        snd_pcm_hw_constraint_list(
            (*substream).runtime,
            0,
            SNDRV_PCM_HW_PARAM_RATE,
            &mut (*es8316).sysclk_constraints,
        );
    }

    0
}

unsafe extern "C" fn es8316_pcm_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let es8316 = snd_soc_component_get_drvdata(component) as *mut es8316_priv;
    let mut wordlen: U8 = 0;
    let mut bclk_divider: U8;
    let lrck_divider: U16;
    let mut i: usize;
    let mut clk: c_uint = (*es8316).sysclk / 2;
    let mut clk_valid = false;

    /* We will start with halved sysclk and see if we can use it
     * for proper clocking. This is to minimise the risk of running
     * the CODEC with a too high frequency. We have an SKU where
     * the sysclk frequency is 48Mhz and this causes the sound to be
     * sped up. If we can run with a halved sysclk, we will use it,
     * if we can't use it, then full sysclk will be used.
     */
    while !clk_valid {
        /* Validate supported sample rates that are autodetected from MCLK */
        i = 0;
        while i < SUPPORTED_MCLK_LRCK_RATIOS.len() {
            let ratio = SUPPORTED_MCLK_LRCK_RATIOS[i];

            if clk % ratio == 0 && clk / ratio == params_rate(params) {
                break;
            }
            i += 1;
        }
        if i == SUPPORTED_MCLK_LRCK_RATIOS.len() {
            if clk == (*es8316).sysclk {
                return -EINVAL;
            }
            clk = (*es8316).sysclk;
        } else {
            clk_valid = true;
        }
    }

    if clk != (*es8316).sysclk {
        snd_soc_component_update_bits(
            component,
            ES8316_CLKMGR_CLKSW,
            ES8316_CLKMGR_CLKSW_MCLK_DIV,
            ES8316_CLKMGR_CLKSW_MCLK_DIV,
        );
    }

    lrck_divider = (clk / params_rate(params)) as U16;
    bclk_divider = (lrck_divider / 4) as U8;
    match params_format(params) {
        x if x == SNDRV_PCM_FORMAT_S16_LE => {
            wordlen = ES8316_SERDATA2_LEN_16 as U8;
            bclk_divider /= 16;
        }
        x if x == SNDRV_PCM_FORMAT_S20_3LE => {
            wordlen = ES8316_SERDATA2_LEN_20 as U8;
            bclk_divider /= 20;
        }
        x if x == SNDRV_PCM_FORMAT_S24_LE || x == SNDRV_PCM_FORMAT_S24_3LE => {
            wordlen = ES8316_SERDATA2_LEN_24 as U8;
            bclk_divider /= 24;
        }
        x if x == SNDRV_PCM_FORMAT_S32_LE => {
            wordlen = ES8316_SERDATA2_LEN_32 as U8;
            bclk_divider /= 32;
        }
        _ => return -EINVAL,
    }

    snd_soc_component_update_bits(component, ES8316_SERDATA_DAC, ES8316_SERDATA2_LEN_MASK, wordlen as c_uint);
    snd_soc_component_update_bits(component, ES8316_SERDATA_ADC, ES8316_SERDATA2_LEN_MASK, wordlen as c_uint);
    snd_soc_component_update_bits(component, ES8316_SERDATA1, 0x1f, bclk_divider as c_uint);
    snd_soc_component_update_bits(component, ES8316_CLKMGR_ADCDIV1, 0x0f, (lrck_divider >> 8) as c_uint);
    snd_soc_component_update_bits(component, ES8316_CLKMGR_ADCDIV2, 0xff, (lrck_divider & 0xff) as c_uint);
    snd_soc_component_update_bits(component, ES8316_CLKMGR_DACDIV1, 0x0f, (lrck_divider >> 8) as c_uint);
    snd_soc_component_update_bits(component, ES8316_CLKMGR_DACDIV2, 0xff, (lrck_divider & 0xff) as c_uint);
    0
}

unsafe extern "C" fn es8316_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    snd_soc_component_update_bits((*dai).component, ES8316_DAC_SET1, 0x20, if mute != 0 { 0x20 } else { 0 });
    0
}

const ES8316_FORMATS: u64 =
    unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE };

static ES8316_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(es8316_pcm_startup),
    hw_params: Some(es8316_pcm_hw_params),
    set_fmt: Some(es8316_set_dai_fmt),
    set_sysclk: Some(es8316_set_dai_sysclk),
    mute_stream: Some(es8316_mute),
    no_capture_mute: 1,
};

static mut ES8316_DAI: snd_soc_dai_driver = snd_soc_dai_driver {
    name: cstr!("ES8316 HiFi"),
    playback: snd_soc_pcm_stream {
        stream_name: cstr!("Playback"),
        channels_min: 1,
        channels_max: 2,
        rates: unsafe { SNDRV_PCM_RATE_8000_48000 },
        formats: ES8316_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: cstr!("Capture"),
        channels_min: 1,
        channels_max: 2,
        rates: unsafe { SNDRV_PCM_RATE_8000_48000 },
        formats: ES8316_FORMATS,
    },
    ops: &ES8316_OPS,
    symmetric_rate: 1,
};

unsafe fn es8316_enable_micbias_for_mic_gnd_short_detect(component: *mut snd_soc_component) {
    let dapm = snd_soc_component_to_dapm(component);

    snd_soc_dapm_mutex_lock(dapm);
    snd_soc_dapm_force_enable_pin_unlocked(dapm, cstr!("Bias"));
    snd_soc_dapm_force_enable_pin_unlocked(dapm, cstr!("Analog power"));
    snd_soc_dapm_force_enable_pin_unlocked(dapm, cstr!("Mic Bias"));
    snd_soc_dapm_sync_unlocked(dapm);
    snd_soc_dapm_mutex_unlock(dapm);

    msleep(20);
}

unsafe fn es8316_disable_micbias_for_mic_gnd_short_detect(component: *mut snd_soc_component) {
    let dapm = snd_soc_component_to_dapm(component);

    snd_soc_dapm_mutex_lock(dapm);
    snd_soc_dapm_disable_pin_unlocked(dapm, cstr!("Mic Bias"));
    snd_soc_dapm_disable_pin_unlocked(dapm, cstr!("Analog power"));
    snd_soc_dapm_disable_pin_unlocked(dapm, cstr!("Bias"));
    snd_soc_dapm_sync_unlocked(dapm);
    snd_soc_dapm_mutex_unlock(dapm);
}

unsafe extern "C" fn es8316_irq(_irq: c_int, data: *mut c_void) -> IrqReturnT {
    let es8316 = data as *mut es8316_priv;
    let comp = (*es8316).component;
    let mut flags: c_uint = 0;

    mutex_lock(&mut (*es8316).lock);

    regmap_read((*es8316).regmap, ES8316_GPIO_FLAG, &mut flags);
    if flags == 0x00 {
        mutex_unlock(&mut (*es8316).lock);
        return IRQ_HANDLED; /* Powered-down / reset */
    }

    /* Catch spurious IRQ before set_jack is called */
    if (*es8316).jack.is_null() {
        mutex_unlock(&mut (*es8316).lock);
        return IRQ_HANDLED;
    }

    if (*es8316).jd_inverted {
        flags ^= ES8316_GPIO_FLAG_HP_NOT_INSERTED;
    }

    dev_dbg((*comp).dev, cstr!("gpio flags %#04x\n"), flags);
    if flags & ES8316_GPIO_FLAG_HP_NOT_INSERTED != 0 {
        /* Jack removed, or spurious IRQ? */
        if (*(*es8316).jack).status & SND_JACK_MICROPHONE != 0 {
            es8316_disable_micbias_for_mic_gnd_short_detect(comp);
        }

        if (*(*es8316).jack).status & SND_JACK_HEADPHONE != 0 {
            snd_soc_jack_report((*es8316).jack, 0, SND_JACK_HEADSET | SND_JACK_BTN_0);
            dev_dbg((*comp).dev, cstr!("jack unplugged\n"));
        }
    } else if (*(*es8316).jack).status & SND_JACK_HEADPHONE == 0 {
        /* Jack inserted, determine type */
        es8316_enable_micbias_for_mic_gnd_short_detect(comp);
        regmap_read((*es8316).regmap, ES8316_GPIO_FLAG, &mut flags);
        if (*es8316).jd_inverted {
            flags ^= ES8316_GPIO_FLAG_HP_NOT_INSERTED;
        }
        dev_dbg((*comp).dev, cstr!("gpio flags %#04x\n"), flags);
        if flags & ES8316_GPIO_FLAG_HP_NOT_INSERTED != 0 {
            /* Jack unplugged underneath us */
            es8316_disable_micbias_for_mic_gnd_short_detect(comp);
        } else if flags & ES8316_GPIO_FLAG_GM_NOT_SHORTED != 0 {
            /* Open, headset */
            snd_soc_jack_report((*es8316).jack, SND_JACK_HEADSET, SND_JACK_HEADSET);
            /* Keep mic-gnd-short detection on for button press */
        } else {
            /* Shorted, headphones */
            snd_soc_jack_report((*es8316).jack, SND_JACK_HEADPHONE, SND_JACK_HEADSET);
            /* No longer need mic-gnd-short detection */
            es8316_disable_micbias_for_mic_gnd_short_detect(comp);
        }
    } else if (*(*es8316).jack).status & SND_JACK_MICROPHONE != 0 {
        /* Interrupt while jack inserted, report button state */
        if flags & ES8316_GPIO_FLAG_GM_NOT_SHORTED != 0 {
            /* Open, button release */
            snd_soc_jack_report((*es8316).jack, 0, SND_JACK_BTN_0);
        } else {
            /* Short, button press */
            snd_soc_jack_report((*es8316).jack, SND_JACK_BTN_0, SND_JACK_BTN_0);
        }
    }

    mutex_unlock(&mut (*es8316).lock);
    IRQ_HANDLED
}

unsafe fn es8316_enable_jack_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack) {
    let es8316 = snd_soc_component_get_drvdata(component) as *mut es8316_priv;

    /*
     * Init es8316->jd_inverted here and not in the probe, as we cannot
     * guarantee that the bytchr-es8316 driver, which might set this
     * property, will probe before us.
     */
    (*es8316).jd_inverted = device_property_read_bool((*component).dev, cstr!("everest,jack-detect-inverted"));

    mutex_lock(&mut (*es8316).lock);
    (*es8316).jack = jack;

    if (*(*es8316).jack).status & SND_JACK_MICROPHONE != 0 {
        es8316_enable_micbias_for_mic_gnd_short_detect(component);
    }

    snd_soc_component_update_bits(
        component,
        ES8316_GPIO_DEBOUNCE,
        ES8316_GPIO_ENABLE_INTERRUPT,
        ES8316_GPIO_ENABLE_INTERRUPT,
    );
    mutex_unlock(&mut (*es8316).lock);

    /* Enable irq and sync initial jack state */
    enable_irq((*es8316).irq);
    es8316_irq((*es8316).irq, es8316 as *mut c_void);
}

unsafe fn es8316_disable_jack_detect(component: *mut snd_soc_component) {
    let es8316 = snd_soc_component_get_drvdata(component) as *mut es8316_priv;

    if (*es8316).jack.is_null() {
        return; /* Already disabled (or never enabled) */
    }

    disable_irq((*es8316).irq);

    mutex_lock(&mut (*es8316).lock);

    snd_soc_component_update_bits(component, ES8316_GPIO_DEBOUNCE, ES8316_GPIO_ENABLE_INTERRUPT, 0);

    if (*(*es8316).jack).status & SND_JACK_MICROPHONE != 0 {
        es8316_disable_micbias_for_mic_gnd_short_detect(component);
        snd_soc_jack_report((*es8316).jack, 0, SND_JACK_BTN_0);
    }

    (*es8316).jack = ptr::null_mut();
    mutex_unlock(&mut (*es8316).lock);
}

unsafe extern "C" fn es8316_set_jack(
    component: *mut snd_soc_component,
    jack: *mut snd_soc_jack,
    _data: *mut c_void,
) -> c_int {
    if !jack.is_null() {
        es8316_enable_jack_detect(component, jack);
    } else {
        es8316_disable_jack_detect(component);
    }

    0
}

unsafe extern "C" fn es8316_probe(component: *mut snd_soc_component) -> c_int {
    let es8316 = snd_soc_component_get_drvdata(component) as *mut es8316_priv;
    let mut ret: c_int;

    (*es8316).component = component;

    (*es8316).mclk = devm_clk_get_optional((*component).dev, cstr!("mclk"));
    if IS_ERR!((*es8316).mclk) {
        dev_err((*component).dev, cstr!("unable to get mclk\n"));
        return PTR_ERR!((*es8316).mclk);
    }
    if (*es8316).mclk.is_null() {
        dev_warn((*component).dev, cstr!("assuming static mclk\n"));
    }

    ret = clk_prepare_enable((*es8316).mclk);
    if ret != 0 {
        dev_err((*component).dev, cstr!("unable to enable mclk\n"));
        return ret;
    }

    /* Reset codec and enable current state machine */
    snd_soc_component_write(component, ES8316_RESET, 0x3f);
    usleep_range(5000, 5500);
    snd_soc_component_write(component, ES8316_RESET, ES8316_RESET_CSM_ON);
    msleep(30);

    /*
     * Documentation is unclear, but this value from the vendor driver is
     * needed otherwise audio output is silent.
     */
    snd_soc_component_write(component, ES8316_SYS_VMIDSEL, 0xff);

    /*
     * Documentation for this register is unclear and incomplete,
     * but here is a vendor-provided value that improves volume
     * and quality for Intel CHT platforms.
     */
    snd_soc_component_write(component, ES8316_CLKMGR_ADCOSR, 0x32);

    0
}

unsafe extern "C" fn es8316_remove(component: *mut snd_soc_component) {
    let es8316 = snd_soc_component_get_drvdata(component) as *mut es8316_priv;

    clk_disable_unprepare((*es8316).mclk);
}

unsafe extern "C" fn es8316_resume(component: *mut snd_soc_component) -> c_int {
    let es8316 = snd_soc_component_get_drvdata(component) as *mut es8316_priv;

    regcache_cache_only((*es8316).regmap, false);
    regcache_sync((*es8316).regmap);

    0
}

unsafe extern "C" fn es8316_suspend(component: *mut snd_soc_component) -> c_int {
    let es8316 = snd_soc_component_get_drvdata(component) as *mut es8316_priv;

    regcache_cache_only((*es8316).regmap, true);
    regcache_mark_dirty((*es8316).regmap);

    0
}

static SOC_COMPONENT_DEV_ES8316: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(es8316_probe),
    remove: Some(es8316_remove),
    resume: Some(es8316_resume),
    suspend: Some(es8316_suspend),
    set_jack: Some(es8316_set_jack),
    controls: ES8316_SND_CONTROLS.as_ptr(),
    num_controls: ES8316_SND_CONTROLS.len() as c_uint,
    dapm_widgets: ES8316_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: ES8316_DAPM_WIDGETS.len() as c_uint,
    dapm_routes: ES8316_DAPM_ROUTES.as_ptr(),
    num_dapm_routes: ES8316_DAPM_ROUTES.len() as c_uint,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn es8316_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        x if x == ES8316_GPIO_FLAG => true,
        _ => false,
    }
}

static ES8316_REGMAP: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    use_single_read: true,
    use_single_write: true,
    max_register: 0x53,
    volatile_reg: Some(es8316_volatile_reg),
    cache_type: unsafe { REGCACHE_MAPLE },
};

unsafe extern "C" fn es8316_i2c_probe(i2c_client: *mut i2c_client) -> c_int {
    let dev = &mut (*i2c_client).dev as *mut device;
    let es8316: *mut es8316_priv;
    let mut ret: c_int;

    es8316 = devm_kzalloc(
        &mut (*i2c_client).dev,
        core::mem::size_of::<es8316_priv>(),
        GFP_KERNEL,
    ) as *mut es8316_priv;
    if es8316.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c_client, es8316 as *mut c_void);

    ret = devm_regulator_bulk_get_enable(dev, ES8316_SUPPLY_NAMES.len() as c_uint, ES8316_SUPPLY_NAMES.as_ptr());
    if ret != 0 {
        return dev_err_probe(dev, ret, cstr!("unable to enable supplies\n"));
    }

    (*es8316).regmap = devm_regmap_init_i2c(i2c_client, &ES8316_REGMAP);
    if IS_ERR!((*es8316).regmap) {
        return PTR_ERR!((*es8316).regmap);
    }

    (*es8316).irq = (*i2c_client).irq;
    mutex_init(&mut (*es8316).lock);

    if (*es8316).irq > 0 {
        ret = devm_request_threaded_irq(
            dev,
            (*es8316).irq,
            ptr::null(),
            Some(es8316_irq),
            IRQF_TRIGGER_HIGH | IRQF_ONESHOT | IRQF_NO_AUTOEN,
            cstr!("es8316"),
            es8316 as *mut c_void,
        );
        if ret != 0 {
            dev_warn(dev, cstr!("Failed to get IRQ %d: %d\n"), (*es8316).irq, ret);
            (*es8316).irq = -ENXIO;
        }
    }

    devm_snd_soc_register_component(
        &mut (*i2c_client).dev,
        &SOC_COMPONENT_DEV_ES8316,
        &mut ES8316_DAI,
        1,
    )
}

static ES8316_I2C_ID: [i2c_device_id; 2] = [
    i2c_device_id {
        name: [
            b'e' as c_char, b's' as c_char, b'8' as c_char, b'3' as c_char, b'1' as c_char, b'6' as c_char, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        driver_data: 0,
    },
    i2c_device_id { name: [0; 20], driver_data: 0 },
];
MODULE_DEVICE_TABLE!(i2c, ES8316_I2C_ID);

// CONFIG_OF:
static ES8316_OF_MATCH: [of_device_id; 2] = [
    of_device_id {
        compatible: cstr!("everest,es8316"),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
MODULE_DEVICE_TABLE!(of, ES8316_OF_MATCH);

// CONFIG_ACPI:
static ES8316_ACPI_MATCH: [acpi_device_id; 3] = [
    acpi_device_id {
        id: [
            b'E' as c_char, b'S' as c_char, b'S' as c_char, b'X' as c_char, b'8' as c_char, b'3' as c_char,
            b'1' as c_char, b'6' as c_char, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        driver_data: 0,
    },
    acpi_device_id {
        id: [
            b'E' as c_char, b'S' as c_char, b'S' as c_char, b'X' as c_char, b'8' as c_char, b'3' as c_char,
            b'3' as c_char, b'6' as c_char, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        driver_data: 0,
    },
    acpi_device_id { id: [0; 16], driver_data: 0 },
];
MODULE_DEVICE_TABLE!(acpi, ES8316_ACPI_MATCH);

static mut ES8316_I2C_DRIVER: i2c_driver = i2c_driver {
    driver: device_driver {
        name: cstr!("es8316"),
        acpi_match_table: ACPI_PTR!(ES8316_ACPI_MATCH),
        of_match_table: of_match_ptr!(ES8316_OF_MATCH),
    },
    probe: Some(es8316_i2c_probe),
    id_table: ES8316_I2C_ID.as_ptr(),
};
module_i2c_driver!(ES8316_I2C_DRIVER);

MODULE_DESCRIPTION!("Everest Semi ES8316 ALSA SoC Codec Driver");
MODULE_AUTHOR!("David Yang <yangxiaohua@everest-semi.com>");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
