// SPDX-License-Identifier: GPL-2.0
//
// CS40L50 Advanced Haptic Driver with waveform memory,
// integrated DSP, and closed-loop algorithms
//
// Copyright 2024 Cirrus Logic, Inc.
//
// Author: James Ogletree <james.ogletree@cirrus.com>

// C dependencies:
// linux/bitfield.h
// linux/mfd/cs40l50.h
// sound/pcm_params.h
// sound/soc.h

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const fn bit(nr: c_uint) -> c_uint {
    1_u32 << nr
}

const fn genmask(h: c_uint, l: c_uint) -> c_uint {
    (!0_u32 << l) & (!0_u32 >> (31 - h))
}

const fn field_prep(mask: c_uint, val: c_uint) -> c_uint {
    (val << mask.trailing_zeros()) & mask
}

const CS40L50_REFCLK_INPUT: c_uint = 0x2C04;
const CS40L50_ASP_CONTROL2: c_uint = 0x4808;
const CS40L50_ASP_DATA_CONTROL5: c_uint = 0x4840;

/* PLL Config */
const CS40L50_PLL_REFCLK_BCLK: c_uint = 0x0;
const CS40L50_PLL_REFCLK_MCLK: c_uint = 0x5;
const CS40L50_PLL_REEFCLK_MCLK_CFG: c_uint = 0x00;
const CS40L50_PLL_REFCLK_LOOP_MASK: c_uint = bit(11);
const CS40L50_PLL_REFCLK_OPEN_LOOP: c_uint = 1;
const CS40L50_PLL_REFCLK_CLOSED_LOOP: c_uint = 0;
const CS40L50_PLL_REFCLK_LOOP_SHIFT: c_uint = 11;
const CS40L50_PLL_REFCLK_FREQ_MASK: c_uint = genmask(10, 5);
const CS40L50_PLL_REFCLK_FREQ_SHIFT: c_uint = 5;
const CS40L50_PLL_REFCLK_SEL_MASK: c_uint = genmask(2, 0);
const CS40L50_BCLK_RATIO_DEFAULT: c_uint = 32;

/* ASP Config */
const CS40L50_ASP_RX_WIDTH_SHIFT: c_uint = 24;
const CS40L50_ASP_RX_WIDTH_MASK: c_uint = genmask(31, 24);
const CS40L50_ASP_RX_WL_MASK: c_uint = genmask(5, 0);
const CS40L50_ASP_FSYNC_INV_MASK: c_uint = bit(2);
const CS40L50_ASP_BCLK_INV_MASK: c_uint = bit(6);
const CS40L50_ASP_FMT_MASK: c_uint = genmask(10, 8);
const CS40L50_ASP_FMT_I2S: c_uint = 0x2;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

extern "C" {
    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static SND_SOC_NOPM: c_int;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static GFP_KERNEL: c_uint;
    static CS40L50_STOP_PLAYBACK: c_uint;
    static CS40L50_START_I2S: c_uint;

    fn regmap_update_bits(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn cs40l50_dsp_write(dev: *mut device, regmap: *mut regmap, cmd: c_uint) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

#[repr(C)]
pub struct device {
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
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct snd_soc_component {
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
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device_with_parent,
}

#[repr(C)]
pub struct device_with_parent {
    pub parent: *mut device,
}

#[repr(C)]
pub struct cs40l50 {
    pub regmap: *mut regmap,
}

#[repr(C)]
struct cs40l50_pll_config {
    freq: c_uint,
    cfg: c_uint,
}

#[repr(C)]
struct cs40l50_codec {
    dev: *mut device,
    regmap: *mut regmap,
    daifmt: c_uint,
    bclk_ratio: c_uint,
    rate: c_uint,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
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
pub struct snd_soc_dai_ops {
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_bclk_ratio: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: usize,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: usize,
}

#[repr(C)]
pub struct platform_device_id {
    pub name: [c_char; 20],
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub id_table: *const platform_device_id,
    pub driver: platform_driver_inner,
}

static CS40L50_PLL_CFG: [cs40l50_pll_config; 6] = [
    cs40l50_pll_config { freq: 32768, cfg: 0x00 },
    cs40l50_pll_config { freq: 1536000, cfg: 0x1B },
    cs40l50_pll_config { freq: 3072000, cfg: 0x21 },
    cs40l50_pll_config { freq: 6144000, cfg: 0x28 },
    cs40l50_pll_config { freq: 9600000, cfg: 0x30 },
    cs40l50_pll_config { freq: 12288000, cfg: 0x33 },
];

unsafe extern "C" fn cs40l50_get_clk_config(freq: c_uint, cfg: *mut c_uint) -> c_int {
    let mut i: c_int = 0;

    while (i as usize) < CS40L50_PLL_CFG.len() {
        if CS40L50_PLL_CFG[i as usize].freq == freq {
            *cfg = CS40L50_PLL_CFG[i as usize].cfg;
            return 0;
        }

        i += 1;
    }

    -EINVAL
}

unsafe extern "C" fn cs40l50_swap_ext_clk(
    codec: *mut cs40l50_codec,
    clk_src: c_uint,
) -> c_int {
    let mut cfg: c_uint = 0;
    let mut ret: c_int;

    match clk_src {
        CS40L50_PLL_REFCLK_BCLK => {
            ret = cs40l50_get_clk_config(
                (*codec).bclk_ratio.wrapping_mul((*codec).rate),
                &mut cfg,
            );
            if ret != 0 {
                return ret;
            }
        }
        CS40L50_PLL_REFCLK_MCLK => {
            cfg = CS40L50_PLL_REEFCLK_MCLK_CFG;
        }
        _ => return -EINVAL,
    }

    ret = regmap_update_bits(
        (*codec).regmap,
        CS40L50_REFCLK_INPUT,
        CS40L50_PLL_REFCLK_LOOP_MASK,
        CS40L50_PLL_REFCLK_OPEN_LOOP << CS40L50_PLL_REFCLK_LOOP_SHIFT,
    );
    if ret != 0 {
        return ret;
    }

    ret = regmap_update_bits(
        (*codec).regmap,
        CS40L50_REFCLK_INPUT,
        CS40L50_PLL_REFCLK_FREQ_MASK | CS40L50_PLL_REFCLK_SEL_MASK,
        (cfg << CS40L50_PLL_REFCLK_FREQ_SHIFT) | clk_src,
    );
    if ret != 0 {
        return ret;
    }

    regmap_update_bits(
        (*codec).regmap,
        CS40L50_REFCLK_INPUT,
        CS40L50_PLL_REFCLK_LOOP_MASK,
        CS40L50_PLL_REFCLK_CLOSED_LOOP << CS40L50_PLL_REFCLK_LOOP_SHIFT,
    )
}

unsafe extern "C" fn cs40l50_clk_en(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let comp: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let codec: *mut cs40l50_codec = snd_soc_component_get_drvdata(comp) as *mut cs40l50_codec;
    let mut ret: c_int;

    if event == SND_SOC_DAPM_POST_PMU {
        ret = cs40l50_dsp_write((*codec).dev, (*codec).regmap, CS40L50_STOP_PLAYBACK);
        if ret != 0 {
            return ret;
        }

        ret = cs40l50_dsp_write((*codec).dev, (*codec).regmap, CS40L50_START_I2S);
        if ret != 0 {
            return ret;
        }

        ret = cs40l50_swap_ext_clk(codec, CS40L50_PLL_REFCLK_BCLK);
        if ret != 0 {
            return ret;
        }
    } else if event == SND_SOC_DAPM_PRE_PMD {
        ret = cs40l50_swap_ext_clk(codec, CS40L50_PLL_REFCLK_MCLK);
        if ret != 0 {
            return ret;
        }
    } else {
        return -EINVAL;
    }

    0
}

// Original C uses SND_SOC_DAPM_* initializer macros supplied by ASoC.
static CS40L50_DAPM_WIDGETS: [snd_soc_dapm_widget; 4] = [
    /* SND_SOC_DAPM_SUPPLY_S("ASP PLL", 0, SND_SOC_NOPM, 0, 0, cs40l50_clk_en,
     *                       SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD)
     */
    snd_soc_dapm_widget { dapm: ptr::null_mut() },
    /* SND_SOC_DAPM_AIF_IN("ASPRX1", NULL, 0, SND_SOC_NOPM, 0, 0) */
    snd_soc_dapm_widget { dapm: ptr::null_mut() },
    /* SND_SOC_DAPM_AIF_IN("ASPRX2", NULL, 0, SND_SOC_NOPM, 0, 0) */
    snd_soc_dapm_widget { dapm: ptr::null_mut() },
    /* SND_SOC_DAPM_OUTPUT("OUT") */
    snd_soc_dapm_widget { dapm: ptr::null_mut() },
];

static CS40L50_DAPM_ROUTES: [snd_soc_dapm_route; 5] = [
    snd_soc_dapm_route {
        sink: b"ASP Playback\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"ASP PLL\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"ASPRX1\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"ASP Playback\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"ASPRX2\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"ASP Playback\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"OUT\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"ASPRX1\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"OUT\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"ASPRX2\0".as_ptr() as *const c_char,
    },
];

unsafe extern "C" fn cs40l50_set_dai_fmt(
    codec_dai: *mut snd_soc_dai,
    fmt: c_uint,
) -> c_int {
    let codec: *mut cs40l50_codec =
        snd_soc_component_get_drvdata((*codec_dai).component) as *mut cs40l50_codec;

    if (fmt & SND_SOC_DAIFMT_MASTER_MASK) != SND_SOC_DAIFMT_CBC_CFC {
        return -EINVAL;
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => {
            (*codec).daifmt = 0;
        }
        x if x == SND_SOC_DAIFMT_NB_IF => {
            (*codec).daifmt = CS40L50_ASP_FSYNC_INV_MASK;
        }
        x if x == SND_SOC_DAIFMT_IB_NF => {
            (*codec).daifmt = CS40L50_ASP_BCLK_INV_MASK;
        }
        x if x == SND_SOC_DAIFMT_IB_IF => {
            (*codec).daifmt = CS40L50_ASP_FSYNC_INV_MASK | CS40L50_ASP_BCLK_INV_MASK;
        }
        _ => {
            dev_err((*codec).dev, b"Invalid clock invert\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => {
            (*codec).daifmt |= field_prep(CS40L50_ASP_FMT_MASK, CS40L50_ASP_FMT_I2S);
        }
        _ => {
            dev_err((*codec).dev, b"Unsupported DAI format\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn cs40l50_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let codec: *mut cs40l50_codec =
        snd_soc_component_get_drvdata((*dai).component) as *mut cs40l50_codec;
    let asp_rx_wl: c_uint = params_width(params);
    let mut ret: c_int;

    (*codec).rate = params_rate(params);

    ret = regmap_update_bits(
        (*codec).regmap,
        CS40L50_ASP_DATA_CONTROL5,
        CS40L50_ASP_RX_WL_MASK,
        asp_rx_wl,
    );
    if ret != 0 {
        return ret;
    }

    (*codec).daifmt |= asp_rx_wl << CS40L50_ASP_RX_WIDTH_SHIFT;

    regmap_update_bits(
        (*codec).regmap,
        CS40L50_ASP_CONTROL2,
        CS40L50_ASP_FSYNC_INV_MASK
            | CS40L50_ASP_BCLK_INV_MASK
            | CS40L50_ASP_FMT_MASK
            | CS40L50_ASP_RX_WIDTH_MASK,
        (*codec).daifmt,
    )
}

unsafe extern "C" fn cs40l50_set_dai_bclk_ratio(
    dai: *mut snd_soc_dai,
    ratio: c_uint,
) -> c_int {
    let codec: *mut cs40l50_codec =
        snd_soc_component_get_drvdata((*dai).component) as *mut cs40l50_codec;

    (*codec).bclk_ratio = ratio;

    0
}

static CS40L50_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(cs40l50_set_dai_fmt),
    set_bclk_ratio: Some(cs40l50_set_dai_bclk_ratio),
    hw_params: Some(cs40l50_hw_params),
};

static mut CS40L50_DAI: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: b"cs40l50-pcm\0".as_ptr() as *const c_char,
    id: 0,
    playback: snd_soc_pcm_stream {
        stream_name: b"ASP Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: unsafe { SNDRV_PCM_RATE_48000 },
        formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE },
    },
    ops: &CS40L50_DAI_OPS,
}];

unsafe extern "C" fn cs40l50_codec_probe(component: *mut snd_soc_component) -> c_int {
    let codec: *mut cs40l50_codec =
        snd_soc_component_get_drvdata(component) as *mut cs40l50_codec;

    (*codec).bclk_ratio = CS40L50_BCLK_RATIO_DEFAULT;

    0
}

static SOC_CODEC_DEV_CS40L50: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(cs40l50_codec_probe),
    dapm_widgets: CS40L50_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: CS40L50_DAPM_WIDGETS.len(),
    dapm_routes: CS40L50_DAPM_ROUTES.as_ptr(),
    num_dapm_routes: CS40L50_DAPM_ROUTES.len(),
};

unsafe extern "C" fn cs40l50_codec_driver_probe(pdev: *mut platform_device) -> c_int {
    let cs40l50: *mut cs40l50 = dev_get_drvdata((*pdev).dev.parent) as *mut cs40l50;
    let codec: *mut cs40l50_codec;

    codec = devm_kzalloc(
        &mut (*pdev).dev as *mut device_with_parent as *mut device,
        size_of::<cs40l50_codec>(),
        GFP_KERNEL,
    ) as *mut cs40l50_codec;
    if codec.is_null() {
        return -ENOMEM;
    }

    (*codec).regmap = (*cs40l50).regmap;
    (*codec).dev = &mut (*pdev).dev as *mut device_with_parent as *mut device;

    devm_snd_soc_register_component(
        &mut (*pdev).dev as *mut device_with_parent as *mut device,
        &SOC_CODEC_DEV_CS40L50,
        CS40L50_DAI.as_mut_ptr(),
        CS40L50_DAI.len() as c_int,
    )
}

static CS40L50_ID: [platform_device_id; 2] = [
    platform_device_id {
        name: [
            b'c' as c_char,
            b's' as c_char,
            b'4' as c_char,
            b'0' as c_char,
            b'l' as c_char,
            b'5' as c_char,
            b'0' as c_char,
            b'-' as c_char,
            b'c' as c_char,
            b'o' as c_char,
            b'd' as c_char,
            b'e' as c_char,
            b'c' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    },
    platform_device_id { name: [0; 20] },
];
// MODULE_DEVICE_TABLE(platform, cs40l50_id);

static CS40L50_CODEC_DRIVER: platform_driver = platform_driver {
    probe: Some(cs40l50_codec_driver_probe),
    id_table: CS40L50_ID.as_ptr(),
    driver: platform_driver_inner {
        name: b"cs40l50-codec\0".as_ptr() as *const c_char,
    },
};
// module_platform_driver(cs40l50_codec_driver);

// MODULE_DESCRIPTION("ASoC CS40L50 driver");
// MODULE_AUTHOR("James Ogletree <james.ogletree@cirrus.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
