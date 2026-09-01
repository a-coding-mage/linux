// SPDX-License-Identifier: GPL-2.0
//
// TSE-850 audio - ASoC driver for the Axentia TSE-850 with a PCM5142 codec
//
// Copyright (C) 2016 Axentia Technologies AB
//
// Author: Peter Rosin <peda@axentia.se>
//
//               loop1 relays
//   IN1 +---o  +------------+  o---+ OUT1
//            \                /
//             +              +
//             |   /          |
//             +--o  +--.     |
//             |  add   |     |
//             |        V     |
//             |      .---.   |
//   DAC +----------->|Sum|---+
//             |      '---'   |
//             |              |
//             +              +
//
//   IN2 +---o--+------------+--o---+ OUT2
//               loop2 relays
//
// The 'loop1' gpio pin controls two relays, which are either in loop
// position, meaning that input and output are directly connected, or
// they are in mixer position, meaning that the signal is passed through
// the 'Sum' mixer. Similarly for 'loop2'.
//
// In the above, the 'loop1' relays are inactive, thus feeding IN1 to the
// mixer (if 'add' is active) and feeding the mixer output to OUT1. The
// 'loop2' relays are active, short-cutting the TSE-850 from channel 2.
// IN1, IN2, OUT1 and OUT2 are TSE-850 connectors and DAC is the PCB name
// of the (filtered) output from the PCM5142 codec.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

// C includes translated as external dependency intent:
// <linux/clk.h>, <linux/gpio/consumer.h>, <linux/module.h>,
// <linux/of.h>, <linux/regulator/consumer.h>,
// <sound/soc.h>, <sound/pcm_params.h>.

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_HIGH: c_uint = 1;
const SND_SOC_NOPM: c_int = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_NB_NF: c_uint = 2;
const SND_SOC_DAIFMT_CBP_CFC: c_uint = 4;

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regulator {
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
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub dai_fmt: c_uint,
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
    pub owner: *mut c_void,
    pub dev: *mut device,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
    pub fully_routed: bool,
}

#[repr(C)]
pub struct soc_enum {
    pub items: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub enumerated: snd_ctl_elem_value_enumerated,
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 4],
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 4],
}

#[allow(non_camel_case_types)]
type c_long = isize;

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct tse850_priv {
    pub add: *mut gpio_desc,
    pub loop1: *mut gpio_desc,
    pub loop2: *mut gpio_desc,

    pub ana: *mut regulator,

    pub add_cache: c_int,
    pub loop1_cache: c_int,
    pub loop2_cache: c_int,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;

    fn snd_soc_dapm_kcontrol_to_dapm(kctrl: *mut snd_kcontrol) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_dapm_put_enum_double(
        kctrl: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
    fn snd_soc_dapm_mixer_update_power(
        dapm: *mut snd_soc_dapm_context,
        kctrl: *mut snd_kcontrol,
        connect: c_int,
        update: *mut c_void,
    );
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn regulator_get_voltage(regulator: *mut regulator) -> c_int;
    fn regulator_set_voltage(regulator: *mut regulator, min_uV: c_uint, max_uV: c_uint) -> c_int;
    fn regulator_enable(regulator: *mut regulator) -> c_int;
    fn regulator_disable(regulator: *mut regulator) -> c_int;
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_long, fmt: *const c_char, ...) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn devm_gpiod_get(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn devm_regulator_get(dev: *mut device, id: *const c_char) -> *mut regulator;
    fn snd_soc_register_card(card: *mut snd_soc_card) -> c_int;
    fn snd_soc_unregister_card(card: *mut snd_soc_card);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
}

unsafe extern "C" fn tse850_get_mux1(
    kctrl: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kctrl);
    let card = snd_soc_dapm_to_card(dapm);
    let tse850 = snd_soc_card_get_drvdata(card) as *mut tse850_priv;

    (*ucontrol).value.enumerated.item[0] = (*tse850).loop1_cache as c_uint;

    0
}

unsafe extern "C" fn tse850_put_mux1(
    kctrl: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kctrl);
    let card = snd_soc_dapm_to_card(dapm);
    let tse850 = snd_soc_card_get_drvdata(card) as *mut tse850_priv;
    let e = (*kctrl).private_value as *mut soc_enum;
    let val = (*ucontrol).value.enumerated.item[0];

    if val >= (*e).items {
        return -EINVAL;
    }

    gpiod_set_value_cansleep((*tse850).loop1, val as c_int);
    (*tse850).loop1_cache = val as c_int;

    snd_soc_dapm_put_enum_double(kctrl, ucontrol)
}

unsafe extern "C" fn tse850_get_mux2(
    kctrl: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kctrl);
    let card = snd_soc_dapm_to_card(dapm);
    let tse850 = snd_soc_card_get_drvdata(card) as *mut tse850_priv;

    (*ucontrol).value.enumerated.item[0] = (*tse850).loop2_cache as c_uint;

    0
}

unsafe extern "C" fn tse850_put_mux2(
    kctrl: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kctrl);
    let card = snd_soc_dapm_to_card(dapm);
    let tse850 = snd_soc_card_get_drvdata(card) as *mut tse850_priv;
    let e = (*kctrl).private_value as *mut soc_enum;
    let val = (*ucontrol).value.enumerated.item[0];

    if val >= (*e).items {
        return -EINVAL;
    }

    gpiod_set_value_cansleep((*tse850).loop2, val as c_int);
    (*tse850).loop2_cache = val as c_int;

    snd_soc_dapm_put_enum_double(kctrl, ucontrol)
}

unsafe extern "C" fn tse850_get_mix(
    kctrl: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kctrl);
    let card = snd_soc_dapm_to_card(dapm);
    let tse850 = snd_soc_card_get_drvdata(card) as *mut tse850_priv;

    (*ucontrol).value.enumerated.item[0] = (*tse850).add_cache as c_uint;

    0
}

unsafe extern "C" fn tse850_put_mix(
    kctrl: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kctrl);
    let card = snd_soc_dapm_to_card(dapm);
    let tse850 = snd_soc_card_get_drvdata(card) as *mut tse850_priv;
    let connect = ((*ucontrol).value.integer.value[0] != 0) as c_int;

    if (*tse850).add_cache == connect {
        return 0;
    }

    /*
     * Hmmm, this gpiod_set_value_cansleep call should probably happen
     * inside snd_soc_dapm_mixer_update_power in the loop.
     */
    gpiod_set_value_cansleep((*tse850).add, connect);
    (*tse850).add_cache = connect;

    snd_soc_dapm_mixer_update_power(dapm, kctrl, connect, ptr::null_mut());
    1
}

unsafe extern "C" fn tse850_get_ana(
    kctrl: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kctrl);
    let card = snd_soc_dapm_to_card(dapm);
    let tse850 = snd_soc_card_get_drvdata(card) as *mut tse850_priv;
    let mut ret: c_int;

    ret = regulator_get_voltage((*tse850).ana);
    if ret < 0 {
        return ret;
    }

    /*
     * Map regulator output values like so:
     *      -11.5V to "Low" (enum 0)
     * 11.5V-12.5V to "12V" (enum 1)
     * 12.5V-13.5V to "13V" (enum 2)
     *     ...
     * 18.5V-19.5V to "19V" (enum 8)
     * 19.5V-      to "20V" (enum 9)
     */
    if ret < 11000000 {
        ret = 11000000;
    } else if ret > 20000000 {
        ret = 20000000;
    }
    ret -= 11000000;
    ret = (ret + 500000) / 1000000;

    (*ucontrol).value.enumerated.item[0] = ret as c_uint;

    0
}

unsafe extern "C" fn tse850_put_ana(
    kctrl: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kctrl);
    let card = snd_soc_dapm_to_card(dapm);
    let tse850 = snd_soc_card_get_drvdata(card) as *mut tse850_priv;
    let e = (*kctrl).private_value as *mut soc_enum;
    let mut uV = (*ucontrol).value.enumerated.item[0];
    let ret: c_int;

    if uV >= (*e).items {
        return -EINVAL;
    }

    /*
     * Map enum zero (Low) to 2 volts on the regulator, do this since
     * the ana regulator is supplied by the system 12V voltage and
     * requesting anything below the system voltage causes the system
     * voltage to be passed through the regulator. Also, the ana
     * regulator induces noise when requesting voltages near the
     * system voltage. So, by mapping Low to 2V, that noise is
     * eliminated when all that is needed is 12V (the system voltage).
     */
    if uV != 0 {
        uV = 11000000 + (1000000 * uV);
    } else {
        uV = 2000000;
    }

    ret = regulator_set_voltage((*tse850).ana, uV, uV);
    if ret < 0 {
        return ret;
    }

    snd_soc_dapm_put_enum_double(kctrl, ucontrol)
}

static MUX_TEXT_0: &[u8] = b"Mixer\0";
static MUX_TEXT_1: &[u8] = b"Loop\0";
static MUX_TEXT: [*const c_char; 2] = [
    MUX_TEXT_0.as_ptr() as *const c_char,
    MUX_TEXT_1.as_ptr() as *const c_char,
];

static MUX_ENUM: soc_enum = SOC_ENUM_SINGLE!(SND_SOC_NOPM, 0, MUX_TEXT.len(), MUX_TEXT.as_ptr());

static MUX1: snd_kcontrol_new =
    SOC_DAPM_ENUM_EXT!(c"MUX1".as_ptr(), MUX_ENUM, tse850_get_mux1, tse850_put_mux1);

static MUX2: snd_kcontrol_new =
    SOC_DAPM_ENUM_EXT!(c"MUX2".as_ptr(), MUX_ENUM, tse850_get_mux2, tse850_put_mux2);

static MIX: [snd_kcontrol_new; 1] = [SOC_SINGLE_EXT!(
    c"IN Switch".as_ptr(),
    SND_SOC_NOPM,
    0,
    1,
    0,
    tse850_get_mix,
    tse850_put_mix
)];

static ANA_TEXT_0: &[u8] = b"Low\0";
static ANA_TEXT_1: &[u8] = b"12V\0";
static ANA_TEXT_2: &[u8] = b"13V\0";
static ANA_TEXT_3: &[u8] = b"14V\0";
static ANA_TEXT_4: &[u8] = b"15V\0";
static ANA_TEXT_5: &[u8] = b"16V\0";
static ANA_TEXT_6: &[u8] = b"17V\0";
static ANA_TEXT_7: &[u8] = b"18V\0";
static ANA_TEXT_8: &[u8] = b"19V\0";
static ANA_TEXT_9: &[u8] = b"20V\0";
static ANA_TEXT: [*const c_char; 10] = [
    ANA_TEXT_0.as_ptr() as *const c_char,
    ANA_TEXT_1.as_ptr() as *const c_char,
    ANA_TEXT_2.as_ptr() as *const c_char,
    ANA_TEXT_3.as_ptr() as *const c_char,
    ANA_TEXT_4.as_ptr() as *const c_char,
    ANA_TEXT_5.as_ptr() as *const c_char,
    ANA_TEXT_6.as_ptr() as *const c_char,
    ANA_TEXT_7.as_ptr() as *const c_char,
    ANA_TEXT_8.as_ptr() as *const c_char,
    ANA_TEXT_9.as_ptr() as *const c_char,
];

static ANA_ENUM: soc_enum = SOC_ENUM_SINGLE!(SND_SOC_NOPM, 0, ANA_TEXT.len(), ANA_TEXT.as_ptr());

static OUT: snd_kcontrol_new =
    SOC_DAPM_ENUM_EXT!(c"ANA".as_ptr(), ANA_ENUM, tse850_get_ana, tse850_put_ana);

static TSE850_DAPM_WIDGETS: [snd_soc_dapm_widget; 11] = [
    SND_SOC_DAPM_LINE!(c"OUT1".as_ptr(), ptr::null_mut()),
    SND_SOC_DAPM_LINE!(c"OUT2".as_ptr(), ptr::null_mut()),
    SND_SOC_DAPM_LINE!(c"IN1".as_ptr(), ptr::null_mut()),
    SND_SOC_DAPM_LINE!(c"IN2".as_ptr(), ptr::null_mut()),
    SND_SOC_DAPM_INPUT!(c"DAC".as_ptr()),
    SND_SOC_DAPM_AIF_IN!(c"AIFINL".as_ptr(), c"Playback".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!(c"AIFINR".as_ptr(), c"Playback".as_ptr(), 1, SND_SOC_NOPM, 0, 0),
    SOC_MIXER_ARRAY!(c"MIX".as_ptr(), SND_SOC_NOPM, 0, 0, MIX),
    SND_SOC_DAPM_MUX!(c"MUX1".as_ptr(), SND_SOC_NOPM, 0, 0, &MUX1),
    SND_SOC_DAPM_MUX!(c"MUX2".as_ptr(), SND_SOC_NOPM, 0, 0, &MUX2),
    SND_SOC_DAPM_OUT_DRV!(c"OUT".as_ptr(), SND_SOC_NOPM, 0, 0, &OUT, 1),
];

/*
 * These connections are not entirely correct, since both IN1 and IN2
 * are always fed to MIX (if the "IN switch" is set so), i.e. without
 * regard to the loop1 and loop2 relays that according to this only
 * control MUX1 and MUX2 but in fact also control how the input signals
 * are routed.
 * But, 1) I don't know how to do it right, and 2) it doesn't seem to
 * matter in practice since nothing is powered in those sections anyway.
 */
static TSE850_INTERCON: [snd_soc_dapm_route; 12] = [
    snd_soc_dapm_route {
        sink: c"OUT1".as_ptr(),
        control: ptr::null(),
        source: c"MUX1".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"OUT2".as_ptr(),
        control: ptr::null(),
        source: c"MUX2".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"MUX1".as_ptr(),
        control: c"Loop".as_ptr(),
        source: c"IN1".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"MUX1".as_ptr(),
        control: c"Mixer".as_ptr(),
        source: c"OUT".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"MUX2".as_ptr(),
        control: c"Loop".as_ptr(),
        source: c"IN2".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"MUX2".as_ptr(),
        control: c"Mixer".as_ptr(),
        source: c"OUT".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"OUT".as_ptr(),
        control: ptr::null(),
        source: c"MIX".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"MIX".as_ptr(),
        control: ptr::null(),
        source: c"DAC".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"MIX".as_ptr(),
        control: c"IN Switch".as_ptr(),
        source: c"IN1".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"MIX".as_ptr(),
        control: c"IN Switch".as_ptr(),
        source: c"IN2".as_ptr(),
    },
    /* connect board input to the codec left channel output pin */
    snd_soc_dapm_route {
        sink: c"DAC".as_ptr(),
        control: ptr::null(),
        source: c"OUTL".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: ptr::null(),
        control: ptr::null(),
        source: ptr::null(),
    },
];

static mut PCM_CPUS: [snd_soc_dai_link_component; 1] =
    [DAILINK_COMP_ARRAY!(COMP_EMPTY!())];
static mut PCM_CODECS: [snd_soc_dai_link_component; 1] =
    [DAILINK_COMP_ARRAY!(COMP_CODEC!(ptr::null(), c"pcm512x-hifi".as_ptr()))];
static mut PCM_PLATFORMS: [snd_soc_dai_link_component; 1] =
    [DAILINK_COMP_ARRAY!(COMP_EMPTY!())];

static mut TSE850_DAILINK: snd_soc_dai_link = snd_soc_dai_link {
    name: c"TSE-850".as_ptr(),
    stream_name: c"TSE-850-PCM".as_ptr(),
    dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFC,
    cpus: unsafe { PCM_CPUS.as_mut_ptr() },
    num_cpus: 1,
    codecs: unsafe { PCM_CODECS.as_mut_ptr() },
    num_codecs: 1,
    platforms: unsafe { PCM_PLATFORMS.as_mut_ptr() },
    num_platforms: 1,
};

static mut TSE850_CARD: snd_soc_card = snd_soc_card {
    name: c"TSE-850-ASoC".as_ptr(),
    owner: unsafe { THIS_MODULE },
    dev: ptr::null_mut(),
    dai_link: unsafe { &mut TSE850_DAILINK },
    num_links: 1,
    dapm_widgets: TSE850_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: TSE850_DAPM_WIDGETS.len() as c_int,
    dapm_routes: TSE850_INTERCON.as_ptr(),
    num_dapm_routes: TSE850_INTERCON.len() as c_int,
    fully_routed: true,
};

unsafe extern "C" fn tse850_dt_init(pdev: *mut platform_device) -> c_int {
    let np = (*pdev).dev.of_node;
    let codec_np: *mut device_node;
    let cpu_np: *mut device_node;
    let dailink = &mut TSE850_DAILINK as *mut snd_soc_dai_link;

    if np.is_null() {
        dev_err(&mut (*pdev).dev, c"only device tree supported\n".as_ptr());
        return -EINVAL;
    }

    cpu_np = of_parse_phandle(np, c"axentia,cpu-dai".as_ptr(), 0);
    if cpu_np.is_null() {
        dev_err(&mut (*pdev).dev, c"failed to get cpu dai\n".as_ptr());
        return -EINVAL;
    }
    (*(*dailink).cpus).of_node = cpu_np;
    (*(*dailink).platforms).of_node = cpu_np;
    of_node_put(cpu_np);

    codec_np = of_parse_phandle(np, c"axentia,audio-codec".as_ptr(), 0);
    if codec_np.is_null() {
        dev_err(&mut (*pdev).dev, c"failed to get codec info\n".as_ptr());
        return -EINVAL;
    }
    (*(*dailink).codecs).of_node = codec_np;
    of_node_put(codec_np);

    0
}

unsafe extern "C" fn tse850_probe(pdev: *mut platform_device) -> c_int {
    let card = &mut TSE850_CARD as *mut snd_soc_card;
    (*card).dev = &mut (*pdev).dev;
    let dev = (*card).dev;
    let tse850: *mut tse850_priv;
    let mut ret: c_int;

    tse850 = devm_kzalloc(dev, core::mem::size_of::<tse850_priv>(), GFP_KERNEL) as *mut tse850_priv;
    if tse850.is_null() {
        return -ENOMEM;
    }

    snd_soc_card_set_drvdata(card, tse850 as *mut c_void);

    ret = tse850_dt_init(pdev);
    if ret != 0 {
        dev_err(dev, c"failed to init dt info\n".as_ptr());
        return ret;
    }

    (*tse850).add = devm_gpiod_get(dev, c"axentia,add".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR((*tse850).add as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*tse850).add as *const c_void),
            c"failed to get 'add' gpio\n".as_ptr(),
        );
    }
    (*tse850).add_cache = 1;

    (*tse850).loop1 = devm_gpiod_get(dev, c"axentia,loop1".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR((*tse850).loop1 as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*tse850).loop1 as *const c_void),
            c"failed to get 'loop1' gpio\n".as_ptr(),
        );
    }
    (*tse850).loop1_cache = 1;

    (*tse850).loop2 = devm_gpiod_get(dev, c"axentia,loop2".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR((*tse850).loop2 as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*tse850).loop2 as *const c_void),
            c"failed to get 'loop2' gpio\n".as_ptr(),
        );
    }
    (*tse850).loop2_cache = 1;

    (*tse850).ana = devm_regulator_get(dev, c"axentia,ana".as_ptr());
    if IS_ERR((*tse850).ana as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*tse850).ana as *const c_void),
            c"failed to get 'ana' regulator\n".as_ptr(),
        );
    }

    ret = regulator_enable((*tse850).ana);
    if ret < 0 {
        dev_err(dev, c"failed to enable the 'ana' regulator\n".as_ptr());
        return ret;
    }

    ret = snd_soc_register_card(card);
    if ret != 0 {
        dev_err(dev, c"snd_soc_register_card failed\n".as_ptr());
        regulator_disable((*tse850).ana);
        return ret;
    }

    0
}

unsafe extern "C" fn tse850_remove(pdev: *mut platform_device) {
    let card = platform_get_drvdata(pdev) as *mut snd_soc_card;
    let tse850 = snd_soc_card_get_drvdata(card) as *mut tse850_priv;

    snd_soc_unregister_card(card);
    regulator_disable((*tse850).ana);
}

static TSE850_DT_IDS: [of_device_id; 2] = [
    of_device_id {
        compatible: c"axentia,tse850-pcm5142".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
MODULE_DEVICE_TABLE!(of, TSE850_DT_IDS);

static mut TSE850_DRIVER: platform_driver = platform_driver {
    driver: driver {
        name: c"axentia-tse850-pcm5142".as_ptr(),
        of_match_table: TSE850_DT_IDS.as_ptr(),
    },
    probe: Some(tse850_probe),
    remove: Some(tse850_remove),
};

module_platform_driver!(TSE850_DRIVER);

/* Module information */
MODULE_AUTHOR!(c"Peter Rosin <peda@axentia.se>".as_ptr());
MODULE_DESCRIPTION!(c"ALSA SoC driver for TSE-850 with PCM5142 codec".as_ptr());
MODULE_LICENSE!(c"GPL v2".as_ptr());

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
