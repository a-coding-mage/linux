// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Interface for OSS sequencer emulation
 *
 *  Copyright (C) 1999 Takashi Iwai <tiwai@suse.de>
 *
 * Changes
 * 19990227   Steve Ratcliffe   Made separate file and merged in latest
 * 				midi emulation.
 */

// External dependencies from Linux kernel headers:
// #include <linux/export.h>
// #include <linux/uaccess.h>
// #include <sound/core.h>
// #include "emux_voice.h"
// #include <sound/asoundef.h>
// #include <linux/ultrasound.h>

use std::os::raw::{c_char, c_int, c_uint, c_void};

// External type declarations from kernel headers
#[repr(C)]
pub struct snd_seq_oss_arg {
    pub private_data: *mut c_void,
    pub seq_mode: c_int,
    pub addr: snd_seq_addr,
    pub event_passing: c_int,
}

#[repr(C)]
pub struct snd_seq_addr {
    pub client: c_int,
    pub port: c_int,
}

#[repr(C)]
pub struct snd_emux {
    pub card: *mut c_void,
    pub name: [c_char; 64],
    pub max_voices: c_int,
    pub oss_synth: *mut snd_seq_device,
    pub sflist: *mut c_void,
    pub memhdr: *mut c_void,
    pub ops: snd_emux_ops,
}

#[repr(C)]
pub struct snd_emux_ops {
    pub load_fx: Option<
        unsafe extern "C" fn(
            *mut snd_emux,
            c_int,
            c_int,
            *const c_char,
            c_int,
        ) -> c_int,
    >,
    pub oss_ioctl: Option<unsafe extern "C" fn(*mut snd_emux, c_int, c_uint, c_int) -> c_int>,
}

#[repr(C)]
pub struct snd_emux_port {
    pub chset: snd_midi_channel_set,
    pub emu: *mut snd_emux,
    pub oss_arg: *mut snd_seq_oss_arg,
    pub port_mode: c_int,
    pub drum_flags: c_uint,
    pub volume_atten: c_int,
    pub ctrls: [c_int; 128],
}

#[repr(C)]
pub struct snd_midi_channel_set {
    pub client: c_int,
    pub port: c_int,
    pub max_channels: c_int,
    pub channels: *mut snd_midi_channel,
}

#[repr(C)]
pub struct snd_midi_channel {
    pub midi_program: c_int,
    pub midi_pressure: c_int,
    pub drum_channel: c_int,
    pub control: [c_int; 128],
}

#[repr(C)]
pub struct snd_seq_port_callback {
    pub owner: *mut c_void,
    pub event_input: Option<
        unsafe extern "C" fn(
            *mut snd_seq_event,
            c_int,
            *mut c_void,
            c_int,
            c_int,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_seq_event {
    pub event_type: c_int,
    pub data: snd_seq_event_data,
}

#[repr(C)]
pub union snd_seq_event_data {
    pub raw8: raw8_data,
    pub control: control_data,
}

#[repr(C)]
pub struct raw8_data {
    pub d: [u8; 12],
}

#[repr(C)]
pub struct control_data {
    pub channel: u8,
    pub param: u8,
    pub value: c_int,
}

#[repr(C)]
pub struct snd_seq_device {
    pub name: [c_char; 80],
}

#[repr(C)]
pub struct snd_seq_oss_callback {
    pub owner: *mut c_void,
    pub open: Option<unsafe extern "C" fn(*mut snd_seq_oss_arg, *mut c_void) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_seq_oss_arg) -> c_int>,
    pub ioctl:
        Option<unsafe extern "C" fn(*mut snd_seq_oss_arg, c_uint, c_ulong) -> c_int>,
    pub load_patch: Option<
        unsafe extern "C" fn(*mut snd_seq_oss_arg, c_int, *const c_char, c_int, c_int) -> c_int,
    >,
    pub reset: Option<unsafe extern "C" fn(*mut snd_seq_oss_arg) -> c_int>,
}

#[repr(C)]
pub struct soundfont_patch_info {
    pub patch_type: c_int,
    pub optarg: c_int,
}

#[repr(C)]
pub struct snd_seq_oss_reg {
    pub register_type: c_int,
    pub subtype: c_int,
    pub nvoices: c_int,
    pub oper: snd_seq_oss_callback,
    pub private_data: *mut c_void,
}

extern "C" {
    static THIS_MODULE: *mut c_void;
    static SNDRV_SEQ_DEV_ID_OSS: [c_char; 16];

    fn snd_seq_device_new(
        card: *mut c_void,
        device: c_int,
        id: *const c_char,
        argsize: c_int,
        rdevice: *mut *mut snd_seq_device,
    ) -> c_int;
    fn snd_device_register(card: *mut c_void, dev: *mut snd_seq_device) -> c_int;
    fn snd_device_free(card: *mut c_void, dev: *mut snd_seq_device);

    fn snd_emux_inc_count(emu: *mut snd_emux) -> c_int;
    fn snd_emux_dec_count(emu: *mut snd_emux);
    fn snd_emux_create_port(
        emu: *mut snd_emux,
        name: *const c_char,
        max_voice: c_int,
        oss: c_int,
        callback: *mut snd_seq_port_callback,
    ) -> *mut snd_emux_port;
    fn snd_emux_reset_port(port: *mut snd_emux_port);
    fn snd_emux_sounds_off_all(port: *mut snd_emux_port);
    fn snd_soundfont_close_check(sflist: *mut c_void, id: c_int);
    fn snd_seq_event_port_detach(client: c_int, port: c_int);

    fn snd_soundfont_load_guspatch(
        card: *mut c_void,
        sflist: *mut c_void,
        buf: *const c_char,
        count: c_int,
    ) -> c_int;
    fn snd_soundfont_load(
        card: *mut c_void,
        sflist: *mut c_void,
        buf: *const c_char,
        count: c_int,
        client: c_int,
    ) -> c_int;
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> c_ulong;

    fn snd_soundfont_remove_samples(sflist: *mut c_void);
    fn snd_soundfont_remove_unlocked(sflist: *mut c_void);
    fn snd_util_mem_avail(memhdr: *mut c_void) -> c_int;

    fn snd_emux_terminate_all(emu: *mut snd_emux);
    fn snd_emux_send_effect_oss(
        port: *mut snd_emux_port,
        chan: *mut snd_midi_channel,
        p1: c_uint,
        p2: c_int,
    );
    fn snd_emux_update_port(port: *mut snd_emux_port, update: c_int);
    fn snd_emux_update_channel(
        port: *mut snd_emux_port,
        chan: *mut snd_midi_channel,
        update: c_int,
    );
    fn snd_emux_event_input(
        ev: *mut snd_seq_event,
        direct: c_int,
        private_data: *mut c_void,
        atomic: c_int,
        hop: c_int,
    ) -> c_int;
    fn snd_emux_send_effect(
        port: *mut snd_emux_port,
        chan: *mut snd_midi_channel,
        fx: c_int,
        val: c_int,
        flag: c_int,
    );
}

// Kernel macro: snd_BUG_ON - similar behavior but in Rust
macro_rules! snd_BUG_ON {
    ($cond:expr) => {
        if $cond {
            true
        } else {
            false
        }
    };
}

// Kernel macro: SNDRV_SEQ_DEVICE_ARGPTR - get argument pointer from device
macro_rules! SNDRV_SEQ_DEVICE_ARGPTR {
    ($dev:expr) => {
        (($dev as *mut c_void as *mut u8).add(std::mem::size_of::<snd_seq_device>())
            as *mut snd_seq_oss_reg)
    };
}

// use port number as a unique soundfont client number
fn SF_CLIENT_NO(p: c_int) -> c_int {
    p + 0x1000
}

const DEFAULT_DRUM_FLAGS: c_uint = ((1 << 9) | (1 << 25)) as c_uint;

// External constants from headers (kernel defined macros)
// SYNTH_TYPE_SAMPLE, SAMPLE_TYPE_AWE32
// SNDRV_EMUX_PORT_MODE_OSS_MIDI, SNDRV_EMUX_PORT_MODE_OSS_SYNTH
// SNDRV_SEQ_OSS_PROCESS_KEYPRESS, SNDRV_SEQ_OSS_PROCESS_EVENTS
// GUS_PATCH, SNDRV_OSS_SOUNDFONT_PATCH, SNDRV_SFNT_LOAD_INFO, SNDRV_SFNT_PROBE_DATA
// SNDCTL_SEQ_RESETSAMPLES, SNDCTL_SYNTH_MEMAVL, SNDRV_SEQ_EVENT_OSS
// _EMUX_OSS_MODE_VALUE_MASK, _EMUX_OSS_MODE_FLAG
// MIDI_CTL_ALL_NOTES_OFF, MIDI_CTL_ALL_SOUNDS_OFF
// SNDRV_EMUX_UPDATE_VOLUME, SNDRV_EMUX_UPDATE_FMMOD, SNDRV_EMUX_UPDATE_FM2FRQ2
// MIDI_CTL_MSB_PAN, SNDRV_EMUX_UPDATE_PAN
// _EMUX_OSS_SEND_EFFECT, _EMUX_OSS_TERMINATE_ALL, _EMUX_OSS_TERMINATE_CHANNEL
// _EMUX_OSS_RESET_CHANNEL, _EMUX_OSS_RELEASE_ALL, _EMUX_OSS_NOTEOFF_ALL
// _EMUX_OSS_INITIAL_VOLUME, _EMUX_OSS_CHN_PRESSURE, _EMUX_OSS_CHANNEL_MODE
// _EMUX_OSS_DRUM_CHANNELS, _EMUX_OSS_MISC_MODE, _EMUX_OSS_DEBUG_MODE, EMUX_MD_END
// _GUS_NUMVOICES, _GUS_VOICESAMPLE, _GUS_VOICEBALA, _GUS_VOICEVOL, _GUS_VOICEVOL2
// _GUS_RAMPRANGE, _GUS_RAMPRATE, _GUS_RAMPMODE, _GUS_RAMPON, _GUS_RAMPOFF
// _GUS_VOLUME_SCALE, _GUS_VOICE_POS
// SNDRV_EMUX_USE_RAW_EFFECT, EMUX_FX_SAMPLE_START, EMUX_FX_COARSE_SAMPLE_START, EMUX_FX_FLAG_SET
// SNDRV_SEQ_EVENT_CONTROLLER

// Operators callback structure
static OSS_CALLBACK: snd_seq_oss_callback = snd_seq_oss_callback {
    owner: unsafe { std::ptr::addr_of!(THIS_MODULE) as *mut c_void },
    open: Some(snd_emux_open_seq_oss),
    close: Some(snd_emux_close_seq_oss),
    ioctl: Some(snd_emux_ioctl_seq_oss),
    load_patch: Some(snd_emux_load_patch_seq_oss),
    reset: Some(snd_emux_reset_seq_oss),
};

// register OSS synth
pub unsafe extern "C" fn snd_emux_init_seq_oss(emu: *mut snd_emux) {
    let mut dev: *mut snd_seq_device = std::ptr::null_mut();

    // using device#1 here for avoiding conflicts with OPL3
    if snd_seq_device_new(
        (*emu).card,
        1,
        SNDRV_SEQ_DEV_ID_OSS.as_ptr(),
        std::mem::size_of::<snd_seq_oss_reg>() as c_int,
        &mut dev,
    ) < 0
    {
        return;
    }

    (*emu).oss_synth = dev;
    let name_len = (*emu).name.iter().position(|&c| c == 0).unwrap_or(63);
    std::ptr::copy_nonoverlapping(
        (*emu).name.as_ptr(),
        (*dev).name.as_mut_ptr(),
        name_len,
    );

    let arg = SNDRV_SEQ_DEVICE_ARGPTR!(dev);
    (*arg).register_type = 1; // SYNTH_TYPE_SAMPLE
    (*arg).subtype = 0x20; // SAMPLE_TYPE_AWE32
    (*arg).nvoices = (*emu).max_voices;
    (*arg).oper = OSS_CALLBACK;
    (*arg).private_data = emu as *mut c_void;

    // register to OSS synth table
    snd_device_register((*emu).card, dev);
}

// unregister
pub unsafe extern "C" fn snd_emux_detach_seq_oss(emu: *mut snd_emux) {
    if !(*emu).oss_synth.is_null() {
        snd_device_free((*emu).card, (*emu).oss_synth);
        (*emu).oss_synth = std::ptr::null_mut();
    }
}

// open port for OSS sequencer
unsafe extern "C" fn snd_emux_open_seq_oss(
    arg: *mut snd_seq_oss_arg,
    closure: *mut c_void,
) -> c_int {
    let emu = closure as *mut snd_emux;

    if snd_BUG_ON!(arg.is_null() || emu.is_null()) {
        return -6; // -ENXIO
    }

    if snd_emux_inc_count(emu) == 0 {
        return -14; // -EFAULT
    }

    let mut callback: snd_seq_port_callback = std::mem::zeroed();
    callback.owner = std::ptr::addr_of!(THIS_MODULE) as *mut c_void;
    callback.event_input = Some(snd_emux_event_oss_input);

    let mut tmpname: [c_char; 64] = [0; 64];
    let name_slice = std::ffi::CStr::from_ptr((*emu).name.as_ptr());
    let name_bytes = name_slice.to_bytes();
    let suffix = b" OSS Port";
    let len = name_bytes.len().min(64 - suffix.len() - 1);
    std::ptr::copy_nonoverlapping(name_bytes.as_ptr(), tmpname.as_mut_ptr() as *mut u8, len);
    std::ptr::copy_nonoverlapping(
        suffix.as_ptr(),
        tmpname.as_mut_ptr().add(len) as *mut u8,
        suffix.len(),
    );

    let p = snd_emux_create_port(emu, tmpname.as_ptr(), 32, 1, &mut callback);
    if p.is_null() {
        snd_emux_dec_count(emu);
        return -12; // -ENOMEM
    }

    // fill the argument data
    (*arg).private_data = p as *mut c_void;
    (*arg).addr.client = (*(*p).chset).client;
    (*arg).addr.port = (*(*p).chset).port;
    (*p).oss_arg = arg;

    reset_port_mode(p, (*arg).seq_mode);

    snd_emux_reset_port(p);
    0
}

// reset port mode
unsafe extern "C" fn reset_port_mode(port: *mut snd_emux_port, midi_mode: c_int) {
    if midi_mode != 0 {
        (*port).port_mode = 0; // SNDRV_EMUX_PORT_MODE_OSS_MIDI
        (*port).drum_flags = DEFAULT_DRUM_FLAGS;
        (*port).volume_atten = 0;
        (*(*port).oss_arg).event_passing = 1; // SNDRV_SEQ_OSS_PROCESS_KEYPRESS
    } else {
        (*port).port_mode = 1; // SNDRV_EMUX_PORT_MODE_OSS_SYNTH
        (*port).drum_flags = 0;
        (*port).volume_atten = 32;
        (*(*port).oss_arg).event_passing = 0; // SNDRV_SEQ_OSS_PROCESS_EVENTS
    }
}

// close port
unsafe extern "C" fn snd_emux_close_seq_oss(arg: *mut snd_seq_oss_arg) -> c_int {
    if snd_BUG_ON!(arg.is_null()) {
        return -6; // -ENXIO
    }
    let p = (*arg).private_data as *mut snd_emux_port;
    if snd_BUG_ON!(p.is_null()) {
        return -6; // -ENXIO
    }

    let emu = (*p).emu;
    if snd_BUG_ON!(emu.is_null()) {
        return -6; // -ENXIO
    }

    snd_emux_sounds_off_all(p);
    snd_soundfont_close_check((*emu).sflist, SF_CLIENT_NO((*(*p).chset).port));
    snd_seq_event_port_detach((*(*p).chset).client, (*(*p).chset).port);
    snd_emux_dec_count(emu);

    0
}

// load patch
unsafe extern "C" fn snd_emux_load_patch_seq_oss(
    arg: *mut snd_seq_oss_arg,
    format: c_int,
    buf: *const c_char,
    _offs: c_int,
    count: c_int,
) -> c_int {
    if snd_BUG_ON!(arg.is_null()) {
        return -6; // -ENXIO
    }
    let p = (*arg).private_data as *mut snd_emux_port;
    if snd_BUG_ON!(p.is_null()) {
        return -6; // -ENXIO
    }

    let emu = (*p).emu;
    if snd_BUG_ON!(emu.is_null()) {
        return -6; // -ENXIO
    }

    let rc = if format == 1 {
        // GUS_PATCH
        snd_soundfont_load_guspatch((*emu).card, (*emu).sflist, buf, count)
    } else if format == 2 {
        // SNDRV_OSS_SOUNDFONT_PATCH
        if count < std::mem::size_of::<soundfont_patch_info>() as c_int {
            return -22; // -EINVAL
        }
        let mut patch: soundfont_patch_info = std::mem::zeroed();
        if copy_from_user(&mut patch as *mut _ as *mut c_void, buf as *const c_void, std::mem::size_of::<soundfont_patch_info>()) != 0
        {
            return -14; // -EFAULT
        }
        if patch.patch_type >= 1 && patch.patch_type <= 8 {
            // SNDRV_SFNT_LOAD_INFO to SNDRV_SFNT_PROBE_DATA
            snd_soundfont_load(
                (*emu).card,
                (*emu).sflist,
                buf,
                count,
                SF_CLIENT_NO((*(*p).chset).port),
            )
        } else {
            if let Some(load_fx) = (*emu).ops.load_fx {
                load_fx(emu, patch.patch_type, patch.optarg, buf, count)
            } else {
                -22 // -EINVAL
            }
        }
    } else {
        0
    };
    rc
}

// ioctl
unsafe extern "C" fn snd_emux_ioctl_seq_oss(
    arg: *mut snd_seq_oss_arg,
    cmd: c_uint,
    _ioarg: c_ulong,
) -> c_int {
    if snd_BUG_ON!(arg.is_null()) {
        return -6; // -ENXIO
    }
    let p = (*arg).private_data as *mut snd_emux_port;
    if snd_BUG_ON!(p.is_null()) {
        return -6; // -ENXIO
    }

    let emu = (*p).emu;
    if snd_BUG_ON!(emu.is_null()) {
        return -6; // -ENXIO
    }

    match cmd {
        0x40004123 => {
            // SNDCTL_SEQ_RESETSAMPLES
            snd_soundfont_remove_samples((*emu).sflist);
            0
        }
        0x40044144 => {
            // SNDCTL_SYNTH_MEMAVL
            if !(*emu).memhdr.is_null() {
                snd_util_mem_avail((*emu).memhdr)
            } else {
                0
            }
        }
        _ => 0,
    }
}

// reset device
unsafe extern "C" fn snd_emux_reset_seq_oss(arg: *mut snd_seq_oss_arg) -> c_int {
    if snd_BUG_ON!(arg.is_null()) {
        return -6; // -ENXIO
    }
    let p = (*arg).private_data as *mut snd_emux_port;
    if snd_BUG_ON!(p.is_null()) {
        return -6; // -ENXIO
    }
    snd_emux_reset_port(p);
    0
}

// receive raw events: only SEQ_PRIVATE is accepted.
unsafe extern "C" fn snd_emux_event_oss_input(
    ev: *mut snd_seq_event,
    direct: c_int,
    private_data: *mut c_void,
    atomic: c_int,
    hop: c_int,
) -> c_int {
    let p = private_data as *mut snd_emux_port;
    if snd_BUG_ON!(p.is_null()) {
        return -22; // -EINVAL
    }
    let emu = (*p).emu;
    if snd_BUG_ON!(emu.is_null()) {
        return -22; // -EINVAL
    }
    if (*ev).event_type != 5 {
        // SNDRV_SEQ_EVENT_OSS
        return snd_emux_event_input(ev, direct, private_data, atomic, hop);
    }

    let data = &(*ev).data.raw8.d;
    // only SEQ_PRIVATE is accepted
    if data[0] != 0xfe {
        return 0;
    }
    let cmd = (data[2] as c_int) & 0x7f; // _EMUX_OSS_MODE_VALUE_MASK
    if (data[2] as c_int) & 0x80 != 0 {
        // _EMUX_OSS_MODE_FLAG
        emuspec_control(emu, p, cmd, data.as_ptr() as *mut u8, atomic, hop);
    } else {
        gusspec_control(emu, p, cmd, data.as_ptr() as *mut u8, atomic, hop);
    }
    0
}

// OSS/AWE driver specific h/w controls
unsafe extern "C" fn emuspec_control(
    emu: *mut snd_emux,
    port: *mut snd_emux_port,
    cmd: c_int,
    event: *const u8,
    atomic: c_int,
    hop: c_int,
) {
    let voice = *event.add(3) as c_int;
    let chan = if voice < 0 || voice >= (*(*port).chset).max_channels {
        std::ptr::null_mut()
    } else {
        (*(*port).chset)
            .channels
            .add(voice as usize) as *mut snd_midi_channel
    };

    let p1 = *(event.add(4) as *const u16);
    let p2 = *(event.add(6) as *const i16);

    match cmd {
        1 => {
            // _EMUX_OSS_SEND_EFFECT
            if !chan.is_null() {
                snd_emux_send_effect_oss(port, chan, p1 as c_uint, p2 as c_int);
            }
        }
        2 => {
            // _EMUX_OSS_TERMINATE_ALL
            snd_emux_terminate_all(emu);
        }
        3 => {
            // _EMUX_OSS_TERMINATE_CHANNEL
            // commented in C: snd_emux_mute_channel(emu, chan);
        }
        4 => {
            // _EMUX_OSS_RESET_CHANNEL
            // commented in C: snd_emux_channel_init(chset, chan);
        }
        5 => {
            // _EMUX_OSS_RELEASE_ALL
            fake_event(emu, port, voice, 123, 0, atomic, hop); // MIDI_CTL_ALL_NOTES_OFF = 123
        }
        6 => {
            // _EMUX_OSS_NOTEOFF_ALL
            fake_event(emu, port, voice, 120, 0, atomic, hop); // MIDI_CTL_ALL_SOUNDS_OFF = 120
        }
        7 => {
            // _EMUX_OSS_INITIAL_VOLUME
            if p2 != 0 {
                (*port).volume_atten = p1 as c_int;
                snd_emux_update_port(port, 1); // SNDRV_EMUX_UPDATE_VOLUME
            }
        }
        8 => {
            // _EMUX_OSS_CHN_PRESSURE
            if !chan.is_null() {
                (*chan).midi_pressure = p1 as c_int;
                snd_emux_update_channel(port, chan, 3); // SNDRV_EMUX_UPDATE_FMMOD|SNDRV_EMUX_UPDATE_FM2FRQ2
            }
        }
        9 => {
            // _EMUX_OSS_CHANNEL_MODE
            reset_port_mode(port, p1 as c_int);
            snd_emux_reset_port(port);
        }
        10 => {
            // _EMUX_OSS_DRUM_CHANNELS
            (*port).drum_flags = *(event.add(4) as *const c_uint);
            for i in 0..(*(*port).chset).max_channels {
                let ch = (*(*port).chset)
                    .channels
                    .add(i as usize) as *mut snd_midi_channel;
                (*ch).drum_channel = if (((*port).drum_flags >> i) & 1) != 0 { 1 } else { 0 };
            }
        }
        11 => {
            // _EMUX_OSS_MISC_MODE
            if (p1 as c_int) < 5 {
                // EMUX_MD_END
                (*port).ctrls[p1 as usize] = p2 as c_int;
            }
        }
        12 => {
            // _EMUX_OSS_DEBUG_MODE
        }
        _ => {
            if let Some(oss_ioctl) = (*emu).ops.oss_ioctl {
                oss_ioctl(emu, cmd, p1 as c_uint, p2 as c_int);
            }
        }
    }
}

// GUS specific h/w controls
// #include <linux/ultrasound.h>
unsafe extern "C" fn gusspec_control(
    _emu: *mut snd_emux,
    port: *mut snd_emux_port,
    cmd: c_int,
    event: *const u8,
    _atomic: c_int,
    _hop: c_int,
) {
    if (*port).port_mode != 1 {
        // SNDRV_EMUX_PORT_MODE_OSS_SYNTH
        return;
    }
    if cmd == 0 {
        // _GUS_NUMVOICES
        return;
    }
    let voice = *event.add(3) as c_int;
    if voice < 0 || voice >= (*(*port).chset).max_channels {
        return;
    }

    let chan = (*(*port).chset)
        .channels
        .add(voice as usize) as *mut snd_midi_channel;

    let p1 = *(event.add(4) as *const u16);
    let plong = *(event.add(4) as *const c_int);

    match cmd {
        1 => {
            // _GUS_VOICESAMPLE
            (*chan).midi_program = p1 as c_int;
        }
        2 => {
            // _GUS_VOICEBALA
            // 0 to 15 --> 0 to 127
            (*chan).control[10] = (p1 as c_int) << 3; // MIDI_CTL_MSB_PAN = 10
            snd_emux_update_channel(port, chan, 4); // SNDRV_EMUX_UPDATE_PAN
        }
        3 | 4 => {
            // _GUS_VOICEVOL, _GUS_VOICEVOL2
            // not supported yet
        }
        5 | 6 | 7 | 8 | 9 => {
            // _GUS_RAMPRANGE, _GUS_RAMPRATE, _GUS_RAMPMODE, _GUS_RAMPON, _GUS_RAMPOFF
            // volume ramping not supported
        }
        10 => {
            // _GUS_VOLUME_SCALE
        }
        11 => {
            // _GUS_VOICE_POS
            // #ifdef SNDRV_EMUX_USE_RAW_EFFECT
            snd_emux_send_effect(
                port,
                chan,
                0, // EMUX_FX_SAMPLE_START
                ((plong & 0x7fff) as c_int),
                1, // EMUX_FX_FLAG_SET
            );
            snd_emux_send_effect(
                port,
                chan,
                1, // EMUX_FX_COARSE_SAMPLE_START
                (((plong >> 15) & 0xffff) as c_int),
                1, // EMUX_FX_FLAG_SET
            );
            // #endif
        }
        _ => {}
    }
}

// send an event to midi emulation
unsafe extern "C" fn fake_event(
    emu: *mut snd_emux,
    port: *mut snd_emux_port,
    ch: c_int,
    param: c_int,
    val: c_int,
    atomic: c_int,
    hop: c_int,
) {
    let mut ev: snd_seq_event = std::mem::zeroed();
    ev.event_type = 10; // SNDRV_SEQ_EVENT_CONTROLLER
    ev.data.control.channel = ch as u8;
    ev.data.control.param = param as u8;
    ev.data.control.value = val;
    snd_emux_event_input(&mut ev, 0, port as *mut c_void, atomic, hop);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
