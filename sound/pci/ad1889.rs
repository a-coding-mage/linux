// SPDX-License-Identifier: GPL-2.0-only
/* Analog Devices 1889 audio driver
 *
 * This is a driver for the AD1889 PCI audio chipset found
 * on the HP PA-RISC [BCJ]-xxx0 workstations.
 *
 * Copyright (C) 2004-2005, Kyle McMartin <kyle@parisc-linux.org>
 * Copyright (C) 2005, Thibaut Varene <varenet@parisc-linux.org>
 *   Based on the OSS AD1889 driver by Randolph Chung <tausq@debian.org>
 *
 * TODO:
 *	Do we need to take care of CCS register?
 *	Maybe we could use finer grained locking (separate locks for pb/cap)?
 * Wishlist:
 *	Control Interface (mixer) support
 *	Better AC97 support (VSR...)?
 *	PM support
 *	MIDI support
 *	Game Port support
 *	SG DMA support (this will need *a lot* of work)
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const AD1889_DRVVER: *const c_char = b"Version: 1.7\0".as_ptr() as *const c_char;

/* MODULE_AUTHOR("Kyle McMartin <kyle@parisc-linux.org>, Thibaut Varene <t-bone@parisc-linux.org>"); */
/* MODULE_DESCRIPTION("Analog Devices AD1889 ALSA sound driver"); */
/* MODULE_LICENSE("GPL"); */

static mut index: [c_int; SNDRV_CARDS as usize] = SNDRV_DEFAULT_IDX;
/* module_param_array(index, int, NULL, 0444); */
/* MODULE_PARM_DESC(index, "Index value for the AD1889 soundcard."); */

static mut id: [*mut c_char; SNDRV_CARDS as usize] = SNDRV_DEFAULT_STR;
/* module_param_array(id, charp, NULL, 0444); */
/* MODULE_PARM_DESC(id, "ID string for the AD1889 soundcard."); */

static mut enable: [bool; SNDRV_CARDS as usize] = SNDRV_DEFAULT_ENABLE_PNP;
/* module_param_array(enable, bool, NULL, 0444); */
/* MODULE_PARM_DESC(enable, "Enable AD1889 soundcard."); */

static mut ac97_quirk: [*mut c_char; SNDRV_CARDS as usize] = [ptr::null_mut(); SNDRV_CARDS as usize];
/* module_param_array(ac97_quirk, charp, NULL, 0444); */
/* MODULE_PARM_DESC(ac97_quirk, "AC'97 workaround for strange hardware."); */

const DEVNAME: *const c_char = b"ad1889\0".as_ptr() as *const c_char;
const PFX: *const c_char = b"ad1889: \0".as_ptr() as *const c_char;

/* keep track of some hw registers */
#[repr(C)]
struct ad1889_register_state {
    reg: u16,          /* reg setup */
    addr: u32,         /* dma base address */
    size: c_ulong,     /* DMA buffer size */
}

#[repr(C)]
struct snd_ad1889 {
    card: *mut snd_card,
    pci: *mut pci_dev,

    irq: c_int,
    bar: c_ulong,
    iobase: *mut c_void,

    ac97: *mut snd_ac97,
    ac97_bus: *mut snd_ac97_bus,
    pcm: *mut snd_pcm,
    proc: *mut snd_info_entry,

    psubs: *mut snd_pcm_substream,
    csubs: *mut snd_pcm_substream,

    /* playback register state */
    wave: ad1889_register_state,
    ramc: ad1889_register_state,

    lock: spinlock_t,
}

#[inline]
unsafe fn ad1889_readw(chip: *mut snd_ad1889, reg: c_uint) -> u16 {
    readw(((*chip).iobase as *mut u8).add(reg as usize) as *const c_void)
}

#[inline]
unsafe fn ad1889_writew(chip: *mut snd_ad1889, reg: c_uint, val: u16) {
    writew(val, ((*chip).iobase as *mut u8).add(reg as usize) as *mut c_void);
}

#[inline]
unsafe fn ad1889_readl(chip: *mut snd_ad1889, reg: c_uint) -> u32 {
    readl(((*chip).iobase as *mut u8).add(reg as usize) as *const c_void)
}

#[inline]
unsafe fn ad1889_writel(chip: *mut snd_ad1889, reg: c_uint, val: u32) {
    writel(val, ((*chip).iobase as *mut u8).add(reg as usize) as *mut c_void);
}

#[inline]
unsafe fn ad1889_unmute(chip: *mut snd_ad1889) {
    let mut st: u16;
    st = ad1889_readw(chip, AD_DS_WADA) & !(AD_DS_WADA_RWAM | AD_DS_WADA_LWAM);
    ad1889_writew(chip, AD_DS_WADA, st);
    ad1889_readw(chip, AD_DS_WADA);
}

#[inline]
unsafe fn ad1889_mute(chip: *mut snd_ad1889) {
    let mut st: u16;
    st = ad1889_readw(chip, AD_DS_WADA) | AD_DS_WADA_RWAM | AD_DS_WADA_LWAM;
    ad1889_writew(chip, AD_DS_WADA, st);
    ad1889_readw(chip, AD_DS_WADA);
}

#[inline]
unsafe fn ad1889_load_adc_buffer_address(chip: *mut snd_ad1889, address: u32) {
    ad1889_writel(chip, AD_DMA_ADCBA, address);
    ad1889_writel(chip, AD_DMA_ADCCA, address);
}

#[inline]
unsafe fn ad1889_load_adc_buffer_count(chip: *mut snd_ad1889, count: u32) {
    ad1889_writel(chip, AD_DMA_ADCBC, count);
    ad1889_writel(chip, AD_DMA_ADCCC, count);
}

#[inline]
unsafe fn ad1889_load_adc_interrupt_count(chip: *mut snd_ad1889, count: u32) {
    ad1889_writel(chip, AD_DMA_ADCIB, count);
    ad1889_writel(chip, AD_DMA_ADCIC, count);
}

#[inline]
unsafe fn ad1889_load_wave_buffer_address(chip: *mut snd_ad1889, address: u32) {
    ad1889_writel(chip, AD_DMA_WAVBA, address);
    ad1889_writel(chip, AD_DMA_WAVCA, address);
}

#[inline]
unsafe fn ad1889_load_wave_buffer_count(chip: *mut snd_ad1889, count: u32) {
    ad1889_writel(chip, AD_DMA_WAVBC, count);
    ad1889_writel(chip, AD_DMA_WAVCC, count);
}

#[inline]
unsafe fn ad1889_load_wave_interrupt_count(chip: *mut snd_ad1889, count: u32) {
    ad1889_writel(chip, AD_DMA_WAVIB, count);
    ad1889_writel(chip, AD_DMA_WAVIC, count);
}

unsafe fn ad1889_channel_reset(chip: *mut snd_ad1889, channel: c_uint) {
    let mut reg: u16;

    if (channel & AD_CHAN_WAV) != 0 {
        /* Disable wave channel */
        reg = ad1889_readw(chip, AD_DS_WSMC) & !AD_DS_WSMC_WAEN;
        ad1889_writew(chip, AD_DS_WSMC, reg);
        (*chip).wave.reg = reg;

        /* disable IRQs */
        reg = ad1889_readw(chip, AD_DMA_WAV);
        reg &= AD_DMA_IM_DIS;
        reg &= !AD_DMA_LOOP;
        ad1889_writew(chip, AD_DMA_WAV, reg);

        /* clear IRQ and address counters and pointers */
        ad1889_load_wave_buffer_address(chip, 0x0);
        ad1889_load_wave_buffer_count(chip, 0x0);
        ad1889_load_wave_interrupt_count(chip, 0x0);

        /* flush */
        ad1889_readw(chip, AD_DMA_WAV);
    }

    if (channel & AD_CHAN_ADC) != 0 {
        /* Disable ADC channel */
        reg = ad1889_readw(chip, AD_DS_RAMC) & !AD_DS_RAMC_ADEN;
        ad1889_writew(chip, AD_DS_RAMC, reg);
        (*chip).ramc.reg = reg;

        reg = ad1889_readw(chip, AD_DMA_ADC);
        reg &= AD_DMA_IM_DIS;
        reg &= !AD_DMA_LOOP;
        ad1889_writew(chip, AD_DMA_ADC, reg);

        ad1889_load_adc_buffer_address(chip, 0x0);
        ad1889_load_adc_buffer_count(chip, 0x0);
        ad1889_load_adc_interrupt_count(chip, 0x0);

        /* flush */
        ad1889_readw(chip, AD_DMA_ADC);
    }
}

unsafe extern "C" fn snd_ad1889_ac97_read(ac97: *mut snd_ac97, reg: c_ushort) -> u16 {
    let chip: *mut snd_ad1889 = (*ac97).private_data as *mut snd_ad1889;
    ad1889_readw(chip, AD_AC97_BASE + reg as c_uint)
}

unsafe extern "C" fn snd_ad1889_ac97_write(ac97: *mut snd_ac97, reg: c_ushort, val: c_ushort) {
    let chip: *mut snd_ad1889 = (*ac97).private_data as *mut snd_ad1889;
    ad1889_writew(chip, AD_AC97_BASE + reg as c_uint, val);
}

unsafe fn snd_ad1889_ac97_ready(chip: *mut snd_ad1889) -> c_int {
    let mut retry: c_int = 400; /* average needs 352 msec */

    while (ad1889_readw(chip, AD_AC97_ACIC) & AD_AC97_ACIC_ACRDY) == 0 && {
        retry -= 1;
        retry != 0
    } {
        usleep_range(1000, 2000);
    }
    if retry == 0 {
        dev_err((*(*chip).card).dev, b"[%s] Link is not ready.\n\0".as_ptr() as *const c_char, b"snd_ad1889_ac97_ready\0".as_ptr() as *const c_char);
        return -EIO;
    }
    dev_dbg((*(*chip).card).dev, b"[%s] ready after %d ms\n\0".as_ptr() as *const c_char, b"snd_ad1889_ac97_ready\0".as_ptr() as *const c_char, 400 - retry);

    0
}

static snd_ad1889_playback_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED |
        SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_BLOCK_TRANSFER,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    rate_min: 8000,        /* docs say 7000, but we're lazy */
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: BUFFER_BYTES_MAX,
    period_bytes_min: PERIOD_BYTES_MIN,
    period_bytes_max: PERIOD_BYTES_MAX,
    periods_min: PERIODS_MIN,
    periods_max: PERIODS_MAX,
    /* .fifo_size = 0, */
    ..unsafe { core::mem::zeroed() }
};

static snd_ad1889_capture_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED |
        SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_BLOCK_TRANSFER,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_48000,
    rate_min: 48000,       /* docs say we could to VSR, but we're lazy */
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: BUFFER_BYTES_MAX,
    period_bytes_min: PERIOD_BYTES_MIN,
    period_bytes_max: PERIOD_BYTES_MAX,
    periods_min: PERIODS_MIN,
    periods_max: PERIODS_MAX,
    /* .fifo_size = 0, */
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn snd_ad1889_playback_open(ss: *mut snd_pcm_substream) -> c_int {
    let chip: *mut snd_ad1889 = snd_pcm_substream_chip(ss) as *mut snd_ad1889;
    let rt: *mut snd_pcm_runtime = (*ss).runtime;

    (*chip).psubs = ss;
    (*rt).hw = snd_ad1889_playback_hw;

    0
}

unsafe extern "C" fn snd_ad1889_capture_open(ss: *mut snd_pcm_substream) -> c_int {
    let chip: *mut snd_ad1889 = snd_pcm_substream_chip(ss) as *mut snd_ad1889;
    let rt: *mut snd_pcm_runtime = (*ss).runtime;

    (*chip).csubs = ss;
    (*rt).hw = snd_ad1889_capture_hw;

    0
}

unsafe extern "C" fn snd_ad1889_playback_close(ss: *mut snd_pcm_substream) -> c_int {
    let chip: *mut snd_ad1889 = snd_pcm_substream_chip(ss) as *mut snd_ad1889;
    (*chip).psubs = ptr::null_mut();
    0
}

unsafe extern "C" fn snd_ad1889_capture_close(ss: *mut snd_pcm_substream) -> c_int {
    let chip: *mut snd_ad1889 = snd_pcm_substream_chip(ss) as *mut snd_ad1889;
    (*chip).csubs = ptr::null_mut();
    0
}

unsafe extern "C" fn snd_ad1889_playback_prepare(ss: *mut snd_pcm_substream) -> c_int {
    let chip: *mut snd_ad1889 = snd_pcm_substream_chip(ss) as *mut snd_ad1889;
    let rt: *mut snd_pcm_runtime = (*ss).runtime;
    let size: c_uint = snd_pcm_lib_buffer_bytes(ss);
    let count: c_uint = snd_pcm_lib_period_bytes(ss);
    let mut reg: u16;

    ad1889_channel_reset(chip, AD_CHAN_WAV);

    reg = ad1889_readw(chip, AD_DS_WSMC);

    /* Mask out 16-bit / Stereo */
    reg &= !(AD_DS_WSMC_WA16 | AD_DS_WSMC_WAST);

    if snd_pcm_format_width((*rt).format) == 16 {
        reg |= AD_DS_WSMC_WA16;
    }

    if (*rt).channels > 1 {
        reg |= AD_DS_WSMC_WAST;
    }

    /* let's make sure we don't clobber ourselves */
    spin_lock_irq(&mut (*chip).lock);

    (*chip).wave.size = size as c_ulong;
    (*chip).wave.reg = reg;
    (*chip).wave.addr = (*rt).dma_addr as u32;

    ad1889_writew(chip, AD_DS_WSMC, (*chip).wave.reg);

    /* Set sample rates on the codec */
    ad1889_writew(chip, AD_DS_WAS, (*rt).rate as u16);

    /* Set up DMA */
    ad1889_load_wave_buffer_address(chip, (*chip).wave.addr);
    ad1889_load_wave_buffer_count(chip, size);
    ad1889_load_wave_interrupt_count(chip, count);

    /* writes flush */
    ad1889_readw(chip, AD_DS_WSMC);

    spin_unlock_irq(&mut (*chip).lock);

    dev_dbg((*(*chip).card).dev,
        b"prepare playback: addr = 0x%x, count = %u, size = %u, reg = 0x%x, rate = %u\n\0".as_ptr() as *const c_char,
        (*chip).wave.addr, count, size, reg as c_uint, (*rt).rate);
    0
}

unsafe extern "C" fn snd_ad1889_capture_prepare(ss: *mut snd_pcm_substream) -> c_int {
    let chip: *mut snd_ad1889 = snd_pcm_substream_chip(ss) as *mut snd_ad1889;
    let rt: *mut snd_pcm_runtime = (*ss).runtime;
    let size: c_uint = snd_pcm_lib_buffer_bytes(ss);
    let count: c_uint = snd_pcm_lib_period_bytes(ss);
    let mut reg: u16;

    ad1889_channel_reset(chip, AD_CHAN_ADC);

    reg = ad1889_readw(chip, AD_DS_RAMC);

    /* Mask out 16-bit / Stereo */
    reg &= !(AD_DS_RAMC_AD16 | AD_DS_RAMC_ADST);

    if snd_pcm_format_width((*rt).format) == 16 {
        reg |= AD_DS_RAMC_AD16;
    }

    if (*rt).channels > 1 {
        reg |= AD_DS_RAMC_ADST;
    }

    /* let's make sure we don't clobber ourselves */
    spin_lock_irq(&mut (*chip).lock);

    (*chip).ramc.size = size as c_ulong;
    (*chip).ramc.reg = reg;
    (*chip).ramc.addr = (*rt).dma_addr as u32;

    ad1889_writew(chip, AD_DS_RAMC, (*chip).ramc.reg);

    /* Set up DMA */
    ad1889_load_adc_buffer_address(chip, (*chip).ramc.addr);
    ad1889_load_adc_buffer_count(chip, size);
    ad1889_load_adc_interrupt_count(chip, count);

    /* writes flush */
    ad1889_readw(chip, AD_DS_RAMC);

    spin_unlock_irq(&mut (*chip).lock);

    dev_dbg((*(*chip).card).dev,
        b"prepare capture: addr = 0x%x, count = %u, size = %u, reg = 0x%x, rate = %u\n\0".as_ptr() as *const c_char,
        (*chip).ramc.addr, count, size, reg as c_uint, (*rt).rate);
    0
}

/* this is called in atomic context with IRQ disabled.
   Must be as fast as possible and not sleep.
   DMA should be *triggered* by this call.
   The WSMC "WAEN" bit triggers DMA Wave On/Off */
unsafe extern "C" fn snd_ad1889_playback_trigger(ss: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let mut wsmc: u16;
    let chip: *mut snd_ad1889 = snd_pcm_substream_chip(ss) as *mut snd_ad1889;

    wsmc = ad1889_readw(chip, AD_DS_WSMC);

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            /* enable DMA loop & interrupts */
            ad1889_writew(chip, AD_DMA_WAV, AD_DMA_LOOP | AD_DMA_IM_CNT);
            wsmc |= AD_DS_WSMC_WAEN;
            /* 1 to clear CHSS bit */
            ad1889_writel(chip, AD_DMA_CHSS, AD_DMA_CHSS_WAVS);
            ad1889_unmute(chip);
        }
        SNDRV_PCM_TRIGGER_STOP => {
            ad1889_mute(chip);
            wsmc &= !AD_DS_WSMC_WAEN;
        }
        _ => {
            snd_BUG();
            return -EINVAL;
        }
    }

    (*chip).wave.reg = wsmc;
    ad1889_writew(chip, AD_DS_WSMC, wsmc);
    ad1889_readw(chip, AD_DS_WSMC); /* flush */

    /* reset the chip when STOP - will disable IRQs */
    if cmd == SNDRV_PCM_TRIGGER_STOP {
        ad1889_channel_reset(chip, AD_CHAN_WAV);
    }

    0
}

/* this is called in atomic context with IRQ disabled.
   Must be as fast as possible and not sleep.
   DMA should be *triggered* by this call.
   The RAMC "ADEN" bit triggers DMA ADC On/Off */
unsafe extern "C" fn snd_ad1889_capture_trigger(ss: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let mut ramc: u16;
    let chip: *mut snd_ad1889 = snd_pcm_substream_chip(ss) as *mut snd_ad1889;

    ramc = ad1889_readw(chip, AD_DS_RAMC);

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            /* enable DMA loop & interrupts */
            ad1889_writew(chip, AD_DMA_ADC, AD_DMA_LOOP | AD_DMA_IM_CNT);
            ramc |= AD_DS_RAMC_ADEN;
            /* 1 to clear CHSS bit */
            ad1889_writel(chip, AD_DMA_CHSS, AD_DMA_CHSS_ADCS);
        }
        SNDRV_PCM_TRIGGER_STOP => {
            ramc &= !AD_DS_RAMC_ADEN;
        }
        _ => return -EINVAL,
    }

    (*chip).ramc.reg = ramc;
    ad1889_writew(chip, AD_DS_RAMC, ramc);
    ad1889_readw(chip, AD_DS_RAMC); /* flush */

    /* reset the chip when STOP - will disable IRQs */
    if cmd == SNDRV_PCM_TRIGGER_STOP {
        ad1889_channel_reset(chip, AD_CHAN_ADC);
    }

    0
}

/* Called in atomic context with IRQ disabled */
unsafe extern "C" fn snd_ad1889_playback_pointer(ss: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let mut ptr_: usize = 0;
    let chip: *mut snd_ad1889 = snd_pcm_substream_chip(ss) as *mut snd_ad1889;

    if unlikely(((*chip).wave.reg & AD_DS_WSMC_WAEN) == 0) {
        return 0;
    }

    ptr_ = ad1889_readl(chip, AD_DMA_WAVCA) as usize;
    ptr_ = ptr_.wrapping_sub((*chip).wave.addr as usize);

    if snd_BUG_ON(ptr_ >= (*chip).wave.size as usize) {
        return 0;
    }

    bytes_to_frames((*ss).runtime, ptr_)
}

/* Called in atomic context with IRQ disabled */
unsafe extern "C" fn snd_ad1889_capture_pointer(ss: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let mut ptr_: usize = 0;
    let chip: *mut snd_ad1889 = snd_pcm_substream_chip(ss) as *mut snd_ad1889;

    if unlikely(((*chip).ramc.reg & AD_DS_RAMC_ADEN) == 0) {
        return 0;
    }

    ptr_ = ad1889_readl(chip, AD_DMA_ADCCA) as usize;
    ptr_ = ptr_.wrapping_sub((*chip).ramc.addr as usize);

    if snd_BUG_ON(ptr_ >= (*chip).ramc.size as usize) {
        return 0;
    }

    bytes_to_frames((*ss).runtime, ptr_)
}

static snd_ad1889_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_ad1889_playback_open),
    close: Some(snd_ad1889_playback_close),
    prepare: Some(snd_ad1889_playback_prepare),
    trigger: Some(snd_ad1889_playback_trigger),
    pointer: Some(snd_ad1889_playback_pointer),
    ..unsafe { core::mem::zeroed() }
};

static snd_ad1889_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_ad1889_capture_open),
    close: Some(snd_ad1889_capture_close),
    prepare: Some(snd_ad1889_capture_prepare),
    trigger: Some(snd_ad1889_capture_trigger),
    pointer: Some(snd_ad1889_capture_pointer),
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn snd_ad1889_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let mut st: c_ulong;
    let chip: *mut snd_ad1889 = dev_id as *mut snd_ad1889;

    st = ad1889_readl(chip, AD_DMA_DISR) as c_ulong;

    /* clear ISR */
    ad1889_writel(chip, AD_DMA_DISR, st as u32);

    st &= AD_INTR_MASK as c_ulong;

    if unlikely(st == 0) {
        return IRQ_NONE;
    }

    if (st & (AD_DMA_DISR_PMAI | AD_DMA_DISR_PTAI) as c_ulong) != 0 {
        dev_dbg((*(*chip).card).dev,
            b"Unexpected master or target abort interrupt!\n\0".as_ptr() as *const c_char);
    }

    if (st & AD_DMA_DISR_WAVI as c_ulong) != 0 && !(*chip).psubs.is_null() {
        snd_pcm_period_elapsed((*chip).psubs);
    }
    if (st & AD_DMA_DISR_ADCI as c_ulong) != 0 && !(*chip).csubs.is_null() {
        snd_pcm_period_elapsed((*chip).csubs);
    }

    IRQ_HANDLED
}

unsafe fn snd_ad1889_pcm_init(chip: *mut snd_ad1889, device: c_int) -> c_int {
    let mut err: c_int;
    let mut pcm: *mut snd_pcm = ptr::null_mut();

    err = snd_pcm_new((*chip).card, (*(*chip).card).driver.as_mut_ptr(), device, 1, 1, &mut pcm);
    if err < 0 {
        return err;
    }

    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_ad1889_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_ad1889_capture_ops);

    (*pcm).private_data = chip as *mut c_void;
    (*pcm).info_flags = 0;
    strscpy((*pcm).name.as_mut_ptr(), (*(*chip).card).shortname.as_ptr());

    (*chip).pcm = pcm;
    (*chip).psubs = ptr::null_mut();
    (*chip).csubs = ptr::null_mut();

    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &mut (*(*chip).pci).dev,
                                   BUFFER_BYTES_MAX / 2, BUFFER_BYTES_MAX);

    0
}

unsafe extern "C" fn snd_ad1889_proc_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let chip: *mut snd_ad1889 = (*entry).private_data as *mut snd_ad1889;
    let mut reg: u16;
    let mut tmp: c_int;

    reg = ad1889_readw(chip, AD_DS_WSMC);
    snd_iprintf(buffer, b"Wave output: %s\n\0".as_ptr() as *const c_char,
            str_enabled_disabled((reg & AD_DS_WSMC_WAEN) as c_int));
    snd_iprintf(buffer, b"Wave Channels: %s\n\0".as_ptr() as *const c_char,
            if (reg & AD_DS_WSMC_WAST) != 0 { b"stereo\0".as_ptr() } else { b"mono\0".as_ptr() } as *const c_char);
    snd_iprintf(buffer, b"Wave Quality: %d-bit linear\n\0".as_ptr() as *const c_char,
            if (reg & AD_DS_WSMC_WA16) != 0 { 16 } else { 8 });

    /* WARQ is at offset 12 */
    tmp = if (reg & AD_DS_WSMC_WARQ) != 0 {
        if (((reg & AD_DS_WSMC_WARQ) >> 12) & 0x01) != 0 { 12 } else { 18 }
    } else {
        4
    };
    tmp /= if (reg & AD_DS_WSMC_WAST) != 0 { 2 } else { 1 };

    snd_iprintf(buffer, b"Wave FIFO: %d %s words\n\n\0".as_ptr() as *const c_char, tmp,
            if (reg & AD_DS_WSMC_WAST) != 0 { b"stereo\0".as_ptr() } else { b"mono\0".as_ptr() } as *const c_char);

    snd_iprintf(buffer, b"Synthesis output: %s\n\0".as_ptr() as *const c_char,
            str_enabled_disabled((reg & AD_DS_WSMC_SYEN) as c_int));

    /* SYRQ is at offset 4 */
    tmp = if (reg & AD_DS_WSMC_SYRQ) != 0 {
        if (((reg & AD_DS_WSMC_SYRQ) >> 4) & 0x01) != 0 { 12 } else { 18 }
    } else {
        4
    };
    tmp /= if (reg & AD_DS_WSMC_WAST) != 0 { 2 } else { 1 };

    snd_iprintf(buffer, b"Synthesis FIFO: %d %s words\n\n\0".as_ptr() as *const c_char, tmp,
            if (reg & AD_DS_WSMC_WAST) != 0 { b"stereo\0".as_ptr() } else { b"mono\0".as_ptr() } as *const c_char);

    reg = ad1889_readw(chip, AD_DS_RAMC);
    snd_iprintf(buffer, b"ADC input: %s\n\0".as_ptr() as *const c_char,
            str_enabled_disabled((reg & AD_DS_RAMC_ADEN) as c_int));
    snd_iprintf(buffer, b"ADC Channels: %s\n\0".as_ptr() as *const c_char,
            if (reg & AD_DS_RAMC_ADST) != 0 { b"stereo\0".as_ptr() } else { b"mono\0".as_ptr() } as *const c_char);
    snd_iprintf(buffer, b"ADC Quality: %d-bit linear\n\0".as_ptr() as *const c_char,
            if (reg & AD_DS_RAMC_AD16) != 0 { 16 } else { 8 });

    /* ACRQ is at offset 4 */
    tmp = if (reg & AD_DS_RAMC_ACRQ) != 0 {
        if (((reg & AD_DS_RAMC_ACRQ) >> 4) & 0x01) != 0 { 12 } else { 18 }
    } else {
        4
    };
    tmp /= if (reg & AD_DS_RAMC_ADST) != 0 { 2 } else { 1 };

    snd_iprintf(buffer, b"ADC FIFO: %d %s words\n\n\0".as_ptr() as *const c_char, tmp,
            if (reg & AD_DS_RAMC_ADST) != 0 { b"stereo\0".as_ptr() } else { b"mono\0".as_ptr() } as *const c_char);

    snd_iprintf(buffer, b"Resampler input: %s\n\0".as_ptr() as *const c_char,
            str_enabled_disabled((reg & AD_DS_RAMC_REEN) as c_int));

    /* RERQ is at offset 12 */
    tmp = if (reg & AD_DS_RAMC_RERQ) != 0 {
        if (((reg & AD_DS_RAMC_RERQ) >> 12) & 0x01) != 0 { 12 } else { 18 }
    } else {
        4
    };
    tmp /= if (reg & AD_DS_RAMC_ADST) != 0 { 2 } else { 1 };

    snd_iprintf(buffer, b"Resampler FIFO: %d %s words\n\n\0".as_ptr() as *const c_char, tmp,
            if (reg & AD_DS_WSMC_WAST) != 0 { b"stereo\0".as_ptr() } else { b"mono\0".as_ptr() } as *const c_char);

    /* doc says LSB represents -1.5dB, but the max value (-94.5dB)
    suggests that LSB is -3dB, which is more coherent with the logarithmic
    nature of the dB scale */
    reg = ad1889_readw(chip, AD_DS_WADA);
    snd_iprintf(buffer, b"Left: %s, -%d dB\n\0".as_ptr() as *const c_char,
            if (reg & AD_DS_WADA_LWAM) != 0 { b"mute\0".as_ptr() } else { b"unmute\0".as_ptr() } as *const c_char,
            (((reg & AD_DS_WADA_LWAA) >> 8) as c_int) * 3);
    reg = ad1889_readw(chip, AD_DS_WADA);
    snd_iprintf(buffer, b"Right: %s, -%d dB\n\0".as_ptr() as *const c_char,
            if (reg & AD_DS_WADA_RWAM) != 0 { b"mute\0".as_ptr() } else { b"unmute\0".as_ptr() } as *const c_char,
            ((reg & AD_DS_WADA_RWAA) as c_int) * 3);

    reg = ad1889_readw(chip, AD_DS_WAS);
    snd_iprintf(buffer, b"Wave samplerate: %u Hz\n\0".as_ptr() as *const c_char, reg as c_uint);
    reg = ad1889_readw(chip, AD_DS_RES);
    snd_iprintf(buffer, b"Resampler samplerate: %u Hz\n\0".as_ptr() as *const c_char, reg as c_uint);
}

unsafe fn snd_ad1889_proc_init(chip: *mut snd_ad1889) {
    snd_card_ro_proc_new((*chip).card, (*(*chip).card).driver.as_mut_ptr(),
                         chip as *mut c_void, Some(snd_ad1889_proc_read));
}

static ac97_quirks: [ac97_quirk; 2] = [
    ac97_quirk {
        subvendor: 0x11d4,     /* AD */
        subdevice: 0x1889,     /* AD1889 */
        codec_id: AC97_ID_AD1819,
        name: b"AD1889\0".as_ptr() as *const c_char,
        type_: AC97_TUNE_HP_ONLY,
    },
    ac97_quirk {
        subvendor: 0,
        subdevice: 0,
        codec_id: 0,
        name: ptr::null(),
        type_: 0,
    }, /* terminator */
];

unsafe fn snd_ad1889_ac97_xinit(chip: *mut snd_ad1889) {
    let mut reg: u16;

    reg = ad1889_readw(chip, AD_AC97_ACIC);
    reg |= AD_AC97_ACIC_ACRD;       /* Reset Disable */
    ad1889_writew(chip, AD_AC97_ACIC, reg);
    ad1889_readw(chip, AD_AC97_ACIC);       /* flush posted write */
    udelay(10);
    /* Interface Enable */
    reg |= AD_AC97_ACIC_ACIE;
    ad1889_writew(chip, AD_AC97_ACIC, reg);

    snd_ad1889_ac97_ready(chip);

    /* Audio Stream Output | Variable Sample Rate Mode */
    reg = ad1889_readw(chip, AD_AC97_ACIC);
    reg |= AD_AC97_ACIC_ASOE | AD_AC97_ACIC_VSRM;
    ad1889_writew(chip, AD_AC97_ACIC, reg);
    ad1889_readw(chip, AD_AC97_ACIC); /* flush posted write */
}

unsafe fn snd_ad1889_ac97_init(chip: *mut snd_ad1889, quirk_override: *const c_char) -> c_int {
    let mut err: c_int;
    let mut ac97: snd_ac97_template = core::mem::zeroed();
    static ops: snd_ac97_bus_ops = snd_ac97_bus_ops {
        write: Some(snd_ad1889_ac97_write),
        read: Some(snd_ad1889_ac97_read),
    };

    /* doing that here, it works. */
    snd_ad1889_ac97_xinit(chip);

    err = snd_ac97_bus((*chip).card, 0, &ops, chip as *mut c_void, &mut (*chip).ac97_bus);
    if err < 0 {
        return err;
    }

    ptr::write_bytes(&mut ac97 as *mut snd_ac97_template as *mut u8, 0, size_of::<snd_ac97_template>());
    ac97.private_data = chip as *mut c_void;
    ac97.pci = (*chip).pci;

    err = snd_ac97_mixer((*chip).ac97_bus, &mut ac97, &mut (*chip).ac97);
    if err < 0 {
        return err;
    }

    snd_ac97_tune_hardware((*chip).ac97, ac97_quirks.as_ptr(), quirk_override);

    0
}

unsafe extern "C" fn snd_ad1889_free(card: *mut snd_card) {
    let chip: *mut snd_ad1889 = (*card).private_data as *mut snd_ad1889;

    spin_lock_irq(&mut (*chip).lock);

    ad1889_mute(chip);

    /* Turn off interrupt on count and zero DMA registers */
    ad1889_channel_reset(chip, AD_CHAN_WAV | AD_CHAN_ADC);

    /* clear DISR. If we don't, we'd better jump off the Eiffel Tower */
    ad1889_writel(chip, AD_DMA_DISR, AD_DMA_DISR_PTAI | AD_DMA_DISR_PMAI);
    ad1889_readl(chip, AD_DMA_DISR);        /* flush, dammit! */

    spin_unlock_irq(&mut (*chip).lock);
}

unsafe fn snd_ad1889_create(card: *mut snd_card, pci: *mut pci_dev) -> c_int {
    let chip: *mut snd_ad1889 = (*card).private_data as *mut snd_ad1889;
    let mut err: c_int;

    err = pcim_enable_device(pci);
    if err < 0 {
        return err;
    }

    /* check PCI availability (32bit DMA) */
    if dma_set_mask_and_coherent(&mut (*pci).dev, DMA_BIT_MASK(32)) != 0 {
        dev_err((*card).dev, b"error setting 32-bit DMA mask.\n\0".as_ptr() as *const c_char);
        return -ENXIO;
    }

    (*chip).card = card;
    (*chip).pci = pci;
    (*chip).irq = -1;

    /* (1) PCI resource allocation */
    (*chip).iobase = pcim_iomap_region(pci, 0, (*card).driver.as_mut_ptr());
    if IS_ERR((*chip).iobase) {
        return PTR_ERR((*chip).iobase) as c_int;
    }

    (*chip).bar = pci_resource_start(pci, 0);

    pci_set_master(pci);

    spin_lock_init(&mut (*chip).lock);      /* only now can we call ad1889_free */

    if devm_request_irq(&mut (*pci).dev, (*pci).irq, Some(snd_ad1889_interrupt),
                        IRQF_SHARED, KBUILD_MODNAME, chip as *mut c_void) != 0 {
        dev_err((*card).dev, b"cannot obtain IRQ %d\n\0".as_ptr() as *const c_char, (*pci).irq);
        return -EBUSY;
    }

    (*chip).irq = (*pci).irq;
    (*card).sync_irq = (*chip).irq;
    (*card).private_free = Some(snd_ad1889_free);

    /* (2) initialization of the chip hardware */
    ad1889_writew(chip, AD_DS_CCS, AD_DS_CCS_CLKEN); /* turn on clock */
    ad1889_readw(chip, AD_DS_CCS);  /* flush posted write */

    usleep_range(10000, 11000);

    /* enable Master and Target abort interrupts */
    ad1889_writel(chip, AD_DMA_DISR, AD_DMA_DISR_PMAE | AD_DMA_DISR_PTAE);

    0
}

unsafe fn __snd_ad1889_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    let mut err: c_int;
    static mut devno: c_int = 0;
    let mut card: *mut snd_card = ptr::null_mut();
    let mut chip: *mut snd_ad1889;

    /* (1) */
    if devno >= SNDRV_CARDS {
        return -ENODEV;
    }
    if !enable[devno as usize] {
        devno += 1;
        return -ENOENT;
    }

    /* (2) */
    err = snd_devm_card_new(&mut (*pci).dev, index[devno as usize], id[devno as usize], THIS_MODULE,
                            size_of::<snd_ad1889>(), &mut card);
    if err < 0 {
        return err;
    }
    chip = (*card).private_data as *mut snd_ad1889;

    strscpy((*card).driver.as_mut_ptr(), b"AD1889\0".as_ptr() as *const c_char);
    strscpy((*card).shortname.as_mut_ptr(), b"Analog Devices AD1889\0".as_ptr() as *const c_char);

    /* (3) */
    err = snd_ad1889_create(card, pci);
    if err < 0 {
        return err;
    }

    /* (4) */
    sprintf((*card).longname.as_mut_ptr(), b"%s at 0x%lx irq %i\0".as_ptr() as *const c_char,
            (*card).shortname.as_ptr(), (*chip).bar, (*chip).irq);

    /* (5) */
    /* register AC97 mixer */
    err = snd_ad1889_ac97_init(chip, ac97_quirk[devno as usize]);
    if err < 0 {
        return err;
    }

    err = snd_ad1889_pcm_init(chip, 0);
    if err < 0 {
        return err;
    }

    /* register proc interface */
    snd_ad1889_proc_init(chip);

    /* (6) */
    err = snd_card_register(card);
    if err < 0 {
        return err;
    }

    /* (7) */
    pci_set_drvdata(pci, card as *mut c_void);

    devno += 1;
    0
}

unsafe extern "C" fn snd_ad1889_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    snd_card_free_on_error(&mut (*pci).dev, __snd_ad1889_probe(pci, pci_id))
}

static snd_ad1889_ids: [pci_device_id; 2] = [
    PCI_DEVICE(PCI_VENDOR_ID_ANALOG_DEVICES, PCI_DEVICE_ID_AD1889JS),
    pci_device_id { vendor: 0, device: 0, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
];
/* MODULE_DEVICE_TABLE(pci, snd_ad1889_ids); */

static mut ad1889_pci_driver: pci_driver = pci_driver {
    name: KBUILD_MODNAME,
    id_table: snd_ad1889_ids.as_ptr(),
    probe: Some(snd_ad1889_probe),
    ..unsafe { core::mem::zeroed() }
};

/* module_pci_driver(ad1889_pci_driver); */

type c_ushort = u16;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;

extern "C" {
    static SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS as usize];
    static SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS as usize];
    static SNDRV_DEFAULT_ENABLE_PNP: [bool; SNDRV_CARDS as usize];
    static THIS_MODULE: *mut c_void;
    static KBUILD_MODNAME: *const c_char;

    fn readw(addr: *const c_void) -> u16;
    fn writew(val: u16, addr: *mut c_void);
    fn readl(addr: *const c_void) -> u32;
    fn writel(val: u32, addr: *mut c_void);
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn udelay(usecs: c_ulong);

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn snd_BUG();
    fn snd_BUG_ON(condition: bool) -> bool;
    fn unlikely(condition: bool) -> bool;
    fn snd_pcm_substream_chip(ss: *mut snd_pcm_substream) -> *mut c_void;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn snd_pcm_lib_buffer_bytes(ss: *mut snd_pcm_substream) -> c_uint;
    fn snd_pcm_lib_period_bytes(ss: *mut snd_pcm_substream) -> c_uint;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: usize) -> snd_pcm_uframes_t;
    fn snd_pcm_period_elapsed(ss: *mut snd_pcm_substream);
    fn snd_pcm_new(card: *mut snd_card, id: *mut c_char, device: c_int, playback_count: c_int, capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, type_: c_int, data: *mut device, min: c_uint, max: c_uint);
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn str_enabled_disabled(value: c_int) -> *const c_char;
    fn snd_card_ro_proc_new(card: *mut snd_card, name: *mut c_char, private_data: *mut c_void, read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>);
    fn snd_ac97_bus(card: *mut snd_card, num: c_int, ops: *const snd_ac97_bus_ops, private_data: *mut c_void, rbus: *mut *mut snd_ac97_bus) -> c_int;
    fn snd_ac97_mixer(bus: *mut snd_ac97_bus, template: *mut snd_ac97_template, rac97: *mut *mut snd_ac97) -> c_int;
    fn snd_ac97_tune_hardware(ac97: *mut snd_ac97, quirks: *const ac97_quirk, override_: *const c_char);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn DMA_BIT_MASK(n: c_int) -> u64;
    fn pcim_iomap_region(pci: *mut pci_dev, bar: c_int, name: *mut c_char) -> *mut c_void;
    fn IS_ERR(ptr: *mut c_void) -> bool;
    fn PTR_ERR(ptr: *mut c_void) -> isize;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn pci_set_master(pci: *mut pci_dev);
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn snd_devm_card_new(dev: *mut device, idx: c_int, xid: *mut c_char, module: *mut c_void, extra_size: usize, card_ret: *mut *mut snd_card) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...);
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn snd_card_free_on_error(dev: *mut device, err: c_int) -> c_int;
    fn PCI_DEVICE(vendor: u32, device: u32) -> pci_device_id;
}

extern "C" {
    type device;
    type snd_ac97_bus;
    type snd_pcm;
    type snd_info_buffer;
    type spinlock_t;
}

#[repr(C)]
struct snd_card {
    dev: *mut device,
    private_data: *mut c_void,
    driver: [c_char; 16],
    shortname: [c_char; 32],
    longname: [c_char; 80],
    sync_irq: c_int,
    private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
}

#[repr(C)]
struct pci_dev {
    dev: device,
    irq: c_int,
}

#[repr(C)]
struct snd_ac97 {
    private_data: *mut c_void,
}

#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
struct snd_pcm_runtime {
    hw: snd_pcm_hardware,
    format: c_int,
    channels: c_uint,
    rate: c_uint,
    dma_addr: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_pcm_hardware {
    info: c_uint,
    formats: c_ulong,
    rates: c_uint,
    rate_min: c_uint,
    rate_max: c_uint,
    channels_min: c_uint,
    channels_max: c_uint,
    buffer_bytes_max: c_uint,
    period_bytes_min: c_uint,
    period_bytes_max: c_uint,
    periods_min: c_uint,
    periods_max: c_uint,
    fifo_size: c_uint,
}

#[repr(C)]
struct snd_pcm_ops {
    open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

#[repr(C)]
struct snd_info_entry {
    private_data: *mut c_void,
}

#[repr(C)]
struct snd_ac97_template {
    private_data: *mut c_void,
    pci: *mut pci_dev,
}

#[repr(C)]
struct snd_ac97_bus_ops {
    write: Option<unsafe extern "C" fn(*mut snd_ac97, c_ushort, c_ushort)>,
    read: Option<unsafe extern "C" fn(*mut snd_ac97, c_ushort) -> u16>,
}

#[repr(C)]
struct ac97_quirk {
    subvendor: u16,
    subdevice: u16,
    codec_id: u32,
    name: *const c_char,
    type_: c_int,
}

#[repr(C)]
struct pci_device_id {
    vendor: u32,
    device: u32,
    subvendor: u32,
    subdevice: u32,
    class: u32,
    class_mask: u32,
    driver_data: c_ulong,
}

#[repr(C)]
struct pci_driver {
    name: *const c_char,
    id_table: *const pci_device_id,
    probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
}

extern "C" {
    static SNDRV_CARDS: c_int;
    static AD_DS_WADA: c_uint;
    static AD_DS_WADA_RWAM: u16;
    static AD_DS_WADA_LWAM: u16;
    static AD_DMA_ADCBA: c_uint;
    static AD_DMA_ADCCA: c_uint;
    static AD_DMA_ADCBC: c_uint;
    static AD_DMA_ADCCC: c_uint;
    static AD_DMA_ADCIB: c_uint;
    static AD_DMA_ADCIC: c_uint;
    static AD_DMA_WAVBA: c_uint;
    static AD_DMA_WAVCA: c_uint;
    static AD_DMA_WAVBC: c_uint;
    static AD_DMA_WAVCC: c_uint;
    static AD_DMA_WAVIB: c_uint;
    static AD_DMA_WAVIC: c_uint;
    static AD_CHAN_WAV: c_uint;
    static AD_DS_WSMC: c_uint;
    static AD_DS_WSMC_WAEN: u16;
    static AD_DMA_WAV: c_uint;
    static AD_DMA_IM_DIS: u16;
    static AD_DMA_LOOP: u16;
    static AD_CHAN_ADC: c_uint;
    static AD_DS_RAMC: c_uint;
    static AD_DS_RAMC_ADEN: u16;
    static AD_DMA_ADC: c_uint;
    static AD_AC97_BASE: c_uint;
    static AD_AC97_ACIC: c_uint;
    static AD_AC97_ACIC_ACRDY: u16;
    static EIO: c_int;
    static SNDRV_PCM_INFO_MMAP: c_uint;
    static SNDRV_PCM_INFO_INTERLEAVED: c_uint;
    static SNDRV_PCM_INFO_MMAP_VALID: c_uint;
    static SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_ulong;
    static SNDRV_PCM_RATE_CONTINUOUS: c_uint;
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static BUFFER_BYTES_MAX: c_uint;
    static PERIOD_BYTES_MIN: c_uint;
    static PERIOD_BYTES_MAX: c_uint;
    static PERIODS_MIN: c_uint;
    static PERIODS_MAX: c_uint;
    static AD_DS_WSMC_WA16: u16;
    static AD_DS_WSMC_WAST: u16;
    static AD_DS_WAS: c_uint;
    static AD_DS_RAMC_AD16: u16;
    static AD_DS_RAMC_ADST: u16;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static AD_DMA_IM_CNT: u16;
    static AD_DMA_CHSS: c_uint;
    static AD_DMA_CHSS_WAVS: u32;
    static EINVAL: c_int;
    static AD_DMA_CHSS_ADCS: u32;
    static AD_DMA_DISR: c_uint;
    static AD_INTR_MASK: u32;
    static IRQ_NONE: irqreturn_t;
    static AD_DMA_DISR_PMAI: u32;
    static AD_DMA_DISR_PTAI: u32;
    static AD_DMA_DISR_WAVI: u32;
    static AD_DMA_DISR_ADCI: u32;
    static IRQ_HANDLED: irqreturn_t;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SNDRV_DMA_TYPE_DEV: c_int;
    static AD_DS_WSMC_WARQ: u16;
    static AD_DS_WSMC_SYEN: u16;
    static AD_DS_WSMC_SYRQ: u16;
    static AD_DS_RAMC_ACRQ: u16;
    static AD_DS_RAMC_REEN: u16;
    static AD_DS_RAMC_RERQ: u16;
    static AD_DS_WADA_LWAA: u16;
    static AD_DS_WADA_RWAA: u16;
    static AD_DS_RES: c_uint;
    static AC97_ID_AD1819: u32;
    static AC97_TUNE_HP_ONLY: c_int;
    static AD_AC97_ACIC_ACRD: u16;
    static AD_AC97_ACIC_ACIE: u16;
    static AD_AC97_ACIC_ASOE: u16;
    static AD_AC97_ACIC_VSRM: u16;
    static AD_DMA_DISR_PTAE: u32;
    static AD_DMA_DISR_PMAE: u32;
    static ENXIO: c_int;
    static AD_DS_CCS: c_uint;
    static AD_DS_CCS_CLKEN: u16;
    static IRQF_SHARED: c_ulong;
    static EBUSY: c_int;
    static ENODEV: c_int;
    static ENOENT: c_int;
    static PCI_VENDOR_ID_ANALOG_DEVICES: u32;
    static PCI_DEVICE_ID_AD1889JS: u32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
