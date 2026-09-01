// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright 10/16/2005 Tilman Kranz <tilde@tk-sls.de>
 *  Creative Audio MIDI, for the CA0106 Driver
 *  Version: 0.0.1
 *
 *  Changelog:
 *    Implementation is based on mpu401 and emu10k1x and
 *    tested with ca0106.
 *    mpu401: Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *    emu10k1x: Copyright (c) by Francisco Moraes <fmoraes@nc.rr.com>
 */

// Dependencies from linux/spinlock.h, sound/core.h, sound/rawmidi.h, and ca_midi.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type SpinlockT = c_void;

const ENXIO: c_int = 6;
const CA_MIDI_MODE_INPUT: c_uint = 1;
const CA_MIDI_MODE_OUTPUT: c_uint = 2;
const SNDRV_RAWMIDI_STREAM_OUTPUT: c_int = 0;
const SNDRV_RAWMIDI_STREAM_INPUT: c_int = 1;
const SNDRV_RAWMIDI_INFO_OUTPUT: c_uint = 0x00000001;
const SNDRV_RAWMIDI_INFO_INPUT: c_uint = 0x00000002;
const SNDRV_RAWMIDI_INFO_DUPLEX: c_uint = 0x00000004;

#[repr(C)]
pub struct snd_rawmidi {
    pub name: [c_char; 80],
    pub info_flags: c_uint,
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_rawmidi)>,
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
pub struct snd_ca_midi {
    pub dev_id: *mut c_void,
    pub rmidi: *mut snd_rawmidi,
    pub interrupt: Option<unsafe extern "C" fn(*mut snd_ca_midi, c_uint)>,
    pub interrupt_enable: Option<unsafe extern "C" fn(*mut snd_ca_midi, c_uint)>,
    pub interrupt_disable: Option<unsafe extern "C" fn(*mut snd_ca_midi, c_uint)>,
    pub read: Option<unsafe extern "C" fn(*mut snd_ca_midi, c_int) -> c_uint>,
    pub write: Option<unsafe extern "C" fn(*mut snd_ca_midi, c_uint, c_int)>,
    pub get_dev_id_card: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    pub get_dev_id_port: Option<unsafe extern "C" fn(*mut c_void) -> c_uint>,
    pub open_lock: SpinlockT,
    pub input_lock: SpinlockT,
    pub output_lock: SpinlockT,
    pub midi_mode: c_uint,
    pub substream_input: *mut snd_rawmidi_substream,
    pub substream_output: *mut snd_rawmidi_substream,
    pub tx_enable: c_uint,
    pub rx_enable: c_uint,
    pub ipr_rx: c_uint,
    pub ipr_tx: c_uint,
    pub input_avail: c_uint,
    pub output_ready: c_uint,
    pub ack: c_uint,
    pub reset: u8,
    pub enter_uart: u8,
}

unsafe extern "C" {
    fn snd_rawmidi_receive(
        substream: *mut snd_rawmidi_substream,
        buffer: *mut u8,
        count: c_int,
    ) -> c_int;
    fn snd_rawmidi_transmit(
        substream: *mut snd_rawmidi_substream,
        buffer: *mut u8,
        count: c_int,
    ) -> c_int;
    fn snd_rawmidi_new(
        card: *mut c_void,
        id: *mut c_char,
        device: c_int,
        output_count: c_int,
        input_count: c_int,
        rmidi: *mut *mut snd_rawmidi,
    ) -> c_int;
    fn snd_rawmidi_set_ops(rmidi: *mut snd_rawmidi, stream: c_int, ops: *const snd_rawmidi_ops);
    fn spin_lock_init(lock: *mut SpinlockT);
    fn spin_lock(lock: *mut SpinlockT);
    fn spin_unlock(lock: *mut SpinlockT);
    fn spin_lock_irqsave(lock: *mut SpinlockT);
    fn spin_unlock_irqrestore(lock: *mut SpinlockT);
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn pr_err(fmt: *const c_char, ...);
    fn snd_BUG_ON(condition: bool) -> bool;
}

#[inline]
unsafe fn ca_midi_write_data(midi: *mut snd_ca_midi, data: c_uint) {
    ((*midi).write.unwrap())(midi, data, 0);
}

#[inline]
unsafe fn ca_midi_write_cmd(midi: *mut snd_ca_midi, data: c_uint) {
    ((*midi).write.unwrap())(midi, data, 1);
}

#[inline]
unsafe fn ca_midi_read_data(midi: *mut snd_ca_midi) -> c_uint {
    ((*midi).read.unwrap())(midi, 0)
}

#[inline]
unsafe fn ca_midi_read_stat(midi: *mut snd_ca_midi) -> c_uint {
    ((*midi).read.unwrap())(midi, 1)
}

#[inline]
unsafe fn ca_midi_input_avail(midi: *mut snd_ca_midi) -> bool {
    !(ca_midi_read_stat(midi) & (*midi).input_avail) != 0
}

#[inline]
unsafe fn ca_midi_output_ready(midi: *mut snd_ca_midi) -> bool {
    !(ca_midi_read_stat(midi) & (*midi).output_ready) != 0
}

unsafe extern "C" fn ca_midi_clear_rx(midi: *mut snd_ca_midi) {
    let mut timeout: c_int = 100000;
    while timeout > 0 && ca_midi_input_avail(midi) {
        timeout -= 1;
        ca_midi_read_data(midi);
    }
    // CONFIG_SND_DEBUG: report timeout status when kernel debug is enabled.
    /*
    if timeout <= 0 {
        pr_err(
            c"ca_midi_clear_rx: timeout (status = 0x%x)\n".as_ptr(),
            ca_midi_read_stat(midi),
        );
    }
    */
}

unsafe extern "C" fn ca_midi_interrupt(midi: *mut snd_ca_midi, status: c_uint) {
    let mut byte: u8 = 0;

    if (*midi).rmidi.is_null() {
        ((*midi).interrupt_disable.unwrap())(midi, (*midi).tx_enable | (*midi).rx_enable);
        return;
    }

    spin_lock(&mut (*midi).input_lock);
    if (status & (*midi).ipr_rx) != 0 && ca_midi_input_avail(midi) {
        if ((*midi).midi_mode & CA_MIDI_MODE_INPUT) == 0 {
            ca_midi_clear_rx(midi);
        } else {
            byte = ca_midi_read_data(midi) as u8;
            if !(*midi).substream_input.is_null() {
                snd_rawmidi_receive((*midi).substream_input, &mut byte, 1);
            }
        }
    }
    spin_unlock(&mut (*midi).input_lock);

    spin_lock(&mut (*midi).output_lock);
    if (status & (*midi).ipr_tx) != 0 && ca_midi_output_ready(midi) {
        if !(*midi).substream_output.is_null()
            && snd_rawmidi_transmit((*midi).substream_output, &mut byte, 1) == 1
        {
            ca_midi_write_data(midi, byte as c_uint);
        } else {
            ((*midi).interrupt_disable.unwrap())(midi, (*midi).tx_enable);
        }
    }
    spin_unlock(&mut (*midi).output_lock);
}

unsafe extern "C" fn ca_midi_cmd(midi: *mut snd_ca_midi, cmd: u8, ack: c_int) {
    let mut timeout: c_int;
    let mut ok: c_int;

    spin_lock_irqsave(&mut (*midi).input_lock);
    ca_midi_write_data(midi, 0x00);
    /* ca_midi_clear_rx(midi); */

    ca_midi_write_cmd(midi, cmd as c_uint);
    if ack != 0 {
        ok = 0;
        timeout = 10000;
        while ok == 0 && {
            let old = timeout;
            timeout -= 1;
            old > 0
        } {
            if ca_midi_input_avail(midi) {
                if ca_midi_read_data(midi) == (*midi).ack {
                    ok = 1;
                }
            }
        }
        if ok == 0 && ca_midi_read_data(midi) == (*midi).ack {
            ok = 1;
        }
    } else {
        ok = 1;
    }
    spin_unlock_irqrestore(&mut (*midi).input_lock);
    if ok == 0 {
        pr_err(
            c"ca_midi_cmd: 0x%x failed at 0x%x (status = 0x%x, data = 0x%x)!!!\n".as_ptr(),
            cmd as c_uint,
            ((*midi).get_dev_id_port.unwrap())((*midi).dev_id),
            ca_midi_read_stat(midi),
            ca_midi_read_data(midi),
        );
    }
}

unsafe extern "C" fn ca_midi_input_open(substream: *mut snd_rawmidi_substream) -> c_int {
    let midi = (*(*substream).rmidi).private_data as *mut snd_ca_midi;

    if snd_BUG_ON((*midi).dev_id.is_null()) {
        return -ENXIO;
    }
    spin_lock_irqsave(&mut (*midi).open_lock);
    (*midi).midi_mode |= CA_MIDI_MODE_INPUT;
    (*midi).substream_input = substream;
    if ((*midi).midi_mode & CA_MIDI_MODE_OUTPUT) != 0 {
        spin_unlock_irqrestore(&mut (*midi).open_lock);
        return 0;
    }
    spin_unlock_irqrestore(&mut (*midi).open_lock);
    ca_midi_cmd(midi, (*midi).reset, 1);
    ca_midi_cmd(midi, (*midi).enter_uart, 1);
    0
}

unsafe extern "C" fn ca_midi_output_open(substream: *mut snd_rawmidi_substream) -> c_int {
    let midi = (*(*substream).rmidi).private_data as *mut snd_ca_midi;

    if snd_BUG_ON((*midi).dev_id.is_null()) {
        return -ENXIO;
    }
    spin_lock_irqsave(&mut (*midi).open_lock);
    (*midi).midi_mode |= CA_MIDI_MODE_OUTPUT;
    (*midi).substream_output = substream;
    if ((*midi).midi_mode & CA_MIDI_MODE_INPUT) != 0 {
        spin_unlock_irqrestore(&mut (*midi).open_lock);
        return 0;
    }
    spin_unlock_irqrestore(&mut (*midi).open_lock);
    ca_midi_cmd(midi, (*midi).reset, 1);
    ca_midi_cmd(midi, (*midi).enter_uart, 1);
    0
}

unsafe extern "C" fn ca_midi_input_close(substream: *mut snd_rawmidi_substream) -> c_int {
    let midi = (*(*substream).rmidi).private_data as *mut snd_ca_midi;

    if snd_BUG_ON((*midi).dev_id.is_null()) {
        return -ENXIO;
    }
    spin_lock_irqsave(&mut (*midi).open_lock);
    ((*midi).interrupt_disable.unwrap())(midi, (*midi).rx_enable);
    (*midi).midi_mode &= !CA_MIDI_MODE_INPUT;
    (*midi).substream_input = ptr::null_mut();
    if ((*midi).midi_mode & CA_MIDI_MODE_OUTPUT) != 0 {
        spin_unlock_irqrestore(&mut (*midi).open_lock);
        return 0;
    }
    spin_unlock_irqrestore(&mut (*midi).open_lock);
    ca_midi_cmd(midi, (*midi).reset, 0);
    0
}

unsafe extern "C" fn ca_midi_output_close(substream: *mut snd_rawmidi_substream) -> c_int {
    let midi = (*(*substream).rmidi).private_data as *mut snd_ca_midi;

    if snd_BUG_ON((*midi).dev_id.is_null()) {
        return -ENXIO;
    }

    spin_lock_irqsave(&mut (*midi).open_lock);
    ((*midi).interrupt_disable.unwrap())(midi, (*midi).tx_enable);
    (*midi).midi_mode &= !CA_MIDI_MODE_OUTPUT;
    (*midi).substream_output = ptr::null_mut();
    if ((*midi).midi_mode & CA_MIDI_MODE_INPUT) != 0 {
        spin_unlock_irqrestore(&mut (*midi).open_lock);
        return 0;
    }
    spin_unlock_irqrestore(&mut (*midi).open_lock);
    ca_midi_cmd(midi, (*midi).reset, 0);
    0
}

unsafe extern "C" fn ca_midi_input_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    let midi = (*(*substream).rmidi).private_data as *mut snd_ca_midi;

    if snd_BUG_ON((*midi).dev_id.is_null()) {
        return;
    }

    if up != 0 {
        ((*midi).interrupt_enable.unwrap())(midi, (*midi).rx_enable);
    } else {
        ((*midi).interrupt_disable.unwrap())(midi, (*midi).rx_enable);
    }
}

unsafe extern "C" fn ca_midi_output_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    let midi = (*(*substream).rmidi).private_data as *mut snd_ca_midi;

    if snd_BUG_ON((*midi).dev_id.is_null()) {
        return;
    }

    if up != 0 {
        let mut max: c_int = 4;
        let mut byte: u8 = 0;

        spin_lock_irqsave(&mut (*midi).output_lock);

        /* try to send some amount of bytes here before interrupts */
        while max > 0 {
            if ca_midi_output_ready(midi) {
                if ((*midi).midi_mode & CA_MIDI_MODE_OUTPUT) == 0
                    || snd_rawmidi_transmit(substream, &mut byte, 1) != 1
                {
                    /* no more data */
                    spin_unlock_irqrestore(&mut (*midi).output_lock);
                    return;
                }
                ca_midi_write_data(midi, byte as c_uint);
                max -= 1;
            } else {
                break;
            }
        }
        spin_unlock_irqrestore(&mut (*midi).output_lock);
        ((*midi).interrupt_enable.unwrap())(midi, (*midi).tx_enable);
    } else {
        ((*midi).interrupt_disable.unwrap())(midi, (*midi).tx_enable);
    }
}

static CA_MIDI_OUTPUT: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(ca_midi_output_open),
    close: Some(ca_midi_output_close),
    trigger: Some(ca_midi_output_trigger),
};

static CA_MIDI_INPUT: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(ca_midi_input_open),
    close: Some(ca_midi_input_close),
    trigger: Some(ca_midi_input_trigger),
};

unsafe extern "C" fn ca_midi_free(midi: *mut snd_ca_midi) {
    (*midi).interrupt = None;
    (*midi).interrupt_enable = None;
    (*midi).interrupt_disable = None;
    (*midi).read = None;
    (*midi).write = None;
    (*midi).get_dev_id_card = None;
    (*midi).get_dev_id_port = None;
    (*midi).rmidi = ptr::null_mut();
}

unsafe extern "C" fn ca_rmidi_free(rmidi: *mut snd_rawmidi) {
    ca_midi_free((*rmidi).private_data as *mut snd_ca_midi);
}

#[no_mangle]
pub unsafe extern "C" fn ca_midi_init(
    dev_id: *mut c_void,
    midi: *mut snd_ca_midi,
    device: c_int,
    name: *mut c_char,
) -> c_int {
    let mut rmidi: *mut snd_rawmidi = ptr::null_mut();
    let err: c_int;

    err = snd_rawmidi_new(
        ((*midi).get_dev_id_card.unwrap())((*midi).dev_id),
        name,
        device,
        1,
        1,
        &mut rmidi,
    );
    if err < 0 {
        return err;
    }

    (*midi).dev_id = dev_id;
    (*midi).interrupt = Some(ca_midi_interrupt);

    spin_lock_init(&mut (*midi).open_lock);
    spin_lock_init(&mut (*midi).input_lock);
    spin_lock_init(&mut (*midi).output_lock);

    strscpy((*rmidi).name.as_mut_ptr(), name);
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT, &CA_MIDI_OUTPUT);
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT, &CA_MIDI_INPUT);
    (*rmidi).info_flags |=
        SNDRV_RAWMIDI_INFO_OUTPUT | SNDRV_RAWMIDI_INFO_INPUT | SNDRV_RAWMIDI_INFO_DUPLEX;
    (*rmidi).private_data = midi as *mut c_void;
    (*rmidi).private_free = Some(ca_rmidi_free);

    (*midi).rmidi = rmidi;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
