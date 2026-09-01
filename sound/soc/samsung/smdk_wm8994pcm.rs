// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (c) 2011 Samsung Electronics Co., Ltd
//		http://www.samsung.com

// C dependencies:
// linux/module.h
// sound/soc.h
// sound/pcm.h
// sound/pcm_params.h
// ../codecs/wm8994.h
// pcm.h

/*
 * Board Settings:
 *  o '1' means 'ON'
 *  o '0' means 'OFF'
 *  o 'X' means 'Don't care'
 *
 * SMDKC210, SMDKV310: CFG3- 1001, CFG5-1000, CFG7-111111
 */

/*
 * Configure audio route as :-
 * $ amixer sset 'DAC1' on,on
 * $ amixer sset 'Right Headphone Mux' 'DAC'
 * $ amixer sset 'Left Headphone Mux' 'DAC'
 * $ amixer sset 'DAC1R Mixer AIF1.1' on
 * $ amixer sset 'DAC1L Mixer AIF1.1' on
 * $ amixer sset 'IN2L' on
 * $ amixer sset 'IN2L PGA IN2LN' on
 * $ amixer sset 'MIXINL IN2L' on
 * $ amixer sset 'AIF1ADC1L Mixer ADC/DMIC' on
 * $ amixer sset 'IN2R' on
 * $ amixer sset 'IN2R PGA IN2RN' on
 * $ amixer sset 'MIXINR IN2R' on
 * $ amixer sset 'AIF1ADC1R Mixer ADC/DMIC' on
 */

use core::ffi::{c_char, c_int, c_ulong};

/* SMDK has a 16.9344MHZ crystal attached to WM8994 */
const SMDK_WM8994_FREQ: c_int = 16934400;

const EINVAL: c_int = 22;

extern "C" {
    static mut THIS_MODULE: *mut module;

    static mut paif_pcm_cpus: [snd_soc_dai_link_component; 1];
    static mut paif_pcm_codecs: [snd_soc_dai_link_component; 1];
    static mut paif_pcm_platforms: [snd_soc_dai_link_component; 1];

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_ulong,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn snd_soc_dai_set_clkdiv(dai: *mut snd_soc_dai, div_id: c_int, div: c_int) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

type c_uint = u32;

#[repr(C)]
pub struct module {
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
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub dai_fmt: c_uint,
    pub ops: *const snd_soc_ops,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut module,
    pub dev: *mut device,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
}

const WM8994_SYSCLK_FLL1: c_int = 1;
const WM8994_FLL1: c_int = 1;
const WM8994_FLL_SRC_MCLK1: c_int = 1;
const SND_SOC_CLOCK_IN: c_int = 0;
const S3C_PCM_CLKSRC_MUX: c_int = 0;
const S3C_PCM_SCLK_PER_FS: c_int = 0;

const SND_SOC_DAIFMT_DSP_B: c_uint = 0;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;

unsafe extern "C" fn smdk_wm8994_pcm_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let cpu_dai: *mut snd_soc_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mclk_freq: c_ulong;
    let rfs: c_int;
    let mut ret: c_int;

    match params_rate(params) {
        8000 => {
            rfs = 512;
        }
        _ => {
            dev_err(
                (*cpu_dai).dev,
                c"%s:%d Sampling Rate %u not supported!\n".as_ptr(),
                c"smdk_wm8994_pcm_hw_params".as_ptr(),
                line!() as c_int,
                params_rate(params),
            );
            return -EINVAL;
        }
    }

    mclk_freq = (params_rate(params) * rfs) as c_ulong;

    ret = snd_soc_dai_set_sysclk(codec_dai, WM8994_SYSCLK_FLL1, mclk_freq, SND_SOC_CLOCK_IN);
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_dai_set_pll(
        codec_dai,
        WM8994_FLL1,
        WM8994_FLL_SRC_MCLK1,
        SMDK_WM8994_FREQ as c_uint,
        mclk_freq as c_uint,
    );
    if ret < 0 {
        return ret;
    }

    /* Set PCM source clock on CPU */
    ret = snd_soc_dai_set_sysclk(cpu_dai, S3C_PCM_CLKSRC_MUX, mclk_freq, SND_SOC_CLOCK_IN);
    if ret < 0 {
        return ret;
    }

    /* Set SCLK_DIV for making bclk */
    ret = snd_soc_dai_set_clkdiv(cpu_dai, S3C_PCM_SCLK_PER_FS, rfs);
    if ret < 0 {
        return ret;
    }

    return 0;
}

static smdk_wm8994_pcm_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(smdk_wm8994_pcm_hw_params),
};

/*
 * SND_SOC_DAILINK_DEFS(paif_pcm,
 *	DAILINK_COMP_ARRAY(COMP_CPU("samsung-pcm.0")),
 *	DAILINK_COMP_ARRAY(COMP_CODEC("wm8994-codec", "wm8994-aif1")),
 *	DAILINK_COMP_ARRAY(COMP_PLATFORM("samsung-pcm.0")));
 */

static mut smdk_dai: [snd_soc_dai_link; 1] = [snd_soc_dai_link {
    name: c"WM8994 PAIF PCM".as_ptr(),
    stream_name: c"Primary PCM".as_ptr(),
    dai_fmt: SND_SOC_DAIFMT_DSP_B | SND_SOC_DAIFMT_IB_NF | SND_SOC_DAIFMT_CBC_CFC,
    ops: &smdk_wm8994_pcm_ops,
    cpus: unsafe { paif_pcm_cpus.as_mut_ptr() },
    num_cpus: 1,
    codecs: unsafe { paif_pcm_codecs.as_mut_ptr() },
    num_codecs: 1,
    platforms: unsafe { paif_pcm_platforms.as_mut_ptr() },
    num_platforms: 1,
}];

static mut smdk_pcm: snd_soc_card = snd_soc_card {
    name: c"SMDK-PCM".as_ptr(),
    owner: unsafe { THIS_MODULE },
    dev: core::ptr::null_mut(),
    dai_link: unsafe { smdk_dai.as_mut_ptr() },
    num_links: 1,
};

unsafe extern "C" fn snd_smdk_probe(pdev: *mut platform_device) -> c_int {
    let mut ret: c_int = 0;

    smdk_pcm.dev = &mut (*pdev).dev;
    ret = devm_snd_soc_register_card(&mut (*pdev).dev, &mut smdk_pcm);
    if ret != 0 {
        dev_err_probe(
            &mut (*pdev).dev,
            ret,
            c"snd_soc_register_card failed\n".as_ptr(),
        );
    }

    return ret;
}

static mut snd_smdk_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"samsung-smdk-pcm".as_ptr(),
    },
    probe: Some(snd_smdk_probe),
};

// module_platform_driver(snd_smdk_driver);

// MODULE_AUTHOR("Sangbeom Kim, <sbkim73@samsung.com>");
// MODULE_DESCRIPTION("ALSA SoC SMDK WM8994 for PCM");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
