// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Driver for SiS7019 Audio Accelerator
 *
 *  Copyright (C) 2004-2007, David Dillow
 *  Written by David Dillow <dave@thedillows.org>
 *  Inspired by the Trident 4D-WaveDX/NX driver.
 *
 *  All rights reserved.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u16 = u16;
type u32 = u32;
type dma_addr_t = u32;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;
type bool_t = bool;

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
    pub irq: c_int,
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
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}
#[repr(C)]
pub struct snd_pcm_runtime {
    pub private_data: *mut c_void,
    pub hw: snd_pcm_hardware,
    pub format: c_int,
    pub channels: c_uint,
    pub dma_addr: dma_addr_t,
    pub buffer_size: u16,
    pub period_size: u16,
    pub rate: c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hardware {
    pub info: u32,
    pub formats: u64,
    pub rates: u32,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub buffer_bytes_max: c_ulong,
    pub period_bytes_min: c_ulong,
    pub period_bytes_max: c_ulong,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm {
    pub private_data: *mut c_void,
    pub name: [c_char; 80],
}
#[repr(C)]
pub struct snd_card {
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    pub sync_irq: c_int,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
}
#[repr(C)]
pub struct snd_ac97 {
    pub private_data: *mut c_void,
    pub num: c_int,
    pub rates: [u32; 8],
}
#[repr(C)]
pub struct snd_ac97_bus {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_ac97_template {
    pub private_data: *mut c_void,
    pub num: c_int,
}
#[repr(C)]
pub struct snd_ac97_bus_ops {
    pub write: Option<unsafe extern "C" fn(*mut snd_ac97, c_ushort, c_ushort)>,
    pub read: Option<unsafe extern "C" fn(*mut snd_ac97, c_ushort) -> c_ushort>,
}
type c_ushort = u16;
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

unsafe extern "C" {
    static mut jiffies: c_ulong;
    static KBUILD_MODNAME: *const c_char;
    static THIS_MODULE: *mut c_void;
    fn writew(value: u16, addr: *mut c_void);
    fn writel(value: u32, addr: *mut c_void);
    fn readw(addr: *mut c_void) -> u16;
    fn readl(addr: *mut c_void) -> u32;
    fn inl(port: c_ulong) -> u32;
    fn inw(port: c_ulong) -> u16;
    fn outl(value: u32, port: c_ulong);
    fn udelay(usecs: c_ulong);
    fn msleep(msecs: c_uint);
    fn msecs_to_jiffies(msecs: c_uint) -> c_ulong;
    fn time_before_eq(a: c_ulong, b: c_ulong) -> bool_t;
    fn __ffs(word: u32) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut c_void;
    fn snd_pcm_hw_constraint_minmax(runtime: *mut snd_pcm_runtime, var: c_int, min: c_ulong, max: c_ulong) -> c_int;
    fn snd_pcm_set_sync(substream: *mut snd_pcm_substream);
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn snd_pcm_format_signed(format: c_int) -> bool_t;
    fn snd_pcm_format_size(format: c_int, samples: c_int) -> c_int;
    fn snd_pcm_limit_hw_rates(runtime: *mut snd_pcm_runtime) -> c_int;
    fn snd_pcm_trigger_done(s: *mut snd_pcm_substream, master: *mut snd_pcm_substream);
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, ty: c_int, dev: *mut device, min: usize, max: usize);
    fn params_period_size(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_buffer_size(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dma_map_single(dev: *mut device, ptr: *mut c_void, size: usize, dir: c_int) -> dma_addr_t;
    fn dma_unmap_single(dev: *mut device, addr: dma_addr_t, size: usize, dir: c_int);
    fn snd_ac97_set_rate(ac97: *mut snd_ac97, reg: c_int, rate: c_uint) -> c_int;
    fn snd_ac97_bus(card: *mut snd_card, num: c_int, ops: *const snd_ac97_bus_ops, private_data: *mut c_void, bus: *mut *mut snd_ac97_bus) -> c_int;
    fn snd_ac97_mixer(bus: *mut snd_ac97_bus, template: *mut snd_ac97_template, rac97: *mut *mut snd_ac97) -> c_int;
    fn snd_ac97_get_short_name(ac97: *mut snd_ac97) -> *const c_char;
    fn snd_ac97_suspend(ac97: *mut snd_ac97);
    fn snd_ac97_resume(ac97: *mut snd_ac97);
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn free_irq(irq: c_int, dev_id: *mut c_void);
    fn request_irq(irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, dev: *mut c_void) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
    fn snd_card_disconnect(card: *mut snd_card);
    fn devm_kmalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy_fromio(to: *mut c_void, from: *mut c_void, count: usize);
    fn memcpy_toio(to: *mut c_void, from: *mut c_void, count: usize);
    fn pcim_enable_device(pdev: *mut pci_dev) -> c_int;
    fn dma_set_mask(dev: *mut device, mask: u64) -> c_int;
    fn mutex_init(lock: *mut mutex);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn pci_resource_start(dev: *mut pci_dev, bar: c_int) -> c_ulong;
    fn pcim_request_region(pdev: *mut pci_dev, bar: c_int, name: *const c_char) -> c_int;
    fn pcim_iomap_region(pdev: *mut pci_dev, bar: c_int, name: *const c_char) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool_t;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn pci_set_master(pdev: *mut pci_dev);
    fn snd_devm_card_new(dev: *mut device, idx: c_int, id: *mut c_char, module: *mut c_void, extra_size: usize, card: *mut *mut snd_card) -> c_int;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(pdev: *mut pci_dev, data: *mut c_void);
    fn snd_card_free_on_error(dev: *mut device, probe: c_int) -> c_int;
}

const SNDRV_DEFAULT_IDX1: c_int = -1;
static mut SNDRV_DEFAULT_STR1_BUF: [c_char; 1] = [0];
const PCI_VENDOR_ID_SI: u32 = 0x1039;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_SHARED: c_ulong = 0x80;
const ENOMEM: c_int = 12;
const EAGAIN: c_int = 11;
const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENXIO: c_int = 6;
const ENOENT: c_int = 2;
const GFP_KERNEL: c_int = 0;
const DMA_TO_DEVICE: c_int = 1;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_HW_PARAM_PERIOD_SIZE: c_int = 0;
const SNDRV_PCM_HW_PARAM_BUFFER_SIZE: c_int = 1;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_INFO_MMAP: u32 = 1 << 0;
const SNDRV_PCM_INFO_MMAP_VALID: u32 = 1 << 1;
const SNDRV_PCM_INFO_INTERLEAVED: u32 = 1 << 2;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: u32 = 1 << 3;
const SNDRV_PCM_INFO_SYNC_START: u32 = 1 << 4;
const SNDRV_PCM_INFO_RESUME: u32 = 1 << 5;
const SNDRV_PCM_FMTBIT_S8: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_U8: u64 = 1 << 1;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_U16_LE: u64 = 1 << 3;
const SNDRV_PCM_RATE_8000_48000: u32 = 0;
const SNDRV_PCM_RATE_CONTINUOUS: u32 = 1 << 30;
const SNDRV_PCM_RATE_48000: u32 = 1 << 15;
const AC97_RATES_ADC: usize = 0;
const AC97_PCM_LR_ADC_RATE: c_int = 0;

/* Constants supplied by sis7019.h in the original source. */
unsafe extern "C" {
    static SIS_GISR: c_ulong;
    static SIS_PISR_A: c_ulong;
    static SIS_PISR_B: c_ulong;
    static SIS_RISR: c_ulong;
    static SIS_GCR: c_ulong;
    static SIS_GIER: c_ulong;
    static SIS_AC97_SEMA: c_ulong;
    static SIS_AC97_STATUS: c_ulong;
    static SIS_AC97_CMD: c_ulong;
    static SIS_AC97_CONF: c_ulong;
    static SIS_AC97_PSR: c_ulong;
    static SIS_DMA_CSR: c_ulong;
    static SIS_PLAY_SYNC_GROUP_A: c_ulong;
    static SIS_PLAY_SYNC_GROUP_B: c_ulong;
    static SIS_PLAY_SYNC_GROUP_C: c_ulong;
    static SIS_PLAY_SYNC_GROUP_D: c_ulong;
    static SIS_MIXER_SYNC_GROUP: c_ulong;
    static SIS_WEVCR: c_ulong;
    static SIS_WECCR: c_ulong;
    static SIS_RECORD_START_REG: c_ulong;
    static SIS_PLAY_START_A_REG: c_ulong;
    static SIS_PLAY_START_B_REG: c_ulong;
    static SIS_RECORD_STOP_REG: c_ulong;
    static SIS_PLAY_STOP_A_REG: c_ulong;
    static SIS_PLAY_STOP_B_REG: c_ulong;
    static SIS_PLAY_DMA_FORMAT_CSO: usize;
    static SIS_PLAY_DMA_BASE: usize;
    static SIS_PLAY_DMA_CONTROL: usize;
    static SIS_PLAY_DMA_SSO_ESO: usize;
    static SIS_CAPTURE_DMA_FORMAT_CSO: usize;
    static SIS_CAPTURE_DMA_BASE: usize;
    static SIS_CAPTURE_DMA_CONTROL: usize;
    static SIS_WAVE_SIZE: u32;
    static SIS_WAVE_GENERAL: usize;
    static SIS_WAVE_GENERAL_ARTICULATION: usize;
    static SIS_WAVE_CHANNEL_CONTROL: usize;
    static SIS_GISR_AUDIO_PLAY_DMA_IRQ_STATUS: u32;
    static SIS_GISR_AUDIO_RECORD_DMA_IRQ_STATUS: u32;
    static SIS_PLAY_DMA_FORMAT_8BIT: u32;
    static SIS_PLAY_DMA_FORMAT_UNSIGNED: u32;
    static SIS_PLAY_DMA_FORMAT_MONO: u32;
    static SIS_PLAY_DMA_LOOP: u32;
    static SIS_PLAY_DMA_INTR_AT_LEO: u32;
    static SIS_PLAY_DMA_INTR_AT_MLP: u32;
    static SIS_PLAY_DMA_INTR_AT_SSO: u32;
    static SIS_CAPTURE_DMA_FORMAT_8BIT: u32;
    static SIS_CAPTURE_DMA_FORMAT_UNSIGNED: u32;
    static SIS_CAPTURE_DMA_FORMAT_MONO: u32;
    static SIS_CAPTURE_DMA_LOOP: u32;
    static SIS_CAPTURE_DMA_INTR_AT_LEO: u32;
    static SIS_CAPTURE_DMA_INTR_AT_MLP: u32;
    static SIS_WAVE_GENERAL_WAVE_VOLUME: u32;
    static SIS_WAVE_CHANNEL_CONTROL_FIRST_SAMPLE: u32;
    static SIS_WAVE_CHANNEL_CONTROL_AMP_ENABLE: u32;
    static SIS_WAVE_CHANNEL_CONTROL_INTERPOLATE_ENABLE: u32;
    static SIS_AC97_STATUS_CODEC_READY: u16;
    static SIS_AC97_STATUS_CODEC2_READY: u16;
    static SIS_AC97_STATUS_CODEC3_READY: u16;
    static SIS_AC97_STATUS_BUSY: u16;
    static SIS_AC97_SEMA_BUSY: u16;
    static SIS_AC97_SEMA_RELEASE: u32;
    static SIS_AC97_CMD_CODEC_WRITE: u32;
    static SIS_AC97_CMD_CODEC2_WRITE: u32;
    static SIS_AC97_CMD_CODEC3_WRITE: u32;
    static SIS_AC97_CMD_CODEC_READ: u32;
    static SIS_AC97_CMD_CODEC2_READ: u32;
    static SIS_AC97_CMD_CODEC3_READ: u32;
    static SIS_GCR_SOFTWARE_RESET: u32;
    static SIS_AC97_CMD_CODEC_COLD_RESET: u32;
    static SIS_AC97_CONF_AUDIO_ALIVE: u32;
    static SIS_AC97_CONF_PCM_LR_ENABLE: u32;
    static SIS_AC97_CONF_PCM_CAP_MIC_ENABLE: u32;
    static SIS_AC97_CONF_PCM_CAP_LR_ENABLE: u32;
    static SIS_AC97_CONF_CODEC_VRA_ENABLE: u32;
    static SIS_DMA_CSR_PCI_SETTINGS: u32;
    static SIS_MIXER_RIGHT_NO_ATTEN: u32;
    static SIS_MIXER_LEFT_NO_ATTEN: u32;
    static SIS_MIXER_DEST_0: u32;
    static SIS_GIER_AUDIO_PLAY_DMA_IRQ_ENABLE: u32;
    static SIS_GIER_AUDIO_RECORD_DMA_IRQ_ENABLE: u32;
    static SIS_CAPTURE_CHAN_AC97_PCM_IN: c_int;
    fn SIS_MIXER_START_ADDR(base: *mut c_void, i: c_int) -> *mut c_void;
    fn SIS_MIXER_ADDR(base: *mut c_void, i: c_int) -> *mut c_void;
    fn SIS_PLAY_DMA_ADDR(base: *mut c_void, i: c_int) -> *mut c_void;
    fn SIS_WAVE_ADDR(base: *mut c_void, i: c_int) -> *mut c_void;
    fn SIS_CAPTURE_DMA_ADDR(base: *mut c_void, i: c_int) -> *mut c_void;
}

unsafe fn ptr_add(base: *mut c_void, off: usize) -> *mut c_void {
    (base as *mut u8).add(off) as *mut c_void
}

unsafe fn DIV_ROUND_CLOSEST(x: c_uint, divisor: c_uint) -> c_uint {
    (x + divisor / 2) / divisor
}

unsafe fn DMA_BIT_MASK(n: u32) -> u64 {
    (1u64 << n) - 1
}

/*
MODULE_AUTHOR("David Dillow <dave@thedillows.org>");
MODULE_DESCRIPTION("SiS7019");
MODULE_LICENSE("GPL");
*/

static mut index: c_int = SNDRV_DEFAULT_IDX1;	/* Index 0-MAX */
static mut id: *mut c_char = unsafe { SNDRV_DEFAULT_STR1_BUF.as_mut_ptr() };	/* ID for this card */
static mut enable: bool_t = true;
static mut codecs: c_int = 1;

/*
module_param(index, int, 0444);
MODULE_PARM_DESC(index, "Index value for SiS7019 Audio Accelerator.");
module_param(id, charp, 0444);
MODULE_PARM_DESC(id, "ID string for SiS7019 Audio Accelerator.");
module_param(enable, bool, 0444);
MODULE_PARM_DESC(enable, "Enable SiS7019 Audio Accelerator.");
module_param(codecs, int, 0444);
MODULE_PARM_DESC(codecs, "Set bit to indicate that codec number is expected to be present (default 1)");
*/

static snd_sis7019_ids: [pci_device_id; 2] = [
    pci_device_id { vendor: PCI_VENDOR_ID_SI, device: 0x7019, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
    pci_device_id { vendor: 0, device: 0, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
];

/* MODULE_DEVICE_TABLE(pci, snd_sis7019_ids); */

/* There are three timing modes for the voices.
 *
 * For both playback and capture, when the buffer is one or two periods long,
 * we use the hardware's built-in Mid-Loop Interrupt and End-Loop Interrupt
 * to let us know when the periods have ended.
 *
 * When performing playback with more than two periods per buffer, we set
 * the "Stop Sample Offset" and tell the hardware to interrupt us when we
 * reach it. We then update the offset and continue on until we are
 * interrupted for the next period.
 *
 * Capture channels do not have a SSO, so we allocate a playback channel to
 * use as a timer for the capture periods. We use the SSO on the playback
 * channel to clock out virtual periods, and adjust the virtual period length
 * to maintain synchronization. This algorithm came from the Trident driver.
 *
 * FIXME: It'd be nice to make use of some of the synth features in the
 * hardware, but a woeful lack of documentation is a significant roadblock.
 */
#[repr(C)]
pub struct voice {
    pub flags: u16,
    pub sync_cso: u16,
    pub period_size: u16,
    pub buffer_size: u16,
    pub sync_period_size: u16,
    pub sync_buffer_size: u16,
    pub sso: u32,
    pub vperiod: u32,
    pub substream: *mut snd_pcm_substream,
    pub timing: *mut voice,
    pub ctrl_base: *mut c_void,
    pub wave_base: *mut c_void,
    pub sync_base: *mut c_void,
    pub num: c_int,
}
const VOICE_IN_USE: u16 = 1;
const VOICE_CAPTURE: u16 = 2;
const VOICE_SSO_TIMING: u16 = 4;
const VOICE_SYNC_TIMING: u16 = 8;

/* We need four pages to store our wave parameters during a suspend. If
 * we're not doing power management, we still need to allocate a page
 * for the silence buffer.
 */
const SIS_SUSPEND_PAGES: usize = 4;

#[repr(C)]
pub struct sis7019 {
    pub ioport: c_ulong,
    pub ioaddr: *mut c_void,
    pub irq: c_int,
    pub codecs_present: c_int,
    pub pci: *mut pci_dev,
    pub pcm: *mut snd_pcm,
    pub card: *mut snd_card,
    pub ac97: [*mut snd_ac97; 3],
    /* Protect against more than one thread hitting the AC97
     * registers (in a more polite manner than pounding the hardware
     * semaphore)
     */
    pub ac97_mutex: mutex,
    /* voice_lock protects allocation/freeing of the voice descriptions
     */
    pub voice_lock: spinlock_t,
    pub voices: [voice; 64],
    pub capture_voice: voice,
    /* Allocate pages to store the internal wave state during
     * suspends. When we're operating, this can be used as a silence
     * buffer for a timing channel.
     */
    pub suspend_state: [*mut c_void; SIS_SUSPEND_PAGES],
    pub silence_users: c_int,
    pub silence_dma_addr: dma_addr_t,
}

/* These values are also used by the module param 'codecs' to indicate
 * which codecs should be present.
 */
const SIS_PRIMARY_CODEC_PRESENT: c_int = 0x0001;
const SIS_SECONDARY_CODEC_PRESENT: c_int = 0x0002;
const SIS_TERTIARY_CODEC_PRESENT: c_int = 0x0004;

static sis_playback_hw_info: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_SYNC_START | SNDRV_PCM_INFO_RESUME,
    formats: SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_U16_LE,
    rates: SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_CONTINUOUS,
    rate_min: 4000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 0xfff9 * 4,
    period_bytes_min: 9,
    period_bytes_max: 0xfff9 * 4,
    periods_min: 1,
    periods_max: 0xfff9 / 9,
};

static sis_capture_hw_info: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_SYNC_START | SNDRV_PCM_INFO_RESUME,
    formats: SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_U16_LE,
    rates: SNDRV_PCM_RATE_48000,
    rate_min: 4000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 0xfff9 * 4,
    period_bytes_min: 9,
    period_bytes_max: 0xfff9 * 4,
    periods_min: 1,
    periods_max: 0xfff9 / 9,
};

unsafe extern "C" fn sis_update_sso(voice: *mut voice, period: u16) {
    let base = (*voice).ctrl_base;
    (*voice).sso = (*voice).sso.wrapping_add(period as u32);
    if (*voice).sso >= (*voice).buffer_size as u32 {
        (*voice).sso = (*voice).sso.wrapping_sub((*voice).buffer_size as u32);
    }
    /* Enforce the documented hardware minimum offset */
    if (*voice).sso < 8 {
        (*voice).sso = 8;
    }
    /* The SSO is in the upper 16 bits of the register. */
    writew(((*voice).sso & 0xffff) as u16, ptr_add(base, SIS_PLAY_DMA_SSO_ESO + 2));
}

unsafe extern "C" fn sis_update_voice(voice: *mut voice) {
    if (*voice).flags & VOICE_SSO_TIMING != 0 {
        sis_update_sso(voice, (*voice).period_size);
    } else if (*voice).flags & VOICE_SYNC_TIMING != 0 {
        let mut sync: c_int;
        /* If we've not hit the end of the virtual period, update
         * our records and keep going.
         */
        if (*voice).vperiod > (*voice).period_size as u32 {
            (*voice).vperiod -= (*voice).period_size as u32;
            if (*voice).vperiod < (*voice).period_size as u32 {
                sis_update_sso(voice, (*voice).vperiod as u16);
            } else {
                sis_update_sso(voice, (*voice).period_size);
            }
            return;
        }
        /* Calculate our relative offset between the target and
         * the actual CSO value. Since we're operating in a loop,
         * if the value is more than half way around, we can
         * consider ourselves wrapped.
         */
        sync = (*voice).sync_cso as c_int;
        sync -= readw(ptr_add((*voice).sync_base, SIS_CAPTURE_DMA_FORMAT_CSO)) as c_int;
        if sync > ((*voice).sync_buffer_size / 2) as c_int {
            sync -= (*voice).sync_buffer_size as c_int;
        }
        /* If sync is positive, then we interrupted too early, and
         * we'll need to come back in a few samples and try again.
         * There's a minimum wait, as it takes some time for the DMA
         * engine to startup, etc...
         */
        if sync > 0 {
            if sync < 16 {
                sync = 16;
            }
            sis_update_sso(voice, sync as u16);
            return;
        }
        /* Ok, we interrupted right on time, or (hopefully) just
         * a bit late. We'll adjst our next waiting period based
         * on how close we got.
         *
         * We need to stay just behind the actual channel to ensure
         * it really is past a period when we get our interrupt --
         * otherwise we'll fall into the early code above and have
         * a minimum wait time, which makes us quite late here,
         * eating into the user's time to refresh the buffer, esp.
         * if using small periods.
         *
         * If we're less than 9 samples behind, we're on target.
         * Otherwise, shorten the next vperiod by the amount we've
         * been delayed.
         */
        if sync > -9 {
            (*voice).vperiod = (*voice).sync_period_size as u32 + 1;
        } else {
            (*voice).vperiod = ((*voice).sync_period_size as c_int + sync + 10) as u32;
        }
        if (*voice).vperiod < (*voice).buffer_size as u32 {
            sis_update_sso(voice, (*voice).vperiod as u16);
            (*voice).vperiod = 0;
        } else {
            sis_update_sso(voice, (*voice).period_size);
        }
        sync = (*voice).sync_cso as c_int + (*voice).sync_period_size as c_int;
        if sync >= (*voice).sync_buffer_size as c_int {
            sync -= (*voice).sync_buffer_size as c_int;
        }
        (*voice).sync_cso = sync as u16;
    }
    snd_pcm_period_elapsed((*voice).substream);
}

unsafe extern "C" fn sis_voice_irq(mut status: u32, mut voice: *mut voice) {
    let mut bit: c_int;
    while status != 0 {
        bit = __ffs(status);
        status >>= bit + 1;
        voice = voice.add(bit as usize);
        sis_update_voice(voice);
        voice = voice.add(1);
    }
}

unsafe extern "C" fn sis_interrupt(_irq: c_int, dev: *mut c_void) -> irqreturn_t {
    let sis = dev as *mut sis7019;
    let io = (*sis).ioport;
    let mut voice_ptr: *mut voice;
    let mut intr: u32;
    let mut status: u32;
    intr = inl(io + SIS_GISR);
    intr &= SIS_GISR_AUDIO_PLAY_DMA_IRQ_STATUS | SIS_GISR_AUDIO_RECORD_DMA_IRQ_STATUS;
    if intr == 0 {
        return IRQ_NONE;
    }
    loop {
        status = inl(io + SIS_PISR_A);
        if status != 0 {
            sis_voice_irq(status, (*sis).voices.as_mut_ptr());
            outl(status, io + SIS_PISR_A);
        }
        status = inl(io + SIS_PISR_B);
        if status != 0 {
            sis_voice_irq(status, (*sis).voices.as_mut_ptr().add(32));
            outl(status, io + SIS_PISR_B);
        }
        status = inl(io + SIS_RISR);
        if status != 0 {
            voice_ptr = &mut (*sis).capture_voice;
            if (*voice_ptr).timing.is_null() {
                snd_pcm_period_elapsed((*voice_ptr).substream);
            }
            outl(status, io + SIS_RISR);
        }
        outl(intr, io + SIS_GISR);
        intr = inl(io + SIS_GISR);
        intr &= SIS_GISR_AUDIO_PLAY_DMA_IRQ_STATUS | SIS_GISR_AUDIO_RECORD_DMA_IRQ_STATUS;
        if intr == 0 {
            break;
        }
    }
    IRQ_HANDLED
}

unsafe extern "C" fn sis_rate_to_delta(rate: c_uint) -> u32 {
    let delta: u32;
    if rate == 44100 {
        delta = 0xeb3;
    } else if rate == 8000 {
        delta = 0x2ab;
    } else if rate == 48000 {
        delta = 0x1000;
    } else {
        delta = DIV_ROUND_CLOSEST(rate << 12, 48000) & 0x0000ffff;
    }
    delta
}

unsafe extern "C" fn __sis_map_silence(sis: *mut sis7019) {
    /* Helper function: must hold sis->voice_lock on entry */
    if (*sis).silence_users == 0 {
        (*sis).silence_dma_addr = dma_map_single(&mut (*(*sis).pci).dev, (*sis).suspend_state[0], 4096, DMA_TO_DEVICE);
    }
    (*sis).silence_users += 1;
}

unsafe extern "C" fn __sis_unmap_silence(sis: *mut sis7019) {
    /* Helper function: must hold sis->voice_lock on entry */
    (*sis).silence_users -= 1;
    if (*sis).silence_users == 0 {
        dma_unmap_single(&mut (*(*sis).pci).dev, (*sis).silence_dma_addr, 4096, DMA_TO_DEVICE);
    }
}

unsafe extern "C" fn sis_free_voice(sis: *mut sis7019, voice: *mut voice) {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*sis).voice_lock, &mut flags);
    if !(*voice).timing.is_null() {
        __sis_unmap_silence(sis);
        (*(*voice).timing).flags &= !(VOICE_IN_USE | VOICE_SSO_TIMING | VOICE_SYNC_TIMING);
        (*voice).timing = ptr::null_mut();
    }
    (*voice).flags &= !(VOICE_IN_USE | VOICE_SSO_TIMING | VOICE_SYNC_TIMING);
    spin_unlock_irqrestore(&mut (*sis).voice_lock, flags);
}

unsafe extern "C" fn __sis_alloc_playback_voice(sis: *mut sis7019) -> *mut voice {
    /* Must hold the voice_lock on entry */
    let mut voice_ptr: *mut voice;
    let mut i: c_int = 0;
    while i < 64 {
        voice_ptr = &mut (*sis).voices[i as usize];
        if (*voice_ptr).flags & VOICE_IN_USE != 0 {
            i += 1;
            continue;
        }
        (*voice_ptr).flags |= VOICE_IN_USE;
        return voice_ptr;
    }
    ptr::null_mut()
}

unsafe extern "C" fn sis_alloc_playback_voice(sis: *mut sis7019) -> *mut voice {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*sis).voice_lock, &mut flags);
    let v = __sis_alloc_playback_voice(sis);
    spin_unlock_irqrestore(&mut (*sis).voice_lock, flags);
    v
}

unsafe extern "C" fn sis_alloc_timing_voice(substream: *mut snd_pcm_substream, hw_params: *mut snd_pcm_hw_params) -> c_int {
    let sis = snd_pcm_substream_chip(substream) as *mut sis7019;
    let runtime = (*substream).runtime;
    let voice_ptr = (*runtime).private_data as *mut voice;
    let period_size = params_period_size(hw_params);
    let buffer_size = params_buffer_size(hw_params);
    let needed = period_size != buffer_size && period_size != buffer_size / 2;
    if needed && (*voice_ptr).timing.is_null() {
        let mut flags: c_ulong = 0;
        spin_lock_irqsave(&mut (*sis).voice_lock, &mut flags);
        (*voice_ptr).timing = __sis_alloc_playback_voice(sis);
        if !(*voice_ptr).timing.is_null() {
            __sis_map_silence(sis);
        }
        spin_unlock_irqrestore(&mut (*sis).voice_lock, flags);
        if (*voice_ptr).timing.is_null() {
            return -ENOMEM;
        }
        (*(*voice_ptr).timing).substream = substream;
    } else if !needed && !(*voice_ptr).timing.is_null() {
        sis_free_voice(sis, voice_ptr);
        (*voice_ptr).timing = ptr::null_mut();
    }
    0
}

unsafe extern "C" fn sis_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let sis = snd_pcm_substream_chip(substream) as *mut sis7019;
    let runtime = (*substream).runtime;
    let voice_ptr = sis_alloc_playback_voice(sis);
    if voice_ptr.is_null() {
        return -EAGAIN;
    }
    (*voice_ptr).substream = substream;
    (*runtime).private_data = voice_ptr as *mut c_void;
    (*runtime).hw = sis_playback_hw_info;
    snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_PERIOD_SIZE, 9, 0xfff9);
    snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_BUFFER_SIZE, 9, 0xfff9);
    snd_pcm_set_sync(substream);
    0
}

unsafe extern "C" fn sis_substream_close(substream: *mut snd_pcm_substream) -> c_int {
    let sis = snd_pcm_substream_chip(substream) as *mut sis7019;
    let runtime = (*substream).runtime;
    let voice_ptr = (*runtime).private_data as *mut voice;
    sis_free_voice(sis, voice_ptr);
    0
}

unsafe extern "C" fn sis_pcm_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let voice_ptr = (*runtime).private_data as *mut voice;
    let ctrl_base = (*voice_ptr).ctrl_base;
    let wave_base = (*voice_ptr).wave_base;
    let mut format: u32 = 0;
    let dma_addr: u32;
    let mut control: u32;
    let mut sso_eso: u32;
    let delta: u32;
    let mut reg: u32;
    let leo: u16;
    if snd_pcm_format_width((*runtime).format) == 8 {
        format |= SIS_PLAY_DMA_FORMAT_8BIT;
    }
    if !snd_pcm_format_signed((*runtime).format) {
        format |= SIS_PLAY_DMA_FORMAT_UNSIGNED;
    }
    if (*runtime).channels == 1 {
        format |= SIS_PLAY_DMA_FORMAT_MONO;
    }
    dma_addr = (*runtime).dma_addr;
    leo = (*runtime).buffer_size - 1;
    control = leo as u32 | SIS_PLAY_DMA_LOOP | SIS_PLAY_DMA_INTR_AT_LEO;
    sso_eso = leo as u32;
    if (*runtime).period_size == (*runtime).buffer_size / 2 {
        control |= SIS_PLAY_DMA_INTR_AT_MLP;
    } else if (*runtime).period_size != (*runtime).buffer_size {
        (*voice_ptr).flags |= VOICE_SSO_TIMING;
        (*voice_ptr).sso = (*runtime).period_size as u32 - 1;
        (*voice_ptr).period_size = (*runtime).period_size;
        (*voice_ptr).buffer_size = (*runtime).buffer_size;
        control &= !SIS_PLAY_DMA_INTR_AT_LEO;
        control |= SIS_PLAY_DMA_INTR_AT_SSO;
        sso_eso |= ((*runtime).period_size as u32 - 1) << 16;
    }
    delta = sis_rate_to_delta((*runtime).rate);
    writel(format, ptr_add(ctrl_base, SIS_PLAY_DMA_FORMAT_CSO));
    writel(dma_addr, ptr_add(ctrl_base, SIS_PLAY_DMA_BASE));
    writel(control, ptr_add(ctrl_base, SIS_PLAY_DMA_CONTROL));
    writel(sso_eso, ptr_add(ctrl_base, SIS_PLAY_DMA_SSO_ESO));
    reg = 0;
    while reg < SIS_WAVE_SIZE {
        writel(0, ptr_add(wave_base, reg as usize));
        reg += 4;
    }
    writel(SIS_WAVE_GENERAL_WAVE_VOLUME, ptr_add(wave_base, SIS_WAVE_GENERAL));
    writel(delta << 16, ptr_add(wave_base, SIS_WAVE_GENERAL_ARTICULATION));
    writel(SIS_WAVE_CHANNEL_CONTROL_FIRST_SAMPLE | SIS_WAVE_CHANNEL_CONTROL_AMP_ENABLE | SIS_WAVE_CHANNEL_CONTROL_INTERPOLATE_ENABLE, ptr_add(wave_base, SIS_WAVE_CHANNEL_CONTROL));
    readl(ctrl_base);
    0
}

unsafe extern "C" fn sis_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let sis = snd_pcm_substream_chip(substream) as *mut sis7019;
    let io = (*sis).ioport;
    let mut s: *mut snd_pcm_substream = substream;
    let mut voice_ptr: *mut voice;
    let mut chip: *mut c_void;
    let starting: c_int;
    let mut record: u32 = 0;
    let mut play: [u32; 2] = [0, 0];
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_RESUME => starting = 1,
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_SUSPEND => starting = 0,
        _ => return -EINVAL,
    }
    /* Rust translation note: snd_pcm_group_for_each_entry() is a C iterator
     * macro; this preserves the single-entry body pending the kernel iterator.
     */
    loop {
        chip = snd_pcm_substream_chip(s);
        if chip != sis as *mut c_void {
            break;
        }
        voice_ptr = (*(*s).runtime).private_data as *mut voice;
        if (*voice_ptr).flags & VOICE_CAPTURE != 0 {
            record |= 1u32 << (*voice_ptr).num;
            voice_ptr = (*voice_ptr).timing;
        }
        if !voice_ptr.is_null() {
            play[((*voice_ptr).num / 32) as usize] |= 1u32 << ((*voice_ptr).num & 0x1f);
        }
        snd_pcm_trigger_done(s, substream);
        break;
    }
    if starting != 0 {
        if record != 0 { outl(record, io + SIS_RECORD_START_REG); }
        if play[0] != 0 { outl(play[0], io + SIS_PLAY_START_A_REG); }
        if play[1] != 0 { outl(play[1], io + SIS_PLAY_START_B_REG); }
    } else {
        if record != 0 { outl(record, io + SIS_RECORD_STOP_REG); }
        if play[0] != 0 { outl(play[0], io + SIS_PLAY_STOP_A_REG); }
        if play[1] != 0 { outl(play[1], io + SIS_PLAY_STOP_B_REG); }
    }
    0
}

unsafe extern "C" fn sis_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let runtime = (*substream).runtime;
    let voice_ptr = (*runtime).private_data as *mut voice;
    let mut cso = readl(ptr_add((*voice_ptr).ctrl_base, SIS_PLAY_DMA_FORMAT_CSO));
    cso &= 0xffff;
    cso as snd_pcm_uframes_t
}

unsafe extern "C" fn sis_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let sis = snd_pcm_substream_chip(substream) as *mut sis7019;
    let runtime = (*substream).runtime;
    let mut voice_ptr = &mut (*sis).capture_voice as *mut voice;
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*sis).voice_lock, &mut flags);
    if (*voice_ptr).flags & VOICE_IN_USE != 0 {
        voice_ptr = ptr::null_mut();
    } else {
        (*voice_ptr).flags |= VOICE_IN_USE;
    }
    spin_unlock_irqrestore(&mut (*sis).voice_lock, flags);
    if voice_ptr.is_null() {
        return -EAGAIN;
    }
    (*voice_ptr).substream = substream;
    (*runtime).private_data = voice_ptr as *mut c_void;
    (*runtime).hw = sis_capture_hw_info;
    (*runtime).hw.rates = (*(*sis).ac97[0]).rates[AC97_RATES_ADC];
    snd_pcm_limit_hw_rates(runtime);
    snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_PERIOD_SIZE, 9, 0xfff9);
    snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_BUFFER_SIZE, 9, 0xfff9);
    snd_pcm_set_sync(substream);
    0
}

unsafe extern "C" fn sis_capture_hw_params(substream: *mut snd_pcm_substream, hw_params: *mut snd_pcm_hw_params) -> c_int {
    let sis = snd_pcm_substream_chip(substream) as *mut sis7019;
    let mut rc = snd_ac97_set_rate((*sis).ac97[0], AC97_PCM_LR_ADC_RATE, params_rate(hw_params));
    if rc != 0 {
        return rc;
    }
    rc = sis_alloc_timing_voice(substream, hw_params);
    rc
}

unsafe extern "C" fn sis_prepare_timing_voice(voice_ptr: *mut voice, substream: *mut snd_pcm_substream) {
    let sis = snd_pcm_substream_chip(substream) as *mut sis7019;
    let runtime = (*substream).runtime;
    let timing = (*voice_ptr).timing;
    let play_base = (*timing).ctrl_base;
    let wave_base = (*timing).wave_base;
    let mut buffer_size: u16;
    let mut period_size: u16;
    let mut format: u32;
    let mut control: u32;
    let mut sso_eso: u32;
    let delta: u32;
    let mut vperiod: u32;
    let mut sso: u32;
    let mut reg: u32;
    buffer_size = (4096 / (*runtime).channels) as u16;
    buffer_size = (buffer_size as c_int / snd_pcm_format_size((*runtime).format, 1)) as u16;
    period_size = buffer_size;
    vperiod = (*runtime).period_size as u32 + 12;
    if vperiod > period_size as u32 {
        let mut tail: u16 = (vperiod % period_size as u32) as u16;
        let quarter_period: u16 = period_size / 4;
        if tail != 0 && tail < quarter_period {
            let loops: u16 = (vperiod / period_size as u32) as u16;
            tail = quarter_period - tail;
            tail += loops - 1;
            tail /= loops;
            period_size -= tail;
        }
        sso = period_size as u32 - 1;
    } else {
        period_size = (*runtime).period_size;
        sso = vperiod - 1;
        vperiod = 0;
    }
    (*timing).flags |= VOICE_SYNC_TIMING;
    (*timing).sync_base = (*voice_ptr).ctrl_base;
    (*timing).sync_cso = (*runtime).period_size;
    (*timing).sync_period_size = (*runtime).period_size;
    (*timing).sync_buffer_size = (*runtime).buffer_size;
    (*timing).period_size = period_size;
    (*timing).buffer_size = buffer_size;
    (*timing).sso = sso;
    (*timing).vperiod = vperiod;
    format = 0;
    if snd_pcm_format_width((*runtime).format) == 8 {
        format = SIS_CAPTURE_DMA_FORMAT_8BIT;
    }
    if (*runtime).channels == 1 {
        format |= SIS_CAPTURE_DMA_FORMAT_MONO;
    }
    control = (*timing).buffer_size as u32 - 1;
    control |= SIS_PLAY_DMA_LOOP | SIS_PLAY_DMA_INTR_AT_SSO;
    sso_eso = (*timing).buffer_size as u32 - 1;
    sso_eso |= (*timing).sso << 16;
    delta = sis_rate_to_delta((*runtime).rate);
    writel(format, ptr_add(play_base, SIS_PLAY_DMA_FORMAT_CSO));
    writel((*sis).silence_dma_addr, ptr_add(play_base, SIS_PLAY_DMA_BASE));
    writel(control, ptr_add(play_base, SIS_PLAY_DMA_CONTROL));
    writel(sso_eso, ptr_add(play_base, SIS_PLAY_DMA_SSO_ESO));
    reg = 0;
    while reg < SIS_WAVE_SIZE {
        writel(0, ptr_add(wave_base, reg as usize));
        reg += 4;
    }
    writel(SIS_WAVE_GENERAL_WAVE_VOLUME, ptr_add(wave_base, SIS_WAVE_GENERAL));
    writel(delta << 16, ptr_add(wave_base, SIS_WAVE_GENERAL_ARTICULATION));
    writel(SIS_WAVE_CHANNEL_CONTROL_FIRST_SAMPLE | SIS_WAVE_CHANNEL_CONTROL_AMP_ENABLE | SIS_WAVE_CHANNEL_CONTROL_INTERPOLATE_ENABLE, ptr_add(wave_base, SIS_WAVE_CHANNEL_CONTROL));
}

unsafe extern "C" fn sis_pcm_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let voice_ptr = (*runtime).private_data as *mut voice;
    let rec_base = (*voice_ptr).ctrl_base;
    let mut format: u32 = 0;
    let dma_addr: u32;
    let mut control: u32;
    let leo: u16;
    if snd_pcm_format_width((*runtime).format) == 8 {
        format = SIS_CAPTURE_DMA_FORMAT_8BIT;
    }
    if !snd_pcm_format_signed((*runtime).format) {
        format |= SIS_CAPTURE_DMA_FORMAT_UNSIGNED;
    }
    if (*runtime).channels == 1 {
        format |= SIS_CAPTURE_DMA_FORMAT_MONO;
    }
    dma_addr = (*runtime).dma_addr;
    leo = (*runtime).buffer_size - 1;
    control = leo as u32 | SIS_CAPTURE_DMA_LOOP;
    if !(*voice_ptr).timing.is_null() {
        sis_prepare_timing_voice(voice_ptr, substream);
    } else {
        control |= SIS_CAPTURE_DMA_INTR_AT_LEO;
        if (*runtime).period_size != (*runtime).buffer_size {
            control |= SIS_CAPTURE_DMA_INTR_AT_MLP;
        }
    }
    writel(format, ptr_add(rec_base, SIS_CAPTURE_DMA_FORMAT_CSO));
    writel(dma_addr, ptr_add(rec_base, SIS_CAPTURE_DMA_BASE));
    writel(control, ptr_add(rec_base, SIS_CAPTURE_DMA_CONTROL));
    readl(rec_base);
    0
}

static sis_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(sis_playback_open),
    close: Some(sis_substream_close),
    hw_params: None,
    prepare: Some(sis_pcm_playback_prepare),
    trigger: Some(sis_pcm_trigger),
    pointer: Some(sis_pcm_pointer),
};

static sis_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(sis_capture_open),
    close: Some(sis_substream_close),
    hw_params: Some(sis_capture_hw_params),
    prepare: Some(sis_pcm_capture_prepare),
    trigger: Some(sis_pcm_trigger),
    pointer: Some(sis_pcm_pointer),
};

unsafe extern "C" fn sis_pcm_create(sis: *mut sis7019) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let rc = snd_pcm_new((*sis).card, b"SiS7019\0".as_ptr() as *const c_char, 0, 64, 1, &mut pcm);
    if rc != 0 {
        return rc;
    }
    (*pcm).private_data = sis as *mut c_void;
    strscpy((*pcm).name.as_mut_ptr(), b"SiS7019\0".as_ptr() as *const c_char);
    (*sis).pcm = pcm;
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &sis_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &sis_capture_ops);
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &mut (*(*sis).pci).dev, 64 * 1024, 128 * 1024);
    0
}

unsafe extern "C" fn sis_ac97_rw(sis: *mut sis7019, codec: c_int, cmd: u32) -> c_ushort {
    let io = (*sis).ioport;
    let mut val: c_ushort = 0xffff;
    let mut status: u16;
    let rdy: u16;
    let mut count: c_int;
    let codec_ready: [u16; 3] = [SIS_AC97_STATUS_CODEC_READY, SIS_AC97_STATUS_CODEC2_READY, SIS_AC97_STATUS_CODEC3_READY];
    rdy = codec_ready[codec as usize];
    mutex_lock(&mut (*sis).ac97_mutex);
    count = 0xffff;
    while inw(io + SIS_AC97_SEMA) & SIS_AC97_SEMA_BUSY != 0 && { count -= 1; count != 0 } {
        udelay(1);
    }
    if count == 0 {
        mutex_unlock(&mut (*sis).ac97_mutex);
        dev_err(&mut (*(*sis).pci).dev, b"ac97 codec %d timeout cmd 0x%08x\n\0".as_ptr() as *const c_char, codec, cmd);
        return val;
    }
    count = 0xffff;
    loop {
        status = inw(io + SIS_AC97_STATUS);
        if status & rdy != 0 && status & SIS_AC97_STATUS_BUSY == 0 {
            break;
        }
        udelay(1);
        count -= 1;
        if count == 0 {
            outl(SIS_AC97_SEMA_RELEASE, io + SIS_AC97_SEMA);
            mutex_unlock(&mut (*sis).ac97_mutex);
            dev_err(&mut (*(*sis).pci).dev, b"ac97 codec %d timeout cmd 0x%08x\n\0".as_ptr() as *const c_char, codec, cmd);
            return val;
        }
    }
    outl(cmd, io + SIS_AC97_CMD);
    udelay(10);
    count = 0xffff;
    while inw(io + SIS_AC97_STATUS) & SIS_AC97_STATUS_BUSY != 0 && { count -= 1; count != 0 } {
        udelay(1);
    }
    val = (inl(io + SIS_AC97_CMD) >> 16) as c_ushort;
    outl(SIS_AC97_SEMA_RELEASE, io + SIS_AC97_SEMA);
    mutex_unlock(&mut (*sis).ac97_mutex);
    if count == 0 {
        dev_err(&mut (*(*sis).pci).dev, b"ac97 codec %d timeout cmd 0x%08x\n\0".as_ptr() as *const c_char, codec, cmd);
    }
    val
}

unsafe extern "C" fn sis_ac97_write(ac97: *mut snd_ac97, reg: c_ushort, val: c_ushort) {
    let cmd: [u32; 3] = [SIS_AC97_CMD_CODEC_WRITE, SIS_AC97_CMD_CODEC2_WRITE, SIS_AC97_CMD_CODEC3_WRITE];
    sis_ac97_rw((*ac97).private_data as *mut sis7019, (*ac97).num, ((val as u32) << 16) | ((reg as u32) << 8) | cmd[(*ac97).num as usize]);
}

unsafe extern "C" fn sis_ac97_read(ac97: *mut snd_ac97, reg: c_ushort) -> c_ushort {
    let cmd: [u32; 3] = [SIS_AC97_CMD_CODEC_READ, SIS_AC97_CMD_CODEC2_READ, SIS_AC97_CMD_CODEC3_READ];
    sis_ac97_rw((*ac97).private_data as *mut sis7019, (*ac97).num, ((reg as u32) << 8) | cmd[(*ac97).num as usize])
}

unsafe extern "C" fn sis_mixer_create(sis: *mut sis7019) -> c_int {
    let mut bus: *mut snd_ac97_bus = ptr::null_mut();
    let mut ac97: snd_ac97_template = core::mem::zeroed();
    let ops = snd_ac97_bus_ops { write: Some(sis_ac97_write), read: Some(sis_ac97_read) };
    let mut rc: c_int;
    ac97.private_data = sis as *mut c_void;
    rc = snd_ac97_bus((*sis).card, 0, &ops, ptr::null_mut(), &mut bus);
    if rc == 0 && (*sis).codecs_present & SIS_PRIMARY_CODEC_PRESENT != 0 {
        rc = snd_ac97_mixer(bus, &mut ac97, &mut (*sis).ac97[0]);
    }
    ac97.num = 1;
    if rc == 0 && (*sis).codecs_present & SIS_SECONDARY_CODEC_PRESENT != 0 {
        rc = snd_ac97_mixer(bus, &mut ac97, &mut (*sis).ac97[1]);
    }
    ac97.num = 2;
    if rc == 0 && (*sis).codecs_present & SIS_TERTIARY_CODEC_PRESENT != 0 {
        rc = snd_ac97_mixer(bus, &mut ac97, &mut (*sis).ac97[2]);
    }
    rc
}

unsafe extern "C" fn sis_chip_free(card: *mut snd_card) {
    let sis = (*card).private_data as *mut sis7019;
    outl(SIS_GCR_SOFTWARE_RESET, (*sis).ioport + SIS_GCR);
    udelay(25);
    outl(0, (*sis).ioport + SIS_GCR);
    outl(0, (*sis).ioport + SIS_GIER);
    if (*sis).irq >= 0 {
        free_irq((*sis).irq, sis as *mut c_void);
    }
}

unsafe extern "C" fn sis_chip_init(sis: *mut sis7019) -> c_int {
    let io = (*sis).ioport;
    let ioaddr = (*sis).ioaddr;
    let timeout: c_ulong;
    let mut status: u16;
    let mut count: c_int;
    let mut i: c_int;
    outl(SIS_GCR_SOFTWARE_RESET, io + SIS_GCR);
    udelay(25);
    outl(0, io + SIS_GCR);
    count = 0xffff;
    while inw(io + SIS_AC97_SEMA) & SIS_AC97_SEMA_BUSY != 0 && { count -= 1; count != 0 } {
        udelay(1);
    }
    if count == 0 {
        return -EIO;
    }
    outl(SIS_AC97_CMD_CODEC_COLD_RESET, io + SIS_AC97_CMD);
    udelay(250);
    count = 0xffff;
    while inw(io + SIS_AC97_STATUS) & SIS_AC97_STATUS_BUSY != 0 && { count -= 1; count != 0 } {
        udelay(1);
    }
    outl(SIS_AC97_SEMA_RELEASE, io + SIS_AC97_SEMA);
    if count == 0 {
        return -EIO;
    }
    (*sis).codecs_present = 0;
    timeout = msecs_to_jiffies(500) + jiffies;
    while time_before_eq(jiffies, timeout) {
        status = inl(io + SIS_AC97_STATUS) as u16;
        if status & SIS_AC97_STATUS_CODEC_READY != 0 { (*sis).codecs_present |= SIS_PRIMARY_CODEC_PRESENT; }
        if status & SIS_AC97_STATUS_CODEC2_READY != 0 { (*sis).codecs_present |= SIS_SECONDARY_CODEC_PRESENT; }
        if status & SIS_AC97_STATUS_CODEC3_READY != 0 { (*sis).codecs_present |= SIS_TERTIARY_CODEC_PRESENT; }
        if (*sis).codecs_present == codecs {
            break;
        }
        msleep(1);
    }
    if (*sis).codecs_present == 0 {
        dev_err(&mut (*(*sis).pci).dev, b"could not find any codecs\n\0".as_ptr() as *const c_char);
        return -EIO;
    }
    if (*sis).codecs_present != codecs {
        dev_warn(&mut (*(*sis).pci).dev, b"missing codecs, found %0x, expected %0x\n\0".as_ptr() as *const c_char, (*sis).codecs_present, codecs);
    }
    outl(SIS_AC97_CONF_AUDIO_ALIVE, io + SIS_AC97_CONF);
    outl(SIS_AC97_CONF_AUDIO_ALIVE | SIS_AC97_CONF_PCM_LR_ENABLE | SIS_AC97_CONF_PCM_CAP_MIC_ENABLE | SIS_AC97_CONF_PCM_CAP_LR_ENABLE | SIS_AC97_CONF_CODEC_VRA_ENABLE, io + SIS_AC97_CONF);
    outl(0, io + SIS_AC97_PSR);
    outl(SIS_DMA_CSR_PCI_SETTINGS, io + SIS_DMA_CSR);
    outl(0, io + SIS_PLAY_SYNC_GROUP_A);
    outl(0, io + SIS_PLAY_SYNC_GROUP_B);
    outl(0, io + SIS_PLAY_SYNC_GROUP_C);
    outl(0, io + SIS_PLAY_SYNC_GROUP_D);
    outl(0, io + SIS_MIXER_SYNC_GROUP);
    i = 0;
    while i < 64 {
        writel(i as u32, SIS_MIXER_START_ADDR(ioaddr, i));
        writel(SIS_MIXER_RIGHT_NO_ATTEN | SIS_MIXER_LEFT_NO_ATTEN | SIS_MIXER_DEST_0, SIS_MIXER_ADDR(ioaddr, i));
        i += 1;
    }
    outl(0xffff0000, io + SIS_WEVCR);
    outl(0, io + SIS_WECCR);
    outl(SIS_GIER_AUDIO_PLAY_DMA_IRQ_ENABLE | SIS_GIER_AUDIO_RECORD_DMA_IRQ_ENABLE, io + SIS_GIER);
    0
}

unsafe extern "C" fn sis_suspend(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let sis = (*card).private_data as *mut sis7019;
    let mut ioaddr = (*sis).ioaddr;
    let mut i: c_int;
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    if (*sis).codecs_present & SIS_PRIMARY_CODEC_PRESENT != 0 { snd_ac97_suspend((*sis).ac97[0]); }
    if (*sis).codecs_present & SIS_SECONDARY_CODEC_PRESENT != 0 { snd_ac97_suspend((*sis).ac97[1]); }
    if (*sis).codecs_present & SIS_TERTIARY_CODEC_PRESENT != 0 { snd_ac97_suspend((*sis).ac97[2]); }
    if (*sis).irq >= 0 {
        free_irq((*sis).irq, sis as *mut c_void);
        (*sis).irq = -1;
    }
    i = 0;
    while i < 4 {
        memcpy_fromio((*sis).suspend_state[i as usize], ioaddr, 4096);
        ioaddr = ptr_add(ioaddr, 4096);
        i += 1;
    }
    0
}

unsafe extern "C" fn sis_resume(dev: *mut device) -> c_int {
    let pci = to_pci_dev(dev);
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let sis = (*card).private_data as *mut sis7019;
    let mut ioaddr = (*sis).ioaddr;
    let mut i: c_int;
    if sis_chip_init(sis) != 0 {
        dev_err(&mut (*pci).dev, b"unable to re-init controller\n\0".as_ptr() as *const c_char);
        snd_card_disconnect(card);
        return -EIO;
    }
    if request_irq((*pci).irq, sis_interrupt, IRQF_SHARED, KBUILD_MODNAME, sis as *mut c_void) != 0 {
        dev_err(&mut (*pci).dev, b"unable to regain IRQ %d\n\0".as_ptr() as *const c_char, (*pci).irq);
        snd_card_disconnect(card);
        return -EIO;
    }
    i = 0;
    while i < 4 {
        memcpy_toio(ioaddr, (*sis).suspend_state[i as usize], 4096);
        ioaddr = ptr_add(ioaddr, 4096);
        i += 1;
    }
    memset((*sis).suspend_state[0], 0, 4096);
    (*sis).irq = (*pci).irq;
    if (*sis).codecs_present & SIS_PRIMARY_CODEC_PRESENT != 0 { snd_ac97_resume((*sis).ac97[0]); }
    if (*sis).codecs_present & SIS_SECONDARY_CODEC_PRESENT != 0 { snd_ac97_resume((*sis).ac97[1]); }
    if (*sis).codecs_present & SIS_TERTIARY_CODEC_PRESENT != 0 { snd_ac97_resume((*sis).ac97[2]); }
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}

static sis_pm: dev_pm_ops = dev_pm_ops { _private: [] };

unsafe extern "C" fn sis_alloc_suspend(sis: *mut sis7019) -> c_int {
    let mut i: c_int = 0;
    while i < SIS_SUSPEND_PAGES as c_int {
        (*sis).suspend_state[i as usize] = devm_kmalloc(&mut (*(*sis).pci).dev, 4096, GFP_KERNEL);
        if (*sis).suspend_state[i as usize].is_null() {
            return -ENOMEM;
        }
        i += 1;
    }
    memset((*sis).suspend_state[0], 0, 4096);
    0
}

unsafe extern "C" fn sis_chip_create(card: *mut snd_card, pci: *mut pci_dev) -> c_int {
    let sis = (*card).private_data as *mut sis7019;
    let mut voice_ptr: *mut voice;
    let mut rc: c_int;
    let mut i: c_int;
    rc = pcim_enable_device(pci);
    if rc != 0 { return rc; }
    rc = dma_set_mask(&mut (*pci).dev, DMA_BIT_MASK(30));
    if rc < 0 {
        dev_err(&mut (*pci).dev, b"architecture does not support 30-bit PCI busmaster DMA\0".as_ptr() as *const c_char);
        return -ENXIO;
    }
    mutex_init(&mut (*sis).ac97_mutex);
    spin_lock_init(&mut (*sis).voice_lock);
    (*sis).card = card;
    (*sis).pci = pci;
    (*sis).irq = -1;
    (*sis).ioport = pci_resource_start(pci, 0);
    rc = pcim_request_region(pci, 0, b"SiS7019\0".as_ptr() as *const c_char);
    if rc != 0 {
        dev_err(&mut (*pci).dev, b"unable request I/O region\n\0".as_ptr() as *const c_char);
        return rc;
    }
    (*sis).ioaddr = pcim_iomap_region(pci, 1, b"SiS7019\0".as_ptr() as *const c_char);
    if IS_ERR((*sis).ioaddr) {
        dev_err(&mut (*pci).dev, b"unable to remap MMIO, aborting\n\0".as_ptr() as *const c_char);
        return PTR_ERR((*sis).ioaddr);
    }
    rc = sis_alloc_suspend(sis);
    if rc < 0 {
        dev_err(&mut (*pci).dev, b"unable to allocate state storage\n\0".as_ptr() as *const c_char);
        return rc;
    }
    rc = sis_chip_init(sis);
    if rc != 0 { return rc; }
    (*card).private_free = Some(sis_chip_free);
    rc = request_irq((*pci).irq, sis_interrupt, IRQF_SHARED, KBUILD_MODNAME, sis as *mut c_void);
    if rc != 0 {
        dev_err(&mut (*pci).dev, b"unable to allocate irq %d\n\0".as_ptr() as *const c_char, (*sis).irq);
        return rc;
    }
    (*sis).irq = (*pci).irq;
    (*card).sync_irq = (*sis).irq;
    pci_set_master(pci);
    i = 0;
    while i < 64 {
        voice_ptr = &mut (*sis).voices[i as usize];
        (*voice_ptr).num = i;
        (*voice_ptr).ctrl_base = SIS_PLAY_DMA_ADDR((*sis).ioaddr, i);
        (*voice_ptr).wave_base = SIS_WAVE_ADDR((*sis).ioaddr, i);
        i += 1;
    }
    voice_ptr = &mut (*sis).capture_voice;
    (*voice_ptr).flags = VOICE_CAPTURE;
    (*voice_ptr).num = SIS_CAPTURE_CHAN_AC97_PCM_IN;
    (*voice_ptr).ctrl_base = SIS_CAPTURE_DMA_ADDR((*sis).ioaddr, (*voice_ptr).num);
    0
}

unsafe extern "C" fn __snd_sis7019_probe(pci: *mut pci_dev, _pci_id: *const pci_device_id) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let mut sis: *mut sis7019;
    let mut rc: c_int;
    if !enable {
        return -ENOENT;
    }
    codecs &= SIS_PRIMARY_CODEC_PRESENT | SIS_SECONDARY_CODEC_PRESENT | SIS_TERTIARY_CODEC_PRESENT;
    if codecs == 0 {
        codecs = SIS_PRIMARY_CODEC_PRESENT;
    }
    rc = snd_devm_card_new(&mut (*pci).dev, index, id, THIS_MODULE, size_of::<sis7019>(), &mut card);
    if rc < 0 { return rc; }
    strscpy((*card).driver.as_mut_ptr(), b"SiS7019\0".as_ptr() as *const c_char);
    strscpy((*card).shortname.as_mut_ptr(), b"SiS7019\0".as_ptr() as *const c_char);
    rc = sis_chip_create(card, pci);
    if rc != 0 { return rc; }
    sis = (*card).private_data as *mut sis7019;
    rc = sis_mixer_create(sis);
    if rc != 0 { return rc; }
    rc = sis_pcm_create(sis);
    if rc != 0 { return rc; }
    snprintf((*card).longname.as_mut_ptr(), (*card).longname.len(), b"%s Audio Accelerator with %s at 0x%lx, irq %d\0".as_ptr() as *const c_char, (*card).shortname.as_ptr(), snd_ac97_get_short_name((*sis).ac97[0]), (*sis).ioport, (*sis).irq);
    rc = snd_card_register(card);
    if rc != 0 { return rc; }
    pci_set_drvdata(pci, card as *mut c_void);
    0
}

unsafe extern "C" fn snd_sis7019_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    snd_card_free_on_error(&mut (*pci).dev, __snd_sis7019_probe(pci, pci_id))
}

static mut sis7019_driver: pci_driver = pci_driver {
    name: ptr::null(),
    id_table: snd_sis7019_ids.as_ptr(),
    probe: Some(snd_sis7019_probe),
    driver: device_driver {
        pm: &sis_pm,
    },
};

/* module_pci_driver(sis7019_driver); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
