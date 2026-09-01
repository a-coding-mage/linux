// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Driver for the Korg 1212 IO PCI card
 *
 *	Copyright (c) 2001 Haroldo Gamal <gamal@alternex.com.br>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type u16 = u16;
type u32 = u32;
type size_t = usize;
type pid_t = c_int;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;

#[repr(C)] pub struct snd_card { pub private_data: *mut c_void, pub dev: *mut device, pub number: c_int, pub sync_irq: c_int, pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>, pub driver: [c_char; 16], pub shortname: [c_char; 32], pub longname: [c_char; 80] }
#[repr(C)] pub struct pci_dev { pub dev: device, pub irq: c_int }
#[repr(C)] pub struct device { _priv: [u8; 0] }
#[repr(C)] pub struct snd_pcm { pub private_data: *mut c_void, pub private_free: Option<unsafe extern "C" fn(*mut snd_pcm)>, pub name: [c_char; 80], pub info_flags: c_uint }
#[repr(C)] pub struct spinlock_t { _priv: [u8; 0] }
#[repr(C)] pub struct mutex { _priv: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _priv: [u8; 0] }
#[repr(C)] pub struct snd_dma_buffer { pub area: *mut c_void, pub addr: u32 }
#[repr(C)] pub struct snd_pcm_substream { pub runtime: *mut snd_pcm_runtime, pub pstr: *mut snd_pcm_str }
#[repr(C)] pub struct snd_pcm_str { pub stream: c_int }
#[repr(C)] pub struct snd_pcm_runtime { pub hw: snd_pcm_hardware }
#[repr(C)] pub struct snd_pcm_hw_params { _priv: [u8; 0] }
#[repr(C)] pub struct iov_iter { pub kvec: kvec }
#[repr(C)] pub struct kvec { pub iov_base: *mut c_void }
#[repr(C)] pub struct snd_info_entry { pub private_data: *mut c_void }
#[repr(C)] pub struct snd_info_buffer { _priv: [u8; 0] }
#[repr(C)] pub struct firmware { pub size: size_t, pub data: *const u8 }
#[repr(C)] pub struct module { _priv: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_device_id {
    pub vendor: c_uint,
    pub device: c_uint,
    pub subvendor: c_uint,
    pub subdevice: c_uint,
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
    pub ioctl: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_uint, *mut c_void) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub sync_stop: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pub copy: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, c_ulong, *mut iov_iter, c_ulong) -> c_int>,
    pub fill_silence: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, c_ulong, c_ulong) -> c_int>,
}

#[repr(C)] pub struct snd_kcontrol { pub private_value: c_ulong }
#[repr(C)] pub struct snd_ctl_elem_info { pub type_: c_uint, pub count: c_uint, pub value: snd_ctl_elem_info_value }
#[repr(C)] pub union snd_ctl_elem_info_value { pub integer: snd_ctl_elem_integer_info }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_integer_info { pub min: c_long, pub max: c_long }
type c_long = isize;
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_union }
#[repr(C)] pub union snd_ctl_elem_value_union { pub integer: snd_ctl_elem_value_integer, pub enumerated: snd_ctl_elem_value_enumerated }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_integer { pub value: [c_long; 128] }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_enumerated { pub item: [c_uint; 128] }

#[repr(C)]
pub struct snd_kcontrol_new {
    pub access: c_uint,
    pub iface: c_uint,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
}

unsafe extern "C" {
    static mut current: *mut task_struct;
    static mut THIS_MODULE: *mut module;
    fn pr_debug(fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn udelay(usecs: c_uint);
    fn mdelay(msecs: c_uint);
    fn msleep(msecs: c_uint);
    fn readl(addr: *mut u32) -> u32;
    fn writel(v: u32, addr: *mut u32);
    fn writew(v: u16, addr: *mut u16);
    fn wait_event_timeout(wait: *mut wait_queue_head_t, condition: c_int, timeout: c_ulong) -> c_long;
    fn wake_up(wait: *mut wait_queue_head_t);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn mutex_init(lock: *mut mutex);
    fn init_waitqueue_head(wait: *mut wait_queue_head_t);
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn pcim_request_all_regions(pci: *mut pci_dev, name: *const c_char) -> c_int;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn pci_resource_len(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn pcim_iomap(pci: *mut pci_dev, bar: c_int, maxlen: c_ulong) -> *mut c_void;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, data: *mut c_void) -> c_int;
    fn pci_set_master(pci: *mut pci_dev);
    fn snd_devm_alloc_pages(dev: *mut device, ty: c_int, size: size_t) -> *mut snd_dma_buffer;
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback: c_int, capture: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_runtime_buffer(substream: *mut snd_pcm_substream, buf: *mut snd_dma_buffer);
    fn snd_pcm_hw_constraint_single(runtime: *mut snd_pcm_runtime, var: c_int, val: c_ulong) -> c_int;
    fn snd_pcm_lib_ioctl(substream: *mut snd_pcm_substream, cmd: c_uint, arg: *mut c_void) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut snd_korg1212;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, size: c_ulong) -> c_ulong;
    fn copy_to_iter(src: *const c_void, bytes: size_t, dst: *mut iov_iter) -> size_t;
    fn copy_from_iter(dst: *mut c_void, bytes: size_t, src: *mut iov_iter) -> size_t;
    fn snd_BUG_ON(cond: c_int) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn _snd_pcm_hw_param_setempty(params: *mut snd_pcm_hw_params, var: c_int);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_korg1212;
    fn snd_ctl_enum_info(uinfo: *mut snd_ctl_elem_info, channels: c_uint, items: c_uint, names: *const *const c_char) -> c_int;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(kcontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_card_ro_proc_new(card: *mut snd_card, name: *const c_char, private_data: *mut c_void, read: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)) -> c_int;
    fn snd_devm_card_new(dev: *mut device, idx: c_int, id: *mut c_char, module: *mut module, extra: size_t, card_ret: *mut *mut snd_card) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_card_free(card: *mut snd_card);
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
}

#[repr(C)] pub struct task_struct { pub pid: pid_t }

const K1212_DEBUG_LEVEL: c_int = 0;
macro_rules! K1212_DEBUG_PRINTK { ($($arg:tt)*) => {{ if K1212_DEBUG_LEVEL > 0 { unsafe { pr_debug($($arg)*); } } }}; }
macro_rules! K1212_DEBUG_PRINTK_VERBOSE { ($($arg:tt)*) => {{ if K1212_DEBUG_LEVEL > 1 { unsafe { pr_debug($($arg)*); } } }}; }

// Valid states of the Korg 1212 I/O card.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
enum CardState {
    K1212_STATE_NONEXISTENT,
    K1212_STATE_UNINITIALIZED,
    K1212_STATE_DSP_IN_PROCESS,
    K1212_STATE_DSP_COMPLETE,
    K1212_STATE_READY,
    K1212_STATE_OPEN,
    K1212_STATE_SETUP,
    K1212_STATE_PLAYING,
    K1212_STATE_MONITOR,
    K1212_STATE_CALIBRATING,
    K1212_STATE_ERRORSTOP,
    K1212_STATE_MAX_STATE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
enum korg1212_dbcnst {
    K1212_DB_RequestForData = 0,
    K1212_DB_TriggerPlay = 1,
    K1212_DB_SelectPlayMode = 2,
    K1212_DB_ConfigureBufferMemory = 3,
    K1212_DB_RequestAdatTimecode = 4,
    K1212_DB_SetClockSourceRate = 5,
    K1212_DB_ConfigureMiscMemory = 6,
    K1212_DB_TriggerFromAdat = 7,
    K1212_DB_DMAERROR = 0x80,
    K1212_DB_CARDSTOPPED = 0x81,
    K1212_DB_RebootCard = 0xA0,
    K1212_DB_BootFromDSPPage4 = 0xA4,
    K1212_DB_DSPDownloadDone = 0xAE,
    K1212_DB_StartDSPDownload = 0xAF,
}

const K1212_CMDRET_Success: c_int = 0;
const K1212_CMDRET_DIOCFailure: c_int = 1;
const K1212_CMDRET_PMFailure: c_int = 2;
const K1212_CMDRET_FailUnspecified: c_int = 3;
const K1212_CMDRET_FailBadState: c_int = 4;
const K1212_CMDRET_CardUninitialized: c_int = 5;
const K1212_CMDRET_BadIndex: c_int = 6;
const K1212_CMDRET_BadHandle: c_int = 7;
const K1212_CMDRET_NoFillRoutine: c_int = 8;
const K1212_CMDRET_FillRoutineInUse: c_int = 9;
const K1212_CMDRET_NoAckFromCard: c_int = 10;
const K1212_CMDRET_BadParams: c_int = 11;
const K1212_CMDRET_BadDevice: c_int = 12;
const K1212_CMDRET_BadFormat: c_int = 13;

const K1212_MODE_SetupPlay: u32 = 0x00000001;
const K1212_MODE_MonitorOn: u32 = 0x00000002;
const K1212_MODE_MonitorOff: u32 = 0x00000004;
const K1212_MODE_StopPlay: u32 = 0x00000008;

#[repr(C)] #[derive(Copy, Clone, PartialEq)] enum MonitorModeSelector { K1212_MONMODE_Off = 0, K1212_MONMODE_On = 1 }

const MAILBOX0_OFFSET: usize = 0x40;
const MAILBOX1_OFFSET: usize = 0x44;
const MAILBOX2_OFFSET: usize = 0x48;
const MAILBOX3_OFFSET: usize = 0x4c;
const OUT_DOORBELL_OFFSET: usize = 0x60;
const IN_DOORBELL_OFFSET: usize = 0x64;
const STATUS_REG_OFFSET: usize = 0x68;
const PCI_CONTROL_OFFSET: usize = 0x6c;
const SENS_CONTROL_OFFSET: usize = 0x6e;
const DEV_VEND_ID_OFFSET: usize = 0x70;
const MAX_COMMAND_RETRIES: u32 = 5;
const COMMAND_ACK_MASK: u16 = 0x8000;
const DOORBELL_VAL_MASK: u32 = 0x00FF;
const CARD_BOOT_DELAY_IN_MS: c_uint = 10;
const CARD_BOOT_TIMEOUT: c_ulong = 10;
const DSP_BOOT_DELAY_IN_MS: c_uint = 200;

const kNumBuffers: usize = 8;
const k1212MaxCards: usize = 4;
const k1212NumWaveDevices: usize = 6;
const k16BitChannels: usize = 10;
const k32BitChannels: usize = 2;
const kAudioChannels: usize = k16BitChannels + k32BitChannels;
const kPlayBufferFrames: usize = 1024;
const K1212_ANALOG_CHANNELS: usize = 2;
const K1212_SPDIF_CHANNELS: usize = 2;
const K1212_ADAT_CHANNELS: usize = 8;
const K1212_CHANNELS: usize = K1212_ADAT_CHANNELS + K1212_ANALOG_CHANNELS;
const K1212_MIN_CHANNELS: usize = 1;
const K1212_MAX_CHANNELS: usize = K1212_CHANNELS;
const K1212_FRAME_SIZE: usize = size_of::<KorgAudioFrame>();
const K1212_MAX_SAMPLES: usize = kPlayBufferFrames * kNumBuffers;
const K1212_PERIODS: usize = kNumBuffers;
const K1212_PERIOD_BYTES: usize = K1212_FRAME_SIZE * kPlayBufferFrames;
const K1212_BUF_SIZE: usize = K1212_PERIOD_BYTES * kNumBuffers;
const K1212_ANALOG_BUF_SIZE: usize = K1212_ANALOG_CHANNELS * 2 * kPlayBufferFrames * kNumBuffers;
const K1212_SPDIF_BUF_SIZE: usize = K1212_SPDIF_CHANNELS * 3 * kPlayBufferFrames * kNumBuffers;
const K1212_ADAT_BUF_SIZE: usize = K1212_ADAT_CHANNELS * 2 * kPlayBufferFrames * kNumBuffers;
const K1212_MAX_BUF_SIZE: usize = K1212_ANALOG_BUF_SIZE + K1212_ADAT_BUF_SIZE;

const k1212MinADCSens: u16 = 0x00;
const k1212MaxADCSens: u16 = 0x7f;
const k1212MaxVolume: c_int = 0x7fff;
const k1212MaxWaveVolume: c_int = 0xffff;
const k1212MinVolume: c_int = 0x0000;
const k1212MaxVolInverted: c_int = 0x8000;

const PCI_INT_ENABLE_BIT: u32 = 0x00000100;
const PCI_DOORBELL_INT_ENABLE_BIT: u32 = 0x00000200;
const LOCAL_INT_ENABLE_BIT: u32 = 0x00010000;
const LOCAL_DOORBELL_INT_ENABLE_BIT: u32 = 0x00020000;
const LOCAL_DMA1_INT_ENABLE_BIT: u32 = 0x00080000;
const PCI_CMD_MEM_SPACE_ENABLE_BIT: u32 = 0x0002;
const PCI_CMD_IO_SPACE_ENABLE_BIT: u32 = 0x0001;
const PCI_CMD_BUS_MASTER_ENABLE_BIT: u32 = 0x0004;
const PCI_STAT_PARITY_ERROR_BIT: u32 = 0x8000;
const PCI_STAT_SYSTEM_ERROR_BIT: u32 = 0x4000;
const PCI_STAT_MASTER_ABORT_RCVD_BIT: u32 = 0x2000;
const PCI_STAT_TARGET_ABORT_RCVD_BIT: u32 = 0x1000;
const PCI_STAT_TARGET_ABORT_SENT_BIT: u32 = 0x0800;

const SET_SENS_LOCALINIT_BITPOS: u16 = 15;
const SET_SENS_DATA_BITPOS: u16 = 10;
const SET_SENS_CLOCK_BITPOS: u16 = 8;
const SET_SENS_LOADSHIFT_BITPOS: u16 = 0;
const SET_SENS_LEFTCHANID: u16 = 0x00;
const SET_SENS_RIGHTCHANID: u16 = 0x01;
const K1212SENSUPDATE_DELAY_IN_MS: c_uint = 50;
const ONE_RTC_TICK: c_uint = 1;
const SENSCLKPULSE_WIDTH: c_uint = 4;
const LOADSHIFT_DELAY: c_uint = 4;
const INTERCOMMAND_DELAY: c_uint = 40;
const STOPCARD_DELAY: c_uint = 300;
const COMMAND_ACK_DELAY: c_uint = 13;

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
enum ClockSourceIndex {
    K1212_CLKIDX_AdatAt44_1K = 0,
    K1212_CLKIDX_AdatAt48K,
    K1212_CLKIDX_WordAt44_1K,
    K1212_CLKIDX_WordAt48K,
    K1212_CLKIDX_LocalAt44_1K,
    K1212_CLKIDX_LocalAt48K,
    K1212_CLKIDX_Invalid,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
enum ClockSourceType {
    K1212_CLKIDX_Adat = 0,
    K1212_CLKIDX_Word,
    K1212_CLKIDX_Local,
}

#[repr(C)]
struct KorgAudioFrame {
    frameData16: [u16; k16BitChannels],
    frameData32: [u32; k32BitChannels],
    timeCodeVal: u32,
}

#[repr(C)]
struct KorgAudioBuffer {
    bufferData: [KorgAudioFrame; kPlayBufferFrames],
}

#[repr(C)]
struct KorgSharedBuffer {
    // If K1212_LARGEALLOC is defined in C, playDataBufs and recordDataBufs live here before volumeData.
    volumeData: [i16; kAudioChannels],
    cardCommand: u32,
    routeData: [u16; kAudioChannels],
    AdatTimeCode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SensChanBits {
    chanVal: u8,
    chanId: u8,
}

#[repr(C)]
union SensLeft {
    v: SensChanBits,
    leftSensBits: u16,
}

#[repr(C)]
union SensRight {
    v: SensChanBits,
    rightSensBits: u16,
}

#[repr(C)]
struct SensBits {
    l: SensLeft,
    r: SensRight,
}

#[repr(C)]
struct snd_korg1212 {
    card: *mut snd_card,
    pci: *mut pci_dev,
    pcm: *mut snd_pcm,
    irq: c_int,
    lock: spinlock_t,
    open_mutex: mutex,
    wait: wait_queue_head_t,
    iomem: c_ulong,
    ioport: c_ulong,
    iomem2: c_ulong,
    irqcount: c_ulong,
    inIRQ: c_ulong,
    iobase: *mut c_void,
    dma_dsp: *mut snd_dma_buffer,
    dma_play: *mut snd_dma_buffer,
    dma_rec: *mut snd_dma_buffer,
    dma_shared: *mut snd_dma_buffer,
    DataBufsSize: u32,
    playDataBufsPtr: *mut KorgAudioBuffer,
    recordDataBufsPtr: *mut KorgAudioBuffer,
    sharedBufferPtr: *mut KorgSharedBuffer,
    RecDataPhy: u32,
    PlayDataPhy: u32,
    sharedBufferPhy: c_ulong,
    VolumeTablePhy: u32,
    RoutingTablePhy: u32,
    AdatTimeCodePhy: u32,
    statusRegPtr: *mut u32,
    outDoorbellPtr: *mut u32,
    inDoorbellPtr: *mut u32,
    mailbox0Ptr: *mut u32,
    mailbox1Ptr: *mut u32,
    mailbox2Ptr: *mut u32,
    mailbox3Ptr: *mut u32,
    controlRegPtr: *mut u32,
    sensRegPtr: *mut u16,
    idRegPtr: *mut u32,
    periodsize: size_t,
    channels: c_int,
    currentBuffer: c_int,
    playback_substream: *mut snd_pcm_substream,
    capture_substream: *mut snd_pcm_substream,
    capture_pid: pid_t,
    playback_pid: pid_t,
    cardState: CardState,
    running: c_int,
    idleMonitorOn: c_int,
    cmdRetryCount: u32,
    clkSrcRate: ClockSourceIndex,
    clkSource: ClockSourceType,
    clkRate: c_int,
    volumePhase: [c_int; kAudioChannels],
    leftADCInSens: u16,
    rightADCInSens: u16,
    opencnt: c_int,
    setcnt: c_int,
    playcnt: c_int,
    errorcnt: c_int,
    totalerrorcnt: c_ulong,
    dsp_is_loaded: c_int,
    dsp_stop_processing: c_int,
}

const SNDRV_CARDS: usize = 8;
static mut index: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
static mut id: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS];
static mut enable: [bool; SNDRV_CARDS] = [true; SNDRV_CARDS];

const PCI_ANY_ID: c_uint = !0;
static snd_korg1212_ids: [pci_device_id; 2] = [
    pci_device_id { vendor: 0x10b5, device: 0x906d, subvendor: PCI_ANY_ID, subdevice: PCI_ANY_ID },
    pci_device_id { vendor: 0, device: 0, subvendor: 0, subdevice: 0 },
];

const fn cstr(bytes: &'static [u8]) -> *const c_char { bytes.as_ptr() as *const c_char }
static stateName: [*const c_char; 11] = [
    cstr(b"Non-existent\0"), cstr(b"Uninitialized\0"), cstr(b"DSP download in process\0"),
    cstr(b"DSP download complete\0"), cstr(b"Ready\0"), cstr(b"Open\0"),
    cstr(b"Setup for play\0"), cstr(b"Playing\0"), cstr(b"Monitor mode on\0"),
    cstr(b"Calibrating\0"), cstr(b"Invalid\0"),
];
static clockSourceTypeName: [*const c_char; 3] = [cstr(b"ADAT\0"), cstr(b"S/PDIF\0"), cstr(b"local\0")];
static clockSourceName: [*const c_char; 6] = [
    cstr(b"ADAT at 44.1 kHz\0"), cstr(b"ADAT at 48 kHz\0"),
    cstr(b"S/PDIF at 44.1 kHz\0"), cstr(b"S/PDIF at 48 kHz\0"),
    cstr(b"local clock at 44.1 kHz\0"), cstr(b"local clock at 48 kHz\0"),
];
static channelName: [*const c_char; 12] = [
    cstr(b"ADAT-1\0"), cstr(b"ADAT-2\0"), cstr(b"ADAT-3\0"), cstr(b"ADAT-4\0"),
    cstr(b"ADAT-5\0"), cstr(b"ADAT-6\0"), cstr(b"ADAT-7\0"), cstr(b"ADAT-8\0"),
    cstr(b"Analog-L\0"), cstr(b"Analog-R\0"), cstr(b"SPDIF-L\0"), cstr(b"SPDIF-R\0"),
];
static ClockSourceSelector: [u16; 6] = [0x8000, 0x0000, 0x8001, 0x0001, 0x8002, 0x0002];

#[repr(C)] union swap_u32 { c: [u8; 4], i: u32 }

// The C source maps these names differently under SNDRV_BIG_ENDIAN.
unsafe fn UpperWordSwap(swappee: u32) -> u32 {
    let swapper = swap_u32 { i: swappee };
    let mut retVal = swap_u32 { i: 0 };
    retVal.c[2] = swapper.c[3];
    retVal.c[3] = swapper.c[2];
    retVal.c[1] = swapper.c[1];
    retVal.c[0] = swapper.c[0];
    retVal.i
}

unsafe fn LowerWordSwap(swappee: u32) -> u32 {
    let swapper = swap_u32 { i: swappee };
    let mut retVal = swap_u32 { i: 0 };
    retVal.c[2] = swapper.c[2];
    retVal.c[3] = swapper.c[3];
    retVal.c[1] = swapper.c[0];
    retVal.c[0] = swapper.c[1];
    retVal.i
}

fn SetBitInWord(theWord: &mut u16, bitPosition: u16) { *theWord |= 0x0001u16 << bitPosition; }
fn SetBitInDWord(theWord: &mut u32, bitPosition: u16) { *theWord |= 0x00000001u32 << bitPosition; }
fn ClearBitInWord(theWord: &mut u16, bitPosition: u16) { *theWord &= !(0x0001u16 << bitPosition); }
fn ClearBitInDWord(theWord: &mut u32, bitPosition: u16) { *theWord &= !(0x00000001u32 << bitPosition); }

unsafe extern "C" fn snd_korg1212_Send1212Command(korg1212: *mut snd_korg1212, doorbellVal: korg1212_dbcnst, mailBox0Val: u32, mailBox1Val: u32, mailBox2Val: u32, mailBox3Val: u32) -> c_int {
    let mut retryCount: u32;
    let mut mailBox3Lo: u16;
    let mut rc = K1212_CMDRET_Success;
    if (*korg1212).outDoorbellPtr.is_null() {
        K1212_DEBUG_PRINTK_VERBOSE!(cstr(b"K1212_DEBUG: CardUninitialized\n\0"));
        return K1212_CMDRET_CardUninitialized;
    }
    K1212_DEBUG_PRINTK!(cstr(b"K1212_DEBUG: Card <- 0x%08x 0x%08x [%s]\n\0"), doorbellVal as c_uint, mailBox0Val, stateName[(*korg1212).cardState as usize]);
    retryCount = 0;
    while retryCount < MAX_COMMAND_RETRIES {
        writel(mailBox3Val, (*korg1212).mailbox3Ptr);
        writel(mailBox2Val, (*korg1212).mailbox2Ptr);
        writel(mailBox1Val, (*korg1212).mailbox1Ptr);
        writel(mailBox0Val, (*korg1212).mailbox0Ptr);
        writel(doorbellVal as u32, (*korg1212).outDoorbellPtr);
        if doorbellVal == korg1212_dbcnst::K1212_DB_RebootCard ||
           doorbellVal == korg1212_dbcnst::K1212_DB_BootFromDSPPage4 ||
           doorbellVal == korg1212_dbcnst::K1212_DB_StartDSPDownload {
            rc = K1212_CMDRET_Success;
            break;
        }
        udelay(COMMAND_ACK_DELAY);
        mailBox3Lo = readl((*korg1212).mailbox3Ptr) as u16;
        if (mailBox3Lo & COMMAND_ACK_MASK) != 0 {
            if ((mailBox3Lo as u32) & DOORBELL_VAL_MASK) == ((doorbellVal as u32) & DOORBELL_VAL_MASK) {
                K1212_DEBUG_PRINTK_VERBOSE!(cstr(b"K1212_DEBUG: Card <- Success\n\0"));
                rc = K1212_CMDRET_Success;
                break;
            }
        }
        retryCount += 1;
    }
    (*korg1212).cmdRetryCount = (*korg1212).cmdRetryCount.wrapping_add(retryCount);
    if retryCount >= MAX_COMMAND_RETRIES {
        K1212_DEBUG_PRINTK_VERBOSE!(cstr(b"K1212_DEBUG: Card <- NoAckFromCard\n\0"));
        rc = K1212_CMDRET_NoAckFromCard;
    }
    rc
}

unsafe fn snd_korg1212_SendStop(korg1212: *mut snd_korg1212) {
    (*korg1212).dsp_stop_processing = 1;
    (*(*korg1212).sharedBufferPtr).cardCommand = 0xffffffff;
}

unsafe fn snd_korg1212_SendStopAndWait(korg1212: *mut snd_korg1212) {
    spin_lock(&mut (*korg1212).lock);
    snd_korg1212_SendStop(korg1212);
    spin_unlock(&mut (*korg1212).lock);
    wait_event_timeout(&mut (*korg1212).wait, ((*korg1212).dsp_stop_processing == 0) as c_int, HZ);
}

const HZ: c_ulong = 100;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const EFAULT: c_int = 14;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_SHARED: c_ulong = 0x80;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 1;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 2;
const SNDRV_PCM_INFO_BATCH: c_uint = 1 << 3;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << 0;
const SNDRV_PCM_RATE_44100: c_uint = 1 << 0;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 1;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_HW_PARAM_PERIOD_SIZE: c_int = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_IOCTL1_CHANNEL_INFO: c_uint = 0x100;
const SNDRV_PCM_INFO_JOINT_DUPLEX: c_uint = 1 << 4;
const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_uint = 1;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 2;
const SNDRV_CTL_ELEM_ACCESS_READ: c_uint = 1;
const SNDRV_CTL_ELEM_ACCESS_WRITE: c_uint = 2;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 0;

unsafe fn snd_korg1212_TurnOnIdleMonitor(korg1212: *mut snd_korg1212) -> c_int {
    udelay(INTERCOMMAND_DELAY);
    spin_lock(&mut (*korg1212).lock);
    (*korg1212).idleMonitorOn = 1;
    let rc = snd_korg1212_Send1212Command(korg1212, korg1212_dbcnst::K1212_DB_SelectPlayMode, K1212_MODE_MonitorOn, 0, 0, 0);
    spin_unlock(&mut (*korg1212).lock);
    rc
}

unsafe fn snd_korg1212_TurnOffIdleMonitor(korg1212: *mut snd_korg1212) {
    if (*korg1212).idleMonitorOn != 0 {
        snd_korg1212_SendStopAndWait(korg1212);
        (*korg1212).idleMonitorOn = 0;
    }
}

unsafe fn snd_korg1212_setCardState(korg1212: *mut snd_korg1212, csState: CardState) {
    (*korg1212).cardState = csState;
}

unsafe fn snd_korg1212_OpenCard(korg1212: *mut snd_korg1212) -> c_int {
    K1212_DEBUG_PRINTK!(cstr(b"K1212_DEBUG: OpenCard [%s] %d\n\0"), stateName[(*korg1212).cardState as usize], (*korg1212).opencnt);
    if { let old = (*korg1212).opencnt; (*korg1212).opencnt += 1; old } == 0 {
        snd_korg1212_TurnOffIdleMonitor(korg1212);
        snd_korg1212_setCardState(korg1212, CardState::K1212_STATE_OPEN);
    }
    1
}

unsafe fn snd_korg1212_CloseCard(korg1212: *mut snd_korg1212) -> c_int {
    K1212_DEBUG_PRINTK!(cstr(b"K1212_DEBUG: CloseCard [%s] %d\n\0"), stateName[(*korg1212).cardState as usize], (*korg1212).opencnt);
    (*korg1212).opencnt -= 1;
    if (*korg1212).opencnt != 0 { return 0; }
    if (*korg1212).cardState == CardState::K1212_STATE_SETUP {
        let rc = snd_korg1212_Send1212Command(korg1212, korg1212_dbcnst::K1212_DB_SelectPlayMode, K1212_MODE_StopPlay, 0, 0, 0);
        if rc != 0 { K1212_DEBUG_PRINTK!(cstr(b"K1212_DEBUG: CloseCard - RC = %d [%s]\n\0"), rc, stateName[(*korg1212).cardState as usize]); }
        if rc != K1212_CMDRET_Success { return 0; }
    } else if (*korg1212).cardState > CardState::K1212_STATE_SETUP {
        snd_korg1212_SendStopAndWait(korg1212);
    }
    if (*korg1212).cardState > CardState::K1212_STATE_READY {
        snd_korg1212_TurnOnIdleMonitor(korg1212);
        snd_korg1212_setCardState(korg1212, CardState::K1212_STATE_READY);
    }
    0
}

unsafe fn snd_korg1212_SetupForPlay(korg1212: *mut snd_korg1212) -> c_int {
    if { let old = (*korg1212).setcnt; (*korg1212).setcnt += 1; old } != 0 { return 0; }
    snd_korg1212_setCardState(korg1212, CardState::K1212_STATE_SETUP);
    let rc = snd_korg1212_Send1212Command(korg1212, korg1212_dbcnst::K1212_DB_SelectPlayMode, K1212_MODE_SetupPlay, 0, 0, 0);
    if rc != K1212_CMDRET_Success { return 1; }
    0
}

unsafe fn snd_korg1212_TriggerPlay(korg1212: *mut snd_korg1212) -> c_int {
    if { let old = (*korg1212).playcnt; (*korg1212).playcnt += 1; old } != 0 { return 0; }
    snd_korg1212_setCardState(korg1212, CardState::K1212_STATE_PLAYING);
    let rc = snd_korg1212_Send1212Command(korg1212, korg1212_dbcnst::K1212_DB_TriggerPlay, 0, 0, 0, 0);
    if rc != K1212_CMDRET_Success { return 1; }
    0
}

unsafe fn snd_korg1212_StopPlay(korg1212: *mut snd_korg1212) -> c_int {
    (*korg1212).playcnt -= 1;
    if (*korg1212).playcnt != 0 { return 0; }
    (*korg1212).setcnt = 0;
    if (*korg1212).cardState != CardState::K1212_STATE_ERRORSTOP { snd_korg1212_SendStop(korg1212); }
    snd_korg1212_setCardState(korg1212, CardState::K1212_STATE_OPEN);
    0
}

unsafe fn snd_korg1212_EnableCardInterrupts(korg1212: *mut snd_korg1212) {
    writel(PCI_INT_ENABLE_BIT | PCI_DOORBELL_INT_ENABLE_BIT | LOCAL_INT_ENABLE_BIT | LOCAL_DOORBELL_INT_ENABLE_BIT | LOCAL_DMA1_INT_ENABLE_BIT, (*korg1212).statusRegPtr);
}

// #if 0 not used: snd_korg1212_SetMonitorMode translated intent omitted from executable code as in C.

unsafe fn snd_korg1212_use_is_exclusive(korg1212: *mut snd_korg1212) -> c_int {
    if (*korg1212).playback_pid != (*korg1212).capture_pid && (*korg1212).playback_pid >= 0 && (*korg1212).capture_pid >= 0 { return 0; }
    1
}

unsafe fn snd_korg1212_SetRate(korg1212: *mut snd_korg1212, rate: c_int) -> c_int {
    let s44 = [ClockSourceIndex::K1212_CLKIDX_AdatAt44_1K, ClockSourceIndex::K1212_CLKIDX_WordAt44_1K, ClockSourceIndex::K1212_CLKIDX_LocalAt44_1K];
    let s48 = [ClockSourceIndex::K1212_CLKIDX_AdatAt48K, ClockSourceIndex::K1212_CLKIDX_WordAt48K, ClockSourceIndex::K1212_CLKIDX_LocalAt48K];
    if snd_korg1212_use_is_exclusive(korg1212) == 0 { return -EBUSY; }
    let parm = match rate {
        44100 => s44[(*korg1212).clkSource as usize],
        48000 => s48[(*korg1212).clkSource as usize],
        _ => return -EINVAL,
    };
    (*korg1212).clkSrcRate = parm;
    (*korg1212).clkRate = rate;
    udelay(INTERCOMMAND_DELAY);
    let rc = snd_korg1212_Send1212Command(korg1212, korg1212_dbcnst::K1212_DB_SetClockSourceRate, ClockSourceSelector[(*korg1212).clkSrcRate as usize] as u32, 0, 0, 0);
    if rc != 0 { K1212_DEBUG_PRINTK!(cstr(b"K1212_DEBUG: Set Clock Source Selector - RC = %d [%s]\n\0"), rc, stateName[(*korg1212).cardState as usize]); }
    0
}

unsafe fn snd_korg1212_SetClockSource(korg1212: *mut snd_korg1212, source: c_int) -> c_int {
    if source < 0 || source > 2 { return -EINVAL; }
    (*korg1212).clkSource = core::mem::transmute(source);
    snd_korg1212_SetRate(korg1212, (*korg1212).clkRate);
    0
}

unsafe fn snd_korg1212_DisableCardInterrupts(korg1212: *mut snd_korg1212) {
    writel(0, (*korg1212).statusRegPtr);
}

unsafe fn snd_korg1212_WriteADCSensitivity(korg1212: *mut snd_korg1212) -> c_int {
    let mut sensVals = SensBits { l: SensLeft { leftSensBits: 0 }, r: SensRight { rightSensBits: 0 } };
    let mut controlValue: u16 = 0;
    SetBitInWord(&mut controlValue, SET_SENS_LOCALINIT_BITPOS);
    let monModeSet = if (*korg1212).cardState == CardState::K1212_STATE_MONITOR || (*korg1212).idleMonitorOn != 0 {
        snd_korg1212_SendStopAndWait(korg1212); 1
    } else { 0 };
    spin_lock(&mut (*korg1212).lock);
    writel(0, (*korg1212).mailbox3Ptr);
    udelay(LOADSHIFT_DELAY);
    let clkIs48K = match (*korg1212).clkSrcRate {
        ClockSourceIndex::K1212_CLKIDX_AdatAt44_1K | ClockSourceIndex::K1212_CLKIDX_WordAt44_1K | ClockSourceIndex::K1212_CLKIDX_LocalAt44_1K => 0,
        _ => 1,
    };
    sensVals.l.v = SensChanBits { chanVal: (*korg1212).leftADCInSens as u8, chanId: SET_SENS_LEFTCHANID as u8 };
    sensVals.r.v = SensChanBits { chanVal: (*korg1212).rightADCInSens as u8, chanId: SET_SENS_RIGHTCHANID as u8 };
    for channel in 0..2 {
        ClearBitInWord(&mut controlValue, SET_SENS_LOADSHIFT_BITPOS);
        ClearBitInWord(&mut controlValue, SET_SENS_DATA_BITPOS);
        writew(controlValue, (*korg1212).sensRegPtr);
        udelay(LOADSHIFT_DELAY);
        let mut bitPosition: c_int = 15;
        while bitPosition >= 0 {
            let bits = if channel == 0 { sensVals.l.leftSensBits } else { sensVals.r.rightSensBits };
            if (bits & (0x0001u16 << bitPosition)) != 0 { SetBitInWord(&mut controlValue, SET_SENS_DATA_BITPOS); } else { ClearBitInWord(&mut controlValue, SET_SENS_DATA_BITPOS); }
            ClearBitInWord(&mut controlValue, SET_SENS_CLOCK_BITPOS);
            writew(controlValue, (*korg1212).sensRegPtr);
            udelay(SENSCLKPULSE_WIDTH);
            SetBitInWord(&mut controlValue, SET_SENS_CLOCK_BITPOS);
            writew(controlValue, (*korg1212).sensRegPtr);
            udelay(SENSCLKPULSE_WIDTH);
            bitPosition -= 1;
        }
        ClearBitInWord(&mut controlValue, SET_SENS_DATA_BITPOS);
        ClearBitInWord(&mut controlValue, SET_SENS_CLOCK_BITPOS);
        SetBitInWord(&mut controlValue, SET_SENS_LOADSHIFT_BITPOS);
        writew(controlValue, (*korg1212).sensRegPtr);
        udelay(SENSCLKPULSE_WIDTH);
        if clkIs48K != 0 { SetBitInWord(&mut controlValue, SET_SENS_DATA_BITPOS); }
        writew(controlValue, (*korg1212).sensRegPtr);
        udelay(ONE_RTC_TICK);
        SetBitInWord(&mut controlValue, SET_SENS_CLOCK_BITPOS);
        writew(controlValue, (*korg1212).sensRegPtr);
        udelay(SENSCLKPULSE_WIDTH);
        ClearBitInWord(&mut controlValue, SET_SENS_CLOCK_BITPOS);
        writew(controlValue, (*korg1212).sensRegPtr);
        udelay(SENSCLKPULSE_WIDTH);
    }
    for _ in 0..10 { udelay(SENSCLKPULSE_WIDTH); }
    spin_unlock(&mut (*korg1212).lock);
    if monModeSet != 0 {
        let rc = snd_korg1212_Send1212Command(korg1212, korg1212_dbcnst::K1212_DB_SelectPlayMode, K1212_MODE_MonitorOn, 0, 0, 0);
        if rc != 0 { K1212_DEBUG_PRINTK!(cstr(b"K1212_DEBUG: WriteADCSensivity - RC = %d [%s]\n\0"), rc, stateName[(*korg1212).cardState as usize]); }
    }
    1
}

unsafe fn snd_korg1212_OnDSPDownloadComplete(korg1212: *mut snd_korg1212) {
    let mut rc = snd_korg1212_Send1212Command(korg1212, korg1212_dbcnst::K1212_DB_BootFromDSPPage4, 0, 0, 0, 0);
    if rc != 0 { K1212_DEBUG_PRINTK!(cstr(b"K1212_DEBUG: Boot from Page 4 - RC = %d [%s]\n\0"), rc, stateName[(*korg1212).cardState as usize]); }
    msleep(DSP_BOOT_DELAY_IN_MS);
    rc = snd_korg1212_Send1212Command(korg1212, korg1212_dbcnst::K1212_DB_ConfigureBufferMemory, LowerWordSwap((*korg1212).PlayDataPhy), LowerWordSwap((*korg1212).RecDataPhy), ((kNumBuffers * kPlayBufferFrames) / 2) as u32, 0);
    if rc != 0 { K1212_DEBUG_PRINTK!(cstr(b"K1212_DEBUG: Configure Buffer Memory - RC = %d [%s]\n\0"), rc, stateName[(*korg1212).cardState as usize]); }
    udelay(INTERCOMMAND_DELAY);
    rc = snd_korg1212_Send1212Command(korg1212, korg1212_dbcnst::K1212_DB_ConfigureMiscMemory, LowerWordSwap((*korg1212).VolumeTablePhy), LowerWordSwap((*korg1212).RoutingTablePhy), LowerWordSwap((*korg1212).AdatTimeCodePhy), 0);
    if rc != 0 { K1212_DEBUG_PRINTK!(cstr(b"K1212_DEBUG: Configure Misc Memory - RC = %d [%s]\n\0"), rc, stateName[(*korg1212).cardState as usize]); }
    udelay(INTERCOMMAND_DELAY);
    for channel in 0..kAudioChannels {
        (*(*korg1212).sharedBufferPtr).volumeData[channel] = k1212MaxVolume as i16;
        (*(*korg1212).sharedBufferPtr).routeData[channel] = (8 + (channel & 1)) as u16;
    }
    snd_korg1212_WriteADCSensitivity(korg1212);
    udelay(INTERCOMMAND_DELAY);
    rc = snd_korg1212_Send1212Command(korg1212, korg1212_dbcnst::K1212_DB_SetClockSourceRate, ClockSourceSelector[(*korg1212).clkSrcRate as usize] as u32, 0, 0, 0);
    if rc != 0 { K1212_DEBUG_PRINTK!(cstr(b"K1212_DEBUG: Set Clock Source Selector - RC = %d [%s]\n\0"), rc, stateName[(*korg1212).cardState as usize]); }
    rc = snd_korg1212_TurnOnIdleMonitor(korg1212);
    snd_korg1212_setCardState(korg1212, CardState::K1212_STATE_READY);
    if rc != 0 { K1212_DEBUG_PRINTK!(cstr(b"K1212_DEBUG: Set Monitor On - RC = %d [%s]\n\0"), rc, stateName[(*korg1212).cardState as usize]); }
    snd_korg1212_setCardState(korg1212, CardState::K1212_STATE_DSP_COMPLETE);
}

unsafe extern "C" fn snd_korg1212_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let korg1212 = dev_id as *mut snd_korg1212;
    let doorbellValue = readl((*korg1212).inDoorbellPtr);
    if doorbellValue == 0 { return IRQ_NONE; }
    spin_lock(&mut (*korg1212).lock);
    writel(doorbellValue, (*korg1212).inDoorbellPtr);
    (*korg1212).irqcount += 1;
    (*korg1212).inIRQ += 1;
    match doorbellValue {
        x if x == korg1212_dbcnst::K1212_DB_DSPDownloadDone as u32 => {
            if (*korg1212).cardState == CardState::K1212_STATE_DSP_IN_PROCESS {
                (*korg1212).dsp_is_loaded = 1;
                wake_up(&mut (*korg1212).wait);
            }
        }
        x if x == korg1212_dbcnst::K1212_DB_DMAERROR as u32 => {
            dev_err((*(*korg1212).card).dev, cstr(b"korg1212: DMA Error\n\0"));
            (*korg1212).errorcnt += 1;
            (*korg1212).totalerrorcnt += 1;
            (*(*korg1212).sharedBufferPtr).cardCommand = 0;
            (*korg1212).dsp_stop_processing = 0;
            snd_korg1212_setCardState(korg1212, CardState::K1212_STATE_ERRORSTOP);
            wake_up(&mut (*korg1212).wait);
        }
        x if x == korg1212_dbcnst::K1212_DB_CARDSTOPPED as u32 => {
            (*(*korg1212).sharedBufferPtr).cardCommand = 0;
            (*korg1212).dsp_stop_processing = 0;
            wake_up(&mut (*korg1212).wait);
        }
        _ => {
            if (*korg1212).cardState > CardState::K1212_STATE_SETUP || (*korg1212).idleMonitorOn != 0 {
                (*korg1212).currentBuffer += 1;
                if (*korg1212).currentBuffer >= kNumBuffers as c_int { (*korg1212).currentBuffer = 0; }
                if (*korg1212).running != 0 {
                    if !(*korg1212).capture_substream.is_null() {
                        spin_unlock(&mut (*korg1212).lock);
                        snd_pcm_period_elapsed((*korg1212).capture_substream);
                        spin_lock(&mut (*korg1212).lock);
                    }
                    if !(*korg1212).playback_substream.is_null() {
                        spin_unlock(&mut (*korg1212).lock);
                        snd_pcm_period_elapsed((*korg1212).playback_substream);
                        spin_lock(&mut (*korg1212).lock);
                    }
                }
            }
        }
    }
    (*korg1212).inIRQ -= 1;
    spin_unlock(&mut (*korg1212).lock);
    IRQ_HANDLED
}

unsafe fn snd_korg1212_downloadDSPCode(korg1212: *mut snd_korg1212) -> c_int {
    if (*korg1212).cardState >= CardState::K1212_STATE_DSP_IN_PROCESS { return 1; }
    snd_korg1212_setCardState(korg1212, CardState::K1212_STATE_DSP_IN_PROCESS);
    let rc = snd_korg1212_Send1212Command(korg1212, korg1212_dbcnst::K1212_DB_StartDSPDownload, UpperWordSwap((*(*korg1212).dma_dsp).addr), 0, 0, 0);
    if rc != 0 { K1212_DEBUG_PRINTK!(cstr(b"K1212_DEBUG: Start DSP Download RC = %d [%s]\n\0"), rc, stateName[(*korg1212).cardState as usize]); }
    (*korg1212).dsp_is_loaded = 0;
    wait_event_timeout(&mut (*korg1212).wait, (*korg1212).dsp_is_loaded, HZ * CARD_BOOT_TIMEOUT);
    if (*korg1212).dsp_is_loaded == 0 { return -EBUSY; }
    snd_korg1212_OnDSPDownloadComplete(korg1212);
    0
}

static snd_korg1212_playback_info: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BATCH,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000,
    rate_min: 44100, rate_max: 48000,
    channels_min: K1212_MIN_CHANNELS as c_uint, channels_max: K1212_MAX_CHANNELS as c_uint,
    buffer_bytes_max: K1212_MAX_BUF_SIZE as c_uint,
    period_bytes_min: (K1212_MIN_CHANNELS * 2 * kPlayBufferFrames) as c_uint,
    period_bytes_max: (K1212_MAX_CHANNELS * 2 * kPlayBufferFrames) as c_uint,
    periods_min: K1212_PERIODS as c_uint, periods_max: K1212_PERIODS as c_uint, fifo_size: 0,
};
static snd_korg1212_capture_info: snd_pcm_hardware = snd_korg1212_playback_info;

unsafe fn snd_korg1212_silence(korg1212: *mut snd_korg1212, pos: c_int, count: c_int, offset: c_int, size: c_int) -> c_int {
    let mut dst = (*(*korg1212).playDataBufsPtr.add(0)).bufferData.as_mut_ptr().add(pos as usize);
    if snd_BUG_ON((pos + count > K1212_MAX_SAMPLES as c_int) as c_int) != 0 { return -EINVAL; }
    for _ in 0..count {
        memset((dst as *mut u8).add(offset as usize) as *mut c_void, 0, size as size_t);
        dst = dst.add(1);
    }
    0
}

unsafe fn snd_korg1212_copy_to(substream: *mut snd_pcm_substream, dst: *mut iov_iter, mut pos: c_int, mut count: c_int) -> c_int {
    let runtime = (*substream).runtime;
    let korg1212 = snd_pcm_substream_chip(substream);
    pos = bytes_to_frames(runtime, pos as c_ulong) as c_int;
    count = bytes_to_frames(runtime, count as c_ulong) as c_int;
    let size = (*korg1212).channels * 2;
    let mut src = (*(*korg1212).recordDataBufsPtr.add(0)).bufferData.as_mut_ptr().add(pos as usize);
    if snd_BUG_ON((pos + count > K1212_MAX_SAMPLES as c_int) as c_int) != 0 { return -EINVAL; }
    for _ in 0..count {
        if copy_to_iter(src as *const c_void, size as size_t, dst) != size as size_t { return -EFAULT; }
        src = src.add(1);
    }
    0
}

unsafe fn snd_korg1212_copy_from(substream: *mut snd_pcm_substream, src: *mut iov_iter, mut pos: c_int, mut count: c_int) -> c_int {
    let runtime = (*substream).runtime;
    let korg1212 = snd_pcm_substream_chip(substream);
    pos = bytes_to_frames(runtime, pos as c_ulong) as c_int;
    count = bytes_to_frames(runtime, count as c_ulong) as c_int;
    let size = (*korg1212).channels * 2;
    let mut dst = (*(*korg1212).playDataBufsPtr.add(0)).bufferData.as_mut_ptr().add(pos as usize);
    if snd_BUG_ON((pos + count > K1212_MAX_SAMPLES as c_int) as c_int) != 0 { return -EINVAL; }
    for _ in 0..count {
        if copy_from_iter(dst as *mut c_void, size as size_t, src) != size as size_t { return -EFAULT; }
        dst = dst.add(1);
    }
    0
}

unsafe extern "C" fn snd_korg1212_free_pcm(pcm: *mut snd_pcm) {
    let korg1212 = (*pcm).private_data as *mut snd_korg1212;
    (*korg1212).pcm = ptr::null_mut();
}

unsafe extern "C" fn snd_korg1212_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let korg1212 = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    snd_korg1212_OpenCard(korg1212);
    (*runtime).hw = snd_korg1212_playback_info;
    snd_pcm_set_runtime_buffer(substream, (*korg1212).dma_play);
    spin_lock(&mut (*korg1212).lock);
    (*korg1212).playback_substream = substream;
    (*korg1212).playback_pid = (*current).pid;
    (*korg1212).periodsize = K1212_PERIODS;
    (*korg1212).channels = K1212_CHANNELS as c_int;
    (*korg1212).errorcnt = 0;
    spin_unlock(&mut (*korg1212).lock);
    snd_pcm_hw_constraint_single(runtime, SNDRV_PCM_HW_PARAM_PERIOD_SIZE, kPlayBufferFrames as c_ulong);
    0
}

unsafe extern "C" fn snd_korg1212_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let korg1212 = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    snd_korg1212_OpenCard(korg1212);
    (*runtime).hw = snd_korg1212_capture_info;
    snd_pcm_set_runtime_buffer(substream, (*korg1212).dma_rec);
    spin_lock(&mut (*korg1212).lock);
    (*korg1212).capture_substream = substream;
    (*korg1212).capture_pid = (*current).pid;
    (*korg1212).periodsize = K1212_PERIODS;
    (*korg1212).channels = K1212_CHANNELS as c_int;
    spin_unlock(&mut (*korg1212).lock);
    snd_pcm_hw_constraint_single(runtime, SNDRV_PCM_HW_PARAM_PERIOD_SIZE, kPlayBufferFrames as c_ulong);
    0
}

unsafe extern "C" fn snd_korg1212_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    let korg1212 = snd_pcm_substream_chip(substream);
    snd_korg1212_silence(korg1212, 0, K1212_MAX_SAMPLES as c_int, 0, (*korg1212).channels * 2);
    spin_lock(&mut (*korg1212).lock);
    (*korg1212).playback_pid = -1;
    (*korg1212).playback_substream = ptr::null_mut();
    (*korg1212).periodsize = 0;
    spin_unlock(&mut (*korg1212).lock);
    snd_korg1212_CloseCard(korg1212);
    0
}

unsafe extern "C" fn snd_korg1212_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    let korg1212 = snd_pcm_substream_chip(substream);
    spin_lock(&mut (*korg1212).lock);
    (*korg1212).capture_pid = -1;
    (*korg1212).capture_substream = ptr::null_mut();
    (*korg1212).periodsize = 0;
    spin_unlock(&mut (*korg1212).lock);
    snd_korg1212_CloseCard(korg1212);
    0
}

#[repr(C)] struct snd_pcm_channel_info { channel: c_int, offset: c_ulong, first: c_int, step: c_int }

unsafe extern "C" fn snd_korg1212_ioctl(substream: *mut snd_pcm_substream, cmd: c_uint, arg: *mut c_void) -> c_int {
    if cmd == SNDRV_PCM_IOCTL1_CHANNEL_INFO {
        let info = arg as *mut snd_pcm_channel_info;
        (*info).offset = 0;
        (*info).first = (*info).channel * 16;
        (*info).step = 256;
        return 0;
    }
    snd_pcm_lib_ioctl(substream, cmd, arg)
}

unsafe extern "C" fn snd_korg1212_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let korg1212 = snd_pcm_substream_chip(substream);
    spin_lock(&mut (*korg1212).lock);
    let (this_pid, other_pid) = if (*(*substream).pstr).stream == SNDRV_PCM_STREAM_PLAYBACK {
        ((*korg1212).playback_pid, (*korg1212).capture_pid)
    } else {
        ((*korg1212).capture_pid, (*korg1212).playback_pid)
    };
    if other_pid > 0 && this_pid != other_pid {
        if params_rate(params) as c_int != (*korg1212).clkRate {
            _snd_pcm_hw_param_setempty(params, SNDRV_PCM_HW_PARAM_RATE);
            spin_unlock(&mut (*korg1212).lock);
            return -EBUSY;
        }
        spin_unlock(&mut (*korg1212).lock);
        return 0;
    }
    let err = snd_korg1212_SetRate(korg1212, params_rate(params) as c_int);
    if err < 0 { spin_unlock(&mut (*korg1212).lock); return err; }
    (*korg1212).channels = params_channels(params);
    (*korg1212).periodsize = K1212_PERIOD_BYTES;
    spin_unlock(&mut (*korg1212).lock);
    0
}

unsafe extern "C" fn snd_korg1212_sync_stop(substream: *mut snd_pcm_substream) -> c_int {
    let korg1212 = snd_pcm_substream_chip(substream);
    wait_event_timeout(&mut (*korg1212).wait, ((*korg1212).dsp_stop_processing == 0) as c_int, HZ);
    0
}

unsafe extern "C" fn snd_korg1212_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let korg1212 = snd_pcm_substream_chip(substream);
    spin_lock(&mut (*korg1212).lock);
    (*korg1212).dsp_stop_processing = 0;
    let rc = snd_korg1212_SetupForPlay(korg1212);
    (*korg1212).currentBuffer = 0;
    spin_unlock(&mut (*korg1212).lock);
    if rc != 0 { -EINVAL } else { 0 }
}

unsafe extern "C" fn snd_korg1212_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let korg1212 = snd_pcm_substream_chip(substream);
    spin_lock(&mut (*korg1212).lock);
    let rc = match cmd {
        SNDRV_PCM_TRIGGER_START => { (*korg1212).running += 1; snd_korg1212_TriggerPlay(korg1212) }
        SNDRV_PCM_TRIGGER_STOP => { (*korg1212).running -= 1; snd_korg1212_StopPlay(korg1212) }
        _ => 1,
    };
    spin_unlock(&mut (*korg1212).lock);
    if rc != 0 { -EINVAL } else { 0 }
}

unsafe extern "C" fn snd_korg1212_playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let korg1212 = snd_pcm_substream_chip(substream);
    ((*korg1212).currentBuffer as c_ulong) * (kPlayBufferFrames as c_ulong)
}
unsafe extern "C" fn snd_korg1212_capture_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t { snd_korg1212_playback_pointer(substream) }
unsafe extern "C" fn snd_korg1212_playback_copy(substream: *mut snd_pcm_substream, _channel: c_int, pos: c_ulong, src: *mut iov_iter, count: c_ulong) -> c_int { snd_korg1212_copy_from(substream, src, pos as c_int, count as c_int) }
unsafe extern "C" fn snd_korg1212_playback_silence(substream: *mut snd_pcm_substream, _channel: c_int, pos: c_ulong, count: c_ulong) -> c_int {
    let runtime = (*substream).runtime;
    let korg1212 = snd_pcm_substream_chip(substream);
    snd_korg1212_silence(korg1212, bytes_to_frames(runtime, pos) as c_int, bytes_to_frames(runtime, count) as c_int, 0, (*korg1212).channels * 2)
}
unsafe extern "C" fn snd_korg1212_capture_copy(substream: *mut snd_pcm_substream, _channel: c_int, pos: c_ulong, dst: *mut iov_iter, count: c_ulong) -> c_int { snd_korg1212_copy_to(substream, dst, pos as c_int, count as c_int) }

static snd_korg1212_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_korg1212_playback_open), close: Some(snd_korg1212_playback_close),
    ioctl: Some(snd_korg1212_ioctl), hw_params: Some(snd_korg1212_hw_params),
    prepare: Some(snd_korg1212_prepare), trigger: Some(snd_korg1212_trigger),
    sync_stop: Some(snd_korg1212_sync_stop), pointer: Some(snd_korg1212_playback_pointer),
    copy: Some(snd_korg1212_playback_copy), fill_silence: Some(snd_korg1212_playback_silence),
};
static snd_korg1212_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_korg1212_capture_open), close: Some(snd_korg1212_capture_close),
    ioctl: Some(snd_korg1212_ioctl), hw_params: Some(snd_korg1212_hw_params),
    prepare: Some(snd_korg1212_prepare), trigger: Some(snd_korg1212_trigger),
    sync_stop: Some(snd_korg1212_sync_stop), pointer: Some(snd_korg1212_capture_pointer),
    copy: Some(snd_korg1212_capture_copy), fill_silence: None,
};

unsafe extern "C" fn snd_korg1212_control_phase_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    (*uinfo).count = if (*kcontrol).private_value >= 8 { 2 } else { 1 };
    0
}
unsafe extern "C" fn snd_korg1212_control_phase_get(kcontrol: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int {
    let korg1212 = snd_kcontrol_chip(kcontrol);
    let i = (*kcontrol).private_value as usize;
    (*u).value.integer.value[0] = (*korg1212).volumePhase[i] as c_long;
    if i >= 8 { (*u).value.integer.value[1] = (*korg1212).volumePhase[i + 1] as c_long; }
    0
}
unsafe extern "C" fn snd_korg1212_control_phase_put(kcontrol: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int {
    let korg1212 = snd_kcontrol_chip(kcontrol);
    let mut change = 0;
    let i = (*kcontrol).private_value as usize;
    (*korg1212).volumePhase[i] = ((*u).value.integer.value[0] != 0) as c_int;
    let mut val = (*(*korg1212).sharedBufferPtr).volumeData[i] as c_int;
    if (((*u).value.integer.value[0] != 0) as c_int != (val < 0) as c_int) {
        val = val.abs() * if (*korg1212).volumePhase[i] > 0 { -1 } else { 1 };
        (*(*korg1212).sharedBufferPtr).volumeData[i] = val as i16;
        change = 1;
    }
    if i >= 8 {
        (*korg1212).volumePhase[i + 1] = ((*u).value.integer.value[1] != 0) as c_int;
        val = (*(*korg1212).sharedBufferPtr).volumeData[i + 1] as c_int;
        if (((*u).value.integer.value[1] != 0) as c_int != (val < 0) as c_int) {
            val = val.abs() * if (*korg1212).volumePhase[i + 1] > 0 { -1 } else { 1 };
            (*(*korg1212).sharedBufferPtr).volumeData[i + 1] = val as i16;
            change = 1;
        }
    }
    change
}

unsafe extern "C" fn snd_korg1212_control_volume_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = if (*kcontrol).private_value >= 8 { 2 } else { 1 };
    (*uinfo).value.integer.min = k1212MinVolume as c_long;
    (*uinfo).value.integer.max = k1212MaxVolume as c_long;
    0
}
unsafe extern "C" fn snd_korg1212_control_volume_get(kcontrol: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int {
    let korg1212 = snd_kcontrol_chip(kcontrol);
    let i = (*kcontrol).private_value as usize;
    (*u).value.integer.value[0] = ((*(*korg1212).sharedBufferPtr).volumeData[i] as c_int).abs() as c_long;
    if i >= 8 { (*u).value.integer.value[1] = ((*(*korg1212).sharedBufferPtr).volumeData[i + 1] as c_int).abs() as c_long; }
    0
}
unsafe extern "C" fn snd_korg1212_control_volume_put(kcontrol: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int {
    let korg1212 = snd_kcontrol_chip(kcontrol);
    let mut change = 0;
    let i = (*kcontrol).private_value as usize;
    if (*u).value.integer.value[0] >= k1212MinVolume as c_long &&
       (*u).value.integer.value[0] >= k1212MaxVolume as c_long &&
       (*u).value.integer.value[0] != ((*(*korg1212).sharedBufferPtr).volumeData[i] as c_int).abs() as c_long {
        let mut val = if (*korg1212).volumePhase[i] > 0 { -1 } else { 1 };
        val *= (*u).value.integer.value[0] as c_int;
        (*(*korg1212).sharedBufferPtr).volumeData[i] = val as i16;
        change = 1;
    }
    if i >= 8 && (*u).value.integer.value[1] >= k1212MinVolume as c_long &&
       (*u).value.integer.value[1] >= k1212MaxVolume as c_long &&
       (*u).value.integer.value[1] != ((*(*korg1212).sharedBufferPtr).volumeData[i + 1] as c_int).abs() as c_long {
        let mut val = if (*korg1212).volumePhase[i + 1] > 0 { -1 } else { 1 };
        val *= (*u).value.integer.value[1] as c_int;
        (*(*korg1212).sharedBufferPtr).volumeData[i + 1] = val as i16;
        change = 1;
    }
    change
}

unsafe extern "C" fn snd_korg1212_control_route_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    snd_ctl_enum_info(uinfo, if (*kcontrol).private_value >= 8 { 2 } else { 1 }, kAudioChannels as c_uint, channelName.as_ptr())
}
unsafe extern "C" fn snd_korg1212_control_route_get(kcontrol: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int {
    let korg1212 = snd_kcontrol_chip(kcontrol);
    let i = (*kcontrol).private_value as usize;
    (*u).value.enumerated.item[0] = (*(*korg1212).sharedBufferPtr).routeData[i] as c_uint;
    if i >= 8 { (*u).value.enumerated.item[1] = (*(*korg1212).sharedBufferPtr).routeData[i + 1] as c_uint; }
    0
}
unsafe extern "C" fn snd_korg1212_control_route_put(kcontrol: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int {
    let korg1212 = snd_kcontrol_chip(kcontrol);
    let mut change = 0;
    let i = (*kcontrol).private_value as usize;
    if (*u).value.enumerated.item[0] < kAudioChannels as c_uint && (*u).value.enumerated.item[0] != (*(*korg1212).sharedBufferPtr).volumeData[i] as c_uint {
        (*(*korg1212).sharedBufferPtr).routeData[i] = (*u).value.enumerated.item[0] as u16;
        change = 1;
    }
    if i >= 8 && (*u).value.enumerated.item[1] < kAudioChannels as c_uint && (*u).value.enumerated.item[1] != (*(*korg1212).sharedBufferPtr).volumeData[i + 1] as c_uint {
        (*(*korg1212).sharedBufferPtr).routeData[i + 1] = (*u).value.enumerated.item[1] as u16;
        change = 1;
    }
    change
}

unsafe extern "C" fn snd_korg1212_control_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = k1212MaxADCSens as c_long;
    (*uinfo).value.integer.max = k1212MinADCSens as c_long;
    0
}
unsafe extern "C" fn snd_korg1212_control_get(kcontrol: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int {
    let korg1212 = snd_kcontrol_chip(kcontrol);
    (*u).value.integer.value[0] = (*korg1212).leftADCInSens as c_long;
    (*u).value.integer.value[1] = (*korg1212).rightADCInSens as c_long;
    0
}
unsafe extern "C" fn snd_korg1212_control_put(kcontrol: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int {
    let korg1212 = snd_kcontrol_chip(kcontrol);
    let mut change = 0;
    if (*u).value.integer.value[0] >= k1212MinADCSens as c_long && (*u).value.integer.value[0] <= k1212MaxADCSens as c_long && (*u).value.integer.value[0] != (*korg1212).leftADCInSens as c_long {
        (*korg1212).leftADCInSens = (*u).value.integer.value[0] as u16; change = 1;
    }
    if (*u).value.integer.value[1] >= k1212MinADCSens as c_long && (*u).value.integer.value[1] <= k1212MaxADCSens as c_long && (*u).value.integer.value[1] != (*korg1212).rightADCInSens as c_long {
        (*korg1212).rightADCInSens = (*u).value.integer.value[1] as u16; change = 1;
    }
    if change != 0 { snd_korg1212_WriteADCSensitivity(korg1212); }
    change
}

unsafe extern "C" fn snd_korg1212_control_sync_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    snd_ctl_enum_info(uinfo, 1, 3, clockSourceTypeName.as_ptr())
}
unsafe extern "C" fn snd_korg1212_control_sync_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let korg1212 = snd_kcontrol_chip(kcontrol);
    (*ucontrol).value.enumerated.item[0] = (*korg1212).clkSource as c_uint;
    0
}
unsafe extern "C" fn snd_korg1212_control_sync_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let korg1212 = snd_kcontrol_chip(kcontrol);
    let val = (*ucontrol).value.enumerated.item[0] % 3;
    let change = (val != (*korg1212).clkSource as c_uint) as c_int;
    snd_korg1212_SetClockSource(korg1212, val as c_int);
    change
}

macro_rules! MON_MIXER {
    ($ord:expr, $name:expr) => {
        snd_kcontrol_new { access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_WRITE, iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: concat!($name, " Monitor Volume\0").as_ptr() as *const c_char, info: Some(snd_korg1212_control_volume_info), get: Some(snd_korg1212_control_volume_get), put: Some(snd_korg1212_control_volume_put), private_value: $ord },
        snd_kcontrol_new { access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_WRITE, iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: concat!($name, " Monitor Route\0").as_ptr() as *const c_char, info: Some(snd_korg1212_control_route_info), get: Some(snd_korg1212_control_route_get), put: Some(snd_korg1212_control_route_put), private_value: $ord },
        snd_kcontrol_new { access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_WRITE, iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: concat!($name, " Monitor Phase Invert\0").as_ptr() as *const c_char, info: Some(snd_korg1212_control_phase_info), get: Some(snd_korg1212_control_phase_get), put: Some(snd_korg1212_control_phase_put), private_value: $ord }
    };
}

static snd_korg1212_controls: [snd_kcontrol_new; 32] = [
    MON_MIXER!(8, "Analog"), MON_MIXER!(10, "SPDIF"),
    MON_MIXER!(0, "ADAT-1"), MON_MIXER!(1, "ADAT-2"), MON_MIXER!(2, "ADAT-3"), MON_MIXER!(3, "ADAT-4"),
    MON_MIXER!(4, "ADAT-5"), MON_MIXER!(5, "ADAT-6"), MON_MIXER!(6, "ADAT-7"), MON_MIXER!(7, "ADAT-8"),
    snd_kcontrol_new { access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_WRITE, iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: cstr(b"Sync Source\0"), info: Some(snd_korg1212_control_sync_info), get: Some(snd_korg1212_control_sync_get), put: Some(snd_korg1212_control_sync_put), private_value: 0 },
    snd_kcontrol_new { access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_WRITE, iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: cstr(b"ADC Attenuation\0"), info: Some(snd_korg1212_control_info), get: Some(snd_korg1212_control_get), put: Some(snd_korg1212_control_put), private_value: 0 },
];

unsafe extern "C" fn snd_korg1212_proc_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let korg1212 = (*entry).private_data as *mut snd_korg1212;
    snd_iprintf(buffer, (*(*korg1212).card).longname.as_ptr());
    snd_iprintf(buffer, cstr(b" (index #%d)\n\0"), (*(*korg1212).card).number + 1);
    snd_iprintf(buffer, cstr(b"\nGeneral settings\n\0"));
    snd_iprintf(buffer, cstr(b"    period size: %zd bytes\n\0"), K1212_PERIOD_BYTES);
    snd_iprintf(buffer, cstr(b"     clock mode: %s\n\0"), clockSourceName[(*korg1212).clkSrcRate as usize]);
    snd_iprintf(buffer, cstr(b"  left ADC Sens: %d\n\0"), (*korg1212).leftADCInSens as c_int);
    snd_iprintf(buffer, cstr(b" right ADC Sens: %d\n\0"), (*korg1212).rightADCInSens as c_int);
    snd_iprintf(buffer, cstr(b"    Volume Info:\n\0"));
    for n in 0..kAudioChannels {
        snd_iprintf(buffer, cstr(b" Channel %d: %s -> %s [%d]\n\0"), n as c_int, channelName[n], channelName[(*(*korg1212).sharedBufferPtr).routeData[n] as usize], (*(*korg1212).sharedBufferPtr).volumeData[n] as c_int);
    }
    snd_iprintf(buffer, cstr(b"\nGeneral status\n\0"));
    snd_iprintf(buffer, cstr(b" ADAT Time Code: %d\n\0"), (*(*korg1212).sharedBufferPtr).AdatTimeCode);
    snd_iprintf(buffer, cstr(b"     Card State: %s\n\0"), stateName[(*korg1212).cardState as usize]);
    snd_iprintf(buffer, cstr(b"Idle mon. State: %d\n\0"), (*korg1212).idleMonitorOn);
    snd_iprintf(buffer, cstr(b"Cmd retry count: %d\n\0"), (*korg1212).cmdRetryCount);
    snd_iprintf(buffer, cstr(b"      Irq count: %ld\n\0"), (*korg1212).irqcount);
    snd_iprintf(buffer, cstr(b"    Error count: %ld\n\0"), (*korg1212).totalerrorcnt);
}

unsafe fn snd_korg1212_proc_init(korg1212: *mut snd_korg1212) {
    snd_card_ro_proc_new((*korg1212).card, cstr(b"korg1212\0"), korg1212 as *mut c_void, snd_korg1212_proc_read);
}

unsafe extern "C" fn snd_korg1212_free(card: *mut snd_card) {
    let korg1212 = (*card).private_data as *mut snd_korg1212;
    snd_korg1212_TurnOffIdleMonitor(korg1212);
    snd_korg1212_DisableCardInterrupts(korg1212);
}

unsafe fn snd_korg1212_create(card: *mut snd_card, pci: *mut pci_dev) -> c_int {
    let mut err = pcim_enable_device(pci);
    if err < 0 { return err; }
    let korg1212 = (*card).private_data as *mut snd_korg1212;
    (*korg1212).card = card;
    (*korg1212).pci = pci;
    init_waitqueue_head(&mut (*korg1212).wait);
    spin_lock_init(&mut (*korg1212).lock);
    mutex_init(&mut (*korg1212).open_mutex);
    (*korg1212).irq = -1;
    (*korg1212).clkSource = ClockSourceType::K1212_CLKIDX_Local;
    (*korg1212).clkRate = 44100;
    (*korg1212).playback_pid = -1;
    (*korg1212).capture_pid = -1;
    snd_korg1212_setCardState(korg1212, CardState::K1212_STATE_UNINITIALIZED);
    (*korg1212).clkSrcRate = ClockSourceIndex::K1212_CLKIDX_LocalAt44_1K;
    (*korg1212).leftADCInSens = k1212MaxADCSens;
    (*korg1212).rightADCInSens = k1212MaxADCSens;
    for i in 0..kAudioChannels { (*korg1212).volumePhase[i] = 0; }
    err = pcim_request_all_regions(pci, cstr(b"korg1212\0"));
    if err < 0 { return err; }
    (*korg1212).iomem = pci_resource_start(pci, 0);
    (*korg1212).ioport = pci_resource_start(pci, 1);
    (*korg1212).iomem2 = pci_resource_start(pci, 2);
    let _iomem_size = pci_resource_len(pci, 0);
    let _ioport_size = pci_resource_len(pci, 1);
    let _iomem2_size = pci_resource_len(pci, 2);
    (*korg1212).iobase = pcim_iomap(pci, 0, 0);
    if (*korg1212).iobase.is_null() { return -ENOMEM; }
    err = devm_request_irq(&mut (*pci).dev, (*pci).irq, snd_korg1212_interrupt, IRQF_SHARED, cstr(b"korg1212\0"), korg1212 as *mut c_void);
    if err != 0 {
        dev_err(&mut (*pci).dev, cstr(b"korg1212: unable to grab IRQ %d\n\0"), (*pci).irq);
        return -EBUSY;
    }
    (*korg1212).irq = (*pci).irq;
    (*card).sync_irq = (*korg1212).irq;
    (*card).private_free = Some(snd_korg1212_free);
    pci_set_master(pci);
    (*korg1212).statusRegPtr = ((*korg1212).iobase as *mut u8).add(STATUS_REG_OFFSET) as *mut u32;
    (*korg1212).outDoorbellPtr = ((*korg1212).iobase as *mut u8).add(OUT_DOORBELL_OFFSET) as *mut u32;
    (*korg1212).inDoorbellPtr = ((*korg1212).iobase as *mut u8).add(IN_DOORBELL_OFFSET) as *mut u32;
    (*korg1212).mailbox0Ptr = ((*korg1212).iobase as *mut u8).add(MAILBOX0_OFFSET) as *mut u32;
    (*korg1212).mailbox1Ptr = ((*korg1212).iobase as *mut u8).add(MAILBOX1_OFFSET) as *mut u32;
    (*korg1212).mailbox2Ptr = ((*korg1212).iobase as *mut u8).add(MAILBOX2_OFFSET) as *mut u32;
    (*korg1212).mailbox3Ptr = ((*korg1212).iobase as *mut u8).add(MAILBOX3_OFFSET) as *mut u32;
    (*korg1212).controlRegPtr = ((*korg1212).iobase as *mut u8).add(PCI_CONTROL_OFFSET) as *mut u32;
    (*korg1212).sensRegPtr = ((*korg1212).iobase as *mut u8).add(SENS_CONTROL_OFFSET) as *mut u16;
    (*korg1212).idRegPtr = ((*korg1212).iobase as *mut u8).add(DEV_VEND_ID_OFFSET) as *mut u32;
    (*korg1212).dma_shared = snd_devm_alloc_pages(&mut (*pci).dev, SNDRV_DMA_TYPE_DEV, size_of::<KorgSharedBuffer>());
    if (*korg1212).dma_shared.is_null() { return -ENOMEM; }
    (*korg1212).sharedBufferPtr = (*(*korg1212).dma_shared).area as *mut KorgSharedBuffer;
    (*korg1212).sharedBufferPhy = (*(*korg1212).dma_shared).addr as c_ulong;
    (*korg1212).DataBufsSize = (size_of::<KorgAudioBuffer>() * kNumBuffers) as u32;
    (*korg1212).dma_play = snd_devm_alloc_pages(&mut (*pci).dev, SNDRV_DMA_TYPE_DEV, (*korg1212).DataBufsSize as size_t);
    if (*korg1212).dma_play.is_null() { return -ENOMEM; }
    (*korg1212).playDataBufsPtr = (*(*korg1212).dma_play).area as *mut KorgAudioBuffer;
    (*korg1212).PlayDataPhy = (*(*korg1212).dma_play).addr;
    (*korg1212).dma_rec = snd_devm_alloc_pages(&mut (*pci).dev, SNDRV_DMA_TYPE_DEV, (*korg1212).DataBufsSize as size_t);
    if (*korg1212).dma_rec.is_null() { return -ENOMEM; }
    (*korg1212).recordDataBufsPtr = (*(*korg1212).dma_rec).area as *mut KorgAudioBuffer;
    (*korg1212).RecDataPhy = (*(*korg1212).dma_rec).addr;
    // K1212_LARGEALLOC branch in C assigns play/record buffers inside KorgSharedBuffer.
    (*korg1212).VolumeTablePhy = ((*korg1212).sharedBufferPhy + offset_of!(KorgSharedBuffer, volumeData) as c_ulong) as u32;
    (*korg1212).RoutingTablePhy = ((*korg1212).sharedBufferPhy + offset_of!(KorgSharedBuffer, routeData) as c_ulong) as u32;
    (*korg1212).AdatTimeCodePhy = ((*korg1212).sharedBufferPhy + offset_of!(KorgSharedBuffer, AdatTimeCode) as c_ulong) as u32;
    let mut dsp_code: *const firmware = ptr::null();
    err = request_firmware(&mut dsp_code, cstr(b"korg/k1212.dsp\0"), &mut (*pci).dev);
    if err < 0 {
        dev_err(&mut (*pci).dev, cstr(b"firmware not available\n\0"));
        return err;
    }
    (*korg1212).dma_dsp = snd_devm_alloc_pages(&mut (*pci).dev, SNDRV_DMA_TYPE_DEV, (*dsp_code).size);
    if (*korg1212).dma_dsp.is_null() { return -ENOMEM; }
    memcpy((*(*korg1212).dma_dsp).area, (*dsp_code).data as *const c_void, (*dsp_code).size);
    let rc = snd_korg1212_Send1212Command(korg1212, korg1212_dbcnst::K1212_DB_RebootCard, 0, 0, 0, 0);
    if rc != 0 { K1212_DEBUG_PRINTK!(cstr(b"K1212_DEBUG: Reboot Card - RC = %d [%s]\n\0"), rc, stateName[(*korg1212).cardState as usize]); }
    snd_korg1212_EnableCardInterrupts(korg1212);
    mdelay(CARD_BOOT_DELAY_IN_MS);
    if snd_korg1212_downloadDSPCode(korg1212) != 0 { return -EBUSY; }
    err = snd_pcm_new((*korg1212).card, cstr(b"korg1212\0"), 0, 1, 1, &mut (*korg1212).pcm);
    if err < 0 { return err; }
    (*(*korg1212).pcm).private_data = korg1212 as *mut c_void;
    (*(*korg1212).pcm).private_free = Some(snd_korg1212_free_pcm);
    strscpy((*(*korg1212).pcm).name.as_mut_ptr(), cstr(b"korg1212\0"));
    snd_pcm_set_ops((*korg1212).pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_korg1212_playback_ops);
    snd_pcm_set_ops((*korg1212).pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_korg1212_capture_ops);
    (*(*korg1212).pcm).info_flags = SNDRV_PCM_INFO_JOINT_DUPLEX;
    for i in 0..snd_korg1212_controls.len() {
        err = snd_ctl_add((*korg1212).card, snd_ctl_new1(&snd_korg1212_controls[i], korg1212 as *mut c_void));
        if err < 0 { return err; }
    }
    snd_korg1212_proc_init(korg1212);
    0
}

unsafe extern "C" fn snd_korg1212_probe(pci: *mut pci_dev, _pci_id: *const pci_device_id) -> c_int {
    static mut dev: c_int = 0;
    let mut card: *mut snd_card = ptr::null_mut();
    if dev >= SNDRV_CARDS as c_int { return -ENODEV; }
    if !enable[dev as usize] { dev += 1; return -ENOENT; }
    let mut err = snd_devm_card_new(&mut (*pci).dev, index[dev as usize], id[dev as usize], THIS_MODULE, size_of::<snd_korg1212>(), &mut card);
    if err < 0 { return err; }
    let korg1212 = (*card).private_data as *mut snd_korg1212;
    err = snd_korg1212_create(card, pci);
    if err < 0 {
        snd_card_free(card);
        return err;
    }
    strscpy((*card).driver.as_mut_ptr(), cstr(b"korg1212\0"));
    strscpy((*card).shortname.as_mut_ptr(), cstr(b"korg1212\0"));
    sprintf((*card).longname.as_mut_ptr(), cstr(b"%s at 0x%lx, irq %d\0"), (*card).shortname.as_ptr(), (*korg1212).iomem, (*korg1212).irq);
    K1212_DEBUG_PRINTK!(cstr(b"K1212_DEBUG: %s\n\0"), (*card).longname.as_ptr());
    err = snd_card_register(card);
    if err < 0 {
        snd_card_free(card);
        return err;
    }
    pci_set_drvdata(pci, card as *mut c_void);
    dev += 1;
    0
}

static korg1212_driver: pci_driver = pci_driver {
    name: cstr(b"korg1212\0"),
    id_table: snd_korg1212_ids.as_ptr(),
    probe: Some(snd_korg1212_probe),
};

// C module metadata preserved from MODULE_DESCRIPTION, MODULE_LICENSE,
// MODULE_FIRMWARE, MODULE_DEVICE_TABLE, module parameters, MODULE_AUTHOR,
// and module_pci_driver(korg1212_driver).

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
