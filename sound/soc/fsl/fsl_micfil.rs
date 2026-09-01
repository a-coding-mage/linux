// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright 2018 NXP

// Rust translation of soc/fsl/fsl_micfil.c.
// External Linux/ALSA/regmap symbols from the original includes and
// fsl_micfil.h/fsl_utils.h are intentionally referenced, not implemented here.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = u32;
type u64 = u64;
type bool_ = bool;
type snd_pcm_format_t = c_int;
type irqreturn_t = c_int;

const MICFIL_OSR_DEFAULT: c_int = 16;

const MICFIL_NUM_RATES: usize = 7;
const MICFIL_CLK_SRC_NUM: usize = 3;
/* clock source ids */
const MICFIL_AUDIO_PLL1: usize = 0;
const MICFIL_AUDIO_PLL2: usize = 1;
const MICFIL_CLK_EXT3: usize = 2;

extern "C" {
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static SNDRV_PCM_FMTBIT_DSD_U32_LE: u64;
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SNDRV_PCM_FORMAT_DSD_U32_LE: snd_pcm_format_t;

    static MICFIL_IRQ_LINES: usize;
    static MICFIL_OUTPUT_CHANNELS: c_int;
    static MICFIL_FIFO_NUM: c_int;
    static MICFIL_DMA_MAXBURST_RX: c_uint;

    static REG_MICFIL_CTRL1: c_uint;
    static REG_MICFIL_CTRL2: c_uint;
    static REG_MICFIL_STAT: c_uint;
    static REG_MICFIL_FIFO_CTRL: c_uint;
    static REG_MICFIL_FIFO_STAT: c_uint;
    static REG_MICFIL_DATACH0: c_uint;
    static REG_MICFIL_DATACH1: c_uint;
    static REG_MICFIL_DATACH2: c_uint;
    static REG_MICFIL_DATACH3: c_uint;
    static REG_MICFIL_DATACH4: c_uint;
    static REG_MICFIL_DATACH5: c_uint;
    static REG_MICFIL_DATACH6: c_uint;
    static REG_MICFIL_DATACH7: c_uint;
    static REG_MICFIL_DC_CTRL: c_uint;
    static REG_MICFIL_DC_OUT_CTRL: c_uint;
    static REG_MICFIL_OUT_CTRL: c_uint;
    static REG_MICFIL_OUT_STAT: c_uint;
    static REG_MICFIL_VAD0_CTRL1: c_uint;
    static REG_MICFIL_VAD0_CTRL2: c_uint;
    static REG_MICFIL_VAD0_STAT: c_uint;
    static REG_MICFIL_VAD0_SCONFIG: c_uint;
    static REG_MICFIL_VAD0_NCONFIG: c_uint;
    static REG_MICFIL_VAD0_NDATA: c_uint;
    static REG_MICFIL_VAD0_ZCD: c_uint;
    static REG_MICFIL_FSYNC_CTRL: c_uint;
    static REG_MICFIL_VERID: c_uint;
    static REG_MICFIL_PARAM: c_uint;

    static MICFIL_OUTGAIN_CHX_SHIFT_BASE: c_uint;
    static MICFIL_CTRL2_QSEL: c_uint;
    static MICFIL_QSEL_HIGH_QUALITY: c_uint;
    static MICFIL_QSEL_MEDIUM_QUALITY: c_uint;
    static MICFIL_QSEL_LOW_QUALITY: c_uint;
    static MICFIL_QSEL_VLOW0_QUALITY: c_uint;
    static MICFIL_QSEL_VLOW1_QUALITY: c_uint;
    static MICFIL_QSEL_VLOW2_QUALITY: c_uint;
    static MICFIL_DC_CTRL_CONFIG: c_uint;
    static MICFIL_DC_BYPASS: c_uint;
    static MICFIL_HWVAD_ENVELOPE_MODE: c_int;
    static MICFIL_HWVAD_ENERGY_MODE: c_int;
    static MICFIL_CTRL1_MDIS: c_uint;
    static MICFIL_CTRL1_SRES: c_uint;
    static MICFIL_VAD0_CTRL1_IE: c_uint;
    static MICFIL_VAD0_CTRL1_ERIE: c_uint;
    static MICFIL_VAD0_CTRL2_FRENDIS: c_uint;
    static MICFIL_VAD0_CTRL2_PREFEN: c_uint;
    static MICFIL_VAD0_SCONFIG_SFILEN: c_uint;
    static MICFIL_VAD0_SCONFIG_SMAXEN: c_uint;
    static MICFIL_VAD0_NCONFIG_NFILAUT: c_uint;
    static MICFIL_VAD0_NCONFIG_NMINEN: c_uint;
    static MICFIL_VAD0_NCONFIG_NDECEN: c_uint;
    static MICFIL_VAD0_NCONFIG_NOREN: c_uint;
    static MICFIL_VAD0_CTRL1_ST10: c_uint;
    static MICFIL_VAD0_CTRL1_RST: c_uint;
    static MICFIL_VAD0_CTRL1_EN: c_uint;
    static MICFIL_CTRL1_DISEL: c_uint;
    static MICFIL_CTRL1_DISEL_DMA: c_uint;
    static MICFIL_CTRL1_DISEL_DISABLE: c_uint;
    static MICFIL_CTRL1_PDMIEN: c_uint;
    static MICFIL_CTRL1_ERREN: c_uint;
    static MICFIL_CTRL2_DEC_BYPASS: c_uint;
    static MICFIL_CTRL2_CLKDIV: c_uint;
    static MICFIL_CTRL2_CICOSR: c_uint;
    static MICFIL_VAD0_CTRL1_CICOSR: c_uint;
    static MICFIL_VAD0_CTRL1_CHSEL: c_uint;
    static MICFIL_VERID_MAJOR_MASK: c_uint;
    static MICFIL_VERID_MINOR_MASK: c_uint;
    static MICFIL_VERID_MINOR_SHIFT: c_uint;
    static MICFIL_VERID_FEATURE_MASK: c_uint;
    static MICFIL_PARAM_NUM_HWVAD_MASK: c_uint;
    static MICFIL_PARAM_NUM_HWVAD_SHIFT: c_uint;
    static MICFIL_PARAM_HWVAD_ZCD: c_uint;
    static MICFIL_PARAM_HWVAD_ENERGY_MODE: c_uint;
    static MICFIL_PARAM_HWVAD: c_uint;
    static MICFIL_PARAM_DC_OUT_BYPASS: c_uint;
    static MICFIL_PARAM_DC_IN_BYPASS: c_uint;
    static MICFIL_PARAM_LOW_POWER: c_uint;
    static MICFIL_PARAM_FIL_OUT_WIDTH: c_uint;
    static MICFIL_PARAM_FIFO_PTRWID_MASK: c_uint;
    static MICFIL_PARAM_FIFO_PTRWID_SHIFT: c_uint;
    static MICFIL_PARAM_NPAIR_MASK: c_uint;
    static MICFIL_PARAM_NPAIR_SHIFT: c_uint;
    static MICFIL_FIFO_CTRL_FIFOWMK: c_uint;
    static MICFIL_STAT_BSY_FIL: c_uint;
    static MICFIL_STAT_FIR_RDY: c_uint;
    static MICFIL_STAT_LOWFREQF: c_uint;
    static MICFIL_VAD0_STAT_IF: c_uint;
    static MICFIL_VAD0_STAT_INSATF: c_uint;

    static EINVAL: c_int;
    static ENOMEM: c_int;
    static ENOSYS: c_int;
    static GFP_KERNEL: c_uint;
    static IRQF_SHARED: c_uint;
    static IRQ_HANDLED: irqreturn_t;
    static IRQ_WAKE_THREAD: irqreturn_t;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static SNDRV_PCM_HW_PARAM_RATE: c_int;
    static SNDRV_CTL_ELEM_IFACE_MIXER: c_uint;
    static SNDRV_CTL_ELEM_ACCESS_READ: c_uint;
    static SNDRV_CTL_ELEM_ACCESS_VOLATILE: c_uint;
    static SNDRV_CTL_EVENT_MASK_VALUE: c_uint;
    static REGCACHE_MAPLE: c_uint;
}

#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct device_node { pub name: *const c_char }
#[repr(C)] pub struct regmap { _priv: [u8; 0] }
#[repr(C)] pub struct clk { _priv: [u8; 0] }
#[repr(C)] pub struct resource { pub start: usize }
#[repr(C)] pub struct snd_soc_card { pub snd_card: *mut c_void }
#[repr(C)] pub struct snd_kcontrol { pub private_value: usize, pub id: c_uint }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device }
#[repr(C)] pub struct snd_soc_dai_component { pub card: *mut snd_soc_card }
#[repr(C)] pub struct snd_soc_dai { pub dev: *mut device, pub component: *mut snd_soc_dai_component }
#[repr(C)] pub struct snd_pcm_runtime { _priv: [u8; 0] }
#[repr(C)] pub struct snd_pcm_substream { pub runtime: *mut snd_pcm_runtime }
#[repr(C)] pub struct snd_pcm_hw_params { _priv: [u8; 0] }
#[repr(C)] pub struct soc_mixer_control { pub shift: c_uint }
#[repr(C)] pub struct soc_enum { _priv: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_constraint_list { pub count: c_uint, pub list: *const c_uint }
#[repr(C)] pub struct sdma_peripheral_config { pub n_fifos_src: c_uint, pub sw_done: bool }
#[repr(C)] pub struct snd_dmaengine_dai_dma_data {
    pub chan_name: *const c_char,
    pub addr: usize,
    pub maxburst: c_uint,
    pub peripheral_config: *mut sdma_peripheral_config,
    pub peripheral_size: usize,
}
#[repr(C)] pub struct fsl_micfil_verid { pub version: c_uint, pub feature: c_uint }
#[repr(C)] pub struct fsl_micfil_param {
    pub hwvad_num: c_uint, pub hwvad_zcd: c_uint, pub hwvad_energy_mode: c_uint,
    pub hwvad: c_uint, pub dc_out_bypass: c_uint, pub dc_in_bypass: c_uint,
    pub low_power: c_uint, pub fil_out_width: c_uint, pub fifo_ptrwid: c_uint,
    pub npair: c_uint,
}
#[repr(C)] pub struct reg_default { pub reg: c_uint, pub def: c_uint }

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
}
#[repr(C)] pub struct snd_ctl_elem_value_integer { pub value: [c_long; 128] }
#[repr(C)] pub struct snd_ctl_elem_value_enumerated { pub item: [c_uint; 128] }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }
type c_long = isize;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum quality {
    QUALITY_HIGH,
    QUALITY_MEDIUM,
    QUALITY_LOW,
    QUALITY_VLOW0,
    QUALITY_VLOW1,
    QUALITY_VLOW2,
}

#[repr(C)]
struct fsl_micfil {
    pdev: *mut platform_device,
    regmap: *mut regmap,
    soc: *const fsl_micfil_soc_data,
    busclk: *mut clk,
    mclk: *mut clk,
    pll8k_clk: *mut clk,
    pll11k_clk: *mut clk,
    clk_src: [*mut clk; MICFIL_CLK_SRC_NUM],
    dma_params_rx: snd_dmaengine_dai_dma_data,
    sdmacfg: sdma_peripheral_config,
    card: *mut snd_soc_card,
    constraint_rates: snd_pcm_hw_constraint_list,
    constraint_rates_list: [c_uint; MICFIL_NUM_RATES],
    dataline: c_uint,
    name: [c_char; 32],
    irq: [c_int; 4],
    quality: quality,
    dc_remover: c_int,
    dc_out_remover: c_int,
    vad_init_mode: c_int,
    vad_enabled: c_int,
    vad_detected: c_int,
    verid: fsl_micfil_verid,
    param: fsl_micfil_param,
    mclk_flag: bool_,
    dec_bypass: bool_,
}

#[repr(C)]
struct fsl_micfil_soc_data {
    fifos: c_uint,
    fifo_depth: c_uint,
    dataline: c_uint,
    imx: bool_,
    use_edma: bool_,
    use_verid: bool_,
    volume_sx: bool_,
    formats: u64,
    fifo_offset: c_int,
    default_quality: quality,
    /* stores const value in formula to calculate range */
    rangeadj_const: [[c_int; 2]; 3],
}

static fsl_micfil_rates: [c_uint; MICFIL_NUM_RATES] =
    [8000, 11025, 16000, 22050, 32000, 44100, 48000];

static fsl_micfil_rate_constraints: snd_pcm_hw_constraint_list =
    snd_pcm_hw_constraint_list { count: MICFIL_NUM_RATES as c_uint, list: fsl_micfil_rates.as_ptr() };

static mut fsl_micfil_imx8mm: fsl_micfil_soc_data = fsl_micfil_soc_data {
    imx: true, fifos: 8, fifo_depth: 8, dataline: 0xf, formats: 0,
    use_edma: false, use_verid: false, volume_sx: true, fifo_offset: 0,
    default_quality: quality::QUALITY_VLOW0, rangeadj_const: [[0; 2]; 3],
};
static mut fsl_micfil_imx8mp: fsl_micfil_soc_data = fsl_micfil_soc_data {
    imx: true, fifos: 8, fifo_depth: 32, dataline: 0xf, formats: 0,
    use_edma: false, use_verid: false, volume_sx: false, fifo_offset: 0,
    default_quality: quality::QUALITY_MEDIUM, rangeadj_const: [[27, 7], [27, 7], [26, 7]],
};
static mut fsl_micfil_imx93: fsl_micfil_soc_data = fsl_micfil_soc_data {
    imx: true, fifos: 8, fifo_depth: 32, dataline: 0xf, formats: 0,
    use_edma: true, use_verid: true, volume_sx: false, fifo_offset: 0,
    default_quality: quality::QUALITY_MEDIUM, rangeadj_const: [[30, 6], [30, 6], [29, 6]],
};
static mut fsl_micfil_imx943: fsl_micfil_soc_data = fsl_micfil_soc_data {
    imx: true, fifos: 8, fifo_depth: 32, dataline: 0xf, formats: 0,
    use_edma: true, use_verid: true, volume_sx: false, fifo_offset: -4,
    default_quality: quality::QUALITY_MEDIUM, rangeadj_const: [[34, 6], [34, 6], [33, 6]],
};

// static const struct of_device_id fsl_micfil_dt_ids[] =
// { "fsl,imx8mm-micfil" -> fsl_micfil_imx8mm, "fsl,imx8mp-micfil" -> fsl_micfil_imx8mp,
//   "fsl,imx93-micfil" -> fsl_micfil_imx93, "fsl,imx943-micfil" -> fsl_micfil_imx943, {} };
// MODULE_DEVICE_TABLE(of, fsl_micfil_dt_ids);

static micfil_quality_select_texts: [&[u8]; 6] =
    [b"High\0", b"Medium\0", b"Low\0", b"VLow0\0", b"Vlow1\0", b"Vlow2\0"];
// static const struct soc_enum fsl_micfil_quality_enum =
//     SOC_ENUM_SINGLE_EXT(ARRAY_SIZE(micfil_quality_select_texts), micfil_quality_select_texts);
// static DECLARE_TLV_DB_SCALE(gain_tlv, 0, 100, 0);

extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut fsl_micfil;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut fsl_micfil;
    fn dev_get_drvdata(dev: *mut device) -> *mut fsl_micfil;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut fsl_micfil);
    fn snd_soc_enum_item_to_val(e: *mut soc_enum, item: c_uint) -> c_int;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device);
    fn pm_runtime_put_sync(dev: *mut device) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_enabled(dev: *mut device) -> bool;
    fn pm_runtime_status_suspended(dev: *mut device) -> bool;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_clear_bits(map: *mut regmap, reg: c_uint, bits: c_uint) -> c_int;
    fn regmap_set_bits(map: *mut regmap, reg: c_uint, bits: c_uint) -> c_int;
    fn regmap_write_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(c: *mut snd_soc_component, r: c_uint, m: c_uint, v: c_uint) -> c_int;
    fn snd_pcm_hw_constraint_list(r: *mut snd_pcm_runtime, c: c_uint, p: c_int, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: c_int) -> c_int;
    fn fsl_asoc_reparent_pll_clocks(dev: *mut device, clk: *mut clk, pll8k: *mut clk, pll11k: *mut clk, ratio: u64);
    fn fsl_asoc_get_pll_clocks(dev: *mut device, pll8k: *mut *mut clk, pll11k: *mut *mut clk);
    fn fsl_asoc_constrain_rates(dst: *mut snd_pcm_hw_constraint_list, src: *const snd_pcm_hw_constraint_list, c0: *mut clk, c1: *mut clk, c2: *mut clk, list: *mut c_uint);
    fn snd_soc_dai_init_dma_data(dai: *mut snd_soc_dai, tx: *mut c_void, rx: *mut snd_dmaengine_dai_dma_data);
    fn snd_soc_add_component_controls(c: *mut snd_soc_component, controls: *const c_void, count: c_uint) -> c_int;
    fn ilog2(n: c_uint) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn FIELD_PREP(mask: c_uint, val: c_uint) -> c_uint;
    fn FIELD_GET(mask: c_uint, val: c_uint) -> c_uint;
    fn MICFIL_OUTGAIN_CHX_SHIFT(i: c_int) -> c_uint;
    fn MICFIL_DC_CHX_SHIFT(i: c_int) -> c_uint;
    fn MICFIL_STAT_CHXF(i: c_int) -> c_uint;
    fn MICFIL_FIFO_STAT_FIFOX_OVER(i: c_int) -> c_uint;
    fn MICFIL_FIFO_STAT_FIFOX_UNDER(i: c_int) -> c_uint;
}

unsafe fn micfil_get_max_range(micfil: *mut fsl_micfil) -> c_int {
    let max_range = match (*micfil).quality {
        quality::QUALITY_HIGH | quality::QUALITY_VLOW0 =>
            (*(*micfil).soc).rangeadj_const[0][0] - (*(*micfil).soc).rangeadj_const[0][1] * ilog2((2 * MICFIL_OSR_DEFAULT) as c_uint),
        quality::QUALITY_MEDIUM | quality::QUALITY_VLOW1 =>
            (*(*micfil).soc).rangeadj_const[1][0] - (*(*micfil).soc).rangeadj_const[1][1] * ilog2(MICFIL_OSR_DEFAULT as c_uint),
        quality::QUALITY_LOW | quality::QUALITY_VLOW2 =>
            (*(*micfil).soc).rangeadj_const[2][0] - (*(*micfil).soc).rangeadj_const[2][1] * ilog2(MICFIL_OSR_DEFAULT as c_uint),
    };
    if max_range < 0 { 0 } else { max_range }
}

unsafe fn micfil_range_set(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let micfil = snd_soc_component_get_drvdata(cmpnt);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let shift = (*mc).shift;
    let new_range = (*ucontrol).value.integer.value[0] as c_int;
    let max_range = micfil_get_max_range(micfil);
    if new_range > max_range {
        dev_warn(&mut (*(*micfil).pdev).dev, b"range makes channel %d data unreliable\n\0".as_ptr() as *const c_char, shift / 4);
    }
    let mut ret = pm_runtime_resume_and_get((*cmpnt).dev);
    if ret != 0 { return ret; }
    ret = snd_soc_component_update_bits(cmpnt, REG_MICFIL_OUT_CTRL, 0xF << shift, (new_range as c_uint) << shift);
    pm_runtime_put_autosuspend((*cmpnt).dev);
    ret
}

unsafe fn micfil_set_quality(micfil: *mut fsl_micfil) -> c_int {
    let mut val: u32 = 0;
    if !(*(*micfil).soc).volume_sx {
        regmap_read((*micfil).regmap, REG_MICFIL_OUT_CTRL, &mut val);
        let max_range = micfil_get_max_range(micfil);
        let mut i = 0;
        while i < (*(*micfil).soc).fifos as c_int {
            let range = ((val >> MICFIL_OUTGAIN_CHX_SHIFT(i)) & 0xF) as c_int;
            if range > max_range {
                dev_warn(&mut (*(*micfil).pdev).dev, b"please reset channel %d range\n\0".as_ptr() as *const c_char, i);
            }
            i += 1;
        }
    }
    let qsel = match (*micfil).quality {
        quality::QUALITY_HIGH => MICFIL_QSEL_HIGH_QUALITY,
        quality::QUALITY_MEDIUM => MICFIL_QSEL_MEDIUM_QUALITY,
        quality::QUALITY_LOW => MICFIL_QSEL_LOW_QUALITY,
        quality::QUALITY_VLOW0 => MICFIL_QSEL_VLOW0_QUALITY,
        quality::QUALITY_VLOW1 => MICFIL_QSEL_VLOW1_QUALITY,
        quality::QUALITY_VLOW2 => MICFIL_QSEL_VLOW2_QUALITY,
    };
    regmap_update_bits((*micfil).regmap, REG_MICFIL_CTRL2, MICFIL_CTRL2_QSEL, FIELD_PREP(MICFIL_CTRL2_QSEL, qsel))
}

unsafe fn micfil_quality_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let micfil = snd_soc_component_get_drvdata(cmpnt);
    (*ucontrol).value.integer.value[0] = (*micfil).quality as c_long;
    0
}

unsafe fn micfil_quality_set(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let micfil = snd_soc_component_get_drvdata(cmpnt);
    let val = (*ucontrol).value.integer.value[0] as c_int;
    let mut change = false;
    if val < quality::QUALITY_HIGH as c_int || val > quality::QUALITY_VLOW2 as c_int { return -EINVAL; }
    if (*micfil).quality as c_int != val {
        let ret0 = pm_runtime_resume_and_get((*cmpnt).dev);
        if ret0 != 0 { return ret0; }
        let old_val = (*micfil).quality;
        (*micfil).quality = core::mem::transmute::<c_int, quality>(val);
        let ret = micfil_set_quality(micfil);
        pm_runtime_put_autosuspend((*cmpnt).dev);
        if ret != 0 {
            (*micfil).quality = old_val;
            return ret;
        }
        change = true;
    }
    change as c_int
}

static micfil_hwvad_enable: [&[u8]; 2] = [b"Disable (Record only)\0", b"Enable (Record with Vad)\0"];
static micfil_hwvad_init_mode: [&[u8]; 2] = [b"Envelope mode\0", b"Energy mode\0"];
static micfil_hwvad_hpf_texts: [&[u8]; 4] = [b"Filter bypass\0", b"Cut-off @1750Hz\0", b"Cut-off @215Hz\0", b"Cut-off @102Hz\0"];

/*
 * DC Remover Control
 * Filter Bypassed 1 1
 * Cut-off @21Hz 0 0
 * Cut-off @83Hz 0 1
 * Cut-off @152HZ 1 0
 */
static micfil_dc_remover_texts: [&[u8]; 4] = [b"Cut-off @21Hz\0", b"Cut-off @83Hz\0", b"Cut-off @152Hz\0", b"Bypass\0"];
static micfil_dc_out_remover_texts: [&[u8]; 4] = [b"Cut-off @20Hz\0", b"Cut-off @13.3Hz\0", b"Cut-off @40Hz\0", b"Bypass\0"];

// hwvad_enable_enum, hwvad_init_mode_enum, hwvad_hpf_enum,
// fsl_micfil_dc_remover_enum and fsl_micfil_dc_out_remover_enum are
// SOC_ENUM_* macro initializers in C.

unsafe fn micfil_put_dc_remover_state(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let e = (*kcontrol).private_value as *mut soc_enum;
    let comp = snd_kcontrol_chip(kcontrol);
    let micfil = snd_soc_component_get_drvdata(comp);
    let val = snd_soc_enum_item_to_val(e, (*ucontrol).value.enumerated.item[0]);
    let mut reg_val: u32 = 0;
    if val < 0 || val > 3 { return -EINVAL; }
    let mut ret = pm_runtime_resume_and_get((*comp).dev);
    if ret != 0 { return ret; }
    (*micfil).dc_remover = val;
    let mut i = 0;
    while i < MICFIL_OUTPUT_CHANNELS {
        reg_val |= (val as u32) << MICFIL_DC_CHX_SHIFT(i);
        i += 1;
    }
    ret = snd_soc_component_update_bits(comp, REG_MICFIL_DC_CTRL, MICFIL_DC_CTRL_CONFIG, reg_val);
    pm_runtime_put_autosuspend((*comp).dev);
    ret
}

unsafe fn micfil_get_dc_remover_state(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let micfil = snd_soc_component_get_drvdata(comp);
    (*ucontrol).value.enumerated.item[0] = (*micfil).dc_remover as c_uint;
    0
}

unsafe fn micfil_put_dc_out_remover_state(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let e = (*kcontrol).private_value as *mut soc_enum;
    let comp = snd_kcontrol_chip(kcontrol);
    let micfil = snd_soc_component_get_drvdata(comp);
    let val = snd_soc_enum_item_to_val(e, (*ucontrol).value.enumerated.item[0]);
    let mut reg_val: u32 = 0;
    if val < 0 || val > 3 { return -EINVAL; }
    let mut ret = pm_runtime_resume_and_get((*comp).dev);
    if ret != 0 { return ret; }
    (*micfil).dc_out_remover = val;
    let mut i = 0;
    while i < MICFIL_OUTPUT_CHANNELS {
        reg_val |= (val as u32) << MICFIL_DC_CHX_SHIFT(i);
        i += 1;
    }
    ret = snd_soc_component_update_bits(comp, REG_MICFIL_DC_OUT_CTRL, MICFIL_DC_CTRL_CONFIG, reg_val);
    pm_runtime_put_autosuspend((*comp).dev);
    ret
}

unsafe fn micfil_get_dc_out_remover_state(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let micfil = snd_soc_component_get_drvdata(comp);
    (*ucontrol).value.enumerated.item[0] = (*micfil).dc_out_remover as c_uint;
    0
}

unsafe fn hwvad_put_enable(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let micfil = snd_soc_component_get_drvdata(comp);
    let val = snd_soc_enum_item_to_val(e, (*ucontrol).value.enumerated.item[0]);
    if val < 0 || val > 1 { return -EINVAL; }
    let change = (*micfil).vad_enabled != val;
    (*micfil).vad_enabled = val;
    change as c_int
}

unsafe fn hwvad_get_enable(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let micfil = snd_soc_component_get_drvdata(comp);
    (*ucontrol).value.enumerated.item[0] = (*micfil).vad_enabled as c_uint;
    0
}

unsafe fn hwvad_put_init_mode(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let micfil = snd_soc_component_get_drvdata(comp);
    let val = snd_soc_enum_item_to_val(e, (*ucontrol).value.enumerated.item[0]);
    if val < MICFIL_HWVAD_ENVELOPE_MODE || val > MICFIL_HWVAD_ENERGY_MODE { return -EINVAL; }
    /* 0 - Envelope-based Mode
     * 1 - Energy-based Mode
     */
    let change = (*micfil).vad_init_mode != val;
    (*micfil).vad_init_mode = val;
    change as c_int
}

unsafe fn hwvad_get_init_mode(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let micfil = snd_soc_component_get_drvdata(comp);
    (*ucontrol).value.enumerated.item[0] = (*micfil).vad_init_mode as c_uint;
    0
}

unsafe fn hwvad_detected(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let micfil = snd_soc_component_get_drvdata(comp);
    (*ucontrol).value.enumerated.item[0] = (*micfil).vad_detected as c_uint;
    0
}

// fsl_micfil_range_controls, fsl_micfil_volume_sx_controls,
// fsl_micfil_dc_out_controls and fsl_micfil_snd_controls are arrays of
// struct snd_kcontrol_new populated through SOC_* ALSA macros in the C source.

unsafe fn fsl_micfil_use_verid(dev: *mut device) -> c_int {
    let micfil = dev_get_drvdata(dev);
    let mut val: c_uint = 0;
    if !(*(*micfil).soc).use_verid { return 0; }
    let mut ret = regmap_read((*micfil).regmap, REG_MICFIL_VERID, &mut val);
    if ret < 0 { return ret; }
    dev_dbg(dev, b"VERID: 0x%016X\n\0".as_ptr() as *const c_char, val);
    (*micfil).verid.version = val & (MICFIL_VERID_MAJOR_MASK | MICFIL_VERID_MINOR_MASK);
    (*micfil).verid.version >>= MICFIL_VERID_MINOR_SHIFT;
    (*micfil).verid.feature = val & MICFIL_VERID_FEATURE_MASK;
    ret = regmap_read((*micfil).regmap, REG_MICFIL_PARAM, &mut val);
    if ret < 0 { return ret; }
    dev_dbg(dev, b"PARAM: 0x%016X\n\0".as_ptr() as *const c_char, val);
    (*micfil).param.hwvad_num = (val & MICFIL_PARAM_NUM_HWVAD_MASK) >> MICFIL_PARAM_NUM_HWVAD_SHIFT;
    (*micfil).param.hwvad_zcd = val & MICFIL_PARAM_HWVAD_ZCD;
    (*micfil).param.hwvad_energy_mode = val & MICFIL_PARAM_HWVAD_ENERGY_MODE;
    (*micfil).param.hwvad = val & MICFIL_PARAM_HWVAD;
    (*micfil).param.dc_out_bypass = val & MICFIL_PARAM_DC_OUT_BYPASS;
    (*micfil).param.dc_in_bypass = val & MICFIL_PARAM_DC_IN_BYPASS;
    (*micfil).param.low_power = val & MICFIL_PARAM_LOW_POWER;
    (*micfil).param.fil_out_width = val & MICFIL_PARAM_FIL_OUT_WIDTH;
    (*micfil).param.fifo_ptrwid = (val & MICFIL_PARAM_FIFO_PTRWID_MASK) >> MICFIL_PARAM_FIFO_PTRWID_SHIFT;
    (*micfil).param.npair = (val & MICFIL_PARAM_NPAIR_MASK) >> MICFIL_PARAM_NPAIR_SHIFT;
    0
}

/* The SRES is a self-negated bit which provides the CPU with the
 * capability to initialize the PDM Interface module through the
 * slave-bus interface. This bit always reads as zero, and this
 * bit is only effective when MDIS is cleared
 */
unsafe fn fsl_micfil_reset(dev: *mut device) -> c_int {
    let micfil = dev_get_drvdata(dev);
    let mut ret = regmap_clear_bits((*micfil).regmap, REG_MICFIL_CTRL1, MICFIL_CTRL1_MDIS);
    if ret != 0 { return ret; }
    ret = regmap_set_bits((*micfil).regmap, REG_MICFIL_CTRL1, MICFIL_CTRL1_SRES);
    if ret != 0 { return ret; }
    /*
     * SRES is self-cleared bit, but REG_MICFIL_CTRL1 is defined
     * as non-volatile register, so SRES still remain in regmap
     * cache after set, that every update of REG_MICFIL_CTRL1,
     * software reset happens. so clear it explicitly.
     */
    ret = regmap_clear_bits((*micfil).regmap, REG_MICFIL_CTRL1, MICFIL_CTRL1_SRES);
    if ret != 0 { return ret; }
    /*
     * Set SRES should clear CHnF flags, But even add delay here
     * the CHnF may not be cleared sometimes, so clear CHnF explicitly.
     */
    ret = regmap_write_bits((*micfil).regmap, REG_MICFIL_STAT, 0xFF, 0xFF);
    if ret != 0 { return ret; }
    0
}

unsafe fn fsl_micfil_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let micfil = snd_soc_dai_get_drvdata(dai);
    if micfil.is_null() {
        dev_err((*dai).dev, b"micfil dai priv_data not set\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    if (*micfil).constraint_rates.count > 0 {
        snd_pcm_hw_constraint_list((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &(*micfil).constraint_rates);
    }
    0
}

/* Enable/disable hwvad interrupts */
unsafe fn fsl_micfil_configure_hwvad_interrupts(micfil: *mut fsl_micfil, enable: c_int) -> c_int {
    let vadie_reg = if enable != 0 { MICFIL_VAD0_CTRL1_IE } else { 0 };
    let vaderie_reg = if enable != 0 { MICFIL_VAD0_CTRL1_ERIE } else { 0 };
    /* Voice Activity Detector Error Interruption */
    regmap_update_bits((*micfil).regmap, REG_MICFIL_VAD0_CTRL1, MICFIL_VAD0_CTRL1_ERIE, vaderie_reg);
    /* Voice Activity Detector Interruption */
    regmap_update_bits((*micfil).regmap, REG_MICFIL_VAD0_CTRL1, MICFIL_VAD0_CTRL1_IE, vadie_reg);
    0
}

/* Configuration done only in energy-based initialization mode */
unsafe fn fsl_micfil_init_hwvad_energy_mode(micfil: *mut fsl_micfil) -> c_int {
    regmap_clear_bits((*micfil).regmap, REG_MICFIL_VAD0_CTRL2, MICFIL_VAD0_CTRL2_FRENDIS);
    regmap_clear_bits((*micfil).regmap, REG_MICFIL_VAD0_CTRL2, MICFIL_VAD0_CTRL2_PREFEN);
    regmap_clear_bits((*micfil).regmap, REG_MICFIL_VAD0_SCONFIG, MICFIL_VAD0_SCONFIG_SFILEN);
    regmap_clear_bits((*micfil).regmap, REG_MICFIL_VAD0_SCONFIG, MICFIL_VAD0_SCONFIG_SMAXEN);
    regmap_set_bits((*micfil).regmap, REG_MICFIL_VAD0_NCONFIG, MICFIL_VAD0_NCONFIG_NFILAUT);
    regmap_clear_bits((*micfil).regmap, REG_MICFIL_VAD0_NCONFIG, MICFIL_VAD0_NCONFIG_NMINEN);
    regmap_clear_bits((*micfil).regmap, REG_MICFIL_VAD0_NCONFIG, MICFIL_VAD0_NCONFIG_NDECEN);
    regmap_clear_bits((*micfil).regmap, REG_MICFIL_VAD0_NCONFIG, MICFIL_VAD0_NCONFIG_NOREN);
    0
}

/* Configuration done only in envelope-based initialization mode */
unsafe fn fsl_micfil_init_hwvad_envelope_mode(micfil: *mut fsl_micfil) -> c_int {
    regmap_set_bits((*micfil).regmap, REG_MICFIL_VAD0_CTRL2, MICFIL_VAD0_CTRL2_FRENDIS);
    regmap_set_bits((*micfil).regmap, REG_MICFIL_VAD0_CTRL2, MICFIL_VAD0_CTRL2_PREFEN);
    regmap_set_bits((*micfil).regmap, REG_MICFIL_VAD0_SCONFIG, MICFIL_VAD0_SCONFIG_SFILEN);
    regmap_set_bits((*micfil).regmap, REG_MICFIL_VAD0_SCONFIG, MICFIL_VAD0_SCONFIG_SMAXEN);
    regmap_clear_bits((*micfil).regmap, REG_MICFIL_VAD0_NCONFIG, MICFIL_VAD0_NCONFIG_NFILAUT);
    regmap_set_bits((*micfil).regmap, REG_MICFIL_VAD0_NCONFIG, MICFIL_VAD0_NCONFIG_NMINEN);
    regmap_set_bits((*micfil).regmap, REG_MICFIL_VAD0_NCONFIG, MICFIL_VAD0_NCONFIG_NDECEN);
    regmap_set_bits((*micfil).regmap, REG_MICFIL_VAD0_NCONFIG, MICFIL_VAD0_NCONFIG_NOREN);
    0
}

/*
 * Hardware Voice Active Detection: The HWVAD takes data from the input
 * of a selected PDM microphone to detect if there is any
 * voice activity. When a voice activity is detected, an interrupt could
 * be delivered to the system. Initialization in section 8.4:
 * Can work in two modes:
 *  -> Eneveope-based mode (section 8.4.1)
 *  -> Energy-based mode (section 8.4.2)
 *
 * It is important to remark that the HWVAD detector could be enabled
 * or reset only when the MICFIL isn't running i.e. when the BSY_FIL
 * bit in STAT register is cleared
 */
unsafe fn fsl_micfil_hwvad_enable(micfil: *mut fsl_micfil) -> c_int {
    (*micfil).vad_detected = 0;
    let mut ret = if (*micfil).vad_init_mode == MICFIL_HWVAD_ENVELOPE_MODE {
        fsl_micfil_init_hwvad_envelope_mode(micfil)
    } else {
        fsl_micfil_init_hwvad_energy_mode(micfil)
    };
    if ret != 0 { return ret; }
    regmap_set_bits((*micfil).regmap, REG_MICFIL_VAD0_CTRL1, MICFIL_VAD0_CTRL1_ST10);
    regmap_clear_bits((*micfil).regmap, REG_MICFIL_VAD0_CTRL1, MICFIL_VAD0_CTRL1_ST10);
    ret = fsl_micfil_configure_hwvad_interrupts(micfil, 1);
    if ret != 0 { return ret; }
    regmap_set_bits((*micfil).regmap, REG_MICFIL_VAD0_CTRL1, MICFIL_VAD0_CTRL1_RST);
    regmap_set_bits((*micfil).regmap, REG_MICFIL_VAD0_CTRL1, MICFIL_VAD0_CTRL1_EN);
    0
}

unsafe fn fsl_micfil_hwvad_disable(micfil: *mut fsl_micfil) -> c_int {
    let dev = &mut (*(*micfil).pdev).dev as *mut device;
    regmap_clear_bits((*micfil).regmap, REG_MICFIL_VAD0_CTRL1, MICFIL_VAD0_CTRL1_EN);
    let ret = fsl_micfil_configure_hwvad_interrupts(micfil, 0);
    if ret != 0 { dev_err(dev, b"Failed to disable interrupts\n\0".as_ptr() as *const c_char); }
    ret
}

unsafe fn fsl_micfil_trigger(_substream: *mut snd_pcm_substream, cmd: c_int, dai: *mut snd_soc_dai) -> c_int {
    let micfil = snd_soc_dai_get_drvdata(dai);
    let dev = &mut (*(*micfil).pdev).dev as *mut device;
    let mut ret: c_int;
    if cmd == SNDRV_PCM_TRIGGER_START || cmd == SNDRV_PCM_TRIGGER_RESUME || cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE {
        ret = fsl_micfil_reset(dev);
        if ret != 0 {
            dev_err(dev, b"failed to soft reset\n\0".as_ptr() as *const c_char);
            return ret;
        }
        ret = regmap_update_bits((*micfil).regmap, REG_MICFIL_CTRL1, MICFIL_CTRL1_DISEL, FIELD_PREP(MICFIL_CTRL1_DISEL, MICFIL_CTRL1_DISEL_DMA));
        if ret != 0 { return ret; }
        ret = regmap_set_bits((*micfil).regmap, REG_MICFIL_CTRL1, MICFIL_CTRL1_PDMIEN | MICFIL_CTRL1_ERREN);
        if ret != 0 { return ret; }
        if (*micfil).vad_enabled != 0 && !(*micfil).dec_bypass { fsl_micfil_hwvad_enable(micfil); }
    } else if cmd == SNDRV_PCM_TRIGGER_STOP || cmd == SNDRV_PCM_TRIGGER_SUSPEND || cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH {
        if (*micfil).vad_enabled != 0 && !(*micfil).dec_bypass { fsl_micfil_hwvad_disable(micfil); }
        ret = regmap_clear_bits((*micfil).regmap, REG_MICFIL_CTRL1, MICFIL_CTRL1_PDMIEN | MICFIL_CTRL1_ERREN);
        if ret != 0 { return ret; }
        ret = regmap_update_bits((*micfil).regmap, REG_MICFIL_CTRL1, MICFIL_CTRL1_DISEL, FIELD_PREP(MICFIL_CTRL1_DISEL, MICFIL_CTRL1_DISEL_DISABLE));
        if ret != 0 { return ret; }
    } else {
        return -EINVAL;
    }
    0
}

unsafe fn fsl_micfil_reparent_rootclk(micfil: *mut fsl_micfil, sample_rate: c_uint) -> c_int {
    let dev = &mut (*(*micfil).pdev).dev as *mut device;
    let ratio: u64 = sample_rate as u64;
    let clk = (*micfil).mclk;
    fsl_asoc_reparent_pll_clocks(dev, clk, (*micfil).pll8k_clk, (*micfil).pll11k_clk, ratio);
    let ret = clk_prepare_enable(clk);
    if ret != 0 { return ret; }
    0
}

unsafe fn fsl_micfil_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let micfil = snd_soc_dai_get_drvdata(dai);
    let channels = params_channels(params);
    let format = params_format(params);
    let rate = params_rate(params);
    let clk_div: c_int = 8;
    let osr: c_int = MICFIL_OSR_DEFAULT;
    let mut ret = regmap_clear_bits((*micfil).regmap, REG_MICFIL_CTRL1, MICFIL_CTRL1_PDMIEN);
    if ret != 0 { return ret; }
    ret = regmap_update_bits((*micfil).regmap, REG_MICFIL_CTRL1, 0xFF, (1u32 << channels) - 1);
    if ret != 0 { return ret; }
    ret = fsl_micfil_reparent_rootclk(micfil, rate);
    if ret != 0 { return ret; }
    (*micfil).mclk_flag = true;
    let div_multiply_k = match (*micfil).quality {
        quality::QUALITY_HIGH => clk_div >> 1,
        quality::QUALITY_LOW | quality::QUALITY_VLOW1 => clk_div << 1,
        quality::QUALITY_VLOW2 => clk_div << 2,
        quality::QUALITY_MEDIUM | quality::QUALITY_VLOW0 => clk_div,
    };
    let mclk_rate: c_int;
    if format == SNDRV_PCM_FORMAT_DSD_U32_LE {
        (*micfil).dec_bypass = true;
        mclk_rate = (rate as c_int) * div_multiply_k * 32 * 2;
    } else {
        (*micfil).dec_bypass = false;
        mclk_rate = (rate as c_int) * clk_div * osr * 8;
    }
    ret = clk_set_rate((*micfil).mclk, mclk_rate);
    if ret != 0 { return ret; }
    ret = micfil_set_quality(micfil);
    if ret != 0 { return ret; }
    regmap_update_bits((*micfil).regmap, REG_MICFIL_CTRL2, MICFIL_CTRL2_DEC_BYPASS, if (*micfil).dec_bypass { MICFIL_CTRL2_DEC_BYPASS } else { 0 });
    ret = regmap_update_bits((*micfil).regmap, REG_MICFIL_CTRL2, MICFIL_CTRL2_CLKDIV | MICFIL_CTRL2_CICOSR,
                             FIELD_PREP(MICFIL_CTRL2_CLKDIV, clk_div as c_uint) | FIELD_PREP(MICFIL_CTRL2_CICOSR, (32 - osr) as c_uint));
    regmap_update_bits((*micfil).regmap, REG_MICFIL_VAD0_CTRL1, MICFIL_VAD0_CTRL1_CICOSR, FIELD_PREP(MICFIL_VAD0_CTRL1_CICOSR, (16 - osr) as c_uint));
    regmap_update_bits((*micfil).regmap, REG_MICFIL_VAD0_CTRL1, MICFIL_VAD0_CTRL1_CHSEL, FIELD_PREP(MICFIL_VAD0_CTRL1_CHSEL, channels - 1));
    (*micfil).dma_params_rx.peripheral_config = &mut (*micfil).sdmacfg;
    (*micfil).dma_params_rx.peripheral_size = size_of::<sdma_peripheral_config>();
    (*micfil).sdmacfg.n_fifos_src = channels;
    (*micfil).sdmacfg.sw_done = true;
    (*micfil).dma_params_rx.maxburst = channels * MICFIL_DMA_MAXBURST_RX;
    if (*(*micfil).soc).use_edma { (*micfil).dma_params_rx.maxburst = channels; }
    0
}

unsafe fn fsl_micfil_hw_free(_substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let micfil = snd_soc_dai_get_drvdata(dai);
    clk_disable_unprepare((*micfil).mclk);
    (*micfil).mclk_flag = false;
    0
}

unsafe fn fsl_micfil_dai_probe(cpu_dai: *mut snd_soc_dai) -> c_int {
    let micfil = dev_get_drvdata((*cpu_dai).dev);
    let dev = (*cpu_dai).dev;
    let mut val: c_uint = 0;
    (*micfil).quality = (*(*micfil).soc).default_quality;
    (*micfil).card = (*(*cpu_dai).component).card;
    if (*(*micfil).soc).volume_sx {
        regmap_write((*micfil).regmap, REG_MICFIL_OUT_CTRL, 0x22222222);
    } else {
        let mut max_range = micfil_get_max_range(micfil) as c_uint;
        let mut i = 1;
        while i < (*(*micfil).soc).fifos as c_int {
            max_range |= max_range << 4;
            i += 1;
        }
        regmap_write((*micfil).regmap, REG_MICFIL_OUT_CTRL, max_range);
    }
    let mut i = 0;
    while i < MICFIL_OUTPUT_CHANNELS {
        val |= MICFIL_DC_BYPASS << MICFIL_DC_CHX_SHIFT(i);
        i += 1;
    }
    let mut ret = regmap_update_bits((*micfil).regmap, REG_MICFIL_DC_CTRL, MICFIL_DC_CTRL_CONFIG, val);
    if ret != 0 {
        dev_err(dev, b"failed to set DC Remover mode bits\n\0".as_ptr() as *const c_char);
        return ret;
    }
    (*micfil).dc_remover = MICFIL_DC_BYPASS as c_int;
    if (*(*micfil).soc).use_verid {
        val = 0;
        i = 0;
        while i < MICFIL_OUTPUT_CHANNELS {
            val |= MICFIL_DC_BYPASS << MICFIL_DC_CHX_SHIFT(i);
            i += 1;
        }
        ret = regmap_update_bits((*micfil).regmap, REG_MICFIL_DC_OUT_CTRL, MICFIL_DC_CTRL_CONFIG, val);
        if ret != 0 {
            dev_err(dev, b"failed to set DC OUT Remover mode bits\n\0".as_ptr() as *const c_char);
            return ret;
        }
        (*micfil).dc_out_remover = MICFIL_DC_BYPASS as c_int;
    }
    snd_soc_dai_init_dma_data(cpu_dai, ptr::null_mut(), &mut (*micfil).dma_params_rx);
    ret = regmap_update_bits((*micfil).regmap, REG_MICFIL_FIFO_CTRL, MICFIL_FIFO_CTRL_FIFOWMK,
                             FIELD_PREP(MICFIL_FIFO_CTRL_FIFOWMK, (*(*micfil).soc).fifo_depth - 1));
    if ret != 0 { return ret; }
    0
}

unsafe fn fsl_micfil_component_probe(component: *mut snd_soc_component) -> c_int {
    let micfil = snd_soc_component_get_drvdata(component);
    // snd_soc_add_component_controls(component, fsl_micfil_volume_sx_controls or fsl_micfil_range_controls, ARRAY_SIZE(...));
    // if use_verid: snd_soc_add_component_controls(component, fsl_micfil_dc_out_controls, ARRAY_SIZE(...));
    let _ = micfil;
    0
}

// static const struct snd_soc_dai_ops fsl_micfil_dai_ops = { probe, startup, trigger, hw_params, hw_free };
// static struct snd_soc_dai_driver fsl_micfil_dai = { capture = { "CPU-Capture", 1, 8, SNDRV_PCM_RATE_8000_48000, SNDRV_PCM_FMTBIT_S16_LE }, ops };
// static const struct snd_soc_component_driver fsl_micfil_component = { name = "fsl-micfil-dai", probe, controls, num_controls, legacy_dai_naming = 1 };

/* REGMAP */
static fsl_micfil_reg_defaults: [reg_default; 24] = [
    reg_default { reg: 0, def: 0x00000000 }, reg_default { reg: 0, def: 0x00000000 },
    reg_default { reg: 0, def: 0x00000000 }, reg_default { reg: 0, def: 0x0000001F },
    reg_default { reg: 0, def: 0x00000000 }, reg_default { reg: 0, def: 0x00000000 },
    reg_default { reg: 0, def: 0x00000000 }, reg_default { reg: 0, def: 0x00000000 },
    reg_default { reg: 0, def: 0x00000000 }, reg_default { reg: 0, def: 0x00000000 },
    reg_default { reg: 0, def: 0x00000000 }, reg_default { reg: 0, def: 0x00000000 },
    reg_default { reg: 0, def: 0x00000000 }, reg_default { reg: 0, def: 0x00000000 },
    reg_default { reg: 0, def: 0x00000000 }, reg_default { reg: 0, def: 0x00000000 },
    reg_default { reg: 0, def: 0x00000000 }, reg_default { reg: 0, def: 0x00000000 },
    reg_default { reg: 0, def: 0x000A0000 }, reg_default { reg: 0, def: 0x00000000 },
    reg_default { reg: 0, def: 0x00000000 }, reg_default { reg: 0, def: 0x80000000 },
    reg_default { reg: 0, def: 0x00000000 }, reg_default { reg: 0, def: 0x00000004 },
];

// fsl_micfil_reg_defaults_v2 is the same default table, but DATA registers are REG_MICFIL_DATACHx - 0x4.

unsafe fn fsl_micfil_readable_reg(dev: *mut device, reg: c_uint) -> bool {
    let micfil = dev_get_drvdata(dev);
    let ofs = (*(*micfil).soc).fifo_offset;
    if reg as c_int >= REG_MICFIL_DATACH0 as c_int + ofs && reg as c_int <= REG_MICFIL_DATACH7 as c_int + ofs { return true; }
    if reg == REG_MICFIL_CTRL1 || reg == REG_MICFIL_CTRL2 || reg == REG_MICFIL_STAT ||
       reg == REG_MICFIL_FIFO_CTRL || reg == REG_MICFIL_FIFO_STAT || reg == REG_MICFIL_DC_CTRL ||
       reg == REG_MICFIL_OUT_CTRL || reg == REG_MICFIL_OUT_STAT || reg == REG_MICFIL_VAD0_CTRL1 ||
       reg == REG_MICFIL_VAD0_CTRL2 || reg == REG_MICFIL_VAD0_STAT || reg == REG_MICFIL_VAD0_SCONFIG ||
       reg == REG_MICFIL_VAD0_NCONFIG || reg == REG_MICFIL_VAD0_NDATA || reg == REG_MICFIL_VAD0_ZCD {
        return true;
    }
    if reg == REG_MICFIL_DC_OUT_CTRL || reg == REG_MICFIL_FSYNC_CTRL || reg == REG_MICFIL_VERID || reg == REG_MICFIL_PARAM {
        if (*(*micfil).soc).use_verid { return true; }
    }
    false
}

unsafe fn fsl_micfil_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    let micfil = dev_get_drvdata(dev);
    if reg == REG_MICFIL_CTRL1 || reg == REG_MICFIL_CTRL2 || reg == REG_MICFIL_STAT ||
       reg == REG_MICFIL_FIFO_CTRL || reg == REG_MICFIL_FIFO_STAT || reg == REG_MICFIL_DC_CTRL ||
       reg == REG_MICFIL_OUT_CTRL || reg == REG_MICFIL_OUT_STAT || reg == REG_MICFIL_VAD0_CTRL1 ||
       reg == REG_MICFIL_VAD0_CTRL2 || reg == REG_MICFIL_VAD0_STAT || reg == REG_MICFIL_VAD0_SCONFIG ||
       reg == REG_MICFIL_VAD0_NCONFIG || reg == REG_MICFIL_VAD0_ZCD {
        return true;
    }
    if reg == REG_MICFIL_DC_OUT_CTRL || reg == REG_MICFIL_FSYNC_CTRL {
        if (*(*micfil).soc).use_verid { return true; }
    }
    false
}

unsafe fn fsl_micfil_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    let micfil = dev_get_drvdata(dev);
    let ofs = (*(*micfil).soc).fifo_offset;
    if reg as c_int >= REG_MICFIL_DATACH0 as c_int + ofs && reg as c_int <= REG_MICFIL_DATACH7 as c_int + ofs { return true; }
    reg == REG_MICFIL_STAT || reg == REG_MICFIL_FIFO_STAT || reg == REG_MICFIL_OUT_STAT ||
    reg == REG_MICFIL_VERID || reg == REG_MICFIL_PARAM || reg == REG_MICFIL_VAD0_STAT || reg == REG_MICFIL_VAD0_NDATA
}

// static const struct regmap_config fsl_micfil_regmap_config and _v2 preserve
// reg_bits=32, reg_stride=4, val_bits=32, max_register=REG_MICFIL_VAD0_ZCD,
// defaults, readable/volatile/writeable callbacks and REGCACHE_MAPLE.
/* END OF REGMAP */

unsafe fn micfil_isr(_irq: c_int, devid: *mut c_void) -> irqreturn_t {
    let micfil = devid as *mut fsl_micfil;
    let pdev = (*micfil).pdev;
    let mut stat_reg: u32 = 0;
    let mut fifo_stat_reg: u32 = 0;
    let mut ctrl1_reg: u32 = 0;
    regmap_read((*micfil).regmap, REG_MICFIL_STAT, &mut stat_reg);
    regmap_read((*micfil).regmap, REG_MICFIL_CTRL1, &mut ctrl1_reg);
    regmap_read((*micfil).regmap, REG_MICFIL_FIFO_STAT, &mut fifo_stat_reg);
    let dma_enabled = FIELD_GET(MICFIL_CTRL1_DISEL, ctrl1_reg) == MICFIL_CTRL1_DISEL_DMA;
    let mut i = 0;
    while i < MICFIL_OUTPUT_CHANNELS {
        if stat_reg & MICFIL_STAT_CHXF(i) != 0 {
            dev_dbg(&mut (*pdev).dev, b"Data available in Data Channel %d\n\0".as_ptr() as *const c_char, i);
        }
        if !dma_enabled {
            regmap_write_bits((*micfil).regmap, REG_MICFIL_STAT, MICFIL_STAT_CHXF(i), MICFIL_STAT_CHXF(i));
        }
        i += 1;
    }
    i = 0;
    while i < MICFIL_FIFO_NUM {
        if fifo_stat_reg & MICFIL_FIFO_STAT_FIFOX_OVER(i) != 0 {
            dev_dbg(&mut (*pdev).dev, b"FIFO Overflow Exception flag for channel %d\n\0".as_ptr() as *const c_char, i);
        }
        if fifo_stat_reg & MICFIL_FIFO_STAT_FIFOX_UNDER(i) != 0 {
            dev_dbg(&mut (*pdev).dev, b"FIFO Underflow Exception flag for channel %d\n\0".as_ptr() as *const c_char, i);
        }
        i += 1;
    }
    IRQ_HANDLED
}

unsafe fn micfil_err_isr(_irq: c_int, devid: *mut c_void) -> irqreturn_t {
    let micfil = devid as *mut fsl_micfil;
    let pdev = (*micfil).pdev;
    let mut fifo_stat_reg: u32 = 0;
    let mut out_stat_reg: u32 = 0;
    let mut stat_reg: u32 = 0;
    regmap_read((*micfil).regmap, REG_MICFIL_STAT, &mut stat_reg);
    if stat_reg & MICFIL_STAT_BSY_FIL != 0 { dev_dbg(&mut (*pdev).dev, b"isr: Decimation Filter is running\n\0".as_ptr() as *const c_char); }
    if stat_reg & MICFIL_STAT_FIR_RDY != 0 { dev_dbg(&mut (*pdev).dev, b"isr: FIR Filter Data ready\n\0".as_ptr() as *const c_char); }
    if stat_reg & MICFIL_STAT_LOWFREQF != 0 {
        dev_dbg(&mut (*pdev).dev, b"isr: ipg_clk_app is too low\n\0".as_ptr() as *const c_char);
        regmap_write_bits((*micfil).regmap, REG_MICFIL_STAT, MICFIL_STAT_LOWFREQF, MICFIL_STAT_LOWFREQF);
    }
    regmap_read((*micfil).regmap, REG_MICFIL_FIFO_STAT, &mut fifo_stat_reg);
    regmap_write_bits((*micfil).regmap, REG_MICFIL_FIFO_STAT, fifo_stat_reg, fifo_stat_reg);
    regmap_read((*micfil).regmap, REG_MICFIL_OUT_STAT, &mut out_stat_reg);
    regmap_write_bits((*micfil).regmap, REG_MICFIL_OUT_STAT, out_stat_reg, out_stat_reg);
    IRQ_HANDLED
}

extern "C" {
    fn snd_soc_card_get_kcontrol(card: *mut snd_soc_card, name: *const c_char) -> *mut snd_kcontrol;
    fn snd_ctl_notify(card: *mut c_void, mask: c_uint, id: *const c_uint);
}

unsafe fn voice_detected_fn(_irq: c_int, devid: *mut c_void) -> irqreturn_t {
    let micfil = devid as *mut fsl_micfil;
    if (*micfil).card.is_null() { return IRQ_HANDLED; }
    let kctl = snd_soc_card_get_kcontrol((*micfil).card, b"VAD Detected\0".as_ptr() as *const c_char);
    if kctl.is_null() { return IRQ_HANDLED; }
    if (*micfil).vad_detected != 0 {
        snd_ctl_notify((*(*micfil).card).snd_card, SNDRV_CTL_EVENT_MASK_VALUE, &(*kctl).id);
    }
    IRQ_HANDLED
}

unsafe fn hwvad_isr(_irq: c_int, devid: *mut c_void) -> irqreturn_t {
    let micfil = devid as *mut fsl_micfil;
    let dev = &mut (*(*micfil).pdev).dev as *mut device;
    let mut vad0_reg: u32 = 0;
    regmap_read((*micfil).regmap, REG_MICFIL_VAD0_STAT, &mut vad0_reg);
    if vad0_reg & MICFIL_VAD0_STAT_IF != 0 {
        regmap_write_bits((*micfil).regmap, REG_MICFIL_VAD0_STAT, MICFIL_VAD0_STAT_IF, MICFIL_VAD0_STAT_IF);
        (*micfil).vad_detected = 1;
    }
    let ret = fsl_micfil_hwvad_disable(micfil);
    if ret != 0 { dev_err(dev, b"Failed to disable hwvad\n\0".as_ptr() as *const c_char); }
    IRQ_WAKE_THREAD
}

unsafe fn hwvad_err_isr(_irq: c_int, devid: *mut c_void) -> irqreturn_t {
    let micfil = devid as *mut fsl_micfil;
    let dev = &mut (*(*micfil).pdev).dev as *mut device;
    let mut vad0_reg: u32 = 0;
    regmap_read((*micfil).regmap, REG_MICFIL_VAD0_STAT, &mut vad0_reg);
    if vad0_reg & MICFIL_VAD0_STAT_INSATF != 0 {
        dev_dbg(dev, b"voice activity input overflow/underflow detected\n\0".as_ptr() as *const c_char);
    }
    IRQ_HANDLED
}

unsafe fn fsl_micfil_runtime_suspend(dev: *mut device) -> c_int {
    let micfil = dev_get_drvdata(dev);
    regcache_cache_only((*micfil).regmap, true);
    if (*micfil).mclk_flag { clk_disable_unprepare((*micfil).mclk); }
    clk_disable_unprepare((*micfil).busclk);
    0
}

unsafe fn fsl_micfil_runtime_resume(dev: *mut device) -> c_int {
    let micfil = dev_get_drvdata(dev);
    let mut ret = clk_prepare_enable((*micfil).busclk);
    if ret < 0 { return ret; }
    if (*micfil).mclk_flag {
        ret = clk_prepare_enable((*micfil).mclk);
        if ret < 0 {
            clk_disable_unprepare((*micfil).busclk);
            return ret;
        }
    }
    regcache_cache_only((*micfil).regmap, false);
    regcache_mark_dirty((*micfil).regmap);
    regcache_sync((*micfil).regmap);
    0
}

extern "C" {
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
}

// fsl_micfil_probe translates the C probe routine: allocate fsl_micfil with
// devm_kzalloc, acquire clocks, constrain rates, ioremap resources, initialize
// regmap (v2 for "fsl,imx943-micfil"), read "fsl,dataline", request four IRQs,
// configure DMA address from res->start + REG_MICFIL_DATACH0 + fifo_offset,
// enable runtime PM, read VERID/PARAM, register DMA PCM and ASoC component,
// and on failure run the err_pm_get_sync/err_pm_disable paths.
// The exact implementation depends on kernel allocation/device-tree/request_irq
// structs and macros not present in this isolated source.

unsafe fn fsl_micfil_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
}

// static const struct dev_pm_ops fsl_micfil_pm_ops =
// { RUNTIME_PM_OPS(fsl_micfil_runtime_suspend, fsl_micfil_runtime_resume, NULL)
//   SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume) };
// static struct platform_driver fsl_micfil_driver =
// { .probe = fsl_micfil_probe, .remove = fsl_micfil_remove,
//   .driver = { .name = "fsl-micfil-dai", .pm = pm_ptr(&fsl_micfil_pm_ops),
//               .of_match_table = fsl_micfil_dt_ids } };
// module_platform_driver(fsl_micfil_driver);
// MODULE_AUTHOR("Cosmin-Gabriel Samoila <cosmin.samoila@nxp.com>");
// MODULE_DESCRIPTION("NXP PDM Microphone Interface (MICFIL) driver");
// MODULE_LICENSE("Dual BSD/GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
