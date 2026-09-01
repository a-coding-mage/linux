// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for audio on multifunction CS5535 companion device
 * Copyright (C) Jaya Kumar
 *
 * Based on Jaroslav Kysela and Takashi Iwai's examples.
 * This work was sponsored by CIS(M) Sdn Bhd.
 *
 * todo: add be fmt support, spdif, pm
 */

// C dependencies:
// <linux/init.h>, <linux/pci.h>, <sound/core.h>, <sound/control.h>,
// <sound/initval.h>, <sound/asoundef.h>, <sound/pcm.h>,
// <sound/pcm_params.h>, <sound/ac97_codec.h>, "cs5535audio.h"

static snd_cs5535audio_playback: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_RESUME,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    rate_min: 4000,
    rate_max: 48000,
    channels_min: 2,
    channels_max: 2,
    buffer_bytes_max: 128 * 1024,
    period_bytes_min: 64,
    period_bytes_max: 64 * 1024 - 16,
    periods_min: 1,
    periods_max: CS5535AUDIO_MAX_DESCRIPTORS,
    fifo_size: 0,
};

static snd_cs5535audio_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    rate_min: 4000,
    rate_max: 48000,
    channels_min: 2,
    channels_max: 2,
    buffer_bytes_max: 128 * 1024,
    period_bytes_min: 64,
    period_bytes_max: 64 * 1024 - 16,
    periods_min: 1,
    periods_max: CS5535AUDIO_MAX_DESCRIPTORS,
    fifo_size: 0,
};

unsafe fn snd_cs5535audio_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let err: c_int;
    let cs5535au: *mut cs5535audio = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;

    (*runtime).hw = snd_cs5535audio_playback;
    (*runtime).hw.rates = (*(*cs5535au).ac97).rates[AC97_RATES_FRONT_DAC as usize];
    snd_pcm_limit_hw_rates(runtime);
    (*cs5535au).playback_substream = substream;
    (*runtime).private_data = &mut (*cs5535au).dmas[CS5535AUDIO_DMA_PLAYBACK as usize] as *mut _ as *mut c_void;
    err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if err < 0 {
        return err;
    }

    0
}

unsafe fn snd_cs5535audio_playback_close(_substream: *mut snd_pcm_substream) -> c_int {
    0
}

const CS5535AUDIO_DESC_LIST_SIZE: usize =
    PAGE_ALIGN(CS5535AUDIO_MAX_DESCRIPTORS as usize * core::mem::size_of::<cs5535audio_dma_desc>());

unsafe fn cs5535audio_build_dma_packets(
    cs5535au: *mut cs5535audio,
    dma: *mut cs5535audio_dma,
    substream: *mut snd_pcm_substream,
    periods: c_uint,
    period_bytes: c_uint,
) -> c_int {
    let mut i: c_uint;
    let mut addr: u32;
    let jmpprd_addr: u32;
    let lastdesc: *mut cs5535audio_dma_desc;

    if periods > CS5535AUDIO_MAX_DESCRIPTORS {
        return -ENOMEM;
    }

    if (*dma).desc_buf.area.is_null() {
        if snd_dma_alloc_pages(
            SNDRV_DMA_TYPE_DEV,
            &mut (*(*cs5535au).pci).dev,
            CS5535AUDIO_DESC_LIST_SIZE + 1,
            &mut (*dma).desc_buf,
        ) < 0
        {
            return -ENOMEM;
        }
        (*dma).period_bytes = 0;
        (*dma).periods = 0;
    }

    if (*dma).periods == periods && (*dma).period_bytes == period_bytes {
        return 0;
    }

    /* the u32 cast is okay because in snd*create we successfully told
       pci alloc that we're only 32 bit capable so the upper will be 0 */
    addr = (*(*substream).runtime).dma_addr as u32;
    i = 0;
    while i < periods {
        let desc: *mut cs5535audio_dma_desc =
            ((*dma).desc_buf.area as *mut cs5535audio_dma_desc).add(i as usize);
        (*desc).addr = cpu_to_le32(addr);
        (*desc).size = cpu_to_le16(period_bytes as u16);
        (*desc).ctlreserved = cpu_to_le16(PRD_EOP);
        addr = addr.wrapping_add(period_bytes);
        i += 1;
    }
    /* we reserved one dummy descriptor at the end to do the PRD jump */
    lastdesc = ((*dma).desc_buf.area as *mut cs5535audio_dma_desc).add(periods as usize);
    (*lastdesc).addr = cpu_to_le32((*dma).desc_buf.addr as u32);
    (*lastdesc).size = 0;
    (*lastdesc).ctlreserved = cpu_to_le16(PRD_JMP);
    jmpprd_addr = ((*dma).desc_buf.addr as u32).wrapping_add(
        (core::mem::size_of::<cs5535audio_dma_desc>() * periods as usize) as u32,
    );

    (*dma).substream = substream;
    (*dma).period_bytes = period_bytes;
    (*dma).periods = periods;
    guard_spinlock_irq(&mut (*cs5535au).reg_lock);
    ((*(*dma).ops).disable_dma)(cs5535au);
    ((*(*dma).ops).setup_prd)(cs5535au, jmpprd_addr);
    0
}

unsafe fn cs5535audio_playback_enable_dma(cs5535au: *mut cs5535audio) {
    cs_writeb(cs5535au, ACC_BM0_CMD, BM_CTL_EN);
}

unsafe fn cs5535audio_playback_disable_dma(cs5535au: *mut cs5535audio) {
    cs_writeb(cs5535au, ACC_BM0_CMD, 0);
}

unsafe fn cs5535audio_playback_pause_dma(cs5535au: *mut cs5535audio) {
    cs_writeb(cs5535au, ACC_BM0_CMD, BM_CTL_PAUSE);
}

unsafe fn cs5535audio_playback_setup_prd(cs5535au: *mut cs5535audio, prd_addr: u32) {
    cs_writel(cs5535au, ACC_BM0_PRD, prd_addr);
}

unsafe fn cs5535audio_playback_read_prd(cs5535au: *mut cs5535audio) -> u32 {
    cs_readl(cs5535au, ACC_BM0_PRD)
}

unsafe fn cs5535audio_playback_read_dma_pntr(cs5535au: *mut cs5535audio) -> u32 {
    cs_readl(cs5535au, ACC_BM0_PNTR)
}

unsafe fn cs5535audio_capture_enable_dma(cs5535au: *mut cs5535audio) {
    cs_writeb(cs5535au, ACC_BM1_CMD, BM_CTL_EN);
}

unsafe fn cs5535audio_capture_disable_dma(cs5535au: *mut cs5535audio) {
    cs_writeb(cs5535au, ACC_BM1_CMD, 0);
}

unsafe fn cs5535audio_capture_pause_dma(cs5535au: *mut cs5535audio) {
    cs_writeb(cs5535au, ACC_BM1_CMD, BM_CTL_PAUSE);
}

unsafe fn cs5535audio_capture_setup_prd(cs5535au: *mut cs5535audio, prd_addr: u32) {
    cs_writel(cs5535au, ACC_BM1_PRD, prd_addr);
}

unsafe fn cs5535audio_capture_read_prd(cs5535au: *mut cs5535audio) -> u32 {
    cs_readl(cs5535au, ACC_BM1_PRD)
}

unsafe fn cs5535audio_capture_read_dma_pntr(cs5535au: *mut cs5535audio) -> u32 {
    cs_readl(cs5535au, ACC_BM1_PNTR)
}

unsafe fn cs5535audio_clear_dma_packets(
    _cs5535au: *mut cs5535audio,
    dma: *mut cs5535audio_dma,
    _substream: *mut snd_pcm_substream,
) {
    snd_dma_free_pages(&mut (*dma).desc_buf);
    (*dma).desc_buf.area = core::ptr::null_mut();
    (*dma).substream = core::ptr::null_mut();
}

unsafe fn snd_cs5535audio_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let cs5535au: *mut cs5535audio = snd_pcm_substream_chip(substream);
    let dma: *mut cs5535audio_dma = (*(*substream).runtime).private_data as *mut cs5535audio_dma;
    let err: c_int;

    (*dma).buf_addr = (*(*substream).runtime).dma_addr;
    (*dma).buf_bytes = params_buffer_bytes(hw_params);

    err = cs5535audio_build_dma_packets(
        cs5535au,
        dma,
        substream,
        params_periods(hw_params),
        params_period_bytes(hw_params),
    );
    if err == 0 {
        (*dma).pcm_open_flag = 1;
    }

    err
}

unsafe fn snd_cs5535audio_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let cs5535au: *mut cs5535audio = snd_pcm_substream_chip(substream);
    let dma: *mut cs5535audio_dma = (*(*substream).runtime).private_data as *mut cs5535audio_dma;

    if (*dma).pcm_open_flag != 0 {
        if substream == (*cs5535au).playback_substream {
            snd_ac97_update_power((*cs5535au).ac97, AC97_PCM_FRONT_DAC_RATE, 0);
        } else {
            snd_ac97_update_power((*cs5535au).ac97, AC97_PCM_LR_ADC_RATE, 0);
        }
        (*dma).pcm_open_flag = 0;
    }
    cs5535audio_clear_dma_packets(cs5535au, dma, substream);
    0
}

unsafe fn snd_cs5535audio_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let cs5535au: *mut cs5535audio = snd_pcm_substream_chip(substream);
    snd_ac97_set_rate(
        (*cs5535au).ac97,
        AC97_PCM_FRONT_DAC_RATE,
        (*(*substream).runtime).rate,
    )
}

unsafe fn snd_cs5535audio_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let cs5535au: *mut cs5535audio = snd_pcm_substream_chip(substream);
    let dma: *mut cs5535audio_dma = (*(*substream).runtime).private_data as *mut cs5535audio_dma;

    guard_spinlock(&mut (*cs5535au).reg_lock);
    match cmd {
        SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            ((*(*dma).ops).pause_dma)(cs5535au);
        }
        SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            ((*(*dma).ops).enable_dma)(cs5535au);
        }
        SNDRV_PCM_TRIGGER_START => {
            ((*(*dma).ops).enable_dma)(cs5535au);
        }
        SNDRV_PCM_TRIGGER_RESUME => {
            ((*(*dma).ops).enable_dma)(cs5535au);
        }
        SNDRV_PCM_TRIGGER_STOP => {
            ((*(*dma).ops).disable_dma)(cs5535au);
        }
        SNDRV_PCM_TRIGGER_SUSPEND => {
            ((*(*dma).ops).disable_dma)(cs5535au);
        }
        _ => {
            dev_err((*(*cs5535au).card).dev, c"unhandled trigger\n".as_ptr());
            return -EINVAL;
        }
    }
    0
}

unsafe fn snd_cs5535audio_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let cs5535au: *mut cs5535audio = snd_pcm_substream_chip(substream);
    let mut curdma: u32;
    let dma: *mut cs5535audio_dma;

    dma = (*(*substream).runtime).private_data as *mut cs5535audio_dma;
    curdma = ((*(*dma).ops).read_dma_pntr)(cs5535au);
    if curdma < (*dma).buf_addr {
        dev_err(
            (*(*cs5535au).card).dev,
            c"curdma=%x < %x bufaddr.\n".as_ptr(),
            curdma,
            (*dma).buf_addr,
        );
        return 0;
    }
    curdma = curdma.wrapping_sub((*dma).buf_addr);
    if curdma >= (*dma).buf_bytes {
        dev_err(
            (*(*cs5535au).card).dev,
            c"diff=%x >= %x buf_bytes.\n".as_ptr(),
            curdma,
            (*dma).buf_bytes,
        );
        return 0;
    }
    bytes_to_frames((*substream).runtime, curdma)
}

unsafe fn snd_cs5535audio_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let err: c_int;
    let cs5535au: *mut cs5535audio = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;

    (*runtime).hw = snd_cs5535audio_capture;
    (*runtime).hw.rates = (*(*cs5535au).ac97).rates[AC97_RATES_ADC as usize];
    snd_pcm_limit_hw_rates(runtime);
    (*cs5535au).capture_substream = substream;
    (*runtime).private_data = &mut (*cs5535au).dmas[CS5535AUDIO_DMA_CAPTURE as usize] as *mut _ as *mut c_void;
    err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if err < 0 {
        return err;
    }
    olpc_capture_open((*cs5535au).ac97);
    0
}

unsafe fn snd_cs5535audio_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    let cs5535au: *mut cs5535audio = snd_pcm_substream_chip(substream);
    olpc_capture_close((*cs5535au).ac97);
    0
}

unsafe fn snd_cs5535audio_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let cs5535au: *mut cs5535audio = snd_pcm_substream_chip(substream);
    snd_ac97_set_rate(
        (*cs5535au).ac97,
        AC97_PCM_LR_ADC_RATE,
        (*(*substream).runtime).rate,
    )
}

static snd_cs5535audio_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_cs5535audio_playback_open),
    close: Some(snd_cs5535audio_playback_close),
    hw_params: Some(snd_cs5535audio_hw_params),
    hw_free: Some(snd_cs5535audio_hw_free),
    prepare: Some(snd_cs5535audio_playback_prepare),
    trigger: Some(snd_cs5535audio_trigger),
    pointer: Some(snd_cs5535audio_pcm_pointer),
};

static snd_cs5535audio_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_cs5535audio_capture_open),
    close: Some(snd_cs5535audio_capture_close),
    hw_params: Some(snd_cs5535audio_hw_params),
    hw_free: Some(snd_cs5535audio_hw_free),
    prepare: Some(snd_cs5535audio_capture_prepare),
    trigger: Some(snd_cs5535audio_trigger),
    pointer: Some(snd_cs5535audio_pcm_pointer),
};

static snd_cs5535audio_playback_dma_ops: cs5535audio_dma_ops = cs5535audio_dma_ops {
    type_: CS5535AUDIO_DMA_PLAYBACK,
    enable_dma: cs5535audio_playback_enable_dma,
    disable_dma: cs5535audio_playback_disable_dma,
    setup_prd: cs5535audio_playback_setup_prd,
    read_prd: cs5535audio_playback_read_prd,
    pause_dma: cs5535audio_playback_pause_dma,
    read_dma_pntr: cs5535audio_playback_read_dma_pntr,
};

static snd_cs5535audio_capture_dma_ops: cs5535audio_dma_ops = cs5535audio_dma_ops {
    type_: CS5535AUDIO_DMA_CAPTURE,
    enable_dma: cs5535audio_capture_enable_dma,
    disable_dma: cs5535audio_capture_disable_dma,
    setup_prd: cs5535audio_capture_setup_prd,
    read_prd: cs5535audio_capture_read_prd,
    pause_dma: cs5535audio_capture_pause_dma,
    read_dma_pntr: cs5535audio_capture_read_dma_pntr,
};

pub unsafe fn snd_cs5535audio_pcm(cs5535au: *mut cs5535audio) -> c_int {
    let mut pcm: *mut snd_pcm = core::ptr::null_mut();
    let err: c_int;

    err = snd_pcm_new((*cs5535au).card, c"CS5535 Audio".as_ptr(), 0, 1, 1, &mut pcm);
    if err < 0 {
        return err;
    }

    (*cs5535au).dmas[CS5535AUDIO_DMA_PLAYBACK as usize].ops = &snd_cs5535audio_playback_dma_ops;
    (*cs5535au).dmas[CS5535AUDIO_DMA_CAPTURE as usize].ops = &snd_cs5535audio_capture_dma_ops;
    snd_pcm_set_ops(
        pcm,
        SNDRV_PCM_STREAM_PLAYBACK,
        &snd_cs5535audio_playback_ops,
    );
    snd_pcm_set_ops(
        pcm,
        SNDRV_PCM_STREAM_CAPTURE,
        &snd_cs5535audio_capture_ops,
    );

    (*pcm).private_data = cs5535au as *mut c_void;
    (*pcm).info_flags = 0;
    strscpy((*pcm).name, c"CS5535 Audio".as_ptr());

    snd_pcm_set_managed_buffer_all(
        pcm,
        SNDRV_DMA_TYPE_DEV,
        &mut (*(*cs5535au).pci).dev,
        64 * 1024,
        128 * 1024,
    );
    (*cs5535au).pcm = pcm;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
