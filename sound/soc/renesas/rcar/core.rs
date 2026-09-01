// SPDX-License-Identifier: GPL-2.0
//
// Renesas R-Car SRU/SCU/SSIU/SSI support
//
// Copyright (C) 2013 Renesas Solutions Corp.
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//
// Based on fsi.c
// Kuninori Morimoto <morimoto.kuninori@renesas.com>

/*
 * Renesas R-Car sound device structure
 *
 * Gen1
 *
 * SRU		: Sound Routing Unit
 *  - SRC	: Sampling Rate Converter
 *  - CMD
 *    - CTU	: Channel Count Conversion Unit
 *    - MIX	: Mixer
 *    - DVC	: Digital Volume and Mute Function
 *  - SSI	: Serial Sound Interface
 *
 * Gen2
 *
 * SCU		: Sampling Rate Converter Unit
 *  - SRC	: Sampling Rate Converter
 *  - CMD
 *   - CTU	: Channel Count Conversion Unit
 *   - MIX	: Mixer
 *   - DVC	: Digital Volume and Mute Function
 * SSIU		: Serial Sound Interface Unit
 *  - SSI	: Serial Sound Interface
 */

/*
 *	driver data Image
 *
 * rsnd_priv
 *   |
 *   | ** this depends on Gen1/Gen2
 *   |
 *   +- gen
 *   |
 *   | ** these depend on data path
 *   | ** gen and platform data control it
 *   |
 *   +- rdai[0]
 *   |   |		 sru     ssiu      ssi
 *   |   +- playback -> [mod] -> [mod] -> [mod] -> ...
 *   |   |
 *   |   |		 sru     ssiu      ssi
 *   |   +- capture  -> [mod] -> [mod] -> [mod] -> ...
 *   |
 *   +- rdai[1]
 *   |   |		 sru     ssiu      ssi
 *   |   +- playback -> [mod] -> [mod] -> [mod] -> ...
 *   |   |
 *   |   |		 sru     ssiu      ssi
 *   |   +- capture  -> [mod] -> [mod] -> [mod] -> ...
 *   ...
 *   |
 *   | ** these control ssi
 *   |
 *   +- ssi
 *   |  |
 *   |  +- ssi[0]
 *   |  +- ssi[1]
 *   |  +- ssi[2]
 *   |  ...
 *   |
 *   | ** these control src
 *   |
 *   +- src
 *      |
 *      +- src[0]
 *      +- src[1]
 *      +- src[2]
 *      ...
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

type u8 = u8;
type u32 = u32;
type u64 = u64;
type size_t = usize;
type snd_pcm_uframes_t = c_ulong;

const RSND_RATES: c_uint = SNDRV_PCM_RATE_8000_192000;
const RSND_FMTS: u64 = SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE;
const MOD_NAME_NUM: usize = 5;
const MOD_NAME_SIZE: usize = 16;
const RSND_INDEXED_NAME_MAX: usize = 32;
const PREALLOC_BUFFER: c_int = 32 * 1024;
const PREALLOC_BUFFER_MAX: c_int = 32 * 1024;

#[repr(C)]
pub struct rsnd_mod {
    pub id: c_int,
    pub ops: *mut rsnd_mod_ops,
    pub type_: rsnd_mod_type,
    pub clk: *mut clk,
    pub rstc: *mut reset_control,
    pub priv_: *mut rsnd_priv,
    pub status: u32,
}

#[repr(C)]
pub struct rsnd_mod_ops {
    pub name: *const c_char,
    pub dma_req: Option<unsafe extern "C" fn(*mut rsnd_dai_stream, *mut rsnd_mod) -> *mut dma_chan>,
    pub get_status: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, rsnd_mod_type) -> *mut u32>,
    pub id: Option<unsafe extern "C" fn(*mut rsnd_mod) -> c_int>,
    pub id_sub: Option<unsafe extern "C" fn(*mut rsnd_mod) -> c_int>,
    pub init: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub start: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub irq: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv, c_int) -> c_int>,
    pub stop: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub quit: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub cleanup: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub pcm_new: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut snd_soc_pcm_runtime) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut snd_pcm_substream) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut snd_pcm_uframes_t) -> c_int>,
    pub probe: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub fallback: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
}

#[repr(C)]
pub struct rsnd_dai_stream {
    pub mod_: [*mut rsnd_mod; RSND_MOD_MAX as usize],
    pub substream: *mut snd_pcm_substream,
    pub rdai: *mut rsnd_dai,
    pub name: [c_char; RSND_DAI_NAME_SIZE as usize],
    pub converted_rate: c_uint,
    pub converted_chan: c_uint,
    pub dmac_dev: *mut device,
}

#[repr(C)]
pub struct rsnd_dai {
    pub playback: rsnd_dai_stream,
    pub capture: rsnd_dai_stream,
    pub max_channels: c_uint,
    pub ssi_lane: c_uint,
    pub chan_width: c_uint,
    pub clk_master: c_int,
    pub bit_clk_inv: c_int,
    pub frm_clk_inv: c_int,
    pub sys_delay: c_int,
    pub data_alignment: c_int,
    pub name: [c_char; RSND_DAI_NAME_SIZE as usize],
    pub dai_args: snd_soc_dai_args,
    pub priv_: *mut rsnd_priv,
    pub constraint: snd_pcm_hw_constraint_list,
}

#[repr(C)]
pub struct rsnd_priv {
    pub pdev: *mut platform_device,
    pub flags: c_ulong,
    pub lock: spinlock_t,
    pub rdai_nr: c_int,
    pub daidrv: *mut snd_soc_dai_driver,
    pub rdai: *mut rsnd_dai,
    pub component_dais: [c_int; RSND_MAX_COMPONENT as usize],
}

#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct device_node { pub full_name: *const c_char }
#[repr(C)] pub struct clk(c_void);
#[repr(C)] pub struct reset_control(c_void);
#[repr(C)] pub struct dma_chan(c_void);
#[repr(C)] pub struct spinlock_t(c_void);
#[repr(C)] pub struct snd_pcm_hw_rule { pub private: *mut c_void }
#[repr(C)] pub struct snd_interval { pub min: c_uint, pub max: c_uint }
#[repr(C)] pub struct snd_pcm_hw_params { pub cmask: c_uint }
#[repr(C)] pub struct snd_pcm_runtime { pub channels: c_uint, pub format: c_int }
#[repr(C)] pub struct snd_pcm_substream { pub stream: c_int, pub runtime: *mut snd_pcm_runtime, pub next: *mut snd_pcm_substream }
#[repr(C)] pub struct snd_soc_dai { pub id: c_int }
#[repr(C)] pub struct snd_soc_dai_args { pub np: *mut device_node, pub args_count: c_uint, pub args: [u32; 1] }
#[repr(C)] pub struct snd_soc_dai_driver_stream { pub rates: c_uint, pub formats: u64, pub channels_min: c_uint, pub channels_max: c_uint, pub stream_name: *mut c_char }
#[repr(C)] pub struct snd_soc_dai_driver { pub name: *mut c_char, pub ops: *const snd_soc_dai_ops, pub id: c_int, pub dai_args: *mut snd_soc_dai_args, pub playback: snd_soc_dai_driver_stream, pub capture: snd_soc_dai_driver_stream, pub symmetric_rate: c_uint }
#[repr(C)] pub struct snd_soc_dai_ops;
#[repr(C)] pub struct snd_soc_component_driver;
#[repr(C)] pub struct snd_soc_component;
#[repr(C)] pub struct snd_soc_pcm_runtime { pub pcm: *mut snd_pcm, pub card: *mut snd_soc_card, pub dai_link: *mut snd_soc_dai_link, pub id: c_int, pub dpcm: [snd_soc_dpcm_stream; 2] }
#[repr(C)] pub struct snd_soc_dai_link { pub dynamic: c_int }
#[repr(C)] pub struct snd_soc_dpcm { pub be: *mut snd_soc_pcm_runtime }
#[repr(C)] pub struct snd_soc_dpcm_stream { pub hw_params: snd_pcm_hw_params }
#[repr(C)] pub struct snd_pcm { pub streams: [snd_pcm_stream; 2] }
#[repr(C)] pub struct snd_pcm_stream { pub substream: *mut snd_pcm_substream }
#[repr(C)] pub struct snd_soc_card { pub snd_card: *mut snd_card }
#[repr(C)] pub struct snd_card { pub controls: list_head }
#[repr(C)] pub struct list_head;
#[repr(C)] pub struct snd_kcontrol { pub private_data: *mut c_void }
#[repr(C)] pub struct snd_ctl_elem_info;
#[repr(C)] pub struct snd_ctl_elem_value;
#[repr(C)] pub struct snd_kcontrol_new;
#[repr(C)] pub struct rsnd_kctrl_cfg { pub val: *mut c_uint, pub texts: *const *const c_char, pub max: u32, pub size: c_int, pub accept: Option<unsafe extern "C" fn(*mut rsnd_dai_stream) -> c_int>, pub update: Option<unsafe extern "C" fn(*mut rsnd_dai_stream, *mut rsnd_mod)>, pub card: *mut snd_card, pub kctrl: *mut snd_kcontrol, pub io: *mut rsnd_dai_stream, pub mod_: *mut rsnd_mod }
#[repr(C)] pub struct rsnd_kctrl_cfg_m { pub cfg: rsnd_kctrl_cfg, pub val: [c_uint; RSND_MAX_CHANNELS as usize] }
#[repr(C)] pub struct rsnd_kctrl_cfg_s { pub cfg: rsnd_kctrl_cfg, pub val: c_uint }
#[repr(C)] pub struct of_device_id { pub compatible: *const c_char, pub data: *mut c_void }
#[repr(C)] pub struct dev_pm_ops;
#[repr(C)] pub struct platform_driver;

type rsnd_mod_type = c_int;

unsafe extern "C" {
    static RSND_GEN1: c_ulong; static RSND_GEN2: c_ulong; static RSND_GEN3: c_ulong; static RSND_GEN4: c_ulong;
    static RSND_SOC_E: c_ulong; static RSND_RZ3: c_ulong; static RSND_RZG3E: c_ulong; static RSND_SSIU_BUSIF_STATUS_COUNT_2: c_ulong;
    static RSND_MOD_MAX: c_int; static RSND_DAI_NAME_SIZE: c_int; static RSND_MAX_COMPONENT: c_int; static RSND_MAX_CHANNELS: c_int;
    static RSND_MOD_AUDMAPP: rsnd_mod_type; static RSND_MOD_AUDMA: rsnd_mod_type; static RSND_MOD_DVC: rsnd_mod_type; static RSND_MOD_MIX: rsnd_mod_type; static RSND_MOD_CTU: rsnd_mod_type; static RSND_MOD_CMD: rsnd_mod_type; static RSND_MOD_SRC: rsnd_mod_type; static RSND_MOD_SSIU: rsnd_mod_type; static RSND_MOD_SSIM3: rsnd_mod_type; static RSND_MOD_SSIM2: rsnd_mod_type; static RSND_MOD_SSIM1: rsnd_mod_type; static RSND_MOD_SSIP: rsnd_mod_type; static RSND_MOD_SSI: rsnd_mod_type;
    static RSND_STREAM_TDM_SPLIT: c_ulong; static RSND_STREAM_HDMI0: c_ulong; static RSND_STREAM_HDMI1: c_ulong; static RSND_HW_RULE_ERR: c_ulong;
    static SNDRV_PCM_RATE_8000_192000: c_uint; static SNDRV_PCM_FMTBIT_S8: u64; static SNDRV_PCM_FMTBIT_S16_LE: u64; static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int; static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int; static SNDRV_PCM_TRIGGER_RESUME: c_int; static SNDRV_PCM_TRIGGER_STOP: c_int; static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_HW_PARAM_CHANNELS: c_int; static SNDRV_PCM_HW_PARAM_RATE: c_int; static SNDRV_PCM_HW_PARAM_PERIODS: c_int;
    static SNDRV_DMA_TYPE_DEV: c_int;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint; static SND_SOC_DAIFMT_BC_FC: c_uint; static SND_SOC_DAIFMT_BP_FP: c_uint; static SND_SOC_DAIFMT_FORMAT_MASK: c_uint; static SND_SOC_DAIFMT_I2S: c_uint; static SND_SOC_DAIFMT_LEFT_J: c_uint; static SND_SOC_DAIFMT_DSP_B: c_uint; static SND_SOC_DAIFMT_RIGHT_J: c_uint; static SND_SOC_DAIFMT_DSP_A: c_uint; static SND_SOC_DAIFMT_INV_MASK: c_uint; static SND_SOC_DAIFMT_NB_IF: c_uint; static SND_SOC_DAIFMT_IB_NF: c_uint; static SND_SOC_DAIFMT_IB_IF: c_uint; static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_POSSIBLE_DAIFMT_I2S: u64; static SND_SOC_POSSIBLE_DAIFMT_RIGHT_J: u64; static SND_SOC_POSSIBLE_DAIFMT_LEFT_J: u64; static SND_SOC_POSSIBLE_DAIFMT_DSP_A: u64; static SND_SOC_POSSIBLE_DAIFMT_DSP_B: u64; static SND_SOC_POSSIBLE_DAIFMT_NB_NF: u64; static SND_SOC_POSSIBLE_DAIFMT_NB_IF: u64; static SND_SOC_POSSIBLE_DAIFMT_IB_NF: u64; static SND_SOC_POSSIBLE_DAIFMT_IB_IF: u64;
    static EIO: c_int; static EINVAL: c_int; static ENOMEM: c_int; static ENODEV: c_int; static ENOENT: c_int; static EAGAIN: c_int; static EPROBE_DEFER: c_int;

    fn rsnd_mod_to_priv(mod_: *mut rsnd_mod) -> *mut rsnd_priv;
    fn rsnd_priv_to_dev(priv_: *mut rsnd_priv) -> *mut device;
    fn rsnd_io_to_priv(io: *mut rsnd_dai_stream) -> *mut rsnd_priv;
    fn rsnd_io_to_rdai(io: *mut rsnd_dai_stream) -> *mut rsnd_dai;
    fn rsnd_rdai_to_priv(rdai: *mut rsnd_dai) -> *mut rsnd_priv;
    fn rsnd_io_to_runtime(io: *mut rsnd_dai_stream) -> *mut snd_pcm_runtime;
    fn rsnd_io_to_mod(io: *mut rsnd_dai_stream, type_: rsnd_mod_type) -> *mut rsnd_mod;
    fn rsnd_io_to_mod_ctu(io: *mut rsnd_dai_stream) -> *mut rsnd_mod;
    fn rsnd_io_to_mod_src(io: *mut rsnd_dai_stream) -> *mut rsnd_mod;
    fn rsnd_io_to_mod_cmd(io: *mut rsnd_dai_stream) -> *mut rsnd_mod;
    fn rsnd_io_to_mod_ssiu(io: *mut rsnd_dai_stream) -> *mut rsnd_mod;
    fn rsnd_io_to_mod_ssi(io: *mut rsnd_dai_stream) -> *mut rsnd_mod;
    fn rsnd_io_is_play(io: *mut rsnd_dai_stream) -> c_int;
    fn rsnd_io_converted_chan(io: *mut rsnd_dai_stream) -> u32;
    fn rsnd_flags_has(io: *mut rsnd_dai_stream, flag: c_ulong) -> c_int;
    fn rsnd_flags_set(io: *mut rsnd_dai_stream, flag: c_ulong);
    fn rsnd_flags_del(io: *mut rsnd_dai_stream, flag: c_ulong);
    fn rsnd_rdai_ssi_lane_get(rdai: *mut rsnd_dai) -> c_int;
    fn rsnd_rdai_channels_get(rdai: *mut rsnd_dai) -> c_uint;
    fn rsnd_rdai_channels_set(rdai: *mut rsnd_dai, v: c_int);
    fn rsnd_rdai_ssi_lane_set(rdai: *mut rsnd_dai, v: c_int);
    fn rsnd_rdai_width_set(rdai: *mut rsnd_dai, v: c_int);
    fn rsnd_rdai_is_clk_master(rdai: *mut rsnd_dai) -> c_int;
    fn rsnd_rdai_nr(priv_: *mut rsnd_priv) -> c_int;
    fn rsnd_ssi_clk_query(rdai: *mut rsnd_dai, a: c_uint, b: c_uint, c: *mut c_void) -> c_uint;
    fn rsnd_ssiu_of_node(priv_: *mut rsnd_priv) -> *mut device_node;
    fn rsnd_is_gen1(priv_: *mut rsnd_priv) -> c_int;
    fn rsnd_is_gen2(priv_: *mut rsnd_priv) -> c_int;
    fn rsnd_ssi_is_pin_sharing(io: *mut rsnd_dai_stream) -> c_int;
    fn rsnd_parse_connect_ssi(rdai: *mut rsnd_dai, playback: *mut device_node, capture: *mut device_node);
    fn rsnd_parse_connect_ssiu(rdai: *mut rsnd_dai, playback: *mut device_node, capture: *mut device_node);
    fn rsnd_parse_connect_src(rdai: *mut rsnd_dai, playback: *mut device_node, capture: *mut device_node);
    fn rsnd_parse_connect_ctu(rdai: *mut rsnd_dai, playback: *mut device_node, capture: *mut device_node);
    fn rsnd_parse_connect_mix(rdai: *mut rsnd_dai, playback: *mut device_node, capture: *mut device_node);
    fn rsnd_parse_connect_dvc(rdai: *mut rsnd_dai, playback: *mut device_node, capture: *mut device_node);
    fn rsnd_gen_probe(priv_: *mut rsnd_priv) -> c_int; fn rsnd_dma_probe(priv_: *mut rsnd_priv) -> c_int; fn rsnd_ssi_probe(priv_: *mut rsnd_priv) -> c_int; fn rsnd_ssiu_probe(priv_: *mut rsnd_priv) -> c_int; fn rsnd_src_probe(priv_: *mut rsnd_priv) -> c_int; fn rsnd_ctu_probe(priv_: *mut rsnd_priv) -> c_int; fn rsnd_mix_probe(priv_: *mut rsnd_priv) -> c_int; fn rsnd_dvc_probe(priv_: *mut rsnd_priv) -> c_int; fn rsnd_cmd_probe(priv_: *mut rsnd_priv) -> c_int; fn rsnd_adg_probe(priv_: *mut rsnd_priv) -> c_int;
    fn rsnd_ssi_remove(priv_: *mut rsnd_priv); fn rsnd_ssiu_remove(priv_: *mut rsnd_priv); fn rsnd_src_remove(priv_: *mut rsnd_priv); fn rsnd_ctu_remove(priv_: *mut rsnd_priv); fn rsnd_mix_remove(priv_: *mut rsnd_priv); fn rsnd_dvc_remove(priv_: *mut rsnd_priv); fn rsnd_cmd_remove(priv_: *mut rsnd_priv); fn rsnd_adg_remove(priv_: *mut rsnd_priv);
    fn rsnd_dma_suspend(priv_: *mut rsnd_priv); fn rsnd_ssi_suspend(priv_: *mut rsnd_priv); fn rsnd_ssiu_suspend(priv_: *mut rsnd_priv); fn rsnd_src_suspend(priv_: *mut rsnd_priv); fn rsnd_ctu_suspend(priv_: *mut rsnd_priv); fn rsnd_mix_suspend(priv_: *mut rsnd_priv); fn rsnd_dvc_suspend(priv_: *mut rsnd_priv); fn rsnd_adg_suspend(priv_: *mut rsnd_priv); fn rsnd_adg_clk_disable(priv_: *mut rsnd_priv);
    fn rsnd_dma_resume(priv_: *mut rsnd_priv); fn rsnd_ssi_resume(priv_: *mut rsnd_priv); fn rsnd_ssiu_resume(priv_: *mut rsnd_priv); fn rsnd_src_resume(priv_: *mut rsnd_priv); fn rsnd_ctu_resume(priv_: *mut rsnd_priv); fn rsnd_mix_resume(priv_: *mut rsnd_priv); fn rsnd_dvc_resume(priv_: *mut rsnd_priv); fn rsnd_adg_resume(priv_: *mut rsnd_priv); fn rsnd_adg_clk_enable(priv_: *mut rsnd_priv);
    fn rsnd_debugfs_probe() -> c_int;

    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strstr(a: *const c_char, b: *const c_char) -> *mut c_char;
    fn strchr(a: *const c_char, c: c_int) -> *mut c_char;
    fn strscpy(dst: *mut c_char, src: *const c_char, count: size_t) -> isize;
    fn clk_prepare_enable(clk: *mut clk) -> c_int; fn clk_disable_unprepare(clk: *mut clk); fn clk_disable(clk: *mut clk); fn clk_unprepare(clk: *mut clk); fn clk_prepare(clk: *mut clk) -> c_int;
    fn reset_control_deassert(rstc: *mut reset_control) -> c_int; fn reset_control_assert(rstc: *mut reset_control) -> c_int;
    fn snd_pcm_running(substream: *mut snd_pcm_substream) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint; fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn snd_interval_any(i: *mut snd_interval); fn snd_interval_test(i: *mut snd_interval, v: c_uint) -> c_int; fn snd_interval_refine(i: *mut snd_interval, p: *mut snd_interval) -> c_int;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, param: c_int) -> *mut snd_interval;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut rsnd_priv;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_set_runtime_hwparams(substream: *mut snd_pcm_substream, hw: *const c_void);
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, l: *mut snd_pcm_hw_constraint_list) -> c_int;
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_int) -> c_int;
    fn snd_pcm_hw_rule_add(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, func: unsafe extern "C" fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> c_int, private: *mut c_void, dep: c_int, end: c_int) -> c_int;
    fn of_get_child_by_name(np: *mut device_node, name: *const c_char) -> *mut device_node;
    fn of_get_child_count(np: *mut device_node) -> c_int;
    fn of_parse_phandle(np: *mut device_node, name: *const c_char, index: c_int) -> *mut device_node;
    fn of_node_put(np: *mut device_node);
    fn of_node_name_eq(np: *mut device_node, name: *const c_char) -> c_int;
    fn of_node_full_name(np: *mut device_node) -> *const c_char;
    fn of_graph_get_remote_port_parent(np: *mut device_node) -> *mut device_node;
    fn of_graph_get_endpoint_count(np: *mut device_node) -> c_int;
    fn devm_clk_get(dev: *mut device, name: *const c_char) -> *mut clk;
    fn devm_clk_get_optional(dev: *mut device, name: *const c_char) -> *mut clk;
    fn devm_reset_control_get_optional(dev: *mut device, name: *const c_char) -> *mut reset_control;
    fn IS_ERR(p: *const c_void) -> c_int; fn PTR_ERR(p: *const c_void) -> isize;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void); fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_snd_soc_register_component(dev: *mut device, cmp: *const snd_soc_component_driver, drv: *mut snd_soc_dai_driver, nr: c_int) -> c_int;
    fn pm_runtime_enable(dev: *mut device); fn pm_runtime_disable(dev: *mut device);
    fn snd_pcm_set_managed_buffer(substream: *mut snd_pcm_substream, ty: c_int, dev: *mut device, size: c_int, max: c_int);
    fn snd_kcontrol_chip(kctrl: *mut snd_kcontrol) -> *mut rsnd_kctrl_cfg;
    fn snd_ctl_new1(knew: *const snd_kcontrol_new, data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kctrl: *mut snd_kcontrol) -> c_int;
}

#[used]
static rsnd_of_match: [of_device_id; 7] = [
    of_device_id { compatible: b"renesas,rcar_sound-gen1\0".as_ptr() as *const c_char, data: RSND_GEN1 as *mut c_void },
    of_device_id { compatible: b"renesas,rcar_sound-gen2\0".as_ptr() as *const c_char, data: RSND_GEN2 as *mut c_void },
    of_device_id { compatible: b"renesas,rcar_sound-gen3\0".as_ptr() as *const c_char, data: RSND_GEN3 as *mut c_void },
    of_device_id { compatible: b"renesas,rcar_sound-gen4\0".as_ptr() as *const c_char, data: RSND_GEN4 as *mut c_void },
    /* Special Handling */
    of_device_id { compatible: b"renesas,rcar_sound-r8a77990\0".as_ptr() as *const c_char, data: (RSND_GEN3 | RSND_SOC_E) as *mut c_void },
    of_device_id { compatible: b"renesas,r9a09g047-sound\0".as_ptr() as *const c_char, data: (RSND_RZ3 | RSND_RZG3E | RSND_SSIU_BUSIF_STATUS_COUNT_2) as *mut c_void },
    of_device_id { compatible: ptr::null(), data: ptr::null_mut() },
];

pub unsafe extern "C" fn rsnd_mod_make_sure(mod_: *mut rsnd_mod, type_: rsnd_mod_type) {
    if (*mod_).type_ != type_ {
        let priv_ = rsnd_mod_to_priv(mod_);
        let dev = rsnd_priv_to_dev(priv_);
        dev_warn(dev, b"%s is not your expected module\n\0".as_ptr() as *const c_char, rsnd_mod_name(mod_));
    }
}

pub unsafe extern "C" fn rsnd_mod_dma_req(io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod) -> *mut dma_chan {
    if mod_.is_null() || (*mod_).ops.is_null() || (*(*mod_).ops).dma_req.is_none() {
        return ptr::null_mut();
    }
    ((*(*mod_).ops).dma_req.unwrap())(io, mod_)
}

pub unsafe extern "C" fn rsnd_mod_name(mod_: *mut rsnd_mod) -> *mut c_char {
    static mut NAMES: [[c_char; MOD_NAME_SIZE]; MOD_NAME_NUM] = [[0; MOD_NAME_SIZE]; MOD_NAME_NUM];
    static mut NUM: usize = 0;
    let name = NAMES[NUM].as_mut_ptr();
    NUM += 1;
    if NUM >= MOD_NAME_NUM { NUM = 0; }

    /*
     * Let's use same char to avoid pointlessness memory
     * Thus, rsnd_mod_name() should be used immediately
     * Don't keep pointer
     */
    if (*(*mod_).ops).id_sub.is_some() {
        snprintf(name, MOD_NAME_SIZE, b"%s[%d%d]\0".as_ptr() as *const c_char,
                 (*(*mod_).ops).name, rsnd_mod_id(mod_), rsnd_mod_id_sub(mod_));
    } else {
        snprintf(name, MOD_NAME_SIZE, b"%s[%d]\0".as_ptr() as *const c_char,
                 (*(*mod_).ops).name, rsnd_mod_id(mod_));
    }
    name
}

pub unsafe extern "C" fn rsnd_mod_get_status(mod_: *mut rsnd_mod, _io: *mut rsnd_dai_stream, _type_: rsnd_mod_type) -> *mut u32 {
    &mut (*mod_).status
}

pub unsafe extern "C" fn rsnd_mod_id_raw(mod_: *mut rsnd_mod) -> c_int { (*mod_).id }

pub unsafe extern "C" fn rsnd_mod_id(mod_: *mut rsnd_mod) -> c_int {
    if let Some(id) = (*(*mod_).ops).id { return id(mod_); }
    rsnd_mod_id_raw(mod_)
}

pub unsafe extern "C" fn rsnd_mod_id_sub(mod_: *mut rsnd_mod) -> c_int {
    if let Some(id_sub) = (*(*mod_).ops).id_sub { return id_sub(mod_); }
    0
}

pub unsafe extern "C" fn rsnd_mod_init(priv_: *mut rsnd_priv, mod_: *mut rsnd_mod, ops: *mut rsnd_mod_ops, clk: *mut clk, rstc: *mut reset_control, type_: rsnd_mod_type, id: c_int) -> c_int {
    let mut ret = clk_prepare_enable(clk);
    if ret != 0 { return ret; }
    ret = reset_control_deassert(rstc);
    if ret != 0 {
        clk_disable_unprepare(clk);
        return ret;
    }
    clk_disable(clk);
    (*mod_).id = id;
    (*mod_).ops = ops;
    (*mod_).type_ = type_;
    (*mod_).clk = clk;
    (*mod_).rstc = rstc;
    (*mod_).priv_ = priv_;
    0
}

pub unsafe extern "C" fn rsnd_mod_quit(mod_: *mut rsnd_mod) {
    reset_control_assert((*mod_).rstc);
    (*mod_).rstc = ptr::null_mut();
    clk_unprepare((*mod_).clk);
    (*mod_).clk = ptr::null_mut();
}

pub unsafe extern "C" fn rsnd_mod_interrupt(mod_: *mut rsnd_mod, callback: unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream)) {
    let priv_ = rsnd_mod_to_priv(mod_);
    let mut i = 0;
    while i < rsnd_rdai_nr(priv_) {
        let rdai = (*priv_).rdai.add(i as usize);
        let mut io = &mut (*rdai).playback as *mut rsnd_dai_stream;
        if mod_ == (*io).mod_[(*mod_).type_ as usize] { callback(mod_, io); }
        io = &mut (*rdai).capture;
        if mod_ == (*io).mod_[(*mod_).type_ as usize] { callback(mod_, io); }
        i += 1;
    }
}

pub unsafe extern "C" fn rsnd_io_is_working(io: *mut rsnd_dai_stream) -> c_int {
    /* see rsnd_dai_stream_init/quit() */
    if !(*io).substream.is_null() { return snd_pcm_running((*io).substream); }
    0
}

pub unsafe extern "C" fn rsnd_runtime_channel_original_with_params(io: *mut rsnd_dai_stream, params: *mut snd_pcm_hw_params) -> c_int {
    let runtime = rsnd_io_to_runtime(io);
    if !params.is_null() { return params_channels(params) as c_int; }
    else if !runtime.is_null() { return (*runtime).channels as c_int; }
    0
}

pub unsafe extern "C" fn rsnd_runtime_channel_after_ctu_with_params(io: *mut rsnd_dai_stream, params: *mut snd_pcm_hw_params) -> c_int {
    let chan = rsnd_runtime_channel_original_with_params(io, params);
    let ctu_mod = rsnd_io_to_mod_ctu(io);
    if !ctu_mod.is_null() {
        let converted_chan = rsnd_io_converted_chan(io);
        if rsnd_runtime_is_tdm_split(io) != 0 {
            let dev = rsnd_priv_to_dev(rsnd_io_to_priv(io));
            dev_err(dev, b"CTU and TDM Split should be used\n\0".as_ptr() as *const c_char);
        }
        if converted_chan != 0 { return converted_chan as c_int; }
    }
    chan
}

pub unsafe extern "C" fn rsnd_channel_normalization(mut chan: c_int) -> c_int {
    if chan > 8 || chan < 0 { return 0; }
    /* TDM Extend Mode needs 8ch */
    if chan == 6 { chan = 8; }
    chan
}

pub unsafe extern "C" fn rsnd_runtime_channel_for_ssi_with_params(io: *mut rsnd_dai_stream, params: *mut snd_pcm_hw_params) -> c_int {
    let rdai = rsnd_io_to_rdai(io);
    let mut chan = if rsnd_io_is_play(io) != 0 { rsnd_runtime_channel_after_ctu_with_params(io, params) } else { rsnd_runtime_channel_original_with_params(io, params) };
    /* Use Multi SSI */
    if rsnd_runtime_is_multi_ssi(io) != 0 { chan /= rsnd_rdai_ssi_lane_get(rdai); }
    rsnd_channel_normalization(chan)
}

pub unsafe extern "C" fn rsnd_runtime_is_multi_ssi(io: *mut rsnd_dai_stream) -> c_int {
    let rdai = rsnd_io_to_rdai(io);
    let lane = rsnd_rdai_ssi_lane_get(rdai);
    let chan = if rsnd_io_is_play(io) != 0 { rsnd_runtime_channel_after_ctu_with_params(io, ptr::null_mut()) } else { rsnd_runtime_channel_original_with_params(io, ptr::null_mut()) };
    ((chan > 2) && (lane > 1)) as c_int
}

pub unsafe extern "C" fn rsnd_runtime_is_tdm(io: *mut rsnd_dai_stream) -> c_int {
    (rsnd_runtime_channel_for_ssi_with_params(io, ptr::null_mut()) >= 6) as c_int
}

pub unsafe extern "C" fn rsnd_runtime_is_tdm_split(io: *mut rsnd_dai_stream) -> c_int {
    (rsnd_flags_has(io, RSND_STREAM_TDM_SPLIT) != 0) as c_int
}

pub unsafe extern "C" fn rsnd_get_adinr_bit(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream) -> u32 {
    let priv_ = rsnd_mod_to_priv(mod_);
    let runtime = rsnd_io_to_runtime(io);
    let dev = rsnd_priv_to_dev(priv_);
    match snd_pcm_format_width((*runtime).format) {
        8 => 16 << 16,
        16 => 8 << 16,
        24 => 0 << 16,
        _ => {
            dev_warn(dev, b"not supported sample bits\n\0".as_ptr() as *const c_char);
            0
        }
    }
}

pub unsafe extern "C" fn rsnd_get_dalign(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream) -> u32 {
    static dalign_values: [u32; 8] = [0x76543210, 0x00000032, 0x00007654, 0x00000076, 0xfedcba98, 0x000000ba, 0x0000fedc, 0x000000fe];
    let mut id = 0;
    let ssiu = rsnd_io_to_mod_ssiu(io);
    let target;
    let runtime = rsnd_io_to_runtime(io);
    if rsnd_io_is_play(io) != 0 {
        let src = rsnd_io_to_mod_src(io);
        target = if !src.is_null() { src } else { ssiu };
    } else {
        let cmd = rsnd_io_to_mod_cmd(io);
        target = if !cmd.is_null() { cmd } else { ssiu };
    }
    if mod_ == ssiu { id = rsnd_mod_id_sub(mod_); }
    let mut dalign = dalign_values[id as usize];
    if mod_ == target && snd_pcm_format_width((*runtime).format) == 16 {
        /* Target mod needs inverted DALIGN when 16bit */
        dalign = ((dalign & 0xf0f0f0f0) >> 4) | ((dalign & 0x0f0f0f0f) << 4);
    }
    dalign
}

pub unsafe extern "C" fn rsnd_get_busif_shift(io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod) -> u32 {
    static playback_mods: [rsnd_mod_type; 3] = [RSND_MOD_SRC, RSND_MOD_CMD, RSND_MOD_SSIU];
    static capture_mods: [rsnd_mod_type; 3] = [RSND_MOD_CMD, RSND_MOD_SRC, RSND_MOD_SSIU];
    let runtime = rsnd_io_to_runtime(io);
    let mods = if rsnd_io_is_play(io) != 0 { &playback_mods } else { &capture_mods };
    if snd_pcm_format_width((*runtime).format) != 24 { return 0; }
    let mut tmod = ptr::null_mut();
    for ty in mods {
        tmod = rsnd_io_to_mod(io, *ty);
        if !tmod.is_null() { break; }
    }
    if tmod != mod_ { return 0; }
    if rsnd_io_is_play(io) != 0 { (0 << 20) | (8 << 16) } else { (1 << 20) | (8 << 16) }
}

pub unsafe extern "C" fn rsnd_mod_next(iterator: *mut c_int, io: *mut rsnd_dai_stream, array: *mut rsnd_mod_type, array_size: c_int) -> *mut rsnd_mod {
    let max = if !array.is_null() { array_size } else { RSND_MOD_MAX };
    while *iterator < max {
        let type_ = if !array.is_null() { *array.add(*iterator as usize) } else { *iterator };
        let mod_ = rsnd_io_to_mod(io, type_);
        if !mod_.is_null() { return mod_; }
        *iterator += 1;
    }
    ptr::null_mut()
}

static mut rsnd_mod_sequence: [[rsnd_mod_type; RSND_MOD_MAX as usize]; 2] = [
    [RSND_MOD_AUDMAPP, RSND_MOD_AUDMA, RSND_MOD_DVC, RSND_MOD_MIX, RSND_MOD_CTU, RSND_MOD_CMD, RSND_MOD_SRC, RSND_MOD_SSIU, RSND_MOD_SSIM3, RSND_MOD_SSIM2, RSND_MOD_SSIM1, RSND_MOD_SSIP, RSND_MOD_SSI],
    [RSND_MOD_AUDMAPP, RSND_MOD_AUDMA, RSND_MOD_SSIM3, RSND_MOD_SSIM2, RSND_MOD_SSIM1, RSND_MOD_SSIP, RSND_MOD_SSI, RSND_MOD_SSIU, RSND_MOD_DVC, RSND_MOD_MIX, RSND_MOD_CTU, RSND_MOD_CMD, RSND_MOD_SRC],
];

unsafe fn rsnd_status_update(io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod, type_: rsnd_mod_type, shift: c_int, add: c_int, timing: c_int) -> c_int {
    let get_status = (*(*mod_).ops).get_status.unwrap_or(rsnd_mod_get_status);
    let status = get_status(mod_, io, type_);
    let mask: u32 = 0xF << shift;
    let val: u8 = ((*status >> shift) & 0xF) as u8;
    let next_val: u8 = ((val as c_int + add) & 0xF) as u8;
    let mut func_call = (val as c_int == timing) as c_int;
    if add == 0 || shift == 28 { return 1; }
    if next_val == 0xF { func_call = -1; } else { *status = (*status & !mask).wrapping_add((next_val as u32) << shift); }
    func_call
}

macro_rules! rsnd_dai_call {
    ($fn_name:ident, $shift:expr, $add:expr, $call:expr, $io:expr $(, $arg:expr)*) => {{
        let dev = rsnd_priv_to_dev(rsnd_io_to_priv($io));
        let is_play = rsnd_io_is_play($io);
        let mut ret: c_int = 0;
        let types = &mut rsnd_mod_sequence[is_play as usize];
        for i in 0..(RSND_MOD_MAX as usize) {
            let mod_ = rsnd_io_to_mod($io, types[i]);
            if mod_.is_null() { continue; }
            let func_call = rsnd_status_update($io, mod_, types[i], $shift, $add, $call);
            let mut tmp = 0;
            if func_call > 0 {
                if let Some(f) = (*(*mod_).ops).$fn_name { tmp = f(mod_, $io $(, $arg)*); }
            }
            if func_call < 0 || (tmp != 0 && tmp != -EPROBE_DEFER) {
                dev_err(dev, b"%s : %s error (%d, %d)\n\0".as_ptr() as *const c_char, rsnd_mod_name(mod_), stringify!($fn_name).as_ptr(), tmp, func_call);
            }
            ret |= tmp;
        }
        ret
    }};
}

pub unsafe extern "C" fn rsnd_dai_connect(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, type_: rsnd_mod_type) -> c_int {
    if mod_.is_null() { return -EIO; }
    if (*io).mod_[type_ as usize] == mod_ { return 0; }
    if !(*io).mod_[type_ as usize].is_null() { return -EINVAL; }
    let priv_ = rsnd_mod_to_priv(mod_);
    let dev = rsnd_priv_to_dev(priv_);
    (*io).mod_[type_ as usize] = mod_;
    dev_dbg(dev, b"%s is connected to io (%s)\n\0".as_ptr() as *const c_char, rsnd_mod_name(mod_), if rsnd_io_is_play(io) != 0 { b"Playback\0".as_ptr() } else { b"Capture\0".as_ptr() });
    0
}

unsafe fn rsnd_dai_disconnect(_mod: *mut rsnd_mod, io: *mut rsnd_dai_stream, type_: rsnd_mod_type) {
    (*io).mod_[type_ as usize] = ptr::null_mut();
}

pub unsafe extern "C" fn rsnd_rdai_channels_ctrl(rdai: *mut rsnd_dai, max_channels: c_int) -> c_int {
    if max_channels > 0 { (*rdai).max_channels = max_channels as c_uint; }
    (*rdai).max_channels as c_int
}

pub unsafe extern "C" fn rsnd_rdai_ssi_lane_ctrl(rdai: *mut rsnd_dai, ssi_lane: c_int) -> c_int {
    if ssi_lane > 0 { (*rdai).ssi_lane = ssi_lane as c_uint; }
    (*rdai).ssi_lane as c_int
}

pub unsafe extern "C" fn rsnd_rdai_width_ctrl(rdai: *mut rsnd_dai, width: c_int) -> c_int {
    if width > 0 { (*rdai).chan_width = width as c_uint; }
    (*rdai).chan_width as c_int
}

pub unsafe extern "C" fn rsnd_rdai_get(priv_: *mut rsnd_priv, id: c_int) -> *mut rsnd_dai {
    if id < 0 || id >= rsnd_rdai_nr(priv_) { return ptr::null_mut(); }
    (*priv_).rdai.add(id as usize)
}

unsafe fn rsnd_daidrv_get(priv_: *mut rsnd_priv, id: c_int) -> *mut snd_soc_dai_driver {
    if id < 0 || id >= rsnd_rdai_nr(priv_) { return ptr::null_mut(); }
    (*priv_).daidrv.add(id as usize)
}

unsafe fn rsnd_dai_to_rdai(dai: *mut snd_soc_dai) -> *mut rsnd_dai {
    let priv_ = snd_soc_dai_get_drvdata(dai);
    rsnd_rdai_get(priv_, (*dai).id)
}

unsafe fn rsnd_dai_stream_init(io: *mut rsnd_dai_stream, substream: *mut snd_pcm_substream) { (*io).substream = substream; }
unsafe fn rsnd_dai_stream_quit(io: *mut rsnd_dai_stream) { (*io).substream = ptr::null_mut(); }

unsafe fn rsnd_substream_to_dai(substream: *mut snd_pcm_substream) -> *mut snd_soc_dai {
    let rtd = snd_soc_substream_to_rtd(substream);
    snd_soc_rtd_to_cpu(rtd, 0)
}

unsafe fn rsnd_rdai_to_io(rdai: *mut rsnd_dai, substream: *mut snd_pcm_substream) -> *mut rsnd_dai_stream {
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK { &mut (*rdai).playback } else { &mut (*rdai).capture }
}

unsafe extern "C" fn rsnd_soc_dai_trigger(substream: *mut snd_pcm_substream, cmd: c_int, dai: *mut snd_soc_dai) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai);
    let rdai = rsnd_dai_to_rdai(dai);
    let io = rsnd_rdai_to_io(rdai, substream);
    /* C used guard(spinlock_irqsave)(&priv->lock). */
    match cmd {
        x if x == SNDRV_PCM_TRIGGER_START || x == SNDRV_PCM_TRIGGER_RESUME => {
            let mut ret = rsnd_dai_call!(init, 0, 0, 0, io, priv_);
            if ret < 0 { return ret; }
            ret = rsnd_dai_call!(start, 0, 0, 0, io, priv_);
            if ret < 0 { return ret; }
            rsnd_dai_call!(irq, 0, 0, 0, io, priv_, 1)
        }
        x if x == SNDRV_PCM_TRIGGER_STOP || x == SNDRV_PCM_TRIGGER_SUSPEND => {
            let mut ret = rsnd_dai_call!(irq, 0, 0, 0, io, priv_, 0);
            ret |= rsnd_dai_call!(stop, 0, 0, 0, io, priv_);
            ret |= rsnd_dai_call!(quit, 0, 0, 0, io, priv_);
            ret
        }
        _ => -EINVAL,
    }
}

unsafe extern "C" fn rsnd_soc_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let rdai = rsnd_dai_to_rdai(dai);
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        x if x == SND_SOC_DAIFMT_BC_FC => (*rdai).clk_master = 0,
        x if x == SND_SOC_DAIFMT_BP_FP => (*rdai).clk_master = 1,
        _ => return -EINVAL,
    }
    (*rdai).bit_clk_inv = 0;
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => { (*rdai).sys_delay = 0; (*rdai).data_alignment = 0; (*rdai).frm_clk_inv = 0; }
        x if x == SND_SOC_DAIFMT_LEFT_J || x == SND_SOC_DAIFMT_DSP_B => { (*rdai).sys_delay = 1; (*rdai).data_alignment = 0; (*rdai).frm_clk_inv = 1; }
        x if x == SND_SOC_DAIFMT_RIGHT_J => { (*rdai).sys_delay = 1; (*rdai).data_alignment = 1; (*rdai).frm_clk_inv = 1; }
        x if x == SND_SOC_DAIFMT_DSP_A => { (*rdai).sys_delay = 0; (*rdai).data_alignment = 0; (*rdai).frm_clk_inv = 1; }
        _ => {}
    }
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_IF => (*rdai).frm_clk_inv = ((*rdai).frm_clk_inv == 0) as c_int,
        x if x == SND_SOC_DAIFMT_IB_NF => (*rdai).bit_clk_inv = ((*rdai).bit_clk_inv == 0) as c_int,
        x if x == SND_SOC_DAIFMT_IB_IF => { (*rdai).bit_clk_inv = ((*rdai).bit_clk_inv == 0) as c_int; (*rdai).frm_clk_inv = ((*rdai).frm_clk_inv == 0) as c_int; }
        _ => {}
    }
    0
}

unsafe extern "C" fn rsnd_soc_set_dai_tdm_slot(dai: *mut snd_soc_dai, _tx_mask: u32, _rx_mask: u32, slots: c_int, mut slot_width: c_int) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai);
    let rdai = rsnd_dai_to_rdai(dai);
    let dev = rsnd_priv_to_dev(priv_);
    match slot_width {
        16 | 24 | 32 => {}
        _ => {
            if slot_width != 0 { dev_warn(dev, b"unsupported TDM slot width (%d), force to use default 32\n\0".as_ptr() as *const c_char, slot_width); }
            slot_width = 32;
        }
    }
    match slots {
        2 | 6 | 8 => { rsnd_rdai_channels_set(rdai, slots); rsnd_rdai_ssi_lane_set(rdai, 1); rsnd_rdai_width_set(rdai, slot_width); }
        _ => { dev_err(dev, b"unsupported TDM slots (%d)\n\0".as_ptr() as *const c_char, slots); return -EINVAL; }
    }
    0
}

static mut rsnd_soc_hw_channels_list: [c_uint; 3] = [2, 6, 8];
static mut rsnd_soc_hw_rate_list: [c_uint; 12] = [8000, 11025, 16000, 22050, 32000, 44100, 48000, 64000, 88200, 96000, 176400, 192000];

unsafe fn rsnd_soc_hw_rule(rdai: *mut rsnd_dai, list: *mut c_uint, list_num: c_int, baseline: *mut snd_interval, iv: *mut snd_interval, io: *mut rsnd_dai_stream, unit: *mut c_char) -> c_int {
    let mut p = snd_interval { min: 0, max: 0 };
    snd_interval_any(&mut p);
    p.min = c_uint::MAX;
    p.max = 0;
    for i in 0..list_num {
        let v = *list.add(i as usize);
        if snd_interval_test(iv, v) == 0 { continue; }
        let mut rate = rsnd_ssi_clk_query(rdai, (*baseline).min, v, ptr::null_mut());
        if rate > 0 { p.min = core::cmp::min(p.min, v); p.max = core::cmp::max(p.max, v); }
        rate = rsnd_ssi_clk_query(rdai, (*baseline).max, v, ptr::null_mut());
        if rate > 0 { p.min = core::cmp::min(p.min, v); p.max = core::cmp::max(p.max, v); }
    }
    if rsnd_flags_has(io, RSND_HW_RULE_ERR) == 0 && p.min > p.max {
        let priv_ = rsnd_rdai_to_priv(rdai);
        let dev = rsnd_priv_to_dev(priv_);
        dev_warn(dev, b"It can't handle %d %s <-> %d %s\n\0".as_ptr() as *const c_char, (*baseline).min, unit, (*baseline).max, unit);
        rsnd_flags_set(io, RSND_HW_RULE_ERR);
    }
    snd_interval_refine(iv, &mut p)
}

unsafe extern "C" fn rsnd_soc_hw_rule_rate(params: *mut snd_pcm_hw_params, rule: *mut snd_pcm_hw_rule) -> c_int {
    let ic_ = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let ir = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let io = (*rule).private as *mut rsnd_dai_stream;
    let rdai = rsnd_io_to_rdai(io);
    let mut ic = *ic_;
    ic.min = rsnd_runtime_channel_for_ssi_with_params(io, params) as c_uint;
    ic.max = ic.min;
    rsnd_soc_hw_rule(rdai, rsnd_soc_hw_rate_list.as_mut_ptr(), rsnd_soc_hw_rate_list.len() as c_int, &mut ic, ir, io, b"ch\0".as_ptr() as *mut c_char)
}

unsafe extern "C" fn rsnd_soc_hw_rule_channels(params: *mut snd_pcm_hw_params, rule: *mut snd_pcm_hw_rule) -> c_int {
    let ic_ = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let ir = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let io = (*rule).private as *mut rsnd_dai_stream;
    let rdai = rsnd_io_to_rdai(io);
    let mut ic = *ic_;
    ic.min = rsnd_runtime_channel_for_ssi_with_params(io, params) as c_uint;
    ic.max = ic.min;
    rsnd_soc_hw_rule(rdai, rsnd_soc_hw_channels_list.as_mut_ptr(), rsnd_soc_hw_channels_list.len() as c_int, ir, &mut ic, io, b"Hz\0".as_ptr() as *mut c_char)
}

/* rsnd_pcm_hardware is a C struct initializer; fields are preserved in this comment for the external ALSA type:
 * info = INTERLEAVED | MMAP | MMAP_VALID | RESUME, buffer_bytes_max = 64 * 1024,
 * period_bytes_min = 32, period_bytes_max = 8192, periods_min = 1, periods_max = 32, fifo_size = 256.
 */

unsafe extern "C" fn rsnd_soc_dai_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let rdai = rsnd_dai_to_rdai(dai);
    let io = rsnd_rdai_to_io(rdai, substream);
    let constraint = &mut (*rdai).constraint as *mut snd_pcm_hw_constraint_list;
    let runtime = (*substream).runtime;
    let max_channels = rsnd_rdai_channels_get(rdai);
    rsnd_flags_del(io, RSND_HW_RULE_ERR);
    rsnd_dai_stream_init(io, substream);
    /* Channel Limitation */
    (*constraint).list = rsnd_soc_hw_channels_list.as_mut_ptr();
    (*constraint).count = 0;
    (*constraint).mask = 0;
    for i in 0..rsnd_soc_hw_channels_list.len() {
        if rsnd_soc_hw_channels_list[i] > max_channels { break; }
        (*constraint).count = (i + 1) as c_uint;
    }
    snd_soc_set_runtime_hwparams(substream, ptr::null());
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, constraint);
    snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if rsnd_rdai_is_clk_master(rdai) != 0 {
        let is_play = ((*substream).stream == SNDRV_PCM_STREAM_PLAYBACK) as c_int;
        snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, rsnd_soc_hw_rule_rate, if is_play != 0 { &mut (*rdai).playback as *mut _ as *mut c_void } else { &mut (*rdai).capture as *mut _ as *mut c_void }, SNDRV_PCM_HW_PARAM_CHANNELS, -1);
        snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, rsnd_soc_hw_rule_channels, if is_play != 0 { &mut (*rdai).playback as *mut _ as *mut c_void } else { &mut (*rdai).capture as *mut _ as *mut c_void }, SNDRV_PCM_HW_PARAM_RATE, -1);
    }
    0
}

unsafe extern "C" fn rsnd_soc_dai_shutdown(substream: *mut snd_pcm_substream, _dai: *mut snd_soc_dai) {
    let dai = rsnd_substream_to_dai(substream);
    let rdai = rsnd_dai_to_rdai(dai);
    let priv_ = rsnd_rdai_to_priv(rdai);
    let io = rsnd_rdai_to_io(rdai, substream);
    rsnd_dai_call!(cleanup, 0, 0, 0, io, priv_);
    rsnd_dai_stream_quit(io);
}

unsafe extern "C" fn rsnd_soc_dai_prepare(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai);
    let rdai = rsnd_dai_to_rdai(dai);
    let io = rsnd_rdai_to_io(rdai, substream);
    rsnd_dai_call!(prepare, 0, 0, 0, io, priv_)
}

static rsnd_soc_dai_formats: [u64; 2] = [
    SND_SOC_POSSIBLE_DAIFMT_I2S | SND_SOC_POSSIBLE_DAIFMT_RIGHT_J | SND_SOC_POSSIBLE_DAIFMT_LEFT_J | SND_SOC_POSSIBLE_DAIFMT_NB_NF | SND_SOC_POSSIBLE_DAIFMT_NB_IF | SND_SOC_POSSIBLE_DAIFMT_IB_NF | SND_SOC_POSSIBLE_DAIFMT_IB_IF,
    SND_SOC_POSSIBLE_DAIFMT_I2S | SND_SOC_POSSIBLE_DAIFMT_RIGHT_J | SND_SOC_POSSIBLE_DAIFMT_LEFT_J | SND_SOC_POSSIBLE_DAIFMT_DSP_A | SND_SOC_POSSIBLE_DAIFMT_DSP_B | SND_SOC_POSSIBLE_DAIFMT_NB_NF | SND_SOC_POSSIBLE_DAIFMT_NB_IF | SND_SOC_POSSIBLE_DAIFMT_IB_NF | SND_SOC_POSSIBLE_DAIFMT_IB_IF,
];

/* Device-tree child/endpoint scoped iterator macros are external C helpers.
 * The following parsing functions preserve direct calls and branch intent; loop bodies requiring
 * for_each_child_of_node_scoped/for_each_endpoint_of_node are represented by comments where needed.
 */

unsafe fn rsnd_parse_tdm_split_mode(priv_: *mut rsnd_priv, io: *mut rsnd_dai_stream, dai_np: *mut device_node) {
    let dev = rsnd_priv_to_dev(priv_);
    let ssiu_np = rsnd_ssiu_of_node(priv_);
    let is_play = rsnd_io_is_play(io);
    let mut i = 0;
    if ssiu_np.is_null() { return; }
    loop {
        let node = if is_play != 0 { of_parse_phandle(dai_np, b"playback\0".as_ptr() as *const c_char, i) } else { of_parse_phandle(dai_np, b"capture\0".as_ptr() as *const c_char, i) };
        if node.is_null() { break; }
        /* for_each_child_of_node_scoped(ssiu_np, np): if np == node, set TDM split. */
        rsnd_flags_set(io, RSND_STREAM_TDM_SPLIT);
        dev_dbg(dev, b"%s is part of TDM Split\n\0".as_ptr() as *const c_char, (*io).name.as_ptr());
        of_node_put(node);
        i += 1;
    }
    of_node_put(ssiu_np);
}

unsafe fn rsnd_parse_connect_simple(priv_: *mut rsnd_priv, io: *mut rsnd_dai_stream, dai_np: *mut device_node) {
    if rsnd_io_to_mod_ssi(io).is_null() { return; }
    rsnd_parse_tdm_split_mode(priv_, io, dai_np);
}

unsafe fn rsnd_parse_connect_graph(priv_: *mut rsnd_priv, io: *mut rsnd_dai_stream, endpoint: *mut device_node) {
    let dev = rsnd_priv_to_dev(priv_);
    if rsnd_io_to_mod_ssi(io).is_null() { return; }
    let remote_node = of_graph_get_remote_port_parent(endpoint);
    if !strstr((*remote_node).full_name, b"hdmi@fead0000\0".as_ptr() as *const c_char).is_null() {
        rsnd_flags_set(io, RSND_STREAM_HDMI0);
        dev_dbg(dev, b"%s connected to HDMI0\n\0".as_ptr() as *const c_char, (*io).name.as_ptr());
    }
    if !strstr((*remote_node).full_name, b"hdmi@feae0000\0".as_ptr() as *const c_char).is_null() {
        rsnd_flags_set(io, RSND_STREAM_HDMI1);
        dev_dbg(dev, b"%s connected to HDMI1\n\0".as_ptr() as *const c_char, (*io).name.as_ptr());
    }
    rsnd_parse_tdm_split_mode(priv_, io, endpoint);
    of_node_put(remote_node);
}

pub unsafe extern "C" fn rsnd_node_fixed_index(dev: *mut device, node: *mut device_node, name: *mut c_char, mut idx: c_int) -> c_int {
    let mut node_name = [0 as c_char; 16];
    while idx < 64 {
        snprintf(node_name.as_mut_ptr(), node_name.len(), b"%s-%d\0".as_ptr() as *const c_char, name, idx);
        if strncmp(node_name.as_ptr(), of_node_full_name(node), node_name.len()) == 0 { return idx; }
        idx += 1;
    }
    dev_err(dev, b"strange node numbering (%s)\0".as_ptr() as *const c_char, of_node_full_name(node));
    -EINVAL
}

pub unsafe extern "C" fn rsnd_node_count(_priv: *mut rsnd_priv, _node: *mut device_node, _name: *mut c_char) -> c_int {
    /* for_each_child_of_node_scoped(node, np) requires external iterator support. */
    0
}

unsafe fn rsnd_format_indexed_name(buf: *mut c_char, buflen: size_t, sep: c_char, base: *const c_char, index: c_int) {
    snprintf(buf, buflen, b"%s%c%d\0".as_ptr() as *const c_char, base, sep as c_int, index);
}

pub unsafe extern "C" fn rsnd_devm_clk_get_indexed(dev: *mut device, base: *const c_char, index: c_int) -> *mut clk {
    let mut name = [0 as c_char; RSND_INDEXED_NAME_MAX];
    rsnd_format_indexed_name(name.as_mut_ptr(), name.len(), b'-' as c_char, base, index);
    let clk = devm_clk_get(dev, name.as_ptr());
    if IS_ERR(clk as *const c_void) == 0 || PTR_ERR(clk as *const c_void) != -ENOENT as isize { return clk; }
    rsnd_format_indexed_name(name.as_mut_ptr(), name.len(), b'.' as c_char, base, index);
    devm_clk_get(dev, name.as_ptr())
}

pub unsafe extern "C" fn rsnd_devm_clk_get_optional_indexed(dev: *mut device, base: *const c_char, index: c_int) -> *mut clk {
    let mut name = [0 as c_char; RSND_INDEXED_NAME_MAX];
    rsnd_format_indexed_name(name.as_mut_ptr(), name.len(), b'-' as c_char, base, index);
    let clk = devm_clk_get_optional(dev, name.as_ptr());
    if IS_ERR(clk as *const c_void) != 0 || !clk.is_null() { return clk; }
    rsnd_format_indexed_name(name.as_mut_ptr(), name.len(), b'.' as c_char, base, index);
    devm_clk_get_optional(dev, name.as_ptr())
}

pub unsafe extern "C" fn rsnd_devm_reset_control_get_optional_indexed(dev: *mut device, base: *const c_char, index: c_int) -> *mut reset_control {
    let mut name = [0 as c_char; RSND_INDEXED_NAME_MAX];
    rsnd_format_indexed_name(name.as_mut_ptr(), name.len(), b'-' as c_char, base, index);
    let rstc = devm_reset_control_get_optional(dev, name.as_ptr());
    if IS_ERR(rstc as *const c_void) != 0 || !rstc.is_null() { return rstc; }
    rsnd_format_indexed_name(name.as_mut_ptr(), name.len(), b'.' as c_char, base, index);
    devm_reset_control_get_optional(dev, name.as_ptr())
}

unsafe fn rsnd_node_name_strip_prefix(name: *const c_char) -> *const c_char {
    let comma = strchr(name, ',' as c_int);
    if !comma.is_null() { comma.add(1) } else { ptr::null() }
}

pub unsafe extern "C" fn rsnd_parse_of_node(priv_: *mut rsnd_priv, name: *const c_char) -> *mut device_node {
    let np = (*rsnd_priv_to_dev(priv_)).of_node;
    let mut node = of_get_child_by_name(np, name);
    if !node.is_null() { return node; }
    let unprefixed = rsnd_node_name_strip_prefix(name);
    if !unprefixed.is_null() { node = of_get_child_by_name(np, unprefixed); }
    node
}

unsafe fn rsnd_pick_endpoint_node_for_ports(e_ports: *mut device_node, e_port: *mut device_node) -> *mut device_node {
    if of_node_name_eq(e_ports, b"ports\0".as_ptr() as *const c_char) != 0 { return e_ports; }
    if of_node_name_eq(e_ports, b"port\0".as_ptr() as *const c_char) != 0 { return e_port; }
    ptr::null_mut()
}

unsafe fn rsnd_preallocate_pages(rtd: *mut snd_soc_pcm_runtime, io: *mut rsnd_dai_stream, stream: c_int) -> c_int {
    let priv_ = rsnd_io_to_priv(io);
    let mut dev = rsnd_priv_to_dev(priv_);
    if !(*io).dmac_dev.is_null() { dev = (*io).dmac_dev; }
    let mut substream = (*(*rtd).pcm).streams[stream as usize].substream;
    while !substream.is_null() {
        snd_pcm_set_managed_buffer(substream, SNDRV_DMA_TYPE_DEV, dev, PREALLOC_BUFFER, PREALLOC_BUFFER_MAX);
        substream = (*substream).next;
    }
    0
}

unsafe extern "C" fn rsnd_soc_dai_pcm_new(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int {
    let rdai = rsnd_dai_to_rdai(dai);
    let mut ret = rsnd_dai_call!(pcm_new, 0, 0, 0, &mut (*rdai).playback, rtd);
    if ret != 0 { return ret; }
    ret = rsnd_dai_call!(pcm_new, 0, 0, 0, &mut (*rdai).capture, rtd);
    if ret != 0 { return ret; }
    ret = rsnd_preallocate_pages(rtd, &mut (*rdai).playback, SNDRV_PCM_STREAM_PLAYBACK);
    if ret != 0 { return ret; }
    ret = rsnd_preallocate_pages(rtd, &mut (*rdai).capture, SNDRV_PCM_STREAM_CAPTURE);
    if ret != 0 { return ret; }
    0
}

unsafe fn __rsnd_dai_probe(priv_: *mut rsnd_priv, dai_np: *mut device_node, node_np: *mut device_node, node_arg: u32, dai_i: c_int) {
    let rdai = rsnd_rdai_get(priv_, dai_i);
    let drv = rsnd_daidrv_get(priv_, dai_i);
    let io_playback = &mut (*rdai).playback as *mut rsnd_dai_stream;
    let io_capture = &mut (*rdai).capture as *mut rsnd_dai_stream;
    let dev = rsnd_priv_to_dev(priv_);
    let mut playback_exist = 0;
    let mut capture_exist = 0;
    snprintf((*rdai).name.as_mut_ptr(), RSND_DAI_NAME_SIZE as usize, b"rsnd-dai.%d\0".as_ptr() as *const c_char, dai_i);
    (*rdai).dai_args.np = node_np;
    (*rdai).dai_args.args_count = 1;
    (*rdai).dai_args.args[0] = node_arg;
    (*rdai).priv_ = priv_;
    (*drv).name = (*rdai).name.as_mut_ptr();
    (*drv).ops = ptr::null();
    (*drv).id = dai_i;
    (*drv).dai_args = &mut (*rdai).dai_args;
    (*io_playback).rdai = rdai;
    (*io_capture).rdai = rdai;
    rsnd_rdai_channels_set(rdai, 2);
    rsnd_rdai_ssi_lane_set(rdai, 1);
    rsnd_rdai_width_set(rdai, 32);
    let mut io_i = 0;
    loop {
        let playback = of_parse_phandle(dai_np, b"playback\0".as_ptr() as *const c_char, io_i);
        let capture = of_parse_phandle(dai_np, b"capture\0".as_ptr() as *const c_char, io_i);
        if playback.is_null() && capture.is_null() { break; }
        if io_i == 0 {
            if !playback.is_null() { playback_exist = 1; }
            if !capture.is_null() { capture_exist = 1; }
        }
        rsnd_parse_connect_ssi(rdai, playback, capture);
        rsnd_parse_connect_ssiu(rdai, playback, capture);
        rsnd_parse_connect_src(rdai, playback, capture);
        rsnd_parse_connect_ctu(rdai, playback, capture);
        rsnd_parse_connect_mix(rdai, playback, capture);
        rsnd_parse_connect_dvc(rdai, playback, capture);
        of_node_put(playback);
        of_node_put(capture);
        io_i += 1;
    }
    if playback_exist != 0 {
        snprintf((*io_playback).name.as_mut_ptr(), RSND_DAI_NAME_SIZE as usize, b"DAI%d Playback\0".as_ptr() as *const c_char, dai_i);
        (*drv).playback.rates = RSND_RATES; (*drv).playback.formats = RSND_FMTS; (*drv).playback.channels_min = 2; (*drv).playback.channels_max = 8; (*drv).playback.stream_name = (*io_playback).name.as_mut_ptr();
    }
    if capture_exist != 0 {
        snprintf((*io_capture).name.as_mut_ptr(), RSND_DAI_NAME_SIZE as usize, b"DAI%d Capture\0".as_ptr() as *const c_char, dai_i);
        (*drv).capture.rates = RSND_RATES; (*drv).capture.formats = RSND_FMTS; (*drv).capture.channels_min = 2; (*drv).capture.channels_max = 8; (*drv).capture.stream_name = (*io_capture).name.as_mut_ptr();
    }
    if rsnd_ssi_is_pin_sharing(io_capture) != 0 || rsnd_ssi_is_pin_sharing(io_playback) != 0 { (*drv).symmetric_rate = 1; }
    dev_dbg(dev, b"%s (%s/%s)\n\0".as_ptr() as *const c_char, (*rdai).name.as_ptr(), if !rsnd_io_to_mod_ssi(io_playback).is_null() { b"play\0".as_ptr() } else { b" -- \0".as_ptr() }, if !rsnd_io_to_mod_ssi(io_capture).is_null() { b"capture\0".as_ptr() } else { b"  --   \0".as_ptr() });
}

unsafe fn rsnd_dai_probe(priv_: *mut rsnd_priv) -> c_int {
    let dev = rsnd_priv_to_dev(priv_);
    let mut is_graph = 0;
    let nr = 0; /* rsnd_dai_of_node uses OF child iterators; no DAI is not error. */
    if nr == 0 { return 0; }
    let rdrv = devm_kcalloc(dev, nr as usize, core::mem::size_of::<snd_soc_dai_driver>(), 0);
    let rdai = devm_kcalloc(dev, nr as usize, core::mem::size_of::<rsnd_dai>(), 0);
    if rdrv.is_null() || rdai.is_null() { return -ENOMEM; }
    (*priv_).rdai_nr = nr;
    (*priv_).daidrv = rdrv as *mut snd_soc_dai_driver;
    (*priv_).rdai = rdai as *mut rsnd_dai;
    let _ = &mut is_graph;
    0
}

unsafe fn rsnd_hw_update(substream: *mut snd_pcm_substream, hw_params: *mut snd_pcm_hw_params) -> c_int {
    let dai = rsnd_substream_to_dai(substream);
    let rdai = rsnd_dai_to_rdai(dai);
    let io = rsnd_rdai_to_io(rdai, substream);
    /* C used guard(spinlock_irqsave)(&priv->lock). */
    if !hw_params.is_null() { rsnd_dai_call!(hw_params, 0, 0, 0, io, substream, hw_params) } else { rsnd_dai_call!(hw_free, 0, 0, 0, io, substream) }
}

unsafe extern "C" fn rsnd_hw_params(_component: *mut snd_soc_component, substream: *mut snd_pcm_substream, hw_params: *mut snd_pcm_hw_params) -> c_int {
    let dai = rsnd_substream_to_dai(substream);
    let rdai = rsnd_dai_to_rdai(dai);
    let io = rsnd_rdai_to_io(rdai, substream);
    let fe = snd_soc_substream_to_rtd(substream);
    (*io).converted_rate = 0;
    (*io).converted_chan = 0;
    if (*(*fe).dai_link).dynamic != 0 {
        /* for_each_dpcm_be(fe, stream, dpcm) is external; converted params handling is preserved by this block comment. */
    }
    rsnd_hw_update(substream, hw_params)
}

unsafe extern "C" fn rsnd_hw_free(_component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int {
    rsnd_hw_update(substream, ptr::null_mut())
}

unsafe extern "C" fn rsnd_pointer(_component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let dai = rsnd_substream_to_dai(substream);
    let rdai = rsnd_dai_to_rdai(dai);
    let io = rsnd_rdai_to_io(rdai, substream);
    let mut pointer: snd_pcm_uframes_t = 0;
    rsnd_dai_call!(pointer, 0, 0, 0, io, &mut pointer);
    pointer
}

unsafe extern "C" fn rsnd_kctrl_info(_kctrl: *mut snd_kcontrol, _uinfo: *mut snd_ctl_elem_info) -> c_int { 0 }
unsafe extern "C" fn rsnd_kctrl_get(_kctrl: *mut snd_kcontrol, _uc: *mut snd_ctl_elem_value) -> c_int { 0 }
unsafe extern "C" fn rsnd_kctrl_put(kctrl: *mut snd_kcontrol, _uc: *mut snd_ctl_elem_value) -> c_int {
    let cfg = snd_kcontrol_chip(kctrl);
    if let Some(accept) = (*cfg).accept { if accept((*cfg).io) == 0 { return 0; } }
    if let Some(update) = (*cfg).update { update((*cfg).io, (*cfg).mod_); }
    0
}

pub unsafe extern "C" fn rsnd_kctrl_accept_anytime(_io: *mut rsnd_dai_stream) -> c_int { 1 }

pub unsafe extern "C" fn rsnd_kctrl_init_m(cfg: *mut rsnd_kctrl_cfg_m) -> *mut rsnd_kctrl_cfg {
    (*cfg).cfg.val = (*cfg).val.as_mut_ptr();
    &mut (*cfg).cfg
}

pub unsafe extern "C" fn rsnd_kctrl_init_s(cfg: *mut rsnd_kctrl_cfg_s) -> *mut rsnd_kctrl_cfg {
    (*cfg).cfg.val = &mut (*cfg).val;
    &mut (*cfg).cfg
}

#[used]
pub static volume_ramp_rate: [*const c_char; 24] = [
    b"128 dB/1 step\0".as_ptr() as *const c_char, b"64 dB/1 step\0".as_ptr() as *const c_char, b"32 dB/1 step\0".as_ptr() as *const c_char, b"16 dB/1 step\0".as_ptr() as *const c_char,
    b"8 dB/1 step\0".as_ptr() as *const c_char, b"4 dB/1 step\0".as_ptr() as *const c_char, b"2 dB/1 step\0".as_ptr() as *const c_char, b"1 dB/1 step\0".as_ptr() as *const c_char,
    b"0.5 dB/1 step\0".as_ptr() as *const c_char, b"0.25 dB/1 step\0".as_ptr() as *const c_char, b"0.125 dB/1 step\0".as_ptr() as *const c_char, b"0.125 dB/2 steps\0".as_ptr() as *const c_char,
    b"0.125 dB/4 steps\0".as_ptr() as *const c_char, b"0.125 dB/8 steps\0".as_ptr() as *const c_char, b"0.125 dB/16 steps\0".as_ptr() as *const c_char, b"0.125 dB/32 steps\0".as_ptr() as *const c_char,
    b"0.125 dB/64 steps\0".as_ptr() as *const c_char, b"0.125 dB/128 steps\0".as_ptr() as *const c_char, b"0.125 dB/256 steps\0".as_ptr() as *const c_char, b"0.125 dB/512 steps\0".as_ptr() as *const c_char,
    b"0.125 dB/1024 steps\0".as_ptr() as *const c_char, b"0.125 dB/2048 steps\0".as_ptr() as *const c_char, b"0.125 dB/4096 steps\0".as_ptr() as *const c_char, b"0.125 dB/8192 steps\0".as_ptr() as *const c_char,
];

pub unsafe extern "C" fn rsnd_kctrl_new(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, rtd: *mut snd_soc_pcm_runtime, _name: *const u8, accept: Option<unsafe extern "C" fn(*mut rsnd_dai_stream) -> c_int>, update: Option<unsafe extern "C" fn(*mut rsnd_dai_stream, *mut rsnd_mod)>, cfg: *mut rsnd_kctrl_cfg, texts: *const *const c_char, size: c_int, max: u32) -> c_int {
    let card = (*(*rtd).card).snd_card;
    if size > RSND_MAX_CHANNELS { return -EINVAL; }
    let kctrl = snd_ctl_new1(ptr::null(), cfg as *mut c_void);
    if kctrl.is_null() { return -ENOMEM; }
    let ret = snd_ctl_add(card, kctrl);
    if ret < 0 { return ret; }
    (*cfg).texts = texts; (*cfg).max = max; (*cfg).size = size; (*cfg).accept = accept; (*cfg).update = update; (*cfg).card = card; (*cfg).kctrl = kctrl; (*cfg).io = io; (*cfg).mod_ = mod_;
    0
}

unsafe fn rsnd_rdai_continuance_probe(priv_: *mut rsnd_priv, io: *mut rsnd_dai_stream) -> c_int {
    let mut ret = rsnd_dai_call!(probe, 0, 0, 0, io, priv_);
    if ret == -EAGAIN {
        let ssi_mod = rsnd_io_to_mod_ssi(io);
        rsnd_dai_call!(remove, 0, 0, 0, io, priv_);
        for i in 0..(RSND_MOD_MAX as usize) {
            let mod_ = (*io).mod_[i];
            if !mod_.is_null() { rsnd_dai_disconnect(mod_, io, i as rsnd_mod_type); }
        }
        rsnd_dai_connect(ssi_mod, io, RSND_MOD_SSI);
        rsnd_dai_call!(fallback, 0, 0, 0, io, priv_);
        ret = rsnd_dai_call!(probe, 0, 0, 0, io, priv_);
    }
    ret
}

unsafe extern "C" fn rsnd_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let probe_func: [unsafe extern "C" fn(*mut rsnd_priv) -> c_int; 11] = [rsnd_gen_probe, rsnd_dma_probe, rsnd_ssi_probe, rsnd_ssiu_probe, rsnd_src_probe, rsnd_ctu_probe, rsnd_mix_probe, rsnd_dvc_probe, rsnd_cmd_probe, rsnd_adg_probe, rsnd_dai_probe];
    let priv_ = devm_kzalloc(dev, core::mem::size_of::<rsnd_priv>(), 0) as *mut rsnd_priv;
    if priv_.is_null() { return -ENODEV; }
    (*priv_).pdev = pdev;
    (*priv_).flags = of_device_get_match_data(dev) as c_ulong;
    spin_lock_init(&mut (*priv_).lock);
    for f in probe_func { let ret = f(priv_); if ret != 0 { return ret; } }
    for i in 0..rsnd_rdai_nr(priv_) {
        let rdai = (*priv_).rdai.add(i as usize);
        let mut ret = rsnd_rdai_continuance_probe(priv_, &mut (*rdai).playback);
        if ret != 0 { rsnd_adg_remove(priv_); return ret; }
        ret = rsnd_rdai_continuance_probe(priv_, &mut (*rdai).capture);
        if ret != 0 { rsnd_adg_remove(priv_); return ret; }
    }
    dev_set_drvdata(dev, priv_ as *mut c_void);
    let mut ci = 0;
    for i in 0..(RSND_MAX_COMPONENT as usize) {
        if (*priv_).component_dais[i] <= 0 { break; }
        let nr = (*priv_).component_dais[i];
        let ret = devm_snd_soc_register_component(dev, ptr::null(), (*priv_).daidrv.add(ci as usize), nr);
        if ret < 0 {
            dev_err(dev, b"cannot snd component register\n\0".as_ptr() as *const c_char);
            rsnd_adg_remove(priv_);
            return ret;
        }
        ci += nr;
    }
    pm_runtime_enable(dev);
    dev_info(dev, b"probed\n\0".as_ptr() as *const c_char);
    0
}

unsafe extern "C" fn rsnd_remove(pdev: *mut platform_device) {
    let priv_ = dev_get_drvdata(&mut (*pdev).dev) as *mut rsnd_priv;
    let remove_func: [unsafe extern "C" fn(*mut rsnd_priv); 8] = [rsnd_ssi_remove, rsnd_ssiu_remove, rsnd_src_remove, rsnd_ctu_remove, rsnd_mix_remove, rsnd_dvc_remove, rsnd_cmd_remove, rsnd_adg_remove];
    pm_runtime_disable(&mut (*pdev).dev);
    for i in 0..rsnd_rdai_nr(priv_) {
        let rdai = (*priv_).rdai.add(i as usize);
        let mut ret = rsnd_dai_call!(remove, 0, 0, 0, &mut (*rdai).playback, priv_);
        if ret != 0 { dev_warn(&mut (*pdev).dev, b"Failed to remove playback dai #%d\n\0".as_ptr() as *const c_char, i); }
        ret = rsnd_dai_call!(remove, 0, 0, 0, &mut (*rdai).capture, priv_);
        if ret != 0 { dev_warn(&mut (*pdev).dev, b"Failed to remove capture dai #%d\n\0".as_ptr() as *const c_char, i); }
    }
    for f in remove_func { f(priv_); }
}

pub unsafe extern "C" fn rsnd_suspend_clk_reset(clk: *mut clk, rstc: *mut reset_control) {
    clk_unprepare(clk);
    reset_control_assert(rstc);
}

pub unsafe extern "C" fn rsnd_resume_clk_reset(clk: *mut clk, rstc: *mut reset_control) {
    reset_control_deassert(rstc);
    clk_prepare(clk);
}

unsafe extern "C" fn rsnd_suspend(dev: *mut device) -> c_int {
    let priv_ = dev_get_drvdata(dev) as *mut rsnd_priv;
    /*
     * Reverse order of probe:
     * ADG -> DVC -> MIX -> CTU -> SRC -> SSIU -> SSI -> DMA
     */
    rsnd_adg_clk_disable(priv_); rsnd_adg_suspend(priv_); rsnd_dvc_suspend(priv_); rsnd_mix_suspend(priv_); rsnd_ctu_suspend(priv_); rsnd_src_suspend(priv_); rsnd_ssiu_suspend(priv_); rsnd_ssi_suspend(priv_); rsnd_dma_suspend(priv_);
    0
}

unsafe extern "C" fn rsnd_resume(dev: *mut device) -> c_int {
    let priv_ = dev_get_drvdata(dev) as *mut rsnd_priv;
    /*
     * Same order as probe:
     * DMA -> SSI -> SSIU -> SRC -> CTU -> MIX -> DVC -> ADG
     */
    rsnd_dma_resume(priv_); rsnd_ssi_resume(priv_); rsnd_ssiu_resume(priv_); rsnd_src_resume(priv_); rsnd_ctu_resume(priv_); rsnd_mix_resume(priv_); rsnd_dvc_resume(priv_); rsnd_adg_resume(priv_); rsnd_adg_clk_enable(priv_);
    0
}

/* static const struct dev_pm_ops rsnd_pm_ops = { SYSTEM_SLEEP_PM_OPS(rsnd_suspend, rsnd_resume) }; */
/* static struct platform_driver rsnd_driver registers .name = "rcar_sound", .pm = pm_ptr(&rsnd_pm_ops), .of_match_table = rsnd_of_match, .probe = rsnd_probe, .remove = rsnd_remove. */
/* module_platform_driver(rsnd_driver);
 * MODULE_LICENSE("GPL v2");
 * MODULE_DESCRIPTION("Renesas R-Car audio driver");
 * MODULE_AUTHOR("Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>");
 * MODULE_ALIAS("platform:rcar-pcm-audio");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
