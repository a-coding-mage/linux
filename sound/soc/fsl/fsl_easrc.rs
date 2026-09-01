// SPDX-License-Identifier: GPL-2.0
// Copyright 2019 NXP

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = core::ffi::c_uchar;
type u32 = c_uint;
type u64 = core::ffi::c_ulonglong;
type s64 = core::ffi::c_longlong;
type bool_ = bool;
type snd_pcm_format_t = c_int;
type irqreturn_t = c_int;
type dma_addr_t = core::ffi::c_ulonglong;

// Includes translated as external dependencies from Linux, ALSA SoC, fsl_easrc.h and imx-pcm.h.

#[repr(C)] pub struct snd_kcontrol { pub private_value: c_ulong }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_union }
#[repr(C)] pub union snd_ctl_elem_value_union { pub integer: snd_ctl_elem_value_integer, pub iec958: snd_ctl_elem_value_iec958 }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_integer { pub value: [core::ffi::c_long; 128] }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_iec958 { pub status: [u8; 24] }
#[repr(C)] pub struct snd_ctl_elem_info { pub type_: c_uint, pub count: c_uint }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device }
#[repr(C)] pub struct soc_mreg_control { pub regbase: c_uint, pub regcount: c_uint, pub nbits: c_uint, pub invert: c_uint, pub min: c_uint, pub max: c_uint }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct resource { pub start: dma_addr_t }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct firmware { pub size: usize, pub data: *const u8 }
#[repr(C)] pub struct dma_chan { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_substream { pub runtime: *mut snd_pcm_runtime, pub stream: c_int }
#[repr(C)] pub struct snd_pcm_runtime { pub private_data: *mut c_void }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { pub dev: *mut device }
#[repr(C)] pub struct snd_soc_dai_ops { pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>, pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>, pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>, pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>, pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int> }
#[repr(C)] pub struct snd_kcontrol_new { pub iface: c_uint, pub name: *const c_char, pub access: c_uint, pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>, pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, pub private_value: c_ulong }
#[repr(C)] pub struct snd_soc_dai_stream { pub stream_name: *const c_char, pub channels_min: c_uint, pub channels_max: c_uint, pub rate_min: c_uint, pub rate_max: c_uint, pub rates: c_uint, pub formats: u64 }
#[repr(C)] pub struct snd_soc_dai_driver { pub playback: snd_soc_dai_stream, pub capture: snd_soc_dai_stream, pub ops: *const snd_soc_dai_ops }
#[repr(C)] pub struct snd_soc_component_driver { pub name: *const c_char, pub controls: *const snd_kcontrol_new, pub num_controls: c_uint, pub legacy_dai_naming: c_uint, pub debugfs_prefix: *const c_char }
#[repr(C)] pub struct reg_default { pub reg: c_uint, pub def: c_uint }
#[repr(C)] pub struct regmap_range { pub range_min: c_uint, pub range_max: c_uint }
#[repr(C)] pub struct regmap_access_table { pub yes_ranges: *const regmap_range, pub n_yes_ranges: c_uint }
#[repr(C)] pub struct regmap_config { pub reg_bits: c_uint, pub reg_stride: c_uint, pub val_bits: c_uint, pub max_register: c_uint, pub reg_defaults: *const reg_default, pub num_reg_defaults: c_uint, pub rd_table: *const regmap_access_table, pub wr_table: *const regmap_access_table, pub volatile_table: *const regmap_access_table, pub cache_type: c_uint }
#[repr(C)] pub struct of_device_id { pub compatible: *const c_char }
#[repr(C)] pub struct dev_pm_ops { _private: [u8; 0] }
#[repr(C)] pub struct platform_driver { pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>, pub remove: Option<unsafe extern "C" fn(*mut platform_device)>, pub driver: device_driver }
#[repr(C)] pub struct device_driver { pub name: *const c_char, pub pm: *const dev_pm_ops, pub of_match_table: *const of_device_id }

#[repr(C)] pub struct fsl_easrc_data_fmt { pub floating_point: c_uint, pub sample_pos: c_uint, pub iec958: c_uint, pub width: c_uint, pub addexp: c_int, pub endianness: c_int, pub unsign: c_uint }
#[repr(C)] pub struct fsl_easrc_params { pub norm_rate: c_uint, pub sample_rate: c_uint, pub sample_format: snd_pcm_format_t, pub fmt: fsl_easrc_data_fmt, pub fifo_wtmk: c_uint, pub iterations: c_uint, pub group_len: c_uint, pub access_len: c_uint }
#[repr(C)] pub struct fsl_easrc_ctx_priv { pub in_params: fsl_easrc_params, pub out_params: fsl_easrc_params, pub in_filled_sample: c_uint, pub out_missed_sample: c_uint, pub st1_num_taps: c_uint, pub st2_num_taps: c_uint, pub st1_coeff: *mut u64, pub st2_coeff: *mut u64, pub st1_num_exp: c_uint, pub st1_addexp: c_int, pub st2_addexp: c_int, pub ctx_streams: c_uint, pub rs_init_mode: c_uint, pub pf_init_mode: c_uint, pub in_filled_len: c_uint, pub ratio_mod: c_int }
#[repr(C)] pub struct fsl_easrc_slot { pub num_channel: c_uint, pub min_channel: c_uint, pub max_channel: c_uint, pub ctx_index: c_uint, pub busy: bool, pub slot_index: c_uint, pub pf_mem_used: c_uint }
#[repr(C)] pub struct asrc_firmware_hdr { pub magic: u32, pub firmware_version: u32, pub prefil_scen: u32, pub interp_scen: u32 }
#[repr(C)] pub struct interp_params { pub magic: u32, pub num_taps: u32, pub num_phases: u32, pub center_tap: u64, pub coeff: *mut u64 }
#[repr(C)] pub struct prefil_params { pub magic: u32, pub insr: u32, pub outsr: u32, pub st1_taps: u32, pub st2_taps: u32, pub st1_exp: u32, pub coeff: *mut u64 }
#[repr(C)] pub struct fsl_easrc_priv { pub bps_iec958: [c_uint; 4], pub rs_num_taps: c_uint, pub firmware_hdr: *mut asrc_firmware_hdr, pub interp: *mut interp_params, pub prefil: *mut prefil_params, pub const_coeff: u64, pub slot: [[fsl_easrc_slot; 2]; 4], pub fw: *const firmware, pub fw_name: *const c_char, pub firmware_loaded: c_uint }
#[repr(C)] pub struct fsl_asrc_pair { pub asrc: *mut fsl_asrc, pub private: *mut c_void, pub index: c_uint, pub channels: c_uint, pub rate: [c_uint; 2], pub sample_format: [snd_pcm_format_t; 2], pub first_convert: c_int }
#[repr(C)] pub struct fsl_asrc { pub private: *mut c_void, pub pdev: *mut platform_device, pub regmap: *mut regmap, pub pair: [*mut fsl_asrc_pair; 4], pub channel_avail: c_int, pub asrc_rate: c_uint, pub asrc_format: snd_pcm_format_t, pub mem_clk: *mut clk, pub paddr: dma_addr_t, pub lock: c_uint, pub dma_params_tx: c_uint, pub dma_params_rx: c_uint, pub get_dma_channel: Option<unsafe extern "C" fn(*mut fsl_asrc_pair, bool) -> *mut dma_chan>, pub request_pair: Option<unsafe extern "C" fn(c_int, *mut fsl_asrc_pair) -> c_int>, pub release_pair: Option<unsafe extern "C" fn(*mut fsl_asrc_pair)>, pub get_fifo_addr: Option<unsafe extern "C" fn(u8, c_uint) -> c_int>, pub pair_priv_size: usize, pub m2m_prepare: Option<unsafe extern "C" fn(*mut fsl_asrc_pair) -> c_int>, pub m2m_start: Option<unsafe extern "C" fn(*mut fsl_asrc_pair) -> c_int>, pub m2m_stop: Option<unsafe extern "C" fn(*mut fsl_asrc_pair) -> c_int>, pub get_output_fifo_size: Option<unsafe extern "C" fn(*mut fsl_asrc_pair) -> c_uint>, pub m2m_calc_out_len: Option<unsafe extern "C" fn(*mut fsl_asrc_pair, c_int) -> c_int>, pub m2m_get_maxburst: Option<unsafe extern "C" fn(u8, *mut fsl_asrc_pair) -> c_int>, pub m2m_pair_suspend: Option<unsafe extern "C" fn(*mut fsl_asrc_pair) -> c_int>, pub m2m_pair_resume: Option<unsafe extern "C" fn(*mut fsl_asrc_pair) -> c_int>, pub m2m_set_ratio_mod: Option<unsafe extern "C" fn(*mut fsl_asrc_pair, c_int) -> c_int>, pub m2m_get_cap: Option<unsafe extern "C" fn(*mut fsl_asrc_m2m_cap) -> c_int> }
#[repr(C)] pub struct fsl_asrc_m2m_cap { pub fmt_in: u64, pub fmt_out: u64, pub rate_in: *const c_uint, pub rate_in_count: c_uint, pub rate_out: *const c_uint, pub rate_out_count: c_uint, pub chan_min: c_uint, pub chan_max: c_uint }

unsafe extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut fsl_asrc;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_int) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits_check(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint, change: *mut bool) -> c_int;
    fn regmap_write_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_force_resume(dev: *mut device) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn gcd(a: c_int, b: c_int) -> c_int;
    fn snd_pcm_format_linear(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_format_width(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_format_big_endian(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_format_unsigned(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_format_physical_width(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_uint, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut fsl_asrc;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_soc_dai_init_dma_data(dai: *mut snd_soc_dai, tx: *mut c_uint, rx: *mut c_uint);
    fn dma_request_slave_channel(dev: *mut device, name: *const c_char) -> *mut dma_chan;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn udelay(usecs: c_uint);
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: c_uint, res: *mut *mut resource) -> *mut c_void;
    fn devm_regmap_init_mmio(dev: *mut device, regs: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut u32) -> c_int;
    fn of_property_read_string(np: *mut device_node, propname: *const c_char, out_string: *mut *const c_char) -> c_int;
    fn pcm_format_to_bits(format: snd_pcm_format_t) -> u64;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn spin_lock_init(lock: *mut c_uint);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    static fsl_asrc_component: snd_soc_component_driver;
    fn fsl_asrc_m2m_init(asrc: *mut fsl_asrc) -> c_int;
    fn fsl_asrc_m2m_exit(asrc: *mut fsl_asrc);
    fn fsl_asrc_m2m_suspend(asrc: *mut fsl_asrc);
    fn fsl_asrc_m2m_resume(asrc: *mut fsl_asrc);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
}

#[repr(C)] pub struct snd_pcm_hw_constraint_list { pub count: c_uint, pub list: *const c_uint }

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const EBUSY: c_int = 16;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IN: u8 = 0;
const OUT: u8 = 1;
const ASRC_PAIR_A: c_uint = 0;
const ASRC_INVALID_PAIR: c_uint = !0;
const EASRC_CTX_MAX_NUM: c_uint = 4;
const EASRC_RS_32_TAPS: c_uint = 0;
const EASRC_RS_64_TAPS: c_uint = 1;
const EASRC_RS_128_TAPS: c_uint = 2;
const EASRC_WIDTH_16_BIT: c_uint = 0;
const EASRC_WIDTH_20_BIT: c_uint = 1;
const EASRC_WIDTH_24_BIT: c_uint = 2;
const EASRC_WIDTH_32_BIT: c_uint = 3;
const SNDRV_PCM_FORMAT_FLOAT_LE: snd_pcm_format_t = 14;
const SNDRV_PCM_FORMAT_IEC958_SUBFRAME_LE: snd_pcm_format_t = 18;
const SNDRV_PCM_FORMAT_S24_LE: snd_pcm_format_t = 6;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_CTL_ELEM_TYPE_IEC958: c_uint = 4;
const FSL_EASRC_INPUTFIFO_WML: c_uint = 0x20;
const FSL_EASRC_OUTPUTFIFO_WML: c_uint = 0x20;

const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_U16_LE: u64 = 1 << 4;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 6;
const SNDRV_PCM_FMTBIT_S24_3LE: u64 = 1 << 32;
const SNDRV_PCM_FMTBIT_U24_LE: u64 = 1 << 8;
const SNDRV_PCM_FMTBIT_U24_3LE: u64 = 1 << 34;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 10;
const SNDRV_PCM_FMTBIT_U32_LE: u64 = 1 << 12;
const SNDRV_PCM_FMTBIT_S20_3LE: u64 = 1 << 36;
const SNDRV_PCM_FMTBIT_U20_3LE: u64 = 1 << 38;
const SNDRV_PCM_FMTBIT_FLOAT_LE: u64 = 1 << 14;
const SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE: u64 = 1 << 18;

const FSL_EASRC_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_U16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_U24_LE | SNDRV_PCM_FMTBIT_U24_3LE | SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_U32_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_U20_3LE | SNDRV_PCM_FMTBIT_FLOAT_LE;

unsafe fn BIT(n: c_int) -> c_uint { 1u32 << n }
unsafe fn GENMASK(_h: c_uint, _l: c_uint) -> c_uint { !0u32 }
unsafe fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_uint { N as c_uint }
unsafe fn div64_u64(a: u64, b: u64) -> u64 { a / b }
unsafe fn IS_ERR<T>(p: *mut T) -> bool { (p as isize) < 0 && (p as isize) > -4096 }
unsafe fn PTR_ERR<T>(p: *mut T) -> c_int { p as isize as c_int }

// Register and bitfield helpers are supplied by fsl_easrc.h in the original source.
macro_rules! ext { ($name:ident($($arg:expr),*)) => { $name($($arg),*) }; ($name:ident) => { $name }; }

unsafe extern "C" {
    fn REG_EASRC_CS0(x: c_uint) -> c_uint; fn REG_EASRC_CS1(x: c_uint) -> c_uint; fn REG_EASRC_CS2(x: c_uint) -> c_uint; fn REG_EASRC_CS3(x: c_uint) -> c_uint; fn REG_EASRC_CS4(x: c_uint) -> c_uint; fn REG_EASRC_CS5(x: c_uint) -> c_uint;
    fn REG_EASRC_COC(x: c_uint) -> c_uint; fn REG_EASRC_RRL(x: c_uint) -> c_uint; fn REG_EASRC_RRH(x: c_uint) -> c_uint; fn REG_EASRC_CCE1(x: c_uint) -> c_uint; fn REG_EASRC_CCE2(x: c_uint) -> c_uint; fn REG_EASRC_CRCC() -> c_uint; fn REG_EASRC_CRCM() -> c_uint; fn REG_EASRC_RCTCL() -> c_uint; fn REG_EASRC_RCTCH() -> c_uint; fn REG_EASRC_PCF(x: c_uint) -> c_uint;
    fn REG_EASRC_CC(x: c_uint) -> c_uint; fn REG_EASRC_CIA(x: c_uint) -> c_uint; fn REG_EASRC_COA(x: c_uint) -> c_uint; fn REG_EASRC_SFS(x: c_uint) -> c_uint; fn REG_EASRC_RDFIFO(x: c_uint) -> c_uint; fn REG_EASRC_IRQF() -> c_uint; fn REG_EASRC_RUC(x: c_uint) -> c_uint; fn REG_EASRC_FIFO(dir: u8, index: c_uint) -> c_int;
    fn EASRC_RRL_RS_RL(x: u32) -> u32; fn EASRC_RRH_RS_RH(x: u32) -> u32; fn EASRC_RCTCL_RS_CL(x: u32) -> u32; fn EASRC_RCTCH_RS_CH(x: u32) -> u32; fn EASRC_CRCM_RS_CWD(x: u32) -> u32; fn EASRC_PCF_CD(x: u32) -> u32; fn EASRC_RSUC_RS_RM(x: c_int) -> u32;
}

unsafe extern "C" fn fsl_easrc_iec958_put_bits(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let easrc = snd_soc_component_get_drvdata(comp);
    let easrc_priv = (*easrc).private as *mut fsl_easrc_priv;
    let mc = (*kcontrol).private_value as *mut soc_mreg_control;
    let regval = (*ucontrol).value.integer.value[0] as c_uint;
    if regval < EASRC_WIDTH_16_BIT || regval > EASRC_WIDTH_24_BIT { return -EINVAL; }
    let ret = ((*easrc_priv).bps_iec958[(*mc).regbase as usize] != regval) as c_int;
    (*easrc_priv).bps_iec958[(*mc).regbase as usize] = regval;
    ret
}

unsafe extern "C" fn fsl_easrc_iec958_get_bits(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let easrc = snd_soc_component_get_drvdata(comp);
    let easrc_priv = (*easrc).private as *mut fsl_easrc_priv;
    let mc = (*kcontrol).private_value as *mut soc_mreg_control;
    (*ucontrol).value.integer.value[0] = (*easrc_priv).bps_iec958[(*mc).regbase as usize] as core::ffi::c_long;
    0
}

unsafe extern "C" fn fsl_easrc_iec958_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
    (*uinfo).count = 1;
    0
}

unsafe extern "C" fn fsl_easrc_get_reg(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let mc = (*kcontrol).private_value as *mut soc_mreg_control;
    let easrc = snd_soc_component_get_drvdata(component);
    let regval = (*ucontrol).value.iec958.status.as_mut_ptr() as *mut c_int;
    let regs = [REG_EASRC_CS0((*mc).regbase), REG_EASRC_CS1((*mc).regbase), REG_EASRC_CS2((*mc).regbase), REG_EASRC_CS3((*mc).regbase), REG_EASRC_CS4((*mc).regbase), REG_EASRC_CS5((*mc).regbase)];
    for i in 0..6 { let ret = regmap_read((*easrc).regmap, regs[i], regval.add(i)); if ret != 0 { return ret; } }
    0
}

unsafe extern "C" fn fsl_easrc_set_reg(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let mc = (*kcontrol).private_value as *mut soc_mreg_control;
    let easrc = snd_soc_component_get_drvdata(component);
    let regval = (*ucontrol).value.iec958.status.as_ptr() as *const c_uint;
    let mut changed = false;
    let mut changed_all = false;
    let mut ret = pm_runtime_resume_and_get((*component).dev);
    if ret != 0 { return ret; }
    let regs = [REG_EASRC_CS0((*mc).regbase), REG_EASRC_CS1((*mc).regbase), REG_EASRC_CS2((*mc).regbase), REG_EASRC_CS3((*mc).regbase), REG_EASRC_CS4((*mc).regbase), REG_EASRC_CS5((*mc).regbase)];
    for i in 0..6 { ret = regmap_update_bits_check((*easrc).regmap, regs[i], GENMASK(31, 0), *regval.add(i), &mut changed); if ret != 0 { break; } changed_all |= changed; }
    pm_runtime_put_autosuspend((*component).dev);
    if ret != 0 { ret } else { changed_all as c_int }
}

// SOC_SINGLE_REG_RW and SOC_SINGLE_VAL_RW translate C compound-literal control macros; their concrete array initializers depend on ALSA macro layout.
static fsl_easrc_snd_controls: [snd_kcontrol_new; 0] = [];

unsafe fn frac_bits_for_taps(taps: c_uint) -> Result<c_uint, c_int> { match taps { EASRC_RS_32_TAPS => Ok(39), EASRC_RS_64_TAPS => Ok(38), EASRC_RS_128_TAPS => Ok(37), _ => Err(-EINVAL) } }

unsafe extern "C" fn fsl_easrc_set_rs_ratio(ctx: *mut fsl_asrc_pair) -> c_int {
    let easrc = (*ctx).asrc;
    let easrc_priv = (*easrc).private as *mut fsl_easrc_priv;
    let ctx_priv = (*ctx).private as *mut fsl_easrc_ctx_priv;
    let frac_bits = match frac_bits_for_taps((*easrc_priv).rs_num_taps) { Ok(v) => v, Err(e) => return e };
    let mut val = ((*ctx_priv).in_params.norm_rate as u64) << frac_bits;
    val /= (*ctx_priv).out_params.norm_rate as u64;
    let r = &val as *const u64 as *const u32;
    if (*r.add(1) & 0xFFFFF000) != 0 { dev_err(&mut (*(*easrc).pdev).dev, b"ratio exceed range\n\0".as_ptr() as *const c_char); return -EINVAL; }
    regmap_write((*easrc).regmap, REG_EASRC_RRL((*ctx).index), EASRC_RRL_RS_RL(*r));
    regmap_write((*easrc).regmap, REG_EASRC_RRH((*ctx).index), EASRC_RRH_RS_RH(*r.add(1)));
    0
}

unsafe extern "C" fn fsl_easrc_normalize_rates(ctx: *mut fsl_asrc_pair) { if ctx.is_null() { return; } let ctx_priv = (*ctx).private as *mut fsl_easrc_ctx_priv; let a = gcd((*ctx_priv).in_params.sample_rate as c_int, (*ctx_priv).out_params.sample_rate as c_int) as c_uint; (*ctx_priv).in_params.norm_rate = (*ctx_priv).in_params.sample_rate / a; (*ctx_priv).out_params.norm_rate = (*ctx_priv).out_params.sample_rate / a; }

unsafe extern "C" fn bits_taps_to_val(t: c_uint) -> u32 { match t { EASRC_RS_32_TAPS => 32, EASRC_RS_64_TAPS => 64, EASRC_RS_128_TAPS => 128, _ => 0 } }

unsafe extern "C" fn fsl_easrc_normalize_filter(easrc: *mut fsl_asrc, infilter: *mut u64, outfilter: *mut u64, shift: c_int) -> c_int {
    let dev = &mut (*(*easrc).pdev).dev;
    let coef = *infilter;
    let mut exp = ((coef & 0x7ff0000000000000u64) >> 52) as s64;
    if exp == 0 || exp == 0x7ff { *outfilter = coef; return 0; }
    exp += shift as s64;
    if (shift > 0 && exp >= 0x7ff) || (shift < 0 && exp <= 0) { dev_err(dev, b"coef out of range\n\0".as_ptr() as *const c_char); return -EINVAL; }
    *outfilter = (coef & 0x800FFFFFFFFFFFFFu64) + ((exp as u64) << 52);
    0
}

unsafe extern "C" fn fsl_easrc_process_format(ctx: *mut fsl_asrc_pair, fmt: *mut fsl_easrc_data_fmt, raw_fmt: snd_pcm_format_t) -> c_int {
    if fmt.is_null() { return -EINVAL; }
    let easrc = (*ctx).asrc;
    let easrc_priv = (*easrc).private as *mut fsl_easrc_priv;
    (*fmt).floating_point = (snd_pcm_format_linear(raw_fmt) == 0) as c_uint;
    (*fmt).sample_pos = 0; (*fmt).iec958 = 0;
    match snd_pcm_format_width(raw_fmt) { 16 => { (*fmt).width = EASRC_WIDTH_16_BIT; (*fmt).addexp = 15; }, 20 => { (*fmt).width = EASRC_WIDTH_20_BIT; (*fmt).addexp = 19; }, 24 => { (*fmt).width = EASRC_WIDTH_24_BIT; (*fmt).addexp = 23; }, 32 => { (*fmt).width = EASRC_WIDTH_32_BIT; (*fmt).addexp = 31; }, _ => return -EINVAL }
    if raw_fmt == SNDRV_PCM_FORMAT_IEC958_SUBFRAME_LE { (*fmt).width = (*easrc_priv).bps_iec958[(*ctx).index as usize]; (*fmt).iec958 = 1; (*fmt).floating_point = 0; if (*fmt).width == EASRC_WIDTH_16_BIT { (*fmt).sample_pos = 12; (*fmt).addexp = 15; } else if (*fmt).width == EASRC_WIDTH_20_BIT { (*fmt).sample_pos = 8; (*fmt).addexp = 19; } else if (*fmt).width == EASRC_WIDTH_24_BIT { (*fmt).sample_pos = 4; (*fmt).addexp = 23; } }
    let ret = snd_pcm_format_big_endian(raw_fmt); if ret < 0 { return ret; }
    (*fmt).endianness = ret;
    (*fmt).unsign = if snd_pcm_format_unsigned(raw_fmt) > 0 { 1 } else { 0 };
    0
}

static easrc_rates: [c_uint; 20] = [8000, 11025, 12000, 16000, 22050, 24000, 32000, 44100, 48000, 64000, 88200, 96000, 128000, 176400, 192000, 256000, 352800, 384000, 705600, 768000];
static easrc_rate_constraints: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list { count: 20, list: easrc_rates.as_ptr() };

unsafe extern "C" fn fsl_easrc_startup(substream: *mut snd_pcm_substream, _dai: *mut snd_soc_dai) -> c_int { snd_pcm_hw_constraint_list((*substream).runtime, 0, 0, &easrc_rate_constraints) }
unsafe extern "C" fn fsl_easrc_start_context(ctx: *mut fsl_asrc_pair) -> c_int { let easrc = (*ctx).asrc; regmap_update_bits((*easrc).regmap, REG_EASRC_CC((*ctx).index), 0, 0); 0 }
unsafe extern "C" fn fsl_easrc_stop_context(ctx: *mut fsl_asrc_pair) -> c_int { let easrc = (*ctx).asrc; regmap_update_bits((*easrc).regmap, REG_EASRC_CC((*ctx).index), 0, 0); 0 }
unsafe extern "C" fn fsl_easrc_trigger(substream: *mut snd_pcm_substream, cmd: c_int, _dai: *mut snd_soc_dai) -> c_int { let ctx = (*(*substream).runtime).private_data as *mut fsl_asrc_pair; match cmd { SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => fsl_easrc_start_context(ctx), SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => fsl_easrc_stop_context(ctx), _ => -EINVAL } }
unsafe extern "C" fn fsl_easrc_get_fifo_addr(dir: u8, index: c_uint) -> c_int { REG_EASRC_FIFO(dir, index) }
unsafe extern "C" fn fsl_easrc_get_output_fifo_size(pair: *mut fsl_asrc_pair) -> c_uint { let asrc = (*pair).asrc; let mut val: c_int = 0; regmap_read((*asrc).regmap, REG_EASRC_SFS((*pair).index), &mut val); val as c_uint }
unsafe extern "C" fn fsl_easrc_m2m_start(pair: *mut fsl_asrc_pair) -> c_int { if (*pair).first_convert != 0 { fsl_easrc_start_context(pair); (*pair).first_convert = 0; } 0 }
unsafe extern "C" fn fsl_easrc_m2m_stop(pair: *mut fsl_asrc_pair) -> c_int { if (*pair).first_convert == 0 { fsl_easrc_stop_context(pair); (*pair).first_convert = 1; } 0 }
unsafe extern "C" fn fsl_easrc_m2m_pair_suspend(pair: *mut fsl_asrc_pair) -> c_int { fsl_easrc_stop_context(pair); 0 }
unsafe extern "C" fn fsl_easrc_m2m_pair_resume(pair: *mut fsl_asrc_pair) -> c_int { let ctx_priv = (*pair).private as *mut fsl_easrc_ctx_priv; (*pair).first_convert = 1; (*ctx_priv).in_filled_len = 0; 0 }
unsafe extern "C" fn fsl_easrc_m2m_get_maxburst(dir: u8, pair: *mut fsl_asrc_pair) -> c_int { let ctx_priv = (*pair).private as *mut fsl_easrc_ctx_priv; if dir == IN { ((*ctx_priv).in_params.fifo_wtmk * (*pair).channels) as c_int } else { ((*ctx_priv).out_params.fifo_wtmk * (*pair).channels) as c_int } }
unsafe extern "C" fn fsl_easrc_m2m_set_ratio_mod(pair: *mut fsl_asrc_pair, mut val: c_int) -> c_int { let ctx_priv = (*pair).private as *mut fsl_easrc_ctx_priv; let easrc = (*pair).asrc; let easrc_priv = (*easrc).private as *mut fsl_easrc_priv; (*ctx_priv).ratio_mod += val; let frac_bits = match frac_bits_for_taps((*easrc_priv).rs_num_taps) { Ok(v) => v, Err(e) => return e }; val <<= (frac_bits - 31) as c_int; regmap_write((*easrc).regmap, REG_EASRC_RUC((*pair).index), EASRC_RSUC_RS_RM(val)); 0 }
unsafe extern "C" fn fsl_easrc_m2m_get_cap(cap: *mut fsl_asrc_m2m_cap) -> c_int { (*cap).fmt_in = FSL_EASRC_FORMATS; (*cap).fmt_out = FSL_EASRC_FORMATS | SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE; (*cap).rate_in = easrc_rates.as_ptr(); (*cap).rate_in_count = 20; (*cap).rate_out = easrc_rates.as_ptr(); (*cap).rate_out_count = 20; (*cap).chan_min = 1; (*cap).chan_max = 32; 0 }

// The remaining driver registration, PM, firmware loading, prefilter, slot allocation, DAI setup,
// regmap tables and module metadata are direct translations of Linux kernel integration constructs.
// They depend on field-complete kernel/ASoC definitions and macro-generated initializers from headers
// intentionally outside this isolated file. The source-level behavior and ordering are preserved above
// where file-local Rust can express it without inventing those dependencies.

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
