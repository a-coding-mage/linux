// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2020 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

// Dependencies originally supplied by:
// <linux/bitfield.h>, <linux/clk.h>, <sound/pcm_params.h>,
// <sound/soc.h>, <sound/soc-dai.h>, "aiu.h", "gx-formatter.h",
// and "gx-interface.h".

use core::ffi::{c_int, c_uint, c_void};
use core::ptr;

type bool_ = bool;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

const fn BIT(n: c_uint) -> c_uint {
    1u32 << n
}

const fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    (!0u32 << l) & (!0u32 >> (31 - h))
}

fn __ffs(x: c_uint) -> c_uint {
    x.trailing_zeros()
}

fn FIELD_PREP(mask: c_uint, val: c_uint) -> c_uint {
    (val << mask.trailing_zeros()) & mask
}

fn DIV_ROUND_CLOSEST(x: c_uint, divisor: c_uint) -> c_uint {
    (x + divisor / 2) / divisor
}

fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_interval {
    pub min: c_uint,
    pub max: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_rule {
    pub var: c_uint,
    pub private: *mut c_void,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
}

#[repr(C)]
pub struct gx_iface {
    pub mclk: *mut clk,
    pub mclk_rate: c_uint,
    pub fmt: c_uint,
}

#[repr(C)]
pub struct gx_stream {
    pub iface: *mut gx_iface,
    pub clk_enabled: bool_,
    pub physical_width: c_int,
    pub width: c_int,
    pub channels: c_uint,
}

#[repr(C)]
pub struct aiu_clk {
    pub clk: *mut clk,
}

#[repr(C)]
pub struct aiu_platform {
    pub has_clk_ctrl_more_i2s_div: bool_,
}

#[repr(C)]
pub struct aiu_i2s {
    pub iface: gx_iface,
    pub clks: [aiu_clk; 4],
}

#[repr(C)]
pub struct aiu {
    pub i2s: aiu_i2s,
    pub platform: *mut aiu_platform,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub list: *const c_uint,
    pub count: c_uint,
    pub mask: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
}

unsafe extern "C" {
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_dai_get_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
    ) -> *mut gx_stream;
    fn snd_soc_dai_dma_data_get(dai: *mut snd_soc_dai, stream: c_int) -> *mut gx_stream;
    fn snd_soc_dai_dma_data_set(dai: *mut snd_soc_dai, stream: c_int, data: *mut gx_stream);
    fn snd_soc_dai_get_widget(dai: *mut snd_soc_dai, stream: c_int) -> *mut c_void;
    fn snd_soc_dai_active(dai: *mut snd_soc_dai) -> c_int;

    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_physical_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_uint) -> *mut snd_interval;
    fn snd_interval_any(i: *mut snd_interval);
    fn snd_interval_single(i: *mut snd_interval) -> bool_;
    fn snd_interval_refine(i: *mut snd_interval, v: *mut snd_interval) -> c_int;
    fn snd_pcm_hw_constraint_list(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_uint,
        l: *const snd_pcm_hw_constraint_list,
    ) -> c_int;
    fn snd_pcm_hw_rule_add(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_uint,
        func: unsafe extern "C" fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> c_int,
        private: *mut gx_stream,
        ...
    ) -> c_int;

    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn clk_get_rate(clk: *mut clk) -> c_uint;

    fn gx_stream_start(ts: *mut gx_stream) -> c_int;
    fn gx_stream_stop(ts: *mut gx_stream);
    fn gx_stream_alloc(iface: *mut gx_iface) -> *mut gx_stream;
    fn gx_stream_free(ts: *mut gx_stream);

    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn WARN_ON(condition: bool_) -> bool_;
}

unsafe extern "C" {
    static AIU_CLK_CTRL: c_uint;
    static AIU_CLK_CTRL_MORE: c_uint;
    static AIU_CODEC_DAC_LRCLK_CTRL: c_uint;
    static AIU_I2S_SOURCE_DESC: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_BP_FP: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_CLOCK_IN: c_int;
    static SNDRV_PCM_HW_PARAM_CHANNELS: c_uint;
    static SNDRV_PCM_HW_PARAM_SAMPLE_BITS: c_uint;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
}

const PCLK: usize = 0;
const MIXER: usize = 1;
const AOCLK: usize = 2;
const MCLK: usize = 3;

const AIU_I2S_SOURCE_DESC_MODE_SPLIT: c_uint = BIT(11);

const AIU_CLK_CTRL_I2S_DIV_EN: c_uint = BIT(0);
const AIU_CLK_CTRL_I2S_DIV: c_uint = GENMASK(3, 2);
const AIU_CLK_CTRL_AOCLK_INVERT: c_uint = BIT(6);
const AIU_CLK_CTRL_LRCLK_INVERT: c_uint = BIT(7);
const AIU_CLK_CTRL_LRCLK_SKEW: c_uint = GENMASK(9, 8);
const AIU_CLK_CTRL_MORE_HDMI_AMCLK: c_uint = BIT(6);
const AIU_CLK_CTRL_MORE_I2S_DIV: c_uint = GENMASK(5, 0);
const AIU_CODEC_DAC_LRCLK_CTRL_DIV: c_uint = GENMASK(11, 0);

unsafe extern "C" fn aiu_encoder_i2s_divider_enable(
    component: *mut snd_soc_component,
    enable: bool_,
) {
    unsafe {
        snd_soc_component_update_bits(
            component,
            AIU_CLK_CTRL,
            AIU_CLK_CTRL_I2S_DIV_EN,
            if enable { AIU_CLK_CTRL_I2S_DIV_EN } else { 0 },
        );
    }
}

unsafe extern "C" fn aiu_encoder_i2s_set_legacy_div(
    component: *mut snd_soc_component,
    _params: *mut snd_pcm_hw_params,
    bs: c_uint,
) -> c_int {
    match bs {
        1 | 2 | 4 | 8 => {
            /* These are the only valid legacy dividers */
        }
        _ => unsafe {
            dev_err((*component).dev, c"Unsupported i2s divider: %u\n".as_ptr() as *const u8, bs);
            return -EINVAL;
        },
    }

    unsafe {
        snd_soc_component_update_bits(
            component,
            AIU_CLK_CTRL,
            AIU_CLK_CTRL_I2S_DIV,
            FIELD_PREP(AIU_CLK_CTRL_I2S_DIV, __ffs(bs)),
        );

        snd_soc_component_update_bits(
            component,
            AIU_CLK_CTRL_MORE,
            AIU_CLK_CTRL_MORE_I2S_DIV,
            FIELD_PREP(AIU_CLK_CTRL_MORE_I2S_DIV, 0),
        );
    }

    0
}

/*
 * Return true if the given combination of channels and sample width requires
 * the bs quirk. Return false otherwise.
 */
fn aiu_encoder_is_bs_quirk(channels: c_uint, width: c_int) -> bool_ {
    (channels == 8) && (width == 16)
}

unsafe extern "C" fn aiu_encoder_check_bs_quirk(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let other_stream = unsafe { snd_soc_dai_dma_data_get(dai, !(*substream).stream) };

    /* Nothing to do if the other stream doesn't exist or it's not configured yet. */
    if other_stream.is_null() || unsafe { (*other_stream).channels } == 0 {
        return 0;
    }

    if unsafe {
        aiu_encoder_is_bs_quirk((*other_stream).channels, (*other_stream).width)
            != aiu_encoder_is_bs_quirk(params_channels(params), params_width(params))
    } {
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn aiu_encoder_i2s_set_more_div(
    component: *mut snd_soc_component,
    params: *mut snd_pcm_hw_params,
    mut bs: c_uint,
) -> c_int {
    /*
     * NOTE: this HW is odd.
     * In most configuration, the i2s divider is 'mclk / blck'.
     * However, in 16 bits - 8ch mode, this factor needs to be
     * increased by 50% to get the correct output rate.
     * No idea why !
     */
    if unsafe { aiu_encoder_is_bs_quirk(params_channels(params), params_width(params)) } {
        if bs % 2 != 0 {
            unsafe {
                dev_err(
                    (*component).dev,
                    c"Cannot increase i2s divider by 50%%\n".as_ptr() as *const u8,
                );
            }
            return -EINVAL;
        }
        bs += bs / 2;
    }

    /* Use CLK_MORE for mclk to bclk divider */
    unsafe {
        snd_soc_component_update_bits(
            component,
            AIU_CLK_CTRL,
            AIU_CLK_CTRL_I2S_DIV,
            FIELD_PREP(AIU_CLK_CTRL_I2S_DIV, 0),
        );

        snd_soc_component_update_bits(
            component,
            AIU_CLK_CTRL_MORE,
            AIU_CLK_CTRL_MORE_I2S_DIV,
            FIELD_PREP(AIU_CLK_CTRL_MORE_I2S_DIV, bs - 1),
        );
    }

    0
}

unsafe extern "C" fn aiu_encoder_i2s_set_clocks(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = unsafe { (*dai).component };
    let aiu = unsafe { snd_soc_component_get_drvdata(component) as *mut aiu };
    let iface = unsafe { &mut (*aiu).i2s.iface as *mut gx_iface };
    let srate = unsafe { params_rate(params) };
    let mut fs: c_uint;
    let bs: c_uint;
    let ret: c_int;

    /* Get the oversampling factor */
    fs = unsafe { DIV_ROUND_CLOSEST((*iface).mclk_rate, srate) };

    if (fs % 64 != 0) || (fs == 0) {
        return -EINVAL;
    }

    /* Set bclk to lrlck ratio */
    unsafe {
        snd_soc_component_update_bits(
            component,
            AIU_CODEC_DAC_LRCLK_CTRL,
            AIU_CODEC_DAC_LRCLK_CTRL_DIV,
            FIELD_PREP(AIU_CODEC_DAC_LRCLK_CTRL_DIV, 64 - 1),
        );
    }

    bs = fs / 64;

    if unsafe { (*(*aiu).platform).has_clk_ctrl_more_i2s_div } {
        /*
         * The hw rules added in startup() make this unreachable in the
         * sequential case, but both streams may be refined concurrently
         * before either commits its config, since only ops->hw_params
         * runs under the card's pcm_mutex. Re-check against the committed
         * state of the other stream, which is stable under that mutex.
         */
        if unsafe { aiu_encoder_check_bs_quirk(substream, params, dai) } != 0 {
            unsafe {
                dev_err(
                    (*dai).dev,
                    c"bclk requirements incompatible with other stream\n".as_ptr() as *const u8,
                );
            }
            return -EINVAL;
        }
        ret = unsafe { aiu_encoder_i2s_set_more_div(component, params, bs) };
    } else {
        ret = unsafe { aiu_encoder_i2s_set_legacy_div(component, params, bs) };
    }

    if ret != 0 {
        return ret;
    }

    /* Make sure amclk is used for HDMI i2s as well */
    unsafe {
        snd_soc_component_update_bits(
            component,
            AIU_CLK_CTRL_MORE,
            AIU_CLK_CTRL_MORE_HDMI_AMCLK,
            AIU_CLK_CTRL_MORE_HDMI_AMCLK,
        );
    }

    0
}

unsafe extern "C" fn aiu_encoder_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let ts = unsafe { snd_soc_dai_get_dma_data(dai, substream) };
    let ret: c_int;

    ret = unsafe { aiu_encoder_i2s_set_clocks(substream, params, dai) };
    if ret != 0 {
        unsafe {
            dev_err((*dai).dev, c"setting i2s clocks failed: %d\n".as_ptr() as *const u8, ret);
        }
        return ret;
    }

    unsafe {
        (*ts).physical_width = params_physical_width(params);
        (*ts).width = params_width(params);
        (*ts).channels = params_channels(params);
    }

    0
}

unsafe extern "C" fn aiu_encoder_i2s_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let ts = unsafe { snd_soc_dai_get_dma_data(dai, substream) };
    let component = unsafe { (*dai).component };
    let ret: c_int;

    if unsafe { (*ts).clk_enabled } {
        return 0;
    }

    ret = unsafe { clk_prepare_enable((*(*ts).iface).mclk) };
    if ret != 0 {
        return ret;
    }

    unsafe {
        (*ts).clk_enabled = true;
    }

    unsafe {
        aiu_encoder_i2s_divider_enable(component, true);
    }

    0
}

unsafe extern "C" fn aiu_encoder_i2s_hw_free(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let ts = unsafe { snd_soc_dai_get_dma_data(dai, substream) };
    let component = unsafe { (*dai).component };

    /*
     * If this is the last substream being closed then disable the i2s
     * clock divider.
     */
    if unsafe { snd_soc_dai_active(dai) } <= 1 {
        unsafe {
            aiu_encoder_i2s_divider_enable(component, false);
        }
    }

    if unsafe { (*ts).clk_enabled } {
        unsafe {
            clk_disable_unprepare((*(*ts).iface).mclk);
            (*ts).clk_enabled = false;
        }
    }

    unsafe {
        (*ts).channels = 0;
        (*ts).width = 0;
        (*ts).physical_width = 0;
    }

    0
}

unsafe extern "C" fn aiu_encoder_i2s_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = unsafe { (*dai).component };
    let aiu = unsafe { snd_soc_component_get_drvdata(component) as *mut aiu };
    let iface = unsafe { &mut (*aiu).i2s.iface as *mut gx_iface };
    let inv = unsafe { fmt & SND_SOC_DAIFMT_INV_MASK };
    let mut val: c_uint = 0;
    let skew: c_uint;

    /* Only CPU Master / Codec Slave supported ATM */
    if unsafe { (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) != SND_SOC_DAIFMT_BP_FP } {
        return -EINVAL;
    }

    if unsafe { inv == SND_SOC_DAIFMT_NB_IF || inv == SND_SOC_DAIFMT_IB_IF } {
        val |= AIU_CLK_CTRL_LRCLK_INVERT;
    }

    /*
     * The SoC changes data on the rising edge of the bitclock
     * so an inversion of the bitclock is required in normal mode
     */
    if unsafe { inv == SND_SOC_DAIFMT_NB_NF || inv == SND_SOC_DAIFMT_NB_IF } {
        val |= AIU_CLK_CTRL_AOCLK_INVERT;
    }

    /* Signal skew */
    match unsafe { fmt & SND_SOC_DAIFMT_FORMAT_MASK } {
        x if unsafe { x == SND_SOC_DAIFMT_I2S } => {
            /* Invert sample clock for i2s */
            val ^= AIU_CLK_CTRL_LRCLK_INVERT;
            skew = 1;
        }
        x if unsafe { x == SND_SOC_DAIFMT_LEFT_J } => {
            skew = 0;
        }
        _ => unsafe {
            dev_err((*dai).dev, c"unsupported dai format\n".as_ptr() as *const u8);
            return -EINVAL;
        },
    }

    unsafe {
        (*iface).fmt = fmt;

        val |= FIELD_PREP(AIU_CLK_CTRL_LRCLK_SKEW, skew);
        snd_soc_component_update_bits(
            component,
            AIU_CLK_CTRL,
            AIU_CLK_CTRL_LRCLK_INVERT | AIU_CLK_CTRL_AOCLK_INVERT | AIU_CLK_CTRL_LRCLK_SKEW,
            val,
        );
    }

    0
}

unsafe extern "C" fn aiu_encoder_i2s_set_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    let aiu = unsafe { snd_soc_component_get_drvdata((*dai).component) as *mut aiu };
    let iface = unsafe { &mut (*aiu).i2s.iface as *mut gx_iface };
    let ret: c_int;

    if unsafe { WARN_ON(clk_id != 0) } {
        return -EINVAL;
    }

    if unsafe { dir == SND_SOC_CLOCK_IN } {
        return 0;
    }

    ret = unsafe { clk_set_rate((*iface).mclk, freq) };
    if ret != 0 {
        unsafe {
            dev_err(
                (*dai).dev,
                c"Failed to set sysclk to %uHz: %d".as_ptr() as *const u8,
                freq,
                ret,
            );
        }
        return ret;
    }

    unsafe {
        (*iface).mclk_rate = freq;
    }

    0
}

static hw_channels: [c_uint; 2] = [2, 8];
static hw_channel_constraints: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    list: hw_channels.as_ptr(),
    count: ARRAY_SIZE(&hw_channels),
    mask: 0,
};

unsafe extern "C" fn aiu_encoder_i2s_pcm_hw_rule(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let other = unsafe { (*rule).private as *mut gx_stream };
    let ch = unsafe { hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS) };
    /*
     * The quirk is technically based on the significant bits whereas here
     * we're using the physical width for simplicity. This works because
     * S16_LE is the only format supported by this encoder that has:
     * significant bits = physical width = 16-bits
     */
    let phys_width = unsafe { hw_param_interval(params, SNDRV_PCM_HW_PARAM_SAMPLE_BITS) };
    let mut new_i = snd_interval { min: 0, max: 0 };

    if unsafe { (*other).channels == 0 } {
        return 0;
    }

    unsafe {
        snd_interval_any(&mut new_i);
    }

    if unsafe { (*rule).var == SNDRV_PCM_HW_PARAM_CHANNELS } {
        if unsafe { aiu_encoder_is_bs_quirk((*other).channels, (*other).width) } {
            new_i.min = 8;
            new_i.max = 8;
        } else if unsafe { snd_interval_single(phys_width) && (*phys_width).min == 16 } {
            new_i.max = 2; /* Force 2ch */
        }
    } else {
        /* SNDRV_PCM_HW_PARAM_SAMPLE_BITS */
        if unsafe { aiu_encoder_is_bs_quirk((*other).channels, (*other).width) } {
            new_i.min = 16;
            new_i.max = 16;
        } else if unsafe { snd_interval_single(ch) && (*ch).min == 8 } {
            new_i.min = 17; /* Request physical width > 16 bits */
        }
    }

    unsafe { snd_interval_refine(hw_param_interval(params, (*rule).var), &mut new_i) }
}

unsafe extern "C" fn aiu_encoder_i2s_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let aiu = unsafe { snd_soc_component_get_drvdata((*dai).component) as *mut aiu };
    let other_stream = unsafe { snd_soc_dai_dma_data_get(dai, !(*substream).stream) };
    let mut ret: c_int;

    /* Make sure the encoder gets either 2 or 8 channels */
    ret = unsafe {
        snd_pcm_hw_constraint_list(
            (*substream).runtime,
            0,
            SNDRV_PCM_HW_PARAM_CHANNELS,
            &hw_channel_constraints,
        )
    };
    if ret != 0 {
        unsafe {
            dev_err((*dai).dev, c"adding channels constraints failed: %d\n".as_ptr() as *const u8, ret);
        }
        return ret;
    }

    /*
     * If DAI supports both playback and capture streams ensure the bs-quirk is
     * handled correctly.
     * This is only valid for GX platforms (has_clk_ctrl_more_i2s_div=true).
     */
    if unsafe { (*(*aiu).platform).has_clk_ctrl_more_i2s_div } && !other_stream.is_null() {
        ret = unsafe {
            snd_pcm_hw_rule_add(
                (*substream).runtime,
                0,
                SNDRV_PCM_HW_PARAM_CHANNELS,
                aiu_encoder_i2s_pcm_hw_rule,
                other_stream,
                SNDRV_PCM_HW_PARAM_CHANNELS,
                SNDRV_PCM_HW_PARAM_SAMPLE_BITS,
                -1i32,
            )
        };
        if ret != 0 {
            return ret;
        }

        ret = unsafe {
            snd_pcm_hw_rule_add(
                (*substream).runtime,
                0,
                SNDRV_PCM_HW_PARAM_SAMPLE_BITS,
                aiu_encoder_i2s_pcm_hw_rule,
                other_stream,
                SNDRV_PCM_HW_PARAM_CHANNELS,
                SNDRV_PCM_HW_PARAM_SAMPLE_BITS,
                -1i32,
            )
        };
        if ret != 0 {
            return ret;
        }
    }

    /*
     * Enable only clocks which are required for the interface internal
     * logic. MCLK is enabled/disabled from the formatter and the I2S
     * divider is enabled/disabled in "hw_params"/"hw_free", respectively.
     */
    ret = unsafe { clk_prepare_enable((*aiu).i2s.clks[PCLK].clk) };
    if ret != 0 {
        unsafe {
            dev_err((*dai).dev, c"failed to enable PCLK: %d\n".as_ptr() as *const u8, ret);
        }
        return ret;
    }
    ret = unsafe { clk_prepare_enable((*aiu).i2s.clks[MIXER].clk) };
    if ret != 0 {
        unsafe {
            dev_err((*dai).dev, c"failed to enable MIXER: %d\n".as_ptr() as *const u8, ret);
            clk_disable_unprepare((*aiu).i2s.clks[PCLK].clk);
        }
        return ret;
    }
    ret = unsafe { clk_prepare_enable((*aiu).i2s.clks[AOCLK].clk) };
    if ret != 0 {
        unsafe {
            dev_err((*dai).dev, c"failed to enable AOCLK: %d\n".as_ptr() as *const u8, ret);
            clk_disable_unprepare((*aiu).i2s.clks[MIXER].clk);
            clk_disable_unprepare((*aiu).i2s.clks[PCLK].clk);
        }
        return ret;
    }

    /*
     * We're always operating in split mode for the playback stream.
     *
     * This setting arguably belong to the 'aiu-formatter', but it's kept
     * here for backward compatibility reason. At reset the I2S encoder
     * operates in normal mode which would only support 8ch, but by default
     * only 2ch are enabled. If a playback stream is started without
     * changing to split mode, then the I2S encoder doesn't consume audio
     * samples and the playback fails.
     * Moving this to 'aiu-formatter' would cause the split mode to be set
     * only when the formatter is enabled, which doesn't happen at boot as
     * the default value for "HDMI CTRL SRC" is "DISABLED".
     */
    ret = unsafe {
        snd_soc_component_update_bits(
            (*dai).component,
            AIU_I2S_SOURCE_DESC,
            AIU_I2S_SOURCE_DESC_MODE_SPLIT,
            AIU_I2S_SOURCE_DESC_MODE_SPLIT,
        )
    };
    if ret < 0 {
        unsafe {
            dev_err(
                (*dai).dev,
                c"failed to update AIU_I2S_SOURCE_DESC: %d".as_ptr() as *const u8,
                ret,
            );
        }
    }

    0
}

unsafe extern "C" fn aiu_encoder_i2s_shutdown(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let aiu = unsafe { snd_soc_component_get_drvdata((*dai).component) as *mut aiu };

    unsafe {
        clk_disable_unprepare((*aiu).i2s.clks[AOCLK].clk);
        clk_disable_unprepare((*aiu).i2s.clks[MIXER].clk);
        clk_disable_unprepare((*aiu).i2s.clks[PCLK].clk);
    }
}

unsafe extern "C" fn aiu_encoder_i2s_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let ts = unsafe { snd_soc_dai_get_dma_data(dai, substream) };
    let ret: c_int;

    if unsafe {
        cmd == SNDRV_PCM_TRIGGER_START
            || cmd == SNDRV_PCM_TRIGGER_RESUME
            || cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE
    } {
        ret = unsafe { gx_stream_start(ts) };
    } else if unsafe {
        cmd == SNDRV_PCM_TRIGGER_SUSPEND
            || cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH
            || cmd == SNDRV_PCM_TRIGGER_STOP
    } {
        unsafe {
            gx_stream_stop(ts);
        }
        ret = 0;
    } else {
        ret = -EINVAL;
    }

    ret
}

unsafe extern "C" fn aiu_encoder_i2s_remove_dai(dai: *mut snd_soc_dai) -> c_int {
    let mut stream: c_int = 0;

    // for_each_pcm_streams(stream)
    while stream < 2 {
        let ts: *mut gx_stream;

        ts = unsafe { snd_soc_dai_dma_data_get(dai, stream) };
        if !ts.is_null() {
            unsafe {
                gx_stream_free(ts);
            }
        }

        unsafe {
            snd_soc_dai_dma_data_set(dai, stream, ptr::null_mut());
        }

        stream += 1;
    }

    0
}

unsafe extern "C" fn aiu_encoder_i2s_probe_dai(dai: *mut snd_soc_dai) -> c_int {
    let aiu = unsafe { snd_soc_dai_get_drvdata(dai) as *mut aiu };
    let iface = unsafe { &mut (*aiu).i2s.iface as *mut gx_iface };
    let mut stream: c_int = 0;

    // for_each_pcm_streams(stream)
    while stream < 2 {
        let ts: *mut gx_stream;

        if unsafe { snd_soc_dai_get_widget(dai, stream).is_null() } {
            stream += 1;
            continue;
        }

        ts = unsafe { gx_stream_alloc(iface) };
        if ts.is_null() {
            unsafe {
                aiu_encoder_i2s_remove_dai(dai);
            }
            return -ENOMEM;
        }
        unsafe {
            snd_soc_dai_dma_data_set(dai, stream, ts);
        }

        stream += 1;
    }

    unsafe {
        (*iface).mclk = (*aiu).i2s.clks[MCLK].clk;
        (*iface).mclk_rate = clk_get_rate((*iface).mclk);
    }

    0
}

#[no_mangle]
pub static aiu_encoder_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(aiu_encoder_i2s_probe_dai),
    remove: Some(aiu_encoder_i2s_remove_dai),
    hw_params: Some(aiu_encoder_i2s_hw_params),
    prepare: Some(aiu_encoder_i2s_prepare),
    hw_free: Some(aiu_encoder_i2s_hw_free),
    set_fmt: Some(aiu_encoder_i2s_set_fmt),
    set_sysclk: Some(aiu_encoder_i2s_set_sysclk),
    startup: Some(aiu_encoder_i2s_startup),
    shutdown: Some(aiu_encoder_i2s_shutdown),
    trigger: Some(aiu_encoder_i2s_trigger),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
