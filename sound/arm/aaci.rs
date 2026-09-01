// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/sound/arm/aaci.c - ARM PrimeCell AACI PL041 driver
 *
 *  Copyright (C) 2003 Deep Blue Solutions Ltd, All Rights Reserved.
 *
 *  Documentation: ARM DDI 0173B
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::{addr_of_mut, null_mut};

type u32 = u32;
type ssize_t = isize;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;

const DRIVER_NAME: &[u8] = b"aaci-pl041\0";
const FRAME_PERIOD_US: c_uint = 21;

/*
 * PM support is not complete.  Turn it off.
 */
/* CONFIG_PM intentionally undefined in the C source. */

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: u64,
}

#[repr(C)]
pub struct amba_device {
    pub dev: device,
    pub res: resource,
    pub irq: [c_int; 1],
}

#[repr(C)]
pub struct amba_id {
    pub id: u32,
    pub mask: u32,
}

#[repr(C)]
pub struct driver_inner {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct amba_driver {
    pub drv: driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut amba_device, *const amba_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut amba_device)>,
    pub id_table: *const amba_id,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_ac97 {
    pub private_data: *mut c_void,
    pub num: c_uint,
}

#[repr(C)]
pub struct snd_ac97_bus {
    pub clock: c_uint,
    pub pcms: *mut ac97_pcm,
}

#[repr(C)]
pub struct snd_ac97_template {
    pub private_data: *mut c_void,
    pub num: c_uint,
    pub scaps: c_uint,
}

#[repr(C)]
pub struct ac97_pcm_r {
    pub slots: c_uint,
}

#[repr(C)]
pub struct ac97_pcm {
    pub stream: c_uint,
    pub exclusive: c_uint,
    pub r: [ac97_pcm_r; 2],
    pub rates: c_uint,
}

#[repr(C)]
pub struct snd_ac97_bus_ops {
    pub write: Option<unsafe extern "C" fn(*mut snd_ac97, u16, u16)>,
    pub read: Option<unsafe extern "C" fn(*mut snd_ac97, u16) -> u16>,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: u64,
    pub rates: c_uint,
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
pub struct snd_pcm_runtime {
    pub private_data: *mut c_void,
    pub hw: snd_pcm_hardware,
    pub dma_area: *mut c_void,
    pub rate: c_uint,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub private_data: *mut c_void,
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_rule {
    pub private: *mut c_void,
    pub var: c_int,
}

#[repr(C)]
pub struct snd_pcm {
    pub private_data: *mut c_void,
    pub info_flags: c_uint,
    pub name: [c_char; 80],
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
pub struct aaci_runtime {
    pub substream: *mut snd_pcm_substream,
    pub start: *mut c_void,
    pub end: *mut c_void,
    pub ptr: *mut c_void,
    pub bytes: c_int,
    pub period: c_int,
    pub fifo_bytes: c_uint,
    pub cr: u32,
    pub base: *mut c_void,
    pub fifo: *mut c_void,
    pub lock: spinlock_t,
    pub pcm: *mut ac97_pcm,
    pub pcm_open: c_int,
}

#[repr(C)]
pub struct aaci {
    pub base: *mut c_void,
    pub maincr: u32,
    pub ac97_sem: mutex,
    pub irq_lock: mutex,
    pub users: c_int,
    pub dev: *mut amba_device,
    pub card: *mut snd_card,
    pub ac97_bus: *mut snd_ac97_bus,
    pub ac97: *mut snd_ac97,
    pub pcm: *mut snd_pcm,
    pub playback: aaci_runtime,
    pub capture: aaci_runtime,
    pub fifo_depth: c_uint,
}

unsafe extern "C" {
    static aaci_dev_pm_ops: dev_pm_ops;
    static THIS_MODULE: c_void;

    fn readl(addr: *mut c_void) -> u32;
    fn writel(val: u32, addr: *mut c_void);
    fn udelay(usecs: c_uint);
    fn cond_resched();
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn printk(fmt: *const c_char, ...);
    fn request_irq(irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, dev: *mut c_void) -> c_int;
    fn free_irq(irq: c_int, dev: *mut c_void);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_interval_list(interval: *mut c_void, count: c_uint, list: *const c_uint, mask: c_uint) -> c_int;
    fn hw_param_interval(p: *mut snd_pcm_hw_params, var: c_int) -> *mut c_void;
    fn snd_pcm_limit_hw_rates(runtime: *mut snd_pcm_runtime);
    fn snd_pcm_hw_rule_add(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, func: unsafe extern "C" fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> c_int, private: *mut c_void, dep: c_int, last: c_int) -> c_int;
    fn snd_ac97_pcm_double_rate_rules(runtime: *mut snd_pcm_runtime);
    fn snd_ac97_pcm_close(pcm: *mut ac97_pcm);
    fn snd_ac97_pcm_open(pcm: *mut ac97_pcm, rate: c_uint, channels: c_uint, slots: c_uint) -> c_int;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> c_int;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> c_int;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: ssize_t) -> snd_pcm_uframes_t;
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_ac97_bus(card: *mut snd_card, num: c_int, ops: *const snd_ac97_bus_ops, private: *mut c_void, bus: *mut *mut snd_ac97_bus) -> c_int;
    fn snd_ac97_mixer(bus: *mut snd_ac97_bus, template: *mut snd_ac97_template, ac97: *mut *mut snd_ac97) -> c_int;
    fn ac97_is_audio(ac97: *mut snd_ac97) -> c_int;
    fn snd_ac97_write_cache(ac97: *mut snd_ac97, reg: c_uint, val: c_uint);
    fn snd_ac97_pcm_assign(bus: *mut snd_ac97_bus, count: c_uint, pcms: *const ac97_pcm) -> c_int;
    fn iounmap(addr: *mut c_void);
    fn snd_card_new(dev: *mut device, idx: c_int, id: *const c_char, module: *const c_void, extra_size: usize, card: *mut *mut snd_card) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...);
    fn amba_part(dev: *mut amba_device) -> c_uint;
    fn amba_rev(dev: *mut amba_device) -> c_uint;
    fn mutex_init(lock: *mut mutex);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, pcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, ty: c_int, dev: *mut device, min: usize, max: usize);
    fn amba_request_regions(dev: *mut amba_device, name: *const c_char) -> c_int;
    fn ioremap(offset: u64, size: usize) -> *mut c_void;
    fn resource_size(res: *mut resource) -> usize;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn amba_set_drvdata(dev: *mut amba_device, data: *mut c_void);
    fn snd_card_free(card: *mut snd_card);
    fn amba_release_regions(dev: *mut amba_device);
    fn amba_get_drvdata(dev: *mut amba_device) -> *mut c_void;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn WARN_ON(condition: u32) -> c_int;
}

/* Constants and register offsets are supplied by the translated headers. */
unsafe extern "C" {
    static AACI_SLFR: usize; static AACI_SL2RX: usize; static AACI_SL1RX: usize;
    static AACI_MAINCR: usize; static AACI_SL2TX: usize; static AACI_SL1TX: usize;
    static AACI_SR: usize; static AACI_INTCLR: usize; static AACI_IE: usize;
    static AACI_ALLINTS: usize; static AACI_TXCR: usize; static AACI_RXCR: usize;
    static AACI_RESET: usize; static AACI_CSCH1: usize; static AACI_DR1: usize;
}

macro_rules! ext_const { ($name:ident) => { unsafe { $name } }; }
macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const c_char }; }

unsafe extern "C" { fn MAINCR_SCRA(num: c_uint) -> u32; }
unsafe extern "C" {
    static SLFR_2RXV: u32; static SLFR_1RXV: u32; static SLFR_1TXB: u32; static SLFR_2TXB: u32;
    static ISR_ORINTR: u32; static ICLR_RXOEC1: u32; static ISR_RXTOINTR: u32; static ICLR_RXTOFEC1: u32;
    static ISR_RXINTR: u32; static CR_EN: u32; static SR_RXHF: u32; static SR_RXFF: u32;
    static ISR_URINTR: u32; static ICLR_TXUEC1: u32; static ISR_TXINTR: u32; static SR_TXHE: u32; static SR_TXFE: u32;
    static SNDRV_PCM_INFO_MMAP: c_uint; static SNDRV_PCM_INFO_MMAP_VALID: c_uint; static SNDRV_PCM_INFO_INTERLEAVED: c_uint;
    static SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint; static SNDRV_PCM_INFO_RESUME: c_uint; static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static PAGE_SIZE: c_ulong; static AC97_SLOT_PCM_SLEFT: c_uint; static AC97_SLOT_LFE: c_uint;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int; static SNDRV_PCM_STREAM_CAPTURE: c_int; static IRQF_SHARED: c_ulong;
    static CR_SL3: u32; static CR_SL4: u32; static CR_SL7: u32; static CR_SL8: u32; static CR_SL6: u32; static CR_SL9: u32;
    static CR_FEN: u32; static CR_COMPACT: u32; static CR_SZ16: u32; static EINVAL: c_int;
    static IE_URIE: u32; static IE_TXIE: u32; static SR_TXB: u32; static SR_RXB: u32; static IE_ORIE: u32; static IE_RXIE: u32;
    static SNDRV_PCM_TRIGGER_START: c_int; static SNDRV_PCM_TRIGGER_RESUME: c_int; static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int; static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int; static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_CTL_POWER_D3cold: c_int; static SNDRV_CTL_POWER_D0: c_int;
    static AC97_SLOT_PCM_LEFT: c_uint; static AC97_SLOT_PCM_RIGHT: c_uint; static AC97_SLOT_PCM_CENTER: c_uint;
    static AC97_SLOT_PCM_SRIGHT: c_uint; static AC97_SLOT_PCM_LEFT_0: c_uint; static AC97_SLOT_PCM_RIGHT_0: c_uint;
    static AC97_SLOT_MIC: c_uint; static RESET_NRST: u32; static AC97_SCAP_SKIP_MODEM: c_uint;
    static AC97_PC_BEEP: c_uint; static SNDRV_DEFAULT_IDX1: c_int; static SNDRV_DEFAULT_STR1: *const c_char;
    static MAINCR_IE: u32; static MAINCR_SL1RXEN: u32; static MAINCR_SL1TXEN: u32; static MAINCR_SL2RXEN: u32; static MAINCR_SL2TXEN: u32;
    static SNDRV_DMA_TYPE_DEV: c_int; static ENOMEM: c_int; static KERN_WARNING: *const c_char; static ENODEV: c_int;
    static IRQ_HANDLED: irqreturn_t; static IRQ_NONE: irqreturn_t; static SR_TXFF: u32;
    static AC97_EXTENDED_STATUS: c_uint; static AC97_PCM_LR_ADC_RATE: c_uint; static AC97_PCM_MIC_ADC_RATE: c_uint; static AC97_REC_SEL: c_uint;
}

unsafe fn offset(base: *mut c_void, off: usize) -> *mut c_void {
    (base as *mut u8).add(off) as *mut c_void
}

unsafe extern "C" fn aaci_ac97_select_codec(aaci: *mut aaci, ac97: *mut snd_ac97) {
    let mut v: u32;
    let maincr = (*aaci).maincr | MAINCR_SCRA((*ac97).num);

    /*
     * Ensure that the slot 1/2 RX registers are empty.
     */
    v = readl(offset((*aaci).base, ext_const!(AACI_SLFR)));
    if v & ext_const!(SLFR_2RXV) != 0 {
        readl(offset((*aaci).base, ext_const!(AACI_SL2RX)));
    }
    if v & ext_const!(SLFR_1RXV) != 0 {
        readl(offset((*aaci).base, ext_const!(AACI_SL1RX)));
    }

    if maincr != readl(offset((*aaci).base, ext_const!(AACI_MAINCR))) {
        writel(maincr, offset((*aaci).base, ext_const!(AACI_MAINCR)));
        readl(offset((*aaci).base, ext_const!(AACI_MAINCR)));
        udelay(1);
    }
}

/*
 * P29:
 *  The recommended use of programming the external codec through slot 1
 *  and slot 2 data is to use the channels during setup routines and the
 *  slot register at any other time.  The data written into slot 1, slot 2
 *  and slot 12 registers is transmitted only when their corresponding
 *  SI1TxEn, SI2TxEn and SI12TxEn bits are set in the AACI_MAINCR
 *  register.
 */
unsafe extern "C" fn aaci_ac97_write(ac97: *mut snd_ac97, reg: u16, val: u16) {
    let aaci = (*ac97).private_data as *mut aaci;
    let mut timeout: c_int;
    let mut v: u32 = 0;

    if (*ac97).num >= 4 {
        return;
    }

    /* guard(mutex)(&aaci->ac97_sem); */
    aaci_ac97_select_codec(aaci, ac97);
    writel((val as u32) << 4, offset((*aaci).base, ext_const!(AACI_SL2TX)));
    writel((reg as u32) << 12, offset((*aaci).base, ext_const!(AACI_SL1TX)));
    udelay(FRAME_PERIOD_US);

    timeout = (FRAME_PERIOD_US * 8) as c_int;
    loop {
        udelay(1);
        v = readl(offset((*aaci).base, ext_const!(AACI_SLFR)));
        if !((v & (ext_const!(SLFR_1TXB) | ext_const!(SLFR_2TXB)) != 0) && { timeout -= 1; timeout != 0 }) {
            break;
        }
    }

    if v & (ext_const!(SLFR_1TXB) | ext_const!(SLFR_2TXB)) != 0 {
        dev_err(addr_of_mut!((*(*aaci).dev).dev), cstr!("timeout waiting for write to complete\n"));
    }
}

/*
 * Read an AC'97 register.
 */
unsafe extern "C" fn aaci_ac97_read(ac97: *mut snd_ac97, reg: u16) -> u16 {
    let aaci = (*ac97).private_data as *mut aaci;
    let mut timeout: c_int;
    let mut retries: c_int = 10;
    let mut v: u32 = 0;

    if (*ac97).num >= 4 {
        return !0;
    }

    /* guard(mutex)(&aaci->ac97_sem); */
    aaci_ac97_select_codec(aaci, ac97);
    writel(((reg as u32) << 12) | (1 << 19), offset((*aaci).base, ext_const!(AACI_SL1TX)));
    udelay(FRAME_PERIOD_US);

    timeout = (FRAME_PERIOD_US * 8) as c_int;
    loop {
        udelay(1);
        v = readl(offset((*aaci).base, ext_const!(AACI_SLFR)));
        if !((v & ext_const!(SLFR_1TXB) != 0) && { timeout -= 1; timeout != 0 }) {
            break;
        }
    }

    if v & ext_const!(SLFR_1TXB) != 0 {
        dev_err(addr_of_mut!((*(*aaci).dev).dev), cstr!("timeout on slot 1 TX busy\n"));
        return !0;
    }

    udelay(FRAME_PERIOD_US);
    timeout = (FRAME_PERIOD_US * 8) as c_int;
    loop {
        udelay(1);
        cond_resched();
        v = readl(offset((*aaci).base, ext_const!(AACI_SLFR))) & (ext_const!(SLFR_1RXV) | ext_const!(SLFR_2RXV));
        if !((v != (ext_const!(SLFR_1RXV) | ext_const!(SLFR_2RXV))) && { timeout -= 1; timeout != 0 }) {
            break;
        }
    }

    if v != (ext_const!(SLFR_1RXV) | ext_const!(SLFR_2RXV)) {
        dev_err(addr_of_mut!((*(*aaci).dev).dev), cstr!("timeout on RX valid\n"));
        return !0;
    }

    loop {
        v = readl(offset((*aaci).base, ext_const!(AACI_SL1RX))) >> 12;
        if v == reg as u32 {
            v = readl(offset((*aaci).base, ext_const!(AACI_SL2RX))) >> 4;
            break;
        } else {
            retries -= 1;
            if retries != 0 {
                dev_warn(addr_of_mut!((*(*aaci).dev).dev), cstr!("ac97 read back fail.  retry\n"));
                continue;
            } else {
                dev_warn(addr_of_mut!((*(*aaci).dev).dev), cstr!("wrong ac97 register read back (%x != %x)\n"), v, reg as c_uint);
                v = !0;
            }
        }
        if retries == 0 {
            break;
        }
    }
    v as u16
}

unsafe fn aaci_chan_wait_ready(aacirun: *mut aaci_runtime, mask: c_ulong) {
    let mut val: u32;
    let mut timeout: c_int = 5000;

    loop {
        udelay(1);
        val = readl(offset((*aacirun).base, ext_const!(AACI_SR)));
        if !(val & mask as u32 != 0 && { let old = timeout; timeout -= 1; old != 0 }) {
            break;
        }
    }
}

/*
 * Interrupt support.
 */
unsafe extern "C" fn aaci_fifo_irq(aaci: *mut aaci, channel: c_int, mask: u32) {
    if mask & ext_const!(ISR_ORINTR) != 0 {
        dev_warn(addr_of_mut!((*(*aaci).dev).dev), cstr!("RX overrun on chan %d\n"), channel);
        writel(ext_const!(ICLR_RXOEC1) << channel, offset((*aaci).base, ext_const!(AACI_INTCLR)));
    }

    if mask & ext_const!(ISR_RXTOINTR) != 0 {
        dev_warn(addr_of_mut!((*(*aaci).dev).dev), cstr!("RX timeout on chan %d\n"), channel);
        writel(ext_const!(ICLR_RXTOFEC1) << channel, offset((*aaci).base, ext_const!(AACI_INTCLR)));
    }

    if mask & ext_const!(ISR_RXINTR) != 0 {
        let aacirun = addr_of_mut!((*aaci).capture);
        let mut period_elapsed = false;
        let mut ptr: *mut c_void;

        if (*aacirun).substream.is_null() || (*aacirun).start.is_null() {
            dev_warn(addr_of_mut!((*(*aaci).dev).dev), cstr!("RX interrupt???\n"));
            writel(0, offset((*aacirun).base, ext_const!(AACI_IE)));
            return;
        }

        /* scoped_guard(spinlock, &aacirun->lock) */
        ptr = (*aacirun).ptr;
        loop {
            let mut len = (*aacirun).fifo_bytes;
            let val: u32;

            if (*aacirun).bytes <= 0 {
                (*aacirun).bytes += (*aacirun).period;
                period_elapsed = true;
            }
            if (*aacirun).cr & ext_const!(CR_EN) == 0 {
                break;
            }

            val = readl(offset((*aacirun).base, ext_const!(AACI_SR)));
            if val & ext_const!(SR_RXHF) == 0 {
                break;
            }
            if val & ext_const!(SR_RXFF) == 0 {
                len >>= 1;
            }

            (*aacirun).bytes -= len as c_int;

            /* reading 16 bytes at a time */
            while len > 0 {
                core::ptr::copy_nonoverlapping((*aacirun).fifo as *const u8, ptr as *mut u8, 16);
                ptr = (ptr as *mut u8).add(16) as *mut c_void;
                len -= 16;
                if (ptr as usize) >= ((*aacirun).end as usize) {
                    ptr = (*aacirun).start;
                }
            }
        }
        (*aacirun).ptr = ptr;

        if period_elapsed {
            snd_pcm_period_elapsed((*aacirun).substream);
        }
    }

    if mask & ext_const!(ISR_URINTR) != 0 {
        dev_dbg(addr_of_mut!((*(*aaci).dev).dev), cstr!("TX underrun on chan %d\n"), channel);
        writel(ext_const!(ICLR_TXUEC1) << channel, offset((*aaci).base, ext_const!(AACI_INTCLR)));
    }

    if mask & ext_const!(ISR_TXINTR) != 0 {
        let aacirun = addr_of_mut!((*aaci).playback);
        let mut period_elapsed = false;
        let mut ptr: *mut c_void;

        if (*aacirun).substream.is_null() || (*aacirun).start.is_null() {
            dev_warn(addr_of_mut!((*(*aaci).dev).dev), cstr!("TX interrupt???\n"));
            writel(0, offset((*aacirun).base, ext_const!(AACI_IE)));
            return;
        }

        /* scoped_guard(spinlock, &aacirun->lock) */
        ptr = (*aacirun).ptr;
        loop {
            let mut len = (*aacirun).fifo_bytes;
            let val: u32;

            if (*aacirun).bytes <= 0 {
                (*aacirun).bytes += (*aacirun).period;
                period_elapsed = true;
            }
            if (*aacirun).cr & ext_const!(CR_EN) == 0 {
                break;
            }

            val = readl(offset((*aacirun).base, ext_const!(AACI_SR)));
            if val & ext_const!(SR_TXHE) == 0 {
                break;
            }
            if val & ext_const!(SR_TXFE) == 0 {
                len >>= 1;
            }

            (*aacirun).bytes -= len as c_int;

            /* writing 16 bytes at a time */
            while len > 0 {
                core::ptr::copy_nonoverlapping(ptr as *const u8, (*aacirun).fifo as *mut u8, 16);
                ptr = (ptr as *mut u8).add(16) as *mut c_void;
                len -= 16;
                if (ptr as usize) >= ((*aacirun).end as usize) {
                    ptr = (*aacirun).start;
                }
            }
        }
        (*aacirun).ptr = ptr;

        if period_elapsed {
            snd_pcm_period_elapsed((*aacirun).substream);
        }
    }
}

unsafe extern "C" fn aaci_irq(_irq: c_int, devid: *mut c_void) -> irqreturn_t {
    let aaci = devid as *mut aaci;
    let mask: u32 = readl(offset((*aaci).base, ext_const!(AACI_ALLINTS)));
    if mask != 0 {
        let mut m = mask;
        let mut i = 0;
        while i < 4 {
            if m & 0x7f != 0 {
                aaci_fifo_irq(aaci, i, m);
            }
            i += 1;
            m >>= 7;
        }
    }
    if mask != 0 { ext_const!(IRQ_HANDLED) } else { ext_const!(IRQ_NONE) }
}

/*
 * ALSA support.
 */
static mut aaci_hw_info: snd_pcm_hardware = snd_pcm_hardware {
    info: 0, formats: 0, rates: 0, channels_min: 2, channels_max: 2,
    buffer_bytes_max: 64 * 1024, period_bytes_min: 256, period_bytes_max: 0,
    periods_min: 4, periods_max: 0, fifo_size: 0,
};

/*
 * We can support two and four channel audio.  Unfortunately
 * six channel audio requires a non-standard channel ordering:
 *   2 -> FL(3), FR(4)
 *   4 -> FL(3), FR(4), SL(7), SR(8)
 *   6 -> FL(3), FR(4), SL(7), SR(8), C(6), LFE(9) (required)
 *        FL(3), FR(4), C(6), SL(7), SR(8), LFE(9) (actual)
 * This requires an ALSA configuration file to correct.
 */
unsafe extern "C" fn aaci_rule_channels(p: *mut snd_pcm_hw_params, rule: *mut snd_pcm_hw_rule) -> c_int {
    static channel_list: [c_uint; 3] = [2, 4, 6];
    let aaci = (*rule).private as *mut aaci;
    let mut mask: c_uint = 1 << 0;
    let slots: c_uint;

    /* pcms[0] is the our 5.1 PCM instance. */
    slots = (*(*aaci).ac97_bus).pcms.add(0).read().r[0].slots;
    if slots & (1 << ext_const!(AC97_SLOT_PCM_SLEFT)) != 0 {
        mask |= 1 << 1;
        if slots & (1 << ext_const!(AC97_SLOT_LFE)) != 0 {
            mask |= 1 << 2;
        }
    }

    snd_interval_list(hw_param_interval(p, (*rule).var), channel_list.len() as c_uint, channel_list.as_ptr(), mask)
}

unsafe extern "C" fn aaci_pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let aaci = (*substream).private_data as *mut aaci;
    let aacirun: *mut aaci_runtime;
    let mut ret: c_int = 0;

    if (*substream).stream == ext_const!(SNDRV_PCM_STREAM_PLAYBACK) {
        aacirun = addr_of_mut!((*aaci).playback);
    } else {
        aacirun = addr_of_mut!((*aaci).capture);
    }

    (*aacirun).substream = substream;
    (*runtime).private_data = aacirun as *mut c_void;
    (*runtime).hw = aaci_hw_info;
    (*runtime).hw.info = ext_const!(SNDRV_PCM_INFO_MMAP) | ext_const!(SNDRV_PCM_INFO_MMAP_VALID) | ext_const!(SNDRV_PCM_INFO_INTERLEAVED) | ext_const!(SNDRV_PCM_INFO_BLOCK_TRANSFER) | ext_const!(SNDRV_PCM_INFO_RESUME);
    (*runtime).hw.formats = ext_const!(SNDRV_PCM_FMTBIT_S16_LE);
    (*runtime).hw.period_bytes_max = ext_const!(PAGE_SIZE);
    (*runtime).hw.periods_max = (ext_const!(PAGE_SIZE) / 16) as c_uint;
    (*runtime).hw.rates = (*(*aacirun).pcm).rates;
    snd_pcm_limit_hw_rates(runtime);

    if (*substream).stream == ext_const!(SNDRV_PCM_STREAM_PLAYBACK) {
        (*runtime).hw.channels_max = 6;
        ret = snd_pcm_hw_rule_add(runtime, 0, 0, aaci_rule_channels, aaci as *mut c_void, 0, -1);
        if ret != 0 {
            return ret;
        }
        if (*(*aacirun).pcm).r[1].slots != 0 {
            snd_ac97_pcm_double_rate_rules(runtime);
        }
    }

    (*runtime).hw.fifo_size = (*aaci).fifo_depth * 2;

    /* guard(mutex)(&aaci->irq_lock); */
    let old_users = (*aaci).users;
    (*aaci).users += 1;
    if old_users == 0 {
        ret = request_irq((*(*aaci).dev).irq[0], aaci_irq, ext_const!(IRQF_SHARED), DRIVER_NAME.as_ptr() as *const c_char, aaci as *mut c_void);
        if ret != 0 {
            (*aaci).users -= 1;
        }
    }

    ret
}

/*
 * Common ALSA stuff
 */
unsafe extern "C" fn aaci_pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    let aaci = (*substream).private_data as *mut aaci;
    let aacirun = (*(*substream).runtime).private_data as *mut aaci_runtime;

    WARN_ON((*aacirun).cr & ext_const!(CR_EN));
    (*aacirun).substream = null_mut();

    /* guard(mutex)(&aaci->irq_lock); */
    (*aaci).users -= 1;
    if (*aaci).users == 0 {
        free_irq((*(*aaci).dev).irq[0], aaci as *mut c_void);
    }

    0
}

unsafe extern "C" fn aaci_pcm_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let aacirun = (*(*substream).runtime).private_data as *mut aaci_runtime;
    WARN_ON((*aacirun).cr & ext_const!(CR_EN));
    if (*aacirun).pcm_open != 0 {
        snd_ac97_pcm_close((*aacirun).pcm);
    }
    (*aacirun).pcm_open = 0;
    0
}

/* Channel to slot mask */
static mut channels_to_slotmask: [u32; 9] = [0; 9];

unsafe extern "C" fn aaci_pcm_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let aacirun = (*(*substream).runtime).private_data as *mut aaci_runtime;
    let aaci = (*substream).private_data as *mut aaci;
    let channels = params_channels(params);
    let rate = params_rate(params);
    let dbl: c_int = if rate > 48000 { 1 } else { 0 };
    let err: c_int;

    aaci_pcm_hw_free(substream);
    if (*aacirun).pcm_open != 0 {
        snd_ac97_pcm_close((*aacirun).pcm);
        (*aacirun).pcm_open = 0;
    }

    /* channels is already limited to 2, 4, or 6 by aaci_rule_channels */
    if dbl != 0 && channels != 2 {
        return -ext_const!(EINVAL);
    }

    err = snd_ac97_pcm_open((*aacirun).pcm, rate, channels, (*(*aacirun).pcm).r[dbl as usize].slots);
    (*aacirun).pcm_open = (err == 0) as c_int;
    (*aacirun).cr = ext_const!(CR_FEN) | ext_const!(CR_COMPACT) | ext_const!(CR_SZ16);
    channels_to_slotmask[2] = ext_const!(CR_SL3) | ext_const!(CR_SL4);
    channels_to_slotmask[4] = ext_const!(CR_SL3) | ext_const!(CR_SL4) | ext_const!(CR_SL7) | ext_const!(CR_SL8);
    channels_to_slotmask[6] = ext_const!(CR_SL3) | ext_const!(CR_SL4) | ext_const!(CR_SL7) | ext_const!(CR_SL8) | ext_const!(CR_SL6) | ext_const!(CR_SL9);
    (*aacirun).cr |= channels_to_slotmask[(channels + (dbl as c_uint) * 2) as usize];
    (*aacirun).fifo_bytes = (*aaci).fifo_depth * 4 / 2;

    err
}

unsafe extern "C" fn aaci_pcm_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let aacirun = (*runtime).private_data as *mut aaci_runtime;

    (*aacirun).period = snd_pcm_lib_period_bytes(substream);
    (*aacirun).start = (*runtime).dma_area;
    (*aacirun).end = ((*aacirun).start as *mut u8).add(snd_pcm_lib_buffer_bytes(substream) as usize) as *mut c_void;
    (*aacirun).ptr = (*aacirun).start;
    (*aacirun).bytes = (*aacirun).period;
    0
}

unsafe extern "C" fn aaci_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let runtime = (*substream).runtime;
    let aacirun = (*runtime).private_data as *mut aaci_runtime;
    let bytes: ssize_t = ((*aacirun).ptr as isize) - ((*aacirun).start as isize);
    bytes_to_frames(runtime, bytes)
}

/*
 * Playback specific ALSA stuff
 */
unsafe extern "C" fn aaci_pcm_playback_stop(aacirun: *mut aaci_runtime) {
    let mut ie = readl(offset((*aacirun).base, ext_const!(AACI_IE)));
    ie &= !(ext_const!(IE_URIE) | ext_const!(IE_TXIE));
    writel(ie, offset((*aacirun).base, ext_const!(AACI_IE)));
    (*aacirun).cr &= !ext_const!(CR_EN);
    aaci_chan_wait_ready(aacirun, ext_const!(SR_TXB) as c_ulong);
    writel((*aacirun).cr, offset((*aacirun).base, ext_const!(AACI_TXCR)));
}

unsafe extern "C" fn aaci_pcm_playback_start(aacirun: *mut aaci_runtime) {
    let mut ie: u32;
    aaci_chan_wait_ready(aacirun, ext_const!(SR_TXB) as c_ulong);
    (*aacirun).cr |= ext_const!(CR_EN);
    ie = readl(offset((*aacirun).base, ext_const!(AACI_IE)));
    ie |= ext_const!(IE_URIE) | ext_const!(IE_TXIE);
    writel(ie, offset((*aacirun).base, ext_const!(AACI_IE)));
    writel((*aacirun).cr, offset((*aacirun).base, ext_const!(AACI_TXCR)));
}

unsafe extern "C" fn aaci_pcm_playback_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let aacirun = (*(*substream).runtime).private_data as *mut aaci_runtime;
    /* guard(spinlock_irqsave)(&aacirun->lock); */
    if cmd == ext_const!(SNDRV_PCM_TRIGGER_START) || cmd == ext_const!(SNDRV_PCM_TRIGGER_RESUME) {
        aaci_pcm_playback_start(aacirun);
    } else if cmd == ext_const!(SNDRV_PCM_TRIGGER_STOP) || cmd == ext_const!(SNDRV_PCM_TRIGGER_SUSPEND) {
        aaci_pcm_playback_stop(aacirun);
    } else if cmd == ext_const!(SNDRV_PCM_TRIGGER_PAUSE_PUSH) || cmd == ext_const!(SNDRV_PCM_TRIGGER_PAUSE_RELEASE) {
    } else {
        return -ext_const!(EINVAL);
    }
    0
}

static aaci_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(aaci_pcm_open), close: Some(aaci_pcm_close), hw_params: Some(aaci_pcm_hw_params),
    hw_free: Some(aaci_pcm_hw_free), prepare: Some(aaci_pcm_prepare), trigger: Some(aaci_pcm_playback_trigger),
    pointer: Some(aaci_pcm_pointer),
};

unsafe extern "C" fn aaci_pcm_capture_stop(aacirun: *mut aaci_runtime) {
    let mut ie: u32;
    aaci_chan_wait_ready(aacirun, ext_const!(SR_RXB) as c_ulong);
    ie = readl(offset((*aacirun).base, ext_const!(AACI_IE)));
    ie &= !(ext_const!(IE_ORIE) | ext_const!(IE_RXIE));
    writel(ie, offset((*aacirun).base, ext_const!(AACI_IE)));
    (*aacirun).cr &= !ext_const!(CR_EN);
    writel((*aacirun).cr, offset((*aacirun).base, ext_const!(AACI_RXCR)));
}

unsafe extern "C" fn aaci_pcm_capture_start(aacirun: *mut aaci_runtime) {
    let mut ie: u32;
    aaci_chan_wait_ready(aacirun, ext_const!(SR_RXB) as c_ulong);
    /* #ifdef DEBUG: RX Timeout value: bits 28:17 in RXCR; aacirun->cr |= 0xf << 17; */
    (*aacirun).cr |= ext_const!(CR_EN);
    writel((*aacirun).cr, offset((*aacirun).base, ext_const!(AACI_RXCR)));
    ie = readl(offset((*aacirun).base, ext_const!(AACI_IE)));
    ie |= ext_const!(IE_ORIE) | ext_const!(IE_RXIE); // overrun and rx interrupt -- half full
    writel(ie, offset((*aacirun).base, ext_const!(AACI_IE)));
}

unsafe extern "C" fn aaci_pcm_capture_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let aacirun = (*(*substream).runtime).private_data as *mut aaci_runtime;
    /* guard(spinlock_irqsave)(&aacirun->lock); */
    if cmd == ext_const!(SNDRV_PCM_TRIGGER_START) || cmd == ext_const!(SNDRV_PCM_TRIGGER_RESUME) {
        aaci_pcm_capture_start(aacirun);
    } else if cmd == ext_const!(SNDRV_PCM_TRIGGER_STOP) || cmd == ext_const!(SNDRV_PCM_TRIGGER_SUSPEND) {
        aaci_pcm_capture_stop(aacirun);
    } else if cmd == ext_const!(SNDRV_PCM_TRIGGER_PAUSE_PUSH) || cmd == ext_const!(SNDRV_PCM_TRIGGER_PAUSE_RELEASE) {
    } else {
        return -ext_const!(EINVAL);
    }
    0
}

unsafe extern "C" fn aaci_pcm_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let aaci = (*substream).private_data as *mut aaci;

    aaci_pcm_prepare(substream);
    /* allow changing of sample rate */
    aaci_ac97_write((*aaci).ac97, ext_const!(AC97_EXTENDED_STATUS) as u16, 0x0001); /* VRA */
    aaci_ac97_write((*aaci).ac97, ext_const!(AC97_PCM_LR_ADC_RATE) as u16, (*runtime).rate as u16);
    aaci_ac97_write((*aaci).ac97, ext_const!(AC97_PCM_MIC_ADC_RATE) as u16, (*runtime).rate as u16);
    /* Record select: Mic: 0, Aux: 3, Line: 4 */
    aaci_ac97_write((*aaci).ac97, ext_const!(AC97_REC_SEL) as u16, 0x0404);
    0
}

static aaci_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(aaci_pcm_open), close: Some(aaci_pcm_close), hw_params: Some(aaci_pcm_hw_params),
    hw_free: Some(aaci_pcm_hw_free), prepare: Some(aaci_pcm_capture_prepare), trigger: Some(aaci_pcm_capture_trigger),
    pointer: Some(aaci_pcm_pointer),
};

/*
 * Power Management.
 */
unsafe extern "C" fn aaci_do_suspend(card: *mut snd_card) -> c_int {
    snd_power_change_state(card, ext_const!(SNDRV_CTL_POWER_D3cold));
    0
}

unsafe extern "C" fn aaci_do_resume(card: *mut snd_card) -> c_int {
    snd_power_change_state(card, ext_const!(SNDRV_CTL_POWER_D0));
    0
}

unsafe extern "C" fn aaci_suspend(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    if !card.is_null() { aaci_do_suspend(card) } else { 0 }
}

unsafe extern "C" fn aaci_resume(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    if !card.is_null() { aaci_do_resume(card) } else { 0 }
}

/* static DEFINE_SIMPLE_DEV_PM_OPS(aaci_dev_pm_ops, aaci_suspend, aaci_resume); */

static mut ac97_defs: [ac97_pcm; 3] = [
    ac97_pcm { /* Front PCM */
        stream: 0, exclusive: 1,
        r: [
            ac97_pcm_r { slots: 0 },
            ac97_pcm_r { slots: 0 },
        ],
        rates: 0,
    },
    ac97_pcm { /* PCM in */
        stream: 1, exclusive: 1,
        r: [ac97_pcm_r { slots: 0 }, ac97_pcm_r { slots: 0 }],
        rates: 0,
    },
    ac97_pcm { /* Mic in */
        stream: 1, exclusive: 1,
        r: [ac97_pcm_r { slots: 0 }, ac97_pcm_r { slots: 0 }],
        rates: 0,
    },
];

unsafe fn aaci_init_ac97_defs() {
    ac97_defs[0].r[0].slots = (1 << ext_const!(AC97_SLOT_PCM_LEFT))
        | (1 << ext_const!(AC97_SLOT_PCM_RIGHT))
        | (1 << ext_const!(AC97_SLOT_PCM_CENTER))
        | (1 << ext_const!(AC97_SLOT_PCM_SLEFT))
        | (1 << ext_const!(AC97_SLOT_PCM_SRIGHT))
        | (1 << ext_const!(AC97_SLOT_LFE));
    ac97_defs[0].r[1].slots = (1 << ext_const!(AC97_SLOT_PCM_LEFT))
        | (1 << ext_const!(AC97_SLOT_PCM_RIGHT))
        | (1 << ext_const!(AC97_SLOT_PCM_LEFT_0))
        | (1 << ext_const!(AC97_SLOT_PCM_RIGHT_0));
    ac97_defs[1].r[0].slots = (1 << ext_const!(AC97_SLOT_PCM_LEFT))
        | (1 << ext_const!(AC97_SLOT_PCM_RIGHT));
    ac97_defs[2].r[0].slots = 1 << ext_const!(AC97_SLOT_MIC);
}

static aaci_bus_ops: snd_ac97_bus_ops = snd_ac97_bus_ops {
    write: Some(aaci_ac97_write),
    read: Some(aaci_ac97_read),
};

unsafe extern "C" fn aaci_probe_ac97(aaci: *mut aaci) -> c_int {
    let mut ac97_template: snd_ac97_template = core::mem::zeroed();
    let mut ac97_bus: *mut snd_ac97_bus = null_mut();
    let mut ac97: *mut snd_ac97 = null_mut();
    let mut ret: c_int;

    /*
     * Assert AACIRESET for 2us
     */
    writel(0, offset((*aaci).base, ext_const!(AACI_RESET)));
    udelay(2);
    writel(ext_const!(RESET_NRST), offset((*aaci).base, ext_const!(AACI_RESET)));
    udelay(FRAME_PERIOD_US * 2);

    ret = snd_ac97_bus((*aaci).card, 0, &aaci_bus_ops, aaci as *mut c_void, &mut ac97_bus);
    if ret != 0 { return ret; }

    (*ac97_bus).clock = 48000;
    (*aaci).ac97_bus = ac97_bus;

    ac97_template.private_data = aaci as *mut c_void;
    ac97_template.num = 0;
    ac97_template.scaps = ext_const!(AC97_SCAP_SKIP_MODEM);

    ret = snd_ac97_mixer(ac97_bus, &mut ac97_template, &mut ac97);
    if ret != 0 { return ret; }
    (*aaci).ac97 = ac97;

    /*
     * Disable AC97 PC Beep input on audio codecs.
     */
    if ac97_is_audio(ac97) != 0 {
        snd_ac97_write_cache(ac97, ext_const!(AC97_PC_BEEP), 0x801e);
    }

    aaci_init_ac97_defs();
    ret = snd_ac97_pcm_assign(ac97_bus, ac97_defs.len() as c_uint, ac97_defs.as_ptr());
    if ret != 0 { return ret; }

    (*aaci).playback.pcm = (*ac97_bus).pcms.add(0);
    (*aaci).capture.pcm = (*ac97_bus).pcms.add(1);
    ret
}

unsafe extern "C" fn aaci_free_card(card: *mut snd_card) {
    let aaci = (*card).private_data as *mut aaci;
    iounmap((*aaci).base);
}

unsafe extern "C" fn aaci_init_card(dev: *mut amba_device) -> *mut aaci {
    let mut aaci: *mut aaci;
    let mut card: *mut snd_card = null_mut();
    let err: c_int;

    err = snd_card_new(addr_of_mut!((*dev).dev), ext_const!(SNDRV_DEFAULT_IDX1), ext_const!(SNDRV_DEFAULT_STR1), &THIS_MODULE, size_of::<aaci>(), &mut card);
    if err < 0 {
        return null_mut();
    }

    (*card).private_free = Some(aaci_free_card);
    strscpy((*card).driver.as_mut_ptr(), DRIVER_NAME.as_ptr() as *const c_char, (*card).driver.len());
    strscpy((*card).shortname.as_mut_ptr(), cstr!("ARM AC'97 Interface"), (*card).shortname.len());
    snprintf((*card).longname.as_mut_ptr(), (*card).longname.len(), cstr!("%s PL%03x rev%u at 0x%08llx, irq %d"), (*card).shortname.as_ptr(), amba_part(dev), amba_rev(dev), (*dev).res.start, (*dev).irq[0]);

    aaci = (*card).private_data as *mut aaci;
    mutex_init(addr_of_mut!((*aaci).ac97_sem));
    mutex_init(addr_of_mut!((*aaci).irq_lock));
    (*aaci).card = card;
    (*aaci).dev = dev;
    (*aaci).maincr = ext_const!(MAINCR_IE) | ext_const!(MAINCR_SL1RXEN) | ext_const!(MAINCR_SL1TXEN) | ext_const!(MAINCR_SL2RXEN) | ext_const!(MAINCR_SL2TXEN);
    aaci
}

unsafe extern "C" fn aaci_init_pcm(aaci: *mut aaci) -> c_int {
    let mut pcm: *mut snd_pcm = null_mut();
    let ret = snd_pcm_new((*aaci).card, cstr!("AACI AC'97"), 0, 1, 1, &mut pcm);
    if ret == 0 {
        (*aaci).pcm = pcm;
        (*pcm).private_data = aaci as *mut c_void;
        (*pcm).info_flags = 0;
        strscpy((*pcm).name.as_mut_ptr(), DRIVER_NAME.as_ptr() as *const c_char, (*pcm).name.len());
        snd_pcm_set_ops(pcm, ext_const!(SNDRV_PCM_STREAM_PLAYBACK), &aaci_playback_ops);
        snd_pcm_set_ops(pcm, ext_const!(SNDRV_PCM_STREAM_CAPTURE), &aaci_capture_ops);
        snd_pcm_set_managed_buffer_all(pcm, ext_const!(SNDRV_DMA_TYPE_DEV), (*(*aaci).card).dev, 0, 64 * 1024);
    }
    ret
}

unsafe extern "C" fn aaci_size_fifo(aaci: *mut aaci) -> c_uint {
    let aacirun = addr_of_mut!((*aaci).playback);
    let mut i: c_int = 0;

    /*
     * Enable the channel, but don't assign it to any slots, so
     * it won't empty onto the AC'97 link.
     */
    writel(ext_const!(CR_FEN) | ext_const!(CR_SZ16) | ext_const!(CR_EN), offset((*aacirun).base, ext_const!(AACI_TXCR)));

    while readl(offset((*aacirun).base, ext_const!(AACI_SR))) & ext_const!(SR_TXFF) == 0 && i < 4096 {
        writel(0, (*aacirun).fifo);
        i += 1;
    }

    writel(0, offset((*aacirun).base, ext_const!(AACI_TXCR)));
    writel((*aaci).maincr & !ext_const!(MAINCR_IE), offset((*aaci).base, ext_const!(AACI_MAINCR)));
    readl(offset((*aaci).base, ext_const!(AACI_MAINCR)));
    udelay(1);
    writel((*aaci).maincr, offset((*aaci).base, ext_const!(AACI_MAINCR)));

    if i == 4096 {
        i = 8;
    }
    i as c_uint
}

unsafe extern "C" fn aaci_probe(dev: *mut amba_device, _id: *const amba_id) -> c_int {
    let mut aaci: *mut aaci;
    let mut ret: c_int;
    let mut i: c_int;

    ret = amba_request_regions(dev, null_mut());
    if ret != 0 { return ret; }

    aaci = aaci_init_card(dev);
    if aaci.is_null() {
        ret = -ext_const!(ENOMEM);
        goto_out(aaci, dev, ret)
    } else {
        (*aaci).base = ioremap((*dev).res.start, resource_size(addr_of_mut!((*dev).res)));
        if (*aaci).base.is_null() {
            ret = -ext_const!(ENOMEM);
            return goto_out(aaci, dev, ret);
        }

        spin_lock_init(addr_of_mut!((*aaci).playback.lock));
        (*aaci).playback.base = offset((*aaci).base, ext_const!(AACI_CSCH1));
        (*aaci).playback.fifo = offset((*aaci).base, ext_const!(AACI_DR1));

        spin_lock_init(addr_of_mut!((*aaci).capture.lock));
        (*aaci).capture.base = offset((*aaci).base, ext_const!(AACI_CSCH1));
        (*aaci).capture.fifo = offset((*aaci).base, ext_const!(AACI_DR1));

        i = 0;
        while i < 4 {
            let base = offset((*aaci).base, (i as usize) * 0x14);
            writel(0, offset(base, ext_const!(AACI_IE)));
            writel(0, offset(base, ext_const!(AACI_TXCR)));
            writel(0, offset(base, ext_const!(AACI_RXCR)));
            i += 1;
        }

        writel(0x1fff, offset((*aaci).base, ext_const!(AACI_INTCLR)));
        writel((*aaci).maincr, offset((*aaci).base, ext_const!(AACI_MAINCR)));
        readl(offset((*aaci).base, ext_const!(AACI_CSCH1)));
        ret = aaci_probe_ac97(aaci);
        if ret != 0 { return goto_out(aaci, dev, ret); }

        (*aaci).fifo_depth = aaci_size_fifo(aaci);
        if (*aaci).fifo_depth & 15 != 0 {
            printk(cstr!("AACI: FIFO depth %d not supported\n"), (*aaci).fifo_depth);
            ret = -ext_const!(ENODEV);
            return goto_out(aaci, dev, ret);
        }

        ret = aaci_init_pcm(aaci);
        if ret != 0 { return goto_out(aaci, dev, ret); }

        ret = snd_card_register((*aaci).card);
        if ret == 0 {
            dev_info(addr_of_mut!((*dev).dev), cstr!("%s\n"), (*(*aaci).card).longname.as_ptr());
            dev_info(addr_of_mut!((*dev).dev), cstr!("FIFO %u entries\n"), (*aaci).fifo_depth);
            amba_set_drvdata(dev, (*aaci).card as *mut c_void);
            return ret;
        }

        goto_out(aaci, dev, ret)
    }
}

unsafe fn goto_out(aaci: *mut aaci, dev: *mut amba_device, ret: c_int) -> c_int {
    if !aaci.is_null() {
        snd_card_free((*aaci).card);
    }
    amba_release_regions(dev);
    ret
}

unsafe extern "C" fn aaci_remove(dev: *mut amba_device) {
    let card = amba_get_drvdata(dev) as *mut snd_card;
    if !card.is_null() {
        let aaci = (*card).private_data as *mut aaci;
        writel(0, offset((*aaci).base, ext_const!(AACI_MAINCR)));
        snd_card_free(card);
        amba_release_regions(dev);
    }
}

static aaci_ids: [amba_id; 2] = [
    amba_id { id: 0x00041041, mask: 0x000fffff },
    amba_id { id: 0, mask: 0 },
];

/* MODULE_DEVICE_TABLE(amba, aaci_ids); */

static mut aaci_driver: amba_driver = amba_driver {
    drv: driver_inner {
        name: DRIVER_NAME.as_ptr() as *const c_char,
        pm: unsafe { &aaci_dev_pm_ops },
    },
    probe: Some(aaci_probe),
    remove: Some(aaci_remove),
    id_table: aaci_ids.as_ptr(),
};

/* module_amba_driver(aaci_driver); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_DESCRIPTION("ARM PrimeCell PL041 Advanced Audio CODEC Interface driver"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
