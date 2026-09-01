// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (c) 2018 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

// C dependencies: linux/clk.h, linux/module.h, linux/of_platform.h,
// linux/regmap.h, sound/soc.h, sound/soc-dai.h, sound/pcm_params.h,
// sound/pcm_iec958.h

/*
 * NOTE:
 * The meaning of bits SPDIFOUT_CTRL0_XXX_SEL is actually the opposite
 * of what the documentation says. Manual control on V, U and C bits is
 * applied when the related sel bits are cleared
 */

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

const fn GENMASK(h: u32, l: u32) -> u32 {
    ((!0u32) << l) & ((!0u32) >> (31 - h))
}

const SPDIFOUT_STAT: u32 = 0x00;
const SPDIFOUT_GAIN0: u32 = 0x04;
const SPDIFOUT_GAIN1: u32 = 0x08;
const SPDIFOUT_CTRL0: u32 = 0x0c;
const SPDIFOUT_CTRL0_EN: u32 = BIT(31);
const SPDIFOUT_CTRL0_RST_OUT: u32 = BIT(29);
const SPDIFOUT_CTRL0_RST_IN: u32 = BIT(28);
const SPDIFOUT_CTRL0_USEL: u32 = BIT(26);
const SPDIFOUT_CTRL0_USET: u32 = BIT(25);
const SPDIFOUT_CTRL0_CHSTS_SEL: u32 = BIT(24);
const SPDIFOUT_CTRL0_DATA_SEL: u32 = BIT(20);
const SPDIFOUT_CTRL0_MSB_FIRST: u32 = BIT(19);
const SPDIFOUT_CTRL0_VSEL: u32 = BIT(18);
const SPDIFOUT_CTRL0_VSET: u32 = BIT(17);
const SPDIFOUT_CTRL0_MASK_MASK: u32 = GENMASK(11, 4);
const fn SPDIFOUT_CTRL0_MASK(x: u32) -> u32 {
    x << 4
}
const SPDIFOUT_CTRL1: u32 = 0x10;
const SPDIFOUT_CTRL1_MSB_POS_MASK: u32 = GENMASK(12, 8);
const fn SPDIFOUT_CTRL1_MSB_POS(x: u32) -> u32 {
    x << 8
}
const SPDIFOUT_CTRL1_TYPE_MASK: u32 = GENMASK(6, 4);
const fn SPDIFOUT_CTRL1_TYPE(x: u32) -> u32 {
    x << 4
}
const SPDIFOUT_PREAMB: u32 = 0x14;
const SPDIFOUT_SWAP: u32 = 0x18;
const SPDIFOUT_CHSTS0: u32 = 0x1c;
const SPDIFOUT_CHSTS1: u32 = 0x20;
const SPDIFOUT_CHSTS2: u32 = 0x24;
const SPDIFOUT_CHSTS3: u32 = 0x28;
const SPDIFOUT_CHSTS4: u32 = 0x2c;
const SPDIFOUT_CHSTS5: u32 = 0x30;
const SPDIFOUT_CHSTS6: u32 = 0x34;
const SPDIFOUT_CHSTS7: u32 = 0x38;
const SPDIFOUT_CHSTS8: u32 = 0x3c;
const SPDIFOUT_CHSTS9: u32 = 0x40;
const SPDIFOUT_CHSTSA: u32 = 0x44;
const SPDIFOUT_CHSTSB: u32 = 0x48;
const SPDIFOUT_MUTE_VAL: u32 = 0x4c;

const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const GFP_KERNEL: u32 = 0;
const SND_SOC_NOPM: i32 = 0;

const SNDRV_PCM_TRIGGER_START: i32 = 0;
const SNDRV_PCM_TRIGGER_RESUME: i32 = 1;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: i32 = 2;
const SNDRV_PCM_TRIGGER_STOP: i32 = 3;
const SNDRV_PCM_TRIGGER_SUSPEND: i32 = 4;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: i32 = 5;

const SNDRV_PCM_RATE_32000: u32 = 1 << 0;
const SNDRV_PCM_RATE_44100: u32 = 1 << 1;
const SNDRV_PCM_RATE_48000: u32 = 1 << 2;
const SNDRV_PCM_RATE_88200: u32 = 1 << 3;
const SNDRV_PCM_RATE_96000: u32 = 1 << 4;
const SNDRV_PCM_RATE_176400: u32 = 1 << 5;
const SNDRV_PCM_RATE_192000: u32 = 1 << 6;

const SNDRV_PCM_FMTBIT_S8: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 1;
const SNDRV_PCM_FMTBIT_S20_LE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 3;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk {
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
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai {
    dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct platform_device {
    dev: device,
}

#[repr(C)]
struct axg_spdifout {
    map: *mut regmap,
    mclk: *mut clk,
    pclk: *mut clk,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, i32, *mut snd_soc_dai) -> i32>,
    mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, i32, i32) -> i32>,
    hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> i32>,
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> i32>,
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    no_capture_mute: i32,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    stream_name: *const i8,
    channels_min: u32,
    channels_max: u32,
    rates: u32,
    formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    name: *const i8,
    playback: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
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
    sink: *const i8,
    control: *const i8,
    source: *const i8,
}
#[repr(C)]
pub struct soc_enum {
    reg: u32,
    shift_l: u32,
    items: u32,
    texts: *const *const i8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_OFF = 0,
    SND_SOC_BIAS_STANDBY = 1,
    SND_SOC_BIAS_PREPARE = 2,
    SND_SOC_BIAS_ON = 3,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    controls: *const snd_kcontrol_new,
    num_controls: u32,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: u32,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: u32,
    set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> i32>,
    legacy_dai_naming: i32,
}

#[repr(C)]
pub struct regmap_config {
    reg_bits: u32,
    val_bits: u32,
    reg_stride: u32,
    max_register: u32,
}

#[repr(C)]
pub struct of_device_id {
    compatible: *const i8,
}

#[repr(C)]
pub struct device_driver {
    name: *const i8,
    of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    driver: device_driver,
}

unsafe extern "C" {
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> i32;
    fn regmap_get_reg_stride(map: *mut regmap) -> u32;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut core::ffi::c_void;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut core::ffi::c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_pcm_create_iec958_consumer_hw_params(
        params: *mut snd_pcm_hw_params,
        cs: *mut u8,
        len: i32,
    ) -> i32;
    fn params_channels(params: *mut snd_pcm_hw_params) -> u32;
    fn params_physical_width(params: *mut snd_pcm_hw_params) -> u32;
    fn params_width(params: *mut snd_pcm_hw_params) -> u32;
    fn params_rate(params: *mut snd_pcm_hw_params) -> u32;
    fn clk_set_rate(clk: *mut clk, rate: u32) -> i32;
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn clk_disable_unprepare(clk: *mut clk);
    fn dev_err(dev: *mut device, fmt: *const i8, ...) -> i32;
    fn dev_err_probe(dev: *mut device, err: isize, fmt: *const i8, ...) -> i32;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut core::ffi::c_void);
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: u32) -> *mut core::ffi::c_void;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut core::ffi::c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_clk_get(dev: *mut device, id: *const i8) -> *mut clk;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: i32,
    ) -> i32;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> isize;
}

unsafe fn axg_spdifout_enable(map: *mut regmap) {
    /* Apply both reset */
    regmap_update_bits(
        map,
        SPDIFOUT_CTRL0,
        SPDIFOUT_CTRL0_RST_OUT | SPDIFOUT_CTRL0_RST_IN,
        0,
    );

    /* Clear out reset before in reset */
    regmap_update_bits(
        map,
        SPDIFOUT_CTRL0,
        SPDIFOUT_CTRL0_RST_OUT,
        SPDIFOUT_CTRL0_RST_OUT,
    );
    regmap_update_bits(
        map,
        SPDIFOUT_CTRL0,
        SPDIFOUT_CTRL0_RST_IN,
        SPDIFOUT_CTRL0_RST_IN,
    );

    /* Enable spdifout */
    regmap_update_bits(map, SPDIFOUT_CTRL0, SPDIFOUT_CTRL0_EN, SPDIFOUT_CTRL0_EN);
}

unsafe fn axg_spdifout_disable(map: *mut regmap) {
    regmap_update_bits(map, SPDIFOUT_CTRL0, SPDIFOUT_CTRL0_EN, 0);
}

unsafe extern "C" fn axg_spdifout_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: i32,
    dai: *mut snd_soc_dai,
) -> i32 {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut axg_spdifout;

    match cmd {
        SNDRV_PCM_TRIGGER_START
        | SNDRV_PCM_TRIGGER_RESUME
        | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            axg_spdifout_enable((*priv_).map);
            0
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            axg_spdifout_disable((*priv_).map);
            0
        }
        _ => -EINVAL,
    }
}

unsafe extern "C" fn axg_spdifout_mute(
    dai: *mut snd_soc_dai,
    mute: i32,
    _direction: i32,
) -> i32 {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut axg_spdifout;

    /* Use spdif valid bit to perform digital mute */
    regmap_update_bits(
        (*priv_).map,
        SPDIFOUT_CTRL0,
        SPDIFOUT_CTRL0_VSET,
        if mute != 0 { SPDIFOUT_CTRL0_VSET } else { 0 },
    );

    0
}

unsafe fn axg_spdifout_sample_fmt(
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> i32 {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut axg_spdifout;
    let mut val: u32;

    /* Set the samples spdifout will pull from the FIFO */
    match params_channels(params) {
        1 => {
            val = SPDIFOUT_CTRL0_MASK(0x1);
        }
        2 => {
            val = SPDIFOUT_CTRL0_MASK(0x3);
        }
        _ => {
            dev_err(
                (*dai).dev,
                c"too many channels for spdif dai: %u\n".as_ptr(),
                params_channels(params),
            );
            return -EINVAL;
        }
    }

    regmap_update_bits((*priv_).map, SPDIFOUT_CTRL0, SPDIFOUT_CTRL0_MASK_MASK, val);

    /* FIFO data are arranged in chunks of 64bits */
    match params_physical_width(params) {
        8 => {
            /* 8 samples of 8 bits */
            val = SPDIFOUT_CTRL1_TYPE(0);
        }
        16 => {
            /* 4 samples of 16 bits - right justified */
            val = SPDIFOUT_CTRL1_TYPE(2);
        }
        32 => {
            /* 2 samples of 32 bits - right justified */
            val = SPDIFOUT_CTRL1_TYPE(4);
        }
        _ => {
            dev_err(
                (*dai).dev,
                c"Unsupported physical width: %u\n".as_ptr(),
                params_physical_width(params),
            );
            return -EINVAL;
        }
    }

    /* Position of the MSB in FIFO samples */
    val |= SPDIFOUT_CTRL1_MSB_POS(params_width(params) - 1);

    regmap_update_bits(
        (*priv_).map,
        SPDIFOUT_CTRL1,
        SPDIFOUT_CTRL1_MSB_POS_MASK | SPDIFOUT_CTRL1_TYPE_MASK,
        val,
    );

    regmap_update_bits(
        (*priv_).map,
        SPDIFOUT_CTRL0,
        SPDIFOUT_CTRL0_MSB_FIRST | SPDIFOUT_CTRL0_DATA_SEL,
        0,
    );

    0
}

unsafe fn axg_spdifout_set_chsts(
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> i32 {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut axg_spdifout;
    let mut offset: u32;
    let ret: i32;
    let mut cs: [u8; 4] = [0; 4];
    let val: u32;

    ret = snd_pcm_create_iec958_consumer_hw_params(params, cs.as_mut_ptr(), 4);
    if ret < 0 {
        dev_err(
            (*dai).dev,
            c"Creating IEC958 channel status failed %d\n".as_ptr(),
            ret,
        );
        return ret;
    }
    val = cs[0] as u32 | (cs[1] as u32) << 8 | (cs[2] as u32) << 16 | (cs[3] as u32) << 24;

    /* Setup channel status A bits [31 - 0]*/
    regmap_write((*priv_).map, SPDIFOUT_CHSTS0, val);

    /* Clear channel status A bits [191 - 32] */
    offset = SPDIFOUT_CHSTS1;
    while offset <= SPDIFOUT_CHSTS5 {
        regmap_write((*priv_).map, offset, 0);
        offset += regmap_get_reg_stride((*priv_).map);
    }

    /* Setup channel status B bits [31 - 0]*/
    regmap_write((*priv_).map, SPDIFOUT_CHSTS6, val);

    /* Clear channel status B bits [191 - 32] */
    offset = SPDIFOUT_CHSTS7;
    while offset <= SPDIFOUT_CHSTSB {
        regmap_write((*priv_).map, offset, 0);
        offset += regmap_get_reg_stride((*priv_).map);
    }

    0
}

unsafe extern "C" fn axg_spdifout_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> i32 {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut axg_spdifout;
    let rate: u32 = params_rate(params);
    let mut ret: i32;

    /* 2 * 32bits per subframe * 2 channels = 128 */
    ret = clk_set_rate((*priv_).mclk, rate * 128);
    if ret != 0 {
        dev_err((*dai).dev, c"failed to set spdif clock\n".as_ptr());
        return ret;
    }

    ret = axg_spdifout_sample_fmt(params, dai);
    if ret != 0 {
        dev_err((*dai).dev, c"failed to setup sample format\n".as_ptr());
        return ret;
    }

    ret = axg_spdifout_set_chsts(params, dai);
    if ret != 0 {
        dev_err(
            (*dai).dev,
            c"failed to setup channel status words\n".as_ptr(),
        );
        return ret;
    }

    0
}

unsafe extern "C" fn axg_spdifout_startup(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> i32 {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut axg_spdifout;
    let ret: i32;

    /* Clock the spdif output block */
    ret = clk_prepare_enable((*priv_).pclk);
    if ret != 0 {
        dev_err((*dai).dev, c"failed to enable pclk\n".as_ptr());
        return ret;
    }

    /* Make sure the block is initially stopped */
    axg_spdifout_disable((*priv_).map);

    /* Insert data from bit 27 lsb first */
    regmap_update_bits(
        (*priv_).map,
        SPDIFOUT_CTRL0,
        SPDIFOUT_CTRL0_MSB_FIRST | SPDIFOUT_CTRL0_DATA_SEL,
        0,
    );

    /* Manual control of V, C and U, U = 0 */
    regmap_update_bits(
        (*priv_).map,
        SPDIFOUT_CTRL0,
        SPDIFOUT_CTRL0_CHSTS_SEL | SPDIFOUT_CTRL0_VSEL | SPDIFOUT_CTRL0_USEL | SPDIFOUT_CTRL0_USET,
        0,
    );

    /* Static SWAP configuration ATM */
    regmap_write((*priv_).map, SPDIFOUT_SWAP, 0x10);

    0
}

unsafe extern "C" fn axg_spdifout_shutdown(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut axg_spdifout;

    clk_disable_unprepare((*priv_).pclk);
}

static axg_spdifout_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    trigger: Some(axg_spdifout_trigger),
    mute_stream: Some(axg_spdifout_mute),
    hw_params: Some(axg_spdifout_hw_params),
    startup: Some(axg_spdifout_startup),
    shutdown: Some(axg_spdifout_shutdown),
    no_capture_mute: 1,
};

static mut axg_spdifout_dai_drv: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"SPDIF Output".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_32000
            | SNDRV_PCM_RATE_44100
            | SNDRV_PCM_RATE_48000
            | SNDRV_PCM_RATE_88200
            | SNDRV_PCM_RATE_96000
            | SNDRV_PCM_RATE_176400
            | SNDRV_PCM_RATE_192000,
        formats: SNDRV_PCM_FMTBIT_S8
            | SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S20_LE
            | SNDRV_PCM_FMTBIT_S24_LE,
    },
    ops: &axg_spdifout_ops,
}];

static spdifout_sel_texts: [*const i8; 3] = [
    c"IN 0".as_ptr(),
    c"IN 1".as_ptr(),
    c"IN 2".as_ptr(),
];

// C macro expansion: SOC_ENUM_SINGLE_DECL(axg_spdifout_sel_enum, SPDIFOUT_CTRL1, 24,
//                                         spdifout_sel_texts)
static axg_spdifout_sel_enum: soc_enum = soc_enum {
    reg: SPDIFOUT_CTRL1,
    shift_l: 24,
    items: spdifout_sel_texts.len() as u32,
    texts: spdifout_sel_texts.as_ptr(),
};

// C macro expansion: SOC_DAPM_ENUM("Input Source", axg_spdifout_sel_enum)
static axg_spdifout_in_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

// C macro expansion: SND_SOC_DAPM_AIF_IN and SND_SOC_DAPM_MUX initializers.
static axg_spdifout_dapm_widgets: [snd_soc_dapm_widget; 4] = [
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];

static axg_spdifout_dapm_routes: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route {
        sink: c"SRC SEL".as_ptr(),
        control: c"IN 0".as_ptr(),
        source: c"IN 0".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"SRC SEL".as_ptr(),
        control: c"IN 1".as_ptr(),
        source: c"IN 1".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"SRC SEL".as_ptr(),
        control: c"IN 2".as_ptr(),
        source: c"IN 2".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Playback".as_ptr(),
        control: core::ptr::null(),
        source: c"SRC SEL".as_ptr(),
    },
];

// C macro expansions: SOC_DOUBLE and SOC_SINGLE control initializers.
static axg_spdifout_controls: [snd_kcontrol_new; 4] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

unsafe extern "C" fn axg_spdifout_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> i32 {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut axg_spdifout;
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);
    let now: snd_soc_bias_level = snd_soc_dapm_get_bias_level(dapm);
    let mut ret: i32 = 0;

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {
            if now == snd_soc_bias_level::SND_SOC_BIAS_STANDBY {
                ret = clk_prepare_enable((*priv_).mclk);
            }
        }
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            if now == snd_soc_bias_level::SND_SOC_BIAS_PREPARE {
                clk_disable_unprepare((*priv_).mclk);
            }
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF | snd_soc_bias_level::SND_SOC_BIAS_ON => {}
    }

    ret
}

static axg_spdifout_component_drv: snd_soc_component_driver = snd_soc_component_driver {
    controls: axg_spdifout_controls.as_ptr(),
    num_controls: axg_spdifout_controls.len() as u32,
    dapm_widgets: axg_spdifout_dapm_widgets.as_ptr(),
    num_dapm_widgets: axg_spdifout_dapm_widgets.len() as u32,
    dapm_routes: axg_spdifout_dapm_routes.as_ptr(),
    num_dapm_routes: axg_spdifout_dapm_routes.len() as u32,
    set_bias_level: Some(axg_spdifout_set_bias_level),
    legacy_dai_naming: 1,
};

static axg_spdifout_regmap_cfg: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 32,
    reg_stride: 4,
    max_register: SPDIFOUT_MUTE_VAL,
};

static axg_spdifout_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"amlogic,axg-spdifout".as_ptr(),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, axg_spdifout_of_match);

unsafe extern "C" fn axg_spdifout_probe(pdev: *mut platform_device) -> i32 {
    let dev: *mut device = &mut (*pdev).dev;
    let priv_: *mut axg_spdifout;
    let regs: *mut core::ffi::c_void;

    priv_ = devm_kzalloc(dev, core::mem::size_of::<axg_spdifout>(), GFP_KERNEL) as *mut axg_spdifout;
    if priv_.is_null() {
        return -ENOMEM;
    }
    platform_set_drvdata(pdev, priv_ as *mut core::ffi::c_void);

    regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(regs) {
        return PTR_ERR(regs) as i32;
    }

    (*priv_).map = devm_regmap_init_mmio(dev, regs, &axg_spdifout_regmap_cfg);
    if IS_ERR((*priv_).map as *const core::ffi::c_void) {
        dev_err(
            dev,
            c"failed to init regmap: %ld\n".as_ptr(),
            PTR_ERR((*priv_).map as *const core::ffi::c_void),
        );
        return PTR_ERR((*priv_).map as *const core::ffi::c_void) as i32;
    }

    (*priv_).pclk = devm_clk_get(dev, c"pclk".as_ptr());
    if IS_ERR((*priv_).pclk as *const core::ffi::c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*priv_).pclk as *const core::ffi::c_void),
            c"failed to get pclk\n".as_ptr(),
        );
    }

    (*priv_).mclk = devm_clk_get(dev, c"mclk".as_ptr());
    if IS_ERR((*priv_).mclk as *const core::ffi::c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*priv_).mclk as *const core::ffi::c_void),
            c"failed to get mclk\n".as_ptr(),
        );
    }

    devm_snd_soc_register_component(
        dev,
        &axg_spdifout_component_drv,
        axg_spdifout_dai_drv.as_mut_ptr(),
        axg_spdifout_dai_drv.len() as i32,
    )
}

static mut axg_spdifout_pdrv: platform_driver = platform_driver {
    probe: Some(axg_spdifout_probe),
    driver: device_driver {
        name: c"axg-spdifout".as_ptr(),
        of_match_table: axg_spdifout_of_match.as_ptr(),
    },
};
// module_platform_driver(axg_spdifout_pdrv);

// MODULE_DESCRIPTION("Amlogic AXG SPDIF Output driver");
// MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
