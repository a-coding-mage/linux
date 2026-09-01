// SPDX-License-Identifier: GPL-2.0+

// Dependencies from:
// ../codecs/wm8994.h
// sound/pcm_params.h
// sound/soc.h
// linux/module.h
// linux/of.h

/*
 * Default CFG switch settings to use this driver:
 *	SMDKV310: CFG5-1000, CFG7-111111
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

use core::ffi::{c_char, c_int, c_uint};
use core::ptr;

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
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
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
    pub owner: *mut core::ffi::c_void,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_uint,
    pub dev: *mut device,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

extern "C" {
    static mut THIS_MODULE: *mut core::ffi::c_void;
    static snd_soc_pm_ops: dev_pm_ops;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char);
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
}

extern "Rust" {
    fn module_platform_driver(driver: *mut platform_driver);
}

const WM8994_FLL1: c_int = 1;
const WM8994_FLL_SRC_MCLK1: c_int = 1;
const WM8994_SYSCLK_FLL1: c_int = 1;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_NB_NF: c_uint = 2;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 4;
const EINVAL: c_int = 22;

/* SMDK has a 16.934MHZ crystal attached to WM8994 */
const SMDK_WM8994_FREQ: c_uint = 16_934_000;

unsafe extern "C" fn smdk_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let pll_out: c_uint;
    let mut ret: c_int;

    /* AIF1CLK should be >=3MHz for optimal performance */
    if params_width(params) == 24 {
        pll_out = params_rate(params).wrapping_mul(384);
    } else if params_rate(params) == 8000 || params_rate(params) == 11025 {
        pll_out = params_rate(params).wrapping_mul(512);
    } else {
        pll_out = params_rate(params).wrapping_mul(256);
    }

    ret = snd_soc_dai_set_pll(
        codec_dai,
        WM8994_FLL1,
        WM8994_FLL_SRC_MCLK1,
        SMDK_WM8994_FREQ,
        pll_out,
    );
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(codec_dai, WM8994_SYSCLK_FLL1, pll_out, SND_SOC_CLOCK_IN);
    if ret < 0 {
        return ret;
    }

    0
}

/*
 * SMDK WM8994 DAI operations.
 */
static SMDK_OPS: snd_soc_ops = snd_soc_ops {
    hw_params: Some(smdk_hw_params),
};

unsafe extern "C" fn smdk_wm8994_init_paiftx(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm((*rtd).card);

    /* Other pins NC */
    snd_soc_dapm_disable_pin(dapm, c"HPOUT2P".as_ptr());
    snd_soc_dapm_disable_pin(dapm, c"HPOUT2N".as_ptr());
    snd_soc_dapm_disable_pin(dapm, c"SPKOUTLN".as_ptr());
    snd_soc_dapm_disable_pin(dapm, c"SPKOUTLP".as_ptr());
    snd_soc_dapm_disable_pin(dapm, c"SPKOUTRP".as_ptr());
    snd_soc_dapm_disable_pin(dapm, c"SPKOUTRN".as_ptr());
    snd_soc_dapm_disable_pin(dapm, c"LINEOUT1N".as_ptr());
    snd_soc_dapm_disable_pin(dapm, c"LINEOUT1P".as_ptr());
    snd_soc_dapm_disable_pin(dapm, c"LINEOUT2N".as_ptr());
    snd_soc_dapm_disable_pin(dapm, c"LINEOUT2P".as_ptr());
    snd_soc_dapm_disable_pin(dapm, c"IN1LP".as_ptr());
    snd_soc_dapm_disable_pin(dapm, c"IN2LP:VXRN".as_ptr());
    snd_soc_dapm_disable_pin(dapm, c"IN1RP".as_ptr());
    snd_soc_dapm_disable_pin(dapm, c"IN2RP:VXRP".as_ptr());

    0
}

static mut AIF1_CPUS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"samsung-i2s.0".as_ptr(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];

static mut AIF1_CODECS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"wm8994-codec".as_ptr(),
    dai_name: c"wm8994-aif1".as_ptr(),
    of_node: ptr::null_mut(),
}];

static mut AIF1_PLATFORMS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"samsung-i2s.0".as_ptr(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];

static mut FIFO_TX_CPUS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"samsung-i2s-sec".as_ptr(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];

static mut FIFO_TX_CODECS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"wm8994-codec".as_ptr(),
    dai_name: c"wm8994-aif1".as_ptr(),
    of_node: ptr::null_mut(),
}];

static mut FIFO_TX_PLATFORMS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"samsung-i2s-sec".as_ptr(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];

static mut SMDK_DAI: [snd_soc_dai_link; 2] = [
    snd_soc_dai_link {
        /* Primary DAI i/f */
        name: c"WM8994 AIF1".as_ptr(),
        stream_name: c"Pri_Dai".as_ptr(),
        init: Some(smdk_wm8994_init_paiftx),
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
        ops: &SMDK_OPS,
        cpus: unsafe { AIF1_CPUS.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { AIF1_CODECS.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { AIF1_PLATFORMS.as_mut_ptr() },
        num_platforms: 1,
    },
    snd_soc_dai_link {
        /* Sec_Fifo Playback i/f */
        name: c"Sec_FIFO TX".as_ptr(),
        stream_name: c"Sec_Dai".as_ptr(),
        init: None,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
        ops: &SMDK_OPS,
        cpus: unsafe { FIFO_TX_CPUS.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { FIFO_TX_CODECS.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { FIFO_TX_PLATFORMS.as_mut_ptr() },
        num_platforms: 1,
    },
];

static mut SMDK: snd_soc_card = snd_soc_card {
    name: c"SMDK-I2S".as_ptr(),
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { SMDK_DAI.as_mut_ptr() },
    num_links: 2,
    dev: ptr::null_mut(),
};

static SAMSUNG_WM8994_OF_MATCH: [of_device_id; 2] = [
    of_device_id {
        compatible: c"samsung,smdk-wm8994".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, samsung_wm8994_of_match);

unsafe extern "C" fn smdk_audio_probe(pdev: *mut platform_device) -> c_int {
    let mut ret: c_int;
    let np: *mut device_node = (*pdev).dev.of_node;
    let card: *mut snd_soc_card = &raw mut SMDK;

    (*card).dev = &raw mut (*pdev).dev;

    if !np.is_null() {
        (*SMDK_DAI[0].cpus).dai_name = ptr::null();
        (*SMDK_DAI[0].cpus).of_node =
            of_parse_phandle(np, c"samsung,i2s-controller".as_ptr(), 0);
        if (*SMDK_DAI[0].cpus).of_node.is_null() {
            dev_err(
                &raw mut (*pdev).dev,
                c"Property 'samsung,i2s-controller' missing or invalid\n".as_ptr(),
            );
            ret = -EINVAL;
            return ret;
        }

        (*SMDK_DAI[0].platforms).name = ptr::null();
        (*SMDK_DAI[0].platforms).of_node = (*SMDK_DAI[0].cpus).of_node;
    }

    ret = devm_snd_soc_register_card(&raw mut (*pdev).dev, card);

    if ret != 0 {
        dev_err_probe(
            &raw mut (*pdev).dev,
            ret,
            c"snd_soc_register_card() failed\n".as_ptr(),
        );
    }

    ret
}

static mut SMDK_AUDIO_DRIVER: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: c"smdk-audio-wm8994".as_ptr(),
        of_match_table: SAMSUNG_WM8994_OF_MATCH.as_ptr(),
        pm: unsafe { &snd_soc_pm_ops },
    },
    probe: Some(smdk_audio_probe),
};

#[used]
static MODULE_INIT: unsafe extern "C" fn(*mut platform_driver) = module_platform_driver;

#[used]
static MODULE_INIT_DRIVER: *mut platform_driver = &raw mut SMDK_AUDIO_DRIVER;

// MODULE_DESCRIPTION("ALSA SoC SMDK WM8994");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:smdk-audio-wm8994");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
