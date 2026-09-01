// SPDX-License-Identifier: GPL-2.0
//
// Renesas RZ/G2L ASoC Serial Sound Interface (SSIF-2) Driver
//
// Copyright (C) 2021 Renesas Electronics Corp.
// Copyright (C) 2019 Chris Brandt.
//
// Rust translation of soc/renesas/rz-ssi.c. Kernel types, helpers, macros, and
// registration machinery referenced here are external dependencies.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type c_int = i32;
type c_uint = u32;
type c_ulong = u64;
type c_char = i8;
type c_void = core::ffi::c_void;
type u8 = u8;
type u16 = u16;
type u32 = u32;
type bool_ = bool;
type uint = c_uint;
type dma_addr_t = u64;
type snd_pcm_uframes_t = u64;
type irqreturn_t = c_int;
type gfp_t = c_uint;
type spinlock_t = c_uint;

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

/* REGISTER OFFSET */
const SSICR: uint = 0x000;
const SSISR: uint = 0x004;
const SSIFCR: uint = 0x010;
const SSIFSR: uint = 0x014;
const SSIFTDR: uint = 0x018;
const SSIFRDR: uint = 0x01c;
const SSIOFR: uint = 0x020;
const SSISCR: uint = 0x024;

/* SSI REGISTER BITS */
const fn SSICR_DWL(x: u32) -> u32 {
    ((x & 0x7) << 19)
}
const fn SSICR_SWL(x: u32) -> u32 {
    ((x & 0x7) << 16)
}

const SSICR_CKS: u32 = BIT(30);
const SSICR_TUIEN: u32 = BIT(29);
const SSICR_TOIEN: u32 = BIT(28);
const SSICR_RUIEN: u32 = BIT(27);
const SSICR_ROIEN: u32 = BIT(26);
const SSICR_MST: u32 = BIT(14);
const SSICR_BCKP: u32 = BIT(13);
const SSICR_LRCKP: u32 = BIT(12);
const SSICR_PDTA: u32 = BIT(9);
const fn SSICR_CKDV(x: u32) -> u32 {
    ((x & 0xf) << 4)
}
const SSICR_TEN: u32 = BIT(1);
const SSICR_REN: u32 = BIT(0);

const SSISR_TUIRQ: u32 = BIT(29);
const SSISR_TOIRQ: u32 = BIT(28);
const SSISR_RUIRQ: u32 = BIT(27);
const SSISR_ROIRQ: u32 = BIT(26);
const SSISR_IIRQ: u32 = BIT(25);

const SSIFCR_AUCKE: u32 = BIT(31);
const SSIFCR_SSIRST: u32 = BIT(16);
const SSIFCR_TIE: u32 = BIT(3);
const SSIFCR_RIE: u32 = BIT(2);
const SSIFCR_TFRST: u32 = BIT(1);
const SSIFCR_RFRST: u32 = BIT(0);
const SSIFCR_FIFO_RST: u32 = SSIFCR_TFRST | SSIFCR_RFRST;

const SSIFSR_TDC_MASK: u32 = 0x3f;
const SSIFSR_TDC_SHIFT: u32 = 24;
const SSIFSR_RDC_MASK: u32 = 0x3f;
const SSIFSR_RDC_SHIFT: u32 = 8;

const SSIFSR_TDE: u32 = BIT(16);
const SSIFSR_RDF: u32 = BIT(0);

const SSIOFR_LRCONT: u32 = BIT(8);

const fn SSISCR_TDES(x: i32) -> u32 {
    (((x as u32) & 0x1f) << 8)
}
const fn SSISCR_RDFS(x: u32) -> u32 {
    ((x & 0x1f) << 0)
}

/* Pre allocated buffers sizes */
const PREALLOC_BUFFER: usize = SZ_32K;
const PREALLOC_BUFFER_MAX: usize = SZ_32K;

const SSI_RATES: u32 = SNDRV_PCM_RATE_8000_48000; /* 8k-48kHz */
const SSI_FMTS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;
const SSI_CHAN_MIN: u32 = 2;
const SSI_CHAN_MAX: u32 = 2;
const SSI_FIFO_DEPTH: i32 = 32;

#[repr(C)]
struct rz_ssi_stream {
    priv_: *mut rz_ssi_priv,
    substream: *mut snd_pcm_substream,
    fifo_sample_size: c_int, /* sample capacity of SSI FIFO */
    period_counter: c_int,   /* for keeping track of periods transferred */
    buffer_pos: c_int,       /* current frame position in the buffer */
    running: c_int,          /* 0=stopped, 1=running */

    uerr_num: c_int,
    oerr_num: c_int,

    transfer: Option<unsafe extern "C" fn(*mut rz_ssi_priv, *mut rz_ssi_stream) -> c_int>,
}

#[repr(C)]
struct rz_ssi_priv {
    base: *mut c_void,
    rstc: *mut reset_control,
    dev: *mut device,
    sfr_clk: *mut clk,
    clk: *mut clk,

    irq_int: c_int,
    irq_tx: c_int,
    irq_rx: c_int,
    irq_rt: c_int,

    lock: spinlock_t,

    /*
     * The SSI supports full-duplex transmission and reception.
     * However, if an error occurs, channel reset (both transmission
     * and reception reset) is required.
     * So it is better to use as half-duplex (playing and recording
     * should be done on separate channels).
     */
    playback: rz_ssi_stream,
    capture: rz_ssi_stream,

    /* clock */
    audio_mck: c_ulong,
    audio_clk_1: c_ulong,
    audio_clk_2: c_ulong,

    lrckp_fsync_fall: bool_, /* LR clock polarity (SSICR.LRCKP) */
    bckp_rise: bool_,        /* Bit clock polarity (SSICR.BCKP) */
    dma_rt: bool_,

    dup: rz_ssi_priv_dup,

    /* Full duplex communication support */
    hw_params_cache: rz_ssi_hw_params_cache,

    dma_dais: [snd_dmaengine_dai_dma_data; SNDRV_PCM_STREAM_LAST + 1],
    dmas: [*mut dma_chan; SNDRV_PCM_STREAM_LAST + 1],
}

#[repr(C)]
struct rz_ssi_priv_dup {
    tx_active: bool_,
    rx_active: bool_,
    one_stream_triggered: bool_,
}

#[repr(C)]
struct rz_ssi_hw_params_cache {
    rate: c_uint,
    channels: c_uint,
    sample_width: c_uint,
    sample_bits: c_uint,
}

unsafe fn rz_ssi_reg_writel(priv_: *mut rz_ssi_priv, reg: uint, data: u32) {
    unsafe { writel(data, ((*priv_).base as *mut u8).add(reg as usize) as *mut c_void) };
}

unsafe fn rz_ssi_reg_readl(priv_: *mut rz_ssi_priv, reg: uint) -> u32 {
    unsafe { readl(((*priv_).base as *mut u8).add(reg as usize) as *mut c_void) }
}

unsafe fn rz_ssi_reg_mask_setl(priv_: *mut rz_ssi_priv, reg: uint, bclr: u32, bset: u32) {
    let mut val: u32;

    unsafe {
        val = readl(((*priv_).base as *mut u8).add(reg as usize) as *mut c_void);
        val = (val & !bclr) | bset;
        writel(val, ((*priv_).base as *mut u8).add(reg as usize) as *mut c_void);
    }
}

unsafe fn rz_ssi_stream_get(
    ssi: *mut rz_ssi_priv,
    substream: *mut snd_pcm_substream,
) -> *mut rz_ssi_stream {
    unsafe {
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK as c_int {
            &mut (*ssi).playback
        } else {
            &mut (*ssi).capture
        }
    }
}

unsafe fn rz_ssi_is_dma_enabled(ssi: *mut rz_ssi_priv) -> bool_ {
    unsafe { (*ssi).playback.transfer.is_none() && (*ssi).capture.transfer.is_none() }
}

unsafe fn rz_ssi_set_substream(strm: *mut rz_ssi_stream, substream: *mut snd_pcm_substream) {
    unsafe {
        let ssi = (*strm).priv_;

        /* guard(spinlock_irqsave)(&ssi->lock); */
        (*strm).substream = substream;
        let _ = ssi;
    }
}

unsafe fn rz_ssi_stream_is_valid(ssi: *mut rz_ssi_priv, strm: *mut rz_ssi_stream) -> bool_ {
    unsafe {
        /* guard(spinlock_irqsave)(&ssi->lock); */
        let _ = ssi;
        !(*strm).substream.is_null() && !(*(*strm).substream).runtime.is_null()
    }
}

unsafe fn rz_ssi_is_stream_running(strm: *mut rz_ssi_stream) -> bool_ {
    unsafe { !(*strm).substream.is_null() && (*strm).running != 0 }
}

unsafe fn rz_ssi_stream_init(strm: *mut rz_ssi_stream, substream: *mut snd_pcm_substream) {
    unsafe {
        rz_ssi_set_substream(strm, substream);
        (*strm).period_counter = 0;
        (*strm).buffer_pos = 0;

        (*strm).oerr_num = 0;
        (*strm).uerr_num = 0;
        (*strm).running = 0;

        /* fifo init */
        (*strm).fifo_sample_size = SSI_FIFO_DEPTH;
    }
}

unsafe fn rz_ssi_stream_quit(ssi: *mut rz_ssi_priv, strm: *mut rz_ssi_stream) {
    unsafe {
        let dev = (*ssi).dev;

        rz_ssi_set_substream(strm, core::ptr::null_mut());

        if (*strm).oerr_num > 0 {
            dev_info(dev, c"overrun = %d\n".as_ptr(), (*strm).oerr_num);
        }

        if (*strm).uerr_num > 0 {
            dev_info(dev, c"underrun = %d\n".as_ptr(), (*strm).uerr_num);
        }
    }
}

unsafe fn rz_ssi_clk_setup(
    ssi: *mut rz_ssi_priv,
    substream: *mut snd_pcm_substream,
    rate: c_uint,
    channels: c_uint,
) -> c_int {
    static ckdv: [u8; 13] = [1, 2, 4, 8, 16, 32, 64, 128, 6, 12, 24, 48, 96];
    let channel_bits: c_uint = 32; /* System Word Length */
    let bclk_rate: c_ulong = (rate * channels * channel_bits) as c_ulong;
    let dma_dai: *mut snd_dmaengine_dai_dma_data;
    let mut div: c_uint;
    let mut i: c_uint;
    let mut ssicr: u32 = 0;
    let mut clk_ckdv: u32;

    unsafe {
        /* Clear AUCKE so we can set MST */
        rz_ssi_reg_writel(ssi, SSIFCR, 0);

        /* Continue to output LRCK pin even when idle */
        rz_ssi_reg_writel(ssi, SSIOFR, SSIOFR_LRCONT);
        if (*ssi).audio_clk_1 != 0 && (*ssi).audio_clk_2 != 0 {
            if (*ssi).audio_clk_1 % bclk_rate != 0 {
                (*ssi).audio_mck = (*ssi).audio_clk_2;
            } else {
                (*ssi).audio_mck = (*ssi).audio_clk_1;
            }
        }

        /* Clock setting */
        ssicr |= SSICR_MST;
        if (*ssi).audio_mck == (*ssi).audio_clk_1 {
            ssicr |= SSICR_CKS;
        }
        if (*ssi).bckp_rise {
            ssicr |= SSICR_BCKP;
        }
        if (*ssi).lrckp_fsync_fall {
            ssicr |= SSICR_LRCKP;
        }

        /* Determine the clock divider */
        clk_ckdv = 0;
        div = ((*ssi).audio_mck / bclk_rate) as c_uint;
        /* try to find an match */
        i = 0;
        while (i as usize) < ckdv.len() {
            if ckdv[i as usize] as c_uint == div {
                clk_ckdv = i;
                break;
            }
            i += 1;
        }

        if i as usize == ckdv.len() {
            dev_err((*ssi).dev, c"Rate not divisible by audio clock source\n".as_ptr());
            return -EINVAL;
        }

        dma_dai = (*ssi).dma_dais.as_mut_ptr().add((*substream).stream as usize);

        /*
         * DWL: Data Word Length = {16, 24, 32} bits
         * SWL: System Word Length = 32 bits
         */
        ssicr |= SSICR_CKDV(clk_ckdv);
        match (*ssi).hw_params_cache.sample_width {
            16 => {
                ssicr |= SSICR_DWL(1);
                (*dma_dai).addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
            }
            24 => {
                ssicr |= SSICR_DWL(5) | SSICR_PDTA;
                (*dma_dai).addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
            }
            32 => {
                ssicr |= SSICR_DWL(6);
                (*dma_dai).addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
            }
            _ => {
                dev_err(
                    (*ssi).dev,
                    c"Not support %u data width".as_ptr(),
                    (*ssi).hw_params_cache.sample_width,
                );
                return -EINVAL;
            }
        }

        ssicr |= SSICR_SWL(3);
        rz_ssi_reg_writel(ssi, SSICR, ssicr);
        rz_ssi_reg_writel(ssi, SSIFCR, SSIFCR_AUCKE | SSIFCR_FIFO_RST);

        0
    }
}

unsafe fn rz_ssi_set_idle(ssi: *mut rz_ssi_priv) {
    let mut tmp: u32 = 0;
    let ret: c_int;

    unsafe {
        /* Disable irqs */
        rz_ssi_reg_mask_setl(ssi, SSICR, SSICR_TUIEN | SSICR_TOIEN | SSICR_RUIEN | SSICR_ROIEN, 0);
        rz_ssi_reg_mask_setl(ssi, SSIFCR, SSIFCR_TIE | SSIFCR_RIE, 0);

        /* Clear all error flags */
        rz_ssi_reg_mask_setl(ssi, SSISR, SSISR_TOIRQ | SSISR_TUIRQ | SSISR_ROIRQ | SSISR_RUIRQ, 0);

        /* Wait for idle */
        ret = readl_poll_timeout_atomic(
            ((*ssi).base as *mut u8).add(SSISR as usize) as *mut c_void,
            &mut tmp,
            SSISR_IIRQ,
            1,
            100,
        );
        if ret != 0 {
            dev_warn_ratelimited((*ssi).dev, c"timeout waiting for SSI idle\n".as_ptr());
        }

        /* Hold FIFOs in reset */
        rz_ssi_reg_mask_setl(ssi, SSIFCR, 0, SSIFCR_FIFO_RST);
    }
}

unsafe fn rz_ssi_start(ssi: *mut rz_ssi_priv, strm: *mut rz_ssi_stream) -> c_int {
    unsafe {
        let is_play = (*(*strm).substream).stream == SNDRV_PCM_STREAM_PLAYBACK as c_int;
        let is_full_duplex: bool_;
        let mut ssicr: u32;
        let mut ssifcr: u32;

        is_full_duplex = (*ssi).dup.tx_active && (*ssi).dup.rx_active;
        ssicr = rz_ssi_reg_readl(ssi, SSICR);
        ssifcr = rz_ssi_reg_readl(ssi, SSIFCR);
        if !is_full_duplex {
            ssifcr &= !0xF;
        } else if (*ssi).dup.one_stream_triggered {
            rz_ssi_reg_mask_setl(ssi, SSICR, SSICR_TEN | SSICR_REN, 0);
            rz_ssi_set_idle(ssi);
            ssifcr &= !SSIFCR_FIFO_RST;
        }

        /* FIFO interrupt thresholds */
        if rz_ssi_is_dma_enabled(ssi) {
            rz_ssi_reg_writel(ssi, SSISCR, 0);
        } else {
            rz_ssi_reg_writel(
                ssi,
                SSISCR,
                SSISCR_TDES((*strm).fifo_sample_size / 2 - 1) | SSISCR_RDFS(0),
            );
        }

        /* enable IRQ */
        if is_play {
            ssicr |= SSICR_TUIEN | SSICR_TOIEN;
            ssifcr |= SSIFCR_TIE;
            if !is_full_duplex {
                ssifcr |= SSIFCR_RFRST;
            }
        } else {
            ssicr |= SSICR_RUIEN | SSICR_ROIEN;
            ssifcr |= SSIFCR_RIE;
            if !is_full_duplex {
                ssifcr |= SSIFCR_TFRST;
            }
        }

        rz_ssi_reg_writel(ssi, SSICR, ssicr);
        rz_ssi_reg_writel(ssi, SSIFCR, ssifcr);

        /* Clear all error flags */
        rz_ssi_reg_mask_setl(ssi, SSISR, SSISR_TOIRQ | SSISR_TUIRQ | SSISR_ROIRQ | SSISR_RUIRQ, 0);

        (*strm).running = 1;
        if !is_full_duplex {
            ssicr |= if is_play { SSICR_TEN } else { SSICR_REN };
            rz_ssi_reg_writel(ssi, SSICR, ssicr);
        } else if (*ssi).dup.one_stream_triggered {
            ssicr |= SSICR_TEN | SSICR_REN;
            rz_ssi_reg_writel(ssi, SSICR, ssicr);
            (*ssi).dup.one_stream_triggered = false;
        } else {
            (*ssi).dup.one_stream_triggered = true;
        }

        0
    }
}

unsafe fn rz_ssi_swreset(ssi: *mut rz_ssi_priv) -> c_int {
    let mut tmp: u32 = 0;

    unsafe {
        rz_ssi_reg_mask_setl(ssi, SSIFCR, 0, SSIFCR_SSIRST);
        rz_ssi_reg_mask_setl(ssi, SSIFCR, SSIFCR_SSIRST, 0);
        readl_poll_timeout_atomic(
            ((*ssi).base as *mut u8).add(SSIFCR as usize) as *mut c_void,
            &mut tmp,
            !SSIFCR_SSIRST,
            1,
            5,
        )
    }
}

unsafe fn rz_ssi_stop(ssi: *mut rz_ssi_priv, strm: *mut rz_ssi_stream) -> c_int {
    unsafe {
        (*strm).running = 0;

        if rz_ssi_is_stream_running(&mut (*ssi).playback) || rz_ssi_is_stream_running(&mut (*ssi).capture) {
            return 0;
        }

        /* Disable TX/RX */
        rz_ssi_reg_mask_setl(ssi, SSICR, SSICR_TEN | SSICR_REN, 0);

        rz_ssi_set_idle(ssi);

        0
    }
}

unsafe fn rz_ssi_pointer_update(strm: *mut rz_ssi_stream, frames: c_int) {
    unsafe {
        let substream = (*strm).substream;
        let runtime: *mut snd_pcm_runtime;
        let current_period: c_int;

        if (*strm).running == 0 || substream.is_null() || (*substream).runtime.is_null() {
            return;
        }

        runtime = (*substream).runtime;
        (*strm).buffer_pos += frames;
        WARN_ON((*strm).buffer_pos > (*runtime).buffer_size as c_int);

        /* ring buffer */
        if (*strm).buffer_pos == (*runtime).buffer_size as c_int {
            (*strm).buffer_pos = 0;
        }

        current_period = (*strm).buffer_pos / (*runtime).period_size as c_int;
        if (*strm).period_counter != current_period {
            snd_pcm_period_elapsed((*strm).substream);
            (*strm).period_counter = current_period;
        }
    }
}

unsafe extern "C" fn rz_ssi_pio_recv(ssi: *mut rz_ssi_priv, strm: *mut rz_ssi_stream) -> c_int {
    unsafe {
        let substream = (*strm).substream;
        let runtime: *mut snd_pcm_runtime;
        let mut fifo_samples: c_int;
        let mut frames_left: c_int;
        let mut samples: c_int;
        let mut i: c_int;

        if !rz_ssi_stream_is_valid(ssi, strm) {
            return -EINVAL;
        }

        runtime = (*substream).runtime;

        loop {
            /* frames left in this period */
            frames_left = (*runtime).period_size as c_int - ((*strm).buffer_pos % (*runtime).period_size as c_int);
            if frames_left == 0 {
                frames_left = (*runtime).period_size as c_int;
            }

            /* Samples in RX FIFO */
            fifo_samples = ((rz_ssi_reg_readl(ssi, SSIFSR) >> SSIFSR_RDC_SHIFT) & SSIFSR_RDC_MASK) as c_int;

            /* Only read full frames at a time */
            samples = 0;
            while frames_left != 0 && fifo_samples >= (*runtime).channels as c_int {
                samples += (*runtime).channels as c_int;
                fifo_samples -= (*runtime).channels as c_int;
                frames_left -= 1;
            }

            /* not enough samples yet */
            if samples == 0 {
                break;
            }

            /* calculate new buffer index */
            if (*ssi).hw_params_cache.sample_width == 16 {
                let mut buf: *mut u16;

                buf = (*runtime).dma_area as *mut u16;
                buf = buf.add(((*strm).buffer_pos as c_uint * (*runtime).channels) as usize);

                i = 0;
                while i < samples {
                    *buf = (rz_ssi_reg_readl(ssi, SSIFRDR) >> 16) as u16;
                    buf = buf.add(1);
                    i += 1;
                }
            } else {
                let mut buf: *mut u32;

                buf = (*runtime).dma_area as *mut u32;
                buf = buf.add(((*strm).buffer_pos as c_uint * (*runtime).channels) as usize);

                i = 0;
                while i < samples {
                    *buf = rz_ssi_reg_readl(ssi, SSIFRDR);
                    buf = buf.add(1);
                    i += 1;
                }
            }

            rz_ssi_reg_mask_setl(ssi, SSIFSR, SSIFSR_RDF, 0);
            rz_ssi_pointer_update(strm, samples / (*runtime).channels as c_int);
            if !(!frames_left != false && fifo_samples >= (*runtime).channels as c_int) {
                if !(frames_left == 0 && fifo_samples >= (*runtime).channels as c_int) {
                    break;
                }
            }
            if !(frames_left == 0 && fifo_samples >= (*runtime).channels as c_int) {
                break;
            }
        }

        0
    }
}

unsafe extern "C" fn rz_ssi_pio_send(ssi: *mut rz_ssi_priv, strm: *mut rz_ssi_stream) -> c_int {
    unsafe {
        let substream = (*strm).substream;
        let runtime = (*substream).runtime;
        let mut sample_space: c_int;
        let mut samples: c_int = 0;
        let mut frames_left: c_int;
        let mut i: c_int;
        let ssifsr: u32;

        if !rz_ssi_stream_is_valid(ssi, strm) {
            return -EINVAL;
        }

        /* frames left in this period */
        frames_left = (*runtime).period_size as c_int - ((*strm).buffer_pos % (*runtime).period_size as c_int);
        if frames_left == 0 {
            frames_left = (*runtime).period_size as c_int;
        }

        sample_space = (*strm).fifo_sample_size;
        ssifsr = rz_ssi_reg_readl(ssi, SSIFSR);
        sample_space -= ((ssifsr >> SSIFSR_TDC_SHIFT) & SSIFSR_TDC_MASK) as c_int;
        if sample_space < 0 {
            return -EINVAL;
        }

        /* Only add full frames at a time */
        while frames_left != 0 && sample_space >= (*runtime).channels as c_int {
            samples += (*runtime).channels as c_int;
            sample_space -= (*runtime).channels as c_int;
            frames_left -= 1;
        }

        /* no space to send anything right now */
        if samples == 0 {
            return 0;
        }

        /* calculate new buffer index */
        if (*ssi).hw_params_cache.sample_width == 16 {
            let mut buf: *mut u16;

            buf = (*runtime).dma_area as *mut u16;
            buf = buf.add(((*strm).buffer_pos as c_uint * (*runtime).channels) as usize);

            i = 0;
            while i < samples {
                rz_ssi_reg_writel(ssi, SSIFTDR, (*buf as u32) << 16);
                buf = buf.add(1);
                i += 1;
            }
        } else {
            let mut buf: *mut u32;

            buf = (*runtime).dma_area as *mut u32;
            buf = buf.add(((*strm).buffer_pos as c_uint * (*runtime).channels) as usize);

            i = 0;
            while i < samples {
                rz_ssi_reg_writel(ssi, SSIFTDR, *buf);
                buf = buf.add(1);
                i += 1;
            }
        }

        rz_ssi_reg_mask_setl(ssi, SSIFSR, SSIFSR_TDE, 0);
        rz_ssi_pointer_update(strm, samples / (*runtime).channels as c_int);

        0
    }
}

unsafe extern "C" fn rz_ssi_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    unsafe {
        let mut strm_playback: *mut rz_ssi_stream = core::ptr::null_mut();
        let mut strm_capture: *mut rz_ssi_stream = core::ptr::null_mut();
        let ssi = data as *mut rz_ssi_priv;
        let ssisr: u32 = rz_ssi_reg_readl(ssi, SSISR);

        if !(*ssi).playback.substream.is_null() {
            strm_playback = &mut (*ssi).playback;
        }
        if !(*ssi).capture.substream.is_null() {
            strm_capture = &mut (*ssi).capture;
        }

        if strm_playback.is_null() && strm_capture.is_null() {
            return IRQ_HANDLED; /* Left over TX/RX interrupt */
        }

        if irq == (*ssi).irq_int {
            /* error or idle */
            let is_stopped = (ssisr & (SSISR_RUIRQ | SSISR_ROIRQ | SSISR_TUIRQ | SSISR_TOIRQ)) != 0;

            if !(*ssi).capture.substream.is_null() && is_stopped {
                if ssisr & SSISR_RUIRQ != 0 {
                    (*strm_capture).uerr_num += 1;
                }
                if ssisr & SSISR_ROIRQ != 0 {
                    (*strm_capture).oerr_num += 1;
                }

                rz_ssi_stop(ssi, strm_capture);
            }

            if !(*ssi).playback.substream.is_null() && is_stopped {
                if ssisr & SSISR_TUIRQ != 0 {
                    (*strm_playback).uerr_num += 1;
                }
                if ssisr & SSISR_TOIRQ != 0 {
                    (*strm_playback).oerr_num += 1;
                }

                rz_ssi_stop(ssi, strm_playback);
            }

            if !rz_ssi_is_stream_running(&mut (*ssi).playback)
                && !rz_ssi_is_stream_running(&mut (*ssi).capture)
                && rz_ssi_is_dma_enabled(ssi)
                && is_stopped
            {
                if !(*ssi).playback.substream.is_null() && !(*ssi).dmas[SNDRV_PCM_STREAM_PLAYBACK].is_null() {
                    dmaengine_pause((*ssi).dmas[SNDRV_PCM_STREAM_PLAYBACK]);
                }
                if !(*ssi).capture.substream.is_null()
                    && !(*ssi).dmas[SNDRV_PCM_STREAM_CAPTURE].is_null()
                    &&
                    /* Avoid calling pause twice in case of half duplex. */
                    (*ssi).dmas[SNDRV_PCM_STREAM_PLAYBACK] != (*ssi).dmas[SNDRV_PCM_STREAM_CAPTURE]
                {
                    dmaengine_pause((*ssi).dmas[SNDRV_PCM_STREAM_CAPTURE]);
                }
            }

            /* Clear all flags */
            rz_ssi_reg_mask_setl(ssi, SSISR, SSISR_TOIRQ | SSISR_TUIRQ | SSISR_ROIRQ | SSISR_RUIRQ, 0);

            /* Add/remove more data */
            if !(*ssi).capture.substream.is_null() && is_stopped {
                if rz_ssi_is_dma_enabled(ssi) {
                    if !(*ssi).dmas[SNDRV_PCM_STREAM_CAPTURE].is_null() {
                        dmaengine_resume((*ssi).dmas[SNDRV_PCM_STREAM_CAPTURE]);
                    }
                } else if let Some(transfer) = (*strm_capture).transfer {
                    transfer(ssi, strm_capture);
                }
            }

            if !(*ssi).playback.substream.is_null() && is_stopped {
                if rz_ssi_is_dma_enabled(ssi) {
                    if !(*ssi).dmas[SNDRV_PCM_STREAM_PLAYBACK].is_null() {
                        dmaengine_resume((*ssi).dmas[SNDRV_PCM_STREAM_PLAYBACK]);
                    }
                } else if let Some(transfer) = (*strm_playback).transfer {
                    transfer(ssi, strm_playback);
                }
            }

            /* Resume */
            if !(*ssi).playback.substream.is_null() && is_stopped {
                rz_ssi_start(ssi, &mut (*ssi).playback);
            }
            if !(*ssi).capture.substream.is_null() && is_stopped {
                rz_ssi_start(ssi, &mut (*ssi).capture);
            }
        }

        if !rz_ssi_is_stream_running(&mut (*ssi).playback) && !rz_ssi_is_stream_running(&mut (*ssi).capture) {
            return IRQ_HANDLED;
        }

        /* tx data empty */
        if irq == (*ssi).irq_tx && rz_ssi_is_stream_running(&mut (*ssi).playback) {
            if let Some(transfer) = (*strm_playback).transfer {
                transfer(ssi, &mut (*ssi).playback);
            }
        }

        /* rx data full */
        if irq == (*ssi).irq_rx && rz_ssi_is_stream_running(&mut (*ssi).capture) {
            if let Some(transfer) = (*strm_capture).transfer {
                transfer(ssi, &mut (*ssi).capture);
            }
            rz_ssi_reg_mask_setl(ssi, SSIFSR, SSIFSR_RDF, 0);
        }

        if irq == (*ssi).irq_rt {
            if !(*ssi).playback.substream.is_null() {
                if let Some(transfer) = (*strm_playback).transfer {
                    transfer(ssi, &mut (*ssi).playback);
                }
            } else {
                if let Some(transfer) = (*strm_capture).transfer {
                    transfer(ssi, &mut (*ssi).capture);
                }
                rz_ssi_reg_mask_setl(ssi, SSIFSR, SSIFSR_RDF, 0);
            }
        }

        IRQ_HANDLED
    }
}

unsafe fn rz_ssi_trigger_resume(ssi: *mut rz_ssi_priv, strm: *mut rz_ssi_stream) -> c_int {
    unsafe {
        let substream = (*strm).substream;
        let ret: c_int;

        if rz_ssi_is_stream_running(&mut (*ssi).playback) || rz_ssi_is_stream_running(&mut (*ssi).capture) {
            return 0;
        }

        ret = rz_ssi_swreset(ssi);
        if ret != 0 {
            return ret;
        }

        rz_ssi_clk_setup(
            ssi,
            substream,
            (*ssi).hw_params_cache.rate,
            (*ssi).hw_params_cache.channels,
        )
    }
}

unsafe extern "C" fn rz_ssi_dai_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let ssi = snd_soc_dai_get_drvdata(dai) as *mut rz_ssi_priv;
        let strm = rz_ssi_stream_get(ssi, substream);
        let mut ret: c_int = 0;

        match cmd {
            SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
                ret = rz_ssi_trigger_resume(ssi, strm);
                if ret != 0 {
                    return ret;
                }

                /* fallthrough */
                if cmd == SNDRV_PCM_TRIGGER_START {
                    rz_ssi_stream_init(strm, substream);
                }

                if !rz_ssi_is_dma_enabled(ssi) {
                    if let Some(transfer) = (*strm).transfer {
                        ret = transfer(ssi, strm);
                    }
                    if ret != 0 {
                        return ret;
                    }
                }

                ret = rz_ssi_start(ssi, strm);
            }
            SNDRV_PCM_TRIGGER_START => {
                if cmd == SNDRV_PCM_TRIGGER_START {
                    rz_ssi_stream_init(strm, substream);
                }

                if !rz_ssi_is_dma_enabled(ssi) {
                    if let Some(transfer) = (*strm).transfer {
                        ret = transfer(ssi, strm);
                    }
                    if ret != 0 {
                        return ret;
                    }
                }

                ret = rz_ssi_start(ssi, strm);
            }
            SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
                rz_ssi_stop(ssi, strm);
            }
            SNDRV_PCM_TRIGGER_STOP => {
                rz_ssi_stop(ssi, strm);
                rz_ssi_stream_quit(ssi, strm);
            }
            _ => {}
        }

        ret
    }
}

unsafe extern "C" fn rz_ssi_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    unsafe {
        let ssi = snd_soc_dai_get_drvdata(dai) as *mut rz_ssi_priv;

        match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
            SND_SOC_DAIFMT_BP_FP => {}
            _ => {
                dev_err((*ssi).dev, c"Codec should be clk and frame consumer\n".as_ptr());
                return -EINVAL;
            }
        }

        /*
         * set clock polarity
         *
         * "normal" BCLK = Signal is available at rising edge of BCLK
         * "normal" FSYNC = (I2S) Left ch starts with falling FSYNC edge
         */
        match fmt & SND_SOC_DAIFMT_INV_MASK {
            SND_SOC_DAIFMT_NB_NF => {
                (*ssi).bckp_rise = false;
                (*ssi).lrckp_fsync_fall = false;
            }
            SND_SOC_DAIFMT_NB_IF => {
                (*ssi).bckp_rise = false;
                (*ssi).lrckp_fsync_fall = true;
            }
            SND_SOC_DAIFMT_IB_NF => {
                (*ssi).bckp_rise = true;
                (*ssi).lrckp_fsync_fall = false;
            }
            SND_SOC_DAIFMT_IB_IF => {
                (*ssi).bckp_rise = true;
                (*ssi).lrckp_fsync_fall = true;
            }
            _ => return -EINVAL,
        }

        /* only i2s support */
        match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
            SND_SOC_DAIFMT_I2S => {}
            _ => {
                dev_err((*ssi).dev, c"Only I2S mode is supported.\n".as_ptr());
                return -EINVAL;
            }
        }

        0
    }
}

unsafe extern "C" fn rz_ssi_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    unsafe {
        let ssi = snd_soc_dai_get_drvdata(dai) as *mut rz_ssi_priv;

        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK as c_int {
            (*ssi).dup.tx_active = true;
        } else {
            (*ssi).dup.rx_active = true;
        }

        0
    }
}

unsafe extern "C" fn rz_ssi_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    unsafe {
        let ssi = snd_soc_dai_get_drvdata(dai) as *mut rz_ssi_priv;

        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK as c_int {
            (*ssi).dup.tx_active = false;
        } else {
            (*ssi).dup.rx_active = false;
        }

        (*ssi).dmas[(*substream).stream as usize] = core::ptr::null_mut();
    }
}

unsafe fn rz_ssi_is_valid_hw_params(
    ssi: *mut rz_ssi_priv,
    rate: c_uint,
    channels: c_uint,
    sample_width: c_uint,
    sample_bits: c_uint,
) -> bool_ {
    unsafe {
        if (*ssi).hw_params_cache.rate != rate
            || (*ssi).hw_params_cache.channels != channels
            || (*ssi).hw_params_cache.sample_width != sample_width
            || (*ssi).hw_params_cache.sample_bits != sample_bits
        {
            return false;
        }

        true
    }
}

unsafe fn rz_ssi_cache_hw_params(
    ssi: *mut rz_ssi_priv,
    rate: c_uint,
    channels: c_uint,
    sample_width: c_uint,
    sample_bits: c_uint,
) {
    unsafe {
        (*ssi).hw_params_cache.rate = rate;
        (*ssi).hw_params_cache.channels = channels;
        (*ssi).hw_params_cache.sample_width = sample_width;
        (*ssi).hw_params_cache.sample_bits = sample_bits;
    }
}

unsafe extern "C" fn rz_ssi_dai_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let ssi = snd_soc_dai_get_drvdata(dai) as *mut rz_ssi_priv;
        let sample_bits = (*hw_param_interval(params, SNDRV_PCM_HW_PARAM_SAMPLE_BITS)).min;
        let sample_width = params_width(params);
        let channels = params_channels(params);
        let rate = params_rate(params);
        let ret: c_int;

        if !(sample_bits == 16 || sample_bits == 24 || sample_bits == 32) {
            dev_err((*ssi).dev, c"Unsupported sample width: %d\n".as_ptr(), sample_bits);
            return -EINVAL;
        }

        if channels != 2 {
            dev_err((*ssi).dev, c"Number of channels not matched: %d\n".as_ptr(), channels);
            return -EINVAL;
        }

        /* Save the DMA channels for recovery. */
        if rz_ssi_is_dma_enabled(ssi) {
            (*ssi).dmas[(*substream).stream as usize] = snd_dmaengine_pcm_get_chan(substream);
        } else {
            (*ssi).dmas[(*substream).stream as usize] = core::ptr::null_mut();
        }

        if rz_ssi_is_stream_running(&mut (*ssi).playback) || rz_ssi_is_stream_running(&mut (*ssi).capture) {
            if rz_ssi_is_valid_hw_params(ssi, rate, channels, sample_width, sample_bits) {
                return 0;
            }

            dev_err((*ssi).dev, c"Full duplex needs same HW params\n".as_ptr());
            return -EINVAL;
        }

        rz_ssi_cache_hw_params(ssi, rate, channels, sample_width, sample_bits);

        ret = rz_ssi_swreset(ssi);
        if ret != 0 {
            return ret;
        }

        rz_ssi_clk_setup(ssi, substream, rate, channels)
    }
}

unsafe extern "C" fn rz_ssi_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    unsafe {
        let ssi = snd_soc_dai_get_drvdata(dai) as *mut rz_ssi_priv;

        snd_soc_dai_init_dma_data(
            dai,
            &mut (*ssi).dma_dais[SNDRV_PCM_STREAM_PLAYBACK],
            &mut (*ssi).dma_dais[SNDRV_PCM_STREAM_CAPTURE],
        );

        0
    }
}

static rz_ssi_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(rz_ssi_dai_probe),
    startup: Some(rz_ssi_startup),
    shutdown: Some(rz_ssi_shutdown),
    trigger: Some(rz_ssi_dai_trigger),
    set_fmt: Some(rz_ssi_dai_set_fmt),
    hw_params: Some(rz_ssi_dai_hw_params),
};

static rz_ssi_pcm_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_RESUME
        | SNDRV_PCM_INFO_PAUSE,
    buffer_bytes_max: 192 * 1024,
    period_bytes_min: 32,
    period_bytes_max: 48 * 1024,
    channels_min: SSI_CHAN_MIN,
    channels_max: SSI_CHAN_MAX,
    periods_min: 1,
    periods_max: 32,
    fifo_size: 32 * 2,
};

unsafe extern "C" fn rz_ssi_pcm_open_pio(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    unsafe {
        snd_soc_set_runtime_hwparams(substream, &rz_ssi_pcm_hardware);

        snd_pcm_hw_constraint_integer((*substream).runtime, SNDRV_PCM_HW_PARAM_PERIODS)
    }
}

unsafe extern "C" fn rz_ssi_pcm_open_dma(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    unsafe { snd_pcm_hw_constraint_integer((*substream).runtime, SNDRV_PCM_HW_PARAM_PERIODS) }
}

unsafe extern "C" fn rz_ssi_pcm_pointer(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    unsafe {
        let rtd = snd_soc_substream_to_rtd(substream);
        let dai = snd_soc_rtd_to_cpu(rtd, 0);
        let ssi = snd_soc_dai_get_drvdata(dai) as *mut rz_ssi_priv;
        let strm = rz_ssi_stream_get(ssi, substream);

        (*strm).buffer_pos as snd_pcm_uframes_t
    }
}

unsafe extern "C" fn rz_ssi_pcm_new(
    _component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    unsafe {
        snd_pcm_set_managed_buffer_all(
            (*rtd).pcm,
            SNDRV_DMA_TYPE_DEV,
            (*(*(*rtd).card).snd_card).dev,
            rz_ssi_pcm_hardware.buffer_bytes_max,
            rz_ssi_pcm_hardware.buffer_bytes_max,
        );
        0
    }
}

static mut rz_ssi_soc_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"rz-ssi-dai".as_ptr(),
    playback: snd_soc_pcm_stream {
        rates: SSI_RATES,
        formats: SSI_FMTS,
        channels_min: SSI_CHAN_MIN,
        channels_max: SSI_CHAN_MAX,
    },
    capture: snd_soc_pcm_stream {
        rates: SSI_RATES,
        formats: SSI_FMTS,
        channels_min: SSI_CHAN_MIN,
        channels_max: SSI_CHAN_MAX,
    },
    ops: &rz_ssi_dai_ops,
}];

static rz_ssi_soc_component_pio: snd_soc_component_driver = snd_soc_component_driver {
    name: c"rz-ssi".as_ptr(),
    open: Some(rz_ssi_pcm_open_pio),
    pointer: Some(rz_ssi_pcm_pointer),
    pcm_new: Some(rz_ssi_pcm_new),
    legacy_dai_naming: 1,
};

static rz_ssi_soc_component_dma: snd_soc_component_driver = snd_soc_component_driver {
    name: c"rz-ssi".as_ptr(),
    open: Some(rz_ssi_pcm_open_dma),
    pointer: None,
    pcm_new: None,
    legacy_dai_naming: 1,
};

static rz_ssi_dmaengine_pcm_conf: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    pcm_hardware: &rz_ssi_pcm_hardware,
    prealloc_buffer_size: 192 * 1024,
    prepare_slave_config: Some(snd_dmaengine_pcm_prepare_slave_config),
    chan_names: [core::ptr::null(); SNDRV_PCM_STREAM_LAST + 1],
};

unsafe extern "C" fn rz_ssi_probe(pdev: *mut platform_device) -> c_int {
    unsafe {
        let mut component_driver: *const snd_soc_component_driver;
        let np = (*(*pdev).dev).of_node;
        let dev = (*pdev).dev;
        let ssi: *mut rz_ssi_priv;
        let mut audio_clk: *mut clk;
        let mut res: *mut resource = core::ptr::null_mut();
        let mut ret: c_int;

        ssi = devm_kzalloc(dev, core::mem::size_of::<rz_ssi_priv>(), GFP_KERNEL) as *mut rz_ssi_priv;
        if ssi.is_null() {
            return -ENOMEM;
        }

        (*ssi).dev = dev;
        (*ssi).base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
        if IS_ERR((*ssi).base) {
            return PTR_ERR((*ssi).base);
        }

        (*ssi).clk = devm_clk_get(dev, c"ssi".as_ptr());
        if IS_ERR((*ssi).clk as *mut c_void) {
            return PTR_ERR((*ssi).clk as *mut c_void);
        }

        (*ssi).sfr_clk = devm_clk_get(dev, c"ssi_sfr".as_ptr());
        if IS_ERR((*ssi).sfr_clk as *mut c_void) {
            return PTR_ERR((*ssi).sfr_clk as *mut c_void);
        }

        audio_clk = devm_clk_get(dev, c"audio_clk1".as_ptr());
        if IS_ERR(audio_clk as *mut c_void) {
            return dev_err_probe(dev, PTR_ERR(audio_clk as *mut c_void), c"no audio clk1".as_ptr());
        }

        (*ssi).audio_clk_1 = clk_get_rate(audio_clk);
        audio_clk = devm_clk_get(dev, c"audio_clk2".as_ptr());
        if IS_ERR(audio_clk as *mut c_void) {
            return dev_err_probe(dev, PTR_ERR(audio_clk as *mut c_void), c"no audio clk2".as_ptr());
        }

        (*ssi).audio_clk_2 = clk_get_rate(audio_clk);
        if !((*ssi).audio_clk_1 != 0 || (*ssi).audio_clk_2 != 0) {
            return dev_err_probe(dev, -EINVAL, c"no audio clk1 or audio clk2".as_ptr());
        }

        (*ssi).audio_mck = if (*ssi).audio_clk_1 != 0 { (*ssi).audio_clk_1 } else { (*ssi).audio_clk_2 };

        (*ssi).dma_dais[SNDRV_PCM_STREAM_PLAYBACK].addr = (*res).start as dma_addr_t + SSIFTDR as dma_addr_t;
        (*ssi).dma_dais[SNDRV_PCM_STREAM_CAPTURE].addr = (*res).start as dma_addr_t + SSIFRDR as dma_addr_t;

        if of_property_present(np, c"dma-names".as_ptr()) {
            let config: *mut snd_dmaengine_pcm_config;
            let mut flags: c_uint = 0;

            config = devm_kzalloc(dev, core::mem::size_of::<snd_dmaengine_pcm_config>(), GFP_KERNEL)
                as *mut snd_dmaengine_pcm_config;
            if config.is_null() {
                return -ENOMEM;
            }

            (*config).pcm_hardware = rz_ssi_dmaengine_pcm_conf.pcm_hardware;
            (*config).prealloc_buffer_size = rz_ssi_dmaengine_pcm_conf.prealloc_buffer_size;
            (*config).prepare_slave_config = rz_ssi_dmaengine_pcm_conf.prepare_slave_config;

            if of_property_match_string(np, c"dma-names".as_ptr(), c"rt".as_ptr()) == 0 {
                flags = SND_DMAENGINE_PCM_FLAG_HALF_DUPLEX;
                (*config).chan_names[SNDRV_PCM_STREAM_PLAYBACK] = c"rt".as_ptr();
            } else {
                (*config).chan_names[SNDRV_PCM_STREAM_PLAYBACK] = c"tx".as_ptr();
                (*config).chan_names[SNDRV_PCM_STREAM_CAPTURE] = c"rx".as_ptr();
            }
            ret = devm_snd_dmaengine_pcm_register(dev, config, flags);
        } else {
            ret = -ENODEV;
        }

        if ret == -EPROBE_DEFER {
            return ret;
        } else if ret != 0 {
            dev_warn(dev, c"DMA not available, using PIO\n".as_ptr());
            (*ssi).playback.transfer = Some(rz_ssi_pio_send);
            (*ssi).capture.transfer = Some(rz_ssi_pio_recv);
            component_driver = &rz_ssi_soc_component_pio;
        } else {
            dev_info(dev, c"DMA enabled\n".as_ptr());
            component_driver = &rz_ssi_soc_component_dma;
        }

        (*ssi).playback.priv_ = ssi;
        (*ssi).capture.priv_ = ssi;

        spin_lock_init(&mut (*ssi).lock);
        dev_set_drvdata(dev, ssi as *mut c_void);

        /* Error Interrupt */
        (*ssi).irq_int = platform_get_irq_byname(pdev, c"int_req".as_ptr());
        if (*ssi).irq_int < 0 {
            return (*ssi).irq_int;
        }

        ret = devm_request_irq(dev, (*ssi).irq_int, Some(rz_ssi_interrupt), 0, dev_name(dev), ssi as *mut c_void);
        if ret < 0 {
            return dev_err_probe(dev, ret, c"irq request error (int_req)\n".as_ptr());
        }

        if !rz_ssi_is_dma_enabled(ssi) {
            /* Tx and Rx interrupts (pio only) */
            (*ssi).irq_tx = platform_get_irq_byname(pdev, c"dma_tx".as_ptr());
            (*ssi).irq_rx = platform_get_irq_byname(pdev, c"dma_rx".as_ptr());
            if (*ssi).irq_tx == -ENXIO && (*ssi).irq_rx == -ENXIO {
                (*ssi).irq_rt = platform_get_irq_byname(pdev, c"dma_rt".as_ptr());
                if (*ssi).irq_rt < 0 {
                    return (*ssi).irq_rt;
                }

                ret = devm_request_irq(
                    dev,
                    (*ssi).irq_rt,
                    Some(rz_ssi_interrupt),
                    0,
                    dev_name(dev),
                    ssi as *mut c_void,
                );
                if ret < 0 {
                    return dev_err_probe(dev, ret, c"irq request error (dma_rt)\n".as_ptr());
                }
            } else {
                if (*ssi).irq_tx < 0 {
                    return (*ssi).irq_tx;
                }

                if (*ssi).irq_rx < 0 {
                    return (*ssi).irq_rx;
                }

                ret = devm_request_irq(
                    dev,
                    (*ssi).irq_tx,
                    Some(rz_ssi_interrupt),
                    0,
                    dev_name(dev),
                    ssi as *mut c_void,
                );
                if ret < 0 {
                    return dev_err_probe(dev, ret, c"irq request error (dma_tx)\n".as_ptr());
                }

                ret = devm_request_irq(
                    dev,
                    (*ssi).irq_rx,
                    Some(rz_ssi_interrupt),
                    0,
                    dev_name(dev),
                    ssi as *mut c_void,
                );
                if ret < 0 {
                    return dev_err_probe(dev, ret, c"irq request error (dma_rx)\n".as_ptr());
                }
            }
        }

        (*ssi).rstc = devm_reset_control_get_exclusive(dev, core::ptr::null());
        if IS_ERR((*ssi).rstc as *mut c_void) {
            return dev_err_probe(dev, PTR_ERR((*ssi).rstc as *mut c_void), c"Failed to get reset\n".as_ptr());
        }

        /* Default 0 for power saving. Can be overridden via sysfs. */
        pm_runtime_set_autosuspend_delay(dev, 0);
        pm_runtime_use_autosuspend(dev);
        ret = devm_pm_runtime_enable(dev);
        if ret < 0 {
            return dev_err_probe(dev, ret, c"Failed to enable runtime PM!\n".as_ptr());
        }

        devm_snd_soc_register_component(dev, component_driver, rz_ssi_soc_dai.as_mut_ptr(), rz_ssi_soc_dai.len() as c_int)
    }
}

static rz_ssi_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"renesas,rz-ssi".as_ptr(),
    },
    of_device_id {
        /* Sentinel */
        compatible: core::ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, rz_ssi_of_match); */

unsafe extern "C" fn rz_ssi_runtime_suspend(dev: *mut device) -> c_int {
    unsafe {
        let ssi = dev_get_drvdata(dev) as *mut rz_ssi_priv;

        reset_control_assert((*ssi).rstc)
    }
}

unsafe extern "C" fn rz_ssi_runtime_resume(dev: *mut device) -> c_int {
    unsafe {
        let ssi = dev_get_drvdata(dev) as *mut rz_ssi_priv;

        reset_control_deassert((*ssi).rstc)
    }
}

static rz_ssi_pm_ops: dev_pm_ops = dev_pm_ops {
    /* RUNTIME_PM_OPS(rz_ssi_runtime_suspend, rz_ssi_runtime_resume, NULL) */
    runtime_suspend: Some(rz_ssi_runtime_suspend),
    runtime_resume: Some(rz_ssi_runtime_resume),
    /* NOIRQ_SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume) */
    suspend_noirq: Some(pm_runtime_force_suspend),
    resume_noirq: Some(pm_runtime_force_resume),
};

static mut rz_ssi_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"rz-ssi-pcm-audio".as_ptr(),
        of_match_table: rz_ssi_of_match.as_ptr(),
        pm: &rz_ssi_pm_ops,
    },
    probe: Some(rz_ssi_probe),
};

/* module_platform_driver(rz_ssi_driver); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_DESCRIPTION("Renesas RZ/G2L ASoC Serial Sound Interface Driver"); */
/* MODULE_AUTHOR("Biju Das <biju.das.jz@bp.renesas.com>"); */

#[repr(C)]
struct clk {
    _private: [u8; 0],
}
#[repr(C)]
struct reset_control {
    _private: [u8; 0],
}
#[repr(C)]
struct dma_chan {
    _private: [u8; 0],
}
#[repr(C)]
struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_soc_dai {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_soc_component {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_pcm {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_card {
    dev: *mut device,
}
#[repr(C)]
struct snd_soc_card {
    snd_card: *mut snd_card,
}
#[repr(C)]
struct snd_soc_pcm_runtime {
    pcm: *mut snd_pcm,
    card: *mut snd_soc_card,
}
#[repr(C)]
struct resource {
    start: dma_addr_t,
}
#[repr(C)]
struct device {
    of_node: *mut device_node,
}
#[repr(C)]
struct platform_device {
    dev: *mut device,
}
#[repr(C)]
struct snd_pcm_substream {
    stream: c_int,
    runtime: *mut snd_pcm_runtime,
}
#[repr(C)]
struct snd_pcm_runtime {
    buffer_size: c_uint,
    period_size: c_uint,
    channels: c_uint,
    dma_area: *mut c_void,
}
#[repr(C)]
struct snd_interval {
    min: c_uint,
}
#[repr(C)]
struct snd_dmaengine_dai_dma_data {
    addr: dma_addr_t,
    addr_width: c_uint,
}
#[repr(C)]
struct snd_soc_dai_ops {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
}
#[repr(C)]
struct snd_pcm_hardware {
    info: u32,
    buffer_bytes_max: usize,
    period_bytes_min: usize,
    period_bytes_max: usize,
    channels_min: u32,
    channels_max: u32,
    periods_min: u32,
    periods_max: u32,
    fifo_size: u32,
}
#[repr(C)]
struct snd_soc_pcm_stream {
    rates: u32,
    formats: u64,
    channels_min: u32,
    channels_max: u32,
}
#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}
#[repr(C)]
struct snd_soc_component_driver {
    name: *const c_char,
    open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int>,
    legacy_dai_naming: c_int,
}
#[repr(C)]
struct snd_dmaengine_pcm_config {
    pcm_hardware: *const snd_pcm_hardware,
    prealloc_buffer_size: usize,
    prepare_slave_config: Option<unsafe extern "C" fn() -> c_int>,
    chan_names: [*const c_char; SNDRV_PCM_STREAM_LAST + 1],
}
#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}
#[repr(C)]
struct dev_pm_ops {
    runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    suspend_noirq: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    resume_noirq: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}
#[repr(C)]
struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
    pm: *const dev_pm_ops,
}
#[repr(C)]
struct platform_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

const SZ_32K: usize = 32 * 1024;
const SNDRV_PCM_STREAM_PLAYBACK: usize = 0;
const SNDRV_PCM_STREAM_CAPTURE: usize = 1;
const SNDRV_PCM_STREAM_LAST: usize = 1;
const SNDRV_PCM_RATE_8000_48000: u32 = 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 1;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 2;
const SNDRV_PCM_INFO_INTERLEAVED: u32 = 1 << 0;
const SNDRV_PCM_INFO_MMAP: u32 = 1 << 1;
const SNDRV_PCM_INFO_MMAP_VALID: u32 = 1 << 2;
const SNDRV_PCM_INFO_RESUME: u32 = 1 << 3;
const SNDRV_PCM_INFO_PAUSE: u32 = 1 << 4;
const SNDRV_PCM_HW_PARAM_SAMPLE_BITS: c_int = 0;
const SNDRV_PCM_HW_PARAM_PERIODS: c_int = 1;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 0;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 2;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 4;
const SNDRV_PCM_TRIGGER_STOP: c_int = 5;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: u32 = 0;
const SND_SOC_DAIFMT_BP_FP: u32 = 0;
const SND_SOC_DAIFMT_INV_MASK: u32 = 0;
const SND_SOC_DAIFMT_NB_NF: u32 = 0;
const SND_SOC_DAIFMT_NB_IF: u32 = 1;
const SND_SOC_DAIFMT_IB_NF: u32 = 2;
const SND_SOC_DAIFMT_IB_IF: u32 = 3;
const SND_SOC_DAIFMT_FORMAT_MASK: u32 = 0;
const SND_SOC_DAIFMT_I2S: u32 = 0;
const DMA_SLAVE_BUSWIDTH_2_BYTES: u32 = 2;
const DMA_SLAVE_BUSWIDTH_4_BYTES: u32 = 4;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SND_DMAENGINE_PCM_FLAG_HALF_DUPLEX: c_uint = 1;
const GFP_KERNEL: gfp_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const ENXIO: c_int = 6;
const EPROBE_DEFER: c_int = 517;

unsafe extern "C" {
    fn writel(data: u32, addr: *mut c_void);
    fn readl(addr: *mut c_void) -> u32;
    fn readl_poll_timeout_atomic(addr: *mut c_void, tmp: *mut u32, cond_mask: u32, delay_us: c_int, timeout_us: c_int) -> c_int;
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn_ratelimited(dev: *mut device, fmt: *const c_char, ...);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn dmaengine_pause(chan: *mut dma_chan) -> c_int;
    fn dmaengine_resume(chan: *mut dma_chan) -> c_int;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_dmaengine_pcm_get_chan(substream: *mut snd_pcm_substream) -> *mut dma_chan;
    fn snd_soc_dai_init_dma_data(dai: *mut snd_soc_dai, playback: *mut snd_dmaengine_dai_dma_data, capture: *mut snd_dmaengine_dai_dma_data);
    fn snd_soc_set_runtime_hwparams(substream: *mut snd_pcm_substream, hw: *const snd_pcm_hardware);
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_int) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, type_: c_int, dev: *mut device, size: usize, max: usize);
    fn snd_dmaengine_pcm_prepare_slave_config() -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: gfp_t) -> *mut c_void;
    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: c_uint, res: *mut *mut resource) -> *mut c_void;
    fn IS_ERR(ptr: *mut c_void) -> bool_;
    fn PTR_ERR(ptr: *mut c_void) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn of_property_present(np: *mut device_node, propname: *const c_char) -> bool_;
    fn of_property_match_string(np: *mut device_node, propname: *const c_char, string: *const c_char) -> c_int;
    fn devm_snd_dmaengine_pcm_register(dev: *mut device, config: *mut snd_dmaengine_pcm_config, flags: c_uint) -> c_int;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn platform_get_irq_byname(pdev: *mut platform_device, name: *const c_char) -> c_int;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        flags: c_ulong,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn devm_reset_control_get_exclusive(dev: *mut device, id: *const c_char) -> *mut reset_control;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn devm_pm_runtime_enable(dev: *mut device) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn reset_control_assert(rstc: *mut reset_control) -> c_int;
    fn reset_control_deassert(rstc: *mut reset_control) -> c_int;
    fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_force_resume(dev: *mut device) -> c_int;
    fn WARN_ON(condition: bool_) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
