// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for RME Digi9652 audio interfaces
 *
 *      Copyright (c) 1999 IEM - Winfried Ritsch
 *      Copyright (c) 1999-2001  Paul Davis
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

/* C include dependencies intentionally remain external: linux/delay.h,
 * linux/interrupt.h, linux/pci.h, linux/io.h, linux/nospec.h, sound/*,
 * asm/current.h.
 */

type u32 = u32;
type size_t = usize;
type pid_t = i32;
type snd_pcm_uframes_t = c_ulong;
type snd_pcm_sframes_t = c_long;
type irqreturn_t = c_int;
type dma_addr_t = c_ulong;
type spinlock_t = c_void;

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: c_int = -1;
const SNDRV_DEFAULT_ENABLE_PNP: bool = true;
const PCI_ANY_ID: u32 = !0;
const PCI_CLASS_REVISION: c_int = 0x08;
const IRQF_SHARED: c_ulong = 0x80;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_IOCTL1_RESET: c_uint = 0;
const SNDRV_PCM_IOCTL1_CHANNEL_INFO: c_uint = 1;
const SNDRV_PCM_INFO_MMAP: u32 = 1 << 0;
const SNDRV_PCM_INFO_MMAP_VALID: u32 = 1 << 1;
const SNDRV_PCM_INFO_NONINTERLEAVED: u32 = 1 << 2;
const SNDRV_PCM_INFO_SYNC_START: u32 = 1 << 3;
const SNDRV_PCM_INFO_DOUBLE: u32 = 1 << 4;
const SNDRV_PCM_INFO_JOINT_DUPLEX: u32 = 1 << 5;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 10;
const SNDRV_PCM_RATE_44100: u32 = 1 << 0;
const SNDRV_PCM_RATE_48000: u32 = 1 << 1;
const SNDRV_PCM_RATE_88200: u32 = 1 << 2;
const SNDRV_PCM_RATE_96000: u32 = 1 << 3;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_HW_PARAM_PERIOD_SIZE: c_int = 1;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 2;
const SNDRV_CTL_ELEM_TYPE_IEC958: c_int = 0;
const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_int = 1;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_int = 2;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_int = 0;
const SNDRV_CTL_ELEM_IFACE_PCM: c_int = 1;
const SNDRV_CTL_ELEM_ACCESS_READ: c_uint = 1 << 0;
const SNDRV_CTL_ELEM_ACCESS_WRITE: c_uint = 1 << 1;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_WRITE;
const SNDRV_CTL_ELEM_ACCESS_INACTIVE: c_uint = 1 << 8;
const SNDRV_CTL_ELEM_ACCESS_VOLATILE: c_uint = 1 << 9;
const SNDRV_CTL_EVENT_MASK_VALUE: c_uint = 1 << 0;
const SNDRV_CTL_EVENT_MASK_INFO: c_uint = 1 << 1;
const IEC958_AES0_PROFESSIONAL: u8 = 1 << 0;
const IEC958_AES0_NONAUDIO: u8 = 1 << 1;
const IEC958_AES0_PRO_EMPHASIS_5015: u8 = 1 << 2;
const IEC958_AES0_CON_EMPHASIS_5015: u8 = 1 << 3;
const IEC958_AES0_CON_EMPHASIS: c_long = 0xff;
const IEC958_AES0_PRO_EMPHASIS: c_long = 0xff;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EIO: c_int = 5;
const EFAULT: c_int = 14;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;

static mut index: [c_int; SNDRV_CARDS] = [SNDRV_DEFAULT_IDX; SNDRV_CARDS]; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS]; /* ID for this card */
static mut enable: [bool; SNDRV_CARDS] = [SNDRV_DEFAULT_ENABLE_PNP; SNDRV_CARDS]; /* Enable this card */
static mut precise_ptr: [bool; SNDRV_CARDS] = [false; SNDRV_CARDS]; /* Enable precise pointer */

/* module_param_array/MODULE_* metadata from C is preserved as module intent:
 * index, id, enable, precise_ptr parameters; author Paul Davis/Winfried Ritsch;
 * description RME Digi9652/Digi9636; license GPL.
 */

const RME9652_NCHANNELS: c_int = 26;
const RME9636_NCHANNELS: c_int = 18;
const RME9652_SYNC_FROM_SPDIF: c_int = 0;
const RME9652_SYNC_FROM_ADAT1: c_int = 1;
const RME9652_SYNC_FROM_ADAT2: c_int = 2;
const RME9652_SYNC_FROM_ADAT3: c_int = 3;
const RME9652_SPDIFIN_OPTICAL: c_int = 0;
const RME9652_SPDIFIN_COAXIAL: c_int = 1;
const RME9652_SPDIFIN_INTERN: c_int = 2;

const RME9652_IRQ: u32 = 1 << 0;
const RME9652_lock_2: u32 = 1 << 1;
const RME9652_lock_1: u32 = 1 << 2;
const RME9652_lock_0: u32 = 1 << 3;
const RME9652_fs48: u32 = 1 << 4;
const RME9652_wsel_rd: u32 = 1 << 5;
const RME9652_sync_2: u32 = 1 << 16;
const RME9652_sync_1: u32 = 1 << 17;
const RME9652_sync_0: u32 = 1 << 18;
const RME9652_DS_rd: u32 = 1 << 19;
const RME9652_tc_busy: u32 = 1 << 20;
const RME9652_tc_out: u32 = 1 << 21;
const RME9652_F_0: u32 = 1 << 22;
const RME9652_F_1: u32 = 1 << 23;
const RME9652_F_2: u32 = 1 << 24;
const RME9652_ERF: u32 = 1 << 25;
const RME9652_buffer_id: u32 = 1 << 26;
const RME9652_tc_valid: u32 = 1 << 27;
const RME9652_SPDIF_READ: u32 = 1 << 28;
const RME9652_sync: u32 = RME9652_sync_0 | RME9652_sync_1 | RME9652_sync_2;
const RME9652_lock: u32 = RME9652_lock_0 | RME9652_lock_1 | RME9652_lock_2;
const RME9652_F: u32 = RME9652_F_0 | RME9652_F_1 | RME9652_F_2;
#[inline] unsafe fn rme9652_decode_spdif_rate(x: u32) -> u32 { x >> 22 }
const RME9652_buf_pos: u32 = 0x000F_FC0;
#[inline] unsafe fn RME9652_REV15_buf_pos(x: u32) -> u32 { ((x & 0xE000_0000) >> 26) | (x & RME9652_buf_pos) }
const RME9652_IO_EXTENT: c_ulong = 1024;
const RME9652_init_buffer: c_int = 0;
const RME9652_play_buffer: c_int = 32;
const RME9652_rec_buffer: c_int = 36;
const RME9652_control_register: c_int = 64;
const RME9652_irq_clear: c_int = 96;
const RME9652_time_code: c_int = 100;
const RME9652_thru_base: c_int = 128;
const RME9652_status_register: c_int = 0;
const RME9652_start_bit: u32 = 1 << 0;
const RME9652_Master: u32 = 1 << 4;
const RME9652_IE: u32 = 1 << 5;
const RME9652_freq: u32 = 1 << 6;
const RME9652_freq1: u32 = 1 << 7;
const RME9652_DS: u32 = 1 << 8;
const RME9652_PRO: u32 = 1 << 9;
const RME9652_EMP: u32 = 1 << 10;
const RME9652_Dolby: u32 = 1 << 11;
const RME9652_opt_out: u32 = 1 << 12;
const RME9652_wsel: u32 = 1 << 13;
const RME9652_inp_0: u32 = 1 << 14;
const RME9652_inp_1: u32 = 1 << 15;
const RME9652_SyncPref_ADAT2: u32 = 1 << 16;
const RME9652_SyncPref_ADAT3: u32 = 1 << 17;
const RME9652_SPDIF_RESET: u32 = 1 << 18;
const RME9652_SPDIF_SELECT: u32 = 1 << 19;
const RME9652_SPDIF_CLOCK: u32 = 1 << 20;
const RME9652_SPDIF_WRITE: u32 = 1 << 21;
const RME9652_ADAT1_INTERNAL: u32 = 1 << 22;
const RME9652_latency: u32 = 0x0e;
#[inline] unsafe fn rme9652_encode_latency(x: c_int) -> u32 { (((x as u32) & 0x7) << 1) }
#[inline] unsafe fn rme9652_decode_latency(x: u32) -> u32 { (x >> 1) & 0x7 }
#[inline] unsafe fn rme9652_running_double_speed(s: *mut snd_rme9652) -> u32 { (*s).control_register & RME9652_DS }
const RME9652_inp: u32 = RME9652_inp_0 | RME9652_inp_1;
#[inline] unsafe fn rme9652_encode_spdif_in(x: c_int) -> u32 { (((x as u32) & 0x3) << 14) }
#[inline] unsafe fn rme9652_decode_spdif_in(x: u32) -> u32 { (x >> 14) & 0x3 }
const RME9652_SyncPref_Mask: u32 = RME9652_SyncPref_ADAT2 | RME9652_SyncPref_ADAT3;
const RME9652_SyncPref_ADAT1: u32 = 0;
const RME9652_SyncPref_SPDIF: u32 = RME9652_SyncPref_ADAT2 | RME9652_SyncPref_ADAT3;
const RME9652_CHANNEL_BUFFER_SAMPLES: usize = 16 * 1024;
const RME9652_CHANNEL_BUFFER_BYTES: usize = 4 * RME9652_CHANNEL_BUFFER_SAMPLES;
const RME9652_DMA_AREA_BYTES: usize = (RME9652_NCHANNELS as usize + 1) * RME9652_CHANNEL_BUFFER_BYTES;
const RME9652_DMA_AREA_KILOBYTES: usize = RME9652_DMA_AREA_BYTES / 1024;

#[repr(C)] pub struct device { _priv: [u8; 0] }
#[repr(C)] pub struct pci_dev { pub dev: device, pub irq: c_int }
#[repr(C)] pub struct pci_device_id { pub vendor: u32, pub device: u32, pub subvendor: u32, pub subdevice: u32 }
#[repr(C)] pub struct snd_card { pub number: c_int, pub dev: *mut device, pub private_data: *mut c_void, pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>, pub sync_irq: c_int, pub driver: [c_char; 16], pub shortname: [c_char; 32], pub longname: [c_char; 80] }
#[repr(C)] pub struct snd_pcm { pub private_data: *mut c_void, pub streams: [snd_pcm_str; 2], pub info_flags: u32, pub name: [c_char; 80] }
#[repr(C)] pub struct snd_pcm_str { pub substream: *mut snd_pcm_substream }
#[repr(C)] pub struct snd_pcm_substream { pub runtime: *mut snd_pcm_runtime, pub pstr: *mut snd_pcm_str_runtime, pub stream: c_int }
#[repr(C)] pub struct snd_pcm_str_runtime { pub stream: c_int }
#[repr(C)] pub struct snd_pcm_runtime { pub hw: snd_pcm_hardware, pub status: *mut snd_pcm_mmap_status }
#[repr(C)] pub struct snd_pcm_mmap_status { pub hw_ptr: snd_pcm_uframes_t }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_pcm_hardware { pub info: u32, pub formats: u64, pub rates: u32, pub rate_min: c_uint, pub rate_max: c_uint, pub channels_min: c_uint, pub channels_max: c_uint, pub buffer_bytes_max: usize, pub period_bytes_min: usize, pub period_bytes_max: usize, pub periods_min: c_uint, pub periods_max: c_uint, pub fifo_size: usize }
#[repr(C)] pub struct snd_pcm_hw_params { _priv: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_rule { pub private: *mut c_void }
#[repr(C)] pub struct snd_interval { pub min: c_uint, pub max: c_uint, pub openmin: c_uint, pub openmax: c_uint, pub integer: c_uint, pub empty: c_uint }
#[repr(C)] pub struct snd_pcm_channel_info { pub channel: c_uint, pub offset: c_int, pub first: c_uint, pub step: c_uint }
#[repr(C)] pub struct iov_iter { _priv: [u8; 0] }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_dma_buffer { pub area: *mut i8, pub addr: dma_addr_t, pub bytes: usize }
#[repr(C)] pub struct snd_info_entry { pub private_data: *mut c_void }
#[repr(C)] pub struct snd_info_buffer { _priv: [u8; 0] }
#[repr(C)] pub struct snd_aes_iec958 { pub status: [u8; 24], pub subcode: [u8; 147], pub pad: u8, pub dig_subframe: [u8; 4] }
#[repr(C)] pub struct snd_ctl_elem_info_integer { pub min: c_long, pub max: c_long }
#[repr(C)] pub union snd_ctl_elem_info_value { pub integer: core::mem::ManuallyDrop<snd_ctl_elem_info_integer> }
#[repr(C)] pub struct snd_ctl_elem_info { pub type_: c_int, pub count: c_uint, pub value: snd_ctl_elem_info_value }
#[repr(C)] pub struct snd_ctl_elem_value_integer { pub value: [c_long; 64] }
#[repr(C)] pub struct snd_ctl_elem_value_enumerated { pub item: [c_uint; 64] }
#[repr(C)] pub union snd_ctl_elem_value_value { pub integer: core::mem::ManuallyDrop<snd_ctl_elem_value_integer>, pub enumerated: core::mem::ManuallyDrop<snd_ctl_elem_value_enumerated>, pub iec958: core::mem::ManuallyDrop<snd_aes_iec958> }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }
#[repr(C)] pub struct snd_kcontrol_volatile { pub access: c_uint }
#[repr(C)] pub struct snd_ctl_elem_id { _priv: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { pub private_value: c_long, pub vd: [snd_kcontrol_volatile; 1], pub id: snd_ctl_elem_id }
#[repr(C)] pub struct snd_kcontrol_new { pub iface: c_int, pub name: *const c_char, pub index: c_uint, pub access: c_uint, pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>, pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, pub private_value: c_long }
#[repr(C)] pub struct pci_driver { pub name: *const c_char, pub id_table: *const pci_device_id, pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int> }
#[repr(C)] pub struct task_struct { pub pid: pid_t }

#[repr(C)]
pub struct snd_rme9652 {
    pub dev: c_int, pub lock: spinlock_t, pub irq: c_int, pub port: c_ulong, pub iobase: *mut c_void,
    pub precise_ptr: c_int, pub control_register: u32, pub thru_bits: u32, pub creg_spdif: u32, pub creg_spdif_stream: u32,
    pub card_name: *const c_char, pub hw_offsetmask: size_t, pub prev_hw_offset: size_t, pub max_jitter: size_t, pub period_bytes: size_t,
    pub ds_channels: u8, pub ss_channels: u8, pub playback_dma_buf: snd_dma_buffer, pub capture_dma_buf: snd_dma_buffer,
    pub capture_buffer: *mut u8, pub playback_buffer: *mut u8, pub capture_pid: pid_t, pub playback_pid: pid_t,
    pub capture_substream: *mut snd_pcm_substream, pub playback_substream: *mut snd_pcm_substream, pub running: c_int,
    pub passthru: c_int, pub hw_rev: c_int, pub last_spdif_sample_rate: c_int, pub last_adat_sample_rate: c_int,
    pub channel_map: *const i8, pub card: *mut snd_card, pub pcm: *mut snd_pcm, pub pci: *mut pci_dev, pub spdif_ctl: *mut snd_kcontrol,
}

static channel_map_9652_ss: [i8; 26] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25];
static channel_map_9636_ss: [i8; 26] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 24, 25, -1, -1, -1, -1, -1, -1, -1, -1];
static channel_map_9652_ds: [i8; 26] = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 24, 25, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
static channel_map_9636_ds: [i8; 26] = [1, 3, 5, 7, 9, 11, 13, 15, 24, 25, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
static snd_rme9652_ids: [pci_device_id; 2] = [pci_device_id { vendor: 0x10ee, device: 0x3fc4, subvendor: PCI_ANY_ID, subdevice: PCI_ANY_ID }, pci_device_id { vendor: 0, device: 0, subvendor: 0, subdevice: 0 }];

unsafe extern "C" {
    static mut current: *mut task_struct;
    static KBUILD_MODNAME: c_char;
    fn snd_devm_alloc_pages(dev: *mut device, typ: c_int, size: usize) -> *mut snd_dma_buffer;
    fn writel(val: c_int, addr: *mut c_void); fn readl(addr: *mut c_void) -> c_uint; fn udelay(usecs: c_ulong);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...); fn snd_kcontrol_chip(k: *mut snd_kcontrol) -> *mut c_void;
    fn snd_ctl_enum_info(uinfo: *mut snd_ctl_elem_info, channels: c_uint, items: c_uint, names: *const *const c_char) -> c_int;
    fn snd_ctl_boolean_mono_info(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_info) -> c_int;
    fn snd_ctl_new1(n: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol; fn snd_ctl_add(card: *mut snd_card, k: *mut snd_kcontrol) -> c_int;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...); fn str_yes_no(v: c_int) -> *const c_char;
    fn snd_card_ro_proc_new(card: *mut snd_card, name: *const c_char, data: *mut c_void, read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>);
    fn snd_pcm_period_elapsed(s: *mut snd_pcm_substream); fn snd_pcm_substream_chip(s: *mut snd_pcm_substream) -> *mut c_void;
    fn copy_from_iter(dst: *mut i8, bytes: c_ulong, src: *mut iov_iter) -> c_ulong; fn copy_to_iter(src: *const i8, bytes: c_ulong, dst: *mut iov_iter) -> c_ulong;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void; fn snd_BUG_ON(cond: bool) -> c_int; fn snd_BUG();
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint; fn params_period_size(params: *mut snd_pcm_hw_params) -> c_ulong;
    fn _snd_pcm_hw_param_setempty(params: *mut snd_pcm_hw_params, var: c_int); fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn snd_interval_list(i: *mut snd_interval, count: c_uint, list: *const c_uint, mask: c_uint) -> c_int; fn snd_interval_refine(i: *mut snd_interval, v: *const snd_interval) -> c_int;
    fn snd_pcm_set_sync(s: *mut snd_pcm_substream); fn snd_pcm_set_runtime_buffer(s: *mut snd_pcm_substream, b: *mut snd_dma_buffer);
    fn snd_pcm_hw_constraint_msbits(r: *mut snd_pcm_runtime, a: c_uint, b: c_uint, c: c_uint) -> c_int; fn snd_pcm_hw_constraint_list(r: *mut snd_pcm_runtime, a: c_uint, p: c_int, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn snd_pcm_hw_rule_add(r: *mut snd_pcm_runtime, cond: c_uint, var: c_int, func: unsafe extern "C" fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> c_int, private: *mut c_void, dep: c_int, term: c_int) -> c_int;
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_ctl_elem_id); fn snd_pcm_trigger_done(s: *mut snd_pcm_substream, master: *mut snd_pcm_substream);
    fn snd_pcm_lib_ioctl(s: *mut snd_pcm_substream, cmd: c_uint, arg: *mut c_void) -> c_int; fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback: c_int, capture: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops); fn strscpy(dst: *mut c_char, src: *const c_char) -> c_int; fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn pci_read_config_word(pci: *mut pci_dev, where_: c_int, val: *mut u16) -> c_int; fn pcim_enable_device(pci: *mut pci_dev) -> c_int; fn spin_lock_init(lock: *mut spinlock_t);
    fn pcim_request_all_regions(pci: *mut pci_dev, name: *const c_char) -> c_int; fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong; fn devm_ioremap(dev: *mut device, start: c_ulong, len: c_ulong) -> *mut c_void;
    fn devm_request_irq(dev: *mut device, irq: c_int, h: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, data: *mut c_void) -> c_int;
    fn pci_set_master(pci: *mut pci_dev); fn snd_devm_card_new(dev: *mut device, idx: c_int, id: *mut c_char, module: *mut c_void, extra: usize, card: *mut *mut snd_card) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int; fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void); fn snd_card_free(card: *mut snd_card);
    fn array_index_nospec(index: c_uint, size: c_int) -> c_uint;
}

#[repr(C)] pub struct snd_pcm_hw_constraint_list { pub count: c_uint, pub list: *const c_uint, pub mask: c_uint }
#[repr(C)] pub struct snd_pcm_ops { pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>, pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>, pub ioctl: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_uint, *mut c_void) -> c_int>, pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>, pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>, pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>, pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>, pub copy: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, c_ulong, *mut iov_iter, c_ulong) -> c_int>, pub fill_silence: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, c_ulong, c_ulong) -> c_int> }

unsafe fn ALIGN(x: c_ulong, a: c_ulong) -> c_ulong { (x + a - 1) & !(a - 1) }
unsafe fn booli(v: bool) -> c_int { if v { 1 } else { 0 } }

unsafe extern "C" fn snd_hammerfall_get_buffer(pci: *mut pci_dev, size: size_t) -> *mut snd_dma_buffer { snd_devm_alloc_pages(&mut (*pci).dev, SNDRV_DMA_TYPE_DEV, size) }
unsafe fn rme9652_write(rme9652: *mut snd_rme9652, reg: c_int, val: c_int) { writel(val, ((*rme9652).iobase as *mut u8).add(reg as usize) as *mut c_void); }
unsafe fn rme9652_read(rme9652: *mut snd_rme9652, reg: c_int) -> c_uint { readl(((*rme9652).iobase as *mut u8).add(reg as usize) as *mut c_void) }
unsafe fn snd_rme9652_use_is_exclusive(rme9652: *mut snd_rme9652) -> c_int { if (*rme9652).playback_pid != (*rme9652).capture_pid && (*rme9652).playback_pid >= 0 && (*rme9652).capture_pid >= 0 { 0 } else { 1 } }
unsafe fn rme9652_adat_sample_rate(rme9652: *mut snd_rme9652) -> c_int { if rme9652_running_double_speed(rme9652) != 0 { if rme9652_read(rme9652, RME9652_status_register) & RME9652_fs48 != 0 { 96000 } else { 88200 } } else if rme9652_read(rme9652, RME9652_status_register) & RME9652_fs48 != 0 { 48000 } else { 44100 } }
unsafe fn rme9652_compute_period_size(rme9652: *mut snd_rme9652) { let i = (*rme9652).control_register & RME9652_latency; (*rme9652).period_bytes = 1usize << (rme9652_decode_latency(i) + 8); (*rme9652).hw_offsetmask = ((*rme9652).period_bytes * 2 - 1) & RME9652_buf_pos as usize; (*rme9652).max_jitter = 80; }
unsafe fn rme9652_hw_pointer(rme9652: *mut snd_rme9652) -> snd_pcm_uframes_t { let status = rme9652_read(rme9652, RME9652_status_register); let period_size = ((*rme9652).period_bytes / 4) as snd_pcm_uframes_t; if (*rme9652).precise_ptr == 0 { return if status & RME9652_buffer_id != 0 { period_size } else { 0 }; } let mut offset = status & RME9652_buf_pos; let mut delta = ((*rme9652).prev_hw_offset as snd_pcm_sframes_t - offset as snd_pcm_sframes_t) & 0xffff; if delta <= ((*rme9652).max_jitter as snd_pcm_sframes_t) * 4 { offset = (*rme9652).prev_hw_offset as u32; } else { (*rme9652).prev_hw_offset = offset as usize; } offset &= (*rme9652).hw_offsetmask as u32; offset /= 4; let frag = status & RME9652_buffer_id; if (offset as c_ulong) < period_size { if offset as usize > (*rme9652).max_jitter { if frag != 0 { dev_err((*(*rme9652).card).dev, c"Unexpected hw_pointer position (bufid == 0): status: %x offset: %d\n".as_ptr(), status, offset); } } else if frag == 0 { return 0; } offset = offset.wrapping_sub((*rme9652).max_jitter as u32); if (offset as c_int) < 0 { offset = offset.wrapping_add((period_size * 2) as u32); } } else { if offset as c_ulong > period_size + (*rme9652).max_jitter as c_ulong { if frag == 0 { dev_err((*(*rme9652).card).dev, c"Unexpected hw_pointer position (bufid == 1): status: %x offset: %d\n".as_ptr(), status, offset); } } else if frag != 0 { return period_size; } offset = offset.wrapping_sub((*rme9652).max_jitter as u32); } offset as snd_pcm_uframes_t }
unsafe fn rme9652_reset_hw_pointer(rme9652: *mut snd_rme9652) { for i in 0..8 { rme9652_write(rme9652, i * 4, 0); udelay(10); } (*rme9652).prev_hw_offset = 0; }
unsafe fn rme9652_start(s: *mut snd_rme9652) { (*s).control_register |= RME9652_IE | RME9652_start_bit; rme9652_write(s, RME9652_control_register, (*s).control_register as c_int); }
unsafe fn rme9652_stop(s: *mut snd_rme9652) { (*s).control_register &= !(RME9652_start_bit | RME9652_IE); rme9652_write(s, RME9652_control_register, (*s).control_register as c_int); }
unsafe extern "C" fn rme9652_set_interrupt_interval(s: *mut snd_rme9652, mut frames: c_uint) -> c_int { let restart = (*s).running; if restart != 0 { rme9652_stop(s); } frames >>= 7; let mut n = 0; while frames != 0 { n += 1; frames >>= 1; } (*s).control_register &= !RME9652_latency; (*s).control_register |= rme9652_encode_latency(n); rme9652_write(s, RME9652_control_register, (*s).control_register as c_int); rme9652_compute_period_size(s); if restart != 0 { rme9652_start(s); } 0 }
unsafe extern "C" fn rme9652_set_rate(rme9652: *mut snd_rme9652, mut rate: c_int) -> c_int { let mut reject_if_open = 0; if snd_rme9652_use_is_exclusive(rme9652) == 0 { return -EBUSY; } let xrate = rme9652_adat_sample_rate(rme9652); match rate { 44100 => { if xrate > 48000 { reject_if_open = 1; } rate = 0; }, 48000 => { if xrate > 48000 { reject_if_open = 1; } rate = RME9652_freq as c_int; }, 88200 => { if xrate < 48000 { reject_if_open = 1; } rate = RME9652_DS as c_int; }, 96000 => { if xrate < 48000 { reject_if_open = 1; } rate = (RME9652_DS | RME9652_freq) as c_int; }, _ => return -EINVAL } if reject_if_open != 0 && ((*rme9652).capture_pid >= 0 || (*rme9652).playback_pid >= 0) { return -EBUSY; } let restart = (*rme9652).running; if restart != 0 { rme9652_stop(rme9652); } (*rme9652).control_register &= !(RME9652_freq | RME9652_DS); (*rme9652).control_register |= rate as u32; rme9652_write(rme9652, RME9652_control_register, (*rme9652).control_register as c_int); if restart != 0 { rme9652_start(rme9652); } if rate as u32 & RME9652_DS != 0 { (*rme9652).channel_map = if (*rme9652).ss_channels as c_int == RME9652_NCHANNELS { channel_map_9652_ds.as_ptr() } else { channel_map_9636_ds.as_ptr() }; } else { (*rme9652).channel_map = if (*rme9652).ss_channels as c_int == RME9652_NCHANNELS { channel_map_9652_ss.as_ptr() } else { channel_map_9636_ss.as_ptr() }; } 0 }
unsafe fn rme9652_set_thru(rme9652: *mut snd_rme9652, channel: c_int, enable_: c_int) { (*rme9652).passthru = 0; if channel < 0 { for i in 0..RME9652_NCHANNELS { if enable_ != 0 { (*rme9652).thru_bits |= 1u32 << i; rme9652_write(rme9652, RME9652_thru_base + i * 4, 1); } else { (*rme9652).thru_bits &= !(1u32 << i); rme9652_write(rme9652, RME9652_thru_base + i * 4, 0); } } } else { let mapped_channel = *(*rme9652).channel_map.add(channel as usize) as c_int; if enable_ != 0 { (*rme9652).thru_bits |= 1u32 << mapped_channel; } else { (*rme9652).thru_bits &= !(1u32 << mapped_channel); } rme9652_write(rme9652, RME9652_thru_base + mapped_channel * 4, if enable_ != 0 { 1 } else { 0 }); } }
unsafe extern "C" fn rme9652_set_passthru(rme9652: *mut snd_rme9652, onoff: c_int) -> c_int { if onoff != 0 { rme9652_set_thru(rme9652, -1, 1); (*rme9652).control_register = RME9652_inp_0 | rme9652_encode_latency(7) | RME9652_start_bit; rme9652_reset_hw_pointer(rme9652); rme9652_write(rme9652, RME9652_control_register, (*rme9652).control_register as c_int); (*rme9652).passthru = 1; } else { rme9652_set_thru(rme9652, -1, 0); rme9652_stop(rme9652); (*rme9652).passthru = 0; } 0 }
unsafe fn rme9652_spdif_set_bit(rme9652: *mut snd_rme9652, mask: c_int, onoff: c_int) { if onoff != 0 { (*rme9652).control_register |= mask as u32; } else { (*rme9652).control_register &= !(mask as u32); } rme9652_write(rme9652, RME9652_control_register, (*rme9652).control_register as c_int); }
unsafe fn rme9652_spdif_write_byte(rme9652: *mut snd_rme9652, val: c_int) { let mut mask: c_long = 0x80; for _i in 0..8 { rme9652_spdif_set_bit(rme9652, RME9652_SPDIF_WRITE as c_int, if val as c_long & mask != 0 { 1 } else { 0 }); rme9652_spdif_set_bit(rme9652, RME9652_SPDIF_CLOCK as c_int, 1); rme9652_spdif_set_bit(rme9652, RME9652_SPDIF_CLOCK as c_int, 0); mask >>= 1; } }
unsafe fn rme9652_spdif_read_byte(rme9652: *mut snd_rme9652) -> c_int { let mut val: c_long = 0; let mut mask: c_long = 0x80; for _i in 0..8 { rme9652_spdif_set_bit(rme9652, RME9652_SPDIF_CLOCK as c_int, 1); if rme9652_read(rme9652, RME9652_status_register) & RME9652_SPDIF_READ != 0 { val |= mask; } rme9652_spdif_set_bit(rme9652, RME9652_SPDIF_CLOCK as c_int, 0); mask >>= 1; } val as c_int }
unsafe fn rme9652_write_spdif_codec(rme9652: *mut snd_rme9652, address: c_int, data: c_int) { rme9652_spdif_set_bit(rme9652, RME9652_SPDIF_SELECT as c_int, 1); rme9652_spdif_write_byte(rme9652, 0x20); rme9652_spdif_write_byte(rme9652, address); rme9652_spdif_write_byte(rme9652, data); rme9652_spdif_set_bit(rme9652, RME9652_SPDIF_SELECT as c_int, 0); }
unsafe fn rme9652_spdif_read_codec(rme9652: *mut snd_rme9652, address: c_int) -> c_int { rme9652_spdif_set_bit(rme9652, RME9652_SPDIF_SELECT as c_int, 1); rme9652_spdif_write_byte(rme9652, 0x20); rme9652_spdif_write_byte(rme9652, address); rme9652_spdif_set_bit(rme9652, RME9652_SPDIF_SELECT as c_int, 0); rme9652_spdif_set_bit(rme9652, RME9652_SPDIF_SELECT as c_int, 1); rme9652_spdif_write_byte(rme9652, 0x21); let ret = rme9652_spdif_read_byte(rme9652); rme9652_spdif_set_bit(rme9652, RME9652_SPDIF_SELECT as c_int, 0); ret }
unsafe fn rme9652_initialize_spdif_receiver(rme9652: *mut snd_rme9652) { (*rme9652).control_register |= RME9652_SPDIF_RESET; rme9652_write_spdif_codec(rme9652, 4, 0x40); rme9652_write_spdif_codec(rme9652, 17, 0x13); rme9652_write_spdif_codec(rme9652, 6, 0x02); }
unsafe fn rme9652_spdif_sample_rate(s: *mut snd_rme9652) -> c_int { if rme9652_read(s, RME9652_status_register) & RME9652_ERF != 0 { return -1; } if (*s).hw_rev == 15 { let x = rme9652_spdif_read_codec(s, 30); let y = if x != 0 { 48000 * 64 / x } else { 0 }; return if y > 30400 && y < 33600 { 32000 } else if y > 41900 && y < 46000 { 44100 } else if y > 46000 && y < 50400 { 48000 } else if y > 60800 && y < 67200 { 64000 } else if y > 83700 && y < 92000 { 88200 } else if y > 92000 && y < 100000 { 96000 } else { 0 }; } let rate_bits = rme9652_read(s, RME9652_status_register) & RME9652_F; match rme9652_decode_spdif_rate(rate_bits) { 0x7 => 32000, 0x6 => 44100, 0x5 => 48000, 0x4 => 88200, 0x3 => 96000, 0x0 => 64000, _ => { dev_err((*(*s).card).dev, c"%s: unknown S/PDIF input rate (bits = 0x%x)\n".as_ptr(), (*s).card_name, rate_bits); 0 } } }

unsafe fn snd_rme9652_convert_from_aes(aes: *mut snd_aes_iec958) -> u32 { let mut val = 0; val |= if (*aes).status[0] & IEC958_AES0_PROFESSIONAL != 0 { RME9652_PRO } else { 0 }; val |= if (*aes).status[0] & IEC958_AES0_NONAUDIO != 0 { RME9652_Dolby } else { 0 }; if val & RME9652_PRO != 0 { val |= if (*aes).status[0] & IEC958_AES0_PRO_EMPHASIS_5015 != 0 { RME9652_EMP } else { 0 }; } else { val |= if (*aes).status[0] & IEC958_AES0_CON_EMPHASIS_5015 != 0 { RME9652_EMP } else { 0 }; } val }
unsafe fn snd_rme9652_convert_to_aes(aes: *mut snd_aes_iec958, val: u32) { (*aes).status[0] = (if val & RME9652_PRO != 0 { IEC958_AES0_PROFESSIONAL } else { 0 }) | (if val & RME9652_Dolby != 0 { IEC958_AES0_NONAUDIO } else { 0 }); if val & RME9652_PRO != 0 { (*aes).status[0] |= if val & RME9652_EMP != 0 { IEC958_AES0_PRO_EMPHASIS_5015 } else { 0 }; } else { (*aes).status[0] |= if val & RME9652_EMP != 0 { IEC958_AES0_CON_EMPHASIS_5015 } else { 0 }; } }
unsafe extern "C" fn snd_rme9652_control_spdif_info(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_info) -> c_int { (*u).type_ = SNDRV_CTL_ELEM_TYPE_IEC958; (*u).count = 1; 0 }
unsafe extern "C" fn snd_rme9652_control_spdif_get(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let r = snd_kcontrol_chip(k) as *mut snd_rme9652; let aes = &mut (*u).value.iec958 as *mut _ as *mut snd_aes_iec958; snd_rme9652_convert_to_aes(aes, (*r).creg_spdif); 0 }
unsafe extern "C" fn snd_rme9652_control_spdif_put(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let r = snd_kcontrol_chip(k) as *mut snd_rme9652; let aes = &mut (*u).value.iec958 as *mut _ as *mut snd_aes_iec958; let val = snd_rme9652_convert_from_aes(aes); let change = booli(val != (*r).creg_spdif); (*r).creg_spdif = val; change }
unsafe extern "C" fn snd_rme9652_control_spdif_stream_info(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_info) -> c_int { snd_rme9652_control_spdif_info(k, u) }
unsafe extern "C" fn snd_rme9652_control_spdif_stream_get(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let r = snd_kcontrol_chip(k) as *mut snd_rme9652; let aes = &mut (*u).value.iec958 as *mut _ as *mut snd_aes_iec958; snd_rme9652_convert_to_aes(aes, (*r).creg_spdif_stream); 0 }
unsafe extern "C" fn snd_rme9652_control_spdif_stream_put(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let r = snd_kcontrol_chip(k) as *mut snd_rme9652; let aes = &mut (*u).value.iec958 as *mut _ as *mut snd_aes_iec958; let val = snd_rme9652_convert_from_aes(aes); let change = booli(val != (*r).creg_spdif_stream); (*r).creg_spdif_stream = val; (*r).control_register &= !(RME9652_PRO | RME9652_Dolby | RME9652_EMP); (*r).control_register |= val; rme9652_write(r, RME9652_control_register, (*r).control_register as c_int); change }
unsafe extern "C" fn snd_rme9652_control_spdif_mask_info(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_info) -> c_int { snd_rme9652_control_spdif_info(k, u) }
unsafe extern "C" fn snd_rme9652_control_spdif_mask_get(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let aes = &mut (*u).value.iec958 as *mut _ as *mut snd_aes_iec958; (*aes).status[0] = (*k).private_value as u8; 0 }

unsafe fn rme9652_adat1_in(r: *mut snd_rme9652) -> c_uint { if (*r).control_register & RME9652_ADAT1_INTERNAL != 0 { 1 } else { 0 } }
unsafe fn rme9652_set_adat1_input(r: *mut snd_rme9652, internal: c_int) -> c_int { if internal != 0 { (*r).control_register |= RME9652_ADAT1_INTERNAL; } else { (*r).control_register &= !RME9652_ADAT1_INTERNAL; } let restart = (*r).running; if restart != 0 { rme9652_stop(r); } rme9652_write(r, RME9652_control_register, (*r).control_register as c_int); if restart != 0 { rme9652_start(r); } 0 }
unsafe extern "C" fn snd_rme9652_info_adat1_in(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_info) -> c_int { let texts = [c"ADAT1".as_ptr(), c"Internal".as_ptr()]; snd_ctl_enum_info(u, 1, 2, texts.as_ptr()) }
unsafe extern "C" fn snd_rme9652_get_adat1_in(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let r = snd_kcontrol_chip(k) as *mut snd_rme9652; (*(&mut (*u).value.enumerated as *mut _ as *mut snd_ctl_elem_value_enumerated)).item[0] = rme9652_adat1_in(r); 0 }
unsafe extern "C" fn snd_rme9652_put_adat1_in(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let r = snd_kcontrol_chip(k) as *mut snd_rme9652; if snd_rme9652_use_is_exclusive(r) == 0 { return -EBUSY; } let val = (*(&mut (*u).value.enumerated as *mut _ as *mut snd_ctl_elem_value_enumerated)).item[0] % 2; let change = booli(val != rme9652_adat1_in(r)); if change != 0 { rme9652_set_adat1_input(r, val as c_int); } change }
unsafe fn rme9652_spdif_in(r: *mut snd_rme9652) -> c_uint { rme9652_decode_spdif_in((*r).control_register & RME9652_inp) }
unsafe fn rme9652_set_spdif_input(r: *mut snd_rme9652, input: c_int) -> c_int { (*r).control_register &= !RME9652_inp; (*r).control_register |= rme9652_encode_spdif_in(input); let restart = (*r).running; if restart != 0 { rme9652_stop(r); } rme9652_write(r, RME9652_control_register, (*r).control_register as c_int); if restart != 0 { rme9652_start(r); } 0 }
unsafe extern "C" fn snd_rme9652_info_spdif_in(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_info) -> c_int { let texts = [c"ADAT1".as_ptr(), c"Coaxial".as_ptr(), c"Internal".as_ptr()]; snd_ctl_enum_info(u, 1, 3, texts.as_ptr()) }
unsafe extern "C" fn snd_rme9652_get_spdif_in(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let r = snd_kcontrol_chip(k) as *mut snd_rme9652; (*(&mut (*u).value.enumerated as *mut _ as *mut snd_ctl_elem_value_enumerated)).item[0] = rme9652_spdif_in(r); 0 }
unsafe extern "C" fn snd_rme9652_put_spdif_in(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let r = snd_kcontrol_chip(k) as *mut snd_rme9652; if snd_rme9652_use_is_exclusive(r) == 0 { return -EBUSY; } let val = (*(&mut (*u).value.enumerated as *mut _ as *mut snd_ctl_elem_value_enumerated)).item[0] % 3; let change = booli(val != rme9652_spdif_in(r)); if change != 0 { rme9652_set_spdif_input(r, val as c_int); } change }
unsafe fn rme9652_spdif_out(r: *mut snd_rme9652) -> c_int { if (*r).control_register & RME9652_opt_out != 0 { 1 } else { 0 } }
unsafe fn rme9652_set_spdif_output(r: *mut snd_rme9652, out: c_int) -> c_int { if out != 0 { (*r).control_register |= RME9652_opt_out; } else { (*r).control_register &= !RME9652_opt_out; } let restart = (*r).running; if restart != 0 { rme9652_stop(r); } rme9652_write(r, RME9652_control_register, (*r).control_register as c_int); if restart != 0 { rme9652_start(r); } 0 }
unsafe extern "C" fn snd_rme9652_get_spdif_out(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let r = snd_kcontrol_chip(k) as *mut snd_rme9652; (*(&mut (*u).value.integer as *mut _ as *mut snd_ctl_elem_value_integer)).value[0] = rme9652_spdif_out(r) as c_long; 0 }
unsafe extern "C" fn snd_rme9652_put_spdif_out(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let r = snd_kcontrol_chip(k) as *mut snd_rme9652; if snd_rme9652_use_is_exclusive(r) == 0 { return -EBUSY; } let val = ((*(&mut (*u).value.integer as *mut _ as *mut snd_ctl_elem_value_integer)).value[0] & 1) as c_uint; let change = booli(val as c_int != rme9652_spdif_out(r)); rme9652_set_spdif_output(r, val as c_int); change }
unsafe fn rme9652_sync_mode(r: *mut snd_rme9652) -> c_int { if (*r).control_register & RME9652_wsel != 0 { 2 } else if (*r).control_register & RME9652_Master != 0 { 1 } else { 0 } }
unsafe fn rme9652_set_sync_mode(r: *mut snd_rme9652, mode: c_int) -> c_int { match mode { 0 => (*r).control_register &= !(RME9652_Master | RME9652_wsel), 1 => (*r).control_register = ((*r).control_register & !RME9652_wsel) | RME9652_Master, 2 => (*r).control_register |= RME9652_Master | RME9652_wsel, _ => {} } let restart = (*r).running; if restart != 0 { rme9652_stop(r); } rme9652_write(r, RME9652_control_register, (*r).control_register as c_int); if restart != 0 { rme9652_start(r); } 0 }
unsafe extern "C" fn snd_rme9652_info_sync_mode(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_info) -> c_int { let texts = [c"AutoSync".as_ptr(), c"Master".as_ptr(), c"Word Clock".as_ptr()]; snd_ctl_enum_info(u, 1, 3, texts.as_ptr()) }
unsafe extern "C" fn snd_rme9652_get_sync_mode(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let r = snd_kcontrol_chip(k) as *mut snd_rme9652; (*(&mut (*u).value.enumerated as *mut _ as *mut snd_ctl_elem_value_enumerated)).item[0] = rme9652_sync_mode(r) as c_uint; 0 }
unsafe extern "C" fn snd_rme9652_put_sync_mode(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let r = snd_kcontrol_chip(k) as *mut snd_rme9652; let val = (*(&mut (*u).value.enumerated as *mut _ as *mut snd_ctl_elem_value_enumerated)).item[0] % 3; let change = booli(val as c_int != rme9652_sync_mode(r)); rme9652_set_sync_mode(r, val as c_int); change }
unsafe fn rme9652_sync_pref(r: *mut snd_rme9652) -> c_int { match (*r).control_register & RME9652_SyncPref_Mask { RME9652_SyncPref_ADAT1 => RME9652_SYNC_FROM_ADAT1, RME9652_SyncPref_ADAT2 => RME9652_SYNC_FROM_ADAT2, RME9652_SyncPref_ADAT3 => RME9652_SYNC_FROM_ADAT3, RME9652_SyncPref_SPDIF => RME9652_SYNC_FROM_SPDIF, _ => 0 } }
unsafe fn rme9652_set_sync_pref(r: *mut snd_rme9652, pref: c_int) -> c_int { (*r).control_register &= !RME9652_SyncPref_Mask; match pref { RME9652_SYNC_FROM_ADAT1 => (*r).control_register |= RME9652_SyncPref_ADAT1, RME9652_SYNC_FROM_ADAT2 => (*r).control_register |= RME9652_SyncPref_ADAT2, RME9652_SYNC_FROM_ADAT3 => (*r).control_register |= RME9652_SyncPref_ADAT3, RME9652_SYNC_FROM_SPDIF => (*r).control_register |= RME9652_SyncPref_SPDIF, _ => {} } let restart = (*r).running; if restart != 0 { rme9652_stop(r); } rme9652_write(r, RME9652_control_register, (*r).control_register as c_int); if restart != 0 { rme9652_start(r); } 0 }
unsafe extern "C" fn snd_rme9652_info_sync_pref(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_info) -> c_int { let r = snd_kcontrol_chip(k) as *mut snd_rme9652; let texts = [c"IEC958 In".as_ptr(), c"ADAT1 In".as_ptr(), c"ADAT2 In".as_ptr(), c"ADAT3 In".as_ptr()]; snd_ctl_enum_info(u, 1, if (*r).ss_channels as c_int == RME9652_NCHANNELS { 4 } else { 3 }, texts.as_ptr()) }
unsafe extern "C" fn snd_rme9652_get_sync_pref(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let r = snd_kcontrol_chip(k) as *mut snd_rme9652; (*(&mut (*u).value.enumerated as *mut _ as *mut snd_ctl_elem_value_enumerated)).item[0] = rme9652_sync_pref(r) as c_uint; 0 }
unsafe extern "C" fn snd_rme9652_put_sync_pref(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let r = snd_kcontrol_chip(k) as *mut snd_rme9652; if snd_rme9652_use_is_exclusive(r) == 0 { return -EBUSY; } let max = if (*r).ss_channels as c_int == RME9652_NCHANNELS { 4 } else { 3 }; let val = (*(&mut (*u).value.enumerated as *mut _ as *mut snd_ctl_elem_value_enumerated)).item[0] % max; let change = booli(val as c_int != rme9652_sync_pref(r)); rme9652_set_sync_pref(r, val as c_int); change }

/* ALSA control constructor macros RME9652_ADAT1_IN/SPDIF_IN/SPDIF_OUT/SYNC_MODE/
 * SYNC_PREF/PASSTHRU/SPDIF_RATE/ADAT_SYNC/TC_VALID are represented below by
 * snd_kcontrol_new initializers and the referenced callback functions.
 */

unsafe extern "C" fn snd_rme9652_info_thru(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_info) -> c_int { let r = snd_kcontrol_chip(k) as *mut snd_rme9652; (*u).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN; (*u).count = (*r).ss_channels as c_uint; (*(&mut (*u).value.integer as *mut _ as *mut snd_ctl_elem_info_integer)).min = 0; (*(&mut (*u).value.integer as *mut _ as *mut snd_ctl_elem_info_integer)).max = 1; 0 }
unsafe extern "C" fn snd_rme9652_get_thru(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let r = snd_kcontrol_chip(k) as *mut snd_rme9652; let thru_bits = (*r).thru_bits; for ch in 0..(*r).ss_channels as usize { (*(&mut (*u).value.integer as *mut _ as *mut snd_ctl_elem_value_integer)).value[ch] = if thru_bits & (1u32 << ch) != 0 { 1 } else { 0 }; } 0 }
unsafe extern "C" fn snd_rme9652_put_thru(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let r = snd_kcontrol_chip(k) as *mut snd_rme9652; if snd_rme9652_use_is_exclusive(r) == 0 { return -EBUSY; } let mut thru_bits = 0u32; for ch in 0..(*r).ss_channels as usize { if (*(&mut (*u).value.integer as *mut _ as *mut snd_ctl_elem_value_integer)).value[ch] != 0 { thru_bits |= 1u32 << ch; } } let change = thru_bits ^ (*r).thru_bits; if change != 0 { for ch in 0..(*r).ss_channels as c_int { if change & (1u32 << ch) != 0 { rme9652_set_thru(r, ch, (thru_bits & (1u32 << ch)) as c_int); } } } booli(change != 0) }
unsafe extern "C" fn snd_rme9652_get_passthru(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let r = snd_kcontrol_chip(k) as *mut snd_rme9652; (*(&mut (*u).value.integer as *mut _ as *mut snd_ctl_elem_value_integer)).value[0] = (*r).passthru as c_long; 0 }
unsafe extern "C" fn snd_rme9652_put_passthru(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let r = snd_kcontrol_chip(k) as *mut snd_rme9652; if snd_rme9652_use_is_exclusive(r) == 0 { return -EBUSY; } let val = ((*(&mut (*u).value.integer as *mut _ as *mut snd_ctl_elem_value_integer)).value[0] & 1) as c_uint; let change = booli((*(&mut (*u).value.integer as *mut _ as *mut snd_ctl_elem_value_integer)).value[0] as c_int != (*r).passthru); let mut err = 0; if change != 0 { err = rme9652_set_passthru(r, val as c_int); } if err != 0 { err } else { change } }
unsafe extern "C" fn snd_rme9652_info_spdif_rate(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_info) -> c_int { (*u).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER; (*u).count = 1; (*(&mut (*u).value.integer as *mut _ as *mut snd_ctl_elem_info_integer)).min = 0; (*(&mut (*u).value.integer as *mut _ as *mut snd_ctl_elem_info_integer)).max = 96000; 0 }
unsafe extern "C" fn snd_rme9652_get_spdif_rate(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let r = snd_kcontrol_chip(k) as *mut snd_rme9652; (*(&mut (*u).value.integer as *mut _ as *mut snd_ctl_elem_value_integer)).value[0] = rme9652_spdif_sample_rate(r) as c_long; 0 }
unsafe extern "C" fn snd_rme9652_info_adat_sync(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_info) -> c_int { let texts = [c"No Lock".as_ptr(), c"Lock".as_ptr(), c"No Lock Sync".as_ptr(), c"Lock Sync".as_ptr()]; snd_ctl_enum_info(u, 1, 4, texts.as_ptr()) }
unsafe extern "C" fn snd_rme9652_get_adat_sync(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let r = snd_kcontrol_chip(k) as *mut snd_rme9652; let (mask1, mask2) = match (*k).private_value { 0 => (RME9652_lock_0, RME9652_sync_0), 1 => (RME9652_lock_1, RME9652_sync_1), 2 => (RME9652_lock_2, RME9652_sync_2), _ => return -EINVAL }; let val = rme9652_read(r, RME9652_status_register); let e = &mut (*(*u).value.enumerated).item as *mut _; (*(&mut (*u).value.enumerated as *mut _ as *mut snd_ctl_elem_value_enumerated)).item[0] = if val & mask1 != 0 { 1 } else { 0 }; (*(&mut (*u).value.enumerated as *mut _ as *mut snd_ctl_elem_value_enumerated)).item[0] |= if val & mask2 != 0 { 2 } else { 0 }; 0 }
unsafe extern "C" fn snd_rme9652_get_tc_valid(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let r = snd_kcontrol_chip(k) as *mut snd_rme9652; (*(&mut (*u).value.integer as *mut _ as *mut snd_ctl_elem_value_integer)).value[0] = if rme9652_read(r, RME9652_status_register) & RME9652_tc_valid != 0 { 1 } else { 0 }; 0 }

/* Code inside #ifdef ALSA_HAS_STANDARD_WAY_OF_RETURNING_TIMECODE translated conditionally in intent. */
#[cfg(ALSA_HAS_STANDARD_WAY_OF_RETURNING_TIMECODE)]
unsafe extern "C" fn snd_rme9652_get_tc_value(private_data: *mut c_void, kswitch: *mut c_void, uswitch: *mut c_void) -> c_int { let s = private_data as *mut snd_rme9652; let mut value: u32; if rme9652_read(s, RME9652_status_register) & RME9652_tc_valid == 0 { return 0; } rme9652_write(s, RME9652_time_code, 0); let mut i = 0; while i < 50 { if rme9652_read(s, i * 4) & RME9652_tc_busy == 0 { break; } i += 1; } if rme9652_read(s, i * 4) & RME9652_tc_busy == 0 { return -EIO; } value = 0; for i in 0..32 { value >>= 1; if rme9652_read(s, i * 4) & RME9652_tc_out != 0 { value |= 0x80000000; } } if value > 2 * 60 * 48000 { value -= 2 * 60 * 48000; } else { value = 0; } 0 }

static snd_rme9652_controls: [snd_kcontrol_new; 14] = [
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_PCM, name: c"IEC958 Playback Default".as_ptr(), index: 0, access: 0, info: Some(snd_rme9652_control_spdif_info), get: Some(snd_rme9652_control_spdif_get), put: Some(snd_rme9652_control_spdif_put), private_value: 0 },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_PCM, name: c"IEC958 Playback PCM Stream".as_ptr(), index: 0, access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_INACTIVE, info: Some(snd_rme9652_control_spdif_stream_info), get: Some(snd_rme9652_control_spdif_stream_get), put: Some(snd_rme9652_control_spdif_stream_put), private_value: 0 },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_PCM, name: c"IEC958 Playback Con Mask".as_ptr(), index: 0, access: SNDRV_CTL_ELEM_ACCESS_READ, info: Some(snd_rme9652_control_spdif_mask_info), get: Some(snd_rme9652_control_spdif_mask_get), put: None, private_value: (IEC958_AES0_NONAUDIO | IEC958_AES0_PROFESSIONAL) as c_long | IEC958_AES0_CON_EMPHASIS },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_PCM, name: c"IEC958 Playback Pro Mask".as_ptr(), index: 0, access: SNDRV_CTL_ELEM_ACCESS_READ, info: Some(snd_rme9652_control_spdif_mask_info), get: Some(snd_rme9652_control_spdif_mask_get), put: None, private_value: (IEC958_AES0_NONAUDIO | IEC958_AES0_PROFESSIONAL) as c_long | IEC958_AES0_PRO_EMPHASIS },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"IEC958 Input Connector".as_ptr(), index: 0, access: 0, info: Some(snd_rme9652_info_spdif_in), get: Some(snd_rme9652_get_spdif_in), put: Some(snd_rme9652_put_spdif_in), private_value: 0 },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"IEC958 Output also on ADAT1".as_ptr(), index: 0, access: 0, info: Some(snd_ctl_boolean_mono_info), get: Some(snd_rme9652_get_spdif_out), put: Some(snd_rme9652_put_spdif_out), private_value: 0 },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"Sync Mode".as_ptr(), index: 0, access: 0, info: Some(snd_rme9652_info_sync_mode), get: Some(snd_rme9652_get_sync_mode), put: Some(snd_rme9652_put_sync_mode), private_value: 0 },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"Preferred Sync Source".as_ptr(), index: 0, access: 0, info: Some(snd_rme9652_info_sync_pref), get: Some(snd_rme9652_get_sync_pref), put: Some(snd_rme9652_put_sync_pref), private_value: 0 },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"Channels Thru".as_ptr(), index: 0, access: 0, info: Some(snd_rme9652_info_thru), get: Some(snd_rme9652_get_thru), put: Some(snd_rme9652_put_thru), private_value: 0 },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"IEC958 Sample Rate".as_ptr(), index: 0, access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE, info: Some(snd_rme9652_info_spdif_rate), get: Some(snd_rme9652_get_spdif_rate), put: None, private_value: 0 },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"ADAT1 Sync Check".as_ptr(), index: 0, access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE, info: Some(snd_rme9652_info_adat_sync), get: Some(snd_rme9652_get_adat_sync), put: None, private_value: 0 },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"ADAT2 Sync Check".as_ptr(), index: 0, access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE, info: Some(snd_rme9652_info_adat_sync), get: Some(snd_rme9652_get_adat_sync), put: None, private_value: 1 },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"Timecode Valid".as_ptr(), index: 0, access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE, info: Some(snd_ctl_boolean_mono_info), get: Some(snd_rme9652_get_tc_valid), put: None, private_value: 0 },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"Passthru".as_ptr(), index: 0, access: 0, info: Some(snd_ctl_boolean_mono_info), get: Some(snd_rme9652_get_passthru), put: Some(snd_rme9652_put_passthru), private_value: 0 },
];
static snd_rme9652_adat3_check: snd_kcontrol_new = snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"ADAT3 Sync Check".as_ptr(), index: 0, access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE, info: Some(snd_rme9652_info_adat_sync), get: Some(snd_rme9652_get_adat_sync), put: None, private_value: 2 };
static snd_rme9652_adat1_input: snd_kcontrol_new = snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"ADAT1 Input Source".as_ptr(), index: 0, access: 0, info: Some(snd_rme9652_info_adat1_in), get: Some(snd_rme9652_get_adat1_in), put: Some(snd_rme9652_put_adat1_in), private_value: 0 };

unsafe extern "C" fn snd_rme9652_create_controls(card: *mut snd_card, r: *mut snd_rme9652) -> c_int { for idx in 0..snd_rme9652_controls.len() { let kctl = snd_ctl_new1(&snd_rme9652_controls[idx], r as *mut c_void); let err = snd_ctl_add(card, kctl); if err < 0 { return err; } if idx == 1 { (*r).spdif_ctl = kctl; } } if (*r).ss_channels as c_int == RME9652_NCHANNELS { let kctl = snd_ctl_new1(&snd_rme9652_adat3_check, r as *mut c_void); let err = snd_ctl_add(card, kctl); if err < 0 { return err; } } if (*r).hw_rev >= 15 { let kctl = snd_ctl_new1(&snd_rme9652_adat1_input, r as *mut c_void); let err = snd_ctl_add(card, kctl); if err < 0 { return err; } } 0 }

unsafe extern "C" fn snd_rme9652_proc_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) { let r = (*entry).private_data as *mut snd_rme9652; let thru_bits = (*r).thru_bits; let mut show_auto_sync_source = 0; let status = rme9652_read(r, RME9652_status_register); snd_iprintf(buffer, c"%s (Card #%d)\n".as_ptr(), (*r).card_name, (*(*r).card).number + 1); snd_iprintf(buffer, c"Buffers: capture %p playback %p\n".as_ptr(), (*r).capture_buffer, (*r).playback_buffer); snd_iprintf(buffer, c"IRQ: %d Registers bus: 0x%lx VM: 0x%lx\n".as_ptr(), (*r).irq, (*r).port, (*r).iobase as c_ulong); snd_iprintf(buffer, c"Control register: %x\n\n".as_ptr(), (*r).control_register); let xlat = 1 << (6 + rme9652_decode_latency((*r).control_register & RME9652_latency)); snd_iprintf(buffer, c"Latency: %d samples (2 periods of %lu bytes)\n".as_ptr(), xlat, (*r).period_bytes as c_ulong); snd_iprintf(buffer, c"Hardware pointer (frames): %ld\n".as_ptr(), rme9652_hw_pointer(r)); snd_iprintf(buffer, c"Passthru: %s\n".as_ptr(), str_yes_no((*r).passthru)); if (*r).control_register & (RME9652_Master | RME9652_wsel) == 0 { snd_iprintf(buffer, c"Clock mode: autosync\n".as_ptr()); show_auto_sync_source = 1; } else if (*r).control_register & RME9652_wsel != 0 { snd_iprintf(buffer, if status & RME9652_wsel_rd != 0 { c"Clock mode: word clock\n".as_ptr() } else { c"Clock mode: word clock (no signal)\n".as_ptr() }); } else { snd_iprintf(buffer, c"Clock mode: master\n".as_ptr()); } if show_auto_sync_source != 0 { match (*r).control_register & RME9652_SyncPref_Mask { RME9652_SyncPref_ADAT1 => snd_iprintf(buffer, c"Pref. sync source: ADAT1\n".as_ptr()), RME9652_SyncPref_ADAT2 => snd_iprintf(buffer, c"Pref. sync source: ADAT2\n".as_ptr()), RME9652_SyncPref_ADAT3 => snd_iprintf(buffer, c"Pref. sync source: ADAT3\n".as_ptr()), RME9652_SyncPref_SPDIF => snd_iprintf(buffer, c"Pref. sync source: IEC958\n".as_ptr()), _ => snd_iprintf(buffer, c"Pref. sync source: ???\n".as_ptr()) } } if (*r).hw_rev >= 15 { snd_iprintf(buffer, c"\nADAT1 Input source: %s\n".as_ptr(), if (*r).control_register & RME9652_ADAT1_INTERNAL != 0 { c"Internal".as_ptr() } else { c"ADAT1 optical".as_ptr() }); } snd_iprintf(buffer, c"\n".as_ptr()); match rme9652_decode_spdif_in((*r).control_register & RME9652_inp) as c_int { RME9652_SPDIFIN_OPTICAL => snd_iprintf(buffer, c"IEC958 input: ADAT1\n".as_ptr()), RME9652_SPDIFIN_COAXIAL => snd_iprintf(buffer, c"IEC958 input: Coaxial\n".as_ptr()), RME9652_SPDIFIN_INTERN => snd_iprintf(buffer, c"IEC958 input: Internal\n".as_ptr()), _ => snd_iprintf(buffer, c"IEC958 input: ???\n".as_ptr()) } snd_iprintf(buffer, if (*r).control_register & RME9652_opt_out != 0 { c"IEC958 output: Coaxial & ADAT1\n".as_ptr() } else { c"IEC958 output: Coaxial only\n".as_ptr() }); snd_iprintf(buffer, if (*r).control_register & RME9652_PRO != 0 { c"IEC958 quality: Professional\n".as_ptr() } else { c"IEC958 quality: Consumer\n".as_ptr() }); snd_iprintf(buffer, if (*r).control_register & RME9652_EMP != 0 { c"IEC958 emphasis: on\n".as_ptr() } else { c"IEC958 emphasis: off\n".as_ptr() }); snd_iprintf(buffer, if (*r).control_register & RME9652_Dolby != 0 { c"IEC958 Dolby: on\n".as_ptr() } else { c"IEC958 Dolby: off\n".as_ptr() }); let i = rme9652_spdif_sample_rate(r); if i < 0 { snd_iprintf(buffer, c"IEC958 sample rate: error flag set\n".as_ptr()); } else if i == 0 { snd_iprintf(buffer, c"IEC958 sample rate: undetermined\n".as_ptr()); } else { snd_iprintf(buffer, c"IEC958 sample rate: %d\n".as_ptr(), i); } snd_iprintf(buffer, c"\nADAT Sample rate: %dHz\n".as_ptr(), rme9652_adat_sample_rate(r)); let syncs = [(RME9652_sync_0, RME9652_lock_0, c"ADAT1: %s\n".as_ptr()), (RME9652_sync_1, RME9652_lock_1, c"ADAT2: %s\n".as_ptr()), (RME9652_sync_2, RME9652_lock_2, c"ADAT3: %s\n".as_ptr())]; for (sm,lm,fmt) in syncs { if status & lm != 0 { snd_iprintf(buffer, fmt, if status & sm != 0 { c"Sync".as_ptr() } else { c"Lock".as_ptr() }); } else { snd_iprintf(buffer, fmt, c"No Lock".as_ptr()); } } snd_iprintf(buffer, c"\nTimecode signal: %s\nPunch Status:\n\n".as_ptr(), str_yes_no((status & RME9652_tc_valid) as c_int)); for i in 0..(*r).ss_channels as c_int { snd_iprintf(buffer, if thru_bits & (1u32 << i) != 0 { c"%2d:  on ".as_ptr() } else { c"%2d: off ".as_ptr() }, i + 1); if ((i + 1) % 8) == 0 { snd_iprintf(buffer, c"\n".as_ptr()); } } snd_iprintf(buffer, c"\n".as_ptr()); }
unsafe fn snd_rme9652_proc_init(r: *mut snd_rme9652) { snd_card_ro_proc_new((*r).card, c"rme9652".as_ptr(), r as *mut c_void, Some(snd_rme9652_proc_read)); }
unsafe extern "C" fn snd_rme9652_card_free(card: *mut snd_card) { let r = (*card).private_data as *mut snd_rme9652; if (*r).irq >= 0 { rme9652_stop(r); } }
unsafe extern "C" fn snd_rme9652_initialize_memory(r: *mut snd_rme9652) -> c_int { let capture_dma = snd_hammerfall_get_buffer((*r).pci, RME9652_DMA_AREA_BYTES); let playback_dma = snd_hammerfall_get_buffer((*r).pci, RME9652_DMA_AREA_BYTES); if capture_dma.is_null() || playback_dma.is_null() { dev_err((*(*r).card).dev, c"%s: no buffers available\n".as_ptr(), (*r).card_name); return -ENOMEM; } (*r).capture_dma_buf = *capture_dma; (*r).playback_dma_buf = *playback_dma; (*r).capture_dma_buf.addr = ALIGN((*capture_dma).addr, 0x10000); (*r).playback_dma_buf.addr = ALIGN((*playback_dma).addr, 0x10000); rme9652_write(r, RME9652_rec_buffer, (*r).capture_dma_buf.addr as c_int); rme9652_write(r, RME9652_play_buffer, (*r).playback_dma_buf.addr as c_int); (*r).capture_dma_buf.area = (*r).capture_dma_buf.area.add(((*r).capture_dma_buf.addr - (*capture_dma).addr) as usize); (*r).playback_dma_buf.area = (*r).playback_dma_buf.area.add(((*r).playback_dma_buf.addr - (*playback_dma).addr) as usize); (*r).capture_buffer = (*r).capture_dma_buf.area as *mut u8; (*r).playback_buffer = (*r).playback_dma_buf.area as *mut u8; 0 }
unsafe fn snd_rme9652_set_defaults(r: *mut snd_rme9652) { (*r).control_register = RME9652_inp_0 | rme9652_encode_latency(7); rme9652_write(r, RME9652_control_register, (*r).control_register as c_int); rme9652_reset_hw_pointer(r); rme9652_compute_period_size(r); for k in 0..RME9652_NCHANNELS { rme9652_write(r, RME9652_thru_base + k * 4, 0); } (*r).thru_bits = 0; (*r).passthru = 0; rme9652_set_rate(r, 48000); }
unsafe extern "C" fn snd_rme9652_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t { let r = dev_id as *mut snd_rme9652; if rme9652_read(r, RME9652_status_register) & RME9652_IRQ == 0 { return IRQ_NONE; } rme9652_write(r, RME9652_irq_clear, 0); if !(*r).capture_substream.is_null() { snd_pcm_period_elapsed((*(*r).pcm).streams[SNDRV_PCM_STREAM_CAPTURE as usize].substream); } if !(*r).playback_substream.is_null() { snd_pcm_period_elapsed((*(*r).pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].substream); } IRQ_HANDLED }
unsafe extern "C" fn snd_rme9652_hw_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t { rme9652_hw_pointer(snd_pcm_substream_chip(substream) as *mut snd_rme9652) }
unsafe fn rme9652_channel_buffer_location(r: *mut snd_rme9652, stream: c_int, channel: c_int) -> *mut i8 { if snd_BUG_ON(channel < 0 || channel >= RME9652_NCHANNELS) != 0 { return ptr::null_mut(); } let mapped_channel = *(*r).channel_map.add(channel as usize) as c_int; if mapped_channel < 0 { return ptr::null_mut(); } if stream == SNDRV_PCM_STREAM_CAPTURE { (*r).capture_buffer.add(mapped_channel as usize * RME9652_CHANNEL_BUFFER_BYTES) as *mut i8 } else { (*r).playback_buffer.add(mapped_channel as usize * RME9652_CHANNEL_BUFFER_BYTES) as *mut i8 } }
unsafe extern "C" fn snd_rme9652_playback_copy(substream: *mut snd_pcm_substream, channel: c_int, pos: c_ulong, src: *mut iov_iter, count: c_ulong) -> c_int { let r = snd_pcm_substream_chip(substream) as *mut snd_rme9652; if snd_BUG_ON(pos + count > RME9652_CHANNEL_BUFFER_BYTES as c_ulong) != 0 { return -EINVAL; } let channel_buf = rme9652_channel_buffer_location(r, (*(*substream).pstr).stream, channel); if snd_BUG_ON(channel_buf.is_null()) != 0 { return -EIO; } if copy_from_iter(channel_buf.add(pos as usize), count, src) != count { return -EFAULT; } 0 }
unsafe extern "C" fn snd_rme9652_capture_copy(substream: *mut snd_pcm_substream, channel: c_int, pos: c_ulong, dst: *mut iov_iter, count: c_ulong) -> c_int { let r = snd_pcm_substream_chip(substream) as *mut snd_rme9652; if snd_BUG_ON(pos + count > RME9652_CHANNEL_BUFFER_BYTES as c_ulong) != 0 { return -EINVAL; } let channel_buf = rme9652_channel_buffer_location(r, (*(*substream).pstr).stream, channel); if snd_BUG_ON(channel_buf.is_null()) != 0 { return -EIO; } if copy_to_iter(channel_buf.add(pos as usize), count, dst) != count { return -EFAULT; } 0 }
unsafe extern "C" fn snd_rme9652_hw_silence(substream: *mut snd_pcm_substream, channel: c_int, pos: c_ulong, count: c_ulong) -> c_int { let r = snd_pcm_substream_chip(substream) as *mut snd_rme9652; let channel_buf = rme9652_channel_buffer_location(r, (*(*substream).pstr).stream, channel); if snd_BUG_ON(channel_buf.is_null()) != 0 { return -EIO; } memset(channel_buf.add(pos as usize) as *mut c_void, 0, count as usize); 0 }
unsafe extern "C" fn snd_rme9652_reset(substream: *mut snd_pcm_substream) -> c_int { let runtime = (*substream).runtime; let r = snd_pcm_substream_chip(substream) as *mut snd_rme9652; let other = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK { (*r).capture_substream } else { (*r).playback_substream }; (*(*runtime).status).hw_ptr = if (*r).running != 0 { rme9652_hw_pointer(r) } else { 0 }; if !other.is_null() { (*(*(*other).runtime).status).hw_ptr = (*(*runtime).status).hw_ptr; } 0 }
unsafe extern "C" fn snd_rme9652_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int { let r = snd_pcm_substream_chip(substream) as *mut snd_rme9652; let (this_pid, other_pid) = if (*(*substream).pstr).stream == SNDRV_PCM_STREAM_PLAYBACK { (*r).control_register &= !(RME9652_PRO | RME9652_Dolby | RME9652_EMP); (*r).control_register |= (*r).creg_spdif_stream; rme9652_write(r, RME9652_control_register, (*r).control_register as c_int); ((*r).playback_pid, (*r).capture_pid) } else { ((*r).capture_pid, (*r).playback_pid) }; if other_pid > 0 && this_pid != other_pid { if params_rate(params) as c_int != rme9652_adat_sample_rate(r) { _snd_pcm_hw_param_setempty(params, SNDRV_PCM_HW_PARAM_RATE); return -EBUSY; } if params_period_size(params) != ((*r).period_bytes / 4) as c_ulong { _snd_pcm_hw_param_setempty(params, SNDRV_PCM_HW_PARAM_PERIOD_SIZE); return -EBUSY; } return 0; } let mut err = rme9652_set_rate(r, params_rate(params) as c_int); if err < 0 { _snd_pcm_hw_param_setempty(params, SNDRV_PCM_HW_PARAM_RATE); return err; } err = rme9652_set_interrupt_interval(r, params_period_size(params) as c_uint); if err < 0 { _snd_pcm_hw_param_setempty(params, SNDRV_PCM_HW_PARAM_PERIOD_SIZE); return err; } 0 }
unsafe extern "C" fn snd_rme9652_channel_info(substream: *mut snd_pcm_substream, info: *mut snd_pcm_channel_info) -> c_int { let r = snd_pcm_substream_chip(substream) as *mut snd_rme9652; if snd_BUG_ON((*info).channel >= RME9652_NCHANNELS as c_uint) != 0 { return -EINVAL; } let chn = *(*r).channel_map.add(array_index_nospec((*info).channel, RME9652_NCHANNELS) as usize) as c_int; if chn < 0 { return -EINVAL; } (*info).offset = chn * RME9652_CHANNEL_BUFFER_BYTES as c_int; (*info).first = 0; (*info).step = 32; 0 }
unsafe extern "C" fn snd_rme9652_ioctl(substream: *mut snd_pcm_substream, cmd: c_uint, arg: *mut c_void) -> c_int { match cmd { SNDRV_PCM_IOCTL1_RESET => snd_rme9652_reset(substream), SNDRV_PCM_IOCTL1_CHANNEL_INFO => snd_rme9652_channel_info(substream, arg as *mut snd_pcm_channel_info), _ => snd_pcm_lib_ioctl(substream, cmd, arg) } }
unsafe fn rme9652_silence_playback(r: *mut snd_rme9652) { memset((*r).playback_buffer as *mut c_void, 0, RME9652_DMA_AREA_BYTES); }
unsafe extern "C" fn snd_rme9652_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int { let r = snd_pcm_substream_chip(substream) as *mut snd_rme9652; let mut running = (*r).running; match cmd { SNDRV_PCM_TRIGGER_START => running |= 1 << (*substream).stream, SNDRV_PCM_TRIGGER_STOP => running &= !(1 << (*substream).stream), _ => { snd_BUG(); return -EINVAL; } } let other = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK { (*r).capture_substream } else { (*r).playback_substream }; if !other.is_null() { if cmd == SNDRV_PCM_TRIGGER_START { if (running & (1 << SNDRV_PCM_STREAM_PLAYBACK)) == 0 && (*substream).stream == SNDRV_PCM_STREAM_CAPTURE { rme9652_silence_playback(r); } } else if running != 0 && (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK { rme9652_silence_playback(r); } } else if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE { rme9652_silence_playback(r); } snd_pcm_trigger_done(substream, substream); if (*r).running == 0 && running != 0 { rme9652_start(r); } else if (*r).running != 0 && running == 0 { rme9652_stop(r); } (*r).running = running; 0 }
unsafe extern "C" fn snd_rme9652_prepare(substream: *mut snd_pcm_substream) -> c_int { let r = snd_pcm_substream_chip(substream) as *mut snd_rme9652; if (*r).running == 0 { rme9652_reset_hw_pointer(r); } 0 }

static snd_rme9652_playback_subinfo: snd_pcm_hardware = snd_pcm_hardware { info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_NONINTERLEAVED | SNDRV_PCM_INFO_SYNC_START | SNDRV_PCM_INFO_DOUBLE, formats: SNDRV_PCM_FMTBIT_S32_LE, rates: SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000, rate_min: 44100, rate_max: 96000, channels_min: 10, channels_max: 26, buffer_bytes_max: RME9652_CHANNEL_BUFFER_BYTES * 26, period_bytes_min: (64 * 4) * 10, period_bytes_max: (8192 * 4) * 26, periods_min: 2, periods_max: 2, fifo_size: 0 };
static snd_rme9652_capture_subinfo: snd_pcm_hardware = snd_pcm_hardware { info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_NONINTERLEAVED | SNDRV_PCM_INFO_SYNC_START, formats: SNDRV_PCM_FMTBIT_S32_LE, rates: SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000, rate_min: 44100, rate_max: 96000, channels_min: 10, channels_max: 26, buffer_bytes_max: RME9652_CHANNEL_BUFFER_BYTES * 26, period_bytes_min: (64 * 4) * 10, period_bytes_max: (8192 * 4) * 26, periods_min: 2, periods_max: 2, fifo_size: 0 };
static period_sizes: [c_uint; 8] = [64, 128, 256, 512, 1024, 2048, 4096, 8192];
static hw_constraints_period_sizes: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list { count: 8, list: period_sizes.as_ptr(), mask: 0 };
unsafe extern "C" fn snd_rme9652_hw_rule_channels(params: *mut snd_pcm_hw_params, rule: *mut snd_pcm_hw_rule) -> c_int { let r = (*rule).private as *mut snd_rme9652; let c = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS); let list = [(*r).ds_channels as c_uint, (*r).ss_channels as c_uint]; snd_interval_list(c, 2, list.as_ptr(), 0) }
unsafe extern "C" fn snd_rme9652_hw_rule_channels_rate(params: *mut snd_pcm_hw_params, rule: *mut snd_pcm_hw_rule) -> c_int { let rme = (*rule).private as *mut snd_rme9652; let c = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS); let r = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE); if (*r).min > 48000 { let t = snd_interval { min: (*rme).ds_channels as c_uint, max: (*rme).ds_channels as c_uint, openmin: 0, openmax: 0, integer: 1, empty: 0 }; return snd_interval_refine(c, &t); } else if (*r).max < 88200 { let t = snd_interval { min: (*rme).ss_channels as c_uint, max: (*rme).ss_channels as c_uint, openmin: 0, openmax: 0, integer: 1, empty: 0 }; return snd_interval_refine(c, &t); } 0 }
unsafe extern "C" fn snd_rme9652_hw_rule_rate_channels(params: *mut snd_pcm_hw_params, rule: *mut snd_pcm_hw_rule) -> c_int { let rme = (*rule).private as *mut snd_rme9652; let c = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS); let r = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE); if (*c).min >= (*rme).ss_channels as c_uint { let t = snd_interval { min: 44100, max: 48000, openmin: 0, openmax: 0, integer: 1, empty: 0 }; return snd_interval_refine(r, &t); } else if (*c).max <= (*rme).ds_channels as c_uint { let t = snd_interval { min: 88200, max: 96000, openmin: 0, openmax: 0, integer: 1, empty: 0 }; return snd_interval_refine(r, &t); } 0 }
unsafe extern "C" fn snd_rme9652_playback_open(substream: *mut snd_pcm_substream) -> c_int { let r = snd_pcm_substream_chip(substream) as *mut snd_rme9652; let runtime = (*substream).runtime; snd_pcm_set_sync(substream); (*runtime).hw = snd_rme9652_playback_subinfo; snd_pcm_set_runtime_buffer(substream, &mut (*r).playback_dma_buf); if (*r).capture_substream.is_null() { rme9652_stop(r); rme9652_set_thru(r, -1, 0); } (*r).playback_pid = (*current).pid; (*r).playback_substream = substream; snd_pcm_hw_constraint_msbits(runtime, 0, 32, 24); snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_SIZE, &hw_constraints_period_sizes); snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, snd_rme9652_hw_rule_channels, r as *mut c_void, SNDRV_PCM_HW_PARAM_CHANNELS, -1); snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, snd_rme9652_hw_rule_channels_rate, r as *mut c_void, SNDRV_PCM_HW_PARAM_RATE, -1); snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, snd_rme9652_hw_rule_rate_channels, r as *mut c_void, SNDRV_PCM_HW_PARAM_CHANNELS, -1); (*r).creg_spdif_stream = (*r).creg_spdif; (*(*r).spdif_ctl).vd[0].access &= !SNDRV_CTL_ELEM_ACCESS_INACTIVE; snd_ctl_notify((*r).card, SNDRV_CTL_EVENT_MASK_VALUE | SNDRV_CTL_EVENT_MASK_INFO, &mut (*(*r).spdif_ctl).id); 0 }
unsafe extern "C" fn snd_rme9652_playback_release(substream: *mut snd_pcm_substream) -> c_int { let r = snd_pcm_substream_chip(substream) as *mut snd_rme9652; (*r).playback_pid = -1; (*r).playback_substream = ptr::null_mut(); (*(*r).spdif_ctl).vd[0].access |= SNDRV_CTL_ELEM_ACCESS_INACTIVE; snd_ctl_notify((*r).card, SNDRV_CTL_EVENT_MASK_VALUE | SNDRV_CTL_EVENT_MASK_INFO, &mut (*(*r).spdif_ctl).id); 0 }
unsafe extern "C" fn snd_rme9652_capture_open(substream: *mut snd_pcm_substream) -> c_int { let r = snd_pcm_substream_chip(substream) as *mut snd_rme9652; let runtime = (*substream).runtime; snd_pcm_set_sync(substream); (*runtime).hw = snd_rme9652_capture_subinfo; snd_pcm_set_runtime_buffer(substream, &mut (*r).capture_dma_buf); if (*r).playback_substream.is_null() { rme9652_stop(r); rme9652_set_thru(r, -1, 0); } (*r).capture_pid = (*current).pid; (*r).capture_substream = substream; snd_pcm_hw_constraint_msbits(runtime, 0, 32, 24); snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_SIZE, &hw_constraints_period_sizes); snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, snd_rme9652_hw_rule_channels, r as *mut c_void, SNDRV_PCM_HW_PARAM_CHANNELS, -1); snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, snd_rme9652_hw_rule_channels_rate, r as *mut c_void, SNDRV_PCM_HW_PARAM_RATE, -1); snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, snd_rme9652_hw_rule_rate_channels, r as *mut c_void, SNDRV_PCM_HW_PARAM_CHANNELS, -1); 0 }
unsafe extern "C" fn snd_rme9652_capture_release(substream: *mut snd_pcm_substream) -> c_int { let r = snd_pcm_substream_chip(substream) as *mut snd_rme9652; (*r).capture_pid = -1; (*r).capture_substream = ptr::null_mut(); 0 }
static snd_rme9652_playback_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_rme9652_playback_open), close: Some(snd_rme9652_playback_release), ioctl: Some(snd_rme9652_ioctl), hw_params: Some(snd_rme9652_hw_params), prepare: Some(snd_rme9652_prepare), trigger: Some(snd_rme9652_trigger), pointer: Some(snd_rme9652_hw_pointer), copy: Some(snd_rme9652_playback_copy), fill_silence: Some(snd_rme9652_hw_silence) };
static snd_rme9652_capture_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_rme9652_capture_open), close: Some(snd_rme9652_capture_release), ioctl: Some(snd_rme9652_ioctl), hw_params: Some(snd_rme9652_hw_params), prepare: Some(snd_rme9652_prepare), trigger: Some(snd_rme9652_trigger), pointer: Some(snd_rme9652_hw_pointer), copy: Some(snd_rme9652_capture_copy), fill_silence: None };
unsafe extern "C" fn snd_rme9652_create_pcm(card: *mut snd_card, r: *mut snd_rme9652) -> c_int { let mut pcm: *mut snd_pcm = ptr::null_mut(); let err = snd_pcm_new(card, (*r).card_name, 0, 1, 1, &mut pcm); if err < 0 { return err; } (*r).pcm = pcm; (*pcm).private_data = r as *mut c_void; strscpy((*pcm).name.as_mut_ptr(), (*r).card_name); snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_rme9652_playback_ops); snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_rme9652_capture_ops); (*pcm).info_flags = SNDRV_PCM_INFO_JOINT_DUPLEX; 0 }
unsafe extern "C" fn snd_rme9652_create(card: *mut snd_card, r: *mut snd_rme9652, precise: c_int) -> c_int { let pci = (*r).pci; let mut rev: u16 = 0; (*r).irq = -1; (*r).card = card; pci_read_config_word((*r).pci, PCI_CLASS_REVISION, &mut rev); match rev & 0xff { 3 | 4 | 8 | 9 => {}, _ => return -ENODEV } let mut err = pcim_enable_device(pci); if err < 0 { return err; } spin_lock_init(&mut (*r).lock); err = pcim_request_all_regions(pci, c"rme9652".as_ptr()); if err < 0 { return err; } (*r).port = pci_resource_start(pci, 0); (*r).iobase = devm_ioremap(&mut (*pci).dev, (*r).port, RME9652_IO_EXTENT); if (*r).iobase.is_null() { dev_err((*card).dev, c"unable to remap region 0x%lx-0x%lx\n".as_ptr(), (*r).port, (*r).port + RME9652_IO_EXTENT - 1); return -EBUSY; } if devm_request_irq(&mut (*pci).dev, (*pci).irq, snd_rme9652_interrupt, IRQF_SHARED, &KBUILD_MODNAME as *const c_char, r as *mut c_void) != 0 { dev_err((*card).dev, c"unable to request IRQ %d\n".as_ptr(), (*pci).irq); return -EBUSY; } (*r).irq = (*pci).irq; (*card).sync_irq = (*r).irq; (*r).precise_ptr = precise; let status = rme9652_read(r, RME9652_status_register); (*r).hw_rev = if rme9652_decode_spdif_rate(status & RME9652_F) == 1 { 15 } else { 11 }; match rev { 8 => { strscpy((*card).driver.as_mut_ptr(), c"RME9636".as_ptr()); (*r).card_name = if (*r).hw_rev == 15 { c"RME Digi9636 (Rev 1.5)".as_ptr() } else { c"RME Digi9636".as_ptr() }; (*r).ss_channels = RME9636_NCHANNELS as u8; }, 9 => { strscpy((*card).driver.as_mut_ptr(), c"RME9636".as_ptr()); (*r).card_name = c"RME Digi9636 (Rev G)".as_ptr(); (*r).ss_channels = RME9636_NCHANNELS as u8; }, 4 => { strscpy((*card).driver.as_mut_ptr(), c"RME9652".as_ptr()); (*r).card_name = c"RME Digi9652 (Rev G)".as_ptr(); (*r).ss_channels = RME9652_NCHANNELS as u8; }, 3 => { strscpy((*card).driver.as_mut_ptr(), c"RME9652".as_ptr()); (*r).card_name = if (*r).hw_rev == 15 { c"RME Digi9652 (Rev 1.5)".as_ptr() } else { c"RME Digi9652".as_ptr() }; (*r).ss_channels = RME9652_NCHANNELS as u8; }, _ => {} } (*r).ds_channels = (((*r).ss_channels as c_int - 2) / 2 + 2) as u8; pci_set_master((*r).pci); err = snd_rme9652_initialize_memory(r); if err < 0 { return err; } err = snd_rme9652_create_pcm(card, r); if err < 0 { return err; } err = snd_rme9652_create_controls(card, r); if err < 0 { return err; } snd_rme9652_proc_init(r); (*r).last_spdif_sample_rate = -1; (*r).last_adat_sample_rate = -1; (*r).playback_pid = -1; (*r).capture_pid = -1; (*r).capture_substream = ptr::null_mut(); (*r).playback_substream = ptr::null_mut(); snd_rme9652_set_defaults(r); if (*r).hw_rev == 15 { rme9652_initialize_spdif_receiver(r); } 0 }
unsafe extern "C" fn snd_rme9652_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int { static mut dev: c_int = 0; let mut r: *mut snd_rme9652; let mut card: *mut snd_card = ptr::null_mut(); if dev >= SNDRV_CARDS as c_int { return -ENODEV; } if !enable[dev as usize] { dev += 1; return -ENOENT; } let mut err = snd_devm_card_new(&mut (*pci).dev, index[dev as usize], id[dev as usize], ptr::null_mut(), core::mem::size_of::<snd_rme9652>(), &mut card); if err < 0 { return err; } r = (*card).private_data as *mut snd_rme9652; (*card).private_free = Some(snd_rme9652_card_free); (*r).dev = dev; (*r).pci = pci; err = snd_rme9652_create(card, r, precise_ptr[dev as usize] as c_int); if err != 0 { snd_card_free(card); return err; } strscpy((*card).shortname.as_mut_ptr(), (*r).card_name); sprintf((*card).longname.as_mut_ptr(), c"%s at 0x%lx, irq %d".as_ptr(), (*card).shortname.as_ptr(), (*r).port, (*r).irq); err = snd_card_register(card); if err != 0 { snd_card_free(card); return err; } pci_set_drvdata(pci, card as *mut c_void); dev += 1; 0 }
static mut rme9652_driver: pci_driver = pci_driver { name: unsafe { &KBUILD_MODNAME as *const c_char }, id_table: snd_rme9652_ids.as_ptr(), probe: Some(snd_rme9652_probe) };
/* module_pci_driver(rme9652_driver); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
