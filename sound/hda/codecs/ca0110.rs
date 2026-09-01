// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * HD audio codec driver for Creative X-Fi CA0110-IBG chip
 *
 * Copyright (c) 2008 Takashi Iwai <tiwai@suse.de>
 */

// C dependencies:
// #include <linux/init.h>
// #include <linux/slab.h>
// #include <linux/module.h>
// #include <sound/core.h>
// #include <sound/hda_codec.h>
// #include "hda_local.h"
// #include "hda_auto_parser.h"
// #include "hda_jack.h"
// #include "generic.h"

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

const ENOMEM: c_int = 12;

#[repr(C)]
pub struct hda_codec {
    pub spec: *mut c_void,
    pub bus: *mut hda_bus,
}

#[repr(C)]
pub struct hda_bus {
    pub core: hda_bus_core,
}

#[repr(C)]
pub struct hda_bus_core {
    pub needs_damn_long_delay: c_int,
}

#[repr(C)]
pub struct hda_gen_spec {
    pub autocfg: hda_auto_pin_cfg,
    pub multi_cap_vol: c_int,
}

#[repr(C)]
pub struct hda_auto_pin_cfg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hda_device_id {
    pub vendor_id: u32,
    pub name: *const c_char,
}

#[repr(C)]
pub struct hda_codec_ops {
    pub probe: Option<unsafe extern "C" fn(*mut hda_codec, *const hda_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut hda_codec)>,
    pub build_controls: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub build_pcms: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub init: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub unsol_event: Option<unsafe extern "C" fn(*mut hda_codec, u32)>,
}

#[repr(C)]
pub struct hda_codec_driver {
    pub id: *const hda_device_id,
    pub ops: *const hda_codec_ops,
}

unsafe extern "C" {
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn snd_hda_parse_pin_defcfg(
        codec: *mut hda_codec,
        cfg: *mut hda_auto_pin_cfg,
        ignore_nids: *mut c_void,
        cond_flags: c_int,
    ) -> c_int;
    fn snd_hda_gen_parse_auto_config(
        codec: *mut hda_codec,
        cfg: *mut hda_auto_pin_cfg,
    ) -> c_int;
    fn snd_hda_gen_spec_init(spec: *mut hda_gen_spec);
    fn snd_hda_gen_remove(codec: *mut hda_codec);
    fn snd_hda_gen_build_controls(codec: *mut hda_codec) -> c_int;
    fn snd_hda_gen_build_pcms(codec: *mut hda_codec) -> c_int;
    fn snd_hda_gen_init(codec: *mut hda_codec) -> c_int;
    fn snd_hda_jack_unsol_event(codec: *mut hda_codec, res: u32);
}

unsafe fn kzalloc_obj<T>() -> *mut T {
    kzalloc(core::mem::size_of::<T>(), 0) as *mut T
}

unsafe extern "C" fn ca0110_parse_auto_config(codec: *mut hda_codec) -> c_int {
    let spec: *mut hda_gen_spec = (*codec).spec as *mut hda_gen_spec;
    let mut err: c_int;

    err = snd_hda_parse_pin_defcfg(codec, &mut (*spec).autocfg, ptr::null_mut(), 0);
    if err < 0 {
        return err;
    }
    err = snd_hda_gen_parse_auto_config(codec, &mut (*spec).autocfg);
    if err < 0 {
        return err;
    }

    0
}

unsafe extern "C" fn ca0110_probe(
    codec: *mut hda_codec,
    _id: *const hda_device_id,
) -> c_int {
    let spec: *mut hda_gen_spec;
    let err: c_int;

    spec = kzalloc_obj::<hda_gen_spec>();
    if spec.is_null() {
        return -ENOMEM;
    }
    snd_hda_gen_spec_init(spec);
    (*codec).spec = spec as *mut c_void;

    (*spec).multi_cap_vol = 1;
    (*(*codec).bus).core.needs_damn_long_delay = 1;

    err = ca0110_parse_auto_config(codec);
    if err < 0 {
        snd_hda_gen_remove(codec);
        return err;
    }

    0
}

static CA0110_CODEC_OPS: hda_codec_ops = hda_codec_ops {
    probe: Some(ca0110_probe),
    remove: Some(snd_hda_gen_remove),
    build_controls: Some(snd_hda_gen_build_controls),
    build_pcms: Some(snd_hda_gen_build_pcms),
    init: Some(snd_hda_gen_init),
    unsol_event: Some(snd_hda_jack_unsol_event),
};

/*
 * driver entries
 */
static SND_HDA_ID_CA0110: [hda_device_id; 4] = [
    hda_device_id {
        vendor_id: 0x1102000a,
        name: c"CA0110-IBG".as_ptr(),
    },
    hda_device_id {
        vendor_id: 0x1102000b,
        name: c"CA0110-IBG".as_ptr(),
    },
    hda_device_id {
        vendor_id: 0x1102000d,
        name: c"SB0880 X-Fi".as_ptr(),
    },
    hda_device_id {
        vendor_id: 0,
        name: ptr::null(),
    }, /* terminator */
];

// MODULE_DEVICE_TABLE(hdaudio, snd_hda_id_ca0110);

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Creative CA0110-IBG HD-audio codec");

static mut CA0110_DRIVER: hda_codec_driver = hda_codec_driver {
    id: SND_HDA_ID_CA0110.as_ptr(),
    ops: &CA0110_CODEC_OPS,
};

// module_hda_codec_driver(ca0110_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
