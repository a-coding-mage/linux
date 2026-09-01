// SPDX-License-Identifier: GPL-2.0-only
/*
 *  bytcr_wm5102.c - ASoc Machine driver for Intel Baytrail platforms with a
 *                   Wolfson Microelectronics WM5102 codec
 *
 *  Copyright (C) 2020 Hans de Goede <hdegoede@redhat.com>
 *  Loosely based on bytcr_rt5640.c which is:
 *  Copyright (C) 2014-2020 Intel Corp
 *  Author: Subhransu S. Prusty <subhransu.s.prusty@intel.com>
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const WM5102_MAX_SYSCLK_4K: c_int = 49152000; /* max sysclk for 4K family */
const WM5102_MAX_SYSCLK_11025: c_int = 45158400; /* max sysclk for 11.025K family */

#[repr(C)]
pub struct byt_wm5102_private {
    jack: snd_soc_jack,
    mclk: *mut clk,
    spkvdd_en_gpio: *mut gpio_desc,
    mclk_freq: c_int,
}

const fn bit(nr: c_uint) -> c_ulong {
    1 as c_ulong << nr
}

const fn genmask(h: c_uint, l: c_uint) -> c_ulong {
    (!0 as c_ulong) << l & ((!0 as c_ulong) >> (c_ulong::BITS - 1 - h))
}

const fn field_prep_const(mask: c_ulong, val: c_ulong) -> c_ulong {
    val << mask.trailing_zeros()
}

const fn field_get(mask: c_ulong, reg: c_ulong) -> c_ulong {
    (reg & mask) >> mask.trailing_zeros()
}

const BYT_WM5102_IN_MAP: c_ulong = genmask(3, 0);
const BYT_WM5102_OUT_MAP: c_ulong = genmask(7, 4);
const BYT_WM5102_SSP2: c_ulong = bit(16);
const BYT_WM5102_MCLK_19_2MHZ: c_ulong = bit(17);

const BYT_WM5102_INTMIC_IN3L_HSMIC_IN1L: c_ulong = 0;
const BYT_WM5102_INTMIC_IN1L_HSMIC_IN2L: c_ulong = 1;

/* Note these values are pre-shifted for easy use of setting quirks */
const BYT_WM5102_SPK_SPK_MAP: c_ulong = field_prep_const(BYT_WM5102_OUT_MAP, 0);
const BYT_WM5102_SPK_HPOUT2_MAP: c_ulong = field_prep_const(BYT_WM5102_OUT_MAP, 1);

static mut quirk: c_ulong = 0;

static mut quirk_override: c_int = -1;
/* module_param_named(quirk, quirk_override, int, 0444); */
/* MODULE_PARM_DESC(quirk, "Board-specific quirk override"); */

unsafe fn log_quirks(dev: *mut device) {
    match quirk & BYT_WM5102_IN_MAP {
        BYT_WM5102_INTMIC_IN3L_HSMIC_IN1L => {
            dev_info_once(dev, c"quirk INTMIC_IN3L_HSMIC_IN1L enabled\n".as_ptr());
        }
        BYT_WM5102_INTMIC_IN1L_HSMIC_IN2L => {
            dev_info_once(dev, c"quirk INTMIC_IN1L_HSMIC_IN2L enabled\n".as_ptr());
        }
        _ => {
            dev_warn_once(
                dev,
                c"quirk sets invalid input map: 0x%lx, defaulting to INTMIC_IN3L_HSMIC_IN1L\n".as_ptr(),
                quirk & BYT_WM5102_IN_MAP,
            );
            quirk &= !BYT_WM5102_IN_MAP;
            quirk |= BYT_WM5102_INTMIC_IN3L_HSMIC_IN1L;
        }
    }
    match quirk & BYT_WM5102_OUT_MAP {
        BYT_WM5102_SPK_SPK_MAP => {
            dev_info_once(dev, c"quirk SPK_SPK_MAP enabled\n".as_ptr());
        }
        BYT_WM5102_SPK_HPOUT2_MAP => {
            dev_info_once(dev, c"quirk SPK_HPOUT2_MAP enabled\n".as_ptr());
        }
        _ => {
            dev_warn_once(
                dev,
                c"quirk sets invalid output map: 0x%lx, defaulting to SPK_SPK_MAP\n".as_ptr(),
                quirk & BYT_WM5102_OUT_MAP,
            );
            quirk &= !BYT_WM5102_OUT_MAP;
            quirk |= BYT_WM5102_SPK_SPK_MAP;
        }
    }
    if quirk & BYT_WM5102_SSP2 != 0 {
        dev_info_once(dev, c"quirk SSP2 enabled".as_ptr());
    }
    if quirk & BYT_WM5102_MCLK_19_2MHZ != 0 {
        dev_info_once(dev, c"quirk MCLK 19.2MHz enabled".as_ptr());
    }
}

unsafe extern "C" fn byt_wm5102_spkvdd_power_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card = snd_soc_dapm_to_card((*w).dapm);
    let priv_ = snd_soc_card_get_drvdata(card) as *mut byt_wm5102_private;

    gpiod_set_value_cansleep(
        (*priv_).spkvdd_en_gpio,
        (SND_SOC_DAPM_EVENT_ON(event) != 0) as c_int,
    );

    0
}

unsafe fn byt_wm5102_prepare_and_enable_pll1(
    codec_dai: *mut snd_soc_dai,
    rate: c_int,
) -> c_int {
    let codec_component = (*codec_dai).component;
    let priv_ = snd_soc_card_get_drvdata((*codec_component).card) as *mut byt_wm5102_private;
    let sr_mult = if rate % 4000 == 0 {
        WM5102_MAX_SYSCLK_4K / rate
    } else {
        WM5102_MAX_SYSCLK_11025 / rate
    };
    let mut ret: c_int;

    /* Reset FLL1 */
    snd_soc_dai_set_pll(codec_dai, WM5102_FLL1_REFCLK, ARIZONA_FLL_SRC_NONE, 0, 0);
    snd_soc_dai_set_pll(codec_dai, WM5102_FLL1, ARIZONA_FLL_SRC_NONE, 0, 0);

    /* Configure the FLL1 PLL before selecting it */
    ret = snd_soc_dai_set_pll(
        codec_dai,
        WM5102_FLL1,
        ARIZONA_CLK_SRC_MCLK1,
        (*priv_).mclk_freq,
        rate * sr_mult,
    );
    if ret != 0 {
        dev_err((*codec_component).dev, c"Error setting PLL: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_component_set_sysclk(
        codec_component,
        ARIZONA_CLK_SYSCLK,
        ARIZONA_CLK_SRC_FLL1,
        rate * sr_mult,
        SND_SOC_CLOCK_IN,
    );
    if ret != 0 {
        dev_err((*codec_component).dev, c"Error setting SYSCLK: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(codec_dai, ARIZONA_CLK_SYSCLK, rate * 512, SND_SOC_CLOCK_IN);
    if ret != 0 {
        dev_err((*codec_component).dev, c"Error setting clock: %d\n".as_ptr(), ret);
        return ret;
    }

    0
}

unsafe extern "C" fn platform_clock_control(
    w: *mut snd_soc_dapm_widget,
    _k: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card = snd_soc_dapm_to_card((*w).dapm);
    let mut codec_dai: *mut snd_soc_dai;
    let priv_ = snd_soc_card_get_drvdata(card) as *mut byt_wm5102_private;
    let mut ret: c_int;

    codec_dai = snd_soc_card_get_codec_dai(card, c"wm5102-aif1".as_ptr());
    if codec_dai.is_null() {
        dev_err((*card).dev, c"Error codec DAI not found\n".as_ptr());
        return -EIO;
    }

    if SND_SOC_DAPM_EVENT_ON(event) != 0 {
        ret = clk_prepare_enable((*priv_).mclk);
        if ret != 0 {
            dev_err((*card).dev, c"Error enabling MCLK: %d\n".as_ptr(), ret);
            return ret;
        }
        ret = byt_wm5102_prepare_and_enable_pll1(codec_dai, 48000);
        if ret != 0 {
            dev_err((*card).dev, c"Error setting codec sysclk: %d\n".as_ptr(), ret);
            clk_disable_unprepare((*priv_).mclk);
            return ret;
        }
    } else {
        /*
         * The WM5102 has a separate 32KHz clock for jack-detect
         * so we can disable the PLL, followed by disabling the
         * platform clock which is the source-clock for the PLL.
         */
        snd_soc_dai_set_pll(codec_dai, WM5102_FLL1, ARIZONA_FLL_SRC_NONE, 0, 0);
        clk_disable_unprepare((*priv_).mclk);
    }

    0
}

/* DAPM widget macros depend on external ASoC kernel definitions. */
static byt_wm5102_widgets: [snd_soc_dapm_widget; 7] = [
    snd_soc_dapm_widget::hp(c"Headphone".as_ptr(), ptr::null()),
    snd_soc_dapm_widget::mic(c"Headset Mic".as_ptr(), ptr::null()),
    snd_soc_dapm_widget::mic(c"Internal Mic".as_ptr(), ptr::null()),
    snd_soc_dapm_widget::spk(c"Speaker".as_ptr(), ptr::null()),
    snd_soc_dapm_widget::line(c"Line Out".as_ptr(), ptr::null()),
    snd_soc_dapm_widget::supply(
        c"Platform Clock".as_ptr(),
        SND_SOC_NOPM,
        0,
        0,
        Some(platform_clock_control),
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD,
    ),
    snd_soc_dapm_widget::supply(
        c"Speaker VDD".as_ptr(),
        SND_SOC_NOPM,
        0,
        0,
        Some(byt_wm5102_spkvdd_power_event),
        SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU,
    ),
];

static byt_wm5102_audio_map: [snd_soc_dapm_route; 10] = [
    snd_soc_dapm_route::new(c"Headphone".as_ptr(), ptr::null(), c"Platform Clock".as_ptr()),
    snd_soc_dapm_route::new(c"Headset Mic".as_ptr(), ptr::null(), c"Platform Clock".as_ptr()),
    snd_soc_dapm_route::new(c"Internal Mic".as_ptr(), ptr::null(), c"Platform Clock".as_ptr()),
    snd_soc_dapm_route::new(c"Speaker".as_ptr(), ptr::null(), c"Platform Clock".as_ptr()),
    snd_soc_dapm_route::new(c"Speaker".as_ptr(), ptr::null(), c"Speaker VDD".as_ptr()),
    snd_soc_dapm_route::new(c"Headphone".as_ptr(), ptr::null(), c"HPOUT1L".as_ptr()),
    snd_soc_dapm_route::new(c"Headphone".as_ptr(), ptr::null(), c"HPOUT1R".as_ptr()),
    /*
     * The Headset Mix uses MICBIAS1 or 2 depending on if a CTIA/OMTP Headset
     * is connected, as the MICBIAS is applied after the CTIA/OMTP cross-switch.
     */
    snd_soc_dapm_route::new(c"Headset Mic".as_ptr(), ptr::null(), c"MICBIAS1".as_ptr()),
    snd_soc_dapm_route::new(c"Headset Mic".as_ptr(), ptr::null(), c"MICBIAS2".as_ptr()),
    snd_soc_dapm_route::new(c"Internal Mic".as_ptr(), ptr::null(), c"MICBIAS3".as_ptr()),
];

static bytcr_wm5102_ssp0_map: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route::new(c"AIF1 Playback".as_ptr(), ptr::null(), c"ssp0 Tx".as_ptr()),
    snd_soc_dapm_route::new(c"ssp0 Tx".as_ptr(), ptr::null(), c"modem_out".as_ptr()),
    snd_soc_dapm_route::new(c"modem_in".as_ptr(), ptr::null(), c"ssp0 Rx".as_ptr()),
    snd_soc_dapm_route::new(c"ssp0 Rx".as_ptr(), ptr::null(), c"AIF1 Capture".as_ptr()),
];

static bytcr_wm5102_ssp2_map: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route::new(c"AIF1 Playback".as_ptr(), ptr::null(), c"ssp2 Tx".as_ptr()),
    snd_soc_dapm_route::new(c"ssp2 Tx".as_ptr(), ptr::null(), c"codec_out0".as_ptr()),
    snd_soc_dapm_route::new(c"ssp2 Tx".as_ptr(), ptr::null(), c"codec_out1".as_ptr()),
    snd_soc_dapm_route::new(c"codec_in0".as_ptr(), ptr::null(), c"ssp2 Rx".as_ptr()),
    snd_soc_dapm_route::new(c"codec_in1".as_ptr(), ptr::null(), c"ssp2 Rx".as_ptr()),
    snd_soc_dapm_route::new(c"ssp2 Rx".as_ptr(), ptr::null(), c"AIF1 Capture".as_ptr()),
];

static byt_wm5102_spk_spk_map: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route::new(c"Speaker".as_ptr(), ptr::null(), c"SPKOUTLP".as_ptr()),
    snd_soc_dapm_route::new(c"Speaker".as_ptr(), ptr::null(), c"SPKOUTLN".as_ptr()),
    snd_soc_dapm_route::new(c"Speaker".as_ptr(), ptr::null(), c"SPKOUTRP".as_ptr()),
    snd_soc_dapm_route::new(c"Speaker".as_ptr(), ptr::null(), c"SPKOUTRN".as_ptr()),
];

static byt_wm5102_spk_hpout2_map: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route::new(c"Speaker".as_ptr(), ptr::null(), c"HPOUT2L".as_ptr()),
    snd_soc_dapm_route::new(c"Speaker".as_ptr(), ptr::null(), c"HPOUT2R".as_ptr()),
];

static byt_wm5102_intmic_in3l_hsmic_in1l_map: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route::new(c"IN3L".as_ptr(), ptr::null(), c"Internal Mic".as_ptr()),
    snd_soc_dapm_route::new(c"IN1L".as_ptr(), ptr::null(), c"Headset Mic".as_ptr()),
];

static byt_wm5102_intmic_in1l_hsmic_in2l_map: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route::new(c"IN1L".as_ptr(), ptr::null(), c"Internal Mic".as_ptr()),
    snd_soc_dapm_route::new(c"IN2L".as_ptr(), ptr::null(), c"Headset Mic".as_ptr()),
];

static byt_wm5102_controls: [snd_kcontrol_new; 5] = [
    snd_kcontrol_new::soc_dapm_pin_switch(c"Headphone".as_ptr()),
    snd_kcontrol_new::soc_dapm_pin_switch(c"Headset Mic".as_ptr()),
    snd_kcontrol_new::soc_dapm_pin_switch(c"Internal Mic".as_ptr()),
    snd_kcontrol_new::soc_dapm_pin_switch(c"Speaker".as_ptr()),
    snd_kcontrol_new::soc_dapm_pin_switch(c"Line Out".as_ptr()),
];

static mut byt_wm5102_pins: [snd_soc_jack_pin; 3] = [
    snd_soc_jack_pin {
        pin: c"Headphone".as_ptr(),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: c"Headset Mic".as_ptr(),
        mask: SND_JACK_MICROPHONE,
    },
    snd_soc_jack_pin {
        pin: c"Line Out".as_ptr(),
        mask: SND_JACK_LINEOUT,
    },
];

unsafe extern "C" fn byt_wm5102_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*runtime).card;
    let dapm = snd_soc_card_to_dapm(card);
    let priv_ = snd_soc_card_get_drvdata(card) as *mut byt_wm5102_private;
    let component = (*snd_soc_rtd_to_codec(runtime, 0)).component;
    let mut custom_map: *const snd_soc_dapm_route = ptr::null();
    let mut ret: c_int;
    let jack_type: c_int;
    let mut num_routes: c_int = 0;

    snd_soc_dapm_set_idle_bias(dapm, false);

    ret = snd_soc_add_card_controls(
        card,
        byt_wm5102_controls.as_ptr(),
        byt_wm5102_controls.len() as c_uint,
    );
    if ret != 0 {
        dev_err((*card).dev, c"Error adding card controls: %d\n".as_ptr(), ret);
        return ret;
    }

    match quirk & BYT_WM5102_IN_MAP {
        BYT_WM5102_INTMIC_IN3L_HSMIC_IN1L => {
            custom_map = byt_wm5102_intmic_in3l_hsmic_in1l_map.as_ptr();
            num_routes = byt_wm5102_intmic_in3l_hsmic_in1l_map.len() as c_int;
        }
        BYT_WM5102_INTMIC_IN1L_HSMIC_IN2L => {
            custom_map = byt_wm5102_intmic_in1l_hsmic_in2l_map.as_ptr();
            num_routes = byt_wm5102_intmic_in1l_hsmic_in2l_map.len() as c_int;
        }
        _ => {}
    }
    ret = snd_soc_dapm_add_routes(dapm, custom_map, num_routes);
    if ret != 0 {
        return ret;
    }

    match quirk & BYT_WM5102_OUT_MAP {
        BYT_WM5102_SPK_SPK_MAP => {
            custom_map = byt_wm5102_spk_spk_map.as_ptr();
            num_routes = byt_wm5102_spk_spk_map.len() as c_int;
        }
        BYT_WM5102_SPK_HPOUT2_MAP => {
            custom_map = byt_wm5102_spk_hpout2_map.as_ptr();
            num_routes = byt_wm5102_spk_hpout2_map.len() as c_int;
        }
        _ => {}
    }
    ret = snd_soc_dapm_add_routes(dapm, custom_map, num_routes);
    if ret != 0 {
        return ret;
    }

    if quirk & BYT_WM5102_SSP2 != 0 {
        custom_map = bytcr_wm5102_ssp2_map.as_ptr();
        num_routes = bytcr_wm5102_ssp2_map.len() as c_int;
    } else {
        custom_map = bytcr_wm5102_ssp0_map.as_ptr();
        num_routes = bytcr_wm5102_ssp0_map.len() as c_int;
    }
    ret = snd_soc_dapm_add_routes(dapm, custom_map, num_routes);
    if ret != 0 {
        return ret;
    }

    if quirk & BYT_WM5102_MCLK_19_2MHZ != 0 {
        (*priv_).mclk_freq = 19200000;
    } else {
        (*priv_).mclk_freq = 25000000;
    }

    /*
     * The firmware might enable the clock at boot (this information
     * may or may not be reflected in the enable clock register).
     * To change the rate we must disable the clock first to cover these
     * cases. Due to common clock framework restrictions that do not allow
     * to disable a clock that has not been enabled, we need to enable
     * the clock first.
     */
    ret = clk_prepare_enable((*priv_).mclk);
    if ret == 0 {
        clk_disable_unprepare((*priv_).mclk);
    }

    ret = clk_set_rate((*priv_).mclk, (*priv_).mclk_freq as c_ulong);
    if ret != 0 {
        dev_err((*card).dev, c"Error setting MCLK rate: %d\n".as_ptr(), ret);
        return ret;
    }

    jack_type = ARIZONA_JACK_MASK | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3;
    ret = snd_soc_card_jack_new_pins(
        card,
        c"Headset".as_ptr(),
        jack_type,
        &mut (*priv_).jack,
        byt_wm5102_pins.as_mut_ptr(),
        byt_wm5102_pins.len() as c_uint,
    );
    if ret != 0 {
        dev_err((*card).dev, c"Error creating jack: %d\n".as_ptr(), ret);
        return ret;
    }

    snd_soc_component_set_jack(component, &mut (*priv_).jack, ptr::null_mut());

    0
}

unsafe extern "C" fn byt_wm5102_codec_fixup(
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

    if quirk & BYT_WM5102_SSP2 != 0 {
        /* set SSP2 to 24-bit */
        params_set_format(params, SNDRV_PCM_FORMAT_S24_LE);
        bits = 24;
    } else {
        /* set SSP0 to 16-bit */
        params_set_format(params, SNDRV_PCM_FORMAT_S16_LE);
        bits = 16;
    }

    /*
     * Default mode for SSP configuration is TDM 4 slot, override config
     * with explicit setting to I2S 2ch 16-bit. The word length is set with
     * dai_set_tdm_slot() since there is no other API exposed
     */
    ret = snd_soc_dai_set_fmt(
        snd_soc_rtd_to_cpu(rtd, 0),
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_BP_FP,
    );
    if ret != 0 {
        dev_err((*rtd).dev, c"Error setting format to I2S: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_dai_set_tdm_slot(snd_soc_rtd_to_cpu(rtd, 0), 0x3, 0x3, 2, bits);
    if ret != 0 {
        dev_err((*rtd).dev, c"Error setting I2S config: %d\n".as_ptr(), ret);
        return ret;
    }

    0
}

unsafe extern "C" fn byt_wm5102_aif1_startup(substream: *mut snd_pcm_substream) -> c_int {
    snd_pcm_hw_constraint_single((*substream).runtime, SNDRV_PCM_HW_PARAM_RATE, 48000)
}

static byt_wm5102_aif1_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(byt_wm5102_aif1_startup),
};

/* SND_SOC_DAILINK_DEF(dummy, DAILINK_COMP_ARRAY(COMP_DUMMY())); */
static mut dummy: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component::dummy()];
/* SND_SOC_DAILINK_DEF(media, DAILINK_COMP_ARRAY(COMP_CPU("media-cpu-dai"))); */
static mut media: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component::cpu(c"media-cpu-dai".as_ptr())];
/* SND_SOC_DAILINK_DEF(deepbuffer, DAILINK_COMP_ARRAY(COMP_CPU("deepbuffer-cpu-dai"))); */
static mut deepbuffer: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component::cpu(c"deepbuffer-cpu-dai".as_ptr())];
/* SND_SOC_DAILINK_DEF(ssp0_port, DAILINK_COMP_ARRAY(COMP_CPU("ssp0-port"))); */
static mut ssp0_port: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component::cpu(c"ssp0-port".as_ptr())];
/* SND_SOC_DAILINK_DEF(ssp0_codec, DAILINK_COMP_ARRAY(COMP_CODEC(...))); */
static mut ssp0_codec: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component::codec(
    /*
     * Note there is no need to overwrite the codec-name as is done in
     * other bytcr machine drivers, because the codec is a MFD child-dev.
     */
    c"wm5102-codec".as_ptr(),
    c"wm5102-aif1".as_ptr(),
)];
/* SND_SOC_DAILINK_DEF(platform, DAILINK_COMP_ARRAY(COMP_PLATFORM("sst-mfld-platform"))); */
static mut platform: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component::platform(c"sst-mfld-platform".as_ptr())];

static mut byt_wm5102_dais: [snd_soc_dai_link; 3] = [
    snd_soc_dai_link {
        name: c"Baytrail Audio Port".as_ptr(),
        stream_name: c"Baytrail Audio".as_ptr(),
        nonatomic: true,
        dynamic: 1,
        ops: &byt_wm5102_aif1_ops,
        cpus: unsafe { media.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { dummy.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { platform.as_mut_ptr() },
        num_platforms: 1,
        ..snd_soc_dai_link::zero()
    },
    snd_soc_dai_link {
        name: c"Deep-Buffer Audio Port".as_ptr(),
        stream_name: c"Deep-Buffer Audio".as_ptr(),
        nonatomic: true,
        dynamic: 1,
        playback_only: 1,
        ops: &byt_wm5102_aif1_ops,
        cpus: unsafe { deepbuffer.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { dummy.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { platform.as_mut_ptr() },
        num_platforms: 1,
        ..snd_soc_dai_link::zero()
    },
    /* back ends */
    snd_soc_dai_link {
        /*
         * This dailink is updated dynamically to point to SSP0 or SSP2.
         * Yet its name is always kept as "SSP2-Codec" because the SOF
         * tplg files hardcode "SSP2-Codec" even in byt-foo-ssp0.tplg.
         */
        name: c"SSP2-Codec".as_ptr(),
        id: 0,
        no_pcm: 1,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        be_hw_params_fixup: Some(byt_wm5102_codec_fixup),
        init: Some(byt_wm5102_init),
        cpus: unsafe { ssp0_port.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { ssp0_codec.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { platform.as_mut_ptr() },
        num_platforms: 1,
        ..snd_soc_dai_link::zero()
    },
];

/* use space before codec name to simplify card ID, and simplify driver name */
const SOF_CARD_NAME: *const c_char = c"bytcht wm5102".as_ptr(); /* card name will be 'sof-bytcht wm5102' */
const SOF_DRIVER_NAME: *const c_char = c"SOF".as_ptr();

const CARD_NAME: *const c_char = c"bytcr-wm5102".as_ptr();
const DRIVER_NAME: *const c_char = ptr::null(); /* card name will be used for driver name */

/* SoC card */
static mut byt_wm5102_card: snd_soc_card = snd_soc_card {
    owner: THIS_MODULE,
    dai_link: unsafe { byt_wm5102_dais.as_mut_ptr() },
    num_links: 3,
    dapm_widgets: byt_wm5102_widgets.as_ptr(),
    num_dapm_widgets: 7,
    dapm_routes: byt_wm5102_audio_map.as_ptr(),
    num_dapm_routes: 10,
    fully_routed: true,
    ..snd_soc_card::zero()
};

static mut byt_wm5102_components: [c_char; 64] = [0; 64]; /* = "cfg-spk:* cfg-int-mic:* cfg-hs-mic:* ..." */

unsafe extern "C" fn snd_byt_wm5102_mc_probe(pdev: *mut platform_device) -> c_int {
    static out_map_name: [*const c_char; 2] = [c"spk".as_ptr(), c"hpout2".as_ptr()];
    static intmic_map_name: [*const c_char; 2] = [c"in3l".as_ptr(), c"in1l".as_ptr()];
    static hsmic_map_name: [*const c_char; 2] = [c"in1l".as_ptr(), c"in2l".as_ptr()];
    let mut codec_name: [c_char; SND_ACPI_I2C_ID_LEN] = [0; SND_ACPI_I2C_ID_LEN];
    let dev = &mut (*pdev).dev as *mut device;
    let mut priv_: *mut byt_wm5102_private;
    let mach: *mut snd_soc_acpi_mach;
    let platform_name: *const c_char;
    let mut adev: *mut acpi_device;
    let codec_dev: *mut device;
    let mut dai_index: c_int = 0;
    let sof_parent: bool;
    let mut i: usize;
    let mut ret: c_int;

    priv_ = devm_kzalloc(dev, size_of::<byt_wm5102_private>(), GFP_KERNEL) as *mut byt_wm5102_private;
    if priv_.is_null() {
        return -ENOMEM;
    }

    /* Get MCLK */
    (*priv_).mclk = devm_clk_get(dev, c"pmc_plt_clk_3".as_ptr());
    if IS_ERR((*priv_).mclk as *const c_void) {
        return dev_err_probe(dev, PTR_ERR((*priv_).mclk as *const c_void) as c_int, c"getting pmc_plt_clk_3\n".as_ptr());
    }

    /*
     * Get speaker VDD enable GPIO:
     * 1. Get codec-device-name
     * 2. Get codec-device
     * 3. Get GPIO from codec-device
     */
    mach = (*dev).platform_data as *mut snd_soc_acpi_mach;
    adev = acpi_dev_get_first_match_dev((*mach).id, ptr::null(), -1);
    if !adev.is_null() {
        snprintf(
            codec_name.as_mut_ptr(),
            codec_name.len(),
            c"spi-%s".as_ptr(),
            acpi_dev_name(adev),
        );
        acpi_dev_put(adev);
    } else {
        /* Special case for when the codec is missing from the DSTD */
        strscpy(codec_name.as_mut_ptr(), c"spi-wm5102".as_ptr(), codec_name.len());
    }

    codec_dev = bus_find_device_by_name(&spi_bus_type, ptr::null_mut(), codec_name.as_ptr());
    if codec_dev.is_null() {
        return -EPROBE_DEFER;
    }

    /* Note no devm_ here since we call gpiod_get on codec_dev rather then dev */
    (*priv_).spkvdd_en_gpio = gpiod_get(codec_dev, c"wlf,spkvdd-ena".as_ptr(), GPIOD_OUT_LOW);
    put_device(codec_dev);

    if IS_ERR((*priv_).spkvdd_en_gpio as *const c_void) {
        ret = PTR_ERR((*priv_).spkvdd_en_gpio as *const c_void) as c_int;
        /*
         * The spkvdd gpio-lookup is registered by: drivers/mfd/arizona-spi.c,
         * so -ENOENT means that arizona-spi hasn't probed yet.
         */
        if ret == -ENOENT {
            ret = -EPROBE_DEFER;
        }

        return dev_err_probe(dev, ret, c"getting spkvdd-GPIO\n".as_ptr());
    }

    if soc_intel_is_cht() {
        /*
         * CHT always uses SSP2 and 19.2 MHz; and
         * the one currently supported CHT design uses HPOUT2 as
         * speaker output and has the intmic on IN1L + hsmic on IN2L.
         */
        quirk = BYT_WM5102_SSP2
            | BYT_WM5102_MCLK_19_2MHZ
            | BYT_WM5102_INTMIC_IN1L_HSMIC_IN2L
            | BYT_WM5102_SPK_HPOUT2_MAP;
    }
    if quirk_override != -1 {
        dev_info_once(
            dev,
            c"Overriding quirk 0x%lx => 0x%x\n".as_ptr(),
            quirk,
            quirk_override,
        );
        quirk = quirk_override as c_ulong;
    }
    log_quirks(dev);

    snprintf(
        byt_wm5102_components.as_mut_ptr(),
        byt_wm5102_components.len(),
        c"cfg-spk:%s cfg-intmic:%s cfg-hsmic:%s".as_ptr(),
        out_map_name[field_get(BYT_WM5102_OUT_MAP, quirk) as usize],
        intmic_map_name[field_get(BYT_WM5102_IN_MAP, quirk) as usize],
        hsmic_map_name[field_get(BYT_WM5102_IN_MAP, quirk) as usize],
    );
    byt_wm5102_card.components = byt_wm5102_components.as_mut_ptr();

    /* find index of codec dai */
    i = 0;
    while i < byt_wm5102_dais.len() {
        if byt_wm5102_dais[i].num_codecs != 0
            && strcmp((*byt_wm5102_dais[i].codecs).name, c"wm5102-codec".as_ptr()) == 0
        {
            dai_index = i as c_int;
            break;
        }
        i += 1;
    }

    /* override platform name, if required */
    byt_wm5102_card.dev = dev;
    platform_name = (*mach).mach_params.platform;
    ret = snd_soc_fixup_dai_links_platform_name(&mut byt_wm5102_card, platform_name);
    if ret != 0 {
        gpiod_put((*priv_).spkvdd_en_gpio);
        return ret;
    }

    /* override SSP port, if required */
    if quirk & BYT_WM5102_SSP2 != 0 {
        (*byt_wm5102_dais[dai_index as usize].cpus).dai_name = c"ssp2-port".as_ptr();
    }

    /* set card and driver name and pm-ops */
    sof_parent = snd_soc_acpi_sof_parent(dev);
    if sof_parent {
        byt_wm5102_card.name = SOF_CARD_NAME;
        byt_wm5102_card.driver_name = SOF_DRIVER_NAME;
        (*(*dev).driver).pm = &snd_soc_pm_ops;
    } else {
        byt_wm5102_card.name = CARD_NAME;
        byt_wm5102_card.driver_name = DRIVER_NAME;
    }

    snd_soc_card_set_drvdata(&mut byt_wm5102_card, priv_ as *mut c_void);
    ret = devm_snd_soc_register_card(dev, &mut byt_wm5102_card);
    if ret != 0 {
        dev_err_probe(dev, ret, c"registering card\n".as_ptr());
        gpiod_put((*priv_).spkvdd_en_gpio);
        return ret;
    }

    platform_set_drvdata(pdev, &mut byt_wm5102_card as *mut _ as *mut c_void);
    0
}

unsafe extern "C" fn snd_byt_wm5102_mc_remove(pdev: *mut platform_device) {
    let card = platform_get_drvdata(pdev) as *mut snd_soc_card;
    let priv_ = snd_soc_card_get_drvdata(card) as *mut byt_wm5102_private;

    gpiod_put((*priv_).spkvdd_en_gpio);
}

static mut snd_byt_wm5102_mc_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"bytcr_wm5102".as_ptr(),
        ..device_driver::zero()
    },
    probe: Some(snd_byt_wm5102_mc_probe),
    remove: Some(snd_byt_wm5102_mc_remove),
};

/* module_platform_driver(snd_byt_wm5102_mc_driver); */

/* MODULE_DESCRIPTION("ASoC Baytrail with WM5102 codec machine driver"); */
/* MODULE_AUTHOR("Hans de Goede <hdegoede@redhat.com>"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_ALIAS("platform:bytcr_wm5102"); */

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_desc {
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
pub struct snd_soc_component {
    card: *mut snd_soc_card,
    dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_dai {
    component: *mut snd_soc_component,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}
#[repr(C)]
pub struct snd_interval {
    min: c_uint,
    max: c_uint,
}
#[repr(C)]
pub struct acpi_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bus_type {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device_driver {
    name: *const c_char,
    pm: *const dev_pm_ops,
}
impl device_driver {
    const fn zero() -> Self {
        Self {
            name: ptr::null(),
            pm: ptr::null(),
        }
    }
}
#[repr(C)]
pub struct device {
    platform_data: *mut c_void,
    driver: *mut device_driver,
}
#[repr(C)]
pub struct platform_device {
    dev: device,
}
#[repr(C)]
pub struct snd_soc_acpi_mach_params {
    platform: *const c_char,
}
#[repr(C)]
pub struct snd_soc_acpi_mach {
    id: *const c_char,
    mach_params: snd_soc_acpi_mach_params,
}
#[repr(C)]
pub struct snd_soc_pcm_runtime {
    card: *mut snd_soc_card,
    dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    dapm: *mut snd_soc_dapm_context,
    name: *const c_char,
}
impl snd_soc_dapm_widget {
    const fn hp(name: *const c_char, _reg: *const c_void) -> Self {
        Self { dapm: ptr::null_mut(), name }
    }
    const fn mic(name: *const c_char, _reg: *const c_void) -> Self {
        Self { dapm: ptr::null_mut(), name }
    }
    const fn spk(name: *const c_char, _reg: *const c_void) -> Self {
        Self { dapm: ptr::null_mut(), name }
    }
    const fn line(name: *const c_char, _reg: *const c_void) -> Self {
        Self { dapm: ptr::null_mut(), name }
    }
    const fn supply(
        name: *const c_char,
        _reg: c_int,
        _shift: c_int,
        _invert: c_int,
        _event: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>,
        _flags: c_int,
    ) -> Self {
        Self { dapm: ptr::null_mut(), name }
    }
}
#[repr(C)]
pub struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}
impl snd_soc_dapm_route {
    const fn new(sink: *const c_char, control: *const c_char, source: *const c_char) -> Self {
        Self { sink, control, source }
    }
}
#[repr(C)]
pub struct snd_kcontrol_new {
    name: *const c_char,
}
impl snd_kcontrol_new {
    const fn soc_dapm_pin_switch(name: *const c_char) -> Self {
        Self { name }
    }
}
#[repr(C)]
pub struct snd_soc_jack_pin {
    pin: *const c_char,
    mask: c_int,
}
#[repr(C)]
pub struct snd_soc_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
}
#[repr(C)]
pub struct snd_soc_dai_link_component {
    name: *const c_char,
    dai_name: *const c_char,
}
impl snd_soc_dai_link_component {
    const fn dummy() -> Self {
        Self { name: ptr::null(), dai_name: ptr::null() }
    }
    const fn cpu(dai_name: *const c_char) -> Self {
        Self { name: ptr::null(), dai_name }
    }
    const fn codec(name: *const c_char, dai_name: *const c_char) -> Self {
        Self { name, dai_name }
    }
    const fn platform(name: *const c_char) -> Self {
        Self { name, dai_name: ptr::null() }
    }
}
#[repr(C)]
pub struct snd_soc_dai_link {
    name: *const c_char,
    stream_name: *const c_char,
    nonatomic: bool,
    dynamic: c_int,
    playback_only: c_int,
    ops: *const snd_soc_ops,
    cpus: *mut snd_soc_dai_link_component,
    num_cpus: c_uint,
    codecs: *mut snd_soc_dai_link_component,
    num_codecs: c_uint,
    platforms: *mut snd_soc_dai_link_component,
    num_platforms: c_uint,
    id: c_int,
    no_pcm: c_int,
    dai_fmt: c_uint,
    be_hw_params_fixup: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
}
impl snd_soc_dai_link {
    const fn zero() -> Self {
        Self {
            name: ptr::null(),
            stream_name: ptr::null(),
            nonatomic: false,
            dynamic: 0,
            playback_only: 0,
            ops: ptr::null(),
            cpus: ptr::null_mut(),
            num_cpus: 0,
            codecs: ptr::null_mut(),
            num_codecs: 0,
            platforms: ptr::null_mut(),
            num_platforms: 0,
            id: 0,
            no_pcm: 0,
            dai_fmt: 0,
            be_hw_params_fixup: None,
            init: None,
        }
    }
}
#[repr(C)]
pub struct snd_soc_card {
    owner: *mut c_void,
    dai_link: *mut snd_soc_dai_link,
    num_links: c_int,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_int,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_int,
    fully_routed: bool,
    components: *mut c_char,
    dev: *mut device,
    name: *const c_char,
    driver_name: *const c_char,
}
impl snd_soc_card {
    const fn zero() -> Self {
        Self {
            owner: ptr::null_mut(),
            dai_link: ptr::null_mut(),
            num_links: 0,
            dapm_widgets: ptr::null(),
            num_dapm_widgets: 0,
            dapm_routes: ptr::null(),
            num_dapm_routes: 0,
            fully_routed: false,
            components: ptr::null_mut(),
            dev: ptr::null_mut(),
            name: ptr::null(),
            driver_name: ptr::null(),
        }
    }
}
#[repr(C)]
pub struct platform_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const EPROBE_DEFER: c_int = 517;
const GFP_KERNEL: c_uint = 0;
const SND_ACPI_I2C_ID_LEN: usize = 32;
const GPIOD_OUT_LOW: c_int = 0;
const THIS_MODULE: *mut c_void = ptr::null_mut();
const SND_SOC_NOPM: c_int = -1;
const SND_SOC_DAPM_PRE_PMU: c_int = 1;
const SND_SOC_DAPM_POST_PMD: c_int = 2;
const SND_SOC_DAPM_PRE_PMD: c_int = 4;
const SND_SOC_DAPM_POST_PMU: c_int = 8;
const SND_SOC_CLOCK_IN: c_int = 0;
const WM5102_FLL1_REFCLK: c_int = 0;
const WM5102_FLL1: c_int = 1;
const ARIZONA_FLL_SRC_NONE: c_int = 0;
const ARIZONA_CLK_SRC_MCLK1: c_int = 1;
const ARIZONA_CLK_SYSCLK: c_int = 0;
const ARIZONA_CLK_SRC_FLL1: c_int = 2;
const ARIZONA_JACK_MASK: c_int = 0;
const SND_JACK_HEADPHONE: c_int = 0x0001;
const SND_JACK_MICROPHONE: c_int = 0x0002;
const SND_JACK_LINEOUT: c_int = 0x0004;
const SND_JACK_BTN_0: c_int = 0x4000;
const SND_JACK_BTN_1: c_int = 0x2000;
const SND_JACK_BTN_2: c_int = 0x1000;
const SND_JACK_BTN_3: c_int = 0x0800;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 1;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 6;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_NB_NF: c_uint = 2;
const SND_SOC_DAIFMT_BP_FP: c_uint = 4;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 8;

fn SND_SOC_DAPM_EVENT_ON(event: c_int) -> c_int {
    event & SND_SOC_DAPM_POST_PMU
}

unsafe extern "C" {
    static spi_bus_type: bus_type;
    static snd_soc_pm_ops: dev_pm_ops;

    fn dev_info_once(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn_once(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn gpiod_get(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;
    fn gpiod_put(desc: *mut gpio_desc);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_card_get_codec_dai(card: *mut snd_soc_card, dai_name: *const c_char) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_pll(dai: *mut snd_soc_dai, pll_id: c_int, source: c_int, freq_in: c_uint, freq_out: c_uint) -> c_int;
    fn snd_soc_component_set_sysclk(component: *mut snd_soc_component, clk_id: c_int, source: c_int, freq: c_uint, dir: c_int) -> c_int;
    fn snd_soc_dai_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, dir: c_int) -> c_int;
    fn snd_soc_dapm_set_idle_bias(dapm: *mut snd_soc_dapm_context, idle_bias: bool);
    fn snd_soc_add_card_controls(card: *mut snd_soc_card, controls: *const snd_kcontrol_new, num_controls: c_uint) -> c_int;
    fn snd_soc_dapm_add_routes(dapm: *mut snd_soc_dapm_context, route: *const snd_soc_dapm_route, num: c_int) -> c_int;
    fn snd_soc_card_jack_new_pins(card: *mut snd_soc_card, id: *const c_char, type_: c_int, jack: *mut snd_soc_jack, pins: *mut snd_soc_jack_pin, num_pins: c_uint) -> c_int;
    fn snd_soc_component_set_jack(component: *mut snd_soc_component, jack: *mut snd_soc_jack, data: *mut c_void) -> c_int;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn params_set_format(params: *mut snd_pcm_hw_params, val: c_int);
    fn snd_soc_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
    fn snd_soc_dai_set_tdm_slot(dai: *mut snd_soc_dai, tx_mask: c_uint, rx_mask: c_uint, slots: c_int, slot_width: c_int) -> c_int;
    fn snd_pcm_hw_constraint_single(runtime: *mut snd_pcm_runtime, var: c_int, val: c_uint) -> c_int;
    fn acpi_dev_get_first_match_dev(hid: *const c_char, uid: *const c_char, hrv: c_long) -> *mut acpi_device;
    fn acpi_dev_name(adev: *mut acpi_device) -> *const c_char;
    fn acpi_dev_put(adev: *mut acpi_device);
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> c_long;
    fn bus_find_device_by_name(bus: *const bus_type, start: *mut device, name: *const c_char) -> *mut device;
    fn put_device(dev: *mut device);
    fn soc_intel_is_cht() -> bool;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn snd_soc_fixup_dai_links_platform_name(card: *mut snd_soc_card, platform_name: *const c_char) -> c_int;
    fn snd_soc_acpi_sof_parent(dev: *mut device) -> bool;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
}

unsafe fn IS_ERR(ptr: *const c_void) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn PTR_ERR(ptr: *const c_void) -> c_long {
    ptr as c_long
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
