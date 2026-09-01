// SPDX-License-Identifier: GPL-2.0-or-later
/*********************************************************************
 *
 * 2002/06/30 Karsten Wiese:
 *	removed kernel-version dependencies.
 *	ripped from linux kernel 2.4.18 (OSS Implementation) by me.
 *	In the OSS Version, this file is compiled to a separate MODULE,
 *	that is used by the pinnacle and the classic driver.
 *	since there is no classic driver for alsa yet (i dont have a classic
 *	& writing one blindfold is difficult) this file's object is statically
 *	linked into the pinnacle-driver-module for now.	look for the string
 *		"uncomment this to make this a module again"
 *	to do guess what.
 *
 * the following is a copy of the 2.4.18 OSS FREE file-heading comment:
 *
 * msnd.c - Driver Base
 *
 * Turtle Beach MultiSound Sound Card Driver for Linux
 *
 * Copyright (C) 1998 Andrew Veliath
 *
 ********************************************************************/

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

const LOGNAME: &[u8] = b"msnd\0";

type u8 = u8;
type u16 = u16;
type u32 = u32;
type snd_pcm_uframes_t = c_ulong;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    pub f_mode: c_int,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub dma_area: *mut c_void,
    pub dma_addr: c_ulong,
    pub dma_bytes: c_uint,
    pub hw: snd_pcm_hardware,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm {
    pub private_data: *mut c_void,
    pub name: [c_char; 80],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: c_ulong,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub buffer_bytes_max: c_uint,
    pub period_bytes_min: c_uint,
    pub period_bytes_max: c_uint,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub fifo_size: c_uint,
}

#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pub mmap: Option<unsafe extern "C" fn() -> c_int>,
}

#[repr(C)]
pub struct snd_msnd {
    pub io: c_uint,
    pub lock: c_ulong,
    pub card: *mut snd_card,
    pub type_: c_int,
    pub irqid: u8,
    pub irq: c_uint,
    pub DSPQ: *mut c_void,
    pub dspq_data_buff: c_int,
    pub dspq_buff_size: c_int,
    pub irq_ref: c_int,
    pub play_sample_size: c_long,
    pub play_sample_rate: c_long,
    pub play_channels: c_long,
    pub mode: c_int,
    pub flags: c_ulong,
    pub play_period_bytes: c_uint,
    pub DARQ: *mut c_void,
    pub capturePeriods: c_uint,
    pub mappedbase: *mut c_void,
    pub capturePeriodBytes: c_uint,
    pub DAPQ: *mut c_void,
    pub playPeriods: c_uint,
    pub last_playbank: c_int,
    pub playLimit: c_uint,
    pub last_recbank: c_int,
    pub captureLimit: c_uint,
    pub capture_sample_size: c_uint,
    pub capture_channels: c_uint,
    pub capture_sample_rate: c_uint,
    pub playback_substream: *mut snd_pcm_substream,
    pub playDMAPos: c_uint,
    pub banksPlayed: c_uint,
    pub capture_substream: *mut snd_pcm_substream,
    pub captureDMAPos: c_uint,
    pub base: c_ulong,
}

unsafe extern "C" {
    static HZ: c_long;
    static current: *mut c_void;

    static JQS_wStart: usize;
    static JQS_wSize: usize;
    static JQS_wHead: usize;
    static JQS_wTail: usize;
    static HP_ISR: c_uint;
    static HPISR_TXDE: c_uint;
    static HP_CVR: c_uint;
    static HPCVR_HC: c_uint;
    static HP_TXH: c_uint;
    static HP_TXM: c_uint;
    static HP_TXL: c_uint;
    static HP_RXL: c_uint;
    static HP_ICR: c_uint;
    static HPICR_TREQ: c_uint;
    static HP_IRQM: c_uint;
    static HPICR_RREQ: c_uint;
    static HPIRQ_NONE: u8;
    static FMODE_WRITE: c_int;
    static FMODE_READ: c_int;
    static F_WRITING: c_int;
    static F_WRITEFLUSH: c_int;
    static F_READING: c_int;
    static F_AUDIO_READ_INUSE: c_int;
    static F_AUDIO_WRITE_INUSE: c_int;
    static HDEX_RECORD_STOP: u8;
    static HDEX_PLAY_STOP: u8;
    static HDEX_PLAY_START: u8;
    static HDEX_RECORD_START: u8;
    static DAQDS__size: c_uint;
    static DAQDS_wStart: usize;
    static DAQDS_wSize: usize;
    static DAQDS_wFormat: usize;
    static DAQDS_wSampleSize: usize;
    static DAQDS_wChannels: usize;
    static DAQDS_wSampleRate: usize;
    static DAQDS_wIntMsg: usize;
    static DAQDS_wFlags: usize;
    static DARQ_DATA_BUFF: usize;
    static DAPQ_DATA_BUFF: usize;
    static HIMT_PLAY_DONE: c_uint;
    static HIMT_RECORD_DONE: c_uint;
    static msndClassic: c_int;
    static SNDRV_PCM_INFO_MMAP_IOMEM: c_uint;
    static SNDRV_PCM_INFO_INTERLEAVED: c_uint;
    static SNDRV_PCM_INFO_MMAP_VALID: c_uint;
    static SNDRV_PCM_INFO_BATCH: c_uint;
    static SNDRV_PCM_FMTBIT_U8: c_ulong;
    static SNDRV_PCM_FMTBIT_S16_LE: c_ulong;
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;

    fn writew(value: u16, addr: *mut c_void);
    fn readw(addr: *mut c_void) -> u16;
    fn inb(port: c_uint) -> u8;
    fn outb(value: u8, port: c_uint);
    fn enable_irq(irq: c_uint);
    fn disable_irq(irq: c_uint);
    fn signal_pending(task: *mut c_void) -> c_int;
    fn schedule_timeout_interruptible(timeout: c_long) -> c_long;
    fn udelay(usecs: c_ulong);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn set_bit(nr: c_int, addr: *mut c_ulong);
    fn clear_bit(nr: c_int, addr: *mut c_ulong);
    fn test_bit(nr: c_int, addr: *const c_ulong) -> c_int;
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut snd_msnd;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: c_uint) -> snd_pcm_uframes_t;
    fn snd_pcm_lib_mmap_iomem() -> c_int;
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops);
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

const EIO: c_int = 5;
const EINVAL: c_int = 22;

unsafe fn PCTODSP_BASED(x: c_uint) -> u16 {
    x as u16
}

unsafe fn PCTODSP_OFFSET(x: c_uint) -> u16 {
    x as u16
}

unsafe fn ptr_add(base: *mut c_void, offset: usize) -> *mut c_void {
    (base as *mut u8).add(offset) as *mut c_void
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_msnd_init_queue(base: *mut c_void, start: c_int, size: c_int) {
    unsafe {
        writew(PCTODSP_BASED(start as c_uint), ptr_add(base, JQS_wStart));
        writew(PCTODSP_OFFSET(size as c_uint).wrapping_sub(1), ptr_add(base, JQS_wSize));
        writew(0, ptr_add(base, JQS_wHead));
        writew(0, ptr_add(base, JQS_wTail));
    }
}

unsafe fn snd_msnd_wait_TXDE(dev: *mut snd_msnd) -> c_int {
    unsafe {
        let io = (*dev).io;
        let mut timeout: c_int = 1000;

        while {
            let old = timeout;
            timeout -= 1;
            old > 0
        } {
            if (inb(io + HP_ISR) as c_uint & HPISR_TXDE) != 0 {
                return 0;
            }
        }

        -EIO
    }
}

unsafe fn snd_msnd_wait_HC0(dev: *mut snd_msnd) -> c_int {
    unsafe {
        let io = (*dev).io;
        let mut timeout: c_int = 1000;

        while {
            let old = timeout;
            timeout -= 1;
            old > 0
        } {
            if (inb(io + HP_CVR) as c_uint & HPCVR_HC) == 0 {
                return 0;
            }
        }

        -EIO
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_msnd_send_dsp_cmd(dev: *mut snd_msnd, cmd: u8) -> c_int {
    unsafe {
        /* guard(spinlock_irqsave)(&dev->lock); */
        if snd_msnd_wait_HC0(dev) == 0 {
            outb(cmd, (*dev).io + HP_CVR);
            return 0;
        }

        dev_dbg((*(*dev).card).dev, c"msnd: Send DSP command timeout\n".as_ptr());

        -EIO
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_msnd_send_word(dev: *mut snd_msnd, high: u8, mid: u8, low: u8) -> c_int {
    unsafe {
        let io = (*dev).io;

        if snd_msnd_wait_TXDE(dev) == 0 {
            outb(high, io + HP_TXH);
            outb(mid, io + HP_TXM);
            outb(low, io + HP_TXL);
            return 0;
        }

        dev_dbg((*(*dev).card).dev, c"msnd: Send host word timeout\n".as_ptr());

        -EIO
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_msnd_upload_host(dev: *mut snd_msnd, bin: *const u8, len: c_int) -> c_int {
    unsafe {
        let mut i: c_int;

        if len % 3 != 0 {
            dev_err((*(*dev).card).dev, c"msnd: Upload host data not multiple of 3!\n".as_ptr());
            return -EINVAL;
        }

        i = 0;
        while i < len {
            if snd_msnd_send_word(dev, *bin.add(i as usize), *bin.add((i + 1) as usize), *bin.add((i + 2) as usize)) != 0 {
                return -EIO;
            }
            i += 3;
        }

        inb((*dev).io + HP_RXL);
        inb((*dev).io + HP_CVR);

        0
    }
}

unsafe fn __snd_msnd_enable_irq(dev: *mut snd_msnd) -> c_int {
    unsafe {
        dev_dbg((*(*dev).card).dev, c"msnd: Enabling IRQ\n".as_ptr());

        /* guard(spinlock_irqsave)(&dev->lock); */
        if snd_msnd_wait_TXDE(dev) == 0 {
            outb((inb((*dev).io + HP_ICR) as c_uint | HPICR_TREQ) as u8, (*dev).io + HP_ICR);
            if (*dev).type_ == msndClassic {
                outb((*dev).irqid, (*dev).io + HP_IRQM);
            }

            outb((inb((*dev).io + HP_ICR) as c_uint & !HPICR_TREQ) as u8, (*dev).io + HP_ICR);
            outb((inb((*dev).io + HP_ICR) as c_uint | HPICR_RREQ) as u8, (*dev).io + HP_ICR);
            enable_irq((*dev).irq);
            snd_msnd_init_queue((*dev).DSPQ, (*dev).dspq_data_buff, (*dev).dspq_buff_size);
            return 0;
        }

        dev_dbg((*(*dev).card).dev, c"msnd: Enable IRQ failed\n".as_ptr());

        -EIO
    }
}

unsafe fn __snd_msnd_disable_irq(dev: *mut snd_msnd) -> c_int {
    unsafe {
        dev_dbg((*(*dev).card).dev, c"msnd: Disabling IRQ\n".as_ptr());

        /* guard(spinlock_irqsave)(&dev->lock); */
        if snd_msnd_wait_TXDE(dev) == 0 {
            outb((inb((*dev).io + HP_ICR) as c_uint & !HPICR_RREQ) as u8, (*dev).io + HP_ICR);
            if (*dev).type_ == msndClassic {
                outb(HPIRQ_NONE, (*dev).io + HP_IRQM);
            }
            disable_irq((*dev).irq);
            return 0;
        }

        dev_dbg((*(*dev).card).dev, c"msnd: Disable IRQ failed\n".as_ptr());

        -EIO
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_msnd_enable_irq(dev: *mut snd_msnd) -> c_int {
    unsafe {
        let old = (*dev).irq_ref;
        (*dev).irq_ref += 1;
        if old != 0 {
            return 0;
        }

        __snd_msnd_enable_irq(dev)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_msnd_disable_irq(dev: *mut snd_msnd) -> c_int {
    unsafe {
        (*dev).irq_ref -= 1;
        if (*dev).irq_ref > 0 {
            return 0;
        }

        if (*dev).irq_ref < 0 {
            dev_dbg((*(*dev).card).dev, c"msnd: IRQ ref count is %d\n".as_ptr(), (*dev).irq_ref);
        }

        __snd_msnd_disable_irq(dev)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_msnd_force_irq(dev: *mut snd_msnd, enable: bool) -> c_int {
    unsafe {
        if (*dev).irq_ref == 0 {
            return 0;
        }

        if enable {
            __snd_msnd_enable_irq(dev)
        } else {
            __snd_msnd_disable_irq(dev)
        }
    }
}

unsafe fn get_play_delay_jiffies(chip: *mut snd_msnd, size: c_long) -> c_long {
    unsafe {
        let tmp = (size * HZ * (*chip).play_sample_size) / 8;
        tmp / ((*chip).play_sample_rate * (*chip).play_channels)
    }
}

unsafe fn snd_msnd_dsp_write_flush(chip: *mut snd_msnd) {
    unsafe {
        if ((*chip).mode & FMODE_WRITE) == 0 || test_bit(F_WRITING, &(*chip).flags) == 0 {
            return;
        }
        set_bit(F_WRITEFLUSH, &mut (*chip).flags);
        /*	interruptible_sleep_on_timeout(
         *		&chip->writeflush,
         *		get_play_delay_jiffies(&chip, chip->DAPF.len));*/
        clear_bit(F_WRITEFLUSH, &mut (*chip).flags);
        if signal_pending(current) == 0 {
            schedule_timeout_interruptible(get_play_delay_jiffies(chip, (*chip).play_period_bytes as c_long));
        }
        clear_bit(F_WRITING, &mut (*chip).flags);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_msnd_dsp_halt(chip: *mut snd_msnd, file: *mut file) {
    unsafe {
        if ((if !file.is_null() { (*file).f_mode } else { (*chip).mode }) & FMODE_READ) != 0 {
            clear_bit(F_READING, &mut (*chip).flags);
            snd_msnd_send_dsp_cmd(chip, HDEX_RECORD_STOP);
            snd_msnd_disable_irq(chip);
            if !file.is_null() {
                dev_dbg((*(*chip).card).dev, c"msnd: Stopping read for %p\n".as_ptr(), file);
                (*chip).mode &= !FMODE_READ;
            }
            clear_bit(F_AUDIO_READ_INUSE, &mut (*chip).flags);
        }
        if ((if !file.is_null() { (*file).f_mode } else { (*chip).mode }) & FMODE_WRITE) != 0 {
            if test_bit(F_WRITING, &(*chip).flags) != 0 {
                snd_msnd_dsp_write_flush(chip);
                snd_msnd_send_dsp_cmd(chip, HDEX_PLAY_STOP);
            }
            snd_msnd_disable_irq(chip);
            if !file.is_null() {
                dev_dbg((*(*chip).card).dev, c"msnd: Stopping write for %p\n".as_ptr(), file);
                (*chip).mode &= !FMODE_WRITE;
            }
            clear_bit(F_AUDIO_WRITE_INUSE, &mut (*chip).flags);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_msnd_DARQ(chip: *mut snd_msnd, bank: c_int) -> c_int {
    unsafe {
        let mut timeout: c_int = 3;
        let mut wTmp: u16;

        /* Increment the tail and check for queue wrap */
        wTmp = readw(ptr_add((*chip).DARQ, JQS_wTail)).wrapping_add(PCTODSP_OFFSET(DAQDS__size));
        if wTmp > readw(ptr_add((*chip).DARQ, JQS_wSize)) {
            wTmp = 0;
        }
        while wTmp == readw(ptr_add((*chip).DARQ, JQS_wHead)) && {
            let old = timeout;
            timeout -= 1;
            old != 0
        } {
            udelay(1);
        }

        if (*chip).capturePeriods == 2 {
            let pDAQ = ptr_add((*chip).mappedbase, DARQ_DATA_BUFF + bank as usize * DAQDS__size as usize + DAQDS_wStart);
            let mut offset: u16 = (0x3000u32 + (*chip).capturePeriodBytes) as u16;

            if readw(pDAQ) != PCTODSP_BASED(0x3000) {
                offset = 0x3000;
            }
            writew(PCTODSP_BASED(offset as c_uint), pDAQ);
        }

        writew(wTmp, ptr_add((*chip).DARQ, JQS_wTail));

        /*
         * Disabled C #if 0 block preserved from source:
         * read captured data from the digital audio queue, switch host port
         * block selection, and copy via msnd_fifo_write().
         */

        1
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_msnd_DAPQ(chip: *mut snd_msnd, start_: c_int) -> c_int {
    static mut play_banks_submitted: c_int = 0;

    unsafe {
        let mut DAPQ_tail: u16;
        let protect = start_;
        let mut start = start_;
        let mut nbanks: c_int = 0;
        let mut DAQD: *mut c_void;
        /* unsigned long flags;
         * spin_lock_irqsave(&chip->lock, flags); not necessary */

        DAPQ_tail = readw(ptr_add((*chip).DAPQ, JQS_wTail));
        while DAPQ_tail != readw(ptr_add((*chip).DAPQ, JQS_wHead)) || start != 0 {
            let mut bank_num: c_int = (DAPQ_tail / PCTODSP_OFFSET(DAQDS__size)) as c_int;

            if start != 0 {
                start = 0;
                play_banks_submitted = 0;
            }

            /* Get our digital audio queue struct */
            DAQD = ptr_add((*chip).mappedbase, bank_num as usize * DAQDS__size as usize + DAPQ_DATA_BUFF);

            /* Write size of this bank */
            writew((*chip).play_period_bytes as u16, ptr_add(DAQD, DAQDS_wSize));
            if play_banks_submitted < 3 {
                play_banks_submitted += 1;
            } else if (*chip).playPeriods == 2 {
                let mut offset: u16 = (*chip).play_period_bytes as u16;

                if readw(ptr_add(DAQD, DAQDS_wStart)) != PCTODSP_BASED(0x0) {
                    offset = 0;
                }

                writew(PCTODSP_BASED(offset as c_uint), ptr_add(DAQD, DAQDS_wStart));
            }
            nbanks += 1;

            /* Then advance the tail */
            bank_num += 1;
            DAPQ_tail = ((bank_num % 3) as u16).wrapping_mul(PCTODSP_OFFSET(DAQDS__size));
            writew(DAPQ_tail, ptr_add((*chip).DAPQ, JQS_wTail));
            /* Tell the DSP to play the bank */
            snd_msnd_send_dsp_cmd(chip, HDEX_PLAY_START);
            if protect != 0 {
                if 2 == bank_num {
                    break;
                }
            }
        }
        /* spin_unlock_irqrestore(&chip->lock, flags); not necessary */
        nbanks
    }
}

unsafe fn snd_msnd_play_reset_queue(chip: *mut snd_msnd, pcm_periods: c_uint, pcm_count: c_uint) {
    unsafe {
        let mut n: c_int;
        let mut pDAQ = ptr_add((*chip).mappedbase, DAPQ_DATA_BUFF);

        (*chip).last_playbank = -1;
        (*chip).playLimit = pcm_count * (pcm_periods - 1);
        (*chip).playPeriods = pcm_periods;
        writew(PCTODSP_OFFSET(0 * DAQDS__size), ptr_add((*chip).DAPQ, JQS_wHead));
        writew(PCTODSP_OFFSET(0 * DAQDS__size), ptr_add((*chip).DAPQ, JQS_wTail));

        (*chip).play_period_bytes = pcm_count;

        n = 0;
        while (n as c_uint) < pcm_periods {
            writew(PCTODSP_BASED(pcm_count * n as c_uint), ptr_add(pDAQ, DAQDS_wStart));
            writew(0, ptr_add(pDAQ, DAQDS_wSize));
            writew(1, ptr_add(pDAQ, DAQDS_wFormat));
            writew((*chip).play_sample_size as u16, ptr_add(pDAQ, DAQDS_wSampleSize));
            writew((*chip).play_channels as u16, ptr_add(pDAQ, DAQDS_wChannels));
            writew((*chip).play_sample_rate as u16, ptr_add(pDAQ, DAQDS_wSampleRate));
            writew((HIMT_PLAY_DONE * 0x100 + n as c_uint) as u16, ptr_add(pDAQ, DAQDS_wIntMsg));
            writew(n as u16, ptr_add(pDAQ, DAQDS_wFlags));
            n += 1;
            pDAQ = ptr_add(pDAQ, DAQDS__size as usize);
        }
    }
}

unsafe fn snd_msnd_capture_reset_queue(chip: *mut snd_msnd, pcm_periods: c_uint, pcm_count: c_uint) {
    unsafe {
        let mut n: c_int;
        let mut pDAQ: *mut c_void;

        /* snd_msnd_init_queue(chip->DARQ, DARQ_DATA_BUFF, DARQ_BUFF_SIZE); */

        (*chip).last_recbank = 2;
        (*chip).captureLimit = pcm_count * (pcm_periods - 1);
        (*chip).capturePeriods = pcm_periods;
        writew(PCTODSP_OFFSET(0 * DAQDS__size), ptr_add((*chip).DARQ, JQS_wHead));
        writew(PCTODSP_OFFSET((*chip).last_recbank as c_uint * DAQDS__size), ptr_add((*chip).DARQ, JQS_wTail));

        /*
         * Disabled C #if 0 critical section preserved from source:
         * select bank 1, memset_io mapped capture buffers, then restore bank 0.
         */

        (*chip).capturePeriodBytes = pcm_count;
        dev_dbg((*(*chip).card).dev, c"%s() %i\n".as_ptr(), c"snd_msnd_capture_reset_queue".as_ptr(), pcm_count);

        pDAQ = ptr_add((*chip).mappedbase, DARQ_DATA_BUFF);

        n = 0;
        while (n as c_uint) < pcm_periods {
            let tmp: u32 = pcm_count * n as c_uint;

            writew(PCTODSP_BASED(tmp + 0x3000), ptr_add(pDAQ, DAQDS_wStart));
            writew(pcm_count as u16, ptr_add(pDAQ, DAQDS_wSize));
            writew(1, ptr_add(pDAQ, DAQDS_wFormat));
            writew((*chip).capture_sample_size as u16, ptr_add(pDAQ, DAQDS_wSampleSize));
            writew((*chip).capture_channels as u16, ptr_add(pDAQ, DAQDS_wChannels));
            writew((*chip).capture_sample_rate as u16, ptr_add(pDAQ, DAQDS_wSampleRate));
            writew((HIMT_RECORD_DONE * 0x100 + n as c_uint) as u16, ptr_add(pDAQ, DAQDS_wIntMsg));
            writew(n as u16, ptr_add(pDAQ, DAQDS_wFlags));
            n += 1;
            pDAQ = ptr_add(pDAQ, DAQDS__size as usize);
        }
    }
}

static mut snd_msnd_playback: snd_pcm_hardware = snd_pcm_hardware {
    info: 0,
    formats: 0,
    rates: 0,
    rate_min: 8000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 0x3000,
    period_bytes_min: 0x40,
    period_bytes_max: 0x1800,
    periods_min: 2,
    periods_max: 3,
    fifo_size: 0,
};

static mut snd_msnd_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: 0,
    formats: 0,
    rates: 0,
    rate_min: 8000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 0x3000,
    period_bytes_min: 0x40,
    period_bytes_max: 0x1800,
    periods_min: 2,
    periods_max: 3,
    fifo_size: 0,
};

unsafe fn init_hardware_constants() {
    unsafe {
        snd_msnd_playback.info = SNDRV_PCM_INFO_MMAP_IOMEM | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_BATCH;
        snd_msnd_playback.formats = SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE;
        snd_msnd_playback.rates = SNDRV_PCM_RATE_8000_48000;
        snd_msnd_capture.info = SNDRV_PCM_INFO_MMAP_IOMEM | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_BATCH;
        snd_msnd_capture.formats = SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE;
        snd_msnd_capture.rates = SNDRV_PCM_RATE_8000_48000;
    }
}

unsafe extern "C" fn snd_msnd_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let runtime = (*substream).runtime;
        let chip = snd_pcm_substream_chip(substream);

        set_bit(F_AUDIO_WRITE_INUSE, &mut (*chip).flags);
        clear_bit(F_WRITING, &mut (*chip).flags);
        snd_msnd_enable_irq(chip);

        (*runtime).dma_area = (*chip).mappedbase;
        (*runtime).dma_addr = (*chip).base;
        (*runtime).dma_bytes = 0x3000;

        (*chip).playback_substream = substream;
        (*runtime).hw = snd_msnd_playback;
        0
    }
}

unsafe extern "C" fn snd_msnd_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);

        snd_msnd_disable_irq(chip);
        clear_bit(F_AUDIO_WRITE_INUSE, &mut (*chip).flags);
        0
    }
}

unsafe extern "C" fn snd_msnd_playback_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    unsafe {
        let mut i: c_int;
        let chip = snd_pcm_substream_chip(substream);
        let mut pDAQ = ptr_add((*chip).mappedbase, DAPQ_DATA_BUFF);

        (*chip).play_sample_size = snd_pcm_format_width(params_format(params)) as c_long;
        (*chip).play_channels = params_channels(params) as c_long;
        (*chip).play_sample_rate = params_rate(params) as c_long;

        i = 0;
        while i < 3 {
            writew((*chip).play_sample_size as u16, ptr_add(pDAQ, DAQDS_wSampleSize));
            writew((*chip).play_channels as u16, ptr_add(pDAQ, DAQDS_wChannels));
            writew((*chip).play_sample_rate as u16, ptr_add(pDAQ, DAQDS_wSampleRate));
            i += 1;
            pDAQ = ptr_add(pDAQ, DAQDS__size as usize);
        }
        /* dont do this here:
         * snd_msnd_calibrate_adc(chip->play_sample_rate);
         */

        0
    }
}

unsafe extern "C" fn snd_msnd_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);
        let pcm_size = snd_pcm_lib_buffer_bytes(substream);
        let pcm_count = snd_pcm_lib_period_bytes(substream);
        let pcm_periods = pcm_size / pcm_count;

        snd_msnd_play_reset_queue(chip, pcm_periods, pcm_count);
        (*chip).playDMAPos = 0;
        0
    }
}

unsafe extern "C" fn snd_msnd_playback_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);

        if cmd == SNDRV_PCM_TRIGGER_START {
            dev_dbg((*(*chip).card).dev, c"%s(START)\n".as_ptr(), c"snd_msnd_playback_trigger".as_ptr());
            (*chip).banksPlayed = 0;
            set_bit(F_WRITING, &mut (*chip).flags);
            snd_msnd_DAPQ(chip, 1);
        } else if cmd == SNDRV_PCM_TRIGGER_STOP || cmd == SNDRV_PCM_TRIGGER_SUSPEND {
            dev_dbg((*(*chip).card).dev, c"%s(STOP)\n".as_ptr(), c"snd_msnd_playback_trigger".as_ptr());
            clear_bit(F_WRITING, &mut (*chip).flags);
            snd_msnd_send_dsp_cmd(chip, HDEX_PLAY_STOP);
        } else {
            dev_dbg((*(*chip).card).dev, c"%s(?????)\n".as_ptr(), c"snd_msnd_playback_trigger".as_ptr());
            return -EINVAL;
        }

        dev_dbg((*(*chip).card).dev, c"%s() ENDE\n".as_ptr(), c"snd_msnd_playback_trigger".as_ptr());
        0
    }
}

unsafe extern "C" fn snd_msnd_playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);

        bytes_to_frames((*substream).runtime, (*chip).playDMAPos)
    }
}

static snd_msnd_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_msnd_playback_open),
    close: Some(snd_msnd_playback_close),
    hw_params: Some(snd_msnd_playback_hw_params),
    prepare: Some(snd_msnd_playback_prepare),
    trigger: Some(snd_msnd_playback_trigger),
    pointer: Some(snd_msnd_playback_pointer),
    mmap: Some(snd_pcm_lib_mmap_iomem),
};

unsafe extern "C" fn snd_msnd_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let runtime = (*substream).runtime;
        let chip = snd_pcm_substream_chip(substream);

        set_bit(F_AUDIO_READ_INUSE, &mut (*chip).flags);
        snd_msnd_enable_irq(chip);
        (*runtime).dma_area = ptr_add((*chip).mappedbase, 0x3000);
        (*runtime).dma_addr = (*chip).base + 0x3000;
        (*runtime).dma_bytes = 0x3000;
        memset((*runtime).dma_area, 0, (*runtime).dma_bytes as usize);
        (*chip).capture_substream = substream;
        (*runtime).hw = snd_msnd_capture;
        0
    }
}

unsafe extern "C" fn snd_msnd_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);

        snd_msnd_disable_irq(chip);
        clear_bit(F_AUDIO_READ_INUSE, &mut (*chip).flags);
        0
    }
}

unsafe extern "C" fn snd_msnd_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);
        let pcm_size = snd_pcm_lib_buffer_bytes(substream);
        let pcm_count = snd_pcm_lib_period_bytes(substream);
        let pcm_periods = pcm_size / pcm_count;

        snd_msnd_capture_reset_queue(chip, pcm_periods, pcm_count);
        (*chip).captureDMAPos = 0;
        0
    }
}

unsafe extern "C" fn snd_msnd_capture_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);

        if cmd == SNDRV_PCM_TRIGGER_START {
            (*chip).last_recbank = -1;
            set_bit(F_READING, &mut (*chip).flags);
            if snd_msnd_send_dsp_cmd(chip, HDEX_RECORD_START) == 0 {
                return 0;
            }

            clear_bit(F_READING, &mut (*chip).flags);
        } else if cmd == SNDRV_PCM_TRIGGER_STOP || cmd == SNDRV_PCM_TRIGGER_SUSPEND {
            clear_bit(F_READING, &mut (*chip).flags);
            snd_msnd_send_dsp_cmd(chip, HDEX_RECORD_STOP);
            return 0;
        }
        -EINVAL
    }
}

unsafe extern "C" fn snd_msnd_capture_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    unsafe {
        let runtime = (*substream).runtime;
        let chip = snd_pcm_substream_chip(substream);

        bytes_to_frames(runtime, (*chip).captureDMAPos)
    }
}

unsafe extern "C" fn snd_msnd_capture_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    unsafe {
        let mut i: c_int;
        let chip = snd_pcm_substream_chip(substream);
        let mut pDAQ = ptr_add((*chip).mappedbase, DARQ_DATA_BUFF);

        (*chip).capture_sample_size = snd_pcm_format_width(params_format(params)) as c_uint;
        (*chip).capture_channels = params_channels(params);
        (*chip).capture_sample_rate = params_rate(params);

        i = 0;
        while i < 3 {
            writew((*chip).capture_sample_size as u16, ptr_add(pDAQ, DAQDS_wSampleSize));
            writew((*chip).capture_channels as u16, ptr_add(pDAQ, DAQDS_wChannels));
            writew((*chip).capture_sample_rate as u16, ptr_add(pDAQ, DAQDS_wSampleRate));
            i += 1;
            pDAQ = ptr_add(pDAQ, DAQDS__size as usize);
        }
        0
    }
}

static snd_msnd_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_msnd_capture_open),
    close: Some(snd_msnd_capture_close),
    hw_params: Some(snd_msnd_capture_hw_params),
    prepare: Some(snd_msnd_capture_prepare),
    trigger: Some(snd_msnd_capture_trigger),
    pointer: Some(snd_msnd_capture_pointer),
    mmap: Some(snd_pcm_lib_mmap_iomem),
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_msnd_pcm(card: *mut snd_card, device: c_int) -> c_int {
    unsafe {
        let chip = (*card).private_data as *mut snd_msnd;
        let mut pcm: *mut snd_pcm = core::ptr::null_mut();
        let mut err: c_int;

        init_hardware_constants();
        err = snd_pcm_new(card, c"MSNDPINNACLE".as_ptr(), device, 1, 1, &mut pcm);
        if err < 0 {
            return err;
        }

        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_msnd_playback_ops);
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_msnd_capture_ops);

        (*pcm).private_data = chip as *mut c_void;
        strscpy((*pcm).name.as_mut_ptr(), c"Hurricane".as_ptr());

        0
    }
}

/* MODULE_DESCRIPTION("Common routines for Turtle Beach Multisound drivers"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
