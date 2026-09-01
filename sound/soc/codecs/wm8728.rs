// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8728.rs  --  WM8728 ALSA SoC Audio driver
 *
 * Copyright 2008 Wolfson Microelectronics plc
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

/* Translated from the implementation source. C include dependencies:
 * linux/module.h, linux/moduleparam.h, linux/init.h, linux/delay.h,
 * linux/pm.h, linux/i2c.h, linux/platform_device.h, linux/regmap.h,
 * linux/spi/spi.h, linux/slab.h, sound/core.h, sound/pcm.h,
 * sound/pcm_params.h, sound/soc.h, sound/initval.h, sound/tlv.h,
 * and "wm8728.h".
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

type u16 = core::ffi::c_ushort;

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
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
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
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
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub set_bias_level:
        Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub suspend_bias_off: c_uint,
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
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_device {
    pub dev: device,
}

#[repr(C)]
pub struct spi_driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct spi_driver {
    pub driver: spi_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
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
pub struct i2c_driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: i2c_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON,
    SND_SOC_BIAS_PREPARE,
    SND_SOC_BIAS_STANDBY,
    SND_SOC_BIAS_OFF,
}

extern "C" {
    static WM8728_DACLVOL: c_uint;
    static WM8728_DACRVOL: c_uint;
    static WM8728_DACCTL: c_uint;
    static WM8728_IFCTL: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SNDRV_PCM_RATE_8000_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S20_3LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static REGCACHE_MAPLE: c_uint;
    static GFP_KERNEL: c_uint;
    static KERN_ERR: *const c_char;

    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint);
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn spi_set_drvdata(spi: *mut spi_device, data: *mut c_void);
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
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
    fn printk(fmt: *const c_char, ...) -> c_int;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const WM8728_RATES: c_uint = unsafe { SNDRV_PCM_RATE_8000_192000 };
const WM8728_FORMATS: u64 =
    unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE };

/*
 * We can't read the WM8728 register space so we cache them instead.
 * Note that the defaults here aren't the physical defaults, we latch
 * the volume update bits, mute the output and enable infinite zero
 * detect.
 */
static wm8728_reg_defaults: [reg_default; 4] = [
    reg_default { reg: 0, def: 0x1ff },
    reg_default { reg: 1, def: 0x1ff },
    reg_default { reg: 2, def: 0x001 },
    reg_default { reg: 3, def: 0x100 },
];

/* codec private data */
#[repr(C)]
pub struct wm8728_priv {
    pub regmap: *mut regmap,
}

/* static const DECLARE_TLV_DB_SCALE(wm8728_tlv, -12750, 50, 1); */
static wm8728_tlv: [c_uint; 0] = [];

static wm8728_snd_controls: [snd_kcontrol_new; 2] = [
    /* SOC_DOUBLE_R_TLV("Digital Playback Volume", WM8728_DACLVOL,
     *                  WM8728_DACRVOL, 0, 255, 0, wm8728_tlv)
     */
    snd_kcontrol_new { _private: [] },
    /* SOC_SINGLE("Deemphasis", WM8728_DACCTL, 1, 1, 0) */
    snd_kcontrol_new { _private: [] },
];

/*
 * DAPM controls.
 */
static wm8728_dapm_widgets: [snd_soc_dapm_widget; 3] = [
    /* SND_SOC_DAPM_DAC("DAC", "HiFi Playback", SND_SOC_NOPM, 0, 0) */
    snd_soc_dapm_widget { _private: [] },
    /* SND_SOC_DAPM_OUTPUT("VOUTL") */
    snd_soc_dapm_widget { _private: [] },
    /* SND_SOC_DAPM_OUTPUT("VOUTR") */
    snd_soc_dapm_widget { _private: [] },
];

static wm8728_intercon: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"VOUTL\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"DAC\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"VOUTR\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"DAC\0".as_ptr() as *const c_char,
    },
];

unsafe extern "C" fn wm8728_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    direction: c_int,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let mute_reg: u16 = snd_soc_component_read(component, WM8728_DACCTL) as u16;

    if mute != 0 {
        snd_soc_component_write(component, WM8728_DACCTL, (mute_reg as c_uint) | 1);
    } else {
        snd_soc_component_write(component, WM8728_DACCTL, (mute_reg as c_uint) & !1);
    }

    0
}

unsafe extern "C" fn wm8728_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let mut dac: u16 = snd_soc_component_read(component, WM8728_DACCTL) as u16;

    dac &= !0x18;

    match params_width(params) {
        16 => {}
        20 => {
            dac |= 0x10;
        }
        24 => {
            dac |= 0x08;
        }
        _ => {
            return -EINVAL;
        }
    }

    snd_soc_component_write(component, WM8728_DACCTL, dac as c_uint);

    0
}

unsafe extern "C" fn wm8728_set_dai_fmt(
    codec_dai: *mut snd_soc_dai,
    fmt: c_uint,
) -> c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let mut iface: u16 = snd_soc_component_read(component, WM8728_IFCTL) as u16;

    /* Currently only I2S is supported by the driver, though the
     * hardware is more flexible.
     */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => {
            iface |= 1;
        }
        _ => {
            return -EINVAL;
        }
    }

    /* The hardware only support full slave mode */
    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        x if x == SND_SOC_DAIFMT_CBC_CFC => {}
        _ => {
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => {
            iface &= !0x22;
        }
        x if x == SND_SOC_DAIFMT_IB_NF => {
            iface |= 0x20;
            iface &= !0x02;
        }
        x if x == SND_SOC_DAIFMT_NB_IF => {
            iface |= 0x02;
            iface &= !0x20;
        }
        x if x == SND_SOC_DAIFMT_IB_IF => {
            iface |= 0x22;
        }
        _ => {
            return -EINVAL;
        }
    }

    snd_soc_component_write(component, WM8728_IFCTL, iface as c_uint);
    0
}

unsafe extern "C" fn wm8728_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let wm8728: *mut wm8728_priv =
        snd_soc_component_get_drvdata(component) as *mut wm8728_priv;
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);
    let mut reg: u16;

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON
        | snd_soc_bias_level::SND_SOC_BIAS_PREPARE
        | snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == snd_soc_bias_level::SND_SOC_BIAS_OFF {
                /* Power everything up... */
                reg = snd_soc_component_read(component, WM8728_DACCTL) as u16;
                snd_soc_component_write(component, WM8728_DACCTL, (reg as c_uint) & !0x4);

                /* ..then sync in the register cache. */
                regcache_sync((*wm8728).regmap);
            }
        }

        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            reg = snd_soc_component_read(component, WM8728_DACCTL) as u16;
            snd_soc_component_write(component, WM8728_DACCTL, (reg as c_uint) | 0x4);
        }
    }
    0
}

static wm8728_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(wm8728_hw_params),
    mute_stream: Some(wm8728_mute),
    set_fmt: Some(wm8728_set_dai_fmt),
    no_capture_mute: 1,
};

static mut wm8728_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"wm8728-hifi\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: WM8728_RATES,
        formats: WM8728_FORMATS,
    },
    ops: &wm8728_dai_ops,
};

static soc_component_dev_wm8728: snd_soc_component_driver = snd_soc_component_driver {
    set_bias_level: Some(wm8728_set_bias_level),
    controls: wm8728_snd_controls.as_ptr(),
    num_controls: wm8728_snd_controls.len() as c_uint,
    dapm_widgets: wm8728_dapm_widgets.as_ptr(),
    num_dapm_widgets: wm8728_dapm_widgets.len() as c_uint,
    dapm_routes: wm8728_intercon.as_ptr(),
    num_dapm_routes: wm8728_intercon.len() as c_uint,
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static wm8728_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"wlf,wm8728\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, wm8728_of_match); */

static wm8728_regmap: regmap_config = regmap_config {
    reg_bits: 7,
    val_bits: 9,
    max_register: unsafe { WM8728_IFCTL },

    reg_defaults: wm8728_reg_defaults.as_ptr(),
    num_reg_defaults: wm8728_reg_defaults.len() as c_uint,
    cache_type: unsafe { REGCACHE_MAPLE },
};

/* #if defined(CONFIG_SPI_MASTER) */
unsafe extern "C" fn wm8728_spi_probe(spi: *mut spi_device) -> c_int {
    let wm8728: *mut wm8728_priv;
    let ret: c_int;

    wm8728 = devm_kzalloc(
        &mut (*spi).dev,
        core::mem::size_of::<wm8728_priv>(),
        GFP_KERNEL,
    ) as *mut wm8728_priv;
    if wm8728.is_null() {
        return -ENOMEM;
    }

    (*wm8728).regmap = devm_regmap_init_spi(spi, &wm8728_regmap);
    if IS_ERR((*wm8728).regmap as *const c_void) {
        return PTR_ERR((*wm8728).regmap as *const c_void);
    }

    spi_set_drvdata(spi, wm8728 as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*spi).dev,
        &soc_component_dev_wm8728,
        &mut wm8728_dai,
        1,
    );

    ret
}

static mut wm8728_spi_driver: spi_driver = spi_driver {
    driver: spi_driver_inner {
        name: b"wm8728\0".as_ptr() as *const c_char,
        of_match_table: wm8728_of_match.as_ptr(),
    },
    probe: Some(wm8728_spi_probe),
};
/* #endif CONFIG_SPI_MASTER */

/* #if IS_ENABLED(CONFIG_I2C) */
unsafe extern "C" fn wm8728_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let wm8728: *mut wm8728_priv;
    let ret: c_int;

    wm8728 = devm_kzalloc(
        &mut (*i2c).dev,
        core::mem::size_of::<wm8728_priv>(),
        GFP_KERNEL,
    ) as *mut wm8728_priv;
    if wm8728.is_null() {
        return -ENOMEM;
    }

    (*wm8728).regmap = devm_regmap_init_i2c(i2c, &wm8728_regmap);
    if IS_ERR((*wm8728).regmap as *const c_void) {
        return PTR_ERR((*wm8728).regmap as *const c_void);
    }

    i2c_set_clientdata(i2c, wm8728 as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_component_dev_wm8728,
        &mut wm8728_dai,
        1,
    );

    ret
}

static wm8728_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: b"wm8728\0".as_ptr() as *const c_char,
    },
    i2c_device_id {
        name: core::ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(i2c, wm8728_i2c_id); */

static mut wm8728_i2c_driver: i2c_driver = i2c_driver {
    driver: i2c_driver_inner {
        name: b"wm8728\0".as_ptr() as *const c_char,
        of_match_table: wm8728_of_match.as_ptr(),
    },
    probe: Some(wm8728_i2c_probe),
    id_table: wm8728_i2c_id.as_ptr(),
};
/* #endif */

unsafe extern "C" fn wm8728_modinit() -> c_int {
    let mut ret: c_int = 0;
    /* #if IS_ENABLED(CONFIG_I2C) */
    ret = i2c_add_driver(&mut wm8728_i2c_driver);
    if ret != 0 {
        printk(
            b"%sFailed to register wm8728 I2C driver: %d\n\0".as_ptr() as *const c_char,
            KERN_ERR,
            ret,
        );
    }
    /* #endif */
    /* #if defined(CONFIG_SPI_MASTER) */
    ret = spi_register_driver(&mut wm8728_spi_driver);
    if ret != 0 {
        printk(
            b"%sFailed to register wm8728 SPI driver: %d\n\0".as_ptr() as *const c_char,
            KERN_ERR,
            ret,
        );
    }
    /* #endif */
    ret
}
/* module_init(wm8728_modinit); */

unsafe extern "C" fn wm8728_exit() {
    /* #if IS_ENABLED(CONFIG_I2C) */
    i2c_del_driver(&mut wm8728_i2c_driver);
    /* #endif */
    /* #if defined(CONFIG_SPI_MASTER) */
    spi_unregister_driver(&mut wm8728_spi_driver);
    /* #endif */
}
/* module_exit(wm8728_exit); */

/* MODULE_DESCRIPTION("ASoC WM8728 driver"); */
/* MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
