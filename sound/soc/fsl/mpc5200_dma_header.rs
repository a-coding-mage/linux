/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Freescale MPC5200 Audio DMA driver
 */

// C header guard removed:
// __SOUND_SOC_FSL_MPC5200_DMA_H__

use core::ffi::{c_char, c_int, c_uint, c_ulong};

pub const PSC_STREAM_NAME_LEN: usize = 32;

/**
 * psc_ac97_stream - Data specific to a single stream (playback or capture)
 * @active:		flag indicating if the stream is active
 * @psc_dma:		pointer back to parent psc_dma data structure
 * @bcom_task:		bestcomm task structure
 * @irq:		irq number for bestcomm task
 * @period_end:		physical address of end of DMA region
 * @period_next_pt:	physical address of next DMA buffer to enqueue
 * @period_bytes:	size of DMA period in bytes
 * @ac97_slot_bits:	Enable bits for turning on the correct AC97 slot
 */
#[repr(C)]
pub struct psc_dma_stream {
    pub runtime: *mut snd_pcm_runtime,
    pub active: c_int,
    pub psc_dma: *mut psc_dma,
    pub bcom_task: *mut bcom_task,
    pub irq: c_int,
    pub stream: *mut snd_pcm_substream,
    pub period_next: c_int,
    pub period_current: c_int,
    pub period_bytes: c_int,
    pub period_count: c_int,

    /* AC97 state */
    pub ac97_slot_bits: u32,
}

#[repr(C)]
pub struct psc_dma_stats {
    pub overrun_count: c_ulong,
    pub underrun_count: c_ulong,
}

/**
 * psc_dma - Private driver data
 * @name: short name for this device ("PSC0", "PSC1", etc)
 * @psc_regs: pointer to the PSC's registers
 * @fifo_regs: pointer to the PSC's FIFO registers
 * @irq: IRQ of this PSC
 * @dev: struct device pointer
 * @dai: the CPU DAI for this device
 * @sicr: Base value used in serial interface control register; mode is ORed
 *        with this value.
 * @playback: Playback stream context data
 * @capture: Capture stream context data
 */
#[repr(C)]
pub struct psc_dma {
    pub name: [c_char; 32],
    /* __iomem */
    pub psc_regs: *mut mpc52xx_psc,
    /* __iomem */
    pub fifo_regs: *mut mpc52xx_psc_fifo,
    pub irq: c_uint,
    pub dev: *mut device,
    pub lock: spinlock_t,
    pub mutex: mutex,
    pub sicr: u32,
    pub sysclk: c_uint,
    pub imr: c_int,
    pub id: c_int,
    pub slots: c_uint,

    /* per-stream data */
    pub playback: psc_dma_stream,
    pub capture: psc_dma_stream,

    /* Statistics */
    pub stats: psc_dma_stats,
}

/* Utility for retrieving psc_dma_stream structure from a substream */
#[inline]
pub unsafe fn to_psc_dma_stream(
    substream: *mut snd_pcm_substream,
    psc_dma: *mut psc_dma,
) -> *mut psc_dma_stream {
    if (*(*substream).pstr).stream == SNDRV_PCM_STREAM_CAPTURE {
        return &mut (*psc_dma).capture;
    }
    &mut (*psc_dma).playback
}

unsafe extern "C" {
    pub fn mpc5200_audio_dma_create(op: *mut platform_device) -> c_int;
    pub fn mpc5200_audio_dma_destroy(op: *mut platform_device) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
