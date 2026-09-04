// SPDX-License-Identifier: GPL-2.0-or-later
//
// Midi Sequencer interface routines.
//
// Copyright (C) 1999 Steve Ratcliffe
// Copyright (c) 1999-2000 Takashi Iwai <tiwai@suse.de>

// Requires emux_voice.h, linux/slab.h, linux/module.h from C build

// Prototypes for static functions
fn free_port(private: *mut core::ffi::c_void);
fn snd_emux_init_port(p: *mut snd_emux_port);
fn snd_emux_use(private_data: *mut core::ffi::c_void, info: *mut snd_seq_port_subscribe) -> i32;
fn snd_emux_unuse(private_data: *mut core::ffi::c_void, info: *mut snd_seq_port_subscribe) -> i32;

// External types referenced from emux_voice.h and sequencer API
#[repr(C)]
pub struct snd_emux {
    // Fields from header
}

#[repr(C)]
pub struct snd_card {
    // Fields from header
}

#[repr(C)]
pub struct snd_seq_port_subscribe {
    // Fields from header
}

#[repr(C)]
pub struct snd_seq_port_callback {
    pub owner: *mut core::ffi::c_void,
    pub use_fn: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut snd_seq_port_subscribe) -> i32>,
    pub unuse: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut snd_seq_port_subscribe) -> i32>,
    pub event_input: Option<unsafe extern "C" fn(*mut snd_seq_event, i32, *mut core::ffi::c_void, i32, i32) -> i32>,
    pub private_free: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub private_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct snd_emux_port {
    pub chset: snd_midi_channel_set,
    pub port_mode: u32,
    pub drum_flags: u32,
    pub volume_atten: i32,
    pub ctrls: [u32; 32],
    pub emu: *mut snd_emux,
}

#[repr(C)]
pub struct snd_midi_channel_set {
    pub channels: [snd_midi_channel; 16],
    pub private_data: *mut core::ffi::c_void,
    pub client: i32,
    pub port: i32,
    pub max_channels: i32,
}

#[repr(C)]
pub struct snd_midi_channel {
    pub number: i32,
    pub drum_channel: i32,
}

#[repr(C)]
pub struct snd_midi_op {
    pub note_on: Option<unsafe extern "C" fn(*mut snd_midi_channel, i32, i32)>,
    pub note_off: Option<unsafe extern "C" fn(*mut snd_midi_channel, i32, i32)>,
    pub key_press: Option<unsafe extern "C" fn(*mut snd_midi_channel, i32, i32)>,
    pub note_terminate: Option<unsafe extern "C" fn(*mut snd_midi_channel, i32)>,
    pub control: Option<unsafe extern "C" fn(*mut snd_midi_channel, i32, i32)>,
    pub nrpn: Option<unsafe extern "C" fn(*mut snd_midi_channel, *mut snd_seq_event)>,
    pub sysex: Option<unsafe extern "C" fn(*mut snd_midi_channel, *mut u8, i32)>,
}

#[repr(C)]
pub struct snd_seq_event {
    // Fields from header
}

// MIDI emulation operators
static EMUX_OPS: snd_midi_op = snd_midi_op {
    note_on: Some(snd_emux_note_on),
    note_off: Some(snd_emux_note_off),
    key_press: Some(snd_emux_key_press),
    note_terminate: Some(snd_emux_terminate_note),
    control: Some(snd_emux_control),
    nrpn: Some(snd_emux_nrpn),
    sysex: Some(snd_emux_sysex),
};

// Number of MIDI channels
const MIDI_CHANNELS: usize = 16;

// Type flags for MIDI sequencer port
const DEFAULT_MIDI_TYPE: u32 = (SNDRV_SEQ_PORT_TYPE_MIDI_GENERIC
    | SNDRV_SEQ_PORT_TYPE_MIDI_GM
    | SNDRV_SEQ_PORT_TYPE_MIDI_GS
    | SNDRV_SEQ_PORT_TYPE_MIDI_XG
    | SNDRV_SEQ_PORT_TYPE_HARDWARE
    | SNDRV_SEQ_PORT_TYPE_SYNTHESIZER);

const DEFAULT_DRUM_FLAGS: u32 = 1 << 9;

// Sequencer port type flags (from sequencer API)
const SNDRV_SEQ_PORT_TYPE_MIDI_GENERIC: u32 = 1;
const SNDRV_SEQ_PORT_TYPE_MIDI_GM: u32 = 2;
const SNDRV_SEQ_PORT_TYPE_MIDI_GS: u32 = 4;
const SNDRV_SEQ_PORT_TYPE_MIDI_XG: u32 = 8;
const SNDRV_SEQ_PORT_TYPE_HARDWARE: u32 = 16;
const SNDRV_SEQ_PORT_TYPE_SYNTHESIZER: u32 = 32;
const SNDRV_SEQ_PORT_TYPE_SPECIFIC: u32 = 64;
const SNDRV_SEQ_PORT_CAP_WRITE: u32 = 1;
const SNDRV_SEQ_PORT_CAP_SUBS_WRITE: u32 = 2;

// Emux mode and control constants
const SNDRV_EMUX_PORT_MODE_MIDI: u32 = 1;
const EMUX_MD_DEF_BANK: usize = 0;
const EMUX_MD_DEF_DRUM: usize = 1;
const EMUX_MD_REALTIME_PAN: usize = 2;

// External functions from kernel/sequencer API
extern "C" {
    fn snd_seq_create_kernel_client(card: *mut snd_card, index: i32, name: *const i8, ...) -> i32;
    fn snd_seq_delete_kernel_client(client: i32);
    fn snd_emux_create_port(
        emu: *mut snd_emux,
        name: *const i8,
        max_channels: i32,
        oss_port: i32,
        callback: *mut snd_seq_port_callback,
    ) -> *mut snd_emux_port;
    fn snd_emux_terminate_all(emu: *mut snd_emux);
    fn snd_seq_event_port_attach(
        client: i32,
        callback: *mut snd_seq_port_callback,
        cap: u32,
        type_: u32,
        max_channels: i32,
        max_voices: i32,
        name: *const i8,
    ) -> i32;
    fn snd_emux_reset_port(port: *mut snd_emux_port);
    fn snd_emux_sounds_off_all(port: *mut snd_emux_port);
    fn snd_midi_channel_set_clear(chset: *mut snd_midi_channel_set);
    fn snd_midi_process_event(
        ops: *const snd_midi_op,
        ev: *mut snd_seq_event,
        chset: *mut snd_midi_channel_set,
    );
    fn dev_err(dev: *mut core::ffi::c_void, format: *const i8, ...);
    fn dev_warn(dev: *mut core::ffi::c_void, format: *const i8, ...);
    fn try_module_get(module: *mut core::ffi::c_void) -> i32;
    fn module_put(module: *mut core::ffi::c_void);
    fn snd_virmidi_new(
        card: *mut snd_card,
        device: i32,
        rmidi: *mut *mut snd_rawmidi,
    ) -> i32;
    fn snd_device_register(card: *mut snd_card, device: *mut core::ffi::c_void) -> i32;
    fn snd_device_free(card: *mut snd_card, device: *mut core::ffi::c_void) -> i32;
    fn sprintf(s: *mut i8, format: *const i8, ...) -> i32;
    fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    fn kfree(p: *const core::ffi::c_void);
    fn kzalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn snd_emux_create_effect(port: *mut snd_emux_port);
    fn snd_emux_delete_effect(port: *mut snd_emux_port);
    fn snd_emux_clear_effect(port: *mut snd_emux_port);
    fn snd_emux_event_input(
        ev: *mut snd_seq_event,
        direct: i32,
        private_data: *mut core::ffi::c_void,
        atomic: i32,
        hop: i32,
    ) -> i32;
    fn snd_emux_note_on(chan: *mut snd_midi_channel, note: i32, velocity: i32);
    fn snd_emux_note_off(chan: *mut snd_midi_channel, note: i32, velocity: i32);
    fn snd_emux_key_press(chan: *mut snd_midi_channel, note: i32, pressure: i32);
    fn snd_emux_terminate_note(chan: *mut snd_midi_channel, note: i32);
    fn snd_emux_control(chan: *mut snd_midi_channel, control: i32, value: i32);
    fn snd_emux_nrpn(chan: *mut snd_midi_channel, ev: *mut snd_seq_event);
    fn snd_emux_sysex(chan: *mut snd_midi_channel, data: *mut u8, len: i32);

    static THIS_MODULE: *mut core::ffi::c_void;

    static mut SNDRV_EMUX_MAX_PORTS: i32;
}

// External structures with fields not fully defined
#[repr(C)]
pub struct snd_rawmidi {
    pub private_data: *mut core::ffi::c_void,
    pub name: [i8; 80],
}

#[repr(C)]
pub struct snd_virmidi_dev {
    pub seq_mode: u32,
    pub client: i32,
    pub port: i32,
}

// Initialise the EMUX Synth by creating a client and registering
// a series of ports.
// Each of the ports will contain the 16 midi channels.  Applications
// can connect to these ports to play midi data.
pub unsafe extern "C" fn snd_emux_init_seq(
    emu: *mut snd_emux,
    card: *mut snd_card,
    index: i32,
) -> i32 {
    let mut i: i32;
    let mut pinfo: snd_seq_port_callback;
    let mut tmpname: [i8; 64] = [0; 64];

    (*emu).client = snd_seq_create_kernel_client(card, index, b"%s WaveTable\0".as_ptr() as *const i8, (*emu).name);
    if (*emu).client < 0 {
        dev_err((*card).dev, b"can't create client\n\0".as_ptr() as *const i8);
        return -19; // -ENODEV
    }

    if (*emu).num_ports <= 0 {
        dev_warn((*card).dev, b"seqports must be greater than zero\n\0".as_ptr() as *const i8);
        (*emu).num_ports = 1;
    } else if (*emu).num_ports > SNDRV_EMUX_MAX_PORTS {
        dev_warn(
            (*card).dev,
            b"too many ports. limited max. ports %d\n\0".as_ptr() as *const i8,
            SNDRV_EMUX_MAX_PORTS,
        );
        (*emu).num_ports = SNDRV_EMUX_MAX_PORTS;
    }

    memset(&mut pinfo as *mut _ as *mut core::ffi::c_void, 0, core::mem::size_of::<snd_seq_port_callback>());
    pinfo.owner = THIS_MODULE;
    pinfo.use_fn = Some(snd_emux_use);
    pinfo.unuse = Some(snd_emux_unuse);
    pinfo.event_input = Some(snd_emux_event_input);

    i = 0;
    while i < (*emu).num_ports {
        let mut p: *mut snd_emux_port;

        sprintf(
            tmpname.as_mut_ptr(),
            b"%s Port %d\0".as_ptr() as *const i8,
            (*emu).name,
            i,
        );
        p = snd_emux_create_port(emu, tmpname.as_ptr(), MIDI_CHANNELS as i32, 0, &mut pinfo);
        if p.is_null() {
            dev_err((*card).dev, b"can't create port\n\0".as_ptr() as *const i8);
            return -12; // -ENOMEM
        }

        (*p).port_mode = SNDRV_EMUX_PORT_MODE_MIDI;
        snd_emux_init_port(p);
        (*emu).ports[i as usize] = (*p).chset.port;
        (*emu).portptrs[i as usize] = p;

        i += 1;
    }

    0
}

// Detach from the ports that were set up for this synthesizer and
// destroy the kernel client.
pub unsafe extern "C" fn snd_emux_detach_seq(emu: *mut snd_emux) {
    if !(*emu).voices.is_null() {
        snd_emux_terminate_all(emu);
    }

    if (*emu).client >= 0 {
        snd_seq_delete_kernel_client((*emu).client);
        (*emu).client = -1;
    }
}

// create a sequencer port and channel_set
pub unsafe extern "C" fn snd_emux_create_port(
    emu: *mut snd_emux,
    name: *const i8,
    max_channels: i32,
    oss_port: i32,
    callback: *mut snd_seq_port_callback,
) -> *mut snd_emux_port {
    let mut p: *mut snd_emux_port;
    let mut i: i32;
    let mut type_: u32;
    let mut cap: u32;

    // Allocate structures for this channel using kzalloc
    // kzalloc_flex(*p, chset.channels, max_channels) is equivalent to
    // allocating p with flexible array member chset.channels[max_channels]
    let alloc_size = core::mem::size_of::<snd_emux_port>()
        + (max_channels as usize) * core::mem::size_of::<snd_midi_channel>();
    p = kzalloc(alloc_size, 0xd0) as *mut snd_emux_port; // GFP_KERNEL = 0xd0
    if p.is_null() {
        return core::ptr::null_mut();
    }

    (*p).chset.max_channels = max_channels;

    i = 0;
    while i < max_channels {
        (*p).chset.channels[i as usize].number = i;
        i += 1;
    }
    (*p).chset.private_data = p as *mut core::ffi::c_void;
    (*p).emu = emu;
    (*p).chset.client = (*emu).client;

    // SNDRV_EMUX_USE_RAW_EFFECT is conditionally compiled
    // snd_emux_create_effect(p);

    (*callback).private_free = Some(free_port);
    (*callback).private_data = p as *mut core::ffi::c_void;

    cap = SNDRV_SEQ_PORT_CAP_WRITE;
    if oss_port != 0 {
        type_ = SNDRV_SEQ_PORT_TYPE_SPECIFIC;
    } else {
        type_ = DEFAULT_MIDI_TYPE;
        cap |= SNDRV_SEQ_PORT_CAP_SUBS_WRITE;
    }

    (*p).chset.port = snd_seq_event_port_attach(
        (*emu).client,
        callback,
        cap,
        type_,
        max_channels,
        (*emu).max_voices,
        name,
    );

    p
}

// release memory block for port
unsafe extern "C" fn free_port(private_data: *mut core::ffi::c_void) {
    let p = private_data as *mut snd_emux_port;

    if !p.is_null() {
        // SNDRV_EMUX_USE_RAW_EFFECT is conditionally compiled
        // snd_emux_delete_effect(p);
        kfree(p as *const core::ffi::c_void);
    }
}

// initialize the port specific parameters
unsafe extern "C" fn snd_emux_init_port(p: *mut snd_emux_port) {
    (*p).drum_flags = DEFAULT_DRUM_FLAGS;
    (*p).volume_atten = 0;

    snd_emux_reset_port(p);
}

// reset port
pub unsafe extern "C" fn snd_emux_reset_port(port: *mut snd_emux_port) {
    let mut i: i32;

    // stop all sounds
    snd_emux_sounds_off_all(port);

    snd_midi_channel_set_clear(&mut (*port).chset);

    // SNDRV_EMUX_USE_RAW_EFFECT is conditionally compiled
    // snd_emux_clear_effect(port);

    // set port specific control parameters
    (*port).ctrls[EMUX_MD_DEF_BANK] = 0;
    (*port).ctrls[EMUX_MD_DEF_DRUM] = 0;
    (*port).ctrls[EMUX_MD_REALTIME_PAN] = 1;

    i = 0;
    while i < (*port).chset.max_channels {
        let chan = &mut (*port).chset.channels[i as usize];
        chan.drum_channel = if ((((*port).drum_flags >> i) & 1) != 0) { 1 } else { 0 };
        i += 1;
    }
}

// input sequencer event
pub unsafe extern "C" fn snd_emux_event_input(
    ev: *mut snd_seq_event,
    direct: i32,
    private_data: *mut core::ffi::c_void,
    atomic: i32,
    hop: i32,
) -> i32 {
    let port = private_data as *mut snd_emux_port;

    if port.is_null() || ev.is_null() {
        return -22; // -EINVAL
    }

    snd_midi_process_event(&EMUX_OPS, ev, &mut (*port).chset);

    0
}

// increment usage count
unsafe fn __snd_emux_inc_count(emu: *mut snd_emux) -> i32 {
    (*emu).used += 1;
    if try_module_get((*(*emu).ops.owner)) == 0 {
        // goto __error;
    } else if try_module_get((*(*emu).card).module) == 0 {
        module_put((*(*emu).ops.owner));
        (*emu).used -= 1;
        return 0;
    } else {
        return 1;
    }
    (*emu).used -= 1;
    0
}

pub unsafe extern "C" fn snd_emux_inc_count(emu: *mut snd_emux) -> i32 {
    // guard(mutex)(&emu->register_mutex); - requires mutex locking mechanism
    // For now, translate directly without the guard syntax
    let result = __snd_emux_inc_count(emu);
    result
}

// decrease usage count
unsafe fn __snd_emux_dec_count(emu: *mut snd_emux) {
    module_put((*(*emu).card).module);
    (*emu).used -= 1;
    if (*emu).used <= 0 {
        snd_emux_terminate_all(emu);
    }
    module_put((*(*emu).ops.owner));
}

pub unsafe extern "C" fn snd_emux_dec_count(emu: *mut snd_emux) {
    // guard(mutex)(&emu->register_mutex); - requires mutex locking mechanism
    // For now, translate directly without the guard syntax
    __snd_emux_dec_count(emu);
}

// Routine that is called upon a first use of a particular port
unsafe extern "C" fn snd_emux_use(
    private_data: *mut core::ffi::c_void,
    info: *mut snd_seq_port_subscribe,
) -> i32 {
    let p = private_data as *mut snd_emux_port;
    let mut emu: *mut snd_emux;

    if p.is_null() {
        return -22; // -EINVAL
    }
    emu = (*p).emu;
    if emu.is_null() {
        return -22; // -EINVAL
    }

    // guard(mutex)(&emu->register_mutex);
    snd_emux_init_port(p);
    __snd_emux_inc_count(emu);
    0
}

// Routine that is called upon the last unuse() of a particular port.
unsafe extern "C" fn snd_emux_unuse(
    private_data: *mut core::ffi::c_void,
    info: *mut snd_seq_port_subscribe,
) -> i32 {
    let p = private_data as *mut snd_emux_port;
    let mut emu: *mut snd_emux;

    if p.is_null() {
        return -22; // -EINVAL
    }
    emu = (*p).emu;
    if emu.is_null() {
        return -22; // -EINVAL
    }

    // guard(mutex)(&emu->register_mutex);
    snd_emux_sounds_off_all(p);
    __snd_emux_dec_count(emu);
    0
}

// attach virtual rawmidi devices
pub unsafe extern "C" fn snd_emux_init_virmidi(emu: *mut snd_emux, card: *mut snd_card) -> i32 {
    let mut i: i32;

    (*emu).vmidi = core::ptr::null_mut();
    if (*emu).midi_ports <= 0 {
        return 0;
    }

    // kzalloc_objs(*emu->vmidi, emu->midi_ports) equivalent
    let alloc_size = ((*emu).midi_ports as usize) * core::mem::size_of::<*mut snd_rawmidi>();
    (*emu).vmidi = kzalloc(alloc_size, 0xd0) as *mut *mut snd_rawmidi;
    if (*emu).vmidi.is_null() {
        return -12; // -ENOMEM
    }

    i = 0;
    while i < (*emu).midi_ports {
        let mut rmidi: *mut snd_rawmidi = core::ptr::null_mut();
        let mut rdev: *mut snd_virmidi_dev;
        if snd_virmidi_new(card, (*emu).midi_devidx + i, &mut rmidi) < 0 {
            // goto __error;
            snd_emux_delete_virmidi(emu);
            return -12; // -ENOMEM
        }
        rdev = (*rmidi).private_data as *mut snd_virmidi_dev;
        sprintf(
            (*rmidi).name.as_mut_ptr(),
            b"%s Synth MIDI\0".as_ptr() as *const i8,
            (*emu).name,
        );
        (*rdev).seq_mode = 1; // SNDRV_VIRMIDI_SEQ_ATTACH = 1
        (*rdev).client = (*emu).client;
        (*rdev).port = (*emu).ports[i as usize];
        if snd_device_register(card, rmidi as *mut core::ffi::c_void) < 0 {
            snd_device_free(card, rmidi as *mut core::ffi::c_void);
            // goto __error;
            snd_emux_delete_virmidi(emu);
            return -12; // -ENOMEM
        }
        *(*emu).vmidi.add(i as usize) = rmidi;

        i += 1;
    }
    0
}

pub unsafe extern "C" fn snd_emux_delete_virmidi(emu: *mut snd_emux) -> i32 {
    let mut i: i32;

    if (*emu).vmidi.is_null() {
        return 0;
    }

    i = 0;
    while i < (*emu).midi_ports {
        if !(*(*emu).vmidi.add(i as usize)).is_null() {
            snd_device_free((*emu).card, *(*emu).vmidi.add(i as usize) as *mut core::ffi::c_void);
        }
        i += 1;
    }
    kfree((*emu).vmidi as *const core::ffi::c_void);
    (*emu).vmidi = core::ptr::null_mut();
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
