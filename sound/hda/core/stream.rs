// SPDX-License-Identifier: GPL-2.0-only
/*
 * HD-audio stream operations
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type bool_ = bool;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type dma_addr_t = u64;
type __le32 = u32;

const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const AZX_MAX_BDL_ENTRIES: c_int = 256;
const NSEC_PER_SEC: u32 = 1_000_000_000;

extern "C" {
    static AZX_GCAP_NSDO: c_uint;
    static GCAP: c_uint;
    static WALLCLK: c_uint;
    static INTCTL: c_uint;
    static SD_CTL_3B: c_uint;
    static SD_CTL: c_uint;
    static SD_CTL_STRIPE_MASK: c_uint;
    static SD_CTL_DMA_START: c_uint;
    static SD_INT_MASK: c_uint;
    static SD_STS: c_uint;
    static SD_CTL_STREAM_RESET: c_uint;
    static SD_CTL_STREAM_TAG_MASK: c_uint;
    static SD_CTL_STREAM_TAG_SHIFT: c_uint;
    static SD_CTL_TRAFFIC_PRIO: c_uint;
    static SD_CBL: c_uint;
    static SD_FORMAT: c_uint;
    static SD_LVI: c_uint;
    static SD_BDLPL: c_uint;
    static SD_BDLPU: c_uint;
    static DPLBASE: c_uint;
    static AZX_DPLBASE_ENABLE: c_uint;
    static SD_FIFOSIZE: c_uint;
    static AZX_SD_FIFOSIZE_MASK: u16;
    static AZX_REG_SD_FIFOSIZE: c_uint;
    static AZX_REG_SSYNC: c_uint;
    static SD_STS_FIFO_READY: c_uint;
    static AZX_SPB_BASE: usize;
    static AZX_SPB_INTERVAL: usize;
    static AZX_SPB_SPIB: usize;
    static AZX_SPB_MAXFIFO: usize;
    static AZX_REG_SPB_SPBFCCTL: c_uint;
    static AZX_DRSM_BASE: usize;
    static AZX_DRSM_INTERVAL: usize;
    static AZX_REG_DRSM_CTL: c_uint;
    static SNDRV_DMA_TYPE_DEV_SG: c_int;

    fn snd_hdac_chip_readl(bus: *mut hdac_bus, reg: c_uint) -> c_uint;
    fn snd_hdac_chip_updatel(bus: *mut hdac_bus, reg: c_uint, mask: c_uint, val: c_uint);
    fn snd_hdac_chip_writel(bus: *mut hdac_bus, reg: c_uint, val: c_uint);
    fn _snd_hdac_chip_readl(bus: *mut hdac_bus, reg: c_uint) -> c_uint;
    fn _snd_hdac_chip_writel(bus: *mut hdac_bus, reg: c_uint, val: c_uint);
    fn snd_hdac_stream_readb(azx_dev: *mut hdac_stream, reg: c_uint) -> c_uint;
    fn snd_hdac_stream_readl(azx_dev: *mut hdac_stream, reg: c_uint) -> c_uint;
    fn snd_hdac_stream_writel(azx_dev: *mut hdac_stream, reg: c_uint, val: c_uint);
    fn snd_hdac_stream_writew(azx_dev: *mut hdac_stream, reg: c_uint, val: c_uint);
    fn snd_hdac_stream_writeb(azx_dev: *mut hdac_stream, reg: c_uint, val: c_uint);
    fn snd_hdac_stream_updatel(azx_dev: *mut hdac_stream, reg: c_uint, mask: c_uint, val: c_uint);
    fn snd_hdac_stream_updateb(azx_dev: *mut hdac_stream, reg: c_uint, mask: c_uint, val: c_uint);
    fn snd_hdac_stream_readb_poll(azx_dev: *mut hdac_stream, reg: c_uint, val: *mut u8, cond: c_int, sleep_us: c_uint, timeout_us: c_uint) -> c_int;
    fn snd_hdac_stream_readw_poll(azx_dev: *mut hdac_stream, reg: c_uint, val: *mut u16, cond: c_int, sleep_us: c_uint, timeout_us: c_uint) -> c_int;
    fn snd_hdac_dsp_lock_init(azx_dev: *mut hdac_stream);
    fn snd_hdac_bus_stop_chip(bus: *mut hdac_bus);
    fn snd_hdac_updatel(addr: *mut c_void, reg: c_uint, mask: c_uint, val: c_uint);
    fn snd_hdac_reg_readl(bus: *mut hdac_bus, reg: *mut c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn udelay(usecs: c_uint);
    fn cpu_relax();
    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn snd_sgbuf_get_addr(dmab: *mut snd_dma_buffer, ofs: c_int) -> dma_addr_t;
    fn snd_sgbuf_get_chunk_size(dmab: *mut snd_dma_buffer, ofs: c_int, size: c_int) -> c_int;
    fn snd_pcm_get_dma_buf_pcm(substream: *mut snd_pcm_substream) -> *mut snd_dma_buffer;
    fn snd_pcm_get_dma_buf_compr(cstream: *mut snd_compr_stream) -> *mut snd_dma_buffer;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: c_int) -> c_int;
    fn clocks_calc_mult_shift(mult: *mut u32, shift: *mut u32, from: u32, to: u32, minsec: u32);
    fn timecounter_init(tc: *mut timecounter, cc: *mut cyclecounter, nsec: u64);
    fn snd_pcm_gettime(runtime: *mut snd_pcm_runtime, tstamp: *mut timespec);
    fn snd_dma_alloc_pages(ty: c_int, dev: *mut c_void, size: c_uint, bufp: *mut snd_dma_buffer) -> c_int;
    fn snd_dma_free_pages(bufp: *mut snd_dma_buffer);
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct snd_pcm {
    pub device: c_int,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub channels: c_uint,
    pub rate: c_uint,
    pub sample_bits: c_uint,
    pub period_size: c_int,
    pub no_period_wakeup: c_uint,
    pub trigger_tstamp: timespec,
    pub trigger_tstamp_latched: bool_,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub number: c_int,
    pub stream: c_int,
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_compr_runtime {
    pub buffer_size: c_uint,
    pub fragment_size: c_uint,
}

#[repr(C)]
pub struct snd_compr_stream {
    pub runtime: *mut snd_compr_runtime,
}

#[repr(C)]
pub struct snd_dma_buffer {
    pub area: *mut c_void,
    pub addr: dma_addr_t,
}

#[repr(C)]
pub struct cyclecounter {
    pub read: Option<unsafe extern "C" fn(*mut cyclecounter) -> u64>,
    pub mask: u64,
    pub mult: u32,
    pub shift: u32,
}

#[repr(C)]
pub struct timecounter {
    pub cc: *mut cyclecounter,
    pub cycle_last: u64,
    pub nsec: u64,
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[repr(C)]
pub struct hdac_bus {
    pub remap_addr: *mut u8,
    pub stream_list: list_head,
    pub spbcap: *mut c_void,
    pub drsmcap: *mut c_void,
    pub access_sdnctl_in_dword: bool_,
    pub sdo_limit: c_uint,
    pub chip_init: bool_,
    pub snoop: bool_,
    pub use_posbuf: bool_,
    pub posbuf: snd_dma_buffer,
    pub addr_offset: dma_addr_t,
    pub dev: *mut c_void,
    pub reverse_assign: bool_,
    pub align_bdle_4k: bool_,
    pub bdl_pos_adj: c_int,
    pub dma_stop_delay: c_uint,
    pub reg_lock: c_void,
}

#[repr(C)]
pub struct hdac_stream {
    pub bus: *mut hdac_bus,
    pub sd_addr: *mut u8,
    pub sd_int_sta_mask: c_uint,
    pub index: c_int,
    pub direction: c_int,
    pub stream_tag: c_int,
    pub list: list_head,
    pub spib_addr: *mut c_void,
    pub fifo_addr: *mut c_void,
    pub dpibr_addr: *mut c_void,
    pub start_wallclk: c_uint,
    pub stripe: bool_,
    pub substream: *mut snd_pcm_substream,
    pub cstream: *mut snd_compr_stream,
    pub running: bool_,
    pub posbuf: *mut u32,
    pub bufsize: c_uint,
    pub period_bytes: c_uint,
    pub format_val: c_uint,
    pub frags: c_int,
    pub bdl: snd_dma_buffer,
    pub fifo_size: u16,
    pub delay_negative_threshold: c_int,
    pub period_wallclk: c_int,
    pub opened: c_int,
    pub assigned_key: c_int,
    pub no_period_wakeup: c_uint,
    pub cc: cyclecounter,
    pub tc: timecounter,
    pub locked: bool_,
}

unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) {
    (*new).prev = (*head).prev;
    (*new).next = head;
    (*(*head).prev).next = new;
    (*head).prev = new;
}

unsafe fn stream_list_first(bus: *mut hdac_bus) -> *mut list_head {
    (*bus).stream_list.next
}

unsafe fn stream_list_end(bus: *mut hdac_bus) -> *mut list_head {
    &mut (*bus).stream_list
}

unsafe fn hdac_stream_from_list(pos: *mut list_head) -> *mut hdac_stream {
    (pos as *mut u8).sub(core::mem::offset_of!(hdac_stream, list)) as *mut hdac_stream
}

fn upper_32_bits(n: dma_addr_t) -> u32 {
    (n >> 32) as u32
}

fn cpu_to_le32(n: u32) -> __le32 {
    n.to_le()
}

fn DIV_ROUND_UP(n: c_int, d: c_int) -> c_int {
    (n + d - 1) / d
}

fn roundup(x: c_int, y: c_int) -> c_int {
    DIV_ROUND_UP(x, y) * y
}

fn CLOCKSOURCE_MASK(bits: u32) -> u64 {
    if bits == 64 { !0 } else { (1u64 << bits) - 1 }
}

/*
 * the hdac_stream library is intended to be used with the following
 * transitions. The states are not formally defined in the code but loosely
 * inspired by boolean variables. Note that the 'prepared' field is not used
 * in this library but by the callers during the hw_params/prepare transitions
 *
 *                         |
 *      stream_init()      |
 *                         v
 *                      +--+-------+
 *                      |  unused  |
 *                      +--+----+--+
 *                         |    ^
 *      stream_assign()    |    |    stream_release()
 *                         v    |
 *                      +--+----+--+
 *                      |  opened  |
 *                      +--+----+--+
 *                         |    ^
 *      stream_reset()     |    |
 *      stream_setup()     |    |    stream_cleanup()
 *                         v    |
 *                      +--+----+--+
 *                      | prepared |
 *                      +--+----+--+
 *                         |    ^
 *      stream_start()     |    |    stream_stop()
 *                         v    |
 *                      +--+----+--+
 *                      |  running |
 *                      +----------+
 */

/**
 * snd_hdac_get_stream_stripe_ctl - get stripe control value
 * @bus: HD-audio core bus
 * @substream: PCM substream
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_get_stream_stripe_ctl(
    bus: *mut hdac_bus,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime = (*substream).runtime;
    let channels = (*runtime).channels;
    let rate = (*runtime).rate;
    let bits_per_sample = (*runtime).sample_bits;
    let mut value: c_uint;
    let mut sdo_line: c_uint;

    /* T_AZA_GCAP_NSDO is 1:2 bitfields in GCAP */
    let max_sdo_lines = snd_hdac_chip_readl(bus, GCAP) & AZX_GCAP_NSDO;

    /* following is from HD audio spec */
    sdo_line = max_sdo_lines;
    while sdo_line > 0 {
        if rate > 48000 {
            value = channels
                .wrapping_mul(bits_per_sample)
                .wrapping_mul(rate / 48000)
                / sdo_line;
        } else {
            value = channels.wrapping_mul(bits_per_sample) / sdo_line;
        }

        if value >= (*bus).sdo_limit {
            break;
        }
        sdo_line >>= 1;
    }

    /* stripe value: 0 for 1SDO, 1 for 2SDO, 2 for 4SDO lines */
    (sdo_line >> 1) as c_int
}

/**
 * snd_hdac_stream_init - initialize each stream (aka device)
 * @bus: HD-audio core bus
 * @azx_dev: HD-audio core stream object to initialize
 * @idx: stream index number
 * @direction: stream direction (SNDRV_PCM_STREAM_PLAYBACK or SNDRV_PCM_STREAM_CAPTURE)
 * @tag: the tag id to assign
 *
 * Assign the starting bdl address to each stream (device) and initialize.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_stream_init(
    bus: *mut hdac_bus,
    azx_dev: *mut hdac_stream,
    idx: c_int,
    direction: c_int,
    tag: c_int,
) {
    (*azx_dev).bus = bus;
    /* offset: SDI0=0x80, SDI1=0xa0, ... SDO3=0x160 */
    (*azx_dev).sd_addr = (*bus).remap_addr.add((0x20 * idx + 0x80) as usize);
    /* int mask: SDI0=0x01, SDI1=0x02, ... SDO3=0x80 */
    (*azx_dev).sd_int_sta_mask = 1u32 << idx;
    (*azx_dev).index = idx;
    (*azx_dev).direction = direction;
    (*azx_dev).stream_tag = tag;
    snd_hdac_dsp_lock_init(azx_dev);
    list_add_tail(&mut (*azx_dev).list, &mut (*bus).stream_list);

    if !(*bus).spbcap.is_null() {
        (*azx_dev).spib_addr = ((*bus).spbcap as *mut u8)
            .add(AZX_SPB_BASE + AZX_SPB_INTERVAL * idx as usize + AZX_SPB_SPIB)
            as *mut c_void;

        (*azx_dev).fifo_addr = ((*bus).spbcap as *mut u8)
            .add(AZX_SPB_BASE + AZX_SPB_INTERVAL * idx as usize + AZX_SPB_MAXFIFO)
            as *mut c_void;
    }

    if !(*bus).drsmcap.is_null() {
        (*azx_dev).dpibr_addr = ((*bus).drsmcap as *mut u8)
            .add(AZX_DRSM_BASE + AZX_DRSM_INTERVAL * idx as usize)
            as *mut c_void;
    }
}

/**
 * snd_hdac_stream_start - start a stream
 * @azx_dev: HD-audio core stream to start
 *
 * Start a stream, set start_wallclk and set the running flag.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_stream_start(azx_dev: *mut hdac_stream) {
    let bus = (*azx_dev).bus;
    let stripe_ctl: c_int;

    (*azx_dev).start_wallclk = snd_hdac_chip_readl(bus, WALLCLK);

    /* enable SIE */
    snd_hdac_chip_updatel(bus, INTCTL, 1u32 << (*azx_dev).index, 1u32 << (*azx_dev).index);
    /* set stripe control */
    if (*azx_dev).stripe {
        if !(*azx_dev).substream.is_null() {
            stripe_ctl = snd_hdac_get_stream_stripe_ctl(bus, (*azx_dev).substream);
        } else {
            stripe_ctl = 0;
        }
        if (*bus).access_sdnctl_in_dword {
            snd_hdac_stream_updatel(azx_dev, SD_CTL_3B, SD_CTL_STRIPE_MASK, stripe_ctl as c_uint);
        } else {
            snd_hdac_stream_updateb(azx_dev, SD_CTL_3B, SD_CTL_STRIPE_MASK, stripe_ctl as c_uint);
        }
    }
    /* set DMA start and interrupt mask */
    if (*bus).access_sdnctl_in_dword {
        snd_hdac_stream_updatel(azx_dev, SD_CTL, 0, SD_CTL_DMA_START | SD_INT_MASK);
    } else {
        snd_hdac_stream_updateb(azx_dev, SD_CTL, 0, SD_CTL_DMA_START | SD_INT_MASK);
    }
    (*azx_dev).running = true;
}

/**
 * snd_hdac_stream_clear - helper to clear stream registers and stop DMA transfers
 * @azx_dev: HD-audio core stream to stop
 */
unsafe fn snd_hdac_stream_clear(azx_dev: *mut hdac_stream) {
    let bus = (*azx_dev).bus;

    if (*bus).access_sdnctl_in_dword {
        snd_hdac_stream_updatel(azx_dev, SD_CTL, SD_CTL_DMA_START | SD_INT_MASK, 0);
        snd_hdac_stream_writeb(azx_dev, SD_STS, SD_INT_MASK); /* to be sure */
        if (*azx_dev).stripe {
            snd_hdac_stream_updatel(azx_dev, SD_CTL_3B, SD_CTL_STRIPE_MASK, 0);
        }
    } else {
        snd_hdac_stream_updateb(azx_dev, SD_CTL, SD_CTL_DMA_START | SD_INT_MASK, 0);
        snd_hdac_stream_writeb(azx_dev, SD_STS, SD_INT_MASK); /* to be sure */
        if (*azx_dev).stripe {
            snd_hdac_stream_updateb(azx_dev, SD_CTL_3B, SD_CTL_STRIPE_MASK, 0);
        }
    }

    (*azx_dev).running = false;
}

/**
 * snd_hdac_stream_stop - stop a stream
 * @azx_dev: HD-audio core stream to stop
 *
 * Stop a stream DMA and disable stream interrupt
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_stream_stop(azx_dev: *mut hdac_stream) {
    snd_hdac_stream_clear(azx_dev);
    /* disable SIE */
    snd_hdac_chip_updatel((*azx_dev).bus, INTCTL, 1u32 << (*azx_dev).index, 0);
}

/**
 * snd_hdac_stop_streams - stop all streams
 * @bus: HD-audio core bus
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_stop_streams(bus: *mut hdac_bus) {
    let mut pos = stream_list_first(bus);
    while pos != stream_list_end(bus) {
        let stream = hdac_stream_from_list(pos);
        pos = (*pos).next;
        snd_hdac_stream_stop(stream);
    }
}

/**
 * snd_hdac_stop_streams_and_chip - stop all streams and chip if running
 * @bus: HD-audio core bus
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_stop_streams_and_chip(bus: *mut hdac_bus) {
    if (*bus).chip_init {
        snd_hdac_stop_streams(bus);
        snd_hdac_bus_stop_chip(bus);
    }
}

/**
 * snd_hdac_stream_reset - reset a stream
 * @azx_dev: HD-audio core stream to reset
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_stream_reset(azx_dev: *mut hdac_stream) {
    let mut val: u8 = 0;
    let dma_run_state: c_int;
    let bus = (*azx_dev).bus;

    snd_hdac_stream_clear(azx_dev);

    dma_run_state = (snd_hdac_stream_readb(azx_dev, SD_CTL) & SD_CTL_DMA_START) as c_int;

    if (*bus).access_sdnctl_in_dword {
        snd_hdac_stream_updatel(azx_dev, SD_CTL, 0, SD_CTL_STREAM_RESET);
    } else {
        snd_hdac_stream_updateb(azx_dev, SD_CTL, 0, SD_CTL_STREAM_RESET);
    }

    /* wait for hardware to report that the stream entered reset */
    snd_hdac_stream_readb_poll(
        azx_dev,
        SD_CTL,
        &mut val,
        (val as c_uint & SD_CTL_STREAM_RESET != 0) as c_int,
        3,
        300,
    );

    if (*(*azx_dev).bus).dma_stop_delay != 0 && dma_run_state != 0 {
        udelay((*(*azx_dev).bus).dma_stop_delay);
    }

    if (*bus).access_sdnctl_in_dword {
        snd_hdac_stream_updatel(azx_dev, SD_CTL, SD_CTL_STREAM_RESET, 0);
    } else {
        snd_hdac_stream_updateb(azx_dev, SD_CTL, SD_CTL_STREAM_RESET, 0);
    }

    /* wait for hardware to report that the stream is out of reset */
    snd_hdac_stream_readb_poll(
        azx_dev,
        SD_CTL,
        &mut val,
        (!(val as c_uint & SD_CTL_STREAM_RESET != 0)) as c_int,
        3,
        300,
    );

    /* reset first position - may not be synced with hw at this time */
    if !(*azx_dev).posbuf.is_null() {
        *(*azx_dev).posbuf = 0;
    }
}

/**
 * snd_hdac_stream_setup -  set up the SD for streaming
 * @azx_dev: HD-audio core stream to set up
 * @code_loading: Whether the stream is for PCM or code-loading.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_stream_setup(azx_dev: *mut hdac_stream, code_loading: bool_) -> c_int {
    let bus = (*azx_dev).bus;
    let runtime: *mut snd_pcm_runtime;
    let mut val: c_uint;
    let mut reg: u16 = 0;
    let ret: c_int;

    if !(*azx_dev).substream.is_null() {
        runtime = (*(*azx_dev).substream).runtime;
    } else {
        runtime = ptr::null_mut();
    }
    /* make sure the run bit is zero for SD */
    snd_hdac_stream_clear(azx_dev);
    /* program the stream_tag */
    val = snd_hdac_stream_readl(azx_dev, SD_CTL);
    val = (val & !SD_CTL_STREAM_TAG_MASK)
        | (((*azx_dev).stream_tag as c_uint) << SD_CTL_STREAM_TAG_SHIFT);
    if !(*bus).snoop {
        val |= SD_CTL_TRAFFIC_PRIO;
    }
    snd_hdac_stream_writel(azx_dev, SD_CTL, val);

    /* program the length of samples in cyclic buffer */
    snd_hdac_stream_writel(azx_dev, SD_CBL, (*azx_dev).bufsize);

    /* program the stream format */
    /* this value needs to be the same as the one programmed */
    snd_hdac_stream_writew(azx_dev, SD_FORMAT, (*azx_dev).format_val);

    /* program the stream LVI (last valid index) of the BDL */
    snd_hdac_stream_writew(azx_dev, SD_LVI, ((*azx_dev).frags - 1) as c_uint);

    /* program the BDL address */
    /* lower BDL address */
    snd_hdac_stream_writel(azx_dev, SD_BDLPL, ((*azx_dev).bdl.addr + (*bus).addr_offset) as u32);
    /* upper BDL address */
    snd_hdac_stream_writel(
        azx_dev,
        SD_BDLPU,
        upper_32_bits((*azx_dev).bdl.addr + (*bus).addr_offset),
    );

    /* enable the position buffer */
    if (*bus).use_posbuf && (*bus).posbuf.addr != 0 {
        if (snd_hdac_chip_readl(bus, DPLBASE) & AZX_DPLBASE_ENABLE) == 0 {
            snd_hdac_chip_writel(
                bus,
                DPLBASE,
                ((*bus).posbuf.addr + (*bus).addr_offset) as u32 | AZX_DPLBASE_ENABLE,
            );
        }
    }

    /* set the interrupt enable bits in the descriptor control register */
    snd_hdac_stream_updatel(azx_dev, SD_CTL, 0, SD_INT_MASK);

    if !code_loading {
        /* Once SDxFMT is set, the controller programs SDxFIFOS to non-zero value. */
        ret = snd_hdac_stream_readw_poll(
            azx_dev,
            SD_FIFOSIZE,
            &mut reg,
            (reg & AZX_SD_FIFOSIZE_MASK != 0) as c_int,
            3,
            300,
        );
        if ret != 0 {
            dev_dbg(
                (*bus).dev,
                c"polling SD_FIFOSIZE 0x%04x failed: %d\n".as_ptr(),
                AZX_REG_SD_FIFOSIZE,
                ret,
            );
        }
        (*azx_dev).fifo_size = reg;
    }

    /* when LPIB delay correction gives a small negative value,
     * we ignore it; currently set the threshold statically to
     * 64 frames
     */
    if !runtime.is_null() && (*runtime).period_size > 64 {
        (*azx_dev).delay_negative_threshold = -frames_to_bytes(runtime, 64);
    } else {
        (*azx_dev).delay_negative_threshold = 0;
    }

    /* wallclk has 24Mhz clock source */
    if !runtime.is_null() {
        (*azx_dev).period_wallclk = ((((*runtime).period_size * 24000) / (*runtime).rate as c_int) * 1000) as c_int;
    }

    0
}

/**
 * snd_hdac_stream_cleanup - cleanup a stream
 * @azx_dev: HD-audio core stream to clean up
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_stream_cleanup(azx_dev: *mut hdac_stream) {
    snd_hdac_stream_writel(azx_dev, SD_BDLPL, 0);
    snd_hdac_stream_writel(azx_dev, SD_BDLPU, 0);
    snd_hdac_stream_writel(azx_dev, SD_CTL, 0);
    (*azx_dev).bufsize = 0;
    (*azx_dev).period_bytes = 0;
    (*azx_dev).format_val = 0;
}

/**
 * snd_hdac_stream_assign - assign a stream for the PCM
 * @bus: HD-audio core bus
 * @substream: PCM substream to assign
 *
 * Look for an unused stream for the given PCM substream, assign it
 * and return the stream object.  If no stream is free, returns NULL.
 * The function tries to keep using the same stream object when it's used
 * beforehand.  Also, when bus->reverse_assign flag is set, the last free
 * or matching entry is returned.  This is needed for some strange codecs.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_stream_assign(
    bus: *mut hdac_bus,
    substream: *mut snd_pcm_substream,
) -> *mut hdac_stream {
    let mut res: *mut hdac_stream = ptr::null_mut();

    /* make a non-zero unique key for the substream */
    let mut key: c_int = ((*substream).number << 2) | ((*substream).stream + 1);

    if !(*substream).pcm.is_null() {
        key |= (*(*substream).pcm).device << 16;
    }

    /* guard(spinlock_irq)(&bus->reg_lock); */
    let mut pos = stream_list_first(bus);
    while pos != stream_list_end(bus) {
        let azx_dev = hdac_stream_from_list(pos);
        pos = (*pos).next;
        if (*azx_dev).direction != (*substream).stream {
            continue;
        }
        if (*azx_dev).opened != 0 {
            continue;
        }
        if (*azx_dev).assigned_key == key {
            res = azx_dev;
            break;
        }
        if res.is_null() || (*bus).reverse_assign {
            res = azx_dev;
        }
    }
    if !res.is_null() {
        (*res).opened = 1;
        (*res).running = false;
        (*res).assigned_key = key;
        (*res).substream = substream;
    }
    res
}

/**
 * snd_hdac_stream_release_locked - release the assigned stream
 * @azx_dev: HD-audio core stream to release
 *
 * Release the stream that has been assigned by snd_hdac_stream_assign().
 * The bus->reg_lock needs to be taken at a higher level
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_stream_release_locked(azx_dev: *mut hdac_stream) {
    (*azx_dev).opened = 0;
    (*azx_dev).running = false;
    (*azx_dev).substream = ptr::null_mut();
}

/**
 * snd_hdac_stream_release - release the assigned stream
 * @azx_dev: HD-audio core stream to release
 *
 * Release the stream that has been assigned by snd_hdac_stream_assign().
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_stream_release(azx_dev: *mut hdac_stream) {
    let _bus = (*azx_dev).bus;

    /* guard(spinlock_irq)(&bus->reg_lock); */
    snd_hdac_stream_release_locked(azx_dev);
}

/**
 * snd_hdac_get_stream - return hdac_stream based on stream_tag and
 * direction
 *
 * @bus: HD-audio core bus
 * @dir: direction for the stream to be found
 * @stream_tag: stream tag for stream to be found
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_get_stream(
    bus: *mut hdac_bus,
    dir: c_int,
    stream_tag: c_int,
) -> *mut hdac_stream {
    let mut pos = stream_list_first(bus);
    while pos != stream_list_end(bus) {
        let s = hdac_stream_from_list(pos);
        pos = (*pos).next;
        if (*s).direction == dir && (*s).stream_tag == stream_tag {
            return s;
        }
    }

    ptr::null_mut()
}

/*
 * set up a BDL entry
 */
unsafe fn setup_bdle(
    bus: *mut hdac_bus,
    dmab: *mut snd_dma_buffer,
    azx_dev: *mut hdac_stream,
    bdlp: *mut *mut __le32,
    mut ofs: c_int,
    mut size: c_int,
    with_ioc: c_int,
) -> c_int {
    let mut bdl = *bdlp;

    while size > 0 {
        let addr: dma_addr_t;
        let mut chunk: c_int;

        if (*azx_dev).frags >= AZX_MAX_BDL_ENTRIES {
            return -EINVAL;
        }

        addr = snd_sgbuf_get_addr(dmab, ofs);
        /* program the address field of the BDL entry */
        *bdl.add(0) = cpu_to_le32((addr + (*bus).addr_offset) as u32);
        *bdl.add(1) = cpu_to_le32(upper_32_bits(addr + (*bus).addr_offset));
        /* program the size field of the BDL entry */
        chunk = snd_sgbuf_get_chunk_size(dmab, ofs, size);
        /* one BDLE cannot cross 4K boundary on CTHDA chips */
        if (*bus).align_bdle_4k {
            let remain: u32 = 0x1000 - (ofs as u32 & 0xfff);

            if chunk > remain as c_int {
                chunk = remain as c_int;
            }
        }
        *bdl.add(2) = cpu_to_le32(chunk as u32);
        /* program the IOC to enable interrupt
         * only when the whole fragment is processed
         */
        size -= chunk;
        *bdl.add(3) = if size != 0 || with_ioc == 0 { 0 } else { cpu_to_le32(0x01) };
        bdl = bdl.add(4);
        (*azx_dev).frags += 1;
        ofs += chunk;
    }
    *bdlp = bdl;
    ofs
}

/**
 * snd_hdac_stream_setup_bdle - set up BDL entries
 * @azx_dev: HD-audio core stream to set up
 * @dmab: allocated DMA buffer
 * @runtime: substream runtime, optional
 *
 * Set up the buffer descriptor table of the given stream based on the
 * period and buffer sizes of the assigned PCM substream.
 */
unsafe fn snd_hdac_stream_setup_bdle(
    azx_dev: *mut hdac_stream,
    dmab: *mut snd_dma_buffer,
    runtime: *mut snd_pcm_runtime,
) -> c_int {
    let bus = (*azx_dev).bus;
    let mut i: c_int;
    let mut ofs: c_int;
    let periods: c_int;
    let period_bytes: c_int;
    let mut pos_adj: c_int;
    let pos_align: c_int;
    let mut bdl: *mut __le32;

    /* reset BDL address */
    snd_hdac_stream_writel(azx_dev, SD_BDLPL, 0);
    snd_hdac_stream_writel(azx_dev, SD_BDLPU, 0);

    period_bytes = (*azx_dev).period_bytes as c_int;
    periods = (*azx_dev).bufsize as c_int / period_bytes;

    /* program the initial BDL entries */
    bdl = (*azx_dev).bdl.area as *mut __le32;
    ofs = 0;
    (*azx_dev).frags = 0;

    pos_adj = (*bus).bdl_pos_adj;
    if !runtime.is_null() && (*azx_dev).no_period_wakeup == 0 && pos_adj > 0 {
        pos_align = pos_adj;
        pos_adj = DIV_ROUND_UP(pos_adj * (*runtime).rate as c_int, 48000);
        if pos_adj == 0 {
            pos_adj = pos_align;
        } else {
            pos_adj = roundup(pos_adj, pos_align);
        }
        pos_adj = frames_to_bytes(runtime, pos_adj);
        if pos_adj >= period_bytes {
            dev_warn((*bus).dev, c"Too big adjustment %d\n".as_ptr(), pos_adj);
            pos_adj = 0;
        } else {
            ofs = setup_bdle(bus, dmab, azx_dev, &mut bdl, ofs, pos_adj, true as c_int);
            if ofs < 0 {
                return {
                    dev_dbg(
                        (*bus).dev,
                        c"Too many BDL entries: buffer=%d, period=%d\n".as_ptr(),
                        (*azx_dev).bufsize,
                        period_bytes,
                    );
                    -EINVAL
                };
            }
        }
    } else {
        pos_adj = 0;
    }

    i = 0;
    while i < periods {
        if i == periods - 1 && pos_adj != 0 {
            ofs = setup_bdle(bus, dmab, azx_dev, &mut bdl, ofs, period_bytes - pos_adj, 0);
        } else {
            ofs = setup_bdle(
                bus,
                dmab,
                azx_dev,
                &mut bdl,
                ofs,
                period_bytes,
                ((*azx_dev).no_period_wakeup == 0) as c_int,
            );
        }
        if ofs < 0 {
            dev_dbg(
                (*bus).dev,
                c"Too many BDL entries: buffer=%d, period=%d\n".as_ptr(),
                (*azx_dev).bufsize,
                period_bytes,
            );
            return -EINVAL;
        }
        i += 1;
    }
    0
}

/**
 * snd_hdac_stream_setup_periods - set up BDL entries
 * @azx_dev: HD-audio core stream to set up
 *
 * Set up the buffer descriptor table of the given stream based on the
 * period and buffer sizes of the assigned PCM substream.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_stream_setup_periods(azx_dev: *mut hdac_stream) -> c_int {
    let substream = (*azx_dev).substream;
    let cstream = (*azx_dev).cstream;
    let mut runtime: *mut snd_pcm_runtime = ptr::null_mut();
    let dmab: *mut snd_dma_buffer;

    if !substream.is_null() {
        runtime = (*substream).runtime;
        dmab = snd_pcm_get_dma_buf_pcm(substream);
    } else if !cstream.is_null() {
        dmab = snd_pcm_get_dma_buf_compr(cstream);
    } else {
        /* WARN(1, "No substream or cstream assigned\n"); */
        return -EINVAL;
    }

    snd_hdac_stream_setup_bdle(azx_dev, dmab, runtime)
}

/**
 * snd_hdac_stream_set_params - set stream parameters
 * @azx_dev: HD-audio core stream for which parameters are to be set
 * @format_val: format value parameter
 *
 * Setup the HD-audio core stream parameters from substream of the stream
 * and passed format value
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_stream_set_params(
    azx_dev: *mut hdac_stream,
    format_val: c_uint,
) -> c_int {
    let substream = (*azx_dev).substream;
    let cstream = (*azx_dev).cstream;
    let bufsize: c_uint;
    let period_bytes: c_uint;
    let no_period_wakeup: c_uint;
    let err: c_int;

    if !substream.is_null() {
        bufsize = snd_pcm_lib_buffer_bytes(substream);
        period_bytes = snd_pcm_lib_period_bytes(substream);
        no_period_wakeup = (*(*substream).runtime).no_period_wakeup;
    } else if !cstream.is_null() {
        bufsize = (*(*cstream).runtime).buffer_size;
        period_bytes = (*(*cstream).runtime).fragment_size;
        no_period_wakeup = 0;
    } else {
        return -EINVAL;
    }

    if bufsize != (*azx_dev).bufsize
        || period_bytes != (*azx_dev).period_bytes
        || format_val != (*azx_dev).format_val
        || no_period_wakeup != (*azx_dev).no_period_wakeup
    {
        (*azx_dev).bufsize = bufsize;
        (*azx_dev).period_bytes = period_bytes;
        (*azx_dev).format_val = format_val;
        (*azx_dev).no_period_wakeup = no_period_wakeup;
        err = snd_hdac_stream_setup_periods(azx_dev);
        if err < 0 {
            return err;
        }
    }
    0
}

unsafe extern "C" fn azx_cc_read(cc: *mut cyclecounter) -> u64 {
    let azx_dev = (cc as *mut u8).sub(core::mem::offset_of!(hdac_stream, cc)) as *mut hdac_stream;

    snd_hdac_chip_readl((*azx_dev).bus, WALLCLK) as u64
}

unsafe fn azx_timecounter_init(azx_dev: *mut hdac_stream, force: bool_, last: u64) {
    let tc = &mut (*azx_dev).tc;
    let cc = &mut (*azx_dev).cc;
    let nsec: u64;

    cc.read = Some(azx_cc_read);
    cc.mask = CLOCKSOURCE_MASK(32);

    /*
     * Calculate the optimal mult/shift values. The counter wraps
     * around after ~178.9 seconds.
     */
    clocks_calc_mult_shift(&mut cc.mult, &mut cc.shift, 24000000, NSEC_PER_SEC, 178);

    nsec = 0; /* audio time is elapsed time since trigger */
    timecounter_init(tc, cc, nsec);
    if force {
        /*
         * force timecounter to use predefined value,
         * used for synchronized starts
         */
        tc.cycle_last = last;
    }
}

/**
 * snd_hdac_stream_timecounter_init - initialize time counter
 * @azx_dev: HD-audio core stream (master stream)
 * @streams: bit flags of streams to set up
 * @start: true for PCM trigger start, false for other cases
 *
 * Initializes the time counter of streams marked by the bit flags (each
 * bit corresponds to the stream index).
 * The trigger timestamp of PCM substream assigned to the given stream is
 * updated accordingly, too.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_stream_timecounter_init(
    azx_dev: *mut hdac_stream,
    streams: c_uint,
    start: bool_,
) {
    let bus = (*azx_dev).bus;
    let runtime = (*(*azx_dev).substream).runtime;
    let mut inited = false;
    let mut cycle_last: u64 = 0;

    if start {
        let mut pos = stream_list_first(bus);
        while pos != stream_list_end(bus) {
            let s = hdac_stream_from_list(pos);
            pos = (*pos).next;
            if streams & (1u32 << (*s).index) != 0 {
                azx_timecounter_init(s, inited, cycle_last);
                if !inited {
                    inited = true;
                    cycle_last = (*s).tc.cycle_last;
                }
            }
        }
    }

    snd_pcm_gettime(runtime, &mut (*runtime).trigger_tstamp);
    (*runtime).trigger_tstamp_latched = true;
}

/**
 * snd_hdac_stream_sync_trigger - turn on/off stream sync register
 * @azx_dev: HD-audio core stream (master stream)
 * @set: true = set, false = clear
 * @streams: bit flags of streams to sync
 * @reg: the stream sync register address
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_stream_sync_trigger(
    azx_dev: *mut hdac_stream,
    set: bool_,
    streams: c_uint,
    mut reg: c_uint,
) {
    let bus = (*azx_dev).bus;
    let mut val: c_uint;

    if reg == 0 {
        reg = AZX_REG_SSYNC;
    }
    val = _snd_hdac_chip_readl(bus, reg);
    if set {
        val |= streams;
    } else {
        val &= !streams;
    }
    _snd_hdac_chip_writel(bus, reg, val);
}

/**
 * snd_hdac_stream_sync - sync with start/stop trigger operation
 * @azx_dev: HD-audio core stream (master stream)
 * @start: true = start, false = stop
 * @streams: bit flags of streams to sync
 *
 * For @start = true, wait until all FIFOs get ready.
 * For @start = false, wait until all RUN bits are cleared.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_stream_sync(
    azx_dev: *mut hdac_stream,
    start: bool_,
    streams: c_uint,
) {
    let bus = (*azx_dev).bus;
    let mut nwait: c_int;
    let mut timeout: c_int;

    timeout = 5000;
    while timeout != 0 {
        nwait = 0;
        let mut pos = stream_list_first(bus);
        while pos != stream_list_end(bus) {
            let s = hdac_stream_from_list(pos);
            pos = (*pos).next;
            if streams & (1u32 << (*s).index) == 0 {
                continue;
            }

            if start {
                /* check FIFO gets ready */
                if snd_hdac_stream_readb(s, SD_STS) & SD_STS_FIFO_READY == 0 {
                    nwait += 1;
                }
            } else {
                /* check RUN bit is cleared */
                if snd_hdac_stream_readb(s, SD_CTL) & SD_CTL_DMA_START != 0 {
                    nwait += 1;
                    /*
                     * Perform stream reset if DMA RUN
                     * bit not cleared within given timeout
                     */
                    if timeout == 1 {
                        snd_hdac_stream_reset(s);
                    }
                }
            }
        }
        if nwait == 0 {
            break;
        }
        cpu_relax();
        timeout -= 1;
    }
}

/**
 * snd_hdac_stream_spbcap_enable - enable SPIB for a stream
 * @bus: HD-audio core bus
 * @enable: flag to enable/disable SPIB
 * @index: stream index for which SPIB need to be enabled
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_stream_spbcap_enable(
    bus: *mut hdac_bus,
    enable: bool_,
    index: c_int,
) {
    let mut mask: u32 = 0;

    if (*bus).spbcap.is_null() {
        dev_err((*bus).dev, c"Address of SPB capability is NULL\n".as_ptr());
        return;
    }

    mask |= 1u32 << index;

    if enable {
        snd_hdac_updatel((*bus).spbcap, AZX_REG_SPB_SPBFCCTL, mask, mask);
    } else {
        snd_hdac_updatel((*bus).spbcap, AZX_REG_SPB_SPBFCCTL, mask, 0);
    }
}

/**
 * snd_hdac_stream_set_spib - sets the spib value of a stream
 * @bus: HD-audio core bus
 * @azx_dev: hdac_stream
 * @value: spib value to set
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_stream_set_spib(
    bus: *mut hdac_bus,
    azx_dev: *mut hdac_stream,
    value: u32,
) -> c_int {
    if (*bus).spbcap.is_null() {
        dev_err((*bus).dev, c"Address of SPB capability is NULL\n".as_ptr());
        return -EINVAL;
    }

    writel(value, (*azx_dev).spib_addr);

    0
}

/**
 * snd_hdac_stream_drsm_enable - enable DMA resume for a stream
 * @bus: HD-audio core bus
 * @enable: flag to enable/disable DRSM
 * @index: stream index for which DRSM need to be enabled
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_stream_drsm_enable(
    bus: *mut hdac_bus,
    enable: bool_,
    index: c_int,
) {
    let mut mask: u32 = 0;

    if (*bus).drsmcap.is_null() {
        dev_err((*bus).dev, c"Address of DRSM capability is NULL\n".as_ptr());
        return;
    }

    mask |= 1u32 << index;

    if enable {
        snd_hdac_updatel((*bus).drsmcap, AZX_REG_DRSM_CTL, mask, mask);
    } else {
        snd_hdac_updatel((*bus).drsmcap, AZX_REG_DRSM_CTL, mask, 0);
    }
}

/*
 * snd_hdac_stream_wait_drsm - wait for HW to clear RSM for a stream
 * @azx_dev: HD-audio core stream to await RSM for
 *
 * Returns 0 on success and -ETIMEDOUT upon a timeout.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_stream_wait_drsm(azx_dev: *mut hdac_stream) -> c_int {
    let bus = (*azx_dev).bus;
    let mask: u32;
    let mut reg: u32;
    let mut ret: c_int = 0;
    let mut timeout = 2000;

    mask = 1u32 << (*azx_dev).index;

    loop {
        reg = snd_hdac_reg_readl(bus, ((*bus).drsmcap as *mut u8).add(AZX_REG_DRSM_CTL as usize) as *mut c_void);
        if reg & mask == 0 {
            break;
        }
        if timeout == 0 {
            ret = -1;
            break;
        }
        udelay(250);
        timeout -= 250;
    }
    if ret != 0 {
        dev_dbg((*bus).dev, c"polling RSM 0x%08x failed: %d\n".as_ptr(), mask, ret);
    }
    ret
}

/**
 * snd_hdac_stream_set_dpibr - sets the dpibr value of a stream
 * @bus: HD-audio core bus
 * @azx_dev: hdac_stream
 * @value: dpib value to set
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_stream_set_dpibr(
    bus: *mut hdac_bus,
    azx_dev: *mut hdac_stream,
    value: u32,
) -> c_int {
    if (*bus).drsmcap.is_null() {
        dev_err((*bus).dev, c"Address of DRSM capability is NULL\n".as_ptr());
        return -EINVAL;
    }

    writel(value, (*azx_dev).dpibr_addr);

    0
}

/**
 * snd_hdac_stream_set_lpib - sets the lpib value of a stream
 * @azx_dev: hdac_stream
 * @value: lpib value to set
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_stream_set_lpib(azx_dev: *mut hdac_stream, value: u32) -> c_int {
    snd_hdac_stream_writel(azx_dev, SD_LPIB, value);

    0
}

extern "C" {
    static SD_LPIB: c_uint;
}

/* CONFIG_SND_HDA_DSP_LOADER */
/**
 * snd_hdac_dsp_prepare - prepare for DSP loading
 * @azx_dev: HD-audio core stream used for DSP loading
 * @format: HD-audio stream format
 * @byte_size: data chunk byte size
 * @bufp: allocated buffer
 *
 * Allocate the buffer for the given size and set up the given stream for
 * DSP loading.  Returns the stream tag (>= 0), or a negative error code.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_dsp_prepare(
    azx_dev: *mut hdac_stream,
    format: c_uint,
    byte_size: c_uint,
    bufp: *mut snd_dma_buffer,
) -> c_int {
    let bus = (*azx_dev).bus;
    let mut err: c_int;

    /* guard(snd_hdac_dsp_lock)(azx_dev); */
    /* scoped_guard(spinlock_irq, &bus->reg_lock) */
    {
        if (*azx_dev).running || (*azx_dev).locked {
            return -EBUSY;
        }
        (*azx_dev).locked = true;
    }

    err = snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV_SG, (*bus).dev, byte_size, bufp);
    if err < 0 {
        (*azx_dev).locked = false;
        return err;
    }

    (*azx_dev).substream = ptr::null_mut();
    (*azx_dev).bufsize = byte_size;
    /* It is recommended to transfer the firmware in two or more chunks. */
    (*azx_dev).period_bytes = byte_size / 2;
    (*azx_dev).format_val = format;
    (*azx_dev).no_period_wakeup = 1;

    snd_hdac_stream_reset(azx_dev);

    err = snd_hdac_stream_setup_bdle(azx_dev, bufp, ptr::null_mut());
    if err < 0 {
        snd_dma_free_pages(bufp);
        (*azx_dev).locked = false;
        return err;
    }

    snd_hdac_stream_setup(azx_dev, true);
    (*azx_dev).stream_tag
}

/**
 * snd_hdac_dsp_trigger - start / stop DSP loading
 * @azx_dev: HD-audio core stream used for DSP loading
 * @start: trigger start or stop
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_dsp_trigger(azx_dev: *mut hdac_stream, start: bool_) {
    if start {
        snd_hdac_stream_start(azx_dev);
    } else {
        snd_hdac_stream_stop(azx_dev);
    }
}

/**
 * snd_hdac_dsp_cleanup - clean up the stream from DSP loading to normal
 * @azx_dev: HD-audio core stream used for DSP loading
 * @dmab: buffer used by DSP loading
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_dsp_cleanup(
    azx_dev: *mut hdac_stream,
    dmab: *mut snd_dma_buffer,
) {
    let bus = (*azx_dev).bus;

    if (*dmab).area.is_null() || !(*azx_dev).locked {
        return;
    }

    /* guard(snd_hdac_dsp_lock)(azx_dev); */
    /* reset BDL address */
    snd_hdac_stream_writel(azx_dev, SD_BDLPL, 0);
    snd_hdac_stream_writel(azx_dev, SD_BDLPU, 0);
    snd_hdac_stream_writel(azx_dev, SD_CTL, 0);
    (*azx_dev).bufsize = 0;
    (*azx_dev).period_bytes = 0;
    (*azx_dev).format_val = 0;

    snd_dma_free_pages(dmab);
    (*dmab).area = ptr::null_mut();

    /* guard(spinlock_irq)(&bus->reg_lock); */
    let _ = bus;
    (*azx_dev).locked = false;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
