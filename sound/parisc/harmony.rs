// SPDX-License-Identifier: GPL-2.0-only
/* Hewlett-Packard Harmony audio driver
 *
 *   This is a driver for the Harmony audio chipset found
 *   on the LASI ASIC of various early HP PA-RISC workstations.
 *
 *   Copyright (C) 2004, Kyle McMartin <kyle@{debian.org,parisc-linux.org}>
 *
 *     Based on the previous Harmony incarnations by,
 *       Copyright 2000 (c) Linuxcare Canada, Alex deVries
 *       Copyright 2000-2003 (c) Helge Deller
 *       Copyright 2001 (c) Matthieu Delahaye
 *       Copyright 2001 (c) Jean-Christophe Vaugeois
 *       Copyright 2003 (c) Laurent Canet
 *       Copyright 2004 (c) Stuart Brady
 *
 * Notes:
 *   - graveyard and silence buffers last for lifetime of
 *     the driver. playback and capture buffers are allocated
 *     per _open()/_close().
 *
 * TODO:
 */

/* C dependencies removed from executable Rust:
 * linux/init.h, linux/slab.h, linux/time.h, linux/wait.h, linux/delay.h,
 * linux/module.h, linux/interrupt.h, linux/spinlock.h, linux/dma-mapping.h,
 * linux/io.h, sound/core.h, sound/pcm.h, sound/control.h, sound/rawmidi.h,
 * sound/initval.h, sound/info.h, asm/hardware.h, asm/parisc-device.h,
 * and "harmony.h".
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::*;
use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

static mut index: c_int = SNDRV_DEFAULT_IDX1; /* Index 0-MAX */
static mut id: *mut c_char = SNDRV_DEFAULT_STR1 as *mut c_char; /* ID for this card */
/* module_param(index, int, 0444);
 * MODULE_PARM_DESC(index, "Index value for Harmony driver.");
 * module_param(id, charp, 0444);
 * MODULE_PARM_DESC(id, "ID string for Harmony driver.");
 */

static snd_harmony_devtable: [parisc_device_id; 5] = [
    /* bushmaster / flounder */
    parisc_device_id { hw_type: HPHW_FIO, hversion_rev: HVERSION_REV_ANY_ID, hversion: HVERSION_ANY_ID, sversion: 0x0007A },
    /* 712 / 715 */
    parisc_device_id { hw_type: HPHW_FIO, hversion_rev: HVERSION_REV_ANY_ID, hversion: HVERSION_ANY_ID, sversion: 0x0007B },
    /* pace */
    parisc_device_id { hw_type: HPHW_FIO, hversion_rev: HVERSION_REV_ANY_ID, hversion: HVERSION_ANY_ID, sversion: 0x0007E },
    /* outfield / coral II */
    parisc_device_id { hw_type: HPHW_FIO, hversion_rev: HVERSION_REV_ANY_ID, hversion: HVERSION_ANY_ID, sversion: 0x0007F },
    parisc_device_id { hw_type: 0, hversion_rev: 0, hversion: 0, sversion: 0 },
];

/* MODULE_DEVICE_TABLE(parisc, snd_harmony_devtable); */

const NAME: &[u8] = b"harmony\0";
const PFX: &[u8] = b"harmony: \0";

static snd_harmony_rates: [c_uint; 14] = [
    5512, 6615, 8000, 9600,
    11025, 16000, 18900, 22050,
    27428, 32000, 33075, 37800,
    44100, 48000,
];

static rate_bits: [c_uint; 14] = [
    HARMONY_SR_5KHZ, HARMONY_SR_6KHZ, HARMONY_SR_8KHZ,
    HARMONY_SR_9KHZ, HARMONY_SR_11KHZ, HARMONY_SR_16KHZ,
    HARMONY_SR_18KHZ, HARMONY_SR_22KHZ, HARMONY_SR_27KHZ,
    HARMONY_SR_32KHZ, HARMONY_SR_33KHZ, HARMONY_SR_37KHZ,
    HARMONY_SR_44KHZ, HARMONY_SR_48KHZ,
];

static hw_constraint_rates: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: snd_harmony_rates.len() as c_uint,
    list: snd_harmony_rates.as_ptr(),
    mask: 0,
};

unsafe fn harmony_read(h: *mut snd_harmony, r: c_uint) -> c_ulong {
    __raw_readl((*h).iobase.add(r as usize)) as c_ulong
}

unsafe fn harmony_write(h: *mut snd_harmony, r: c_uint, v: c_ulong) {
    __raw_writel(v, (*h).iobase.add(r as usize));
}

unsafe fn harmony_wait_for_control(h: *mut snd_harmony) {
    while (harmony_read(h, HARMONY_CNTL) & HARMONY_CNTL_C as c_ulong) != 0 {}
}

unsafe fn harmony_reset(h: *mut snd_harmony) {
    harmony_write(h, HARMONY_RESET, 1);
    mdelay(50);
    harmony_write(h, HARMONY_RESET, 0);
}

unsafe fn harmony_disable_interrupts(h: *mut snd_harmony) {
    let mut dstatus: u32;
    harmony_wait_for_control(h);
    dstatus = harmony_read(h, HARMONY_DSTATUS) as u32;
    dstatus &= !HARMONY_DSTATUS_IE;
    harmony_write(h, HARMONY_DSTATUS, dstatus as c_ulong);
}

unsafe fn harmony_enable_interrupts(h: *mut snd_harmony) {
    let mut dstatus: u32;
    harmony_wait_for_control(h);
    dstatus = harmony_read(h, HARMONY_DSTATUS) as u32;
    dstatus |= HARMONY_DSTATUS_IE;
    harmony_write(h, HARMONY_DSTATUS, dstatus as c_ulong);
}

unsafe fn harmony_mute(h: *mut snd_harmony) {
    let _guard = spin_lock_irqsave_guard(&mut (*h).mixer_lock);
    harmony_wait_for_control(h);
    harmony_write(h, HARMONY_GAINCTL, HARMONY_GAIN_SILENCE as c_ulong);
}

unsafe fn harmony_unmute(h: *mut snd_harmony) {
    let _guard = spin_lock_irqsave_guard(&mut (*h).mixer_lock);
    harmony_wait_for_control(h);
    harmony_write(h, HARMONY_GAINCTL, (*h).st.gain as c_ulong);
}

unsafe fn harmony_set_control(h: *mut snd_harmony) {
    let ctrl: u32;
    let _guard = spin_lock_irqsave_guard(&mut (*h).lock);

    ctrl = HARMONY_CNTL_C
        | ((*h).st.format << 6)
        | ((*h).st.stereo << 5)
        | (*h).st.rate;

    harmony_wait_for_control(h);
    harmony_write(h, HARMONY_CNTL, ctrl as c_ulong);
}

unsafe extern "C" fn snd_harmony_interrupt(_irq: c_int, dev: *mut c_void) -> irqreturn_t {
    let mut dstatus: u32;
    let h = dev as *mut snd_harmony;

    {
        let _guard = spin_lock_guard(&mut (*h).lock);
        harmony_disable_interrupts(h);
        harmony_wait_for_control(h);
        dstatus = harmony_read(h, HARMONY_DSTATUS) as u32;
    }

    if (dstatus & HARMONY_DSTATUS_PN) != 0 {
        if !(*h).psubs.is_null() && (*h).st.playing != 0 {
            {
                let _guard = spin_lock_guard(&mut (*h).lock);
                (*h).pbuf.buf = ((*h).pbuf.buf).wrapping_add((*h).pbuf.count); /* PAGE_SIZE */
                (*h).pbuf.buf %= (*h).pbuf.size; /* MAX_BUFS*PAGE_SIZE */
                harmony_write(h, HARMONY_PNXTADD, ((*h).pbuf.addr).wrapping_add((*h).pbuf.buf) as c_ulong);
                (*h).stats.play_intr = (*h).stats.play_intr.wrapping_add(1);
            }
            snd_pcm_period_elapsed((*h).psubs);
        } else {
            let _guard = spin_lock_guard(&mut (*h).lock);
            harmony_write(h, HARMONY_PNXTADD, (*h).sdma.addr as c_ulong);
            (*h).stats.silence_intr = (*h).stats.silence_intr.wrapping_add(1);
        }
    }

    if (dstatus & HARMONY_DSTATUS_RN) != 0 {
        if !(*h).csubs.is_null() && (*h).st.capturing != 0 {
            {
                let _guard = spin_lock_guard(&mut (*h).lock);
                (*h).cbuf.buf = ((*h).cbuf.buf).wrapping_add((*h).cbuf.count);
                (*h).cbuf.buf %= (*h).cbuf.size;
                harmony_write(h, HARMONY_RNXTADD, ((*h).cbuf.addr).wrapping_add((*h).cbuf.buf) as c_ulong);
                (*h).stats.rec_intr = (*h).stats.rec_intr.wrapping_add(1);
            }
            snd_pcm_period_elapsed((*h).csubs);
        } else {
            let _guard = spin_lock_guard(&mut (*h).lock);
            harmony_write(h, HARMONY_RNXTADD, (*h).gdma.addr as c_ulong);
            (*h).stats.graveyard_intr = (*h).stats.graveyard_intr.wrapping_add(1);
        }
    }

    {
        let _guard = spin_lock_guard(&mut (*h).lock);
        harmony_enable_interrupts(h);
    }

    IRQ_HANDLED
}

fn snd_harmony_rate_bits(rate: c_int) -> c_uint {
    for i in 0..snd_harmony_rates.len() {
        if snd_harmony_rates[i] == rate as c_uint {
            return rate_bits[i];
        }
    }
    HARMONY_SR_44KHZ
}

static snd_harmony_playback: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED |
        SNDRV_PCM_INFO_JOINT_DUPLEX | SNDRV_PCM_INFO_MMAP_VALID |
        SNDRV_PCM_INFO_BLOCK_TRANSFER,
    formats: SNDRV_PCM_FMTBIT_S16_BE | SNDRV_PCM_FMTBIT_MU_LAW | SNDRV_PCM_FMTBIT_A_LAW,
    rates: SNDRV_PCM_RATE_5512 | SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_KNOT,
    rate_min: 5512,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: MAX_BUF_SIZE,
    period_bytes_min: BUF_SIZE,
    period_bytes_max: BUF_SIZE,
    periods_min: 1,
    periods_max: MAX_BUFS,
    fifo_size: 0,
};

static snd_harmony_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED |
        SNDRV_PCM_INFO_JOINT_DUPLEX | SNDRV_PCM_INFO_MMAP_VALID |
        SNDRV_PCM_INFO_BLOCK_TRANSFER,
    formats: SNDRV_PCM_FMTBIT_S16_BE | SNDRV_PCM_FMTBIT_MU_LAW | SNDRV_PCM_FMTBIT_A_LAW,
    rates: SNDRV_PCM_RATE_5512 | SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_KNOT,
    rate_min: 5512,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: MAX_BUF_SIZE,
    period_bytes_min: BUF_SIZE,
    period_bytes_max: BUF_SIZE,
    periods_min: 1,
    periods_max: MAX_BUFS,
    fifo_size: 0,
};

unsafe extern "C" fn snd_harmony_playback_trigger(ss: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let h = snd_pcm_substream_chip(ss) as *mut snd_harmony;
    if (*h).st.capturing != 0 {
        return -EBUSY;
    }

    let _guard = spin_lock_guard(&mut (*h).lock);
    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            (*h).st.playing = 1;
            harmony_write(h, HARMONY_PNXTADD, (*h).pbuf.addr as c_ulong);
            harmony_write(h, HARMONY_RNXTADD, (*h).gdma.addr as c_ulong);
            harmony_unmute(h);
            harmony_enable_interrupts(h);
        }
        SNDRV_PCM_TRIGGER_STOP => {
            (*h).st.playing = 0;
            harmony_mute(h);
            harmony_write(h, HARMONY_PNXTADD, (*h).sdma.addr as c_ulong);
            harmony_disable_interrupts(h);
        }
        SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_SUSPEND | _ => {
            snd_BUG();
            return -EINVAL;
        }
    }
    0
}

unsafe extern "C" fn snd_harmony_capture_trigger(ss: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let h = snd_pcm_substream_chip(ss) as *mut snd_harmony;
    if (*h).st.playing != 0 {
        return -EBUSY;
    }

    let _guard = spin_lock_guard(&mut (*h).lock);
    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            (*h).st.capturing = 1;
            harmony_write(h, HARMONY_PNXTADD, (*h).sdma.addr as c_ulong);
            harmony_write(h, HARMONY_RNXTADD, (*h).cbuf.addr as c_ulong);
            harmony_unmute(h);
            harmony_enable_interrupts(h);
        }
        SNDRV_PCM_TRIGGER_STOP => {
            (*h).st.capturing = 0;
            harmony_mute(h);
            harmony_write(h, HARMONY_RNXTADD, (*h).gdma.addr as c_ulong);
            harmony_disable_interrupts(h);
        }
        SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_SUSPEND | _ => {
            snd_BUG();
            return -EINVAL;
        }
    }
    0
}

unsafe fn snd_harmony_set_data_format(h: *mut snd_harmony, fmt: c_int, force: c_int) -> c_int {
    let o = (*h).st.format;
    let n = match fmt {
        SNDRV_PCM_FORMAT_S16_BE => HARMONY_DF_16BIT_LINEAR,
        SNDRV_PCM_FORMAT_A_LAW => HARMONY_DF_8BIT_ALAW,
        SNDRV_PCM_FORMAT_MU_LAW => HARMONY_DF_8BIT_ULAW,
        _ => HARMONY_DF_16BIT_LINEAR,
    };

    if force != 0 || o != n {
        snd_pcm_format_set_silence(
            fmt,
            (*h).sdma.area,
            SILENCE_BUFSZ / (snd_pcm_format_physical_width(fmt) / 8),
        );
    }
    n
}

unsafe extern "C" fn snd_harmony_playback_prepare(ss: *mut snd_pcm_substream) -> c_int {
    let h = snd_pcm_substream_chip(ss) as *mut snd_harmony;
    let rt = (*ss).runtime;
    if (*h).st.capturing != 0 {
        return -EBUSY;
    }

    (*h).pbuf.size = snd_pcm_lib_buffer_bytes(ss);
    (*h).pbuf.count = snd_pcm_lib_period_bytes(ss);
    if (*h).pbuf.buf >= (*h).pbuf.size {
        (*h).pbuf.buf = 0;
    }
    (*h).st.playing = 0;
    (*h).st.rate = snd_harmony_rate_bits((*rt).rate);
    (*h).st.format = snd_harmony_set_data_format(h, (*rt).format, 0);
    (*h).st.stereo = if (*rt).channels == 2 { HARMONY_SS_STEREO } else { HARMONY_SS_MONO };
    harmony_set_control(h);
    (*h).pbuf.addr = (*rt).dma_addr;
    0
}

unsafe extern "C" fn snd_harmony_capture_prepare(ss: *mut snd_pcm_substream) -> c_int {
    let h = snd_pcm_substream_chip(ss) as *mut snd_harmony;
    let rt = (*ss).runtime;
    if (*h).st.playing != 0 {
        return -EBUSY;
    }

    (*h).cbuf.size = snd_pcm_lib_buffer_bytes(ss);
    (*h).cbuf.count = snd_pcm_lib_period_bytes(ss);
    if (*h).cbuf.buf >= (*h).cbuf.size {
        (*h).cbuf.buf = 0;
    }
    (*h).st.capturing = 0;
    (*h).st.rate = snd_harmony_rate_bits((*rt).rate);
    (*h).st.format = snd_harmony_set_data_format(h, (*rt).format, 0);
    (*h).st.stereo = if (*rt).channels == 2 { HARMONY_SS_STEREO } else { HARMONY_SS_MONO };
    harmony_set_control(h);
    (*h).cbuf.addr = (*rt).dma_addr;
    0
}

unsafe extern "C" fn snd_harmony_playback_pointer(ss: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let rt = (*ss).runtime;
    let h = snd_pcm_substream_chip(ss) as *mut snd_harmony;
    let pcuradd: c_ulong;
    let played: c_ulong;

    if (*h).st.playing == 0 || (*h).psubs.is_null() {
        return 0;
    }
    if (*h).pbuf.addr == 0 || (*h).pbuf.size == 0 {
        return 0;
    }

    pcuradd = harmony_read(h, HARMONY_PCURADD);
    played = pcuradd.wrapping_sub((*h).pbuf.addr as c_ulong);

    /* HARMONY_DEBUG: printk(KERN_DEBUG PFX "playback_pointer is 0x%lx-0x%lx = %d bytes\n",
     * pcuradd, h->pbuf.addr, played);
     */

    if pcuradd > ((*h).pbuf.addr).wrapping_add((*h).pbuf.size) as c_ulong {
        return 0;
    }
    bytes_to_frames(rt, played)
}

unsafe extern "C" fn snd_harmony_capture_pointer(ss: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let rt = (*ss).runtime;
    let h = snd_pcm_substream_chip(ss) as *mut snd_harmony;
    let rcuradd: c_ulong;
    let caught: c_ulong;

    if (*h).st.capturing == 0 || (*h).csubs.is_null() {
        return 0;
    }
    if (*h).cbuf.addr == 0 || (*h).cbuf.size == 0 {
        return 0;
    }

    rcuradd = harmony_read(h, HARMONY_RCURADD);
    caught = rcuradd.wrapping_sub((*h).cbuf.addr as c_ulong);

    /* HARMONY_DEBUG: printk(KERN_DEBUG PFX "capture_pointer is 0x%lx-0x%lx = %d bytes\n",
     * rcuradd, h->cbuf.addr, caught);
     */

    if rcuradd > ((*h).cbuf.addr).wrapping_add((*h).cbuf.size) as c_ulong {
        return 0;
    }
    bytes_to_frames(rt, caught)
}

unsafe extern "C" fn snd_harmony_playback_open(ss: *mut snd_pcm_substream) -> c_int {
    let h = snd_pcm_substream_chip(ss) as *mut snd_harmony;
    let rt = (*ss).runtime;
    let mut err: c_int;

    (*h).psubs = ss;
    (*rt).hw = snd_harmony_playback;
    snd_pcm_hw_constraint_list(rt, 0, SNDRV_PCM_HW_PARAM_RATE, &hw_constraint_rates);
    err = snd_pcm_hw_constraint_integer(rt, SNDRV_PCM_HW_PARAM_PERIODS);
    if err < 0 {
        return err;
    }
    0
}

unsafe extern "C" fn snd_harmony_capture_open(ss: *mut snd_pcm_substream) -> c_int {
    let h = snd_pcm_substream_chip(ss) as *mut snd_harmony;
    let rt = (*ss).runtime;
    let mut err: c_int;

    (*h).csubs = ss;
    (*rt).hw = snd_harmony_capture;
    snd_pcm_hw_constraint_list(rt, 0, SNDRV_PCM_HW_PARAM_RATE, &hw_constraint_rates);
    err = snd_pcm_hw_constraint_integer(rt, SNDRV_PCM_HW_PARAM_PERIODS);
    if err < 0 {
        return err;
    }
    0
}

unsafe extern "C" fn snd_harmony_playback_close(ss: *mut snd_pcm_substream) -> c_int {
    let h = snd_pcm_substream_chip(ss) as *mut snd_harmony;
    (*h).psubs = ptr::null_mut();
    0
}

unsafe extern "C" fn snd_harmony_capture_close(ss: *mut snd_pcm_substream) -> c_int {
    let h = snd_pcm_substream_chip(ss) as *mut snd_harmony;
    (*h).csubs = ptr::null_mut();
    0
}

static snd_harmony_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_harmony_playback_open),
    close: Some(snd_harmony_playback_close),
    prepare: Some(snd_harmony_playback_prepare),
    trigger: Some(snd_harmony_playback_trigger),
    pointer: Some(snd_harmony_playback_pointer),
};

static snd_harmony_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_harmony_capture_open),
    close: Some(snd_harmony_capture_close),
    prepare: Some(snd_harmony_capture_prepare),
    trigger: Some(snd_harmony_capture_trigger),
    pointer: Some(snd_harmony_capture_pointer),
};

unsafe fn snd_harmony_pcm_init(h: *mut snd_harmony) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let mut err: c_int;

    if snd_BUG_ON(h.is_null()) != 0 {
        return -EINVAL;
    }

    harmony_disable_interrupts(h);
    err = snd_pcm_new((*h).card, b"harmony\0".as_ptr() as *const c_char, 0, 1, 1, &mut pcm);
    if err < 0 {
        return err;
    }

    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_harmony_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_harmony_capture_ops);

    (*pcm).private_data = h as *mut c_void;
    (*pcm).info_flags = 0;
    strscpy((*pcm).name.as_mut_ptr(), b"harmony\0".as_ptr() as *const c_char);
    (*h).pcm = pcm;
    (*h).psubs = ptr::null_mut();
    (*h).csubs = ptr::null_mut();

    /* initialize graveyard buffer */
    (*h).dma.type_ = SNDRV_DMA_TYPE_DEV;
    (*h).dma.dev = &mut (*(*h).dev).dev;
    err = snd_dma_alloc_pages((*h).dma.type_, (*h).dma.dev, BUF_SIZE * GRAVEYARD_BUFS, &mut (*h).gdma);
    if err < 0 {
        printk(b"%scannot allocate graveyard buffer!\n\0".as_ptr() as *const c_char, PFX.as_ptr());
        return err;
    }

    /* initialize silence buffers */
    err = snd_dma_alloc_pages((*h).dma.type_, (*h).dma.dev, BUF_SIZE * SILENCE_BUFS, &mut (*h).sdma);
    if err < 0 {
        printk(b"%scannot allocate silence buffer!\n\0".as_ptr() as *const c_char, PFX.as_ptr());
        return err;
    }

    /* pre-allocate space for DMA */
    snd_pcm_set_managed_buffer_all(pcm, (*h).dma.type_, (*h).dma.dev, MAX_BUF_SIZE, MAX_BUF_SIZE);
    (*h).st.format = snd_harmony_set_data_format(h, SNDRV_PCM_FORMAT_S16_BE, 1);
    0
}

unsafe fn snd_harmony_set_new_gain(h: *mut snd_harmony) {
    harmony_wait_for_control(h);
    harmony_write(h, HARMONY_GAINCTL, (*h).st.gain as c_ulong);
}

unsafe extern "C" fn snd_harmony_mixercontrol_info(kc: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let mask = ((*kc).private_value >> 16) & 0xff;
    let left_shift = (*kc).private_value & 0xff;
    let right_shift = ((*kc).private_value >> 8) & 0xff;

    (*uinfo).type_ = if mask == 1 { SNDRV_CTL_ELEM_TYPE_BOOLEAN } else { SNDRV_CTL_ELEM_TYPE_INTEGER };
    (*uinfo).count = if left_shift == right_shift { 1 } else { 2 };
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = mask;
    0
}

unsafe extern "C" fn snd_harmony_volume_get(kc: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let h = snd_kcontrol_chip(kc) as *mut snd_harmony;
    let shift_left = (*kc).private_value & 0xff;
    let shift_right = ((*kc).private_value >> 8) & 0xff;
    let mask = ((*kc).private_value >> 16) & 0xff;
    let invert = ((*kc).private_value >> 24) & 0xff;
    let mut left: c_int;
    let mut right: c_int;

    let _guard = spin_lock_irq_guard(&mut (*h).mixer_lock);
    left = ((*h).st.gain >> shift_left) & mask;
    right = ((*h).st.gain >> shift_right) & mask;
    if invert != 0 {
        left = mask - left;
        right = mask - right;
    }
    (*ucontrol).value.integer.value[0] = left as c_long;
    if shift_left != shift_right {
        (*ucontrol).value.integer.value[1] = right as c_long;
    }
    0
}

unsafe extern "C" fn snd_harmony_volume_put(kc: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let h = snd_kcontrol_chip(kc) as *mut snd_harmony;
    let shift_left = (*kc).private_value & 0xff;
    let shift_right = ((*kc).private_value >> 8) & 0xff;
    let mask = ((*kc).private_value >> 16) & 0xff;
    let invert = ((*kc).private_value >> 24) & 0xff;
    let mut left: c_int;
    let mut right: c_int;
    let old_gain = (*h).st.gain;

    let _guard = spin_lock_irq_guard(&mut (*h).mixer_lock);
    left = ((*ucontrol).value.integer.value[0] as c_int) & mask;
    if invert != 0 {
        left = mask - left;
    }
    (*h).st.gain &= !(mask << shift_left);
    (*h).st.gain |= left << shift_left;

    if shift_left != shift_right {
        right = ((*ucontrol).value.integer.value[1] as c_int) & mask;
        if invert != 0 {
            right = mask - right;
        }
        (*h).st.gain &= !(mask << shift_right);
        (*h).st.gain |= right << shift_right;
    }

    snd_harmony_set_new_gain(h);
    ((*h).st.gain != old_gain) as c_int
}

unsafe extern "C" fn snd_harmony_captureroute_info(_kc: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    static texts: [*const c_char; 2] = [
        b"Line\0".as_ptr() as *const c_char,
        b"Mic\0".as_ptr() as *const c_char,
    ];
    snd_ctl_enum_info(uinfo, 1, 2, texts.as_ptr())
}

unsafe extern "C" fn snd_harmony_captureroute_get(kc: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let h = snd_kcontrol_chip(kc) as *mut snd_harmony;
    let value: c_int;

    let _guard = spin_lock_irq_guard(&mut (*h).mixer_lock);
    value = ((*h).st.gain >> HARMONY_GAIN_IS_SHIFT) & 1;
    (*ucontrol).value.enumerated.item[0] = value as c_uint;
    0
}

unsafe extern "C" fn snd_harmony_captureroute_put(kc: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let h = snd_kcontrol_chip(kc) as *mut snd_harmony;
    let value: c_int;
    let old_gain = (*h).st.gain;

    let _guard = spin_lock_irq_guard(&mut (*h).mixer_lock);
    value = ((*ucontrol).value.enumerated.item[0] as c_int) & 1;
    (*h).st.gain &= !HARMONY_GAIN_IS_MASK;
    (*h).st.gain |= value << HARMONY_GAIN_IS_SHIFT;
    snd_harmony_set_new_gain(h);
    ((*h).st.gain != old_gain) as c_int
}

const HARMONY_CONTROLS: usize = snd_harmony_controls.len();

const fn HARMONY_VOLUME(
    xname: *const c_char,
    left_shift: c_int,
    right_shift: c_int,
    mask: c_int,
    invert: c_int,
) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: xname,
        info: Some(snd_harmony_mixercontrol_info),
        get: Some(snd_harmony_volume_get),
        put: Some(snd_harmony_volume_put),
        private_value: (left_shift | (right_shift << 8) | (mask << 16) | (invert << 24)) as c_ulong,
    }
}

static snd_harmony_controls: [snd_kcontrol_new; 7] = [
    HARMONY_VOLUME(b"Master Playback Volume\0".as_ptr() as *const c_char, HARMONY_GAIN_LO_SHIFT, HARMONY_GAIN_RO_SHIFT, HARMONY_GAIN_OUT, 1),
    HARMONY_VOLUME(b"Capture Volume\0".as_ptr() as *const c_char, HARMONY_GAIN_LI_SHIFT, HARMONY_GAIN_RI_SHIFT, HARMONY_GAIN_IN, 0),
    HARMONY_VOLUME(b"Monitor Volume\0".as_ptr() as *const c_char, HARMONY_GAIN_MA_SHIFT, HARMONY_GAIN_MA_SHIFT, HARMONY_GAIN_MA, 1),
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Input Route\0".as_ptr() as *const c_char,
        info: Some(snd_harmony_captureroute_info),
        get: Some(snd_harmony_captureroute_get),
        put: Some(snd_harmony_captureroute_put),
        private_value: 0,
    },
    HARMONY_VOLUME(b"Internal Speaker Switch\0".as_ptr() as *const c_char, HARMONY_GAIN_SE_SHIFT, HARMONY_GAIN_SE_SHIFT, 1, 0),
    HARMONY_VOLUME(b"Line-Out Switch\0".as_ptr() as *const c_char, HARMONY_GAIN_LE_SHIFT, HARMONY_GAIN_LE_SHIFT, 1, 0),
    HARMONY_VOLUME(b"Headphones Switch\0".as_ptr() as *const c_char, HARMONY_GAIN_HE_SHIFT, HARMONY_GAIN_HE_SHIFT, 1, 0),
];

unsafe fn snd_harmony_mixer_reset(h: *mut snd_harmony) {
    harmony_mute(h);
    harmony_reset(h);
    (*h).st.gain = HARMONY_GAIN_DEFAULT;
    harmony_unmute(h);
}

unsafe fn snd_harmony_mixer_init(h: *mut snd_harmony) -> c_int {
    let card: *mut snd_card;
    let mut idx: c_int;
    let mut err: c_int;

    if snd_BUG_ON(h.is_null()) != 0 {
        return -EINVAL;
    }
    card = (*h).card;
    strscpy((*card).mixername.as_mut_ptr(), b"Harmony Gain control interface\0".as_ptr() as *const c_char);

    idx = 0;
    while idx < HARMONY_CONTROLS as c_int {
        err = snd_ctl_add(card, snd_ctl_new1(&snd_harmony_controls[idx as usize], h as *mut c_void));
        if err < 0 {
            return err;
        }
        idx += 1;
    }

    snd_harmony_mixer_reset(h);
    0
}

unsafe fn snd_harmony_free(h: *mut snd_harmony) -> c_int {
    if (*h).gdma.addr != 0 {
        snd_dma_free_pages(&mut (*h).gdma);
    }
    if (*h).sdma.addr != 0 {
        snd_dma_free_pages(&mut (*h).sdma);
    }
    if (*h).irq >= 0 {
        free_irq((*h).irq, h as *mut c_void);
    }
    iounmap((*h).iobase);
    kfree(h as *mut c_void);
    0
}

unsafe extern "C" fn snd_harmony_dev_free(dev: *mut snd_device) -> c_int {
    let h = (*dev).device_data as *mut snd_harmony;
    snd_harmony_free(h)
}

unsafe fn snd_harmony_create(card: *mut snd_card, padev: *mut parisc_device, rchip: *mut *mut snd_harmony) -> c_int {
    let mut err: c_int;
    let h: *mut snd_harmony;
    static ops: snd_device_ops = snd_device_ops {
        dev_free: Some(snd_harmony_dev_free),
    };

    *rchip = ptr::null_mut();
    h = kzalloc_obj_snd_harmony();
    if h.is_null() {
        return -ENOMEM;
    }

    (*h).hpa = (*padev).hpa.start;
    (*h).card = card;
    (*h).dev = padev;
    (*h).irq = -1;
    (*h).iobase = ioremap((*padev).hpa.start, HARMONY_SIZE);
    if (*h).iobase.is_null() {
        printk(b"%sunable to remap hpa 0x%lx\n\0".as_ptr() as *const c_char, PFX.as_ptr(), (*padev).hpa.start as c_ulong);
        err = -EBUSY;
        snd_harmony_free(h);
        return err;
    }

    err = request_irq((*padev).irq, Some(snd_harmony_interrupt), 0, b"harmony\0".as_ptr() as *const c_char, h as *mut c_void);
    if err != 0 {
        printk(b"%scould not obtain interrupt %d\0".as_ptr() as *const c_char, PFX.as_ptr(), (*padev).irq);
        snd_harmony_free(h);
        return err;
    }
    (*h).irq = (*padev).irq;

    spin_lock_init(&mut (*h).mixer_lock);
    spin_lock_init(&mut (*h).lock);

    err = snd_device_new(card, SNDRV_DEV_LOWLEVEL, h as *mut c_void, &ops);
    if err < 0 {
        snd_harmony_free(h);
        return err;
    }

    *rchip = h;
    0
}

unsafe extern "C" fn snd_harmony_probe(padev: *mut parisc_device) -> c_int {
    let mut err: c_int;
    let mut card: *mut snd_card = ptr::null_mut();
    let mut h: *mut snd_harmony = ptr::null_mut();

    err = snd_card_new(&mut (*padev).dev, index, id, THIS_MODULE, 0, &mut card);
    if err < 0 {
        return err;
    }

    err = snd_harmony_create(card, padev, &mut h);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    err = snd_harmony_pcm_init(h);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    err = snd_harmony_mixer_init(h);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    strscpy((*card).driver.as_mut_ptr(), b"harmony\0".as_ptr() as *const c_char);
    strscpy((*card).shortname.as_mut_ptr(), b"Harmony\0".as_ptr() as *const c_char);
    sprintf((*card).longname.as_mut_ptr(), b"%s at 0x%lx, irq %i\0".as_ptr() as *const c_char, (*card).shortname.as_ptr(), (*h).hpa, (*h).irq);

    err = snd_card_register(card);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    parisc_set_drvdata(padev, card as *mut c_void);
    0
}

unsafe extern "C" fn snd_harmony_remove(padev: *mut parisc_device) {
    snd_card_free(parisc_get_drvdata(padev) as *mut snd_card);
}

static mut snd_harmony_driver: parisc_driver = parisc_driver {
    name: b"harmony\0".as_ptr() as *const c_char,
    id_table: snd_harmony_devtable.as_ptr(),
    probe: Some(snd_harmony_probe),
    remove: Some(snd_harmony_remove),
};

unsafe extern "C" fn alsa_harmony_init() -> c_int {
    register_parisc_driver(&mut snd_harmony_driver)
}

unsafe extern "C" fn alsa_harmony_fini() {
    unregister_parisc_driver(&mut snd_harmony_driver);
}

/* MODULE_LICENSE("GPL");
 * MODULE_AUTHOR("Kyle McMartin <kyle@parisc-linux.org>");
 * MODULE_DESCRIPTION("Harmony sound driver");
 * module_init(alsa_harmony_init);
 * module_exit(alsa_harmony_fini);
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
