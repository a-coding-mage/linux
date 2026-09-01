// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm_adsp.rs  --  Wolfson ADSP support
 *
 * Rust source-level translation of wm_adsp.c.
 *
 * Copyright 2012 Wolfson Microelectronics plc
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type u32 = u32;
type u64 = u64;
type size_t = usize;
type bool_t = bool;
type __be32 = u32;
type __be64 = u64;
type irqreturn_t = c_int;

const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const ENOMEM: c_int = 12;
const EPERM: c_int = 1;
const EFAULT: c_int = 14;
const ENOENT: c_int = 2;
const ENXIO: c_int = 6;
const ENODEV: c_int = 19;
const EIO: c_int = 5;
const ENOTSUPP: c_int = 524;

const GFP_KERNEL: c_uint = 0;
const GFP_DMA: c_uint = 0;
const IRQ_HANDLED: irqreturn_t = 1;

const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 1;
const SNDRV_CTL_ELEM_TYPE_BYTES: c_uint = 4;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_ELEM_ACCESS_READ: c_uint = 1 << 0;
const SNDRV_CTL_ELEM_ACCESS_WRITE: c_uint = 1 << 1;
const SNDRV_CTL_ELEM_ACCESS_VOLATILE: c_uint = 1 << 2;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 1 << 4;
const SNDRV_CTL_ELEM_ACCESS_TLV_WRITE: c_uint = 1 << 5;
const SNDRV_CTL_ELEM_ACCESS_TLV_CALLBACK: c_uint = 1 << 6;
const SNDRV_CTL_ELEM_ID_NAME_MAXLEN: usize = 44;

const SND_COMPRESS_PLAYBACK: c_int = 0;
const SND_COMPRESS_CAPTURE: c_int = 1;
const SND_AUDIOCODEC_BESPOKE: u32 = 0x1000;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_STATE_XRUN: c_int = 4;

const SND_SOC_DAPM_POST_PMU: c_int = 1;
const SND_SOC_DAPM_PRE_PMD: c_int = 2;
const SND_SOC_DAPM_PRE_PMU: c_int = 3;

const CS_DSP_DATA_WORD_SIZE: c_uint = 4;
const CS_DSP_ACKED_CTL_MIN_VALUE: i64 = 0;
const CS_DSP_ACKED_CTL_MAX_VALUE: i64 = 0x7fffffff;
const WMFW_CTL_TYPE_ACKED: c_int = 1;
const WMFW_CTL_TYPE_HOST_BUFFER: c_int = 2;
const WMFW_CTL_FLAG_WRITEABLE: c_uint = 1 << 0;
const WMFW_CTL_FLAG_VOLATILE: c_uint = 1 << 1;
const WMFW_CTL_FLAG_SYS: c_uint = 1 << 2;
const WMFW_ADSP2_XM: c_uint = 1;
const WMFW_ADSP2_YM: c_uint = 2;
const WM_ADSP_COMPR_VOICE_TRIGGER: c_int = 1;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct work_struct { _private: [usize; 0] }
#[repr(C)] pub struct mutex { _private: [usize; 0] }
#[repr(C)] pub struct device { _private: [usize; 0] }
#[repr(C)] pub struct firmware { _private: [usize; 0] }
#[repr(C)] pub struct dentry { _private: [usize; 0] }
#[repr(C)] pub struct snd_soc_component { pub name_prefix: *const c_char, pub debugfs_root: *mut dentry }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [usize; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context, pub shift: c_int }
#[repr(C)] pub struct snd_soc_pcm_runtime { _private: [usize; 0] }
#[repr(C)] pub struct snd_soc_dai { pub name: *const c_char }
#[repr(C)] pub struct snd_kcontrol { pub private_value: c_ulong }
#[repr(C)] pub struct soc_enum { pub shift_l: c_uint }
#[repr(C)] pub struct soc_mixer_control { pub shift: c_uint }
#[repr(C)] pub struct snd_compressed_buffer { pub fragment_size: c_uint, pub fragments: c_uint }
#[repr(C)] pub struct snd_codec_desc { pub max_ch: c_uint, pub sample_rates: [c_uint; 32], pub num_sample_rates: c_uint, pub formats: u64 }
#[repr(C)] pub struct snd_compr_codec { pub id: u32, pub ch_in: c_uint, pub ch_out: c_uint, pub sample_rate: c_uint, pub format: c_uint }
#[repr(C)] pub struct snd_compr_params { pub buffer: snd_compressed_buffer, pub codec: snd_compr_codec }
#[repr(C)] pub struct snd_compr_caps { pub codecs: [u32; 32], pub num_codecs: c_uint, pub direction: c_int, pub min_fragment_size: c_uint, pub max_fragment_size: c_uint, pub min_fragments: c_uint, pub max_fragments: c_uint }
#[repr(C)] pub struct snd_compr_runtime { pub private_data: *mut c_void }
#[repr(C)] pub struct snd_compr_stream { pub private_data: *mut c_void, pub runtime: *mut snd_compr_runtime, pub direction: c_int }
#[repr(C)] pub struct snd_compr_tstamp64 { pub copied_total: u64, pub sampling_rate: c_uint }
#[repr(C)] pub struct snd_ctl_elem_info_integer { pub min: i64, pub max: i64, pub step: i64 }
#[repr(C)] pub union snd_ctl_elem_info_value { pub integer: core::mem::ManuallyDrop<snd_ctl_elem_info_integer> }
#[repr(C)] pub struct snd_ctl_elem_info { pub type_: c_uint, pub count: c_uint, pub value: snd_ctl_elem_info_value }
#[repr(C)] pub struct snd_ctl_elem_value_bytes { pub data: *mut c_char }
#[repr(C)] pub struct snd_ctl_elem_value_integer { pub value: [i64; 128] }
#[repr(C)] pub struct snd_ctl_elem_value_enumerated { pub item: [c_uint; 128] }
#[repr(C)] pub union snd_ctl_elem_value_value { pub bytes: core::mem::ManuallyDrop<snd_ctl_elem_value_bytes>, pub integer: core::mem::ManuallyDrop<snd_ctl_elem_value_integer>, pub enumerated: core::mem::ManuallyDrop<snd_ctl_elem_value_enumerated> }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }
#[repr(C)] pub struct soc_bytes_ext { pub max: c_uint, pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut c_uint, c_uint) -> c_int>, pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *const c_uint, c_uint) -> c_int> }
#[repr(C)] pub union snd_kcontrol_tlv { pub c: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct snd_kcontrol_new { pub name: *const c_char, pub iface: c_uint, pub access: c_uint, pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>, pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, pub tlv: snd_kcontrol_tlv, pub private_value: c_ulong }

#[repr(C)] pub struct cs_dsp_alg_region { pub list: list_head, pub type_: c_uint, pub alg: c_uint, pub base: c_uint }
#[repr(C)] pub struct cs_dsp_coeff_ctl { pub list: list_head, pub dsp: *mut cs_dsp, pub type_: c_int, pub flags: c_uint, pub len: c_uint, pub enabled: bool_t, pub cache: *mut c_void, pub priv_: *mut c_void, pub alg_region: cs_dsp_alg_region, pub subname: *const c_char, pub subname_len: c_int }
#[repr(C)] pub struct cs_dsp_client_ops { pub control_add: Option<unsafe extern "C" fn(*mut cs_dsp_coeff_ctl) -> c_int>, pub control_remove: Option<unsafe extern "C" fn(*mut cs_dsp_coeff_ctl)>, pub pre_run: Option<unsafe extern "C" fn(*mut cs_dsp) -> c_int>, pub post_run: Option<unsafe extern "C" fn(*mut cs_dsp) -> c_int>, pub post_stop: Option<unsafe extern "C" fn(*mut cs_dsp)>, pub watchdog_expired: Option<unsafe extern "C" fn(*mut cs_dsp)> }
#[repr(C)] pub struct cs_dsp { pub dev: *mut device, pub name: *const c_char, pub pwr_lock: mutex, pub booted: bool_t, pub wmfw_ver: c_int, pub fw_id: c_uint, pub ctl_list: list_head, pub client_ops: *const cs_dsp_client_ops, pub no_core_startstop: bool_t }
#[repr(C)] pub struct wm_adsp_fw_file { pub firmware: *const firmware, pub filename: *mut c_char }
#[repr(C)] pub struct wm_adsp_fw_files { pub wmfw: wm_adsp_fw_file, pub coeff: wm_adsp_fw_file }
#[repr(C)] pub struct wm_adsp { pub cs_dsp: cs_dsp, pub compr_list: list_head, pub buffer_list: list_head, pub fw: c_uint, pub fwf_name: *const c_char, pub fwf_suffix: *const c_char, pub system_name: *const c_char, pub part: *const c_char, pub component: *mut snd_soc_component, pub wmfw_optional: bool_t, pub bin_mandatory: bool_t, pub preloaded: c_int, pub toggle_preload: bool_t, pub boot_work: work_struct, pub sys_config_size: size_t, pub pre_run: Option<unsafe extern "C" fn(*mut wm_adsp) -> c_int>, pub control_add: Option<unsafe extern "C" fn(*mut wm_adsp, *mut cs_dsp_coeff_ctl) -> c_int>, pub fatal_error: bool_t }

unsafe extern "C" {
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn kasprintf(gfp: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn kmemdup(src: *const c_void, len: size_t, gfp: c_uint) -> *mut c_void;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kmalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn kvfree(p: *mut c_void);
    fn vmemdup_user(src: *const c_void, len: size_t) -> *mut c_void;
    fn copy_to_user(dst: *mut c_void, src: *const c_void, len: size_t) -> c_int;
    fn firmware_request_nowarn(firmware: *mut *const firmware, filename: *const c_char, dev: *mut device) -> c_int;
    fn release_firmware(firmware: *const firmware);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut wm_adsp;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_add_component_controls(component: *mut snd_soc_component, controls: *mut snd_kcontrol_new, num: c_uint) -> c_int;
    fn snd_soc_bytes_tlv_callback();
    fn snd_soc_dapm_force_enable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_compr_fragment_elapsed(stream: *mut snd_compr_stream);
    fn snd_compr_stop_error(stream: *mut snd_compr_stream, state: c_int);
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_empty(list: *const list_head) -> c_int;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn schedule_work(work: *mut work_struct) -> bool_t;
    fn queue_work(wq: *mut c_void, work: *mut work_struct) -> bool_t;
    fn flush_work(work: *mut work_struct) -> bool_t;
    fn cancel_work_sync(work: *mut work_struct) -> bool_t;
    static mut system_dfl_wq: *mut c_void;
    fn mutex_lock(m: *mut mutex);
    fn mutex_unlock(m: *mut mutex);
    fn usleep_range(min: c_uint, max: c_uint);
    fn isalnum(c: c_int) -> c_int;
    fn tolower(c: c_int) -> c_int;
    fn PTR_ERR(p: *const c_void) -> c_int;
    fn IS_ERR(p: *const c_void) -> bool_t;
    fn be32_to_cpu(v: __be32) -> u32;
    fn sign_extend32(value: u32, index: c_int) -> c_int;
    fn cs_dsp_mem_region_name(type_: c_uint) -> *const c_char;
    fn cs_dsp_coeff_lock_and_write_ctrl(ctl: *mut cs_dsp_coeff_ctl, off: c_uint, buf: *const c_void, len: size_t) -> c_int;
    fn cs_dsp_coeff_lock_and_read_ctrl(ctl: *mut cs_dsp_coeff_ctl, off: c_uint, buf: *mut c_void, len: size_t) -> c_int;
    fn cs_dsp_coeff_write_acked_control(ctl: *mut cs_dsp_coeff_ctl, val: c_uint) -> c_int;
    fn cs_dsp_coeff_write_ctrl(ctl: *mut cs_dsp_coeff_ctl, off: c_uint, buf: *const c_void, len: size_t) -> c_int;
    fn cs_dsp_coeff_read_ctrl(ctl: *mut cs_dsp_coeff_ctl, off: c_uint, buf: *mut c_void, len: size_t) -> c_int;
    fn cs_dsp_get_ctl(dsp: *mut cs_dsp, name: *const c_char, type_: c_int, alg: c_uint) -> *mut cs_dsp_coeff_ctl;
    fn cs_dsp_adsp1_init(dsp: *mut cs_dsp) -> c_int;
    fn cs_dsp_adsp1_power_up(dsp: *mut cs_dsp, wmfw: *const firmware, wmfw_name: *const c_char, coeff: *const firmware, coeff_name: *const c_char, fw_text: *const c_char) -> c_int;
    fn cs_dsp_adsp1_power_down(dsp: *mut cs_dsp);
    fn cs_dsp_set_dspclk(dsp: *mut cs_dsp, freq: c_uint) -> c_int;
    fn cs_dsp_power_up(dsp: *mut cs_dsp, wmfw: *const firmware, wmfw_name: *const c_char, coeff: *const firmware, coeff_name: *const c_char, fw_text: *const c_char) -> c_int;
    fn cs_dsp_power_down(dsp: *mut cs_dsp);
    fn cs_dsp_run(dsp: *mut cs_dsp) -> c_int;
    fn cs_dsp_stop(dsp: *mut cs_dsp);
    fn cs_dsp_hibernate(dsp: *mut cs_dsp, hibernate: bool_t);
    fn cs_dsp_init_debugfs(dsp: *mut cs_dsp, root: *mut dentry);
    fn cs_dsp_cleanup_debugfs(dsp: *mut cs_dsp);
    fn cs_dsp_adsp2_init(dsp: *mut cs_dsp) -> c_int;
    fn cs_dsp_halo_init(dsp: *mut cs_dsp) -> c_int;
    fn cs_dsp_remove(dsp: *mut cs_dsp);
    fn cs_dsp_read_data_word(dsp: *mut cs_dsp, mem_type: c_int, addr: c_uint, data: *mut u32) -> c_int;
    fn cs_dsp_write_data_word(dsp: *mut cs_dsp, mem_type: c_int, addr: c_uint, data: u32) -> c_int;
    fn cs_dsp_find_alg_region(dsp: *mut cs_dsp, mem_type: c_uint, alg: c_uint) -> *mut cs_dsp_alg_region;
    fn cs_dsp_remove_padding(buf: *mut u32, nwords: c_int);
    fn cs_dsp_read_raw_data_block(dsp: *mut cs_dsp, mem_type: c_int, addr: c_uint, nwords: c_int, buf: *mut __be32) -> c_int;
    fn cs_dsp_adsp2_bus_error(dsp: *mut cs_dsp);
    fn cs_dsp_halo_bus_error(dsp: *mut cs_dsp);
    fn cs_dsp_halo_wdt_expire(dsp: *mut cs_dsp);
}

macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const c_char }; }
macro_rules! adsp_err { ($dsp:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {{ let _ = ($dsp, $($arg),*); }}; }
macro_rules! adsp_warn { ($dsp:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {{ let _ = ($dsp, $($arg),*); }}; }
macro_rules! adsp_info { ($dsp:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {{ let _ = ($dsp, $($arg),*); }}; }
macro_rules! adsp_dbg { ($dsp:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {{ let _ = ($dsp, $($arg),*); }}; }
macro_rules! compr_err { ($obj:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {{ let _ = ($obj, $($arg),*); }}; }
macro_rules! compr_dbg { ($obj:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {{ let _ = ($obj, $($arg),*); }}; }

const ADSP_MAX_STD_CTRL_SIZE: c_uint = 512;
const WM_ADSP_FW_MBC_VSS: usize = 0;
const WM_ADSP_FW_HIFI: usize = 1;
const WM_ADSP_FW_TX: usize = 2;
const WM_ADSP_FW_TX_SPK: usize = 3;
const WM_ADSP_FW_RX: usize = 4;
const WM_ADSP_FW_RX_ANC: usize = 5;
const WM_ADSP_FW_CTRL: usize = 6;
const WM_ADSP_FW_ASR: usize = 7;
const WM_ADSP_FW_TRACE: usize = 8;
const WM_ADSP_FW_SPK_PROT: usize = 9;
const WM_ADSP_FW_SPK_CALI: usize = 10;
const WM_ADSP_FW_SPK_DIAG: usize = 11;
const WM_ADSP_FW_MISC: usize = 12;
const WM_ADSP_NUM_FW: usize = 13;

static wm_adsp_fw_text: [*const c_char; WM_ADSP_NUM_FW] = [
    cstr!("MBC/VSS"), cstr!("MasterHiFi"), cstr!("Tx"), cstr!("Tx Speaker"),
    cstr!("Rx"), cstr!("Rx ANC"), cstr!("Voice Ctrl"), cstr!("ASR Assist"),
    cstr!("Dbg Trace"), cstr!("Protection"), cstr!("Calibration"),
    cstr!("Diagnostic"), cstr!("Misc"),
];

#[repr(C, packed)] pub struct wm_adsp_system_config_xm_hdr { pub sys_enable: __be32, pub fw_id: __be32, pub fw_rev: __be32, pub boot_status: __be32, pub watchdog: __be32, pub dma_buffer_size: __be32, pub rdma: [__be32; 6], pub wdma: [__be32; 8], pub build_job_name: [__be32; 3], pub build_job_number: __be32 }
#[repr(C, packed)] pub struct wm_halo_system_config_xm_hdr { pub halo_heartbeat: __be32, pub build_job_name: [__be32; 3], pub build_job_number: __be32 }
#[repr(C, packed)] pub struct wm_adsp_alg_xm_struct { pub magic: __be32, pub smoothing: __be32, pub threshold: __be32, pub host_buf_ptr: __be32, pub start_seq: __be32, pub high_water_mark: __be32, pub low_water_mark: __be32, pub smoothed_power: __be64 }
#[repr(C, packed)] pub struct wm_adsp_host_buf_coeff_v1 { pub host_buf_ptr: __be32, pub versions: __be32, pub name: [__be32; 4] }
#[repr(C, packed)] pub struct wm_adsp_buffer { pub buf1_base: __be32, pub buf1_size: __be32, pub buf2_base: __be32, pub buf1_buf2_size: __be32, pub buf3_base: __be32, pub buf_total_size: __be32, pub high_water_mark: __be32, pub irq_count: __be32, pub irq_ack: __be32, pub next_write_index: __be32, pub next_read_index: __be32, pub error: __be32, pub oldest_block_index: __be32, pub requested_rewind: __be32, pub reserved_space: __be32, pub min_free: __be32, pub blocks_written: [__be32; 2], pub words_written: [__be32; 2] }

#[repr(C)] pub struct wm_adsp_compr_buf { pub list: list_head, pub dsp: *mut wm_adsp, pub compr: *mut wm_adsp_compr, pub regions: *mut wm_adsp_buffer_region, pub host_buf_ptr: u32, pub error: u32, pub irq_count: u32, pub read_index: c_int, pub avail: c_int, pub host_buf_mem_type: c_int, pub name: *mut c_char }
#[repr(C)] pub struct wm_adsp_compr { pub list: list_head, pub dsp: *mut wm_adsp, pub buf: *mut wm_adsp_compr_buf, pub stream: *mut snd_compr_stream, pub size: snd_compressed_buffer, pub raw_buf: *mut u32, pub copied_total: u64, pub sample_rate: c_uint, pub name: *const c_char }

const WM_ADSP_MIN_FRAGMENTS: c_uint = 1;
const WM_ADSP_MAX_FRAGMENTS: c_uint = 256;
const WM_ADSP_MIN_FRAGMENT_SIZE: c_uint = 16 * CS_DSP_DATA_WORD_SIZE;
const WM_ADSP_MAX_FRAGMENT_SIZE: c_uint = 4096 * CS_DSP_DATA_WORD_SIZE;
const WM_ADSP_ALG_XM_STRUCT_MAGIC: u32 = 0x49aec7;
const HOST_BUF_COEFF_SUPPORTED_COMPAT_VER: c_uint = 1;
const HOST_BUF_COEFF_COMPAT_VER_MASK: c_uint = 0xFF00;
const HOST_BUF_COEFF_COMPAT_VER_SHIFT: c_uint = 8;

const fn HOST_BUFFER_FIELD_error() -> c_uint { (offset_of!(wm_adsp_buffer, error) / size_of::<__be32>()) as c_uint }
const fn HOST_BUFFER_FIELD_irq_count() -> c_uint { (offset_of!(wm_adsp_buffer, irq_count) / size_of::<__be32>()) as c_uint }
const fn HOST_BUFFER_FIELD_irq_ack() -> c_uint { (offset_of!(wm_adsp_buffer, irq_ack) / size_of::<__be32>()) as c_uint }
const fn HOST_BUFFER_FIELD_next_read_index() -> c_uint { (offset_of!(wm_adsp_buffer, next_read_index) / size_of::<__be32>()) as c_uint }
const fn HOST_BUFFER_FIELD_next_write_index() -> c_uint { (offset_of!(wm_adsp_buffer, next_write_index) / size_of::<__be32>()) as c_uint }
const fn HOST_BUFFER_FIELD_high_water_mark() -> c_uint { (offset_of!(wm_adsp_buffer, high_water_mark) / size_of::<__be32>()) as c_uint }
const fn ALG_XM_FIELD_magic() -> c_uint { (offset_of!(wm_adsp_alg_xm_struct, magic) / size_of::<__be32>()) as c_uint }
const fn ALG_XM_FIELD_host_buf_ptr() -> c_uint { (offset_of!(wm_adsp_alg_xm_struct, host_buf_ptr) / size_of::<__be32>()) as c_uint }

#[repr(C)] pub struct wm_adsp_buffer_region { pub offset: c_uint, pub cumulative_size: c_uint, pub mem_type: c_uint, pub base_addr: c_uint }
#[repr(C)] pub struct wm_adsp_buffer_region_def { pub mem_type: c_uint, pub base_offset: c_uint, pub size_offset: c_uint }

static default_regions: [wm_adsp_buffer_region_def; 3] = [
    wm_adsp_buffer_region_def { mem_type: WMFW_ADSP2_XM, base_offset: (offset_of!(wm_adsp_buffer, buf1_base) / size_of::<__be32>()) as c_uint, size_offset: (offset_of!(wm_adsp_buffer, buf1_size) / size_of::<__be32>()) as c_uint },
    wm_adsp_buffer_region_def { mem_type: WMFW_ADSP2_XM, base_offset: (offset_of!(wm_adsp_buffer, buf2_base) / size_of::<__be32>()) as c_uint, size_offset: (offset_of!(wm_adsp_buffer, buf1_buf2_size) / size_of::<__be32>()) as c_uint },
    wm_adsp_buffer_region_def { mem_type: WMFW_ADSP2_YM, base_offset: (offset_of!(wm_adsp_buffer, buf3_base) / size_of::<__be32>()) as c_uint, size_offset: (offset_of!(wm_adsp_buffer, buf_total_size) / size_of::<__be32>()) as c_uint },
];

#[repr(C)] pub struct wm_adsp_fw_caps { pub id: u32, pub desc: snd_codec_desc, pub num_regions: c_int, pub region_defs: *const wm_adsp_buffer_region_def }

static ctrl_caps: [wm_adsp_fw_caps; 1] = [wm_adsp_fw_caps { id: SND_AUDIOCODEC_BESPOKE, desc: snd_codec_desc { max_ch: 8, sample_rates: { let mut a = [0; 32]; a[0] = 16000; a }, num_sample_rates: 1, formats: SNDRV_PCM_FMTBIT_S16_LE }, num_regions: 3, region_defs: default_regions.as_ptr() }];
static trace_caps: [wm_adsp_fw_caps; 1] = [wm_adsp_fw_caps { id: SND_AUDIOCODEC_BESPOKE, desc: snd_codec_desc { max_ch: 8, sample_rates: { let mut a = [0; 32]; a[0]=4000; a[1]=8000; a[2]=11025; a[3]=12000; a[4]=16000; a[5]=22050; a[6]=24000; a[7]=32000; a[8]=44100; a[9]=48000; a[10]=64000; a[11]=88200; a[12]=96000; a[13]=176400; a[14]=192000; a }, num_sample_rates: 15, formats: SNDRV_PCM_FMTBIT_S16_LE }, num_regions: 3, region_defs: default_regions.as_ptr() }];

#[repr(C)] pub struct wm_adsp_fw_entry { pub file: *const c_char, pub compr_direction: c_int, pub num_caps: c_int, pub caps: *const wm_adsp_fw_caps, pub voice_trigger: bool_t }
static wm_adsp_fw: [wm_adsp_fw_entry; WM_ADSP_NUM_FW] = [
    wm_adsp_fw_entry { file: cstr!("mbc-vss"), compr_direction: 0, num_caps: 0, caps: ptr::null(), voice_trigger: false },
    wm_adsp_fw_entry { file: cstr!("hifi"), compr_direction: 0, num_caps: 0, caps: ptr::null(), voice_trigger: false },
    wm_adsp_fw_entry { file: cstr!("tx"), compr_direction: 0, num_caps: 0, caps: ptr::null(), voice_trigger: false },
    wm_adsp_fw_entry { file: cstr!("tx-spk"), compr_direction: 0, num_caps: 0, caps: ptr::null(), voice_trigger: false },
    wm_adsp_fw_entry { file: cstr!("rx"), compr_direction: 0, num_caps: 0, caps: ptr::null(), voice_trigger: false },
    wm_adsp_fw_entry { file: cstr!("rx-anc"), compr_direction: 0, num_caps: 0, caps: ptr::null(), voice_trigger: false },
    wm_adsp_fw_entry { file: cstr!("ctrl"), compr_direction: SND_COMPRESS_CAPTURE, num_caps: 1, caps: ctrl_caps.as_ptr(), voice_trigger: true },
    wm_adsp_fw_entry { file: cstr!("asr"), compr_direction: 0, num_caps: 0, caps: ptr::null(), voice_trigger: false },
    wm_adsp_fw_entry { file: cstr!("trace"), compr_direction: SND_COMPRESS_CAPTURE, num_caps: 1, caps: trace_caps.as_ptr(), voice_trigger: false },
    wm_adsp_fw_entry { file: cstr!("spk-prot"), compr_direction: SND_COMPRESS_CAPTURE, num_caps: 1, caps: trace_caps.as_ptr(), voice_trigger: false },
    wm_adsp_fw_entry { file: cstr!("spk-cali"), compr_direction: 0, num_caps: 0, caps: ptr::null(), voice_trigger: false },
    wm_adsp_fw_entry { file: cstr!("spk-diag"), compr_direction: 0, num_caps: 0, caps: ptr::null(), voice_trigger: false },
    wm_adsp_fw_entry { file: cstr!("misc"), compr_direction: 0, num_caps: 0, caps: ptr::null(), voice_trigger: false },
];

#[repr(C)] pub struct wm_coeff_ctl { pub name: *const c_char, pub cs_ctl: *mut cs_dsp_coeff_ctl, pub bytes_ext: soc_bytes_ext, pub work: work_struct }

unsafe fn container_of<T, U>(ptr: *mut U, offset: usize) -> *mut T { (ptr as *mut u8).sub(offset) as *mut T }
unsafe fn list_first_entry<T>(head: *mut list_head, offset: usize) -> *mut T { ((*head).next as *mut u8).sub(offset) as *mut T }
unsafe fn list_next_entry<T>(pos: *mut T, offset: usize) -> *mut T { ((*(pos as *mut u8).add(offset).cast::<list_head>()).next as *mut u8).sub(offset) as *mut T }

#[no_mangle] pub unsafe extern "C" fn wm_adsp_get_fwf_name_by_index(index: c_int) -> *const c_char {
    if index < WM_ADSP_NUM_FW as c_int { wm_adsp_fw[index as usize].file } else { ptr::null() }
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp_fw_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let dsp = snd_soc_component_get_drvdata(component);
    (*ucontrol).value.enumerated.item[0] = (*dsp.add((*e).shift_l as usize)).fw;
    0
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp_fw_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let dsp = snd_soc_component_get_drvdata(component).add((*e).shift_l as usize);
    let item = (*ucontrol).value.enumerated.item[0];
    if item == (*dsp).fw { return 0; }
    if item >= WM_ADSP_NUM_FW as c_uint { return -EINVAL; }
    mutex_lock(&mut (*dsp).cs_dsp.pwr_lock);
    let ret = if (*dsp).cs_dsp.booted || list_empty(&(*dsp).compr_list) == 0 { -EBUSY } else { (*dsp).fw = item; 1 };
    mutex_unlock(&mut (*dsp).cs_dsp.pwr_lock);
    ret
}

#[no_mangle] pub static wm_adsp_fw_enum: [soc_enum; 7] = [
    soc_enum { shift_l: 0 }, soc_enum { shift_l: 1 }, soc_enum { shift_l: 2 },
    soc_enum { shift_l: 3 }, soc_enum { shift_l: 4 }, soc_enum { shift_l: 5 },
    soc_enum { shift_l: 6 },
];

unsafe fn bytes_ext_to_ctl(ext: *mut soc_bytes_ext) -> *mut wm_coeff_ctl {
    container_of(ext, offset_of!(wm_coeff_ctl, bytes_ext))
}

unsafe extern "C" fn wm_coeff_info(kctl: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let ctl = bytes_ext_to_ctl((*kctl).private_value as *mut soc_bytes_ext);
    let cs_ctl = (*ctl).cs_ctl;
    match (*cs_ctl).type_ {
        WMFW_CTL_TYPE_ACKED => {
            (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
            (*uinfo).value.integer.min = CS_DSP_ACKED_CTL_MIN_VALUE;
            (*uinfo).value.integer.max = CS_DSP_ACKED_CTL_MAX_VALUE;
            (*uinfo).value.integer.step = 1;
            (*uinfo).count = 1;
        }
        _ => {
            (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BYTES;
            (*uinfo).count = (*cs_ctl).len;
        }
    }
    0
}

unsafe extern "C" fn wm_coeff_put(kctl: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ctl = bytes_ext_to_ctl((*kctl).private_value as *mut soc_bytes_ext);
    let cs_ctl = (*ctl).cs_ctl;
    cs_dsp_coeff_lock_and_write_ctrl(cs_ctl, 0, (*ucontrol).value.bytes.data as *const c_void, (*cs_ctl).len as size_t)
}

unsafe extern "C" fn wm_coeff_tlv_put(kctl: *mut snd_kcontrol, bytes: *const c_uint, size: c_uint) -> c_int {
    let ctl = bytes_ext_to_ctl((*kctl).private_value as *mut soc_bytes_ext);
    let cs_ctl = (*ctl).cs_ctl;
    let scratch = vmemdup_user(bytes as *const c_void, size as size_t);
    if IS_ERR(scratch) { return PTR_ERR(scratch); }
    let ret = cs_dsp_coeff_lock_and_write_ctrl(cs_ctl, 0, scratch, size as size_t);
    kvfree(scratch);
    ret
}

unsafe extern "C" fn wm_coeff_put_acked(kctl: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ctl = bytes_ext_to_ctl((*kctl).private_value as *mut soc_bytes_ext);
    let cs_ctl = (*ctl).cs_ctl;
    let val = (*ucontrol).value.integer.value[0] as c_uint;
    if val == 0 { return 0; }
    mutex_lock(&mut (*(*cs_ctl).dsp).pwr_lock);
    let ret = if (*cs_ctl).enabled { cs_dsp_coeff_write_acked_control(cs_ctl, val) } else { -EPERM };
    mutex_unlock(&mut (*(*cs_ctl).dsp).pwr_lock);
    if ret < 0 { ret } else { 1 }
}

unsafe extern "C" fn wm_coeff_get(kctl: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ctl = bytes_ext_to_ctl((*kctl).private_value as *mut soc_bytes_ext);
    let cs_ctl = (*ctl).cs_ctl;
    cs_dsp_coeff_lock_and_read_ctrl(cs_ctl, 0, (*ucontrol).value.bytes.data as *mut c_void, (*cs_ctl).len as size_t)
}

unsafe extern "C" fn wm_coeff_tlv_get(kctl: *mut snd_kcontrol, bytes: *mut c_uint, size: c_uint) -> c_int {
    let ctl = bytes_ext_to_ctl((*kctl).private_value as *mut soc_bytes_ext);
    let cs_ctl = (*ctl).cs_ctl;
    mutex_lock(&mut (*(*cs_ctl).dsp).pwr_lock);
    let mut ret = cs_dsp_coeff_read_ctrl(cs_ctl, 0, (*cs_ctl).cache, size as size_t);
    if ret == 0 && copy_to_user(bytes as *mut c_void, (*cs_ctl).cache, size as size_t) != 0 { ret = -EFAULT; }
    mutex_unlock(&mut (*(*cs_ctl).dsp).pwr_lock);
    ret
}

unsafe extern "C" fn wm_coeff_get_acked(_kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    /*
     * Although it's not useful to read an acked control, return 0 meaning
     * "no event" so valid event numbers will always be a change.
     */
    (*ucontrol).value.integer.value[0] = 0;
    0
}

fn wmfw_convert_flags(in_: c_uint, len: c_uint) -> c_uint {
    let (rd, wr, vol, mut out) = if len > ADSP_MAX_STD_CTRL_SIZE {
        (SNDRV_CTL_ELEM_ACCESS_TLV_READ, SNDRV_CTL_ELEM_ACCESS_TLV_WRITE, SNDRV_CTL_ELEM_ACCESS_VOLATILE, SNDRV_CTL_ELEM_ACCESS_TLV_CALLBACK)
    } else {
        (SNDRV_CTL_ELEM_ACCESS_READ, SNDRV_CTL_ELEM_ACCESS_WRITE, SNDRV_CTL_ELEM_ACCESS_VOLATILE, 0)
    };
    if in_ != 0 {
        out |= rd;
        if in_ & WMFW_CTL_FLAG_WRITEABLE != 0 { out |= wr; }
        if in_ & WMFW_CTL_FLAG_VOLATILE != 0 { out |= vol; }
    } else {
        out |= rd | wr | vol;
    }
    out
}

unsafe extern "C" fn wm_adsp_ctl_work(work: *mut work_struct) {
    let ctl: *mut wm_coeff_ctl = container_of(work, offset_of!(wm_coeff_ctl, work));
    let cs_ctl = (*ctl).cs_ctl;
    let dsp: *mut wm_adsp = container_of((*cs_ctl).dsp, offset_of!(wm_adsp, cs_dsp));
    let kcontrol = kzalloc(size_of::<snd_kcontrol_new>(), GFP_KERNEL) as *mut snd_kcontrol_new;
    if kcontrol.is_null() { return; }
    (*kcontrol).name = (*ctl).name;
    (*kcontrol).info = Some(wm_coeff_info);
    (*kcontrol).iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    (*kcontrol).tlv.c = Some(snd_soc_bytes_tlv_callback);
    (*kcontrol).private_value = &mut (*ctl).bytes_ext as *mut _ as c_ulong;
    (*kcontrol).access = wmfw_convert_flags((*cs_ctl).flags, (*cs_ctl).len);
    if (*cs_ctl).type_ == WMFW_CTL_TYPE_ACKED {
        (*kcontrol).get = Some(wm_coeff_get_acked);
        (*kcontrol).put = Some(wm_coeff_put_acked);
    } else if (*kcontrol).access & SNDRV_CTL_ELEM_ACCESS_TLV_CALLBACK != 0 {
        (*ctl).bytes_ext.max = (*cs_ctl).len;
        (*ctl).bytes_ext.get = Some(wm_coeff_tlv_get);
        (*ctl).bytes_ext.put = Some(wm_coeff_tlv_put);
    } else {
        (*kcontrol).get = Some(wm_coeff_get);
        (*kcontrol).put = Some(wm_coeff_put);
    }
    snd_soc_add_component_controls((*dsp).component, kcontrol, 1);
    kfree(kcontrol as *mut c_void);
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp_control_add(cs_ctl: *mut cs_dsp_coeff_ctl) -> c_int {
    let dsp: *mut wm_adsp = container_of((*cs_ctl).dsp, offset_of!(wm_adsp, cs_dsp));
    let cs_dsp = &mut (*dsp).cs_dsp as *mut cs_dsp;
    let mut name = [0 as c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN];
    if (*cs_ctl).flags & WMFW_CTL_FLAG_SYS != 0 { return 0; }
    let region_name = cs_dsp_mem_region_name((*cs_ctl).alg_region.type_);
    if region_name.is_null() { adsp_err!(dsp, "Unknown region type: %d\n", (*cs_ctl).alg_region.type_); return -EINVAL; }
    let mut ret = match (*cs_dsp).wmfw_ver {
        0 | 1 => scnprintf(name.as_mut_ptr(), name.len(), cstr!("%s %s %x"), (*cs_dsp).name, region_name, (*cs_ctl).alg_region.alg),
        2 => scnprintf(name.as_mut_ptr(), name.len(), cstr!("%s%c %.12s %x"), (*cs_dsp).name, *region_name, wm_adsp_fw_text[(*dsp).fw as usize], (*cs_ctl).alg_region.alg),
        _ => scnprintf(name.as_mut_ptr(), name.len(), cstr!("%s %.12s %x"), (*cs_dsp).name, wm_adsp_fw_text[(*dsp).fw as usize], (*cs_ctl).alg_region.alg),
    };
    if !(*cs_ctl).subname.is_null() {
        let mut avail = SNDRV_CTL_ELEM_ID_NAME_MAXLEN as c_int - ret - 2;
        let mut skip = 0;
        if !(*(*dsp).component).name_prefix.is_null() { avail -= strlen((*(*dsp).component).name_prefix) as c_int + 1; }
        if (*cs_ctl).subname_len > avail { skip = (*cs_ctl).subname_len - avail; }
        snprintf(name.as_mut_ptr().add(ret as usize), SNDRV_CTL_ELEM_ID_NAME_MAXLEN - ret as usize, cstr!(" %.*s"), (*cs_ctl).subname_len - skip, (*cs_ctl).subname.add(skip as usize));
    }
    let ctl = kzalloc(size_of::<wm_coeff_ctl>(), GFP_KERNEL) as *mut wm_coeff_ctl;
    if ctl.is_null() { return -ENOMEM; }
    (*ctl).cs_ctl = cs_ctl;
    (*ctl).name = kmemdup(name.as_ptr() as *const c_void, strlen(name.as_ptr()) + 1, GFP_KERNEL) as *const c_char;
    if (*ctl).name.is_null() { ret = -ENOMEM; kfree(ctl as *mut c_void); return ret; }
    (*cs_ctl).priv_ = ctl as *mut c_void;
    INIT_WORK(&mut (*ctl).work, wm_adsp_ctl_work);
    schedule_work(&mut (*ctl).work);
    0
}

unsafe extern "C" fn wm_adsp_control_add_cb(cs_ctl: *mut cs_dsp_coeff_ctl) -> c_int {
    let dsp: *mut wm_adsp = container_of((*cs_ctl).dsp, offset_of!(wm_adsp, cs_dsp));
    if let Some(cb) = (*dsp).control_add { cb(dsp, cs_ctl) } else { wm_adsp_control_add(cs_ctl) }
}

unsafe extern "C" fn wm_adsp_control_remove(cs_ctl: *mut cs_dsp_coeff_ctl) {
    let ctl = (*cs_ctl).priv_ as *mut wm_coeff_ctl;
    if ctl.is_null() { return; }
    cancel_work_sync(&mut (*ctl).work);
    kfree((*ctl).name as *mut c_void);
    kfree(ctl as *mut c_void);
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp_write_ctl(dsp: *mut wm_adsp, name: *const c_char, type_: c_int, alg: c_uint, buf: *mut c_void, len: size_t) -> c_int {
    mutex_lock(&mut (*dsp).cs_dsp.pwr_lock);
    let cs_ctl = cs_dsp_get_ctl(&mut (*dsp).cs_dsp, name, type_, alg);
    let ret = cs_dsp_coeff_write_ctrl(cs_ctl, 0, buf, len);
    mutex_unlock(&mut (*dsp).cs_dsp.pwr_lock);
    if ret < 0 { ret } else { 0 }
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp_read_ctl(dsp: *mut wm_adsp, name: *const c_char, type_: c_int, alg: c_uint, buf: *mut c_void, len: size_t) -> c_int {
    mutex_lock(&mut (*dsp).cs_dsp.pwr_lock);
    let ret = cs_dsp_coeff_read_ctrl(cs_dsp_get_ctl(&mut (*dsp).cs_dsp, name, type_, alg), 0, buf, len);
    mutex_unlock(&mut (*dsp).cs_dsp.pwr_lock);
    ret
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp_release_firmware_files(fw: *mut wm_adsp_fw_files) {
    release_firmware((*fw).wmfw.firmware);
    kfree((*fw).wmfw.filename as *mut c_void);
    release_firmware((*fw).coeff.firmware);
    kfree((*fw).coeff.filename as *mut c_void);
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp_firmware_request(firmware: *mut *const firmware, filename: *const c_char, dev: *mut device) -> c_int {
    firmware_request_nowarn(firmware, filename, dev)
}

unsafe fn wm_adsp_request_firmware_file(dsp: *mut wm_adsp, fw: *mut wm_adsp_fw_file, dir: *const c_char, system_name: *const c_char, asoc_component_prefix: *const c_char, filetype: *const c_char) -> c_int {
    let fwf = if !(*dsp).fwf_name.is_null() { (*dsp).fwf_name } else { (*dsp).cs_dsp.name };
    (*fw).filename = if !system_name.is_null() && !asoc_component_prefix.is_null() {
        kasprintf(GFP_KERNEL, cstr!("%s%s-%s-%s-%s-%s.%s"), dir, (*dsp).part, fwf, wm_adsp_fw[(*dsp).fw as usize].file, system_name, asoc_component_prefix, filetype)
    } else if !system_name.is_null() {
        kasprintf(GFP_KERNEL, cstr!("%s%s-%s-%s-%s.%s"), dir, (*dsp).part, fwf, wm_adsp_fw[(*dsp).fw as usize].file, system_name, filetype)
    } else {
        kasprintf(GFP_KERNEL, cstr!("%s%s-%s-%s.%s"), dir, (*dsp).part, fwf, wm_adsp_fw[(*dsp).fw as usize].file, filetype)
    };
    if (*fw).filename.is_null() { return -ENOMEM; }
    let mut s = (*fw).filename.add(strlen(dir));
    while *s != 0 {
        let c = *s;
        if isalnum(c as c_int) != 0 { *s = tolower(c as c_int) as c_char; } else if c != b'.' as c_char { *s = b'-' as c_char; }
        s = s.add(1);
    }
    let ret = wm_adsp_firmware_request(&mut (*fw).firmware, (*fw).filename, (*dsp).cs_dsp.dev);
    if ret < 0 {
        adsp_dbg!(dsp, "Failed to request '%s': %d\n", (*fw).filename, ret);
        kfree((*fw).filename as *mut c_void);
        (*fw).filename = ptr::null_mut();
        if ret != -ENOENT { return ret; }
    } else {
        adsp_dbg!(dsp, "Found '%s'\n", (*fw).filename);
    }
    0
}

static cirrus_dir: *const c_char = cstr!("cirrus/");

#[no_mangle] pub unsafe extern "C" fn wm_adsp_request_firmware_files(dsp: *mut wm_adsp, fw: *mut wm_adsp_fw_files) -> c_int {
    let system_name = (*dsp).system_name;
    let mut suffix = (*(*dsp).component).name_prefix;
    let mut require_bin_suffix = false;
    let mut ret = 0;
    if !(*dsp).fwf_suffix.is_null() { suffix = (*dsp).fwf_suffix; }
    if !system_name.is_null() {
        ret = wm_adsp_request_firmware_file(dsp, &mut (*fw).wmfw, cirrus_dir, system_name, suffix, cstr!("wmfw"));
        if ret < 0 { goto_err(fw); return ret; }
        if !suffix.is_null() {
            if !(*fw).wmfw.firmware.is_null() { require_bin_suffix = true; }
            else {
                ret = wm_adsp_request_firmware_file(dsp, &mut (*fw).wmfw, cirrus_dir, system_name, ptr::null(), cstr!("wmfw"));
                if ret < 0 { goto_err(fw); return ret; }
            }
        }
        if !(*fw).wmfw.firmware.is_null() || (*dsp).wmfw_optional {
            ret = wm_adsp_request_firmware_file(dsp, &mut (*fw).coeff, cirrus_dir, system_name, suffix, cstr!("bin"));
            if ret < 0 { goto_err(fw); return ret; }
            if !suffix.is_null() && (*fw).coeff.firmware.is_null() && !require_bin_suffix {
                ret = wm_adsp_request_firmware_file(dsp, &mut (*fw).coeff, cirrus_dir, system_name, ptr::null(), cstr!("bin"));
                if ret < 0 { goto_err(fw); return ret; }
            }
        }
        if !(*fw).wmfw.firmware.is_null() || ((*dsp).wmfw_optional && !(*fw).coeff.firmware.is_null()) { return 0; }
    }
    ret = wm_adsp_request_firmware_file(dsp, &mut (*fw).wmfw, cstr!(""), ptr::null(), ptr::null(), cstr!("wmfw"));
    if ret < 0 { goto_err(fw); return ret; }
    if !(*fw).wmfw.firmware.is_null() {
        ret = wm_adsp_request_firmware_file(dsp, &mut (*fw).coeff, cstr!(""), ptr::null(), ptr::null(), cstr!("bin"));
        if ret < 0 { goto_err(fw); return ret; }
        return 0;
    }
    ret = wm_adsp_request_firmware_file(dsp, &mut (*fw).wmfw, cirrus_dir, ptr::null(), ptr::null(), cstr!("wmfw"));
    if ret < 0 { goto_err(fw); return ret; }
    if !(*fw).wmfw.firmware.is_null() || (*dsp).wmfw_optional {
        ret = wm_adsp_request_firmware_file(dsp, &mut (*fw).coeff, cirrus_dir, ptr::null(), ptr::null(), cstr!("bin"));
        if ret < 0 { goto_err(fw); return ret; }
        return 0;
    }
    adsp_err!(dsp, "Failed to request firmware <%s>%s-%s-%s<-%s<%s>>.wmfw\n", cirrus_dir, (*dsp).part, if !(*dsp).fwf_name.is_null() { (*dsp).fwf_name } else { (*dsp).cs_dsp.name }, wm_adsp_fw[(*dsp).fw as usize].file, system_name, suffix);
    ret = -ENOENT;
    wm_adsp_release_firmware_files(fw);
    ret
}
unsafe fn goto_err(fw: *mut wm_adsp_fw_files) { wm_adsp_release_firmware_files(fw); }

unsafe fn wm_adsp_common_init(dsp: *mut wm_adsp) -> c_int {
    INIT_LIST_HEAD(&mut (*dsp).compr_list);
    INIT_LIST_HEAD(&mut (*dsp).buffer_list);
    0
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp1_init(dsp: *mut wm_adsp) -> c_int {
    (*dsp).cs_dsp.client_ops = &wm_adsp1_client_ops;
    let ret = cs_dsp_adsp1_init(&mut (*dsp).cs_dsp);
    if ret != 0 { return ret; }
    wm_adsp_common_init(dsp)
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp1_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let dsps = snd_soc_component_get_drvdata(component);
    let dsp = dsps.add((*w).shift as usize);
    let mut fw: wm_adsp_fw_files = core::mem::zeroed();
    let mut ret = 0;
    (*dsp).component = component;
    match event {
        SND_SOC_DAPM_POST_PMU => {
            ret = wm_adsp_request_firmware_files(dsp, &mut fw);
            if ret == 0 {
                ret = cs_dsp_adsp1_power_up(&mut (*dsp).cs_dsp, fw.wmfw.firmware, fw.wmfw.filename, fw.coeff.firmware, fw.coeff.filename, wm_adsp_fw_text[(*dsp).fw as usize]);
                wm_adsp_release_firmware_files(&mut fw);
            }
        }
        SND_SOC_DAPM_PRE_PMD => cs_dsp_adsp1_power_down(&mut (*dsp).cs_dsp),
        _ => {}
    }
    ret
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp2_set_dspclk(w: *mut snd_soc_dapm_widget, freq: c_uint) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let dsp = snd_soc_component_get_drvdata(component).add((*w).shift as usize);
    cs_dsp_set_dspclk(&mut (*dsp).cs_dsp, freq)
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp2_preloader_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let dsps = snd_soc_component_get_drvdata(component);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let dsp = dsps.add(((*mc).shift - 1) as usize);
    (*ucontrol).value.integer.value[0] = (*dsp).preloaded as i64;
    0
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp2_preloader_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let dsps = snd_soc_component_get_drvdata(component);
    let dapm = snd_soc_component_to_dapm(component);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let dsp = dsps.add(((*mc).shift - 1) as usize);
    let mut preload = [0 as c_char; 32];
    if (*dsp).preloaded as i64 == (*ucontrol).value.integer.value[0] { return 0; }
    snprintf(preload.as_mut_ptr(), preload.len(), cstr!("%s Preload"), (*dsp).cs_dsp.name);
    if (*ucontrol).value.integer.value[0] != 0 || (*dsp).toggle_preload { snd_soc_dapm_force_enable_pin(dapm, preload.as_ptr()); } else { snd_soc_dapm_disable_pin(dapm, preload.as_ptr()); }
    snd_soc_dapm_sync(dapm);
    flush_work(&mut (*dsp).boot_work);
    (*dsp).preloaded = (*ucontrol).value.integer.value[0] as c_int;
    if (*dsp).toggle_preload { snd_soc_dapm_disable_pin(dapm, preload.as_ptr()); snd_soc_dapm_sync(dapm); }
    1
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp_power_up(dsp: *mut wm_adsp, load_firmware: bool_t) -> c_int {
    let mut fw: wm_adsp_fw_files = core::mem::zeroed();
    let mut ret = 0;
    if load_firmware {
        ret = wm_adsp_request_firmware_files(dsp, &mut fw);
        if ret != 0 { return ret; }
    }
    if (*dsp).bin_mandatory && fw.coeff.firmware.is_null() { ret = -ENOENT; }
    else { ret = cs_dsp_power_up(&mut (*dsp).cs_dsp, fw.wmfw.firmware, fw.wmfw.filename, fw.coeff.firmware, fw.coeff.filename, wm_adsp_fw_text[(*dsp).fw as usize]); }
    wm_adsp_release_firmware_files(&mut fw);
    ret
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp_power_down(dsp: *mut wm_adsp) { cs_dsp_power_down(&mut (*dsp).cs_dsp); }

unsafe extern "C" fn wm_adsp_boot_work(work: *mut work_struct) {
    let dsp: *mut wm_adsp = container_of(work, offset_of!(wm_adsp, boot_work));
    wm_adsp_power_up(dsp, true);
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp_early_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let dsp = snd_soc_component_get_drvdata(component).add((*w).shift as usize);
    match event {
        SND_SOC_DAPM_PRE_PMU => { queue_work(system_dfl_wq, &mut (*dsp).boot_work); }
        SND_SOC_DAPM_PRE_PMD => wm_adsp_power_down(dsp),
        _ => {}
    }
    0
}

unsafe extern "C" fn wm_adsp_pre_run(cs_dsp: *mut cs_dsp) -> c_int {
    let dsp: *mut wm_adsp = container_of(cs_dsp, offset_of!(wm_adsp, cs_dsp));
    if let Some(pre_run) = (*dsp).pre_run { pre_run(dsp) } else { 0 }
}

unsafe extern "C" fn wm_adsp_event_post_run(cs_dsp: *mut cs_dsp) -> c_int {
    let dsp: *mut wm_adsp = container_of(cs_dsp, offset_of!(wm_adsp, cs_dsp));
    if wm_adsp_fw[(*dsp).fw as usize].num_caps != 0 { wm_adsp_buffer_init(dsp) } else { 0 }
}

unsafe extern "C" fn wm_adsp_event_post_stop(cs_dsp: *mut cs_dsp) {
    let dsp: *mut wm_adsp = container_of(cs_dsp, offset_of!(wm_adsp, cs_dsp));
    if wm_adsp_fw[(*dsp).fw as usize].num_caps != 0 { wm_adsp_buffer_free(dsp); }
    (*dsp).fatal_error = false;
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp_run(dsp: *mut wm_adsp) -> c_int { flush_work(&mut (*dsp).boot_work); cs_dsp_run(&mut (*dsp).cs_dsp) }
#[no_mangle] pub unsafe extern "C" fn wm_adsp_stop(dsp: *mut wm_adsp) { cs_dsp_stop(&mut (*dsp).cs_dsp); }
#[no_mangle] pub unsafe extern "C" fn wm_adsp_hibernate(dsp: *mut wm_adsp, hibernate: bool_t) { cs_dsp_hibernate(&mut (*dsp).cs_dsp, hibernate); }

#[no_mangle] pub unsafe extern "C" fn wm_adsp_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let dsp = snd_soc_component_get_drvdata(component).add((*w).shift as usize);
    match event { SND_SOC_DAPM_POST_PMU => wm_adsp_run(dsp), SND_SOC_DAPM_PRE_PMD => { wm_adsp_stop(dsp); 0 }, _ => 0 }
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp2_component_probe(dsp: *mut wm_adsp, component: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let mut preload = [0 as c_char; 32];
    if !(*dsp).cs_dsp.no_core_startstop {
        snprintf(preload.as_mut_ptr(), preload.len(), cstr!("%s Preload"), (*dsp).cs_dsp.name);
        snd_soc_dapm_disable_pin(dapm, preload.as_ptr());
    }
    cs_dsp_init_debugfs(&mut (*dsp).cs_dsp, (*component).debugfs_root);
    (*dsp).component = component;
    0
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp2_component_remove(dsp: *mut wm_adsp, _component: *mut snd_soc_component) -> c_int {
    if dsp.is_null() || (*dsp).component.is_null() { return 0; }
    cs_dsp_cleanup_debugfs(&mut (*dsp).cs_dsp);
    (*dsp).component = ptr::null_mut();
    0
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp2_init(dsp: *mut wm_adsp) -> c_int {
    INIT_WORK(&mut (*dsp).boot_work, wm_adsp_boot_work);
    (*dsp).sys_config_size = size_of::<wm_adsp_system_config_xm_hdr>();
    (*dsp).cs_dsp.client_ops = &wm_adsp2_client_ops;
    let ret = cs_dsp_adsp2_init(&mut (*dsp).cs_dsp);
    if ret != 0 { return ret; }
    wm_adsp_common_init(dsp)
}

#[no_mangle] pub unsafe extern "C" fn wm_halo_init(dsp: *mut wm_adsp) -> c_int {
    INIT_WORK(&mut (*dsp).boot_work, wm_adsp_boot_work);
    (*dsp).sys_config_size = size_of::<wm_halo_system_config_xm_hdr>();
    (*dsp).cs_dsp.client_ops = &wm_adsp2_client_ops;
    let ret = cs_dsp_halo_init(&mut (*dsp).cs_dsp);
    if ret != 0 { return ret; }
    wm_adsp_common_init(dsp)
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp2_remove(dsp: *mut wm_adsp) { cs_dsp_remove(&mut (*dsp).cs_dsp); }

unsafe fn wm_adsp_compr_attached(compr: *mut wm_adsp_compr) -> c_int { (!(*compr).buf.is_null()) as c_int }

unsafe fn wm_adsp_compr_attach(compr: *mut wm_adsp_compr) -> c_int {
    if (*(*compr).dsp).fatal_error { return -EINVAL; }
    let head = &mut (*(*compr).dsp).buffer_list as *mut list_head;
    let mut p = (*head).next;
    while p != head {
        let tmp: *mut wm_adsp_compr_buf = container_of(p, offset_of!(wm_adsp_compr_buf, list));
        if (*tmp).name.is_null() || strcmp((*compr).name, (*tmp).name) == 0 {
            (*compr).buf = tmp;
            (*tmp).compr = compr;
            return 0;
        }
        p = (*p).next;
    }
    -EINVAL
}

unsafe fn wm_adsp_compr_detach(compr: *mut wm_adsp_compr) {
    if compr.is_null() { return; }
    if !(*compr).stream.is_null() { snd_compr_fragment_elapsed((*compr).stream); }
    if wm_adsp_compr_attached(compr) != 0 {
        (*(*compr).buf).compr = ptr::null_mut();
        (*compr).buf = ptr::null_mut();
    }
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp_compr_open(dsp: *mut wm_adsp, stream: *mut snd_compr_stream) -> c_int {
    let rtd = (*stream).private_data as *mut snd_soc_pcm_runtime;
    mutex_lock(&mut (*dsp).cs_dsp.pwr_lock);
    if wm_adsp_fw[(*dsp).fw as usize].num_caps == 0 { mutex_unlock(&mut (*dsp).cs_dsp.pwr_lock); return -ENXIO; }
    if wm_adsp_fw[(*dsp).fw as usize].compr_direction != (*stream).direction { mutex_unlock(&mut (*dsp).cs_dsp.pwr_lock); return -EINVAL; }
    let name = (*snd_soc_rtd_to_codec(rtd, 0)).name;
    let head = &mut (*dsp).compr_list as *mut list_head;
    let mut p = (*head).next;
    while p != head {
        let tmp: *mut wm_adsp_compr = container_of(p, offset_of!(wm_adsp_compr, list));
        if strcmp((*tmp).name, name) == 0 { mutex_unlock(&mut (*dsp).cs_dsp.pwr_lock); return -EBUSY; }
        p = (*p).next;
    }
    let compr = kzalloc(size_of::<wm_adsp_compr>(), GFP_KERNEL) as *mut wm_adsp_compr;
    if compr.is_null() { mutex_unlock(&mut (*dsp).cs_dsp.pwr_lock); return -ENOMEM; }
    (*compr).dsp = dsp;
    (*compr).stream = stream;
    (*compr).name = name;
    list_add_tail(&mut (*compr).list, &mut (*dsp).compr_list);
    (*(*stream).runtime).private_data = compr as *mut c_void;
    mutex_unlock(&mut (*dsp).cs_dsp.pwr_lock);
    0
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp_compr_free(_component: *mut snd_soc_component, stream: *mut snd_compr_stream) -> c_int {
    let compr = (*(*stream).runtime).private_data as *mut wm_adsp_compr;
    let dsp = (*compr).dsp;
    mutex_lock(&mut (*dsp).cs_dsp.pwr_lock);
    wm_adsp_compr_detach(compr);
    list_del(&mut (*compr).list);
    kfree((*compr).raw_buf as *mut c_void);
    kfree(compr as *mut c_void);
    mutex_unlock(&mut (*dsp).cs_dsp.pwr_lock);
    0
}

unsafe fn wm_adsp_compr_check_params(stream: *mut snd_compr_stream, params: *mut snd_compr_params) -> c_int {
    let compr = (*(*stream).runtime).private_data as *mut wm_adsp_compr;
    let dsp = (*compr).dsp;
    if (*params).buffer.fragment_size < WM_ADSP_MIN_FRAGMENT_SIZE || (*params).buffer.fragment_size > WM_ADSP_MAX_FRAGMENT_SIZE || (*params).buffer.fragments < WM_ADSP_MIN_FRAGMENTS || (*params).buffer.fragments > WM_ADSP_MAX_FRAGMENTS || (*params).buffer.fragment_size % CS_DSP_DATA_WORD_SIZE != 0 { return -EINVAL; }
    for i in 0..wm_adsp_fw[(*dsp).fw as usize].num_caps as isize {
        let caps = wm_adsp_fw[(*dsp).fw as usize].caps.offset(i);
        let desc = &(*caps).desc;
        if (*caps).id != (*params).codec.id { continue; }
        if (*stream).direction == SND_COMPRESS_PLAYBACK {
            if desc.max_ch < (*params).codec.ch_out { continue; }
        } else if desc.max_ch < (*params).codec.ch_in { continue; }
        if desc.formats & (1u64 << (*params).codec.format) == 0 { continue; }
        for j in 0..desc.num_sample_rates as usize {
            if desc.sample_rates[j] == (*params).codec.sample_rate { return 0; }
        }
    }
    -EINVAL
}

unsafe fn wm_adsp_compr_frag_words(compr: *mut wm_adsp_compr) -> c_uint { (*compr).size.fragment_size / CS_DSP_DATA_WORD_SIZE }

#[no_mangle] pub unsafe extern "C" fn wm_adsp_compr_set_params(_component: *mut snd_soc_component, stream: *mut snd_compr_stream, params: *mut snd_compr_params) -> c_int {
    let compr = (*(*stream).runtime).private_data as *mut wm_adsp_compr;
    let ret = wm_adsp_compr_check_params(stream, params);
    if ret != 0 { return ret; }
    (*compr).size = (*params).buffer;
    let size = wm_adsp_compr_frag_words(compr) as size_t * size_of::<u32>();
    (*compr).raw_buf = kmalloc(size, GFP_DMA | GFP_KERNEL) as *mut u32;
    if (*compr).raw_buf.is_null() { return -ENOMEM; }
    (*compr).sample_rate = (*params).codec.sample_rate;
    0
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp_compr_get_caps(_component: *mut snd_soc_component, stream: *mut snd_compr_stream, caps: *mut snd_compr_caps) -> c_int {
    let compr = (*(*stream).runtime).private_data as *mut wm_adsp_compr;
    let fw = (*(*compr).dsp).fw as usize;
    if !wm_adsp_fw[fw].caps.is_null() {
        let mut i = 0;
        while i < wm_adsp_fw[fw].num_caps {
            (*caps).codecs[i as usize] = (*wm_adsp_fw[fw].caps.offset(i as isize)).id;
            i += 1;
        }
        (*caps).num_codecs = i as c_uint;
        (*caps).direction = wm_adsp_fw[fw].compr_direction;
        (*caps).min_fragment_size = WM_ADSP_MIN_FRAGMENT_SIZE;
        (*caps).max_fragment_size = WM_ADSP_MAX_FRAGMENT_SIZE;
        (*caps).min_fragments = WM_ADSP_MIN_FRAGMENTS;
        (*caps).max_fragments = WM_ADSP_MAX_FRAGMENTS;
    }
    0
}

unsafe fn wm_adsp_buffer_read(buf: *mut wm_adsp_compr_buf, field_offset: c_uint, data: *mut u32) -> c_int {
    cs_dsp_read_data_word(&mut (*(*buf).dsp).cs_dsp, (*buf).host_buf_mem_type, (*buf).host_buf_ptr + field_offset, data)
}
unsafe fn wm_adsp_buffer_write(buf: *mut wm_adsp_compr_buf, field_offset: c_uint, data: u32) -> c_int {
    cs_dsp_write_data_word(&mut (*(*buf).dsp).cs_dsp, (*buf).host_buf_mem_type, (*buf).host_buf_ptr + field_offset, data)
}

unsafe fn wm_adsp_buffer_populate(buf: *mut wm_adsp_compr_buf) -> c_int {
    let caps = wm_adsp_fw[(*(*buf).dsp).fw as usize].caps;
    (*buf).regions = kzalloc(size_of::<wm_adsp_buffer_region>() * (*caps).num_regions as usize, GFP_KERNEL) as *mut wm_adsp_buffer_region;
    if (*buf).regions.is_null() { return -ENOMEM; }
    let mut offset = 0;
    for i in 0..(*caps).num_regions as isize {
        let region = (*buf).regions.offset(i);
        let def = (*caps).region_defs.offset(i);
        (*region).offset = offset;
        (*region).mem_type = (*def).mem_type;
        let mut ret = wm_adsp_buffer_read(buf, (*def).base_offset, &mut (*region).base_addr);
        if ret < 0 { kfree((*buf).regions as *mut c_void); return ret; }
        ret = wm_adsp_buffer_read(buf, (*def).size_offset, &mut offset);
        if ret < 0 { kfree((*buf).regions as *mut c_void); return ret; }
        (*region).cumulative_size = offset;
    }
    0
}

unsafe fn wm_adsp_buffer_clear(buf: *mut wm_adsp_compr_buf) { (*buf).irq_count = 0xFFFFFFFF; (*buf).read_index = -1; (*buf).avail = 0; }
unsafe fn wm_adsp_buffer_alloc(dsp: *mut wm_adsp) -> *mut wm_adsp_compr_buf {
    let buf = kzalloc(size_of::<wm_adsp_compr_buf>(), GFP_KERNEL) as *mut wm_adsp_compr_buf;
    if buf.is_null() { return ptr::null_mut(); }
    (*buf).dsp = dsp;
    wm_adsp_buffer_clear(buf);
    buf
}

unsafe fn wm_adsp_buffer_parse_legacy(dsp: *mut wm_adsp) -> c_int {
    let alg_region = cs_dsp_find_alg_region(&mut (*dsp).cs_dsp, WMFW_ADSP2_XM, (*dsp).cs_dsp.fw_id);
    if alg_region.is_null() { return -EINVAL; }
    let xmalg = ((*dsp).sys_config_size / size_of::<__be32>()) as u32;
    let mut magic = 0;
    let mut addr = (*alg_region).base + xmalg + ALG_XM_FIELD_magic();
    let mut ret = cs_dsp_read_data_word(&mut (*dsp).cs_dsp, WMFW_ADSP2_XM as c_int, addr, &mut magic);
    if ret < 0 { return ret; }
    if magic != WM_ADSP_ALG_XM_STRUCT_MAGIC { return -ENODEV; }
    let buf = wm_adsp_buffer_alloc(dsp);
    if buf.is_null() { return -ENOMEM; }
    addr = (*alg_region).base + xmalg + ALG_XM_FIELD_host_buf_ptr();
    for _ in 0..5 {
        ret = cs_dsp_read_data_word(&mut (*dsp).cs_dsp, WMFW_ADSP2_XM as c_int, addr, &mut (*buf).host_buf_ptr);
        if ret < 0 { kfree(buf as *mut c_void); return ret; }
        if (*buf).host_buf_ptr != 0 { break; }
        usleep_range(1000, 2000);
    }
    if (*buf).host_buf_ptr == 0 { kfree(buf as *mut c_void); return -EIO; }
    (*buf).host_buf_mem_type = WMFW_ADSP2_XM as c_int;
    ret = wm_adsp_buffer_populate(buf);
    if ret < 0 { kfree(buf as *mut c_void); return ret; }
    list_add_tail(&mut (*buf).list, &mut (*dsp).buffer_list);
    0
}

unsafe fn wm_adsp_buffer_parse_coeff(cs_ctl: *mut cs_dsp_coeff_ctl) -> c_int {
    let mut coeff_v1: wm_adsp_host_buf_coeff_v1 = core::mem::zeroed();
    let dsp: *mut wm_adsp = container_of((*cs_ctl).dsp, offset_of!(wm_adsp, cs_dsp));
    let mut version: c_uint = 0;
    let mut ret = 0;
    for _ in 0..5 {
        let len = if (*cs_ctl).len as usize > size_of::<wm_adsp_host_buf_coeff_v1>() { size_of::<wm_adsp_host_buf_coeff_v1>() } else { (*cs_ctl).len as usize };
        ret = cs_dsp_coeff_read_ctrl(cs_ctl, 0, &mut coeff_v1 as *mut _ as *mut c_void, len);
        if ret < 0 { return ret; }
        if coeff_v1.host_buf_ptr != 0 { break; }
        usleep_range(1000, 2000);
    }
    if coeff_v1.host_buf_ptr == 0 { return -EIO; }
    let buf = wm_adsp_buffer_alloc(dsp);
    if buf.is_null() { return -ENOMEM; }
    (*buf).host_buf_mem_type = (*cs_ctl).alg_region.type_ as c_int;
    (*buf).host_buf_ptr = be32_to_cpu(coeff_v1.host_buf_ptr);
    ret = wm_adsp_buffer_populate(buf);
    if ret < 0 { kfree(buf as *mut c_void); return ret; }
    if (*cs_ctl).len != 4 {
        version = (be32_to_cpu(coeff_v1.versions) & HOST_BUF_COEFF_COMPAT_VER_MASK) >> HOST_BUF_COEFF_COMPAT_VER_SHIFT;
        if version > HOST_BUF_COEFF_SUPPORTED_COMPAT_VER { kfree(buf as *mut c_void); return -EINVAL; }
        cs_dsp_remove_padding(coeff_v1.name.as_mut_ptr(), coeff_v1.name.len() as c_int);
        (*buf).name = kasprintf(GFP_KERNEL, cstr!("%s-dsp-%s"), (*dsp).part, coeff_v1.name.as_ptr() as *const c_char);
    }
    list_add_tail(&mut (*buf).list, &mut (*dsp).buffer_list);
    version as c_int
}

unsafe fn wm_adsp_buffer_init(dsp: *mut wm_adsp) -> c_int {
    let head = &mut (*dsp).cs_dsp.ctl_list as *mut list_head;
    let mut p = (*head).next;
    while p != head {
        let cs_ctl: *mut cs_dsp_coeff_ctl = container_of(p, offset_of!(cs_dsp_coeff_ctl, list));
        if (*cs_ctl).type_ == WMFW_CTL_TYPE_HOST_BUFFER && (*cs_ctl).enabled {
            let ret = wm_adsp_buffer_parse_coeff(cs_ctl);
            if ret < 0 { wm_adsp_buffer_free(dsp); return ret; }
            else if ret == 0 { return 0; }
        }
        p = (*p).next;
    }
    if list_empty(&(*dsp).buffer_list) != 0 {
        let ret = wm_adsp_buffer_parse_legacy(dsp);
        if ret != 0 && ret != -ENODEV { adsp_warn!(dsp, "Failed to parse legacy: %d\n", ret); }
    }
    0
}

unsafe fn wm_adsp_buffer_free(dsp: *mut wm_adsp) -> c_int {
    let head = &mut (*dsp).buffer_list as *mut list_head;
    let mut p = (*head).next;
    while p != head {
        let next = (*p).next;
        let buf: *mut wm_adsp_compr_buf = container_of(p, offset_of!(wm_adsp_compr_buf, list));
        wm_adsp_compr_detach((*buf).compr);
        kfree((*buf).name as *mut c_void);
        kfree((*buf).regions as *mut c_void);
        list_del(&mut (*buf).list);
        kfree(buf as *mut c_void);
        p = next;
    }
    0
}

unsafe fn wm_adsp_buffer_get_error(buf: *mut wm_adsp_compr_buf) -> c_int {
    let ret = wm_adsp_buffer_read(buf, HOST_BUFFER_FIELD_error(), &mut (*buf).error);
    if ret < 0 { return ret; }
    if (*buf).error != 0 { return -EIO; }
    0
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp_compr_trigger(_component: *mut snd_soc_component, stream: *mut snd_compr_stream, cmd: c_int) -> c_int {
    let compr = (*(*stream).runtime).private_data as *mut wm_adsp_compr;
    let dsp = (*compr).dsp;
    let mut ret = 0;
    mutex_lock(&mut (*dsp).cs_dsp.pwr_lock);
    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            if wm_adsp_compr_attached(compr) == 0 {
                ret = wm_adsp_compr_attach(compr);
                if ret < 0 { mutex_unlock(&mut (*dsp).cs_dsp.pwr_lock); return ret; }
            }
            ret = wm_adsp_buffer_get_error((*compr).buf);
            if ret >= 0 { ret = wm_adsp_buffer_write((*compr).buf, HOST_BUFFER_FIELD_high_water_mark(), wm_adsp_compr_frag_words(compr)); }
        }
        SNDRV_PCM_TRIGGER_STOP => if wm_adsp_compr_attached(compr) != 0 { wm_adsp_buffer_clear((*compr).buf); },
        _ => ret = -EINVAL,
    }
    mutex_unlock(&mut (*dsp).cs_dsp.pwr_lock);
    ret
}

unsafe fn wm_adsp_buffer_size(buf: *mut wm_adsp_compr_buf) -> c_int {
    let last_region = (*wm_adsp_fw[(*(*buf).dsp).fw as usize].caps).num_regions - 1;
    (*(*buf).regions.offset(last_region as isize)).cumulative_size as c_int
}

unsafe fn wm_adsp_buffer_update_avail(buf: *mut wm_adsp_compr_buf) -> c_int {
    let mut next_read_index = 0;
    let mut next_write_index = 0;
    if (*buf).read_index < 0 {
        let ret = wm_adsp_buffer_read(buf, HOST_BUFFER_FIELD_next_read_index(), &mut next_read_index);
        if ret < 0 { return ret; }
        let read_index = sign_extend32(next_read_index, 23);
        if read_index < 0 { return 0; }
        (*buf).read_index = read_index;
    }
    let ret = wm_adsp_buffer_read(buf, HOST_BUFFER_FIELD_next_write_index(), &mut next_write_index);
    if ret < 0 { return ret; }
    let write_index = sign_extend32(next_write_index, 23);
    let mut avail = write_index - (*buf).read_index;
    if avail < 0 { avail += wm_adsp_buffer_size(buf); }
    (*buf).avail = avail;
    0
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp_compr_handle_irq(dsp: *mut wm_adsp) -> c_int {
    let mut ret = 0;
    mutex_lock(&mut (*dsp).cs_dsp.pwr_lock);
    if list_empty(&(*dsp).buffer_list) != 0 { mutex_unlock(&mut (*dsp).cs_dsp.pwr_lock); return -ENODEV; }
    let head = &mut (*dsp).buffer_list as *mut list_head;
    let mut p = (*head).next;
    while p != head {
        let buf: *mut wm_adsp_compr_buf = container_of(p, offset_of!(wm_adsp_compr_buf, list));
        let compr = (*buf).compr;
        ret = wm_adsp_buffer_get_error(buf);
        if ret >= 0 { ret = wm_adsp_buffer_read(buf, HOST_BUFFER_FIELD_irq_count(), &mut (*buf).irq_count); }
        if ret >= 0 { ret = wm_adsp_buffer_update_avail(buf); }
        if ret >= 0 && wm_adsp_fw[(*dsp).fw as usize].voice_trigger && (*buf).irq_count == 2 { ret = WM_ADSP_COMPR_VOICE_TRIGGER; }
        if !compr.is_null() && !(*compr).stream.is_null() { snd_compr_fragment_elapsed((*compr).stream); }
        if ret < 0 { break; }
        p = (*p).next;
    }
    mutex_unlock(&mut (*dsp).cs_dsp.pwr_lock);
    ret
}

unsafe fn wm_adsp_buffer_reenable_irq(buf: *mut wm_adsp_compr_buf) -> c_int {
    if (*buf).irq_count & 0x01 != 0 { return 0; }
    (*buf).irq_count |= 0x01;
    wm_adsp_buffer_write(buf, HOST_BUFFER_FIELD_irq_ack(), (*buf).irq_count)
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp_compr_pointer(_component: *mut snd_soc_component, stream: *mut snd_compr_stream, tstamp: *mut snd_compr_tstamp64) -> c_int {
    let compr = (*(*stream).runtime).private_data as *mut wm_adsp_compr;
    let dsp = (*compr).dsp;
    mutex_lock(&mut (*dsp).cs_dsp.pwr_lock);
    let buf = (*compr).buf;
    if (*dsp).fatal_error || buf.is_null() || (*buf).error != 0 {
        snd_compr_stop_error(stream, SNDRV_PCM_STATE_XRUN);
        mutex_unlock(&mut (*dsp).cs_dsp.pwr_lock);
        return -EIO;
    }
    let mut ret = 0;
    if (*buf).avail < wm_adsp_compr_frag_words(compr) as c_int {
        ret = wm_adsp_buffer_update_avail(buf);
        if ret >= 0 && (*buf).avail < wm_adsp_compr_frag_words(compr) as c_int {
            ret = wm_adsp_buffer_get_error(buf);
            if ret >= 0 { ret = wm_adsp_buffer_reenable_irq(buf); }
        }
    }
    if ret >= 0 {
        (*tstamp).copied_total = (*compr).copied_total + ((*buf).avail as u64 * CS_DSP_DATA_WORD_SIZE as u64);
        (*tstamp).sampling_rate = (*compr).sample_rate;
    }
    mutex_unlock(&mut (*dsp).cs_dsp.pwr_lock);
    ret
}

unsafe fn wm_adsp_buffer_capture_block(compr: *mut wm_adsp_compr, target: c_int) -> c_int {
    let buf = (*compr).buf;
    let mut i = 0;
    while i < (*wm_adsp_fw[(*(*buf).dsp).fw as usize].caps).num_regions {
        if (*buf).read_index < (*(*buf).regions.offset(i as isize)).cumulative_size as c_int { break; }
        i += 1;
    }
    if i == (*wm_adsp_fw[(*(*buf).dsp).fw as usize].caps).num_regions { return -EINVAL; }
    let region = (*buf).regions.offset(i as isize);
    let mem_type = (*region).mem_type as c_int;
    let adsp_addr = (*region).base_addr + ((*buf).read_index - (*region).offset as c_int) as c_uint;
    let max_read = wm_adsp_compr_frag_words(compr) as c_int;
    let mut nwords = (*region).cumulative_size as c_int - (*buf).read_index;
    if nwords > target { nwords = target; }
    if nwords > (*buf).avail { nwords = (*buf).avail; }
    if nwords > max_read { nwords = max_read; }
    if nwords == 0 { return 0; }
    let ret = cs_dsp_read_raw_data_block(&mut (*(*buf).dsp).cs_dsp, mem_type, adsp_addr, nwords, (*compr).raw_buf as *mut __be32);
    if ret < 0 { return ret; }
    cs_dsp_remove_padding((*compr).raw_buf, nwords);
    (*buf).read_index += nwords;
    if (*buf).read_index == wm_adsp_buffer_size(buf) { (*buf).read_index = 0; }
    let ret2 = wm_adsp_buffer_write(buf, HOST_BUFFER_FIELD_next_read_index(), (*buf).read_index as u32);
    if ret2 < 0 { return ret2; }
    (*buf).avail -= nwords;
    nwords
}

unsafe fn wm_adsp_compr_read(compr: *mut wm_adsp_compr, buf: *mut c_char, mut count: size_t) -> c_int {
    let dsp = (*compr).dsp;
    let mut ntotal = 0;
    if (*dsp).fatal_error || (*compr).buf.is_null() || (*(*compr).buf).error != 0 {
        snd_compr_stop_error((*compr).stream, SNDRV_PCM_STATE_XRUN);
        return -EIO;
    }
    count /= CS_DSP_DATA_WORD_SIZE as usize;
    loop {
        let nwords = wm_adsp_buffer_capture_block(compr, count as c_int);
        if nwords < 0 { return nwords; }
        let nbytes = nwords * CS_DSP_DATA_WORD_SIZE as c_int;
        if copy_to_user(buf.add(ntotal as usize) as *mut c_void, (*compr).raw_buf as *const c_void, nbytes as size_t) != 0 { return -EFAULT; }
        count -= nwords as usize;
        ntotal += nbytes;
        if !(nwords > 0 && count > 0) { break; }
    }
    (*compr).copied_total += ntotal as u64;
    ntotal
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp_compr_copy(_component: *mut snd_soc_component, stream: *mut snd_compr_stream, buf: *mut c_char, count: size_t) -> c_int {
    let compr = (*(*stream).runtime).private_data as *mut wm_adsp_compr;
    let dsp = (*compr).dsp;
    mutex_lock(&mut (*dsp).cs_dsp.pwr_lock);
    let ret = if (*stream).direction == SND_COMPRESS_CAPTURE { wm_adsp_compr_read(compr, buf, count) } else { -ENOTSUPP };
    mutex_unlock(&mut (*dsp).cs_dsp.pwr_lock);
    ret
}

unsafe extern "C" fn wm_adsp_fatal_error(cs_dsp: *mut cs_dsp) {
    let dsp: *mut wm_adsp = container_of(cs_dsp, offset_of!(wm_adsp, cs_dsp));
    (*dsp).fatal_error = true;
    let head = &mut (*dsp).compr_list as *mut list_head;
    let mut p = (*head).next;
    while p != head {
        let compr: *mut wm_adsp_compr = container_of(p, offset_of!(wm_adsp_compr, list));
        if !(*compr).stream.is_null() { snd_compr_fragment_elapsed((*compr).stream); }
        p = (*p).next;
    }
}

#[no_mangle] pub unsafe extern "C" fn wm_adsp2_bus_error(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let dsp = data as *mut wm_adsp;
    cs_dsp_adsp2_bus_error(&mut (*dsp).cs_dsp);
    IRQ_HANDLED
}

#[no_mangle] pub unsafe extern "C" fn wm_halo_bus_error(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let dsp = data as *mut wm_adsp;
    cs_dsp_halo_bus_error(&mut (*dsp).cs_dsp);
    IRQ_HANDLED
}

#[no_mangle] pub unsafe extern "C" fn wm_halo_wdt_expire(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let dsp = data as *mut wm_adsp;
    cs_dsp_halo_wdt_expire(&mut (*dsp).cs_dsp);
    IRQ_HANDLED
}

static wm_adsp1_client_ops: cs_dsp_client_ops = cs_dsp_client_ops {
    control_add: Some(wm_adsp_control_add_cb),
    control_remove: Some(wm_adsp_control_remove),
    pre_run: None,
    post_run: None,
    post_stop: None,
    watchdog_expired: None,
};

static wm_adsp2_client_ops: cs_dsp_client_ops = cs_dsp_client_ops {
    control_add: Some(wm_adsp_control_add_cb),
    control_remove: Some(wm_adsp_control_remove),
    pre_run: Some(wm_adsp_pre_run),
    post_run: Some(wm_adsp_event_post_run),
    post_stop: Some(wm_adsp_event_post_stop),
    watchdog_expired: Some(wm_adsp_fatal_error),
};

/* MODULE_DESCRIPTION("Cirrus Logic ASoC DSP Support"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_IMPORT_NS("FW_CS_DSP"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
