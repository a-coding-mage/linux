// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Routines for the GF1 MIDI interface - like UART 6850
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_ulong, c_void};
use core::ptr;

type bool_ = bool;

const SNDRV_GF1_HANDLER_MIDI_OUT: c_int = 0;
const SNDRV_GF1_HANDLER_MIDI_IN: c_int = 0;
const SNDRV_RAWMIDI_STREAM_OUTPUT: c_int = 0;
const SNDRV_RAWMIDI_STREAM_INPUT: c_int = 1;
const SNDRV_RAWMIDI_INFO_OUTPUT: c_uint = 0;
const SNDRV_RAWMIDI_INFO_INPUT: c_uint = 0;
const SNDRV_RAWMIDI_INFO_DUPLEX: c_uint = 0;
const MIDICTRL: c_int = 0;

#[repr(C)]
pub struct snd_card {
    pub dev: *mut c_void,
}

#[repr(C)]
pub struct snd_rawmidi {
    pub name: [c_char; 80],
    pub info_flags: c_uint,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_rawmidi_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_rawmidi_substream {
    pub rmidi: *mut snd_rawmidi,
    pub runtime: *mut snd_rawmidi_runtime,
}

#[repr(C)]
pub struct snd_rawmidi_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream, c_int)>,
}

type snd_gf1_interrupt_handler_t = Option<unsafe extern "C" fn(*mut snd_gus_card)>;

#[repr(C)]
pub struct snd_gus_card_gf1 {
    pub uart_cmd: c_uchar,
    pub uart_framing: c_uint,
    pub uart_overrun: c_uint,
    pub interrupt_handler_midi_out: snd_gf1_interrupt_handler_t,
    pub interrupt_handler_midi_in: snd_gf1_interrupt_handler_t,
    pub port: c_ulong,
}

#[repr(C)]
pub struct snd_gus_card {
    pub uart_cmd_lock: c_void,
    pub gf1: snd_gus_card_gf1,
    pub midi_substream_input: *mut snd_rawmidi_substream,
    pub midi_substream_output: *mut snd_rawmidi_substream,
    pub uart_enable: bool_,
    pub card: *mut snd_card,
    pub interwave: bool_,
    pub midi_uart: *mut snd_rawmidi,
}

unsafe extern "C" {
    fn spin_lock_irqsave(lock: *mut c_void, flags: c_ulong);
    fn spin_unlock_irqrestore(lock: *mut c_void, flags: c_ulong);
    fn snd_gf1_uart_stat(gus: *mut snd_gus_card) -> c_uchar;
    fn snd_gf1_uart_get(gus: *mut snd_gus_card) -> c_uchar;
    fn snd_gf1_uart_put(gus: *mut snd_gus_card, byte: c_char);
    fn snd_gf1_uart_cmd(gus: *mut snd_gus_card, cmd: c_uchar);
    fn snd_rawmidi_receive(substream: *mut snd_rawmidi_substream, buffer: *mut c_uchar, count: c_int) -> c_int;
    fn snd_rawmidi_transmit(substream: *mut snd_rawmidi_substream, buffer: *mut c_char, count: c_int) -> c_int;
    fn snd_gf1_set_default_handlers(gus: *mut snd_gus_card, what: c_int);
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
    fn udelay(usecs: c_ulong);
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn outb(value: c_uchar, port: c_ulong);
    fn GUSP(gus: *mut snd_gus_card, reg: c_int) -> c_ulong;
}

unsafe extern "C" fn snd_gf1_interrupt_midi_in(gus: *mut snd_gus_card) {
    let mut count: c_int;
    let mut stat: c_uchar;
    let mut byte: c_uchar;
    let mut data: c_uchar;
    let mut flags: c_ulong = 0;

    count = 10;
    while count != 0 {
        spin_lock_irqsave(&mut (*gus).uart_cmd_lock, flags);
        stat = snd_gf1_uart_stat(gus);
        if stat & 0x01 == 0 {
            /* data in Rx FIFO? */
            spin_unlock_irqrestore(&mut (*gus).uart_cmd_lock, flags);
            count -= 1;
            continue;
        }
        count = 100; /* arm counter to new value */
        data = snd_gf1_uart_get(gus);
        let _ = data;
        if (*gus).gf1.uart_cmd & 0x80 == 0 {
            spin_unlock_irqrestore(&mut (*gus).uart_cmd_lock, flags);
            continue;
        }
        if stat & 0x10 != 0 {
            /* framing error */
            (*gus).gf1.uart_framing = (*gus).gf1.uart_framing.wrapping_add(1);
            spin_unlock_irqrestore(&mut (*gus).uart_cmd_lock, flags);
            continue;
        }
        byte = snd_gf1_uart_get(gus);
        spin_unlock_irqrestore(&mut (*gus).uart_cmd_lock, flags);
        snd_rawmidi_receive((*gus).midi_substream_input, &mut byte, 1);
        if stat & 0x20 != 0 {
            (*gus).gf1.uart_overrun = (*gus).gf1.uart_overrun.wrapping_add(1);
        }
    }
}

unsafe extern "C" fn snd_gf1_interrupt_midi_out(gus: *mut snd_gus_card) {
    let mut byte: c_char = 0;
    let flags: c_ulong = 0;

    /* try unlock output */
    if snd_gf1_uart_stat(gus) & 0x01 != 0 {
        snd_gf1_interrupt_midi_in(gus);
    }

    spin_lock_irqsave(&mut (*gus).uart_cmd_lock, flags);
    if snd_gf1_uart_stat(gus) & 0x02 != 0 {
        /* Tx FIFO free? */
        if snd_rawmidi_transmit((*gus).midi_substream_output, &mut byte, 1) != 1 {
            /* no other bytes or error */
            snd_gf1_uart_cmd(gus, (*gus).gf1.uart_cmd & !0x20); /* disable Tx interrupt */
        } else {
            snd_gf1_uart_put(gus, byte);
        }
    }
    spin_unlock_irqrestore(&mut (*gus).uart_cmd_lock, flags);
}

unsafe extern "C" fn snd_gf1_uart_reset(gus: *mut snd_gus_card, close: c_int) {
    snd_gf1_uart_cmd(gus, 0x03); /* reset */
    if close == 0 && (*gus).uart_enable {
        udelay(160);
        snd_gf1_uart_cmd(gus, 0x00); /* normal operations */
    }
}

unsafe extern "C" fn snd_gf1_uart_output_open(substream: *mut snd_rawmidi_substream) -> c_int {
    let gus: *mut snd_gus_card;
    let flags: c_ulong = 0;

    gus = (*(*substream).rmidi).private_data as *mut snd_gus_card;
    spin_lock_irqsave(&mut (*gus).uart_cmd_lock, flags);
    if (*gus).gf1.uart_cmd & 0x80 == 0 {
        /* input active? */
        snd_gf1_uart_reset(gus, 0);
    }
    (*gus).gf1.interrupt_handler_midi_out = Some(snd_gf1_interrupt_midi_out);
    (*gus).midi_substream_output = substream;
    /*
     * #if 0
     * dev_dbg(gus->card->dev,
     *     "write init - cmd = 0x%x, stat = 0x%x\n",
     *     gus->gf1.uart_cmd, snd_gf1_uart_stat(gus));
     * #endif
     */
    spin_unlock_irqrestore(&mut (*gus).uart_cmd_lock, flags);
    0
}

unsafe extern "C" fn snd_gf1_uart_input_open(substream: *mut snd_rawmidi_substream) -> c_int {
    let gus: *mut snd_gus_card;
    let mut i: c_int;
    let flags: c_ulong = 0;

    gus = (*(*substream).rmidi).private_data as *mut snd_gus_card;
    spin_lock_irqsave(&mut (*gus).uart_cmd_lock, flags);
    if (*gus).gf1.interrupt_handler_midi_out != Some(snd_gf1_interrupt_midi_out) {
        snd_gf1_uart_reset(gus, 0);
    }
    (*gus).gf1.interrupt_handler_midi_in = Some(snd_gf1_interrupt_midi_in);
    (*gus).midi_substream_input = substream;
    if (*gus).uart_enable {
        i = 0;
        while i < 1000 && snd_gf1_uart_stat(gus) & 0x01 != 0 {
            snd_gf1_uart_get(gus); /* clean Rx */
            i += 1;
        }
        if i >= 1000 {
            dev_err(
                (*(*gus).card).dev,
                c"gus midi uart init read - cleanup error\n".as_ptr(),
            );
        }
    }
    /*
     * #if 0
     * dev_dbg(gus->card->dev,
     *     "read init - enable = %i, cmd = 0x%x, stat = 0x%x\n",
     *     gus->uart_enable, gus->gf1.uart_cmd, snd_gf1_uart_stat(gus));
     * dev_dbg(gus->card->dev,
     *     "[0x%x] reg (ctrl/status) = 0x%x, reg (data) = 0x%x (page = 0x%x)\n",
     *     gus->gf1.port + 0x100, inb(gus->gf1.port + 0x100),
     *     inb(gus->gf1.port + 0x101), inb(gus->gf1.port + 0x102));
     * #endif
     */
    spin_unlock_irqrestore(&mut (*gus).uart_cmd_lock, flags);
    0
}

unsafe extern "C" fn snd_gf1_uart_output_close(substream: *mut snd_rawmidi_substream) -> c_int {
    let gus: *mut snd_gus_card;
    let flags: c_ulong = 0;

    gus = (*(*substream).rmidi).private_data as *mut snd_gus_card;
    spin_lock_irqsave(&mut (*gus).uart_cmd_lock, flags);
    if (*gus).gf1.interrupt_handler_midi_in != Some(snd_gf1_interrupt_midi_in) {
        snd_gf1_uart_reset(gus, 1);
    }
    snd_gf1_set_default_handlers(gus, SNDRV_GF1_HANDLER_MIDI_OUT);
    (*gus).midi_substream_output = ptr::null_mut();
    spin_unlock_irqrestore(&mut (*gus).uart_cmd_lock, flags);
    0
}

unsafe extern "C" fn snd_gf1_uart_input_close(substream: *mut snd_rawmidi_substream) -> c_int {
    let gus: *mut snd_gus_card;
    let flags: c_ulong = 0;

    gus = (*(*substream).rmidi).private_data as *mut snd_gus_card;
    spin_lock_irqsave(&mut (*gus).uart_cmd_lock, flags);
    if (*gus).gf1.interrupt_handler_midi_out != Some(snd_gf1_interrupt_midi_out) {
        snd_gf1_uart_reset(gus, 1);
    }
    snd_gf1_set_default_handlers(gus, SNDRV_GF1_HANDLER_MIDI_IN);
    (*gus).midi_substream_input = ptr::null_mut();
    spin_unlock_irqrestore(&mut (*gus).uart_cmd_lock, flags);
    0
}

unsafe extern "C" fn snd_gf1_uart_input_trigger(
    substream: *mut snd_rawmidi_substream,
    up: c_int,
) {
    let gus: *mut snd_gus_card;
    let flags: c_ulong = 0;

    gus = (*(*substream).rmidi).private_data as *mut snd_gus_card;

    spin_lock_irqsave(&mut (*gus).uart_cmd_lock, flags);
    if up != 0 {
        if (*gus).gf1.uart_cmd & 0x80 == 0 {
            snd_gf1_uart_cmd(gus, (*gus).gf1.uart_cmd | 0x80); /* enable Rx interrupts */
        }
    } else if (*gus).gf1.uart_cmd & 0x80 != 0 {
        snd_gf1_uart_cmd(gus, (*gus).gf1.uart_cmd & !0x80); /* disable Rx interrupts */
    }
    spin_unlock_irqrestore(&mut (*gus).uart_cmd_lock, flags);
}

unsafe extern "C" fn snd_gf1_uart_output_trigger(
    substream: *mut snd_rawmidi_substream,
    up: c_int,
) {
    let mut flags: c_ulong = 0;
    let gus: *mut snd_gus_card;
    let mut byte: c_char = 0;
    let mut timeout: c_int;

    gus = (*(*substream).rmidi).private_data as *mut snd_gus_card;

    spin_lock_irqsave(&mut (*gus).uart_cmd_lock, flags);
    if up != 0 {
        if (*gus).gf1.uart_cmd & 0x20 == 0 {
            spin_unlock_irqrestore(&mut (*gus).uart_cmd_lock, flags);
            /* wait for empty Rx - Tx is probably unlocked */
            timeout = 10000;
            while {
                let cond = timeout > 0 && snd_gf1_uart_stat(gus) & 0x01 != 0;
                timeout -= 1;
                cond
            } {}
            /* Tx FIFO free? */
            spin_lock_irqsave(&mut (*gus).uart_cmd_lock, flags);
            if (*gus).gf1.uart_cmd & 0x20 != 0 {
                spin_unlock_irqrestore(&mut (*gus).uart_cmd_lock, flags);
                return;
            }
            if snd_gf1_uart_stat(gus) & 0x02 != 0 {
                if snd_rawmidi_transmit(substream, &mut byte, 1) != 1 {
                    spin_unlock_irqrestore(&mut (*gus).uart_cmd_lock, flags);
                    return;
                }
                snd_gf1_uart_put(gus, byte);
            }
            snd_gf1_uart_cmd(gus, (*gus).gf1.uart_cmd | 0x20); /* enable Tx interrupt */
        }
    } else if (*gus).gf1.uart_cmd & 0x20 != 0 {
        snd_gf1_uart_cmd(gus, (*gus).gf1.uart_cmd & !0x20);
    }
    spin_unlock_irqrestore(&mut (*gus).uart_cmd_lock, flags);
}

static snd_gf1_uart_output: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_gf1_uart_output_open),
    close: Some(snd_gf1_uart_output_close),
    trigger: Some(snd_gf1_uart_output_trigger),
};

static snd_gf1_uart_input: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_gf1_uart_input_open),
    close: Some(snd_gf1_uart_input_close),
    trigger: Some(snd_gf1_uart_input_trigger),
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_gf1_rawmidi_new(gus: *mut snd_gus_card, device: c_int) -> c_int {
    let mut rmidi: *mut snd_rawmidi = ptr::null_mut();
    let mut err: c_int;

    err = snd_rawmidi_new((*gus).card, c"GF1".as_ptr(), device, 1, 1, &mut rmidi);
    if err < 0 {
        return err;
    }
    strscpy(
        (*rmidi).name.as_mut_ptr(),
        if (*gus).interwave {
            c"AMD InterWave".as_ptr()
        } else {
            c"GF1".as_ptr()
        },
    );
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT, &snd_gf1_uart_output);
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT, &snd_gf1_uart_input);
    (*rmidi).info_flags |=
        SNDRV_RAWMIDI_INFO_OUTPUT | SNDRV_RAWMIDI_INFO_INPUT | SNDRV_RAWMIDI_INFO_DUPLEX;
    (*rmidi).private_data = gus as *mut c_void;
    (*gus).midi_uart = rmidi;
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_gf1_uart_suspend(gus: *mut snd_gus_card) {
    let flags: c_ulong = 0;

    spin_lock_irqsave(&mut (*gus).uart_cmd_lock, flags);
    outb(0x03, GUSP(gus, MIDICTRL));
    spin_unlock_irqrestore(&mut (*gus).uart_cmd_lock, flags);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_gf1_uart_resume(gus: *mut snd_gus_card) {
    let mut uart_cmd: u16;
    let active: bool_;
    let mut i: c_int;
    let flags: c_ulong = 0;

    spin_lock_irqsave(&mut (*gus).uart_cmd_lock, flags);
    active = !(*gus).midi_substream_input.is_null() || !(*gus).midi_substream_output.is_null();
    spin_unlock_irqrestore(&mut (*gus).uart_cmd_lock, flags);
    if !active {
        return;
    }

    /* snd_gf1_hw_start() already left MIDICTRL in reset. */
    usleep_range(160, 200);

    spin_lock_irqsave(&mut (*gus).uart_cmd_lock, flags);
    if (*gus).midi_substream_input.is_null() && (*gus).midi_substream_output.is_null() {
        spin_unlock_irqrestore(&mut (*gus).uart_cmd_lock, flags);
        return;
    }

    if !(*gus).midi_substream_output.is_null() {
        (*gus).gf1.interrupt_handler_midi_out = Some(snd_gf1_interrupt_midi_out);
    }
    if !(*gus).midi_substream_input.is_null() {
        (*gus).gf1.interrupt_handler_midi_in = Some(snd_gf1_interrupt_midi_in);
    }

    if !(*gus).uart_enable {
        spin_unlock_irqrestore(&mut (*gus).uart_cmd_lock, flags);
        return;
    }

    uart_cmd = (*gus).gf1.uart_cmd as u16;
    snd_gf1_uart_cmd(gus, 0x00);

    if !(*gus).midi_substream_input.is_null() {
        i = 0;
        while i < 1000 && snd_gf1_uart_stat(gus) & 0x01 != 0 {
            snd_gf1_uart_get(gus);
            i += 1;
        }
        if i >= 1000 {
            dev_err(
                (*(*gus).card).dev,
                c"gus midi uart resume - cleanup error\n".as_ptr(),
            );
        }
    }

    snd_gf1_uart_cmd(gus, uart_cmd as c_uchar);
    spin_unlock_irqrestore(&mut (*gus).uart_cmd_lock, flags);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
