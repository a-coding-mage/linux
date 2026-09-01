// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for RME Digi32, Digi32/8 and Digi32 PRO audio interfaces
 *
 *      Copyright (c) 2002-2004 Martin Langer <martin-langer@gmx.de>,
 *                              Pilo Chambert <pilo.c@wanadoo.fr>
 *
 *      Thanks to :        Anders Torger <torger@ludd.luth.se>,
 *                         Henk Hesselink <henk@anda.nl>
 *                         for writing the digi96-driver
 *                         and RME for all informations.
 *
 * ****************************************************************************
 *
 * Note #1 "Sek'd models" ................................... martin 2002-12-07
 *
 * Identical soundcards by Sek'd were labeled:
 * RME Digi 32     = Sek'd Prodif 32
 * RME Digi 32 Pro = Sek'd Prodif 96
 * RME Digi 32/8   = Sek'd Prodif Gold
 *
 * ****************************************************************************
 *
 * Note #2 "full duplex mode" ............................... martin 2002-12-07
 *
 * Full duplex doesn't work. All cards (32, 32/8, 32Pro) are working identical
 * in this mode. Rec data and play data are using the same buffer therefore. At
 * first you have got the playing bits in the buffer and then (after playing
 * them) they were overwitten by the captured sound of the CS8412/14. Both
 * modes (play/record) are running harmonically hand in hand in the same buffer
 * and you have only one start bit plus one interrupt bit to control this
 * paired action.
 * This is opposite to the latter rme96 where playing and capturing is totally
 * separated and so their full duplex mode is supported by alsa (using two
 * start bits and two interrupts for two different buffers).
 * But due to the wrong sequence of playing and capturing ALSA shows no solved
 * full duplex support for the rme32 at the moment. That's bad, but I'm not
 * able to solve it. Are you motivated enough to solve this problem now? Your
 * patch would be welcome!
 *
 * ****************************************************************************
 *
 * "The story after the long seeking" -- tiwai
 *
 * Ok, the situation regarding the full duplex is now improved a bit.
 * In the fullduplex mode (given by the module parameter), the hardware buffer
 * is split to halves for read and write directions at the DMA pointer.
 * That is, the half above the current DMA pointer is used for write, and
 * the half below is used for read.  To mangle this strange behavior, an
 * software intermediate buffer is introduced.  This is, of course, not good
 * from the viewpoint of the data transfer efficiency.  However, this allows
 * you to use arbitrary buffer sizes, instead of the fixed I/O buffer size.
 *
 * ****************************************************************************
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = ::core::primitive::u8;
type u32 = ::core::primitive::u32;
type size_t = usize;
type snd_pcm_format_t = c_int;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;

#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct pci_device_id { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct iov_iter { _private: [u8; 0] }
#[repr(C)] pub struct snd_info_entry { pub private_data: *mut c_void }
#[repr(C)] pub struct snd_info_buffer { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
    pub irq: c_int,
    pub device: c_int,
}

#[repr(C)]
pub struct snd_pcm_indirect {
    pub hw_buffer_size: size_t,
    pub sw_buffer_size: size_t,
    pub hw_queue_size: size_t,
    pub hw_data: size_t,
    pub sw_data: size_t,
    pub hw_io: size_t,
    pub hw_ready: size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hardware {
    pub info: u32,
    pub formats: u32,
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
pub struct snd_pcm_runtime {
    pub dma_area: *mut c_void,
    pub dma_addr: c_ulong,
    pub dma_bytes: size_t,
    pub hw: snd_pcm_hardware,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub sync_irq: c_int,
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    pub number: c_int,
    pub driver: *mut c_char,
    pub shortname: *mut c_char,
    pub longname: *mut c_char,
}

#[repr(C)]
pub struct snd_pcm {
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_pcm)>,
    pub name: *mut c_char,
    pub info_flags: u32,
}

#[repr(C)]
pub struct snd_kcontrol_volatile {
    pub access: u32,
}

#[repr(C)]
pub struct snd_ctl_elem_id { _private: [u8; 0] }

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
    pub vd: *mut snd_kcontrol_volatile,
    pub id: snd_ctl_elem_id,
}

#[repr(C)]
pub struct snd_aes_iec958 {
    pub status: [u8; 24],
    pub subcode: [u8; 147],
    pub pad: u8,
    pub dig_subframe: [u8; 4],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
    pub iec958: snd_aes_iec958,
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer { pub value: [c_long; 128] }
#[repr(C)]
pub struct snd_ctl_elem_value_enumerated { pub item: [u32; 128] }
type c_long = isize;

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_int,
    pub count: u32,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_int,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub access: u32,
    pub private_value: c_ulong,
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
    pub ack: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
}

#[repr(C)]
pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
}

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE_PNP: [bool; SNDRV_CARDS] = [true; SNDRV_CARDS];

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR; /* ID for this card */
static mut enable: [bool; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP; /* Enable this card */
static mut fullduplex: [bool; SNDRV_CARDS] = [false; SNDRV_CARDS]; // = {[0 ... (SNDRV_CARDS - 1)] = 1};

// module_param_array / MODULE_* metadata from the C source is preserved as module intent.

/* Defines for RME Digi32 series */
const RME32_SPDIF_NCHANNELS: u32 = 2;
/* Playback and capture buffer size */
const RME32_BUFFER_SIZE: size_t = 0x20000;
/* IO area size */
const RME32_IO_SIZE: size_t = 0x30000;
/* IO area offsets */
const RME32_IO_DATA_BUFFER: usize = 0x0;
const RME32_IO_CONTROL_REGISTER: usize = 0x20000;
const RME32_IO_GET_POS: usize = 0x20000;
const RME32_IO_CONFIRM_ACTION_IRQ: usize = 0x20004;
const RME32_IO_RESET_POS: usize = 0x20100;

const RME32_WCR_START: u32 = 1 << 0;    /* startbit */
const RME32_WCR_MONO: u32 = 1 << 1;     /* 0=stereo, 1=mono */
const RME32_WCR_MODE24: u32 = 1 << 2;   /* 0=16bit, 1=32bit */
const RME32_WCR_SEL: u32 = 1 << 3;      /* 0=input on output, 1=normal playback/capture */
const RME32_WCR_FREQ_0: u32 = 1 << 4;   /* frequency (play) */
const RME32_WCR_FREQ_1: u32 = 1 << 5;
const RME32_WCR_INP_0: u32 = 1 << 6;    /* input switch */
const RME32_WCR_INP_1: u32 = 1 << 7;
const RME32_WCR_RESET: u32 = 1 << 8;    /* Reset address */
const RME32_WCR_MUTE: u32 = 1 << 9;     /* digital mute for output */
const RME32_WCR_PRO: u32 = 1 << 10;     /* 1=professional, 0=consumer */
const RME32_WCR_DS_BM: u32 = 1 << 11;   /* 1=DoubleSpeed (only PRO-Version); 1=BlockMode (only Adat-Version) */
const RME32_WCR_ADAT: u32 = 1 << 12;    /* Adat Mode (only Adat-Version) */
const RME32_WCR_AUTOSYNC: u32 = 1 << 13; /* AutoSync */
const RME32_WCR_PD: u32 = 1 << 14;      /* DAC Reset (only PRO-Version) */
const RME32_WCR_EMP: u32 = 1 << 15;     /* 1=Emphasis on (only PRO-Version) */

const RME32_WCR_BITPOS_FREQ_0: u32 = 4;
const RME32_WCR_BITPOS_FREQ_1: u32 = 5;
const RME32_WCR_BITPOS_INP_0: u32 = 6;
const RME32_WCR_BITPOS_INP_1: u32 = 7;

const RME32_RCR_AUDIO_ADDR_MASK: u32 = 0x1ffff;
const RME32_RCR_LOCK: u32 = 1 << 23;
const RME32_RCR_ERF: u32 = 1 << 26;
const RME32_RCR_FREQ_0: u32 = 1 << 27;
const RME32_RCR_FREQ_1: u32 = 1 << 28;
const RME32_RCR_FREQ_2: u32 = 1 << 29;
const RME32_RCR_KMODE: u32 = 1 << 30;
const RME32_RCR_IRQ: u32 = 1 << 31;

const RME32_RCR_BITPOS_F0: u32 = 27;
const RME32_RCR_BITPOS_F1: u32 = 28;
const RME32_RCR_BITPOS_F2: u32 = 29;

const RME32_INPUT_OPTICAL: c_int = 0;
const RME32_INPUT_COAXIAL: c_int = 1;
const RME32_INPUT_INTERNAL: c_int = 2;
const RME32_INPUT_XLR: c_int = 3;

const RME32_CLOCKMODE_SLAVE: c_int = 0;
const RME32_CLOCKMODE_MASTER_32: c_int = 1;
const RME32_CLOCKMODE_MASTER_44: c_int = 2;
const RME32_CLOCKMODE_MASTER_48: c_int = 3;

const RME32_BLOCK_SIZE: size_t = 8192;
const RME32_MID_BUFFER_SIZE: size_t = 1024 * 1024;

const RME32_32_REVISION: u8 = 192;
const RME32_328_REVISION_OLD: u8 = 100;
const RME32_328_REVISION_NEW: u8 = 101;
const RME32_PRO_REVISION_WITH_8412: u8 = 192;
const RME32_PRO_REVISION_WITH_8414: u8 = 150;

const EFAULT: c_int = 14;
const EINVAL: c_int = 22;
const EIO: c_int = 5;
const EBUSY: c_int = 16;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_SHARED: c_ulong = 0x80;

const PCI_DEVICE_ID_RME_DIGI32: c_int = 0;
const PCI_DEVICE_ID_RME_DIGI32_8: c_int = 1;
const PCI_DEVICE_ID_RME_DIGI32_PRO: c_int = 2;

const SNDRV_PCM_INFO_MMAP_IOMEM: u32 = 1 << 0;
const SNDRV_PCM_INFO_MMAP_VALID: u32 = 1 << 1;
const SNDRV_PCM_INFO_INTERLEAVED: u32 = 1 << 2;
const SNDRV_PCM_INFO_PAUSE: u32 = 1 << 3;
const SNDRV_PCM_INFO_SYNC_START: u32 = 1 << 4;
const SNDRV_PCM_INFO_SYNC_APPLPTR: u32 = 1 << 5;
const SNDRV_PCM_INFO_MMAP: u32 = 1 << 6;
const SNDRV_PCM_INFO_JOINT_DUPLEX: u32 = 1 << 7;
const SNDRV_PCM_INFO_HALF_DUPLEX: u32 = 1 << 8;
const SNDRV_PCM_FMTBIT_S16_LE: u32 = 1 << 0;
const SNDRV_PCM_FMTBIT_S32_LE: u32 = 1 << 1;
const SNDRV_PCM_RATE_32000: u32 = 1 << 0;
const SNDRV_PCM_RATE_44100: u32 = 1 << 1;
const SNDRV_PCM_RATE_48000: u32 = 1 << 2;
const SNDRV_PCM_RATE_64000: u32 = 1 << 3;
const SNDRV_PCM_RATE_88200: u32 = 1 << 4;
const SNDRV_PCM_RATE_96000: u32 = 1 << 5;
const SNDRV_PCM_FORMAT_S16_LE: snd_pcm_format_t = 0;
const SNDRV_PCM_FORMAT_S32_LE: snd_pcm_format_t = 1;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 2;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 3;
const SNDRV_DMA_TYPE_CONTINUOUS: c_int = 0;
const SNDRV_PCM_HW_PARAM_BUFFER_BYTES: c_int = 0;
const SNDRV_PCM_HW_PARAM_PERIOD_BYTES: c_int = 1;
const SNDRV_CTL_ELEM_ACCESS_INACTIVE: u32 = 1 << 0;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: u32 = 1 << 1;
const SNDRV_CTL_ELEM_ACCESS_READ: u32 = 1 << 2;
const SNDRV_CTL_ELEM_IFACE_PCM: c_int = 0;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_int = 1;
const SNDRV_CTL_ELEM_TYPE_IEC958: c_int = 0;
const SNDRV_CTL_EVENT_MASK_VALUE: u32 = 1 << 0;
const SNDRV_CTL_EVENT_MASK_INFO: u32 = 1 << 1;
const IEC958_AES0_PROFESSIONAL: u8 = 1 << 0;
const IEC958_AES0_PRO_EMPHASIS_5015: u8 = 1 << 1;
const IEC958_AES0_CON_EMPHASIS_5015: u8 = 1 << 2;
const IEC958_AES0_CON_EMPHASIS: c_ulong = 1 << 3;
const IEC958_AES0_PRO_EMPHASIS: c_ulong = 1 << 4;

#[repr(C)]
pub struct rme32 {
    pub lock: spinlock_t,
    pub irq: c_int,
    pub port: c_ulong,
    pub iobase: *mut c_void,
    pub wcreg: u32,
    pub wcreg_spdif: u32,
    pub wcreg_spdif_stream: u32,
    pub rcreg: u32,
    pub rev: u8,
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub playback_frlog: c_int,
    pub capture_frlog: c_int,
    pub playback_periodsize: size_t,
    pub capture_periodsize: size_t,
    pub fullduplex_mode: u32,
    pub running: c_int,
    pub playback_pcm: snd_pcm_indirect,
    pub capture_pcm: snd_pcm_indirect,
    pub card: *mut snd_card,
    pub spdif_pcm: *mut snd_pcm,
    pub adat_pcm: *mut snd_pcm,
    pub pci: *mut pci_dev,
    pub spdif_ctl: *mut snd_kcontrol,
}

static snd_rme32_ids: [pci_device_id; 4] = [
    pci_device_id { _private: [] },
    pci_device_id { _private: [] },
    pci_device_id { _private: [] },
    pci_device_id { _private: [] },
];

#[inline]
unsafe fn RME32_ISWORKING(rme32: *mut rme32) -> bool {
    ((*rme32).wcreg & RME32_WCR_START) != 0
}

#[inline]
unsafe fn RME32_PRO_WITH_8414(rme32: *mut rme32) -> bool {
    (*(*rme32).pci).device == PCI_DEVICE_ID_RME_DIGI32_PRO
        && (*rme32).rev == RME32_PRO_REVISION_WITH_8414
}

unsafe extern "C" {
    fn readl(addr: *mut c_void) -> u32;
    fn writel(val: u32, addr: *mut c_void);
    fn memset_io(addr: *mut c_void, val: c_int, count: size_t);
    fn copy_from_iter_toio(addr: *mut c_void, count: size_t, src: *mut iov_iter) -> size_t;
    fn copy_to_iter_fromio(addr: *mut c_void, count: size_t, dst: *mut iov_iter) -> size_t;
    fn memcpy_toio(dst: *mut c_void, src: *const c_void, count: size_t);
    fn memcpy_fromio(dst: *mut c_void, src: *const c_void, count: size_t);
    fn memset(dst: *mut c_void, val: c_int, count: size_t) -> *mut c_void;
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut rme32;
    fn snd_pcm_set_sync(substream: *mut snd_pcm_substream);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_trigger_done(s: *mut snd_pcm_substream, substream: *mut snd_pcm_substream);
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> size_t;
    fn snd_pcm_indirect_playback_transfer(substream: *mut snd_pcm_substream, rec: *mut snd_pcm_indirect, cb: unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_indirect, size_t)) -> c_int;
    fn snd_pcm_indirect_capture_transfer(substream: *mut snd_pcm_substream, rec: *mut snd_pcm_indirect, cb: unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_indirect, size_t)) -> c_int;
    fn snd_pcm_indirect_playback_pointer(substream: *mut snd_pcm_substream, rec: *mut snd_pcm_indirect, ptr: u32) -> snd_pcm_uframes_t;
    fn snd_pcm_indirect_capture_pointer(substream: *mut snd_pcm_substream, rec: *mut snd_pcm_indirect, ptr: u32) -> snd_pcm_uframes_t;
    fn snd_pcm_hw_constraint_single(runtime: *mut snd_pcm_runtime, var: c_int, val: size_t) -> c_int;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: u32, var: c_int, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn snd_pcm_rate_to_rate_bit(rate: c_int) -> u32;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_period_size(params: *mut snd_pcm_hw_params) -> size_t;
    fn snd_pcm_lib_mmap_iomem() -> c_int;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn pcim_request_all_regions(pci: *mut pci_dev, name: *const c_char) -> c_int;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn devm_ioremap(dev: *mut device, offset: c_ulong, size: size_t) -> *mut c_void;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, data: *mut c_void) -> c_int;
    fn pci_read_config_byte(pci: *mut pci_dev, where_: c_int, val: *mut u8) -> c_int;
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback: c_int, capture: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, ty: c_int, data: *mut c_void, min: size_t, max: size_t);
    fn strscpy(dst: *mut c_char, src: *const c_char) -> size_t;
    fn sprintf(dst: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_card_ro_proc_new(card: *mut snd_card, name: *const c_char, data: *mut rme32, read: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)) -> c_int;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut rme32;
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_ctl_enum_info(uinfo: *mut snd_ctl_elem_info, channels: u32, items: c_int, texts: *const *const c_char) -> c_int;
    fn snd_BUG();
    fn snd_ctl_notify(card: *mut snd_card, mask: u32, id: *mut snd_ctl_elem_id);
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut rme32) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_devm_card_new(dev: *mut device, idx: c_int, xid: *mut c_char, module: *mut c_void, extra_size: size_t, card_ret: *mut *mut snd_card) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut snd_card);
    fn snd_card_free_on_error(dev: *mut device, err: c_int) -> c_int;
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: u32,
    pub list: *const u32,
    pub mask: u32,
}

#[inline]
unsafe fn ptr_add(base: *mut c_void, off: usize) -> *mut c_void {
    (base as *mut u8).add(off) as *mut c_void
}

#[inline]
unsafe extern "C" fn snd_rme32_pcm_byteptr(rme32: *mut rme32) -> u32 {
    readl(ptr_add((*rme32).iobase, RME32_IO_GET_POS)) & RME32_RCR_AUDIO_ADDR_MASK
}

/* silence callback for halfduplex mode */
unsafe extern "C" fn snd_rme32_playback_silence(substream: *mut snd_pcm_substream, _channel: c_int, pos: c_ulong, count: c_ulong) -> c_int {
    let rme32 = snd_pcm_substream_chip(substream);
    memset_io(ptr_add((*rme32).iobase, RME32_IO_DATA_BUFFER + pos as usize), 0, count as size_t);
    0
}

/* copy callback for halfduplex mode */
unsafe extern "C" fn snd_rme32_playback_copy(substream: *mut snd_pcm_substream, _channel: c_int, pos: c_ulong, src: *mut iov_iter, count: c_ulong) -> c_int {
    let rme32 = snd_pcm_substream_chip(substream);
    if copy_from_iter_toio(ptr_add((*rme32).iobase, RME32_IO_DATA_BUFFER + pos as usize), count as size_t, src) != count as size_t {
        return -EFAULT;
    }
    0
}

/* copy callback for halfduplex mode */
unsafe extern "C" fn snd_rme32_capture_copy(substream: *mut snd_pcm_substream, _channel: c_int, pos: c_ulong, dst: *mut iov_iter, count: c_ulong) -> c_int {
    let rme32 = snd_pcm_substream_chip(substream);
    if copy_to_iter_fromio(ptr_add((*rme32).iobase, RME32_IO_DATA_BUFFER + pos as usize), count as size_t, dst) != count as size_t {
        return -EFAULT;
    }
    0
}

static snd_rme32_spdif_info: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP_IOMEM | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_SYNC_START | SNDRV_PCM_INFO_SYNC_APPLPTR,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE,
    rates: SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000,
    rate_min: 32000, rate_max: 48000, channels_min: 2, channels_max: 2,
    buffer_bytes_max: RME32_BUFFER_SIZE, period_bytes_min: RME32_BLOCK_SIZE, period_bytes_max: RME32_BLOCK_SIZE,
    periods_min: (RME32_BUFFER_SIZE / RME32_BLOCK_SIZE) as u32, periods_max: (RME32_BUFFER_SIZE / RME32_BLOCK_SIZE) as u32, fifo_size: 0,
};

static snd_rme32_adat_info: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP_IOMEM | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_SYNC_START | SNDRV_PCM_INFO_SYNC_APPLPTR,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000,
    rate_min: 44100, rate_max: 48000, channels_min: 8, channels_max: 8,
    buffer_bytes_max: RME32_BUFFER_SIZE, period_bytes_min: RME32_BLOCK_SIZE, period_bytes_max: RME32_BLOCK_SIZE,
    periods_min: (RME32_BUFFER_SIZE / RME32_BLOCK_SIZE) as u32, periods_max: (RME32_BUFFER_SIZE / RME32_BLOCK_SIZE) as u32, fifo_size: 0,
};

static snd_rme32_spdif_fd_info: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_SYNC_START | SNDRV_PCM_INFO_SYNC_APPLPTR,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE,
    rates: SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000,
    rate_min: 32000, rate_max: 48000, channels_min: 2, channels_max: 2,
    buffer_bytes_max: RME32_MID_BUFFER_SIZE, period_bytes_min: RME32_BLOCK_SIZE, period_bytes_max: RME32_BLOCK_SIZE,
    periods_min: 2, periods_max: (RME32_MID_BUFFER_SIZE / RME32_BLOCK_SIZE) as u32, fifo_size: 0,
};

static snd_rme32_adat_fd_info: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_SYNC_START | SNDRV_PCM_INFO_SYNC_APPLPTR,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000,
    rate_min: 44100, rate_max: 48000, channels_min: 8, channels_max: 8,
    buffer_bytes_max: RME32_MID_BUFFER_SIZE, period_bytes_min: RME32_BLOCK_SIZE, period_bytes_max: RME32_BLOCK_SIZE,
    periods_min: 2, periods_max: (RME32_MID_BUFFER_SIZE / RME32_BLOCK_SIZE) as u32, fifo_size: 0,
};

unsafe extern "C" fn snd_rme32_reset_dac(rme32: *mut rme32) {
    writel((*rme32).wcreg | RME32_WCR_PD, ptr_add((*rme32).iobase, RME32_IO_CONTROL_REGISTER));
    writel((*rme32).wcreg, ptr_add((*rme32).iobase, RME32_IO_CONTROL_REGISTER));
}

unsafe extern "C" fn snd_rme32_playback_getrate(rme32: *mut rme32) -> c_int {
    let mut rate = (((*rme32).wcreg >> RME32_WCR_BITPOS_FREQ_0) & 1) as c_int
        + (((((*rme32).wcreg >> RME32_WCR_BITPOS_FREQ_1) & 1) << 1) as c_int);
    match rate {
        1 => rate = 32000,
        2 => rate = 44100,
        3 => rate = 48000,
        _ => return -1,
    }
    if ((*rme32).wcreg & RME32_WCR_DS_BM) != 0 { rate << 1 } else { rate }
}

unsafe extern "C" fn snd_rme32_capture_getrate(rme32: *mut rme32, is_adat: *mut c_int) -> c_int {
    *is_adat = 0;
    if ((*rme32).rcreg & RME32_RCR_LOCK) != 0 {
        /* ADAT rate */
        *is_adat = 1;
    }
    if ((*rme32).rcreg & RME32_RCR_ERF) != 0 {
        return -1;
    }
    /* S/PDIF rate */
    let n = (((*rme32).rcreg >> RME32_RCR_BITPOS_F0) & 1)
        + ((((*rme32).rcreg >> RME32_RCR_BITPOS_F1) & 1) << 1)
        + ((((*rme32).rcreg >> RME32_RCR_BITPOS_F2) & 1) << 2);
    if RME32_PRO_WITH_8414(rme32) {
        match n { 3 => 96000, 4 => 88200, 5 => 48000, 6 => 44100, 7 => 32000, _ => -1 }
    } else {
        match n { 1 => 48000, 2 => 44100, 3 => 32000, 4 => 48000, 5 => 44100, 6 => 44056, 7 => 32000, _ => -1 }
    }
}

unsafe extern "C" fn snd_rme32_playback_setrate(rme32: *mut rme32, rate: c_int) -> c_int {
    let ds = (*rme32).wcreg & RME32_WCR_DS_BM;
    match rate {
        32000 => { (*rme32).wcreg &= !RME32_WCR_DS_BM; (*rme32).wcreg = ((*rme32).wcreg | RME32_WCR_FREQ_0) & !RME32_WCR_FREQ_1; }
        44100 => { (*rme32).wcreg &= !RME32_WCR_DS_BM; (*rme32).wcreg = ((*rme32).wcreg | RME32_WCR_FREQ_1) & !RME32_WCR_FREQ_0; }
        48000 => { (*rme32).wcreg &= !RME32_WCR_DS_BM; (*rme32).wcreg = ((*rme32).wcreg | RME32_WCR_FREQ_0) | RME32_WCR_FREQ_1; }
        64000 => { if (*(*rme32).pci).device != PCI_DEVICE_ID_RME_DIGI32_PRO { return -EINVAL; } (*rme32).wcreg |= RME32_WCR_DS_BM; (*rme32).wcreg = ((*rme32).wcreg | RME32_WCR_FREQ_0) & !RME32_WCR_FREQ_1; }
        88200 => { if (*(*rme32).pci).device != PCI_DEVICE_ID_RME_DIGI32_PRO { return -EINVAL; } (*rme32).wcreg |= RME32_WCR_DS_BM; (*rme32).wcreg = ((*rme32).wcreg | RME32_WCR_FREQ_1) & !RME32_WCR_FREQ_0; }
        96000 => { if (*(*rme32).pci).device != PCI_DEVICE_ID_RME_DIGI32_PRO { return -EINVAL; } (*rme32).wcreg |= RME32_WCR_DS_BM; (*rme32).wcreg = ((*rme32).wcreg | RME32_WCR_FREQ_0) | RME32_WCR_FREQ_1; }
        _ => return -EINVAL,
    }
    if (ds == 0 && ((*rme32).wcreg & RME32_WCR_DS_BM) != 0) || (ds != 0 && ((*rme32).wcreg & RME32_WCR_DS_BM) == 0) {
        /* change to/from double-speed: reset the DAC (if available) */
        snd_rme32_reset_dac(rme32);
    } else {
        writel((*rme32).wcreg, ptr_add((*rme32).iobase, RME32_IO_CONTROL_REGISTER));
    }
    0
}

unsafe extern "C" fn snd_rme32_setclockmode(rme32: *mut rme32, mode: c_int) -> c_int {
    match mode {
        RME32_CLOCKMODE_SLAVE => (*rme32).wcreg = ((*rme32).wcreg & !RME32_WCR_FREQ_0) & !RME32_WCR_FREQ_1,
        RME32_CLOCKMODE_MASTER_32 => (*rme32).wcreg = ((*rme32).wcreg | RME32_WCR_FREQ_0) & !RME32_WCR_FREQ_1,
        RME32_CLOCKMODE_MASTER_44 => (*rme32).wcreg = ((*rme32).wcreg & !RME32_WCR_FREQ_0) | RME32_WCR_FREQ_1,
        RME32_CLOCKMODE_MASTER_48 => (*rme32).wcreg = ((*rme32).wcreg | RME32_WCR_FREQ_0) | RME32_WCR_FREQ_1,
        _ => return -EINVAL,
    }
    writel((*rme32).wcreg, ptr_add((*rme32).iobase, RME32_IO_CONTROL_REGISTER));
    0
}

unsafe extern "C" fn snd_rme32_getclockmode(rme32: *mut rme32) -> c_int {
    (((*rme32).wcreg >> RME32_WCR_BITPOS_FREQ_0) & 1) as c_int
        + (((((*rme32).wcreg >> RME32_WCR_BITPOS_FREQ_1) & 1) << 1) as c_int)
}

unsafe extern "C" fn snd_rme32_setinputtype(rme32: *mut rme32, type_: c_int) -> c_int {
    match type_ {
        RME32_INPUT_OPTICAL => (*rme32).wcreg = ((*rme32).wcreg & !RME32_WCR_INP_0) & !RME32_WCR_INP_1,
        RME32_INPUT_COAXIAL => (*rme32).wcreg = ((*rme32).wcreg | RME32_WCR_INP_0) & !RME32_WCR_INP_1,
        RME32_INPUT_INTERNAL => (*rme32).wcreg = ((*rme32).wcreg & !RME32_WCR_INP_0) | RME32_WCR_INP_1,
        RME32_INPUT_XLR => (*rme32).wcreg = ((*rme32).wcreg | RME32_WCR_INP_0) | RME32_WCR_INP_1,
        _ => return -EINVAL,
    }
    writel((*rme32).wcreg, ptr_add((*rme32).iobase, RME32_IO_CONTROL_REGISTER));
    0
}

unsafe extern "C" fn snd_rme32_getinputtype(rme32: *mut rme32) -> c_int {
    (((*rme32).wcreg >> RME32_WCR_BITPOS_INP_0) & 1) as c_int
        + (((((*rme32).wcreg >> RME32_WCR_BITPOS_INP_1) & 1) << 1) as c_int)
}

unsafe extern "C" fn snd_rme32_setframelog(rme32: *mut rme32, n_channels: c_int, is_playback: c_int) {
    let mut frlog = if n_channels == 2 { 1 } else { 3 };
    frlog += if ((*rme32).wcreg & RME32_WCR_MODE24) != 0 { 2 } else { 1 };
    if is_playback != 0 { (*rme32).playback_frlog = frlog; } else { (*rme32).capture_frlog = frlog; }
}

unsafe extern "C" fn snd_rme32_setformat(rme32: *mut rme32, format: snd_pcm_format_t) -> c_int {
    match format {
        SNDRV_PCM_FORMAT_S16_LE => (*rme32).wcreg &= !RME32_WCR_MODE24,
        SNDRV_PCM_FORMAT_S32_LE => (*rme32).wcreg |= RME32_WCR_MODE24,
        _ => return -EINVAL,
    }
    writel((*rme32).wcreg, ptr_add((*rme32).iobase, RME32_IO_CONTROL_REGISTER));
    0
}

unsafe extern "C" fn snd_rme32_playback_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let rme32 = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    if (*rme32).fullduplex_mode == 0 {
        (*runtime).dma_area = ptr_add((*rme32).iobase, RME32_IO_DATA_BUFFER);
        (*runtime).dma_addr = (*rme32).port + RME32_IO_DATA_BUFFER as c_ulong;
        (*runtime).dma_bytes = RME32_BUFFER_SIZE;
    }
    let mut dummy = 0;
    let mut rate = 0;
    if ((*rme32).rcreg & RME32_RCR_KMODE) != 0 { rate = snd_rme32_capture_getrate(rme32, &mut dummy); }
    if rate > 0 {
        if params_rate(params) != rate { return -EIO; }
    } else {
        let err = snd_rme32_playback_setrate(rme32, params_rate(params));
        if err < 0 { return err; }
    }
    let err = snd_rme32_setformat(rme32, params_format(params));
    if err < 0 { return err; }
    snd_rme32_setframelog(rme32, params_channels(params), 1);
    if (*rme32).capture_periodsize != 0 && (params_period_size(params) << (*rme32).playback_frlog) != (*rme32).capture_periodsize {
        return -EBUSY;
    }
    (*rme32).playback_periodsize = params_period_size(params) << (*rme32).playback_frlog;
    if ((*rme32).wcreg & RME32_WCR_ADAT) == 0 {
        (*rme32).wcreg &= !(RME32_WCR_PRO | RME32_WCR_EMP);
        (*rme32).wcreg |= (*rme32).wcreg_spdif_stream;
        writel((*rme32).wcreg, ptr_add((*rme32).iobase, RME32_IO_CONTROL_REGISTER));
    }
    0
}

unsafe extern "C" fn snd_rme32_capture_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let rme32 = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    if (*rme32).fullduplex_mode == 0 {
        (*runtime).dma_area = ptr_add((*rme32).iobase, RME32_IO_DATA_BUFFER);
        (*runtime).dma_addr = (*rme32).port + RME32_IO_DATA_BUFFER as c_ulong;
        (*runtime).dma_bytes = RME32_BUFFER_SIZE;
    }
    (*rme32).wcreg |= RME32_WCR_AUTOSYNC;
    writel((*rme32).wcreg, ptr_add((*rme32).iobase, RME32_IO_CONTROL_REGISTER));
    let mut err = snd_rme32_setformat(rme32, params_format(params));
    if err < 0 { return err; }
    err = snd_rme32_playback_setrate(rme32, params_rate(params));
    if err < 0 { return err; }
    let mut isadat = 0;
    let rate = snd_rme32_capture_getrate(rme32, &mut isadat);
    if rate > 0 {
        if params_rate(params) != rate { return -EIO; }
        if (isadat != 0 && (*runtime).hw.channels_min == 2) || (isadat == 0 && (*runtime).hw.channels_min == 8) { return -EIO; }
    }
    (*rme32).wcreg &= !RME32_WCR_AUTOSYNC;
    writel((*rme32).wcreg, ptr_add((*rme32).iobase, RME32_IO_CONTROL_REGISTER));
    snd_rme32_setframelog(rme32, params_channels(params), 0);
    if (*rme32).playback_periodsize != 0 && (params_period_size(params) << (*rme32).capture_frlog) != (*rme32).playback_periodsize {
        return -EBUSY;
    }
    (*rme32).capture_periodsize = params_period_size(params) << (*rme32).capture_frlog;
    0
}

unsafe extern "C" fn snd_rme32_pcm_start(rme32: *mut rme32, from_pause: c_int) {
    if from_pause == 0 { writel(0, ptr_add((*rme32).iobase, RME32_IO_RESET_POS)); }
    (*rme32).wcreg |= RME32_WCR_START;
    writel((*rme32).wcreg, ptr_add((*rme32).iobase, RME32_IO_CONTROL_REGISTER));
}

unsafe extern "C" fn snd_rme32_pcm_stop(rme32: *mut rme32, to_pause: c_int) {
    (*rme32).rcreg = readl(ptr_add((*rme32).iobase, RME32_IO_CONTROL_REGISTER));
    if ((*rme32).rcreg & RME32_RCR_IRQ) != 0 { writel(0, ptr_add((*rme32).iobase, RME32_IO_CONFIRM_ACTION_IRQ)); }
    (*rme32).wcreg &= !RME32_WCR_START;
    if ((*rme32).wcreg & RME32_WCR_SEL) != 0 { (*rme32).wcreg |= RME32_WCR_MUTE; }
    writel((*rme32).wcreg, ptr_add((*rme32).iobase, RME32_IO_CONTROL_REGISTER));
    if to_pause == 0 { writel(0, ptr_add((*rme32).iobase, RME32_IO_RESET_POS)); }
}

unsafe extern "C" fn snd_rme32_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let rme32 = dev_id as *mut rme32;
    (*rme32).rcreg = readl(ptr_add((*rme32).iobase, RME32_IO_CONTROL_REGISTER));
    if ((*rme32).rcreg & RME32_RCR_IRQ) == 0 { return IRQ_NONE; }
    if !(*rme32).capture_substream.is_null() { snd_pcm_period_elapsed((*rme32).capture_substream); }
    if !(*rme32).playback_substream.is_null() { snd_pcm_period_elapsed((*rme32).playback_substream); }
    writel(0, ptr_add((*rme32).iobase, RME32_IO_CONFIRM_ACTION_IRQ));
    IRQ_HANDLED
}

static period_bytes: [u32; 1] = [RME32_BLOCK_SIZE as u32];
static hw_constraints_period_bytes: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list { count: 1, list: period_bytes.as_ptr(), mask: 0 };

unsafe extern "C" fn snd_rme32_set_buffer_constraint(rme32: *mut rme32, runtime: *mut snd_pcm_runtime) {
    if (*rme32).fullduplex_mode == 0 {
        snd_pcm_hw_constraint_single(runtime, SNDRV_PCM_HW_PARAM_BUFFER_BYTES, RME32_BUFFER_SIZE);
        snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, &hw_constraints_period_bytes);
    }
}

unsafe extern "C" fn snd_rme32_playback_spdif_open(substream: *mut snd_pcm_substream) -> c_int {
    let rme32 = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    snd_pcm_set_sync(substream);
    if !(*rme32).playback_substream.is_null() { return -EBUSY; }
    (*rme32).wcreg &= !RME32_WCR_ADAT;
    writel((*rme32).wcreg, ptr_add((*rme32).iobase, RME32_IO_CONTROL_REGISTER));
    (*rme32).playback_substream = substream;
    (*runtime).hw = if (*rme32).fullduplex_mode != 0 { snd_rme32_spdif_fd_info } else { snd_rme32_spdif_info };
    if (*(*rme32).pci).device == PCI_DEVICE_ID_RME_DIGI32_PRO {
        (*runtime).hw.rates |= SNDRV_PCM_RATE_64000 | SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000;
        (*runtime).hw.rate_max = 96000;
    }
    let mut dummy = 0;
    let mut rate = 0;
    if ((*rme32).rcreg & RME32_RCR_KMODE) != 0 { rate = snd_rme32_capture_getrate(rme32, &mut dummy); }
    if rate > 0 {
        (*runtime).hw.rates = snd_pcm_rate_to_rate_bit(rate);
        (*runtime).hw.rate_min = rate as u32;
        (*runtime).hw.rate_max = rate as u32;
    }
    snd_rme32_set_buffer_constraint(rme32, runtime);
    (*rme32).wcreg_spdif_stream = (*rme32).wcreg_spdif;
    (*(*rme32).spdif_ctl).vd.as_mut().unwrap().access &= !SNDRV_CTL_ELEM_ACCESS_INACTIVE;
    snd_ctl_notify((*rme32).card, SNDRV_CTL_EVENT_MASK_VALUE | SNDRV_CTL_EVENT_MASK_INFO, &mut (*(*rme32).spdif_ctl).id);
    0
}

unsafe extern "C" fn snd_rme32_capture_spdif_open(substream: *mut snd_pcm_substream) -> c_int {
    let rme32 = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    snd_pcm_set_sync(substream);
    if !(*rme32).capture_substream.is_null() { return -EBUSY; }
    (*rme32).capture_substream = substream;
    (*runtime).hw = if (*rme32).fullduplex_mode != 0 { snd_rme32_spdif_fd_info } else { snd_rme32_spdif_info };
    if RME32_PRO_WITH_8414(rme32) {
        (*runtime).hw.rates |= SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000;
        (*runtime).hw.rate_max = 96000;
    }
    let mut isadat = 0;
    let rate = snd_rme32_capture_getrate(rme32, &mut isadat);
    if rate > 0 {
        if isadat != 0 { return -EIO; }
        (*runtime).hw.rates = snd_pcm_rate_to_rate_bit(rate);
        (*runtime).hw.rate_min = rate as u32;
        (*runtime).hw.rate_max = rate as u32;
    }
    snd_rme32_set_buffer_constraint(rme32, runtime);
    0
}

unsafe extern "C" fn snd_rme32_playback_adat_open(substream: *mut snd_pcm_substream) -> c_int {
    let rme32 = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    snd_pcm_set_sync(substream);
    if !(*rme32).playback_substream.is_null() { return -EBUSY; }
    (*rme32).wcreg |= RME32_WCR_ADAT;
    writel((*rme32).wcreg, ptr_add((*rme32).iobase, RME32_IO_CONTROL_REGISTER));
    (*rme32).playback_substream = substream;
    (*runtime).hw = if (*rme32).fullduplex_mode != 0 { snd_rme32_adat_fd_info } else { snd_rme32_adat_info };
    let mut dummy = 0;
    let mut rate = 0;
    if ((*rme32).rcreg & RME32_RCR_KMODE) != 0 { rate = snd_rme32_capture_getrate(rme32, &mut dummy); }
    if rate > 0 {
        (*runtime).hw.rates = snd_pcm_rate_to_rate_bit(rate);
        (*runtime).hw.rate_min = rate as u32;
        (*runtime).hw.rate_max = rate as u32;
    }
    snd_rme32_set_buffer_constraint(rme32, runtime);
    0
}

unsafe extern "C" fn snd_rme32_capture_adat_open(substream: *mut snd_pcm_substream) -> c_int {
    let rme32 = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    (*runtime).hw = if (*rme32).fullduplex_mode != 0 { snd_rme32_adat_fd_info } else { snd_rme32_adat_info };
    let mut isadat = 0;
    let rate = snd_rme32_capture_getrate(rme32, &mut isadat);
    if rate > 0 {
        if isadat == 0 { return -EIO; }
        (*runtime).hw.rates = snd_pcm_rate_to_rate_bit(rate);
        (*runtime).hw.rate_min = rate as u32;
        (*runtime).hw.rate_max = rate as u32;
    }
    snd_pcm_set_sync(substream);
    if !(*rme32).capture_substream.is_null() { return -EBUSY; }
    (*rme32).capture_substream = substream;
    snd_rme32_set_buffer_constraint(rme32, runtime);
    0
}

unsafe extern "C" fn snd_rme32_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    let rme32 = snd_pcm_substream_chip(substream);
    (*rme32).playback_substream = ptr::null_mut();
    (*rme32).playback_periodsize = 0;
    let spdif = ((*rme32).wcreg & RME32_WCR_ADAT) == 0;
    if spdif {
        (*(*rme32).spdif_ctl).vd.as_mut().unwrap().access |= SNDRV_CTL_ELEM_ACCESS_INACTIVE;
        snd_ctl_notify((*rme32).card, SNDRV_CTL_EVENT_MASK_VALUE | SNDRV_CTL_EVENT_MASK_INFO, &mut (*(*rme32).spdif_ctl).id);
    }
    0
}

unsafe extern "C" fn snd_rme32_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    let rme32 = snd_pcm_substream_chip(substream);
    (*rme32).capture_substream = ptr::null_mut();
    (*rme32).capture_periodsize = 0;
    0
}

unsafe extern "C" fn snd_rme32_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let rme32 = snd_pcm_substream_chip(substream);
    if (*rme32).fullduplex_mode != 0 {
        memset(&mut (*rme32).playback_pcm as *mut _ as *mut c_void, 0, size_of::<snd_pcm_indirect>());
        (*rme32).playback_pcm.hw_buffer_size = RME32_BUFFER_SIZE;
        (*rme32).playback_pcm.sw_buffer_size = snd_pcm_lib_buffer_bytes(substream);
    } else {
        writel(0, ptr_add((*rme32).iobase, RME32_IO_RESET_POS));
    }
    if ((*rme32).wcreg & RME32_WCR_SEL) != 0 { (*rme32).wcreg &= !RME32_WCR_MUTE; }
    writel((*rme32).wcreg, ptr_add((*rme32).iobase, RME32_IO_CONTROL_REGISTER));
    0
}

unsafe extern "C" fn snd_rme32_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let rme32 = snd_pcm_substream_chip(substream);
    if (*rme32).fullduplex_mode != 0 {
        memset(&mut (*rme32).capture_pcm as *mut _ as *mut c_void, 0, size_of::<snd_pcm_indirect>());
        (*rme32).capture_pcm.hw_buffer_size = RME32_BUFFER_SIZE;
        (*rme32).capture_pcm.hw_queue_size = RME32_BUFFER_SIZE / 2;
        (*rme32).capture_pcm.sw_buffer_size = snd_pcm_lib_buffer_bytes(substream);
    } else {
        writel(0, ptr_add((*rme32).iobase, RME32_IO_RESET_POS));
    }
    0
}

unsafe extern "C" fn snd_rme32_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let rme32 = snd_pcm_substream_chip(substream);
    let streams = [(*rme32).playback_substream, (*rme32).capture_substream];
    for &s in streams.iter() {
        if s.is_null() { continue; }
        match cmd {
            SNDRV_PCM_TRIGGER_START => {
                (*rme32).running |= 1 << (*s).stream;
                if (*rme32).fullduplex_mode != 0 {
                    if s == (*rme32).playback_substream {
                        let p = snd_rme32_pcm_byteptr(rme32) as size_t;
                        (*rme32).playback_pcm.hw_io = p;
                        (*rme32).playback_pcm.hw_data = p;
                    } else {
                        let p = snd_rme32_pcm_byteptr(rme32) as size_t;
                        (*rme32).capture_pcm.hw_io = p;
                        (*rme32).capture_pcm.hw_data = p;
                    }
                }
            }
            SNDRV_PCM_TRIGGER_STOP => (*rme32).running &= !(1 << (*s).stream),
            _ => {}
        }
        snd_pcm_trigger_done(s, substream);
    }
    match cmd {
        SNDRV_PCM_TRIGGER_START => if (*rme32).running != 0 && !RME32_ISWORKING(rme32) { snd_rme32_pcm_start(rme32, 0); },
        SNDRV_PCM_TRIGGER_STOP => if (*rme32).running == 0 && RME32_ISWORKING(rme32) { snd_rme32_pcm_stop(rme32, 0); },
        SNDRV_PCM_TRIGGER_PAUSE_PUSH => if (*rme32).running != 0 && RME32_ISWORKING(rme32) { snd_rme32_pcm_stop(rme32, 1); },
        SNDRV_PCM_TRIGGER_PAUSE_RELEASE => if (*rme32).running != 0 && !RME32_ISWORKING(rme32) { snd_rme32_pcm_start(rme32, 1); },
        _ => {}
    }
    0
}

/* pointer callback for halfduplex mode */
unsafe extern "C" fn snd_rme32_playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let rme32 = snd_pcm_substream_chip(substream);
    (snd_rme32_pcm_byteptr(rme32) >> (*rme32).playback_frlog) as snd_pcm_uframes_t
}

unsafe extern "C" fn snd_rme32_capture_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let rme32 = snd_pcm_substream_chip(substream);
    (snd_rme32_pcm_byteptr(rme32) >> (*rme32).capture_frlog) as snd_pcm_uframes_t
}

/* ack and pointer callbacks for fullduplex mode */
unsafe extern "C" fn snd_rme32_pb_trans_copy(substream: *mut snd_pcm_substream, rec: *mut snd_pcm_indirect, bytes: size_t) {
    let rme32 = snd_pcm_substream_chip(substream);
    memcpy_toio(ptr_add((*rme32).iobase, RME32_IO_DATA_BUFFER + (*rec).hw_data), ptr_add((*(*substream).runtime).dma_area, (*rec).sw_data), bytes);
}

unsafe extern "C" fn snd_rme32_playback_fd_ack(substream: *mut snd_pcm_substream) -> c_int {
    let rme32 = snd_pcm_substream_chip(substream);
    let rec = &mut (*rme32).playback_pcm as *mut snd_pcm_indirect;
    let cprec = &mut (*rme32).capture_pcm as *mut snd_pcm_indirect;
    (*rec).hw_queue_size = RME32_BUFFER_SIZE;
    if ((*rme32).running & (1 << SNDRV_PCM_STREAM_CAPTURE)) != 0 { (*rec).hw_queue_size -= (*cprec).hw_ready; }
    snd_pcm_indirect_playback_transfer(substream, rec, snd_rme32_pb_trans_copy)
}

unsafe extern "C" fn snd_rme32_cp_trans_copy(substream: *mut snd_pcm_substream, rec: *mut snd_pcm_indirect, bytes: size_t) {
    let rme32 = snd_pcm_substream_chip(substream);
    memcpy_fromio(ptr_add((*(*substream).runtime).dma_area, (*rec).sw_data), ptr_add((*rme32).iobase, RME32_IO_DATA_BUFFER + (*rec).hw_data), bytes);
}

unsafe extern "C" fn snd_rme32_capture_fd_ack(substream: *mut snd_pcm_substream) -> c_int {
    let rme32 = snd_pcm_substream_chip(substream);
    snd_pcm_indirect_capture_transfer(substream, &mut (*rme32).capture_pcm, snd_rme32_cp_trans_copy)
}

unsafe extern "C" fn snd_rme32_playback_fd_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let rme32 = snd_pcm_substream_chip(substream);
    snd_pcm_indirect_playback_pointer(substream, &mut (*rme32).playback_pcm, snd_rme32_pcm_byteptr(rme32))
}

unsafe extern "C" fn snd_rme32_capture_fd_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let rme32 = snd_pcm_substream_chip(substream);
    snd_pcm_indirect_capture_pointer(substream, &mut (*rme32).capture_pcm, snd_rme32_pcm_byteptr(rme32))
}

static snd_rme32_playback_spdif_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_rme32_playback_spdif_open), close: Some(snd_rme32_playback_close), hw_params: Some(snd_rme32_playback_hw_params), prepare: Some(snd_rme32_playback_prepare), trigger: Some(snd_rme32_pcm_trigger), pointer: Some(snd_rme32_playback_pointer), copy: Some(snd_rme32_playback_copy), fill_silence: Some(snd_rme32_playback_silence), mmap: Some(snd_pcm_lib_mmap_iomem), ack: None };
static snd_rme32_capture_spdif_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_rme32_capture_spdif_open), close: Some(snd_rme32_capture_close), hw_params: Some(snd_rme32_capture_hw_params), prepare: Some(snd_rme32_capture_prepare), trigger: Some(snd_rme32_pcm_trigger), pointer: Some(snd_rme32_capture_pointer), copy: Some(snd_rme32_capture_copy), fill_silence: None, mmap: Some(snd_pcm_lib_mmap_iomem), ack: None };
static snd_rme32_playback_adat_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_rme32_playback_adat_open), close: Some(snd_rme32_playback_close), hw_params: Some(snd_rme32_playback_hw_params), prepare: Some(snd_rme32_playback_prepare), trigger: Some(snd_rme32_pcm_trigger), pointer: Some(snd_rme32_playback_pointer), copy: Some(snd_rme32_playback_copy), fill_silence: Some(snd_rme32_playback_silence), mmap: Some(snd_pcm_lib_mmap_iomem), ack: None };
static snd_rme32_capture_adat_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_rme32_capture_adat_open), close: Some(snd_rme32_capture_close), hw_params: Some(snd_rme32_capture_hw_params), prepare: Some(snd_rme32_capture_prepare), trigger: Some(snd_rme32_pcm_trigger), pointer: Some(snd_rme32_capture_pointer), copy: Some(snd_rme32_capture_copy), fill_silence: None, mmap: Some(snd_pcm_lib_mmap_iomem), ack: None };
static snd_rme32_playback_spdif_fd_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_rme32_playback_spdif_open), close: Some(snd_rme32_playback_close), hw_params: Some(snd_rme32_playback_hw_params), prepare: Some(snd_rme32_playback_prepare), trigger: Some(snd_rme32_pcm_trigger), pointer: Some(snd_rme32_playback_fd_pointer), copy: None, fill_silence: None, mmap: None, ack: Some(snd_rme32_playback_fd_ack) };
static snd_rme32_capture_spdif_fd_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_rme32_capture_spdif_open), close: Some(snd_rme32_capture_close), hw_params: Some(snd_rme32_capture_hw_params), prepare: Some(snd_rme32_capture_prepare), trigger: Some(snd_rme32_pcm_trigger), pointer: Some(snd_rme32_capture_fd_pointer), copy: None, fill_silence: None, mmap: None, ack: Some(snd_rme32_capture_fd_ack) };
static snd_rme32_playback_adat_fd_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_rme32_playback_adat_open), close: Some(snd_rme32_playback_close), hw_params: Some(snd_rme32_playback_hw_params), prepare: Some(snd_rme32_playback_prepare), trigger: Some(snd_rme32_pcm_trigger), pointer: Some(snd_rme32_playback_fd_pointer), copy: None, fill_silence: None, mmap: None, ack: Some(snd_rme32_playback_fd_ack) };
static snd_rme32_capture_adat_fd_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_rme32_capture_adat_open), close: Some(snd_rme32_capture_close), hw_params: Some(snd_rme32_capture_hw_params), prepare: Some(snd_rme32_capture_prepare), trigger: Some(snd_rme32_pcm_trigger), pointer: Some(snd_rme32_capture_fd_pointer), copy: None, fill_silence: None, mmap: None, ack: Some(snd_rme32_capture_fd_ack) };

unsafe extern "C" fn snd_rme32_free(rme32: *mut rme32) { if (*rme32).irq >= 0 { snd_rme32_pcm_stop(rme32, 0); } }
unsafe extern "C" fn snd_rme32_free_spdif_pcm(pcm: *mut snd_pcm) { let rme32 = (*pcm).private_data as *mut rme32; (*rme32).spdif_pcm = ptr::null_mut(); }
unsafe extern "C" fn snd_rme32_free_adat_pcm(pcm: *mut snd_pcm) { let rme32 = (*pcm).private_data as *mut rme32; (*rme32).adat_pcm = ptr::null_mut(); }

unsafe extern "C" fn snd_rme32_create(rme32: *mut rme32) -> c_int {
    let pci = (*rme32).pci;
    (*rme32).irq = -1;
    spin_lock_init(&mut (*rme32).lock);
    let mut err = pcim_enable_device(pci);
    if err < 0 { return err; }
    err = pcim_request_all_regions(pci, b"RME32\0".as_ptr() as *const c_char);
    if err < 0 { return err; }
    (*rme32).port = pci_resource_start((*rme32).pci, 0);
    (*rme32).iobase = devm_ioremap(&mut (*pci).dev, (*rme32).port, RME32_IO_SIZE);
    if (*rme32).iobase.is_null() {
        dev_err((*(*rme32).card).dev, b"unable to remap memory region 0x%lx-0x%lx\n\0".as_ptr() as *const c_char, (*rme32).port, (*rme32).port + RME32_IO_SIZE as c_ulong - 1);
        return -ENOMEM;
    }
    if devm_request_irq(&mut (*pci).dev, (*pci).irq, snd_rme32_interrupt, IRQF_SHARED, b"rme32\0".as_ptr() as *const c_char, rme32 as *mut c_void) != 0 {
        dev_err((*(*rme32).card).dev, b"unable to grab IRQ %d\n\0".as_ptr() as *const c_char, (*pci).irq);
        return -EBUSY;
    }
    (*rme32).irq = (*pci).irq;
    (*(*rme32).card).sync_irq = (*rme32).irq;
    pci_read_config_byte(pci, 8, &mut (*rme32).rev);
    err = snd_pcm_new((*rme32).card, b"Digi32 IEC958\0".as_ptr() as *const c_char, 0, 1, 1, &mut (*rme32).spdif_pcm);
    if err < 0 { return err; }
    (*(*rme32).spdif_pcm).private_data = rme32 as *mut c_void;
    (*(*rme32).spdif_pcm).private_free = Some(snd_rme32_free_spdif_pcm);
    strscpy((*(*rme32).spdif_pcm).name, b"Digi32 IEC958\0".as_ptr() as *const c_char);
    if (*rme32).fullduplex_mode != 0 {
        snd_pcm_set_ops((*rme32).spdif_pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_rme32_playback_spdif_fd_ops);
        snd_pcm_set_ops((*rme32).spdif_pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_rme32_capture_spdif_fd_ops);
        snd_pcm_set_managed_buffer_all((*rme32).spdif_pcm, SNDRV_DMA_TYPE_CONTINUOUS, ptr::null_mut(), 0, RME32_MID_BUFFER_SIZE);
        (*(*rme32).spdif_pcm).info_flags = SNDRV_PCM_INFO_JOINT_DUPLEX;
    } else {
        snd_pcm_set_ops((*rme32).spdif_pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_rme32_playback_spdif_ops);
        snd_pcm_set_ops((*rme32).spdif_pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_rme32_capture_spdif_ops);
        (*(*rme32).spdif_pcm).info_flags = SNDRV_PCM_INFO_HALF_DUPLEX;
    }
    if (*pci).device == PCI_DEVICE_ID_RME_DIGI32 || (*pci).device == PCI_DEVICE_ID_RME_DIGI32_PRO {
        (*rme32).adat_pcm = ptr::null_mut();
    } else {
        err = snd_pcm_new((*rme32).card, b"Digi32 ADAT\0".as_ptr() as *const c_char, 1, 1, 1, &mut (*rme32).adat_pcm);
        if err < 0 { return err; }
        (*(*rme32).adat_pcm).private_data = rme32 as *mut c_void;
        (*(*rme32).adat_pcm).private_free = Some(snd_rme32_free_adat_pcm);
        strscpy((*(*rme32).adat_pcm).name, b"Digi32 ADAT\0".as_ptr() as *const c_char);
        if (*rme32).fullduplex_mode != 0 {
            snd_pcm_set_ops((*rme32).adat_pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_rme32_playback_adat_fd_ops);
            snd_pcm_set_ops((*rme32).adat_pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_rme32_capture_adat_fd_ops);
            snd_pcm_set_managed_buffer_all((*rme32).adat_pcm, SNDRV_DMA_TYPE_CONTINUOUS, ptr::null_mut(), 0, RME32_MID_BUFFER_SIZE);
            (*(*rme32).adat_pcm).info_flags = SNDRV_PCM_INFO_JOINT_DUPLEX;
        } else {
            snd_pcm_set_ops((*rme32).adat_pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_rme32_playback_adat_ops);
            snd_pcm_set_ops((*rme32).adat_pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_rme32_capture_adat_ops);
            (*(*rme32).adat_pcm).info_flags = SNDRV_PCM_INFO_HALF_DUPLEX;
        }
    }
    (*rme32).playback_periodsize = 0;
    (*rme32).capture_periodsize = 0;
    snd_rme32_pcm_stop(rme32, 0);
    snd_rme32_reset_dac(rme32);
    writel(0, ptr_add((*rme32).iobase, RME32_IO_RESET_POS));
    (*rme32).wcreg = RME32_WCR_SEL | RME32_WCR_INP_0 | RME32_WCR_MUTE;
    writel((*rme32).wcreg, ptr_add((*rme32).iobase, RME32_IO_CONTROL_REGISTER));
    err = snd_rme32_create_switches((*rme32).card, rme32);
    if err < 0 { return err; }
    snd_rme32_proc_init(rme32);
    (*rme32).capture_substream = ptr::null_mut();
    (*rme32).playback_substream = ptr::null_mut();
    0
}

/*
 * proc interface
 */
unsafe extern "C" fn snd_rme32_proc_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let mut n = 0;
    let rme32 = (*entry).private_data as *mut rme32;
    (*rme32).rcreg = readl(ptr_add((*rme32).iobase, RME32_IO_CONTROL_REGISTER));
    snd_iprintf(buffer, (*(*rme32).card).longname);
    snd_iprintf(buffer, b" (index #%d)\n\0".as_ptr() as *const c_char, (*(*rme32).card).number + 1);
    snd_iprintf(buffer, b"\nGeneral settings\n\0".as_ptr() as *const c_char);
    if (*rme32).fullduplex_mode != 0 { snd_iprintf(buffer, b"  Full-duplex mode\n\0".as_ptr() as *const c_char); } else { snd_iprintf(buffer, b"  Half-duplex mode\n\0".as_ptr() as *const c_char); }
    if RME32_PRO_WITH_8414(rme32) { snd_iprintf(buffer, b"  receiver: CS8414\n\0".as_ptr() as *const c_char); } else { snd_iprintf(buffer, b"  receiver: CS8412\n\0".as_ptr() as *const c_char); }
    if ((*rme32).wcreg & RME32_WCR_MODE24) != 0 { snd_iprintf(buffer, b"  format: 24 bit\0".as_ptr() as *const c_char); } else { snd_iprintf(buffer, b"  format: 16 bit\0".as_ptr() as *const c_char); }
    if ((*rme32).wcreg & RME32_WCR_MONO) != 0 { snd_iprintf(buffer, b", Mono\n\0".as_ptr() as *const c_char); } else { snd_iprintf(buffer, b", Stereo\n\0".as_ptr() as *const c_char); }
    snd_iprintf(buffer, b"\nInput settings\n\0".as_ptr() as *const c_char);
    match snd_rme32_getinputtype(rme32) {
        RME32_INPUT_OPTICAL => snd_iprintf(buffer, b"  input: optical\0".as_ptr() as *const c_char),
        RME32_INPUT_COAXIAL => snd_iprintf(buffer, b"  input: coaxial\0".as_ptr() as *const c_char),
        RME32_INPUT_INTERNAL => snd_iprintf(buffer, b"  input: internal\0".as_ptr() as *const c_char),
        RME32_INPUT_XLR => snd_iprintf(buffer, b"  input: XLR\0".as_ptr() as *const c_char),
        _ => {}
    }
    if snd_rme32_capture_getrate(rme32, &mut n) < 0 {
        snd_iprintf(buffer, b"\n  sample rate: no valid signal\n\0".as_ptr() as *const c_char);
    } else {
        if n != 0 { snd_iprintf(buffer, b" (8 channels)\n\0".as_ptr() as *const c_char); } else { snd_iprintf(buffer, b" (2 channels)\n\0".as_ptr() as *const c_char); }
        snd_iprintf(buffer, b"  sample rate: %d Hz\n\0".as_ptr() as *const c_char, snd_rme32_capture_getrate(rme32, &mut n));
    }
    snd_iprintf(buffer, b"\nOutput settings\n\0".as_ptr() as *const c_char);
    if ((*rme32).wcreg & RME32_WCR_SEL) != 0 { snd_iprintf(buffer, b"  output signal: normal playback\0".as_ptr() as *const c_char); } else { snd_iprintf(buffer, b"  output signal: same as input\0".as_ptr() as *const c_char); }
    if ((*rme32).wcreg & RME32_WCR_MUTE) != 0 { snd_iprintf(buffer, b" (muted)\n\0".as_ptr() as *const c_char); } else { snd_iprintf(buffer, b"\n\0".as_ptr() as *const c_char); }
    if !(((*rme32).wcreg & RME32_WCR_FREQ_0) == 0 && ((*rme32).wcreg & RME32_WCR_FREQ_1) == 0) {
        snd_iprintf(buffer, b"  sample rate: %d Hz\n\0".as_ptr() as *const c_char, snd_rme32_playback_getrate(rme32));
    }
    if ((*rme32).rcreg & RME32_RCR_KMODE) != 0 { snd_iprintf(buffer, b"  sample clock source: AutoSync\n\0".as_ptr() as *const c_char); } else { snd_iprintf(buffer, b"  sample clock source: Internal\n\0".as_ptr() as *const c_char); }
    if ((*rme32).wcreg & RME32_WCR_PRO) != 0 { snd_iprintf(buffer, b"  format: AES/EBU (professional)\n\0".as_ptr() as *const c_char); } else { snd_iprintf(buffer, b"  format: IEC958 (consumer)\n\0".as_ptr() as *const c_char); }
    if ((*rme32).wcreg & RME32_WCR_EMP) != 0 { snd_iprintf(buffer, b"  emphasis: on\n\0".as_ptr() as *const c_char); } else { snd_iprintf(buffer, b"  emphasis: off\n\0".as_ptr() as *const c_char); }
}

unsafe extern "C" fn snd_rme32_proc_init(rme32: *mut rme32) {
    snd_card_ro_proc_new((*rme32).card, b"rme32\0".as_ptr() as *const c_char, rme32, snd_rme32_proc_read);
}

/*
 * control interface
 */
unsafe extern "C" fn snd_rme32_info_loopback_control(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    snd_ctl_boolean_mono_info(kcontrol, uinfo)
}

unsafe extern "C" fn snd_rme32_get_loopback_control(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rme32 = snd_kcontrol_chip(kcontrol);
    (*ucontrol).value.integer.value[0] = if ((*rme32).wcreg & RME32_WCR_SEL) != 0 { 0 } else { 1 };
    0
}

unsafe extern "C" fn snd_rme32_put_loopback_control(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rme32 = snd_kcontrol_chip(kcontrol);
    let mut val = if (*ucontrol).value.integer.value[0] != 0 { 0 } else { RME32_WCR_SEL };
    val = ((*rme32).wcreg & !RME32_WCR_SEL) | val;
    let change = (val != (*rme32).wcreg) as c_int;
    if (*ucontrol).value.integer.value[0] != 0 { val &= !RME32_WCR_MUTE; } else { val |= RME32_WCR_MUTE; }
    (*rme32).wcreg = val;
    writel(val, ptr_add((*rme32).iobase, RME32_IO_CONTROL_REGISTER));
    change
}

unsafe extern "C" fn snd_rme32_info_inputtype_control(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let rme32 = snd_kcontrol_chip(kcontrol);
    static TEXTS: [*const c_char; 4] = [b"Optical\0".as_ptr() as *const c_char, b"Coaxial\0".as_ptr() as *const c_char, b"Internal\0".as_ptr() as *const c_char, b"XLR\0".as_ptr() as *const c_char];
    let num_items = match (*(*rme32).pci).device {
        PCI_DEVICE_ID_RME_DIGI32 | PCI_DEVICE_ID_RME_DIGI32_8 => 3,
        PCI_DEVICE_ID_RME_DIGI32_PRO => 4,
        _ => { snd_BUG(); return -EINVAL; }
    };
    snd_ctl_enum_info(uinfo, 1, num_items, TEXTS.as_ptr())
}

unsafe extern "C" fn snd_rme32_get_inputtype_control(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rme32 = snd_kcontrol_chip(kcontrol);
    let mut items: u32 = 3;
    (*ucontrol).value.enumerated.item[0] = snd_rme32_getinputtype(rme32) as u32;
    match (*(*rme32).pci).device {
        PCI_DEVICE_ID_RME_DIGI32 | PCI_DEVICE_ID_RME_DIGI32_8 => items = 3,
        PCI_DEVICE_ID_RME_DIGI32_PRO => items = 4,
        _ => snd_BUG(),
    }
    if (*ucontrol).value.enumerated.item[0] >= items { (*ucontrol).value.enumerated.item[0] = items - 1; }
    0
}

unsafe extern "C" fn snd_rme32_put_inputtype_control(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rme32 = snd_kcontrol_chip(kcontrol);
    let items = match (*(*rme32).pci).device {
        PCI_DEVICE_ID_RME_DIGI32 | PCI_DEVICE_ID_RME_DIGI32_8 => 3,
        PCI_DEVICE_ID_RME_DIGI32_PRO => 4,
        _ => { snd_BUG(); 3 }
    };
    let val = (*ucontrol).value.enumerated.item[0] % items;
    let change = (val != snd_rme32_getinputtype(rme32) as u32) as c_int;
    snd_rme32_setinputtype(rme32, val as c_int);
    change
}

unsafe extern "C" fn snd_rme32_info_clockmode_control(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    static TEXTS: [*const c_char; 4] = [b"AutoSync\0".as_ptr() as *const c_char, b"Internal 32.0kHz\0".as_ptr() as *const c_char, b"Internal 44.1kHz\0".as_ptr() as *const c_char, b"Internal 48.0kHz\0".as_ptr() as *const c_char];
    snd_ctl_enum_info(uinfo, 1, 4, TEXTS.as_ptr())
}

unsafe extern "C" fn snd_rme32_get_clockmode_control(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rme32 = snd_kcontrol_chip(kcontrol);
    (*ucontrol).value.enumerated.item[0] = snd_rme32_getclockmode(rme32) as u32;
    0
}

unsafe extern "C" fn snd_rme32_put_clockmode_control(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rme32 = snd_kcontrol_chip(kcontrol);
    let val = (*ucontrol).value.enumerated.item[0] % 3;
    let change = (val != snd_rme32_getclockmode(rme32) as u32) as c_int;
    snd_rme32_setclockmode(rme32, val as c_int);
    change
}

unsafe extern "C" fn snd_rme32_convert_from_aes(aes: *mut snd_aes_iec958) -> u32 {
    let mut val = 0;
    val |= if ((*aes).status[0] & IEC958_AES0_PROFESSIONAL) != 0 { RME32_WCR_PRO } else { 0 };
    if (val & RME32_WCR_PRO) != 0 {
        val |= if ((*aes).status[0] & IEC958_AES0_PRO_EMPHASIS_5015) != 0 { RME32_WCR_EMP } else { 0 };
    } else {
        val |= if ((*aes).status[0] & IEC958_AES0_CON_EMPHASIS_5015) != 0 { RME32_WCR_EMP } else { 0 };
    }
    val
}

unsafe extern "C" fn snd_rme32_convert_to_aes(aes: *mut snd_aes_iec958, val: u32) {
    (*aes).status[0] = if (val & RME32_WCR_PRO) != 0 { IEC958_AES0_PROFESSIONAL } else { 0 };
    if (val & RME32_WCR_PRO) != 0 {
        (*aes).status[0] |= if (val & RME32_WCR_EMP) != 0 { IEC958_AES0_PRO_EMPHASIS_5015 } else { 0 };
    } else {
        (*aes).status[0] |= if (val & RME32_WCR_EMP) != 0 { IEC958_AES0_CON_EMPHASIS_5015 } else { 0 };
    }
}

unsafe extern "C" fn snd_rme32_control_spdif_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
    (*uinfo).count = 1;
    0
}

unsafe extern "C" fn snd_rme32_control_spdif_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rme32 = snd_kcontrol_chip(kcontrol);
    snd_rme32_convert_to_aes(&mut (*ucontrol).value.iec958, (*rme32).wcreg_spdif);
    0
}

unsafe extern "C" fn snd_rme32_control_spdif_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rme32 = snd_kcontrol_chip(kcontrol);
    let val = snd_rme32_convert_from_aes(&mut (*ucontrol).value.iec958);
    let change = (val != (*rme32).wcreg_spdif) as c_int;
    (*rme32).wcreg_spdif = val;
    change
}

unsafe extern "C" fn snd_rme32_control_spdif_stream_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    snd_rme32_control_spdif_info(kcontrol, uinfo)
}

unsafe extern "C" fn snd_rme32_control_spdif_stream_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rme32 = snd_kcontrol_chip(kcontrol);
    snd_rme32_convert_to_aes(&mut (*ucontrol).value.iec958, (*rme32).wcreg_spdif_stream);
    0
}

unsafe extern "C" fn snd_rme32_control_spdif_stream_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let rme32 = snd_kcontrol_chip(kcontrol);
    let val = snd_rme32_convert_from_aes(&mut (*ucontrol).value.iec958);
    let change = (val != (*rme32).wcreg_spdif_stream) as c_int;
    (*rme32).wcreg_spdif_stream = val;
    (*rme32).wcreg &= !(RME32_WCR_PRO | RME32_WCR_EMP);
    (*rme32).wcreg |= val;
    writel((*rme32).wcreg, ptr_add((*rme32).iobase, RME32_IO_CONTROL_REGISTER));
    change
}

unsafe extern "C" fn snd_rme32_control_spdif_mask_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    snd_rme32_control_spdif_info(kcontrol, uinfo)
}

unsafe extern "C" fn snd_rme32_control_spdif_mask_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    (*ucontrol).value.iec958.status[0] = (*kcontrol).private_value as u8;
    0
}

static snd_rme32_controls: [snd_kcontrol_new; 7] = [
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_PCM, name: b"IEC958 Playback Default\0".as_ptr() as *const c_char, info: Some(snd_rme32_control_spdif_info), get: Some(snd_rme32_control_spdif_get), put: Some(snd_rme32_control_spdif_put), access: 0, private_value: 0 },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_PCM, name: b"IEC958 Playback PCM Stream\0".as_ptr() as *const c_char, info: Some(snd_rme32_control_spdif_stream_info), get: Some(snd_rme32_control_spdif_stream_get), put: Some(snd_rme32_control_spdif_stream_put), access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_INACTIVE, private_value: 0 },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_PCM, name: b"IEC958 Playback Con Mask\0".as_ptr() as *const c_char, info: Some(snd_rme32_control_spdif_mask_info), get: Some(snd_rme32_control_spdif_mask_get), put: None, access: SNDRV_CTL_ELEM_ACCESS_READ, private_value: IEC958_AES0_PROFESSIONAL as c_ulong | IEC958_AES0_CON_EMPHASIS },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_PCM, name: b"IEC958 Playback Pro Mask\0".as_ptr() as *const c_char, info: Some(snd_rme32_control_spdif_mask_info), get: Some(snd_rme32_control_spdif_mask_get), put: None, access: SNDRV_CTL_ELEM_ACCESS_READ, private_value: IEC958_AES0_PROFESSIONAL as c_ulong | IEC958_AES0_PRO_EMPHASIS },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Input Connector\0".as_ptr() as *const c_char, info: Some(snd_rme32_info_inputtype_control), get: Some(snd_rme32_get_inputtype_control), put: Some(snd_rme32_put_inputtype_control), access: 0, private_value: 0 },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Loopback Input\0".as_ptr() as *const c_char, info: Some(snd_rme32_info_loopback_control), get: Some(snd_rme32_get_loopback_control), put: Some(snd_rme32_put_loopback_control), access: 0, private_value: 0 },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Sample Clock Source\0".as_ptr() as *const c_char, info: Some(snd_rme32_info_clockmode_control), get: Some(snd_rme32_get_clockmode_control), put: Some(snd_rme32_put_clockmode_control), access: 0, private_value: 0 },
];

unsafe extern "C" fn snd_rme32_create_switches(card: *mut snd_card, rme32: *mut rme32) -> c_int {
    let mut idx = 0usize;
    while idx < snd_rme32_controls.len() {
        let kctl = snd_ctl_new1(&snd_rme32_controls[idx], rme32);
        let err = snd_ctl_add(card, kctl);
        if err < 0 { return err; }
        if idx == 1 { (*rme32).spdif_ctl = kctl; }
        idx += 1;
    }
    0
}

/*
 * Card initialisation
 */
unsafe extern "C" fn snd_rme32_card_free(card: *mut snd_card) {
    snd_rme32_free((*card).private_data as *mut rme32);
}

unsafe extern "C" fn __snd_rme32_probe(pci: *mut pci_dev, _pci_id: *const pci_device_id) -> c_int {
    static mut dev: c_int = 0;
    let mut rme32: *mut rme32;
    let mut card: *mut snd_card = ptr::null_mut();
    if dev >= SNDRV_CARDS as c_int { return -ENODEV; }
    if !enable[dev as usize] {
        dev += 1;
        return -ENOENT;
    }
    let mut err = snd_devm_card_new(&mut (*pci).dev, index[dev as usize], id[dev as usize], ptr::null_mut(), size_of::<rme32>(), &mut card);
    if err < 0 { return err; }
    (*card).private_free = Some(snd_rme32_card_free);
    rme32 = (*card).private_data as *mut rme32;
    (*rme32).card = card;
    (*rme32).pci = pci;
    if fullduplex[dev as usize] { (*rme32).fullduplex_mode = 1; }
    err = snd_rme32_create(rme32);
    if err < 0 { return err; }
    strscpy((*card).driver, b"Digi32\0".as_ptr() as *const c_char);
    match (*(*rme32).pci).device {
        PCI_DEVICE_ID_RME_DIGI32 => { strscpy((*card).shortname, b"RME Digi32\0".as_ptr() as *const c_char); }
        PCI_DEVICE_ID_RME_DIGI32_8 => { strscpy((*card).shortname, b"RME Digi32/8\0".as_ptr() as *const c_char); }
        PCI_DEVICE_ID_RME_DIGI32_PRO => { strscpy((*card).shortname, b"RME Digi32 PRO\0".as_ptr() as *const c_char); }
        _ => {}
    }
    sprintf((*card).longname, b"%s (Rev. %d) at 0x%lx, irq %d\0".as_ptr() as *const c_char, (*card).shortname, (*rme32).rev as c_int, (*rme32).port, (*rme32).irq);
    err = snd_card_register(card);
    if err < 0 { return err; }
    pci_set_drvdata(pci, card);
    dev += 1;
    0
}

unsafe extern "C" fn snd_rme32_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    snd_card_free_on_error(&mut (*pci).dev, __snd_rme32_probe(pci, pci_id))
}

static mut rme32_driver: pci_driver = pci_driver {
    name: b"rme32\0".as_ptr() as *const c_char,
    id_table: snd_rme32_ids.as_ptr(),
    probe: Some(snd_rme32_probe),
};

// module_pci_driver(rme32_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
