// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * kirkwood-dma.c
 *
 * (c) 2010 Arnaud Patard <apatard@mandriva.com>
 * (c) 2010 Arnaud Patard <arnaud.patard@rtp-net.org>
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

type size_t = usize;
type irqreturn_t = c_int;
type snd_pcm_uframes_t = c_ulong;
type dma_addr_t = c_ulong;

const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_SHARED: c_ulong = 0x00000080;
const EBUSY: c_int = 16;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SNDRV_PCM_HW_PARAM_BUFFER_BYTES: c_int = 0;
const SNDRV_PCM_HW_PARAM_PERIOD_BYTES: c_int = 0;

const SNDRV_PCM_INFO_INTERLEAVED: u32 = 0;
const SNDRV_PCM_INFO_MMAP: u32 = 0;
const SNDRV_PCM_INFO_MMAP_VALID: u32 = 0;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: u32 = 0;
const SNDRV_PCM_INFO_PAUSE: u32 = 0;
const SNDRV_PCM_INFO_NO_PERIOD_WAKEUP: u32 = 0;

const KIRKWOOD_SND_MAX_BUFFER_BYTES: size_t = 0;
const KIRKWOOD_SND_MIN_PERIOD_BYTES: size_t = 0;
const KIRKWOOD_SND_MAX_PERIOD_BYTES: size_t = 0;
const KIRKWOOD_SND_MIN_PERIODS: u32 = 0;
const KIRKWOOD_SND_MAX_PERIODS: u32 = 0;
const KIRKWOOD_AUDIO_BUF_MAX: c_ulong = 0;

const KIRKWOOD_INT_MASK: usize = 0;
const KIRKWOOD_INT_CAUSE: usize = 0;
const KIRKWOOD_ERR_CAUSE: usize = 0;
const KIRKWOOD_ERR_MASK: usize = 0;
const KIRKWOOD_INT_CAUSE_PLAY_BYTES: c_ulong = 0;
const KIRKWOOD_INT_CAUSE_REC_BYTES: c_ulong = 0;
const KIRKWOOD_PLAYBACK_WIN: c_int = 0;
const KIRKWOOD_RECORD_WIN: c_int = 0;
const KIRKWOOD_PLAY_BYTE_INT_COUNT: usize = 0;
const KIRKWOOD_PLAY_BUF_ADDR: usize = 0;
const KIRKWOOD_PLAY_BUF_SIZE: usize = 0;
const KIRKWOOD_REC_BYTE_INT_COUNT: usize = 0;
const KIRKWOOD_REC_BUF_ADDR: usize = 0;
const KIRKWOOD_REC_BUF_SIZE: usize = 0;
const KIRKWOOD_PLAY_BYTE_COUNT: usize = 0;
const KIRKWOOD_REC_BYTE_COUNT: usize = 0;

const DRV_NAME: *const c_char = b"kirkwood-i2s\0".as_ptr() as *const c_char;
const KERN_WARNING: *const c_char = b"\0".as_ptr() as *const c_char;

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub dma_addr: dma_addr_t,
    pub buffer_size: snd_pcm_uframes_t,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_soc_card {
    pub snd_card: *mut snd_card,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kirkwood_dma_data {
    pub io: *mut c_void,
    pub irq: c_int,
    pub burst: c_ulong,
    pub substream_play: *mut snd_pcm_substream,
    pub substream_rec: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: u32,
    pub buffer_bytes_max: size_t,
    pub period_bytes_min: size_t,
    pub period_bytes_max: size_t,
    pub periods_min: u32,
    pub periods_max: u32,
    pub fifo_size: u32,
}

#[repr(C)]
pub struct mbus_dram_window {
    pub base: c_ulong,
    pub size: c_ulong,
    pub mbus_attr: c_ulong,
}

#[repr(C)]
pub struct mbus_dram_target_info {
    pub num_cs: c_int,
    pub cs: *const mbus_dram_window,
    pub mbus_dram_target_id: c_ulong,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub open: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int,
    >,
    pub close: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int,
    >,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
        ) -> c_int,
    >,
    pub prepare: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int,
    >,
    pub pointer: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *mut snd_pcm_substream,
        ) -> snd_pcm_uframes_t,
    >,
    pub pcm_new: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int,
    >,
}

unsafe extern "C" {
    fn snd_soc_substream_to_rtd(subs: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut c_void;
    fn snd_soc_dai_get_drvdata(dai: *mut c_void) -> *mut c_void;
    fn readl(addr: *mut c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn printk(fmt: *const c_char, ...) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_soc_set_runtime_hwparams(
        substream: *mut snd_pcm_substream,
        hw: *const snd_pcm_hardware,
    );
    fn snd_pcm_hw_constraint_minmax(
        runtime: *mut snd_pcm_runtime,
        var: c_int,
        min: c_ulong,
        max: c_ulong,
    ) -> c_int;
    fn snd_pcm_hw_constraint_step(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        step: c_ulong,
    ) -> c_int;
    fn request_irq(
        irq: c_int,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        flags: c_ulong,
        name: *const c_char,
        dev: *mut c_void,
    ) -> c_int;
    fn free_irq(irq: c_int, dev: *mut c_void);
    fn mv_mbus_dram_info() -> *const mbus_dram_target_info;
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: snd_pcm_uframes_t) -> c_ulong;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> c_ulong;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: c_ulong) -> snd_pcm_uframes_t;
    fn dma_coerce_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        typ: c_int,
        dev: *mut device,
        size: size_t,
        max: size_t,
    );
}

type c_uint = u32;

#[inline]
fn DMA_BIT_MASK(nr: u32) -> u64 {
    if nr == 64 {
        !0u64
    } else {
        (1u64 << nr) - 1
    }
}

#[inline]
unsafe fn io_add(base: *mut c_void, offset: usize) -> *mut c_void {
    (base as *mut u8).add(offset) as *mut c_void
}

#[inline]
fn KIRKWOOD_AUDIO_WIN_CTRL_REG(win: c_int) -> usize {
    win as usize
}

#[inline]
fn KIRKWOOD_AUDIO_WIN_BASE_REG(win: c_int) -> usize {
    win as usize
}

unsafe fn kirkwood_priv(subs: *mut snd_pcm_substream) -> *mut kirkwood_dma_data {
    let soc_runtime: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(subs);
    snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(soc_runtime, 0)) as *mut kirkwood_dma_data
}

static kirkwood_dma_snd_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_NO_PERIOD_WAKEUP,
    buffer_bytes_max: KIRKWOOD_SND_MAX_BUFFER_BYTES,
    period_bytes_min: KIRKWOOD_SND_MIN_PERIOD_BYTES,
    period_bytes_max: KIRKWOOD_SND_MAX_PERIOD_BYTES,
    periods_min: KIRKWOOD_SND_MIN_PERIODS,
    periods_max: KIRKWOOD_SND_MAX_PERIODS,
    fifo_size: 0,
};

unsafe extern "C" fn kirkwood_dma_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let priv_: *mut kirkwood_dma_data = dev_id as *mut kirkwood_dma_data;
    let mask: c_ulong;
    let status: c_ulong;
    let cause: c_ulong;

    mask = readl(io_add((*priv_).io, KIRKWOOD_INT_MASK)) as c_ulong;
    status = (readl(io_add((*priv_).io, KIRKWOOD_INT_CAUSE)) as c_ulong) & mask;

    cause = readl(io_add((*priv_).io, KIRKWOOD_ERR_CAUSE)) as c_ulong;
    if cause != 0 {
        printk(
            b"%s: got err interrupt 0x%lx\n\0".as_ptr() as *const c_char,
            b"kirkwood_dma_irq\0".as_ptr() as *const c_char,
            cause,
        );
        writel(cause as u32, io_add((*priv_).io, KIRKWOOD_ERR_CAUSE));
    }

    /* we've enabled only bytes interrupts ... */
    if (status & !(KIRKWOOD_INT_CAUSE_PLAY_BYTES | KIRKWOOD_INT_CAUSE_REC_BYTES)) != 0 {
        printk(
            b"%s: unexpected interrupt %lx\n\0".as_ptr() as *const c_char,
            b"kirkwood_dma_irq\0".as_ptr() as *const c_char,
            status,
        );
        return IRQ_NONE;
    }

    /* ack int */
    writel(status as u32, io_add((*priv_).io, KIRKWOOD_INT_CAUSE));

    if (status & KIRKWOOD_INT_CAUSE_PLAY_BYTES) != 0 {
        snd_pcm_period_elapsed((*priv_).substream_play);
    }

    if (status & KIRKWOOD_INT_CAUSE_REC_BYTES) != 0 {
        snd_pcm_period_elapsed((*priv_).substream_rec);
    }

    IRQ_HANDLED
}

unsafe fn kirkwood_dma_conf_mbus_windows(
    base: *mut c_void,
    win: c_int,
    dma: c_ulong,
    dram: *const mbus_dram_target_info,
) {
    let mut i: c_int;

    /* First disable and clear windows */
    writel(0, io_add(base, KIRKWOOD_AUDIO_WIN_CTRL_REG(win)));
    writel(0, io_add(base, KIRKWOOD_AUDIO_WIN_BASE_REG(win)));

    /* try to find matching cs for current dma address */
    i = 0;
    while i < (*dram).num_cs {
        let cs: *const mbus_dram_window = (*dram).cs.add(i as usize);
        if ((*cs).base & 0xffff0000) < (dma & 0xffff0000) {
            writel(
                ((*cs).base & 0xffff0000) as u32,
                io_add(base, KIRKWOOD_AUDIO_WIN_BASE_REG(win)),
            );
            writel(
                ((((*cs).size - 1) & 0xffff0000)
                    | ((*cs).mbus_attr << 8)
                    | ((*dram).mbus_dram_target_id << 4)
                    | 1) as u32,
                io_add(base, KIRKWOOD_AUDIO_WIN_CTRL_REG(win)),
            );
        }
        i += 1;
    }
}

unsafe extern "C" fn kirkwood_dma_open(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let mut err: c_int;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let priv_: *mut kirkwood_dma_data = kirkwood_priv(substream);

    snd_soc_set_runtime_hwparams(substream, &kirkwood_dma_snd_hw);

    /* Ensure that all constraints linked to dma burst are fulfilled */
    err = snd_pcm_hw_constraint_minmax(
        runtime,
        SNDRV_PCM_HW_PARAM_BUFFER_BYTES,
        (*priv_).burst * 2,
        KIRKWOOD_AUDIO_BUF_MAX - 1,
    );
    if err < 0 {
        return err;
    }

    err = snd_pcm_hw_constraint_step(
        runtime,
        0,
        SNDRV_PCM_HW_PARAM_BUFFER_BYTES,
        (*priv_).burst,
    );
    if err < 0 {
        return err;
    }

    err = snd_pcm_hw_constraint_step(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_PERIOD_BYTES,
        (*priv_).burst,
    );
    if err < 0 {
        return err;
    }

    if (*priv_).substream_play.is_null() && (*priv_).substream_rec.is_null() {
        err = request_irq(
            (*priv_).irq,
            kirkwood_dma_irq,
            IRQF_SHARED,
            b"kirkwood-i2s\0".as_ptr() as *const c_char,
            priv_ as *mut c_void,
        );
        if err != 0 {
            return err;
        }

        /*
         * Enable Error interrupts. We're only ack'ing them but
         * it's useful for diagnostics
         */
        writel(!0u32, io_add((*priv_).io, KIRKWOOD_ERR_MASK));
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        if !(*priv_).substream_play.is_null() {
            return -EBUSY;
        }
        (*priv_).substream_play = substream;
    } else {
        if !(*priv_).substream_rec.is_null() {
            return -EBUSY;
        }
        (*priv_).substream_rec = substream;
    }

    0
}

unsafe extern "C" fn kirkwood_dma_close(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let priv_: *mut kirkwood_dma_data = kirkwood_priv(substream);

    if priv_.is_null() {
        return 0;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*priv_).substream_play = core::ptr::null_mut();
    } else {
        (*priv_).substream_rec = core::ptr::null_mut();
    }

    if (*priv_).substream_play.is_null() && (*priv_).substream_rec.is_null() {
        writel(0, io_add((*priv_).io, KIRKWOOD_ERR_MASK));
        free_irq((*priv_).irq, priv_ as *mut c_void);
    }

    0
}

unsafe extern "C" fn kirkwood_dma_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let priv_: *mut kirkwood_dma_data = kirkwood_priv(substream);
    let dram: *const mbus_dram_target_info = mv_mbus_dram_info();
    let addr: c_ulong = (*(*substream).runtime).dma_addr as c_ulong;

    if dram.is_null() {
        return 0;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        kirkwood_dma_conf_mbus_windows((*priv_).io, KIRKWOOD_PLAYBACK_WIN, addr, dram);
    } else {
        kirkwood_dma_conf_mbus_windows((*priv_).io, KIRKWOOD_RECORD_WIN, addr, dram);
    }
    0
}

unsafe extern "C" fn kirkwood_dma_prepare(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let priv_: *mut kirkwood_dma_data = kirkwood_priv(substream);
    let mut size: c_ulong;
    let count: c_ulong;

    /* compute buffer size in term of "words" as requested in specs */
    size = frames_to_bytes(runtime, (*runtime).buffer_size);
    size = (size >> 2) - 1;
    count = snd_pcm_lib_period_bytes(substream);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        writel(count as u32, io_add((*priv_).io, KIRKWOOD_PLAY_BYTE_INT_COUNT));
        writel((*runtime).dma_addr as u32, io_add((*priv_).io, KIRKWOOD_PLAY_BUF_ADDR));
        writel(size as u32, io_add((*priv_).io, KIRKWOOD_PLAY_BUF_SIZE));
    } else {
        writel(count as u32, io_add((*priv_).io, KIRKWOOD_REC_BYTE_INT_COUNT));
        writel((*runtime).dma_addr as u32, io_add((*priv_).io, KIRKWOOD_REC_BUF_ADDR));
        writel(size as u32, io_add((*priv_).io, KIRKWOOD_REC_BUF_SIZE));
    }

    0
}

unsafe extern "C" fn kirkwood_dma_pointer(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let priv_: *mut kirkwood_dma_data = kirkwood_priv(substream);
    let count: snd_pcm_uframes_t;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        count = bytes_to_frames(
            (*substream).runtime,
            readl(io_add((*priv_).io, KIRKWOOD_PLAY_BYTE_COUNT)) as c_ulong,
        );
    } else {
        count = bytes_to_frames(
            (*substream).runtime,
            readl(io_add((*priv_).io, KIRKWOOD_REC_BYTE_COUNT)) as c_ulong,
        );
    }

    count
}

unsafe extern "C" fn kirkwood_dma_new(
    component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let size: size_t = kirkwood_dma_snd_hw.buffer_bytes_max;
    let card: *mut snd_card = (*(*rtd).card).snd_card;
    let ret: c_int;

    ret = dma_coerce_mask_and_coherent((*card).dev, DMA_BIT_MASK(32));
    if ret != 0 {
        return ret;
    }

    snd_pcm_set_managed_buffer_all(
        (*rtd).pcm,
        SNDRV_DMA_TYPE_DEV,
        (*card).dev,
        size,
        size,
    );

    0
}

#[no_mangle]
pub static kirkwood_soc_component: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME,
    open: Some(kirkwood_dma_open),
    close: Some(kirkwood_dma_close),
    hw_params: Some(kirkwood_dma_hw_params),
    prepare: Some(kirkwood_dma_prepare),
    pointer: Some(kirkwood_dma_pointer),
    pcm_new: Some(kirkwood_dma_new),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
