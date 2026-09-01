// SPDX-License-Identifier: GPL-2.0-or-later
/*********************************************************************
 *
 * Linux multisound pinnacle/fiji driver for ALSA.
 *
 * 2002/06/30 Karsten Wiese:
 *	for now this is only used to build a pinnacle / fiji driver.
 *	the OSS parent of this code is designed to also support
 *	the multisound classic via the file msnd_classic.c.
 *	to make it easier for some brave heart to implemt classic
 *	support in alsa, i left all the MSND_CLASSIC tokens in this file.
 *	but for now this untested & undone.
 *
 * ripped from linux kernel 2.4.18 by Karsten Wiese.
 *
 * the following is a copy of the 2.4.18 OSS FREE file-heading comment:
 *
 * Turtle Beach MultiSound Sound Card Driver for Linux
 * msnd_pinnacle.c / msnd_classic.c
 *
 * -- If MSND_CLASSIC is defined:
 *
 *     -> driver for Turtle Beach Classic/Monterey/Tahiti
 *
 * -- Else
 *
 *     -> driver for Turtle Beach Pinnacle/Fiji
 *
 * 12-3-2000  Modified IO port validation  Steve Sycamore
 *
 * Copyright (C) 1998 Andrew Veliath
 *
 ********************************************************************/

// C includes translated as dependency intent:
// linux/kernel.h, module.h, interrupt.h, types.h, delay.h, ioport.h,
// firmware.h, isa.h, isapnp.h, irq.h, io.h; sound/core.h, initval.h,
// asound.h, pcm.h, mpu401.h; msnd.h; and either msnd_classic.h or
// msnd_pinnacle.h depending on MSND_CLASSIC.
// If MSND_CLASSIC is defined and __alpha__ is not defined, SLOWIO is defined.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

type u8 = u8;
type u16 = u16;
type irqreturn_t = c_int;
type bool_t = bool;
type pm_message_t = c_int;

const IRQ_HANDLED: irqreturn_t = 1;
const NULL: *mut c_void = ptr::null_mut();

const LOGNAME: &[u8] = b"snd_msnd_pinnacle\0";
const DEV_NAME: &[u8] = b"msnd-pinnacle\0";

#[repr(C)]
struct snd_msnd {
    card: *mut snd_card,
    play_sample_size: c_int,
    play_sample_rate: u16,
    play_channels: u16,
    capture_sample_size: c_int,
    capture_sample_rate: u16,
    capture_channels: u16,
    banksPlayed: c_int,
    last_playbank: u8,
    flags: c_ulong,
    playDMAPos: c_ulong,
    play_period_bytes: c_ulong,
    playLimit: c_ulong,
    playback_substream: *mut c_void,
    last_recbank: u8,
    captureDMAPos: c_ulong,
    capturePeriodBytes: c_ulong,
    captureLimit: c_ulong,
    capture_substream: *mut c_void,
    io: c_long,
    irq: c_int,
    base: c_long,
    mappedbase: *mut c_void,
    DSPQ: *mut c_void,
    SMA: *mut c_void,
    DAPQ: *mut c_void,
    DARQ: *mut c_void,
    MODQ: *mut c_void,
    MIDQ: *mut c_void,
    lock: c_ulong,
    calibrate_signal: c_int,
    recsrc: c_ulong,
    dspq_data_buff: c_int,
    dspq_buff_size: c_int,
    nresets: c_int,
    rmidi: *mut snd_rawmidi,
    pm_recsrc: u8,
    pm_mpu_input: bool_t,
    irqid: c_int,
    memid: c_int,
    type_: c_int,
}

#[repr(C)]
struct snd_card {
    dev: *mut device,
    private_data: *mut snd_msnd,
    shortname: [c_char; 80],
    longname: [c_char; 80],
    sync_irq: c_int,
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_mpu401 {
    private_data: *mut c_void,
    open_input: Option<unsafe extern "C" fn(*mut snd_mpu401) -> c_int>,
    close_input: Option<unsafe extern "C" fn(*mut snd_mpu401)>,
    mode: c_ulong,
}

#[repr(C)]
struct snd_rawmidi {
    private_data: *mut snd_mpu401,
}

#[repr(C)]
struct firmware {
    data: *const u8,
    size: usize,
}

#[repr(C)]
struct isa_driver_driver {
    name: *const c_char,
}

#[repr(C)]
struct isa_driver {
    match_: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    probe: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    suspend: Option<unsafe extern "C" fn(*mut device, c_uint, pm_message_t) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    driver: isa_driver_driver,
}

#[repr(C)]
struct pnp_card_link {
    card: *mut pnp_card,
}

#[repr(C)]
struct pnp_card {
    dev: device,
}

#[repr(C)]
struct pnp_dev {
    _private: [u8; 0],
}

#[repr(C)]
struct pnp_card_devs_id {
    id: [c_char; 8],
}

#[repr(C)]
struct pnp_card_device_id {
    id: [c_char; 8],
    devs: [pnp_card_devs_id; 2],
}

#[repr(C)]
struct pnp_card_driver {
    flags: c_int,
    name: *const c_char,
    id_table: *const pnp_card_device_id,
    probe: Option<unsafe extern "C" fn(*mut pnp_card_link, *const pnp_card_device_id) -> c_int>,
    suspend: Option<unsafe extern "C" fn(*mut pnp_card_link, pm_message_t) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut pnp_card_link) -> c_int>,
}

unsafe extern "C" {
    static mut jiffies: c_ulong;
    static THIS_MODULE: *mut c_void;

    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn test_bit(nr: c_int, addr: *const c_ulong) -> c_int;
    fn set_bit(nr: c_int, addr: *mut c_ulong);
    fn clear_bit(nr: c_int, addr: *mut c_ulong);
    fn snd_msnd_DAPQ(chip: *mut snd_msnd, start: c_int);
    fn snd_msnd_DARQ(chip: *mut snd_msnd, bank: u8);
    fn snd_pcm_period_elapsed(substream: *mut c_void);
    fn readw(addr: *const c_void) -> u16;
    fn writew(value: u16, addr: *mut c_void);
    fn writel(value: u32, addr: *mut c_void);
    fn inb(port: c_long) -> u8;
    fn outb(value: u8, port: c_long);
    fn msleep(ms: c_uint);
    fn request_region(start: c_long, n: c_int, name: *const c_char) -> *mut c_void;
    fn release_region(start: c_long, n: c_int);
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn memset_io(addr: *mut c_void, val: c_int, count: usize);
    fn snd_msnd_init_queue(base: *mut c_void, data: c_int, size: c_int);
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> c_int;
    fn memcpy_toio(dst: *mut c_void, src: *const u8, count: usize);
    fn snd_msnd_upload_host(chip: *mut snd_msnd, data: *const u8, size: usize) -> c_int;
    fn snd_msndmix_setup(chip: *mut snd_msnd);
    fn snd_msnd_dsp_halt(chip: *mut snd_msnd, arg: *mut c_void);
    fn snd_msndmix_force_recsrc(chip: *mut snd_msnd, recsrc: u8);
    fn snd_msnd_send_dsp_cmd(chip: *mut snd_msnd, cmd: u8) -> c_int;
    fn snd_msnd_send_word(chip: *mut snd_msnd, high: u16, low: u16, cmd: u16) -> c_int;
    fn schedule_timeout_interruptible(timeout: c_ulong) -> c_long;
    fn msecs_to_jiffies(msecs: c_uint) -> c_ulong;
    fn snd_msnd_enable_irq(chip: *mut c_void);
    fn snd_msnd_disable_irq(chip: *mut c_void);
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, data: *mut c_void) -> c_int;
    fn devm_request_region(dev: *mut device, start: c_long, n: c_int, name: *const c_char) -> *mut c_void;
    fn devm_request_mem_region(dev: *mut device, start: c_long, n: c_int, name: *const c_char) -> *mut c_void;
    fn devm_ioremap(dev: *mut device, offset: c_long, size: usize) -> *mut c_void;
    fn snd_msnd_pcm(card: *mut snd_card, device: c_int) -> c_int;
    fn snd_msndmix_new(card: *mut snd_card) -> c_int;
    fn snd_mpu401_uart_new(card: *mut snd_card, device: c_int, hardware: c_int, port: c_long, info_flags: c_int, irq: c_int, rrawmidi: *mut *mut snd_rawmidi) -> c_int;
    fn disable_irq(irq: c_int);
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_devm_card_new(dev: *mut device, idx: c_int, id: *mut c_char, module: *mut c_void, extra_size: usize, card_ret: *mut *mut snd_card) -> c_int;
    fn spin_lock_init(lock: *mut c_ulong);
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_msnd_force_irq(chip: *mut snd_msnd, enable: bool_t) -> c_int;
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn isa_register_driver(driver: *mut isa_driver, ndev: c_uint) -> c_int;
    fn isa_unregister_driver(driver: *mut isa_driver);
    fn pnp_request_card_device(card: *mut pnp_card_link, id: *const c_char, from: *mut pnp_dev) -> *mut pnp_dev;
    fn pnp_is_active(dev: *mut pnp_dev) -> c_int;
    fn pnp_activate_dev(dev: *mut pnp_dev) -> c_int;
    fn pnp_port_start(dev: *mut pnp_dev, bar: c_uint) -> c_long;
    fn pnp_irq(dev: *mut pnp_dev, bar: c_uint) -> c_int;
    fn pnp_mem_start(dev: *mut pnp_dev, bar: c_uint) -> c_long;
    fn pnp_set_card_drvdata(card: *mut pnp_card_link, data: *mut c_void);
    fn pnp_get_card_drvdata(card: *mut pnp_card_link) -> *mut c_void;
    fn pnp_register_card_driver(driver: *mut pnp_card_driver) -> c_int;
    fn pnp_unregister_card_driver(driver: *mut pnp_card_driver);
}

// Constants supplied by translated dependency headers.
unsafe extern "C" {
    static DEFSAMPLESIZE: c_int;
    static DEFSAMPLERATE: u16;
    static DEFCHANNELS: u16;
}

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_PORT: [c_long; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_IRQ: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE_PNP: [bool_t; SNDRV_CARDS] = [true; SNDRV_CARDS];
const SNDRV_AUTO_PORT: c_long = -1;
const SNDRV_AUTO_IRQ: c_int = -1;
const ENODEV: c_int = 19;
const EIO: c_int = 5;
const EBUSY: c_int = 16;
const DSP_NUMIO: c_int = 16;
const BUFFSIZE: c_int = 0x8000;
const HP_RXL: c_long = 0;
const HP_DSPR: c_long = 0;
const HP_INFO: c_long = 0;
const HP_CVR: c_long = 0;
const HP_BLKS: c_long = 0;
const HP_MEMM: c_long = 0;
const HP_WAIT: c_long = 0;
const HP_BITM: c_long = 0;
const HP_PROR: c_long = 0;
const HPDSPRESET_ON: u8 = 0;
const HPDSPRESET_OFF: u8 = 0;
const HP_CVR_DEF: u8 = 0;
const HPBLKSEL_0: u8 = 0;
const HPBLKSEL_1: u8 = 1;
const HPWAITSTATE_0: u8 = 0;
const HPBITMODE_16: u8 = 0;
const HPPRORESET_ON: u8 = 0;
const HPPRORESET_OFF: u8 = 0;
const TIME_PRO_RESET: c_uint = 0;
const TIME_PRO_RESET_DONE: c_uint = 0;
const DSPQ_DATA_BUFF: c_int = 0;
const DSPQ_OFFSET: isize = 0;
const DSPQ_BUFF_SIZE: c_int = 0;
const DAPQ_OFFSET: isize = 0;
const DAPQ_DATA_BUFF: c_int = 0;
const DAPQ_BUFF_SIZE: c_int = 0;
const DARQ_OFFSET: isize = 0;
const DARQ_DATA_BUFF: c_int = 0;
const DARQ_BUFF_SIZE: c_int = 0;
const MODQ_OFFSET: isize = 0;
const MODQ_DATA_BUFF: c_int = 0;
const MODQ_BUFF_SIZE: c_int = 0;
const MIDQ_OFFSET: isize = 0;
const MIDQ_DATA_BUFF: c_int = 0;
const MIDQ_BUFF_SIZE: c_int = 0;
const SMA_STRUCT_START: isize = 0;
const JQS_wHead: isize = 0;
const JQS_wTail: isize = 0;
const JQS_wSize: isize = 0;
const SMA_wCurrMastVolLeft: isize = 0;
const SMA_wCurrMastVolRight: isize = 0;
const SMA_wCurrPlayFormat: isize = 0;
const SMA_wCurrPlaySampleSize: isize = 0;
const SMA_wCurrPlayChannels: isize = 0;
const SMA_wCurrPlaySampleRate: isize = 0;
const SMA_wCalFreqAtoD: isize = 0;
const SMA_dwCurrPlayPitch: isize = 0;
const SMA_dwCurrPlayRate: isize = 0;
const SMA_wCurrInputTagBits: isize = 0;
const SMA_wCurrHostStatusFlags: isize = 0;
const HIMT_PLAY_DONE: u8 = 0;
const HIMT_RECORD_DONE: u8 = 0;
const HIMT_DSP: u8 = 0;
const HIDSP_PLAY_UNDER: u8 = 0;
const HIDSP_INT_PLAY_UNDER: u8 = 0;
const HIDSP_INT_RECORD_OVER: u8 = 0;
const F_WRITING: c_int = 0;
const F_READING: c_int = 0;
const F_RESETTING: c_int = 0;
const F_DISABLE_WRITE_NDELAY: c_int = 0;
const F_HAVEDIGITAL: c_int = 0;
const HDEXAR_CAL_A_TO_D: u16 = 0;
const HDEX_AUX_REQ: u8 = 0;
const HDEX_MIDI_IN_START: u8 = 0;
const HDEX_MIDI_IN_STOP: u8 = 0;
const MPU401_HW_MPU401: c_int = 0;
const MPU401_MODE_INPUT: c_int = 1;
const MPU401_MODE_OUTPUT: c_int = 2;
const MPU401_MODE_BIT_INPUT: c_int = 0;
const IREG_LOGDEVICE: c_int = 0;
const IREG_IO0_BASEHI: c_int = 0;
const IREG_IO0_BASELO: c_int = 0;
const IREG_IO1_BASEHI: c_int = 0;
const IREG_IO1_BASELO: c_int = 0;
const IREG_IRQ_NUMBER: c_int = 0;
const IREG_IRQ_TYPE: c_int = 0;
const IRQTYPE_EDGE: c_int = 0;
const IREG_MEMBASEHI: c_int = 0;
const IREG_MEMBASELO: c_int = 0;
const IREG_MEMCONTROL: c_int = 0;
const MEMTYPE_HIADDR: c_int = 0;
const MEMTYPE_16BIT: c_int = 0;
const IREG_ACTIVATE: c_int = 0;
const LD_ACTIVATE: c_int = 0;
const msndClassic: c_int = 0;
const msndPinnacle: c_int = 1;
const SNDRV_CTL_POWER_D3hot: c_int = 0;
const SNDRV_CTL_POWER_D0: c_int = 0;
const PNP_DRIVER_RES_DO_NOT_CHANGE: c_int = 0;
const INITCODEFILE: *const c_char = b"INITCODEFILE\0".as_ptr() as *const c_char;
const PERMCODEFILE: *const c_char = b"PERMCODEFILE\0".as_ptr() as *const c_char;

#[inline]
fn HIBYTE(x: u16) -> u8 {
    ((x >> 8) & 0xff) as u8
}

#[inline]
fn LOBYTE(x: u16) -> u8 {
    (x & 0xff) as u8
}

#[inline]
fn BIT(x: c_int) -> c_ulong {
    1u64.wrapping_shl(x as u32) as c_ulong
}

#[inline]
unsafe fn ptr_add(base: *mut c_void, offset: isize) -> *mut c_void {
    (base as *mut u8).offset(offset) as *mut c_void
}

unsafe fn set_default_audio_parameters(chip: *mut snd_msnd) {
    (*chip).play_sample_size = snd_pcm_format_width(DEFSAMPLESIZE);
    (*chip).play_sample_rate = DEFSAMPLERATE;
    (*chip).play_channels = DEFCHANNELS;
    (*chip).capture_sample_size = snd_pcm_format_width(DEFSAMPLESIZE);
    (*chip).capture_sample_rate = DEFSAMPLERATE;
    (*chip).capture_channels = DEFCHANNELS;
}

unsafe fn snd_msnd_eval_dsp_msg(chip: *mut snd_msnd, wMessage: u16) {
    match HIBYTE(wMessage) {
        HIMT_PLAY_DONE => {
            if (*chip).banksPlayed < 3 {
                dev_dbg((*(*chip).card).dev, b"%08X: HIMT_PLAY_DONE: %i\n\0".as_ptr() as *const c_char, jiffies as c_uint, LOBYTE(wMessage) as c_int);
            }
            if (*chip).last_playbank == LOBYTE(wMessage) {
                dev_dbg((*(*chip).card).dev, b"chip.last_playbank == LOBYTE(wMessage)\n\0".as_ptr() as *const c_char);
                return;
            }
            (*chip).banksPlayed += 1;
            if test_bit(F_WRITING, &(*chip).flags) != 0 {
                snd_msnd_DAPQ(chip, 0);
            }
            (*chip).last_playbank = LOBYTE(wMessage);
            (*chip).playDMAPos = (*chip).playDMAPos.wrapping_add((*chip).play_period_bytes);
            if (*chip).playDMAPos > (*chip).playLimit {
                (*chip).playDMAPos = 0;
            }
            snd_pcm_period_elapsed((*chip).playback_substream);
        }
        HIMT_RECORD_DONE => {
            if (*chip).last_recbank == LOBYTE(wMessage) {
                return;
            }
            (*chip).last_recbank = LOBYTE(wMessage);
            (*chip).captureDMAPos = (*chip).captureDMAPos.wrapping_add((*chip).capturePeriodBytes);
            if (*chip).captureDMAPos > (*chip).captureLimit {
                (*chip).captureDMAPos = 0;
            }
            if test_bit(F_READING, &(*chip).flags) != 0 {
                snd_msnd_DARQ(chip, (*chip).last_recbank);
            }
            snd_pcm_period_elapsed((*chip).capture_substream);
        }
        HIMT_DSP => {
            match LOBYTE(wMessage) {
                // !MSND_CLASSIC also includes HIDSP_PLAY_UNDER here.
                HIDSP_PLAY_UNDER | HIDSP_INT_PLAY_UNDER => {
                    dev_dbg((*(*chip).card).dev, b"snd_msnd_pinnacle: Play underflow %i\n\0".as_ptr() as *const c_char, (*chip).banksPlayed);
                    if (*chip).banksPlayed > 2 {
                        clear_bit(F_WRITING, &mut (*chip).flags);
                    }
                }
                HIDSP_INT_RECORD_OVER => {
                    dev_dbg((*(*chip).card).dev, b"snd_msnd_pinnacle: Record overflow\n\0".as_ptr() as *const c_char);
                    clear_bit(F_READING, &mut (*chip).flags);
                }
                _ => {
                    dev_dbg((*(*chip).card).dev, b"snd_msnd_pinnacle: DSP message %d 0x%02x\n\0".as_ptr() as *const c_char, LOBYTE(wMessage) as c_int, LOBYTE(wMessage) as c_int);
                }
            }
        }
        _ => {
            dev_dbg((*(*chip).card).dev, b"snd_msnd_pinnacle: HIMT message %d 0x%02x\n\0".as_ptr() as *const c_char, HIBYTE(wMessage) as c_int, HIBYTE(wMessage) as c_int);
        }
    }
}

unsafe extern "C" fn snd_msnd_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chip = dev_id as *mut snd_msnd;
    let pwDSPQData = ptr_add((*chip).mappedbase, DSPQ_DATA_BUFF as isize);
    let mut head: u16;
    let tail: u16;
    let size: u16;

    /* Send ack to DSP */
    /* inb(chip->io + HP_RXL); */

    /* Evaluate queued DSP messages */
    head = readw(ptr_add((*chip).DSPQ, JQS_wHead));
    tail = readw(ptr_add((*chip).DSPQ, JQS_wTail));
    size = readw(ptr_add((*chip).DSPQ, JQS_wSize));
    if !(head > size || tail > size) {
        while head != tail {
            snd_msnd_eval_dsp_msg(chip, readw(ptr_add(pwDSPQData, 2 * head as isize)));
            head = head.wrapping_add(1);
            if head > size {
                head = 0;
            }
            writew(head, ptr_add((*chip).DSPQ, JQS_wHead));
        }
    }
    /* Send ack to DSP */
    inb((*chip).io + HP_RXL);
    IRQ_HANDLED
}

unsafe fn snd_msnd_reset_dsp(chip: *mut snd_msnd, info: *mut u8) -> c_int {
    let io = (*chip).io;
    let mut timeout = 100;

    outb(HPDSPRESET_ON, io + HP_DSPR);
    msleep(1);
    // !MSND_CLASSIC
    if !info.is_null() {
        *info = inb(io + HP_INFO);
    }
    outb(HPDSPRESET_OFF, io + HP_DSPR);
    msleep(1);
    while timeout > 0 {
        timeout -= 1;
        if inb(io + HP_CVR) == HP_CVR_DEF {
            return 0;
        }
        msleep(1);
    }
    dev_err((*(*chip).card).dev, b"snd_msnd_pinnacle: Cannot reset DSP\n\0".as_ptr() as *const c_char);
    -EIO
}

unsafe fn snd_msnd_probe(card: *mut snd_card) -> c_int {
    let chip = (*card).private_data;
    let mut info: u8 = 0;
    let xv: *const c_char;
    let rev: *const c_char;
    let pin = b"TB Pinnacle\0".as_ptr() as *const c_char;
    let fiji = b"TB Fiji\0".as_ptr() as *const c_char;
    let pinfiji = b"TB Pinnacle/Fiji\0".as_ptr() as *const c_char;

    if request_region((*chip).io, DSP_NUMIO, b"probing\0".as_ptr() as *const c_char).is_null() {
        dev_err((*card).dev, b"snd_msnd_pinnacle: I/O port conflict\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }

    if snd_msnd_reset_dsp(chip, &mut info) < 0 {
        release_region((*chip).io, DSP_NUMIO);
        return -ENODEV;
    }

    // MSND_CLASSIC alternative sets Classic/Tahiti/Monterey names and logs classic range.
    match info >> 4 {
        0xf => xv = b"<= 1.15\0".as_ptr() as *const c_char,
        0x1 => xv = b"1.18/1.2\0".as_ptr() as *const c_char,
        0x2 => xv = b"1.3\0".as_ptr() as *const c_char,
        0x3 => xv = b"1.4\0".as_ptr() as *const c_char,
        _ => xv = b"unknown\0".as_ptr() as *const c_char,
    }

    match info & 0x7 {
        0x0 => {
            rev = b"I\0".as_ptr() as *const c_char;
            strscpy((*card).shortname.as_mut_ptr(), pin);
        }
        0x1 => {
            rev = b"F\0".as_ptr() as *const c_char;
            strscpy((*card).shortname.as_mut_ptr(), pin);
        }
        0x2 => {
            rev = b"G\0".as_ptr() as *const c_char;
            strscpy((*card).shortname.as_mut_ptr(), pin);
        }
        0x3 => {
            rev = b"H\0".as_ptr() as *const c_char;
            strscpy((*card).shortname.as_mut_ptr(), pin);
        }
        0x4 => {
            rev = b"E\0".as_ptr() as *const c_char;
            strscpy((*card).shortname.as_mut_ptr(), fiji);
        }
        0x5 => {
            rev = b"C\0".as_ptr() as *const c_char;
            strscpy((*card).shortname.as_mut_ptr(), fiji);
        }
        0x6 => {
            rev = b"D\0".as_ptr() as *const c_char;
            strscpy((*card).shortname.as_mut_ptr(), fiji);
        }
        _ => {
            rev = b"A-B (Fiji) or A-E (Pinnacle)\0".as_ptr() as *const c_char;
            strscpy((*card).shortname.as_mut_ptr(), pinfiji);
        }
    }
    strscpy((*card).longname.as_mut_ptr(), b"Turtle Beach Multisound Pinnacle\0".as_ptr() as *const c_char);
    dev_info((*card).dev, b"snd_msnd_pinnacle: %s revision %s, Xilinx version %s, I/O 0x%lx-0x%lx, IRQ %d, memory mapped to 0x%lX-0x%lX\n\0".as_ptr() as *const c_char, (*card).shortname.as_ptr(), rev, xv, (*chip).io, (*chip).io + DSP_NUMIO as c_long - 1, (*chip).irq, (*chip).base, (*chip).base + 0x7fff);

    release_region((*chip).io, DSP_NUMIO);
    0
}

unsafe fn snd_msnd_init_sma(chip: *mut snd_msnd) -> c_int {
    static mut initted: c_int = 0;
    let mastVolLeft: u16;
    let mastVolRight: u16;

    // MSND_CLASSIC: outb(chip->memid, chip->io + HP_MEMM);
    outb(HPBLKSEL_0, (*chip).io + HP_BLKS);
    /* Motorola 56k shared memory base */
    (*chip).SMA = ptr_add((*chip).mappedbase, SMA_STRUCT_START);

    if initted != 0 {
        mastVolLeft = readw(ptr_add((*chip).SMA, SMA_wCurrMastVolLeft));
        mastVolRight = readw(ptr_add((*chip).SMA, SMA_wCurrMastVolRight));
    } else {
        mastVolLeft = 0;
        mastVolRight = 0;
    }
    memset_io((*chip).mappedbase, 0, 0x8000);

    /* Critical section: bank 1 access */
    outb(HPBLKSEL_1, (*chip).io + HP_BLKS);
    memset_io((*chip).mappedbase, 0, 0x8000);
    outb(HPBLKSEL_0, (*chip).io + HP_BLKS);

    /* Digital audio play queue */
    (*chip).DAPQ = ptr_add((*chip).mappedbase, DAPQ_OFFSET);
    snd_msnd_init_queue((*chip).DAPQ, DAPQ_DATA_BUFF, DAPQ_BUFF_SIZE);

    /* Digital audio record queue */
    (*chip).DARQ = ptr_add((*chip).mappedbase, DARQ_OFFSET);
    snd_msnd_init_queue((*chip).DARQ, DARQ_DATA_BUFF, DARQ_BUFF_SIZE);

    /* MIDI out queue */
    (*chip).MODQ = ptr_add((*chip).mappedbase, MODQ_OFFSET);
    snd_msnd_init_queue((*chip).MODQ, MODQ_DATA_BUFF, MODQ_BUFF_SIZE);

    /* MIDI in queue */
    (*chip).MIDQ = ptr_add((*chip).mappedbase, MIDQ_OFFSET);
    snd_msnd_init_queue((*chip).MIDQ, MIDQ_DATA_BUFF, MIDQ_BUFF_SIZE);

    /* DSP -> host message queue */
    (*chip).DSPQ = ptr_add((*chip).mappedbase, DSPQ_OFFSET);
    snd_msnd_init_queue((*chip).DSPQ, DSPQ_DATA_BUFF, DSPQ_BUFF_SIZE);

    /* Setup some DSP values */
    // !MSND_CLASSIC
    writew(1, ptr_add((*chip).SMA, SMA_wCurrPlayFormat));
    writew((*chip).play_sample_size as u16, ptr_add((*chip).SMA, SMA_wCurrPlaySampleSize));
    writew((*chip).play_channels, ptr_add((*chip).SMA, SMA_wCurrPlayChannels));
    writew((*chip).play_sample_rate, ptr_add((*chip).SMA, SMA_wCurrPlaySampleRate));
    writew((*chip).play_sample_rate, ptr_add((*chip).SMA, SMA_wCalFreqAtoD));
    writew(mastVolLeft, ptr_add((*chip).SMA, SMA_wCurrMastVolLeft));
    writew(mastVolRight, ptr_add((*chip).SMA, SMA_wCurrMastVolRight));
    // !MSND_CLASSIC
    writel(0x00010000, ptr_add((*chip).SMA, SMA_dwCurrPlayPitch));
    writel(0x00000001, ptr_add((*chip).SMA, SMA_dwCurrPlayRate));
    writew(0x303, ptr_add((*chip).SMA, SMA_wCurrInputTagBits));

    initted = 1;
    0
}

unsafe fn upload_dsp_code(card: *mut snd_card) -> c_int {
    let chip = (*card).private_data;
    let mut init_fw: *const firmware = ptr::null();
    let mut perm_fw: *const firmware = ptr::null();
    let mut err: c_int;

    outb(HPBLKSEL_0, (*chip).io + HP_BLKS);

    err = request_firmware(&mut init_fw, INITCODEFILE, (*card).dev);
    if err < 0 {
        dev_err((*card).dev, b"snd_msnd_pinnacle: Error loading INITCODEFILE\0".as_ptr() as *const c_char);
        return err;
    }
    err = request_firmware(&mut perm_fw, PERMCODEFILE, (*card).dev);
    if err < 0 {
        dev_err((*card).dev, b"snd_msnd_pinnacle: Error loading PERMCODEFILE\0".as_ptr() as *const c_char);
        return err;
    }

    memcpy_toio((*chip).mappedbase, (*perm_fw).data, (*perm_fw).size);
    if snd_msnd_upload_host(chip, (*init_fw).data, (*init_fw).size) < 0 {
        dev_warn((*card).dev, b"snd_msnd_pinnacle: Error uploading to DSP\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }
    dev_info((*card).dev, b"snd_msnd_pinnacle: DSP firmware uploaded\n\0".as_ptr() as *const c_char);
    0
}

// MSND_CLASSIC only.
unsafe fn reset_proteus(chip: *mut snd_msnd) {
    outb(HPPRORESET_ON, (*chip).io + HP_PROR);
    msleep(TIME_PRO_RESET);
    outb(HPPRORESET_OFF, (*chip).io + HP_PROR);
    msleep(TIME_PRO_RESET_DONE);
}

unsafe fn snd_msnd_initialize(card: *mut snd_card) -> c_int {
    let chip = (*card).private_data;
    let mut err: c_int;
    let mut timeout: c_int;

    // MSND_CLASSIC:
    // outb(HPWAITSTATE_0, chip->io + HP_WAIT);
    // outb(HPBITMODE_16, chip->io + HP_BITM);
    // reset_proteus(chip);
    err = snd_msnd_init_sma(chip);
    if err < 0 {
        dev_warn((*card).dev, b"snd_msnd_pinnacle: Cannot initialize SMA\n\0".as_ptr() as *const c_char);
        return err;
    }

    err = snd_msnd_reset_dsp(chip, ptr::null_mut());
    if err < 0 {
        return err;
    }

    err = upload_dsp_code(card);
    if err < 0 {
        dev_warn((*card).dev, b"snd_msnd_pinnacle: Cannot upload DSP code\n\0".as_ptr() as *const c_char);
        return err;
    }

    timeout = 200;
    while readw((*chip).mappedbase) != 0 {
        msleep(1);
        if timeout == 0 {
            dev_err((*card).dev, b"snd_msnd_pinnacle: DSP reset timeout\n\0".as_ptr() as *const c_char);
            return -EIO;
        }
        timeout -= 1;
    }

    snd_msndmix_setup(chip);
    0
}

unsafe fn snd_msnd_dsp_full_reset(card: *mut snd_card) -> c_int {
    let chip = (*card).private_data;
    let rv: c_int;

    (*chip).nresets += 1;
    if test_bit(F_RESETTING, &(*chip).flags) != 0 || (*chip).nresets > 10 {
        return 0;
    }

    set_bit(F_RESETTING, &mut (*chip).flags);
    snd_msnd_dsp_halt(chip, ptr::null_mut()); /* Unconditionally halt */

    rv = snd_msnd_initialize(card);
    if rv != 0 {
        dev_warn((*card).dev, b"snd_msnd_pinnacle: DSP reset failed\n\0".as_ptr() as *const c_char);
    }
    snd_msndmix_force_recsrc(chip, 0);
    clear_bit(F_RESETTING, &mut (*chip).flags);
    rv
}

unsafe fn snd_msnd_send_dsp_cmd_chk(chip: *mut snd_msnd, cmd: u8) -> c_int {
    if snd_msnd_send_dsp_cmd(chip, cmd) == 0 {
        return 0;
    }
    snd_msnd_dsp_full_reset((*chip).card);
    snd_msnd_send_dsp_cmd(chip, cmd)
}

unsafe fn snd_msnd_calibrate_adc(chip: *mut snd_msnd, srate: u16) -> c_int {
    dev_dbg((*(*chip).card).dev, b"snd_msnd_calibrate_adc(%i)\n\0".as_ptr() as *const c_char, srate as c_int);
    writew(srate, ptr_add((*chip).SMA, SMA_wCalFreqAtoD));
    if (*chip).calibrate_signal == 0 {
        writew(readw(ptr_add((*chip).SMA, SMA_wCurrHostStatusFlags)) | 0x0001, ptr_add((*chip).SMA, SMA_wCurrHostStatusFlags));
    } else {
        writew(readw(ptr_add((*chip).SMA, SMA_wCurrHostStatusFlags)) & !0x0001, ptr_add((*chip).SMA, SMA_wCurrHostStatusFlags));
    }
    if snd_msnd_send_word(chip, 0, 0, HDEXAR_CAL_A_TO_D) == 0 &&
        snd_msnd_send_dsp_cmd_chk(chip, HDEX_AUX_REQ) == 0 {
        schedule_timeout_interruptible(msecs_to_jiffies(333));
        return 0;
    }
    dev_warn((*(*chip).card).dev, b"snd_msnd_pinnacle: ADC calibration failed\n\0".as_ptr() as *const c_char);
    -EIO
}

/*
 * ALSA callback function, called when attempting to open the MIDI device.
 */
unsafe extern "C" fn snd_msnd_mpu401_open(mpu: *mut snd_mpu401) -> c_int {
    snd_msnd_enable_irq((*mpu).private_data);
    snd_msnd_send_dsp_cmd((*mpu).private_data as *mut snd_msnd, HDEX_MIDI_IN_START);
    0
}

unsafe extern "C" fn snd_msnd_mpu401_close(mpu: *mut snd_mpu401) {
    snd_msnd_send_dsp_cmd((*mpu).private_data as *mut snd_msnd, HDEX_MIDI_IN_STOP);
    snd_msnd_disable_irq((*mpu).private_data);
}

// CONFIG_PM
unsafe fn snd_msnd_pm_recsrc(chip: *mut snd_msnd) -> u8 {
    /* Convert recsrc to the Capture Source selector: 0=Analog, 1=MASS, 2=SPDIF. */
    if ((*chip).recsrc & BIT(4)) != 0 {
        return 1;
    }
    if ((*chip).recsrc & BIT(17)) != 0 && test_bit(F_HAVEDIGITAL, &(*chip).flags) != 0 {
        return 2;
    }
    0
}

static mut mpu_io: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT;
static mut mpu_irq: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IRQ;

unsafe fn snd_msnd_attach(card: *mut snd_card) -> c_int {
    let chip = (*card).private_data;
    let mut err: c_int;

    err = devm_request_irq((*card).dev, (*chip).irq, snd_msnd_interrupt, 0, (*card).shortname.as_ptr(), chip as *mut c_void);
    if err < 0 {
        dev_err((*card).dev, b"snd_msnd_pinnacle: Couldn't grab IRQ %d\n\0".as_ptr() as *const c_char, (*chip).irq);
        return err;
    }
    (*card).sync_irq = (*chip).irq;
    if devm_request_region((*card).dev, (*chip).io, DSP_NUMIO, (*card).shortname.as_ptr()).is_null() {
        return -EBUSY;
    }

    if devm_request_mem_region((*card).dev, (*chip).base, BUFFSIZE, (*card).shortname.as_ptr()).is_null() {
        dev_err((*card).dev, b"snd_msnd_pinnacle: unable to grab memory region 0x%lx-0x%lx\n\0".as_ptr() as *const c_char, (*chip).base, (*chip).base + BUFFSIZE as c_long - 1);
        return -EBUSY;
    }
    (*chip).mappedbase = devm_ioremap((*card).dev, (*chip).base, 0x8000);
    if (*chip).mappedbase.is_null() {
        dev_err((*card).dev, b"snd_msnd_pinnacle: unable to map memory region 0x%lx-0x%lx\n\0".as_ptr() as *const c_char, (*chip).base, (*chip).base + BUFFSIZE as c_long - 1);
        return -EIO;
    }

    err = snd_msnd_dsp_full_reset(card);
    if err < 0 {
        return err;
    }

    err = snd_msnd_pcm(card, 0);
    if err < 0 {
        dev_err((*card).dev, b"snd_msnd_pinnacle: error creating new PCM device\n\0".as_ptr() as *const c_char);
        return err;
    }

    err = snd_msndmix_new(card);
    if err < 0 {
        dev_err((*card).dev, b"snd_msnd_pinnacle: error creating new Mixer device\n\0".as_ptr() as *const c_char);
        return err;
    }

    if mpu_io[0] != SNDRV_AUTO_PORT {
        let mpu: *mut snd_mpu401;

        err = snd_mpu401_uart_new(card, 0, MPU401_HW_MPU401, mpu_io[0], MPU401_MODE_INPUT | MPU401_MODE_OUTPUT, mpu_irq[0], &mut (*chip).rmidi);
        if err < 0 {
            dev_err((*card).dev, b"snd_msnd_pinnacle: error creating new Midi device\n\0".as_ptr() as *const c_char);
            return err;
        }
        mpu = (*(*chip).rmidi).private_data;

        (*mpu).open_input = Some(snd_msnd_mpu401_open);
        (*mpu).close_input = Some(snd_msnd_mpu401_close);
        (*mpu).private_data = chip as *mut c_void;
    }

    disable_irq((*chip).irq);
    snd_msnd_calibrate_adc(chip, (*chip).play_sample_rate);
    snd_msndmix_force_recsrc(chip, 0);

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }

    0
}

/* Pinnacle/Fiji Logical Device Configuration */

unsafe fn snd_msnd_write_cfg(chip: *mut snd_msnd, cfg: c_int, reg: c_int, value: c_int) -> c_int {
    outb(reg as u8, cfg as c_long);
    outb(value as u8, cfg as c_long + 1);
    if value != inb(cfg as c_long + 1) as c_int {
        dev_err((*(*chip).card).dev, b"snd_msnd_pinnacle: %s: I/O error\n\0".as_ptr() as *const c_char, b"snd_msnd_write_cfg\0".as_ptr() as *const c_char);
        return -EIO;
    }
    0
}

unsafe fn snd_msnd_write_cfg_io0(chip: *mut snd_msnd, cfg: c_int, num: c_int, io: u16) -> c_int {
    if snd_msnd_write_cfg(chip, cfg, IREG_LOGDEVICE, num) != 0 {
        return -EIO;
    }
    if snd_msnd_write_cfg(chip, cfg, IREG_IO0_BASEHI, HIBYTE(io) as c_int) != 0 {
        return -EIO;
    }
    if snd_msnd_write_cfg(chip, cfg, IREG_IO0_BASELO, LOBYTE(io) as c_int) != 0 {
        return -EIO;
    }
    0
}

unsafe fn snd_msnd_write_cfg_io1(chip: *mut snd_msnd, cfg: c_int, num: c_int, io: u16) -> c_int {
    if snd_msnd_write_cfg(chip, cfg, IREG_LOGDEVICE, num) != 0 {
        return -EIO;
    }
    if snd_msnd_write_cfg(chip, cfg, IREG_IO1_BASEHI, HIBYTE(io) as c_int) != 0 {
        return -EIO;
    }
    if snd_msnd_write_cfg(chip, cfg, IREG_IO1_BASELO, LOBYTE(io) as c_int) != 0 {
        return -EIO;
    }
    0
}

unsafe fn snd_msnd_write_cfg_irq(chip: *mut snd_msnd, cfg: c_int, num: c_int, irq: u16) -> c_int {
    if snd_msnd_write_cfg(chip, cfg, IREG_LOGDEVICE, num) != 0 {
        return -EIO;
    }
    if snd_msnd_write_cfg(chip, cfg, IREG_IRQ_NUMBER, LOBYTE(irq) as c_int) != 0 {
        return -EIO;
    }
    if snd_msnd_write_cfg(chip, cfg, IREG_IRQ_TYPE, IRQTYPE_EDGE) != 0 {
        return -EIO;
    }
    0
}

unsafe fn snd_msnd_write_cfg_mem(chip: *mut snd_msnd, cfg: c_int, num: c_int, mut mem: c_int) -> c_int {
    let wmem: u16;

    mem >>= 8;
    wmem = (mem & 0xfff) as u16;
    if snd_msnd_write_cfg(chip, cfg, IREG_LOGDEVICE, num) != 0 {
        return -EIO;
    }
    if snd_msnd_write_cfg(chip, cfg, IREG_MEMBASEHI, HIBYTE(wmem) as c_int) != 0 {
        return -EIO;
    }
    if snd_msnd_write_cfg(chip, cfg, IREG_MEMBASELO, LOBYTE(wmem) as c_int) != 0 {
        return -EIO;
    }
    if wmem != 0 && snd_msnd_write_cfg(chip, cfg, IREG_MEMCONTROL, MEMTYPE_HIADDR | MEMTYPE_16BIT) != 0 {
        return -EIO;
    }
    0
}

unsafe fn snd_msnd_activate_logical(chip: *mut snd_msnd, cfg: c_int, num: c_int) -> c_int {
    if snd_msnd_write_cfg(chip, cfg, IREG_LOGDEVICE, num) != 0 {
        return -EIO;
    }
    if snd_msnd_write_cfg(chip, cfg, IREG_ACTIVATE, LD_ACTIVATE) != 0 {
        return -EIO;
    }
    0
}

unsafe fn snd_msnd_write_cfg_logical(chip: *mut snd_msnd, cfg: c_int, num: c_int, io0: u16, io1: u16, irq: u16, mem: c_int) -> c_int {
    if snd_msnd_write_cfg(chip, cfg, IREG_LOGDEVICE, num) != 0 {
        return -EIO;
    }
    if snd_msnd_write_cfg_io0(chip, cfg, num, io0) != 0 {
        return -EIO;
    }
    if snd_msnd_write_cfg_io1(chip, cfg, num, io1) != 0 {
        return -EIO;
    }
    if snd_msnd_write_cfg_irq(chip, cfg, num, irq) != 0 {
        return -EIO;
    }
    if snd_msnd_write_cfg_mem(chip, cfg, num, mem) != 0 {
        return -EIO;
    }
    if snd_msnd_activate_logical(chip, cfg, num) != 0 {
        return -EIO;
    }
    0
}

unsafe fn snd_msnd_pinnacle_cfg_reset(chip: *mut snd_msnd, cfg: c_int) -> c_int {
    let mut i: c_int;

    /* Reset devices if told to */
    dev_info((*(*chip).card).dev, b"snd_msnd_pinnacle: Resetting all devices\n\0".as_ptr() as *const c_char);
    i = 0;
    while i < 4 {
        if snd_msnd_write_cfg_logical(chip, cfg, i, 0, 0, 0, 0) != 0 {
            return -EIO;
        }
        i += 1;
    }

    0
}

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR; /* ID for this card */

// module_param_array(index, int, NULL, 0444);
// MODULE_PARM_DESC(index, "Index value for msnd_pinnacle soundcard.");
// module_param_array(id, charp, NULL, 0444);
// MODULE_PARM_DESC(id, "ID string for msnd_pinnacle soundcard.");

static mut io: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT;
static mut irq: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IRQ;
static mut mem: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT;

static mut cfg: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT;

/* Extra Peripheral Configuration (Default: Disable) */
static mut ide_io0: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT;
static mut ide_io1: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT;
static mut ide_irq: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IRQ;

static mut joystick_io: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT;
/* If we have the digital daugherboard... */
static mut digital: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];

/* Extra Peripheral Configuration */
static mut reset: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];

static mut write_ndelay: [c_int; SNDRV_CARDS] = [1; SNDRV_CARDS];

static mut calibrate_signal: c_int = 0;

// CONFIG_PNP
static mut isapnp: [bool_t; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;

#[inline]
unsafe fn has_isapnp(x: usize) -> bool_t {
    isapnp[x]
}

// MODULE_AUTHOR("Karsten Wiese <annabellesgarden@yahoo.de>");
// MODULE_DESCRIPTION("Turtle Beach " LONGNAME " Linux Driver");
// MODULE_LICENSE("GPL");
// MODULE_FIRMWARE(INITCODEFILE);
// MODULE_FIRMWARE(PERMCODEFILE);

unsafe extern "C" fn snd_msnd_isa_match(pdev: *mut device, i: c_uint) -> c_int {
    let i = i as usize;
    if io[i] == SNDRV_AUTO_PORT {
        return 0;
    }

    if irq[i] == SNDRV_AUTO_PORT as c_int || mem[i] == SNDRV_AUTO_PORT {
        dev_warn(pdev, b"snd_msnd_pinnacle: io, irq and mem must be set\n\0".as_ptr() as *const c_char);
        return 0;
    }

    // MSND_CLASSIC has a fixed allow-list for io.
    if io[i] < 0x100 || io[i] > 0x3e0 || (io[i] % 0x10) != 0 {
        dev_err(pdev, b"snd_msnd_pinnacle: \"io\" - DSP I/O base must within the range 0x100 to 0x3E0 and must be evenly divisible by 0x10\n\0".as_ptr() as *const c_char);
        return 0;
    }

    if !(irq[i] == 5 || irq[i] == 7 || irq[i] == 9 || irq[i] == 10 || irq[i] == 11 || irq[i] == 12) {
        dev_err(pdev, b"snd_msnd_pinnacle: \"irq\" - must be set to 5, 7, 9, 10, 11 or 12\n\0".as_ptr() as *const c_char);
        return 0;
    }

    if !(mem[i] == 0xb0000 || mem[i] == 0xc8000 || mem[i] == 0xd0000 || mem[i] == 0xd8000 || mem[i] == 0xe0000 || mem[i] == 0xe8000) {
        dev_err(pdev, b"snd_msnd_pinnacle: \"mem\" - must be set to 0xb0000, 0xc8000, 0xd0000, 0xd8000, 0xe0000 or 0xe8000\n\0".as_ptr() as *const c_char);
        return 0;
    }

    if cfg[i] == SNDRV_AUTO_PORT {
        dev_info(pdev, b"snd_msnd_pinnacle: Assuming PnP mode\n\0".as_ptr() as *const c_char);
    } else if cfg[i] != 0x250 && cfg[i] != 0x260 && cfg[i] != 0x270 {
        dev_info(pdev, b"snd_msnd_pinnacle: Config port must be 0x250, 0x260 or 0x270 (or unspecified for PnP mode)\n\0".as_ptr() as *const c_char);
        return 0;
    }

    1
}

unsafe extern "C" fn snd_msnd_isa_probe(pdev: *mut device, idx: c_uint) -> c_int {
    let idx = idx as usize;
    let mut err: c_int;
    let mut card: *mut snd_card = ptr::null_mut();
    let chip: *mut snd_msnd;

    if has_isapnp(idx) || cfg[idx] == SNDRV_AUTO_PORT {
        dev_info(pdev, b"snd_msnd_pinnacle: Assuming PnP mode\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }

    err = snd_devm_card_new(pdev, index[idx], id[idx], THIS_MODULE, core::mem::size_of::<snd_msnd>(), &mut card);
    if err < 0 {
        return err;
    }

    chip = (*card).private_data;
    (*chip).card = card;

    // MSND_CLASSIC maps irq and mem to chip->irqid/chip->memid here.
    dev_info(pdev, b"snd_msnd_pinnacle: Non-PnP mode: configuring at port 0x%lx\n\0".as_ptr() as *const c_char, cfg[idx]);

    if devm_request_region((*card).dev, cfg[idx], 2, b"Pinnacle/Fiji Config\0".as_ptr() as *const c_char).is_null() {
        dev_err(pdev, b"snd_msnd_pinnacle: Config port 0x%lx conflict\n\0".as_ptr() as *const c_char, cfg[idx]);
        return -EIO;
    }
    if reset[idx] != 0 {
        if snd_msnd_pinnacle_cfg_reset(chip, cfg[idx] as c_int) != 0 {
            return -EIO;
        }
    }

    /* DSP */
    err = snd_msnd_write_cfg_logical(chip, cfg[idx] as c_int, 0, io[idx] as u16, 0, irq[idx] as u16, mem[idx] as c_int);
    if err != 0 {
        return err;
    }

    /* The following are Pinnacle specific */

    /* MPU */
    if mpu_io[idx] != SNDRV_AUTO_PORT && mpu_irq[idx] != SNDRV_AUTO_IRQ {
        dev_info(pdev, b"snd_msnd_pinnacle: Configuring MPU to I/O 0x%lx IRQ %d\n\0".as_ptr() as *const c_char, mpu_io[idx], mpu_irq[idx]);
        err = snd_msnd_write_cfg_logical(chip, cfg[idx] as c_int, 1, mpu_io[idx] as u16, 0, mpu_irq[idx] as u16, 0);
        if err != 0 {
            return err;
        }
    }

    /* IDE */
    if ide_io0[idx] != SNDRV_AUTO_PORT && ide_io1[idx] != SNDRV_AUTO_PORT && ide_irq[idx] != SNDRV_AUTO_IRQ {
        dev_info(pdev, b"snd_msnd_pinnacle: Configuring IDE to I/O 0x%lx, 0x%lx IRQ %d\n\0".as_ptr() as *const c_char, ide_io0[idx], ide_io1[idx], ide_irq[idx]);
        err = snd_msnd_write_cfg_logical(chip, cfg[idx] as c_int, 2, ide_io0[idx] as u16, ide_io1[idx] as u16, ide_irq[idx] as u16, 0);
        if err != 0 {
            return err;
        }
    }

    /* Joystick */
    if joystick_io[idx] != SNDRV_AUTO_PORT {
        dev_info(pdev, b"snd_msnd_pinnacle: Configuring joystick to I/O 0x%lx\n\0".as_ptr() as *const c_char, joystick_io[idx]);
        err = snd_msnd_write_cfg_logical(chip, cfg[idx] as c_int, 3, joystick_io[idx] as u16, 0, 0, 0);
        if err != 0 {
            return err;
        }
    }

    set_default_audio_parameters(chip);
    (*chip).type_ = msndPinnacle;
    (*chip).io = io[idx];
    (*chip).irq = irq[idx];
    (*chip).base = mem[idx];

    (*chip).calibrate_signal = if calibrate_signal != 0 { 1 } else { 0 };
    (*chip).recsrc = 0;
    (*chip).dspq_data_buff = DSPQ_DATA_BUFF;
    (*chip).dspq_buff_size = DSPQ_BUFF_SIZE;
    if write_ndelay[idx] != 0 {
        clear_bit(F_DISABLE_WRITE_NDELAY, &mut (*chip).flags);
    } else {
        set_bit(F_DISABLE_WRITE_NDELAY, &mut (*chip).flags);
    }
    if digital[idx] != 0 {
        set_bit(F_HAVEDIGITAL, &mut (*chip).flags);
    }
    spin_lock_init(&mut (*chip).lock);
    err = snd_msnd_probe(card);
    if err < 0 {
        dev_err(pdev, b"snd_msnd_pinnacle: Probe failed\n\0".as_ptr() as *const c_char);
        return err;
    }

    err = snd_msnd_attach(card);
    if err < 0 {
        dev_err(pdev, b"snd_msnd_pinnacle: Attach failed\n\0".as_ptr() as *const c_char);
        return err;
    }
    dev_set_drvdata(pdev, card as *mut c_void);

    0
}

// CONFIG_PM
unsafe fn snd_msnd_card_suspend(card: *mut snd_card) -> c_int {
    let chip = (*card).private_data;
    let mpu: *mut snd_mpu401;
    let err: c_int;

    mpu = if !(*chip).rmidi.is_null() { (*(*chip).rmidi).private_data } else { ptr::null_mut() };
    (*chip).pm_recsrc = snd_msnd_pm_recsrc(chip);
    (*chip).pm_mpu_input = !mpu.is_null() && test_bit(MPU401_MODE_BIT_INPUT, &(*mpu).mode) != 0;
    if (*chip).pm_mpu_input {
        snd_msnd_send_dsp_cmd(chip, HDEX_MIDI_IN_STOP);
    }

    err = snd_msnd_force_irq(chip, false);
    if err < 0 {
        if (*chip).pm_mpu_input {
            snd_msnd_send_dsp_cmd(chip, HDEX_MIDI_IN_START);
        }
        return err;
    }

    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    0
}

unsafe fn snd_msnd_card_resume(card: *mut snd_card) -> c_int {
    let chip = (*card).private_data;
    let mut err: c_int;

    err = snd_msnd_initialize(card);
    if err < 0 {
        return err;
    }

    snd_msnd_calibrate_adc(chip, (*chip).play_sample_rate);
    snd_msndmix_force_recsrc(chip, (*chip).pm_recsrc);

    err = snd_msnd_force_irq(chip, true);
    if err < 0 {
        return err;
    }

    if (*chip).pm_mpu_input {
        snd_msnd_send_dsp_cmd(chip, HDEX_MIDI_IN_START);
    }

    (*chip).nresets = 0;
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}

unsafe extern "C" fn snd_msnd_isa_suspend(dev: *mut device, idx: c_uint, state: pm_message_t) -> c_int {
    let _ = idx;
    let _ = state;
    snd_msnd_card_suspend(dev_get_drvdata(dev) as *mut snd_card)
}

unsafe extern "C" fn snd_msnd_isa_resume(dev: *mut device, idx: c_uint) -> c_int {
    let _ = idx;
    snd_msnd_card_resume(dev_get_drvdata(dev) as *mut snd_card)
}

static mut snd_msnd_driver: isa_driver = isa_driver {
    match_: Some(snd_msnd_isa_match),
    probe: Some(snd_msnd_isa_probe),
    suspend: Some(snd_msnd_isa_suspend),
    resume: Some(snd_msnd_isa_resume),
    driver: isa_driver_driver {
        name: DEV_NAME.as_ptr() as *const c_char,
    },
};

// CONFIG_PNP
unsafe extern "C" fn snd_msnd_pnp_detect(pcard: *mut pnp_card_link, pid: *const pnp_card_device_id) -> c_int {
    static mut idx: c_int = 0;
    let pnp_dev: *mut pnp_dev;
    let mpu_dev: *mut pnp_dev;
    let mut card: *mut snd_card = ptr::null_mut();
    let chip: *mut snd_msnd;
    let mut ret: c_int;

    while idx < SNDRV_CARDS as c_int {
        if has_isapnp(idx as usize) {
            break;
        }
        idx += 1;
    }
    if idx >= SNDRV_CARDS as c_int {
        return -ENODEV;
    }

    /*
     * Check that we still have room for another sound card ...
     */
    pnp_dev = pnp_request_card_device(pcard, (*pid).devs[0].id.as_ptr(), ptr::null_mut());
    if pnp_dev.is_null() {
        return -ENODEV;
    }

    mpu_dev = pnp_request_card_device(pcard, (*pid).devs[1].id.as_ptr(), ptr::null_mut());
    if mpu_dev.is_null() {
        return -ENODEV;
    }

    if pnp_is_active(pnp_dev) == 0 && pnp_activate_dev(pnp_dev) < 0 {
        dev_info(&mut (*(*pcard).card).dev, b"msnd_pinnacle: device is inactive\n\0".as_ptr() as *const c_char);
        return -EBUSY;
    }

    if pnp_is_active(mpu_dev) == 0 && pnp_activate_dev(mpu_dev) < 0 {
        dev_info(&mut (*(*pcard).card).dev, b"msnd_pinnacle: MPU device is inactive\n\0".as_ptr() as *const c_char);
        return -EBUSY;
    }

    /*
     * Create a new ALSA sound card entry, in anticipation
     * of detecting our hardware ...
     */
    ret = snd_devm_card_new(&mut (*(*pcard).card).dev, index[idx as usize], id[idx as usize], THIS_MODULE, core::mem::size_of::<snd_msnd>(), &mut card);
    if ret < 0 {
        return ret;
    }

    chip = (*card).private_data;
    (*chip).card = card;

    /*
     * Read the correct parameters off the ISA PnP bus ...
     */
    io[idx as usize] = pnp_port_start(pnp_dev, 0);
    irq[idx as usize] = pnp_irq(pnp_dev, 0);
    mem[idx as usize] = pnp_mem_start(pnp_dev, 0);
    mpu_io[idx as usize] = pnp_port_start(mpu_dev, 0);
    mpu_irq[idx as usize] = pnp_irq(mpu_dev, 0);

    set_default_audio_parameters(chip);
    (*chip).type_ = msndPinnacle;
    (*chip).io = io[idx as usize];
    (*chip).irq = irq[idx as usize];
    (*chip).base = mem[idx as usize];

    (*chip).calibrate_signal = if calibrate_signal != 0 { 1 } else { 0 };
    (*chip).recsrc = 0;
    (*chip).dspq_data_buff = DSPQ_DATA_BUFF;
    (*chip).dspq_buff_size = DSPQ_BUFF_SIZE;
    if write_ndelay[idx as usize] != 0 {
        clear_bit(F_DISABLE_WRITE_NDELAY, &mut (*chip).flags);
    } else {
        set_bit(F_DISABLE_WRITE_NDELAY, &mut (*chip).flags);
    }
    if digital[idx as usize] != 0 {
        set_bit(F_HAVEDIGITAL, &mut (*chip).flags);
    }
    spin_lock_init(&mut (*chip).lock);
    ret = snd_msnd_probe(card);
    if ret < 0 {
        dev_err(&mut (*(*pcard).card).dev, b"snd_msnd_pinnacle: Probe failed\n\0".as_ptr() as *const c_char);
        return ret;
    }

    ret = snd_msnd_attach(card);
    if ret < 0 {
        dev_err(&mut (*(*pcard).card).dev, b"snd_msnd_pinnacle: Attach failed\n\0".as_ptr() as *const c_char);
        return ret;
    }

    pnp_set_card_drvdata(pcard, card as *mut c_void);
    idx += 1;
    0
}

// CONFIG_PM
unsafe extern "C" fn snd_msnd_pnp_suspend(pcard: *mut pnp_card_link, state: pm_message_t) -> c_int {
    let _ = state;
    snd_msnd_card_suspend(pnp_get_card_drvdata(pcard) as *mut snd_card)
}

unsafe extern "C" fn snd_msnd_pnp_resume(pcard: *mut pnp_card_link) -> c_int {
    snd_msnd_card_resume(pnp_get_card_drvdata(pcard) as *mut snd_card)
}

static mut isa_registered: c_int = 0;
static mut pnp_registered: c_int = 0;

static msnd_pnpids: [pnp_card_device_id; 2] = [
    /* Pinnacle PnP */
    pnp_card_device_id {
        id: [b'B' as c_char, b'V' as c_char, b'J' as c_char, b'0' as c_char, b'4' as c_char, b'4' as c_char, b'0' as c_char, 0],
        devs: [
            pnp_card_devs_id { id: [b'T' as c_char, b'B' as c_char, b'S' as c_char, b'0' as c_char, b'0' as c_char, b'0' as c_char, b'0' as c_char, 0] },
            pnp_card_devs_id { id: [b'T' as c_char, b'B' as c_char, b'S' as c_char, b'0' as c_char, b'0' as c_char, b'0' as c_char, b'1' as c_char, 0] },
        ],
    },
    pnp_card_device_id {
        id: [0; 8],
        devs: [
            pnp_card_devs_id { id: [0; 8] },
            pnp_card_devs_id { id: [0; 8] },
        ],
    }, /* end */
];

// MODULE_DEVICE_TABLE(pnp_card, msnd_pnpids);

static mut msnd_pnpc_driver: pnp_card_driver = pnp_card_driver {
    flags: PNP_DRIVER_RES_DO_NOT_CHANGE,
    name: b"msnd_pinnacle\0".as_ptr() as *const c_char,
    id_table: msnd_pnpids.as_ptr(),
    probe: Some(snd_msnd_pnp_detect),
    suspend: Some(snd_msnd_pnp_suspend),
    resume: Some(snd_msnd_pnp_resume),
};

unsafe fn snd_msnd_init() -> c_int {
    let mut err: c_int;

    err = isa_register_driver(&mut snd_msnd_driver, SNDRV_CARDS as c_uint);
    // CONFIG_PNP
    if err == 0 {
        isa_registered = 1;
    }

    err = pnp_register_card_driver(&mut msnd_pnpc_driver);
    if err == 0 {
        pnp_registered = 1;
    }

    if isa_registered != 0 {
        err = 0;
    }
    err
}

unsafe fn snd_msnd_exit() {
    // CONFIG_PNP
    if pnp_registered != 0 {
        pnp_unregister_card_driver(&mut msnd_pnpc_driver);
    }
    if isa_registered != 0 {
        isa_unregister_driver(&mut snd_msnd_driver);
    }
}

// module_init(snd_msnd_init);
// module_exit(snd_msnd_exit);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
