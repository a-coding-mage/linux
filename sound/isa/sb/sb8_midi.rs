// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Routines for control of SoundBlaster cards - MIDI interface
 *
 * --
 *
 * Sun May  9 22:54:38 BST 1999 George David Morrison <gdm@gedamo.demon.co.uk>
 *   Fixed typo in snd_sb8dsp_midi_new_device which prevented midi from
 *   working.
 *
 * Sun May 11 12:34:56 UTC 2003 Clemens Ladisch <clemens@ladisch.de>
 *   Added full duplex UART mode for DSP version 2.0 and later.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type irqreturn_t = c_uint;

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timer_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_rawmidi {
    pub name: *mut c_char,
    pub info_flags: c_uint,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_rawmidi_substream {
    pub rmidi: *mut snd_rawmidi,
}

#[repr(C)]
pub struct snd_rawmidi_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream, c_int)>,
}

#[repr(C)]
pub struct snd_sb {
    pub rmidi: *mut snd_rawmidi,
    pub midi_input_lock: spinlock_t,
    pub open_lock: spinlock_t,
    pub open: c_uint,
    pub midi_substream_input: *mut snd_rawmidi_substream,
    pub midi_substream_output: *mut snd_rawmidi_substream,
    pub hardware: c_uint,
    pub midi_timer: timer_list,
    pub card: *mut snd_card,
}

unsafe extern "C" {
    static IRQ_NONE: irqreturn_t;
    static IRQ_HANDLED: irqreturn_t;
    static EAGAIN: c_int;
    static SB_HW_20: c_uint;
    static SB_OPEN_MIDI_INPUT: c_uint;
    static SB_OPEN_MIDI_OUTPUT: c_uint;
    static SB_OPEN_MIDI_INPUT_TRIGGER: c_uint;
    static SB_OPEN_MIDI_OUTPUT_TRIGGER: c_uint;
    static SB_DSP_MIDI_UART_IRQ: c_int;
    static SB_DSP_MIDI_INPUT_IRQ: c_int;
    static SB_DSP_MIDI_OUTPUT: c_int;
    static DATA_AVAIL: c_uint;
    static READ: c_uint;
    static STATUS: c_uint;
    static WRITE: c_uint;
    static SNDRV_RAWMIDI_STREAM_OUTPUT: c_int;
    static SNDRV_RAWMIDI_STREAM_INPUT: c_int;
    static SNDRV_RAWMIDI_INFO_OUTPUT: c_uint;
    static SNDRV_RAWMIDI_INFO_INPUT: c_uint;
    static SNDRV_RAWMIDI_INFO_DUPLEX: c_uint;
    static mut jiffies: c_ulong;

    fn inb(port: c_ulong) -> u8;
    fn outb(value: c_char, port: c_ulong);
    fn SBP(chip: *mut snd_sb, reg: c_uint) -> c_ulong;
    fn snd_rawmidi_receive(substream: *mut snd_rawmidi_substream, buffer: *const c_char, count: usize);
    fn snd_sbdsp_reset(chip: *mut snd_sb);
    fn snd_sbdsp_command(chip: *mut snd_sb, val: c_int) -> c_int;
    fn timer_delete_sync(timer: *mut timer_list) -> c_int;
    fn timer_delete(timer: *mut timer_list) -> c_int;
    fn snd_rawmidi_transmit_peek(
        substream: *mut snd_rawmidi_substream,
        buffer: *mut c_char,
        count: usize,
    ) -> c_int;
    fn snd_rawmidi_transmit_ack(substream: *mut snd_rawmidi_substream, count: c_int) -> c_int;
    fn mod_timer(timer: *mut timer_list, expires: c_ulong) -> c_int;
    fn snd_rawmidi_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        output_count: c_int,
        input_count: c_int,
        rmidi: *mut *mut snd_rawmidi,
    ) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn snd_rawmidi_set_ops(rmidi: *mut snd_rawmidi, stream: c_int, ops: *const snd_rawmidi_ops);
    fn timer_setup(timer: *mut timer_list, callback: unsafe extern "C" fn(*mut timer_list), flags: c_uint);
    fn timer_container_of_snd_sb_midi_timer(t: *mut timer_list) -> *mut snd_sb;
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t) -> c_ulong;
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
}

type c_ulong = core::ffi::c_ulong;

pub unsafe extern "C" fn snd_sb8dsp_midi_interrupt(chip: *mut snd_sb) -> irqreturn_t {
    let rmidi: *mut snd_rawmidi;
    let mut max: c_int = 64;
    let mut byte: c_char = 0;

    if chip.is_null() {
        return IRQ_NONE;
    }

    rmidi = (*chip).rmidi;
    if rmidi.is_null() {
        inb(SBP(chip, DATA_AVAIL)); /* ack interrupt */
        return IRQ_NONE;
    }

    spin_lock(&mut (*chip).midi_input_lock);
    while max > 0 {
        max -= 1;
        if (inb(SBP(chip, DATA_AVAIL)) & 0x80) != 0 {
            byte = inb(SBP(chip, READ)) as c_char;
            if ((*chip).open & SB_OPEN_MIDI_INPUT_TRIGGER) != 0 {
                snd_rawmidi_receive((*chip).midi_substream_input, &byte, 1);
            }
        }
    }
    spin_unlock(&mut (*chip).midi_input_lock);
    IRQ_HANDLED
}

unsafe extern "C" fn snd_sb8dsp_midi_input_open(substream: *mut snd_rawmidi_substream) -> c_int {
    let chip: *mut snd_sb;
    let valid_open_flags: c_uint;

    chip = (*(*substream).rmidi).private_data as *mut snd_sb;
    valid_open_flags = if (*chip).hardware >= SB_HW_20 {
        SB_OPEN_MIDI_OUTPUT | SB_OPEN_MIDI_OUTPUT_TRIGGER
    } else {
        0
    };
    let flags = spin_lock_irqsave(&mut (*chip).open_lock);
    if ((*chip).open & !valid_open_flags) != 0 {
        spin_unlock_irqrestore(&mut (*chip).open_lock, flags);
        return -EAGAIN;
    }
    (*chip).open |= SB_OPEN_MIDI_INPUT;
    (*chip).midi_substream_input = substream;
    if ((*chip).open & SB_OPEN_MIDI_OUTPUT) != 0 {
        spin_unlock_irqrestore(&mut (*chip).open_lock, flags);
        return 0;
    }
    spin_unlock_irqrestore(&mut (*chip).open_lock, flags);
    snd_sbdsp_reset(chip); /* reset DSP */
    if (*chip).hardware >= SB_HW_20 {
        snd_sbdsp_command(chip, SB_DSP_MIDI_UART_IRQ);
    }
    0
}

unsafe extern "C" fn snd_sb8dsp_midi_output_open(substream: *mut snd_rawmidi_substream) -> c_int {
    let chip: *mut snd_sb;
    let valid_open_flags: c_uint;

    chip = (*(*substream).rmidi).private_data as *mut snd_sb;
    valid_open_flags = if (*chip).hardware >= SB_HW_20 {
        SB_OPEN_MIDI_INPUT | SB_OPEN_MIDI_INPUT_TRIGGER
    } else {
        0
    };
    let flags = spin_lock_irqsave(&mut (*chip).open_lock);
    if ((*chip).open & !valid_open_flags) != 0 {
        spin_unlock_irqrestore(&mut (*chip).open_lock, flags);
        return -EAGAIN;
    }
    (*chip).open |= SB_OPEN_MIDI_OUTPUT;
    (*chip).midi_substream_output = substream;
    if ((*chip).open & SB_OPEN_MIDI_INPUT) != 0 {
        spin_unlock_irqrestore(&mut (*chip).open_lock, flags);
        return 0;
    }
    spin_unlock_irqrestore(&mut (*chip).open_lock, flags);
    snd_sbdsp_reset(chip); /* reset DSP */
    if (*chip).hardware >= SB_HW_20 {
        snd_sbdsp_command(chip, SB_DSP_MIDI_UART_IRQ);
    }
    0
}

unsafe extern "C" fn snd_sb8dsp_midi_input_close(substream: *mut snd_rawmidi_substream) -> c_int {
    let chip: *mut snd_sb;

    chip = (*(*substream).rmidi).private_data as *mut snd_sb;
    let flags = spin_lock_irqsave(&mut (*chip).open_lock);
    (*chip).open &= !(SB_OPEN_MIDI_INPUT | SB_OPEN_MIDI_INPUT_TRIGGER);
    (*chip).midi_substream_input = core::ptr::null_mut();
    if ((*chip).open & SB_OPEN_MIDI_OUTPUT) != 0 {
        spin_unlock_irqrestore(&mut (*chip).open_lock, flags);
        return 0;
    }
    spin_unlock_irqrestore(&mut (*chip).open_lock, flags);
    snd_sbdsp_reset(chip); /* reset DSP */
    0
}

unsafe extern "C" fn snd_sb8dsp_midi_output_close(substream: *mut snd_rawmidi_substream) -> c_int {
    let chip: *mut snd_sb;

    chip = (*(*substream).rmidi).private_data as *mut snd_sb;
    timer_delete_sync(&mut (*chip).midi_timer);
    let flags = spin_lock_irqsave(&mut (*chip).open_lock);
    (*chip).open &= !(SB_OPEN_MIDI_OUTPUT | SB_OPEN_MIDI_OUTPUT_TRIGGER);
    (*chip).midi_substream_output = core::ptr::null_mut();
    if ((*chip).open & SB_OPEN_MIDI_INPUT) != 0 {
        spin_unlock_irqrestore(&mut (*chip).open_lock, flags);
        return 0;
    }
    spin_unlock_irqrestore(&mut (*chip).open_lock, flags);
    snd_sbdsp_reset(chip); /* reset DSP */
    0
}

unsafe extern "C" fn snd_sb8dsp_midi_input_trigger(
    substream: *mut snd_rawmidi_substream,
    up: c_int,
) {
    let chip: *mut snd_sb;

    chip = (*(*substream).rmidi).private_data as *mut snd_sb;
    let flags = spin_lock_irqsave(&mut (*chip).open_lock);
    if up != 0 {
        if ((*chip).open & SB_OPEN_MIDI_INPUT_TRIGGER) == 0 {
            if (*chip).hardware < SB_HW_20 {
                snd_sbdsp_command(chip, SB_DSP_MIDI_INPUT_IRQ);
            }
            (*chip).open |= SB_OPEN_MIDI_INPUT_TRIGGER;
        }
    } else if ((*chip).open & SB_OPEN_MIDI_INPUT_TRIGGER) != 0 {
        if (*chip).hardware < SB_HW_20 {
            snd_sbdsp_command(chip, SB_DSP_MIDI_INPUT_IRQ);
        }
        (*chip).open &= !SB_OPEN_MIDI_INPUT_TRIGGER;
    }
    spin_unlock_irqrestore(&mut (*chip).open_lock, flags);
}

unsafe extern "C" fn snd_sb8dsp_midi_output_write(substream: *mut snd_rawmidi_substream) {
    let chip: *mut snd_sb;
    let mut byte: c_char = 0;
    let mut max: c_int = 32;

    /* how big is Tx FIFO? */
    chip = (*(*substream).rmidi).private_data as *mut snd_sb;
    while max > 0 {
        max -= 1;
        let flags = spin_lock_irqsave(&mut (*chip).open_lock);
        if snd_rawmidi_transmit_peek(substream, &mut byte, 1) != 1 {
            (*chip).open &= !SB_OPEN_MIDI_OUTPUT_TRIGGER;
            timer_delete(&mut (*chip).midi_timer);
            spin_unlock_irqrestore(&mut (*chip).open_lock, flags);
            break;
        }
        if (*chip).hardware >= SB_HW_20 {
            let mut timeout: c_int = 8;
            while (inb(SBP(chip, STATUS)) & 0x80) != 0 && {
                timeout -= 1;
                timeout > 0
            } {}
            if timeout == 0 {
                /* Tx FIFO full - try again later */
                spin_unlock_irqrestore(&mut (*chip).open_lock, flags);
                break;
            }
            outb(byte, SBP(chip, WRITE));
        } else {
            snd_sbdsp_command(chip, SB_DSP_MIDI_OUTPUT);
            snd_sbdsp_command(chip, byte as c_int);
        }
        snd_rawmidi_transmit_ack(substream, 1);
        spin_unlock_irqrestore(&mut (*chip).open_lock, flags);
    }
}

unsafe extern "C" fn snd_sb8dsp_midi_output_timer(t: *mut timer_list) {
    let chip: *mut snd_sb = timer_container_of_snd_sb_midi_timer(t);
    let substream: *mut snd_rawmidi_substream = (*chip).midi_substream_output;

    let flags = spin_lock_irqsave(&mut (*chip).open_lock);
    mod_timer(&mut (*chip).midi_timer, 1 + jiffies);
    spin_unlock_irqrestore(&mut (*chip).open_lock, flags);
    snd_sb8dsp_midi_output_write(substream);
}

unsafe extern "C" fn snd_sb8dsp_midi_output_trigger(
    substream: *mut snd_rawmidi_substream,
    up: c_int,
) {
    let chip: *mut snd_sb;

    chip = (*(*substream).rmidi).private_data as *mut snd_sb;
    let flags = spin_lock_irqsave(&mut (*chip).open_lock);
    if up != 0 {
        if ((*chip).open & SB_OPEN_MIDI_OUTPUT_TRIGGER) == 0 {
            mod_timer(&mut (*chip).midi_timer, 1 + jiffies);
            (*chip).open |= SB_OPEN_MIDI_OUTPUT_TRIGGER;
        }
    } else if ((*chip).open & SB_OPEN_MIDI_OUTPUT_TRIGGER) != 0 {
        (*chip).open &= !SB_OPEN_MIDI_OUTPUT_TRIGGER;
    }
    spin_unlock_irqrestore(&mut (*chip).open_lock, flags);

    if up != 0 {
        snd_sb8dsp_midi_output_write(substream);
    }
}

static snd_sb8dsp_midi_output: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_sb8dsp_midi_output_open),
    close: Some(snd_sb8dsp_midi_output_close),
    trigger: Some(snd_sb8dsp_midi_output_trigger),
};

static snd_sb8dsp_midi_input: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_sb8dsp_midi_input_open),
    close: Some(snd_sb8dsp_midi_input_close),
    trigger: Some(snd_sb8dsp_midi_input_trigger),
};

pub unsafe extern "C" fn snd_sb8dsp_midi(chip: *mut snd_sb, device: c_int) -> c_int {
    let mut rmidi: *mut snd_rawmidi = core::ptr::null_mut();
    let err: c_int;

    err = snd_rawmidi_new((*chip).card, c"SB8 MIDI".as_ptr(), device, 1, 1, &mut rmidi);
    if err < 0 {
        return err;
    }
    strscpy((*rmidi).name, c"SB8 MIDI".as_ptr());
    snd_rawmidi_set_ops(
        rmidi,
        SNDRV_RAWMIDI_STREAM_OUTPUT,
        &snd_sb8dsp_midi_output,
    );
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT, &snd_sb8dsp_midi_input);
    (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_OUTPUT | SNDRV_RAWMIDI_INFO_INPUT;
    if (*chip).hardware >= SB_HW_20 {
        (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_DUPLEX;
    }
    (*rmidi).private_data = chip as *mut c_void;
    timer_setup(&mut (*chip).midi_timer, snd_sb8dsp_midi_output_timer, 0);
    (*chip).rmidi = rmidi;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
