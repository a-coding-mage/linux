// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Routines for control of CS4231(A)/CS4232/InterWave & compatible chips
 *
 *  Bugs:
 *     - sometimes record brokes playback with WSS portion of
 *       Yamaha OPL3-SA3 chip
 *     - CS4231 (GUS MAX) - still trouble with occasional noises
 *			  - broken initialization?
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type snd_pcm_format_t = c_int;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;
type size_t = usize;

#[repr(C)]
pub struct snd_wss {
    pub port: c_ulong,
    pub cport: c_ulong,
    pub irq: c_int,
    pub dma1: c_int,
    pub dma2: c_int,
    pub p_dma_size: c_uint,
    pub c_dma_size: c_uint,
    pub hardware: c_uint,
    pub hwshare: c_uint,
    pub single_dma: c_int,
    pub thinkpad_flag: c_int,
    pub mce_bit: u8,
    pub calibrate_mute: c_int,
    pub mode: c_uint,
    pub image: [u8; 32],
    pub eimage: [u8; 32],
    pub reg_lock: spinlock_t,
    pub mce_mutex: mutex,
    pub open_mutex: mutex,
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub timer: *mut snd_timer,
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub res_port: *mut resource,
    pub res_cport: *mut resource,
    pub dma_private_data: *mut c_void,
    pub rate_constraint: Option<unsafe extern "C" fn(*mut snd_pcm_runtime) -> c_int>,
    pub set_playback_format: Option<unsafe extern "C" fn(*mut snd_wss, *mut snd_pcm_hw_params, u8)>,
    pub set_capture_format: Option<unsafe extern "C" fn(*mut snd_wss, *mut snd_pcm_hw_params, u8)>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_wss, c_uint, c_int)>,
    pub claim_dma: Option<unsafe extern "C" fn(*mut snd_wss, *mut c_void, c_int) -> c_int>,
    pub release_dma: Option<unsafe extern "C" fn(*mut snd_wss, *mut c_void, c_int)>,
    /* CONFIG_PM */
    pub suspend: Option<unsafe extern "C" fn(*mut snd_wss)>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_wss)>,
}

#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct resource { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub number: c_int,
    pub sync_irq: c_int,
    pub shortname: *const c_char,
    pub driver: *const c_char,
    pub mixername: *mut c_char,
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
    pub buffer_bytes_max: size_t,
    pub period_bytes_min: size_t,
    pub period_bytes_max: size_t,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub fifo_size: size_t,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
    pub dma_addr: c_ulong,
    pub overrange: c_uint,
}

#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm {
    pub private_data: *mut c_void,
    pub info_flags: c_uint,
    pub name: *mut c_char,
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

#[repr(C)]
pub struct snd_timer {
    pub sticks: c_uint,
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_timer)>,
    pub hw: snd_timer_hardware,
    pub name: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_timer_hardware {
    pub flags: c_uint,
    pub resolution: c_uint,
    pub ticks: c_uint,
    pub open: Option<unsafe extern "C" fn(*mut snd_timer) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_timer) -> c_int>,
    pub c_resolution: Option<unsafe extern "C" fn(*mut snd_timer) -> c_ulong>,
    pub start: Option<unsafe extern "C" fn(*mut snd_timer) -> c_int>,
    pub stop: Option<unsafe extern "C" fn(*mut snd_timer) -> c_int>,
}

#[repr(C)]
pub struct snd_timer_id {
    pub dev_class: c_int,
    pub dev_sclass: c_int,
    pub card: c_int,
    pub device: c_int,
    pub subdevice: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *const c_uint,
    pub mask: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}

#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)] pub union snd_ctl_elem_info_value { pub integer: snd_ctl_elem_info_integer }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_info_integer { pub min: c_long, pub max: c_long }
type c_long = isize;

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)] pub union snd_ctl_elem_value_value { pub integer: snd_ctl_elem_value_integer, pub enumerated: snd_ctl_elem_value_enumerated }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_integer { pub value: [c_long; 128] }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_enumerated { pub item: [c_uint; 128] }

const EINVAL: c_int = 22;
const EAGAIN: c_int = 11;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const GFP_KERNEL: c_uint = 0;
const IRQ_HANDLED: irqreturn_t = 1;

const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 1;
const SNDRV_PCM_TRIGGER_STOP: c_int = 2;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 3;
const SNDRV_PCM_FORMAT_MU_LAW: snd_pcm_format_t = 1;
const SNDRV_PCM_FORMAT_A_LAW: snd_pcm_format_t = 2;
const SNDRV_PCM_FORMAT_S16_LE: snd_pcm_format_t = 3;
const SNDRV_PCM_FORMAT_S16_BE: snd_pcm_format_t = 4;
const SNDRV_PCM_FORMAT_IMA_ADPCM: snd_pcm_format_t = 5;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;

const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 1;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 2;
const SNDRV_PCM_INFO_SYNC_START: c_uint = 1 << 3;
const SNDRV_PCM_INFO_RESUME: c_uint = 1 << 4;
const SNDRV_PCM_INFO_HALF_DUPLEX: c_uint = 1 << 5;
const SNDRV_PCM_INFO_JOINT_DUPLEX: c_uint = 1 << 6;
const SNDRV_PCM_FMTBIT_MU_LAW: c_ulong = 1 << 0;
const SNDRV_PCM_FMTBIT_A_LAW: c_ulong = 1 << 1;
const SNDRV_PCM_FMTBIT_IMA_ADPCM: c_ulong = 1 << 2;
const SNDRV_PCM_FMTBIT_U8: c_ulong = 1 << 3;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << 4;
const SNDRV_PCM_FMTBIT_S16_BE: c_ulong = 1 << 5;
const SNDRV_PCM_RATE_KNOT: c_uint = 1 << 0;
const SNDRV_PCM_RATE_8000_48000: c_uint = 1 << 1;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const DMA_MODE_WRITE: c_int = 0x48;
const DMA_MODE_READ: c_int = 0x44;
const DMA_AUTOINIT: c_int = 0x10;
const SNDRV_TIMER_HW_AUTO: c_uint = 1;
const SNDRV_TIMER_CLASS_CARD: c_int = 0;
const SNDRV_TIMER_SCLASS_NONE: c_int = 0;
const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_uint = 1;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 2;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;

const CS4231_XTAL2: u8 = 0x01;
const CS4231_XTAL1: u8 = 0x00;
const CS4231_INIT: u8 = 0x80;
const CS4231_MCE: u8 = 0x40;
const CS4231_AUTOCALIB: u8 = 0x08;
const CS4231_MODE2: u8 = 0x40;
const CS4231_IW_MODE3: u8 = 0x6c;
const CS4231_4236_MODE3: u8 = 0xe0;
const CS4231_CALIB_IN_PROGRESS: u8 = 0x20;
const CS4231_PLAYBACK_ENABLE: u8 = 0x01;
const CS4231_RECORD_ENABLE: u8 = 0x02;
const CS4231_PLAYBACK_PIO: u8 = 0x40;
const CS4231_RECORD_PIO: u8 = 0x80;
const CS4231_CALIB_MODE: u8 = 0x18;
const CS4231_TIMER_ENABLE: u8 = 0x40;
const CS4231_STEREO: u8 = 0x10;
const CS4231_LINEAR_8: u8 = 0x00;
const CS4231_ULAW_8: u8 = 0x20;
const CS4231_ALAW_8: u8 = 0x60;
const CS4231_LINEAR_16: u8 = 0x40;
const CS4231_LINEAR_16_BIG: u8 = 0xc0;
const CS4231_ADPCM_16: u8 = 0xa0;
const CS4231_SINGLE_DMA: u8 = 0x04;
const CS4231_IRQ_ENABLE: u8 = 0x02;
const CS4231_PLAYBACK_IRQ: u8 = 0x10;
const CS4231_RECORD_IRQ: u8 = 0x20;
const CS4231_TIMER_IRQ: u8 = 0x40;
const CS4231_ALL_IRQS: u8 = CS4231_PLAYBACK_IRQ | CS4231_RECORD_IRQ | CS4231_TIMER_IRQ;
const CS4231_MIXS_ALL: u8 = 0xc0;
const CS4231_ENABLE_MIC_GAIN: u8 = 0x20;

const CS4231_LEFT_INPUT: usize = 0x00;
const CS4231_RIGHT_INPUT: usize = 0x01;
const CS4231_AUX1_LEFT_INPUT: usize = 0x02;
const CS4231_AUX1_RIGHT_INPUT: usize = 0x03;
const CS4231_AUX2_LEFT_INPUT: usize = 0x04;
const CS4231_AUX2_RIGHT_INPUT: usize = 0x05;
const CS4231_LEFT_OUTPUT: usize = 0x06;
const CS4231_RIGHT_OUTPUT: usize = 0x07;
const CS4231_PLAYBK_FORMAT: usize = 0x08;
const CS4231_IFACE_CTRL: usize = 0x09;
const CS4231_PIN_CTRL: usize = 0x0a;
const CS4231_TEST_INIT: usize = 0x0b;
const CS4231_MISC_INFO: usize = 0x0c;
const CS4231_LOOPBACK: usize = 0x0d;
const CS4231_TIMER_HIGH: usize = 0x0e;
const CS4231_TIMER_LOW: usize = 0x0f;
const CS4231_ALT_FEATURE_1: usize = 0x10;
const CS4231_ALT_FEATURE_2: usize = 0x11;
const CS4231_LEFT_LINE_IN: usize = 0x12;
const CS4231_RIGHT_LINE_IN: usize = 0x13;
const CS4231_VERSION: usize = 0x19;
const CS4231_MONO_CTRL: usize = 0x1a;
const CS4231_REC_FORMAT: usize = 0x1c;
const CS4231_EXT_REG: usize = 0x17;
const CS4231_PLY_LWR_CNT: usize = 0x0f;
const CS4231_PLY_UPR_CNT: usize = 0x0e;
const CS4231_REC_LWR_CNT: usize = 0x1f;
const CS4231_REC_UPR_CNT: usize = 0x1e;
const CS4231_LEFT_MIC_INPUT: usize = 0x16;
const CS4231_RIGHT_MIC_INPUT: usize = 0x17;
const CS4231_LINE_LEFT_OUTPUT: usize = 0x19;
const CS4231_LINE_RIGHT_OUTPUT: usize = 0x1b;
const CS4231_IRQ_STATUS: usize = 0x18;
const AD1845_UPR_FREQ_SEL: usize = 0x16;
const AD1845_LWR_FREQ_SEL: usize = 0x17;
const AD1845_PWR_DOWN: usize = 0x1b;
const CS4236_VERSION: u8 = 0x01;

const WSS_HW_DETECT: c_uint = 0x0000;
const WSS_HW_DETECT3: c_uint = 0x0001;
const WSS_HW_TYPE_MASK: c_uint = 0x00ff;
const WSS_HW_AD1847: c_uint = 0x0002;
const WSS_HW_AD1848: c_uint = 0x0004;
const WSS_HW_CS4248: c_uint = 0x0008;
const WSS_HW_CMI8330: c_uint = 0x0010;
const WSS_HW_CS4231: c_uint = 0x0020;
const WSS_HW_CS4231A: c_uint = 0x0040;
const WSS_HW_CS4232: c_uint = 0x0080;
const WSS_HW_CS4232A: c_uint = 0x0100;
const WSS_HW_CS4235: c_uint = 0x0200;
const WSS_HW_CS4236: c_uint = 0x0400;
const WSS_HW_CS4236B: c_uint = 0x0800;
const WSS_HW_CS4237B: c_uint = 0x1000;
const WSS_HW_CS4238B: c_uint = 0x2000;
const WSS_HW_CS4239: c_uint = 0x4000;
const WSS_HW_INTERWAVE: c_uint = 0x8000;
const WSS_HW_OPL3SA2: c_uint = 0x10000;
const WSS_HW_AD1845: c_uint = 0x20000;
const WSS_HW_OPTI93X: c_uint = 0x40000;
const WSS_HW_THINKPAD: c_uint = 0x80000;
const WSS_HW_AD1848_MASK: c_uint = WSS_HW_AD1847 | WSS_HW_AD1848 | WSS_HW_CS4248;
const WSS_HW_CS4231_MASK: c_uint = WSS_HW_CS4231 | WSS_HW_CS4231A;
const WSS_HW_CS4232_MASK: c_uint = WSS_HW_CS4232 | WSS_HW_CS4232A;
const WSS_HW_CS4236B_MASK: c_uint = WSS_HW_CS4236B | WSS_HW_CS4237B | WSS_HW_CS4238B | WSS_HW_CS4239;
const WSS_HWSHARE_IRQ: c_uint = 1;
const WSS_HWSHARE_DMA1: c_uint = 2;
const WSS_HWSHARE_DMA2: c_uint = 4;
const WSS_MODE_PLAY: c_uint = 1;
const WSS_MODE_RECORD: c_uint = 2;
const WSS_MODE_TIMER: c_uint = 4;
const WSS_MODE_OPEN: c_uint = WSS_MODE_PLAY | WSS_MODE_RECORD | WSS_MODE_TIMER;
const AD1848_THINKPAD_CTL_PORT1: c_ulong = 0x15e8;
const AD1848_THINKPAD_CTL_PORT2: c_ulong = 0x15e9;
const AD1848_THINKPAD_CS4248_ENABLE_BIT: c_int = 0x02;

const fn CS4231P(x: usize) -> u8 { x as u8 }
const fn CS4236_REG(x: u8) -> usize { (x & 0x1f) as usize }

unsafe extern "C" {
    static mut jiffies: c_ulong;
    fn outb(value: u8, port: c_ulong);
    fn inb(port: c_ulong) -> u8;
    fn udelay(usecs: c_ulong);
    fn mdelay(msecs: c_ulong);
    fn msleep(msecs: c_uint);
    fn cond_resched();
    fn mb();
    fn msecs_to_jiffies(msecs: c_uint) -> c_ulong;
    fn time_after(a: c_ulong, b: c_ulong) -> bool;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut snd_wss;
    fn snd_pcm_group_for_each_entry_next(s: *mut *mut snd_pcm_substream, substream: *mut snd_pcm_substream) -> bool;
    fn snd_pcm_trigger_done(s: *mut snd_pcm_substream, master: *mut snd_pcm_substream);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_timer_chip(timer: *mut snd_timer) -> *mut snd_wss;
    fn snd_dma_program(dma: c_int, addr: c_ulong, size: c_uint, mode: c_int);
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn snd_timer_interrupt(timer: *mut snd_timer, ticks: c_uint);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_dma_pointer(dma: c_int, size: c_uint) -> size_t;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: size_t) -> snd_pcm_uframes_t;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_request_region(dev: *mut device, start: c_ulong, n: c_ulong, name: *const c_char) -> *mut resource;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn snd_devm_request_dma(dev: *mut device, dma: c_int, name: *const c_char) -> c_int;
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, ty: c_int, dev: *mut device, min: size_t, max: size_t);
    fn snd_pcm_limit_isa_dma_size(dma: c_int, max: *mut size_t);
    fn snd_pcm_set_sync(substream: *mut snd_pcm_substream);
    fn snd_timer_new(card: *mut snd_card, id: *const c_char, tid: *mut snd_timer_id, rtimer: *mut *mut snd_timer) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> size_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn snd_ctl_enum_info(uinfo: *mut snd_ctl_elem_info, channels: c_uint, items: c_uint, texts: *const *const c_char) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_wss;
    fn snd_BUG_ON(cond: bool) -> bool;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut c_void) -> c_int;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut c_void;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn mutex_init(lock: *mut mutex);
}

static freq_bits: [u8; 14] = [
    0x00 | CS4231_XTAL2, 0x0e | CS4231_XTAL2, 0x00 | CS4231_XTAL1,
    0x0e | CS4231_XTAL1, 0x02 | CS4231_XTAL2, 0x02 | CS4231_XTAL1,
    0x04 | CS4231_XTAL2, 0x06 | CS4231_XTAL2, 0x04 | CS4231_XTAL1,
    0x06 | CS4231_XTAL1, 0x0c | CS4231_XTAL2, 0x08 | CS4231_XTAL2,
    0x0a | CS4231_XTAL2, 0x0c | CS4231_XTAL1,
];

static rates: [c_uint; 14] = [
    5510, 6620, 8000, 9600, 11025, 16000, 18900, 22050,
    27042, 32000, 33075, 37800, 44100, 48000,
];

static hw_constraints_rates: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: rates.len() as c_uint,
    list: rates.as_ptr(),
    mask: 0,
};

unsafe extern "C" fn snd_wss_xrate(runtime: *mut snd_pcm_runtime) -> c_int {
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &hw_constraints_rates)
}

static snd_wss_original_image: [u8; 32] = [
    0x00, 0x00, 0x9f, 0x9f, 0x9f, 0x9f, 0xbf, 0xbf,
    0x20, CS4231_AUTOCALIB, 0x00, 0x00, CS4231_MODE2, 0xfc, 0x00, 0x00,
    0x80, 0x01, 0x9f, 0x9f, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xcf, 0x00, 0x20, 0x00, 0x00, 0x00,
];

static snd_opti93x_original_image: [u8; 32] = [
    0x00, 0x00, 0x88, 0x88, 0x88, 0x88, 0x80, 0x80,
    0x00, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x00,
    0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x80, 0x80,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

#[inline]
unsafe fn wss_outb(chip: *mut snd_wss, offset: u8, val: u8) {
    outb(val, (*chip).port + offset as c_ulong);
}

#[inline]
unsafe fn wss_inb(chip: *mut snd_wss, offset: u8) -> u8 {
    inb((*chip).port + offset as c_ulong)
}

unsafe fn snd_wss_wait(chip: *mut snd_wss) {
    let mut timeout = 250;
    while timeout > 0 && (wss_inb(chip, CS4231P(0)) & CS4231_INIT) != 0 {
        udelay(100);
        timeout -= 1;
    }
}

unsafe fn snd_wss_dout(chip: *mut snd_wss, reg: u8, value: u8) {
    let mut timeout = 250;
    while timeout > 0 && (wss_inb(chip, CS4231P(0)) & CS4231_INIT) != 0 {
        udelay(10);
        timeout -= 1;
    }
    wss_outb(chip, CS4231P(0), (*chip).mce_bit | reg);
    wss_outb(chip, CS4231P(1), value);
    mb();
}

#[no_mangle]
pub unsafe extern "C" fn snd_wss_out(chip: *mut snd_wss, reg: u8, value: u8) {
    snd_wss_wait(chip);
    wss_outb(chip, CS4231P(0), (*chip).mce_bit | reg);
    wss_outb(chip, CS4231P(1), value);
    (*chip).image[reg as usize] = value;
    mb();
}

#[no_mangle]
pub unsafe extern "C" fn snd_wss_in(chip: *mut snd_wss, reg: u8) -> u8 {
    snd_wss_wait(chip);
    wss_outb(chip, CS4231P(0), (*chip).mce_bit | reg);
    mb();
    wss_inb(chip, CS4231P(1))
}

#[no_mangle]
pub unsafe extern "C" fn snd_cs4236_ext_out(chip: *mut snd_wss, reg: u8, val: u8) {
    wss_outb(chip, CS4231P(0), (*chip).mce_bit | 0x17);
    wss_outb(chip, CS4231P(1), reg | ((*chip).image[CS4231_EXT_REG] & 0x01));
    wss_outb(chip, CS4231P(1), val);
    (*chip).eimage[CS4236_REG(reg)] = val;
}

#[no_mangle]
pub unsafe extern "C" fn snd_cs4236_ext_in(chip: *mut snd_wss, reg: u8) -> u8 {
    wss_outb(chip, CS4231P(0), (*chip).mce_bit | 0x17);
    wss_outb(chip, CS4231P(1), reg | ((*chip).image[CS4231_EXT_REG] & 0x01));
    wss_inb(chip, CS4231P(1))
}

/*
 * The original source contains a disabled #if 0 snd_wss_debug() register dump.
 */

unsafe fn snd_wss_busy_wait(chip: *mut snd_wss) {
    let mut timeout = 5;
    while timeout > 0 {
        wss_inb(chip, CS4231P(0));
        timeout -= 1;
    }
    timeout = 25000;
    while timeout > 0 && (wss_inb(chip, CS4231P(0)) & CS4231_INIT) != 0 {
        udelay(10);
        timeout -= 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_wss_mce_up(chip: *mut snd_wss) {
    snd_wss_wait(chip);
    (*chip).mce_bit |= CS4231_MCE;
    let timeout = wss_inb(chip, CS4231P(0));
    if (timeout & CS4231_MCE) == 0 {
        wss_outb(chip, CS4231P(0), (*chip).mce_bit | (timeout & 0x1f));
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_wss_mce_down(chip: *mut snd_wss) {
    let hw_mask = WSS_HW_CS4231_MASK | WSS_HW_CS4232_MASK | WSS_HW_AD1848;
    snd_wss_busy_wait(chip);
    (*chip).mce_bit &= !CS4231_MCE;
    let timeout = wss_inb(chip, CS4231P(0));
    wss_outb(chip, CS4231P(0), (*chip).mce_bit | (timeout & 0x1f));
    if (timeout & CS4231_MCE) == 0 || ((*chip).hardware & hw_mask) == 0 {
        return;
    }
    msleep(1);
    let mut end_time = jiffies.wrapping_add(msecs_to_jiffies(250));
    while (snd_wss_in(chip, CS4231_TEST_INIT as u8) & CS4231_CALIB_IN_PROGRESS) != 0 {
        if time_after(jiffies, end_time) {
            return;
        }
        msleep(1);
    }
    end_time = jiffies.wrapping_add(msecs_to_jiffies(100));
    while (wss_inb(chip, CS4231P(0)) & CS4231_INIT) != 0 {
        if time_after(jiffies, end_time) {
            return;
        }
        msleep(1);
    }
}

unsafe fn snd_wss_get_count(format: u8, mut size: c_uint) -> c_uint {
    match format & 0xe0 {
        CS4231_LINEAR_16 | CS4231_LINEAR_16_BIG => size >>= 1,
        CS4231_ADPCM_16 => return size >> 2,
        _ => {}
    }
    if (format & CS4231_STEREO) != 0 {
        size >>= 1;
    }
    size
}

unsafe extern "C" fn snd_wss_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let do_start = match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => 1,
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => 0,
        _ => return -EINVAL,
    };
    let mut what: c_uint = 0;
    let mut s: *mut snd_pcm_substream = ptr::null_mut();
    while snd_pcm_group_for_each_entry_next(&mut s, substream) {
        if s == (*chip).playback_substream {
            what |= CS4231_PLAYBACK_ENABLE as c_uint;
            snd_pcm_trigger_done(s, substream);
        } else if s == (*chip).capture_substream {
            what |= CS4231_RECORD_ENABLE as c_uint;
            snd_pcm_trigger_done(s, substream);
        }
    }
    if do_start != 0 {
        (*chip).image[CS4231_IFACE_CTRL] |= what as u8;
        if let Some(trigger) = (*chip).trigger {
            trigger(chip, what, 1);
        }
    } else {
        (*chip).image[CS4231_IFACE_CTRL] &= !(what as u8);
        if let Some(trigger) = (*chip).trigger {
            trigger(chip, what, 0);
        }
    }
    snd_wss_out(chip, CS4231_IFACE_CTRL as u8, (*chip).image[CS4231_IFACE_CTRL]);
    0
}

unsafe fn snd_wss_get_rate(rate: c_uint) -> u8 {
    for i in 0..rates.len() {
        if rate == rates[i] {
            return freq_bits[i];
        }
    }
    freq_bits[rates.len() - 1]
}

unsafe fn snd_wss_get_format(_chip: *mut snd_wss, format: snd_pcm_format_t, channels: c_int) -> u8 {
    let mut rformat = CS4231_LINEAR_8;
    match format {
        SNDRV_PCM_FORMAT_MU_LAW => rformat = CS4231_ULAW_8,
        SNDRV_PCM_FORMAT_A_LAW => rformat = CS4231_ALAW_8,
        SNDRV_PCM_FORMAT_S16_LE => rformat = CS4231_LINEAR_16,
        SNDRV_PCM_FORMAT_S16_BE => rformat = CS4231_LINEAR_16_BIG,
        SNDRV_PCM_FORMAT_IMA_ADPCM => rformat = CS4231_ADPCM_16,
        _ => {}
    }
    if channels > 1 {
        rformat |= CS4231_STEREO;
    }
    rformat
}

unsafe fn snd_wss_calibrate_mute(chip: *mut snd_wss, mute_arg: c_int) {
    let mute: c_int = if mute_arg != 0 { 0x80 } else { 0 };
    if (*chip).calibrate_mute == mute {
        return;
    }
    if mute == 0 {
        snd_wss_dout(chip, CS4231_LEFT_INPUT as u8, (*chip).image[CS4231_LEFT_INPUT]);
        snd_wss_dout(chip, CS4231_RIGHT_INPUT as u8, (*chip).image[CS4231_RIGHT_INPUT]);
        snd_wss_dout(chip, CS4231_LOOPBACK as u8, (*chip).image[CS4231_LOOPBACK]);
    } else {
        snd_wss_dout(chip, CS4231_LEFT_INPUT as u8, 0);
        snd_wss_dout(chip, CS4231_RIGHT_INPUT as u8, 0);
        snd_wss_dout(chip, CS4231_LOOPBACK as u8, 0xfd);
    }
    snd_wss_dout(chip, CS4231_AUX1_LEFT_INPUT as u8, mute as u8 | (*chip).image[CS4231_AUX1_LEFT_INPUT]);
    snd_wss_dout(chip, CS4231_AUX1_RIGHT_INPUT as u8, mute as u8 | (*chip).image[CS4231_AUX1_RIGHT_INPUT]);
    snd_wss_dout(chip, CS4231_AUX2_LEFT_INPUT as u8, mute as u8 | (*chip).image[CS4231_AUX2_LEFT_INPUT]);
    snd_wss_dout(chip, CS4231_AUX2_RIGHT_INPUT as u8, mute as u8 | (*chip).image[CS4231_AUX2_RIGHT_INPUT]);
    snd_wss_dout(chip, CS4231_LEFT_OUTPUT as u8, mute as u8 | (*chip).image[CS4231_LEFT_OUTPUT]);
    snd_wss_dout(chip, CS4231_RIGHT_OUTPUT as u8, mute as u8 | (*chip).image[CS4231_RIGHT_OUTPUT]);
    if ((*chip).hardware & WSS_HW_AD1848_MASK) == 0 {
        snd_wss_dout(chip, CS4231_LEFT_LINE_IN as u8, mute as u8 | (*chip).image[CS4231_LEFT_LINE_IN]);
        snd_wss_dout(chip, CS4231_RIGHT_LINE_IN as u8, mute as u8 | (*chip).image[CS4231_RIGHT_LINE_IN]);
        snd_wss_dout(chip, CS4231_MONO_CTRL as u8, if mute != 0 { 0xc0 } else { (*chip).image[CS4231_MONO_CTRL] });
    }
    if (*chip).hardware == WSS_HW_INTERWAVE {
        snd_wss_dout(chip, CS4231_LEFT_MIC_INPUT as u8, mute as u8 | (*chip).image[CS4231_LEFT_MIC_INPUT]);
        snd_wss_dout(chip, CS4231_RIGHT_MIC_INPUT as u8, mute as u8 | (*chip).image[CS4231_RIGHT_MIC_INPUT]);
        snd_wss_dout(chip, CS4231_LINE_LEFT_OUTPUT as u8, mute as u8 | (*chip).image[CS4231_LINE_LEFT_OUTPUT]);
        snd_wss_dout(chip, CS4231_LINE_RIGHT_OUTPUT as u8, mute as u8 | (*chip).image[CS4231_LINE_RIGHT_OUTPUT]);
    }
    (*chip).calibrate_mute = mute;
}

unsafe extern "C" fn snd_wss_playback_format(chip: *mut snd_wss, params: *mut snd_pcm_hw_params, mut pdfr: u8) {
    let mut full_calib = 1;
    if (*chip).hardware == WSS_HW_CS4231A || ((*chip).hardware & WSS_HW_CS4232_MASK) != 0 {
        if ((*chip).image[CS4231_PLAYBK_FORMAT] & 0x0f) == (pdfr & 0x0f) {
            snd_wss_out(chip, CS4231_ALT_FEATURE_1 as u8, (*chip).image[CS4231_ALT_FEATURE_1] | 0x10);
            (*chip).image[CS4231_PLAYBK_FORMAT] = pdfr;
            snd_wss_out(chip, CS4231_PLAYBK_FORMAT as u8, (*chip).image[CS4231_PLAYBK_FORMAT]);
            (*chip).image[CS4231_ALT_FEATURE_1] &= !0x10;
            snd_wss_out(chip, CS4231_ALT_FEATURE_1 as u8, (*chip).image[CS4231_ALT_FEATURE_1]);
            udelay(100);
            full_calib = 0;
        }
    } else if (*chip).hardware == WSS_HW_AD1845 {
        let rate = params_rate(params);
        snd_wss_out(chip, CS4231_PLAYBK_FORMAT as u8, pdfr & 0xf0);
        snd_wss_out(chip, AD1845_UPR_FREQ_SEL as u8, ((rate >> 8) & 0xff) as u8);
        snd_wss_out(chip, AD1845_LWR_FREQ_SEL as u8, (rate & 0xff) as u8);
        full_calib = 0;
    }
    if full_calib != 0 {
        snd_wss_mce_up(chip);
        if (*chip).hardware != WSS_HW_INTERWAVE && (*chip).single_dma == 0 {
            if ((*chip).image[CS4231_IFACE_CTRL] & CS4231_RECORD_ENABLE) != 0 {
                pdfr = (pdfr & 0xf0) | ((*chip).image[CS4231_REC_FORMAT] & 0x0f);
            }
        } else {
            (*chip).image[CS4231_PLAYBK_FORMAT] = pdfr;
        }
        snd_wss_out(chip, CS4231_PLAYBK_FORMAT as u8, pdfr);
        if (*chip).hardware == WSS_HW_OPL3SA2 {
            udelay(100);
        }
        snd_wss_mce_down(chip);
    }
}

unsafe extern "C" fn snd_wss_capture_format(chip: *mut snd_wss, params: *mut snd_pcm_hw_params, cdfr: u8) {
    let mut full_calib = 1;
    if (*chip).hardware == WSS_HW_CS4231A || ((*chip).hardware & WSS_HW_CS4232_MASK) != 0 {
        if ((*chip).image[CS4231_PLAYBK_FORMAT] & 0x0f) == (cdfr & 0x0f) ||
            ((*chip).image[CS4231_IFACE_CTRL] & CS4231_PLAYBACK_ENABLE) != 0 {
            snd_wss_out(chip, CS4231_ALT_FEATURE_1 as u8, (*chip).image[CS4231_ALT_FEATURE_1] | 0x20);
            (*chip).image[CS4231_REC_FORMAT] = cdfr;
            snd_wss_out(chip, CS4231_REC_FORMAT as u8, (*chip).image[CS4231_REC_FORMAT]);
            (*chip).image[CS4231_ALT_FEATURE_1] &= !0x20;
            snd_wss_out(chip, CS4231_ALT_FEATURE_1 as u8, (*chip).image[CS4231_ALT_FEATURE_1]);
            full_calib = 0;
        }
    } else if (*chip).hardware == WSS_HW_AD1845 {
        let rate = params_rate(params);
        snd_wss_out(chip, CS4231_REC_FORMAT as u8, cdfr & 0xf0);
        snd_wss_out(chip, AD1845_UPR_FREQ_SEL as u8, ((rate >> 8) & 0xff) as u8);
        snd_wss_out(chip, AD1845_LWR_FREQ_SEL as u8, (rate & 0xff) as u8);
        full_calib = 0;
    }
    if full_calib != 0 {
        snd_wss_mce_up(chip);
        if (*chip).hardware != WSS_HW_INTERWAVE &&
           ((*chip).image[CS4231_IFACE_CTRL] & CS4231_PLAYBACK_ENABLE) == 0 {
            if (*chip).single_dma != 0 {
                snd_wss_out(chip, CS4231_PLAYBK_FORMAT as u8, cdfr);
            } else {
                snd_wss_out(chip, CS4231_PLAYBK_FORMAT as u8,
                    ((*chip).image[CS4231_PLAYBK_FORMAT] & 0xf0) | (cdfr & 0x0f));
            }
            snd_wss_mce_down(chip);
            snd_wss_mce_up(chip);
        }
        if ((*chip).hardware & WSS_HW_AD1848_MASK) != 0 {
            snd_wss_out(chip, CS4231_PLAYBK_FORMAT as u8, cdfr);
        } else {
            snd_wss_out(chip, CS4231_REC_FORMAT as u8, cdfr);
        }
        snd_wss_mce_down(chip);
    }
}

unsafe extern "C" fn snd_wss_timer_resolution(timer: *mut snd_timer) -> c_ulong {
    let chip = snd_timer_chip(timer);
    if ((*chip).hardware & WSS_HW_CS4236B_MASK) != 0 {
        14467
    } else if ((*chip).image[CS4231_PLAYBK_FORMAT] & 1) != 0 {
        9969
    } else {
        9920
    }
}

unsafe extern "C" fn snd_wss_timer_start(timer: *mut snd_timer) -> c_int {
    let chip = snd_timer_chip(timer);
    let ticks = (*timer).sticks;
    if ((*chip).image[CS4231_ALT_FEATURE_1] & CS4231_TIMER_ENABLE) == 0 ||
       ((ticks >> 8) as u8) != (*chip).image[CS4231_TIMER_HIGH] ||
       (ticks as u8) != (*chip).image[CS4231_TIMER_LOW] {
        (*chip).image[CS4231_TIMER_HIGH] = (ticks >> 8) as u8;
        snd_wss_out(chip, CS4231_TIMER_HIGH as u8, (*chip).image[CS4231_TIMER_HIGH]);
        (*chip).image[CS4231_TIMER_LOW] = ticks as u8;
        snd_wss_out(chip, CS4231_TIMER_LOW as u8, (*chip).image[CS4231_TIMER_LOW]);
        snd_wss_out(chip, CS4231_ALT_FEATURE_1 as u8, (*chip).image[CS4231_ALT_FEATURE_1] | CS4231_TIMER_ENABLE);
    }
    0
}

unsafe extern "C" fn snd_wss_timer_stop(timer: *mut snd_timer) -> c_int {
    let chip = snd_timer_chip(timer);
    (*chip).image[CS4231_ALT_FEATURE_1] &= !CS4231_TIMER_ENABLE;
    snd_wss_out(chip, CS4231_ALT_FEATURE_1 as u8, (*chip).image[CS4231_ALT_FEATURE_1]);
    0
}

unsafe fn snd_wss_init(chip: *mut snd_wss) {
    snd_wss_calibrate_mute(chip, 1);
    snd_wss_mce_down(chip);
    snd_wss_mce_up(chip);
    (*chip).image[CS4231_IFACE_CTRL] &= !(CS4231_PLAYBACK_ENABLE | CS4231_PLAYBACK_PIO | CS4231_RECORD_ENABLE | CS4231_RECORD_PIO | CS4231_CALIB_MODE);
    (*chip).image[CS4231_IFACE_CTRL] |= CS4231_AUTOCALIB;
    snd_wss_out(chip, CS4231_IFACE_CTRL as u8, (*chip).image[CS4231_IFACE_CTRL]);
    snd_wss_mce_down(chip);
    snd_wss_mce_up(chip);
    (*chip).image[CS4231_IFACE_CTRL] &= !CS4231_AUTOCALIB;
    snd_wss_out(chip, CS4231_IFACE_CTRL as u8, (*chip).image[CS4231_IFACE_CTRL]);
    snd_wss_out(chip, CS4231_ALT_FEATURE_1 as u8, (*chip).image[CS4231_ALT_FEATURE_1]);
    snd_wss_mce_down(chip);
    snd_wss_out(chip, CS4231_ALT_FEATURE_2 as u8, (*chip).image[CS4231_ALT_FEATURE_2]);
    snd_wss_mce_up(chip);
    snd_wss_out(chip, CS4231_PLAYBK_FORMAT as u8, (*chip).image[CS4231_PLAYBK_FORMAT]);
    snd_wss_mce_down(chip);
    snd_wss_mce_up(chip);
    if ((*chip).hardware & WSS_HW_AD1848_MASK) == 0 {
        snd_wss_out(chip, CS4231_REC_FORMAT as u8, (*chip).image[CS4231_REC_FORMAT]);
    }
    snd_wss_mce_down(chip);
    snd_wss_calibrate_mute(chip, 0);
}

unsafe fn snd_wss_open(chip: *mut snd_wss, mode: c_uint) -> c_int {
    if ((*chip).mode & mode) != 0 || (((*chip).mode & WSS_MODE_OPEN) != 0 && (*chip).single_dma != 0) {
        return -EAGAIN;
    }
    if ((*chip).mode & WSS_MODE_OPEN) != 0 {
        (*chip).mode |= mode;
        return 0;
    }
    if ((*chip).hardware & WSS_HW_AD1848_MASK) == 0 {
        snd_wss_out(chip, CS4231_IRQ_STATUS as u8, CS4231_PLAYBACK_IRQ | CS4231_RECORD_IRQ | CS4231_TIMER_IRQ);
        snd_wss_out(chip, CS4231_IRQ_STATUS as u8, 0);
    }
    wss_outb(chip, CS4231P(2), 0);
    wss_outb(chip, CS4231P(2), 0);
    (*chip).image[CS4231_PIN_CTRL] |= CS4231_IRQ_ENABLE;
    snd_wss_out(chip, CS4231_PIN_CTRL as u8, (*chip).image[CS4231_PIN_CTRL]);
    if ((*chip).hardware & WSS_HW_AD1848_MASK) == 0 {
        snd_wss_out(chip, CS4231_IRQ_STATUS as u8, CS4231_PLAYBACK_IRQ | CS4231_RECORD_IRQ | CS4231_TIMER_IRQ);
        snd_wss_out(chip, CS4231_IRQ_STATUS as u8, 0);
    }
    (*chip).mode = mode;
    0
}

unsafe fn snd_wss_close(chip: *mut snd_wss, mode: c_uint) {
    (*chip).mode &= !mode;
    if ((*chip).mode & WSS_MODE_OPEN) != 0 {
        return;
    }
    if ((*chip).hardware & WSS_HW_AD1848_MASK) == 0 {
        snd_wss_out(chip, CS4231_IRQ_STATUS as u8, 0);
    }
    wss_outb(chip, CS4231P(2), 0);
    wss_outb(chip, CS4231P(2), 0);
    (*chip).image[CS4231_PIN_CTRL] &= !CS4231_IRQ_ENABLE;
    snd_wss_out(chip, CS4231_PIN_CTRL as u8, (*chip).image[CS4231_PIN_CTRL]);
    if ((*chip).image[CS4231_IFACE_CTRL] & (CS4231_PLAYBACK_ENABLE | CS4231_PLAYBACK_PIO | CS4231_RECORD_ENABLE | CS4231_RECORD_PIO)) != 0 {
        snd_wss_mce_up(chip);
        (*chip).image[CS4231_IFACE_CTRL] &= !(CS4231_PLAYBACK_ENABLE | CS4231_PLAYBACK_PIO | CS4231_RECORD_ENABLE | CS4231_RECORD_PIO);
        snd_wss_out(chip, CS4231_IFACE_CTRL as u8, (*chip).image[CS4231_IFACE_CTRL]);
        snd_wss_mce_down(chip);
    }
    if ((*chip).hardware & WSS_HW_AD1848_MASK) == 0 {
        snd_wss_out(chip, CS4231_IRQ_STATUS as u8, 0);
    }
    wss_outb(chip, CS4231P(2), 0);
    wss_outb(chip, CS4231P(2), 0);
    (*chip).mode = 0;
}

unsafe extern "C" fn snd_wss_timer_open(timer: *mut snd_timer) -> c_int {
    snd_wss_open(snd_timer_chip(timer), WSS_MODE_TIMER);
    0
}

unsafe extern "C" fn snd_wss_timer_close(timer: *mut snd_timer) -> c_int {
    snd_wss_close(snd_timer_chip(timer), WSS_MODE_TIMER);
    0
}

static snd_wss_timer_table: snd_timer_hardware = snd_timer_hardware {
    flags: SNDRV_TIMER_HW_AUTO,
    resolution: 9945,
    ticks: 65535,
    open: Some(snd_wss_timer_open),
    close: Some(snd_wss_timer_close),
    c_resolution: Some(snd_wss_timer_resolution),
    start: Some(snd_wss_timer_start),
    stop: Some(snd_wss_timer_stop),
};

unsafe extern "C" fn snd_wss_playback_hw_params(substream: *mut snd_pcm_substream, hw_params: *mut snd_pcm_hw_params) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let new_pdfr = snd_wss_get_format(chip, params_format(hw_params), params_channels(hw_params)) |
        snd_wss_get_rate(params_rate(hw_params));
    if let Some(set_playback_format) = (*chip).set_playback_format {
        set_playback_format(chip, hw_params, new_pdfr);
    }
    0
}

unsafe extern "C" fn snd_wss_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let size = snd_pcm_lib_buffer_bytes(substream);
    let mut count = snd_pcm_lib_period_bytes(substream);
    (*chip).p_dma_size = size;
    (*chip).image[CS4231_IFACE_CTRL] &= !(CS4231_PLAYBACK_ENABLE | CS4231_PLAYBACK_PIO);
    snd_dma_program((*chip).dma1, (*runtime).dma_addr, size, DMA_MODE_WRITE | DMA_AUTOINIT);
    count = snd_wss_get_count((*chip).image[CS4231_PLAYBK_FORMAT], count).wrapping_sub(1);
    snd_wss_out(chip, CS4231_PLY_LWR_CNT as u8, count as u8);
    snd_wss_out(chip, CS4231_PLY_UPR_CNT as u8, (count >> 8) as u8);
    0
}

unsafe extern "C" fn snd_wss_capture_hw_params(substream: *mut snd_pcm_substream, hw_params: *mut snd_pcm_hw_params) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let new_cdfr = snd_wss_get_format(chip, params_format(hw_params), params_channels(hw_params)) |
        snd_wss_get_rate(params_rate(hw_params));
    if let Some(set_capture_format) = (*chip).set_capture_format {
        set_capture_format(chip, hw_params, new_cdfr);
    }
    0
}

unsafe extern "C" fn snd_wss_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let size = snd_pcm_lib_buffer_bytes(substream);
    let mut count = snd_pcm_lib_period_bytes(substream);
    (*chip).c_dma_size = size;
    (*chip).image[CS4231_IFACE_CTRL] &= !(CS4231_RECORD_ENABLE | CS4231_RECORD_PIO);
    snd_dma_program((*chip).dma2, (*runtime).dma_addr, size, DMA_MODE_READ | DMA_AUTOINIT);
    if ((*chip).hardware & WSS_HW_AD1848_MASK) != 0 {
        count = snd_wss_get_count((*chip).image[CS4231_PLAYBK_FORMAT], count);
    } else {
        count = snd_wss_get_count((*chip).image[CS4231_REC_FORMAT], count);
    }
    count = count.wrapping_sub(1);
    if (*chip).single_dma != 0 && (*chip).hardware != WSS_HW_INTERWAVE {
        snd_wss_out(chip, CS4231_PLY_LWR_CNT as u8, count as u8);
        snd_wss_out(chip, CS4231_PLY_UPR_CNT as u8, (count >> 8) as u8);
    } else {
        snd_wss_out(chip, CS4231_REC_LWR_CNT as u8, count as u8);
        snd_wss_out(chip, CS4231_REC_UPR_CNT as u8, (count >> 8) as u8);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_wss_overrange(chip: *mut snd_wss) {
    let res = snd_wss_in(chip, CS4231_TEST_INIT as u8);
    if (res & (0x08 | 0x02)) != 0 {
        (*(*(*chip).capture_substream).runtime).overrange =
            (*(*(*chip).capture_substream).runtime).overrange.wrapping_add(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_wss_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chip = dev_id as *mut snd_wss;
    let mut status: u8;
    if ((*chip).hardware & WSS_HW_AD1848_MASK) != 0 {
        status = CS4231_PLAYBACK_IRQ;
    } else {
        status = snd_wss_in(chip, CS4231_IRQ_STATUS as u8);
    }
    if (status & CS4231_TIMER_IRQ) != 0 && !(*chip).timer.is_null() {
        snd_timer_interrupt((*chip).timer, (*(*chip).timer).sticks);
    }
    if (*chip).single_dma != 0 && (*chip).hardware != WSS_HW_INTERWAVE {
        if (status & CS4231_PLAYBACK_IRQ) != 0 {
            if ((*chip).mode & WSS_MODE_PLAY) != 0 && !(*chip).playback_substream.is_null() {
                snd_pcm_period_elapsed((*chip).playback_substream);
            }
            if ((*chip).mode & WSS_MODE_RECORD) != 0 && !(*chip).capture_substream.is_null() {
                snd_wss_overrange(chip);
                snd_pcm_period_elapsed((*chip).capture_substream);
            }
        }
    } else {
        if (status & CS4231_PLAYBACK_IRQ) != 0 && !(*chip).playback_substream.is_null() {
            snd_pcm_period_elapsed((*chip).playback_substream);
        }
        if (status & CS4231_RECORD_IRQ) != 0 && !(*chip).capture_substream.is_null() {
            snd_wss_overrange(chip);
            snd_pcm_period_elapsed((*chip).capture_substream);
        }
    }
    status = (!CS4231_ALL_IRQS) | (!status);
    if ((*chip).hardware & WSS_HW_AD1848_MASK) != 0 {
        wss_outb(chip, CS4231P(2), 0);
    } else {
        snd_wss_out(chip, CS4231_IRQ_STATUS as u8, status);
    }
    IRQ_HANDLED
}

unsafe extern "C" fn snd_wss_playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    if ((*chip).image[CS4231_IFACE_CTRL] & CS4231_PLAYBACK_ENABLE) == 0 {
        return 0;
    }
    bytes_to_frames((*substream).runtime, snd_dma_pointer((*chip).dma1, (*chip).p_dma_size))
}

unsafe extern "C" fn snd_wss_capture_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    if ((*chip).image[CS4231_IFACE_CTRL] & CS4231_RECORD_ENABLE) == 0 {
        return 0;
    }
    bytes_to_frames((*substream).runtime, snd_dma_pointer((*chip).dma2, (*chip).c_dma_size))
}

unsafe fn snd_ad1848_probe(chip: *mut snd_wss) -> c_int {
    let timeout = jiffies.wrapping_add(msecs_to_jiffies(1000));
    let mut hardware: c_uint = 0;
    while (wss_inb(chip, CS4231P(0)) & CS4231_INIT) != 0 {
        if time_after(jiffies, timeout) {
            return -ENODEV;
        }
        cond_resched();
    }
    snd_wss_dout(chip, CS4231_MISC_INFO as u8, 0);
    snd_wss_dout(chip, CS4231_RIGHT_INPUT as u8, 0x45);
    let mut r = snd_wss_in(chip, CS4231_RIGHT_INPUT as u8);
    if r != 0x45 {
        if (r & !CS4231_ENABLE_MIC_GAIN) != 0x45 {
            return -ENODEV;
        }
        hardware = WSS_HW_AD1847;
    } else {
        snd_wss_dout(chip, CS4231_LEFT_INPUT as u8, 0xaa);
        r = snd_wss_in(chip, CS4231_LEFT_INPUT as u8);
        if (r | CS4231_ENABLE_MIC_GAIN) != 0xaa {
            return -ENODEV;
        }
    }
    wss_inb(chip, CS4231P(2));
    wss_outb(chip, CS4231P(2), 0);
    mb();
    if ((*chip).hardware & WSS_HW_TYPE_MASK) != WSS_HW_DETECT {
        return 0;
    }
    if hardware != 0 {
        (*chip).hardware = hardware;
        return 0;
    }
    r = snd_wss_in(chip, CS4231_MISC_INFO as u8);
    snd_wss_dout(chip, CS4231_MISC_INFO as u8, CS4231_MODE2);
    for i in 0..16 {
        if snd_wss_in(chip, i as u8) != snd_wss_in(chip, (16 + i) as u8) {
            if (r & 0x0f) != 0x0a {
                break;
            }
            snd_wss_dout(chip, CS4231_VERSION as u8, 0);
            r = snd_wss_in(chip, CS4231_VERSION as u8) & 0xe7;
            if r == 0 {
                (*chip).hardware = WSS_HW_CMI8330;
            }
            break;
        }
        if i == 15 {
            if (r & 0x80) != 0 {
                (*chip).hardware = WSS_HW_CS4248;
            } else {
                (*chip).hardware = WSS_HW_AD1848;
            }
        }
    }
    snd_wss_dout(chip, CS4231_MISC_INFO as u8, 0);
    0
}

unsafe fn snd_wss_probe(chip: *mut snd_wss) -> c_int {
    let mut id = snd_ad1848_probe(chip);
    if id < 0 {
        return id;
    }
    let hw = (*chip).hardware;
    if (hw & WSS_HW_TYPE_MASK) == WSS_HW_DETECT {
        for _i in 0..50 {
            mb();
            if (wss_inb(chip, CS4231P(0)) & CS4231_INIT) != 0 {
                msleep(2);
            } else {
                snd_wss_out(chip, CS4231_MISC_INFO as u8, CS4231_MODE2);
                id = (snd_wss_in(chip, CS4231_MISC_INFO as u8) & 0x0f) as c_int;
                if id == 0x0a {
                    break;
                }
            }
        }
        if id != 0x0a {
            return -ENODEV;
        }
        let rev = (snd_wss_in(chip, CS4231_VERSION as u8) & 0xe7) as c_int;
        if rev == 0x80 {
            let tmp = snd_wss_in(chip, 23);
            snd_wss_out(chip, 23, !tmp);
            if snd_wss_in(chip, 23) != tmp {
                (*chip).hardware = WSS_HW_AD1845;
            } else {
                (*chip).hardware = WSS_HW_CS4231;
            }
        } else if rev == 0xa0 {
            (*chip).hardware = WSS_HW_CS4231A;
        } else if rev == 0xa2 {
            (*chip).hardware = WSS_HW_CS4232;
        } else if rev == 0xb2 {
            (*chip).hardware = WSS_HW_CS4232A;
        } else if rev == 0x83 {
            (*chip).hardware = WSS_HW_CS4236;
        } else if rev == 0x03 {
            (*chip).hardware = WSS_HW_CS4236B;
        } else {
            return -ENODEV;
        }
    }
    wss_inb(chip, CS4231P(2));
    wss_outb(chip, CS4231P(2), 0);
    mb();
    if ((*chip).hardware & WSS_HW_AD1848_MASK) == 0 {
        (*chip).image[CS4231_MISC_INFO] = CS4231_MODE2;
    }
    match (*chip).hardware {
        WSS_HW_INTERWAVE => (*chip).image[CS4231_MISC_INFO] = CS4231_IW_MODE3,
        WSS_HW_CS4235 | WSS_HW_CS4236B | WSS_HW_CS4237B | WSS_HW_CS4238B | WSS_HW_CS4239 => {
            if hw == WSS_HW_DETECT3 {
                (*chip).image[CS4231_MISC_INFO] = CS4231_4236_MODE3;
            } else {
                (*chip).hardware = WSS_HW_CS4236;
            }
        }
        _ => {}
    }
    (*chip).image[CS4231_IFACE_CTRL] =
        ((*chip).image[CS4231_IFACE_CTRL] & !CS4231_SINGLE_DMA) |
        if (*chip).single_dma != 0 { CS4231_SINGLE_DMA } else { 0 };
    if (*chip).hardware != WSS_HW_OPTI93X {
        (*chip).image[CS4231_ALT_FEATURE_1] = 0x80;
        (*chip).image[CS4231_ALT_FEATURE_2] = if (*chip).hardware == WSS_HW_INTERWAVE { 0xc2 } else { 0x01 };
    }
    if (*chip).hardware == WSS_HW_AD1845 {
        (*chip).image[AD1845_PWR_DOWN] = 8;
    }
    let regnum = if ((*chip).hardware & WSS_HW_AD1848_MASK) != 0 { 16 } else { 32 };
    snd_wss_mce_down(chip);
    for i in 0..regnum {
        snd_wss_out(chip, i as u8, (*chip).image[i]);
    }
    snd_wss_mce_up(chip);
    snd_wss_mce_down(chip);
    mdelay(2);
    if (hw & WSS_HW_TYPE_MASK) == WSS_HW_DETECT && (*chip).hardware == WSS_HW_CS4236B {
        let rev = snd_cs4236_ext_in(chip, CS4236_VERSION);
        snd_cs4236_ext_out(chip, CS4236_VERSION, 0xff);
        id = snd_cs4236_ext_in(chip, CS4236_VERSION) as c_int;
        snd_cs4236_ext_out(chip, CS4236_VERSION, rev);
        if (id & 0x1f) == 0x1d {
            (*chip).hardware = WSS_HW_CS4235;
        } else if (id & 0x1f) == 0x0b {
            match id >> 5 {
                4 | 5 | 6 | 7 => (*chip).hardware = WSS_HW_CS4236B,
                _ => {}
            }
        } else if (id & 0x1f) == 0x08 {
            (*chip).hardware = WSS_HW_CS4237B;
        } else if (id & 0x1f) == 0x09 {
            (*chip).hardware = WSS_HW_CS4238B;
        } else if (id & 0x1f) == 0x1e {
            (*chip).hardware = WSS_HW_CS4239;
        }
    }
    0
}

static snd_wss_playback: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_SYNC_START,
    formats: SNDRV_PCM_FMTBIT_MU_LAW | SNDRV_PCM_FMTBIT_A_LAW | SNDRV_PCM_FMTBIT_IMA_ADPCM | SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S16_BE,
    rates: SNDRV_PCM_RATE_KNOT | SNDRV_PCM_RATE_8000_48000,
    rate_min: 5510, rate_max: 48000, channels_min: 1, channels_max: 2,
    buffer_bytes_max: 128 * 1024, period_bytes_min: 64, period_bytes_max: 128 * 1024,
    periods_min: 1, periods_max: 1024, fifo_size: 0,
};

static snd_wss_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_RESUME | SNDRV_PCM_INFO_SYNC_START,
    formats: SNDRV_PCM_FMTBIT_MU_LAW | SNDRV_PCM_FMTBIT_A_LAW | SNDRV_PCM_FMTBIT_IMA_ADPCM | SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S16_BE,
    rates: SNDRV_PCM_RATE_KNOT | SNDRV_PCM_RATE_8000_48000,
    rate_min: 5510, rate_max: 48000, channels_min: 1, channels_max: 2,
    buffer_bytes_max: 128 * 1024, period_bytes_min: 64, period_bytes_max: 128 * 1024,
    periods_min: 1, periods_max: 1024, fifo_size: 0,
};

unsafe extern "C" fn snd_wss_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    (*runtime).hw = snd_wss_playback;
    if ((*chip).hardware & WSS_HW_AD1848_MASK) != 0 {
        (*runtime).hw.formats &= !(SNDRV_PCM_FMTBIT_IMA_ADPCM | SNDRV_PCM_FMTBIT_S16_BE);
    }
    if (*chip).hardware == WSS_HW_INTERWAVE && (*chip).dma1 > 3 {
        (*runtime).hw.formats &= !SNDRV_PCM_FMTBIT_MU_LAW;
    }
    if (*chip).hardware == WSS_HW_CS4235 || (*chip).hardware == WSS_HW_CS4239 {
        (*runtime).hw.formats = SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE;
    }
    snd_pcm_limit_isa_dma_size((*chip).dma1, &mut (*runtime).hw.buffer_bytes_max);
    snd_pcm_limit_isa_dma_size((*chip).dma1, &mut (*runtime).hw.period_bytes_max);
    if let Some(claim_dma) = (*chip).claim_dma {
        let err = claim_dma(chip, (*chip).dma_private_data, (*chip).dma1);
        if err < 0 { return err; }
    }
    let err = snd_wss_open(chip, WSS_MODE_PLAY);
    if err < 0 {
        if let Some(release_dma) = (*chip).release_dma {
            release_dma(chip, (*chip).dma_private_data, (*chip).dma1);
        }
        return err;
    }
    (*chip).playback_substream = substream;
    snd_pcm_set_sync(substream);
    if let Some(rate_constraint) = (*chip).rate_constraint {
        rate_constraint(runtime);
    }
    0
}

unsafe extern "C" fn snd_wss_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    (*runtime).hw = snd_wss_capture;
    if ((*chip).hardware & WSS_HW_AD1848_MASK) != 0 {
        (*runtime).hw.formats &= !(SNDRV_PCM_FMTBIT_IMA_ADPCM | SNDRV_PCM_FMTBIT_S16_BE);
    }
    if (*chip).hardware == WSS_HW_CS4235 || (*chip).hardware == WSS_HW_CS4239 || (*chip).hardware == WSS_HW_OPTI93X {
        (*runtime).hw.formats = SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE;
    }
    snd_pcm_limit_isa_dma_size((*chip).dma2, &mut (*runtime).hw.buffer_bytes_max);
    snd_pcm_limit_isa_dma_size((*chip).dma2, &mut (*runtime).hw.period_bytes_max);
    if let Some(claim_dma) = (*chip).claim_dma {
        let err = claim_dma(chip, (*chip).dma_private_data, (*chip).dma2);
        if err < 0 { return err; }
    }
    let err = snd_wss_open(chip, WSS_MODE_RECORD);
    if err < 0 {
        if let Some(release_dma) = (*chip).release_dma {
            release_dma(chip, (*chip).dma_private_data, (*chip).dma2);
        }
        return err;
    }
    (*chip).capture_substream = substream;
    snd_pcm_set_sync(substream);
    if let Some(rate_constraint) = (*chip).rate_constraint {
        rate_constraint(runtime);
    }
    0
}

unsafe extern "C" fn snd_wss_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    (*chip).playback_substream = ptr::null_mut();
    snd_wss_close(chip, WSS_MODE_PLAY);
    0
}

unsafe extern "C" fn snd_wss_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    (*chip).capture_substream = ptr::null_mut();
    snd_wss_close(chip, WSS_MODE_RECORD);
    0
}

unsafe fn snd_wss_thinkpad_twiddle(chip: *mut snd_wss, on: c_int) {
    if (*chip).thinkpad_flag == 0 {
        return;
    }
    outb(0x1c, AD1848_THINKPAD_CTL_PORT1);
    let mut tmp = inb(AD1848_THINKPAD_CTL_PORT2);
    if on != 0 {
        tmp |= AD1848_THINKPAD_CS4248_ENABLE_BIT as u8;
    } else {
        tmp &= !(AD1848_THINKPAD_CS4248_ENABLE_BIT as u8);
    }
    outb(tmp, AD1848_THINKPAD_CTL_PORT2);
}

/* CONFIG_PM lowlevel suspend callback for CS4231 */
unsafe extern "C" fn snd_wss_suspend(chip: *mut snd_wss) {
    for reg in 0..32 {
        (*chip).image[reg] = snd_wss_in(chip, reg as u8);
    }
    if (*chip).thinkpad_flag != 0 {
        snd_wss_thinkpad_twiddle(chip, 0);
    }
}

/* CONFIG_PM lowlevel resume callback for CS4231 */
unsafe extern "C" fn snd_wss_resume(chip: *mut snd_wss) {
    if (*chip).thinkpad_flag != 0 {
        snd_wss_thinkpad_twiddle(chip, 1);
    }
    snd_wss_mce_up(chip);
    for reg in 0..32 {
        if reg != CS4231_VERSION {
            snd_wss_out(chip, reg as u8, (*chip).image[reg]);
        }
    }
    if (*chip).hardware == WSS_HW_OPL3SA2 {
        snd_wss_out(chip, CS4231_PLAYBK_FORMAT as u8, (*chip).image[CS4231_PLAYBK_FORMAT]);
    }
    snd_wss_mce_down(chip);
}

#[no_mangle]
pub unsafe extern "C" fn snd_wss_chip_id(chip: *mut snd_wss) -> *const c_char {
    match (*chip).hardware {
        WSS_HW_CS4231 => b"CS4231\0".as_ptr() as *const c_char,
        WSS_HW_CS4231A => b"CS4231A\0".as_ptr() as *const c_char,
        WSS_HW_CS4232 => b"CS4232\0".as_ptr() as *const c_char,
        WSS_HW_CS4232A => b"CS4232A\0".as_ptr() as *const c_char,
        WSS_HW_CS4235 => b"CS4235\0".as_ptr() as *const c_char,
        WSS_HW_CS4236 => b"CS4236\0".as_ptr() as *const c_char,
        WSS_HW_CS4236B => b"CS4236B\0".as_ptr() as *const c_char,
        WSS_HW_CS4237B => b"CS4237B\0".as_ptr() as *const c_char,
        WSS_HW_CS4238B => b"CS4238B\0".as_ptr() as *const c_char,
        WSS_HW_CS4239 => b"CS4239\0".as_ptr() as *const c_char,
        WSS_HW_INTERWAVE => b"AMD InterWave\0".as_ptr() as *const c_char,
        WSS_HW_OPL3SA2 => (*(*chip).card).shortname,
        WSS_HW_AD1845 => b"AD1845\0".as_ptr() as *const c_char,
        WSS_HW_OPTI93X => b"OPTi 93x\0".as_ptr() as *const c_char,
        WSS_HW_AD1847 => b"AD1847\0".as_ptr() as *const c_char,
        WSS_HW_AD1848 => b"AD1848\0".as_ptr() as *const c_char,
        WSS_HW_CS4248 => b"CS4248\0".as_ptr() as *const c_char,
        WSS_HW_CMI8330 => b"CMI8330/C3D\0".as_ptr() as *const c_char,
        _ => b"???\0".as_ptr() as *const c_char,
    }
}

unsafe fn snd_wss_new(card: *mut snd_card, hardware: c_uint, hwshare: c_uint, rchip: *mut *mut snd_wss) -> c_int {
    *rchip = ptr::null_mut();
    let chip = devm_kzalloc((*card).dev, size_of::<snd_wss>(), GFP_KERNEL) as *mut snd_wss;
    if chip.is_null() {
        return -ENOMEM;
    }
    (*chip).hardware = hardware;
    (*chip).hwshare = hwshare;
    spin_lock_init(&mut (*chip).reg_lock);
    mutex_init(&mut (*chip).mce_mutex);
    mutex_init(&mut (*chip).open_mutex);
    (*chip).card = card;
    (*chip).rate_constraint = Some(snd_wss_xrate);
    (*chip).set_playback_format = Some(snd_wss_playback_format);
    (*chip).set_capture_format = Some(snd_wss_capture_format);
    if (*chip).hardware == WSS_HW_OPTI93X {
        (*chip).image.copy_from_slice(&snd_opti93x_original_image);
    } else {
        (*chip).image.copy_from_slice(&snd_wss_original_image);
    }
    if ((*chip).hardware & WSS_HW_AD1848_MASK) != 0 {
        (*chip).image[CS4231_PIN_CTRL] = 0;
        (*chip).image[CS4231_TEST_INIT] = 0;
    }
    *rchip = chip;
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_wss_create(card: *mut snd_card, port: c_ulong, cport: c_ulong, irq: c_int, dma1: c_int, dma2: c_int, hardware: c_uint, hwshare: c_uint, rchip: *mut *mut snd_wss) -> c_int {
    let mut chip: *mut snd_wss = ptr::null_mut();
    let mut err = snd_wss_new(card, hardware, hwshare, &mut chip);
    if err < 0 { return err; }
    (*chip).irq = -1;
    (*chip).dma1 = -1;
    (*chip).dma2 = -1;
    (*chip).res_port = devm_request_region((*card).dev, port, 4, b"WSS\0".as_ptr() as *const c_char);
    if (*chip).res_port.is_null() { return -EBUSY; }
    (*chip).port = port;
    if (cport as c_long) >= 0 {
        (*chip).res_cport = devm_request_region((*card).dev, cport, 8, b"CS4232 Control\0".as_ptr() as *const c_char);
        if (*chip).res_cport.is_null() { return -ENODEV; }
    }
    (*chip).cport = cport;
    if (hwshare & WSS_HWSHARE_IRQ) == 0 {
        if devm_request_irq((*card).dev, irq, snd_wss_interrupt, 0, b"WSS\0".as_ptr() as *const c_char, chip as *mut c_void) != 0 {
            return -EBUSY;
        }
    }
    (*chip).irq = irq;
    (*card).sync_irq = (*chip).irq;
    if (hwshare & WSS_HWSHARE_DMA1) == 0 && snd_devm_request_dma((*card).dev, dma1, b"WSS - 1\0".as_ptr() as *const c_char) != 0 {
        return -EBUSY;
    }
    (*chip).dma1 = dma1;
    if (hwshare & WSS_HWSHARE_DMA2) == 0 && dma1 != dma2 && dma2 >= 0 &&
        snd_devm_request_dma((*card).dev, dma2, b"WSS - 2\0".as_ptr() as *const c_char) != 0 {
        return -EBUSY;
    }
    if dma1 == dma2 || dma2 < 0 {
        (*chip).single_dma = 1;
        (*chip).dma2 = (*chip).dma1;
    } else {
        (*chip).dma2 = dma2;
    }
    if hardware == WSS_HW_THINKPAD {
        (*chip).thinkpad_flag = 1;
        (*chip).hardware = WSS_HW_DETECT;
        snd_wss_thinkpad_twiddle(chip, 1);
    }
    if snd_wss_probe(chip) < 0 {
        return -ENODEV;
    }
    snd_wss_init(chip);
    (*chip).suspend = Some(snd_wss_suspend);
    (*chip).resume = Some(snd_wss_resume);
    *rchip = chip;
    0
}

static snd_wss_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_wss_playback_open),
    close: Some(snd_wss_playback_close),
    hw_params: Some(snd_wss_playback_hw_params),
    prepare: Some(snd_wss_playback_prepare),
    trigger: Some(snd_wss_trigger),
    pointer: Some(snd_wss_playback_pointer),
};

static snd_wss_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_wss_capture_open),
    close: Some(snd_wss_capture_close),
    hw_params: Some(snd_wss_capture_hw_params),
    prepare: Some(snd_wss_capture_prepare),
    trigger: Some(snd_wss_trigger),
    pointer: Some(snd_wss_capture_pointer),
};

#[no_mangle]
pub unsafe extern "C" fn snd_wss_pcm(chip: *mut snd_wss, device: c_int) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let err = snd_pcm_new((*chip).card, b"WSS\0".as_ptr() as *const c_char, device, 1, 1, &mut pcm);
    if err < 0 { return err; }
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_wss_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_wss_capture_ops);
    (*pcm).private_data = chip as *mut c_void;
    (*pcm).info_flags = 0;
    if (*chip).single_dma != 0 {
        (*pcm).info_flags |= SNDRV_PCM_INFO_HALF_DUPLEX;
    }
    if (*chip).hardware != WSS_HW_INTERWAVE {
        (*pcm).info_flags |= SNDRV_PCM_INFO_JOINT_DUPLEX;
    }
    strscpy((*pcm).name, snd_wss_chip_id(chip));
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, (*(*chip).card).dev, 64 * 1024,
        if (*chip).dma1 > 3 || (*chip).dma2 > 3 { 128 * 1024 } else { 64 * 1024 });
    (*chip).pcm = pcm;
    0
}

unsafe extern "C" fn snd_wss_timer_free(timer: *mut snd_timer) {
    let chip = (*timer).private_data as *mut snd_wss;
    (*chip).timer = ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn snd_wss_timer(chip: *mut snd_wss, device: c_int) -> c_int {
    let mut timer: *mut snd_timer = ptr::null_mut();
    let mut tid = snd_timer_id {
        dev_class: SNDRV_TIMER_CLASS_CARD,
        dev_sclass: SNDRV_TIMER_SCLASS_NONE,
        card: (*(*chip).card).number,
        device,
        subdevice: 0,
    };
    let err = snd_timer_new((*chip).card, b"CS4231\0".as_ptr() as *const c_char, &mut tid, &mut timer);
    if err < 0 { return err; }
    strscpy((*timer).name, snd_wss_chip_id(chip));
    (*timer).private_data = chip as *mut c_void;
    (*timer).private_free = Some(snd_wss_timer_free);
    (*timer).hw = snd_wss_timer_table;
    (*chip).timer = timer;
    0
}

unsafe extern "C" fn snd_wss_info_mux(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    static texts: [*const c_char; 4] = [
        b"Line\0".as_ptr() as *const c_char, b"Aux\0".as_ptr() as *const c_char,
        b"Mic\0".as_ptr() as *const c_char, b"Mix\0".as_ptr() as *const c_char,
    ];
    static opl3sa_texts: [*const c_char; 4] = [
        b"Line\0".as_ptr() as *const c_char, b"CD\0".as_ptr() as *const c_char,
        b"Mic\0".as_ptr() as *const c_char, b"Mix\0".as_ptr() as *const c_char,
    ];
    static gusmax_texts: [*const c_char; 4] = [
        b"Line\0".as_ptr() as *const c_char, b"Synth\0".as_ptr() as *const c_char,
        b"Mic\0".as_ptr() as *const c_char, b"Mix\0".as_ptr() as *const c_char,
    ];
    let chip = snd_kcontrol_chip(kcontrol);
    if snd_BUG_ON((*chip).card.is_null()) { return -EINVAL; }
    let mut ptexts = texts.as_ptr();
    if strcmp((*(*chip).card).driver, b"GUS MAX\0".as_ptr() as *const c_char) == 0 {
        ptexts = gusmax_texts.as_ptr();
    }
    match (*chip).hardware {
        WSS_HW_INTERWAVE => ptexts = gusmax_texts.as_ptr(),
        WSS_HW_OPTI93X | WSS_HW_OPL3SA2 => ptexts = opl3sa_texts.as_ptr(),
        _ => {}
    }
    snd_ctl_enum_info(uinfo, 2, 4, ptexts)
}

unsafe extern "C" fn snd_wss_get_mux(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    (*ucontrol).value.enumerated.item[0] = (((*chip).image[CS4231_LEFT_INPUT] & CS4231_MIXS_ALL) >> 6) as c_uint;
    (*ucontrol).value.enumerated.item[1] = (((*chip).image[CS4231_RIGHT_INPUT] & CS4231_MIXS_ALL) >> 6) as c_uint;
    0
}

unsafe extern "C" fn snd_wss_put_mux(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    if (*ucontrol).value.enumerated.item[0] > 3 || (*ucontrol).value.enumerated.item[1] > 3 {
        return -EINVAL;
    }
    let mut left = ((*ucontrol).value.enumerated.item[0] << 6) as u8;
    let mut right = ((*ucontrol).value.enumerated.item[1] << 6) as u8;
    left = ((*chip).image[CS4231_LEFT_INPUT] & !CS4231_MIXS_ALL) | left;
    right = ((*chip).image[CS4231_RIGHT_INPUT] & !CS4231_MIXS_ALL) | right;
    let change = (left != (*chip).image[CS4231_LEFT_INPUT] || right != (*chip).image[CS4231_RIGHT_INPUT]) as c_int;
    snd_wss_out(chip, CS4231_LEFT_INPUT as u8, left);
    snd_wss_out(chip, CS4231_RIGHT_INPUT as u8, right);
    change
}

#[no_mangle]
pub unsafe extern "C" fn snd_wss_info_single(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let mask = (((*kcontrol).private_value >> 16) & 0xff) as c_long;
    (*uinfo).type_ = if mask == 1 { SNDRV_CTL_ELEM_TYPE_BOOLEAN } else { SNDRV_CTL_ELEM_TYPE_INTEGER };
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = mask;
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_wss_get_single(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let reg = ((*kcontrol).private_value & 0xff) as usize;
    let shift = (((*kcontrol).private_value >> 8) & 0xff) as u8;
    let mask = (((*kcontrol).private_value >> 16) & 0xff) as c_long;
    let invert = (((*kcontrol).private_value >> 24) & 0xff) != 0;
    (*ucontrol).value.integer.value[0] = (((*chip).image[reg] >> shift) as c_long) & mask;
    if invert {
        (*ucontrol).value.integer.value[0] = mask - (*ucontrol).value.integer.value[0];
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_wss_put_single(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let reg = ((*kcontrol).private_value & 0xff) as usize;
    let shift = (((*kcontrol).private_value >> 8) & 0xff) as u8;
    let mask = (((*kcontrol).private_value >> 16) & 0xff) as c_long;
    let invert = (((*kcontrol).private_value >> 24) & 0xff) != 0;
    let mut val = (*ucontrol).value.integer.value[0] & mask;
    if invert { val = mask - val; }
    let mut val_u8 = ((val as u8) << shift) as u8;
    val_u8 = ((*chip).image[reg] & !((mask as u8) << shift)) | val_u8;
    let change = (val_u8 != (*chip).image[reg]) as c_int;
    snd_wss_out(chip, reg as u8, val_u8);
    change
}

#[no_mangle]
pub unsafe extern "C" fn snd_wss_info_double(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let mask = (((*kcontrol).private_value >> 24) & 0xff) as c_long;
    (*uinfo).type_ = if mask == 1 { SNDRV_CTL_ELEM_TYPE_BOOLEAN } else { SNDRV_CTL_ELEM_TYPE_INTEGER };
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = mask;
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_wss_get_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let left_reg = ((*kcontrol).private_value & 0xff) as usize;
    let right_reg = (((*kcontrol).private_value >> 8) & 0xff) as usize;
    let shift_left = (((*kcontrol).private_value >> 16) & 0x07) as u8;
    let shift_right = (((*kcontrol).private_value >> 19) & 0x07) as u8;
    let mask = (((*kcontrol).private_value >> 24) & 0xff) as c_long;
    let invert = (((*kcontrol).private_value >> 22) & 1) != 0;
    (*ucontrol).value.integer.value[0] = (((*chip).image[left_reg] >> shift_left) as c_long) & mask;
    (*ucontrol).value.integer.value[1] = (((*chip).image[right_reg] >> shift_right) as c_long) & mask;
    if invert {
        (*ucontrol).value.integer.value[0] = mask - (*ucontrol).value.integer.value[0];
        (*ucontrol).value.integer.value[1] = mask - (*ucontrol).value.integer.value[1];
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_wss_put_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let left_reg = ((*kcontrol).private_value & 0xff) as usize;
    let right_reg = (((*kcontrol).private_value >> 8) & 0xff) as usize;
    let shift_left = (((*kcontrol).private_value >> 16) & 0x07) as u8;
    let shift_right = (((*kcontrol).private_value >> 19) & 0x07) as u8;
    let mut mask = (((*kcontrol).private_value >> 24) & 0xff) as u8;
    let invert = (((*kcontrol).private_value >> 22) & 1) != 0;
    let mut val1 = ((*ucontrol).value.integer.value[0] as u8) & mask;
    let mut val2 = ((*ucontrol).value.integer.value[1] as u8) & mask;
    if invert {
        val1 = mask.wrapping_sub(val1);
        val2 = mask.wrapping_sub(val2);
    }
    val1 <<= shift_left;
    val2 <<= shift_right;
    if left_reg != right_reg {
        val1 = ((*chip).image[left_reg] & !(mask << shift_left)) | val1;
        val2 = ((*chip).image[right_reg] & !(mask << shift_right)) | val2;
        let change = (val1 != (*chip).image[left_reg] || val2 != (*chip).image[right_reg]) as c_int;
        snd_wss_out(chip, left_reg as u8, val1);
        snd_wss_out(chip, right_reg as u8, val2);
        change
    } else {
        mask = (mask << shift_left) | (mask << shift_right);
        val1 = ((*chip).image[left_reg] & !mask) | val1 | val2;
        let change = (val1 != (*chip).image[left_reg]) as c_int;
        snd_wss_out(chip, left_reg as u8, val1);
        change
    }
}

/* DECLARE_TLV_DB_SCALE(db_scale_6bit, -9450, 150, 0) */
static db_scale_6bit: [c_uint; 4] = [0, (-9450i32) as c_uint, 150, 0];
/* DECLARE_TLV_DB_SCALE(db_scale_5bit_12db_max, -3450, 150, 0) */
static db_scale_5bit_12db_max: [c_uint; 4] = [0, (-3450i32) as c_uint, 150, 0];
/* DECLARE_TLV_DB_SCALE(db_scale_rec_gain, 0, 150, 0) */
static db_scale_rec_gain: [c_uint; 4] = [0, 0, 150, 0];
/* DECLARE_TLV_DB_SCALE(db_scale_4bit, -4500, 300, 0) */
static db_scale_4bit: [c_uint; 4] = [0, (-4500i32) as c_uint, 300, 0];

/*
 * The C source initializes snd_wss_controls[] with WSS_SINGLE/WSS_DOUBLE and
 * WSS_*_TLV macros plus an explicit "Capture Source" control. Those macro
 * expansions are dependency-provided, so the source-level Rust translation
 * preserves the table's existence as an external/static placeholder.
 */
static snd_wss_controls: [snd_kcontrol_new; 16] = [
    snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] },
];

#[no_mangle]
pub unsafe extern "C" fn snd_wss_mixer(chip: *mut snd_wss) -> c_int {
    if snd_BUG_ON(chip.is_null() || (*chip).pcm.is_null()) {
        return -EINVAL;
    }
    let card = (*chip).card;
    strscpy((*card).mixername, (*(*chip).pcm).name);
    let mut count = snd_wss_controls.len();
    if ((*chip).hardware & WSS_HW_AD1848_MASK) != 0 {
        count = 11;
    } else if (*chip).hardware == WSS_HW_OPTI93X {
        count = 9;
    }
    for idx in 0..count {
        let err = snd_ctl_add(card, snd_ctl_new1(&snd_wss_controls[idx], chip as *mut c_void));
        if err < 0 {
            return err;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_wss_get_pcm_ops(direction: c_int) -> *const snd_pcm_ops {
    if direction == SNDRV_PCM_STREAM_PLAYBACK {
        &snd_wss_playback_ops
    } else {
        &snd_wss_capture_ops
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
