// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for RME Digi96, Digi96/8 and Digi96/8 PRO/PAD/PST audio
 *   interfaces
 *
 *	Copyright (c) 2000, 2001 Anders Torger <torger@ludd.luth.se>
 *
 *      Thanks to Henk Hesselink <henk@anda.nl> for the analog volume control
 *      code.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

/* Linux/ALSA includes in the original C source are external dependencies. */

type u8 = u8;
type u16 = u16;
type u32 = u32;
type size_t = usize;
type snd_pcm_uframes_t = c_ulong;
type snd_pcm_format_t = c_int;
type irqreturn_t = c_int;
type bool_ = bool;

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pci_dev {
    pub dev: device,
    pub irq: c_int,
    pub device: u16,
}
#[repr(C)]
pub struct pci_device_id {
    pub vendor: u32,
    pub device: u32,
    pub subvendor: u32,
    pub subdevice: u32,
    pub class: u32,
    pub class_mask: u32,
    pub driver_data: c_ulong,
}
#[repr(C)]
pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    pub driver: device_driver,
}
#[repr(C)]
pub struct device_driver {
    pub pm: *const dev_pm_ops,
}
#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub sync_irq: c_int,
    pub number: c_int,
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
}
#[repr(C)]
pub struct snd_pcm {
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_pcm)>,
    pub name: [c_char; 80],
    pub info_flags: c_int,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub group: *mut c_void,
}
#[repr(C)]
pub struct snd_pcm_runtime {
    pub dma_area: *mut c_void,
    pub dma_addr: c_ulong,
    pub dma_bytes: size_t,
    pub hw: snd_pcm_hardware,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hardware {
    pub info: u32,
    pub formats: u64,
    pub rates: u32,
    pub rate_min: u32,
    pub rate_max: u32,
    pub channels_min: u32,
    pub channels_max: u32,
    pub buffer_bytes_max: size_t,
    pub period_bytes_min: size_t,
    pub period_bytes_max: size_t,
    pub periods_min: u32,
    pub periods_max: u32,
    pub fifo_size: size_t,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: u32,
    pub list: *const u32,
    pub mask: u32,
}
#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pub copy: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, c_ulong, *mut iov_iter, c_ulong) -> c_int>,
    pub fill_silence: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, c_ulong, c_ulong) -> c_int>,
    pub mmap: Option<unsafe extern "C" fn() -> c_int>,
}
#[repr(C)]
pub struct iov_iter {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_info_entry {
    pub private_data: *mut c_void,
}
#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
    pub vd: [snd_kcontrol_volatile; 1],
    pub id: snd_ctl_elem_id,
}
#[repr(C)]
pub struct snd_kcontrol_volatile {
    pub access: u32,
}
#[repr(C)]
pub struct snd_ctl_elem_id {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_int,
    pub count: u32,
    pub value: snd_ctl_elem_info_value,
}
#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_integer {
    pub min: i64,
    pub max: i64,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_union,
}
#[repr(C)]
pub union snd_ctl_elem_value_union {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
    pub iec958: snd_aes_iec958,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [u32; 128],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_aes_iec958 {
    pub status: [u8; 24],
    pub subcode: [u8; 147],
    pub pad: u8,
    pub dig_subframe: [u8; 4],
}
#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_int,
    pub name: *const c_char,
    pub access: u32,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
}

unsafe extern "C" {
    static SNDRV_CARDS: c_int;
    static SNDRV_DEFAULT_IDX: [c_int; 0];
    static SNDRV_DEFAULT_STR: [*mut c_char; 0];
    static SNDRV_DEFAULT_ENABLE_PNP: [bool_; 0];
    static THIS_MODULE: *mut c_void;
    static KBUILD_MODNAME: *const c_char;
    static rme96_pm: dev_pm_ops;

    fn readl(addr: *mut c_void) -> u32;
    fn writel(val: u32, addr: *mut c_void);
    fn udelay(usecs: c_ulong);
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn memset_io(addr: *mut c_void, val: c_int, count: c_ulong);
    fn memcpy_fromio(dst: *mut c_void, src: *mut c_void, count: c_ulong);
    fn memcpy_toio(dst: *mut c_void, src: *const c_void, count: c_ulong);
    fn copy_from_iter_toio(addr: *mut c_void, count: c_ulong, src: *mut iov_iter) -> c_ulong;
    fn copy_to_iter_fromio(addr: *mut c_void, count: c_ulong, dst: *mut iov_iter) -> c_ulong;
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut rme96;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_set_sync(substream: *mut snd_pcm_substream);
    fn snd_pcm_trigger_done(s: *mut snd_pcm_substream, substream: *mut snd_pcm_substream);
    fn snd_pcm_rate_to_rate_bit(rate: c_int) -> u32;
    fn snd_pcm_hw_constraint_single(runtime: *mut snd_pcm_runtime, var: c_int, val: c_ulong) -> c_int;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn snd_pcm_lib_mmap_iomem() -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_period_size(params: *mut snd_pcm_hw_params) -> c_ulong;
    fn snd_BUG();
    fn spin_lock_init(lock: *mut spinlock_t);
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn pcim_request_all_regions(pci: *mut pci_dev, name: *const c_char) -> c_int;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn devm_ioremap(dev: *mut device, offset: c_ulong, size: c_ulong) -> *mut c_void;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn pci_read_config_byte(pci: *mut pci_dev, where_: c_int, val: *mut u8) -> c_int;
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_card_ro_proc_new(card: *mut snd_card, name: *const c_char, private_data: *mut rme96, read: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)) -> c_int;
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut rme96;
    fn snd_ctl_enum_info(uinfo: *mut snd_ctl_elem_info, channels: c_uint, items: c_uint, names: *const *const c_char) -> c_int;
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_ctl_elem_id);
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut rme96) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_power_change_state(card: *mut snd_card, state: c_int) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn vmalloc(size: c_ulong) -> *mut c_void;
    fn vfree(addr: *mut c_void);
    fn snd_devm_card_new(dev: *mut device, idx: c_int, id: *mut c_char, module: *mut c_void, extra_size: size_t, card_ret: *mut *mut snd_card) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_card_free_on_error(dev: *mut device, ret: c_int) -> c_int;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
}
type c_uint = u32;

const EFAULT: c_int = 14;
const EINVAL: c_int = 22;
const EIO: c_int = 5;
const EBUSY: c_int = 16;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;

const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_SHARED: c_ulong = 0x80;

const PCI_DEVICE_ID_RME_DIGI96: u16 = 0x3fc0;
const PCI_DEVICE_ID_RME_DIGI96_8: u16 = 0x3fc1;
const PCI_DEVICE_ID_RME_DIGI96_8_PRO: u16 = 0x3fc2;
const PCI_DEVICE_ID_RME_DIGI96_8_PAD_OR_PST: u16 = 0x3fc3;

const SNDRV_PCM_INFO_MMAP_IOMEM: u32 = 1 << 0;
const SNDRV_PCM_INFO_MMAP_VALID: u32 = 1 << 1;
const SNDRV_PCM_INFO_SYNC_START: u32 = 1 << 2;
const SNDRV_PCM_INFO_RESUME: u32 = 1 << 3;
const SNDRV_PCM_INFO_INTERLEAVED: u32 = 1 << 4;
const SNDRV_PCM_INFO_PAUSE: u32 = 1 << 5;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 1;
const SNDRV_PCM_RATE_32000: u32 = 1 << 0;
const SNDRV_PCM_RATE_44100: u32 = 1 << 1;
const SNDRV_PCM_RATE_48000: u32 = 1 << 2;
const SNDRV_PCM_RATE_64000: u32 = 1 << 3;
const SNDRV_PCM_RATE_88200: u32 = 1 << 4;
const SNDRV_PCM_RATE_96000: u32 = 1 << 5;
const SNDRV_PCM_FORMAT_S16_LE: snd_pcm_format_t = 0;
const SNDRV_PCM_FORMAT_S32_LE: snd_pcm_format_t = 1;
const SNDRV_PCM_HW_PARAM_BUFFER_BYTES: c_int = 0;
const SNDRV_PCM_HW_PARAM_PERIOD_BYTES: c_int = 1;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_CTL_ELEM_ACCESS_INACTIVE: u32 = 1 << 8;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: u32 = 3;
const SNDRV_CTL_ELEM_ACCESS_READ: u32 = 1;
const SNDRV_CTL_ELEM_IFACE_PCM: c_int = 2;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_int = 1;
const SNDRV_CTL_ELEM_TYPE_IEC958: c_int = 5;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_int = 2;
const SNDRV_CTL_EVENT_MASK_VALUE: c_uint = 1;
const SNDRV_CTL_EVENT_MASK_INFO: c_uint = 2;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;
const IEC958_AES0_PROFESSIONAL: u8 = 1 << 0;
const IEC958_AES0_NONAUDIO: u8 = 1 << 1;
const IEC958_AES0_PRO_EMPHASIS_5015: u8 = 1 << 2;
const IEC958_AES0_CON_EMPHASIS_5015: u8 = 1 << 3;
const IEC958_AES0_CON_EMPHASIS: c_ulong = 1 << 3;
const IEC958_AES0_PRO_EMPHASIS: c_ulong = 1 << 2;

/* note, two last pcis should be equal, it is not a bug */
/* MODULE_AUTHOR("Anders Torger <torger@ludd.luth.se>"); */
/* MODULE_DESCRIPTION("RME Digi96, Digi96/8, Digi96/8 PRO, Digi96/8 PST, Digi96/8 PAD"); */
/* MODULE_LICENSE("GPL"); */

static mut index: [c_int; 32] = [0; 32];	/* Index 0-MAX */
static mut id: [*mut c_char; 32] = [core::ptr::null_mut(); 32];	/* ID for this card */
static mut enable: [bool_; 32] = [false; 32];	/* Enable this card */

/*
 * Defines for RME Digi96 series, from internal RME reference documents
 * dated 12.01.00
 */
const RME96_SPDIF_NCHANNELS: c_int = 2;
/* Playback and capture buffer size */
const RME96_BUFFER_SIZE: c_ulong = 0x10000;
/* IO area size */
const RME96_IO_SIZE: c_ulong = 0x60000;
/* IO area offsets */
const RME96_IO_PLAY_BUFFER: c_ulong = 0x0;
const RME96_IO_REC_BUFFER: c_ulong = 0x10000;
const RME96_IO_CONTROL_REGISTER: c_ulong = 0x20000;
const RME96_IO_ADDITIONAL_REG: c_ulong = 0x20004;
const RME96_IO_CONFIRM_PLAY_IRQ: c_ulong = 0x20008;
const RME96_IO_CONFIRM_REC_IRQ: c_ulong = 0x2000C;
const RME96_IO_SET_PLAY_POS: c_ulong = 0x40000;
const RME96_IO_RESET_PLAY_POS: c_ulong = 0x4FFFC;
const RME96_IO_SET_REC_POS: c_ulong = 0x50000;
const RME96_IO_RESET_REC_POS: c_ulong = 0x5FFFC;
const RME96_IO_GET_PLAY_POS: c_ulong = 0x20000;
const RME96_IO_GET_REC_POS: c_ulong = 0x30000;

const RME96_WCR_START: u32 = 1 << 0;
const RME96_WCR_START_2: u32 = 1 << 1;
const RME96_WCR_GAIN_0: u32 = 1 << 2;
const RME96_WCR_GAIN_1: u32 = 1 << 3;
const RME96_WCR_MODE24: u32 = 1 << 4;
const RME96_WCR_MODE24_2: u32 = 1 << 5;
const RME96_WCR_BM: u32 = 1 << 6;
const RME96_WCR_BM_2: u32 = 1 << 7;
const RME96_WCR_ADAT: u32 = 1 << 8;
const RME96_WCR_FREQ_0: u32 = 1 << 9;
const RME96_WCR_FREQ_1: u32 = 1 << 10;
const RME96_WCR_DS: u32 = 1 << 11;
const RME96_WCR_PRO: u32 = 1 << 12;
const RME96_WCR_EMP: u32 = 1 << 13;
const RME96_WCR_SEL: u32 = 1 << 14;
const RME96_WCR_MASTER: u32 = 1 << 15;
const RME96_WCR_PD: u32 = 1 << 16;
const RME96_WCR_INP_0: u32 = 1 << 17;
const RME96_WCR_INP_1: u32 = 1 << 18;
const RME96_WCR_THRU_0: u32 = 1 << 19;
const RME96_WCR_THRU_1: u32 = 1 << 20;
const RME96_WCR_THRU_2: u32 = 1 << 21;
const RME96_WCR_THRU_3: u32 = 1 << 22;
const RME96_WCR_THRU_4: u32 = 1 << 23;
const RME96_WCR_THRU_5: u32 = 1 << 24;
const RME96_WCR_THRU_6: u32 = 1 << 25;
const RME96_WCR_THRU_7: u32 = 1 << 26;
const RME96_WCR_DOLBY: u32 = 1 << 27;
const RME96_WCR_MONITOR_0: u32 = 1 << 28;
const RME96_WCR_MONITOR_1: u32 = 1 << 29;
const RME96_WCR_ISEL: u32 = 1 << 30;
const RME96_WCR_IDIS: u32 = 1 << 31;

const RME96_WCR_BITPOS_GAIN_0: c_int = 2;
const RME96_WCR_BITPOS_GAIN_1: c_int = 3;
const RME96_WCR_BITPOS_FREQ_0: c_int = 9;
const RME96_WCR_BITPOS_FREQ_1: c_int = 10;
const RME96_WCR_BITPOS_INP_0: c_int = 17;
const RME96_WCR_BITPOS_INP_1: c_int = 18;
const RME96_WCR_BITPOS_MONITOR_0: c_int = 28;
const RME96_WCR_BITPOS_MONITOR_1: c_int = 29;

const RME96_RCR_AUDIO_ADDR_MASK: u32 = 0xFFFF;
const RME96_RCR_IRQ_2: u32 = 1 << 16;
const RME96_RCR_T_OUT: u32 = 1 << 17;
const RME96_RCR_DEV_ID_0: u32 = 1 << 21;
const RME96_RCR_DEV_ID_1: u32 = 1 << 22;
const RME96_RCR_LOCK: u32 = 1 << 23;
const RME96_RCR_VERF: u32 = 1 << 26;
const RME96_RCR_F0: u32 = 1 << 27;
const RME96_RCR_F1: u32 = 1 << 28;
const RME96_RCR_F2: u32 = 1 << 29;
const RME96_RCR_AUTOSYNC: u32 = 1 << 30;
const RME96_RCR_IRQ: u32 = 1 << 31;
const RME96_RCR_BITPOS_F0: c_int = 27;
const RME96_RCR_BITPOS_F1: c_int = 28;
const RME96_RCR_BITPOS_F2: c_int = 29;

const RME96_AR_WSEL: u32 = 1 << 0;
const RME96_AR_ANALOG: u32 = 1 << 1;
const RME96_AR_FREQPAD_0: u32 = 1 << 2;
const RME96_AR_FREQPAD_1: u32 = 1 << 3;
const RME96_AR_FREQPAD_2: u32 = 1 << 4;
const RME96_AR_PD2: u32 = 1 << 5;
const RME96_AR_DAC_EN: u32 = 1 << 6;
const RME96_AR_CLATCH: u32 = 1 << 7;
const RME96_AR_CCLK: u32 = 1 << 8;
const RME96_AR_CDATA: u32 = 1 << 9;
const RME96_AR_BITPOS_F0: c_int = 2;
const RME96_AR_BITPOS_F1: c_int = 3;
const RME96_AR_BITPOS_F2: c_int = 4;

const RME96_MONITOR_TRACKS_1_2: c_int = 0;
const RME96_MONITOR_TRACKS_3_4: c_int = 1;
const RME96_MONITOR_TRACKS_5_6: c_int = 2;
const RME96_MONITOR_TRACKS_7_8: c_int = 3;
const RME96_ATTENUATION_0: c_int = 0;
const RME96_ATTENUATION_6: c_int = 1;
const RME96_ATTENUATION_12: c_int = 2;
const RME96_ATTENUATION_18: c_int = 3;
const RME96_INPUT_OPTICAL: c_int = 0;
const RME96_INPUT_COAXIAL: c_int = 1;
const RME96_INPUT_INTERNAL: c_int = 2;
const RME96_INPUT_XLR: c_int = 3;
const RME96_INPUT_ANALOG: c_int = 4;
const RME96_CLOCKMODE_SLAVE: c_int = 0;
const RME96_CLOCKMODE_MASTER: c_int = 1;
const RME96_CLOCKMODE_WORDCLOCK: c_int = 2;
const RME96_SMALL_BLOCK_SIZE: c_ulong = 2048;
const RME96_LARGE_BLOCK_SIZE: c_ulong = 8192;
const RME96_AD1852_VOL_BITS: c_int = 14;
const RME96_AD1855_VOL_BITS: c_int = 10;

const RME96_TB_START_PLAYBACK: c_int = 1;
const RME96_TB_START_CAPTURE: c_int = 2;
const RME96_TB_STOP_PLAYBACK: c_int = 4;
const RME96_TB_STOP_CAPTURE: c_int = 8;
const RME96_TB_RESET_PLAYPOS: c_int = 16;
const RME96_TB_RESET_CAPTUREPOS: c_int = 32;
const RME96_TB_CLEAR_PLAYBACK_IRQ: c_int = 64;
const RME96_TB_CLEAR_CAPTURE_IRQ: c_int = 128;
const RME96_RESUME_PLAYBACK: c_int = RME96_TB_START_PLAYBACK;
const RME96_RESUME_CAPTURE: c_int = RME96_TB_START_CAPTURE;
const RME96_RESUME_BOTH: c_int = RME96_RESUME_PLAYBACK | RME96_RESUME_CAPTURE;
const RME96_START_PLAYBACK: c_int = RME96_TB_START_PLAYBACK | RME96_TB_RESET_PLAYPOS;
const RME96_START_CAPTURE: c_int = RME96_TB_START_CAPTURE | RME96_TB_RESET_CAPTUREPOS;
const RME96_START_BOTH: c_int = RME96_START_PLAYBACK | RME96_START_CAPTURE;
const RME96_STOP_PLAYBACK: c_int = RME96_TB_STOP_PLAYBACK | RME96_TB_CLEAR_PLAYBACK_IRQ;
const RME96_STOP_CAPTURE: c_int = RME96_TB_STOP_CAPTURE | RME96_TB_CLEAR_CAPTURE_IRQ;
const RME96_STOP_BOTH: c_int = RME96_STOP_PLAYBACK | RME96_STOP_CAPTURE;

#[repr(C)]
pub struct rme96 {
    pub lock: spinlock_t,
    pub irq: c_int,
    pub port: c_ulong,
    pub iobase: *mut c_void,
    pub wcreg: u32,
    pub wcreg_spdif: u32,
    pub wcreg_spdif_stream: u32,
    pub rcreg: u32,
    pub areg: u32,
    pub vol: [u16; 2],
    pub rev: u8,
    pub playback_pointer: u32,
    pub capture_pointer: u32,
    pub playback_suspend_buffer: *mut c_void,
    pub capture_suspend_buffer: *mut c_void,
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub playback_frlog: c_int,
    pub capture_frlog: c_int,
    pub playback_periodsize: size_t,
    pub capture_periodsize: size_t,
    pub card: *mut snd_card,
    pub spdif_pcm: *mut snd_pcm,
    pub adat_pcm: *mut snd_pcm,
    pub pci: *mut pci_dev,
    pub spdif_ctl: *mut snd_kcontrol,
}

static snd_rme96_ids: [pci_device_id; 5] = [
    pci_device_id { vendor: 0, device: PCI_DEVICE_ID_RME_DIGI96 as u32, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
    pci_device_id { vendor: 0, device: PCI_DEVICE_ID_RME_DIGI96_8 as u32, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
    pci_device_id { vendor: 0, device: PCI_DEVICE_ID_RME_DIGI96_8_PRO as u32, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
    pci_device_id { vendor: 0, device: PCI_DEVICE_ID_RME_DIGI96_8_PAD_OR_PST as u32, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
    pci_device_id { vendor: 0, device: 0, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
];
/* MODULE_DEVICE_TABLE(pci, snd_rme96_ids); */

#[inline] unsafe fn RME96_ISPLAYING(r: *mut rme96) -> bool { ((*r).wcreg & RME96_WCR_START) != 0 }
#[inline] unsafe fn RME96_ISRECORDING(r: *mut rme96) -> bool { ((*r).wcreg & RME96_WCR_START_2) != 0 }
#[inline] unsafe fn RME96_HAS_ANALOG_IN(r: *mut rme96) -> bool { (*(*r).pci).device == PCI_DEVICE_ID_RME_DIGI96_8_PAD_OR_PST }
#[inline] unsafe fn RME96_HAS_ANALOG_OUT(r: *mut rme96) -> bool { (*(*r).pci).device == PCI_DEVICE_ID_RME_DIGI96_8_PRO || (*(*r).pci).device == PCI_DEVICE_ID_RME_DIGI96_8_PAD_OR_PST }
#[inline] unsafe fn RME96_DAC_IS_1852(r: *mut rme96) -> bool { RME96_HAS_ANALOG_OUT(r) && (*r).rev >= 4 }
#[inline] unsafe fn RME96_DAC_IS_1855(r: *mut rme96) -> bool {
    ((*(*r).pci).device == PCI_DEVICE_ID_RME_DIGI96_8_PAD_OR_PST && (*r).rev < 4) ||
    ((*(*r).pci).device == PCI_DEVICE_ID_RME_DIGI96_8_PRO && (*r).rev == 2)
}
#[inline] unsafe fn RME96_185X_MAX_OUT(r: *mut rme96) -> u32 {
    (1u32 << if RME96_DAC_IS_1852(r) { RME96_AD1852_VOL_BITS } else { RME96_AD1855_VOL_BITS }) - 1
}
#[inline] unsafe fn ioadd(base: *mut c_void, off: c_ulong) -> *mut c_void { (base as *mut u8).add(off as usize) as *mut c_void }

unsafe extern "C" fn snd_rme96_playback_ptr(rme96: *mut rme96) -> c_uint {
    (readl(ioadd((*rme96).iobase, RME96_IO_GET_PLAY_POS)) & RME96_RCR_AUDIO_ADDR_MASK) >> (*rme96).playback_frlog
}
unsafe extern "C" fn snd_rme96_capture_ptr(rme96: *mut rme96) -> c_uint {
    (readl(ioadd((*rme96).iobase, RME96_IO_GET_REC_POS)) & RME96_RCR_AUDIO_ADDR_MASK) >> (*rme96).capture_frlog
}

unsafe extern "C" fn snd_rme96_playback_silence(substream: *mut snd_pcm_substream, _channel: c_int, pos: c_ulong, count: c_ulong) -> c_int {
    let rme96 = snd_pcm_substream_chip(substream);
    memset_io(ioadd((*rme96).iobase, RME96_IO_PLAY_BUFFER + pos), 0, count);
    0
}
unsafe extern "C" fn snd_rme96_playback_copy(substream: *mut snd_pcm_substream, _channel: c_int, pos: c_ulong, src: *mut iov_iter, count: c_ulong) -> c_int {
    let rme96 = snd_pcm_substream_chip(substream);
    if copy_from_iter_toio(ioadd((*rme96).iobase, RME96_IO_PLAY_BUFFER + pos), count, src) != count { return -EFAULT; }
    0
}
unsafe extern "C" fn snd_rme96_capture_copy(substream: *mut snd_pcm_substream, _channel: c_int, pos: c_ulong, dst: *mut iov_iter, count: c_ulong) -> c_int {
    let rme96 = snd_pcm_substream_chip(substream);
    if copy_to_iter_fromio(ioadd((*rme96).iobase, RME96_IO_REC_BUFFER + pos), count, dst) != count { return -EFAULT; }
    0
}

static snd_rme96_playback_spdif_info: snd_pcm_hardware = snd_pcm_hardware { info: SNDRV_PCM_INFO_MMAP_IOMEM | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_SYNC_START | SNDRV_PCM_INFO_RESUME | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_PAUSE, formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE, rates: SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_64000 | SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000, rate_min: 32000, rate_max: 96000, channels_min: 2, channels_max: 2, buffer_bytes_max: RME96_BUFFER_SIZE as usize, period_bytes_min: RME96_SMALL_BLOCK_SIZE as usize, period_bytes_max: RME96_LARGE_BLOCK_SIZE as usize, periods_min: (RME96_BUFFER_SIZE / RME96_LARGE_BLOCK_SIZE) as u32, periods_max: (RME96_BUFFER_SIZE / RME96_SMALL_BLOCK_SIZE) as u32, fifo_size: 0 };
static snd_rme96_capture_spdif_info: snd_pcm_hardware = snd_rme96_playback_spdif_info;
static snd_rme96_playback_adat_info: snd_pcm_hardware = snd_pcm_hardware { info: SNDRV_PCM_INFO_MMAP_IOMEM | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_SYNC_START | SNDRV_PCM_INFO_RESUME | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_PAUSE, formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE, rates: SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000, rate_min: 44100, rate_max: 48000, channels_min: 8, channels_max: 8, buffer_bytes_max: RME96_BUFFER_SIZE as usize, period_bytes_min: RME96_SMALL_BLOCK_SIZE as usize, period_bytes_max: RME96_LARGE_BLOCK_SIZE as usize, periods_min: (RME96_BUFFER_SIZE / RME96_LARGE_BLOCK_SIZE) as u32, periods_max: (RME96_BUFFER_SIZE / RME96_SMALL_BLOCK_SIZE) as u32, fifo_size: 0 };
static snd_rme96_capture_adat_info: snd_pcm_hardware = snd_rme96_playback_adat_info;

/*
 * The CDATA, CCLK and CLATCH bits can be used to write to the SPI interface
 * of the AD1852 or AD1852 D/A converter on the board.
 */
unsafe extern "C" fn snd_rme96_write_SPI(rme96: *mut rme96, mut val: u16) {
    let mut i = 0;
    while i < 16 {
        if (val & 0x8000) != 0 { (*rme96).areg |= RME96_AR_CDATA; } else { (*rme96).areg &= !RME96_AR_CDATA; }
        (*rme96).areg &= !(RME96_AR_CCLK | RME96_AR_CLATCH);
        writel((*rme96).areg, ioadd((*rme96).iobase, RME96_IO_ADDITIONAL_REG));
        udelay(10);
        (*rme96).areg |= RME96_AR_CCLK;
        writel((*rme96).areg, ioadd((*rme96).iobase, RME96_IO_ADDITIONAL_REG));
        udelay(10);
        val <<= 1;
        i += 1;
    }
    (*rme96).areg &= !(RME96_AR_CCLK | RME96_AR_CDATA);
    (*rme96).areg |= RME96_AR_CLATCH;
    writel((*rme96).areg, ioadd((*rme96).iobase, RME96_IO_ADDITIONAL_REG));
    udelay(10);
    (*rme96).areg &= !RME96_AR_CLATCH;
    writel((*rme96).areg, ioadd((*rme96).iobase, RME96_IO_ADDITIONAL_REG));
}

unsafe extern "C" fn snd_rme96_apply_dac_volume(rme96: *mut rme96) {
    if RME96_DAC_IS_1852(rme96) {
        snd_rme96_write_SPI(rme96, (((*rme96).vol[0] << 2) | 0x0) as u16);
        snd_rme96_write_SPI(rme96, (((*rme96).vol[1] << 2) | 0x2) as u16);
    } else if RME96_DAC_IS_1855(rme96) {
        snd_rme96_write_SPI(rme96, (((*rme96).vol[0] & 0x3FF) | 0x000) as u16);
        snd_rme96_write_SPI(rme96, (((*rme96).vol[1] & 0x3FF) | 0x400) as u16);
    }
}
unsafe extern "C" fn snd_rme96_reset_dac(rme96: *mut rme96) {
    writel((*rme96).wcreg | RME96_WCR_PD, ioadd((*rme96).iobase, RME96_IO_CONTROL_REGISTER));
    writel((*rme96).wcreg, ioadd((*rme96).iobase, RME96_IO_CONTROL_REGISTER));
}
unsafe extern "C" fn snd_rme96_getmontracks(rme96: *mut rme96) -> c_int {
    (((*rme96).wcreg >> RME96_WCR_BITPOS_MONITOR_0) & 1) as c_int + ((((*rme96).wcreg >> RME96_WCR_BITPOS_MONITOR_1) & 1) << 1) as c_int
}
unsafe extern "C" fn snd_rme96_setmontracks(rme96: *mut rme96, montracks: c_int) -> c_int {
    if (montracks & 1) != 0 { (*rme96).wcreg |= RME96_WCR_MONITOR_0; } else { (*rme96).wcreg &= !RME96_WCR_MONITOR_0; }
    if (montracks & 2) != 0 { (*rme96).wcreg |= RME96_WCR_MONITOR_1; } else { (*rme96).wcreg &= !RME96_WCR_MONITOR_1; }
    writel((*rme96).wcreg, ioadd((*rme96).iobase, RME96_IO_CONTROL_REGISTER));
    0
}
unsafe extern "C" fn snd_rme96_getattenuation(rme96: *mut rme96) -> c_int {
    (((*rme96).wcreg >> RME96_WCR_BITPOS_GAIN_0) & 1) as c_int + ((((*rme96).wcreg >> RME96_WCR_BITPOS_GAIN_1) & 1) << 1) as c_int
}
unsafe extern "C" fn snd_rme96_setattenuation(rme96: *mut rme96, attenuation: c_int) -> c_int {
    match attenuation {
        0 => (*rme96).wcreg = ((*rme96).wcreg & !RME96_WCR_GAIN_0) & !RME96_WCR_GAIN_1,
        1 => (*rme96).wcreg = ((*rme96).wcreg | RME96_WCR_GAIN_0) & !RME96_WCR_GAIN_1,
        2 => (*rme96).wcreg = ((*rme96).wcreg & !RME96_WCR_GAIN_0) | RME96_WCR_GAIN_1,
        3 => (*rme96).wcreg = ((*rme96).wcreg | RME96_WCR_GAIN_0) | RME96_WCR_GAIN_1,
        _ => return -EINVAL,
    }
    writel((*rme96).wcreg, ioadd((*rme96).iobase, RME96_IO_CONTROL_REGISTER));
    0
}

unsafe extern "C" fn snd_rme96_capture_getrate(rme96: *mut rme96, is_adat: *mut c_int) -> c_int {
    let mut n: c_int;
    let rate: c_int;
    *is_adat = 0;
    if ((*rme96).areg & RME96_AR_ANALOG) != 0 {
        n = (((*rme96).areg >> RME96_AR_BITPOS_F0) & 1) as c_int + ((((*rme96).areg >> RME96_AR_BITPOS_F1) & 1) << 1) as c_int;
        rate = match n { 1 => 32000, 2 => 44100, 3 => 48000, _ => return -1 };
        return if ((*rme96).areg & RME96_AR_FREQPAD_2) != 0 { rate << 1 } else { rate };
    }
    (*rme96).rcreg = readl(ioadd((*rme96).iobase, RME96_IO_CONTROL_REGISTER));
    if ((*rme96).rcreg & RME96_RCR_LOCK) != 0 {
        *is_adat = 1;
        return if ((*rme96).rcreg & RME96_RCR_T_OUT) != 0 { 48000 } else { 44100 };
    }
    if ((*rme96).rcreg & RME96_RCR_VERF) != 0 { return -1; }
    n = (((*rme96).rcreg >> RME96_RCR_BITPOS_F0) & 1) as c_int + ((((*rme96).rcreg >> RME96_RCR_BITPOS_F1) & 1) << 1) as c_int + ((((*rme96).rcreg >> RME96_RCR_BITPOS_F2) & 1) << 2) as c_int;
    match n {
        0 => if ((*rme96).rcreg & RME96_RCR_T_OUT) != 0 { 64000 } else { -1 },
        3 => 96000,
        4 => 88200,
        5 => 48000,
        6 => 44100,
        7 => 32000,
        _ => -1,
    }
}

unsafe extern "C" fn snd_rme96_playback_getrate(rme96: *mut rme96) -> c_int {
    let mut rate: c_int;
    let mut dummy: c_int = 0;
    if ((*rme96).wcreg & RME96_WCR_MASTER) == 0 && snd_rme96_getinputtype(rme96) != RME96_INPUT_ANALOG {
        rate = snd_rme96_capture_getrate(rme96, &mut dummy);
        if rate > 0 { return rate; }
    }
    rate = (((*rme96).wcreg >> RME96_WCR_BITPOS_FREQ_0) & 1) as c_int + ((((*rme96).wcreg >> RME96_WCR_BITPOS_FREQ_1) & 1) << 1) as c_int;
    rate = match rate { 1 => 32000, 2 => 44100, 3 => 48000, _ => return -1 };
    if ((*rme96).wcreg & RME96_WCR_DS) != 0 { rate << 1 } else { rate }
}

unsafe extern "C" fn snd_rme96_playback_setrate(rme96: *mut rme96, rate: c_int) -> c_int {
    let ds = (*rme96).wcreg & RME96_WCR_DS;
    match rate {
        32000 => { (*rme96).wcreg &= !RME96_WCR_DS; (*rme96).wcreg = ((*rme96).wcreg | RME96_WCR_FREQ_0) & !RME96_WCR_FREQ_1; }
        44100 => { (*rme96).wcreg &= !RME96_WCR_DS; (*rme96).wcreg = ((*rme96).wcreg | RME96_WCR_FREQ_1) & !RME96_WCR_FREQ_0; }
        48000 => { (*rme96).wcreg &= !RME96_WCR_DS; (*rme96).wcreg = ((*rme96).wcreg | RME96_WCR_FREQ_0) | RME96_WCR_FREQ_1; }
        64000 => { (*rme96).wcreg |= RME96_WCR_DS; (*rme96).wcreg = ((*rme96).wcreg | RME96_WCR_FREQ_0) & !RME96_WCR_FREQ_1; }
        88200 => { (*rme96).wcreg |= RME96_WCR_DS; (*rme96).wcreg = ((*rme96).wcreg | RME96_WCR_FREQ_1) & !RME96_WCR_FREQ_0; }
        96000 => { (*rme96).wcreg |= RME96_WCR_DS; (*rme96).wcreg = ((*rme96).wcreg | RME96_WCR_FREQ_0) | RME96_WCR_FREQ_1; }
        _ => return -EINVAL,
    }
    if (ds == 0 && ((*rme96).wcreg & RME96_WCR_DS) != 0) || (ds != 0 && ((*rme96).wcreg & RME96_WCR_DS) == 0) {
        snd_rme96_reset_dac(rme96);
        1
    } else {
        writel((*rme96).wcreg, ioadd((*rme96).iobase, RME96_IO_CONTROL_REGISTER));
        0
    }
}

unsafe extern "C" fn snd_rme96_capture_analog_setrate(rme96: *mut rme96, rate: c_int) -> c_int {
    match rate {
        32000 => (*rme96).areg = (((*rme96).areg | RME96_AR_FREQPAD_0) & !RME96_AR_FREQPAD_1) & !RME96_AR_FREQPAD_2,
        44100 => (*rme96).areg = (((*rme96).areg & !RME96_AR_FREQPAD_0) | RME96_AR_FREQPAD_1) & !RME96_AR_FREQPAD_2,
        48000 => (*rme96).areg = (((*rme96).areg | RME96_AR_FREQPAD_0) | RME96_AR_FREQPAD_1) & !RME96_AR_FREQPAD_2,
        64000 => { if (*rme96).rev < 4 { return -EINVAL; } (*rme96).areg = (((*rme96).areg | RME96_AR_FREQPAD_0) & !RME96_AR_FREQPAD_1) | RME96_AR_FREQPAD_2; }
        88200 => { if (*rme96).rev < 4 { return -EINVAL; } (*rme96).areg = (((*rme96).areg & !RME96_AR_FREQPAD_0) | RME96_AR_FREQPAD_1) | RME96_AR_FREQPAD_2; }
        96000 => (*rme96).areg = (((*rme96).areg | RME96_AR_FREQPAD_0) | RME96_AR_FREQPAD_1) | RME96_AR_FREQPAD_2,
        _ => return -EINVAL,
    }
    writel((*rme96).areg, ioadd((*rme96).iobase, RME96_IO_ADDITIONAL_REG));
    0
}

unsafe extern "C" fn snd_rme96_setclockmode(rme96: *mut rme96, mode: c_int) -> c_int {
    match mode {
        RME96_CLOCKMODE_SLAVE => { (*rme96).wcreg &= !RME96_WCR_MASTER; (*rme96).areg &= !RME96_AR_WSEL; }
        RME96_CLOCKMODE_MASTER => { (*rme96).wcreg |= RME96_WCR_MASTER; (*rme96).areg &= !RME96_AR_WSEL; }
        RME96_CLOCKMODE_WORDCLOCK => { (*rme96).wcreg |= RME96_WCR_MASTER; (*rme96).areg |= RME96_AR_WSEL; }
        _ => return -EINVAL,
    }
    writel((*rme96).wcreg, ioadd((*rme96).iobase, RME96_IO_CONTROL_REGISTER));
    writel((*rme96).areg, ioadd((*rme96).iobase, RME96_IO_ADDITIONAL_REG));
    0
}
unsafe extern "C" fn snd_rme96_getclockmode(rme96: *mut rme96) -> c_int {
    if ((*rme96).areg & RME96_AR_WSEL) != 0 { RME96_CLOCKMODE_WORDCLOCK } else if ((*rme96).wcreg & RME96_WCR_MASTER) != 0 { RME96_CLOCKMODE_MASTER } else { RME96_CLOCKMODE_SLAVE }
}

unsafe extern "C" fn snd_rme96_setinputtype(rme96: *mut rme96, type_: c_int) -> c_int {
    let mut n = 0;
    match type_ {
        RME96_INPUT_OPTICAL => (*rme96).wcreg = ((*rme96).wcreg & !RME96_WCR_INP_0) & !RME96_WCR_INP_1,
        RME96_INPUT_COAXIAL => (*rme96).wcreg = ((*rme96).wcreg | RME96_WCR_INP_0) & !RME96_WCR_INP_1,
        RME96_INPUT_INTERNAL => (*rme96).wcreg = ((*rme96).wcreg & !RME96_WCR_INP_0) | RME96_WCR_INP_1,
        RME96_INPUT_XLR => {
            if (((*(*rme96).pci).device != PCI_DEVICE_ID_RME_DIGI96_8_PAD_OR_PST) && ((*(*rme96).pci).device != PCI_DEVICE_ID_RME_DIGI96_8_PRO)) || (((*(*rme96).pci).device == PCI_DEVICE_ID_RME_DIGI96_8_PAD_OR_PST) && (*rme96).rev > 4) { return -EINVAL; }
            (*rme96).wcreg = ((*rme96).wcreg | RME96_WCR_INP_0) | RME96_WCR_INP_1;
        }
        RME96_INPUT_ANALOG => {
            if !RME96_HAS_ANALOG_IN(rme96) { return -EINVAL; }
            (*rme96).areg |= RME96_AR_ANALOG;
            writel((*rme96).areg, ioadd((*rme96).iobase, RME96_IO_ADDITIONAL_REG));
            if (*rme96).rev < 4 {
                if snd_rme96_capture_getrate(rme96, &mut n) == 88200 { snd_rme96_capture_analog_setrate(rme96, 44100); }
                if snd_rme96_capture_getrate(rme96, &mut n) == 64000 { snd_rme96_capture_analog_setrate(rme96, 32000); }
            }
            return 0;
        }
        _ => return -EINVAL,
    }
    if type_ != RME96_INPUT_ANALOG && RME96_HAS_ANALOG_IN(rme96) {
        (*rme96).areg &= !RME96_AR_ANALOG;
        writel((*rme96).areg, ioadd((*rme96).iobase, RME96_IO_ADDITIONAL_REG));
    }
    writel((*rme96).wcreg, ioadd((*rme96).iobase, RME96_IO_CONTROL_REGISTER));
    0
}
unsafe extern "C" fn snd_rme96_getinputtype(rme96: *mut rme96) -> c_int {
    if ((*rme96).areg & RME96_AR_ANALOG) != 0 { return RME96_INPUT_ANALOG; }
    (((*rme96).wcreg >> RME96_WCR_BITPOS_INP_0) & 1) as c_int + ((((*rme96).wcreg >> RME96_WCR_BITPOS_INP_1) & 1) << 1) as c_int
}
unsafe extern "C" fn snd_rme96_setframelog(rme96: *mut rme96, n_channels: c_int, is_playback: c_int) {
    let mut frlog = if n_channels == 2 { 1 } else { 3 };
    if is_playback != 0 {
        frlog += if ((*rme96).wcreg & RME96_WCR_MODE24) != 0 { 2 } else { 1 };
        (*rme96).playback_frlog = frlog;
    } else {
        frlog += if ((*rme96).wcreg & RME96_WCR_MODE24_2) != 0 { 2 } else { 1 };
        (*rme96).capture_frlog = frlog;
    }
}
unsafe extern "C" fn snd_rme96_playback_setformat(rme96: *mut rme96, format: snd_pcm_format_t) -> c_int {
    match format { SNDRV_PCM_FORMAT_S16_LE => (*rme96).wcreg &= !RME96_WCR_MODE24, SNDRV_PCM_FORMAT_S32_LE => (*rme96).wcreg |= RME96_WCR_MODE24, _ => return -EINVAL }
    writel((*rme96).wcreg, ioadd((*rme96).iobase, RME96_IO_CONTROL_REGISTER)); 0
}
unsafe extern "C" fn snd_rme96_capture_setformat(rme96: *mut rme96, format: snd_pcm_format_t) -> c_int {
    match format { SNDRV_PCM_FORMAT_S16_LE => (*rme96).wcreg &= !RME96_WCR_MODE24_2, SNDRV_PCM_FORMAT_S32_LE => (*rme96).wcreg |= RME96_WCR_MODE24_2, _ => return -EINVAL }
    writel((*rme96).wcreg, ioadd((*rme96).iobase, RME96_IO_CONTROL_REGISTER)); 0
}
unsafe extern "C" fn snd_rme96_set_period_properties(rme96: *mut rme96, period_bytes_: size_t) {
    match period_bytes_ as c_ulong {
        RME96_LARGE_BLOCK_SIZE => (*rme96).wcreg &= !RME96_WCR_ISEL,
        RME96_SMALL_BLOCK_SIZE => (*rme96).wcreg |= RME96_WCR_ISEL,
        _ => snd_BUG(),
    }
    (*rme96).wcreg &= !RME96_WCR_IDIS;
    writel((*rme96).wcreg, ioadd((*rme96).iobase, RME96_IO_CONTROL_REGISTER));
}

unsafe extern "C" fn snd_rme96_playback_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let rme96 = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let mut err: c_int;
    let mut rate: c_int;
    let mut dummy: c_int = 0;
    let mut apply_dac_volume = false;
    (*runtime).dma_area = ioadd((*rme96).iobase, RME96_IO_PLAY_BUFFER);
    (*runtime).dma_addr = (*rme96).port + RME96_IO_PLAY_BUFFER;
    (*runtime).dma_bytes = RME96_BUFFER_SIZE as usize;
    /* scoped_guard(spinlock_irq, &rme96->lock) */
    rate = 0;
    if ((*rme96).wcreg & RME96_WCR_MASTER) == 0 && snd_rme96_getinputtype(rme96) != RME96_INPUT_ANALOG { rate = snd_rme96_capture_getrate(rme96, &mut dummy); }
    if rate > 0 {
        if params_rate(params) != rate { return -EIO; }
    } else {
        err = snd_rme96_playback_setrate(rme96, params_rate(params));
        if err < 0 { return err; }
        apply_dac_volume = err > 0;
    }
    err = snd_rme96_playback_setformat(rme96, params_format(params));
    if err < 0 { return err; }
    snd_rme96_setframelog(rme96, params_channels(params), 1);
    if (*rme96).capture_periodsize != 0 && ((params_period_size(params) << (*rme96).playback_frlog) as usize) != (*rme96).capture_periodsize { err = -EBUSY; } else {
        (*rme96).playback_periodsize = (params_period_size(params) << (*rme96).playback_frlog) as usize;
        snd_rme96_set_period_properties(rme96, (*rme96).playback_periodsize);
        if ((*rme96).wcreg & RME96_WCR_ADAT) == 0 {
            (*rme96).wcreg &= !(RME96_WCR_PRO | RME96_WCR_DOLBY | RME96_WCR_EMP);
            (*rme96).wcreg |= (*rme96).wcreg_spdif_stream;
            writel((*rme96).wcreg, ioadd((*rme96).iobase, RME96_IO_CONTROL_REGISTER));
        }
        err = 0;
    }
    if apply_dac_volume { usleep_range(3000, 10000); snd_rme96_apply_dac_volume(rme96); }
    err
}

unsafe extern "C" fn snd_rme96_capture_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let rme96 = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let mut err: c_int;
    let mut isadat: c_int = 0;
    let rate: c_int;
    (*runtime).dma_area = ioadd((*rme96).iobase, RME96_IO_REC_BUFFER);
    (*runtime).dma_addr = (*rme96).port + RME96_IO_REC_BUFFER;
    (*runtime).dma_bytes = RME96_BUFFER_SIZE as usize;
    err = snd_rme96_capture_setformat(rme96, params_format(params));
    if err < 0 { return err; }
    if snd_rme96_getinputtype(rme96) == RME96_INPUT_ANALOG {
        err = snd_rme96_capture_analog_setrate(rme96, params_rate(params));
        if err < 0 { return err; }
    } else {
        rate = snd_rme96_capture_getrate(rme96, &mut isadat);
        if rate > 0 {
            if params_rate(params) != rate { return -EIO; }
            if (isadat != 0 && (*runtime).hw.channels_min == 2) || (isadat == 0 && (*runtime).hw.channels_min == 8) { return -EIO; }
        }
    }
    snd_rme96_setframelog(rme96, params_channels(params), 0);
    if (*rme96).playback_periodsize != 0 && ((params_period_size(params) << (*rme96).capture_frlog) as usize) != (*rme96).playback_periodsize { return -EBUSY; }
    (*rme96).capture_periodsize = (params_period_size(params) << (*rme96).capture_frlog) as usize;
    snd_rme96_set_period_properties(rme96, (*rme96).capture_periodsize);
    0
}

unsafe extern "C" fn snd_rme96_trigger(rme96: *mut rme96, op: c_int) {
    if (op & RME96_TB_RESET_PLAYPOS) != 0 { writel(0, ioadd((*rme96).iobase, RME96_IO_RESET_PLAY_POS)); }
    if (op & RME96_TB_RESET_CAPTUREPOS) != 0 { writel(0, ioadd((*rme96).iobase, RME96_IO_RESET_REC_POS)); }
    if (op & RME96_TB_CLEAR_PLAYBACK_IRQ) != 0 {
        (*rme96).rcreg = readl(ioadd((*rme96).iobase, RME96_IO_CONTROL_REGISTER));
        if ((*rme96).rcreg & RME96_RCR_IRQ) != 0 { writel(0, ioadd((*rme96).iobase, RME96_IO_CONFIRM_PLAY_IRQ)); }
    }
    if (op & RME96_TB_CLEAR_CAPTURE_IRQ) != 0 {
        (*rme96).rcreg = readl(ioadd((*rme96).iobase, RME96_IO_CONTROL_REGISTER));
        if ((*rme96).rcreg & RME96_RCR_IRQ_2) != 0 { writel(0, ioadd((*rme96).iobase, RME96_IO_CONFIRM_REC_IRQ)); }
    }
    if (op & RME96_TB_START_PLAYBACK) != 0 { (*rme96).wcreg |= RME96_WCR_START; }
    if (op & RME96_TB_STOP_PLAYBACK) != 0 { (*rme96).wcreg &= !RME96_WCR_START; }
    if (op & RME96_TB_START_CAPTURE) != 0 { (*rme96).wcreg |= RME96_WCR_START_2; }
    if (op & RME96_TB_STOP_CAPTURE) != 0 { (*rme96).wcreg &= !RME96_WCR_START_2; }
    writel((*rme96).wcreg, ioadd((*rme96).iobase, RME96_IO_CONTROL_REGISTER));
}

unsafe extern "C" fn snd_rme96_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let rme96 = dev_id as *mut rme96;
    (*rme96).rcreg = readl(ioadd((*rme96).iobase, RME96_IO_CONTROL_REGISTER));
    if !(((*rme96).rcreg & RME96_RCR_IRQ) != 0 || ((*rme96).rcreg & RME96_RCR_IRQ_2) != 0) { return IRQ_NONE; }
    if ((*rme96).rcreg & RME96_RCR_IRQ) != 0 {
        snd_pcm_period_elapsed((*rme96).playback_substream);
        writel(0, ioadd((*rme96).iobase, RME96_IO_CONFIRM_PLAY_IRQ));
    }
    if ((*rme96).rcreg & RME96_RCR_IRQ_2) != 0 {
        snd_pcm_period_elapsed((*rme96).capture_substream);
        writel(0, ioadd((*rme96).iobase, RME96_IO_CONFIRM_REC_IRQ));
    }
    IRQ_HANDLED
}

static period_bytes: [u32; 2] = [RME96_SMALL_BLOCK_SIZE as u32, RME96_LARGE_BLOCK_SIZE as u32];
static hw_constraints_period_bytes: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list { count: 2, list: period_bytes.as_ptr(), mask: 0 };

unsafe extern "C" fn rme96_set_buffer_size_constraint(rme96: *mut rme96, runtime: *mut snd_pcm_runtime) {
    let mut size: u32;
    snd_pcm_hw_constraint_single(runtime, SNDRV_PCM_HW_PARAM_BUFFER_BYTES, RME96_BUFFER_SIZE);
    size = (*rme96).playback_periodsize as u32;
    if size == 0 { size = (*rme96).capture_periodsize as u32; }
    if size != 0 { snd_pcm_hw_constraint_single(runtime, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, size as c_ulong); }
    else { snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, &hw_constraints_period_bytes); }
}

unsafe extern "C" fn snd_rme96_playback_spdif_open(substream: *mut snd_pcm_substream) -> c_int {
    let mut rate: c_int;
    let mut dummy: c_int = 0;
    let rme96 = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    snd_pcm_set_sync(substream);
    if !(*rme96).playback_substream.is_null() { return -EBUSY; }
    (*rme96).wcreg &= !RME96_WCR_ADAT;
    writel((*rme96).wcreg, ioadd((*rme96).iobase, RME96_IO_CONTROL_REGISTER));
    (*rme96).playback_substream = substream;
    (*runtime).hw = snd_rme96_playback_spdif_info;
    if ((*rme96).wcreg & RME96_WCR_MASTER) == 0 && snd_rme96_getinputtype(rme96) != RME96_INPUT_ANALOG {
        rate = snd_rme96_capture_getrate(rme96, &mut dummy);
        if rate > 0 { (*runtime).hw.rates = snd_pcm_rate_to_rate_bit(rate); (*runtime).hw.rate_min = rate as u32; (*runtime).hw.rate_max = rate as u32; }
    }
    rme96_set_buffer_size_constraint(rme96, runtime);
    (*rme96).wcreg_spdif_stream = (*rme96).wcreg_spdif;
    (*(*rme96).spdif_ctl).vd[0].access &= !SNDRV_CTL_ELEM_ACCESS_INACTIVE;
    snd_ctl_notify((*rme96).card, SNDRV_CTL_EVENT_MASK_VALUE | SNDRV_CTL_EVENT_MASK_INFO, &mut (*(*rme96).spdif_ctl).id);
    0
}
unsafe extern "C" fn snd_rme96_capture_spdif_open(substream: *mut snd_pcm_substream) -> c_int {
    let mut isadat: c_int = 0;
    let rate: c_int;
    let rme96 = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    snd_pcm_set_sync(substream);
    (*runtime).hw = snd_rme96_capture_spdif_info;
    if snd_rme96_getinputtype(rme96) != RME96_INPUT_ANALOG {
        rate = snd_rme96_capture_getrate(rme96, &mut isadat);
        if rate > 0 {
            if isadat != 0 { return -EIO; }
            (*runtime).hw.rates = snd_pcm_rate_to_rate_bit(rate); (*runtime).hw.rate_min = rate as u32; (*runtime).hw.rate_max = rate as u32;
        }
    }
    if !(*rme96).capture_substream.is_null() { return -EBUSY; }
    (*rme96).capture_substream = substream;
    rme96_set_buffer_size_constraint(rme96, runtime);
    0
}
unsafe extern "C" fn snd_rme96_playback_adat_open(substream: *mut snd_pcm_substream) -> c_int {
    let mut rate: c_int;
    let mut dummy: c_int = 0;
    let rme96 = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    snd_pcm_set_sync(substream);
    if !(*rme96).playback_substream.is_null() { return -EBUSY; }
    (*rme96).wcreg |= RME96_WCR_ADAT;
    writel((*rme96).wcreg, ioadd((*rme96).iobase, RME96_IO_CONTROL_REGISTER));
    (*rme96).playback_substream = substream;
    (*runtime).hw = snd_rme96_playback_adat_info;
    if ((*rme96).wcreg & RME96_WCR_MASTER) == 0 && snd_rme96_getinputtype(rme96) != RME96_INPUT_ANALOG {
        rate = snd_rme96_capture_getrate(rme96, &mut dummy);
        if rate > 0 { (*runtime).hw.rates = snd_pcm_rate_to_rate_bit(rate); (*runtime).hw.rate_min = rate as u32; (*runtime).hw.rate_max = rate as u32; }
    }
    rme96_set_buffer_size_constraint(rme96, runtime);
    0
}
unsafe extern "C" fn snd_rme96_capture_adat_open(substream: *mut snd_pcm_substream) -> c_int {
    let mut isadat: c_int = 0;
    let rate: c_int;
    let rme96 = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    snd_pcm_set_sync(substream);
    (*runtime).hw = snd_rme96_capture_adat_info;
    if snd_rme96_getinputtype(rme96) == RME96_INPUT_ANALOG { return -EIO; }
    rate = snd_rme96_capture_getrate(rme96, &mut isadat);
    if rate > 0 {
        if isadat == 0 { return -EIO; }
        (*runtime).hw.rates = snd_pcm_rate_to_rate_bit(rate); (*runtime).hw.rate_min = rate as u32; (*runtime).hw.rate_max = rate as u32;
    }
    if !(*rme96).capture_substream.is_null() { return -EBUSY; }
    (*rme96).capture_substream = substream;
    rme96_set_buffer_size_constraint(rme96, runtime);
    0
}

unsafe extern "C" fn snd_rme96_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    let rme96 = snd_pcm_substream_chip(substream);
    let spdif: c_int;
    if RME96_ISPLAYING(rme96) { snd_rme96_trigger(rme96, RME96_STOP_PLAYBACK); }
    (*rme96).playback_substream = core::ptr::null_mut();
    (*rme96).playback_periodsize = 0;
    spdif = (((*rme96).wcreg & RME96_WCR_ADAT) == 0) as c_int;
    if spdif != 0 {
        (*(*rme96).spdif_ctl).vd[0].access |= SNDRV_CTL_ELEM_ACCESS_INACTIVE;
        snd_ctl_notify((*rme96).card, SNDRV_CTL_EVENT_MASK_VALUE | SNDRV_CTL_EVENT_MASK_INFO, &mut (*(*rme96).spdif_ctl).id);
    }
    0
}
unsafe extern "C" fn snd_rme96_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    let rme96 = snd_pcm_substream_chip(substream);
    if RME96_ISRECORDING(rme96) { snd_rme96_trigger(rme96, RME96_STOP_CAPTURE); }
    (*rme96).capture_substream = core::ptr::null_mut();
    (*rme96).capture_periodsize = 0;
    0
}
unsafe extern "C" fn snd_rme96_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let rme96 = snd_pcm_substream_chip(substream);
    if RME96_ISPLAYING(rme96) { snd_rme96_trigger(rme96, RME96_STOP_PLAYBACK); }
    writel(0, ioadd((*rme96).iobase, RME96_IO_RESET_PLAY_POS)); 0
}
unsafe extern "C" fn snd_rme96_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let rme96 = snd_pcm_substream_chip(substream);
    if RME96_ISRECORDING(rme96) { snd_rme96_trigger(rme96, RME96_STOP_CAPTURE); }
    writel(0, ioadd((*rme96).iobase, RME96_IO_RESET_REC_POS)); 0
}

unsafe extern "C" fn snd_rme96_playback_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let rme96 = snd_pcm_substream_chip(substream);
    /* snd_pcm_group_for_each_entry(s, substream) translated as dependency-specific iteration omitted. */
    let sync = !(*rme96).playback_substream.is_null() && !(*rme96).capture_substream.is_null() && (*(*rme96).playback_substream).group == (*(*rme96).capture_substream).group;
    match cmd {
        SNDRV_PCM_TRIGGER_START => if !RME96_ISPLAYING(rme96) { if substream != (*rme96).playback_substream { return -EBUSY; } snd_rme96_trigger(rme96, if sync { RME96_START_BOTH } else { RME96_START_PLAYBACK }); },
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_STOP => if RME96_ISPLAYING(rme96) { if substream != (*rme96).playback_substream { return -EBUSY; } snd_rme96_trigger(rme96, if sync { RME96_STOP_BOTH } else { RME96_STOP_PLAYBACK }); },
        SNDRV_PCM_TRIGGER_PAUSE_PUSH => if RME96_ISPLAYING(rme96) { snd_rme96_trigger(rme96, if sync { RME96_STOP_BOTH } else { RME96_STOP_PLAYBACK }); },
        SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => if !RME96_ISPLAYING(rme96) { snd_rme96_trigger(rme96, if sync { RME96_RESUME_BOTH } else { RME96_RESUME_PLAYBACK }); },
        _ => return -EINVAL,
    }
    0
}
unsafe extern "C" fn snd_rme96_capture_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let rme96 = snd_pcm_substream_chip(substream);
    /* snd_pcm_group_for_each_entry(s, substream) translated as dependency-specific iteration omitted. */
    let sync = !(*rme96).playback_substream.is_null() && !(*rme96).capture_substream.is_null() && (*(*rme96).playback_substream).group == (*(*rme96).capture_substream).group;
    match cmd {
        SNDRV_PCM_TRIGGER_START => if !RME96_ISRECORDING(rme96) { if substream != (*rme96).capture_substream { return -EBUSY; } snd_rme96_trigger(rme96, if sync { RME96_START_BOTH } else { RME96_START_CAPTURE }); },
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_STOP => if RME96_ISRECORDING(rme96) { if substream != (*rme96).capture_substream { return -EBUSY; } snd_rme96_trigger(rme96, if sync { RME96_STOP_BOTH } else { RME96_STOP_CAPTURE }); },
        SNDRV_PCM_TRIGGER_PAUSE_PUSH => if RME96_ISRECORDING(rme96) { snd_rme96_trigger(rme96, if sync { RME96_STOP_BOTH } else { RME96_STOP_CAPTURE }); },
        SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => if !RME96_ISRECORDING(rme96) { snd_rme96_trigger(rme96, if sync { RME96_RESUME_BOTH } else { RME96_RESUME_CAPTURE }); },
        _ => return -EINVAL,
    }
    0
}
unsafe extern "C" fn snd_rme96_playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let rme96 = snd_pcm_substream_chip(substream);
    snd_rme96_playback_ptr(rme96) as snd_pcm_uframes_t
}
unsafe extern "C" fn snd_rme96_capture_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let rme96 = snd_pcm_substream_chip(substream);
    snd_rme96_capture_ptr(rme96) as snd_pcm_uframes_t
}

static snd_rme96_playback_spdif_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_rme96_playback_spdif_open), close: Some(snd_rme96_playback_close), hw_params: Some(snd_rme96_playback_hw_params), prepare: Some(snd_rme96_playback_prepare), trigger: Some(snd_rme96_playback_trigger), pointer: Some(snd_rme96_playback_pointer), copy: Some(snd_rme96_playback_copy), fill_silence: Some(snd_rme96_playback_silence), mmap: Some(snd_pcm_lib_mmap_iomem) };
static snd_rme96_capture_spdif_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_rme96_capture_spdif_open), close: Some(snd_rme96_capture_close), hw_params: Some(snd_rme96_capture_hw_params), prepare: Some(snd_rme96_capture_prepare), trigger: Some(snd_rme96_capture_trigger), pointer: Some(snd_rme96_capture_pointer), copy: Some(snd_rme96_capture_copy), fill_silence: None, mmap: Some(snd_pcm_lib_mmap_iomem) };
static snd_rme96_playback_adat_ops: snd_pcm_ops = snd_rme96_playback_spdif_ops;
static snd_rme96_capture_adat_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_rme96_capture_adat_open), close: Some(snd_rme96_capture_close), hw_params: Some(snd_rme96_capture_hw_params), prepare: Some(snd_rme96_capture_prepare), trigger: Some(snd_rme96_capture_trigger), pointer: Some(snd_rme96_capture_pointer), copy: Some(snd_rme96_capture_copy), fill_silence: None, mmap: Some(snd_pcm_lib_mmap_iomem) };

unsafe extern "C" fn snd_rme96_free(rme96: *mut rme96) {
    if (*rme96).irq >= 0 {
        snd_rme96_trigger(rme96, RME96_STOP_BOTH);
        (*rme96).areg &= !RME96_AR_DAC_EN;
        writel((*rme96).areg, ioadd((*rme96).iobase, RME96_IO_ADDITIONAL_REG));
    }
    vfree((*rme96).playback_suspend_buffer);
    vfree((*rme96).capture_suspend_buffer);
}
unsafe extern "C" fn snd_rme96_free_spdif_pcm(pcm: *mut snd_pcm) { let rme96 = (*pcm).private_data as *mut rme96; (*rme96).spdif_pcm = core::ptr::null_mut(); }
unsafe extern "C" fn snd_rme96_free_adat_pcm(pcm: *mut snd_pcm) { let rme96 = (*pcm).private_data as *mut rme96; (*rme96).adat_pcm = core::ptr::null_mut(); }

unsafe extern "C" fn snd_rme96_create(rme96: *mut rme96) -> c_int {
    let pci = (*rme96).pci;
    let mut err: c_int;
    (*rme96).irq = -1;
    spin_lock_init(&mut (*rme96).lock);
    err = pcim_enable_device(pci); if err < 0 { return err; }
    err = pcim_request_all_regions(pci, b"RME96\0".as_ptr() as *const c_char); if err < 0 { return err; }
    (*rme96).port = pci_resource_start((*rme96).pci, 0);
    (*rme96).iobase = devm_ioremap(&mut (*pci).dev, (*rme96).port, RME96_IO_SIZE);
    if (*rme96).iobase.is_null() { dev_err((*(*rme96).card).dev, b"unable to remap memory region 0x%lx-0x%lx\n\0".as_ptr() as *const c_char, (*rme96).port, (*rme96).port + RME96_IO_SIZE - 1); return -EBUSY; }
    if devm_request_irq(&mut (*pci).dev, (*pci).irq, snd_rme96_interrupt, IRQF_SHARED, KBUILD_MODNAME, rme96 as *mut c_void) != 0 { dev_err((*(*rme96).card).dev, b"unable to grab IRQ %d\n\0".as_ptr() as *const c_char, (*pci).irq); return -EBUSY; }
    (*rme96).irq = (*pci).irq; (*(*rme96).card).sync_irq = (*rme96).irq;
    pci_read_config_byte(pci, 8, &mut (*rme96).rev);
    err = snd_pcm_new((*rme96).card, b"Digi96 IEC958\0".as_ptr() as *const c_char, 0, 1, 1, &mut (*rme96).spdif_pcm); if err < 0 { return err; }
    (*(*rme96).spdif_pcm).private_data = rme96 as *mut c_void; (*(*rme96).spdif_pcm).private_free = Some(snd_rme96_free_spdif_pcm); strscpy((*(*rme96).spdif_pcm).name.as_mut_ptr(), b"Digi96 IEC958\0".as_ptr() as *const c_char);
    snd_pcm_set_ops((*rme96).spdif_pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_rme96_playback_spdif_ops);
    snd_pcm_set_ops((*rme96).spdif_pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_rme96_capture_spdif_ops);
    (*(*rme96).spdif_pcm).info_flags = 0;
    if (*pci).device == PCI_DEVICE_ID_RME_DIGI96 { (*rme96).adat_pcm = core::ptr::null_mut(); } else {
        err = snd_pcm_new((*rme96).card, b"Digi96 ADAT\0".as_ptr() as *const c_char, 1, 1, 1, &mut (*rme96).adat_pcm); if err < 0 { return err; }
        (*(*rme96).adat_pcm).private_data = rme96 as *mut c_void; (*(*rme96).adat_pcm).private_free = Some(snd_rme96_free_adat_pcm); strscpy((*(*rme96).adat_pcm).name.as_mut_ptr(), b"Digi96 ADAT\0".as_ptr() as *const c_char);
        snd_pcm_set_ops((*rme96).adat_pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_rme96_playback_adat_ops);
        snd_pcm_set_ops((*rme96).adat_pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_rme96_capture_adat_ops);
        (*(*rme96).adat_pcm).info_flags = 0;
    }
    (*rme96).playback_periodsize = 0; (*rme96).capture_periodsize = 0;
    snd_rme96_trigger(rme96, RME96_STOP_BOTH);
    (*rme96).wcreg = RME96_WCR_FREQ_1 | RME96_WCR_SEL | RME96_WCR_MASTER | RME96_WCR_INP_0;
    (*rme96).areg = RME96_AR_FREQPAD_1;
    writel((*rme96).wcreg, ioadd((*rme96).iobase, RME96_IO_CONTROL_REGISTER));
    writel((*rme96).areg, ioadd((*rme96).iobase, RME96_IO_ADDITIONAL_REG));
    writel((*rme96).areg | RME96_AR_PD2, ioadd((*rme96).iobase, RME96_IO_ADDITIONAL_REG));
    writel((*rme96).areg, ioadd((*rme96).iobase, RME96_IO_ADDITIONAL_REG));
    snd_rme96_reset_dac(rme96);
    (*rme96).areg |= RME96_AR_DAC_EN;
    writel((*rme96).areg, ioadd((*rme96).iobase, RME96_IO_ADDITIONAL_REG));
    writel(0, ioadd((*rme96).iobase, RME96_IO_RESET_PLAY_POS));
    writel(0, ioadd((*rme96).iobase, RME96_IO_RESET_REC_POS));
    (*rme96).vol[1] = 0; (*rme96).vol[0] = (*rme96).vol[1];
    if RME96_HAS_ANALOG_OUT(rme96) { snd_rme96_apply_dac_volume(rme96); }
    err = snd_rme96_create_switches((*rme96).card, rme96); if err < 0 { return err; }
    snd_rme96_proc_init(rme96);
    0
}

unsafe extern "C" fn snd_rme96_proc_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let mut n: c_int = 0;
    let rme96 = (*entry).private_data as *mut rme96;
    (*rme96).rcreg = readl(ioadd((*rme96).iobase, RME96_IO_CONTROL_REGISTER));
    snd_iprintf(buffer, (*(*rme96).card).longname.as_ptr());
    snd_iprintf(buffer, b" (index #%d)\n\0".as_ptr() as *const c_char, (*(*rme96).card).number + 1);
    snd_iprintf(buffer, b"\nGeneral settings\n\0".as_ptr() as *const c_char);
    if ((*rme96).wcreg & RME96_WCR_IDIS) != 0 { snd_iprintf(buffer, b"  period size: N/A (interrupts disabled)\n\0".as_ptr() as *const c_char); }
    else if ((*rme96).wcreg & RME96_WCR_ISEL) != 0 { snd_iprintf(buffer, b"  period size: 2048 bytes\n\0".as_ptr() as *const c_char); }
    else { snd_iprintf(buffer, b"  period size: 8192 bytes\n\0".as_ptr() as *const c_char); }
    snd_iprintf(buffer, b"\nInput settings\n\0".as_ptr() as *const c_char);
    match snd_rme96_getinputtype(rme96) {
        RME96_INPUT_OPTICAL => snd_iprintf(buffer, b"  input: optical\0".as_ptr() as *const c_char),
        RME96_INPUT_COAXIAL => snd_iprintf(buffer, b"  input: coaxial\0".as_ptr() as *const c_char),
        RME96_INPUT_INTERNAL => snd_iprintf(buffer, b"  input: internal\0".as_ptr() as *const c_char),
        RME96_INPUT_XLR => snd_iprintf(buffer, b"  input: XLR\0".as_ptr() as *const c_char),
        RME96_INPUT_ANALOG => snd_iprintf(buffer, b"  input: analog\0".as_ptr() as *const c_char),
        _ => (),
    }
    if snd_rme96_capture_getrate(rme96, &mut n) < 0 { snd_iprintf(buffer, b"\n  sample rate: no valid signal\n\0".as_ptr() as *const c_char); }
    else {
        if n != 0 { snd_iprintf(buffer, b" (8 channels)\n\0".as_ptr() as *const c_char); } else { snd_iprintf(buffer, b" (2 channels)\n\0".as_ptr() as *const c_char); }
        snd_iprintf(buffer, b"  sample rate: %d Hz\n\0".as_ptr() as *const c_char, snd_rme96_capture_getrate(rme96, &mut n));
    }
    if ((*rme96).wcreg & RME96_WCR_MODE24_2) != 0 { snd_iprintf(buffer, b"  sample format: 24 bit\n\0".as_ptr() as *const c_char); } else { snd_iprintf(buffer, b"  sample format: 16 bit\n\0".as_ptr() as *const c_char); }
    snd_iprintf(buffer, b"\nOutput settings\n\0".as_ptr() as *const c_char);
    if ((*rme96).wcreg & RME96_WCR_SEL) != 0 { snd_iprintf(buffer, b"  output signal: normal playback\n\0".as_ptr() as *const c_char); } else { snd_iprintf(buffer, b"  output signal: same as input\n\0".as_ptr() as *const c_char); }
    snd_iprintf(buffer, b"  sample rate: %d Hz\n\0".as_ptr() as *const c_char, snd_rme96_playback_getrate(rme96));
    if ((*rme96).wcreg & RME96_WCR_MODE24) != 0 { snd_iprintf(buffer, b"  sample format: 24 bit\n\0".as_ptr() as *const c_char); } else { snd_iprintf(buffer, b"  sample format: 16 bit\n\0".as_ptr() as *const c_char); }
    if ((*rme96).areg & RME96_AR_WSEL) != 0 { snd_iprintf(buffer, b"  sample clock source: word clock\n\0".as_ptr() as *const c_char); }
    else if ((*rme96).wcreg & RME96_WCR_MASTER) != 0 { snd_iprintf(buffer, b"  sample clock source: internal\n\0".as_ptr() as *const c_char); }
    else if snd_rme96_getinputtype(rme96) == RME96_INPUT_ANALOG { snd_iprintf(buffer, b"  sample clock source: autosync (internal anyway due to analog input setting)\n\0".as_ptr() as *const c_char); }
    else if snd_rme96_capture_getrate(rme96, &mut n) < 0 { snd_iprintf(buffer, b"  sample clock source: autosync (internal anyway due to no valid signal)\n\0".as_ptr() as *const c_char); }
    else { snd_iprintf(buffer, b"  sample clock source: autosync\n\0".as_ptr() as *const c_char); }
    if ((*rme96).wcreg & RME96_WCR_PRO) != 0 { snd_iprintf(buffer, b"  format: AES/EBU (professional)\n\0".as_ptr() as *const c_char); } else { snd_iprintf(buffer, b"  format: IEC958 (consumer)\n\0".as_ptr() as *const c_char); }
    if ((*rme96).wcreg & RME96_WCR_EMP) != 0 { snd_iprintf(buffer, b"  emphasis: on\n\0".as_ptr() as *const c_char); } else { snd_iprintf(buffer, b"  emphasis: off\n\0".as_ptr() as *const c_char); }
    if ((*rme96).wcreg & RME96_WCR_DOLBY) != 0 { snd_iprintf(buffer, b"  non-audio (dolby): on\n\0".as_ptr() as *const c_char); } else { snd_iprintf(buffer, b"  non-audio (dolby): off\n\0".as_ptr() as *const c_char); }
    if RME96_HAS_ANALOG_IN(rme96) {
        snd_iprintf(buffer, b"\nAnalog output settings\n\0".as_ptr() as *const c_char);
        match snd_rme96_getmontracks(rme96) {
            RME96_MONITOR_TRACKS_1_2 => snd_iprintf(buffer, b"  monitored ADAT tracks: 1+2\n\0".as_ptr() as *const c_char),
            RME96_MONITOR_TRACKS_3_4 => snd_iprintf(buffer, b"  monitored ADAT tracks: 3+4\n\0".as_ptr() as *const c_char),
            RME96_MONITOR_TRACKS_5_6 => snd_iprintf(buffer, b"  monitored ADAT tracks: 5+6\n\0".as_ptr() as *const c_char),
            RME96_MONITOR_TRACKS_7_8 => snd_iprintf(buffer, b"  monitored ADAT tracks: 7+8\n\0".as_ptr() as *const c_char),
            _ => (),
        }
        match snd_rme96_getattenuation(rme96) {
            RME96_ATTENUATION_0 => snd_iprintf(buffer, b"  attenuation: 0 dB\n\0".as_ptr() as *const c_char),
            RME96_ATTENUATION_6 => snd_iprintf(buffer, b"  attenuation: -6 dB\n\0".as_ptr() as *const c_char),
            RME96_ATTENUATION_12 => snd_iprintf(buffer, b"  attenuation: -12 dB\n\0".as_ptr() as *const c_char),
            RME96_ATTENUATION_18 => snd_iprintf(buffer, b"  attenuation: -18 dB\n\0".as_ptr() as *const c_char),
            _ => (),
        }
        snd_iprintf(buffer, b"  volume left: %u\n\0".as_ptr() as *const c_char, (*rme96).vol[0] as c_uint);
        snd_iprintf(buffer, b"  volume right: %u\n\0".as_ptr() as *const c_char, (*rme96).vol[1] as c_uint);
    }
}
unsafe extern "C" fn snd_rme96_proc_init(rme96: *mut rme96) { snd_card_ro_proc_new((*rme96).card, b"rme96\0".as_ptr() as *const c_char, rme96, snd_rme96_proc_read); }

unsafe extern "C" fn snd_rme96_info_loopback_control(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_info) -> c_int { snd_ctl_boolean_mono_info(k, u) }
unsafe extern "C" fn snd_rme96_get_loopback_control(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rme96 = snd_kcontrol_chip(kcontrol);
    (*ucontrol).value.integer.value[0] = if ((*rme96).wcreg & RME96_WCR_SEL) != 0 { 0 } else { 1 };
    0
}
unsafe extern "C" fn snd_rme96_put_loopback_control(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rme96 = snd_kcontrol_chip(kcontrol);
    let mut val: u32 = if (*ucontrol).value.integer.value[0] != 0 { 0 } else { RME96_WCR_SEL };
    val = ((*rme96).wcreg & !RME96_WCR_SEL) | val;
    let change = (val != (*rme96).wcreg) as c_int;
    (*rme96).wcreg = val;
    writel(val, ioadd((*rme96).iobase, RME96_IO_CONTROL_REGISTER));
    change
}

static TEXT_OPTICAL: &[u8] = b"Optical\0"; static TEXT_COAXIAL: &[u8] = b"Coaxial\0"; static TEXT_INTERNAL: &[u8] = b"Internal\0"; static TEXT_XLR: &[u8] = b"XLR\0"; static TEXT_ANALOG: &[u8] = b"Analog\0";
unsafe extern "C" fn snd_rme96_info_inputtype_control(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let rme96 = snd_kcontrol_chip(kcontrol);
    let mut texts = [TEXT_OPTICAL.as_ptr() as *const c_char, TEXT_COAXIAL.as_ptr() as *const c_char, TEXT_INTERNAL.as_ptr() as *const c_char, TEXT_XLR.as_ptr() as *const c_char, TEXT_ANALOG.as_ptr() as *const c_char];
    let num_items: c_int;
    match (*(*rme96).pci).device {
        PCI_DEVICE_ID_RME_DIGI96 | PCI_DEVICE_ID_RME_DIGI96_8 => num_items = 3,
        PCI_DEVICE_ID_RME_DIGI96_8_PRO => num_items = 4,
        PCI_DEVICE_ID_RME_DIGI96_8_PAD_OR_PST => { if (*rme96).rev > 4 { num_items = 4; texts[3] = texts[4]; } else { num_items = 5; } }
        _ => { snd_BUG(); return -EINVAL; }
    }
    snd_ctl_enum_info(uinfo, 1, num_items as c_uint, texts.as_ptr())
}
unsafe extern "C" fn snd_rme96_get_inputtype_control(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rme96 = snd_kcontrol_chip(kcontrol);
    let mut items: c_uint = 3;
    (*ucontrol).value.enumerated.item[0] = snd_rme96_getinputtype(rme96) as u32;
    match (*(*rme96).pci).device {
        PCI_DEVICE_ID_RME_DIGI96 | PCI_DEVICE_ID_RME_DIGI96_8 => items = 3,
        PCI_DEVICE_ID_RME_DIGI96_8_PRO => items = 4,
        PCI_DEVICE_ID_RME_DIGI96_8_PAD_OR_PST => if (*rme96).rev > 4 { if (*ucontrol).value.enumerated.item[0] == RME96_INPUT_ANALOG as u32 { (*ucontrol).value.enumerated.item[0] = RME96_INPUT_XLR as u32; } items = 4; } else { items = 5; },
        _ => snd_BUG(),
    }
    if (*ucontrol).value.enumerated.item[0] >= items { (*ucontrol).value.enumerated.item[0] = items - 1; }
    0
}
unsafe extern "C" fn snd_rme96_put_inputtype_control(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rme96 = snd_kcontrol_chip(kcontrol);
    let mut items: c_int = 3;
    match (*(*rme96).pci).device {
        PCI_DEVICE_ID_RME_DIGI96 | PCI_DEVICE_ID_RME_DIGI96_8 => items = 3,
        PCI_DEVICE_ID_RME_DIGI96_8_PRO => items = 4,
        PCI_DEVICE_ID_RME_DIGI96_8_PAD_OR_PST => if (*rme96).rev > 4 { items = 4; } else { items = 5; },
        _ => snd_BUG(),
    }
    let mut val = ((*ucontrol).value.enumerated.item[0] % items as u32) as c_int;
    if (*(*rme96).pci).device == PCI_DEVICE_ID_RME_DIGI96_8_PAD_OR_PST && (*rme96).rev > 4 && val == RME96_INPUT_XLR { val = RME96_INPUT_ANALOG; }
    let change = (val != snd_rme96_getinputtype(rme96)) as c_int;
    snd_rme96_setinputtype(rme96, val);
    change
}
unsafe extern "C" fn snd_rme96_info_clockmode_control(_k: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let texts = [b"AutoSync\0".as_ptr() as *const c_char, b"Internal\0".as_ptr() as *const c_char, b"Word\0".as_ptr() as *const c_char];
    snd_ctl_enum_info(uinfo, 1, 3, texts.as_ptr())
}
unsafe extern "C" fn snd_rme96_get_clockmode_control(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let rme96 = snd_kcontrol_chip(kcontrol); (*ucontrol).value.enumerated.item[0] = snd_rme96_getclockmode(rme96) as u32; 0 }
unsafe extern "C" fn snd_rme96_put_clockmode_control(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let rme96 = snd_kcontrol_chip(kcontrol); let val = ((*ucontrol).value.enumerated.item[0] % 3) as c_int; let change = (val != snd_rme96_getclockmode(rme96)) as c_int; snd_rme96_setclockmode(rme96, val); change }
unsafe extern "C" fn snd_rme96_info_attenuation_control(_k: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int { let texts = [b"0 dB\0".as_ptr() as *const c_char, b"-6 dB\0".as_ptr() as *const c_char, b"-12 dB\0".as_ptr() as *const c_char, b"-18 dB\0".as_ptr() as *const c_char]; snd_ctl_enum_info(uinfo, 1, 4, texts.as_ptr()) }
unsafe extern "C" fn snd_rme96_get_attenuation_control(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let rme96 = snd_kcontrol_chip(kcontrol); (*ucontrol).value.enumerated.item[0] = snd_rme96_getattenuation(rme96) as u32; 0 }
unsafe extern "C" fn snd_rme96_put_attenuation_control(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let rme96 = snd_kcontrol_chip(kcontrol); let val = ((*ucontrol).value.enumerated.item[0] % 4) as c_int; let change = (val != snd_rme96_getattenuation(rme96)) as c_int; snd_rme96_setattenuation(rme96, val); change }
unsafe extern "C" fn snd_rme96_info_montracks_control(_k: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int { let texts = [b"1+2\0".as_ptr() as *const c_char, b"3+4\0".as_ptr() as *const c_char, b"5+6\0".as_ptr() as *const c_char, b"7+8\0".as_ptr() as *const c_char]; snd_ctl_enum_info(uinfo, 1, 4, texts.as_ptr()) }
unsafe extern "C" fn snd_rme96_get_montracks_control(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let rme96 = snd_kcontrol_chip(kcontrol); (*ucontrol).value.enumerated.item[0] = snd_rme96_getmontracks(rme96) as u32; 0 }
unsafe extern "C" fn snd_rme96_put_montracks_control(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let rme96 = snd_kcontrol_chip(kcontrol); let val = ((*ucontrol).value.enumerated.item[0] % 4) as c_int; let change = (val != snd_rme96_getmontracks(rme96)) as c_int; snd_rme96_setmontracks(rme96, val); change }

unsafe extern "C" fn snd_rme96_convert_from_aes(aes: *mut snd_aes_iec958) -> u32 {
    let mut val: u32 = 0;
    val |= if ((*aes).status[0] & IEC958_AES0_PROFESSIONAL) != 0 { RME96_WCR_PRO } else { 0 };
    val |= if ((*aes).status[0] & IEC958_AES0_NONAUDIO) != 0 { RME96_WCR_DOLBY } else { 0 };
    if (val & RME96_WCR_PRO) != 0 { val |= if ((*aes).status[0] & IEC958_AES0_PRO_EMPHASIS_5015) != 0 { RME96_WCR_EMP } else { 0 }; }
    else { val |= if ((*aes).status[0] & IEC958_AES0_CON_EMPHASIS_5015) != 0 { RME96_WCR_EMP } else { 0 }; }
    val
}
unsafe extern "C" fn snd_rme96_convert_to_aes(aes: *mut snd_aes_iec958, val: u32) {
    (*aes).status[0] = (if (val & RME96_WCR_PRO) != 0 { IEC958_AES0_PROFESSIONAL } else { 0 }) | (if (val & RME96_WCR_DOLBY) != 0 { IEC958_AES0_NONAUDIO } else { 0 });
    if (val & RME96_WCR_PRO) != 0 { (*aes).status[0] |= if (val & RME96_WCR_EMP) != 0 { IEC958_AES0_PRO_EMPHASIS_5015 } else { 0 }; }
    else { (*aes).status[0] |= if (val & RME96_WCR_EMP) != 0 { IEC958_AES0_CON_EMPHASIS_5015 } else { 0 }; }
}
unsafe extern "C" fn snd_rme96_control_spdif_info(_k: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int { (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958; (*uinfo).count = 1; 0 }
unsafe extern "C" fn snd_rme96_control_spdif_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let rme96 = snd_kcontrol_chip(kcontrol); snd_rme96_convert_to_aes(&mut (*ucontrol).value.iec958, (*rme96).wcreg_spdif); 0 }
unsafe extern "C" fn snd_rme96_control_spdif_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let rme96 = snd_kcontrol_chip(kcontrol); let val = snd_rme96_convert_from_aes(&mut (*ucontrol).value.iec958); let change = (val != (*rme96).wcreg_spdif) as c_int; (*rme96).wcreg_spdif = val; change }
unsafe extern "C" fn snd_rme96_control_spdif_stream_info(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_info) -> c_int { snd_rme96_control_spdif_info(k, u) }
unsafe extern "C" fn snd_rme96_control_spdif_stream_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let rme96 = snd_kcontrol_chip(kcontrol); snd_rme96_convert_to_aes(&mut (*ucontrol).value.iec958, (*rme96).wcreg_spdif_stream); 0 }
unsafe extern "C" fn snd_rme96_control_spdif_stream_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let rme96 = snd_kcontrol_chip(kcontrol); let val = snd_rme96_convert_from_aes(&mut (*ucontrol).value.iec958); let change = (val != (*rme96).wcreg_spdif_stream) as c_int; (*rme96).wcreg_spdif_stream = val; (*rme96).wcreg &= !(RME96_WCR_PRO | RME96_WCR_DOLBY | RME96_WCR_EMP); (*rme96).wcreg |= val; writel((*rme96).wcreg, ioadd((*rme96).iobase, RME96_IO_CONTROL_REGISTER)); change }
unsafe extern "C" fn snd_rme96_control_spdif_mask_info(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_info) -> c_int { snd_rme96_control_spdif_info(k, u) }
unsafe extern "C" fn snd_rme96_control_spdif_mask_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { (*ucontrol).value.iec958.status[0] = (*kcontrol).private_value as u8; 0 }
unsafe extern "C" fn snd_rme96_dac_volume_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int { let rme96 = snd_kcontrol_chip(kcontrol); (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER; (*uinfo).count = 2; (*uinfo).value.integer.min = 0; (*uinfo).value.integer.max = RME96_185X_MAX_OUT(rme96) as i64; 0 }
unsafe extern "C" fn snd_rme96_dac_volume_get(kcontrol: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let rme96 = snd_kcontrol_chip(kcontrol); (*u).value.integer.value[0] = (*rme96).vol[0] as i64; (*u).value.integer.value[1] = (*rme96).vol[1] as i64; 0 }
unsafe extern "C" fn snd_rme96_dac_volume_put(kcontrol: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int {
    let rme96 = snd_kcontrol_chip(kcontrol);
    let mut change = 0;
    if !RME96_HAS_ANALOG_OUT(rme96) { return -EINVAL; }
    let maxvol = RME96_185X_MAX_OUT(rme96);
    let mut vol = (*u).value.integer.value[0] as u32;
    if vol != (*rme96).vol[0] as u32 && vol <= maxvol { (*rme96).vol[0] = vol as u16; change = 1; }
    vol = (*u).value.integer.value[1] as u32;
    if vol != (*rme96).vol[1] as u32 && vol <= maxvol { (*rme96).vol[1] = vol as u16; change = 1; }
    if change != 0 { snd_rme96_apply_dac_volume(rme96); }
    change
}

static snd_rme96_controls: [snd_kcontrol_new; 10] = [
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_PCM, name: b"IEC958 Playback Default\0".as_ptr() as *const c_char, access: 0, info: Some(snd_rme96_control_spdif_info), get: Some(snd_rme96_control_spdif_get), put: Some(snd_rme96_control_spdif_put), private_value: 0 },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_PCM, name: b"IEC958 Playback PCM Stream\0".as_ptr() as *const c_char, access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_INACTIVE, info: Some(snd_rme96_control_spdif_stream_info), get: Some(snd_rme96_control_spdif_stream_get), put: Some(snd_rme96_control_spdif_stream_put), private_value: 0 },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_PCM, name: b"IEC958 Playback Con Mask\0".as_ptr() as *const c_char, access: SNDRV_CTL_ELEM_ACCESS_READ, info: Some(snd_rme96_control_spdif_mask_info), get: Some(snd_rme96_control_spdif_mask_get), put: None, private_value: IEC958_AES0_NONAUDIO as c_ulong | IEC958_AES0_PROFESSIONAL as c_ulong | IEC958_AES0_CON_EMPHASIS },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_PCM, name: b"IEC958 Playback Pro Mask\0".as_ptr() as *const c_char, access: SNDRV_CTL_ELEM_ACCESS_READ, info: Some(snd_rme96_control_spdif_mask_info), get: Some(snd_rme96_control_spdif_mask_get), put: None, private_value: IEC958_AES0_NONAUDIO as c_ulong | IEC958_AES0_PROFESSIONAL as c_ulong | IEC958_AES0_PRO_EMPHASIS },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Input Connector\0".as_ptr() as *const c_char, access: 0, info: Some(snd_rme96_info_inputtype_control), get: Some(snd_rme96_get_inputtype_control), put: Some(snd_rme96_put_inputtype_control), private_value: 0 },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Loopback Input\0".as_ptr() as *const c_char, access: 0, info: Some(snd_rme96_info_loopback_control), get: Some(snd_rme96_get_loopback_control), put: Some(snd_rme96_put_loopback_control), private_value: 0 },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Sample Clock Source\0".as_ptr() as *const c_char, access: 0, info: Some(snd_rme96_info_clockmode_control), get: Some(snd_rme96_get_clockmode_control), put: Some(snd_rme96_put_clockmode_control), private_value: 0 },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Monitor Tracks\0".as_ptr() as *const c_char, access: 0, info: Some(snd_rme96_info_montracks_control), get: Some(snd_rme96_get_montracks_control), put: Some(snd_rme96_put_montracks_control), private_value: 0 },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Attenuation\0".as_ptr() as *const c_char, access: 0, info: Some(snd_rme96_info_attenuation_control), get: Some(snd_rme96_get_attenuation_control), put: Some(snd_rme96_put_attenuation_control), private_value: 0 },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"DAC Playback Volume\0".as_ptr() as *const c_char, access: 0, info: Some(snd_rme96_dac_volume_info), get: Some(snd_rme96_dac_volume_get), put: Some(snd_rme96_dac_volume_put), private_value: 0 },
];

unsafe extern "C" fn snd_rme96_create_switches(card: *mut snd_card, rme96: *mut rme96) -> c_int {
    let mut idx = 0;
    let mut err: c_int;
    let mut kctl: *mut snd_kcontrol;
    while idx < 7 {
        kctl = snd_ctl_new1(&snd_rme96_controls[idx], rme96);
        err = snd_ctl_add(card, kctl);
        if err < 0 { return err; }
        if idx == 1 { (*rme96).spdif_ctl = kctl; }
        idx += 1;
    }
    if RME96_HAS_ANALOG_OUT(rme96) {
        idx = 7;
        while idx < 10 {
            err = snd_ctl_add(card, snd_ctl_new1(&snd_rme96_controls[idx], rme96));
            if err < 0 { return err; }
            idx += 1;
        }
    }
    0
}

unsafe extern "C" fn rme96_suspend(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let rme96 = (*card).private_data as *mut rme96;
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    (*rme96).playback_pointer = readl(ioadd((*rme96).iobase, RME96_IO_GET_PLAY_POS)) & RME96_RCR_AUDIO_ADDR_MASK;
    (*rme96).capture_pointer = readl(ioadd((*rme96).iobase, RME96_IO_GET_REC_POS)) & RME96_RCR_AUDIO_ADDR_MASK;
    memcpy_fromio((*rme96).playback_suspend_buffer, ioadd((*rme96).iobase, RME96_IO_PLAY_BUFFER), RME96_BUFFER_SIZE);
    memcpy_fromio((*rme96).capture_suspend_buffer, ioadd((*rme96).iobase, RME96_IO_REC_BUFFER), RME96_BUFFER_SIZE);
    (*rme96).areg &= !RME96_AR_DAC_EN;
    writel((*rme96).areg, ioadd((*rme96).iobase, RME96_IO_ADDITIONAL_REG));
    0
}
unsafe extern "C" fn rme96_resume(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let rme96 = (*card).private_data as *mut rme96;
    writel(0, ioadd((*rme96).iobase, RME96_IO_SET_PLAY_POS + (*rme96).playback_pointer as c_ulong));
    writel(0, ioadd((*rme96).iobase, RME96_IO_SET_REC_POS + (*rme96).capture_pointer as c_ulong));
    memcpy_toio(ioadd((*rme96).iobase, RME96_IO_PLAY_BUFFER), (*rme96).playback_suspend_buffer, RME96_BUFFER_SIZE);
    memcpy_toio(ioadd((*rme96).iobase, RME96_IO_REC_BUFFER), (*rme96).capture_suspend_buffer, RME96_BUFFER_SIZE);
    writel((*rme96).areg | RME96_AR_PD2, ioadd((*rme96).iobase, RME96_IO_ADDITIONAL_REG));
    writel((*rme96).areg, ioadd((*rme96).iobase, RME96_IO_ADDITIONAL_REG));
    snd_rme96_reset_dac(rme96);
    (*rme96).areg |= RME96_AR_DAC_EN;
    writel((*rme96).areg, ioadd((*rme96).iobase, RME96_IO_ADDITIONAL_REG));
    if RME96_HAS_ANALOG_OUT(rme96) { usleep_range(3000, 10000); snd_rme96_apply_dac_volume(rme96); }
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}
/* static DEFINE_SIMPLE_DEV_PM_OPS(rme96_pm, rme96_suspend, rme96_resume); */

unsafe extern "C" fn snd_rme96_card_free(card: *mut snd_card) { snd_rme96_free((*card).private_data as *mut rme96); }

unsafe extern "C" fn __snd_rme96_probe(pci: *mut pci_dev, _pci_id: *const pci_device_id) -> c_int {
    static mut dev: c_int = 0;
    let mut rme96: *mut rme96;
    let mut card: *mut snd_card = core::ptr::null_mut();
    let mut err: c_int;
    let mut val: u8 = 0;
    if dev >= 32 { return -ENODEV; }
    if !enable[dev as usize] { dev += 1; return -ENOENT; }
    err = snd_devm_card_new(&mut (*pci).dev, index[dev as usize], id[dev as usize], THIS_MODULE, core::mem::size_of::<rme96>(), &mut card);
    if err < 0 { return err; }
    (*card).private_free = Some(snd_rme96_card_free);
    rme96 = (*card).private_data as *mut rme96;
    (*rme96).card = card;
    (*rme96).pci = pci;
    err = snd_rme96_create(rme96);
    if err != 0 { return err; }
    /* IS_ENABLED(CONFIG_PM_SLEEP) */
    {
        (*rme96).playback_suspend_buffer = vmalloc(RME96_BUFFER_SIZE);
        if (*rme96).playback_suspend_buffer.is_null() { return -ENOMEM; }
        (*rme96).capture_suspend_buffer = vmalloc(RME96_BUFFER_SIZE);
        if (*rme96).capture_suspend_buffer.is_null() { return -ENOMEM; }
    }
    strscpy((*card).driver.as_mut_ptr(), b"Digi96\0".as_ptr() as *const c_char);
    match (*(*rme96).pci).device {
        PCI_DEVICE_ID_RME_DIGI96 => { strscpy((*card).shortname.as_mut_ptr(), b"RME Digi96\0".as_ptr() as *const c_char); }
        PCI_DEVICE_ID_RME_DIGI96_8 => { strscpy((*card).shortname.as_mut_ptr(), b"RME Digi96/8\0".as_ptr() as *const c_char); }
        PCI_DEVICE_ID_RME_DIGI96_8_PRO => { strscpy((*card).shortname.as_mut_ptr(), b"RME Digi96/8 PRO\0".as_ptr() as *const c_char); }
        PCI_DEVICE_ID_RME_DIGI96_8_PAD_OR_PST => {
            pci_read_config_byte((*rme96).pci, 8, &mut val);
            if val < 5 { strscpy((*card).shortname.as_mut_ptr(), b"RME Digi96/8 PAD\0".as_ptr() as *const c_char); }
            else { strscpy((*card).shortname.as_mut_ptr(), b"RME Digi96/8 PST\0".as_ptr() as *const c_char); }
        }
        _ => (),
    }
    sprintf((*card).longname.as_mut_ptr(), b"%s at 0x%lx, irq %d\0".as_ptr() as *const c_char, (*card).shortname.as_ptr(), (*rme96).port, (*rme96).irq);
    err = snd_card_register(card);
    if err != 0 { return err; }
    pci_set_drvdata(pci, card as *mut c_void);
    dev += 1;
    0
}

unsafe extern "C" fn snd_rme96_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    snd_card_free_on_error(&mut (*pci).dev, __snd_rme96_probe(pci, pci_id))
}

static mut rme96_driver: pci_driver = pci_driver {
    name: core::ptr::null(),
    id_table: snd_rme96_ids.as_ptr(),
    probe: Some(snd_rme96_probe),
    driver: device_driver { pm: unsafe { &rme96_pm as *const dev_pm_ops } },
};

/* module_pci_driver(rme96_driver); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
