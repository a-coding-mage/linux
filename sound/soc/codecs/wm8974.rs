// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8974.c  --  WM8974 ALSA Soc Audio driver
 *
 * Copyright 2006-2009 Wolfson Microelectronics PLC.
 *
 * Author: Liam Girdwood <Liam.Girdwood@wolfsonmicro.com>
 */

// C includes translated as external dependency intent:
// linux/module.h, linux/kernel.h, linux/init.h, linux/delay.h, linux/pm.h,
// linux/i2c.h, linux/regmap.h, linux/slab.h, sound/core.h, sound/pcm.h,
// sound/pcm_params.h, sound/soc.h, sound/initval.h, sound/tlv.h, wm8974.h.

#[repr(C)]
pub struct wm8974_priv {
    pub mclk: core::ffi::c_uint,
    pub fs: core::ffi::c_uint,
}

static wm8974_reg_defaults: [reg_default; 57] = [
    reg_default { reg: 0, def: 0x0000 }, reg_default { reg: 1, def: 0x0000 }, reg_default { reg: 2, def: 0x0000 }, reg_default { reg: 3, def: 0x0000 },
    reg_default { reg: 4, def: 0x0050 }, reg_default { reg: 5, def: 0x0000 }, reg_default { reg: 6, def: 0x0140 }, reg_default { reg: 7, def: 0x0000 },
    reg_default { reg: 8, def: 0x0000 }, reg_default { reg: 9, def: 0x0000 }, reg_default { reg: 10, def: 0x0000 }, reg_default { reg: 11, def: 0x00ff },
    reg_default { reg: 12, def: 0x0000 }, reg_default { reg: 13, def: 0x0000 }, reg_default { reg: 14, def: 0x0100 }, reg_default { reg: 15, def: 0x00ff },
    reg_default { reg: 16, def: 0x0000 }, reg_default { reg: 17, def: 0x0000 }, reg_default { reg: 18, def: 0x012c }, reg_default { reg: 19, def: 0x002c },
    reg_default { reg: 20, def: 0x002c }, reg_default { reg: 21, def: 0x002c }, reg_default { reg: 22, def: 0x002c }, reg_default { reg: 23, def: 0x0000 },
    reg_default { reg: 24, def: 0x0032 }, reg_default { reg: 25, def: 0x0000 }, reg_default { reg: 26, def: 0x0000 }, reg_default { reg: 27, def: 0x0000 },
    reg_default { reg: 28, def: 0x0000 }, reg_default { reg: 29, def: 0x0000 }, reg_default { reg: 30, def: 0x0000 }, reg_default { reg: 31, def: 0x0000 },
    reg_default { reg: 32, def: 0x0038 }, reg_default { reg: 33, def: 0x000b }, reg_default { reg: 34, def: 0x0032 }, reg_default { reg: 35, def: 0x0000 },
    reg_default { reg: 36, def: 0x0008 }, reg_default { reg: 37, def: 0x000c }, reg_default { reg: 38, def: 0x0093 }, reg_default { reg: 39, def: 0x00e9 },
    reg_default { reg: 40, def: 0x0000 }, reg_default { reg: 41, def: 0x0000 }, reg_default { reg: 42, def: 0x0000 }, reg_default { reg: 43, def: 0x0000 },
    reg_default { reg: 44, def: 0x0003 }, reg_default { reg: 45, def: 0x0010 }, reg_default { reg: 46, def: 0x0000 }, reg_default { reg: 47, def: 0x0000 },
    reg_default { reg: 48, def: 0x0000 }, reg_default { reg: 49, def: 0x0002 }, reg_default { reg: 50, def: 0x0000 }, reg_default { reg: 51, def: 0x0000 },
    reg_default { reg: 52, def: 0x0000 }, reg_default { reg: 53, def: 0x0000 }, reg_default { reg: 54, def: 0x0039 }, reg_default { reg: 55, def: 0x0000 },
    reg_default { reg: 56, def: 0x0000 },
];

const WM8974_POWER1_BIASEN: u16 = 0x08;
const WM8974_POWER1_BUFIOEN: u16 = 0x04;

unsafe fn wm8974_reset(c: *mut snd_soc_component) -> core::ffi::c_int {
    snd_soc_component_write(c, WM8974_RESET, 0)
}

static wm8974_companding: [&str; 4] = ["Off", "NC", "u-law", "A-law"];
static wm8974_deemp: [&str; 4] = ["None", "32kHz", "44.1kHz", "48kHz"];
static wm8974_eqmode: [&str; 2] = ["Capture", "Playback"];
static wm8974_bw: [&str; 2] = ["Narrow", "Wide"];
static wm8974_eq1: [&str; 4] = ["80Hz", "105Hz", "135Hz", "175Hz"];
static wm8974_eq2: [&str; 4] = ["230Hz", "300Hz", "385Hz", "500Hz"];
static wm8974_eq3: [&str; 4] = ["650Hz", "850Hz", "1.1kHz", "1.4kHz"];
static wm8974_eq4: [&str; 4] = ["1.8kHz", "2.4kHz", "3.2kHz", "4.1kHz"];
static wm8974_eq5: [&str; 4] = ["5.3kHz", "6.9kHz", "9kHz", "11.7kHz"];
static wm8974_alc: [&str; 2] = ["ALC", "Limiter"];

static wm8974_enum: [soc_enum; 14] = [
    SOC_ENUM_SINGLE!(WM8974_COMP, 1, 4, wm8974_companding), /* adc */
    SOC_ENUM_SINGLE!(WM8974_COMP, 3, 4, wm8974_companding), /* dac */
    SOC_ENUM_SINGLE!(WM8974_DAC, 4, 4, wm8974_deemp),
    SOC_ENUM_SINGLE!(WM8974_EQ1, 8, 2, wm8974_eqmode),
    SOC_ENUM_SINGLE!(WM8974_EQ1, 5, 4, wm8974_eq1),
    SOC_ENUM_SINGLE!(WM8974_EQ2, 8, 2, wm8974_bw),
    SOC_ENUM_SINGLE!(WM8974_EQ2, 5, 4, wm8974_eq2),
    SOC_ENUM_SINGLE!(WM8974_EQ3, 8, 2, wm8974_bw),
    SOC_ENUM_SINGLE!(WM8974_EQ3, 5, 4, wm8974_eq3),
    SOC_ENUM_SINGLE!(WM8974_EQ4, 8, 2, wm8974_bw),
    SOC_ENUM_SINGLE!(WM8974_EQ4, 5, 4, wm8974_eq4),
    SOC_ENUM_SINGLE!(WM8974_EQ5, 8, 2, wm8974_bw),
    SOC_ENUM_SINGLE!(WM8974_EQ5, 5, 4, wm8974_eq5),
    SOC_ENUM_SINGLE!(WM8974_ALC3, 8, 2, wm8974_alc),
];

static wm8974_auxmode_text: [&str; 2] = ["Buffer", "Mixer"];
static wm8974_auxmode: soc_enum = SOC_ENUM_SINGLE!(WM8974_INPUT, 3, wm8974_auxmode_text);

static digital_tlv: [core::ffi::c_uint; 0] = DECLARE_TLV_DB_SCALE!(digital_tlv, -12750, 50, 1);
static eq_tlv: [core::ffi::c_uint; 0] = DECLARE_TLV_DB_SCALE!(eq_tlv, -1200, 100, 0);
static inpga_tlv: [core::ffi::c_uint; 0] = DECLARE_TLV_DB_SCALE!(inpga_tlv, -1200, 75, 0);
static spk_tlv: [core::ffi::c_uint; 0] = DECLARE_TLV_DB_SCALE!(spk_tlv, -5700, 100, 0);

static wm8974_snd_controls: [snd_kcontrol_new; 48] = [
    SOC_SINGLE!("Digital Loopback Switch", WM8974_COMP, 0, 1, 0),
    SOC_ENUM!("DAC Companding", wm8974_enum[1]),
    SOC_ENUM!("ADC Companding", wm8974_enum[0]),
    SOC_ENUM!("Playback De-emphasis", wm8974_enum[2]),
    SOC_SINGLE!("DAC Inversion Switch", WM8974_DAC, 0, 1, 0),
    SOC_SINGLE_TLV!("PCM Volume", WM8974_DACVOL, 0, 255, 0, digital_tlv),
    SOC_SINGLE!("High Pass Filter Switch", WM8974_ADC, 8, 1, 0),
    SOC_SINGLE!("High Pass Cut Off", WM8974_ADC, 4, 7, 0),
    SOC_SINGLE!("ADC Inversion Switch", WM8974_ADC, 0, 1, 0),
    SOC_SINGLE_TLV!("Capture Volume", WM8974_ADCVOL, 0, 255, 0, digital_tlv),
    SOC_ENUM!("Equaliser Function", wm8974_enum[3]),
    SOC_ENUM!("EQ1 Cut Off", wm8974_enum[4]),
    SOC_SINGLE_TLV!("EQ1 Volume", WM8974_EQ1, 0, 24, 1, eq_tlv),
    SOC_ENUM!("Equaliser EQ2 Bandwidth", wm8974_enum[5]),
    SOC_ENUM!("EQ2 Cut Off", wm8974_enum[6]),
    SOC_SINGLE_TLV!("EQ2 Volume", WM8974_EQ2, 0, 24, 1, eq_tlv),
    SOC_ENUM!("Equaliser EQ3 Bandwidth", wm8974_enum[7]),
    SOC_ENUM!("EQ3 Cut Off", wm8974_enum[8]),
    SOC_SINGLE_TLV!("EQ3 Volume", WM8974_EQ3, 0, 24, 1, eq_tlv),
    SOC_ENUM!("Equaliser EQ4 Bandwidth", wm8974_enum[9]),
    SOC_ENUM!("EQ4 Cut Off", wm8974_enum[10]),
    SOC_SINGLE_TLV!("EQ4 Volume", WM8974_EQ4, 0, 24, 1, eq_tlv),
    SOC_ENUM!("Equaliser EQ5 Bandwidth", wm8974_enum[11]),
    SOC_ENUM!("EQ5 Cut Off", wm8974_enum[12]),
    SOC_SINGLE_TLV!("EQ5 Volume", WM8974_EQ5, 0, 24, 1, eq_tlv),
    SOC_SINGLE!("DAC Playback Limiter Switch", WM8974_DACLIM1, 8, 1, 0),
    SOC_SINGLE!("DAC Playback Limiter Decay", WM8974_DACLIM1, 4, 15, 0),
    SOC_SINGLE!("DAC Playback Limiter Attack", WM8974_DACLIM1, 0, 15, 0),
    SOC_SINGLE!("DAC Playback Limiter Threshold", WM8974_DACLIM2, 4, 7, 0),
    SOC_SINGLE!("DAC Playback Limiter Boost", WM8974_DACLIM2, 0, 15, 0),
    SOC_SINGLE!("ALC Enable Switch", WM8974_ALC1, 8, 1, 0),
    SOC_SINGLE!("ALC Capture Max Gain", WM8974_ALC1, 3, 7, 0),
    SOC_SINGLE!("ALC Capture Min Gain", WM8974_ALC1, 0, 7, 0),
    SOC_SINGLE!("ALC Capture ZC Switch", WM8974_ALC2, 8, 1, 0),
    SOC_SINGLE!("ALC Capture Hold", WM8974_ALC2, 4, 7, 0),
    SOC_SINGLE!("ALC Capture Target", WM8974_ALC2, 0, 15, 0),
    SOC_ENUM!("ALC Capture Mode", wm8974_enum[13]),
    SOC_SINGLE!("ALC Capture Decay", WM8974_ALC3, 4, 15, 0),
    SOC_SINGLE!("ALC Capture Attack", WM8974_ALC3, 0, 15, 0),
    SOC_SINGLE!("ALC Capture Noise Gate Switch", WM8974_NGATE, 3, 1, 0),
    SOC_SINGLE!("ALC Capture Noise Gate Threshold", WM8974_NGATE, 0, 7, 0),
    SOC_SINGLE!("Capture PGA ZC Switch", WM8974_INPPGA, 7, 1, 0),
    SOC_SINGLE_TLV!("Capture PGA Volume", WM8974_INPPGA, 0, 63, 0, inpga_tlv),
    SOC_SINGLE!("Speaker Playback ZC Switch", WM8974_SPKVOL, 7, 1, 0),
    SOC_SINGLE!("Speaker Playback Switch", WM8974_SPKVOL, 6, 1, 1),
    SOC_SINGLE_TLV!("Speaker Playback Volume", WM8974_SPKVOL, 0, 63, 0, spk_tlv),
    SOC_ENUM!("Aux Mode", wm8974_auxmode),
    SOC_SINGLE!("Capture Boost(+20dB)", WM8974_ADCBOOST, 8, 1, 0),
    SOC_SINGLE!("Mono Playback Switch", WM8974_MONOMIX, 6, 1, 1),
    /* DAC / ADC oversampling */
    SOC_SINGLE!("DAC 128x Oversampling Switch", WM8974_DAC, 8, 1, 0),
    SOC_SINGLE!("ADC 128x Oversampling Switch", WM8974_ADC, 8, 1, 0),
];

/* Speaker Output Mixer */
static wm8974_speaker_mixer_controls: [snd_kcontrol_new; 3] = [
    SOC_DAPM_SINGLE!("Line Bypass Switch", WM8974_SPKMIX, 1, 1, 0),
    SOC_DAPM_SINGLE!("Aux Playback Switch", WM8974_SPKMIX, 5, 1, 0),
    SOC_DAPM_SINGLE!("PCM Playback Switch", WM8974_SPKMIX, 0, 1, 0),
];

/* Mono Output Mixer */
static wm8974_mono_mixer_controls: [snd_kcontrol_new; 3] = [
    SOC_DAPM_SINGLE!("Line Bypass Switch", WM8974_MONOMIX, 1, 1, 0),
    SOC_DAPM_SINGLE!("Aux Playback Switch", WM8974_MONOMIX, 2, 1, 0),
    SOC_DAPM_SINGLE!("PCM Playback Switch", WM8974_MONOMIX, 0, 1, 0),
];

/* Boost mixer */
static wm8974_boost_mixer: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE!("PGA Switch", WM8974_INPPGA, 6, 1, 1),
];

/* Input PGA */
static wm8974_inpga: [snd_kcontrol_new; 3] = [
    SOC_DAPM_SINGLE!("Aux Switch", WM8974_INPUT, 2, 1, 0),
    SOC_DAPM_SINGLE!("MicN Switch", WM8974_INPUT, 1, 1, 0),
    SOC_DAPM_SINGLE!("MicP Switch", WM8974_INPUT, 0, 1, 0),
];

static wm8974_dapm_widgets: [snd_soc_dapm_widget; 18] = [
    SND_SOC_DAPM_MIXER!("Speaker Mixer", WM8974_POWER3, 2, 0, &wm8974_speaker_mixer_controls[0], ARRAY_SIZE!(wm8974_speaker_mixer_controls)),
    SND_SOC_DAPM_MIXER!("Mono Mixer", WM8974_POWER3, 3, 0, &wm8974_mono_mixer_controls[0], ARRAY_SIZE!(wm8974_mono_mixer_controls)),
    SND_SOC_DAPM_DAC!("DAC", "HiFi Playback", WM8974_POWER3, 0, 0),
    SND_SOC_DAPM_ADC!("ADC", "HiFi Capture", WM8974_POWER2, 0, 0),
    SND_SOC_DAPM_PGA!("Aux Input", WM8974_POWER1, 6, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("SpkN Out", WM8974_POWER3, 5, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("SpkP Out", WM8974_POWER3, 6, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Mono Out", WM8974_POWER3, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("Input PGA", WM8974_POWER2, 2, 0, wm8974_inpga, ARRAY_SIZE!(wm8974_inpga)),
    SND_SOC_DAPM_MIXER!("Boost Mixer", WM8974_POWER2, 4, 0, wm8974_boost_mixer, ARRAY_SIZE!(wm8974_boost_mixer)),
    SND_SOC_DAPM_SUPPLY!("Mic Bias", WM8974_POWER1, 4, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_INPUT!("MICN"),
    SND_SOC_DAPM_INPUT!("MICP"),
    SND_SOC_DAPM_INPUT!("AUX"),
    SND_SOC_DAPM_OUTPUT!("MONOOUT"),
    SND_SOC_DAPM_OUTPUT!("SPKOUTP"),
    SND_SOC_DAPM_OUTPUT!("SPKOUTN"),
];

static wm8974_dapm_routes: [snd_soc_dapm_route; 17] = [
    /* Mono output mixer */
    snd_soc_dapm_route { sink: "Mono Mixer", control: "PCM Playback Switch", source: "DAC" },
    snd_soc_dapm_route { sink: "Mono Mixer", control: "Aux Playback Switch", source: "Aux Input" },
    snd_soc_dapm_route { sink: "Mono Mixer", control: "Line Bypass Switch", source: "Boost Mixer" },
    /* Speaker output mixer */
    snd_soc_dapm_route { sink: "Speaker Mixer", control: "PCM Playback Switch", source: "DAC" },
    snd_soc_dapm_route { sink: "Speaker Mixer", control: "Aux Playback Switch", source: "Aux Input" },
    snd_soc_dapm_route { sink: "Speaker Mixer", control: "Line Bypass Switch", source: "Boost Mixer" },
    /* Outputs */
    snd_soc_dapm_route { sink: "Mono Out", control: core::ptr::null(), source: "Mono Mixer" },
    snd_soc_dapm_route { sink: "MONOOUT", control: core::ptr::null(), source: "Mono Out" },
    snd_soc_dapm_route { sink: "SpkN Out", control: core::ptr::null(), source: "Speaker Mixer" },
    snd_soc_dapm_route { sink: "SpkP Out", control: core::ptr::null(), source: "Speaker Mixer" },
    snd_soc_dapm_route { sink: "SPKOUTN", control: core::ptr::null(), source: "SpkN Out" },
    snd_soc_dapm_route { sink: "SPKOUTP", control: core::ptr::null(), source: "SpkP Out" },
    /* Boost Mixer */
    snd_soc_dapm_route { sink: "ADC", control: core::ptr::null(), source: "Boost Mixer" },
    snd_soc_dapm_route { sink: "Boost Mixer", control: core::ptr::null(), source: "Aux Input" },
    snd_soc_dapm_route { sink: "Boost Mixer", control: "PGA Switch", source: "Input PGA" },
    snd_soc_dapm_route { sink: "Boost Mixer", control: core::ptr::null(), source: "MICP" },
    /* Input PGA */
    snd_soc_dapm_route { sink: "Input PGA", control: "Aux Switch", source: "Aux Input" },
    snd_soc_dapm_route { sink: "Input PGA", control: "MicN Switch", source: "MICN" },
    snd_soc_dapm_route { sink: "Input PGA", control: "MicP Switch", source: "MICP" },
    /* Inputs */
    snd_soc_dapm_route { sink: "Aux Input", control: core::ptr::null(), source: "AUX" },
];

#[repr(C)]
pub struct pll_ {
    pub pre_div: core::ffi::c_uint,
    pub n: core::ffi::c_uint,
    pub k: core::ffi::c_uint,
}

/* The size in bits of the pll divide multiplied by 10
 * to allow rounding later */
const FIXED_PLL_SIZE: u64 = ((1u64 << 24) * 10);

unsafe fn pll_factors(pll_div: *mut pll_, mut target: core::ffi::c_uint, mut source: core::ffi::c_uint) {
    let mut Kpart: core::ffi::c_ulonglong;
    let mut K: core::ffi::c_uint;
    let mut Ndiv: core::ffi::c_uint;
    let Nmod: core::ffi::c_uint;

    /* There is a fixed divide by 4 in the output path */
    target = target.wrapping_mul(4);

    Ndiv = target / source;
    if Ndiv < 6 {
        source /= 2;
        (*pll_div).pre_div = 1;
        Ndiv = target / source;
    } else {
        (*pll_div).pre_div = 0;
    }

    if (Ndiv < 6) || (Ndiv > 12) {
        printk(KERN_WARNING, "WM8974 N value %u outwith recommended range!\n", Ndiv);
    }

    (*pll_div).n = Ndiv;
    Nmod = target % source;
    Kpart = FIXED_PLL_SIZE.wrapping_mul(Nmod as u64);
    Kpart /= source as u64;

    K = (Kpart & 0xFFFFFFFF) as core::ffi::c_uint;

    /* Check if we need to round */
    if (K % 10) >= 5 {
        K = K.wrapping_add(5);
    }

    /* Move down to proper range now rounding is done */
    K /= 10;

    (*pll_div).k = K;
}

unsafe fn wm8974_set_dai_pll(
    codec_dai: *mut snd_soc_dai,
    _pll_id: core::ffi::c_int,
    _source: core::ffi::c_int,
    freq_in: core::ffi::c_uint,
    freq_out: core::ffi::c_uint,
) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let mut pll_div: pll_ = core::mem::zeroed();
    let mut reg: u16;

    if freq_in == 0 || freq_out == 0 {
        /* Clock CODEC directly from MCLK */
        reg = snd_soc_component_read(component, WM8974_CLOCK) as u16;
        snd_soc_component_write(component, WM8974_CLOCK, (reg & 0x0ff) as core::ffi::c_uint);

        /* Turn off PLL */
        reg = snd_soc_component_read(component, WM8974_POWER1) as u16;
        snd_soc_component_write(component, WM8974_POWER1, (reg & 0x1df) as core::ffi::c_uint);
        return 0;
    }

    pll_factors(&mut pll_div, freq_out, freq_in);

    snd_soc_component_write(component, WM8974_PLLN, (pll_div.pre_div << 4) | pll_div.n);
    snd_soc_component_write(component, WM8974_PLLK1, pll_div.k >> 18);
    snd_soc_component_write(component, WM8974_PLLK2, (pll_div.k >> 9) & 0x1ff);
    snd_soc_component_write(component, WM8974_PLLK3, pll_div.k & 0x1ff);
    reg = snd_soc_component_read(component, WM8974_POWER1) as u16;
    snd_soc_component_write(component, WM8974_POWER1, (reg | 0x020) as core::ffi::c_uint);

    /* Run CODEC from PLL instead of MCLK */
    reg = snd_soc_component_read(component, WM8974_CLOCK) as u16;
    snd_soc_component_write(component, WM8974_CLOCK, (reg | 0x100) as core::ffi::c_uint);

    0
}

/*
 * Configure WM8974 clock dividers.
 */
unsafe fn wm8974_set_dai_clkdiv(
    codec_dai: *mut snd_soc_dai,
    div_id: core::ffi::c_int,
    div: core::ffi::c_int,
) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let mut reg: u16;

    match div_id {
        WM8974_OPCLKDIV => {
            reg = (snd_soc_component_read(component, WM8974_GPIO) & 0x1cf) as u16;
            snd_soc_component_write(component, WM8974_GPIO, (reg as core::ffi::c_int | div) as core::ffi::c_uint);
        }
        WM8974_MCLKDIV => {
            reg = (snd_soc_component_read(component, WM8974_CLOCK) & 0x11f) as u16;
            snd_soc_component_write(component, WM8974_CLOCK, (reg as core::ffi::c_int | div) as core::ffi::c_uint);
        }
        WM8974_BCLKDIV => {
            reg = (snd_soc_component_read(component, WM8974_CLOCK) & 0x1e3) as u16;
            snd_soc_component_write(component, WM8974_CLOCK, (reg as core::ffi::c_int | div) as core::ffi::c_uint);
        }
        _ => return -EINVAL,
    }

    0
}

unsafe fn wm8974_get_mclkdiv(
    f_in: core::ffi::c_uint,
    f_out: core::ffi::c_uint,
    mclkdiv: *mut core::ffi::c_int,
) -> core::ffi::c_uint {
    let mut ratio: core::ffi::c_uint = 2u32.wrapping_mul(f_in) / f_out;

    if ratio <= 2 {
        *mclkdiv = WM8974_MCLKDIV_1;
        ratio = 2;
    } else if ratio == 3 {
        *mclkdiv = WM8974_MCLKDIV_1_5;
    } else if ratio == 4 {
        *mclkdiv = WM8974_MCLKDIV_2;
    } else if ratio <= 6 {
        *mclkdiv = WM8974_MCLKDIV_3;
        ratio = 6;
    } else if ratio <= 8 {
        *mclkdiv = WM8974_MCLKDIV_4;
        ratio = 8;
    } else if ratio <= 12 {
        *mclkdiv = WM8974_MCLKDIV_6;
        ratio = 12;
    } else if ratio <= 16 {
        *mclkdiv = WM8974_MCLKDIV_8;
        ratio = 16;
    } else {
        *mclkdiv = WM8974_MCLKDIV_12;
        ratio = 24;
    }

    f_out.wrapping_mul(ratio) / 2
}

unsafe fn wm8974_update_clocks(dai: *mut snd_soc_dai) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let priv_: *mut wm8974_priv = snd_soc_component_get_drvdata(component) as *mut wm8974_priv;
    let fs256: core::ffi::c_uint;
    let mut fpll: core::ffi::c_uint = 0;
    let mut f: core::ffi::c_uint;
    let mut mclkdiv: core::ffi::c_int = 0;

    if (*priv_).mclk == 0 || (*priv_).fs == 0 {
        return 0;
    }

    fs256 = 256u32.wrapping_mul((*priv_).fs);

    f = wm8974_get_mclkdiv((*priv_).mclk, fs256, &mut mclkdiv);
    if f != (*priv_).mclk {
        /* The PLL performs best around 90MHz */
        if fs256 % 8000 != 0 {
            f = 22579200;
        } else {
            f = 24576000;
        }

        fpll = wm8974_get_mclkdiv(f, fs256, &mut mclkdiv);
    }

    wm8974_set_dai_pll(dai, 0, 0, (*priv_).mclk, fpll);
    wm8974_set_dai_clkdiv(dai, WM8974_MCLKDIV, mclkdiv);

    0
}

unsafe fn wm8974_set_dai_sysclk(
    dai: *mut snd_soc_dai,
    _clk_id: core::ffi::c_int,
    freq: core::ffi::c_uint,
    dir: core::ffi::c_int,
) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let priv_: *mut wm8974_priv = snd_soc_component_get_drvdata(component) as *mut wm8974_priv;

    if dir != SND_SOC_CLOCK_IN {
        return -EINVAL;
    }

    (*priv_).mclk = freq;

    wm8974_update_clocks(dai)
}

unsafe fn wm8974_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: core::ffi::c_uint) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let mut iface: u16 = 0;
    let mut clk: u16 = (snd_soc_component_read(component, WM8974_CLOCK) & 0x1fe) as u16;

    /* set master/slave audio interface */
    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => {
            clk |= 0x0001;
        }
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            iface |= 0x0010;
        }
        SND_SOC_DAIFMT_RIGHT_J => {}
        SND_SOC_DAIFMT_LEFT_J => {
            iface |= 0x0008;
        }
        SND_SOC_DAIFMT_DSP_A => {
            if (fmt & SND_SOC_DAIFMT_INV_MASK) == SND_SOC_DAIFMT_IB_IF
                || (fmt & SND_SOC_DAIFMT_INV_MASK) == SND_SOC_DAIFMT_NB_IF
            {
                return -EINVAL;
            }
            iface |= 0x0018;
        }
        _ => return -EINVAL,
    }

    /* clock inversion */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_IF => {
            iface |= 0x0180;
        }
        SND_SOC_DAIFMT_IB_NF => {
            iface |= 0x0100;
        }
        SND_SOC_DAIFMT_NB_IF => {
            iface |= 0x0080;
        }
        _ => return -EINVAL,
    }

    snd_soc_component_write(component, WM8974_IFACE, iface as core::ffi::c_uint);
    snd_soc_component_write(component, WM8974_CLOCK, clk as core::ffi::c_uint);
    0
}

unsafe fn wm8974_pcm_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let priv_: *mut wm8974_priv = snd_soc_component_get_drvdata(component) as *mut wm8974_priv;
    let mut iface: u16 = (snd_soc_component_read(component, WM8974_IFACE) & 0x19f) as u16;
    let mut adn: u16 = (snd_soc_component_read(component, WM8974_ADD) & 0x1f1) as u16;
    let err: core::ffi::c_int;

    (*priv_).fs = params_rate(params);
    err = wm8974_update_clocks(dai);
    if err != 0 {
        return err;
    }

    /* bit size */
    match params_width(params) {
        16 => {}
        20 => {
            iface |= 0x0020;
        }
        24 => {
            iface |= 0x0040;
        }
        32 => {
            iface |= 0x0060;
        }
        _ => {}
    }

    /* filter coefficient */
    match params_rate(params) {
        8000 => {
            adn |= 0x5 << 1;
        }
        11025 => {
            adn |= 0x4 << 1;
        }
        16000 => {
            adn |= 0x3 << 1;
        }
        22050 => {
            adn |= 0x2 << 1;
        }
        32000 => {
            adn |= 0x1 << 1;
        }
        44100 | 48000 => {}
        _ => {}
    }

    snd_soc_component_write(component, WM8974_IFACE, iface as core::ffi::c_uint);
    snd_soc_component_write(component, WM8974_ADD, adn as core::ffi::c_uint);
    0
}

unsafe fn wm8974_mute(
    dai: *mut snd_soc_dai,
    mute: core::ffi::c_int,
    _direction: core::ffi::c_int,
) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let mute_reg: u16 = (snd_soc_component_read(component, WM8974_DAC) & 0xffbf) as u16;

    if mute != 0 {
        snd_soc_component_write(component, WM8974_DAC, (mute_reg | 0x40) as core::ffi::c_uint);
    } else {
        snd_soc_component_write(component, WM8974_DAC, mute_reg as core::ffi::c_uint);
    }
    0
}

/* liam need to make this lower power with dapm */
unsafe fn wm8974_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> core::ffi::c_int {
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);
    let mut power1: u16 = (snd_soc_component_read(component, WM8974_POWER1) & !0x3) as u16;

    match level {
        SND_SOC_BIAS_ON | SND_SOC_BIAS_PREPARE => {
            power1 |= 0x1; /* VMID 50k */
            snd_soc_component_write(component, WM8974_POWER1, power1 as core::ffi::c_uint);
        }
        SND_SOC_BIAS_STANDBY => {
            power1 |= WM8974_POWER1_BIASEN | WM8974_POWER1_BUFIOEN;

            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                regcache_sync(dev_get_regmap((*component).dev, core::ptr::null()));

                /* Initial cap charge at VMID 5k */
                snd_soc_component_write(component, WM8974_POWER1, (power1 | 0x3) as core::ffi::c_uint);
                mdelay(100);
            }

            power1 |= 0x2; /* VMID 500k */
            snd_soc_component_write(component, WM8974_POWER1, power1 as core::ffi::c_uint);
        }
        SND_SOC_BIAS_OFF => {
            snd_soc_component_write(component, WM8974_POWER1, 0);
            snd_soc_component_write(component, WM8974_POWER2, 0);
            snd_soc_component_write(component, WM8974_POWER3, 0);
        }
    }

    0
}

const WM8974_RATES: core::ffi::c_uint = SNDRV_PCM_RATE_8000_48000;

const WM8974_FORMATS: core::ffi::c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE;

static wm8974_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(wm8974_pcm_hw_params),
    mute_stream: Some(wm8974_mute),
    set_fmt: Some(wm8974_set_dai_fmt),
    set_clkdiv: Some(wm8974_set_dai_clkdiv),
    set_pll: Some(wm8974_set_dai_pll),
    set_sysclk: Some(wm8974_set_dai_sysclk),
    no_capture_mute: 1,
};

static mut wm8974_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: "wm8974-hifi",
    playback: snd_soc_pcm_stream {
        stream_name: "Playback",
        channels_min: 1,
        channels_max: 2, /* Only 1 channel of data */
        rates: WM8974_RATES,
        formats: WM8974_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: "Capture",
        channels_min: 1,
        channels_max: 2, /* Only 1 channel of data */
        rates: WM8974_RATES,
        formats: WM8974_FORMATS,
    },
    ops: &wm8974_ops,
    symmetric_rate: 1,
};

static wm8974_regmap: regmap_config = regmap_config {
    reg_bits: 7,
    val_bits: 9,
    max_register: WM8974_MONOMIX,
    reg_defaults: wm8974_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE!(wm8974_reg_defaults),
    cache_type: REGCACHE_FLAT,
};

unsafe fn wm8974_probe(component: *mut snd_soc_component) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int = 0;

    ret = wm8974_reset(component);
    if ret < 0 {
        dev_err((*component).dev, "Failed to issue reset\n");
        return ret;
    }

    0
}

static soc_component_dev_wm8974: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm8974_probe),
    set_bias_level: Some(wm8974_set_bias_level),
    controls: wm8974_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(wm8974_snd_controls),
    dapm_widgets: wm8974_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(wm8974_dapm_widgets),
    dapm_routes: wm8974_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(wm8974_dapm_routes),
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe fn wm8974_i2c_probe(i2c: *mut i2c_client) -> core::ffi::c_int {
    let priv_: *mut wm8974_priv;
    let regmap: *mut regmap;
    let ret: core::ffi::c_int;

    priv_ = devm_kzalloc(
        &mut (*i2c).dev,
        core::mem::size_of::<wm8974_priv>(),
        GFP_KERNEL,
    ) as *mut wm8974_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c, priv_ as *mut core::ffi::c_void);

    regmap = devm_regmap_init_i2c(i2c, &wm8974_regmap);
    if IS_ERR(regmap as *const core::ffi::c_void) {
        return PTR_ERR(regmap as *const core::ffi::c_void) as core::ffi::c_int;
    }

    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_component_dev_wm8974,
        &mut wm8974_dai,
        1,
    );

    ret
}

static wm8974_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: "wm8974" },
    i2c_device_id::default(),
];
MODULE_DEVICE_TABLE!(i2c, wm8974_i2c_id);

static wm8974_of_match: [of_device_id; 2] = [
    of_device_id { compatible: "wlf,wm8974" },
    of_device_id::default(),
];
MODULE_DEVICE_TABLE!(of, wm8974_of_match);

static mut wm8974_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: "wm8974",
        of_match_table: wm8974_of_match.as_ptr(),
    },
    probe: Some(wm8974_i2c_probe),
    id_table: wm8974_i2c_id.as_ptr(),
};

module_i2c_driver!(wm8974_i2c_driver);

MODULE_DESCRIPTION!("ASoC WM8974 driver");
MODULE_AUTHOR!("Liam Girdwood");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
