// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8955.rs  --  WM8955 ALSA SoC Audio driver
 *
 * Copyright 2009 Wolfson Microelectronics plc
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 *
 * Rust translation of wm8955.c.
 */

// C dependencies:
// linux/module.h, linux/moduleparam.h, linux/init.h, linux/delay.h,
// linux/pm.h, linux/i2c.h, linux/regmap.h, linux/regulator/consumer.h,
// linux/slab.h, sound/core.h, sound/pcm.h, sound/pcm_params.h,
// sound/soc.h, sound/initval.h, sound/tlv.h, sound/wm8955.h, "wm8955.h"

const WM8955_NUM_SUPPLIES: usize = 4;

static wm8955_supply_names: [*const c_char; WM8955_NUM_SUPPLIES] = [
    c"DCVDD".as_ptr(),
    c"DBVDD".as_ptr(),
    c"HPVDD".as_ptr(),
    c"AVDD".as_ptr(),
];

/* codec private data */
#[repr(C)]
struct wm8955_priv {
    regmap: *mut regmap,
    mclk_rate: c_uint,
    deemph: c_int,
    fs: c_int,
    supplies: [regulator_bulk_data; WM8955_NUM_SUPPLIES],
}

static wm8955_reg_defaults: [reg_default; 28] = [
    reg_default { reg: 2, def: 0x0079 },     /* R2  - LOUT1 volume */
    reg_default { reg: 3, def: 0x0079 },     /* R3  - ROUT1 volume */
    reg_default { reg: 5, def: 0x0008 },     /* R5  - DAC Control */
    reg_default { reg: 7, def: 0x000A },     /* R7  - Audio Interface */
    reg_default { reg: 8, def: 0x0000 },     /* R8  - Sample Rate */
    reg_default { reg: 10, def: 0x00FF },    /* R10 - Left DAC volume */
    reg_default { reg: 11, def: 0x00FF },    /* R11 - Right DAC volume */
    reg_default { reg: 12, def: 0x000F },    /* R12 - Bass control */
    reg_default { reg: 13, def: 0x000F },    /* R13 - Treble control */
    reg_default { reg: 23, def: 0x00C1 },    /* R23 - Additional control (1) */
    reg_default { reg: 24, def: 0x0000 },    /* R24 - Additional control (2) */
    reg_default { reg: 25, def: 0x0000 },    /* R25 - Power Management (1) */
    reg_default { reg: 26, def: 0x0000 },    /* R26 - Power Management (2) */
    reg_default { reg: 27, def: 0x0000 },    /* R27 - Additional Control (3) */
    reg_default { reg: 34, def: 0x0050 },    /* R34 - Left out Mix (1) */
    reg_default { reg: 35, def: 0x0050 },    /* R35 - Left out Mix (2) */
    reg_default { reg: 36, def: 0x0050 },    /* R36 - Right out Mix (1) */
    reg_default { reg: 37, def: 0x0050 },    /* R37 - Right Out Mix (2) */
    reg_default { reg: 38, def: 0x0050 },    /* R38 - Mono out Mix (1) */
    reg_default { reg: 39, def: 0x0050 },    /* R39 - Mono out Mix (2) */
    reg_default { reg: 40, def: 0x0079 },    /* R40 - LOUT2 volume */
    reg_default { reg: 41, def: 0x0079 },    /* R41 - ROUT2 volume */
    reg_default { reg: 42, def: 0x0079 },    /* R42 - MONOOUT volume */
    reg_default { reg: 43, def: 0x0000 },    /* R43 - Clocking / PLL */
    reg_default { reg: 44, def: 0x0103 },    /* R44 - PLL Control 1 */
    reg_default { reg: 45, def: 0x0024 },    /* R45 - PLL Control 2 */
    reg_default { reg: 46, def: 0x01BA },    /* R46 - PLL Control 3 */
    reg_default { reg: 59, def: 0x0000 },    /* R59 - PLL Control 4 */
];

unsafe extern "C" fn wm8955_writeable(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        WM8955_LOUT1_VOLUME | WM8955_ROUT1_VOLUME | WM8955_DAC_CONTROL |
        WM8955_AUDIO_INTERFACE | WM8955_SAMPLE_RATE | WM8955_LEFT_DAC_VOLUME |
        WM8955_RIGHT_DAC_VOLUME | WM8955_BASS_CONTROL | WM8955_TREBLE_CONTROL |
        WM8955_RESET | WM8955_ADDITIONAL_CONTROL_1 | WM8955_ADDITIONAL_CONTROL_2 |
        WM8955_POWER_MANAGEMENT_1 | WM8955_POWER_MANAGEMENT_2 |
        WM8955_ADDITIONAL_CONTROL_3 | WM8955_LEFT_OUT_MIX_1 |
        WM8955_LEFT_OUT_MIX_2 | WM8955_RIGHT_OUT_MIX_1 |
        WM8955_RIGHT_OUT_MIX_2 | WM8955_MONO_OUT_MIX_1 |
        WM8955_MONO_OUT_MIX_2 | WM8955_LOUT2_VOLUME | WM8955_ROUT2_VOLUME |
        WM8955_MONOOUT_VOLUME | WM8955_CLOCKING_PLL | WM8955_PLL_CONTROL_1 |
        WM8955_PLL_CONTROL_2 | WM8955_PLL_CONTROL_3 | WM8955_PLL_CONTROL_4 => true,
        _ => false,
    }
}

unsafe extern "C" fn wm8955_volatile(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        WM8955_RESET => true,
        _ => false,
    }
}

unsafe extern "C" fn wm8955_reset(component: *mut snd_soc_component) -> c_int {
    snd_soc_component_write(component, WM8955_RESET, 0)
}

#[repr(C)]
struct pll_factors {
    n: c_int,
    k: c_int,
    outdiv: c_int,
}

/* The size in bits of the FLL divide multiplied by 10
 * to allow rounding later */
const FIXED_FLL_SIZE: u64 = ((1u64 << 22) * 10);

unsafe fn wm8955_pll_factors(
    dev: *mut device,
    Fref: c_int,
    Fout: c_int,
    pll: *mut pll_factors,
) -> c_int {
    let mut Kpart: u64;
    let mut K: c_uint;
    let Ndiv: c_uint;
    let Nmod: c_uint;
    let mut target: c_uint;

    dev_dbg(dev, c"Fref=%u Fout=%u\n".as_ptr(), Fref, Fout);

    /* The oscilator should run at should be 90-100MHz, and
     * there's a divide by 4 plus an optional divide by 2 in the
     * output path to generate the system clock.  The clock table
     * is sortd so we should always generate a suitable target. */
    target = (Fout as c_uint).wrapping_mul(4);
    if target < 90000000 {
        (*pll).outdiv = 1;
        target = target.wrapping_mul(2);
    } else {
        (*pll).outdiv = 0;
    }

    WARN_ON(target < 90000000 || target > 100000000);

    dev_dbg(dev, c"Fvco=%dHz\n".as_ptr(), target);

    /* Now, calculate N.K */
    Ndiv = target / Fref as c_uint;

    (*pll).n = Ndiv as c_int;
    Nmod = target % Fref as c_uint;
    dev_dbg(dev, c"Nmod=%d\n".as_ptr(), Nmod);

    /* Calculate fractional part - scale up so we can round. */
    Kpart = FIXED_FLL_SIZE.wrapping_mul(Nmod as i64 as u64);
    Kpart /= Fref as u64;
    K = (Kpart & 0xFFFFFFFF) as c_uint;

    if K % 10 >= 5 {
        K = K.wrapping_add(5);
    }

    /* Move down to proper range now rounding is done */
    (*pll).k = (K / 10) as c_int;

    dev_dbg(dev, c"N=%x K=%x OUTDIV=%x\n".as_ptr(), (*pll).n, (*pll).k, (*pll).outdiv);

    0
}

/* Lookup table specifying SRATE (table 25 in datasheet); some of the
 * output frequencies have been rounded to the standard frequencies
 * they are intended to match where the error is slight. */
#[repr(C)]
struct clock_cfg {
    mclk: c_int,
    fs: c_int,
    usb: c_int,
    sr: c_int,
}

static mut clock_cfgs: [clock_cfg; 36] = [
    clock_cfg { mclk: 18432000, fs: 8000, usb: 0, sr: 3 },
    clock_cfg { mclk: 18432000, fs: 12000, usb: 0, sr: 9 },
    clock_cfg { mclk: 18432000, fs: 16000, usb: 0, sr: 11 },
    clock_cfg { mclk: 18432000, fs: 24000, usb: 0, sr: 29 },
    clock_cfg { mclk: 18432000, fs: 32000, usb: 0, sr: 13 },
    clock_cfg { mclk: 18432000, fs: 48000, usb: 0, sr: 1 },
    clock_cfg { mclk: 18432000, fs: 96000, usb: 0, sr: 15 },
    clock_cfg { mclk: 16934400, fs: 8018, usb: 0, sr: 19 },
    clock_cfg { mclk: 16934400, fs: 11025, usb: 0, sr: 25 },
    clock_cfg { mclk: 16934400, fs: 22050, usb: 0, sr: 27 },
    clock_cfg { mclk: 16934400, fs: 44100, usb: 0, sr: 17 },
    clock_cfg { mclk: 16934400, fs: 88200, usb: 0, sr: 31 },
    clock_cfg { mclk: 12000000, fs: 8000, usb: 1, sr: 2 },
    clock_cfg { mclk: 12000000, fs: 11025, usb: 1, sr: 25 },
    clock_cfg { mclk: 12000000, fs: 12000, usb: 1, sr: 8 },
    clock_cfg { mclk: 12000000, fs: 16000, usb: 1, sr: 10 },
    clock_cfg { mclk: 12000000, fs: 22050, usb: 1, sr: 27 },
    clock_cfg { mclk: 12000000, fs: 24000, usb: 1, sr: 28 },
    clock_cfg { mclk: 12000000, fs: 32000, usb: 1, sr: 12 },
    clock_cfg { mclk: 12000000, fs: 44100, usb: 1, sr: 17 },
    clock_cfg { mclk: 12000000, fs: 48000, usb: 1, sr: 0 },
    clock_cfg { mclk: 12000000, fs: 88200, usb: 1, sr: 31 },
    clock_cfg { mclk: 12000000, fs: 96000, usb: 1, sr: 14 },
    clock_cfg { mclk: 12288000, fs: 8000, usb: 0, sr: 2 },
    clock_cfg { mclk: 12288000, fs: 12000, usb: 0, sr: 8 },
    clock_cfg { mclk: 12288000, fs: 16000, usb: 0, sr: 10 },
    clock_cfg { mclk: 12288000, fs: 24000, usb: 0, sr: 28 },
    clock_cfg { mclk: 12288000, fs: 32000, usb: 0, sr: 12 },
    clock_cfg { mclk: 12288000, fs: 48000, usb: 0, sr: 0 },
    clock_cfg { mclk: 12288000, fs: 96000, usb: 0, sr: 14 },
    clock_cfg { mclk: 12289600, fs: 8018, usb: 0, sr: 18 },
    clock_cfg { mclk: 12289600, fs: 11025, usb: 0, sr: 24 },
    clock_cfg { mclk: 12289600, fs: 22050, usb: 0, sr: 26 },
    clock_cfg { mclk: 11289600, fs: 44100, usb: 0, sr: 16 },
    clock_cfg { mclk: 11289600, fs: 88200, usb: 0, sr: 31 },
];

unsafe extern "C" fn wm8955_configure_clocking(component: *mut snd_soc_component) -> c_int {
    let wm8955 = snd_soc_component_get_drvdata(component) as *mut wm8955_priv;
    let mut i: usize;
    let mut ret: c_int;
    let mut val: c_int;
    let mut clocking: c_int = 0;
    let mut srate: c_int;
    let mut sr: c_int = -1;
    let mut pll = pll_factors { n: 0, k: 0, outdiv: 0 };

    /* If we're not running a sample rate currently just pick one */
    if (*wm8955).fs == 0 {
        (*wm8955).fs = 8000;
    }

    /* Can we generate an exact output? */
    i = 0;
    while i < clock_cfgs.len() {
        if (*wm8955).fs != clock_cfgs[i].fs {
            i += 1;
            continue;
        }
        sr = i as c_int;
        if (*wm8955).mclk_rate as c_int == clock_cfgs[i].mclk {
            break;
        }
        i += 1;
    }

    /* We should never get here with an unsupported sample rate */
    if sr == -1 {
        dev_err((*component).dev, c"Sample rate %dHz unsupported\n".as_ptr(), (*wm8955).fs);
        WARN_ON(sr == -1);
        return -EINVAL;
    }

    if i == clock_cfgs.len() {
        /* If we can't generate the right clock from MCLK then
         * we should configure the PLL to supply us with an
         * appropriate clock.
         */
        clocking |= WM8955_MCLKSEL;

        /* Use the last divider configuration we saw for the
         * sample rate. */
        ret = wm8955_pll_factors((*component).dev, (*wm8955).mclk_rate as c_int,
                                 clock_cfgs[sr as usize].mclk, &mut pll);
        if ret != 0 {
            dev_err((*component).dev,
                    c"Unable to generate %dHz from %dHz MCLK\n".as_ptr(),
                    (*wm8955).fs, (*wm8955).mclk_rate);
            return -EINVAL;
        }

        snd_soc_component_update_bits(component, WM8955_PLL_CONTROL_1,
            WM8955_N_MASK | WM8955_K_21_18_MASK,
            (pll.n << WM8955_N_SHIFT) | (pll.k >> 18));
        snd_soc_component_update_bits(component, WM8955_PLL_CONTROL_2,
            WM8955_K_17_9_MASK, (pll.k >> 9) & WM8955_K_17_9_MASK);
        snd_soc_component_update_bits(component, WM8955_PLL_CONTROL_3,
            WM8955_K_8_0_MASK, pll.k & WM8955_K_8_0_MASK);
        if pll.k != 0 {
            snd_soc_component_update_bits(component, WM8955_PLL_CONTROL_4, WM8955_KEN, WM8955_KEN);
        } else {
            snd_soc_component_update_bits(component, WM8955_PLL_CONTROL_4, WM8955_KEN, 0);
        }

        if pll.outdiv != 0 {
            val = WM8955_PLL_RB | WM8955_PLLOUTDIV2;
        } else {
            val = WM8955_PLL_RB;
        }

        /* Now start the PLL running */
        snd_soc_component_update_bits(component, WM8955_CLOCKING_PLL,
            WM8955_PLL_RB | WM8955_PLLOUTDIV2, val);
        snd_soc_component_update_bits(component, WM8955_CLOCKING_PLL, WM8955_PLLEN, WM8955_PLLEN);
    }

    srate = clock_cfgs[sr as usize].usb | (clock_cfgs[sr as usize].sr << WM8955_SR_SHIFT);
    snd_soc_component_update_bits(component, WM8955_SAMPLE_RATE, WM8955_USB | WM8955_SR_MASK, srate);
    snd_soc_component_update_bits(component, WM8955_CLOCKING_PLL, WM8955_MCLKSEL, clocking);

    0
}

unsafe extern "C" fn wm8955_sysclk(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let mut ret: c_int = 0;

    /* Always disable the clocks - if we're doing reconfiguration this
     * avoids misclocking.
     */
    snd_soc_component_update_bits(component, WM8955_POWER_MANAGEMENT_1, WM8955_DIGENB, 0);
    snd_soc_component_update_bits(component, WM8955_CLOCKING_PLL, WM8955_PLL_RB | WM8955_PLLEN, 0);

    match event {
        SND_SOC_DAPM_POST_PMD => {}
        SND_SOC_DAPM_PRE_PMU => ret = wm8955_configure_clocking(component),
        _ => ret = -EINVAL,
    }

    ret
}

static mut deemph_settings: [c_int; 4] = [0, 32000, 44100, 48000];

unsafe extern "C" fn wm8955_set_deemph(component: *mut snd_soc_component) -> c_int {
    let wm8955 = snd_soc_component_get_drvdata(component) as *mut wm8955_priv;
    let mut val: c_int;
    let mut i: usize;
    let mut best: usize;

    /* If we're using deemphasis select the nearest available sample
     * rate.
     */
    if (*wm8955).deemph != 0 {
        best = 1;
        i = 2;
        while i < deemph_settings.len() {
            if abs(deemph_settings[i] - (*wm8955).fs) <
               abs(deemph_settings[best] - (*wm8955).fs) {
                best = i;
            }
            i += 1;
        }
        val = (best as c_int) << WM8955_DEEMPH_SHIFT;
    } else {
        val = 0;
    }

    dev_dbg((*component).dev, c"Set deemphasis %d\n".as_ptr(), val);

    snd_soc_component_update_bits(component, WM8955_DAC_CONTROL, WM8955_DEEMPH_MASK, val)
}

unsafe extern "C" fn wm8955_get_deemph(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let wm8955 = snd_soc_component_get_drvdata(component) as *mut wm8955_priv;

    (*ucontrol).value.integer.value[0] = (*wm8955).deemph as c_long;
    0
}

unsafe extern "C" fn wm8955_put_deemph(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let wm8955 = snd_soc_component_get_drvdata(component) as *mut wm8955_priv;
    let deemph: c_uint = (*ucontrol).value.integer.value[0] as c_uint;

    if deemph > 1 {
        return -EINVAL;
    }

    (*wm8955).deemph = deemph as c_int;
    wm8955_set_deemph(component)
}

static bass_mode_text: [*const c_char; 2] = [c"Linear".as_ptr(), c"Adaptive".as_ptr()];
SOC_ENUM_SINGLE_DECL!(bass_mode, WM8955_BASS_CONTROL, 7, bass_mode_text);

static bass_cutoff_text: [*const c_char; 2] = [c"Low".as_ptr(), c"High".as_ptr()];
SOC_ENUM_SINGLE_DECL!(bass_cutoff, WM8955_BASS_CONTROL, 6, bass_cutoff_text);

static treble_cutoff_text: [*const c_char; 2] = [c"High".as_ptr(), c"Low".as_ptr()];
SOC_ENUM_SINGLE_DECL!(treble_cutoff, WM8955_TREBLE_CONTROL, 2, treble_cutoff_text);

DECLARE_TLV_DB_SCALE!(digital_tlv, -12750, 50, 1);
DECLARE_TLV_DB_SCALE!(atten_tlv, -600, 600, 0);
DECLARE_TLV_DB_SCALE!(bypass_tlv, -1500, 300, 0);
DECLARE_TLV_DB_SCALE!(mono_tlv, -2100, 300, 0);
DECLARE_TLV_DB_SCALE!(out_tlv, -12100, 100, 1);
DECLARE_TLV_DB_SCALE!(treble_tlv, -1200, 150, 1);

static wm8955_snd_controls: [snd_kcontrol_new; 24] = [
    SOC_DOUBLE_R_TLV!("Digital Playback Volume", WM8955_LEFT_DAC_VOLUME, WM8955_RIGHT_DAC_VOLUME, 0, 255, 0, digital_tlv),
    SOC_SINGLE_TLV!("Playback Attenuation Volume", WM8955_DAC_CONTROL, 7, 1, 1, atten_tlv),
    SOC_SINGLE_BOOL_EXT!("DAC Deemphasis Switch", 0, wm8955_get_deemph, wm8955_put_deemph),
    SOC_ENUM!("Bass Mode", bass_mode),
    SOC_ENUM!("Bass Cutoff", bass_cutoff),
    SOC_SINGLE!("Bass Volume", WM8955_BASS_CONTROL, 0, 15, 1),
    SOC_ENUM!("Treble Cutoff", treble_cutoff),
    SOC_SINGLE_TLV!("Treble Volume", WM8955_TREBLE_CONTROL, 0, 14, 1, treble_tlv),
    SOC_SINGLE_TLV!("Left Bypass Volume", WM8955_LEFT_OUT_MIX_1, 4, 7, 1, bypass_tlv),
    SOC_SINGLE_TLV!("Left Mono Volume", WM8955_LEFT_OUT_MIX_2, 4, 7, 1, bypass_tlv),
    SOC_SINGLE_TLV!("Right Mono Volume", WM8955_RIGHT_OUT_MIX_1, 4, 7, 1, bypass_tlv),
    SOC_SINGLE_TLV!("Right Bypass Volume", WM8955_RIGHT_OUT_MIX_2, 4, 7, 1, bypass_tlv),
    /* Not a stereo pair so they line up with the DAPM switches */
    SOC_SINGLE_TLV!("Mono Left Bypass Volume", WM8955_MONO_OUT_MIX_1, 4, 7, 1, mono_tlv),
    SOC_SINGLE_TLV!("Mono Right Bypass Volume", WM8955_MONO_OUT_MIX_2, 4, 7, 1, mono_tlv),
    SOC_DOUBLE_R_TLV!("Headphone Volume", WM8955_LOUT1_VOLUME, WM8955_ROUT1_VOLUME, 0, 127, 0, out_tlv),
    SOC_DOUBLE_R!("Headphone ZC Switch", WM8955_LOUT1_VOLUME, WM8955_ROUT1_VOLUME, 7, 1, 0),
    SOC_DOUBLE_R_TLV!("Speaker Volume", WM8955_LOUT2_VOLUME, WM8955_ROUT2_VOLUME, 0, 127, 0, out_tlv),
    SOC_DOUBLE_R!("Speaker ZC Switch", WM8955_LOUT2_VOLUME, WM8955_ROUT2_VOLUME, 7, 1, 0),
    SOC_SINGLE_TLV!("Mono Volume", WM8955_MONOOUT_VOLUME, 0, 127, 0, out_tlv),
    SOC_SINGLE!("Mono ZC Switch", WM8955_MONOOUT_VOLUME, 7, 1, 0),
];

static lmixer: [snd_kcontrol_new; 4] = [
    SOC_DAPM_SINGLE!("Playback Switch", WM8955_LEFT_OUT_MIX_1, 8, 1, 0),
    SOC_DAPM_SINGLE!("Bypass Switch", WM8955_LEFT_OUT_MIX_1, 7, 1, 0),
    SOC_DAPM_SINGLE!("Right Playback Switch", WM8955_LEFT_OUT_MIX_2, 8, 1, 0),
    SOC_DAPM_SINGLE!("Mono Switch", WM8955_LEFT_OUT_MIX_2, 7, 1, 0),
];

static rmixer: [snd_kcontrol_new; 4] = [
    SOC_DAPM_SINGLE!("Left Playback Switch", WM8955_RIGHT_OUT_MIX_1, 8, 1, 0),
    SOC_DAPM_SINGLE!("Mono Switch", WM8955_RIGHT_OUT_MIX_1, 7, 1, 0),
    SOC_DAPM_SINGLE!("Playback Switch", WM8955_RIGHT_OUT_MIX_2, 8, 1, 0),
    SOC_DAPM_SINGLE!("Bypass Switch", WM8955_RIGHT_OUT_MIX_2, 7, 1, 0),
];

static mmixer: [snd_kcontrol_new; 4] = [
    SOC_DAPM_SINGLE!("Left Playback Switch", WM8955_MONO_OUT_MIX_1, 8, 1, 0),
    SOC_DAPM_SINGLE!("Left Bypass Switch", WM8955_MONO_OUT_MIX_1, 7, 1, 0),
    SOC_DAPM_SINGLE!("Right Playback Switch", WM8955_MONO_OUT_MIX_2, 8, 1, 0),
    SOC_DAPM_SINGLE!("Right Bypass Switch", WM8955_MONO_OUT_MIX_2, 7, 1, 0),
];

static wm8955_dapm_widgets: [snd_soc_dapm_widget; 20] = [
    SND_SOC_DAPM_INPUT!("MONOIN-"),
    SND_SOC_DAPM_INPUT!("MONOIN+"),
    SND_SOC_DAPM_INPUT!("LINEINR"),
    SND_SOC_DAPM_INPUT!("LINEINL"),
    SND_SOC_DAPM_PGA!("Mono Input", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("SYSCLK", WM8955_POWER_MANAGEMENT_1, 0, 1, wm8955_sysclk, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY!("TSDEN", WM8955_ADDITIONAL_CONTROL_1, 8, 0, NULL, 0),
    SND_SOC_DAPM_DAC!("DACL", "Playback", WM8955_POWER_MANAGEMENT_2, 8, 0),
    SND_SOC_DAPM_DAC!("DACR", "Playback", WM8955_POWER_MANAGEMENT_2, 7, 0),
    SND_SOC_DAPM_PGA!("LOUT1 PGA", WM8955_POWER_MANAGEMENT_2, 6, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("ROUT1 PGA", WM8955_POWER_MANAGEMENT_2, 5, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("LOUT2 PGA", WM8955_POWER_MANAGEMENT_2, 4, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("ROUT2 PGA", WM8955_POWER_MANAGEMENT_2, 3, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("MOUT PGA", WM8955_POWER_MANAGEMENT_2, 2, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("OUT3 PGA", WM8955_POWER_MANAGEMENT_2, 1, 0, NULL, 0),
    /* The names are chosen to make the control names nice */
    SND_SOC_DAPM_MIXER!("Left", SND_SOC_NOPM, 0, 0, lmixer, lmixer.len()),
    SND_SOC_DAPM_MIXER!("Right", SND_SOC_NOPM, 0, 0, rmixer, rmixer.len()),
    SND_SOC_DAPM_MIXER!("Mono", SND_SOC_NOPM, 0, 0, mmixer, mmixer.len()),
    SND_SOC_DAPM_OUTPUT!("LOUT1"), SND_SOC_DAPM_OUTPUT!("ROUT1"), SND_SOC_DAPM_OUTPUT!("LOUT2"),
    SND_SOC_DAPM_OUTPUT!("ROUT2"), SND_SOC_DAPM_OUTPUT!("MONOOUT"), SND_SOC_DAPM_OUTPUT!("OUT3"),
];

static wm8955_dapm_routes: [snd_soc_dapm_route; 30] = [
    snd_soc_dapm_route { sink: c"DACL".as_ptr(), control: NULL, source: c"SYSCLK".as_ptr() },
    snd_soc_dapm_route { sink: c"DACR".as_ptr(), control: NULL, source: c"SYSCLK".as_ptr() },
    snd_soc_dapm_route { sink: c"Mono Input".as_ptr(), control: NULL, source: c"MONOIN-".as_ptr() },
    snd_soc_dapm_route { sink: c"Mono Input".as_ptr(), control: NULL, source: c"MONOIN+".as_ptr() },
    snd_soc_dapm_route { sink: c"Left".as_ptr(), control: c"Playback Switch".as_ptr(), source: c"DACL".as_ptr() },
    snd_soc_dapm_route { sink: c"Left".as_ptr(), control: c"Right Playback Switch".as_ptr(), source: c"DACR".as_ptr() },
    snd_soc_dapm_route { sink: c"Left".as_ptr(), control: c"Bypass Switch".as_ptr(), source: c"LINEINL".as_ptr() },
    snd_soc_dapm_route { sink: c"Left".as_ptr(), control: c"Mono Switch".as_ptr(), source: c"Mono Input".as_ptr() },
    snd_soc_dapm_route { sink: c"Right".as_ptr(), control: c"Playback Switch".as_ptr(), source: c"DACR".as_ptr() },
    snd_soc_dapm_route { sink: c"Right".as_ptr(), control: c"Left Playback Switch".as_ptr(), source: c"DACL".as_ptr() },
    snd_soc_dapm_route { sink: c"Right".as_ptr(), control: c"Bypass Switch".as_ptr(), source: c"LINEINR".as_ptr() },
    snd_soc_dapm_route { sink: c"Right".as_ptr(), control: c"Mono Switch".as_ptr(), source: c"Mono Input".as_ptr() },
    snd_soc_dapm_route { sink: c"Mono".as_ptr(), control: c"Left Playback Switch".as_ptr(), source: c"DACL".as_ptr() },
    snd_soc_dapm_route { sink: c"Mono".as_ptr(), control: c"Right Playback Switch".as_ptr(), source: c"DACR".as_ptr() },
    snd_soc_dapm_route { sink: c"Mono".as_ptr(), control: c"Left Bypass Switch".as_ptr(), source: c"LINEINL".as_ptr() },
    snd_soc_dapm_route { sink: c"Mono".as_ptr(), control: c"Right Bypass Switch".as_ptr(), source: c"LINEINR".as_ptr() },
    snd_soc_dapm_route { sink: c"LOUT1 PGA".as_ptr(), control: NULL, source: c"Left".as_ptr() },
    snd_soc_dapm_route { sink: c"LOUT1".as_ptr(), control: NULL, source: c"TSDEN".as_ptr() },
    snd_soc_dapm_route { sink: c"LOUT1".as_ptr(), control: NULL, source: c"LOUT1 PGA".as_ptr() },
    snd_soc_dapm_route { sink: c"ROUT1 PGA".as_ptr(), control: NULL, source: c"Right".as_ptr() },
    snd_soc_dapm_route { sink: c"ROUT1".as_ptr(), control: NULL, source: c"TSDEN".as_ptr() },
    snd_soc_dapm_route { sink: c"ROUT1".as_ptr(), control: NULL, source: c"ROUT1 PGA".as_ptr() },
    snd_soc_dapm_route { sink: c"LOUT2 PGA".as_ptr(), control: NULL, source: c"Left".as_ptr() },
    snd_soc_dapm_route { sink: c"LOUT2".as_ptr(), control: NULL, source: c"TSDEN".as_ptr() },
    snd_soc_dapm_route { sink: c"LOUT2".as_ptr(), control: NULL, source: c"LOUT2 PGA".as_ptr() },
    snd_soc_dapm_route { sink: c"ROUT2 PGA".as_ptr(), control: NULL, source: c"Right".as_ptr() },
    snd_soc_dapm_route { sink: c"ROUT2".as_ptr(), control: NULL, source: c"TSDEN".as_ptr() },
    snd_soc_dapm_route { sink: c"ROUT2".as_ptr(), control: NULL, source: c"ROUT2 PGA".as_ptr() },
    snd_soc_dapm_route { sink: c"MOUT PGA".as_ptr(), control: NULL, source: c"Mono".as_ptr() },
    snd_soc_dapm_route { sink: c"MONOOUT".as_ptr(), control: NULL, source: c"MOUT PGA".as_ptr() },
    /* OUT3 not currently implemented */
    snd_soc_dapm_route { sink: c"OUT3".as_ptr(), control: NULL, source: c"OUT3 PGA".as_ptr() },
];

unsafe extern "C" fn wm8955_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let wm8955 = snd_soc_component_get_drvdata(component) as *mut wm8955_priv;
    let ret: c_int;
    let wl: c_int;

    match params_width(params) {
        16 => wl = 0,
        20 => wl = 0x4,
        24 => wl = 0x8,
        32 => wl = 0xc,
        _ => return -EINVAL,
    }
    snd_soc_component_update_bits(component, WM8955_AUDIO_INTERFACE, WM8955_WL_MASK, wl);

    (*wm8955).fs = params_rate(params);
    wm8955_set_deemph(component);

    /* If the chip is clocked then disable the clocks and force a
     * reconfiguration, otherwise DAPM will power up the
     * clocks for us later. */
    ret = snd_soc_component_read(component, WM8955_POWER_MANAGEMENT_1);
    if ret < 0 {
        return ret;
    }
    if (ret & WM8955_DIGENB) != 0 {
        snd_soc_component_update_bits(component, WM8955_POWER_MANAGEMENT_1, WM8955_DIGENB, 0);
        snd_soc_component_update_bits(component, WM8955_CLOCKING_PLL, WM8955_PLL_RB | WM8955_PLLEN, 0);
        wm8955_configure_clocking(component);
    }

    0
}

unsafe extern "C" fn wm8955_set_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    mut freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut wm8955_priv;
    let div: c_int;

    match clk_id {
        WM8955_CLK_MCLK => {
            if freq > 15000000 {
                freq /= 2;
                (*priv_).mclk_rate = freq;
                div = WM8955_MCLKDIV2;
            } else {
                (*priv_).mclk_rate = freq;
                div = 0;
            }
            snd_soc_component_update_bits(component, WM8955_SAMPLE_RATE, WM8955_MCLKDIV2, div);
        }
        _ => return -EINVAL,
    }

    dev_dbg((*dai).dev, c"Clock source is %d at %uHz\n".as_ptr(), clk_id, freq);
    0
}

unsafe extern "C" fn wm8955_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let mut aif: u16 = 0;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => {}
        SND_SOC_DAIFMT_CBP_CFP => aif |= WM8955_MS as u16,
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_DSP_B => {
            aif |= WM8955_LRP as u16;
            aif |= 0x3;
        }
        SND_SOC_DAIFMT_DSP_A => aif |= 0x3,
        SND_SOC_DAIFMT_I2S => aif |= 0x2,
        SND_SOC_DAIFMT_RIGHT_J => {}
        SND_SOC_DAIFMT_LEFT_J => aif |= 0x1,
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_DSP_B => {
            /* frame inversion not valid for DSP modes */
            match fmt & SND_SOC_DAIFMT_INV_MASK {
                SND_SOC_DAIFMT_NB_NF => {}
                SND_SOC_DAIFMT_IB_NF => aif |= WM8955_BCLKINV as u16,
                _ => return -EINVAL,
            }
        }
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_RIGHT_J | SND_SOC_DAIFMT_LEFT_J => {
            match fmt & SND_SOC_DAIFMT_INV_MASK {
                SND_SOC_DAIFMT_NB_NF => {}
                SND_SOC_DAIFMT_IB_IF => aif |= (WM8955_BCLKINV | WM8955_LRP) as u16,
                SND_SOC_DAIFMT_IB_NF => aif |= WM8955_BCLKINV as u16,
                SND_SOC_DAIFMT_NB_IF => aif |= WM8955_LRP as u16,
                _ => return -EINVAL,
            }
        }
        _ => return -EINVAL,
    }

    snd_soc_component_update_bits(component, WM8955_AUDIO_INTERFACE,
        WM8955_MS | WM8955_FORMAT_MASK | WM8955_BCLKINV | WM8955_LRP, aif as c_int);

    0
}

unsafe extern "C" fn wm8955_mute(
    codec_dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let val: c_int = if mute != 0 { WM8955_DACMU } else { 0 };

    snd_soc_component_update_bits(component, WM8955_DAC_CONTROL, WM8955_DACMU, val);
    0
}

unsafe extern "C" fn wm8955_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let wm8955 = snd_soc_component_get_drvdata(component) as *mut wm8955_priv;
    let dapm = snd_soc_component_to_dapm(component);
    let mut ret: c_int;

    match level {
        SND_SOC_BIAS_ON => {}
        SND_SOC_BIAS_PREPARE => {
            /* VMID resistance 2*50k */
            snd_soc_component_update_bits(component, WM8955_POWER_MANAGEMENT_1,
                WM8955_VMIDSEL_MASK, 0x1 << WM8955_VMIDSEL_SHIFT);
            /* Default bias current */
            snd_soc_component_update_bits(component, WM8955_ADDITIONAL_CONTROL_1,
                WM8955_VSEL_MASK, 0x2 << WM8955_VSEL_SHIFT);
        }
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                ret = regulator_bulk_enable((*wm8955).supplies.len(), (*wm8955).supplies.as_mut_ptr());
                if ret != 0 {
                    dev_err((*component).dev, c"Failed to enable supplies: %d\n".as_ptr(), ret);
                    return ret;
                }

                regcache_sync((*wm8955).regmap);

                /* Enable VREF and VMID */
                snd_soc_component_update_bits(component, WM8955_POWER_MANAGEMENT_1,
                    WM8955_VREF | WM8955_VMIDSEL_MASK,
                    WM8955_VREF | (0x3 << WM8955_VREF_SHIFT));

                /* Let VMID ramp */
                msleep(500);

                /* High resistance VROI to maintain outputs */
                snd_soc_component_update_bits(component, WM8955_ADDITIONAL_CONTROL_3,
                    WM8955_VROI, WM8955_VROI);
            }

            /* Maintain VMID with 2*250k */
            snd_soc_component_update_bits(component, WM8955_POWER_MANAGEMENT_1,
                WM8955_VMIDSEL_MASK, 0x2 << WM8955_VMIDSEL_SHIFT);

            /* Minimum bias current */
            snd_soc_component_update_bits(component, WM8955_ADDITIONAL_CONTROL_1, WM8955_VSEL_MASK, 0);
        }
        SND_SOC_BIAS_OFF => {
            /* Low resistance VROI to help discharge */
            snd_soc_component_update_bits(component, WM8955_ADDITIONAL_CONTROL_3, WM8955_VROI, 0);

            /* Turn off VMID and VREF */
            snd_soc_component_update_bits(component, WM8955_POWER_MANAGEMENT_1,
                WM8955_VREF | WM8955_VMIDSEL_MASK, 0);

            regulator_bulk_disable((*wm8955).supplies.len(), (*wm8955).supplies.as_mut_ptr());
        }
        _ => {}
    }
    0
}

const WM8955_RATES: c_uint = SNDRV_PCM_RATE_8000_96000;
const WM8955_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE |
    SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static wm8955_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_sysclk: Some(wm8955_set_sysclk),
    set_fmt: Some(wm8955_set_fmt),
    hw_params: Some(wm8955_hw_params),
    mute_stream: Some(wm8955_mute),
    no_capture_mute: 1,
    ..unsafe { core::mem::zeroed() }
};

static mut wm8955_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"wm8955-hifi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: WM8955_RATES,
        formats: WM8955_FORMATS,
        ..unsafe { core::mem::zeroed() }
    },
    ops: &wm8955_dai_ops,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn wm8955_probe(component: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let wm8955 = snd_soc_component_get_drvdata(component) as *mut wm8955_priv;
    let pdata = dev_get_platdata((*component).dev) as *mut wm8955_pdata;
    let mut ret: c_int;
    let mut i: usize;

    i = 0;
    while i < (*wm8955).supplies.len() {
        (*wm8955).supplies[i].supply = wm8955_supply_names[i];
        i += 1;
    }

    ret = devm_regulator_bulk_get((*component).dev, (*wm8955).supplies.len(), (*wm8955).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err((*component).dev, c"Failed to request supplies: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = regulator_bulk_enable((*wm8955).supplies.len(), (*wm8955).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err((*component).dev, c"Failed to enable supplies: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = wm8955_reset(component);
    if ret < 0 {
        dev_err((*component).dev, c"Failed to issue reset: %d\n".as_ptr(), ret);
        regulator_bulk_disable((*wm8955).supplies.len(), (*wm8955).supplies.as_mut_ptr());
        return ret;
    }

    /* Change some default settings - latch VU and enable ZC */
    snd_soc_component_update_bits(component, WM8955_LEFT_DAC_VOLUME, WM8955_LDVU, WM8955_LDVU);
    snd_soc_component_update_bits(component, WM8955_RIGHT_DAC_VOLUME, WM8955_RDVU, WM8955_RDVU);
    snd_soc_component_update_bits(component, WM8955_LOUT1_VOLUME, WM8955_LO1VU | WM8955_LO1ZC, WM8955_LO1VU | WM8955_LO1ZC);
    snd_soc_component_update_bits(component, WM8955_ROUT1_VOLUME, WM8955_RO1VU | WM8955_RO1ZC, WM8955_RO1VU | WM8955_RO1ZC);
    snd_soc_component_update_bits(component, WM8955_LOUT2_VOLUME, WM8955_LO2VU | WM8955_LO2ZC, WM8955_LO2VU | WM8955_LO2ZC);
    snd_soc_component_update_bits(component, WM8955_ROUT2_VOLUME, WM8955_RO2VU | WM8955_RO2ZC, WM8955_RO2VU | WM8955_RO2ZC);
    snd_soc_component_update_bits(component, WM8955_MONOOUT_VOLUME, WM8955_MOZC, WM8955_MOZC);

    /* Also enable adaptive bass boost by default */
    snd_soc_component_update_bits(component, WM8955_BASS_CONTROL, WM8955_BB, WM8955_BB);

    /* Set platform data values */
    if !pdata.is_null() {
        if (*pdata).out2_speaker != 0 {
            snd_soc_component_update_bits(component, WM8955_ADDITIONAL_CONTROL_2, WM8955_ROUT2INV, WM8955_ROUT2INV);
        }
        if (*pdata).monoin_diff != 0 {
            snd_soc_component_update_bits(component, WM8955_MONO_OUT_MIX_1, WM8955_DMEN, WM8955_DMEN);
        }
    }

    snd_soc_dapm_force_bias_level(dapm, SND_SOC_BIAS_STANDBY);

    /* Bias level configuration will have done an extra enable */
    regulator_bulk_disable((*wm8955).supplies.len(), (*wm8955).supplies.as_mut_ptr());

    0
}

static soc_component_dev_wm8955: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm8955_probe),
    set_bias_level: Some(wm8955_set_bias_level),
    controls: wm8955_snd_controls.as_ptr(),
    num_controls: wm8955_snd_controls.len() as c_uint,
    dapm_widgets: wm8955_dapm_widgets.as_ptr(),
    num_dapm_widgets: wm8955_dapm_widgets.len() as c_uint,
    dapm_routes: wm8955_dapm_routes.as_ptr(),
    num_dapm_routes: wm8955_dapm_routes.len() as c_uint,
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
    ..unsafe { core::mem::zeroed() }
};

static wm8955_regmap: regmap_config = regmap_config {
    reg_bits: 7,
    val_bits: 9,
    max_register: WM8955_MAX_REGISTER,
    volatile_reg: Some(wm8955_volatile),
    writeable_reg: Some(wm8955_writeable),
    cache_type: REGCACHE_MAPLE,
    reg_defaults: wm8955_reg_defaults.as_ptr(),
    num_reg_defaults: wm8955_reg_defaults.len() as c_uint,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn wm8955_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let wm8955: *mut wm8955_priv;
    let mut ret: c_int;

    wm8955 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<wm8955_priv>(), GFP_KERNEL) as *mut wm8955_priv;
    if wm8955.is_null() {
        return -ENOMEM;
    }

    (*wm8955).regmap = devm_regmap_init_i2c(i2c, &wm8955_regmap);
    if IS_ERR((*wm8955).regmap as *const c_void) {
        ret = PTR_ERR((*wm8955).regmap as *const c_void) as c_int;
        dev_err(&mut (*i2c).dev, c"Failed to allocate register map: %d\n".as_ptr(), ret);
        return ret;
    }

    i2c_set_clientdata(i2c, wm8955 as *mut c_void);

    ret = devm_snd_soc_register_component(&mut (*i2c).dev,
        &soc_component_dev_wm8955, &mut wm8955_dai, 1);

    ret
}

static wm8955_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: *b"wm8955\0", driver_data: 0 },
    unsafe { core::mem::zeroed() },
];
MODULE_DEVICE_TABLE!(i2c, wm8955_i2c_id);

static mut wm8955_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"wm8955".as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(wm8955_i2c_probe),
    id_table: wm8955_i2c_id.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};

module_i2c_driver!(wm8955_i2c_driver);

MODULE_DESCRIPTION!("ASoC WM8955 driver");
MODULE_AUTHOR!("Mark Brown <broonie@opensource.wolfsonmicro.com>");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
