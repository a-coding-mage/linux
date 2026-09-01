// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * AK4104 ALSA SoC (ASoC) driver
 *
 * Copyright (c) 2009 Daniel Mack <daniel@caiaq.de>
 */

/* Dependencies from:
 * <linux/module.h>
 * <linux/slab.h>
 * <linux/spi/spi.h>
 * <linux/gpio/consumer.h>
 * <linux/regulator/consumer.h>
 * <sound/asoundef.h>
 * <sound/core.h>
 * <sound/soc.h>
 * <sound/initval.h>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

/* AK4104 registers addresses */
const AK4104_REG_CONTROL1: c_uint = 0x00;
const AK4104_REG_RESERVED: c_uint = 0x01;
const AK4104_REG_CONTROL2: c_uint = 0x02;
const AK4104_REG_TX: c_uint = 0x03;
const fn AK4104_REG_CHN_STATUS(x: c_uint) -> c_uint {
    x + 0x04
}
const AK4104_NUM_REGS: c_uint = 10;

const AK4104_REG_MASK: c_uint = 0x1f;
const AK4104_READ: c_uint = 0xc0;
const AK4104_WRITE: c_uint = 0xe0;
const AK4104_RESERVED_VAL: c_uint = 0x5b;

/* Bit masks for AK4104 registers */
const AK4104_CONTROL1_RSTN: c_int = 1 << 0;
const AK4104_CONTROL1_PW: c_int = 1 << 1;
const AK4104_CONTROL1_DIF0: c_int = 1 << 2;
const AK4104_CONTROL1_DIF1: c_int = 1 << 3;

const AK4104_CONTROL2_SEL0: c_int = 1 << 0;
const AK4104_CONTROL2_SEL1: c_int = 1 << 1;
const AK4104_CONTROL2_MODE: c_int = 1 << 2;

const AK4104_TX_TXE: c_int = 1 << 0;
const AK4104_TX_V: c_int = 1 << 1;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const EPROBE_DEFER: c_int = 517;
const GFP_KERNEL: c_uint = 0;
const SPI_MODE_0: c_uint = 0;
const GPIOD_OUT_HIGH: c_uint = 1;
const REGCACHE_RBTREE: c_uint = 2;

extern "C" {
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;

    static IEC958_AES0_CON_NOT_COPYRIGHT: c_int;
    static IEC958_AES3_CON_FS_22050: c_int;
    static IEC958_AES3_CON_FS_24000: c_int;
    static IEC958_AES3_CON_FS_32000: c_int;
    static IEC958_AES3_CON_FS_44100: c_int;
    static IEC958_AES3_CON_FS_48000: c_int;
    static IEC958_AES3_CON_FS_88200: c_int;
    static IEC958_AES3_CON_FS_96000: c_int;
    static IEC958_AES3_CON_FS_176400: c_int;
    static IEC958_AES3_CON_FS_192000: c_int;

    static SNDRV_PCM_RATE_22050: c_uint;
    static SNDRV_PCM_RATE_32000: c_uint;
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_RATE_88200: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_176400: c_uint;
    static SNDRV_PCM_RATE_192000: c_uint;

    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regulator {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
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
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct spi_device {
    pub dev: device,
    pub bits_per_word: c_uint,
    pub mode: c_uint,
}

#[repr(C)]
pub struct ak4104_private {
    pub regmap: *mut regmap,
    pub regulator: *mut regulator,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub name: *const c_char,
    pub reg: c_int,
    pub shift: c_int,
    pub invert: c_int,
    pub kcontrol_news: *const c_void,
    pub num_kcontrols: c_int,
    pub kind: c_int,
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
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
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
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
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
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub read_flag_mask: c_uint,
    pub write_flag_mask: c_uint,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct spi_device_id {
    pub name: *const c_char,
    pub driver_data: c_ulong,
}

type c_ulong = u64;

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct spi_driver {
    pub driver: device_driver,
    pub id_table: *const spi_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
}

extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn regmap_update_bits(
        map: *mut regmap,
        reg: c_uint,
        mask: c_int,
        val: c_int,
    ) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_int) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regulator_enable(regulator: *mut regulator) -> c_int;
    fn regulator_disable(regulator: *mut regulator) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn spi_setup(spi: *mut spi_device) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regulator_get(dev: *mut device, id: *const c_char) -> *mut regulator;
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_uint,
    ) -> *mut gpio_desc;
    fn spi_set_drvdata(spi: *mut spi_device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

unsafe fn IS_ERR(ptr: *const c_void) -> bool {
    (ptr as isize) >= -4095isize && (ptr as isize) < 0
}

unsafe fn PTR_ERR<T>(ptr: *const T) -> c_int {
    ptr as isize as c_int
}

const fn array_size<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

const fn snd_soc_dapm_pga(
    name: *const c_char,
    reg: c_int,
    shift: c_int,
    invert: c_int,
    kcontrol_news: *const c_void,
    num_kcontrols: c_int,
) -> snd_soc_dapm_widget {
    snd_soc_dapm_widget {
        name,
        reg,
        shift,
        invert,
        kcontrol_news,
        num_kcontrols,
        kind: 0,
    }
}

const fn snd_soc_dapm_output(name: *const c_char) -> snd_soc_dapm_widget {
    snd_soc_dapm_widget {
        name,
        reg: 0,
        shift: 0,
        invert: 0,
        kcontrol_news: ptr::null(),
        num_kcontrols: 0,
        kind: 1,
    }
}

static ak4104_dapm_widgets: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_pga(
        c"TXE".as_ptr(),
        AK4104_REG_TX as c_int,
        AK4104_TX_TXE,
        0,
        ptr::null(),
        0,
    ),
    snd_soc_dapm_output(c"TX".as_ptr()),
];

static ak4104_dapm_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: c"TXE".as_ptr(),
        control: ptr::null(),
        source: c"Playback".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"TX".as_ptr(),
        control: ptr::null(),
        source: c"TXE".as_ptr(),
    },
];

unsafe extern "C" fn ak4104_set_dai_fmt(
    codec_dai: *mut snd_soc_dai,
    format: c_uint,
) -> c_int {
    let component = unsafe { (*codec_dai).component };
    let ak4104 =
        unsafe { snd_soc_component_get_drvdata(component) as *mut ak4104_private };
    let mut val: c_int = 0;
    let ret: c_int;

    /* set DAI format */
    unsafe {
        match format & SND_SOC_DAIFMT_FORMAT_MASK {
            x if x == SND_SOC_DAIFMT_RIGHT_J => {}
            x if x == SND_SOC_DAIFMT_LEFT_J => {
                val |= AK4104_CONTROL1_DIF0;
            }
            x if x == SND_SOC_DAIFMT_I2S => {
                val |= AK4104_CONTROL1_DIF0 | AK4104_CONTROL1_DIF1;
            }
            _ => {
                dev_err((*component).dev, c"invalid dai format\n".as_ptr());
                return -EINVAL;
            }
        }
    }

    /* This device can only be consumer */
    unsafe {
        if (format & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) != SND_SOC_DAIFMT_CBC_CFC {
            return -EINVAL;
        }
    }

    ret = unsafe {
        regmap_update_bits(
            (*ak4104).regmap,
            AK4104_REG_CONTROL1,
            AK4104_CONTROL1_DIF0 | AK4104_CONTROL1_DIF1,
            val,
        )
    };
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn ak4104_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = unsafe { (*dai).component };
    let ak4104 =
        unsafe { snd_soc_component_get_drvdata(component) as *mut ak4104_private };
    let ret: c_int;
    let mut val: c_int = 0;

    /* set the IEC958 bits: consumer mode, no copyright bit */
    unsafe {
        val |= IEC958_AES0_CON_NOT_COPYRIGHT;
        regmap_write((*ak4104).regmap, AK4104_REG_CHN_STATUS(0), val);
    }

    val = 0;

    unsafe {
        match params_rate(params) {
            22050 => {
                val |= IEC958_AES3_CON_FS_22050;
            }
            24000 => {
                val |= IEC958_AES3_CON_FS_24000;
            }
            32000 => {
                val |= IEC958_AES3_CON_FS_32000;
            }
            44100 => {
                val |= IEC958_AES3_CON_FS_44100;
            }
            48000 => {
                val |= IEC958_AES3_CON_FS_48000;
            }
            88200 => {
                val |= IEC958_AES3_CON_FS_88200;
            }
            96000 => {
                val |= IEC958_AES3_CON_FS_96000;
            }
            176400 => {
                val |= IEC958_AES3_CON_FS_176400;
            }
            192000 => {
                val |= IEC958_AES3_CON_FS_192000;
            }
            _ => {
                dev_err((*component).dev, c"unsupported sampling rate\n".as_ptr());
                return -EINVAL;
            }
        }
    }

    ret = unsafe {
        regmap_write(
            (*ak4104).regmap,
            AK4104_REG_CHN_STATUS(3),
            val,
        )
    };
    if ret < 0 {
        return ret;
    }

    0
}

static ak4101_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(ak4104_hw_params),
    set_fmt: Some(ak4104_set_dai_fmt),
};

static mut ak4104_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"ak4104-hifi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: unsafe {
            SNDRV_PCM_RATE_22050
                | SNDRV_PCM_RATE_32000
                | SNDRV_PCM_RATE_44100
                | SNDRV_PCM_RATE_48000
                | SNDRV_PCM_RATE_88200
                | SNDRV_PCM_RATE_96000
                | SNDRV_PCM_RATE_176400
                | SNDRV_PCM_RATE_192000
        },
        formats: unsafe {
            SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S24_LE
        },
    },
    ops: &ak4101_dai_ops,
};

unsafe extern "C" fn ak4104_probe(component: *mut snd_soc_component) -> c_int {
    let ak4104 =
        unsafe { snd_soc_component_get_drvdata(component) as *mut ak4104_private };
    let mut ret: c_int;

    ret = unsafe { regulator_enable((*ak4104).regulator) };
    if ret < 0 {
        unsafe {
            dev_err(
                (*component).dev,
                c"Unable to enable regulator: %d\n".as_ptr(),
                ret,
            );
        }
        return ret;
    }

    /* set power-up and non-reset bits */
    ret = unsafe {
        regmap_update_bits(
            (*ak4104).regmap,
            AK4104_REG_CONTROL1,
            AK4104_CONTROL1_PW | AK4104_CONTROL1_RSTN,
            AK4104_CONTROL1_PW | AK4104_CONTROL1_RSTN,
        )
    };
    if ret < 0 {
        unsafe {
            regulator_disable((*ak4104).regulator);
        }
        return ret;
    }

    /* enable transmitter */
    ret = unsafe {
        regmap_update_bits(
            (*ak4104).regmap,
            AK4104_REG_TX,
            AK4104_TX_TXE,
            AK4104_TX_TXE,
        )
    };
    if ret < 0 {
        unsafe {
            regulator_disable((*ak4104).regulator);
        }
        return ret;
    }

    0
}

unsafe extern "C" fn ak4104_remove(component: *mut snd_soc_component) {
    let ak4104 =
        unsafe { snd_soc_component_get_drvdata(component) as *mut ak4104_private };

    unsafe {
        regmap_update_bits(
            (*ak4104).regmap,
            AK4104_REG_CONTROL1,
            AK4104_CONTROL1_PW | AK4104_CONTROL1_RSTN,
            0,
        );
        regulator_disable((*ak4104).regulator);
    }
}

/* Original C condition: #ifdef CONFIG_PM */
unsafe extern "C" fn ak4104_soc_suspend(component: *mut snd_soc_component) -> c_int {
    let priv_ =
        unsafe { snd_soc_component_get_drvdata(component) as *mut ak4104_private };

    unsafe {
        regulator_disable((*priv_).regulator);
    }

    0
}

unsafe extern "C" fn ak4104_soc_resume(component: *mut snd_soc_component) -> c_int {
    let priv_ =
        unsafe { snd_soc_component_get_drvdata(component) as *mut ak4104_private };
    let ret: c_int;

    ret = unsafe { regulator_enable((*priv_).regulator) };
    if ret < 0 {
        return ret;
    }

    0
}
/* Original C #else mapped ak4104_soc_suspend and ak4104_soc_resume to NULL. */

static soc_component_device_ak4104: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(ak4104_probe),
    remove: Some(ak4104_remove),
    suspend: Some(ak4104_soc_suspend),
    resume: Some(ak4104_soc_resume),
    dapm_widgets: ak4104_dapm_widgets.as_ptr(),
    num_dapm_widgets: array_size(&ak4104_dapm_widgets),
    dapm_routes: ak4104_dapm_routes.as_ptr(),
    num_dapm_routes: array_size(&ak4104_dapm_routes),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static ak4104_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,

    max_register: AK4104_NUM_REGS - 1,
    read_flag_mask: AK4104_READ,
    write_flag_mask: AK4104_WRITE,

    cache_type: REGCACHE_RBTREE,
};

unsafe extern "C" fn ak4104_spi_probe(spi: *mut spi_device) -> c_int {
    let ak4104: *mut ak4104_private;
    let reset_gpiod: *mut gpio_desc;
    let mut val: c_uint = 0;
    let mut ret: c_int;

    unsafe {
        (*spi).bits_per_word = 8;
        (*spi).mode = SPI_MODE_0;
    }
    ret = unsafe { spi_setup(spi) };
    if ret < 0 {
        return ret;
    }

    ak4104 = unsafe {
        devm_kzalloc(
            &mut (*spi).dev,
            size_of::<ak4104_private>(),
            GFP_KERNEL,
        ) as *mut ak4104_private
    };
    if ak4104.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*ak4104).regulator = devm_regulator_get(&mut (*spi).dev, c"vdd".as_ptr());
        if IS_ERR((*ak4104).regulator as *const c_void) {
            ret = PTR_ERR((*ak4104).regulator);
            dev_err(
                &mut (*spi).dev,
                c"Unable to get Vdd regulator: %d\n".as_ptr(),
                ret,
            );
            return ret;
        }
    }

    unsafe {
        (*ak4104).regmap = devm_regmap_init_spi(spi, &ak4104_regmap);
        if IS_ERR((*ak4104).regmap as *const c_void) {
            ret = PTR_ERR((*ak4104).regmap);
            return ret;
        }
    }

    reset_gpiod = unsafe {
        devm_gpiod_get_optional(&mut (*spi).dev, c"reset".as_ptr(), GPIOD_OUT_HIGH)
    };
    if unsafe { PTR_ERR(reset_gpiod) } == -EPROBE_DEFER {
        return -EPROBE_DEFER;
    }

    /* read the 'reserved' register - according to the datasheet, it
     * should contain 0x5b. Not a good way to verify the presence of
     * the device, but there is no hardware ID register. */
    ret = unsafe { regmap_read((*ak4104).regmap, AK4104_REG_RESERVED, &mut val) };
    if ret != 0 {
        return ret;
    }
    if val != AK4104_RESERVED_VAL {
        return -ENODEV;
    }

    unsafe {
        spi_set_drvdata(spi, ak4104 as *mut c_void);
    }

    ret = unsafe {
        devm_snd_soc_register_component(
            &mut (*spi).dev,
            &soc_component_device_ak4104,
            &raw mut ak4104_dai,
            1,
        )
    };
    ret
}

static ak4104_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"asahi-kasei,ak4104".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, ak4104_of_match); */

static ak4104_id_table: [spi_device_id; 2] = [
    spi_device_id {
        name: c"ak4104".as_ptr(),
        driver_data: 0,
    },
    spi_device_id {
        name: ptr::null(),
        driver_data: 0,
    },
];
/* MODULE_DEVICE_TABLE(spi, ak4104_id_table); */

static mut ak4104_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: c"ak4104".as_ptr(),
        of_match_table: ak4104_of_match.as_ptr(),
    },
    id_table: ak4104_id_table.as_ptr(),
    probe: Some(ak4104_spi_probe),
};

/* module_spi_driver(ak4104_spi_driver); */

/* MODULE_AUTHOR("Daniel Mack <daniel@caiaq.de>"); */
/* MODULE_DESCRIPTION("Asahi Kasei AK4104 ALSA SoC driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
