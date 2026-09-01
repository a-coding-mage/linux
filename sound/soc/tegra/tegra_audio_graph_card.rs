// SPDX-License-Identifier: GPL-2.0-only
// SPDX-FileCopyrightText: Copyright (c) 2020-2025 NVIDIA CORPORATION. All rights reserved.
//
// tegra_audio_graph_card.c - Audio Graph based Tegra Machine Driver

// Dependencies from the original C source:
// linux/math64.h, linux/module.h, linux/of.h, linux/platform_device.h,
// sound/graph_card.h, sound/pcm_params.h, sound/soc-dai.h

const MAX_PLLA_OUT0_DIV: u32 = 128;
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const GFP_KERNEL: gfp_t = 0;

type c_char = i8;
type c_int = i32;
type c_uint = u32;
type c_ulong = u64;
type bool_t = bool;
type gfp_t = c_uint;

#[repr(C)]
pub struct clk {
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
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub ops: *const core::ffi::c_void,
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub driver: *mut snd_soc_dai_driver,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub driver_name: *const c_char,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub component_chaining: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream)>,
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct simple_util_priv {
    pub ops: *const snd_soc_ops,
    pub force_dpcm: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const core::ffi::c_void,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const core::ffi::c_void,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
enum srate_type {
    /*
     * Sample rates multiple of 8000 Hz and below are supported:
     * ( 8000, 16000, 32000, 48000, 96000, 192000 Hz )
     */
    x8_RATE = 0,

    /*
     * Sample rates multiple of 11025 Hz and below are supported:
     * ( 11025, 22050, 44100, 88200, 176400 Hz )
     */
    x11_RATE = 1,

    NUM_RATE_TYPE = 2,
}

#[repr(C)]
struct tegra_audio_priv {
    simple: simple_util_priv,
    clk_plla_out0: *mut clk,
    clk_plla: *mut clk,
}

/* Tegra audio chip data */
#[repr(C)]
struct tegra_audio_cdata {
    plla_rates: [c_uint; srate_type::NUM_RATE_TYPE as usize],
    plla_out0_rates: [c_uint; srate_type::NUM_RATE_TYPE as usize],
}

unsafe fn simple_to_tegra_priv(simple: *mut simple_util_priv) -> *mut tegra_audio_priv {
    simple as *mut tegra_audio_priv
}

unsafe extern "C" {
    static snd_soc_pm_ops: core::ffi::c_void;

    fn snd_soc_dai_is_dummy(dai: *mut snd_soc_dai) -> bool_t;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut simple_util_priv;
    fn of_device_get_match_data(dev: *mut device) -> *const core::ffi::c_void;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn div_u64(dividend: u64, divisor: u32) -> u64;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn simple_util_hw_params(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
    ) -> c_int;
    fn simple_util_startup(substream: *mut snd_pcm_substream) -> c_int;
    fn simple_util_shutdown(substream: *mut snd_pcm_substream);
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool_t;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn graph_util_card_probe(card: *mut snd_soc_card) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: gfp_t) -> *mut core::ffi::c_void;
    fn simple_priv_to_card(simple: *mut simple_util_priv) -> *mut snd_soc_card;
    fn audio_graph_parse_of(simple: *mut simple_util_priv, dev: *mut device) -> c_int;
    fn simple_util_remove(pdev: *mut platform_device);
}

static I2S: &[u8] = b"I2S\0";
static DMIC: &[u8] = b"DMIC\0";
static DSPK: &[u8] = b"DSPK\0";
static UNSUPPORTED_SAMPLE_RATE: &[u8] = b"Unsupported sample rate %u\n\0";
static UPDATE_CLOCK_RATES: &[u8] =
    b"Update clock rates: PLLA(= %u Hz) and PLLA_OUT0(= %u Hz)\n\0";
static CANT_SET_PLLA_RATE: &[u8] = b"Can't set plla rate for %u, err: %d\n\0";
static CANT_SET_PLLA_OUT0_RATE: &[u8] = b"Can't set plla_out0 rate %u, err: %d\n\0";
static PLL_A: &[u8] = b"pll_a\0";
static CANT_RETRIEVE_CLK_PLL_A: &[u8] = b"can't retrieve clk pll_a\n\0";
static PLLA_OUT0: &[u8] = b"plla_out0\0";
static CANT_RETRIEVE_CLK_PLLA_OUT0: &[u8] = b"can't retrieve clk plla_out0\n\0";
static GRAPH_UTIL_CARD_PROBE_FAILED: &[u8] = b"graph_util_card_probe failed\n\0";
static TEGRA_APE: &[u8] = b"tegra-ape\0";
static NVIDIA_TEGRA210_AUDIO_GRAPH_CARD: &[u8] = b"nvidia,tegra210-audio-graph-card\0";
static NVIDIA_TEGRA186_AUDIO_GRAPH_CARD: &[u8] = b"nvidia,tegra186-audio-graph-card\0";
static NVIDIA_TEGRA238_AUDIO_GRAPH_CARD: &[u8] = b"nvidia,tegra238-audio-graph-card\0";
static NVIDIA_TEGRA264_AUDIO_GRAPH_CARD: &[u8] = b"nvidia,tegra264-audio-graph-card\0";
static TEGRA_AUDIO_GRAPH_CARD_NAME: &[u8] = b"tegra-audio-graph-card\0";

unsafe fn need_clk_update(dai: *mut snd_soc_dai) -> bool {
    if snd_soc_dai_is_dummy(dai)
        || (*(*dai).driver).ops.is_null()
        || (*(*dai).driver).name.is_null()
    {
        return false;
    }

    if !strstr((*(*dai).driver).name, I2S.as_ptr() as *const c_char).is_null()
        || !strstr((*(*dai).driver).name, DMIC.as_ptr() as *const c_char).is_null()
        || !strstr((*(*dai).driver).name, DSPK.as_ptr() as *const c_char).is_null()
    {
        return true;
    }

    false
}

/* Setup PLL clock as per the given sample rate */
unsafe extern "C" fn tegra_audio_graph_update_pll(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let simple = snd_soc_card_get_drvdata((*rtd).card);
    let priv_ = simple_to_tegra_priv(simple);
    let dev = (*(*rtd).card).dev;
    let data = of_device_get_match_data(dev) as *const tegra_audio_cdata;
    let mut plla_rate: c_uint;
    let mut plla_out0_rate: c_uint;
    let bclk: c_uint;
    let srate: c_uint = params_rate(params);
    let mut err: c_int;

    match srate {
        11025 | 22050 | 44100 | 88200 | 176400 => {
            plla_out0_rate = (*data).plla_out0_rates[srate_type::x11_RATE as usize];
            plla_rate = (*data).plla_rates[srate_type::x11_RATE as usize];
        }
        8000 | 16000 | 32000 | 48000 | 96000 | 192000 => {
            plla_out0_rate = (*data).plla_out0_rates[srate_type::x8_RATE as usize];
            plla_rate = (*data).plla_rates[srate_type::x8_RATE as usize];
        }
        _ => {
            dev_err(
                (*(*rtd).card).dev,
                UNSUPPORTED_SAMPLE_RATE.as_ptr() as *const c_char,
                srate,
            );
            return -EINVAL;
        }
    }

    /*
     * Below is the clock relation:
     *
     *      PLLA
     *        |
     *        |--> PLLA_OUT0
     *                |
     *                |---> I2S modules
     *                |
     *                |---> DMIC modules
     *                |
     *                |---> DSPK modules
     *
     *
     * Default PLLA_OUT0 rate might be too high when I/O is running
     * at minimum PCM configurations. This may result in incorrect
     * clock rates and glitchy audio. The maximum divider is 128
     * and any thing higher than that won't work. Thus reduce PLLA_OUT0
     * to work for lower configurations.
     *
     * This problem is seen for I2S only, as DMIC and DSPK minimum
     * clock requirements are under allowed divider limits.
     */
    bclk = srate
        .wrapping_mul(params_channels(params))
        .wrapping_mul(params_width(params));
    if div_u64(plla_out0_rate as u64, bclk) > MAX_PLLA_OUT0_DIV as u64 {
        plla_out0_rate >>= 1;
    }

    dev_dbg(
        (*(*rtd).card).dev,
        UPDATE_CLOCK_RATES.as_ptr() as *const c_char,
        plla_rate,
        plla_out0_rate,
    );

    /* Set PLLA rate */
    err = clk_set_rate((*priv_).clk_plla, plla_rate as c_ulong);
    if err != 0 {
        dev_err(
            (*(*rtd).card).dev,
            CANT_SET_PLLA_RATE.as_ptr() as *const c_char,
            plla_rate,
            err,
        );
        return err;
    }

    /* Set PLLA_OUT0 rate */
    err = clk_set_rate((*priv_).clk_plla_out0, plla_out0_rate as c_ulong);
    if err != 0 {
        dev_err(
            (*(*rtd).card).dev,
            CANT_SET_PLLA_OUT0_RATE.as_ptr() as *const c_char,
            plla_out0_rate,
            err,
        );
        return err;
    }

    err
}

unsafe extern "C" fn tegra_audio_graph_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut err: c_int;

    if need_clk_update(cpu_dai) {
        err = tegra_audio_graph_update_pll(substream, params);
        if err != 0 {
            return err;
        }
    }

    simple_util_hw_params(substream, params)
}

unsafe extern "C" {
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
}

static tegra_audio_graph_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(simple_util_startup),
    shutdown: Some(simple_util_shutdown),
    hw_params: Some(tegra_audio_graph_hw_params),
};

unsafe extern "C" fn tegra_audio_graph_card_probe(card: *mut snd_soc_card) -> c_int {
    let simple = snd_soc_card_get_drvdata(card);
    let priv_ = simple_to_tegra_priv(simple);
    let ret: c_int;

    (*priv_).clk_plla = devm_clk_get((*card).dev, PLL_A.as_ptr() as *const c_char);
    if IS_ERR((*priv_).clk_plla as *const core::ffi::c_void) {
        return dev_err_probe(
            (*card).dev,
            PTR_ERR((*priv_).clk_plla as *const core::ffi::c_void),
            CANT_RETRIEVE_CLK_PLL_A.as_ptr() as *const c_char,
        );
    }

    (*priv_).clk_plla_out0 = devm_clk_get((*card).dev, PLLA_OUT0.as_ptr() as *const c_char);
    if IS_ERR((*priv_).clk_plla_out0 as *const core::ffi::c_void) {
        return dev_err_probe(
            (*card).dev,
            PTR_ERR((*priv_).clk_plla_out0 as *const core::ffi::c_void),
            CANT_RETRIEVE_CLK_PLLA_OUT0.as_ptr() as *const c_char,
        );
    }

    ret = graph_util_card_probe(card);
    if ret < 0 {
        return dev_err_probe(
            (*card).dev,
            ret,
            GRAPH_UTIL_CARD_PROBE_FAILED.as_ptr() as *const c_char,
        );
    }

    ret
}

unsafe extern "C" fn tegra_audio_graph_probe(pdev: *mut platform_device) -> c_int {
    let priv_: *mut tegra_audio_priv;
    let dev = &mut (*pdev).dev as *mut device;
    let card: *mut snd_soc_card;

    priv_ = devm_kzalloc(dev, core::mem::size_of::<tegra_audio_priv>(), GFP_KERNEL)
        as *mut tegra_audio_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    card = simple_priv_to_card(&mut (*priv_).simple);
    (*card).driver_name = TEGRA_APE.as_ptr() as *const c_char;

    (*card).probe = Some(tegra_audio_graph_card_probe);

    /* audio_graph_parse_of() depends on below */
    (*card).component_chaining = 1;
    (*priv_).simple.ops = &tegra_audio_graph_ops;
    (*priv_).simple.force_dpcm = 1;

    audio_graph_parse_of(&mut (*priv_).simple, dev)
}

static tegra210_data: tegra_audio_cdata = tegra_audio_cdata {
    /* PLLA */
    plla_rates: [368640000, 338688000],
    /* PLLA_OUT0 */
    plla_out0_rates: [49152000, 45158400],
};

static tegra186_data: tegra_audio_cdata = tegra_audio_cdata {
    /* PLLA */
    plla_rates: [245760000, 270950400],
    /* PLLA_OUT0 */
    plla_out0_rates: [49152000, 45158400],
};

static tegra238_data: tegra_audio_cdata = tegra_audio_cdata {
    /* PLLA */
    plla_rates: [1277952000, 1264435200],
    /* PLLA_OUT0 */
    plla_out0_rates: [49152000, 45158400],
};

static tegra264_data: tegra_audio_cdata = tegra_audio_cdata {
    /* PLLA1 */
    plla_rates: [983040000, 993484800],
    /* PLLA1_OUT1 */
    plla_out0_rates: [49152000, 45158400],
};

static graph_of_tegra_match: [of_device_id; 5] = [
    of_device_id {
        compatible: NVIDIA_TEGRA210_AUDIO_GRAPH_CARD.as_ptr() as *const c_char,
        data: &tegra210_data as *const tegra_audio_cdata as *const core::ffi::c_void,
    },
    of_device_id {
        compatible: NVIDIA_TEGRA186_AUDIO_GRAPH_CARD.as_ptr() as *const c_char,
        data: &tegra186_data as *const tegra_audio_cdata as *const core::ffi::c_void,
    },
    of_device_id {
        compatible: NVIDIA_TEGRA238_AUDIO_GRAPH_CARD.as_ptr() as *const c_char,
        data: &tegra238_data as *const tegra_audio_cdata as *const core::ffi::c_void,
    },
    of_device_id {
        compatible: NVIDIA_TEGRA264_AUDIO_GRAPH_CARD.as_ptr() as *const c_char,
        data: &tegra264_data as *const tegra_audio_cdata as *const core::ffi::c_void,
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, graph_of_tegra_match);

static mut tegra_audio_graph_card: platform_driver = platform_driver {
    driver: device_driver {
        name: TEGRA_AUDIO_GRAPH_CARD_NAME.as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops as *const core::ffi::c_void },
        of_match_table: graph_of_tegra_match.as_ptr(),
    },
    probe: Some(tegra_audio_graph_probe),
    remove: Some(simple_util_remove),
};
// module_platform_driver(tegra_audio_graph_card);

// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("ASoC Tegra Audio Graph Sound Card");
// MODULE_AUTHOR("Sameer Pujar <spujar@nvidia.com>");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
