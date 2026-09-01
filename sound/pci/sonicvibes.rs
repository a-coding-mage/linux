// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Driver for S3 SonicVibes soundcard
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *
 *  BUGS:
 *    It looks like 86c617 rev 3 doesn't supports DDMA buffers above 16MB?
 *    Driver sometimes hangs... Nobody knows why at this moment...
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type bool_t = bool;
type size_t = usize;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_uint;
type dma_addr_t = c_uint;

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: c_int = -1;
static mut SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [core::ptr::null_mut(); SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE_PNP: bool = true;

const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOSYS: c_int = 38;
const ENXIO: c_int = 6;
const EBUSY: c_int = 16;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;

const SNDRV_PCM_TRIGGER_START: c_int = 1;
const SNDRV_PCM_TRIGGER_STOP: c_int = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_int = 2;
const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_int = 1;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_int = 2;
const SNDRV_CTL_EVENT_MASK_VALUE: c_uint = 1;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 1;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint = 1 << 2;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 3;
const SNDRV_PCM_FMTBIT_U8: c_ulong = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << 1;
const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 1 << 0;
const SNDRV_PCM_RATE_8000_48000: c_uint = 1 << 1;
const DMA_BIT_MASK_24: u64 = (1u64 << 24) - 1;
const IRQF_SHARED: c_ulong = 0x80;
const MPU401_HW_SONICVIBES: c_int = 0;
const MPU401_INFO_INTEGRATED: c_uint = 1 << 0;
const MPU401_INFO_IRQ_HOOK: c_uint = 1 << 1;
const OPL3_HW_OPL3_SV: c_int = 0;

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pci_dev {
    pub dev: device,
    pub irq: c_int,
}
#[repr(C)]
pub struct snd_ctl_elem_id {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_card {
    pub private_data: *mut sonicvibes,
    pub dev: *mut device,
    pub sync_irq: c_int,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    pub mixername: *mut c_char,
    pub driver: *mut c_char,
    pub shortname: *mut c_char,
    pub longname: *mut c_char,
}
#[repr(C)]
pub struct snd_pcm {
    pub private_data: *mut c_void,
    pub info_flags: c_uint,
    pub name: *mut c_char,
}
#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
    pub channels: c_uint,
    pub format: c_int,
    pub rate: c_uint,
    pub dma_addr: dma_addr_t,
    pub rate_num: c_uint,
    pub rate_den: c_uint,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}
#[repr(C)]
pub struct snd_rawmidi {
    pub private_data: *mut snd_mpu401,
}
#[repr(C)]
pub struct snd_hwdep {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_opl3 {
    _private: [u8; 0],
}
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_kcontrol)>,
    pub id: snd_ctl_elem_id,
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
pub struct gameport {
    pub io: c_ulong,
}
#[repr(C)]
pub struct snd_mpu401 {
    pub private_data: *mut sonicvibes,
    pub open_input: Option<unsafe extern "C" fn(*mut snd_mpu401) -> c_int>,
    pub close_input: Option<unsafe extern "C" fn(*mut snd_mpu401)>,
}
#[repr(C)]
pub struct snd_ratden {
    pub num_min: c_uint,
    pub num_max: c_uint,
    pub num_step: c_uint,
    pub den: c_uint,
}
#[repr(C)]
pub struct snd_pcm_hw_constraint_ratdens {
    pub nrats: c_uint,
    pub rats: *const snd_ratden,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    pub rate_num: c_uint,
    pub rate_den: c_uint,
}
#[repr(C)]
pub struct snd_pcm_hw_rule {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_interval {
    pub min: c_uint,
    pub max: c_uint,
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
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
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
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub index: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
}
#[repr(C)]
pub struct pci_device_id {
    pub vendor: c_uint,
    pub device: c_uint,
}
#[repr(C)]
pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
}

unsafe extern "C" {
    fn outb(value: c_uint, port: c_ulong);
    fn outl(value: c_uint, port: c_ulong);
    fn inb(port: c_ulong) -> c_uint;
    fn inl(port: c_ulong) -> c_uint;
    fn udelay(usecs: c_ulong);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut sonicvibes;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: size_t) -> snd_pcm_uframes_t;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn snd_pcm_hw_rule_add(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, func: unsafe extern "C" fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> c_int, private_: *mut c_void, dep: c_int, last: c_int) -> c_int;
    fn snd_pcm_hw_constraint_ratdens(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, r: *const snd_pcm_hw_constraint_ratdens) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_mpu401_uart_interrupt(irq: c_int, private_data: *mut c_void);
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_ctl_elem_id);
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops);
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, ty: c_int, dev: *mut device, min: size_t, max: size_t);
    fn snd_ctl_enum_info(uinfo: *mut snd_ctl_elem_info, channels: c_uint, items: c_uint, texts: *const *const c_char) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut sonicvibes;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn str_off_on(value: c_uint) -> *const c_char;
    fn str_on_off(value: c_uint) -> *const c_char;
    fn snd_card_ro_proc_new(card: *mut snd_card, name: *const c_char, private_data: *mut c_void, read: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)) -> c_int;
    fn pci_write_config_dword(pci: *mut pci_dev, where_: c_int, val: c_uint) -> c_int;
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn pcim_request_all_regions(pci: *mut pci_dev, name: *const c_char) -> c_int;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn pci_read_config_dword(pci: *mut pci_dev, where_: c_int, val: *mut c_uint) -> c_int;
    fn devm_request_region(dev: *mut device, start: c_uint, n: c_uint, name: *const c_char) -> *mut resource;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn snd_devm_card_new(dev: *mut device, idx: c_int, id: *mut c_char, module: *mut c_void, extra_size: size_t, card_ret: *mut *mut snd_card) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snd_mpu401_uart_new(card: *mut snd_card, device: c_int, hardware: c_uint, port: c_ulong, info_flags: c_uint, irq: c_int, rrawmidi: *mut *mut snd_rawmidi) -> c_int;
    fn snd_opl3_create(card: *mut snd_card, l_port: c_ulong, r_port: c_ulong, hardware: c_uint, integrated: c_int, ropl3: *mut *mut snd_opl3) -> c_int;
    fn snd_opl3_hwdep_new(opl3: *mut snd_opl3, device: c_int, seq_device: c_int, rhwdep: *mut *mut snd_hwdep) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn snd_card_free_on_error(dev: *mut device, err: c_int) -> c_int;
    fn gameport_allocate_port() -> *mut gameport;
    fn gameport_set_name(gp: *mut gameport, name: *const c_char);
    fn gameport_set_phys(gp: *mut gameport, fmt: *const c_char, ...);
    fn gameport_set_dev_parent(gp: *mut gameport, dev: *mut device);
    fn gameport_register_port(gp: *mut gameport);
    fn gameport_unregister_port(gp: *mut gameport);
    fn pci_name(pci: *mut pci_dev) -> *const c_char;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn snd_BUG_ON(cond: bool) -> bool {
    cond
}

fn DIV_ROUND_CLOSEST(x: c_uint, divisor: c_uint) -> c_uint {
    x.wrapping_add(divisor / 2) / divisor
}

static mut index: [c_int; SNDRV_CARDS] = [SNDRV_DEFAULT_IDX; SNDRV_CARDS]; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_STR }; /* ID for this card */
static mut enable: [bool_t; SNDRV_CARDS] = [SNDRV_DEFAULT_ENABLE_PNP; SNDRV_CARDS]; /* Enable this card */
static mut reverb: [bool_t; SNDRV_CARDS] = [false; SNDRV_CARDS];
static mut mge: [bool_t; SNDRV_CARDS] = [false; SNDRV_CARDS];
static mut dmaio: c_uint = 0x7a00; /* DDMA i/o address */

/*
 * Enhanced port direct registers
 */

unsafe fn SV_REG(sonic: *mut sonicvibes, x: c_ulong) -> c_ulong {
    (*sonic).enh_port + x
}

const SV_REG_CONTROL: c_ulong = 0x00; /* R/W: CODEC/Mixer control register */
const SV_ENHANCED: c_uint = 0x01; /* audio mode select - enhanced mode */
const SV_TEST: c_uint = 0x02; /* test bit */
const SV_REVERB: c_uint = 0x04; /* reverb enable */
const SV_WAVETABLE: c_uint = 0x08; /* wavetable active / FM active if not set */
const SV_INTA: c_uint = 0x20; /* INTA driving - should be always 1 */
const SV_RESET: c_uint = 0x80; /* reset chip */
const SV_REG_IRQMASK: c_ulong = 0x01; /* R/W: CODEC/Mixer interrupt mask register */
const SV_DMAA_MASK: c_uint = 0x01; /* mask DMA-A interrupt */
const SV_DMAC_MASK: c_uint = 0x04; /* mask DMA-C interrupt */
const SV_SPEC_MASK: c_uint = 0x08; /* special interrupt mask - should be always masked */
const SV_UD_MASK: c_uint = 0x40; /* Up/Down button interrupt mask */
const SV_MIDI_MASK: c_uint = 0x80; /* mask MIDI interrupt */
const SV_REG_STATUS: c_ulong = 0x02; /* R/O: CODEC/Mixer status register */
const SV_DMAA_IRQ: c_uint = 0x01; /* DMA-A interrupt */
const SV_DMAC_IRQ: c_uint = 0x04; /* DMA-C interrupt */
const SV_SPEC_IRQ: c_uint = 0x08; /* special interrupt */
const SV_UD_IRQ: c_uint = 0x40; /* Up/Down interrupt */
const SV_MIDI_IRQ: c_uint = 0x80; /* MIDI interrupt */
const SV_REG_INDEX: c_ulong = 0x04; /* R/W: CODEC/Mixer index address register */
const SV_MCE: c_uint = 0x40; /* mode change enable */
const SV_TRD: c_uint = 0x80; /* DMA transfer request disabled */
const SV_REG_DATA: c_ulong = 0x05; /* R/W: CODEC/Mixer index data register */

/*
 * Enhanced port indirect registers
 */

const SV_IREG_LEFT_ADC: c_uint = 0x00; /* Left ADC Input Control */
const SV_IREG_RIGHT_ADC: c_uint = 0x01; /* Right ADC Input Control */
const SV_IREG_LEFT_AUX1: c_uint = 0x02; /* Left AUX1 Input Control */
const SV_IREG_RIGHT_AUX1: c_uint = 0x03; /* Right AUX1 Input Control */
const SV_IREG_LEFT_CD: c_uint = 0x04; /* Left CD Input Control */
const SV_IREG_RIGHT_CD: c_uint = 0x05; /* Right CD Input Control */
const SV_IREG_LEFT_LINE: c_uint = 0x06; /* Left Line Input Control */
const SV_IREG_RIGHT_LINE: c_uint = 0x07; /* Right Line Input Control */
const SV_IREG_MIC: c_uint = 0x08; /* MIC Input Control */
const SV_IREG_GAME_PORT: c_uint = 0x09; /* Game Port Control */
const SV_IREG_LEFT_SYNTH: c_uint = 0x0a; /* Left Synth Input Control */
const SV_IREG_RIGHT_SYNTH: c_uint = 0x0b; /* Right Synth Input Control */
const SV_IREG_LEFT_AUX2: c_uint = 0x0c; /* Left AUX2 Input Control */
const SV_IREG_RIGHT_AUX2: c_uint = 0x0d; /* Right AUX2 Input Control */
const SV_IREG_LEFT_ANALOG: c_uint = 0x0e; /* Left Analog Mixer Output Control */
const SV_IREG_RIGHT_ANALOG: c_uint = 0x0f; /* Right Analog Mixer Output Control */
const SV_IREG_LEFT_PCM: c_uint = 0x10; /* Left PCM Input Control */
const SV_IREG_RIGHT_PCM: c_uint = 0x11; /* Right PCM Input Control */
const SV_IREG_DMA_DATA_FMT: c_uint = 0x12; /* DMA Data Format */
const SV_IREG_PC_ENABLE: c_uint = 0x13; /* Playback/Capture Enable Register */
const SV_IREG_UD_BUTTON: c_uint = 0x14; /* Up/Down Button Register */
const SV_IREG_REVISION: c_uint = 0x15; /* Revision */
const SV_IREG_ADC_OUTPUT_CTRL: c_uint = 0x16; /* ADC Output Control */
const SV_IREG_DMA_A_UPPER: c_uint = 0x18; /* DMA A Upper Base Count */
const SV_IREG_DMA_A_LOWER: c_uint = 0x19; /* DMA A Lower Base Count */
const SV_IREG_DMA_C_UPPER: c_uint = 0x1c; /* DMA C Upper Base Count */
const SV_IREG_DMA_C_LOWER: c_uint = 0x1d; /* DMA C Lower Base Count */
const SV_IREG_PCM_RATE_LOW: c_uint = 0x1e; /* PCM Sampling Rate Low Byte */
const SV_IREG_PCM_RATE_HIGH: c_uint = 0x1f; /* PCM Sampling Rate High Byte */
const SV_IREG_SYNTH_RATE_LOW: c_uint = 0x20; /* Synthesizer Sampling Rate Low Byte */
const SV_IREG_SYNTH_RATE_HIGH: c_uint = 0x21; /* Synthesizer Sampling Rate High Byte */
const SV_IREG_ADC_CLOCK: c_uint = 0x22; /* ADC Clock Source Selection */
const SV_IREG_ADC_ALT_RATE: c_uint = 0x23; /* ADC Alternative Sampling Rate Selection */
const SV_IREG_ADC_PLL_M: c_uint = 0x24; /* ADC PLL M Register */
const SV_IREG_ADC_PLL_N: c_uint = 0x25; /* ADC PLL N Register */
const SV_IREG_SYNTH_PLL_M: c_uint = 0x26; /* Synthesizer PLL M Register */
const SV_IREG_SYNTH_PLL_N: c_uint = 0x27; /* Synthesizer PLL N Register */
const SV_IREG_MPU401: c_uint = 0x2a; /* MPU-401 UART Operation */
const SV_IREG_DRIVE_CTRL: c_uint = 0x2b; /* Drive Control */
const SV_IREG_SRS_SPACE: c_uint = 0x2c; /* SRS Space Control */
const SV_IREG_SRS_CENTER: c_uint = 0x2d; /* SRS Center Control */
const SV_IREG_WAVE_SOURCE: c_uint = 0x2e; /* Wavetable Sample Source Select */
const SV_IREG_ANALOG_POWER: c_uint = 0x30; /* Analog Power Down Control */
const SV_IREG_DIGITAL_POWER: c_uint = 0x31; /* Digital Power Down Control */

const SV_IREG_ADC_PLL: c_uint = SV_IREG_ADC_PLL_M;
const SV_IREG_SYNTH_PLL: c_uint = SV_IREG_SYNTH_PLL_M;

/*
 *  DMA registers
 */

const SV_DMA_ADDR0: c_ulong = 0x00;
const SV_DMA_ADDR1: c_ulong = 0x01;
const SV_DMA_ADDR2: c_ulong = 0x02;
const SV_DMA_ADDR3: c_ulong = 0x03;
const SV_DMA_COUNT0: c_ulong = 0x04;
const SV_DMA_COUNT1: c_ulong = 0x05;
const SV_DMA_COUNT2: c_ulong = 0x06;
const SV_DMA_MODE: c_ulong = 0x0b;
const SV_DMA_RESET: c_ulong = 0x0d;
const SV_DMA_MASK: c_ulong = 0x0f;

/*
 *  Record sources
 */

const SV_RECSRC_RESERVED: c_uint = 0x00 << 5;
const SV_RECSRC_CD: c_uint = 0x01 << 5;
const SV_RECSRC_DAC: c_uint = 0x02 << 5;
const SV_RECSRC_AUX2: c_uint = 0x03 << 5;
const SV_RECSRC_LINE: c_uint = 0x04 << 5;
const SV_RECSRC_AUX1: c_uint = 0x05 << 5;
const SV_RECSRC_MIC: c_uint = 0x06 << 5;
const SV_RECSRC_OUT: c_uint = 0x07 << 5;

/*
 *  constants
 */

const SV_FULLRATE: c_uint = 48000;
const SV_REFFREQUENCY: c_uint = 24576000;
const SV_ADCMULT: c_uint = 512;

const SV_MODE_PLAY: c_uint = 1;
const SV_MODE_CAPTURE: c_uint = 2;

#[repr(C)]
pub struct sonicvibes {
    pub dma1size: c_ulong,
    pub dma2size: c_ulong,
    pub irq: c_int,
    pub sb_port: c_ulong,
    pub enh_port: c_ulong,
    pub synth_port: c_ulong,
    pub midi_port: c_ulong,
    pub game_port: c_ulong,
    pub dmaa_port: c_uint,
    pub res_dmaa: *mut resource,
    pub dmac_port: c_uint,
    pub res_dmac: *mut resource,
    pub enable: u8,
    pub irqmask: u8,
    pub revision: u8,
    pub format: u8,
    pub srs_space: u8,
    pub srs_center: u8,
    pub mpu_switch: u8,
    pub wave_source: u8,
    pub mode: c_uint,
    pub pci: *mut pci_dev,
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub rmidi: *mut snd_rawmidi,
    pub fmsynth: *mut snd_hwdep, /* S3FM */
    pub reg_lock: spinlock_t,
    pub p_dma_size: c_uint,
    pub c_dma_size: c_uint,
    pub master_mute: *mut snd_kcontrol,
    pub master_volume: *mut snd_kcontrol,
    /* #ifdef SUPPORT_JOYSTICK */
    pub gameport: *mut gameport,
}

static snd_sonic_ids: [pci_device_id; 2] = [
    pci_device_id { vendor: 0x5333, device: 0xca00 },
    pci_device_id { vendor: 0, device: 0 },
];

static sonicvibes_adc_clock: snd_ratden = snd_ratden {
    num_min: 4000 * 65536,
    num_max: 48000 * 65536,
    num_step: 1,
    den: 65536,
};
static snd_sonicvibes_hw_constraints_adc_clock: snd_pcm_hw_constraint_ratdens =
    snd_pcm_hw_constraint_ratdens {
        nrats: 1,
        rats: &sonicvibes_adc_clock,
    };

/*
 *  common I/O routines
 */

unsafe fn snd_sonicvibes_setdmaa(sonic: *mut sonicvibes, addr: c_uint, mut count: c_uint) {
    count = count.wrapping_sub(1);
    outl(addr, ((*sonic).dmaa_port as c_ulong) + SV_DMA_ADDR0);
    outl(count, ((*sonic).dmaa_port as c_ulong) + SV_DMA_COUNT0);
    outb(0x18, ((*sonic).dmaa_port as c_ulong) + SV_DMA_MODE);
    /* #if 0: debug print of programmed DMA-A address. */
}

unsafe fn snd_sonicvibes_setdmac(sonic: *mut sonicvibes, addr: c_uint, mut count: c_uint) {
    /* note: dmac is working in word mode!!! */
    count >>= 1;
    count = count.wrapping_sub(1);
    outl(addr, ((*sonic).dmac_port as c_ulong) + SV_DMA_ADDR0);
    outl(count, ((*sonic).dmac_port as c_ulong) + SV_DMA_COUNT0);
    outb(0x14, ((*sonic).dmac_port as c_ulong) + SV_DMA_MODE);
    /* #if 0: debug print of programmed DMA-C address. */
}

unsafe fn snd_sonicvibes_getdmaa(sonic: *mut sonicvibes) -> c_uint {
    (inl(((*sonic).dmaa_port as c_ulong) + SV_DMA_COUNT0) & 0x00ff_ffff).wrapping_add(1)
}

unsafe fn snd_sonicvibes_getdmac(sonic: *mut sonicvibes) -> c_uint {
    /* note: dmac is working in word mode!!! */
    ((inl(((*sonic).dmac_port as c_ulong) + SV_DMA_COUNT0) & 0x00ff_ffff).wrapping_add(1)) << 1
}

unsafe fn snd_sonicvibes_out1(sonic: *mut sonicvibes, reg: u8, value: u8) {
    outb(reg as c_uint, SV_REG(sonic, SV_REG_INDEX));
    udelay(10);
    outb(value as c_uint, SV_REG(sonic, SV_REG_DATA));
    udelay(10);
}

unsafe fn snd_sonicvibes_out(sonic: *mut sonicvibes, reg: u8, value: u8) {
    /* guard(spinlock_irqsave)(&sonic->reg_lock); */
    outb(reg as c_uint, SV_REG(sonic, SV_REG_INDEX));
    udelay(10);
    outb(value as c_uint, SV_REG(sonic, SV_REG_DATA));
    udelay(10);
}

unsafe fn snd_sonicvibes_in1(sonic: *mut sonicvibes, reg: u8) -> u8 {
    let value: u8;
    outb(reg as c_uint, SV_REG(sonic, SV_REG_INDEX));
    udelay(10);
    value = inb(SV_REG(sonic, SV_REG_DATA)) as u8;
    udelay(10);
    value
}

unsafe fn snd_sonicvibes_in(sonic: *mut sonicvibes, reg: u8) -> u8 {
    let value: u8;
    /* guard(spinlock_irqsave)(&sonic->reg_lock); */
    outb(reg as c_uint, SV_REG(sonic, SV_REG_INDEX));
    udelay(10);
    value = inb(SV_REG(sonic, SV_REG_DATA)) as u8;
    udelay(10);
    value
}

/* #if 0: snd_sonicvibes_debug() register dump helper intentionally disabled in C. */

unsafe fn snd_sonicvibes_setfmt(sonic: *mut sonicvibes, mask: u8, value: u8) {
    /* guard(spinlock_irqsave)(&sonic->reg_lock); */
    outb((SV_MCE | SV_IREG_DMA_DATA_FMT) as c_uint, SV_REG(sonic, SV_REG_INDEX));
    if mask != 0 {
        (*sonic).format = inb(SV_REG(sonic, SV_REG_DATA)) as u8;
        udelay(10);
    }
    (*sonic).format = ((*sonic).format & mask) | value;
    outb((*sonic).format as c_uint, SV_REG(sonic, SV_REG_DATA));
    udelay(10);
    outb(0, SV_REG(sonic, SV_REG_INDEX));
    udelay(10);
}

unsafe fn snd_sonicvibes_pll(rate: c_uint, res_r: *mut c_uint, res_m: *mut c_uint, res_n: *mut c_uint) {
    let mut rate = rate;
    let mut r: c_uint;
    let mut m: c_uint = 0;
    let mut n: c_uint = 0;
    let mut metric: c_uint = !0;

    if rate < 625000 / SV_ADCMULT {
        rate = 625000 / SV_ADCMULT;
    }
    if rate > 150000000 / SV_ADCMULT {
        rate = 150000000 / SV_ADCMULT;
    }
    /* slight violation of specs, needed for continuous sampling rates */
    r = 0;
    while rate < 75000000 / SV_ADCMULT {
        r = r.wrapping_add(0x20);
        rate <<= 1;
    }
    let mut xn: c_uint = 3;
    while xn < 33 {
        let mut xm: c_uint = 3;
        while xm < 257 {
            let xr = ((SV_REFFREQUENCY / SV_ADCMULT) * xm) / xn;
            let xd = if xr >= rate { xr - rate } else { rate - xr };
            if xd < metric {
                metric = xd;
                m = xm - 2;
                n = xn - 2;
            }
            xm += 1;
        }
        xn += 1;
    }
    *res_r = r;
    *res_m = m;
    *res_n = n;
    /* #if 0: PLL debug prints. */
}

unsafe fn snd_sonicvibes_setpll(sonic: *mut sonicvibes, reg: u8, rate: c_uint) {
    let mut r: c_uint = 0;
    let mut m: c_uint = 0;
    let mut n: c_uint = 0;

    snd_sonicvibes_pll(rate, &mut r, &mut m, &mut n);
    if !sonic.is_null() {
        /* guard(spinlock_irqsave)(&sonic->reg_lock); */
        snd_sonicvibes_out1(sonic, reg, m as u8);
        snd_sonicvibes_out1(sonic, reg.wrapping_add(1), (r | n) as u8);
    }
}

unsafe fn snd_sonicvibes_set_adc_rate(sonic: *mut sonicvibes, rate: c_uint) {
    let mut div: c_uint;
    let clock: u8;

    div = 48000 / rate;
    if div > 8 {
        div = 8;
    }
    if (48000 / div) == rate {
        /* use the alternate clock */
        clock = 0x10;
    } else {
        /* use the PLL source */
        clock = 0x00;
        snd_sonicvibes_setpll(sonic, SV_IREG_ADC_PLL as u8, rate);
    }
    /* guard(spinlock_irqsave)(&sonic->reg_lock); */
    snd_sonicvibes_out1(sonic, SV_IREG_ADC_ALT_RATE as u8, ((div - 1) << 4) as u8);
    snd_sonicvibes_out1(sonic, SV_IREG_ADC_CLOCK as u8, clock);
}

unsafe extern "C" fn snd_sonicvibes_hw_constraint_dac_rate(params: *mut snd_pcm_hw_params, _rule: *mut snd_pcm_hw_rule) -> c_int {
    let mut rate: c_uint;
    let mut div: c_uint;
    let mut r: c_uint = 0;
    let mut m: c_uint = 0;
    let mut n: c_uint = 0;

    if (*hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE)).min ==
        (*hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE)).max {
        rate = (*hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE)).min;
        div = 48000 / rate;
        if div > 8 {
            div = 8;
        }
        if (48000 / div) == rate {
            (*params).rate_num = rate;
            (*params).rate_den = 1;
        } else {
            snd_sonicvibes_pll(rate, &mut r, &mut m, &mut n);
            snd_BUG_ON(SV_REFFREQUENCY % 16 != 0);
            snd_BUG_ON(SV_ADCMULT % 512 != 0);
            (*params).rate_num = (SV_REFFREQUENCY / 16) * (n + 2) * r;
            (*params).rate_den = (SV_ADCMULT / 512) * (m + 2);
        }
    }
    0
}

unsafe fn snd_sonicvibes_set_dac_rate(sonic: *mut sonicvibes, rate: c_uint) {
    let mut div: c_uint;

    div = DIV_ROUND_CLOSEST(rate * 65536, SV_FULLRATE);
    if div > 65535 {
        div = 65535;
    }
    /* guard(spinlock_irqsave)(&sonic->reg_lock); */
    snd_sonicvibes_out1(sonic, SV_IREG_PCM_RATE_HIGH as u8, (div >> 8) as u8);
    snd_sonicvibes_out1(sonic, SV_IREG_PCM_RATE_LOW as u8, div as u8);
}

unsafe fn snd_sonicvibes_trigger(sonic: *mut sonicvibes, what: c_int, cmd: c_int) -> c_int {
    /* guard(spinlock)(&sonic->reg_lock); */
    if cmd == SNDRV_PCM_TRIGGER_START {
        if ((*sonic).enable as c_int & what) == 0 {
            (*sonic).enable |= what as u8;
            snd_sonicvibes_out1(sonic, SV_IREG_PC_ENABLE as u8, (*sonic).enable);
        }
    } else if cmd == SNDRV_PCM_TRIGGER_STOP {
        if ((*sonic).enable as c_int & what) != 0 {
            (*sonic).enable &= !(what as u8);
            snd_sonicvibes_out1(sonic, SV_IREG_PC_ENABLE as u8, (*sonic).enable);
        }
    } else {
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn snd_sonicvibes_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let sonic = dev_id as *mut sonicvibes;
    let status: u8;

    status = inb(SV_REG(sonic, SV_REG_STATUS)) as u8;
    if (status as c_uint & (SV_DMAA_IRQ | SV_DMAC_IRQ | SV_MIDI_IRQ)) == 0 {
        return IRQ_NONE;
    }
    if status == 0xff {
        /* failure */
        (*sonic).irqmask = !0;
        outb((*sonic).irqmask as c_uint, SV_REG(sonic, SV_REG_IRQMASK));
        dev_err((*(*sonic).card).dev, cstr!("IRQ failure - interrupts disabled!!\n"));
        return IRQ_HANDLED;
    }
    if !(*sonic).pcm.is_null() {
        if (status as c_uint & SV_DMAA_IRQ) != 0 {
            snd_pcm_period_elapsed((*sonic).playback_substream);
        }
        if (status as c_uint & SV_DMAC_IRQ) != 0 {
            snd_pcm_period_elapsed((*sonic).capture_substream);
        }
    }
    if !(*sonic).rmidi.is_null() {
        if (status as c_uint & SV_MIDI_IRQ) != 0 {
            snd_mpu401_uart_interrupt(irq, (*(*sonic).rmidi).private_data as *mut c_void);
        }
    }
    if (status as c_uint & SV_UD_IRQ) != 0 {
        let mut udreg: u8;
        let mut vol: c_int;
        let mut oleft: c_int;
        let mut oright: c_int;
        let mut mleft: c_int;
        let mut mright: c_int;

        /* scoped_guard(spinlock, &sonic->reg_lock) */
        udreg = snd_sonicvibes_in1(sonic, SV_IREG_UD_BUTTON as u8);
        vol = (udreg & 0x3f) as c_int;
        if (udreg & 0x40) == 0 {
            vol = -vol;
        }
        mleft = snd_sonicvibes_in1(sonic, SV_IREG_LEFT_ANALOG as u8) as c_int;
        oleft = mleft;
        mright = snd_sonicvibes_in1(sonic, SV_IREG_RIGHT_ANALOG as u8) as c_int;
        oright = mright;
        oleft &= 0x1f;
        oright &= 0x1f;
        oleft += vol;
        if oleft < 0 { oleft = 0; }
        if oleft > 0x1f { oleft = 0x1f; }
        oright += vol;
        if oright < 0 { oright = 0; }
        if oright > 0x1f { oright = 0x1f; }
        if (udreg & 0x80) != 0 {
            mleft ^= 0x80;
            mright ^= 0x80;
        }
        oleft |= mleft & 0x80;
        oright |= mright & 0x80;
        snd_sonicvibes_out1(sonic, SV_IREG_LEFT_ANALOG as u8, oleft as u8);
        snd_sonicvibes_out1(sonic, SV_IREG_RIGHT_ANALOG as u8, oright as u8);
        snd_ctl_notify((*sonic).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*sonic).master_mute).id);
        snd_ctl_notify((*sonic).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*sonic).master_volume).id);
    }
    IRQ_HANDLED
}

/*
 *  PCM part
 */

unsafe extern "C" fn snd_sonicvibes_playback_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let sonic = snd_pcm_substream_chip(substream);
    snd_sonicvibes_trigger(sonic, 1, cmd)
}

unsafe extern "C" fn snd_sonicvibes_capture_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let sonic = snd_pcm_substream_chip(substream);
    snd_sonicvibes_trigger(sonic, 2, cmd)
}

unsafe extern "C" fn snd_sonicvibes_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let sonic = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let mut fmt: u8 = 0;
    let size = snd_pcm_lib_buffer_bytes(substream);
    let mut count = snd_pcm_lib_period_bytes(substream);

    (*sonic).p_dma_size = size;
    count = count.wrapping_sub(1);
    if (*runtime).channels > 1 {
        fmt |= 1;
    }
    if snd_pcm_format_width((*runtime).format) == 16 {
        fmt |= 2;
    }
    snd_sonicvibes_setfmt(sonic, !3u8, fmt);
    snd_sonicvibes_set_dac_rate(sonic, (*runtime).rate);
    /* guard(spinlock_irq)(&sonic->reg_lock); */
    snd_sonicvibes_setdmaa(sonic, (*runtime).dma_addr, size);
    snd_sonicvibes_out1(sonic, SV_IREG_DMA_A_UPPER as u8, (count >> 8) as u8);
    snd_sonicvibes_out1(sonic, SV_IREG_DMA_A_LOWER as u8, count as u8);
    0
}

unsafe extern "C" fn snd_sonicvibes_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let sonic = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let mut fmt: u8 = 0;
    let size = snd_pcm_lib_buffer_bytes(substream);
    let mut count = snd_pcm_lib_period_bytes(substream);

    (*sonic).c_dma_size = size;
    count >>= 1;
    count = count.wrapping_sub(1);
    if (*runtime).channels > 1 {
        fmt |= 0x10;
    }
    if snd_pcm_format_width((*runtime).format) == 16 {
        fmt |= 0x20;
    }
    snd_sonicvibes_setfmt(sonic, !0x30u8, fmt);
    snd_sonicvibes_set_adc_rate(sonic, (*runtime).rate);
    /* guard(spinlock_irq)(&sonic->reg_lock); */
    snd_sonicvibes_setdmac(sonic, (*runtime).dma_addr, size);
    snd_sonicvibes_out1(sonic, SV_IREG_DMA_C_UPPER as u8, (count >> 8) as u8);
    snd_sonicvibes_out1(sonic, SV_IREG_DMA_C_LOWER as u8, count as u8);
    0
}

unsafe extern "C" fn snd_sonicvibes_playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let sonic = snd_pcm_substream_chip(substream);
    let ptr: size_t;

    if ((*sonic).enable & 1) == 0 {
        return 0;
    }
    ptr = ((*sonic).p_dma_size - snd_sonicvibes_getdmaa(sonic)) as size_t;
    bytes_to_frames((*substream).runtime, ptr)
}

unsafe extern "C" fn snd_sonicvibes_capture_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let sonic = snd_pcm_substream_chip(substream);
    let ptr: size_t;
    if ((*sonic).enable & 2) == 0 {
        return 0;
    }
    ptr = ((*sonic).c_dma_size - snd_sonicvibes_getdmac(sonic)) as size_t;
    bytes_to_frames((*substream).runtime, ptr)
}

static snd_sonicvibes_playback: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    rate_min: 4000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 128 * 1024,
    period_bytes_min: 32,
    period_bytes_max: 128 * 1024,
    periods_min: 1,
    periods_max: 1024,
    fifo_size: 0,
};

static snd_sonicvibes_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    rate_min: 4000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 128 * 1024,
    period_bytes_min: 32,
    period_bytes_max: 128 * 1024,
    periods_min: 1,
    periods_max: 1024,
    fifo_size: 0,
};

unsafe extern "C" fn snd_sonicvibes_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let sonic = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;

    (*sonic).mode |= SV_MODE_PLAY;
    (*sonic).playback_substream = substream;
    (*runtime).hw = snd_sonicvibes_playback;
    snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, snd_sonicvibes_hw_constraint_dac_rate, core::ptr::null_mut(), SNDRV_PCM_HW_PARAM_RATE, -1);
    0
}

unsafe extern "C" fn snd_sonicvibes_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let sonic = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;

    (*sonic).mode |= SV_MODE_CAPTURE;
    (*sonic).capture_substream = substream;
    (*runtime).hw = snd_sonicvibes_capture;
    snd_pcm_hw_constraint_ratdens(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &snd_sonicvibes_hw_constraints_adc_clock);
    0
}

unsafe extern "C" fn snd_sonicvibes_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    let sonic = snd_pcm_substream_chip(substream);

    (*sonic).playback_substream = core::ptr::null_mut();
    (*sonic).mode &= !SV_MODE_PLAY;
    0
}

unsafe extern "C" fn snd_sonicvibes_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    let sonic = snd_pcm_substream_chip(substream);

    (*sonic).capture_substream = core::ptr::null_mut();
    (*sonic).mode &= !SV_MODE_CAPTURE;
    0
}

static snd_sonicvibes_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_sonicvibes_playback_open),
    close: Some(snd_sonicvibes_playback_close),
    prepare: Some(snd_sonicvibes_playback_prepare),
    trigger: Some(snd_sonicvibes_playback_trigger),
    pointer: Some(snd_sonicvibes_playback_pointer),
};

static snd_sonicvibes_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_sonicvibes_capture_open),
    close: Some(snd_sonicvibes_capture_close),
    prepare: Some(snd_sonicvibes_capture_prepare),
    trigger: Some(snd_sonicvibes_capture_trigger),
    pointer: Some(snd_sonicvibes_capture_pointer),
};

unsafe fn snd_sonicvibes_pcm(sonic: *mut sonicvibes, device: c_int) -> c_int {
    let mut pcm: *mut snd_pcm = core::ptr::null_mut();
    let mut err: c_int;

    err = snd_pcm_new((*sonic).card, cstr!("s3_86c617"), device, 1, 1, &mut pcm);
    if err < 0 {
        return err;
    }
    if snd_BUG_ON(pcm.is_null()) {
        return -EINVAL;
    }

    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_sonicvibes_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_sonicvibes_capture_ops);

    (*pcm).private_data = sonic as *mut c_void;
    (*pcm).info_flags = 0;
    strscpy((*pcm).name, cstr!("S3 SonicVibes"));
    (*sonic).pcm = pcm;

    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &mut (*(*sonic).pci).dev, 64 * 1024, 128 * 1024);

    0
}

/*
 *  Mixer part
 */

const fn SONICVIBES_MUX(xname: *const c_char, xindex: c_uint) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER as c_uint,
        name: xname,
        index: xindex,
        info: Some(snd_sonicvibes_info_mux),
        get: Some(snd_sonicvibes_get_mux),
        put: Some(snd_sonicvibes_put_mux),
        private_value: 0,
    }
}

unsafe extern "C" fn snd_sonicvibes_info_mux(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    static texts: [*const c_char; 7] = [
        cstr!("CD"), cstr!("PCM"), cstr!("Aux1"), cstr!("Line"), cstr!("Aux0"), cstr!("Mic"), cstr!("Mix"),
    ];

    snd_ctl_enum_info(uinfo, 2, 7, texts.as_ptr())
}

unsafe extern "C" fn snd_sonicvibes_get_mux(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let sonic = snd_kcontrol_chip(kcontrol);

    /* guard(spinlock_irq)(&sonic->reg_lock); */
    (*ucontrol).value.enumerated.item[0] = (((snd_sonicvibes_in1(sonic, SV_IREG_LEFT_ADC as u8) as c_uint & SV_RECSRC_OUT) >> 5) - 1) as c_uint;
    (*ucontrol).value.enumerated.item[1] = (((snd_sonicvibes_in1(sonic, SV_IREG_RIGHT_ADC as u8) as c_uint & SV_RECSRC_OUT) >> 5) - 1) as c_uint;
    0
}

unsafe extern "C" fn snd_sonicvibes_put_mux(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let sonic = snd_kcontrol_chip(kcontrol);
    let mut left: u16;
    let mut right: u16;
    let oval1: u16;
    let oval2: u16;
    let change: c_int;

    if (*ucontrol).value.enumerated.item[0] >= 7 || (*ucontrol).value.enumerated.item[1] >= 7 {
        return -EINVAL;
    }
    left = (((*ucontrol).value.enumerated.item[0] + 1) << 5) as u16;
    right = (((*ucontrol).value.enumerated.item[1] + 1) << 5) as u16;
    /* guard(spinlock_irq)(&sonic->reg_lock); */
    oval1 = snd_sonicvibes_in1(sonic, SV_IREG_LEFT_ADC as u8) as u16;
    oval2 = snd_sonicvibes_in1(sonic, SV_IREG_RIGHT_ADC as u8) as u16;
    left = (oval1 & !(SV_RECSRC_OUT as u16)) | left;
    right = (oval2 & !(SV_RECSRC_OUT as u16)) | right;
    change = (left != oval1 || right != oval2) as c_int;
    snd_sonicvibes_out1(sonic, SV_IREG_LEFT_ADC as u8, left as u8);
    snd_sonicvibes_out1(sonic, SV_IREG_RIGHT_ADC as u8, right as u8);
    change
}

const fn SONICVIBES_SINGLE(xname: *const c_char, xindex: c_uint, reg: c_uint, shift: c_uint, mask: c_uint, invert: c_uint) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER as c_uint,
        name: xname,
        index: xindex,
        info: Some(snd_sonicvibes_info_single),
        get: Some(snd_sonicvibes_get_single),
        put: Some(snd_sonicvibes_put_single),
        private_value: (reg | (shift << 8) | (mask << 16) | (invert << 24)) as c_ulong,
    }
}

unsafe extern "C" fn snd_sonicvibes_info_single(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let mask = (((*kcontrol).private_value >> 16) & 0xff) as c_int;

    (*uinfo).type_ = if mask == 1 { SNDRV_CTL_ELEM_TYPE_BOOLEAN } else { SNDRV_CTL_ELEM_TYPE_INTEGER } as c_uint;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = mask as c_long;
    0
}

unsafe extern "C" fn snd_sonicvibes_get_single(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let sonic = snd_kcontrol_chip(kcontrol);
    let reg = ((*kcontrol).private_value & 0xff) as c_uint;
    let shift = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    let mask = (((*kcontrol).private_value >> 16) & 0xff) as c_long;
    let invert = (((*kcontrol).private_value >> 24) & 0xff) as c_int;

    /* guard(spinlock_irq)(&sonic->reg_lock); */
    (*ucontrol).value.integer.value[0] = (((snd_sonicvibes_in1(sonic, reg as u8) as c_long) >> shift) & mask) as c_long;
    if invert != 0 {
        (*ucontrol).value.integer.value[0] = mask - (*ucontrol).value.integer.value[0];
    }
    0
}

unsafe extern "C" fn snd_sonicvibes_put_single(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let sonic = snd_kcontrol_chip(kcontrol);
    let reg = ((*kcontrol).private_value & 0xff) as c_uint;
    let shift = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    let mask = (((*kcontrol).private_value >> 16) & 0xff) as u16;
    let invert = (((*kcontrol).private_value >> 24) & 0xff) as c_int;
    let change: c_int;
    let mut val: u16;
    let oval: u16;

    val = ((*ucontrol).value.integer.value[0] as u16) & mask;
    if invert != 0 {
        val = mask - val;
    }
    val <<= shift;
    /* guard(spinlock_irq)(&sonic->reg_lock); */
    oval = snd_sonicvibes_in1(sonic, reg as u8) as u16;
    val = (oval & !(mask << shift)) | val;
    change = (val != oval) as c_int;
    snd_sonicvibes_out1(sonic, reg as u8, val as u8);
    change
}

const fn SONICVIBES_DOUBLE(xname: *const c_char, xindex: c_uint, left_reg: c_uint, right_reg: c_uint, shift_left: c_uint, shift_right: c_uint, mask: c_uint, invert: c_uint) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER as c_uint,
        name: xname,
        index: xindex,
        info: Some(snd_sonicvibes_info_double),
        get: Some(snd_sonicvibes_get_double),
        put: Some(snd_sonicvibes_put_double),
        private_value: (left_reg | (right_reg << 8) | (shift_left << 16) | (shift_right << 19) | (mask << 24) | (invert << 22)) as c_ulong,
    }
}

unsafe extern "C" fn snd_sonicvibes_info_double(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let mask = (((*kcontrol).private_value >> 24) & 0xff) as c_int;

    (*uinfo).type_ = if mask == 1 { SNDRV_CTL_ELEM_TYPE_BOOLEAN } else { SNDRV_CTL_ELEM_TYPE_INTEGER } as c_uint;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = mask as c_long;
    0
}

unsafe extern "C" fn snd_sonicvibes_get_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let sonic = snd_kcontrol_chip(kcontrol);
    let left_reg = ((*kcontrol).private_value & 0xff) as c_uint;
    let right_reg = (((*kcontrol).private_value >> 8) & 0xff) as c_uint;
    let shift_left = (((*kcontrol).private_value >> 16) & 0x07) as c_int;
    let shift_right = (((*kcontrol).private_value >> 19) & 0x07) as c_int;
    let mask = (((*kcontrol).private_value >> 24) & 0xff) as c_long;
    let invert = (((*kcontrol).private_value >> 22) & 1) as c_int;

    /* guard(spinlock_irq)(&sonic->reg_lock); */
    (*ucontrol).value.integer.value[0] = (((snd_sonicvibes_in1(sonic, left_reg as u8) as c_long) >> shift_left) & mask) as c_long;
    (*ucontrol).value.integer.value[1] = (((snd_sonicvibes_in1(sonic, right_reg as u8) as c_long) >> shift_right) & mask) as c_long;
    if invert != 0 {
        (*ucontrol).value.integer.value[0] = mask - (*ucontrol).value.integer.value[0];
        (*ucontrol).value.integer.value[1] = mask - (*ucontrol).value.integer.value[1];
    }
    0
}

unsafe extern "C" fn snd_sonicvibes_put_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let sonic = snd_kcontrol_chip(kcontrol);
    let left_reg = ((*kcontrol).private_value & 0xff) as c_uint;
    let right_reg = (((*kcontrol).private_value >> 8) & 0xff) as c_uint;
    let shift_left = (((*kcontrol).private_value >> 16) & 0x07) as c_int;
    let shift_right = (((*kcontrol).private_value >> 19) & 0x07) as c_int;
    let mask = (((*kcontrol).private_value >> 24) & 0xff) as u16;
    let invert = (((*kcontrol).private_value >> 22) & 1) as c_int;
    let change: c_int;
    let mut val1: u16;
    let mut val2: u16;
    let oval1: u16;
    let oval2: u16;

    val1 = ((*ucontrol).value.integer.value[0] as u16) & mask;
    val2 = ((*ucontrol).value.integer.value[1] as u16) & mask;
    if invert != 0 {
        val1 = mask - val1;
        val2 = mask - val2;
    }
    val1 <<= shift_left;
    val2 <<= shift_right;
    /* guard(spinlock_irq)(&sonic->reg_lock); */
    oval1 = snd_sonicvibes_in1(sonic, left_reg as u8) as u16;
    oval2 = snd_sonicvibes_in1(sonic, right_reg as u8) as u16;
    val1 = (oval1 & !(mask << shift_left)) | val1;
    val2 = (oval2 & !(mask << shift_right)) | val2;
    change = (val1 != oval1 || val2 != oval2) as c_int;
    snd_sonicvibes_out1(sonic, left_reg as u8, val1 as u8);
    snd_sonicvibes_out1(sonic, right_reg as u8, val2 as u8);
    change
}

static snd_sonicvibes_controls: [snd_kcontrol_new; 20] = [
    SONICVIBES_DOUBLE(cstr!("Capture Volume"), 0, SV_IREG_LEFT_ADC, SV_IREG_RIGHT_ADC, 0, 0, 15, 0),
    SONICVIBES_DOUBLE(cstr!("Aux Playback Switch"), 0, SV_IREG_LEFT_AUX1, SV_IREG_RIGHT_AUX1, 7, 7, 1, 1),
    SONICVIBES_DOUBLE(cstr!("Aux Playback Volume"), 0, SV_IREG_LEFT_AUX1, SV_IREG_RIGHT_AUX1, 0, 0, 31, 1),
    SONICVIBES_DOUBLE(cstr!("CD Playback Switch"), 0, SV_IREG_LEFT_CD, SV_IREG_RIGHT_CD, 7, 7, 1, 1),
    SONICVIBES_DOUBLE(cstr!("CD Playback Volume"), 0, SV_IREG_LEFT_CD, SV_IREG_RIGHT_CD, 0, 0, 31, 1),
    SONICVIBES_DOUBLE(cstr!("Line Playback Switch"), 0, SV_IREG_LEFT_LINE, SV_IREG_RIGHT_LINE, 7, 7, 1, 1),
    SONICVIBES_DOUBLE(cstr!("Line Playback Volume"), 0, SV_IREG_LEFT_LINE, SV_IREG_RIGHT_LINE, 0, 0, 31, 1),
    SONICVIBES_SINGLE(cstr!("Mic Playback Switch"), 0, SV_IREG_MIC, 7, 1, 1),
    SONICVIBES_SINGLE(cstr!("Mic Playback Volume"), 0, SV_IREG_MIC, 0, 15, 1),
    SONICVIBES_SINGLE(cstr!("Mic Boost"), 0, SV_IREG_LEFT_ADC, 4, 1, 0),
    SONICVIBES_DOUBLE(cstr!("Synth Playback Switch"), 0, SV_IREG_LEFT_SYNTH, SV_IREG_RIGHT_SYNTH, 7, 7, 1, 1),
    SONICVIBES_DOUBLE(cstr!("Synth Playback Volume"), 0, SV_IREG_LEFT_SYNTH, SV_IREG_RIGHT_SYNTH, 0, 0, 31, 1),
    SONICVIBES_DOUBLE(cstr!("Aux Playback Switch"), 1, SV_IREG_LEFT_AUX2, SV_IREG_RIGHT_AUX2, 7, 7, 1, 1),
    SONICVIBES_DOUBLE(cstr!("Aux Playback Volume"), 1, SV_IREG_LEFT_AUX2, SV_IREG_RIGHT_AUX2, 0, 0, 31, 1),
    SONICVIBES_DOUBLE(cstr!("Master Playback Switch"), 0, SV_IREG_LEFT_ANALOG, SV_IREG_RIGHT_ANALOG, 7, 7, 1, 1),
    SONICVIBES_DOUBLE(cstr!("Master Playback Volume"), 0, SV_IREG_LEFT_ANALOG, SV_IREG_RIGHT_ANALOG, 0, 0, 31, 1),
    SONICVIBES_DOUBLE(cstr!("PCM Playback Switch"), 0, SV_IREG_LEFT_PCM, SV_IREG_RIGHT_PCM, 7, 7, 1, 1),
    SONICVIBES_DOUBLE(cstr!("PCM Playback Volume"), 0, SV_IREG_LEFT_PCM, SV_IREG_RIGHT_PCM, 0, 0, 63, 1),
    SONICVIBES_SINGLE(cstr!("Loopback Capture Switch"), 0, SV_IREG_ADC_OUTPUT_CTRL, 0, 1, 0),
    SONICVIBES_SINGLE(cstr!("Loopback Capture Volume"), 0, SV_IREG_ADC_OUTPUT_CTRL, 2, 63, 1),
    /* SONICVIBES_MUX("Capture Source", 0) follows in C; represented separately below would make 21 items. */
];

unsafe extern "C" fn snd_sonicvibes_master_free(kcontrol: *mut snd_kcontrol) {
    let sonic = snd_kcontrol_chip(kcontrol);
    (*sonic).master_mute = core::ptr::null_mut();
    (*sonic).master_volume = core::ptr::null_mut();
}

unsafe fn snd_sonicvibes_mixer(sonic: *mut sonicvibes) -> c_int {
    let card: *mut snd_card;
    let mut kctl: *mut snd_kcontrol;
    let mut idx: c_uint;
    let mut err: c_int;

    if snd_BUG_ON(sonic.is_null() || (*sonic).card.is_null()) {
        return -EINVAL;
    }
    card = (*sonic).card;
    strscpy((*card).mixername, cstr!("S3 SonicVibes"));

    idx = 0;
    while (idx as usize) < snd_sonicvibes_controls.len() {
        kctl = snd_ctl_new1(&snd_sonicvibes_controls[idx as usize], sonic as *mut c_void);
        err = snd_ctl_add(card, kctl);
        if err < 0 {
            return err;
        }
        match idx {
            0 | 1 => {
                (*kctl).private_free = Some(snd_sonicvibes_master_free);
            }
            _ => {}
        }
        idx += 1;
    }
    kctl = snd_ctl_new1(&SONICVIBES_MUX(cstr!("Capture Source"), 0), sonic as *mut c_void);
    err = snd_ctl_add(card, kctl);
    if err < 0 {
        return err;
    }
    0
}

unsafe extern "C" fn snd_sonicvibes_proc_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let sonic = (*entry).private_data as *mut sonicvibes;
    let mut tmp: u8;

    tmp = (*sonic).srs_space & 0x0f;
    snd_iprintf(buffer, cstr!("SRS 3D           : %s\n"), str_off_on(((*sonic).srs_space & 0x80) as c_uint));
    snd_iprintf(buffer, cstr!("SRS Space        : %s\n"),
        if tmp == 0x00 { cstr!("100%") } else if tmp == 0x01 { cstr!("75%") } else if tmp == 0x02 { cstr!("50%") } else if tmp == 0x03 { cstr!("25%") } else { cstr!("0%") });
    tmp = (*sonic).srs_center & 0x0f;
    snd_iprintf(buffer, cstr!("SRS Center       : %s\n"),
        if tmp == 0x00 { cstr!("100%") } else if tmp == 0x01 { cstr!("75%") } else if tmp == 0x02 { cstr!("50%") } else if tmp == 0x03 { cstr!("25%") } else { cstr!("0%") });
    tmp = (*sonic).wave_source & 0x03;
    snd_iprintf(buffer, cstr!("WaveTable Source : %s\n"),
        if tmp == 0x00 { cstr!("on-board ROM") } else if tmp == 0x01 { cstr!("PCI bus") } else { cstr!("on-board ROM + PCI bus") });
    tmp = (*sonic).mpu_switch;
    snd_iprintf(buffer, cstr!("Onboard synth    : %s\n"), str_on_off((tmp & 0x01) as c_uint));
    snd_iprintf(buffer, cstr!("Ext. Rx to synth : %s\n"), str_on_off((tmp & 0x02) as c_uint));
    snd_iprintf(buffer, cstr!("MIDI to ext. Tx  : %s\n"), str_on_off((tmp & 0x04) as c_uint));
}

unsafe fn snd_sonicvibes_proc_init(sonic: *mut sonicvibes) {
    snd_card_ro_proc_new((*sonic).card, cstr!("sonicvibes"), sonic as *mut c_void, snd_sonicvibes_proc_read);
}

/* #ifdef SUPPORT_JOYSTICK */
static snd_sonicvibes_game_control: snd_kcontrol_new =
    SONICVIBES_SINGLE(cstr!("Joystick Speed"), 0, SV_IREG_GAME_PORT, 1, 15, 0);

unsafe fn snd_sonicvibes_create_gameport(sonic: *mut sonicvibes) -> c_int {
    let gp: *mut gameport;
    let err: c_int;

    gp = gameport_allocate_port();
    (*sonic).gameport = gp;
    if gp.is_null() {
        dev_err((*(*sonic).card).dev, cstr!("sonicvibes: cannot allocate memory for gameport\n"));
        return -ENOMEM;
    }

    gameport_set_name(gp, cstr!("SonicVibes Gameport"));
    gameport_set_phys(gp, cstr!("pci%s/gameport0"), pci_name((*sonic).pci));
    gameport_set_dev_parent(gp, &mut (*(*sonic).pci).dev);
    (*gp).io = (*sonic).game_port;

    gameport_register_port(gp);

    err = snd_ctl_add((*sonic).card, snd_ctl_new1(&snd_sonicvibes_game_control, sonic as *mut c_void));
    if err < 0 {
        return err;
    }

    0
}

unsafe fn snd_sonicvibes_free_gameport(sonic: *mut sonicvibes) {
    if !(*sonic).gameport.is_null() {
        gameport_unregister_port((*sonic).gameport);
        (*sonic).gameport = core::ptr::null_mut();
    }
}
/* #else in C: create_gameport returns -ENOSYS and free_gameport is empty. */

unsafe extern "C" fn snd_sonicvibes_free(card: *mut snd_card) {
    let sonic = (*card).private_data;

    snd_sonicvibes_free_gameport(sonic);
    pci_write_config_dword((*sonic).pci, 0x40, (*sonic).dmaa_port);
    pci_write_config_dword((*sonic).pci, 0x48, (*sonic).dmac_port);
}

unsafe fn snd_sonicvibes_create(card: *mut snd_card, pci: *mut pci_dev, reverb_arg: c_int, mge_arg: c_int) -> c_int {
    let sonic = (*card).private_data;
    let mut dmaa: c_uint = 0;
    let mut dmac: c_uint = 0;
    let mut err: c_int;

    /* enable PCI device */
    err = pcim_enable_device(pci);
    if err < 0 {
        return err;
    }
    /* check, if we can restrict PCI DMA transfers to 24 bits */
    if dma_set_mask_and_coherent(&mut (*pci).dev, DMA_BIT_MASK_24) != 0 {
        dev_err((*card).dev, cstr!("architecture does not support 24bit PCI busmaster DMA\n"));
        return -ENXIO;
    }

    spin_lock_init(&mut (*sonic).reg_lock);
    (*sonic).card = card;
    (*sonic).pci = pci;
    (*sonic).irq = -1;

    err = pcim_request_all_regions(pci, cstr!("S3 SonicVibes"));
    if err < 0 {
        return err;
    }

    (*sonic).sb_port = pci_resource_start(pci, 0);
    (*sonic).enh_port = pci_resource_start(pci, 1);
    (*sonic).synth_port = pci_resource_start(pci, 2);
    (*sonic).midi_port = pci_resource_start(pci, 3);
    (*sonic).game_port = pci_resource_start(pci, 4);

    if devm_request_irq(&mut (*pci).dev, (*pci).irq, snd_sonicvibes_interrupt, IRQF_SHARED, cstr!("KBUILD_MODNAME"), sonic as *mut c_void) != 0 {
        dev_err((*card).dev, cstr!("unable to grab IRQ %d\n"), (*pci).irq);
        return -EBUSY;
    }
    (*sonic).irq = (*pci).irq;
    (*card).sync_irq = (*sonic).irq;
    (*card).private_free = Some(snd_sonicvibes_free);

    pci_read_config_dword(pci, 0x40, &mut dmaa);
    pci_read_config_dword(pci, 0x48, &mut dmac);
    dmaio &= !0x0f;
    dmaa &= !0x0f;
    dmac &= !0x0f;
    if dmaa == 0 {
        dmaa = dmaio;
        dmaio = dmaio.wrapping_add(0x10);
        dev_info((*card).dev, cstr!("BIOS did not allocate DDMA channel A i/o, allocated at 0x%x\n"), dmaa);
    }
    if dmac == 0 {
        dmac = dmaio;
        dmaio = dmaio.wrapping_add(0x10);
        dev_info((*card).dev, cstr!("BIOS did not allocate DDMA channel C i/o, allocated at 0x%x\n"), dmac);
    }
    pci_write_config_dword(pci, 0x40, dmaa);
    pci_write_config_dword(pci, 0x48, dmac);

    (*sonic).res_dmaa = devm_request_region(&mut (*pci).dev, dmaa, 0x10, cstr!("S3 SonicVibes DDMA-A"));
    if (*sonic).res_dmaa.is_null() {
        dev_err((*card).dev, cstr!("unable to grab DDMA-A port at 0x%x-0x%x\n"), dmaa, dmaa + 0x10 - 1);
        return -EBUSY;
    }
    (*sonic).res_dmac = devm_request_region(&mut (*pci).dev, dmac, 0x10, cstr!("S3 SonicVibes DDMA-C"));
    if (*sonic).res_dmac.is_null() {
        dev_err((*card).dev, cstr!("unable to grab DDMA-C port at 0x%x-0x%x\n"), dmac, dmac + 0x10 - 1);
        return -EBUSY;
    }

    pci_read_config_dword(pci, 0x40, &mut (*sonic).dmaa_port);
    pci_read_config_dword(pci, 0x48, &mut (*sonic).dmac_port);
    (*sonic).dmaa_port &= !0x0f;
    (*sonic).dmac_port &= !0x0f;
    pci_write_config_dword(pci, 0x40, (*sonic).dmaa_port | 9); /* enable + enhanced */
    pci_write_config_dword(pci, 0x48, (*sonic).dmac_port | 9); /* enable */
    /* ok.. initialize S3 SonicVibes chip */
    outb(SV_RESET, SV_REG(sonic, SV_REG_CONTROL)); /* reset chip */
    udelay(100);
    outb(0, SV_REG(sonic, SV_REG_CONTROL)); /* release reset */
    udelay(100);
    outb(SV_ENHANCED | SV_INTA | if reverb_arg != 0 { SV_REVERB } else { 0 }, SV_REG(sonic, SV_REG_CONTROL));
    inb(SV_REG(sonic, SV_REG_STATUS)); /* clear IRQs */
    snd_sonicvibes_out(sonic, SV_IREG_DRIVE_CTRL as u8, 0); /* drive current 16mA */
    (*sonic).enable = 0;
    snd_sonicvibes_out(sonic, SV_IREG_PC_ENABLE as u8, (*sonic).enable); /* disable playback & capture */
    (*sonic).irqmask = (!(SV_DMAA_MASK | SV_DMAC_MASK | SV_UD_MASK)) as u8;
    outb((*sonic).irqmask as c_uint, SV_REG(sonic, SV_REG_IRQMASK));
    inb(SV_REG(sonic, SV_REG_STATUS)); /* clear IRQs */
    snd_sonicvibes_out(sonic, SV_IREG_ADC_CLOCK as u8, 0); /* use PLL as clock source */
    snd_sonicvibes_out(sonic, SV_IREG_ANALOG_POWER as u8, 0); /* power up analog parts */
    snd_sonicvibes_out(sonic, SV_IREG_DIGITAL_POWER as u8, 0); /* power up digital parts */
    snd_sonicvibes_setpll(sonic, SV_IREG_ADC_PLL as u8, 8000);
    (*sonic).srs_space = 0x80;
    snd_sonicvibes_out(sonic, SV_IREG_SRS_SPACE as u8, (*sonic).srs_space); /* SRS space off */
    (*sonic).srs_center = 0x00;
    snd_sonicvibes_out(sonic, SV_IREG_SRS_CENTER as u8, (*sonic).srs_center); /* SRS center off */
    (*sonic).mpu_switch = 0x05;
    snd_sonicvibes_out(sonic, SV_IREG_MPU401 as u8, (*sonic).mpu_switch); /* MPU-401 switch */
    (*sonic).wave_source = 0x00;
    snd_sonicvibes_out(sonic, SV_IREG_WAVE_SOURCE as u8, (*sonic).wave_source); /* onboard ROM */
    snd_sonicvibes_out(sonic, SV_IREG_PCM_RATE_LOW as u8, ((8000 * 65536 / SV_FULLRATE) & 0xff) as u8);
    snd_sonicvibes_out(sonic, SV_IREG_PCM_RATE_HIGH as u8, (((8000 * 65536 / SV_FULLRATE) >> 8) & 0xff) as u8);
    snd_sonicvibes_out(sonic, SV_IREG_LEFT_ADC as u8, if mge_arg != 0 { 0xd0 } else { 0xc0 });
    snd_sonicvibes_out(sonic, SV_IREG_RIGHT_ADC as u8, 0xc0);
    snd_sonicvibes_out(sonic, SV_IREG_LEFT_AUX1 as u8, 0x9f);
    snd_sonicvibes_out(sonic, SV_IREG_RIGHT_AUX1 as u8, 0x9f);
    snd_sonicvibes_out(sonic, SV_IREG_LEFT_CD as u8, 0x9f);
    snd_sonicvibes_out(sonic, SV_IREG_RIGHT_CD as u8, 0x9f);
    snd_sonicvibes_out(sonic, SV_IREG_LEFT_LINE as u8, 0x9f);
    snd_sonicvibes_out(sonic, SV_IREG_RIGHT_LINE as u8, 0x9f);
    snd_sonicvibes_out(sonic, SV_IREG_MIC as u8, 0x8f);
    snd_sonicvibes_out(sonic, SV_IREG_LEFT_SYNTH as u8, 0x9f);
    snd_sonicvibes_out(sonic, SV_IREG_RIGHT_SYNTH as u8, 0x9f);
    snd_sonicvibes_out(sonic, SV_IREG_LEFT_AUX2 as u8, 0x9f);
    snd_sonicvibes_out(sonic, SV_IREG_RIGHT_AUX2 as u8, 0x9f);
    snd_sonicvibes_out(sonic, SV_IREG_LEFT_ANALOG as u8, 0x9f);
    snd_sonicvibes_out(sonic, SV_IREG_RIGHT_ANALOG as u8, 0x9f);
    snd_sonicvibes_out(sonic, SV_IREG_LEFT_PCM as u8, 0xbf);
    snd_sonicvibes_out(sonic, SV_IREG_RIGHT_PCM as u8, 0xbf);
    snd_sonicvibes_out(sonic, SV_IREG_ADC_OUTPUT_CTRL as u8, 0xfc);
    /* #if 0: snd_sonicvibes_debug(sonic); */
    (*sonic).revision = snd_sonicvibes_in(sonic, SV_IREG_REVISION as u8);

    snd_sonicvibes_proc_init(sonic);
    0
}

/*
 *  MIDI section
 */

static snd_sonicvibes_midi_controls: [snd_kcontrol_new; 5] = [
    SONICVIBES_SINGLE(cstr!("SonicVibes Wave Source RAM"), 0, SV_IREG_WAVE_SOURCE, 0, 1, 0),
    SONICVIBES_SINGLE(cstr!("SonicVibes Wave Source RAM+ROM"), 0, SV_IREG_WAVE_SOURCE, 1, 1, 0),
    SONICVIBES_SINGLE(cstr!("SonicVibes Onboard Synth"), 0, SV_IREG_MPU401, 0, 1, 0),
    SONICVIBES_SINGLE(cstr!("SonicVibes External Rx to Synth"), 0, SV_IREG_MPU401, 1, 1, 0),
    SONICVIBES_SINGLE(cstr!("SonicVibes External Tx"), 0, SV_IREG_MPU401, 2, 1, 0),
];

unsafe extern "C" fn snd_sonicvibes_midi_input_open(mpu: *mut snd_mpu401) -> c_int {
    let sonic = (*mpu).private_data;
    (*sonic).irqmask &= !(SV_MIDI_MASK as u8);
    outb((*sonic).irqmask as c_uint, SV_REG(sonic, SV_REG_IRQMASK));
    0
}

unsafe extern "C" fn snd_sonicvibes_midi_input_close(mpu: *mut snd_mpu401) {
    let sonic = (*mpu).private_data;
    (*sonic).irqmask |= SV_MIDI_MASK as u8;
    outb((*sonic).irqmask as c_uint, SV_REG(sonic, SV_REG_IRQMASK));
}

unsafe fn snd_sonicvibes_midi(sonic: *mut sonicvibes, rmidi: *mut snd_rawmidi) -> c_int {
    let mpu = (*rmidi).private_data;
    let card = (*sonic).card;
    let mut idx: c_uint;
    let mut err: c_int;

    (*mpu).private_data = sonic;
    (*mpu).open_input = Some(snd_sonicvibes_midi_input_open);
    (*mpu).close_input = Some(snd_sonicvibes_midi_input_close);
    idx = 0;
    while (idx as usize) < snd_sonicvibes_midi_controls.len() {
        err = snd_ctl_add(card, snd_ctl_new1(&snd_sonicvibes_midi_controls[idx as usize], sonic as *mut c_void));
        if err < 0 {
            return err;
        }
        idx += 1;
    }
    0
}

unsafe fn __snd_sonic_probe(pci: *mut pci_dev, _pci_id: *const pci_device_id) -> c_int {
    static mut dev: c_int = 0;
    let mut card: *mut snd_card = core::ptr::null_mut();
    let sonic: *mut sonicvibes;
    let mut midi_uart: *mut snd_rawmidi = core::ptr::null_mut();
    let mut opl3: *mut snd_opl3 = core::ptr::null_mut();
    let mut err: c_int;

    if dev >= SNDRV_CARDS as c_int {
        return -ENODEV;
    }
    if !enable[dev as usize] {
        dev += 1;
        return -ENOENT;
    }

    err = snd_devm_card_new(&mut (*pci).dev, index[dev as usize], id[dev as usize], core::ptr::null_mut(), core::mem::size_of::<sonicvibes>(), &mut card);
    if err < 0 {
        return err;
    }
    sonic = (*card).private_data;
    err = snd_sonicvibes_create(card, pci, if reverb[dev as usize] { 1 } else { 0 }, if mge[dev as usize] { 1 } else { 0 });
    if err < 0 {
        return err;
    }

    strscpy((*card).driver, cstr!("SonicVibes"));
    strscpy((*card).shortname, cstr!("S3 SonicVibes"));
    sprintf((*card).longname, cstr!("%s rev %i at 0x%llx, irq %i"),
        (*card).shortname,
        (*sonic).revision as c_int,
        pci_resource_start(pci, 1) as u64,
        (*sonic).irq);

    err = snd_sonicvibes_pcm(sonic, 0);
    if err < 0 {
        return err;
    }
    err = snd_sonicvibes_mixer(sonic);
    if err < 0 {
        return err;
    }
    err = snd_mpu401_uart_new(card, 0, MPU401_HW_SONICVIBES as c_uint,
        (*sonic).midi_port,
        MPU401_INFO_INTEGRATED | MPU401_INFO_IRQ_HOOK,
        -1, &mut midi_uart);
    if err < 0 {
        return err;
    }
    snd_sonicvibes_midi(sonic, midi_uart);
    err = snd_opl3_create(card, (*sonic).synth_port,
        (*sonic).synth_port + 2,
        OPL3_HW_OPL3_SV as c_uint, 1, &mut opl3);
    if err < 0 {
        return err;
    }
    err = snd_opl3_hwdep_new(opl3, 0, 1, core::ptr::null_mut());
    if err < 0 {
        return err;
    }

    err = snd_sonicvibes_create_gameport(sonic);
    if err < 0 {
        return err;
    }

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }

    pci_set_drvdata(pci, card as *mut c_void);
    dev += 1;
    0
}

unsafe extern "C" fn snd_sonic_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    snd_card_free_on_error(&mut (*pci).dev, __snd_sonic_probe(pci, pci_id))
}

static mut sonicvibes_driver: pci_driver = pci_driver {
    name: cstr!("KBUILD_MODNAME"),
    id_table: snd_sonic_ids.as_ptr(),
    probe: Some(snd_sonic_probe),
};

/* module_pci_driver(sonicvibes_driver); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
