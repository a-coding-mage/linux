// SPDX-License-Identifier: GPL-2.0-only
/*
 * Texas Instruments TLV320AIC26 low power audio CODEC
 * ALSA SoC CODEC driver
 *
 * Copyright (C) 2008 Secret Lab Technologies Ltd.
 */

/*
 * C dependencies removed from executable Rust:
 * linux/module.h, linux/moduleparam.h, linux/init.h, linux/delay.h,
 * linux/pm.h, linux/device.h, linux/sysfs.h, linux/spi/spi.h,
 * linux/slab.h, sound/core.h, sound/pcm.h, sound/pcm_params.h,
 * sound/soc.h, sound/initval.h, and "tlv320aic26.h".
 */

// MODULE_DESCRIPTION("ASoC TLV320AIC26 codec driver");
// MODULE_AUTHOR("Grant Likely <grant.likely@secretlab.ca>");
// MODULE_LICENSE("GPL");

type ssize_t = isize;
type size_t = usize;
type u16 = u16;

const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const GFP_KERNEL: u32 = 0;

extern "C" {
    static AIC26_DIV_6: i32;
    static AIC26_DIV_4: i32;
    static AIC26_DIV_3: i32;
    static AIC26_DIV_2: i32;
    static AIC26_DIV_1_5: i32;
    static AIC26_DIV_1: i32;
    static AIC26_WLEN_16: i32;
    static AIC26_WLEN_24: i32;
    static AIC26_WLEN_32: i32;
    static AIC26_REG_PLL_PROG1: u32;
    static AIC26_REG_PLL_PROG2: u32;
    static AIC26_REG_AUDIO_CTRL3: u32;
    static AIC26_REG_AUDIO_CTRL1: u32;
    static AIC26_REG_DAC_GAIN: u32;
    static AIC26_REG_ADC_GAIN: u32;
    static AIC26_REG_AUDIO_CTRL2: u32;
    static AIC26_REG_RESET: u32;
    static AIC26_REG_POWER_CTRL: u32;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: u32;
    static SND_SOC_DAIFMT_CBP_CFP: u32;
    static SND_SOC_DAIFMT_CBC_CFC: u32;
    static SND_SOC_DAIFMT_FORMAT_MASK: u32;
    static SND_SOC_DAIFMT_I2S: u32;
    static SND_SOC_DAIFMT_DSP_A: u32;
    static SND_SOC_DAIFMT_RIGHT_J: u32;
    static SND_SOC_DAIFMT_LEFT_J: u32;
    static AIC26_DATFM_I2S: i32;
    static AIC26_DATFM_DSP: i32;
    static AIC26_DATFM_RIGHTJ: i32;
    static AIC26_DATFM_LEFTJ: i32;
    static SNDRV_PCM_RATE_8000: u32;
    static SNDRV_PCM_RATE_11025: u32;
    static SNDRV_PCM_RATE_16000: u32;
    static SNDRV_PCM_RATE_22050: u32;
    static SNDRV_PCM_RATE_32000: u32;
    static SNDRV_PCM_RATE_44100: u32;
    static SNDRV_PCM_RATE_48000: u32;
    static SNDRV_PCM_FMTBIT_S8: u64;
    static SNDRV_PCM_FMTBIT_S16_BE: u64;
    static SNDRV_PCM_FMTBIT_S24_BE: u64;
    static SNDRV_PCM_FMTBIT_S32_BE: u64;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_device {
    pub dev: device,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
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
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const i8,
    pub control: *const i8,
    pub source: *const i8,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct soc_enum {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> i32,
    >,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, i32, i32) -> i32>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, i32, u32, i32) -> i32>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, u32) -> i32>,
    pub no_capture_mute: i32,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const i8,
    pub channels_min: u32,
    pub channels_max: u32,
    pub rates: u32,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const i8,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: u32,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: u32,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: u32,
    pub idle_bias_on: i32,
    pub use_pmdown_time: i32,
    pub endianness: i32,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: u32,
    pub val_bits: u32,
}

#[repr(C)]
pub struct driver {
    pub name: *const i8,
}

#[repr(C)]
pub struct spi_driver {
    pub driver: driver,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> i32>,
}

extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut aic26;
    fn dev_get_drvdata(dev: *mut device) -> *mut aic26;
    fn dev_set_drvdata(dev: *mut device, data: *mut aic26);
    fn params_rate(params: *mut snd_pcm_hw_params) -> i32;
    fn params_width(params: *mut snd_pcm_hw_params) -> i32;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: u32, val: u32) -> i32;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: u32) -> i32;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: u32,
        mask: u32,
        val: u32,
    ) -> i32;
    fn sysfs_emit(buf: *mut i8, fmt: *const i8, ...) -> ssize_t;
    fn device_create_file(dev: *mut device, attr: *const device_attribute) -> i32;
    fn device_remove_file(dev: *mut device, attr: *const device_attribute);
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: u32) -> *mut core::ffi::c_void;
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> i32;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: i32,
    ) -> i32;
    fn dev_dbg(dev: *mut device, fmt: *const i8, ...);
    fn dev_info(dev: *mut device, fmt: *const i8, ...);
}

/* AIC26 driver private data */
#[repr(C)]
pub struct aic26 {
    pub spi: *mut spi_device,
    pub regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub clock_provider: i32,
    pub datfm: i32,
    pub mclk: i32,

    /* Keyclick parameters */
    pub keyclick_amplitude: i32,
    pub keyclick_freq: i32,
    pub keyclick_len: i32,
}

// SND_SOC_DAPM_INPUT("MICIN"), SND_SOC_DAPM_INPUT("AUX"),
// SND_SOC_DAPM_OUTPUT("HPL"), SND_SOC_DAPM_OUTPUT("HPR"),
static tlv320aic26_dapm_widgets: [snd_soc_dapm_widget; 4] = unsafe { core::mem::zeroed() };

static tlv320aic26_dapm_routes: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route {
        sink: b"Capture\0".as_ptr() as *const i8,
        control: core::ptr::null(),
        source: b"MICIN\0".as_ptr() as *const i8,
    },
    snd_soc_dapm_route {
        sink: b"Capture\0".as_ptr() as *const i8,
        control: core::ptr::null(),
        source: b"AUX\0".as_ptr() as *const i8,
    },
    snd_soc_dapm_route {
        sink: b"HPL\0".as_ptr() as *const i8,
        control: core::ptr::null(),
        source: b"Playback\0".as_ptr() as *const i8,
    },
    snd_soc_dapm_route {
        sink: b"HPR\0".as_ptr() as *const i8,
        control: core::ptr::null(),
        source: b"Playback\0".as_ptr() as *const i8,
    },
];

/* ---------------------------------------------------------------------
 * Digital Audio Interface Operations
 */
unsafe extern "C" fn aic26_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> i32 {
    let component: *mut snd_soc_component = (*dai).component;
    let aic26: *mut aic26 = snd_soc_component_get_drvdata(component);
    let fsref: i32;
    let divisor: i32;
    let wlen: i32;
    let pval: i32;
    let jval: i32;
    let mut dval: i32;
    let qval: i32;
    let mut reg: u16;

    dev_dbg(
        &mut (*(*aic26).spi).dev,
        b"aic26_hw_params(substream=%p, params=%p)\n\0".as_ptr() as *const i8,
        substream,
        params,
    );
    dev_dbg(
        &mut (*(*aic26).spi).dev,
        b"rate=%i width=%d\n\0".as_ptr() as *const i8,
        params_rate(params),
        params_width(params),
    );

    match params_rate(params) {
        8000 => {
            fsref = 48000;
            divisor = AIC26_DIV_6;
        }
        11025 => {
            fsref = 44100;
            divisor = AIC26_DIV_4;
        }
        12000 => {
            fsref = 48000;
            divisor = AIC26_DIV_4;
        }
        16000 => {
            fsref = 48000;
            divisor = AIC26_DIV_3;
        }
        22050 => {
            fsref = 44100;
            divisor = AIC26_DIV_2;
        }
        24000 => {
            fsref = 48000;
            divisor = AIC26_DIV_2;
        }
        32000 => {
            fsref = 48000;
            divisor = AIC26_DIV_1_5;
        }
        44100 => {
            fsref = 44100;
            divisor = AIC26_DIV_1;
        }
        48000 => {
            fsref = 48000;
            divisor = AIC26_DIV_1;
        }
        _ => {
            dev_dbg(&mut (*(*aic26).spi).dev, b"bad rate\n\0".as_ptr() as *const i8);
            return -EINVAL;
        }
    }

    /* select data word length */
    match params_width(params) {
        8 => wlen = AIC26_WLEN_16,
        16 => wlen = AIC26_WLEN_16,
        24 => wlen = AIC26_WLEN_24,
        32 => wlen = AIC26_WLEN_32,
        _ => {
            dev_dbg(&mut (*(*aic26).spi).dev, b"bad format\n\0".as_ptr() as *const i8);
            return -EINVAL;
        }
    }

    /**
     * Configure PLL
     * fsref = (mclk * PLLM) / 2048
     * where PLLM = J.DDDD (DDDD register ranges from 0 to 9999, decimal)
     */
    pval = 1;
    /* compute J portion of multiplier */
    jval = fsref / ((*aic26).mclk / 2048);
    /* compute fractional DDDD component of multiplier */
    dval = fsref - (jval * ((*aic26).mclk / 2048));
    dval = (10000 * dval) / ((*aic26).mclk / 2048);
    dev_dbg(
        &mut (*(*aic26).spi).dev,
        b"Setting PLLM to %d.%04d\n\0".as_ptr() as *const i8,
        jval,
        dval,
    );
    qval = 0;
    reg = (0x8000 | qval << 11 | pval << 8 | jval << 2) as u16;
    snd_soc_component_write(component, AIC26_REG_PLL_PROG1, reg as u32);
    reg = (dval << 2) as u16;
    snd_soc_component_write(component, AIC26_REG_PLL_PROG2, reg as u32);

    /* Audio Control 3 (clock provider mode, fsref rate) */
    if (*aic26).clock_provider != 0 {
        reg = 0x0800;
    }
    if fsref == 48000 {
        reg = 0x2000;
    }
    snd_soc_component_update_bits(component, AIC26_REG_AUDIO_CTRL3, 0xf800, reg as u32);

    /* Audio Control 1 (FSref divisor) */
    reg = (wlen | (*aic26).datfm | (divisor << 3) | divisor) as u16;
    snd_soc_component_update_bits(component, AIC26_REG_AUDIO_CTRL1, 0xfff, reg as u32);

    0
}

/*
 * aic26_mute - Mute control to reduce noise when changing audio format
 */
unsafe extern "C" fn aic26_mute(dai: *mut snd_soc_dai, mute: i32, direction: i32) -> i32 {
    let component: *mut snd_soc_component = (*dai).component;
    let aic26: *mut aic26 = snd_soc_component_get_drvdata(component);
    let reg: u16;

    dev_dbg(
        &mut (*(*aic26).spi).dev,
        b"aic26_mute(dai=%p, mute=%i)\n\0".as_ptr() as *const i8,
        dai,
        mute,
    );

    if mute != 0 {
        reg = 0x8080;
    } else {
        reg = 0;
    }
    snd_soc_component_update_bits(component, AIC26_REG_DAC_GAIN, 0x8000, reg as u32);

    0
}

unsafe extern "C" fn aic26_set_sysclk(
    codec_dai: *mut snd_soc_dai,
    clk_id: i32,
    freq: u32,
    dir: i32,
) -> i32 {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let aic26: *mut aic26 = snd_soc_component_get_drvdata(component);

    dev_dbg(
        &mut (*(*aic26).spi).dev,
        b"aic26_set_sysclk(dai=%p, clk_id==%i, freq=%i, dir=%i)\n\0".as_ptr() as *const i8,
        codec_dai,
        clk_id,
        freq,
        dir,
    );

    /* MCLK needs to fall between 2MHz and 50 MHz */
    if (freq < 2000000) || (freq > 50000000) {
        return -EINVAL;
    }

    (*aic26).mclk = freq as i32;
    0
}

unsafe extern "C" fn aic26_set_fmt(codec_dai: *mut snd_soc_dai, fmt: u32) -> i32 {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let aic26: *mut aic26 = snd_soc_component_get_drvdata(component);

    dev_dbg(
        &mut (*(*aic26).spi).dev,
        b"aic26_set_fmt(dai=%p, fmt==%i)\n\0".as_ptr() as *const i8,
        codec_dai,
        fmt,
    );

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP => (*aic26).clock_provider = 1,
        x if x == SND_SOC_DAIFMT_CBC_CFC => (*aic26).clock_provider = 0,
        _ => {
            dev_dbg(&mut (*(*aic26).spi).dev, b"bad master\n\0".as_ptr() as *const i8);
            return -EINVAL;
        }
    }

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => (*aic26).datfm = AIC26_DATFM_I2S,
        x if x == SND_SOC_DAIFMT_DSP_A => (*aic26).datfm = AIC26_DATFM_DSP,
        x if x == SND_SOC_DAIFMT_RIGHT_J => (*aic26).datfm = AIC26_DATFM_RIGHTJ,
        x if x == SND_SOC_DAIFMT_LEFT_J => (*aic26).datfm = AIC26_DATFM_LEFTJ,
        _ => {
            dev_dbg(&mut (*(*aic26).spi).dev, b"bad format\n\0".as_ptr() as *const i8);
            return -EINVAL;
        }
    }

    0
}

/* ---------------------------------------------------------------------
 * Digital Audio Interface Definition
 */
unsafe fn AIC26_RATES() -> u32 {
    SNDRV_PCM_RATE_8000
        | SNDRV_PCM_RATE_11025
        | SNDRV_PCM_RATE_16000
        | SNDRV_PCM_RATE_22050
        | SNDRV_PCM_RATE_32000
        | SNDRV_PCM_RATE_44100
        | SNDRV_PCM_RATE_48000
}

unsafe fn AIC26_FORMATS() -> u64 {
    SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_BE | SNDRV_PCM_FMTBIT_S24_BE | SNDRV_PCM_FMTBIT_S32_BE
}

static aic26_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(aic26_hw_params),
    mute_stream: Some(aic26_mute),
    set_sysclk: Some(aic26_set_sysclk),
    set_fmt: Some(aic26_set_fmt),
    no_capture_mute: 1,
};

static mut aic26_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"tlv320aic26-hifi\0".as_ptr() as *const i8,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const i8,
        channels_min: 2,
        channels_max: 2,
        rates: 0,   // AIC26_RATES
        formats: 0, // AIC26_FORMATS
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const i8,
        channels_min: 2,
        channels_max: 2,
        rates: 0,   // AIC26_RATES
        formats: 0, // AIC26_FORMATS
    },
    ops: &aic26_dai_ops,
};

/* ---------------------------------------------------------------------
 * ALSA controls
 */
static aic26_capture_src_text: [*const i8; 2] = [
    b"Mic\0".as_ptr() as *const i8,
    b"Aux\0".as_ptr() as *const i8,
];

// static SOC_ENUM_SINGLE_DECL(aic26_capture_src_enum,
//                             AIC26_REG_AUDIO_CTRL1, 12,
//                             aic26_capture_src_text);
static aic26_capture_src_enum: soc_enum = unsafe { core::mem::zeroed() };

// Controls created by SOC_DOUBLE, SOC_SINGLE, and SOC_ENUM C macros.
static aic26_snd_controls: [snd_kcontrol_new; 9] = unsafe { core::mem::zeroed() };

/* ---------------------------------------------------------------------
 * SPI device portion of driver: sysfs files for debugging
 */

unsafe extern "C" fn keyclick_show(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut i8,
) -> ssize_t {
    let aic26: *mut aic26 = dev_get_drvdata(dev);
    let val: i32;
    let amp: i32;
    let freq: i32;
    let len: i32;

    val = snd_soc_component_read((*aic26).component, AIC26_REG_AUDIO_CTRL2);
    amp = (val >> 12) & 0x7;
    freq = (125 << ((val >> 8) & 0x7)) >> 1;
    len = 2 * (1 + ((val >> 4) & 0xf));

    sysfs_emit(
        buf,
        b"amp=%x freq=%iHz len=%iclks\n\0".as_ptr() as *const i8,
        amp,
        freq,
        len,
    )
}

/* Any write to the keyclick attribute will trigger the keyclick event */
unsafe extern "C" fn keyclick_store(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *const i8,
    count: size_t,
) -> ssize_t {
    let aic26: *mut aic26 = dev_get_drvdata(dev);

    snd_soc_component_update_bits(
        (*aic26).component,
        AIC26_REG_AUDIO_CTRL2,
        0x8000,
        0x800,
    );

    count as ssize_t
}

// static DEVICE_ATTR_RW(keyclick);
extern "C" {
    static dev_attr_keyclick: device_attribute;
}

/* ---------------------------------------------------------------------
 * SoC CODEC portion of driver: probe and release routines
 */
unsafe extern "C" fn aic26_probe(component: *mut snd_soc_component) -> i32 {
    let aic26: *mut aic26 = dev_get_drvdata((*component).dev);
    let ret: i32;
    let mut reg: i32;

    (*aic26).component = component;

    /* Reset the codec to power on defaults */
    snd_soc_component_write(component, AIC26_REG_RESET, 0xBB00);

    /* Power up CODEC */
    snd_soc_component_write(component, AIC26_REG_POWER_CTRL, 0);

    /* Audio Control 3 (master mode, fsref rate) */
    reg = snd_soc_component_read(component, AIC26_REG_AUDIO_CTRL3);
    reg &= !0xf800;
    reg |= 0x0800; /* set master mode */
    snd_soc_component_write(component, AIC26_REG_AUDIO_CTRL3, reg as u32);

    /* Register the sysfs files for debugging */
    /* Create SysFS files */
    ret = device_create_file((*component).dev, &dev_attr_keyclick);
    if ret != 0 {
        dev_info(
            (*component).dev,
            b"error creating sysfs files\n\0".as_ptr() as *const i8,
        );
    }

    0
}

unsafe extern "C" fn aic26_remove(component: *mut snd_soc_component) {
    device_remove_file((*component).dev, &dev_attr_keyclick);
}

static aic26_soc_component_dev: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(aic26_probe),
    remove: Some(aic26_remove),
    controls: aic26_snd_controls.as_ptr(),
    num_controls: aic26_snd_controls.len() as u32,
    dapm_widgets: tlv320aic26_dapm_widgets.as_ptr(),
    num_dapm_widgets: tlv320aic26_dapm_widgets.len() as u32,
    dapm_routes: tlv320aic26_dapm_routes.as_ptr(),
    num_dapm_routes: tlv320aic26_dapm_routes.len() as u32,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static aic26_regmap: regmap_config = regmap_config {
    reg_bits: 16,
    val_bits: 16,
};

/* ---------------------------------------------------------------------
 * SPI device portion of driver: probe and release routines and SPI
 *                                  driver registration.
 */
unsafe extern "C" fn aic26_spi_probe(spi: *mut spi_device) -> i32 {
    let aic26: *mut aic26;
    let ret: i32;

    dev_dbg(
        &mut (*spi).dev,
        b"probing tlv320aic26 spi device\n\0".as_ptr() as *const i8,
    );

    /* Allocate driver data */
    aic26 = devm_kzalloc(
        &mut (*spi).dev,
        core::mem::size_of::<aic26>(),
        GFP_KERNEL,
    ) as *mut aic26;
    if aic26.is_null() {
        return -ENOMEM;
    }

    (*aic26).regmap = devm_regmap_init_spi(spi, &aic26_regmap);
    if IS_ERR((*aic26).regmap as *const core::ffi::c_void) {
        return PTR_ERR((*aic26).regmap as *const core::ffi::c_void);
    }

    /* Initialize the driver data */
    (*aic26).spi = spi;
    dev_set_drvdata(&mut (*spi).dev, aic26);
    (*aic26).clock_provider = 1;

    ret = devm_snd_soc_register_component(
        &mut (*spi).dev,
        &aic26_soc_component_dev,
        &mut aic26_dai,
        1,
    );
    ret
}

static mut aic26_spi: spi_driver = spi_driver {
    driver: driver {
        name: b"tlv320aic26-codec\0".as_ptr() as *const i8,
    },
    probe: Some(aic26_spi_probe),
};

// module_spi_driver(aic26_spi);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
