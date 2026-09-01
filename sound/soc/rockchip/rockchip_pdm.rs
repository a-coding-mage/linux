// SPDX-License-Identifier: GPL-2.0-only
/*
 * Rockchip PDM ALSA SoC Digital Audio Interface(DAI)  driver
 *
 * Copyright (C) 2017 Fuzhou Rockchip Electronics Co., Ltd
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

const PDM_DMA_BURST_SIZE: u32 = 8; /* size * width: 8*4 = 32 bytes */
const PDM_SIGNOFF_CLK_RATE: u32 = 100000000;
const PDM_PATH_MAX: usize = 4;

type bool_ = bool;

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct reset_control {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct resource {
    pub start: u64,
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: u64,
    pub addr_width: u32,
    pub maxburst: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum rk_pdm_version {
    RK_PDM_RK3229,
    RK_PDM_RK3308,
    RK_PDM_RV1126,
}

#[repr(C)]
struct rk_pdm_dev {
    dev: *mut device,
    clk: *mut clk,
    hclk: *mut clk,
    regmap: *mut regmap,
    capture_dma_data: snd_dmaengine_dai_dma_data,
    reset: *mut reset_control,
    version: rk_pdm_version,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct rk_pdm_clkref {
    sr: u32,
    clk: u32,
    clk_out: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct rk_pdm_ds_ratio {
    ratio: u32,
    sr: u32,
}

static clkref: [rk_pdm_clkref; 5] = [
    rk_pdm_clkref { sr: 8000, clk: 40960000, clk_out: 2048000 },
    rk_pdm_clkref { sr: 11025, clk: 56448000, clk_out: 2822400 },
    rk_pdm_clkref { sr: 12000, clk: 61440000, clk_out: 3072000 },
    rk_pdm_clkref { sr: 8000, clk: 98304000, clk_out: 2048000 },
    rk_pdm_clkref { sr: 12000, clk: 98304000, clk_out: 3072000 },
];

static ds_ratio: [rk_pdm_ds_ratio; 15] = [
    rk_pdm_ds_ratio { ratio: 0, sr: 192000 },
    rk_pdm_ds_ratio { ratio: 0, sr: 176400 },
    rk_pdm_ds_ratio { ratio: 0, sr: 128000 },
    rk_pdm_ds_ratio { ratio: 1, sr: 96000 },
    rk_pdm_ds_ratio { ratio: 1, sr: 88200 },
    rk_pdm_ds_ratio { ratio: 1, sr: 64000 },
    rk_pdm_ds_ratio { ratio: 2, sr: 48000 },
    rk_pdm_ds_ratio { ratio: 2, sr: 44100 },
    rk_pdm_ds_ratio { ratio: 2, sr: 32000 },
    rk_pdm_ds_ratio { ratio: 3, sr: 24000 },
    rk_pdm_ds_ratio { ratio: 3, sr: 22050 },
    rk_pdm_ds_ratio { ratio: 3, sr: 16000 },
    rk_pdm_ds_ratio { ratio: 4, sr: 12000 },
    rk_pdm_ds_ratio { ratio: 4, sr: 11025 },
    rk_pdm_ds_ratio { ratio: 4, sr: 8000 },
];

extern "C" {
    static PDM_DMA_CTRL: u32;
    static PDM_DMA_RD_MSK: u32;
    static PDM_DMA_RD_EN: u32;
    static PDM_DMA_RD_DIS: u32;
    static PDM_SYSCONFIG: u32;
    static PDM_RX_MASK: u32;
    static PDM_RX_START: u32;
    static PDM_RX_CLR_MASK: u32;
    static PDM_RX_STOP: u32;
    static PDM_RX_CLR_WR: u32;
    static PDM_CTRL0: u32;
    static PDM_CTRL1: u32;
    static PDM_CLK_CTRL: u32;
    static PDM_HPF_CTRL: u32;
    static PDM_FIFO_CTRL: u32;
    static PDM_INT_EN: u32;
    static PDM_INT_CLR: u32;
    static PDM_INT_ST: u32;
    static PDM_DATA_VALID: u32;
    static PDM_RXFIFO_DATA: u32;
    static PDM_VERSION: u32;
    static PDM_FD_NUMERATOR_SFT: u32;
    static PDM_FD_DENOMINATOR_SFT: u32;
    static PDM_FD_NUMERATOR_MSK: u32;
    static PDM_FD_DENOMINATOR_MSK: u32;
    static PDM_CLK_FD_RATIO_40: u32;
    static PDM_CLK_FD_RATIO_35: u32;
    static PDM_CLK_FD_RATIO_MSK: u32;
    static PDM_CIC_RATIO_MSK: u32;
    static PDM_SAMPLERATE_MSK: u32;
    static PDM_DS_RATIO_MSK: u32;
    static PDM_HPF_CF_MSK: u32;
    static PDM_HPF_60HZ: u32;
    static PDM_HPF_LE: u32;
    static PDM_HPF_RE: u32;
    static PDM_CLK_EN: u32;
    static PDM_MODE_MSK: u32;
    static PDM_MODE_LJ: u32;
    static PDM_PATH3_EN: u32;
    static PDM_PATH2_EN: u32;
    static PDM_PATH1_EN: u32;
    static PDM_PATH0_EN: u32;
    static PDM_PATH_MSK: u32;
    static PDM_VDW_MSK: u32;
    static PDM_DMA_RDL_MSK: u32;
    static PDM_CKP_MSK: u32;
    static PDM_CKP_NORMAL: u32;
    static PDM_CKP_INVERTED: u32;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SNDRV_PCM_FORMAT_S8: c_int;
    static SNDRV_PCM_FORMAT_S16_LE: c_int;
    static SNDRV_PCM_FORMAT_S20_3LE: c_int;
    static SNDRV_PCM_FORMAT_S24_LE: c_int;
    static SNDRV_PCM_FORMAT_S32_LE: c_int;
    static SND_SOC_DAIFMT_INV_MASK: u32;
    static SND_SOC_DAIFMT_NB_NF: u32;
    static SND_SOC_DAIFMT_IB_NF: u32;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static SNDRV_PCM_RATE_8000_192000: u32;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S20_3LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static REGCACHE_FLAT: u32;
    static DMA_SLAVE_BUSWIDTH_4_BYTES: u32;
    static GFP_KERNEL: u32;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static ENOENT: c_int;
}

extern "C" {
    fn clk_round_rate(clk: *mut clk, rate: u32) -> u32;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> c_int;
    fn regmap_update_bits_check(
        map: *mut regmap,
        reg: u32,
        mask: u32,
        val: u32,
        change: *mut bool_,
    ) -> c_int;
    fn clk_set_rate(clk: *mut clk, rate: u32) -> c_int;
    fn rational_best_approximation(
        given_numerator: u32,
        given_denominator: u32,
        max_numerator: c_ulong,
        max_denominator: c_ulong,
        best_numerator: *mut c_ulong,
        best_denominator: *mut c_ulong,
    );
    fn reset_control_assert(rstc: *mut reset_control) -> c_int;
    fn reset_control_deassert(rstc: *mut reset_control) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> u32;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> u32;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put(dev: *mut device) -> c_int;
    fn snd_soc_dai_dma_data_set_capture(dai: *mut snd_soc_dai, data: *mut snd_dmaengine_dai_dma_data);
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn of_count_phandle_with_args(np: *mut device_node, list_name: *const c_char, cells_name: *const c_char) -> c_int;
    fn of_property_read_u32_array(np: *mut device_node, propname: *const c_char, out_values: *mut u32, sz: c_int) -> c_int;
    fn device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: u32) -> *mut c_void;
    fn devm_reset_control_get(dev: *mut device, id: *const c_char) -> *mut reset_control;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: u32, res: *mut *mut resource) -> *mut c_void;
    fn devm_regmap_init_mmio(dev: *mut device, regs: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_enabled(dev: *mut device) -> bool_;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn devm_snd_dmaengine_pcm_register(dev: *mut device, config: *const c_void, flags: u32) -> c_int;
    fn pm_runtime_status_suspended(dev: *mut device) -> bool_;
    fn pm_runtime_disable(dev: *mut device);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn PDM_SAMPLERATE(val: u32) -> u32;
    fn PDM_VDW(val: u32) -> u32;
    fn PDM_DMA_RDL(val: u32) -> u32;
    fn PDM_PATH_MASK(i: c_int) -> c_int;
    fn PDM_PATH(i: c_int, path: u32) -> c_int;
}

const fn GENMASK(h: u32, l: u32) -> c_ulong {
    if h >= c_ulong::BITS - 1 && l == 0 {
        !0
    } else {
        (((1 as c_ulong) << (h - l + 1)) - 1) << l
    }
}

unsafe fn get_pdm_clk(pdm: *mut rk_pdm_dev, sr: u32, clk_src: *mut u32, clk_out: *mut u32) -> u32 {
    let mut clk: u32 = 0;
    if sr == 0 {
        return clk;
    }

    let count = clkref.len();
    let mut i = 0;
    while i < count {
        if sr % clkref[i].sr != 0 {
            i += 1;
            continue;
        }
        let div = sr / clkref[i].sr;
        if (div & div.wrapping_sub(1)) == 0 {
            *clk_out = clkref[i].clk_out;
            let rate = clk_round_rate((*pdm).clk, clkref[i].clk);
            if rate != clkref[i].clk {
                i += 1;
                continue;
            }
            clk = clkref[i].clk;
            *clk_src = clkref[i].clk;
            break;
        }
        i += 1;
    }

    if clk == 0 {
        clk = clk_round_rate((*pdm).clk, PDM_SIGNOFF_CLK_RATE);
        *clk_src = clk;
    }
    clk
}

fn get_pdm_ds_ratio(sr: u32) -> u32 {
    let mut ratio: u32 = 0;
    if sr == 0 {
        return ratio;
    }

    let count = ds_ratio.len();
    let mut i = 0;
    while i < count {
        if sr == ds_ratio[i].sr {
            ratio = ds_ratio[i].ratio;
        }
        i += 1;
    }
    ratio
}

fn get_pdm_cic_ratio(clk: u32) -> u32 {
    match clk {
        4096000 | 5644800 | 6144000 => 0,
        2048000 | 2822400 | 3072000 => 1,
        1024000 | 1411200 | 1536000 => 2,
        _ => 1,
    }
}

fn samplerate_to_bit(samplerate: u32) -> u32 {
    match samplerate {
        8000 | 11025 | 12000 => 0,
        16000 | 22050 | 24000 => 1,
        32000 => 2,
        44100 | 48000 => 3,
        64000 | 88200 | 96000 => 4,
        128000 | 176400 | 192000 => 5,
        _ => 1,
    }
}

unsafe fn to_info(dai: *mut snd_soc_dai) -> *mut rk_pdm_dev {
    snd_soc_dai_get_drvdata(dai) as *mut rk_pdm_dev
}

unsafe fn rockchip_pdm_rxctrl(pdm: *mut rk_pdm_dev, on: c_int) {
    if on != 0 {
        regmap_update_bits((*pdm).regmap, PDM_DMA_CTRL, PDM_DMA_RD_MSK, PDM_DMA_RD_EN);
        regmap_update_bits((*pdm).regmap, PDM_SYSCONFIG, PDM_RX_MASK, PDM_RX_START);
    } else {
        regmap_update_bits((*pdm).regmap, PDM_DMA_CTRL, PDM_DMA_RD_MSK, PDM_DMA_RD_DIS);
        regmap_update_bits(
            (*pdm).regmap,
            PDM_SYSCONFIG,
            PDM_RX_MASK | PDM_RX_CLR_MASK,
            PDM_RX_STOP | PDM_RX_CLR_WR,
        );
    }
}

unsafe extern "C" fn rockchip_pdm_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let pdm = to_info(dai);
    let mut val: u32 = 0;
    let mut clk_out: u32 = 0;
    let mut clk_src: u32 = 0;
    let mut m: c_ulong = 0;
    let mut n: c_ulong = 0;
    let mut change: bool_ = false;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        return 0;
    }

    let samplerate = params_rate(params);
    let clk_rate = get_pdm_clk(pdm, samplerate, &mut clk_src, &mut clk_out);
    if clk_rate == 0 {
        return -EINVAL;
    }

    let mut ret = clk_set_rate((*pdm).clk, clk_src);
    if ret != 0 {
        return -EINVAL;
    }

    if (*pdm).version == rk_pdm_version::RK_PDM_RK3308 || (*pdm).version == rk_pdm_version::RK_PDM_RV1126 {
        rational_best_approximation(clk_out, clk_src, GENMASK(16 - 1, 0), GENMASK(16 - 1, 0), &mut m, &mut n);

        val = ((m as u32) << PDM_FD_NUMERATOR_SFT) | ((n as u32) << PDM_FD_DENOMINATOR_SFT);
        regmap_update_bits_check(
            (*pdm).regmap,
            PDM_CTRL1,
            PDM_FD_NUMERATOR_MSK | PDM_FD_DENOMINATOR_MSK,
            val,
            &mut change,
        );
        if change {
            reset_control_assert((*pdm).reset);
            reset_control_deassert((*pdm).reset);
            rockchip_pdm_rxctrl(pdm, 0);
        }
        let clk_div = n / m;
        if clk_div >= 40 {
            val = PDM_CLK_FD_RATIO_40;
        } else if clk_div <= 35 {
            val = PDM_CLK_FD_RATIO_35;
        } else {
            return -EINVAL;
        }
        regmap_update_bits((*pdm).regmap, PDM_CLK_CTRL, PDM_CLK_FD_RATIO_MSK, val);
    }

    if (*pdm).version == rk_pdm_version::RK_PDM_RV1126 {
        val = get_pdm_cic_ratio(clk_out);
        regmap_update_bits((*pdm).regmap, PDM_CLK_CTRL, PDM_CIC_RATIO_MSK, val);
        val = samplerate_to_bit(samplerate);
        regmap_update_bits((*pdm).regmap, PDM_CTRL0, PDM_SAMPLERATE_MSK, PDM_SAMPLERATE(val));
    } else {
        val = get_pdm_ds_ratio(samplerate);
        regmap_update_bits((*pdm).regmap, PDM_CLK_CTRL, PDM_DS_RATIO_MSK, val);
    }

    regmap_update_bits((*pdm).regmap, PDM_HPF_CTRL, PDM_HPF_CF_MSK, PDM_HPF_60HZ);
    regmap_update_bits((*pdm).regmap, PDM_HPF_CTRL, PDM_HPF_LE | PDM_HPF_RE, PDM_HPF_LE | PDM_HPF_RE);
    regmap_update_bits((*pdm).regmap, PDM_CLK_CTRL, PDM_CLK_EN, PDM_CLK_EN);
    if (*pdm).version != rk_pdm_version::RK_PDM_RK3229 {
        regmap_update_bits((*pdm).regmap, PDM_CTRL0, PDM_MODE_MSK, PDM_MODE_LJ);
    }

    val = 0;
    match params_format(params) {
        x if x == SNDRV_PCM_FORMAT_S8 => val |= PDM_VDW(8),
        x if x == SNDRV_PCM_FORMAT_S16_LE => val |= PDM_VDW(16),
        x if x == SNDRV_PCM_FORMAT_S20_3LE => val |= PDM_VDW(20),
        x if x == SNDRV_PCM_FORMAT_S24_LE => val |= PDM_VDW(24),
        x if x == SNDRV_PCM_FORMAT_S32_LE => val |= PDM_VDW(32),
        _ => return -EINVAL,
    }

    match params_channels(params) {
        8 => {
            val |= PDM_PATH3_EN;
            val |= PDM_PATH2_EN;
            val |= PDM_PATH1_EN;
            val |= PDM_PATH0_EN;
        }
        6 => {
            val |= PDM_PATH2_EN;
            val |= PDM_PATH1_EN;
            val |= PDM_PATH0_EN;
        }
        4 => {
            val |= PDM_PATH1_EN;
            val |= PDM_PATH0_EN;
        }
        2 => {
            val |= PDM_PATH0_EN;
        }
        _ => {
            dev_err((*pdm).dev, b"invalid channel: %d\n\0".as_ptr() as *const c_char, params_channels(params));
            return -EINVAL;
        }
    }

    regmap_update_bits((*pdm).regmap, PDM_CTRL0, PDM_PATH_MSK | PDM_VDW_MSK, val);
    /* all channels share the single FIFO */
    regmap_update_bits((*pdm).regmap, PDM_DMA_CTRL, PDM_DMA_RDL_MSK, PDM_DMA_RDL(8 * params_channels(params)));

    0
}

unsafe extern "C" fn rockchip_pdm_set_fmt(cpu_dai: *mut snd_soc_dai, fmt: u32) -> c_int {
    let pdm = to_info(cpu_dai);
    let mask = PDM_CKP_MSK;
    let val: u32;

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => val = PDM_CKP_NORMAL,
        x if x == SND_SOC_DAIFMT_IB_NF => val = PDM_CKP_INVERTED,
        _ => return -EINVAL,
    }

    let ret = pm_runtime_resume_and_get((*cpu_dai).dev);
    if ret != 0 {
        return ret;
    }

    regmap_update_bits((*pdm).regmap, PDM_CLK_CTRL, mask, val);
    pm_runtime_put((*cpu_dai).dev);

    0
}

unsafe extern "C" fn rockchip_pdm_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let pdm = to_info(dai);
    let mut ret: c_int = 0;

    match cmd {
        x if x == SNDRV_PCM_TRIGGER_START || x == SNDRV_PCM_TRIGGER_RESUME || x == SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
                rockchip_pdm_rxctrl(pdm, 1);
            }
        }
        x if x == SNDRV_PCM_TRIGGER_SUSPEND || x == SNDRV_PCM_TRIGGER_STOP || x == SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
                rockchip_pdm_rxctrl(pdm, 0);
            }
        }
        _ => {
            ret = -EINVAL;
        }
    }

    ret
}

unsafe extern "C" fn rockchip_pdm_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let pdm = to_info(dai);

    snd_soc_dai_dma_data_set_capture(dai, &mut (*pdm).capture_dma_data);

    0
}

#[repr(C)]
struct snd_soc_dai_ops {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, u32) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
}

static rockchip_pdm_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(rockchip_pdm_dai_probe),
    set_fmt: Some(rockchip_pdm_set_fmt),
    trigger: Some(rockchip_pdm_trigger),
    hw_params: Some(rockchip_pdm_hw_params),
};

unsafe fn ROCKCHIP_PDM_RATES() -> u32 {
    SNDRV_PCM_RATE_8000_192000
}
unsafe fn ROCKCHIP_PDM_FORMATS() -> u64 {
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: u32,
    channels_max: u32,
    rates: u32,
    formats: u64,
}

#[repr(C)]
struct snd_soc_dai_driver {
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
    symmetric_rate: u32,
}

static mut rockchip_pdm_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 8,
        rates: 0,
        formats: 0,
    },
    ops: &rockchip_pdm_dai_ops,
    symmetric_rate: 1,
};

#[repr(C)]
struct snd_soc_component_driver {
    name: *const c_char,
    legacy_dai_naming: u32,
}

static rockchip_pdm_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"rockchip-pdm\0".as_ptr() as *const c_char,
    legacy_dai_naming: 1,
};

unsafe extern "C" fn rockchip_pdm_runtime_suspend(dev: *mut device) -> c_int {
    let pdm = dev_get_drvdata(dev) as *mut rk_pdm_dev;

    clk_disable_unprepare((*pdm).clk);
    clk_disable_unprepare((*pdm).hclk);

    0
}

unsafe extern "C" fn rockchip_pdm_runtime_resume(dev: *mut device) -> c_int {
    let pdm = dev_get_drvdata(dev) as *mut rk_pdm_dev;

    let mut ret = clk_prepare_enable((*pdm).hclk);
    if ret != 0 {
        dev_err((*pdm).dev, b"hclock enable failed %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = clk_prepare_enable((*pdm).clk);
    if ret != 0 {
        clk_disable_unprepare((*pdm).hclk);
        dev_err((*pdm).dev, b"clock enable failed %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    0
}

unsafe extern "C" fn rockchip_pdm_wr_reg(_dev: *mut device, reg: u32) -> bool_ {
    match reg {
        x if x == PDM_SYSCONFIG || x == PDM_CTRL0 || x == PDM_CTRL1 || x == PDM_CLK_CTRL ||
             x == PDM_HPF_CTRL || x == PDM_FIFO_CTRL || x == PDM_DMA_CTRL || x == PDM_INT_EN ||
             x == PDM_INT_CLR || x == PDM_DATA_VALID => true,
        _ => false,
    }
}

unsafe extern "C" fn rockchip_pdm_rd_reg(_dev: *mut device, reg: u32) -> bool_ {
    match reg {
        x if x == PDM_SYSCONFIG || x == PDM_CTRL0 || x == PDM_CTRL1 || x == PDM_CLK_CTRL ||
             x == PDM_HPF_CTRL || x == PDM_FIFO_CTRL || x == PDM_DMA_CTRL || x == PDM_INT_EN ||
             x == PDM_INT_CLR || x == PDM_INT_ST || x == PDM_DATA_VALID || x == PDM_RXFIFO_DATA ||
             x == PDM_VERSION => true,
        _ => false,
    }
}

unsafe extern "C" fn rockchip_pdm_volatile_reg(_dev: *mut device, reg: u32) -> bool_ {
    match reg {
        x if x == PDM_SYSCONFIG || x == PDM_FIFO_CTRL || x == PDM_INT_CLR || x == PDM_INT_ST || x == PDM_RXFIFO_DATA => true,
        _ => false,
    }
}

unsafe extern "C" fn rockchip_pdm_precious_reg(_dev: *mut device, reg: u32) -> bool_ {
    match reg {
        x if x == PDM_RXFIFO_DATA => true,
        _ => false,
    }
}

#[repr(C)]
struct reg_default {
    reg: u32,
    def: u32,
}

static rockchip_pdm_reg_defaults: [reg_default; 4] = [
    reg_default { reg: PDM_CTRL0, def: 0x78000017 },
    reg_default { reg: PDM_CTRL1, def: 0x0bb8ea60 },
    reg_default { reg: PDM_CLK_CTRL, def: 0x0000e401 },
    reg_default { reg: PDM_DMA_CTRL, def: 0x0000001f },
];

#[repr(C)]
struct regmap_config {
    reg_bits: u32,
    reg_stride: u32,
    val_bits: u32,
    max_register: u32,
    reg_defaults: *const reg_default,
    num_reg_defaults: usize,
    writeable_reg: Option<unsafe extern "C" fn(*mut device, u32) -> bool_>,
    readable_reg: Option<unsafe extern "C" fn(*mut device, u32) -> bool_>,
    volatile_reg: Option<unsafe extern "C" fn(*mut device, u32) -> bool_>,
    precious_reg: Option<unsafe extern "C" fn(*mut device, u32) -> bool_>,
    cache_type: u32,
}

static rockchip_pdm_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: PDM_VERSION,
    reg_defaults: rockchip_pdm_reg_defaults.as_ptr(),
    num_reg_defaults: rockchip_pdm_reg_defaults.len(),
    writeable_reg: Some(rockchip_pdm_wr_reg),
    readable_reg: Some(rockchip_pdm_rd_reg),
    volatile_reg: Some(rockchip_pdm_volatile_reg),
    precious_reg: Some(rockchip_pdm_precious_reg),
    cache_type: REGCACHE_FLAT,
};

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
    data: *const c_void,
}

/* __maybe_unused */
static rockchip_pdm_match: [of_device_id; 7] = [
    of_device_id { compatible: b"rockchip,pdm\0".as_ptr() as *const c_char, data: rk_pdm_version::RK_PDM_RK3229 as usize as *const c_void },
    of_device_id { compatible: b"rockchip,px30-pdm\0".as_ptr() as *const c_char, data: rk_pdm_version::RK_PDM_RK3308 as usize as *const c_void },
    of_device_id { compatible: b"rockchip,rk1808-pdm\0".as_ptr() as *const c_char, data: rk_pdm_version::RK_PDM_RK3308 as usize as *const c_void },
    of_device_id { compatible: b"rockchip,rk3308-pdm\0".as_ptr() as *const c_char, data: rk_pdm_version::RK_PDM_RK3308 as usize as *const c_void },
    of_device_id { compatible: b"rockchip,rk3568-pdm\0".as_ptr() as *const c_char, data: rk_pdm_version::RK_PDM_RV1126 as usize as *const c_void },
    of_device_id { compatible: b"rockchip,rv1126-pdm\0".as_ptr() as *const c_char, data: rk_pdm_version::RK_PDM_RV1126 as usize as *const c_void },
    of_device_id { compatible: ptr::null(), data: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, rockchip_pdm_match); */

unsafe fn rockchip_pdm_path_parse(pdm: *mut rk_pdm_dev, node: *mut device_node) -> c_int {
    let mut path = [0u32; PDM_PATH_MAX];
    let mut val: c_int = 0;
    let mut msk: c_int = 0;

    let cnt = of_count_phandle_with_args(node, b"rockchip,path-map\0".as_ptr() as *const c_char, ptr::null());
    if cnt != PDM_PATH_MAX as c_int {
        return cnt;
    }

    let ret = of_property_read_u32_array(node, b"rockchip,path-map\0".as_ptr() as *const c_char, path.as_mut_ptr(), cnt);
    if ret != 0 {
        return ret;
    }

    let mut i: c_int = 0;
    while i < cnt {
        if path[i as usize] >= PDM_PATH_MAX as u32 {
            return -EINVAL;
        }
        msk |= PDM_PATH_MASK(i);
        val |= PDM_PATH(i, path[i as usize]);
        i += 1;
    }

    regmap_update_bits((*pdm).regmap, PDM_CLK_CTRL, msk as u32, val as u32);

    0
}

unsafe extern "C" fn rockchip_pdm_probe(pdev: *mut platform_device) -> c_int {
    let node = (*pdev).dev.of_node;
    let mut res: *mut resource = ptr::null_mut();
    let ret: c_int;

    let pdm = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<rk_pdm_dev>(), GFP_KERNEL) as *mut rk_pdm_dev;
    if pdm.is_null() {
        return -ENOMEM;
    }

    (*pdm).version = core::mem::transmute::<usize, rk_pdm_version>(device_get_match_data(&mut (*pdev).dev) as usize);
    if (*pdm).version == rk_pdm_version::RK_PDM_RK3308 {
        (*pdm).reset = devm_reset_control_get(&mut (*pdev).dev, b"pdm-m\0".as_ptr() as *const c_char);
        if IS_ERR((*pdm).reset as *const c_void) {
            return PTR_ERR((*pdm).reset as *const c_void);
        }
    }

    let regs = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(regs as *const c_void) {
        return PTR_ERR(regs as *const c_void);
    }

    (*pdm).regmap = devm_regmap_init_mmio(&mut (*pdev).dev, regs, &rockchip_pdm_regmap_config);
    if IS_ERR((*pdm).regmap as *const c_void) {
        return PTR_ERR((*pdm).regmap as *const c_void);
    }

    (*pdm).capture_dma_data.addr = (*res).start + PDM_RXFIFO_DATA as u64;
    (*pdm).capture_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    (*pdm).capture_dma_data.maxburst = PDM_DMA_BURST_SIZE;

    (*pdm).dev = &mut (*pdev).dev;
    dev_set_drvdata(&mut (*pdev).dev, pdm as *mut c_void);

    (*pdm).clk = devm_clk_get(&mut (*pdev).dev, b"pdm_clk\0".as_ptr() as *const c_char);
    if IS_ERR((*pdm).clk as *const c_void) {
        return PTR_ERR((*pdm).clk as *const c_void);
    }

    (*pdm).hclk = devm_clk_get(&mut (*pdev).dev, b"pdm_hclk\0".as_ptr() as *const c_char);
    if IS_ERR((*pdm).hclk as *const c_void) {
        return PTR_ERR((*pdm).hclk as *const c_void);
    }

    let mut ret = clk_prepare_enable((*pdm).hclk);
    if ret != 0 {
        return ret;
    }

    pm_runtime_enable(&mut (*pdev).dev);
    if !pm_runtime_enabled(&mut (*pdev).dev) {
        ret = rockchip_pdm_runtime_resume(&mut (*pdev).dev);
        if ret != 0 {
            pm_runtime_disable(&mut (*pdev).dev);
            clk_disable_unprepare((*pdm).hclk);
            return ret;
        }
    }

    rockchip_pdm_dai.capture.rates = ROCKCHIP_PDM_RATES();
    rockchip_pdm_dai.capture.formats = ROCKCHIP_PDM_FORMATS();

    ret = devm_snd_soc_register_component(&mut (*pdev).dev, &rockchip_pdm_component, &mut rockchip_pdm_dai, 1);
    if ret != 0 {
        if !pm_runtime_status_suspended(&mut (*pdev).dev) {
            rockchip_pdm_runtime_suspend(&mut (*pdev).dev);
        }
        pm_runtime_disable(&mut (*pdev).dev);
        clk_disable_unprepare((*pdm).hclk);
        return ret;
    }

    rockchip_pdm_rxctrl(pdm, 0);

    ret = rockchip_pdm_path_parse(pdm, node);
    if ret != 0 && ret != -ENOENT {
        if !pm_runtime_status_suspended(&mut (*pdev).dev) {
            rockchip_pdm_runtime_suspend(&mut (*pdev).dev);
        }
        pm_runtime_disable(&mut (*pdev).dev);
        clk_disable_unprepare((*pdm).hclk);
        return ret;
    }

    ret = devm_snd_dmaengine_pcm_register(&mut (*pdev).dev, ptr::null(), 0);
    if ret != 0 {
        if !pm_runtime_status_suspended(&mut (*pdev).dev) {
            rockchip_pdm_runtime_suspend(&mut (*pdev).dev);
        }
        pm_runtime_disable(&mut (*pdev).dev);
        clk_disable_unprepare((*pdm).hclk);
        return ret;
    }

    0
}

unsafe extern "C" fn rockchip_pdm_remove(pdev: *mut platform_device) {
    let pdm = dev_get_drvdata(&mut (*pdev).dev) as *mut rk_pdm_dev;

    pm_runtime_disable(&mut (*pdev).dev);
    if !pm_runtime_status_suspended(&mut (*pdev).dev) {
        rockchip_pdm_runtime_suspend(&mut (*pdev).dev);
    }

    clk_disable_unprepare((*pdm).clk);
    clk_disable_unprepare((*pdm).hclk);
}

unsafe extern "C" fn rockchip_pdm_suspend(dev: *mut device) -> c_int {
    let pdm = dev_get_drvdata(dev) as *mut rk_pdm_dev;

    regcache_mark_dirty((*pdm).regmap);

    0
}

unsafe extern "C" fn rockchip_pdm_resume(dev: *mut device) -> c_int {
    let pdm = dev_get_drvdata(dev) as *mut rk_pdm_dev;

    let mut ret = pm_runtime_resume_and_get(dev);
    if ret < 0 {
        return ret;
    }

    ret = regcache_sync((*pdm).regmap);

    pm_runtime_put(dev);

    ret
}

#[repr(C)]
struct dev_pm_ops {
    runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

static rockchip_pdm_pm_ops: dev_pm_ops = dev_pm_ops {
    /* RUNTIME_PM_OPS(rockchip_pdm_runtime_suspend, rockchip_pdm_runtime_resume, NULL) */
    runtime_suspend: Some(rockchip_pdm_runtime_suspend),
    runtime_resume: Some(rockchip_pdm_runtime_resume),
    /* SYSTEM_SLEEP_PM_OPS(rockchip_pdm_suspend, rockchip_pdm_resume) */
    suspend: Some(rockchip_pdm_suspend),
    resume: Some(rockchip_pdm_resume),
};

#[repr(C)]
struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
    pm: *const dev_pm_ops,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    driver: device_driver,
}

static mut rockchip_pdm_driver: platform_driver = platform_driver {
    probe: Some(rockchip_pdm_probe),
    remove: Some(rockchip_pdm_remove),
    driver: device_driver {
        name: b"rockchip-pdm\0".as_ptr() as *const c_char,
        of_match_table: rockchip_pdm_match.as_ptr(),
        pm: &rockchip_pdm_pm_ops,
    },
};

/* module_platform_driver(rockchip_pdm_driver); */

/* MODULE_AUTHOR("Sugar <sugar.zhang@rock-chips.com>"); */
/* MODULE_DESCRIPTION("Rockchip PDM Controller Driver"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
