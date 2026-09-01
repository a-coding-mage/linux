// SPDX-License-Identifier: GPL-2.0-only
/*
 * Apple Onboard Audio driver -- layout/machine id fabric
 *
 * Copyright 2006-2008 Johannes Berg <johannes@sipsolutions.net>
 *
 * This fabric module looks for sound codecs based on the
 * layout-id or device-id property in the device tree.
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

const MAX_CODECS_PER_BUS: usize = 2;

/* These are the connections the layout fabric
 * knows about. It doesn't really care about the
 * input ones, but I thought I'd separate them
 * to give them proper names. The thing is that
 * Apple usually will distinguish the active output
 * by GPIOs, while the active input is set directly
 * on the codec. Hence we here tell the codec what
 * we think is connected. This information is hard-
 * coded below ... */
const CC_SPEAKERS: c_int = 1 << 0;
const CC_HEADPHONE: c_int = 1 << 1;
const CC_LINEOUT: c_int = 1 << 2;
const CC_DIGITALOUT: c_int = 1 << 3;
const CC_LINEIN: c_int = 1 << 4;
const CC_MICROPHONE: c_int = 1 << 5;
const CC_DIGITALIN: c_int = 1 << 6;
/* pretty bogus but users complain...
 * This is a flag saying that the LINEOUT
 * should be renamed to HEADPHONE.
 * be careful with input detection! */
const CC_LINEOUT_LABELLED_HEADPHONE: c_int = 1 << 7;

#[repr(C)]
struct codec_connection {
    /* CC_ flags from above */
    connected: c_int,
    /* codec dependent bit to be set in the aoa_codec.connected field.
     * This intentionally doesn't have any generic flags because the
     * fabric has to know the codec anyway and all codecs might have
     * different connectors */
    codec_bit: c_int,
}

#[repr(C)]
struct codec_connect_info {
    name: *mut c_char,
    connections: *const codec_connection,
}

const LAYOUT_FLAG_COMBO_LINEOUT_SPDIF: c_int = 1 << 0;

#[repr(C)]
struct layout {
    layout_id: c_uint,
    device_id: c_uint,
    codecs: [codec_connect_info; MAX_CODECS_PER_BUS],
    flags: c_int,

    /* if busname is not assigned, we use 'Master' below,
     * so that our layout table doesn't need to be filled
     * too much.
     * We only assign these two if we expect to find more
     * than one soundbus, i.e. on those machines with
     * multiple layout-ids */
    busname: *mut c_char,
    pcmid: c_int,
}

const fn cci(name: *mut c_char, connections: *const codec_connection) -> codec_connect_info {
    codec_connect_info { name, connections }
}

const fn layout_entry(
    layout_id: c_uint,
    device_id: c_uint,
    codec0: codec_connect_info,
    codec1: codec_connect_info,
    flags: c_int,
    busname: *mut c_char,
    pcmid: c_int,
) -> layout {
    layout {
        layout_id,
        device_id,
        codecs: [codec0, codec1],
        flags,
        busname,
        pcmid,
    }
}

const fn null_cci() -> codec_connect_info {
    codec_connect_info {
        name: ptr::null_mut(),
        connections: ptr::null(),
    }
}

/* MODULE_AUTHOR("Johannes Berg <johannes@sipsolutions.net>");
 * MODULE_LICENSE("GPL");
 * MODULE_DESCRIPTION("Layout-ID fabric for snd-aoa");
 *
 * MODULE_ALIAS entries:
 * sound-layout-36, 41, 45, 47, 48, 49, 50, 51, 56, 57, 58, 60, 61, 62, 64,
 * 65, 66, 67, 68, 69, 70, 72, 76, 80, 82, 84, 86, 90, 92, 94, 96, 98, 100
 * aoa-device-id-14, 22, 31, 35, 44
 */

/* onyx with all but microphone connected */
static onyx_connections_nomic: [codec_connection; 4] = [
    codec_connection { connected: CC_SPEAKERS | CC_HEADPHONE | CC_LINEOUT, codec_bit: 0 },
    codec_connection { connected: CC_DIGITALOUT, codec_bit: 1 },
    codec_connection { connected: CC_LINEIN, codec_bit: 2 },
    codec_connection { connected: 0, codec_bit: 0 }, /* terminate array by .connected == 0 */
];

/* onyx on machines without headphone */
static onyx_connections_noheadphones: [codec_connection; 5] = [
    codec_connection {
        connected: CC_SPEAKERS | CC_LINEOUT | CC_LINEOUT_LABELLED_HEADPHONE,
        codec_bit: 0,
    },
    codec_connection { connected: CC_DIGITALOUT, codec_bit: 1 },
    /* FIXME: are these correct? probably not for all the machines
     * below ... If not this will need separating. */
    codec_connection { connected: CC_LINEIN, codec_bit: 2 },
    codec_connection { connected: CC_MICROPHONE, codec_bit: 3 },
    codec_connection { connected: 0, codec_bit: 0 }, /* terminate array by .connected == 0 */
];

/* onyx on machines with real line-out */
static onyx_connections_reallineout: [codec_connection; 4] = [
    codec_connection { connected: CC_SPEAKERS | CC_LINEOUT | CC_HEADPHONE, codec_bit: 0 },
    codec_connection { connected: CC_DIGITALOUT, codec_bit: 1 },
    codec_connection { connected: CC_LINEIN, codec_bit: 2 },
    codec_connection { connected: 0, codec_bit: 0 }, /* terminate array by .connected == 0 */
];

/* tas on machines without line out */
static tas_connections_nolineout: [codec_connection; 4] = [
    codec_connection { connected: CC_SPEAKERS | CC_HEADPHONE, codec_bit: 0 },
    codec_connection { connected: CC_LINEIN, codec_bit: 2 },
    codec_connection { connected: CC_MICROPHONE, codec_bit: 3 },
    codec_connection { connected: 0, codec_bit: 0 }, /* terminate array by .connected == 0 */
];

/* tas on machines with neither line out nor line in */
static tas_connections_noline: [codec_connection; 3] = [
    codec_connection { connected: CC_SPEAKERS | CC_HEADPHONE, codec_bit: 0 },
    codec_connection { connected: CC_MICROPHONE, codec_bit: 3 },
    codec_connection { connected: 0, codec_bit: 0 }, /* terminate array by .connected == 0 */
];

/* tas on machines without microphone */
static tas_connections_nomic: [codec_connection; 3] = [
    codec_connection { connected: CC_SPEAKERS | CC_HEADPHONE | CC_LINEOUT, codec_bit: 0 },
    codec_connection { connected: CC_LINEIN, codec_bit: 2 },
    codec_connection { connected: 0, codec_bit: 0 }, /* terminate array by .connected == 0 */
];

/* tas on machines with everything connected */
static tas_connections_all: [codec_connection; 4] = [
    codec_connection { connected: CC_SPEAKERS | CC_HEADPHONE | CC_LINEOUT, codec_bit: 0 },
    codec_connection { connected: CC_LINEIN, codec_bit: 2 },
    codec_connection { connected: CC_MICROPHONE, codec_bit: 3 },
    codec_connection { connected: 0, codec_bit: 0 }, /* terminate array by .connected == 0 */
];

static toonie_connections: [codec_connection; 2] = [
    codec_connection { connected: CC_SPEAKERS | CC_HEADPHONE, codec_bit: 0 },
    codec_connection { connected: 0, codec_bit: 0 }, /* terminate array by .connected == 0 */
];

static topaz_input: [codec_connection; 2] = [
    codec_connection { connected: CC_DIGITALIN, codec_bit: 0 },
    codec_connection { connected: 0, codec_bit: 0 }, /* terminate array by .connected == 0 */
];

static topaz_output: [codec_connection; 2] = [
    codec_connection { connected: CC_DIGITALOUT, codec_bit: 1 },
    codec_connection { connected: 0, codec_bit: 0 }, /* terminate array by .connected == 0 */
];

static topaz_inout: [codec_connection; 3] = [
    codec_connection { connected: CC_DIGITALIN, codec_bit: 0 },
    codec_connection { connected: CC_DIGITALOUT, codec_bit: 1 },
    codec_connection { connected: 0, codec_bit: 0 }, /* terminate array by .connected == 0 */
];

static mut layouts: [layout; 38] = [
    /* last PowerBooks (15" Oct 2005) */
    layout_entry(82, 0, cci(b"onyx\0".as_ptr() as *mut c_char, onyx_connections_noheadphones.as_ptr()), cci(b"topaz\0".as_ptr() as *mut c_char, topaz_input.as_ptr()), LAYOUT_FLAG_COMBO_LINEOUT_SPDIF, ptr::null_mut(), 0),
    /* PowerMac9,1 */
    layout_entry(60, 0, cci(b"onyx\0".as_ptr() as *mut c_char, onyx_connections_reallineout.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    /* PowerMac9,1 */
    layout_entry(61, 0, cci(b"topaz\0".as_ptr() as *mut c_char, topaz_input.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    /* PowerBook5,7 */
    layout_entry(64, 0, cci(b"onyx\0".as_ptr() as *mut c_char, onyx_connections_noheadphones.as_ptr()), null_cci(), LAYOUT_FLAG_COMBO_LINEOUT_SPDIF, ptr::null_mut(), 0),
    /* PowerBook5,7 */
    layout_entry(65, 0, cci(b"topaz\0".as_ptr() as *mut c_char, topaz_input.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    /* PowerBook5,9 [17" Oct 2005] */
    layout_entry(84, 0, cci(b"onyx\0".as_ptr() as *mut c_char, onyx_connections_noheadphones.as_ptr()), cci(b"topaz\0".as_ptr() as *mut c_char, topaz_input.as_ptr()), LAYOUT_FLAG_COMBO_LINEOUT_SPDIF, ptr::null_mut(), 0),
    /* PowerMac8,1 */
    layout_entry(45, 0, cci(b"onyx\0".as_ptr() as *mut c_char, onyx_connections_noheadphones.as_ptr()), cci(b"topaz\0".as_ptr() as *mut c_char, topaz_input.as_ptr()), 0, ptr::null_mut(), 0),
    /* Quad PowerMac (analog in, analog/digital out) */
    layout_entry(68, 0, cci(b"onyx\0".as_ptr() as *mut c_char, onyx_connections_nomic.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    /* Quad PowerMac (digital in) */
    layout_entry(69, 0, cci(b"topaz\0".as_ptr() as *mut c_char, topaz_input.as_ptr()), null_cci(), 0, b"digital in\0".as_ptr() as *mut c_char, 1),
    /* Early 2005 PowerBook (PowerBook 5,6) */
    layout_entry(70, 0, cci(b"tas\0".as_ptr() as *mut c_char, tas_connections_nolineout.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    /* PowerBook 5,4 */
    layout_entry(51, 0, cci(b"tas\0".as_ptr() as *mut c_char, tas_connections_nolineout.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    /* PowerBook6,1 */
    layout_entry(0, 31, cci(b"tas\0".as_ptr() as *mut c_char, tas_connections_nolineout.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    /* PowerBook6,5 */
    layout_entry(0, 44, cci(b"tas\0".as_ptr() as *mut c_char, tas_connections_all.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    /* PowerBook6,7 */
    layout_entry(80, 0, cci(b"tas\0".as_ptr() as *mut c_char, tas_connections_noline.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    /* PowerBook6,8 */
    layout_entry(72, 0, cci(b"tas\0".as_ptr() as *mut c_char, tas_connections_nolineout.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    /* PowerMac8,2 */
    layout_entry(86, 0, cci(b"onyx\0".as_ptr() as *mut c_char, onyx_connections_nomic.as_ptr()), cci(b"topaz\0".as_ptr() as *mut c_char, topaz_input.as_ptr()), 0, ptr::null_mut(), 0),
    /* PowerBook6,7 */
    layout_entry(92, 0, cci(b"tas\0".as_ptr() as *mut c_char, tas_connections_nolineout.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    /* PowerMac10,1 (Mac Mini) */
    layout_entry(58, 0, cci(b"toonie\0".as_ptr() as *mut c_char, toonie_connections.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    layout_entry(96, 0, cci(b"onyx\0".as_ptr() as *mut c_char, onyx_connections_noheadphones.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    /* unknown, untested, but this comes from Apple */
    layout_entry(41, 0, cci(b"tas\0".as_ptr() as *mut c_char, tas_connections_all.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    layout_entry(36, 0, cci(b"tas\0".as_ptr() as *mut c_char, tas_connections_nomic.as_ptr()), cci(b"topaz\0".as_ptr() as *mut c_char, topaz_inout.as_ptr()), 0, ptr::null_mut(), 0),
    layout_entry(47, 0, cci(b"onyx\0".as_ptr() as *mut c_char, onyx_connections_noheadphones.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    layout_entry(48, 0, cci(b"topaz\0".as_ptr() as *mut c_char, topaz_input.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    layout_entry(49, 0, cci(b"onyx\0".as_ptr() as *mut c_char, onyx_connections_nomic.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    layout_entry(50, 0, cci(b"topaz\0".as_ptr() as *mut c_char, topaz_input.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    layout_entry(56, 0, cci(b"onyx\0".as_ptr() as *mut c_char, onyx_connections_noheadphones.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    layout_entry(57, 0, cci(b"topaz\0".as_ptr() as *mut c_char, topaz_input.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    layout_entry(62, 0, cci(b"onyx\0".as_ptr() as *mut c_char, onyx_connections_noheadphones.as_ptr()), cci(b"topaz\0".as_ptr() as *mut c_char, topaz_output.as_ptr()), 0, ptr::null_mut(), 0),
    layout_entry(66, 0, cci(b"onyx\0".as_ptr() as *mut c_char, onyx_connections_noheadphones.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    layout_entry(67, 0, cci(b"topaz\0".as_ptr() as *mut c_char, topaz_input.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    layout_entry(76, 0, cci(b"tas\0".as_ptr() as *mut c_char, tas_connections_nomic.as_ptr()), cci(b"topaz\0".as_ptr() as *mut c_char, topaz_inout.as_ptr()), 0, ptr::null_mut(), 0),
    layout_entry(90, 0, cci(b"tas\0".as_ptr() as *mut c_char, tas_connections_noline.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    layout_entry(94, 0, cci(b"onyx\0".as_ptr() as *mut c_char, onyx_connections_noheadphones.as_ptr()), null_cci(), 0, ptr::null_mut(), 0), /* but it has an external mic?? how to select? */
    layout_entry(98, 0, cci(b"toonie\0".as_ptr() as *mut c_char, toonie_connections.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    layout_entry(100, 0, cci(b"topaz\0".as_ptr() as *mut c_char, topaz_input.as_ptr()), cci(b"onyx\0".as_ptr() as *mut c_char, onyx_connections_noheadphones.as_ptr()), 0, ptr::null_mut(), 0),
    /* PowerMac3,4 */
    layout_entry(0, 14, cci(b"tas\0".as_ptr() as *mut c_char, tas_connections_noline.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    /* PowerMac3,6 */
    layout_entry(0, 22, cci(b"tas\0".as_ptr() as *mut c_char, tas_connections_all.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    /* PowerBook5,2 */
    layout_entry(0, 35, cci(b"tas\0".as_ptr() as *mut c_char, tas_connections_all.as_ptr()), null_cci(), 0, ptr::null_mut(), 0),
    layout_entry(0, 0, null_cci(), null_cci(), 0, ptr::null_mut(), 0),
];

unsafe fn find_layout_by_id(id: c_uint) -> *mut layout {
    let mut l = layouts.as_mut_ptr();

    while !(*l).codecs[0].name.is_null() {
        if (*l).layout_id == id {
            return l;
        }
        l = l.add(1);
    }
    ptr::null_mut()
}

unsafe fn find_layout_by_device(id: c_uint) -> *mut layout {
    let mut l = layouts.as_mut_ptr();

    while !(*l).codecs[0].name.is_null() {
        if (*l).device_id == id {
            return l;
        }
        l = l.add(1);
    }
    ptr::null_mut()
}

unsafe fn use_layout(l: *mut layout) {
    let mut i = 0;

    while i < MAX_CODECS_PER_BUS {
        if !(*l).codecs[i].name.is_null() {
            request_module(b"snd-aoa-codec-%s\0".as_ptr() as *const c_char, (*l).codecs[i].name);
        }
        i += 1;
    }
    /* now we wait for the codecs to call us back */
}

#[repr(C)]
struct layout_dev {
    list: list_head,
    sdev: *mut soundbus_dev,
    sound: *mut device_node,
    codecs: [*mut aoa_codec; MAX_CODECS_PER_BUS],
    layout: *mut layout,
    gpio: gpio_runtime,

    /* we need these for headphone/lineout detection */
    headphone_ctrl: *mut snd_kcontrol,
    lineout_ctrl: *mut snd_kcontrol,
    speaker_ctrl: *mut snd_kcontrol,
    master_ctrl: *mut snd_kcontrol,
    headphone_detected_ctrl: *mut snd_kcontrol,
    lineout_detected_ctrl: *mut snd_kcontrol,

    selfptr_headphone: layout_dev_ptr,
    selfptr_lineout: layout_dev_ptr,

    have_lineout_detect: u32,
    have_headphone_detect: u32,
    switch_on_headphone: u32,
    switch_on_lineout: u32,
}

#[repr(C)]
struct layout_dev_ptr {
    ptr: *mut layout_dev,
}

static mut layouts_list: list_head = list_head {
    next: ptr::null_mut(),
    prev: ptr::null_mut(),
};
static mut layouts_list_items: c_int = 0;
/* this can go away but only if we allow multiple cards,
 * make the fabric handle all the card stuff, etc... */
static mut layout_device: *mut layout_dev = ptr::null_mut();

type control_info = snd_ctl_boolean_mono_info;

unsafe extern "C" fn headphone_control_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let gpio = snd_kcontrol_chip(kcontrol) as *mut gpio_runtime;
    if !(*gpio).methods.is_null() && !(*(*gpio).methods).get_headphone.is_none() {
        (*ucontrol).value.integer.value[0] = (*(*gpio).methods).get_headphone.unwrap()(gpio) as c_long;
    }
    0
}

unsafe extern "C" fn headphone_control_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let gpio = snd_kcontrol_chip(kcontrol) as *mut gpio_runtime;
    if !(*gpio).methods.is_null() && !(*(*gpio).methods).set_headphone.is_none() {
        (*(*gpio).methods).set_headphone.unwrap()(gpio, ((*ucontrol).value.integer.value[0] != 0) as c_int);
    }
    1
}

unsafe extern "C" fn speakers_control_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let gpio = snd_kcontrol_chip(kcontrol) as *mut gpio_runtime;
    if !(*gpio).methods.is_null() && !(*(*gpio).methods).get_speakers.is_none() {
        (*ucontrol).value.integer.value[0] = (*(*gpio).methods).get_speakers.unwrap()(gpio) as c_long;
    }
    0
}

unsafe extern "C" fn speakers_control_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let gpio = snd_kcontrol_chip(kcontrol) as *mut gpio_runtime;
    if !(*gpio).methods.is_null() && !(*(*gpio).methods).set_speakers.is_none() {
        (*(*gpio).methods).set_speakers.unwrap()(gpio, ((*ucontrol).value.integer.value[0] != 0) as c_int);
    }
    1
}

unsafe extern "C" fn lineout_control_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let gpio = snd_kcontrol_chip(kcontrol) as *mut gpio_runtime;
    if !(*gpio).methods.is_null() && !(*(*gpio).methods).get_lineout.is_none() {
        (*ucontrol).value.integer.value[0] = (*(*gpio).methods).get_lineout.unwrap()(gpio) as c_long;
    }
    0
}

unsafe extern "C" fn lineout_control_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let gpio = snd_kcontrol_chip(kcontrol) as *mut gpio_runtime;
    if !(*gpio).methods.is_null() && !(*(*gpio).methods).set_lineout.is_none() {
        (*(*gpio).methods).set_lineout.unwrap()(gpio, ((*ucontrol).value.integer.value[0] != 0) as c_int);
    }
    1
}

unsafe extern "C" fn master_control_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let gpio = snd_kcontrol_chip(kcontrol) as *mut gpio_runtime;
    if !(*gpio).methods.is_null() && !(*(*gpio).methods).get_master.is_none() {
        (*ucontrol).value.integer.value[0] = (*(*gpio).methods).get_master.unwrap()(gpio) as c_long;
    }
    0
}

unsafe extern "C" fn master_control_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let gpio = snd_kcontrol_chip(kcontrol) as *mut gpio_runtime;
    if !(*gpio).methods.is_null() && !(*(*gpio).methods).set_master.is_none() {
        (*(*gpio).methods).set_master.unwrap()(gpio, ((*ucontrol).value.integer.value[0] != 0) as c_int);
    }
    1
}

static headphone_ctl: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Headphone Switch\0".as_ptr() as *const c_char,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(snd_ctl_boolean_mono_info),
    get: Some(headphone_control_get),
    put: Some(headphone_control_put),
    private_value: 0,
};

static speakers_ctl: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Speakers Switch\0".as_ptr() as *const c_char,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(snd_ctl_boolean_mono_info),
    get: Some(speakers_control_get),
    put: Some(speakers_control_put),
    private_value: 0,
};

static lineout_ctl: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Line-Out Switch\0".as_ptr() as *const c_char,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(snd_ctl_boolean_mono_info),
    get: Some(lineout_control_get),
    put: Some(lineout_control_put),
    private_value: 0,
};

static master_ctl: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Master Switch\0".as_ptr() as *const c_char,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(snd_ctl_boolean_mono_info),
    get: Some(master_control_get),
    put: Some(master_control_put),
    private_value: 0,
};

unsafe extern "C" fn detect_choice_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ldev = snd_kcontrol_chip(kcontrol) as *mut layout_dev;

    match (*kcontrol).private_value {
        0 => (*ucontrol).value.integer.value[0] = (*ldev).switch_on_headphone as c_long,
        1 => (*ucontrol).value.integer.value[0] = (*ldev).switch_on_lineout as c_long,
        _ => return -ENODEV,
    }
    0
}

unsafe extern "C" fn detect_choice_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ldev = snd_kcontrol_chip(kcontrol) as *mut layout_dev;

    match (*kcontrol).private_value {
        0 => (*ldev).switch_on_headphone = ((*ucontrol).value.integer.value[0] != 0) as u32,
        1 => (*ldev).switch_on_lineout = ((*ucontrol).value.integer.value[0] != 0) as u32,
        _ => return -ENODEV,
    }
    1
}

static headphone_detect_choice: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Headphone Detect Autoswitch\0".as_ptr() as *const c_char,
    info: Some(snd_ctl_boolean_mono_info),
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    get: Some(detect_choice_get),
    put: Some(detect_choice_put),
    private_value: 0,
};

static lineout_detect_choice: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Line-Out Detect Autoswitch\0".as_ptr() as *const c_char,
    info: Some(snd_ctl_boolean_mono_info),
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    get: Some(detect_choice_get),
    put: Some(detect_choice_put),
    private_value: 1,
};

unsafe extern "C" fn detected_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ldev = snd_kcontrol_chip(kcontrol) as *mut layout_dev;
    let v: c_int;

    match (*kcontrol).private_value {
        0 => v = (*(*(*ldev).gpio.methods).get_detect.unwrap())(&mut (*ldev).gpio, AOA_NOTIFY_HEADPHONE),
        1 => v = (*(*(*ldev).gpio.methods).get_detect.unwrap())(&mut (*ldev).gpio, AOA_NOTIFY_LINE_OUT),
        _ => return -ENODEV,
    }
    (*ucontrol).value.integer.value[0] = v as c_long;
    0
}

static headphone_detected: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Headphone Detected\0".as_ptr() as *const c_char,
    info: Some(snd_ctl_boolean_mono_info),
    access: SNDRV_CTL_ELEM_ACCESS_READ,
    get: Some(detected_get),
    put: None,
    private_value: 0,
};

static lineout_detected: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Line-Out Detected\0".as_ptr() as *const c_char,
    info: Some(snd_ctl_boolean_mono_info),
    access: SNDRV_CTL_ELEM_ACCESS_READ,
    get: Some(detected_get),
    put: None,
    private_value: 1,
};

unsafe fn check_codec(codec: *mut aoa_codec, ldev: *mut layout_dev, cci: *mut codec_connect_info) -> c_int {
    let mut ref_: *const u32;
    let mut propname = [0 as c_char; 32];
    let mut cc: *const codec_connection;

    /* if the codec has a 'codec' node, we require a reference */
    if of_node_name_eq((*codec).node, b"codec\0".as_ptr() as *const c_char) != 0 {
        snprintf(
            propname.as_mut_ptr(),
            propname.len(),
            b"platform-%s-codec-ref\0".as_ptr() as *const c_char,
            (*codec).name,
        );
        ref_ = of_get_property((*ldev).sound, propname.as_ptr(), ptr::null_mut()) as *const u32;
        if ref_.is_null() {
            printk(
                KERN_INFO,
                b"snd-aoa-fabric-layout: required property %s not present\n\0".as_ptr() as *const c_char,
                propname.as_ptr(),
            );
            return -ENODEV;
        }
        if *ref_ != (*(*codec).node).phandle {
            printk(
                KERN_INFO,
                b"snd-aoa-fabric-layout: %s doesn't match!\n\0".as_ptr() as *const c_char,
                propname.as_ptr(),
            );
            return -ENODEV;
        }
    } else if layouts_list_items != 1 {
        printk(
            KERN_INFO,
            b"snd-aoa-fabric-layout: more than one soundbus, but no references.\n\0".as_ptr() as *const c_char,
        );
        return -ENODEV;
    }
    (*codec).soundbus_dev = (*ldev).sdev;
    (*codec).gpio = &mut (*ldev).gpio;

    cc = (*cci).connections;
    if cc.is_null() {
        return -EINVAL;
    }

    printk(KERN_INFO, b"snd-aoa-fabric-layout: can use this codec\n\0".as_ptr() as *const c_char);

    (*codec).connected = 0;
    (*codec).fabric_data = cc as *mut c_void;

    while (*cc).connected != 0 {
        (*codec).connected |= 1 << (*cc).codec_bit;
        cc = cc.add(1);
    }

    0
}

unsafe fn layout_found_codec(codec: *mut aoa_codec) -> c_int {
    let mut ldev: *mut layout_dev;
    let mut i: usize;

    ldev = container_first_layout_dev(&mut layouts_list);
    while !ldev.is_null() {
        i = 0;
        while i < MAX_CODECS_PER_BUS {
            if (*(*ldev).layout).codecs[i].name.is_null() {
                i += 1;
                continue;
            }
            if strcmp((*(*ldev).layout).codecs[i].name, (*codec).name) == 0 {
                if check_codec(codec, ldev, &mut (*(*ldev).layout).codecs[i]) == 0 {
                    return 0;
                }
            }
            i += 1;
        }
        ldev = container_next_layout_dev(ldev, &mut layouts_list);
    }
    -ENODEV
}

unsafe fn layout_remove_codec(codec: *mut aoa_codec) {
    let mut i: c_int;
    /* here remove the codec from the layout dev's
     * codec reference */

    (*codec).soundbus_dev = ptr::null_mut();
    (*codec).gpio = ptr::null_mut();
    i = 0;
    while i < MAX_CODECS_PER_BUS as c_int {
        i += 1;
    }
}

unsafe extern "C" fn layout_notify(data: *mut c_void) {
    let dptr = data as *mut layout_dev_ptr;
    let ldev: *mut layout_dev;
    let v: c_int;
    let update: c_int;
    let detected: *mut snd_kcontrol;
    let mut c: *mut snd_kcontrol;
    let card = aoa_get_card();

    ldev = (*dptr).ptr;
    if data == &mut (*ldev).selfptr_headphone as *mut _ as *mut c_void {
        v = (*(*ldev).gpio.methods).get_detect.unwrap()(&mut (*ldev).gpio, AOA_NOTIFY_HEADPHONE);
        detected = (*ldev).headphone_detected_ctrl;
        update = (*ldev).switch_on_headphone as c_int;
        if update != 0 {
            (*(*ldev).gpio.methods).set_speakers.unwrap()(&mut (*ldev).gpio, (v == 0) as c_int);
            (*(*ldev).gpio.methods).set_headphone.unwrap()(&mut (*ldev).gpio, v);
            (*(*ldev).gpio.methods).set_lineout.unwrap()(&mut (*ldev).gpio, 0);
        }
    } else if data == &mut (*ldev).selfptr_lineout as *mut _ as *mut c_void {
        v = (*(*ldev).gpio.methods).get_detect.unwrap()(&mut (*ldev).gpio, AOA_NOTIFY_LINE_OUT);
        detected = (*ldev).lineout_detected_ctrl;
        update = (*ldev).switch_on_lineout as c_int;
        if update != 0 {
            (*(*ldev).gpio.methods).set_speakers.unwrap()(&mut (*ldev).gpio, (v == 0) as c_int);
            (*(*ldev).gpio.methods).set_headphone.unwrap()(&mut (*ldev).gpio, 0);
            (*(*ldev).gpio.methods).set_lineout.unwrap()(&mut (*ldev).gpio, v);
        }
    } else {
        return;
    }

    if !detected.is_null() {
        snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*detected).id);
    }
    if update != 0 {
        c = (*ldev).headphone_ctrl;
        if !c.is_null() {
            snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*c).id);
        }
        c = (*ldev).speaker_ctrl;
        if !c.is_null() {
            snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*c).id);
        }
        c = (*ldev).lineout_ctrl;
        if !c.is_null() {
            snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*c).id);
        }
    }
}

unsafe fn layout_attached_codec(codec: *mut aoa_codec) {
    let mut cc: *const codec_connection;
    let mut ctl: *mut snd_kcontrol;
    let headphones: c_int;
    let lineout: c_int;
    let ldev = layout_device;

    /* need to add this codec to our codec array! */

    cc = (*codec).fabric_data as *const codec_connection;

    headphones = (*(*(*codec).gpio).methods).get_detect.unwrap()((*codec).gpio, AOA_NOTIFY_HEADPHONE);
    lineout = (*(*(*codec).gpio).methods).get_detect.unwrap()((*codec).gpio, AOA_NOTIFY_LINE_OUT);

    if (*(*(*codec).gpio).methods).set_master.is_some() {
        ctl = snd_ctl_new1(&master_ctl, (*codec).gpio as *mut c_void);
        (*ldev).master_ctrl = ctl;
        aoa_snd_ctl_add(ctl);
    }
    while (*cc).connected != 0 {
        if (*cc).connected & CC_SPEAKERS != 0 {
            if headphones <= 0 && lineout <= 0 {
                (*(*ldev).gpio.methods).set_speakers.unwrap()((*codec).gpio, 1);
            }
            ctl = snd_ctl_new1(&speakers_ctl, (*codec).gpio as *mut c_void);
            (*ldev).speaker_ctrl = ctl;
            aoa_snd_ctl_add(ctl);
        }
        if (*cc).connected & CC_HEADPHONE != 0 {
            if headphones == 1 {
                (*(*ldev).gpio.methods).set_headphone.unwrap()((*codec).gpio, 1);
            }
            ctl = snd_ctl_new1(&headphone_ctl, (*codec).gpio as *mut c_void);
            (*ldev).headphone_ctrl = ctl;
            aoa_snd_ctl_add(ctl);
            (*ldev).have_headphone_detect = ((*(*ldev).gpio.methods).set_notify.unwrap()(
                &mut (*ldev).gpio,
                AOA_NOTIFY_HEADPHONE,
                Some(layout_notify),
                &mut (*ldev).selfptr_headphone as *mut _ as *mut c_void,
            ) == 0) as u32;
            if (*ldev).have_headphone_detect != 0 {
                ctl = snd_ctl_new1(&headphone_detect_choice, ldev as *mut c_void);
                aoa_snd_ctl_add(ctl);
                ctl = snd_ctl_new1(&headphone_detected, ldev as *mut c_void);
                (*ldev).headphone_detected_ctrl = ctl;
                aoa_snd_ctl_add(ctl);
            }
        }
        if (*cc).connected & CC_LINEOUT != 0 {
            if lineout == 1 {
                (*(*ldev).gpio.methods).set_lineout.unwrap()((*codec).gpio, 1);
            }
            ctl = snd_ctl_new1(&lineout_ctl, (*codec).gpio as *mut c_void);
            if ctl.is_null() {
                return;
            }
            if (*cc).connected & CC_LINEOUT_LABELLED_HEADPHONE != 0 {
                strscpy((*ctl).id.name.as_mut_ptr(), b"Headphone Switch\0".as_ptr() as *const c_char);
            }
            (*ldev).lineout_ctrl = ctl;
            aoa_snd_ctl_add(ctl);
            (*ldev).have_lineout_detect = ((*(*ldev).gpio.methods).set_notify.unwrap()(
                &mut (*ldev).gpio,
                AOA_NOTIFY_LINE_OUT,
                Some(layout_notify),
                &mut (*ldev).selfptr_lineout as *mut _ as *mut c_void,
            ) == 0) as u32;
            if (*ldev).have_lineout_detect != 0 {
                ctl = snd_ctl_new1(&lineout_detect_choice, ldev as *mut c_void);
                if ctl.is_null() {
                    return;
                }
                if (*cc).connected & CC_LINEOUT_LABELLED_HEADPHONE != 0 {
                    strscpy((*ctl).id.name.as_mut_ptr(), b"Headphone Detect Autoswitch\0".as_ptr() as *const c_char);
                }
                aoa_snd_ctl_add(ctl);
                ctl = snd_ctl_new1(&lineout_detected, ldev as *mut c_void);
                if ctl.is_null() {
                    return;
                }
                if (*cc).connected & CC_LINEOUT_LABELLED_HEADPHONE != 0 {
                    strscpy((*ctl).id.name.as_mut_ptr(), b"Headphone Detected\0".as_ptr() as *const c_char);
                }
                (*ldev).lineout_detected_ctrl = ctl;
                aoa_snd_ctl_add(ctl);
            }
        }
        cc = cc.add(1);
    }
    /* now update initial state */
    if (*ldev).have_headphone_detect != 0 {
        layout_notify(&mut (*ldev).selfptr_headphone as *mut _ as *mut c_void);
    }
    if (*ldev).have_lineout_detect != 0 {
        layout_notify(&mut (*ldev).selfptr_lineout as *mut _ as *mut c_void);
    }
}

static mut layout_fabric: aoa_fabric = aoa_fabric {
    name: b"SoundByLayout\0".as_ptr() as *const c_char,
    owner: THIS_MODULE,
    found_codec: Some(layout_found_codec),
    remove_codec: Some(layout_remove_codec),
    attached_codec: Some(layout_attached_codec),
};

unsafe fn aoa_fabric_layout_probe(sdev: *mut soundbus_dev) -> c_int {
    let mut sound: *mut device_node = ptr::null_mut();
    let mut id: *const c_uint;
    let mut layout: *mut layout = ptr::null_mut();
    let mut ldev: *mut layout_dev = ptr::null_mut();
    let err: c_int;

    /* hm, currently we can only have one ... */
    if !layout_device.is_null() {
        return -ENODEV;
    }

    /* by breaking out we keep a reference */
    sound = first_child_of_node((*sdev).ofdev.dev.of_node);
    while !sound.is_null() {
        if of_node_is_type(sound, b"soundchip\0".as_ptr() as *const c_char) != 0 {
            break;
        }
        sound = next_child_of_node((*sdev).ofdev.dev.of_node, sound);
    }
    if sound.is_null() {
        return -ENODEV;
    }

    id = of_get_property(sound, b"layout-id\0".as_ptr() as *const c_char, ptr::null_mut()) as *const c_uint;
    if !id.is_null() {
        layout = find_layout_by_id(*id);
    } else {
        id = of_get_property(sound, b"device-id\0".as_ptr() as *const c_char, ptr::null_mut()) as *const c_uint;
        if !id.is_null() {
            layout = find_layout_by_device(*id);
        }
    }

    if layout.is_null() {
        printk(KERN_ERR, b"snd-aoa-fabric-layout: unknown layout\n\0".as_ptr() as *const c_char);
        return outnodev(sound);
    }

    ldev = kzalloc_obj_layout_dev();
    if ldev.is_null() {
        return outnodev(sound);
    }

    layout_device = ldev;
    (*ldev).sdev = sdev;
    (*ldev).sound = sound;
    (*ldev).layout = layout;
    (*ldev).gpio.node = (*sound).parent;
    match (*layout).layout_id {
        0 | 41 | 51 | 58 => {
            (*ldev).gpio.methods = ftr_gpio_methods;
            printk(KERN_DEBUG, b"snd-aoa-fabric-layout: Using direct GPIOs\n\0".as_ptr() as *const c_char);
        }
        _ => {
            (*ldev).gpio.methods = pmf_gpio_methods;
            printk(KERN_DEBUG, b"snd-aoa-fabric-layout: Using PMF GPIOs\n\0".as_ptr() as *const c_char);
        }
    }
    (*ldev).selfptr_headphone.ptr = ldev;
    (*ldev).selfptr_lineout.ptr = ldev;
    dev_set_drvdata(&mut (*sdev).ofdev.dev, ldev as *mut c_void);
    list_add(&mut (*ldev).list, &mut layouts_list);
    layouts_list_items += 1;

    /* assign these before registering ourselves, so
     * callbacks that are done during registration
     * already have the values */
    (*sdev).pcmid = (*(*ldev).layout).pcmid;
    if !(*(*ldev).layout).busname.is_null() {
        (*sdev).pcmname = (*(*ldev).layout).busname;
    } else {
        (*sdev).pcmname = b"Master\0".as_ptr() as *mut c_char;
    }

    (*(*ldev).gpio.methods).init.unwrap()(&mut (*ldev).gpio);

    err = aoa_fabric_register(&mut layout_fabric, &mut (*sdev).ofdev.dev);
    if err != 0 && err != -EALREADY {
        printk(
            KERN_INFO,
            b"snd-aoa-fabric-layout: can't use, another fabric is active!\n\0".as_ptr() as *const c_char,
        );
        /* we won't be using these then... */
        (*(*ldev).gpio.methods).exit.unwrap()(&mut (*ldev).gpio);
        /* reset if we didn't use it */
        (*sdev).pcmname = ptr::null_mut();
        (*sdev).pcmid = -1;
        list_del(&mut (*ldev).list);
        layouts_list_items -= 1;
        kfree(ldev as *mut c_void);
        return outnodev(sound);
    }

    use_layout(layout);
    (*ldev).switch_on_headphone = 1;
    (*ldev).switch_on_lineout = 1;
    0
}

unsafe fn outnodev(sound: *mut device_node) -> c_int {
    of_node_put(sound);
    layout_device = ptr::null_mut();
    -ENODEV
}

unsafe fn aoa_fabric_layout_remove(sdev: *mut soundbus_dev) {
    let ldev = dev_get_drvdata(&mut (*sdev).ofdev.dev) as *mut layout_dev;
    let mut i: usize;

    i = 0;
    while i < MAX_CODECS_PER_BUS {
        if !(*ldev).codecs[i].is_null() {
            aoa_fabric_unlink_codec((*ldev).codecs[i]);
        }
        (*ldev).codecs[i] = ptr::null_mut();
        i += 1;
    }
    list_del(&mut (*ldev).list);
    layouts_list_items -= 1;
    of_node_put((*ldev).sound);

    (*(*ldev).gpio.methods).set_notify.unwrap()(
        &mut (*ldev).gpio,
        AOA_NOTIFY_HEADPHONE,
        None,
        ptr::null_mut(),
    );
    (*(*ldev).gpio.methods).set_notify.unwrap()(
        &mut (*ldev).gpio,
        AOA_NOTIFY_LINE_OUT,
        None,
        ptr::null_mut(),
    );

    (*(*ldev).gpio.methods).exit.unwrap()(&mut (*ldev).gpio);
    layout_device = ptr::null_mut();
    kfree(ldev as *mut c_void);
    (*sdev).pcmid = -1;
    (*sdev).pcmname = ptr::null_mut();
}

unsafe fn aoa_fabric_layout_suspend(dev: *mut device) -> c_int {
    let ldev = dev_get_drvdata(dev) as *mut layout_dev;

    if !(*ldev).gpio.methods.is_null() && (*(*ldev).gpio.methods).all_amps_off.is_some() {
        (*(*ldev).gpio.methods).all_amps_off.unwrap()(&mut (*ldev).gpio);
    }

    0
}

unsafe fn aoa_fabric_layout_resume(dev: *mut device) -> c_int {
    let ldev = dev_get_drvdata(dev) as *mut layout_dev;

    if !(*ldev).gpio.methods.is_null() && (*(*ldev).gpio.methods).all_amps_restore.is_some() {
        (*(*ldev).gpio.methods).all_amps_restore.unwrap()(&mut (*ldev).gpio);
    }

    0
}

static mut aoa_fabric_layout_pm_ops: dev_pm_ops = dev_pm_ops {
    suspend: Some(aoa_fabric_layout_suspend),
    resume: Some(aoa_fabric_layout_resume),
};

static mut aoa_soundbus_driver: soundbus_driver = soundbus_driver {
    name: b"snd_aoa_soundbus_drv\0".as_ptr() as *const c_char,
    owner: THIS_MODULE,
    probe: Some(aoa_fabric_layout_probe),
    remove: Some(aoa_fabric_layout_remove),
    driver: device_driver {
        owner: THIS_MODULE,
        pm: unsafe { &aoa_fabric_layout_pm_ops as *const dev_pm_ops },
    },
};

unsafe fn aoa_fabric_layout_init() -> c_int {
    soundbus_register_driver(&mut aoa_soundbus_driver)
}

unsafe fn aoa_fabric_layout_exit() {
    soundbus_unregister_driver(&mut aoa_soundbus_driver);
    aoa_fabric_unregister(&mut layout_fabric);
}

/* module_init(aoa_fabric_layout_init);
 * module_exit(aoa_fabric_layout_exit);
 */

type c_long = isize;
type snd_ctl_boolean_mono_info = unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int;

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct device_node {
    parent: *mut device_node,
    phandle: u32,
}

#[repr(C)]
struct gpio_runtime {
    node: *mut device_node,
    methods: *mut gpio_methods,
}

#[repr(C)]
struct gpio_methods {
    init: Option<unsafe extern "C" fn(*mut gpio_runtime)>,
    exit: Option<unsafe extern "C" fn(*mut gpio_runtime)>,
    get_headphone: Option<unsafe extern "C" fn(*mut gpio_runtime) -> c_int>,
    set_headphone: Option<unsafe extern "C" fn(*mut gpio_runtime, c_int)>,
    get_speakers: Option<unsafe extern "C" fn(*mut gpio_runtime) -> c_int>,
    set_speakers: Option<unsafe extern "C" fn(*mut gpio_runtime, c_int)>,
    get_lineout: Option<unsafe extern "C" fn(*mut gpio_runtime) -> c_int>,
    set_lineout: Option<unsafe extern "C" fn(*mut gpio_runtime, c_int)>,
    get_master: Option<unsafe extern "C" fn(*mut gpio_runtime) -> c_int>,
    set_master: Option<unsafe extern "C" fn(*mut gpio_runtime, c_int)>,
    get_detect: Option<unsafe extern "C" fn(*mut gpio_runtime, c_int) -> c_int>,
    set_notify: Option<unsafe extern "C" fn(*mut gpio_runtime, c_int, Option<unsafe extern "C" fn(*mut c_void)>, *mut c_void) -> c_int>,
    all_amps_off: Option<unsafe extern "C" fn(*mut gpio_runtime)>,
    all_amps_restore: Option<unsafe extern "C" fn(*mut gpio_runtime)>,
}

#[repr(C)]
struct aoa_codec {
    node: *mut device_node,
    name: *mut c_char,
    soundbus_dev: *mut soundbus_dev,
    gpio: *mut gpio_runtime,
    connected: c_int,
    fabric_data: *mut c_void,
}

#[repr(C)]
struct snd_kcontrol {
    private_value: c_ulong,
    id: snd_ctl_elem_id,
}

#[repr(C)]
struct snd_ctl_elem_id {
    name: [c_char; 44],
}

#[repr(C)]
struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}

#[repr(C)]
struct snd_ctl_elem_value_value {
    integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
struct snd_ctl_elem_value_integer {
    value: [c_long; 128],
}

#[repr(C)]
struct snd_ctl_elem_info;

#[repr(C)]
struct snd_kcontrol_new {
    iface: c_int,
    name: *const c_char,
    access: c_uint,
    info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    private_value: c_ulong,
}

#[repr(C)]
struct snd_card;

#[repr(C)]
struct aoa_fabric {
    name: *const c_char,
    owner: *mut c_void,
    found_codec: Option<unsafe fn(*mut aoa_codec) -> c_int>,
    remove_codec: Option<unsafe fn(*mut aoa_codec)>,
    attached_codec: Option<unsafe fn(*mut aoa_codec)>,
}

#[repr(C)]
struct device {
    of_node: *mut device_node,
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct soundbus_dev {
    ofdev: platform_device,
    pcmid: c_int,
    pcmname: *mut c_char,
}

#[repr(C)]
struct dev_pm_ops {
    suspend: Option<unsafe fn(*mut device) -> c_int>,
    resume: Option<unsafe fn(*mut device) -> c_int>,
}

#[repr(C)]
struct device_driver {
    owner: *mut c_void,
    pm: *const dev_pm_ops,
}

#[repr(C)]
struct soundbus_driver {
    name: *const c_char,
    owner: *mut c_void,
    probe: Option<unsafe fn(*mut soundbus_dev) -> c_int>,
    remove: Option<unsafe fn(*mut soundbus_dev)>,
    driver: device_driver,
}

const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const EALREADY: c_int = 114;
const AOA_NOTIFY_HEADPHONE: c_int = 0;
const AOA_NOTIFY_LINE_OUT: c_int = 1;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_int = 2;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 3;
const SNDRV_CTL_ELEM_ACCESS_READ: c_uint = 1;
const SNDRV_CTL_EVENT_MASK_VALUE: c_uint = 1;
static mut THIS_MODULE: *mut c_void = ptr::null_mut();
const KERN_INFO: *const c_char = b"\x016\0".as_ptr() as *const c_char;
const KERN_ERR: *const c_char = b"\x013\0".as_ptr() as *const c_char;
const KERN_DEBUG: *const c_char = b"\x017\0".as_ptr() as *const c_char;

extern "C" {
    static mut ftr_gpio_methods: *mut gpio_methods;
    static mut pmf_gpio_methods: *mut gpio_methods;

    fn request_module(fmt: *const c_char, ...) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn of_node_name_eq(node: *mut device_node, name: *const c_char) -> c_int;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn of_get_property(node: *mut device_node, name: *const c_char, lenp: *mut c_int) -> *const c_void;
    fn printk(level: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn aoa_get_card() -> *mut snd_card;
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_ctl_elem_id);
    fn snd_ctl_new1(template: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn aoa_snd_ctl_add(ctl: *mut snd_kcontrol) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn aoa_fabric_register(fabric: *mut aoa_fabric, dev: *mut device) -> c_int;
    fn aoa_fabric_unregister(fabric: *mut aoa_fabric);
    fn aoa_fabric_unlink_codec(codec: *mut aoa_codec);
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn of_node_put(node: *mut device_node);
    fn kfree(ptr: *mut c_void);
    fn soundbus_register_driver(driver: *mut soundbus_driver) -> c_int;
    fn soundbus_unregister_driver(driver: *mut soundbus_driver);
    fn of_node_is_type(node: *mut device_node, type_: *const c_char) -> c_int;
    fn first_child_of_node(node: *mut device_node) -> *mut device_node;
    fn next_child_of_node(node: *mut device_node, prev: *mut device_node) -> *mut device_node;
}

unsafe fn kzalloc_obj_layout_dev() -> *mut layout_dev {
    extern "C" {
        fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    }
    kzalloc(core::mem::size_of::<layout_dev>(), 0) as *mut layout_dev
}

unsafe fn container_first_layout_dev(_head: *mut list_head) -> *mut layout_dev {
    /* Rust translation of list_for_each_entry(ldev, &layouts_list, list).
     * The concrete container_of/list traversal comes from Linux list.h. */
    ptr::null_mut()
}

unsafe fn container_next_layout_dev(_ldev: *mut layout_dev, _head: *mut list_head) -> *mut layout_dev {
    /* Rust translation placeholder for the next list_for_each_entry step. */
    ptr::null_mut()
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
