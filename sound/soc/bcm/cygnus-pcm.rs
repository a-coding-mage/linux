// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2014-2015 Broadcom Corporation

// Dependencies originally provided by Linux/ALSA/ASoC headers and "cygnus-ssp.h".

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type u32 = u32;
type u64 = u64;
type size_t = usize;
type bool_ = bool;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_uint;

const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

/* Register offset needed for ASoC PCM module */

const INTH_R5F_STATUS_OFFSET: u32 = 0x040;
const INTH_R5F_CLEAR_OFFSET: u32 = 0x048;
const INTH_R5F_MASK_SET_OFFSET: u32 = 0x050;
const INTH_R5F_MASK_CLEAR_OFFSET: u32 = 0x054;

const BF_REARM_FREE_MARK_OFFSET: u32 = 0x344;
const BF_REARM_FULL_MARK_OFFSET: u32 = 0x348;

/* Ring Buffer Ctrl Regs --- Start */
/* AUD_FMM_BF_CTRL_SOURCECH_RINGBUF_X_RDADDR_REG_BASE */
const SRC_RBUF_0_RDADDR_OFFSET: u32 = 0x500;
const SRC_RBUF_1_RDADDR_OFFSET: u32 = 0x518;
const SRC_RBUF_2_RDADDR_OFFSET: u32 = 0x530;
const SRC_RBUF_3_RDADDR_OFFSET: u32 = 0x548;
const SRC_RBUF_4_RDADDR_OFFSET: u32 = 0x560;
const SRC_RBUF_5_RDADDR_OFFSET: u32 = 0x578;
const SRC_RBUF_6_RDADDR_OFFSET: u32 = 0x590;

/* AUD_FMM_BF_CTRL_SOURCECH_RINGBUF_X_WRADDR_REG_BASE */
const SRC_RBUF_0_WRADDR_OFFSET: u32 = 0x504;
const SRC_RBUF_1_WRADDR_OFFSET: u32 = 0x51c;
const SRC_RBUF_2_WRADDR_OFFSET: u32 = 0x534;
const SRC_RBUF_3_WRADDR_OFFSET: u32 = 0x54c;
const SRC_RBUF_4_WRADDR_OFFSET: u32 = 0x564;
const SRC_RBUF_5_WRADDR_OFFSET: u32 = 0x57c;
const SRC_RBUF_6_WRADDR_OFFSET: u32 = 0x594;

/* AUD_FMM_BF_CTRL_SOURCECH_RINGBUF_X_BASEADDR_REG_BASE */
const SRC_RBUF_0_BASEADDR_OFFSET: u32 = 0x508;
const SRC_RBUF_1_BASEADDR_OFFSET: u32 = 0x520;
const SRC_RBUF_2_BASEADDR_OFFSET: u32 = 0x538;
const SRC_RBUF_3_BASEADDR_OFFSET: u32 = 0x550;
const SRC_RBUF_4_BASEADDR_OFFSET: u32 = 0x568;
const SRC_RBUF_5_BASEADDR_OFFSET: u32 = 0x580;
const SRC_RBUF_6_BASEADDR_OFFSET: u32 = 0x598;

/* AUD_FMM_BF_CTRL_SOURCECH_RINGBUF_X_ENDADDR_REG_BASE */
const SRC_RBUF_0_ENDADDR_OFFSET: u32 = 0x50c;
const SRC_RBUF_1_ENDADDR_OFFSET: u32 = 0x524;
const SRC_RBUF_2_ENDADDR_OFFSET: u32 = 0x53c;
const SRC_RBUF_3_ENDADDR_OFFSET: u32 = 0x554;
const SRC_RBUF_4_ENDADDR_OFFSET: u32 = 0x56c;
const SRC_RBUF_5_ENDADDR_OFFSET: u32 = 0x584;
const SRC_RBUF_6_ENDADDR_OFFSET: u32 = 0x59c;

/* AUD_FMM_BF_CTRL_SOURCECH_RINGBUF_X_FREE_MARK_REG_BASE */
const SRC_RBUF_0_FREE_MARK_OFFSET: u32 = 0x510;
const SRC_RBUF_1_FREE_MARK_OFFSET: u32 = 0x528;
const SRC_RBUF_2_FREE_MARK_OFFSET: u32 = 0x540;
const SRC_RBUF_3_FREE_MARK_OFFSET: u32 = 0x558;
const SRC_RBUF_4_FREE_MARK_OFFSET: u32 = 0x570;
const SRC_RBUF_5_FREE_MARK_OFFSET: u32 = 0x588;
const SRC_RBUF_6_FREE_MARK_OFFSET: u32 = 0x5a0;

/* AUD_FMM_BF_CTRL_DESTCH_RINGBUF_X_RDADDR_REG_BASE */
const DST_RBUF_0_RDADDR_OFFSET: u32 = 0x5c0;
const DST_RBUF_1_RDADDR_OFFSET: u32 = 0x5d8;
const DST_RBUF_2_RDADDR_OFFSET: u32 = 0x5f0;
const DST_RBUF_3_RDADDR_OFFSET: u32 = 0x608;
const DST_RBUF_4_RDADDR_OFFSET: u32 = 0x620;
const DST_RBUF_5_RDADDR_OFFSET: u32 = 0x638;

/* AUD_FMM_BF_CTRL_DESTCH_RINGBUF_X_WRADDR_REG_BASE */
const DST_RBUF_0_WRADDR_OFFSET: u32 = 0x5c4;
const DST_RBUF_1_WRADDR_OFFSET: u32 = 0x5dc;
const DST_RBUF_2_WRADDR_OFFSET: u32 = 0x5f4;
const DST_RBUF_3_WRADDR_OFFSET: u32 = 0x60c;
const DST_RBUF_4_WRADDR_OFFSET: u32 = 0x624;
const DST_RBUF_5_WRADDR_OFFSET: u32 = 0x63c;

/* AUD_FMM_BF_CTRL_DESTCH_RINGBUF_X_BASEADDR_REG_BASE */
const DST_RBUF_0_BASEADDR_OFFSET: u32 = 0x5c8;
const DST_RBUF_1_BASEADDR_OFFSET: u32 = 0x5e0;
const DST_RBUF_2_BASEADDR_OFFSET: u32 = 0x5f8;
const DST_RBUF_3_BASEADDR_OFFSET: u32 = 0x610;
const DST_RBUF_4_BASEADDR_OFFSET: u32 = 0x628;
const DST_RBUF_5_BASEADDR_OFFSET: u32 = 0x640;

/* AUD_FMM_BF_CTRL_DESTCH_RINGBUF_X_ENDADDR_REG_BASE */
const DST_RBUF_0_ENDADDR_OFFSET: u32 = 0x5cc;
const DST_RBUF_1_ENDADDR_OFFSET: u32 = 0x5e4;
const DST_RBUF_2_ENDADDR_OFFSET: u32 = 0x5fc;
const DST_RBUF_3_ENDADDR_OFFSET: u32 = 0x614;
const DST_RBUF_4_ENDADDR_OFFSET: u32 = 0x62c;
const DST_RBUF_5_ENDADDR_OFFSET: u32 = 0x644;

/* AUD_FMM_BF_CTRL_DESTCH_RINGBUF_X_FULL_MARK_REG_BASE */
const DST_RBUF_0_FULL_MARK_OFFSET: u32 = 0x5d0;
const DST_RBUF_1_FULL_MARK_OFFSET: u32 = 0x5e8;
const DST_RBUF_2_FULL_MARK_OFFSET: u32 = 0x600;
const DST_RBUF_3_FULL_MARK_OFFSET: u32 = 0x618;
const DST_RBUF_4_FULL_MARK_OFFSET: u32 = 0x630;
const DST_RBUF_5_FULL_MARK_OFFSET: u32 = 0x648;
/* Ring Buffer Ctrl Regs --- End */

/* Error Status Regs --- Start */
/* AUD_FMM_BF_ESR_ESRX_STATUS_REG_BASE */
const ESR0_STATUS_OFFSET: u32 = 0x900;
const ESR1_STATUS_OFFSET: u32 = 0x918;
const ESR2_STATUS_OFFSET: u32 = 0x930;
const ESR3_STATUS_OFFSET: u32 = 0x948;
const ESR4_STATUS_OFFSET: u32 = 0x960;

/* AUD_FMM_BF_ESR_ESRX_STATUS_CLEAR_REG_BASE */
const ESR0_STATUS_CLR_OFFSET: u32 = 0x908;
const ESR1_STATUS_CLR_OFFSET: u32 = 0x920;
const ESR2_STATUS_CLR_OFFSET: u32 = 0x938;
const ESR3_STATUS_CLR_OFFSET: u32 = 0x950;
const ESR4_STATUS_CLR_OFFSET: u32 = 0x968;

/* AUD_FMM_BF_ESR_ESRX_MASK_REG_BASE */
const ESR0_MASK_STATUS_OFFSET: u32 = 0x90c;
const ESR1_MASK_STATUS_OFFSET: u32 = 0x924;
const ESR2_MASK_STATUS_OFFSET: u32 = 0x93c;
const ESR3_MASK_STATUS_OFFSET: u32 = 0x954;
const ESR4_MASK_STATUS_OFFSET: u32 = 0x96c;

/* AUD_FMM_BF_ESR_ESRX_MASK_SET_REG_BASE */
const ESR0_MASK_SET_OFFSET: u32 = 0x910;
const ESR1_MASK_SET_OFFSET: u32 = 0x928;
const ESR2_MASK_SET_OFFSET: u32 = 0x940;
const ESR3_MASK_SET_OFFSET: u32 = 0x958;
const ESR4_MASK_SET_OFFSET: u32 = 0x970;

/* AUD_FMM_BF_ESR_ESRX_MASK_CLEAR_REG_BASE */
const ESR0_MASK_CLR_OFFSET: u32 = 0x914;
const ESR1_MASK_CLR_OFFSET: u32 = 0x92c;
const ESR2_MASK_CLR_OFFSET: u32 = 0x944;
const ESR3_MASK_CLR_OFFSET: u32 = 0x95c;
const ESR4_MASK_CLR_OFFSET: u32 = 0x974;
/* Error Status Regs --- End */

const R5F_ESR0_SHIFT: u32 = 0; /* esr0 = fifo underflow */
const R5F_ESR1_SHIFT: u32 = 1; /* esr1 = ringbuf underflow */
const R5F_ESR2_SHIFT: u32 = 2; /* esr2 = ringbuf overflow */
const R5F_ESR3_SHIFT: u32 = 3; /* esr3 = freemark */
const R5F_ESR4_SHIFT: u32 = 4; /* esr4 = fullmark */

/* Mask for R5F register.  Set all relevant interrupt for playback handler */
const ANY_PLAYBACK_IRQ: u32 = BIT(R5F_ESR0_SHIFT) | BIT(R5F_ESR1_SHIFT) | BIT(R5F_ESR3_SHIFT);

/* Mask for R5F register.  Set all relevant interrupt for capture handler */
const ANY_CAPTURE_IRQ: u32 = BIT(R5F_ESR2_SHIFT) | BIT(R5F_ESR4_SHIFT);

/*
 * PERIOD_BYTES_MIN is the number of bytes to at which the interrupt will tick.
 * This number should be a multiple of 256. Minimum value is 256
 */
const PERIOD_BYTES_MIN: u32 = 0x100;

const DMA_BIT_MASK_32: u64 = (1u64 << 32) - 1;

extern "C" {
    static SNDRV_PCM_INFO_MMAP: u32;
    static SNDRV_PCM_INFO_MMAP_VALID: u32;
    static SNDRV_PCM_INFO_INTERLEAVED: u32;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_HW_PARAM_PERIOD_BYTES: c_int;
    static SNDRV_PCM_HW_PARAM_BUFFER_BYTES: c_int;
    static SNDRV_DMA_TYPE_DEV: c_int;
    static IRQF_SHARED: c_ulong;
    static IRQ_NONE: irqreturn_t;
    static IRQ_HANDLED: irqreturn_t;
    static CYGNUS_MAX_PLAYBACK_PORTS: u32;
    static CYGNUS_MAX_CAPTURE_PORTS: u32;
}

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;

#[repr(C)]
pub struct device {
    pub dma_mask: *mut u64,
    pub coherent_dma_mask: u64,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    pub snd_card: *mut snd_card,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub dma_addr: u32,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ringbuf_regs {
    pub rdaddr: u32,
    pub wraddr: u32,
    pub baseaddr: u32,
    pub endaddr: u32,
    pub fmark: u32,
    pub period_bytes: u32,
    pub buf_size: u32,
}

const fn RINGBUF_REG_PLAYBACK(num: u32) -> ringbuf_regs {
    ringbuf_regs {
        rdaddr: SRC_RBUF_0_RDADDR_OFFSET + num * 0x18,
        wraddr: SRC_RBUF_0_WRADDR_OFFSET + num * 0x18,
        baseaddr: SRC_RBUF_0_BASEADDR_OFFSET + num * 0x18,
        endaddr: SRC_RBUF_0_ENDADDR_OFFSET + num * 0x18,
        fmark: SRC_RBUF_0_FREE_MARK_OFFSET + num * 0x18,
        period_bytes: 0,
        buf_size: 0,
    }
}

const fn RINGBUF_REG_CAPTURE(num: u32) -> ringbuf_regs {
    ringbuf_regs {
        rdaddr: DST_RBUF_0_RDADDR_OFFSET + num * 0x18,
        wraddr: DST_RBUF_0_WRADDR_OFFSET + num * 0x18,
        baseaddr: DST_RBUF_0_BASEADDR_OFFSET + num * 0x18,
        endaddr: DST_RBUF_0_ENDADDR_OFFSET + num * 0x18,
        fmark: DST_RBUF_0_FULL_MARK_OFFSET + num * 0x18,
        period_bytes: 0,
        buf_size: 0,
    }
}

#[repr(C)]
pub struct cygnus_port_info {
    pub play_stream: *mut snd_pcm_substream,
    pub capture_stream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct cygnus_audio {
    pub audio: *mut c_void,
    pub irq_num: c_int,
    pub dev: *mut device,
    pub portinfo: *mut cygnus_port_info,
}

#[repr(C)]
pub struct cygnus_aio_port {
    pub portnum: u32,
    pub play_rb_regs: ringbuf_regs,
    pub capture_rb_regs: ringbuf_regs,
    pub play_stream: *mut snd_pcm_substream,
    pub capture_stream: *mut snd_pcm_substream,
    pub cygaud: *mut cygnus_audio,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: u32,
    pub formats: u64,
    pub period_bytes_min: u32,
    pub period_bytes_max: u32,
    pub periods_min: u32,
    pub periods_max: u32,
    pub buffer_bytes_max: size_t,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pub pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int>,
}

extern "C" {
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream) -> *mut cygnus_aio_port;
    fn writel(value: u32, addr: *mut c_void);
    fn readl(addr: *mut c_void) -> u32;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_soc_set_runtime_hwparams(substream: *mut snd_pcm_substream, hw: *const snd_pcm_hardware);
    fn snd_pcm_hw_constraint_step(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, step: u32) -> c_int;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> c_ulong;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> c_ulong;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: c_uint) -> snd_pcm_uframes_t;
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, ty: c_int, dev: *mut device, size: size_t, max: size_t);
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        flags: c_ulong,
        name: *const c_char,
        data: *mut c_void,
    ) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut c_void,
        num_dai: c_int,
    ) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

static mut cygnus_pcm_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: 0,
    formats: 0,

    /* A period is basically an interrupt */
    period_bytes_min: PERIOD_BYTES_MIN,
    period_bytes_max: 0x10000,

    /* period_min/max gives range of approx interrupts per buffer */
    periods_min: 2,
    periods_max: 8,

    /*
     * maximum buffer size in bytes = period_bytes_max * periods_max
     * We allocate this amount of data for each enabled channel
     */
    buffer_bytes_max: 4 * 0x8000,
};

static mut cygnus_dma_dmamask: u64 = DMA_BIT_MASK_32;

unsafe fn init_cygnus_pcm_hw() {
    cygnus_pcm_hw.info = SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED;
    cygnus_pcm_hw.formats = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE;
}

unsafe fn ptr_add(base: *mut c_void, offset: u32) -> *mut c_void {
    (base as *mut u8).add(offset as usize) as *mut c_void
}

unsafe fn cygnus_dai_get_dma_data(substream: *mut snd_pcm_substream) -> *mut cygnus_aio_port {
    let soc_runtime: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);

    snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(soc_runtime, 0), substream)
}

unsafe fn ringbuf_set_initial(
    audio_io: *mut c_void,
    p_rbuf: *mut ringbuf_regs,
    is_playback: bool_,
    start: u32,
    periodsize: u32,
    bufsize: u32,
) {
    let initial_rd: u32;
    let initial_wr: u32;
    let end: u32;
    let fmark_val: u32; /* free or full mark */

    (*p_rbuf).period_bytes = periodsize;
    (*p_rbuf).buf_size = bufsize;

    if is_playback {
        /* Set the pointers to indicate full (flip uppermost bit) */
        initial_rd = start;
        initial_wr = initial_rd ^ BIT(31);
    } else {
        /* Set the pointers to indicate empty */
        initial_wr = start;
        initial_rd = initial_wr;
    }

    end = start.wrapping_add(bufsize).wrapping_sub(1);

    /*
     * The interrupt will fire when free/full mark is *exceeded*
     * The fmark value must be multiple of PERIOD_BYTES_MIN so set fmark
     * to be PERIOD_BYTES_MIN less than the period size.
     */
    fmark_val = periodsize.wrapping_sub(PERIOD_BYTES_MIN);

    writel(start, ptr_add(audio_io, (*p_rbuf).baseaddr));
    writel(end, ptr_add(audio_io, (*p_rbuf).endaddr));
    writel(fmark_val, ptr_add(audio_io, (*p_rbuf).fmark));
    writel(initial_rd, ptr_add(audio_io, (*p_rbuf).rdaddr));
    writel(initial_wr, ptr_add(audio_io, (*p_rbuf).wraddr));
}

unsafe fn configure_ringbuf_regs(substream: *mut snd_pcm_substream) -> c_int {
    let aio: *mut cygnus_aio_port;
    let p_rbuf: *mut ringbuf_regs;
    let mut status: c_int = 0;

    aio = cygnus_dai_get_dma_data(substream);

    /* Map the ssp portnum to a set of ring buffers. */
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        p_rbuf = &mut (*aio).play_rb_regs;

        match (*aio).portnum {
            0 => *p_rbuf = RINGBUF_REG_PLAYBACK(0),
            1 => *p_rbuf = RINGBUF_REG_PLAYBACK(2),
            2 => *p_rbuf = RINGBUF_REG_PLAYBACK(4),
            3 => *p_rbuf = RINGBUF_REG_PLAYBACK(6), /* SPDIF */
            _ => status = -EINVAL,
        }
    } else {
        p_rbuf = &mut (*aio).capture_rb_regs;

        match (*aio).portnum {
            0 => *p_rbuf = RINGBUF_REG_CAPTURE(0),
            1 => *p_rbuf = RINGBUF_REG_CAPTURE(2),
            2 => *p_rbuf = RINGBUF_REG_CAPTURE(4),
            _ => status = -EINVAL,
        }
    }

    status
}

unsafe fn get_ringbuf(substream: *mut snd_pcm_substream) -> *mut ringbuf_regs {
    let aio: *mut cygnus_aio_port;
    let p_rbuf: *mut ringbuf_regs;

    aio = cygnus_dai_get_dma_data(substream);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        p_rbuf = &mut (*aio).play_rb_regs;
    } else {
        p_rbuf = &mut (*aio).capture_rb_regs;
    }

    p_rbuf
}

unsafe fn enable_intr(substream: *mut snd_pcm_substream) {
    let aio: *mut cygnus_aio_port;
    let clear_mask: u32;

    aio = cygnus_dai_get_dma_data(substream);

    /* The port number maps to the bit position to be cleared */
    clear_mask = BIT((*aio).portnum);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        /* Clear interrupt status before enabling them */
        writel(clear_mask, ptr_add((*(*aio).cygaud).audio, ESR0_STATUS_CLR_OFFSET));
        writel(clear_mask, ptr_add((*(*aio).cygaud).audio, ESR1_STATUS_CLR_OFFSET));
        writel(clear_mask, ptr_add((*(*aio).cygaud).audio, ESR3_STATUS_CLR_OFFSET));
        /* Unmask the interrupts of the given port*/
        writel(clear_mask, ptr_add((*(*aio).cygaud).audio, ESR0_MASK_CLR_OFFSET));
        writel(clear_mask, ptr_add((*(*aio).cygaud).audio, ESR1_MASK_CLR_OFFSET));
        writel(clear_mask, ptr_add((*(*aio).cygaud).audio, ESR3_MASK_CLR_OFFSET));

        writel(ANY_PLAYBACK_IRQ, ptr_add((*(*aio).cygaud).audio, INTH_R5F_MASK_CLEAR_OFFSET));
    } else {
        writel(clear_mask, ptr_add((*(*aio).cygaud).audio, ESR2_STATUS_CLR_OFFSET));
        writel(clear_mask, ptr_add((*(*aio).cygaud).audio, ESR4_STATUS_CLR_OFFSET));
        writel(clear_mask, ptr_add((*(*aio).cygaud).audio, ESR2_MASK_CLR_OFFSET));
        writel(clear_mask, ptr_add((*(*aio).cygaud).audio, ESR4_MASK_CLR_OFFSET));

        writel(ANY_CAPTURE_IRQ, ptr_add((*(*aio).cygaud).audio, INTH_R5F_MASK_CLEAR_OFFSET));
    }
}

unsafe fn disable_intr(substream: *mut snd_pcm_substream) {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let aio: *mut cygnus_aio_port;
    let set_mask: u32;

    aio = cygnus_dai_get_dma_data(substream);

    dev_dbg((*snd_soc_rtd_to_cpu(rtd, 0)).dev, c"%s on port %d\n".as_ptr(), c"disable_intr".as_ptr(), (*aio).portnum);

    /* The port number maps to the bit position to be set */
    set_mask = BIT((*aio).portnum);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        /* Mask the interrupts of the given port*/
        writel(set_mask, ptr_add((*(*aio).cygaud).audio, ESR0_MASK_SET_OFFSET));
        writel(set_mask, ptr_add((*(*aio).cygaud).audio, ESR1_MASK_SET_OFFSET));
        writel(set_mask, ptr_add((*(*aio).cygaud).audio, ESR3_MASK_SET_OFFSET));
    } else {
        writel(set_mask, ptr_add((*(*aio).cygaud).audio, ESR2_MASK_SET_OFFSET));
        writel(set_mask, ptr_add((*(*aio).cygaud).audio, ESR4_MASK_SET_OFFSET));
    }
}

unsafe extern "C" fn cygnus_pcm_trigger(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let mut ret: c_int = 0;

    if cmd == SNDRV_PCM_TRIGGER_START || cmd == SNDRV_PCM_TRIGGER_RESUME {
        enable_intr(substream);
    } else if cmd == SNDRV_PCM_TRIGGER_STOP || cmd == SNDRV_PCM_TRIGGER_SUSPEND {
        disable_intr(substream);
    } else {
        ret = -EINVAL;
    }

    ret
}

unsafe fn cygnus_pcm_period_elapsed(substream: *mut snd_pcm_substream) {
    let aio: *mut cygnus_aio_port;
    let p_rbuf: *mut ringbuf_regs;
    let mut regval: u32;

    aio = cygnus_dai_get_dma_data(substream);

    p_rbuf = get_ringbuf(substream);

    /*
     * If free/full mark interrupt occurs, provide timestamp
     * to ALSA and update appropriate idx by period_bytes
     */
    snd_pcm_period_elapsed(substream);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        /* Set the ring buffer to full */
        regval = readl(ptr_add((*(*aio).cygaud).audio, (*p_rbuf).rdaddr));
        regval ^= BIT(31);
        writel(regval, ptr_add((*(*aio).cygaud).audio, (*p_rbuf).wraddr));
    } else {
        /* Set the ring buffer to empty */
        regval = readl(ptr_add((*(*aio).cygaud).audio, (*p_rbuf).wraddr));
        writel(regval, ptr_add((*(*aio).cygaud).audio, (*p_rbuf).rdaddr));
    }
}

/*
 * ESR0/1/3 status  Description
 *  0x1	I2S0_out port caused interrupt
 *  0x2	I2S1_out port caused interrupt
 *  0x4	I2S2_out port caused interrupt
 *  0x8	SPDIF_out port caused interrupt
 */
unsafe fn handle_playback_irq(cygaud: *mut cygnus_audio) {
    let audio_io: *mut c_void;
    let mut port: u32;
    let mut esr_status0: u32;
    let mut esr_status1: u32;
    let mut esr_status3: u32;

    audio_io = (*cygaud).audio;

    /*
     * ESR status gets updates with/without interrupts enabled.
     * So, check the ESR mask, which provides interrupt enable/
     * disable status and use it to determine which ESR status
     * should be serviced.
     */
    esr_status0 = readl(ptr_add(audio_io, ESR0_STATUS_OFFSET));
    esr_status0 &= !readl(ptr_add(audio_io, ESR0_MASK_STATUS_OFFSET));
    esr_status1 = readl(ptr_add(audio_io, ESR1_STATUS_OFFSET));
    esr_status1 &= !readl(ptr_add(audio_io, ESR1_MASK_STATUS_OFFSET));
    esr_status3 = readl(ptr_add(audio_io, ESR3_STATUS_OFFSET));
    esr_status3 &= !readl(ptr_add(audio_io, ESR3_MASK_STATUS_OFFSET));

    port = 0;
    while port < CYGNUS_MAX_PLAYBACK_PORTS {
        let esrmask: u32 = BIT(port);

        /*
         * Ringbuffer or FIFO underflow
         * If we get this interrupt then, it is also true that we have
         * not yet responded to the freemark interrupt.
         * Log a debug message.  The freemark handler below will
         * handle getting everything going again.
         */
        if (esrmask & esr_status1) != 0 || (esrmask & esr_status0) != 0 {
            dev_dbg(
                (*cygaud).dev,
                c"Underrun: esr0=0x%x, esr1=0x%x esr3=0x%x\n".as_ptr(),
                esr_status0,
                esr_status1,
                esr_status3,
            );
        }

        /*
         * Freemark is hit. This is the normal interrupt.
         * In typical operation the read and write regs will be equal
         */
        if (esrmask & esr_status3) != 0 {
            let playstr: *mut snd_pcm_substream;

            playstr = (*(*cygaud).portinfo.add(port as usize)).play_stream;
            cygnus_pcm_period_elapsed(playstr);
        }
        port += 1;
    }

    /* Clear ESR interrupt */
    writel(esr_status0, ptr_add(audio_io, ESR0_STATUS_CLR_OFFSET));
    writel(esr_status1, ptr_add(audio_io, ESR1_STATUS_CLR_OFFSET));
    writel(esr_status3, ptr_add(audio_io, ESR3_STATUS_CLR_OFFSET));
    /* Rearm freemark logic by writing 1 to the correct bit */
    writel(esr_status3, ptr_add(audio_io, BF_REARM_FREE_MARK_OFFSET));
}

/*
 * ESR2/4 status  Description
 *  0x1	I2S0_in port caused interrupt
 *  0x2	I2S1_in port caused interrupt
 *  0x4	I2S2_in port caused interrupt
 */
unsafe fn handle_capture_irq(cygaud: *mut cygnus_audio) {
    let audio_io: *mut c_void;
    let mut port: u32;
    let mut esr_status2: u32;
    let mut esr_status4: u32;

    audio_io = (*cygaud).audio;

    /*
     * ESR status gets updates with/without interrupts enabled.
     * So, check the ESR mask, which provides interrupt enable/
     * disable status and use it to determine which ESR status
     * should be serviced.
     */
    esr_status2 = readl(ptr_add(audio_io, ESR2_STATUS_OFFSET));
    esr_status2 &= !readl(ptr_add(audio_io, ESR2_MASK_STATUS_OFFSET));
    esr_status4 = readl(ptr_add(audio_io, ESR4_STATUS_OFFSET));
    esr_status4 &= !readl(ptr_add(audio_io, ESR4_MASK_STATUS_OFFSET));

    port = 0;
    while port < CYGNUS_MAX_CAPTURE_PORTS {
        let esrmask: u32 = BIT(port);

        /*
         * Ringbuffer or FIFO overflow
         * If we get this interrupt then, it is also true that we have
         * not yet responded to the fullmark interrupt.
         * Log a debug message.  The fullmark handler below will
         * handle getting everything going again.
         */
        if (esrmask & esr_status2) != 0 {
            dev_dbg((*cygaud).dev, c"Overflow: esr2=0x%x\n".as_ptr(), esr_status2);
        }

        if (esrmask & esr_status4) != 0 {
            let capstr: *mut snd_pcm_substream;

            capstr = (*(*cygaud).portinfo.add(port as usize)).capture_stream;
            cygnus_pcm_period_elapsed(capstr);
        }
        port += 1;
    }

    writel(esr_status2, ptr_add(audio_io, ESR2_STATUS_CLR_OFFSET));
    writel(esr_status4, ptr_add(audio_io, ESR4_STATUS_CLR_OFFSET));
    /* Rearm fullmark logic by writing 1 to the correct bit */
    writel(esr_status4, ptr_add(audio_io, BF_REARM_FULL_MARK_OFFSET));
}

unsafe extern "C" fn cygnus_dma_irq(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let r5_status: u32;
    let cygaud: *mut cygnus_audio = data as *mut cygnus_audio;

    /*
     * R5 status bits	Description
     *  0		ESR0 (playback FIFO interrupt)
     *  1		ESR1 (playback rbuf interrupt)
     *  2		ESR2 (capture rbuf interrupt)
     *  3		ESR3 (Freemark play. interrupt)
     *  4		ESR4 (Fullmark capt. interrupt)
     */
    r5_status = readl(ptr_add((*cygaud).audio, INTH_R5F_STATUS_OFFSET));

    if (r5_status & (ANY_PLAYBACK_IRQ | ANY_CAPTURE_IRQ)) == 0 {
        return IRQ_NONE;
    }

    /* If playback interrupt happened */
    if (ANY_PLAYBACK_IRQ & r5_status) != 0 {
        handle_playback_irq(cygaud);
        writel(ANY_PLAYBACK_IRQ & r5_status, ptr_add((*cygaud).audio, INTH_R5F_CLEAR_OFFSET));
    }

    /* If  capture interrupt happened */
    if (ANY_CAPTURE_IRQ & r5_status) != 0 {
        handle_capture_irq(cygaud);
        writel(ANY_CAPTURE_IRQ & r5_status, ptr_add((*cygaud).audio, INTH_R5F_CLEAR_OFFSET));
    }

    IRQ_HANDLED
}

unsafe extern "C" fn cygnus_pcm_open(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let aio: *mut cygnus_aio_port;
    let mut ret: c_int;

    aio = cygnus_dai_get_dma_data(substream);
    if aio.is_null() {
        return -ENODEV;
    }

    dev_dbg((*snd_soc_rtd_to_cpu(rtd, 0)).dev, c"%s port %d\n".as_ptr(), c"cygnus_pcm_open".as_ptr(), (*aio).portnum);

    init_cygnus_pcm_hw();
    snd_soc_set_runtime_hwparams(substream, &raw const cygnus_pcm_hw);

    ret = snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, PERIOD_BYTES_MIN);
    if ret < 0 {
        return ret;
    }

    ret = snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_BYTES, PERIOD_BYTES_MIN);
    if ret < 0 {
        return ret;
    }
    /*
     * Keep track of which substream belongs to which port.
     * This info is needed by snd_pcm_period_elapsed() in irq_handler
     */
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*aio).play_stream = substream;
    } else {
        (*aio).capture_stream = substream;
    }

    0
}

unsafe extern "C" fn cygnus_pcm_close(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let aio: *mut cygnus_aio_port;

    aio = cygnus_dai_get_dma_data(substream);

    dev_dbg((*snd_soc_rtd_to_cpu(rtd, 0)).dev, c"%s  port %d\n".as_ptr(), c"cygnus_pcm_close".as_ptr(), (*aio).portnum);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*aio).play_stream = core::ptr::null_mut();
    } else {
        (*aio).capture_stream = core::ptr::null_mut();
    }

    if (*aio).play_stream.is_null() && (*aio).capture_stream.is_null() {
        dev_dbg((*snd_soc_rtd_to_cpu(rtd, 0)).dev, c"freed  port %d\n".as_ptr(), (*aio).portnum);
    }

    0
}

unsafe extern "C" fn cygnus_pcm_prepare(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let aio: *mut cygnus_aio_port;
    let bufsize: c_ulong;
    let periodsize: c_ulong;
    let is_play: bool_;
    let start: u32;
    let p_rbuf: *mut ringbuf_regs;

    aio = cygnus_dai_get_dma_data(substream);
    dev_dbg((*snd_soc_rtd_to_cpu(rtd, 0)).dev, c"%s port %d\n".as_ptr(), c"cygnus_pcm_prepare".as_ptr(), (*aio).portnum);

    bufsize = snd_pcm_lib_buffer_bytes(substream);
    periodsize = snd_pcm_lib_period_bytes(substream);

    dev_dbg(
        (*snd_soc_rtd_to_cpu(rtd, 0)).dev,
        c"%s (buf_size %lu) (period_size %lu)\n".as_ptr(),
        c"cygnus_pcm_prepare".as_ptr(),
        bufsize,
        periodsize,
    );

    configure_ringbuf_regs(substream);

    p_rbuf = get_ringbuf(substream);

    start = (*runtime).dma_addr;

    is_play = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK { true } else { false };

    ringbuf_set_initial((*(*aio).cygaud).audio, p_rbuf, is_play, start, periodsize as u32, bufsize as u32);

    0
}

unsafe extern "C" fn cygnus_pcm_pointer(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let aio: *mut cygnus_aio_port;
    let res: c_uint;
    let cur: c_uint;
    let base: c_uint;
    let p_rbuf: *mut ringbuf_regs;

    aio = cygnus_dai_get_dma_data(substream);

    /*
     * Get the offset of the current read (for playack) or write
     * index (for capture).  Report this value back to the asoc framework.
     */
    p_rbuf = get_ringbuf(substream);
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        cur = readl(ptr_add((*(*aio).cygaud).audio, (*p_rbuf).rdaddr));
    } else {
        cur = readl(ptr_add((*(*aio).cygaud).audio, (*p_rbuf).wraddr));
    }

    base = readl(ptr_add((*(*aio).cygaud).audio, (*p_rbuf).baseaddr));

    /*
     * Mask off the MSB of the rdaddr,wraddr and baseaddr
     * since MSB is not part of the address
     */
    res = (cur & 0x7fffffff).wrapping_sub(base & 0x7fffffff);

    bytes_to_frames((*substream).runtime, res)
}

unsafe extern "C" fn cygnus_dma_new(
    _component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let size: size_t = cygnus_pcm_hw.buffer_bytes_max;
    let card: *mut snd_card = (*(*rtd).card).snd_card;

    if (*(*card).dev).dma_mask.is_null() {
        (*(*card).dev).dma_mask = &raw mut cygnus_dma_dmamask;
    }
    if (*(*card).dev).coherent_dma_mask == 0 {
        (*(*card).dev).coherent_dma_mask = DMA_BIT_MASK_32;
    }

    snd_pcm_set_managed_buffer_all((*rtd).pcm, SNDRV_DMA_TYPE_DEV, (*card).dev, size, size);

    0
}

static cygnus_soc_platform: snd_soc_component_driver = snd_soc_component_driver {
    open: Some(cygnus_pcm_open),
    close: Some(cygnus_pcm_close),
    prepare: Some(cygnus_pcm_prepare),
    trigger: Some(cygnus_pcm_trigger),
    pointer: Some(cygnus_pcm_pointer),
    pcm_new: Some(cygnus_dma_new),
};

#[no_mangle]
pub unsafe extern "C" fn cygnus_soc_platform_register(
    dev: *mut device,
    cygaud: *mut cygnus_audio,
) -> c_int {
    let mut rc: c_int;

    dev_dbg(dev, c"%s Enter\n".as_ptr(), c"cygnus_soc_platform_register".as_ptr());

    rc = devm_request_irq(
        dev,
        (*cygaud).irq_num,
        cygnus_dma_irq,
        IRQF_SHARED,
        c"cygnus-audio".as_ptr(),
        cygaud as *mut c_void,
    );
    if rc != 0 {
        dev_err(dev, c"%s request_irq error %d\n".as_ptr(), c"cygnus_soc_platform_register".as_ptr(), rc);
        return rc;
    }

    rc = devm_snd_soc_register_component(dev, &cygnus_soc_platform, core::ptr::null_mut(), 0);
    if rc != 0 {
        dev_err(dev, c"%s failed\n".as_ptr(), c"cygnus_soc_platform_register".as_ptr());
        return rc;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn cygnus_soc_platform_unregister(_dev: *mut device) -> c_int {
    0
}

// MODULE_LICENSE("GPL v2");
// MODULE_AUTHOR("Broadcom");
// MODULE_DESCRIPTION("Cygnus ASoC PCM module");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
