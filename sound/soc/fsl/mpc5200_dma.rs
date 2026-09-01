// SPDX-License-Identifier: GPL-2.0-only
//
// Freescale MPC5200 PSC DMA
// ALSA SoC Platform driver
//
// Copyright (C) 2008 Secret Lab Technologies Ltd.
// Copyright (C) 2009 Jon Smirl, Digispeaker

// C dependencies translated as external Rust dependencies:
// linux/module.h, linux/dma-mapping.h, linux/slab.h, linux/of.h,
// linux/of_address.h, linux/of_irq.h, linux/platform_device.h,
// sound/soc.h, linux/fsl/bestcomm/bestcomm.h,
// linux/fsl/bestcomm/gen_bd.h, asm/mpc52xx_psc.h, mpc5200_dma.h

use core::ffi::{c_int, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

const DRV_NAME: &[u8] = b"mpc5200_dma\0";

extern "C" {
    fn in_be16(addr: *const u16) -> u16;
    fn out_be16(addr: *mut u16, value: u16);
    fn out_8(addr: *mut u8, value: u8);

    fn bcom_prepare_next_buffer(task: *mut bcom_task) -> *mut bcom_bd;
    fn bcom_submit_next_buffer(task: *mut bcom_task, cookie: *mut c_void);
    fn bcom_buffer_done(task: *mut bcom_task) -> c_int;
    fn bcom_retrieve_buffer(task: *mut bcom_task, p1: *mut c_void, p2: *mut c_void);
    fn bcom_gen_bd_rx_reset(task: *mut bcom_task);
    fn bcom_gen_bd_tx_reset(task: *mut bcom_task);
    fn bcom_queue_full(task: *mut bcom_task) -> c_int;
    fn bcom_enable(task: *mut bcom_task);
    fn bcom_disable(task: *mut bcom_task);
    fn bcom_psc_gen_bd_rx_init(id: c_int, queue_len: c_int, fifo: phys_addr_t, maxbuf: c_int) -> *mut bcom_task;
    fn bcom_psc_gen_bd_tx_init(id: c_int, queue_len: c_int, fifo: phys_addr_t) -> *mut bcom_task;
    fn bcom_get_task_irq(task: *mut bcom_task) -> c_int;
    fn bcom_gen_bd_rx_release(task: *mut bcom_task);
    fn bcom_gen_bd_tx_release(task: *mut bcom_task);

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: snd_pcm_uframes_t) -> usize;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: dma_addr_t) -> snd_pcm_uframes_t;
    fn snd_soc_set_runtime_hwparams(substream: *mut snd_pcm_substream, hw: *const snd_pcm_hardware);
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_int) -> c_int;
    fn snd_pcm_set_fixed_buffer_all(
        pcm: *mut snd_pcm,
        typ: c_int,
        dev: *mut device,
        size: usize,
    ) -> c_int;

    fn dma_coerce_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn devm_platform_get_and_ioremap_resource(
        op: *mut platform_device,
        index: c_int,
        res: *mut *mut resource,
    ) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn platform_get_irq(op: *mut platform_device, num: c_int) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn of_get_property(node: *mut device_node, name: *const u8, lenp: *mut c_int) -> *const __be32;
    fn be32_to_cpu(value: __be32) -> u32;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn mutex_init(mutex: *mut mutex);
    fn snprintf(buf: *mut u8, size: usize, fmt: *const u8, ...) -> c_int;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        flags: c_ulong,
        name: *const u8,
        dev_id: *mut c_void,
    ) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut c_void,
        num_dai: c_int,
    ) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const u8, ...);
    fn dev_err(dev: *mut device, fmt: *const u8, ...);

    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
}

type c_ulong = usize;
type phys_addr_t = usize;
type dma_addr_t = usize;
type snd_pcm_uframes_t = usize;
type irqreturn_t = c_int;
type __be32 = u32;

const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_SHARED: c_ulong = 0x00000080;
const GFP_KERNEL: c_int = 0;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_HW_PARAM_PERIODS: c_int = 12;
const SNDRV_DMA_TYPE_DEV: c_int = 1;

const SNDRV_PCM_INFO_MMAP: u32 = 0x00000001;
const SNDRV_PCM_INFO_MMAP_VALID: u32 = 0x00000002;
const SNDRV_PCM_INFO_INTERLEAVED: u32 = 0x00000100;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: u32 = 0x00010000;
const SNDRV_PCM_INFO_BATCH: u32 = 0x00020000;
const SNDRV_PCM_FMTBIT_S8: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_BE: u64 = 1 << 3;
const SNDRV_PCM_FMTBIT_S24_BE: u64 = 1 << 7;
const SNDRV_PCM_FMTBIT_S32_BE: u64 = 1 << 11;

const MPC52xx_PSC_IMR_TXEMP: u16 = 0x0800;
const MPC52xx_PSC_IMR_ORERR: u16 = 0x0001;
const MPC52xx_PSC_RST_ERR_STAT: u8 = 4 << 4;
const MPC52xx_PSC_RST_RX: u8 = 2 << 4;
const MPC52xx_PSC_RST_TX: u8 = 3 << 4;
const MPC52xx_PSC_SEL_MODE_REG_1: u8 = 1;

const fn DMA_BIT_MASK(nr: u32) -> u64 {
    if nr == 64 {
        !0
    } else {
        (1u64 << nr) - 1
    }
}

#[repr(C)]
pub struct bcom_task {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bcom_bd {
    pub status: usize,
    pub data: [dma_addr_t; 1],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct resource {
    pub start: phys_addr_t,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_card_wrapper {
    pub snd_card: *mut snd_card,
}

#[repr(C)]
pub struct snd_pcm {
    pub card: *mut snd_card_wrapper,
}

#[repr(C)]
pub struct snd_pcm_str {
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub frame_bits: u32,
    pub period_size: snd_pcm_uframes_t,
    pub periods: u32,
    pub dma_addr: dma_addr_t,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub pstr: *mut snd_pcm_str,
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_card_wrapper,
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: u32,
    pub formats: u64,
    pub period_bytes_max: usize,
    pub period_bytes_min: usize,
    pub periods_min: u32,
    pub periods_max: u32,
    pub buffer_bytes_max: usize,
    pub fifo_size: u32,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const u8,
    pub open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int) -> c_int>,
    pub pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int>,
}

#[repr(C)]
pub struct mpc52xx_psc_isr_imr {
    pub imr: u16,
}

#[repr(C)]
pub struct mpc52xx_psc_buffer {
    pub buffer_32: u32,
}

#[repr(C)]
pub struct mpc52xx_psc {
    pub mpc52xx_psc_isr: u16,
    pub command: u8,
    pub isr_imr: mpc52xx_psc_isr_imr,
    pub mode: u8,
    pub buffer: mpc52xx_psc_buffer,
}

#[repr(C)]
pub struct mpc52xx_psc_fifo {
    pub rfalarm: u16,
    pub rfcntl: u8,
    pub tfalarm: u16,
    pub tfcntl: u8,
}

#[repr(C)]
pub struct psc_dma_stats {
    pub underrun_count: u32,
    pub overrun_count: u32,
}

#[repr(C)]
pub struct psc_dma_stream {
    pub psc_dma: *mut psc_dma,
    pub bcom_task: *mut bcom_task,
    pub irq: c_int,
    pub stream: *mut snd_pcm_substream,
    pub runtime: *mut snd_pcm_runtime,
    pub period_bytes: usize,
    pub period_next: u32,
    pub period_current: u32,
    pub period_count: c_int,
    pub active: c_int,
}

#[repr(C)]
pub struct psc_dma {
    pub lock: spinlock_t,
    pub mutex: mutex,
    pub id: u32,
    pub irq: c_int,
    pub imr: u16,
    pub psc_regs: *mut mpc52xx_psc,
    pub fifo_regs: *mut mpc52xx_psc_fifo,
    pub dev: *mut device,
    pub playback: psc_dma_stream,
    pub capture: psc_dma_stream,
    pub stats: psc_dma_stats,
    pub name: [u8; 32],
}

unsafe fn to_psc_dma_stream(
    substream: *mut snd_pcm_substream,
    psc_dma: *mut psc_dma,
) -> *mut psc_dma_stream {
    if (*(*substream).pstr).stream == SNDRV_PCM_STREAM_CAPTURE {
        &mut (*psc_dma).capture
    } else {
        &mut (*psc_dma).playback
    }
}

/*
 * Interrupt handlers
 */
unsafe extern "C" fn psc_dma_status_irq(_irq: c_int, _psc_dma: *mut c_void) -> irqreturn_t {
    let psc_dma = _psc_dma as *mut psc_dma;
    let regs: *mut mpc52xx_psc = (*psc_dma).psc_regs;
    let isr: u16;

    isr = in_be16(&(*regs).mpc52xx_psc_isr);

    /* Playback underrun error */
    if (*psc_dma).playback.active != 0 && (isr & MPC52xx_PSC_IMR_TXEMP) != 0 {
        (*psc_dma).stats.underrun_count = (*psc_dma).stats.underrun_count.wrapping_add(1);
    }

    /* Capture overrun error */
    if (*psc_dma).capture.active != 0 && (isr & MPC52xx_PSC_IMR_ORERR) != 0 {
        (*psc_dma).stats.overrun_count = (*psc_dma).stats.overrun_count.wrapping_add(1);
    }

    out_8(&mut (*regs).command, MPC52xx_PSC_RST_ERR_STAT);

    IRQ_HANDLED
}

/**
 * psc_dma_bcom_enqueue_next_buffer - Enqueue another audio buffer
 * @s: pointer to stream private data structure
 *
 * Enqueues another audio period buffer into the bestcomm queue.
 *
 * Note: The routine must only be called when there is space available in
 * the queue.  Otherwise the enqueue will fail and the audio ring buffer
 * will get out of sync
 */
unsafe fn psc_dma_bcom_enqueue_next_buffer(s: *mut psc_dma_stream) {
    let bd: *mut bcom_bd;

    /* Prepare and enqueue the next buffer descriptor */
    bd = bcom_prepare_next_buffer((*s).bcom_task);
    (*bd).status = (*s).period_bytes;
    (*bd).data[0] = (*(*s).runtime).dma_addr
        .wrapping_add(((*s).period_next as usize).wrapping_mul((*s).period_bytes));
    bcom_submit_next_buffer((*s).bcom_task, ptr::null_mut());

    /* Update for next period */
    (*s).period_next = ((*s).period_next + 1) % (*(*s).runtime).periods;
}

/* Bestcomm DMA irq handler */
unsafe extern "C" fn psc_dma_bcom_irq(_irq: c_int, _psc_dma_stream: *mut c_void) -> irqreturn_t {
    let s = _psc_dma_stream as *mut psc_dma_stream;

    spin_lock(&mut (*(*s).psc_dma).lock);
    /*
     * For each finished period, dequeue the completed period buffer
     * and enqueue a new one in its place
     */
    while bcom_buffer_done((*s).bcom_task) != 0 {
        bcom_retrieve_buffer((*s).bcom_task, ptr::null_mut(), ptr::null_mut());

        (*s).period_current = ((*s).period_current + 1) % (*(*s).runtime).periods;
        (*s).period_count += 1;

        psc_dma_bcom_enqueue_next_buffer(s);
    }
    spin_unlock(&mut (*(*s).psc_dma).lock);

    /* If the stream is active, then also inform the PCM middle layer
     * of the period finished event. */
    if (*s).active != 0 {
        snd_pcm_period_elapsed((*s).stream);
    }

    IRQ_HANDLED
}

/**
 * psc_dma_trigger: start and stop the DMA transfer.
 * @component: triggered component
 * @substream: triggered substream
 * @cmd: triggered command
 *
 * This function is called by ALSA to start, stop, pause, and resume the DMA
 * transfer of data.
 */
unsafe extern "C" fn psc_dma_trigger(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let psc_dma = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut psc_dma;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let s: *mut psc_dma_stream = to_psc_dma_stream(substream, psc_dma);
    let regs: *mut mpc52xx_psc = (*psc_dma).psc_regs;
    let mut imr: u16;
    let mut i: c_int;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            dev_dbg(
                (*psc_dma).dev,
                b"START: stream=%i fbits=%u ps=%u #p=%u\n\0".as_ptr(),
                (*(*substream).pstr).stream,
                (*runtime).frame_bits,
                (*runtime).period_size as c_int,
                (*runtime).periods,
            );
            (*s).period_bytes = frames_to_bytes(runtime, (*runtime).period_size);
            (*s).period_next = 0;
            (*s).period_current = 0;
            (*s).active = 1;
            (*s).period_count = 0;
            (*s).runtime = runtime;

            /* Fill up the bestcomm bd queue and enable DMA.
             * This will begin filling the PSC's fifo.
             */
            let mut flags: c_ulong = 0;
            spin_lock_irqsave(&mut (*psc_dma).lock, &mut flags);
            if (*(*substream).pstr).stream == SNDRV_PCM_STREAM_CAPTURE {
                bcom_gen_bd_rx_reset((*s).bcom_task);
            } else {
                bcom_gen_bd_tx_reset((*s).bcom_task);
            }

            i = 0;
            while i < (*runtime).periods as c_int {
                if bcom_queue_full((*s).bcom_task) == 0 {
                    psc_dma_bcom_enqueue_next_buffer(s);
                }
                i += 1;
            }

            bcom_enable((*s).bcom_task);
            spin_unlock_irqrestore(&mut (*psc_dma).lock, flags);

            out_8(&mut (*regs).command, MPC52xx_PSC_RST_ERR_STAT);
        }

        SNDRV_PCM_TRIGGER_STOP => {
            dev_dbg(
                (*psc_dma).dev,
                b"STOP: stream=%i periods_count=%i\n\0".as_ptr(),
                (*(*substream).pstr).stream,
                (*s).period_count,
            );
            (*s).active = 0;

            let mut flags: c_ulong = 0;
            spin_lock_irqsave(&mut (*psc_dma).lock, &mut flags);
            bcom_disable((*s).bcom_task);
            if (*(*substream).pstr).stream == SNDRV_PCM_STREAM_CAPTURE {
                bcom_gen_bd_rx_reset((*s).bcom_task);
            } else {
                bcom_gen_bd_tx_reset((*s).bcom_task);
            }
            spin_unlock_irqrestore(&mut (*psc_dma).lock, flags);
        }

        _ => {
            dev_dbg(
                (*psc_dma).dev,
                b"unhandled trigger: stream=%i cmd=%i\n\0".as_ptr(),
                (*(*substream).pstr).stream,
                cmd,
            );
            return -EINVAL;
        }
    }

    /* Update interrupt enable settings */
    imr = 0;
    if (*psc_dma).playback.active != 0 {
        imr |= MPC52xx_PSC_IMR_TXEMP;
    }
    if (*psc_dma).capture.active != 0 {
        imr |= MPC52xx_PSC_IMR_ORERR;
    }
    out_be16(&mut (*regs).isr_imr.imr, (*psc_dma).imr | imr);

    0
}

/* ---------------------------------------------------------------------
 * The PSC DMA 'ASoC platform' driver
 *
 * Can be referenced by an 'ASoC machine' driver
 * This driver only deals with the audio bus; it doesn't have any
 * interaction with the attached codec
 */

static psc_dma_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_BATCH,
    formats: SNDRV_PCM_FMTBIT_S8
        | SNDRV_PCM_FMTBIT_S16_BE
        | SNDRV_PCM_FMTBIT_S24_BE
        | SNDRV_PCM_FMTBIT_S32_BE,
    period_bytes_max: 1024 * 1024,
    period_bytes_min: 32,
    periods_min: 2,
    periods_max: 256,
    buffer_bytes_max: 2 * 1024 * 1024,
    fifo_size: 512,
};

unsafe extern "C" fn psc_dma_open(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let psc_dma = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut psc_dma;
    let s: *mut psc_dma_stream;
    let rc: c_int;

    dev_dbg((*psc_dma).dev, b"psc_dma_open(substream=%p)\n\0".as_ptr(), substream);

    if (*(*substream).pstr).stream == SNDRV_PCM_STREAM_CAPTURE {
        s = &mut (*psc_dma).capture;
    } else {
        s = &mut (*psc_dma).playback;
    }

    snd_soc_set_runtime_hwparams(substream, &psc_dma_hardware);

    rc = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if rc < 0 {
        dev_err((*(*(*substream).pcm).card).snd_card.as_mut().unwrap().dev, b"invalid buffer size\n\0".as_ptr());
        return rc;
    }

    (*s).stream = substream;
    0
}

unsafe extern "C" fn psc_dma_close(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let psc_dma = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut psc_dma;
    let s: *mut psc_dma_stream;

    dev_dbg((*psc_dma).dev, b"psc_dma_close(substream=%p)\n\0".as_ptr(), substream);

    if (*(*substream).pstr).stream == SNDRV_PCM_STREAM_CAPTURE {
        s = &mut (*psc_dma).capture;
    } else {
        s = &mut (*psc_dma).playback;
    }

    if (*psc_dma).playback.active == 0 && (*psc_dma).capture.active == 0 {
        /* Disable all interrupts and reset the PSC */
        out_be16(&mut (*(*psc_dma).psc_regs).isr_imr.imr, (*psc_dma).imr);
        out_8(&mut (*(*psc_dma).psc_regs).command, 4 << 4); /* reset error */
    }
    (*s).stream = ptr::null_mut();
    0
}

unsafe extern "C" fn psc_dma_pointer(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let psc_dma = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut psc_dma;
    let s: *mut psc_dma_stream;
    let count: dma_addr_t;

    if (*(*substream).pstr).stream == SNDRV_PCM_STREAM_CAPTURE {
        s = &mut (*psc_dma).capture;
    } else {
        s = &mut (*psc_dma).playback;
    }

    count = ((*s).period_current as usize).wrapping_mul((*s).period_bytes);

    bytes_to_frames((*substream).runtime, count)
}

unsafe extern "C" fn psc_dma_new(
    component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let card: *mut snd_card = (*(*rtd).card).snd_card;
    let dai: *mut snd_soc_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let pcm: *mut snd_pcm = (*rtd).pcm;
    let size: usize = psc_dma_hardware.buffer_bytes_max;
    let rc: c_int;

    dev_dbg(
        (*component).dev,
        b"psc_dma_new(card=%p, dai=%p, pcm=%p)\n\0".as_ptr(),
        card,
        dai,
        pcm,
    );

    rc = dma_coerce_mask_and_coherent((*card).dev, DMA_BIT_MASK(32));
    if rc != 0 {
        return rc;
    }

    snd_pcm_set_fixed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, (*card).dev, size)
}

static mpc5200_audio_dma_component: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME.as_ptr(),
    open: Some(psc_dma_open),
    close: Some(psc_dma_close),
    pointer: Some(psc_dma_pointer),
    trigger: Some(psc_dma_trigger),
    pcm_new: Some(psc_dma_new),
};

#[no_mangle]
pub unsafe extern "C" fn mpc5200_audio_dma_create(op: *mut platform_device) -> c_int {
    let fifo: phys_addr_t;
    let psc_dma: *mut psc_dma;
    let mut res: *mut resource = ptr::null_mut();
    let mut size: c_int = 0;
    let irq: c_int;
    let rc: c_int;
    let prop: *const __be32;
    let regs: *mut c_void;

    regs = devm_platform_get_and_ioremap_resource(op, 0, &mut res);
    if IS_ERR(regs) {
        return PTR_ERR(regs);
    }

    /* Fetch the registers and IRQ of the PSC */
    irq = platform_get_irq(op, 0);
    if irq < 0 {
        return irq;
    }

    /* Allocate and initialize the driver private data */
    psc_dma = devm_kzalloc(&mut (*op).dev, size_of::<psc_dma>(), GFP_KERNEL) as *mut psc_dma;
    if psc_dma.is_null() {
        return -ENOMEM;
    }

    /* Get the PSC ID */
    prop = of_get_property((*op).dev.of_node, b"cell-index\0".as_ptr(), &mut size);
    if prop.is_null() || (size as usize) < size_of::<__be32>() {
        return -ENODEV;
    }

    spin_lock_init(&mut (*psc_dma).lock);
    mutex_init(&mut (*psc_dma).mutex);
    (*psc_dma).id = be32_to_cpu(*prop);
    (*psc_dma).irq = irq;
    (*psc_dma).psc_regs = regs as *mut mpc52xx_psc;
    (*psc_dma).fifo_regs = (regs as *mut u8).add(size_of::<*mut mpc52xx_psc>()) as *mut mpc52xx_psc_fifo;
    (*psc_dma).dev = &mut (*op).dev;
    (*psc_dma).playback.psc_dma = psc_dma;
    (*psc_dma).capture.psc_dma = psc_dma;
    snprintf(
        (*psc_dma).name.as_mut_ptr(),
        (*psc_dma).name.len(),
        b"PSC%d\0".as_ptr(),
        (*psc_dma).id,
    );

    /* Find the address of the fifo data registers and setup the
     * DMA tasks */
    fifo = (*res).start + offset_of!(mpc52xx_psc, buffer) + offset_of!(mpc52xx_psc_buffer, buffer_32);
    (*psc_dma).capture.bcom_task = bcom_psc_gen_bd_rx_init((*psc_dma).id as c_int, 10, fifo, 512);
    (*psc_dma).playback.bcom_task = bcom_psc_gen_bd_tx_init((*psc_dma).id as c_int, 10, fifo);
    if (*psc_dma).capture.bcom_task.is_null() || (*psc_dma).playback.bcom_task.is_null() {
        dev_err(&mut (*op).dev, b"Could not allocate bestcomm tasks\n\0".as_ptr());
        return -ENODEV;
    }

    /* Disable all interrupts and reset the PSC */
    out_be16(&mut (*(*psc_dma).psc_regs).isr_imr.imr, (*psc_dma).imr);
    /* reset receiver */
    out_8(&mut (*(*psc_dma).psc_regs).command, MPC52xx_PSC_RST_RX);
    /* reset transmitter */
    out_8(&mut (*(*psc_dma).psc_regs).command, MPC52xx_PSC_RST_TX);
    /* reset error */
    out_8(&mut (*(*psc_dma).psc_regs).command, MPC52xx_PSC_RST_ERR_STAT);
    /* reset mode */
    out_8(&mut (*(*psc_dma).psc_regs).command, MPC52xx_PSC_SEL_MODE_REG_1);

    /* Set up mode register;
     * First write: RxRdy (FIFO Alarm) generates rx FIFO irq
     * Second write: register Normal mode for non loopback
     */
    out_8(&mut (*(*psc_dma).psc_regs).mode, 0);
    out_8(&mut (*(*psc_dma).psc_regs).mode, 0);

    /* Set the TX and RX fifo alarm thresholds */
    out_be16(&mut (*(*psc_dma).fifo_regs).rfalarm, 0x100);
    out_8(&mut (*(*psc_dma).fifo_regs).rfcntl, 0x4);
    out_be16(&mut (*(*psc_dma).fifo_regs).tfalarm, 0x100);
    out_8(&mut (*(*psc_dma).fifo_regs).tfcntl, 0x7);

    /* Lookup the IRQ numbers */
    (*psc_dma).playback.irq = bcom_get_task_irq((*psc_dma).playback.bcom_task);
    (*psc_dma).capture.irq = bcom_get_task_irq((*psc_dma).capture.bcom_task);

    rc = devm_request_irq(
        &mut (*op).dev,
        (*psc_dma).irq,
        psc_dma_status_irq,
        IRQF_SHARED,
        b"psc-dma-status\0".as_ptr(),
        psc_dma as *mut c_void,
    ) | devm_request_irq(
        &mut (*op).dev,
        (*psc_dma).capture.irq,
        psc_dma_bcom_irq,
        IRQF_SHARED,
        b"psc-dma-capture\0".as_ptr(),
        &mut (*psc_dma).capture as *mut psc_dma_stream as *mut c_void,
    ) | devm_request_irq(
        &mut (*op).dev,
        (*psc_dma).playback.irq,
        psc_dma_bcom_irq,
        IRQF_SHARED,
        b"psc-dma-playback\0".as_ptr(),
        &mut (*psc_dma).playback as *mut psc_dma_stream as *mut c_void,
    );
    if rc != 0 {
        return -ENODEV;
    }

    /* Save what we've done so it can be found again later */
    dev_set_drvdata(&mut (*op).dev, psc_dma as *mut c_void);

    /* Tell the ASoC OF helpers about it */
    devm_snd_soc_register_component(
        &mut (*op).dev,
        &mpc5200_audio_dma_component,
        ptr::null_mut(),
        0,
    )
}

// EXPORT_SYMBOL_GPL(mpc5200_audio_dma_create);

#[no_mangle]
pub unsafe extern "C" fn mpc5200_audio_dma_destroy(op: *mut platform_device) -> c_int {
    let psc_dma = dev_get_drvdata(&mut (*op).dev) as *mut psc_dma;

    dev_dbg(&mut (*op).dev, b"mpc5200_audio_dma_destroy()\n\0".as_ptr());

    bcom_gen_bd_rx_release((*psc_dma).capture.bcom_task);
    bcom_gen_bd_tx_release((*psc_dma).playback.bcom_task);

    dev_set_drvdata(&mut (*op).dev, ptr::null_mut());

    0
}

// EXPORT_SYMBOL_GPL(mpc5200_audio_dma_destroy);

// MODULE_AUTHOR("Grant Likely <grant.likely@secretlab.ca>");
// MODULE_DESCRIPTION("Freescale MPC5200 PSC in DMA mode ASoC Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
