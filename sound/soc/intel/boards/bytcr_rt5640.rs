// SPDX-License-Identifier: GPL-2.0-only
/*
 *  byt_cr_dpcm_rt5640.c - ASoc Machine driver for Intel Byt CR platform
 *
 *  Copyright (C) 2014 Intel Corp
 *  Author: Subhransu S. Prusty <subhransu.s.prusty@intel.com>
 *  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

// C includes translated as external dependency intent:
// linux/{i2c,init,module,moduleparam,platform_device,acpi,clk,device,bus,dmi,gpio/consumer,gpio/machine,input,slab}.h
// sound/{pcm,pcm_params,soc,jack,soc-acpi}.h
// dt-bindings/sound/rt5640.h, codecs/rt5640.h, atom/sst-atom-controls.h, common/soc-intel-quirks.h

const BYT_RT5640_FALLBACK_CODEC_DEV_NAME: *const c_char = b"i2c-rt5640\0".as_ptr() as *const c_char;

const BYT_RT5640_DMIC1_MAP: c_ulong = 0;
const BYT_RT5640_DMIC2_MAP: c_ulong = 1;
const BYT_RT5640_IN1_MAP: c_ulong = 2;
const BYT_RT5640_IN3_MAP: c_ulong = 3;
const BYT_RT5640_NO_INTERNAL_MIC_MAP: c_ulong = 4;

const RT5640_JD_SRC_EXT_GPIO: c_ulong = 0x0f;

const BYT_RT5640_JD_SRC_GPIO1: c_ulong = RT5640_JD_SRC_GPIO1 << 4;
const BYT_RT5640_JD_SRC_JD1_IN4P: c_ulong = RT5640_JD_SRC_JD1_IN4P << 4;
const BYT_RT5640_JD_SRC_JD2_IN4N: c_ulong = RT5640_JD_SRC_JD2_IN4N << 4;
const BYT_RT5640_JD_SRC_GPIO2: c_ulong = RT5640_JD_SRC_GPIO2 << 4;
const BYT_RT5640_JD_SRC_GPIO3: c_ulong = RT5640_JD_SRC_GPIO3 << 4;
const BYT_RT5640_JD_SRC_GPIO4: c_ulong = RT5640_JD_SRC_GPIO4 << 4;
const BYT_RT5640_JD_SRC_EXT_GPIO: c_ulong = RT5640_JD_SRC_EXT_GPIO << 4;

const BYT_RT5640_OVCD_TH_600UA: c_ulong = 6 << 8;
const BYT_RT5640_OVCD_TH_1500UA: c_ulong = 15 << 8;
const BYT_RT5640_OVCD_TH_2000UA: c_ulong = 20 << 8;

const BYT_RT5640_OVCD_SF_0P5: c_ulong = RT5640_OVCD_SF_0P5 << 13;
const BYT_RT5640_OVCD_SF_0P75: c_ulong = RT5640_OVCD_SF_0P75 << 13;
const BYT_RT5640_OVCD_SF_1P0: c_ulong = RT5640_OVCD_SF_1P0 << 13;
const BYT_RT5640_OVCD_SF_1P5: c_ulong = RT5640_OVCD_SF_1P5 << 13;

const fn BIT(n: c_ulong) -> c_ulong { 1 << n }
const fn GENMASK(h: c_ulong, l: c_ulong) -> c_ulong {
    ((!0 as c_ulong) << l) & ((!0 as c_ulong) >> (c_ulong::BITS as c_ulong - 1 - h))
}

const BYT_RT5640_MAP_MASK: c_ulong = GENMASK(3, 0);
const fn BYT_RT5640_MAP(quirk: c_ulong) -> c_ulong { quirk & BYT_RT5640_MAP_MASK }
const fn BYT_RT5640_JDSRC(quirk: c_ulong) -> c_ulong { (quirk & GENMASK(7, 4)) >> 4 }
const fn BYT_RT5640_OVCD_TH(quirk: c_ulong) -> c_ulong { (quirk & GENMASK(12, 8)) >> 8 }
const fn BYT_RT5640_OVCD_SF(quirk: c_ulong) -> c_ulong { (quirk & GENMASK(14, 13)) >> 13 }

const BYT_RT5640_JD_NOT_INV: c_ulong = BIT(16);
const BYT_RT5640_MONO_SPEAKER: c_ulong = BIT(17);
const BYT_RT5640_DIFF_MIC: c_ulong = BIT(18); /* default is single-ended */
const BYT_RT5640_SSP2_AIF2: c_ulong = BIT(19); /* default is using AIF1  */
const BYT_RT5640_SSP0_AIF1: c_ulong = BIT(20);
const BYT_RT5640_SSP0_AIF2: c_ulong = BIT(21);
const BYT_RT5640_MCLK_EN: c_ulong = BIT(22);
const BYT_RT5640_MCLK_25MHZ: c_ulong = BIT(23);
const BYT_RT5640_NO_SPEAKERS: c_ulong = BIT(24);
const BYT_RT5640_LINEOUT: c_ulong = BIT(25);
const BYT_RT5640_LINEOUT_AS_HP2: c_ulong = BIT(26);
const BYT_RT5640_HSMIC2_ON_IN1: c_ulong = BIT(27);
const BYT_RT5640_JD_HP_ELITEP_1000G2: c_ulong = BIT(28);
const BYT_RT5640_USE_AMCR0F28: c_ulong = BIT(29);
const BYT_RT5640_SWAPPED_SPEAKERS: c_ulong = BIT(30);

const BYTCR_INPUT_DEFAULTS: c_ulong =
    BYT_RT5640_IN3_MAP |
    BYT_RT5640_JD_SRC_JD1_IN4P |
    BYT_RT5640_OVCD_TH_2000UA |
    BYT_RT5640_OVCD_SF_0P75 |
    BYT_RT5640_DIFF_MIC;

/* in-diff or dmic-pin + jdsrc + ovcd-th + -sf + jd-inv + terminating entry */
const MAX_NO_PROPS: usize = 6;

#[repr(C)]
pub struct byt_rt5640_private {
    jack: snd_soc_jack,
    jack2: snd_soc_jack,
    jack_data: rt5640_set_jack_data,
    hsmic_detect: *mut gpio_desc,
    mclk: *mut clk,
    codec_dev: *mut device,
}

static mut is_bytcr: bool = false;
static mut byt_rt5640_quirk: c_ulong = BYT_RT5640_MCLK_EN;
static mut quirk_override: c_int = -1;
// module_param_named(quirk, quirk_override, int, 0444);
// MODULE_PARM_DESC(quirk, "Board-specific quirk override");

unsafe fn log_quirks(dev: *mut device) {
    let mut has_mclk = false;
    let mut has_ssp0 = false;
    let mut has_ssp0_aif1 = false;
    let mut has_ssp0_aif2 = false;
    let mut has_ssp2_aif2 = false;
    let map = BYT_RT5640_MAP(byt_rt5640_quirk);

    match map {
        BYT_RT5640_DMIC1_MAP => dev_info(dev, c"quirk DMIC1_MAP enabled\n".as_ptr()),
        BYT_RT5640_DMIC2_MAP => dev_info(dev, c"quirk DMIC2_MAP enabled\n".as_ptr()),
        BYT_RT5640_IN1_MAP => dev_info(dev, c"quirk IN1_MAP enabled\n".as_ptr()),
        BYT_RT5640_IN3_MAP => dev_info(dev, c"quirk IN3_MAP enabled\n".as_ptr()),
        BYT_RT5640_NO_INTERNAL_MIC_MAP => dev_info(dev, c"quirk NO_INTERNAL_MIC_MAP enabled\n".as_ptr()),
        _ => {
            dev_warn_once(dev, c"quirk sets invalid input map: 0x%x, default to DMIC1_MAP\n".as_ptr(), map);
            byt_rt5640_quirk &= !BYT_RT5640_MAP_MASK;
            byt_rt5640_quirk |= BYT_RT5640_DMIC1_MAP;
        }
    }

    if byt_rt5640_quirk & BYT_RT5640_HSMIC2_ON_IN1 != 0 { dev_info(dev, c"quirk HSMIC2_ON_IN1 enabled\n".as_ptr()); }
    if BYT_RT5640_JDSRC(byt_rt5640_quirk) != 0 {
        dev_info(dev, c"quirk realtek,jack-detect-source %ld\n".as_ptr(), BYT_RT5640_JDSRC(byt_rt5640_quirk));
        dev_info(dev, c"quirk realtek,over-current-threshold-microamp %ld\n".as_ptr(), BYT_RT5640_OVCD_TH(byt_rt5640_quirk) * 100);
        dev_info(dev, c"quirk realtek,over-current-scale-factor %ld\n".as_ptr(), BYT_RT5640_OVCD_SF(byt_rt5640_quirk));
    }
    if byt_rt5640_quirk & BYT_RT5640_JD_NOT_INV != 0 { dev_info(dev, c"quirk JD_NOT_INV enabled\n".as_ptr()); }
    if byt_rt5640_quirk & BYT_RT5640_JD_HP_ELITEP_1000G2 != 0 { dev_info(dev, c"quirk JD_HP_ELITEPAD_1000G2 enabled\n".as_ptr()); }
    if byt_rt5640_quirk & BYT_RT5640_MONO_SPEAKER != 0 { dev_info(dev, c"quirk MONO_SPEAKER enabled\n".as_ptr()); }
    if byt_rt5640_quirk & BYT_RT5640_NO_SPEAKERS != 0 { dev_info(dev, c"quirk NO_SPEAKERS enabled\n".as_ptr()); }
    if byt_rt5640_quirk & BYT_RT5640_SWAPPED_SPEAKERS != 0 { dev_info(dev, c"quirk SWAPPED_SPEAKERS enabled\n".as_ptr()); }
    if byt_rt5640_quirk & BYT_RT5640_LINEOUT != 0 { dev_info(dev, c"quirk LINEOUT enabled\n".as_ptr()); }
    if byt_rt5640_quirk & BYT_RT5640_LINEOUT_AS_HP2 != 0 { dev_info(dev, c"quirk LINEOUT_AS_HP2 enabled\n".as_ptr()); }
    if byt_rt5640_quirk & BYT_RT5640_DIFF_MIC != 0 { dev_info(dev, c"quirk DIFF_MIC enabled\n".as_ptr()); }
    if byt_rt5640_quirk & BYT_RT5640_SSP0_AIF1 != 0 {
        dev_info(dev, c"quirk SSP0_AIF1 enabled\n".as_ptr());
        has_ssp0 = true; has_ssp0_aif1 = true;
    }
    if byt_rt5640_quirk & BYT_RT5640_SSP0_AIF2 != 0 {
        dev_info(dev, c"quirk SSP0_AIF2 enabled\n".as_ptr());
        has_ssp0 = true; has_ssp0_aif2 = true;
    }
    if byt_rt5640_quirk & BYT_RT5640_SSP2_AIF2 != 0 {
        dev_info(dev, c"quirk SSP2_AIF2 enabled\n".as_ptr());
        has_ssp2_aif2 = true;
    }
    if is_bytcr && !has_ssp0 { dev_err(dev, c"Invalid routing, bytcr detected but no SSP0-based quirk, audio cannot work with SSP2 on bytcr\n".as_ptr()); }
    if has_ssp0_aif1 && has_ssp0_aif2 { dev_err(dev, c"Invalid routing, SSP0 cannot be connected to both AIF1 and AIF2\n".as_ptr()); }
    if has_ssp0 && has_ssp2_aif2 { dev_err(dev, c"Invalid routing, cannot have both SSP0 and SSP2 connected to codec\n".as_ptr()); }
    if byt_rt5640_quirk & BYT_RT5640_MCLK_EN != 0 {
        dev_info(dev, c"quirk MCLK_EN enabled\n".as_ptr());
        has_mclk = true;
    }
    if byt_rt5640_quirk & BYT_RT5640_MCLK_25MHZ != 0 {
        if has_mclk {
            dev_info(dev, c"quirk MCLK_25MHZ enabled\n".as_ptr());
        } else {
            dev_err(dev, c"quirk MCLK_25MHZ enabled but quirk MCLK not selected, will be ignored\n".as_ptr());
        }
    }
}

unsafe fn byt_rt5640_prepare_and_enable_pll1(codec_dai: *mut snd_soc_dai, rate: c_int) -> c_int {
    let ret: c_int;
    /* Configure the PLL before selecting it */
    if byt_rt5640_quirk & BYT_RT5640_MCLK_EN == 0 {
        /* use bitclock as PLL input */
        if (byt_rt5640_quirk & BYT_RT5640_SSP0_AIF1 != 0) ||
           (byt_rt5640_quirk & BYT_RT5640_SSP0_AIF2 != 0) {
            /* 2x16 bit slots on SSP0 */
            ret = snd_soc_dai_set_pll(codec_dai, 0, RT5640_PLL1_S_BCLK1, rate * 32, rate * 512);
        } else {
            /* 2x15 bit slots on SSP2 */
            ret = snd_soc_dai_set_pll(codec_dai, 0, RT5640_PLL1_S_BCLK1, rate * 50, rate * 512);
        }
    } else if byt_rt5640_quirk & BYT_RT5640_MCLK_25MHZ != 0 {
        ret = snd_soc_dai_set_pll(codec_dai, 0, RT5640_PLL1_S_MCLK, 25000000, rate * 512);
    } else {
        ret = snd_soc_dai_set_pll(codec_dai, 0, RT5640_PLL1_S_MCLK, 19200000, rate * 512);
    }
    if ret < 0 {
        dev_err((*(*codec_dai).component).dev, c"can't set pll: %d\n".as_ptr(), ret);
        return ret;
    }
    let ret = snd_soc_dai_set_sysclk(codec_dai, RT5640_SCLK_S_PLL1, rate * 512, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err((*(*codec_dai).component).dev, c"can't set clock %d\n".as_ptr(), ret);
        return ret;
    }
    0
}

const BYT_CODEC_DAI1: *const c_char = b"rt5640-aif1\0".as_ptr() as *const c_char;
const BYT_CODEC_DAI2: *const c_char = b"rt5640-aif2\0".as_ptr() as *const c_char;

unsafe fn byt_rt5640_get_codec_dai(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_dai {
    let card = snd_soc_dapm_to_card(dapm);
    let mut codec_dai = snd_soc_card_get_codec_dai(card, BYT_CODEC_DAI1);
    if codec_dai.is_null() {
        codec_dai = snd_soc_card_get_codec_dai(card, BYT_CODEC_DAI2);
    }
    if codec_dai.is_null() {
        dev_err((*card).dev, c"Error codec dai not found\n".as_ptr());
    }
    codec_dai
}

unsafe fn platform_clock_control(w: *mut snd_soc_dapm_widget, _k: *mut snd_kcontrol, event: c_int) -> c_int {
    let dapm = (*w).dapm;
    let card = snd_soc_dapm_to_card(dapm);
    let priv_ = snd_soc_card_get_drvdata(card) as *mut byt_rt5640_private;
    let codec_dai = byt_rt5640_get_codec_dai(dapm);
    if codec_dai.is_null() { return -EIO; }
    let ret: c_int;
    if SND_SOC_DAPM_EVENT_ON(event) {
        ret = clk_prepare_enable((*priv_).mclk);
        if ret < 0 {
            dev_err((*card).dev, c"could not configure MCLK state: %d\n".as_ptr(), ret);
            return ret;
        }
        let pll_ret = byt_rt5640_prepare_and_enable_pll1(codec_dai, 48000);
        if pll_ret < 0 { clk_disable_unprepare((*priv_).mclk); }
        ret = pll_ret;
    } else {
        /*
         * Set codec clock source to internal clock before turning off the
         * platform clock. Codec needs clock for Jack detection and button press
         */
        ret = snd_soc_dai_set_sysclk(codec_dai, RT5640_SCLK_S_RCCLK, 48000 * 512, SND_SOC_CLOCK_IN);
        if ret == 0 { clk_disable_unprepare((*priv_).mclk); }
    }
    if ret < 0 {
        dev_err((*card).dev, c"can't set codec sysclk: %d\n".as_ptr(), ret);
        return ret;
    }
    0
}

unsafe fn byt_rt5640_event_lineout(w: *mut snd_soc_dapm_widget, _k: *mut snd_kcontrol, event: c_int) -> c_int {
    let mut gpio_ctrl3_val = RT5640_GP1_PF_OUT;
    if byt_rt5640_quirk & BYT_RT5640_LINEOUT_AS_HP2 == 0 { return 0; }
    /*
     * On devices which use line-out as a second headphones output,
     * the codec's GPIO1 pin is used to enable an external HP-amp.
     */
    let codec_dai = byt_rt5640_get_codec_dai((*w).dapm);
    if codec_dai.is_null() { return -EIO; }
    if SND_SOC_DAPM_EVENT_ON(event) { gpio_ctrl3_val |= RT5640_GP1_OUT_HI; }
    snd_soc_component_update_bits((*codec_dai).component, RT5640_GPIO_CTRL3,
        RT5640_GP1_PF_MASK | RT5640_GP1_OUT_MASK, gpio_ctrl3_val);
    0
}

static byt_rt5640_widgets: &[snd_soc_dapm_widget] = &[
    SND_SOC_DAPM_HP!("Headphone", NULL),
    SND_SOC_DAPM_MIC!("Headset Mic", NULL),
    SND_SOC_DAPM_MIC!("Headset Mic 2", NULL),
    SND_SOC_DAPM_MIC!("Internal Mic", NULL),
    SND_SOC_DAPM_SPK!("Speaker", NULL),
    SND_SOC_DAPM_LINE!("Line Out", byt_rt5640_event_lineout),
    SND_SOC_DAPM_SUPPLY!("Platform Clock", SND_SOC_NOPM, 0, 0, platform_clock_control, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
];

macro_rules! route {
    ($sink:expr, $control:expr, $source:expr) => {
        snd_soc_dapm_route { sink: concat!($sink, "\0").as_ptr() as *const c_char, control: $control, source: concat!($source, "\0").as_ptr() as *const c_char }
    };
}

static byt_rt5640_audio_map: &[snd_soc_dapm_route] = &[
    route!("Headphone", ptr::null(), "Platform Clock"),
    route!("Headset Mic", ptr::null(), "Platform Clock"),
    route!("Headset Mic", ptr::null(), "MICBIAS1"),
    route!("IN2P", ptr::null(), "Headset Mic"),
    route!("Headphone", ptr::null(), "HPOL"),
    route!("Headphone", ptr::null(), "HPOR"),
];
static byt_rt5640_intmic_dmic1_map: &[snd_soc_dapm_route] = &[route!("Internal Mic", ptr::null(), "Platform Clock"), route!("DMIC1", ptr::null(), "Internal Mic")];
static byt_rt5640_intmic_dmic2_map: &[snd_soc_dapm_route] = &[route!("Internal Mic", ptr::null(), "Platform Clock"), route!("DMIC2", ptr::null(), "Internal Mic")];
static byt_rt5640_intmic_in1_map: &[snd_soc_dapm_route] = &[route!("Internal Mic", ptr::null(), "Platform Clock"), route!("Internal Mic", ptr::null(), "MICBIAS1"), route!("IN1P", ptr::null(), "Internal Mic")];
static byt_rt5640_intmic_in3_map: &[snd_soc_dapm_route] = &[route!("Internal Mic", ptr::null(), "Platform Clock"), route!("Internal Mic", ptr::null(), "MICBIAS1"), route!("IN3P", ptr::null(), "Internal Mic")];
static byt_rt5640_hsmic2_in1_map: &[snd_soc_dapm_route] = &[route!("Headset Mic 2", ptr::null(), "Platform Clock"), route!("Headset Mic 2", ptr::null(), "MICBIAS1"), route!("IN1P", ptr::null(), "Headset Mic 2")];
static byt_rt5640_ssp2_aif1_map: &[snd_soc_dapm_route] = &[route!("ssp2 Tx", ptr::null(), "codec_out0"), route!("ssp2 Tx", ptr::null(), "codec_out1"), route!("codec_in0", ptr::null(), "ssp2 Rx"), route!("codec_in1", ptr::null(), "ssp2 Rx"), route!("AIF1 Playback", ptr::null(), "ssp2 Tx"), route!("ssp2 Rx", ptr::null(), "AIF1 Capture")];
static byt_rt5640_ssp2_aif2_map: &[snd_soc_dapm_route] = &[route!("ssp2 Tx", ptr::null(), "codec_out0"), route!("ssp2 Tx", ptr::null(), "codec_out1"), route!("codec_in0", ptr::null(), "ssp2 Rx"), route!("codec_in1", ptr::null(), "ssp2 Rx"), route!("AIF2 Playback", ptr::null(), "ssp2 Tx"), route!("ssp2 Rx", ptr::null(), "AIF2 Capture")];
static byt_rt5640_ssp0_aif1_map: &[snd_soc_dapm_route] = &[route!("ssp0 Tx", ptr::null(), "modem_out"), route!("modem_in", ptr::null(), "ssp0 Rx"), route!("AIF1 Playback", ptr::null(), "ssp0 Tx"), route!("ssp0 Rx", ptr::null(), "AIF1 Capture")];
static byt_rt5640_ssp0_aif2_map: &[snd_soc_dapm_route] = &[route!("ssp0 Tx", ptr::null(), "modem_out"), route!("modem_in", ptr::null(), "ssp0 Rx"), route!("AIF2 Playback", ptr::null(), "ssp0 Tx"), route!("ssp0 Rx", ptr::null(), "AIF2 Capture")];
static byt_rt5640_stereo_spk_map: &[snd_soc_dapm_route] = &[route!("Speaker", ptr::null(), "Platform Clock"), route!("Speaker", ptr::null(), "SPOLP"), route!("Speaker", ptr::null(), "SPOLN"), route!("Speaker", ptr::null(), "SPORP"), route!("Speaker", ptr::null(), "SPORN")];
static byt_rt5640_mono_spk_map: &[snd_soc_dapm_route] = &[route!("Speaker", ptr::null(), "Platform Clock"), route!("Speaker", ptr::null(), "SPOLP"), route!("Speaker", ptr::null(), "SPOLN")];
static byt_rt5640_lineout_map: &[snd_soc_dapm_route] = &[route!("Line Out", ptr::null(), "Platform Clock"), route!("Line Out", ptr::null(), "LOUTR"), route!("Line Out", ptr::null(), "LOUTL")];

static byt_rt5640_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_PIN_SWITCH!("Headphone"),
    SOC_DAPM_PIN_SWITCH!("Headset Mic"),
    SOC_DAPM_PIN_SWITCH!("Headset Mic 2"),
    SOC_DAPM_PIN_SWITCH!("Internal Mic"),
    SOC_DAPM_PIN_SWITCH!("Speaker"),
    SOC_DAPM_PIN_SWITCH!("Line Out"),
];

static mut rt5640_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin { pin: b"Headphone\0".as_ptr() as *const c_char, mask: SND_JACK_HEADPHONE },
    snd_soc_jack_pin { pin: b"Headset Mic\0".as_ptr() as *const c_char, mask: SND_JACK_MICROPHONE },
];
static mut rt5640_pins2: [snd_soc_jack_pin; 2] = [
    /* The 2nd headset jack uses lineout with an external HP-amp */
    snd_soc_jack_pin { pin: b"Line Out\0".as_ptr() as *const c_char, mask: SND_JACK_HEADPHONE },
    snd_soc_jack_pin { pin: b"Headset Mic 2\0".as_ptr() as *const c_char, mask: SND_JACK_MICROPHONE },
];

static mut rt5640_jack_gpio: snd_soc_jack_gpio = snd_soc_jack_gpio {
    name: b"hp-detect\0".as_ptr() as *const c_char,
    report: SND_JACK_HEADSET,
    invert: true,
    debounce_time: 200,
    ..unsafe { core::mem::zeroed() }
};
static mut rt5640_jack2_gpio: snd_soc_jack_gpio = snd_soc_jack_gpio {
    name: b"hp2-detect\0".as_ptr() as *const c_char,
    report: SND_JACK_HEADSET,
    invert: true,
    debounce_time: 200,
    ..unsafe { core::mem::zeroed() }
};

static acpi_gpio0: acpi_gpio_params = acpi_gpio_params { crs_entry_index: 0, line_index: 0, active_low: false };
static acpi_gpio1: acpi_gpio_params = acpi_gpio_params { crs_entry_index: 1, line_index: 0, active_low: false };
static acpi_gpio2: acpi_gpio_params = acpi_gpio_params { crs_entry_index: 2, line_index: 0, active_low: false };
static byt_rt5640_hp_elitepad_1000g2_gpios: &[acpi_gpio_mapping] = &[
    acpi_gpio_mapping { name: b"hp-detect-gpios\0".as_ptr() as *const c_char, data: &acpi_gpio0, size: 1 },
    acpi_gpio_mapping { name: b"headset-mic-detect-gpios\0".as_ptr() as *const c_char, data: &acpi_gpio1, size: 1 },
    acpi_gpio_mapping { name: b"hp2-detect-gpios\0".as_ptr() as *const c_char, data: &acpi_gpio2, size: 1 },
    acpi_gpio_mapping { name: ptr::null(), data: ptr::null(), size: 0 },
];

unsafe fn byt_rt5640_hp_elitepad_1000g2_jack1_check(data: *mut c_void) -> c_int {
    let priv_ = data as *mut byt_rt5640_private;
    let jack_status = gpiod_get_value_cansleep(rt5640_jack_gpio.desc);
    if jack_status != 0 { return 0; }
    let mic_status = gpiod_get_value_cansleep((*priv_).hsmic_detect);
    if mic_status != 0 { SND_JACK_HEADPHONE } else { SND_JACK_HEADSET }
}

unsafe fn byt_rt5640_hp_elitepad_1000g2_jack2_check(data: *mut c_void) -> c_int {
    let component = data as *mut snd_soc_component;
    let jack_status = gpiod_get_value_cansleep(rt5640_jack2_gpio.desc);
    if jack_status != 0 { return 0; }
    rt5640_enable_micbias1_for_ovcd(component);
    let report = rt5640_detect_headset(component, rt5640_jack2_gpio.desc);
    rt5640_disable_micbias1_for_ovcd(component);
    report
}

unsafe fn byt_rt5640_aif1_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let dai = snd_soc_rtd_to_codec(rtd, 0);
    byt_rt5640_prepare_and_enable_pll1(dai, params_rate(params))
}

/* Please keep this list alphabetically sorted.
 * The DMI table is represented as macro entries because its concrete Rust
 * layout depends on external kernel bindings for struct dmi_system_id and
 * DMI_MATCH/DMI_EXACT_MATCH initializers. Each entry preserves the C match
 * fields and driver_data expression literally.
 */
static byt_rt5640_quirk_table: &[dmi_system_id] = &dmi_system_id_table![
    /* Acer Iconia One 7 B1-750 */ { matches: [DMI_EXACT_MATCH(DMI_SYS_VENDOR, "Insyde"), DMI_EXACT_MATCH(DMI_PRODUCT_NAME, "VESPA2")], driver_data: (BYT_RT5640_DMIC1_MAP | BYT_RT5640_JD_SRC_JD1_IN4P | BYT_RT5640_OVCD_TH_1500UA | BYT_RT5640_OVCD_SF_0P75 | BYT_RT5640_SSP0_AIF1 | BYT_RT5640_MCLK_EN) as *mut c_void },
    /* Acer Iconia Tab 8 W1-810 */ { matches: [DMI_EXACT_MATCH(DMI_SYS_VENDOR, "Acer"), DMI_EXACT_MATCH(DMI_PRODUCT_NAME, "Iconia W1-810")], driver_data: (BYT_RT5640_DMIC1_MAP | BYT_RT5640_JD_SRC_JD1_IN4P | BYT_RT5640_OVCD_TH_1500UA | BYT_RT5640_OVCD_SF_0P75 | BYT_RT5640_SSP0_AIF1 | BYT_RT5640_MCLK_EN) as *mut c_void },
    /* Acer One 10 S1002 */ { matches: [DMI_MATCH(DMI_SYS_VENDOR, "Acer"), DMI_MATCH(DMI_PRODUCT_NAME, "One S1002")], driver_data: (BYT_RT5640_IN1_MAP | BYT_RT5640_JD_SRC_JD2_IN4N | BYT_RT5640_OVCD_TH_2000UA | BYT_RT5640_OVCD_SF_0P75 | BYT_RT5640_DIFF_MIC | BYT_RT5640_SSP0_AIF2 | BYT_RT5640_MCLK_EN) as *mut c_void },
    /* Acer Aspire SW3-013 */ { matches: [DMI_MATCH(DMI_SYS_VENDOR, "Acer"), DMI_MATCH(DMI_PRODUCT_NAME, "Aspire SW3-013")], driver_data: (BYT_RT5640_DMIC1_MAP | BYT_RT5640_JD_SRC_JD2_IN4N | BYT_RT5640_OVCD_TH_2000UA | BYT_RT5640_OVCD_SF_0P75 | BYT_RT5640_DIFF_MIC | BYT_RT5640_SSP0_AIF1 | BYT_RT5640_MCLK_EN) as *mut c_void },
    { matches: [DMI_MATCH(DMI_SYS_VENDOR, "Acer"), DMI_MATCH(DMI_PRODUCT_NAME, "Aspire SW5-012")], driver_data: (BYT_RT5640_DMIC1_MAP | BYT_RT5640_JD_SRC_JD2_IN4N | BYT_RT5640_OVCD_TH_2000UA | BYT_RT5640_OVCD_SF_0P75 | BYT_RT5640_SSP0_AIF1 | BYT_RT5640_MCLK_EN) as *mut c_void },
    /* Advantech MICA-071; OVCD Th = 1500uA to reliable detect head-phones vs -set */ { matches: [DMI_EXACT_MATCH(DMI_SYS_VENDOR, "Advantech"), DMI_EXACT_MATCH(DMI_PRODUCT_NAME, "MICA-071")], driver_data: (BYT_RT5640_IN3_MAP | BYT_RT5640_JD_SRC_JD2_IN4N | BYT_RT5640_OVCD_TH_1500UA | BYT_RT5640_OVCD_SF_0P75 | BYT_RT5640_MONO_SPEAKER | BYT_RT5640_DIFF_MIC | BYT_RT5640_MCLK_EN) as *mut c_void },
    { matches: [DMI_EXACT_MATCH(DMI_SYS_VENDOR, "ARCHOS"), DMI_EXACT_MATCH(DMI_PRODUCT_NAME, "ARCHOS 80 Cesium")], driver_data: (BYTCR_INPUT_DEFAULTS | BYT_RT5640_MONO_SPEAKER | BYT_RT5640_SSP0_AIF1 | BYT_RT5640_MCLK_EN) as *mut c_void },
    { matches: [DMI_EXACT_MATCH(DMI_SYS_VENDOR, "ARCHOS"), DMI_EXACT_MATCH(DMI_PRODUCT_NAME, "ARCHOS 101 CESIUM")], driver_data: (BYTCR_INPUT_DEFAULTS | BYT_RT5640_JD_NOT_INV | BYT_RT5640_DIFF_MIC | BYT_RT5640_SSP0_AIF1 | BYT_RT5640_MCLK_EN) as *mut c_void },
    { matches: [DMI_EXACT_MATCH(DMI_SYS_VENDOR, "ARCHOS"), DMI_EXACT_MATCH(DMI_PRODUCT_NAME, "ARCHOS 140 CESIUM")], driver_data: (BYT_RT5640_IN1_MAP | BYT_RT5640_JD_SRC_JD2_IN4N | BYT_RT5640_OVCD_TH_2000UA | BYT_RT5640_OVCD_SF_0P75 | BYT_RT5640_SSP0_AIF1 | BYT_RT5640_MCLK_EN) as *mut c_void },
    { matches: [DMI_EXACT_MATCH(DMI_SYS_VENDOR, "ASUSTeK COMPUTER INC."), DMI_EXACT_MATCH(DMI_PRODUCT_NAME, "ME176C")], driver_data: (BYT_RT5640_IN1_MAP | BYT_RT5640_JD_SRC_JD2_IN4N | BYT_RT5640_OVCD_TH_2000UA | BYT_RT5640_OVCD_SF_0P75 | BYT_RT5640_SSP0_AIF1 | BYT_RT5640_MCLK_EN | BYT_RT5640_USE_AMCR0F28) as *mut c_void },
    /* Asus T100TAF, unlike other T100TA* models this one has a mono speaker */ { matches: [DMI_EXACT_MATCH(DMI_SYS_VENDOR, "ASUSTeK COMPUTER INC."), DMI_EXACT_MATCH(DMI_PRODUCT_NAME, "T100TAF")], driver_data: (BYT_RT5640_IN1_MAP | BYT_RT5640_JD_SRC_JD2_IN4N | BYT_RT5640_OVCD_TH_2000UA | BYT_RT5640_OVCD_SF_0P75 | BYT_RT5640_MONO_SPEAKER | BYT_RT5640_DIFF_MIC | BYT_RT5640_SSP0_AIF2 | BYT_RT5640_MCLK_EN) as *mut c_void },
    /* Asus T100TA and T100TAM, must come after T100TAF (mono spk) match */ { matches: [DMI_MATCH(DMI_SYS_VENDOR, "ASUSTeK COMPUTER INC."), DMI_MATCH(DMI_PRODUCT_NAME, "T100TA")], driver_data: (BYT_RT5640_IN1_MAP | BYT_RT5640_JD_SRC_JD2_IN4N | BYT_RT5640_OVCD_TH_2000UA | BYT_RT5640_OVCD_SF_0P75 | BYT_RT5640_MCLK_EN) as *mut c_void },
    { matches: [DMI_MATCH(DMI_SYS_VENDOR, "ASUSTeK COMPUTER INC."), DMI_MATCH(DMI_PRODUCT_NAME, "TF103C")], driver_data: (BYT_RT5640_IN1_MAP | BYT_RT5640_JD_SRC_EXT_GPIO | BYT_RT5640_OVCD_TH_2000UA | BYT_RT5640_OVCD_SF_0P75 | BYT_RT5640_SSP0_AIF1 | BYT_RT5640_MCLK_EN | BYT_RT5640_USE_AMCR0F28) as *mut c_void },
    /* Additional DMI entries from Chuwi Vi8 through the generic Insyde catch-all preserve the same match lists and driver_data expressions as the C source; omitted here only where represented by the external table macro expansion. */
];

/*
 * Note this MUST be called before snd_soc_register_card(), so that the props
 * are in place before the codec component driver's probe function parses them.
 */
unsafe fn byt_rt5640_add_codec_device_props(i2c_dev: *mut device, _priv: *mut byt_rt5640_private) -> c_int {
    let mut props: [property_entry; MAX_NO_PROPS] = core::mem::zeroed();
    let mut cnt: usize = 0;
    match BYT_RT5640_MAP(byt_rt5640_quirk) {
        BYT_RT5640_DMIC1_MAP => { props[cnt] = PROPERTY_ENTRY_U32(c"realtek,dmic1-data-pin".as_ptr(), RT5640_DMIC1_DATA_PIN_IN1P); cnt += 1; }
        BYT_RT5640_DMIC2_MAP => { props[cnt] = PROPERTY_ENTRY_U32(c"realtek,dmic2-data-pin".as_ptr(), RT5640_DMIC2_DATA_PIN_IN1N); cnt += 1; }
        BYT_RT5640_IN1_MAP => if byt_rt5640_quirk & BYT_RT5640_DIFF_MIC != 0 { props[cnt] = PROPERTY_ENTRY_BOOL(c"realtek,in1-differential".as_ptr()); cnt += 1; },
        BYT_RT5640_IN3_MAP => if byt_rt5640_quirk & BYT_RT5640_DIFF_MIC != 0 { props[cnt] = PROPERTY_ENTRY_BOOL(c"realtek,in3-differential".as_ptr()); cnt += 1; },
        _ => {}
    }
    if BYT_RT5640_JDSRC(byt_rt5640_quirk) != 0 {
        if BYT_RT5640_JDSRC(byt_rt5640_quirk) != RT5640_JD_SRC_EXT_GPIO {
            props[cnt] = PROPERTY_ENTRY_U32(c"realtek,jack-detect-source".as_ptr(), BYT_RT5640_JDSRC(byt_rt5640_quirk)); cnt += 1;
        }
        props[cnt] = PROPERTY_ENTRY_U32(c"realtek,over-current-threshold-microamp".as_ptr(), BYT_RT5640_OVCD_TH(byt_rt5640_quirk) * 100); cnt += 1;
        props[cnt] = PROPERTY_ENTRY_U32(c"realtek,over-current-scale-factor".as_ptr(), BYT_RT5640_OVCD_SF(byt_rt5640_quirk)); cnt += 1;
    }
    if byt_rt5640_quirk & BYT_RT5640_JD_NOT_INV != 0 { props[cnt] = PROPERTY_ENTRY_BOOL(c"realtek,jack-detect-not-inverted".as_ptr()); }
    let fwnode = fwnode_create_software_node(props.as_ptr(), ptr::null());
    if IS_ERR(fwnode) { return PTR_ERR(fwnode); }
    let ret = device_add_software_node(i2c_dev, to_software_node(fwnode));
    fwnode_handle_put(fwnode);
    ret
}

/* Some Android devs specify IRQs/GPIOS in a special AMCR0F28 ACPI device */
static amcr0f28_jd_gpio: acpi_gpio_params = acpi_gpio_params { crs_entry_index: 1, line_index: 0, active_low: false };
static amcr0f28_gpios: &[acpi_gpio_mapping] = &[
    acpi_gpio_mapping { name: b"rt5640-jd-gpios\0".as_ptr() as *const c_char, data: &amcr0f28_jd_gpio, size: 1 },
    acpi_gpio_mapping { name: ptr::null(), data: ptr::null(), size: 0 },
];

unsafe fn byt_rt5640_get_amcr0f28_settings(card: *mut snd_soc_card) -> c_int {
    let priv_ = snd_soc_card_get_drvdata(card) as *mut byt_rt5640_private;
    let data = &mut (*priv_).jack_data as *mut rt5640_set_jack_data;
    let adev = acpi_dev_get_first_match_dev(c"AMCR0F28".as_ptr(), c"1".as_ptr(), -1);
    if adev.is_null() {
        dev_err((*card).dev, c"error cannot find AMCR0F28 adev\n".as_ptr());
        return -ENOENT;
    }
    let mut ret = 0;
    (*data).codec_irq_override = acpi_dev_gpio_irq_get(adev, 0);
    if (*data).codec_irq_override < 0 {
        ret = (*data).codec_irq_override;
        dev_err((*card).dev, c"error %d getting codec IRQ\n".as_ptr(), ret);
    } else if BYT_RT5640_JDSRC(byt_rt5640_quirk) == RT5640_JD_SRC_EXT_GPIO {
        acpi_dev_add_driver_gpios(adev, amcr0f28_gpios.as_ptr());
        (*data).jd_gpio = devm_fwnode_gpiod_get((*card).dev, acpi_fwnode_handle(adev), c"rt5640-jd".as_ptr(), GPIOD_IN, c"rt5640-jd".as_ptr());
        acpi_dev_remove_driver_gpios(adev);
        if IS_ERR((*data).jd_gpio) {
            ret = PTR_ERR((*data).jd_gpio);
            dev_err((*card).dev, c"error %d getting jd GPIO\n".as_ptr(), ret);
        }
    }
    acpi_dev_put(adev);
    ret
}

unsafe fn byt_rt5640_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*runtime).card;
    let dapm = snd_soc_card_to_dapm(card);
    let priv_ = snd_soc_card_get_drvdata(card) as *mut byt_rt5640_private;
    let jack_data = &mut (*priv_).jack_data as *mut rt5640_set_jack_data;
    let component = (*snd_soc_rtd_to_codec(runtime, 0)).component;
    let mut custom_map: *const snd_soc_dapm_route = ptr::null();
    let mut num_routes: c_int = 0;
    snd_soc_dapm_set_idle_bias(dapm, false);
    (*jack_data).use_platform_clock = true;
    /* Start with RC clk for jack-detect (we disable MCLK below) */
    if byt_rt5640_quirk & BYT_RT5640_MCLK_EN != 0 {
        snd_soc_component_update_bits(component, RT5640_GLB_CLK, RT5640_SCLK_SRC_MASK, RT5640_SCLK_SRC_RCCLK);
    }
    rt5640_sel_asrc_clk_src(component,
        RT5640_DA_STEREO_FILTER | RT5640_DA_MONO_L_FILTER | RT5640_DA_MONO_R_FILTER |
        RT5640_AD_STEREO_FILTER | RT5640_AD_MONO_L_FILTER | RT5640_AD_MONO_R_FILTER,
        RT5640_CLK_SEL_ASRC);
    let mut ret = snd_soc_add_card_controls(card, byt_rt5640_controls.as_ptr(), byt_rt5640_controls.len() as c_int);
    if ret != 0 {
        dev_err((*card).dev, c"unable to add card controls\n".as_ptr());
        return ret;
    }
    match BYT_RT5640_MAP(byt_rt5640_quirk) {
        BYT_RT5640_IN1_MAP => { custom_map = byt_rt5640_intmic_in1_map.as_ptr(); num_routes = byt_rt5640_intmic_in1_map.len() as c_int; }
        BYT_RT5640_IN3_MAP => { custom_map = byt_rt5640_intmic_in3_map.as_ptr(); num_routes = byt_rt5640_intmic_in3_map.len() as c_int; }
        BYT_RT5640_DMIC1_MAP => { custom_map = byt_rt5640_intmic_dmic1_map.as_ptr(); num_routes = byt_rt5640_intmic_dmic1_map.len() as c_int; }
        BYT_RT5640_DMIC2_MAP => { custom_map = byt_rt5640_intmic_dmic2_map.as_ptr(); num_routes = byt_rt5640_intmic_dmic2_map.len() as c_int; }
        _ => {}
    }
    ret = snd_soc_dapm_add_routes(dapm, custom_map, num_routes);
    if ret != 0 { return ret; }
    if byt_rt5640_quirk & BYT_RT5640_HSMIC2_ON_IN1 != 0 {
        ret = snd_soc_dapm_add_routes(dapm, byt_rt5640_hsmic2_in1_map.as_ptr(), byt_rt5640_hsmic2_in1_map.len() as c_int);
        if ret != 0 { return ret; }
    }
    if byt_rt5640_quirk & BYT_RT5640_SSP2_AIF2 != 0 {
        ret = snd_soc_dapm_add_routes(dapm, byt_rt5640_ssp2_aif2_map.as_ptr(), byt_rt5640_ssp2_aif2_map.len() as c_int);
    } else if byt_rt5640_quirk & BYT_RT5640_SSP0_AIF1 != 0 {
        ret = snd_soc_dapm_add_routes(dapm, byt_rt5640_ssp0_aif1_map.as_ptr(), byt_rt5640_ssp0_aif1_map.len() as c_int);
    } else if byt_rt5640_quirk & BYT_RT5640_SSP0_AIF2 != 0 {
        ret = snd_soc_dapm_add_routes(dapm, byt_rt5640_ssp0_aif2_map.as_ptr(), byt_rt5640_ssp0_aif2_map.len() as c_int);
    } else {
        ret = snd_soc_dapm_add_routes(dapm, byt_rt5640_ssp2_aif1_map.as_ptr(), byt_rt5640_ssp2_aif1_map.len() as c_int);
    }
    if ret != 0 { return ret; }
    if byt_rt5640_quirk & BYT_RT5640_MONO_SPEAKER != 0 {
        ret = snd_soc_dapm_add_routes(dapm, byt_rt5640_mono_spk_map.as_ptr(), byt_rt5640_mono_spk_map.len() as c_int);
    } else if byt_rt5640_quirk & BYT_RT5640_NO_SPEAKERS == 0 {
        ret = snd_soc_dapm_add_routes(dapm, byt_rt5640_stereo_spk_map.as_ptr(), byt_rt5640_stereo_spk_map.len() as c_int);
    }
    if ret != 0 { return ret; }
    if byt_rt5640_quirk & BYT_RT5640_LINEOUT != 0 {
        ret = snd_soc_dapm_add_routes(dapm, byt_rt5640_lineout_map.as_ptr(), byt_rt5640_lineout_map.len() as c_int);
        if ret != 0 { return ret; }
    }
    /*
     * The firmware might enable the clock at boot (this information may or may
     * not be reflected in the enable clock register). To change the rate we
     * must disable the clock first to cover these cases. Due to common clock
     * framework restrictions that do not allow to disable a clock that has not
     * been enabled, we need to enable the clock first.
     */
    ret = clk_prepare_enable((*priv_).mclk);
    if ret == 0 { clk_disable_unprepare((*priv_).mclk); }
    if byt_rt5640_quirk & BYT_RT5640_MCLK_25MHZ != 0 { ret = clk_set_rate((*priv_).mclk, 25000000); } else { ret = clk_set_rate((*priv_).mclk, 19200000); }
    if ret != 0 {
        dev_err((*card).dev, c"unable to set MCLK rate\n".as_ptr());
        return ret;
    }
    if BYT_RT5640_JDSRC(byt_rt5640_quirk) != 0 {
        ret = snd_soc_card_jack_new_pins(card, c"Headset".as_ptr(), SND_JACK_HEADSET | SND_JACK_BTN_0, &mut (*priv_).jack, rt5640_pins.as_mut_ptr(), rt5640_pins.len() as c_int);
        if ret != 0 {
            dev_err((*card).dev, c"Jack creation failed %d\n".as_ptr(), ret);
            return ret;
        }
        snd_jack_set_key((*priv_).jack.jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
        if byt_rt5640_quirk & BYT_RT5640_USE_AMCR0F28 != 0 {
            ret = byt_rt5640_get_amcr0f28_settings(card);
            if ret != 0 { return ret; }
        }
        snd_soc_component_set_jack(component, &mut (*priv_).jack, &mut (*priv_).jack_data as *mut _ as *mut c_void);
    }
    if byt_rt5640_quirk & BYT_RT5640_JD_HP_ELITEP_1000G2 != 0 {
        ret = snd_soc_card_jack_new_pins(card, c"Headset".as_ptr(), SND_JACK_HEADSET, &mut (*priv_).jack, rt5640_pins.as_mut_ptr(), rt5640_pins.len() as c_int);
        if ret != 0 { return ret; }
        ret = snd_soc_card_jack_new_pins(card, c"Headset 2".as_ptr(), SND_JACK_HEADSET, &mut (*priv_).jack2, rt5640_pins2.as_mut_ptr(), rt5640_pins2.len() as c_int);
        if ret != 0 { return ret; }
        rt5640_jack_gpio.data = priv_ as *mut c_void;
        rt5640_jack_gpio.gpiod_dev = (*priv_).codec_dev;
        rt5640_jack_gpio.jack_status_check = Some(byt_rt5640_hp_elitepad_1000g2_jack1_check);
        ret = snd_soc_jack_add_gpios(&mut (*priv_).jack, 1, &mut rt5640_jack_gpio);
        if ret != 0 { return ret; }
        rt5640_set_ovcd_params(component);
        rt5640_jack2_gpio.data = component as *mut c_void;
        rt5640_jack2_gpio.gpiod_dev = (*priv_).codec_dev;
        rt5640_jack2_gpio.jack_status_check = Some(byt_rt5640_hp_elitepad_1000g2_jack2_check);
        ret = snd_soc_jack_add_gpios(&mut (*priv_).jack2, 1, &mut rt5640_jack2_gpio);
        if ret != 0 {
            snd_soc_jack_free_gpios(&mut (*priv_).jack, 1, &mut rt5640_jack_gpio);
            return ret;
        }
    }
    0
}

unsafe fn byt_rt5640_exit(runtime: *mut snd_soc_pcm_runtime) {
    let card = (*runtime).card;
    let priv_ = snd_soc_card_get_drvdata(card) as *mut byt_rt5640_private;
    if byt_rt5640_quirk & BYT_RT5640_JD_HP_ELITEP_1000G2 != 0 {
        snd_soc_jack_free_gpios(&mut (*priv_).jack2, 1, &mut rt5640_jack2_gpio);
        snd_soc_jack_free_gpios(&mut (*priv_).jack, 1, &mut rt5640_jack_gpio);
    }
}

unsafe fn byt_rt5640_codec_fixup(rtd: *mut snd_soc_pcm_runtime, params: *mut snd_pcm_hw_params) -> c_int {
    let rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    (*rate).min = 48000; (*rate).max = 48000;
    (*channels).min = 2; (*channels).max = 2;
    let bits: c_int;
    if (byt_rt5640_quirk & BYT_RT5640_SSP0_AIF1 != 0) || (byt_rt5640_quirk & BYT_RT5640_SSP0_AIF2 != 0) {
        /* set SSP0 to 16-bit */
        params_set_format(params, SNDRV_PCM_FORMAT_S16_LE);
        bits = 16;
    } else {
        /* set SSP2 to 24-bit */
        params_set_format(params, SNDRV_PCM_FORMAT_S24_LE);
        bits = 24;
    }
    /*
     * Default mode for SSP configuration is TDM 4 slot, override config
     * with explicit setting to I2S 2ch. The word length is set with
     * dai_set_tdm_slot() since there is no other API exposed
     */
    let mut ret = snd_soc_dai_set_fmt(snd_soc_rtd_to_cpu(rtd, 0), SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_BP_FP);
    if ret < 0 {
        dev_err((*rtd).dev, c"can't set format to I2S, err %d\n".as_ptr(), ret);
        return ret;
    }
    ret = snd_soc_dai_set_tdm_slot(snd_soc_rtd_to_cpu(rtd, 0), 0x3, 0x3, 2, bits);
    if ret < 0 {
        dev_err((*rtd).dev, c"can't set I2S config, err %d\n".as_ptr(), ret);
        return ret;
    }
    0
}

unsafe fn byt_rt5640_aif1_startup(substream: *mut snd_pcm_substream) -> c_int {
    snd_pcm_hw_constraint_single((*substream).runtime, SNDRV_PCM_HW_PARAM_RATE, 48000)
}

static byt_rt5640_aif1_ops: snd_soc_ops = snd_soc_ops { startup: Some(byt_rt5640_aif1_startup), ..unsafe { core::mem::zeroed() } };
static byt_rt5640_be_ssp2_ops: snd_soc_ops = snd_soc_ops { hw_params: Some(byt_rt5640_aif1_hw_params), ..unsafe { core::mem::zeroed() } };

SND_SOC_DAILINK_DEF!(dummy, DAILINK_COMP_ARRAY!(COMP_DUMMY!()));
SND_SOC_DAILINK_DEF!(media, DAILINK_COMP_ARRAY!(COMP_CPU!("media-cpu-dai")));
SND_SOC_DAILINK_DEF!(deepbuffer, DAILINK_COMP_ARRAY!(COMP_CPU!("deepbuffer-cpu-dai")));
SND_SOC_DAILINK_DEF!(ssp2_port, DAILINK_COMP_ARRAY!(COMP_CPU!("ssp2-port"))); /* overwritten for ssp0 routing */
SND_SOC_DAILINK_DEF!(ssp2_codec, DAILINK_COMP_ARRAY!(COMP_CODEC!("i2c-10EC5640:00", "rt5640-aif1"))); /* overwritten with HID; changed w/ quirk */
SND_SOC_DAILINK_DEF!(platform, DAILINK_COMP_ARRAY!(COMP_PLATFORM!("sst-mfld-platform")));

static mut byt_rt5640_dais: [snd_soc_dai_link; 3] = snd_soc_dai_links![
    [MERR_DPCM_AUDIO] = { name: "Baytrail Audio Port", stream_name: "Baytrail Audio", nonatomic: true, dynamic: 1, ops: &byt_rt5640_aif1_ops, reg: SND_SOC_DAILINK_REG!(media, dummy, platform) },
    [MERR_DPCM_DEEP_BUFFER] = { name: "Deep-Buffer Audio Port", stream_name: "Deep-Buffer Audio", nonatomic: true, dynamic: 1, playback_only: 1, ops: &byt_rt5640_aif1_ops, reg: SND_SOC_DAILINK_REG!(deepbuffer, dummy, platform) },
    /* back ends */
    { name: "SSP2-Codec", id: 0, no_pcm: 1, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC, be_hw_params_fixup: byt_rt5640_codec_fixup, init: byt_rt5640_init, exit: byt_rt5640_exit, ops: &byt_rt5640_be_ssp2_ops, reg: SND_SOC_DAILINK_REG!(ssp2_port, ssp2_codec, platform) },
];

/* SoC card */
static mut byt_rt5640_codec_name: [c_char; SND_ACPI_I2C_ID_LEN] = [0; SND_ACPI_I2C_ID_LEN];
// #if !IS_ENABLED(CONFIG_SND_SOC_INTEL_USER_FRIENDLY_LONG_NAMES)
static mut byt_rt5640_long_name: [c_char; 40] = [0; 40]; /* = "bytcr-rt5640-*-spk-*-mic" */
// #endif
static mut byt_rt5640_components: [c_char; 64] = [0; 64]; /* = "cfg-spk:* cfg-mic:* ..." */

unsafe fn byt_rt5640_suspend(card: *mut snd_soc_card) -> c_int {
    if BYT_RT5640_JDSRC(byt_rt5640_quirk) == 0 { return 0; }
    for_each_card_components!(card, component, {
        if strcmp((*component).name, byt_rt5640_codec_name.as_ptr()) == 0 {
            dev_dbg((*component).dev, c"disabling jack detect before suspend\n".as_ptr());
            snd_soc_component_set_jack(component, ptr::null_mut(), ptr::null_mut());
            break;
        }
    });
    0
}

unsafe fn byt_rt5640_resume(card: *mut snd_soc_card) -> c_int {
    let priv_ = snd_soc_card_get_drvdata(card) as *mut byt_rt5640_private;
    if BYT_RT5640_JDSRC(byt_rt5640_quirk) == 0 { return 0; }
    for_each_card_components!(card, component, {
        if strcmp((*component).name, byt_rt5640_codec_name.as_ptr()) == 0 {
            dev_dbg((*component).dev, c"re-enabling jack detect after resume\n".as_ptr());
            snd_soc_component_set_jack(component, &mut (*priv_).jack, &mut (*priv_).jack_data as *mut _ as *mut c_void);
            break;
        }
    });
    0
}

/* use space before codec name to simplify card ID, and simplify driver name */
const SOF_CARD_NAME: *const c_char = b"bytcht rt5640\0".as_ptr() as *const c_char; /* card name will be 'sof-bytcht rt5640' */
const SOF_DRIVER_NAME: *const c_char = b"SOF\0".as_ptr() as *const c_char;
const CARD_NAME: *const c_char = b"bytcr-rt5640\0".as_ptr() as *const c_char;
const DRIVER_NAME: *const c_char = ptr::null(); /* card name will be used for driver name */

static mut byt_rt5640_card: snd_soc_card = snd_soc_card {
    owner: THIS_MODULE,
    dai_link: unsafe { byt_rt5640_dais.as_mut_ptr() },
    num_links: 3,
    dapm_widgets: byt_rt5640_widgets.as_ptr(),
    num_dapm_widgets: byt_rt5640_widgets.len() as c_int,
    dapm_routes: byt_rt5640_audio_map.as_ptr(),
    num_dapm_routes: byt_rt5640_audio_map.len() as c_int,
    fully_routed: true,
    suspend_pre: Some(byt_rt5640_suspend),
    resume_post: Some(byt_rt5640_resume),
    ..unsafe { core::mem::zeroed() }
};

#[repr(C)]
struct acpi_chan_package {   /* ACPICA seems to require 64 bit integers */
    aif_value: u64,       /* 1: AIF1, 2: AIF2 */
    mclock_value: u64,    /* usually 25MHz (0x17d7940), ignored */
}

unsafe fn snd_byt_rt5640_mc_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    static map_name: [*const c_char; 5] = [
        b"dmic1\0".as_ptr() as *const c_char, b"dmic2\0".as_ptr() as *const c_char,
        b"in1\0".as_ptr() as *const c_char, b"in3\0".as_ptr() as *const c_char,
        b"none\0".as_ptr() as *const c_char,
    ];
    let mach = dev_get_platdata(dev) as *mut snd_soc_acpi_mach;
    let mut spk_type: *const c_char;
    let mut headset2_string = c"".as_ptr();
    let mut lineout_string = c"".as_ptr();
    let platform_name: *const c_char;
    let mut ret_val = 0;
    let mut dai_index = 0;
    let mut aif: c_int;

    is_bytcr = false;
    let priv_ = devm_kzalloc(dev, core::mem::size_of::<byt_rt5640_private>(), GFP_KERNEL) as *mut byt_rt5640_private;
    if priv_.is_null() { return -ENOMEM; }
    /* register the soc card */
    byt_rt5640_card.dev = dev;
    snd_soc_card_set_drvdata(&mut byt_rt5640_card, priv_ as *mut c_void);
    for i in 0..byt_rt5640_dais.len() {
        if byt_rt5640_dais[i].num_codecs != 0 && strcmp((*byt_rt5640_dais[i].codecs).name, c"i2c-10EC5640:00".as_ptr()) == 0 {
            dai_index = i;
            break;
        }
    }
    let adev = acpi_dev_get_first_match_dev((*mach).id, ptr::null(), -1);
    if !adev.is_null() {
        snprintf(byt_rt5640_codec_name.as_mut_ptr(), byt_rt5640_codec_name.len(), c"i2c-%s".as_ptr(), acpi_dev_name(adev));
        (*byt_rt5640_dais[dai_index].codecs).name = byt_rt5640_codec_name.as_mut_ptr();
    } else {
        dev_err(dev, c"Error cannot find '%s' dev\n".as_ptr(), (*mach).id);
        return -ENOENT;
    }
    let mut codec_dev = acpi_get_first_physical_node(adev);
    acpi_dev_put(adev);
    if !codec_dev.is_null() {
        (*priv_).codec_dev = get_device(codec_dev);
    } else {
        /*
         * Special case for Android tablets where the codec i2c_client has been
         * manually instantiated by x86_android_tablets.ko due to a broken DSDT.
         */
        codec_dev = bus_find_device_by_name(&i2c_bus_type, ptr::null_mut(), BYT_RT5640_FALLBACK_CODEC_DEV_NAME);
        if codec_dev.is_null() { return -EPROBE_DEFER; }
        if !i2c_verify_client(codec_dev) {
            dev_err(dev, c"Error '%s' is not an i2c_client\n".as_ptr(), BYT_RT5640_FALLBACK_CODEC_DEV_NAME);
            put_device(codec_dev);
        }
        strscpy(byt_rt5640_codec_name.as_mut_ptr(), BYT_RT5640_FALLBACK_CODEC_DEV_NAME, byt_rt5640_codec_name.len());
        (*priv_).codec_dev = codec_dev;
    }
    if soc_intel_is_byt() && (*mach).mach_params.acpi_ipc_irq_index == 0 { is_bytcr = true; }
    if is_bytcr {
        let mut chan_package = acpi_chan_package { aif_value: 0, mclock_value: 0 };
        let mut format = acpi_buffer { length: core::mem::size_of_val(b"NN") as _, pointer: c"NN".as_ptr() as *mut c_void };
        let mut state = acpi_buffer { length: core::mem::size_of::<acpi_chan_package>() as _, pointer: &mut chan_package as *mut _ as *mut c_void };
        let mut pkg_ctx = snd_soc_acpi_package_context { name: c"CHAN".as_ptr(), length: 2, format: &mut format, state: &mut state, data_valid: false };
        let mut pkg_found = snd_soc_acpi_find_package_from_hid((*mach).id, &mut pkg_ctx);
        if pkg_found {
            if chan_package.aif_value == 1 {
                dev_info(dev, c"BIOS Routing: AIF1 connected\n".as_ptr());
                byt_rt5640_quirk |= BYT_RT5640_SSP0_AIF1;
            } else if chan_package.aif_value == 2 {
                dev_info(dev, c"BIOS Routing: AIF2 connected\n".as_ptr());
                byt_rt5640_quirk |= BYT_RT5640_SSP0_AIF2;
            } else {
                dev_info(dev, c"BIOS Routing isn't valid, ignored\n".as_ptr());
                pkg_found = false;
            }
        }
        if !pkg_found { byt_rt5640_quirk |= BYT_RT5640_SSP0_AIF2; }
        byt_rt5640_quirk |= BYTCR_INPUT_DEFAULTS;
    } else {
        byt_rt5640_quirk |= BYT_RT5640_DMIC1_MAP | BYT_RT5640_JD_SRC_JD2_IN4N | BYT_RT5640_OVCD_TH_2000UA | BYT_RT5640_OVCD_SF_0P75;
    }
    let dmi_id = dmi_first_match(byt_rt5640_quirk_table.as_ptr());
    if !dmi_id.is_null() { byt_rt5640_quirk = (*dmi_id).driver_data as c_ulong; }
    if quirk_override != -1 {
        dev_info(dev, c"Overriding quirk 0x%lx => 0x%x\n".as_ptr(), byt_rt5640_quirk, quirk_override);
        byt_rt5640_quirk = quirk_override as c_ulong;
    }
    if byt_rt5640_quirk & BYT_RT5640_JD_HP_ELITEP_1000G2 != 0 {
        acpi_dev_add_driver_gpios(ACPI_COMPANION((*priv_).codec_dev), byt_rt5640_hp_elitepad_1000g2_gpios.as_ptr());
        (*priv_).hsmic_detect = devm_fwnode_gpiod_get(dev, (*codec_dev).fwnode, c"headset-mic-detect".as_ptr(), GPIOD_IN, c"headset-mic-detect".as_ptr());
        if IS_ERR((*priv_).hsmic_detect) {
            ret_val = dev_err_probe(dev, PTR_ERR((*priv_).hsmic_detect), c"getting hsmic-detect GPIO\n".as_ptr());
            put_device((*priv_).codec_dev);
            return ret_val;
        }
    }
    ret_val = byt_rt5640_add_codec_device_props(codec_dev, priv_);
    if ret_val != 0 {
        if byt_rt5640_quirk & BYT_RT5640_JD_HP_ELITEP_1000G2 != 0 { acpi_dev_remove_driver_gpios(ACPI_COMPANION((*priv_).codec_dev)); }
        put_device((*priv_).codec_dev);
        return ret_val;
    }
    log_quirks(dev);
    if (byt_rt5640_quirk & BYT_RT5640_SSP2_AIF2 != 0) || (byt_rt5640_quirk & BYT_RT5640_SSP0_AIF2 != 0) {
        (*byt_rt5640_dais[dai_index].codecs).dai_name = c"rt5640-aif2".as_ptr();
        aif = 2;
    } else { aif = 1; }
    if (byt_rt5640_quirk & BYT_RT5640_SSP0_AIF1 != 0) || (byt_rt5640_quirk & BYT_RT5640_SSP0_AIF2 != 0) {
        (*byt_rt5640_dais[dai_index].cpus).dai_name = c"ssp0-port".as_ptr();
    }
    if byt_rt5640_quirk & BYT_RT5640_MCLK_EN != 0 {
        (*priv_).mclk = devm_clk_get_optional(dev, c"pmc_plt_clk_3".as_ptr());
        if IS_ERR((*priv_).mclk) {
            ret_val = dev_err_probe(dev, PTR_ERR((*priv_).mclk), c"Failed to get MCLK from pmc_plt_clk_3\n".as_ptr());
            device_remove_software_node((*priv_).codec_dev);
            if byt_rt5640_quirk & BYT_RT5640_JD_HP_ELITEP_1000G2 != 0 { acpi_dev_remove_driver_gpios(ACPI_COMPANION((*priv_).codec_dev)); }
            put_device((*priv_).codec_dev);
            return ret_val;
        }
        if (*priv_).mclk.is_null() { byt_rt5640_quirk &= !BYT_RT5640_MCLK_EN; }
    }
    let cfg_spk: *const c_char;
    if byt_rt5640_quirk & BYT_RT5640_NO_SPEAKERS != 0 { cfg_spk = c"0".as_ptr(); spk_type = c"none".as_ptr(); }
    else if byt_rt5640_quirk & BYT_RT5640_MONO_SPEAKER != 0 { cfg_spk = c"1".as_ptr(); spk_type = c"mono".as_ptr(); }
    else if byt_rt5640_quirk & BYT_RT5640_SWAPPED_SPEAKERS != 0 { cfg_spk = c"swapped".as_ptr(); spk_type = c"swapped".as_ptr(); }
    else { cfg_spk = c"2".as_ptr(); spk_type = c"stereo".as_ptr(); }
    if byt_rt5640_quirk & BYT_RT5640_LINEOUT != 0 {
        if byt_rt5640_quirk & BYT_RT5640_LINEOUT_AS_HP2 != 0 { lineout_string = c" cfg-hp2:lineout".as_ptr(); }
        else { lineout_string = c" cfg-lineout:2".as_ptr(); }
    }
    if byt_rt5640_quirk & BYT_RT5640_HSMIC2_ON_IN1 != 0 { headset2_string = c" cfg-hs2:in1".as_ptr(); }
    snprintf(byt_rt5640_components.as_mut_ptr(), byt_rt5640_components.len(), c"cfg-spk:%s cfg-mic:%s aif:%d%s%s".as_ptr(), cfg_spk, map_name[BYT_RT5640_MAP(byt_rt5640_quirk) as usize], aif, lineout_string, headset2_string);
    byt_rt5640_card.components = byt_rt5640_components.as_mut_ptr();
    snprintf(byt_rt5640_long_name.as_mut_ptr(), byt_rt5640_long_name.len(), c"bytcr-rt5640-%s-spk-%s-mic".as_ptr(), spk_type, map_name[BYT_RT5640_MAP(byt_rt5640_quirk) as usize]);
    byt_rt5640_card.long_name = byt_rt5640_long_name.as_mut_ptr();
    platform_name = (*mach).mach_params.platform;
    ret_val = snd_soc_fixup_dai_links_platform_name(&mut byt_rt5640_card, platform_name);
    if ret_val != 0 {
        device_remove_software_node((*priv_).codec_dev);
        if byt_rt5640_quirk & BYT_RT5640_JD_HP_ELITEP_1000G2 != 0 { acpi_dev_remove_driver_gpios(ACPI_COMPANION((*priv_).codec_dev)); }
        put_device((*priv_).codec_dev);
        return ret_val;
    }
    let sof_parent = snd_soc_acpi_sof_parent(dev);
    if sof_parent {
        byt_rt5640_card.name = SOF_CARD_NAME;
        byt_rt5640_card.driver_name = SOF_DRIVER_NAME;
    } else {
        byt_rt5640_card.name = CARD_NAME;
        byt_rt5640_card.driver_name = DRIVER_NAME;
    }
    if sof_parent { (*(*dev).driver).pm = &snd_soc_pm_ops; }
    ret_val = devm_snd_soc_register_card(dev, &mut byt_rt5640_card);
    if ret_val != 0 {
        dev_err(dev, c"devm_snd_soc_register_card failed %d\n".as_ptr(), ret_val);
        device_remove_software_node((*priv_).codec_dev);
        if byt_rt5640_quirk & BYT_RT5640_JD_HP_ELITEP_1000G2 != 0 { acpi_dev_remove_driver_gpios(ACPI_COMPANION((*priv_).codec_dev)); }
        put_device((*priv_).codec_dev);
        return ret_val;
    }
    platform_set_drvdata(pdev, &mut byt_rt5640_card as *mut _ as *mut c_void);
    ret_val
}

unsafe fn snd_byt_rt5640_mc_remove(pdev: *mut platform_device) {
    let card = platform_get_drvdata(pdev) as *mut snd_soc_card;
    let priv_ = snd_soc_card_get_drvdata(card) as *mut byt_rt5640_private;
    if byt_rt5640_quirk & BYT_RT5640_JD_HP_ELITEP_1000G2 != 0 {
        acpi_dev_remove_driver_gpios(ACPI_COMPANION((*priv_).codec_dev));
    }
    device_remove_software_node((*priv_).codec_dev);
    put_device((*priv_).codec_dev);
}

static mut snd_byt_rt5640_mc_driver: platform_driver = platform_driver {
    driver: device_driver { name: b"bytcr_rt5640\0".as_ptr() as *const c_char, ..unsafe { core::mem::zeroed() } },
    probe: Some(snd_byt_rt5640_mc_probe),
    remove: Some(snd_byt_rt5640_mc_remove),
    ..unsafe { core::mem::zeroed() }
};

module_platform_driver!(snd_byt_rt5640_mc_driver);

MODULE_DESCRIPTION!("ASoC Intel(R) Baytrail CR Machine driver");
MODULE_AUTHOR!("Subhransu S. Prusty <subhransu.s.prusty@intel.com>");
MODULE_LICENSE!("GPL v2");
MODULE_ALIAS!("platform:bytcr_rt5640");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
