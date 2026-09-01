// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8741.c  --  WM8741 ALSA SoC Audio driver
 *
 * Copyright 2010-1 Wolfson Microelectronics plc
 *
 * Author: Ian Lartey <ian@opensource.wolfsonmicro.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = c_uint;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;
const SND_SOC_NOPM: c_int = 0;
const KERN_ERR: *const c_char = b"\0".as_ptr() as *const c_char;

const WM8741_NUM_SUPPLIES: usize = 2;

static wm8741_supply_names: [*const c_char; WM8741_NUM_SUPPLIES] = [
    b"AVDD\0".as_ptr() as *const c_char,
    b"DVDD\0".as_ptr() as *const c_char,
];

#[repr(C)]
pub struct wm8741_platform_data {
    pub diff_mode: c_uint,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *const c_uint,
}

/* codec private data */
#[repr(C)]
pub struct wm8741_priv {
    pub pdata: wm8741_platform_data,
    pub regmap: *mut regmap,
    pub supplies: [regulator_bulk_data; WM8741_NUM_SUPPLIES],
    pub sysclk: c_uint,
    pub sysclk_constraints: *const snd_pcm_hw_constraint_list,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
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
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
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
    pub formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
}

#[repr(C)]
pub struct driver_private {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: driver_private,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

#[repr(C)]
pub struct spi_device {
    pub dev: device,
}

#[repr(C)]
pub struct spi_driver {
    pub driver: driver_private,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
}

unsafe extern "C" {
    static WM8741_RESET: c_uint;
    static WM8741_DACLLSB_ATTENUATION: c_uint;
    static WM8741_DACLMSB_ATTENUATION: c_uint;
    static WM8741_DACRLSB_ATTENUATION: c_uint;
    static WM8741_DACRMSB_ATTENUATION: c_uint;
    static WM8741_VOLUME_CONTROL: c_uint;
    static WM8741_FORMAT_CONTROL: c_uint;
    static WM8741_MODE_CONTROL_1: c_uint;
    static WM8741_MODE_CONTROL_2: c_uint;
    static WM8741_MAX_REGISTER: c_uint;
    static WM8741_IWL_MASK: c_uint;
    static WM8741_OSR_MASK: c_uint;
    static WM8741_BCP_MASK: c_uint;
    static WM8741_LRP_MASK: c_uint;
    static WM8741_FMT_MASK: c_uint;
    static WM8741_SOFT_MASK: c_uint;
    static WM8741_SOFT_SHIFT: c_uint;
    static WM8741_DIFF_MASK: c_uint;
    static WM8741_DIFF_SHIFT: c_uint;
    static WM8741_DIFF_MODE_STEREO: c_uint;
    static WM8741_DIFF_MODE_STEREO_REVERSED: c_uint;
    static WM8741_DIFF_MODE_MONO_LEFT: c_uint;
    static WM8741_DIFF_MODE_MONO_RIGHT: c_uint;
    static WM8741_UPDATELL: c_uint;
    static WM8741_UPDATELM: c_uint;
    static WM8741_UPDATERL: c_uint;
    static WM8741_UPDATERM: c_uint;
    static SNDRV_PCM_HW_PARAM_RATE: c_uint;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SNDRV_PCM_RATE_32000: c_uint;
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_RATE_88200: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_176400: c_uint;
    static SNDRV_PCM_RATE_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;

    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_pcm_hw_constraint_list(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_uint,
        l: *const snd_pcm_hw_constraint_list,
    ) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_cache_sync(component: *mut snd_soc_component) -> c_int;
    fn snd_soc_add_component_controls(
        component: *mut snd_soc_component,
        controls: *const snd_kcontrol_new,
        num_controls: c_uint,
    ) -> c_int;
    fn regulator_bulk_enable(num_consumers: c_uint, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_uint, consumers: *mut regulator_bulk_data) -> c_int;
    fn dev_get_platdata(dev: *mut device) -> *const wm8741_platform_data;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut u32) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regulator_bulk_get(
        dev: *mut device,
        num_consumers: c_uint,
        consumers: *mut regulator_bulk_data,
    ) -> c_int;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn spi_set_drvdata(spi: *mut spi_device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn i2c_add_driver(driver: *mut i2c_driver) -> c_int;
    fn i2c_del_driver(driver: *mut i2c_driver);
    fn spi_register_driver(driver: *mut spi_driver) -> c_int;
    fn spi_unregister_driver(driver: *mut spi_driver);
    fn pr_err(fmt: *const c_char, ...) -> c_int;
    fn printk(fmt: *const c_char, ...) -> c_int;
}

static wm8741_reg_defaults: [reg_default; 10] = [
    reg_default { reg: 0, def: 0x0000 },  /* R0  - DACLLSB Attenuation */
    reg_default { reg: 1, def: 0x0000 },  /* R1  - DACLMSB Attenuation */
    reg_default { reg: 2, def: 0x0000 },  /* R2  - DACRLSB Attenuation */
    reg_default { reg: 3, def: 0x0000 },  /* R3  - DACRMSB Attenuation */
    reg_default { reg: 4, def: 0x0000 },  /* R4  - Volume Control */
    reg_default { reg: 5, def: 0x000A },  /* R5  - Format Control */
    reg_default { reg: 6, def: 0x0000 },  /* R6  - Filter Control */
    reg_default { reg: 7, def: 0x0000 },  /* R7  - Mode Control 1 */
    reg_default { reg: 8, def: 0x0002 },  /* R8  - Mode Control 2 */
    reg_default { reg: 32, def: 0x0002 }, /* R32 - ADDITONAL_CONTROL_1 */
];

unsafe extern "C" fn wm8741_reset(component: *mut snd_soc_component) -> c_int {
    snd_soc_component_write(component, WM8741_RESET, 0)
}

static dac_tlv_fine: [c_uint; 4] = [0, 0, (-12700i32) as c_uint, 13];
static dac_tlv: [c_uint; 4] = [0, 0, (-12700i32) as c_uint, 400];

/* SOC_* and SND_SOC_DAPM_* macro initializers are supplied by ALSA headers. */
static wm8741_snd_controls_stereo: [snd_kcontrol_new; 2] = unsafe { core::mem::zeroed() };
static wm8741_snd_controls_mono_left: [snd_kcontrol_new; 2] = unsafe { core::mem::zeroed() };
static wm8741_snd_controls_mono_right: [snd_kcontrol_new; 2] = unsafe { core::mem::zeroed() };
static wm8741_dapm_widgets: [snd_soc_dapm_widget; 6] = unsafe { core::mem::zeroed() };

static wm8741_dapm_routes: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route { sink: b"VOUTLP\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DACL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"VOUTLN\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DACL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"VOUTRP\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DACR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"VOUTRN\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DACR\0".as_ptr() as *const c_char },
];

static rates_11289: [c_uint; 2] = [44100, 88200];
static constraints_11289: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: rates_11289.len() as c_uint,
    list: rates_11289.as_ptr(),
};

static rates_12288: [c_uint; 3] = [32000, 48000, 96000];
static constraints_12288: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: rates_12288.len() as c_uint,
    list: rates_12288.as_ptr(),
};

static rates_16384: [c_uint; 1] = [32000];
static constraints_16384: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: rates_16384.len() as c_uint,
    list: rates_16384.as_ptr(),
};

static rates_16934: [c_uint; 2] = [44100, 88200];
static constraints_16934: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: rates_16934.len() as c_uint,
    list: rates_16934.as_ptr(),
};

static rates_18432: [c_uint; 2] = [48000, 96000];
static constraints_18432: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: rates_18432.len() as c_uint,
    list: rates_18432.as_ptr(),
};

static rates_22579: [c_uint; 3] = [44100, 88200, 176400];
static constraints_22579: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: rates_22579.len() as c_uint,
    list: rates_22579.as_ptr(),
};

static rates_24576: [c_uint; 4] = [32000, 48000, 96000, 192000];
static constraints_24576: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: rates_24576.len() as c_uint,
    list: rates_24576.as_ptr(),
};

static rates_36864: [c_uint; 3] = [48000, 96000, 192000];
static constraints_36864: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: rates_36864.len() as c_uint,
    list: rates_36864.as_ptr(),
};

unsafe extern "C" fn wm8741_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let wm8741 = snd_soc_component_get_drvdata(component) as *mut wm8741_priv;

    if (*wm8741).sysclk != 0 {
        snd_pcm_hw_constraint_list(
            (*substream).runtime,
            0,
            SNDRV_PCM_HW_PARAM_RATE,
            (*wm8741).sysclk_constraints,
        );
    }

    0
}

unsafe extern "C" fn wm8741_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let wm8741 = snd_soc_component_get_drvdata(component) as *mut wm8741_priv;
    let iface: c_uint;
    let mode: c_uint;
    let mut i: c_int;

    /* The set of sample rates that can be supported depends on the
     * MCLK supplied to the CODEC - enforce this.
     */
    if (*wm8741).sysclk == 0 {
        dev_err(
            (*component).dev,
            b"No MCLK configured, call set_sysclk() on init or in hw_params\n\0".as_ptr() as *const c_char,
        );
        return -EINVAL;
    }

    /* Find a supported LRCLK rate */
    i = 0;
    while i < (*(*wm8741).sysclk_constraints).count as c_int {
        if *(*(*wm8741).sysclk_constraints).list.add(i as usize) == params_rate(params) {
            break;
        }
        i += 1;
    }

    if i == (*(*wm8741).sysclk_constraints).count as c_int {
        dev_err(
            (*component).dev,
            b"LRCLK %d unsupported with MCLK %d\n\0".as_ptr() as *const c_char,
            params_rate(params),
            (*wm8741).sysclk,
        );
        return -EINVAL;
    }

    /* bit size */
    match params_width(params) {
        16 => iface = 0x0,
        20 => iface = 0x1,
        24 => iface = 0x2,
        32 => iface = 0x3,
        _ => {
            dev_dbg(
                (*component).dev,
                b"wm8741_hw_params:    Unsupported bit size param = %d\0".as_ptr() as *const c_char,
                params_width(params),
            );
            return -EINVAL;
        }
    }

    /* oversampling rate */
    if params_rate(params) > 96000 {
        mode = 0x40;
    } else if params_rate(params) > 48000 {
        mode = 0x20;
    } else {
        mode = 0x00;
    }

    dev_dbg(
        (*component).dev,
        b"wm8741_hw_params:    bit size param = %d, rate param = %d\0".as_ptr() as *const c_char,
        params_width(params),
        params_rate(params),
    );

    snd_soc_component_update_bits(component, WM8741_FORMAT_CONTROL, WM8741_IWL_MASK, iface);
    snd_soc_component_update_bits(component, WM8741_MODE_CONTROL_1, WM8741_OSR_MASK, mode);

    0
}

unsafe extern "C" fn wm8741_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let wm8741 = snd_soc_component_get_drvdata(component) as *mut wm8741_priv;

    dev_dbg(
        (*component).dev,
        b"wm8741_set_dai_sysclk info: freq=%dHz\n\0".as_ptr() as *const c_char,
        freq,
    );

    match freq {
        0 => (*wm8741).sysclk_constraints = ptr::null(),
        11289600 => (*wm8741).sysclk_constraints = &constraints_11289,
        12288000 => (*wm8741).sysclk_constraints = &constraints_12288,
        16384000 => (*wm8741).sysclk_constraints = &constraints_16384,
        16934400 => (*wm8741).sysclk_constraints = &constraints_16934,
        18432000 => (*wm8741).sysclk_constraints = &constraints_18432,
        22579200 | 33868800 => (*wm8741).sysclk_constraints = &constraints_22579,
        24576000 => (*wm8741).sysclk_constraints = &constraints_24576,
        36864000 => (*wm8741).sysclk_constraints = &constraints_36864,
        _ => return -EINVAL,
    }

    (*wm8741).sysclk = freq;
    0
}

unsafe extern "C" fn wm8741_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let mut iface: c_uint;

    /* check master/slave audio interface */
    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        x if x == SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => iface = 0x08,
        x if x == SND_SOC_DAIFMT_RIGHT_J => iface = 0x00,
        x if x == SND_SOC_DAIFMT_LEFT_J => iface = 0x04,
        x if x == SND_SOC_DAIFMT_DSP_A => iface = 0x0C,
        x if x == SND_SOC_DAIFMT_DSP_B => iface = 0x1C,
        _ => return -EINVAL,
    }

    /* clock inversion */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => {}
        x if x == SND_SOC_DAIFMT_NB_IF => iface |= 0x10,
        x if x == SND_SOC_DAIFMT_IB_NF => iface |= 0x20,
        x if x == SND_SOC_DAIFMT_IB_IF => iface |= 0x30,
        _ => return -EINVAL,
    }

    dev_dbg(
        (*component).dev,
        b"wm8741_set_dai_fmt:    Format=%x, Clock Inv=%x\n\0".as_ptr() as *const c_char,
        fmt & SND_SOC_DAIFMT_FORMAT_MASK,
        fmt & SND_SOC_DAIFMT_INV_MASK,
    );

    snd_soc_component_update_bits(
        component,
        WM8741_FORMAT_CONTROL,
        WM8741_BCP_MASK | WM8741_LRP_MASK | WM8741_FMT_MASK,
        iface,
    );

    0
}

unsafe extern "C" fn wm8741_mute(
    codec_dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let component = (*codec_dai).component;

    snd_soc_component_update_bits(
        component,
        WM8741_VOLUME_CONTROL,
        WM8741_SOFT_MASK,
        ((mute != 0) as c_uint) << WM8741_SOFT_SHIFT,
    );
    0
}

unsafe fn WM8741_RATES() -> c_uint {
    SNDRV_PCM_RATE_32000
        | SNDRV_PCM_RATE_44100
        | SNDRV_PCM_RATE_48000
        | SNDRV_PCM_RATE_88200
        | SNDRV_PCM_RATE_96000
        | SNDRV_PCM_RATE_176400
        | SNDRV_PCM_RATE_192000
}

unsafe fn WM8741_FORMATS() -> c_uint {
    SNDRV_PCM_FMTBIT_S16_LE
        | SNDRV_PCM_FMTBIT_S20_3LE
        | SNDRV_PCM_FMTBIT_S24_LE
        | SNDRV_PCM_FMTBIT_S32_LE
}

static wm8741_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(wm8741_startup),
    hw_params: Some(wm8741_hw_params),
    set_sysclk: Some(wm8741_set_dai_sysclk),
    set_fmt: Some(wm8741_set_dai_fmt),
    mute_stream: Some(wm8741_mute),
    no_capture_mute: 1,
};

static mut wm8741_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"wm8741\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: 0,   /* WM8741_RATES */
        formats: 0, /* WM8741_FORMATS */
    },
    ops: &wm8741_dai_ops,
};

/* CONFIG_PM */
unsafe extern "C" fn wm8741_resume(component: *mut snd_soc_component) -> c_int {
    snd_soc_component_cache_sync(component);
    0
}
/* Without CONFIG_PM, wm8741_resume is NULL. */

unsafe extern "C" fn wm8741_configure(component: *mut snd_soc_component) -> c_int {
    let wm8741 = snd_soc_component_get_drvdata(component) as *mut wm8741_priv;

    /* Configure differential mode */
    match (*wm8741).pdata.diff_mode {
        x if x == WM8741_DIFF_MODE_STEREO
            || x == WM8741_DIFF_MODE_STEREO_REVERSED
            || x == WM8741_DIFF_MODE_MONO_LEFT
            || x == WM8741_DIFF_MODE_MONO_RIGHT =>
        {
            snd_soc_component_update_bits(
                component,
                WM8741_MODE_CONTROL_2,
                WM8741_DIFF_MASK,
                (*wm8741).pdata.diff_mode << WM8741_DIFF_SHIFT,
            );
        }
        _ => return -EINVAL,
    }

    /* Change some default settings - latch VU */
    snd_soc_component_update_bits(
        component,
        WM8741_DACLLSB_ATTENUATION,
        WM8741_UPDATELL,
        WM8741_UPDATELL,
    );
    snd_soc_component_update_bits(
        component,
        WM8741_DACLMSB_ATTENUATION,
        WM8741_UPDATELM,
        WM8741_UPDATELM,
    );
    snd_soc_component_update_bits(
        component,
        WM8741_DACRLSB_ATTENUATION,
        WM8741_UPDATERL,
        WM8741_UPDATERL,
    );
    snd_soc_component_update_bits(
        component,
        WM8741_DACRMSB_ATTENUATION,
        WM8741_UPDATERM,
        WM8741_UPDATERM,
    );

    0
}

unsafe extern "C" fn wm8741_add_controls(component: *mut snd_soc_component) -> c_int {
    let wm8741 = snd_soc_component_get_drvdata(component) as *mut wm8741_priv;

    match (*wm8741).pdata.diff_mode {
        x if x == WM8741_DIFF_MODE_STEREO || x == WM8741_DIFF_MODE_STEREO_REVERSED => {
            snd_soc_add_component_controls(
                component,
                wm8741_snd_controls_stereo.as_ptr(),
                wm8741_snd_controls_stereo.len() as c_uint,
            );
        }
        x if x == WM8741_DIFF_MODE_MONO_LEFT => {
            snd_soc_add_component_controls(
                component,
                wm8741_snd_controls_mono_left.as_ptr(),
                wm8741_snd_controls_mono_left.len() as c_uint,
            );
        }
        x if x == WM8741_DIFF_MODE_MONO_RIGHT => {
            snd_soc_add_component_controls(
                component,
                wm8741_snd_controls_mono_right.as_ptr(),
                wm8741_snd_controls_mono_right.len() as c_uint,
            );
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn wm8741_probe(component: *mut snd_soc_component) -> c_int {
    let wm8741 = snd_soc_component_get_drvdata(component) as *mut wm8741_priv;
    let mut ret: c_int = 0;

    ret = regulator_bulk_enable(WM8741_NUM_SUPPLIES as c_uint, (*wm8741).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err((*component).dev, b"Failed to enable supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = wm8741_reset(component);
    if ret < 0 {
        dev_err((*component).dev, b"Failed to issue reset\n\0".as_ptr() as *const c_char);
        regulator_bulk_disable(WM8741_NUM_SUPPLIES as c_uint, (*wm8741).supplies.as_mut_ptr());
        return ret;
    }

    ret = wm8741_configure(component);
    if ret < 0 {
        dev_err((*component).dev, b"Failed to change default settings\n\0".as_ptr() as *const c_char);
        regulator_bulk_disable(WM8741_NUM_SUPPLIES as c_uint, (*wm8741).supplies.as_mut_ptr());
        return ret;
    }

    ret = wm8741_add_controls(component);
    if ret < 0 {
        dev_err((*component).dev, b"Failed to add controls\n\0".as_ptr() as *const c_char);
        regulator_bulk_disable(WM8741_NUM_SUPPLIES as c_uint, (*wm8741).supplies.as_mut_ptr());
        return ret;
    }

    dev_dbg((*component).dev, b"Successful registration\n\0".as_ptr() as *const c_char);
    ret
}

unsafe extern "C" fn wm8741_remove(component: *mut snd_soc_component) {
    let wm8741 = snd_soc_component_get_drvdata(component) as *mut wm8741_priv;

    regulator_bulk_disable(WM8741_NUM_SUPPLIES as c_uint, (*wm8741).supplies.as_mut_ptr());
}

static soc_component_dev_wm8741: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm8741_probe),
    remove: Some(wm8741_remove),
    resume: Some(wm8741_resume),
    dapm_widgets: wm8741_dapm_widgets.as_ptr(),
    num_dapm_widgets: wm8741_dapm_widgets.len() as c_uint,
    dapm_routes: wm8741_dapm_routes.as_ptr(),
    num_dapm_routes: wm8741_dapm_routes.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static wm8741_of_match: [of_device_id; 2] = [
    of_device_id { compatible: b"wlf,wm8741\0".as_ptr() as *const c_char },
    of_device_id { compatible: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, wm8741_of_match); */

static wm8741_regmap: regmap_config = regmap_config {
    reg_bits: 7,
    val_bits: 9,
    max_register: 0, /* WM8741_MAX_REGISTER */
    reg_defaults: wm8741_reg_defaults.as_ptr(),
    num_reg_defaults: wm8741_reg_defaults.len() as c_uint,
    cache_type: REGCACHE_MAPLE,
};

unsafe extern "C" fn wm8741_set_pdata(dev: *mut device, wm8741: *mut wm8741_priv) -> c_int {
    let pdata = dev_get_platdata(dev);
    let mut diff_mode: u32 = 0;

    if !(*dev).of_node.is_null() {
        if of_property_read_u32((*dev).of_node, b"diff-mode\0".as_ptr() as *const c_char, &mut diff_mode) >= 0 {
            (*wm8741).pdata.diff_mode = diff_mode;
        }
    } else if !pdata.is_null() {
        memcpy(
            &mut (*wm8741).pdata as *mut wm8741_platform_data as *mut c_void,
            pdata as *const c_void,
            size_of::<wm8741_platform_data>(),
        );
    }

    0
}

/* #if IS_ENABLED(CONFIG_I2C) */
unsafe extern "C" fn wm8741_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let wm8741: *mut wm8741_priv;
    let mut ret: c_int;
    let mut i: c_int;

    wm8741 = devm_kzalloc(&mut (*i2c).dev, size_of::<wm8741_priv>(), GFP_KERNEL) as *mut wm8741_priv;
    if wm8741.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < WM8741_NUM_SUPPLIES as c_int {
        (*wm8741).supplies[i as usize].supply = wm8741_supply_names[i as usize];
        i += 1;
    }

    ret = devm_regulator_bulk_get(&mut (*i2c).dev, WM8741_NUM_SUPPLIES as c_uint, (*wm8741).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(&mut (*i2c).dev, b"Failed to request supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    (*wm8741).regmap = devm_regmap_init_i2c(i2c, &wm8741_regmap);
    if IS_ERR((*wm8741).regmap as *const c_void) {
        ret = PTR_ERR((*wm8741).regmap as *const c_void);
        dev_err(&mut (*i2c).dev, b"Failed to init regmap: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = wm8741_set_pdata(&mut (*i2c).dev, wm8741);
    if ret != 0 {
        dev_err(&mut (*i2c).dev, b"Failed to set pdata: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    i2c_set_clientdata(i2c, wm8741 as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_component_dev_wm8741,
        &mut wm8741_dai,
        1,
    );

    ret
}

static wm8741_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: b"wm8741\0".as_ptr() as *const c_char },
    i2c_device_id { name: ptr::null() },
];
/* MODULE_DEVICE_TABLE(i2c, wm8741_i2c_id); */

static mut wm8741_i2c_driver: i2c_driver = i2c_driver {
    driver: driver_private {
        name: b"wm8741\0".as_ptr() as *const c_char,
        of_match_table: wm8741_of_match.as_ptr(),
    },
    probe: Some(wm8741_i2c_probe),
    id_table: wm8741_i2c_id.as_ptr(),
};
/* #endif */

/* #if defined(CONFIG_SPI_MASTER) */
unsafe extern "C" fn wm8741_spi_probe(spi: *mut spi_device) -> c_int {
    let wm8741: *mut wm8741_priv;
    let mut ret: c_int;
    let mut i: c_int;

    wm8741 = devm_kzalloc(&mut (*spi).dev, size_of::<wm8741_priv>(), GFP_KERNEL) as *mut wm8741_priv;
    if wm8741.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < WM8741_NUM_SUPPLIES as c_int {
        (*wm8741).supplies[i as usize].supply = wm8741_supply_names[i as usize];
        i += 1;
    }

    ret = devm_regulator_bulk_get(&mut (*spi).dev, WM8741_NUM_SUPPLIES as c_uint, (*wm8741).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(&mut (*spi).dev, b"Failed to request supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    (*wm8741).regmap = devm_regmap_init_spi(spi, &wm8741_regmap);
    if IS_ERR((*wm8741).regmap as *const c_void) {
        ret = PTR_ERR((*wm8741).regmap as *const c_void);
        dev_err(&mut (*spi).dev, b"Failed to init regmap: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = wm8741_set_pdata(&mut (*spi).dev, wm8741);
    if ret != 0 {
        dev_err(&mut (*spi).dev, b"Failed to set pdata: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    spi_set_drvdata(spi, wm8741 as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*spi).dev,
        &soc_component_dev_wm8741,
        &mut wm8741_dai,
        1,
    );
    ret
}

static mut wm8741_spi_driver: spi_driver = spi_driver {
    driver: driver_private {
        name: b"wm8741\0".as_ptr() as *const c_char,
        of_match_table: wm8741_of_match.as_ptr(),
    },
    probe: Some(wm8741_spi_probe),
};
/* #endif CONFIG_SPI_MASTER */

unsafe extern "C" fn wm8741_modinit() -> c_int {
    let mut ret: c_int = 0;

    /* #if IS_ENABLED(CONFIG_I2C) */
    ret = i2c_add_driver(&mut wm8741_i2c_driver);
    if ret != 0 {
        pr_err(b"Failed to register WM8741 I2C driver: %d\n\0".as_ptr() as *const c_char, ret);
    }
    /* #endif */
    /* #if defined(CONFIG_SPI_MASTER) */
    ret = spi_register_driver(&mut wm8741_spi_driver);
    if ret != 0 {
        printk(
            b"%sFailed to register wm8741 SPI driver: %d\n\0".as_ptr() as *const c_char,
            KERN_ERR,
            ret,
        );
    }
    /* #endif */

    ret
}
/* module_init(wm8741_modinit); */

unsafe extern "C" fn wm8741_exit() {
    /* #if defined(CONFIG_SPI_MASTER) */
    spi_unregister_driver(&mut wm8741_spi_driver);
    /* #endif */
    /* #if IS_ENABLED(CONFIG_I2C) */
    i2c_del_driver(&mut wm8741_i2c_driver);
    /* #endif */
}
/* module_exit(wm8741_exit); */

/* MODULE_DESCRIPTION("ASoC WM8741 driver"); */
/* MODULE_AUTHOR("Ian Lartey <ian@opensource.wolfsonmicro.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
