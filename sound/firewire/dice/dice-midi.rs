// SPDX-License-Identifier: GPL-2.0-only
/*
 * dice_midi.c - a part of driver for Dice based devices
 *
 * Copyright (c) 2014 Takashi Sakamoto
 */

// Source dependency: "dice.h".
// The surrounding driver is expected to provide the C-compatible types,
// constants, helpers, and guard/list macros referenced below.

unsafe extern "C" fn midi_open(substream: *mut snd_rawmidi_substream) -> core::ffi::c_int {
    let dice: *mut snd_dice = (*(*substream).rmidi).private_data as *mut snd_dice;
    let mut err: core::ffi::c_int;

    err = snd_dice_stream_lock_try(dice);
    if err < 0 {
        return err;
    }

    scoped_guard!(mutex, &mut (*dice).mutex, {
        err = snd_dice_stream_reserve_duplex(dice, 0, 0, 0);
        if err >= 0 {
            (*dice).substreams_counter += 1;
            err = snd_dice_stream_start_duplex(dice);
            if err < 0 {
                (*dice).substreams_counter -= 1;
            }
        }
    });

    if err < 0 {
        snd_dice_stream_lock_release(dice);
    }

    err
}

unsafe extern "C" fn midi_close(substream: *mut snd_rawmidi_substream) -> core::ffi::c_int {
    let dice: *mut snd_dice = (*(*substream).rmidi).private_data as *mut snd_dice;

    scoped_guard!(mutex, &mut (*dice).mutex, {
        (*dice).substreams_counter -= 1;
        snd_dice_stream_stop_duplex(dice);
    });

    snd_dice_stream_lock_release(dice);
    0
}

unsafe extern "C" fn midi_capture_trigger(
    substrm: *mut snd_rawmidi_substream,
    up: core::ffi::c_int,
) {
    let dice: *mut snd_dice = (*(*substrm).rmidi).private_data as *mut snd_dice;

    guard!(spinlock_irqsave)(&mut (*dice).lock);

    if up != 0 {
        amdtp_am824_midi_trigger(
            &mut (*dice).tx_stream[0],
            (*substrm).number,
            substrm,
        );
    } else {
        amdtp_am824_midi_trigger(
            &mut (*dice).tx_stream[0],
            (*substrm).number,
            core::ptr::null_mut(),
        );
    }
}

unsafe extern "C" fn midi_playback_trigger(
    substrm: *mut snd_rawmidi_substream,
    up: core::ffi::c_int,
) {
    let dice: *mut snd_dice = (*(*substrm).rmidi).private_data as *mut snd_dice;

    guard!(spinlock_irqsave)(&mut (*dice).lock);

    if up != 0 {
        amdtp_am824_midi_trigger(
            &mut (*dice).rx_stream[0],
            (*substrm).number,
            substrm,
        );
    } else {
        amdtp_am824_midi_trigger(
            &mut (*dice).rx_stream[0],
            (*substrm).number,
            core::ptr::null_mut(),
        );
    }
}

unsafe fn set_midi_substream_names(dice: *mut snd_dice, str_: *mut snd_rawmidi_str) {
    let mut subs: *mut snd_rawmidi_substream;

    list_for_each_entry!(subs, &mut (*str_).substreams, list, {
        scnprintf(
            (*subs).name.as_mut_ptr(),
            core::mem::size_of_val(&(*subs).name),
            c"%s MIDI %d".as_ptr(),
            (*(*dice).card).shortname.as_ptr(),
            (*subs).number + 1,
        );
    });
}

pub unsafe extern "C" fn snd_dice_create_midi(dice: *mut snd_dice) -> core::ffi::c_int {
    static CAPTURE_OPS: snd_rawmidi_ops = snd_rawmidi_ops {
        open: Some(midi_open),
        close: Some(midi_close),
        trigger: Some(midi_capture_trigger),
    };
    static PLAYBACK_OPS: snd_rawmidi_ops = snd_rawmidi_ops {
        open: Some(midi_open),
        close: Some(midi_close),
        trigger: Some(midi_playback_trigger),
    };
    let mut rmidi: *mut snd_rawmidi = core::ptr::null_mut();
    let mut str_: *mut snd_rawmidi_str;
    let mut midi_in_ports: core::ffi::c_uint;
    let mut midi_out_ports: core::ffi::c_uint;
    let mut i: core::ffi::c_int;
    let mut err: core::ffi::c_int;

    midi_in_ports = 0;
    midi_out_ports = 0;
    i = 0;
    while i < MAX_STREAMS as core::ffi::c_int {
        midi_in_ports = core::cmp::max(midi_in_ports, (*dice).tx_midi_ports[i as usize]);
        midi_out_ports = core::cmp::max(midi_out_ports, (*dice).rx_midi_ports[i as usize]);
        i += 1;
    }

    if midi_in_ports + midi_out_ports == 0 {
        return 0;
    }

    /* create midi ports */
    err = snd_rawmidi_new(
        (*dice).card,
        (*(*dice).card).driver.as_ptr(),
        0,
        midi_out_ports,
        midi_in_ports,
        &mut rmidi,
    );
    if err < 0 {
        return err;
    }

    snprintf(
        (*rmidi).name.as_mut_ptr(),
        core::mem::size_of_val(&(*rmidi).name),
        c"%s MIDI".as_ptr(),
        (*(*dice).card).shortname.as_ptr(),
    );
    (*rmidi).private_data = dice as *mut core::ffi::c_void;

    if midi_in_ports > 0 {
        (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_INPUT;

        snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT, &CAPTURE_OPS);

        str_ = &mut (*rmidi).streams[SNDRV_RAWMIDI_STREAM_INPUT as usize];

        set_midi_substream_names(dice, str_);
    }

    if midi_out_ports > 0 {
        (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_OUTPUT;

        snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT, &PLAYBACK_OPS);

        str_ = &mut (*rmidi).streams[SNDRV_RAWMIDI_STREAM_OUTPUT as usize];

        set_midi_substream_names(dice, str_);
    }

    if (midi_out_ports > 0) && (midi_in_ports > 0) {
        (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_DUPLEX;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
