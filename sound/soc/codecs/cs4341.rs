// SPDX-License-Identifier: GPL-2.0+
/*
 *  Cirrus Logic CS4341A ALSA SoC Codec Driver
 *  Author: Alexander Shiyan <shc_work@mail.ru>
 */

// C dependencies:
// linux/i2c.h, linux/module.h, linux/of.h, linux/regmap.h, linux/spi/spi.h
// sound/pcm.h, sound/pcm_params.h, sound/soc.h, sound/tlv.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const CS4341_REG_MODE1: c_uint = 0x00;
const CS4341_REG_MODE2: c_uint = 0x01;
const CS4341_REG_MIX: c_uint = 0x02;
const CS4341_REG_VOLA: c_uint = 0x03;
const CS4341_REG_VOLB: c_uint = 0x04;

const CS4341_MODE2_DIF: c_uint = 7 << 4;
const CS4341_MODE2_DIF_I2S_24: c_uint = 0 << 4;
const CS4341_MODE2_DIF_I2S_16: c_uint = 1 << 4;
const CS4341_MODE2_DIF_LJ_24: c_uint = 2 << 4;
const CS4341_MODE2_DIF_RJ_24: c_uint = 3 << 4;
const CS4341_MODE2_DIF_RJ_16: c_uint = 5 << 4;
const CS4341_VOLX_MUTE: c_uint = 1 << 7;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_FLAT: c_uint = 0;
const SND_SOC_NOPM: c_int = 0;

extern "C" {
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SNDRV_PCM_FORMAT_S24_LE: c_int;
    static SNDRV_PCM_FORMAT_S16_LE: c_int;
    static SNDRV_PCM_RATE_8000_96000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;

    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn of_match_ptr(ids: *const of_device_id) -> *const of_device_id;
    fn i2c_add_driver(driver: *mut i2c_driver) -> c_int;
    fn i2c_del_driver(driver: *mut i2c_driver);
    fn spi_setup(spi: *mut spi_device) -> c_int;
    fn spi_set_drvdata(spi: *mut spi_device, data: *mut c_void);
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn spi_register_driver(driver: *mut spi_driver) -> c_int;
    fn spi_unregister_driver(driver: *mut spi_driver);
}

#[repr(C)]
struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
}

#[repr(C)]
struct snd_soc_dai {
    component: *mut snd_soc_component,
}

#[repr(C)]
struct reg_default {
    reg: c_uint,
    def: c_uint,
}

#[repr(C)]
struct regmap_config {
    reg_bits: c_uint,
    val_bits: c_uint,
    write_flag_mask: c_uint,
    max_register: c_uint,
    cache_type: c_uint,
    readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    reg_defaults: *const reg_default,
    num_reg_defaults: c_uint,
}

#[repr(C)]
struct cs4341_priv {
    fmt: c_uint,
    regmap: *mut regmap,
    regcfg: regmap_config,
}

#[repr(C)]
struct soc_enum {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dai_ops {
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
    mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    no_capture_mute: c_uint,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: c_uint,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    playback: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
    symmetric_rate: c_uint,
}

#[repr(C)]
struct snd_soc_component_driver {
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    idle_bias_on: c_uint,
    use_pmdown_time: c_uint,
    endianness: c_uint,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct i2c_client {
    dev: device,
}

#[repr(C)]
struct i2c_device_id {
    name: *const c_char,
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
struct spi_device {
    dev: device,
    bits_per_word: c_uint,
    max_speed_hz: c_uint,
}

#[repr(C)]
struct spi_device_id {
    name: *const c_char,
}

#[repr(C)]
struct spi_driver {
    driver: driver_private,
    probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
    id_table: *const spi_device_id,
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

static cs4341_reg_defaults: [reg_default; 5] = [
    reg_default { reg: CS4341_REG_MODE1, def: 0x00 },
    reg_default { reg: CS4341_REG_MODE2, def: 0x82 },
    reg_default { reg: CS4341_REG_MIX, def: 0x49 },
    reg_default { reg: CS4341_REG_VOLA, def: 0x80 },
    reg_default { reg: CS4341_REG_VOLB, def: 0x80 },
];

unsafe extern "C" fn cs4341_set_fmt(dai: *mut snd_soc_dai, format: c_uint) -> c_int {
    let component = (*dai).component;
    let cs4341 = snd_soc_component_get_drvdata(component) as *mut cs4341_priv;

    match format & SND_SOC_DAIFMT_MASTER_MASK {
        x if x == SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    match format & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => {}
        _ => return -EINVAL,
    }

    match format & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S || x == SND_SOC_DAIFMT_LEFT_J || x == SND_SOC_DAIFMT_RIGHT_J => {
            (*cs4341).fmt = format & SND_SOC_DAIFMT_FORMAT_MASK;
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn cs4341_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let cs4341 = snd_soc_component_get_drvdata(component) as *mut cs4341_priv;
    let mut mode: c_uint = 0;
    let mut b24: c_int = 0;

    match params_format(params) {
        x if x == SNDRV_PCM_FORMAT_S24_LE => {
            b24 = 1;
        }
        x if x == SNDRV_PCM_FORMAT_S16_LE => {}
        _ => {
            dev_err(
                (*component).dev,
                cstr!("Unsupported PCM format 0x%08x.\n"),
                params_format(params),
            );
            return -EINVAL;
        }
    }

    match (*cs4341).fmt {
        x if x == SND_SOC_DAIFMT_I2S => {
            mode = if b24 != 0 {
                CS4341_MODE2_DIF_I2S_24
            } else {
                CS4341_MODE2_DIF_I2S_16
            };
        }
        x if x == SND_SOC_DAIFMT_LEFT_J => {
            mode = CS4341_MODE2_DIF_LJ_24;
        }
        x if x == SND_SOC_DAIFMT_RIGHT_J => {
            mode = if b24 != 0 {
                CS4341_MODE2_DIF_RJ_24
            } else {
                CS4341_MODE2_DIF_RJ_16
            };
        }
        _ => {
            dev_err(
                (*component).dev,
                cstr!("Unsupported DAI format 0x%08x.\n"),
                (*cs4341).fmt,
            );
            return -EINVAL;
        }
    }

    snd_soc_component_update_bits(component, CS4341_REG_MODE2, CS4341_MODE2_DIF, mode)
}

unsafe extern "C" fn cs4341_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let component = (*dai).component;
    let mut ret: c_int;

    ret = snd_soc_component_update_bits(
        component,
        CS4341_REG_VOLA,
        CS4341_VOLX_MUTE,
        if mute != 0 { CS4341_VOLX_MUTE } else { 0 },
    );
    if ret < 0 {
        return ret;
    }

    snd_soc_component_update_bits(
        component,
        CS4341_REG_VOLB,
        CS4341_VOLX_MUTE,
        if mute != 0 { CS4341_VOLX_MUTE } else { 0 },
    )
}

// static DECLARE_TLV_DB_SCALE(out_tlv, -9000, 100, 0);
DECLARE_TLV_DB_SCALE!(out_tlv, -9000, 100, 0);

static deemph: [*const c_char; 4] = [
    cstr!("None"),
    cstr!("44.1k"),
    cstr!("48k"),
    cstr!("32k"),
];

static deemph_enum: soc_enum = SOC_ENUM_SINGLE!(CS4341_REG_MODE2, 2, 4, deemph);

static srzc: [*const c_char; 4] = [
    cstr!("Immediate"),
    cstr!("Zero Cross"),
    cstr!("Soft Ramp"),
    cstr!("SR on ZC"),
];

static srzc_enum: soc_enum = SOC_ENUM_SINGLE!(CS4341_REG_MIX, 5, 4, srzc);

static cs4341_dapm_widgets: [snd_soc_dapm_widget; 3] = [
    SND_SOC_DAPM_DAC!(cstr!("HiFi DAC"), ptr::null(), SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_OUTPUT!(cstr!("OutA")),
    SND_SOC_DAPM_OUTPUT!(cstr!("OutB")),
];

static cs4341_routes: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route { sink: cstr!("OutA"), control: ptr::null(), source: cstr!("HiFi DAC") },
    snd_soc_dapm_route { sink: cstr!("OutB"), control: ptr::null(), source: cstr!("HiFi DAC") },
    snd_soc_dapm_route { sink: cstr!("DAC Playback"), control: ptr::null(), source: cstr!("OutA") },
    snd_soc_dapm_route { sink: cstr!("DAC Playback"), control: ptr::null(), source: cstr!("OutB") },
];

static cs4341_controls: [snd_kcontrol_new; 5] = [
    SOC_DOUBLE_R_TLV!(
        cstr!("Master Playback Volume"),
        CS4341_REG_VOLA,
        CS4341_REG_VOLB,
        0,
        90,
        1,
        out_tlv
    ),
    SOC_ENUM!(cstr!("De-Emphasis Control"), deemph_enum),
    SOC_ENUM!(cstr!("Soft Ramp Zero Cross Control"), srzc_enum),
    SOC_SINGLE!(cstr!("Auto-Mute Switch"), CS4341_REG_MODE2, 7, 1, 0),
    SOC_SINGLE!(cstr!("Popguard Transient Switch"), CS4341_REG_MODE2, 1, 1, 0),
];

static cs4341_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(cs4341_set_fmt),
    hw_params: Some(cs4341_hw_params),
    mute_stream: Some(cs4341_mute),
    no_capture_mute: 1,
};

static mut cs4341_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: cstr!("cs4341a-hifi"),
    playback: snd_soc_pcm_stream {
        stream_name: cstr!("DAC Playback"),
        channels_min: 1,
        channels_max: 2,
        rates: unsafe { SNDRV_PCM_RATE_8000_96000 },
        formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE },
    },
    ops: &cs4341_dai_ops,
    symmetric_rate: 1,
};

static soc_component_cs4341: snd_soc_component_driver = snd_soc_component_driver {
    controls: cs4341_controls.as_ptr(),
    num_controls: ARRAY_SIZE(&cs4341_controls),
    dapm_widgets: cs4341_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE(&cs4341_dapm_widgets),
    dapm_routes: cs4341_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&cs4341_routes),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

// __maybe_unused in C.
static cs4341_dt_ids: [of_device_id; 2] = [
    of_device_id { compatible: cstr!("cirrus,cs4341a") },
    of_device_id { compatible: ptr::null() },
];
// MODULE_DEVICE_TABLE(of, cs4341_dt_ids);

unsafe extern "C" fn cs4341_probe(dev: *mut device) -> c_int {
    let cs4341 = dev_get_drvdata(dev) as *mut cs4341_priv;
    let mut i: c_int = 0;

    while i < ARRAY_SIZE(&cs4341_reg_defaults) as c_int {
        regmap_write(
            (*cs4341).regmap,
            cs4341_reg_defaults[i as usize].reg,
            cs4341_reg_defaults[i as usize].def,
        );
        i += 1;
    }

    devm_snd_soc_register_component(dev, &soc_component_cs4341, &raw mut cs4341_dai, 1)
}

// C conditional: #if IS_ENABLED(CONFIG_I2C)
#[cfg(CONFIG_I2C)]
unsafe extern "C" fn cs4341_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let cs4341: *mut cs4341_priv;

    cs4341 = devm_kzalloc(
        &raw mut (*i2c).dev,
        core::mem::size_of::<cs4341_priv>(),
        GFP_KERNEL,
    ) as *mut cs4341_priv;
    if cs4341.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c, cs4341 as *mut c_void);

    (*cs4341).regcfg.reg_bits = 8;
    (*cs4341).regcfg.val_bits = 8;
    (*cs4341).regcfg.max_register = CS4341_REG_VOLB;
    (*cs4341).regcfg.cache_type = REGCACHE_FLAT;
    (*cs4341).regcfg.reg_defaults = cs4341_reg_defaults.as_ptr();
    (*cs4341).regcfg.num_reg_defaults = ARRAY_SIZE(&cs4341_reg_defaults);
    (*cs4341).regmap = devm_regmap_init_i2c(i2c, &(*cs4341).regcfg);
    if IS_ERR((*cs4341).regmap as *const c_void) {
        return PTR_ERR((*cs4341).regmap as *const c_void);
    }

    cs4341_probe(&raw mut (*i2c).dev)
}

#[cfg(CONFIG_I2C)]
static cs4341_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: cstr!("cs4341") },
    i2c_device_id { name: ptr::null() },
];
// MODULE_DEVICE_TABLE(i2c, cs4341_i2c_id);

#[cfg(CONFIG_I2C)]
static mut cs4341_i2c_driver: i2c_driver = i2c_driver {
    driver: driver_private {
        name: cstr!("cs4341-i2c"),
        of_match_table: unsafe { of_match_ptr(cs4341_dt_ids.as_ptr()) },
    },
    probe: Some(cs4341_i2c_probe),
    id_table: cs4341_i2c_id.as_ptr(),
};

// C conditional: #if defined(CONFIG_SPI_MASTER)
#[cfg(CONFIG_SPI_MASTER)]
unsafe extern "C" fn cs4341_reg_readable(_dev: *mut device, _reg: c_uint) -> bool {
    false
}

#[cfg(CONFIG_SPI_MASTER)]
unsafe extern "C" fn cs4341_spi_probe(spi: *mut spi_device) -> c_int {
    let cs4341: *mut cs4341_priv;
    let ret: c_int;

    cs4341 = devm_kzalloc(
        &raw mut (*spi).dev,
        core::mem::size_of::<cs4341_priv>(),
        GFP_KERNEL,
    ) as *mut cs4341_priv;
    if cs4341.is_null() {
        return -ENOMEM;
    }

    if (*spi).bits_per_word == 0 {
        (*spi).bits_per_word = 8;
    }
    if (*spi).max_speed_hz == 0 {
        (*spi).max_speed_hz = 6000000;
    }
    ret = spi_setup(spi);
    if ret != 0 {
        return ret;
    }

    spi_set_drvdata(spi, cs4341 as *mut c_void);

    (*cs4341).regcfg.reg_bits = 16;
    (*cs4341).regcfg.val_bits = 8;
    (*cs4341).regcfg.write_flag_mask = 0x20;
    (*cs4341).regcfg.max_register = CS4341_REG_VOLB;
    (*cs4341).regcfg.cache_type = REGCACHE_FLAT;
    (*cs4341).regcfg.readable_reg = Some(cs4341_reg_readable);
    (*cs4341).regcfg.reg_defaults = cs4341_reg_defaults.as_ptr();
    (*cs4341).regcfg.num_reg_defaults = ARRAY_SIZE(&cs4341_reg_defaults);
    (*cs4341).regmap = devm_regmap_init_spi(spi, &(*cs4341).regcfg);
    if IS_ERR((*cs4341).regmap as *const c_void) {
        return PTR_ERR((*cs4341).regmap as *const c_void);
    }

    cs4341_probe(&raw mut (*spi).dev)
}

#[cfg(CONFIG_SPI_MASTER)]
static cs4341_spi_ids: [spi_device_id; 2] = [
    spi_device_id { name: cstr!("cs4341a") },
    spi_device_id { name: ptr::null() },
];
// MODULE_DEVICE_TABLE(spi, cs4341_spi_ids);

#[cfg(CONFIG_SPI_MASTER)]
static mut cs4341_spi_driver: spi_driver = spi_driver {
    driver: driver_private {
        name: cstr!("cs4341-spi"),
        of_match_table: unsafe { of_match_ptr(cs4341_dt_ids.as_ptr()) },
    },
    probe: Some(cs4341_spi_probe),
    id_table: cs4341_spi_ids.as_ptr(),
};

unsafe extern "C" fn cs4341_init() -> c_int {
    let mut ret: c_int = 0;

    // C conditional: #if IS_ENABLED(CONFIG_I2C)
    #[cfg(CONFIG_I2C)]
    {
        ret = i2c_add_driver(&raw mut cs4341_i2c_driver);
        if ret != 0 {
            return ret;
        }
    }

    // C conditional: #if defined(CONFIG_SPI_MASTER)
    #[cfg(CONFIG_SPI_MASTER)]
    {
        ret = spi_register_driver(&raw mut cs4341_spi_driver);
    }

    ret
}
// module_init(cs4341_init);

unsafe extern "C" fn cs4341_exit() {
    // C conditional: #if IS_ENABLED(CONFIG_I2C)
    #[cfg(CONFIG_I2C)]
    {
        i2c_del_driver(&raw mut cs4341_i2c_driver);
    }

    // C conditional: #if defined(CONFIG_SPI_MASTER)
    #[cfg(CONFIG_SPI_MASTER)]
    {
        spi_unregister_driver(&raw mut cs4341_spi_driver);
    }
}
// module_exit(cs4341_exit);

// MODULE_AUTHOR("Alexander Shiyan <shc_work@mail.ru>");
// MODULE_DESCRIPTION("Cirrus Logic CS4341 ALSA SoC Codec Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
