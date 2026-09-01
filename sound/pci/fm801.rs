// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  The driver for the ForteMedia FM801 based soundcards
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::zeroed;
use core::ptr::{null, null_mut};

type bool_ = bool;
type u8 = u8;
type u16 = u16;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;
type size_t = usize;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pci_dev {
    pub dev: device,
    pub irq: c_int,
    pub revision: u8,
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
    pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    pub sync_irq: c_int,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
}
#[repr(C)]
pub struct snd_pcm {
    pub private_data: *mut c_void,
    pub info_flags: c_uint,
    pub name: [c_char; 80],
}
#[repr(C)]
pub struct snd_rawmidi {
    pub private_data: *mut c_void,
}
#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
    pub format: c_int,
    pub channels: c_uint,
    pub rate: c_uint,
    pub dma_addr: c_ulong,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}
#[repr(C)]
pub struct snd_ac97_bus {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_ac97 {
    pub private_data: *mut fm801,
    pub num: c_int,
    pub addr: c_uint,
}
#[repr(C)]
pub struct snd_opl3 {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_info_entry {
    _private: [u8; 0],
}
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct v4l2_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_tea575x {
    pub v4l2_dev: *mut v4l2_device,
    pub radio_nr: c_int,
    pub private_data: *mut fm801,
    pub ops: *const snd_tea575x_ops,
    pub bus_info: [c_char; 32],
    pub card: [c_char; 32],
}
#[repr(C)]
pub struct snd_tea575x_ops {
    pub set_pins: Option<unsafe extern "C" fn(*mut snd_tea575x, u8)>,
    pub get_pins: Option<unsafe extern "C" fn(*mut snd_tea575x) -> u8>,
    pub set_direction: Option<unsafe extern "C" fn(*mut snd_tea575x, bool_)>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *const c_uint,
    pub mask: c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: c_uint,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub buffer_bytes_max: c_uint,
    pub period_bytes_min: c_uint,
    pub period_bytes_max: c_uint,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub fifo_size: c_uint,
}
#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub ioctl: *const c_void,
    pub hw_params: *const c_void,
    pub hw_free: *const c_void,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}
#[repr(C)]
pub struct snd_ac97_bus_ops {
    pub write: Option<unsafe extern "C" fn(*mut snd_ac97, c_ushort, c_ushort)>,
    pub read: Option<unsafe extern "C" fn(*mut snd_ac97, c_ushort) -> c_ushort>,
}
type c_ushort = u16;
#[repr(C)]
pub struct snd_ac97_template {
    pub private_data: *mut fm801,
    pub num: c_int,
    pub addr: c_uint,
}
#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}
#[repr(C)]
pub struct snd_ctl_elem_info_integer {
    pub min: c_long,
    pub max: c_long,
}
type c_long = isize;
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
pub union snd_kcontrol_tlv {
    pub p: *const c_uint,
}
#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub access: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
    pub tlv: snd_kcontrol_tlv,
}
#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pci_driver_driver {
    pub pm: *const dev_pm_ops,
}
#[repr(C)]
pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    pub driver: pci_driver_driver,
}

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [null_mut(); SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE_PNP: [bool_; SNDRV_CARDS] = [true; SNDRV_CARDS];

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR; /* ID for this card */
static mut enable: [bool_; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP; /* Enable this card */
/*
 *  Enable TEA575x tuner
 *    1 = MediaForte 256-PCS
 *    2 = MediaForte 256-PCP
 *    3 = MediaForte 64-PCR
 *   16 = setup tuner only (this is additional bit), i.e. SF64-PCR FM card
 *  High 16-bits are video (radio) device number + 1
 */
static mut tea575x_tuner: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
static mut radio_nr: [c_int; SNDRV_CARDS] = [-1; SNDRV_CARDS];

// module_param_array / MODULE_PARM_DESC declarations are Linux module metadata.

const TUNER_DISABLED: c_uint = 1 << 3;
const TUNER_ONLY: c_uint = 1 << 4;
const TUNER_TYPE_MASK: c_uint = (!TUNER_ONLY) & 0xFFFF;

/*
 *  Direct registers
 */

const FM801_PCM_VOL: c_ushort = 0x00; /* PCM Output Volume */
const FM801_FM_VOL: c_ushort = 0x02; /* FM Output Volume */
const FM801_I2S_VOL: c_ushort = 0x04; /* I2S Volume */
const FM801_REC_SRC: c_ushort = 0x06; /* Record Source */
const FM801_PLY_CTRL: c_ushort = 0x08; /* Playback Control */
const FM801_PLY_COUNT: c_ushort = 0x0a; /* Playback Count */
const FM801_PLY_BUF1: c_ushort = 0x0c; /* Playback Bufer I */
const FM801_PLY_BUF2: c_ushort = 0x10; /* Playback Buffer II */
const FM801_CAP_CTRL: c_ushort = 0x14; /* Capture Control */
const FM801_CAP_COUNT: c_ushort = 0x16; /* Capture Count */
const FM801_CAP_BUF1: c_ushort = 0x18; /* Capture Buffer I */
const FM801_CAP_BUF2: c_ushort = 0x1c; /* Capture Buffer II */
const FM801_CODEC_CTRL: c_ushort = 0x22; /* Codec Control */
const FM801_I2S_MODE: c_ushort = 0x24; /* I2S Mode Control */
const FM801_VOLUME: c_ushort = 0x26; /* Volume Up/Down/Mute Status */
const FM801_I2C_CTRL: c_ushort = 0x29; /* I2C Control */
const FM801_AC97_CMD: c_ushort = 0x2a; /* AC'97 Command */
const FM801_AC97_DATA: c_ushort = 0x2c; /* AC'97 Data */
const FM801_MPU401_DATA: c_ushort = 0x30; /* MPU401 Data */
const FM801_MPU401_CMD: c_ushort = 0x31; /* MPU401 Command */
const FM801_GPIO_CTRL: c_ushort = 0x52; /* General Purpose I/O Control */
const FM801_GEN_CTRL: c_ushort = 0x54; /* General Control */
const FM801_IRQ_MASK: c_ushort = 0x56; /* Interrupt Mask */
const FM801_IRQ_STATUS: c_ushort = 0x5a; /* Interrupt Status */
const FM801_OPL3_BANK0: c_ushort = 0x68; /* OPL3 Status Read / Bank 0 Write */
const FM801_OPL3_DATA0: c_ushort = 0x69; /* OPL3 Data 0 Write */
const FM801_OPL3_BANK1: c_ushort = 0x6a; /* OPL3 Bank 1 Write */
const FM801_OPL3_DATA1: c_ushort = 0x6b; /* OPL3 Bank 1 Write */
const FM801_POWERDOWN: c_ushort = 0x70; /* Blocks Power Down Control */

/* codec access */
const FM801_AC97_READ: c_ushort = 1 << 7; /* read=1, write=0 */
const FM801_AC97_VALID: c_ushort = 1 << 8; /* port valid=1 */
const FM801_AC97_BUSY: c_ushort = 1 << 9; /* busy=1 */
const FM801_AC97_ADDR_SHIFT: c_uint = 10; /* codec id (2bit) */

/* playback and record control register bits */
const FM801_BUF1_LAST: c_ushort = 1 << 1;
const FM801_BUF2_LAST: c_ushort = 1 << 2;
const FM801_START: c_ushort = 1 << 5;
const FM801_PAUSE: c_ushort = 1 << 6;
const FM801_IMMED_STOP: c_ushort = 1 << 7;
const FM801_RATE_SHIFT: c_uint = 8;
const FM801_RATE_MASK: c_ushort = 15 << FM801_RATE_SHIFT;
const FM801_CHANNELS_4: c_ushort = 1 << 12; /* playback only */
const FM801_CHANNELS_6: c_ushort = 2 << 12; /* playback only */
const FM801_CHANNELS_6MS: c_ushort = 3 << 12; /* playback only */
const FM801_CHANNELS_MASK: c_ushort = 3 << 12;
const FM801_16BIT: c_ushort = 1 << 14;
const FM801_STEREO: c_ushort = 1 << 15;

/* IRQ status bits */
const FM801_IRQ_PLAYBACK: c_ushort = 1 << 8;
const FM801_IRQ_CAPTURE: c_ushort = 1 << 9;
const FM801_IRQ_VOLUME: c_ushort = 1 << 14;
const FM801_IRQ_MPU: c_ushort = 1 << 15;

/* GPIO control register */
const FM801_GPIO_GP0: c_ushort = 1 << 0; /* read/write */
const FM801_GPIO_GP1: c_ushort = 1 << 1;
const FM801_GPIO_GP2: c_ushort = 1 << 2;
const FM801_GPIO_GP3: c_ushort = 1 << 3;
const fn FM801_GPIO_GP(x: u8) -> c_ushort { 1u16 << (0 + x) }
const FM801_GPIO_GD0: c_ushort = 1 << 8; /* directions: 1 = input, 0 = output*/
const FM801_GPIO_GD1: c_ushort = 1 << 9;
const FM801_GPIO_GD2: c_ushort = 1 << 10;
const FM801_GPIO_GD3: c_ushort = 1 << 11;
const fn FM801_GPIO_GD(x: u8) -> c_ushort { 1u16 << (8 + x) }
const FM801_GPIO_GS0: c_ushort = 1 << 12; /* function select: */
const FM801_GPIO_GS1: c_ushort = 1 << 13; /*    1 = GPIO */
const FM801_GPIO_GS2: c_ushort = 1 << 14; /*    0 = other (S/PDIF, VOL) */
const FM801_GPIO_GS3: c_ushort = 1 << 15;
const fn FM801_GPIO_GS(x: u8) -> c_ushort { 1u16 << (12 + x) }

/**
 * struct fm801 - describes FM801 chip
 * @dev:		device for this chio
 * @irq:		irq number
 * @port:		I/O port number
 * @multichannel:	multichannel support
 * @secondary:		secondary codec
 * @secondary_addr:	address of the secondary codec
 * @tea575x_tuner:	tuner access method & flags
 * @ply_ctrl:		playback control
 * @cap_ctrl:		capture control
 * @ply_buffer:		playback buffer
 * @ply_buf:		playback buffer index
 * @ply_count:		playback buffer count
 * @ply_size:		playback buffer size
 * @ply_pos:		playback position
 * @cap_buffer:		capture buffer
 * @cap_buf:		capture buffer index
 * @cap_count:		capture buffer count
 * @cap_size:		capture buffer size
 * @cap_pos:		capture position
 * @ac97_bus:		ac97 bus handle
 * @ac97:		ac97 handle
 * @ac97_sec:		ac97 secondary handle
 * @card:		ALSA card
 * @pcm:		PCM devices
 * @rmidi:		rmidi device
 * @playback_substream:	substream for playback
 * @capture_substream:	substream for capture
 * @p_dma_size:		playback DMA size
 * @c_dma_size:		capture DMA size
 * @reg_lock:		lock
 * @proc_entry:		/proc entry
 * @v4l2_dev:		v4l2 device
 * @tea:		tea575a structure
 * @saved_regs:		context saved during suspend
 */
#[repr(C)]
pub struct fm801 {
    pub dev: *mut device,
    pub irq: c_int,
    pub port: c_ulong,
    pub multichannel: c_uint,
    pub secondary: c_uint,
    pub secondary_addr: u8,
    pub tea575x_tuner: c_uint,
    pub ply_ctrl: c_ushort,
    pub cap_ctrl: c_ushort,
    pub ply_buffer: c_ulong,
    pub ply_buf: c_uint,
    pub ply_count: c_uint,
    pub ply_size: c_uint,
    pub ply_pos: c_uint,
    pub cap_buffer: c_ulong,
    pub cap_buf: c_uint,
    pub cap_count: c_uint,
    pub cap_size: c_uint,
    pub cap_pos: c_uint,
    pub ac97_bus: *mut snd_ac97_bus,
    pub ac97: *mut snd_ac97,
    pub ac97_sec: *mut snd_ac97,
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub rmidi: *mut snd_rawmidi,
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub p_dma_size: c_uint,
    pub c_dma_size: c_uint,
    pub reg_lock: spinlock_t,
    pub proc_entry: *mut snd_info_entry,
    /* CONFIG_SND_FM801_TEA575X_BOOL */
    pub v4l2_dev: v4l2_device,
    pub tea: snd_tea575x,
    pub saved_regs: [u16; 0x20],
}

extern "C" {
    fn outw(value: c_ushort, port: c_ulong);
    fn inw(port: c_ulong) -> c_ushort;
    fn outl(value: c_ulong, port: c_ulong);
    fn udelay(usecs: c_uint);
    fn schedule_timeout_uninterruptible(timeout: c_long) -> c_long;
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn time_after(a: c_ulong, b: c_ulong) -> bool_;
    static mut jiffies: c_ulong;
    fn snd_BUG();
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut fm801;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: size_t) -> snd_pcm_uframes_t;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_mpu401_uart_interrupt(irq: c_int, private_data: *mut c_void);
    fn snd_pcm_new(card: *mut snd_card, name: *const c_char, device: c_int, playback: c_int, capture: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops);
    fn strscpy(dst: *mut c_char, src: *const c_char, size: usize) -> isize;
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn sprintf(dst: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, ty: c_int, dev: *mut device, min: usize, max: usize);
    fn snd_pcm_add_chmap_ctls(pcm: *mut snd_pcm, stream: c_int, chmaps: *const c_void, max_channels: c_int, private_value: c_int, info: *mut c_void) -> c_int;
    static snd_pcm_alt_chmaps: c_void;
    fn snd_ctl_enum_info(uinfo: *mut snd_ctl_elem_info, channels: c_uint, items: c_uint, texts: *const *const c_char) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut fm801;
    fn snd_ac97_bus(card: *mut snd_card, num: c_int, ops: *const snd_ac97_bus_ops, private_data: *mut fm801, rbus: *mut *mut snd_ac97_bus) -> c_int;
    fn snd_ac97_mixer(bus: *mut snd_ac97_bus, template: *mut snd_ac97_template, rac97: *mut *mut snd_ac97) -> c_int;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut fm801) -> *mut snd_kcontrol;
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn pcim_request_all_regions(pci: *mut pci_dev, name: *const c_char) -> c_int;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, data: *mut fm801) -> c_int;
    fn pci_set_master(pci: *mut pci_dev);
    fn v4l2_device_register(dev: *mut device, v4l2_dev: *mut v4l2_device) -> c_int;
    fn v4l2_device_unregister(v4l2_dev: *mut v4l2_device);
    fn pci_name(pci: *mut pci_dev) -> *const c_char;
    fn snd_tea575x_init(tea: *mut snd_tea575x, module: *mut c_void) -> c_int;
    fn snd_tea575x_exit(tea: *mut snd_tea575x);
    fn snd_tea575x_set_freq(tea: *mut snd_tea575x);
    fn snd_devm_card_new(dev: *mut device, idx: c_int, id: *mut c_char, module: *mut c_void, extra: usize, card: *mut *mut snd_card) -> c_int;
    fn snd_mpu401_uart_new(card: *mut snd_card, device: c_int, hardware: c_int, port: c_ulong, info: c_uint, irq: c_int, rmidi: *mut *mut snd_rawmidi) -> c_int;
    fn snd_opl3_create(card: *mut snd_card, l_port: c_ulong, r_port: c_ulong, hardware: c_int, integrated: c_int, opl3: *mut *mut snd_opl3) -> c_int;
    fn snd_opl3_hwdep_new(opl3: *mut snd_opl3, device: c_int, seq_device: c_int, rhwdep: *mut c_void) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut snd_card);
    fn snd_card_free_on_error(dev: *mut device, err: c_int) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut snd_card;
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn snd_ac97_suspend(ac97: *mut snd_ac97);
    fn snd_ac97_resume(ac97: *mut snd_ac97);
}

const EINVAL: c_int = 22;
const EIO: c_int = 5;
const EBUSY: c_int = 16;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_SHARED: c_ulong = 0x80;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 1;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint = 1 << 2;
const SNDRV_PCM_INFO_PAUSE: c_uint = 1 << 3;
const SNDRV_PCM_INFO_RESUME: c_uint = 1 << 4;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 5;
const SNDRV_PCM_FMTBIT_U8: c_uint = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << 1;
const SNDRV_PCM_RATE_KNOT: c_uint = 1 << 0;
const SNDRV_PCM_RATE_8000_48000: c_uint = 1 << 1;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 1;
const SNDRV_PCM_HW_PARAM_PERIODS: c_int = 2;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_uint = 1;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 2;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 0x3;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 0x4;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;
const AC97_RESET: c_ushort = 0x00;
const AC97_VENDOR_ID1: c_ushort = 0x7c;
const TEA575X_DATA: u8 = 1 << 0;
const TEA575X_CLK: u8 = 1 << 1;
const TEA575X_WREN: u8 = 1 << 2;
const TEA575X_MOST: u8 = 1 << 3;
const MPU401_HW_FM801: c_int = 0;
const MPU401_INFO_INTEGRATED: c_uint = 1 << 0;
const MPU401_INFO_IRQ_HOOK: c_uint = 1 << 1;
const OPL3_HW_OPL3_FM801: c_int = 0;
static mut THIS_MODULE: *mut c_void = null_mut();
static KBUILD_MODNAME: &[u8] = b"fm801\0";

/*
 * IO accessors
 */

unsafe fn fm801_iowrite16(chip: *mut fm801, offset: c_ushort, value: u16) {
    outw(value, (*chip).port + offset as c_ulong);
}

unsafe fn fm801_ioread16(chip: *mut fm801, offset: c_ushort) -> u16 {
    inw((*chip).port + offset as c_ulong)
}

unsafe fn fm801_writew(chip: *mut fm801, reg: c_ushort, value: c_ushort) {
    outw(value, (*chip).port + reg as c_ulong);
}

unsafe fn fm801_readw(chip: *mut fm801, reg: c_ushort) -> c_ushort {
    inw((*chip).port + reg as c_ulong)
}

unsafe fn fm801_writel(chip: *mut fm801, reg: c_ushort, value: c_ulong) {
    outl(value, (*chip).port + reg as c_ulong);
}

static snd_fm801_ids: [pci_device_id; 3] = [
    pci_device_id {
        /* FM801 */
        vendor: 0x1319,
        device: 0x0801,
        subvendor: 0,
        subdevice: 0,
        class: 0x0401 << 8,
        class_mask: 0xffff00,
        driver_data: 0,
    },
    pci_device_id {
        /* Gallant Odyssey Sound 4 */
        vendor: 0x5213,
        device: 0x0510,
        subvendor: 0,
        subdevice: 0,
        class: 0x0401 << 8,
        class_mask: 0xffff00,
        driver_data: 0,
    },
    pci_device_id { vendor: 0, device: 0, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
];

/*
 *  common I/O routines
 */

unsafe fn fm801_ac97_is_ready(chip: *mut fm801, iterations: c_uint) -> bool_ {
    let mut idx: c_uint = 0;
    while idx < iterations {
        if (fm801_readw(chip, FM801_AC97_CMD) & FM801_AC97_BUSY) == 0 {
            return true;
        }
        udelay(10);
        idx += 1;
    }
    false
}

unsafe fn fm801_ac97_is_valid(chip: *mut fm801, iterations: c_uint) -> bool_ {
    let mut idx: c_uint = 0;
    while idx < iterations {
        if (fm801_readw(chip, FM801_AC97_CMD) & FM801_AC97_VALID) != 0 {
            return true;
        }
        udelay(10);
        idx += 1;
    }
    false
}

unsafe fn snd_fm801_update_bits(chip: *mut fm801, reg: c_ushort, mask: c_ushort, value: c_ushort) -> c_int {
    let old = fm801_ioread16(chip, reg);
    let new = (old & !mask) | value;
    let change = old != new;
    if change {
        fm801_iowrite16(chip, reg, new);
    }
    change as c_int
}

unsafe extern "C" fn snd_fm801_codec_write(ac97: *mut snd_ac97, reg: c_ushort, val: c_ushort) {
    let chip = (*ac97).private_data;
    /*
     *  Wait until the codec interface is not ready..
     */
    if !fm801_ac97_is_ready(chip, 100) {
        dev_err((*(*chip).card).dev, b"AC'97 interface is busy (1)\n\0".as_ptr() as *const c_char);
        return;
    }
    /* write data and address */
    fm801_writew(chip, FM801_AC97_DATA, val);
    fm801_writew(chip, FM801_AC97_CMD, reg | (((*ac97).addr as c_ushort) << FM801_AC97_ADDR_SHIFT));
    /*
     *  Wait until the write command is not completed..
     */
    if !fm801_ac97_is_ready(chip, 1000) {
        dev_err((*(*chip).card).dev, b"AC'97 interface #%d is busy (2)\n\0".as_ptr() as *const c_char, (*ac97).num);
    }
}

unsafe extern "C" fn snd_fm801_codec_read(ac97: *mut snd_ac97, reg: c_ushort) -> c_ushort {
    let chip = (*ac97).private_data;
    /*
     *  Wait until the codec interface is not ready..
     */
    if !fm801_ac97_is_ready(chip, 100) {
        dev_err((*(*chip).card).dev, b"AC'97 interface is busy (1)\n\0".as_ptr() as *const c_char);
        return 0;
    }
    /* read command */
    fm801_writew(chip, FM801_AC97_CMD, reg | (((*ac97).addr as c_ushort) << FM801_AC97_ADDR_SHIFT) | FM801_AC97_READ);
    if !fm801_ac97_is_ready(chip, 100) {
        dev_err((*(*chip).card).dev, b"AC'97 interface #%d is busy (2)\n\0".as_ptr() as *const c_char, (*ac97).num);
        return 0;
    }
    if !fm801_ac97_is_valid(chip, 1000) {
        dev_err((*(*chip).card).dev, b"AC'97 interface #%d is not valid (2)\n\0".as_ptr() as *const c_char, (*ac97).num);
        return 0;
    }
    fm801_readw(chip, FM801_AC97_DATA)
}

static rates: [c_uint; 11] = [5500, 8000, 9600, 11025, 16000, 19200, 22050, 32000, 38400, 44100, 48000];

static hw_constraints_rates: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: rates.len() as c_uint,
    list: rates.as_ptr(),
    mask: 0,
};

static channels: [c_uint; 3] = [2, 4, 6];

static hw_constraints_channels: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: channels.len() as c_uint,
    list: channels.as_ptr(),
    mask: 0,
};

/*
 *  Sample rate routines
 */

unsafe fn snd_fm801_rate_bits(rate: c_uint) -> c_ushort {
    let mut idx = 0usize;
    while idx < rates.len() {
        if rates[idx] == rate {
            return idx as c_ushort;
        }
        idx += 1;
    }
    snd_BUG();
    (rates.len() - 1) as c_ushort
}

/*
 *  PCM part
 */

unsafe extern "C" fn snd_fm801_playback_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            (*chip).ply_ctrl &= !(FM801_BUF1_LAST | FM801_BUF2_LAST | FM801_PAUSE);
            (*chip).ply_ctrl |= FM801_START | FM801_IMMED_STOP;
        }
        SNDRV_PCM_TRIGGER_STOP => {
            (*chip).ply_ctrl &= !(FM801_START | FM801_PAUSE);
        }
        SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_SUSPEND => {
            (*chip).ply_ctrl |= FM801_PAUSE;
        }
        SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_RESUME => {
            (*chip).ply_ctrl &= !FM801_PAUSE;
        }
        _ => {
            snd_BUG();
            return -EINVAL;
        }
    }
    fm801_writew(chip, FM801_PLY_CTRL, (*chip).ply_ctrl);
    0
}

unsafe extern "C" fn snd_fm801_capture_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            (*chip).cap_ctrl &= !(FM801_BUF1_LAST | FM801_BUF2_LAST | FM801_PAUSE);
            (*chip).cap_ctrl |= FM801_START | FM801_IMMED_STOP;
        }
        SNDRV_PCM_TRIGGER_STOP => {
            (*chip).cap_ctrl &= !(FM801_START | FM801_PAUSE);
        }
        SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_SUSPEND => {
            (*chip).cap_ctrl |= FM801_PAUSE;
        }
        SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_RESUME => {
            (*chip).cap_ctrl &= !FM801_PAUSE;
        }
        _ => {
            snd_BUG();
            return -EINVAL;
        }
    }
    fm801_writew(chip, FM801_CAP_CTRL, (*chip).cap_ctrl);
    0
}

unsafe extern "C" fn snd_fm801_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    (*chip).ply_size = snd_pcm_lib_buffer_bytes(substream);
    (*chip).ply_count = snd_pcm_lib_period_bytes(substream);
    (*chip).ply_ctrl &= !(FM801_START | FM801_16BIT | FM801_STEREO | FM801_RATE_MASK | FM801_CHANNELS_MASK);
    if snd_pcm_format_width((*runtime).format) == 16 {
        (*chip).ply_ctrl |= FM801_16BIT;
    }
    if (*runtime).channels > 1 {
        (*chip).ply_ctrl |= FM801_STEREO;
        if (*runtime).channels == 4 {
            (*chip).ply_ctrl |= FM801_CHANNELS_4;
        } else if (*runtime).channels == 6 {
            (*chip).ply_ctrl |= FM801_CHANNELS_6;
        }
    }
    (*chip).ply_ctrl |= snd_fm801_rate_bits((*runtime).rate) << FM801_RATE_SHIFT;
    (*chip).ply_buf = 0;
    fm801_writew(chip, FM801_PLY_CTRL, (*chip).ply_ctrl);
    fm801_writew(chip, FM801_PLY_COUNT, ((*chip).ply_count - 1) as c_ushort);
    (*chip).ply_buffer = (*runtime).dma_addr;
    (*chip).ply_pos = 0;
    fm801_writel(chip, FM801_PLY_BUF1, (*chip).ply_buffer);
    fm801_writel(chip, FM801_PLY_BUF2, (*chip).ply_buffer + ((*chip).ply_count % (*chip).ply_size) as c_ulong);
    0
}

unsafe extern "C" fn snd_fm801_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    (*chip).cap_size = snd_pcm_lib_buffer_bytes(substream);
    (*chip).cap_count = snd_pcm_lib_period_bytes(substream);
    (*chip).cap_ctrl &= !(FM801_START | FM801_16BIT | FM801_STEREO | FM801_RATE_MASK);
    if snd_pcm_format_width((*runtime).format) == 16 {
        (*chip).cap_ctrl |= FM801_16BIT;
    }
    if (*runtime).channels > 1 {
        (*chip).cap_ctrl |= FM801_STEREO;
    }
    (*chip).cap_ctrl |= snd_fm801_rate_bits((*runtime).rate) << FM801_RATE_SHIFT;
    (*chip).cap_buf = 0;
    fm801_writew(chip, FM801_CAP_CTRL, (*chip).cap_ctrl);
    fm801_writew(chip, FM801_CAP_COUNT, ((*chip).cap_count - 1) as c_ushort);
    (*chip).cap_buffer = (*runtime).dma_addr;
    (*chip).cap_pos = 0;
    fm801_writel(chip, FM801_CAP_BUF1, (*chip).cap_buffer);
    fm801_writel(chip, FM801_CAP_BUF2, (*chip).cap_buffer + ((*chip).cap_count % (*chip).cap_size) as c_ulong);
    0
}

unsafe extern "C" fn snd_fm801_playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    let mut ptr: size_t;
    if ((*chip).ply_ctrl & FM801_START) == 0 {
        return 0;
    }
    ptr = ((*chip).ply_pos + ((*chip).ply_count - 1) - fm801_readw(chip, FM801_PLY_COUNT) as c_uint) as size_t;
    if (fm801_readw(chip, FM801_IRQ_STATUS) & FM801_IRQ_PLAYBACK) != 0 {
        ptr += (*chip).ply_count as usize;
        ptr %= (*chip).ply_size as usize;
    }
    bytes_to_frames((*substream).runtime, ptr)
}

unsafe extern "C" fn snd_fm801_capture_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    let mut ptr: size_t;
    if ((*chip).cap_ctrl & FM801_START) == 0 {
        return 0;
    }
    ptr = ((*chip).cap_pos + ((*chip).cap_count - 1) - fm801_readw(chip, FM801_CAP_COUNT) as c_uint) as size_t;
    if (fm801_readw(chip, FM801_IRQ_STATUS) & FM801_IRQ_CAPTURE) != 0 {
        ptr += (*chip).cap_count as usize;
        ptr %= (*chip).cap_size as usize;
    }
    bytes_to_frames((*substream).runtime, ptr)
}

unsafe extern "C" fn snd_fm801_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chip = dev_id as *mut fm801;
    let mut status = fm801_readw(chip, FM801_IRQ_STATUS);
    let mut tmp: c_uint;
    status &= FM801_IRQ_PLAYBACK | FM801_IRQ_CAPTURE | FM801_IRQ_MPU | FM801_IRQ_VOLUME;
    if status == 0 {
        return IRQ_NONE;
    }
    /* ack first */
    fm801_writew(chip, FM801_IRQ_STATUS, status);
    if !(*chip).pcm.is_null() && (status & FM801_IRQ_PLAYBACK) != 0 && !(*chip).playback_substream.is_null() {
        (*chip).ply_buf += 1;
        (*chip).ply_pos += (*chip).ply_count;
        (*chip).ply_pos %= (*chip).ply_size;
        tmp = (*chip).ply_pos + (*chip).ply_count;
        tmp %= (*chip).ply_size;
        if ((*chip).ply_buf & 1) != 0 {
            fm801_writel(chip, FM801_PLY_BUF1, (*chip).ply_buffer + tmp as c_ulong);
        } else {
            fm801_writel(chip, FM801_PLY_BUF2, (*chip).ply_buffer + tmp as c_ulong);
        }
        snd_pcm_period_elapsed((*chip).playback_substream);
    }
    if !(*chip).pcm.is_null() && (status & FM801_IRQ_CAPTURE) != 0 && !(*chip).capture_substream.is_null() {
        (*chip).cap_buf += 1;
        (*chip).cap_pos += (*chip).cap_count;
        (*chip).cap_pos %= (*chip).cap_size;
        tmp = (*chip).cap_pos + (*chip).cap_count;
        tmp %= (*chip).cap_size;
        if ((*chip).cap_buf & 1) != 0 {
            fm801_writel(chip, FM801_CAP_BUF1, (*chip).cap_buffer + tmp as c_ulong);
        } else {
            fm801_writel(chip, FM801_CAP_BUF2, (*chip).cap_buffer + tmp as c_ulong);
        }
        snd_pcm_period_elapsed((*chip).capture_substream);
    }
    if !(*chip).rmidi.is_null() && (status & FM801_IRQ_MPU) != 0 {
        snd_mpu401_uart_interrupt(irq, (*(*chip).rmidi).private_data);
    }
    if (status & FM801_IRQ_VOLUME) != 0 {
        /* TODO */
    }
    IRQ_HANDLED
}

static snd_fm801_playback: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_RESUME | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_KNOT | SNDRV_PCM_RATE_8000_48000,
    rate_min: 5500,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 128 * 1024,
    period_bytes_min: 64,
    period_bytes_max: 128 * 1024,
    periods_min: 1,
    periods_max: 1024,
    fifo_size: 0,
};

static snd_fm801_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_RESUME | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_KNOT | SNDRV_PCM_RATE_8000_48000,
    rate_min: 5500,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 128 * 1024,
    period_bytes_min: 64,
    period_bytes_max: 128 * 1024,
    periods_min: 1,
    periods_max: 1024,
    fifo_size: 0,
};

unsafe extern "C" fn snd_fm801_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let mut err: c_int;
    (*chip).playback_substream = substream;
    (*runtime).hw = snd_fm801_playback;
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &hw_constraints_rates);
    if (*chip).multichannel != 0 {
        (*runtime).hw.channels_max = 6;
        snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, &hw_constraints_channels);
    }
    err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if err < 0 {
        return err;
    }
    0
}

extern "C" {
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_int) -> c_int;
}

unsafe extern "C" fn snd_fm801_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let mut err: c_int;
    (*chip).capture_substream = substream;
    (*runtime).hw = snd_fm801_capture;
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &hw_constraints_rates);
    err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if err < 0 {
        return err;
    }
    0
}

unsafe extern "C" fn snd_fm801_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    (*chip).playback_substream = null_mut();
    0
}

unsafe extern "C" fn snd_fm801_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    (*chip).capture_substream = null_mut();
    0
}

static snd_fm801_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_fm801_playback_open),
    close: Some(snd_fm801_playback_close),
    ioctl: null(),
    hw_params: null(),
    hw_free: null(),
    prepare: Some(snd_fm801_playback_prepare),
    trigger: Some(snd_fm801_playback_trigger),
    pointer: Some(snd_fm801_playback_pointer),
};

static snd_fm801_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_fm801_capture_open),
    close: Some(snd_fm801_capture_close),
    ioctl: null(),
    hw_params: null(),
    hw_free: null(),
    prepare: Some(snd_fm801_capture_prepare),
    trigger: Some(snd_fm801_capture_trigger),
    pointer: Some(snd_fm801_capture_pointer),
};

unsafe fn to_pci_dev(dev: *mut device) -> *mut pci_dev {
    dev as *mut pci_dev
}

unsafe fn snd_fm801_pcm(chip: *mut fm801, device: c_int) -> c_int {
    let pdev = to_pci_dev((*chip).dev);
    let mut pcm: *mut snd_pcm = null_mut();
    let mut err: c_int;
    err = snd_pcm_new((*chip).card, b"FM801\0".as_ptr() as *const c_char, device, 1, 1, &mut pcm);
    if err < 0 {
        return err;
    }
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_fm801_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_fm801_capture_ops);
    (*pcm).private_data = chip as *mut c_void;
    (*pcm).info_flags = 0;
    strscpy((*pcm).name.as_mut_ptr(), b"FM801\0".as_ptr() as *const c_char, (*pcm).name.len());
    (*chip).pcm = pcm;
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &mut (*pdev).dev, if (*chip).multichannel != 0 { 128 * 1024 } else { 64 * 1024 }, 128 * 1024);
    snd_pcm_add_chmap_ctls(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_pcm_alt_chmaps as *const _ as *const c_void, if (*chip).multichannel != 0 { 6 } else { 2 }, 0, null_mut())
}

/*
 *  TEA5757 radio
 */

/* CONFIG_SND_FM801_TEA575X_BOOL */

/* GPIO to TEA575x maps */
#[repr(C)]
#[derive(Copy, Clone)]
struct snd_fm801_tea575x_gpio {
    data: u8,
    clk: u8,
    wren: u8,
    most: u8,
    name: *const c_char,
}

static snd_fm801_tea575x_gpios: [snd_fm801_tea575x_gpio; 3] = [
    snd_fm801_tea575x_gpio { data: 1, clk: 3, wren: 2, most: 0, name: b"SF256-PCS\0".as_ptr() as *const c_char },
    snd_fm801_tea575x_gpio { data: 1, clk: 0, wren: 2, most: 3, name: b"SF256-PCP\0".as_ptr() as *const c_char },
    snd_fm801_tea575x_gpio { data: 2, clk: 0, wren: 1, most: 3, name: b"SF64-PCR\0".as_ptr() as *const c_char },
];

unsafe fn get_tea575x_gpio(chip: *mut fm801) -> *const snd_fm801_tea575x_gpio {
    &snd_fm801_tea575x_gpios[(((*chip).tea575x_tuner & TUNER_TYPE_MASK) - 1) as usize]
}

unsafe extern "C" fn snd_fm801_tea575x_set_pins(tea: *mut snd_tea575x, pins: u8) {
    let chip = (*tea).private_data;
    let mut reg = fm801_readw(chip, FM801_GPIO_CTRL);
    let gpio = *get_tea575x_gpio(chip);
    reg &= !(FM801_GPIO_GP(gpio.data) | FM801_GPIO_GP(gpio.clk) | FM801_GPIO_GP(gpio.wren));
    reg |= if (pins & TEA575X_DATA) != 0 { FM801_GPIO_GP(gpio.data) } else { 0 };
    reg |= if (pins & TEA575X_CLK) != 0 { FM801_GPIO_GP(gpio.clk) } else { 0 };
    /* WRITE_ENABLE is inverted */
    reg |= if (pins & TEA575X_WREN) != 0 { 0 } else { FM801_GPIO_GP(gpio.wren) };
    fm801_writew(chip, FM801_GPIO_CTRL, reg);
}

unsafe extern "C" fn snd_fm801_tea575x_get_pins(tea: *mut snd_tea575x) -> u8 {
    let chip = (*tea).private_data;
    let reg = fm801_readw(chip, FM801_GPIO_CTRL);
    let gpio = *get_tea575x_gpio(chip);
    let mut ret: u8 = 0;
    if (reg & FM801_GPIO_GP(gpio.data)) != 0 {
        ret |= TEA575X_DATA;
    }
    if (reg & FM801_GPIO_GP(gpio.most)) != 0 {
        ret |= TEA575X_MOST;
    }
    ret
}

unsafe extern "C" fn snd_fm801_tea575x_set_direction(tea: *mut snd_tea575x, output: bool_) {
    let chip = (*tea).private_data;
    let mut reg = fm801_readw(chip, FM801_GPIO_CTRL);
    let gpio = *get_tea575x_gpio(chip);
    /* use GPIO lines and set write enable bit */
    reg |= FM801_GPIO_GS(gpio.data) | FM801_GPIO_GS(gpio.wren) | FM801_GPIO_GS(gpio.clk) | FM801_GPIO_GS(gpio.most);
    if output {
        /* all of lines are in the write direction */
        /* clear data and clock lines */
        reg &= !(FM801_GPIO_GD(gpio.data) | FM801_GPIO_GD(gpio.wren) | FM801_GPIO_GD(gpio.clk) | FM801_GPIO_GP(gpio.data) | FM801_GPIO_GP(gpio.clk) | FM801_GPIO_GP(gpio.wren));
    } else {
        /* use GPIO lines, set data direction to input */
        reg |= FM801_GPIO_GD(gpio.data) | FM801_GPIO_GD(gpio.most) | FM801_GPIO_GP(gpio.data) | FM801_GPIO_GP(gpio.most) | FM801_GPIO_GP(gpio.wren);
        /* all of lines are in the write direction, except data */
        /* clear data, write enable and clock lines */
        reg &= !(FM801_GPIO_GD(gpio.wren) | FM801_GPIO_GD(gpio.clk) | FM801_GPIO_GP(gpio.clk));
    }
    fm801_writew(chip, FM801_GPIO_CTRL, reg);
}

static snd_fm801_tea_ops: snd_tea575x_ops = snd_tea575x_ops {
    set_pins: Some(snd_fm801_tea575x_set_pins),
    get_pins: Some(snd_fm801_tea575x_get_pins),
    set_direction: Some(snd_fm801_tea575x_set_direction),
};

/*
 *  Mixer routines
 */

const fn FM801_SINGLE_VALUE(reg: c_uint, shift: c_uint, mask: c_uint, invert: c_uint) -> c_ulong {
    (reg | (shift << 8) | (mask << 16) | (invert << 24)) as c_ulong
}

const fn FM801_DOUBLE_VALUE(reg: c_uint, shift_left: c_uint, shift_right: c_uint, mask: c_uint, invert: c_uint) -> c_ulong {
    (reg | (shift_left << 8) | (shift_right << 12) | (mask << 16) | (invert << 24)) as c_ulong
}

unsafe extern "C" fn snd_fm801_info_single(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let mask = ((*kcontrol).private_value >> 16) & 0xff;
    (*uinfo).type_ = if mask == 1 { SNDRV_CTL_ELEM_TYPE_BOOLEAN } else { SNDRV_CTL_ELEM_TYPE_INTEGER };
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = mask as c_long;
    0
}

unsafe extern "C" fn snd_fm801_get_single(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let reg = ((*kcontrol).private_value & 0xff) as c_ushort;
    let shift = ((*kcontrol).private_value >> 8) & 0xff;
    let mask = ((*kcontrol).private_value >> 16) & 0xff;
    let invert = ((*kcontrol).private_value >> 24) & 0xff;
    let value = (*ucontrol).value.integer.value.as_mut_ptr();
    *value.add(0) = ((fm801_ioread16(chip, reg) as c_ulong >> shift) & mask) as c_long;
    if invert != 0 {
        *value.add(0) = mask as c_long - *value.add(0);
    }
    0
}

unsafe extern "C" fn snd_fm801_put_single(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let reg = ((*kcontrol).private_value & 0xff) as c_ushort;
    let shift = ((*kcontrol).private_value >> 8) & 0xff;
    let mask = ((*kcontrol).private_value >> 16) & 0xff;
    let invert = ((*kcontrol).private_value >> 24) & 0xff;
    let mut val = ((*ucontrol).value.integer.value[0] as c_ulong & mask) as c_ushort;
    if invert != 0 {
        val = mask as c_ushort - val;
    }
    snd_fm801_update_bits(chip, reg, (mask << shift) as c_ushort, (val as c_ulong << shift) as c_ushort)
}

unsafe extern "C" fn snd_fm801_info_double(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let mask = ((*kcontrol).private_value >> 16) & 0xff;
    (*uinfo).type_ = if mask == 1 { SNDRV_CTL_ELEM_TYPE_BOOLEAN } else { SNDRV_CTL_ELEM_TYPE_INTEGER };
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = mask as c_long;
    0
}

unsafe extern "C" fn snd_fm801_get_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let reg = ((*kcontrol).private_value & 0xff) as c_ushort;
    let shift_left = ((*kcontrol).private_value >> 8) & 0x0f;
    let shift_right = ((*kcontrol).private_value >> 12) & 0x0f;
    let mask = ((*kcontrol).private_value >> 16) & 0xff;
    let invert = ((*kcontrol).private_value >> 24) & 0xff;
    let value = (*ucontrol).value.integer.value.as_mut_ptr();
    *value.add(0) = ((fm801_ioread16(chip, reg) as c_ulong >> shift_left) & mask) as c_long;
    *value.add(1) = ((fm801_ioread16(chip, reg) as c_ulong >> shift_right) & mask) as c_long;
    if invert != 0 {
        *value.add(0) = mask as c_long - *value.add(0);
        *value.add(1) = mask as c_long - *value.add(1);
    }
    0
}

unsafe extern "C" fn snd_fm801_put_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let reg = ((*kcontrol).private_value & 0xff) as c_ushort;
    let shift_left = ((*kcontrol).private_value >> 8) & 0x0f;
    let shift_right = ((*kcontrol).private_value >> 12) & 0x0f;
    let mask = ((*kcontrol).private_value >> 16) & 0xff;
    let invert = ((*kcontrol).private_value >> 24) & 0xff;
    let mut val1 = ((*ucontrol).value.integer.value[0] as c_ulong & mask) as c_ushort;
    let mut val2 = ((*ucontrol).value.integer.value[1] as c_ulong & mask) as c_ushort;
    if invert != 0 {
        val1 = mask as c_ushort - val1;
        val2 = mask as c_ushort - val2;
    }
    snd_fm801_update_bits(chip, reg, ((mask << shift_left) | (mask << shift_right)) as c_ushort, ((val1 as c_ulong << shift_left) | (val2 as c_ulong << shift_right)) as c_ushort)
}

unsafe extern "C" fn snd_fm801_info_mux(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    static texts: [*const c_char; 5] = [
        b"AC97 Primary\0".as_ptr() as *const c_char,
        b"FM\0".as_ptr() as *const c_char,
        b"I2S\0".as_ptr() as *const c_char,
        b"PCM\0".as_ptr() as *const c_char,
        b"AC97 Secondary\0".as_ptr() as *const c_char,
    ];
    snd_ctl_enum_info(uinfo, 1, 5, texts.as_ptr())
}

unsafe extern "C" fn snd_fm801_get_mux(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let mut val = fm801_readw(chip, FM801_REC_SRC) & 7;
    if val > 4 {
        val = 4;
    }
    (*ucontrol).value.enumerated.item[0] = val as c_uint;
    0
}

unsafe extern "C" fn snd_fm801_put_mux(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let val = (*ucontrol).value.enumerated.item[0] as c_ushort;
    if val > 4 {
        return -EINVAL;
    }
    snd_fm801_update_bits(chip, FM801_REC_SRC, 7, val)
}

static db_scale_dsp: [c_uint; 4] = [0, (-3450i32) as c_uint, 150, 0];

static snd_fm801_controls: [snd_kcontrol_new; 7] = [
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Wave Playback Volume\0".as_ptr() as *const c_char, access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, info: Some(snd_fm801_info_double), get: Some(snd_fm801_get_double), put: Some(snd_fm801_put_double), private_value: FM801_DOUBLE_VALUE(FM801_PCM_VOL as c_uint, 0, 8, 31, 1), tlv: snd_kcontrol_tlv { p: db_scale_dsp.as_ptr() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Wave Playback Switch\0".as_ptr() as *const c_char, access: 0, info: Some(snd_fm801_info_single), get: Some(snd_fm801_get_single), put: Some(snd_fm801_put_single), private_value: FM801_SINGLE_VALUE(FM801_PCM_VOL as c_uint, 15, 1, 1), tlv: snd_kcontrol_tlv { p: null() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"I2S Playback Volume\0".as_ptr() as *const c_char, access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, info: Some(snd_fm801_info_double), get: Some(snd_fm801_get_double), put: Some(snd_fm801_put_double), private_value: FM801_DOUBLE_VALUE(FM801_I2S_VOL as c_uint, 0, 8, 31, 1), tlv: snd_kcontrol_tlv { p: db_scale_dsp.as_ptr() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"I2S Playback Switch\0".as_ptr() as *const c_char, access: 0, info: Some(snd_fm801_info_single), get: Some(snd_fm801_get_single), put: Some(snd_fm801_put_single), private_value: FM801_SINGLE_VALUE(FM801_I2S_VOL as c_uint, 15, 1, 1), tlv: snd_kcontrol_tlv { p: null() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"FM Playback Volume\0".as_ptr() as *const c_char, access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, info: Some(snd_fm801_info_double), get: Some(snd_fm801_get_double), put: Some(snd_fm801_put_double), private_value: FM801_DOUBLE_VALUE(FM801_FM_VOL as c_uint, 0, 8, 31, 1), tlv: snd_kcontrol_tlv { p: db_scale_dsp.as_ptr() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"FM Playback Switch\0".as_ptr() as *const c_char, access: 0, info: Some(snd_fm801_info_single), get: Some(snd_fm801_get_single), put: Some(snd_fm801_put_single), private_value: FM801_SINGLE_VALUE(FM801_FM_VOL as c_uint, 15, 1, 1), tlv: snd_kcontrol_tlv { p: null() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Digital Capture Source\0".as_ptr() as *const c_char, access: 0, info: Some(snd_fm801_info_mux), get: Some(snd_fm801_get_mux), put: Some(snd_fm801_put_mux), private_value: 0, tlv: snd_kcontrol_tlv { p: null() } },
];
const FM801_CONTROLS: usize = 7;

static snd_fm801_controls_multi: [snd_kcontrol_new; 6] = [
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"AC97 2ch->4ch Copy Switch\0".as_ptr() as *const c_char, access: 0, info: Some(snd_fm801_info_single), get: Some(snd_fm801_get_single), put: Some(snd_fm801_put_single), private_value: FM801_SINGLE_VALUE(FM801_CODEC_CTRL as c_uint, 7, 1, 0), tlv: snd_kcontrol_tlv { p: null() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"AC97 18-bit Switch\0".as_ptr() as *const c_char, access: 0, info: Some(snd_fm801_info_single), get: Some(snd_fm801_get_single), put: Some(snd_fm801_put_single), private_value: FM801_SINGLE_VALUE(FM801_CODEC_CTRL as c_uint, 10, 1, 0), tlv: snd_kcontrol_tlv { p: null() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"IEC958 Capture Switch\0".as_ptr() as *const c_char, access: 0, info: Some(snd_fm801_info_single), get: Some(snd_fm801_get_single), put: Some(snd_fm801_put_single), private_value: FM801_SINGLE_VALUE(FM801_I2S_MODE as c_uint, 8, 1, 0), tlv: snd_kcontrol_tlv { p: null() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"IEC958 Raw Data Playback Switch\0".as_ptr() as *const c_char, access: 0, info: Some(snd_fm801_info_single), get: Some(snd_fm801_get_single), put: Some(snd_fm801_put_single), private_value: FM801_SINGLE_VALUE(FM801_I2S_MODE as c_uint, 9, 1, 0), tlv: snd_kcontrol_tlv { p: null() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"IEC958 Raw Data Capture Switch\0".as_ptr() as *const c_char, access: 0, info: Some(snd_fm801_info_single), get: Some(snd_fm801_get_single), put: Some(snd_fm801_put_single), private_value: FM801_SINGLE_VALUE(FM801_I2S_MODE as c_uint, 10, 1, 0), tlv: snd_kcontrol_tlv { p: null() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"IEC958 Playback Switch\0".as_ptr() as *const c_char, access: 0, info: Some(snd_fm801_info_single), get: Some(snd_fm801_get_single), put: Some(snd_fm801_put_single), private_value: FM801_SINGLE_VALUE(FM801_GEN_CTRL as c_uint, 2, 1, 0), tlv: snd_kcontrol_tlv { p: null() } },
];
const FM801_CONTROLS_MULTI: usize = 6;

unsafe fn snd_fm801_mixer(chip: *mut fm801) -> c_int {
    let mut ac97: snd_ac97_template = zeroed();
    let mut i: usize;
    let mut err: c_int;
    static ops: snd_ac97_bus_ops = snd_ac97_bus_ops {
        write: Some(snd_fm801_codec_write),
        read: Some(snd_fm801_codec_read),
    };
    err = snd_ac97_bus((*chip).card, 0, &ops, chip, &mut (*chip).ac97_bus);
    if err < 0 {
        return err;
    }
    ac97.private_data = chip;
    err = snd_ac97_mixer((*chip).ac97_bus, &mut ac97, &mut (*chip).ac97);
    if err < 0 {
        return err;
    }
    if (*chip).secondary != 0 {
        ac97.num = 1;
        ac97.addr = (*chip).secondary_addr as c_uint;
        err = snd_ac97_mixer((*chip).ac97_bus, &mut ac97, &mut (*chip).ac97_sec);
        if err < 0 {
            return err;
        }
    }
    i = 0;
    while i < FM801_CONTROLS {
        err = snd_ctl_add((*chip).card, snd_ctl_new1(&snd_fm801_controls[i], chip));
        if err < 0 {
            return err;
        }
        i += 1;
    }
    if (*chip).multichannel != 0 {
        i = 0;
        while i < FM801_CONTROLS_MULTI {
            err = snd_ctl_add((*chip).card, snd_ctl_new1(&snd_fm801_controls_multi[i], chip));
            if err < 0 {
                return err;
            }
            i += 1;
        }
    }
    0
}

/*
 *  initialization routines
 */

unsafe fn wait_for_codec(chip: *mut fm801, codec_id: c_uint, reg: c_ushort, waits: c_ulong) -> c_int {
    let timeout = jiffies + waits;
    fm801_writew(chip, FM801_AC97_CMD, reg | ((codec_id as c_ushort) << FM801_AC97_ADDR_SHIFT) | FM801_AC97_READ);
    udelay(5);
    loop {
        if (fm801_readw(chip, FM801_AC97_CMD) & (FM801_AC97_VALID | FM801_AC97_BUSY)) == FM801_AC97_VALID {
            return 0;
        }
        schedule_timeout_uninterruptible(1);
        if !time_after(timeout, jiffies) {
            break;
        }
    }
    -EIO
}

unsafe fn reset_codec(chip: *mut fm801) -> c_int {
    /* codec cold reset + AC'97 warm reset */
    fm801_writew(chip, FM801_CODEC_CTRL, (1 << 5) | (1 << 6));
    fm801_readw(chip, FM801_CODEC_CTRL); /* flush posting data */
    udelay(100);
    fm801_writew(chip, FM801_CODEC_CTRL, 0);
    wait_for_codec(chip, 0, AC97_RESET, msecs_to_jiffies(750))
}

unsafe fn snd_fm801_chip_multichannel_init(chip: *mut fm801) {
    let mut cmdw: c_ushort;
    if (*chip).multichannel != 0 {
        if (*chip).secondary_addr != 0 {
            wait_for_codec(chip, (*chip).secondary_addr as c_uint, AC97_VENDOR_ID1, msecs_to_jiffies(50));
        } else {
            /* my card has the secondary codec */
            /* at address #3, so the loop is inverted */
            let mut i: c_int = 3;
            while i > 0 {
                if wait_for_codec(chip, i as c_uint, AC97_VENDOR_ID1, msecs_to_jiffies(50)) == 0 {
                    cmdw = fm801_readw(chip, FM801_AC97_DATA);
                    if cmdw != 0xffff && cmdw != 0 {
                        (*chip).secondary = 1;
                        (*chip).secondary_addr = i as u8;
                        break;
                    }
                }
                i -= 1;
            }
        }
        /* the recovery phase, it seems that probing for non-existing codec might */
        /* cause timeout problems */
        wait_for_codec(chip, 0, AC97_VENDOR_ID1, msecs_to_jiffies(750));
    }
}

unsafe fn snd_fm801_chip_init(chip: *mut fm801) {
    let mut cmdw: c_ushort;
    /* init volume */
    fm801_writew(chip, FM801_PCM_VOL, 0x0808);
    fm801_writew(chip, FM801_FM_VOL, 0x9f1f);
    fm801_writew(chip, FM801_I2S_VOL, 0x8808);
    /* I2S control - I2S mode */
    fm801_writew(chip, FM801_I2S_MODE, 0x0003);
    /* interrupt setup */
    cmdw = fm801_readw(chip, FM801_IRQ_MASK);
    if (*chip).irq < 0 {
        cmdw |= 0x00c3; /* mask everything, no PCM nor MPU */
    } else {
        cmdw &= !0x0083; /* unmask MPU, PLAYBACK & CAPTURE */
    }
    fm801_writew(chip, FM801_IRQ_MASK, cmdw);
    /* interrupt clear */
    fm801_writew(chip, FM801_IRQ_STATUS, FM801_IRQ_PLAYBACK | FM801_IRQ_CAPTURE | FM801_IRQ_MPU);
}

unsafe extern "C" fn snd_fm801_free(card: *mut snd_card) {
    let chip = (*card).private_data as *mut fm801;
    let mut cmdw: c_ushort;
    /* interrupt setup - mask everything */
    cmdw = fm801_readw(chip, FM801_IRQ_MASK);
    cmdw |= 0x00c3;
    fm801_writew(chip, FM801_IRQ_MASK, cmdw);
    /* CONFIG_SND_FM801_TEA575X_BOOL */
    if ((*chip).tea575x_tuner & TUNER_DISABLED) == 0 {
        snd_tea575x_exit(&mut (*chip).tea);
        v4l2_device_unregister(&mut (*chip).v4l2_dev);
    }
}

unsafe fn snd_fm801_create(card: *mut snd_card, pci: *mut pci_dev, mut tea575x_tuner_arg: c_int, radio_nr_arg: c_int) -> c_int {
    let chip = (*card).private_data as *mut fm801;
    let mut err: c_int;
    err = pcim_enable_device(pci);
    if err < 0 {
        return err;
    }
    spin_lock_init(&mut (*chip).reg_lock);
    (*chip).card = card;
    (*chip).dev = &mut (*pci).dev;
    (*chip).irq = -1;
    (*chip).tea575x_tuner = tea575x_tuner_arg as c_uint;
    err = pcim_request_all_regions(pci, b"FM801\0".as_ptr() as *const c_char);
    if err < 0 {
        return err;
    }
    (*chip).port = pci_resource_start(pci, 0);
    if (*pci).revision >= 0xb1 {
        /* FM801-AU */
        (*chip).multichannel = 1;
    }
    if ((*chip).tea575x_tuner & TUNER_ONLY) == 0 {
        if reset_codec(chip) < 0 {
            dev_info((*(*chip).card).dev, b"Primary AC'97 codec not found, assume SF64-PCR (tuner-only)\n\0".as_ptr() as *const c_char);
            (*chip).tea575x_tuner = 3 | TUNER_ONLY;
        } else {
            snd_fm801_chip_multichannel_init(chip);
        }
    }
    if ((*chip).tea575x_tuner & TUNER_ONLY) == 0 {
        if devm_request_irq(&mut (*pci).dev, (*pci).irq, snd_fm801_interrupt, IRQF_SHARED, KBUILD_MODNAME.as_ptr() as *const c_char, chip) != 0 {
            dev_err((*card).dev, b"unable to grab IRQ %d\n\0".as_ptr() as *const c_char, (*pci).irq);
            return -EBUSY;
        }
        (*chip).irq = (*pci).irq;
        (*card).sync_irq = (*chip).irq;
        pci_set_master(pci);
    }
    (*card).private_free = Some(snd_fm801_free);
    snd_fm801_chip_init(chip);
    /* CONFIG_SND_FM801_TEA575X_BOOL */
    err = v4l2_device_register(&mut (*pci).dev, &mut (*chip).v4l2_dev);
    if err < 0 {
        return err;
    }
    (*chip).tea.v4l2_dev = &mut (*chip).v4l2_dev;
    (*chip).tea.radio_nr = radio_nr_arg;
    (*chip).tea.private_data = chip;
    (*chip).tea.ops = &snd_fm801_tea_ops;
    sprintf((*chip).tea.bus_info.as_mut_ptr(), b"PCI:%s\0".as_ptr() as *const c_char, pci_name(pci));
    if ((*chip).tea575x_tuner & TUNER_TYPE_MASK) > 0 && ((*chip).tea575x_tuner & TUNER_TYPE_MASK) < 4 {
        if snd_tea575x_init(&mut (*chip).tea, THIS_MODULE) != 0 {
            dev_err((*card).dev, b"TEA575x radio not found\n\0".as_ptr() as *const c_char);
            return -ENODEV;
        }
    } else if ((*chip).tea575x_tuner & TUNER_TYPE_MASK) == 0 {
        let tuner_only = (*chip).tea575x_tuner & TUNER_ONLY;
        /* autodetect tuner connection */
        tea575x_tuner_arg = 1;
        while tea575x_tuner_arg <= 3 {
            (*chip).tea575x_tuner = tea575x_tuner_arg as c_uint;
            if snd_tea575x_init(&mut (*chip).tea, THIS_MODULE) == 0 {
                dev_info((*card).dev, b"detected TEA575x radio type %s\n\0".as_ptr() as *const c_char, (*get_tea575x_gpio(chip)).name);
                break;
            }
            tea575x_tuner_arg += 1;
        }
        if tea575x_tuner_arg == 4 {
            dev_err((*card).dev, b"TEA575x radio not found\n\0".as_ptr() as *const c_char);
            (*chip).tea575x_tuner = TUNER_DISABLED;
        }
        (*chip).tea575x_tuner |= tuner_only;
    }
    if ((*chip).tea575x_tuner & TUNER_DISABLED) == 0 {
        strscpy((*chip).tea.card.as_mut_ptr(), (*get_tea575x_gpio(chip)).name, (*chip).tea.card.len());
    }
    0
}

unsafe fn __snd_card_fm801_probe(pci: *mut pci_dev, _pci_id: *const pci_device_id) -> c_int {
    static mut dev: c_int = 0;
    let mut card: *mut snd_card = null_mut();
    let mut chip: *mut fm801;
    let mut opl3: *mut snd_opl3 = null_mut();
    let mut err: c_int;
    if dev >= SNDRV_CARDS as c_int {
        return -ENODEV;
    }
    if !enable[dev as usize] {
        dev += 1;
        return -ENOENT;
    }
    err = snd_devm_card_new(&mut (*pci).dev, index[dev as usize], id[dev as usize], THIS_MODULE, core::mem::size_of::<fm801>(), &mut card);
    if err < 0 {
        return err;
    }
    chip = (*card).private_data as *mut fm801;
    err = snd_fm801_create(card, pci, tea575x_tuner[dev as usize], radio_nr[dev as usize]);
    if err < 0 {
        return err;
    }
    strscpy((*card).driver.as_mut_ptr(), b"FM801\0".as_ptr() as *const c_char, (*card).driver.len());
    strscpy((*card).shortname.as_mut_ptr(), b"ForteMedia FM801-\0".as_ptr() as *const c_char, (*card).shortname.len());
    strcat((*card).shortname.as_mut_ptr(), if (*chip).multichannel != 0 { b"AU\0".as_ptr() as *const c_char } else { b"AS\0".as_ptr() as *const c_char });
    sprintf((*card).longname.as_mut_ptr(), b"%s at 0x%lx, irq %i\0".as_ptr() as *const c_char, (*card).shortname.as_ptr(), (*chip).port, (*chip).irq);
    if ((*chip).tea575x_tuner & TUNER_ONLY) != 0 {
        goto_fm801_tuner_only(card, pci, &mut dev)
    } else {
        err = snd_fm801_pcm(chip, 0);
        if err < 0 {
            return err;
        }
        err = snd_fm801_mixer(chip);
        if err < 0 {
            return err;
        }
        err = snd_mpu401_uart_new(card, 0, MPU401_HW_FM801, (*chip).port + FM801_MPU401_DATA as c_ulong, MPU401_INFO_INTEGRATED | MPU401_INFO_IRQ_HOOK, -1, &mut (*chip).rmidi);
        if err < 0 {
            return err;
        }
        err = snd_opl3_create(card, (*chip).port + FM801_OPL3_BANK0 as c_ulong, (*chip).port + FM801_OPL3_BANK1 as c_ulong, OPL3_HW_OPL3_FM801, 1, &mut opl3);
        if err < 0 {
            return err;
        }
        err = snd_opl3_hwdep_new(opl3, 0, 1, null_mut());
        if err < 0 {
            return err;
        }
        goto_fm801_tuner_only(card, pci, &mut dev)
    }
}

unsafe fn goto_fm801_tuner_only(card: *mut snd_card, pci: *mut pci_dev, devp: *mut c_int) -> c_int {
    let mut err = snd_card_register(card);
    if err < 0 {
        return err;
    }
    pci_set_drvdata(pci, card);
    *devp += 1;
    0
}

unsafe extern "C" fn snd_card_fm801_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    snd_card_free_on_error(&mut (*pci).dev, __snd_card_fm801_probe(pci, pci_id))
}

static saved_regs: [u8; 16] = [
    FM801_PCM_VOL as u8, FM801_I2S_VOL as u8, FM801_FM_VOL as u8, FM801_REC_SRC as u8,
    FM801_PLY_CTRL as u8, FM801_PLY_COUNT as u8, FM801_PLY_BUF1 as u8, FM801_PLY_BUF2 as u8,
    FM801_CAP_CTRL as u8, FM801_CAP_COUNT as u8, FM801_CAP_BUF1 as u8, FM801_CAP_BUF2 as u8,
    FM801_CODEC_CTRL as u8, FM801_I2S_MODE as u8, FM801_VOLUME as u8, FM801_GEN_CTRL as u8,
];

unsafe extern "C" fn snd_fm801_suspend(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev);
    let chip = (*card).private_data as *mut fm801;
    let mut i: usize;
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    i = 0;
    while i < saved_regs.len() {
        (*chip).saved_regs[i] = fm801_ioread16(chip, saved_regs[i] as c_ushort);
        i += 1;
    }
    if ((*chip).tea575x_tuner & TUNER_ONLY) != 0 {
        /* FIXME: tea575x suspend */
    } else {
        snd_ac97_suspend((*chip).ac97);
        snd_ac97_suspend((*chip).ac97_sec);
    }
    0
}

unsafe extern "C" fn snd_fm801_resume(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev);
    let chip = (*card).private_data as *mut fm801;
    let mut i: usize;
    if ((*chip).tea575x_tuner & TUNER_ONLY) != 0 {
        snd_fm801_chip_init(chip);
    } else {
        reset_codec(chip);
        snd_fm801_chip_multichannel_init(chip);
        snd_fm801_chip_init(chip);
        snd_ac97_resume((*chip).ac97);
        snd_ac97_resume((*chip).ac97_sec);
    }
    i = 0;
    while i < saved_regs.len() {
        fm801_iowrite16(chip, saved_regs[i] as c_ushort, (*chip).saved_regs[i]);
        i += 1;
    }
    /* CONFIG_SND_FM801_TEA575X_BOOL */
    if ((*chip).tea575x_tuner & TUNER_DISABLED) == 0 {
        snd_tea575x_set_freq(&mut (*chip).tea);
    }
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}

static snd_fm801_pm: dev_pm_ops = dev_pm_ops { _private: [] };

static mut fm801_driver: pci_driver = pci_driver {
    name: KBUILD_MODNAME.as_ptr() as *const c_char,
    id_table: snd_fm801_ids.as_ptr(),
    probe: Some(snd_card_fm801_probe),
    driver: pci_driver_driver {
        pm: &snd_fm801_pm,
    },
};

// module_pci_driver(fm801_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
