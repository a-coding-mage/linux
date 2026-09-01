// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * bt87x.c - Brooktree Bt878/Bt879 driver for ALSA
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 *
 * based on btaudio.c by Gerd Knorr <kraxel@bytesex.org>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type u32 = u32;
type irqreturn_t = c_uint;
type snd_pcm_uframes_t = c_ulong;
type dma_addr_t = c_ulong;
type spinlock_t = c_ulong;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
    pub rate: c_uint,
    pub format: c_int,
}

#[repr(C)]
pub struct snd_pcm {
    pub private_data: *mut c_void,
    pub name: [c_char; 80],
    pub device: c_int,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dma_buffer {
    pub area: *mut c_void,
    pub addr: dma_addr_t,
}

#[repr(C)]
pub struct snd_card {
    pub private_data: *mut c_void,
    pub dev: *mut device,
    pub sync_irq: c_int,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
    pub mixername: [c_char; 80],
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
    pub irq: c_int,
    pub device: c_uint,
    pub subsystem_vendor: c_uint,
    pub subsystem_device: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_info_value_integer {
    pub min: c_long,
    pub max: c_long,
}

type c_long = isize;

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_value_integer,
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
}

#[repr(C)]
pub struct snd_ratnum {
    pub num: c_uint,
    pub den_min: c_uint,
    pub den_max: c_uint,
    pub den_step: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_ratnums {
    pub nrats: c_uint,
    pub rats: *const snd_ratnum,
}

#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_device_id {
    pub vendor: c_uint,
    pub device: c_uint,
    pub subvendor: c_uint,
    pub subdevice: c_uint,
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
}

unsafe extern "C" {
    static THIS_MODULE: *mut module;
    static KBUILD_MODNAME: *const c_char;

    fn readl(addr: *mut c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn snd_dma_alloc_pages(t: c_int, dev: *mut device, size: c_ulong, dmab: *mut snd_dma_buffer) -> c_int;
    fn snd_dma_free_pages(dmab: *mut snd_dma_buffer);
    fn pci_status_get_and_clear_errors(pci: *mut pci_dev) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn snd_pcm_sgbuf_get_addr(substream: *mut snd_pcm_substream, offset: c_uint) -> c_uint;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_rate_to_rate_bit(rate: c_uint) -> c_uint;
    fn snd_pcm_hw_constraint_ratnums(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_uint, r: *const snd_pcm_hw_constraint_ratnums) -> c_int;
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_uint) -> c_int;
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut snd_bt87x;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_bt87x;
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> c_int;
    fn snd_ctl_enum_info(info: *mut snd_ctl_elem_info, channels: c_uint, items: c_uint, texts: *const *const c_char) -> c_int;
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn pcim_iomap_region(pci: *mut pci_dev, bar: c_int, name: *const c_char) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, data: *mut c_void) -> c_int;
    fn pci_set_master(pci: *mut pci_dev);
    fn snd_pcm_new(card: *mut snd_card, id: *mut c_char, device: c_int, playback: c_int, capture: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, t: c_int, dev: *mut device, min: c_ulong, max: c_ulong);
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(n: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn pci_match_id(ids: *const pci_device_id, pci: *mut pci_dev) -> *const pci_device_id;
    fn snd_devm_card_new(dev: *mut device, idx: c_int, id: *mut c_char, module: *mut module, extra_size: usize, card_ret: *mut *mut snd_card) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn snd_card_free_on_error(dev: *mut device, ret: c_int) -> c_int;
    fn pci_register_driver(driver: *mut pci_driver) -> c_int;
    fn pci_unregister_driver(driver: *mut pci_driver);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn test_and_set_bit(nr: c_int, addr: *mut c_ulong) -> c_int;
    fn clear_bit(nr: c_int, addr: *mut c_ulong);
    fn smp_mb__after_atomic();
    fn params_periods(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_period_bytes(params: *mut snd_pcm_hw_params) -> c_uint;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: c_uint) -> snd_pcm_uframes_t;
}

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_ENABLE_PNP: bool = true;

static mut index: [c_int; SNDRV_CARDS] = [-2; SNDRV_CARDS]; /* Exclude the first card */
static mut id: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS]; /* ID for this card */
static mut enable: [bool; SNDRV_CARDS] = [SNDRV_DEFAULT_ENABLE_PNP; SNDRV_CARDS]; /* Enable this card */
static mut digital_rate: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS]; /* digital input rate */
static mut load_all: bool = false; /* allow to load cards not the allowlist */

/* module parameters: index, id, enable, digital_rate, load_all */

/* register offsets */
const REG_INT_STAT: u32 = 0x100; /* interrupt status */
const REG_INT_MASK: u32 = 0x104; /* interrupt mask */
const REG_GPIO_DMA_CTL: u32 = 0x10c; /* audio control */
const REG_PACKET_LEN: u32 = 0x110; /* audio packet lengths */
const REG_RISC_STRT_ADD: u32 = 0x114; /* RISC program start address */
const REG_RISC_COUNT: u32 = 0x120; /* RISC program counter */

/* interrupt bits */
const INT_OFLOW: u32 = 1 << 3; /* audio A/D overflow */
const INT_RISCI: u32 = 1 << 11; /* RISC instruction IRQ bit set */
const INT_FBUS: u32 = 1 << 12; /* FIFO overrun due to bus access latency */
const INT_FTRGT: u32 = 1 << 13; /* FIFO overrun due to target latency */
const INT_FDSR: u32 = 1 << 14; /* FIFO data stream resynchronization */
const INT_PPERR: u32 = 1 << 15; /* PCI parity error */
const INT_RIPERR: u32 = 1 << 16; /* RISC instruction parity error */
const INT_PABORT: u32 = 1 << 17; /* PCI master or target abort */
const INT_OCERR: u32 = 1 << 18; /* invalid opcode */
const INT_SCERR: u32 = 1 << 19; /* sync counter overflow */
const INT_RISC_EN: u32 = 1 << 27; /* DMA controller running */
const INT_RISCS_SHIFT: u32 = 28; /* RISC status bits */

/* audio control bits */
const CTL_FIFO_ENABLE: u32 = 1 << 0; /* enable audio data FIFO */
const CTL_RISC_ENABLE: u32 = 1 << 1; /* enable audio DMA controller */
const CTL_PKTP_4: u32 = 0 << 2; /* packet mode FIFO trigger point - 4 DWORDs */
const CTL_PKTP_8: u32 = 1 << 2; /* 8 DWORDs */
const CTL_PKTP_16: u32 = 2 << 2; /* 16 DWORDs */
const CTL_ACAP_EN: u32 = 1 << 4; /* enable audio capture */
const CTL_DA_APP: u32 = 1 << 5; /* GPIO input */
const CTL_DA_IOM_AFE: u32 = 0 << 6; /* audio A/D input */
const CTL_DA_IOM_DA: u32 = 1 << 6; /* digital audio input */
const CTL_DA_SDR_SHIFT: u32 = 8; /* DDF first stage decimation rate */
const CTL_DA_SDR_MASK: u32 = 0xf << 8;
const CTL_DA_LMT: u32 = 1 << 12; /* limit audio data values */
const CTL_DA_ES2: u32 = 1 << 13; /* enable DDF stage 2 */
const CTL_DA_SBR: u32 = 1 << 14; /* samples rounded to 8 bits */
const CTL_DA_DPM: u32 = 1 << 15; /* data packet mode */
const CTL_DA_LRD_SHIFT: u32 = 16; /* ALRCK delay */
const CTL_DA_MLB: u32 = 1 << 21; /* MSB/LSB format */
const CTL_DA_LRI: u32 = 1 << 22; /* left/right indication */
const CTL_DA_SCE: u32 = 1 << 23; /* sample clock edge */
const CTL_A_SEL_STV: u32 = 0 << 24; /* TV tuner audio input */
const CTL_A_SEL_SFM: u32 = 1 << 24; /* FM audio input */
const CTL_A_SEL_SML: u32 = 2 << 24; /* mic/line audio input */
const CTL_A_SEL_SMXC: u32 = 3 << 24; /* MUX bypass */
const CTL_A_SEL_SHIFT: u32 = 24;
const CTL_A_SEL_MASK: u32 = 3 << 24;
const CTL_A_PWRDN: u32 = 1 << 26; /* analog audio power-down */
const CTL_A_G2X: u32 = 1 << 27; /* audio gain boost */
const CTL_A_GAIN_SHIFT: u32 = 28; /* audio input gain */
const CTL_A_GAIN_MASK: u32 = 0xf << 28;

/* RISC instruction opcodes */
const RISC_WRITE: u32 = 0x1 << 28; /* write FIFO data to memory at address */
const RISC_WRITEC: u32 = 0x5 << 28; /* write FIFO data to memory at current address */
const RISC_SKIP: u32 = 0x2 << 28; /* skip FIFO data */
const RISC_JUMP: u32 = 0x7 << 28; /* jump to address */
const RISC_SYNC: u32 = 0x8 << 28; /* synchronize with FIFO */

/* RISC instruction bits */
const RISC_BYTES_ENABLE: u32 = 0xf << 12; /* byte enable bits */
const RISC_RESYNC: u32 = 1 << 15; /* disable FDSR errors */
const RISC_SET_STATUS_SHIFT: u32 = 16; /* set status bits */
const RISC_RESET_STATUS_SHIFT: u32 = 20; /* clear status bits */
const RISC_IRQ: u32 = 1 << 24; /* interrupt */
const RISC_EOL: u32 = 1 << 26; /* end of line */
const RISC_SOL: u32 = 1 << 27; /* start of line */

/* SYNC status bits values */
const RISC_SYNC_FM1: u32 = 0x6;
const RISC_SYNC_VRO: u32 = 0xc;

const ANALOG_CLOCK: c_uint = 1792000;
/* CONFIG_SND_BT87X_OVERCLOCK selects CLOCK_DIV_MIN = 1; otherwise it is 4. */
const CLOCK_DIV_MIN: c_uint = 4;
const CLOCK_DIV_MAX: c_uint = 15;

const ERROR_INTERRUPTS: u32 = INT_FBUS | INT_FTRGT | INT_PPERR | INT_RIPERR | INT_PABORT | INT_OCERR;
const MY_INTERRUPTS: u32 = INT_RISCI | ERROR_INTERRUPTS;

const PAGE_SIZE: c_uint = 4096;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const PCI_STATUS_DETECTED_PARITY: c_int = 0x0100;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_SHARED: c_ulong = 0x80;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SNDRV_DMA_TYPE_DEV_SG: c_int = 1;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_FORMAT_S8: c_int = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_uint = 0;
const SNDRV_PCM_HW_PARAM_PERIODS: c_uint = 1;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 1;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint = 1 << 2;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 3;
const SNDRV_PCM_INFO_BATCH: c_uint = 1 << 4;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << 2;
const SNDRV_PCM_FMTBIT_S8: c_ulong = 1 << 0;
const SNDRV_PCM_RATE_KNOT: c_uint = 1 << 31;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 1;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const PCI_VENDOR_ID_BROOKTREE: c_uint = 0x109e;
const PCI_DEVICE_ID_BROOKTREE_878: c_uint = 0x0878;
const PCI_DEVICE_ID_BROOKTREE_879: c_uint = 0x0879;
const PCI_ANY_ID: c_uint = !0;

const fn PAGE_ALIGN(x: c_uint) -> c_uint {
    (x + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)
}

const fn ALIGN(x: c_ulong, a: c_ulong) -> c_ulong {
    (x + a - 1) & !(a - 1)
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

fn DIV_ROUND_UP(n: c_uint, d: c_uint) -> c_uint {
    (n + d - 1) / d
}

fn cpu_to_le32(v: u32) -> u32 {
    v.to_le()
}

/* SYNC, one WRITE per line, one extra WRITE per page boundary, SYNC, JUMP */
const MAX_RISC_SIZE: c_uint = (1 + 255 + (PAGE_ALIGN(255 * 4092) / PAGE_SIZE - 1) + 1 + 1) * 8;

/* Cards with configuration information */
#[repr(C)]
#[derive(Copy, Clone)]
enum snd_bt87x_boardid {
    SND_BT87X_BOARD_UNKNOWN,
    SND_BT87X_BOARD_GENERIC, /* both an & dig interfaces, 32kHz */
    SND_BT87X_BOARD_ANALOG,  /* board with no external A/D */
    SND_BT87X_BOARD_OSPREY2x0,
    SND_BT87X_BOARD_OSPREY440,
    SND_BT87X_BOARD_AVPHONE98,
}

/* Card configuration */
#[repr(C)]
#[derive(Copy, Clone)]
struct snd_bt87x_board {
    dig_rate: c_int,      /* Digital input sampling rate */
    digital_fmt: u32,    /* Register settings for digital input */
    no_analog: c_uint,   /* No analog input */
    no_digital: c_uint,  /* No digital input */
}

static snd_bt87x_boards: [snd_bt87x_board; 6] = [
    snd_bt87x_board { dig_rate: 32000, digital_fmt: 0, no_analog: 0, no_digital: 0 }, /* just a guess */
    snd_bt87x_board { dig_rate: 32000, digital_fmt: 0, no_analog: 0, no_digital: 0 },
    snd_bt87x_board { dig_rate: 0, digital_fmt: 0, no_analog: 0, no_digital: 1 },
    snd_bt87x_board { dig_rate: 44100, digital_fmt: CTL_DA_LRI | (1 << CTL_DA_LRD_SHIFT), no_analog: 0, no_digital: 0 },
    snd_bt87x_board { dig_rate: 32000, digital_fmt: CTL_DA_LRI | (1 << CTL_DA_LRD_SHIFT), no_analog: 1, no_digital: 0 },
    snd_bt87x_board { dig_rate: 48000, digital_fmt: 0, no_analog: 0, no_digital: 0 },
];

#[repr(C)]
struct snd_bt87x {
    card: *mut snd_card,
    pci: *mut pci_dev,
    board: snd_bt87x_board,
    mmio: *mut c_void,
    irq: c_int,
    reg_lock: spinlock_t,
    opened: c_ulong,
    substream: *mut snd_pcm_substream,
    dma_risc: snd_dma_buffer,
    line_bytes: c_uint,
    lines: c_uint,
    reg_control: u32,
    interrupt_mask: u32,
    current_line: c_int,
    pci_parity_errors: c_int,
}

const DEVICE_DIGITAL: c_int = 0;
const DEVICE_ANALOG: c_int = 1;

unsafe fn snd_bt87x_readl(chip: *mut snd_bt87x, reg: u32) -> u32 {
    readl(((*chip).mmio as *mut u8).add(reg as usize) as *mut c_void)
}

unsafe fn snd_bt87x_writel(chip: *mut snd_bt87x, reg: u32, value: u32) {
    writel(value, ((*chip).mmio as *mut u8).add(reg as usize) as *mut c_void);
}

unsafe extern "C" fn snd_bt87x_create_risc(chip: *mut snd_bt87x, substream: *mut snd_pcm_substream, periods: c_uint, period_bytes: c_uint) -> c_int {
    let mut i: c_uint;
    let mut offset: c_uint;
    let mut risc: *mut u32;

    if (*chip).dma_risc.area.is_null() {
        if snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, &mut (*(*chip).pci).dev, PAGE_ALIGN(MAX_RISC_SIZE) as c_ulong, &mut (*chip).dma_risc) < 0 {
            return -ENOMEM;
        }
    }
    risc = (*chip).dma_risc.area as *mut u32;
    offset = 0;
    *risc = cpu_to_le32(RISC_SYNC | RISC_SYNC_FM1);
    risc = risc.add(1);
    *risc = cpu_to_le32(0);
    risc = risc.add(1);
    i = 0;
    while i < periods {
        let mut rest: u32 = period_bytes;
        loop {
            let mut cmd: u32;
            let mut len: u32;
            let addr: c_uint;

            len = PAGE_SIZE - (offset % PAGE_SIZE);
            if len > rest {
                len = rest;
            }
            cmd = RISC_WRITE | len;
            if rest == period_bytes {
                let block: u32 = i * 16 / periods;
                cmd |= RISC_SOL;
                cmd |= block << RISC_SET_STATUS_SHIFT;
                cmd |= (!block & 0xf) << RISC_RESET_STATUS_SHIFT;
            }
            if len == rest {
                cmd |= RISC_EOL | RISC_IRQ;
            }
            *risc = cpu_to_le32(cmd);
            risc = risc.add(1);
            addr = snd_pcm_sgbuf_get_addr(substream, offset);
            *risc = cpu_to_le32(addr);
            risc = risc.add(1);
            offset = offset.wrapping_add(len);
            rest = rest.wrapping_sub(len);
            if rest == 0 {
                break;
            }
        }
        i += 1;
    }
    *risc = cpu_to_le32(RISC_SYNC | RISC_SYNC_VRO);
    risc = risc.add(1);
    *risc = cpu_to_le32(0);
    risc = risc.add(1);
    *risc = cpu_to_le32(RISC_JUMP);
    risc = risc.add(1);
    *risc = cpu_to_le32((*chip).dma_risc.addr as u32);
    (*chip).line_bytes = period_bytes;
    (*chip).lines = periods;
    0
}

unsafe extern "C" fn snd_bt87x_free_risc(chip: *mut snd_bt87x) {
    if !(*chip).dma_risc.area.is_null() {
        snd_dma_free_pages(&mut (*chip).dma_risc);
        (*chip).dma_risc.area = ptr::null_mut();
    }
}

unsafe extern "C" fn snd_bt87x_pci_error(chip: *mut snd_bt87x, status: c_uint) {
    let pci_status = pci_status_get_and_clear_errors((*chip).pci);

    if pci_status != PCI_STATUS_DETECTED_PARITY {
        dev_err((*(*chip).card).dev, c"Aieee - PCI error! status %#08x, PCI status %#04x\n".as_ptr(), status & ERROR_INTERRUPTS, pci_status);
    } else {
        dev_err((*(*chip).card).dev, c"Aieee - PCI parity error detected!\n".as_ptr());
        /* error 'handling' similar to aic7xxx_pci.c: */
        (*chip).pci_parity_errors += 1;
        if (*chip).pci_parity_errors > 20 {
            dev_err((*(*chip).card).dev, c"Too many PCI parity errors observed.\n".as_ptr());
            dev_err((*(*chip).card).dev, c"Some device on this bus is generating bad parity.\n".as_ptr());
            dev_err((*(*chip).card).dev, c"This is an error *observed by*, not *generated by*, this card.\n".as_ptr());
            dev_err((*(*chip).card).dev, c"PCI parity error checking has been disabled.\n".as_ptr());
            (*chip).interrupt_mask &= !(INT_PPERR | INT_RIPERR);
            snd_bt87x_writel(chip, REG_INT_MASK, (*chip).interrupt_mask);
        }
    }
}

unsafe extern "C" fn snd_bt87x_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chip: *mut snd_bt87x = dev_id as *mut snd_bt87x;
    let status: c_uint = snd_bt87x_readl(chip, REG_INT_STAT);
    let irq_status: c_uint = status & (*chip).interrupt_mask;
    if irq_status == 0 {
        return IRQ_NONE;
    }
    snd_bt87x_writel(chip, REG_INT_STAT, irq_status);

    if (irq_status & ERROR_INTERRUPTS) != 0 {
        if (irq_status & (INT_FBUS | INT_FTRGT)) != 0 {
            dev_warn((*(*chip).card).dev, c"FIFO overrun, status %#08x\n".as_ptr(), status);
        }
        if (irq_status & INT_OCERR) != 0 {
            dev_err((*(*chip).card).dev, c"internal RISC error, status %#08x\n".as_ptr(), status);
        }
        if (irq_status & (INT_PPERR | INT_RIPERR | INT_PABORT)) != 0 {
            snd_bt87x_pci_error(chip, irq_status);
        }
    }
    if (irq_status & INT_RISCI) != 0 && ((*chip).reg_control & CTL_ACAP_EN) != 0 {
        let current_block: c_int;
        let irq_block: c_int;

        /* assume that exactly one line has been recorded */
        (*chip).current_line = ((*chip).current_line + 1) % (*chip).lines as c_int;
        /* but check if some interrupts have been skipped */
        current_block = (*chip).current_line * 16 / (*chip).lines as c_int;
        irq_block = (status >> INT_RISCS_SHIFT) as c_int;
        if current_block != irq_block {
            (*chip).current_line = DIV_ROUND_UP((irq_block as c_uint) * (*chip).lines, 16) as c_int;
        }

        snd_pcm_period_elapsed((*chip).substream);
    }
    IRQ_HANDLED
}

static snd_bt87x_digital_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_BATCH,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    rates: 0, /* set at runtime */
    rate_min: 0,
    rate_max: 0,
    channels_min: 2,
    channels_max: 2,
    buffer_bytes_max: 255 * 4092,
    period_bytes_min: 32,
    period_bytes_max: 4092,
    periods_min: 2,
    periods_max: 255,
};

static snd_bt87x_analog_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_BATCH,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8,
    rates: SNDRV_PCM_RATE_KNOT,
    rate_min: ANALOG_CLOCK / CLOCK_DIV_MAX,
    rate_max: ANALOG_CLOCK / CLOCK_DIV_MIN,
    channels_min: 1,
    channels_max: 1,
    buffer_bytes_max: 255 * 4092,
    period_bytes_min: 32,
    period_bytes_max: 4092,
    periods_min: 2,
    periods_max: 255,
};

unsafe extern "C" fn snd_bt87x_set_digital_hw(chip: *mut snd_bt87x, runtime: *mut snd_pcm_runtime) -> c_int {
    (*chip).reg_control |= CTL_DA_IOM_DA | CTL_A_PWRDN;
    (*runtime).hw = snd_bt87x_digital_hw;
    (*runtime).hw.rates = snd_pcm_rate_to_rate_bit((*chip).board.dig_rate as c_uint);
    (*runtime).hw.rate_min = (*chip).board.dig_rate as c_uint;
    (*runtime).hw.rate_max = (*chip).board.dig_rate as c_uint;
    0
}

unsafe extern "C" fn snd_bt87x_set_analog_hw(chip: *mut snd_bt87x, runtime: *mut snd_pcm_runtime) -> c_int {
    static analog_clock: snd_ratnum = snd_ratnum {
        num: ANALOG_CLOCK,
        den_min: CLOCK_DIV_MIN,
        den_max: CLOCK_DIV_MAX,
        den_step: 1,
    };
    static constraint_rates: snd_pcm_hw_constraint_ratnums = snd_pcm_hw_constraint_ratnums {
        nrats: 1,
        rats: &analog_clock,
    };

    (*chip).reg_control &= !(CTL_DA_IOM_DA | CTL_A_PWRDN);
    (*runtime).hw = snd_bt87x_analog_hw;
    snd_pcm_hw_constraint_ratnums(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &constraint_rates)
}

unsafe extern "C" fn snd_bt87x_pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let mut err: c_int;

    if test_and_set_bit(0, &mut (*chip).opened) != 0 {
        return -EBUSY;
    }

    if (*(*substream).pcm).device == DEVICE_DIGITAL {
        err = snd_bt87x_set_digital_hw(chip, runtime);
    } else {
        err = snd_bt87x_set_analog_hw(chip, runtime);
    }
    if err < 0 {
        clear_bit(0, &mut (*chip).opened);
        smp_mb__after_atomic();
        return err;
    }

    err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if err < 0 {
        clear_bit(0, &mut (*chip).opened);
        smp_mb__after_atomic();
        return err;
    }

    (*chip).substream = substream;
    0
}

unsafe extern "C" fn snd_bt87x_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);

    spin_lock_irq(&mut (*chip).reg_lock);
    (*chip).reg_control |= CTL_A_PWRDN;
    snd_bt87x_writel(chip, REG_GPIO_DMA_CTL, (*chip).reg_control);
    spin_unlock_irq(&mut (*chip).reg_lock);

    (*chip).substream = ptr::null_mut();
    clear_bit(0, &mut (*chip).opened);
    smp_mb__after_atomic();
    0
}

unsafe extern "C" fn snd_bt87x_hw_params(substream: *mut snd_pcm_substream, hw_params: *mut snd_pcm_hw_params) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    snd_bt87x_create_risc(chip, substream, params_periods(hw_params), params_period_bytes(hw_params))
}

unsafe extern "C" fn snd_bt87x_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    snd_bt87x_free_risc(chip);
    0
}

unsafe extern "C" fn snd_bt87x_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let decimation: c_int;

    spin_lock_irq(&mut (*chip).reg_lock);
    (*chip).reg_control &= !(CTL_DA_SDR_MASK | CTL_DA_SBR);
    decimation = ((ANALOG_CLOCK + (*runtime).rate / 4) / (*runtime).rate) as c_int;
    (*chip).reg_control |= (decimation as u32) << CTL_DA_SDR_SHIFT;
    if (*runtime).format == SNDRV_PCM_FORMAT_S8 {
        (*chip).reg_control |= CTL_DA_SBR;
    }
    snd_bt87x_writel(chip, REG_GPIO_DMA_CTL, (*chip).reg_control);
    spin_unlock_irq(&mut (*chip).reg_lock);
    0
}

unsafe extern "C" fn snd_bt87x_start(chip: *mut snd_bt87x) -> c_int {
    spin_lock(&mut (*chip).reg_lock);
    (*chip).current_line = 0;
    (*chip).reg_control |= CTL_FIFO_ENABLE | CTL_RISC_ENABLE | CTL_ACAP_EN;
    snd_bt87x_writel(chip, REG_RISC_STRT_ADD, (*chip).dma_risc.addr as u32);
    snd_bt87x_writel(chip, REG_PACKET_LEN, (*chip).line_bytes | ((*chip).lines << 16));
    snd_bt87x_writel(chip, REG_INT_MASK, (*chip).interrupt_mask);
    snd_bt87x_writel(chip, REG_GPIO_DMA_CTL, (*chip).reg_control);
    spin_unlock(&mut (*chip).reg_lock);
    0
}

unsafe extern "C" fn snd_bt87x_stop(chip: *mut snd_bt87x) -> c_int {
    spin_lock(&mut (*chip).reg_lock);
    (*chip).reg_control &= !(CTL_FIFO_ENABLE | CTL_RISC_ENABLE | CTL_ACAP_EN);
    snd_bt87x_writel(chip, REG_GPIO_DMA_CTL, (*chip).reg_control);
    snd_bt87x_writel(chip, REG_INT_MASK, 0);
    snd_bt87x_writel(chip, REG_INT_STAT, MY_INTERRUPTS);
    spin_unlock(&mut (*chip).reg_lock);
    0
}

unsafe extern "C" fn snd_bt87x_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream);

    match cmd {
        SNDRV_PCM_TRIGGER_START => snd_bt87x_start(chip),
        SNDRV_PCM_TRIGGER_STOP => snd_bt87x_stop(chip),
        _ => -EINVAL,
    }
}

unsafe extern "C" fn snd_bt87x_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    bytes_to_frames(runtime, ((*chip).current_line as c_uint).wrapping_mul((*chip).line_bytes))
}

static snd_bt87x_pcm_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_bt87x_pcm_open),
    close: Some(snd_bt87x_close),
    hw_params: Some(snd_bt87x_hw_params),
    hw_free: Some(snd_bt87x_hw_free),
    prepare: Some(snd_bt87x_prepare),
    trigger: Some(snd_bt87x_trigger),
    pointer: Some(snd_bt87x_pointer),
};

unsafe extern "C" fn snd_bt87x_capture_volume_info(_kcontrol: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> c_int {
    (*info).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*info).count = 1;
    (*info).value.integer.min = 0;
    (*info).value.integer.max = 15;
    0
}

unsafe extern "C" fn snd_bt87x_capture_volume_get(kcontrol: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    (*value).value.integer.value[0] = (((*chip).reg_control & CTL_A_GAIN_MASK) >> CTL_A_GAIN_SHIFT) as c_long;
    0
}

unsafe extern "C" fn snd_bt87x_capture_volume_put(kcontrol: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let old_control: u32;
    let changed: c_int;

    spin_lock_irq(&mut (*chip).reg_lock);
    old_control = (*chip).reg_control;
    (*chip).reg_control = ((*chip).reg_control & !CTL_A_GAIN_MASK)
        | (((*value).value.integer.value[0] as u32) << CTL_A_GAIN_SHIFT);
    snd_bt87x_writel(chip, REG_GPIO_DMA_CTL, (*chip).reg_control);
    changed = (old_control != (*chip).reg_control) as c_int;
    spin_unlock_irq(&mut (*chip).reg_lock);
    changed
}

static snd_bt87x_capture_volume: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c"Capture Volume".as_ptr(),
    info: Some(snd_bt87x_capture_volume_info),
    get: Some(snd_bt87x_capture_volume_get),
    put: Some(snd_bt87x_capture_volume_put),
};

unsafe extern "C" fn snd_bt87x_capture_boost_info(kcontrol: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> c_int {
    snd_ctl_boolean_mono_info(kcontrol, info)
}

unsafe extern "C" fn snd_bt87x_capture_boost_get(kcontrol: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    (*value).value.integer.value[0] = (((*chip).reg_control & CTL_A_G2X) != 0) as c_long;
    0
}

unsafe extern "C" fn snd_bt87x_capture_boost_put(kcontrol: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let old_control: u32;
    let changed: c_int;

    spin_lock_irq(&mut (*chip).reg_lock);
    old_control = (*chip).reg_control;
    (*chip).reg_control = ((*chip).reg_control & !CTL_A_G2X)
        | if (*value).value.integer.value[0] != 0 { CTL_A_G2X } else { 0 };
    snd_bt87x_writel(chip, REG_GPIO_DMA_CTL, (*chip).reg_control);
    changed = ((*chip).reg_control != old_control) as c_int;
    spin_unlock_irq(&mut (*chip).reg_lock);
    changed
}

static snd_bt87x_capture_boost: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c"Capture Boost".as_ptr(),
    info: Some(snd_bt87x_capture_boost_info),
    get: Some(snd_bt87x_capture_boost_get),
    put: Some(snd_bt87x_capture_boost_put),
};

unsafe extern "C" fn snd_bt87x_capture_source_info(_kcontrol: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> c_int {
    static texts: [*const c_char; 3] = [
        c"TV Tuner".as_ptr(),
        c"FM".as_ptr(),
        c"Mic/Line".as_ptr(),
    ];

    snd_ctl_enum_info(info, 1, 3, texts.as_ptr())
}

unsafe extern "C" fn snd_bt87x_capture_source_get(kcontrol: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    (*value).value.enumerated.item[0] = ((*chip).reg_control & CTL_A_SEL_MASK) >> CTL_A_SEL_SHIFT;
    0
}

unsafe extern "C" fn snd_bt87x_capture_source_put(kcontrol: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let old_control: u32;
    let changed: c_int;

    spin_lock_irq(&mut (*chip).reg_lock);
    old_control = (*chip).reg_control;
    (*chip).reg_control = ((*chip).reg_control & !CTL_A_SEL_MASK)
        | ((*value).value.enumerated.item[0] << CTL_A_SEL_SHIFT);
    snd_bt87x_writel(chip, REG_GPIO_DMA_CTL, (*chip).reg_control);
    changed = ((*chip).reg_control != old_control) as c_int;
    spin_unlock_irq(&mut (*chip).reg_lock);
    changed
}

static snd_bt87x_capture_source: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c"Capture Source".as_ptr(),
    info: Some(snd_bt87x_capture_source_info),
    get: Some(snd_bt87x_capture_source_get),
    put: Some(snd_bt87x_capture_source_put),
};

unsafe extern "C" fn snd_bt87x_free(card: *mut snd_card) {
    let chip: *mut snd_bt87x = (*card).private_data as *mut snd_bt87x;
    snd_bt87x_stop(chip);
}

unsafe extern "C" fn snd_bt87x_pcm(chip: *mut snd_bt87x, device: c_int, name: *mut c_char) -> c_int {
    let mut err: c_int;
    let mut pcm: *mut snd_pcm = ptr::null_mut();

    err = snd_pcm_new((*chip).card, name, device, 0, 1, &mut pcm);
    if err < 0 {
        return err;
    }
    (*pcm).private_data = chip as *mut c_void;
    strscpy((*pcm).name.as_mut_ptr(), name);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_bt87x_pcm_ops);
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV_SG, &mut (*(*chip).pci).dev, 128 * 1024, ALIGN(255 * 4092, 1024));
    0
}

unsafe extern "C" fn snd_bt87x_create(card: *mut snd_card, pci: *mut pci_dev) -> c_int {
    let chip: *mut snd_bt87x = (*card).private_data as *mut snd_bt87x;
    let mut err: c_int;

    err = pcim_enable_device(pci);
    if err < 0 {
        return err;
    }

    (*chip).card = card;
    (*chip).pci = pci;
    (*chip).irq = -1;
    spin_lock_init(&mut (*chip).reg_lock);

    (*chip).mmio = pcim_iomap_region(pci, 0, c"Bt87x audio".as_ptr());
    if IS_ERR((*chip).mmio) {
        return PTR_ERR((*chip).mmio);
    }

    (*chip).reg_control = CTL_A_PWRDN | CTL_DA_ES2 | CTL_PKTP_16 | (15 << CTL_DA_SDR_SHIFT);
    (*chip).interrupt_mask = MY_INTERRUPTS;
    snd_bt87x_writel(chip, REG_GPIO_DMA_CTL, (*chip).reg_control);
    snd_bt87x_writel(chip, REG_INT_MASK, 0);
    snd_bt87x_writel(chip, REG_INT_STAT, MY_INTERRUPTS);

    err = devm_request_irq(&mut (*pci).dev, (*pci).irq, snd_bt87x_interrupt, IRQF_SHARED, KBUILD_MODNAME, chip as *mut c_void);
    if err < 0 {
        dev_err((*card).dev, c"cannot grab irq %d\n".as_ptr(), (*pci).irq);
        return err;
    }
    (*chip).irq = (*pci).irq;
    (*card).sync_irq = (*chip).irq;
    (*card).private_free = Some(snd_bt87x_free);
    pci_set_master(pci);

    0
}

const fn BT_DEVICE(chip: c_uint, subvend: c_uint, subdev: c_uint, id: snd_bt87x_boardid) -> pci_device_id {
    pci_device_id {
        vendor: PCI_VENDOR_ID_BROOKTREE,
        device: chip,
        subvendor: subvend,
        subdevice: subdev,
        driver_data: id as c_ulong,
    }
}
/* driver_data is the card id for that device */

static snd_bt87x_ids: [pci_device_id; 14] = [
    /* Hauppauge WinTV series */
    BT_DEVICE(PCI_DEVICE_ID_BROOKTREE_878, 0x0070, 0x13eb, snd_bt87x_boardid::SND_BT87X_BOARD_GENERIC),
    /* Hauppauge WinTV series */
    BT_DEVICE(PCI_DEVICE_ID_BROOKTREE_879, 0x0070, 0x13eb, snd_bt87x_boardid::SND_BT87X_BOARD_GENERIC),
    /* Viewcast Osprey 200 */
    BT_DEVICE(PCI_DEVICE_ID_BROOKTREE_878, 0x0070, 0xff01, snd_bt87x_boardid::SND_BT87X_BOARD_OSPREY2x0),
    /* Viewcast Osprey 440 (rate is configurable via gpio) */
    BT_DEVICE(PCI_DEVICE_ID_BROOKTREE_878, 0x0070, 0xff07, snd_bt87x_boardid::SND_BT87X_BOARD_OSPREY440),
    /* ATI TV-Wonder */
    BT_DEVICE(PCI_DEVICE_ID_BROOKTREE_878, 0x1002, 0x0001, snd_bt87x_boardid::SND_BT87X_BOARD_GENERIC),
    /* Leadtek Winfast tv 2000xp delux */
    BT_DEVICE(PCI_DEVICE_ID_BROOKTREE_878, 0x107d, 0x6606, snd_bt87x_boardid::SND_BT87X_BOARD_GENERIC),
    /* Pinnacle PCTV */
    BT_DEVICE(PCI_DEVICE_ID_BROOKTREE_878, 0x11bd, 0x0012, snd_bt87x_boardid::SND_BT87X_BOARD_GENERIC),
    /* Voodoo TV 200 */
    BT_DEVICE(PCI_DEVICE_ID_BROOKTREE_878, 0x121a, 0x3000, snd_bt87x_boardid::SND_BT87X_BOARD_GENERIC),
    /* Askey Computer Corp. MagicTView'99 */
    BT_DEVICE(PCI_DEVICE_ID_BROOKTREE_878, 0x144f, 0x3000, snd_bt87x_boardid::SND_BT87X_BOARD_GENERIC),
    /* AVerMedia Studio No. 103, 203, ...? */
    BT_DEVICE(PCI_DEVICE_ID_BROOKTREE_878, 0x1461, 0x0003, snd_bt87x_boardid::SND_BT87X_BOARD_AVPHONE98),
    /* Prolink PixelView PV-M4900 */
    BT_DEVICE(PCI_DEVICE_ID_BROOKTREE_878, 0x1554, 0x4011, snd_bt87x_boardid::SND_BT87X_BOARD_GENERIC),
    /* Pinnacle  Studio PCTV rave */
    BT_DEVICE(PCI_DEVICE_ID_BROOKTREE_878, 0xbd11, 0x1200, snd_bt87x_boardid::SND_BT87X_BOARD_GENERIC),
    pci_device_id { vendor: 0, device: 0, subvendor: 0, subdevice: 0, driver_data: 0 },
    pci_device_id { vendor: 0, device: 0, subvendor: 0, subdevice: 0, driver_data: 0 },
];
/* MODULE_DEVICE_TABLE(pci, snd_bt87x_ids); */

/* cards known not to have audio
 * (DVB cards use the audio function to transfer MPEG data) */
#[repr(C)]
#[derive(Copy, Clone)]
struct denylist_entry {
    subvendor: u16,
    subdevice: u16,
}

static denylist: [denylist_entry; 11] = [
    denylist_entry { subvendor: 0x0071, subdevice: 0x0101 }, /* Nebula Electronics DigiTV */
    denylist_entry { subvendor: 0x11bd, subdevice: 0x001c }, /* Pinnacle PCTV Sat */
    denylist_entry { subvendor: 0x11bd, subdevice: 0x0026 }, /* Pinnacle PCTV SAT CI */
    denylist_entry { subvendor: 0x1461, subdevice: 0x0761 }, /* AVermedia AverTV DVB-T */
    denylist_entry { subvendor: 0x1461, subdevice: 0x0771 }, /* AVermedia DVB-T 771 */
    denylist_entry { subvendor: 0x1822, subdevice: 0x0001 }, /* Twinhan VisionPlus DVB-T */
    denylist_entry { subvendor: 0x18ac, subdevice: 0xd500 }, /* DVICO FusionHDTV 5 Lite */
    denylist_entry { subvendor: 0x18ac, subdevice: 0xdb10 }, /* DVICO FusionHDTV DVB-T Lite */
    denylist_entry { subvendor: 0x18ac, subdevice: 0xdb11 }, /* Ultraview DVB-T Lite */
    denylist_entry { subvendor: 0x270f, subdevice: 0xfc00 }, /* Chaintech Digitop DST-1000 DVB-S */
    denylist_entry { subvendor: 0x7063, subdevice: 0x2000 }, /* pcHDTV HD-2000 TV */
];

static mut driver: pci_driver = pci_driver {
    name: ptr::null(),
    id_table: ptr::null(),
    probe: None,
};

/* return the id of the card, or a negative value if it's on the denylist */
unsafe extern "C" fn snd_bt87x_detect_card(pci: *mut pci_dev) -> c_int {
    let mut i: usize;
    let supported: *const pci_device_id;

    supported = pci_match_id(snd_bt87x_ids.as_ptr(), pci);
    if !supported.is_null() && (*supported).driver_data > 0 {
        return (*supported).driver_data as c_int;
    }

    i = 0;
    while i < ARRAY_SIZE(&denylist) {
        if denylist[i].subvendor as c_uint == (*pci).subsystem_vendor
            && denylist[i].subdevice as c_uint == (*pci).subsystem_device
        {
            dev_dbg(&mut (*pci).dev, c"card %#04x-%#04x:%#04x has no audio\n".as_ptr(), (*pci).device, (*pci).subsystem_vendor, (*pci).subsystem_device);
            return -EBUSY;
        }
        i += 1;
    }

    dev_info(&mut (*pci).dev, c"unknown card %#04x-%#04x:%#04x\n".as_ptr(), (*pci).device, (*pci).subsystem_vendor, (*pci).subsystem_device);
    dev_info(&mut (*pci).dev, c"please mail id, board name, and, if it works, the correct digital_rate option to <alsa-devel@alsa-project.org>\n".as_ptr());
    snd_bt87x_boardid::SND_BT87X_BOARD_UNKNOWN as c_int
}

unsafe extern "C" fn __snd_bt87x_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    static mut dev: c_int = 0;
    let mut card: *mut snd_card = ptr::null_mut();
    let chip: *mut snd_bt87x;
    let mut err: c_int;
    let boardid: snd_bt87x_boardid;

    if (*pci_id).driver_data == 0 {
        err = snd_bt87x_detect_card(pci);
        if err < 0 {
            return -ENODEV;
        }
        boardid = mem::transmute::<c_int, snd_bt87x_boardid>(err);
    } else {
        boardid = mem::transmute::<c_ulong, snd_bt87x_boardid>((*pci_id).driver_data);
    }

    if dev >= SNDRV_CARDS as c_int {
        return -ENODEV;
    }
    if !enable[dev as usize] {
        dev += 1;
        return -ENOENT;
    }

    err = snd_devm_card_new(&mut (*pci).dev, index[dev as usize], id[dev as usize], THIS_MODULE, mem::size_of::<snd_bt87x>(), &mut card);
    if err < 0 {
        return err;
    }
    chip = (*card).private_data as *mut snd_bt87x;

    err = snd_bt87x_create(card, pci);
    if err < 0 {
        return err;
    }

    memcpy(&mut (*chip).board as *mut _ as *mut c_void, &snd_bt87x_boards[boardid as usize] as *const _ as *const c_void, mem::size_of_val(&(*chip).board));

    if (*chip).board.no_digital == 0 {
        if digital_rate[dev as usize] > 0 {
            (*chip).board.dig_rate = digital_rate[dev as usize];
        }

        (*chip).reg_control |= (*chip).board.digital_fmt;

        err = snd_bt87x_pcm(chip, DEVICE_DIGITAL, c"Bt87x Digital".as_ptr() as *mut c_char);
        if err < 0 {
            return err;
        }
    }
    if (*chip).board.no_analog == 0 {
        err = snd_bt87x_pcm(chip, DEVICE_ANALOG, c"Bt87x Analog".as_ptr() as *mut c_char);
        if err < 0 {
            return err;
        }
        err = snd_ctl_add(card, snd_ctl_new1(&snd_bt87x_capture_volume, chip as *mut c_void));
        if err < 0 {
            return err;
        }
        err = snd_ctl_add(card, snd_ctl_new1(&snd_bt87x_capture_boost, chip as *mut c_void));
        if err < 0 {
            return err;
        }
        err = snd_ctl_add(card, snd_ctl_new1(&snd_bt87x_capture_source, chip as *mut c_void));
        if err < 0 {
            return err;
        }
    }
    dev_info((*card).dev, c"bt87x%d: Using board %d, %sanalog, %sdigital (rate %d Hz)\n".as_ptr(),
        dev, boardid as c_int,
        if (*chip).board.no_analog != 0 { c"no ".as_ptr() } else { c"".as_ptr() },
        if (*chip).board.no_digital != 0 { c"no ".as_ptr() } else { c"".as_ptr() },
        (*chip).board.dig_rate);

    strscpy((*card).driver.as_mut_ptr(), c"Bt87x".as_ptr());
    sprintf((*card).shortname.as_mut_ptr(), c"Brooktree Bt%x".as_ptr(), (*pci).device);
    sprintf((*card).longname.as_mut_ptr(), c"%s at %#llx, irq %i".as_ptr(), (*card).shortname.as_ptr(), pci_resource_start(pci, 0) as c_ulong, (*chip).irq);
    strscpy((*card).mixername.as_mut_ptr(), c"Bt87x".as_ptr());

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }

    pci_set_drvdata(pci, card as *mut c_void);
    dev += 1;
    0
}

unsafe extern "C" fn snd_bt87x_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    snd_card_free_on_error(&mut (*pci).dev, __snd_bt87x_probe(pci, pci_id))
}

/* default entries for all Bt87x cards - it's not exported */
/* driver_data is set to 0 to call detection */
static snd_bt87x_default_ids: [pci_device_id; 3] = [
    BT_DEVICE(PCI_DEVICE_ID_BROOKTREE_878, PCI_ANY_ID, PCI_ANY_ID, snd_bt87x_boardid::SND_BT87X_BOARD_UNKNOWN),
    BT_DEVICE(PCI_DEVICE_ID_BROOKTREE_879, PCI_ANY_ID, PCI_ANY_ID, snd_bt87x_boardid::SND_BT87X_BOARD_UNKNOWN),
    pci_device_id { vendor: 0, device: 0, subvendor: 0, subdevice: 0, driver_data: 0 },
];

unsafe fn init_driver_struct() {
    driver = pci_driver {
        name: KBUILD_MODNAME,
        id_table: snd_bt87x_ids.as_ptr(),
        probe: Some(snd_bt87x_probe),
    };
}

unsafe extern "C" fn alsa_card_bt87x_init() -> c_int {
    init_driver_struct();
    if load_all {
        driver.id_table = snd_bt87x_default_ids.as_ptr();
    }
    pci_register_driver(&mut driver)
}

unsafe extern "C" fn alsa_card_bt87x_exit() {
    pci_unregister_driver(&mut driver);
}

/* module_init(alsa_card_bt87x_init) */
/* module_exit(alsa_card_bt87x_exit) */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
