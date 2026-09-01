// SPDX-License-Identifier: GPL-2.0
//
// ALSA SoC Texas Instruments TAS2770 20-W Digital Input Mono Class-D
// Audio Amplifier with Speaker I/V Sense
//
// Copyright (C) 2016-2017 Texas Instruments Incorporated - https://www.ti.com/
//	Author: Tracy Yi <tracy-yi@ti.com>
//	Frank Shi <shifu0704@thundersoft.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type bool_ = bool;
type u8 = u8;
type u32 = u32;
type umode_t = c_uint;

const TAS2770_MDELAY: c_uint = 0xFFFFFFFE;

const NULL: *mut c_void = ptr::null_mut();
const true_: c_int = 1;
const false_: c_int = 0;

extern "C" {
    static tas2770_ASI1_src_enum: soc_enum;
    static tas2770_i2c_regmap: regmap_config;

    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn msleep(msecs: c_uint);
    fn usleep_range(min: c_uint, max: c_uint);
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regcache_reinit_cache(map: *mut regmap, config: *const regmap_config) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_int) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn fwnode_property_read_u32(fwnode: *mut fwnode_handle, propname: *const c_char, val: *mut c_int) -> c_int;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn devm_hwmon_device_register_with_info(
        dev: *mut device,
        name: *const c_char,
        drvdata: *mut c_void,
        chip: *const hwmon_chip_info,
        groups: *const *const c_void,
    ) -> *mut device;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
}

#[repr(C)]
struct tas2770_priv {
    dev: *mut device,
    component: *mut snd_soc_component,
    regmap: *mut regmap,
    reset_gpio: *mut gpio_desc,
    sdz_gpio: *mut gpio_desc,
    dac_powered: c_int,
    unmuted: c_int,
    i_sense_slot: c_int,
    v_sense_slot: c_int,
    pdm_slot: c_int,
    idle_tx_mode: c_int,
}

#[repr(C)]
struct device {
    of_node: *mut c_void,
    fwnode: *mut fwnode_handle,
}
#[repr(C)]
struct i2c_client {
    dev: device,
}
#[repr(C)]
struct snd_soc_component;
#[repr(C)]
struct snd_soc_dai {
    component: *mut snd_soc_component,
}
#[repr(C)]
struct snd_pcm_substream;
#[repr(C)]
struct snd_pcm_hw_params;
#[repr(C)]
struct snd_soc_dapm_context;
#[repr(C)]
struct snd_soc_dapm_widget {
    dapm: *mut snd_soc_dapm_context,
}
#[repr(C)]
struct snd_kcontrol;
#[repr(C)]
struct regmap;
#[repr(C)]
struct gpio_desc;
#[repr(C)]
struct fwnode_handle;
#[repr(C)]
struct soc_enum;

#[repr(C)]
struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_soc_dapm_widget_init {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}
#[repr(C)]
struct snd_soc_dai_ops {
    mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    set_tdm_idle: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    no_capture_mute: c_uint,
}
#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: u64,
}
#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    id: c_int,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
    symmetric_rate: c_uint,
}
#[repr(C)]
struct hwmon_channel_info {
    _private: [u8; 0],
}
#[repr(C)]
struct hwmon_ops {
    is_visible: Option<unsafe extern "C" fn(*const c_void, hwmon_sensor_types, u32, c_int) -> umode_t>,
    read: Option<unsafe extern "C" fn(*mut device, hwmon_sensor_types, u32, c_int, *mut i64) -> c_int>,
}
#[repr(C)]
struct hwmon_chip_info {
    ops: *const hwmon_ops,
    info: *const *const hwmon_channel_info,
}
#[repr(C)]
struct reg_default {
    reg: c_uint,
    def: c_uint,
}
#[repr(C)]
struct regmap_range_cfg {
    range_min: c_uint,
    range_max: c_uint,
    selector_reg: c_uint,
    selector_mask: c_uint,
    selector_shift: c_uint,
    window_start: c_uint,
    window_len: c_uint,
}
#[repr(C)]
struct regmap_config {
    reg_bits: c_uint,
    val_bits: c_uint,
    writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    reg_defaults: *const reg_default,
    num_reg_defaults: c_uint,
    cache_type: c_uint,
    ranges: *const regmap_range_cfg,
    num_ranges: c_uint,
    max_register: c_uint,
}
#[repr(C)]
struct snd_soc_component_driver {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget_init,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    idle_bias_on: c_uint,
    endianness: c_uint,
}
#[repr(C)]
struct i2c_device_id {
    name: [c_char; 20],
}
#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}
#[repr(C)]
struct driver_private {
    name: *const c_char,
    of_match_table: *const of_device_id,
}
#[repr(C)]
struct i2c_driver {
    driver: driver_private,
    probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    id_table: *const i2c_device_id,
}

#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone)]
enum hwmon_sensor_types {
    hwmon_temp,
}

unsafe fn params_format(_params: *mut snd_pcm_hw_params) -> c_int {
    extern "C" {
        fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    }
    params_format(_params)
}

unsafe fn params_rate(_params: *mut snd_pcm_hw_params) -> c_int {
    extern "C" {
        fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    }
    params_rate(_params)
}

unsafe fn __ffs(word: c_uint) -> c_int {
    word.trailing_zeros() as c_int
}

unsafe fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

/* Constants normally supplied by Linux, ASoC, regmap, hwmon, and tas2770.h. */
extern "C" {
    static TAS2770_SW_RST: c_uint;
    static TAS2770_RST: c_uint;
    static TAS2770_PWR_CTRL: c_uint;
    static TAS2770_PWR_CTRL_ACTIVE: c_uint;
    static TAS2770_PWR_CTRL_MUTE: c_uint;
    static TAS2770_PWR_CTRL_SHUTDOWN: c_uint;
    static TAS2770_PWR_CTRL_MASK: c_uint;
    static TAS2770_TDM_CFG_REG2: c_uint;
    static TAS2770_TDM_CFG_REG5: c_uint;
    static TAS2770_TDM_CFG_REG5_VSNS_MASK: c_uint;
    static TAS2770_TDM_CFG_REG5_50_MASK: c_uint;
    static TAS2770_TDM_CFG_REG5_VSNS_ENABLE: c_uint;
    static TAS2770_TDM_CFG_REG6: c_uint;
    static TAS2770_TDM_CFG_REG6_ISNS_MASK: c_uint;
    static TAS2770_TDM_CFG_REG6_50_MASK: c_uint;
    static TAS2770_TDM_CFG_REG6_ISNS_ENABLE: c_uint;
    static TAS2770_TDM_CFG_REG7: c_uint;
    static TAS2770_TDM_CFG_REG7_PDM_MASK: c_uint;
    static TAS2770_TDM_CFG_REG7_50_MASK: c_uint;
    static TAS2770_TDM_CFG_REG7_PDM_ENABLE: c_uint;
    static TAS2770_TDM_CFG_REG2_RXW_MASK: c_uint;
    static TAS2770_TDM_CFG_REG2_RXW_16BITS: c_uint;
    static TAS2770_TDM_CFG_REG2_RXW_24BITS: c_uint;
    static TAS2770_TDM_CFG_REG2_RXW_32BITS: c_uint;
    static TAS2770_TDM_CFG_REG0: c_uint;
    static TAS2770_TDM_CFG_REG0_SMP_48KHZ: c_uint;
    static TAS2770_TDM_CFG_REG0_31_44_1_48KHZ: c_uint;
    static TAS2770_TDM_CFG_REG0_SMP_44_1KHZ: c_uint;
    static TAS2770_TDM_CFG_REG0_31_88_2_96KHZ: c_uint;
    static TAS2770_TDM_CFG_REG0_31_176_4_192KHZ: c_uint;
    static TAS2770_TDM_CFG_REG0_SMP_MASK: c_uint;
    static TAS2770_TDM_CFG_REG0_31_MASK: c_uint;
    static TAS2770_TDM_CFG_REG1: c_uint;
    static TAS2770_TDM_CFG_REG1_RX_RSING: c_uint;
    static TAS2770_TDM_CFG_REG1_RX_FALING: c_uint;
    static TAS2770_TDM_CFG_REG1_RX_MASK: c_uint;
    static TAS2770_TDM_CFG_REG1_MASK: c_uint;
    static TAS2770_TDM_CFG_REG1_51_SHIFT: c_uint;
    static TAS2770_TDM_CFG_REG0_FPOL_MASK: c_uint;
    static TAS2770_TDM_CFG_REG0_FPOL_RSING: c_uint;
    static TAS2770_TDM_CFG_REG0_FPOL_FALING: c_uint;
    static TAS2770_TDM_CFG_REG3: c_uint;
    static TAS2770_TDM_CFG_REG3_30_MASK: c_uint;
    static TAS2770_TDM_CFG_REG3_30_SHIFT: c_uint;
    static TAS2770_TDM_CFG_REG3_RXS_MASK: c_uint;
    static TAS2770_TDM_CFG_REG3_RXS_SHIFT: c_uint;
    static TAS2770_TDM_CFG_REG2_RXS_MASK: c_uint;
    static TAS2770_TDM_CFG_REG2_RXS_16BITS: c_uint;
    static TAS2770_TDM_CFG_REG2_RXS_24BITS: c_uint;
    static TAS2770_TDM_CFG_REG2_RXS_32BITS: c_uint;
    static TAS2770_DIN_PD: c_uint;
    static TAS2770_DIN_PD_SDOUT: c_uint;
    static TAS2770_TDM_CFG_REG4: c_uint;
    static TAS2770_TDM_CFG_REG4_TX_KEEPER: c_uint;
    static TAS2770_TDM_CFG_REG4_TX_FILL: c_uint;
    static TAS2770_TEMP_MSB: c_uint;
    static TAS2770_TEMP_LSB: c_uint;
    static TAS2770_PLAY_CFG_REG2: c_uint;
    static TAS2770_PLAY_CFG_REG2_VMAX: c_uint;
    static TAS2770_PLAY_CFG_REG0: c_uint;
    static TAS2770_PAGE: c_uint;
    static TAS2770_PLAY_CFG_REG1: c_uint;
    static TAS2770_MSC_CFG_REG0: c_uint;
    static TAS2770_INT_MASK_REG0: c_uint;
    static TAS2770_INT_MASK_REG1: c_uint;
    static TAS2770_INT_CFG: c_uint;
    static TAS2770_MISC_IRQ: c_uint;
    static TAS2770_CLK_CGF: c_uint;
    static TAS2770_BO_PRV_REG0: c_uint;
    static TAS2770_LVE_INT_REG0: c_uint;
    static TAS2770_LVE_INT_REG1: c_uint;
    static TAS2770_LAT_INT_REG0: c_uint;
    static TAS2770_LAT_INT_REG1: c_uint;
    static TAS2770_VBAT_MSB: c_uint;
    static TAS2770_VBAT_LSB: c_uint;
    static TAS2770_TDM_CLK_DETC: c_uint;
    static TAS2770_REV_AND_GPID: c_uint;
    static SNDRV_PCM_FORMAT_S16_LE: c_int;
    static SNDRV_PCM_FORMAT_S24_LE: c_int;
    static SNDRV_PCM_FORMAT_S32_LE: c_int;
    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static SND_SOC_DAPM_PRE_REG: c_int;
    static SND_SOC_DAPM_POST_REG: c_int;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAI_TDM_IDLE_PULLDOWN: c_int;
    static SND_SOC_DAI_TDM_IDLE_ZERO: c_int;
    static SND_SOC_DAI_TDM_IDLE_HIZ: c_int;
    static SND_SOC_DAI_TDM_IDLE_OFF: c_int;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S20_3LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_192000: c_uint;
    static hwmon_temp_input: u32;
    static hwmon_temp_fault: u32;
    static HWMON_T_INPUT: c_uint;
    static HWMON_T_FAULT: c_uint;
    static REGCACHE_RBTREE: c_uint;
    static GFP_KERNEL: c_uint;
    static GPIOD_OUT_HIGH: c_uint;
    static CONFIG_HWMON: c_int;
}

const EINVAL: c_int = 22;
const EOPNOTSUPP: c_int = 95;
const ENODATA: c_int = 61;
const EPROBE_DEFER: c_int = 517;
const ENOMEM: c_int = 12;

unsafe extern "C" fn tas2770_reset(tas2770: *mut tas2770_priv) {
    if !(*tas2770).reset_gpio.is_null() {
        gpiod_set_value_cansleep((*tas2770).reset_gpio, 0);
        msleep(20);
        gpiod_set_value_cansleep((*tas2770).reset_gpio, 1);
        usleep_range(1000, 2000);
    }

    snd_soc_component_write((*tas2770).component, TAS2770_SW_RST, TAS2770_RST);
    usleep_range(1000, 2000);
}

unsafe extern "C" fn tas2770_update_pwr_ctrl(tas2770: *mut tas2770_priv) -> c_int {
    let component = (*tas2770).component;
    let val: c_uint;
    let ret: c_int;

    if (*tas2770).dac_powered != 0 {
        val = if (*tas2770).unmuted != 0 {
            TAS2770_PWR_CTRL_ACTIVE
        } else {
            TAS2770_PWR_CTRL_MUTE
        };
    } else {
        val = TAS2770_PWR_CTRL_SHUTDOWN;
    }

    ret = snd_soc_component_update_bits(component, TAS2770_PWR_CTRL, TAS2770_PWR_CTRL_MASK, val);
    if ret < 0 {
        return ret;
    }

    0
}

/* CONFIG_PM maps these callbacks to NULL when disabled in C. */
unsafe extern "C" fn tas2770_codec_suspend(component: *mut snd_soc_component) -> c_int {
    let tas2770 = snd_soc_component_get_drvdata(component) as *mut tas2770_priv;
    let mut ret: c_int = 0;

    regcache_cache_only((*tas2770).regmap, true);
    regcache_mark_dirty((*tas2770).regmap);

    if !(*tas2770).sdz_gpio.is_null() {
        gpiod_set_value_cansleep((*tas2770).sdz_gpio, 0);
    } else {
        ret = snd_soc_component_update_bits(
            component,
            TAS2770_PWR_CTRL,
            TAS2770_PWR_CTRL_MASK,
            TAS2770_PWR_CTRL_SHUTDOWN,
        );
        if ret < 0 {
            regcache_cache_only((*tas2770).regmap, false);
            regcache_sync((*tas2770).regmap);
            return ret;
        }

        ret = 0;
    }

    ret
}

unsafe extern "C" fn tas2770_codec_resume(component: *mut snd_soc_component) -> c_int {
    let tas2770 = snd_soc_component_get_drvdata(component) as *mut tas2770_priv;
    let ret: c_int;

    if !(*tas2770).sdz_gpio.is_null() {
        gpiod_set_value_cansleep((*tas2770).sdz_gpio, 1);
        usleep_range(1000, 2000);
    } else {
        ret = tas2770_update_pwr_ctrl(tas2770);
        if ret < 0 {
            return ret;
        }
    }

    regcache_cache_only((*tas2770).regmap, false);

    regcache_sync((*tas2770).regmap)
}

static tas2770_ASI1_src: [*const c_char; 4] = [
    b"I2C offset\0".as_ptr() as *const c_char,
    b"Left\0".as_ptr() as *const c_char,
    b"Right\0".as_ptr() as *const c_char,
    b"LeftRightDiv2\0".as_ptr() as *const c_char,
];

/* SOC_ENUM_SINGLE_DECL(tas2770_ASI1_src_enum, TAS2770_TDM_CFG_REG2, 4, tas2770_ASI1_src); */
static tas2770_asi1_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

unsafe extern "C" fn tas2770_dac_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let tas2770 = snd_soc_component_get_drvdata(component) as *mut tas2770_priv;
    let ret: c_int;

    if event == SND_SOC_DAPM_POST_PMU {
        (*tas2770).dac_powered = 1;
        ret = tas2770_update_pwr_ctrl(tas2770);
    } else if event == SND_SOC_DAPM_PRE_PMD {
        (*tas2770).dac_powered = 0;
        ret = tas2770_update_pwr_ctrl(tas2770);
    } else {
        dev_err((*tas2770).dev, b"Not supported evevt\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    ret
}

static isense_switch: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static vsense_switch: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

unsafe extern "C" fn sense_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let tas2770 = snd_soc_component_get_drvdata(component) as *mut tas2770_priv;

    /*
     * Powering up ISENSE/VSENSE requires a trip through the shutdown state.
     * Do that here to ensure that our changes are applied properly, otherwise
     * we might end up with non-functional IVSENSE if playback started earlier,
     * which would break software speaker protection.
     */
    if event == SND_SOC_DAPM_PRE_REG {
        return snd_soc_component_update_bits(
            component,
            TAS2770_PWR_CTRL,
            TAS2770_PWR_CTRL_MASK,
            TAS2770_PWR_CTRL_SHUTDOWN,
        );
    } else if event == SND_SOC_DAPM_POST_REG {
        return tas2770_update_pwr_ctrl(tas2770);
    }

    0
}

/* SND_SOC_DAPM_* macro initializers from C are represented as opaque widget values. */
static tas2770_dapm_widgets: [snd_soc_dapm_widget_init; 8] = [
    snd_soc_dapm_widget_init { _private: [] },
    snd_soc_dapm_widget_init { _private: [] },
    snd_soc_dapm_widget_init { _private: [] },
    snd_soc_dapm_widget_init { _private: [] },
    snd_soc_dapm_widget_init { _private: [] },
    snd_soc_dapm_widget_init { _private: [] },
    snd_soc_dapm_widget_init { _private: [] },
    snd_soc_dapm_widget_init { _private: [] },
];

static tas2770_audio_map: [snd_soc_dapm_route; 8] = [
    snd_soc_dapm_route { sink: b"ASI1 Sel\0".as_ptr() as *const c_char, control: b"I2C offset\0".as_ptr() as *const c_char, source: b"ASI1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ASI1 Sel\0".as_ptr() as *const c_char, control: b"Left\0".as_ptr() as *const c_char, source: b"ASI1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ASI1 Sel\0".as_ptr() as *const c_char, control: b"Right\0".as_ptr() as *const c_char, source: b"ASI1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ASI1 Sel\0".as_ptr() as *const c_char, control: b"LeftRightDiv2\0".as_ptr() as *const c_char, source: b"ASI1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ASI1 Sel\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"OUT\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ISENSE\0".as_ptr() as *const c_char, control: b"Switch\0".as_ptr() as *const c_char, source: b"IMON\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"VSENSE\0".as_ptr() as *const c_char, control: b"Switch\0".as_ptr() as *const c_char, source: b"VMON\0".as_ptr() as *const c_char },
];

unsafe extern "C" fn tas2770_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*dai).component;
    let tas2770 = snd_soc_component_get_drvdata(component) as *mut tas2770_priv;

    (*tas2770).unmuted = if mute == 0 { 1 } else { 0 };
    tas2770_update_pwr_ctrl(tas2770)
}

unsafe extern "C" fn tas2770_set_ivsense_transmit(tas2770: *mut tas2770_priv, i_slot: c_int, v_slot: c_int) -> c_int {
    let component = (*tas2770).component;
    let mut ret: c_int;

    ret = snd_soc_component_update_bits(
        component,
        TAS2770_TDM_CFG_REG5,
        TAS2770_TDM_CFG_REG5_VSNS_MASK | TAS2770_TDM_CFG_REG5_50_MASK,
        TAS2770_TDM_CFG_REG5_VSNS_ENABLE | v_slot as c_uint,
    );
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_component_update_bits(
        component,
        TAS2770_TDM_CFG_REG6,
        TAS2770_TDM_CFG_REG6_ISNS_MASK | TAS2770_TDM_CFG_REG6_50_MASK,
        TAS2770_TDM_CFG_REG6_ISNS_ENABLE | i_slot as c_uint,
    );
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn tas2770_set_pdm_transmit(tas2770: *mut tas2770_priv, slot: c_int) -> c_int {
    let component = (*tas2770).component;
    let ret: c_int;

    ret = snd_soc_component_update_bits(
        component,
        TAS2770_TDM_CFG_REG7,
        TAS2770_TDM_CFG_REG7_PDM_MASK | TAS2770_TDM_CFG_REG7_50_MASK,
        TAS2770_TDM_CFG_REG7_PDM_ENABLE | slot as c_uint,
    );
    ret
}

unsafe extern "C" fn tas2770_set_bitwidth(tas2770: *mut tas2770_priv, bitwidth: c_int) -> c_int {
    let ret: c_int;
    let component = (*tas2770).component;

    if bitwidth == SNDRV_PCM_FORMAT_S16_LE {
        ret = snd_soc_component_update_bits(
            component,
            TAS2770_TDM_CFG_REG2,
            TAS2770_TDM_CFG_REG2_RXW_MASK,
            TAS2770_TDM_CFG_REG2_RXW_16BITS,
        );
    } else if bitwidth == SNDRV_PCM_FORMAT_S24_LE {
        ret = snd_soc_component_update_bits(
            component,
            TAS2770_TDM_CFG_REG2,
            TAS2770_TDM_CFG_REG2_RXW_MASK,
            TAS2770_TDM_CFG_REG2_RXW_24BITS,
        );
    } else if bitwidth == SNDRV_PCM_FORMAT_S32_LE {
        ret = snd_soc_component_update_bits(
            component,
            TAS2770_TDM_CFG_REG2,
            TAS2770_TDM_CFG_REG2_RXW_MASK,
            TAS2770_TDM_CFG_REG2_RXW_32BITS,
        );
    } else {
        return -EINVAL;
    }

    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn tas2770_set_samplerate(tas2770: *mut tas2770_priv, samplerate: c_int) -> c_int {
    let component = (*tas2770).component;
    let ramp_rate_val: c_uint;
    let ret: c_int;

    match samplerate {
        48000 => ramp_rate_val = TAS2770_TDM_CFG_REG0_SMP_48KHZ | TAS2770_TDM_CFG_REG0_31_44_1_48KHZ,
        44100 => ramp_rate_val = TAS2770_TDM_CFG_REG0_SMP_44_1KHZ | TAS2770_TDM_CFG_REG0_31_44_1_48KHZ,
        96000 => ramp_rate_val = TAS2770_TDM_CFG_REG0_SMP_48KHZ | TAS2770_TDM_CFG_REG0_31_88_2_96KHZ,
        88200 => ramp_rate_val = TAS2770_TDM_CFG_REG0_SMP_44_1KHZ | TAS2770_TDM_CFG_REG0_31_88_2_96KHZ,
        192000 => ramp_rate_val = TAS2770_TDM_CFG_REG0_SMP_48KHZ | TAS2770_TDM_CFG_REG0_31_176_4_192KHZ,
        176400 => ramp_rate_val = TAS2770_TDM_CFG_REG0_SMP_44_1KHZ | TAS2770_TDM_CFG_REG0_31_176_4_192KHZ,
        _ => return -EINVAL,
    }

    ret = snd_soc_component_update_bits(
        component,
        TAS2770_TDM_CFG_REG0,
        TAS2770_TDM_CFG_REG0_SMP_MASK | TAS2770_TDM_CFG_REG0_31_MASK,
        ramp_rate_val,
    );
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn tas2770_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let tas2770 = snd_soc_component_get_drvdata(component) as *mut tas2770_priv;
    let mut ret: c_int;

    ret = tas2770_set_bitwidth(tas2770, params_format(params));
    if ret != 0 {
        return ret;
    }

    tas2770_set_samplerate(tas2770, params_rate(params))
}

unsafe extern "C" fn tas2770_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let tas2770 = snd_soc_component_get_drvdata(component) as *mut tas2770_priv;
    let mut tdm_rx_start_slot: u8 = 0;
    let mut invert_fpol: u8 = 0;
    let mut fpol_preinv: u8 = 0;
    let mut asi_cfg_1: u8 = 0;
    let mut ret: c_int;

    if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_CBC_CFC {
    } else {
        dev_err((*tas2770).dev, b"ASI invalid DAI clocking\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    if (fmt & SND_SOC_DAIFMT_INV_MASK) == SND_SOC_DAIFMT_NB_IF {
        invert_fpol = 1;
        asi_cfg_1 |= TAS2770_TDM_CFG_REG1_RX_RSING as u8;
    } else if (fmt & SND_SOC_DAIFMT_INV_MASK) == SND_SOC_DAIFMT_NB_NF {
        asi_cfg_1 |= TAS2770_TDM_CFG_REG1_RX_RSING as u8;
    } else if (fmt & SND_SOC_DAIFMT_INV_MASK) == SND_SOC_DAIFMT_IB_IF {
        invert_fpol = 1;
        asi_cfg_1 |= TAS2770_TDM_CFG_REG1_RX_FALING as u8;
    } else if (fmt & SND_SOC_DAIFMT_INV_MASK) == SND_SOC_DAIFMT_IB_NF {
        asi_cfg_1 |= TAS2770_TDM_CFG_REG1_RX_FALING as u8;
    } else {
        dev_err((*tas2770).dev, b"ASI format Inverse is not found\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    ret = snd_soc_component_update_bits(
        component,
        TAS2770_TDM_CFG_REG1,
        TAS2770_TDM_CFG_REG1_RX_MASK,
        asi_cfg_1 as c_uint,
    );
    if ret < 0 {
        return ret;
    }

    if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_I2S {
        tdm_rx_start_slot = 1;
        fpol_preinv = 0;
    } else if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_DSP_A {
        tdm_rx_start_slot = 0;
        fpol_preinv = 1;
    } else if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_DSP_B {
        tdm_rx_start_slot = 1;
        fpol_preinv = 1;
    } else if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_LEFT_J {
        tdm_rx_start_slot = 0;
        fpol_preinv = 1;
    } else {
        dev_err((*tas2770).dev, b"DAI Format is not found, fmt=0x%x\n\0".as_ptr() as *const c_char, fmt);
        return -EINVAL;
    }

    ret = snd_soc_component_update_bits(
        component,
        TAS2770_TDM_CFG_REG1,
        TAS2770_TDM_CFG_REG1_MASK,
        (tdm_rx_start_slot as c_uint) << TAS2770_TDM_CFG_REG1_51_SHIFT,
    );
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_component_update_bits(
        component,
        TAS2770_TDM_CFG_REG0,
        TAS2770_TDM_CFG_REG0_FPOL_MASK,
        if (fpol_preinv ^ invert_fpol) != 0 {
            TAS2770_TDM_CFG_REG0_FPOL_RSING
        } else {
            TAS2770_TDM_CFG_REG0_FPOL_FALING
        },
    );
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn tas2770_set_dai_tdm_slot(
    dai: *mut snd_soc_dai,
    mut tx_mask: c_uint,
    rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let component = (*dai).component;
    let left_slot: c_int;
    let right_slot: c_int;
    let mut ret: c_int;

    if tx_mask == 0 || rx_mask != 0 {
        return -EINVAL;
    }

    left_slot = __ffs(tx_mask);
    tx_mask &= !(1_u32 << left_slot);
    if tx_mask == 0 {
        right_slot = left_slot;
    } else {
        right_slot = __ffs(tx_mask);
        tx_mask &= !(1_u32 << right_slot);
    }

    if tx_mask != 0 || left_slot >= slots || right_slot >= slots {
        return -EINVAL;
    }

    ret = snd_soc_component_update_bits(
        component,
        TAS2770_TDM_CFG_REG3,
        TAS2770_TDM_CFG_REG3_30_MASK,
        (left_slot as c_uint) << TAS2770_TDM_CFG_REG3_30_SHIFT,
    );
    if ret < 0 {
        return ret;
    }
    ret = snd_soc_component_update_bits(
        component,
        TAS2770_TDM_CFG_REG3,
        TAS2770_TDM_CFG_REG3_RXS_MASK,
        (right_slot as c_uint) << TAS2770_TDM_CFG_REG3_RXS_SHIFT,
    );
    if ret < 0 {
        return ret;
    }

    match slot_width {
        16 => {
            ret = snd_soc_component_update_bits(component, TAS2770_TDM_CFG_REG2, TAS2770_TDM_CFG_REG2_RXS_MASK, TAS2770_TDM_CFG_REG2_RXS_16BITS);
        }
        24 => {
            ret = snd_soc_component_update_bits(component, TAS2770_TDM_CFG_REG2, TAS2770_TDM_CFG_REG2_RXS_MASK, TAS2770_TDM_CFG_REG2_RXS_24BITS);
        }
        32 => {
            ret = snd_soc_component_update_bits(component, TAS2770_TDM_CFG_REG2, TAS2770_TDM_CFG_REG2_RXS_MASK, TAS2770_TDM_CFG_REG2_RXS_32BITS);
        }
        0 => {
            /* Do not change slot width */
            ret = 0;
        }
        _ => {
            ret = -EINVAL;
        }
    }

    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn tas2770_set_dai_tdm_idle(
    dai: *mut snd_soc_dai,
    _tx_mask: c_uint,
    _rx_mask: c_uint,
    tx_mode: c_int,
    rx_mode: c_int,
) -> c_int {
    let component = (*dai).component;
    let tas2770 = snd_soc_component_get_drvdata(component) as *mut tas2770_priv;
    let mut ret: c_int;

    /* We don't support setting anything for SDIN */
    if rx_mode != 0 {
        return -EOPNOTSUPP;
    }

    if (*tas2770).idle_tx_mode == tx_mode {
        return 0;
    }

    if tx_mode == SND_SOC_DAI_TDM_IDLE_PULLDOWN {
        ret = snd_soc_component_update_bits(component, TAS2770_DIN_PD, TAS2770_DIN_PD_SDOUT, TAS2770_DIN_PD_SDOUT);
        if ret != 0 {
            return ret;
        }
    } else if tx_mode == SND_SOC_DAI_TDM_IDLE_ZERO {
        ret = snd_soc_component_update_bits(component, TAS2770_TDM_CFG_REG4, TAS2770_TDM_CFG_REG4_TX_KEEPER, TAS2770_TDM_CFG_REG4_TX_KEEPER);
        if ret != 0 {
            return ret;
        }

        ret = snd_soc_component_update_bits(component, TAS2770_TDM_CFG_REG4, TAS2770_TDM_CFG_REG4_TX_FILL, 0);
        if ret != 0 {
            return ret;
        }
    } else if tx_mode == SND_SOC_DAI_TDM_IDLE_HIZ {
        ret = snd_soc_component_update_bits(component, TAS2770_TDM_CFG_REG4, TAS2770_TDM_CFG_REG4_TX_KEEPER, TAS2770_TDM_CFG_REG4_TX_KEEPER);
        if ret != 0 {
            return ret;
        }

        ret = snd_soc_component_update_bits(component, TAS2770_TDM_CFG_REG4, TAS2770_TDM_CFG_REG4_TX_FILL, TAS2770_TDM_CFG_REG4_TX_FILL);
        if ret != 0 {
            return ret;
        }
    } else if tx_mode == SND_SOC_DAI_TDM_IDLE_OFF {
        ret = snd_soc_component_update_bits(component, TAS2770_DIN_PD, TAS2770_DIN_PD_SDOUT, 0);
        if ret != 0 {
            return ret;
        }

        ret = snd_soc_component_update_bits(component, TAS2770_TDM_CFG_REG4, TAS2770_TDM_CFG_REG4_TX_KEEPER, 0);
        if ret != 0 {
            return ret;
        }
    } else {
        return -EOPNOTSUPP;
    }

    (*tas2770).idle_tx_mode = tx_mode;

    0
}

static tas2770_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    mute_stream: Some(tas2770_mute),
    hw_params: Some(tas2770_hw_params),
    set_fmt: Some(tas2770_set_fmt),
    set_tdm_slot: Some(tas2770_set_dai_tdm_slot),
    set_tdm_idle: Some(tas2770_set_dai_tdm_idle),
    no_capture_mute: 1,
};

unsafe fn TAS2770_FORMATS() -> u64 {
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE
}

unsafe fn TAS2770_RATES() -> c_uint {
    SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000
}

static mut tas2770_dai_driver: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: b"tas2770 ASI1\0".as_ptr() as *const c_char,
    id: 0,
    playback: snd_soc_pcm_stream {
        stream_name: b"ASI1 Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: 0,
        formats: 0,
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"ASI1 Capture\0".as_ptr() as *const c_char,
        channels_min: 0,
        channels_max: 2,
        rates: 0,
        formats: 0,
    },
    ops: unsafe { &tas2770_dai_ops },
    symmetric_rate: 1,
}];

unsafe extern "C" fn tas2770_read_die_temp(tas2770: *mut tas2770_priv, result: *mut i64) -> c_int {
    let mut ret: c_int = 0;
    let mut reading: c_int;
    let mut msb: c_int = 0;
    let mut lsb: c_int = 0;

    ret = regmap_read((*tas2770).regmap, TAS2770_TEMP_MSB, &mut msb);
    if ret != 0 {
        return ret;
    }

    ret = regmap_read((*tas2770).regmap, TAS2770_TEMP_LSB, &mut lsb);
    if ret != 0 {
        return ret;
    }

    reading = (msb << 4) | (lsb >> 4);

    /*
     * As per datasheet: divide register by 16 and subtract 93 to get
     * degrees Celsius. hwmon requires millidegrees. Let's avoid rounding
     * errors by subtracting 93 * 16 and scaling before dividing.
     *
     * NOTE: The ADC registers are initialised to 0 on reset. This means
     * that the temperature will read -93 *C until the chip is brought out
     * of software shutdown (e.g. the PCM it's attached to is opened). The
     * ADC is also shut down in software shutdown/low-power mode, so the
     * value read back from its registers will be the last value sampled
     * before entering software shutdown.
     */
    if reading == 0 {
        return -ENODATA;
    }

    *result = ((reading - (93 * 16)) * 1000 / 16) as i64;
    0
}

unsafe extern "C" fn tas2770_hwmon_is_fault(tas2770: *mut tas2770_priv, result: *mut i64) -> c_int {
    let ret: c_int;
    let mut temp: i64 = 0;

    ret = tas2770_read_die_temp(tas2770, &mut temp);
    if ret == -ENODATA {
        *result = true_ as i64;
        return 0;
    }

    ret
}

unsafe extern "C" fn tas2770_hwmon_is_visible(
    _data: *const c_void,
    type_: hwmon_sensor_types,
    attr: u32,
    _channel: c_int,
) -> umode_t {
    if type_ != hwmon_sensor_types::hwmon_temp {
        return 0;
    }

    if attr == hwmon_temp_input || attr == hwmon_temp_fault {
        return 0o444;
    }

    0
}

unsafe extern "C" fn tas2770_hwmon_read(
    dev: *mut device,
    _type: hwmon_sensor_types,
    attr: u32,
    _channel: c_int,
    val: *mut i64,
) -> c_int {
    let tas2770 = dev_get_drvdata(dev) as *mut tas2770_priv;
    let ret: c_int;

    if attr == hwmon_temp_input {
        ret = tas2770_read_die_temp(tas2770, val);
    } else if attr == hwmon_temp_fault {
        ret = tas2770_hwmon_is_fault(tas2770, val);
    } else {
        ret = -EOPNOTSUPP;
    }

    ret
}

/* HWMON_CHANNEL_INFO(temp, HWMON_T_INPUT | HWMON_T_FAULT), NULL */
static tas2770_hwmon_channel_temp: hwmon_channel_info = hwmon_channel_info { _private: [] };
static tas2770_hwmon_info: [*const hwmon_channel_info; 2] = [
    &tas2770_hwmon_channel_temp,
    ptr::null(),
];

static tas2770_hwmon_ops: hwmon_ops = hwmon_ops {
    is_visible: Some(tas2770_hwmon_is_visible),
    read: Some(tas2770_hwmon_read),
};

static tas2770_hwmon_chip_info: hwmon_chip_info = hwmon_chip_info {
    ops: &tas2770_hwmon_ops,
    info: tas2770_hwmon_info.as_ptr(),
};

unsafe extern "C" fn tas2770_codec_probe(component: *mut snd_soc_component) -> c_int {
    let tas2770 = snd_soc_component_get_drvdata(component) as *mut tas2770_priv;
    let mut ret: c_int;

    (*tas2770).component = component;

    if !(*tas2770).sdz_gpio.is_null() {
        gpiod_set_value_cansleep((*tas2770).sdz_gpio, 1);
        usleep_range(1000, 2000);
    }

    tas2770_reset(tas2770);
    regcache_reinit_cache((*tas2770).regmap, &tas2770_i2c_regmap);

    if (*tas2770).i_sense_slot != -1 && (*tas2770).v_sense_slot != -1 {
        ret = tas2770_set_ivsense_transmit(tas2770, (*tas2770).i_sense_slot, (*tas2770).v_sense_slot);

        if ret < 0 {
            return ret;
        }
    }

    if (*tas2770).pdm_slot != -1 {
        ret = tas2770_set_pdm_transmit(tas2770, (*tas2770).pdm_slot);

        if ret < 0 {
            return ret;
        }
    }

    0
}

/* DECLARE_TLV_DB_SCALE(tas2770_digital_tlv, 1100, 50, 0); */
static tas2770_digital_tlv: [c_uint; 4] = [0, 1100u32, 50u32, 0u32];
/* DECLARE_TLV_DB_SCALE(tas2770_playback_volume, -10050, 50, 0); */
static tas2770_playback_volume: [c_int; 4] = [0, -10050, 50, 0];

/* SOC_SINGLE_TLV controls from C are represented as opaque kcontrol values. */
static tas2770_snd_controls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

static soc_component_driver_tas2770: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(tas2770_codec_probe),
    suspend: Some(tas2770_codec_suspend),
    resume: Some(tas2770_codec_resume),
    controls: tas2770_snd_controls.as_ptr(),
    num_controls: 2,
    dapm_widgets: tas2770_dapm_widgets.as_ptr(),
    num_dapm_widgets: 8,
    dapm_routes: tas2770_audio_map.as_ptr(),
    num_dapm_routes: 8,
    idle_bias_on: 1,
    endianness: 1,
};

unsafe extern "C" fn tas2770_register_codec(tas2770: *mut tas2770_priv) -> c_int {
    devm_snd_soc_register_component(
        (*tas2770).dev,
        &soc_component_driver_tas2770,
        tas2770_dai_driver.as_mut_ptr(),
        ARRAY_SIZE(&tas2770_dai_driver),
    )
}

static tas2770_reg_defaults: [reg_default; 16] = [
    reg_default { reg: unsafe { TAS2770_PAGE }, def: 0x00 },
    reg_default { reg: unsafe { TAS2770_SW_RST }, def: 0x00 },
    reg_default { reg: unsafe { TAS2770_PWR_CTRL }, def: 0x0e },
    reg_default { reg: unsafe { TAS2770_PLAY_CFG_REG0 }, def: 0x10 },
    reg_default { reg: unsafe { TAS2770_PLAY_CFG_REG1 }, def: 0x01 },
    reg_default { reg: unsafe { TAS2770_PLAY_CFG_REG2 }, def: 0x00 },
    reg_default { reg: unsafe { TAS2770_MSC_CFG_REG0 }, def: 0x07 },
    reg_default { reg: unsafe { TAS2770_TDM_CFG_REG1 }, def: 0x02 },
    reg_default { reg: unsafe { TAS2770_TDM_CFG_REG2 }, def: 0x0a },
    reg_default { reg: unsafe { TAS2770_TDM_CFG_REG3 }, def: 0x10 },
    reg_default { reg: unsafe { TAS2770_INT_MASK_REG0 }, def: 0xfc },
    reg_default { reg: unsafe { TAS2770_INT_MASK_REG1 }, def: 0xb1 },
    reg_default { reg: unsafe { TAS2770_INT_CFG }, def: 0x05 },
    reg_default { reg: unsafe { TAS2770_MISC_IRQ }, def: 0x81 },
    reg_default { reg: unsafe { TAS2770_CLK_CGF }, def: 0x0c },
    reg_default { reg: 0, def: 0 },
];

unsafe extern "C" fn tas2770_volatile(_dev: *mut device, reg: c_uint) -> bool_ {
    if reg == TAS2770_PAGE
        || reg == TAS2770_SW_RST
        || reg == TAS2770_BO_PRV_REG0
        || reg == TAS2770_LVE_INT_REG0
        || reg == TAS2770_LVE_INT_REG1
        || reg == TAS2770_LAT_INT_REG0
        || reg == TAS2770_LAT_INT_REG1
        || reg == TAS2770_VBAT_MSB
        || reg == TAS2770_VBAT_LSB
        || reg == TAS2770_TEMP_MSB
        || reg == TAS2770_TEMP_LSB
    {
        return true;
    }

    false
}

unsafe extern "C" fn tas2770_writeable(_dev: *mut device, reg: c_uint) -> bool_ {
    if reg == TAS2770_LVE_INT_REG0
        || reg == TAS2770_LVE_INT_REG1
        || reg == TAS2770_LAT_INT_REG0
        || reg == TAS2770_LAT_INT_REG1
        || reg == TAS2770_VBAT_MSB
        || reg == TAS2770_VBAT_LSB
        || reg == TAS2770_TEMP_MSB
        || reg == TAS2770_TEMP_LSB
        || reg == TAS2770_TDM_CLK_DETC
        || reg == TAS2770_REV_AND_GPID
    {
        return false;
    }

    true
}

static tas2770_regmap_ranges: [regmap_range_cfg; 1] = [regmap_range_cfg {
    range_min: 0,
    range_max: 1 * 128,
    selector_reg: unsafe { TAS2770_PAGE },
    selector_mask: 0xff,
    selector_shift: 0,
    window_start: 0,
    window_len: 128,
}];

static tas2770_i2c_regmap_local: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    writeable_reg: Some(tas2770_writeable),
    volatile_reg: Some(tas2770_volatile),
    reg_defaults: tas2770_reg_defaults.as_ptr(),
    num_reg_defaults: 16,
    cache_type: unsafe { REGCACHE_RBTREE },
    ranges: tas2770_regmap_ranges.as_ptr(),
    num_ranges: 1,
    max_register: 1 * 128,
};

unsafe extern "C" fn tas2770_parse_dt(dev: *mut device, tas2770: *mut tas2770_priv) -> c_int {
    let mut rc: c_int = 0;

    rc = fwnode_property_read_u32((*dev).fwnode, b"ti,imon-slot-no\0".as_ptr() as *const c_char, &mut (*tas2770).i_sense_slot);
    if rc != 0 {
        dev_info(
            (*tas2770).dev,
            b"Property %s is missing setting default slot\n\0".as_ptr() as *const c_char,
            b"ti,imon-slot-no\0".as_ptr() as *const c_char,
        );

        (*tas2770).i_sense_slot = -1;
    }

    rc = fwnode_property_read_u32((*dev).fwnode, b"ti,vmon-slot-no\0".as_ptr() as *const c_char, &mut (*tas2770).v_sense_slot);
    if rc != 0 {
        dev_info(
            (*tas2770).dev,
            b"Property %s is missing setting default slot\n\0".as_ptr() as *const c_char,
            b"ti,vmon-slot-no\0".as_ptr() as *const c_char,
        );

        (*tas2770).v_sense_slot = -1;
    }

    rc = fwnode_property_read_u32((*dev).fwnode, b"ti,pdm-slot-no\0".as_ptr() as *const c_char, &mut (*tas2770).pdm_slot);
    if rc != 0 {
        (*tas2770).pdm_slot = -1;
    }

    (*tas2770).sdz_gpio = devm_gpiod_get_optional(dev, b"shutdown\0".as_ptr() as *const c_char, GPIOD_OUT_HIGH);
    if IS_ERR((*tas2770).sdz_gpio as *const c_void) {
        if PTR_ERR((*tas2770).sdz_gpio as *const c_void) == -EPROBE_DEFER {
            return -EPROBE_DEFER;
        }

        (*tas2770).sdz_gpio = ptr::null_mut();
    }

    0
}

unsafe extern "C" fn tas2770_i2c_probe(client: *mut i2c_client) -> c_int {
    let tas2770: *mut tas2770_priv;
    let mut result: c_int;

    tas2770 = devm_kzalloc(&mut (*client).dev, core::mem::size_of::<tas2770_priv>(), GFP_KERNEL) as *mut tas2770_priv;
    if tas2770.is_null() {
        return -ENOMEM;
    }

    (*tas2770).dev = &mut (*client).dev;
    i2c_set_clientdata(client, tas2770 as *mut c_void);
    dev_set_drvdata(&mut (*client).dev, tas2770 as *mut c_void);

    (*tas2770).regmap = devm_regmap_init_i2c(client, &tas2770_i2c_regmap);
    if IS_ERR((*tas2770).regmap as *const c_void) {
        result = PTR_ERR((*tas2770).regmap as *const c_void);
        dev_err(&mut (*client).dev, b"Failed to allocate register map: %d\n\0".as_ptr() as *const c_char, result);
        return result;
    }

    if !(*client).dev.of_node.is_null() {
        result = tas2770_parse_dt(&mut (*client).dev, tas2770);
        if result != 0 {
            dev_err((*tas2770).dev, b"%s: Failed to parse devicetree\n\0".as_ptr() as *const c_char, b"tas2770_i2c_probe\0".as_ptr() as *const c_char);
            return result;
        }
    }

    (*tas2770).reset_gpio = devm_gpiod_get_optional((*tas2770).dev, b"reset\0".as_ptr() as *const c_char, GPIOD_OUT_HIGH);
    if IS_ERR((*tas2770).reset_gpio as *const c_void) {
        if PTR_ERR((*tas2770).reset_gpio as *const c_void) == -EPROBE_DEFER {
            (*tas2770).reset_gpio = ptr::null_mut();
            return -EPROBE_DEFER;
        }
    }

    if CONFIG_HWMON != 0 {
        let hwmon: *mut device;

        hwmon = devm_hwmon_device_register_with_info(
            &mut (*client).dev,
            b"tas2770\0".as_ptr() as *const c_char,
            tas2770 as *mut c_void,
            &tas2770_hwmon_chip_info,
            ptr::null(),
        );
        if IS_ERR(hwmon as *const c_void) {
            return dev_err_probe(
                &mut (*client).dev,
                PTR_ERR(hwmon as *const c_void),
                b"Failed to register temp sensor\n\0".as_ptr() as *const c_char,
            );
        }
    }

    result = tas2770_register_codec(tas2770);
    if result != 0 {
        dev_err((*tas2770).dev, b"Register codec failed.\n\0".as_ptr() as *const c_char);
    }

    result
}

static tas2770_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: [116, 97, 115, 50, 55, 55, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
    i2c_device_id { name: [0; 20] },
];
/* MODULE_DEVICE_TABLE(i2c, tas2770_i2c_id); */

/* #if defined(CONFIG_OF) */
static tas2770_of_match: [of_device_id; 2] = [
    of_device_id { compatible: b"ti,tas2770\0".as_ptr() as *const c_char },
    of_device_id { compatible: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, tas2770_of_match); */
/* #endif */

static mut tas2770_i2c_driver: i2c_driver = i2c_driver {
    driver: driver_private {
        name: b"tas2770\0".as_ptr() as *const c_char,
        of_match_table: tas2770_of_match.as_ptr(),
    },
    probe: Some(tas2770_i2c_probe),
    id_table: tas2770_i2c_id.as_ptr(),
};
/* module_i2c_driver(tas2770_i2c_driver); */

/* MODULE_AUTHOR("Shi Fu <shifu0704@thundersoft.com>"); */
/* MODULE_DESCRIPTION("TAS2770 I2C Smart Amplifier driver"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
