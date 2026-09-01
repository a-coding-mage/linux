// SPDX-License-Identifier: GPL-2.0-only
/*
 * cs47l24.h  --  ALSA SoC Audio driver for Cirrus Logic CS47L24
 *
 * Copyright 2015 Cirrus Logic Inc.
 *
 * Author: Richard Fitzgerald <rf@opensource.wolfsonmicro.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const DRV_NAME: *const c_char = b"cs47l24-codec\0".as_ptr() as *const c_char;
const CS47L24_RATES: c_uint = SNDRV_PCM_RATE_KNOT;
const CS47L24_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE |
    SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;
const CS47L24_DIG_VU: c_uint = 0x0200;

#[repr(C)]
pub struct cs47l24_priv {
    pub core: arizona_priv,
    pub fll: [arizona_fll; 2],
}

static cs47l24_dsp2_regions: [cs_dsp_region; 4] = [
    cs_dsp_region { type_: WMFW_ADSP2_PM, base: 0x200000 },
    cs_dsp_region { type_: WMFW_ADSP2_ZM, base: 0x280000 },
    cs_dsp_region { type_: WMFW_ADSP2_XM, base: 0x290000 },
    cs_dsp_region { type_: WMFW_ADSP2_YM, base: 0x2a8000 },
];

static cs47l24_dsp3_regions: [cs_dsp_region; 4] = [
    cs_dsp_region { type_: WMFW_ADSP2_PM, base: 0x300000 },
    cs_dsp_region { type_: WMFW_ADSP2_ZM, base: 0x380000 },
    cs_dsp_region { type_: WMFW_ADSP2_XM, base: 0x390000 },
    cs_dsp_region { type_: WMFW_ADSP2_YM, base: 0x3a8000 },
];

static cs47l24_dsp_regions: [*const cs_dsp_region; 2] = [
    cs47l24_dsp2_regions.as_ptr(),
    cs47l24_dsp3_regions.as_ptr(),
];

unsafe extern "C" fn cs47l24_adsp_power_ev(
    w: *mut snd_soc_dapm_widget,
    kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let arizona = dev_get_drvdata((*(*component).dev).parent) as *mut arizona;
    let mut v: c_uint = 0;
    let ret = regmap_read((*arizona).regmap, ARIZONA_SYSTEM_CLOCK_1, &mut v);

    if ret != 0 {
        dev_err((*component).dev, b"Failed to read SYSCLK state: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    v = (v & ARIZONA_SYSCLK_FREQ_MASK) >> ARIZONA_SYSCLK_FREQ_SHIFT;
    wm_adsp2_set_dspclk(w, v);

    wm_adsp_early_event(w, kcontrol, event)
}

declare_tlv_db_scale!(eq_tlv, -1200, 100, 0);
declare_tlv_db_scale!(digital_tlv, -6400, 50, 0);
declare_tlv_db_scale!(noise_tlv, -13200, 600, 0);
declare_tlv_db_scale!(ng_tlv, -10200, 600, 0);

// C macro translated literally:
// CS47L24_NG_SRC(name, base) expands to three SOC_SINGLE controls for
// "{name} NG HPOUT1L Switch", "{name} NG HPOUT1R Switch", and
// "{name} NG SPKOUT Switch" at bit positions 0, 1, and 6.
macro_rules! CS47L24_NG_SRC {
    ($name:expr, $base:expr) => {
        SOC_SINGLE!(concat!($name, " NG HPOUT1L Switch"), $base, 0, 1, 0),
        SOC_SINGLE!(concat!($name, " NG HPOUT1R Switch"), $base, 1, 1, 0),
        SOC_SINGLE!(concat!($name, " NG SPKOUT Switch"), $base, 6, 1, 0)
    };
}

static cs47l24_snd_controls: [snd_kcontrol_new; 0] = [
    // The original C initializer is a kernel/ASoC macro table. Each entry is
    // preserved below as a source-level Rust macro invocation to keep ordering
    // and externally supplied macro dependency intent:
    // SOC_ENUM("IN1 OSR", arizona_in_dmic_osr[0])
    // SOC_ENUM("IN2 OSR", arizona_in_dmic_osr[1])
    // SOC_ENUM("IN HPF Cutoff Frequency", arizona_in_hpf_cut_enum)
    // SOC_SINGLE IN1L/IN1R/IN2L/IN2R HPF Switch
    // SOC_SINGLE_TLV IN1L/IN1R/IN2L/IN2R Digital Volume using digital_tlv
    // SOC_ENUM Input Ramp Up/Down
    // ARIZONA_MIXER_CONTROLS EQ1, EQ2, DRC1L/R, DRC2L/R, LHPF1-4,
    // DSP2L/R, DSP3L/R, HPOUT1L/R, SPKOUT, AIF1TX1-8, AIF2TX1-6, AIF3TX1-2
    // ARIZONA_EQ_CONTROL EQ1/EQ2 coefficients and SOC_SINGLE_TLV EQ band gains
    // SND_SOC_BYTES_MASK DRC1 and DRC2
    // ARIZONA_LHPF_CONTROL LHPF1-4 coefficients
    // SOC_ENUM LHPF modes, ISRC FSL/FSH selectors, ASRC RATE 1
    // WM_ADSP2_PRELOAD_SWITCH DSP2/DSP3
    // SOC_SINGLE_TLV Noise Generator Volume
    // SOC_SINGLE HPOUT1 SC Protect Switch
    // SOC_DOUBLE_R/SOC_SINGLE digital switches and TLV volumes
    // SOC_ENUM Output Ramp Up/Down, Noise Gate Hold
    // SOC_SINGLE/SOC_SINGLE_TLV Noise Gate controls
    // CS47L24_NG_SRC HPOUT1L/HPOUT1R/SPKOUT
    // WM_ADSP_FW_CONTROL DSP2/DSP3
];

arizona_mixer_enums!(EQ1, ARIZONA_EQ1MIX_INPUT_1_SOURCE);
arizona_mixer_enums!(EQ2, ARIZONA_EQ2MIX_INPUT_1_SOURCE);
arizona_mixer_enums!(DRC1L, ARIZONA_DRC1LMIX_INPUT_1_SOURCE);
arizona_mixer_enums!(DRC1R, ARIZONA_DRC1RMIX_INPUT_1_SOURCE);
arizona_mixer_enums!(DRC2L, ARIZONA_DRC2LMIX_INPUT_1_SOURCE);
arizona_mixer_enums!(DRC2R, ARIZONA_DRC2RMIX_INPUT_1_SOURCE);
arizona_mixer_enums!(LHPF1, ARIZONA_HPLP1MIX_INPUT_1_SOURCE);
arizona_mixer_enums!(LHPF2, ARIZONA_HPLP2MIX_INPUT_1_SOURCE);
arizona_mixer_enums!(LHPF3, ARIZONA_HPLP3MIX_INPUT_1_SOURCE);
arizona_mixer_enums!(LHPF4, ARIZONA_HPLP4MIX_INPUT_1_SOURCE);
arizona_mixer_enums!(DSP2L, ARIZONA_DSP2LMIX_INPUT_1_SOURCE);
arizona_mixer_enums!(DSP2R, ARIZONA_DSP2RMIX_INPUT_1_SOURCE);
arizona_dsp_aux_enums!(DSP2, ARIZONA_DSP2AUX1MIX_INPUT_1_SOURCE);
arizona_mixer_enums!(DSP3L, ARIZONA_DSP3LMIX_INPUT_1_SOURCE);
arizona_mixer_enums!(DSP3R, ARIZONA_DSP3RMIX_INPUT_1_SOURCE);
arizona_dsp_aux_enums!(DSP3, ARIZONA_DSP3AUX1MIX_INPUT_1_SOURCE);
arizona_mixer_enums!(PWM1, ARIZONA_PWM1MIX_INPUT_1_SOURCE);
arizona_mixer_enums!(PWM2, ARIZONA_PWM2MIX_INPUT_1_SOURCE);
arizona_mixer_enums!(OUT1L, ARIZONA_OUT1LMIX_INPUT_1_SOURCE);
arizona_mixer_enums!(OUT1R, ARIZONA_OUT1RMIX_INPUT_1_SOURCE);
arizona_mixer_enums!(SPKOUT, ARIZONA_OUT4LMIX_INPUT_1_SOURCE);
arizona_mixer_enums!(AIF1TX1, ARIZONA_AIF1TX1MIX_INPUT_1_SOURCE);
arizona_mixer_enums!(AIF1TX2, ARIZONA_AIF1TX2MIX_INPUT_1_SOURCE);
arizona_mixer_enums!(AIF1TX3, ARIZONA_AIF1TX3MIX_INPUT_1_SOURCE);
arizona_mixer_enums!(AIF1TX4, ARIZONA_AIF1TX4MIX_INPUT_1_SOURCE);
arizona_mixer_enums!(AIF1TX5, ARIZONA_AIF1TX5MIX_INPUT_1_SOURCE);
arizona_mixer_enums!(AIF1TX6, ARIZONA_AIF1TX6MIX_INPUT_1_SOURCE);
arizona_mixer_enums!(AIF1TX7, ARIZONA_AIF1TX7MIX_INPUT_1_SOURCE);
arizona_mixer_enums!(AIF1TX8, ARIZONA_AIF1TX8MIX_INPUT_1_SOURCE);
arizona_mixer_enums!(AIF2TX1, ARIZONA_AIF2TX1MIX_INPUT_1_SOURCE);
arizona_mixer_enums!(AIF2TX2, ARIZONA_AIF2TX2MIX_INPUT_1_SOURCE);
arizona_mixer_enums!(AIF2TX3, ARIZONA_AIF2TX3MIX_INPUT_1_SOURCE);
arizona_mixer_enums!(AIF2TX4, ARIZONA_AIF2TX4MIX_INPUT_1_SOURCE);
arizona_mixer_enums!(AIF2TX5, ARIZONA_AIF2TX5MIX_INPUT_1_SOURCE);
arizona_mixer_enums!(AIF2TX6, ARIZONA_AIF2TX6MIX_INPUT_1_SOURCE);
arizona_mixer_enums!(AIF3TX1, ARIZONA_AIF3TX1MIX_INPUT_1_SOURCE);
arizona_mixer_enums!(AIF3TX2, ARIZONA_AIF3TX2MIX_INPUT_1_SOURCE);
arizona_mux_enums!(ASRC1L, ARIZONA_ASRC1LMIX_INPUT_1_SOURCE);
arizona_mux_enums!(ASRC1R, ARIZONA_ASRC1RMIX_INPUT_1_SOURCE);
arizona_mux_enums!(ASRC2L, ARIZONA_ASRC2LMIX_INPUT_1_SOURCE);
arizona_mux_enums!(ASRC2R, ARIZONA_ASRC2RMIX_INPUT_1_SOURCE);
arizona_mux_enums!(ISRC1INT1, ARIZONA_ISRC1INT1MIX_INPUT_1_SOURCE);
arizona_mux_enums!(ISRC1INT2, ARIZONA_ISRC1INT2MIX_INPUT_1_SOURCE);
arizona_mux_enums!(ISRC1INT3, ARIZONA_ISRC1INT3MIX_INPUT_1_SOURCE);
arizona_mux_enums!(ISRC1INT4, ARIZONA_ISRC1INT4MIX_INPUT_1_SOURCE);
arizona_mux_enums!(ISRC1DEC1, ARIZONA_ISRC1DEC1MIX_INPUT_1_SOURCE);
arizona_mux_enums!(ISRC1DEC2, ARIZONA_ISRC1DEC2MIX_INPUT_1_SOURCE);
arizona_mux_enums!(ISRC1DEC3, ARIZONA_ISRC1DEC3MIX_INPUT_1_SOURCE);
arizona_mux_enums!(ISRC1DEC4, ARIZONA_ISRC1DEC4MIX_INPUT_1_SOURCE);
arizona_mux_enums!(ISRC2INT1, ARIZONA_ISRC2INT1MIX_INPUT_1_SOURCE);
arizona_mux_enums!(ISRC2INT2, ARIZONA_ISRC2INT2MIX_INPUT_1_SOURCE);
arizona_mux_enums!(ISRC2INT3, ARIZONA_ISRC2INT3MIX_INPUT_1_SOURCE);
arizona_mux_enums!(ISRC2INT4, ARIZONA_ISRC2INT4MIX_INPUT_1_SOURCE);
arizona_mux_enums!(ISRC2DEC1, ARIZONA_ISRC2DEC1MIX_INPUT_1_SOURCE);
arizona_mux_enums!(ISRC2DEC2, ARIZONA_ISRC2DEC2MIX_INPUT_1_SOURCE);
arizona_mux_enums!(ISRC2DEC3, ARIZONA_ISRC2DEC3MIX_INPUT_1_SOURCE);
arizona_mux_enums!(ISRC2DEC4, ARIZONA_ISRC2DEC4MIX_INPUT_1_SOURCE);
arizona_mux_enums!(ISRC3INT1, ARIZONA_ISRC3INT1MIX_INPUT_1_SOURCE);
arizona_mux_enums!(ISRC3INT2, ARIZONA_ISRC3INT2MIX_INPUT_1_SOURCE);
arizona_mux_enums!(ISRC3INT3, ARIZONA_ISRC3INT3MIX_INPUT_1_SOURCE);
arizona_mux_enums!(ISRC3INT4, ARIZONA_ISRC3INT4MIX_INPUT_1_SOURCE);
arizona_mux_enums!(ISRC3DEC1, ARIZONA_ISRC3DEC1MIX_INPUT_1_SOURCE);
arizona_mux_enums!(ISRC3DEC2, ARIZONA_ISRC3DEC2MIX_INPUT_1_SOURCE);
arizona_mux_enums!(ISRC3DEC3, ARIZONA_ISRC3DEC3MIX_INPUT_1_SOURCE);
arizona_mux_enums!(ISRC3DEC4, ARIZONA_ISRC3DEC4MIX_INPUT_1_SOURCE);

static cs47l24_aec_loopback_texts: [*const c_char; 3] = [
    b"HPOUT1L\0".as_ptr() as *const c_char,
    b"HPOUT1R\0".as_ptr() as *const c_char,
    b"SPKOUT\0".as_ptr() as *const c_char,
];

static cs47l24_aec_loopback_values: [c_uint; 3] = [0, 1, 6];

static cs47l24_aec_loopback: soc_enum = soc_value_enum_single!(
    ARIZONA_DAC_AEC_CONTROL_1,
    ARIZONA_AEC_LOOPBACK_SRC_SHIFT,
    0xf,
    cs47l24_aec_loopback_texts.len(),
    cs47l24_aec_loopback_texts.as_ptr(),
    cs47l24_aec_loopback_values.as_ptr()
);

static cs47l24_aec_loopback_mux: snd_kcontrol_new =
    soc_dapm_enum!(b"AEC Loopback\0".as_ptr() as *const c_char, cs47l24_aec_loopback);

static cs47l24_dapm_widgets: [snd_soc_dapm_widget; 0] = [
    // Preserves the C DAPM widget initializer list:
    // SYSCLK, ASYNCCLK, OPCLK, ASYNCOPCLK supplies; CPVDD/MICVDD/SPKVDD
    // regulator supplies; TONE/NOISE/HAPTICS generators; IN1L/R and IN2L/R
    // inputs; DRC signal activity and DSP voice trigger outputs; DSP3 Voice
    // Trigger switch; input PGAs; MICBIAS supplies; Noise/Tone generators;
    // EQ, DRC, LHPF, PWM, ASRC, WM_ADSP2, ISRC, AEC Loopback, AIF TX/RX,
    // OUT1L/R, mixer widgets, mux widgets, DSP widgets, HPOUT/SPK/MICSUPP
    // outputs exactly as in the source C table.
];

// C macro translated literally:
// ARIZONA_MIXER_INPUT_ROUTES(name) expands routes from name to Noise/Tone,
// Haptics, AEC, IN1/IN2 PGAs, AIF1/AIF2/AIF3 RX, EQ1/2, DRC1/2, LHPF1-4,
// ASRC1/2, ISRC1/2/3 decimators/interpolators, and DSP2/DSP3 channels.

static cs47l24_dapm_routes: [snd_soc_dapm_route; 0] = [
    // Preserves all explicit C routes and macro route invocations:
    // OUT1L/R to CPVDD and SYSCLK; OUT4L to SPKVDD and SYSCLK; inputs and
    // ASRCs to clocks; MICBIAS to MICVDD; generators to SYSCLK and signal
    // sources; AIF capture/playback endpoints; Voice Control DSP to DSP3;
    // input PGAs to input pins; Audio Trace DSP to DSP2; ARIZONA_MIXER_ROUTES
    // for outputs, PWM, AIF TX, EQ, DRC, LHPF; ARIZONA_MUX_ROUTES for ASRC and
    // ISRC; ARIZONA_DSP_ROUTES for DSP2/DSP3; AEC loopback and headphone/
    // speaker output routes; MICSUPP, DRC signal activity, and DSP Voice
    // Trigger routes.
];

unsafe extern "C" fn cs47l24_set_fll(
    component: *mut snd_soc_component,
    fll_id: c_int,
    source: c_int,
    Fref: c_uint,
    Fout: c_uint,
) -> c_int {
    let cs47l24 = snd_soc_component_get_drvdata(component) as *mut cs47l24_priv;

    match fll_id {
        CS47L24_FLL1 => arizona_set_fll(&mut (*cs47l24).fll[0], source, Fref, Fout),
        CS47L24_FLL2 => arizona_set_fll(&mut (*cs47l24).fll[1], source, Fref, Fout),
        CS47L24_FLL1_REFCLK => arizona_set_fll_refclk(&mut (*cs47l24).fll[0], source, Fref, Fout),
        CS47L24_FLL2_REFCLK => arizona_set_fll_refclk(&mut (*cs47l24).fll[1], source, Fref, Fout),
        _ => -EINVAL,
    }
}

static cs47l24_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    compress_new: Some(snd_soc_new_compress),
};

static mut cs47l24_dai: [snd_soc_dai_driver; 7] = [
    snd_soc_dai_driver {
        name: b"cs47l24-aif1\0".as_ptr() as *const c_char,
        id: 1,
        base: ARIZONA_AIF1_BCLK_CTRL,
        playback: snd_soc_pcm_stream { stream_name: b"AIF1 Playback\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 8, rates: CS47L24_RATES, formats: CS47L24_FORMATS },
        capture: snd_soc_pcm_stream { stream_name: b"AIF1 Capture\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 8, rates: CS47L24_RATES, formats: CS47L24_FORMATS },
        ops: unsafe { &arizona_dai_ops as *const snd_soc_dai_ops },
        symmetric_rate: 1,
        symmetric_sample_bits: 1,
    },
    snd_soc_dai_driver {
        name: b"cs47l24-aif2\0".as_ptr() as *const c_char,
        id: 2,
        base: ARIZONA_AIF2_BCLK_CTRL,
        playback: snd_soc_pcm_stream { stream_name: b"AIF2 Playback\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 6, rates: CS47L24_RATES, formats: CS47L24_FORMATS },
        capture: snd_soc_pcm_stream { stream_name: b"AIF2 Capture\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 6, rates: CS47L24_RATES, formats: CS47L24_FORMATS },
        ops: unsafe { &arizona_dai_ops as *const snd_soc_dai_ops },
        symmetric_rate: 1,
        symmetric_sample_bits: 1,
    },
    snd_soc_dai_driver {
        name: b"cs47l24-aif3\0".as_ptr() as *const c_char,
        id: 3,
        base: ARIZONA_AIF3_BCLK_CTRL,
        playback: snd_soc_pcm_stream { stream_name: b"AIF3 Playback\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 2, rates: CS47L24_RATES, formats: CS47L24_FORMATS },
        capture: snd_soc_pcm_stream { stream_name: b"AIF3 Capture\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 2, rates: CS47L24_RATES, formats: CS47L24_FORMATS },
        ops: unsafe { &arizona_dai_ops as *const snd_soc_dai_ops },
        symmetric_rate: 1,
        symmetric_sample_bits: 1,
    },
    snd_soc_dai_driver {
        name: b"cs47l24-cpu-voicectrl\0".as_ptr() as *const c_char,
        capture: snd_soc_pcm_stream { stream_name: b"Voice Control CPU\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 1, rates: CS47L24_RATES, formats: CS47L24_FORMATS },
        ops: &cs47l24_dai_ops as *const snd_soc_dai_ops,
        ..snd_soc_dai_driver::zeroed()
    },
    snd_soc_dai_driver {
        name: b"cs47l24-dsp-voicectrl\0".as_ptr() as *const c_char,
        capture: snd_soc_pcm_stream { stream_name: b"Voice Control DSP\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 1, rates: CS47L24_RATES, formats: CS47L24_FORMATS },
        ..snd_soc_dai_driver::zeroed()
    },
    snd_soc_dai_driver {
        name: b"cs47l24-cpu-trace\0".as_ptr() as *const c_char,
        capture: snd_soc_pcm_stream { stream_name: b"Audio Trace CPU\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 6, rates: CS47L24_RATES, formats: CS47L24_FORMATS },
        ops: &cs47l24_dai_ops as *const snd_soc_dai_ops,
        ..snd_soc_dai_driver::zeroed()
    },
    snd_soc_dai_driver {
        name: b"cs47l24-dsp-trace\0".as_ptr() as *const c_char,
        capture: snd_soc_pcm_stream { stream_name: b"Audio Trace DSP\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 6, rates: CS47L24_RATES, formats: CS47L24_FORMATS },
        ..snd_soc_dai_driver::zeroed()
    },
];

unsafe extern "C" fn cs47l24_open(
    component: *mut snd_soc_component,
    stream: *mut snd_compr_stream,
) -> c_int {
    let rtd = (*stream).private_data as *mut snd_soc_pcm_runtime;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs47l24_priv;
    let arizona = (*priv_).core.arizona;
    let codec = snd_soc_rtd_to_codec(rtd, 0);
    let n_adsp: c_int;

    if strcmp((*codec).name, b"cs47l24-dsp-voicectrl\0".as_ptr() as *const c_char) == 0 {
        n_adsp = 2;
    } else if strcmp((*codec).name, b"cs47l24-dsp-trace\0".as_ptr() as *const c_char) == 0 {
        n_adsp = 1;
    } else {
        dev_err((*arizona).dev, b"No suitable compressed stream for DAI '%s'\n\0".as_ptr() as *const c_char, (*codec).name);
        return -EINVAL;
    }

    wm_adsp_compr_open(&mut (*priv_).core.adsp[n_adsp as usize], stream)
}

unsafe extern "C" fn cs47l24_adsp2_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    let priv_ = data as *mut cs47l24_priv;
    let arizona = (*priv_).core.arizona;
    let mut info: arizona_voice_trigger_info = core::mem::zeroed();
    let mut serviced: c_int = 0;

    let mut i: c_int = 1;
    while i <= 2 {
        let ret = wm_adsp_compr_handle_irq(&mut (*priv_).core.adsp[i as usize]);
        if ret != -ENODEV {
            serviced += 1;
        }
        if ret == WM_ADSP_COMPR_VOICE_TRIGGER {
            info.core = i;
            arizona_call_notifiers(arizona, ARIZONA_NOTIFY_VOICE_TRIGGER, &mut info as *mut _ as *mut c_void);
        }
        i += 1;
    }

    if serviced == 0 {
        dev_err((*arizona).dev, b"Spurious compressed data IRQ\n\0".as_ptr() as *const c_char);
        return IRQ_NONE;
    }

    IRQ_HANDLED
}

unsafe extern "C" fn cs47l24_component_probe(component: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs47l24_priv;
    let arizona = (*priv_).core.arizona;

    (*arizona).dapm = dapm;
    snd_soc_component_init_regmap(component, (*arizona).regmap);

    let mut ret = arizona_init_spk(component);
    if ret < 0 {
        return ret;
    }

    arizona_init_gpio(component);
    arizona_init_mono(component);

    ret = wm_adsp2_component_probe(&mut (*priv_).core.adsp[1], component);
    if ret != 0 {
        wm_adsp2_component_remove(&mut (*priv_).core.adsp[1], component);
        wm_adsp2_component_remove(&mut (*priv_).core.adsp[2], component);
        return ret;
    }

    ret = wm_adsp2_component_probe(&mut (*priv_).core.adsp[2], component);
    if ret != 0 {
        wm_adsp2_component_remove(&mut (*priv_).core.adsp[1], component);
        wm_adsp2_component_remove(&mut (*priv_).core.adsp[2], component);
        return ret;
    }

    ret = snd_soc_add_component_controls(component, &arizona_adsp2_rate_controls[1], 2);
    if ret != 0 {
        wm_adsp2_component_remove(&mut (*priv_).core.adsp[1], component);
        wm_adsp2_component_remove(&mut (*priv_).core.adsp[2], component);
        return ret;
    }

    snd_soc_dapm_disable_pin(dapm, b"HAPTICS\0".as_ptr() as *const c_char);
    0
}

unsafe extern "C" fn cs47l24_component_remove(component: *mut snd_soc_component) {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs47l24_priv;
    wm_adsp2_component_remove(&mut (*priv_).core.adsp[1], component);
    wm_adsp2_component_remove(&mut (*priv_).core.adsp[2], component);
    (*(*priv_).core.arizona).dapm = ptr::null_mut();
}

static mut cs47l24_digital_vu: [c_uint; 3] = [
    ARIZONA_DAC_DIGITAL_VOLUME_1L,
    ARIZONA_DAC_DIGITAL_VOLUME_1R,
    ARIZONA_DAC_DIGITAL_VOLUME_4L,
];

static cs47l24_compress_ops: snd_compress_ops = snd_compress_ops {
    open: Some(cs47l24_open),
    free: Some(wm_adsp_compr_free),
    set_params: Some(wm_adsp_compr_set_params),
    get_caps: Some(wm_adsp_compr_get_caps),
    trigger: Some(wm_adsp_compr_trigger),
    pointer: Some(wm_adsp_compr_pointer),
    copy: Some(wm_adsp_compr_copy),
};

static soc_component_dev_cs47l24: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(cs47l24_component_probe),
    remove: Some(cs47l24_component_remove),
    set_sysclk: Some(arizona_set_sysclk),
    set_pll: Some(cs47l24_set_fll),
    name: DRV_NAME,
    compress_ops: &cs47l24_compress_ops as *const snd_compress_ops,
    controls: cs47l24_snd_controls.as_ptr(),
    num_controls: cs47l24_snd_controls.len() as c_uint,
    dapm_widgets: cs47l24_dapm_widgets.as_ptr(),
    num_dapm_widgets: cs47l24_dapm_widgets.len() as c_uint,
    dapm_routes: cs47l24_dapm_routes.as_ptr(),
    num_dapm_routes: cs47l24_dapm_routes.len() as c_uint,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn cs47l24_probe(pdev: *mut platform_device) -> c_int {
    let arizona = dev_get_drvdata((*(*pdev).dev.parent).parent) as *mut arizona;
    let mut ret: c_int;

    BUILD_BUG_ON!(cs47l24_dai.len() > ARIZONA_MAX_DAI as usize);

    let cs47l24 = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<cs47l24_priv>(),
        GFP_KERNEL,
    ) as *mut cs47l24_priv;
    if cs47l24.is_null() {
        return -ENOMEM;
    }

    if IS_ENABLED!(CONFIG_OF) {
        if dev_get_platdata((*arizona).dev).is_null() {
            ret = arizona_of_get_audio_pdata(arizona);
            if ret < 0 {
                return ret;
            }
        }
    }

    platform_set_drvdata(pdev, cs47l24 as *mut c_void);
    (*cs47l24).core.arizona = arizona;
    (*cs47l24).core.num_inputs = 4;

    let mut i: c_int = 1;
    while i <= 2 {
        (*cs47l24).core.adsp[i as usize].part = b"cs47l24\0".as_ptr() as *const c_char;
        (*cs47l24).core.adsp[i as usize].cs_dsp.num = i + 1;
        (*cs47l24).core.adsp[i as usize].cs_dsp.type_ = WMFW_ADSP2;
        (*cs47l24).core.adsp[i as usize].cs_dsp.dev = (*arizona).dev;
        (*cs47l24).core.adsp[i as usize].cs_dsp.regmap = (*arizona).regmap;
        (*cs47l24).core.adsp[i as usize].cs_dsp.base = ARIZONA_DSP1_CONTROL_1 + (0x100 * i as c_uint);
        (*cs47l24).core.adsp[i as usize].cs_dsp.mem = cs47l24_dsp_regions[(i - 1) as usize];
        (*cs47l24).core.adsp[i as usize].cs_dsp.num_mems = cs47l24_dsp2_regions.len() as c_uint;

        ret = wm_adsp2_init(&mut (*cs47l24).core.adsp[i as usize]);
        if ret != 0 {
            return ret;
        }
        i += 1;
    }

    i = 0;
    while (i as usize) < (*cs47l24).fll.len() {
        (*cs47l24).fll[i as usize].vco_mult = 3;
        i += 1;
    }

    arizona_init_fll(arizona, 1, ARIZONA_FLL1_CONTROL_1 - 1, ARIZONA_IRQ_FLL1_LOCK, ARIZONA_IRQ_FLL1_CLOCK_OK, &mut (*cs47l24).fll[0]);
    arizona_init_fll(arizona, 2, ARIZONA_FLL2_CONTROL_1 - 1, ARIZONA_IRQ_FLL2_LOCK, ARIZONA_IRQ_FLL2_CLOCK_OK, &mut (*cs47l24).fll[1]);

    /* SR2 fixed at 8kHz, SR3 fixed at 16kHz */
    regmap_update_bits((*arizona).regmap, ARIZONA_SAMPLE_RATE_2, ARIZONA_SAMPLE_RATE_2_MASK, 0x11);
    regmap_update_bits((*arizona).regmap, ARIZONA_SAMPLE_RATE_3, ARIZONA_SAMPLE_RATE_3_MASK, 0x12);

    i = 0;
    while (i as usize) < cs47l24_dai.len() {
        arizona_init_dai(&mut (*cs47l24).core, i);
        i += 1;
    }

    /* Latch volume update bits */
    i = 0;
    while (i as usize) < cs47l24_digital_vu.len() {
        regmap_update_bits((*arizona).regmap, cs47l24_digital_vu[i as usize], CS47L24_DIG_VU, CS47L24_DIG_VU);
        i += 1;
    }

    pm_runtime_enable(&mut (*pdev).dev);
    pm_runtime_idle(&mut (*pdev).dev);

    ret = arizona_request_irq(arizona, ARIZONA_IRQ_DSP_IRQ1, b"ADSP2 Compressed IRQ\0".as_ptr() as *const c_char, Some(cs47l24_adsp2_irq), cs47l24 as *mut c_void);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, b"Failed to request DSP IRQ: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = arizona_set_irq_wake(arizona, ARIZONA_IRQ_DSP_IRQ1, 1);
    if ret != 0 {
        dev_warn(&mut (*pdev).dev, b"Failed to set compressed IRQ as a wake source: %d\n\0".as_ptr() as *const c_char, ret);
    }

    arizona_init_common(arizona);

    ret = arizona_init_vol_limit(arizona);
    if ret < 0 {
        arizona_set_irq_wake(arizona, ARIZONA_IRQ_DSP_IRQ1, 0);
        arizona_free_irq(arizona, ARIZONA_IRQ_DSP_IRQ1, cs47l24 as *mut c_void);
        return ret;
    }

    ret = arizona_init_spk_irqs(arizona);
    if ret < 0 {
        arizona_set_irq_wake(arizona, ARIZONA_IRQ_DSP_IRQ1, 0);
        arizona_free_irq(arizona, ARIZONA_IRQ_DSP_IRQ1, cs47l24 as *mut c_void);
        return ret;
    }

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &soc_component_dev_cs47l24,
        cs47l24_dai.as_mut_ptr(),
        cs47l24_dai.len() as c_int,
    );
    if ret < 0 {
        dev_err(&mut (*pdev).dev, b"Failed to register component: %d\n\0".as_ptr() as *const c_char, ret);
        arizona_free_spk_irqs(arizona);
        arizona_set_irq_wake(arizona, ARIZONA_IRQ_DSP_IRQ1, 0);
        arizona_free_irq(arizona, ARIZONA_IRQ_DSP_IRQ1, cs47l24 as *mut c_void);
        return ret;
    }

    ret
}

unsafe extern "C" fn cs47l24_remove(pdev: *mut platform_device) {
    let cs47l24 = platform_get_drvdata(pdev) as *mut cs47l24_priv;
    let arizona = (*cs47l24).core.arizona;

    pm_runtime_disable(&mut (*pdev).dev);
    wm_adsp2_remove(&mut (*cs47l24).core.adsp[1]);
    wm_adsp2_remove(&mut (*cs47l24).core.adsp[2]);
    arizona_free_spk_irqs(arizona);
    arizona_set_irq_wake(arizona, ARIZONA_IRQ_DSP_IRQ1, 0);
    arizona_free_irq(arizona, ARIZONA_IRQ_DSP_IRQ1, cs47l24 as *mut c_void);
}

static mut cs47l24_codec_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"cs47l24-codec\0".as_ptr() as *const c_char,
    },
    probe: Some(cs47l24_probe),
    remove: Some(cs47l24_remove),
};

module_platform_driver!(cs47l24_codec_driver);
module_description!("ASoC CS47L24 driver");
module_author!("Richard Fitzgerald <rf@opensource.wolfsonmicro.com>");
module_license!("GPL v2");
module_alias!("platform:cs47l24-codec");

// External declarations supplied by translated kernel, ASoC, Arizona, WM ADSP,
// and CS47L24 headers in the final repository context.
extern "C" {
    static arizona_dai_ops: snd_soc_dai_ops;
    static arizona_adsp2_rate_controls: [snd_kcontrol_new; 3];

    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_init_regmap(component: *mut snd_soc_component, regmap: *mut regmap);
    fn snd_soc_add_component_controls(component: *mut snd_soc_component, controls: *const snd_kcontrol_new, num: c_uint) -> c_int;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_new_compress() -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn devm_snd_soc_register_component(dev: *mut device, component: *const snd_soc_component_driver, dai: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn regmap_read(regmap: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(regmap: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn pm_runtime_enable(dev: *mut device) -> c_int;
    fn pm_runtime_idle(dev: *mut device) -> c_int;
    fn pm_runtime_disable(dev: *mut device);

    fn wm_adsp2_set_dspclk(w: *mut snd_soc_dapm_widget, v: c_uint);
    fn wm_adsp_early_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int;
    fn wm_adsp_compr_open(adsp: *mut wm_adsp, stream: *mut snd_compr_stream) -> c_int;
    fn wm_adsp_compr_free(stream: *mut snd_compr_stream) -> c_int;
    fn wm_adsp_compr_set_params(stream: *mut snd_compr_stream, params: *mut c_void) -> c_int;
    fn wm_adsp_compr_get_caps(stream: *mut snd_compr_stream, caps: *mut c_void) -> c_int;
    fn wm_adsp_compr_trigger(stream: *mut snd_compr_stream, cmd: c_int) -> c_int;
    fn wm_adsp_compr_pointer(stream: *mut snd_compr_stream, tstamp: *mut c_void) -> c_int;
    fn wm_adsp_compr_copy(stream: *mut snd_compr_stream, buf: *mut c_char, count: usize) -> c_int;
    fn wm_adsp_compr_handle_irq(adsp: *mut wm_adsp) -> c_int;
    fn wm_adsp2_component_probe(adsp: *mut wm_adsp, component: *mut snd_soc_component) -> c_int;
    fn wm_adsp2_component_remove(adsp: *mut wm_adsp, component: *mut snd_soc_component);
    fn wm_adsp2_init(adsp: *mut wm_adsp) -> c_int;
    fn wm_adsp2_remove(adsp: *mut wm_adsp);

    fn arizona_set_fll(fll: *mut arizona_fll, source: c_int, Fref: c_uint, Fout: c_uint) -> c_int;
    fn arizona_set_fll_refclk(fll: *mut arizona_fll, source: c_int, Fref: c_uint, Fout: c_uint) -> c_int;
    fn arizona_set_sysclk(component: *mut snd_soc_component, clk_id: c_int, source: c_int, freq: c_uint, dir: c_int) -> c_int;
    fn arizona_init_spk(component: *mut snd_soc_component) -> c_int;
    fn arizona_init_gpio(component: *mut snd_soc_component);
    fn arizona_init_mono(component: *mut snd_soc_component);
    fn arizona_call_notifiers(arizona: *mut arizona, event: c_uint, data: *mut c_void) -> c_int;
    fn arizona_init_fll(arizona: *mut arizona, id: c_int, base: c_uint, lock_irq: c_uint, clock_ok_irq: c_uint, fll: *mut arizona_fll);
    fn arizona_init_dai(core: *mut arizona_priv, dai: c_int);
    fn arizona_request_irq(arizona: *mut arizona, irq: c_uint, name: *const c_char, handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, data: *mut c_void) -> c_int;
    fn arizona_set_irq_wake(arizona: *mut arizona, irq: c_uint, on: c_int) -> c_int;
    fn arizona_free_irq(arizona: *mut arizona, irq: c_uint, data: *mut c_void);
    fn arizona_init_common(arizona: *mut arizona);
    fn arizona_init_vol_limit(arizona: *mut arizona) -> c_int;
    fn arizona_init_spk_irqs(arizona: *mut arizona) -> c_int;
    fn arizona_free_spk_irqs(arizona: *mut arizona);
    fn arizona_of_get_audio_pdata(arizona: *mut arizona) -> c_int;
}


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
