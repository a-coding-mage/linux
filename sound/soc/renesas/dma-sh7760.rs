// SPDX-License-Identifier: GPL-2.0
//
// SH7760 ("camelot") DMABRG audio DMA unit support
//
// Copyright (C) 2007 Manuel Lauss <mano@roarinelk.homelinux.net>
//
// The SH7760 DMABRG provides 4 dma channels (2x rec, 2x play), which
// trigger an interrupt when one half of the programmed transfer size
// has been xmitted.
//
// FIXME: little-endian only for now

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr::{read_volatile, write_volatile};

/* registers and bits */
const BRGATXSAR: c_ulong = 0x00;
const BRGARXDAR: c_ulong = 0x04;
const BRGATXTCR: c_ulong = 0x08;
const BRGARXTCR: c_ulong = 0x0C;
const BRGACR: c_ulong = 0x10;
const BRGATXTCNT: c_ulong = 0x14;
const BRGARXTCNT: c_ulong = 0x18;

const ACR_RAR: c_ulong = 1 << 18;
const ACR_RDS: c_ulong = 1 << 17;
const ACR_RDE: c_ulong = 1 << 16;
const ACR_TAR: c_ulong = 1 << 2;
const ACR_TDS: c_ulong = 1 << 1;
const ACR_TDE: c_ulong = 1 << 0;

/* receiver/transmitter data alignment */
const ACR_RAM_NONE: c_ulong = 0 << 24;
const ACR_RAM_4BYTE: c_ulong = 1 << 24;
const ACR_RAM_2WORD: c_ulong = 2 << 24;
const ACR_TAM_NONE: c_ulong = 0 << 8;
const ACR_TAM_4BYTE: c_ulong = 1 << 8;
const ACR_TAM_2WORD: c_ulong = 2 << 8;

const DMABRGIRQ_A0TXF: c_int = 0;
const DMABRGIRQ_A1TXF: c_int = 0;

const SNDRV_PCM_INFO_MMAP: u32 = 0;
const SNDRV_PCM_INFO_INTERLEAVED: u32 = 0;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: u32 = 0;
const SNDRV_PCM_INFO_MMAP_VALID: u32 = 0;
const SNDRV_PCM_INFO_BATCH: u32 = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 0;
const SNDRV_DMA_TYPE_CONTINUOUS: c_int = 0;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;

type SndPcmUframesT = c_ulong;

#[repr(C)]
pub struct snd_pcm_substream {
    stream: c_int,
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    dma_addr: c_ulong,
    dma_area: *mut c_void,
    dma_bytes: usize,
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_soc_dai {
    id: usize,
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
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    dev: device,
}

#[repr(C)]
pub struct device_driver {
    name: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    info: u32,
    buffer_bytes_max: c_ulong,
    period_bytes_min: c_ulong,
    period_bytes_max: c_ulong,
    periods_min: u32,
    periods_max: u32,
    fifo_size: u32,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    open: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int,
    >,
    close: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int,
    >,
    hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
        ) -> c_int,
    >,
    prepare: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int,
    >,
    trigger: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int) -> c_int,
    >,
    pointer: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> SndPcmUframesT,
    >,
    pcm_new: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int,
    >,
}

#[repr(C)]
struct camelot_pcm {
    mmio: c_ulong, /* DMABRG audio channel control reg MMIO */
    txid: u32,    /* ID of first DMABRG IRQ for this unit */

    tx_ss: *mut snd_pcm_substream,
    tx_period_size: c_ulong,
    tx_period: u32,

    rx_ss: *mut snd_pcm_substream,
    rx_period_size: c_ulong,
    rx_period: u32,
}

static mut CAM_PCM_DATA: [camelot_pcm; 2] = [
    camelot_pcm {
        mmio: 0xFE3C0040,
        txid: DMABRGIRQ_A0TXF as u32,
        tx_ss: core::ptr::null_mut(),
        tx_period_size: 0,
        tx_period: 0,
        rx_ss: core::ptr::null_mut(),
        rx_period_size: 0,
        rx_period: 0,
    },
    camelot_pcm {
        mmio: 0xFE3C0060,
        txid: DMABRGIRQ_A1TXF as u32,
        tx_ss: core::ptr::null_mut(),
        tx_period_size: 0,
        tx_period: 0,
        rx_ss: core::ptr::null_mut(),
        rx_period_size: 0,
        rx_period: 0,
    },
];

unsafe fn brgreg_read(cam: *mut camelot_pcm, x: c_ulong) -> c_ulong {
    read_volatile(((*cam).mmio.wrapping_add(x)) as *const c_ulong)
}

unsafe fn brgreg_write(cam: *mut camelot_pcm, x: c_ulong, value: c_ulong) {
    write_volatile(((*cam).mmio.wrapping_add(x)) as *mut c_ulong, value);
}

/*
 * set a minimum of 16kb per period, to avoid interrupt-"storm" and
 * resulting skipping. In general, the bigger the minimum size, the
 * better for overall system performance. (The SH7760 is a puny CPU
 * with a slow SDRAM interface and poor internal bus bandwidth,
 * *especially* when the LCDC is active).  The minimum for the DMAC
 * is 8 bytes; 16kbytes are enough to get skip-free playback of a
 * 44kHz/16bit/stereo MP3 on a lightly loaded system, and maintain
 * reasonable responsiveness in MPlayer.
 */
const DMABRG_PERIOD_MIN: c_ulong = 16 * 1024;
const DMABRG_PERIOD_MAX: c_ulong = 0x03fffffc;
const DMABRG_PREALLOC_BUFFER: usize = 32 * 1024;
const DMABRG_PREALLOC_BUFFER_MAX: usize = 32 * 1024;

static CAMELOT_PCM_HARDWARE: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_BATCH,
    buffer_bytes_max: DMABRG_PERIOD_MAX,
    period_bytes_min: DMABRG_PERIOD_MIN,
    period_bytes_max: DMABRG_PERIOD_MAX / 2,
    periods_min: 2,
    periods_max: 2,
    fifo_size: 128,
};

unsafe extern "C" {
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_set_runtime_hwparams(
        substream: *mut snd_pcm_substream,
        hw: *const snd_pcm_hardware,
    ) -> c_int;
    fn dmabrg_request_irq(
        dmairq: c_int,
        handler: unsafe extern "C" fn(*mut c_void),
        data: *mut camelot_pcm,
    ) -> c_int;
    fn dmabrg_free_irq(dmairq: c_int);
    fn params_period_bytes(hw_params: *mut snd_pcm_hw_params) -> c_ulong;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, size: c_ulong) -> SndPcmUframesT;
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        type_: c_int,
        data: *mut c_void,
        size: usize,
        max: usize,
    ) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut c_void,
        num_dai: c_int,
    ) -> c_int;
    fn pr_debug(fmt: *const c_char, ...);
}

unsafe extern "C" fn camelot_txdma(data: *mut c_void) {
    let cam = data as *mut camelot_pcm;
    (*cam).tx_period ^= 1;
    snd_pcm_period_elapsed((*cam).tx_ss);
}

unsafe extern "C" fn camelot_rxdma(data: *mut c_void) {
    let cam = data as *mut camelot_pcm;
    (*cam).rx_period ^= 1;
    snd_pcm_period_elapsed((*cam).rx_ss);
}

unsafe extern "C" fn camelot_pcm_open(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cam = &mut CAM_PCM_DATA[(*snd_soc_rtd_to_cpu(rtd, 0)).id] as *mut camelot_pcm;
    let recv = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        0
    } else {
        1
    };
    let ret: c_int;
    let dmairq: c_int;

    snd_soc_set_runtime_hwparams(substream, &CAMELOT_PCM_HARDWARE);

    /* DMABRG buffer half/full events */
    dmairq = if recv != 0 {
        (*cam).txid as c_int + 2
    } else {
        (*cam).txid as c_int
    };
    if recv != 0 {
        (*cam).rx_ss = substream;
        ret = dmabrg_request_irq(dmairq, camelot_rxdma, cam);
        if ret != 0 {
            pr_debug(
                c"audio unit %d irqs already taken!\n".as_ptr(),
                (*snd_soc_rtd_to_cpu(rtd, 0)).id as c_int,
            );
            return -EBUSY;
        }
        dmabrg_request_irq(dmairq + 1, camelot_rxdma, cam);
    } else {
        (*cam).tx_ss = substream;
        ret = dmabrg_request_irq(dmairq, camelot_txdma, cam);
        if ret != 0 {
            pr_debug(
                c"audio unit %d irqs already taken!\n".as_ptr(),
                (*snd_soc_rtd_to_cpu(rtd, 0)).id as c_int,
            );
            return -EBUSY;
        }
        dmabrg_request_irq(dmairq + 1, camelot_txdma, cam);
    }
    0
}

unsafe extern "C" fn camelot_pcm_close(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cam = &mut CAM_PCM_DATA[(*snd_soc_rtd_to_cpu(rtd, 0)).id] as *mut camelot_pcm;
    let recv = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        0
    } else {
        1
    };
    let dmairq: c_int;

    dmairq = if recv != 0 {
        (*cam).txid as c_int + 2
    } else {
        (*cam).txid as c_int
    };

    if recv != 0 {
        (*cam).rx_ss = core::ptr::null_mut();
    } else {
        (*cam).tx_ss = core::ptr::null_mut();
    }

    dmabrg_free_irq(dmairq + 1);
    dmabrg_free_irq(dmairq);

    0
}

unsafe extern "C" fn camelot_hw_params(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cam = &mut CAM_PCM_DATA[(*snd_soc_rtd_to_cpu(rtd, 0)).id] as *mut camelot_pcm;
    let recv = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        0
    } else {
        1
    };

    if recv != 0 {
        (*cam).rx_period_size = params_period_bytes(hw_params);
        (*cam).rx_period = 0;
    } else {
        (*cam).tx_period_size = params_period_bytes(hw_params);
        (*cam).tx_period = 0;
    }
    0
}

unsafe extern "C" fn camelot_prepare(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime = (*substream).runtime;
    let rtd = snd_soc_substream_to_rtd(substream);
    let cam = &mut CAM_PCM_DATA[(*snd_soc_rtd_to_cpu(rtd, 0)).id] as *mut camelot_pcm;

    pr_debug(
        c"PCM data: addr %pad len %zu\n".as_ptr(),
        &mut (*runtime).dma_addr as *mut c_ulong,
        (*runtime).dma_bytes,
    );

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        brgreg_write(cam, BRGATXSAR, (*runtime).dma_area as c_ulong);
        brgreg_write(cam, BRGATXTCR, (*runtime).dma_bytes as c_ulong);
    } else {
        brgreg_write(cam, BRGARXDAR, (*runtime).dma_area as c_ulong);
        brgreg_write(cam, BRGARXTCR, (*runtime).dma_bytes as c_ulong);
    }

    0
}

unsafe fn dmabrg_play_dma_start(cam: *mut camelot_pcm) {
    let acr = brgreg_read(cam, BRGACR) & !(ACR_TDS | ACR_RDS);
    /* start DMABRG engine: XFER start, auto-addr-reload */
    brgreg_write(cam, BRGACR, acr | ACR_TDE | ACR_TAR | ACR_TAM_2WORD);
}

unsafe fn dmabrg_play_dma_stop(cam: *mut camelot_pcm) {
    let acr = brgreg_read(cam, BRGACR) & !(ACR_TDS | ACR_RDS);
    /* forcibly terminate data transmission */
    brgreg_write(cam, BRGACR, acr | ACR_TDS);
}

unsafe fn dmabrg_rec_dma_start(cam: *mut camelot_pcm) {
    let acr = brgreg_read(cam, BRGACR) & !(ACR_TDS | ACR_RDS);
    /* start DMABRG engine: recv start, auto-reload */
    brgreg_write(cam, BRGACR, acr | ACR_RDE | ACR_RAR | ACR_RAM_2WORD);
}

unsafe fn dmabrg_rec_dma_stop(cam: *mut camelot_pcm) {
    let acr = brgreg_read(cam, BRGACR) & !(ACR_TDS | ACR_RDS);
    /* forcibly terminate data receiver */
    brgreg_write(cam, BRGACR, acr | ACR_RDS);
}

unsafe extern "C" fn camelot_trigger(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cam = &mut CAM_PCM_DATA[(*snd_soc_rtd_to_cpu(rtd, 0)).id] as *mut camelot_pcm;
    let recv = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        0
    } else {
        1
    };

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            if recv != 0 {
                dmabrg_rec_dma_start(cam);
            } else {
                dmabrg_play_dma_start(cam);
            }
        }
        SNDRV_PCM_TRIGGER_STOP => {
            if recv != 0 {
                dmabrg_rec_dma_stop(cam);
            } else {
                dmabrg_play_dma_stop(cam);
            }
        }
        _ => {
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn camelot_pos(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> SndPcmUframesT {
    let runtime = (*substream).runtime;
    let rtd = snd_soc_substream_to_rtd(substream);
    let cam = &mut CAM_PCM_DATA[(*snd_soc_rtd_to_cpu(rtd, 0)).id] as *mut camelot_pcm;
    let recv = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        0
    } else {
        1
    };
    let pos: c_ulong;

    /* cannot use the DMABRG pointer register: under load, by the
     * time ALSA comes around to read the register, it is already
     * far ahead (or worse, already done with the fragment) of the
     * position at the time the IRQ was triggered, which results in
     * fast-playback sound in my test application (ScummVM)
     */
    if recv != 0 {
        pos = if (*cam).rx_period != 0 {
            (*cam).rx_period_size
        } else {
            0
        };
    } else {
        pos = if (*cam).tx_period != 0 {
            (*cam).tx_period_size
        } else {
            0
        };
    }

    bytes_to_frames(runtime, pos)
}

unsafe extern "C" fn camelot_pcm_new(
    _component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let pcm = (*rtd).pcm;

    /* dont use SNDRV_DMA_TYPE_DEV, since it will oops the SH kernel
     * in MMAP mode (i.e. aplay -M)
     */
    snd_pcm_set_managed_buffer_all(
        pcm,
        SNDRV_DMA_TYPE_CONTINUOUS,
        core::ptr::null_mut(),
        DMABRG_PREALLOC_BUFFER,
        DMABRG_PREALLOC_BUFFER_MAX,
    );

    0
}

static SH7760_SOC_COMPONENT: snd_soc_component_driver = snd_soc_component_driver {
    open: Some(camelot_pcm_open),
    close: Some(camelot_pcm_close),
    hw_params: Some(camelot_hw_params),
    prepare: Some(camelot_prepare),
    trigger: Some(camelot_trigger),
    pointer: Some(camelot_pos),
    pcm_new: Some(camelot_pcm_new),
};

unsafe extern "C" fn sh7760_soc_platform_probe(pdev: *mut platform_device) -> c_int {
    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &SH7760_SOC_COMPONENT,
        core::ptr::null_mut(),
        0,
    )
}

static mut SH7760_PCM_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: c"sh7760-pcm-audio".as_ptr(),
    },

    probe: Some(sh7760_soc_platform_probe),
};

/* module_platform_driver(sh7760_pcm_driver); */

/* MODULE_LICENSE("GPL v2"); */
/* MODULE_DESCRIPTION("SH7760 Audio DMA (DMABRG) driver"); */
/* MODULE_AUTHOR("Manuel Lauss <mano@roarinelk.homelinux.net>"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
