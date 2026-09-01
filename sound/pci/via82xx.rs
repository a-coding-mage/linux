// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for VIA VT82xx (South Bridge)
 *
 *   VT82C686A/B/C, VT8233A/C, VT8235
 *
 *	Copyright (c) 2000 Jaroslav Kysela <perex@perex.cz>
 *	                   Tjeerd.Mulder <Tjeerd.Mulder@fujitsu-siemens.com>
 *                    2002 Takashi Iwai <tiwai@suse.de>
 */

/*
 * Changes:
 *
 * Dec. 19, 2002	Takashi Iwai <tiwai@suse.de>
 *	- use the DSX channels for the first pcm playback.
 *	  (on VIA8233, 8233C and 8235 only)
 *	  this will allow you play simultaneously up to 4 streams.
 *	  multi-channel playback is assigned to the second device
 *	  on these chips.
 *	- support the secondary capture (on VIA8233/C,8235)
 *	- SPDIF support
 *	  the DSX3 channel can be used for SPDIF output.
 *	  on VIA8233A, this channel is assigned to the second pcm
 *	  playback.
 *	  the card config of alsa-lib will assign the correct
 *	  device for applications.
 *	- clean up the code, separate low-level initialization
 *	  routines for each chipset.
 *
 * Sep. 26, 2005	Karsten Wiese <annabellesgarden@yahoo.de>
 *	- Optimize position calculation for the 823x chips.
 */

/* C include dependencies:
 * linux/io.h, linux/delay.h, linux/interrupt.h, linux/init.h, linux/pci.h,
 * linux/slab.h, linux/gameport.h, linux/module.h, sound/core.h, sound/pcm.h,
 * sound/pcm_params.h, sound/info.h, sound/tlv.h, sound/ac97_codec.h,
 * sound/mpu401.h, sound/initval.h
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type u8 = u8;
type u32 = u32;
type __le32 = u32;
type bool_ = bool;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;
type spinlock_t = c_ulong;

const NULL: usize = 0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pci_dev {
    pub dev: device,
    pub irq: c_int,
    pub revision: c_uint,
}
#[repr(C)]
pub struct pci_device_id {
    pub vendor: c_uint,
    pub device: c_uint,
    pub subvendor: c_uint,
    pub subdevice: c_uint,
    pub class: c_uint,
    pub class_mask: c_uint,
    pub driver_data: c_ulong,
}
#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub private_data: *mut c_void,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
    pub sync_irq: c_int,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
}
#[repr(C)]
pub struct snd_pcm {
    pub private_data: *mut c_void,
    pub name: [c_char; 80],
    pub device: c_int,
}
#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
    pub private_data: *mut c_void,
    pub format: c_int,
    pub channels: c_uint,
    pub rate: c_uint,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub number: c_uint,
    pub pcm: *mut snd_pcm,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: c_ulong,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub buffer_bytes_max: c_ulong,
    pub period_bytes_min: c_ulong,
    pub period_bytes_max: c_ulong,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub fifo_size: c_uint,
}
#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub ioctl: *const c_void,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}
#[repr(C)]
pub struct snd_dma_buffer {
    pub area: *mut c_void,
    pub addr: c_ulong,
}
#[repr(C)]
pub struct snd_rawmidi {
    pub private_data: *mut c_void,
}
#[repr(C)]
pub struct snd_kcontrol_id {
    pub index: c_uint,
    pub subdevice: c_uint,
}
#[repr(C)]
pub struct snd_kcontrol_volatile {
    pub access: c_uint,
}
#[repr(C)]
pub struct snd_kcontrol {
    pub id: snd_kcontrol_id,
    pub vd: [snd_kcontrol_volatile; 1],
}
#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub device: c_uint,
    pub subdevice: c_uint,
    pub name: *const c_char,
    pub index: c_uint,
    pub access: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub tlv: snd_kcontrol_tlv,
}
#[repr(C)]
pub union snd_kcontrol_tlv {
    pub p: *const c_uint,
}
#[repr(C)]
pub struct snd_ctl_elem_info_integer {
    pub min: c_long,
    pub max: c_long,
}
#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}
#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}
#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}
#[repr(C)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}
#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}
#[repr(C)]
pub struct snd_ac97_bus {
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_ac97_bus)>,
    pub clock: c_uint,
}
#[repr(C)]
pub struct snd_ac97 {
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_ac97)>,
    pub num: c_int,
    pub rates: [c_uint; 8],
    pub chmaps: [*mut snd_pcm_chmap; 2],
}
#[repr(C)]
pub struct snd_ac97_bus_ops {
    pub write: Option<unsafe extern "C" fn(*mut snd_ac97, u16, u16)>,
    pub read: Option<unsafe extern "C" fn(*mut snd_ac97, u16) -> u16>,
    pub wait: Option<unsafe extern "C" fn(*mut snd_ac97)>,
}
#[repr(C)]
pub struct snd_ac97_template {
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_ac97)>,
    pub pci: *mut pci_dev,
    pub scaps: c_uint,
}
#[repr(C)]
pub struct ac97_quirk {
    pub subvendor: c_uint,
    pub subdevice: c_uint,
    pub codec_id: c_uint,
    pub name: *const c_char,
    pub type_: c_int,
}
#[repr(C)]
pub struct snd_pcm_chmap {
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
pub struct snd_pci_quirk {
    pub subvendor: c_uint,
    pub subdevice: c_uint,
    pub name: *const c_char,
    pub value: c_int,
}
#[repr(C)]
pub struct pci_driver_driver {
    pub pm: *const c_void,
}
#[repr(C)]
pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    pub driver: pci_driver_driver,
}
#[repr(C)]
pub struct gameport {
    pub io: c_ulong,
}

unsafe extern "C" {
    static mut jiffies: c_ulong;
    static snd_pcm_std_chmaps: *const c_void;
    static snd_pcm_alt_chmaps: *const c_void;

    fn inb(port: c_ulong) -> u8;
    fn inl(port: c_ulong) -> c_uint;
    fn outb(val: u8, port: c_ulong);
    fn outl(val: c_uint, port: c_ulong);
    fn udelay(usecs: c_uint);
    fn msleep(msecs: c_uint);
    fn schedule_timeout_uninterruptible(timeout: c_long);
    fn time_before(a: c_ulong, b: c_ulong) -> c_int;
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn snd_dma_alloc_pages(t: c_int, dev: *mut device, size: c_ulong, dmab: *mut snd_dma_buffer) -> c_int;
    fn snd_dma_free_pages(dmab: *mut snd_dma_buffer);
    fn kmalloc(size: c_ulong, flags: c_uint) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut via82xx;
    fn snd_pcm_sgbuf_get_addr(substream: *mut snd_pcm_substream, ofs: c_uint) -> c_uint;
    fn snd_pcm_sgbuf_get_chunk_size(substream: *mut snd_pcm_substream, ofs: c_uint, size: c_uint) -> c_uint;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: c_uint) -> snd_pcm_uframes_t;
    fn params_periods(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_period_bytes(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_ac97_set_rate(ac97: *mut snd_ac97, reg: c_int, rate: c_uint);
    fn snd_pcm_limit_hw_rates(runtime: *mut snd_pcm_runtime);
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, param: c_int) -> c_int;
    fn snd_pcm_hw_rule_noresample(runtime: *mut snd_pcm_runtime, rate: c_uint) -> c_int;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, param: c_int, list: *const snd_pcm_hw_constraint_list) -> c_int;
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, t: c_int, dev: *mut device, min: c_ulong, max: c_ulong);
    fn snd_pcm_add_chmap_ctls(pcm: *mut snd_pcm, stream: c_int, chmaps: *const c_void, channels: c_int, private_value: c_uint, chmap: *mut *mut snd_pcm_chmap) -> c_int;
    fn snd_ctl_enum_info(uinfo: *mut snd_ctl_elem_info, channels: c_uint, items: c_uint, texts: *const *const c_char) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut via82xx;
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_kcontrol_id);
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(n: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_find_id_mixer(card: *mut snd_card, name: *const c_char) -> *mut snd_kcontrol;
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_ac97_bus(card: *mut snd_card, num: c_int, ops: *const snd_ac97_bus_ops, private_data: *mut c_void, rbus: *mut *mut snd_ac97_bus) -> c_int;
    fn snd_ac97_mixer(bus: *mut snd_ac97_bus, template: *mut snd_ac97_template, rac97: *mut *mut snd_ac97) -> c_int;
    fn snd_ac97_tune_hardware(ac97: *mut snd_ac97, quirks: *const ac97_quirk, override_: *const c_char);
    fn snd_ac97_update_bits(ac97: *mut snd_ac97, reg: c_int, mask: c_uint, value: c_uint);
    fn ac97_can_spdif(ac97: *mut snd_ac97) -> c_int;
    fn snd_ac97_update_power(ac97: *mut snd_ac97, reg: c_int, power: c_int);
    fn snd_ac97_suspend(ac97: *mut snd_ac97);
    fn snd_ac97_resume(ac97: *mut snd_ac97);
    fn snd_ac97_get_short_name(ac97: *mut snd_ac97) -> *const c_char;
    fn pci_read_config_byte(pci: *mut pci_dev, where_: c_int, val: *mut u8) -> c_int;
    fn pci_write_config_byte(pci: *mut pci_dev, where_: c_int, val: u8) -> c_int;
    fn pci_write_config_dword(pci: *mut pci_dev, where_: c_int, val: c_uint) -> c_int;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn pci_get_device(vendor: c_uint, device: c_uint, from: *mut pci_dev) -> *mut pci_dev;
    fn pci_dev_put(pci: *mut pci_dev);
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn pcim_request_all_regions(pci: *mut pci_dev, name: *const c_char) -> c_int;
    fn pci_set_master(pci: *mut pci_dev);
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn devm_request_region(dev: *mut device, start: c_ulong, n: c_ulong, name: *const c_char) -> *mut resource;
    fn snd_mpu401_uart_interrupt(irq: c_int, private_data: *mut c_void) -> irqreturn_t;
    fn snd_mpu401_uart_new(card: *mut snd_card, device: c_int, hardware: c_int, port: c_ulong, info_flags: c_uint, irq: c_int, rrawmidi: *mut *mut snd_rawmidi) -> c_int;
    fn gameport_allocate_port() -> *mut gameport;
    fn gameport_set_name(gp: *mut gameport, name: *const c_char);
    fn gameport_set_phys(gp: *mut gameport, fmt: *const c_char, name: *const c_char);
    fn gameport_set_dev_parent(gp: *mut gameport, dev: *mut device);
    fn gameport_register_port(gp: *mut gameport);
    fn gameport_unregister_port(gp: *mut gameport);
    fn pci_name(pci: *mut pci_dev) -> *const c_char;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn snd_card_ro_proc_new(card: *mut snd_card, name: *const c_char, private_data: *mut c_void, read: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer));
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_devm_card_new(dev: *mut device, idx: c_int, id: *const c_char, module: *mut c_void, extra_size: usize, card: *mut *mut snd_card) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_card_free_on_error(dev: *mut device, err: c_int) -> c_int;
    fn snd_pci_quirk_lookup(pci: *mut pci_dev, list: *const snd_pci_quirk) -> *const snd_pci_quirk;
    fn snd_pci_quirk_name(q: *const snd_pci_quirk) -> *const c_char;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> c_long;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *const c_uint,
    pub mask: c_uint,
}

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENODEV: c_int = 19;
const EBUSY: c_int = 16;
const ENOSYS: c_int = 38;
const GFP_KERNEL: c_uint = 0;
const PAGE_SIZE: c_ulong = 4096;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_SHARED: c_ulong = 0x80;
const KBUILD_MODNAME: *const c_char = b"via82xx\0".as_ptr() as *const c_char;
const THIS_MODULE: *mut c_void = ptr::null_mut();

const SNDRV_DEFAULT_IDX1: c_int = -1;
const SNDRV_DEFAULT_STR1: *mut c_char = ptr::null_mut();
const SNDRV_DMA_TYPE_DEV: c_int = 1;
const SNDRV_DMA_TYPE_DEV_SG: c_int = 2;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 1;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint = 1 << 2;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 3;
const SNDRV_PCM_INFO_PAUSE: c_uint = 1 << 4;
const SNDRV_PCM_FMTBIT_U8: c_ulong = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << 1;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 0;
const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 1 << 30;
const SNDRV_PCM_RATE_8000_48000: c_uint = 0x0000ffff;
const SNDRV_PCM_RATE_KNOT: c_uint = 1 << 31;
const SNDRV_PCM_HW_PARAM_PERIODS: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 1;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_ELEM_IFACE_PCM: c_uint = 3;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 0x3;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 0x10000;
const SNDRV_CTL_ELEM_ACCESS_INACTIVE: c_uint = 0x200;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 2;
const SNDRV_CTL_EVENT_MASK_VALUE: c_uint = 1;
const SNDRV_CTL_EVENT_MASK_INFO: c_uint = 2;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;
const AC97_PCM_FRONT_DAC_RATE: c_int = 0x2c;
const AC97_SPDIF: c_int = 0x3a;
const AC97_PCM_LR_ADC_RATE: c_int = 0x32;
const AC97_PCM_SURR_DAC_RATE: c_int = 0x2e;
const AC97_PCM_LFE_DAC_RATE: c_int = 0x30;
const AC97_EXTENDED_STATUS: c_int = 0x2a;
const AC97_RATES_SPDIF: usize = 0;
const AC97_RATES_ADC: usize = 1;
const AC97_RATES_FRONT_DAC: usize = 2;
const AC97_SCAP_SKIP_MODEM: c_uint = 1 << 0;
const AC97_SCAP_POWER_SAVE: c_uint = 1 << 1;
const AC97_TUNE_NONE: c_int = 0;
const AC97_TUNE_HP_ONLY: c_int = 1;
const AC97_TUNE_ALC_JACK: c_int = 2;
const MPU401_HW_VIA686A: c_int = 1;
const MPU401_INFO_INTEGRATED: c_uint = 1 << 0;
const MPU401_INFO_IRQ_HOOK: c_uint = 1 << 1;
const PCI_DEVICE_ID_VIA_82C686_5: c_uint = 0x3058;
const PCI_DEVICE_ID_VIA_8233_5: c_uint = 0x3059;
const PCI_VENDOR_ID_VIA: c_uint = 0x1106;

static mut index: c_int = SNDRV_DEFAULT_IDX1; /* Index 0-MAX */
static mut id: *mut c_char = SNDRV_DEFAULT_STR1; /* ID for this card */
static mut mpu_port: c_long = 0;
/* SUPPORT_JOYSTICK is derived from IS_REACHABLE(CONFIG_GAMEPORT) in C. */
static mut joystick: bool_ = false;
static mut ac97_clock: c_int = 48000;
static mut ac97_quirk: *mut c_char = ptr::null_mut();
static mut dxs_support: c_int = 0;
static mut dxs_init_volume: c_int = 31;
static mut nodelay: c_int = 0;

/* just for backward compatibility */
static mut enable: bool_ = false;

/* revision numbers for via686 */
const VIA_REV_686_A: c_uint = 0x10;
const VIA_REV_686_B: c_uint = 0x11;
const VIA_REV_686_C: c_uint = 0x12;
const VIA_REV_686_D: c_uint = 0x13;
const VIA_REV_686_E: c_uint = 0x14;
const VIA_REV_686_H: c_uint = 0x20;

/* revision numbers for via8233 */
const VIA_REV_PRE_8233: c_uint = 0x10; /* not in market */
const VIA_REV_8233C: c_uint = 0x20; /* 2 rec, 4 pb, 1 multi-pb */
const VIA_REV_8233: c_uint = 0x30; /* 2 rec, 4 pb, 1 multi-pb, spdif */
const VIA_REV_8233A: c_uint = 0x40; /* 1 rec, 1 multi-pb, spdf */
const VIA_REV_8235: c_uint = 0x50; /* 2 rec, 4 pb, 1 multi-pb, spdif */
const VIA_REV_8237: c_uint = 0x60;
const VIA_REV_8251: c_uint = 0x70;

/* common offsets */
const VIA_REG_OFFSET_STATUS: c_ulong = 0x00; /* byte - channel status */
const VIA_REG_STAT_ACTIVE: c_uint = 0x80; /* RO */
const VIA8233_SHADOW_STAT_ACTIVE: c_uint = 0x08; /* RO */
const VIA_REG_STAT_PAUSED: c_uint = 0x40; /* RO */
const VIA_REG_STAT_TRIGGER_QUEUED: c_uint = 0x08; /* RO */
const VIA_REG_STAT_STOPPED: c_uint = 0x04; /* RWC */
const VIA_REG_STAT_EOL: c_uint = 0x02; /* RWC */
const VIA_REG_STAT_FLAG: c_uint = 0x01; /* RWC */
const VIA_REG_OFFSET_CONTROL: c_ulong = 0x01; /* byte - channel control */
const VIA_REG_CTRL_START: c_uint = 0x80; /* WO */
const VIA_REG_CTRL_TERMINATE: c_uint = 0x40; /* WO */
const VIA_REG_CTRL_AUTOSTART: c_uint = 0x20;
const VIA_REG_CTRL_PAUSE: c_uint = 0x08; /* RW */
const VIA_REG_CTRL_INT_STOP: c_uint = 0x04;
const VIA_REG_CTRL_INT_EOL: c_uint = 0x02;
const VIA_REG_CTRL_INT_FLAG: c_uint = 0x01;
const VIA_REG_CTRL_RESET: c_uint = 0x01; /* RW - probably reset? undocumented */
const VIA_REG_CTRL_INT: c_uint = VIA_REG_CTRL_INT_FLAG | VIA_REG_CTRL_INT_EOL | VIA_REG_CTRL_AUTOSTART;
const VIA_REG_OFFSET_TYPE: c_ulong = 0x02; /* byte - channel type (686 only) */
const VIA_REG_TYPE_AUTOSTART: c_uint = 0x80; /* RW - autostart at EOL */
const VIA_REG_TYPE_16BIT: c_uint = 0x20; /* RW */
const VIA_REG_TYPE_STEREO: c_uint = 0x10; /* RW */
const VIA_REG_TYPE_INT_LLINE: c_uint = 0x00;
const VIA_REG_TYPE_INT_LSAMPLE: c_uint = 0x04;
const VIA_REG_TYPE_INT_LESSONE: c_uint = 0x08;
const VIA_REG_TYPE_INT_MASK: c_uint = 0x0c;
const VIA_REG_TYPE_INT_EOL: c_uint = 0x02;
const VIA_REG_TYPE_INT_FLAG: c_uint = 0x01;
const VIA_REG_OFFSET_TABLE_PTR: c_ulong = 0x04; /* dword - channel table pointer */
const VIA_REG_OFFSET_CURR_PTR: c_ulong = 0x04; /* dword - channel current pointer */
const VIA_REG_OFFSET_STOP_IDX: c_ulong = 0x08; /* dword - stop index, channel type, sample rate */
const VIA8233_REG_TYPE_16BIT: c_uint = 0x00200000; /* RW */
const VIA8233_REG_TYPE_STEREO: c_uint = 0x00100000; /* RW */
const VIA_REG_OFFSET_CURR_COUNT: c_ulong = 0x0c; /* dword - channel current count (24 bit) */
const VIA_REG_OFFSET_CURR_INDEX: c_ulong = 0x0f; /* byte - channel current index (for via8233 only) */

const VIA_REG_PLAYBACK_STATUS: c_ulong = 0x00;
const VIA_REG_PLAYBACK_CONTROL: c_ulong = 0x01;
const VIA_REG_PLAYBACK_TYPE: c_ulong = 0x02;
const VIA_REG_PLAYBACK_TABLE_PTR: c_ulong = 0x04;
const VIA_REG_PLAYBACK_CURR_PTR: c_ulong = 0x04;
const VIA_REG_PLAYBACK_STOP_IDX: c_ulong = 0x08;
const VIA_REG_PLAYBACK_CURR_COUNT: c_ulong = 0x0c;
const VIA_REG_CAPTURE_STATUS: c_ulong = 0x10;
const VIA_REG_CAPTURE_CONTROL: c_ulong = 0x11;
const VIA_REG_CAPTURE_TYPE: c_ulong = 0x12;
const VIA_REG_CAPTURE_TABLE_PTR: c_ulong = 0x14;
const VIA_REG_CAPTURE_CURR_PTR: c_ulong = 0x14;
const VIA_REG_CAPTURE_STOP_IDX: c_ulong = 0x18;
const VIA_REG_CAPTURE_CURR_COUNT: c_ulong = 0x1c;
const VIA_REG_FM_STATUS: c_ulong = 0x20;
const VIA_REG_FM_CONTROL: c_ulong = 0x21;
const VIA_REG_FM_TYPE: c_ulong = 0x22;
const VIA_REG_FM_TABLE_PTR: c_ulong = 0x24;
const VIA_REG_FM_CURR_PTR: c_ulong = 0x24;
const VIA_REG_FM_STOP_IDX: c_ulong = 0x28;
const VIA_REG_FM_CURR_COUNT: c_ulong = 0x2c;

/* AC'97 */
const VIA_REG_AC97: c_ulong = 0x80; /* dword */
const VIA_REG_AC97_CODEC_ID_MASK: c_uint = 3 << 30;
const VIA_REG_AC97_CODEC_ID_SHIFT: c_uint = 30;
const VIA_REG_AC97_CODEC_ID_PRIMARY: c_uint = 0x00;
const VIA_REG_AC97_CODEC_ID_SECONDARY: c_uint = 0x01;
const VIA_REG_AC97_SECONDARY_VALID: c_uint = 1 << 27;
const VIA_REG_AC97_PRIMARY_VALID: c_uint = 1 << 25;
const VIA_REG_AC97_BUSY: c_uint = 1 << 24;
const VIA_REG_AC97_READ: c_uint = 1 << 23;
const VIA_REG_AC97_CMD_SHIFT: c_uint = 16;
const VIA_REG_AC97_CMD_MASK: c_uint = 0x7e;
const VIA_REG_AC97_DATA_SHIFT: c_uint = 0;
const VIA_REG_AC97_DATA_MASK: c_uint = 0xffff;

const VIA_REG_SGD_SHADOW: c_ulong = 0x84; /* dword */
const VIA_REG_SGD_STAT_PB_FLAG: c_uint = 1 << 0;
const VIA_REG_SGD_STAT_CP_FLAG: c_uint = 1 << 1;
const VIA_REG_SGD_STAT_FM_FLAG: c_uint = 1 << 2;
const VIA_REG_SGD_STAT_PB_EOL: c_uint = 1 << 4;
const VIA_REG_SGD_STAT_CP_EOL: c_uint = 1 << 5;
const VIA_REG_SGD_STAT_FM_EOL: c_uint = 1 << 6;
const VIA_REG_SGD_STAT_PB_STOP: c_uint = 1 << 8;
const VIA_REG_SGD_STAT_CP_STOP: c_uint = 1 << 9;
const VIA_REG_SGD_STAT_FM_STOP: c_uint = 1 << 10;
const VIA_REG_SGD_STAT_PB_ACTIVE: c_uint = 1 << 12;
const VIA_REG_SGD_STAT_CP_ACTIVE: c_uint = 1 << 13;
const VIA_REG_SGD_STAT_FM_ACTIVE: c_uint = 1 << 14;
const VIA8233_REG_SGD_STAT_FLAG: c_uint = 1 << 0;
const VIA8233_REG_SGD_STAT_EOL: c_uint = 1 << 1;
const VIA8233_REG_SGD_STAT_STOP: c_uint = 1 << 2;
const VIA8233_REG_SGD_STAT_ACTIVE: c_uint = 1 << 3;
const VIA8233_REG_SGD_CHAN_SDX: c_uint = 0;
const VIA8233_REG_SGD_CHAN_MULTI: c_uint = 4;
const VIA8233_REG_SGD_CHAN_REC: c_uint = 6;
const VIA8233_REG_SGD_CHAN_REC1: c_uint = 7;

const VIA_REG_GPI_STATUS: c_ulong = 0x88;
const VIA_REG_GPI_INTR: c_ulong = 0x8c;

const VIA_REG_MULTPLAY_STATUS: c_ulong = 0x40;
const VIA_REG_MULTPLAY_CONTROL: c_ulong = 0x41;
const VIA_REG_MULTPLAY_TYPE: c_ulong = 0x42;
const VIA_REG_MULTPLAY_TABLE_PTR: c_ulong = 0x44;
const VIA_REG_MULTPLAY_CURR_PTR: c_ulong = 0x44;
const VIA_REG_MULTPLAY_STOP_IDX: c_ulong = 0x48;
const VIA_REG_MULTPLAY_CURR_COUNT: c_ulong = 0x4c;
const VIA_REG_CAPTURE_8233_STATUS: c_ulong = 0x60;
const VIA_REG_CAPTURE_8233_CONTROL: c_ulong = 0x61;
const VIA_REG_CAPTURE_8233_TYPE: c_ulong = 0x62;
const VIA_REG_CAPTURE_8233_TABLE_PTR: c_ulong = 0x64;
const VIA_REG_CAPTURE_8233_CURR_PTR: c_ulong = 0x64;
const VIA_REG_CAPTURE_8233_STOP_IDX: c_ulong = 0x68;
const VIA_REG_CAPTURE_8233_CURR_COUNT: c_ulong = 0x6c;

const VIA_REG_OFS_PLAYBACK_VOLUME_L: c_ulong = 0x02; /* byte */
const VIA_REG_OFS_PLAYBACK_VOLUME_R: c_ulong = 0x03; /* byte */
const VIA_REG_OFS_MULTPLAY_FORMAT: c_ulong = 0x02; /* byte - format and channels */
const VIA_REG_MULTPLAY_FMT_8BIT: c_int = 0x00;
const VIA_REG_MULTPLAY_FMT_16BIT: c_int = 0x80;
const VIA_REG_MULTPLAY_FMT_CH_MASK: c_int = 0x70; /* # channels << 4 (valid = 1,2,4,6) */
const VIA_REG_OFS_CAPTURE_FIFO: c_ulong = 0x02; /* byte - bit 6 = fifo enable */
const VIA_REG_CAPTURE_FIFO_ENABLE: c_uint = 0x40;

const VIA_DXS_MAX_VOLUME: c_uint = 31; /* max. volume (attenuation) of reg 0x32/33 */

const VIA_REG_CAPTURE_CHANNEL: c_ulong = 0x63; /* byte - input select */
const VIA_REG_CAPTURE_CHANNEL_MIC: c_uint = 0x4;
const VIA_REG_CAPTURE_CHANNEL_LINE: c_uint = 0;
const VIA_REG_CAPTURE_SELECT_CODEC: c_uint = 0x03; /* recording source codec (0 = primary) */

const VIA_TBL_BIT_FLAG: c_uint = 0x40000000;
const VIA_TBL_BIT_EOL: c_uint = 0x80000000;

/* pci space */
const VIA_ACLINK_STAT: c_int = 0x40;
const VIA_ACLINK_C11_READY: c_uint = 0x20;
const VIA_ACLINK_C10_READY: c_uint = 0x10;
const VIA_ACLINK_C01_READY: c_uint = 0x04; /* secondary codec ready */
const VIA_ACLINK_LOWPOWER: c_uint = 0x02; /* low-power state */
const VIA_ACLINK_C00_READY: c_uint = 0x01; /* primary codec ready */
const VIA_ACLINK_CTRL: c_int = 0x41;
const VIA_ACLINK_CTRL_ENABLE: c_uint = 0x80; /* 0: disable, 1: enable */
const VIA_ACLINK_CTRL_RESET: c_uint = 0x40; /* 0: assert, 1: de-assert */
const VIA_ACLINK_CTRL_SYNC: c_uint = 0x20; /* 0: release SYNC, 1: force SYNC hi */
const VIA_ACLINK_CTRL_SDO: c_uint = 0x10; /* 0: release SDO, 1: force SDO hi */
const VIA_ACLINK_CTRL_VRA: c_uint = 0x08; /* 0: disable VRA, 1: enable VRA */
const VIA_ACLINK_CTRL_PCM: c_uint = 0x04; /* 0: disable PCM, 1: enable */
const VIA_ACLINK_CTRL_FM: c_uint = 0x02; /* via686 only */
const VIA_ACLINK_CTRL_SB: c_uint = 0x01; /* via686 only */
const VIA_ACLINK_CTRL_INIT: c_uint = VIA_ACLINK_CTRL_ENABLE | VIA_ACLINK_CTRL_RESET | VIA_ACLINK_CTRL_PCM | VIA_ACLINK_CTRL_VRA;
const VIA_FUNC_ENABLE: c_int = 0x42;
const VIA_FUNC_MIDI_PNP: c_uint = 0x80; /* FIXME: it's 0x40 in the datasheet! */
const VIA_FUNC_MIDI_IRQMASK: c_uint = 0x40; /* FIXME: not documented! */
const VIA_FUNC_RX2C_WRITE: c_uint = 0x20;
const VIA_FUNC_SB_FIFO_EMPTY: c_uint = 0x10;
const VIA_FUNC_ENABLE_GAME: c_uint = 0x08;
const VIA_FUNC_ENABLE_FM: c_uint = 0x04;
const VIA_FUNC_ENABLE_MIDI: c_uint = 0x02;
const VIA_FUNC_ENABLE_SB: c_uint = 0x01;
const VIA_PNP_CONTROL: c_int = 0x43;
const VIA_FM_NMI_CTRL: c_int = 0x48;
const VIA8233_VOLCHG_CTRL: c_int = 0x48;
const VIA8233_SPDIF_CTRL: c_int = 0x49;
const VIA8233_SPDIF_DX3: c_uint = 0x08;
const VIA8233_SPDIF_SLOT_MASK: c_uint = 0x03;
const VIA8233_SPDIF_SLOT_1011: c_uint = 0x00;
const VIA8233_SPDIF_SLOT_34: c_uint = 0x01;
const VIA8233_SPDIF_SLOT_78: c_uint = 0x02;
const VIA8233_SPDIF_SLOT_69: c_uint = 0x03;

const VIA_DXS_AUTO: c_int = 0;
const VIA_DXS_ENABLE: c_int = 1;
const VIA_DXS_DISABLE: c_int = 2;
const VIA_DXS_48K: c_int = 3;
const VIA_DXS_NO_VRA: c_int = 4;
const VIA_DXS_SRC: c_int = 5;

#[repr(C)]
pub struct snd_via_sg_table {
    pub offset: c_uint,
    pub size: c_uint,
}

const VIA_TABLE_SIZE: c_uint = 255;
const VIA_MAX_BUFSIZE: c_uint = 1 << 24;
const VIA_MAX_DEVS: usize = 7; /* 4 playback, 1 multi, 2 capture */

#[repr(C)]
pub struct viadev {
    pub reg_offset: c_uint,
    pub port: c_ulong,
    pub direction: c_int, /* playback = 0, capture = 1 */
    pub substream: *mut snd_pcm_substream,
    pub running: c_int,
    pub tbl_entries: c_uint, /* # descriptors */
    pub table: snd_dma_buffer,
    pub idx_table: *mut snd_via_sg_table,
    /* for recovery from the unexpected pointer */
    pub lastpos: c_uint,
    pub fragsize: c_uint,
    pub bufsize: c_uint,
    pub bufsize2: c_uint,
    pub hwptr_done: c_int, /* processed frame position in the buffer */
    pub in_interrupt: c_int,
    pub shadow_shift: c_int,
}

const TYPE_CARD_VIA686: c_int = 1;
const TYPE_CARD_VIA8233: c_int = 2;
const TYPE_VIA686: c_int = 0;
const TYPE_VIA8233: c_int = 1;
const TYPE_VIA8233A: c_int = 2;

#[repr(C)]
pub struct via_rate_lock {
    pub lock: spinlock_t,
    pub rate: c_int,
    pub used: c_int,
}

#[repr(C)]
pub struct via82xx {
    pub irq: c_int,
    pub port: c_ulong,
    pub mpu_res: *mut resource,
    pub chip_type: c_int,
    pub revision: u8,
    pub old_legacy: u8,
    pub old_legacy_cfg: u8,
    pub legacy_saved: u8,
    pub legacy_cfg_saved: u8,
    pub spdif_ctrl_saved: u8,
    pub capture_src_saved: [u8; 2],
    pub mpu_port_saved: c_uint,
    pub playback_volume: [[u8; 2]; 4],
    pub playback_volume_c: [u8; 2],
    pub intr_mask: c_uint,
    pub pci: *mut pci_dev,
    pub card: *mut snd_card,
    pub num_devs: c_uint,
    pub playback_devno: c_uint,
    pub multi_devno: c_uint,
    pub capture_devno: c_uint,
    pub devs: [viadev; VIA_MAX_DEVS],
    pub rates: [via_rate_lock; 2],
    pub dxs_fixed: c_uint,
    pub no_vra: c_uint,
    pub dxs_src: c_uint,
    pub spdif_on: c_uint,
    pub pcms: [*mut snd_pcm; 2],
    pub rmidi: *mut snd_rawmidi,
    pub dxs_controls: [*mut snd_kcontrol; 4],
    pub ac97_bus: *mut snd_ac97_bus,
    pub ac97: *mut snd_ac97,
    pub ac97_clock: c_uint,
    pub ac97_secondary: c_uint,
    pub reg_lock: spinlock_t,
    pub proc_entry: *mut snd_info_entry,
    pub gameport: *mut gameport,
}

const fn pci_vdevice(vendor: c_uint, device: c_uint, driver_data: c_ulong) -> pci_device_id {
    pci_device_id {
        vendor,
        device,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data,
    }
}

static snd_via82xx_ids: [pci_device_id; 3] = [
    /* 0x1106, 0x3058 */
    pci_vdevice(PCI_VENDOR_ID_VIA, PCI_DEVICE_ID_VIA_82C686_5, TYPE_CARD_VIA686 as c_ulong), /* 686A */
    /* 0x1106, 0x3059 */
    pci_vdevice(PCI_VENDOR_ID_VIA, PCI_DEVICE_ID_VIA_8233_5, TYPE_CARD_VIA8233 as c_ulong), /* VT8233 */
    pci_device_id { vendor: 0, device: 0, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
];

#[inline]
fn PAGE_ALIGN(x: c_uint) -> c_ulong {
    ((x as c_ulong + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)) as c_ulong
}

#[inline]
unsafe fn VIAREG(via: *mut via82xx, x: c_ulong) -> c_ulong {
    unsafe { (*via).port + x }
}

#[inline]
unsafe fn VIADEV_REG(viadev: *mut viadev, x: c_ulong) -> c_ulong {
    unsafe { (*viadev).port + x }
}

#[inline]
fn cpu_to_le32(x: c_uint) -> __le32 {
    x.to_le()
}

unsafe fn build_via_table(
    dev: *mut viadev,
    substream: *mut snd_pcm_substream,
    pci: *mut pci_dev,
    periods: c_uint,
    fragsize: c_uint,
) -> c_int {
    let mut i: c_uint;
    let mut idx: c_uint;
    let mut ofs: c_uint;
    let mut rest: c_uint;
    let chip: *mut via82xx = unsafe { snd_pcm_substream_chip(substream) };
    let pgtbl: *mut __le32;

    unsafe {
        if (*dev).table.area.is_null() {
            /* the start of each lists must be aligned to 8 bytes,
             * but the kernel pages are much bigger, so we don't care
             */
            if snd_dma_alloc_pages(
                SNDRV_DMA_TYPE_DEV,
                &mut (*(*chip).pci).dev,
                PAGE_ALIGN(VIA_TABLE_SIZE * 2 * 8),
                &mut (*dev).table,
            ) < 0
            {
                return -ENOMEM;
            }
        }
        if (*dev).idx_table.is_null() {
            (*dev).idx_table = kmalloc(
                mem::size_of::<snd_via_sg_table>() as c_ulong * VIA_TABLE_SIZE as c_ulong,
                GFP_KERNEL,
            ) as *mut snd_via_sg_table;
            if (*dev).idx_table.is_null() {
                return -ENOMEM;
            }
        }

        /* fill the entries */
        idx = 0;
        ofs = 0;
        pgtbl = (*dev).table.area as *mut __le32;
        i = 0;
        while i < periods {
            rest = fragsize;
            /* fill descriptors for a period.
             * a period can be split to several descriptors if it's
             * over page boundary.
             */
            loop {
                let r: c_uint;
                let flag: c_uint;
                let addr: c_uint;

                if idx >= VIA_TABLE_SIZE {
                    dev_err(&mut (*pci).dev, b"too much table size!\n\0".as_ptr() as *const c_char);
                    return -EINVAL;
                }
                addr = snd_pcm_sgbuf_get_addr(substream, ofs);
                *pgtbl.add((idx << 1) as usize) = cpu_to_le32(addr);
                r = snd_pcm_sgbuf_get_chunk_size(substream, ofs, rest);
                rest = rest.wrapping_sub(r);
                if rest == 0 {
                    if i == periods - 1 {
                        flag = VIA_TBL_BIT_EOL; /* buffer boundary */
                    } else {
                        flag = VIA_TBL_BIT_FLAG; /* period boundary */
                    }
                } else {
                    flag = 0; /* period continues to the next */
                }
                /*
                dev_dbg(&pci->dev,
                    "tbl %d: at %d  size %d (rest %d)\n",
                    idx, ofs, r, rest);
                */
                *pgtbl.add(((idx << 1) + 1) as usize) = cpu_to_le32(r | flag);
                (*(*dev).idx_table.add(idx as usize)).offset = ofs;
                (*(*dev).idx_table.add(idx as usize)).size = r;
                ofs = ofs.wrapping_add(r);
                idx = idx.wrapping_add(1);
                if !(rest > 0) {
                    break;
                }
            }
            i = i.wrapping_add(1);
        }
        (*dev).tbl_entries = idx;
        (*dev).bufsize = periods.wrapping_mul(fragsize);
        (*dev).bufsize2 = (*dev).bufsize / 2;
        (*dev).fragsize = fragsize;
    }
    0
}

unsafe fn clean_via_table(dev: *mut viadev, _substream: *mut snd_pcm_substream, _pci: *mut pci_dev) -> c_int {
    unsafe {
        if !(*dev).table.area.is_null() {
            snd_dma_free_pages(&mut (*dev).table);
            (*dev).table.area = ptr::null_mut();
        }
        kfree((*dev).idx_table as *mut c_void);
        (*dev).idx_table = ptr::null_mut();
    }
    0
}

#[inline]
unsafe fn snd_via82xx_codec_xread(chip: *mut via82xx) -> c_uint {
    unsafe { inl(VIAREG(chip, VIA_REG_AC97)) }
}

#[inline]
unsafe fn snd_via82xx_codec_xwrite(chip: *mut via82xx, val: c_uint) {
    unsafe { outl(val, VIAREG(chip, VIA_REG_AC97)) };
}

unsafe fn snd_via82xx_codec_ready(chip: *mut via82xx, secondary: c_int) -> c_int {
    let mut timeout: c_uint = 1000; /* 1ms */
    let mut val: c_uint;

    unsafe {
        while timeout > 0 {
            timeout -= 1;
            udelay(1);
            val = snd_via82xx_codec_xread(chip);
            if (val & VIA_REG_AC97_BUSY) == 0 {
                return (val & 0xffff) as c_int;
            }
        }
        dev_err(
            (*(*chip).card).dev,
            b"codec_ready: codec %i is not ready [0x%x]\n\0".as_ptr() as *const c_char,
            secondary,
            snd_via82xx_codec_xread(chip),
        );
    }
    -EIO
}

unsafe fn snd_via82xx_codec_valid(chip: *mut via82xx, secondary: c_int) -> c_int {
    let mut timeout: c_uint = 1000; /* 1ms */
    let mut val: c_uint;
    let val1: c_uint;
    let stat: c_uint = if secondary == 0 { VIA_REG_AC97_PRIMARY_VALID } else { VIA_REG_AC97_SECONDARY_VALID };

    unsafe {
        while timeout > 0 {
            timeout -= 1;
            val = snd_via82xx_codec_xread(chip);
            let val1 = val & (VIA_REG_AC97_BUSY | stat);
            if val1 == stat {
                return (val & 0xffff) as c_int;
            }
            udelay(1);
        }
    }
    -EIO
}

unsafe extern "C" fn snd_via82xx_codec_wait(ac97: *mut snd_ac97) {
    unsafe {
        let chip: *mut via82xx = (*ac97).private_data as *mut via82xx;
        let _err: c_int;
        _err = snd_via82xx_codec_ready(chip, (*ac97).num);
        /* here we need to wait fairly for long time.. */
        if nodelay == 0 {
            msleep(500);
        }
    }
}

unsafe extern "C" fn snd_via82xx_codec_write(ac97: *mut snd_ac97, reg: u16, val: u16) {
    unsafe {
        let chip: *mut via82xx = (*ac97).private_data as *mut via82xx;
        let mut xval: c_uint;

        xval = if (*ac97).num == 0 { VIA_REG_AC97_CODEC_ID_PRIMARY } else { VIA_REG_AC97_CODEC_ID_SECONDARY };
        xval <<= VIA_REG_AC97_CODEC_ID_SHIFT;
        xval |= (reg as c_uint) << VIA_REG_AC97_CMD_SHIFT;
        xval |= (val as c_uint) << VIA_REG_AC97_DATA_SHIFT;
        snd_via82xx_codec_xwrite(chip, xval);
        snd_via82xx_codec_ready(chip, (*ac97).num);
    }
}

unsafe extern "C" fn snd_via82xx_codec_read(ac97: *mut snd_ac97, reg: u16) -> u16 {
    unsafe {
        let chip: *mut via82xx = (*ac97).private_data as *mut via82xx;
        let mut xval: c_uint;
        let mut val: c_uint = 0xffff;
        let mut again: c_int = 0;

        xval = ((*ac97).num as c_uint) << VIA_REG_AC97_CODEC_ID_SHIFT;
        xval |= if (*ac97).num != 0 { VIA_REG_AC97_SECONDARY_VALID } else { VIA_REG_AC97_PRIMARY_VALID };
        xval |= VIA_REG_AC97_READ;
        xval |= ((reg as c_uint) & 0x7f) << VIA_REG_AC97_CMD_SHIFT;
        loop {
            if again > 3 {
                dev_err(
                    (*(*chip).card).dev,
                    b"codec_read: codec %i is not valid [0x%x]\n\0".as_ptr() as *const c_char,
                    (*ac97).num,
                    snd_via82xx_codec_xread(chip),
                );
                return 0xffff;
            }
            again += 1;
            snd_via82xx_codec_xwrite(chip, xval);
            udelay(20);
            if snd_via82xx_codec_valid(chip, (*ac97).num) >= 0 {
                udelay(25);
                val = snd_via82xx_codec_xread(chip);
                break;
            }
        }
        (val & 0xffff) as u16
    }
}

unsafe fn snd_via82xx_channel_reset(chip: *mut via82xx, viadev: *mut viadev) {
    unsafe {
        outb((VIA_REG_CTRL_PAUSE | VIA_REG_CTRL_TERMINATE | VIA_REG_CTRL_RESET) as u8, VIADEV_REG(viadev, VIA_REG_OFFSET_CONTROL));
        inb(VIADEV_REG(viadev, VIA_REG_OFFSET_CONTROL));
        udelay(50);
        /* disable interrupts */
        outb(0x00, VIADEV_REG(viadev, VIA_REG_OFFSET_CONTROL));
        /* clear interrupts */
        outb(0x03, VIADEV_REG(viadev, VIA_REG_OFFSET_STATUS));
        outb(0x00, VIADEV_REG(viadev, VIA_REG_OFFSET_TYPE)); /* for via686 */
        // outl(0, VIADEV_REG(viadev, OFFSET_CURR_PTR));
        (*viadev).lastpos = 0;
        (*viadev).hwptr_done = 0;
    }
}

unsafe extern "C" fn snd_via686_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    unsafe {
        let chip: *mut via82xx = dev_id as *mut via82xx;
        let status: c_uint;
        let mut i: c_uint;

        status = inl(VIAREG(chip, VIA_REG_SGD_SHADOW));
        if (status & (*chip).intr_mask) == 0 {
            if !(*chip).rmidi.is_null() {
                /* check mpu401 interrupt */
                return snd_mpu401_uart_interrupt(irq, (*(*chip).rmidi).private_data);
            }
            return IRQ_NONE;
        }

        /* check status for each stream */
        spin_lock(&mut (*chip).reg_lock);
        i = 0;
        while i < (*chip).num_devs {
            let viadev = &mut (*chip).devs[i as usize] as *mut viadev;
            let c_status: u8 = inb(VIADEV_REG(viadev, VIA_REG_OFFSET_STATUS));
            if (c_status as c_uint & (VIA_REG_STAT_EOL | VIA_REG_STAT_FLAG | VIA_REG_STAT_STOPPED)) == 0 {
                i += 1;
                continue;
            }
            if !(*viadev).substream.is_null() && (*viadev).running != 0 {
                /*
                 * Update hwptr_done based on 'period elapsed'
                 * interrupts. We'll use it, when the chip returns 0
                 * for OFFSET_CURR_COUNT.
                 */
                if (c_status as c_uint & VIA_REG_STAT_EOL) != 0 {
                    (*viadev).hwptr_done = 0;
                } else {
                    (*viadev).hwptr_done = (*viadev).hwptr_done.wrapping_add((*viadev).fragsize as c_int);
                }
                (*viadev).in_interrupt = c_status as c_int;
                spin_unlock(&mut (*chip).reg_lock);
                snd_pcm_period_elapsed((*viadev).substream);
                spin_lock(&mut (*chip).reg_lock);
                (*viadev).in_interrupt = 0;
            }
            outb(c_status, VIADEV_REG(viadev, VIA_REG_OFFSET_STATUS)); /* ack */
            i += 1;
        }
        spin_unlock(&mut (*chip).reg_lock);
        IRQ_HANDLED
    }
}

unsafe extern "C" fn snd_via8233_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    unsafe {
        let chip: *mut via82xx = dev_id as *mut via82xx;
        let status: c_uint;
        let mut i: c_uint;
        let mut irqreturn: c_int = 0;

        /* check status for each stream */
        spin_lock(&mut (*chip).reg_lock);
        status = inl(VIAREG(chip, VIA_REG_SGD_SHADOW));

        i = 0;
        while i < (*chip).num_devs {
            let viadev = &mut (*chip).devs[i as usize] as *mut viadev;
            let substream: *mut snd_pcm_substream;
            let c_status: u8;
            let shadow_status: u8;

            shadow_status = ((status >> (*viadev).shadow_shift) & (VIA8233_SHADOW_STAT_ACTIVE | VIA_REG_STAT_EOL | VIA_REG_STAT_FLAG)) as u8;
            c_status = (shadow_status as c_uint & (VIA_REG_STAT_EOL | VIA_REG_STAT_FLAG)) as u8;
            if c_status == 0 {
                i += 1;
                continue;
            }

            substream = (*viadev).substream;
            if !substream.is_null() && (*viadev).running != 0 {
                /*
                 * Update hwptr_done based on 'period elapsed'
                 * interrupts. We'll use it, when the chip returns 0
                 * for OFFSET_CURR_COUNT.
                 */
                if (c_status as c_uint & VIA_REG_STAT_EOL) != 0 {
                    (*viadev).hwptr_done = 0;
                } else {
                    (*viadev).hwptr_done = (*viadev).hwptr_done.wrapping_add((*viadev).fragsize as c_int);
                }
                (*viadev).in_interrupt = c_status as c_int;
                if (shadow_status as c_uint & VIA8233_SHADOW_STAT_ACTIVE) != 0 {
                    (*viadev).in_interrupt |= VIA_REG_STAT_ACTIVE as c_int;
                }
                spin_unlock(&mut (*chip).reg_lock);

                snd_pcm_period_elapsed(substream);

                spin_lock(&mut (*chip).reg_lock);
                (*viadev).in_interrupt = 0;
            }
            outb(c_status, VIADEV_REG(viadev, VIA_REG_OFFSET_STATUS)); /* ack */
            irqreturn = 1;
            i += 1;
        }
        spin_unlock(&mut (*chip).reg_lock);
        if irqreturn != 0 { IRQ_HANDLED } else { IRQ_NONE }
    }
}

unsafe extern "C" fn snd_via82xx_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    unsafe {
        let chip: *mut via82xx = snd_pcm_substream_chip(substream);
        let viadev: *mut viadev = (*(*substream).runtime).private_data as *mut viadev;
        let mut val: u8;

        if (*chip).chip_type != TYPE_VIA686 {
            val = VIA_REG_CTRL_INT as u8;
        } else {
            val = 0;
        }
        match cmd {
            SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
                val |= VIA_REG_CTRL_START as u8;
                (*viadev).running = 1;
            }
            SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
                val = VIA_REG_CTRL_TERMINATE as u8;
                (*viadev).running = 0;
            }
            SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
                val |= VIA_REG_CTRL_PAUSE as u8;
                (*viadev).running = 0;
            }
            SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
                (*viadev).running = 1;
            }
            _ => return -EINVAL,
        }
        outb(val, VIADEV_REG(viadev, VIA_REG_OFFSET_CONTROL));
        if cmd == SNDRV_PCM_TRIGGER_STOP {
            snd_via82xx_channel_reset(chip, viadev);
        }
    }
    0
}

#[inline]
unsafe fn check_invalid_pos(viadev: *mut viadev, pos: c_uint) -> bool {
    unsafe { pos < (*viadev).lastpos && (pos >= (*viadev).bufsize2 || (*viadev).lastpos < (*viadev).bufsize2) }
}

unsafe fn calc_linear_pos(chip: *mut via82xx, viadev: *mut viadev, idx: c_uint, count: c_uint) -> c_uint {
    unsafe {
        let size: c_uint;
        let base: c_uint;
        let mut res: c_uint;

        size = (*(*viadev).idx_table.add(idx as usize)).size;
        base = (*(*viadev).idx_table.add(idx as usize)).offset;
        res = base.wrapping_add(size).wrapping_sub(count);
        if res >= (*viadev).bufsize {
            res = res.wrapping_sub((*viadev).bufsize);
        }

        /* check the validity of the calculated position */
        if size < count {
            dev_dbg(
                (*(*chip).card).dev,
                b"invalid via82xx_cur_ptr (size = %d, count = %d)\n\0".as_ptr() as *const c_char,
                size as c_int,
                count as c_int,
            );
            res = (*viadev).lastpos;
        } else {
            if count == 0 {
                /* Some mobos report count = 0 on the DMA boundary,
                 * i.e. count = size indeed.
                 * Let's check whether this step is above the expected size.
                 */
                let mut delta: c_int = res as c_int - (*viadev).lastpos as c_int;
                if delta < 0 {
                    delta += (*viadev).bufsize as c_int;
                }
                if delta as c_uint > (*viadev).fragsize {
                    res = base;
                }
            }
            if check_invalid_pos(viadev, res) {
                /* POINTER_DEBUG logging was conditionally compiled in C. */
                /* count register returns full size when end of buffer is reached */
                res = base.wrapping_add(size);
                if check_invalid_pos(viadev, res) {
                    dev_dbg((*(*chip).card).dev, b"invalid via82xx_cur_ptr (2), using last valid pointer\n\0".as_ptr() as *const c_char);
                    res = (*viadev).lastpos;
                }
            }
        }
        res
    }
}

unsafe extern "C" fn snd_via686_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    unsafe {
        let chip: *mut via82xx = snd_pcm_substream_chip(substream);
        let viadev: *mut viadev = (*(*substream).runtime).private_data as *mut viadev;
        let idx: c_uint;
        let ptr_: c_uint;
        let count: c_uint;
        let res: c_uint;

        if (*viadev).tbl_entries == 0 {
            return 0;
        }
        if (inb(VIADEV_REG(viadev, VIA_REG_OFFSET_STATUS)) as c_uint & VIA_REG_STAT_ACTIVE) == 0 {
            return 0;
        }

        spin_lock(&mut (*chip).reg_lock);
        count = inl(VIADEV_REG(viadev, VIA_REG_OFFSET_CURR_COUNT)) & 0xffffff;
        /* The via686a does not have the current index register,
         * so we need to calculate the index from CURR_PTR.
         */
        ptr_ = inl(VIADEV_REG(viadev, VIA_REG_OFFSET_CURR_PTR));
        if ptr_ <= (*viadev).table.addr as c_uint {
            idx = 0;
        } else {
            /* CURR_PTR holds the address + 8 */
            idx = ((ptr_.wrapping_sub((*viadev).table.addr as c_uint)) / 8 - 1) % (*viadev).tbl_entries;
        }
        res = calc_linear_pos(chip, viadev, idx, count);
        (*viadev).lastpos = res; /* remember the last position */
        spin_unlock(&mut (*chip).reg_lock);

        bytes_to_frames((*substream).runtime, res)
    }
}

unsafe extern "C" fn snd_via8233_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    unsafe {
        let chip: *mut via82xx = snd_pcm_substream_chip(substream);
        let viadev: *mut viadev = (*(*substream).runtime).private_data as *mut viadev;
        let idx: c_uint;
        let mut count: c_uint;
        let mut res: c_uint;
        let mut status: c_int;

        if (*viadev).tbl_entries == 0 {
            return 0;
        }

        spin_lock(&mut (*chip).reg_lock);
        count = inl(VIADEV_REG(viadev, VIA_REG_OFFSET_CURR_COUNT));
        status = (*viadev).in_interrupt;
        if status == 0 {
            status = inb(VIADEV_REG(viadev, VIA_REG_OFFSET_STATUS)) as c_int;
        }

        /* An apparent bug in the 8251 is worked around by sending a
         * REG_CTRL_START. */
        if (*chip).revision as c_uint == VIA_REV_8251 && (status as c_uint & VIA_REG_STAT_EOL) != 0 {
            snd_via82xx_pcm_trigger(substream, SNDRV_PCM_TRIGGER_START);
        }

        if (status as c_uint & VIA_REG_STAT_ACTIVE) == 0 {
            res = 0;
        } else if (count & 0xffffff) != 0 {
            idx = count >> 24;
            if idx >= (*viadev).tbl_entries {
                /* POINTER_DEBUG logging was conditionally compiled in C. */
                res = (*viadev).lastpos;
            } else {
                count &= 0xffffff;
                res = calc_linear_pos(chip, viadev, idx, count);
            }
        } else {
            res = (*viadev).hwptr_done as c_uint;
            if (*viadev).in_interrupt == 0 {
                if (status as c_uint & VIA_REG_STAT_EOL) != 0 {
                    res = 0;
                } else if (status as c_uint & VIA_REG_STAT_FLAG) != 0 {
                    res = res.wrapping_add((*viadev).fragsize);
                }
            }
        }
        (*viadev).lastpos = res;
        spin_unlock(&mut (*chip).reg_lock);

        bytes_to_frames((*substream).runtime, res)
    }
}

unsafe extern "C" fn snd_via82xx_hw_params(substream: *mut snd_pcm_substream, hw_params: *mut snd_pcm_hw_params) -> c_int {
    unsafe {
        let chip: *mut via82xx = snd_pcm_substream_chip(substream);
        let viadev: *mut viadev = (*(*substream).runtime).private_data as *mut viadev;

        build_via_table(
            viadev,
            substream,
            (*chip).pci,
            params_periods(hw_params),
            params_period_bytes(hw_params),
        )
    }
}

unsafe extern "C" fn snd_via82xx_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let chip: *mut via82xx = snd_pcm_substream_chip(substream);
        let viadev: *mut viadev = (*(*substream).runtime).private_data as *mut viadev;

        clean_via_table(viadev, substream, (*chip).pci);
    }
    0
}

unsafe fn snd_via82xx_set_table_ptr(chip: *mut via82xx, viadev: *mut viadev) {
    unsafe {
        snd_via82xx_codec_ready(chip, 0);
        outl((*viadev).table.addr as u32, VIADEV_REG(viadev, VIA_REG_OFFSET_TABLE_PTR));
        udelay(20);
        snd_via82xx_codec_ready(chip, 0);
    }
}

unsafe fn via686_setup_format(chip: *mut via82xx, viadev: *mut viadev, runtime: *mut snd_pcm_runtime) {
    unsafe {
        snd_via82xx_channel_reset(chip, viadev);
        /* this must be set after channel_reset */
        snd_via82xx_set_table_ptr(chip, viadev);
        outb(
            (VIA_REG_TYPE_AUTOSTART
                | (if (*runtime).format == SNDRV_PCM_FORMAT_S16_LE { VIA_REG_TYPE_16BIT } else { 0 })
                | (if (*runtime).channels > 1 { VIA_REG_TYPE_STEREO } else { 0 })
                | (if ((*viadev).reg_offset & 0x10) == 0 { VIA_REG_TYPE_INT_LSAMPLE } else { 0 })
                | VIA_REG_TYPE_INT_EOL
                | VIA_REG_TYPE_INT_FLAG) as u8,
            VIADEV_REG(viadev, VIA_REG_OFFSET_TYPE),
        );
    }
}

unsafe extern "C" fn snd_via686_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let chip: *mut via82xx = snd_pcm_substream_chip(substream);
        let viadev: *mut viadev = (*(*substream).runtime).private_data as *mut viadev;
        let runtime: *mut snd_pcm_runtime = (*substream).runtime;

        snd_ac97_set_rate((*chip).ac97, AC97_PCM_FRONT_DAC_RATE, (*runtime).rate);
        snd_ac97_set_rate((*chip).ac97, AC97_SPDIF, (*runtime).rate);
        via686_setup_format(chip, viadev, runtime);
    }
    0
}

unsafe extern "C" fn snd_via686_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let chip: *mut via82xx = snd_pcm_substream_chip(substream);
        let viadev: *mut viadev = (*(*substream).runtime).private_data as *mut viadev;
        let runtime: *mut snd_pcm_runtime = (*substream).runtime;

        snd_ac97_set_rate((*chip).ac97, AC97_PCM_LR_ADC_RATE, (*runtime).rate);
        via686_setup_format(chip, viadev, runtime);
    }
    0
}

unsafe fn via_lock_rate(rec: *mut via_rate_lock, rate: c_int) -> c_int {
    unsafe {
        let mut changed: c_int = 0;

        spin_lock(&mut (*rec).lock);
        if (*rec).rate != rate {
            if (*rec).rate != 0 && (*rec).used > 1 {
                changed = -EINVAL;
            } else {
                (*rec).rate = rate;
                changed = 1;
            }
        }
        spin_unlock(&mut (*rec).lock);
        changed
    }
}

unsafe extern "C" fn snd_via8233_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let chip: *mut via82xx = snd_pcm_substream_chip(substream);
        let viadev: *mut viadev = (*(*substream).runtime).private_data as *mut viadev;
        let runtime: *mut snd_pcm_runtime = (*substream).runtime;
        let ac97_rate: c_int = if (*chip).dxs_src != 0 { 48000 } else { (*runtime).rate as c_int };
        let rate_changed: c_int;
        let rbits: u32;

        rate_changed = via_lock_rate(&mut (*chip).rates[0], ac97_rate);
        if rate_changed < 0 {
            return rate_changed;
        }
        if rate_changed != 0 {
            snd_ac97_set_rate((*chip).ac97, AC97_PCM_FRONT_DAC_RATE, if (*chip).no_vra != 0 { 48000 } else { (*runtime).rate });
        }
        if (*chip).spdif_on != 0 && (*viadev).reg_offset == 0x30 {
            snd_ac97_set_rate((*chip).ac97, AC97_SPDIF, (*runtime).rate);
        }

        if (*runtime).rate == 48000 {
            rbits = 0xfffff;
        } else {
            rbits = (0x100000 / 48000) * (*runtime).rate + ((0x100000 % 48000) * (*runtime).rate) / 48000;
        }
        snd_via82xx_channel_reset(chip, viadev);
        snd_via82xx_set_table_ptr(chip, viadev);
        outb((*chip).playback_volume[((*viadev).reg_offset / 0x10) as usize][0], VIADEV_REG(viadev, VIA_REG_OFS_PLAYBACK_VOLUME_L));
        outb((*chip).playback_volume[((*viadev).reg_offset / 0x10) as usize][1], VIADEV_REG(viadev, VIA_REG_OFS_PLAYBACK_VOLUME_R));
        outl(
            (if (*runtime).format == SNDRV_PCM_FORMAT_S16_LE { VIA8233_REG_TYPE_16BIT } else { 0 })
                | (if (*runtime).channels > 1 { VIA8233_REG_TYPE_STEREO } else { 0 })
                | rbits
                | 0xff000000,
            VIADEV_REG(viadev, VIA_REG_OFFSET_STOP_IDX),
        );
        udelay(20);
        snd_via82xx_codec_ready(chip, 0);
    }
    0
}

unsafe extern "C" fn snd_via8233_multi_prepare(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let chip: *mut via82xx = snd_pcm_substream_chip(substream);
        let viadev: *mut viadev = (*(*substream).runtime).private_data as *mut viadev;
        let runtime: *mut snd_pcm_runtime = (*substream).runtime;
        let slots: c_uint;
        let mut fmt: c_int;

        if via_lock_rate(&mut (*chip).rates[0], (*runtime).rate as c_int) < 0 {
            return -EINVAL;
        }
        snd_ac97_set_rate((*chip).ac97, AC97_PCM_FRONT_DAC_RATE, (*runtime).rate);
        snd_ac97_set_rate((*chip).ac97, AC97_PCM_SURR_DAC_RATE, (*runtime).rate);
        snd_ac97_set_rate((*chip).ac97, AC97_PCM_LFE_DAC_RATE, (*runtime).rate);
        snd_ac97_set_rate((*chip).ac97, AC97_SPDIF, (*runtime).rate);
        snd_via82xx_channel_reset(chip, viadev);
        snd_via82xx_set_table_ptr(chip, viadev);

        fmt = if (*runtime).format == SNDRV_PCM_FORMAT_S16_LE { VIA_REG_MULTPLAY_FMT_16BIT } else { VIA_REG_MULTPLAY_FMT_8BIT };
        fmt |= ((*runtime).channels << 4) as c_int;
        outb(fmt as u8, VIADEV_REG(viadev, VIA_REG_OFS_MULTPLAY_FORMAT));
        /* C had a disabled VIA_REV_8233A branch here. */
        /* set sample number to slot 3, 4, 7, 8, 6, 9 (for VIA8233/C,8235) */
        /* corresponding to FL, FR, RL, RR, C, LFE ?? */
        slots = match (*runtime).channels {
            1 => (1 << 0) | (1 << 4),
            2 => (1 << 0) | (2 << 4),
            3 => (1 << 0) | (2 << 4) | (5 << 8),
            4 => (1 << 0) | (2 << 4) | (3 << 8) | (4 << 12),
            5 => (1 << 0) | (2 << 4) | (3 << 8) | (4 << 12) | (5 << 16),
            6 => (1 << 0) | (2 << 4) | (3 << 8) | (4 << 12) | (5 << 16) | (6 << 20),
            _ => 0,
        };
        /* STOP index is never reached */
        outl(0xff000000 | slots, VIADEV_REG(viadev, VIA_REG_OFFSET_STOP_IDX));
        udelay(20);
        snd_via82xx_codec_ready(chip, 0);
    }
    0
}

unsafe extern "C" fn snd_via8233_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let chip: *mut via82xx = snd_pcm_substream_chip(substream);
        let viadev: *mut viadev = (*(*substream).runtime).private_data as *mut viadev;
        let runtime: *mut snd_pcm_runtime = (*substream).runtime;

        if via_lock_rate(&mut (*chip).rates[1], (*runtime).rate as c_int) < 0 {
            return -EINVAL;
        }
        snd_ac97_set_rate((*chip).ac97, AC97_PCM_LR_ADC_RATE, (*runtime).rate);
        snd_via82xx_channel_reset(chip, viadev);
        snd_via82xx_set_table_ptr(chip, viadev);
        outb(VIA_REG_CAPTURE_FIFO_ENABLE as u8, VIADEV_REG(viadev, VIA_REG_OFS_CAPTURE_FIFO));
        outl(
            (if (*runtime).format == SNDRV_PCM_FORMAT_S16_LE { VIA8233_REG_TYPE_16BIT } else { 0 })
                | (if (*runtime).channels > 1 { VIA8233_REG_TYPE_STEREO } else { 0 })
                | 0xff000000,
            VIADEV_REG(viadev, VIA_REG_OFFSET_STOP_IDX),
        );
        udelay(20);
        snd_via82xx_codec_ready(chip, 0);
    }
    0
}

static snd_via82xx_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_PAUSE,
    formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_48000,
    rate_min: 48000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: VIA_MAX_BUFSIZE as c_ulong,
    period_bytes_min: 32,
    period_bytes_max: (VIA_MAX_BUFSIZE / 2) as c_ulong,
    periods_min: 2,
    periods_max: VIA_TABLE_SIZE / 2,
    fifo_size: 0,
};

unsafe fn snd_via82xx_pcm_open(chip: *mut via82xx, viadev: *mut viadev, substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let runtime: *mut snd_pcm_runtime = (*substream).runtime;
        let mut err: c_int;
        let ratep: *mut via_rate_lock;
        let mut use_src: bool = false;

        (*runtime).hw = snd_via82xx_hw;

        /* set the hw rate condition */
        ratep = &mut (*chip).rates[(*viadev).direction as usize];
        spin_lock(&mut (*ratep).lock);
        (*ratep).used += 1;
        if (*chip).spdif_on != 0 && (*viadev).reg_offset == 0x30 {
            /* DXS#3 and spdif is on */
            (*runtime).hw.rates = (*(*chip).ac97).rates[AC97_RATES_SPDIF];
            snd_pcm_limit_hw_rates(runtime);
        } else if (*chip).dxs_fixed != 0 && (*viadev).reg_offset < 0x40 {
            /* fixed DXS playback rate */
            (*runtime).hw.rates = SNDRV_PCM_RATE_48000;
            (*runtime).hw.rate_min = 48000;
            (*runtime).hw.rate_max = 48000;
        } else if (*chip).dxs_src != 0 && (*viadev).reg_offset < 0x40 {
            /* use full SRC capabilities of DXS */
            (*runtime).hw.rates = SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000;
            (*runtime).hw.rate_min = 8000;
            (*runtime).hw.rate_max = 48000;
            use_src = true;
        } else if (*ratep).rate == 0 {
            let idx = if (*viadev).direction != 0 { AC97_RATES_ADC } else { AC97_RATES_FRONT_DAC };
            (*runtime).hw.rates = (*(*chip).ac97).rates[idx];
            snd_pcm_limit_hw_rates(runtime);
        } else {
            /* a fixed rate */
            (*runtime).hw.rates = SNDRV_PCM_RATE_KNOT;
            (*runtime).hw.rate_max = (*ratep).rate as c_uint;
            (*runtime).hw.rate_min = (*ratep).rate as c_uint;
        }
        spin_unlock(&mut (*ratep).lock);

        /* we may remove following constaint when we modify table entries
           in interrupt */
        err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
        if err < 0 {
            return err;
        }

        if use_src {
            err = snd_pcm_hw_rule_noresample(runtime, 48000);
            if err < 0 {
                return err;
            }
        }

        (*runtime).private_data = viadev as *mut c_void;
        (*viadev).substream = substream;
    }
    0
}

unsafe extern "C" fn snd_via686_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let chip: *mut via82xx = snd_pcm_substream_chip(substream);
        let viadev: *mut viadev = &mut (*chip).devs[((*chip).playback_devno + (*substream).number) as usize];
        let err: c_int;

        err = snd_via82xx_pcm_open(chip, viadev, substream);
        if err < 0 {
            return err;
        }
    }
    0
}

unsafe extern "C" fn snd_via8233_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let chip: *mut via82xx = snd_pcm_substream_chip(substream);
        let viadev: *mut viadev;
        let stream: c_uint;
        let err: c_int;

        viadev = &mut (*chip).devs[((*chip).playback_devno + (*substream).number) as usize];
        err = snd_via82xx_pcm_open(chip, viadev, substream);
        if err < 0 {
            return err;
        }
        stream = (*viadev).reg_offset / 0x10;
        if !(*chip).dxs_controls[stream as usize].is_null() {
            (*chip).playback_volume[stream as usize][0] = (VIA_DXS_MAX_VOLUME - (dxs_init_volume as c_uint & 31)) as u8;
            (*chip).playback_volume[stream as usize][1] = (VIA_DXS_MAX_VOLUME - (dxs_init_volume as c_uint & 31)) as u8;
            (*(*chip).dxs_controls[stream as usize]).vd[0].access &= !SNDRV_CTL_ELEM_ACCESS_INACTIVE;
            snd_ctl_notify(
                (*chip).card,
                SNDRV_CTL_EVENT_MASK_VALUE | SNDRV_CTL_EVENT_MASK_INFO,
                &mut (*(*chip).dxs_controls[stream as usize]).id,
            );
        }
    }
    0
}

unsafe extern "C" fn snd_via8233_multi_open(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let chip: *mut via82xx = snd_pcm_substream_chip(substream);
        let viadev: *mut viadev = &mut (*chip).devs[(*chip).multi_devno as usize];
        let err: c_int;
        /* channels constraint for VIA8233A
         * 3 and 5 channels are not supported
         */
        static channels: [c_uint; 4] = [1, 2, 4, 6];
        static hw_constraints_channels: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
            count: 4,
            list: channels.as_ptr(),
            mask: 0,
        };

        err = snd_via82xx_pcm_open(chip, viadev, substream);
        if err < 0 {
            return err;
        }
        (*(*substream).runtime).hw.channels_max = 6;
        if (*chip).revision as c_uint == VIA_REV_8233A {
            snd_pcm_hw_constraint_list((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, &hw_constraints_channels);
        }
    }
    0
}

unsafe extern "C" fn snd_via82xx_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let chip: *mut via82xx = snd_pcm_substream_chip(substream);
        let viadev: *mut viadev = &mut (*chip).devs[((*chip).capture_devno + (*(*substream).pcm).device as c_uint) as usize];

        snd_via82xx_pcm_open(chip, viadev, substream)
    }
}

unsafe extern "C" fn snd_via82xx_pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let chip: *mut via82xx = snd_pcm_substream_chip(substream);
        let viadev: *mut viadev = (*(*substream).runtime).private_data as *mut viadev;
        let ratep: *mut via_rate_lock;

        /* release the rate lock */
        ratep = &mut (*chip).rates[(*viadev).direction as usize];
        spin_lock(&mut (*ratep).lock);
        (*ratep).used -= 1;
        if (*ratep).used == 0 {
            (*ratep).rate = 0;
        }
        spin_unlock(&mut (*ratep).lock);
        if (*ratep).rate == 0 {
            if (*viadev).direction == 0 {
                snd_ac97_update_power((*chip).ac97, AC97_PCM_FRONT_DAC_RATE, 0);
                snd_ac97_update_power((*chip).ac97, AC97_PCM_SURR_DAC_RATE, 0);
                snd_ac97_update_power((*chip).ac97, AC97_PCM_LFE_DAC_RATE, 0);
            } else {
                snd_ac97_update_power((*chip).ac97, AC97_PCM_LR_ADC_RATE, 0);
            }
        }
        (*viadev).substream = ptr::null_mut();
    }
    0
}

unsafe extern "C" fn snd_via8233_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let chip: *mut via82xx = snd_pcm_substream_chip(substream);
        let viadev: *mut viadev = (*(*substream).runtime).private_data as *mut viadev;
        let stream: c_uint;

        stream = (*viadev).reg_offset / 0x10;
        if !(*chip).dxs_controls[stream as usize].is_null() {
            (*(*chip).dxs_controls[stream as usize]).vd[0].access |= SNDRV_CTL_ELEM_ACCESS_INACTIVE;
            snd_ctl_notify(
                (*chip).card,
                SNDRV_CTL_EVENT_MASK_INFO,
                &mut (*(*chip).dxs_controls[stream as usize]).id,
            );
        }
        snd_via82xx_pcm_close(substream)
    }
}

static snd_via686_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_via686_playback_open),
    close: Some(snd_via82xx_pcm_close),
    ioctl: ptr::null(),
    hw_params: Some(snd_via82xx_hw_params),
    hw_free: Some(snd_via82xx_hw_free),
    prepare: Some(snd_via686_playback_prepare),
    trigger: Some(snd_via82xx_pcm_trigger),
    pointer: Some(snd_via686_pcm_pointer),
};

static snd_via686_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_via82xx_capture_open),
    close: Some(snd_via82xx_pcm_close),
    ioctl: ptr::null(),
    hw_params: Some(snd_via82xx_hw_params),
    hw_free: Some(snd_via82xx_hw_free),
    prepare: Some(snd_via686_capture_prepare),
    trigger: Some(snd_via82xx_pcm_trigger),
    pointer: Some(snd_via686_pcm_pointer),
};

static snd_via8233_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_via8233_playback_open),
    close: Some(snd_via8233_playback_close),
    ioctl: ptr::null(),
    hw_params: Some(snd_via82xx_hw_params),
    hw_free: Some(snd_via82xx_hw_free),
    prepare: Some(snd_via8233_playback_prepare),
    trigger: Some(snd_via82xx_pcm_trigger),
    pointer: Some(snd_via8233_pcm_pointer),
};

static snd_via8233_multi_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_via8233_multi_open),
    close: Some(snd_via82xx_pcm_close),
    ioctl: ptr::null(),
    hw_params: Some(snd_via82xx_hw_params),
    hw_free: Some(snd_via82xx_hw_free),
    prepare: Some(snd_via8233_multi_prepare),
    trigger: Some(snd_via82xx_pcm_trigger),
    pointer: Some(snd_via8233_pcm_pointer),
};

static snd_via8233_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_via82xx_capture_open),
    close: Some(snd_via82xx_pcm_close),
    ioctl: ptr::null(),
    hw_params: Some(snd_via82xx_hw_params),
    hw_free: Some(snd_via82xx_hw_free),
    prepare: Some(snd_via8233_capture_prepare),
    trigger: Some(snd_via82xx_pcm_trigger),
    pointer: Some(snd_via8233_pcm_pointer),
};

unsafe fn init_viadev(chip: *mut via82xx, idx: c_int, reg_offset: c_uint, shadow_pos: c_int, direction: c_int) {
    unsafe {
        (*chip).devs[idx as usize].reg_offset = reg_offset;
        (*chip).devs[idx as usize].shadow_shift = shadow_pos * 4;
        (*chip).devs[idx as usize].direction = direction;
        (*chip).devs[idx as usize].port = (*chip).port + reg_offset as c_ulong;
    }
}

unsafe fn snd_via8233_pcm_new(chip: *mut via82xx) -> c_int {
    unsafe {
        let mut pcm: *mut snd_pcm = ptr::null_mut();
        let mut chmap: *mut snd_pcm_chmap = ptr::null_mut();
        let mut i: c_int;
        let mut err: c_int;

        (*chip).playback_devno = 0; /* x 4 */
        (*chip).multi_devno = 4; /* x 1 */
        (*chip).capture_devno = 5; /* x 2 */
        (*chip).num_devs = 7;
        (*chip).intr_mask = 0x33033333; /* FLAG|EOL for rec0-1, mc, sdx0-3 */

        /* PCM #0:  4 DSX playbacks and 1 capture */
        err = snd_pcm_new((*chip).card, (*(*chip).card).shortname.as_ptr(), 0, 4, 1, &mut pcm);
        if err < 0 {
            return err;
        }
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_via8233_playback_ops);
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_via8233_capture_ops);
        (*pcm).private_data = chip as *mut c_void;
        strscpy((*pcm).name.as_mut_ptr(), (*(*chip).card).shortname.as_ptr());
        (*chip).pcms[0] = pcm;
        /* set up playbacks */
        i = 0;
        while i < 4 {
            init_viadev(chip, i, (0x10 * i) as c_uint, i, 0);
            i += 1;
        }
        /* capture */
        init_viadev(chip, (*chip).capture_devno as c_int, VIA_REG_CAPTURE_8233_STATUS as c_uint, 6, 1);

        snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV_SG, &mut (*(*chip).pci).dev, 64 * 1024, VIA_MAX_BUFSIZE as c_ulong);

        err = snd_pcm_add_chmap_ctls(pcm, SNDRV_PCM_STREAM_PLAYBACK, snd_pcm_std_chmaps, 2, 0, &mut chmap);
        if err < 0 {
            return err;
        }

        /* PCM #1:  multi-channel playback and 2nd capture */
        err = snd_pcm_new((*chip).card, (*(*chip).card).shortname.as_ptr(), 1, 1, 1, &mut pcm);
        if err < 0 {
            return err;
        }
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_via8233_multi_ops);
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_via8233_capture_ops);
        (*pcm).private_data = chip as *mut c_void;
        strscpy((*pcm).name.as_mut_ptr(), (*(*chip).card).shortname.as_ptr());
        (*chip).pcms[1] = pcm;
        /* set up playback */
        init_viadev(chip, (*chip).multi_devno as c_int, VIA_REG_MULTPLAY_STATUS as c_uint, 4, 0);
        /* set up capture */
        init_viadev(chip, ((*chip).capture_devno + 1) as c_int, (VIA_REG_CAPTURE_8233_STATUS + 0x10) as c_uint, 7, 1);

        snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV_SG, &mut (*(*chip).pci).dev, 64 * 1024, VIA_MAX_BUFSIZE as c_ulong);

        err = snd_pcm_add_chmap_ctls(pcm, SNDRV_PCM_STREAM_PLAYBACK, snd_pcm_alt_chmaps, 6, 0, &mut chmap);
        if err < 0 {
            return err;
        }
        (*(*chip).ac97).chmaps[SNDRV_PCM_STREAM_PLAYBACK as usize] = chmap;
    }
    0
}

unsafe fn snd_via8233a_pcm_new(chip: *mut via82xx) -> c_int {
    unsafe {
        let mut pcm: *mut snd_pcm = ptr::null_mut();
        let mut chmap: *mut snd_pcm_chmap = ptr::null_mut();
        let mut err: c_int;

        (*chip).multi_devno = 0;
        (*chip).playback_devno = 1;
        (*chip).capture_devno = 2;
        (*chip).num_devs = 3;
        (*chip).intr_mask = 0x03033000; /* FLAG|EOL for rec0, mc, sdx3 */

        /* PCM #0:  multi-channel playback and capture */
        err = snd_pcm_new((*chip).card, (*(*chip).card).shortname.as_ptr(), 0, 1, 1, &mut pcm);
        if err < 0 {
            return err;
        }
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_via8233_multi_ops);
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_via8233_capture_ops);
        (*pcm).private_data = chip as *mut c_void;
        strscpy((*pcm).name.as_mut_ptr(), (*(*chip).card).shortname.as_ptr());
        (*chip).pcms[0] = pcm;
        /* set up playback */
        init_viadev(chip, (*chip).multi_devno as c_int, VIA_REG_MULTPLAY_STATUS as c_uint, 4, 0);
        /* capture */
        init_viadev(chip, (*chip).capture_devno as c_int, VIA_REG_CAPTURE_8233_STATUS as c_uint, 6, 1);

        snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV_SG, &mut (*(*chip).pci).dev, 64 * 1024, VIA_MAX_BUFSIZE as c_ulong);

        err = snd_pcm_add_chmap_ctls(pcm, SNDRV_PCM_STREAM_PLAYBACK, snd_pcm_alt_chmaps, 6, 0, &mut chmap);
        if err < 0 {
            return err;
        }
        (*(*chip).ac97).chmaps[SNDRV_PCM_STREAM_PLAYBACK as usize] = chmap;

        /* SPDIF supported? */
        if ac97_can_spdif((*chip).ac97) == 0 {
            return 0;
        }

        /* PCM #1:  DXS3 playback (for spdif) */
        err = snd_pcm_new((*chip).card, (*(*chip).card).shortname.as_ptr(), 1, 1, 0, &mut pcm);
        if err < 0 {
            return err;
        }
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_via8233_playback_ops);
        (*pcm).private_data = chip as *mut c_void;
        strscpy((*pcm).name.as_mut_ptr(), (*(*chip).card).shortname.as_ptr());
        (*chip).pcms[1] = pcm;
        /* set up playback */
        init_viadev(chip, (*chip).playback_devno as c_int, 0x30, 3, 0);

        snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV_SG, &mut (*(*chip).pci).dev, 64 * 1024, VIA_MAX_BUFSIZE as c_ulong);
    }
    0
}

unsafe fn snd_via686_pcm_new(chip: *mut via82xx) -> c_int {
    unsafe {
        let mut pcm: *mut snd_pcm = ptr::null_mut();
        let err: c_int;

        (*chip).playback_devno = 0;
        (*chip).capture_devno = 1;
        (*chip).num_devs = 2;
        (*chip).intr_mask = 0x77; /* FLAG | EOL for PB, CP, FM */

        err = snd_pcm_new((*chip).card, (*(*chip).card).shortname.as_ptr(), 0, 1, 1, &mut pcm);
        if err < 0 {
            return err;
        }
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_via686_playback_ops);
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_via686_capture_ops);
        (*pcm).private_data = chip as *mut c_void;
        strscpy((*pcm).name.as_mut_ptr(), (*(*chip).card).shortname.as_ptr());
        (*chip).pcms[0] = pcm;
        init_viadev(chip, 0, VIA_REG_PLAYBACK_STATUS as c_uint, 0, 0);
        init_viadev(chip, 1, VIA_REG_CAPTURE_STATUS as c_uint, 0, 1);

        snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV_SG, &mut (*(*chip).pci).dev, 64 * 1024, VIA_MAX_BUFSIZE as c_ulong);
    }
    0
}

unsafe extern "C" fn snd_via8233_capture_source_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    /* formerly they were "Line" and "Mic", but it looks like that they
     * have nothing to do with the actual physical connections...
     */
    static TEXT0: &[u8] = b"Input1\0";
    static TEXT1: &[u8] = b"Input2\0";
    static texts: [*const c_char; 2] = [TEXT0.as_ptr() as *const c_char, TEXT1.as_ptr() as *const c_char];
    unsafe { snd_ctl_enum_info(uinfo, 1, 2, texts.as_ptr()) }
}

unsafe extern "C" fn snd_via8233_capture_source_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let chip: *mut via82xx = snd_kcontrol_chip(kcontrol);
        let port: c_ulong = (*chip).port + if (*kcontrol).id.index != 0 { VIA_REG_CAPTURE_CHANNEL + 0x10 } else { VIA_REG_CAPTURE_CHANNEL };
        (*ucontrol).value.enumerated.item[0] = if (inb(port) as c_uint & VIA_REG_CAPTURE_CHANNEL_MIC) != 0 { 1 } else { 0 };
    }
    0
}

unsafe extern "C" fn snd_via8233_capture_source_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let chip: *mut via82xx = snd_kcontrol_chip(kcontrol);
        let port: c_ulong = (*chip).port + if (*kcontrol).id.index != 0 { VIA_REG_CAPTURE_CHANNEL + 0x10 } else { VIA_REG_CAPTURE_CHANNEL };
        let mut val: u8;
        let oval: u8;

        spin_lock(&mut (*chip).reg_lock);
        oval = inb(port);
        val = oval & !(VIA_REG_CAPTURE_CHANNEL_MIC as u8);
        if (*ucontrol).value.enumerated.item[0] != 0 {
            val |= VIA_REG_CAPTURE_CHANNEL_MIC as u8;
        }
        if val != oval {
            outb(val, port);
        }
        spin_unlock(&mut (*chip).reg_lock);
        (val != oval) as c_int
    }
}

static mut snd_via8233_capture_source: snd_kcontrol_new = snd_kcontrol_new {
    name: b"Input Source Select\0".as_ptr() as *const c_char,
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    device: 0,
    subdevice: 0,
    index: 0,
    access: 0,
    info: Some(snd_via8233_capture_source_info),
    get: Some(snd_via8233_capture_source_get),
    put: Some(snd_via8233_capture_source_put),
    tlv: snd_kcontrol_tlv { p: ptr::null() },
};

unsafe extern "C" fn snd_via8233_dxs3_spdif_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let chip: *mut via82xx = snd_kcontrol_chip(kcontrol);
        let mut val: u8 = 0;

        pci_read_config_byte((*chip).pci, VIA8233_SPDIF_CTRL, &mut val);
        (*ucontrol).value.integer.value[0] = if (val as c_uint & VIA8233_SPDIF_DX3) != 0 { 1 } else { 0 };
    }
    0
}

unsafe extern "C" fn snd_via8233_dxs3_spdif_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let chip: *mut via82xx = snd_kcontrol_chip(kcontrol);
        let mut val: u8;
        let mut oval: u8 = 0;

        pci_read_config_byte((*chip).pci, VIA8233_SPDIF_CTRL, &mut oval);
        val = oval & !(VIA8233_SPDIF_DX3 as u8);
        if (*ucontrol).value.integer.value[0] != 0 {
            val |= VIA8233_SPDIF_DX3 as u8;
        }
        /* save the spdif flag for rate filtering */
        (*chip).spdif_on = if (*ucontrol).value.integer.value[0] != 0 { 1 } else { 0 };
        if val != oval {
            pci_write_config_byte((*chip).pci, VIA8233_SPDIF_CTRL, val);
            return 1;
        }
    }
    0
}

static snd_via8233_dxs3_spdif_control: snd_kcontrol_new = snd_kcontrol_new {
    name: b"IEC958 Output Switch\0".as_ptr() as *const c_char,
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    device: 0,
    subdevice: 0,
    index: 0,
    access: 0,
    info: Some(snd_ctl_boolean_mono_info),
    get: Some(snd_via8233_dxs3_spdif_get),
    put: Some(snd_via8233_dxs3_spdif_put),
    tlv: snd_kcontrol_tlv { p: ptr::null() },
};

unsafe extern "C" fn snd_via8233_dxs_volume_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = 2;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = VIA_DXS_MAX_VOLUME as c_long;
    }
    0
}

unsafe extern "C" fn snd_via8233_dxs_volume_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let chip: *mut via82xx = snd_kcontrol_chip(kcontrol);
        let idx: c_uint = (*kcontrol).id.subdevice;

        (*ucontrol).value.integer.value[0] = (VIA_DXS_MAX_VOLUME - (*chip).playback_volume[idx as usize][0] as c_uint) as c_long;
        (*ucontrol).value.integer.value[1] = (VIA_DXS_MAX_VOLUME - (*chip).playback_volume[idx as usize][1] as c_uint) as c_long;
    }
    0
}

unsafe extern "C" fn snd_via8233_pcmdxs_volume_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let chip: *mut via82xx = snd_kcontrol_chip(kcontrol);
        (*ucontrol).value.integer.value[0] = (VIA_DXS_MAX_VOLUME - (*chip).playback_volume_c[0] as c_uint) as c_long;
        (*ucontrol).value.integer.value[1] = (VIA_DXS_MAX_VOLUME - (*chip).playback_volume_c[1] as c_uint) as c_long;
    }
    0
}

unsafe extern "C" fn snd_via8233_dxs_volume_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let chip: *mut via82xx = snd_kcontrol_chip(kcontrol);
        let idx: c_uint = (*kcontrol).id.subdevice;
        let port: c_ulong = (*chip).port + 0x10 * idx as c_ulong;
        let mut val: u8;
        let mut i: c_int;
        let mut change: c_int = 0;

        i = 0;
        while i < 2 {
            val = (*ucontrol).value.integer.value[i as usize] as u8;
            if val as c_uint > VIA_DXS_MAX_VOLUME {
                val = VIA_DXS_MAX_VOLUME as u8;
            }
            val = (VIA_DXS_MAX_VOLUME as u8).wrapping_sub(val);
            change |= (val != (*chip).playback_volume[idx as usize][i as usize]) as c_int;
            if change != 0 {
                (*chip).playback_volume[idx as usize][i as usize] = val;
                outb(val, port + VIA_REG_OFS_PLAYBACK_VOLUME_L + i as c_ulong);
            }
            i += 1;
        }
        change
    }
}

unsafe extern "C" fn snd_via8233_pcmdxs_volume_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let chip: *mut via82xx = snd_kcontrol_chip(kcontrol);
        let mut idx: c_uint;
        let mut val: u8;
        let mut i: c_int;
        let mut change: c_int = 0;

        i = 0;
        while i < 2 {
            val = (*ucontrol).value.integer.value[i as usize] as u8;
            if val as c_uint > VIA_DXS_MAX_VOLUME {
                val = VIA_DXS_MAX_VOLUME as u8;
            }
            val = (VIA_DXS_MAX_VOLUME as u8).wrapping_sub(val);
            if val != (*chip).playback_volume_c[i as usize] {
                change = 1;
                (*chip).playback_volume_c[i as usize] = val;
                idx = 0;
                while idx < 4 {
                    let port: c_ulong = (*chip).port + 0x10 * idx as c_ulong;
                    (*chip).playback_volume[idx as usize][i as usize] = val;
                    outb(val, port + VIA_REG_OFS_PLAYBACK_VOLUME_L + i as c_ulong);
                    idx += 1;
                }
            }
            i += 1;
        }
        change
    }
}

static db_scale_dxs: [c_uint; 4] = [0, (-4650i32) as c_uint, 150, 1];

static snd_via8233_pcmdxs_volume_control: snd_kcontrol_new = snd_kcontrol_new {
    name: b"PCM Playback Volume\0".as_ptr() as *const c_char,
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    device: 0,
    subdevice: 0,
    index: 0,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
    info: Some(snd_via8233_dxs_volume_info),
    get: Some(snd_via8233_pcmdxs_volume_get),
    put: Some(snd_via8233_pcmdxs_volume_put),
    tlv: snd_kcontrol_tlv { p: db_scale_dxs.as_ptr() },
};

static snd_via8233_dxs_volume_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_PCM,
    device: 0,
    subdevice: 0,
    name: b"PCM Playback Volume\0".as_ptr() as *const c_char,
    index: 0,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ | SNDRV_CTL_ELEM_ACCESS_INACTIVE,
    info: Some(snd_via8233_dxs_volume_info),
    get: Some(snd_via8233_dxs_volume_get),
    put: Some(snd_via8233_dxs_volume_put),
    tlv: snd_kcontrol_tlv { p: db_scale_dxs.as_ptr() },
};

unsafe extern "C" fn snd_via82xx_mixer_free_ac97_bus(bus: *mut snd_ac97_bus) {
    unsafe {
        let chip: *mut via82xx = (*bus).private_data as *mut via82xx;
        (*chip).ac97_bus = ptr::null_mut();
    }
}

unsafe extern "C" fn snd_via82xx_mixer_free_ac97(ac97: *mut snd_ac97) {
    unsafe {
        let chip: *mut via82xx = (*ac97).private_data as *mut via82xx;
        (*chip).ac97 = ptr::null_mut();
    }
}

static ac97_quirks: [ac97_quirk; 14] = [
    ac97_quirk { subvendor: 0x1106, subdevice: 0x4161, codec_id: 0x56494161, name: b"Soltek SL-75DRV5\0".as_ptr() as *const c_char, type_: AC97_TUNE_NONE },
    ac97_quirk { subvendor: 0x1106, subdevice: 0x4161, codec_id: 0, name: b"ASRock K7VT2\0".as_ptr() as *const c_char, type_: AC97_TUNE_HP_ONLY },
    ac97_quirk { subvendor: 0x110a, subdevice: 0x0079, codec_id: 0, name: b"Fujitsu Siemens D1289\0".as_ptr() as *const c_char, type_: AC97_TUNE_HP_ONLY },
    ac97_quirk { subvendor: 0x1019, subdevice: 0x0a81, codec_id: 0, name: b"ECS K7VTA3\0".as_ptr() as *const c_char, type_: AC97_TUNE_HP_ONLY },
    ac97_quirk { subvendor: 0x1019, subdevice: 0x0a85, codec_id: 0, name: b"ECS L7VMM2\0".as_ptr() as *const c_char, type_: AC97_TUNE_HP_ONLY },
    ac97_quirk { subvendor: 0x1019, subdevice: 0x1841, codec_id: 0, name: b"ECS K7VTA3\0".as_ptr() as *const c_char, type_: AC97_TUNE_HP_ONLY },
    ac97_quirk { subvendor: 0x1849, subdevice: 0x3059, codec_id: 0, name: b"ASRock K7VM2\0".as_ptr() as *const c_char, type_: AC97_TUNE_HP_ONLY },
    ac97_quirk { subvendor: 0x14cd, subdevice: 0x7002, codec_id: 0, name: b"Unknown\0".as_ptr() as *const c_char, type_: AC97_TUNE_ALC_JACK },
    ac97_quirk { subvendor: 0x1071, subdevice: 0x8590, codec_id: 0, name: b"Mitac Mobo\0".as_ptr() as *const c_char, type_: AC97_TUNE_ALC_JACK },
    ac97_quirk { subvendor: 0x161f, subdevice: 0x202b, codec_id: 0, name: b"Arima Notebook\0".as_ptr() as *const c_char, type_: AC97_TUNE_HP_ONLY },
    ac97_quirk { subvendor: 0x161f, subdevice: 0x2032, codec_id: 0, name: b"Targa Traveller 811\0".as_ptr() as *const c_char, type_: AC97_TUNE_HP_ONLY },
    ac97_quirk { subvendor: 0x161f, subdevice: 0x2032, codec_id: 0, name: b"m680x\0".as_ptr() as *const c_char, type_: AC97_TUNE_HP_ONLY },
    ac97_quirk { subvendor: 0x1297, subdevice: 0xa232, codec_id: 0, name: b"Shuttle AK32VN\0".as_ptr() as *const c_char, type_: AC97_TUNE_HP_ONLY },
    ac97_quirk { subvendor: 0, subdevice: 0, codec_id: 0, name: ptr::null(), type_: 0 },
];

unsafe fn snd_via82xx_mixer_new(chip: *mut via82xx, quirk_override: *const c_char) -> c_int {
    unsafe {
        let mut ac97: snd_ac97_template = mem::zeroed();
        let mut err: c_int;
        static ops: snd_ac97_bus_ops = snd_ac97_bus_ops {
            write: Some(snd_via82xx_codec_write),
            read: Some(snd_via82xx_codec_read),
            wait: Some(snd_via82xx_codec_wait),
        };

        err = snd_ac97_bus((*chip).card, 0, &ops, chip as *mut c_void, &mut (*chip).ac97_bus);
        if err < 0 {
            return err;
        }
        (*(*chip).ac97_bus).private_free = Some(snd_via82xx_mixer_free_ac97_bus);
        (*(*chip).ac97_bus).clock = (*chip).ac97_clock;

        ac97.private_data = chip as *mut c_void;
        ac97.private_free = Some(snd_via82xx_mixer_free_ac97);
        ac97.pci = (*chip).pci;
        ac97.scaps = AC97_SCAP_SKIP_MODEM | AC97_SCAP_POWER_SAVE;
        err = snd_ac97_mixer((*chip).ac97_bus, &mut ac97, &mut (*chip).ac97);
        if err < 0 {
            return err;
        }

        snd_ac97_tune_hardware((*chip).ac97, ac97_quirks.as_ptr(), quirk_override);

        if (*chip).chip_type != TYPE_VIA686 {
            /* use slot 10/11 */
            snd_ac97_update_bits((*chip).ac97, AC97_EXTENDED_STATUS, 0x03 << 4, 0x03 << 4);
        }
    }
    0
}

const JOYSTICK_ADDR: c_ulong = 0x200;
unsafe fn snd_via686_create_gameport(chip: *mut via82xx, legacy: *mut u8) -> c_int {
    unsafe {
        let gp: *mut gameport;

        if !joystick {
            return -ENODEV;
        }

        if devm_request_region((*(*chip).card).dev, JOYSTICK_ADDR, 8, b"VIA686 gameport\0".as_ptr() as *const c_char).is_null() {
            dev_warn((*(*chip).card).dev, b"cannot reserve joystick port %#x\n\0".as_ptr() as *const c_char, JOYSTICK_ADDR as c_uint);
            return -EBUSY;
        }

        gp = gameport_allocate_port();
        (*chip).gameport = gp;
        if gp.is_null() {
            dev_err((*(*chip).card).dev, b"cannot allocate memory for gameport\n\0".as_ptr() as *const c_char);
            return -ENOMEM;
        }

        gameport_set_name(gp, b"VIA686 Gameport\0".as_ptr() as *const c_char);
        gameport_set_phys(gp, b"pci%s/gameport0\0".as_ptr() as *const c_char, pci_name((*chip).pci));
        gameport_set_dev_parent(gp, &mut (*(*chip).pci).dev);
        (*gp).io = JOYSTICK_ADDR;

        /* Enable legacy joystick port */
        *legacy |= VIA_FUNC_ENABLE_GAME as u8;
        pci_write_config_byte((*chip).pci, VIA_FUNC_ENABLE, *legacy);

        gameport_register_port((*chip).gameport);
    }
    0
}

unsafe fn snd_via686_free_gameport(chip: *mut via82xx) {
    unsafe {
        if !(*chip).gameport.is_null() {
            gameport_unregister_port((*chip).gameport);
            (*chip).gameport = ptr::null_mut();
        }
    }
}

unsafe fn snd_via8233_init_misc(chip: *mut via82xx) -> c_int {
    unsafe {
        let mut i: c_int;
        let mut err: c_int;
        let caps: c_int;
        let mut val: u8 = 0;

        caps = if (*chip).chip_type == TYPE_VIA8233A { 1 } else { 2 };
        i = 0;
        while i < caps {
            snd_via8233_capture_source.index = i as c_uint;
            err = snd_ctl_add((*chip).card, snd_ctl_new1(&raw const snd_via8233_capture_source, chip as *mut c_void));
            if err < 0 {
                return err;
            }
            i += 1;
        }
        if ac97_can_spdif((*chip).ac97) != 0 {
            err = snd_ctl_add((*chip).card, snd_ctl_new1(&snd_via8233_dxs3_spdif_control, chip as *mut c_void));
            if err < 0 {
                return err;
            }
        }
        if (*chip).chip_type != TYPE_VIA8233A {
            /* when no h/w PCM volume control is found, use DXS volume control
             * as the PCM vol control
             */
            if snd_ctl_find_id_mixer((*chip).card, b"PCM Playback Volume\0".as_ptr() as *const c_char).is_null() {
                dev_info((*(*chip).card).dev, b"Using DXS as PCM Playback\n\0".as_ptr() as *const c_char);
                err = snd_ctl_add((*chip).card, snd_ctl_new1(&snd_via8233_pcmdxs_volume_control, chip as *mut c_void));
                if err < 0 {
                    return err;
                }
            } else {
                /* Using DXS when PCM emulation is enabled is really weird */
                i = 0;
                while i < 4 {
                    let kctl: *mut snd_kcontrol;

                    kctl = snd_ctl_new1(&snd_via8233_dxs_volume_control, chip as *mut c_void);
                    if kctl.is_null() {
                        return -ENOMEM;
                    }
                    (*kctl).id.subdevice = i as c_uint;
                    err = snd_ctl_add((*chip).card, kctl);
                    if err < 0 {
                        return err;
                    }
                    (*chip).dxs_controls[i as usize] = kctl;
                    i += 1;
                }
            }
        }
        /* select spdif data slot 10/11 */
        pci_read_config_byte((*chip).pci, VIA8233_SPDIF_CTRL, &mut val);
        val = ((val as c_uint & !VIA8233_SPDIF_SLOT_MASK) | VIA8233_SPDIF_SLOT_1011) as u8;
        val &= !(VIA8233_SPDIF_DX3 as u8); /* SPDIF off as default */
        pci_write_config_byte((*chip).pci, VIA8233_SPDIF_CTRL, val);
    }
    0
}

unsafe fn snd_via686_init_misc(chip: *mut via82xx) -> c_int {
    unsafe {
        let mut legacy: u8;
        let mut legacy_cfg: u8;
        let mut rev_h: c_int = 0;

        legacy = (*chip).old_legacy;
        legacy_cfg = (*chip).old_legacy_cfg;
        legacy |= VIA_FUNC_MIDI_IRQMASK as u8; /* FIXME: correct? (disable MIDI) */
        legacy &= !(VIA_FUNC_ENABLE_GAME as u8); /* disable joystick */
        if (*chip).revision as c_uint >= VIA_REV_686_H {
            rev_h = 1;
            if mpu_port >= 0x200 {
                /* force MIDI */
                mpu_port &= 0xfffc;
                pci_write_config_dword((*chip).pci, 0x18, (mpu_port | 0x01) as c_uint);
                (*chip).mpu_port_saved = mpu_port as c_uint;
            } else {
                mpu_port = pci_resource_start((*chip).pci, 2) as c_long;
            }
        } else {
            match mpu_port {
                0x300 | 0x310 | 0x320 | 0x330 => {
                    legacy_cfg &= !(3 << 2);
                    legacy_cfg |= ((mpu_port & 0x0030) >> 2) as u8;
                }
                _ => {
                    if (legacy as c_uint & VIA_FUNC_ENABLE_MIDI) != 0 {
                        mpu_port = 0x300 + ((legacy_cfg as c_long & 0x000c) << 2);
                    }
                }
            }
        }
        if mpu_port >= 0x200 {
            (*chip).mpu_res = devm_request_region(&mut (*(*chip).pci).dev, mpu_port as c_ulong, 2, b"VIA82xx MPU401\0".as_ptr() as *const c_char);
        }
        if !(*chip).mpu_res.is_null() {
            if rev_h != 0 {
                legacy |= VIA_FUNC_MIDI_PNP as u8; /* enable PCI I/O 2 */
            }
            legacy |= VIA_FUNC_ENABLE_MIDI as u8;
        } else {
            if rev_h != 0 {
                legacy &= !(VIA_FUNC_MIDI_PNP as u8); /* disable PCI I/O 2 */
            }
            legacy &= !(VIA_FUNC_ENABLE_MIDI as u8);
            mpu_port = 0;
        }

        pci_write_config_byte((*chip).pci, VIA_FUNC_ENABLE, legacy);
        pci_write_config_byte((*chip).pci, VIA_PNP_CONTROL, legacy_cfg);
        if !(*chip).mpu_res.is_null() {
            if snd_mpu401_uart_new(
                (*chip).card,
                0,
                MPU401_HW_VIA686A,
                mpu_port as c_ulong,
                MPU401_INFO_INTEGRATED | MPU401_INFO_IRQ_HOOK,
                -1,
                &mut (*chip).rmidi,
            ) < 0
            {
                dev_warn((*(*chip).card).dev, b"unable to initialize MPU-401 at 0x%lx, skipping\n\0".as_ptr() as *const c_char, mpu_port as c_ulong);
                legacy &= !(VIA_FUNC_ENABLE_MIDI as u8);
            } else {
                legacy &= !(VIA_FUNC_MIDI_IRQMASK as u8); /* enable MIDI interrupt */
            }
            pci_write_config_byte((*chip).pci, VIA_FUNC_ENABLE, legacy);
        }

        snd_via686_create_gameport(chip, &mut legacy);

        (*chip).legacy_saved = legacy;
        (*chip).legacy_cfg_saved = legacy_cfg;
    }
    0
}

unsafe extern "C" fn snd_via82xx_proc_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    unsafe {
        let chip: *mut via82xx = (*entry).private_data as *mut via82xx;
        let mut i: c_int;

        snd_iprintf(buffer, b"%s\n\n\0".as_ptr() as *const c_char, (*(*chip).card).longname.as_ptr());
        i = 0;
        while i < 0xa0 {
            snd_iprintf(buffer, b"%02x: %08x\n\0".as_ptr() as *const c_char, i, inl((*chip).port + i as c_ulong));
            i += 4;
        }
    }
}

unsafe fn snd_via82xx_proc_init(chip: *mut via82xx) {
    unsafe {
        snd_card_ro_proc_new((*chip).card, b"via82xx\0".as_ptr() as *const c_char, chip as *mut c_void, snd_via82xx_proc_read);
    }
}

unsafe fn snd_via82xx_chip_init(chip: *mut via82xx) -> c_int {
    unsafe {
        let mut val: c_uint;
        let mut end_time: c_ulong;
        let mut pval: u8 = 0;

        /* Disabled C block: broken on K7M? disable all legacy ports for TYPE_VIA686. */
        pci_read_config_byte((*chip).pci, VIA_ACLINK_STAT, &mut pval);
        if (pval as c_uint & VIA_ACLINK_C00_READY) == 0 {
            /* codec not ready? */
            /* deassert ACLink reset, force SYNC */
            pci_write_config_byte(
                (*chip).pci,
                VIA_ACLINK_CTRL,
                (VIA_ACLINK_CTRL_ENABLE | VIA_ACLINK_CTRL_RESET | VIA_ACLINK_CTRL_SYNC) as u8,
            );
            udelay(100);
            /* FIXME: should we do full reset here for all chip models? */
            pci_write_config_byte((*chip).pci, VIA_ACLINK_CTRL, 0x00);
            udelay(100);
            /* ACLink on, deassert ACLink reset, VSR, SGD data out */
            /* note - FM data out has trouble with non VRA codecs !! */
            pci_write_config_byte((*chip).pci, VIA_ACLINK_CTRL, VIA_ACLINK_CTRL_INIT as u8);
            udelay(100);
        }

        /* Make sure VRA is enabled, in case we didn't do a
         * complete codec reset, above */
        pci_read_config_byte((*chip).pci, VIA_ACLINK_CTRL, &mut pval);
        if (pval as c_uint & VIA_ACLINK_CTRL_INIT) != VIA_ACLINK_CTRL_INIT {
            /* ACLink on, deassert ACLink reset, VSR, SGD data out */
            /* note - FM data out has trouble with non VRA codecs !! */
            pci_write_config_byte((*chip).pci, VIA_ACLINK_CTRL, VIA_ACLINK_CTRL_INIT as u8);
            udelay(100);
        }

        /* wait until codec ready */
        end_time = jiffies.wrapping_add(msecs_to_jiffies(750));
        loop {
            pci_read_config_byte((*chip).pci, VIA_ACLINK_STAT, &mut pval);
            if (pval as c_uint & VIA_ACLINK_C00_READY) != 0 {
                break;
            }
            schedule_timeout_uninterruptible(1);
            if time_before(jiffies, end_time) == 0 {
                break;
            }
        }

        val = snd_via82xx_codec_xread(chip);
        if (val & VIA_REG_AC97_BUSY) != 0 {
            dev_err((*(*chip).card).dev, b"AC'97 codec is not ready [0x%x]\n\0".as_ptr() as *const c_char, val);
        }

        /* Disabled C block: secondary AC'97 codec detection is skipped. */

        if (*chip).chip_type == TYPE_VIA686 {
            /* route FM trap to IRQ, disable FM trap */
            pci_write_config_byte((*chip).pci, VIA_FM_NMI_CTRL, 0);
            /* disable all GPI interrupts */
            outl(0, VIAREG(chip, VIA_REG_GPI_INTR));
        }

        if (*chip).chip_type != TYPE_VIA686 {
            /* Workaround for Award BIOS bug:
             * DXS channels don't work properly with VRA if MC97 is disabled.
             */
            let pci: *mut pci_dev;
            pci = pci_get_device(0x1106, 0x3068, ptr::null_mut()); /* MC97 */
            if !pci.is_null() {
                let mut data: u8 = 0;
                pci_read_config_byte(pci, 0x44, &mut data);
                pci_write_config_byte(pci, 0x44, data | 0x40);
                pci_dev_put(pci);
            }
        }

        if (*chip).chip_type != TYPE_VIA8233A {
            let mut i: c_int;
            let mut idx: c_int;
            idx = 0;
            while idx < 4 {
                let port: c_ulong = (*chip).port + 0x10 * idx as c_ulong;
                i = 0;
                while i < 2 {
                    (*chip).playback_volume[idx as usize][i as usize] = (*chip).playback_volume_c[i as usize];
                    outb((*chip).playback_volume_c[i as usize], port + VIA_REG_OFS_PLAYBACK_VOLUME_L + i as c_ulong);
                    i += 1;
                }
                idx += 1;
            }
        }
    }
    0
}

unsafe extern "C" fn snd_via82xx_suspend(dev: *mut device) -> c_int {
    unsafe {
        let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;
        let chip: *mut via82xx = (*card).private_data as *mut via82xx;
        let mut i: c_int;

        snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
        i = 0;
        while i < (*chip).num_devs as c_int {
            snd_via82xx_channel_reset(chip, &mut (*chip).devs[i as usize]);
            i += 1;
        }
        snd_ac97_suspend((*chip).ac97);

        /* save misc values */
        if (*chip).chip_type != TYPE_VIA686 {
            pci_read_config_byte((*chip).pci, VIA8233_SPDIF_CTRL, &mut (*chip).spdif_ctrl_saved);
            (*chip).capture_src_saved[0] = inb((*chip).port + VIA_REG_CAPTURE_CHANNEL);
            (*chip).capture_src_saved[1] = inb((*chip).port + VIA_REG_CAPTURE_CHANNEL + 0x10);
        }
    }
    0
}

unsafe extern "C" fn snd_via82xx_resume(dev: *mut device) -> c_int {
    unsafe {
        let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;
        let chip: *mut via82xx = (*card).private_data as *mut via82xx;
        let mut i: c_int;

        snd_via82xx_chip_init(chip);

        if (*chip).chip_type == TYPE_VIA686 {
            if (*chip).mpu_port_saved != 0 {
                pci_write_config_dword((*chip).pci, 0x18, (*chip).mpu_port_saved | 0x01);
            }
            pci_write_config_byte((*chip).pci, VIA_FUNC_ENABLE, (*chip).legacy_saved);
            pci_write_config_byte((*chip).pci, VIA_PNP_CONTROL, (*chip).legacy_cfg_saved);
        } else {
            pci_write_config_byte((*chip).pci, VIA8233_SPDIF_CTRL, (*chip).spdif_ctrl_saved);
            outb((*chip).capture_src_saved[0], (*chip).port + VIA_REG_CAPTURE_CHANNEL);
            outb((*chip).capture_src_saved[1], (*chip).port + VIA_REG_CAPTURE_CHANNEL + 0x10);
        }

        snd_ac97_resume((*chip).ac97);

        i = 0;
        while i < (*chip).num_devs as c_int {
            snd_via82xx_channel_reset(chip, &mut (*chip).devs[i as usize]);
            i += 1;
        }

        snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    }
    0
}

static snd_via82xx_pm: [unsafe extern "C" fn(*mut device) -> c_int; 2] = [snd_via82xx_suspend, snd_via82xx_resume];

unsafe extern "C" fn snd_via82xx_free(card: *mut snd_card) {
    unsafe {
        let chip: *mut via82xx = (*card).private_data as *mut via82xx;
        let mut i: c_uint;

        /* disable interrupts */
        i = 0;
        while i < (*chip).num_devs {
            snd_via82xx_channel_reset(chip, &mut (*chip).devs[i as usize]);
            i += 1;
        }

        if (*chip).chip_type == TYPE_VIA686 {
            snd_via686_free_gameport(chip);
            pci_write_config_byte((*chip).pci, VIA_FUNC_ENABLE, (*chip).old_legacy);
            pci_write_config_byte((*chip).pci, VIA_PNP_CONTROL, (*chip).old_legacy_cfg);
        }
    }
}

unsafe fn snd_via82xx_create(card: *mut snd_card, pci: *mut pci_dev, chip_type: c_int, revision: c_int, ac97_clock_: c_uint) -> c_int {
    unsafe {
        let chip: *mut via82xx = (*card).private_data as *mut via82xx;
        let mut err: c_int;

        err = pcim_enable_device(pci);
        if err < 0 {
            return err;
        }

        (*chip).chip_type = chip_type;
        (*chip).revision = revision as u8;

        spin_lock_init(&mut (*chip).reg_lock);
        spin_lock_init(&mut (*chip).rates[0].lock);
        spin_lock_init(&mut (*chip).rates[1].lock);
        (*chip).card = card;
        (*chip).pci = pci;
        (*chip).irq = -1;

        pci_read_config_byte(pci, VIA_FUNC_ENABLE, &mut (*chip).old_legacy);
        pci_read_config_byte(pci, VIA_PNP_CONTROL, &mut (*chip).old_legacy_cfg);
        pci_write_config_byte((*chip).pci, VIA_FUNC_ENABLE, (*chip).old_legacy & !((VIA_FUNC_ENABLE_SB | VIA_FUNC_ENABLE_FM) as u8));

        err = pcim_request_all_regions(pci, (*card).driver.as_ptr());
        if err < 0 {
            return err;
        }
        (*chip).port = pci_resource_start(pci, 0);
        if devm_request_irq(
            &mut (*pci).dev,
            (*pci).irq,
            if chip_type == TYPE_VIA8233 { snd_via8233_interrupt } else { snd_via686_interrupt },
            IRQF_SHARED,
            KBUILD_MODNAME,
            chip as *mut c_void,
        ) != 0
        {
            dev_err((*card).dev, b"unable to grab IRQ %d\n\0".as_ptr() as *const c_char, (*pci).irq);
            return -EBUSY;
        }
        (*chip).irq = (*pci).irq;
        (*card).sync_irq = (*chip).irq;
        (*card).private_free = Some(snd_via82xx_free);
        if ac97_clock_ >= 8000 && ac97_clock_ <= 48000 {
            (*chip).ac97_clock = ac97_clock_;
        }

        err = snd_via82xx_chip_init(chip);
        if err < 0 {
            return err;
        }

        /* The 8233 ac97 controller does not implement the master bit
         * in the pci command register. IMHO this is a violation of the PCI spec.
         * We call pci_set_master here because it does not hurt. */
        pci_set_master(pci);
    }
    0
}

#[repr(C)]
pub struct via823x_info {
    pub revision: c_int,
    pub name: *mut c_char,
    pub type_: c_int,
}

static via823x_cards: [via823x_info; 7] = [
    via823x_info { revision: VIA_REV_PRE_8233 as c_int, name: b"VIA 8233-Pre\0".as_ptr() as *mut c_char, type_: TYPE_VIA8233 },
    via823x_info { revision: VIA_REV_8233C as c_int, name: b"VIA 8233C\0".as_ptr() as *mut c_char, type_: TYPE_VIA8233 },
    via823x_info { revision: VIA_REV_8233 as c_int, name: b"VIA 8233\0".as_ptr() as *mut c_char, type_: TYPE_VIA8233 },
    via823x_info { revision: VIA_REV_8233A as c_int, name: b"VIA 8233A\0".as_ptr() as *mut c_char, type_: TYPE_VIA8233A },
    via823x_info { revision: VIA_REV_8235 as c_int, name: b"VIA 8235\0".as_ptr() as *mut c_char, type_: TYPE_VIA8233 },
    via823x_info { revision: VIA_REV_8237 as c_int, name: b"VIA 8237\0".as_ptr() as *mut c_char, type_: TYPE_VIA8233 },
    via823x_info { revision: VIA_REV_8251 as c_int, name: b"VIA 8251\0".as_ptr() as *mut c_char, type_: TYPE_VIA8233 },
];

const fn SND_PCI_QUIRK(subvendor: c_uint, subdevice: c_uint, name: *const c_char, value: c_int) -> snd_pci_quirk {
    snd_pci_quirk { subvendor, subdevice, name, value }
}
const fn SND_PCI_QUIRK_VENDOR(subvendor: c_uint, name: *const c_char, value: c_int) -> snd_pci_quirk {
    snd_pci_quirk { subvendor, subdevice: 0, name, value }
}

static dxs_allowlist: [snd_pci_quirk; 40] = [
    SND_PCI_QUIRK(0x1005, 0x4710, b"Avance Logic Mobo\0".as_ptr() as *const c_char, VIA_DXS_ENABLE),
    SND_PCI_QUIRK(0x1019, 0x0996, b"ESC Mobo\0".as_ptr() as *const c_char, VIA_DXS_48K),
    SND_PCI_QUIRK(0x1019, 0x0a81, b"ECS K7VTA3 v8.0\0".as_ptr() as *const c_char, VIA_DXS_NO_VRA),
    SND_PCI_QUIRK(0x1019, 0x0a85, b"ECS L7VMM2\0".as_ptr() as *const c_char, VIA_DXS_NO_VRA),
    SND_PCI_QUIRK_VENDOR(0x1019, b"ESC K8\0".as_ptr() as *const c_char, VIA_DXS_SRC),
    SND_PCI_QUIRK(0x1019, 0xaa01, b"ESC K8T890-A\0".as_ptr() as *const c_char, VIA_DXS_SRC),
    SND_PCI_QUIRK(0x1025, 0x0033, b"Acer Inspire 1353LM\0".as_ptr() as *const c_char, VIA_DXS_NO_VRA),
    SND_PCI_QUIRK(0x1025, 0x0046, b"Acer Aspire 1524 WLMi\0".as_ptr() as *const c_char, VIA_DXS_SRC),
    SND_PCI_QUIRK_VENDOR(0x1043, b"ASUS A7/A8\0".as_ptr() as *const c_char, VIA_DXS_NO_VRA),
    SND_PCI_QUIRK_VENDOR(0x1071, b"Diverse Notebook\0".as_ptr() as *const c_char, VIA_DXS_NO_VRA),
    SND_PCI_QUIRK(0x10cf, 0x118e, b"FSC Laptop\0".as_ptr() as *const c_char, VIA_DXS_ENABLE),
    SND_PCI_QUIRK_VENDOR(0x1106, b"ASRock\0".as_ptr() as *const c_char, VIA_DXS_SRC),
    SND_PCI_QUIRK(0x1297, 0xa231, b"Shuttle AK31v2\0".as_ptr() as *const c_char, VIA_DXS_SRC),
    SND_PCI_QUIRK(0x1297, 0xa232, b"Shuttle\0".as_ptr() as *const c_char, VIA_DXS_SRC),
    SND_PCI_QUIRK(0x1297, 0xc160, b"Shuttle Sk41G\0".as_ptr() as *const c_char, VIA_DXS_SRC),
    SND_PCI_QUIRK(0x1458, 0xa002, b"Gigabyte GA-7VAXP\0".as_ptr() as *const c_char, VIA_DXS_ENABLE),
    SND_PCI_QUIRK(0x1462, 0x3800, b"MSI KT266\0".as_ptr() as *const c_char, VIA_DXS_ENABLE),
    SND_PCI_QUIRK(0x1462, 0x7120, b"MSI KT4V\0".as_ptr() as *const c_char, VIA_DXS_ENABLE),
    SND_PCI_QUIRK(0x1462, 0x7142, b"MSI K8MM-V\0".as_ptr() as *const c_char, VIA_DXS_ENABLE),
    SND_PCI_QUIRK_VENDOR(0x1462, b"MSI Mobo\0".as_ptr() as *const c_char, VIA_DXS_SRC),
    SND_PCI_QUIRK(0x147b, 0x1401, b"ABIT KD7(-RAID)\0".as_ptr() as *const c_char, VIA_DXS_ENABLE),
    SND_PCI_QUIRK(0x147b, 0x1411, b"ABIT VA-20\0".as_ptr() as *const c_char, VIA_DXS_ENABLE),
    SND_PCI_QUIRK(0x147b, 0x1413, b"ABIT KV8 Pro\0".as_ptr() as *const c_char, VIA_DXS_ENABLE),
    SND_PCI_QUIRK(0x147b, 0x1415, b"ABIT AV8\0".as_ptr() as *const c_char, VIA_DXS_NO_VRA),
    SND_PCI_QUIRK(0x14ff, 0x0403, b"Twinhead mobo\0".as_ptr() as *const c_char, VIA_DXS_ENABLE),
    SND_PCI_QUIRK(0x14ff, 0x0408, b"Twinhead laptop\0".as_ptr() as *const c_char, VIA_DXS_SRC),
    SND_PCI_QUIRK(0x1558, 0x4701, b"Clevo D470\0".as_ptr() as *const c_char, VIA_DXS_SRC),
    SND_PCI_QUIRK(0x1584, 0x8120, b"Diverse Laptop\0".as_ptr() as *const c_char, VIA_DXS_ENABLE),
    SND_PCI_QUIRK(0x1584, 0x8123, b"Targa/Uniwill\0".as_ptr() as *const c_char, VIA_DXS_NO_VRA),
    SND_PCI_QUIRK(0x161f, 0x202b, b"Amira Notebook\0".as_ptr() as *const c_char, VIA_DXS_NO_VRA),
    SND_PCI_QUIRK(0x161f, 0x2032, b"m680x machines\0".as_ptr() as *const c_char, VIA_DXS_48K),
    SND_PCI_QUIRK(0x1631, 0xe004, b"PB EasyNote 3174\0".as_ptr() as *const c_char, VIA_DXS_ENABLE),
    SND_PCI_QUIRK(0x1695, 0x3005, b"EPoX EP-8K9A\0".as_ptr() as *const c_char, VIA_DXS_ENABLE),
    SND_PCI_QUIRK_VENDOR(0x1695, b"EPoX mobo\0".as_ptr() as *const c_char, VIA_DXS_SRC),
    SND_PCI_QUIRK_VENDOR(0x16f3, b"Jetway K8\0".as_ptr() as *const c_char, VIA_DXS_SRC),
    SND_PCI_QUIRK_VENDOR(0x1734, b"FSC Laptop\0".as_ptr() as *const c_char, VIA_DXS_SRC),
    SND_PCI_QUIRK(0x1849, 0x3059, b"ASRock K7VM2\0".as_ptr() as *const c_char, VIA_DXS_NO_VRA),
    SND_PCI_QUIRK_VENDOR(0x1849, b"ASRock mobo\0".as_ptr() as *const c_char, VIA_DXS_SRC),
    SND_PCI_QUIRK(0x1919, 0x200a, b"Soltek SL-K8\0".as_ptr() as *const c_char, VIA_DXS_NO_VRA),
    SND_PCI_QUIRK(0x4005, 0x4710, b"MSI K7T266\0".as_ptr() as *const c_char, VIA_DXS_SRC),
];

unsafe fn check_dxs_list(pci: *mut pci_dev, revision: c_int) -> c_int {
    unsafe {
        let w: *const snd_pci_quirk;

        w = snd_pci_quirk_lookup(pci, dxs_allowlist.as_ptr());
        if !w.is_null() {
            dev_dbg(&mut (*pci).dev, b"DXS allow list for %s found\n\0".as_ptr() as *const c_char, snd_pci_quirk_name(w));
            return (*w).value;
        }

        /* for newer revision, default to DXS_SRC */
        if revision >= VIA_REV_8235 as c_int {
            return VIA_DXS_SRC;
        }

        /*
         * not detected, try 48k rate only to be sure.
         */
        dev_info(&mut (*pci).dev, b"Assuming DXS channels with 48k fixed sample rate.\n\0".as_ptr() as *const c_char);
        dev_info(&mut (*pci).dev, b"         Please try dxs_support=5 option\n\0".as_ptr() as *const c_char);
        dev_info(&mut (*pci).dev, b"         and report if it works on your machine.\n\0".as_ptr() as *const c_char);
        dev_info(&mut (*pci).dev, b"         For more details, read ALSA-Configuration.txt.\n\0".as_ptr() as *const c_char);
    }
    VIA_DXS_48K
}

unsafe fn __snd_via82xx_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    unsafe {
        let mut card: *mut snd_card = ptr::null_mut();
        let chip: *mut via82xx;
        let mut chip_type: c_int = 0;
        let card_type: c_int;
        let mut i: c_uint;
        let mut err: c_int;

        err = snd_devm_card_new(&mut (*pci).dev, index, id, THIS_MODULE, mem::size_of::<via82xx>(), &mut card);
        if err < 0 {
            return err;
        }
        chip = (*card).private_data as *mut via82xx;

        card_type = (*pci_id).driver_data as c_int;
        match card_type {
            TYPE_CARD_VIA686 => {
                strscpy((*card).driver.as_mut_ptr(), b"VIA686A\0".as_ptr() as *const c_char);
                sprintf((*card).shortname.as_mut_ptr(), b"VIA 82C686A/B rev%x\0".as_ptr() as *const c_char, (*pci).revision);
                chip_type = TYPE_VIA686;
            }
            TYPE_CARD_VIA8233 => {
                chip_type = TYPE_VIA8233;
                sprintf((*card).shortname.as_mut_ptr(), b"VIA 823x rev%x\0".as_ptr() as *const c_char, (*pci).revision);
                i = 0;
                while i < via823x_cards.len() as c_uint {
                    if (*pci).revision as c_int == via823x_cards[i as usize].revision {
                        chip_type = via823x_cards[i as usize].type_;
                        strscpy((*card).shortname.as_mut_ptr(), via823x_cards[i as usize].name);
                        break;
                    }
                    i += 1;
                }
                if chip_type != TYPE_VIA8233A {
                    if dxs_support == VIA_DXS_AUTO {
                        dxs_support = check_dxs_list(pci, (*pci).revision as c_int);
                    }
                    /* force to use VIA8233 or 8233A model according to
                     * dxs_support module option
                     */
                    if dxs_support == VIA_DXS_DISABLE {
                        chip_type = TYPE_VIA8233A;
                    } else {
                        chip_type = TYPE_VIA8233;
                    }
                }
                if chip_type == TYPE_VIA8233A {
                    strscpy((*card).driver.as_mut_ptr(), b"VIA8233A\0".as_ptr() as *const c_char);
                } else if (*pci).revision >= VIA_REV_8237 {
                    strscpy((*card).driver.as_mut_ptr(), b"VIA8237\0".as_ptr() as *const c_char); /* no slog assignment */
                } else {
                    strscpy((*card).driver.as_mut_ptr(), b"VIA8233\0".as_ptr() as *const c_char);
                }
            }
            _ => {
                dev_err((*card).dev, b"invalid card type %d\n\0".as_ptr() as *const c_char, card_type);
                return -EINVAL;
            }
        }

        err = snd_via82xx_create(card, pci, chip_type, (*pci).revision as c_int, ac97_clock as c_uint);
        if err < 0 {
            return err;
        }
        err = snd_via82xx_mixer_new(chip, ac97_quirk);
        if err < 0 {
            return err;
        }

        if chip_type == TYPE_VIA686 {
            err = snd_via686_pcm_new(chip);
            if err < 0 {
                return err;
            }
            err = snd_via686_init_misc(chip);
            if err < 0 {
                return err;
            }
        } else {
            if chip_type == TYPE_VIA8233A {
                err = snd_via8233a_pcm_new(chip);
                if err < 0 {
                    return err;
                }
                // chip->dxs_fixed = 1; /* FIXME: use 48k for DXS #3? */
            } else {
                err = snd_via8233_pcm_new(chip);
                if err < 0 {
                    return err;
                }
                if dxs_support == VIA_DXS_48K {
                    (*chip).dxs_fixed = 1;
                } else if dxs_support == VIA_DXS_NO_VRA {
                    (*chip).no_vra = 1;
                } else if dxs_support == VIA_DXS_SRC {
                    (*chip).no_vra = 1;
                    (*chip).dxs_src = 1;
                }
            }
            err = snd_via8233_init_misc(chip);
            if err < 0 {
                return err;
            }
        }

        /* disable interrupts */
        i = 0;
        while i < (*chip).num_devs {
            snd_via82xx_channel_reset(chip, &mut (*chip).devs[i as usize]);
            i += 1;
        }

        snprintf(
            (*card).longname.as_mut_ptr(),
            (*card).longname.len(),
            b"%s with %s at %#lx, irq %d\0".as_ptr() as *const c_char,
            (*card).shortname.as_ptr(),
            snd_ac97_get_short_name((*chip).ac97),
            (*chip).port,
            (*chip).irq,
        );

        snd_via82xx_proc_init(chip);

        err = snd_card_register(card);
        if err < 0 {
            return err;
        }
        pci_set_drvdata(pci, card as *mut c_void);
    }
    0
}

unsafe extern "C" fn snd_via82xx_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    unsafe { snd_card_free_on_error(&mut (*pci).dev, __snd_via82xx_probe(pci, pci_id)) }
}

static mut via82xx_driver: pci_driver = pci_driver {
    name: KBUILD_MODNAME,
    id_table: snd_via82xx_ids.as_ptr(),
    probe: Some(snd_via82xx_probe),
    driver: pci_driver_driver {
        pm: snd_via82xx_pm.as_ptr() as *const c_void,
    },
};

/* module metadata and registration translated from:
 * MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>");
 * MODULE_DESCRIPTION("VIA VT82xx audio");
 * MODULE_LICENSE("GPL");
 * MODULE_DEVICE_TABLE(pci, snd_via82xx_ids);
 * module_pci_driver(via82xx_driver);
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
