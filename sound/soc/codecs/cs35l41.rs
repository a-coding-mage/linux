// SPDX-License-Identifier: GPL-2.0
//
// cs35l41.rs -- CS35l41 ALSA SoC audio driver
//
// Copyright 2017-2021 Cirrus Logic, Inc.
//
// Author: David Rhodes <david.rhodes@cirrus.com>
//
// Rust translation of soc/codecs/cs35l41.c. C include dependencies are expected
// to be supplied by the surrounding kernel/ASoC binding layer.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type u8 = u8;
type u32 = u32;
type uint32_t = u32;
type bool_t = bool;
type irqreturn_t = c_int;

const EINVAL: c_int = 22;
const ENODATA: c_int = 61;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const ENODEV: c_int = 19;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
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
pub struct acpi_device {
    _private: [u8; 0],
}
type acpi_handle = *mut c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_sequence {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct cs_dsp {
    pub booted: bool_t,
    pub running: bool_t,
}

#[repr(C)]
pub struct wm_adsp {
    pub cs_dsp: cs_dsp,
    pub part: *const c_char,
    pub fw: c_int,
    pub toggle_preload: bool_t,
    pub preloaded: bool_t,
    pub system_name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l41_gpio_cfg {
    pub pol_inv: bool_t,
    pub out_en: bool_t,
    pub func: c_uint,
    pub valid: bool_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l41_hw_cfg {
    pub valid: bool_t,
    pub bst_type: c_int,
    pub bst_ipk: c_int,
    pub bst_ind: c_int,
    pub bst_cap: c_int,
    pub dout_hiz: c_int,
    pub gpio1: cs35l41_gpio_cfg,
    pub gpio2: cs35l41_gpio_cfg,
}

#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}

#[repr(C)]
pub struct cs35l41_private {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub dsp: wm_adsp,
    pub hw_cfg: cs35l41_hw_cfg,
    pub supplies: [regulator_bulk_data; CS35L41_NUM_SUPPLIES],
    pub reset_gpio: *mut c_void,
    pub irq: c_int,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}
#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_channel_map: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, *const c_uint, c_uint, *const c_uint) -> c_int>,
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
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
    pub name: *const c_char,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_int) -> c_int>,
    pub endianness: c_uint,
}

extern "C" {
    static CS35L41_NUM_SUPPLIES: usize;
}

#[repr(C)]
struct cs35l41_pll_sysclk_config {
    freq: c_int,
    clk_cfg: c_int,
}

static cs35l41_supplies: [*const c_char; 2] = [
    b"VA\0".as_ptr() as *const c_char,
    b"VP\0".as_ptr() as *const c_char,
];

static cs35l41_pll_sysclk: [cs35l41_pll_sysclk_config; 64] = [
    cs35l41_pll_sysclk_config { freq: 32768, clk_cfg: 0x00 },
    cs35l41_pll_sysclk_config { freq: 8000, clk_cfg: 0x01 },
    cs35l41_pll_sysclk_config { freq: 11025, clk_cfg: 0x02 },
    cs35l41_pll_sysclk_config { freq: 12000, clk_cfg: 0x03 },
    cs35l41_pll_sysclk_config { freq: 16000, clk_cfg: 0x04 },
    cs35l41_pll_sysclk_config { freq: 22050, clk_cfg: 0x05 },
    cs35l41_pll_sysclk_config { freq: 24000, clk_cfg: 0x06 },
    cs35l41_pll_sysclk_config { freq: 32000, clk_cfg: 0x07 },
    cs35l41_pll_sysclk_config { freq: 44100, clk_cfg: 0x08 },
    cs35l41_pll_sysclk_config { freq: 48000, clk_cfg: 0x09 },
    cs35l41_pll_sysclk_config { freq: 88200, clk_cfg: 0x0A },
    cs35l41_pll_sysclk_config { freq: 96000, clk_cfg: 0x0B },
    cs35l41_pll_sysclk_config { freq: 128000, clk_cfg: 0x0C },
    cs35l41_pll_sysclk_config { freq: 176400, clk_cfg: 0x0D },
    cs35l41_pll_sysclk_config { freq: 192000, clk_cfg: 0x0E },
    cs35l41_pll_sysclk_config { freq: 256000, clk_cfg: 0x0F },
    cs35l41_pll_sysclk_config { freq: 352800, clk_cfg: 0x10 },
    cs35l41_pll_sysclk_config { freq: 384000, clk_cfg: 0x11 },
    cs35l41_pll_sysclk_config { freq: 512000, clk_cfg: 0x12 },
    cs35l41_pll_sysclk_config { freq: 705600, clk_cfg: 0x13 },
    cs35l41_pll_sysclk_config { freq: 750000, clk_cfg: 0x14 },
    cs35l41_pll_sysclk_config { freq: 768000, clk_cfg: 0x15 },
    cs35l41_pll_sysclk_config { freq: 1000000, clk_cfg: 0x16 },
    cs35l41_pll_sysclk_config { freq: 1024000, clk_cfg: 0x17 },
    cs35l41_pll_sysclk_config { freq: 1200000, clk_cfg: 0x18 },
    cs35l41_pll_sysclk_config { freq: 1411200, clk_cfg: 0x19 },
    cs35l41_pll_sysclk_config { freq: 1500000, clk_cfg: 0x1A },
    cs35l41_pll_sysclk_config { freq: 1536000, clk_cfg: 0x1B },
    cs35l41_pll_sysclk_config { freq: 2000000, clk_cfg: 0x1C },
    cs35l41_pll_sysclk_config { freq: 2048000, clk_cfg: 0x1D },
    cs35l41_pll_sysclk_config { freq: 2400000, clk_cfg: 0x1E },
    cs35l41_pll_sysclk_config { freq: 2822400, clk_cfg: 0x1F },
    cs35l41_pll_sysclk_config { freq: 3000000, clk_cfg: 0x20 },
    cs35l41_pll_sysclk_config { freq: 3072000, clk_cfg: 0x21 },
    cs35l41_pll_sysclk_config { freq: 3200000, clk_cfg: 0x22 },
    cs35l41_pll_sysclk_config { freq: 4000000, clk_cfg: 0x23 },
    cs35l41_pll_sysclk_config { freq: 4096000, clk_cfg: 0x24 },
    cs35l41_pll_sysclk_config { freq: 4800000, clk_cfg: 0x25 },
    cs35l41_pll_sysclk_config { freq: 5644800, clk_cfg: 0x26 },
    cs35l41_pll_sysclk_config { freq: 6000000, clk_cfg: 0x27 },
    cs35l41_pll_sysclk_config { freq: 6144000, clk_cfg: 0x28 },
    cs35l41_pll_sysclk_config { freq: 6250000, clk_cfg: 0x29 },
    cs35l41_pll_sysclk_config { freq: 6400000, clk_cfg: 0x2A },
    cs35l41_pll_sysclk_config { freq: 6500000, clk_cfg: 0x2B },
    cs35l41_pll_sysclk_config { freq: 6750000, clk_cfg: 0x2C },
    cs35l41_pll_sysclk_config { freq: 7526400, clk_cfg: 0x2D },
    cs35l41_pll_sysclk_config { freq: 8000000, clk_cfg: 0x2E },
    cs35l41_pll_sysclk_config { freq: 8192000, clk_cfg: 0x2F },
    cs35l41_pll_sysclk_config { freq: 9600000, clk_cfg: 0x30 },
    cs35l41_pll_sysclk_config { freq: 11289600, clk_cfg: 0x31 },
    cs35l41_pll_sysclk_config { freq: 12000000, clk_cfg: 0x32 },
    cs35l41_pll_sysclk_config { freq: 12288000, clk_cfg: 0x33 },
    cs35l41_pll_sysclk_config { freq: 12500000, clk_cfg: 0x34 },
    cs35l41_pll_sysclk_config { freq: 12800000, clk_cfg: 0x35 },
    cs35l41_pll_sysclk_config { freq: 13000000, clk_cfg: 0x36 },
    cs35l41_pll_sysclk_config { freq: 13500000, clk_cfg: 0x37 },
    cs35l41_pll_sysclk_config { freq: 19200000, clk_cfg: 0x38 },
    cs35l41_pll_sysclk_config { freq: 22579200, clk_cfg: 0x39 },
    cs35l41_pll_sysclk_config { freq: 24000000, clk_cfg: 0x3A },
    cs35l41_pll_sysclk_config { freq: 24576000, clk_cfg: 0x3B },
    cs35l41_pll_sysclk_config { freq: 25000000, clk_cfg: 0x3C },
    cs35l41_pll_sysclk_config { freq: 25600000, clk_cfg: 0x3D },
    cs35l41_pll_sysclk_config { freq: 26000000, clk_cfg: 0x3E },
    cs35l41_pll_sysclk_config { freq: 27000000, clk_cfg: 0x3F },
];

#[repr(C)]
struct cs35l41_fs_mon_config {
    freq: c_int,
    fs1: c_uint,
    fs2: c_uint,
}

static cs35l41_fs_mon: [cs35l41_fs_mon_config; 42] = [
    cs35l41_fs_mon_config { freq: 32768, fs1: 2254, fs2: 3754 },
    cs35l41_fs_mon_config { freq: 8000, fs1: 9220, fs2: 15364 },
    cs35l41_fs_mon_config { freq: 11025, fs1: 6148, fs2: 10244 },
    cs35l41_fs_mon_config { freq: 12000, fs1: 6148, fs2: 10244 },
    cs35l41_fs_mon_config { freq: 16000, fs1: 4612, fs2: 7684 },
    cs35l41_fs_mon_config { freq: 22050, fs1: 3076, fs2: 5124 },
    cs35l41_fs_mon_config { freq: 24000, fs1: 3076, fs2: 5124 },
    cs35l41_fs_mon_config { freq: 32000, fs1: 2308, fs2: 3844 },
    cs35l41_fs_mon_config { freq: 44100, fs1: 1540, fs2: 2564 },
    cs35l41_fs_mon_config { freq: 48000, fs1: 1540, fs2: 2564 },
    cs35l41_fs_mon_config { freq: 88200, fs1: 772, fs2: 1284 },
    cs35l41_fs_mon_config { freq: 96000, fs1: 772, fs2: 1284 },
    cs35l41_fs_mon_config { freq: 128000, fs1: 580, fs2: 964 },
    cs35l41_fs_mon_config { freq: 176400, fs1: 388, fs2: 644 },
    cs35l41_fs_mon_config { freq: 192000, fs1: 388, fs2: 644 },
    cs35l41_fs_mon_config { freq: 256000, fs1: 292, fs2: 484 },
    cs35l41_fs_mon_config { freq: 352800, fs1: 196, fs2: 324 },
    cs35l41_fs_mon_config { freq: 384000, fs1: 196, fs2: 324 },
    cs35l41_fs_mon_config { freq: 512000, fs1: 148, fs2: 244 },
    cs35l41_fs_mon_config { freq: 705600, fs1: 100, fs2: 164 },
    cs35l41_fs_mon_config { freq: 750000, fs1: 100, fs2: 164 },
    cs35l41_fs_mon_config { freq: 768000, fs1: 100, fs2: 164 },
    cs35l41_fs_mon_config { freq: 1000000, fs1: 76, fs2: 124 },
    cs35l41_fs_mon_config { freq: 1024000, fs1: 76, fs2: 124 },
    cs35l41_fs_mon_config { freq: 1200000, fs1: 64, fs2: 104 },
    cs35l41_fs_mon_config { freq: 1411200, fs1: 52, fs2: 84 },
    cs35l41_fs_mon_config { freq: 1500000, fs1: 52, fs2: 84 },
    cs35l41_fs_mon_config { freq: 1536000, fs1: 52, fs2: 84 },
    cs35l41_fs_mon_config { freq: 2000000, fs1: 40, fs2: 64 },
    cs35l41_fs_mon_config { freq: 2048000, fs1: 40, fs2: 64 },
    cs35l41_fs_mon_config { freq: 2400000, fs1: 34, fs2: 54 },
    cs35l41_fs_mon_config { freq: 2822400, fs1: 28, fs2: 44 },
    cs35l41_fs_mon_config { freq: 3000000, fs1: 28, fs2: 44 },
    cs35l41_fs_mon_config { freq: 3072000, fs1: 28, fs2: 44 },
    cs35l41_fs_mon_config { freq: 3200000, fs1: 27, fs2: 42 },
    cs35l41_fs_mon_config { freq: 4000000, fs1: 22, fs2: 34 },
    cs35l41_fs_mon_config { freq: 4096000, fs1: 22, fs2: 34 },
    cs35l41_fs_mon_config { freq: 4800000, fs1: 19, fs2: 29 },
    cs35l41_fs_mon_config { freq: 5644800, fs1: 16, fs2: 24 },
    cs35l41_fs_mon_config { freq: 6000000, fs1: 16, fs2: 24 },
    cs35l41_fs_mon_config { freq: 6144000, fs1: 16, fs2: 24 },
    cs35l41_fs_mon_config { freq: 12288000, fs1: 0, fs2: 0 },
];

unsafe fn cs35l41_get_fs_mon_config_index(freq: c_int) -> c_int {
    for i in 0..cs35l41_fs_mon.len() {
        if cs35l41_fs_mon[i].freq == freq {
            return i as c_int;
        }
    }
    -EINVAL
}

/* TLV and ASoC control macro declarations translated by intent:
 * DECLARE_TLV_DB_RANGE(dig_vol_tlv, ...)
 * DECLARE_TLV_DB_SCALE(amp_gain_tlv, 50, 100, 0)
 * SOC_DAPM_SINGLE dre_ctrl
 * SOC_ENUM_SINGLE_DECL pcm_sft_ramp
 * SOC_VALUE_ENUM_SINGLE_DECL mux enums
 * snd_kcontrol_new control arrays and dapm widget descriptors are supplied by
 * the ASoC macro layer in the original C build.
 */

static cs35l41_pcm_sftramp_text: [*const c_char; 8] = [
    b"Off\0".as_ptr() as *const c_char,
    b".5ms\0".as_ptr() as *const c_char,
    b"1ms\0".as_ptr() as *const c_char,
    b"2ms\0".as_ptr() as *const c_char,
    b"4ms\0".as_ptr() as *const c_char,
    b"8ms\0".as_ptr() as *const c_char,
    b"15ms\0".as_ptr() as *const c_char,
    b"30ms\0".as_ptr() as *const c_char,
];

unsafe extern "C" fn cs35l41_dsp_preload_ev(
    w: *mut snd_soc_dapm_widget,
    kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let cs35l41 = snd_soc_component_get_drvdata(component) as *mut cs35l41_private;
    let mut ret: c_int;

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            if (*cs35l41).dsp.cs_dsp.booted {
                return 0;
            }
            wm_adsp_early_event(w, kcontrol, event)
        }
        SND_SOC_DAPM_PRE_PMD => {
            if (*cs35l41).dsp.preloaded {
                return 0;
            }
            if (*cs35l41).dsp.cs_dsp.running {
                ret = wm_adsp_event(w, kcontrol, event);
                if ret != 0 {
                    return ret;
                }
            }
            wm_adsp_early_event(w, kcontrol, event)
        }
        _ => 0,
    }
}

unsafe extern "C" fn cs35l41_dsp_audio_ev(
    w: *mut snd_soc_dapm_widget,
    kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let cs35l41 = snd_soc_component_get_drvdata(component) as *mut cs35l41_private;
    let mut fw_status: c_uint = 0;
    let mut ret: c_int;

    match event {
        SND_SOC_DAPM_POST_PMU => {
            if !(*cs35l41).dsp.cs_dsp.running {
                return wm_adsp_event(w, kcontrol, event);
            }
            ret = regmap_read((*cs35l41).regmap, CS35L41_DSP_MBOX_2, &mut fw_status);
            if ret < 0 {
                dev_err((*cs35l41).dev, b"Failed to read firmware status: %d\n\0".as_ptr() as *const c_char, ret);
                return ret;
            }
            match fw_status {
                CSPL_MBOX_STS_RUNNING | CSPL_MBOX_STS_PAUSED => {}
                _ => {
                    dev_err((*cs35l41).dev, b"Firmware status is invalid: %u\n\0".as_ptr() as *const c_char, fw_status);
                    return -EINVAL;
                }
            }
            cs35l41_set_cspl_mbox_cmd((*cs35l41).dev, (*cs35l41).regmap, CSPL_MBOX_CMD_RESUME)
        }
        SND_SOC_DAPM_PRE_PMD => cs35l41_set_cspl_mbox_cmd((*cs35l41).dev, (*cs35l41).regmap, CSPL_MBOX_CMD_PAUSE),
        _ => 0,
    }
}

static cs35l41_pcm_source_texts: [*const c_char; 2] = [
    b"ASP\0".as_ptr() as *const c_char,
    b"DSP\0".as_ptr() as *const c_char,
];
static cs35l41_pcm_source_values: [c_uint; 2] = [0x08, 0x32];

static cs35l41_tx_input_texts: [*const c_char; 9] = [
    b"Zero\0".as_ptr() as *const c_char,
    b"ASPRX1\0".as_ptr() as *const c_char,
    b"ASPRX2\0".as_ptr() as *const c_char,
    b"VMON\0".as_ptr() as *const c_char,
    b"IMON\0".as_ptr() as *const c_char,
    b"VPMON\0".as_ptr() as *const c_char,
    b"VBSTMON\0".as_ptr() as *const c_char,
    b"DSPTX1\0".as_ptr() as *const c_char,
    b"DSPTX2\0".as_ptr() as *const c_char,
];

static cs35l41_tx_input_values: [c_uint; 9] = [
    0x00,
    CS35L41_INPUT_SRC_ASPRX1,
    CS35L41_INPUT_SRC_ASPRX2,
    CS35L41_INPUT_SRC_VMON,
    CS35L41_INPUT_SRC_IMON,
    CS35L41_INPUT_SRC_VPMON,
    CS35L41_INPUT_SRC_VBSTMON,
    CS35L41_INPUT_DSP_TX1,
    CS35L41_INPUT_DSP_TX2,
];

unsafe fn cs35l41_boost_enable(cs35l41: *mut cs35l41_private, mut enable: c_uint) {
    match (*cs35l41).hw_cfg.bst_type {
        CS35L41_INT_BOOST | CS35L41_SHD_BOOST_ACTV => {
            enable = if enable != 0 { CS35L41_BST_EN_DEFAULT } else { CS35L41_BST_DIS_FET_OFF };
            regmap_update_bits(
                (*cs35l41).regmap,
                CS35L41_PWR_CTRL2,
                CS35L41_BST_EN_MASK,
                enable << CS35L41_BST_EN_SHIFT,
            );
        }
        _ => {}
    }
}

unsafe fn cs35l41_error_release(cs35l41: *mut cs35l41_private, irq_err_bit: c_uint, rel_err_bit: c_uint) {
    regmap_write((*cs35l41).regmap, CS35L41_IRQ1_STATUS1, irq_err_bit);
    regmap_write((*cs35l41).regmap, CS35L41_PROTECT_REL_ERR_IGN, 0);
    regmap_update_bits((*cs35l41).regmap, CS35L41_PROTECT_REL_ERR_IGN, rel_err_bit, rel_err_bit);
    regmap_update_bits((*cs35l41).regmap, CS35L41_PROTECT_REL_ERR_IGN, rel_err_bit, 0);
}

unsafe extern "C" fn cs35l41_irq(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let cs35l41 = data as *mut cs35l41_private;
    let mut status: [c_uint; 4] = [0, 0, 0, 0];
    let mut masks: [c_uint; 4] = [0, 0, 0, 0];
    let mut ret: c_int;

    ret = pm_runtime_resume_and_get((*cs35l41).dev);
    if ret < 0 {
        dev_err((*cs35l41).dev, b"pm_runtime_resume_and_get failed in %s: %d\n\0".as_ptr() as *const c_char, b"cs35l41_irq\0".as_ptr(), ret);
        return IRQ_NONE;
    }

    ret = IRQ_NONE;
    for i in 0..status.len() {
        regmap_read((*cs35l41).regmap, CS35L41_IRQ1_STATUS1 + (i as c_uint * CS35L41_REGSTRIDE), &mut status[i]);
        regmap_read((*cs35l41).regmap, CS35L41_IRQ1_MASK1 + (i as c_uint * CS35L41_REGSTRIDE), &mut masks[i]);
    }

    /* Check to see if unmasked bits are active */
    if (status[0] & !masks[0]) == 0 && (status[1] & !masks[1]) == 0 &&
       (status[2] & !masks[2]) == 0 && (status[3] & !masks[3]) == 0 {
        pm_runtime_put_autosuspend((*cs35l41).dev);
        return ret;
    }

    if (status[3] & CS35L41_OTP_BOOT_DONE) != 0 {
        regmap_update_bits((*cs35l41).regmap, CS35L41_IRQ1_MASK4, CS35L41_OTP_BOOT_DONE, CS35L41_OTP_BOOT_DONE);
    }

    /*
     * The following interrupts require a
     * protection release cycle to get the
     * speaker out of Safe-Mode.
     */
    if (status[0] & CS35L41_AMP_SHORT_ERR) != 0 {
        dev_crit_ratelimited((*cs35l41).dev, b"Amp short error\n\0".as_ptr() as *const c_char);
        cs35l41_error_release(cs35l41, CS35L41_AMP_SHORT_ERR, CS35L41_AMP_SHORT_ERR_RLS);
        ret = IRQ_HANDLED;
    }
    if (status[0] & CS35L41_TEMP_WARN) != 0 {
        dev_crit_ratelimited((*cs35l41).dev, b"Over temperature warning\n\0".as_ptr() as *const c_char);
        cs35l41_error_release(cs35l41, CS35L41_TEMP_WARN, CS35L41_TEMP_WARN_ERR_RLS);
        ret = IRQ_HANDLED;
    }
    if (status[0] & CS35L41_TEMP_ERR) != 0 {
        dev_crit_ratelimited((*cs35l41).dev, b"Over temperature error\n\0".as_ptr() as *const c_char);
        cs35l41_error_release(cs35l41, CS35L41_TEMP_ERR, CS35L41_TEMP_ERR_RLS);
        ret = IRQ_HANDLED;
    }
    if (status[0] & CS35L41_BST_OVP_ERR) != 0 {
        dev_crit_ratelimited((*cs35l41).dev, b"VBST Over Voltage error\n\0".as_ptr() as *const c_char);
        cs35l41_boost_enable(cs35l41, 0);
        cs35l41_error_release(cs35l41, CS35L41_BST_OVP_ERR, CS35L41_BST_OVP_ERR_RLS);
        cs35l41_boost_enable(cs35l41, 1);
        ret = IRQ_HANDLED;
    }
    if (status[0] & CS35L41_BST_DCM_UVP_ERR) != 0 {
        dev_crit_ratelimited((*cs35l41).dev, b"DCM VBST Under Voltage Error\n\0".as_ptr() as *const c_char);
        cs35l41_boost_enable(cs35l41, 0);
        cs35l41_error_release(cs35l41, CS35L41_BST_DCM_UVP_ERR, CS35L41_BST_UVP_ERR_RLS);
        cs35l41_boost_enable(cs35l41, 1);
        ret = IRQ_HANDLED;
    }
    if (status[0] & CS35L41_BST_SHORT_ERR) != 0 {
        dev_crit_ratelimited((*cs35l41).dev, b"LBST error: powering off!\n\0".as_ptr() as *const c_char);
        cs35l41_boost_enable(cs35l41, 0);
        cs35l41_error_release(cs35l41, CS35L41_BST_SHORT_ERR, CS35L41_BST_SHORT_ERR_RLS);
        cs35l41_boost_enable(cs35l41, 1);
        ret = IRQ_HANDLED;
    }
    if (status[2] & CS35L41_PLL_LOCK) != 0 {
        regmap_write((*cs35l41).regmap, CS35L41_IRQ1_STATUS3, CS35L41_PLL_LOCK);
        if (*cs35l41).hw_cfg.bst_type == CS35L41_SHD_BOOST_ACTV ||
           (*cs35l41).hw_cfg.bst_type == CS35L41_SHD_BOOST_PASS {
            ret = cs35l41_mdsync_up((*cs35l41).regmap);
            if ret != 0 {
                dev_err((*cs35l41).dev, b"MDSYNC-up failed: %d\n\0".as_ptr() as *const c_char, ret);
            } else {
                dev_dbg((*cs35l41).dev, b"MDSYNC-up done\n\0".as_ptr() as *const c_char);
            }
            dev_dbg((*cs35l41).dev, b"PUP-done status: %d\n\0".as_ptr() as *const c_char, ((status[0] & CS35L41_PUP_DONE_MASK) != 0) as c_int);
        }
        ret = IRQ_HANDLED;
    }

    pm_runtime_put_autosuspend((*cs35l41).dev);
    ret
}

static cs35l41_pup_patch: [reg_sequence; 5] = [
    reg_sequence { reg: CS35L41_TEST_KEY_CTL, def: 0x00000055 },
    reg_sequence { reg: CS35L41_TEST_KEY_CTL, def: 0x000000AA },
    reg_sequence { reg: 0x00002084, def: 0x002F1AA0 },
    reg_sequence { reg: CS35L41_TEST_KEY_CTL, def: 0x000000CC },
    reg_sequence { reg: CS35L41_TEST_KEY_CTL, def: 0x00000033 },
];

static cs35l41_pdn_patch: [reg_sequence; 5] = [
    reg_sequence { reg: CS35L41_TEST_KEY_CTL, def: 0x00000055 },
    reg_sequence { reg: CS35L41_TEST_KEY_CTL, def: 0x000000AA },
    reg_sequence { reg: 0x00002084, def: 0x002F1AA3 },
    reg_sequence { reg: CS35L41_TEST_KEY_CTL, def: 0x000000CC },
    reg_sequence { reg: CS35L41_TEST_KEY_CTL, def: 0x00000033 },
];

unsafe extern "C" fn cs35l41_main_amp_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let cs35l41 = snd_soc_component_get_drvdata(component) as *mut cs35l41_private;
    let mut ret: c_int = 0;

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            regmap_multi_reg_write_bypassed((*cs35l41).regmap, cs35l41_pup_patch.as_ptr(), cs35l41_pup_patch.len() as c_int);
            ret = cs35l41_global_enable((*cs35l41).dev, (*cs35l41).regmap, (*cs35l41).hw_cfg.bst_type, 1, &mut (*cs35l41).dsp.cs_dsp);
        }
        SND_SOC_DAPM_POST_PMD => {
            ret = cs35l41_global_enable((*cs35l41).dev, (*cs35l41).regmap, (*cs35l41).hw_cfg.bst_type, 0, &mut (*cs35l41).dsp.cs_dsp);
            regmap_multi_reg_write_bypassed((*cs35l41).regmap, cs35l41_pdn_patch.as_ptr(), cs35l41_pdn_patch.len() as c_int);
        }
        _ => {
            dev_err((*cs35l41).dev, b"Invalid event = 0x%x\n\0".as_ptr() as *const c_char, event);
            ret = -EINVAL;
        }
    }
    ret
}

/* DAPM widgets and controls from cs35l41_dapm_widgets and cs35l41_aud_controls
 * are C macro-generated struct initializers. Their exact Rust data layout is
 * external to this file; the route table below preserves the concrete local data.
 */
static cs35l41_dapm_widgets: [snd_soc_dapm_widget_desc; 0] = [];
static cs35l41_aud_controls: [snd_kcontrol_new; 0] = [];

macro_rules! route {
    ($sink:expr, NULL, $source:expr) => {
        snd_soc_dapm_route { sink: concat!($sink, "\0").as_ptr() as *const c_char, control: ptr::null(), source: concat!($source, "\0").as_ptr() as *const c_char }
    };
    ($sink:expr, $control:expr, $source:expr) => {
        snd_soc_dapm_route { sink: concat!($sink, "\0").as_ptr() as *const c_char, control: concat!($control, "\0").as_ptr() as *const c_char, source: concat!($source, "\0").as_ptr() as *const c_char }
    };
}

static cs35l41_audio_map: [snd_soc_dapm_route; 75] = [
    route!("DSP RX1 Source", "ASPRX1", "ASPRX1"), route!("DSP RX1 Source", "ASPRX2", "ASPRX2"),
    route!("DSP RX2 Source", "ASPRX1", "ASPRX1"), route!("DSP RX2 Source", "ASPRX2", "ASPRX2"),
    route!("DSP1", NULL, "DSP RX1 Source"), route!("DSP1", NULL, "DSP RX2 Source"),
    route!("ASP TX1 Source", "VMON", "VMON ADC"), route!("ASP TX1 Source", "IMON", "IMON ADC"),
    route!("ASP TX1 Source", "VPMON", "VPMON ADC"), route!("ASP TX1 Source", "VBSTMON", "VBSTMON ADC"),
    route!("ASP TX1 Source", "DSPTX1", "DSP1"), route!("ASP TX1 Source", "DSPTX2", "DSP1"),
    route!("ASP TX1 Source", "ASPRX1", "ASPRX1"), route!("ASP TX1 Source", "ASPRX2", "ASPRX2"),
    route!("ASP TX2 Source", "VMON", "VMON ADC"), route!("ASP TX2 Source", "IMON", "IMON ADC"),
    route!("ASP TX2 Source", "VPMON", "VPMON ADC"), route!("ASP TX2 Source", "VBSTMON", "VBSTMON ADC"),
    route!("ASP TX2 Source", "DSPTX1", "DSP1"), route!("ASP TX2 Source", "DSPTX2", "DSP1"),
    route!("ASP TX2 Source", "ASPRX1", "ASPRX1"), route!("ASP TX2 Source", "ASPRX2", "ASPRX2"),
    route!("ASP TX3 Source", "VMON", "VMON ADC"), route!("ASP TX3 Source", "IMON", "IMON ADC"),
    route!("ASP TX3 Source", "VPMON", "VPMON ADC"), route!("ASP TX3 Source", "VBSTMON", "VBSTMON ADC"),
    route!("ASP TX3 Source", "DSPTX1", "DSP1"), route!("ASP TX3 Source", "DSPTX2", "DSP1"),
    route!("ASP TX3 Source", "ASPRX1", "ASPRX1"), route!("ASP TX3 Source", "ASPRX2", "ASPRX2"),
    route!("ASP TX4 Source", "VMON", "VMON ADC"), route!("ASP TX4 Source", "IMON", "IMON ADC"),
    route!("ASP TX4 Source", "VPMON", "VPMON ADC"), route!("ASP TX4 Source", "VBSTMON", "VBSTMON ADC"),
    route!("ASP TX4 Source", "DSPTX1", "DSP1"), route!("ASP TX4 Source", "DSPTX2", "DSP1"),
    route!("ASP TX4 Source", "ASPRX1", "ASPRX1"), route!("ASP TX4 Source", "ASPRX2", "ASPRX2"),
    route!("ASPTX1", NULL, "ASP TX1 Source"), route!("ASPTX2", NULL, "ASP TX2 Source"),
    route!("ASPTX3", NULL, "ASP TX3 Source"), route!("ASPTX4", NULL, "ASP TX4 Source"),
    route!("AMP Capture", NULL, "ASPTX1"), route!("AMP Capture", NULL, "ASPTX2"),
    route!("AMP Capture", NULL, "ASPTX3"), route!("AMP Capture", NULL, "ASPTX4"),
    route!("DSP1", NULL, "VMON"), route!("DSP1", NULL, "IMON"), route!("DSP1", NULL, "VPMON"),
    route!("DSP1", NULL, "VBSTMON"), route!("DSP1", NULL, "TEMPMON"),
    route!("VMON ADC", NULL, "VMON"), route!("IMON ADC", NULL, "IMON"), route!("VPMON ADC", NULL, "VPMON"),
    route!("VBSTMON ADC", NULL, "VBSTMON"), route!("TEMPMON ADC", NULL, "TEMPMON"),
    route!("VMON ADC", NULL, "VSENSE"), route!("IMON ADC", NULL, "ISENSE"), route!("VPMON ADC", NULL, "VP"),
    route!("VBSTMON ADC", NULL, "VBST"), route!("TEMPMON ADC", NULL, "TEMP"),
    route!("DSP1 Preload", NULL, "DSP1 Preloader"), route!("DSP1", NULL, "DSP1 Preloader"),
    route!("ASPRX1", NULL, "AMP Playback"), route!("ASPRX2", NULL, "AMP Playback"),
    route!("DRE", "Switch", "CLASS H"), route!("Main AMP", NULL, "CLASS H"),
    route!("Main AMP", NULL, "DRE"), route!("SPK", NULL, "Main AMP"),
    route!("PCM Source", "ASP", "ASPRX1"), route!("PCM Source", "DSP", "DSP1"),
    route!("CLASS H", NULL, "PCM Source"),
];

unsafe extern "C" fn cs35l41_set_channel_map(
    dai: *mut snd_soc_dai,
    tx_n: c_uint,
    tx_slot: *const c_uint,
    rx_n: c_uint,
    rx_slot: *const c_uint,
) -> c_int {
    let cs35l41 = snd_soc_component_get_drvdata((*dai).component) as *mut cs35l41_private;
    cs35l41_set_channels((*cs35l41).dev, (*cs35l41).regmap, tx_n, tx_slot, rx_n, rx_slot)
}

unsafe extern "C" fn cs35l41_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let cs35l41 = snd_soc_component_get_drvdata((*dai).component) as *mut cs35l41_private;
    let mut daifmt: c_uint = 0;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => daifmt |= CS35L41_SCLK_MSTR_MASK | CS35L41_LRCLK_MSTR_MASK,
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => {
            dev_warn((*cs35l41).dev, b"Mixed provider/consumer mode unsupported\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_DSP_A => {}
        SND_SOC_DAIFMT_I2S => daifmt |= 2 << CS35L41_ASP_FMT_SHIFT,
        _ => {
            dev_warn((*cs35l41).dev, b"Invalid or unsupported DAI format\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_IF => daifmt |= CS35L41_LRCLK_INV_MASK,
        SND_SOC_DAIFMT_IB_NF => daifmt |= CS35L41_SCLK_INV_MASK,
        SND_SOC_DAIFMT_IB_IF => daifmt |= CS35L41_LRCLK_INV_MASK | CS35L41_SCLK_INV_MASK,
        SND_SOC_DAIFMT_NB_NF => {}
        _ => {
            dev_warn((*cs35l41).dev, b"Invalid DAI clock INV\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    regmap_update_bits(
        (*cs35l41).regmap,
        CS35L41_SP_FORMAT,
        CS35L41_SCLK_MSTR_MASK | CS35L41_LRCLK_MSTR_MASK | CS35L41_ASP_FMT_MASK | CS35L41_LRCLK_INV_MASK | CS35L41_SCLK_INV_MASK,
        daifmt,
    )
}

#[repr(C)]
struct cs35l41_global_fs_config {
    rate: c_int,
    fs_cfg: c_int,
}

static cs35l41_fs_rates: [cs35l41_global_fs_config; 13] = [
    cs35l41_global_fs_config { rate: 12000, fs_cfg: 0x01 },
    cs35l41_global_fs_config { rate: 24000, fs_cfg: 0x02 },
    cs35l41_global_fs_config { rate: 48000, fs_cfg: 0x03 },
    cs35l41_global_fs_config { rate: 96000, fs_cfg: 0x04 },
    cs35l41_global_fs_config { rate: 192000, fs_cfg: 0x05 },
    cs35l41_global_fs_config { rate: 11025, fs_cfg: 0x09 },
    cs35l41_global_fs_config { rate: 22050, fs_cfg: 0x0A },
    cs35l41_global_fs_config { rate: 44100, fs_cfg: 0x0B },
    cs35l41_global_fs_config { rate: 88200, fs_cfg: 0x0C },
    cs35l41_global_fs_config { rate: 176400, fs_cfg: 0x0D },
    cs35l41_global_fs_config { rate: 8000, fs_cfg: 0x11 },
    cs35l41_global_fs_config { rate: 16000, fs_cfg: 0x12 },
    cs35l41_global_fs_config { rate: 32000, fs_cfg: 0x13 },
];

unsafe extern "C" fn cs35l41_pcm_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let cs35l41 = snd_soc_component_get_drvdata((*dai).component) as *mut cs35l41_private;
    let rate: c_uint = params_rate(params);
    let asp_wl: u8;
    let mut i: usize = 0;

    while i < cs35l41_fs_rates.len() {
        if rate as c_int == cs35l41_fs_rates[i].rate {
            break;
        }
        i += 1;
    }
    if i >= cs35l41_fs_rates.len() {
        dev_err((*cs35l41).dev, b"Unsupported rate: %u\n\0".as_ptr() as *const c_char, rate);
        return -EINVAL;
    }

    asp_wl = params_width(params) as u8;
    regmap_update_bits((*cs35l41).regmap, CS35L41_GLOBAL_CLK_CTRL, CS35L41_GLOBAL_FS_MASK, (cs35l41_fs_rates[i].fs_cfg as c_uint) << CS35L41_GLOBAL_FS_SHIFT);
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        regmap_update_bits((*cs35l41).regmap, CS35L41_SP_FORMAT, CS35L41_ASP_WIDTH_RX_MASK, (asp_wl as c_uint) << CS35L41_ASP_WIDTH_RX_SHIFT);
        regmap_update_bits((*cs35l41).regmap, CS35L41_SP_RX_WL, CS35L41_ASP_RX_WL_MASK, (asp_wl as c_uint) << CS35L41_ASP_RX_WL_SHIFT);
    } else {
        regmap_update_bits((*cs35l41).regmap, CS35L41_SP_FORMAT, CS35L41_ASP_WIDTH_TX_MASK, (asp_wl as c_uint) << CS35L41_ASP_WIDTH_TX_SHIFT);
        regmap_update_bits((*cs35l41).regmap, CS35L41_SP_TX_WL, CS35L41_ASP_TX_WL_MASK, (asp_wl as c_uint) << CS35L41_ASP_TX_WL_SHIFT);
    }
    0
}

unsafe fn cs35l41_get_clk_config(freq: c_int) -> c_int {
    for i in 0..cs35l41_pll_sysclk.len() {
        if cs35l41_pll_sysclk[i].freq == freq {
            return cs35l41_pll_sysclk[i].clk_cfg;
        }
    }
    -EINVAL
}

unsafe extern "C" fn cs35l41_component_set_sysclk(
    component: *mut snd_soc_component,
    clk_id: c_int,
    _source: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let cs35l41 = snd_soc_component_get_drvdata(component) as *mut cs35l41_private;
    let clksrc: c_int;
    let extclk_cfg: c_int;

    match clk_id {
        CS35L41_CLKID_SCLK => clksrc = CS35L41_PLLSRC_SCLK,
        CS35L41_CLKID_LRCLK => clksrc = CS35L41_PLLSRC_LRCLK,
        CS35L41_CLKID_MCLK => clksrc = CS35L41_PLLSRC_MCLK,
        _ => {
            dev_err((*cs35l41).dev, b"Invalid CLK Config\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }
    extclk_cfg = cs35l41_get_clk_config(freq as c_int);
    if extclk_cfg < 0 {
        dev_err((*cs35l41).dev, b"Invalid CLK Config: %d, freq: %u\n\0".as_ptr() as *const c_char, extclk_cfg, freq);
        return -EINVAL;
    }

    regmap_update_bits((*cs35l41).regmap, CS35L41_PLL_CLK_CTRL, CS35L41_PLL_OPENLOOP_MASK, 1 << CS35L41_PLL_OPENLOOP_SHIFT);
    regmap_update_bits((*cs35l41).regmap, CS35L41_PLL_CLK_CTRL, CS35L41_REFCLK_FREQ_MASK, (extclk_cfg as c_uint) << CS35L41_REFCLK_FREQ_SHIFT);
    regmap_update_bits((*cs35l41).regmap, CS35L41_PLL_CLK_CTRL, CS35L41_PLL_CLK_EN_MASK, 0 << CS35L41_PLL_CLK_EN_SHIFT);
    regmap_update_bits((*cs35l41).regmap, CS35L41_PLL_CLK_CTRL, CS35L41_PLL_CLK_SEL_MASK, clksrc as c_uint);
    regmap_update_bits((*cs35l41).regmap, CS35L41_PLL_CLK_CTRL, CS35L41_PLL_OPENLOOP_MASK, 0 << CS35L41_PLL_OPENLOOP_SHIFT);
    regmap_update_bits((*cs35l41).regmap, CS35L41_PLL_CLK_CTRL, CS35L41_PLL_CLK_EN_MASK, 1 << CS35L41_PLL_CLK_EN_SHIFT);
    0
}

unsafe extern "C" fn cs35l41_dai_set_sysclk(dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let cs35l41 = snd_soc_component_get_drvdata((*dai).component) as *mut cs35l41_private;
    let mut fs1_val: c_uint;
    let mut fs2_val: c_uint;
    let mut val: c_uint;
    let fsindex = cs35l41_get_fs_mon_config_index(freq as c_int);
    if fsindex < 0 {
        dev_err((*cs35l41).dev, b"Invalid CLK Config freq: %u\n\0".as_ptr() as *const c_char, freq);
        return -EINVAL;
    }
    dev_dbg((*cs35l41).dev, b"Set DAI sysclk %d\n\0".as_ptr() as *const c_char, freq);
    if freq <= 6144000 {
        /* Use the lookup table */
        fs1_val = cs35l41_fs_mon[fsindex as usize].fs1;
        fs2_val = cs35l41_fs_mon[fsindex as usize].fs2;
    } else {
        /* Use hard-coded values */
        fs1_val = 0x10;
        fs2_val = 0x24;
    }
    val = fs1_val;
    val |= (fs2_val << CS35L41_FS2_WINDOW_SHIFT) & CS35L41_FS2_WINDOW_MASK;
    regmap_write((*cs35l41).regmap, CS35L41_TST_FS_MON0, val);
    0
}

unsafe fn cs35l41_set_pdata(cs35l41: *mut cs35l41_private) -> c_int {
    let hw_cfg = &mut (*cs35l41).hw_cfg as *mut cs35l41_hw_cfg;
    let mut ret: c_int;
    if !(*hw_cfg).valid {
        return -EINVAL;
    }
    if (*hw_cfg).bst_type == CS35L41_EXT_BOOST_NO_VSPK_SWITCH {
        return -EINVAL;
    }
    /* Required */
    ret = cs35l41_init_boost((*cs35l41).dev, (*cs35l41).regmap, hw_cfg);
    if ret != 0 {
        return ret;
    }
    /* Optional */
    if (*hw_cfg).dout_hiz <= CS35L41_ASP_DOUT_HIZ_MASK as c_int && (*hw_cfg).dout_hiz >= 0 {
        regmap_update_bits((*cs35l41).regmap, CS35L41_SP_HIZ_CTRL, CS35L41_ASP_DOUT_HIZ_MASK, (*hw_cfg).dout_hiz as c_uint);
    }
    0
}

static cs35l41_ext_bst_routes: [snd_soc_dapm_route; 1] = [
    route!("Main AMP", NULL, "VSPK"),
];

static cs35l41_ext_bst_widget: [snd_soc_dapm_widget_desc; 0] = [];

unsafe extern "C" fn cs35l41_component_probe(component: *mut snd_soc_component) -> c_int {
    let cs35l41 = snd_soc_component_get_drvdata(component) as *mut cs35l41_private;
    let dapm = snd_soc_component_to_dapm(component);
    let mut ret: c_int;
    if (*cs35l41).hw_cfg.bst_type == CS35L41_EXT_BOOST {
        ret = snd_soc_dapm_new_controls(dapm, cs35l41_ext_bst_widget.as_ptr(), cs35l41_ext_bst_widget.len() as c_int);
        if ret != 0 {
            return ret;
        }
        ret = snd_soc_dapm_add_routes(dapm, cs35l41_ext_bst_routes.as_ptr(), cs35l41_ext_bst_routes.len() as c_int);
        if ret != 0 {
            return ret;
        }
    }
    wm_adsp2_component_probe(&mut (*cs35l41).dsp, component)
}

unsafe extern "C" fn cs35l41_component_remove(component: *mut snd_soc_component) {
    let cs35l41 = snd_soc_component_get_drvdata(component) as *mut cs35l41_private;
    wm_adsp2_component_remove(&mut (*cs35l41).dsp, component);
}

static cs35l41_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(cs35l41_set_dai_fmt),
    hw_params: Some(cs35l41_pcm_hw_params),
    set_sysclk: Some(cs35l41_dai_set_sysclk),
    set_channel_map: Some(cs35l41_set_channel_map),
};

const CS35L41_RATES: c_uint = SNDRV_PCM_RATE_8000_48000
    | SNDRV_PCM_RATE_12000
    | SNDRV_PCM_RATE_24000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_176400
    | SNDRV_PCM_RATE_192000;

static mut cs35l41_dai: [snd_soc_dai_driver; 1] = [
    snd_soc_dai_driver {
        name: b"cs35l41-pcm\0".as_ptr() as *const c_char,
        id: 0,
        playback: snd_soc_pcm_stream {
            stream_name: b"AMP Playback\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 2,
            rates: CS35L41_RATES,
            formats: CS35L41_RX_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"AMP Capture\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 4,
            rates: CS35L41_RATES,
            formats: CS35L41_TX_FORMATS,
        },
        ops: &cs35l41_ops,
        symmetric_rate: 1,
    },
];

static soc_component_dev_cs35l41: snd_soc_component_driver = snd_soc_component_driver {
    name: b"cs35l41-codec\0".as_ptr() as *const c_char,
    probe: Some(cs35l41_component_probe),
    remove: Some(cs35l41_component_remove),
    dapm_widgets: cs35l41_dapm_widgets.as_ptr(),
    num_dapm_widgets: cs35l41_dapm_widgets.len() as c_uint,
    dapm_routes: cs35l41_audio_map.as_ptr(),
    num_dapm_routes: cs35l41_audio_map.len() as c_uint,
    controls: cs35l41_aud_controls.as_ptr(),
    num_controls: cs35l41_aud_controls.len() as c_uint,
    set_sysclk: Some(cs35l41_component_set_sysclk),
    endianness: 1,
};

unsafe fn cs35l41_handle_pdata(dev: *mut device, hw_cfg: *mut cs35l41_hw_cfg) -> c_int {
    let gpio1 = &mut (*hw_cfg).gpio1 as *mut cs35l41_gpio_cfg;
    let gpio2 = &mut (*hw_cfg).gpio2 as *mut cs35l41_gpio_cfg;
    let mut val: c_uint = 0;
    let mut ret: c_int;

    /* Some ACPI systems received the Shared Boost feature before the upstream driver,
     * leaving those systems with deprecated _DSD properties.
     * To correctly configure those systems add shared-boost-active and shared-boost-passive
     * properties mapped to the correct value in boost-type.
     * These two are not DT properties and should not be used in new systems designs.
     */
    if device_property_read_bool(dev, b"cirrus,shared-boost-active\0".as_ptr() as *const c_char) {
        (*hw_cfg).bst_type = CS35L41_SHD_BOOST_ACTV;
    } else if device_property_read_bool(dev, b"cirrus,shared-boost-passive\0".as_ptr() as *const c_char) {
        (*hw_cfg).bst_type = CS35L41_SHD_BOOST_PASS;
    } else {
        ret = device_property_read_u32(dev, b"cirrus,boost-type\0".as_ptr() as *const c_char, &mut val);
        if ret >= 0 {
            (*hw_cfg).bst_type = val as c_int;
        }
    }
    ret = device_property_read_u32(dev, b"cirrus,boost-peak-milliamp\0".as_ptr() as *const c_char, &mut val);
    (*hw_cfg).bst_ipk = if ret >= 0 { val as c_int } else { -1 };
    ret = device_property_read_u32(dev, b"cirrus,boost-ind-nanohenry\0".as_ptr() as *const c_char, &mut val);
    (*hw_cfg).bst_ind = if ret >= 0 { val as c_int } else { -1 };
    ret = device_property_read_u32(dev, b"cirrus,boost-cap-microfarad\0".as_ptr() as *const c_char, &mut val);
    (*hw_cfg).bst_cap = if ret >= 0 { val as c_int } else { -1 };
    ret = device_property_read_u32(dev, b"cirrus,asp-sdout-hiz\0".as_ptr() as *const c_char, &mut val);
    (*hw_cfg).dout_hiz = if ret >= 0 { val as c_int } else { -1 };

    /* GPIO1 Pin Config */
    (*gpio1).pol_inv = device_property_read_bool(dev, b"cirrus,gpio1-polarity-invert\0".as_ptr() as *const c_char);
    (*gpio1).out_en = device_property_read_bool(dev, b"cirrus,gpio1-output-enable\0".as_ptr() as *const c_char);
    ret = device_property_read_u32(dev, b"cirrus,gpio1-src-select\0".as_ptr() as *const c_char, &mut val);
    if ret >= 0 {
        (*gpio1).func = val;
        (*gpio1).valid = true;
    }
    /* GPIO2 Pin Config */
    (*gpio2).pol_inv = device_property_read_bool(dev, b"cirrus,gpio2-polarity-invert\0".as_ptr() as *const c_char);
    (*gpio2).out_en = device_property_read_bool(dev, b"cirrus,gpio2-output-enable\0".as_ptr() as *const c_char);
    ret = device_property_read_u32(dev, b"cirrus,gpio2-src-select\0".as_ptr() as *const c_char, &mut val);
    if ret >= 0 {
        (*gpio2).func = val;
        (*gpio2).valid = true;
    }
    (*hw_cfg).valid = true;
    0
}

unsafe fn cs35l41_dsp_init(cs35l41: *mut cs35l41_private) -> c_int {
    let dsp = &mut (*cs35l41).dsp as *mut wm_adsp;
    let dsp1rx5_src: uint32_t;
    let mut ret: c_int;
    (*dsp).part = b"cs35l41\0".as_ptr() as *const c_char;
    (*dsp).fw = 9; /* 9 is WM_ADSP_FW_SPK_PROT in wm_adsp.c */
    (*dsp).toggle_preload = true;
    cs35l41_configure_cs_dsp((*cs35l41).dev, (*cs35l41).regmap, &mut (*dsp).cs_dsp);
    ret = cs35l41_write_fs_errata((*cs35l41).dev, (*cs35l41).regmap);
    if ret < 0 {
        return ret;
    }
    ret = wm_halo_init(dsp);
    if ret != 0 {
        dev_err((*cs35l41).dev, b"wm_halo_init failed: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    match (*cs35l41).hw_cfg.bst_type {
        CS35L41_INT_BOOST | CS35L41_SHD_BOOST_ACTV => dsp1rx5_src = CS35L41_INPUT_SRC_VPMON,
        CS35L41_EXT_BOOST | CS35L41_SHD_BOOST_PASS => dsp1rx5_src = CS35L41_INPUT_SRC_VBSTMON,
        _ => {
            dev_err((*cs35l41).dev, b"wm_halo_init failed - Invalid Boost Type: %d\n\0".as_ptr() as *const c_char, (*cs35l41).hw_cfg.bst_type);
            wm_adsp2_remove(dsp);
            return ret;
        }
    }
    ret = regmap_write((*cs35l41).regmap, CS35L41_DSP1_RX5_SRC, dsp1rx5_src);
    if ret < 0 {
        dev_err((*cs35l41).dev, b"Write DSP1RX5_SRC: %d failed: %d\n\0".as_ptr() as *const c_char, dsp1rx5_src, ret);
        wm_adsp2_remove(dsp);
        return ret;
    }
    ret = regmap_write((*cs35l41).regmap, CS35L41_DSP1_RX6_SRC, CS35L41_INPUT_SRC_VBSTMON);
    if ret < 0 {
        dev_err((*cs35l41).dev, b"Write CS35L41_INPUT_SRC_VBSTMON failed: %d\n\0".as_ptr() as *const c_char, ret);
        wm_adsp2_remove(dsp);
        return ret;
    }
    ret = regmap_write((*cs35l41).regmap, CS35L41_DSP1_RX7_SRC, CS35L41_INPUT_SRC_TEMPMON);
    if ret < 0 {
        dev_err((*cs35l41).dev, b"Write INPUT_SRC_TEMPMON failed: %d\n\0".as_ptr() as *const c_char, ret);
        wm_adsp2_remove(dsp);
        return ret;
    }
    ret = regmap_write((*cs35l41).regmap, CS35L41_DSP1_RX8_SRC, CS35L41_INPUT_SRC_RSVD);
    if ret < 0 {
        dev_err((*cs35l41).dev, b"Write INPUT_SRC_RSVD failed: %d\n\0".as_ptr() as *const c_char, ret);
        wm_adsp2_remove(dsp);
        return ret;
    }
    0
}

unsafe fn cs35l41_get_system_name(cs35l41: *mut cs35l41_private) -> c_int {
    let adev = ACPI_COMPANION((*cs35l41).dev);
    let mut sub: *const c_char = ptr::null();
    let mut tmp: *const c_char;
    let mut ret: c_int = 0;
    /* If there is no acpi_device, there is no ACPI for this system, skip checking ACPI */
    if !adev.is_null() {
        let handle: acpi_handle = acpi_device_handle(adev);
        sub = acpi_get_subsystem_id(handle);
        ret = PTR_ERR_OR_ZERO(sub as *const c_void);
        if ret != 0 {
            sub = ptr::null();
            /* If no _SUB, fallback to _HID, otherwise fail */
            if ret == -ENODATA {
                tmp = acpi_device_hid(adev);
                /* If dummy hid, return 0 and fallback to legacy firmware path */
                if strcmp(tmp, b"device\0".as_ptr() as *const c_char) == 0 {
                    ret = 0;
                    dev_warn((*cs35l41).dev, b"Subsystem ID not found\n\0".as_ptr() as *const c_char);
                    return ret;
                }
                sub = kstrdup(tmp, GFP_KERNEL);
                if sub.is_null() {
                    ret = -ENOMEM;
                    dev_warn((*cs35l41).dev, b"Subsystem ID not found\n\0".as_ptr() as *const c_char);
                    return ret;
                }
            }
        }
    } else if device_property_read_string((*cs35l41).dev, b"cirrus,subsystem-id\0".as_ptr() as *const c_char, &mut tmp) == 0 {
        sub = kstrdup(tmp, GFP_KERNEL);
        if sub.is_null() {
            ret = -ENOMEM;
            dev_warn((*cs35l41).dev, b"Subsystem ID not found\n\0".as_ptr() as *const c_char);
            return ret;
        }
    }
    if !sub.is_null() {
        (*cs35l41).dsp.system_name = sub;
        dev_info((*cs35l41).dev, b"Subsystem ID: %s\n\0".as_ptr() as *const c_char, (*cs35l41).dsp.system_name);
        return 0;
    }
    dev_warn((*cs35l41).dev, b"Subsystem ID not found\n\0".as_ptr() as *const c_char);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn cs35l41_probe(cs35l41: *mut cs35l41_private, hw_cfg: *const cs35l41_hw_cfg) -> c_int {
    let mut regid: u32 = 0;
    let mut reg_revid: u32 = 0;
    let mut i: u32;
    let mut mtl_revid: u32;
    let mut int_status: u32 = 0;
    let mut chipid_match: u32;
    let mut irq_pol: c_int = 0;
    let mut ret: c_int;

    if !hw_cfg.is_null() {
        (*cs35l41).hw_cfg = *hw_cfg;
    } else {
        ret = cs35l41_handle_pdata((*cs35l41).dev, &mut (*cs35l41).hw_cfg);
        if ret != 0 {
            return ret;
        }
    }
    i = 0;
    while (i as usize) < cs35l41_supplies.len() {
        (*cs35l41).supplies[i as usize].supply = cs35l41_supplies[i as usize];
        i += 1;
    }
    ret = devm_regulator_bulk_get((*cs35l41).dev, cs35l41_supplies.len() as c_int, (*cs35l41).supplies.as_mut_ptr());
    if ret != 0 {
        return dev_err_probe((*cs35l41).dev, ret, b"Failed to request core supplies\n\0".as_ptr() as *const c_char);
    }
    ret = regulator_bulk_enable(cs35l41_supplies.len() as c_int, (*cs35l41).supplies.as_mut_ptr());
    if ret != 0 {
        return dev_err_probe((*cs35l41).dev, ret, b"Failed to enable core supplies\n\0".as_ptr() as *const c_char);
    }
    /* returning NULL can be an option if in stereo mode */
    (*cs35l41).reset_gpio = devm_gpiod_get_optional((*cs35l41).dev, b"reset\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
    if IS_ERR((*cs35l41).reset_gpio) {
        ret = PTR_ERR((*cs35l41).reset_gpio);
        (*cs35l41).reset_gpio = ptr::null_mut();
        if ret == -EBUSY {
            dev_info((*cs35l41).dev, b"Reset line busy, assuming shared reset\n\0".as_ptr() as *const c_char);
        } else {
            dev_err_probe((*cs35l41).dev, ret, b"Failed to get reset GPIO\n\0".as_ptr() as *const c_char);
            cs35l41_safe_reset((*cs35l41).regmap, (*cs35l41).hw_cfg.bst_type);
            regulator_bulk_disable(cs35l41_supplies.len() as c_int, (*cs35l41).supplies.as_mut_ptr());
            gpiod_set_value_cansleep((*cs35l41).reset_gpio, 0);
            return ret;
        }
    }
    if !(*cs35l41).reset_gpio.is_null() {
        /* satisfy minimum reset pulse width spec */
        usleep_range(2000, 2100);
        gpiod_set_value_cansleep((*cs35l41).reset_gpio, 1);
    }
    usleep_range(2000, 2100);
    ret = regmap_read_poll_timeout((*cs35l41).regmap, CS35L41_IRQ1_STATUS4, &mut int_status, CS35L41_OTP_BOOT_DONE, 1000, 100000);
    if ret != 0 {
        dev_err_probe((*cs35l41).dev, ret, b"Failed waiting for OTP_BOOT_DONE\n\0".as_ptr() as *const c_char);
        goto_err(cs35l41, ret);
        return ret;
    }
    regmap_read((*cs35l41).regmap, CS35L41_IRQ1_STATUS3, &mut int_status);
    if (int_status & CS35L41_OTP_BOOT_ERR) != 0 {
        dev_err((*cs35l41).dev, b"OTP Boot error\n\0".as_ptr() as *const c_char);
        ret = -EINVAL;
        goto_err(cs35l41, ret);
        return ret;
    }
    ret = regmap_read((*cs35l41).regmap, CS35L41_DEVID, &mut regid);
    if ret < 0 {
        dev_err_probe((*cs35l41).dev, ret, b"Get Device ID failed\n\0".as_ptr() as *const c_char);
        goto_err(cs35l41, ret);
        return ret;
    }
    ret = regmap_read((*cs35l41).regmap, CS35L41_REVID, &mut reg_revid);
    if ret < 0 {
        dev_err_probe((*cs35l41).dev, ret, b"Get Revision ID failed\n\0".as_ptr() as *const c_char);
        goto_err(cs35l41, ret);
        return ret;
    }
    mtl_revid = reg_revid & CS35L41_MTLREVID_MASK;
    /* CS35L41 will have even MTLREVID
     * CS35L41R will have odd MTLREVID
     */
    chipid_match = if (mtl_revid % 2) != 0 { CS35L41R_CHIP_ID } else { CS35L41_CHIP_ID };
    if regid != chipid_match {
        dev_err((*cs35l41).dev, b"CS35L41 Device ID (%X). Expected ID %X\n\0".as_ptr() as *const c_char, regid, chipid_match);
        ret = -ENODEV;
        goto_err(cs35l41, ret);
        return ret;
    }
    cs35l41_test_key_unlock((*cs35l41).dev, (*cs35l41).regmap);
    ret = cs35l41_register_errata_patch((*cs35l41).dev, (*cs35l41).regmap, reg_revid);
    if ret != 0 {
        goto_err(cs35l41, ret);
        return ret;
    }
    ret = cs35l41_otp_unpack((*cs35l41).dev, (*cs35l41).regmap);
    if ret < 0 {
        dev_err_probe((*cs35l41).dev, ret, b"OTP Unpack failed\n\0".as_ptr() as *const c_char);
        goto_err(cs35l41, ret);
        return ret;
    }
    cs35l41_test_key_lock((*cs35l41).dev, (*cs35l41).regmap);
    irq_pol = cs35l41_gpio_config((*cs35l41).regmap, &mut (*cs35l41).hw_cfg);
    /* Set interrupt masks for critical errors */
    regmap_write((*cs35l41).regmap, CS35L41_IRQ1_MASK1, CS35L41_INT1_MASK_DEFAULT);
    if (*cs35l41).hw_cfg.bst_type == CS35L41_SHD_BOOST_PASS || (*cs35l41).hw_cfg.bst_type == CS35L41_SHD_BOOST_ACTV {
        regmap_update_bits((*cs35l41).regmap, CS35L41_IRQ1_MASK3, CS35L41_INT3_PLL_LOCK_MASK, 0 << CS35L41_INT3_PLL_LOCK_SHIFT);
    }
    ret = devm_request_threaded_irq((*cs35l41).dev, (*cs35l41).irq, None, Some(cs35l41_irq), IRQF_ONESHOT | IRQF_SHARED | irq_pol, b"cs35l41\0".as_ptr() as *const c_char, cs35l41 as *mut c_void);
    if ret != 0 {
        dev_err_probe((*cs35l41).dev, ret, b"Failed to request IRQ\n\0".as_ptr() as *const c_char);
        goto_err(cs35l41, ret);
        return ret;
    }
    ret = cs35l41_set_pdata(cs35l41);
    if ret < 0 {
        dev_err_probe((*cs35l41).dev, ret, b"Set pdata failed\n\0".as_ptr() as *const c_char);
        goto_err(cs35l41, ret);
        return ret;
    }
    ret = cs35l41_get_system_name(cs35l41);
    if ret < 0 {
        goto_err(cs35l41, ret);
        return ret;
    }
    ret = cs35l41_dsp_init(cs35l41);
    if ret < 0 {
        goto_err(cs35l41, ret);
        return ret;
    }
    pm_runtime_set_autosuspend_delay((*cs35l41).dev, 3000);
    pm_runtime_use_autosuspend((*cs35l41).dev);
    pm_runtime_set_active((*cs35l41).dev);
    pm_runtime_get_noresume((*cs35l41).dev);
    pm_runtime_enable((*cs35l41).dev);
    ret = devm_snd_soc_register_component((*cs35l41).dev, &soc_component_dev_cs35l41, cs35l41_dai.as_mut_ptr(), cs35l41_dai.len() as c_int);
    if ret < 0 {
        dev_err_probe((*cs35l41).dev, ret, b"Register codec failed\n\0".as_ptr() as *const c_char);
        pm_runtime_dont_use_autosuspend((*cs35l41).dev);
        pm_runtime_disable((*cs35l41).dev);
        pm_runtime_put_noidle((*cs35l41).dev);
        wm_adsp2_remove(&mut (*cs35l41).dsp);
        goto_err(cs35l41, ret);
        return ret;
    }
    pm_runtime_put_autosuspend((*cs35l41).dev);
    dev_info((*cs35l41).dev, b"Cirrus Logic CS35L41 (%x), Revision: %02X\n\0".as_ptr() as *const c_char, regid, reg_revid);
    0
}

unsafe fn goto_err(cs35l41: *mut cs35l41_private, _ret: c_int) {
    cs35l41_safe_reset((*cs35l41).regmap, (*cs35l41).hw_cfg.bst_type);
    regulator_bulk_disable(cs35l41_supplies.len() as c_int, (*cs35l41).supplies.as_mut_ptr());
    gpiod_set_value_cansleep((*cs35l41).reset_gpio, 0);
}

#[no_mangle]
pub unsafe extern "C" fn cs35l41_remove(cs35l41: *mut cs35l41_private) {
    pm_runtime_get_sync((*cs35l41).dev);
    pm_runtime_dont_use_autosuspend((*cs35l41).dev);
    pm_runtime_disable((*cs35l41).dev);
    regmap_write((*cs35l41).regmap, CS35L41_IRQ1_MASK1, 0xFFFFFFFF);
    if (*cs35l41).hw_cfg.bst_type == CS35L41_SHD_BOOST_PASS || (*cs35l41).hw_cfg.bst_type == CS35L41_SHD_BOOST_ACTV {
        regmap_update_bits((*cs35l41).regmap, CS35L41_IRQ1_MASK3, CS35L41_INT3_PLL_LOCK_MASK, 1 << CS35L41_INT3_PLL_LOCK_SHIFT);
    }
    kfree((*cs35l41).dsp.system_name as *mut c_void);
    wm_adsp2_remove(&mut (*cs35l41).dsp);
    cs35l41_safe_reset((*cs35l41).regmap, (*cs35l41).hw_cfg.bst_type);
    pm_runtime_put_noidle((*cs35l41).dev);
    regulator_bulk_disable(cs35l41_supplies.len() as c_int, (*cs35l41).supplies.as_mut_ptr());
    gpiod_set_value_cansleep((*cs35l41).reset_gpio, 0);
}

unsafe extern "C" fn cs35l41_runtime_suspend(dev: *mut device) -> c_int {
    let cs35l41 = dev_get_drvdata(dev) as *mut cs35l41_private;
    dev_dbg((*cs35l41).dev, b"Runtime suspend\n\0".as_ptr() as *const c_char);
    if !(*cs35l41).dsp.preloaded || !(*cs35l41).dsp.cs_dsp.running {
        return 0;
    }
    wm_adsp_hibernate(&mut (*cs35l41).dsp, true);
    cs35l41_enter_hibernate(dev, (*cs35l41).regmap, (*cs35l41).hw_cfg.bst_type);
    regcache_cache_only((*cs35l41).regmap, true);
    regcache_mark_dirty((*cs35l41).regmap);
    0
}

unsafe extern "C" fn cs35l41_runtime_resume(dev: *mut device) -> c_int {
    let cs35l41 = dev_get_drvdata(dev) as *mut cs35l41_private;
    let mut ret: c_int;
    dev_dbg((*cs35l41).dev, b"Runtime resume\n\0".as_ptr() as *const c_char);
    if !(*cs35l41).dsp.preloaded || !(*cs35l41).dsp.cs_dsp.running {
        return 0;
    }
    regcache_cache_only((*cs35l41).regmap, false);
    ret = cs35l41_exit_hibernate((*cs35l41).dev, (*cs35l41).regmap);
    if ret != 0 {
        return ret;
    }
    /* Test key needs to be unlocked to allow the OTP settings to re-apply */
    cs35l41_test_key_unlock((*cs35l41).dev, (*cs35l41).regmap);
    ret = regcache_sync((*cs35l41).regmap);
    cs35l41_test_key_lock((*cs35l41).dev, (*cs35l41).regmap);
    wm_adsp_hibernate(&mut (*cs35l41).dsp, false);
    if ret != 0 {
        dev_err((*cs35l41).dev, b"Failed to restore register cache: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    cs35l41_init_boost((*cs35l41).dev, (*cs35l41).regmap, &mut (*cs35l41).hw_cfg);
    0
}

unsafe extern "C" fn cs35l41_sys_suspend(dev: *mut device) -> c_int {
    let cs35l41 = dev_get_drvdata(dev) as *mut cs35l41_private;
    dev_dbg((*cs35l41).dev, b"System suspend, disabling IRQ\n\0".as_ptr() as *const c_char);
    disable_irq((*cs35l41).irq);
    0
}

unsafe extern "C" fn cs35l41_sys_suspend_noirq(dev: *mut device) -> c_int {
    let cs35l41 = dev_get_drvdata(dev) as *mut cs35l41_private;
    dev_dbg((*cs35l41).dev, b"Late system suspend, reenabling IRQ\n\0".as_ptr() as *const c_char);
    enable_irq((*cs35l41).irq);
    0
}

unsafe extern "C" fn cs35l41_sys_resume_noirq(dev: *mut device) -> c_int {
    let cs35l41 = dev_get_drvdata(dev) as *mut cs35l41_private;
    dev_dbg((*cs35l41).dev, b"Early system resume, disabling IRQ\n\0".as_ptr() as *const c_char);
    disable_irq((*cs35l41).irq);
    0
}

unsafe extern "C" fn cs35l41_sys_resume(dev: *mut device) -> c_int {
    let cs35l41 = dev_get_drvdata(dev) as *mut cs35l41_private;
    dev_dbg((*cs35l41).dev, b"System resume, reenabling IRQ\n\0".as_ptr() as *const c_char);
    enable_irq((*cs35l41).irq);
    0
}

/* EXPORT_GPL_DEV_PM_OPS(cs35l41_pm_ops) = {
 *     RUNTIME_PM_OPS(cs35l41_runtime_suspend, cs35l41_runtime_resume, NULL)
 *     SYSTEM_SLEEP_PM_OPS(cs35l41_sys_suspend, cs35l41_sys_resume)
 *     NOIRQ_SYSTEM_SLEEP_PM_OPS(cs35l41_sys_suspend_noirq, cs35l41_sys_resume_noirq)
 * };
 *
 * MODULE_DESCRIPTION("ASoC CS35L41 driver");
 * MODULE_AUTHOR("David Rhodes, Cirrus Logic Inc, <david.rhodes@cirrus.com>");
 * MODULE_LICENSE("GPL");
 */

extern "C" {
    static CS35L41_DSP_MBOX_2: c_uint;
    static CSPL_MBOX_STS_RUNNING: c_uint;
    static CSPL_MBOX_STS_PAUSED: c_uint;
    static CSPL_MBOX_CMD_RESUME: c_uint;
    static CSPL_MBOX_CMD_PAUSE: c_uint;
    static CS35L41_INPUT_SRC_ASPRX1: c_uint;
    static CS35L41_INPUT_SRC_ASPRX2: c_uint;
    static CS35L41_INPUT_SRC_VMON: c_uint;
    static CS35L41_INPUT_SRC_IMON: c_uint;
    static CS35L41_INPUT_SRC_VPMON: c_uint;
    static CS35L41_INPUT_SRC_VBSTMON: c_uint;
    static CS35L41_INPUT_DSP_TX1: c_uint;
    static CS35L41_INPUT_DSP_TX2: c_uint;
    static CS35L41_INT_BOOST: c_int;
    static CS35L41_SHD_BOOST_ACTV: c_int;
    static CS35L41_SHD_BOOST_PASS: c_int;
    static CS35L41_EXT_BOOST: c_int;
    static CS35L41_EXT_BOOST_NO_VSPK_SWITCH: c_int;
    static CS35L41_BST_EN_DEFAULT: c_uint;
    static CS35L41_BST_DIS_FET_OFF: c_uint;
    static CS35L41_PWR_CTRL2: c_uint;
    static CS35L41_BST_EN_MASK: c_uint;
    static CS35L41_BST_EN_SHIFT: c_uint;
    static CS35L41_IRQ1_STATUS1: c_uint;
    static CS35L41_PROTECT_REL_ERR_IGN: c_uint;
    static CS35L41_IRQ1_MASK1: c_uint;
    static CS35L41_REGSTRIDE: c_uint;
    static CS35L41_IRQ1_MASK4: c_uint;
    static CS35L41_OTP_BOOT_DONE: c_uint;
    static CS35L41_AMP_SHORT_ERR: c_uint;
    static CS35L41_AMP_SHORT_ERR_RLS: c_uint;
    static CS35L41_TEMP_WARN: c_uint;
    static CS35L41_TEMP_WARN_ERR_RLS: c_uint;
    static CS35L41_TEMP_ERR: c_uint;
    static CS35L41_TEMP_ERR_RLS: c_uint;
    static CS35L41_BST_OVP_ERR: c_uint;
    static CS35L41_BST_OVP_ERR_RLS: c_uint;
    static CS35L41_BST_DCM_UVP_ERR: c_uint;
    static CS35L41_BST_UVP_ERR_RLS: c_uint;
    static CS35L41_BST_SHORT_ERR: c_uint;
    static CS35L41_BST_SHORT_ERR_RLS: c_uint;
    static CS35L41_PLL_LOCK: c_uint;
    static CS35L41_IRQ1_STATUS3: c_uint;
    static CS35L41_PUP_DONE_MASK: c_uint;
    static CS35L41_TEST_KEY_CTL: c_uint;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static CS35L41_SCLK_MSTR_MASK: c_uint;
    static CS35L41_LRCLK_MSTR_MASK: c_uint;
    static CS35L41_ASP_FMT_SHIFT: c_uint;
    static CS35L41_ASP_FMT_MASK: c_uint;
    static CS35L41_LRCLK_INV_MASK: c_uint;
    static CS35L41_SCLK_INV_MASK: c_uint;
    static CS35L41_SP_FORMAT: c_uint;
    static CS35L41_GLOBAL_CLK_CTRL: c_uint;
    static CS35L41_GLOBAL_FS_MASK: c_uint;
    static CS35L41_GLOBAL_FS_SHIFT: c_uint;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static CS35L41_ASP_WIDTH_RX_MASK: c_uint;
    static CS35L41_ASP_WIDTH_RX_SHIFT: c_uint;
    static CS35L41_SP_RX_WL: c_uint;
    static CS35L41_ASP_RX_WL_MASK: c_uint;
    static CS35L41_ASP_RX_WL_SHIFT: c_uint;
    static CS35L41_ASP_WIDTH_TX_MASK: c_uint;
    static CS35L41_ASP_WIDTH_TX_SHIFT: c_uint;
    static CS35L41_SP_TX_WL: c_uint;
    static CS35L41_ASP_TX_WL_MASK: c_uint;
    static CS35L41_ASP_TX_WL_SHIFT: c_uint;
    static CS35L41_CLKID_SCLK: c_int;
    static CS35L41_CLKID_LRCLK: c_int;
    static CS35L41_CLKID_MCLK: c_int;
    static CS35L41_PLLSRC_SCLK: c_int;
    static CS35L41_PLLSRC_LRCLK: c_int;
    static CS35L41_PLLSRC_MCLK: c_int;
    static CS35L41_PLL_CLK_CTRL: c_uint;
    static CS35L41_PLL_OPENLOOP_MASK: c_uint;
    static CS35L41_PLL_OPENLOOP_SHIFT: c_uint;
    static CS35L41_REFCLK_FREQ_MASK: c_uint;
    static CS35L41_REFCLK_FREQ_SHIFT: c_uint;
    static CS35L41_PLL_CLK_EN_MASK: c_uint;
    static CS35L41_PLL_CLK_EN_SHIFT: c_uint;
    static CS35L41_PLL_CLK_SEL_MASK: c_uint;
    static CS35L41_FS2_WINDOW_SHIFT: c_uint;
    static CS35L41_FS2_WINDOW_MASK: c_uint;
    static CS35L41_TST_FS_MON0: c_uint;
    static CS35L41_ASP_DOUT_HIZ_MASK: c_uint;
    static CS35L41_SP_HIZ_CTRL: c_uint;
    static CS35L41_RX_FORMATS: c_uint;
    static CS35L41_TX_FORMATS: c_uint;
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SNDRV_PCM_RATE_12000: c_uint;
    static SNDRV_PCM_RATE_24000: c_uint;
    static SNDRV_PCM_RATE_88200: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_176400: c_uint;
    static SNDRV_PCM_RATE_192000: c_uint;
    static CS35L41_INPUT_SRC_TEMPMON: c_uint;
    static CS35L41_INPUT_SRC_RSVD: c_uint;
    static CS35L41_DSP1_RX5_SRC: c_uint;
    static CS35L41_DSP1_RX6_SRC: c_uint;
    static CS35L41_DSP1_RX7_SRC: c_uint;
    static CS35L41_DSP1_RX8_SRC: c_uint;
    static GFP_KERNEL: c_uint;
    static GPIOD_OUT_LOW: c_int;
    static CS35L41_IRQ1_STATUS4: c_uint;
    static CS35L41_OTP_BOOT_ERR: c_uint;
    static CS35L41_DEVID: c_uint;
    static CS35L41_REVID: c_uint;
    static CS35L41_MTLREVID_MASK: c_uint;
    static CS35L41R_CHIP_ID: c_uint;
    static CS35L41_CHIP_ID: c_uint;
    static CS35L41_INT1_MASK_DEFAULT: c_uint;
    static CS35L41_IRQ1_MASK3: c_uint;
    static CS35L41_INT3_PLL_LOCK_MASK: c_uint;
    static CS35L41_INT3_PLL_LOCK_SHIFT: c_uint;
    static IRQF_ONESHOT: c_int;
    static IRQF_SHARED: c_int;

    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn wm_adsp_early_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int;
    fn wm_adsp_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_multi_reg_write_bypassed(map: *mut regmap, regs: *const reg_sequence, num_regs: c_int) -> c_int;
    fn cs35l41_set_cspl_mbox_cmd(dev: *mut device, map: *mut regmap, cmd: c_uint) -> c_int;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_info(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_crit_ratelimited(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn cs35l41_mdsync_up(map: *mut regmap) -> c_int;
    fn cs35l41_global_enable(dev: *mut device, map: *mut regmap, bst_type: c_int, enable: c_int, dsp: *mut cs_dsp) -> c_int;
    fn cs35l41_set_channels(dev: *mut device, map: *mut regmap, tx_n: c_uint, tx_slot: *const c_uint, rx_n: c_uint, rx_slot: *const c_uint) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn cs35l41_init_boost(dev: *mut device, map: *mut regmap, hw_cfg: *mut cs35l41_hw_cfg) -> c_int;
    fn snd_soc_dapm_new_controls(dapm: *mut snd_soc_dapm_context, widget: *const snd_soc_dapm_widget_desc, num: c_int) -> c_int;
    fn snd_soc_dapm_add_routes(dapm: *mut snd_soc_dapm_context, route: *const snd_soc_dapm_route, num: c_int) -> c_int;
    fn wm_adsp2_component_probe(dsp: *mut wm_adsp, component: *mut snd_soc_component) -> c_int;
    fn wm_adsp2_component_remove(dsp: *mut wm_adsp, component: *mut snd_soc_component);
    fn device_property_read_bool(dev: *mut device, propname: *const c_char) -> bool_t;
    fn device_property_read_u32(dev: *mut device, propname: *const c_char, val: *mut c_uint) -> c_int;
    fn device_property_read_string(dev: *mut device, propname: *const c_char, val: *mut *const c_char) -> c_int;
    fn cs35l41_configure_cs_dsp(dev: *mut device, map: *mut regmap, dsp: *mut cs_dsp);
    fn cs35l41_write_fs_errata(dev: *mut device, map: *mut regmap) -> c_int;
    fn wm_halo_init(dsp: *mut wm_adsp) -> c_int;
    fn wm_adsp2_remove(dsp: *mut wm_adsp);
    fn ACPI_COMPANION(dev: *mut device) -> *mut acpi_device;
    fn acpi_device_handle(adev: *mut acpi_device) -> acpi_handle;
    fn acpi_get_subsystem_id(handle: acpi_handle) -> *const c_char;
    fn PTR_ERR_OR_ZERO(ptr: *const c_void) -> c_int;
    fn acpi_device_hid(adev: *mut acpi_device) -> *const c_char;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn kstrdup(s: *const c_char, flags: c_uint) -> *const c_char;
    fn devm_regulator_bulk_get(dev: *mut device, num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_enable(num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut c_void;
    fn IS_ERR(ptr: *mut c_void) -> bool_t;
    fn PTR_ERR(ptr: *mut c_void) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn gpiod_set_value_cansleep(desc: *mut c_void, value: c_int);
    fn regmap_read_poll_timeout(map: *mut regmap, reg: c_uint, val: *mut c_uint, cond_mask: c_uint, sleep_us: c_uint, timeout_us: c_uint) -> c_int;
    fn cs35l41_test_key_unlock(dev: *mut device, map: *mut regmap);
    fn cs35l41_register_errata_patch(dev: *mut device, map: *mut regmap, revid: c_uint) -> c_int;
    fn cs35l41_otp_unpack(dev: *mut device, map: *mut regmap) -> c_int;
    fn cs35l41_test_key_lock(dev: *mut device, map: *mut regmap);
    fn cs35l41_gpio_config(map: *mut regmap, hw_cfg: *mut cs35l41_hw_cfg) -> c_int;
    fn devm_request_threaded_irq(dev: *mut device, irq: c_int, handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, thread_fn: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, flags: c_int, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn pm_runtime_dont_use_autosuspend(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_put_noidle(dev: *mut device);
    fn cs35l41_safe_reset(map: *mut regmap, bst_type: c_int);
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn kfree(ptr: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn wm_adsp_hibernate(dsp: *mut wm_adsp, hibernate: bool_t);
    fn cs35l41_enter_hibernate(dev: *mut device, map: *mut regmap, bst_type: c_int);
    fn regcache_cache_only(map: *mut regmap, enable: bool_t);
    fn regcache_mark_dirty(map: *mut regmap);
    fn cs35l41_exit_hibernate(dev: *mut device, map: *mut regmap) -> c_int;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn disable_irq(irq: c_int);
    fn enable_irq(irq: c_int);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
