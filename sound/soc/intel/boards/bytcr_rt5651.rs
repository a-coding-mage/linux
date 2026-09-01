// SPDX-License-Identifier: GPL-2.0-only
/*
 *  bytcr_rt5651.rs - ASoc Machine driver for Intel Byt CR platform
 *  (derived from bytcr_rt5640.c)
 *
 *  Copyright (C) 2015 Intel Corp
 *  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

/* Rust translation of bytcr_rt5651.c. Kernel headers are external dependencies. */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const fn bit(n: u32) -> c_ulong {
    1usize.wrapping_shl(n) as c_ulong
}

const fn genmask(h: u32, l: u32) -> c_ulong {
    ((!0usize << l) & (!0usize >> (usize::BITS - 1 - h))) as c_ulong
}

const BYT_RT5651_DMIC_MAP: c_ulong = 0;
const BYT_RT5651_IN1_MAP: c_ulong = 1;
const BYT5651_IN2_MAP: c_ulong = 2;
const BYT_RT5651_IN2_MAP: c_ulong = BYT5651_IN2_MAP;
const BYT_RT5651_IN1_IN2_MAP: c_ulong = 3;

const BYT_RT5651_JD_NULL: c_ulong = (RT5651_JD_NULL as c_ulong) << 4;
const BYT_RT5651_JD1_1: c_ulong = (RT5651_JD1_1 as c_ulong) << 4;
const BYT_RT5651_JD1_2: c_ulong = (RT5651_JD1_2 as c_ulong) << 4;
const BYT_RT5651_JD2: c_ulong = (RT5651_JD2 as c_ulong) << 4;

const BYT_RT5651_OVCD_TH_600UA: c_ulong = 6 << 8;
const BYT_RT5651_OVCD_TH_1500UA: c_ulong = 15 << 8;
const BYT_RT5651_OVCD_TH_2000UA: c_ulong = 20 << 8;

const BYT_RT5651_OVCD_SF_0P5: c_ulong = (RT5651_OVCD_SF_0P5 as c_ulong) << 13;
const BYT_RT5651_OVCD_SF_0P75: c_ulong = (RT5651_OVCD_SF_0P75 as c_ulong) << 13;
const BYT_RT5651_OVCD_SF_1P0: c_ulong = (RT5651_OVCD_SF_1P0 as c_ulong) << 13;
const BYT_RT5651_OVCD_SF_1P5: c_ulong = (RT5651_OVCD_SF_1P5 as c_ulong) << 13;

const BYT_RT5651_MAP_MASK: c_ulong = genmask(3, 0);
const fn BYT_RT5651_MAP(quirk: c_ulong) -> c_ulong {
    quirk & BYT_RT5651_MAP_MASK
}
const fn BYT_RT5651_JDSRC(quirk: c_ulong) -> c_ulong {
    (quirk & genmask(7, 4)) >> 4
}
const fn BYT_RT5651_OVCD_TH(quirk: c_ulong) -> c_ulong {
    (quirk & genmask(12, 8)) >> 8
}
const fn BYT_RT5651_OVCD_SF(quirk: c_ulong) -> c_ulong {
    (quirk & genmask(14, 13)) >> 13
}
const BYT_RT5651_DMIC_EN: c_ulong = bit(16);
const BYT_RT5651_MCLK_EN: c_ulong = bit(17);
const BYT_RT5651_MCLK_25MHZ: c_ulong = bit(18);
const BYT_RT5651_SSP2_AIF2: c_ulong = bit(19); /* default is using AIF1  */
const BYT_RT5651_SSP0_AIF1: c_ulong = bit(20);
const BYT_RT5651_SSP0_AIF2: c_ulong = bit(21);
const BYT_RT5651_HP_LR_SWAPPED: c_ulong = bit(22);
const BYT_RT5651_MONO_SPEAKER: c_ulong = bit(23);
const BYT_RT5651_JD_NOT_INV: c_ulong = bit(24);

const BYT_RT5651_DEFAULT_QUIRKS: c_ulong = BYT_RT5651_MCLK_EN
    | BYT_RT5651_JD1_1
    | BYT_RT5651_OVCD_TH_2000UA
    | BYT_RT5651_OVCD_SF_0P75;

/* jack-detect-source + inv + dmic-en + ovcd-th + -sf + terminating entry */
const MAX_NO_PROPS: usize = 6;

#[repr(C)]
struct byt_rt5651_private {
    mclk: *mut clk,
    ext_amp_gpio: *mut gpio_desc,
    hp_detect: *mut gpio_desc,
    jack: snd_soc_jack,
    codec_dev: *mut device,
}

static mut byt_rt5651_gpios: *const acpi_gpio_mapping = ptr::null();

/* Default: jack-detect on JD1_1, internal mic on in2, headsetmic on in3 */
static mut byt_rt5651_quirk: c_ulong = BYT_RT5651_DEFAULT_QUIRKS | BYT_RT5651_IN2_MAP;

static mut quirk_override: c_int = -1;
/* module_param_named(quirk, quirk_override, int, 0444); */
/* MODULE_PARM_DESC(quirk, "Board-specific quirk override"); */

unsafe fn log_quirks(dev: *mut device) {
    let mut map: c_int;

    map = BYT_RT5651_MAP(byt_rt5651_quirk) as c_int;
    match map as c_ulong {
        BYT_RT5651_DMIC_MAP => dev_info(dev, c"quirk DMIC_MAP enabled".as_ptr()),
        BYT_RT5651_IN1_MAP => dev_info(dev, c"quirk IN1_MAP enabled".as_ptr()),
        BYT_RT5651_IN2_MAP => dev_info(dev, c"quirk IN2_MAP enabled".as_ptr()),
        BYT_RT5651_IN1_IN2_MAP => dev_info(dev, c"quirk IN1_IN2_MAP enabled".as_ptr()),
        _ => {
            dev_warn_once(
                dev,
                c"quirk sets invalid input map: 0x%x, default to DMIC_MAP\n".as_ptr(),
                map,
            );
            byt_rt5651_quirk &= !BYT_RT5651_MAP_MASK;
            byt_rt5651_quirk |= BYT_RT5651_DMIC_MAP;
        }
    }

    if BYT_RT5651_JDSRC(byt_rt5651_quirk) != 0 {
        dev_info(
            dev,
            c"quirk realtek,jack-detect-source %ld\n".as_ptr(),
            BYT_RT5651_JDSRC(byt_rt5651_quirk),
        );
        dev_info(
            dev,
            c"quirk realtek,over-current-threshold-microamp %ld\n".as_ptr(),
            BYT_RT5651_OVCD_TH(byt_rt5651_quirk) * 100,
        );
        dev_info(
            dev,
            c"quirk realtek,over-current-scale-factor %ld\n".as_ptr(),
            BYT_RT5651_OVCD_SF(byt_rt5651_quirk),
        );
    }
    if byt_rt5651_quirk & BYT_RT5651_DMIC_EN != 0 {
        dev_info(dev, c"quirk DMIC enabled".as_ptr());
    }
    if byt_rt5651_quirk & BYT_RT5651_MCLK_EN != 0 {
        dev_info(dev, c"quirk MCLK_EN enabled".as_ptr());
    }
    if byt_rt5651_quirk & BYT_RT5651_MCLK_25MHZ != 0 {
        dev_info(dev, c"quirk MCLK_25MHZ enabled".as_ptr());
    }
    if byt_rt5651_quirk & BYT_RT5651_SSP2_AIF2 != 0 {
        dev_info(dev, c"quirk SSP2_AIF2 enabled\n".as_ptr());
    }
    if byt_rt5651_quirk & BYT_RT5651_SSP0_AIF1 != 0 {
        dev_info(dev, c"quirk SSP0_AIF1 enabled\n".as_ptr());
    }
    if byt_rt5651_quirk & BYT_RT5651_SSP0_AIF2 != 0 {
        dev_info(dev, c"quirk SSP0_AIF2 enabled\n".as_ptr());
    }
    if byt_rt5651_quirk & BYT_RT5651_MONO_SPEAKER != 0 {
        dev_info(dev, c"quirk MONO_SPEAKER enabled\n".as_ptr());
    }
    if byt_rt5651_quirk & BYT_RT5651_JD_NOT_INV != 0 {
        dev_info(dev, c"quirk JD_NOT_INV enabled\n".as_ptr());
    }
}

const BYT_CODEC_DAI1: *const c_char = c"rt5651-aif1".as_ptr();
const BYT_CODEC_DAI2: *const c_char = c"rt5651-aif2".as_ptr();

unsafe fn byt_rt5651_prepare_and_enable_pll1(
    codec_dai: *mut snd_soc_dai,
    rate: c_int,
    bclk_ratio: c_int,
) -> c_int {
    let clk_id: c_int;
    let clk_freq: c_int;
    let mut ret: c_int;

    /* Configure the PLL before selecting it */
    if byt_rt5651_quirk & BYT_RT5651_MCLK_EN == 0 {
        clk_id = RT5651_PLL1_S_BCLK1;
        clk_freq = rate * bclk_ratio;
    } else {
        clk_id = RT5651_PLL1_S_MCLK;
        if byt_rt5651_quirk & BYT_RT5651_MCLK_25MHZ != 0 {
            clk_freq = 25000000;
        } else {
            clk_freq = 19200000;
        }
    }
    ret = snd_soc_dai_set_pll(codec_dai, 0, clk_id, clk_freq, rate * 512);
    if ret < 0 {
        dev_err((*(*codec_dai).component).dev, c"can't set pll: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(codec_dai, RT5651_SCLK_S_PLL1, rate * 512, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err((*(*codec_dai).component).dev, c"can't set clock %d\n".as_ptr(), ret);
        return ret;
    }

    0
}

unsafe fn platform_clock_control(
    w: *mut snd_soc_dapm_widget,
    _k: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card = snd_soc_dapm_to_card((*w).dapm);
    let mut codec_dai: *mut snd_soc_dai;
    let priv_ = snd_soc_card_get_drvdata(card) as *mut byt_rt5651_private;
    let mut ret: c_int;

    codec_dai = snd_soc_card_get_codec_dai(card, BYT_CODEC_DAI1);
    if codec_dai.is_null() {
        codec_dai = snd_soc_card_get_codec_dai(card, BYT_CODEC_DAI2);
    }
    if codec_dai.is_null() {
        dev_err(
            (*card).dev,
            c"Codec dai not found; Unable to set platform clock\n".as_ptr(),
        );
        return -EIO;
    }

    if SND_SOC_DAPM_EVENT_ON(event) {
        ret = clk_prepare_enable((*priv_).mclk);
        if ret < 0 {
            dev_err((*card).dev, c"could not configure MCLK state: %d\n".as_ptr(), ret);
            return ret;
        }
        ret = byt_rt5651_prepare_and_enable_pll1(codec_dai, 48000, 50);
        if ret < 0 {
            clk_disable_unprepare((*priv_).mclk);
        }
    } else {
        /*
         * Set codec clock source to internal clock before
         * turning off the platform clock. Codec needs clock
         * for Jack detection and button press
         */
        ret = snd_soc_dai_set_sysclk(
            codec_dai,
            RT5651_SCLK_S_RCCLK,
            48000 * 512,
            SND_SOC_CLOCK_IN,
        );
        if ret == 0 {
            clk_disable_unprepare((*priv_).mclk);
        }
    }

    if ret < 0 {
        dev_err((*card).dev, c"can't set codec sysclk: %d\n".as_ptr(), ret);
        return ret;
    }

    0
}

unsafe fn rt5651_ext_amp_power_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card = snd_soc_dapm_to_card((*w).dapm);
    let priv_ = snd_soc_card_get_drvdata(card) as *mut byt_rt5651_private;

    if SND_SOC_DAPM_EVENT_ON(event) {
        gpiod_set_value_cansleep((*priv_).ext_amp_gpio, 1);
    } else {
        gpiod_set_value_cansleep((*priv_).ext_amp_gpio, 0);
    }

    0
}

static byt_rt5651_widgets: [snd_soc_dapm_widget; 8] = [
    SND_SOC_DAPM_HP(c"Headphone".as_ptr(), ptr::null()),
    SND_SOC_DAPM_MIC(c"Headset Mic".as_ptr(), ptr::null()),
    SND_SOC_DAPM_MIC(c"Internal Mic".as_ptr(), ptr::null()),
    SND_SOC_DAPM_SPK(c"Speaker".as_ptr(), ptr::null()),
    SND_SOC_DAPM_LINE(c"Line In".as_ptr(), ptr::null()),
    SND_SOC_DAPM_SUPPLY(
        c"Platform Clock".as_ptr(),
        SND_SOC_NOPM,
        0,
        0,
        Some(platform_clock_control),
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD,
    ),
    SND_SOC_DAPM_SUPPLY(
        c"Ext Amp Power".as_ptr(),
        SND_SOC_NOPM,
        0,
        0,
        Some(rt5651_ext_amp_power_event),
        SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU,
    ),
];

static byt_rt5651_audio_map: [snd_soc_dapm_route; 13] = [
    route(c"Headphone".as_ptr(), ptr::null(), c"Platform Clock".as_ptr()),
    route(c"Headset Mic".as_ptr(), ptr::null(), c"Platform Clock".as_ptr()),
    route(c"Internal Mic".as_ptr(), ptr::null(), c"Platform Clock".as_ptr()),
    route(c"Speaker".as_ptr(), ptr::null(), c"Platform Clock".as_ptr()),
    route(c"Speaker".as_ptr(), ptr::null(), c"Ext Amp Power".as_ptr()),
    route(c"Line In".as_ptr(), ptr::null(), c"Platform Clock".as_ptr()),
    route(c"Headset Mic".as_ptr(), ptr::null(), c"micbias1".as_ptr()), /* lowercase for rt5651 */
    route(c"Headphone".as_ptr(), ptr::null(), c"HPOL".as_ptr()),
    route(c"Headphone".as_ptr(), ptr::null(), c"HPOR".as_ptr()),
    route(c"Speaker".as_ptr(), ptr::null(), c"LOUTL".as_ptr()),
    route(c"Speaker".as_ptr(), ptr::null(), c"LOUTR".as_ptr()),
    route(c"IN2P".as_ptr(), ptr::null(), c"Line In".as_ptr()),
    route(c"IN2N".as_ptr(), ptr::null(), c"Line In".as_ptr()),
];

static byt_rt5651_intmic_dmic_map: [snd_soc_dapm_route; 3] = [
    route(c"DMIC L1".as_ptr(), ptr::null(), c"Internal Mic".as_ptr()),
    route(c"DMIC R1".as_ptr(), ptr::null(), c"Internal Mic".as_ptr()),
    route(c"IN2P".as_ptr(), ptr::null(), c"Headset Mic".as_ptr()),
];

static byt_rt5651_intmic_in1_map: [snd_soc_dapm_route; 3] = [
    route(c"Internal Mic".as_ptr(), ptr::null(), c"micbias1".as_ptr()),
    route(c"IN1P".as_ptr(), ptr::null(), c"Internal Mic".as_ptr()),
    route(c"IN3P".as_ptr(), ptr::null(), c"Headset Mic".as_ptr()),
];

static byt_rt5651_intmic_in2_map: [snd_soc_dapm_route; 3] = [
    route(c"Internal Mic".as_ptr(), ptr::null(), c"micbias1".as_ptr()),
    route(c"IN2P".as_ptr(), ptr::null(), c"Internal Mic".as_ptr()),
    route(c"IN3P".as_ptr(), ptr::null(), c"Headset Mic".as_ptr()),
];

static byt_rt5651_intmic_in1_in2_map: [snd_soc_dapm_route; 4] = [
    route(c"Internal Mic".as_ptr(), ptr::null(), c"micbias1".as_ptr()),
    route(c"IN1P".as_ptr(), ptr::null(), c"Internal Mic".as_ptr()),
    route(c"IN2P".as_ptr(), ptr::null(), c"Internal Mic".as_ptr()),
    route(c"IN3P".as_ptr(), ptr::null(), c"Headset Mic".as_ptr()),
];

static byt_rt5651_ssp0_aif1_map: [snd_soc_dapm_route; 4] = [
    route(c"ssp0 Tx".as_ptr(), ptr::null(), c"modem_out".as_ptr()),
    route(c"modem_in".as_ptr(), ptr::null(), c"ssp0 Rx".as_ptr()),
    route(c"AIF1 Playback".as_ptr(), ptr::null(), c"ssp0 Tx".as_ptr()),
    route(c"ssp0 Rx".as_ptr(), ptr::null(), c"AIF1 Capture".as_ptr()),
];

static byt_rt5651_ssp0_aif2_map: [snd_soc_dapm_route; 4] = [
    route(c"ssp0 Tx".as_ptr(), ptr::null(), c"modem_out".as_ptr()),
    route(c"modem_in".as_ptr(), ptr::null(), c"ssp0 Rx".as_ptr()),
    route(c"AIF2 Playback".as_ptr(), ptr::null(), c"ssp0 Tx".as_ptr()),
    route(c"ssp0 Rx".as_ptr(), ptr::null(), c"AIF2 Capture".as_ptr()),
];

static byt_rt5651_ssp2_aif1_map: [snd_soc_dapm_route; 6] = [
    route(c"ssp2 Tx".as_ptr(), ptr::null(), c"codec_out0".as_ptr()),
    route(c"ssp2 Tx".as_ptr(), ptr::null(), c"codec_out1".as_ptr()),
    route(c"codec_in0".as_ptr(), ptr::null(), c"ssp2 Rx".as_ptr()),
    route(c"codec_in1".as_ptr(), ptr::null(), c"ssp2 Rx".as_ptr()),
    route(c"AIF1 Playback".as_ptr(), ptr::null(), c"ssp2 Tx".as_ptr()),
    route(c"ssp2 Rx".as_ptr(), ptr::null(), c"AIF1 Capture".as_ptr()),
];

static byt_rt5651_ssp2_aif2_map: [snd_soc_dapm_route; 6] = [
    route(c"ssp2 Tx".as_ptr(), ptr::null(), c"codec_out0".as_ptr()),
    route(c"ssp2 Tx".as_ptr(), ptr::null(), c"codec_out1".as_ptr()),
    route(c"codec_in0".as_ptr(), ptr::null(), c"ssp2 Rx".as_ptr()),
    route(c"codec_in1".as_ptr(), ptr::null(), c"ssp2 Rx".as_ptr()),
    route(c"AIF2 Playback".as_ptr(), ptr::null(), c"ssp2 Tx".as_ptr()),
    route(c"ssp2 Rx".as_ptr(), ptr::null(), c"AIF2 Capture".as_ptr()),
];

static byt_rt5651_controls: [snd_kcontrol_new; 5] = [
    SOC_DAPM_PIN_SWITCH(c"Headphone".as_ptr()),
    SOC_DAPM_PIN_SWITCH(c"Headset Mic".as_ptr()),
    SOC_DAPM_PIN_SWITCH(c"Internal Mic".as_ptr()),
    SOC_DAPM_PIN_SWITCH(c"Speaker".as_ptr()),
    SOC_DAPM_PIN_SWITCH(c"Line In".as_ptr()),
];

static mut bytcr_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c"Headphone".as_ptr(),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: c"Headset Mic".as_ptr(),
        mask: SND_JACK_MICROPHONE,
    },
];

unsafe fn byt_rt5651_aif1_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let format: snd_pcm_format_t = params_format(params);
    let rate: c_int = params_rate(params);
    let bclk_ratio: c_int;

    if format == SNDRV_PCM_FORMAT_S16_LE {
        bclk_ratio = 32;
    } else {
        bclk_ratio = 50;
    }

    byt_rt5651_prepare_and_enable_pll1(codec_dai, rate, bclk_ratio)
}

static pov_p1006w_hp_detect: acpi_gpio_params = acpi_gpio_params { crs_entry_index: 1, line_index: 0, active_low: false };
static pov_p1006w_ext_amp_en: acpi_gpio_params = acpi_gpio_params { crs_entry_index: 2, line_index: 0, active_low: true };

static byt_rt5651_pov_p1006w_gpios: [acpi_gpio_mapping; 3] = [
    acpi_gpio_mapping { name: c"hp-detect-gpios".as_ptr(), data: &pov_p1006w_hp_detect, size: 1, quirks: 0 },
    acpi_gpio_mapping { name: c"ext-amp-enable-gpios".as_ptr(), data: &pov_p1006w_ext_amp_en, size: 1, quirks: 0 },
    acpi_gpio_mapping::zeroed(),
];

unsafe fn byt_rt5651_pov_p1006w_quirk_cb(id: *const dmi_system_id) -> c_int {
    byt_rt5651_quirk = (*id).driver_data as c_ulong;
    byt_rt5651_gpios = byt_rt5651_pov_p1006w_gpios.as_ptr();
    1
}

unsafe fn byt_rt5651_quirk_cb(id: *const dmi_system_id) -> c_int {
    byt_rt5651_quirk = (*id).driver_data as c_ulong;
    1
}

static byt_rt5651_quirk_table: [dmi_system_id; 11] = [
    dmi_id(c"Chuwi Hi8 Pro (CWI513)".as_ptr(), Some(byt_rt5651_quirk_cb), &[
        DMI_MATCH(DMI_SYS_VENDOR, c"Hampoo".as_ptr()),
        DMI_MATCH(DMI_PRODUCT_NAME, c"X1D3_C806N".as_ptr()),
    ], (BYT_RT5651_DEFAULT_QUIRKS | BYT_RT5651_IN2_MAP | BYT_RT5651_HP_LR_SWAPPED | BYT_RT5651_MONO_SPEAKER) as *mut c_void),
    dmi_id(c"Chuwi Vi8 Plus (CWI519)".as_ptr(), Some(byt_rt5651_quirk_cb), &[
        DMI_MATCH(DMI_SYS_VENDOR, c"Hampoo".as_ptr()),
        DMI_MATCH(DMI_PRODUCT_NAME, c"D2D3_Vi8A1".as_ptr()),
    ], (BYT_RT5651_DEFAULT_QUIRKS | BYT_RT5651_IN2_MAP | BYT_RT5651_HP_LR_SWAPPED | BYT_RT5651_MONO_SPEAKER) as *mut c_void),
    dmi_id(c"Complet Electro Serv MY8307".as_ptr(), Some(byt_rt5651_quirk_cb), &[
        DMI_MATCH(DMI_SYS_VENDOR, c"Complet Electro Serv".as_ptr()),
        DMI_MATCH(DMI_PRODUCT_NAME, c"MY8307".as_ptr()),
    ], (BYT_RT5651_DEFAULT_QUIRKS | BYT_RT5651_IN2_MAP | BYT_RT5651_MONO_SPEAKER | BYT_RT5651_JD_NOT_INV) as *mut c_void),
    dmi_id(c"I.T.Works TW701, Ployer Momo7w and Trekstor ST70416-6".as_ptr(), Some(byt_rt5651_quirk_cb), &[
        DMI_MATCH(DMI_BIOS_VENDOR, c"INSYDE Corp.".as_ptr()),
        DMI_MATCH(DMI_BIOS_VERSION, c".G.WI71C.".as_ptr()),
    ], (BYT_RT5651_DEFAULT_QUIRKS | BYT_RT5651_IN2_MAP | BYT_RT5651_SSP0_AIF1 | BYT_RT5651_MONO_SPEAKER) as *mut c_void),
    dmi_id(c"Jumper EZpad 7".as_ptr(), Some(byt_rt5651_quirk_cb), &[
        DMI_MATCH(DMI_SYS_VENDOR, c"Jumper".as_ptr()),
        DMI_MATCH(DMI_PRODUCT_NAME, c"EZpad".as_ptr()),
        DMI_MATCH(DMI_BIOS_VERSION, c"Jumper12x.WJ2012.bsBKRCP".as_ptr()),
    ], (BYT_RT5651_DEFAULT_QUIRKS | BYT_RT5651_IN2_MAP | BYT_RT5651_JD_NOT_INV) as *mut c_void),
    dmi_id(c"KIANO SlimNote 14.2".as_ptr(), Some(byt_rt5651_quirk_cb), &[
        DMI_MATCH(DMI_SYS_VENDOR, c"KIANO".as_ptr()),
        DMI_MATCH(DMI_PRODUCT_NAME, c"KIANO SlimNote 14.2".as_ptr()),
    ], (BYT_RT5651_DEFAULT_QUIRKS | BYT_RT5651_IN1_IN2_MAP) as *mut c_void),
    dmi_id(c"Minnowboard Max B3".as_ptr(), Some(byt_rt5651_quirk_cb), &[
        DMI_MATCH(DMI_SYS_VENDOR, c"Circuitco".as_ptr()),
        DMI_MATCH(DMI_PRODUCT_NAME, c"Minnowboard Max B3 PLATFORM".as_ptr()),
    ], BYT_RT5651_IN1_MAP as *mut c_void),
    dmi_id(c"Minnowboard Turbot".as_ptr(), Some(byt_rt5651_quirk_cb), &[
        DMI_MATCH(DMI_SYS_VENDOR, c"ADI".as_ptr()),
        DMI_MATCH(DMI_PRODUCT_NAME, c"Minnowboard Turbot".as_ptr()),
    ], (BYT_RT5651_MCLK_EN | BYT_RT5651_IN1_MAP) as *mut c_void),
    dmi_id(c"Point of View mobii wintab p1006w (v1.0)".as_ptr(), Some(byt_rt5651_pov_p1006w_quirk_cb), &[
        DMI_EXACT_MATCH(DMI_SYS_VENDOR, c"Insyde".as_ptr()),
        DMI_EXACT_MATCH(DMI_PRODUCT_NAME, c"BayTrail".as_ptr()),
        DMI_EXACT_MATCH(DMI_BOARD_VENDOR, c"105B".as_ptr()),
        DMI_EXACT_MATCH(DMI_BOARD_NAME, c"0E57".as_ptr()),
    ], (BYT_RT5651_DMIC_MAP | BYT_RT5651_OVCD_TH_2000UA | BYT_RT5651_OVCD_SF_0P75 | BYT_RT5651_DMIC_EN | BYT_RT5651_MCLK_EN | BYT_RT5651_SSP0_AIF1) as *mut c_void),
    dmi_id(c"VIOS LTH17".as_ptr(), Some(byt_rt5651_quirk_cb), &[
        DMI_MATCH(DMI_SYS_VENDOR, c"VIOS".as_ptr()),
        DMI_MATCH(DMI_PRODUCT_NAME, c"LTH17".as_ptr()),
    ], (BYT_RT5651_IN1_IN2_MAP | BYT_RT5651_JD1_1 | BYT_RT5651_OVCD_TH_2000UA | BYT_RT5651_OVCD_SF_1P0 | BYT_RT5651_MCLK_EN) as *mut c_void),
    dmi_id(c"Yours Y8W81 (and others using the same mainboard)".as_ptr(), Some(byt_rt5651_quirk_cb), &[
        DMI_MATCH(DMI_BIOS_VENDOR, c"INSYDE Corp.".as_ptr()),
        DMI_MATCH(DMI_BIOS_VERSION, c".F.W86C.".as_ptr()),
    ], (BYT_RT5651_DEFAULT_QUIRKS | BYT_RT5651_IN2_MAP | BYT_RT5651_SSP0_AIF1 | BYT_RT5651_MONO_SPEAKER) as *mut c_void),
    dmi_system_id::zeroed(),
];

/*
 * Note this MUST be called before snd_soc_register_card(), so that the props
 * are in place before the codec component driver's probe function parses them.
 */
unsafe fn byt_rt5651_add_codec_device_props(
    i2c_dev: *mut device,
    _priv: *mut byt_rt5651_private,
) -> c_int {
    let mut props: [property_entry; MAX_NO_PROPS] = [property_entry::zeroed(); MAX_NO_PROPS];
    let fwnode: *mut fwnode_handle;
    let mut cnt: c_int = 0;
    let ret: c_int;

    props[cnt as usize] = PROPERTY_ENTRY_U32(c"realtek,jack-detect-source".as_ptr(), BYT_RT5651_JDSRC(byt_rt5651_quirk) as u32);
    cnt += 1;
    props[cnt as usize] = PROPERTY_ENTRY_U32(c"realtek,over-current-threshold-microamp".as_ptr(), (BYT_RT5651_OVCD_TH(byt_rt5651_quirk) * 100) as u32);
    cnt += 1;
    props[cnt as usize] = PROPERTY_ENTRY_U32(c"realtek,over-current-scale-factor".as_ptr(), BYT_RT5651_OVCD_SF(byt_rt5651_quirk) as u32);
    cnt += 1;

    if byt_rt5651_quirk & BYT_RT5651_DMIC_EN != 0 {
        props[cnt as usize] = PROPERTY_ENTRY_BOOL(c"realtek,dmic-en".as_ptr());
        cnt += 1;
    }

    if byt_rt5651_quirk & BYT_RT5651_JD_NOT_INV != 0 {
        props[cnt as usize] = PROPERTY_ENTRY_BOOL(c"realtek,jack-detect-not-inverted".as_ptr());
    }

    fwnode = fwnode_create_software_node(props.as_ptr(), ptr::null());
    if IS_ERR(fwnode as *const c_void) {
        /* put_device(i2c_dev) is handled in caller */
        return PTR_ERR(fwnode as *const c_void);
    }

    ret = device_add_software_node(i2c_dev, to_software_node(fwnode));

    fwnode_handle_put(fwnode);

    ret
}

unsafe fn byt_rt5651_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*runtime).card;
    let dapm = snd_soc_card_to_dapm(card);
    let codec = (*snd_soc_rtd_to_codec(runtime, 0)).component;
    let priv_ = snd_soc_card_get_drvdata(card) as *mut byt_rt5651_private;
    let custom_map: *const snd_soc_dapm_route;
    let num_routes: c_int;
    let mut report: c_int;
    let mut ret: c_int;

    snd_soc_dapm_set_idle_bias(dapm, false);

    /* Start with RC clk for jack-detect (we disable MCLK below) */
    if byt_rt5651_quirk & BYT_RT5651_MCLK_EN != 0 {
        snd_soc_component_update_bits(codec, RT5651_GLB_CLK, RT5651_SCLK_SRC_MASK, RT5651_SCLK_SRC_RCCLK);
    }

    match BYT_RT5651_MAP(byt_rt5651_quirk) {
        BYT_RT5651_IN1_MAP => {
            custom_map = byt_rt5651_intmic_in1_map.as_ptr();
            num_routes = byt_rt5651_intmic_in1_map.len() as c_int;
        }
        BYT_RT5651_IN2_MAP => {
            custom_map = byt_rt5651_intmic_in2_map.as_ptr();
            num_routes = byt_rt5651_intmic_in2_map.len() as c_int;
        }
        BYT_RT5651_IN1_IN2_MAP => {
            custom_map = byt_rt5651_intmic_in1_in2_map.as_ptr();
            num_routes = byt_rt5651_intmic_in1_in2_map.len() as c_int;
        }
        _ => {
            custom_map = byt_rt5651_intmic_dmic_map.as_ptr();
            num_routes = byt_rt5651_intmic_dmic_map.len() as c_int;
        }
    }
    ret = snd_soc_dapm_add_routes(dapm, custom_map, num_routes);
    if ret != 0 {
        return ret;
    }

    if byt_rt5651_quirk & BYT_RT5651_SSP2_AIF2 != 0 {
        ret = snd_soc_dapm_add_routes(dapm, byt_rt5651_ssp2_aif2_map.as_ptr(), byt_rt5651_ssp2_aif2_map.len() as c_int);
    } else if byt_rt5651_quirk & BYT_RT5651_SSP0_AIF1 != 0 {
        ret = snd_soc_dapm_add_routes(dapm, byt_rt5651_ssp0_aif1_map.as_ptr(), byt_rt5651_ssp0_aif1_map.len() as c_int);
    } else if byt_rt5651_quirk & BYT_RT5651_SSP0_AIF2 != 0 {
        ret = snd_soc_dapm_add_routes(dapm, byt_rt5651_ssp0_aif2_map.as_ptr(), byt_rt5651_ssp0_aif2_map.len() as c_int);
    } else {
        ret = snd_soc_dapm_add_routes(dapm, byt_rt5651_ssp2_aif1_map.as_ptr(), byt_rt5651_ssp2_aif1_map.len() as c_int);
    }
    if ret != 0 {
        return ret;
    }

    ret = snd_soc_add_card_controls(card, byt_rt5651_controls.as_ptr(), byt_rt5651_controls.len() as c_int);
    if ret != 0 {
        dev_err((*card).dev, c"unable to add card controls\n".as_ptr());
        return ret;
    }

    /*
     * The firmware might enable the clock at boot (this information
     * may or may not be reflected in the enable clock register).
     * To change the rate we must disable the clock first to cover
     * these cases. Due to common clock framework restrictions that
     * do not allow to disable a clock that has not been enabled,
     * we need to enable the clock first.
     */
    ret = clk_prepare_enable((*priv_).mclk);
    if ret == 0 {
        clk_disable_unprepare((*priv_).mclk);
    }

    if byt_rt5651_quirk & BYT_RT5651_MCLK_25MHZ != 0 {
        ret = clk_set_rate((*priv_).mclk, 25000000);
    } else {
        ret = clk_set_rate((*priv_).mclk, 19200000);
    }

    if ret != 0 {
        dev_err((*card).dev, c"unable to set MCLK rate\n".as_ptr());
    }

    report = 0;
    if BYT_RT5651_JDSRC(byt_rt5651_quirk) != 0 {
        report = SND_JACK_HEADSET | SND_JACK_BTN_0;
    } else if !(*priv_).hp_detect.is_null() {
        report = SND_JACK_HEADSET;
    }

    if report != 0 {
        ret = snd_soc_card_jack_new_pins(
            (*runtime).card,
            c"Headset".as_ptr(),
            report,
            &mut (*priv_).jack,
            bytcr_jack_pins.as_mut_ptr(),
            bytcr_jack_pins.len() as c_int,
        );
        if ret != 0 {
            dev_err((*runtime).dev, c"jack creation failed %d\n".as_ptr(), ret);
            return ret;
        }

        if report & SND_JACK_BTN_0 != 0 {
            snd_jack_set_key((*priv_).jack.jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
        }

        ret = snd_soc_component_set_jack(codec, &mut (*priv_).jack, (*priv_).hp_detect);
        if ret != 0 {
            return ret;
        }
    }

    0
}

unsafe fn byt_rt5651_codec_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let mut ret: c_int;
    let bits: c_int;

    /* The DSP will convert the FE rate to 48k, stereo */
    (*rate).min = 48000;
    (*rate).max = 48000;
    (*channels).min = 2;
    (*channels).max = 2;

    if (byt_rt5651_quirk & BYT_RT5651_SSP0_AIF1 != 0)
        || (byt_rt5651_quirk & BYT_RT5651_SSP0_AIF2 != 0)
    {
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
    ret = snd_soc_dai_set_fmt(
        snd_soc_rtd_to_cpu(rtd, 0),
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_BP_FP,
    );

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

static rates_48000: [u32; 1] = [48000];

static constraints_48000: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: rates_48000.len() as u32,
    list: rates_48000.as_ptr(),
};

unsafe fn byt_rt5651_aif1_startup(substream: *mut snd_pcm_substream) -> c_int {
    snd_pcm_hw_constraint_list(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        &constraints_48000,
    )
}

static byt_rt5651_aif1_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(byt_rt5651_aif1_startup),
    ..snd_soc_ops::zeroed()
};

static byt_rt5651_be_ssp2_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(byt_rt5651_aif1_hw_params),
    ..snd_soc_ops::zeroed()
};

/* SND_SOC_DAILINK_DEF(dummy, DAILINK_COMP_ARRAY(COMP_DUMMY())); */
static mut dummy: [snd_soc_dai_link_component; 1] = [COMP_DUMMY()];
/* SND_SOC_DAILINK_DEF(media, DAILINK_COMP_ARRAY(COMP_CPU("media-cpu-dai"))); */
static mut media: [snd_soc_dai_link_component; 1] = [COMP_CPU(c"media-cpu-dai".as_ptr())];
/* SND_SOC_DAILINK_DEF(deepbuffer, DAILINK_COMP_ARRAY(COMP_CPU("deepbuffer-cpu-dai"))); */
static mut deepbuffer: [snd_soc_dai_link_component; 1] = [COMP_CPU(c"deepbuffer-cpu-dai".as_ptr())];
/* SND_SOC_DAILINK_DEF(ssp2_port, DAILINK_COMP_ARRAY(COMP_CPU("ssp2-port"))); */
static mut ssp2_port: [snd_soc_dai_link_component; 1] = [COMP_CPU(c"ssp2-port".as_ptr())];
/* SND_SOC_DAILINK_DEF(ssp2_codec, DAILINK_COMP_ARRAY(COMP_CODEC("i2c-10EC5651:00", "rt5651-aif1"))); */
static mut ssp2_codec: [snd_soc_dai_link_component; 1] = [COMP_CODEC(c"i2c-10EC5651:00".as_ptr(), c"rt5651-aif1".as_ptr())];
/* SND_SOC_DAILINK_DEF(platform, DAILINK_COMP_ARRAY(COMP_PLATFORM("sst-mfld-platform"))); */
static mut platform: [snd_soc_dai_link_component; 1] = [COMP_PLATFORM(c"sst-mfld-platform".as_ptr())];

static mut byt_rt5651_dais: [snd_soc_dai_link; 3] = [
    snd_soc_dai_link {
        name: c"Audio Port".as_ptr(),
        stream_name: c"Audio".as_ptr(),
        nonatomic: true,
        dynamic: 1,
        ops: &byt_rt5651_aif1_ops,
        cpus: unsafe { media.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { dummy.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { platform.as_mut_ptr() },
        num_platforms: 1,
        ..snd_soc_dai_link::zeroed()
    },
    snd_soc_dai_link {
        name: c"Deep-Buffer Audio Port".as_ptr(),
        stream_name: c"Deep-Buffer Audio".as_ptr(),
        nonatomic: true,
        dynamic: 1,
        playback_only: 1,
        ops: &byt_rt5651_aif1_ops,
        cpus: unsafe { deepbuffer.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { dummy.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { platform.as_mut_ptr() },
        num_platforms: 1,
        ..snd_soc_dai_link::zeroed()
    },
    /* CODEC<->CODEC link */
    /* back ends */
    snd_soc_dai_link {
        name: c"SSP2-Codec".as_ptr(),
        id: 0,
        no_pcm: 1,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        be_hw_params_fixup: Some(byt_rt5651_codec_fixup),
        init: Some(byt_rt5651_init),
        ops: &byt_rt5651_be_ssp2_ops,
        cpus: unsafe { ssp2_port.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { ssp2_codec.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { platform.as_mut_ptr() },
        num_platforms: 1,
        ..snd_soc_dai_link::zeroed()
    },
];

/* SoC card */
static mut byt_rt5651_codec_name: [c_char; SND_ACPI_I2C_ID_LEN] = [0; SND_ACPI_I2C_ID_LEN];
/* #if !IS_ENABLED(CONFIG_SND_SOC_INTEL_USER_FRIENDLY_LONG_NAMES) */
static mut byt_rt5651_long_name: [c_char; 50] = [0; 50]; /* = "bytcr-rt5651-*-spk-*-mic[-swapped-hp]" */
/* #endif */
static mut byt_rt5651_components: [c_char; 50] = [0; 50]; /* = "cfg-spk:* cfg-mic:*" */

unsafe fn byt_rt5651_suspend(card: *mut snd_soc_card) -> c_int {
    let mut component: *mut snd_soc_component;

    if BYT_RT5651_JDSRC(byt_rt5651_quirk) == 0 {
        return 0;
    }

    component = first_card_component(card);
    while !component.is_null() {
        if strcmp((*component).name, byt_rt5651_codec_name.as_ptr()) == 0 {
            dev_dbg((*component).dev, c"disabling jack detect before suspend\n".as_ptr());
            snd_soc_component_set_jack(component, ptr::null_mut(), ptr::null_mut());
            break;
        }
        component = next_card_component(card, component);
    }

    0
}

unsafe fn byt_rt5651_resume(card: *mut snd_soc_card) -> c_int {
    let priv_ = snd_soc_card_get_drvdata(card) as *mut byt_rt5651_private;
    let mut component: *mut snd_soc_component;

    if BYT_RT5651_JDSRC(byt_rt5651_quirk) == 0 {
        return 0;
    }

    component = first_card_component(card);
    while !component.is_null() {
        if strcmp((*component).name, byt_rt5651_codec_name.as_ptr()) == 0 {
            dev_dbg((*component).dev, c"re-enabling jack detect after resume\n".as_ptr());
            snd_soc_component_set_jack(component, &mut (*priv_).jack, (*priv_).hp_detect);
            break;
        }
        component = next_card_component(card, component);
    }

    0
}

/* use space before codec name to simplify card ID, and simplify driver name */
const SOF_CARD_NAME: *const c_char = c"bytcht rt5651".as_ptr(); /* card name will be 'sof-bytcht rt5651' */
const SOF_DRIVER_NAME: *const c_char = c"SOF".as_ptr();

const CARD_NAME: *const c_char = c"bytcr-rt5651".as_ptr();
const DRIVER_NAME: *const c_char = ptr::null(); /* card name will be used for driver name */

static mut byt_rt5651_card: snd_soc_card = snd_soc_card {
    name: CARD_NAME,
    driver_name: DRIVER_NAME,
    owner: THIS_MODULE,
    dai_link: unsafe { byt_rt5651_dais.as_mut_ptr() },
    num_links: 3,
    dapm_widgets: byt_rt5651_widgets.as_ptr(),
    num_dapm_widgets: 8,
    dapm_routes: byt_rt5651_audio_map.as_ptr(),
    num_dapm_routes: 13,
    fully_routed: true,
    suspend_pre: Some(byt_rt5651_suspend),
    resume_post: Some(byt_rt5651_resume),
    ..snd_soc_card::zeroed()
};

static ext_amp_enable_gpios: acpi_gpio_params = acpi_gpio_params { crs_entry_index: 0, line_index: 0, active_low: false };

static cht_rt5651_gpios: [acpi_gpio_mapping; 2] = [
    /*
     * Some boards have I2cSerialBusV2, GpioIo, GpioInt as ACPI resources,
     * other boards may  have I2cSerialBusV2, GpioInt, GpioIo instead.
     * We want the GpioIo one for the ext-amp-enable-gpio.
     */
    acpi_gpio_mapping { name: c"ext-amp-enable-gpios".as_ptr(), data: &ext_amp_enable_gpios, size: 1, quirks: ACPI_GPIO_QUIRK_ONLY_GPIOIO },
    acpi_gpio_mapping::zeroed(),
];

#[repr(C)]
struct acpi_chan_package {
    /* ACPICA seems to require 64 bit integers */
    aif_value: u64,    /* 1: AIF1, 2: AIF2 */
    mclock_value: u64, /* usually 25MHz (0x17d7940), ignored */
}

unsafe fn snd_byt_rt5651_mc_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    static mic_name: [*const c_char; 4] = [
        c"dmic".as_ptr(),
        c"in1".as_ptr(),
        c"in2".as_ptr(),
        c"in12".as_ptr(),
    ];
    let mach = dev_get_platdata(dev) as *mut snd_soc_acpi_mach;
    let priv_: *mut byt_rt5651_private;
    let platform_name: *const c_char;
    let mut adev: *mut acpi_device;
    let codec_dev: *mut device;
    let sof_parent: bool;
    let mut is_bytcr = false;
    let mut ret_val: c_int = 0;
    let mut dai_index: c_int = 0;
    let mut i: c_int;

    priv_ = devm_kzalloc(dev, size_of::<byt_rt5651_private>(), GFP_KERNEL) as *mut byt_rt5651_private;
    if priv_.is_null() {
        return -ENOMEM;
    }

    /* register the soc card */
    byt_rt5651_card.dev = dev;
    snd_soc_card_set_drvdata(&mut byt_rt5651_card, priv_ as *mut c_void);

    /* fix index of codec dai */
    i = 0;
    while i < byt_rt5651_dais.len() as c_int {
        if byt_rt5651_dais[i as usize].num_codecs != 0
            && strcmp((*byt_rt5651_dais[i as usize].codecs).name, c"i2c-10EC5651:00".as_ptr()) == 0
        {
            dai_index = i;
            break;
        }
        i += 1;
    }

    /* fixup codec name based on HID */
    adev = acpi_dev_get_first_match_dev((*mach).id, ptr::null(), -1);
    if !adev.is_null() {
        snprintf(
            byt_rt5651_codec_name.as_mut_ptr(),
            byt_rt5651_codec_name.len(),
            c"i2c-%s".as_ptr(),
            acpi_dev_name(adev),
        );
        (*byt_rt5651_dais[dai_index as usize].codecs).name = byt_rt5651_codec_name.as_mut_ptr();
    } else {
        dev_err(dev, c"Error cannot find '%s' dev\n".as_ptr(), (*mach).id);
        return -ENOENT;
    }

    codec_dev = acpi_get_first_physical_node(adev);
    acpi_dev_put(adev);
    if codec_dev.is_null() {
        return -EPROBE_DEFER;
    }
    (*priv_).codec_dev = get_device(codec_dev);

    /*
     * swap SSP0 if bytcr is detected
     * (will be overridden if DMI quirk is detected)
     */
    if soc_intel_is_byt() {
        if (*mach).mach_params.acpi_ipc_irq_index == 0 {
            is_bytcr = true;
        }
    }

    if is_bytcr {
        /*
         * Baytrail CR platforms may have CHAN package in BIOS, try
         * to find relevant routing quirk based as done on Windows
         * platforms. We have to read the information directly from the
         * BIOS, at this stage the card is not created and the links
         * with the codec driver/pdata are non-existent
         */

        let mut chan_package = acpi_chan_package { aif_value: 0, mclock_value: 0 };

        /* format specified: 2 64-bit integers */
        let mut format = acpi_buffer { length: size_of_val(c"NN".to_bytes_with_nul()), pointer: c"NN".as_ptr() as *mut c_void };
        let mut state = acpi_buffer { length: 0, pointer: ptr::null_mut() };
        let mut pkg_ctx: snd_soc_acpi_package_context = snd_soc_acpi_package_context::zeroed();
        let mut pkg_found = false;

        state.length = size_of::<acpi_chan_package>();
        state.pointer = &mut chan_package as *mut _ as *mut c_void;

        pkg_ctx.name = c"CHAN".as_ptr();
        pkg_ctx.length = 2;
        pkg_ctx.format = &mut format;
        pkg_ctx.state = &mut state;
        pkg_ctx.data_valid = false;

        pkg_found = snd_soc_acpi_find_package_from_hid((*mach).id, &mut pkg_ctx);
        if pkg_found {
            if chan_package.aif_value == 1 {
                dev_info(dev, c"BIOS Routing: AIF1 connected\n".as_ptr());
                byt_rt5651_quirk |= BYT_RT5651_SSP0_AIF1;
            } else if chan_package.aif_value == 2 {
                dev_info(dev, c"BIOS Routing: AIF2 connected\n".as_ptr());
                byt_rt5651_quirk |= BYT_RT5651_SSP0_AIF2;
            } else {
                dev_info(dev, c"BIOS Routing isn't valid, ignored\n".as_ptr());
                pkg_found = false;
            }
        }

        if !pkg_found {
            /* no BIOS indications, assume SSP0-AIF2 connection */
            byt_rt5651_quirk |= BYT_RT5651_SSP0_AIF2;
        }
    }

    /* check quirks before creating card */
    dmi_check_system(byt_rt5651_quirk_table.as_ptr());

    if quirk_override != -1 {
        dev_info(
            dev,
            c"Overriding quirk 0x%lx => 0x%x\n".as_ptr(),
            byt_rt5651_quirk,
            quirk_override,
        );
        byt_rt5651_quirk = quirk_override as c_ulong;
    }

    /* Must be called before register_card, also see declaration comment. */
    ret_val = byt_rt5651_add_codec_device_props(codec_dev, priv_);
    if ret_val != 0 {
        goto_err_device(priv_, ret_val);
        return ret_val;
    }

    /* Cherry Trail devices use an external amplifier enable gpio */
    if soc_intel_is_cht() && byt_rt5651_gpios.is_null() {
        byt_rt5651_gpios = cht_rt5651_gpios.as_ptr();
    }

    if !byt_rt5651_gpios.is_null() {
        devm_acpi_dev_add_driver_gpios(codec_dev, byt_rt5651_gpios);
        (*priv_).ext_amp_gpio = devm_fwnode_gpiod_get(
            dev,
            (*codec_dev).fwnode,
            c"ext-amp-enable".as_ptr(),
            GPIOD_OUT_LOW,
            c"speaker-amp".as_ptr(),
        );
        if IS_ERR((*priv_).ext_amp_gpio as *const c_void) {
            ret_val = PTR_ERR((*priv_).ext_amp_gpio as *const c_void);
            match ret_val {
                x if x == -ENOENT => {
                    (*priv_).ext_amp_gpio = ptr::null_mut();
                }
                _ => {
                    dev_err(dev, c"Failed to get ext-amp-enable GPIO: %d\n".as_ptr(), ret_val);
                    if ret_val == -EPROBE_DEFER {
                        goto_err(priv_, ret_val);
                        return ret_val;
                    }
                    goto_err(priv_, ret_val);
                    return ret_val;
                }
            }
        }
        (*priv_).hp_detect = devm_fwnode_gpiod_get(
            dev,
            (*codec_dev).fwnode,
            c"hp-detect".as_ptr(),
            GPIOD_IN,
            c"hp-detect".as_ptr(),
        );
        if IS_ERR((*priv_).hp_detect as *const c_void) {
            ret_val = PTR_ERR((*priv_).hp_detect as *const c_void);
            match ret_val {
                x if x == -ENOENT => {
                    (*priv_).hp_detect = ptr::null_mut();
                }
                _ => {
                    dev_err(dev, c"Failed to get hp-detect GPIO: %d\n".as_ptr(), ret_val);
                    if ret_val == -EPROBE_DEFER {
                        goto_err(priv_, ret_val);
                        return ret_val;
                    }
                    goto_err(priv_, ret_val);
                    return ret_val;
                }
            }
        }
    }

    log_quirks(dev);

    if (byt_rt5651_quirk & BYT_RT5651_SSP2_AIF2 != 0)
        || (byt_rt5651_quirk & BYT_RT5651_SSP0_AIF2 != 0)
    {
        (*byt_rt5651_dais[dai_index as usize].codecs).dai_name = c"rt5651-aif2".as_ptr();
    }

    if (byt_rt5651_quirk & BYT_RT5651_SSP0_AIF1 != 0)
        || (byt_rt5651_quirk & BYT_RT5651_SSP0_AIF2 != 0)
    {
        (*byt_rt5651_dais[dai_index as usize].cpus).dai_name = c"ssp0-port".as_ptr();
    }

    if byt_rt5651_quirk & BYT_RT5651_MCLK_EN != 0 {
        (*priv_).mclk = devm_clk_get_optional(dev, c"pmc_plt_clk_3".as_ptr());
        if IS_ERR((*priv_).mclk as *const c_void) {
            ret_val = dev_err_probe(
                dev,
                PTR_ERR((*priv_).mclk as *const c_void),
                c"Failed to get MCLK from pmc_plt_clk_3\n".as_ptr(),
            );
            goto_err(priv_, ret_val);
            return ret_val;
        }
        /*
         * Fall back to bit clock usage when clock is not
         * available likely due to missing dependencies.
         */
        if (*priv_).mclk.is_null() {
            byt_rt5651_quirk &= !BYT_RT5651_MCLK_EN;
        }
    }

    snprintf(
        byt_rt5651_components.as_mut_ptr(),
        byt_rt5651_components.len(),
        c"cfg-spk:%s cfg-mic:%s%s".as_ptr(),
        if byt_rt5651_quirk & BYT_RT5651_MONO_SPEAKER != 0 { c"1".as_ptr() } else { c"2".as_ptr() },
        mic_name[BYT_RT5651_MAP(byt_rt5651_quirk) as usize],
        if byt_rt5651_quirk & BYT_RT5651_HP_LR_SWAPPED != 0 { c" cfg-hp:lrswap".as_ptr() } else { c"".as_ptr() },
    );
    byt_rt5651_card.components = byt_rt5651_components.as_mut_ptr();

    /* #if !IS_ENABLED(CONFIG_SND_SOC_INTEL_USER_FRIENDLY_LONG_NAMES) */
    snprintf(
        byt_rt5651_long_name.as_mut_ptr(),
        byt_rt5651_long_name.len(),
        c"bytcr-rt5651-%s-spk-%s-mic%s".as_ptr(),
        if byt_rt5651_quirk & BYT_RT5651_MONO_SPEAKER != 0 { c"mono".as_ptr() } else { c"stereo".as_ptr() },
        mic_name[BYT_RT5651_MAP(byt_rt5651_quirk) as usize],
        if byt_rt5651_quirk & BYT_RT5651_HP_LR_SWAPPED != 0 { c"-hp-swapped".as_ptr() } else { c"".as_ptr() },
    );
    byt_rt5651_card.long_name = byt_rt5651_long_name.as_mut_ptr();
    /* #endif */

    /* override platform name, if required */
    platform_name = (*mach).mach_params.platform;

    ret_val = snd_soc_fixup_dai_links_platform_name(&mut byt_rt5651_card, platform_name);
    if ret_val != 0 {
        goto_err(priv_, ret_val);
        return ret_val;
    }

    sof_parent = snd_soc_acpi_sof_parent(dev);

    /* set card and driver name */
    if sof_parent {
        byt_rt5651_card.name = SOF_CARD_NAME;
        byt_rt5651_card.driver_name = SOF_DRIVER_NAME;
    } else {
        byt_rt5651_card.name = CARD_NAME;
        byt_rt5651_card.driver_name = DRIVER_NAME;
    }

    /* set pm ops */
    if sof_parent {
        (*(*dev).driver).pm = &snd_soc_pm_ops;
    }

    ret_val = devm_snd_soc_register_card(dev, &mut byt_rt5651_card);
    if ret_val != 0 {
        dev_err(dev, c"devm_snd_soc_register_card failed %d\n".as_ptr(), ret_val);
        goto_err(priv_, ret_val);
        return ret_val;
    }
    platform_set_drvdata(pdev, &mut byt_rt5651_card as *mut _ as *mut c_void);
    ret_val
}

unsafe fn goto_err(priv_: *mut byt_rt5651_private, _ret_val: c_int) {
    device_remove_software_node((*priv_).codec_dev);
    put_device((*priv_).codec_dev);
}

unsafe fn goto_err_device(priv_: *mut byt_rt5651_private, _ret_val: c_int) {
    put_device((*priv_).codec_dev);
}

unsafe fn snd_byt_rt5651_mc_remove(pdev: *mut platform_device) {
    let card = platform_get_drvdata(pdev) as *mut snd_soc_card;
    let priv_ = snd_soc_card_get_drvdata(card) as *mut byt_rt5651_private;

    device_remove_software_node((*priv_).codec_dev);
    put_device((*priv_).codec_dev);
}

static mut snd_byt_rt5651_mc_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"bytcr_rt5651".as_ptr(),
        ..device_driver::zeroed()
    },
    probe: Some(snd_byt_rt5651_mc_probe),
    remove: Some(snd_byt_rt5651_mc_remove),
    ..platform_driver::zeroed()
};

/* module_platform_driver(snd_byt_rt5651_mc_driver); */

/* MODULE_DESCRIPTION("ASoC Intel(R) Baytrail CR Machine driver for RT5651"); */
/* MODULE_AUTHOR("Pierre-Louis Bossart <pierre-louis.bossart@linux.intel.com>"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_ALIAS("platform:bytcr_rt5651"); */

extern "C" {
    static THIS_MODULE: *mut module;
    static snd_soc_pm_ops: dev_pm_ops;

    static RT5651_JD_NULL: c_int;
    static RT5651_JD1_1: c_int;
    static RT5651_JD1_2: c_int;
    static RT5651_JD2: c_int;
    static RT5651_OVCD_SF_0P5: c_int;
    static RT5651_OVCD_SF_0P75: c_int;
    static RT5651_OVCD_SF_1P0: c_int;
    static RT5651_OVCD_SF_1P5: c_int;

    static RT5651_PLL1_S_BCLK1: c_int;
    static RT5651_PLL1_S_MCLK: c_int;
    static RT5651_SCLK_S_PLL1: c_int;
    static RT5651_SCLK_S_RCCLK: c_int;
    static RT5651_GLB_CLK: c_int;
    static RT5651_SCLK_SRC_MASK: c_int;
    static RT5651_SCLK_SRC_RCCLK: c_int;

    static SND_SOC_CLOCK_IN: c_int;
    static SND_SOC_NOPM: c_int;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_JACK_HEADPHONE: c_int;
    static SND_JACK_MICROPHONE: c_int;
    static SND_JACK_HEADSET: c_int;
    static SND_JACK_BTN_0: c_int;
    static KEY_PLAYPAUSE: c_int;
    static SNDRV_PCM_FORMAT_S16_LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_S24_LE: snd_pcm_format_t;
    static SNDRV_PCM_HW_PARAM_RATE: c_int;
    static SNDRV_PCM_HW_PARAM_CHANNELS: c_int;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_BP_FP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static MERR_DPCM_AUDIO: usize;
    static MERR_DPCM_DEEP_BUFFER: usize;
    static SND_ACPI_I2C_ID_LEN: usize;
    static ACPI_GPIO_QUIRK_ONLY_GPIOIO: c_ulong;
    static GFP_KERNEL: c_ulong;
    static GPIOD_OUT_LOW: c_int;
    static GPIOD_IN: c_int;
    static EIO: c_int;
    static ENOMEM: c_int;
    static ENOENT: c_int;
    static EPROBE_DEFER: c_int;
    static DMI_SYS_VENDOR: c_int;
    static DMI_PRODUCT_NAME: c_int;
    static DMI_BIOS_VENDOR: c_int;
    static DMI_BIOS_VERSION: c_int;
    static DMI_BOARD_VENDOR: c_int;
    static DMI_BOARD_NAME: c_int;

    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn_once(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_dai_set_pll(dai: *mut snd_soc_dai, pll_id: c_int, source: c_int, freq_in: c_int, freq_out: c_int) -> c_int;
    fn snd_soc_dai_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_int, dir: c_int) -> c_int;
    fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_card_get_codec_dai(card: *mut snd_soc_card, dai_name: *const c_char) -> *mut snd_soc_dai;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn fwnode_create_software_node(props: *const property_entry, parent: *const fwnode_handle) -> *mut fwnode_handle;
    fn device_add_software_node(dev: *mut device, swnode: *const software_node) -> c_int;
    fn to_software_node(fwnode: *mut fwnode_handle) -> *const software_node;
    fn fwnode_handle_put(fwnode: *mut fwnode_handle);
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_set_idle_bias(dapm: *mut snd_soc_dapm_context, idle_bias: bool);
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_int, mask: c_int, val: c_int) -> c_int;
    fn snd_soc_dapm_add_routes(dapm: *mut snd_soc_dapm_context, route: *const snd_soc_dapm_route, num: c_int) -> c_int;
    fn snd_soc_add_card_controls(card: *mut snd_soc_card, controls: *const snd_kcontrol_new, num_controls: c_int) -> c_int;
    fn snd_soc_card_jack_new_pins(card: *mut snd_soc_card, id: *const c_char, type_: c_int, jack: *mut snd_soc_jack, pins: *mut snd_soc_jack_pin, num_pins: c_int) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_int, keytype: c_int);
    fn snd_soc_component_set_jack(component: *mut snd_soc_component, jack: *mut snd_soc_jack, data: *mut c_void) -> c_int;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn params_set_format(params: *mut snd_pcm_hw_params, val: snd_pcm_format_t);
    fn snd_soc_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
    fn snd_soc_dai_set_tdm_slot(dai: *mut snd_soc_dai, tx_mask: c_uint, rx_mask: c_uint, slots: c_int, slot_width: c_int) -> c_int;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    fn first_card_component(card: *mut snd_soc_card) -> *mut snd_soc_component;
    fn next_card_component(card: *mut snd_soc_card, component: *mut snd_soc_component) -> *mut snd_soc_component;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_ulong) -> *mut c_void;
    fn acpi_dev_get_first_match_dev(hid: *const c_char, uid: *const c_char, hrv: c_int) -> *mut acpi_device;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn acpi_dev_name(adev: *mut acpi_device) -> *const c_char;
    fn acpi_get_first_physical_node(adev: *mut acpi_device) -> *mut device;
    fn acpi_dev_put(adev: *mut acpi_device);
    fn get_device(dev: *mut device) -> *mut device;
    fn put_device(dev: *mut device);
    fn soc_intel_is_byt() -> bool;
    fn snd_soc_acpi_find_package_from_hid(hid: *const c_char, ctx: *mut snd_soc_acpi_package_context) -> bool;
    fn dmi_check_system(list: *const dmi_system_id) -> c_int;
    fn soc_intel_is_cht() -> bool;
    fn devm_acpi_dev_add_driver_gpios(dev: *mut device, gpios: *const acpi_gpio_mapping);
    fn devm_fwnode_gpiod_get(dev: *mut device, fwnode: *mut fwnode_handle, con_id: *const c_char, flags: c_int, label: *const c_char) -> *mut gpio_desc;
    fn devm_clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk;
    fn snd_soc_fixup_dai_links_platform_name(card: *mut snd_soc_card, platform_name: *const c_char) -> c_int;
    fn snd_soc_acpi_sof_parent(dev: *mut device) -> bool;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn device_remove_software_node(dev: *mut device);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
}

type c_uint = u32;
type snd_pcm_format_t = c_int;

enum clk {}
enum gpio_desc {}
enum snd_kcontrol {}
enum module {}
enum dev_pm_ops {}
enum fwnode_handle {}
enum software_node {}
enum snd_pcm_hw_params {}
enum snd_pcm_runtime {}
enum snd_jack {}
enum acpi_device {}

#[repr(C)]
struct device {
    driver: *mut device_driver,
    fwnode: *mut fwnode_handle,
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
    pm: *const dev_pm_ops,
}

impl device_driver {
    const fn zeroed() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct platform_driver {
    driver: device_driver,
    probe: Option<unsafe fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe fn(*mut platform_device)>,
}

impl platform_driver {
    const fn zeroed() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
    name: *const c_char,
}

#[repr(C)]
struct snd_soc_dai {
    component: *mut snd_soc_component,
}

#[repr(C)]
struct snd_soc_dapm_context;

#[repr(C)]
struct snd_soc_dapm_widget {
    dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

const fn route(sink: *const c_char, control: *const c_char, source: *const c_char) -> snd_soc_dapm_route {
    snd_soc_dapm_route { sink, control, source }
}

#[repr(C)]
struct snd_kcontrol_new;

#[repr(C)]
struct snd_soc_jack {
    jack: *mut snd_jack,
}

#[repr(C)]
struct snd_soc_jack_pin {
    pin: *const c_char,
    mask: c_int,
}

#[repr(C)]
struct acpi_gpio_params {
    crs_entry_index: c_uint,
    line_index: c_uint,
    active_low: bool,
}

#[repr(C)]
struct acpi_gpio_mapping {
    name: *const c_char,
    data: *const acpi_gpio_params,
    size: c_uint,
    quirks: c_ulong,
}

impl acpi_gpio_mapping {
    const fn zeroed() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
struct dmi_system_id {
    callback: Option<unsafe fn(*const dmi_system_id) -> c_int>,
    matches: [dmi_strmatch; 4],
    driver_data: *mut c_void,
}

impl dmi_system_id {
    const fn zeroed() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
struct dmi_strmatch {
    slot: c_int,
    substr: *const c_char,
    exact: bool,
}

fn dmi_id(
    _ident: *const c_char,
    callback: Option<unsafe fn(*const dmi_system_id) -> c_int>,
    matches: &[dmi_strmatch],
    driver_data: *mut c_void,
) -> dmi_system_id {
    let mut id = dmi_system_id::zeroed();
    id.callback = callback;
    id.driver_data = driver_data;
    let mut i = 0;
    while i < matches.len() && i < id.matches.len() {
        id.matches[i] = matches[i];
        i += 1;
    }
    id
}

const fn DMI_MATCH(slot: c_int, substr: *const c_char) -> dmi_strmatch {
    dmi_strmatch { slot, substr, exact: false }
}

const fn DMI_EXACT_MATCH(slot: c_int, substr: *const c_char) -> dmi_strmatch {
    dmi_strmatch { slot, substr, exact: true }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct property_entry;

impl property_entry {
    const fn zeroed() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

fn PROPERTY_ENTRY_U32(_name: *const c_char, _val: u32) -> property_entry {
    property_entry::zeroed()
}

fn PROPERTY_ENTRY_BOOL(_name: *const c_char) -> property_entry {
    property_entry::zeroed()
}

#[repr(C)]
struct snd_soc_pcm_runtime {
    card: *mut snd_soc_card,
    dev: *mut device,
}

#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
struct snd_interval {
    min: c_uint,
    max: c_uint,
}

#[repr(C)]
struct snd_pcm_hw_constraint_list {
    count: c_uint,
    list: *const c_uint,
}

#[repr(C)]
struct snd_soc_ops {
    startup: Option<unsafe fn(*mut snd_pcm_substream) -> c_int>,
    hw_params: Option<unsafe fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

impl snd_soc_ops {
    const fn zeroed() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
struct snd_soc_dai_link_component {
    name: *const c_char,
    dai_name: *const c_char,
}

fn COMP_DUMMY() -> snd_soc_dai_link_component {
    snd_soc_dai_link_component { name: ptr::null(), dai_name: ptr::null() }
}

fn COMP_CPU(name: *const c_char) -> snd_soc_dai_link_component {
    snd_soc_dai_link_component { name, dai_name: name }
}

fn COMP_CODEC(name: *const c_char, dai_name: *const c_char) -> snd_soc_dai_link_component {
    snd_soc_dai_link_component { name, dai_name }
}

fn COMP_PLATFORM(name: *const c_char) -> snd_soc_dai_link_component {
    snd_soc_dai_link_component { name, dai_name: ptr::null() }
}

#[repr(C)]
struct snd_soc_dai_link {
    name: *const c_char,
    stream_name: *const c_char,
    id: c_int,
    nonatomic: bool,
    dynamic: c_int,
    playback_only: c_int,
    no_pcm: c_int,
    dai_fmt: c_uint,
    be_hw_params_fixup: Option<unsafe fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    init: Option<unsafe fn(*mut snd_soc_pcm_runtime) -> c_int>,
    ops: *const snd_soc_ops,
    cpus: *mut snd_soc_dai_link_component,
    num_cpus: c_int,
    codecs: *mut snd_soc_dai_link_component,
    num_codecs: c_int,
    platforms: *mut snd_soc_dai_link_component,
    num_platforms: c_int,
}

impl snd_soc_dai_link {
    const fn zeroed() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
struct snd_soc_card {
    name: *const c_char,
    driver_name: *const c_char,
    owner: *mut module,
    dev: *mut device,
    dai_link: *mut snd_soc_dai_link,
    num_links: c_int,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_int,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_int,
    fully_routed: bool,
    suspend_pre: Option<unsafe fn(*mut snd_soc_card) -> c_int>,
    resume_post: Option<unsafe fn(*mut snd_soc_card) -> c_int>,
    components: *mut c_char,
    long_name: *mut c_char,
}

impl snd_soc_card {
    const fn zeroed() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
struct acpi_buffer {
    length: usize,
    pointer: *mut c_void,
}

#[repr(C)]
struct snd_soc_acpi_package_context {
    name: *const c_char,
    length: c_int,
    format: *mut acpi_buffer,
    state: *mut acpi_buffer,
    data_valid: bool,
}

impl snd_soc_acpi_package_context {
    const fn zeroed() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
struct snd_soc_acpi_mach {
    id: *const c_char,
    mach_params: snd_soc_acpi_mach_params,
}

#[repr(C)]
struct snd_soc_acpi_mach_params {
    acpi_ipc_irq_index: c_int,
    platform: *const c_char,
}

fn SND_SOC_DAPM_EVENT_ON(event: c_int) -> bool {
    event != 0
}

fn SND_SOC_DAPM_HP(_name: *const c_char, _reg: *const c_void) -> snd_soc_dapm_widget {
    unsafe { core::mem::zeroed() }
}

fn SND_SOC_DAPM_MIC(_name: *const c_char, _reg: *const c_void) -> snd_soc_dapm_widget {
    unsafe { core::mem::zeroed() }
}

fn SND_SOC_DAPM_SPK(_name: *const c_char, _reg: *const c_void) -> snd_soc_dapm_widget {
    unsafe { core::mem::zeroed() }
}

fn SND_SOC_DAPM_LINE(_name: *const c_char, _reg: *const c_void) -> snd_soc_dapm_widget {
    unsafe { core::mem::zeroed() }
}

fn SND_SOC_DAPM_SUPPLY(
    _name: *const c_char,
    _reg: c_int,
    _shift: c_int,
    _invert: c_int,
    _event: Option<unsafe fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>,
    _flags: c_int,
) -> snd_soc_dapm_widget {
    unsafe { core::mem::zeroed() }
}

fn SOC_DAPM_PIN_SWITCH(_name: *const c_char) -> snd_kcontrol_new {
    unsafe { core::mem::zeroed() }
}

fn size_of_val<T: ?Sized>(_: &T) -> usize {
    size_of::<*const c_char>()
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
