// SPDX-License-Identifier: GPL-2.0
//
// Freescale DMA ALSA SoC PCM driver
//
// Author: Timur Tabi <timur@freescale.com>
//
// Copyright 2007-2010 Freescale Semiconductor, Inc.
//
// This driver implements ASoC support for the Elo DMA controller, which is
// the DMA controller on Freescale 83xx, 85xx, and 86xx SOCs. In ALSA terms,
// the PCM driver is what handles the DMA buffer.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u32 = u32;
type u64 = u64;
type uint32_t = u32;
type size_t = usize;
type dma_addr_t = u64;
type snd_pcm_uframes_t = u64;
type irqreturn_t = c_uint;

const DRV_NAME: &[u8] = b"fsl_dma\0";

/*
 * The formats that the DMA controller supports, which is anything
 * that is 8, 16, or 32 bits.
 */
const FSLDMA_PCM_FORMATS: u64 = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_U8
    | SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S16_BE
    | SNDRV_PCM_FMTBIT_U16_LE
    | SNDRV_PCM_FMTBIT_U16_BE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S24_BE
    | SNDRV_PCM_FMTBIT_U24_LE
    | SNDRV_PCM_FMTBIT_U24_BE
    | SNDRV_PCM_FMTBIT_S32_LE
    | SNDRV_PCM_FMTBIT_S32_BE
    | SNDRV_PCM_FMTBIT_U32_LE
    | SNDRV_PCM_FMTBIT_U32_BE;

#[repr(C)]
struct dma_object {
    dai: snd_soc_component_driver,
    ssi_stx_phys: dma_addr_t,
    ssi_srx_phys: dma_addr_t,
    ssi_fifo_depth: c_uint,
    channel: *mut ccsr_dma_channel,
    irq: c_uint,
    assigned: bool_,
}

/*
 * The number of DMA links to use.  Two is the bare minimum, but if you
 * have really small links you might need more.
 */
const NUM_DMA_LINKS: usize = 2;

/** fsl_dma_private: p-substream DMA data
 *
 * Each substream has a 1-to-1 association with a DMA channel.
 *
 * The link[] array is first because it needs to be aligned on a 32-byte
 * boundary, so putting it first will ensure alignment without padding the
 * structure.
 *
 * @link[]: array of link descriptors
 * @dma_channel: pointer to the DMA channel's registers
 * @irq: IRQ for this DMA channel
 * @substream: pointer to the substream object, needed by the ISR
 * @ssi_sxx_phys: bus address of the STX or SRX register to use
 * @ld_buf_phys: physical address of the LD buffer
 * @current_link: index into link[] of the link currently being processed
 * @dma_buf_phys: physical address of the DMA buffer
 * @dma_buf_next: physical address of the next period to process
 * @dma_buf_end: physical address of the byte after the end of the DMA
 * @buffer period_size: the size of a single period
 * @num_periods: the number of periods in the DMA buffer
 */
#[repr(C)]
struct fsl_dma_private {
    link: [fsl_dma_link_descriptor; NUM_DMA_LINKS],
    dma_channel: *mut ccsr_dma_channel,
    irq: c_uint,
    substream: *mut snd_pcm_substream,
    ssi_sxx_phys: dma_addr_t,
    ssi_fifo_depth: c_uint,
    ld_buf_phys: dma_addr_t,
    current_link: c_uint,
    dma_buf_phys: dma_addr_t,
    dma_buf_next: dma_addr_t,
    dma_buf_end: dma_addr_t,
    period_size: size_t,
    num_periods: c_uint,
}

/**
 * fsl_dma_hardare: define characteristics of the PCM hardware.
 *
 * The PCM hardware is the Freescale DMA controller.  This structure defines
 * the capabilities of that hardware.
 *
 * Since the sampling rate and data format are not controlled by the DMA
 * controller, we specify no limits for those values.  The only exception is
 * period_bytes_min, which is set to a reasonably low value to prevent the
 * DMA controller from generating too many interrupts per second.
 *
 * Since each link descriptor has a 32-bit byte count field, we set
 * period_bytes_max to the largest 32-bit number.  We also have no maximum
 * number of periods.
 *
 * Note that we specify SNDRV_PCM_INFO_JOINT_DUPLEX here, but only because a
 * limitation in the SSI driver requires the sample rates for playback and
 * capture to be the same.
 */
static fsl_dma_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_JOINT_DUPLEX
        | SNDRV_PCM_INFO_PAUSE,
    formats: FSLDMA_PCM_FORMATS,
    period_bytes_min: 512,
    period_bytes_max: u32::MAX as size_t,
    periods_min: NUM_DMA_LINKS as c_uint,
    periods_max: c_uint::MAX,
    buffer_bytes_max: 128 * 1024,
};

/**
 * fsl_dma_abort_stream: tell ALSA that the DMA transfer has aborted
 *
 * This function should be called by the ISR whenever the DMA controller
 * halts data transfer.
 */
unsafe fn fsl_dma_abort_stream(substream: *mut snd_pcm_substream) {
    snd_pcm_stop_xrun(substream);
}

/**
 * fsl_dma_update_pointers - update LD pointers to point to the next period
 *
 * As each period is completed, this function changes the link
 * descriptor pointers for that period to point to the next period.
 */
unsafe fn fsl_dma_update_pointers(dma_private: *mut fsl_dma_private) {
    let link = &mut (*dma_private).link[(*dma_private).current_link as usize] as *mut fsl_dma_link_descriptor;

    /* Update our link descriptors to point to the next period. On a 36-bit
     * system, we also need to update the ESAD bits.  We also set (keep) the
     * snoop bits.  See the comments in fsl_dma_hw_params() about snooping.
     */
    if (*(*dma_private).substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*link).source_addr = cpu_to_be32((*dma_private).dma_buf_next as u32);
        // CONFIG_PHYS_64BIT: update source_attr ESAD bits when physical addresses are 64-bit.
        (*link).source_attr = cpu_to_be32(CCSR_DMA_ATR_SNOOP | upper_32_bits((*dma_private).dma_buf_next));
    } else {
        (*link).dest_addr = cpu_to_be32((*dma_private).dma_buf_next as u32);
        // CONFIG_PHYS_64BIT: update dest_attr ESAD bits when physical addresses are 64-bit.
        (*link).dest_attr = cpu_to_be32(CCSR_DMA_ATR_SNOOP | upper_32_bits((*dma_private).dma_buf_next));
    }

    /* Update our variables for next time */
    (*dma_private).dma_buf_next = (*dma_private).dma_buf_next.wrapping_add((*dma_private).period_size as dma_addr_t);

    if (*dma_private).dma_buf_next >= (*dma_private).dma_buf_end {
        (*dma_private).dma_buf_next = (*dma_private).dma_buf_phys;
    }

    (*dma_private).current_link = (*dma_private).current_link.wrapping_add(1);
    if (*dma_private).current_link >= NUM_DMA_LINKS as c_uint {
        (*dma_private).current_link = 0;
    }
}

/**
 * fsl_dma_isr: interrupt handler for the DMA controller
 *
 * @irq: IRQ of the DMA channel
 * @dev_id: pointer to the dma_private structure for this DMA channel
 */
unsafe extern "C" fn fsl_dma_isr(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let dma_private = dev_id as *mut fsl_dma_private;
    let substream = (*dma_private).substream;
    let rtd = snd_soc_substream_to_rtd(substream);
    let dev = (*rtd).dev;
    let dma_channel = (*dma_private).dma_channel;
    let mut ret = IRQ_NONE;
    let mut sr2: u32 = 0;

    /* We got an interrupt, so read the status register to see what we
       were interrupted for.
     */
    let sr = in_be32(&mut (*dma_channel).sr);

    if sr & CCSR_DMA_SR_TE != 0 {
        dev_err(dev, c"dma transmit error\n".as_ptr());
        fsl_dma_abort_stream(substream);
        sr2 |= CCSR_DMA_SR_TE;
        ret = IRQ_HANDLED;
    }

    if sr & CCSR_DMA_SR_CH != 0 {
        ret = IRQ_HANDLED;
    }

    if sr & CCSR_DMA_SR_PE != 0 {
        dev_err(dev, c"dma programming error\n".as_ptr());
        fsl_dma_abort_stream(substream);
        sr2 |= CCSR_DMA_SR_PE;
        ret = IRQ_HANDLED;
    }

    if sr & CCSR_DMA_SR_EOLNI != 0 {
        sr2 |= CCSR_DMA_SR_EOLNI;
        ret = IRQ_HANDLED;
    }

    if sr & CCSR_DMA_SR_CB != 0 {
        ret = IRQ_HANDLED;
    }

    if sr & CCSR_DMA_SR_EOSI != 0 {
        /* Tell ALSA we completed a period. */
        snd_pcm_period_elapsed(substream);

        /*
         * Update our link descriptors to point to the next period. We
         * only need to do this if the number of periods is not equal to
         * the number of links.
         */
        if (*dma_private).num_periods != NUM_DMA_LINKS as c_uint {
            fsl_dma_update_pointers(dma_private);
        }

        sr2 |= CCSR_DMA_SR_EOSI;
        ret = IRQ_HANDLED;
    }

    if sr & CCSR_DMA_SR_EOLSI != 0 {
        sr2 |= CCSR_DMA_SR_EOLSI;
        ret = IRQ_HANDLED;
    }

    /* Clear the bits that we set */
    if sr2 != 0 {
        out_be32(&mut (*dma_channel).sr, sr2);
    }

    ret
}

/**
 * fsl_dma_new: initialize this PCM driver.
 *
 * This function is called by soc_new_pcm(), once for each DAI link
 * in the machine driver's snd_soc_card structure.
 *
 * Regardless of where the memory is actually allocated, since the device can
 * technically DMA to any 36-bit address, we do need to set the DMA mask to 36.
 */
unsafe extern "C" fn fsl_dma_new(_component: *mut snd_soc_component, rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*(*rtd).card).snd_card;
    let pcm = (*rtd).pcm;
    let ret = dma_coerce_mask_and_coherent((*card).dev, DMA_BIT_MASK(36));
    if ret != 0 {
        return ret;
    }

    snd_pcm_set_fixed_buffer_all(
        pcm,
        SNDRV_DMA_TYPE_DEV,
        (*card).dev,
        fsl_dma_hardware.buffer_bytes_max,
    )
}

/**
 * fsl_dma_open: open a new substream.
 *
 * Each substream has its own DMA buffer.
 *
 * ALSA divides the DMA buffer into N periods.  We create NUM_DMA_LINKS link
 * descriptors that ping-pong from one period to the next.
 */
unsafe extern "C" fn fsl_dma_open(component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let dev = (*component).dev;
    let dma = container_of_dma_object((*component).driver);
    let mut ld_buf_phys: dma_addr_t = 0;
    let mut ret: c_int;

    /*
     * Reject any DMA buffer whose size is not a multiple of the period
     * size.  We need to make sure that the DMA buffer can be evenly divided
     * into periods.
     */
    ret = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        dev_err(dev, c"invalid buffer size\n".as_ptr());
        return ret;
    }

    if (*dma).assigned {
        dev_err(dev, c"dma channel already assigned\n".as_ptr());
        return -EBUSY;
    }

    let dma_private = dma_alloc_coherent(
        dev,
        size_of::<fsl_dma_private>(),
        &mut ld_buf_phys,
        GFP_KERNEL,
    ) as *mut fsl_dma_private;
    if dma_private.is_null() {
        dev_err(dev, c"can't allocate dma private data\n".as_ptr());
        return -ENOMEM;
    }
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*dma_private).ssi_sxx_phys = (*dma).ssi_stx_phys;
    } else {
        (*dma_private).ssi_sxx_phys = (*dma).ssi_srx_phys;
    }

    (*dma_private).ssi_fifo_depth = (*dma).ssi_fifo_depth;
    (*dma_private).dma_channel = (*dma).channel;
    (*dma_private).irq = (*dma).irq;
    (*dma_private).substream = substream;
    (*dma_private).ld_buf_phys = ld_buf_phys;
    (*dma_private).dma_buf_phys = (*substream).dma_buffer.addr;

    ret = request_irq((*dma_private).irq, Some(fsl_dma_isr), 0, c"fsldma-audio".as_ptr(), dma_private as *mut c_void);
    if ret != 0 {
        dev_err(dev, c"can't register ISR for IRQ %u (ret=%i)\n".as_ptr(), (*dma_private).irq, ret);
        dma_free_coherent(dev, size_of::<fsl_dma_private>(), dma_private as *mut c_void, (*dma_private).ld_buf_phys);
        return ret;
    }

    (*dma).assigned = true;

    snd_soc_set_runtime_hwparams(substream, &fsl_dma_hardware);
    (*runtime).private_data = dma_private as *mut c_void;

    /* Program the fixed DMA controller parameters */

    let dma_channel = (*dma_private).dma_channel;

    let mut temp_link = (*dma_private).ld_buf_phys.wrapping_add(size_of::<fsl_dma_link_descriptor>() as dma_addr_t);

    let mut i: c_uint = 0;
    while i < NUM_DMA_LINKS as c_uint {
        (*dma_private).link[i as usize].next = cpu_to_be64(temp_link);
        temp_link = temp_link.wrapping_add(size_of::<fsl_dma_link_descriptor>() as dma_addr_t);
        i += 1;
    }
    /* The last link descriptor points to the first */
    (*dma_private).link[(i - 1) as usize].next = cpu_to_be64((*dma_private).ld_buf_phys);

    /* Tell the DMA controller where the first link descriptor is */
    out_be32(&mut (*dma_channel).clndar, CCSR_DMA_CLNDAR_ADDR((*dma_private).ld_buf_phys));
    out_be32(&mut (*dma_channel).eclndar, CCSR_DMA_ECLNDAR_ADDR((*dma_private).ld_buf_phys));

    /* The manual says the BCR must be clear before enabling EMP */
    out_be32(&mut (*dma_channel).bcr, 0);

    /*
     * Program the mode register for interrupts, external master control,
     * and source/destination hold.  Also clear the Channel Abort bit.
     */
    let mut mr = in_be32(&mut (*dma_channel).mr) & !(CCSR_DMA_MR_CA | CCSR_DMA_MR_DAHE | CCSR_DMA_MR_SAHE);

    /*
     * We want External Master Start and External Master Pause enabled,
     * because the SSI is controlling the DMA controller.
     */
    mr |= CCSR_DMA_MR_EOSIE | CCSR_DMA_MR_EIE | CCSR_DMA_MR_EMP_EN | CCSR_DMA_MR_EMS_EN;

    /* For playback, we want the destination address to be held.  For
       capture, set the source address to be held. */
    mr |= if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        CCSR_DMA_MR_DAHE
    } else {
        CCSR_DMA_MR_SAHE
    };

    out_be32(&mut (*dma_channel).mr, mr);

    0
}

/**
 * fsl_dma_hw_params: continue initializing the DMA links
 */
unsafe extern "C" fn fsl_dma_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let runtime = (*substream).runtime;
    let dma_private = (*runtime).private_data as *mut fsl_dma_private;
    let dev = (*component).dev;
    let sample_bits = snd_pcm_format_physical_width(params_format(hw_params));
    let sample_bytes = sample_bits / 8;
    let mut ssi_sxx_phys = (*dma_private).ssi_sxx_phys;
    let buffer_size = params_buffer_bytes(hw_params);
    let period_size = params_period_bytes(hw_params);
    let mut temp_addr = (*substream).dma_buffer.addr;
    let dma_channel = (*dma_private).dma_channel;

    /* Initialize our DMA tracking variables */
    (*dma_private).period_size = period_size;
    (*dma_private).num_periods = params_periods(hw_params);
    (*dma_private).dma_buf_end = (*dma_private).dma_buf_phys.wrapping_add(buffer_size as dma_addr_t);
    (*dma_private).dma_buf_next = (*dma_private)
        .dma_buf_phys
        .wrapping_add((NUM_DMA_LINKS * period_size) as dma_addr_t);

    if (*dma_private).dma_buf_next >= (*dma_private).dma_buf_end {
        /* This happens if the number of periods == NUM_DMA_LINKS */
        (*dma_private).dma_buf_next = (*dma_private).dma_buf_phys;
    }

    let mut mr = in_be32(&mut (*dma_channel).mr)
        & !(CCSR_DMA_MR_BWC_MASK | CCSR_DMA_MR_SAHTS_MASK | CCSR_DMA_MR_DAHTS_MASK);

    /* Due to a quirk of the SSI's STX register, the target address
     * for the DMA operations depends on the sample size.
     */
    match sample_bits {
        8 => {
            mr |= CCSR_DMA_MR_DAHTS_1 | CCSR_DMA_MR_SAHTS_1;
            ssi_sxx_phys = ssi_sxx_phys.wrapping_add(3);
        }
        16 => {
            mr |= CCSR_DMA_MR_DAHTS_2 | CCSR_DMA_MR_SAHTS_2;
            ssi_sxx_phys = ssi_sxx_phys.wrapping_add(2);
        }
        32 => {
            mr |= CCSR_DMA_MR_DAHTS_4 | CCSR_DMA_MR_SAHTS_4;
        }
        _ => {
            /* We should never get here */
            dev_err(dev, c"unsupported sample size %u\n".as_ptr(), sample_bits);
            return -EINVAL;
        }
    }

    /*
     * BWC determines how many bytes are sent/received before the DMA
     * controller checks the SSI to see if it needs to stop.
     */
    mr |= CCSR_DMA_MR_BWC(((*dma_private).ssi_fifo_depth - 2) * sample_bytes);

    out_be32(&mut (*dma_channel).mr, mr);

    let mut i: c_uint = 0;
    while i < NUM_DMA_LINKS as c_uint {
        let link = &mut (*dma_private).link[i as usize] as *mut fsl_dma_link_descriptor;

        (*link).count = cpu_to_be32(period_size as u32);

        /* The snoop bit tells the DMA controller whether it should tell
         * the ECM to snoop during a read or write to an address.
         */
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            (*link).source_addr = cpu_to_be32(temp_addr as u32);
            (*link).source_attr = cpu_to_be32(CCSR_DMA_ATR_SNOOP | upper_32_bits(temp_addr));

            (*link).dest_addr = cpu_to_be32(ssi_sxx_phys as u32);
            (*link).dest_attr = cpu_to_be32(CCSR_DMA_ATR_NOSNOOP | upper_32_bits(ssi_sxx_phys));
        } else {
            (*link).source_addr = cpu_to_be32(ssi_sxx_phys as u32);
            (*link).source_attr = cpu_to_be32(CCSR_DMA_ATR_NOSNOOP | upper_32_bits(ssi_sxx_phys));

            (*link).dest_addr = cpu_to_be32(temp_addr as u32);
            (*link).dest_attr = cpu_to_be32(CCSR_DMA_ATR_SNOOP | upper_32_bits(temp_addr));
        }

        temp_addr = temp_addr.wrapping_add(period_size as dma_addr_t);
        i += 1;
    }

    0
}

/**
 * fsl_dma_pointer: determine the current position of the DMA transfer
 */
unsafe extern "C" fn fsl_dma_pointer(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let runtime = (*substream).runtime;
    let dma_private = (*runtime).private_data as *mut fsl_dma_private;
    let dev = (*component).dev;
    let dma_channel = (*dma_private).dma_channel;
    let mut position: dma_addr_t;

    /* Obtain the current DMA pointer, but don't read the ESAD bits if we
     * only have 32-bit DMA addresses.
     */
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        position = in_be32(&mut (*dma_channel).sar) as dma_addr_t;
        // CONFIG_PHYS_64BIT: include source ESAD bits in the DMA address.
        position |= (((in_be32(&mut (*dma_channel).satr) & CCSR_DMA_ATR_ESAD_MASK) as u64) << 32) as dma_addr_t;
    } else {
        position = in_be32(&mut (*dma_channel).dar) as dma_addr_t;
        // CONFIG_PHYS_64BIT: include destination ESAD bits in the DMA address.
        position |= (((in_be32(&mut (*dma_channel).datr) & CCSR_DMA_ATR_ESAD_MASK) as u64) << 32) as dma_addr_t;
    }

    /*
     * When capture is started, the SSI immediately starts to fill its FIFO.
     */
    if position == 0 {
        return 0;
    }

    if position < (*dma_private).dma_buf_phys || position > (*dma_private).dma_buf_end {
        dev_err(dev, c"dma pointer is out of range, halting stream\n".as_ptr());
        return SNDRV_PCM_POS_XRUN;
    }

    let mut frames = bytes_to_frames(runtime, position.wrapping_sub((*dma_private).dma_buf_phys));

    /*
     * If the current address is just past the end of the buffer, wrap it
     * around.
     */
    if frames == (*runtime).buffer_size {
        frames = 0;
    }

    frames
}

/**
 * fsl_dma_hw_free: release resources allocated in fsl_dma_hw_params()
 */
unsafe extern "C" fn fsl_dma_hw_free(_component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let dma_private = (*runtime).private_data as *mut fsl_dma_private;

    if !dma_private.is_null() {
        let dma_channel = (*dma_private).dma_channel;

        /* Stop the DMA */
        out_be32(&mut (*dma_channel).mr, CCSR_DMA_MR_CA);
        out_be32(&mut (*dma_channel).mr, 0);

        /* Reset all the other registers */
        out_be32(&mut (*dma_channel).sr, u32::MAX);
        out_be32(&mut (*dma_channel).clndar, 0);
        out_be32(&mut (*dma_channel).eclndar, 0);
        out_be32(&mut (*dma_channel).satr, 0);
        out_be32(&mut (*dma_channel).sar, 0);
        out_be32(&mut (*dma_channel).datr, 0);
        out_be32(&mut (*dma_channel).dar, 0);
        out_be32(&mut (*dma_channel).bcr, 0);
        out_be32(&mut (*dma_channel).nlndar, 0);
        out_be32(&mut (*dma_channel).enlndar, 0);
    }

    0
}

/**
 * fsl_dma_close: close the stream.
 */
unsafe extern "C" fn fsl_dma_close(component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let dma_private = (*runtime).private_data as *mut fsl_dma_private;
    let dev = (*component).dev;
    let dma = container_of_dma_object((*component).driver);

    if !dma_private.is_null() {
        if (*dma_private).irq != 0 {
            free_irq((*dma_private).irq, dma_private as *mut c_void);
        }

        /* Deallocate the fsl_dma_private structure */
        dma_free_coherent(
            dev,
            size_of::<fsl_dma_private>(),
            dma_private as *mut c_void,
            (*dma_private).ld_buf_phys,
        );
        (*(*substream).runtime).private_data = ptr::null_mut();
    }

    (*dma).assigned = false;

    0
}

/**
 * find_ssi_node -- returns the SSI node that points to its DMA channel node
 */
unsafe fn find_ssi_node(dma_channel_np: *mut device_node) -> *mut device_node {
    let mut ssi_np: *mut device_node = ptr::null_mut();

    // for_each_compatible_node(ssi_np, NULL, "fsl,mpc8610-ssi")
    while {
        ssi_np = of_find_compatible_node(ssi_np, ptr::null_mut(), c"fsl,mpc8610-ssi".as_ptr());
        !ssi_np.is_null()
    } {
        /* Check each DMA phandle to see if it points to us.  We
         * assume that device_node pointers are a valid comparison.
         */
        let mut np = of_parse_phandle(ssi_np, c"fsl,playback-dma".as_ptr(), 0);
        of_node_put(np);
        if np == dma_channel_np {
            return ssi_np;
        }

        np = of_parse_phandle(ssi_np, c"fsl,capture-dma".as_ptr(), 0);
        of_node_put(np);
        if np == dma_channel_np {
            return ssi_np;
        }
    }

    ptr::null_mut()
}

unsafe extern "C" fn fsl_soc_dma_probe(pdev: *mut platform_device) -> c_int {
    let np = (*pdev).dev.of_node;
    let mut res: resource = core::mem::zeroed();
    let mut iprop: *const uint32_t;

    let channel = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(channel) {
        return PTR_ERR(channel);
    }

    let irq = platform_get_irq(pdev, 0);
    if irq < 0 {
        return irq;
    }

    let dma = devm_kzalloc(&mut (*pdev).dev, size_of::<dma_object>(), GFP_KERNEL) as *mut dma_object;
    if dma.is_null() {
        return -ENOMEM;
    }

    (*dma).dai.name = DRV_NAME.as_ptr() as *const c_char;
    (*dma).dai.open = Some(fsl_dma_open);
    (*dma).dai.close = Some(fsl_dma_close);
    (*dma).dai.hw_params = Some(fsl_dma_hw_params);
    (*dma).dai.hw_free = Some(fsl_dma_hw_free);
    (*dma).dai.pointer = Some(fsl_dma_pointer);
    (*dma).dai.pcm_new = Some(fsl_dma_new);

    (*dma).channel = channel as *mut ccsr_dma_channel;
    (*dma).irq = irq as c_uint;

    /* Find the SSI node that points to us. */
    let ssi_np = find_ssi_node(np);
    if ssi_np.is_null() {
        dev_err(&mut (*pdev).dev, c"cannot find parent SSI node\n".as_ptr());
        return -ENODEV;
    }

    let ret = of_address_to_resource(ssi_np, 0, &mut res);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, c"could not determine resources for %pOF\n".as_ptr(), ssi_np);
        of_node_put(ssi_np);
        return ret;
    }

    /* Store the SSI-specific information that we need */
    (*dma).ssi_stx_phys = res.start.wrapping_add(REG_SSI_STX0 as dma_addr_t);
    (*dma).ssi_srx_phys = res.start.wrapping_add(REG_SSI_SRX0 as dma_addr_t);

    iprop = of_get_property(ssi_np, c"fsl,fifo-depth".as_ptr(), ptr::null_mut()) as *const uint32_t;
    of_node_put(ssi_np);
    if !iprop.is_null() {
        (*dma).ssi_fifo_depth = be32_to_cpup(iprop);
    } else {
        /* Older 8610 DTs didn't have the fifo-depth property */
        (*dma).ssi_fifo_depth = 8;
    }

    devm_snd_soc_register_component(&mut (*pdev).dev, &mut (*dma).dai, ptr::null_mut(), 0)
}

static fsl_soc_dma_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: c"fsl,ssi-dma-channel".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, fsl_soc_dma_ids);

static mut fsl_soc_dma_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"fsl-pcm-audio".as_ptr(),
        of_match_table: fsl_soc_dma_ids.as_ptr(),
    },
    probe: Some(fsl_soc_dma_probe),
};

// module_platform_driver(fsl_soc_dma_driver);
// MODULE_AUTHOR("Timur Tabi <timur@freescale.com>");
// MODULE_DESCRIPTION("Freescale Elo DMA ASoC PCM Driver");
// MODULE_LICENSE("GPL v2");

#[repr(C)]
struct snd_soc_component_driver {
    name: *const c_char,
    open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    hw_free: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int>,
}

#[repr(C)]
struct fsl_dma_link_descriptor {
    source_attr: u32,
    source_addr: u32,
    dest_attr: u32,
    dest_addr: u32,
    next: u64,
    count: u32,
}

#[repr(C)]
struct ccsr_dma_channel {
    mr: u32,
    sr: u32,
    eclndar: u32,
    clndar: u32,
    satr: u32,
    sar: u32,
    datr: u32,
    dar: u32,
    bcr: u32,
    enlndar: u32,
    nlndar: u32,
}

#[repr(C)]
struct snd_pcm_hardware {
    info: u64,
    formats: u64,
    period_bytes_min: size_t,
    period_bytes_max: size_t,
    periods_min: c_uint,
    periods_max: c_uint,
    buffer_bytes_max: size_t,
}

#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
    driver: *mut snd_soc_component_driver,
}

#[repr(C)]
struct snd_soc_pcm_runtime {
    card: *mut snd_soc_card,
    pcm: *mut snd_pcm,
    dev: *mut device,
}

#[repr(C)]
struct snd_soc_card {
    snd_card: *mut snd_card,
}

#[repr(C)]
struct snd_card {
    dev: *mut device,
}

#[repr(C)]
struct snd_pcm {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
    stream: c_int,
    dma_buffer: snd_dma_buffer,
}

#[repr(C)]
struct snd_dma_buffer {
    addr: dma_addr_t,
}

#[repr(C)]
struct snd_pcm_runtime {
    private_data: *mut c_void,
    buffer_size: snd_pcm_uframes_t,
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct device {
    of_node: *mut device_node,
}

#[repr(C)]
struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
struct resource {
    start: dma_addr_t,
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct platform_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

const SNDRV_PCM_FMTBIT_S8: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_U8: u64 = 1 << 1;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_S16_BE: u64 = 1 << 3;
const SNDRV_PCM_FMTBIT_U16_LE: u64 = 1 << 4;
const SNDRV_PCM_FMTBIT_U16_BE: u64 = 1 << 5;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 6;
const SNDRV_PCM_FMTBIT_S24_BE: u64 = 1 << 7;
const SNDRV_PCM_FMTBIT_U24_LE: u64 = 1 << 8;
const SNDRV_PCM_FMTBIT_U24_BE: u64 = 1 << 9;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 10;
const SNDRV_PCM_FMTBIT_S32_BE: u64 = 1 << 11;
const SNDRV_PCM_FMTBIT_U32_LE: u64 = 1 << 12;
const SNDRV_PCM_FMTBIT_U32_BE: u64 = 1 << 13;

const SNDRV_PCM_INFO_INTERLEAVED: u64 = 1 << 0;
const SNDRV_PCM_INFO_MMAP: u64 = 1 << 1;
const SNDRV_PCM_INFO_MMAP_VALID: u64 = 1 << 2;
const SNDRV_PCM_INFO_JOINT_DUPLEX: u64 = 1 << 3;
const SNDRV_PCM_INFO_PAUSE: u64 = 1 << 4;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_HW_PARAM_PERIODS: c_int = 0;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SNDRV_PCM_POS_XRUN: snd_pcm_uframes_t = !0;

const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const GFP_KERNEL: c_uint = 0;
const EBUSY: c_int = 16;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;

const CCSR_DMA_SR_TE: u32 = 1 << 0;
const CCSR_DMA_SR_CH: u32 = 1 << 1;
const CCSR_DMA_SR_PE: u32 = 1 << 2;
const CCSR_DMA_SR_EOLNI: u32 = 1 << 3;
const CCSR_DMA_SR_CB: u32 = 1 << 4;
const CCSR_DMA_SR_EOSI: u32 = 1 << 5;
const CCSR_DMA_SR_EOLSI: u32 = 1 << 6;
const CCSR_DMA_ATR_SNOOP: u32 = 1 << 0;
const CCSR_DMA_ATR_NOSNOOP: u32 = 0;
const CCSR_DMA_ATR_ESAD_MASK: u32 = 0xF;
const CCSR_DMA_MR_CA: u32 = 1 << 0;
const CCSR_DMA_MR_DAHE: u32 = 1 << 1;
const CCSR_DMA_MR_SAHE: u32 = 1 << 2;
const CCSR_DMA_MR_EOSIE: u32 = 1 << 3;
const CCSR_DMA_MR_EIE: u32 = 1 << 4;
const CCSR_DMA_MR_EMP_EN: u32 = 1 << 5;
const CCSR_DMA_MR_EMS_EN: u32 = 1 << 6;
const CCSR_DMA_MR_BWC_MASK: u32 = 0x0000_ff00;
const CCSR_DMA_MR_SAHTS_MASK: u32 = 0x000f_0000;
const CCSR_DMA_MR_DAHTS_MASK: u32 = 0x00f0_0000;
const CCSR_DMA_MR_DAHTS_1: u32 = 1 << 20;
const CCSR_DMA_MR_SAHTS_1: u32 = 1 << 16;
const CCSR_DMA_MR_DAHTS_2: u32 = 2 << 20;
const CCSR_DMA_MR_SAHTS_2: u32 = 2 << 16;
const CCSR_DMA_MR_DAHTS_4: u32 = 4 << 20;
const CCSR_DMA_MR_SAHTS_4: u32 = 4 << 16;
const REG_SSI_STX0: u32 = 0;
const REG_SSI_SRX0: u32 = 0;

const fn DMA_BIT_MASK(n: u32) -> u64 {
    if n == 64 { !0 } else { (1u64 << n) - 1 }
}

fn upper_32_bits(n: dma_addr_t) -> u32 {
    (n >> 32) as u32
}

fn CCSR_DMA_CLNDAR_ADDR(n: dma_addr_t) -> u32 {
    n as u32
}

fn CCSR_DMA_ECLNDAR_ADDR(n: dma_addr_t) -> u32 {
    upper_32_bits(n)
}

fn CCSR_DMA_MR_BWC(n: c_uint) -> u32 {
    n << 8
}

fn cpu_to_be32(n: u32) -> u32 {
    n.to_be()
}

fn cpu_to_be64(n: u64) -> u64 {
    n.to_be()
}

unsafe fn container_of_dma_object(dai: *mut snd_soc_component_driver) -> *mut dma_object {
    dai as *mut dma_object
}

unsafe fn IS_ERR(ptr: *mut c_void) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn PTR_ERR(ptr: *mut c_void) -> c_int {
    ptr as isize as c_int
}

extern "C" {
    fn snd_pcm_stop_xrun(substream: *mut snd_pcm_substream);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn in_be32(addr: *mut u32) -> u32;
    fn out_be32(addr: *mut u32, val: u32);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn dma_coerce_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn snd_pcm_set_fixed_buffer_all(pcm: *mut snd_pcm, ty: c_int, dev: *mut device, size: size_t) -> c_int;
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_int) -> c_int;
    fn dma_alloc_coherent(dev: *mut device, size: size_t, dma_handle: *mut dma_addr_t, flag: c_uint) -> *mut c_void;
    fn dma_free_coherent(dev: *mut device, size: size_t, cpu_addr: *mut c_void, dma_handle: dma_addr_t);
    fn request_irq(
        irq: c_uint,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        flags: c_uint,
        name: *const c_char,
        dev: *mut c_void,
    ) -> c_int;
    fn free_irq(irq: c_uint, dev_id: *mut c_void);
    fn snd_soc_set_runtime_hwparams(substream: *mut snd_pcm_substream, hw: *const snd_pcm_hardware);
    fn snd_pcm_format_physical_width(format: c_int) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_buffer_bytes(params: *mut snd_pcm_hw_params) -> size_t;
    fn params_period_bytes(params: *mut snd_pcm_hw_params) -> size_t;
    fn params_periods(params: *mut snd_pcm_hw_params) -> c_uint;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: dma_addr_t) -> snd_pcm_uframes_t;
    fn of_find_compatible_node(from: *mut device_node, ty: *mut c_void, compatible: *const c_char) -> *mut device_node;
    fn of_parse_phandle(np: *mut device_node, phandle_name: *const c_char, index: c_int) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn of_address_to_resource(dev: *mut device_node, index: c_uint, res: *mut resource) -> c_int;
    fn of_get_property(np: *mut device_node, name: *const c_char, lenp: *mut c_void) -> *const c_void;
    fn be32_to_cpup(p: *const uint32_t) -> c_uint;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *mut snd_soc_component_driver,
        dai_drv: *mut c_void,
        num_dai: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
