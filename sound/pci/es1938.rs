// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Driver for ESS Solo-1 (ES1938, ES1946, ES1969) soundcard
 *  Copyright (c) by Jaromir Koutek <miri@punknet.cz>,
 *                   Jaroslav Kysela <perex@perex.cz>,
 *                   Thomas Sailer <sailer@ife.ee.ethz.ch>,
 *                   Abramo Bagnara <abramo@alsa-project.org>,
 *                   Markus Gruber <gruber@eikon.tum.de>
 *
 * Rewritten from sonicvibes.c source.
 *
 *  TODO:
 *    Rewrite better spinlocks
 */

/*
  NOTES:
  - Capture data is written unaligned starting from dma_base + 1 so I need to
    disable mmap and to add a copy callback.
  - After several cycle of the following:
    while : ; do arecord -d1 -f cd -t raw | aplay -f cd ; done
    a "playback write error (DMA or IRQ trouble?)" may happen.
    This is due to playback interrupts not generated.
    I suspect a timing issue.
  - Sometimes the interrupt handler is invoked wrongly during playback.
    This generates some harmless "Unexpected hw_pointer: wrong interrupt
    acknowledge".
    I've seen that using small period sizes.
    Reproducible with:
    mpg123 test.mp3 &
    hdparm -t -T /dev/hda
*/

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type bool_ = bool;
type size_t = usize;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_uint;
type spinlock_t = c_uint;

const SNDRV_CARDS: usize = 32;
const SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [core::ptr::null_mut(); SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE_PNP: [bool_; SNDRV_CARDS] = [true; SNDRV_CARDS];

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR; /* ID for this card */
static mut enable: [bool_; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP; /* Enable this card */

const SL_PCI_LEGACYCONTROL: c_int = 0x40;
const SL_PCI_CONFIG: c_int = 0x50;
const SL_PCI_DDMACONTROL: c_int = 0x60;

const ESSIO_REG_AUDIO2DMAADDR: c_ulong = 0;
const ESSIO_REG_AUDIO2DMACOUNT: c_ulong = 4;
const ESSIO_REG_AUDIO2MODE: c_ulong = 6;
const ESSIO_REG_IRQCONTROL: c_ulong = 7;

const ESSDM_REG_DMAADDR: c_ulong = 0x00;
const ESSDM_REG_DMACOUNT: c_ulong = 0x04;
const ESSDM_REG_DMACOMMAND: c_ulong = 0x08;
const ESSDM_REG_DMASTATUS: c_ulong = 0x08;
const ESSDM_REG_DMAMODE: c_ulong = 0x0b;
const ESSDM_REG_DMACLEAR: c_ulong = 0x0d;
const ESSDM_REG_DMAMASK: c_ulong = 0x0f;

const ESSSB_REG_FMLOWADDR: c_ulong = 0x00;
const ESSSB_REG_FMHIGHADDR: c_ulong = 0x02;
const ESSSB_REG_MIXERADDR: c_ulong = 0x04;
const ESSSB_REG_MIXERDATA: c_ulong = 0x05;
const ESSSB_REG_RESET: c_ulong = 0x06;
const ESSSB_REG_READDATA: c_ulong = 0x0a;
const ESSSB_REG_WRITEDATA: c_ulong = 0x0c;
const ESSSB_REG_READSTATUS: c_ulong = 0x0c;
const ESSSB_REG_STATUS: c_ulong = 0x0e;

const ESSSB_IREG_AUDIO1: u8 = 0x14;
const ESSSB_IREG_MICMIX: u8 = 0x1a;
const ESSSB_IREG_RECSRC: u8 = 0x1c;
const ESSSB_IREG_MASTER: u8 = 0x32;
const ESSSB_IREG_FM: u8 = 0x36;
const ESSSB_IREG_AUXACD: u8 = 0x38;
const ESSSB_IREG_AUXB: u8 = 0x3a;
const ESSSB_IREG_PCSPEAKER: u8 = 0x3c;
const ESSSB_IREG_LINE: u8 = 0x3e;
const ESSSB_IREG_SPATCONTROL: u8 = 0x50;
const ESSSB_IREG_SPATLEVEL: u8 = 0x52;
const ESSSB_IREG_MASTER_LEFT: u8 = 0x60;
const ESSSB_IREG_MASTER_RIGHT: u8 = 0x62;
const ESSSB_IREG_MPU401CONTROL: u8 = 0x64;
const ESSSB_IREG_MICMIXRECORD: u8 = 0x68;
const ESSSB_IREG_AUDIO2RECORD: u8 = 0x69;
const ESSSB_IREG_AUXACDRECORD: u8 = 0x6a;
const ESSSB_IREG_FMRECORD: u8 = 0x6b;
const ESSSB_IREG_AUXBRECORD: u8 = 0x6c;
const ESSSB_IREG_MONO: u8 = 0x6d;
const ESSSB_IREG_LINERECORD: u8 = 0x6e;
const ESSSB_IREG_MONORECORD: u8 = 0x6f;
const ESSSB_IREG_AUDIO2SAMPLE: u8 = 0x70;
const ESSSB_IREG_AUDIO2MODE: u8 = 0x71;
const ESSSB_IREG_AUDIO2FILTER: u8 = 0x72;
const ESSSB_IREG_AUDIO2TCOUNTL: u8 = 0x74;
const ESSSB_IREG_AUDIO2TCOUNTH: u8 = 0x76;
const ESSSB_IREG_AUDIO2CONTROL1: u8 = 0x78;
const ESSSB_IREG_AUDIO2CONTROL2: u8 = 0x7a;
const ESSSB_IREG_AUDIO2: u8 = 0x7c;

const ESS_CMD_EXTSAMPLERATE: u8 = 0xa1;
const ESS_CMD_FILTERDIV: u8 = 0xa2;
const ESS_CMD_DMACNTRELOADL: u8 = 0xa4;
const ESS_CMD_DMACNTRELOADH: u8 = 0xa5;
const ESS_CMD_ANALOGCONTROL: u8 = 0xa8;
const ESS_CMD_IRQCONTROL: u8 = 0xb1;
const ESS_CMD_DRQCONTROL: u8 = 0xb2;
const ESS_CMD_RECLEVEL: u8 = 0xb4;
const ESS_CMD_SETFORMAT: u8 = 0xb6;
const ESS_CMD_SETFORMAT2: u8 = 0xb7;
const ESS_CMD_DMACONTROL: u8 = 0xb8;
const ESS_CMD_DMATYPE: u8 = 0xb9;
const ESS_CMD_OFFSETLEFT: u8 = 0xba;
const ESS_CMD_OFFSETRIGHT: u8 = 0xbb;
const ESS_CMD_READREG: u8 = 0xc0;
const ESS_CMD_ENABLEEXT: u8 = 0xc6;
const ESS_CMD_PAUSEDMA: u8 = 0xd0;
const ESS_CMD_ENABLEAUDIO1: u8 = 0xd1;
const ESS_CMD_STOPAUDIO1: u8 = 0xd3;
const ESS_CMD_AUDIO1STATUS: u8 = 0xd8;
const ESS_CMD_CONTDMA: u8 = 0xd4;
const ESS_CMD_TESTIRQ: u8 = 0xf2;

const ESS_RECSRC_MIC: c_int = 0;
const ESS_RECSRC_AUXACD: c_int = 2;
const ESS_RECSRC_AUXB: c_int = 5;
const ESS_RECSRC_LINE: c_int = 6;
const ESS_RECSRC_NONE: c_int = 7;

const DAC1: c_uint = 0x01;
const ADC1: c_uint = 0x02;
const DAC2: c_uint = 0x04;
const SAVED_REG_SIZE: usize = 32; /* max. number of registers to save */

#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct pci_dev { pub dev: device, pub irq: c_int }
#[repr(C)]
pub struct pci_device_id { pub vendor: c_uint, pub device: c_uint, pub subvendor: c_uint, pub subdevice: c_uint, pub class: c_uint, pub class_mask: c_uint, pub driver_data: c_ulong }
#[repr(C)]
pub struct snd_info_entry { _private: [u8; 0] }
#[repr(C)]
pub struct snd_pcm { pub private_data: *mut c_void, pub info_flags: c_uint, pub name: [c_char; 80] }
#[repr(C)]
pub struct snd_rawmidi { pub private_data: *mut c_void }
#[repr(C)]
pub struct snd_card { pub dev: *mut device, pub private_data: *mut c_void, pub sync_irq: c_int, pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>, pub mixername: [c_char; 80], pub driver: [c_char; 16], pub shortname: [c_char; 32], pub longname: [c_char; 80] }
#[repr(C)]
pub struct snd_kcontrol_id { _private: [u8; 0] }
#[repr(C)]
pub union snd_kcontrol_tlv { pub p: *const c_uint }
#[repr(C)]
pub struct snd_kcontrol { pub private_value: c_ulong, pub private_free: Option<unsafe extern "C" fn(*mut snd_kcontrol)>, pub id: snd_kcontrol_id }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ratnum { pub num: c_uint, pub den_min: c_uint, pub den_max: c_uint, pub den_step: c_uint }
#[repr(C)]
pub struct snd_pcm_hw_constraint_ratnums { pub nrats: c_uint, pub rats: *const snd_ratnum }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hardware { pub info: c_uint, pub formats: c_uint, pub rates: c_uint, pub rate_min: c_uint, pub rate_max: c_uint, pub channels_min: c_uint, pub channels_max: c_uint, pub buffer_bytes_max: c_uint, pub period_bytes_min: c_uint, pub period_bytes_max: c_uint, pub periods_min: c_uint, pub periods_max: c_uint, pub fifo_size: c_uint }
#[repr(C)]
pub struct snd_pcm_runtime { pub hw: snd_pcm_hardware, pub rate_num: c_uint, pub rate_den: c_uint, pub rate: c_uint, pub channels: c_uint, pub format: c_int, pub dma_addr: c_uint, pub dma_area: *mut u8 }
#[repr(C)]
pub struct snd_pcm_substream { pub runtime: *mut snd_pcm_runtime, pub number: c_int }
#[repr(C)]
pub struct iov_iter { _private: [u8; 0] }
#[repr(C)]
pub struct snd_ctl_elem_info_integer { pub min: c_long, pub max: c_long }
type c_long = isize;
#[repr(C)]
pub union snd_ctl_elem_info_value { pub integer: snd_ctl_elem_info_integer }
#[repr(C)]
pub struct snd_ctl_elem_info { pub type_: c_uint, pub count: c_uint, pub value: snd_ctl_elem_info_value }
#[repr(C)]
pub struct snd_ctl_elem_value_enumerated { pub item: [c_uint; 4] }
#[repr(C)]
pub struct snd_ctl_elem_value_integer { pub value: [c_long; 128] }
#[repr(C)]
pub union snd_ctl_elem_value_value { pub enumerated: snd_ctl_elem_value_enumerated, pub integer: snd_ctl_elem_value_integer }
#[repr(C)]
pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }
#[repr(C)]
pub struct snd_kcontrol_new { pub iface: c_uint, pub access: c_uint, pub name: *const c_char, pub index: c_uint, pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>, pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, pub private_value: c_ulong, pub tlv: snd_kcontrol_tlv }
#[repr(C)]
pub struct snd_pcm_ops { pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>, pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>, pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>, pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>, pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>, pub copy: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, c_ulong, *mut iov_iter, c_ulong) -> c_int> }
#[repr(C)]
pub struct gameport { pub io: c_ulong }
#[repr(C)]
pub struct snd_opl3 { _private: [u8; 0] }
#[repr(C)]
pub struct dev_pm_ops { _private: [u8; 0] }
#[repr(C)]
pub struct pci_driver_inner { pub pm: *const dev_pm_ops }
#[repr(C)]
pub struct pci_driver { pub name: *const c_char, pub id_table: *const pci_device_id, pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>, pub driver: pci_driver_inner }

#[repr(C)]
pub struct es1938 {
    pub irq: c_int,
    pub io_port: c_ulong,
    pub sb_port: c_ulong,
    pub vc_port: c_ulong,
    pub mpu_port: c_ulong,
    pub game_port: c_ulong,
    pub ddma_port: c_ulong,
    pub irqmask: u8,
    pub revision: u8,
    pub hw_volume: *mut snd_kcontrol,
    pub hw_switch: *mut snd_kcontrol,
    pub master_volume: *mut snd_kcontrol,
    pub master_switch: *mut snd_kcontrol,
    pub pci: *mut pci_dev,
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub capture_substream: *mut snd_pcm_substream,
    pub playback1_substream: *mut snd_pcm_substream,
    pub playback2_substream: *mut snd_pcm_substream,
    pub rmidi: *mut snd_rawmidi,
    pub dma1_size: c_uint,
    pub dma2_size: c_uint,
    pub dma1_start: c_uint,
    pub dma2_start: c_uint,
    pub dma1_shift: c_uint,
    pub dma2_shift: c_uint,
    pub last_capture_dmaaddr: c_uint,
    pub active: c_uint,
    pub reg_lock: spinlock_t,
    pub mixer_lock: spinlock_t,
    pub proc_entry: *mut snd_info_entry,
    /* SUPPORT_JOYSTICK */
    pub gameport: *mut gameport,
    pub saved_regs: [u8; SAVED_REG_SIZE],
}

extern "C" {
    fn outb(value: u8, port: c_ulong);
    fn outw(value: c_uint, port: c_ulong);
    fn outl(value: c_uint, port: c_ulong);
    fn inb(port: c_ulong) -> u8;
    fn inw(port: c_ulong) -> c_uint;
    fn inl(port: c_ulong) -> c_uint;
    fn udelay(usecs: c_uint);
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut es1938;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn snd_pcm_format_unsigned(format: c_int) -> c_int;
    fn copy_to_iter(addr: *const u8, bytes: c_ulong, dst: *mut iov_iter) -> c_ulong;
    fn snd_pcm_hw_constraint_ratnums(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_uint, rats: *const snd_pcm_hw_constraint_ratnums) -> c_int;
    fn snd_pcm_hw_constraint_minmax(runtime: *mut snd_pcm_runtime, var: c_uint, min: c_uint, max: c_uint) -> c_int;
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback: c_int, capture: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, typ: c_int, dev: *mut device, min: size_t, max: size_t);
    fn snd_ctl_enum_info(uinfo: *mut snd_ctl_elem_info, channels: c_uint, items: c_uint, texts: *const *const c_char) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut es1938;
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_ctl_boolean_stereo_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn dev_get_drvdata(dev: *mut device) -> *mut snd_card;
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
    fn request_irq(irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, dev: *mut c_void) -> c_int;
    fn free_irq(irq: c_int, dev: *mut c_void);
    fn snd_card_disconnect(card: *mut snd_card);
    fn gameport_allocate_port() -> *mut gameport;
    fn gameport_set_name(gp: *mut gameport, name: *const c_char);
    fn gameport_set_phys(gp: *mut gameport, fmt: *const c_char, name: *const c_char);
    fn gameport_set_dev_parent(gp: *mut gameport, dev: *mut device);
    fn gameport_register_port(gp: *mut gameport);
    fn gameport_unregister_port(gp: *mut gameport);
    fn pci_name(pci: *mut pci_dev) -> *const c_char;
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn pcim_request_all_regions(pci: *mut pci_dev, name: *const c_char) -> c_int;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn pci_resource_flags(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn pci_set_master(pci: *mut pci_dev);
    fn pci_write_config_word(pci: *mut pci_dev, where_: c_int, val: c_uint) -> c_int;
    fn pci_write_config_dword(pci: *mut pci_dev, where_: c_int, val: c_uint) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_kcontrol_id);
    fn snd_mpu401_uart_interrupt(irq: c_int, private_data: *mut c_void) -> irqreturn_t;
    fn snd_ctl_new1(template: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_devm_card_new(dev: *mut device, idx: c_int, id: *mut c_char, module: *mut c_void, extra_size: size_t, card_ret: *mut *mut snd_card) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> size_t;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snd_opl3_create(card: *mut snd_card, l: c_ulong, r: c_ulong, hw: c_int, integrated: c_int, opl3: *mut *mut snd_opl3) -> c_int;
    fn snd_opl3_timer_new(opl3: *mut snd_opl3, device: c_int, seq_device: c_int) -> c_int;
    fn snd_opl3_hwdep_new(opl3: *mut snd_opl3, device: c_int, seq_device: c_int, rhwdep: *mut c_void) -> c_int;
    fn snd_mpu401_uart_new(card: *mut snd_card, device: c_int, hardware: c_uint, port: c_ulong, info: c_uint, irq: c_int, rrawmidi: *mut *mut snd_rawmidi) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn snd_card_free_on_error(dev: *mut device, probe: c_int) -> c_int;
    fn IRQ_RETVAL(handled: c_int) -> irqreturn_t;
    fn snd_BUG();
    fn snd_BUG_ON(cond: bool_) -> bool_;
}

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const EFAULT: c_int = 14;
const EAGAIN: c_int = 11;
const ENOMEM: c_int = 12;
const ENOSYS: c_int = 38;
const EIO: c_int = 5;
const ENXIO: c_int = 6;
const EBUSY: c_int = 16;
const ENOENT: c_int = 2;
const IRQF_SHARED: c_ulong = 0x80;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 2;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 3;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 1;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint = 1 << 2;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 3;
const SNDRV_PCM_FMTBIT_U8: c_uint = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << 1;
const SNDRV_PCM_FMTBIT_S8: c_uint = 1 << 2;
const SNDRV_PCM_FMTBIT_U16_LE: c_uint = 1 << 3;
const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 1 << 30;
const SNDRV_PCM_RATE_8000_48000: c_uint = 1 << 6;
const SNDRV_PCM_HW_PARAM_RATE: c_uint = 0;
const SNDRV_PCM_HW_PARAM_BUFFER_BYTES: c_uint = 1;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_ELEM_ACCESS_READ: c_uint = 1 << 0;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 3;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 1 << 2;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 1;
const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_uint = 2;
const SNDRV_CTL_EVENT_MASK_VALUE: c_uint = 1;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;
const IORESOURCE_IO: c_ulong = 0x00000100;
const OPL3_HW_OPL3: c_int = 0;
const MPU401_HW_MPU401: c_uint = 0;
const MPU401_INFO_INTEGRATED: c_uint = 1;
const MPU401_INFO_IRQ_HOOK: c_uint = 2;
const THIS_MODULE: *mut c_void = core::ptr::null_mut();
const KBUILD_MODNAME: *const c_char = b"es1938\0".as_ptr() as *const c_char;

const fn DMA_BIT_MASK(n: u32) -> u64 { (1u64 << n) - 1 }
const fn SLIO_REG(chip: *mut es1938, x: c_ulong) -> c_ulong { unsafe { (*chip).io_port + x } }
const fn SLDM_REG(chip: *mut es1938, x: c_ulong) -> c_ulong { unsafe { (*chip).ddma_port + x } }
const fn SLSB_REG(chip: *mut es1938, x: c_ulong) -> c_ulong { unsafe { (*chip).sb_port + x } }

static snd_es1938_ids: [pci_device_id; 2] = [
    pci_device_id { vendor: 0x125d, device: 0x1969, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 }, /* Solo-1 */
    pci_device_id { vendor: 0, device: 0, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
];

const RESET_LOOP_TIMEOUT: c_int = 0x10000;
const WRITE_LOOP_TIMEOUT: c_int = 0x10000;
const GET_LOOP_TIMEOUT: c_int = 0x01000;

unsafe extern "C" fn snd_es1938_mixer_write(chip: *mut es1938, reg: u8, val: u8) {
    outb(reg, SLSB_REG(chip, ESSSB_REG_MIXERADDR));
    outb(val, SLSB_REG(chip, ESSSB_REG_MIXERDATA));
}

unsafe extern "C" fn snd_es1938_mixer_read(chip: *mut es1938, reg: u8) -> c_int {
    outb(reg, SLSB_REG(chip, ESSSB_REG_MIXERADDR));
    inb(SLSB_REG(chip, ESSSB_REG_MIXERDATA)) as c_int
}

unsafe extern "C" fn snd_es1938_mixer_bits(chip: *mut es1938, reg: u8, mask: u8, val: u8) -> c_int {
    outb(reg, SLSB_REG(chip, ESSSB_REG_MIXERADDR));
    let old = inb(SLSB_REG(chip, ESSSB_REG_MIXERDATA));
    let oval = old & mask;
    if val != oval {
        let new = (old & !mask) | (val & mask);
        outb(new, SLSB_REG(chip, ESSSB_REG_MIXERDATA));
    }
    oval as c_int
}

unsafe extern "C" fn snd_es1938_write_cmd(chip: *mut es1938, cmd: u8) {
    let mut v: u8 = 0;
    for _i in 0..WRITE_LOOP_TIMEOUT {
        v = inb(SLSB_REG(chip, ESSSB_REG_READSTATUS));
        if (v & 0x80) == 0 {
            outb(cmd, SLSB_REG(chip, ESSSB_REG_WRITEDATA));
            return;
        }
    }
    let _ = v;
}

unsafe extern "C" fn snd_es1938_get_byte(chip: *mut es1938) -> c_int {
    let mut v: u8 = 0;
    let mut i = GET_LOOP_TIMEOUT;
    while i != 0 {
        v = inb(SLSB_REG(chip, ESSSB_REG_STATUS));
        if (v & 0x80) != 0 {
            return inb(SLSB_REG(chip, ESSSB_REG_READDATA)) as c_int;
        }
        i -= 1;
    }
    let _ = v;
    -ENODEV
}

unsafe extern "C" fn snd_es1938_write(chip: *mut es1938, reg: u8, val: u8) {
    snd_es1938_write_cmd(chip, reg);
    snd_es1938_write_cmd(chip, val);
}

unsafe extern "C" fn snd_es1938_read(chip: *mut es1938, reg: u8) -> u8 {
    snd_es1938_write_cmd(chip, ESS_CMD_READREG);
    snd_es1938_write_cmd(chip, reg);
    snd_es1938_get_byte(chip) as u8
}

unsafe extern "C" fn snd_es1938_bits(chip: *mut es1938, reg: u8, mask: u8, val: u8) -> c_int {
    snd_es1938_write_cmd(chip, ESS_CMD_READREG);
    snd_es1938_write_cmd(chip, reg);
    let old = snd_es1938_get_byte(chip) as u8;
    let oval = old & mask;
    if val != oval {
        snd_es1938_write_cmd(chip, reg);
        let new = (old & !mask) | (val & mask);
        snd_es1938_write_cmd(chip, new);
    }
    oval as c_int
}

unsafe extern "C" fn snd_es1938_reset(chip: *mut es1938) {
    outb(3, SLSB_REG(chip, ESSSB_REG_RESET));
    inb(SLSB_REG(chip, ESSSB_REG_RESET));
    outb(0, SLSB_REG(chip, ESSSB_REG_RESET));
    for _i in 0..RESET_LOOP_TIMEOUT {
        if (inb(SLSB_REG(chip, ESSSB_REG_STATUS)) & 0x80) != 0 {
            if inb(SLSB_REG(chip, ESSSB_REG_READDATA)) == 0xaa {
                break;
            }
        }
    }
    snd_es1938_write_cmd(chip, ESS_CMD_ENABLEEXT);
    snd_es1938_write(chip, ESS_CMD_DMATYPE, 2);
    snd_es1938_mixer_write(chip, ESSSB_IREG_AUDIO2MODE, 0x32);
    snd_es1938_bits(chip, ESS_CMD_IRQCONTROL, 0xf0, 0x50);
    snd_es1938_bits(chip, ESS_CMD_DRQCONTROL, 0xf0, 0x50);
    snd_es1938_write_cmd(chip, ESS_CMD_ENABLEAUDIO1);
    snd_es1938_mixer_write(chip, 0x54, 0x8f);
    snd_es1938_mixer_write(chip, 0x56, 0x95);
    snd_es1938_mixer_write(chip, 0x58, 0x94);
    snd_es1938_mixer_write(chip, 0x5a, 0x80);
}

unsafe extern "C" fn snd_es1938_reset_fifo(chip: *mut es1938) {
    outb(2, SLSB_REG(chip, ESSSB_REG_RESET));
    outb(0, SLSB_REG(chip, ESSSB_REG_RESET));
}

static clocks: [snd_ratnum; 2] = [
    snd_ratnum { num: 793800, den_min: 1, den_max: 128, den_step: 1 },
    snd_ratnum { num: 768000, den_min: 1, den_max: 128, den_step: 1 },
];

static hw_constraints_clocks: snd_pcm_hw_constraint_ratnums =
    snd_pcm_hw_constraint_ratnums { nrats: 2, rats: clocks.as_ptr() };

unsafe extern "C" fn snd_es1938_rate_set(chip: *mut es1938, substream: *mut snd_pcm_substream, mode: c_int) {
    let runtime = (*substream).runtime;
    let bits = if (*runtime).rate_num == clocks[0].num { 128 - (*runtime).rate_den } else { 256 - (*runtime).rate_den };
    let div0 = 256 - 7160000 * 20 / (8 * 82 * (*runtime).rate);
    if mode == DAC2 as c_int {
        snd_es1938_mixer_write(chip, 0x70, bits as u8);
        snd_es1938_mixer_write(chip, 0x72, div0 as u8);
    } else {
        snd_es1938_write(chip, 0xA1, bits as u8);
        snd_es1938_write(chip, 0xA2, div0 as u8);
    }
}

unsafe extern "C" fn snd_es1938_playback1_setdma(chip: *mut es1938) {
    outb(0x00, SLIO_REG(chip, ESSIO_REG_AUDIO2MODE));
    outl((*chip).dma2_start, SLIO_REG(chip, ESSIO_REG_AUDIO2DMAADDR));
    outw(0, SLIO_REG(chip, ESSIO_REG_AUDIO2DMACOUNT));
    outw((*chip).dma2_size, SLIO_REG(chip, ESSIO_REG_AUDIO2DMACOUNT));
}

unsafe extern "C" fn snd_es1938_playback2_setdma(chip: *mut es1938) {
    outb(0xc4, SLDM_REG(chip, ESSDM_REG_DMACOMMAND));
    outb(0, SLDM_REG(chip, ESSDM_REG_DMACLEAR));
    outb(1, SLDM_REG(chip, ESSDM_REG_DMAMASK));
    outb(0x18, SLDM_REG(chip, ESSDM_REG_DMAMODE));
    outl((*chip).dma1_start, SLDM_REG(chip, ESSDM_REG_DMAADDR));
    outw((*chip).dma1_size - 1, SLDM_REG(chip, ESSDM_REG_DMACOUNT));
    outb(0, SLDM_REG(chip, ESSDM_REG_DMAMASK));
}

unsafe extern "C" fn snd_es1938_capture_setdma(chip: *mut es1938) {
    outb(0xc4, SLDM_REG(chip, ESSDM_REG_DMACOMMAND));
    outb(0, SLDM_REG(chip, ESSDM_REG_DMACLEAR));
    outb(1, SLDM_REG(chip, ESSDM_REG_DMAMASK));
    outb(0x14, SLDM_REG(chip, ESSDM_REG_DMAMODE));
    outl((*chip).dma1_start, SLDM_REG(chip, ESSDM_REG_DMAADDR));
    (*chip).last_capture_dmaaddr = (*chip).dma1_start;
    outw((*chip).dma1_size - 1, SLDM_REG(chip, ESSDM_REG_DMACOUNT));
    outb(0, SLDM_REG(chip, ESSDM_REG_DMAMASK));
}

unsafe extern "C" fn snd_es1938_capture_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let val: c_int;
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => { val = 0x0f; (*chip).active |= ADC1; }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => { val = 0x00; (*chip).active &= !ADC1; }
        _ => return -EINVAL,
    }
    snd_es1938_write(chip, ESS_CMD_DMACONTROL, val as u8);
    0
}

unsafe extern "C" fn snd_es1938_playback1_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            /* According to the documentation this should be:
               0x13 but that value may randomly swap stereo channels */
            snd_es1938_mixer_write(chip, ESSSB_IREG_AUDIO2CONTROL1, 0x92);
            udelay(10);
            snd_es1938_mixer_write(chip, ESSSB_IREG_AUDIO2CONTROL1, 0x93);
            /* This two stage init gives the FIFO -> DAC connection time to
             * settle before first data from DMA flows in.  This should ensure
             * no swapping of stereo channels.  Report a bug if otherwise :-) */
            outb(0x0a, SLIO_REG(chip, ESSIO_REG_AUDIO2MODE));
            (*chip).active |= DAC2;
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            outb(0, SLIO_REG(chip, ESSIO_REG_AUDIO2MODE));
            snd_es1938_mixer_write(chip, ESSSB_IREG_AUDIO2CONTROL1, 0);
            (*chip).active &= !DAC2;
        }
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn snd_es1938_playback2_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let val: c_int;
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => { val = 5; (*chip).active |= DAC1; }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => { val = 0; (*chip).active &= !DAC1; }
        _ => return -EINVAL,
    }
    snd_es1938_write(chip, ESS_CMD_DMACONTROL, val as u8);
    0
}

unsafe extern "C" fn snd_es1938_playback_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    match (*substream).number {
        0 => snd_es1938_playback1_trigger(substream, cmd),
        1 => snd_es1938_playback2_trigger(substream, cmd),
        _ => { snd_BUG(); -EINVAL }
    }
}

unsafe extern "C" fn snd_es1938_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let size = snd_pcm_lib_buffer_bytes(substream);
    let mut count = snd_pcm_lib_period_bytes(substream);
    (*chip).dma1_size = size;
    (*chip).dma1_start = (*runtime).dma_addr;
    let mono = if (*runtime).channels > 1 { 0 } else { 1 };
    let is8 = if snd_pcm_format_width((*runtime).format) == 16 { 0 } else { 1 };
    let u = snd_pcm_format_unsigned((*runtime).format);
    (*chip).dma1_shift = (2 - mono - is8) as c_uint;
    snd_es1938_reset_fifo(chip);
    snd_es1938_bits(chip, ESS_CMD_ANALOGCONTROL, 0x03, if mono != 0 { 2 } else { 1 });
    snd_es1938_rate_set(chip, substream, ADC1 as c_int);
    count = 0x10000 - count;
    snd_es1938_write(chip, ESS_CMD_DMACNTRELOADL, (count & 0xff) as u8);
    snd_es1938_write(chip, ESS_CMD_DMACNTRELOADH, (count >> 8) as u8);
    snd_es1938_write(chip, ESS_CMD_SETFORMAT2, if u != 0 { 0x51 } else { 0x71 });
    snd_es1938_write(chip, ESS_CMD_SETFORMAT2, (0x90 | if u != 0 { 0x00 } else { 0x20 } | if is8 != 0 { 0x00 } else { 0x04 } | if mono != 0 { 0x40 } else { 0x08 }) as u8);
    snd_es1938_capture_setdma(chip);
    0
}

unsafe extern "C" fn snd_es1938_playback1_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let size = snd_pcm_lib_buffer_bytes(substream);
    let mut count = snd_pcm_lib_period_bytes(substream);
    (*chip).dma2_size = size;
    (*chip).dma2_start = (*runtime).dma_addr;
    let mono = if (*runtime).channels > 1 { 0 } else { 1 };
    let is8 = if snd_pcm_format_width((*runtime).format) == 16 { 0 } else { 1 };
    let u = snd_pcm_format_unsigned((*runtime).format);
    (*chip).dma2_shift = (2 - mono - is8) as c_uint;
    snd_es1938_reset_fifo(chip);
    snd_es1938_rate_set(chip, substream, DAC2 as c_int);
    count >>= 1;
    count = 0x10000 - count;
    snd_es1938_mixer_write(chip, ESSSB_IREG_AUDIO2TCOUNTL, (count & 0xff) as u8);
    snd_es1938_mixer_write(chip, ESSSB_IREG_AUDIO2TCOUNTH, (count >> 8) as u8);
    snd_es1938_mixer_write(chip, ESSSB_IREG_AUDIO2CONTROL2, (0x40 | if u != 0 { 0 } else { 4 } | if mono != 0 { 0 } else { 2 } | if is8 != 0 { 0 } else { 1 }) as u8);
    snd_es1938_playback1_setdma(chip);
    0
}

unsafe extern "C" fn snd_es1938_playback2_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let size = snd_pcm_lib_buffer_bytes(substream);
    let mut count = snd_pcm_lib_period_bytes(substream);
    (*chip).dma1_size = size;
    (*chip).dma1_start = (*runtime).dma_addr;
    let mono = if (*runtime).channels > 1 { 0 } else { 1 };
    let is8 = if snd_pcm_format_width((*runtime).format) == 16 { 0 } else { 1 };
    let u = snd_pcm_format_unsigned((*runtime).format);
    (*chip).dma1_shift = (2 - mono - is8) as c_uint;
    count = 0x10000 - count;
    snd_es1938_reset_fifo(chip);
    snd_es1938_bits(chip, ESS_CMD_ANALOGCONTROL, 0x03, if mono != 0 { 2 } else { 1 });
    snd_es1938_rate_set(chip, substream, DAC1 as c_int);
    snd_es1938_write(chip, ESS_CMD_DMACNTRELOADL, (count & 0xff) as u8);
    snd_es1938_write(chip, ESS_CMD_DMACNTRELOADH, (count >> 8) as u8);
    snd_es1938_write(chip, ESS_CMD_SETFORMAT, if u != 0 { 0x80 } else { 0x00 });
    snd_es1938_write(chip, ESS_CMD_SETFORMAT, if u != 0 { 0x51 } else { 0x71 });
    snd_es1938_write(chip, ESS_CMD_SETFORMAT2, (0x90 | if mono != 0 { 0x40 } else { 0x08 } | if is8 != 0 { 0x00 } else { 0x04 } | if u != 0 { 0x00 } else { 0x20 }) as u8);
    snd_es1938_playback2_setdma(chip);
    0
}

unsafe extern "C" fn snd_es1938_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    match (*substream).number {
        0 => snd_es1938_playback1_prepare(substream),
        1 => snd_es1938_playback2_prepare(substream),
        _ => { snd_BUG(); -EINVAL }
    }
}

unsafe extern "C" fn snd_es1938_capture_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    let mut ptr = inl(SLDM_REG(chip, ESSDM_REG_DMAADDR)) as size_t;
    let count = inw(SLDM_REG(chip, ESSDM_REG_DMACOUNT)) as size_t;
    let diff = ((*chip).dma1_start as size_t + (*chip).dma1_size as size_t).wrapping_sub(ptr).wrapping_sub(count) as c_uint;
    if diff > 3 || ptr < (*chip).dma1_start as size_t || ptr >= ((*chip).dma1_start + (*chip).dma1_size) as size_t {
        ptr = (*chip).last_capture_dmaaddr as size_t;
    } else {
        (*chip).last_capture_dmaaddr = ptr as c_uint;
    }
    ptr = ptr.wrapping_sub((*chip).dma1_start as size_t);
    (ptr >> (*chip).dma1_shift) as snd_pcm_uframes_t
}

unsafe extern "C" fn snd_es1938_playback1_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    let ptr = (*chip).dma2_size as size_t - inw(SLIO_REG(chip, ESSIO_REG_AUDIO2DMACOUNT)) as size_t;
    (ptr >> (*chip).dma2_shift) as snd_pcm_uframes_t
}

unsafe extern "C" fn snd_es1938_playback2_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    let mut old = inw(SLDM_REG(chip, ESSDM_REG_DMACOUNT)) as size_t;
    let mut new = inw(SLDM_REG(chip, ESSDM_REG_DMACOUNT)) as size_t;
    while new != old {
        old = new;
        new = inw(SLDM_REG(chip, ESSDM_REG_DMACOUNT)) as size_t;
    }
    let ptr = (*chip).dma1_size as size_t - 1 - new;
    (ptr >> (*chip).dma1_shift) as snd_pcm_uframes_t
}

unsafe extern "C" fn snd_es1938_playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    match (*substream).number {
        0 => snd_es1938_playback1_pointer(substream),
        1 => snd_es1938_playback2_pointer(substream),
        _ => { snd_BUG(); (-EINVAL) as snd_pcm_uframes_t }
    }
}

unsafe extern "C" fn snd_es1938_capture_copy(substream: *mut snd_pcm_substream, _channel: c_int, pos: c_ulong, dst: *mut iov_iter, count: c_ulong) -> c_int {
    let runtime = (*substream).runtime;
    let chip = snd_pcm_substream_chip(substream);
    if snd_BUG_ON(pos + count > (*chip).dma1_size as c_ulong) { return -EINVAL; }
    if pos + count < (*chip).dma1_size as c_ulong {
        if copy_to_iter((*runtime).dma_area.add(pos as usize + 1), count, dst) != count { return -EFAULT; }
    } else {
        if copy_to_iter((*runtime).dma_area.add(pos as usize + 1), count - 1, dst) != count - 1 { return -EFAULT; }
        if copy_to_iter((*runtime).dma_area, 1, dst) != 1 { return -EFAULT; }
    }
    0
}

static snd_es1938_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER,
    formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_U16_LE,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    rate_min: 6000, rate_max: 48000, channels_min: 1, channels_max: 2,
    buffer_bytes_max: 0x8000, period_bytes_min: 64, period_bytes_max: 0x8000,
    periods_min: 1, periods_max: 1024, fifo_size: 256,
};

static snd_es1938_playback: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_U16_LE,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    rate_min: 6000, rate_max: 48000, channels_min: 1, channels_max: 2,
    buffer_bytes_max: 0x8000, period_bytes_min: 64, period_bytes_max: 0x8000,
    periods_min: 1, periods_max: 1024, fifo_size: 256,
};

unsafe extern "C" fn snd_es1938_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    if !(*chip).playback2_substream.is_null() { return -EAGAIN; }
    (*chip).capture_substream = substream;
    (*runtime).hw = snd_es1938_capture;
    snd_pcm_hw_constraint_ratnums(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &hw_constraints_clocks);
    snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_BUFFER_BYTES, 0, 0xff00);
    0
}

unsafe extern "C" fn snd_es1938_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    match (*substream).number {
        0 => (*chip).playback1_substream = substream,
        1 => { if !(*chip).capture_substream.is_null() { return -EAGAIN; } (*chip).playback2_substream = substream; }
        _ => { snd_BUG(); return -EINVAL; }
    }
    (*runtime).hw = snd_es1938_playback;
    snd_pcm_hw_constraint_ratnums(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &hw_constraints_clocks);
    snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_BUFFER_BYTES, 0, 0xff00);
    0
}

unsafe extern "C" fn snd_es1938_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    (*chip).capture_substream = core::ptr::null_mut();
    0
}

unsafe extern "C" fn snd_es1938_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    match (*substream).number {
        0 => (*chip).playback1_substream = core::ptr::null_mut(),
        1 => (*chip).playback2_substream = core::ptr::null_mut(),
        _ => { snd_BUG(); return -EINVAL; }
    }
    0
}

static snd_es1938_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_es1938_playback_open), close: Some(snd_es1938_playback_close),
    prepare: Some(snd_es1938_playback_prepare), trigger: Some(snd_es1938_playback_trigger),
    pointer: Some(snd_es1938_playback_pointer), copy: None,
};

static snd_es1938_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_es1938_capture_open), close: Some(snd_es1938_capture_close),
    prepare: Some(snd_es1938_capture_prepare), trigger: Some(snd_es1938_capture_trigger),
    pointer: Some(snd_es1938_capture_pointer), copy: Some(snd_es1938_capture_copy),
};

unsafe extern "C" fn snd_es1938_new_pcm(chip: *mut es1938, device: c_int) -> c_int {
    let mut pcm: *mut snd_pcm = core::ptr::null_mut();
    let mut err = snd_pcm_new((*chip).card, b"es-1938-1946\0".as_ptr() as *const c_char, device, 2, 1, &mut pcm);
    if err < 0 { return err; }
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_es1938_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_es1938_capture_ops);
    (*pcm).private_data = chip as *mut c_void;
    (*pcm).info_flags = 0;
    strscpy((*pcm).name.as_mut_ptr(), b"ESS Solo-1\0".as_ptr() as *const c_char);
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &mut (*(*chip).pci).dev, 64 * 1024, 64 * 1024);
    (*chip).pcm = pcm;
    err
}

unsafe extern "C" fn snd_es1938_info_mux(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    static TEXTS: [*const c_char; 8] = [
        b"Mic\0".as_ptr() as *const c_char, b"Mic Master\0".as_ptr() as *const c_char,
        b"CD\0".as_ptr() as *const c_char, b"AOUT\0".as_ptr() as *const c_char,
        b"Mic1\0".as_ptr() as *const c_char, b"Mix\0".as_ptr() as *const c_char,
        b"Line\0".as_ptr() as *const c_char, b"Master\0".as_ptr() as *const c_char,
    ];
    snd_ctl_enum_info(uinfo, 1, 8, TEXTS.as_ptr())
}

unsafe extern "C" fn snd_es1938_get_mux(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    (*ucontrol).value.enumerated.item[0] = (snd_es1938_mixer_read(chip, 0x1c) & 0x07) as c_uint;
    0
}

unsafe extern "C" fn snd_es1938_put_mux(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let val = (*ucontrol).value.enumerated.item[0] as u8;
    if val > 7 { return -EINVAL; }
    (snd_es1938_mixer_bits(chip, 0x1c, 0x07, val) != val as c_int) as c_int
}

unsafe extern "C" fn snd_es1938_get_spatializer_enable(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let val = snd_es1938_mixer_read(chip, 0x50) as u8;
    (*ucontrol).value.integer.value[0] = ((val & 8) != 0) as c_long;
    0
}

unsafe extern "C" fn snd_es1938_put_spatializer_enable(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let nval: u8 = if (*ucontrol).value.integer.value[0] != 0 { 0x0c } else { 0x04 };
    let oval = (snd_es1938_mixer_read(chip, 0x50) as u8) & 0x0c;
    let change = nval != oval;
    if change {
        snd_es1938_mixer_write(chip, 0x50, nval & !0x04);
        snd_es1938_mixer_write(chip, 0x50, nval);
    }
    change as c_int
}

unsafe extern "C" fn snd_es1938_info_hw_volume(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 63;
    0
}

unsafe extern "C" fn snd_es1938_get_hw_volume(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    (*ucontrol).value.integer.value[0] = (snd_es1938_mixer_read(chip, 0x61) & 0x3f) as c_long;
    (*ucontrol).value.integer.value[1] = (snd_es1938_mixer_read(chip, 0x63) & 0x3f) as c_long;
    0
}

unsafe extern "C" fn snd_es1938_get_hw_switch(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    (*ucontrol).value.integer.value[0] = ((snd_es1938_mixer_read(chip, 0x61) & 0x40) == 0) as c_long;
    (*ucontrol).value.integer.value[1] = ((snd_es1938_mixer_read(chip, 0x63) & 0x40) == 0) as c_long;
    0
}

unsafe extern "C" fn snd_es1938_hwv_free(kcontrol: *mut snd_kcontrol) {
    let chip = snd_kcontrol_chip(kcontrol);
    (*chip).master_volume = core::ptr::null_mut();
    (*chip).master_switch = core::ptr::null_mut();
    (*chip).hw_volume = core::ptr::null_mut();
    (*chip).hw_switch = core::ptr::null_mut();
}

unsafe extern "C" fn snd_es1938_reg_bits(chip: *mut es1938, reg: u8, mask: u8, val: u8) -> c_int {
    if reg < 0xa0 { snd_es1938_mixer_bits(chip, reg, mask, val) } else { snd_es1938_bits(chip, reg, mask, val) }
}

unsafe extern "C" fn snd_es1938_reg_read(chip: *mut es1938, reg: u8) -> c_int {
    if reg < 0xa0 { snd_es1938_mixer_read(chip, reg) } else { snd_es1938_read(chip, reg) as c_int }
}

const fn ES1938_SINGLE_PRIV(reg: c_ulong, shift: c_ulong, mask: c_ulong, invert: c_ulong) -> c_ulong {
    reg | (shift << 8) | (mask << 16) | (invert << 24)
}
const fn ES1938_DOUBLE_PRIV(left_reg: c_ulong, right_reg: c_ulong, shift_left: c_ulong, shift_right: c_ulong, mask: c_ulong, invert: c_ulong) -> c_ulong {
    left_reg | (right_reg << 8) | (shift_left << 16) | (shift_right << 19) | (mask << 24) | (invert << 22)
}

unsafe extern "C" fn snd_es1938_info_single(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let mask = ((*kcontrol).private_value >> 16) & 0xff;
    (*uinfo).type_ = if mask == 1 { SNDRV_CTL_ELEM_TYPE_BOOLEAN } else { SNDRV_CTL_ELEM_TYPE_INTEGER };
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = mask as c_long;
    0
}

unsafe extern "C" fn snd_es1938_get_single(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let reg = ((*kcontrol).private_value & 0xff) as u8;
    let shift = ((*kcontrol).private_value >> 8) & 0xff;
    let mask = ((*kcontrol).private_value >> 16) & 0xff;
    let invert = ((*kcontrol).private_value >> 24) & 0xff;
    let val = snd_es1938_reg_read(chip, reg) as c_ulong;
    (*ucontrol).value.integer.value[0] = ((val >> shift) & mask) as c_long;
    if invert != 0 { (*ucontrol).value.integer.value[0] = mask as c_long - (*ucontrol).value.integer.value[0]; }
    0
}

unsafe extern "C" fn snd_es1938_put_single(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let reg = ((*kcontrol).private_value & 0xff) as u8;
    let shift = ((*kcontrol).private_value >> 8) & 0xff;
    let mut mask = ((*kcontrol).private_value >> 16) & 0xff;
    let invert = ((*kcontrol).private_value >> 24) & 0xff;
    let mut val = ((*ucontrol).value.integer.value[0] as c_ulong & mask) as u8;
    if invert != 0 { val = (mask as u8).wrapping_sub(val); }
    mask <<= shift;
    val <<= shift;
    (snd_es1938_reg_bits(chip, reg, mask as u8, val) != val as c_int) as c_int
}

unsafe extern "C" fn snd_es1938_info_double(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let mask = ((*kcontrol).private_value >> 24) & 0xff;
    (*uinfo).type_ = if mask == 1 { SNDRV_CTL_ELEM_TYPE_BOOLEAN } else { SNDRV_CTL_ELEM_TYPE_INTEGER };
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = mask as c_long;
    0
}

unsafe extern "C" fn snd_es1938_get_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let left_reg = ((*kcontrol).private_value & 0xff) as u8;
    let right_reg = ((*kcontrol).private_value >> 8 & 0xff) as u8;
    let shift_left = ((*kcontrol).private_value >> 16) & 0x07;
    let shift_right = ((*kcontrol).private_value >> 19) & 0x07;
    let mask = ((*kcontrol).private_value >> 24) & 0xff;
    let invert = ((*kcontrol).private_value >> 22) & 1;
    let left = snd_es1938_reg_read(chip, left_reg) as c_ulong;
    let right = if left_reg != right_reg { snd_es1938_reg_read(chip, right_reg) as c_ulong } else { left };
    (*ucontrol).value.integer.value[0] = ((left >> shift_left) & mask) as c_long;
    (*ucontrol).value.integer.value[1] = ((right >> shift_right) & mask) as c_long;
    if invert != 0 {
        (*ucontrol).value.integer.value[0] = mask as c_long - (*ucontrol).value.integer.value[0];
        (*ucontrol).value.integer.value[1] = mask as c_long - (*ucontrol).value.integer.value[1];
    }
    0
}

unsafe extern "C" fn snd_es1938_put_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let left_reg = ((*kcontrol).private_value & 0xff) as u8;
    let right_reg = ((*kcontrol).private_value >> 8 & 0xff) as u8;
    let shift_left = ((*kcontrol).private_value >> 16) & 0x07;
    let shift_right = ((*kcontrol).private_value >> 19) & 0x07;
    let mask = ((*kcontrol).private_value >> 24) & 0xff;
    let invert = ((*kcontrol).private_value >> 22) & 1;
    let mut val1 = ((*ucontrol).value.integer.value[0] as c_ulong & mask) as u8;
    let mut val2 = ((*ucontrol).value.integer.value[1] as c_ulong & mask) as u8;
    if invert != 0 {
        val1 = (mask as u8).wrapping_sub(val1);
        val2 = (mask as u8).wrapping_sub(val2);
    }
    val1 <<= shift_left;
    val2 <<= shift_right;
    let mask1 = (mask << shift_left) as u8;
    let mask2 = (mask << shift_right) as u8;
    let mut change: c_int;
    if left_reg != right_reg {
        change = 0;
        if snd_es1938_reg_bits(chip, left_reg, mask1, val1) != val1 as c_int { change = 1; }
        if snd_es1938_reg_bits(chip, right_reg, mask2, val2) != val2 as c_int { change = 1; }
    } else {
        change = (snd_es1938_reg_bits(chip, left_reg, mask1 | mask2, val1 | val2) != (val1 | val2) as c_int) as c_int;
    }
    change
}

static db_scale_master: [c_uint; 8] = [0, 54, 0, 54, 63, 0, 0, 0];
static db_scale_audio1: [c_uint; 8] = [0, 8, 0, 8, 15, 0, 0, 0];
static db_scale_audio2: [c_uint; 8] = [0, 8, 0, 8, 15, 0, 0, 0];
static db_scale_mic: [c_uint; 8] = [0, 8, 0, 8, 15, 0, 0, 0];
static db_scale_line: [c_uint; 8] = [0, 8, 0, 8, 15, 0, 0, 0];
static db_scale_capture: [c_uint; 4] = [0, 150, 0, 0];

const fn control(name: *const c_char, index: c_uint, info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>, get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, private_value: c_ulong, access: c_uint, tlv: *const c_uint) -> snd_kcontrol_new {
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access, name, index, info, get, put, private_value, tlv: snd_kcontrol_tlv { p: tlv } }
}

static snd_es1938_controls: [snd_kcontrol_new; 31] = [
    control(b"Master Playback Volume\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_double), Some(snd_es1938_get_double), Some(snd_es1938_put_double), ES1938_DOUBLE_PRIV(0x60, 0x62, 0, 0, 63, 0), SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, db_scale_master.as_ptr()),
    control(b"Master Playback Switch\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_double), Some(snd_es1938_get_double), Some(snd_es1938_put_double), ES1938_DOUBLE_PRIV(0x60, 0x62, 6, 6, 1, 1), SNDRV_CTL_ELEM_ACCESS_READWRITE, core::ptr::null()),
    control(b"Hardware Master Playback Volume\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_hw_volume), Some(snd_es1938_get_hw_volume), None, 0, SNDRV_CTL_ELEM_ACCESS_READ, core::ptr::null()),
    control(b"Hardware Master Playback Switch\0".as_ptr() as *const c_char, 0, Some(snd_ctl_boolean_stereo_info), Some(snd_es1938_get_hw_switch), None, 0, SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_TLV_READ, db_scale_master.as_ptr()),
    control(b"Hardware Volume Split\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_single), Some(snd_es1938_get_single), Some(snd_es1938_put_single), ES1938_SINGLE_PRIV(0x64, 7, 1, 0), SNDRV_CTL_ELEM_ACCESS_READWRITE, core::ptr::null()),
    control(b"Line Playback Volume\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_double), Some(snd_es1938_get_double), Some(snd_es1938_put_double), ES1938_DOUBLE_PRIV(0x3e, 0x3e, 4, 0, 15, 0), SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, db_scale_line.as_ptr()),
    control(b"CD Playback Volume\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_double), Some(snd_es1938_get_double), Some(snd_es1938_put_double), ES1938_DOUBLE_PRIV(0x38, 0x38, 4, 0, 15, 0), SNDRV_CTL_ELEM_ACCESS_READWRITE, core::ptr::null()),
    control(b"FM Playback Volume\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_double), Some(snd_es1938_get_double), Some(snd_es1938_put_double), ES1938_DOUBLE_PRIV(0x36, 0x36, 4, 0, 15, 0), SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, db_scale_mic.as_ptr()),
    control(b"Mono Playback Volume\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_double), Some(snd_es1938_get_double), Some(snd_es1938_put_double), ES1938_DOUBLE_PRIV(0x6d, 0x6d, 4, 0, 15, 0), SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, db_scale_line.as_ptr()),
    control(b"Mic Playback Volume\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_double), Some(snd_es1938_get_double), Some(snd_es1938_put_double), ES1938_DOUBLE_PRIV(0x1a, 0x1a, 4, 0, 15, 0), SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, db_scale_mic.as_ptr()),
    control(b"Aux Playback Volume\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_double), Some(snd_es1938_get_double), Some(snd_es1938_put_double), ES1938_DOUBLE_PRIV(0x3a, 0x3a, 4, 0, 15, 0), SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, db_scale_line.as_ptr()),
    control(b"Capture Volume\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_double), Some(snd_es1938_get_double), Some(snd_es1938_put_double), ES1938_DOUBLE_PRIV(0xb4, 0xb4, 4, 0, 15, 0), SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, db_scale_capture.as_ptr()),
    control(b"Beep Volume\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_single), Some(snd_es1938_get_single), Some(snd_es1938_put_single), ES1938_SINGLE_PRIV(0x3c, 0, 7, 0), SNDRV_CTL_ELEM_ACCESS_READWRITE, core::ptr::null()),
    control(b"Record Monitor\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_single), Some(snd_es1938_get_single), Some(snd_es1938_put_single), ES1938_SINGLE_PRIV(0xa8, 3, 1, 0), SNDRV_CTL_ELEM_ACCESS_READWRITE, core::ptr::null()),
    control(b"Capture Switch\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_single), Some(snd_es1938_get_single), Some(snd_es1938_put_single), ES1938_SINGLE_PRIV(0x1c, 4, 1, 1), SNDRV_CTL_ELEM_ACCESS_READWRITE, core::ptr::null()),
    control(b"Capture Source\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_mux), Some(snd_es1938_get_mux), Some(snd_es1938_put_mux), 0, SNDRV_CTL_ELEM_ACCESS_READWRITE, core::ptr::null()),
    control(b"Mono Input Playback Volume\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_double), Some(snd_es1938_get_double), Some(snd_es1938_put_double), ES1938_DOUBLE_PRIV(0x6d, 0x6d, 4, 0, 15, 0), SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, db_scale_line.as_ptr()),
    control(b"PCM Capture Volume\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_double), Some(snd_es1938_get_double), Some(snd_es1938_put_double), ES1938_DOUBLE_PRIV(0x69, 0x69, 4, 0, 15, 0), SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, db_scale_audio2.as_ptr()),
    control(b"Mic Capture Volume\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_double), Some(snd_es1938_get_double), Some(snd_es1938_put_double), ES1938_DOUBLE_PRIV(0x68, 0x68, 4, 0, 15, 0), SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, db_scale_mic.as_ptr()),
    control(b"Line Capture Volume\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_double), Some(snd_es1938_get_double), Some(snd_es1938_put_double), ES1938_DOUBLE_PRIV(0x6e, 0x6e, 4, 0, 15, 0), SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, db_scale_line.as_ptr()),
    control(b"FM Capture Volume\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_double), Some(snd_es1938_get_double), Some(snd_es1938_put_double), ES1938_DOUBLE_PRIV(0x6b, 0x6b, 4, 0, 15, 0), SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, db_scale_mic.as_ptr()),
    control(b"Mono Capture Volume\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_double), Some(snd_es1938_get_double), Some(snd_es1938_put_double), ES1938_DOUBLE_PRIV(0x6f, 0x6f, 4, 0, 15, 0), SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, db_scale_line.as_ptr()),
    control(b"CD Capture Volume\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_double), Some(snd_es1938_get_double), Some(snd_es1938_put_double), ES1938_DOUBLE_PRIV(0x6a, 0x6a, 4, 0, 15, 0), SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, db_scale_line.as_ptr()),
    control(b"Aux Capture Volume\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_double), Some(snd_es1938_get_double), Some(snd_es1938_put_double), ES1938_DOUBLE_PRIV(0x6c, 0x6c, 4, 0, 15, 0), SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, db_scale_line.as_ptr()),
    control(b"PCM Playback Volume\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_double), Some(snd_es1938_get_double), Some(snd_es1938_put_double), ES1938_DOUBLE_PRIV(0x7c, 0x7c, 4, 0, 15, 0), SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, db_scale_audio2.as_ptr()),
    control(b"PCM Playback Volume\0".as_ptr() as *const c_char, 1, Some(snd_es1938_info_double), Some(snd_es1938_get_double), Some(snd_es1938_put_double), ES1938_DOUBLE_PRIV(0x14, 0x14, 4, 0, 15, 0), SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, db_scale_audio1.as_ptr()),
    control(b"3D Control - Level\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_single), Some(snd_es1938_get_single), Some(snd_es1938_put_single), ES1938_SINGLE_PRIV(0x52, 0, 63, 0), SNDRV_CTL_ELEM_ACCESS_READWRITE, core::ptr::null()),
    control(b"3D Control - Switch\0".as_ptr() as *const c_char, 0, Some(snd_ctl_boolean_mono_info), Some(snd_es1938_get_spatializer_enable), Some(snd_es1938_put_spatializer_enable), 0, SNDRV_CTL_ELEM_ACCESS_READWRITE, core::ptr::null()),
    control(b"Mic Boost (+26dB)\0".as_ptr() as *const c_char, 0, Some(snd_es1938_info_single), Some(snd_es1938_get_single), Some(snd_es1938_put_single), ES1938_SINGLE_PRIV(0x7d, 3, 1, 0), SNDRV_CTL_ELEM_ACCESS_READWRITE, core::ptr::null()),
    control(core::ptr::null(), 0, None, None, None, 0, 0, core::ptr::null()),
    control(core::ptr::null(), 0, None, None, None, 0, 0, core::ptr::null()),
];

unsafe extern "C" fn snd_es1938_chip_init(chip: *mut es1938) {
    snd_es1938_reset(chip);
    pci_set_master((*chip).pci);
    pci_write_config_word((*chip).pci, SL_PCI_LEGACYCONTROL, 0x805f);
    pci_write_config_word((*chip).pci, SL_PCI_DDMACONTROL, (*chip).ddma_port as c_uint | 1);
    pci_write_config_dword((*chip).pci, SL_PCI_CONFIG, 0);
    outb(0xf0, SLIO_REG(chip, ESSIO_REG_IRQCONTROL));
    outb(0, SLDM_REG(chip, ESSDM_REG_DMACLEAR));
}

static saved_regs: [u8; SAVED_REG_SIZE + 1] = [
    0x14, 0x1a, 0x1c, 0x3a, 0x3c, 0x3e, 0x36, 0x38,
    0x50, 0x52, 0x60, 0x61, 0x62, 0x63, 0x64, 0x68,
    0x69, 0x6a, 0x6b, 0x6d, 0x6e, 0x6f, 0x7c, 0x7d,
    0xa8, 0xb4, 0, 0, 0, 0, 0, 0, 0,
];

unsafe extern "C" fn es1938_suspend(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev);
    let chip = (*card).private_data as *mut es1938;
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    let mut i = 0usize;
    while saved_regs[i] != 0 {
        (*chip).saved_regs[i] = snd_es1938_reg_read(chip, saved_regs[i]) as u8;
        i += 1;
    }
    outb(0x00, SLIO_REG(chip, ESSIO_REG_IRQCONTROL));
    if (*chip).irq >= 0 {
        free_irq((*chip).irq, chip as *mut c_void);
        (*chip).irq = -1;
        (*card).sync_irq = -1;
    }
    0
}

unsafe extern "C" fn es1938_resume(dev: *mut device) -> c_int {
    let pci = to_pci_dev(dev);
    let card = dev_get_drvdata(dev);
    let chip = (*card).private_data as *mut es1938;
    if request_irq((*pci).irq, snd_es1938_interrupt, IRQF_SHARED, KBUILD_MODNAME, chip as *mut c_void) != 0 {
        snd_card_disconnect(card);
        return -EIO;
    }
    (*chip).irq = (*pci).irq;
    (*card).sync_irq = (*chip).irq;
    snd_es1938_chip_init(chip);
    let mut i = 0usize;
    while saved_regs[i] != 0 {
        if saved_regs[i] < 0xa0 { snd_es1938_mixer_write(chip, saved_regs[i], (*chip).saved_regs[i]); }
        else { snd_es1938_write(chip, saved_regs[i], (*chip).saved_regs[i]); }
        i += 1;
    }
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}

static es1938_pm: dev_pm_ops = dev_pm_ops { _private: [] };

unsafe extern "C" fn snd_es1938_create_gameport(chip: *mut es1938) -> c_int {
    let gp = gameport_allocate_port();
    (*chip).gameport = gp;
    if gp.is_null() { return -ENOMEM; }
    gameport_set_name(gp, b"ES1938\0".as_ptr() as *const c_char);
    gameport_set_phys(gp, b"pci%s/gameport0\0".as_ptr() as *const c_char, pci_name((*chip).pci));
    gameport_set_dev_parent(gp, &mut (*(*chip).pci).dev);
    (*gp).io = (*chip).game_port;
    gameport_register_port(gp);
    0
}

unsafe extern "C" fn snd_es1938_free_gameport(chip: *mut es1938) {
    if !(*chip).gameport.is_null() {
        gameport_unregister_port((*chip).gameport);
        (*chip).gameport = core::ptr::null_mut();
    }
}

unsafe extern "C" fn snd_es1938_free(card: *mut snd_card) {
    let chip = (*card).private_data as *mut es1938;
    outb(0x00, SLIO_REG(chip, ESSIO_REG_IRQCONTROL));
    if !(*chip).rmidi.is_null() { snd_es1938_mixer_bits(chip, ESSSB_IREG_MPU401CONTROL, 0x40, 0); }
    snd_es1938_free_gameport(chip);
    if (*chip).irq >= 0 { free_irq((*chip).irq, chip as *mut c_void); }
}

unsafe extern "C" fn snd_es1938_create(card: *mut snd_card, pci: *mut pci_dev) -> c_int {
    let chip = (*card).private_data as *mut es1938;
    let mut err = pcim_enable_device(pci);
    if err < 0 { return err; }
    if dma_set_mask_and_coherent(&mut (*pci).dev, DMA_BIT_MASK(24)) != 0 { return -ENXIO; }
    spin_lock_init(&mut (*chip).reg_lock);
    spin_lock_init(&mut (*chip).mixer_lock);
    (*chip).card = card;
    (*chip).pci = pci;
    (*chip).irq = -1;
    err = pcim_request_all_regions(pci, b"ESS Solo-1\0".as_ptr() as *const c_char);
    if err < 0 { return err; }
    (*chip).io_port = pci_resource_start(pci, 0);
    (*chip).sb_port = pci_resource_start(pci, 1);
    (*chip).vc_port = pci_resource_start(pci, 2);
    (*chip).mpu_port = pci_resource_start(pci, 3);
    (*chip).game_port = pci_resource_start(pci, 4);
    if request_irq((*pci).irq, snd_es1938_interrupt, IRQF_SHARED, KBUILD_MODNAME, chip as *mut c_void) != 0 { return -EBUSY; }
    (*chip).irq = (*pci).irq;
    (*card).sync_irq = (*chip).irq;
    (*card).private_free = Some(snd_es1938_free);
    (*chip).ddma_port = (*chip).vc_port + 0x00; /* fix from Thomas Sailer */
    snd_es1938_chip_init(chip);
    0
}

unsafe extern "C" fn snd_es1938_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chip = dev_id as *mut es1938;
    let status = inb(SLIO_REG(chip, ESSIO_REG_IRQCONTROL));
    let mut handled: c_int = 0;
    if (status & 0x10) != 0 {
        handled = 1;
        let _audiostatus = inb(SLSB_REG(chip, ESSSB_REG_STATUS));
        if ((*chip).active & ADC1) != 0 { snd_pcm_period_elapsed((*chip).capture_substream); }
        else if ((*chip).active & DAC1) != 0 { snd_pcm_period_elapsed((*chip).playback2_substream); }
    }
    if (status & 0x20) != 0 {
        handled = 1;
        snd_es1938_mixer_bits(chip, ESSSB_IREG_AUDIO2CONTROL2, 0x80, 0);
        if ((*chip).active & DAC2) != 0 { snd_pcm_period_elapsed((*chip).playback1_substream); }
    }
    if (status & 0x40) != 0 {
        let split = snd_es1938_mixer_read(chip, 0x64) & 0x80;
        handled = 1;
        snd_ctl_notify((*chip).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*chip).hw_switch).id);
        snd_ctl_notify((*chip).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*chip).hw_volume).id);
        if split == 0 {
            snd_ctl_notify((*chip).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*chip).master_switch).id);
            snd_ctl_notify((*chip).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*chip).master_volume).id);
        }
        snd_es1938_mixer_write(chip, 0x66, 0x00);
    }
    if (status & 0x80) != 0 {
        /* the original comments note that disabling MIDI interrupt handling here is evil. */
        if !(*chip).rmidi.is_null() {
            handled = 1;
            snd_mpu401_uart_interrupt(irq, (*(*chip).rmidi).private_data);
        }
    }
    IRQ_RETVAL(handled)
}

const ES1938_DMA_SIZE: c_int = 64;

unsafe extern "C" fn snd_es1938_mixer(chip: *mut es1938) -> c_int {
    let card = (*chip).card;
    strscpy((*card).mixername.as_mut_ptr(), b"ESS Solo-1\0".as_ptr() as *const c_char);
    let mut idx = 0usize;
    while idx < snd_es1938_controls.len() {
        let kctl = snd_ctl_new1(&snd_es1938_controls[idx], chip as *mut c_void);
        if kctl.is_null() { return -ENOMEM; }
        match idx {
            0 => { (*chip).master_volume = kctl; (*kctl).private_free = Some(snd_es1938_hwv_free); }
            1 => { (*chip).master_switch = kctl; (*kctl).private_free = Some(snd_es1938_hwv_free); }
            2 => { (*chip).hw_volume = kctl; (*kctl).private_free = Some(snd_es1938_hwv_free); }
            3 => { (*chip).hw_switch = kctl; (*kctl).private_free = Some(snd_es1938_hwv_free); }
            _ => {}
        }
        let err = snd_ctl_add(card, kctl);
        if err < 0 { return err; }
        idx += 1;
    }
    0
}

unsafe extern "C" fn __snd_es1938_probe(pci: *mut pci_dev, _pci_id: *const pci_device_id) -> c_int {
    static mut dev: c_int = 0;
    let mut card: *mut snd_card = core::ptr::null_mut();
    let mut opl3: *mut snd_opl3 = core::ptr::null_mut();
    if dev >= SNDRV_CARDS as c_int { return -ENODEV; }
    if !enable[dev as usize] { dev += 1; return -ENOENT; }
    let mut err = snd_devm_card_new(&mut (*pci).dev, index[dev as usize], id[dev as usize], THIS_MODULE, core::mem::size_of::<es1938>(), &mut card);
    if err < 0 { return err; }
    let chip = (*card).private_data as *mut es1938;
    for idx in 0..5 {
        if pci_resource_start(pci, idx) == 0 || (pci_resource_flags(pci, idx) & IORESOURCE_IO) == 0 { return -ENODEV; }
    }
    err = snd_es1938_create(card, pci);
    if err < 0 { return err; }
    strscpy((*card).driver.as_mut_ptr(), b"ES1938\0".as_ptr() as *const c_char);
    strscpy((*card).shortname.as_mut_ptr(), b"ESS ES1938 (Solo-1)\0".as_ptr() as *const c_char);
    sprintf((*card).longname.as_mut_ptr(), b"%s rev %i, irq %i\0".as_ptr() as *const c_char, (*card).shortname.as_ptr(), (*chip).revision as c_int, (*chip).irq);
    err = snd_es1938_new_pcm(chip, 0);
    if err < 0 { return err; }
    err = snd_es1938_mixer(chip);
    if err < 0 { return err; }
    if snd_opl3_create(card, SLSB_REG(chip, ESSSB_REG_FMLOWADDR), SLSB_REG(chip, ESSSB_REG_FMHIGHADDR), OPL3_HW_OPL3, 1, &mut opl3) >= 0 {
        err = snd_opl3_timer_new(opl3, 0, 1);
        if err < 0 { return err; }
        err = snd_opl3_hwdep_new(opl3, 0, 1, core::ptr::null_mut());
        if err < 0 { return err; }
    }
    if snd_mpu401_uart_new(card, 0, MPU401_HW_MPU401, (*chip).mpu_port, MPU401_INFO_INTEGRATED | MPU401_INFO_IRQ_HOOK, -1, &mut (*chip).rmidi) >= 0 {
        snd_es1938_mixer_bits(chip, ESSSB_IREG_MPU401CONTROL, 0x40, 0x40);
    }
    snd_es1938_create_gameport(chip);
    err = snd_card_register(card);
    if err < 0 { return err; }
    pci_set_drvdata(pci, card as *mut c_void);
    dev += 1;
    0
}

unsafe extern "C" fn snd_es1938_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    snd_card_free_on_error(&mut (*pci).dev, __snd_es1938_probe(pci, pci_id))
}

static mut es1938_driver: pci_driver = pci_driver {
    name: KBUILD_MODNAME,
    id_table: snd_es1938_ids.as_ptr(),
    probe: Some(snd_es1938_probe),
    driver: pci_driver_inner { pm: &es1938_pm },
};

/* module_pci_driver(es1938_driver); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
