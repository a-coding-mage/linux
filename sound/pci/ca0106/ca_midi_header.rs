/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Copyright 10/16/2005 Tilman Kranz <tilde@tk-sls.de>
 *  Creative Audio MIDI, for the CA0106 Driver
 *  Version: 0.0.1
 *
 *  Changelog:
 *    See ca_midi.c
 */

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_void};

/* C dependencies:
 * #include <linux/spinlock.h>
 * #include <sound/rawmidi.h>
 * #include <sound/mpu401.h>
 */

pub const CA_MIDI_MODE_INPUT: c_uint = MPU401_MODE_INPUT;
pub const CA_MIDI_MODE_OUTPUT: c_uint = MPU401_MODE_OUTPUT;

#[repr(C)]
pub struct snd_ca_midi {
    pub rmidi: *mut snd_rawmidi,
    pub substream_input: *mut snd_rawmidi_substream,
    pub substream_output: *mut snd_rawmidi_substream,

    pub dev_id: *mut c_void,

    pub input_lock: spinlock_t,
    pub output_lock: spinlock_t,
    pub open_lock: spinlock_t,

    pub channel: c_uint,

    pub midi_mode: c_uint,
    pub port: c_int,
    pub tx_enable: c_int,
    pub rx_enable: c_int,
    pub ipr_tx: c_int,
    pub ipr_rx: c_int,

    pub input_avail: c_int,
    pub output_ready: c_int,
    pub ack: c_int,
    pub reset: c_int,
    pub enter_uart: c_int,

    pub interrupt: Option<unsafe extern "C" fn(midi: *mut snd_ca_midi, status: c_uint)>,
    pub interrupt_enable: Option<unsafe extern "C" fn(midi: *mut snd_ca_midi, intr: c_int)>,
    pub interrupt_disable: Option<unsafe extern "C" fn(midi: *mut snd_ca_midi, intr: c_int)>,

    pub read: Option<unsafe extern "C" fn(midi: *mut snd_ca_midi, idx: c_int) -> c_uchar>,
    pub write: Option<unsafe extern "C" fn(midi: *mut snd_ca_midi, data: c_int, idx: c_int)>,

    /* get info from dev_id */
    pub get_dev_id_card: Option<unsafe extern "C" fn(dev_id: *mut c_void) -> *mut snd_card>,
    pub get_dev_id_port: Option<unsafe extern "C" fn(dev_id: *mut c_void) -> c_int>,
}

unsafe extern "C" {
    pub fn ca_midi_init(
        card: *mut c_void,
        midi: *mut snd_ca_midi,
        device: c_int,
        name: *mut c_char,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
