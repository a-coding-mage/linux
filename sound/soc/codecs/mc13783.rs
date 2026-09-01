// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2008 Juergen Beisert, kernel@pengutronix.de
 * Copyright 2009 Sascha Hauer, s.hauer@pengutronix.de
 * Copyright 2012 Philippe Retornaz, philippe.retornaz@epfl.ch
 *
 * Initial development of this code was funded by
 * Phytec Messtechnik GmbH, https://www.phytec.de
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

/* Dependencies from linux/module.h, linux/device.h, linux/of.h,
 * linux/mfd/mc13xxx.h, linux/slab.h, sound/core.h, sound/control.h,
 * sound/pcm.h, sound/soc.h, sound/initval.h, sound/soc-dapm.h,
 * linux/regmap.h, and "mc13783.h" are expected from the surrounding tree.
 */

#[repr(C)]
pub struct mc13xxx {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub id: c_int,
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
pub struct snd_soc_dai_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_sysclk:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_tdm_slot:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub controls: *mut snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *mut snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
}
#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}
#[repr(C)]
pub struct device {
    pub parent: *mut device,
    pub platform_data: *mut c_void,
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mc13xxx_codec_platform_data {
    pub adc_ssi_port: mc13783_ssi_port,
    pub dac_ssi_port: mc13783_ssi_port,
}

pub type mc13783_ssi_port = c_uint;

unsafe extern "C" {
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_init_regmap(component: *mut snd_soc_component, regmap: *mut regmap);
    fn dev_get_regmap(dev: *mut device, name: *const c_char) -> *mut regmap;
    fn mc13xxx_reg_write(mc13xxx: *mut mc13xxx, reg: c_uint, val: c_uint) -> c_int;
    fn mc13xxx_reg_rmw(
        mc13xxx: *mut mc13xxx,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn of_get_child_by_name(node: *mut device_node, name: *const c_char) -> *mut device_node;
    fn of_property_read_u32(node: *mut device_node, propname: *const c_char, out_value: *mut c_uint)
        -> c_int;
    fn of_node_put(node: *mut device_node);
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOSYS: c_int = 38;
const GFP_KERNEL: c_uint = 0;

const MC13783_AUDIO_RX0: c_uint = 0;
const MC13783_AUDIO_RX1: c_uint = 1;
const MC13783_AUDIO_TX: c_uint = 2;
const MC13783_SSI_NETWORK: c_uint = 3;
const MC13783_AUDIO_CODEC: c_uint = 4;
const MC13783_AUDIO_DAC: c_uint = 5;
const MC13783_ID_STEREO_DAC: c_int = 0;
const MC13783_ID_STEREO_CODEC: c_int = 1;
const MC13783_ID_SYNC: c_int = 2;
const MC13783_CLK_CLIB: c_int = 1;
const MC13783_SSI1_PORT: mc13783_ssi_port = 0;

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_RATE_8000: c_uint = 1 << 0;
const SNDRV_PCM_RATE_16000: c_uint = 1 << 1;
const SNDRV_PCM_RATE_8000_96000: c_uint = 0xffff;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S20_3LE: u64 = 1 << 1;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 2;

const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_DSP_A: c_uint = 2;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0x00f0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0x0010;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0x0020;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0x0030;
const SND_SOC_DAIFMT_IB_IF: c_uint = 0x0040;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0x0f00;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0x0100;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0x0200;

const SND_SOC_NOPM: c_uint = 0;

const AUDIO_RX0_ALSPEN: c_uint = 1 << 5;
const AUDIO_RX0_ALSPSEL: c_uint = 1 << 7;
const AUDIO_RX0_ADDCDC: c_uint = 1 << 21;
const AUDIO_RX0_ADDSTDC: c_uint = 1 << 22;
const AUDIO_RX0_ADDRXIN: c_uint = 1 << 23;

const AUDIO_RX1_PGARXEN: c_uint = 1 << 0;
const AUDIO_RX1_PGASTEN: c_uint = 1 << 5;
const AUDIO_RX1_ARXINEN: c_uint = 1 << 10;

const AUDIO_TX_AMC1REN: c_uint = 1 << 5;
const AUDIO_TX_AMC1LEN: c_uint = 1 << 7;
const AUDIO_TX_AMC2EN: c_uint = 1 << 9;
const AUDIO_TX_ATXINEN: c_uint = 1 << 11;
const AUDIO_TX_RXINREC: c_uint = 1 << 13;

const fn SSI_NETWORK_CDCTXRXSLOT(x: c_uint) -> c_uint {
    (x & 0x3) << 2
}
const fn SSI_NETWORK_CDCTXSECSLOT(x: c_uint) -> c_uint {
    (x & 0x3) << 4
}
const fn SSI_NETWORK_CDCRXSECSLOT(x: c_uint) -> c_uint {
    (x & 0x3) << 6
}
const fn SSI_NETWORK_CDCRXSECGAIN(x: c_uint) -> c_uint {
    (x & 0x3) << 8
}
const fn SSI_NETWORK_CDCFSDLY(_x: c_uint) -> c_uint {
    1 << 11
}
const SSI_NETWORK_CDCSUMGAIN: c_uint = 1 << 10;
const SSI_NETWORK_DAC_SLOTS_8: c_uint = 1 << 12;
const SSI_NETWORK_DAC_SLOTS_4: c_uint = 2 << 12;
const SSI_NETWORK_DAC_SLOTS_2: c_uint = 3 << 12;
const SSI_NETWORK_DAC_SLOT_MASK: c_uint = 3 << 12;
const SSI_NETWORK_DAC_RXSLOT_0_1: c_uint = 0 << 14;
const SSI_NETWORK_DAC_RXSLOT_2_3: c_uint = 1 << 14;
const SSI_NETWORK_DAC_RXSLOT_4_5: c_uint = 2 << 14;
const SSI_NETWORK_DAC_RXSLOT_6_7: c_uint = 3 << 14;
const SSI_NETWORK_DAC_RXSLOT_MASK: c_uint = 3 << 14;
const fn SSI_NETWORK_STDCRXSECSLOT(x: c_uint) -> c_uint {
    (x & 0x3) << 16
}
const fn SSI_NETWORK_STDCRXSECGAIN(x: c_uint) -> c_uint {
    (x & 0x3) << 18
}
const SSI_NETWORK_STDCSUMGAIN: c_uint = 1 << 20;

/*
 * MC13783_AUDIO_CODEC and MC13783_AUDIO_DAC mostly share the same
 * register layout
 */
const AUDIO_SSI_SEL: c_uint = 1 << 0;
const AUDIO_CLK_SEL: c_uint = 1 << 1;
const AUDIO_CSM: c_uint = 1 << 2;
const AUDIO_BCL_INV: c_uint = 1 << 3;
const AUDIO_CFS_INV: c_uint = 1 << 4;
const fn AUDIO_CFS(x: c_uint) -> c_uint {
    (x & 0x3) << 5
}
const fn AUDIO_CLK(x: c_uint) -> c_uint {
    (x & 0x7) << 7
}
const AUDIO_C_EN: c_uint = 1 << 11;
const AUDIO_C_CLK_EN: c_uint = 1 << 12;
const AUDIO_C_RESET: c_uint = 1 << 15;

const AUDIO_CODEC_CDCFS8K16K: c_uint = 1 << 10;
const AUDIO_DAC_CFS_DLY_B: c_uint = 1 << 10;

#[repr(C)]
struct mc13783_priv {
    mc13xxx: *mut mc13xxx,
    regmap: *mut regmap,
    adc_ssi_port: mc13783_ssi_port,
    dac_ssi_port: mc13783_ssi_port,
}

/* Mapping between sample rates and register value */
static mut mc13783_rates: [c_uint; 11] = [
    8000, 11025, 12000, 16000, 22050, 24000, 32000, 44100, 48000, 64000, 96000,
];

unsafe extern "C" fn mc13783_pcm_hw_params_dac(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let rate = params_rate(params);
    let mut i: usize = 0;

    while i < mc13783_rates.len() {
        if rate == mc13783_rates[i] {
            snd_soc_component_update_bits(component, MC13783_AUDIO_DAC, 0xf << 17, (i as c_uint) << 17);
            return 0;
        }
        i += 1;
    }

    -EINVAL
}

unsafe extern "C" fn mc13783_pcm_hw_params_codec(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let rate = params_rate(params);
    let val: c_uint;

    match rate {
        8000 => val = 0,
        16000 => val = AUDIO_CODEC_CDCFS8K16K,
        _ => return -EINVAL,
    }

    snd_soc_component_update_bits(
        component,
        MC13783_AUDIO_CODEC,
        AUDIO_CODEC_CDCFS8K16K,
        val,
    );

    0
}

unsafe extern "C" fn mc13783_pcm_hw_params_sync(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        mc13783_pcm_hw_params_dac(substream, params, dai)
    } else {
        mc13783_pcm_hw_params_codec(substream, params, dai)
    }
}

unsafe extern "C" fn mc13783_set_fmt(
    dai: *mut snd_soc_dai,
    mut fmt: c_uint,
    reg: c_uint,
) -> c_int {
    let component = (*dai).component;
    let mut val: c_uint = 0;
    let mask: c_uint =
        AUDIO_CFS(3) | AUDIO_BCL_INV | AUDIO_CFS_INV | AUDIO_CSM | AUDIO_C_CLK_EN | AUDIO_C_RESET;

    /* DAI mode */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => val |= AUDIO_CFS(2),
        SND_SOC_DAIFMT_DSP_A => val |= AUDIO_CFS(1),
        _ => return -EINVAL,
    }

    /* DAI clock inversion */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => val |= AUDIO_BCL_INV,
        SND_SOC_DAIFMT_NB_IF => val |= AUDIO_BCL_INV | AUDIO_CFS_INV,
        SND_SOC_DAIFMT_IB_NF => {}
        SND_SOC_DAIFMT_IB_IF => val |= AUDIO_CFS_INV,
        _ => {}
    }

    /* DAI clock master masks */
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => val |= AUDIO_C_CLK_EN,
        SND_SOC_DAIFMT_CBC_CFC => val |= AUDIO_CSM,
        _ => return -EINVAL,
    }

    val |= AUDIO_C_RESET;

    snd_soc_component_update_bits(component, reg, mask, val);

    0
}

unsafe extern "C" fn mc13783_set_fmt_async(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    if (*dai).id == MC13783_ID_STEREO_DAC {
        mc13783_set_fmt(dai, fmt, MC13783_AUDIO_DAC)
    } else {
        mc13783_set_fmt(dai, fmt, MC13783_AUDIO_CODEC)
    }
}

unsafe extern "C" fn mc13783_set_fmt_sync(dai: *mut snd_soc_dai, mut fmt: c_uint) -> c_int {
    let mut ret: c_int;

    ret = mc13783_set_fmt(dai, fmt, MC13783_AUDIO_DAC);
    if ret != 0 {
        return ret;
    }

    /*
     * In synchronous mode force the voice codec into consumer mode
     * so that the clock / framesync from the stereo DAC is used
     */
    fmt &= !SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK;
    fmt |= SND_SOC_DAIFMT_CBC_CFC;
    ret = mc13783_set_fmt(dai, fmt, MC13783_AUDIO_CODEC);

    ret
}

static mc13783_sysclk: [c_int; 8] = [
    13000000,
    15360000,
    16800000,
    -1,
    26000000,
    -1, /* 12000000, invalid for voice codec */
    -1, /* 3686400, invalid for voice codec */
    33600000,
];

unsafe extern "C" fn mc13783_set_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
    reg: c_uint,
) -> c_int {
    let component = (*dai).component;
    let mut clk: usize = 0;
    let mut val: c_uint = 0;
    let mask: c_uint = AUDIO_CLK(0x7) | AUDIO_CLK_SEL;

    while clk < mc13783_sysclk.len() {
        if mc13783_sysclk[clk] < 0 {
            clk += 1;
            continue;
        }
        if mc13783_sysclk[clk] as c_uint == freq {
            break;
        }
        clk += 1;
    }

    if clk == mc13783_sysclk.len() {
        return -EINVAL;
    }

    if clk_id == MC13783_CLK_CLIB {
        val |= AUDIO_CLK_SEL;
    }

    val |= AUDIO_CLK(clk as c_uint);

    snd_soc_component_update_bits(component, reg, mask, val);

    0
}

unsafe extern "C" fn mc13783_set_sysclk_dac(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    mc13783_set_sysclk(dai, clk_id, freq, dir, MC13783_AUDIO_DAC)
}

unsafe extern "C" fn mc13783_set_sysclk_codec(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    mc13783_set_sysclk(dai, clk_id, freq, dir, MC13783_AUDIO_CODEC)
}

unsafe extern "C" fn mc13783_set_sysclk_sync(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    let mut ret: c_int;

    ret = mc13783_set_sysclk(dai, clk_id, freq, dir, MC13783_AUDIO_DAC);
    if ret != 0 {
        return ret;
    }

    mc13783_set_sysclk(dai, clk_id, freq, dir, MC13783_AUDIO_CODEC)
}

unsafe extern "C" fn mc13783_set_tdm_slot_dac(
    dai: *mut snd_soc_dai,
    _tx_mask: c_uint,
    rx_mask: c_uint,
    slots: c_int,
    _slot_width: c_int,
) -> c_int {
    let component = (*dai).component;
    let mut val: c_uint = 0;
    let mask: c_uint = SSI_NETWORK_DAC_SLOT_MASK | SSI_NETWORK_DAC_RXSLOT_MASK;

    match slots {
        2 => val |= SSI_NETWORK_DAC_SLOTS_2,
        4 => val |= SSI_NETWORK_DAC_SLOTS_4,
        8 => val |= SSI_NETWORK_DAC_SLOTS_8,
        _ => return -EINVAL,
    }

    match rx_mask {
        0x03 => val |= SSI_NETWORK_DAC_RXSLOT_0_1,
        0x0c => val |= SSI_NETWORK_DAC_RXSLOT_2_3,
        0x30 => val |= SSI_NETWORK_DAC_RXSLOT_4_5,
        0xc0 => val |= SSI_NETWORK_DAC_RXSLOT_6_7,
        _ => return -EINVAL,
    }

    snd_soc_component_update_bits(component, MC13783_SSI_NETWORK, mask, val);

    0
}

unsafe extern "C" fn mc13783_set_tdm_slot_codec(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    _rx_mask: c_uint,
    slots: c_int,
    _slot_width: c_int,
) -> c_int {
    let component = (*dai).component;
    let mut val: c_uint = 0;
    let mask: c_uint = 0x3f;

    if slots != 4 {
        return -EINVAL;
    }

    if tx_mask != 0x3 {
        return -EINVAL;
    }

    val |= 0x00 << 2; /* primary timeslot RX/TX(?) is 0 */
    val |= 0x01 << 4; /* secondary timeslot TX is 1 */

    snd_soc_component_update_bits(component, MC13783_SSI_NETWORK, mask, val);

    0
}

unsafe extern "C" fn mc13783_set_tdm_slot_sync(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let mut ret: c_int;

    ret = mc13783_set_tdm_slot_dac(dai, tx_mask, rx_mask, slots, slot_width);
    if ret != 0 {
        return ret;
    }

    ret = mc13783_set_tdm_slot_codec(dai, tx_mask, rx_mask, slots, slot_width);

    ret
}

/* The following control, enum, DAPM widget, and SOC_* macro initializers are
 * translated as declarations of the same file-local objects. Their concrete
 * layouts are supplied by the external ASoC macro/type definitions.
 */
static mc1l_amp_ctl: snd_kcontrol_new = unsafe { core::mem::zeroed() };
static mc1r_amp_ctl: snd_kcontrol_new = unsafe { core::mem::zeroed() };
static mc2_amp_ctl: snd_kcontrol_new = unsafe { core::mem::zeroed() };
static atx_amp_ctl: snd_kcontrol_new = unsafe { core::mem::zeroed() };

/* Virtual mux. The chip does the input selection automatically
 * as soon as we enable one input. */
static adcl_enum_text: [*const c_char; 2] = [b"MC1L\0".as_ptr() as *const c_char, b"RXINL\0".as_ptr() as *const c_char];
static left_input_mux: snd_kcontrol_new = unsafe { core::mem::zeroed() };

static adcr_enum_text: [*const c_char; 4] = [
    b"MC1R\0".as_ptr() as *const c_char,
    b"MC2\0".as_ptr() as *const c_char,
    b"RXINR\0".as_ptr() as *const c_char,
    b"TXIN\0".as_ptr() as *const c_char,
];
static right_input_mux: snd_kcontrol_new = unsafe { core::mem::zeroed() };

static samp_ctl: snd_kcontrol_new = unsafe { core::mem::zeroed() };

static speaker_amp_source_text: [*const c_char; 2] = [
    b"CODEC\0".as_ptr() as *const c_char,
    b"Right\0".as_ptr() as *const c_char,
];
static speaker_amp_source_mux: snd_kcontrol_new = unsafe { core::mem::zeroed() };

static headset_amp_source_text: [*const c_char; 2] = [
    b"CODEC\0".as_ptr() as *const c_char,
    b"Mixer\0".as_ptr() as *const c_char,
];
static headset_amp_source_mux: snd_kcontrol_new = unsafe { core::mem::zeroed() };

static cdcout_ctl: snd_kcontrol_new = unsafe { core::mem::zeroed() };
static adc_bypass_ctl: snd_kcontrol_new = unsafe { core::mem::zeroed() };
static lamp_ctl: snd_kcontrol_new = unsafe { core::mem::zeroed() };
static hlamp_ctl: snd_kcontrol_new = unsafe { core::mem::zeroed() };
static hramp_ctl: snd_kcontrol_new = unsafe { core::mem::zeroed() };
static llamp_ctl: snd_kcontrol_new = unsafe { core::mem::zeroed() };
static lramp_ctl: snd_kcontrol_new = unsafe { core::mem::zeroed() };

static mc13783_dapm_widgets: [snd_soc_dapm_widget; 39] = unsafe { core::mem::zeroed() };

static mut mc13783_routes: [snd_soc_dapm_route; 36] = [
    /* Input */
    snd_soc_dapm_route { sink: b"MC1L Amp\0".as_ptr() as *const c_char, control: ptr::null(), source: b"MC1LIN\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"MC1R Amp\0".as_ptr() as *const c_char, control: ptr::null(), source: b"MC1RIN\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"MC2 Amp\0".as_ptr() as *const c_char, control: ptr::null(), source: b"MC2IN\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"TXIN Amp\0".as_ptr() as *const c_char, control: ptr::null(), source: b"TXIN\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"PGA Left Input Mux\0".as_ptr() as *const c_char, control: b"MC1L\0".as_ptr() as *const c_char, source: b"MC1L Amp\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"PGA Left Input Mux\0".as_ptr() as *const c_char, control: b"RXINL\0".as_ptr() as *const c_char, source: b"RXINL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"PGA Right Input Mux\0".as_ptr() as *const c_char, control: b"MC1R\0".as_ptr() as *const c_char, source: b"MC1R Amp\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"PGA Right Input Mux\0".as_ptr() as *const c_char, control: b"MC2\0".as_ptr() as *const c_char, source: b"MC2 Amp\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"PGA Right Input Mux\0".as_ptr() as *const c_char, control: b"TXIN\0".as_ptr() as *const c_char, source: b"TXIN Amp\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"PGA Right Input Mux\0".as_ptr() as *const c_char, control: b"RXINR\0".as_ptr() as *const c_char, source: b"RXINR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"PGA Left Input\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PGA Left Input Mux\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"PGA Right Input\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PGA Right Input Mux\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PGA Left Input\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PGA Right Input\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ADC_Reset\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Voice CODEC PGA\0".as_ptr() as *const c_char, control: b"Voice CODEC Bypass\0".as_ptr() as *const c_char, source: b"ADC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Speaker Amp Source MUX\0".as_ptr() as *const c_char, control: b"CODEC\0".as_ptr() as *const c_char, source: b"Voice CODEC PGA\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Speaker Amp Source MUX\0".as_ptr() as *const c_char, control: b"Right\0".as_ptr() as *const c_char, source: b"DAC PGA\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headset Amp Source MUX\0".as_ptr() as *const c_char, control: b"CODEC\0".as_ptr() as *const c_char, source: b"Voice CODEC PGA\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headset Amp Source MUX\0".as_ptr() as *const c_char, control: b"Mixer\0".as_ptr() as *const c_char, source: b"DAC PGA\0".as_ptr() as *const c_char },
    /* Output */
    snd_soc_dapm_route { sink: b"HSL\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Headset Amp Left\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"HSR\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Headset Amp Right\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"RXOUTL\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Line out Amp Left\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"RXOUTR\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Line out Amp Right\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SP\0".as_ptr() as *const c_char, control: b"Speaker Amp Switch\0".as_ptr() as *const c_char, source: b"Speaker Amp Source MUX\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"LSP\0".as_ptr() as *const c_char, control: b"Loudspeaker Amp\0".as_ptr() as *const c_char, source: b"Speaker Amp Source MUX\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"HSL\0".as_ptr() as *const c_char, control: b"Headset Amp Left\0".as_ptr() as *const c_char, source: b"Headset Amp Source MUX\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"HSR\0".as_ptr() as *const c_char, control: b"Headset Amp Right\0".as_ptr() as *const c_char, source: b"Headset Amp Source MUX\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Line out Amp Left\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DAC PGA\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Line out Amp Right\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DAC PGA\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DAC PGA\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DAC_E\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"CDCOUT\0".as_ptr() as *const c_char, control: b"CDCOUT Switch\0".as_ptr() as *const c_char, source: b"Voice CODEC PGA\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: ptr::null(), control: ptr::null(), source: ptr::null() },
    snd_soc_dapm_route { sink: ptr::null(), control: ptr::null(), source: ptr::null() },
    snd_soc_dapm_route { sink: ptr::null(), control: ptr::null(), source: ptr::null() },
];

static mc13783_3d_mixer: [*const c_char; 4] = [
    b"Stereo\0".as_ptr() as *const c_char,
    b"Phase Mix\0".as_ptr() as *const c_char,
    b"Mono\0".as_ptr() as *const c_char,
    b"Mono Mix\0".as_ptr() as *const c_char,
];

static mut mc13783_control_list: [snd_kcontrol_new; 18] = unsafe { core::mem::zeroed() };

unsafe extern "C" fn mc13783_probe(component: *mut snd_soc_component) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut mc13783_priv;

    snd_soc_component_init_regmap(
        component,
        dev_get_regmap((*(*component).dev).parent, ptr::null()),
    );

    /* these are the reset values */
    mc13xxx_reg_write((*priv_).mc13xxx, MC13783_AUDIO_RX0, 0x25893);
    mc13xxx_reg_write((*priv_).mc13xxx, MC13783_AUDIO_RX1, 0x00d35A);
    mc13xxx_reg_write((*priv_).mc13xxx, MC13783_AUDIO_TX, 0x420000);
    mc13xxx_reg_write((*priv_).mc13xxx, MC13783_SSI_NETWORK, 0x013060);
    mc13xxx_reg_write((*priv_).mc13xxx, MC13783_AUDIO_CODEC, 0x180027);
    mc13xxx_reg_write((*priv_).mc13xxx, MC13783_AUDIO_DAC, 0x0e0004);

    if (*priv_).adc_ssi_port == MC13783_SSI1_PORT {
        mc13xxx_reg_rmw((*priv_).mc13xxx, MC13783_AUDIO_CODEC, AUDIO_SSI_SEL, 0);
    } else {
        mc13xxx_reg_rmw(
            (*priv_).mc13xxx,
            MC13783_AUDIO_CODEC,
            AUDIO_SSI_SEL,
            AUDIO_SSI_SEL,
        );
    }

    if (*priv_).dac_ssi_port == MC13783_SSI1_PORT {
        mc13xxx_reg_rmw((*priv_).mc13xxx, MC13783_AUDIO_DAC, AUDIO_SSI_SEL, 0);
    } else {
        mc13xxx_reg_rmw(
            (*priv_).mc13xxx,
            MC13783_AUDIO_DAC,
            AUDIO_SSI_SEL,
            AUDIO_SSI_SEL,
        );
    }

    0
}

unsafe extern "C" fn mc13783_remove(component: *mut snd_soc_component) {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut mc13783_priv;

    /* Make sure VAUDIOON is off */
    mc13xxx_reg_rmw((*priv_).mc13xxx, MC13783_AUDIO_RX0, 0x3, 0);
}

const MC13783_RATES_RECORD: c_uint = SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000;

const MC13783_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE;

static mc13783_ops_dac: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mc13783_pcm_hw_params_dac),
    set_fmt: Some(mc13783_set_fmt_async),
    set_sysclk: Some(mc13783_set_sysclk_dac),
    set_tdm_slot: Some(mc13783_set_tdm_slot_dac),
};

static mc13783_ops_codec: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mc13783_pcm_hw_params_codec),
    set_fmt: Some(mc13783_set_fmt_async),
    set_sysclk: Some(mc13783_set_sysclk_codec),
    set_tdm_slot: Some(mc13783_set_tdm_slot_codec),
};

/*
 * The mc13783 has two SSI ports, both of them can be routed either
 * to the voice codec or the stereo DAC. When two different SSI ports
 * are used for the voice codec and the stereo DAC we can do different
 * formats and sysclock settings for playback and capture
 * (mc13783-hifi-playback and mc13783-hifi-capture). Using the same port
 * forces us to use symmetric rates (mc13783-hifi).
 */
static mut mc13783_dai_async: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: b"mc13783-hifi-playback\0".as_ptr() as *const c_char,
        id: MC13783_ID_STEREO_DAC,
        playback: snd_soc_pcm_stream {
            stream_name: b"Playback\0".as_ptr() as *const c_char,
            channels_min: 2,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_96000,
            formats: MC13783_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: ptr::null(),
            channels_min: 0,
            channels_max: 0,
            rates: 0,
            formats: 0,
        },
        ops: &mc13783_ops_dac,
        symmetric_rate: 0,
    },
    snd_soc_dai_driver {
        name: b"mc13783-hifi-capture\0".as_ptr() as *const c_char,
        id: MC13783_ID_STEREO_CODEC,
        playback: snd_soc_pcm_stream {
            stream_name: ptr::null(),
            channels_min: 0,
            channels_max: 0,
            rates: 0,
            formats: 0,
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"Capture\0".as_ptr() as *const c_char,
            channels_min: 2,
            channels_max: 2,
            rates: MC13783_RATES_RECORD,
            formats: MC13783_FORMATS,
        },
        ops: &mc13783_ops_codec,
        symmetric_rate: 0,
    },
];

static mc13783_ops_sync: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mc13783_pcm_hw_params_sync),
    set_fmt: Some(mc13783_set_fmt_sync),
    set_sysclk: Some(mc13783_set_sysclk_sync),
    set_tdm_slot: Some(mc13783_set_tdm_slot_sync),
};

static mut mc13783_dai_sync: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: b"mc13783-hifi\0".as_ptr() as *const c_char,
    id: MC13783_ID_SYNC,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: MC13783_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: MC13783_RATES_RECORD,
        formats: MC13783_FORMATS,
    },
    ops: &mc13783_ops_sync,
    symmetric_rate: 1,
}];

static soc_component_dev_mc13783: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(mc13783_probe),
    remove: Some(mc13783_remove),
    controls: unsafe { mc13783_control_list.as_mut_ptr() },
    num_controls: 18,
    dapm_widgets: mc13783_dapm_widgets.as_ptr(),
    num_dapm_widgets: 39,
    dapm_routes: unsafe { mc13783_routes.as_mut_ptr() },
    num_dapm_routes: 36,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn mc13783_codec_probe(pdev: *mut platform_device) -> c_int {
    let mut priv_: *mut mc13783_priv;
    let pdata = (*pdev).dev.platform_data as *mut mc13xxx_codec_platform_data;
    let mut np: *mut device_node;
    let mut ret: c_int;

    priv_ = devm_kzalloc(&mut (*pdev).dev, size_of::<mc13783_priv>(), GFP_KERNEL) as *mut mc13783_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    if !pdata.is_null() {
        (*priv_).adc_ssi_port = (*pdata).adc_ssi_port;
        (*priv_).dac_ssi_port = (*pdata).dac_ssi_port;
    } else {
        np = of_get_child_by_name((*(*pdev).dev.parent).of_node, b"codec\0".as_ptr() as *const c_char);
        if np.is_null() {
            return -ENOSYS;
        }

        ret = of_property_read_u32(
            np,
            b"adc-port\0".as_ptr() as *const c_char,
            &mut (*priv_).adc_ssi_port,
        );
        if ret != 0 {
            of_node_put(np);
            return ret;
        }

        ret = of_property_read_u32(
            np,
            b"dac-port\0".as_ptr() as *const c_char,
            &mut (*priv_).dac_ssi_port,
        );
        if ret != 0 {
            of_node_put(np);
            return ret;
        }

        of_node_put(np);
    }

    dev_set_drvdata(&mut (*pdev).dev, priv_ as *mut c_void);
    (*priv_).mc13xxx = dev_get_drvdata((*pdev).dev.parent) as *mut mc13xxx;

    if (*priv_).adc_ssi_port == (*priv_).dac_ssi_port {
        ret = devm_snd_soc_register_component(
            &mut (*pdev).dev,
            &soc_component_dev_mc13783,
            mc13783_dai_sync.as_mut_ptr(),
            mc13783_dai_sync.len() as c_int,
        );
    } else {
        ret = devm_snd_soc_register_component(
            &mut (*pdev).dev,
            &soc_component_dev_mc13783,
            mc13783_dai_async.as_mut_ptr(),
            mc13783_dai_async.len() as c_int,
        );
    }

    ret
}

static mut mc13783_codec_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"mc13783-codec\0".as_ptr() as *const c_char,
    },
};

/* module_platform_driver_probe(mc13783_codec_driver, mc13783_codec_probe); */

/* MODULE_DESCRIPTION("ASoC MC13783 driver"); */
/* MODULE_AUTHOR("Sascha Hauer, Pengutronix <s.hauer@pengutronix.de>"); */
/* MODULE_AUTHOR("Philippe Retornaz <philippe.retornaz@epfl.ch>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
