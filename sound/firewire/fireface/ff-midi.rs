// SPDX-License-Identifier: GPL-2.0-only
/*
 * ff-midi.c - a part of driver for RME Fireface series
 *
 * Copyright (c) 2015-2017 Takashi Sakamoto
 */

// C source dependency: "ff.h".

use core::ffi::{c_char, c_int};
use core::ptr;

unsafe extern "C" fn midi_capture_open(
    substream: *mut snd_rawmidi_substream,
) -> c_int {
    /* Do nothing. */
    0
}

unsafe extern "C" fn midi_playback_open(
    substream: *mut snd_rawmidi_substream,
) -> c_int {
    let ff = (*(*substream).rmidi).private_data as *mut snd_ff;

    /* Initialize internal status. */
    (*ff).on_sysex[(*substream).number as usize] = 0;
    (*ff).rx_midi_error[(*substream).number as usize] = false;

    WRITE_ONCE(
        &mut (*ff).rx_midi_substreams[(*substream).number as usize],
        substream,
    );

    0
}

unsafe extern "C" fn midi_capture_close(
    substream: *mut snd_rawmidi_substream,
) -> c_int {
    /* Do nothing. */
    0
}

unsafe extern "C" fn midi_playback_close(
    substream: *mut snd_rawmidi_substream,
) -> c_int {
    let ff = (*(*substream).rmidi).private_data as *mut snd_ff;

    cancel_work_sync(&mut (*ff).rx_midi_work[(*substream).number as usize]);
    WRITE_ONCE(
        &mut (*ff).rx_midi_substreams[(*substream).number as usize],
        ptr::null_mut::<snd_rawmidi_substream>(),
    );

    0
}

unsafe extern "C" fn midi_capture_trigger(
    substream: *mut snd_rawmidi_substream,
    up: c_int,
) {
    let ff = (*(*substream).rmidi).private_data as *mut snd_ff;

    let _guard = guard_spinlock_irqsave(&mut (*ff).lock);

    if up != 0 {
        WRITE_ONCE(
            &mut (*ff).tx_midi_substreams[(*substream).number as usize],
            substream,
        );
    } else {
        WRITE_ONCE(
            &mut (*ff).tx_midi_substreams[(*substream).number as usize],
            ptr::null_mut::<snd_rawmidi_substream>(),
        );
    }
}

unsafe extern "C" fn midi_playback_trigger(
    substream: *mut snd_rawmidi_substream,
    up: c_int,
) {
    let ff = (*(*substream).rmidi).private_data as *mut snd_ff;

    let _guard = guard_spinlock_irqsave(&mut (*ff).lock);

    if up != 0 || !(*ff).rx_midi_error[(*substream).number as usize] {
        schedule_work(&mut (*ff).rx_midi_work[(*substream).number as usize]);
    }
}

unsafe fn set_midi_substream_names(
    stream: *mut snd_rawmidi_str,
    name: *const c_char,
) {
    let mut substream: *mut snd_rawmidi_substream;

    list_for_each_entry!(substream, &mut (*stream).substreams, list, {
        scnprintf(
            (*substream).name.as_mut_ptr(),
            core::mem::size_of_val(&(*substream).name),
            c"%s MIDI %d".as_ptr(),
            name,
            (*substream).number + 1,
        );
    });
}

pub unsafe extern "C" fn snd_ff_create_midi_devices(ff: *mut snd_ff) -> c_int {
    static MIDI_CAPTURE_OPS: snd_rawmidi_ops = snd_rawmidi_ops {
        open: Some(midi_capture_open),
        close: Some(midi_capture_close),
        trigger: Some(midi_capture_trigger),
    };
    static MIDI_PLAYBACK_OPS: snd_rawmidi_ops = snd_rawmidi_ops {
        open: Some(midi_playback_open),
        close: Some(midi_playback_close),
        trigger: Some(midi_playback_trigger),
    };
    let mut rmidi: *mut snd_rawmidi = ptr::null_mut();
    let mut stream: *mut snd_rawmidi_str;
    let mut err: c_int;

    err = snd_rawmidi_new(
        (*ff).card,
        (*(*ff).card).driver.as_ptr(),
        0,
        (*(*ff).spec).midi_out_ports,
        (*(*ff).spec).midi_in_ports,
        &mut rmidi,
    );
    if err < 0 {
        return err;
    }

    snprintf(
        (*rmidi).name.as_mut_ptr(),
        core::mem::size_of_val(&(*rmidi).name),
        c"%s MIDI".as_ptr(),
        (*(*ff).card).shortname.as_ptr(),
    );
    (*rmidi).private_data = ff as *mut _;

    (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_INPUT;
    snd_rawmidi_set_ops(
        rmidi,
        SNDRV_RAWMIDI_STREAM_INPUT,
        &MIDI_CAPTURE_OPS,
    );
    stream = &mut (*rmidi).streams[SNDRV_RAWMIDI_STREAM_INPUT as usize];
    set_midi_substream_names(stream, (*(*ff).card).shortname.as_ptr());

    (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_OUTPUT;
    snd_rawmidi_set_ops(
        rmidi,
        SNDRV_RAWMIDI_STREAM_OUTPUT,
        &MIDI_PLAYBACK_OPS,
    );
    stream = &mut (*rmidi).streams[SNDRV_RAWMIDI_STREAM_OUTPUT as usize];
    set_midi_substream_names(stream, (*(*ff).card).shortname.as_ptr());

    (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_DUPLEX;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
