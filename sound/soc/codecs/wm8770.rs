// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8770.c  --  WM8770 ALSA SoC Audio driver
 *
 * Copyright 2010 Wolfson Microelectronics plc
 *
 * Author: Dimitris Papastamos <dp@opensource.wolfsonmicro.com>
 */

// Translated from Linux C implementation source. Kernel/ASoC/SPI/regmap
// declarations and macros are expected from the surrounding translated tree.

pub const WM8770_NUM_SUPPLIES: usize = 3;

static wm8770_supply_names: [*const c_char; WM8770_NUM_SUPPLIES] = [
    c"AVDD1".as_ptr(),
    c"AVDD2".as_ptr(),
    c"DVDD".as_ptr(),
];

static wm8770_reg_defaults: [reg_default; 31] = [
    reg_default { reg: 0, def: 0x7f },
    reg_default { reg: 1, def: 0x7f },
    reg_default { reg: 2, def: 0x7f },
    reg_default { reg: 3, def: 0x7f },
    reg_default { reg: 4, def: 0x7f },
    reg_default { reg: 5, def: 0x7f },
    reg_default { reg: 6, def: 0x7f },
    reg_default { reg: 7, def: 0x7f },
    reg_default { reg: 8, def: 0x7f },
    reg_default { reg: 9, def: 0xff },
    reg_default { reg: 10, def: 0xff },
    reg_default { reg: 11, def: 0xff },
    reg_default { reg: 12, def: 0xff },
    reg_default { reg: 13, def: 0xff },
    reg_default { reg: 14, def: 0xff },
    reg_default { reg: 15, def: 0xff },
    reg_default { reg: 16, def: 0xff },
    reg_default { reg: 17, def: 0xff },
    reg_default { reg: 18, def: 0 },
    reg_default { reg: 19, def: 0x90 },
    reg_default { reg: 20, def: 0 },
    reg_default { reg: 21, def: 0 },
    reg_default { reg: 22, def: 0x22 },
    reg_default { reg: 23, def: 0x22 },
    reg_default { reg: 24, def: 0x3e },
    reg_default { reg: 25, def: 0xc },
    reg_default { reg: 26, def: 0xc },
    reg_default { reg: 27, def: 0x100 },
    reg_default { reg: 28, def: 0x189 },
    reg_default { reg: 29, def: 0x189 },
    reg_default { reg: 30, def: 0x8770 },
];

unsafe extern "C" fn wm8770_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        WM8770_RESET => true,
        _ => false,
    }
}

#[repr(C)]
pub struct wm8770_priv {
    regmap: *mut regmap,
    supplies: [regulator_bulk_data; WM8770_NUM_SUPPLIES],
    disable_nb: [notifier_block; WM8770_NUM_SUPPLIES],
    component: *mut snd_soc_component,
    sysclk: c_int,
}

/*
 * We can't use the same notifier block for more than one supply and
 * there's no way I can see to get from a callback to the caller
 * except container_of().
 */
macro_rules! WM8770_REGULATOR_EVENT {
    ($n:expr, $name:ident) => {
        unsafe extern "C" fn $name(
            nb: *mut notifier_block,
            event: c_ulong,
            _data: *mut c_void,
        ) -> c_int {
            let wm8770: *mut wm8770_priv =
                container_of!(nb, wm8770_priv, disable_nb[$n]);
            if event & REGULATOR_EVENT_DISABLE != 0 {
                regcache_mark_dirty((*wm8770).regmap);
            }
            0
        }
    };
}

WM8770_REGULATOR_EVENT!(0, wm8770_regulator_event_0);
WM8770_REGULATOR_EVENT!(1, wm8770_regulator_event_1);
WM8770_REGULATOR_EVENT!(2, wm8770_regulator_event_2);

static adc_tlv: TlvDbScale = DECLARE_TLV_DB_SCALE!(-1200, 100, 0);
static dac_dig_tlv: TlvDbScale = DECLARE_TLV_DB_SCALE!(-12750, 50, 1);
static dac_alg_tlv: TlvDbScale = DECLARE_TLV_DB_SCALE!(-12700, 100, 1);

static dac_phase_text: [[*const c_char; 2]; 4] = [
    [c"DAC1 Normal".as_ptr(), c"DAC1 Inverted".as_ptr()],
    [c"DAC2 Normal".as_ptr(), c"DAC2 Inverted".as_ptr()],
    [c"DAC3 Normal".as_ptr(), c"DAC3 Inverted".as_ptr()],
    [c"DAC4 Normal".as_ptr(), c"DAC4 Inverted".as_ptr()],
];

static dac_phase: [soc_enum; 4] = [
    SOC_ENUM_DOUBLE!(WM8770_DACPHASE, 0, 1, 2, dac_phase_text[0]),
    SOC_ENUM_DOUBLE!(WM8770_DACPHASE, 2, 3, 2, dac_phase_text[1]),
    SOC_ENUM_DOUBLE!(WM8770_DACPHASE, 4, 5, 2, dac_phase_text[2]),
    SOC_ENUM_DOUBLE!(WM8770_DACPHASE, 6, 7, 2, dac_phase_text[3]),
];

static wm8770_snd_controls: [snd_kcontrol_new; 29] = [
    /* global DAC playback controls */
    SOC_SINGLE_TLV!(c"DAC Playback Volume", WM8770_MSDIGVOL, 0, 255, 0, dac_dig_tlv),
    SOC_SINGLE!(c"DAC Playback Switch", WM8770_DACMUTE, 4, 1, 1),
    SOC_SINGLE!(c"DAC Playback ZC Switch", WM8770_DACCTRL1, 0, 1, 0),
    /* global VOUT playback controls */
    SOC_SINGLE_TLV!(c"VOUT Playback Volume", WM8770_MSALGVOL, 0, 127, 0, dac_alg_tlv),
    SOC_SINGLE!(c"VOUT Playback ZC Switch", WM8770_MSALGVOL, 7, 1, 0),
    /* VOUT1/2/3/4 specific controls */
    SOC_DOUBLE_R_TLV!(c"VOUT1 Playback Volume", WM8770_VOUT1LVOL, WM8770_VOUT1RVOL, 0, 127, 0, dac_alg_tlv),
    SOC_DOUBLE_R!(c"VOUT1 Playback ZC Switch", WM8770_VOUT1LVOL, WM8770_VOUT1RVOL, 7, 1, 0),
    SOC_DOUBLE_R_TLV!(c"VOUT2 Playback Volume", WM8770_VOUT2LVOL, WM8770_VOUT2RVOL, 0, 127, 0, dac_alg_tlv),
    SOC_DOUBLE_R!(c"VOUT2 Playback ZC Switch", WM8770_VOUT2LVOL, WM8770_VOUT2RVOL, 7, 1, 0),
    SOC_DOUBLE_R_TLV!(c"VOUT3 Playback Volume", WM8770_VOUT3LVOL, WM8770_VOUT3RVOL, 0, 127, 0, dac_alg_tlv),
    SOC_DOUBLE_R!(c"VOUT3 Playback ZC Switch", WM8770_VOUT3LVOL, WM8770_VOUT3RVOL, 7, 1, 0),
    SOC_DOUBLE_R_TLV!(c"VOUT4 Playback Volume", WM8770_VOUT4LVOL, WM8770_VOUT4RVOL, 0, 127, 0, dac_alg_tlv),
    SOC_DOUBLE_R!(c"VOUT4 Playback ZC Switch", WM8770_VOUT4LVOL, WM8770_VOUT4RVOL, 7, 1, 0),
    /* DAC1/2/3/4 specific controls */
    SOC_DOUBLE_R_TLV!(c"DAC1 Playback Volume", WM8770_DAC1LVOL, WM8770_DAC1RVOL, 0, 255, 0, dac_dig_tlv),
    SOC_SINGLE!(c"DAC1 Deemphasis Switch", WM8770_DACCTRL2, 0, 1, 0),
    SOC_ENUM!(c"DAC1 Phase", dac_phase[0]),
    SOC_DOUBLE_R_TLV!(c"DAC2 Playback Volume", WM8770_DAC2LVOL, WM8770_DAC2RVOL, 0, 255, 0, dac_dig_tlv),
    SOC_SINGLE!(c"DAC2 Deemphasis Switch", WM8770_DACCTRL2, 1, 1, 0),
    SOC_ENUM!(c"DAC2 Phase", dac_phase[1]),
    SOC_DOUBLE_R_TLV!(c"DAC3 Playback Volume", WM8770_DAC3LVOL, WM8770_DAC3RVOL, 0, 255, 0, dac_dig_tlv),
    SOC_SINGLE!(c"DAC3 Deemphasis Switch", WM8770_DACCTRL2, 2, 1, 0),
    SOC_ENUM!(c"DAC3 Phase", dac_phase[2]),
    SOC_DOUBLE_R_TLV!(c"DAC4 Playback Volume", WM8770_DAC4LVOL, WM8770_DAC4RVOL, 0, 255, 0, dac_dig_tlv),
    SOC_SINGLE!(c"DAC4 Deemphasis Switch", WM8770_DACCTRL2, 3, 1, 0),
    SOC_ENUM!(c"DAC4 Phase", dac_phase[3]),
    /* ADC specific controls */
    SOC_DOUBLE_R_TLV!(c"Capture Volume", WM8770_ADCLCTRL, WM8770_ADCRCTRL, 0, 31, 0, adc_tlv),
    SOC_DOUBLE_R!(c"Capture Switch", WM8770_ADCLCTRL, WM8770_ADCRCTRL, 5, 1, 1),
    /* other controls */
    SOC_SINGLE!(c"ADC 128x Oversampling Switch", WM8770_MSTRCTRL, 3, 1, 0),
    SOC_SINGLE!(c"ADC Highpass Filter Switch", WM8770_IFACECTRL, 8, 1, 1),
];

static ain_text: [*const c_char; 8] = [
    c"AIN1".as_ptr(), c"AIN2".as_ptr(), c"AIN3".as_ptr(), c"AIN4".as_ptr(),
    c"AIN5".as_ptr(), c"AIN6".as_ptr(), c"AIN7".as_ptr(), c"AIN8".as_ptr(),
];

static ain_enum: soc_enum = SOC_ENUM_DOUBLE_DECL!(WM8770_ADCMUX, 0, 4, ain_text);

static ain_mux: snd_kcontrol_new = SOC_DAPM_ENUM!(c"Capture Mux", ain_enum);

static vout1_mix_controls: [snd_kcontrol_new; 3] = [
    SOC_DAPM_SINGLE!(c"DAC1 Switch", WM8770_OUTMUX1, 0, 1, 0),
    SOC_DAPM_SINGLE!(c"AUX1 Switch", WM8770_OUTMUX1, 1, 1, 0),
    SOC_DAPM_SINGLE!(c"Bypass Switch", WM8770_OUTMUX1, 2, 1, 0),
];

static vout2_mix_controls: [snd_kcontrol_new; 3] = [
    SOC_DAPM_SINGLE!(c"DAC2 Switch", WM8770_OUTMUX1, 3, 1, 0),
    SOC_DAPM_SINGLE!(c"AUX2 Switch", WM8770_OUTMUX1, 4, 1, 0),
    SOC_DAPM_SINGLE!(c"Bypass Switch", WM8770_OUTMUX1, 5, 1, 0),
];

static vout3_mix_controls: [snd_kcontrol_new; 3] = [
    SOC_DAPM_SINGLE!(c"DAC3 Switch", WM8770_OUTMUX2, 0, 1, 0),
    SOC_DAPM_SINGLE!(c"AUX3 Switch", WM8770_OUTMUX2, 1, 1, 0),
    SOC_DAPM_SINGLE!(c"Bypass Switch", WM8770_OUTMUX2, 2, 1, 0),
];

static vout4_mix_controls: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE!(c"DAC4 Switch", WM8770_OUTMUX2, 3, 1, 0),
    SOC_DAPM_SINGLE!(c"Bypass Switch", WM8770_OUTMUX2, 4, 1, 0),
];

static wm8770_dapm_widgets: [snd_soc_dapm_widget; 27] = [
    SND_SOC_DAPM_INPUT!(c"AUX1"),
    SND_SOC_DAPM_INPUT!(c"AUX2"),
    SND_SOC_DAPM_INPUT!(c"AUX3"),
    SND_SOC_DAPM_INPUT!(c"AIN1"),
    SND_SOC_DAPM_INPUT!(c"AIN2"),
    SND_SOC_DAPM_INPUT!(c"AIN3"),
    SND_SOC_DAPM_INPUT!(c"AIN4"),
    SND_SOC_DAPM_INPUT!(c"AIN5"),
    SND_SOC_DAPM_INPUT!(c"AIN6"),
    SND_SOC_DAPM_INPUT!(c"AIN7"),
    SND_SOC_DAPM_INPUT!(c"AIN8"),
    SND_SOC_DAPM_MUX!(c"Capture Mux", WM8770_ADCMUX, 8, 1, &ain_mux),
    SND_SOC_DAPM_ADC!(c"ADC", c"Capture", WM8770_PWDNCTRL, 1, 1),
    SND_SOC_DAPM_DAC!(c"DAC1", c"Playback", WM8770_PWDNCTRL, 2, 1),
    SND_SOC_DAPM_DAC!(c"DAC2", c"Playback", WM8770_PWDNCTRL, 3, 1),
    SND_SOC_DAPM_DAC!(c"DAC3", c"Playback", WM8770_PWDNCTRL, 4, 1),
    SND_SOC_DAPM_DAC!(c"DAC4", c"Playback", WM8770_PWDNCTRL, 5, 1),
    SND_SOC_DAPM_SUPPLY!(c"VOUT12 Supply", SND_SOC_NOPM, 0, 0, vout12supply_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY!(c"VOUT34 Supply", SND_SOC_NOPM, 0, 0, vout34supply_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MIXER!(c"VOUT1 Mixer", SND_SOC_NOPM, 0, 0, vout1_mix_controls, ARRAY_SIZE!(vout1_mix_controls)),
    SND_SOC_DAPM_MIXER!(c"VOUT2 Mixer", SND_SOC_NOPM, 0, 0, vout2_mix_controls, ARRAY_SIZE!(vout2_mix_controls)),
    SND_SOC_DAPM_MIXER!(c"VOUT3 Mixer", SND_SOC_NOPM, 0, 0, vout3_mix_controls, ARRAY_SIZE!(vout3_mix_controls)),
    SND_SOC_DAPM_MIXER!(c"VOUT4 Mixer", SND_SOC_NOPM, 0, 0, vout4_mix_controls, ARRAY_SIZE!(vout4_mix_controls)),
    SND_SOC_DAPM_OUTPUT!(c"VOUT1"),
    SND_SOC_DAPM_OUTPUT!(c"VOUT2"),
    SND_SOC_DAPM_OUTPUT!(c"VOUT3"),
    SND_SOC_DAPM_OUTPUT!(c"VOUT4"),
];

static wm8770_intercon: [snd_soc_dapm_route; 29] = [
    snd_soc_dapm_route { sink: c"Capture Mux".as_ptr(), control: c"AIN1".as_ptr(), source: c"AIN1".as_ptr() },
    snd_soc_dapm_route { sink: c"Capture Mux".as_ptr(), control: c"AIN2".as_ptr(), source: c"AIN2".as_ptr() },
    snd_soc_dapm_route { sink: c"Capture Mux".as_ptr(), control: c"AIN3".as_ptr(), source: c"AIN3".as_ptr() },
    snd_soc_dapm_route { sink: c"Capture Mux".as_ptr(), control: c"AIN4".as_ptr(), source: c"AIN4".as_ptr() },
    snd_soc_dapm_route { sink: c"Capture Mux".as_ptr(), control: c"AIN5".as_ptr(), source: c"AIN5".as_ptr() },
    snd_soc_dapm_route { sink: c"Capture Mux".as_ptr(), control: c"AIN6".as_ptr(), source: c"AIN6".as_ptr() },
    snd_soc_dapm_route { sink: c"Capture Mux".as_ptr(), control: c"AIN7".as_ptr(), source: c"AIN7".as_ptr() },
    snd_soc_dapm_route { sink: c"Capture Mux".as_ptr(), control: c"AIN8".as_ptr(), source: c"AIN8".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC".as_ptr(), control: core::ptr::null(), source: c"Capture Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT1 Mixer".as_ptr(), control: core::ptr::null(), source: c"VOUT12 Supply".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT1 Mixer".as_ptr(), control: c"DAC1 Switch".as_ptr(), source: c"DAC1".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT1 Mixer".as_ptr(), control: c"AUX1 Switch".as_ptr(), source: c"AUX1".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT1 Mixer".as_ptr(), control: c"Bypass Switch".as_ptr(), source: c"Capture Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT2 Mixer".as_ptr(), control: core::ptr::null(), source: c"VOUT12 Supply".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT2 Mixer".as_ptr(), control: c"DAC2 Switch".as_ptr(), source: c"DAC2".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT2 Mixer".as_ptr(), control: c"AUX2 Switch".as_ptr(), source: c"AUX2".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT2 Mixer".as_ptr(), control: c"Bypass Switch".as_ptr(), source: c"Capture Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT3 Mixer".as_ptr(), control: core::ptr::null(), source: c"VOUT34 Supply".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT3 Mixer".as_ptr(), control: c"DAC3 Switch".as_ptr(), source: c"DAC3".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT3 Mixer".as_ptr(), control: c"AUX3 Switch".as_ptr(), source: c"AUX3".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT3 Mixer".as_ptr(), control: c"Bypass Switch".as_ptr(), source: c"Capture Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT4 Mixer".as_ptr(), control: core::ptr::null(), source: c"VOUT34 Supply".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT4 Mixer".as_ptr(), control: c"DAC4 Switch".as_ptr(), source: c"DAC4".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT4 Mixer".as_ptr(), control: c"Bypass Switch".as_ptr(), source: c"Capture Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT1".as_ptr(), control: core::ptr::null(), source: c"VOUT1 Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT2".as_ptr(), control: core::ptr::null(), source: c"VOUT2 Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT3".as_ptr(), control: core::ptr::null(), source: c"VOUT3 Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT4".as_ptr(), control: core::ptr::null(), source: c"VOUT4 Mixer".as_ptr() },
];

unsafe extern "C" fn vout12supply_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            snd_soc_component_update_bits(component, WM8770_OUTMUX1, 0x180, 0);
        }
        SND_SOC_DAPM_POST_PMD => {
            snd_soc_component_update_bits(component, WM8770_OUTMUX1, 0x180, 0x180);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn vout34supply_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            snd_soc_component_update_bits(component, WM8770_OUTMUX2, 0x180, 0);
        }
        SND_SOC_DAPM_POST_PMD => {
            snd_soc_component_update_bits(component, WM8770_OUTMUX2, 0x180, 0x180);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn wm8770_reset(component: *mut snd_soc_component) -> c_int {
    snd_soc_component_write(component, WM8770_RESET, 0)
}

unsafe extern "C" fn wm8770_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component: *mut snd_soc_component;
    let mut iface: c_int;
    let master: c_int;

    component = (*dai).component;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => {
            master = 0x100;
        }
        SND_SOC_DAIFMT_CBC_CFC => {
            master = 0;
        }
        _ => {
            return -EINVAL;
        }
    }

    iface = 0;
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            iface |= 0x2;
        }
        SND_SOC_DAIFMT_RIGHT_J => {}
        SND_SOC_DAIFMT_LEFT_J => {
            iface |= 0x1;
        }
        _ => {
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_IF => {
            iface |= 0xc;
        }
        SND_SOC_DAIFMT_IB_NF => {
            iface |= 0x8;
        }
        SND_SOC_DAIFMT_NB_IF => {
            iface |= 0x4;
        }
        _ => {
            return -EINVAL;
        }
    }

    snd_soc_component_update_bits(component, WM8770_IFACECTRL, 0xf, iface as c_uint);
    snd_soc_component_update_bits(component, WM8770_MSTRCTRL, 0x100, master as c_uint);

    0
}

static mclk_ratios: [c_int; 6] = [128, 192, 256, 384, 512, 768];

unsafe extern "C" fn wm8770_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component;
    let wm8770: *mut wm8770_priv;
    let mut i: usize;
    let mut iface: c_int;
    let shift: c_int;
    let mut ratio: c_int;

    component = (*dai).component;
    wm8770 = snd_soc_component_get_drvdata(component) as *mut wm8770_priv;

    iface = 0;
    match params_width(params) {
        16 => {}
        20 => {
            iface |= 0x10;
        }
        24 => {
            iface |= 0x20;
        }
        32 => {
            iface |= 0x30;
        }
        _ => {}
    }

    match (*substream).stream {
        SNDRV_PCM_STREAM_PLAYBACK => {
            i = 0;
            shift = 4;
        }
        SNDRV_PCM_STREAM_CAPTURE => {
            i = 2;
            shift = 0;
        }
        _ => {
            return -EINVAL;
        }
    }

    /* Only need to set MCLK/LRCLK ratio if we're master */
    if snd_soc_component_read(component, WM8770_MSTRCTRL) & 0x100 != 0 {
        while i < ARRAY_SIZE!(mclk_ratios) {
            ratio = (*wm8770).sysclk / params_rate(params);
            if ratio == mclk_ratios[i] {
                break;
            }
            i += 1;
        }

        if i == ARRAY_SIZE!(mclk_ratios) {
            dev_err!(
                (*component).dev,
                c"Unable to configure MCLK ratio %d/%d\n",
                (*wm8770).sysclk,
                params_rate(params)
            );
            return -EINVAL;
        }

        dev_dbg!((*component).dev, c"MCLK is %dfs\n", mclk_ratios[i]);

        snd_soc_component_update_bits(
            component,
            WM8770_MSTRCTRL,
            (0x7 << shift) as c_uint,
            ((i as c_int) << shift) as c_uint,
        );
    }

    snd_soc_component_update_bits(component, WM8770_IFACECTRL, 0x30, iface as c_uint);

    0
}

unsafe extern "C" fn wm8770_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let component: *mut snd_soc_component;

    component = (*dai).component;
    snd_soc_component_update_bits(
        component,
        WM8770_DACMUTE,
        0x10,
        (((mute != 0) as c_int) << 4) as c_uint,
    )
}

unsafe extern "C" fn wm8770_set_sysclk(
    dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component: *mut snd_soc_component;
    let wm8770: *mut wm8770_priv;

    component = (*dai).component;
    wm8770 = snd_soc_component_get_drvdata(component) as *mut wm8770_priv;
    (*wm8770).sysclk = freq as c_int;
    0
}

unsafe extern "C" fn wm8770_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let mut ret: c_int;
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);
    let wm8770: *mut wm8770_priv;

    wm8770 = snd_soc_component_get_drvdata(component) as *mut wm8770_priv;

    match level {
        SND_SOC_BIAS_ON => {}
        SND_SOC_BIAS_PREPARE => {}
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                ret = regulator_bulk_enable(ARRAY_SIZE!((*wm8770).supplies), (*wm8770).supplies.as_mut_ptr());
                if ret != 0 {
                    dev_err!((*component).dev, c"Failed to enable supplies: %d\n", ret);
                    return ret;
                }

                regcache_sync((*wm8770).regmap);

                /* global powerup */
                snd_soc_component_write(component, WM8770_PWDNCTRL, 0);
            }
        }
        SND_SOC_BIAS_OFF => {
            /* global powerdown */
            snd_soc_component_write(component, WM8770_PWDNCTRL, 1);
            regulator_bulk_disable(ARRAY_SIZE!((*wm8770).supplies), (*wm8770).supplies.as_mut_ptr());
        }
    }

    0
}

pub const WM8770_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static wm8770_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    mute_stream: Some(wm8770_mute),
    hw_params: Some(wm8770_hw_params),
    set_fmt: Some(wm8770_set_fmt),
    set_sysclk: Some(wm8770_set_sysclk),
    no_capture_mute: 1,
};

static mut wm8770_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"wm8770-hifi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: WM8770_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: WM8770_FORMATS,
    },
    ops: &wm8770_dai_ops,
    symmetric_rate: 1,
};

unsafe extern "C" fn wm8770_probe(component: *mut snd_soc_component) -> c_int {
    let wm8770: *mut wm8770_priv;
    let mut ret: c_int;

    wm8770 = snd_soc_component_get_drvdata(component) as *mut wm8770_priv;
    (*wm8770).component = component;

    ret = regulator_bulk_enable(ARRAY_SIZE!((*wm8770).supplies), (*wm8770).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err!((*component).dev, c"Failed to enable supplies: %d\n", ret);
        return ret;
    }

    ret = wm8770_reset(component);
    if ret < 0 {
        dev_err!((*component).dev, c"Failed to issue reset: %d\n", ret);
        regulator_bulk_disable(ARRAY_SIZE!((*wm8770).supplies), (*wm8770).supplies.as_mut_ptr());
        return ret;
    }

    /* latch the volume update bits */
    snd_soc_component_update_bits(component, WM8770_MSDIGVOL, 0x100, 0x100);
    snd_soc_component_update_bits(component, WM8770_MSALGVOL, 0x100, 0x100);
    snd_soc_component_update_bits(component, WM8770_VOUT1RVOL, 0x100, 0x100);
    snd_soc_component_update_bits(component, WM8770_VOUT2RVOL, 0x100, 0x100);
    snd_soc_component_update_bits(component, WM8770_VOUT3RVOL, 0x100, 0x100);
    snd_soc_component_update_bits(component, WM8770_VOUT4RVOL, 0x100, 0x100);
    snd_soc_component_update_bits(component, WM8770_DAC1RVOL, 0x100, 0x100);
    snd_soc_component_update_bits(component, WM8770_DAC2RVOL, 0x100, 0x100);
    snd_soc_component_update_bits(component, WM8770_DAC3RVOL, 0x100, 0x100);
    snd_soc_component_update_bits(component, WM8770_DAC4RVOL, 0x100, 0x100);

    /* mute all DACs */
    snd_soc_component_update_bits(component, WM8770_DACMUTE, 0x10, 0x10);

    regulator_bulk_disable(ARRAY_SIZE!((*wm8770).supplies), (*wm8770).supplies.as_mut_ptr());
    ret
}

static soc_component_dev_wm8770: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm8770_probe),
    set_bias_level: Some(wm8770_set_bias_level),
    controls: wm8770_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(wm8770_snd_controls),
    dapm_widgets: wm8770_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(wm8770_dapm_widgets),
    dapm_routes: wm8770_intercon.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(wm8770_intercon),
    use_pmdown_time: 1,
    endianness: 1,
};

static wm8770_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c"wlf,wm8770".as_ptr() },
    of_device_id::zeroed(),
];
MODULE_DEVICE_TABLE!(of, wm8770_of_match);

static wm8770_regmap: regmap_config = regmap_config {
    reg_bits: 7,
    val_bits: 9,
    max_register: WM8770_RESET,
    reg_defaults: wm8770_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE!(wm8770_reg_defaults),
    cache_type: REGCACHE_MAPLE,
    volatile_reg: Some(wm8770_volatile_reg),
};

unsafe extern "C" fn wm8770_spi_probe(spi: *mut spi_device) -> c_int {
    let wm8770: *mut wm8770_priv;
    let mut ret: c_int;
    let mut i: usize;

    wm8770 = devm_kzalloc(
        &mut (*spi).dev,
        core::mem::size_of::<wm8770_priv>(),
        GFP_KERNEL,
    ) as *mut wm8770_priv;
    if wm8770.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < ARRAY_SIZE!((*wm8770).supplies) {
        (*wm8770).supplies[i].supply = wm8770_supply_names[i];
        i += 1;
    }

    ret = devm_regulator_bulk_get(
        &mut (*spi).dev,
        ARRAY_SIZE!((*wm8770).supplies),
        (*wm8770).supplies.as_mut_ptr(),
    );
    if ret != 0 {
        dev_err!(&mut (*spi).dev, c"Failed to request supplies: %d\n", ret);
        return ret;
    }

    (*wm8770).disable_nb[0].notifier_call = Some(wm8770_regulator_event_0);
    (*wm8770).disable_nb[1].notifier_call = Some(wm8770_regulator_event_1);
    (*wm8770).disable_nb[2].notifier_call = Some(wm8770_regulator_event_2);

    /* This should really be moved into the regulator core */
    i = 0;
    while i < ARRAY_SIZE!((*wm8770).supplies) {
        ret = devm_regulator_register_notifier(
            (*wm8770).supplies[i].consumer,
            &mut (*wm8770).disable_nb[i],
        );
        if ret != 0 {
            dev_err!(
                &mut (*spi).dev,
                c"Failed to register regulator notifier: %d\n",
                ret
            );
        }
        i += 1;
    }

    (*wm8770).regmap = devm_regmap_init_spi(spi, &wm8770_regmap);
    if IS_ERR((*wm8770).regmap as *const c_void) {
        return PTR_ERR((*wm8770).regmap as *const c_void);
    }

    spi_set_drvdata(spi, wm8770 as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*spi).dev,
        &soc_component_dev_wm8770,
        &raw mut wm8770_dai,
        1,
    );

    ret
}

static mut wm8770_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: c"wm8770".as_ptr(),
        of_match_table: wm8770_of_match.as_ptr(),
    },
    probe: Some(wm8770_spi_probe),
};

module_spi_driver!(wm8770_spi_driver);

MODULE_DESCRIPTION!(c"ASoC WM8770 driver");
MODULE_AUTHOR!(c"Dimitris Papastamos <dp@opensource.wolfsonmicro.com>");
MODULE_LICENSE!(c"GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
